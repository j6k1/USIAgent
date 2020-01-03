use std::{thread,time};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt;
use std::error::Error;
use std::time::Instant;

use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;

use command::*;
use error::*;
use event::*;
use shogi::*;
use protocol::*;
use rule::*;
use output::*;
use Logger;
use OnErrorHandler;
use TryFrom;

pub trait USIPlayer<E>: fmt::Debug where E: PlayerError {
	const ID: &'static str;
	const AUTHOR: &'static str;
	fn get_option_kinds(&mut self) -> Result<BTreeMap<String,SysEventOptionKind>,E>;
	fn get_options(&mut self) -> Result<BTreeMap<String,UsiOptType>,E>;
	fn take_ready(&mut self) -> Result<(),E>;
	fn set_option(&mut self,name:String,value:SysEventOption) -> Result<(),E>;
	fn newgame(&mut self) -> Result<(),E>;
	fn set_position(&mut self,teban:Teban,ban:Banmen,ms:HashMap<MochigomaKind,u32>,mg:HashMap<MochigomaKind,u32>,n:u32,m:Vec<Move>)
		-> Result<(),E>;
	fn think<L,S>(&mut self,limit:&UsiGoTimeLimit,event_queue:Arc<Mutex<UserEventQueue>>,
			info_sender:S,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<BestMove,E> where L: Logger, S: InfoSender, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static;
	fn think_mate<L,S>(&mut self,limit:&UsiGoMateTimeLimit,event_queue:Arc<Mutex<UserEventQueue>>,
			info_sender:S,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<CheckMate,E> where L: Logger, S: InfoSender, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static;
	fn on_stop(&mut self,e:&UserEvent) -> Result<(), E> where E: PlayerError;
	fn gameover<L>(&mut self,s:&GameEndState,
			event_queue:Arc<Mutex<UserEventQueue>>,
			on_error_handler:Arc<Mutex<OnErrorHandler<L>>>) -> Result<(),E> where L: Logger, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static;
	fn on_quit(&mut self,e:&UserEvent) -> Result<(), E> where E: PlayerError;
	fn quit(&mut self) -> Result<(),E>;
	fn handle_events<'a,L>(&mut self,event_queue:&'a Mutex<UserEventQueue>,
						on_error_handler:&Mutex<OnErrorHandler<L>>) -> Result<bool,E>
						where L: Logger, E: Error + fmt::Debug,
								Arc<Mutex<OnErrorHandler<L>>>: Send + 'static,
								EventHandlerError<UserEventKind,E>: From<E> {
		Ok(match self.dispatch_events(event_queue,&on_error_handler) {
			Ok(_)=> true,
			Err(ref e) => {
				let _ = on_error_handler.lock().map(|h| h.call(e));
				false
			}
		})
	}

	fn dispatch_events<'a,L>(&mut self, event_queue:&'a Mutex<UserEventQueue>,
						on_error_handler:&Mutex<OnErrorHandler<L>>) ->
						Result<(), EventDispatchError<'a,UserEventQueue,UserEvent,E>>
							where L: Logger, E: Error + fmt::Debug,
									Arc<Mutex<OnErrorHandler<L>>>: Send + 'static,
									EventHandlerError<UserEventKind,E>: From<E> {
		let events = {
			event_queue.lock()?.drain_events()
		};

		let mut has_error = false;

		for e in &events {
			match e {
				&UserEvent::Stop => {
					match self.on_stop(e) {
						Ok(_) => (),
						Err(ref e) => {
							let _ = on_error_handler.lock().map(|h| h.call(e));
							has_error = true;
						}
					};
				},
				&UserEvent::Quit => {
					match self.on_quit(e) {
						Ok(_) => (),
						Err(ref e) => {
							let _ = on_error_handler.lock().map(|h| h.call(e));
							has_error = true;
						}
					};
				}
			};
		}

		match has_error {
			true => Err(EventDispatchError::ContainError),
			false => Ok(()),
		}
	}

	fn extract_kyokumen(&self,kyokumen:&Option<Kyokumen>)
		-> Result<Kyokumen,UsiProtocolError>
		where E: Error + fmt::Debug + From<UsiProtocolError> {

		match kyokumen {
			&Some(ref kyokumen) => {
				Ok(kyokumen.clone())
			},
			&None => Err(UsiProtocolError::InvalidState(
						String::from("Position information is not initialized."))),
		}
	}

	fn extract_teban(&self,kyokumen:&Option<Kyokumen>)
		-> Result<Teban,UsiProtocolError>
		where E: Error + fmt::Debug + From<UsiProtocolError> {

		match kyokumen {
			&Some(ref kyokumen) => {
				Ok(kyokumen.teban.clone())
			},
			&None => Err(UsiProtocolError::InvalidState(
						String::from("Position information is not initialized."))),
		}
	}

