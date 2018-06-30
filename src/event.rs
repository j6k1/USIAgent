use std::fmt;
use std::marker::PhantomData;
use std::sync::Mutex;
use std::sync::Arc;
use std::error::Error;
use std::collections::HashMap;
use std::time::{Instant,Duration};

use TryFrom;
use MaxIndex;
use error::EventDispatchError;
use error::EventHandlerError;
use error::TypeConvertError;
use error::PlayerError;
use UsiOutput;
use Logger;
use OnErrorHandler;
use shogi::*;
pub trait MapEventKind<K> {
	fn event_kind(&self) -> K;
}
#[derive(Debug)]
pub enum SystemEvent {
	Usi,
	IsReady,
	SetOption(String,SysEventOption),
	UsiNewGame,
	Position(Teban,UsiInitialPosition,u32,Vec<Move>),
	Go(UsiGo),
	Stop,
	PonderHit,
	Quit,
	GameOver(GameEndState),
	SendUsiCommand(UsiOutput),
	QuitReady,
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum SystemEventKind {
	Usi = 0,
	IsReady,
	SetOption,
	UsiNewGame,
	Position,
	Go,
	Stop,
	PonderHit,
	Quit,
	GameOver,
	SendUsiCommand,
	QuitReady,
}
impl From<SystemEventKind> for usize {
	fn from(kind: SystemEventKind) -> usize {
		kind as usize
	}
}
impl MaxIndex for SystemEventKind {
	fn max_index() -> usize {
		SystemEventKind::QuitReady as usize
	}
}
impl MapEventKind<SystemEventKind> for SystemEvent {
	fn event_kind(&self) -> SystemEventKind {
		match *self {
			SystemEvent::Usi => SystemEventKind::Usi,
			SystemEvent::IsReady => SystemEventKind::IsReady,
			SystemEvent::SetOption(_,_) => SystemEventKind::SetOption,
			SystemEvent::UsiNewGame => SystemEventKind::UsiNewGame,
			SystemEvent::Position(_,_,_,_) => SystemEventKind::Position,
			SystemEvent::Go(_) => SystemEventKind::Go,
			SystemEvent::Stop => SystemEventKind::Stop,
			SystemEvent::PonderHit => SystemEventKind::PonderHit,
			SystemEvent::Quit => SystemEventKind::Quit,
			SystemEvent::GameOver(_) => SystemEventKind::GameOver,
			SystemEvent::SendUsiCommand(_) => SystemEventKind::SendUsiCommand,
			SystemEvent::QuitReady => SystemEventKind::QuitReady,
		}
	}
}
#[derive(Debug)]
pub enum UserEvent {
	Stop,
	Quit,
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum UserEventKind {
	Stop = 0,
	Quit,
}
impl MapEventKind<UserEventKind> for UserEvent {
	fn event_kind(&self) -> UserEventKind {
		match *self {
			UserEvent::Stop => UserEventKind::Stop,
			UserEvent::Quit => UserEventKind::Quit,
		}
	}
}
impl From<UserEventKind> for usize {
	fn from(kind: UserEventKind) -> usize {
		kind as usize
	}
}
impl MaxIndex for UserEventKind {
	fn max_index() -> usize {
		UserEventKind::Quit as usize
	}
}
#[derive(Debug)]
pub enum SelfMatchEvent {
	GameStart(u32,Teban,String),
	Moved(Teban,Moved),
	GameEnd(SelfMatchGameEndState),
	Abort,
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum SelfMatchGameEndState {
	Win(Teban),
	Resign(Teban),
	NyuGyokuWin(Teban),
	NyuGyokuLose(Teban),
	Draw,
	Foul(Teban,FoulKind),
	Timeover(Teban),
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum Moved {
	To(MovedKind,(u32,u32),(u32,u32),bool),
	Put(MochigomaKind,(u32,u32)),
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum MovedKind {
	Fu = 0,
	Kyou,
	Kei,
	Gin,
	Kin,
	Kaku,
	Hisha,
	SOu,
	GOu,
	FuN,
	KyouN,
	KeiN,
	GinN,
	KakuN,
	HishaN,
}
impl<'a> TryFrom<(&'a Banmen,&'a Move),TypeConvertError<String>> for Moved {
	fn try_from(s:(&'a Banmen,&'a Move)) -> Result<Moved, TypeConvertError<String>> {
		Ok(match s {
			(&Banmen(ref kinds),&Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,n))) => {
				match kinds[sy as usize - 1][9 - sx as usize] {
					KomaKind::SFu => Moved::To(MovedKind::Fu,(sx,sy),(dx,dy),n),
					KomaKind::SKyou => Moved::To(MovedKind::Kyou,(sx,sy),(dx,dy),n),
					KomaKind::SKei => Moved::To(MovedKind::Kei,(sx,sy),(dx,dy),n),
					KomaKind::SGin => Moved::To(MovedKind::Gin,(sx,sy),(dx,dy),n),
					KomaKind::SKin => Moved::To(MovedKind::Kin,(sx,sy),(dx,dy),n),
					KomaKind::SKaku => Moved::To(MovedKind::Kaku,(sx,sy),(dx,dy),n),
					KomaKind::SHisha => Moved::To(MovedKind::Hisha,(sx,sy),(dx,dy),n),
					KomaKind::SOu => Moved::To(MovedKind::SOu,(sx,sy),(dx,dy),n),
					KomaKind::SFuN => Moved::To(MovedKind::FuN,(sx,sy),(dx,dy),n),
					KomaKind::SKyouN => Moved::To(MovedKind::KyouN,(sx,sy),(dx,dy),n),
					KomaKind::SKeiN => Moved::To(MovedKind::KeiN,(sx,sy),(dx,dy),n),
					KomaKind::SGinN => Moved::To(MovedKind::GinN,(sx,sy),(dx,dy),n),
					KomaKind::SKakuN => Moved::To(MovedKind::KakuN,(sx,sy),(dx,dy),n),
					KomaKind::SHishaN => Moved::To(MovedKind::HishaN,(sx,sy),(dx,dy),n),
					KomaKind::GFu => Moved::To(MovedKind::Fu,(sx,sy),(dx,dy),n),
					KomaKind::GKyou => Moved::To(MovedKind::Kyou,(sx,sy),(dx,dy),n),
					KomaKind::GKei => Moved::To(MovedKind::Kei,(sx,sy),(dx,dy),n),
					KomaKind::GGin => Moved::To(MovedKind::Gin,(sx,sy),(dx,dy),n),
					KomaKind::GKin => Moved::To(MovedKind::Kin,(sx,sy),(dx,dy),n),
					KomaKind::GKaku => Moved::To(MovedKind::Kaku,(sx,sy),(dx,dy),n),
					KomaKind::GHisha => Moved::To(MovedKind::Hisha,(sx,sy),(dx,dy),n),
					KomaKind::GOu => Moved::To(MovedKind::GOu,(sx,sy),(dx,dy),n),
					KomaKind::GFuN => Moved::To(MovedKind::FuN,(sx,sy),(dx,dy),n),
					KomaKind::GKyouN => Moved::To(MovedKind::KyouN,(sx,sy),(dx,dy),n),
					KomaKind::GKeiN => Moved::To(MovedKind::KeiN,(sx,sy),(dx,dy),n),
					KomaKind::GGinN => Moved::To(MovedKind::GinN,(sx,sy),(dx,dy),n),
					KomaKind::GKakuN => Moved::To(MovedKind::KakuN,(sx,sy),(dx,dy),n),
					KomaKind::GHishaN => Moved::To(MovedKind::HishaN,(sx,sy),(dx,dy),n),
					KomaKind::Blank => {
						return Err(TypeConvertError::SyntaxError(String::from(
							"There is no koma in the coordinates of the move source position."
						)));
					}
				}
			},
			(_,&Move::Put(k,KomaDstPutPosition(x,y))) => {
				match k {
					MochigomaKind::Fu => Moved::Put(MochigomaKind::Fu,(x,y)),
					MochigomaKind::Kyou => Moved::Put(MochigomaKind::Kyou,(x,y)),
					MochigomaKind::Kei => Moved::Put(MochigomaKind::Kei,(x,y)),
					MochigomaKind::Gin => Moved::Put(MochigomaKind::Gin,(x,y)),
					MochigomaKind::Kin => Moved::Put(MochigomaKind::Kin,(x,y)),
					MochigomaKind::Hisha => Moved::Put(MochigomaKind::Hisha,(x,y)),
					MochigomaKind::Kaku => Moved::Put(MochigomaKind::Kaku,(x,y)),
				}
			}
		})
	}
}
const KANSUJI_MAP:[char; 10] = ['零','一','二','三','四','五','六','七','八','九'];

impl fmt::Display for Moved {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	match *self {
	 		Moved::To(MovedKind::Fu,(sx,sy),(dx,dy),true) => {
				write!(f,"{}{}歩 -> {}{}成",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::Kyou,(sx,sy),(dx,dy),true) => {
				write!(f,"{}{}香 -> {}{}成",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::Kei,(sx,sy),(dx,dy),true) => {
				write!(f,"{}{}桂 -> {}{}成",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::Gin,(sx,sy),(dx,dy),true) => {
				write!(f,"{}{}銀 -> {}{}成",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::Kaku,(sx,sy),(dx,dy),true) => {
				write!(f,"{}{}角 -> {}{}成",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::Hisha,(sx,sy),(dx,dy),true) => {
				write!(f,"{}{}飛 -> {}{}成",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::FuN,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}成歩 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::KyouN,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}成香 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::KeiN,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}成桂 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::GinN,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}成銀 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::KakuN,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}馬 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::HishaN,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}龍 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::Fu,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}歩 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::Kyou,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}香 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::Kei,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}桂 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::Gin,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}銀 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::Kin,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}金 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::Kaku,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}角 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::Hisha,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}飛 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::SOu,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}王 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::To(MovedKind::GOu,(sx,sy),(dx,dy),false) => {
				write!(f,"{}{}玉 -> {}{}",sx,KANSUJI_MAP[sy as usize],dx,KANSUJI_MAP[dy as usize])
	 		},
	 		Moved::Put(MochigomaKind::Fu,(x,y)) => {
	 			write!(f,"{}{}歩",x,KANSUJI_MAP[y as usize])
	 		},
	 		Moved::Put(MochigomaKind::Kyou,(x,y)) => {
	 			write!(f,"{}{}香",x,KANSUJI_MAP[y as usize])
	 		},
	 		Moved::Put(MochigomaKind::Kei,(x,y)) => {
	 			write!(f,"{}{}桂",x,KANSUJI_MAP[y as usize])
	 		},
	 		Moved::Put(MochigomaKind::Gin,(x,y)) => {
	 			write!(f,"{}{}銀",x,KANSUJI_MAP[y as usize])
	 		},
	 		Moved::Put(MochigomaKind::Kin,(x,y)) => {
	 			write!(f,"{}{}金",x,KANSUJI_MAP[y as usize])
	 		},
	 		Moved::Put(MochigomaKind::Kaku,(x,y)) => {
	 			write!(f,"{}{}角",x,KANSUJI_MAP[y as usize])
	 		},
	 		Moved::Put(MochigomaKind::Hisha,(x,y)) => {
	 			write!(f,"{}{}飛",x,KANSUJI_MAP[y as usize])
	 		},
	 		_ => write!(f,"UNKNOWN.")
	 	}
	 }
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum FoulKind {
	InvalidMove,
	PutFuAndMate,
	Sennichite,
	SennichiteOu,
	NotRespondedOute,
}
#[derive(Debug)]
pub enum SelfMatchEventKind {
	GameStart = 0,
	Moved,
	GameEnd,
	Abort,
}
impl MapEventKind<SelfMatchEventKind> for SelfMatchEvent {
	fn event_kind(&self) -> SelfMatchEventKind {
		match *self {
			SelfMatchEvent::GameStart(_,_,_) => SelfMatchEventKind::GameStart,
			SelfMatchEvent::Moved(_,_) => SelfMatchEventKind::Moved,
			SelfMatchEvent::GameEnd(_) => SelfMatchEventKind::GameEnd,
			SelfMatchEvent::Abort => SelfMatchEventKind::Abort,
		}
	}
}
impl From<SelfMatchEventKind> for usize {
	fn from(kind: SelfMatchEventKind) -> usize {
		kind as usize
	}
}
impl MaxIndex for SelfMatchEventKind {
	fn max_index() -> usize {
		SelfMatchEventKind::Abort as usize
	}
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum GameEndState {
	Win,
	Lose,
	Draw,
}
#[derive(Debug)]
pub enum UsiInitialPosition {
	Sfen(Banmen, MochigomaCollections),
	Startpos,
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum UsiGo {
	Go(UsiGoTimeLimit),
	Ponder(UsiGoTimeLimit),
	Mate(UsiGoMateTimeLimit),
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum UsiGoTimeLimit {
	None,
	Limit(Option<(u32,u32)>,Option<UsiGoByoyomiOrInc>),
	Infinite,
}
impl UsiGoTimeLimit {
	pub fn to_instant(&self,teban:Teban) -> Option<Instant> {
		let now = Instant::now();
		(match self {
			&UsiGoTimeLimit::None => None,
			&UsiGoTimeLimit::Infinite => None,
			&UsiGoTimeLimit::Limit(Some((ms,mg)),None) => {
				Some(match teban {
					Teban::Sente => {
						now + Duration::from_millis(ms as u64)
					},
					Teban::Gote => {
						now + Duration::from_millis(mg as u64)
					}
				})
			},
			&UsiGoTimeLimit::Limit(Some((ms,mg)),Some(UsiGoByoyomiOrInc::Byoyomi(b))) => {
				Some(match teban {
					Teban::Sente => {
						now + Duration::from_millis(ms as u64 + b as u64)
					},
					Teban::Gote => {
						now + Duration::from_millis(mg as u64 + b as u64)
					}
				})
			}
			&UsiGoTimeLimit::Limit(Some((ms,mg)),Some(UsiGoByoyomiOrInc::Inc(bs,bg))) => {
				Some(match teban {
					Teban::Sente => {
						now + Duration::from_millis(ms as u64 + bs as u64)
					},
					Teban::Gote => {
						now + Duration::from_millis(mg as u64 + bg as u64)
					}
				})
			},
			&UsiGoTimeLimit::Limit(None,Some(UsiGoByoyomiOrInc::Byoyomi(b))) => {
				Some(now + Duration::from_millis(b as u64))
			}
			&UsiGoTimeLimit::Limit(None,Some(UsiGoByoyomiOrInc::Inc(bs,bg))) => {
				Some(match teban {
					Teban::Sente => {
						now + Duration::from_millis(bs as u64)
					},
					Teban::Gote => {
						now + Duration::from_millis(bg as u64)
					}
				})
			},
			&UsiGoTimeLimit::Limit(None,None) => {
				Some(now)
			}
		})
	}
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum UsiGoMateTimeLimit {
	Limit(u32),
	Infinite,
}
impl UsiGoMateTimeLimit {
	pub fn to_instant(&self) -> Option<Instant> {
		match *self {
			UsiGoMateTimeLimit::Infinite => None,
			UsiGoMateTimeLimit::Limit(limit) => {
				let now = Instant::now();
				Some(now + Duration::from_millis(limit as u64))
			}
		}
	}
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum UsiGoByoyomiOrInc {
	Byoyomi(u32),
	Inc(u32,u32),
}
#[derive(Debug)]
pub enum SysEventOption {
	Str(String),
	Num(u32),
	Bool(bool),
}
impl Clone for SysEventOption {
	fn clone(&self) -> SysEventOption {
		match *self {
			SysEventOption::Str(ref s) => SysEventOption::Str(s.clone()),
			SysEventOption::Num(n) => SysEventOption::Num(n),
			SysEventOption::Bool(b) => SysEventOption::Bool(b),
		}
	}
}
#[derive(Debug)]
pub enum SysEventOptionKind {
	Str,
	Num,
	Bool,
}
impl<'a> TryFrom<&'a str,TypeConvertError<String>> for MochigomaCollections {
	fn try_from(s: &'a str) -> Result<MochigomaCollections, TypeConvertError<String>> {
		Ok(match &*s {
			"-" => MochigomaCollections::Pair(HashMap::new(),HashMap::new()),
			_ => {
				let mut chars = s.chars();

				let mut sente:HashMap<MochigomaKind,u32> = HashMap::new();
				let mut gote:HashMap<MochigomaKind,u32> = HashMap::new();

				while let Some(c) = chars.next() {
					let t = match c {
						'R' | 'B' | 'G' | 'S' | 'N' | 'L' | 'P' => Teban::Sente,
						'r' | 'b' | 'g' | 's' | 'n' | 'l' | 'p' => Teban::Gote,
						_ => {
							return Err(TypeConvertError::SyntaxError(
								String::from("Invalid SFEN character string (illegal representation character string of the piece)"
							)));
						}
					};

					let k = match c {
						'R' | 'r' => MochigomaKind::Hisha,
						'B' | 'b' => MochigomaKind::Kaku,
						'G' | 'g' => MochigomaKind::Kin,
						'S' | 's'=> MochigomaKind::Gin,
						'N' | 'n' => MochigomaKind::Kei,
						'L' | 'l' => MochigomaKind::Kyou,
						'P' | 'p' => MochigomaKind::Fu,
						_ => {
							return Err(TypeConvertError::LogicError(String::from(
								"SFEN This is a logic error of the pieces analysis phase of the character string analysis process.")
							));
						}
					};

					match chars.next() {
						Some(n) if n >= '1' && n <= '9' => {
							let mut ns = String::new();
							ns.push(n);

							let mut nchars = chars.clone();

							while let Some(next) = nchars.next() {
								match next {
									'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
										ns.push(next);
										chars.clone_from(&nchars);
									},
									_ => {
										break;
									}
								}
							}

							let n = ns.parse::<u32>()?;

							match t {
								Teban::Sente => {
									let n = match sente.get(&k) {
										Some(count) => count+n,
										None => n,
									};

									sente.insert(k,n);
								},
								Teban::Gote => {
									let n = match gote.get(&k) {
										Some(count) => count+n,
										None => n,
									};

									gote.insert(k,n);
								},
							}
						},
						_ => {
							return Err(TypeConvertError::SyntaxError(
								String::from("Invalid SFEN character string (no number of pieces count)"
							)));
						}
					}
				}

				MochigomaCollections::Pair(sente,gote)
			}
		})
	}
}
pub struct PositionParser {
}
impl PositionParser {
	pub fn new() -> PositionParser {
		PositionParser{}
	}

	pub fn parse<'a>(&self,params:&'a [&'a str]) -> Result<SystemEvent,TypeConvertError<String>> {
		let p = match params.len() {
			0 => {
				return Err(TypeConvertError::SyntaxError(String::from(
					"The format of the position command input is invalid."
				)));
			},
			_ => params,
		};

		match p[0] {
			"startpos"=> self.parse_startpos(&params[1..]),
			"sfen" => self.parse_sfen(&params[1..]),
			_ => {
				Err(TypeConvertError::SyntaxError(String::from(
					"The input form of the go command is invalid. (Insufficient parameters)"
				)))
			}
		}
	}

	fn parse_startpos<'a>(&self,params:&'a [&'a str]) -> Result<SystemEvent,TypeConvertError<String>> {
		let mut r:Vec<Move> = Vec::new();

		if params.len() == 0 {
			return Ok(SystemEvent::Position(Teban::Sente,UsiInitialPosition::Startpos,1,r));
		}

		match params[0] {
			"moves" if params.len() >= 2 => {
				for m in &params[1..] {
					r.push(Move::try_from(m)?);
				}

				Ok(SystemEvent::Position(Teban::Sente,UsiInitialPosition::Startpos,1,r))
			},
			_ => {
				return Err(TypeConvertError::SyntaxError(String::from(
					"The format of the position command input is invalid."
				)));
			}
		}
	}

	fn parse_sfen<'a>(&self,params:&'a [&'a str]) -> Result<SystemEvent,TypeConvertError<String>> {
		if params.len() > 4 && (params[4] != "moves" || params.len() <= 5) {
			return Err(TypeConvertError::SyntaxError(String::from(
					"The format of the position command input is invalid."
				)));
		}
		Ok(match params {
			params if params.len() > 3 => match (params[0],params[1],params[2],params[3]) {
				(p, t, m, n) => {
					let mut mv:Vec<Move> = Vec::new();

					if params.len() > 5 {
						for m in &params[5..] {
								mv.push(Move::try_from(m)?);
						}
					}

					SystemEvent::Position(
						Teban::try_from(t)?,
						UsiInitialPosition::Sfen(Banmen::try_from(p)?,MochigomaCollections::try_from(m)?),
						n.parse::<u32>()?,mv)
				}
			},
			_ => {
				return Err(TypeConvertError::SyntaxError(String::from(
					"The format of the position command input is invalid."
				)));
			}
		})
	}
}
struct UsiGoCreator {
	f:Box<Fn(UsiGoTimeLimit) -> SystemEvent>,
}
impl UsiGoCreator {
	pub fn new(f:Box<Fn(UsiGoTimeLimit) -> SystemEvent>) -> UsiGoCreator {
		UsiGoCreator {
			f:f,
		}
	}

	pub fn create(&self,l:UsiGoTimeLimit) -> SystemEvent {
		(*self.f)(l)
	}
}
pub struct GoParser {
}
impl GoParser {
	pub fn new() -> GoParser {
		GoParser{}
	}

	pub fn parse<'a>(&self,params:&'a [&'a str]) -> Result<SystemEvent, TypeConvertError<String>> {
		if params.len() == 0 {
			return Ok(SystemEvent::Go(UsiGo::Go(UsiGoTimeLimit::None)));
		}

		match params[0]{
			"mate" if params.len() == 2 => {
				match params[1] {
					"infinite" => return Ok(SystemEvent::Go(UsiGo::Mate(UsiGoMateTimeLimit::Infinite))),
					n => return Ok(SystemEvent::Go(
									UsiGo::Mate(UsiGoMateTimeLimit::Limit(n.parse::<u32>()?)))),
				}
			},
			_ => (),
		}

		let (params,f) = match params[0] {
			"ponder" if params.len() == 1 => {
				return Ok(SystemEvent::Go(UsiGo::Ponder(UsiGoTimeLimit::None)));
			},
			"ponder" => (&params[1..], UsiGoCreator::new(Box::new(|l| SystemEvent::Go(UsiGo::Ponder(l))))),
			_ => (params, UsiGoCreator::new(Box::new(|l| SystemEvent::Go(UsiGo::Go(l))))),
		};

		match params[0] {
			"infinite" => match params.len() {
				1 => {
					return Ok(f.create(UsiGoTimeLimit::Infinite));
				},
				_ => {
					return Err(TypeConvertError::SyntaxError(String::from(
						"The format of the position command input is invalid."
					)));
				}
			},
			_ => (),
		}

		let mut it = params.iter();
		let mut limit = None;
		let mut byori = None;

		while let Some(&p) = it.next() {
			match p {
				"btime" => {
					limit.map_or(Ok(()), |_| Err(TypeConvertError::SyntaxError(String::from(
						"The input form of the go command is invalid. (Duplicate parameters)"
					))))?;
					let bt = it.next().ok_or(TypeConvertError::SyntaxError(String::from(
						"The input form of the go command is invalid. (There is no value for item)"
					))).and_then(|n| match n.parse::<u32>() {
						Ok(n) => Ok(n),
						Err(_) => {
							Err(TypeConvertError::SyntaxError(String::from("Failed parse string to integer.")))
						}
					})?;
					let wt = match it.next() {
						Some(&"wtime") => {
							it.next().ok_or(
								TypeConvertError::SyntaxError(String::from(
									"The input form of the go command is invalid. (There is no value for item)"
								))).and_then(|n| match n.parse::<u32>() {
									Ok(n) => Ok(n),
									Err(_) => Err(TypeConvertError::SyntaxError(String::from("Failed parse string to integer.")))
								})?
						},
						_ => {
							return Err(TypeConvertError::SyntaxError(String::from(
								"The input form of the go command is invalid. (Insufficient parameters)"
							)));
						}
					};
					limit = Some((bt,wt));
				},
				"binc" => {
					byori.map_or(
						Ok(()),
						|_| Err(TypeConvertError::SyntaxError(String::from(
							"The input form of the go command is invalid. (Duplicate parameters)"
					))))?;
					let bi = it.next()
								.ok_or(TypeConvertError::SyntaxError(String::from(
									"The input form of the go command is invalid. (There is no value for item)"
								))).and_then(|n| match n.parse::<u32>() {
									Ok(n) => Ok(n),
									Err(_) => Err(TypeConvertError::SyntaxError(String::from("Failed parse string to integer."))),
								})?;
					let wi = match it.next() {
						Some(&"winc") => {
							it.next().ok_or(
								TypeConvertError::SyntaxError(String::from(
									"The input form of the go command is invalid. (There is no value for item)"
								))).and_then(|n| match n.parse::<u32>() {
									Ok(n) => Ok(n),
									Err(_) => Err(TypeConvertError::SyntaxError(String::from("Failed parse string to integer.")))
								})?
						},
						_ => {
							return Err(TypeConvertError::SyntaxError(String::from(
								"The input form of the go command is invalid. (Insufficient parameters)"
							)));
						}
					};
					byori = Some(UsiGoByoyomiOrInc::Inc(bi,wi));
				},
				"byoyomi" => {
					byori.map_or(
						Ok(()),
						|_| {
							Err(TypeConvertError::SyntaxError(String::from(
								"The input form of the go command is invalid. (Duplicate parameters)"
						)))})?;
					byori = it.next().ok_or(
						TypeConvertError::SyntaxError(String::from(
							"The input form of the go command is invalid. (There is no value for item)"
						))).and_then(|n| match n.parse::<u32>() {
							Ok(n) => Ok(Some(UsiGoByoyomiOrInc::Byoyomi(n))),
							Err(_) => Err(TypeConvertError::SyntaxError(String::from("Failed parse string to integer."))),
						})?;
				},
				_ => {
					return Err(TypeConvertError::SyntaxError(String::from(
						"The input form of the go command is invalid. (Unknown parameter)")));
				}
			}
		}

		it.next().map_or(
			limit.map_or(
				byori.map_or(
					Ok(f.create(UsiGoTimeLimit::None)),
					|_| Err(TypeConvertError::SyntaxError(String::from(
							"The input form of the go command is invalid. (Insufficient parameters)"
						)))
				),
				|ref limit| Ok(f.create(UsiGoTimeLimit::Limit(Some(*limit), byori)))
			),
			|_| Err(TypeConvertError::SyntaxError(String::from(
				"The input form of the go command is invalid. (Unknown parameter)")))
		)
	}
}
#[derive(Debug)]
pub struct EventQueue<E,K> where E: MapEventKind<K> + fmt::Debug, K: fmt::Debug {
	event_kind:PhantomData<K>,
	events:Vec<E>,
}
impl<E,K> EventQueue<E,K> where E: MapEventKind<K> + fmt::Debug, K: fmt::Debug {
	pub fn new() -> EventQueue<E,K> {
		EventQueue {
			event_kind:PhantomData::<K>,
			events: Vec::new()
		}
	}
	pub fn push(&mut self,e:E) {
		self.events.push(e);
	}
	pub fn drain_events(&mut self) -> Vec<E> {
		self.events.drain(0..).collect()
	}
	pub fn has_event(&self) -> bool {
		self.events.len() > 0
	}
}
pub trait EventDispatcher<'b,K,E,T,UE> where K: MaxIndex + fmt::Debug,
											E: MapEventKind<K> + fmt::Debug,
											UE: PlayerError,
											EventHandlerError<K,UE>: From<UE>,
											usize: From<K> {
	fn add_handler<F>(&mut self, id:K, handler:F) where F: Fn(&T,&E) ->
													Result<(), EventHandlerError<K,UE>> + 'b;

	fn add_once_handler<F>(&mut self, id:K, handler:F) where F: Fn(&T,&E) ->
													Result<(), EventHandlerError<K,UE>> + 'b;

	fn dispatch_events<'a>(&mut self, ctx:&T, event_queue:&'a Mutex<EventQueue<E,K>>) ->
										Result<(), EventDispatchError<'a,EventQueue<E,K>,E,UE>>
										where E: fmt::Debug, K: fmt::Debug,
												UE: Error + fmt::Debug,
												EventHandlerError<K,UE>: From<UE>,
												usize: From<K>;
}
pub struct USIEventDispatcher<'b,K,E,T,L,UE>
	where K: MaxIndex + fmt::Debug,
			E: MapEventKind<K> + fmt::Debug,
			L: Logger,
			UE: PlayerError,
			EventHandlerError<K,UE>: From<UE>,
			usize: From<K> {
	on_error_handler:Arc<Mutex<OnErrorHandler<L>>>,
	context_type:PhantomData<T>,
	event_kind:PhantomData<K>,
	handlers:Vec<Vec<Box<Fn(&T,&E) -> Result<(), EventHandlerError<K,UE>> + 'b>>>,
	once_handlers:Vec<Vec<Box<Fn(&T, &E) -> Result<(), EventHandlerError<K,UE>> + 'b>>>,
}
impl<'b,K,E,T,L,UE> USIEventDispatcher<'b,K,E,T,L,UE>
	where K: MaxIndex + fmt::Debug,
			E: MapEventKind<K> + fmt::Debug,
			L: Logger,
			UE: PlayerError,
			EventHandlerError<K,UE>: From<UE>,
			usize: From<K> {
	pub fn new(logger:&Arc<Mutex<L>>) -> USIEventDispatcher<'b,K,E,T,L,UE>
											where K: MaxIndex + fmt::Debug, usize: From<K>,
											E: MapEventKind<K> + fmt::Debug,
											L: Logger,
											UE: PlayerError,
											EventHandlerError<K,UE>: From<UE>, {

		let mut o = USIEventDispatcher {
			on_error_handler:Arc::new(Mutex::new(OnErrorHandler::new(logger.clone()))),
			context_type:PhantomData::<T>,
			event_kind:PhantomData::<K>,
			handlers:Vec::with_capacity(K::max_index()+1),
			once_handlers:Vec::with_capacity(K::max_index()+1),
		};
		for _ in 0..K::max_index() + 1 {
			o.handlers.push(Vec::new());
			o.once_handlers.push(Vec::new());
		}
		o
	}
}
impl<'b,K,E,T,L,UE> EventDispatcher<'b,K,E,T,UE> for USIEventDispatcher<'b,K,E,T,L,UE> where K: MaxIndex + fmt::Debug,
																		E: MapEventKind<K> + fmt::Debug,
																		L: Logger,
																		UE: PlayerError,
																		EventHandlerError<K,UE>: From<UE>,
																		usize: From<K> {
	fn add_handler<F>(&mut self, id:K, handler:F) where F: Fn(&T,&E) ->
											Result<(), EventHandlerError<K,UE>> + 'b {
		self.handlers[usize::from(id)].push(Box::new(handler));
	}

	fn add_once_handler<F>(&mut self, id:K, handler:F) where F: Fn(&T,&E) ->
											Result<(), EventHandlerError<K,UE>> + 'b {
		self.once_handlers[usize::from(id)].push(Box::new(handler));
	}

