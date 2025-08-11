//! 各種エラーの定義
use std::error;
use std::error::Error;
use std::fmt;
use std::io;
use std::sync::{mpsc, MutexGuard};
use std::sync::PoisonError;
use std::num::ParseIntError;

use crossbeam_channel::SendError;
use crossbeam_channel::RecvError;

use command::UsiCommand;
use player::UsiInfoMessage;
use selfmatch::SelfMatchMessage;

/// イベント処理時のエラー
#[derive(Debug)]
pub enum EventDispatchError<'a,T,K,E>
	where T: fmt::Debug + 'a, K: fmt::Debug, E: Error + fmt::Debug + 'static {
	/// イベントハンドラ内エラー発生
	ErrorFromHandler(EventHandlerError<K,E>),
	/// イベントキューの排他的ロックの取得に失敗した
	MutexLockFailedError(PoisonError<MutexGuard<'a,T>>),
	/// イベントの受信に失敗した
	RecvError(mpsc::RecvError),
	/// エラーが含まれる
	ContainError,
}
/// イベントハンドラ内で発生したエラー
#[derive(Debug)]
pub enum EventHandlerError<K,E> where K: fmt::Debug, E: Error + fmt::Debug + 'static {
	/// その他のエラー
	Fail(String),
	/// 内部状態不正
	InvalidState(K),
	/// `USIPlayer`の実装が投げたエラー
	PlayerError(E),
}
impl<K,E> fmt::Display for EventHandlerError<K,E> where K: fmt::Debug, E: Error + fmt::Debug {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		EventHandlerError::Fail(ref s) => write!(f,"An error occurred while executing the event handler. ({})",s),
	 		EventHandlerError::InvalidState(ref e) => write!(f,
	 			"The type of event passed and the event being processed do not match. (Event kind = {:?})", e),
		 	EventHandlerError::PlayerError(_) => write!(f,"An error occurred while processing the player object in the event handler."),
	 	}
	 }
}
impl<K,E> error::Error for EventHandlerError<K,E> where K: fmt::Debug, E: Error + fmt::Debug {
	 fn description(&self) -> &str {
	 	match *self {
	 		EventHandlerError::Fail(_) => "An error occurred while executing the event handler.",
	 		EventHandlerError::InvalidState(_) => "The type of event passed and the event being processed do not match.",
		 	EventHandlerError::PlayerError(_) => "An error occurred while processing the player object in the event handler.",
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		EventHandlerError::Fail(_) => None,
	 		EventHandlerError::InvalidState(_) => None,
	 		EventHandlerError::PlayerError(ref e) => Some(e),
	 	}
	 }
}
impl<'a,T,K,E> fmt::Display for EventDispatchError<'a,T,K,E>
	where T: fmt::Debug, K: fmt::Debug, E: Error + fmt::Debug {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		EventDispatchError::ErrorFromHandler(_) => write!(f, "An error occurred while processing an event."),
	 		EventDispatchError::MutexLockFailedError(_) => write!(f, "Could not get exclusive lock on object."),
			EventDispatchError::RecvError(_) => write!(f, "An error occurred while receiving data."),
	 		EventDispatchError::ContainError => write!(f, "One or more errors occurred while executing the event handler."),
	 	}
	 }
}
impl<'a,T,K,E> error::Error for EventDispatchError<'a,T,K,E>
	where T: fmt::Debug, K: fmt::Debug + 'static, E: Error + fmt::Debug {
	 fn description(&self) -> &str {
	 	match *self {
	 		EventDispatchError::ErrorFromHandler(_) => "An error occurred while processing an event.",
	 		EventDispatchError::MutexLockFailedError(_) => "Could not get exclusive lock on object.",
			EventDispatchError::RecvError(_) => "An error occurred while receiving data.",
	 		EventDispatchError::ContainError => "Error executing event handler",
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		EventDispatchError::ErrorFromHandler(ref e) => Some(e),
	 		EventDispatchError::MutexLockFailedError(_) => None,
			EventDispatchError::RecvError(ref e) => Some(e),
	 		EventDispatchError::ContainError => None,
	 	}
	 }
}
impl<'a,T,K,E> From<PoisonError<MutexGuard<'a,T>>> for EventDispatchError<'a,T,K,E>
	where T: fmt::Debug + 'a, K: fmt::Debug, E: Error + fmt::Debug {
	fn from(err: PoisonError<MutexGuard<'a,T>>) -> EventDispatchError<'a,T,K,E> {
		EventDispatchError::MutexLockFailedError(err)
	}
}
impl<'a,T,K,E> From<EventHandlerError<K,E>> for EventDispatchError<'a,T,K,E>
	where T: fmt::Debug, K: fmt::Debug, E: Error + fmt::Debug {
	fn from(err: EventHandlerError<K,E>) -> EventDispatchError<'a,T,K,E> {
		EventDispatchError::ErrorFromHandler(err)
	}
}
impl<'a,T,K,E> From<mpsc::RecvError> for EventDispatchError<'a,T,K,E>
	where T: fmt::Debug + 'a, K: fmt::Debug, E: Error + fmt::Debug {
	fn from(err: mpsc::RecvError) -> EventDispatchError<'a,T,K,E> {
		EventDispatchError::RecvError(err)
	}
}
/// 状態不正
#[derive(Debug,Eq,PartialEq)]
pub struct InvalidStateError(pub String);
impl fmt::Display for InvalidStateError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		InvalidStateError(ref s) => write!(f, "{}",s)
	 	}
	 }
}
impl error::Error for InvalidStateError {
	 fn description(&self) -> &str {
	 	match *self {
	 		InvalidStateError(_) => "invalid state."
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		InvalidStateError(_) => None
	 	}
	 }
}
/// 盤面上の段に値をマッピングできなかった
#[derive(Debug,Eq,PartialEq)]
pub struct DanConvertError(pub u32);
impl fmt::Display for DanConvertError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		DanConvertError(n) => write!(f, "The 'dan' value {} is outside the range of valid values.",n)
	 	}
	 }
}
impl error::Error for DanConvertError {
	 fn description(&self) -> &str {
	 	match *self {
	 		DanConvertError(_) => "The value of the 'dan' is out of the range of valid values"
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		DanConvertError(_) => None
	 	}
	 }
}
/// 指し手の文字列表現と内部表現の変換時のエラー
#[derive(Debug,Eq,PartialEq)]
pub enum ToMoveStringConvertError {
	/// 値から盤面の段への変換に失敗した
	CharConvert(DanConvertError),
	/// 思考を中断したケースで指し手文字列を生成しようとした
	AbortedError,
}
impl fmt::Display for ToMoveStringConvertError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		ToMoveStringConvertError::CharConvert(_) => {
	 			write!(f, "Conversion of move to string representation failed.")
	 		},
	 		ToMoveStringConvertError::AbortedError => {
	 			write!(f,"The command string can not be generated because the operation was interrupted.")
	 		}
	 	}
	 }
}
impl error::Error for ToMoveStringConvertError {
	 fn description(&self) -> &str {
	 	match *self {
	 		ToMoveStringConvertError::CharConvert(_) => {
	 			"Conversion of move to string representation failed."
	 		},
	 		ToMoveStringConvertError::AbortedError => {
	 			"The command string can not be generated because the operation was interrupted."
	 		}
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		ToMoveStringConvertError::CharConvert(ref e) => Some(e),
	 		ToMoveStringConvertError::AbortedError => None,
	 	}
	 }
}
impl From<DanConvertError> for ToMoveStringConvertError {
	fn from(err: DanConvertError) -> ToMoveStringConvertError {
		ToMoveStringConvertError::CharConvert(err)
	}
}
/// USIコマンドの生成時のエラー
#[derive(Debug,Eq,PartialEq)]
pub enum UsiOutputCreateError {
	/// 生成元のオブジェクトの状態が不正
	ValidationError(UsiCommand),
	/// 状態が不正
	InvalidStateError(String),
	/// infoコマンドの生成元のオブジェクトの状態が不正
	InvalidInfoCommand(String),
	/// 指し手の文字列生成時のエラー
	ConvertError(ToMoveStringConvertError),
	/// 思考を中断したケースで指し手文字列を生成しようとした
	AbortedError,
}
impl fmt::Display for UsiOutputCreateError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		UsiOutputCreateError::ConvertError(_) => write!(f, "Failed to generate command string output to USI protocol."),
	 		UsiOutputCreateError::InvalidStateError(ref s) => write!(f, "The state of {} is invalid.", s),
	 		UsiOutputCreateError::InvalidInfoCommand(ref s) => write!(f, "The content of the info command is invalid. ({})", s),
	 		UsiOutputCreateError::ValidationError(ref cmd) => {
	 			match *cmd {
	 				UsiCommand::UsiBestMove(_) => write!(f, "The state of the object that generated the bestmove command is invalid."),
	 				UsiCommand::UsiInfo(_) => write!(f, "The state of the object that generated the info command is invalid."),
	 				UsiCommand::UsiCheckMate(_) => write!(f, "The state of the object that generated the checkmate command is invalid."),
		 			_ => write!(f,"An unexpected exception occurred in command validation."),
	 			}
	 		},
	 		UsiOutputCreateError::AbortedError => write!(f,"The command string can not be generated because the operation was interrupted."),
	 	}
	 }
}
impl error::Error for UsiOutputCreateError {
	 fn description(&self) -> &str {
	 	match *self {
	 		UsiOutputCreateError::ConvertError(_) => "Failed to generate command string output to USI protocol.",
	 		UsiOutputCreateError::InvalidStateError(_) => "The state of the command generation object is invalid",
	 		UsiOutputCreateError::InvalidInfoCommand(_) => "The content of the info command is invalid.",
	 		UsiOutputCreateError::ValidationError(ref cmd) => {
	 			match *cmd {
	 				UsiCommand::UsiBestMove(_) => "The state of the object that generated the bestmove command is invalid",
	 				UsiCommand::UsiInfo(_) => "The state of the object that generated the info command is invalid",
	 				UsiCommand::UsiCheckMate(_) => "The state of the object that generated the checkmate command is invalid",
	 				_ => "An unexpected exception occurred in command validation",
	 			}
	 		},
	 		UsiOutputCreateError::AbortedError => "The command string can not be generated because the operation was interrupted.",
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		UsiOutputCreateError::ConvertError(ref e) => Some(e),
	 		UsiOutputCreateError::InvalidStateError(_) => None,
	 		UsiOutputCreateError::InvalidInfoCommand(_)=> None,
	 		UsiOutputCreateError::ValidationError(_) => None,
	 		UsiOutputCreateError::AbortedError => None,
	 	}
	 }
}
impl From<ToMoveStringConvertError> for UsiOutputCreateError {
	fn from(err: ToMoveStringConvertError) -> UsiOutputCreateError {
		UsiOutputCreateError::ConvertError(err)
	}
}
impl<T,E> From<UsiOutputCreateError> for EventHandlerError<T,E> where T: fmt::Debug, E: Error + fmt::Debug {
	fn from(err: UsiOutputCreateError) -> EventHandlerError<T,E> {
		EventHandlerError::Fail(err.to_string())
	}
}
impl<T,E> From<E> for EventHandlerError<T,E> where T: fmt::Debug, E: PlayerError {
	fn from(err: E) -> EventHandlerError<T,E> {
		EventHandlerError::PlayerError(err)
	}
}
/// infoコマンド出力時のエラー
#[derive(Debug,Eq,PartialEq)]
pub enum InfoSendError {
	Fail(String)
}
impl fmt::Display for InfoSendError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		InfoSendError::Fail(ref s) => write!(f,"An error occurred when sending the info command. ({})",s),
	 	}
	 }
}
impl error::Error for InfoSendError {
	 fn description(&self) -> &str {
	 	match *self {
	 		InfoSendError::Fail(_) => "An error occurred when sending the info command.",
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		InfoSendError::Fail(_) => None,
	 	}
	 }
}
impl From<UsiOutputCreateError> for InfoSendError {
	fn from(e: UsiOutputCreateError) -> InfoSendError {
		InfoSendError::Fail(format!("{}",e))
	}
}
/// infoコマンド送信スレッドのエラー
#[derive(Debug)]
pub enum InfoSendWorkerError {
	RecvError(std::sync::mpsc::RecvError),
	SendError(std::sync::mpsc::SendError<UsiInfoMessage>)
}
impl fmt::Display for InfoSendWorkerError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			InfoSendWorkerError::RecvError(_) => write!(f,"An error occurred when receiving the message."),
			InfoSendWorkerError::SendError(_) => write!(f,"An error occurred when sending the message."),
		}
	}
}
impl error::Error for InfoSendWorkerError {
	fn description(&self) -> &str {
		match *self {
			InfoSendWorkerError::RecvError(_) => "An error occurred when receiving the message.",
			InfoSendWorkerError::SendError(_) => "An error occurred while sending the message.",
		}
	}

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match *self {
			InfoSendWorkerError::RecvError(ref e) => Some(e),
			InfoSendWorkerError::SendError(ref e) => Some(e)
		}
	}
}
impl From<std::sync::mpsc::RecvError> for InfoSendWorkerError {
	fn from(e: std::sync::mpsc::RecvError) -> InfoSendWorkerError {
		InfoSendWorkerError::RecvError(e)
	}
}
impl From<std::sync::mpsc::SendError<UsiInfoMessage>> for InfoSendWorkerError {
	fn from(e: std::sync::mpsc::SendError<UsiInfoMessage>) -> InfoSendWorkerError {
		InfoSendWorkerError::SendError(e)
	}
}
/// USIコマンド文字列の型変換エラー
#[derive(Debug,Eq,PartialEq)]
pub enum TypeConvertError<T> where T: fmt::Debug + fmt::Display {
	/// 書式エラー
	SyntaxError(T),
	/// 内部実装の誤りを検出
	LogicError(T),
}
impl<T> fmt::Display for TypeConvertError<T> where T: fmt::Debug + fmt::Display {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		TypeConvertError::SyntaxError(ref e) => write!(f, "An error occurred in type conversion. from = ({:?})",e),
		 	TypeConvertError::LogicError(ref e) => write!(f, "An error occurred in type conversion (logic error, {})",e),
	 	}
	 }
}
impl<T> error::Error for TypeConvertError<T> where T: fmt::Debug + fmt::Display {
	 fn description(&self) -> &str {
	 	match *self {
	 		TypeConvertError::SyntaxError(_) => "An error occurred in type conversion",
	 		TypeConvertError::LogicError(_) => "An error occurred in type conversion (logic error)",
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		TypeConvertError::SyntaxError(_) => None,
	 		TypeConvertError::LogicError(_) => None,
	 	}
	 }
}
impl From<ParseIntError> for TypeConvertError<String> where String: fmt::Debug {
	fn from(_: ParseIntError) -> TypeConvertError<String> {
		TypeConvertError::SyntaxError(String::from("Failed parse string to integer."))
	}
}
/// USIAgent開始時のエラー
#[derive(Debug)]
pub enum USIAgentStartupError<E> where E: PlayerError {
	/// オブジェクトの排他的ロックの獲得に失敗
	MutexLockFailedOtherError(String),
	/// 入出力時のエラー
	IOError(String),
	/// `USIPlayer`の実装がエラーを投げた
	PlayerError(E),
}
impl<E> fmt::Display for USIAgentStartupError<E> where E: PlayerError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
		 	USIAgentStartupError::MutexLockFailedOtherError(ref s) => write!(f, "Could not get exclusive lock on object. ({})",s),
		 	USIAgentStartupError::IOError(ref s) => write!(f, "IO Error. ({})",s),
		 	USIAgentStartupError::PlayerError(_) => write!(f,"An error occurred in the processing within the player object."),
	 	}
	 }
}
impl<E> error::Error for USIAgentStartupError<E> where E: PlayerError {
	 fn description(&self) -> &str {
	 	match *self {
	 		USIAgentStartupError::MutexLockFailedOtherError(_) => "Could not get exclusive lock on object.",
	 		USIAgentStartupError::IOError(_) => "IO Error.",
	 		USIAgentStartupError::PlayerError(_) => "An error occurred in the processing within the player object.",
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		USIAgentStartupError::MutexLockFailedOtherError(_) => None,
	 		USIAgentStartupError::IOError(_) => None,
	 		USIAgentStartupError::PlayerError(ref e) => Some(e),
	 	}
	 }
}
/// USIAgentの実行中のエラー
#[derive(Debug)]
pub enum USIAgentRunningError<'a,T,E> where T: fmt::Debug + 'a, E: PlayerError {
	/// オブジェクトの排他的ロックの獲得に失敗
	MutexLockFailedError(PoisonError<MutexGuard<'a,T>>),
	/// 開始時のエラー
	StartupError(USIAgentStartupError<E>),
}
impl<'a,T,E> fmt::Display for USIAgentRunningError<'a,T,E> where T: fmt::Debug, E: PlayerError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		USIAgentRunningError::MutexLockFailedError(_) => write!(f, "Could not get exclusive lock on object."),
		 	USIAgentRunningError::StartupError(_) => write!(f,"An error occurred during startup."),
	 	}
	 }
}
impl<'a,T,E> error::Error for USIAgentRunningError<'a,T,E> where T: fmt::Debug, E: PlayerError {
	 fn description(&self) -> &str {
	 	match *self {
	 		USIAgentRunningError::MutexLockFailedError(_) => "Could not get exclusive lock on object.",
	 		USIAgentRunningError::StartupError(_) => "An error occurred during the startup process of USIAgent.",
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		USIAgentRunningError::MutexLockFailedError(_) => None,
	 		USIAgentRunningError::StartupError(ref e) => Some(e),
	 	}
	 }
}
impl<'a,T,E> From<PoisonError<MutexGuard<'a,T>>> for USIAgentRunningError<'a,T,E>
	where T: fmt::Debug + 'a, E: PlayerError {
	fn from(err: PoisonError<MutexGuard<'a,T>>) -> USIAgentRunningError<'a,T,E> {
		USIAgentRunningError::MutexLockFailedError(err)
	}
}
impl<'a,T,E> From<USIAgentStartupError<E>> for USIAgentRunningError<'a,T,E>
	where T: fmt::Debug + 'a, E: PlayerError {
	fn from(err: USIAgentStartupError<E>) -> USIAgentRunningError<'a,T,E> {
		USIAgentRunningError::StartupError(err)
	}
}
/// 将棋のルールに関するエラー
#[derive(Debug,Eq,PartialEq)]
pub enum ShogiError {
	/// 状態不正
	InvalidState(String),
}
impl fmt::Display for ShogiError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
		 	ShogiError::InvalidState(ref s) => write!(f,"{}",s)
	 	}
	 }
}
impl error::Error for ShogiError {
	 fn description(&self) -> &str {
	 	match *self {
	 		ShogiError::InvalidState(_) => "invalid state.",
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		ShogiError::InvalidState(_) => None,
	 	}
	 }
}
/// usiプロトコル関係のエラー
#[derive(Debug,Eq,PartialEq)]
pub enum UsiProtocolError {
	/// 状態不正
	InvalidState(String),
}
impl fmt::Display for UsiProtocolError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
		 	UsiProtocolError::InvalidState(ref s) => write!(f,"invalid state.(protocol was not processed proper, {})",s)
	 	}
	 }
}
impl error::Error for UsiProtocolError {
	 fn description(&self) -> &str {
	 	match *self {
	 		UsiProtocolError::InvalidState(_) => "invalid state.(protocol was not processed proper)",
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		UsiProtocolError::InvalidState(_) => None,
	 	}
	 }
}
/// 自己対局機能実行時のエラー
#[derive(Debug)]
pub enum SelfMatchRunningError<E> where E: PlayerError {
	/// 状態不正
	InvalidState(String),
	/// `USIPlayer`の実装がエラーを投げた
	PlayerError(E),
	/// プレイヤースレッド内でエラー発生
	PlayerThreadError(usize),
	/// 入出力時のエラー
	IOError(io::Error),
	/// 棋譜書き込み時のエラー
	KifuWriteError(KifuWriteError),
	/// `crossbeam_channel`によるメッセージ受信時のエラー
	RecvError(RecvError),
	/// `crossbeam_channel`によるメッセージ送信時のエラー
	SendError(SendError<SelfMatchMessage>),
	/// ブリッジスレッド内でエラー発生
	ThreadJoinFailed(String),
	/// その他
	Fail(String),
}
impl<E> fmt::Display for SelfMatchRunningError<E> where E: PlayerError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
		 	SelfMatchRunningError::InvalidState(ref s) => write!(f,"invalid state. ({})",s),
			SelfMatchRunningError::PlayerError(_) => write!(f,"An error occurred in player thread."),
		 	SelfMatchRunningError::PlayerThreadError(n) => write!(f,"An error occurred in player {}'s thread.",n),
		 	SelfMatchRunningError::IOError(_) => write!(f,"IO Error."),
			SelfMatchRunningError::KifuWriteError(_) => write!(f,"An error occurred when recording kifu.s"),
		 	SelfMatchRunningError::RecvError(_) => write!(f,"An error occurred when receiving the message."),
		 	SelfMatchRunningError::SendError(_) => write!(f,"An error occurred when sending the message."),
		 	SelfMatchRunningError::ThreadJoinFailed(ref s) => write!(f,"An panic occurred in child thread. ({})",s),
		 	SelfMatchRunningError::Fail(ref s) => write!(f,"An error occurred while running the self-match. ({})",s),
	 	}
	 }
}
impl<E> error::Error for SelfMatchRunningError<E> where E: PlayerError {
	 fn description(&self) -> &str {
	 	match *self {
	 		SelfMatchRunningError::InvalidState(_) => "invalid state.",
	 		SelfMatchRunningError::PlayerError(_) => "An error occurred in player thread.",
	 		SelfMatchRunningError::PlayerThreadError(_) => "An error occurred in player thread.",
		 	SelfMatchRunningError::IOError(_) => "IO Error.",
		 	SelfMatchRunningError::KifuWriteError(_) => "There was an error writing kifu.",
		 	SelfMatchRunningError::RecvError(_) => "An error occurred when receiving the message.",
		 	SelfMatchRunningError::SendError(_) => "An error occurred while sending the message.",
			SelfMatchRunningError::ThreadJoinFailed(_) => "An panic occurred in child thread.",
	 		SelfMatchRunningError::Fail(_) => "An error occurred while running the self-match.",
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		SelfMatchRunningError::InvalidState(_) => None,
	 		SelfMatchRunningError::PlayerError(ref e) => Some(e),
	 		SelfMatchRunningError::PlayerThreadError(_) => None,
	 		SelfMatchRunningError::IOError(ref e) => Some(e),
	 		SelfMatchRunningError::KifuWriteError(ref e) => Some(e),
	 		SelfMatchRunningError::RecvError(ref e) => Some(e),
	 		SelfMatchRunningError::SendError(ref e) => Some(e),
	 		SelfMatchRunningError::ThreadJoinFailed(_) => None,
	 		SelfMatchRunningError::Fail(_) => None,
	 	}
	 }
}
impl<E> From<TypeConvertError<String>> for SelfMatchRunningError<E> where String: fmt::Debug, E: PlayerError {
	fn from(_: TypeConvertError<String>) -> SelfMatchRunningError<E> {
		SelfMatchRunningError::Fail(String::from("An error occurred during type conversion from Move to Moved."))
	}
}
impl<E> From<RecvError> for SelfMatchRunningError<E> where E: PlayerError {
	fn from(err: RecvError) -> SelfMatchRunningError<E> {
		SelfMatchRunningError::RecvError(err)
	}
}
impl<E> From<SendError<SelfMatchMessage>> for SelfMatchRunningError<E> where E: PlayerError {
	fn from(err: SendError<SelfMatchMessage>) -> SelfMatchRunningError<E> {
		SelfMatchRunningError::SendError(err)
	}
}
impl<E> From<io::Error> for SelfMatchRunningError<E> where E: PlayerError {
	fn from(err: io::Error) -> SelfMatchRunningError<E> {
		SelfMatchRunningError::IOError(err)
	}
}
impl<E> From<KifuWriteError> for SelfMatchRunningError<E> where E: PlayerError {
	fn from(err: KifuWriteError) -> SelfMatchRunningError<E> {
		SelfMatchRunningError::KifuWriteError(err)
	}
}
impl<E> From<E> for SelfMatchRunningError<E> where E: PlayerError {
	fn from(err: E) -> SelfMatchRunningError<E> {
		SelfMatchRunningError::PlayerError(err)
	}
}
/// sfen文字列の生成時エラー
#[derive(Debug,Eq,PartialEq)]
pub enum SfenStringConvertError {
	/// 指し手文字列生成時のエラー
	ToMoveString(ToMoveStringConvertError),
	/// 型変換時のエラー
	TypeConvertError(TypeConvertError<String>),
	/// 不正なsfen文字列フォーマット
	InvalidFormat(String),
}
impl fmt::Display for SfenStringConvertError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		SfenStringConvertError::ToMoveString(_) => {
	 			write!(f,"Conversion of move to string representation failed.")
	 		},
	 		SfenStringConvertError::TypeConvertError(_) => {
	 			write!(f,"An error occurred during conversion to sfen string.")
	 		},
	 		SfenStringConvertError::InvalidFormat(ref sfen) => {
	 			write!(f,"The sfen string format is invalid. (passed value: {})", sfen)
	 		}
	 	}
	 }
}
impl error::Error for SfenStringConvertError {
	 fn description(&self) -> &str {
	 	match *self {
	 		SfenStringConvertError::ToMoveString(_) => {
	 			"Conversion of move to string representation failed."
	 		},
	 		SfenStringConvertError::TypeConvertError(_) => {
	 			"An error occurred during conversion to sfen string."
	 		},
	 		SfenStringConvertError::InvalidFormat(_) => {
	 			"The sfen string format is invalid."
	 		}
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		SfenStringConvertError::ToMoveString(ref e) => Some(e),
	 		SfenStringConvertError::TypeConvertError(ref e) => Some(e),
	 		SfenStringConvertError::InvalidFormat(_) => None,
	 	}
	 }
}
impl From<ToMoveStringConvertError> for SfenStringConvertError {
	fn from(err: ToMoveStringConvertError) -> SfenStringConvertError {
		SfenStringConvertError::ToMoveString(err)
	}
}
impl From<TypeConvertError<String>> for SfenStringConvertError {
	fn from(err: TypeConvertError<String>) -> SfenStringConvertError {
		SfenStringConvertError::TypeConvertError(err)
	}
}
/// 棋譜書き込み時のエラー
#[derive(Debug)]
pub enum KifuWriteError {
	/// その他
	Fail(String),
	/// 状態不正
	InvalidState(String),
	/// sfen文字←→内部表現変換時のエラー
	SfenStringConvertError(SfenStringConvertError),
	/// 入出力時のエラー
	IOError(io::Error),
}
impl fmt::Display for KifuWriteError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		KifuWriteError::Fail(ref s) => write!(f,"There was an error writing kifu. ({})",s),
	 		KifuWriteError::InvalidState(ref e) => write!(f,"There was an error writing kifu. (invalid state, {}).", e),
	 		KifuWriteError::SfenStringConvertError(_) => write!(f,"An error occurred during conversion to sfen string."),
		 	KifuWriteError::IOError(ref e) => write!(f,"IO Error. ({})",e),
	 	}
	 }
}
impl error::Error for KifuWriteError {
	 fn description(&self) -> &str {
	 	match *self {
	 		KifuWriteError::Fail(_) => "There was an error writing kifu.",
	 		KifuWriteError::InvalidState(_) => "There was an error writing kifu. (invalid state).",
	 		KifuWriteError::SfenStringConvertError(_) => "An error occurred during conversion to sfen string.",
		 	KifuWriteError::IOError(_) => "There was an error writing kifu. (IO Error).",
	 	}
	 }

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	 	match *self {
	 		KifuWriteError::Fail(_) => None,
	 		KifuWriteError::InvalidState(_) => None,
	 		KifuWriteError::SfenStringConvertError(ref e) => Some(e),
	 		KifuWriteError::IOError(ref e) => Some(e),
	 	}
	 }
}
impl From<SfenStringConvertError> for KifuWriteError {
	fn from(err: SfenStringConvertError) -> KifuWriteError {
		KifuWriteError::SfenStringConvertError(err)
	}
}
impl From<io::Error> for KifuWriteError {
	fn from(err:io::Error) -> KifuWriteError {
		KifuWriteError::IOError(err)
	}
}
/// `USIPlayer`の実装から投げられるエラーであることを示すマーカートレイト
pub trait PlayerError: Error + fmt::Debug + Send + 'static {}
/// サイズ超過のエラー
#[derive(Debug,Eq,PartialEq)]
pub struct LimitSizeError(pub usize);
impl fmt::Display for LimitSizeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			LimitSizeError(s) => write!(f,"Size exceeded the following values {}",s)
		}
	}
}
impl error::Error for LimitSizeError {
	fn description(&self) -> &str {
		"Size exceeds the upper limit."
	}

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		None
	}
}
