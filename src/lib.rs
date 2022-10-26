//! USIプロトコルを用いた将棋AIを実装するためのフレームワーク
#![doc(html_root_url="https://j6k1.github.io/USIAgent/")]
extern crate chrono;
extern crate rand;
#[macro_use]
extern crate crossbeam_channel;
extern crate queuingtask;

pub mod event;
pub mod error;
pub mod command;
pub mod logger;
pub mod string;
pub mod output;
pub mod input;
pub mod player;
pub mod shogi;
pub mod interpreter;
pub mod hash;
pub mod selfmatch;
pub mod protocol;
pub mod rule;

use std::error::Error;
use std::fmt;
use std::{thread,time};
use std::convert::TryFrom;
use std::time::Instant;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::marker::Send;
use std::marker::PhantomData;

use queuingtask::ThreadQueue;

use command::*;
use event::*;
use error::*;
use logger::*;
use input::*;
use output::*;
use interpreter::*;
use player::*;
use shogi::*;
use protocol::*;
use rule::*;

/// enumの各項目にインデックスが対応する型の最大のインデックスを取得する
pub trait MaxIndex {
	fn max_index() -> usize;
}
/// `query`で表されるものを検索する
pub trait Find<Q,R> {
	fn find(&self,query:&Q) -> Option<R>;
}
/// エラーを受け取ってロガーで出力する構造体
pub struct OnErrorHandler<L> where L: Logger {
	logger:Arc<Mutex<L>>,
}
impl<L> OnErrorHandler<L> where L: Logger {
	/// `OnErrorHanderl`の生成
	///
	/// # Arguments
	/// * `logger` - ロガー
	pub fn new(logger:Arc<Mutex<L>>) -> OnErrorHandler<L> {
		OnErrorHandler {
			logger:logger,
		}
	}

	/// エラーを出力する
	///
	/// # Arguments
	/// * `e` - エラーオブジェクト
	pub fn call<E>(&self,e:&E) -> bool where E: Error {
		self.logger.lock().map(|mut logger| logger.logging_error(e)).map_err(|_| {
			USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
				false
		}).is_err()
	}
}
/// 処理を実行した結果発生したエラーを自動で出力するためのサンドボックス
pub struct SandBox {

}
impl SandBox {
	/// その場で処理を実行してエラー発生時`OnErrorHandler`を用いて出力する
	///
	/// # Arguments
	/// * `f` - 処理を実行する関数
	/// * `on_error_handler` - エラーをログファイルなどに出力するためのオブジェクト
	pub fn immediate<F,R,E,L>(f:F,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>) -> Result<R,E>
	where E: Error, F: FnOnce() -> Result<R,E>, L: Logger {
		let r = f();
		match r {
			Ok(_) => (),
			Err(ref e) => {
				let _ = on_error_handler.lock().map(|h| h.call(e));
			}
		}
		r
	}
}
/// go ponderによって生成された手を相手の指し手が決まるまで覚えておくための構造体
pub enum OnAcceptMove  {
	/// 指し手を格納済み
	Some(BestMove),
	/// まだ指し手が格納されていない
	None,
}
impl OnAcceptMove {
	/// `OnAcceptMove`の生成
	///
	/// # Arguments
	/// * `m` - 指し手
	pub fn new(m:BestMove) -> OnAcceptMove {
		OnAcceptMove::Some(m)
	}