	fn dispatch_events<'a>(&mut self, ctx:&T, event_queue:&'a Mutex<EventQueue<E,K>>) ->
									Result<(), EventDispatchError<'a,EventQueue<E,K>,E,UE>>
									where E: fmt::Debug, K: fmt::Debug, usize: From<K> {
		let events = {
			event_queue.lock()?.drain_events()
		};

		let mut has_error = false;

		for e in &events {
			for h in &self.handlers[usize::from(e.event_kind())] {
				match h(ctx, e) {
					Ok(_) => true,
					Err(ref e) => {
						has_error = true;
						self.on_error_handler.lock().map(|h| h.call(e)).is_err()
					}
				};
			}

			if !self.once_handlers[usize::from(e.event_kind())].is_empty() {
				let once_handlers:Vec<Box<Fn(&T, &E) -> Result<(), EventHandlerError<K,UE>>>> =
											self.once_handlers[usize::from(e.event_kind())].drain(0..)
																							.collect();
				for h in &once_handlers {
					match h(ctx, e) {
						Ok(_) => true,
						Err(ref e) => {
							has_error = true;
							self.on_error_handler.lock().map(|h| h.call(e)).is_err()
						}
					};
				}
			}
		}

		match has_error {
			true => Err(EventDispatchError::ContainError),
			false => Ok(()),
		}
	}
}