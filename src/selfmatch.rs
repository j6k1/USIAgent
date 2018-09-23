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

use std::error::Error;
use std::fmt;
use std::{thread};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SendError;
use std::thread::JoinHandle;
use std::marker::Send;
use std::marker::PhantomData;
use std::sync::mpsc;
use std::time::{Instant,Duration};
use std::collections::HashMap;
use std::io::Write;
use std::io::BufWriter;
use std::fs;
use std::fs::OpenOptions;

pub trait SelfMatchKifuWriter<OE> where OE: Error + fmt::Debug {
	fn write(&mut self,initial_sfen:&String,m:&Vec<Move>) -> Result<(),OE>;
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
	pub fn new(file:String) -> Result<FileSfenKifuWriter,SelfMatchRunningError> {
		Ok(FileSfenKifuWriter {
			writer:BufWriter::new(OpenOptions::new().append(true).create(true).open(file)?),
		})
	}
}
impl SelfMatchKifuWriter<SelfMatchRunningError> for FileSfenKifuWriter {
	fn write(&mut self,initial_sfen:&String,m:&Vec<Move>) -> Result<(),SelfMatchRunningError> {
		let sfen = match self.to_sfen(initial_sfen,m) {
			Err(ref e) => {
				return Err(SelfMatchRunningError::InvalidState(e.to_string()));
			},
			Ok(sfen) => sfen,
		};

		match self.writer.write(format!("{}\n",sfen).as_bytes()) {
			Err(ref e) => {
				Err(SelfMatchRunningError::InvalidState(e.to_string()))
			},
			Ok(_) => Ok(()),
		}
	}
}
#[derive(Debug)]
pub enum SelfMatchMessage {
	Ready,
	GameStart,
	StartThink(Teban,Banmen,MochigomaCollections,u32,Vec<Move>),
	StartPonderThink(Teban,Banmen,MochigomaCollections,u32,Vec<Move>),
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
			EventHandlerError<SystemEventKind, E>: From<E>,
			S: InfoSender,
			Arc<Mutex<S>>: Send + 'static {
	player_error_type:PhantomData<E>,
	player1:Arc<Mutex<T>>,
	player2:Arc<Mutex<T>>,
	info_sender:Arc<Mutex<S>>,
	game_time_limit:UsiGoTimeLimit,
	end_time:Option<Duration>,
	number_of_games:Option<u32>,
	pub system_event_queue:Arc<Mutex<EventQueue<SystemEvent,SystemEventKind>>>,
}
impl<T,E,S> SelfMatchEngine<T,E,S>
	where T: USIPlayer<E> + fmt::Debug, Arc<Mutex<T>>: Send + 'static,
			E: PlayerError,
			EventHandlerError<SystemEventKind, E>: From<E>,
			S: InfoSender,
			Arc<Mutex<S>>: Send + 'static {
	pub fn new(player1:T,player2:T,
				info_sender:Arc<Mutex<S>>,
				game_time_limit:UsiGoTimeLimit,
				end_time:Option<Duration>,number_of_games:Option<u32>)
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
			end_time:end_time,
			number_of_games:number_of_games,
			system_event_queue:Arc::new(Mutex::new(EventQueue::new())),
		}
	}

	pub fn start_default<I,F,RH,OE,KW,EH>(&mut self, on_init_event_dispatcher:I,
						on_before_newgame:F,
						initial_position_creator:Option<Box<FnMut() -> String + Send + 'static>>,
						kifu_writer:Option<KW>,
						input_handler:RH,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						on_error:EH) -> Result<SelfMatchResult,SelfMatchRunningError>
		where F: FnMut() -> bool + Send + 'static,
				RH: FnMut(String) -> Result<(),SelfMatchRunningError> + Send + 'static,
				OE: Error + fmt::Debug,
				KW:SelfMatchKifuWriter<OE> + Send + 'static,
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

	pub fn start_with_log_path<I,F,RH,OE,KW,EH>(&mut self,path:String,
						on_init_event_dispatcher:I,
						on_before_newgame:F,
						initial_position_creator:Option<Box<FnMut() -> String + Send + 'static>>,
						kifu_writer:Option<KW>,
						input_handler:RH,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						mut on_error:EH) -> Result<SelfMatchResult,SelfMatchRunningError>
		where F: FnMut() -> bool + Send + 'static,
				RH: FnMut(String) -> Result<(),SelfMatchRunningError> + Send + 'static,
				OE: Error + fmt::Debug,
				KW:SelfMatchKifuWriter<OE> + Send + 'static,
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

	pub fn start<I,F,R,RH,OE,KW,L,EH>(&mut self, on_init_event_dispatcher:I,
						on_before_newgame:F,
						initial_position_creator:Option<Box<FnMut() -> String + Send + 'static>>,
						kifu_writer:Option<KW>,
						input_reader:R,
						input_handler:RH,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						logger:L, mut on_error:EH) -> Result<SelfMatchResult,SelfMatchRunningError>
		where F: FnMut() -> bool + Send + 'static,
				R: USIInputReader + Send + 'static,
				RH: FnMut(String) -> Result<(),SelfMatchRunningError> + Send + 'static,
				OE: Error + fmt::Debug,
				KW:SelfMatchKifuWriter<OE> + Send + 'static,
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

	fn run<I,F,R,RH,OE,KW,L>(&mut self, mut on_init_event_dispatcher:I,
						mut on_before_newgame:F,
						initial_position_creator:Option<Box<FnMut() -> String + Send + 'static>>,
						kifu_writer:Option<KW>,
						mut input_reader:R,
						mut input_handler:RH,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						logger_arc:Arc<Mutex<L>>,
						on_error_handler_arc:Arc<Mutex<OnErrorHandler<L>>>) -> Result<SelfMatchResult,SelfMatchRunningError>
		where F: FnMut() -> bool + Send + 'static,
				R: USIInputReader + Send + 'static,
				RH: FnMut(String) -> Result<(),SelfMatchRunningError> + Send + 'static,
				OE: Error + fmt::Debug,
				KW:SelfMatchKifuWriter<OE> + Send + 'static,
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
														SelfMatchEngine<T, E, S>,L,E>::new(&logger_arc);

		on_init_event_dispatcher(&mut self_match_event_dispatcher);

		let mut system_event_dispatcher:USIEventDispatcher<SystemEventKind,
														SystemEvent,SelfMatchEngine<T, E, S>,L,E> = USIEventDispatcher::new(&logger_arc);

		let user_event_queue_arc:[Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>; 2] = [Arc::new(Mutex::new(EventQueue::new())),Arc::new(Mutex::new(EventQueue::new()))];

		let user_event_queue = [user_event_queue_arc[0].clone(),user_event_queue_arc[1].clone()];

		let mut initial_position_creator:Box<FnMut() -> String + Send + 'static> =
			initial_position_creator.map_or(Box::new(|| String::from("startpos")), |f| {
				f
			});

		let on_error_handler = on_error_handler_arc.clone();

		let mut kifu_writer:Box<FnMut(&String,&Vec<Move>) +Send + 'static> =
			kifu_writer.map_or(Box::new(|_,_| ()), |mut w| Box::new(move |sfen,m| {
				w.write(sfen,m).map_err(|e| {
					on_error_handler.lock().map(|h| h.call(&e)).is_err();
				}).is_err();
			}));

		let quit_ready_arc = Arc::new(Mutex::new(false));

		let notify_quit_arc = Arc::new(Mutex::new(false));
		let notify_quit = notify_quit_arc.clone();

		let on_error_handler = on_error_handler_arc.clone();

		system_event_dispatcher.add_handler(SystemEventKind::Quit, move |_,e| {
			match e {
				&SystemEvent::Quit => {
					match notify_quit.lock() {
						Ok(mut notify_quit) => {
							*notify_quit = true;
						},
						Err(ref e) => {
							on_error_handler.lock().map(|h| h.call(e)).is_err();
						}
					};
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
					Ok(())
				},
				e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		});

		let on_error_handler = on_error_handler_arc.clone();

		let self_match_event_queue:EventQueue<SelfMatchEvent,SelfMatchEventKind> = EventQueue::new();
		let self_match_event_queue_arc = Arc::new(Mutex::new(self_match_event_queue));

		let info_sender_arc = self.info_sender.clone();

		let (ss,sr) = mpsc::channel();
		let (cs1,cr1) = mpsc::channel();
		let (cs2,cr2) = mpsc::channel();
		let mut cr = vec![cr1,cr2];

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
		let notify_quit = notify_quit_arc.clone();
		let quit_ready = quit_ready_arc.clone();

		let on_error_handler = on_error_handler_arc.clone();

		let end_time = self.end_time.map(|t| t);
		let number_of_games = self.number_of_games.map(|n| n);
		let game_time_limit = self.game_time_limit;

		let bridge_h = thread::spawn(move || SandBox::immediate(|| {
			let cs = [cs1.clone(),cs2.clone()];
			let mut prev_move:Option<Move> = None;
			let mut ponders:[Option<Move>; 2] = [None,None];

			let on_error_handler_inner = on_error_handler.clone();
			let quit_ready_inner = quit_ready.clone();

			let quit_notification =  move || {
				match quit_ready_inner.lock() {
					Ok(mut quit_ready) => {
						*quit_ready = true;
					},
					Err(ref e) => {
						on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
					}
				};
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

				let quit_notification =  || {
					match quit_ready_inner.lock() {
						Ok(mut quit_ready) => {
							*quit_ready = true;
						},
						Err(ref e) => {
							on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
						}
					};
				};

				match self_match_event_queue_inner.lock() {
					Ok(mut self_match_event_queue) => {
						self_match_event_queue.push(SelfMatchEvent::GameEnd(s));
					},
					Err(ref e) => {
						on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
						win_cs.send(SelfMatchMessage::Error(0))?;
						lose_cs.send(SelfMatchMessage::Error(1))?;

						match quit_ready_inner.lock() {
							Ok(mut quit_ready) => {
								*quit_ready = true;
							},
							Err(ref e) => {
								on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
							}
						};

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

			while number_of_games.map_or(true, |n| game_count < n) &&
			  end_time.map_or(true, |t| Instant::now() - start_time < t) && !(match notify_quit.lock() {
				Ok(notify_quit) => {
					*notify_quit
				},
				Err(ref e) => {
					on_error_handler.lock().map(|h| h.call(e)).is_err();
					true
				}
			}) {

				cs[0].send(SelfMatchMessage::GameStart)?;
				cs[1].send(SelfMatchMessage::GameStart)?;

				game_count += 1;

				let mut cs_index = if on_before_newgame() {
					1
				} else {
					0
				};

				let sfen = initial_position_creator();
				let (teban, banmen, mc, n, mut mvs) = match position_parser.parse(&sfen.split(" ").collect::<Vec<&str>>()) {
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
								1
							} else {
								2
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

				let kyokumen_hash_map:TwoKeyHashMap<u64,u32> = TwoKeyHashMap::new();
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

				let (mut teban,
					 mut banmen,
					 mut mc,
					 mut mhash,
					 mut shash,
					 mut kyokumen_hash_map) = Rule::apply_moves(&banmen,teban,mc,&mvs,mhash,shash,kyokumen_hash_map,&hasher);

				let mut oute_kyokumen_hash_maps:[Option<TwoKeyHashMap<u64,u32>>; 2] = [None,None];

				while end_time.map_or(true, |t| Instant::now() - start_time < t)  && !(match notify_quit.lock() {
					Ok(notify_quit) => {
						*notify_quit
					},
					Err(ref e) => {
						on_error_handler.lock().map(|h| h.call(e)).is_err();
						true
					}
				}) {
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

					match sr.recv()? {
						SelfMatchMessage::NotifyMove(BestMove::Move(m,pm)) => {
							match self_match_event_queue.lock() {
								Ok(mut self_match_event_queue) => {
									self_match_event_queue.push(SelfMatchEvent::Moved(teban,Moved::try_from((&banmen,&m))?));
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

							if let Some(limit) = current_time_limit {
								if limit < Instant::now() {
									kifu_writer(&sfen,&mvs);
									on_gameend(
										cs[(cs_index+1) % 2].clone(),
										cs[cs_index].clone(),
										[cs[0].clone(),cs[1].clone()],
										&sr,
										SelfMatchGameEndState::Timeover(teban))?;
									break;
								}
							}

							current_game_time_limit[cs_index] = Rule::update_time_limit(
								&current_game_time_limit[cs_index],
								teban,think_start_time.elapsed()
							);
							current_time_limit = current_game_time_limit[cs_index].to_instant(teban);

							match Rule::apply_valid_move(&banmen,&teban,&mc,&m) {
								Ok((next,nmc,o)) => {

									if let Some(_) = prev_move {
										if Rule::win_only_moves(&teban.opposite(),&banmen).len() > 0 {
											if Rule::win_only_moves(&teban.opposite(),&next).len() > 0 {
												on_gameend(
													cs[(cs_index+1) % 2].clone(),
													cs[cs_index].clone(),
													[cs[0].clone(),cs[1].clone()],
													&sr,
													SelfMatchGameEndState::Foul(teban,FoulKind::NotRespondedOute)
												)?;
												mvs.push(m);
												kifu_writer(&sfen,&mvs);
												break;
											}
										}
									}

									let is_win = Rule::is_win(&banmen,&teban,&m);

									mvs.push(m);

									if is_win {
										kifu_writer(&sfen,&mvs);
										on_gameend(
											cs[cs_index].clone(),
											cs[(cs_index+1) % 2].clone(),
											[cs[0].clone(),cs[1].clone()],
											&sr,
											SelfMatchGameEndState::Win(teban)
										)?;
										break;
									}

									mhash = hasher.calc_main_hash(mhash,&teban,&banmen,&mc,&m,&o);
									shash = hasher.calc_sub_hash(shash,&teban,&banmen,&mc,&m,&o);

									mc = nmc;
									teban = teban.opposite();

									banmen = next;

									if Rule::is_put_fu_and_mate(&banmen,&teban.opposite(),&mc,&m) {
										kifu_writer(&sfen,&mvs);
										on_gameend(
											cs[(cs_index+1) % 2].clone(),
											cs[cs_index].clone(),
											[cs[0].clone(),cs[1].clone()],
											&sr,
											SelfMatchGameEndState::Foul(teban.opposite(),FoulKind::PutFuAndMate)
										)?;
										break;
									}

									if !Rule::check_sennichite_by_oute(
										&banmen,
										&teban.opposite(),mhash,shash,
										&mut oute_kyokumen_hash_maps[cs_index]
									) {
										kifu_writer(&sfen,&mvs);
										on_gameend(
											cs[(cs_index+1) % 2].clone(),
											cs[cs_index].clone(),
											[cs[0].clone(),cs[1].clone()],
											&sr,
											SelfMatchGameEndState::Foul(teban.opposite(),FoulKind::SennichiteOu)
										)?;
										break;
									}

									if !Rule::check_sennichite(
										&banmen,mhash,shash,&mut kyokumen_hash_map
									) {
										kifu_writer(&sfen,&mvs);
										on_gameend(
											cs[(cs_index+1) % 2].clone(),
											cs[cs_index].clone(),
											[cs[0].clone(),cs[1].clone()],
											&sr,
											SelfMatchGameEndState::Foul(teban.opposite(),FoulKind::Sennichite)
										)?;
										break;
									}

									ponders[cs_index] = pm;

									match pm {
										Some(pm) => {
											match mvs.clone() {
												mut mvs => {
													mvs.push(pm);
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
									kifu_writer(&sfen,&mvs);
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
							kifu_writer(&sfen,&mvs);
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
						SelfMatchMessage::NotifyMove(BestMove::Win) if Rule::is_nyugyoku_win(&banmen,&teban,&mc,&current_time_limit)=> {
							kifu_writer(&sfen,&mvs);
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
							kifu_writer(&sfen,&mvs);
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
			match quit_ready.lock() {
				Ok(mut quit_ready) => {
					*quit_ready = true;
				},
				Err(ref e) => {
					on_error_handler.lock().map(|h| h.call(e)).is_err();
				}
			};
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
												match player.set_position(t, b, ms, mg, n, m) {
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

										match player.lock() {
											Ok(mut player) => {
												match player.set_position(t, b, ms, mg, n, m) {
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

												match cr.recv()? {
													SelfMatchMessage::PonderHit => {
														ss.send(SelfMatchMessage::NotifyMove(m))?;
													},
													SelfMatchMessage::PonderNG => (),
													SelfMatchMessage::Quit | SelfMatchMessage::Error(_) => {
														break;
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
											},
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
												ss.send(SelfMatchMessage::Error(player_i))?;
												break;
											}
										};
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
									SelfMatchMessage::Quit | SelfMatchMessage::Error(_) => {
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
						SelfMatchMessage::Quit | SelfMatchMessage::Error(_) => {
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

		thread::spawn(move || {
			loop {
				match input_reader.read() {
					Ok(line) => {
						if let Err(ref e) = input_handler(line) {
							on_error_handler.lock().map(|h| h.call(e)).is_err();
							return;
						}
					},
					Err(ref e) => {
						on_error_handler.lock().map(|h| h.call(e)).is_err();
						return;
					}
				}
			}
		});

		let on_error_handler = on_error_handler_arc.clone();

		let quit_ready = quit_ready_arc.clone();

		while !(match quit_ready.lock() {
			Ok(quit_ready) => *quit_ready,
			Err(ref e) => {
				on_error_handler.lock().map(|h| h.call(e)).is_err();
				true
			}
		}) || (match self.system_event_queue.lock() {
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