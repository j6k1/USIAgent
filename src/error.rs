use std::error;
use std::error::Error;
use std::fmt;
use std::sync::MutexGuard;
use std::sync::PoisonError;
use std::num::ParseIntError;
use command::UsiCommand;

#[derive(Debug)]
pub enum EventDispatchError<'a,T,K,E>
	where T: fmt::Debug + 'a, K: fmt::Debug, E: Error + fmt::Debug {
	ErrorFromHandler(EventHandlerError<K,E>),
	MutexLockFailedError(PoisonError<MutexGuard<'a,T>>),
	ContainError,
}
#[derive(Debug)]
pub enum EventHandlerError<K,E> where K: fmt::Debug, E: Error + fmt::Debug {
	Fail(String),
	InvalidState(K),
	PlayerError(E),
}
impl<K,E> fmt::Display for EventHandlerError<K,E> where K: fmt::Debug, E: Error + fmt::Debug {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		EventHandlerError::Fail(ref s) => write!(f,"{}",s),
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

	fn cause(&self) -> Option<&error::Error> {
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
	 		EventDispatchError::ErrorFromHandler(ref e) => e.fmt(f),
	 		EventDispatchError::MutexLockFailedError(_) => write!(f, "Could not get exclusive lock on object."),
	 		EventDispatchError::ContainError => write!(f, "One or more errors occurred while executing the event handler."),
	 	}
	 }
}
impl<'a,T,K,E> error::Error for EventDispatchError<'a,T,K,E>
	where T: fmt::Debug, K: fmt::Debug, E: Error + fmt::Debug {
	 fn description(&self) -> &str {
	 	match *self {
	 		EventDispatchError::ErrorFromHandler(ref e) => e.description(),
	 		EventDispatchError::MutexLockFailedError(_) => "Could not get exclusive lock on object.",
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

	fn cause(&self) -> Option<&error::Error> {
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
#[derive(Debug)]
pub enum UsiOutputCreateError {
	ValidationError(UsiCommand),
	InvalidStateError(String),
	ConvertError(ToMoveStringConvertError),
	AbortedError,
}
impl fmt::Display for UsiOutputCreateError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		UsiOutputCreateError::ConvertError(_) => write!(f, "Failed to generate command string output to USI protocol."),
	 		UsiOutputCreateError::InvalidStateError(ref s) => write!(f, "The state of {} is invalid.", s),
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

	fn cause(&self) -> Option<&error::Error> {
	 	match *self {
	 		UsiOutputCreateError::ConvertError(ref e) => Some(e),
	 		UsiOutputCreateError::InvalidStateError(_) => None,
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
		EventHandlerError::Fail(err.description().to_string())
	}
}
impl<T,E> From<E> for EventHandlerError<T,E> where T: fmt::Debug, E: PlayerError {
	fn from(err: E) -> EventHandlerError<T,E> {
		EventHandlerError::PlayerError(err)
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
	 		UsiEventSendError::FailCreateOutput(_) => write!(f, "Failed to generate command string to send from AI to USI."),
	 		UsiEventSendError::MutexLockFailedError(_) => write!(f, "Could not get exclusive lock on object."),
	 	}
	 }
}
impl<'a,T> error::Error for UsiEventSendError<'a,T> where T: fmt::Debug + 'a {
	 fn description(&self) -> &str {
	 	match *self {
	 		UsiEventSendError::FailCreateOutput(_) => "Failed to generate command string to send from AI to USI.",
	 		UsiEventSendError::MutexLockFailedError(_) => "Could not get exclusive lock on object.",
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
pub enum InfoSendError {
	Fail(String)
}
impl fmt::Display for InfoSendError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		InfoSendError::Fail(ref e) => e.fmt(f),
	 	}
	 }
}
impl error::Error for InfoSendError {
	 fn description(&self) -> &str {
	 	match *self {
	 		InfoSendError::Fail(_) => "An error occurred when sending the info command.",
	 	}
	 }

	fn cause(&self) -> Option<&error::Error> {
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
#[derive(Debug)]
pub enum USIAgentStartupError<'a,T,E> where T: fmt::Debug + 'a, E: PlayerError {
	MutexLockFailedError(PoisonError<MutexGuard<'a,T>>),
	MutexLockFailedOtherError(String),
	IOError(String),
	PlayerError(E),
}
impl<'a,T,E> fmt::Display for USIAgentStartupError<'a,T,E> where T: fmt::Debug, E: PlayerError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		USIAgentStartupError::MutexLockFailedError(_) => write!(f, "Could not get exclusive lock on object."),
		 	USIAgentStartupError::MutexLockFailedOtherError(ref s) => write!(f, "{}",s),
		 	USIAgentStartupError::IOError(ref s) => write!(f, "{}",s),
		 	USIAgentStartupError::PlayerError(_) => write!(f,"An error occurred in the processing within the player object."),
	 	}
	 }
}
impl<'a,T,E> error::Error for USIAgentStartupError<'a,T,E> where T: fmt::Debug, E: PlayerError {
	 fn description(&self) -> &str {
	 	match *self {
	 		USIAgentStartupError::MutexLockFailedError(_) => "Could not get exclusive lock on object.",
	 		USIAgentStartupError::MutexLockFailedOtherError(_) => "Could not get exclusive lock on object.",
	 		USIAgentStartupError::IOError(_) => "IO Error.",
	 		USIAgentStartupError::PlayerError(_) => "An error occurred in the processing within the player object.",
	 	}
	 }

	fn cause(&self) -> Option<&error::Error> {
	 	match *self {
	 		USIAgentStartupError::MutexLockFailedError(ref e) => Some(e),
	 		USIAgentStartupError::MutexLockFailedOtherError(_) => None,
	 		USIAgentStartupError::IOError(_) => None,
	 		USIAgentStartupError::PlayerError(ref e) => Some(e),
	 	}
	 }
}
impl<'a,T,E> From<PoisonError<MutexGuard<'a,T>>> for USIAgentStartupError<'a,T,E>
	where T: fmt::Debug + 'a, E: PlayerError {
	fn from(err: PoisonError<MutexGuard<'a,T>>) -> USIAgentStartupError<'a,T,E> {
		USIAgentStartupError::MutexLockFailedError(err)
	}
}
#[derive(Debug)]
pub enum ShogiError {
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

	fn cause(&self) -> Option<&error::Error> {
	 	match *self {
	 		ShogiError::InvalidState(_) => None,
	 	}
	 }
}
#[derive(Debug)]
pub enum UsiProtocolError {
	InvalidState(String),
}
impl fmt::Display for UsiProtocolError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
		 	UsiProtocolError::InvalidState(ref s) => write!(f,"{}",s)
	 	}
	 }
}
impl error::Error for UsiProtocolError {
	 fn description(&self) -> &str {
	 	match *self {
	 		UsiProtocolError::InvalidState(_) => "invalid state.(protocol was not processed proper)",
	 	}
	 }

	fn cause(&self) -> Option<&error::Error> {
	 	match *self {
	 		UsiProtocolError::InvalidState(_) => None,
	 	}
	 }
}
#[derive(Debug)]
pub enum SelfMatchRunningError {
	InvalidState(String),
}
impl fmt::Display for SelfMatchRunningError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
		 	SelfMatchRunningError::InvalidState(ref s) => write!(f,"{}",s)
	 	}
	 }
}
impl error::Error for SelfMatchRunningError {
	 fn description(&self) -> &str {
	 	match *self {
	 		SelfMatchRunningError::InvalidState(_) => "invalid state.(protocol was not processed proper)",
	 	}
	 }

	fn cause(&self) -> Option<&error::Error> {
	 	match *self {
	 		SelfMatchRunningError::InvalidState(_) => None,
	 	}
	 }
}
pub trait PlayerError: Error + fmt::Debug {}
