use command::*;
use event::*;
use error::*;
use input::*;
use output::*;
use player::*;
use shogi::*;
use hash::*;
use Logger;
use logger::FileLogger;
use OnErrorHandler;
use TryFrom;
use SandBox;
use rule;
use rule::*;
use protocol::*;

use chrono::prelude::*;

use std::fmt;
use std::{thread};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread::JoinHandle;
use std::marker::Send;
use std::marker::PhantomData;
use std::time::{Instant,Duration};
use std::collections::HashMap;
use std::io::Write;
use std::io::BufWriter;
use std::fs;
use std::fs::OpenOptions;

use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;
use crossbeam_channel::SendError;
use crossbeam_channel::after;
use crossbeam_channel::never;

pub trait SelfMatchKifuWriter {
	fn write(&mut self,initial_sfen:&String,m:&Vec<Move>) -> Result<(),KifuWriteError>;
	fn to_sfen(&self,initial_sfen:&String,m:&Vec<Move>)
		-> Result<String, SfenStringConvertError> {

		let sfen = initial_sfen.split(" ").collect::<Vec<&str>>();

		if sfen.len() >= 5 {
			match (sfen[0],sfen[1],sfen[2],sfen[3],sfen[4]) {
				("sfen",p1,p2,p3,p4) if m.len() > 0 => {
					Ok(format!("sfen {} {} {} {} moves {}",p1,p2,p3,p4,m.to_sfen()?))
				},
				("sfen",p1,p2,p3,p4) => {
					Ok(format!("sfen {} {} {} {}",p1,p2,p3,p4))
				},
				("startpos",_,_,_,_) if m.len() > 0=> {
					Ok(format!("startpos moves {}",m.to_sfen()?))
				},
				("startpos",_,_,_,_)=> {
					Ok(format!("startpos"))
				},
				_ => {
					Err(SfenStringConvertError::InvalidFormat(initial_sfen.clone()))
				}
			}
		} else if sfen.len() >= 1 && sfen[0] == "startpos" {
			if m.len() > 0 {
				Ok(format!("startpos moves {}",m.to_sfen()?))
			} else {
				Ok(format!("startpos"))
			}
		} else {
			Err(SfenStringConvertError::InvalidFormat(initial_sfen.clone()))
		}
	}
}
#[derive(Debug)]
pub struct FileSfenKifuWriter {
	writer:BufWriter<fs::File>,
}
impl FileSfenKifuWriter {
	pub fn new(file:String) -> Result<FileSfenKifuWriter,KifuWriteError> {
		Ok(FileSfenKifuWriter {
			writer:BufWriter::new(OpenOptions::new().append(true).create(true).open(file)?),
		})
	}
}
impl SelfMatchKifuWriter for FileSfenKifuWriter {
	fn write(&mut self,initial_sfen:&String,m:&Vec<Move>) -> Result<(),KifuWriteError> {
		let sfen = self.to_sfen(initial_sfen,m)?;

		let _ = self.writer.write(format!("{}\n",sfen).as_bytes())?;
		Ok(())
	}
}
#[derive(Debug)]
enum TimeoutKind {
	Never,
	Turn,
	Uptime,
}
#[derive(Debug)]
pub enum SelfMatchMessage {
	Ready,
	GameStart,
	StartThink(Teban,Banmen,MochigomaCollections,u32,Vec<AppliedMove>),
	StartPonderThink(Teban,Banmen,MochigomaCollections,u32,Vec<AppliedMove>),
	NotifyMove(BestMove),
	PonderHit,
	PonderNG,
	GameEnd(GameEndState),
	Quit,
	Error(usize),
}
#[derive(Debug)]
pub struct SelfMatchResult {
	pub game_count:u32,
	pub elapsed:Duration,
	pub start_dt:DateTime<Local>,
	pub end_dt:DateTime<Local>,
}
#[derive(Debug)]
pub struct SelfMatchEngine<T,E,S>
	where T: USIPlayer<E> + fmt::Debug, Arc<Mutex<T>>: Send + 'static,
			E: PlayerError,
			S: InfoSender,
			Arc<Mutex<S>>: Send + 'static {
	player_error_type:PhantomData<E>,
	player1:Arc<Mutex<T>>,
	player2:Arc<Mutex<T>>,
	info_sender:S,
	game_time_limit:UsiGoTimeLimit,
	uptime:Option<Duration>,
	number_of_games:Option<u32>,
	pub system_event_queue:Arc<Mutex<EventQueue<SystemEvent,SystemEventKind>>>,
}
impl<T,E,S> SelfMatchEngine<T,E,S>
	where T: USIPlayer<E> + fmt::Debug, Arc<Mutex<T>>: Send + 'static,
			E: PlayerError,
			S: InfoSender {
	pub fn new(player1:T,player2:T,
				info_sender:S,
				game_time_limit:UsiGoTimeLimit,
				uptime:Option<Duration>,number_of_games:Option<u32>)
	-> SelfMatchEngine<T,E,S>
	where T: USIPlayer<E> + fmt::Debug,
			Arc<Mutex<T>>: Send + 'static,
			E: PlayerError,
			S: InfoSender,
			Arc<Mutex<S>>: Send + 'static {
		SelfMatchEngine {
			player_error_type:PhantomData::<E>,
			player1:Arc::new(Mutex::new(player1)),
			player2:Arc::new(Mutex::new(player2)),
			info_sender:info_sender,
			game_time_limit:game_time_limit,
			uptime:uptime,
			number_of_games:number_of_games,
			system_event_queue:Arc::new(Mutex::new(EventQueue::new())),
		}
	}

	pub fn start_default<I,F,RH,EH>(&mut self, on_init_event_dispatcher:I,
						on_before_newgame:F,
						initial_position_creator:Option<Box<FnMut() -> String + Send + 'static>>,
						kifu_writer:Option<Box<FnMut(&String,&Vec<Move>) -> Result<(),KifuWriteError>  +Send + 'static>>,
						input_handler:RH,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						on_error:EH) -> Result<SelfMatchResult,SelfMatchRunningError>
		where F: FnMut() -> bool + Send + 'static,
				RH: FnMut(String) -> Result<bool,SelfMatchRunningError> + Send + 'static,
				I: FnMut(&mut USIEventDispatcher<
														SelfMatchEventKind,
														SelfMatchEvent,
														SelfMatchEngine<T, E, S>,FileLogger,E>),
				Arc<Mutex<FileLogger>>: Send + 'static,
				EH: FnMut(Option<Arc<Mutex<OnErrorHandler<FileLogger>>>>,
					&SelfMatchRunningError) {
		self.start_with_log_path(String::from("logs/log.txt"),
								on_init_event_dispatcher,
								on_before_newgame,
								initial_position_creator,
								kifu_writer, input_handler,
								player1_options, player2_options, on_error)
	}

	pub fn start_with_log_path<I,F,RH,EH>(&mut self,path:String,
						on_init_event_dispatcher:I,
						on_before_newgame:F,
						initial_position_creator:Option<Box<FnMut() -> String + Send + 'static>>,
						kifu_writer:Option<Box<FnMut(&String,&Vec<Move>) -> Result<(),KifuWriteError>  +Send + 'static>>,
						input_handler:RH,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						mut on_error:EH) -> Result<SelfMatchResult,SelfMatchRunningError>
		where F: FnMut() -> bool + Send + 'static,
				RH: FnMut(String) -> Result<bool,SelfMatchRunningError> + Send + 'static,
				I: FnMut(&mut USIEventDispatcher<
														SelfMatchEventKind,
														SelfMatchEvent,
														SelfMatchEngine<T, E, S>,FileLogger,E>),
				Arc<Mutex<FileLogger>>: Send + 'static,
				EH: FnMut(Option<Arc<Mutex<OnErrorHandler<FileLogger>>>>,
					&SelfMatchRunningError) {
		let logger = match FileLogger::new(path) {
			Err(e) => {
				let e = SelfMatchRunningError::IOError(e);
				on_error(None,&e);
				return Err(e);
			},
			Ok(logger) => logger,
		};

		let input_reader = USIStdInputReader::new();

		self.start(on_init_event_dispatcher,
					on_before_newgame,
					initial_position_creator,
					kifu_writer, input_reader, input_handler,
					player1_options, player2_options, logger, on_error)
	}

	pub fn start<I,F,R,RH,L,EH>(&mut self, on_init_event_dispatcher:I,
						on_before_newgame:F,
						initial_position_creator:Option<Box<FnMut() -> String + Send + 'static>>,
						kifu_writer:Option<Box<FnMut(&String,&Vec<Move>) -> Result<(),KifuWriteError>  +Send + 'static>>,
						input_reader:R,
						input_handler:RH,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						logger:L, mut on_error:EH) -> Result<SelfMatchResult,SelfMatchRunningError>
		where F: FnMut() -> bool + Send + 'static,
				R: USIInputReader + Send + 'static,
				RH: FnMut(String) -> Result<bool,SelfMatchRunningError> + Send + 'static,
				I: FnMut(&mut USIEventDispatcher<
														SelfMatchEventKind,
														SelfMatchEvent,
														SelfMatchEngine<T, E, S>,L,E>),
				L: Logger + fmt::Debug,
				Arc<Mutex<L>>: Send + 'static,
				EH: FnMut(Option<Arc<Mutex<OnErrorHandler<L>>>>,
					&SelfMatchRunningError) {
		let logger_arc = Arc::new(Mutex::new(logger));
		let on_error_handler_arc = Arc::new(Mutex::new(OnErrorHandler::new(logger_arc.clone())));
		let on_error_handler = on_error_handler_arc.clone();

		let r = self.run(on_init_event_dispatcher,
							on_before_newgame,
							initial_position_creator,
							kifu_writer, input_reader, input_handler,
							player1_options, player2_options,
							logger_arc, on_error_handler_arc);

		if let Err(ref e) = r {
			on_error(Some(on_error_handler),e);
		}

		r
	}

	fn run<I,F,R,RH,L>(&mut self, mut on_init_event_dispatcher:I,
						mut on_before_newgame:F,
						initial_position_creator:Option<Box<FnMut() -> String + Send + 'static>>,
						kifu_writer:Option<Box<FnMut(&String,&Vec<Move>) -> Result<(),KifuWriteError> + Send + 'static>>,
						mut input_reader:R,
						mut input_handler:RH,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						logger_arc:Arc<Mutex<L>>,
						on_error_handler_arc:Arc<Mutex<OnErrorHandler<L>>>) -> Result<SelfMatchResult,SelfMatchRunningError>
		where F: FnMut() -> bool + Send + 'static,
				R: USIInputReader + Send + 'static,
				RH: FnMut(String) -> Result<bool,SelfMatchRunningError> + Send + 'static,
				I: FnMut(&mut USIEventDispatcher<
														SelfMatchEventKind,
														SelfMatchEvent,
														SelfMatchEngine<T, E, S>,L,E>),
				L: Logger + fmt::Debug,
				Arc<Mutex<L>>: Send + 'static {
		let start_time = Instant::now();
		let start_dt = Local::now();

		let mut self_match_event_dispatcher = USIEventDispatcher::<
														SelfMatchEventKind,
														SelfMatchEvent,
														SelfMatchEngine<T, E, S>,L,E>::new(&on_error_handler_arc);

		on_init_event_dispatcher(&mut self_match_event_dispatcher);

		let mut system_event_dispatcher:USIEventDispatcher<SystemEventKind,
														SystemEvent,SelfMatchEngine<T, E, S>,L,E> = USIEventDispatcher::new(&on_error_handler_arc);

		let user_event_queue_arc:[Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>; 2] = [Arc::new(Mutex::new(EventQueue::new())),Arc::new(Mutex::new(EventQueue::new()))];

		let user_event_queue = [user_event_queue_arc[0].clone(),user_event_queue_arc[1].clone()];

		let mut initial_position_creator:Box<FnMut() -> String + Send + 'static> =
			initial_position_creator.map_or(Box::new(|| String::from("startpos")), |f| {
				f
			});

		let on_error_handler = on_error_handler_arc.clone();

		let mut kifu_writer = kifu_writer;
		let mut kifu_writer = move |sfen:&String,m:&Vec<Move>| {
			let _ = kifu_writer.as_mut().map(|w| {
				let _= w(sfen,m).map_err(|e| on_error_handler.lock().map(|h| h.call(&e)).is_err());
			});
		};

		let quit_ready_arc = Arc::new(AtomicBool::new(false));

		let on_error_handler = on_error_handler_arc.clone();

		let self_match_event_queue:EventQueue<SelfMatchEvent,SelfMatchEventKind> = EventQueue::new();
		let self_match_event_queue_arc = Arc::new(Mutex::new(self_match_event_queue));

		let info_sender_arc = self.info_sender.clone();

		let (ss,sr) = unbounded();
		let (cs1,cr1) = unbounded();
		let (cs2,cr2) = unbounded();
		let mut cr = vec![cr1,cr2];

		{
			let ss = ss.clone();
			let quit_ready = quit_ready_arc.clone();

			let on_error_handler = on_error_handler_arc.clone();

			system_event_dispatcher.add_handler(SystemEventKind::Quit, move |_,e| {
				match e {
					&SystemEvent::Quit => {
						for i in 0..2 {
							match user_event_queue[i].lock() {
								Ok(mut user_event_queue) => {
									user_event_queue.push(UserEvent::Quit);
								},
								Err(ref e) => {
									on_error_handler.lock().map(|h| h.call(e)).is_err();
								}
							};
						};

						if !quit_ready.load(Ordering::Acquire) {
							if let Err(ref e) = ss.send(SelfMatchMessage::Quit) {
								on_error_handler.lock().map(|h| h.call(e)).is_err();
							}
						}

						Ok(())
					},
					e => Err(EventHandlerError::InvalidState(e.event_kind())),
				}
			});
		}

		let player1 = self.player1.clone();
		let player2 = self.player2.clone();

		match player1.lock() {
			Ok(mut player) => {
				for (k,v) in player1_options {
					match player.set_option(k,v) {
						Ok(()) => (),
						Err(ref e) => {
							on_error_handler.lock().map(|h| h.call(e)).is_err();
							return Err(SelfMatchRunningError::Fail(String::from(
								"An error occurred while executing a self match. Please see the log for details ..."
							)));
						}
					}
				}
			},
			Err(_) => {
				return Err(SelfMatchRunningError::InvalidState(String::from(
					"Failed to secure exclusive lock of player object."
				)));
			}
		}

		match player2.lock() {
			Ok(mut player) => {
				for (k,v) in player2_options {
					match player.set_option(k,v) {
						Ok(()) => (),
						Err(ref e) => {
							on_error_handler.lock().map(|h| h.call(e)).is_err();
							return Err(SelfMatchRunningError::Fail(String::from(
								"An error occurred while executing a self match. Please see the log for details ..."
							)));
						}
					}
				}
			},
			Err(_) => {
				return Err(SelfMatchRunningError::InvalidState(String::from(
					"Failed to secure exclusive lock of player object."
				)));
			}
		}

		let position_parser = PositionParser::new();

		let self_match_event_queue = self_match_event_queue_arc.clone();
		let quit_ready = quit_ready_arc.clone();

		let on_error_handler = on_error_handler_arc.clone();

		let uptime = self.uptime.map(|t| t);
		let number_of_games = self.number_of_games.map(|n| n);
		let game_time_limit = self.game_time_limit;
		let user_event_queue = user_event_queue_arc.clone();

		let bridge_h = thread::spawn(move || SandBox::immediate(|| {
			let cs = [cs1.clone(),cs2.clone()];
			let mut prev_move:Option<AppliedMove> = None;
			let mut ponders:[Option<AppliedMove>; 2] = [None,None];

			let quit_ready_inner = quit_ready.clone();

			let quit_notification =  move || {
				quit_ready_inner.store(true,Ordering::Release);
			};

			let self_match_event_queue_inner = self_match_event_queue.clone();
			let on_error_handler_inner = on_error_handler.clone();

			let quit_ready_inner = quit_ready.clone();

			let on_gameend = move |win_cs:Sender<SelfMatchMessage>,
									lose_cs:Sender<SelfMatchMessage>,
									cs:[Sender<SelfMatchMessage>; 2],
									sr:&Receiver<SelfMatchMessage>,
									s:SelfMatchGameEndState| {
				let mut message_state = GameEndState::Win;

				let quit_notification = || {
					quit_ready_inner.store(true,Ordering::Release);
				};

				match self_match_event_queue_inner.lock() {
					Ok(mut self_match_event_queue) => {
						self_match_event_queue.push(SelfMatchEvent::GameEnd(s));
					},
					Err(ref e) => {
						on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
						win_cs.send(SelfMatchMessage::Error(0))?;
						lose_cs.send(SelfMatchMessage::Error(1))?;

						quit_notification();

						return Err(SelfMatchRunningError::InvalidState(String::from(
							"Exclusive lock on self_match_event_queue failed."
						)));
					}
				}

				for current_cs in &[win_cs.clone(),lose_cs.clone()] {
					current_cs.send(SelfMatchMessage::GameEnd(message_state))?;
					match sr.recv()? {
						SelfMatchMessage::Ready => (),
						SelfMatchMessage::Error(n) => {
							cs[(n+1)%2].send(SelfMatchMessage::Error((n+1)%2))?;
							cs[n].send(SelfMatchMessage::Error(n))?;
							quit_notification();
							return Err(SelfMatchRunningError::InvalidState(String::from(
								"An error occurred while executing the player thread."
							)));
						},
						SelfMatchMessage::Quit => {
							cs[0].send(SelfMatchMessage::Quit)?;
							cs[1].send(SelfMatchMessage::Quit)?;

							quit_notification();

							return Ok(());
						},
						_ => {
							cs[0].send(SelfMatchMessage::Error(0))?;
							cs[1].send(SelfMatchMessage::Error(1))?;

							quit_notification();

							return Err(SelfMatchRunningError::InvalidState(String::from(
								"An invalid message was sent to the self-match management thread."
							)));
						}
					}
					message_state = GameEndState::Lose;
				}
				Ok(())
			};

			let mut game_count = 0;

			'gameloop: while number_of_games.map_or(true, |n| game_count < n) &&
			  uptime.map_or(true, |t| Instant::now() - start_time < t) {

				cs[0].send(SelfMatchMessage::GameStart)?;
				cs[1].send(SelfMatchMessage::GameStart)?;

				game_count += 1;

				let mut cs_index = if on_before_newgame() {
					1
				} else {
					0
				};

				let sfen = initial_position_creator();
				let (teban, banmen, mc, n, mvs) = match position_parser.parse(&sfen.split(" ").collect::<Vec<&str>>()) {
					Ok(mut position) => match position {
						SystemEvent::Position(teban, p, n, m) => {
							let(banmen,mc) = match p {
								UsiInitialPosition::Startpos => {
									(rule::BANMEN_START_POS.clone(), MochigomaCollections::Pair(HashMap::new(),HashMap::new()))
								},
								UsiInitialPosition::Sfen(ref b,MochigomaCollections::Pair(ref ms,ref mg)) => {
									(b.clone(),MochigomaCollections::Pair(ms.clone(),mg.clone()))
								},
								UsiInitialPosition::Sfen(ref b,MochigomaCollections::Empty) => {
									(b.clone(),MochigomaCollections::Pair(HashMap::new(),HashMap::new()))
								}
							};
							(teban,banmen,mc,n,m)
						},
						e => {
							cs[0].send(SelfMatchMessage::Error(0))?;
							cs[1].send(SelfMatchMessage::Error(1))?;

							quit_notification();

							return Err(SelfMatchRunningError::InvalidState(format!(
								"The type of event passed and the event being processed do not match. (Event kind = {:?})",
								 e.event_kind()
							)));
						}
					},
					Err(ref e) => {
						on_error_handler.lock().map(|h| h.call(e)).is_err();

						cs[0].send(SelfMatchMessage::Error(0))?;
						cs[1].send(SelfMatchMessage::Error(1))?;

						quit_notification();

						return Err(SelfMatchRunningError::InvalidState(String::from(
							"An error occurred parsing the sfen string."
						)));
					}
				};

				match self_match_event_queue.lock() {
					Ok(mut self_match_event_queue) => {
						self_match_event_queue.push(
							SelfMatchEvent::GameStart(if cs_index == 1 {
								2
							} else {
								1
							}, teban, sfen.clone()));
					},
					Err(ref e) => {
						on_error_handler.lock().map(|h| h.call(e)).is_err();
						cs[0].send(SelfMatchMessage::Error(0))?;
						cs[1].send(SelfMatchMessage::Error(1))?;

						quit_notification();

						return Err(SelfMatchRunningError::InvalidState(String::from(
							"Exclusive lock on self_match_event_queue failed."
						)));
					}
				}

				let banmen_at_start = banmen.clone();
				let mc_at_start = mc.clone();
				let teban_at_start = teban.clone();

				let mut current_game_time_limit = [game_time_limit,game_time_limit];
				let mut current_time_limit = current_game_time_limit[cs_index].to_instant(teban);

				let kyokumen_map:KyokumenMap<u64,u32> = KyokumenMap::new();
				let oute_kyokumen_map:KyokumenMap<u64,u32> = KyokumenMap::new();

				let hasher = KyokumenHash::new();

				let (ms,mg) = match mc {
					MochigomaCollections::Pair(ref ms, ref mg) => {
						match teban {
							Teban::Sente => (ms.clone(),mg.clone()),
							Teban::Gote => (mg.clone(),ms.clone()),
						}
					},
					MochigomaCollections::Empty => {
						(HashMap::new(),HashMap::new())
					},
				};

				let (mhash, shash) = hasher.calc_initial_hash(&banmen,&ms,&mg);

				let mut mvs = mvs.into_iter().map(|m| m.to_applied_move()).collect::<Vec<AppliedMove>>();

				let (mut teban,
					 mut state,
					 mut mc,
					 mut mhash,
					 mut shash,
					 mut kyokumen_map,
					 mut oute_kyokumen_map) = Rule::apply_moves(State::new(banmen),
															 	teban,mc,&mvs,
															 	mhash,shash,
															 	kyokumen_map,
															 	oute_kyokumen_map,&hasher);

				while uptime.map_or(true, |t| Instant::now() - start_time < t) {
					match ponders[cs_index] {
						None => {
							cs[cs_index].send(SelfMatchMessage::StartThink(
								teban_at_start.clone(),banmen_at_start.clone(),mc_at_start.clone(),n,mvs.clone()))?;
						},
						pm @ Some(_) if pm == prev_move => {
							cs[cs_index].send(SelfMatchMessage::PonderHit)?;
						},
						_ => {
							cs[cs_index].send(SelfMatchMessage::PonderNG)?;
							cs[cs_index].send(SelfMatchMessage::StartThink(
								teban_at_start.clone(),banmen_at_start.clone(),mc_at_start.clone(),n,mvs.clone()))?;
						}
					}

					let think_start_time = Instant::now();

					let timeout = current_time_limit.map(|cl| uptime.map(|u| {
						if start_time + u < cl {
							start_time + u - Instant::now()
						} else {
							cl - Instant::now()
						}
					}).unwrap_or(cl - Instant::now())).map(|d| after(d)).unwrap_or_else(|| uptime.map(|d| after(d)).unwrap_or(never()));

					let timeout_kind = current_time_limit.map(|cl| uptime.map(|u| {
						if start_time + u < cl {
							TimeoutKind::Uptime
						} else {
							TimeoutKind::Turn
						}
					}).unwrap_or(TimeoutKind::Turn)).unwrap_or(TimeoutKind::Never);

					select! {
						recv(sr) -> message => {
							match message? {
								SelfMatchMessage::NotifyMove(BestMove::Move(m,pm)) => {
									match self_match_event_queue.lock() {
										Ok(mut self_match_event_queue) => {
											self_match_event_queue.push(SelfMatchEvent::Moved(teban,Moved::try_from((&state.get_banmen(),&m))?));
										},
										Err(ref e) => {
											on_error_handler.lock().map(|h| h.call(e)).is_err();
											cs[0].send(SelfMatchMessage::Error(0))?;
											cs[1].send(SelfMatchMessage::Error(1))?;

											quit_notification();

											return Err(SelfMatchRunningError::InvalidState(String::from(
												"Exclusive lock on self_match_event_queue failed."
											)));
										}
									}

									current_game_time_limit[cs_index] = Rule::update_time_limit(
										&current_game_time_limit[cs_index],
										teban,think_start_time.elapsed()
									);
									current_time_limit = current_game_time_limit[cs_index].to_instant(teban);

									let m = m.to_applied_move();

									match Rule::apply_valid_move(&state,teban,&mc,m) {
										Ok((next,nmc,o)) => {

											let is_win = Rule::is_win(&state,teban,m);

											if is_win {
												mvs.push(m);

												kifu_writer(&sfen,&mvs.into_iter()
																		.map(|m| m.to_move())
																		.collect::<Vec<Move>>());
												on_gameend(
													cs[cs_index].clone(),
													cs[(cs_index+1) % 2].clone(),
													[cs[0].clone(),cs[1].clone()],
													&sr,
													SelfMatchGameEndState::Win(teban)
												)?;
												break;
											}

											if let Some(_) = prev_move {
												if Rule::win_only_moves(teban.opposite(),&state).len() > 0 {
													if Rule::win_only_moves(teban.opposite(),&next).len() > 0 {
														on_gameend(
															cs[(cs_index+1) % 2].clone(),
															cs[cs_index].clone(),
															[cs[0].clone(),cs[1].clone()],
															&sr,
															SelfMatchGameEndState::Foul(teban,FoulKind::NotRespondedOute)
														)?;
														mvs.push(m);
														kifu_writer(&sfen,&mvs.into_iter()
																				.map(|m| m.to_move())
																				.collect::<Vec<Move>>());
														break;
													}
												}
											}

											mvs.push(m);

											mhash = hasher.calc_main_hash(mhash,teban,&state.get_banmen(),&mc,m,&o);
											shash = hasher.calc_sub_hash(shash,teban,&state.get_banmen(),&mc,m,&o);

											mc = nmc;
											state = next;

											if Rule::is_put_fu_and_mate(&state,teban,&mc,m) {
												kifu_writer(&sfen,&mvs.into_iter()
																				.map(|m| m.to_move())
																				.collect::<Vec<Move>>());
												on_gameend(
													cs[(cs_index+1) % 2].clone(),
													cs[cs_index].clone(),
													[cs[0].clone(),cs[1].clone()],
													&sr,
													SelfMatchGameEndState::Foul(teban,FoulKind::PutFuAndMate)
												)?;
												break;
											}

											if Rule::is_sennichite_by_oute(
												&state,
												teban,mhash,shash,
												&oute_kyokumen_map
											) {
												kifu_writer(&sfen,&mvs.into_iter()
																		.map(|m| m.to_move())
																		.collect::<Vec<Move>>());
												on_gameend(
													cs[(cs_index+1) % 2].clone(),
													cs[cs_index].clone(),
													[cs[0].clone(),cs[1].clone()],
													&sr,
													SelfMatchGameEndState::Foul(teban,FoulKind::SennichiteOu)
												)?;
												break;
											}

											Rule::update_sennichite_by_oute_map(
												&state,
												teban,mhash,shash,
												&mut oute_kyokumen_map
											);

											if Rule::is_sennichite(
												&state,teban,mhash,shash,&kyokumen_map
											) {
												kifu_writer(&sfen,&mvs.into_iter()
																		.map(|m| m.to_move())
																		.collect::<Vec<Move>>());
												on_gameend(
													cs[(cs_index+1) % 2].clone(),
													cs[cs_index].clone(),
													[cs[0].clone(),cs[1].clone()],
													&sr,
													SelfMatchGameEndState::Foul(teban,FoulKind::Sennichite)
												)?;
												break;
											}

											Rule::update_sennichite_map(
												&state,teban,mhash,shash,&mut kyokumen_map
											);

											teban = teban.opposite();

											ponders[cs_index] = pm.map(|pm| pm.to_applied_move());

											match pm {
												Some(pm) => {
													match mvs.clone() {
														mut mvs => {
															mvs.push(pm.to_applied_move());
															cs[cs_index].send(
																SelfMatchMessage::StartPonderThink(
																	teban_at_start.clone(),banmen_at_start.clone(),
																	mc_at_start.clone(),n,mvs))?;
														}
													}
												},
												None => (),
											}

											cs_index = (cs_index + 1) % 2;
										},
										Err(_) => {
											mvs.push(m);
											kifu_writer(&sfen,&mvs.into_iter()
																	.map(|m| m.to_move())
																	.collect::<Vec<Move>>());
											on_gameend(
												cs[(cs_index+1) % 2].clone(),
												cs[cs_index].clone(),
												[cs[0].clone(),cs[1].clone()],
												&sr,
												SelfMatchGameEndState::Foul(teban,FoulKind::InvalidMove)
											)?;
											break;
										}
									}
									prev_move = Some(m)
								},
								SelfMatchMessage::NotifyMove(BestMove::Resign) => {
									kifu_writer(&sfen,&mvs.into_iter()
															.map(|m| m.to_move())
															.collect::<Vec<Move>>());
									on_gameend(
										cs[(cs_index+1) % 2].clone(),
										cs[cs_index].clone(),
										[cs[0].clone(),cs[1].clone()],
										&sr,
										SelfMatchGameEndState::Resign(teban)
									)?;
									break;
								},
								SelfMatchMessage::NotifyMove(BestMove::Abort) => {
									match self_match_event_queue.lock() {
										Ok(mut self_match_event_queue) => {
											self_match_event_queue.push(SelfMatchEvent::Abort);
										},
										Err(ref e) => {
											on_error_handler.lock().map(|h| h.call(e)).is_err();
											cs[0].send(SelfMatchMessage::Error(0))?;
											cs[1].send(SelfMatchMessage::Error(1))?;

											quit_notification();

											return Err(SelfMatchRunningError::InvalidState(String::from(
												"Exclusive lock on self_match_event_queue failed."
											)));
										}
									}
									break;
								},
								SelfMatchMessage::NotifyMove(BestMove::Win) if Rule::is_nyugyoku_win(&state,teban,&mc,&current_time_limit)=> {
									kifu_writer(&sfen,&mvs.into_iter()
															.map(|m| m.to_move())
															.collect::<Vec<Move>>());
									on_gameend(
										cs[cs_index].clone(),
										cs[(cs_index+1) % 2].clone(),
										[cs[0].clone(),cs[1].clone()],
										&sr,
										SelfMatchGameEndState::NyuGyokuWin(teban)
									)?;
									break;
								},
								SelfMatchMessage::NotifyMove(BestMove::Win) => {
									kifu_writer(&sfen,&mvs.into_iter()
															.map(|m| m.to_move())
															.collect::<Vec<Move>>());
									on_gameend(
										cs[(cs_index+1) % 2].clone(),
										cs[cs_index].clone(),
										[cs[0].clone(),cs[1].clone()],
										&sr,
										SelfMatchGameEndState::NyuGyokuLose(teban)
									)?;
									break;
								},
								SelfMatchMessage::Error(n) => {
									cs[(n+1)%2].send(SelfMatchMessage::Error((n+1)%2))?;
									cs[n].send(SelfMatchMessage::Error(n))?;
									quit_notification();
									return Err(SelfMatchRunningError::InvalidState(String::from(
										"An error occurred while executing the player thread."
									)));
								},
								SelfMatchMessage::Quit => {
									cs[0].send(SelfMatchMessage::Quit)?;
									cs[1].send(SelfMatchMessage::Quit)?;

									quit_notification();

									return Ok(SelfMatchResult {
										game_count: game_count,
										elapsed: start_time.elapsed(),
										start_dt:start_dt,
										end_dt:Local::now(),
									});
								},
								_ => {
									cs[0].send(SelfMatchMessage::Error(0))?;
									cs[1].send(SelfMatchMessage::Error(1))?;

									quit_notification();
									return Err(SelfMatchRunningError::InvalidState(String::from(
										"An invalid message was sent to the self-match management thread."
									)));
								}
							}
						},
						recv(timeout) -> message => {
							match message? {
								_ => {
									match timeout_kind {
										TimeoutKind::Turn => {
											kifu_writer(&sfen,&mvs.into_iter().map(|m| m.to_move()).collect::<Vec<Move>>());
											match user_event_queue[cs_index].lock() {
												Ok(mut user_event_queue) => {
													user_event_queue.push(UserEvent::Stop);
												},
												Err(ref e) => {
													on_error_handler.lock().map(|h| h.call(e)).is_err();
												}
											}

											match sr.recv()? {
												SelfMatchMessage::NotifyMove(_) => {
													on_gameend(
													cs[(cs_index+1) % 2].clone(),
													cs[cs_index].clone(),
													[cs[0].clone(),cs[1].clone()],
													&sr,
													SelfMatchGameEndState::Timeover(teban))?;

												},
												_ => {
													return Err(SelfMatchRunningError::InvalidState(String::from(
														"An invalid message was sent to the self-match management thread."
													)));
												}
											}

											break;
										},
										TimeoutKind::Uptime => {
											match user_event_queue[cs_index].lock() {
												Ok(mut user_event_queue) => {
													user_event_queue.push(UserEvent::Stop);
												},
												Err(ref e) => {
													on_error_handler.lock().map(|h| h.call(e)).is_err();
												}
											}

											match sr.recv()? {
												SelfMatchMessage::NotifyMove(_) => {
													break 'gameloop;
												},
												_ => {
													return Err(SelfMatchRunningError::InvalidState(String::from(
														"An invalid message was sent to the self-match management thread."
													)));
												}
											}
										},
										_ => (),
									}
								}
							}
						}
					}
				}
			}

			cs[0].send(SelfMatchMessage::Quit)?;
			cs[1].send(SelfMatchMessage::Quit)?;

			quit_notification();

			Ok(SelfMatchResult {
				game_count: game_count,
				elapsed: start_time.elapsed(),
				start_dt:start_dt,
				end_dt:Local::now()
			})
		}, on_error_handler.clone()).map_err(|e| {
			match e {
				SelfMatchRunningError::SendError(SendError(SelfMatchMessage::Error(n))) => {
					let r = if n == 0 {
						cs1.send(SelfMatchMessage::Error(0))
					} else {
						cs2.send(SelfMatchMessage::Error(1))
					};
					if let Err(ref e) = r {
						on_error_handler.lock().map(|h| h.call(e)).is_err();
					}
				},
				_ => {
					if let Err(ref e) = cs1.send(SelfMatchMessage::Error(0)) {
						on_error_handler.lock().map(|h| h.call(e)).is_err();
					}
					if let Err(ref e) = cs2.send(SelfMatchMessage::Error(1)) {
						on_error_handler.lock().map(|h| h.call(e)).is_err();
					}
				}
			}
			quit_ready.store(true,Ordering::Release);
			e
		}));

		let players = [self.player1.clone(),self.player2.clone()];
		let mut handlers:Vec<JoinHandle<Result<(),SelfMatchRunningError>>> = Vec::with_capacity(2);

		for i in 0..2 {
			let cr = cr.remove(0);
			let player = players[i].clone();
			let on_error_handler = on_error_handler_arc.clone();
			let logger = logger_arc.clone();
			let user_event_queue = [user_event_queue_arc[0].clone(),user_event_queue_arc[1].clone()];
			let info_sender = info_sender_arc.clone();
			let limit = self.game_time_limit;

			let ss = ss.clone();

			let player_i = i;

			handlers.push(thread::spawn(move || SandBox::immediate(|| {
				loop {
					match cr.recv()? {
						SelfMatchMessage::GameStart => {
							match player.lock() {
								Ok(mut player) => {
									match player.take_ready() {
										Ok(_) => (),
										Err(ref e) => {
											on_error_handler.lock().map(|h| h.call(e)).is_err();
											ss.send(SelfMatchMessage::Error(player_i))?;
											continue;
										}
									}
									match player.newgame() {
										Ok(_) => (),
										Err(ref e) => {
											on_error_handler.lock().map(|h| h.call(e)).is_err();
											ss.send(SelfMatchMessage::Error(player_i))?;
											continue;
										}
									}
								},
								Err(ref e) => {
									on_error_handler.lock().map(|h| h.call(e)).is_err();
									ss.send(SelfMatchMessage::Error(player_i))?;
									continue;
								}
							}

							loop {
								match cr.recv()? {
									SelfMatchMessage::StartThink(t,b,mc,n,m) => {
										let (mut ms, mut mg) = match mc {
											MochigomaCollections::Pair(ref ms, ref mg) => {
												(ms.clone(),mg.clone())
											},
											MochigomaCollections::Empty => {
												(HashMap::new(),HashMap::new())
											}
										};

										match player.lock() {
											Ok(mut player) => {
												match player.set_position(t, b, ms, mg, n, m.into_iter().map(|m| {
													m.to_move()
												}).collect::<Vec<Move>>()) {
													Ok(_) => (),
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i))?;
														break;
													}
												}
												let m = match player.think(&limit,
															user_event_queue[player_i].clone(),
															info_sender.clone(),on_error_handler.clone()) {
													Ok(m) => m,
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i))?;
														break;
													}
												};
												ss.send(SelfMatchMessage::NotifyMove(m))?;
											},
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
												ss.send(SelfMatchMessage::Error(player_i))?;
												break;
											}
										};
									},
									SelfMatchMessage::StartPonderThink(t,b,mc,n,m) => {
										let (mut ms, mut mg) = match mc {
											MochigomaCollections::Pair(ref ms, ref mg) => {
												(ms.clone(),mg.clone())
											},
											MochigomaCollections::Empty => {
												(HashMap::new(),HashMap::new())
											}
										};

										let m = match player.lock() {
											Ok(mut player) => {
												match player.set_position(t, b, ms, mg, n, m.into_iter().map(|m| {
													m.to_move()
												}).collect::<Vec<Move>>()) {
													Ok(_) => (),
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i))?;
														break;
													}
												}
												match player.think(&limit,
															user_event_queue[player_i].clone(),
															info_sender.clone(),on_error_handler.clone()) {
													Ok(m) => m,
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i))?;
														break;
													}
												}
											},
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
												ss.send(SelfMatchMessage::Error(player_i))?;
												break;
											}
										};

										match cr.recv()? {
											SelfMatchMessage::PonderHit => {
												ss.send(SelfMatchMessage::NotifyMove(m))?;
											},
											SelfMatchMessage::PonderNG => (),
											SelfMatchMessage::Quit => {
												match player.lock() {
													Ok(mut player) => {
														match player.quit(){
															Ok(()) => (),
															Err(ref e) => {
																on_error_handler.lock().map(|h| h.call(e)).is_err();
															}
														}
													},
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
													}
												}
												return Ok(());
											},
											SelfMatchMessage::Error(_) => {
												return Ok(());
											}
											_ => {
												logger.lock().map(|mut logger| {
													logger.logging(&format!("Invalid message."))
												}).map_err(|_| {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
													false
												}).is_err();
												ss.send(SelfMatchMessage::Error(player_i))?;
												break;
											}
										}
									},
									SelfMatchMessage::GameEnd(s) => {
										match player.lock() {
											Ok(mut player) => {
												match player.gameover(&s,user_event_queue[player_i].clone(),
																on_error_handler.clone()) {
													Ok(()) => (),
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i))?;
														break;
													}
												};
											},
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
												ss.send(SelfMatchMessage::Error(player_i))?;
												break;
											}
										}
										ss.send(SelfMatchMessage::Ready)?;
										break;
									},
									SelfMatchMessage::Quit => {
										match player.lock() {
											Ok(mut player) => {
												match player.quit(){
													Ok(()) => (),
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
													}
												}
											},
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
											}
										}
										return Ok(());
									},
									SelfMatchMessage::Error(_) => {
										return Ok(());
									},
									_ => {
										logger.lock().map(|mut logger| {
											logger.logging(&format!("Invalid message."))
										}).map_err(|_| {
											USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
											false
										}).is_err();
										ss.send(SelfMatchMessage::Error(player_i))?;
										break;
									}
								}
							}
						},
						SelfMatchMessage::Quit => {
							match player.lock() {
								Ok(mut player) => {
									match player.quit(){
										Ok(()) => (),
										Err(ref e) => {
											on_error_handler.lock().map(|h| h.call(e)).is_err();
										}
									}
								},
								Err(ref e) => {
									on_error_handler.lock().map(|h| h.call(e)).is_err();
								}
							}

							return Ok(());
						},
						SelfMatchMessage::Error(_) => {
							return Ok(());
						},
						_ => {
							logger.lock().map(|mut logger| {
								logger.logging(&format!("Invalid message."))
							}).map_err(|_| {
								USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
								false
							}).is_err();
							ss.send(SelfMatchMessage::Error(player_i))?;
						}
					}
				}
			}, on_error_handler.clone()).map_err(|e| {
				match e {
					SelfMatchRunningError::SendError(SendError(_)) |
						SelfMatchRunningError::RecvError(_) => (),
					_ => {
						if let Err(ref e) = ss.send(SelfMatchMessage::Error(player_i)) {
							on_error_handler.lock().map(|h| h.call(e)).is_err();
						}
					}
				}
				e
			})));
		}

		let delay = Duration::from_millis(50);
		let on_error_handler = on_error_handler_arc.clone();
		let self_match_event_queue = self_match_event_queue_arc.clone();
		let logger = logger_arc.clone();
		let quit_ready = quit_ready_arc.clone();

		thread::spawn(move || {
			while !quit_ready.load(Ordering::Acquire) {
				match input_reader.read() {
					Ok(line) => {
						match input_handler(line) {
							Ok(false) => {
								return;
							},
							Err(ref e) => {
								on_error_handler.lock().map(|h| h.call(e)).is_err();
								return;
							},
							_ => {
								return;
							},
						}
					},
					Err(ref e) if !quit_ready.load(Ordering::Acquire) => {
						on_error_handler.lock().map(|h| h.call(e)).is_err();
						return;
					},
					_ => (),
				}
			}
		});

		let on_error_handler = on_error_handler_arc.clone();

		let quit_ready = quit_ready_arc.clone();

		while !quit_ready.load(Ordering::Acquire) || (match self.system_event_queue.lock() {
			Ok(system_event_queue) => system_event_queue.has_event(),
			Err(ref e) => {
				on_error_handler.lock().map(|h| h.call(e)).is_err();
				false
			}
		}) || (match self_match_event_queue.lock() {
			Ok(self_match_event_queue) => self_match_event_queue.has_event(),
			Err(ref e) => {
				on_error_handler.lock().map(|h| h.call(e)).is_err();
				false
			}
		}) {
			match system_event_dispatcher.dispatch_events(self, &*self.system_event_queue) {
				Ok(_) => true,
				Err(ref e) => {
					on_error_handler.lock().map(|h| h.call(e)).is_err()
				}
			};
			match self_match_event_dispatcher.dispatch_events(self, &*self_match_event_queue) {
				Ok(_) => true,
				Err(ref e) => {
					on_error_handler.lock().map(|h| h.call(e)).is_err()
				}
			};
			thread::sleep(delay);
		}

		let mut has_error = false;

		let result = bridge_h.join().map_err(|_| {
			has_error = true;
			logger.lock().map(|mut logger| {
				logger.logging(&format!("Main thread join failed."))
			}).map_err(|_| {
				USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
				false
			}).is_err();
		}).unwrap_or(Err(SelfMatchRunningError::ThreadJoinFailed(String::from(
			"Main thread join failed."
		)))).map_err(|e| {
			has_error = true;
			on_error_handler.lock().map(|h| h.call(&e)).is_err();
			e
		});

		for h in handlers {
			h.join().map_err(|_| {
				has_error = true;
				logger.lock().map(|mut logger| {
					logger.logging(&format!("Sub thread join failed."))
				}).map_err(|_| {
					USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
					false
				}).is_err();
			}).map(|r| {
				r.map_err(|e| {
					has_error = true;
					on_error_handler.lock().map(|h| h.call(&e)).is_err();
					e
				}).is_err()
			}).is_err();
		}

		if has_error {
			Err(SelfMatchRunningError::Fail(String::from(
				"An error occurred while executing a self match. Please see the log for details ..."
			)))
		} else {
			result
		}
	}
}