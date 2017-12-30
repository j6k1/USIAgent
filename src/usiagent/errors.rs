use std::error;
use std::fmt;
use std::sync::MutexGuard;
use std::sync::PoisonError;
use usiagent::commands::UsiCommand;

#[derive(Debug)]
pub enum EventDispatchError<'a,T> where T: fmt::Debug + 'a {
	ErrorFromHandler(EventHandlerError),
	MutexLockFailedError(PoisonError<MutexGuard<'a,T>>),
}
#[derive(Debug)]
pub enum EventHandlerError {
	Fail(String),
}
impl<'a,T> fmt::Display for EventDispatchError<'a,T> where T: fmt::Debug {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		EventDispatchError::ErrorFromHandler(EventHandlerError::Fail(ref s)) => write!(f, "{}", s),
	 		EventDispatchError::MutexLockFailedError(_) => write!(f, "オブジェクトのロックを確保できませんでした。"),
	 	}
	 }
}
impl<'a,T> error::Error for EventDispatchError<'a,T> where T: fmt::Debug {
	 fn description(&self) -> &str {
	 	match *self {
	 		EventDispatchError::ErrorFromHandler(EventHandlerError::Fail(ref s)) => s,
	 		EventDispatchError::MutexLockFailedError(ref e) => e.description(),
	 	}
	 }

	fn cause(&self) -> Option<&error::Error> {
	 	match *self {
	 		EventDispatchError::ErrorFromHandler(_) => None,
	 		EventDispatchError::MutexLockFailedError(ref e) => Some(e),
	 	}
	 }
}
impl<'a,T> From<PoisonError<MutexGuard<'a,T>>> for EventDispatchError<'a,T> where T: fmt::Debug + 'a {
	fn from(err: PoisonError<MutexGuard<'a,T>>) -> EventDispatchError<'a,T> {
		EventDispatchError::MutexLockFailedError(err)
	}
}
impl<'a,T> From<EventHandlerError> for EventDispatchError<'a,T> where T: fmt::Debug {
	fn from(err: EventHandlerError) -> EventDispatchError<'a,T> {
		EventDispatchError::ErrorFromHandler(err)
	}
}
#[derive(Debug)]
pub struct DanConvertError(pub u32);
impl fmt::Display for DanConvertError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		DanConvertError(n) => write!(f, "段の値{}は有効な値の範囲外です。",n)
	 	}
	 }
}
impl error::Error for DanConvertError {
	 fn description(&self) -> &str {
	 	match *self {
	 		DanConvertError(_) => "段の値が有効な値の範囲外です。"
	 	}
	 }

	fn cause(&self) -> Option<&error::Error> {
	 	match *self {
	 		DanConvertError(_) => None
	 	}
	 }
}
#[derive(Debug)]
pub enum ToMoveStringConvertError {
	CharConvert(DanConvertError),
}
impl fmt::Display for ToMoveStringConvertError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		ToMoveStringConvertError::CharConvert(ref e) => e.fmt(f),
	 	}
	 }
}
impl error::Error for ToMoveStringConvertError {
	 fn description(&self) -> &str {
	 	match *self {
	 		ToMoveStringConvertError::CharConvert(ref e) => e.description(),
	 	}
	 }

	fn cause(&self) -> Option<&error::Error> {
	 	match *self {
	 		ToMoveStringConvertError::CharConvert(ref e) => Some(e)
	 	}
	 }
}
impl From<DanConvertError> for ToMoveStringConvertError {
	fn from(err: DanConvertError) -> ToMoveStringConvertError {
		ToMoveStringConvertError::CharConvert(err)
	}
}
#[derive(Debug)]
pub enum UsiOutputCreateError {
	ValidationError(UsiCommand),
	InvalidStateError(String),
	ConvertError(ToMoveStringConvertError),
}
impl fmt::Display for UsiOutputCreateError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		UsiOutputCreateError::ConvertError(ref e) => e.fmt(f),
	 		UsiOutputCreateError::InvalidStateError(ref s) => write!(f, "{}の状態が不正です。", s),
	 		UsiOutputCreateError::ValidationError(ref cmd) => {
	 			match *cmd {
	 				UsiCommand::UsiBestMove(_) => write!(f, "bestmoveコマンドの生成元オブジェクトの状態が不正です。"),
	 				UsiCommand::UsiInfo(_) => write!(f, "infoコマンドの生成元オブジェクトの状態が不正です。"),
	 				UsiCommand::UsiCheckMate(_) => write!(f, "checkmateコマンドの生成元オブジェクトの状態が不正です。"),
		 			_ => write!(f,"コマンドのバリデーションで想定しない例外が発生しました。"),
	 			}
	 		}
	 	}
	 }
}
impl error::Error for UsiOutputCreateError {
	 fn description(&self) -> &str {
	 	match *self {
	 		UsiOutputCreateError::ConvertError(ref e) => e.description(),
	 		UsiOutputCreateError::InvalidStateError(_) => "コマンドの生成オブジェクトの状態が不正です。",
	 		UsiOutputCreateError::ValidationError(ref cmd) => {
	 			match *cmd {
	 				UsiCommand::UsiBestMove(_) => "bestmoveコマンドの生成元オブジェクトの状態が不正です。",
	 				UsiCommand::UsiInfo(_) => "infoコマンドの生成元オブジェクトの状態が不正です。",
	 				UsiCommand::UsiCheckMate(_) => "checkmateコマンドの生成元オブジェクトの状態が不正です。",
	 				_ => "コマンドのバリデーションで想定しない例外が発生しました。",
	 			}
	 		}
	 	}
	 }

	fn cause(&self) -> Option<&error::Error> {
	 	match *self {
	 		UsiOutputCreateError::ConvertError(ref e) => Some(e),
	 		UsiOutputCreateError::InvalidStateError(_) => None,
	 		UsiOutputCreateError::ValidationError(_) => None,
	 	}
	 }
}
impl From<ToMoveStringConvertError> for UsiOutputCreateError {
	fn from(err: ToMoveStringConvertError) -> UsiOutputCreateError {
		UsiOutputCreateError::ConvertError(err)
	}
}