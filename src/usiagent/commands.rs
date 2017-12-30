use std::collections::HashSet;
use std::fmt;
use std::error;

use usiagent::errors::DanConvertError;
use usiagent::errors::ToMoveStringConvertError;

pub enum UsiCommand {
	UsiOk,
	UsiId(String, String),
	UsiReadyOk,
	UsiBestMove(BestMove),
	UsiInfo(Vec<UsiInfoSubCommand>),
	UsiOption(String,UsiOptType),
	UsiCheckMate,
}
pub enum BestMove {
	Move(Teban,KomaSrcPosition,KomaDstPosition,Option<(KomaSrcPosition,KomaDstPosition)>),
	Resign,
	Win,
}
pub enum UsiInfoSubCommand {
	Depth(u32),
	SelDepth(u32),
	Time(u32),
	Nodes(u32),
	Pv(Teban,Vec<(KomaSrcPosition,KomaDstPosition)>),
	Score(UsiScore,Option<UsiScoreKind>),
	CurMove(Teban,KomaSrcPosition,KomaDstPosition),
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
pub enum UsiScore {
	Cp(i32),
	Mate(UsiScoreMate),
}
pub enum UsiScoreKind {
	Upper,
	Lower,
}
pub enum UsiScoreMate {
	Num(i32),
	Plus,
	Minus,
}
pub enum UsiCheckMate {
	Moves(Vec<(KomaSrcPosition,KomaDstPosition)>),
	NotiImplemented,
	Timeout,
	Nomate,
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum KomaKind {
	Fu = 0,
	Kyou,
	Kei,
	Gin,
	Kin,
	Kaku,
	Hisha,
	Ou,
	FuN,
	KyouN,
	KeiN,
	KakuN,
	HishaN,
	Tail,
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum MochigomaKind {
	Fu = 0,
	Kyou,
	Kei,
	Gin,
	Kin,
	Kaku,
	Hisha,
	Ou,
	Tail,
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum Kyokumen {
	Ban(Teban,KomaKind,u32,u32),
	MochigomaKind(Teban,MochigomaKind),
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum KomaSrcPosition {
	Ban(u32,u32),
	Mochigoma(MochigomaKind),
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum KomaDstPosition {
	Ban(u32,u32),
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum Teban {
	Sente,
	Gote,
}
impl Teban {
	pub fn opposite(&self) -> Teban {
		match *self {
			Teban::Sente => Teban::Gote,
			Teban::Gote => Teban::Sente,
		}
	}
}
pub enum UsiOptType {
	Check(Option<bool>),
	Spin(u32, u32),
	Combo(Option<String>, Option<Vec<String>>),
	Button,
	String(Option<String>),
	FileName(Option<String>),
}
impl UsiCommand {
	pub fn validate(&self) -> bool {
		match *self {
			UsiCommand::UsiBestMove(BestMove::Move(_,ref s,ref d,_)) if !s.validate() || !d.validate() => false,
			UsiCommand::UsiBestMove(BestMove::Move(_,_,_,Some((ref s,ref d)))) if !s.validate() || !d.validate() => false,
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
						ref c @ UsiInfoSubCommand::Pv(_,_) => {
							return c.validate();
						},
						ref c @ UsiInfoSubCommand::CurMove(_,_,_) => {
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
			ref c @ UsiCommand::UsiCheckMate => c.validate(),
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
			UsiInfoSubCommand::Score(_,_) => UsiInfoSubCommandKind::Score,
			UsiInfoSubCommand::CurMove(_,_,_) => UsiInfoSubCommandKind::CurMove,
			UsiInfoSubCommand::Hashfull(_) => UsiInfoSubCommandKind::Hashfull,
			UsiInfoSubCommand::Nps(_) => UsiInfoSubCommandKind::Nps,
			UsiInfoSubCommand::Str(_) => UsiInfoSubCommandKind::Str,
		}
	}

	pub fn validate(&self) -> bool {
		match *self {
			UsiInfoSubCommand::Pv(_,ref v) if v.len() < 1 => false,
			UsiInfoSubCommand::Pv(_,ref v) => {
				for m in v {
					match *m {
						(ref s,ref d) if !s.validate() || !d.validate() => {
							return false;
						},
						_ => (),
					}
				}
				true
			},
			UsiInfoSubCommand::CurMove(_,ref s,ref d) if !s.validate() || !d.validate() => false,
			_ => true,
		}
	}
}
impl UsiCheckMate {
	pub fn validate(&self) -> bool {
		match *self {
			UsiCheckMate::Moves(ref v) if v.len() < 1 => false,
			UsiCheckMate::Moves(ref v) => {
				for m in v {
					match *m {
						(ref s,ref d) if !s.validate() || !d.validate() => {
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
impl KomaSrcPosition {
	pub fn validate(&self) -> bool {
		match *self {
			KomaSrcPosition::Mochigoma(_) => true,
			KomaSrcPosition::Ban(x, y) => x < 9 && y < 9,
		}
	}
}
impl KomaDstPosition {
	pub fn validate(&self) -> bool {
		match *self {
			KomaDstPosition::Ban(x, y) => x < 9 && y < 9,
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
			Teban::Sente if k > MochigomaKind::Ou => {
				format!("+{}", SENTE_KOMA_MAP[(k as usize) - (MochigomaKind::Ou as usize)])
			},
			Teban::Sente => format!("{}",SENTE_KOMA_MAP[k as usize]),
			Teban::Gote if k > MochigomaKind::Ou => {
				format!("+{}", GOTE_KOMA_MAP[(k as usize) - (MochigomaKind::Ou as usize)])
			},
			Teban::Gote => format!("{}",GOTE_KOMA_MAP[k as usize]),
		}
	}
}
trait MoveStringFrom {
	fn str_from(t:Teban,s:KomaSrcPosition,d:KomaDstPosition) -> Result<String, ToMoveStringConvertError>;
}
struct MoveStringCreator {

}
impl MoveStringFrom for MoveStringCreator {
	fn str_from(teban:Teban,ms:KomaSrcPosition,md:KomaDstPosition) -> Result<String, ToMoveStringConvertError> {
		match (teban, ms, md) {
			(t,KomaSrcPosition::Mochigoma(s),KomaDstPosition::Ban(x,y)) => {
				Ok(format!("{}*{}{}", KomaStringCreator::str_from(t,s), x+1, DanCharCreator::char_from(y)?))
			},
			(_,KomaSrcPosition::Ban(sx,sy),KomaDstPosition::Ban(dx,dy)) => {
				Ok(format!("{}{}{}{}", sx+1, DanCharCreator::char_from(sy)?, dx+1, DanCharCreator::char_from(dy)?))
			},
		}
	}
}
pub trait TryToString<E> where E: fmt::Debug + error::Error {
	fn try_to_string(&self) -> Result<String,E>;
}
impl TryToString<ToMoveStringConvertError> for BestMove {
	fn try_to_string(&self) -> Result<String, ToMoveStringConvertError> {
		match *self {
			BestMove::Resign => Ok(String::from("bestmove resign")),
			BestMove::Win => Ok(String::from("bestmove win")),
			BestMove::Move(t,s,d,None) => Ok(MoveStringCreator::str_from(t,s,d)?),
			BestMove::Move(t,s,d,Some((ps,pd))) => {
				Ok(format!("bestmove {} ponder {}",
						MoveStringCreator::str_from(t,s,d)?,
						MoveStringCreator::str_from(t.opposite(),ps,pd)?))

			}
		}
	}
}