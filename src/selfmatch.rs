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
use TryToString;
use shogi;

use std::error::Error;
use std::fmt;
use std::{thread};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::marker::Send;
use std::marker::PhantomData;
use std::sync::mpsc;
use std::time::{Instant,Duration};
use std::collections::HashMap;

pub trait SelfMatchKifuWriter<OE> where OE: Error + fmt::Debug {
	fn write(&mut self,initial_sfen:&String,m:&Vec<Move>) -> Result<(),OE>;
	fn to_sfen(&self,initial_sfen:&String,m:&Vec<Move>)
		-> Result<String, SfenStringConvertError> {

		let sfen = initial_sfen.split(" ").collect::<Vec<&str>>();

		if sfen.len() >= 5 {
			match (sfen[0],sfen[1],sfen[2],sfen[3],sfen[4]) {
				("sfen",p1,p2,p3,p4) => {
					Ok(format!("sfen {} {} {} {} moves {}",p1,p2,p3,p4,m.try_to_string()?))
				},
				("startpos",_,_,_,_) => {
					Ok(format!("startpos moves {}",m.try_to_string()?))
				},
				_ => {
					Err(SfenStringConvertError::InvalidFormat(initial_sfen.clone()))
				}
			}
		} else if sfen.len() >= 1 && sfen[0] == "startpos" {
			Ok(format!("startpos moves {}",m.try_to_string()?))
		} else {
			Err(SfenStringConvertError::InvalidFormat(initial_sfen.clone()))
		}
	}
}
#[derive(Debug)]
pub enum SelfMatchMessage {
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
	silent:bool,
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
				end_time:Option<Duration>,number_of_games:Option<u32>,
				silent:bool)
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
			silent:silent,
			system_event_queue:Arc::new(Mutex::new(EventQueue::new())),
		}
	}

	pub fn start_default<F,RH,C,OE,KW,EH>(&mut self,on_before_newgame:F,
						initial_position_creator:Option<C>,
						kifu_writer:Option<KW>,
						input_handler:RH,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						self_match_event_dispatcher:USIEventDispatcher<
																SelfMatchEventKind,
																SelfMatchEvent,
																SelfMatchEngine<T, E, S>,FileLogger,E>,
						on_error:EH) -> Result<(),SelfMatchRunningError>
		where F: FnMut() -> bool + Send + 'static,
				RH: FnMut(String) + Send + 'static,
				C: FnMut() -> String + Send + 'static,
				OE: Error + fmt::Debug,
				KW:SelfMatchKifuWriter<OE> + Send + 'static,
				Arc<Mutex<FileLogger>>: Send + 'static,
				EH: FnMut(Option<Arc<Mutex<OnErrorHandler<FileLogger>>>>,
					&SelfMatchRunningError) {
		self.start_with_log_path(String::from("logs/log.txt"),
								on_before_newgame,
								initial_position_creator,
								kifu_writer, input_handler,
								player1_options, player2_options,
								self_match_event_dispatcher, on_error)
	}

	pub fn start_with_log_path<F,RH,C,OE,KW,EH>(&mut self,path:String,
						on_before_newgame:F,
						initial_position_creator:Option<C>,
						kifu_writer:Option<KW>,
						input_handler:RH,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						self_match_event_dispatcher:USIEventDispatcher<
																SelfMatchEventKind,
																SelfMatchEvent,
																SelfMatchEngine<T, E, S>,FileLogger,E>,
						mut on_error:EH) -> Result<(),SelfMatchRunningError>
		where F: FnMut() -> bool + Send + 'static,
				RH: FnMut(String) + Send + 'static,
				C: FnMut() -> String + Send + 'static,
				OE: Error + fmt::Debug,
				KW:SelfMatchKifuWriter<OE> + Send + 'static,
				Arc<Mutex<FileLogger>>: Send + 'static,
				EH: FnMut(Option<Arc<Mutex<OnErrorHandler<FileLogger>>>>,
					&SelfMatchRunningError) {
		let logger = match FileLogger::new(path) {
			Err(_) => {
				let e = SelfMatchRunningError::IOError(String::from(
					"The log output destination file could not be opened."
				));
				on_error(None,&e);
				return Err(e);
			},
			Ok(logger) => logger,
		};

		let input_reader = USIStdInputReader::new();

		self.start(on_before_newgame,
					initial_position_creator,
					kifu_writer, input_reader, input_handler,
					player1_options, player2_options,
					self_match_event_dispatcher, logger, on_error)
	}

	pub fn start<F,R,RH,C,OE,KW,L,EH>(&mut self,on_before_newgame:F,
						initial_position_creator:Option<C>,
						kifu_writer:Option<KW>,
						input_reader:R,
						input_handler:RH,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						self_match_event_dispatcher:USIEventDispatcher<
																SelfMatchEventKind,
																SelfMatchEvent,
																SelfMatchEngine<T, E, S>,L,E>,
						logger:L, mut on_error:EH) -> Result<(),SelfMatchRunningError>
		where F: FnMut() -> bool + Send + 'static,
				R: USIInputReader + Send + 'static,
				RH: FnMut(String) + Send + 'static,
				C: FnMut() -> String + Send + 'static,
				OE: Error + fmt::Debug,
				KW:SelfMatchKifuWriter<OE> + Send + 'static,
				L: Logger + fmt::Debug,
				Arc<Mutex<L>>: Send + 'static,
				EH: FnMut(Option<Arc<Mutex<OnErrorHandler<L>>>>,
					&SelfMatchRunningError) {
		let logger_arc = Arc::new(Mutex::new(logger));
		let on_error_handler_arc = Arc::new(Mutex::new(OnErrorHandler::new(logger_arc.clone())));
		let on_error_handler = on_error_handler_arc.clone();

		let r = self.run(on_before_newgame,
							initial_position_creator,
							kifu_writer, input_reader, input_handler,
							player1_options, player2_options,
							self_match_event_dispatcher,
							logger_arc, on_error_handler_arc);

		if let Err(ref e) = r {
			on_error(Some(on_error_handler),e);
		}

		r
	}

	fn run<F,R,RH,C,OE,KW,L>(&mut self,mut on_before_newgame:F,
						initial_position_creator:Option<C>,
						kifu_writer:Option<KW>,
						mut input_reader:R,
						mut input_handler:RH,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						mut self_match_event_dispatcher:USIEventDispatcher<
																SelfMatchEventKind,
																SelfMatchEvent,
																SelfMatchEngine<T, E, S>,L,E>,
						logger_arc:Arc<Mutex<L>>,
						on_error_handler_arc:Arc<Mutex<OnErrorHandler<L>>>) -> Result<(),SelfMatchRunningError>
		where F: FnMut() -> bool + Send + 'static,
				R: USIInputReader + Send + 'static,
				RH: FnMut(String) + Send + 'static,
				C: FnMut() -> String + Send + 'static,
				OE: Error + fmt::Debug,
				KW:SelfMatchKifuWriter<OE> + Send + 'static,
				L: Logger + fmt::Debug,
				Arc<Mutex<L>>: Send + 'static {
		let start_time = Instant::now();

		let mut system_event_dispatcher:USIEventDispatcher<SystemEventKind,
														SystemEvent,SelfMatchEngine<T, E, S>,L,E> = USIEventDispatcher::new(&logger_arc);

		let user_event_queue:EventQueue<UserEvent,UserEventKind> = EventQueue::new();
		let user_event_queue_arc = Arc::new(Mutex::new(user_event_queue));

		let user_event_queue = user_event_queue_arc.clone();

		let mut initial_position_creator:Box<FnMut() -> String + Send + 'static> =
			initial_position_creator.map_or(Box::new(|| String::from("startpos")), |f| {
				Box::new(f)
			});

		let on_error_handler = on_error_handler_arc.clone();

		let mut kifu_writer:Box<FnMut(&String,&Vec<Move>) +Send + 'static> =
			kifu_writer.map_or(Box::new(|_,_| ()), |mut w| Box::new(move |sfen,m| {
				w.write(sfen,m).map_err(|e| {
					on_error_handler.lock().map(|h| h.call(&e)).is_err();
				}).is_err();
			}));

		let quit_ready_arc = Arc::new(Mutex::new(false));
		let quit_ready = quit_ready_arc.clone();

		let on_error_handler = on_error_handler_arc.clone();

		system_event_dispatcher.add_handler(SystemEventKind::Quit, Box::new(move |_,e| {
			match e {
				&SystemEvent::Quit => {
					match user_event_queue.lock() {
						Ok(mut user_event_queue) => {
							user_event_queue.push(UserEvent::Quit);
							match quit_ready.lock() {
								Ok(mut quit_ready) => {
									*quit_ready = true;
								},
								Err(ref e) => {
									on_error_handler.lock().map(|h| h.call(e)).is_err();
								}
							}
						},
						Err(ref e) => {
							on_error_handler.lock().map(|h| h.call(e)).is_err();
						}
					};
					Ok(())
				},
				e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		}));

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
		let quit_ready = quit_ready_arc.clone();

		let on_error_handler = on_error_handler_arc.clone();

		let end_time = self.end_time.map(|t| t);
		let number_of_games = self.number_of_games.map(|n| n);
		let game_time_limit = self.game_time_limit;

		let bridge_h = thread::spawn(move || {
			let cs = [cs1,cs2];
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
									s:SelfMatchGameEndState| {
				win_cs.send(SelfMatchMessage::GameEnd(GameEndState::Win)).unwrap();
				lose_cs.send(SelfMatchMessage::GameEnd(GameEndState::Lose)).unwrap();
				Ok(match self_match_event_queue_inner.lock() {
					Ok(mut self_match_event_queue) => {
						self_match_event_queue.push(SelfMatchEvent::GameEnd(s));
					},
					Err(ref e) => {
						on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
						win_cs.send(SelfMatchMessage::Error(0)).unwrap();
						lose_cs.send(SelfMatchMessage::Error(1)).unwrap();

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
				})
			};

			let mut game_count = 0;

			while number_of_games.map_or(true, |n| game_count < n) &&
				  end_time.map_or(true, |t| Instant::now() - start_time < t){
				cs[0].send(SelfMatchMessage::GameStart).unwrap();
				cs[1].send(SelfMatchMessage::GameStart).unwrap();

				game_count += 1;

				let mut cs_index = if on_before_newgame() {
					1
				} else {
					0
				};

				let sfen = initial_position_creator();

				match self_match_event_queue.lock() {
					Ok(mut self_match_event_queue) => {
						self_match_event_queue.push(
							SelfMatchEvent::GameStart(if cs_index == 1 {
								1
							} else {
								2
							}, sfen.clone()));
					},
					Err(ref e) => {
						on_error_handler.lock().map(|h| h.call(e)).is_err();
						cs[0].send(SelfMatchMessage::Error(0)).unwrap();
						cs[1].send(SelfMatchMessage::Error(1)).unwrap();

						quit_notification();

						return Err(SelfMatchRunningError::InvalidState(String::from(
							"Exclusive lock on self_match_event_queue failed."
						)));
					}
				}

				let (teban, banmen, mc, n, mut mvs) = match position_parser.parse(&sfen.split(" ").collect::<Vec<&str>>()) {
					Ok(mut position) => match position {
						SystemEvent::Position(teban, p, n, m) => {
							let(banmen,mc) = match p {
								UsiInitialPosition::Startpos => {
									(shogi::BANMEN_START_POS, MochigomaCollections::Pair(HashMap::new(),HashMap::new()))
								},
								UsiInitialPosition::Sfen(Banmen(b),MochigomaCollections::Pair(ref ms,ref mg)) => {
									(b,MochigomaCollections::Pair(ms.clone(),mg.clone()))
								},
								UsiInitialPosition::Sfen(Banmen(b),MochigomaCollections::Empty) => {
									(b,MochigomaCollections::Pair(HashMap::new(),HashMap::new()))
								}
							};

							(teban,Banmen(banmen),mc,n,m)
						},
						e => {
							cs[0].send(SelfMatchMessage::Error(0)).unwrap();
							cs[1].send(SelfMatchMessage::Error(1)).unwrap();

							quit_notification();

							return Err(SelfMatchRunningError::InvalidState(format!(
								"The type of event passed and the event being processed do not match. (Event kind = {:?})",
								 e.event_kind()
							)));
						}
					},
					Err(ref e) => {
						on_error_handler.lock().map(|h| h.call(e)).is_err();

						cs[0].send(SelfMatchMessage::Error(0)).unwrap();
						cs[1].send(SelfMatchMessage::Error(1)).unwrap();

						quit_notification();

						return Err(SelfMatchRunningError::InvalidState(String::from(
							"An error occurred parsing the sfen string."
						)));
					}
				};

				let mut current_time_limit = game_time_limit.to_instant(teban,0);

				let kyokumen_hash_map:TwoKeyHashMap<u32> = TwoKeyHashMap::new();
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
					 mut kyokumen_hash_map) = banmen.apply_moves(teban,mc,&mvs,mhash,shash,kyokumen_hash_map,&hasher);

				let mut oute_kyokumen_hash_maps:[Option<TwoKeyHashMap<u32>>; 2] = [None,None];

				while end_time.map_or(true, |t| Instant::now() - start_time < t) {
					match ponders[cs_index] {
						Some(_) if ponders[cs_index] == prev_move => {
							cs[cs_index].send(SelfMatchMessage::PonderHit).unwrap();
							match sr.recv().unwrap() {
								SelfMatchMessage::NotifyMove(BestMove::Move(ref m,pm)) => {
									match self_match_event_queue.lock() {
										Ok(mut self_match_event_queue) => {
											self_match_event_queue.push(SelfMatchEvent::Moved(teban,m.clone()));
										},
										Err(ref e) => {
											on_error_handler.lock().map(|h| h.call(e)).is_err();
											cs[0].send(SelfMatchMessage::Error(0)).unwrap();
											cs[1].send(SelfMatchMessage::Error(1)).unwrap();

											quit_notification();
										}
									}

									if let (Some(limit),inc) = current_time_limit {
										if limit + Duration::from_millis(inc as u64) > Instant::now() {
											kifu_writer(&sfen,&mvs);
											on_gameend(
												cs[(cs_index+1) % 2].clone(),
												cs[cs_index].clone(),
												SelfMatchGameEndState::Timeover(teban.opposite()))?;
											break;
										}
									}

									let tinc = match current_time_limit {
										(Some(limit),tinc) => {
											(tinc + (limit - Instant::now()).subsec_nanos() * 1000000)
										},
										(_,tinc) => {
											tinc
										}
									};

									current_time_limit = game_time_limit.to_instant(teban,tinc);

									match banmen.apply_valid_move(&teban,&mc,m) {
										Ok((next,nmc,o)) => {
											if let Some(pm) = prev_move{
												if next.win_only_moves(&teban.opposite()).len() > 0 {
													if !banmen.responded_oute(&teban,&mc,&pm,&m)? {
														on_gameend(
															cs[(cs_index+1) % 2].clone(),
															cs[cs_index].clone(),
															SelfMatchGameEndState::Foul(teban.opposite(),FoulKind::NotRespondedOute)
														)?;
														mvs.push(*m);
														kifu_writer(&sfen,&mvs);
														break;
													}
												}
											}
											mc = nmc;
											teban = teban.opposite();

											mhash = hasher.calc_main_hash(mhash,&teban,&banmen,&mc,m,&o);
											shash = hasher.calc_sub_hash(shash,&teban,&banmen,&mc,m,&o);

											let is_win = match m {
												&Move::To(_,KomaDstToPosition(dx,dy,_)) => {
													match banmen {
														Banmen(ref kinds) => {
															match teban {
																Teban::Sente => {
																	kinds[dy as usize+1][9-dx as usize] == KomaKind::GOu
																},
																Teban::Gote => {
																	kinds[dy as usize+1][9-dx as usize] == KomaKind::SOu
																}
															}
														}
													}
												},
												_ => false,
											};

											mvs.push(*m);

											if is_win {
												kifu_writer(&sfen,&mvs);
												on_gameend(
													cs[cs_index].clone(),
													cs[(cs_index+1) % 2].clone(),
													SelfMatchGameEndState::Win(teban.opposite()))?;
												break;
											}

											banmen = next;

											match m {
												&Move::Put(MochigomaKind::Fu,_) if banmen.legal_moves_all(&teban, &mc).into_iter().filter(|m| {
													match m {
														&LegalMove::To(_,_,Some(ObtainKind::Ou)) => true,
														m @ _ => {
															match banmen.apply_move_none_check(&teban,&mc,&m.to_move()) {
																(ref b,_,_) => b.win_only_moves(&teban.opposite()).len() == 0
															}
														},
													}
												}).count() == 0 => {
													on_gameend(
														cs[(cs_index+1) % 2].clone(),
														cs[cs_index].clone(),
														SelfMatchGameEndState::Foul(teban.opposite(),FoulKind::PutFuAndMate)
													)?;
													kifu_writer(&sfen,&mvs);
													break;
												},
												_ => (),
											}

											match oute_kyokumen_hash_maps[cs_index] {
												None if banmen.win_only_moves(&teban).len() > 0 => {
													let mut m = TwoKeyHashMap::new();
													m.insert(mhash,shash,1);
													oute_kyokumen_hash_maps[cs_index] = Some(m);
												},
												Some(ref mut m) if banmen.win_only_moves(&teban).len() > 0 => {
													if let Some(_) = m.get(&mhash,&shash) {
														kifu_writer(&sfen,&mvs);
														on_gameend(
															cs[(cs_index+1) % 2].clone(),
															cs[cs_index].clone(),
															SelfMatchGameEndState::Foul(teban.opposite(),FoulKind::SennichiteOu)
														)?;
														break;
													}

													m.insert(mhash,shash,1);
												},
												_ => {
													oute_kyokumen_hash_maps[cs_index] = None;
												}
											};

											match kyokumen_hash_map.get(&mhash,&shash) {
												Some(c) if c == 3 => {
													kifu_writer(&sfen,&mvs);
													on_gameend(
														cs[(cs_index+1) % 2].clone(),
														cs[cs_index].clone(),
														SelfMatchGameEndState::Foul(teban,FoulKind::Sennichite)
													)?;
													break;
												},
												Some(c) => {
													kyokumen_hash_map.insert(mhash,shash,c+1);
												},
												None => {
													kyokumen_hash_map.insert(mhash,shash,1);
												}
											}
											cs_index = (cs_index + 1) % 2;
										},
										Err(_) => {
											mvs.push(*m);
											kifu_writer(&sfen,&mvs);
											on_gameend(
												cs[(cs_index+1) % 2].clone(),
												cs[cs_index].clone(),
												SelfMatchGameEndState::Foul(teban,FoulKind::InvalidMove)
											)?;
											break;
										}
									}

									prev_move = Some(*m);

									match pm {
										Some(pm) => {
											ponders[cs_index] = Some(pm);
											match mvs.clone() {
												mut mvs => {
													mvs.push(pm);
													cs[cs_index].send(
														SelfMatchMessage::StartPonderThink(
															teban,banmen.clone(),mc.clone(),n,mvs
														)).unwrap();
												}
											}
										},
										None => (),
									};
								},
								SelfMatchMessage::NotifyMove(BestMove::Resign) => {
									kifu_writer(&sfen,&mvs);
									on_gameend(
										cs[(cs_index+1) % 2].clone(),
										cs[cs_index].clone(),
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
											cs[0].send(SelfMatchMessage::Error(0)).unwrap();
											cs[1].send(SelfMatchMessage::Error(1)).unwrap();

											quit_notification();

											return Err(SelfMatchRunningError::InvalidState(String::from(
												"Exclusive lock on self_match_event_queue failed."
											)));
										}
									}
									break;
								},
								SelfMatchMessage::NotifyMove(BestMove::Win) if banmen.is_nyugyoku_win(&teban,&mc,&current_time_limit)=> {
									kifu_writer(&sfen,&mvs);
									on_gameend(
										cs[cs_index].clone(),
										cs[(cs_index+1) % 2].clone(),
										SelfMatchGameEndState::NyuGyokuWin(teban)
									)?;
									break;
								},
								SelfMatchMessage::NotifyMove(BestMove::Win) => {
									kifu_writer(&sfen,&mvs);
									on_gameend(
										cs[(cs_index+1) % 2].clone(),
										cs[cs_index].clone(),
										SelfMatchGameEndState::NyuGyokuLose(teban)
									)?;
									break;
								},
								SelfMatchMessage::Error(n) => {
									cs[(n+1)%2].send(SelfMatchMessage::Error((n+1)%2)).unwrap();
									quit_notification();
									return Err(SelfMatchRunningError::InvalidState(String::from(
										"An error occurred while executing the player thread."
									)));
								},
								SelfMatchMessage::Quit => {
									cs[0].send(SelfMatchMessage::Quit).unwrap();
									cs[1].send(SelfMatchMessage::Quit).unwrap();

									quit_notification();

									return Ok(());
								},
								_ => {
									cs[0].send(SelfMatchMessage::Error(0)).unwrap();
									cs[1].send(SelfMatchMessage::Error(1)).unwrap();

									quit_notification();

									return Err(SelfMatchRunningError::InvalidState(String::from(
										"An invalid message was sent to the self-match management thread."
									)));
								}
							}
						},
						Some(_) => {
							cs[cs_index].send(SelfMatchMessage::PonderNG).unwrap();
						},
						None => {
							cs[cs_index].send(SelfMatchMessage::StartThink(
										teban,banmen.clone(),mc.clone(),n,mvs.clone())).unwrap();

							match sr.recv().unwrap() {
								SelfMatchMessage::NotifyMove(m) => {
									prev_move = match m {
										BestMove::Move(m,pm) => {
											match self_match_event_queue.lock() {
												Ok(mut self_match_event_queue) => {
													self_match_event_queue.push(SelfMatchEvent::Moved(teban,m.clone()));
												},
												Err(ref e) => {
													on_error_handler.lock().map(|h| h.call(e)).is_err();
													cs[0].send(SelfMatchMessage::Error(0)).unwrap();
													cs[1].send(SelfMatchMessage::Error(1)).unwrap();

													quit_notification();

													return Err(SelfMatchRunningError::InvalidState(String::from(
														"Exclusive lock on self_match_event_queue failed."
													)));
												}
											}

											if let (Some(limit),inc) = current_time_limit {
												if limit + Duration::from_millis(inc as u64) > Instant::now() {
													kifu_writer(&sfen,&mvs);
													on_gameend(
														cs[(cs_index+1) % 2].clone(),
														cs[cs_index].clone(),
														SelfMatchGameEndState::Timeover(teban.opposite()))?;
													break;
												}
											}

											let tinc = match current_time_limit {
												(Some(limit),tinc) => {
													(tinc + (limit - Instant::now()).subsec_nanos() * 1000000)
												},
												(_,tinc) => {
													tinc
												}
											};

											current_time_limit = game_time_limit.to_instant(teban,tinc);

											match banmen.apply_valid_move(&teban,&mc,&m) {
												Ok((next,nmc,o)) => {
													if let Some(pm) = prev_move{
														if next.win_only_moves(&teban.opposite()).len() > 0 {
															if !banmen.responded_oute(&teban,&mc,&pm,&m)? {
																on_gameend(
																	cs[(cs_index+1) % 2].clone(),
																	cs[cs_index].clone(),
																	SelfMatchGameEndState::Foul(teban.opposite(),FoulKind::NotRespondedOute)
																)?;
																mvs.push(m);
																kifu_writer(&sfen,&mvs);
																break;
															}
														}
													}
													mc = nmc;
													teban = teban.opposite();

													mhash = hasher.calc_main_hash(mhash,&teban,&banmen,&mc,&m,&o);
													shash = hasher.calc_sub_hash(shash,&teban,&banmen,&mc,&m,&o);

													let is_win = match m {
														Move::To(_,KomaDstToPosition(dx,dy,_)) => {
															match banmen {
																Banmen(ref kinds) => {
																	match teban {
																		Teban::Sente => {
																			kinds[dy as usize+1][9-dx as usize] == KomaKind::GOu
																		},
																		Teban::Gote => {
																			kinds[dy as usize+1][9-dx as usize] == KomaKind::SOu
																		}
																	}
																}
															}
														},
														_  => false,
													};

													mvs.push(m);

													if is_win {
														kifu_writer(&sfen,&mvs);
														on_gameend(
															cs[cs_index].clone(),
															cs[(cs_index+1) % 2].clone(),
															SelfMatchGameEndState::Win(teban.opposite())
														)?;
														break;
													}

													banmen = next;

													match m {
														Move::Put(MochigomaKind::Fu,_) if banmen.legal_moves_all(&teban, &mc).into_iter().filter(|m| {
															match m {
																&LegalMove::To(_,_,Some(ObtainKind::Ou)) => true,
																m @ _ => {
																	match banmen.apply_move_none_check(&teban,&mc,&m.to_move()) {
																		(ref b,_,_) => b.win_only_moves(&teban.opposite()).len() == 0
																	}
																},
															}
														}).count() == 0 => {
															kifu_writer(&sfen,&mvs);
															on_gameend(
																cs[(cs_index+1) % 2].clone(),
																cs[cs_index].clone(),
																SelfMatchGameEndState::Foul(teban.opposite(),FoulKind::PutFuAndMate)
															)?;
															break;
														},
														_ => (),
													}

													match oute_kyokumen_hash_maps[cs_index] {
														None if banmen.win_only_moves(&teban).len() > 0 => {
															let mut m = TwoKeyHashMap::new();
															m.insert(mhash,shash,1);
															oute_kyokumen_hash_maps[cs_index] = Some(m);
														},
														Some(ref mut m) if banmen.win_only_moves(&teban).len() > 0 => {
															if let Some(_) = m.get(&mhash,&shash) {
																kifu_writer(&sfen,&mvs);
																on_gameend(
																	cs[(cs_index+1) % 2].clone(),
																	cs[cs_index].clone(),
																	SelfMatchGameEndState::Foul(teban.opposite(),FoulKind::SennichiteOu)
																)?;
																break;
															}

															m.insert(mhash,shash,1);
														},
														_ => {
															oute_kyokumen_hash_maps[cs_index] = None;
														}
													};

													match kyokumen_hash_map.get(&mhash,&shash) {
														Some(c) if c == 3 => {
															kifu_writer(&sfen,&mvs);
															on_gameend(
																cs[(cs_index+1) % 2].clone(),
																cs[cs_index].clone(),
																SelfMatchGameEndState::Foul(teban.opposite(),FoulKind::Sennichite)
															)?;
															break;
														},
														Some(c) => {
															kyokumen_hash_map.insert(mhash,shash,c+1);
														},
														None => {
															kyokumen_hash_map.insert(mhash,shash,1);
														}
													}
													ponders[cs_index] = pm;

													match pm {
														Some(pm) => {
															match mvs.clone() {
																mut mvs => {
																	mvs.push(pm);
																	cs[cs_index].send(
																		SelfMatchMessage::StartPonderThink(
																			teban,banmen.clone(),
																			mc.clone(),n,mvs)).unwrap();
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
														SelfMatchGameEndState::Foul(teban,FoulKind::InvalidMove)
													)?;
													break;
												}
											}
											Some(m)
										},
										BestMove::Resign => {
											kifu_writer(&sfen,&mvs);
											on_gameend(
												cs[(cs_index+1) % 2].clone(),
												cs[cs_index].clone(),
												SelfMatchGameEndState::Resign(teban)
											)?;
											break;
										},
										BestMove::Abort => {
											match self_match_event_queue.lock() {
												Ok(mut self_match_event_queue) => {
													self_match_event_queue.push(SelfMatchEvent::Abort);
												},
												Err(ref e) => {
													on_error_handler.lock().map(|h| h.call(e)).is_err();
													cs[0].send(SelfMatchMessage::Error(0)).unwrap();
													cs[1].send(SelfMatchMessage::Error(1)).unwrap();

													quit_notification();

													return Err(SelfMatchRunningError::InvalidState(String::from(
														"Exclusive lock on self_match_event_queue failed."
													)));
												}
											}
											break;
										},
										BestMove::Win if banmen.is_nyugyoku_win(&teban,&mc,&current_time_limit)=> {
											kifu_writer(&sfen,&mvs);
											on_gameend(
												cs[cs_index].clone(),
												cs[(cs_index+1) % 2].clone(),
												SelfMatchGameEndState::NyuGyokuWin(teban)
											)?;
											break;
										},
										BestMove::Win => {
											kifu_writer(&sfen,&mvs);
											on_gameend(
												cs[(cs_index+1) % 2].clone(),
												cs[cs_index].clone(),
												SelfMatchGameEndState::NyuGyokuLose(teban)
											)?;
											break;
										}
									}
								},
								SelfMatchMessage::Error(n) => {
									cs[(n+1)%2].send(SelfMatchMessage::Error((n+1)%2)).unwrap();
									quit_notification();
									return Err(SelfMatchRunningError::InvalidState(String::from(
										"An error occurred while executing the player thread."
									)));
								},
								SelfMatchMessage::Quit => {
									cs[0].send(SelfMatchMessage::Quit).unwrap();
									cs[1].send(SelfMatchMessage::Quit).unwrap();

									quit_notification();

									return Ok(());
								},
								_ => {
									cs[0].send(SelfMatchMessage::Error(0)).unwrap();
									cs[1].send(SelfMatchMessage::Error(1)).unwrap();

									quit_notification();
									return Err(SelfMatchRunningError::InvalidState(String::from(
										"An invalid message was sent to the self-match management thread."
									)));
								}
							}
						}
					}
				}
			}

			cs[0].send(SelfMatchMessage::Quit).unwrap();
			cs[1].send(SelfMatchMessage::Quit).unwrap();

			quit_notification();

			Ok(())
		});

		let players = [self.player1.clone(),self.player2.clone()];
		let mut handlers:Vec<JoinHandle<()>> = Vec::with_capacity(2);

		for i in 0..2 {
			let cr = cr.remove(0);
			let player = players[i].clone();
			let on_error_handler = on_error_handler_arc.clone();
			let logger = logger_arc.clone();
			let user_event_queue = user_event_queue_arc.clone();
			let info_sender = info_sender_arc.clone();
			let limit = self.game_time_limit;

			let ss = ss.clone();

			let player_i = i;

			handlers.push(thread::spawn(move || {
				loop {
					match cr.recv().unwrap() {
						SelfMatchMessage::GameStart => {
							loop {
								match player.lock() {
									Ok(mut player) => {
										match player.take_ready() {
											Ok(_) => (),
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
												ss.send(SelfMatchMessage::Error(player_i)).unwrap();
												return;
											}
										}
										match player.newgame() {
											Ok(_) => (),
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
												ss.send(SelfMatchMessage::Error(player_i)).unwrap();
												return;
											}
										}
									},
									Err(ref e) => {
										on_error_handler.lock().map(|h| h.call(e)).is_err();
										ss.send(SelfMatchMessage::Error(player_i)).unwrap();
										return;
									}
								}

								match cr.recv().unwrap() {
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
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();
														return;
													}
												}
												let mut info_sender = match info_sender.lock() {
													Ok(info_sender) => info_sender,
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();
														return;
													}
												};
												let m = match player.think(&limit,
															user_event_queue.clone(),
															&mut *info_sender,on_error_handler.clone()) {
													Ok(m) => m,
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();
														return;
													}
												};
												ss.send(SelfMatchMessage::NotifyMove(m)).unwrap();
											},
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
												ss.send(SelfMatchMessage::Error(player_i)).unwrap();
												return;
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
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();
														return;
													}
												}
												let mut info_sender = match info_sender.lock() {
													Ok(info_sender) => info_sender,
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();
														return;
													}
												};
												let m = match player.think(&limit,
															user_event_queue.clone(),
															&mut *info_sender,on_error_handler.clone()) {
													Ok(m) => m,
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();
														return;
													}
												};

												match cr.recv().unwrap() {
													SelfMatchMessage::PonderHit => {
														ss.send(SelfMatchMessage::NotifyMove(m)).unwrap();
													},
													SelfMatchMessage::PonderNG => (),
													SelfMatchMessage::Quit | SelfMatchMessage::Error(_) => {
														return;
													},
													_ => {
														logger.lock().map(|mut logger| {
															logger.logging(&format!("Invalid message."))
														}).map_err(|_| {
															USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
															false
														}).is_err();
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();
														return;
													}
												}
											},
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
												ss.send(SelfMatchMessage::Error(player_i)).unwrap();
												return;
											}
										};
									},
									SelfMatchMessage::GameEnd(s) => {
										match player.lock() {
											Ok(mut player) => {
												match player.gameover(&s,user_event_queue.clone(),
																on_error_handler.clone()) {
													Ok(()) => (),
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();
														return;
													}
												};
											},
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
												ss.send(SelfMatchMessage::Error(player_i)).unwrap();
												return;
											}
										}

									},
									SelfMatchMessage::Quit | SelfMatchMessage::Error(_) => {
										return;
									},
									_ => {
										logger.lock().map(|mut logger| {
											logger.logging(&format!("Invalid message."))
										}).map_err(|_| {
											USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
											false
										}).is_err();
										ss.send(SelfMatchMessage::Error(player_i)).unwrap();
										return;
									}
								}
							}
						},
						SelfMatchMessage::Quit | SelfMatchMessage::Error(_) => {
							return;
						},
						_ => {
							logger.lock().map(|mut logger| {
								logger.logging(&format!("Invalid message."))
							}).map_err(|_| {
								USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
								false
							}).is_err();
							ss.send(SelfMatchMessage::Error(player_i)).unwrap();
							return;
						}
					}
				}
			}));
		}

		let delay = Duration::from_millis(50);
		let on_error_handler = on_error_handler_arc.clone();
		let self_match_event_queue = self_match_event_queue_arc.clone();
		let logger = logger_arc.clone();

		thread::spawn(move || {
			loop {
				match input_reader.read() {
					Ok(line) => {
						input_handler(line);
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

		bridge_h.join().map_err(|_| {
			has_error = true;
			logger.lock().map(|mut logger| {
				logger.logging(&format!("Main thread join failed."))
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

		for h in handlers {
			h.join().map_err(|_| {
				has_error = true;
				logger.lock().map(|mut logger| {
					logger.logging(&format!("Sub thread join failed."))
				}).map_err(|_| {
					USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
					false
				}).is_err();
			}).is_err();
		}

		if has_error {
			Err(SelfMatchRunningError::Fail(String::from(
				"An error occurred while executing a self match. Please see the log for details ..."
			)))
		} else {
			Ok(())
		}
	}
}