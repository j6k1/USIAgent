pub mod event;
pub mod error;
pub mod command;
pub mod logger;
pub mod string;
pub mod output;
pub mod input;
pub mod player;
pub mod shogi;
pub mod interpreter;
use std::error::Error;
use std::fmt;
use std::{thread,time};
use std::sync::Mutex;
use std::sync::Arc;
use std::marker::Send;

use usiagent::command::*;
use usiagent::event::*;
use usiagent::error::*;
use usiagent::logger::*;
use usiagent::input::*;
use usiagent::output::*;
use usiagent::interpreter::*;
use usiagent::player::*;
use usiagent::shogi::*;

pub trait TryFrom<T,E> where Self: Sized {
	fn try_from(s:T) -> Result<Self, TypeConvertError<E>> where E: fmt::Debug;
}
pub trait TryToString<E> where E: fmt::Debug + Error {
	fn try_to_string(&self) -> Result<String,E>;
}
pub trait Validate {
	fn validate(&self) -> bool;
}
pub enum OnPonderHit  {
	Some(BestMove),
	None,
}
impl OnPonderHit {
	pub fn new(m:BestMove) -> OnPonderHit {
		OnPonderHit::Some(m)
	}

	pub fn notify<L>(&self,
		system_event_queue:&Arc<Mutex<EventQueue<SystemEvent,SystemEventKind>>>,
		logger:&Arc<Mutex<L>>) where L: Logger, Arc<Mutex<L>>: Send + 'static {
		match *self {
			OnPonderHit::Some(m) => {
				match UsiOutput::try_from(&UsiCommand::UsiBestMove(m)) {
					Ok(cmd) => match system_event_queue.lock() {
						Ok(mut system_event_queue) => {
							system_event_queue.push(SystemEvent::SendUsiCommand(cmd));
						},
						Err(ref e) => {
							logger.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
								USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
								false
							}).is_err();
						}
					},
					Err(ref e) => {
						logger.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
							USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
							false
						}).is_err();
					}
				}
			},
			OnPonderHit::None => (),
		};
	}
}
#[derive(Debug)]
pub struct UsiAgent<T> where T: USIPlayer + fmt::Debug, Arc<Mutex<T>>: Send + 'static {
	player:Arc<Mutex<T>>,
	system_event_queue:Arc<Mutex<EventQueue<SystemEvent,SystemEventKind>>>,
}
impl<T> UsiAgent<T> where T: USIPlayer + fmt::Debug, Arc<Mutex<T>>: Send + 'static {
	pub fn new<F>(factory:F) -> UsiAgent<T>
	where T: USIPlayer + fmt::Debug,
			F: Fn(Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>) -> T,
			Arc<Mutex<T>>: Send + 'static, {
		UsiAgent {
			player:Arc::new(Mutex::new(factory(Arc::new(Mutex::new(EventQueue::new()))))),
			system_event_queue:Arc::new(Mutex::new(EventQueue::new())),
		}
	}

	pub fn start_default(&self) ->
		Result<(),USIAgentStartupError<EventQueue<SystemEvent,SystemEventKind>>> {
		self.start_with_log_path(String::from("logs/log.txt"))
	}

	pub fn start_with_log_path(&self,path:String) ->
		Result<(),USIAgentStartupError<EventQueue<SystemEvent,SystemEventKind>>> {

		let logger = FileLogger::new(path)
									.or(Err(USIAgentStartupError::IOError(
										String::from("The log output destination file could not be opened."))))?;

		let input_reader = USIStdInputReader::new();
		let output_writer = USIStdOutputWriter::new();

		self.start::<USIStdInputReader,USIStdOutputWriter,FileLogger>(input_reader,output_writer,logger)
	}

	pub fn start<R,W,L>(&self,reader:R,writer:W,logger:L) ->
		Result<(),USIAgentStartupError<EventQueue<SystemEvent,SystemEventKind>>>
		where R: USIInputReader, W: USIOutputWriter, L: Logger + fmt::Debug,
			Arc<Mutex<R>>: Send + 'static,
			Arc<Mutex<L>>: Send + 'static,
			Arc<Mutex<W>>: Send + 'static,
			Arc<Mutex<OnPonderHit>>: Send + 'static {

		let reader_arc = Arc::new(Mutex::new(reader));
		let writer_arc = Arc::new(Mutex::new(writer));
		let logger_arc = Arc::new(Mutex::new(logger));

		let system_event_queue_arc = self.system_event_queue.clone();

		let system_event_dispatcher:USIEventDispatcher<SystemEventKind,SystemEvent,UsiAgent<T>,L> =
																			USIEventDispatcher::new(&logger_arc);

		let system_event_dispatcher_arc = Arc::new(Mutex::new(system_event_dispatcher));

		let system_event_dispatcher = system_event_dispatcher_arc.clone();

		let user_event_queue:EventQueue<UserEvent,UserEventKind> = EventQueue::new();
		let user_event_queue_arc = Arc::new(Mutex::new(user_event_queue));

		match system_event_dispatcher.lock() {
			Err(_) => {
				return Err(USIAgentStartupError::MutexLockFailedOtherError(
					String::from("Failed to get exclusive lock of system event queue.")));
			},
			Ok(mut system_event_dispatcher) => {

				let writer = writer_arc.clone();

				let logger = logger_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::SendUsiCommand, Box::new(move |_,e| {
					match e {
						&SystemEvent::SendUsiCommand(UsiOutput::Command(ref s)) => {
							match writer.lock() {
								Err(ref e) => {
									logger.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
										USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
										false
									}).is_err()
								},
								Ok(ref writer) => {
									writer.write(s).is_err()
								}
							};
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				let logger = logger_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::Usi, Box::new(move |ctx,e| {
					match e {
						&SystemEvent::Usi => {
							let mut commands:Vec<UsiCommand> = Vec::new();

							match ctx.player.lock() {
								Ok(player) => {
									commands.push(UsiCommand::UsiId(T::ID,T::AUTHOR));
									for cmd in player.get_options().iter()
																	.map(|(k,v)| UsiCommand::UsiOption(k.clone(),v.clone()))
																	.collect::<Vec<UsiCommand>>().into_iter() {
										commands.push(cmd);
									}
								},
								Err(_) => {
									return Err(EventHandlerError::Fail(String::from(
										"Could not get exclusive lock on player object"
									)));
								}
							};

							commands.push(UsiCommand::UsiOk);

							let mut outputs:Vec<UsiOutput> = Vec::new();

							for cmd in &commands {
								outputs.push(UsiOutput::try_from(&cmd)?);
							}

							match ctx.system_event_queue.lock() {
								Ok(mut system_event_queue) => {
									for cmd in outputs {
										system_event_queue.push(SystemEvent::SendUsiCommand(cmd));
									}
								},
								Err(ref e) => {
									logger.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
										USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
										e
									}).is_err();
								}
							};
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));


				let logger = logger_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::IsReady, Box::new(move |ctx,e| {
					match e {
						&SystemEvent::IsReady => {
							let player = match ctx.player.lock() {
								Ok(player) => player,
								Err(_) => {
									return Err(EventHandlerError::Fail(String::from(
										"Could not get exclusive lock on player object"
									)));
								}
							};

							let system_event_queue = ctx.system_event_queue.clone();
							let logger_inner = logger.clone();

							player.take_ready(move || {
								let logger = &logger_inner;
								let cmd = UsiOutput::try_from(&UsiCommand::UsiReadyOk)?;

								match system_event_queue.lock() {
									Ok(mut system_event_queue) => {
										system_event_queue.push(SystemEvent::SendUsiCommand(cmd));
										Ok(())
									},
									Err(ref e) => {
										logger.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
											USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
											false
										}).is_err();
										Err(EventHandlerError::Fail(
												String::from("Failed to get exclusive lock of system event queue.")))
									}
								}
							});
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				system_event_dispatcher.add_handler(SystemEventKind::SetOption, Box::new(move |ctx,e| {
					match e {
						&SystemEvent::SetOption(ref name, ref value) => {
							match ctx.player.lock() {
								Ok(player) => {
									player.set_option(name.clone(), value.clone());
								},
								Err(_) => {
									return Err(EventHandlerError::Fail(String::from(
										"Could not get exclusive lock on player object"
									)));
								}
							};
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				system_event_dispatcher.add_handler(SystemEventKind::UsiNewGame, Box::new(move |ctx,e| {
					match e {
						&SystemEvent::UsiNewGame => {
							match ctx.player.lock() {
								Ok(player) => {
									player.newgame();
								},
								Err(_) => {
									return Err(EventHandlerError::Fail(String::from(
										"Could not get exclusive lock on player object"
									)));
								}
							};
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				system_event_dispatcher.add_handler(SystemEventKind::Position, Box::new(move |ctx,e| {
					match e {
						&SystemEvent::Position(ref t, ref p, ref n, ref v) => {
							let(b,m) = match p {
								&UsiInitialPosition::Startpos => {
									(shogi::BANMEN_START_POS, MochigomaCollections::Pair(Vec::new(),Vec::new()))
								},
								&UsiInitialPosition::Sfen(Banmen(b),MochigomaCollections::Pair(ref ms,ref mg)) => {
									(b,MochigomaCollections::Pair(ms.clone(),mg.clone()))
								},
								&UsiInitialPosition::Sfen(Banmen(b),MochigomaCollections::Empty) => {
									(b,MochigomaCollections::Pair(Vec::new(),Vec::new()))
								}
							};

							let (ms,mg) = match m {
								MochigomaCollections::Pair(ms, mg) => (ms, mg),
								_ => (Vec::new(), Vec::new())
							};

							match ctx.player.lock() {
								Ok(player) => {
									player.set_position(*t, b, ms, mg, *n, v.clone());
								},
								Err(_) => {
									return Err(EventHandlerError::Fail(String::from(
										"Could not get exclusive lock on player object"
									)));
								}
							};
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				let ready_accept = true;
				let ready_accept_arc = Arc::new(Mutex::new(ready_accept));

				let busy = false;
				let busy_arc = Arc::new(Mutex::new(busy));

				let logger = logger_arc.clone();
				let on_ponder_move_handler_arc:Arc<Mutex<OnPonderHit>> = Arc::new(Mutex::new(OnPonderHit::None));
				let allow_immediate_ponder_move_arc = Arc::new(Mutex::new(false));
				let allow_immediate_ponder_move = allow_immediate_ponder_move_arc.clone();
				let on_ponder_move_handler = on_ponder_move_handler_arc.clone();

				let user_event_queue = user_event_queue_arc.clone();
				let system_event_queue = system_event_queue_arc.clone();

				let info_sender_arc = Arc::new(Mutex::new(USIInfoSender::new(system_event_queue)));

				system_event_dispatcher.add_handler(SystemEventKind::Go, Box::new(move |ctx,e| {
					match *e {
						SystemEvent::Go(UsiGo::Go(ref opt)) => {
							let system_event_queue = ctx.system_event_queue.clone();
							let logger_inner = logger.clone();
							let player = ctx.player.clone();
							let info_sender = info_sender_arc.clone();
							let user_event_queue_inner = user_event_queue.clone();
							let opt = Arc::new(*opt);
							let opt = opt.clone();

							thread::spawn(move || {
								match player.lock() {
									Ok(player) => {
										let info_sender = match info_sender.lock() {
											Ok(info_sender) => info_sender,
											Err(ref e) => {
												logger_inner.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
													false
												}).is_err();
												return;
											}
										};
										let user_event_queue = match user_event_queue_inner.lock() {
											Ok(user_event_queue) => user_event_queue,
											Err(ref e) => {
												logger_inner.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
													false
												}).is_err();
												return;
											}
										};
										let m = player.think(&*opt,&*user_event_queue,&*info_sender);
										match UsiOutput::try_from(&UsiCommand::UsiBestMove(m)) {
											Ok(cmd) => {
												match system_event_queue.lock() {
													Ok(mut system_event_queue) => system_event_queue.push(SystemEvent::SendUsiCommand(cmd)),
													Err(ref e) => {
														logger_inner.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
															USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
															false
														}).is_err();
													}
												};
											},
											Err(ref e) => {
												logger_inner.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
													false
												}).is_err();
											}
										}
									},
									Err(ref e) => {
										logger_inner.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
											USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
											false
										}).is_err();
									}
								};
							});
							Ok(())
						},
						SystemEvent::Go(UsiGo::Ponder(ref opt)) => {
							let player = ctx.player.clone();
							let system_event_queue = ctx.system_event_queue.clone();
							let logger_inner = logger.clone();
							let allow_immediate_ponder_move_inner = allow_immediate_ponder_move.clone();
							let on_ponder_move_handler_inner = on_ponder_move_handler.clone();
							let info_sender = info_sender_arc.clone();
							let user_event_queue_inner = user_event_queue.clone();
							let opt = Arc::new(*opt);
							let opt = opt.clone();

							thread::spawn(move || {
								match player.lock() {
									Ok(player) => {
										let info_sender = match info_sender.lock() {
											Ok(info_sender) => info_sender,
											Err(ref e) => {
												logger_inner.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
													false
												}).is_err();
												return;
											}
										};
										let user_event_queue = match user_event_queue_inner.lock() {
											Ok(user_event_queue) => user_event_queue,
											Err(ref e) => {
												logger_inner.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
													false
												}).is_err();
												return;
											}
										};
										let bm = player.think(&*opt,&*user_event_queue,&*info_sender);
										match UsiOutput::try_from(&UsiCommand::UsiBestMove(bm)) {
											Ok(cmd) => {
												match allow_immediate_ponder_move_inner.lock() {
													Ok(allow_immediate_ponder_move) => {
														if *allow_immediate_ponder_move {
															match system_event_queue.lock() {
																Ok(mut system_event_queue) => {
																	system_event_queue.push(SystemEvent::SendUsiCommand(cmd));
																},
																Err(ref e) => {
																	logger_inner.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
																		USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
																		false
																	}).is_err();
																}
															}
														} else {
															match on_ponder_move_handler_inner.lock() {
																Ok(mut on_ponder_move_handler_inner) => {
																	*on_ponder_move_handler_inner = OnPonderHit::new(bm);
																},
																Err(ref e) => {
																	logger_inner.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
																		USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
																		false
																	}).is_err();
																}
															}
														}
													},
													Err(ref e) => {
														logger_inner.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
															USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
															false
														}).is_err();
														return;
													}
												}
											},
											Err(ref e) => {
												logger_inner.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
													false
												}).is_err();
											}
										}
									},
									Err(ref e) => {
										logger_inner.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
											USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
											false
										}).is_err();
									}
								};
							});
							Ok(())
						},
						/*
						&SystemEvent::Go(UsiGo::Mate(opt)) => {

						},
						*/
						ref e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				let ready_accept = ready_accept_arc.clone();
				let busy = busy_arc.clone();
				let user_event_queue = user_event_queue_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::Stop, Box::new(move |_,e| {
					match e {
						&SystemEvent::Stop => {
							let mut ready_accept = ready_accept.lock().or(Err(EventHandlerError::Fail(String::from(
								"Could not get exclusive lock on ready accept flag object."
							))))?;

							*ready_accept = true;

							if *busy.lock().or(Err(EventHandlerError::Fail(String::from(
								"Could not get exclusive lock on busy flag object."
							))))? {
								match user_event_queue.lock() {
									Ok(mut user_event_queue) => {
										user_event_queue.push(UserEvent::Stop);
									},
									Err(_) => {
										return Err(EventHandlerError::Fail(String::from(
											"Could not get exclusive lock on user event queue object."
										)));
									}
								}
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				let ready_accept = ready_accept_arc.clone();
				let allow_immediate_ponder_move = allow_immediate_ponder_move_arc.clone();
				let on_ponder_move_handler = on_ponder_move_handler_arc.clone();
				let logger = logger_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::PonderHit, Box::new(move |ctx,e| {
					match e {
						&SystemEvent::PonderHit => {
							let mut ready_accept = ready_accept.lock().or(Err(EventHandlerError::Fail(String::from(
								"Could not get exclusive lock on ready accept flag object."
							))))?;

							*ready_accept = true;

							match allow_immediate_ponder_move.lock() {
								Err(_) => {
									return Err(EventHandlerError::Fail(String::from(
										 "Could not get exclusive lock on ready allow immediate ponder move flag object."
									)));
								},
								Ok(mut allow_immediate_ponder_move) => *allow_immediate_ponder_move = true,
							};
							match on_ponder_move_handler.lock().or(Err(EventHandlerError::Fail(String::from(
								 "Could not get exclusive lock on on ponder handler object."
							))))? {
								mut g => {
									match *g {
										ref mut n @ OnPonderHit::Some(_) => {
											let system_event_queue = ctx.system_event_queue.clone();
											n.notify(&system_event_queue,&logger);
										},
										OnPonderHit::None => (),
									};
									*g = OnPonderHit::None;
								}
							};
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));
			}
		}

		let interpreter = USIInterpreter::new();

		let logger = logger_arc.clone();
		let reader = reader_arc.clone();

		let player = self.player.clone();

		let system_event_queue = system_event_queue_arc.clone();

		player.lock().map(|player| {
			interpreter.start(system_event_queue,reader,player.get_option_kinds(),&logger);
			true
		}).or_else(|e| {
			logger.lock().map(|ref mut logger| logger.logging_error(&e)).map_err(|_| {
				USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
				e
			})
		}).or(Err(USIAgentStartupError::MutexLockFailedOtherError(
					String::from("Failed to acquire exclusive lock of player object."))))?;

		let quit_ready_arc = Arc::new(Mutex::new(false));

		let system_event_queue = system_event_queue_arc.clone();

		let delay = time::Duration::from_millis(50);

		let quit_ready = quit_ready_arc.clone();

		let system_event_dispatcher = system_event_dispatcher_arc.clone();

		let logger = logger_arc.clone();

		while !(match quit_ready.lock() {
			Ok(quit_ready) => *quit_ready,
			Err(ref e) => {
				logger.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
					USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
					e
				}).is_err()
			}
		}) {
			match system_event_dispatcher.lock().or(
				Err(USIAgentStartupError::MutexLockFailedOtherError(
					String::from("Failed to get exclusive lock of system event queue.")))
			)?.dispatch_events(self, &*system_event_queue) {
				Ok(_) => true,
				Err(ref e) => {
					logger.lock().map(|ref mut logger| logger.logging_error(e)).map_err(|_| {
						USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
						e
					}).is_err()
				}
			};
			thread::sleep(delay);
		}

		Ok(())
	}
}
#[derive(Debug)]
pub enum UsiOutput {
	Command(Vec<String>),
}
impl UsiOutput {
	fn try_from(cmd: &UsiCommand) -> Result<UsiOutput, UsiOutputCreateError> {
		Ok(UsiOutput::Command(match *cmd {
			UsiCommand::UsiOk => vec![String::from("usiok")],
			UsiCommand::UsiId(ref name, ref author) => {
				vec![format!("id name {}", name), format!("id author {}", author)]
			},
			UsiCommand::UsiReadyOk => vec![String::from("readyok")],
			UsiCommand::UsiBestMove(ref m) => vec![format!("bestmove {}", m.try_to_string()?)],
			UsiCommand::UsiInfo(ref i) => vec![format!("info {}", i.try_to_string()?)],
			UsiCommand::UsiOption(ref s,ref opt) => vec![format!("option name {} type {}",s,opt.try_to_string()?)],
			UsiCommand::UsiCheckMate(ref c) => vec![format!("checkmate {}", c.try_to_string()?)],
		}))
	}
}
pub trait Logger {
	fn logging(&mut self, msg:&String) -> bool;
	fn logging_error<E: Error>(&mut self, e:&E) -> bool;
}
