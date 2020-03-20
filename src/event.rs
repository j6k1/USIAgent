//! イベント処理
use std::fmt;
use std::marker::PhantomData;
use std::sync::Mutex;
use std::sync::Arc;
use std::error::Error;
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
use selfmatch::*;
use rule::Validate;

/// enumを自身の各項目に対応する種別を表現する型の対応する項目にマップする
pub trait MapEventKind<K> {
	fn event_kind(&self) -> K;
}
/// システムイベント
#[derive(Debug)]
pub enum SystemEvent {
	/// usiコマンド受信イベント
	Usi,
	/// isreadyコマンド受信イベント
	IsReady,
	/// setoptionコマンド受信イベント
	SetOption(String,SysEventOption),
	/// usinewgameコマンド受信イベント
	UsiNewGame,
	/// positionコマンド受信イベント
	Position(Teban,UsiInitialPosition,u32,Vec<Move>),
	/// goコマンド受信イベント
	Go(UsiGo),
	/// stopコマンド受信イベント
	Stop,
	/// ponderhitコマンド受信イベント
	PonderHit,
	/// quitコマンド受信イベント
	Quit,
	/// gomeoverコマンド受信イベント
	GameOver(GameEndState),
	/// USIコマンド送信要求
	SendUsiCommand(UsiOutput),
	/// 終了
	QuitReady,
}
/// システムイベント種別
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum SystemEventKind {
	/// usiコマンド受信イベント
	Usi = 0,
	/// isreadyコマンド受信イベント
	IsReady,
	/// setoptionコマンド受信イベント
	SetOption,
	/// usinewgameコマンド受信イベント
	UsiNewGame,
	/// positionコマンド受信イベント
	Position,
	/// goコマンド受信イベント
	Go,
	/// stopコマンド受信イベント
	Stop,
	/// ponderhitコマンド受信イベント
	PonderHit,
	/// quitコマンド受信イベント
	Quit,
	/// gomeoverコマンド受信イベント
	GameOver,
	/// USIコマンド送信要求
	SendUsiCommand,
	/// 終了
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
/// USIオプション項目
#[derive(Debug, Eq, PartialEq)]
pub enum SysEventOption {
	/// 文字列
	Str(String),
	/// 数値
	Num(i64),
	/// 真偽値
	Bool(bool),
	/// 存在する（値がないオプション用）
	Exist,
}
impl Clone for SysEventOption {
	fn clone(&self) -> SysEventOption {
		match *self {
			SysEventOption::Str(ref s) => SysEventOption::Str(s.clone()),
			SysEventOption::Num(n) => SysEventOption::Num(n),
			SysEventOption::Bool(b) => SysEventOption::Bool(b),
			SysEventOption::Exist => SysEventOption::Exist,
		}
	}
}
/// USIオプション項目の種別
#[derive(Debug)]
pub enum SysEventOptionKind {
	/// 文字列
	Str,
	/// 数値
	Num,
	/// 真偽値
	Bool,
	/// 存在する（値がないオプション用）
	Exist,
}
/// 初期局面
#[derive(Eq,PartialEq,Debug)]
pub enum UsiInitialPosition {
	/// 平手初期局面以外
	Sfen(Banmen, MochigomaCollections),
	/// 平手初期局面
	Startpos,
}
/// goコマンド
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum UsiGo {
	/// go
	Go(UsiGoTimeLimit),
	/// go ponder
	Ponder(UsiGoTimeLimit),
	/// go mate
	Mate(UsiGoMateTimeLimit),
}
/// 持ち時間
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum UsiGoTimeLimit {
	/// 指定なし
	None,
	/// 指定あり
	Limit(Option<(u32,u32)>,Option<UsiGoByoyomiOrInc>),
	/// 無制限(go infinite)
	Infinite,
}
impl UsiGoTimeLimit {
	/// `Instant`へ変換(制限時間未指定時やinfinite指定時などはNoneが返る)
	///
	/// # Arguments
	/// * `teban` - 手番
	/// * `now` - 現在時刻
	pub fn to_instant(&self,teban:Teban,now:Instant) -> Option<Instant> {
		match self {
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
		}
	}

	/// 次の手番時の制限時間を計算
	/// (フィッシャークロックルール時以外は持ち時間から消費した時間を引いたものが返る。制限時間未指定時やinfinite指定時などはNoneが返る)
	///
	/// # Arguments
	/// * `teban` - 手番
	/// * `think_start_time` - 現在の局面が開始した時刻
	/// * `now` - 現在時刻
	pub fn calc_next_limit(&self,teban:Teban,think_start_time:Instant,now:Instant) -> Option<u64> {
		let limit = self.to_instant(teban, think_start_time);

		limit.and_then(|limit| match self {
			&UsiGoTimeLimit::None => None,
			&UsiGoTimeLimit::Infinite => None,
			&UsiGoTimeLimit::Limit(Some((ms,mg)),None) |
			&UsiGoTimeLimit::Limit(Some((ms,mg)),Some(UsiGoByoyomiOrInc::Byoyomi(_))) => {
				Some(match teban {
					Teban::Sente => ms as u64,
					Teban::Gote => mg as u64,
				})
			},
			&UsiGoTimeLimit::Limit(Some((ms,mg)),Some(UsiGoByoyomiOrInc::Inc(_,_))) => {
				let elapsed = limit - now;

				Some(match teban {
					Teban::Sente => {
						ms as u64 + elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64
					},
					Teban::Gote => {
						mg as u64 + elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64
					}
				})
			},
			&UsiGoTimeLimit::Limit(None,None) |
			&UsiGoTimeLimit::Limit(None,Some(UsiGoByoyomiOrInc::Byoyomi(_))) => {
				Some(0)
			}
			&UsiGoTimeLimit::Limit(None,Some(UsiGoByoyomiOrInc::Inc(_,_))) => {
				let elapsed = limit - now;

				Some(elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64)
			},
		})
	}
}
/// go mate時の制限時間
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum UsiGoMateTimeLimit {
	/// 未指定
	None,
	/// 指定あり
	Limit(u32),
	/// 無制限(go mate infinite)
	Infinite,
}
impl UsiGoMateTimeLimit {
	/// `Instant`へ変換(制限時間未指定時やinfinite指定時などはNoneが返る)
	///
	/// # Arguments
	/// * `now` - 現在時刻
	pub fn to_instant(&self,now:Instant) -> Option<Instant> {
		match *self {
			UsiGoMateTimeLimit::Infinite | UsiGoMateTimeLimit::None => None,
			UsiGoMateTimeLimit::Limit(limit) => {
				Some(now + Duration::from_millis(limit as u64))
			}
		}
	}
}
/// 持ち時間（'go byoyomi <x>' or 'go binc <x>, winc<x>'）
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum UsiGoByoyomiOrInc {
	/// 秒読み
	Byoyomi(u32),
	/// 加算時間（フィッシャークロックルール）
	Inc(u32,u32),
}
/// ユーザーイベント
#[derive(Debug)]
pub enum UserEvent {
	/// 思考の停止
	Stop,
	/// go ponderの予測手にHit
	PonderHit(Instant),
	/// 終了要求
	Quit,
}
/// ユーザーイベントの種別
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum UserEventKind {
	/// 思考の停止
	Stop = 0,
	/// go ponderの予測手にHit
	PonderHit,
	/// 終了要求
	Quit,
}
impl MapEventKind<UserEventKind> for UserEvent {
	fn event_kind(&self) -> UserEventKind {
		match *self {
			UserEvent::Stop => UserEventKind::Stop,
			UserEvent::PonderHit(_) => UserEventKind::PonderHit,
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
/// 自己対局時のイベント
#[derive(Debug)]
pub enum SelfMatchEvent {
	/// 対局開始
	GameStart(u32,Teban,String),
	/// 手が指された
	Moved(Teban,Moved),
	/// 対局終了
	GameEnd(SelfMatchGameEndState),
	/// 中断
	Abort,
}
/// 指された手
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum Moved {
	/// 盤面上の駒の移動
	To(MovedKind,(u32,u32),(u32,u32),bool),
	/// 持ち駒を置いた
	Put(MochigomaKind,(u32,u32)),
}
/// 動かした駒の種類
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum MovedKind {
	/// 歩
	Fu = 0,
	/// 香
	Kyou,
	/// 桂
	Kei,
	/// 銀
	Gin,
	/// 金
	Kin,
	/// 角
	Kaku,
	/// 飛車
	Hisha,
	/// 王
	SOu,
	/// 玉
	GOu,
	/// と金
	FuN,
	/// 成香
	KyouN,
	/// 成桂
	KeiN,
	/// 成銀
	GinN,
	/// 馬
	KakuN,
	/// 龍
	HishaN,
	/// 空白
	Blank,
}
/// 自己対局時の対局終了時の状態
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum SelfMatchGameEndState {
	/// 勝ち
	Win(Teban),
	/// 投了
	Resign(Teban),
	/// 入玉宣言勝ち
	NyuGyokuWin(Teban),
	/// 入玉宣言勝ちを宣言したが条件を満たさず負けになった
	NyuGyokuLose(Teban),
	/// 引き分け
	Draw,
	/// 反則負け
	Foul(Teban,FoulKind),
	/// 時間切れ負け
	Timeover(Teban),
}
/// 対局の勝敗
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum GameEndState {
	/// 勝ち
	Win,
	/// 負け
	Lose,
	/// 引き分け
	Draw,
}
/// 自己対局時の反則負けの種類
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum FoulKind {
	/// 合法手でない
	InvalidMove,
	/// 打ち歩詰め
	PutFuAndMate,
	/// 千日手
	Sennichite,
	/// 連続王手の千日手
	SennichiteOu,
	/// 王手に応じなかった
	NotRespondedOute,
	/// 自分から相手に王を取られる位置に駒を動かした
	Suicide,
}
/// 自己対局時のイベントの種別
#[derive(Debug)]
pub enum SelfMatchEventKind {
	/// 対局開始
	GameStart = 0,
	/// 手が指された
	Moved,
	/// 対局終了
	GameEnd,
	/// 中断
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
impl fmt::Display for MovedKind {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			MovedKind::Fu => write!(f,"歩"),
			MovedKind::Kyou => write!(f,"香"),
			MovedKind::Kei => write!(f,"桂"),
			MovedKind::Gin => write!(f,"銀"),
			MovedKind::Kin => write!(f,"金"),
			MovedKind::Kaku => write!(f,"角"),
			MovedKind::Hisha => write!(f,"飛"),
			MovedKind::SOu => write!(f,"王"),
			MovedKind::GOu => write!(f,"玉"),
			MovedKind::FuN => write!(f,"成歩"),
			MovedKind::KyouN => write!(f,"成香"),
			MovedKind::KeiN => write!(f,"成桂"),
			MovedKind::GinN => write!(f,"成銀"),
			MovedKind::KakuN => write!(f,"馬"),
			MovedKind::HishaN => write!(f,"龍"),
			MovedKind::Blank => write!(f,"駒無し"),
		}
	}
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
					KomaKind::Blank => Moved::To(MovedKind::Blank,(sx,sy),(dx,dy),n),
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
const MOCHIGOMA_DISPLAY_MAP:[char; 7] = ['歩','香','桂','銀','金','角','飛'];

impl fmt::Display for Moved {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	if !self.validate() {
	 		match self {
		 		&Moved::To(k,(sx,sy),(dx,dy),true) if sy > 9 || dy > 9 => {
					write!(f,"{},{}{} -> {},{}成 （不正な手です）",sx,sy,k,dx,dy)
		 		},
		 		&Moved::To(k,(sx,sy),(dx,dy),false) if sy > 9 || dy > 9 => {
					write!(f,"{},{}{} -> {},{} （不正な手です）",sx,sy,k,dx,dy)
		 		},
		 		&Moved::To(k,(sx,sy),(dx,dy),true) => {
					write!(f,"{}{}{} -> {}{}成 （不正な手です）",sx,KANSUJI_MAP[sy as usize],k,dx,KANSUJI_MAP[dy as usize])
		 		},
		 		&Moved::To(k,(sx,sy),(dx,dy),false) => {
					write!(f,"{}{}{} -> {}{} （不正な手です）",sx,KANSUJI_MAP[sy as usize],k,dx,KANSUJI_MAP[dy as usize])
		 		},
		 		&Moved::Put(k,(x,y)) if y > 9 => {
		 			write!(f,"{},{}{} （不正な手です）",x,y,MOCHIGOMA_DISPLAY_MAP[k as usize])
		 		},
		 		&Moved::Put(k,(x,y)) => {
		 			write!(f,"{}{}{} （不正な手です）",x,KANSUJI_MAP[y as usize],MOCHIGOMA_DISPLAY_MAP[k as usize])
		 		},
	 		}
	 	} else {
		 	match self {
		 		&Moved::To(k,(sx,sy),(dx,dy),true) => {
					write!(f,"{}{}{} -> {}{}成",sx,KANSUJI_MAP[sy as usize],k,dx,KANSUJI_MAP[dy as usize])
		 		},
		 		&Moved::To(k,(sx,sy),(dx,dy),false) => {
					write!(f,"{}{}{} -> {}{}",sx,KANSUJI_MAP[sy as usize],k,dx,KANSUJI_MAP[dy as usize])
		 		},
		 		&Moved::Put(k,(x,y)) => {
		 			write!(f,"{}{}{}",x,KANSUJI_MAP[y as usize],MOCHIGOMA_DISPLAY_MAP[k as usize])
		 		},
		 	}
	 	}
	 }
}
/// イベントキュー
#[derive(Debug)]
pub struct EventQueue<E,K> where E: MapEventKind<K> + fmt::Debug, K: fmt::Debug {
	event_kind:PhantomData<K>,
	events:Vec<E>,
}
impl<E,K> EventQueue<E,K> where E: MapEventKind<K> + fmt::Debug, K: fmt::Debug {
	/// `EventQueue`の生成
	pub fn new() -> EventQueue<E,K> {
		EventQueue {
			event_kind:PhantomData::<K>,
			events: Vec::new()
		}
	}
	/// イベントの追加
	///
	/// # Arguments
	/// * `e` - イベント
	pub fn push(&mut self,e:E) {
		self.events.push(e);
	}
	/// イベントキューのクリア
	pub fn clear(&mut self) {
		self.events.clear();
	}
	/// イベントキューの中身の取り出し（呼出し直後キューは空になる）
	pub fn drain_events(&mut self) -> Vec<E> {
		self.events.drain(0..).collect()
	}
	/// キューにイベントがあるか否か？
	pub fn has_event(&self) -> bool {
		self.events.len() > 0
	}
}
/// イベントディスパッチャ
pub trait EventDispatcher<'b,K,E,T,UE> where K: MaxIndex + fmt::Debug,
											E: MapEventKind<K> + fmt::Debug,
											UE: PlayerError,
											EventHandlerError<K,UE>: From<UE>,
											usize: From<K> {
	/// イベントハンドラの追加
	///
	/// # Arguments
	/// * `id` - イベント種別
	/// * `handler` - イベントハンドラ
	///
	/// # Errors
	///
	/// この関数は以下のエラーを返すケースがあります。
	/// * [`EventHandlerError`] 手が合法手でない
	///
	/// [`EventHandlerError`]: ../error/enum.EventHandlerError.html
	fn add_handler<F>(&mut self, id:K, handler:F) where F: FnMut(&T,&E) ->
													Result<(), EventHandlerError<K,UE>> + 'b;

	/// 一度だけ実行されるイベントハンドラの追加
	///
	/// # Arguments
	/// * `id` - イベント種別
	/// * `handler` - イベントハンドラ
	///
	/// # Errors
	///
	/// この関数は以下のエラーを返すケースがあります。
	/// * [`EventHandlerError`] 手が合法手でない
	///
	/// [`EventHandlerError`]: ../error/enum.EventHandlerError.html
	fn add_once_handler<F>(&mut self, id:K, handler:F) where F: FnMut(&T,&E) ->
													Result<(), EventHandlerError<K,UE>> + 'b;

	/// イベントのディスパッチ
	///
	/// # Arguments
	/// * `ctx` - コンテキストオブジェクト
	/// * `event_queue` - イベントキュー
	///
	/// # Errors
	///
	/// この関数は以下のエラーを返すケースがあります。
	/// * [`EventDispatchError`] 手が合法手でない
	///
	/// [`EventDispatchError`]: ../error/enum.EventDispatchError.html
	fn dispatch_events<'a>(&mut self, ctx:&T, event_queue:&'a Mutex<EventQueue<E,K>>) ->
										Result<(), EventDispatchError<'a,EventQueue<E,K>,E,UE>>
										where E: fmt::Debug, K: fmt::Debug,
												UE: Error + fmt::Debug,
												EventHandlerError<K,UE>: From<UE>,
												usize: From<K>;
}
/// `EventDispatcher`の実装
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
	handlers:Vec<Vec<Box<dyn FnMut(&T,&E) -> Result<(), EventHandlerError<K,UE>> + 'b>>>,
	once_handlers:Vec<Vec<Box<dyn FnMut(&T, &E) -> Result<(), EventHandlerError<K,UE>> + 'b>>>,
}
impl<'b,K,E,T,L,UE> USIEventDispatcher<'b,K,E,T,L,UE>
	where K: MaxIndex + fmt::Debug,
			E: MapEventKind<K> + fmt::Debug,
			L: Logger,
			UE: PlayerError,
			EventHandlerError<K,UE>: From<UE>,
			usize: From<K> {
	/// `USIEventDispatcher`の生成
	///
	/// # Arguments
	/// * `error_handler` - エラーを書き込むためのオブジェクト
	pub fn new(on_error_handler:&Arc<Mutex<OnErrorHandler<L>>>) -> USIEventDispatcher<'b,K,E,T,L,UE>
											where K: MaxIndex + fmt::Debug, usize: From<K>,
											E: MapEventKind<K> + fmt::Debug,
											L: Logger,
											UE: PlayerError,
											EventHandlerError<K,UE>: From<UE>, {

		let mut o = USIEventDispatcher {
			on_error_handler:on_error_handler.clone(),
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
	fn add_handler<F>(&mut self, id:K, handler:F) where F: FnMut(&T,&E) ->
											Result<(), EventHandlerError<K,UE>> + 'b {
		self.handlers[usize::from(id)].push(Box::new(handler));
	}

	fn add_once_handler<F>(&mut self, id:K, handler:F) where F: FnMut(&T,&E) ->
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
			for h in self.handlers[usize::from(e.event_kind())].iter_mut() {
				match h(ctx, e) {
					Ok(_) => true,
					Err(ref e) => {
						has_error = true;
						self.on_error_handler.lock().map(|h| h.call(e)).is_err()
					}
				};
			}

			if !self.once_handlers[usize::from(e.event_kind())].is_empty() {
				let mut once_handlers:Vec<Box<dyn FnMut(&T, &E) -> Result<(), EventHandlerError<K,UE>>>> =
											self.once_handlers[usize::from(e.event_kind())].drain(0..)
																							.collect();
				for h in once_handlers.iter_mut() {
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
/// システムイベントキュー
pub type SystemEventQueue = EventQueue<SystemEvent,SystemEventKind>;
/// システムイベントキューを処理するためのイベントディスパッチャ
pub type SystemEventDispatcher<'a,T,E,L> = USIEventDispatcher<'a,SystemEventKind,SystemEvent,T,L,E>;
/// ユーザーイベントキュー
pub type UserEventQueue = EventQueue<UserEvent,UserEventKind>;
/// システムイベントキューを処理するためのイベントディスパッチャ
pub type UserEventDispatcher<'a,T,E,L> = USIEventDispatcher<'a,UserEventKind,UserEvent,T,L,E>;
/// 自己対局イベントキュー
pub type SelfMatchEventQueue = EventQueue<SelfMatchEvent,SelfMatchEventKind>;
/// 自己対局イベントキューを処理するためのイベントディスパッチャ
pub type SelfMatchEventDispatcher<'a,E,L> = USIEventDispatcher<'a,SelfMatchEventKind,SelfMatchEvent,SelfMatchEngine<E>,L,E>;
