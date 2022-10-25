//! AIの本体を実装するためのtrait等
use std::{thread,time};
use std::time::Instant;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::error::Error;
use std::convert::TryFrom;

use std::sync::mpsc::{Sender,Receiver};

use command::*;
use error::*;
use event::*;
use shogi::*;
use protocol::*;
use rule::*;
use output::*;
use Logger;
use OnErrorHandler;
use crossbeam_channel::{unbounded, after};

/// プレイヤー（AI本体）の実装
pub trait USIPlayer<E>: fmt::Debug where E: PlayerError {
	/// このAIの名前
	const ID: &'static str;
	/// このAIの作者
	const AUTHOR: &'static str;
	/// サポートしているオブションの一覧をオプション名をキーとしたマップで返す
	fn get_option_kinds(&mut self) -> Result<BTreeMap<String,SysEventOptionKind>,E>;
	/// サポートしているオプションに関する設定情報（maxとminの値など）をオプション名をキーとしたマップで返す
	fn get_options(&mut self) -> Result<BTreeMap<String,UsiOptType>,E>;
	/// プレイヤーの機能で必要な時間のかかる前処理などをここで行う
	/// # Arguments
	/// * `on_keep_alive` - 定期的にGUIに空行を送信してエンジンがフリーズしていないことを通知するためのオブジェクト
	fn take_ready<W,L>(&mut self, on_keep_alive:OnKeepAlive<W,L>)
		-> Result<(),E> where W: USIOutputWriter + Send + 'static, L: Logger + Send + 'static;
	/// オプションを設定する
	/// # Arguments
	/// * `name` - オプションの名前
	/// * `value` - オプションの値
	fn set_option(&mut self,name:String,value:SysEventOption) -> Result<(),E>;
	/// ゲーム開始前の処理。対局ごとに毎回呼ばれる
	fn newgame(&mut self) -> Result<(),E>;
	/// 局面の初期化。毎回初期局面と現在の局面までの全ての指し手のリストが送られてくる。
	/// # Arguments
	/// * `teban` - 初期局面時の手番
	/// * `ban` - 盤面
	/// * `ms` - 先手の持ち駒
	/// * `mg` - 後手の持ち駒
	/// * `n` - 次の手が何手目か。（USIプロトコルのSFENの原案にあるために存在するが、現在固定で1が送られてくるため無視してかまわない）
	/// * `m` - 指し手のリスト
	fn set_position(&mut self,teban:Teban,ban:Banmen,ms:Mochigoma,mg:Mochigoma,n:u32,m:Vec<Move>)
		-> Result<(),E>;
	/// 思考開始。この関数の戻り値が指し手となる。AIの実装の核となる部分
	/// # Arguments
	/// * `think_start_time` - 思考開始時の時間。通常は現在の時刻だが、go ponderの後に予想した指し手が外れた場合などはstopコマンドを受け取った時刻となる。
	/// * `limit` - 持ち時間
	/// * `event_queue` - ユーザーイベントが格納されているキュー。stopコマンドを受信した時やgo ponderの指し手が当たった時,エンジンの終了時などに送られてくる。
	/// * `info_sender` - infoコマンドを送信するためのオブジェクト。
	/// * `pinfo_sender` - あらかじめスケジュールされた一定の間隔でinfoコマンドを送信するための機能を持つオブジェクト
	/// * `on_error_handler` - エラーをログファイルなどに出力するためのオブジェクト
	fn think<L,S,P>(&mut self,think_start_time:Instant,limit:&UsiGoTimeLimit,event_queue:Arc<Mutex<UserEventQueue>>,
			info_sender:S,periodically_info:P,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<BestMove,E> where L: Logger + Send + 'static,
										S: InfoSender,
										P: PeriodicallyInfo,;
	/// 思考開始。この関数の戻り値が指し手となる。AIの実装の核となる部分
	/// # Arguments
	/// * `limit` - 持ち時間
	/// * `event_queue` - ユーザーイベントが格納されているキュー。stopコマンドを受信した時やgo ponderの指し手が当たった時,エンジンの終了時などに送られてくる。
	/// * `info_sender` - infoコマンドを送信するためのオブジェクト。
	/// * `pinfo_sender` - あらかじめスケジュールされた一定の間隔でinfoコマンドを送信するための機能を持つオブジェクト
	/// * `on_error_handler` - エラーをログファイルなどに出力するためのオブジェクト
	fn think_ponder<L,S,P>(&mut self,limit:&UsiGoTimeLimit,event_queue:Arc<Mutex<UserEventQueue>>,
			info_sender:S,periodically_info:P,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<BestMove,E> where L: Logger + Send + 'static,
										S: InfoSender + Send + 'static,
										P: PeriodicallyInfo;
	/// 詰め将棋回答時に呼ばれる関数
	/// # Arguments
	/// * `limit` - 持ち時間
	/// * `event_queue` - ユーザーイベントが格納されているキュー。stopコマンドを受信した時やgo ponderの指し手が当たった時,エンジンの終了時などに送られてくる。
	/// * `info_sender` - infoコマンドを送信するためのオブジェクト。
	/// * `pinfo_sender` - あらかじめスケジュールされた一定の間隔でinfoコマンドを送信するための機能を持つオブジェクト
	/// * `on_error_handler` - エラーをログファイルなどに出力するためのオブジェクト
	fn think_mate<L,S,P>(&mut self,limit:&UsiGoMateTimeLimit,event_queue:Arc<Mutex<UserEventQueue>>,
			info_sender:S,periodically_info:P,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
			-> Result<CheckMate,E> where L: Logger + Send + 'static,
										 S: InfoSender,
										 P: PeriodicallyInfo;
	/// `UserEvent::Stop`イベントがキューに追加されている状態で`dispatch_events`でイベントを処理すると呼ばれる。
	fn on_stop(&mut self,e:&UserEvent) -> Result<(), E> where E: PlayerError;
	/// `UserEvent::PonderHit`イベントがキューに追加されている状態で`dispatch_events`でイベントを処理すると呼ばれる。
	fn on_ponderhit(&mut self,e:&UserEvent) -> Result<(), E> where E: PlayerError;
	/// 対局終了時に呼ばれる
	/// # Arguments
	/// * `s` - 勝敗を表すオブジェクト
	/// * `event_queue` - ユーザーイベントが格納されているキュー。stopコマンドを受信した時やgo ponderの指し手が当たった時,エンジンの終了時などに送られてくる。
	/// * `on_error_handler` - エラーをログファイルなどに出力するためのオブジェクト
	fn gameover<L>(&mut self,s:&GameEndState,
			event_queue:Arc<Mutex<UserEventQueue>>,
			on_error_handler:Arc<Mutex<OnErrorHandler<L>>>) -> Result<(),E> where L: Logger, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static;
	/// `UserEvent::Quit`イベントがキューに追加されている状態で`dispatch_events`でイベントを処理すると呼ばれる。
	fn on_quit(&mut self,e:&UserEvent) -> Result<(), E> where E: PlayerError;
	/// 終了時に呼ばれる関数
	fn quit(&mut self) -> Result<(),E>;
	/// イベントを処理する関数。これにイベントキューを渡すか`EventDispatcher`を実装したオブジェクトの`dispatch_events`にイベントキューを渡すまでイベントは処理されない。
	/// # Arguments
	/// * `event_queue` - ユーザーイベントが格納されているキュー。stopコマンドを受信した時やgo ponderの指し手が当たった時,エンジンの終了時などに送られてくる。
	/// * `on_error_handler` - エラーをログファイルなどに出力するためのオブジェクト
	fn handle_events<'a,L>(&mut self,event_queue:&'a Mutex<UserEventQueue>,
						on_error_handler:&Mutex<OnErrorHandler<L>>) -> Result<bool,E>
						where L: Logger, E: Error + fmt::Debug,
								Arc<Mutex<OnErrorHandler<L>>>: Send + 'static,
								EventHandlerError<UserEventKind,E>: From<E> {
		Ok(match self.dispatch_events(event_queue,&on_error_handler) {
			Ok(_)=> true,
			Err(ref e) => {
				let _ = on_error_handler.lock().map(|h| h.call(e));
				false
			}
		})
	}

	/// `USIPlayer::handle_events`から呼ばれる内部関数。イベントキュー内のイベントを処理する。
	/// # Arguments
	/// * `event_queue` - ユーザーイベントが格納されているキュー。stopコマンドを受信した時やgo ponderの指し手が当たった時,エンジンの終了時などに送られてくる。
	/// * `on_error_handler` - エラーをログファイルなどに出力するためのオブジェクト
	fn dispatch_events<'a,L>(&mut self, event_queue:&'a Mutex<UserEventQueue>,
						on_error_handler:&Mutex<OnErrorHandler<L>>) ->
						Result<(), EventDispatchError<'a,UserEventQueue,UserEvent,E>>
							where L: Logger, E: Error + fmt::Debug,
									Arc<Mutex<OnErrorHandler<L>>>: Send + 'static,
									EventHandlerError<UserEventKind,E>: From<E> {
		let events = {
			event_queue.lock()?.drain_events()
		};

		let mut has_error = false;

		for e in &events {
			match e {
				&UserEvent::Stop => {
					match self.on_stop(e) {
						Ok(_) => (),
						Err(ref e) => {
							let _ = on_error_handler.lock().map(|h| h.call(e));
							has_error = true;
						}
					};
				},
				&UserEvent::PonderHit(_) => {
					match self.on_ponderhit(e) {
						Ok(_) => (),
						Err(ref e) => {
							let _ = on_error_handler.lock().map(|h| h.call(e));
							has_error = true;
						}
					};
				},
				&UserEvent::Quit => {
					match self.on_quit(e) {
						Ok(_) => (),
						Err(ref e) => {
							let _ = on_error_handler.lock().map(|h| h.call(e));
							has_error = true;
						}
					};
				}
			};
		}

		match has_error {
			true => Err(EventDispatchError::ContainError),
			false => Ok(()),
		}
	}

	/// 手のリストを現在の局面に適用した結果を返す
	/// # Arguments
	/// * `state` - 手の列挙に使うビットボードと盤面などの内部状態を持つオブジェクト
	/// * `teban` - 局面開始時の手番
	/// * `mc` - 局面開始時の持ち駒
	/// * `m` - 開始局面から現在までの指し手のリスト
	/// * `r` - コールバック関数に渡され関数の戻り値の一部となるオブジェクト(任意の型)
	/// * `f` - 手の適用のたびに呼ばれるコールバック関数
	fn apply_moves<T,F>(&self,mut state:State,
						mut teban:Teban,
						mut mc:MochigomaCollections,
						m:&Vec<AppliedMove>,mut r:T,mut f:F)
		-> (Teban,State,MochigomaCollections,T)
		where F: FnMut(&Self,Teban,&Banmen,
						&MochigomaCollections,&Option<AppliedMove>,
						&Option<MochigomaKind>,T) -> T {

		for m in m {
			match Rule::apply_move_none_check(&state,teban,&mc,*m) {
				(next,nmc,o) => {
					r = f(self,teban,&state.get_banmen(),&mc,&Some(*m),&o,r);
					state = next;
					mc = nmc;
					teban = teban.opposite();
				}
			}
		}
		r = f(self,teban,&state.get_banmen(),&mc,&None,&None,r);

		(teban,state,mc,r)
	}
}
/// infoコマンドの発行スレッドに対してコマンドの出力、スレッドの停止などを通知するためのメッセージオブジェクト
#[derive(Clone, Debug)]
pub enum UsiInfoMessage {
	/// infoコマンドのサブコマンドのリスト
	Commands(Vec<UsiInfoSubCommand>),
	/// infoコマンド発行スレッドを停止させる
	Quit,
}
/// infoコマンドを出力する
pub trait InfoSender: Clone + Send + 'static {
	/// infoコマンドを出力する
	///
	/// # Arguments
	/// * `commands` - infoサブコマンドのリスト
	fn send(&mut self,commands:Vec<UsiInfoSubCommand>) -> Result<(), InfoSendError>;
	/// infoコマンドを遅延なしで出力する
	///
	/// # Arguments
	/// * `commands` - infoサブコマンドのリスト
	fn send_immediate(&mut self,commands:Vec<UsiInfoSubCommand>) -> Result<(), InfoSendError>;
}
/// infoコマンドを標準出力へ出力する`InfoSender`の実装
pub struct USIInfoSender<W> where W: USIOutputWriter + Send + 'static {
	sender:Sender<UsiInfoMessage>,
	writer:Arc<Mutex<W>>
}
impl<W> USIInfoSender<W> where W: USIOutputWriter + Send + 'static {
	/// `USIInfoSender`の生成
	///
	/// # Arguments
	/// * `sender` - infoコマンド出力スレッドへ通知するためのSender
	/// * `writer` - USIコマンドを出力するためのオブジェクト。実装によって標準出力以外へ書き込むものを指定することも可能。
	pub fn new(sender:Sender<UsiInfoMessage>,writer:Arc<Mutex<W>>) -> USIInfoSender<W> {
		USIInfoSender {
			sender:sender,
			writer:writer
		}
	}

	pub(crate) fn start_worker_thread<L>(&self,thinking:Arc<AtomicBool>,
		receiver:Receiver<UsiInfoMessage>,
		writer:Arc<Mutex<W>>,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>)
		where W: USIOutputWriter, L: Logger, Arc<Mutex<W>>: Send + 'static, Arc<Mutex<OnErrorHandler<L>>>: Send + 'static {

		thinking.store(true,Ordering::Release);

		thread::spawn(move || {
			loop {
				if !thinking.load(Ordering::Acquire) {
					break;
				}

				match receiver.recv() {
					Ok(UsiInfoMessage::Commands(commands)) => {
						match UsiOutput::try_from(&UsiCommand::UsiInfo(commands)) {
							Ok(UsiOutput::Command(ref s)) => {
								match writer.lock() {
									Err(ref e) => {
										let _ = on_error_handler.lock().map(|h| h.call(e));
										break;
									},
									Ok(ref writer) => {
										let s = writer.write(s).is_err();
										thread::sleep(time::Duration::from_millis(10));
										s
									}
								};
							},
							Err(ref e) => {
								let _ = on_error_handler.lock().map(|h| h.call(e));
								break;
							}
						}
					},
					Ok(UsiInfoMessage::Quit) => {
						break;
					},
					Err(ref e) => {
						let _ = on_error_handler.lock().map(|h| h.call(e));
						break;
					}
				}
			}
		});
	}
}
impl<W> InfoSender for USIInfoSender<W> where W: USIOutputWriter + Send + 'static {
	fn send(&mut self,commands:Vec<UsiInfoSubCommand>) -> Result<(), InfoSendError> {
		if let Err(_) = self.sender.send(UsiInfoMessage::Commands(commands)) {
			Err(InfoSendError::Fail(String::from(
				"info command send failed.")))
		} else {
			Ok(())
		}
	}
	
	fn send_immediate(&mut self, commands: Vec<UsiInfoSubCommand>) -> Result<(), InfoSendError> {
		let lines = vec![commands.to_usi_command()?];

		match self.writer.lock() {
			Ok(writer) => {
				if let Err(_) =  writer.write(&lines) {
					return Err(InfoSendError::Fail(String::from(
						"info command send failed.")))
				}
			},
			Err(_) => {
				return Err(InfoSendError::Fail(String::from(
					"Failed to secure exclusive lock on writer in the process of sending info command.")))
			}
		}

		Ok(())
	}
}
impl<W> Clone for USIInfoSender<W> where W: USIOutputWriter + Send + 'static {
	fn clone(&self) -> USIInfoSender<W> {
		USIInfoSender::new(self.sender.clone(),Arc::clone(&self.writer))
	}
}
/// コンソールへ出力する`InfoSender`の実装（出力用に別にスレッドを持ってはおらず呼び出し時に直接出力する）
pub struct ConsoleInfoSender {
	writer:USIStdOutputWriter,
	silent:bool,
}
impl ConsoleInfoSender {
	/// `ConsoleInfoSender`の生成
	///
	/// # Arguments
	/// * `silent` - infoコマンドを出力するか否かのフラグ。`true`の場合、出力しない
	pub fn new(silent:bool) -> ConsoleInfoSender {
		ConsoleInfoSender {
			writer:USIStdOutputWriter::new(),
			silent:silent
		}
	}
}
impl InfoSender for ConsoleInfoSender {
	fn send(&mut self,commands:Vec<UsiInfoSubCommand>) -> Result<(), InfoSendError> {
		if !self.silent {
			let lines = vec![commands.to_usi_command()?];

			if let Err(_) =  self.writer.write(&lines) {
				return Err(InfoSendError::Fail(String::from(
					"info command send failed.")))
			}
		}
		Ok(())
	}

