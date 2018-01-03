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

pub trait TryFrom<T,E> where Self: Sized {
	fn try_from(s:T) -> Result<Self, TypeConvertError<E>> where E: fmt::Debug;
}
pub trait TryToString<E> where E: fmt::Debug + Error {
	fn try_to_string(&self) -> Result<String,E>;
}
pub trait Validate {
	fn validate(&self) -> bool;
}
#[derive(Debug)]
pub struct UsiAgent<T> where T: USIPlayer + fmt::Debug {
	player:Arc<Mutex<T>>,
	system_event_queue:Arc<Mutex<EventQueue<SystemEvent,SystemEventKind>>>,
}
impl<T> UsiAgent<T> where T: USIPlayer + fmt::Debug {
	pub fn new<F>(factory:F) -> UsiAgent<T>
	where T: USIPlayer + fmt::Debug, F: Fn(Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>) -> T {
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

		self.start(input_reader,output_writer,logger)
	}

	pub fn start<R,W,L>(&self,reader:R,writer:W,logger:L) ->
		Result<(),USIAgentStartupError<EventQueue<SystemEvent,SystemEventKind>>>
		where R: USIInputReader, W: USIOutputWriter, L: Logger,
		Arc<Mutex<R>>: Send + 'static, Arc<Mutex<L>>: Send + 'static, Arc<Mutex<W>>: Send + 'static {

		let reader_arc = Arc::new(Mutex::new(reader));
		let writer_arc = Arc::new(Mutex::new(writer));
		let logger_arc = Arc::new(Mutex::new(logger));

		let system_event_queue_arc = self.system_event_queue.clone();

		let mut system_event_dispatcher:USIEventDispatcher<SystemEventKind,SystemEvent,UsiAgent<T>,L> =
																			USIEventDispatcher::new(&logger_arc);

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

		let system_event_queue = system_event_queue_arc.clone();

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

		let interpreter = USIInterpreter::new();

		let logger = logger_arc.clone();
		let reader = reader_arc.clone();

		let player = self.player.clone();

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

		while !(match quit_ready.lock() {
			Ok(quit_ready) => *quit_ready,
			Err(ref e) => {
				logger.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
					USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
					e
				}).is_err()
			}
		}) {
			match system_event_dispatcher.dispatch_events(self, &*system_event_queue) {
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
