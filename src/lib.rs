extern crate chrono;
extern crate rand;
extern crate queuingtask;

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
pub mod hash;
use std::error::Error;
use std::fmt;
use std::{thread,time};
use std::sync::Mutex;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::marker::Send;
use std::marker::PhantomData;
use std::collections::HashMap;
use std::sync::mpsc;
use std::time::{Instant,Duration};

use queuingtask::ThreadQueue;

use command::*;
use event::*;
use error::*;
use logger::*;
use input::*;
use output::*;
use interpreter::*;
use player::*;
use shogi::*;
use hash::*;

pub trait TryFrom<T,E> where Self: Sized {
	fn try_from(s:T) -> Result<Self, TypeConvertError<E>> where E: fmt::Debug;
}
pub trait TryToString<E> where E: fmt::Debug + Error {
	fn try_to_string(&self) -> Result<String,E>;
}
pub trait Find<Q,R> {
	fn find(&self,query:&Q) -> Option<R>;
}
pub trait Validate {
	fn validate(&self) -> bool;
}
pub trait MaxIndex {
	fn max_index() -> usize;
}
pub struct OnErrorHandler<L> where L: Logger {
	logger:Arc<Mutex<L>>,
}
impl<L> OnErrorHandler<L> where L: Logger {
	pub fn new(logger:Arc<Mutex<L>>) -> OnErrorHandler<L> {
		OnErrorHandler {
			logger:logger,
		}
	}

	pub fn call<E>(&self,e:&E) -> bool where E: Error {
		self.logger.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
			USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
				false
		}).is_err()
	}
}
pub struct SandBox {

}
impl SandBox {
	pub fn immediate<F,E,L>(f:F,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
	where E: Error, F: Fn() -> Result<(),E>, L: Logger {
		match f() {
			Ok(_) => (),
			Err(ref e) => {
				on_error_handler.lock().map(|h| h.call(e)).is_err();
			}
		}
	}
}
pub enum OnAcceptMove  {
	Some(BestMove),
	None,
}
impl OnAcceptMove {
	pub fn new(m:BestMove) -> OnAcceptMove {
		OnAcceptMove::Some(m)
	}

