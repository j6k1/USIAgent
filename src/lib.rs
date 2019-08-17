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
pub mod protocol;
pub mod rule;

use std::error::Error;
use std::fmt;
use std::{thread,time};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
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
use protocol::*;
use rule::*;

pub trait TryFrom<T,E> where Self: Sized, E: Error + fmt::Display {
	fn try_from(s:T) -> Result<Self, E> where E: fmt::Debug;
}
pub trait MaxIndex {
	fn max_index() -> usize;
}
pub trait Find<Q,R> {
	fn find(&self,query:&Q) -> Option<R>;
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
	pub fn immediate<F,R,E,L>(f:F,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>) -> Result<R,E>
	where E: Error, F: FnOnce() -> Result<R,E>, L: Logger {
		let r = f();
		match r {
			Ok(_) => (),
			Err(ref e) => {
				on_error_handler.lock().map(|h| h.call(e)).is_err();
			}
		}
		r
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
	where T: USIPlayer<E> + fmt::Debug + Send + 'static,
			E: PlayerError,
			EventHandlerError<SystemEventKind, E>: From<E> {
	player_error_type:PhantomData<E>,
	player:Arc<Mutex<T>>,
	system_event_queue:Arc<Mutex<EventQueue<SystemEvent,SystemEventKind>>>,
}
impl<T,E> UsiAgent<T,E>
	where T: USIPlayer<E> + fmt::Debug + Send + 'static,
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

	pub fn start_default<F>(&self,on_error:F) ->
		Result<(),USIAgentRunningError<EventQueue<SystemEvent,SystemEventKind>,E>>
		where F: FnMut(Option<Arc<Mutex<OnErrorHandler<FileLogger>>>>,
					&USIAgentRunningError<EventQueue<SystemEvent,SystemEventKind>,E>) {
		self.start_with_log_path(String::from("logs/log.txt"),on_error)
	}

	pub fn start_with_log_path<F>(&self,path:String,mut on_error:F) ->
		Result<(),USIAgentRunningError<EventQueue<SystemEvent,SystemEventKind>,E>>
		where F: FnMut(Option<Arc<Mutex<OnErrorHandler<FileLogger>>>>,
					&USIAgentRunningError<EventQueue<SystemEvent,SystemEventKind>,E>) {

		let logger = match FileLogger::new(path) {
			Err(_) => {
				let e = USIAgentStartupError::IOError(String::from(
					"The log output destination file could not be opened."
				));
				let e = USIAgentRunningError::from(e);
				on_error(None,&e);
				return Err(e);
			},
			Ok(logger) => logger,
		};

		let input_reader = USIStdInputReader::new();
		let output_writer = USIStdOutputWriter::new();

		self.start(input_reader,output_writer,logger,on_error)
	}

	pub fn start<R,W,L,F>(&self,reader:R,writer:W,logger:L,mut on_error:F) ->
		Result<(),USIAgentRunningError<EventQueue<SystemEvent,SystemEventKind>,E>>
		where R: USIInputReader, W: USIOutputWriter, L: Logger + fmt::Debug,
			F: FnMut(Option<Arc<Mutex<OnErrorHandler<L>>>>,
					&USIAgentRunningError<EventQueue<SystemEvent,SystemEventKind>,E>),
			EventHandlerError<SystemEventKind, E>: From<E>,
			R: Send + 'static,
			L: Send + 'static,
			W: Send + 'static,
			OnAcceptMove: Send + 'static {

		let logger_arc = Arc::new(Mutex::new(logger));
		let on_error_handler_arc = Arc::new(Mutex::new(OnErrorHandler::new(logger_arc.clone())));
		let on_error_handler = on_error_handler_arc.clone();

		let r = self.run(reader,writer,logger_arc,on_error_handler_arc);

		if let Err(ref e) = r {
			on_error(Some(on_error_handler),e);
		}

		r
	}

	fn run<R,W,L>(&self,reader:R,writer:W,logger_arc:Arc<Mutex<L>>,
								on_error_handler_arc:Arc<Mutex<OnErrorHandler<L>>>) ->
		Result<(),USIAgentRunningError<EventQueue<SystemEvent,SystemEventKind>,E>>
		where R: USIInputReader, W: USIOutputWriter, L: Logger + fmt::Debug,
			EventHandlerError<SystemEventKind, E>: From<E>,
			R: Send + 'static,
			L: Send + 'static,
			W: Send + 'static,
			OnAcceptMove: Send + 'static {
		let reader_arc = Arc::new(Mutex::new(reader));
		let writer_arc = Arc::new(Mutex::new(writer));

