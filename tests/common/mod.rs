use std;
use std::fmt;
use std::error;
use std::io;
use std::io::Write;
use std::convert::From;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::iter::Iterator;
use std::ops::Add;
use std::time::Instant;

use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;

use usiagent::event::SystemEventKind;
use usiagent::event::UserEventKind;
use usiagent::error::USIAgentRunningError;
use usiagent::error::USIAgentStartupError;
use usiagent::error::PlayerError;
use usiagent::error::UsiProtocolError;
use usiagent::error::TypeConvertError;
use usiagent::error::InfoSendError;
use usiagent::error::KifuWriteError;
use usiagent::output::USIOutputWriter;
use usiagent::shogi::*;
use usiagent::event::*;
use usiagent::command::*;
use usiagent::rule::*;
use usiagent::logger::Logger;
use usiagent::player::USIPlayer;
use usiagent::player::InfoSender;
use usiagent::player::UsiInfoMessage;
use usiagent::OnErrorHandler;
use usiagent::input::USIInputReader;
use usiagent::selfmatch::SelfMatchKifuWriter;

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

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
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
#[derive(Debug)]
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
		let l = self.rcv.recv().expect("Failed to receive input.");

		Ok(l.to_string())
	}
}
#[derive(Debug)]
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
#[derive(Debug)]
pub struct StdErrorLogger {

}
impl StdErrorLogger {
	pub fn new() -> StdErrorLogger {
		StdErrorLogger {

		}
	}
}
impl Logger for StdErrorLogger {
	fn logging(&mut self, message:&String) -> bool {
		let _ = writeln!(&mut std::io::stderr(),"errror: {}",message);
		true
	}
}
#[derive(Debug)]
pub struct MockLogger {
	sender:Sender<String>,
}
impl MockLogger {
	pub fn new(sender:Sender<String>) -> MockLogger {
		MockLogger {
			sender:sender
		}
	}
}
impl Logger for MockLogger {
	fn logging(&mut self, message:&String) -> bool {
		self.sender.send(message.clone()).is_ok()
	}
}
#[allow(dead_code)]
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug, Hash)]
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
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug, Hash)]
pub enum EventState {
	GameStart = 0,
	Moved,
	GameEnd,
	Abort,
}
pub struct MockInfoSender {
	sender:Sender<UsiInfoMessage>
}
impl MockInfoSender {
	pub fn new(sender:Sender<UsiInfoMessage>) -> MockInfoSender {
		MockInfoSender {
			sender:sender
		}
	}
}
impl InfoSender for MockInfoSender {
	fn send(&mut self,commands:Vec<UsiInfoSubCommand>) -> Result<(), InfoSendError> {
		if let Err(_) = self.sender.send(UsiInfoMessage::Commands(commands)) {
			Err(InfoSendError::Fail(String::from(
				"info command send failed.")))
		} else {
			Ok(())
		}
	}
}
#[derive(Debug)]
pub struct MockSfenKifuWriter {
	sender:Sender<String>,
}
impl MockSfenKifuWriter {
	pub fn new(sender:Sender<String>) -> MockSfenKifuWriter {
		MockSfenKifuWriter {
			sender:sender,
		}
	}
}
impl SelfMatchKifuWriter for MockSfenKifuWriter {
	fn write(&mut self,initial_sfen:&String,m:&Vec<Move>) -> Result<(),KifuWriteError> {
		let sfen = self.to_sfen(initial_sfen,m)?;

		let _ = self.sender.send(sfen);

		Ok(())
	}
}
impl Clone for MockInfoSender {
	fn clone(&self) -> MockInfoSender {
		MockInfoSender::new(self.sender.clone())
	}
}
pub struct ConsumedIterator<T> where T: Send + 'static {
	v:Vec<T>,
}
impl<T> ConsumedIterator<T> where T: Send + 'static {
	pub fn new(v:Vec<T>) -> ConsumedIterator<T> {
		ConsumedIterator {
			v:v,
		}
	}
}
impl<T> Iterator for ConsumedIterator<T> where T: Send + 'static {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.v.len() == 0 {
			None
		} else {
			Some(self.v.remove(0))
		}
	}
}
pub struct MockPlayer {
	pub on_isready: ConsumedIterator<Box<(dyn FnMut(&mut MockPlayer) -> Result<(),CommonError> + Send + 'static)>>,
	pub on_newgame: ConsumedIterator<Box<(dyn FnMut(&mut MockPlayer) -> Result<(),CommonError> + Send + 'static)>>,
	pub on_position: ConsumedIterator<Box<(dyn FnMut(&mut MockPlayer,Teban,Banmen,
												HashMap<MochigomaKind,u32>,
												HashMap<MochigomaKind,u32>,u32,Vec<Move>) -> Result<(),CommonError> + Send + 'static)>>,
	pub on_think: ConsumedIterator<Box<(dyn FnMut(&mut MockPlayer,
												Option<Instant>,
												&UsiGoTimeLimit,
												Arc<Mutex<UserEventQueue>>,
												Box<(dyn FnMut(Vec<UsiInfoSubCommand>) -> Result<(),InfoSendError> + Send + 'static)>,
												Box<(dyn FnMut(&mut MockPlayer) -> Result<bool,CommonError> + Send + 'static)>
	) -> Result<BestMove,CommonError> + Send + 'static)>>,

	pub on_think_mate: ConsumedIterator<Box<(dyn FnMut(&mut MockPlayer,&UsiGoMateTimeLimit,
												Arc<Mutex<UserEventQueue>>,
												Box<(dyn FnMut(Vec<UsiInfoSubCommand>) -> Result<(),InfoSendError> + Send + 'static)>,
												Box<(dyn FnMut(&mut MockPlayer) -> Result<bool,CommonError> + Send + 'static)>
	) -> Result<CheckMate,CommonError> + Send + 'static)>>,

	pub on_gameover: ConsumedIterator<Box<(dyn FnMut(&mut MockPlayer,&GameEndState,
												Arc<Mutex<UserEventQueue>>)
				-> Result<(),CommonError> + Send + 'static)>>,
	pub options_it:ConsumedIterator<(String,SysEventOption)>,
	pub sender:Sender<Result<ActionKind,String>>,
	info_send_notifier:Sender<()>,
	pub started:bool,
	pub stop:bool,
	pub quited:bool,
	pub kyokumen:Option<Kyokumen>,
	pub ponderhit_time:Option<Instant>,
	pub game_start_time:Option<Instant>,
}
impl MockPlayer {
	pub fn new(sender:Sender<Result<ActionKind,String>>,
				info_send_notifier:Sender<()>,
				on_isready: ConsumedIterator<Box<(dyn FnMut(&mut MockPlayer) -> Result<(),CommonError> + Send + 'static)>>,
				on_newgame: ConsumedIterator<Box<(dyn FnMut(&mut MockPlayer) -> Result<(),CommonError> + Send + 'static)>>,
				on_position: ConsumedIterator<Box<(dyn FnMut(&mut MockPlayer,Teban,Banmen,
															HashMap<MochigomaKind,u32>,
															HashMap<MochigomaKind,u32>,u32,Vec<Move>
				) -> Result<(),CommonError> + Send + 'static)>>,

				on_think: ConsumedIterator<Box<(dyn FnMut(&mut MockPlayer,
															Option<Instant>,
															&UsiGoTimeLimit,
															Arc<Mutex<UserEventQueue>>,
															Box<(dyn FnMut(Vec<UsiInfoSubCommand>) -> Result<(),InfoSendError> + Send + 'static)>,
															Box<(dyn FnMut(&mut MockPlayer) -> Result<bool,CommonError> + Send + 'static)>
				) -> Result<BestMove,CommonError> + Send + 'static)>>,

				on_think_mate: ConsumedIterator<Box<(dyn FnMut(&mut MockPlayer,&UsiGoMateTimeLimit,
															Arc<Mutex<UserEventQueue>>,
															Box<(dyn FnMut(Vec<UsiInfoSubCommand>) -> Result<(),InfoSendError> + Send + 'static)>,
															Box<(dyn FnMut(&mut MockPlayer) -> Result<bool,CommonError> + Send + 'static)>
				) -> Result<CheckMate,CommonError> + Send + 'static)>>,

				on_gameover: ConsumedIterator<Box<(dyn FnMut(&mut MockPlayer,&GameEndState,
															Arc<Mutex<UserEventQueue>>)
				-> Result<(),CommonError> + Send + 'static)>>
	) -> MockPlayer {
		MockPlayer {
			on_isready:on_isready,
			on_newgame: on_newgame,
			on_position:on_position,
			on_think:on_think,
			on_think_mate:on_think_mate,
			on_gameover:on_gameover,
			options_it:ConsumedIterator::new(vec![
				(String::from("OptionButton"),SysEventOption::Exist),
				(String::from("OptionCheck"),SysEventOption::Bool(true)),
				(String::from("OptionCombo"),SysEventOption::Str(String::from("cccc"))),
				(String::from("OptionCombo2"),SysEventOption::Str(String::from("eeee"))),
				(String::from("OptionFileName"),SysEventOption::Str(String::from("book.bin"))),
				(String::from("OptionFileName2"),SysEventOption::Str(String::from("book2.bin"))),
				(String::from("OptionSpin"),SysEventOption::Num(25)),
				(String::from("OptionString"),SysEventOption::Str(String::from("string.."))),
				(String::from("OptionString2"),SysEventOption::Str(String::from("string..."))),
				(String::from("USI_Hash"),SysEventOption::Num(1000)),
				(String::from("USI_Ponder"),SysEventOption::Bool(false)),
			]),
			sender:sender,
			info_send_notifier:info_send_notifier,
			started:false,
			stop:false,
			quited:false,
			kyokumen:None,
			ponderhit_time:None,
			game_start_time:None,
		}
	}

	fn think_inner<L,S>(&mut self,think_start_time:Option<Instant>,limit:&UsiGoTimeLimit,event_queue:Arc<Mutex<UserEventQueue>>,
			info_sender:S,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<BestMove,CommonError> where L: Logger, S: InfoSender, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static {
		let mut info_sender = info_sender.clone();
		let info_send_notifier = self.info_send_notifier.clone();
		let event_queue = event_queue.clone();
		let on_error_handler = on_error_handler.clone();

		(self.on_think.next().expect("Iterator of on think callback is empty."))(
			self,think_start_time,limit,event_queue.clone(),Box::new(move |commands| {
				let r = info_sender.send(commands);

				if let Ok(_) = r {
					let _ = info_send_notifier.send(());
				}
				r
			}),Box::new(move |player| {
				player.handle_events(&*event_queue,&*on_error_handler)
			})
		)
	}
}
impl fmt::Debug for MockPlayer {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "MockPlayer")
	}
}
impl USIPlayer<CommonError> for MockPlayer {
	const ID: &'static str = "mockplayer";
	const AUTHOR: &'static str = "j6k1";
	fn get_option_kinds(&mut self) -> Result<BTreeMap<String,SysEventOptionKind>,CommonError> {
		let mut kinds:BTreeMap<String,SysEventOptionKind> = BTreeMap::new();
		kinds.insert(String::from("USI_Hash"),SysEventOptionKind::Num);
		kinds.insert(String::from("USI_Ponder"),SysEventOptionKind::Bool);
		kinds.insert(String::from("OptionButton"),SysEventOptionKind::Exist);
		kinds.insert(String::from("OptionCheck"),SysEventOptionKind::Bool);
		kinds.insert(String::from("OptionCombo"),SysEventOptionKind::Str);
		kinds.insert(String::from("OptionCombo2"),SysEventOptionKind::Str);
		kinds.insert(String::from("OptionFileName"),SysEventOptionKind::Str);
		kinds.insert(String::from("OptionFileName2"),SysEventOptionKind::Str);
		kinds.insert(String::from("OptionSpin"),SysEventOptionKind::Num);
		kinds.insert(String::from("OptionString"),SysEventOptionKind::Str);
		kinds.insert(String::from("OptionString2"),SysEventOptionKind::Str);

		Ok(kinds)
	}

	fn get_options(&mut self) -> Result<BTreeMap<String,UsiOptType>,CommonError> {
		let mut options:BTreeMap<String,UsiOptType> = BTreeMap::new();
		options.insert(String::from("USI_Hash"),UsiOptType::Spin(1,100,None));
		options.insert(String::from("USI_Ponder"),UsiOptType::Check(Some(false)));
		options.insert(String::from("OptionButton"),UsiOptType::Button);
		options.insert(String::from("OptionCheck"),UsiOptType::Check(None));
		options.insert(String::from("OptionCombo"),UsiOptType::Combo(Some(String::from("bbbb")),
																	["bbbb","cccc"]
																		.into_iter()
																		.map(|&s| String::from(s))
																		.collect::<Vec<String>>()));
		options.insert(String::from("OptionCombo2"),UsiOptType::Combo(None,["dddd","eeee"]
																		.into_iter()
																		.map(|&s| String::from(s))
																		.collect::<Vec<String>>()));
		options.insert(String::from("OptionFileName"),UsiOptType::FileName(Some(String::from("filename."))));
		options.insert(String::from("OptionFileName2"),UsiOptType::FileName(None));
		options.insert(String::from("OptionSpin"),UsiOptType::Spin(5,50,Some(10)));
		options.insert(String::from("OptionString"),UsiOptType::String(Some(String::from("string."))));
		options.insert(String::from("OptionString2"),UsiOptType::String(None));
		Ok(options)
	}

	fn take_ready(&mut self) -> Result<(),CommonError> {
		(self.on_isready.next().expect("Iterator of on take_ready callback is empty."))(self)
	}

	fn set_option(&mut self,name:String,value:SysEventOption) -> Result<(),CommonError> {
		let (n,v) = self.options_it.next().expect("on set_option iterator is empty.");

		if (&name,&value) == (&n,&v) {
			let _ = self.sender.send(Ok(ActionKind::SetOption));
		} else {
			let _ = self.sender.send(Err(format!("The option {} value is different. {:?} Is correct.",name,v)));
		}
		Ok(())
	}

	fn newgame(&mut self) -> Result<(),CommonError> {
		self.stop = false;
		self.started = true;
		self.game_start_time = Some(Instant::now());
		(self.on_newgame.next().expect("Iterator of on newgame callback is empty."))(self)
	}

	fn set_position(&mut self,teban:Teban,ban:Banmen,ms:HashMap<MochigomaKind,u32>,mg:HashMap<MochigomaKind,u32>,n:u32,m:Vec<Move>)
		-> Result<(),CommonError> {
		(self.on_position.next().expect("Iterator of on set_position callback is empty."))(
			self,teban,ban,ms,mg,n,m
		)
	}

	fn think<L,S>(&mut self,think_start_time:Instant,limit:&UsiGoTimeLimit,event_queue:Arc<Mutex<UserEventQueue>>,
			info_sender:S,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<BestMove,CommonError> where L: Logger, S: InfoSender, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static {
		self.think_inner(Some(think_start_time), limit, event_queue, info_sender, on_error_handler)
	}

	fn think_ponder<L,S>(&mut self,limit:&UsiGoTimeLimit,event_queue:Arc<Mutex<UserEventQueue>>,
			info_sender:S,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<BestMove,CommonError> where L: Logger, S: InfoSender, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static {
		self.think_inner(None, limit, event_queue, info_sender, on_error_handler)
	}

	fn think_mate<L,S>(&mut self,limit:&UsiGoMateTimeLimit,event_queue:Arc<Mutex<UserEventQueue>>,
			info_sender:S,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<CheckMate,CommonError> where L: Logger, S: InfoSender, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static {
		let mut info_sender = info_sender.clone();
		let info_send_notifier = self.info_send_notifier.clone();
		let event_queue = event_queue.clone();
		let on_error_handler = on_error_handler.clone();

		(self.on_think_mate.next().expect("Iterator of on think_mate callback is empty."))(
			self,limit,event_queue.clone(),Box::new(move |commands| {
				let r = info_sender.send(commands);

				if let Ok(_) = r {
					let _ = info_send_notifier.send(());
				}
				r
			}),Box::new(move |player| {
				player.handle_events(&*event_queue,&*on_error_handler)
			})
		)
	}

	fn on_stop(&mut self,_:&UserEvent) -> Result<(), CommonError> {
		let _ = self.sender.send(Ok(ActionKind::OnStop));
		self.stop = true;
		Ok(())
	}

	fn on_ponderhit(&mut self,e:&UserEvent) -> Result<(), CommonError> {
		let _ = self.sender.send(Ok(ActionKind::OnStop));
		if let &UserEvent::PonderHit(t) = e {
			self.ponderhit_time = Some(t);
		}
		Ok(())
	}

	fn gameover<L>(&mut self,s:&GameEndState,
			event_queue:Arc<Mutex<UserEventQueue>>,
			_:Arc<Mutex<OnErrorHandler<L>>>)
	-> Result<(),CommonError> where L: Logger, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static {
		self.started = false;
		(self.on_gameover.next()
			.expect("Iterator of gameover callback is empty."))(
			self,s,event_queue
		)
	}

	fn on_quit(&mut self,_:&UserEvent) -> Result<(), CommonError> {
		let _ = self.sender.send(Ok(ActionKind::OnQuit));
		self.stop = true;
		self.quited = true;
		self.started = false;
		Ok(())
	}

	fn quit(&mut self) -> Result<(),CommonError> {
		let _ = self.sender.send(Ok(ActionKind::Quit));
		self.stop = true;
		self.quited = true;
		self.started = false;
		Ok(())
	}
}
