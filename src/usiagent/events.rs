use std::fmt;
use std::marker::PhantomData;
use std::sync::Mutex;
use std::io::{self, Write};
use std::fmt::Debug;
use std::fmt::Formatter;

use usiagent::errors::EventDispatchError;
use usiagent::errors::EventHandlerError;
use usiagent::UsiOutput;
use usiagent::Logger;
use usiagent::commands::*;

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
	Tail
}
impl MaxIndex for SystemEventKind {
	fn max_index() -> usize {
		SystemEventKind::Tail as usize
	}
}
#[derive(Debug)]
pub enum SystemEvent {
	Usi,
	IsReady,
	SetOption(String,SysEventOption),
	UsiNewGame,
	Position(Teban,UsiInitialPosition,u32),
	Go,
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
pub struct Banmen([KomaKind; 81]);
impl Debug for Banmen {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Banmen(ref v) => write!(f, "{}", v.iter()
												.map(|k| format!("{:?}", k))
												.collect::<Vec<String>>().join(" "))
		}
	}
}
#[derive(Debug)]
pub enum MochigomaCollections {
	Empty,
	Pair((Vec<MochigomaKind>,Vec<MochigomaKind>)),
}
#[derive(Debug)]
pub enum UsiGo {
	Go(UsiGoTimeLimit),
	Ponder(UsiGoTimeLimit),
	Mate(UsiGoMateTimeLimit),
}
#[derive(Debug)]
pub enum UsiGoTimeLimit {
	None,
	Limit(Option<u32>,Option<u32>,UsiGoByoyomiOrInc),
	Infinite,
}
#[derive(Debug)]
pub enum UsiGoMateTimeLimit {
	Limit(u32),
	Infinite,
}
#[derive(Debug)]
pub enum UsiGoByoyomiOrInc {
	None,
	Byoyomi(u32),
	Inc(u32,u32),
}
#[derive(Debug)]
pub enum SysEventOption {
	Str(String),
	Num(u32),
	Bool(bool),
}
impl MapEventKind<SystemEventKind> for SystemEvent {
	fn event_kind(&self) -> SystemEventKind {
		match *self {
			SystemEvent::Usi => SystemEventKind::Usi,
			SystemEvent::IsReady => SystemEventKind::IsReady,
			SystemEvent::SetOption(_,_) => SystemEventKind::SetOption,
			SystemEvent::UsiNewGame => SystemEventKind::UsiNewGame,
			SystemEvent::Position(_,_,_) => SystemEventKind::Position,
			SystemEvent::Go => SystemEventKind::Go,
			SystemEvent::Stop => SystemEventKind::Stop,
			SystemEvent::PonderHit => SystemEventKind::PonderHit,
			SystemEvent::Gameover(_) => SystemEventKind::GameOver,
			SystemEvent::SendUSICommand(_) => SystemEventKind::SendUsiCommand,
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
			handlers:Vec::with_capacity(K::max_index()),
			once_handlers:Vec::with_capacity(K::max_index()),
		};
		for _ in 0..K::max_index() {
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
								let stderr = io::stderr();
								let mut h = stderr.lock();
								h.write(b"Logger's exclusive lock could not be secured").unwrap();
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
									let stderr = io::stderr();
									let mut h = stderr.lock();
									h.write(b"Logger's exclusive lock could not be secured").unwrap();
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