		let system_event_queue_arc = self.system_event_queue.clone();

		let system_event_dispatcher:USIEventDispatcher<SystemEventKind,
														SystemEvent,UsiAgent<T,E>,L,E> = USIEventDispatcher::new(&on_error_handler_arc);

		let system_event_dispatcher_arc = Arc::new(Mutex::new(system_event_dispatcher));

		let system_event_dispatcher = system_event_dispatcher_arc.clone();

		let user_event_queue:EventQueue<UserEvent,UserEventKind> = EventQueue::new();
		let user_event_queue_arc = Arc::new(Mutex::new(user_event_queue));
		let thread_queue_arc = Arc::new(Mutex::new(ThreadQueue::new()));

		let quit_ready_arc = Arc::new(Mutex::new(false));

		match system_event_dispatcher.lock() {
			Err(_) => {
				return Err(USIAgentRunningError::from(
					USIAgentStartupError::MutexLockFailedOtherError(
						String::from("Failed to get exclusive lock of system event queue."))));
			},
			Ok(mut system_event_dispatcher) => {

				let writer = writer_arc.clone();

				let on_error_handler = on_error_handler_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::SendUsiCommand, move |_,e| {
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
				});

				let on_error_handler = on_error_handler_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::Usi, move |ctx,e| {
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
				});