	pub fn notify<L>(&self,
		system_event_queue:&Arc<Mutex<EventQueue<SystemEvent,SystemEventKind>>>,
		on_error_handler:&Arc<Mutex<OnErrorHandler<L>>>) where L: Logger, Arc<Mutex<L>>: Send + 'static {
		match *self {
			OnAcceptMove::Some(m) => {
				match UsiOutput::try_from(&UsiCommand::UsiBestMove(m)) {
					Ok(cmd) => match system_event_queue.lock() {
						Ok(mut system_event_queue) => {
							system_event_queue.push(SystemEvent::SendUsiCommand(cmd));
						},
						Err(ref e) => {
							on_error_handler.lock().map(|h| h.call(e)).is_err();
						}
					},
					Err(ref e) => {
						on_error_handler.lock().map(|h| h.call(e)).is_err();
					}
				}
			},
			OnAcceptMove::None => (),
		};
	}
}
#[derive(Debug)]
pub struct UsiAgent<T,E>
	where T: USIPlayer<E> + fmt::Debug, Arc<Mutex<T>>: Send + 'static,
			E: PlayerError,
			EventHandlerError<SystemEventKind, E>: From<E> {
	player_error_type:PhantomData<E>,
	player:Arc<Mutex<T>>,
	system_event_queue:Arc<Mutex<EventQueue<SystemEvent,SystemEventKind>>>,
}
impl<T,E> UsiAgent<T,E>
	where T: USIPlayer<E> + fmt::Debug, Arc<Mutex<T>>: Send + 'static,
			E: PlayerError,
			EventHandlerError<SystemEventKind, E>: From<E> {
	pub fn new(player:T) -> UsiAgent<T,E>
	where T: USIPlayer<E> + fmt::Debug,
			Arc<Mutex<T>>: Send + 'static,
			E: Error + fmt::Debug {
		UsiAgent {
			player_error_type:PhantomData::<E>,
			player:Arc::new(Mutex::new(player)),
			system_event_queue:Arc::new(Mutex::new(EventQueue::new())),
		}
	}

	pub fn start_default(&self) ->
		Result<(),USIAgentStartupError<EventQueue<SystemEvent,SystemEventKind>,E>> {
		self.start_with_log_path(String::from("logs/log.txt"))
	}

	pub fn start_with_log_path(&self,path:String) ->
		Result<(),USIAgentStartupError<EventQueue<SystemEvent,SystemEventKind>,E>> {

		let logger = FileLogger::new(path)
									.or(Err(USIAgentStartupError::IOError(
										String::from("The log output destination file could not be opened."))))?;

		let input_reader = USIStdInputReader::new();
		let output_writer = USIStdOutputWriter::new();

		self.start::<USIStdInputReader,USIStdOutputWriter,FileLogger>(input_reader,output_writer,logger)
	}

	pub fn start<R,W,L>(&self,reader:R,writer:W,logger:L) ->
		Result<(),USIAgentStartupError<EventQueue<SystemEvent,SystemEventKind>,E>>
		where R: USIInputReader, W: USIOutputWriter, L: Logger + fmt::Debug,
			EventHandlerError<SystemEventKind, E>: From<E>,
			Arc<Mutex<R>>: Send + 'static,
			Arc<Mutex<L>>: Send + 'static,
			Arc<Mutex<W>>: Send + 'static,
			Arc<Mutex<OnAcceptMove>>: Send + 'static {
		let reader_arc = Arc::new(Mutex::new(reader));
		let writer_arc = Arc::new(Mutex::new(writer));
		let logger_arc = Arc::new(Mutex::new(logger));
		let on_error_handler_arc = Arc::new(Mutex::new(OnErrorHandler::new(logger_arc.clone())));

		let system_event_queue_arc = self.system_event_queue.clone();

		let system_event_dispatcher:USIEventDispatcher<SystemEventKind,
														SystemEvent,UsiAgent<T,E>,L,E> = USIEventDispatcher::new(&logger_arc);

		let system_event_dispatcher_arc = Arc::new(Mutex::new(system_event_dispatcher));

		let system_event_dispatcher = system_event_dispatcher_arc.clone();

		let user_event_queue:EventQueue<UserEvent,UserEventKind> = EventQueue::new();
		let user_event_queue_arc = Arc::new(Mutex::new(user_event_queue));
		let thread_queue_arc = Arc::new(Mutex::new(ThreadQueue::new()));

		let quit_ready_arc = Arc::new(Mutex::new(false));

		match system_event_dispatcher.lock() {
			Err(_) => {
				return Err(USIAgentStartupError::MutexLockFailedOtherError(
					String::from("Failed to get exclusive lock of system event queue.")));
			},
			Ok(mut system_event_dispatcher) => {

				let writer = writer_arc.clone();

				let on_error_handler = on_error_handler_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::SendUsiCommand, Box::new(move |_,e| {
					match e {
						&SystemEvent::SendUsiCommand(UsiOutput::Command(ref s)) => {
							match writer.lock() {
								Err(ref e) => {
									on_error_handler.lock().map(|h| h.call(e)).is_err()
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

				let on_error_handler = on_error_handler_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::Usi, Box::new(move |ctx,e| {
					match e {
						&SystemEvent::Usi => {
							let mut commands:Vec<UsiCommand> = Vec::new();

							match ctx.player.lock() {
								Ok(mut player) => {
									commands.push(UsiCommand::UsiId(T::ID.to_string(),T::AUTHOR.to_string()));
									for cmd in player.get_options()?.iter()
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
									on_error_handler.lock().map(|h| h.call(e)).is_err();
								}
							};
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				let on_error_handler = on_error_handler_arc.clone();
				let thread_queue = thread_queue_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::IsReady, Box::new(move |ctx,e| {
					match e {
						&SystemEvent::IsReady => {
							let system_event_queue = ctx.system_event_queue.clone();
							let on_error_handler_inner = on_error_handler.clone();
							let player = ctx.player.clone();

							match thread_queue.lock() {
								Ok(mut thread_queue) => {
									thread_queue.submit(move || {
										match player.lock() {
											Ok(mut player) => {
												match player.take_ready() {
													Ok(_) => (),
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
														return;
													}
												}
												match UsiOutput::try_from(&UsiCommand::UsiReadyOk) {
													Ok(cmd) => {
														match system_event_queue.lock() {
															Ok(mut system_event_queue) => {
																system_event_queue.push(SystemEvent::SendUsiCommand(cmd));
															},
															Err(ref e) => {
																on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
															}
														};
													},
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													}
												}
											},
											Err(ref e) => {
												on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
											}
										};
									}).map(|_| {
										()
									}).map_err(|_| {
										EventHandlerError::Fail(
											String::from("An error occurred while starting the user thread."))
									})
								},
								Err(_) => {
									Err(EventHandlerError::Fail(
											String::from("Could not get exclusive lock on thread queue object.")))
								}
							}
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				system_event_dispatcher.add_handler(SystemEventKind::SetOption, Box::new(move |ctx,e| {
					match e {
						&SystemEvent::SetOption(ref name, ref value) => {
							match ctx.player.lock() {
								Ok(mut player) => {
									player.set_option(name.clone(), value.clone())?;
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
								Ok(mut player) => {
									player.newgame()?;
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

				let on_error_handler = on_error_handler_arc.clone();
				let thread_queue = thread_queue_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::Position, Box::new(move |ctx,e| {
					match e {
						&SystemEvent::Position(ref t, ref p, ref n, ref v) => {
							let(b,m) = match p {
								&UsiInitialPosition::Startpos => {
									(shogi::BANMEN_START_POS, MochigomaCollections::Pair(HashMap::new(),HashMap::new()))
								},
								&UsiInitialPosition::Sfen(Banmen(b),MochigomaCollections::Pair(ref ms,ref mg)) => {
									(b,MochigomaCollections::Pair(ms.clone(),mg.clone()))
								},
								&UsiInitialPosition::Sfen(Banmen(b),MochigomaCollections::Empty) => {
									(b,MochigomaCollections::Pair(HashMap::new(),HashMap::new()))
								}
							};

							let (ms,mg) = match m {
								MochigomaCollections::Pair(ms, mg) => (ms, mg),
								_ => (HashMap::new(),HashMap::new())
							};

							let on_error_handler_inner = on_error_handler.clone();
							let player = ctx.player.clone();
							let v = v.clone();
							let n = n.clone();
							let t = t.clone();

							match thread_queue.lock() {
								Ok(mut thread_queue) => {
									thread_queue.submit(move || {
										match player.lock() {
											Ok(mut player) => {
												match player.set_position(t, Banmen(b), ms, mg, n, v) {
													Ok(_) => (),
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													}
												}
											},
											Err(ref e) => {
												on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
											}
										};
									}).map(|_| {
										()
									}).map_err(|_| {
										EventHandlerError::Fail(
											String::from("An error occurred while starting the user thread."))
									})
								},
								Err(_) => {
									Err(EventHandlerError::Fail(
											String::from("Could not get exclusive lock on thread queue object.")))
								}
							}
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				let busy = false;
				let busy_arc = Arc::new(Mutex::new(busy));

				let on_error_handler = on_error_handler_arc.clone();

				let on_delay_move_handler_arc:Arc<Mutex<OnAcceptMove>> = Arc::new(Mutex::new(OnAcceptMove::None));
				let allow_immediate_move_arc = Arc::new(Mutex::new(false));
				let allow_immediate_move = allow_immediate_move_arc.clone();
				let on_delay_move_handler = on_delay_move_handler_arc.clone();

				let user_event_queue = user_event_queue_arc.clone();
				let system_event_queue = system_event_queue_arc.clone();

				let info_sender_arc = Arc::new(Mutex::new(USIInfoSender::new(system_event_queue)));
				let busy = busy_arc.clone();

				let thread_queue = thread_queue_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::Go, Box::new(move |ctx,e| {
					match busy.lock() {
						Ok(mut busy) => {
							*busy = true;
						},
						Err(_) => {
							return Err(EventHandlerError::Fail(String::from(
								"Could not get exclusive lock on busy flag object."
							)));
						}
					};
					match *e {
						SystemEvent::Go(UsiGo::Ponder(ref opt)) |
							SystemEvent::Go(UsiGo::Go(ref opt @ UsiGoTimeLimit::Infinite)) => {

							let player = ctx.player.clone();
							let system_event_queue = ctx.system_event_queue.clone();
							let on_error_handler_inner = on_error_handler.clone();
							let allow_immediate_move_inner = allow_immediate_move.clone();
							let on_delay_move_handler_inner = on_delay_move_handler.clone();
							let info_sender = info_sender_arc.clone();
							let user_event_queue_inner = user_event_queue.clone();
							let opt = Arc::new(*opt);
							let opt = opt.clone();
							let busy_inner = busy.clone();

							match allow_immediate_move.lock() {
								Err(_) => {
									return Err(EventHandlerError::Fail(String::from(
										 "Could not get exclusive lock on ready allow immediate ponder move flag object."
									)));
								},
								Ok(mut allow_immediate_move) => *allow_immediate_move = false,
							};

							match thread_queue.lock() {
								Ok(mut thread_queue) => {
									thread_queue.submit(move || {
										match player.lock() {
											Ok(mut player) => {
												let info_sender = match info_sender.lock() {
													Ok(info_sender) => info_sender,
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
														return;
													}
												};
												let bm = match player.think(&*opt,
																user_event_queue_inner.clone(),
																&*info_sender,on_error_handler_inner.clone()) {
																	Ok(bm) => bm,
																	Err(ref e) => {
																		on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
																		return;
																	}
																};

												match busy_inner.lock() {
													Ok(mut busy) => {
														*busy = false;
													},
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													}
												};

												match bm {
													BestMove::Abort => {
														return;
													},
													_ => (),
												}

												match UsiOutput::try_from(&UsiCommand::UsiBestMove(bm)) {
													Ok(cmd) => {
														match allow_immediate_move_inner.lock() {
															Ok(allow_immediate_move) => {
																if *allow_immediate_move {
																	match system_event_queue.lock() {
																		Ok(mut system_event_queue) => {
																			system_event_queue.push(SystemEvent::SendUsiCommand(cmd));
																		},
																		Err(ref e) => {
																			on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
																		}
																	}
																} else {
																	match on_delay_move_handler_inner.lock() {
																		Ok(mut on_delay_move_handler_inner) => {
																			*on_delay_move_handler_inner = OnAcceptMove::new(bm);
																		},
																		Err(ref e) => {
																			on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
																		}
																	}
																}
															},
															Err(ref e) => {
																on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
																return;
															}
														}
													},
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													}
												}
											},
											Err(ref e) => {
												on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
											}
										};
									}).map(|_| {
										()
									}).map_err(|_| {
										EventHandlerError::Fail(
											String::from("An error occurred while starting the user thread."))
									})
								},
								Err(_) => {
									Err(EventHandlerError::Fail(
											String::from("Could not get exclusive lock on thread queue object.")))
								}
							}
						},
						SystemEvent::Go(UsiGo::Go(ref opt)) => {
							let system_event_queue = ctx.system_event_queue.clone();
							let on_error_handler_inner = on_error_handler.clone();
							let player = ctx.player.clone();
							let info_sender = info_sender_arc.clone();
							let user_event_queue_inner = user_event_queue.clone();
							let opt = Arc::new(*opt);
							let opt = opt.clone();
							let busy_inner = busy.clone();

							match thread_queue.lock() {
								Ok(mut thread_queue) => {
									thread_queue.submit(move || {
										match player.lock() {
											Ok(mut player) => {
												let info_sender = match info_sender.lock() {
													Ok(info_sender) => info_sender,
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
														return;
													}
												};
												let m = match player.think(&*opt,
																user_event_queue_inner.clone(),
																&*info_sender,on_error_handler_inner.clone()) {
																	Ok(m) => m,
																	Err(ref e) => {
																		on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
																		return;
																	}
																};
												match busy_inner.lock() {
													Ok(mut busy) => {
														*busy = false;
													},
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													}
												};

												match m {
													BestMove::Abort => {
														return;
													},
													_ => (),
												}

												match UsiOutput::try_from(&UsiCommand::UsiBestMove(m)) {
													Ok(cmd) => {
														match system_event_queue.lock() {
															Ok(mut system_event_queue) => system_event_queue.push(SystemEvent::SendUsiCommand(cmd)),
															Err(ref e) => {
																on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
															}
														};
													},
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													}
												}
											},
											Err(ref e) => {
												on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
											}
										};
									}).map(|_| {
										()
									}).map_err(|_| {
										EventHandlerError::Fail(
											String::from("An error occurred while starting the user thread."))
									})
								},
								Err(_) => {
									Err(EventHandlerError::Fail(
											String::from("Could not get exclusive lock on thread queue object.")))
								}
							}
						},
						SystemEvent::Go(UsiGo::Mate(opt)) => {
							let system_event_queue = ctx.system_event_queue.clone();
							let on_error_handler_inner = on_error_handler.clone();
							let player = ctx.player.clone();
							let info_sender = info_sender_arc.clone();
							let user_event_queue_inner = user_event_queue.clone();
							let opt = Arc::new(opt);
							let opt = opt.clone();
							let busy_inner = busy.clone();

							match thread_queue.lock() {
								Ok(mut thread_queue) => {
									thread_queue.submit(move || {
										match player.lock() {
											Ok(mut player) => {
												let info_sender = match info_sender.lock() {
													Ok(info_sender) => info_sender,
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
														return;
													}
												};
												let m = match player.think_mate(&*opt,
																user_event_queue_inner.clone(),
																&*info_sender,on_error_handler_inner.clone()) {
																	Ok(m) => m,
																	Err(ref e) => {
																		on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
																		return;
																	}
																};
												match busy_inner.lock() {
													Ok(mut busy) => {
														*busy = false;
													},
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													}
												};

												match m {
													CheckMate::Abort => {
														return;
													},
													_ => (),
												}

												match UsiOutput::try_from(&UsiCommand::UsiCheckMate(m)) {
													Ok(cmd) => {
														match system_event_queue.lock() {
															Ok(mut system_event_queue) => system_event_queue.push(SystemEvent::SendUsiCommand(cmd)),
															Err(ref e) => {
																on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
															}
														};
													},
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													}
												}
											},
											Err(ref e) => {
												on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
											}
										};
									}).map(|_| {
										()
									}).map_err(|_| {
										EventHandlerError::Fail(
											String::from("An error occurred while starting the user thread."))
									})
								},
								Err(_) => {
									Err(EventHandlerError::Fail(
											String::from("Could not get exclusive lock on thread queue object.")))
								}
							}
						},
						ref e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				let busy = busy_arc.clone();
				let user_event_queue = user_event_queue_arc.clone();
				let allow_immediate_move = allow_immediate_move_arc.clone();
				let on_delay_move_handler = on_delay_move_handler_arc.clone();
				let on_error_handler = on_error_handler_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::Stop, Box::new(move |ctx,e| {
					match e {
						&SystemEvent::Stop => {
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
							match allow_immediate_move.lock() {
								Err(_) => {
									return Err(EventHandlerError::Fail(String::from(
										 "Could not get exclusive lock on ready allow immediate ponder move flag object."
									)));
								},
								Ok(mut allow_immediate_move) => *allow_immediate_move = true,
							};
							match on_delay_move_handler.lock().or(Err(EventHandlerError::Fail(String::from(
								 "Could not get exclusive lock on on ponder handler object."
							))))? {
								mut g => {
									match *g {
										ref mut n @ OnAcceptMove::Some(_) => {
											let system_event_queue = ctx.system_event_queue.clone();
											n.notify(&system_event_queue,&on_error_handler);
										},
										OnAcceptMove::None => (),
									};
									*g = OnAcceptMove::None;
								}
							};
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				let allow_immediate_move = allow_immediate_move_arc.clone();
				let on_delay_move_handler = on_delay_move_handler_arc.clone();
				let on_error_handler = on_error_handler_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::PonderHit, Box::new(move |ctx,e| {
					match e {
						&SystemEvent::PonderHit => {
							match allow_immediate_move.lock() {
								Err(_) => {
									return Err(EventHandlerError::Fail(String::from(
										 "Could not get exclusive lock on ready allow immediate ponder move flag object."
									)));
								},
								Ok(mut allow_immediate_move) => *allow_immediate_move = true,
							};
							match on_delay_move_handler.lock().or(Err(EventHandlerError::Fail(String::from(
								 "Could not get exclusive lock on on ponder handler object."
							))))? {
								mut g => {
									match *g {
										ref mut n @ OnAcceptMove::Some(_) => {
											let system_event_queue = ctx.system_event_queue.clone();
											n.notify(&system_event_queue,&on_error_handler);
										},
										OnAcceptMove::None => (),
									};
									*g = OnAcceptMove::None;
								}
							};
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				let on_error_handler = on_error_handler_arc.clone();
				let busy = busy_arc.clone();
				let user_event_queue = user_event_queue_arc.clone();
				let thread_queue = thread_queue_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::Quit, Box::new(move |ctx,e| {
					match e {
						&SystemEvent::Quit => {
							let system_event_queue = ctx.system_event_queue.clone();
							let on_error_handler_inner = on_error_handler.clone();
							let player = ctx.player.clone();
							let user_event_queue_inner = user_event_queue.clone();
							let busy_inner = busy.clone();

							match thread_queue.lock() {
								Ok(mut thread_queue) => {
									thread_queue.submit(move || {
										match busy_inner.lock() {
											Ok(busy) => {
												if *busy {
													match user_event_queue_inner.lock() {
														Ok(mut user_event_queue) => {
															user_event_queue.push(UserEvent::Quit);
														},
														Err(ref e) => {
															on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
														}
													}
												}
											},
											Err(ref e) => {
												on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
												return;
											}
										}
										match player.lock() {
											Ok(mut player) => {
												match player.quit() {
													Ok(_) => (),
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													}
												}
												match system_event_queue.lock() {
													Ok(mut system_event_queue) => {
														system_event_queue.push(SystemEvent::QuitReady);
													},
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													}
												};
											},
											Err(ref e) => {
												on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
											}
										};
									}).map(|_| {
										()
									}).map_err(|_| {
										EventHandlerError::Fail(
											String::from("An error occurred while starting the user thread."))
									})
								},
								Err(_) => {
									Err(EventHandlerError::Fail(
											String::from("Could not get exclusive lock on thread queue object.")))
								}
							}
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				let on_error_handler = on_error_handler_arc.clone();
				let busy = busy_arc.clone();
				let user_event_queue = user_event_queue_arc.clone();

				let thread_queue = thread_queue_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::GameOver, Box::new(move |ctx,e| {
					match *e {
						SystemEvent::GameOver(ref s) => {
							let player = ctx.player.clone();
							let on_error_handler_inner = on_error_handler.clone();
							let s = s.clone();
							let user_event_queue_inner = user_event_queue.clone();

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

							match thread_queue.lock() {
								Ok(mut thread_queue) => {
									thread_queue.submit(move || {
										match player.lock() {
											Ok(mut player) => {
												match player.gameover(&s,user_event_queue_inner.clone(),
																				on_error_handler_inner.clone()) {
													Ok(_) => (),
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													}
												}
											},
											Err(ref e) => {
												on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
											}
										};
									}).map(|_| {
										()
									}).map_err(|_| {
										EventHandlerError::Fail(
											String::from("An error occurred while starting the user thread."))
									})
								},
								Err(_) => {
									Err(EventHandlerError::Fail(
											String::from("Could not get exclusive lock on thread queue object.")))
								}
							}
						},
						ref e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));

				let quit_ready = quit_ready_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::QuitReady, Box::new(move |_,e| {
					match e {
						&SystemEvent::QuitReady => {
							match quit_ready.lock() {
								Ok(mut quit_ready) => *quit_ready = true,
								Err(_) => {
									return Err(EventHandlerError::Fail(String::from(
										"Could not get exclusive lock on quit ready flag object."
									)));
								}
							}
							Ok(())
						},
						e => Err(EventHandlerError::InvalidState(e.event_kind())),
					}
				}));
			}
		}

		let interpreter = USIInterpreter::new();

		let logger = logger_arc.clone();
		let on_error_handler = on_error_handler_arc.clone();
		let reader = reader_arc.clone();

		let player = self.player.clone();

		let system_event_queue = system_event_queue_arc.clone();

		player.lock().map(|mut player| {
			let option_kinds = match player.get_option_kinds() {
				Ok(option_kinds) => option_kinds,
				Err(ref e) => {
					on_error_handler.lock().map(|h| h.call(e)).is_err();
					return false;
				}
			};
			interpreter.start(system_event_queue,reader,option_kinds,&logger);
			true
		}).or_else(|e| {
			on_error_handler.lock().map(|h| h.call(&e))
		}).or(Err(USIAgentStartupError::MutexLockFailedOtherError(
					String::from("Failed to acquire exclusive lock of player object."))))?;
		let system_event_queue = system_event_queue_arc.clone();

		let delay = time::Duration::from_millis(50);

		let quit_ready = quit_ready_arc.clone();

		let system_event_dispatcher = system_event_dispatcher_arc.clone();

		let on_error_handler = on_error_handler_arc.clone();

		while !(match quit_ready.lock() {
			Ok(quit_ready) => *quit_ready,
			Err(ref e) => {
				on_error_handler.lock().map(|h| h.call(e)).is_err();
				true
			}
		}) {
			match system_event_dispatcher.lock().or(
				Err(USIAgentStartupError::MutexLockFailedOtherError(
					String::from("Failed to get exclusive lock of system event queue.")))
			)?.dispatch_events(self, &*system_event_queue) {
				Ok(_) => true,
				Err(ref e) => {
					on_error_handler.lock().map(|h| h.call(e)).is_err()
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
pub trait SelfMatchKifuWriter<OE> where OE: Error + fmt::Debug {
	fn write(&mut self,initial_sfen:&String,m:&Vec<Move>) -> Result<(),OE>;
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
	end_time:Option<Instant>,
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
				end_time:Option<Instant>,number_of_games:Option<u32>,
				silent:bool)
	-> SelfMatchEngine<T,E,S>
	where T: USIPlayer<E> + fmt::Debug,
			Arc<Mutex<T>>: Send + 'static,
			E: Error + fmt::Debug,
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

	pub fn start<F,R,RH,C,OE,KW,L>(&mut self,mut on_before_newgame:F,
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
						logger:L)
		where F: FnMut() -> bool + Send + 'static,
				R: USIInputReader + Send + 'static,
				RH: FnMut(String) + Send + 'static,
				C: FnMut() -> String + Send + 'static,
				OE: Error + fmt::Debug,
				KW:SelfMatchKifuWriter<OE> + Send + 'static,
				L: Logger + fmt::Debug,
				Arc<Mutex<L>>: Send + 'static {
		let logger_arc = Arc::new(Mutex::new(logger));
		let on_error_handler_arc = Arc::new(Mutex::new(OnErrorHandler::new(logger_arc.clone())));

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
							return;
						}
					}
				}
			},
			Err(ref e) => {
				on_error_handler.lock().map(|h| h.call(e)).is_err();
				return;
			}
		}

		match player2.lock() {
			Ok(mut player) => {
				for (k,v) in player2_options {
					match player.set_option(k,v) {
						Ok(()) => (),
						Err(ref e) => {
							on_error_handler.lock().map(|h| h.call(e)).is_err();
							return;
						}
					}
				}
			},
			Err(ref e) => {
				on_error_handler.lock().map(|h| h.call(e)).is_err();
				return;
			}
		}

		let position_parser = PositionParser::new();

		let self_match_event_queue = self_match_event_queue_arc.clone();
		let quit_ready = quit_ready_arc.clone();

		let on_error_handler = on_error_handler_arc.clone();
		let logger = logger_arc.clone();

		let bridge_h = std::thread::spawn(move || {
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

			loop {
				cs[0].send(SelfMatchMessage::GameStart).unwrap();
				cs[1].send(SelfMatchMessage::GameStart).unwrap();

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
						cs[0].send(SelfMatchMessage::Error(0)).unwrap();;
						cs[1].send(SelfMatchMessage::Error(1)).unwrap();;

						quit_notification();

						return;
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
							let e = EventHandlerError::InvalidState(e.event_kind());
							on_error_handler.lock().map(|h| h.call(&e)).is_err();

							cs[0].send(SelfMatchMessage::Error(0)).unwrap();;
							cs[1].send(SelfMatchMessage::Error(1)).unwrap();;

							quit_notification();

							return;
						}
					},
					Err(ref e) => {
						on_error_handler.lock().map(|h| h.call(e)).is_err();

						cs[0].send(SelfMatchMessage::Error(0)).unwrap();;
						cs[1].send(SelfMatchMessage::Error(1)).unwrap();;

						quit_notification();

						return;
					}
				};

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

				loop {
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
											cs[0].send(SelfMatchMessage::Error(0)).unwrap();;
											cs[1].send(SelfMatchMessage::Error(1)).unwrap();;

											quit_notification();
										}
									}
									match banmen.apply_valid_move(&teban,&mc,&m) {
										Ok((next,nmc,o)) => {
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
												cs[cs_index].send(SelfMatchMessage::GameEnd(GameEndState::Win)).unwrap();
												cs[(cs_index+1) % 2].send(SelfMatchMessage::GameEnd(GameEndState::Lose)).unwrap();
												kifu_writer(&sfen,&mvs);
												break;
											}

											banmen = next;

											match kyokumen_hash_map.get(&mhash,&shash) {
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
											cs[cs_index].send(SelfMatchMessage::GameEnd(GameEndState::Lose)).unwrap();
											cs[(cs_index+1) % 2].send(SelfMatchMessage::GameEnd(GameEndState::Win)).unwrap();
											kifu_writer(&sfen,&mvs);
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
								SelfMatchMessage::Error(n) => {
									cs[(n+1)%2].send(SelfMatchMessage::Error((n+1)%2)).unwrap();;
									quit_notification();
									return;
								},
								SelfMatchMessage::Quit => {
									cs[0].send(SelfMatchMessage::Quit).unwrap();;
									cs[1].send(SelfMatchMessage::Quit).unwrap();;

									quit_notification();
									return;
								},
								_ => {
									logger.lock().map(|mut logger| {
										logger.logging(&format!("Invalid message."))
									}).map_err(|_| {
										USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
										false
									}).is_err();

									cs[0].send(SelfMatchMessage::Error(0)).unwrap();;
									cs[1].send(SelfMatchMessage::Error(1)).unwrap();;

									quit_notification();
									return;
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
													cs[0].send(SelfMatchMessage::Error(0)).unwrap();;
													cs[1].send(SelfMatchMessage::Error(1)).unwrap();;

													quit_notification();
													return;
												}
											}

											match banmen.apply_valid_move(&teban,&mc,&m) {
												Ok((next,nmc,o)) => {
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
														cs[cs_index].send(
																SelfMatchMessage::GameEnd(GameEndState::Win)).unwrap();
														cs[(cs_index+1) % 2].send(
																SelfMatchMessage::GameEnd(GameEndState::Lose)).unwrap();
														kifu_writer(&sfen,&mvs);
														match self_match_event_queue.lock() {
															Ok(mut self_match_event_queue) => {
																self_match_event_queue.push(SelfMatchEvent::GameEnd(
																		SelfMatchGameEndState::Win(teban.opposite())
																	));
															},
															Err(ref e) => {
																on_error_handler.lock().map(|h| h.call(e)).is_err();
																cs[0].send(SelfMatchMessage::Error(0)).unwrap();;
																cs[1].send(SelfMatchMessage::Error(1)).unwrap();;

																quit_notification();
																return;
															}
														}
														break;
													}

													banmen = next;

													match kyokumen_hash_map.get(&mhash,&shash) {
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
													cs[cs_index].send(
															SelfMatchMessage::GameEnd(GameEndState::Lose)).unwrap();
													cs[(cs_index+1) % 2].send(
															SelfMatchMessage::GameEnd(GameEndState::Win)).unwrap();
													kifu_writer(&sfen,&mvs);
												}
											}
											Some(m)
										},
										BestMove::Resign => {
											cs[cs_index].send(SelfMatchMessage::GameEnd(GameEndState::Lose)).unwrap();
											cs[(cs_index+1) % 2].send(SelfMatchMessage::GameEnd(GameEndState::Win)).unwrap();
											kifu_writer(&sfen,&mvs);
											match self_match_event_queue.lock() {
												Ok(mut self_match_event_queue) => {
													self_match_event_queue.push(SelfMatchEvent::GameEnd(
															SelfMatchGameEndState::Resign(teban)
														));
												},
												Err(ref e) => {
													on_error_handler.lock().map(|h| h.call(e)).is_err();
													cs[0].send(SelfMatchMessage::Error(0)).unwrap();;
													cs[1].send(SelfMatchMessage::Error(1)).unwrap();;

													quit_notification();
													return;
												}
											}
											break;
										},
										BestMove::Abort => {
											match self_match_event_queue.lock() {
												Ok(mut self_match_event_queue) => {
													self_match_event_queue.push(SelfMatchEvent::Abort);
												},
												Err(ref e) => {
													on_error_handler.lock().map(|h| h.call(e)).is_err();
													cs[0].send(SelfMatchMessage::Error(0)).unwrap();;
													cs[1].send(SelfMatchMessage::Error(1)).unwrap();;

													quit_notification();
													return;
												}
											}
											break;
										},
										BestMove::Win if banmen.is_nyugyoku_win(&teban)=> {
											cs[cs_index].send(
													SelfMatchMessage::GameEnd(GameEndState::Win)).unwrap();
											cs[(cs_index+1) % 2].send(
													SelfMatchMessage::GameEnd(GameEndState::Lose)).unwrap();
											kifu_writer(&sfen,&mvs);
											match self_match_event_queue.lock() {
												Ok(mut self_match_event_queue) => {
													self_match_event_queue.push(SelfMatchEvent::GameEnd(
															SelfMatchGameEndState::NyuGyokuWin(teban)
														));
												},
												Err(ref e) => {
													on_error_handler.lock().map(|h| h.call(e)).is_err();
													cs[0].send(SelfMatchMessage::Error(0)).unwrap();;
													cs[1].send(SelfMatchMessage::Error(1)).unwrap();;

													quit_notification();
													return;
												}
											}
											break;
										},
										BestMove::Win => {
											cs[cs_index].send(
													SelfMatchMessage::GameEnd(GameEndState::Lose)).unwrap();
											cs[(cs_index+1) % 2].send(
														SelfMatchMessage::GameEnd(GameEndState::Win)).unwrap();
											kifu_writer(&sfen,&mvs);
											match self_match_event_queue.lock() {
												Ok(mut self_match_event_queue) => {
													self_match_event_queue.push(SelfMatchEvent::GameEnd(
															SelfMatchGameEndState::NyuGyokuLose(teban)
														));
												},
												Err(ref e) => {
													on_error_handler.lock().map(|h| h.call(e)).is_err();
													cs[0].send(SelfMatchMessage::Error(0)).unwrap();;
													cs[1].send(SelfMatchMessage::Error(1)).unwrap();;

													quit_notification();
													return;
												}
											}
											break;
										}
									}
								},
								SelfMatchMessage::Error(n) => {
									cs[(n+1)%2].send(SelfMatchMessage::Error((n+1)%2)).unwrap();;
									quit_notification();
									return;
								},
								SelfMatchMessage::Quit => {
									cs[0].send(SelfMatchMessage::Quit).unwrap();;
									cs[1].send(SelfMatchMessage::Quit).unwrap();;

									quit_notification();
									return;
								},
								_ => {
									logger.lock().map(|mut logger| {
										logger.logging(&format!("Invalid message."))
									}).map_err(|_| {
										USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
										false
									}).is_err();

									cs[0].send(SelfMatchMessage::Error(0)).unwrap();;
									cs[1].send(SelfMatchMessage::Error(1)).unwrap();;

									quit_notification();
									return;
								}
							}
						}
					}
				}
			}
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

			handlers.push(std::thread::spawn(move || {
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
												ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
												return;
											}
										}
										match player.newgame() {
											Ok(_) => (),
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
												ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
												return;
											}
										}
									},
									Err(ref e) => {
										on_error_handler.lock().map(|h| h.call(e)).is_err();
										ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
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
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
														return;
													}
												}
												let info_sender = match info_sender.lock() {
													Ok(info_sender) => info_sender,
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
														return;
													}
												};
												let m = match player.think(&limit,
															user_event_queue.clone(),
															&*info_sender,on_error_handler.clone()) {
													Ok(m) => m,
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
														return;
													}
												};
												ss.send(SelfMatchMessage::NotifyMove(m)).unwrap();
											},
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
												ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
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
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
														return;
													}
												}
												let info_sender = match info_sender.lock() {
													Ok(info_sender) => info_sender,
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
														return;
													}
												};
												let m = match player.think(&limit,
															user_event_queue.clone(),
															&*info_sender,on_error_handler.clone()) {
													Ok(m) => m,
													Err(ref e) => {
														on_error_handler.lock().map(|h| h.call(e)).is_err();
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
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
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
														return;
													}
												}
											},
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
												ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
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
														ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
														return;
													}
												};
											},
											Err(ref e) => {
												on_error_handler.lock().map(|h| h.call(e)).is_err();
												ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
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
										ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
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
							ss.send(SelfMatchMessage::Error(player_i)).unwrap();;
							return;
						}
					}
				}
			}));
		}

		let delay = Duration::from_millis(50);
		let on_error_handler = on_error_handler_arc.clone();
		let self_match_event_queue = self_match_event_queue_arc.clone();
		let quit_ready = quit_ready_arc.clone();
		let logger = logger_arc.clone();

		let input_reader_h = std::thread::spawn(move || {
			while !(match quit_ready.lock() {
				Ok(quit_ready) => *quit_ready,
				Err(ref e) => {
					on_error_handler.lock().map(|h| h.call(e)).is_err();
					true
				}
			}) {
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

		bridge_h.join().map_err(|_| {
			logger.lock().map(|mut logger| {
				logger.logging(&format!("Main thread join failed."))
			}).map_err(|_| {
				USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
				false
			}).is_err();
		}).is_err();

		for h in handlers {
			h.join().map_err(|_| {
				logger.lock().map(|mut logger| {
					logger.logging(&format!("Sub thread join failed."))
				}).map_err(|_| {
					USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
					false
				}).is_err();
			}).is_err();
		}

		input_reader_h.join().map_err(|_| {
			logger.lock().map(|mut logger| {
				logger.logging(&format!("Input reader thread join failed."))
			}).map_err(|_| {
				USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
				false
			}).is_err();
		}).is_err();
	}
}