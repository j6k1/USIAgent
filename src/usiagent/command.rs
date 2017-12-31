use std::collections::HashSet;

use usiagent::shogi::*;
use usiagent::TryToString;
use usiagent::Validate;
use usiagent::error::DanConvertError;
use usiagent::error::ToMoveStringConvertError;
use usiagent::error::UsiOutputCreateError;

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
#[derive(Debug)]
pub enum BestMove {
	Move(Teban,Move,Option<Move>),
	Resign,
	Win,
}
#[derive(Debug)]
pub enum UsiInfoSubCommand {
	Depth(u32),
	SelDepth(u32),
	Time(u32),
	Nodes(u32),
	Pv(Teban,Vec<Move>),
	Score(UsiScore),
	CurMove(Teban,Move),
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
	Moves(Teban,Vec<Move>),
	NotiImplemented,
	Timeout,
	Nomate,
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
impl Validate for UsiCommand {
	fn validate(&self) -> bool {
		match *self {
			UsiCommand::UsiBestMove(BestMove::Move(_,ref m,_)) if !m.validate() => false,
			UsiCommand::UsiBestMove(BestMove::Move(_,_,Some(ref m))) if !m.validate() => false,
			UsiCommand::UsiInfo(ref commands) => {
				let mut hs = HashSet::new();

				for cmd in commands {
					match *cmd {
						UsiInfoSubCommand::Pv(_,_) if hs.contains(&UsiInfoSubCommandKind::Str) => {
							return false;
						},
						UsiInfoSubCommand::Str(_) if hs.contains(&UsiInfoSubCommandKind::Pv) => {
							return false;
						},
						UsiInfoSubCommand::SelDepth(_) if !hs.contains(&UsiInfoSubCommandKind::Depth) => {
							return false;
						},
						ref c @ UsiInfoSubCommand::Pv(_,_) => {
							return c.validate();
						},
						ref c @ UsiInfoSubCommand::CurMove(_,_) => {
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
			UsiInfoSubCommand::Pv(_,_) => UsiInfoSubCommandKind::Pv,
			UsiInfoSubCommand::Score(_) => UsiInfoSubCommandKind::Score,
			UsiInfoSubCommand::CurMove(_,_) => UsiInfoSubCommandKind::CurMove,
			UsiInfoSubCommand::Hashfull(_) => UsiInfoSubCommandKind::Hashfull,
			UsiInfoSubCommand::Nps(_) => UsiInfoSubCommandKind::Nps,
			UsiInfoSubCommand::Str(_) => UsiInfoSubCommandKind::Str,
		}
	}
}
impl Validate for UsiInfoSubCommand {
	fn validate(&self) -> bool {
		match *self {
			UsiInfoSubCommand::Pv(_,ref v) if v.len() < 1 => false,
			UsiInfoSubCommand::Pv(_,ref v) => {
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
			UsiInfoSubCommand::CurMove(_,ref m) if !m.validate() => false,
			_ => true,
		}
	}
}
impl Validate for CheckMate {
	fn validate(&self) -> bool {
		match *self {
			CheckMate::Moves(_, ref v) if v.len() < 1 => false,
			CheckMate::Moves(_, ref v) => {
				for m in v {
					match *m {
						Move(ref s,ref d) if !s.validate() || !d.validate() => {
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
	fn str_from(t:Teban,k:T) -> String;
}
const SENTE_KOMA_MAP:[char; 8] = ['P','L','N','S','G','B','R','K'];
const GOTE_KOMA_MAP:[char; 8] = ['p','l','n','s','g','b','r','k'];
struct KomaStringCreator {

}
impl KomaStrFromKind<MochigomaKind> for KomaStringCreator {
	fn str_from(t:Teban,k:MochigomaKind) -> String {
		match t {
			Teban::Sente => format!("{}",SENTE_KOMA_MAP[k as usize]),
			Teban::Gote => format!("{}",GOTE_KOMA_MAP[k as usize]),
		}
	}
}
trait MoveStringFrom {
	fn str_from(t:Teban,m:&Move) -> Result<String, ToMoveStringConvertError>;
}
struct MoveStringCreator {

}
impl MoveStringFrom for MoveStringCreator {
	fn str_from(teban:Teban,m:&Move) -> Result<String, ToMoveStringConvertError> {
		match (teban, m) {
			(t,&Move(KomaSrcPosition::Mochigoma(s),KomaDstPosition::Ban(x,y))) => {
				Ok(format!("{}*{}{}", KomaStringCreator::str_from(t,s), x+1, DanCharCreator::char_from(y)?))
			},
			(_,&Move(KomaSrcPosition::Ban(sx,sy),KomaDstPosition::Ban(dx,dy))) => {
				Ok(format!("{}{}{}{}", sx+1, DanCharCreator::char_from(sy)?, dx+1, DanCharCreator::char_from(dy)?))
			},
		}
	}
}
impl TryToString<ToMoveStringConvertError> for BestMove {
	fn try_to_string(&self) -> Result<String, ToMoveStringConvertError> {
		match *self {
			BestMove::Resign => Ok(String::from("resign")),
			BestMove::Win => Ok(String::from("win")),
			BestMove::Move(t,ref m,None) => Ok(MoveStringCreator::str_from(t,m)?),
			BestMove::Move(t,ref m,Some(ref pm)) => {
				Ok(format!("{} ponder {}",
						MoveStringCreator::str_from(t,m)?,
						MoveStringCreator::str_from(t.opposite(),pm)?))

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
			UsiInfoSubCommand::Pv(_,ref v) if v.len() < 1 => {
				return Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))
			},
			UsiInfoSubCommand::Pv(t,ref v) => {
				let mut t:Teban = t;
				let mut mv:Vec<String> = Vec::with_capacity(v.len());
				for m in v {
					match *m {
						ref m if !m.validate() => {
							return Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))
						},
						ref m => {
							mv.push(MoveStringCreator::str_from(t,m)?);
							t = t.opposite();
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
			UsiInfoSubCommand::CurMove(t,ref m) => {
				MoveStringCreator::str_from(t,m)?
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
			CheckMate::Moves(_, ref v) if v.len() < 1 => {
				return Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))
			},
			CheckMate::Moves(t, ref v) => {
				let mut t:Teban = t;
				let mut mv:Vec<String> = Vec::with_capacity(v.len());
				for m in v {
					match *m {
						ref m if !m.validate() => {
							return Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))
						},
						ref m => {
							mv.push(MoveStringCreator::str_from(t,m)?);
							t = t.opposite();
						}
					}
				}
				mv.join(" ")
			},
			CheckMate::NotiImplemented => format!("notimplemented"),
			CheckMate::Timeout => format!("timeout"),
			CheckMate::Nomate => format!("nomate"),
		})
	}
}