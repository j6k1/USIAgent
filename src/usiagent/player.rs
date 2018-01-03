use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

use usiagent::command::*;
use usiagent::error::*;
use usiagent::event::*;
use usiagent::UsiOutput;
use usiagent::shogi::*;

pub trait USIPlayer {
	const ID: String;
	const AUTHOR: String;
	fn with_user_event_queue(event_queue:Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>) -> Self;
	fn get_option_kinds(&self) -> HashMap<String,SysEventOptionKind>;
	fn get_options(&self) -> HashMap<String,UsiOptType>;
	fn take_ready<F: Fn() -> Result<(), EventHandlerError<SystemEventKind>>>(&self,on_ready:F) -> bool;
	fn set_option(&self,name:String,value:SysEventOption);
	fn newgame(&self);
	fn set_position(&self,Teban,[KomaKind; 81],Vec<MochigomaKind>,Vec<MochigomaKind>,u32,Vec<Move>);
}
pub struct USIInfoSender {
	system_event_queue:Mutex<EventQueue<SystemEvent,SystemEventKind>>,
}
impl USIInfoSender {
	pub fn new(system_event_queue:Mutex<EventQueue<SystemEvent,SystemEventKind>>) -> USIInfoSender {
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