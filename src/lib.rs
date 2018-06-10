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
pub mod selfmatch;
use std::error::Error;
use std::fmt;
use std::{thread,time};
use std::sync::Mutex;
use std::sync::Arc;
use std::marker::Send;
use std::marker::PhantomData;
use std::collections::HashMap;

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

	pub fn start_default<F,AE>(&self,on_error:F) ->
		Result<(),AE>
		where F: FnMut(Option<Arc<Mutex<OnErrorHandler<FileLogger>>>>,
					&USIAgentStartupError<EventQueue<SystemEvent,SystemEventKind>,E>)
					-> Result<(),AE>,
			AE: Error + fmt::Debug {
		self.start_with_log_path(String::from("logs/log.txt"),on_error)
	}

	pub fn start_with_log_path<F,AE>(&self,path:String,mut on_error:F) ->
		Result<(),AE>
		where F: FnMut(Option<Arc<Mutex<OnErrorHandler<FileLogger>>>>,
					&USIAgentStartupError<EventQueue<SystemEvent,SystemEventKind>,E>)
					-> Result<(),AE>,
			AE: Error + fmt::Debug {

		let logger = match FileLogger::new(path) {
			Err(_) => {
				let e = USIAgentStartupError::IOError(String::from(
					"The log output destination file could not be opened."
				));
				return on_error(None,&e)
			},
			Ok(logger) => logger,
		};

		let input_reader = USIStdInputReader::new();
		let output_writer = USIStdOutputWriter::new();

		self.start::<USIStdInputReader,USIStdOutputWriter,FileLogger,F,AE>(input_reader,output_writer,logger,on_error)
	}

	pub fn start<R,W,L,F,AE>(&self,reader:R,writer:W,logger:L,mut on_error:F) ->
		Result<(),AE>
		where R: USIInputReader, W: USIOutputWriter, L: Logger + fmt::Debug,
			F: FnMut(Option<Arc<Mutex<OnErrorHandler<L>>>>,
					&USIAgentStartupError<EventQueue<SystemEvent,SystemEventKind>,E>)
					-> Result<(),AE>,
			AE: Error + fmt::Debug,
			EventHandlerError<SystemEventKind, E>: From<E>,
			Arc<Mutex<R>>: Send + 'static,
			Arc<Mutex<L>>: Send + 'static,
			Arc<Mutex<W>>: Send + 'static,
			Arc<Mutex<OnAcceptMove>>: Send + 'static {

		let logger_arc = Arc::new(Mutex::new(logger));
		let on_error_handler_arc = Arc::new(Mutex::new(OnErrorHandler::new(logger_arc.clone())));
		let on_error_handler = on_error_handler_arc.clone();

		match self.run(reader,writer,logger_arc,on_error_handler_arc) {
			Err(ref e) => {
				on_error(Some(on_error_handler),e)
			},
			Ok(()) => Ok(())
		}
	}

	fn run<R,W,L>(&self,reader:R,writer:W,logger_arc:Arc<Mutex<L>>,
								on_error_handler_arc:Arc<Mutex<OnErrorHandler<L>>>) ->
		Result<(),USIAgentStartupError<EventQueue<SystemEvent,SystemEventKind>,E>>
		where R: USIInputReader, W: USIOutputWriter, L: Logger + fmt::Debug,
			EventHandlerError<SystemEventKind, E>: From<E>,
			Arc<Mutex<R>>: Send + 'static,
			Arc<Mutex<L>>: Send + 'static,
			Arc<Mutex<W>>: Send + 'static,
			Arc<Mutex<OnAcceptMove>>: Send + 'static {
		let reader_arc = Arc::new(Mutex::new(reader));
		let writer_arc = Arc::new(Mutex::new(writer));

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
												let mut info_sender = match info_sender.lock() {
													Ok(info_sender) => info_sender,
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
														return;
													}
												};
												let bm = match player.think(&*opt,
																user_event_queue_inner.clone(),
																&mut *info_sender,on_error_handler_inner.clone()) {
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
												let mut info_sender = match info_sender.lock() {
													Ok(info_sender) => info_sender,
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
														return;
													}
												};
												let m = match player.think(&*opt,
																user_event_queue_inner.clone(),
																&mut *info_sender,on_error_handler_inner.clone()) {
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
												let mut info_sender = match info_sender.lock() {
													Ok(info_sender) => info_sender,
													Err(ref e) => {
														on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
														return;
													}
												};
												let m = match player.think_mate(&*opt,
																user_event_queue_inner.clone(),
																&mut *info_sender,on_error_handler_inner.clone()) {
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