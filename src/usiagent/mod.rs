pub mod event;
pub mod error;
pub mod command;
pub mod logger;
pub mod string;
pub mod output;
pub mod input;
pub mod player;
pub mod shogi;
pub mod interpreter;
use std::error::Error;
use std::fmt;

use usiagent::command::UsiCommand;
use usiagent::error::*;

pub trait TryFrom<T,E> where Self: Sized {
	fn try_from(s:T) -> Result<Self, TypeConvertError<E>> where E: fmt::Debug;
}
pub trait TryToString<E> where E: fmt::Debug + Error {
	fn try_to_string(&self) -> Result<String,E>;
}
pub trait Validate {
	fn validate(&self) -> bool;
}
#[derive(Debug)]
pub enum UsiOutput {
	Command(Vec<String>),
}
impl UsiOutput {
	fn try_from(cmd: UsiCommand) -> Result<UsiOutput, UsiOutputCreateError> {
		Ok(UsiOutput::Command(match cmd {
			UsiCommand::UsiOk => vec![String::from("usiok")],
			UsiCommand::UsiId(name, author) => {
				vec![format!("id name {}", name), format!("id author {}", author)]
			},
			UsiCommand::UsiReadyOk => vec![String::from("readyok")],
			UsiCommand::UsiBestMove(m) => vec![format!("bestmove {}", m.try_to_string()?)],
			UsiCommand::UsiInfo(i) => vec![format!("info {}", i.try_to_string()?)],
			UsiCommand::UsiOption(s,opt) => vec![format!("option name {} type {}",s,opt.try_to_string()?)],
			UsiCommand::UsiCheckMate(ref c) => vec![format!("checkmate {}", c.try_to_string()?)],
		}))
	}
}
pub trait Logger {
	fn logging(&mut self, msg:&String);
	fn logging_error<E: Error>(&mut self, e:&E);
}
pub struct UsiAgent {

}