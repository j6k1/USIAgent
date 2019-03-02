use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::fmt;
use std::error::Error;
use std::time::Instant;

use command::*;
use error::*;
use event::*;
use shogi::*;
use rule::*;
use UsiOutput;
use Logger;
use OnErrorHandler;
use TryFrom;

pub trait USIPlayer<E>: fmt::Debug where E: PlayerError {
	const ID: &'static str;
	const AUTHOR: &'static str;
	fn get_option_kinds(&mut self) -> Result<HashMap<String,SysEventOptionKind>,E>;
	fn get_options(&mut self) -> Result<HashMap<String,UsiOptType>,E>;
	fn take_ready(&mut self) -> Result<(),E>;
	fn set_option(&mut self,name:String,value:SysEventOption) -> Result<(),E>;
	fn newgame(&mut self) -> Result<(),E>;
	fn set_position(&mut self,teban:Teban,ban:Banmen,ms:HashMap<MochigomaKind,u32>,mg:HashMap<MochigomaKind,u32>,n:u32,m:Vec<Move>)
		-> Result<(),E>;
	fn think<L,S>(&mut self,limit:&UsiGoTimeLimit,event_queue:Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
			info_sender:Arc<Mutex<S>>,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<BestMove,E> where L: Logger, S: InfoSender;
	fn think_mate<L,S>(&mut self,limit:&UsiGoMateTimeLimit,event_queue:Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
			info_sender:Arc<Mutex<S>>,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<CheckMate,E> where L: Logger, S: InfoSender;
	fn on_stop(&mut self,e:&UserEvent) -> Result<(), E> where E: PlayerError;
	fn gameover<L>(&mut self,s:&GameEndState,
			event_queue:Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
			on_error_handler:Arc<Mutex<OnErrorHandler<L>>>) -> Result<(),E> where L: Logger;
	fn on_quit(&mut self,e:&UserEvent) -> Result<(), E> where E: PlayerError;
	fn quit(&mut self) -> Result<(),E>;
	fn handle_events<'a,L>(&mut self,event_queue:&'a Mutex<EventQueue<UserEvent,UserEventKind>>,
						on_error_handler:&Mutex<OnErrorHandler<L>>) -> Result<bool,E>
						where L: Logger, E: Error + fmt::Debug,
								EventHandlerError<UserEventKind,E>: From<E> {
		Ok(match self.dispatch_events(event_queue,&on_error_handler) {
			Ok(_)=> true,
			Err(ref e) => {
				on_error_handler.lock().map(|h| h.call(e)).is_err();
				false
			}
		})
	}

	fn dispatch_events<'a,L>(&mut self, event_queue:&'a Mutex<EventQueue<UserEvent,UserEventKind>>,
						on_error_handler:&Mutex<OnErrorHandler<L>>) ->
						Result<(), EventDispatchError<'a,EventQueue<UserEvent,UserEventKind>,UserEvent,E>>
							where L: Logger, E: Error + fmt::Debug,
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
							on_error_handler.lock().map(|h| h.call(e)).is_err();
							has_error = true;
						}
					};
				},
				&UserEvent::Quit => {
					match self.on_quit(e) {
						Ok(_) => (),
						Err(ref e) => {
							on_error_handler.lock().map(|h| h.call(e)).is_err();
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

	fn extract_kyokumen(&self,teban:&Option<Teban>,
						banmen:&Option<Banmen>,
						mc:&Option<MochigomaCollections>)
		-> Result<(Teban,Banmen,MochigomaCollections),UsiProtocolError>
		where E: Error + fmt::Debug + From<UsiProtocolError> {

		let r = match teban {
			&Some(ref teban) => match banmen {
				&Some(ref banmen) => match mc {
					&Some(ref mc) => Some((teban.clone(),banmen.clone(),mc.clone())),
					&None => None,
				},
				&None => None,
			},
			&None => None,
		};

		match r {
			Some(r) => Ok(r),
			None => Err(UsiProtocolError::InvalidState(
						String::from("Position information is not initialized."))),
		}
	}

	fn apply_moves<T,F>(&self,mut teban:Teban,
						mut state:State,
						mut mc:MochigomaCollections,
						m:Vec<AppliedMove>,mut r:T,mut f:F)
		-> (Teban,State,MochigomaCollections,T)
		where F: FnMut(&Self,Teban,&Banmen,
						&MochigomaCollections,&Option<AppliedMove>,
						&Option<MochigomaKind>,T) -> T {

		for m in &m {
			match Rule::apply_move_none_check(&state,teban,&mc,&m) {
				(next,nmc,o) => {
					r = f(self,teban,&next.get_banmen(),&mc,&Some(*m),&o,r);
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
pub trait InfoSender {
	fn send(&mut self,commands:Vec<UsiInfoSubCommand>) -> Result<(), InfoSendError>;
}
pub struct USIInfoSender {
	system_event_queue:Arc<Mutex<EventQueue<SystemEvent,SystemEventKind>>>,
}
impl USIInfoSender {
	pub fn new(system_event_queue:Arc<Mutex<EventQueue<SystemEvent,SystemEventKind>>>) -> USIInfoSender {
		USIInfoSender {
			system_event_queue:system_event_queue
		}
	}
}
impl InfoSender for USIInfoSender {
	fn send(&mut self,commands:Vec<UsiInfoSubCommand>) -> Result<(), InfoSendError> {
		match self.system_event_queue.lock() {
			Ok(mut system_event_queue) => {
				system_event_queue.push(
				SystemEvent::SendUsiCommand(UsiOutput::try_from(&UsiCommand::UsiInfo(commands))?));
				Ok(())
			},
			Err(_) => {
				Err(InfoSendError::Fail(String::from(
					"I attempted to lock the event queue for info command transmission, but it failed.")))
			}
		}
	}
}