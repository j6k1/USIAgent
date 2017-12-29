use std::error;
use std::fmt;
use std::sync::MutexGuard;
use std::sync::PoisonError;

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
