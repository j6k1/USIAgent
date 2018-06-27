use std::collections::HashSet;
use std::clone::Clone;

use shogi::*;
use Validate;

#[derive(Debug)]
pub enum UsiCommand {
	UsiOk,
	UsiId(String, String),
	UsiReadyOk,
	UsiBestMove(BestMove),
	UsiInfo(Vec<UsiInfoSubCommand>),
	UsiOption(String,UsiOptType),
	UsiCheckMate(CheckMate),
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum BestMove {
	Move(Move,Option<Move>),
	Resign,
	Win,
	Abort,
}
#[derive(Debug)]
pub enum UsiInfoSubCommand {
	Depth(u32),
	SelDepth(u32),
	Time(u32),
	Nodes(u32),
	Pv(Vec<Move>),
	Score(UsiScore),
	CurMove(Move),
	Hashfull(u32),
	Nps(u32),
	Str(String),
}
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum UsiInfoSubCommandKind {
	Depth,
	SelDepth,
	Time,
	Nodes,
	Pv,
	Score,
	CurMove,
	Hashfull,
	Nps,
	Str,
}
#[derive(Debug)]
pub enum UsiScore {
	Cp(i32),
	CpUpper(i32),
	CpLower(i32),
	Mate(UsiScoreMate),
	MateUpper(i32),
	MateLower(i32),
}
#[derive(Debug)]
pub enum UsiScoreMate {
	Num(i32),
	Plus,
	Minus,
}
#[derive(Debug)]
pub enum CheckMate {
	Moves(Vec<Move>),
	NotiImplemented,
	Timeout,
	Nomate,
	Abort,
}
#[derive(Debug)]
pub enum UsiOptType {
	Check(Option<bool>),
	Spin(u32, u32,Option<u32>),
	Combo(Option<String>, Vec<String>),
	Button,
	String(Option<String>),
	FileName(Option<String>),
}
impl Clone for UsiOptType {
	fn clone(&self) -> UsiOptType {
		match *self {
			UsiOptType::Check(None) => UsiOptType::Check(None),
			UsiOptType::Check(Some(b)) => UsiOptType::Check(Some(b)),
			UsiOptType::Spin(l,u,None) => UsiOptType::Spin(l,u,None),
			UsiOptType::Spin(l,u,Some(d)) => UsiOptType::Spin(l,u,Some(d)),
			UsiOptType::Combo(None, ref i) => UsiOptType::Combo(None, i.iter().map(|s| s.clone())
																.collect::<Vec<String>>()),
			UsiOptType::Combo(Some(ref d), ref i) => UsiOptType::Combo(Some(d.clone()), i.iter().map(|s| s.clone())
																.collect::<Vec<String>>()),
			UsiOptType::Button => UsiOptType::Button,
			UsiOptType::String(None) => UsiOptType::String(None),
			UsiOptType::String(Some(ref s)) => UsiOptType::String(Some(s.clone())),
			UsiOptType::FileName(None) => UsiOptType::FileName(None),
			UsiOptType::FileName(Some(ref s)) => UsiOptType::FileName(Some(s.clone())),
		}
	}
}
impl Validate for UsiCommand {
	fn validate(&self) -> bool {
		match *self {
			UsiCommand::UsiBestMove(BestMove::Move(ref m,_)) if !m.validate() => false,
			UsiCommand::UsiBestMove(BestMove::Move(_,Some(ref m))) if !m.validate() => false,
			UsiCommand::UsiInfo(ref commands) => {
				let mut hs = HashSet::new();

				for cmd in commands {
					match *cmd {
						UsiInfoSubCommand::Pv(_) if hs.contains(&UsiInfoSubCommandKind::Str) => {
							return false;
						},
						UsiInfoSubCommand::Str(_) if hs.contains(&UsiInfoSubCommandKind::Pv) => {
							return false;
						},
						UsiInfoSubCommand::SelDepth(_) if !hs.contains(&UsiInfoSubCommandKind::Depth) => {
							return false;
						},
						ref c @ UsiInfoSubCommand::Pv(_) => {
							return c.validate();
						},
						ref c @ UsiInfoSubCommand::CurMove(_) => {
							c.validate();
						}
						_ => (),
					}
					if hs.contains(&cmd.get_kind()) {
						return false;
					}
					else {
						hs.insert(cmd.get_kind());
					}
				}
				true
			},
			UsiCommand::UsiOption(_,ref opt) => opt.validate(),
			UsiCommand::UsiCheckMate(ref c) => c.validate(),
			_ => true
		}
	}
}
impl UsiInfoSubCommand {
	pub fn get_kind(&self) -> UsiInfoSubCommandKind {
		match *self {
			UsiInfoSubCommand::Depth(_) => UsiInfoSubCommandKind::Depth,
			UsiInfoSubCommand::SelDepth(_) => UsiInfoSubCommandKind::SelDepth,
			UsiInfoSubCommand::Time(_) => UsiInfoSubCommandKind::Time,
			UsiInfoSubCommand::Nodes(_) => UsiInfoSubCommandKind::Nodes,
			UsiInfoSubCommand::Pv(_) => UsiInfoSubCommandKind::Pv,
			UsiInfoSubCommand::Score(_) => UsiInfoSubCommandKind::Score,
			UsiInfoSubCommand::CurMove(_) => UsiInfoSubCommandKind::CurMove,
			UsiInfoSubCommand::Hashfull(_) => UsiInfoSubCommandKind::Hashfull,
			UsiInfoSubCommand::Nps(_) => UsiInfoSubCommandKind::Nps,
			UsiInfoSubCommand::Str(_) => UsiInfoSubCommandKind::Str,
		}
	}
}
impl Validate for UsiInfoSubCommand {
	fn validate(&self) -> bool {
		match *self {
			UsiInfoSubCommand::Pv(ref v) if v.len() < 1 => false,
			UsiInfoSubCommand::Pv(ref v) => {
				for m in v {
					match *m {
						ref mv if !mv.validate() => {
							return false;
						},
						_ => (),
					}
				}
				true
			},
			UsiInfoSubCommand::CurMove(ref m) if !m.validate() => false,
			_ => true,
		}
	}
}
impl Validate for CheckMate {
	fn validate(&self) -> bool {
		match *self {
			CheckMate::Moves(ref v) if v.len() < 1 => false,
			CheckMate::Moves(ref v) => {
				for m in v {
					match m.validate() {
						false => {
							return false;
						},
						_ => (),
					}
				}
				true
			},
			_ => true,
		}
	}
}
impl Validate for UsiOptType {
	fn validate(&self) -> bool {
		match *self {
			UsiOptType::Combo(_,ref l) if l.len() < 1 => false,
			_ => true,
		}
	}
}