	fn apply_moves<T,F>(&self,mut state:State,
						mut teban:Teban,
						mut mc:MochigomaCollections,
						m:&Vec<AppliedMove>,mut r:T,mut f:F)
		-> (Teban,State,MochigomaCollections,T)
		where F: FnMut(&Self,Teban,&Banmen,
						&MochigomaCollections,&Option<AppliedMove>,
						&Option<MochigomaKind>,T) -> T {

		for m in m {
			match Rule::apply_move_none_check(&state,teban,&mc,*m) {
				(next,nmc,o) => {
					r = f(self,teban,&state.get_banmen(),&mc,&Some(*m),&o,r);
					state = next;
					mc = nmc;
					teban = teban.opposite();
				}
			}
		}
		r = f(self,teban,&state.get_banmen(),&mc,&None,&None,r);

		(teban,state,mc,r)
	}

	fn get_update_inc(&self,tinc:&u32,limit:&Option<Instant>) -> Option<u32> {
		match limit {
			&Some(limit) => {
				Some(tinc + (limit - Instant::now()).subsec_nanos() * 1000000)
			},
			&None => None,
		}
	}
}
#[derive(Clone, Debug)]
pub enum UsiInfoMessage {
	Commands(Vec<UsiInfoSubCommand>),
	Quit,
}
pub trait InfoSender: Clone + Send + 'static {
	fn send(&mut self,commands:Vec<UsiInfoSubCommand>) -> Result<(), InfoSendError>;
}
pub struct USIInfoSender {
	sender:Sender<UsiInfoMessage>
}
impl USIInfoSender {
	pub fn new(sender:Sender<UsiInfoMessage>) -> USIInfoSender {
		USIInfoSender {
			sender:sender
		}
	}

	pub(crate) fn start_worker_thread<W,L>(&self,thinking:Arc<AtomicBool>,
		receiver:Receiver<UsiInfoMessage>,
		writer:Arc<Mutex<W>>,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
		where W: USIOutputWriter, L: Logger, Arc<Mutex<W>>: Send + 'static, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static {

		thinking.store(true,Ordering::Release);

		thread::spawn(move || {
			loop {
				if !thinking.load(Ordering::Acquire) {
					break;
				}

				match receiver.recv() {
					Ok(UsiInfoMessage::Commands(commands)) => {
						match UsiOutput::try_from(&UsiCommand::UsiInfo(commands)) {
							Ok(UsiOutput::Command(ref s)) => {
								match writer.lock() {
									Err(ref e) => {
										let _ = on_error_handler.lock().map(|h| h.call(e));
										break;
									},
									Ok(ref writer) => {
										let s = writer.write(s).is_err();
										thread::sleep(time::Duration::from_millis(10));
										s
									}
								};
							},
							Err(ref e) => {
								let _ = on_error_handler.lock().map(|h| h.call(e));
								break;
							}
						}
					},
					Ok(UsiInfoMessage::Quit) => {
						break;
					},
					Err(ref e) => {
						let _ = on_error_handler.lock().map(|h| h.call(e));
						break;
					}
				}
			}
		});
	}
}
impl InfoSender for USIInfoSender {
	fn send(&mut self,commands:Vec<UsiInfoSubCommand>) -> Result<(), InfoSendError> {
		if let Err(_) = self.sender.send(UsiInfoMessage::Commands(commands)) {
			Err(InfoSendError::Fail(String::from(
				"info command send failed.")))
		} else {
			Ok(())
		}
	}
}
impl Clone for USIInfoSender {
	fn clone(&self) -> USIInfoSender {
		USIInfoSender::new(self.sender.clone())
	}
}
pub struct ConsoleInfoSender {
	writer:USIStdOutputWriter,
	silent:bool,
}
impl ConsoleInfoSender {
	pub fn new(silent:bool) -> ConsoleInfoSender {
		ConsoleInfoSender {
			writer:USIStdOutputWriter::new(),
			silent:silent
		}
	}
}
impl InfoSender for ConsoleInfoSender {
	fn send(&mut self,commands:Vec<UsiInfoSubCommand>) -> Result<(), InfoSendError> {
		if !self.silent {
			let lines = vec![commands.to_usi_command()?];

			if let Err(_) =  self.writer.write(&lines) {
				return Err(InfoSendError::Fail(String::from(
					"info command send failed.")))
			}
		}
		Ok(())
	}
}
impl Clone for ConsoleInfoSender {
	fn clone(&self) -> ConsoleInfoSender {
		ConsoleInfoSender::new(self.silent)
	}
}