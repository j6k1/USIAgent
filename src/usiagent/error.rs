use std::error;
use std::fmt;
use std::sync::MutexGuard;
use std::sync::PoisonError;
use std::num::ParseIntError;
use usiagent::command::UsiCommand;

#[derive(Debug)]
pub enum EventDispatchError<'a,T> where T: fmt::Debug + 'a {
	ErrorFromHandler(EventHandlerError),
	MutexLockFailedError(PoisonError<MutexGuard<'a,T>>),
	ContainError,
}
#[derive(Debug)]
pub enum EventHandlerError {
	Fail(String),
}
impl fmt::Display for EventHandlerError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		EventHandlerError::Fail(ref s) => write!(f,"{}",s)
	 	}
	 }
}
impl error::Error for EventHandlerError {
	 fn description(&self) -> &str {
	 	match *self {
	 		EventHandlerError::Fail(_) => "An error occurred while executing the event handler."
	 	}
	 }

	fn cause(&self) -> Option<&error::Error> {
	 	match *self {
	 		EventHandlerError::Fail(_) => None
	 	}
	 }
}
impl<'a,T> fmt::Display for EventDispatchError<'a,T> where T: fmt::Debug {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		EventDispatchError::ErrorFromHandler(EventHandlerError::Fail(ref s)) => write!(f, "{}", s),
	 		EventDispatchError::MutexLockFailedError(_) => write!(f, "Could not get exclusive lock on object."),
	 		EventDispatchError::ContainError => write!(f, "One or more errors occurred while executing the event handler."),
	 	}
	 }
}
impl<'a,T> error::Error for EventDispatchError<'a,T> where T: fmt::Debug {
	 fn description(&self) -> &str {
	 	match *self {
	 		EventDispatchError::ErrorFromHandler(EventHandlerError::Fail(ref s)) => s,
	 		EventDispatchError::MutexLockFailedError(ref e) => e.description(),
	 		EventDispatchError::ContainError => "Error executing event handler",
	 	}
	 }

	fn cause(&self) -> Option<&error::Error> {
	 	match *self {
	 		EventDispatchError::ErrorFromHandler(_) => None,
	 		EventDispatchError::MutexLockFailedError(ref e) => Some(e),
	 		EventDispatchError::ContainError => None,
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
	 		UsiOutputCreateError::InvalidStateError(ref s) => write!(f, "The state of {} is invalid.", s),
	 		UsiOutputCreateError::ValidationError(ref cmd) => {
	 			match *cmd {
	 				UsiCommand::UsiBestMove(_) => write!(f, "The state of the object that generated the bestmove command is invalid."),
	 				UsiCommand::UsiInfo(_) => write!(f, "The state of the object that generated the info command is invalid."),
	 				UsiCommand::UsiCheckMate(_) => write!(f, "The state of the object that generated the checkmate command is invalid."),
		 			_ => write!(f,"An unexpected exception occurred in command validation."),
	 			}
	 		}
	 	}
	 }
}
impl error::Error for UsiOutputCreateError {
	 fn description(&self) -> &str {
	 	match *self {
	 		UsiOutputCreateError::ConvertError(ref e) => e.description(),
	 		UsiOutputCreateError::InvalidStateError(_) => "The state of the command generation object is invalid",
	 		UsiOutputCreateError::ValidationError(ref cmd) => {
	 			match *cmd {
	 				UsiCommand::UsiBestMove(_) => "The state of the object that generated the bestmove command is invalid",
	 				UsiCommand::UsiInfo(_) => "The state of the object that generated the info command is invalid",
	 				UsiCommand::UsiCheckMate(_) => "The state of the object that generated the checkmate command is invalid",
	 				_ => "An unexpected exception occurred in command validation",
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
#[derive(Debug)]
pub enum UsiEventSendError<'a,T> where T: fmt::Debug + 'a {
	FailCreateOutput(UsiOutputCreateError),
	MutexLockFailedError(PoisonError<MutexGuard<'a,T>>),
}
impl<'a,T> fmt::Display for UsiEventSendError<'a,T> where T: fmt::Debug + 'a {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		UsiEventSendError::FailCreateOutput(ref e) => e.fmt(f),
	 		UsiEventSendError::MutexLockFailedError(_) => write!(f, "Could not get exclusive lock on object."),
	 	}
	 }
}
impl<'a,T> error::Error for UsiEventSendError<'a,T> where T: fmt::Debug + 'a {
	 fn description(&self) -> &str {
	 	match *self {
	 		UsiEventSendError::FailCreateOutput(ref e) => e.description(),
	 		UsiEventSendError::MutexLockFailedError(ref e) => e.description(),
	 	}
	 }

	fn cause(&self) -> Option<&error::Error> {
	 	match *self {
	 		UsiEventSendError::FailCreateOutput(ref e) => Some(e),
	 		UsiEventSendError::MutexLockFailedError(ref e) => Some(e),
	 	}
	 }
}
impl<'a,T> From<UsiOutputCreateError> for UsiEventSendError<'a,T> where T: fmt::Debug + 'a {
	fn from(err: UsiOutputCreateError) -> UsiEventSendError<'a,T> {
		UsiEventSendError::FailCreateOutput(err)
	}
}
impl<'a,T> From<PoisonError<MutexGuard<'a,T>>> for UsiEventSendError<'a,T> where T: fmt::Debug + 'a {
	fn from(err: PoisonError<MutexGuard<'a,T>>) -> UsiEventSendError<'a,T> {
		UsiEventSendError::MutexLockFailedError(err)
	}
}
#[derive(Debug)]
pub enum TypeConvertError<T> where T: fmt::Debug {
	SyntaxError(T),
	LogicError(T),
}
impl<T> fmt::Display for TypeConvertError<T> where T: fmt::Debug {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		TypeConvertError::SyntaxError(ref e) => write!(f, "An error occurred in type conversion. from = ({:?})",e),
		 	TypeConvertError::LogicError(ref e) => e.fmt(f),
	 	}
	 }
}
impl<T> error::Error for TypeConvertError<T> where T: fmt::Debug {
	 fn description(&self) -> &str {
	 	match *self {
	 		TypeConvertError::SyntaxError(_) => "An error occurred in type conversion",
	 		TypeConvertError::LogicError(_) => "An error occurred in type conversion (logic error)",
	 	}
	 }

	fn cause(&self) -> Option<&error::Error> {
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