	fn send_immediate(&mut self, commands: Vec<UsiInfoSubCommand>) -> Result<(), InfoSendError> {
		self.send(commands)
	}
}
impl Clone for ConsoleInfoSender {
	fn clone(&self) -> ConsoleInfoSender {
		ConsoleInfoSender::new(self.silent)
	}
}
/// 初期化処理時にKeepAliveとして空行を送信する
pub trait KeepAliveSender {
	/// 空行を送信する
	fn send(&self);
	#[must_use]
	/// Dropされる前の間指定された間隔（単位は秒）で空行を送信するオブジェクトを生成する
	///
	/// # Arguments
	/// * `sec` - 空行を送信する間隔（単位は秒単位）
	///
	/// note: このメソッドから返された値を_から始まる任意の名前の変数に格納して、
	/// 　　　　KeepAliveの送信が必要なくなるタイミングまで保持してください。
	///       このオブジェクトがスコープを抜けてdropされた時点でKeepAlive送信スレッドには停止要求が投げられ、
	///       その後KeepAlive送信処理は終了します。
	fn auto(&self,sec:u64) -> AutoKeepAlive;
}
/// `KeepAliveSender`の実装
pub struct OnKeepAlive<W,L> where W: USIOutputWriter + Send + 'static, L: Logger + Send + 'static {
	/// * `writer` - USIコマンドを出力するためのオブジェクト。実装によって標準出力以外へ書き込むものを指定することも可能。
	writer:Arc<Mutex<W>>,
	/// * `on_error_handler` - エラーをログファイルなどに出力するためのオブジェクト
	on_error_handler:Arc<Mutex<OnErrorHandler<L>>>
}
impl<W,L> OnKeepAlive<W,L> where W: USIOutputWriter + Send + 'static, L: Logger + Send + 'static {
	/// `OnKeepAlive`の生成
	///
	/// # Arguments
	/// * `writer` - USIコマンドを出力するためのオブジェクト。実装によって標準出力以外へ書き込むものを指定することも可能。
	/// * `on_error_handler` - エラーをログファイルなどに出力するためのオブジェクト
	pub fn new(writer:Arc<Mutex<W>>,on_error_handler:Arc<Mutex<OnErrorHandler<L>>>) -> OnKeepAlive<W,L> {
		OnKeepAlive {
			writer:writer,
			on_error_handler:on_error_handler
		}
	}
}
impl<W,L> KeepAliveSender for OnKeepAlive<W,L> where W: USIOutputWriter + Send + 'static, L: Logger + Send + 'static {
	fn send(&self) {
		match self.writer.lock() {
			Err(ref e) => {
				let _ = self.on_error_handler.lock().map(|h| h.call(e));
			},
			Ok(ref writer) => {
				if let Err(ref e) = writer.write(&vec![String::from("")]) {
					let _ = self.on_error_handler.lock().map(|h| h.call(e));
				}
			}
		};
	}