				let on_error_handler = on_error_handler_arc.clone();
				let thread_queue = thread_queue_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::IsReady, move |ctx,e| {
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
				});

				system_event_dispatcher.add_handler(SystemEventKind::SetOption, move |ctx,e| {
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
				});

				system_event_dispatcher.add_handler(SystemEventKind::UsiNewGame, move |ctx,e| {
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
				});

				let on_error_handler = on_error_handler_arc.clone();
				let thread_queue = thread_queue_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::Position, move |ctx,e| {
					match e {
						&SystemEvent::Position(ref t, ref p, ref n, ref v) => {
							let(b,m) = match p {
								&UsiInitialPosition::Startpos => {
									(rule::BANMEN_START_POS.clone(), MochigomaCollections::Pair(HashMap::new(),HashMap::new()))
								},
								&UsiInitialPosition::Sfen(ref b,MochigomaCollections::Pair(ref ms,ref mg)) => {
									(b.clone(),MochigomaCollections::Pair(ms.clone(),mg.clone()))
								},
								&UsiInitialPosition::Sfen(ref b,MochigomaCollections::Empty) => {
									(b.clone(),MochigomaCollections::Pair(HashMap::new(),HashMap::new()))
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
												match player.set_position(t, b, ms, mg, n, v) {
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
				});

				let busy = false;
				let busy_arc = Arc::new(Mutex::new(busy));

				let on_error_handler = on_error_handler_arc.clone();

				let on_delay_move_handler_arc:Arc<Mutex<OnAcceptMove>> = Arc::new(Mutex::new(OnAcceptMove::None));
				let allow_immediate_move_arc = Arc::new(Mutex::new(false));
				let allow_immediate_move = allow_immediate_move_arc.clone();
				let on_delay_move_handler = on_delay_move_handler_arc.clone();

				let user_event_queue = user_event_queue_arc.clone();

				let busy = busy_arc.clone();

				let thread_queue = thread_queue_arc.clone();

				let thinking_arc = Arc::new(AtomicBool::new(false));

				let writer = writer_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::Go, move |ctx,e| {
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
							let thinking = thinking_arc.clone();
							let allow_immediate_move_inner = allow_immediate_move.clone();
							let on_delay_move_handler_inner = on_delay_move_handler.clone();
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
									let (sender,receiver) = mpsc::channel();

									let info_sender = USIInfoSender::new(sender.clone());

									info_sender.start_worker_thread(
										thinking.clone(),receiver,writer.clone(),on_error_handler_inner.clone()
									);

									let thinking_inner = thinking.clone();

									thread_queue.submit(move || {
										match player.lock() {
											Ok(mut player) => {
												let bm = match player.think(&*opt,
																user_event_queue_inner.clone(),
																info_sender,on_error_handler_inner.clone()) {
																	Ok(bm) => bm,
																	Err(ref e) => {
																		on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
																		return;
																	}
																};

												thinking_inner.store(false,Ordering::Release);

												if let Err(ref e) = sender.send(UsiInfoMessage::Quit) {
													on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													return;
												}

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
							let thinking = thinking_arc.clone();
							let player = ctx.player.clone();
							let user_event_queue_inner = user_event_queue.clone();
							let opt = Arc::new(*opt);
							let opt = opt.clone();
							let busy_inner = busy.clone();

							match thread_queue.lock() {
								Ok(mut thread_queue) => {
									let (sender,receiver) = mpsc::channel();

									let info_sender = USIInfoSender::new(sender.clone());

									info_sender.start_worker_thread(
										thinking.clone(),receiver,writer.clone(),on_error_handler_inner.clone()
									);

									let thinking_inner = thinking.clone();

									thread_queue.submit(move || {
										match player.lock() {
											Ok(mut player) => {
												let m = match player.think(&*opt,
																user_event_queue_inner.clone(),
																info_sender,on_error_handler_inner.clone()) {
																	Ok(m) => m,
																	Err(ref e) => {
																		on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
																		return;
																	}
																};

												thinking_inner.store(false,Ordering::Release);

												if let Err(ref e) = sender.send(UsiInfoMessage::Quit) {
													on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													return;
												}

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
							let thinking = thinking_arc.clone();
							let player = ctx.player.clone();
							let user_event_queue_inner = user_event_queue.clone();
							let opt = Arc::new(opt);
							let opt = opt.clone();
							let busy_inner = busy.clone();

							match thread_queue.lock() {
								Ok(mut thread_queue) => {
									let (sender,receiver) = mpsc::channel();

									let info_sender = USIInfoSender::new(sender.clone());

									info_sender.start_worker_thread(
										thinking.clone(),receiver,writer.clone(),on_error_handler_inner.clone()
									);

									let thinking_inner = thinking.clone();

									thread_queue.submit(move || {
										match player.lock() {
											Ok(mut player) => {
												let m = match player.think_mate(&*opt,
																user_event_queue_inner.clone(),
																info_sender,on_error_handler_inner.clone()) {
																	Ok(m) => m,
																	Err(ref e) => {
																		on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
																		return;
																	}
																};
												thinking_inner.store(false,Ordering::Release);

												if let Err(ref e) = sender.send(UsiInfoMessage::Quit) {
													on_error_handler_inner.lock().map(|h| h.call(e)).is_err();
													return;
												}

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
				});

				let busy = busy_arc.clone();
				let user_event_queue = user_event_queue_arc.clone();
				let allow_immediate_move = allow_immediate_move_arc.clone();
				let on_delay_move_handler = on_delay_move_handler_arc.clone();
				let on_error_handler = on_error_handler_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::Stop, move |ctx,e| {
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
				});

				let allow_immediate_move = allow_immediate_move_arc.clone();
				let on_delay_move_handler = on_delay_move_handler_arc.clone();
				let on_error_handler = on_error_handler_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::PonderHit, move |ctx,e| {
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
				});

				let on_error_handler = on_error_handler_arc.clone();
				let busy = busy_arc.clone();
				let user_event_queue = user_event_queue_arc.clone();
				let thread_queue = thread_queue_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::Quit, move |ctx,e| {
					match e {
						&SystemEvent::Quit => {
							let system_event_queue = ctx.system_event_queue.clone();
							let on_error_handler_inner = on_error_handler.clone();
							let player = ctx.player.clone();
							let user_event_queue_inner = user_event_queue.clone();
							let busy_inner = busy.clone();

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
									return Err(EventHandlerError::Fail(String::from(
										"Could not get exclusive lock on busy object."
									)));
								}
							}

							match thread_queue.lock() {
								Ok(mut thread_queue) => {
									thread_queue.submit(move || {
										match player.lock() {
											Ok(mut player) => {
												match player.quit() {
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
										match system_event_queue.lock() {
											Ok(mut system_event_queue) => {
												system_event_queue.push(SystemEvent::QuitReady);
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
				});

				let on_error_handler = on_error_handler_arc.clone();
				let busy = busy_arc.clone();
				let user_event_queue = user_event_queue_arc.clone();

				let thread_queue = thread_queue_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::GameOver, move |ctx,e| {
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
				});

				let quit_ready = quit_ready_arc.clone();

				system_event_dispatcher.add_handler(SystemEventKind::QuitReady, move |_,e| {
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
				});
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