use std::collections::HashSet;
use std::clone::Clone;

use shogi::*;
use TryToString;
use Validate;
use error::DanConvertError;
use error::ToMoveStringConvertError;
use error::UsiOutputCreateError;

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
trait DanCharFromNum {
	fn char_from(n: u32) -> Result<char, DanConvertError>;
}
struct DanCharCreator {

}
impl DanCharFromNum for DanCharCreator {
	fn char_from(n: u32) -> Result<char, DanConvertError> {
		const DAN_MAP:[char; 9] = ['a','b','c','d','e','f','g','h','i'];

		match n {
			n if n >= 9 => Err(DanConvertError(n)),
			n => Ok(DAN_MAP[n as usize]),
		}
	}
}
trait KomaStrFromKind<T> {
	fn str_from(k:T) -> String;
}
const MOCHIGOMA_MAP:[char; 8] = ['P','L','N','S','G','B','R','K'];

struct KomaStringCreator {

}
impl KomaStrFromKind<MochigomaKind> for KomaStringCreator {
	fn str_from(k:MochigomaKind) -> String {
		format!("{}",MOCHIGOMA_MAP[k as usize])
	}
}
trait MoveStringFrom {
	fn str_from(m:&Move) -> Result<String, ToMoveStringConvertError>;
}
struct MoveStringCreator {

}
impl MoveStringFrom for MoveStringCreator {
	fn str_from(m:&Move) -> Result<String, ToMoveStringConvertError> {
		match m {
			&Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,false)) => {
				Ok(format!("{}{}{}{}", 9 - sx, DanCharCreator::char_from(sy)?, 9 - dx, DanCharCreator::char_from(dy)?))
			},
			&Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,true)) => {
				Ok(format!("{}{}{}{}+", 9 - sx, DanCharCreator::char_from(sy)?, 9 - dx, DanCharCreator::char_from(dy)?))
			},
			&Move::Put(k,KomaDstPutPosition(x,y)) => {
				Ok(format!("{}*{}{}", KomaStringCreator::str_from(k), 9 - x, DanCharCreator::char_from(y)?))
			},
		}
	}
}
impl TryToString<ToMoveStringConvertError> for BestMove {
	fn try_to_string(&self) -> Result<String, ToMoveStringConvertError> {
		match *self {
			BestMove::Resign => Ok(String::from("resign")),
			BestMove::Win => Ok(String::from("win")),
			BestMove::Move(ref m,None) => Ok(MoveStringCreator::str_from(m)?),
			BestMove::Move(ref m,Some(ref pm)) => {
				Ok(format!("{} ponder {}",
						MoveStringCreator::str_from(m)?,
						MoveStringCreator::str_from(pm)?))

			},
			BestMove::Abort => {
				Err(ToMoveStringConvertError::AbortedError)
			}
		}
	}
}
impl TryToString<UsiOutputCreateError> for Vec<UsiInfoSubCommand> {
	fn try_to_string(&self) -> Result<String, UsiOutputCreateError> {
		let mut strs:Vec<String> = Vec::with_capacity(self.len());

		for cmd in self {
			strs.push(cmd.try_to_string()?);
		}

		Ok(strs.join(" "))
	}
}
impl TryToString<UsiOutputCreateError> for UsiInfoSubCommand {
	fn try_to_string(&self) -> Result<String, UsiOutputCreateError> {
		Ok(match *self {
			UsiInfoSubCommand::Depth(d) => format!("depth {}", d),
			UsiInfoSubCommand::SelDepth(d) => format!("seldepth {}", d),
			UsiInfoSubCommand::Time(t) => format!("time {}",t),
			UsiInfoSubCommand::Nodes(n) => format!("nodes {}", n),
			UsiInfoSubCommand::Pv(ref v) if v.len() < 1 => {
				return Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))
			},
			UsiInfoSubCommand::Pv(ref v) => {
				let mut mv:Vec<String> = Vec::with_capacity(v.len());
				for m in v {
					match *m {
						ref m if !m.validate() => {
							return Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))
						},
						ref m => {
							mv.push(MoveStringCreator::str_from(m)?);
						}
					}
				}
				mv.join(" ")
			},
			UsiInfoSubCommand::Score(UsiScore::Cp(cp)) => format!("score cp {}", cp),
			UsiInfoSubCommand::Score(UsiScore::CpUpper(cp)) => {
				format!("score cp {} upperbound", cp)
			},
			UsiInfoSubCommand::Score(UsiScore::CpLower(cp)) => {
				format!("score cp {} lowerbound", cp)
			},
			UsiInfoSubCommand::Score(UsiScore::Mate(UsiScoreMate::Num(n))) => format!("score mate {}",n),
			UsiInfoSubCommand::Score(UsiScore::Mate(UsiScoreMate::Plus)) => format!("score mate +"),
			UsiInfoSubCommand::Score(UsiScore::Mate(UsiScoreMate::Minus)) => format!("score mate -"),
			UsiInfoSubCommand::Score(UsiScore::MateUpper(n)) => {
				format!("score mate {} upperbound",n)
			},
			UsiInfoSubCommand::Score(UsiScore::MateLower(n)) => {
				format!("score mate {} lowerbound",n)
			},
			UsiInfoSubCommand::CurMove(ref m) => {
				MoveStringCreator::str_from(m)?
			},
			UsiInfoSubCommand::Hashfull(v) => format!("hashfull {}", v),
			UsiInfoSubCommand::Nps(v) => format!("nps {}",v),
			UsiInfoSubCommand::Str(ref s) => format!("string {}",s),
		})
	}
}
impl TryToString<UsiOutputCreateError> for UsiOptType {
	fn try_to_string(&self) -> Result<String, UsiOutputCreateError> {
		Ok(match *self {
			UsiOptType::Check(Some(b)) if b => format!("check default true"),
			UsiOptType::Check(Some(_)) => format!("check default false"),
			UsiOptType::Check(None) => format!("check"),
			UsiOptType::Spin(min, max,Some(d)) => format!("spin min {} max {} default {}",min,max,d),
			UsiOptType::Spin(min, max,None) => format!("spin min {} max {}", min, max),
			UsiOptType::Combo(Some(_), ref v) if v.len() < 1 => {
				return Err(UsiOutputCreateError::InvalidStateError(String::from("There is no selection item of combo")))
			},
			UsiOptType::Combo(Some(ref d), ref v) => {
				format!("combo default {} {}", d,
					v.iter().map(|va| format!("var {}", va)).collect::<Vec<String>>().join(" "))
			},
			UsiOptType::Combo(None, ref v) => {
				format!("combo {}", v.iter()
									.map(|va| format!("var {}", va)).collect::<Vec<String>>().join(" "))
			},
			UsiOptType::Button => format!("button"),
			UsiOptType::String(Some(ref s)) if s.is_empty() => format!("string default <empty>"),
			UsiOptType::String(Some(ref s)) => format!("string default {}", s),
			UsiOptType::String(None) => format!("string"),
			UsiOptType::FileName(Some(ref s)) if s.is_empty() => format!("filename <empty>"),
			UsiOptType::FileName(Some(ref s)) => format!("filename {}", s),
			UsiOptType::FileName(None) => format!("filename"),
		})
	}
}
impl TryToString<UsiOutputCreateError> for CheckMate {
	fn try_to_string(&self) -> Result<String, UsiOutputCreateError> {
		Ok(match *self {
			CheckMate::Moves(ref v) if v.len() < 1 => {
				return Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))
			},
			CheckMate::Moves(ref v) => {
				let mut mv:Vec<String> = Vec::with_capacity(v.len());
				for m in v {
					match *m {
						ref m if !m.validate() => {
							return Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))
						},
						ref m => {
							mv.push(MoveStringCreator::str_from(m)?);
						}
					}
				}
				mv.join(" ")
			},
			CheckMate::NotiImplemented => format!("notimplemented"),
			CheckMate::Timeout => format!("timeout"),
			CheckMate::Nomate => format!("nomate"),
			CheckMate::Abort => {
				return Err(UsiOutputCreateError::AbortedError);
			}
		})
	}
}