	/// 覚えておいた手を通知する
	///
	/// # Arguments
	/// * `system_event_queue` - システムイベントキュー
	/// * `on_error_handler` - エラーハンドラー
	pub fn notify<L>(&self,
		system_event_queue:&Arc<Mutex<SystemEventQueue>>,
		on_error_handler:&Arc<Mutex<OnErrorHandler<L>>>) where L: Logger, Arc<Mutex<L>>: Send + 'static {
		match *self {
			OnAcceptMove::Some(m) => {
				match UsiOutput::try_from(&UsiCommand::UsiBestMove(m)) {
					Ok(cmd) => match system_event_queue.lock() {
						Ok(mut system_event_queue) => {
							system_event_queue.push(SystemEvent::SendUsiCommand(cmd));
						},
						Err(ref e) => {
							let _ = on_error_handler.lock().map(|h| h.call(e));
						}
					},
					Err(ref e) => {
						let _ = on_error_handler.lock().map(|h| h.call(e));
					}
				}
			},
			OnAcceptMove::None => (),
		};
	}
}
/// USIプロトコルをイベントシステムを用いてやり取りするための機能の実装
#[derive(Debug)]
pub struct UsiAgent<T,E>
	where T: USIPlayer<E> + fmt::Debug + Send + 'static,
			E: PlayerError,
			EventHandlerError<SystemEventKind, E>: From<E> {
	player_error_type:PhantomData<E>,
	player:Arc<Mutex<T>>,
	system_event_queue:Arc<Mutex<SystemEventQueue>>,
}
impl<T,E> UsiAgent<T,E>
	where T: USIPlayer<E> + fmt::Debug + Send + 'static,
			E: PlayerError,
			EventHandlerError<SystemEventKind, E>: From<E> {
	/// `UsiAgent`の生成
	///
	/// # Arguments
	/// * `player` - プレイヤーオブジェクト
	pub fn new(player:T) -> UsiAgent<T,E>
	where T: USIPlayer<E> + fmt::Debug,
			Arc<Mutex<T>>: Send + 'static,
			E: Error + fmt::Debug {
		UsiAgent {
			player_error_type:PhantomData::<E>,
			player:Arc::new(Mutex::new(player)),
			system_event_queue:Arc::new(Mutex::new(EventQueue::new())),
		}
	}

	/// デフォルト設定で開始（ログファイルのパスlogs/log.txt,ログをファイルに記録）
	///
	/// # Arguments
	/// * `on_error` - エラー発生時に呼ばれるコールバック関数。エラーオブジェクトへの参照とロガーが渡される。
	pub fn start_default<F>(&self,on_error:F) ->
		Result<(),USIAgentRunningError<SystemEventQueue,E>>
		where F: FnMut(Option<Arc<Mutex<OnErrorHandler<FileLogger>>>>,
					&USIAgentRunningError<SystemEventQueue,E>) {
		self.start_with_log_path(String::from("logs/log.txt"),on_error)
	}

	/// ログファイルのパスを指定して開始
	///
	/// # Arguments
	/// * `path` - ログファイルのパス
	/// * `on_error` - エラー発生時に呼ばれるコールバック関数。エラーオブジェクトへの参照とロガーが渡される。
	pub fn start_with_log_path<F>(&self,path:String,mut on_error:F) ->
		Result<(),USIAgentRunningError<SystemEventQueue,E>>
		where F: FnMut(Option<Arc<Mutex<OnErrorHandler<FileLogger>>>>,
					&USIAgentRunningError<SystemEventQueue,E>) {

		let logger = match FileLogger::new(path) {
			Err(_) => {
				let e = USIAgentStartupError::IOError(String::from(
					"The log output destination file could not be opened."
				));
				let e = USIAgentRunningError::from(e);
				on_error(None,&e);
				return Err(e);
			},
			Ok(logger) => logger,
		};

		let input_reader = USIStdInputReader::new();
		let output_writer = USIStdOutputWriter::new();

		self.start(input_reader,output_writer,logger,on_error)
	}

	/// `Logger`,`USIInputReader`,`USIOutputWriter`を指定して開始
	///
	/// # Arguments
	/// * `reader` - 入力を読み取るためのオブジェクト。実装によって標準入力以外から読み取るものを指定することも可能。
	/// * `writer` - USIコマンドを出力するためのオブジェクト。実装によって標準出力以外へ書き込むものを指定することも可能。
	/// * `logger` - ログを書き込むためのオブジェクト。実装によってファイル以外に書き込むものを指定することも可能。
	/// * `on_error` - エラー発生時に呼ばれるコールバック関数。エラーオブジェクトへの参照とロガーが渡される。
	pub fn start<R,W,L,F>(&self,reader:R,writer:W,logger:L,mut on_error:F) ->
		Result<(),USIAgentRunningError<SystemEventQueue,E>>
		where R: USIInputReader, W: USIOutputWriter, L: Logger + fmt::Debug,
			F: FnMut(Option<Arc<Mutex<OnErrorHandler<L>>>>,
					&USIAgentRunningError<SystemEventQueue,E>),
			EventHandlerError<SystemEventKind, E>: From<E>,
			R: Send + 'static,
			L: Send + 'static,
			W: Send + 'static,
			OnAcceptMove: Send + 'static {

		let logger_arc = Arc::new(Mutex::new(logger));
		let on_error_handler_arc = Arc::new(Mutex::new(OnErrorHandler::new(logger_arc.clone())));
		let on_error_handler = on_error_handler_arc.clone();

		let r = self.run(reader,writer,logger_arc,on_error_handler_arc);

		if let Err(ref e) = r {
			on_error(Some(on_error_handler),e);
		}

		r
	}

	fn run<R,W,L>(&self,reader:R,writer:W,logger_arc:Arc<Mutex<L>>,
								on_error_handler_arc:Arc<Mutex<OnErrorHandler<L>>>) ->
		Result<(),USIAgentRunningError<SystemEventQueue,E>>
		where R: USIInputReader, W: USIOutputWriter, L: Logger + fmt::Debug,
			EventHandlerError<SystemEventKind, E>: From<E>,
			R: Send + 'static,
			L: Send + 'static,
			W: Send + 'static,
			OnAcceptMove: Send + 'static {
		let writer_arc = Arc::new(Mutex::new(writer));

		let system_event_queue_arc = self.system_event_queue.clone();

		let mut system_event_dispatcher:SystemEventDispatcher<UsiAgent<T,E>,E,L> = USIEventDispatcher::new(&on_error_handler_arc);

		let user_event_queue:UserEventQueue = EventQueue::new();
		let user_event_queue_arc = Arc::new(Mutex::new(user_event_queue));
		let thread_queue_arc = Arc::new(Mutex::new(ThreadQueue::new()));

		let quit_ready_arc = Arc::new(AtomicBool::new(false));
		let think_start_time_arc = Arc::new(Mutex::new(None));

		let writer = writer_arc.clone();

		let on_error_handler = on_error_handler_arc.clone();

		system_event_dispatcher.add_handler(SystemEventKind::SendUsiCommand, move |_,e| {
			match e {
				&SystemEvent::SendUsiCommand(UsiOutput::Command(ref s)) => {
					match writer.lock() {
						Err(ref e) => {
							let _ = on_error_handler.lock().map(|h| h.call(e));
						},
						Ok(ref writer) => {
							let _ = writer.write(s);
						}
					};
					Ok(())
				},
				e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		});

		let on_error_handler = on_error_handler_arc.clone();

		system_event_dispatcher.add_handler(SystemEventKind::Usi, move |ctx,e| {
			match e {
				&SystemEvent::Usi => {
					let mut commands:Vec<UsiCommand> = Vec::new();

					match ctx.player.lock() {
						Ok(mut player) => {
							commands.push(UsiCommand::UsiId(T::ID.to_string(),T::AUTHOR.to_string()));
							for cmd in player.get_options()?.iter()
															.map(|(k,v)| UsiCommand::UsiOption(k.clone(),v.clone()))
															.collect::<Vec<UsiCommand>>().into_iter() {
								commands.push(cmd);
							}
						},
						Err(_) => {
							return Err(EventHandlerError::Fail(String::from(
								"Could not get exclusive lock on player object"
							)));
						}
					};

					commands.push(UsiCommand::UsiOk);

					let mut outputs:Vec<UsiOutput> = Vec::new();

					for cmd in &commands {
						outputs.push(UsiOutput::try_from(cmd)?);
					}

					match ctx.system_event_queue.lock() {
						Ok(mut system_event_queue) => {
							for cmd in outputs {
								system_event_queue.push(SystemEvent::SendUsiCommand(cmd));
							}
						},
						Err(ref e) => {
							let _ = on_error_handler.lock().map(|h| h.call(e));
						}
					};
					Ok(())
				},
				e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		});

		let on_error_handler = on_error_handler_arc.clone();
		let thread_queue = thread_queue_arc.clone();
		let writer = writer_arc.clone();

		system_event_dispatcher.add_handler(SystemEventKind::IsReady, move |ctx,e| {
			match e {
				&SystemEvent::IsReady => {
					let system_event_queue = ctx.system_event_queue.clone();
					let on_error_handler_inner = on_error_handler.clone();
					let player = ctx.player.clone();

					let writer_inner = writer.clone();

					match thread_queue.lock() {
						Ok(mut thread_queue) => {
							thread_queue.submit(move || {
								match player.lock() {
									Ok(mut player) => {
										match player.take_ready(OnKeepAlive::new(writer_inner,on_error_handler_inner.clone())) {
											Ok(_) => (),
											Err(ref e) => {
												let _ = on_error_handler_inner.lock().map(|h| h.call(e));
												return;
											}
										}
										match UsiOutput::try_from(&UsiCommand::UsiReadyOk) {
											Ok(cmd) => {
												match system_event_queue.lock() {
													Ok(mut system_event_queue) => {
														system_event_queue.push(SystemEvent::SendUsiCommand(cmd));
													},
													Err(ref e) => {
														let _ = on_error_handler_inner.lock().map(|h| h.call(e));
													}
												};
											},
											Err(ref e) => {
												let _ = on_error_handler_inner.lock().map(|h| h.call(e));
											}
										}
									},
									Err(ref e) => {
										let _ = on_error_handler_inner.lock().map(|h| h.call(e));
									}
								};
							}).map(|_| {
								()
							}).map_err(|_| {
								EventHandlerError::Fail(
									String::from("An error occurred while starting the user thread."))
							})
						},
						Err(_) => {
							Err(EventHandlerError::Fail(
									String::from("Could not get exclusive lock on thread queue object.")))
						}
					}
				},
				e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		});

		system_event_dispatcher.add_handler(SystemEventKind::SetOption, move |ctx,e| {
			match e {
				&SystemEvent::SetOption(ref name, ref value) => {
					match ctx.player.lock() {
						Ok(mut player) => {
							player.set_option(name.clone(), value.clone())?;
						},
						Err(_) => {
							return Err(EventHandlerError::Fail(String::from(
								"Could not get exclusive lock on player object"
							)));
						}
					};
					Ok(())
				},
				e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		});

		let think_start_time = think_start_time_arc.clone();

		system_event_dispatcher.add_handler(SystemEventKind::UsiNewGame, move |ctx,e| {
			match e {
				&SystemEvent::UsiNewGame => {
					match think_start_time.lock() {
						Ok(mut think_start_time) => {
							*think_start_time = None;
						},
						Err(_) => {
							return Err(EventHandlerError::Fail(String::from(
								"Could not get exclusive lock on think_start_time object"
							)));
						}
					};
					match ctx.player.lock() {
						Ok(mut player) => {
							player.newgame()?;
						},
						Err(_) => {
							return Err(EventHandlerError::Fail(String::from(
								"Could not get exclusive lock on player object"
							)));
						}
					};
					Ok(())
				},
				e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		});

		let on_error_handler = on_error_handler_arc.clone();
		let thread_queue = thread_queue_arc.clone();

		system_event_dispatcher.add_handler(SystemEventKind::Position, move |ctx,e| {
			match e {
				&SystemEvent::Position(ref t, ref p, ref n, ref v) => {
					let(b,m) = match p {
						&UsiInitialPosition::Startpos => {
							(rule::BANMEN_START_POS.clone(), MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()))
						},
						&UsiInitialPosition::Sfen(ref b,MochigomaCollections::Pair(ref ms,ref mg)) => {
							(b.clone(),MochigomaCollections::Pair(ms.clone(),mg.clone()))
						},
						&UsiInitialPosition::Sfen(ref b,MochigomaCollections::Empty) => {
							(b.clone(),MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()))
						}
					};

					let (ms,mg) = match m {
						MochigomaCollections::Pair(ms, mg) => (ms, mg),
						_ => (Mochigoma::new(),Mochigoma::new())
					};

					let on_error_handler_inner = on_error_handler.clone();
					let player = ctx.player.clone();
					let v = v.clone();
					let n = n.clone();
					let t = t.clone();

					match thread_queue.lock() {
						Ok(mut thread_queue) => {
							thread_queue.submit(move || {
								match player.lock() {
									Ok(mut player) => {
										match player.set_position(t, b, ms, mg, n, v) {
											Ok(_) => (),
											Err(ref e) => {
												let _ = on_error_handler_inner.lock().map(|h| h.call(e));
											}
										}
									},
									Err(ref e) => {
										let _ = on_error_handler_inner.lock().map(|h| h.call(e));
									}
								};
							}).map(|_| {
								()
							}).map_err(|_| {
								EventHandlerError::Fail(
									String::from("An error occurred while starting the user thread."))
							})
						},
						Err(_) => {
							Err(EventHandlerError::Fail(
									String::from("Could not get exclusive lock on thread queue object.")))
						}
					}
				},
				e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		});

		let busy = false;
		let busy_arc = Arc::new(AtomicBool::new(busy));

		let on_error_handler = on_error_handler_arc.clone();

		let on_delay_move_handler_arc:Arc<Mutex<OnAcceptMove>> = Arc::new(Mutex::new(OnAcceptMove::None));
		let allow_immediate_move_arc = Arc::new(AtomicBool::new(false));
		let allow_immediate_move = allow_immediate_move_arc.clone();
		let on_delay_move_handler = on_delay_move_handler_arc.clone();

		let user_event_queue = user_event_queue_arc.clone();

		let busy = busy_arc.clone();

		let thread_queue = thread_queue_arc.clone();

		let in_ponder_arc = Arc::new(AtomicBool::new(false));

		let writer = writer_arc.clone();

		let think_start_time = think_start_time_arc.clone();

		let in_ponder = in_ponder_arc.clone();

		system_event_dispatcher.add_handler(SystemEventKind::Go, move |ctx,e| {
			busy.store(true,Ordering::Release);

			match user_event_queue.lock() {
				Ok(mut user_event_queue) => {
					user_event_queue.clear();
				},
				Err(_) => {
					return Err(EventHandlerError::Fail(String::from(
						"Could not get exclusive lock on user event queue object."
					)));
				}
			}

			let think_start_time = match think_start_time.lock() {
				Ok(mut think_start_time) => {
					think_start_time.take().unwrap_or(Instant::now())
				},
				Err(_) => {
					return Err(EventHandlerError::Fail(String::from(
						"Could not get exclusive lock on think_start_time object"
					)));
				}
			};

			let is_ponder = if let SystemEvent::Go(UsiGo::Ponder(_)) = *e {
				true
			} else {
				false
			};

			in_ponder.store(is_ponder,Ordering::Release);

			match *e {
				SystemEvent::Go(UsiGo::Ponder(ref opt)) |
					SystemEvent::Go(UsiGo::Go(ref opt @ UsiGoTimeLimit::Infinite)) => {

					let player = ctx.player.clone();
					let system_event_queue = ctx.system_event_queue.clone();
					let on_error_handler_inner = on_error_handler.clone();
					let allow_immediate_move_inner = allow_immediate_move.clone();
					let on_delay_move_handler_inner = on_delay_move_handler.clone();
					let user_event_queue_inner = user_event_queue.clone();
					let opt = Arc::new(*opt);
					let opt = opt.clone();
					let busy_inner = busy.clone();

					allow_immediate_move.store(false,Ordering::Release);

					match thread_queue.lock() {
						Ok(mut thread_queue) => {
							let info_send_worker = InfoSendWorker::new(writer.clone(),on_error_handler_inner.clone());

							let info_sender = {
								let info_send_worker = info_send_worker.clone();
								USIInfoSender::new(info_send_worker)
							};

							let pinfo_sender = USIPeriodicallyInfo::new(
																		writer.clone(),
																		false);

							thread_queue.submit(move || {
								match player.lock() {
									Ok(mut player) => {
										let bm = if is_ponder {
											match player.think_ponder(&*opt,
													user_event_queue_inner.clone(),
													info_sender,
													pinfo_sender,on_error_handler_inner.clone()) {
												Ok(bm) => bm,
												Err(ref e) => {
													let _ = on_error_handler_inner.lock().map(|h| h.call(e));
													return;
												}
											}
										} else {
											match player.think(Instant::now(),
													&*opt,
													user_event_queue_inner.clone(),
													info_sender,
													pinfo_sender,
													on_error_handler_inner.clone()) {
												Ok(bm) => bm,
												Err(ref e) => {
													let _ = on_error_handler_inner.lock().map(|h| h.call(e));
													return;
												}
											}
										};

										if let Err(ref e) = info_send_worker.quit() {
											let _ = on_error_handler_inner.lock().map(|h| h.call(e));
											return;
										}

										busy_inner.store(false,Ordering::Release);

										match bm {
											BestMove::Abort => {
												return;
											},
											_ => (),
										}

										match UsiOutput::try_from(&UsiCommand::UsiBestMove(bm)) {
											Ok(cmd) => {
												if allow_immediate_move_inner.load(Ordering::Acquire) {
													match system_event_queue.lock() {
														Ok(mut system_event_queue) => {
															system_event_queue.push(SystemEvent::SendUsiCommand(cmd));
														},
														Err(ref e) => {
															let _ = on_error_handler_inner.lock().map(|h| h.call(e));
														}
													}
												} else {
													match on_delay_move_handler_inner.lock() {
														Ok(mut on_delay_move_handler_inner) => {
															*on_delay_move_handler_inner = OnAcceptMove::new(bm);
														},
														Err(ref e) => {
															let _ = on_error_handler_inner.lock().map(|h| h.call(e));
														}
													}
												}
											},
											Err(ref e) => {
												let _ = on_error_handler_inner.lock().map(|h| h.call(e));
											}
										}
									},
									Err(ref e) => {
										let _ = on_error_handler_inner.lock().map(|h| h.call(e));
									}
								};
							}).map(|_| {
								()
							}).map_err(|_| {
								EventHandlerError::Fail(
									String::from("An error occurred while starting the user thread."))
							})
						},
						Err(_) => {
							Err(EventHandlerError::Fail(
									String::from("Could not get exclusive lock on thread queue object.")))
						}
					}
				},
				SystemEvent::Go(UsiGo::Go(ref opt)) => {
					let system_event_queue = ctx.system_event_queue.clone();
					let on_error_handler_inner = on_error_handler.clone();
					let player = ctx.player.clone();
					let user_event_queue_inner = user_event_queue.clone();
					let opt = Arc::new(*opt);
					let opt = opt.clone();
					let busy_inner = busy.clone();

					match thread_queue.lock() {
						Ok(mut thread_queue) => {
							let info_send_worker = InfoSendWorker::new(writer.clone(),on_error_handler_inner.clone());

							let info_sender = {
								let info_send_worker = info_send_worker.clone();
								USIInfoSender::new(info_send_worker)
							};

							let pinfo_sender = USIPeriodicallyInfo::new(
																		writer.clone(),
																		false);

							thread_queue.submit(move || {
								match player.lock() {
									Ok(mut player) => {
										let m = match player.think(think_start_time,
														&*opt,
														user_event_queue_inner.clone(),
														info_sender,
														   pinfo_sender,
														   on_error_handler_inner.clone()) {
															Ok(m) => m,
															Err(ref e) => {
																let _ = on_error_handler_inner.lock().map(|h| h.call(e));
																return;
															}
														};

										if let Err(ref e) = info_send_worker.quit() {
											let _ = on_error_handler_inner.lock().map(|h| h.call(e));
											return;
										}

										busy_inner.store(false,Ordering::Release);

										match m {
											BestMove::Abort => {
												return;
											},
											_ => (),
										}

										match UsiOutput::try_from(&UsiCommand::UsiBestMove(m)) {
											Ok(cmd) => {
												match system_event_queue.lock() {
													Ok(mut system_event_queue) => system_event_queue.push(SystemEvent::SendUsiCommand(cmd)),
													Err(ref e) => {
														let _ = on_error_handler_inner.lock().map(|h| h.call(e));
													}
												};
											},
											Err(ref e) => {
												let _ = on_error_handler_inner.lock().map(|h| h.call(e));
											}
										}
									},
									Err(ref e) => {
										let _ = on_error_handler_inner.lock().map(|h| h.call(e));
									}
								};
							}).map(|_| {
								()
							}).map_err(|_| {
								EventHandlerError::Fail(
									String::from("An error occurred while starting the user thread."))
							})
						},
						Err(_) => {
							Err(EventHandlerError::Fail(
									String::from("Could not get exclusive lock on thread queue object.")))
						}
					}
				},
				SystemEvent::Go(UsiGo::Mate(opt)) => {
					let system_event_queue = ctx.system_event_queue.clone();
					let on_error_handler_inner = on_error_handler.clone();
					let player = ctx.player.clone();
					let user_event_queue_inner = user_event_queue.clone();
					let opt = Arc::new(opt);
					let opt = opt.clone();
					let busy_inner = busy.clone();

					match thread_queue.lock() {
						Ok(mut thread_queue) => {
							let info_send_worker = InfoSendWorker::new(writer.clone(),on_error_handler_inner.clone());

							let info_sender = {
								let info_send_worker = info_send_worker.clone();
								USIInfoSender::new(info_send_worker)
							};

							let pinfo_sender = USIPeriodicallyInfo::new(writer.clone(),false);

							thread_queue.submit(move || {
								match player.lock() {
									Ok(mut player) => {
										let m = match player.think_mate(&*opt,
														user_event_queue_inner.clone(),
														info_sender,
														pinfo_sender,
														on_error_handler_inner.clone()) {
															Ok(m) => m,
															Err(ref e) => {
																let _ = on_error_handler_inner.lock().map(|h| h.call(e));
																return;
															}
														};

										if let Err(ref e) = info_send_worker.quit() {
											let _ = on_error_handler_inner.lock().map(|h| h.call(e));
											return;
										}

										busy_inner.store(false,Ordering::Release);

										match m {
											CheckMate::Abort => {
												return;
											},
											_ => (),
										}

										match UsiOutput::try_from(&UsiCommand::UsiCheckMate(m)) {
											Ok(cmd) => {
												match system_event_queue.lock() {
													Ok(mut system_event_queue) => system_event_queue.push(SystemEvent::SendUsiCommand(cmd)),
													Err(ref e) => {
														let _ = on_error_handler_inner.lock().map(|h| h.call(e));
													}
												};
											},
											Err(ref e) => {
												let _ = on_error_handler_inner.lock().map(|h| h.call(e));
											}
										}
									},
									Err(ref e) => {
										let _ = on_error_handler_inner.lock().map(|h| h.call(e));
									}
								};
							}).map(|_| {
								()
							}).map_err(|_| {
								EventHandlerError::Fail(
									String::from("An error occurred while starting the user thread."))
							})
						},
						Err(_) => {
							Err(EventHandlerError::Fail(
									String::from("Could not get exclusive lock on thread queue object.")))
						}
					}
				},
				ref e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		});

		let busy = busy_arc.clone();
		let user_event_queue = user_event_queue_arc.clone();
		let allow_immediate_move = allow_immediate_move_arc.clone();
		let on_delay_move_handler = on_delay_move_handler_arc.clone();
		let on_error_handler = on_error_handler_arc.clone();
		let in_ponder = in_ponder_arc.clone();
		let think_start_time = think_start_time_arc.clone();

		system_event_dispatcher.add_handler(SystemEventKind::Stop, move |ctx,e| {
			match e {
				&SystemEvent::Stop => {
					if in_ponder.load(Ordering::Acquire) {
						match think_start_time.lock() {
							Ok(mut think_start_time) => {
								*think_start_time = Some(Instant::now());
							},
							Err(_) => {
								return Err(EventHandlerError::Fail(String::from(
									"Could not get exclusive lock on think_start_time object"
								)));
							}
						}
					}

					in_ponder.store(false,Ordering::Release);

					if busy.load(Ordering::Acquire) {
						match user_event_queue.lock() {
							Ok(mut user_event_queue) => {
								user_event_queue.push(UserEvent::Stop);
							},
							Err(_) => {
								return Err(EventHandlerError::Fail(String::from(
									"Could not get exclusive lock on user event queue object."
								)));
							}
						}
					}

					allow_immediate_move.store(true,Ordering::Release);

					match on_delay_move_handler.lock().or(Err(EventHandlerError::Fail(String::from(
						 "Could not get exclusive lock on on ponder handler object."
					))))? {
						mut g => {
							match *g {
								ref mut n @ OnAcceptMove::Some(_) => {
									let system_event_queue = ctx.system_event_queue.clone();
									n.notify(&system_event_queue,&on_error_handler);
								},
								OnAcceptMove::None => (),
							};
							*g = OnAcceptMove::None;
						}
					};
					Ok(())
				},
				e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		});

		let user_event_queue = user_event_queue_arc.clone();
		let allow_immediate_move = allow_immediate_move_arc.clone();
		let on_delay_move_handler = on_delay_move_handler_arc.clone();
		let on_error_handler = on_error_handler_arc.clone();
		let in_ponder = in_ponder_arc.clone();

		system_event_dispatcher.add_handler(SystemEventKind::PonderHit, move |ctx,e| {
			match e {
				&SystemEvent::PonderHit => {
					in_ponder.store(false,Ordering::Release);

					match user_event_queue.lock() {
						Ok(mut user_event_queue) => {
							user_event_queue.push(UserEvent::PonderHit(Instant::now()));
						},
						Err(_) => {
							return Err(EventHandlerError::Fail(String::from(
								"Could not get exclusive lock on user event queue object."
							)));
						}
					}

					allow_immediate_move.store(true,Ordering::Release);

					match on_delay_move_handler.lock().or(Err(EventHandlerError::Fail(String::from(
						 "Could not get exclusive lock on on ponder handler object."
					))))? {
						mut g => {
							match *g {
								ref mut n @ OnAcceptMove::Some(_) => {
									let system_event_queue = ctx.system_event_queue.clone();
									n.notify(&system_event_queue,&on_error_handler);
								},
								OnAcceptMove::None => (),
							};
							*g = OnAcceptMove::None;
						}
					};
					Ok(())
				},
				e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		});

		let on_error_handler = on_error_handler_arc.clone();
		let busy = busy_arc.clone();
		let user_event_queue = user_event_queue_arc.clone();
		let thread_queue = thread_queue_arc.clone();

		system_event_dispatcher.add_handler(SystemEventKind::Quit, move |ctx,e| {
			match e {
				&SystemEvent::Quit => {
					let system_event_queue = ctx.system_event_queue.clone();
					let on_error_handler_inner = on_error_handler.clone();
					let player = ctx.player.clone();
					let user_event_queue_inner = user_event_queue.clone();

					if busy.load(Ordering::Acquire) {
						match user_event_queue_inner.lock() {
							Ok(mut user_event_queue) => {
								user_event_queue.push(UserEvent::Quit);
							},
							Err(ref e) => {
								let _ = on_error_handler_inner.lock().map(|h| h.call(e));
							}
						}
					}

					match thread_queue.lock() {
						Ok(mut thread_queue) => {
							thread_queue.submit(move || {
								match player.lock() {
									Ok(mut player) => {
										match player.quit() {
											Ok(_) => (),
											Err(ref e) => {
												let _ = on_error_handler_inner.lock().map(|h| h.call(e));
											}
										}
									},
									Err(ref e) => {
										let _ = on_error_handler_inner.lock().map(|h| h.call(e));
									}
								};
								match system_event_queue.lock() {
									Ok(mut system_event_queue) => {
										system_event_queue.push(SystemEvent::QuitReady);
									},
									Err(ref e) => {
										let _ = on_error_handler_inner.lock().map(|h| h.call(e));
									}
								};
							}).map(|_| {
								()
							}).map_err(|_| {
								EventHandlerError::Fail(
									String::from("An error occurred while starting the user thread."))
							})
						},
						Err(_) => {
							Err(EventHandlerError::Fail(
									String::from("Could not get exclusive lock on thread queue object.")))
						}
					}
				},
				e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		});

		let on_error_handler = on_error_handler_arc.clone();
		let busy = busy_arc.clone();
		let user_event_queue = user_event_queue_arc.clone();

		let thread_queue = thread_queue_arc.clone();

		system_event_dispatcher.add_handler(SystemEventKind::GameOver, move |ctx,e| {
			match *e {
				SystemEvent::GameOver(ref s) => {
					let player = ctx.player.clone();
					let on_error_handler_inner = on_error_handler.clone();
					let s = s.clone();
					let user_event_queue_inner = user_event_queue.clone();

					if busy.load(Ordering::Acquire) {
						match user_event_queue.lock() {
							Ok(mut user_event_queue) => {
								user_event_queue.push(UserEvent::Stop);
							},
							Err(_) => {
								return Err(EventHandlerError::Fail(String::from(
									"Could not get exclusive lock on user event queue object."
								)));
							}
						}
					}

					match thread_queue.lock() {
						Ok(mut thread_queue) => {
							thread_queue.submit(move || {
								match player.lock() {
									Ok(mut player) => {
										match player.gameover(&s,user_event_queue_inner.clone(),
																		on_error_handler_inner.clone()) {
											Ok(_) => (),
											Err(ref e) => {
												let _ = on_error_handler_inner.lock().map(|h| h.call(e));
											}
										}
									},
									Err(ref e) => {
										let _ = on_error_handler_inner.lock().map(|h| h.call(e));
									}
								};
							}).map(|_| {
								()
							}).map_err(|_| {
								EventHandlerError::Fail(
									String::from("An error occurred while starting the user thread."))
							})
						},
						Err(_) => {
							Err(EventHandlerError::Fail(
									String::from("Could not get exclusive lock on thread queue object.")))
						}
					}
				},
				ref e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		});

		let quit_ready = quit_ready_arc.clone();

		system_event_dispatcher.add_handler(SystemEventKind::QuitReady, move |_,e| {
			match e {
				&SystemEvent::QuitReady => {
					quit_ready.store(true,Ordering::Release);
					Ok(())
				},
				e => Err(EventHandlerError::InvalidState(e.event_kind())),
			}
		});

		let interpreter = USIInterpreter::new();

		let logger = logger_arc.clone();
		let on_error_handler = on_error_handler_arc.clone();

		let player = self.player.clone();

		let system_event_queue = system_event_queue_arc.clone();

		player.lock().map(|mut player| {
			let option_kinds = match player.get_option_kinds() {
				Ok(option_kinds) => option_kinds,
				Err(ref e) => {
					let _ = on_error_handler.lock().map(|h| h.call(e));
					return false;
				}
			};
			interpreter.start(system_event_queue,reader,option_kinds,&logger);
			true
		}).or_else(|e| {
			on_error_handler.lock().map(|h| h.call(&e))
		}).or(Err(USIAgentStartupError::MutexLockFailedOtherError(
					String::from("Failed to acquire exclusive lock of player object."))))?;
		let system_event_queue = system_event_queue_arc.clone();

		let delay = time::Duration::from_millis(50);

		let quit_ready = quit_ready_arc.clone();

		let on_error_handler = on_error_handler_arc.clone();

		while !quit_ready.load(Ordering::Acquire) {
			match system_event_dispatcher.dispatch_events(self, &*system_event_queue) {
				Ok(_) => true,
				Err(ref e) => {
					on_error_handler.lock().map(|h| h.call(e)).is_err()
				}
			};
			thread::sleep(delay);
		}

		Ok(())
	}
}