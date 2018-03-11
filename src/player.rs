use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::fmt;
use std::error::Error;

use command::*;
use error::*;
use event::*;
use UsiOutput;
use Logger;
use OnErrorHandler;
use shogi::*;

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
	fn think<L>(&mut self,limit:&UsiGoTimeLimit,event_queue:Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
			info_sender:&USIInfoSender,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<BestMove,E> where L: Logger;
	fn think_mate<L>(&mut self,limit:&UsiGoMateTimeLimit,event_queue:Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
			info_sender:&USIInfoSender,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<CheckMate,E> where L: Logger;
	fn on_stop(&mut self,e:&UserEvent) -> Result<(), E> where E: PlayerError;
	fn gameover<L>(&mut self,s:&GameEndState,
			event_queue:Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
			on_error_handler:&Mutex<OnErrorHandler<L>>) -> Result<(),E> where L: Logger;
	fn on_quit(&mut self,e:&UserEvent) -> Result<(), E> where E: PlayerError;
	fn quit(&mut self) -> Result<(),E>;
	fn handle_events<'a,L>(&mut self,event_queue:&'a Mutex<EventQueue<UserEvent,UserEventKind>>,
						on_error_handler:&Mutex<OnErrorHandler<L>>) -> Result<bool,E>
						where L: Logger, E: Error + fmt::Debug, E: From<E>,
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
									E: From<E>,
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
	pub fn send(&self,commands:Vec<UsiInfoSubCommand>) ->
		Result<(), UsiEventSendError<EventQueue<SystemEvent,SystemEventKind>>> {
		self.system_event_queue.lock()?.push(
			SystemEvent::SendUsiCommand(UsiOutput::try_from(&UsiCommand::UsiInfo(commands))?));
		Ok(())
	}
}