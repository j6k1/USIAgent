use std::iter::IntoIterator;

use std::fmt;
use std::error;
use std::io;
use std::convert::From;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::collections::HashMap;
use std::ops::Add;
use usiagent::event::SystemEventKind;
use usiagent::event::UserEventKind;
use usiagent::error::USIAgentRunningError;
use usiagent::error::USIAgentStartupError;
use usiagent::error::PlayerError;
use usiagent::error::UsiProtocolError;
use usiagent::error::TypeConvertError;
use usiagent::error::InfoSendError;
use usiagent::shogi::*;
use usiagent::event::*;
use usiagent::command::*;
use usiagent::logger::Logger;
use usiagent::player::USIPlayer;
use usiagent::player::InfoSender;
use usiagent::OnErrorHandler;
use usiagent::input::USIInputReader;
use usiagent::output::USIOutputWriter;

#[derive(Debug)]
pub enum CommonError {
	Fail(String),
}
impl PlayerError for CommonError {

}
impl fmt::Display for CommonError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			CommonError::Fail(ref s) => write!(f, "{}",s),
		}
	}
}
impl error::Error for CommonError {
	fn description(&self) -> &str {
		match *self {
			CommonError::Fail(_) => "Player error.",
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		match *self {
			CommonError::Fail(_) => None,
		}
	}
}
impl<'a> From<CommonError> for USIAgentRunningError<'a,SystemEventKind,CommonError>
	where SystemEventKind: fmt::Debug {
	fn from(err: CommonError) -> USIAgentRunningError<'a,SystemEventKind,CommonError> {
		USIAgentRunningError::from(USIAgentStartupError::PlayerError(err))
	}
}
impl<'a> From<CommonError> for USIAgentRunningError<'a,UserEventKind,CommonError>
	where UserEventKind: fmt::Debug {
	fn from(err: CommonError) -> USIAgentRunningError<'a,UserEventKind,CommonError> {
		USIAgentRunningError::from(USIAgentStartupError::PlayerError(err))
	}
}
impl From<TypeConvertError<String>> for CommonError {
	fn from(err: TypeConvertError<String>) -> CommonError {
		CommonError::Fail(format!("An error occurred during type conversion. ({})",err))
	}
}
impl From<io::Error> for CommonError {
	fn from(_: io::Error) -> CommonError {
		CommonError::Fail(String::from("I/O Error."))
	}
}
impl From<UsiProtocolError> for CommonError {
	fn from(err: UsiProtocolError) -> CommonError {
		match err {
			UsiProtocolError::InvalidState(s) => CommonError::Fail(s)
		}
	}
}
pub struct MockInputReader {
	rcv:Receiver<String>,
}
impl MockInputReader {
	pub fn new(rcv:Receiver<String>) -> MockInputReader {
		MockInputReader {
			rcv:rcv
		}
	}
}
impl USIInputReader for MockInputReader {
	fn read(&mut self) -> io::Result<String> {
		let l = self.rcv.recv().expect_err("Failed to receive input.");

		Ok(l.to_string())
	}
}
pub struct MockOutputWriter {
	sender:Sender<String>,
}
impl MockOutputWriter {
	pub fn new(sender:Sender<String>) -> MockOutputWriter {
		MockOutputWriter {
			sender:sender
		}
	}
}
impl USIOutputWriter for MockOutputWriter {
	fn write(&self,lines:&Vec<String>) -> io::Result<usize> {
		let s = lines.join("\n").add("\n").as_bytes().len();

		for l in lines {
			let _ = self.sender.send(l.to_string());
		}

		Ok(s)
	}
}
pub struct Actions<'a,T,R> {
	actions:Vec<Box<(dyn FnMut(T) -> R + 'a)>>
}
impl<'a,T,R> Actions<'a,T,R> {
	pub fn add<A>(&mut self,act:A) where A: FnMut(T) -> R + 'a {
		self.actions.push(Box::new(act));
	}
}
impl<'a,T,R> IntoIterator for Actions<'a,T,R> {
	type Item = Box<dyn FnMut(T) -> R + 'a>;
	type IntoIter = ::std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.actions.into_iter()
	}
}
#[allow(dead_code)]
pub enum ActionKind {
	TakeReady,
	SetOption,
	NewGame,
	SetPosition,
	Think,
	ThinkMate,
	OnStop,
	GameOver,
	OnQuit,
	Quit,
}
pub struct MockPlayer<IR,IP,IG,IM,IE,IOP>
	where IR: Iterator<Item=Box<dyn FnMut(&mut MockPlayer<IR,IP,IG,IM,IE,IOP>) -> Result<(),CommonError>>>,
			IP: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,Teban,Banmen,
												HashMap<MochigomaKind,u32>,
												HashMap<MochigomaKind,u32>,u32,Vec<Move>)) -> Result<(),CommonError>>>,
			IG: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,&UsiGoTimeLimit,
												Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
												Box<dyn FnMut(Vec<UsiInfoSubCommand>) -> Result<(),InfoSendError>>)) -> Result<BestMove,CommonError>>>,
			IM: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,&UsiGoMateTimeLimit,
												Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
												Box<dyn FnMut(Vec<UsiInfoSubCommand>) -> Result<(),InfoSendError>>)) -> Result<CheckMate,CommonError>>>,
			IE: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,&GameEndState,
												Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,)) -> Result<(),CommonError>>>,
			IOP: Iterator<Item=(String,SysEventOption)> {
	pub on_isready: Option<IR>,
	pub on_position: Option<IP>,
	pub on_think:Option<IG>,
	pub on_think_mate:Option<IM>,
	pub on_gameover: Option<IE>,
	pub options_it:Option<IOP>,
	sender:Sender<Result<ActionKind,String>>,
	info_send_notifier:Sender<()>,
}
impl<IR,IP,IG,IM,IE,IOP> MockPlayer<IR,IP,IG,IM,IE,IOP> where IR: Iterator<Item=Box<dyn FnMut(&mut MockPlayer<IR,IP,IG,IM,IE,IOP>) -> Result<(),CommonError>>>,
			IP: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,Teban,Banmen,
												HashMap<MochigomaKind,u32>,
												HashMap<MochigomaKind,u32>,u32,Vec<Move>)) -> Result<(),CommonError>>>,
			IG: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,&UsiGoTimeLimit,
												Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
												Box<dyn FnMut(Vec<UsiInfoSubCommand>) -> Result<(),InfoSendError>>)) -> Result<BestMove,CommonError>>>,
			IM: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,&UsiGoMateTimeLimit,
												Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
												Box<dyn FnMut(Vec<UsiInfoSubCommand>) -> Result<(),InfoSendError>>)) -> Result<CheckMate,CommonError>>>,
			IE: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,&GameEndState,
														Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,)) -> Result<(),CommonError>>>,
			IOP: Iterator<Item=(String,SysEventOption)> {

	pub fn new(sender:Sender<Result<ActionKind,String>>,info_send_notifier:Sender<()>) -> MockPlayer<IR,IP,IG,IM,IE,IOP> {
		MockPlayer {
			on_isready:None,
			on_position:None,
			on_think:None,
			on_think_mate:None,
			on_gameover:None,
			options_it:None,
			sender:sender,
			info_send_notifier:info_send_notifier,
		}
	}
}
impl<IR,IP,IG,IM,IE,IOP> fmt::Debug for MockPlayer<IR,IP,IG,IM,IE,IOP>
	where IR: Iterator<Item=Box<dyn FnMut(&mut MockPlayer<IR,IP,IG,IM,IE,IOP>) -> Result<(),CommonError>>>,
			IP: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,Teban,Banmen,
												HashMap<MochigomaKind,u32>,
												HashMap<MochigomaKind,u32>,u32,Vec<Move>)) -> Result<(),CommonError>>>,
			IG: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,&UsiGoTimeLimit,
												Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
												Box<dyn FnMut(Vec<UsiInfoSubCommand>) -> Result<(),InfoSendError>>)) -> Result<BestMove,CommonError>>>,
			IM: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,&UsiGoMateTimeLimit,
												Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
												Box<dyn FnMut(Vec<UsiInfoSubCommand>) -> Result<(),InfoSendError>>)) -> Result<CheckMate,CommonError>>>,
			IE: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,&GameEndState,
														Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,)) -> Result<(),CommonError>>>,
			IOP: Iterator<Item=(String,SysEventOption)> {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "MockPlayer")
	}
}
impl<IR,IP,IG,IM,IE,IOP> USIPlayer<CommonError> for MockPlayer<IR,IP,IG,IM,IE,IOP>
	where IR: Iterator<Item=Box<dyn FnMut(&mut MockPlayer<IR,IP,IG,IM,IE,IOP>) -> Result<(),CommonError>>>,
			IP: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,Teban,Banmen,
												HashMap<MochigomaKind,u32>,
												HashMap<MochigomaKind,u32>,u32,Vec<Move>)) -> Result<(),CommonError>>>,
			IG: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,&UsiGoTimeLimit,
												Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
												Box<dyn FnMut(Vec<UsiInfoSubCommand>) -> Result<(),InfoSendError>>)) -> Result<BestMove,CommonError>>>,
			IM: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,&UsiGoMateTimeLimit,
												Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
												Box<dyn FnMut(Vec<UsiInfoSubCommand>) -> Result<(),InfoSendError>>)) -> Result<CheckMate,CommonError>>>,
			IE: Iterator<Item=Box<dyn FnMut((&mut MockPlayer<IR,IP,IG,IM,IE,IOP>,&GameEndState,
												Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>)) -> Result<(),CommonError>>>,
			IOP: Iterator<Item=(String,SysEventOption)> {
	const ID: &'static str = "mockplayer";
	const AUTHOR: &'static str = "j6k1";
	fn get_option_kinds(&mut self) -> Result<HashMap<String,SysEventOptionKind>,CommonError> {
		let mut kinds:HashMap<String,SysEventOptionKind> = HashMap::new();
		kinds.insert(String::from("USI_Hash"),SysEventOptionKind::Num);
		kinds.insert(String::from("USI_Ponder"),SysEventOptionKind::Bool);

		Ok(kinds)
	}

	fn get_options(&mut self) -> Result<HashMap<String,UsiOptType>,CommonError> {
		let mut options:HashMap<String,UsiOptType> = HashMap::new();
		options.insert(String::from("USI_Hash"),UsiOptType::Spin(1,100,None));
		options.insert(String::from("USI_Ponder"),UsiOptType::Check(Some(false)));
		Ok(options)
	}

	fn take_ready(&mut self) -> Result<(),CommonError> {
		(self.on_isready.as_mut().expect("on take_ready callback is empty.").next().expect("Iterator of on take_ready callback is empty."))(self)
	}

	fn set_option(&mut self,name:String,value:SysEventOption) -> Result<(),CommonError> {
		if (name,value) == self.options_it.as_mut().expect("USI Option is empty.").next().expect("USI Option iterator is empty.") {
			let _ = self.sender.send(Ok(ActionKind::SetOption));
		}
		Ok(())
	}

	fn newgame(&mut self) -> Result<(),CommonError> {
		let _ = self.sender.send(Ok(ActionKind::NewGame));
		Ok(())
	}

	fn set_position(&mut self,teban:Teban,ban:Banmen,ms:HashMap<MochigomaKind,u32>,mg:HashMap<MochigomaKind,u32>,n:u32,m:Vec<Move>)
		-> Result<(),CommonError> {
		(self.on_position.as_mut().expect("on set_position callback is empty.").next().expect("Iterator of on set_position callback is empty."))(
			(self,teban,ban,ms,mg,n,m)
		)
	}

	fn think<L,S>(&mut self,limit:&UsiGoTimeLimit,event_queue:Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
			info_sender:S,_:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<BestMove,CommonError> where L: Logger, S: InfoSender, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static {
		let mut info_sender = info_sender;
		let info_send_notifier = self.info_send_notifier.clone();
		(self.on_think.as_mut().expect("on think callback is empty.").next().expect("Iterator of on think callback is empty."))(
			(self,limit,event_queue,Box::new(move |commands| {
				let r = info_sender.send(commands);

				if let Ok(_) = r {
					let _ = info_send_notifier.send(());
				}
				r
			}))
		)
	}

	fn think_mate<L,S>(&mut self,limit:&UsiGoMateTimeLimit,event_queue:Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
			info_sender:S,_:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<CheckMate,CommonError> where L: Logger, S: InfoSender, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static {
		let mut info_sender = info_sender;
		let info_send_notifier = self.info_send_notifier.clone();
		(self.on_think_mate.as_mut().expect("on think_mate callback is empty.").next().expect("Iterator of on think_mate callback is empty."))(
			(self,limit,event_queue,Box::new(move |commands| {
				let r = info_sender.send(commands);

				if let Ok(_) = r {
					let _ = info_send_notifier.send(());
				}
				r
			}))
		)
	}

	fn on_stop(&mut self,_:&UserEvent) -> Result<(), CommonError> {
		let _ = self.sender.send(Ok(ActionKind::OnStop));
		Ok(())
	}

	fn gameover<L>(&mut self,s:&GameEndState,
			event_queue:Arc<Mutex<EventQueue<UserEvent,UserEventKind>>>,
			_:Arc<Mutex<OnErrorHandler<L>>>)
	-> Result<(),CommonError> where L: Logger, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static {
		(self.on_gameover.as_mut().expect("on gameover callback is empty.").next().expect("Iterator of gameover callback is empty."))(
			(self,s,event_queue)
		)
	}

	fn on_quit(&mut self,_:&UserEvent) -> Result<(), CommonError> {
		let _ = self.sender.send(Ok(ActionKind::OnQuit));
		Ok(())
	}

	fn quit(&mut self) -> Result<(),CommonError> {
		let _ = self.sender.send(Ok(ActionKind::Quit));
		Ok(())
	}
}