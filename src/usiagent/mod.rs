pub mod events;
pub mod errors;
pub mod commands;
use std::fmt;
use std::error;

use usiagent::commands::TryToString;
use usiagent::commands::UsiCommand;
use usiagent::errors::ToMoveStringConvertError;

pub enum UsiOutput {
	Command(Vec<String>),
}
impl UsiOutput {
	fn try_from(cmd: UsiCommand) -> Result<UsiOutput, ToMoveStringConvertError> {
		Ok(UsiOutput::Command(match cmd {
			UsiCommand::UsiOk => vec![String::from("usiok")],
			UsiCommand::UsiId(name, author) => {
				vec![format!("id name {}", name), format!("id author {}", author)]
			},
			UsiCommand::UsiReadyOk => vec![String::from("readyok")],
			UsiCommand::UsiBestMove(m) => vec![m.try_to_string()?],
			_ => panic!(),
//			UsiCommand::UsiInfo(Vec<UsiInfoSubCommand>),
//			UsiCommand::UsiOption(String,UsiOptType),
//			UsiCommand::UsiCheckMate,
		}))
	}
}
#[derive(Debug)]
enum UsiOutputCreateError {
	ConvertError(ToMoveStringConvertError),
}
impl fmt::Display for UsiOutputCreateError {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		UsiOutputCreateError::ConvertError(ref e) => e.fmt(f),
	 	}
	 }
}
impl error::Error for UsiOutputCreateError {
	 fn description(&self) -> &str {
	 	match *self {
	 		UsiOutputCreateError::ConvertError(ref e) => e.description(),
	 	}
	 }

	fn cause(&self) -> Option<&error::Error> {
	 	match *self {
	 		UsiOutputCreateError::ConvertError(ref e) => Some(e)
	 	}
	 }
}
impl From<ToMoveStringConvertError> for UsiOutputCreateError {
	fn from(err: ToMoveStringConvertError) -> UsiOutputCreateError {
		UsiOutputCreateError::ConvertError(err)
	}
}
pub struct UsiAgent {

}