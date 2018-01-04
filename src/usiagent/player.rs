use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::fmt;

use usiagent::command::*;
use usiagent::error::*;
use usiagent::event::*;
use usiagent::UsiOutput;
use usiagent::Logger;
use usiagent::OnErrorHandler;
use usiagent::shogi::*;

pub trait USIPlayer: fmt::Debug {
	const ID: String;
	const AUTHOR: String;
	fn get_option_kinds(&mut self) -> HashMap<String,SysEventOptionKind>;
	fn get_options(&mut self) -> HashMap<String,UsiOptType>;
	fn take_ready(&mut self) -> bool;
	fn set_option(&mut self,name:String,value:SysEventOption);
	fn newgame(&mut self);
	fn set_position(&mut self,Teban,[KomaKind; 81],Vec<MochigomaKind>,Vec<MochigomaKind>,u32,Vec<Move>);
	fn think<L>(&mut self,&UsiGoTimeLimit,event_queue:Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
			info_sender:&USIInfoSender,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>) -> BestMove where L: Logger;
	fn think_mate<L>(&mut self,&UsiGoMateTimeLimit,event_queue:Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
			info_sender:&USIInfoSender,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>) -> CheckMate where L: Logger;
	fn on_stop(&mut self,e:&UserEvent) -> Result<(), EventHandlerError<UserEventKind>>;
	fn gameover(&mut self,&GameEndState);
	fn quit(&mut self);
	fn handle_events<L>(&mut self,event_queue:Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
						on_error_handler:&Mutex<OnErrorHandler<L>>) -> bool where L: Logger {
		match self.dispatch_events(&*event_queue,&on_error_handler) {
			Ok(_)=> true,
			Err(ref e) => {
				on_error_handler.lock().map(|h| h.call(e)).is_err();
				return false
			}
		}
	}

	fn dispatch_events<'a,L>(&mut self, event_queue:&'a Mutex<EventQueue<UserEvent,UserEventKind>>,
						on_error_handler:&Mutex<OnErrorHandler<L>>) ->
						Result<(), EventDispatchError<'a,EventQueue<UserEvent,UserEventKind>,UserEvent>> where L: Logger {
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