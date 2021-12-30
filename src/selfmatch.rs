//! 自己対局機能
use chrono::prelude::*;

use std::fmt;
use std::{thread};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread::JoinHandle;
use std::marker::Send;
use std::marker::PhantomData;
use std::time::{Instant,Duration};
use std::collections::HashMap;
use std::io::Write;
use std::io::BufWriter;
use std::fs;
use std::fs::OpenOptions;

use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;
use crossbeam_channel::SendError;
use crossbeam_channel::after;
use crossbeam_channel::never;

use command::*;
use event::*;
use error::*;
use input::*;
use output::*;
use player::*;
use shogi::*;
use hash::*;
use Logger;
use logger::FileLogger;
use OnErrorHandler;
use TryFrom;
use SandBox;
use rule::*;
use protocol::*;

/// 棋譜を記録する
pub trait SelfMatchKifuWriter {
	/// 棋譜の書き込みを行う
	///
	/// # Arguments
	/// * `initial_sfen` - 開始時の局面のsfen文字列表現
	/// * `m` - 開始局面からの指し手のリスト
	fn write(&mut self,initial_sfen:&String,m:&Vec<Move>) -> Result<(),KifuWriteError>;
	/// 開始時の局面のsfen文字列と`Vec<Move>`から棋譜のsfen文字列を生成するメソッドのデフォルト実装
	///
	/// # Arguments
	/// * `initial_sfen` - 開始時の局面のsfen文字列表現
	/// * `m` - 開始局面からの指し手のリスト
	fn to_sfen(&self,initial_sfen:&String,m:&Vec<Move>)
		-> Result<String, SfenStringConvertError> {

		let sfen = initial_sfen.split(" ").collect::<Vec<&str>>();

		if sfen.len() >= 5 {
			match (sfen[0],sfen[1],sfen[2],sfen[3],sfen[4]) {
				("sfen",p1,p2,p3,p4) if m.len() > 0 => {
					Ok(format!("sfen {} {} {} {} moves {}",p1,p2,p3,p4,m.to_sfen()?))
				},
				("sfen",p1,p2,p3,p4) => {
					Ok(format!("sfen {} {} {} {}",p1,p2,p3,p4))
				},
				("startpos",_,_,_,_) if m.len() > 0 => {
					Ok(format!("startpos moves {}",m.to_sfen()?))
				},
				("startpos",_,_,_,_)=> {
					Ok(format!("startpos"))
				},
				_ => {
					Err(SfenStringConvertError::InvalidFormat(initial_sfen.clone()))
				}
			}
		} else if sfen.len() >= 1 && sfen[0] == "startpos" {
			if m.len() > 0 {
				Ok(format!("startpos moves {}",m.to_sfen()?))
			} else {
				Ok(format!("startpos"))
			}
		} else {
			Err(SfenStringConvertError::InvalidFormat(initial_sfen.clone()))
		}
	}
}
/// ファイルに記録する`SelfMatchKifuWriter`の実装
#[derive(Debug)]
pub struct FileSfenKifuWriter {
	writer:BufWriter<fs::File>,
}
impl FileSfenKifuWriter {
	/// FileSfenKifuWriterの生成
	///
	/// # Arguments
	/// * `file` - 書き込み先ファイル
	pub fn new(file:String) -> Result<FileSfenKifuWriter,KifuWriteError> {
		Ok(FileSfenKifuWriter {
			writer:BufWriter::new(OpenOptions::new().append(true).create(true).open(file)?),
		})
	}
}
impl SelfMatchKifuWriter for FileSfenKifuWriter {
	/// ファイルに棋譜を書き込む
	///
	/// # Arguments
	/// * `initial_sfen` - 開始時の局面のsfen文字列表現
	/// * `m` - 開始局面からの指し手のリスト
	fn write(&mut self,initial_sfen:&String,m:&Vec<Move>) -> Result<(),KifuWriteError> {
		let sfen = self.to_sfen(initial_sfen,m)?;

		let _ = self.writer.write(format!("{}\n",sfen).as_bytes())?;
		Ok(())
	}
}
/// タイムアウトの種別
#[derive(Debug)]
enum TimeoutKind {
	/// タイムアウト無し
	Never,
	/// 現在のターンのタイムアウト
	Turn,
	/// 自己対局機能の起動時に指定した終了時刻に達した
	Uptime,
}
/// 自己対局機能の実装内でやり取りするメッセージオブジェクト
#[derive(Debug)]
pub enum SelfMatchMessage {
	/// 準備完了
	Ready,
	/// ゲーム開始
	GameStart,
	/// プレイヤーの思考を開始する
	StartThink(Teban,Banmen,MochigomaCollections,u32,Vec<AppliedMove>,Instant),
	/// プレイヤーの思考を開始する（go ponder)
	StartPonderThink(Teban,Banmen,MochigomaCollections,u32,Vec<AppliedMove>),
	/// プレイヤーから指し手を返す
	NotifyMove(BestMove),
	/// ponderで予測した指し手と一致した
	PonderHit,
	/// ponderで予測した指し手と一致しない
	PonderNG,
	/// 対局終了
	GameEnd(GameEndState),
	/// 中断
	Abort,
	/// 自己対局終了
	Quit,
	/// エラー発生を通知
	Error(usize),
}
/// 自己対局の結果
#[derive(Debug)]
pub struct SelfMatchResult {
	/// 実施した対局回数
	pub game_count:u32,
	/// 自己対局開始からの経過時間
	pub elapsed:Duration,
	/// 自己対局の開始時間
	pub start_dt:DateTime<Local>,
	/// 自己対局の終了時間
	pub end_dt:DateTime<Local>,
}
/// 自己対局エンジン
#[derive(Debug)]
pub struct SelfMatchEngine<E>
	where 	E: PlayerError {
	player_error_type:PhantomData<E>,
	/// システムイベントキュー
	pub system_event_queue:Arc<Mutex<SystemEventQueue>>,
}
impl<E> SelfMatchEngine<E>
	where E: PlayerError {
	/// `SelfMatchEngine`の生成
	pub fn new() -> SelfMatchEngine<E> where E: PlayerError {
		SelfMatchEngine {
			player_error_type:PhantomData::<E>,
			system_event_queue:Arc::new(Mutex::new(EventQueue::new())),
		}
	}

	/// デフォルト設定で開始（ログファイルのパスlogs/log.txt,ログをファイルに記録）
	///
	/// # Arguments
	/// * `on_init_event_dispatcher` - 自己対局時に通知されるSelfMatchEventのイベントディスパッチャーを初期化
	/// * `flip_players` - 対局時の初期局面時のplayer1とplayer2の手番の割り当てを逆にする。(通常はplayer1が先手)
	/// * `initial_position_creator` - 対局毎の初期局面を生成して返す関数
	/// * `kifu_writer` - 対局終了時に棋譜を書き込むためのコールバック関数
	/// * `input_handler` - 標準入力から読みこんだ行が渡されるコールバック関数。システムイベントの発行などに使う（'quit'で終了など）
	/// * `player1` - USIPlayerを実装したプレイヤーオブジェクト
	/// * `player2` - USIPlayerを実装したプレイヤーオブジェクト
	/// * `player1_options` - player1に渡されるオプション
	/// * `player2_options` - player2に渡されるオプション
	/// * `info_sender` - infoコマンドを送信するための機能を持つオブジェクト
	/// * `pinfo_sender` - あらかじめスケジュールされた一定の間隔でinfoコマンドを送信するための機能を持つオブジェクト
	/// * `game_time_limit` - 対局毎の制限時間
	/// * `uptime` - 自己対局機能全体の実行時間制限。この時間に達すると自己対局は終了する（現在の対局だけではない）
	/// * `number_of_games` - 自己対局機能で行われる対局の回数。この回数を終えると自己対局は終了する
	/// * `on_error` - エラー発生時に呼ばれるコールバック関数。エラーオブジェクトへの参照とロガーが渡される。
	pub fn start_default<T,S,P,I,F,RH,EH>(&mut self, on_init_event_dispatcher:I,
						flip_players:F,
						initial_position_creator:Option<Box<dyn FnMut() -> String + Send + 'static>>,
						kifu_writer:Option<Box<dyn FnMut(&String,&Vec<Move>) -> Result<(),KifuWriteError>  +Send + 'static>>,
						input_handler:RH,
						player1:T,
						player2:T,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						info_sender:S,
						pinfo_sender:P,
						game_time_limit:UsiGoTimeLimit,
						uptime:Option<Duration>,
						number_of_games:Option<u32>,
						on_error:EH) -> Result<SelfMatchResult,SelfMatchRunningError<E>>
		where T: USIPlayer<E> + fmt::Debug + Send + 'static,
				F: FnMut() -> bool + Send + 'static,
				RH: FnMut(String) -> Result<bool,SelfMatchRunningError<E>> + Send + 'static,
				I: FnMut(&mut SelfMatchEventDispatcher<E,FileLogger>),
				S: InfoSender,
				P: PeriodicallyInfo + Clone + Send + 'static,
				Arc<Mutex<FileLogger>>: Send + 'static,
				EH: FnMut(Option<Arc<Mutex<OnErrorHandler<FileLogger>>>>,
					&SelfMatchRunningError<E>) {
		self.start_with_log_path(String::from("logs/log.txt"),
								on_init_event_dispatcher,
								flip_players,
								initial_position_creator,
								kifu_writer, input_handler,
								player1,player2,
								player1_options, player2_options,
								info_sender,
								pinfo_sender,
								game_time_limit,
								uptime,
								number_of_games,
								on_error)
	}

	/// ログファイルのパスを指定して開始
	///
	/// # Arguments
	/// * `path` - ログファイルのパス
	/// * `on_init_event_dispatcher` - 自己対局時に通知されるSelfMatchEventのイベントディスパッチャーを初期化
	/// * `flip_players` - 対局時の初期局面時のplayer1とplayer2の手番の割り当てを逆にする。(通常はplayer1が先手)
	/// * `initial_position_creator` - 対局毎の初期局面を生成して返す関数
	/// * `kifu_writer` - 対局終了時に棋譜を書き込むためのコールバック関数
	/// * `input_handler` - 標準入力から読みこんだ行が渡されるコールバック関数。システムイベントの発行などに使う（'quit'で終了など）
	/// * `player1` - USIPlayerを実装したプレイヤーオブジェクト
	/// * `player2` - USIPlayerを実装したプレイヤーオブジェクト
	/// * `player1_options` - player1に渡されるオプション
	/// * `player2_options` - player2に渡されるオプション
	/// * `info_sender` - infoコマンドを送信するための機能を持つオブジェクト
	/// * `pinfo_sender` - あらかじめスケジュールされた一定の間隔でinfoコマンドを送信するための機能を持つオブジェクト
	/// * `game_time_limit` - 対局毎の制限時間
	/// * `uptime` - 自己対局機能全体の実行時間制限。この時間に達すると自己対局は終了する（現在の対局だけではない）
	/// * `number_of_games` - 自己対局機能で行われる対局の回数。この回数を終えると自己対局は終了する
	/// * `on_error` - エラー発生時に呼ばれるコールバック関数。エラーオブジェクトへの参照とロガーが渡される。
	pub fn start_with_log_path<T,S,P,I,F,RH,EH>(&mut self,path:String,
						on_init_event_dispatcher:I,
						flip_players:F,
						initial_position_creator:Option<Box<dyn FnMut() -> String + Send + 'static>>,
						kifu_writer:Option<Box<dyn FnMut(&String,&Vec<Move>) -> Result<(),KifuWriteError>  +Send + 'static>>,
						input_handler:RH,
						player1:T,
						player2:T,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						info_sender:S,
						pinfo_sender:P,
						game_time_limit:UsiGoTimeLimit,
						uptime:Option<Duration>,
						number_of_games:Option<u32>,
						mut on_error:EH) -> Result<SelfMatchResult,SelfMatchRunningError<E>>
		where T: USIPlayer<E> + fmt::Debug + Send + 'static,
				F: FnMut() -> bool + Send + 'static,
				RH: FnMut(String) -> Result<bool,SelfMatchRunningError<E>> + Send + 'static,
				I: FnMut(&mut SelfMatchEventDispatcher<E,FileLogger>),
				S: InfoSender,
				P: PeriodicallyInfo + Clone + Send + 'static,
				Arc<Mutex<FileLogger>>: Send + 'static,
				EH: FnMut(Option<Arc<Mutex<OnErrorHandler<FileLogger>>>>,
					&SelfMatchRunningError<E>) {
		let logger = match FileLogger::new(path) {
			Err(e) => {
				let e = SelfMatchRunningError::IOError(e);
				on_error(None,&e);
				return Err(e);
			},
			Ok(logger) => logger,
		};

		let input_reader = USIStdInputReader::new();

		self.start(on_init_event_dispatcher,
					flip_players,
					initial_position_creator,
					kifu_writer, input_reader, input_handler,
					player1,player2,
					player1_options, player2_options,
					info_sender,
					pinfo_sender,
					game_time_limit,
					uptime,
					number_of_games,
					logger, on_error)
	}

	/// `Logger`,`USIInputReader`を指定して開始
	///
	/// # Arguments
	/// * `on_init_event_dispatcher` - 自己対局時に通知されるSelfMatchEventのイベントディスパッチャーを初期化
	/// * `flip_players` - 対局時の初期局面時のplayer1とplayer2の手番の割り当てを逆にする。(通常はplayer1が先手)
	/// * `initial_position_creator` - 対局毎の初期局面を生成して返す関数
	/// * `kifu_writer` - 対局終了時に棋譜を書き込むためのコールバック関数
	/// * `input_reader` - 入力を読み取るためのオブジェクト。実装によって標準入力以外から読み取るものを指定することも可能。
	/// * `input_handler` - 標準入力から読みこんだ行が渡されるコールバック関数。システムイベントの発行などに使う（'quit'で終了など）
	/// * `player1` - USIPlayerを実装したプレイヤーオブジェクト
	/// * `player2` - USIPlayerを実装したプレイヤーオブジェクト
	/// * `player1_options` - player1に渡されるオプション
	/// * `player2_options` - player2に渡されるオプション
	/// * `info_sender` - infoコマンドを送信するための機能を持つオブジェクト
	/// * `pinfo_sender` - あらかじめスケジュールされた一定の間隔でinfoコマンドを送信するための機能を持つオブジェクト
	/// * `game_time_limit` - 対局毎の制限時間
	/// * `uptime` - 自己対局機能全体の実行時間制限。この時間に達すると自己対局は終了する（現在の対局だけではない）
	/// * `number_of_games` - 自己対局機能で行われる対局の回数。この回数を終えると自己対局は終了する
	/// * `logger` - ログを書き込むためのオブジェクト。実装によってファイル以外に書き込むものを指定することも可能。
	/// * `on_error` - エラー発生時に呼ばれるコールバック関数。エラーオブジェクトへの参照とロガーが渡される。
	pub fn start<T,S,P,I,F,R,RH,L,EH>(&mut self, on_init_event_dispatcher:I,
						flip_players:F,
						initial_position_creator:Option<Box<dyn FnMut() -> String + Send + 'static>>,
						kifu_writer:Option<Box<dyn FnMut(&String,&Vec<Move>) -> Result<(),KifuWriteError>  +Send + 'static>>,
						input_reader:R,
						input_handler:RH,
						player1:T,
						player2:T,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						info_sender:S,
						pinfo_sender:P,
						game_time_limit:UsiGoTimeLimit,
						uptime:Option<Duration>,
						number_of_games:Option<u32>,
						logger:L, mut on_error:EH) -> Result<SelfMatchResult,SelfMatchRunningError<E>>
		where T: USIPlayer<E> + fmt::Debug + Send + 'static,
				F: FnMut() -> bool + Send + 'static,
				R: USIInputReader + Send + 'static,
				RH: FnMut(String) -> Result<bool,SelfMatchRunningError<E>> + Send + 'static,
				I: FnMut(&mut SelfMatchEventDispatcher<E,L>),
				S: InfoSender,
				P: PeriodicallyInfo + Clone + Send + 'static,
				L: Logger + fmt::Debug + Send + 'static,
				Arc<Mutex<L>>: Send + 'static,
				EH: FnMut(Option<Arc<Mutex<OnErrorHandler<L>>>>,
					&SelfMatchRunningError<E>) {
		let logger_arc = Arc::new(Mutex::new(logger));
		let on_error_handler_arc = Arc::new(Mutex::new(OnErrorHandler::new(logger_arc.clone())));
		let on_error_handler = on_error_handler_arc.clone();

		let r = self.run(on_init_event_dispatcher,
							flip_players,
							initial_position_creator,
							kifu_writer, input_reader, input_handler,
							player1,player2,
							player1_options, player2_options,
							info_sender,
							pinfo_sender,
							game_time_limit,
							uptime,
							number_of_games,
							logger_arc, on_error_handler_arc);

		if let Err(ref e) = r {
			on_error(Some(on_error_handler),e);
		}

		r
	}

	fn run<T,S,P,I,F,R,RH,L>(&mut self, mut on_init_event_dispatcher:I,
						mut flip_players:F,
						initial_position_creator:Option<Box<dyn FnMut() -> String + Send + 'static>>,
						kifu_writer:Option<Box<dyn FnMut(&String,&Vec<Move>) -> Result<(),KifuWriteError> + Send + 'static>>,
						mut input_reader:R,
						mut input_handler:RH,
						mut player1:T,
						mut player2:T,
						player1_options:Vec<(String,SysEventOption)>,
						player2_options:Vec<(String,SysEventOption)>,
						info_sender:S,
						pinfo_sender:P,
						game_time_limit:UsiGoTimeLimit,
						uptime:Option<Duration>,
						number_of_games:Option<u32>,
						logger_arc:Arc<Mutex<L>>,
						on_error_handler_arc:Arc<Mutex<OnErrorHandler<L>>>) -> Result<SelfMatchResult,SelfMatchRunningError<E>>
		where T: USIPlayer<E> + fmt::Debug + Send + 'static,
				F: FnMut() -> bool + Send + 'static,
				R: USIInputReader + Send + 'static,
				RH: FnMut(String) -> Result<bool,SelfMatchRunningError<E>> + Send + 'static,
				I: FnMut(&mut SelfMatchEventDispatcher<E,L>),
				S: InfoSender,
				P: PeriodicallyInfo + Clone + Send + 'static,
				L: Logger + fmt::Debug + Send + 'static,
				Arc<Mutex<L>>: Send + 'static {
		let start_time = Instant::now();
		let start_dt = Local::now();

		let mut self_match_event_dispatcher:SelfMatchEventDispatcher<E,L> = USIEventDispatcher::new(&on_error_handler_arc);

		on_init_event_dispatcher(&mut self_match_event_dispatcher);

		let mut system_event_dispatcher:SystemEventDispatcher<SelfMatchEngine<E>,E,L> = USIEventDispatcher::new(&on_error_handler_arc);

		let user_event_queue_arc:[Arc<Mutex<UserEventQueue>>; 2] = [Arc::new(Mutex::new(EventQueue::new())),Arc::new(Mutex::new(EventQueue::new()))];

		let user_event_queue = [user_event_queue_arc[0].clone(),user_event_queue_arc[1].clone()];

		let mut initial_position_creator:Box<dyn FnMut() -> String + Send + 'static> =
			initial_position_creator.map_or(Box::new(|| String::from("startpos")), |f| {
				f
			});

		let on_error_handler = on_error_handler_arc.clone();

		let mut kifu_writer = kifu_writer;
		let mut kifu_writer = move |sfen:&String,m:&Vec<Move>| {
			let _ = kifu_writer.as_mut().map(|w| {
				let _= w(sfen,m).map_err(|e| on_error_handler.lock().map(|h| h.call(&e)));
			});
		};

		let quit_ready_arc = Arc::new(AtomicBool::new(false));
		let on_error_handler = on_error_handler_arc.clone();

		let self_match_event_queue:SelfMatchEventQueue = EventQueue::new();
		let self_match_event_queue_arc = Arc::new(Mutex::new(self_match_event_queue));

		let (ss,sr) = unbounded();
		let (cs1,cr1) = unbounded();
		let (cs2,cr2) = unbounded();
		let mut cr = vec![cr1,cr2];

		{
			let ss = ss.clone();
			let quit_ready = quit_ready_arc.clone();

			let on_error_handler = on_error_handler_arc.clone();

			system_event_dispatcher.add_handler(SystemEventKind::Quit, move |_,e| {
				match e {
					&SystemEvent::Quit => {
						for i in 0..2 {
							match user_event_queue[i].lock() {
								Ok(mut user_event_queue) => {
									user_event_queue.push(UserEvent::Quit);
								},
								Err(ref e) => {
									let _ = on_error_handler.lock().map(|h| h.call(e));
								}
							};
						};

						if !quit_ready.load(Ordering::Acquire) {
							if let Err(ref e) = ss.send(SelfMatchMessage::Quit) {
								let _ = on_error_handler.lock().map(|h| h.call(e));
							}
						}

						Ok(())
					},
					e => Err(EventHandlerError::InvalidState(e.event_kind())),
				}
			});
		}

		for (k,v) in player1_options {
			match player1.set_option(k,v) {
				Ok(()) => (),
				Err(ref e) => {
					let _ = on_error_handler.lock().map(|h| h.call(e));
					return Err(SelfMatchRunningError::Fail(String::from(
						"An error occurred while executing a self match. Please see the log for details ..."
					)));
				}
			}
		}

		for (k,v) in player2_options {
			match player2.set_option(k,v) {
				Ok(()) => (),
				Err(ref e) => {
					let _ = on_error_handler.lock().map(|h| h.call(e));
					return Err(SelfMatchRunningError::Fail(String::from(
						"An error occurred while executing a self match. Please see the log for details ..."
					)));
				}
			}
		}

		let position_parser = PositionParser::new();

		let self_match_event_queue = self_match_event_queue_arc.clone();
		let quit_ready = quit_ready_arc.clone();

		let on_error_handler = on_error_handler_arc.clone();

		let user_event_queue = user_event_queue_arc.clone();

		let bridge_h = thread::spawn(move || SandBox::immediate(|| {
			let cs = [cs1.clone(),cs2.clone()];
			let mut prev_move:Option<AppliedMove> = None;
			let mut ponders:[Option<AppliedMove>; 2] = [None,None];

			let quit_ready_inner = quit_ready.clone();

			let quit_notification =  move || {
				quit_ready_inner.store(true,Ordering::Release);
			};

			let self_match_event_queue_inner = self_match_event_queue.clone();
			let on_error_handler_inner = on_error_handler.clone();

			let quit_ready_inner = quit_ready.clone();

			let on_gameend = move |win_cs:Sender<SelfMatchMessage>,
									lose_cs:Sender<SelfMatchMessage>,
									_:[Sender<SelfMatchMessage>; 2],
									sr:&Receiver<SelfMatchMessage>,
									s:SelfMatchGameEndState| {
				let mut message_state = GameEndState::Win;

				let quit_notification = || {
					quit_ready_inner.store(true,Ordering::Release);
				};

				match self_match_event_queue_inner.lock() {
					Ok(mut self_match_event_queue) => {
						self_match_event_queue.push(SelfMatchEvent::GameEnd(s));
					},
					Err(ref e) => {
						let _ = on_error_handler_inner.lock().map(|h| h.call(e));
						return Err(SelfMatchRunningError::InvalidState(String::from(
							"Exclusive lock on self_match_event_queue failed."
						)));
					}
				}

				for current_cs in &[win_cs.clone(),lose_cs.clone()] {
					current_cs.send(SelfMatchMessage::GameEnd(message_state))?;
					match sr.recv()? {
						SelfMatchMessage::Ready => (),
						SelfMatchMessage::Error(n) => {
							return Err(SelfMatchRunningError::PlayerThreadError(n));
						},
						SelfMatchMessage::Quit => {
							quit_notification();

							return Ok(());
						},
						_ => {
							return Err(SelfMatchRunningError::InvalidState(String::from(
								"An invalid message was sent to the self-match management thread."
							)));
						}
					}
					message_state = GameEndState::Lose;
				}
				Ok(())
			};

			let mut game_count = 0;

			'gameloop: while !quit_ready.load(Ordering::Acquire) &&
				number_of_games.map_or(true, |n| game_count < n) &&
				uptime.map_or(true, |t| Instant::now() - start_time < t) {

				cs[0].send(SelfMatchMessage::GameStart)?;
				cs[1].send(SelfMatchMessage::GameStart)?;

				game_count += 1;

				let mut cs_index = if flip_players() {
					1
				} else {
					0
				};

				let sfen = initial_position_creator();
				let (teban, banmen, mc, n, mvs) = match position_parser.parse(&sfen.split(" ").collect::<Vec<&str>>()) {
					Ok(position) => {
						position.extract()
					},
					Err(ref e) => {
						let _ = on_error_handler.lock().map(|h| h.call(e));
						return Err(SelfMatchRunningError::InvalidState(String::from(
							"An error occurred parsing the sfen string."
						)));
					}
				};

				if teban == Teban::Gote {
					cs_index = (cs_index + 1) % 2;
				}

				let banmen_at_start = banmen.clone();
				let mc_at_start = mc.clone();
				let teban_at_start = teban.clone();

				let mut current_game_time_limit = [game_time_limit,game_time_limit];
				let mut current_time_limit = current_game_time_limit[cs_index].to_instant(teban,Instant::now());

				let kyokumen_map:KyokumenMap<u64,u32> = KyokumenMap::new();
				let oute_kyokumen_map:KyokumenMap<u64,u32> = KyokumenMap::new();

				let hasher = KyokumenHash::new();

				let (ms,mg) = match mc {
					MochigomaCollections::Pair(ref ms, ref mg) => {
						match teban {
							Teban::Sente => (ms.clone(),mg.clone()),
							Teban::Gote => (mg.clone(),ms.clone()),
						}
					},
					MochigomaCollections::Empty => {
						(HashMap::new(),HashMap::new())
					},
				};

				let (mhash, shash) = hasher.calc_initial_hash(&banmen,&ms,&mg);

				let mut mvs = mvs.into_iter().map(|m| m.to_applied_move()).collect::<Vec<AppliedMove>>();

				let (mut teban,
					 mut state,
					 mut mc,
					 mut mhash,
					 mut shash,
					 mut kyokumen_map,
					 mut oute_kyokumen_map) = Rule::apply_moves(State::new(banmen),
															 	teban,mc,&mvs,
															 	mhash,shash,
															 	kyokumen_map,
															 	oute_kyokumen_map,&hasher);

				if teban != teban_at_start {
					cs_index = (cs_index + 1) % 2;
				}

				match self_match_event_queue.lock() {
					Ok(mut self_match_event_queue) => {
						self_match_event_queue.push(
							SelfMatchEvent::GameStart(if cs_index == 1 {
								2
							} else {
								1
							}, teban, sfen.clone()));
					},
					Err(ref e) => {
						let _ = on_error_handler.lock().map(|h| h.call(e));
						return Err(SelfMatchRunningError::InvalidState(String::from(
							"Exclusive lock on self_match_event_queue failed."
						)));
					}
				}

				while uptime.map_or(true, |t| Instant::now() - start_time < t) {
					match user_event_queue[cs_index].lock() {
						Ok(mut user_event_queue) => {
							user_event_queue.clear();
						},
						Err(ref e) => {
							let _ = on_error_handler.lock().map(|h| h.call(e));
						}
					}

					match ponders[cs_index] {
						None => {
							let _ = cs[cs_index].send(SelfMatchMessage::StartThink(
								teban_at_start.clone(),banmen_at_start.clone(),mc_at_start.clone(),n,mvs.clone(),Instant::now()));
						},
						pm @ Some(_) if pm == prev_move => {
							match user_event_queue[cs_index].lock() {
								Ok(mut user_event_queue) => {
									user_event_queue.push(UserEvent::PonderHit(Instant::now()));
								},
								Err(ref e) => {
									let _ = on_error_handler.lock().map(|h| h.call(e));
								}
							}
							let _ = cs[cs_index].send(SelfMatchMessage::PonderHit);
						},
						_ => {
							match user_event_queue[cs_index].lock() {
								Ok(mut user_event_queue) => {
									user_event_queue.push(UserEvent::Stop);
								},
								Err(ref e) => {
									let _ = on_error_handler.lock().map(|h| h.call(e));
								}
							}
							let _ = cs[cs_index].send(SelfMatchMessage::PonderNG);
							let _ = cs[cs_index].send(SelfMatchMessage::StartThink(
								teban_at_start.clone(),banmen_at_start.clone(),mc_at_start.clone(),n,mvs.clone(),Instant::now()));
						}
					}

					let think_start_time = Instant::now();

					let timeout = current_time_limit.map(|cl| uptime.map(|u| {
						if start_time + u < cl {
							start_time + u - Instant::now()
						} else {
							cl - Instant::now()
						}
					}).unwrap_or(cl - Instant::now()))
						.map(|d| after(d))
						.unwrap_or_else(|| uptime.map(|u| after(start_time + u - Instant::now()))
						.unwrap_or(never()));

					let timeout_kind = current_time_limit.map(|cl| uptime.map(|u| {
						if start_time + u < cl {
							TimeoutKind::Uptime
						} else {
							TimeoutKind::Turn
						}
					}).unwrap_or(TimeoutKind::Turn))
						.unwrap_or_else(|| uptime.map(|_| TimeoutKind::Uptime).unwrap_or(TimeoutKind::Never));

					select! {
						recv(sr) -> message => {
							match message? {
								SelfMatchMessage::NotifyMove(BestMove::Move(m,pm)) => {
									match self_match_event_queue.lock() {
										Ok(mut self_match_event_queue) => {
											self_match_event_queue.push(SelfMatchEvent::Moved(teban,Moved::try_from((&state.get_banmen(),&m))?));
										},
										Err(ref e) => {
											let _ = on_error_handler.lock().map(|h| h.call(e));
											return Err(SelfMatchRunningError::InvalidState(String::from(
												"Exclusive lock on self_match_event_queue failed."
											)));
										}
									}

									current_game_time_limit[cs_index] = Rule::update_time_limit(
										&current_game_time_limit[cs_index],
										teban,think_start_time.elapsed()
									);
									current_time_limit = current_game_time_limit[cs_index].to_instant(teban,Instant::now());

									let m = m.to_applied_move();

									match Rule::apply_valid_move(&state,teban,&mc,m) {
										Ok((next,nmc,o)) => {

											let is_win = Rule::is_win(&state,teban,m);

											if is_win {
												mvs.push(m);

												kifu_writer(&sfen,&mvs.into_iter()
																		.map(|m| m.to_move())
																		.collect::<Vec<Move>>());
												on_gameend(
													cs[cs_index].clone(),
													cs[(cs_index+1) % 2].clone(),
													[cs[0].clone(),cs[1].clone()],
													&sr,
													SelfMatchGameEndState::Win(teban)
												)?;
												break;
											}

											if Rule::is_mate(teban.opposite(),&state) {
												if Rule::is_mate(teban.opposite(),&next) {
													mvs.push(m);
													kifu_writer(&sfen,&mvs.into_iter()
																			.map(|m| m.to_move())
																			.collect::<Vec<Move>>());
													on_gameend(
														cs[(cs_index+1) % 2].clone(),
														cs[cs_index].clone(),
														[cs[0].clone(),cs[1].clone()],
														&sr,
														SelfMatchGameEndState::Foul(teban,FoulKind::NotRespondedOute)
													)?;
													break;
												}
											} else {
												if Rule::is_mate(teban.opposite(),&next) {
													mvs.push(m);
													kifu_writer(&sfen,&mvs.into_iter()
																			.map(|m| m.to_move())
																			.collect::<Vec<Move>>());
													on_gameend(
														cs[(cs_index+1) % 2].clone(),
														cs[cs_index].clone(),
														[cs[0].clone(),cs[1].clone()],
														&sr,
														SelfMatchGameEndState::Foul(teban,FoulKind::Suicide)
													)?;
													break;
												}
											}

											mvs.push(m);

											mhash = hasher.calc_main_hash(mhash,teban,&state.get_banmen(),&mc,m,&o);
											shash = hasher.calc_sub_hash(shash,teban,&state.get_banmen(),&mc,m,&o);

											mc = nmc;
											state = next;

											if Rule::is_put_fu_and_mate(&state,teban,&mc,m) {
												kifu_writer(&sfen,&mvs.into_iter()
																				.map(|m| m.to_move())
																				.collect::<Vec<Move>>());
												on_gameend(
													cs[(cs_index+1) % 2].clone(),
													cs[cs_index].clone(),
													[cs[0].clone(),cs[1].clone()],
													&sr,
													SelfMatchGameEndState::Foul(teban,FoulKind::PutFuAndMate)
												)?;
												break;
											}

											if Rule::is_sennichite_by_oute(
												&state,
												teban,mhash,shash,
												&oute_kyokumen_map
											) {
												kifu_writer(&sfen,&mvs.into_iter()
																		.map(|m| m.to_move())
																		.collect::<Vec<Move>>());
												on_gameend(
													cs[(cs_index+1) % 2].clone(),
													cs[cs_index].clone(),
													[cs[0].clone(),cs[1].clone()],
													&sr,
													SelfMatchGameEndState::Foul(teban,FoulKind::SennichiteOu)
												)?;
												break;
											}

											Rule::update_sennichite_by_oute_map(
												&state,
												teban,mhash,shash,
												&mut oute_kyokumen_map
											);

											if Rule::is_sennichite(
												&state,teban,mhash,shash,&kyokumen_map
											) {
												kifu_writer(&sfen,&mvs.into_iter()
																		.map(|m| m.to_move())
																		.collect::<Vec<Move>>());
												on_gameend(
													cs[(cs_index+1) % 2].clone(),
													cs[cs_index].clone(),
													[cs[0].clone(),cs[1].clone()],
													&sr,
													SelfMatchGameEndState::Foul(teban,FoulKind::Sennichite)
												)?;
												break;
											}

											Rule::update_sennichite_map(
												&state,teban,mhash,shash,&mut kyokumen_map
											);

											teban = teban.opposite();

											ponders[cs_index] = pm.map(|pm| pm.to_applied_move());

											match pm {
												Some(pm) => {
													match mvs.clone() {
														mut mvs => {
															mvs.push(pm.to_applied_move());
															cs[cs_index].send(
																SelfMatchMessage::StartPonderThink(
																	teban_at_start.clone(),banmen_at_start.clone(),
																	mc_at_start.clone(),n,mvs))?;
														}
													}
												},
												None => (),
											}

											cs_index = (cs_index + 1) % 2;
										},
										Err(_) => {
											mvs.push(m);
											kifu_writer(&sfen,&mvs.into_iter()
																	.map(|m| m.to_move())
																	.collect::<Vec<Move>>());
											on_gameend(
												cs[(cs_index+1) % 2].clone(),
												cs[cs_index].clone(),
												[cs[0].clone(),cs[1].clone()],
												&sr,
												SelfMatchGameEndState::Foul(teban,FoulKind::InvalidMove)
											)?;
											break;
										}
									}
									prev_move = Some(m)
								},
								SelfMatchMessage::NotifyMove(BestMove::Resign) => {
									kifu_writer(&sfen,&mvs.into_iter()
															.map(|m| m.to_move())
															.collect::<Vec<Move>>());
									on_gameend(
										cs[(cs_index+1) % 2].clone(),
										cs[cs_index].clone(),
										[cs[0].clone(),cs[1].clone()],
										&sr,
										SelfMatchGameEndState::Resign(teban)
									)?;
									break;
								},
								SelfMatchMessage::NotifyMove(BestMove::Abort) => {
									match self_match_event_queue.lock() {
										Ok(mut self_match_event_queue) => {
											self_match_event_queue.push(SelfMatchEvent::Abort);
											cs[0].send(SelfMatchMessage::Abort)?;
											cs[1].send(SelfMatchMessage::Abort)?;
										},
										Err(ref e) => {
											let _ = on_error_handler.lock().map(|h| h.call(e));
											return Err(SelfMatchRunningError::InvalidState(String::from(
												"Exclusive lock on self_match_event_queue failed."
											)));
										}
									}
									break;
								},
								SelfMatchMessage::NotifyMove(BestMove::Win) if Rule::is_nyugyoku_win(&state,teban,&mc,&current_time_limit)=> {
									kifu_writer(&sfen,&mvs.into_iter()
															.map(|m| m.to_move())
															.collect::<Vec<Move>>());
									on_gameend(
										cs[cs_index].clone(),
										cs[(cs_index+1) % 2].clone(),
										[cs[0].clone(),cs[1].clone()],
										&sr,
										SelfMatchGameEndState::NyuGyokuWin(teban)
									)?;
									break;
								},
								SelfMatchMessage::NotifyMove(BestMove::Win) => {
									kifu_writer(&sfen,&mvs.into_iter()
															.map(|m| m.to_move())
															.collect::<Vec<Move>>());
									on_gameend(
										cs[(cs_index+1) % 2].clone(),
										cs[cs_index].clone(),
										[cs[0].clone(),cs[1].clone()],
										&sr,
										SelfMatchGameEndState::NyuGyokuLose(teban)
									)?;
									break;
								},
								SelfMatchMessage::Error(n) => {
									return Err(SelfMatchRunningError::PlayerThreadError(n));
								},
								SelfMatchMessage::Quit => {
									quit_notification();

									cs[0].send(SelfMatchMessage::Quit)?;
									cs[1].send(SelfMatchMessage::Quit)?;

									return Ok(SelfMatchResult {
										game_count: game_count,
										elapsed: start_time.elapsed(),
										start_dt:start_dt,
										end_dt:Local::now(),
									});
								},
								_ => {
									return Err(SelfMatchRunningError::InvalidState(String::from(
										"An invalid message was sent to the self-match management thread."
									)));
								}
							}
						},
						recv(timeout) -> message => {
							match message? {
								_ => {
									match user_event_queue[cs_index].lock() {
										Ok(mut user_event_queue) => {
											user_event_queue.push(UserEvent::Stop);
										},
										Err(ref e) => {
											let _ = on_error_handler.lock().map(|h| h.call(e));
										}
									}

									match timeout_kind {
										TimeoutKind::Turn => {
											kifu_writer(&sfen,&mvs.into_iter().map(|m| m.to_move()).collect::<Vec<Move>>());
											match sr.recv()? {
												SelfMatchMessage::NotifyMove(_) => {
													on_gameend(
													cs[(cs_index+1) % 2].clone(),
													cs[cs_index].clone(),
													[cs[0].clone(),cs[1].clone()],
													&sr,
													SelfMatchGameEndState::Timeover(teban))?;

												},
												_ => {
													return Err(SelfMatchRunningError::InvalidState(String::from(
														"An invalid message was sent to the self-match management thread."
													)));
												}
											}

											break;
										},
										TimeoutKind::Uptime => {
											match sr.recv()? {
												SelfMatchMessage::NotifyMove(_) => {
													break 'gameloop;
												},
												_ => {
													return Err(SelfMatchRunningError::InvalidState(String::from(
														"An invalid message was sent to the self-match management thread."
													)));
												}
											}
										},
										_ => {
											return Err(SelfMatchRunningError::InvalidState(String::from(
												"Timeout kind is invalid."
											)));
										}
									}
								}
							}
						}
					}
				}
			}
			quit_notification();

			cs[0].send(SelfMatchMessage::Quit)?;
			cs[1].send(SelfMatchMessage::Quit)?;

			Ok(SelfMatchResult {
				game_count: game_count,
				elapsed: start_time.elapsed(),
				start_dt:start_dt,
				end_dt:Local::now()
			})
		}, on_error_handler.clone()).map_err(|e| {
			match e {
				SelfMatchRunningError::SendError(SendError(SelfMatchMessage::Error(n))) => {
					let r = if n == 0 {
						cs2.send(SelfMatchMessage::Error(0))
					} else {
						cs1.send(SelfMatchMessage::Error(1))
					};
					if let Err(ref e) = r {
						let _ = on_error_handler.lock().map(|h| h.call(e));
					}
				},
				SelfMatchRunningError::PlayerThreadError(0) => {
					if let Err(ref e) = cs2.send(SelfMatchMessage::Error(0)) {
						let _ = on_error_handler.lock().map(|h| h.call(e));
					}
				},
				SelfMatchRunningError::PlayerThreadError(1) => {
					if let Err(ref e) = cs1.send(SelfMatchMessage::Error(1)) {
						let _ = on_error_handler.lock().map(|h| h.call(e));
					}
				},
				_ => {
					if let Err(ref e) = cs1.send(SelfMatchMessage::Error(0)) {
						let _ = on_error_handler.lock().map(|h| h.call(e));
					}
					if let Err(ref e) = cs2.send(SelfMatchMessage::Error(1)) {
						let _ = on_error_handler.lock().map(|h| h.call(e));
					}
				}
			}
			quit_ready.store(true,Ordering::Release);
			e
		}));

		let mut players = vec![player1,player2];
		let mut handlers:Vec<JoinHandle<Result<(),SelfMatchRunningError<E>>>> = Vec::with_capacity(2);

		for i in 0..2 {
			let cr = cr.remove(0);
			let mut player = players.remove(0);
			let on_error_handler = on_error_handler_arc.clone();
			let logger = logger_arc.clone();
			let user_event_queue = [user_event_queue_arc[0].clone(),user_event_queue_arc[1].clone()];
			let quit_ready = quit_ready_arc.clone();
			let info_sender = info_sender.clone();
			let pinfo_sender = pinfo_sender.clone();
			let limit = game_time_limit;

			let ss = ss.clone();

			let player_i = i;

			handlers.push(thread::spawn(move || SandBox::immediate(|| {
				loop {
					match cr.recv()? {
						SelfMatchMessage::GameStart => {
							let writer = Arc::new(Mutex::new(VoidOutPutWriter));

							player.take_ready(OnKeepAlive::new(writer,on_error_handler.clone()))?;
							player.newgame()?;

							loop {
								match cr.recv()? {
									SelfMatchMessage::StartThink(t,b,mc,n,m,s) => {
										let (ms, mg) = match mc {
											MochigomaCollections::Pair(ref ms, ref mg) => {
												(ms.clone(),mg.clone())
											},
											MochigomaCollections::Empty => {
												(HashMap::new(),HashMap::new())
											}
										};

										player.set_position(t, b, ms, mg, n, m.into_iter().map(|m| {
											m.to_move()
										}).collect::<Vec<Move>>())?;

										let m = player.think(s,&limit,
															user_event_queue[player_i].clone(),
															info_sender.clone(),
															 pinfo_sender.clone(),
															 on_error_handler.clone())?;

										if !quit_ready.load(Ordering::Acquire) {
											ss.send(SelfMatchMessage::NotifyMove(m))?;
										}
									},
									SelfMatchMessage::StartPonderThink(t,b,mc,n,m) => {
										let (ms, mg) = match mc {
											MochigomaCollections::Pair(ref ms, ref mg) => {
												(ms.clone(),mg.clone())
											},
											MochigomaCollections::Empty => {
												(HashMap::new(),HashMap::new())
											}
										};

										player.set_position(t, b, ms, mg, n, m.into_iter().map(|m| {
											m.to_move()
										}).collect::<Vec<Move>>())?;

										let m = player.think_ponder(&limit,
																user_event_queue[player_i].clone(),
																info_sender.clone(),
																	pinfo_sender.clone(),
																	on_error_handler.clone())?;

										match cr.recv()? {
											SelfMatchMessage::PonderHit => {
												if !quit_ready.load(Ordering::Acquire) {
													ss.send(SelfMatchMessage::NotifyMove(m))?;
												}
											},
											SelfMatchMessage::PonderNG => (),
											SelfMatchMessage::Quit => {
												player.quit()?;

												return Ok(());
											},
											SelfMatchMessage::Abort => {
												break;
											},
											SelfMatchMessage::Error(_) => {
												return Ok(());
											}
											_ => {
												let _ = logger.lock().map(|mut logger| {
													logger.logging(&format!("Invalid message."))
												}).map_err(|_| {
													USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
													false
												});

												if !quit_ready.load(Ordering::Acquire) {
													if !quit_ready.load(Ordering::Acquire) {
														ss.send(SelfMatchMessage::Error(player_i))?;
													}
												}
												break;
											}
										}
									},
									SelfMatchMessage::GameEnd(s) => {
										player.gameover(&s,user_event_queue[player_i].clone(),
																		on_error_handler.clone())?;

										if !quit_ready.load(Ordering::Acquire) {
											ss.send(SelfMatchMessage::Ready)?;
										}

										break;
									},
									SelfMatchMessage::Abort => {
										break;
									},
									SelfMatchMessage::Quit => {
										player.quit()?;

										return Ok(());
									},
									SelfMatchMessage::Error(_) => {
										return Ok(());
									},
									_ => {
										let _ = logger.lock().map(|mut logger| {
											logger.logging(&format!("Invalid message."))
										}).map_err(|_| {
											USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
											false
										});

										if !quit_ready.load(Ordering::Acquire) {
											ss.send(SelfMatchMessage::Error(player_i))?;
										}

										break;
									}
								}
							}
						},
						SelfMatchMessage::Quit => {
							player.quit()?;

							return Ok(());
						},
						SelfMatchMessage::Error(_) => {
							return Ok(());
						},
						_ => {
							let _ = logger.lock().map(|mut logger| {
								logger.logging(&format!("Invalid message."))
							}).map_err(|_| {
								USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
								false
							});

							if !quit_ready.load(Ordering::Acquire) {
								ss.send(SelfMatchMessage::Error(player_i))?;
							}
						}
					}
				}
			}, on_error_handler.clone()).map_err(|e| {
				match e {
					SelfMatchRunningError::SendError(SendError(_)) |
						SelfMatchRunningError::RecvError(_) => (),
					_ if !quit_ready.load(Ordering::Acquire) => {
						if let Err(ref e) = ss.send(SelfMatchMessage::Error(player_i)) {
							let _ = on_error_handler.lock().map(|h| h.call(e));
						}
					},
					_ => (),
				}
				e
			})));
		}

		let delay = Duration::from_millis(50);
		let on_error_handler = on_error_handler_arc.clone();
		let self_match_event_queue = self_match_event_queue_arc.clone();
		let logger = logger_arc.clone();
		let quit_ready = quit_ready_arc.clone();

		thread::spawn(move || {
			while !quit_ready.load(Ordering::Acquire) {
				match input_reader.read() {
					Ok(line) => {
						match input_handler(line) {
							Ok(false) => {
								return;
							},
							Err(ref e) => {
								let _ = on_error_handler.lock().map(|h| h.call(e));
								return;
							},
							_ => (),
						}
					},
					Err(ref e) if !quit_ready.load(Ordering::Acquire) => {
						let _ = on_error_handler.lock().map(|h| h.call(e));
						return;
					},
					_ => (),
				}
			}
		});

		let on_error_handler = on_error_handler_arc.clone();

		let quit_ready = quit_ready_arc.clone();

		while !quit_ready.load(Ordering::Acquire) || (match self.system_event_queue.lock() {
			Ok(system_event_queue) => system_event_queue.has_event(),
			Err(ref e) => {
				let _ = on_error_handler.lock().map(|h| h.call(e));
				false
			}
		}) || (match self_match_event_queue.lock() {
			Ok(self_match_event_queue) => self_match_event_queue.has_event(),
			Err(ref e) => {
				let _ = on_error_handler.lock().map(|h| h.call(e));
				false
			}
		}) {
			match system_event_dispatcher.dispatch_events(self, &*self.system_event_queue) {
				Ok(_) => true,
				Err(ref e) => {
					on_error_handler.lock().map(|h| h.call(e)).is_err()
				}
			};
			match self_match_event_dispatcher.dispatch_events(self, &*self_match_event_queue) {
				Ok(_) => true,
				Err(ref e) => {
					on_error_handler.lock().map(|h| h.call(e)).is_err()
				}
			};
			thread::sleep(delay);
		}

		let mut has_error = false;

		let result = bridge_h.join().map_err(|_| {
			has_error = true;
			let _ = logger.lock().map(|mut logger| {
				logger.logging(&format!("Main thread join failed."))
			}).map_err(|_| {
				USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
				false
			});
		}).unwrap_or(Err(SelfMatchRunningError::ThreadJoinFailed(String::from(
			"Main thread join failed."
		)))).map_err(|e| {
			has_error = true;
			e
		});

		for h in handlers {
			let _ = h.join().map_err(|_| {
				has_error = true;
				let _ = logger.lock().map(|mut logger| {
					logger.logging(&format!("Sub thread join failed."))
				}).map_err(|_| {
					USIStdErrorWriter::write("Logger's exclusive lock could not be secured").unwrap();
					false
				});
			}).map(|r| {
				r.map_err(|e| {
					has_error = true;
					e
				}).is_err()
			});
		}

		if has_error {
			Err(SelfMatchRunningError::Fail(String::from(
				"An error occurred while executing a self match. Please see the log for details ..."
			)))
		} else {
			result
		}
	}
}