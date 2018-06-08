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
#[derive(Debug)]
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
	GameStart(u32,String),
	Moved(Teban,Move),
	GameEnd(SelfMatchGameEndState),
	Abort,
}
#[derive(Debug)]
pub enum SelfMatchGameEndState {
	Win(Teban),
	Resign(Teban),
	NyuGyokuWin(Teban),
	NyuGyokuLose(Teban),
	Draw,
	Foul(Teban,FoulKind),
	Timeover(Teban),
}
#[derive(Debug)]
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
			SelfMatchEvent::GameStart(_,_) => SelfMatchEventKind::GameStart,
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
	pub fn to_instant(&self,teban:Teban,tinc:u32) -> (Option<Instant>,u32) {
		let now = Instant::now();
		let mut tinc = tinc;
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
						tinc = tinc + bs as u32;
						now + Duration::from_millis(ms as u64 + tinc as u64)
					},
					Teban::Gote => {
						tinc = tinc + bg as u32;
						now + Duration::from_millis(mg as u64 + tinc as u64)
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
		}, tinc)
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
impl<'a> TryFrom<&'a str,String> for MochigomaCollections {
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
			"sefn" => self.parse_sfen(&params[1..]),
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
		if params.len() >= 5 && params[4] != "moves" {
			return Err(TypeConvertError::SyntaxError(String::from(
					"The format of the position command input is invalid."
				)));
		}
		Ok(match params {
			params if params.len() >= 6 => match (params[0],params[1],params[2],params[3]) {
				(p, t, m, n) => {
					let mut mv:Vec<Move> = Vec::new();

					for m in &params[5..] {
							mv.push(Move::try_from(m)?);
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
}
pub trait EventDispatcher<K,E,T,UE> where K: MaxIndex + fmt::Debug,
											E: MapEventKind<K> + fmt::Debug,
											UE: PlayerError,
											EventHandlerError<K,UE>: From<UE>,
											usize: From<K> {
	fn add_handler(&mut self, id:K, handler:Box<Fn(&T,&E) ->
													Result<(), EventHandlerError<K,UE>>>);

	fn add_once_handler(&mut self, id:K, handler:Box<Fn(&T,&E) ->
													Result<(), EventHandlerError<K,UE>>>);

	fn dispatch_events<'a>(&mut self, ctx:&T, event_queue:&'a Mutex<EventQueue<E,K>>) ->
										Result<(), EventDispatchError<'a,EventQueue<E,K>,E,UE>>
										where E: fmt::Debug, K: fmt::Debug,
												UE: Error + fmt::Debug,
												EventHandlerError<K,UE>: From<UE>,
												usize: From<K>;
}
pub struct USIEventDispatcher<K,E,T,L,UE>
	where K: MaxIndex + fmt::Debug,
			E: MapEventKind<K> + fmt::Debug,
			L: Logger,
			UE: PlayerError,
			EventHandlerError<K,UE>: From<UE>,
			usize: From<K> {
	on_error_handler:Arc<Mutex<OnErrorHandler<L>>>,
	event_kind:PhantomData<K>,
	handlers:Vec<Vec<Box<Fn(&T,&E) -> Result<(), EventHandlerError<K,UE>>>>>,
	once_handlers:Vec<Vec<Box<Fn(&T, &E) -> Result<(), EventHandlerError<K,UE>>>>>,
}
impl<K,E,T,L,UE> USIEventDispatcher<K,E,T,L,UE>
	where K: MaxIndex + fmt::Debug,
			E: MapEventKind<K> + fmt::Debug,
			L: Logger,
			UE: PlayerError,
			EventHandlerError<K,UE>: From<UE>,
			usize: From<K> {
	pub fn new(logger:&Arc<Mutex<L>>) -> USIEventDispatcher<K,E,T,L,UE>
											where K: MaxIndex + fmt::Debug, usize: From<K>,
											E: MapEventKind<K> + fmt::Debug,
											L: Logger,
											UE: PlayerError,
											EventHandlerError<K,UE>: From<UE>, {

		let mut o = USIEventDispatcher {
			on_error_handler:Arc::new(Mutex::new(OnErrorHandler::new(logger.clone()))),
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
impl<K,E,T,L,UE> EventDispatcher<K,E,T,UE> for USIEventDispatcher<K,E,T,L,UE> where K: MaxIndex + fmt::Debug,
																		E: MapEventKind<K> + fmt::Debug,
																		L: Logger,
																		UE: PlayerError,
																		EventHandlerError<K,UE>: From<UE>,
																		usize: From<K> {
	fn add_handler(&mut self, id:K, handler:Box<Fn(&T,&E) ->
											Result<(), EventHandlerError<K,UE>>>) {
		self.handlers[usize::from(id)].push(handler);
	}

	fn add_once_handler(&mut self, id:K, handler:Box<Fn(&T,&E) ->
											Result<(), EventHandlerError<K,UE>>>) {
		self.once_handlers[usize::from(id)].push(handler);
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