	#[must_use]
	fn auto(&self,sec:u64) -> AutoKeepAlive {
		AutoKeepAlive::new(sec,self.clone())
	}
}
impl<W,L> Clone for OnKeepAlive<W,L> where W: USIOutputWriter + Send + 'static, L: Logger + Send + 'static {
	fn clone(&self) -> OnKeepAlive<W,L> {
		OnKeepAlive {
			writer:self.writer.clone(),
			on_error_handler:self.on_error_handler.clone()
		}
	}
}
/// KeepAliveの送信を指定された間隔で定期的に行う
///
/// note: このオブジェクトは`KeepAliveSender`を実装した型の`auto`メソッドから返されますが、
///       必ず返された値を_から始まる任意の名前の変数に格納して、KeepAliveの送信が必要なくなるタイミングまで保持してください。
///       このオブジェクトがスコープを抜けてdropされた時点でKeepAlive送信スレッドには停止要求が投げられ、
///       その後KeepAlive送信処理は終了します。
pub struct AutoKeepAlive {
	/// Drop時に送信スレッドに停止メッセージを送るためのSender
	stop_sender:crossbeam_channel::Sender<()>
}
impl AutoKeepAlive {
	/// `AutoKeepAlive`の生成
	///
	/// # Arguments
	/// * `sec` - KeepAlive送信の間隔（単位は秒単位）
	/// * `on_keep_alive` - KeepAlive送信用オブジェクト
	fn new<W,L>(sec:u64,on_keep_alive: OnKeepAlive<W,L>)
		-> AutoKeepAlive where W: USIOutputWriter + Send + 'static, L: Logger + Send + 'static {
		let(s,r) = unbounded();

		std::thread::spawn(move || {
			let mut timeout = after(time::Duration::from_secs(sec));

			loop {
				select! {
					recv(r) -> _ => {
						return;
					},
					recv(timeout) -> _ => {
						on_keep_alive.send();
						timeout = after(time::Duration::from_secs(sec));
					}
				}
			}
		});

		AutoKeepAlive {
			stop_sender:s
		}
	}
}
impl Drop for AutoKeepAlive {
	fn drop(&mut self) {
		let _ = self.stop_sender.send(());
	}
}
/// 一定時間ごとに定期的に送信するinfoコマンドの送信用
/// これ自体はコマンドの送信を行わない。dropされたタイミングで送信用スレッドを止める役割を担う
///
/// note: このオブジェクトは`PeriodicallyInfo`を実装した型の`start`メソッドから返されますが、
///       必ず返された値を_から始まる任意の名前の変数に格納して、
///       定期的伊送信するinfoコマンドの送信が必要なくなるタイミングまで保持してください。
///       このオブジェクトがスコープを抜けてdropされた時点で定期的に送信するinfoコマンド送信スレッドには停止要求が投げられ、
///       その後送信処理は終了します。
pub struct PeriodicallyInfoSender {
	/// * `stop_sender` Drop時に送信スレッドに停止メッセージを送るためのSender
	stop_sender:crossbeam_channel::Sender<()>,
}
impl PeriodicallyInfoSender {
	/// * `stop_sender` Drop時に送信スレッドに停止メッセージを送るためのSender
	pub fn new(stop_sender:crossbeam_channel::Sender<()>) -> PeriodicallyInfoSender {
		PeriodicallyInfoSender {
			stop_sender:stop_sender
		}
	}
}
impl Drop for PeriodicallyInfoSender {
	fn drop(&mut self) {
		let _ = self.stop_sender.send(());
	}
}
/// 一定時間ごとに定期的に送信するinfoコマンドの送信用
pub trait PeriodicallyInfo: Send + 'static {
	#[must_use]
	/// 送信するコマンドの生成用コールバックの登録と共に送信開始
	///
	/// # Arguments
	/// * `interval` - infoコマンド送信の間隔（単位はミリ秒））
	/// * `info_generator` - `UsiInfoSubCommand`のリストを返すジェネレータ。定期的に呼びdされ返されたコマンドを僧院する。
	/// * `on_error_handler` - エラーをログファイルなどに出力するためのオブジェクト
	///
	/// note: このメソッドから返された値を_から始まる任意の名前の変数に格納して、
	///       定期的伊送信するinfoコマンドの送信が必要なくなるタイミングまで保持してください。
	///       このオブジェクトがスコープを抜けてdropされた時点で定期的に送信するinfoコマンド送信スレッドには停止要求が投げられ、
	///       その後送信処理は終了します。
	fn start<F,L>(self,interval:u64,info_generator:F,on_error_handler:&Arc<Mutex<OnErrorHandler<L>>>)
		-> PeriodicallyInfoSender where F: FnMut() -> Vec<UsiInfoSubCommand> + Sized + Send + 'static,
			  L: Logger + Send + 'static;
}
pub struct USIPeriodicallyInfo<W> where W: USIOutputWriter + Send + 'static {
	/// * `writer` - USIコマンドを出力するためのオブジェクト。実装によって標準出力以外へ書き込むものを指定することも可能。
	writer:Arc<Mutex<W>>,
	/// * `silent` - infoコマンドを出力するか否かのフラグ。`true`の場合、出力しない
	silent:bool
}
impl<W> USIPeriodicallyInfo<W>
	where W: USIOutputWriter + Send + 'static {
	/// `USIPeriodicallyInfoSender`の生成
	///
	/// # Arguments
	/// * `writer` - USIコマンドを出力するためのオブジェクト。実装によって標準出力以外へ書き込むものを指定することも可能。
	/// * `silent` - infoコマンドを出力するか否かのフラグ。`true`の場合、出力しない
	pub fn new(writer:Arc<Mutex<W>>,silent:bool) -> USIPeriodicallyInfo<W> {
		USIPeriodicallyInfo {
			writer:writer,
			silent:silent
		}
	}
}
impl<W> PeriodicallyInfo for USIPeriodicallyInfo<W>
	where W: USIOutputWriter + Send + 'static {

	#[must_use]
	fn start<F,L>(self,interval:u64,info_generator:F,on_error_handler:&Arc<Mutex<OnErrorHandler<L>>>)
		-> PeriodicallyInfoSender where F: FnMut() -> Vec<UsiInfoSubCommand> + Send + 'static,
			  L: Logger + Send + 'static {
		let (s,r) = unbounded();

		let writer = self.writer.clone();
		let mut info_generator = info_generator;
		let silent = self.silent;
		let on_error_handler = on_error_handler.clone();

		std::thread::spawn(move || {
			let mut timeout = after(time::Duration::from_millis(interval));

			loop {
				select! {
					recv(r) -> _ => {
						return;
					},
					recv(timeout) -> _ => {
						match UsiOutput::try_from(&UsiCommand::UsiInfo(info_generator())) {
							Ok(UsiOutput::Command(ref s)) => {
								match writer.lock() {
									Err(ref e) => {
										let _ = on_error_handler.lock().map(|h| h.call(e));
										break;
									},
									Ok(ref writer) => {
										if !silent {
											let _ = writer.write(s).map_err(|e| on_error_handler.lock().map(|h| h.call(&e)));
										}
									}
								};
							},
							Err(ref e) => {
								let _ = on_error_handler.lock().map(|h| h.call(e));
								break;
							}
						}
						timeout = after(time::Duration::from_millis(interval));
					}
				}
			}
		});

		PeriodicallyInfoSender::new(s)
	}
}
impl<W> Clone for USIPeriodicallyInfo<W> where W: USIOutputWriter + Send + 'static {
	fn clone(&self) -> USIPeriodicallyInfo<W> {
		USIPeriodicallyInfo {
			writer:self.writer.clone(),
			silent:self.silent
		}
	}
}
pub struct ConsolePeriodicallyInfo {
	inner:USIPeriodicallyInfo<USIStdOutputWriter>
}
impl ConsolePeriodicallyInfo {
	/// `ConsolePeriodicallyInfoSender`の生成
	///
	/// # Arguments
	/// * `silent` - infoコマンドを出力するか否かのフラグ。`true`の場合、出力しない
	pub fn new(silent:bool) -> ConsolePeriodicallyInfo {
		ConsolePeriodicallyInfo {
			inner:USIPeriodicallyInfo::new(
				Arc::new(Mutex::new(USIStdOutputWriter::new())),
					  silent
			)
		}
	}
}
impl PeriodicallyInfo for ConsolePeriodicallyInfo {
	#[must_use]
	fn start<F,L>(self,interval:u64,info_generator:F,on_error_handler:&Arc<Mutex<OnErrorHandler<L>>>)
		-> PeriodicallyInfoSender where F: FnMut() -> Vec<UsiInfoSubCommand> + Send + 'static,
											  L: Logger + Send + 'static {
		self.inner.start(interval,info_generator,on_error_handler)
	}
}
impl Clone for ConsolePeriodicallyInfo {
	fn clone(&self) -> ConsolePeriodicallyInfo {
		ConsolePeriodicallyInfo {
			inner:self.inner.clone()
		}
	}
}