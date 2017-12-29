use std::fmt;
use std::marker::PhantomData;
use std::sync::Mutex;
use usiagent::errors::EventDispatchError;
use usiagent::errors::EventHandlerError;

pub trait MapEventKind<K> {
	fn event_kind(&self) -> K;
}
pub trait MaxIndex {
	fn max_index() -> usize;
}
pub enum SystemEventKind {
	USIID = 0,
	USIOPT,
	Tail
}
impl MaxIndex for SystemEventKind {
	fn max_index() -> usize {
		SystemEventKind::Tail as usize
	}
}
pub enum SystemEvent {
	USIID(String, String),
	USIOPT(String, USIOPTType),
}
pub enum USIOPTType {
	Check(Option<bool>),
	Spin(u32, u32),
	Combo(Option<String>, Option<Vec<String>>),
	Button,
	String(Option<String>),
	FileName(Option<String>),
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
pub struct USIEventDispatcher<K,E,T>
	where K: MaxIndex + fmt::Debug, E: MapEventKind<K> + fmt::Debug {
	event_kind:PhantomData<K>,
	handlers:Vec<Vec<Box<Fn(&T,&E) -> Result<(), EventHandlerError>>>>,
	once_handlers:Vec<Vec<Box<Fn(&T, &E) -> Result<(), EventHandlerError>>>>,
}
impl<K,E,T> USIEventDispatcher<K,E,T>
	where K: MaxIndex + fmt::Debug, E: MapEventKind<K> + fmt::Debug {
	pub fn new() -> USIEventDispatcher<K,E,T> {
		USIEventDispatcher {
			event_kind:PhantomData::<K>,
			handlers:Vec::with_capacity(K::max_index()),
			once_handlers:Vec::with_capacity(K::max_index()),
		}
	}
}
impl<K,E,T> EventDispatcher<K,E,T> for USIEventDispatcher<K,E,T> where K: MaxIndex + fmt::Debug,
																		E: MapEventKind<K> + fmt::Debug,
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

		for e in &events {
			for h in &self.handlers[usize::from(e.event_kind())] {
				h(ctx, &e)?;
			}

			if !self.once_handlers[usize::from(e.event_kind())].is_empty() {
				let once_handlers:Vec<Box<Fn(&T, &E) -> Result<(), EventHandlerError>>> =
											self.once_handlers[usize::from(e.event_kind())].drain(0..)
																							.collect();
				for h in &once_handlers {
					h(ctx, &e)?;
				}
			}
		}

		Ok(())
	}
}