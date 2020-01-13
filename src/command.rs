//! USIプロトコル準拠のcommandを取り扱う
use std::collections::HashSet;
use std::clone::Clone;

use shogi::*;
use Validate;
/// USIプロトコル準拠のコマンド
#[derive(Debug,Eq,PartialEq)]
pub enum UsiCommand {
	/// usiok
	UsiOk,
	/// id name {name}, id author {author}
	UsiId(String, String),
	/// readyok
	UsiReadyOk,
	/// bestmove
	UsiBestMove(BestMove),
	/// info
	UsiInfo(Vec<UsiInfoSubCommand>),
	/// option
	UsiOption(String,UsiOptType),
	/// checkmate
	UsiCheckMate(CheckMate),
}
/// 指し手
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum BestMove {
	/// 通常の指し手（ponderをOptionで指定可能）
	Move(Move,Option<Move>),
	/// 投了
	Resign,
	/// 入玉勝ち宣言
	Win,
	/// 中断（USIプロトコルの仕様にはない。返してもGUI側にコマンドは送信されない）
	Abort,
}
/// infoコマンドのサブコマンド
#[derive(Clone, Debug,Eq,PartialEq)]
pub enum UsiInfoSubCommand {
	/// depth
	Depth(u32),
	/// seldepth
	SelDepth(u32),
	/// time
	Time(u64),
	/// nodes
	Nodes(u64),
	/// pv
	Pv(Vec<Move>),
	/// multipv
	MultiPv(u32),
	/// score
	Score(UsiScore),
	/// curmove
	CurMove(Move),
	/// hashfull
	Hashfull(u64),
	/// nps
	Nps(u64),
	/// string
	Str(String),
}
/// infoサブコマンドの種別
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum UsiInfoSubCommandKind {
	/// depth
	Depth,
	/// seldepth
	SelDepth,
	/// time
	Time,
	/// nodes
	Nodes,
	/// pv
	Pv,
	/// multipv
	MultiPv,
	/// score
	Score,
	/// curmove
	CurMove,
	/// hashfull
	Hashfull,
	/// nps
	Nps,
	/// string
	Str,
}
/// infoコマンドのscore
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum UsiScore {
	/// score cp <x>
	Cp(i64),
	/// score cp upper
	CpUpper(i64),
	/// score cp lower
	CpLower(i64),
	/// score mate <y>
	Mate(UsiScoreMate),
	/// score mate upper
	MateUpper(i64),
	/// score mate lower
	MateLower(i64),
}
/// infoコマンドのscoreサブコマンドのmateの値
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum UsiScoreMate {
	/// 数値
	Num(i64),
	/// \+
	Plus,
	/// \-
	Minus,
}
/// 詰め将棋の解答
#[derive(Debug,Eq,PartialEq)]
pub enum CheckMate {
	/// 詰みまでの指し手
	Moves(Vec<Move>),
	/// 未実装であることをGUI側に伝える
	NotiImplemented,
	/// 時間内に詰みを見つけられなかった
	Timeout,
	/// 詰まない
	Nomate,
	/// 中断（USIプロトコルの仕様にはない。返してもGUI側にコマンドは送信されない）
	Abort,
}
/// optionコマンドの値
#[derive(Debug,Eq,PartialEq)]
pub enum UsiOptType {
	/// check
	///
	/// デフォルト値としてtrueかfalseを指定可能
	Check(Option<bool>),
	/// spin
	///
	/// min,max,デフォルト値（オプション）を指定
	Spin(i64, i64,Option<i64>),
	/// combo
	///
	/// デフォルト値、項目のVecを指定。項目は一つ以上なければならない。
	Combo(Option<String>, Vec<String>),
	/// button
	Button,
	/// string
	///
	/// デフォルト値を指定可能
	String(Option<String>),
	/// filename
	///
	/// デフォルト値を指定可能
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
				let mut prev_kind = None;

				for cmd in commands {
					match *cmd {
						UsiInfoSubCommand::Pv(_) if hs.contains(&UsiInfoSubCommandKind::Str) => {
							return false;
						},
						UsiInfoSubCommand::Str(_) if hs.contains(&UsiInfoSubCommandKind::Pv) => {
							return false;
						},
						UsiInfoSubCommand::SelDepth(_) if !prev_kind.map(|k| k == UsiInfoSubCommandKind::Depth).unwrap_or(false) => {
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
						let kind = cmd.get_kind();
						hs.insert(kind);
						prev_kind = Some(kind);
					}
				}

				if hs.contains(&UsiInfoSubCommandKind::MultiPv) && !hs.contains(&UsiInfoSubCommandKind::Pv) {
					false
				} else {
					true
				}
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
			UsiInfoSubCommand::MultiPv(_) => UsiInfoSubCommandKind::MultiPv,
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