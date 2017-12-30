use std::fmt;
use std::marker::PhantomData;
use std::sync::Mutex;
use usiagent::errors::EventDispatchError;
use usiagent::errors::EventHandlerError;
use usiagent::UsiOutput;
use usiagent::Logger;

pub trait MapEventKind<K> {
	fn event_kind(&self) -> K;
}
pub trait MaxIndex {
	fn max_index() -> usize;
}
pub enum SystemEventKind {
	SENDUSICOMMAND = 0,
	Tail
}
impl MaxIndex for SystemEventKind {
	fn max_index() -> usize {
		SystemEventKind::Tail as usize
	}
}
pub enum SystemEvent {
	SendUSICommand(UsiOutput),
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
	logger:L,
	event_kind:PhantomData<K>,
	handlers:Vec<Vec<Box<Fn(&T,&E) -> Result<(), EventHandlerError>>>>,
	once_handlers:Vec<Vec<Box<Fn(&T, &E) -> Result<(), EventHandlerError>>>>,
}
impl<K,E,T,L> USIEventDispatcher<K,E,T,L>
	where K: MaxIndex + fmt::Debug,
			E: MapEventKind<K> + fmt::Debug,
			L: Logger {
	pub fn new(logger:L) -> USIEventDispatcher<K,E,T,L> {
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
						self.logger.logging_error(e);
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
							self.logger.logging_error(e);
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