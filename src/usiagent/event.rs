use std::fmt;
use std::marker::PhantomData;
use std::sync::Mutex;

use usiagent::TryFrom;
use usiagent::output::USIStdErrorWriter;
use usiagent::error::EventDispatchError;
use usiagent::error::EventHandlerError;
use usiagent::error::TypeConvertError;
use usiagent::UsiOutput;
use usiagent::Logger;
use usiagent::shogi::*;

pub trait MapEventKind<K> {
	fn event_kind(&self) -> K;
}
pub trait MaxIndex {
	fn max_index() -> usize;
}
#[derive(Debug)]
pub enum SystemEventKind {
	Usi = 0,
	IsReady,
	SetOption,
	UsiNewGame,
	Position,
	Go,
	Stop,
	PonderHit,
	GameOver,
	SendUsiCommand,
}
impl MaxIndex for SystemEventKind {
	fn max_index() -> usize {
		SystemEventKind::SendUsiCommand as usize
	}
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
	Gameover(GameEndState),
	SendUSICommand(UsiOutput),
}
#[derive(Debug)]
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
#[derive(Debug)]
pub enum MochigomaCollections {
	Empty,
	Pair(Vec<MochigomaKind>,Vec<MochigomaKind>),
}
#[derive(Debug)]
pub enum UsiGo {
	Go(UsiGoTimeLimit),
	Ponder(UsiGoTimeLimit),
	Mate(UsiGoMateTimeLimit),
}
#[derive(Debug)]
pub enum UsiGoKind {
	Go,
	Ponder,
}
#[derive(Debug)]
pub enum UsiGoTimeLimit {
	None,
	Limit(Option<(u32,u32)>,UsiGoByoyomiOrInc),
	Infinite,
}
#[derive(Debug)]
pub enum UsiGoMateTimeLimit {
	Limit(u32),
	Infinite,
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
impl<'a> TryFrom<&'a str,String> for MochigomaCollections {
	fn try_from(s: &'a str) -> Result<MochigomaCollections, TypeConvertError<String>> {
		Ok(match &*s {
			"-" => MochigomaCollections::Pair(Vec::new(),Vec::new()),
			_ => {
				let mut chars = s.chars();

				let mut sente:Vec<MochigomaKind> = Vec::new();
				let mut gote:Vec<MochigomaKind> = Vec::new();

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
									for _ in 0..n {
										sente.push(k);
									}
								},
								Teban::Gote => {
									for _ in 0..n {
										gote.push(k);
									}
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
			SystemEvent::Gameover(_) => SystemEventKind::GameOver,
			SystemEvent::SendUSICommand(_) => SystemEventKind::SendUsiCommand,
		}
	}
}
struct PositionParser {
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
			"startpos" => self.parse_startpos(&params[1..]),
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

		for m in params {
			r.push(Move::try_from(m)?);
		}

		Ok(SystemEvent::Position(Teban::Sente,UsiInitialPosition::Startpos,1,r))
	}

	fn parse_sfen<'a>(&self,params:&'a [&'a str]) -> Result<SystemEvent,TypeConvertError<String>> {
		Ok(match params {
			params if params.len() >= 4 => match (params[0],params[1],params[2],params[3]) {
				(p, t, m, n) => {
					let mut mv:Vec<Move> = Vec::new();

					for m in &params[4..] {
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
struct GoParser {
}
impl GoParser {
	pub fn new() -> GoParser {
		GoParser{}
	}

	pub fn parse<'a>(&self,params:&'a [&'a str]) -> Result<SystemEvent, TypeConvertError<String>> {
		match params[0] {
			"mate" if params.len() == 2 => {
				match params[1] {
					"infinite" => return Ok(SystemEvent::Go(UsiGo::Mate(UsiGoMateTimeLimit::Infinite))),
					n => return Ok(SystemEvent::Go(
									UsiGo::Mate(UsiGoMateTimeLimit::Limit(n.parse::<u32>()?)))),
				}
			},
			_ => (),
		}

		let (params,kind) = match params[0] {
			"ponder" => (&params[1..], UsiGoKind::Ponder),
			_ => (params, UsiGoKind::Go),
		};

		match params[0] {
			"infinite" => match params.len() {
				1 => match kind {
					UsiGoKind::Ponder => return Ok(SystemEvent::Go(UsiGo::Ponder(UsiGoTimeLimit::Infinite))),
					UsiGoKind::Go => return Ok(SystemEvent::Go(UsiGo::Go(UsiGoTimeLimit::Infinite))),
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
					match limit {
						Some(_) => {
							return Err(TypeConvertError::SyntaxError(String::from(
								"The input form of the go command is invalid. (Duplicate parameters)"
							)));
						},
						_ => (),
					}
					let bt = match it.next() {
						Some(n) => n.parse::<u32>()?,
						_ => {
							return Err(TypeConvertError::SyntaxError(String::from(
								"The input form of the go command is invalid. (There is no value for item)"
							)));
						}
					};
					let wt = match it.next() {
						Some(&"wtime") => match it.next () {
							Some(n) => n.parse::<u32>()?,
							_ => {
								return Err(TypeConvertError::SyntaxError(String::from(
									"The input form of the go command is invalid. (There is no value for item)"
								)));
							}
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
					match byori {
						Some(_) => {
							return Err(TypeConvertError::SyntaxError(String::from(
								"The input form of the go command is invalid. (Duplicate parameters)"
							)));
						},
						_ => (),
					}
					let bi = match it.next() {
						Some(n) => n.parse::<u32>()?,
						_ => {
							return Err(TypeConvertError::SyntaxError(String::from(
								"The input form of the go command is invalid. (There is no value for item)"
							)));
						}
					};
					let wi = match it.next() {
						Some(&"winc") => match it.next() {
							Some(n) => n.parse::<u32>()?,
							_ => {
								return Err(TypeConvertError::SyntaxError(String::from(
									"The input form of the go command is invalid. (There is no value for item)"
								)));
							}
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
					match byori {
						Some(_) => {
							return Err(TypeConvertError::SyntaxError(String::from(
								"The input form of the go command is invalid. (Duplicate parameters)"
							)));
						},
						_ => (),
					}
					byori = match it.next() {
						Some(n) => Some(UsiGoByoyomiOrInc::Byoyomi(n.parse::<u32>()?)),
						_ => {
							return Err(TypeConvertError::SyntaxError(String::from(
								"The input form of the go command is invalid. (There is no value for item)"
							)));
						}
					};
				},
				_ => {
					return Err(TypeConvertError::SyntaxError(String::from(
						"The input form of the go command is invalid. (Unknown parameter)")));
				}
			}
		}

		match it.next() {
			Some(_) => Err(TypeConvertError::SyntaxError(String::from(
						"The input form of the go command is invalid. (Unknown parameter)"))),
			None => match limit {
				None => Ok(match byori {
					None => match kind {
						UsiGoKind::Ponder => SystemEvent::Go(UsiGo::Ponder(UsiGoTimeLimit::None)),
						UsiGoKind::Go => SystemEvent::Go(UsiGo::Go(UsiGoTimeLimit::None)),
					},
					Some(ref byori) => match kind {
						UsiGoKind::Ponder => SystemEvent::Go(
												UsiGo::Ponder(UsiGoTimeLimit::Limit(limit,*byori))),
						UsiGoKind::Go => SystemEvent::Go(UsiGo::Go(UsiGoTimeLimit::Limit(limit,*byori))),
					}
				}),
				ref limit @ Some(_) => Ok(match byori {
					Some(byori) => match kind {
						UsiGoKind::Ponder => SystemEvent::Go(
													UsiGo::Ponder(UsiGoTimeLimit::Limit(*limit,byori))),
						UsiGoKind::Go => SystemEvent::Go(
													UsiGo::Go(UsiGoTimeLimit::Limit(*limit,byori))),
					},
					None => {
						return Err(TypeConvertError::SyntaxError(String::from(
							"The input form of the go command is invalid. (Insufficient parameters)"
						)));
					}
				})
			}
		}
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
pub trait EventDispatcher<K,E,T> where K: MaxIndex + fmt::Debug,
											E: MapEventKind<K> + fmt::Debug {
	fn add_handler(&mut self, id:K, handler:Box<Fn(&T,&E) ->
													Result<(), EventHandlerError>>);

	fn add_once_handler(&mut self, id:K, handler:Box<Fn(&T,&E) ->
													Result<(), EventHandlerError>>);

	fn dispatch_events<'a>(&mut self, ctx:&T, event_queue:&'a Mutex<EventQueue<E,K>>) ->
										Result<(), EventDispatchError<'a,EventQueue<E,K>>>
										where E: fmt::Debug, K: fmt::Debug;
}
pub struct USIEventDispatcher<K,E,T,L>
	where K: MaxIndex + fmt::Debug,
			E: MapEventKind<K> + fmt::Debug,
			L: Logger {
	logger:Mutex<L>,
	event_kind:PhantomData<K>,
	handlers:Vec<Vec<Box<Fn(&T,&E) -> Result<(), EventHandlerError>>>>,
	once_handlers:Vec<Vec<Box<Fn(&T, &E) -> Result<(), EventHandlerError>>>>,
}
impl<K,E,T,L> USIEventDispatcher<K,E,T,L>
	where K: MaxIndex + fmt::Debug,
			E: MapEventKind<K> + fmt::Debug,
			L: Logger {
	pub fn new(logger:Mutex<L>) -> USIEventDispatcher<K,E,T,L> {
		let mut o = USIEventDispatcher {
			logger:logger,
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
impl<K,E,T,L> EventDispatcher<K,E,T> for USIEventDispatcher<K,E,T,L> where K: MaxIndex + fmt::Debug,
																		E: MapEventKind<K> + fmt::Debug,
																		L: Logger,
																		usize: From<K> {
	fn add_handler(&mut self, id:K, handler:Box<Fn(&T,&E) ->
											Result<(), EventHandlerError>>) {
		self.handlers[usize::from(id)].push(handler);
	}

	fn add_once_handler(&mut self, id:K, handler:Box<Fn(&T,&E) ->
											Result<(), EventHandlerError>>) {
		self.once_handlers[usize::from(id)].push(handler);
	}

	fn dispatch_events<'a>(&mut self, ctx:&T, event_queue:&'a Mutex<EventQueue<E,K>>) ->
									Result<(), EventDispatchError<'a,EventQueue<E,K>>>
									where E: fmt::Debug, K: fmt::Debug {
		let events = {
			event_queue.lock()?.drain_events()
		};

		let mut has_error = false;

		for e in &events {
			for h in &self.handlers[usize::from(e.event_kind())] {
				match h(ctx, &e) {
					Ok(_) => (),
					Err(ref e) => {
						match self.logger.lock() {
							Ok(logger) => {
								logger.logging_error(e);
							},
							Err(_) => {
								USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
							}
						}
						has_error = true;
					}
				}
			}

			if !self.once_handlers[usize::from(e.event_kind())].is_empty() {
				let once_handlers:Vec<Box<Fn(&T, &E) -> Result<(), EventHandlerError>>> =
											self.once_handlers[usize::from(e.event_kind())].drain(0..)
																							.collect();
				for h in &once_handlers {
					match h(ctx, &e) {
						Ok(_) => (),
						Err(ref e) => {
							match self.logger.lock() {
								Ok(logger) => {
									logger.logging_error(e);
								},
								Err(_) => {
									USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
								}
							}
							has_error = true;
						}
					}
				}
			}
		}

		match has_error {
			true => Err(EventDispatchError::ContainError),
			false => Ok(()),
		}
	}
}