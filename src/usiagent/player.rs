use std::sync::Mutex;
use std::collections::HashMap;

use usiagent::command::*;
use usiagent::error::*;
use usiagent::event::*;
use usiagent::UsiOutput;

pub trait USIPlayer {
	fn get_option_map(&self) -> HashMap<String,SysEventOptionKind>;
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
			SystemEvent::SendUsiCommand(UsiOutput::try_from(UsiCommand::UsiInfo(commands))?));
		Ok(())
	}
}