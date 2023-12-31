//! 合法手の列挙等、将棋のルールに関連した機能
use std::collections::HashMap;
use std::time::{Instant,Duration};
use std::fmt;
use std::fmt::Formatter;
use std::ops::BitOr;
use std::ops::Not;
use std::convert::TryFrom;

use shogi::*;
use hash::*;
use error::*;
use event::*;

use shogi::KomaKind::{
	SFu,
	SKyou,
	SKei,
	SGin,
	SKin,
	SKaku,
	SHisha,
	SOu,
	SFuN,
	SKyouN,
	SKeiN,
	SGinN,
	SKakuN,
	SHishaN,
	GFu,
	GKyou,
	GKei,
	GGin,
	GKin,
	GKaku,
	GHisha,
	GOu,
	GFuN,
	GKyouN,
	GKeiN,
	GGinN,
	GKakuN,
	GHishaN,
	Blank
};
use Find;

trait KomaKindFrom<T> {
	fn kind_from(k:T) -> Self;
}
impl KomaKindFrom<u32> for ObtainKind {
	fn kind_from(k:u32) -> ObtainKind {
		match k {
			0 => ObtainKind::Fu,
			1 => ObtainKind::Kyou,
			2 => ObtainKind::Kei,
			3 => ObtainKind::Gin,
			4 => ObtainKind::Kin,
			5 => ObtainKind::Kaku,
			6 => ObtainKind::Hisha,
			7 => ObtainKind::Ou,
			8 => ObtainKind::FuN,
			9 => ObtainKind::KyouN,
			10 => ObtainKind::KeiN,
			11=> ObtainKind::GinN,
			12 => ObtainKind::KakuN,
			13 => ObtainKind::HishaN,
			_ => unreachable!(),
		}
	}
}
impl KomaKindFrom<u32> for MochigomaKind {
	fn kind_from(k:u32) -> MochigomaKind {
		match k {
			0 => MochigomaKind::Fu,
			1 => MochigomaKind::Kyou,
			2 => MochigomaKind::Kei,
			3 => MochigomaKind::Gin,
			4 => MochigomaKind::Kin,
			5 => MochigomaKind::Kaku,
			6 => MochigomaKind::Hisha,
			_ => unreachable!(),
		}
	}
}
/// 左上からx * 9 + yで表されるインデックスからx,yへの変換
pub trait SquareToPoint {
	fn square_to_point(self) -> (u32,u32);
}
pub type Square = i32;
impl SquareToPoint for Square {
	#[inline]
	fn square_to_point(self) -> (u32,u32) {
		let x = self * 114 / 1024;
		let y = self - x * 9;

		(x as u32,y as u32)
	}
}
impl SquareToPoint for u32 {
	#[inline]
	fn square_to_point(self) -> (u32,u32) {
		let x = self * 114 / 1024;
		let y = self - x * 9;

		(x,y)
	}
}
/// 合法手
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum LegalMove {
	/// 盤面上の駒を動かす手
	To(LegalMoveTo),
	/// 持ち駒を置く手
	Put(LegalMovePut),
}
/// 盤面上の駒を動かす手
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct LegalMoveTo(u32);
impl LegalMoveTo {
	/// `LegalMoveTo`を生成
	///
	/// # Arguments
	/// * `src` - 盤面左上を0,0とし、x * 9 + yで表される移動元の位置
	/// * `to` - 盤面左上を0,0とし、x * 9 + yで表される移動先の駒の位置
	/// * `nari` - 成るか否か
	/// * `obtaind` - 獲った駒
	pub fn new(src:u32,to:u32,nari:bool,obtaind:Option<ObtainKind>) -> LegalMoveTo {
		let n:u32 = if nari {
			1
		} else {
			0
		};

		LegalMoveTo(
			obtaind.map_or(0, |o| o as u32 + 1) << 15 |
			n << 14 |
			(to & 0b1111111) << 7 |
			src & 0b1111111
		)
	}

	/// 移動元の左上からx * 9 + yで表されるインデックス
	#[inline]
	pub fn src(&self) -> u32 {
		self.0 & 0b1111111
	}
	/// 移動先の左上からx * 9 + yで表されるインデックス
	#[inline]
	pub fn dst(&self) -> u32 {
		(self.0 >> 7) & 0b1111111
	}
	/// 成る手が否か
	#[inline]
	pub fn is_nari(&self) -> bool {
		(self.0 & 1 << 14) != 0
	}
	/// 獲った駒
	#[inline]
	pub fn obtained(&self) -> Option<ObtainKind> {
		let o:u32 = self.0 >> 15;

		if o == 0 {
			None
		} else {
			Some(ObtainKind::kind_from(o-1))
		}
	}
}
/// 持ち駒を置く手
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct LegalMovePut(u32);
impl LegalMovePut {
	/// `LegalMovePut`を生成
	///
	/// # Arguments
	/// * `kind` - 置く駒の種類
	/// * `to` - 盤面左上を0,0とし、x * 9 + yで表される移動先の駒の位置
	pub fn new(kind:MochigomaKind,to:u32) -> LegalMovePut {
		LegalMovePut(
			(to & 0b1111111) << 3 |
			(kind as u32) & 0b111
		)
	}
	/// 駒を置く位置のx * 9 + yで表されるインデックス
	#[inline]
	pub fn dst(&self) -> u32 {
		(self.0 >> 3) & 0b1111111
	}
	/// 置く駒の種類
	#[inline]
	pub fn kind(&self) -> MochigomaKind {
		MochigomaKind::kind_from(self.0 & 0b111)
	}
}
/// 適用される手
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum AppliedMove {
	/// 盤面上の駒を動かす手
	To(AppliedMoveTo),
	/// 持ち駒を置く手
	Put(AppliedMovePut)
}
/// 盤面上の駒を動かす手
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct AppliedMoveTo(u32);
impl AppliedMoveTo {
	/// 移動元の左上からx * 9 + yで表されるインデックス
	#[inline]
	pub fn src(&self) -> u32 {
		self.0 & 0b1111111
	}
	/// 移動先の左上からx * 9 + yで表されるインデックス
	#[inline]
	pub fn dst(&self) -> u32 {
		(self.0 >> 7) & 0b1111111
	}
	/// 成るか否か
	#[inline]
	pub fn is_nari(&self) -> bool {
		(self.0 & 1 << 14) != 0
	}
}
impl From<LegalMoveTo> for AppliedMoveTo {
	fn from(m:LegalMoveTo) -> AppliedMoveTo {
		AppliedMoveTo(m.0 & 0b111111111111111)
	}
}
/// 持ち駒を置く手
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct AppliedMovePut(u32);
impl AppliedMovePut {
	/// 駒を置く位置の左上からx * 9 + yで表されるインデックス
	#[inline]
	pub fn dst(&self) -> u32 {
		(self.0 >> 3) & 0b1111111
	}
	/// 置く駒の種類
	#[inline]
	pub fn kind(&self) -> MochigomaKind {
		MochigomaKind::kind_from(self.0 & 0b111)
	}
}
impl From<LegalMovePut> for AppliedMovePut {
	fn from(m:LegalMovePut) -> AppliedMovePut {
		AppliedMovePut(m.0)
	}
}
impl From<LegalMove> for AppliedMove {
	fn from(m:LegalMove) -> AppliedMove {
		match m {
			LegalMove::To(m) => AppliedMove::To(AppliedMoveTo::from(m)),
			LegalMove::Put(m) => AppliedMove::Put(AppliedMovePut::from(m))
		}
	}
}
impl From<Move> for AppliedMove {
	fn from(m:Move) -> AppliedMove {
		match m {
			Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,n)) => {
				let sx = 9 - sx;
				let sy = sy - 1;
				let dx = 9 - dx;
				let dy = dy - 1;

				let src = sx * 9 + sy;
				let dst = dx * 9 + dy;

				let n = if n {
					1
				} else {
					0
				};

				AppliedMove::To(AppliedMoveTo(
					n << 14 |
					(dst & 0b1111111) << 7 |
					src & 0b1111111
				))
			},
			Move::Put(kind,KomaDstPutPosition(x,y)) => {
				let x = 9 - x;
				let y = y - 1;

				let dst = x * 9 + y;

				AppliedMove::Put(AppliedMovePut(
					(dst & 0b1111111) << 3 |
					(kind as u32) & 0111
				))
			}
		}
	}
}
impl LegalMove {
	/// `AppliedMove`へ変換
	pub fn to_applied_move(self) -> AppliedMove {
		AppliedMove::from(self)
	}
	/// `Move`へ変換
	pub fn to_move(self) -> Move {
		Move::from(self)
	}
}
impl From<LegalMove> for Move {
	fn from(m:LegalMove) -> Move {
		match m {
			LegalMove::To(m) => {
				let src = m.src();
				let dst = m.dst();
				let n = m.is_nari();
				let (sx,sy) = src.square_to_point();
				let (dx,dy) = dst.square_to_point();

				let sx = 9 - sx;
				let sy = sy + 1;
				let dx = 9 - dx;
				let dy = dy + 1;

				Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,n))
			},
			LegalMove::Put(m) => {
				let dst = m.dst();
				let kind = m.kind();
				let (dx,dy) = dst.square_to_point();
				let dx = 9 - dx;
				let dy = dy + 1;

				Move::Put(kind,KomaDstPutPosition(dx,dy))
			}
		}
	}
}
impl AppliedMove {
	/// `Move`へ変換
	pub fn to_move(self) -> Move {
		Move::from(self)
	}
}
impl From<AppliedMove> for Move {
	fn from(m:AppliedMove) -> Move {
		match m {
			AppliedMove::To(m) => {
				let src = m.src();
				let dst = m.dst();
				let n = m.is_nari();
				let (sx,sy) = src.square_to_point();
				let (dx,dy) = dst.square_to_point();
				let sx = 9 - sx;
				let sy = sy + 1;
				let dx = 9 - dx;
				let dy = dy + 1;

				Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,n))
			},
			AppliedMove::Put(m) => {
				let dst = m.dst();
				let kind = m.kind();
				let (dx,dy) = dst.square_to_point();
				let dx = 9 - dx;
				let dy = dy + 1;

				Move::Put(kind,KomaDstPutPosition(dx,dy))
			}
		}
	}
}
impl Find<(KomaSrcPosition,KomaDstToPosition),Move> for Vec<LegalMove> {
	fn find(&self,query:&(KomaSrcPosition,KomaDstToPosition)) -> Option<Move> {
		match query {
			&(ref s,ref d) => {
				let (sx,sy) = match s {
					KomaSrcPosition(sx,sy) => (9 - sx, sy - 1)
				};

				let (dx,dy) = match d {
					KomaDstToPosition(dx,dy,_) => (9 - dx, dy -1),
				};

				let src = sx * 9 + sy;
				let dst = dx * 9 + dy;

				for m in self {
					match m {
						&LegalMove::To(m) => {
							if src == m.src() && dst == m.dst() {
								return Some(Move::To(*s,*d));
							}
						},
						_ => (),
					}
				}
			}
		}

		None
	}
}
impl Find<KomaPosition,Move> for Vec<LegalMove> {
	fn find(&self,query:&KomaPosition) -> Option<Move> {
		let (x,y) = match query {
			&KomaPosition(x,y) => (x,y)
		};

		let index = (9 - x) * 9 + (y - 1);

		for m in self {
			match m {
				LegalMove::To(mt) => {
					if index == mt.dst() {
						return Some(Move::from(*m));
					}
				},
				LegalMove::Put(mp) => {
					if index == mp.dst() {
						return Some(Move::from(*m));
					}
				}
			}
		}

		None
	}
}
impl Find<(MochigomaKind,KomaDstPutPosition),Move> for Vec<LegalMove> {
	fn find(&self,query:&(MochigomaKind,KomaDstPutPosition)) -> Option<Move> {
		match query {
			&(ref k, ref d) => {
				let (dx,dy) = match d {
					&KomaDstPutPosition(dx,dy) => (9 - dx, dy - 1)
				};
				let index = dx * 9 + dy;

				for m in self {
					match m {
						LegalMove::Put(mp) => {
							if *k == mp.kind() && index == mp.dst() {
								return Some(Move::from(*m));
							}
						},
						_ => (),
					}
				}
			}
		}

		None
	}
}
impl Find<ObtainKind,Vec<Move>> for Vec<LegalMove> {
	fn find(&self,query:&ObtainKind) -> Option<Vec<Move>> {
		let mut mvs:Vec<Move> = Vec::new();

		for m in self {
			match m {
				LegalMove::To(mt) => {
					if let Some(o) = mt.obtained() {
						if  o == *query {
							mvs.push(Move::from(*m));
						}
					}
				},
				_ => (),
			}
		}

		match mvs.len() {
			0 => None,
			_ => Some(mvs),
		}
	}
}
/// 合法手を生成するために内部で利用するビットボード
#[derive(Clone, Copy, Eq)]
pub union BitBoard {
	pub merged_bitboard:u128,
	pub bitboard:[u64; 2]
}
impl BitOr for BitBoard {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self {
		unsafe {
			BitBoard { merged_bitboard: self.merged_bitboard | rhs.merged_bitboard }
		}
	}
}
impl Not for BitBoard {
	type Output = Self;

	fn not(self) -> Self {
		unsafe {
			BitBoard { merged_bitboard: !self.merged_bitboard }
		}
	}
}
impl PartialEq for BitBoard {
	fn eq(&self,other:&BitBoard) -> bool {
		unsafe { self.merged_bitboard == other.merged_bitboard }
	}
}
impl fmt::Debug for BitBoard {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", unsafe { self.merged_bitboard })
	}
}
impl Iterator for BitBoard {
	type Item = Square;
	fn next(&mut self) -> Option<Self::Item> {
		match Rule::pop_lsb(self) {
			-1 => None,
			p => Some(p)
		}
	}
}
/// 合法手生成に内部で利用するビットボード群と盤面を管理する構造体
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct State {
	banmen:Banmen,
	part:PartialState
}
impl State {
	/// `State`の生成
	///
	/// # Arguments
	/// * `banmen` - 盤面
	pub fn new(banmen:Banmen) -> State {
		let mut sente_self_board:u128 = 0;
		let mut sente_opponent_board:u128 = 0;
		let mut gote_self_board:u128 = 0;
		let mut gote_opponent_board:u128 = 0;
		let mut sente_nari_board:u128 = 0;
		let mut gote_nari_board:u128 = 0;
		let mut sente_fu_board:u128 = 0;
		let mut gote_fu_board:u128 = 0;
		let mut sente_kyou_board:u128 = 0;
		let mut gote_kyou_board:u128 = 0;
		let mut sente_kei_board:u128 = 0;
		let mut gote_kei_board:u128 = 0;
		let mut sente_gin_board:u128 = 0;
		let mut gote_gin_board:u128 = 0;
		let mut sente_kin_board:u128 = 0;
		let mut gote_kin_board:u128 = 0;
		let mut sente_kaku_board:u128 = 0;
		let mut gote_kaku_board:u128 = 0;
		let mut sente_hisha_board:u128 = 0;
		let mut gote_hisha_board:u128 = 0;
		let mut sente_opponent_ou_position_board:u128 = 0;
		let mut gote_opponent_ou_position_board:u128 = 0;

		match banmen {
			Banmen(ref kinds) => {
				for y in 0..9 {
					for x in 0..9 {
						let kind = kinds[y][x];
						match kind {
							SFu | SFuN => sente_fu_board ^= 1 << (x * 9 + y + 1),
							SKyou | SKyouN => sente_kyou_board ^= 1 << (x * 9 + y + 1),
							SKei | SKeiN => sente_kei_board ^= 1 << (x * 9 + y + 1),
							SGin | SGinN => sente_gin_board ^= 1 << (x * 9 + y + 1),
							SKin => sente_kin_board ^= 1 << (x * 9 + y + 1),
							SKaku | SKakuN => sente_kaku_board ^= 1 << (x * 9 + y + 1),
							SHisha | SHishaN => sente_hisha_board ^= 1 << (x * 9 + y + 1),
							SOu => gote_opponent_ou_position_board ^= 1 << ((8 - x) * 9 + (8 - y) + 1),
							GFu | GFuN => gote_fu_board ^= 1 << (x * 9 + y + 1),
							GKyou | GKyouN => gote_kyou_board ^= 1 << (x * 9 + y + 1),
							GKei | GKeiN => gote_kei_board ^= 1 << (x * 9 + y + 1),
							GGin | GGinN => gote_gin_board ^= 1 << (x * 9 + y + 1),
							GKin => gote_kin_board ^= 1 << (x * 9 + y + 1),
							GKaku | GKakuN => gote_kaku_board ^= 1 << (x * 9 + y + 1),
							GHisha | GHishaN => gote_hisha_board ^= 1 << (x * 9 + y + 1),
							GOu => sente_opponent_ou_position_board ^= 1 << (x * 9 + y + 1),
							_ => (),
						}

						if kind < GFu {
							sente_self_board ^= 1 << (x * 9 + y + 1);
							gote_opponent_board ^= 1 << ((8 - x) * 9 + (8 - y) + 1);
						} else if kind >= GFu && kind < Blank {
							gote_self_board ^= 1 << ((8 - x) * 9 + (8 - y) + 1);
							sente_opponent_board ^= 1 << (x * 9 + y + 1);
						}

						if kind >= SFuN && kind < GFu {
							sente_nari_board ^= 1 << (x * 9 + y + 1);
						} else if kind >= GFuN && kind < Blank {
							gote_nari_board ^= 1 << (x * 9 + y + 1)
						}
					}
				}
			}
		}

		State {
			banmen:banmen,
			part:PartialState {
				sente_self_board:BitBoard{ merged_bitboard: sente_self_board },
				sente_opponent_board:BitBoard{ merged_bitboard: sente_opponent_board },
				gote_self_board:BitBoard{ merged_bitboard: gote_self_board },
				gote_opponent_board:BitBoard{ merged_bitboard: gote_opponent_board },
				sente_nari_board:BitBoard{ merged_bitboard: sente_nari_board },
				gote_nari_board:BitBoard{ merged_bitboard: gote_nari_board },
				sente_fu_board:BitBoard{ merged_bitboard: sente_fu_board },
				gote_fu_board:BitBoard{ merged_bitboard: gote_fu_board },
				sente_kyou_board:BitBoard{ merged_bitboard: sente_kyou_board },
				gote_kyou_board:BitBoard{ merged_bitboard: gote_kyou_board },
				sente_kei_board:BitBoard{ merged_bitboard: sente_kei_board },
				gote_kei_board:BitBoard{ merged_bitboard: gote_kei_board },
				sente_gin_board:BitBoard{ merged_bitboard: sente_gin_board },
				gote_gin_board:BitBoard{ merged_bitboard: gote_gin_board },
				sente_kin_board:BitBoard{ merged_bitboard: sente_kin_board },
				gote_kin_board:BitBoard{ merged_bitboard: gote_kin_board },
				sente_kaku_board:BitBoard{ merged_bitboard: sente_kaku_board },
				gote_kaku_board:BitBoard{ merged_bitboard: gote_kaku_board },
				sente_hisha_board:BitBoard{ merged_bitboard: sente_hisha_board },
				gote_hisha_board:BitBoard{ merged_bitboard: gote_hisha_board },
				sente_opponent_ou_position_board:BitBoard{ merged_bitboard: sente_opponent_ou_position_board },
				gote_opponent_ou_position_board:BitBoard{ merged_bitboard: gote_opponent_ou_position_board }
			}
		}
	}

	/// 関数を盤面に適用する
	///
	/// # Arguments
	/// * `f` - コールバック関数
	pub fn map_banmen<F,T>(&self,mut f:F) -> T where F: FnMut(&Banmen) -> T {
		f(&self.banmen)
	}
	/// 関数を盤面と`PartialState`に適用する
	///
	/// # Arguments
	/// * `f` - コールバック関数
	pub fn map<F,T>(&self,mut f:F) -> T where F: FnMut(&Banmen,&PartialState) -> T {
		f(&self.banmen,&self.part)
	}
	/// 盤面への不変な参照を返す
	pub fn get_banmen(&self) -> &Banmen {
		&self.banmen
	}
	/// `PartialState`への不変な参照を返す
	pub fn get_part(&self) -> &PartialState {
		&self.part
	}
}
/// 合法手の生成に内部で利用するビットボードの集合
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PartialState {
	/// 先手視点の先手側の駒のビットボード
	pub sente_self_board:BitBoard,
	/// 先手視点の後手側の駒のビットボード
	pub sente_opponent_board:BitBoard,
	/// 後手視点の後手側の駒のビットボード
	pub gote_self_board:BitBoard,
	/// 後手視点の先手側の駒のビットボード
	pub gote_opponent_board:BitBoard,
	/// 先手の成っている駒のビットボード
	pub sente_nari_board:BitBoard,
	/// 後手の成っている駒のビットボード
	pub gote_nari_board:BitBoard,
	/// 先手の歩の位置のビットボード
	pub sente_fu_board:BitBoard,
	/// 後手の歩の位置のビットボード
	pub gote_fu_board:BitBoard,
	/// 先手の香車の位置のビットボード
	pub sente_kyou_board:BitBoard,
	/// 後手の香車の位置のビットボード
	pub gote_kyou_board:BitBoard,
	/// 先手の桂馬の位置のビットボード
	pub sente_kei_board:BitBoard,
	/// 後手の桂馬の位置のビットボード
	pub gote_kei_board:BitBoard,
	/// 先手の銀の位置のビットボード
	pub sente_gin_board:BitBoard,
	/// 後手の銀の位置のビットボード
	pub gote_gin_board:BitBoard,
	/// 先手の金の位置のビットボード
	pub sente_kin_board:BitBoard,
	/// 後手の金の位置のビットボード
	pub gote_kin_board:BitBoard,
	/// 先手の角の位置のビットボード
	pub sente_kaku_board:BitBoard,
	/// 後手の角の位置のビットボード
	pub gote_kaku_board:BitBoard,
	/// 先手の飛車の位置のビットボード
	pub sente_hisha_board:BitBoard,
	/// 後手の飛車の位置のビットボード
	pub gote_hisha_board:BitBoard,
	/// 先手視点の後手の玉の位置のビットボード
	pub sente_opponent_ou_position_board:BitBoard,
	/// 後手視点の先手の王の位置のビットボード
	pub gote_opponent_ou_position_board:BitBoard
}
impl PartialState {
	/// 自身に対応する盤面を引数に受け取り`State`へと変換して返す。
	///
	/// 引数に渡した`Banmen`が自身の状態に対して正しくない場合は正しく動作しない。
	///
	/// # Arguments
	/// * `banmen` - 自身に対応する盤面
	pub fn to_full_state(&self,banmen:Banmen) -> State {
		State {
			banmen:banmen,
			part:self.clone()
		}
	}
}
/// 局面情報
#[derive(Clone)]
pub struct Kyokumen {
	pub teban:Teban,
	pub mc:MochigomaCollections,
	pub state:State
}
const CANDIDATE_BITS:[u128; 14] = [
	// 歩
	0b000000000_000000010_000000000,
	// 香車(この値は利用しない)
	0b0,
	// 桂馬
	0b000000001_000000000_000000001,
	// 銀
	0b000001010_000000010_000001010,
	// 金
	0b000000110_000001010_000000110,
	// 角(この値は利用しない)
	0b0,
	// 飛車(この値は利用しない)
	0b0,
	// 王
	0b000001110_000001010_000001110,
	// 成歩
	0b000000110_000001010_000000110,
	// 成香
	0b000000110_000001010_000000110,
	// 成桂
	0b000000110_000001010_000000110,
	// 成銀
	0b000000110_000001010_000000110,
	// 成角(一マスだけ進める手だけここに定義)
	0b000000100_000001010_000000100,
	// 成飛(一マスだけ進める手だけここに定義)
	0b000001010_000000000_000001010
];
const TO_RIGHT_TOP_MASK:[u128;17] = [
	0b000000000_000000000_000000000_000000000_000000000_000000000_000000000_000000000_000000001,
	0b000000000_000000000_000000000_000000000_000000000_000000000_000000000_000000001_000000010,
	0b000000000_000000000_000000000_000000000_000000000_000000000_000000001_000000010_000000100,
	0b000000000_000000000_000000000_000000000_000000000_000000001_000000010_000000100_000001000,
	0b000000000_000000000_000000000_000000000_000000001_000000010_000000100_000001000_000010000,
	0b000000000_000000000_000000000_000000001_000000010_000000100_000001000_000010000_000100000,
	0b000000000_000000000_000000001_000000010_000000100_000001000_000010000_000100000_001000000,
	0b000000000_000000001_000000010_000000100_000001000_000010000_000100000_001000000_010000000,
	0b000000001_000000010_000000100_000001000_000010000_000100000_001000000_010000000_100000000,
	0b000000010_000000100_000001000_000010000_000100000_001000000_010000000_100000000_000000000,
	0b000000100_000001000_000010000_000100000_001000000_010000000_100000000_000000000_000000000,
	0b000001000_000010000_000100000_001000000_010000000_100000000_000000000_000000000_000000000,
	0b000010000_000100000_001000000_010000000_100000000_000000000_000000000_000000000_000000000,
	0b000100000_001000000_010000000_100000000_000000000_000000000_000000000_000000000_000000000,
	0b001000000_010000000_100000000_000000000_000000000_000000000_000000000_000000000_000000000,
	0b010000000_000000001_000000000_000000000_000000000_000000000_000000000_000000000_000000000,
	0b100000000_000000000_000000000_000000000_000000000_000000000_000000000_000000000_000000000,
];
const TO_RIGHT_BOTTOM_MASK:[u128;17] = [
	0b100000000_010000000_001000000_000100000_000010000_000001000_000000100_000000010_000000001,
	0b000000000_100000000_010000000_001000000_000100000_000010000_000001000_000000100_000000010,
	0b000000000_000000000_100000000_010000000_001000000_000100000_000010000_000001000_000000100,
	0b000000000_000000000_000000000_100000000_010000000_001000000_000100000_000010000_000001000,
	0b000000000_000000000_000000000_000000000_100000000_010000000_001000000_000100000_000010000,
	0b000000000_000000000_000000000_000000000_000000000_100000000_010000000_001000000_000100000,
	0b000000000_000000000_000000000_000000000_000000000_000000000_100000000_010000000_001000000,
	0b000000000_000000000_000000000_000000000_000000000_000000000_000000000_100000000_010000000,
	0b000000000_000000000_000000000_000000000_000000000_000000000_000000000_000000000_100000000,
	0b010000000_001000000_000100000_000010000_000001000_000000100_000000010_000000001_000000000,
	0b001000000_000100000_000010000_000001000_000000100_000000010_000000001_000000000_000000000,
	0b000100000_000010000_000001000_000000100_000000010_000000001_000000000_000000000_000000000,
	0b000010000_000001000_000000100_000000010_000000001_000000000_000000000_000000000_000000000,
	0b000001000_000000100_000000010_000000001_000000000_000000000_000000000_000000000_000000000,
	0b000000100_000000010_000000001_000000000_000000000_000000000_000000000_000000000_000000000,
	0b000000010_000000001_000000000_000000000_000000000_000000000_000000000_000000000_000000000,
	0b000000001_000000000_000000000_000000000_000000000_000000000_000000000_000000000_000000000,
];
const KAKU_TO_RIGHT_TOP_MASK_MAP:[u128;81] = [
	TO_RIGHT_TOP_MASK[0],
	TO_RIGHT_TOP_MASK[1],
	TO_RIGHT_TOP_MASK[2],
	TO_RIGHT_TOP_MASK[3],
	TO_RIGHT_TOP_MASK[4],
	TO_RIGHT_TOP_MASK[5],
	TO_RIGHT_TOP_MASK[6],
	TO_RIGHT_TOP_MASK[7],
	TO_RIGHT_TOP_MASK[8],

	TO_RIGHT_TOP_MASK[1],
	TO_RIGHT_TOP_MASK[2],
	TO_RIGHT_TOP_MASK[3],
	TO_RIGHT_TOP_MASK[4],
	TO_RIGHT_TOP_MASK[5],
	TO_RIGHT_TOP_MASK[6],
	TO_RIGHT_TOP_MASK[7],
	TO_RIGHT_TOP_MASK[8],
	TO_RIGHT_TOP_MASK[9],

	TO_RIGHT_TOP_MASK[2],
	TO_RIGHT_TOP_MASK[3],
	TO_RIGHT_TOP_MASK[4],
	TO_RIGHT_TOP_MASK[5],
	TO_RIGHT_TOP_MASK[6],
	TO_RIGHT_TOP_MASK[7],
	TO_RIGHT_TOP_MASK[8],
	TO_RIGHT_TOP_MASK[9],
	TO_RIGHT_TOP_MASK[10],

	TO_RIGHT_TOP_MASK[3],
	TO_RIGHT_TOP_MASK[4],
	TO_RIGHT_TOP_MASK[5],
	TO_RIGHT_TOP_MASK[6],
	TO_RIGHT_TOP_MASK[7],
	TO_RIGHT_TOP_MASK[8],
	TO_RIGHT_TOP_MASK[9],
	TO_RIGHT_TOP_MASK[10],
	TO_RIGHT_TOP_MASK[11],

	TO_RIGHT_TOP_MASK[4],
	TO_RIGHT_TOP_MASK[5],
	TO_RIGHT_TOP_MASK[6],
	TO_RIGHT_TOP_MASK[7],
	TO_RIGHT_TOP_MASK[8],
	TO_RIGHT_TOP_MASK[9],
	TO_RIGHT_TOP_MASK[10],
	TO_RIGHT_TOP_MASK[11],
	TO_RIGHT_TOP_MASK[12],

	TO_RIGHT_TOP_MASK[5],
	TO_RIGHT_TOP_MASK[6],
	TO_RIGHT_TOP_MASK[7],
	TO_RIGHT_TOP_MASK[8],
	TO_RIGHT_TOP_MASK[9],
	TO_RIGHT_TOP_MASK[10],
	TO_RIGHT_TOP_MASK[11],
	TO_RIGHT_TOP_MASK[12],
	TO_RIGHT_TOP_MASK[13],

	TO_RIGHT_TOP_MASK[6],
	TO_RIGHT_TOP_MASK[7],
	TO_RIGHT_TOP_MASK[8],
	TO_RIGHT_TOP_MASK[9],
	TO_RIGHT_TOP_MASK[10],
	TO_RIGHT_TOP_MASK[11],
	TO_RIGHT_TOP_MASK[12],
	TO_RIGHT_TOP_MASK[13],
	TO_RIGHT_TOP_MASK[14],

	TO_RIGHT_TOP_MASK[7],
	TO_RIGHT_TOP_MASK[8],
	TO_RIGHT_TOP_MASK[9],
	TO_RIGHT_TOP_MASK[10],
	TO_RIGHT_TOP_MASK[11],
	TO_RIGHT_TOP_MASK[12],
	TO_RIGHT_TOP_MASK[13],
	TO_RIGHT_TOP_MASK[14],
	TO_RIGHT_TOP_MASK[15],

	TO_RIGHT_TOP_MASK[8],
	TO_RIGHT_TOP_MASK[9],
	TO_RIGHT_TOP_MASK[10],
	TO_RIGHT_TOP_MASK[11],
	TO_RIGHT_TOP_MASK[12],
	TO_RIGHT_TOP_MASK[13],
	TO_RIGHT_TOP_MASK[14],
	TO_RIGHT_TOP_MASK[15],
	TO_RIGHT_TOP_MASK[16],
];
const KAKU_TO_RIGHT_BOTTOM_MASK_MAP:[u128;81] = [
	TO_RIGHT_BOTTOM_MASK[0],
	TO_RIGHT_BOTTOM_MASK[1],
	TO_RIGHT_BOTTOM_MASK[2],
	TO_RIGHT_BOTTOM_MASK[3],
	TO_RIGHT_BOTTOM_MASK[4],
	TO_RIGHT_BOTTOM_MASK[5],
	TO_RIGHT_BOTTOM_MASK[6],
	TO_RIGHT_BOTTOM_MASK[7],
	TO_RIGHT_BOTTOM_MASK[8],

	TO_RIGHT_BOTTOM_MASK[9],
	TO_RIGHT_BOTTOM_MASK[0],
	TO_RIGHT_BOTTOM_MASK[1],
	TO_RIGHT_BOTTOM_MASK[2],
	TO_RIGHT_BOTTOM_MASK[3],
	TO_RIGHT_BOTTOM_MASK[4],
	TO_RIGHT_BOTTOM_MASK[5],
	TO_RIGHT_BOTTOM_MASK[6],
	TO_RIGHT_BOTTOM_MASK[7],

	TO_RIGHT_BOTTOM_MASK[10],
	TO_RIGHT_BOTTOM_MASK[9],
	TO_RIGHT_BOTTOM_MASK[0],
	TO_RIGHT_BOTTOM_MASK[1],
	TO_RIGHT_BOTTOM_MASK[2],
	TO_RIGHT_BOTTOM_MASK[3],
	TO_RIGHT_BOTTOM_MASK[4],
	TO_RIGHT_BOTTOM_MASK[5],
	TO_RIGHT_BOTTOM_MASK[6],

	TO_RIGHT_BOTTOM_MASK[11],
	TO_RIGHT_BOTTOM_MASK[10],
	TO_RIGHT_BOTTOM_MASK[9],
	TO_RIGHT_BOTTOM_MASK[0],
	TO_RIGHT_BOTTOM_MASK[1],
	TO_RIGHT_BOTTOM_MASK[2],
	TO_RIGHT_BOTTOM_MASK[3],
	TO_RIGHT_BOTTOM_MASK[4],
	TO_RIGHT_BOTTOM_MASK[5],

	TO_RIGHT_BOTTOM_MASK[12],
	TO_RIGHT_BOTTOM_MASK[11],
	TO_RIGHT_BOTTOM_MASK[10],
	TO_RIGHT_BOTTOM_MASK[9],
	TO_RIGHT_BOTTOM_MASK[0],
	TO_RIGHT_BOTTOM_MASK[1],
	TO_RIGHT_BOTTOM_MASK[2],
	TO_RIGHT_BOTTOM_MASK[3],
	TO_RIGHT_BOTTOM_MASK[4],

	TO_RIGHT_BOTTOM_MASK[13],
	TO_RIGHT_BOTTOM_MASK[12],
	TO_RIGHT_BOTTOM_MASK[11],
	TO_RIGHT_BOTTOM_MASK[10],
	TO_RIGHT_BOTTOM_MASK[9],
	TO_RIGHT_BOTTOM_MASK[0],
	TO_RIGHT_BOTTOM_MASK[1],
	TO_RIGHT_BOTTOM_MASK[2],
	TO_RIGHT_BOTTOM_MASK[3],

	TO_RIGHT_BOTTOM_MASK[14],
	TO_RIGHT_BOTTOM_MASK[13],
	TO_RIGHT_BOTTOM_MASK[12],
	TO_RIGHT_BOTTOM_MASK[11],
	TO_RIGHT_BOTTOM_MASK[10],
	TO_RIGHT_BOTTOM_MASK[9],
	TO_RIGHT_BOTTOM_MASK[0],
	TO_RIGHT_BOTTOM_MASK[1],
	TO_RIGHT_BOTTOM_MASK[2],

	TO_RIGHT_BOTTOM_MASK[15],
	TO_RIGHT_BOTTOM_MASK[14],
	TO_RIGHT_BOTTOM_MASK[13],
	TO_RIGHT_BOTTOM_MASK[12],
	TO_RIGHT_BOTTOM_MASK[11],
	TO_RIGHT_BOTTOM_MASK[10],
	TO_RIGHT_BOTTOM_MASK[9],
	TO_RIGHT_BOTTOM_MASK[0],
	TO_RIGHT_BOTTOM_MASK[1],

	TO_RIGHT_BOTTOM_MASK[16],
	TO_RIGHT_BOTTOM_MASK[15],
	TO_RIGHT_BOTTOM_MASK[14],
	TO_RIGHT_BOTTOM_MASK[13],
	TO_RIGHT_BOTTOM_MASK[12],
	TO_RIGHT_BOTTOM_MASK[11],
	TO_RIGHT_BOTTOM_MASK[10],
	TO_RIGHT_BOTTOM_MASK[9],
	TO_RIGHT_BOTTOM_MASK[0],
];
const V_MASK: u128 = 0b111111111;
const H_MASK: u128 = 0b000000001_000000001_000000001_000000001_000000001_000000001_000000001_000000001_000000001;
const TOP_MASK: u128 = 0b111111100_111111100_111111100;
const BOTTOM_MASK: u128 = 0b000000111_000000111_000000111;
const RIGHT_MASK: u128 = 0b000000000_111111111_111111111;
const SENTE_NARI_MASK: u128 = 0b000000111_000000111_000000111_000000111_000000111_000000111_000000111_000000111_000000111;
const GOTE_NARI_MASK: u128 = 0b111000000_111000000_111000000_111000000_111000000_111000000_111000000_111000000_111000000;
const DENY_MOVE_SENTE_FU_AND_KYOU_MASK: u128 = 0b000000001_000000001_000000001_000000001_000000001_000000001_000000001_000000001_000000001;
const DENY_MOVE_SENTE_KEI_MASK: u128 = 0b000000011_000000011_000000011_000000011_000000011_000000011_000000011_000000011_000000011;
const DENY_MOVE_GOTE_FU_AND_KYOU_MASK: u128 = 0b100000000_100000000_100000000_100000000_100000000_100000000_100000000_100000000_100000000;
const DENY_MOVE_GOTE_KEI_MASK: u128 = 0b110000000_110000000_110000000_110000000_110000000_110000000_110000000_110000000_110000000;
const BANMEN_MASK: u128 = 0b111111111_111111111_111111111_111111111_111111111_111111111_111111111_111111111_111111111_0;
const NYUGYOKU_MASK:u128 = 0b111000000_111000000_111000000_111000000_111000000_111000000_111000000_111000000_111000000;
/// 左上を(0,0)とした平手初期局面
pub const BANMEN_START_POS:Banmen = Banmen([
	[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
	[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
	[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
	[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
	[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
	[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
	[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
	[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
	[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
]);
/// オブジェクトの状態の検証用
pub trait Validate {
	/// 状態が正しければtrueを、そうでなければfalseを返す
	fn validate(&self) -> bool;
}
impl Validate for KomaSrcPosition {
	fn validate(&self) -> bool {
		match *self {
			KomaSrcPosition(x, y) => x > 0 && x <= 9 && y > 0 && y <= 9,
		}
	}
}
impl Validate for KomaDstToPosition {
	fn validate(&self) -> bool {
		match *self {
			KomaDstToPosition(x, y, _) => x > 0 && x <= 9 && y > 0 && y <= 9,
		}
	}
}
impl Validate for KomaDstPutPosition {
	fn validate(&self) -> bool {
		match *self {
			KomaDstPutPosition(x, y) => x > 0 && x <= 9 && y > 0 && y <= 9,
		}
	}
}
impl Validate for Move {
	fn validate(&self) -> bool {
		match *self {
			Move::To(ref s, ref d) => s.validate() && d.validate(),
			Move::Put(_, ref d) => d.validate()
		}
	}
}
impl Validate for Moved {
	fn validate(&self) -> bool {
		match self {
			&Moved::To(_,(sx,sy),(dx,dy),_) if sx < 1 || sx > 9 || sy < 1 || sy > 9 || dx < 1 || dx > 9 || dy < 1 || dy > 9 => {
				return false;
			},
			&Moved::Put(_,(x,y)) if x < 1 || x > 9 || y < 1 || y > 9 => {
				return false;
			},
			_ => ()
		};

		match self {
			&Moved::Put(_,_) |
			&Moved::To(MovedKind::Fu,_,_,true) |
			&Moved::To(MovedKind::Kyou,_,_,true) |
			&Moved::To(MovedKind::Kei,_,_,true) |
			&Moved::To(MovedKind::Gin,_,_,true)|
			&Moved::To(MovedKind::Kin,_,_,false) |
			&Moved::To(MovedKind::SOu,_,_,false) |
			&Moved::To(MovedKind::GOu,_,_,false) |
			&Moved::To(MovedKind::Kaku,_,_,true) |
			&Moved::To(MovedKind::Hisha,_,_,true) |
			&Moved::To(MovedKind::Fu,_,_,false) |
			&Moved::To(MovedKind::Kyou,_,_,false) |
			&Moved::To(MovedKind::Kei,_,_,false) |
			&Moved::To(MovedKind::Gin,_,_,false) |
			&Moved::To(MovedKind::Kaku,_,_,false) |
			&Moved::To(MovedKind::Hisha,_,_,false) |
			&Moved::To(MovedKind::FuN,_,_,false) |
			&Moved::To(MovedKind::KyouN,_,_,false) |
			&Moved::To(MovedKind::KeiN,_,_,false) |
			&Moved::To(MovedKind::GinN,_,_,false) |
			&Moved::To(MovedKind::KakuN,_,_,false) |
			&Moved::To(MovedKind::HishaN,_,_,false) => {
				true
			},
			&Moved::To(MovedKind::Kin,_,_,true) |
			&Moved::To(MovedKind::SOu,_,_,true) |
			&Moved::To(MovedKind::GOu,_,_,true) |
			&Moved::To(MovedKind::FuN,_,_,true) |
			&Moved::To(MovedKind::KyouN,_,_,true) |
			&Moved::To(MovedKind::KeiN,_,_,true) |
			&Moved::To(MovedKind::GinN,_,_,true) |
			&Moved::To(MovedKind::KakuN,_,_,true) |
			&Moved::To(MovedKind::HishaN,_,_,true) |
			&Moved::To(MovedKind::Blank,_,_,_) => {
				false
			}
		}
	}
}
/*
enum Inverse {
	True = 0,
	False = 1
}
*/
/// 合法手の列挙等を行う将棋のルールを管理
pub struct Rule {

}
impl Rule {
	/// 盤面上の駒を移動する合法手をビットボードに列挙
	///
	/// # Arguments
	/// * `teban` - 手を列挙したい手番
	/// * `self_occupied` - 手番側から見た手番側の駒の配置を表すビットボード。(後手の場合は上下逆さになっている)
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される駒の移動元の位置。常に先手側から見た位置になる（後手の場合も逆さまにならない）
	/// * `kind` - 移動する駒の種類
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	#[inline]
	pub fn gen_candidate_bits(
		teban:Teban,self_occupied:BitBoard,from:u32,kind:KomaKind
	) -> BitBoard {
		let from = if teban == Teban::Sente {
			from
		} else {
			80 - from
		};

		let (x,y) = from.square_to_point();

		let mut mask = if kind < GFu {
			CANDIDATE_BITS[kind as usize]
		} else if kind < Blank {
			CANDIDATE_BITS[kind as usize - 14]
		} else {
			return BitBoard { merged_bitboard: 0 };
		};

		if y == 0 || ((kind == SKei || kind == GKei) && y <= 1) {
			mask = mask & TOP_MASK;
		} else if y == 8 {
			mask = mask & BOTTOM_MASK;
		}

		if x == 8 {
			mask = mask & RIGHT_MASK;
		}

		let mask = mask as u128;
		let self_occupied = unsafe { self_occupied.merged_bitboard };

		let mut board = !self_occupied & !1;

		if from < 10 {
			board &= mask >> (11 - from - 1);
		} else if from == 10 {
			board &= mask;
		} else {
			board &= mask << (from - 11 + 1);
		}

		BitBoard { merged_bitboard: board }
	}

	/// 合法手をバッファに追加
	///
	/// # Arguments
	/// * `m` - 盤面の左上を0,0とし、x * 9 + yで表される移動先の駒の位置。後手の手の場合は上下さかさまになっている
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 移動する駒の種類
	/// * `nari_mask` - ビットボードを用いて移動先で駒が成れるか判定するためのマスク
	/// * `deny_move_mask` - ビットボードを用いて移動先で駒が成らなくても合法手か判定するためのマスク
	/// * `inverse_position` - ビットボードを上下逆さにするか否か
	/// * `move_builder` - LegalMoveを生成するためのコールバック
	/// * `mvs` - 手を追加するバッファ
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	pub fn append_legal_moves_from_banmen<F>(
		m:Square,
		from:u32,
		kind:KomaKind,
		nari_mask:u128,
		deny_move_mask:u128,
		inverse_position:bool,
		move_builder:&F,
		mvs:&mut Vec<LegalMove>
	) where F: Fn(u32,u32,bool) -> LegalMove {
		let to = m as u32;

		let to = if inverse_position {
			80 - to
		} else {
			to
		};

		let to_mask = 1 << to;
		let from_mask = 1 << from;

		let nari = kind.is_nari();

		if !nari && (nari_mask & to_mask != 0 || nari_mask & from_mask != 0) {
			mvs.push(move_builder(from, to, true));
		}

		if nari || deny_move_mask & to_mask == 0 {
			mvs.push(move_builder(from, to, false));
		}
	}

	/// 王を取る合法手をバッファに追加
	///
	/// # Arguments
	/// * `m` - 盤面の左上を0,0とし、x * 9 + yで表される移動先の駒の位置。後手の手の場合は上下さかさまになっている
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 移動する駒の種類
	/// * `nari_mask` - ビットボードを用いて移動先で駒が成れるか判定するためのマスク
	/// * `deny_move_mask` - ビットボードを用いて移動先で駒が成らなくても合法手か判定するためのマスク
	/// * `inverse_position` - ビットボードを上下逆さにするか否か
	/// * `mvs` - 手を追加するバッファ
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	pub fn append_win_only_move(
		m:Square,
		from:u32,
		kind:KomaKind,
		nari_mask:u128,
		deny_move_mask:u128,
		inverse_position:bool,
		mvs:&mut Vec<LegalMove>
	) {
		let to = m as u32;

		let to = if inverse_position {
			80 - to
		} else {
			to
		};

		let to_mask = 1 << to;
		let from_mask = 1 << from;

		let nari = kind.is_nari();

		let o = Some(ObtainKind::Ou);

		if !nari && (nari_mask & to_mask != 0 || nari_mask & from_mask != 0) {
			mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
		}

		if deny_move_mask & to_mask == 0 {
			mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
		}
	}

	/// 一マスだけ駒を動かす合法手を列挙してバッファに追加する
	///
	/// # Arguments
	/// * `teban` - 手を列挙したい手番
	/// * `self_occupied` - 手番側から見た手番側の駒の配置を表すビットボード。(後手の場合は上下逆さになっている)
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 移動する駒の種類
	/// * `nari_mask` - ビットボードを用いて移動先で駒が成れるか判定するためのマスク
	/// * `deny_move_mask` - ビットボードを用いて移動先で駒が成らなくても合法手か判定するためのマスク
	/// * `inverse_position` - ビットボードを上下逆さにするか否か
	/// * `move_builder` - LegalMoveを生成するためのコールバック
	/// * `mvs` - 手を追加するバッファ
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	pub fn legal_moves_once_with_point_and_kind_and_bitboard_and_buffer<F>(
		teban:Teban,
		self_occupied:BitBoard,
		from:u32,kind:KomaKind,
		nari_mask:u128,
		deny_move_mask:u128,
		inverse_position:bool,
		move_builder:&F,
		mvs:&mut Vec<LegalMove>
	) where F: Fn(u32,u32,bool) -> LegalMove {
		let mut board = Rule::gen_candidate_bits(teban, self_occupied, from, kind);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,inverse_position,move_builder,mvs
				);
			}
		}
	}

	/// 一マスだけ駒を動かす合法手を列挙して返す
	///
	/// # Arguments
	/// * `teban` - 手を列挙したい手番
	/// * `self_occupied` - 手番側から見た手番側の駒の配置を表すビットボード。(後手の場合は上下逆さになっている)
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 移動する駒の種類
	/// * `nari_mask` - ビットボードを用いて移動先で駒が成れるか判定するためのマスク
	/// * `deny_move_mask` - ビットボードを用いて移動先で駒が成らなくても合法手か判定するためのマスク
	/// * `inverse_position` - ビットボードを上下逆さにするか否か
	/// * `move_builder` - LegalMoveを生成するためのコールバック
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	#[inline]
	pub fn legal_moves_once_with_point_and_kind_and_bitboard<F>(
		teban:Teban,
		self_occupied:BitBoard,
		from:u32,kind:KomaKind,
		nari_mask:u128,
		deny_move_mask:u128,
		inverse_position:bool,
		move_builder:&F
	) -> Vec<LegalMove> where F: Fn(u32,u32,bool) -> LegalMove {
		let mut mvs:Vec<LegalMove> = Vec::with_capacity(8);

		Rule::legal_moves_once_with_point_and_kind_and_bitboard_and_buffer(
			teban,self_occupied,from,kind,nari_mask,deny_move_mask,inverse_position,move_builder,&mut mvs);

		mvs
	}

	#[inline]
	fn gen_candidate_bits_by_kaku_to_right_top_with_exclude(
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		exclude:BitBoard,
		from:u32
	) -> BitBoard {
		let mut occ = unsafe { self_occupied.merged_bitboard | opponent_occupied.merged_bitboard };

		let mask = KAKU_TO_RIGHT_TOP_MASK_MAP[from as usize] << 1;

		occ = occ & mask;

		// ビットボードの最初の1ビット目は盤面の範囲外でfrom+1を引いた値を求めたいので4をビットシフトする
		let candidate = (((occ.wrapping_sub(4 << from)) ^ occ) & mask) & !unsafe { exclude.merged_bitboard };

		BitBoard {
			merged_bitboard: candidate
		}
	}

	/// 盤面上の駒を移動する合法手のうち、角の右上に移動する手をビットボードに列挙
	///
	/// # Arguments
	/// * `self_occupied` - 手番側の駒の配置を表すビットボード。盤面の上もしくは左方向へ移動するときは逆さまにひっくり返した配置になる。
	/// * `opponent_occupied` - 相手側の駒の配置を表すビットボード。盤面の上もしくは左方向へ移動するときは逆さまにひっくり返した配置になる。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される駒の移動元の位置。盤面の上もしくは左方向へ移動するときは逆さまにした時の位置になる。
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	#[inline]
	pub fn gen_candidate_bits_by_kaku_to_right_top(
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		from:u32
	) -> BitBoard {
		Rule::gen_candidate_bits_by_kaku_to_right_top_with_exclude(self_occupied,opponent_occupied,self_occupied,from)
	}

	#[inline]
	fn gen_candidate_bits_by_kaku_to_right_bottom_with_exclude(
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		exclude:BitBoard,
		from:u32
	) -> BitBoard {
		let mut occ = unsafe { self_occupied.merged_bitboard | opponent_occupied.merged_bitboard };

		let mask = KAKU_TO_RIGHT_BOTTOM_MASK_MAP[from as usize] << 1;

		occ = occ & mask;

		// ビットボードの最初の1ビット目は盤面の範囲外でfrom+1を引いた値を求めたいので4をビットシフトする
		let candidate = (((occ.wrapping_sub(4 << from)) ^ occ) & mask) & !unsafe { exclude.merged_bitboard };

		BitBoard {
			merged_bitboard: candidate
		}
	}

	/// 盤面上の駒を移動する合法手のうち、角の右下に移動する手をビットボードに列挙
	///
	/// # Arguments
	/// * `self_occupied` - 手番側の駒の配置を表すビットボード。盤面の上もしくは左方向へ移動するときは逆さまにひっくり返した配置になる。
	/// * `opponent_occupied` - 相手側の駒の配置を表すビットボード。盤面の上もしくは左方向へ移動するときは逆さまにひっくり返した配置になる。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される駒の移動元の位置。盤面の上もしくは左方向へ移動するときは逆さまにした時の位置になる。
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	#[inline]
	pub fn gen_candidate_bits_by_kaku_to_right_bottom(
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		from:u32
	) -> BitBoard {
		Rule::gen_candidate_bits_by_kaku_to_right_bottom_with_exclude(self_occupied,opponent_occupied,self_occupied,from)
	}

	/// 先手の角の合法手を列挙してバッファに追加
	///
	/// # Arguments
	/// * `sente_self_occupied` - 先手側から見た先手側の駒の配置を表すビットボード。
	/// * `sente_opponent_occupied` - 先手側から見た後手側の駒の配置を表すビットボード。
	/// * `flip_self_occupied` - 後手側から見た後手側の駒の配置を表すビットボード。
	/// * `flip_opponent_occupied` - 後手側から見た先手側の駒の配置を表すビットボード。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 駒の種類
	/// * `nari_mask` - ビットボードを用いて移動先で駒が成れるか判定するためのマスク
	/// * `deny_move_mask` - ビットボードを用いて移動先で駒が成らなくても合法手か判定するためのマスク
	/// * `move_builder` - LegalMoveを生成するためのコールバック
	/// * `mvs` - 手を追加するバッファ
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	pub fn legal_moves_sente_kaku_with_point_and_kind_and_bitboard_and_buffer<F>(
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		from:u32,kind:KomaKind,
		nari_mask:u128,
		deny_move_mask:u128,
		move_builder:&F,
		mvs:&mut Vec<LegalMove>
	) where F: Fn(u32,u32,bool) -> LegalMove {
		let mut board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(flip_opponent_occupied,flip_self_occupied,80 - from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,true,move_builder,mvs
				);
			}
		}

		let mut board = Rule::gen_candidate_bits_by_kaku_to_right_top(self_occupied,opponent_occupied,from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,false,move_builder,mvs
				);
			}
		}

		let mut board = Rule::gen_candidate_bits_by_kaku_to_right_top(flip_opponent_occupied,flip_self_occupied,80 - from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,true,move_builder,mvs
				);
			}
		}

		let mut board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(self_occupied,opponent_occupied,from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,false,move_builder,mvs
				);
			}
		}

		if kind == SKakuN {
			Rule::legal_moves_once_with_point_and_kind_and_bitboard_and_buffer(
				Teban::Sente,self_occupied,from,kind,nari_mask,deny_move_mask,false,move_builder,mvs
			);
		}
	}

	/// 後手の角の合法手を列挙してバッファに追加
	///
	/// # Arguments
	/// * `self_occupied` - 後手側から見た後手側の駒の配置を表すビットボード。
	/// * `opponent_occupied` - 後手側から見た先手側の駒の配置を表すビットボード。
	/// * `flip_self_occupied` - 先手側から見た先手側の駒の配置を表すビットボード。
	/// * `flip_opponent_occupied` - 先手側から見た後手側の駒の配置を表すビットボード。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 駒の種類
	/// * `nari_mask` - ビットボードを用いて移動先で駒が成れるか判定するためのマスク
	/// * `deny_move_mask` - ビットボードを用いて移動先で駒が成らなくても合法手か判定するためのマスク
	/// * `move_builder` - LegalMoveを生成するためのコールバック
	/// * `mvs` - 手を追加するバッファ
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	pub fn legal_moves_gote_kaku_with_point_and_kind_and_bitboard_and_buffer<F>(
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		from:u32,kind:KomaKind,
		nari_mask:u128,
		deny_move_mask:u128,
		move_builder:&F,
		mvs:&mut Vec<LegalMove>
	) where F: Fn(u32,u32,bool) -> LegalMove {
		let mut board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(flip_opponent_occupied,flip_self_occupied,from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,false,move_builder,mvs
				);
			}
		}

		let mut board = Rule::gen_candidate_bits_by_kaku_to_right_top(self_occupied,opponent_occupied,80 - from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,true,move_builder,mvs
				);
			}
		}

		let mut board = Rule::gen_candidate_bits_by_kaku_to_right_top(flip_opponent_occupied,flip_self_occupied,from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,false,move_builder,mvs
				);
			}
		}

		let mut board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(self_occupied,opponent_occupied,80 - from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,true,move_builder,mvs
				);
			}
		}

		if kind == GKakuN {
			Rule::legal_moves_once_with_point_and_kind_and_bitboard_and_buffer(
				Teban::Gote,self_occupied,from,kind,nari_mask,deny_move_mask,true,move_builder,mvs
			);
		}
	}

	#[inline]
	fn gen_candidate_bits_by_hisha_or_kyou_to_top_with_exclude(
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		exclude:BitBoard,
		from:u32
	) -> BitBoard {
		let occ = unsafe { flip_self_occupied.merged_bitboard | flip_opponent_occupied.merged_bitboard };

		let x = from / 9;

		let mask = V_MASK << (x * 9 + 1);

		// ビットボードの最初の1ビット目は盤面の範囲外でfrom+1を引いた値を求めたいので4をビットシフトする
		let candidate = (((occ.wrapping_sub(4 << from)) ^ occ) & mask) & !unsafe { exclude.merged_bitboard };

		BitBoard {
			merged_bitboard: candidate
		}
	}

	/// 盤面上の駒を移動する合法手のうち、飛車と香車の上に移動する手をビットボードに列挙
	/// 反転したビットボードを渡す仕様のため返されたビットボードから列挙される手は盤面をひっくり返した時の形になっていることに注意。
	///
	/// # Arguments
	/// * `flip_self_occupied` - 手番側の駒の配置を表すビットボード。盤面の上もしくは左方向へ移動するときは逆さまにひっくり返した配置になる。
	/// * `flip_opponent_occupied` - 相手番側の駒の配置を表すビットボード。盤面の上もしくは左方向へ移動するときは逆さまにひっくり返した配置になる。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される駒の移動元の位置。盤面の上もしくは左方向へ移動するときは逆さまにした時の位置になる。
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	#[inline]
	pub fn gen_candidate_bits_by_hisha_or_kyou_to_top(
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		from:u32
	) -> BitBoard {
		Rule::gen_candidate_bits_by_hisha_or_kyou_to_top_with_exclude(flip_self_occupied,flip_opponent_occupied,flip_self_occupied,from)
	}

	#[inline]
	fn gen_candidate_bits_by_hisha_to_right_with_exclude(
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		exclude:BitBoard,
		from:u32
	) -> BitBoard {
		let mut occ = unsafe { self_occupied.merged_bitboard | opponent_occupied.merged_bitboard };

		let x = from * 114 / 1024;

		let mask = H_MASK << (from - 9 * x + 1);

		occ = occ & mask;

		// ビットボードの最初の1ビット目は盤面の範囲外でfrom+1を引いた値を求めたいので4をビットシフトする
		let candidate = (((occ.wrapping_sub(4 << from)) ^ occ) & mask) & !unsafe { exclude.merged_bitboard };

		BitBoard {
			merged_bitboard: candidate
		}
	}

	/// 盤面上の駒を移動する合法手のうち、飛車と香車の右に移動する手をビットボードに列挙
	///
	/// # Arguments
	/// * `self_occupied` - 手番側の駒の配置を表すビットボード。盤面の上もしくは左方向へ移動するときは逆さまにひっくり返した配置になる。
	/// * `opponent_occupied` - 相手側の駒の配置を表すビットボード。盤面の上もしくは左方向へ移動するときは逆さまにひっくり返した配置になる。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される駒の移動元の位置。盤面の上もしくは左方向へ移動するときは逆さまにした時の位置になる。
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	#[inline]
	pub fn gen_candidate_bits_by_hisha_to_right(
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		from:u32
	) -> BitBoard {
		Rule::gen_candidate_bits_by_hisha_to_right_with_exclude(self_occupied,opponent_occupied,self_occupied,from)
	}

	/// 先手の飛車の合法手を列挙してバッファに追加
	///
	/// # Arguments
	/// * `self_occupied` - 先手側から見た先手側の駒の配置を表すビットボード。
	/// * `opponent_occupied` - 先手側から見た後手側の駒の配置を表すビットボード。
	/// * `flip_self_occupied` - 後手側から見た後手側の駒の配置を表すビットボード。
	/// * `flip_opponent_occupied` - 後手側から見た先手側の駒の配置を表すビットボード。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 駒の種類
	/// * `nari_mask` - ビットボードを用いて移動先で駒が成れるか判定するためのマスク
	/// * `deny_move_mask` - ビットボードを用いて移動先で駒が成らなくても合法手か判定するためのマスク
	/// * `move_builder` - LegalMoveを生成するためのコールバック
	/// * `mvs` - 手を追加するバッファ
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	pub fn legal_moves_sente_hisha_with_point_and_kind_and_bitboard_and_buffer<F>(
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		from:u32,kind:KomaKind,
		nari_mask:u128,
		deny_move_mask:u128,
		move_builder:&F,
		mvs:&mut Vec<LegalMove>
	) where F: Fn(u32,u32,bool) -> LegalMove {
		let mut board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(flip_opponent_occupied,flip_self_occupied, 80 - from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,true,move_builder,mvs
				);
			}
		}

		let mut board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(self_occupied, opponent_occupied, from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,false,move_builder,mvs
				);
			}
		}

		let mut board = Rule::gen_candidate_bits_by_hisha_to_right(flip_opponent_occupied,flip_self_occupied,80 - from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,true,move_builder,mvs
				);
			}
		}

		let mut board = Rule::gen_candidate_bits_by_hisha_to_right(self_occupied,opponent_occupied,from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,false,move_builder,mvs
				);
			}
		}

		if kind == SHishaN {
			Rule::legal_moves_once_with_point_and_kind_and_bitboard_and_buffer(
				Teban::Sente,self_occupied,from,kind,nari_mask,deny_move_mask,false,move_builder,mvs
			);
		}
	}

	/// 後手の飛車の合法手を列挙してバッファに追加
	///
	/// # Arguments
	/// * `self_occupied` - 後手側から見た後手側の駒の配置を表すビットボード。
	/// * `opponent_occupied` - 後手側から見た先手側の駒の配置を表すビットボード。
	/// * `flip_self_occupied` - 先手側から見た先手側の駒の配置を表すビットボード。
	/// * `flip_opponent_occupied` - 先手側から見た後手側の駒の配置を表すビットボード。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 駒の種類
	/// * `nari_mask` - ビットボードを用いて移動先で駒が成れるか判定するためのマスク
	/// * `deny_move_mask` - ビットボードを用いて移動先で駒が成らなくても合法手か判定するためのマスク
	/// * `move_builder` - LegalMoveを生成するためのコールバック
	/// * `mvs` - 手を追加するバッファ
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	pub fn legal_moves_gote_hisha_with_point_and_kind_and_bitboard_and_buffer<F>(
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		from:u32,kind:KomaKind,
		nari_mask:u128,
		deny_move_mask:u128,
		move_builder:&F,
		mvs:&mut Vec<LegalMove>
	) where F: Fn(u32,u32,bool) -> LegalMove {
		let mut board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(flip_opponent_occupied,flip_self_occupied, from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,false,move_builder,mvs
				);
			}
		}

		let mut board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(self_occupied, opponent_occupied, 80 - from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,true,move_builder,mvs
				);
			}
		}

		let mut board = Rule::gen_candidate_bits_by_hisha_to_right(flip_opponent_occupied,flip_self_occupied,from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,false,move_builder,mvs
				);
			}
		}

		let mut board = Rule::gen_candidate_bits_by_hisha_to_right(self_occupied,opponent_occupied,80 - from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,kind,nari_mask,deny_move_mask,true,move_builder,mvs
				);
			}
		}

		if kind == GHishaN {
			Rule::legal_moves_once_with_point_and_kind_and_bitboard_and_buffer(
				Teban::Gote,self_occupied,from,kind,nari_mask,deny_move_mask,true,move_builder,mvs
			);
		}
	}

	/// 先手の香車の合法手を列挙してバッファに追加
	///
	/// # Arguments
	/// * `flip_self_occupied` - 後手側から見た後手側の駒の配置を表すビットボード。
	/// * `flip_opponent_occupied` - 後手側から見た先手側の駒の配置を表すビットボード。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `nari_mask` - ビットボードを用いて移動先で駒が成れるか判定するためのマスク
	/// * `deny_move_mask` - ビットボードを用いて移動先で駒が成らなくても合法手か判定するためのマスク
	/// * `move_builder` - LegalMoveを生成するためのコールバック
	/// * `mvs` - 手を追加するバッファ
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	pub fn legal_moves_sente_kyou_with_point_and_kind_and_bitboard_and_buffer<F>(
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		from:u32,
		nari_mask:u128,
		deny_move_mask:u128,
		move_builder:&F,
		mvs:&mut Vec<LegalMove>
	) where F: Fn(u32,u32,bool) -> LegalMove {
		let mut board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(flip_opponent_occupied,flip_self_occupied, 80 - from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,SKyou,nari_mask,deny_move_mask,true,move_builder,mvs
				);
			}
		}
	}

	/// 後手の香車の合法手を列挙してバッファに追加
	///
	/// # Arguments
	/// * `flip_self_occupied` - 先手側から見た先手側の駒の配置を表すビットボード。
	/// * `flip_opponent_occupied` - 先手側から見た後手側の駒の配置を表すビットボード。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `nari_mask` - ビットボードを用いて移動先で駒が成れるか判定するためのマスク
	/// * `deny_move_mask` - ビットボードを用いて移動先で駒が成らなくても合法手か判定するためのマスク
	/// * `move_builder` - LegalMoveを生成するためのコールバック
	/// * `mvs` - 手を追加するバッファ
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	pub fn legal_moves_gote_kyou_with_point_and_kind_and_bitboard_and_buffer<F>(
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		from:u32,
		nari_mask:u128,
		deny_move_mask:u128,
		move_builder:&F,
		mvs:&mut Vec<LegalMove>
	) where F: Fn(u32,u32,bool) -> LegalMove {
		let mut board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(flip_opponent_occupied,flip_self_occupied, from);

		loop {
			let p = Rule::pop_lsb(&mut board);
			if p == -1 {
				break;
			} else {
				Rule::append_legal_moves_from_banmen(
					p,from,GKyou,nari_mask,deny_move_mask,false,move_builder,mvs
				);
			}
		}
	}

	/// ビットボードの立っているビットのうち最下位のものの位置を返す
	///
	/// 呼出し後、最下位にあったビットは0に更新される
	#[inline]
	pub fn pop_lsb(bitboard:&mut BitBoard) -> Square {
		let (bl,br) = unsafe {
			match bitboard {
				BitBoard { bitboard } => {
					(*bitboard.get_unchecked(0),*bitboard.get_unchecked(1))
				}
			}
		};

		if bl != 0 {
			let p = bl.trailing_zeros() as Square;
			unsafe {
				*(bitboard.bitboard.get_unchecked_mut(0)) = bl & (bl - 1);
			}
			return p - 1;
		} else if br != 0 {
			let p = br.trailing_zeros() as Square;
			unsafe {
				*(bitboard.bitboard.get_unchecked_mut(1)) = br & (br - 1);
			}
			return p + 63;
		} else {
			return -1;
		}
	}

	/// 盤面上の位置と駒の種別を元に合法手を列挙して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `x` - 左上を0,0とする移動元のx座標
	/// * `y` - 左上を0,0とする移動元のy座標
	/// * `kind` - 移動する駒の種類
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	pub fn legal_moves_with_point_and_kind(
		t:Teban,state:&State,x:u32,y:u32,kind:KomaKind
	) -> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		Rule::legal_moves_with_point_and_kind_and_buffer(
			t,state,x,y,kind,&mut mvs
		);

		mvs
	}

	/// ビットボードから列挙された合法手の情報から`LegalMove`を生成して返す
	///
	/// # Arguments
	/// * `banmen` - 現在の盤面
	/// * `opponent_bitboard` - 相手の駒の配置を表すビットボード。常に先手視点
	pub fn default_moveto_builder<'a>(banmen:&'a Banmen,opponent_bitboard:u128) -> impl Fn(u32,u32,bool) -> LegalMove + 'a {
		move |from,to,nari| {
			let to_mask = 1 << (to + 1);

			let (dx,dy) = to.square_to_point();

			let o = match banmen {
				&Banmen(ref kinds) => {
					if opponent_bitboard & to_mask != 0 {
						ObtainKind::try_from(kinds[dy as usize][dx as usize]).ok()
					} else {
						None
					}
				}
			};

			LegalMove::To(LegalMoveTo::new(from,to,nari,o))
		}
	}

	/// 盤面上の位置と駒の種別を元に合法手を列挙してバッファに追加する
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `x` - 左上を0,0とする移動元のx座標
	/// * `y` - 左上を0,0とする移動元のy座標
	/// * `kind` - 移動する駒の種類
	/// * `mvs` - 手を追加するバッファ
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	pub fn legal_moves_with_point_and_kind_and_buffer(
		t:Teban,state:&State,
		x:u32,y:u32,kind:KomaKind,
		mvs:&mut Vec<LegalMove>
	) {
		let from = x * 9 + y;

		let (nari_mask,deny_move_mask) = match kind {
			SFu | SKyou => (SENTE_NARI_MASK,DENY_MOVE_SENTE_FU_AND_KYOU_MASK),
			SKei => (SENTE_NARI_MASK,DENY_MOVE_SENTE_KEI_MASK),
			SGin | SHisha | SKaku => (SENTE_NARI_MASK,0),
			GFu | GKyou => (GOTE_NARI_MASK,DENY_MOVE_GOTE_FU_AND_KYOU_MASK),
			GKei => (GOTE_NARI_MASK,DENY_MOVE_GOTE_KEI_MASK),
			GGin | GHisha | GKaku => (GOTE_NARI_MASK,0),
			SKin | SOu | SFuN | SKyouN | SKeiN | SGinN | SHishaN | SKakuN => {
				(0,0)
			},
			GKin | GOu | GFuN | GKyouN | GKeiN | GGinN | GHishaN | GKakuN => {
				(0,0)
			},
			Blank => {
				return;
			}
		};

		let (self_bitboard,opponent_bitboard) = if kind < GFu {
			(state.part.sente_self_board,
				unsafe { state.part.sente_opponent_board.merged_bitboard }
			)
		} else if kind < Blank {
			(state.part.gote_self_board,
				unsafe { state.part.sente_self_board.merged_bitboard }
			)
		} else {
			return;
		};

		match kind {
			SFu | SKei | SGin | SKin | SOu | SFuN | SKyouN | SKeiN | SGinN |
			GFu | GKei | GGin | GKin | GOu | GFuN | GKyouN | GKeiN | GGinN => {
				match t {
					Teban::Sente if kind >= GFu => {
						return;
					},
					Teban::Gote if kind < GFu => {
						return;
					},
					_ => (),
				}

				Rule::legal_moves_once_with_point_and_kind_and_bitboard_and_buffer(
					t,self_bitboard,from,kind,
					nari_mask,deny_move_mask,
					kind >= GFu && kind < Blank,
					&Rule::default_moveto_builder(&state.banmen,opponent_bitboard),
					mvs
				);
			},
			SKyou if t == Teban::Sente => {
				Rule::legal_moves_sente_kyou_with_point_and_kind_and_bitboard_and_buffer(
					state.part.gote_self_board,
					state.part.gote_opponent_board,
					from,
					nari_mask,deny_move_mask,
					&Rule::default_moveto_builder(&state.banmen,opponent_bitboard),
					mvs
				);
			}
			SKaku | SKakuN if t == Teban::Sente => {
				Rule::legal_moves_sente_kaku_with_point_and_kind_and_bitboard_and_buffer(
					state.part.sente_self_board,
					state.part.sente_opponent_board,
					state.part.gote_self_board,
					state.part.gote_opponent_board,
					from,kind,
					nari_mask,deny_move_mask,
					&Rule::default_moveto_builder(&state.banmen,opponent_bitboard),
					mvs
				);
			},
			SHisha | SHishaN if t == Teban::Sente => {
				Rule::legal_moves_sente_hisha_with_point_and_kind_and_bitboard_and_buffer(
				state.part.sente_self_board,
				state.part.sente_opponent_board,
				state.part.gote_self_board,
				state.part.gote_opponent_board,
				from,kind,
					nari_mask,deny_move_mask,
					&Rule::default_moveto_builder(&state.banmen,opponent_bitboard),
					mvs
				);
			},
			GKyou if t == Teban::Gote => {
				Rule::legal_moves_gote_kyou_with_point_and_kind_and_bitboard_and_buffer(
					state.part.sente_self_board,
					state.part.sente_opponent_board,
					from,
					nari_mask,deny_move_mask,
					&Rule::default_moveto_builder(&state.banmen,opponent_bitboard),
					mvs
				);
			},
			GKaku | GKakuN if t == Teban::Gote => {
				Rule::legal_moves_gote_kaku_with_point_and_kind_and_bitboard_and_buffer(
					state.part.gote_self_board,
					state.part.gote_opponent_board,
					state.part.sente_self_board,
					state.part.sente_opponent_board,
					from,kind,
					nari_mask,deny_move_mask,
					&Rule::default_moveto_builder(&state.banmen,opponent_bitboard),
					mvs
				);
			},
			GHisha | GHishaN if t == Teban::Gote => {
				Rule::legal_moves_gote_hisha_with_point_and_kind_and_bitboard_and_buffer(
					state.part.gote_self_board,
					state.part.gote_opponent_board,
					state.part.sente_self_board,
					state.part.sente_opponent_board,
					from,kind,
					nari_mask,deny_move_mask,
					&Rule::default_moveto_builder(&state.banmen,opponent_bitboard),
					mvs
				)
			},
			_ => (),
		}
	}

	/// 盤面上の位置を元に合法手を列挙して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `x` - 左上を0,0とする移動元のx座標
	/// * `y` - 左上を0,0とする移動元のy座標
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::legal_moves_allの内部から呼び出される）
	pub fn legal_moves_with_point(t:Teban,state:&State,x:u32,y:u32)
		-> Vec<LegalMove> {
		match &state.banmen {
			&Banmen(ref kinds) => {
				Rule::legal_moves_with_point_and_kind(t,state,x,y,kinds[y as usize][x as usize])
			}
		}
	}

	/// 盤面上の位置と駒の種別を元に合法手を列挙して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `src` - 移動元の位置
	///
	/// 渡した引数の状態が不正な場合の動作は未定義
	pub fn legal_moves_with_src(t:Teban,state:&State,src:KomaSrcPosition)
		-> Vec<LegalMove> {
		match src {
			KomaSrcPosition(x,y) => Rule::legal_moves_with_point(t, state, 9 - x, y - 1)
		}
	}

	/// 盤面上の位置と駒の種別を元に合法手を列挙して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `dst` - 移動元の位置
	///
	/// 移動元の位置からさらに移動できる位置を取得するときに使う。
	/// 渡した引数の状態が不正な場合の動作は未定義
	pub fn legal_moves_with_dst_to(t:Teban,state:&State,dst:KomaDstToPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstToPosition(x,y,_) => Rule::legal_moves_with_point(t, state, 9 - x, y - 1)
		}
	}

	/// 盤面上の位置と駒の種別を元に合法手を列挙して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `dst` - 移動元の位置
	///
	/// 移動元の位置からさらに移動できる位置を取得するときに使う。
	/// 渡した引数の状態が不正な場合の動作は未定義
	pub fn legal_moves_with_dst_put(t:Teban,state:&State,dst:KomaDstPutPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstPutPosition(x,y) => Rule::legal_moves_with_point(t, state, 9 - x, y - 1)
		}
	}

	/// 手番と盤面の状態を元に合法手を生成して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	///
	/// `State`の状態が不正な場合の動作は未定義
	/// ```
	/// use usiagent::rule::*;
	/// use usiagent::shogi::*;
	/// let state = State::new(BANMEN_START_POS.clone());
	/// let mvs = Rule::legal_moves_from_banmen(Teban::Sente,&state);
	/// assert!(mvs.len() > 0);
	/// ```
	pub fn legal_moves_from_banmen(t:Teban,state:&State)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		Rule::legal_moves_from_banmen_with_buffer(t,state,&mut mvs);

		mvs
	}

	/// 手番と盤面の状態を元に合法手を生成してバッファに追加
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `mvs` - 手を追加するバッファ
	///
	/// `State`の状態が不正な時の動作は未定義
	/// # Examples
	/// ```
	/// use usiagent::rule::*;
	/// use usiagent::shogi::*;
	/// let mut mvs = Vec::new();
	/// let state = State::new(BANMEN_START_POS.clone());
	/// Rule::legal_moves_from_banmen_with_buffer(Teban::Sente,&state,&mut mvs);
	/// assert!(mvs.len() > 0);
	/// ```
	pub fn legal_moves_from_banmen_with_buffer(t:Teban,state:&State,mvs:&mut Vec<LegalMove>) {
		if t == Teban::Sente {
			for p in &mut state.part.sente_fu_board.clone() {
				if unsafe { state.part.sente_nari_board.merged_bitboard & (2 << p) } == 0 {
					let (x,y) = p.square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SFu, mvs
					);
				} else {
					let (x,y) = p.square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SFuN, mvs
					);
				}
			}

			for p in &mut state.part.sente_kyou_board.clone() {
				if unsafe { state.part.sente_nari_board.merged_bitboard & (2 << p) } == 0 {
					let (x,y) = p.square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SKyou, mvs
					);
				} else {
					let (x,y) = p.square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SKyouN, mvs
					);
				}
			}

			for p in &mut state.part.sente_kei_board.clone() {
				if unsafe { state.part.sente_nari_board.merged_bitboard & (2 << p) } == 0 {
					let (x,y) = p.square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SKei, mvs
					);
				} else {
					let (x,y) = p.square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SKeiN, mvs
					);
				}
			}

			for p in &mut state.part.sente_gin_board.clone() {
				if unsafe { state.part.sente_nari_board.merged_bitboard & (2 << p) } == 0 {
					let (x,y) = p.square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SGin, mvs
					);
				} else {
					let (x,y) = p.square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SGinN, mvs
					);
				}
			}

			for p in &mut state.part.sente_kin_board.clone() {
				let (x,y) = p.square_to_point();

				Rule::legal_moves_with_point_and_kind_and_buffer(
					t, state, x as u32, y as u32, KomaKind::SKin, mvs
				);
			}

			for p in &mut state.part.sente_kaku_board.clone() {
				if unsafe { state.part.sente_nari_board.merged_bitboard & (2 << p) } == 0 {
					let (x,y) = p.square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SKaku, mvs
					);
				} else {
					let (x,y) = p.square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SKakuN, mvs
					);
				}
			}

			for p in &mut state.part.sente_hisha_board.clone() {
				if unsafe { state.part.sente_nari_board.merged_bitboard & (2 << p) } == 0 {
					let (x,y) = p.square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SHisha, mvs
					);
				} else {
					let (x,y) = p.square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SHishaN, mvs
					);
				}
			}

			let mut b = BitBoard { merged_bitboard: unsafe {
				state.part.gote_opponent_ou_position_board.merged_bitboard.reverse_bits() >> 45
			} };

			for p in &mut b {
				let (x,y) = p.square_to_point();

				Rule::legal_moves_with_point_and_kind_and_buffer(
					t, state, x as u32, y as u32, KomaKind::SOu, mvs
				);
			}
		} else {
			let mut b = BitBoard { merged_bitboard: unsafe {
				state.part.gote_fu_board.merged_bitboard.reverse_bits() >> 45
			} };

			for p in &mut b {
				if unsafe { state.part.gote_nari_board.merged_bitboard & (2 << (80 - p)) } == 0 {
					let (x,y) = (80 - p).square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::GFu, mvs
					);
				} else {
					let (x,y) = (80 - p).square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::GFuN, mvs
					);
				}
			}

			let mut b = BitBoard { merged_bitboard: unsafe {
				state.part.gote_kyou_board.merged_bitboard.reverse_bits() >> 45
			} };

			for p in &mut b {
				if unsafe { state.part.gote_nari_board.merged_bitboard & (2 << (80 - p)) } == 0 {
					let (x,y) = (80 - p).square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::GKyou, mvs
					);
				} else {
					let (x,y) = (80 - p).square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::GKyouN, mvs
					);
				}
			}

			let mut b = BitBoard { merged_bitboard: unsafe {
				state.part.gote_kei_board.merged_bitboard.reverse_bits() >> 45
			} };

			for p in &mut b {
				if unsafe { state.part.gote_nari_board.merged_bitboard & (2 << (80 - p)) } == 0 {
					let (x,y) = (80 - p).square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::GKei, mvs
					);
				} else {
					let (x,y) = (80 - p).square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::GKeiN, mvs
					);
				}
			}

			let mut b = BitBoard { merged_bitboard: unsafe {
				state.part.gote_gin_board.merged_bitboard.reverse_bits() >> 45
			} };

			for p in &mut b {
				if unsafe { state.part.gote_nari_board.merged_bitboard & (2 << (80 - p)) } == 0 {
					let (x,y) = (80 - p).square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::GGin, mvs
					);
				} else {
					let (x,y) = (80 - p).square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::GGinN, mvs
					);
				}
			}

			let mut b = BitBoard { merged_bitboard: unsafe {
				state.part.gote_kin_board.merged_bitboard.reverse_bits() >> 45
			} };

			for p in &mut b {
				let (x,y) = (80 - p).square_to_point();

				Rule::legal_moves_with_point_and_kind_and_buffer(
					t, state, x as u32, y as u32, KomaKind::GKin, mvs
				);
			}

			let mut b = BitBoard { merged_bitboard: unsafe {
				state.part.gote_kaku_board.merged_bitboard.reverse_bits() >> 45
			} };

			for p in &mut b {
				if unsafe { state.part.gote_nari_board.merged_bitboard & (2 << (80 - p)) } == 0 {
					let (x,y) = (80 - p).square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SKaku, mvs
					);
				} else {
					let (x,y) = (80 - p).square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::SKakuN, mvs
					);
				}
			}

			let mut b = BitBoard { merged_bitboard: unsafe {
				state.part.gote_hisha_board.merged_bitboard.reverse_bits() >> 45
			} };

			for p in &mut b {
				if unsafe { state.part.gote_nari_board.merged_bitboard & (2 << (80 - p)) } == 0 {
					let (x,y) = (80 - p).square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::GHisha, mvs
					);
				} else {
					let (x,y) = (80 - p).square_to_point();

					Rule::legal_moves_with_point_and_kind_and_buffer(
						t, state, x as u32, y as u32, KomaKind::GHishaN, mvs
					);
				}
			}

			let mut b = BitBoard { merged_bitboard: unsafe {
				state.part.sente_opponent_ou_position_board.merged_bitboard.reverse_bits() >> 45
			} };

			for p in &mut b {
				let (x,y) = (80 - p).square_to_point();

				Rule::legal_moves_with_point_and_kind_and_buffer(
					t, state, x as u32, y as u32, KomaKind::GOu, mvs
				);
			}
		}
	}

	/// 手番と盤面の状態と持ち駒を元に駒を置く合法手を生成して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `mc` - 持ち駒
	/// * `state` - 盤面の状態
	///
	/// `State`もしくは`MochigomaCollections`の状態が不正な時の動作は未定義
	/// # Examples
	/// ```
	/// use usiagent::rule::*;
	/// use usiagent::shogi::*;
	/// let state = State::new(BANMEN_START_POS.clone());
	/// let mvs = Rule::legal_moves_from_mochigoma(Teban::Sente,&MochigomaCollections::Empty,&state);
	/// assert!(mvs.len() == 0);
	/// ```
	pub fn legal_moves_from_mochigoma(t:Teban,mc:&MochigomaCollections,state:&State)
		-> Vec<LegalMove> {

		let mut mvs:Vec<LegalMove> = Vec::new();

		Rule::legal_moves_from_mochigoma_with_buffer(t,mc,state,&mut mvs);

		mvs
	}

	/// 手番と盤面の状態と持ち駒を元に駒を置く合法手を生成してバッファに追加
	///
	/// * `t` - 手を列挙したい手番
	/// * `mc` - 持ち駒
	/// * `state` - 盤面の状態
	/// * `mvs` - 手を追加するバッファ
	/// `State`もしくは`MochigomaCollections`の状態が不正な時の動作は未定義
	/// # Examples
	/// ```
	/// use usiagent::rule::*;
	/// use usiagent::shogi::*;
	/// let mut mvs = Vec::new();
	/// let state = State::new(BANMEN_START_POS.clone());
	/// Rule::legal_moves_from_mochigoma_with_buffer(Teban::Sente,&MochigomaCollections::Empty,&state,&mut mvs);
	/// assert!(mvs.len() == 0);
	/// ```
	pub fn legal_moves_from_mochigoma_with_buffer(
		t:Teban,mc:&MochigomaCollections,state:&State,mvs:&mut Vec<LegalMove>
	) {
		let mc = match mc {
			&MochigomaCollections::Pair(ref ms, ref mg) => {
				match t {
					Teban::Sente => {
						ms
					},
					Teban::Gote => {
						mg
					}
				}
			},
			&MochigomaCollections::Empty => {
				return;
			}
		};

		for (m,count) in mc.iter() {
			if count == 0 {
				continue;
			}

			let (deny_move_bitboard,candidate_bitboard,fu_bitboard) = match t {
				Teban::Sente => {
					let deny_move_bitboard = match m {
						MochigomaKind::Fu | MochigomaKind::Kyou => DENY_MOVE_SENTE_FU_AND_KYOU_MASK,
						MochigomaKind::Kei => DENY_MOVE_SENTE_KEI_MASK,
						_ => 0
					};

					let candidate_bitboard = {
						state.part.sente_self_board | state.part.sente_opponent_board
					};

					let sente_fu_bitboard = unsafe {
						state.part.sente_fu_board.merged_bitboard >> 1
					};

					(deny_move_bitboard,candidate_bitboard,sente_fu_bitboard)
				},
				Teban::Gote => {
					let deny_move_bitboard = match m {
						MochigomaKind::Fu | MochigomaKind::Kyou => DENY_MOVE_GOTE_FU_AND_KYOU_MASK,
						MochigomaKind::Kei => DENY_MOVE_GOTE_KEI_MASK,
						_ => 0
					};

					let candidate_bitboard = {
						state.part.gote_self_board | state.part.gote_opponent_board
					};

					let gote_fu_bitboard = unsafe {
						state.part.gote_fu_board.merged_bitboard >> 1
					};

					(deny_move_bitboard,candidate_bitboard,gote_fu_bitboard)
				}
			};

			let mut candidate_bitboard = BitBoard {
				merged_bitboard: unsafe { !candidate_bitboard.merged_bitboard & BANMEN_MASK }
			};

			loop {
				let p = Rule::pop_lsb(&mut candidate_bitboard);

				if p == -1 {
					break;
				}

				let (x,p_mask) = if t == Teban::Sente {
					(p * 114 / 1024,1 << p)
				} else {
					((80 - p) * 114 / 1024, 1 << (80 - p))
				};

				if deny_move_bitboard & p_mask != 0 {
					continue;
				}

				if m == MochigomaKind::Fu && fu_bitboard & 0b111111111 << x * 9 != 0 {
					continue;
				}

				if t == Teban::Sente {
					mvs.push(LegalMove::Put(LegalMovePut::new(m,p as u32)));
				} else {
					mvs.push(LegalMove::Put(LegalMovePut::new(m,80 - p as u32)));
				}
			}
		}
	}

	/// 手番と盤面の状態と持ち駒を元に合法手を生成して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `mc` - 持ち駒
	///
	/// `State`もしくは`MochigomaCollections`の状態が不正な時の動作は未定義
	/// # Examples
	/// ```
	/// use usiagent::rule::*;
	/// use usiagent::shogi::*;
	/// let state = State::new(BANMEN_START_POS.clone());
	/// let mvs = Rule::legal_moves_all(Teban::Sente,&state,&MochigomaCollections::Empty);
	/// assert!(mvs.len() > 0);
	/// ```
	pub fn legal_moves_all(t:Teban,state:&State,mc:&MochigomaCollections)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		Rule::legal_moves_from_banmen_with_buffer(t, state, &mut mvs);
		Rule::legal_moves_from_mochigoma_with_buffer(t, mc, state, &mut mvs);
		mvs
	}

	/// 王を取る手のうち一マスだけ駒を動かす手を返す
	///
	/// # Arguments
	/// * `teban` - 手を列挙したい手番
	/// * `self_occupied` - 手番側から見た手番側の駒の配置を表すビットボード。(後手の場合は上下逆さになっている)
	/// * `opponent_ou_bitboard` - 手番側から見た相手の王の配置を表すビットボード。(後手の場合は上下逆さになっている)
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 移動する駒の種類
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::win_only_movesの内部から呼び出される）
	pub fn win_only_move_once_with_point_and_kind_and_bitboard(
		teban:Teban,self_occupied:BitBoard,opponent_ou_bitboard:BitBoard,from:u32,kind:KomaKind
	) -> Option<Square> {
		let board = Rule::gen_candidate_bits(teban, self_occupied, from, kind);

		let opponent_ou_bitboard = unsafe {
			opponent_ou_bitboard.merged_bitboard
		};

		if opponent_ou_bitboard & unsafe { board.merged_bitboard } != 0 {
			let mut board = BitBoard { merged_bitboard: opponent_ou_bitboard };
			let p = Rule::pop_lsb(&mut board);

			Some(p)
		} else {
			None
		}
	}

	/// 先手の角を動かす手で王を取れる手を返す
	///
	/// # Arguments
	/// * `opponent_ou_bitboard` - 先手側から見た相手の王の配置を表すビットボード。
	/// * `sente_self_occupied` - 先手側から見た先手側の駒の配置を表すビットボード。
	/// * `sente_opponent_occupied` - 先手側から見た後手側の駒の配置を表すビットボード。
	/// * `flip_self_occupied` - 後手側から見た後手側の駒の配置を表すビットボード。
	/// * `flip_opponent_occupied` - 後手側から見た先手側の駒の配置を表すビットボード。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 駒の種類
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::win_only_movesの内部から呼び出される）
	pub fn win_only_move_sente_kaku_with_point_and_kind_and_bitboard(
		opponent_ou_bitboard:BitBoard,
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		from:u32, kind:KomaKind
	) -> Option<Square> {
		{
			let mut opponent_ou_bitboard = opponent_ou_bitboard;

			let o = Rule::pop_lsb(&mut opponent_ou_bitboard);

			if o == -1 {
				return None;
			}

			let opponent_ou_bitboard = 2 << o;

			let board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(
				self_occupied,
				opponent_occupied,
				from
			) | Rule::gen_candidate_bits_by_kaku_to_right_top(
				self_occupied,
				opponent_occupied,
				from
			);

			let b = opponent_ou_bitboard & unsafe { board.merged_bitboard };

			if b != 0 {
				let mut b = BitBoard { merged_bitboard: b };
				let p = Rule::pop_lsb(&mut b);

				return Some(p as Square)
			}

			let opponent_ou_bitboard = 2 << (80 - o);

			let board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(
				flip_opponent_occupied,
				flip_self_occupied,
				80 - from
			) | Rule::gen_candidate_bits_by_kaku_to_right_top(
				flip_opponent_occupied,
				flip_self_occupied,
				80 - from
			);

			let b = opponent_ou_bitboard & unsafe { board.merged_bitboard };

			if b != 0 {
				let mut b = BitBoard { merged_bitboard: b };
				let p = Rule::pop_lsb(&mut b);

				return Some((80 - p) as Square)
			}
		}

		if kind == SKakuN {
			let opponent_ou_bitboard = opponent_ou_bitboard;

			Rule::win_only_move_once_with_point_and_kind_and_bitboard(
				Teban::Sente,self_occupied,opponent_ou_bitboard,from,kind
			)
		} else {
			None
		}
	}

	/// 後手の角を動かす手で王を取れる手を返す
	///
	/// # Arguments
	/// * `opponent_ou_bitboard` - 後手側から見た相手の王の配置を表すビットボード。
	/// * `self_occupied` - 後手側から見た後手側の駒の配置を表すビットボード。
	/// * `opponent_occupied` - 後手側から見た先手側の駒の配置を表すビットボード。
	/// * `flip_self_occupied` - 先手側から見た先手側の駒の配置を表すビットボード。
	/// * `flip_opponent_occupied` - 先手側から見た後手側の駒の配置を表すビットボード。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 駒の種類
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::win_only_movesの内部から呼び出される）
	pub fn win_only_move_gote_kaku_with_point_and_kind_and_bitboard(
		opponent_ou_bitboard:BitBoard,
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		from:u32, kind:KomaKind
	) -> Option<Square> {
		{
			let mut opponent_ou_bitboard = opponent_ou_bitboard;

			let o = Rule::pop_lsb(&mut opponent_ou_bitboard);

			if o == -1 {
				return None;
			}

			let opponent_ou_bitboard = 2 << (80 - o);

			let board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(
				flip_opponent_occupied,
				flip_self_occupied,
				from
			) | Rule::gen_candidate_bits_by_kaku_to_right_top(
				flip_opponent_occupied,
				flip_self_occupied,
				from
			);

			let b = opponent_ou_bitboard & unsafe { board.merged_bitboard };

			if b != 0 {
				let mut b = BitBoard { merged_bitboard: b };
				let p = Rule::pop_lsb(&mut b);

				return Some(p as Square)
			}

			let opponent_ou_bitboard = 2 << o;

			let board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(
				self_occupied,
				opponent_occupied,
				80 - from
			) | Rule::gen_candidate_bits_by_kaku_to_right_top(
				self_occupied,
				opponent_occupied,
				80 - from
			);

			let b = opponent_ou_bitboard & unsafe { board.merged_bitboard };

			if b != 0 {
				let mut b = BitBoard { merged_bitboard: b };
				let p = Rule::pop_lsb(&mut b);

				return Some((80 - p) as Square)
			}
		}

		if kind == GKakuN {
			let opponent_ou_bitboard = opponent_ou_bitboard;

			Rule::win_only_move_once_with_point_and_kind_and_bitboard(
				Teban::Gote,self_occupied,opponent_ou_bitboard,from,kind
			)
		} else {
			None
		}
	}

	/// 先手の飛車を動かす手で王を取れる手を返す
	///
	/// # Arguments
	/// * `opponent_ou_bitboard` - 先手側から見た相手の王の配置を表すビットボード。
	/// * `sente_self_occupied` - 先手側から見た先手側の駒の配置を表すビットボード。
	/// * `sente_opponent_occupied` - 先手側から見た後手側の駒の配置を表すビットボード。
	/// * `flip_self_occupied` - 後手側から見た後手側の駒の配置を表すビットボード。
	/// * `flip_opponent_occupied` - 後手側から見た先手側の駒の配置を表すビットボード。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 駒の種類
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::win_only_movesの内部から呼び出される）
	pub fn win_only_move_sente_hisha_with_point_and_kind_and_bitboard(
		opponent_ou_bitboard:BitBoard,
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		from:u32, kind:KomaKind
	) -> Option<Square> {
		{
			let mut opponent_ou_bitboard = opponent_ou_bitboard;

			let o = Rule::pop_lsb(&mut opponent_ou_bitboard);

			if o == -1 {
				return None;
			}

			let opponent_ou_bitboard = 2 << (80 - o);

			let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
				flip_opponent_occupied,
				flip_self_occupied,
				80 - from
			) | Rule::gen_candidate_bits_by_hisha_to_right(
				flip_opponent_occupied,
				flip_self_occupied,
				80 - from
			);

			let b = opponent_ou_bitboard & unsafe { board.merged_bitboard };

			if b != 0 {
				let mut b = BitBoard { merged_bitboard: b };
				let p = Rule::pop_lsb(&mut b);

				return Some((80 - p) as Square)
			}

			let opponent_ou_bitboard = 2 << o;

			let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
				self_occupied,
				opponent_occupied,
				from
			) | Rule::gen_candidate_bits_by_hisha_to_right(
				self_occupied,
				opponent_occupied,
				from
			);

			let b = opponent_ou_bitboard & unsafe { board.merged_bitboard };

			if b != 0 {
				let mut b = BitBoard { merged_bitboard: b };
				let p = Rule::pop_lsb(&mut b);

				return Some(p as Square)
			}
		}

		if kind == SHishaN {
			let opponent_ou_bitboard = opponent_ou_bitboard;

			Rule::win_only_move_once_with_point_and_kind_and_bitboard(
				Teban::Sente,self_occupied,opponent_ou_bitboard,from,kind
			)
		} else {
			None
		}
	}

	/// 後手の飛車を動かす手で王を取れる手を返す
	///
	/// # Arguments
	/// * `opponent_ou_bitboard` - 後手側から見た相手の王の配置を表すビットボード。
	/// * `self_occupied` - 後手側から見た後手側の駒の配置を表すビットボード。
	/// * `opponent_occupied` - 後手側から見た先手側の駒の配置を表すビットボード。
	/// * `flip_self_occupied` - 先手側から見た先手側の駒の配置を表すビットボード。
	/// * `flip_opponent_occupied` - 先手側から見た後手側の駒の配置を表すビットボード。
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 駒の種類
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::win_only_movesの内部から呼び出される）
	pub fn win_only_move_gote_hisha_with_point_and_kind_and_bitboard(
		opponent_ou_bitboard:BitBoard,
		self_occupied:BitBoard,
		opponent_occupied:BitBoard,
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		from:u32, kind:KomaKind
	) -> Option<Square> {
		{
			let mut opponent_ou_bitboard = opponent_ou_bitboard;

			let o = Rule::pop_lsb(&mut opponent_ou_bitboard);

			if o == -1 {
				return None;
			}

			let opponent_ou_bitboard = 2 << (80 - o);

			let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
				flip_opponent_occupied,
				flip_self_occupied,
				from
			) | Rule::gen_candidate_bits_by_hisha_to_right(
				flip_opponent_occupied,
				flip_self_occupied,
				from
			);

			let b = opponent_ou_bitboard & unsafe { board.merged_bitboard };

			if b != 0 {
				let mut b = BitBoard { merged_bitboard: b };
				let p = Rule::pop_lsb(&mut b);

				return Some(p as Square)
			}

			let opponent_ou_bitboard = 2 << o;

			let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
				self_occupied,
				opponent_occupied,
				80 - from
			) | Rule::gen_candidate_bits_by_hisha_to_right(
				self_occupied,
				opponent_occupied,
				80 - from
			);

			let b = opponent_ou_bitboard & unsafe { board.merged_bitboard };

			if b != 0 {
				let mut b = BitBoard { merged_bitboard: b };
				let p = Rule::pop_lsb(&mut b);

				return Some((80 - p) as Square)
			}
		}

		if kind == GHishaN {
			let opponent_ou_bitboard = opponent_ou_bitboard;

			Rule::win_only_move_once_with_point_and_kind_and_bitboard(
				Teban::Gote,self_occupied,opponent_ou_bitboard,from,kind
			)
		} else {
			None
		}
	}

	/// 先手の香車を動かす手で王を取れる手を返す
	///
	/// # Arguments
	/// * `opponent_ou_bitboard` - 先手側から見た相手の王の配置を表すビットボード。
	/// * `flip_self_occupied` - 手番側の駒の配置を表すビットボード。逆さになっているものを渡す
	/// * `flip_opponent_occupied` - 相手番側の駒の配置を表すビットボード。逆さになっているものを渡す
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::win_only_movesの内部から呼び出される）
	pub fn win_only_move_sente_kyou_with_point_and_kind_and_bitboard(
		opponent_ou_bitboard:BitBoard,
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		from:u32
	) -> Option<Square> {
		let mut opponent_ou_bitboard = opponent_ou_bitboard;

		let o = Rule::pop_lsb(&mut opponent_ou_bitboard);

		if o == -1 {
			return None;
		}

		let opponent_ou_bitboard = 2 << (80 - o);

		let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
			flip_self_occupied,
			flip_opponent_occupied,
			80 - from
		);

		let b = opponent_ou_bitboard & unsafe { board.merged_bitboard };

		if b == 0 {
			None
		} else {
			let mut b = BitBoard { merged_bitboard:b };
			let p = Rule::pop_lsb(&mut b);

			Some(80 - p as Square)
		}
	}

	/// 後手の香車を動かす手で王を取れる手を返す
	///
	/// # Arguments
	/// * `opponent_ou_bitboard` - 後手側から見た相手の王の配置を表すビットボード。
	/// * `flip_self_occupied` - 手番側の駒の配置を表すビットボード。逆さになっているものを渡す
	/// * `flip_opponent_occupied` - 相手番側の駒の配置を表すビットボード。逆さになっているものを渡す
	/// * `from` - 盤面の左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::win_only_movesの内部から呼び出される）
	pub fn win_only_move_gote_kyou_with_point_and_kind_and_bitboard(
		opponent_ou_bitboard:BitBoard,
		flip_self_occupied:BitBoard,
		flip_opponent_occupied:BitBoard,
		from:u32
	) -> Option<Square> {
		let mut opponent_ou_bitboard = opponent_ou_bitboard;

		let o = Rule::pop_lsb(&mut opponent_ou_bitboard);

		if o == -1 {
			return None;
		}

		let opponent_ou_bitboard = 2 << (80 - o);

		let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
			flip_self_occupied,
			flip_opponent_occupied,
			from
		);

		let b = opponent_ou_bitboard & unsafe { board.merged_bitboard };

		if b == 0 {
			None
		} else {
			let mut b = BitBoard { merged_bitboard:b };
			let p = Rule::pop_lsb(&mut b);

			Some(p as Square)
		}
	}

	/// 王を取れる手のうち一マスだけ駒を動かす合法手を列挙して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `x` - 盤面左上を0,0とする移動元のx座標
	/// * `y` - 盤面左上を0,0とする移動元のy座標
	/// * `kind` - 駒の種類
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::win_only_movesの内部から呼び出される）
	pub fn win_only_moves_with_point_and_kind(
		t:Teban,state:&State,x:u32,y:u32,kind:KomaKind
	) -> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();
		Rule::win_only_moves_with_point_and_kind_and_buffer(t, state, x, y, kind, &mut mvs);
		mvs
	}

	/// 王を取れる手のうち一マスだけ駒を動かす合法手を列挙してバッファに追加する
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `x` - 盤面左上を0,0とする移動元のx座標
	/// * `y` - 盤面左上を0,0とする移動元のy座標
	/// * `kind` - 駒の種類
	/// * `mvs` - 手を追加するバッファ
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::win_only_movesの内部から呼び出される）
	pub fn win_only_moves_with_point_and_kind_and_buffer(
		t:Teban,state:&State,x:u32,y:u32,kind:KomaKind,mvs:&mut Vec<LegalMove>
	) {
		let from = x * 9 + y;

		let (nari_mask,deny_move_mask) = match kind {
			SFu | SKyou => (SENTE_NARI_MASK,DENY_MOVE_SENTE_FU_AND_KYOU_MASK),
			SKei => (SENTE_NARI_MASK,DENY_MOVE_SENTE_KEI_MASK),
			SGin | SHisha | SKaku => (SENTE_NARI_MASK,0),
			GFu | GKyou => (GOTE_NARI_MASK,DENY_MOVE_GOTE_FU_AND_KYOU_MASK),
			GKei => (GOTE_NARI_MASK,DENY_MOVE_GOTE_KEI_MASK),
			GGin | GHisha | GKaku => (GOTE_NARI_MASK,0),
			SKin | SOu | SFuN | SKyouN | SKeiN | SGinN | SHishaN | SKakuN |
			GKin | GOu | GFuN | GKyouN | GKeiN | GGinN | GHishaN | GKakuN => {
				(0,0)
			},
			Blank => {
				return;
			}
		};

		let (self_bitboard,ou_position_board) = if kind < GFu {
			(state.part.sente_self_board,state.part.sente_opponent_ou_position_board)
		} else if kind < Blank {
			(state.part.gote_self_board,state.part.gote_opponent_ou_position_board)
		} else {
			return;
		};

		match kind {
			SFu | SKei | SGin | SKin | SOu | SFuN | SKyouN | SKeiN | SGinN |
			GFu | GKei | GGin | GKin | GOu | GFuN | GKyouN | GKeiN | GGinN => {
				match t {
					Teban::Sente if kind >= GFu => {
						return;
					},
					Teban::Gote if kind < GFu => {
						return;
					},
					_ => (),
				}

				if let Some(p) = Rule::win_only_move_once_with_point_and_kind_and_bitboard(
					t,self_bitboard,ou_position_board,from,kind
				) {
					Rule::append_win_only_move(p,from,kind,nari_mask,deny_move_mask,t == Teban::Gote, mvs);
				}
			},
			SKyou if t == Teban::Sente => {
				if let Some(p) = Rule::win_only_move_sente_kyou_with_point_and_kind_and_bitboard(
					 ou_position_board,
					 state.part.gote_opponent_board,
					 state.part.gote_self_board,
					 from
				) {
					Rule::append_win_only_move(p,from,kind,nari_mask,deny_move_mask,false, mvs);
				}
			}
			SKaku | SKakuN if t == Teban::Sente => {
				if let Some(p) = Rule::win_only_move_sente_kaku_with_point_and_kind_and_bitboard(
					ou_position_board,
					state.part.sente_self_board,
					state.part.sente_opponent_board,
					state.part.gote_self_board,
					state.part.gote_opponent_board,
					from, kind
				) {
					Rule::append_win_only_move(p,from,kind,nari_mask,deny_move_mask,false, mvs);
				}
			},
			SHisha | SHishaN if t == Teban::Sente => {
				if let Some(p) = Rule::win_only_move_sente_hisha_with_point_and_kind_and_bitboard(
					ou_position_board,
					state.part.sente_self_board,
					state.part.sente_opponent_board,
					state.part.gote_self_board,
					state.part.gote_opponent_board,
					from, kind
				) {
					Rule::append_win_only_move(p,from,kind,nari_mask,deny_move_mask,false, mvs);
				}
			},
			GKyou if t == Teban::Gote => {
				if let Some(p) = Rule::win_only_move_gote_kyou_with_point_and_kind_and_bitboard(
					ou_position_board,
					state.part.sente_opponent_board,
					state.part.sente_self_board,
					from
				) {
					Rule::append_win_only_move(p,from,kind,nari_mask,deny_move_mask,false, mvs);
				}
			},
			GKaku | GKakuN if t == Teban::Gote => {
				if let Some(p) = Rule::win_only_move_gote_kaku_with_point_and_kind_and_bitboard(
	ou_position_board,
					state.part.gote_self_board,
		state.part.gote_opponent_board,
		state.part.sente_self_board,
	state.part.sente_opponent_board,
					from, kind
				) {
					Rule::append_win_only_move(p,from,kind,nari_mask,deny_move_mask,false, mvs);
				}
			},
			GHisha | GHishaN if t == Teban::Gote => {
				if let Some(p) = Rule::win_only_move_gote_hisha_with_point_and_kind_and_bitboard(
					ou_position_board,
					state.part.gote_self_board,
					state.part.gote_opponent_board,
					state.part.sente_self_board,
					state.part.sente_opponent_board,
					from, kind
				) {
					Rule::append_win_only_move(p,from,kind,nari_mask,deny_move_mask,false, mvs);
				}
			},
			_ => (),
		}
	}

	/// 盤面上の位置を元に王を取れる合法手のみを列挙して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `x` - 盤面左上を0,0とする移動元のx座標
	/// * `y` - 盤面左上を0,0とする移動元のy座標
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::win_only_movesの内部から呼び出される）
	pub fn win_only_moves_with_point(t:Teban,state:&State,x:u32,y:u32)
		-> Vec<LegalMove> {
		match &state.banmen {
			&Banmen(ref kinds) => {
				Rule::win_only_moves_with_point_and_kind(t,state,x,y,kinds[y as usize][x as usize])
			}
		}
	}

	/// 盤面上の位置を元に王を取れる合法手のみを列挙して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `src` - 移動元の位置
	///
	/// 渡した引数の状態が不正な場合の動作は未定義
	pub fn win_only_moves_with_src(t:Teban,state:&State,src:KomaSrcPosition)
		-> Vec<LegalMove> {
		match src {
			KomaSrcPosition(x,y) => Rule::win_only_moves_with_point(t,state, 9 - x, y - 1)
		}
	}

	/// 盤面上の位置を元に王を取れる合法手のみを列挙して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `dst` - 移動元の位置
	///
	/// 移動元の位置からさらに移動できる位置を取得するときに使う。
	pub fn win_only_moves_with_dst_to(t:Teban,state:&State,dst:KomaDstToPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstToPosition(x,y,_) => Rule::win_only_moves_with_point(t, state, 9 - x, y - 1)
		}
	}

	/// 盤面上の位置を元に王を取れる合法手のみを列挙して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `dst` - 移動元の位置
	///
	/// 移動元の位置からさらに移動できる位置を取得するときに使う。
	pub fn win_only_moves_with_dst_put(t:Teban,state:&State,dst:KomaDstPutPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstPutPosition(x,y) => Rule::win_only_moves_with_point(t, state, 9 - x, y - 1)
		}
	}

	/// 手番と盤面の状態を元に王を取れる合法手のみを生成して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	///
	/// `State`の状態が不正な時の動作は未定義
	/// # Examples
	/// ```
	/// use usiagent::rule::*;
	/// use usiagent::shogi::*;
	/// let state = State::new(BANMEN_START_POS.clone());
	/// let mvs = Rule::win_only_moves(Teban::Sente,&state);
	/// assert!(mvs.len() == 0);
	/// ```
	pub fn win_only_moves(t:Teban,state:&State)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		match &state.banmen {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match t {
							Teban::Sente => (x,y),
							Teban::Gote => (8 - x, 8 - y),
						};
						Rule::win_only_moves_with_point_and_kind_and_buffer(
							t, state, x as u32, y as u32, kinds[y as usize][x as usize], &mut mvs
						);
					}
				}
			}
		}
		mvs
	}

	/// 盤面上の位置を元に王を取れる手か王手の合法手のみを列挙して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `mc` - 持ち駒
	/// * `x` - 左上を0,0とする移動元のx座標
	/// * `y` - 左上を0,0とする移動元のy座標
	///
	/// 渡した引数の状態が不正な場合の動作は未定義（通常,Rule::oute_only_moves_allの内部から呼び出される）
	pub fn oute_only_moves_with_point(
		t:Teban,state:&State,mc:&MochigomaCollections,x:u32,y:u32
	) -> Vec<LegalMove> {
		let mvs = Rule::win_only_moves_with_point(t,state,x,y);

		if mvs.len() > 0 {
			return mvs;
		}

		let kind = match &state.banmen {
			&Banmen(ref kinds) => kinds[y as usize][x as usize]
		};

		Rule::legal_moves_with_point(t, state, x, y)
			.into_iter().filter(|mv| {
				let mv = mv.to_applied_move();
				match mv {
					AppliedMove::To(m) => {
						let ps = Rule::apply_move_to_partial_state_none_check(state, t, mc, mv);

						let kind = if m.is_nari() {
							kind.to_nari()
						} else {
							kind
						};

						if Rule::is_mate_with_partial_state_and_from_and_kind(t, &ps, m.dst(), kind) {
							return true;
						}

						if Rule::is_mate_with_partial_state_repeat_move_kinds(t, &ps) {
							return true;
						}

						false
					},
					_ => unreachable!(),
				}
			}).collect::<Vec<LegalMove>>()
	}

	/// 手番と盤面の状態と持ち駒を元に王を取れる手か王手の盤面上の駒を動かす合法手のみを生成して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `mc` - 持ち駒
	///
	/// `State`もしくは`MochigomaCollections`の状態が不正な時の動作は未定義（通常,Rule::oute_only_moves_allの内部から呼び出される）
	/// # Examples
	/// ```
	/// use usiagent::rule::*;
	/// use usiagent::shogi::*;
	/// let state = State::new(BANMEN_START_POS.clone());
	/// let mvs = Rule::oute_only_moves_from_banmen(Teban::Sente,&state,&MochigomaCollections::Empty);
	/// assert!(mvs.len() == 0);
	/// ```
	pub fn oute_only_moves_from_banmen(t:Teban,state:&State,mc:&MochigomaCollections)
		-> Vec<LegalMove> {
		let mvs = Rule::win_only_moves(t,state);

		if mvs.len() > 0 {
			return mvs;
		}

		Rule::legal_moves_from_banmen(t, state)
			.into_iter().filter(|mv| {
				let mv = mv.to_applied_move();
				match mv {
					AppliedMove::To(m) => {
						let from = m.src();
						let (x,y) = from.square_to_point();

						let kind = match &state.banmen {
							&Banmen(ref kinds) => kinds[y as usize][x as usize]
						};

						let kind = if m.is_nari() {
							kind.to_nari()
						} else {
							kind
						};

						let ps = Rule::apply_move_to_partial_state_none_check(state, t, mc, mv);

						if Rule::is_mate_with_partial_state_and_from_and_kind(t, &ps, m.dst(), kind) {
							return true;
						}

						if Rule::is_mate_with_partial_state_repeat_move_kinds(t, &ps) {
							return true;
						}

						false
					},
					_ => unreachable!(),
				}
			}).collect::<Vec<LegalMove>>()
	}

	/// 手番と盤面の状態と持ち駒を元に駒を置く王手の合法手のみを生成して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `mc` - 持ち駒
	/// * `state` - 盤面の状態
	///
	/// `State`もしくは`MochigomaCollections`の状態が不正な時の動作は未定義（通常,Rule::oute_only_moves_allの内部から呼び出される）
	/// # Examples
	/// ```
	/// use usiagent::rule::*;
	/// use usiagent::shogi::*;
	/// let state = State::new(BANMEN_START_POS.clone());
	/// let mvs = Rule::oute_only_moves_from_mochigoma(Teban::Sente,&MochigomaCollections::Empty,&state);
	/// assert!(mvs.len() == 0);
	/// ```
	pub fn oute_only_moves_from_mochigoma(t:Teban,mc:&MochigomaCollections,state:&State) -> Vec<LegalMove> {
		Rule::legal_moves_from_mochigoma(t, mc, state)
			.into_iter().filter(|mv| {
				let mv = mv.to_applied_move();
				match mv {
					AppliedMove::Put(m) => {
						let ps = Rule::apply_move_to_partial_state_none_check(state, t, mc, mv);

						if Rule::is_mate_with_partial_state_and_from_and_kind(
							t, &ps, m.dst(), KomaKind::from((t,m.kind()))
						) {
							return true;
						}

						if Rule::is_mate_with_partial_state_repeat_move_kinds(t, &ps) {
							return true;
						}

						false
					},
					_ => unreachable!()
				}
			}).collect::<Vec<LegalMove>>()
	}

	/// 手番と盤面の状態と持ち駒を元に王を取れる手か王手の合法手のみを生成して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `mc` - 持ち駒
	///
	/// `State`もしくは`MochigomaCollections`の状態が不正な時の動作は未定義
	/// # Examples
	/// ```
	/// use usiagent::rule::*;
	/// use usiagent::shogi::*;
	/// let state = State::new(BANMEN_START_POS.clone());
	/// let mvs = Rule::oute_only_moves_all(Teban::Sente,&state,&MochigomaCollections::Empty);
	/// assert!(mvs.len() == 0);
	/// ```
	pub fn oute_only_moves_all(t:Teban,state:&State,mc:&MochigomaCollections)
		-> Vec<LegalMove> {
		let mvs = Rule::win_only_moves(t,state);

		if mvs.len() > 0 {
			return mvs;
		}

		Rule::legal_moves_all(t, state, mc)
			.into_iter().filter(|mv| {
				let mv = mv.to_applied_move();
				let (kind,dst) = match mv {
					AppliedMove::To(m) => {
						let from = m.src();
						let (x,y) = from.square_to_point();

						let kind = match &state.banmen {
							&Banmen(ref kinds) => kinds[y as usize][x as usize]
						};


						let kind = if m.is_nari() {
							kind.to_nari()
						} else {
							kind
						};

						(kind,m.dst())
					},
					AppliedMove::Put(m) => {
						(KomaKind::from((t,m.kind())),m.dst())
					}
				};

				let ps = Rule::apply_move_to_partial_state_none_check(state, t, mc, mv);

				if Rule::is_mate_with_partial_state_and_from_and_kind(t, &ps, dst, kind) {
					return true;
				}

				if Rule::is_mate_with_partial_state_repeat_move_kinds(t, &ps) {
					return true;
				}

				false
			}).collect::<Vec<LegalMove>>()
	}

	/// 手番と盤面の状態と持ち駒を元に王手に応ずる合法手のみを生成して返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// * `mc` - 持ち駒
	///
	/// `State`もしくは`MochigomaCollections`の状態が不正な時の動作は未定義
	/// 王手がかかってない状態で呼ばれると手を適用した結果相手に王を取られない手が列挙される
	/// # Examples
	/// ```
	/// use usiagent::rule::*;
	/// use usiagent::shogi::*;
	/// let state = State::new(BANMEN_START_POS.clone());
	/// let mvs = Rule::respond_oute_only_moves_all(Teban::Sente,&state,&MochigomaCollections::Empty);
	/// assert!(mvs.len() > 0);
	/// ```
	pub fn respond_oute_only_moves_all(t:Teban,state:&State,mc:&MochigomaCollections)
		-> Vec<LegalMove> {
		Rule::legal_moves_all(t, state, mc)
			.into_iter().filter(|mv| {
				match *mv {
					LegalMove::To(m) if m.obtained() == Some(ObtainKind::Ou) => true,
					mv => {
						let mv = mv.to_applied_move();
						let ps = Rule::apply_move_to_partial_state_none_check(state, t, mc, mv);
						!Rule::is_mate_with_partial_state_and_old_banmen_and_opponent_move(
							t.opposite(),&state.banmen,&ps,mv
						)
					}
				}
			}).collect::<Vec<LegalMove>>()
	}

	/// 盤面の状態を管理するビットボードを手を適用した状態に更新して返す
	///
	/// # Arguments
	/// * `state` - 盤面の状態
	/// * `t` - 手を列挙したい手番
	/// * `m` - 適用する手
	/// `State`もしくは`AppliedMove`の状態が不正な時の動作は未定義
	/// # Examples
	/// ```
	/// use usiagent::rule::*;
	/// use usiagent::shogi::*;
	/// let state = State::new(BANMEN_START_POS.clone());
	/// let m = Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)).to_applied_move();
	/// let p = Rule::apply_move_to_partial_state_none_check(&state,Teban::Sente,&MochigomaCollections::Empty,m);
	/// let mut banmen = BANMEN_START_POS.clone();
	/// banmen.0[6][8] = KomaKind::Blank;
	/// banmen.0[5][8] = KomaKind::SFu;
	/// assert_eq!(State::new(banmen).get_part(),&p);
	/// ```
	pub fn apply_move_to_partial_state_none_check(state:&State,t:Teban,_:&MochigomaCollections,m:AppliedMove)
		-> PartialState {
		let mut ps = state.part.clone();

		match &state.banmen {
			&Banmen(ref kinds) => {
				match m {
					AppliedMove::To(m) => {
						let from = m.src();
						let (sx,sy) = from.square_to_point();
						let to = m.dst();
						let nari = m.is_nari();

						let from_mask = 1 << (from + 1);

						let to_mask = 1 << (to + 1);

						let inverse_from_mask = 1 << (80 - from + 1);

						let inverse_to_mask = 1 << (80 - to + 1);

						let kind = kinds[sy as usize][sx as usize];

						let obtained = if kind < GFu {
							ps.sente_self_board = unsafe {
								BitBoard {
									merged_bitboard: ps.sente_self_board.merged_bitboard ^ (from_mask  | to_mask)
								}
							};
							ps.gote_opponent_board = unsafe {
								BitBoard {
									merged_bitboard: ps.gote_opponent_board.merged_bitboard ^ (inverse_from_mask | inverse_to_mask)
								}
							};

							(unsafe { ps.sente_opponent_board.merged_bitboard } & to_mask) != 0
						} else if kind < Blank {
							ps.gote_self_board = unsafe {
								BitBoard {
									merged_bitboard: ps.gote_self_board.merged_bitboard ^ (inverse_from_mask | inverse_to_mask)
								}
							};
							ps.sente_opponent_board = unsafe {
								BitBoard {
									merged_bitboard: ps.sente_opponent_board.merged_bitboard ^ (from_mask  | to_mask)
								}
							};

							(unsafe { ps.gote_opponent_board.merged_bitboard } & inverse_to_mask) != 0
						} else {
							false
						};

						match kind {
							SFu | SFuN => {
								ps.sente_fu_board = unsafe {
									BitBoard {
										merged_bitboard: ps.sente_fu_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							SKyou | SKyouN => {
								ps.sente_kyou_board = unsafe {
									BitBoard {
										merged_bitboard: ps.sente_kyou_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							SKei | SKeiN => {
								ps.sente_kei_board = unsafe {
									BitBoard {
										merged_bitboard: ps.sente_kei_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							SGin | SGinN => {
								ps.sente_gin_board = unsafe {
									BitBoard {
										merged_bitboard: ps.sente_gin_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							SKin => {
								ps.sente_kin_board = unsafe {
									BitBoard {
										merged_bitboard: ps.sente_kin_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							SKaku | SKakuN => {
								ps.sente_kaku_board = unsafe {
									BitBoard {
										merged_bitboard: ps.sente_kaku_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							SHisha | SHishaN => {
								ps.sente_hisha_board = unsafe {
									BitBoard {
										merged_bitboard: ps.sente_hisha_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							SOu => {
								ps.gote_opponent_ou_position_board = unsafe {
									BitBoard {
										merged_bitboard: ps.gote_opponent_ou_position_board.merged_bitboard ^ (inverse_from_mask | inverse_to_mask)
									}
								};
							},
							GFu | GFuN => {
								ps.gote_fu_board = unsafe {
									BitBoard {
										merged_bitboard: ps.gote_fu_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							GKyou | GKyouN => {
								ps.gote_kyou_board = unsafe {
									BitBoard {
										merged_bitboard: ps.gote_kyou_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							GKei | GKeiN => {
								ps.gote_kei_board = unsafe {
									BitBoard {
										merged_bitboard: ps.gote_kei_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							GGin | GGinN => {
								ps.gote_gin_board = unsafe {
									BitBoard {
										merged_bitboard: ps.gote_gin_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							GKin => {
								ps.gote_kin_board = unsafe {
									BitBoard {
										merged_bitboard: ps.gote_kin_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							GKaku | GKakuN => {
								ps.gote_kaku_board = unsafe {
									BitBoard {
										merged_bitboard: ps.gote_kaku_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							GHisha | GHishaN => {
								ps.gote_hisha_board = unsafe {
									BitBoard {
										merged_bitboard: ps.gote_hisha_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							GOu => {
								ps.sente_opponent_ou_position_board = unsafe {
									BitBoard {
										merged_bitboard: ps.sente_opponent_ou_position_board.merged_bitboard ^ (from_mask | to_mask)
									}
								};
							},
							Blank => (),
						}

						if kind >= SFu && kind < GFu && (nari || (unsafe { from_mask & ps.sente_nari_board.merged_bitboard != 0 })) {
							ps.sente_nari_board = unsafe {
								BitBoard {
									merged_bitboard: ps.sente_nari_board.merged_bitboard | to_mask
								}
							}
						} else if kind >= GFu && kind < Blank && (unsafe { nari || (from_mask & ps.gote_nari_board.merged_bitboard) != 0 }) {
							ps.gote_nari_board = unsafe {
								BitBoard {
									merged_bitboard: ps.gote_nari_board.merged_bitboard | to_mask
								}
							}
						}

						if obtained {
							let obtained_mask = !to_mask;

							let (ox,oy) = to.square_to_point();

							let kind = kinds[oy as usize][ox as usize];

							if kind < GFu {
								ps.sente_opponent_board = unsafe {
									BitBoard {
										merged_bitboard: ps.sente_opponent_board.merged_bitboard & obtained_mask
									}
								};
								ps.gote_self_board = unsafe {
									BitBoard {
										merged_bitboard: ps.gote_self_board.merged_bitboard & !inverse_to_mask
									}
								};
								ps.sente_nari_board = unsafe {
									BitBoard {
										merged_bitboard: ps.sente_nari_board.merged_bitboard & obtained_mask
									}
								};
							} else {
								ps.sente_self_board = unsafe {
									BitBoard {
										merged_bitboard: ps.sente_self_board.merged_bitboard & obtained_mask
									}
								};
								ps.gote_opponent_board = unsafe {
									BitBoard {
										merged_bitboard: ps.gote_opponent_board.merged_bitboard & !inverse_to_mask
									}
								};
								ps.gote_nari_board = unsafe {
									BitBoard {
										merged_bitboard: ps.gote_nari_board.merged_bitboard & obtained_mask
									}
								};
							}

							match kind {
								SFu | SFuN => {
									ps.sente_fu_board = unsafe {
										BitBoard {
											merged_bitboard: ps.sente_fu_board.merged_bitboard & obtained_mask
										}
									};
								},
								SKyou | SKyouN => {
									ps.sente_kyou_board = unsafe {
										BitBoard {
											merged_bitboard: ps.sente_kyou_board.merged_bitboard & obtained_mask
										}
									};
								},
								SKei | SKeiN => {
									ps.sente_kei_board = unsafe {
										BitBoard {
											merged_bitboard: ps.sente_kei_board.merged_bitboard & obtained_mask
										}
									};
								},
								SGin | SGinN => {
									ps.sente_gin_board = unsafe {
										BitBoard {
											merged_bitboard: ps.sente_gin_board.merged_bitboard & obtained_mask
										}
									};
								},
								SKin => {
									ps.sente_kin_board = unsafe {
										BitBoard {
											merged_bitboard: ps.sente_kin_board.merged_bitboard & obtained_mask
										}
									};
								},
								SKaku | SKakuN => {
									ps.sente_kaku_board = unsafe {
										BitBoard {
											merged_bitboard: ps.sente_kaku_board.merged_bitboard & obtained_mask
										}
									};
								},
								SHisha | SHishaN => {
									ps.sente_hisha_board = unsafe {
										BitBoard {
											merged_bitboard: ps.sente_hisha_board.merged_bitboard & obtained_mask
										}
									};
								}
								SOu => {
									ps.gote_opponent_ou_position_board = unsafe {
										BitBoard {
											merged_bitboard: ps.gote_opponent_ou_position_board.merged_bitboard & (inverse_from_mask | inverse_to_mask)
										}
									};
								},
								GFu | GFuN => {
									ps.gote_fu_board = unsafe {
										BitBoard {
											merged_bitboard: ps.gote_fu_board.merged_bitboard & obtained_mask
										}
									};
								},
								GKyou | GKyouN => {
									ps.gote_kyou_board = unsafe {
										BitBoard {
											merged_bitboard: ps.gote_kyou_board.merged_bitboard & obtained_mask
										}
									};
								},
								GKei | GKeiN => {
									ps.gote_kei_board = unsafe {
										BitBoard {
											merged_bitboard: ps.gote_kei_board.merged_bitboard & obtained_mask
										}
									};
								},
								GGin | GGinN => {
									ps.gote_gin_board = unsafe {
										BitBoard {
											merged_bitboard: ps.gote_gin_board.merged_bitboard & obtained_mask
										}
									};
								},
								GKin => {
									ps.gote_kin_board = unsafe {
										BitBoard {
											merged_bitboard: ps.gote_kin_board.merged_bitboard & obtained_mask
										}
									};
								},
								GKaku | GKakuN => {
									ps.gote_kaku_board = unsafe {
										BitBoard {
											merged_bitboard: ps.gote_kaku_board.merged_bitboard & obtained_mask
										}
									};
								},
								GHisha | GHishaN => {
									ps.gote_hisha_board = unsafe {
										BitBoard {
											merged_bitboard: ps.gote_hisha_board.merged_bitboard & obtained_mask
										}
									};
								},
								GOu => {
									ps.sente_opponent_ou_position_board = unsafe {
										BitBoard {
											merged_bitboard: ps.sente_opponent_ou_position_board.merged_bitboard & obtained_mask
										}
									};
								},
								Blank => ()
							}
						}
					},
					AppliedMove::Put(m) => {
						let to = m.dst();
						let to_mask = 1 << (to + 1);
						let inverse_to_mask = 1 << (80 - to + 1);

						match t {
							Teban::Sente => {
								ps.sente_self_board = unsafe {
									BitBoard {
										merged_bitboard: ps.sente_self_board.merged_bitboard ^ to_mask
									}
								};
								ps.gote_opponent_board = unsafe {
									BitBoard {
										merged_bitboard: ps.gote_opponent_board.merged_bitboard ^ inverse_to_mask
									}
								};

								match m.kind() {
									MochigomaKind::Fu => {
										ps.sente_fu_board = unsafe {
											BitBoard {
												merged_bitboard: ps.sente_fu_board.merged_bitboard ^ to_mask
											}
										};
									},
									MochigomaKind::Kyou => {
										ps.sente_kyou_board = unsafe {
											BitBoard {
												merged_bitboard: ps.sente_kyou_board.merged_bitboard ^ to_mask
											}
										};
									},
									MochigomaKind::Kei => {
										ps.sente_kei_board = unsafe {
											BitBoard {
												merged_bitboard: ps.sente_kei_board.merged_bitboard ^ to_mask
											}
										};
									},
									MochigomaKind::Gin => {
										ps.sente_gin_board = unsafe {
											BitBoard {
												merged_bitboard: ps.sente_gin_board.merged_bitboard ^ to_mask
											}
										};
									},
									MochigomaKind::Kin => {
										ps.sente_kin_board = unsafe {
											BitBoard {
												merged_bitboard: ps.sente_kin_board.merged_bitboard ^ to_mask
											}
										};
									},
									MochigomaKind::Kaku => {
										ps.sente_kaku_board = unsafe {
											BitBoard {
												merged_bitboard: ps.sente_kaku_board.merged_bitboard ^ to_mask
											}
										};
									},
									MochigomaKind::Hisha => {
										ps.sente_hisha_board = unsafe {
											BitBoard {
												merged_bitboard: ps.sente_hisha_board.merged_bitboard ^ to_mask
											}
										};
									}
								}
							},
							Teban::Gote => {
								ps.gote_self_board = unsafe {
									BitBoard {
										merged_bitboard: ps.gote_self_board.merged_bitboard ^ inverse_to_mask
									}
								};
								ps.sente_opponent_board = unsafe {
									BitBoard {
										merged_bitboard: ps.sente_opponent_board.merged_bitboard ^ to_mask
									}
								};

								match m.kind() {
									MochigomaKind::Fu => {
										ps.gote_fu_board = unsafe {
											BitBoard {
												merged_bitboard: ps.gote_fu_board.merged_bitboard ^ to_mask
											}
										};
									},
									MochigomaKind::Kyou => {
										ps.gote_kyou_board = unsafe {
											BitBoard {
												merged_bitboard: ps.gote_kyou_board.merged_bitboard ^ to_mask
											}
										};
									},
									MochigomaKind::Kei => {
										ps.gote_kei_board = unsafe {
											BitBoard {
												merged_bitboard: ps.gote_kei_board.merged_bitboard ^ to_mask
											}
										};
									},
									MochigomaKind::Gin => {
										ps.gote_gin_board = unsafe {
											BitBoard {
												merged_bitboard: ps.gote_gin_board.merged_bitboard ^ to_mask
											}
										};
									},
									MochigomaKind::Kin => {
										ps.gote_kin_board = unsafe {
											BitBoard {
												merged_bitboard: ps.gote_kin_board.merged_bitboard ^ to_mask
											}
										};
									},
									MochigomaKind::Kaku => {
										ps.gote_kaku_board = unsafe {
											BitBoard {
												merged_bitboard: ps.gote_kaku_board.merged_bitboard ^ to_mask
											}
										};
									},
									MochigomaKind::Hisha => {
										ps.gote_hisha_board = unsafe {
											BitBoard {
												merged_bitboard: ps.gote_hisha_board.merged_bitboard ^ to_mask
											}
										};
									}
								}
							}
						}
					}
				}
			}
		}
		ps
	}

	/// 現在の局面に手を適用した結果を返す。合法手か否かのチェックは行わない。
	///
	/// # Arguments
	/// * `state` - 盤面の状態
	/// * `t` - 手を列挙したい手番
	/// * `mc` - 持ち駒
	/// * `m` - 適用する手
	/// `State`もしくは`AppliedMove`の状態が不正な時の動作は未定義
	pub fn apply_move_none_check(state:&State,t:Teban,mc:&MochigomaCollections,m:AppliedMove)
		-> (State,MochigomaCollections,Option<MochigomaKind>) {
		let ps = Rule::apply_move_to_partial_state_none_check(state,t,mc,m);
		let (banmen,mc,o) = Rule::apply_move_to_banmen_and_mochigoma_none_check(
			&state.banmen,t,mc,m
		);
		(ps.to_full_state(banmen),mc,o)
	}

	/// 現在の局面に手を適用した結果を返す。適用対象は盤面と持ち駒のみで`State`は返さない。
	///
	/// # Arguments
	/// * `state` - 盤面の状態
	/// * `t` - 手を列挙したい手番
	/// * `mc` - 持ち駒
	/// * `m` - 適用する手
	/// `State`もしくは`AppliedMove`の状態が不正な時の動作は未定義
	pub fn apply_move_to_banmen_and_mochigoma_none_check(
		banmen:&Banmen,t:Teban,mc:&MochigomaCollections,m:AppliedMove
	) -> (Banmen,MochigomaCollections,Option<MochigomaKind>) {

		let mut kinds = match banmen {
			&Banmen(ref kinds) => kinds.clone(),
		};

		let (nmc,obtained) = match m {
			AppliedMove::To(m) => {
				let from = m.src();
				let to = m.dst();
				let n = m.is_nari();

				let (sx,sy) = from.square_to_point();
				let (dx,dy) = to.square_to_point();

				let sx = sx as usize;
				let sy = sy as usize;
				let dx = dx as usize;
				let dy = dy as usize;

				let k = kinds[sy][sx];

				kinds[sy][sx] = KomaKind::Blank;

				match kinds[dy][dx] {
					KomaKind::Blank => {
						kinds[dy][dx] = match n {
							true => {
								match k {
									KomaKind::SFu => KomaKind::SFuN,
									KomaKind::SKyou => KomaKind::SKyouN,
									KomaKind::SKei => KomaKind::SKeiN,
									KomaKind::SGin => KomaKind::SGinN,
									KomaKind::SKaku => KomaKind::SKakuN,
									KomaKind::SHisha => KomaKind::SHishaN,
									KomaKind::GFu => KomaKind::GFuN,
									KomaKind::GKyou => KomaKind::GKyouN,
									KomaKind::GKei => KomaKind::GKeiN,
									KomaKind::GGin => KomaKind::GGinN,
									KomaKind::GKaku => KomaKind::GKakuN,
									KomaKind::GHisha => KomaKind::GHishaN,
									_ => k,
								}
							},
							false => k,
						};
						(mc.clone(),None)
					},
					dst => {
						let obtained = match ObtainKind::try_from(dst) {
							Ok(obtained) => {
								match MochigomaKind::try_from(obtained) {
									Ok(obtained) => Some(obtained),
									_ => None,
								}
							},
							Err(_) => None,
						};

						kinds[dy][dx] = match n {
							true => {
								match k {
									KomaKind::SFu => KomaKind::SFuN,
									KomaKind::SKyou => KomaKind::SKyouN,
									KomaKind::SKei => KomaKind::SKeiN,
									KomaKind::SGin => KomaKind::SGinN,
									KomaKind::SKaku => KomaKind::SKakuN,
									KomaKind::SHisha => KomaKind::SHishaN,
									KomaKind::GFu => KomaKind::GFuN,
									KomaKind::GKyou => KomaKind::GKyouN,
									KomaKind::GKei => KomaKind::GKeiN,
									KomaKind::GGin => KomaKind::GGinN,
									KomaKind::GKaku => KomaKind::GKakuN,
									KomaKind::GHisha => KomaKind::GHishaN,
									_ => k,
								}
							},
							false => k,
						};

						match obtained {
							Some(obtained) => {
								match mc {
									&MochigomaCollections::Pair(ref ms, ref mg) => {
										match t {
											Teban::Sente => {
												let mut ms = ms.clone();

												ms.put(obtained);

												(MochigomaCollections::Pair(ms,mg.clone()),Some(obtained))
											},
											Teban::Gote => {
												let mut mg = mg.clone();

												mg.put(obtained);

												(MochigomaCollections::Pair(ms.clone(),mg),Some(obtained))
											}
										}
									},
									&MochigomaCollections::Empty => {
										match t {
											Teban::Sente => {
												let mut ms:Mochigoma = Mochigoma::new();

												ms.insert(obtained,1);
												(MochigomaCollections::Pair(ms,Mochigoma::new()),Some(obtained))
											},
											Teban::Gote => {
												let mut mg:Mochigoma = Mochigoma::new();

												mg.insert(obtained,1);
												(MochigomaCollections::Pair(Mochigoma::new(),mg),Some(obtained))
											}
										}
									}
								}
							},
							None => {
								(mc.clone(),None)
							}
						}
					}
				}
			},
			AppliedMove::Put(m) => {
				let to = m.dst();
				let k = m.kind();

				let (dx,dy) = to.square_to_point();
				let dx = dx as usize;
				let dy = dy as usize;

				kinds[dy][dx] = KomaKind::from((t,k));

				let mut mc = mc.clone();

				match t {
					Teban::Sente => {
						match mc {
							MochigomaCollections::Pair(ref mut mc,_) => {
								let c = match mc.get(k) {
									0 => {
										0
									},
									c => c - 1,
								};
								mc.insert(k,c);
							},
							_ => (),
						}
					},
					Teban::Gote => {
						match mc {
							MochigomaCollections::Pair(_,ref mut mc) => {
								let c = match mc.get(k) {
									0 => {
										0
									},
									c => c - 1
								};
								mc.insert(k,c);
							},
							_ => (),
						}
					}
				};

				(mc,None)
			}
		};

		(Banmen(kinds),nmc,obtained)
	}

	/// 手が合法かどうかを返す
	///
	/// # Arguments
	/// * `state` - 盤面の状態
	/// * `t` - 手を列挙したい手番
	/// * `mc` - 持ち駒
	/// * `m` - 適用する手
	/// `State`が不正な時の動作は未定義
	pub fn is_valid_move(state:&State,t:Teban,mc:&MochigomaCollections,m:AppliedMove) -> bool {
		match m {
			AppliedMove::To(m) => {
				let from = m.src();
				let to = m.dst();

				if from > 80 || to > 80 {
					return false;
				}

				let (x,y) = from.square_to_point();

				let kind = match &state.banmen {
					&Banmen(ref kinds) => kinds[y as usize][x as usize]
				};

				if kind == Blank || (t == Teban::Sente && kind >= GFu) || (t == Teban::Gote && kind < GFu) {
					return false;
				}

				if m.is_nari() {
					let nari_mask = match kind {
						SFu | SKyou | SKei | SGin | SHisha | SKaku => SENTE_NARI_MASK,
						GFu | GKyou | GKei | GGin | GHisha | GKaku => GOTE_NARI_MASK,
						_  => {
							return false;
						}
					};

					if nari_mask & (1 << to) == 0 && nari_mask & (1 << from) == 0 {
						return false;
					}
				} else {
					let to_mask = 1 << to;

					let deny_move_mask = match kind {
						SFu | SKyou => DENY_MOVE_SENTE_FU_AND_KYOU_MASK,
						SKei => DENY_MOVE_SENTE_KEI_MASK,
						GFu | GKyou => DENY_MOVE_GOTE_FU_AND_KYOU_MASK,
						GKei => DENY_MOVE_GOTE_KEI_MASK,
						_ => 0,
					};

					if deny_move_mask & to_mask != 0 {
						return false;
					}
				}

				let (to,self_occupied) = if kind < GFu {
					(to,state.part.sente_self_board)
				} else if kind < Blank {
					(80 - to,state.part.gote_self_board)
				} else {
					return false;
				};

				match kind {
					SFu | SKei | SGin | SKin | SOu | SFuN | SKyouN | SKeiN | SGinN | SHishaN | SKakuN |
					GFu | GKei | GGin | GKin | GOu | GFuN | GKyouN | GKeiN | GGinN | GHishaN | GKakuN => {
						let board = Rule::gen_candidate_bits(t, self_occupied, from, kind);

						if (unsafe { board.merged_bitboard } & (1 << (to + 1))) != 0 {
							return true;
						}
					},
					SKyou | SHisha | SKaku | GKyou | GHisha | GKaku | Blank => (),
				}

				match kind {
					SKyou => {
						let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
							state.part.gote_opponent_board,
							state.part.gote_self_board, 80 - from);
						if (unsafe { board.merged_bitboard } & (2 << (80 - to))) != 0 {
							return true;
						}
					},
					GKyou => {
						let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
							state.part.sente_opponent_board,
							state.part.sente_self_board,
							from);
						if (unsafe { board.merged_bitboard } & (2 << (80 - to))) != 0 {
							return true;
						}
					},
					SKaku | SKakuN => {
						let board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(
							state.part.sente_self_board,
							state.part.sente_opponent_board,
							from
						) | Rule::gen_candidate_bits_by_kaku_to_right_top(
							state.part.sente_self_board,
							state.part.sente_opponent_board,
							from
						);

						if (unsafe { board.merged_bitboard } & (2 << to)) != 0 {
							return true;
						}

						let board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(
							state.part.gote_opponent_board,
							state.part.gote_self_board,
							80 - from
						) | Rule::gen_candidate_bits_by_kaku_to_right_top(
							state.part.gote_opponent_board,
							state.part.gote_self_board,
							80 - from
						);

						if (unsafe { board.merged_bitboard } & (2 << (80 - to))) != 0 {
							return true;
						}
					},
					GKaku | GKakuN => {
						let board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(
							state.part.gote_self_board,
							state.part.gote_opponent_board,
							80 - from
						) | Rule::gen_candidate_bits_by_kaku_to_right_top(
							state.part.gote_self_board,
							state.part.gote_opponent_board,
							80 - from
						);

						if (unsafe { board.merged_bitboard } & (2 << to)) != 0 {
							return true;
						}

						let board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(
							state.part.sente_opponent_board,
							state.part.sente_self_board,
							from
						) | Rule::gen_candidate_bits_by_kaku_to_right_top(
							state.part.sente_opponent_board,
							state.part.sente_self_board,
							from
						);

						if (unsafe { board.merged_bitboard } & (2 << (80 - to))) != 0 {
							return true;
						}
					},
					SHisha | SHishaN => {
						let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
							state.part.gote_opponent_board,
							state.part.gote_self_board,
							80 - from
						) | Rule::gen_candidate_bits_by_hisha_to_right(
							state.part.gote_opponent_board,
							state.part.gote_self_board,
							80 - from
						);

						if (unsafe { board.merged_bitboard } & (2 << (80 - to))) != 0 {
							return true;
						}

						let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
							state.part.sente_self_board,
							state.part.sente_opponent_board,
							from
						) | Rule::gen_candidate_bits_by_hisha_to_right(
							state.part.sente_self_board,
							state.part.sente_opponent_board,
							from
						);

						if (unsafe { board.merged_bitboard } & (2 << to)) != 0 {
							return true;
						}
					},
					GHisha | GHishaN => {
						let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
							state.part.sente_opponent_board,
							state.part.sente_self_board,
							from
						) | Rule::gen_candidate_bits_by_hisha_to_right(
							state.part.sente_opponent_board,
							state.part.sente_self_board,
							from
						);

						if (unsafe { board.merged_bitboard } & (2 << (80 - to))) != 0 {
							return true;
						}

						let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
							state.part.gote_self_board,
							state.part.gote_opponent_board,
							80 - from
						) | Rule::gen_candidate_bits_by_hisha_to_right(
							state.part.gote_self_board,
							state.part.gote_opponent_board,
							80 - from
						);

						if (unsafe { board.merged_bitboard } & (2 << to)) != 0 {
							return true;
						}
					},
					_ => (),
				}

				false
			},
			AppliedMove::Put(m) => {
				let to = m.dst();

				if to > 80 {
					return false;
				}

				let to_mask = 1 << (to + 1);

				let occupied = state.part.sente_self_board | state.part.sente_opponent_board;

				let occupied = unsafe { occupied.merged_bitboard };

				if (occupied & to_mask) != 0 {
					return false;
				}

				let mc = match t {
					Teban::Sente => {
						match mc {
							&MochigomaCollections::Empty => {
								return false;
							},
							&MochigomaCollections::Pair(ref ms,_) => {
								ms
							}
						}
					},
					Teban::Gote => {
						match mc {
							&MochigomaCollections::Empty => {
								return false;
							},
							&MochigomaCollections::Pair(_,ref mg) => {
								mg
							}
						}
					}
				};

				let kind = m.kind();

				match mc.get(kind) {
					0 => {
						return false;
					},
					_ => ()
				}

				match t {
					Teban::Sente => {
						match kind {
							MochigomaKind::Fu | MochigomaKind::Kyou  => {
								if (DENY_MOVE_SENTE_FU_AND_KYOU_MASK << 1) & to_mask != 0 {
									return false;
								}
							},
							MochigomaKind::Kei => {
								if (DENY_MOVE_SENTE_KEI_MASK << 1) & to_mask != 0 {
									return false;
								}
							},
							_ => (),
						}
					},
					Teban::Gote => {
						match kind {
							MochigomaKind::Fu | MochigomaKind::Kyou  => {
								if (DENY_MOVE_GOTE_FU_AND_KYOU_MASK << 1) & to_mask != 0 {
									return false;
								}
							},
							MochigomaKind::Kei => {
								if (DENY_MOVE_GOTE_KEI_MASK << 1) & to_mask != 0 {
									return false;
								}
							},
							_ => (),
						}
					}
				}

				true
			}
		}
	}

	/// 手が合法かチェック後現在の局面に手を適用して返す。
	///
	/// # Arguments
	/// * `state` - 盤面の状態
	/// * `t` - 手を列挙したい手番
	/// * `mc` - 持ち駒
	/// * `m` - 適用する手
	/// `State`が不正な時の動作は未定義
	/// # Errors
	///
	/// この関数は以下のエラーを返すケースがあります。
	/// * [`InvalidState`] 手が合法手でない
	///
	/// [`InvalidState`]: ../error/enum.ShogiError.html#variant.InvalidState
	pub fn apply_valid_move(state:&State,t:Teban,mc:&MochigomaCollections,m:AppliedMove)
		-> Result<(State,MochigomaCollections,Option<MochigomaKind>),ShogiError> {

		if !Rule::is_valid_move(state, t, mc, m) {
			Err(ShogiError::InvalidState(String::from(
				"This is not legal move."
			)))
		} else {
			Ok(Rule::apply_move_none_check(state,t,mc,m))
		}
	}

	/// 現在の局面に手のシーケンスを適用して返す
	///
	/// # Arguments
	/// * `state` - 盤面の状態
	/// * `teban` - 手を列挙したい手番
	/// * `mc` - 持ち駒
	/// * `m` - 適用する手
	/// * `mhash` - 局面を表すハッシュ(第一キー)
	/// * `shash` - 局面を表すハッシュ(第二キー)
	/// * `kyokumen_map` - 千日手を検出するためのマップ
	/// * `oute_kyokumen_map` - 連続王手の千日手を検出するためのマップ
	/// * `hasher` - 局面のハッシュを計算するためのオブジェクト
	/// 引数の状態が不正な場合の動作は未定義
	pub fn apply_moves(mut state:State,mut teban:Teban,
						mut mc:MochigomaCollections,
						m:&Vec<AppliedMove>,mut mhash:u64,mut shash:u64,
						mut kyokumen_map:KyokumenMap<u64,u32>,
						mut oute_kyokumen_map:KyokumenMap<u64,u32>,
						hasher:&KyokumenHash<u64>)
		-> (Teban,State,MochigomaCollections,u64,u64,KyokumenMap<u64,u32>,KyokumenMap<u64,u32>) {

		for m in m {
			match Rule::apply_move_none_check(&state,teban,&mc,*m) {
				(next,nmc,o) => {
					mhash = hasher.calc_main_hash(mhash,teban,&state.banmen,&mc,*m,&o);
					shash = hasher.calc_sub_hash(shash,teban,&state.banmen,&mc,*m,&o);

					match kyokumen_map.get(teban,&mhash,&shash) {
						Some(&c) => {
							kyokumen_map.insert(teban,mhash,shash,c+1);
						},
						None => {
							kyokumen_map.insert(teban,mhash,shash,1);
						}
					}

					if Rule::is_mate(teban,&next) {
						match oute_kyokumen_map.get(teban, &mhash,&shash) {
							Some(&c) => {
								oute_kyokumen_map.insert(teban,mhash,shash,c+1);
							},
							None => {
								oute_kyokumen_map.insert(teban,mhash,shash,1);
							}
						}
					} else {
						oute_kyokumen_map.clear(teban);
					}

					mc = nmc;
					teban = teban.opposite();
					state = next;
				}
			}
		}

		(teban,state,mc,mhash,shash,kyokumen_map,oute_kyokumen_map)
	}

	/// 現在の局面に手のシーケンスを適用しつつコールバックを呼び出して結果を返す
	///
	/// # Arguments
	/// * `state` - 盤面の状態
	/// * `teban` - 手を列挙したい手番
	/// * `mc` - 持ち駒
	/// * `m` - 適用する手
	/// * `r` - コールバックから返されて最終的にこの関数からの返却値の一部となる値
	/// * `f` - コールバック関数
	/// 引数の状態が不正な場合の動作は未定義
	pub fn apply_moves_with_callback<T,F>(
						mut state:State,
						mut teban:Teban,
						mut mc:MochigomaCollections,
						m:&Vec<AppliedMove>,mut r:T,mut f:F)
		-> (Teban,State,MochigomaCollections,T)
		where F: FnMut(Teban,&Banmen,
						&MochigomaCollections,&Option<AppliedMove>,
						&Option<MochigomaKind>,T) -> T {
		for m in m {
			match Rule::apply_move_none_check(&state,teban,&mc,*m) {
				(next,nmc,o) => {
					r = f(teban,&state.banmen,&mc,&Some(*m),&o,r);
					state = next;
					mc = nmc;
					teban = teban.opposite();
				}
			}
		}

		r = f(teban,&state.banmen,&mc,&None,&None,r);

		(teban,state,mc,r)
	}

	/// 入玉宣言勝ちが成立しているかどうかを返す
	///
	/// # Arguments
	/// * `state` - 盤面の状態
	/// * `teban` - 手を列挙したい手番
	/// * `mc` - 持ち駒
	/// * `limit` - 持ち時間を使い切った時点の時間
	/// `State`の状態が不正な場合の動作は未定義
	pub fn is_nyugyoku_win(state:&State,t:Teban,mc:&MochigomaCollections,limit:&Option<Instant>) -> bool {
		if Rule::is_mate(t.opposite(),state) {
			return false
		}

		if let &Some(limit) = limit {
			if limit > Instant::now() {
				return false;
			}
		}

		match t {
			Teban::Sente => {
				if unsafe { state.part.gote_opponent_ou_position_board.merged_bitboard } & NYUGYOKU_MASK << 1 == 0 {
					return false;
				}
			},
			Teban::Gote => {
				if unsafe { state.part.sente_opponent_ou_position_board.merged_bitboard } & NYUGYOKU_MASK << 1 == 0 {
					return false;
				}
			},
		}

		match t {
			Teban::Sente => {
				let mut ou_bitboard = state.part.gote_opponent_ou_position_board;

				let ou_position = Rule::pop_lsb(&mut ou_bitboard);

				if ou_position == -1 {
					return false;
				}

				let ou_position = 80 - ou_position;
				let ou_bitboard:u128 = 1 << (ou_position + 1);

				let mut count = 0;
				let mut point = 0;

				let sente_occupied_board = state.part.sente_self_board;
				let sente_hisha_board = unsafe { state.part.sente_hisha_board.merged_bitboard };
				let sente_kaku_board = unsafe { state.part.sente_kaku_board.merged_bitboard };

				let mut sente_occupied_board = BitBoard {
					merged_bitboard: unsafe {
						sente_occupied_board.merged_bitboard &
						!(sente_hisha_board | sente_kaku_board | ou_bitboard) & (SENTE_NARI_MASK << 1)
					}
				};

				let mut sente_oogoma_board = BitBoard {
					merged_bitboard: (sente_hisha_board | sente_kaku_board) & (SENTE_NARI_MASK << 1)
				};

				loop {
					let p = Rule::pop_lsb(&mut sente_occupied_board);

					if p == -1 {
						break;
					} else {
						count += 1;
						point += 1;
					}
				}

				loop {
					let p = Rule::pop_lsb(&mut sente_oogoma_board);

					if p == -1 {
						break;
					} else {
						count += 1;
						point += 5;
					}
				}

				if count < 10 {
					return false;
				} else if point >= 28 {
					return true;
				}

				point += match mc {
					&MochigomaCollections::Pair(ref mc, _) => {
						mc.iter().map(|(k,count)| {
							match k {
								MochigomaKind::Hisha | MochigomaKind::Kaku => {
									count * 5
								},
								_ => {
									count
								}
							}
						}).fold(0, |sum,s| sum + s)
					},
					&MochigomaCollections::Empty => {
						0
					}
				};

				point >= 28
			},
			Teban::Gote => {
				let ou_bitboard = unsafe { state.part.sente_opponent_ou_position_board.merged_bitboard };

				if ou_bitboard == 0 {
					return false;
				}

				let mut count = 0;
				let mut point = 0;

				let gote_occupied_board = state.part.sente_opponent_board;
				let gote_hisha_board = unsafe { state.part.gote_hisha_board.merged_bitboard };
				let gote_kaku_board = unsafe { state.part.gote_kaku_board.merged_bitboard };

				let mut gote_occupied_board = BitBoard {
					merged_bitboard: unsafe {
						gote_occupied_board.merged_bitboard &
						!(gote_hisha_board | gote_kaku_board | ou_bitboard) & (GOTE_NARI_MASK << 1)
					}
				};

				let mut gote_oogoma_board = BitBoard {
					merged_bitboard: (gote_hisha_board | gote_kaku_board) & (GOTE_NARI_MASK << 1)
				};

				loop {
					let p = Rule::pop_lsb(&mut gote_occupied_board);

					if p == -1 {
						break;
					} else {
						count += 1;
						point += 1;
					}
				}

				loop {
					let p = Rule::pop_lsb(&mut gote_oogoma_board);

					if p == -1 {
						break;
					} else {
						count += 1;
						point += 5;
					}
				}

				if count < 10 {
					return false;
				} else if point >= 27 {
					return true;
				}

				point += match mc {
					&MochigomaCollections::Pair(_, ref mc) => {
						mc.iter().map(|(k,count)| {
							match k {
								MochigomaKind::Hisha | MochigomaKind::Kaku => {
									count * 5
								},
								_ => {
									count
								}
							}
						}).fold(0, |sum,s| sum + s)
					},
					&MochigomaCollections::Empty => {
						0
					}
				};

				point >= 27
			}
		}
	}

	/// 王手に応じたか否か
	///
	/// # Arguments
	/// * `state` - 盤面の状態
	/// * `t` - 手を列挙したい手番
	/// * `mc` - 持ち駒
	/// * `m` - 適用する手
	/// `State`もしくは`AppliedMove`の状態が不正な場合の動作は未定義
	///
	/// # Errors
	///
	/// この関数は以下のエラーを返すケースがあります
	///
	/// * [`InvalidStateError`] 王手をかけられていない状態で呼び出された
	///
	/// [`InvalidStateError`]: ../error/struct.InvalidStateError.html
	pub fn responded_oute(state:&State,t:Teban,mc:&MochigomaCollections,m:AppliedMove)
		-> Result<bool,InvalidStateError> {

		let o = t.opposite();

		if !Rule::is_mate(o, state) {
			return Err(InvalidStateError(String::from(
				"The argument m is not Move of oute."
			)));
		}

		let ps = Rule::apply_move_to_partial_state_none_check(state, t, mc, m);

		Ok(!Rule::is_mate_with_partial_state_and_old_banmen_and_opponent_move(o, &state.banmen, &ps, m))
	}

	/// 手が打ち歩詰めか否かを返す
	///
	/// # Arguments
	/// * `state` - 盤面の状態
	/// * `teban` - 手を列挙したい手番
	/// * `mc` - 持ち駒
	/// * `m` - 適用する手
	/// `State`もしくは`AppliedMove`の状態が不正な場合の動作は未定義
	pub fn is_put_fu_and_mate(state:&State,teban:Teban,mc:&MochigomaCollections,m:AppliedMove) -> bool {
		match m {
			AppliedMove::Put(m) => {
				let to = m.dst();
				let (dx,dy) = to.square_to_point();

				let kind = match &state.banmen {
					&Banmen(ref kinds) => kinds[dy as usize][dx as usize]
				};

				match kind {
					SFu | GFu => (),
					_ => {
						return false;
					}
				}

				let is_oute = Rule::is_mate_with_partial_state_and_point_and_kind(teban,&state.part,dx,dy,kind);

				is_oute && Rule::legal_moves_all(teban.opposite(), state, &mc).into_iter().filter(|m| {
					match *m {
						LegalMove::To(m) if m.obtained() == Some(ObtainKind::Ou) => true,
						m @ _ => {
							let m = m.to_applied_move();
							let ps = Rule::apply_move_to_partial_state_none_check(state, teban.opposite(), mc, m);
							!Rule::is_mate_with_partial_state_and_old_banmen_and_opponent_move(teban,&state.banmen,&ps,m)
						},
					}
				}).count() == 0
			},
			_ => false,
		}
	}

	/// 手が王を取る手か否かを返す
	///
	/// # Arguments
	/// * `state` - 盤面の状態
	/// * `teban` - 手を列挙したい手番
	/// * `m` - 適用する手
	/// `State`もしくは`AppliedMove`の状態が不正な場合の動作は未定義
	pub fn is_win(state:&State,teban:Teban,m:AppliedMove) -> bool {
		match m {
			AppliedMove::To(m) => {
				match teban {
					Teban::Sente => {
						let to = m.dst();
						let bitboard = unsafe { state.part.sente_opponent_ou_position_board.merged_bitboard };

						let to_mask = 1 << (to + 1);

						bitboard & to_mask != 0
					},
					Teban::Gote => {
						let to = m.dst();
						let bitboard = unsafe { state.part.gote_opponent_ou_position_board.merged_bitboard };

						let to_mask = 1 << (80 - to + 1);

						bitboard & to_mask != 0
					}
				}
			},
			_ => false,
		}
	}

	/// 相手が詰んでいるか否かを返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `state` - 盤面の状態
	/// `State`が不正な場合の動作は未定義
	pub fn is_mate(t:Teban,state:&State)
		-> bool {

		match &state.banmen {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match t {
							Teban::Sente => (x,y),
							Teban::Gote => (8 - x, 8 - y),
						};
						if Rule::is_mate_with_partial_state_and_point_and_kind(
							t, &state.part, x as u32, y as u32, kinds[y as usize][x as usize]
						) {
							return true;
						}
					}
				}
			}
		}
		false
	}

	/// 相手が詰んでいるか否かビットボードと移動元座標と駒の種類から返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `ps` - 盤面の状態を表すビットボード
	/// * `x` - 盤面左上を0,0とした時の移動元のx座標
	/// * `y` - 盤面左上を0,0とした時の移動元のy座標
	/// * `kind` - 駒の種類
	/// 引数が不正な場合の動作は未定義
	pub fn is_mate_with_partial_state_and_point_and_kind(t:Teban,ps:&PartialState,x:u32,y:u32,kind:KomaKind) -> bool {
		let from = x * 9 + y;

		Rule::is_mate_with_partial_state_and_from_and_kind(t,ps,from,kind)
	}

	/// 相手が詰んでいるか否かビットボードと移動元座標(x*9+y)と駒の種類から返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `ps` - 盤面の状態を表すビットボード
	/// * `from` - 盤面左上を0,0とし、x * 9 + yで表される移動元の駒の位置
	/// * `kind` - 駒の種類
	/// 引数が不正な場合の動作は未定義
	pub fn is_mate_with_partial_state_and_from_and_kind(t:Teban,ps:&PartialState,from:u32,kind:KomaKind) -> bool {
		let state = ps;

		(match kind {
			SFu | SKei | SGin | SKin | SOu | SFuN | SKyouN | SKeiN | SGinN if t == Teban::Sente => {
				Rule::win_only_move_once_with_point_and_kind_and_bitboard(
					t,state.sente_self_board,state.sente_opponent_ou_position_board,from,kind
				)
			},
			SKyou if t == Teban::Sente => {
				Rule::win_only_move_sente_kyou_with_point_and_kind_and_bitboard(
					state.sente_opponent_ou_position_board,
					state.gote_opponent_board,
					state.gote_self_board,
					from
				)
			}
			SKaku | SKakuN if t == Teban::Sente => {
				Rule::win_only_move_sente_kaku_with_point_and_kind_and_bitboard(
					state.sente_opponent_ou_position_board,
					state.sente_self_board,
					state.sente_opponent_board,
					state.gote_self_board,
					state.gote_opponent_board,
					from, kind
				)
			},
			SHisha | SHishaN if t == Teban::Sente => {
				Rule::win_only_move_sente_hisha_with_point_and_kind_and_bitboard(
					state.sente_opponent_ou_position_board,
					state.sente_self_board,
					state.sente_opponent_board,
					state.gote_self_board,
					state.gote_opponent_board,
					from, kind
				)
			},
			GFu | GKei | GGin | GKin | GOu | GFuN | GKyouN | GKeiN | GGinN if t == Teban::Gote => {
				Rule::win_only_move_once_with_point_and_kind_and_bitboard(
					t,state.gote_self_board,state.gote_opponent_ou_position_board,from,kind
				)
			},
			GKyou if t == Teban::Gote => {
				Rule::win_only_move_gote_kyou_with_point_and_kind_and_bitboard(
					state.gote_opponent_ou_position_board,
					state.sente_opponent_board,
					state.sente_self_board,
					from
				)
			},
			GKaku | GKakuN if t == Teban::Gote => {
				Rule::win_only_move_gote_kaku_with_point_and_kind_and_bitboard(
					state.gote_opponent_ou_position_board,
					state.gote_self_board,
					state.gote_opponent_board,
					state.sente_self_board,
					state.sente_opponent_board,
					from, kind
				)
			},
			GHisha | GHishaN if t == Teban::Gote => {
				Rule::win_only_move_gote_hisha_with_point_and_kind_and_bitboard(
					state.gote_opponent_ou_position_board,
					state.gote_self_board,
					state.gote_opponent_board,
					state.sente_self_board,
					state.sente_opponent_board,
					from, kind
				)
			},
			_ => None,
		}).is_some()
	}

	/// 相手が詰んでいるか否かビットボードから返す(香車、飛車、角のいずれかで詰むケースのみ)
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `ps` - 盤面の状態を表すビットボード
	/// `PartialState`が不正な場合の動作は未定義
	pub fn is_mate_with_partial_state_repeat_move_kinds(t:Teban,ps:&PartialState) -> bool {
		match t {
			Teban::Sente => {
				let mut bitboard = ps.sente_hisha_board;

				loop {
					let p = Rule::pop_lsb(&mut bitboard);

					if p == -1 {
						break;
					}

					if Rule::is_mate_with_partial_state_and_from_and_kind(t, ps, p as u32, SHisha) {
						return true;
					}
				}

				let mut bitboard = ps.sente_kaku_board;

				loop {
					let p = Rule::pop_lsb(&mut bitboard);

					if p == -1 {
						break;
					}

					if Rule::is_mate_with_partial_state_and_from_and_kind(t, ps, p as u32, SKaku) {
						return true;
					}
				}
				let mut bitboard = BitBoard { merged_bitboard: unsafe { ps.sente_kyou_board.merged_bitboard & !ps.sente_nari_board.merged_bitboard } };

				loop {
					let p = Rule::pop_lsb(&mut bitboard);

					if p == -1 {
						break;
					}

					if Rule::is_mate_with_partial_state_and_from_and_kind(t, ps, p as u32, SKyou) {
						return true;
					}
				}
			},
			Teban::Gote => {
				let mut bitboard = ps.gote_hisha_board;

				loop {
					let p = Rule::pop_lsb(&mut bitboard);

					if p == -1 {
						break;
					}

					if Rule::is_mate_with_partial_state_and_from_and_kind(t, ps, p as u32, GHisha) {
						return true;
					}
				}

				let mut bitboard = ps.gote_kaku_board;

				loop {
					let p = Rule::pop_lsb(&mut bitboard);

					if p == -1 {
						break;
					}

					if Rule::is_mate_with_partial_state_and_from_and_kind(t, ps, p as u32, GKaku) {
						return true;
					}
				}
				let mut bitboard = BitBoard { merged_bitboard: unsafe {
					ps.gote_kyou_board.merged_bitboard & !ps.gote_nari_board.merged_bitboard
				} };

				loop {
					let p = Rule::pop_lsb(&mut bitboard);

					if p == -1 {
						break;
					}

					if Rule::is_mate_with_partial_state_and_from_and_kind(t, ps, p as u32, GKyou) {
						return true;
					}
				}
			}
		}

		false
	}

	/// 相手の手番側が詰んでいるか否か手の適用後のビットボードと手の適用前の盤面と手番側の打った手から返す
	///
	/// # Arguments
	/// * `t` - 手を列挙したい手番
	/// * `banmen` - 手の適用前の盤面
	/// * `ps` - 相手の手番側の手の適用後の盤面の状態を表すビットボード
	/// * `m` - 相手の手番側が打った手
	/// 引数が不正な場合の動作は未定義
	pub fn is_mate_with_partial_state_and_old_banmen_and_opponent_move(
		t:Teban,banmen:&Banmen,ps:&PartialState,m:AppliedMove
	) -> bool {
		let from = match m {
			AppliedMove::To(m) => m.src() as i32,
			_ => -1,
		};

		let (sx,sy) = if from != -1 {
			let (sx,sy) = from.square_to_point();
			(sx as i32,sy as i32)
		} else {
			(-1,-1)
		};

		let to = match m {
			AppliedMove::To(m) => m.dst(),
			AppliedMove::Put(m) => m.dst(),
		};

		let (dx,dy) = to.square_to_point();
		let dx = dx as usize;
		let dy = dy as usize;

		match banmen {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match t {
							Teban::Sente => (x,y),
							Teban::Gote => (8 - x, 8 - y),
						};

						let kind = if x as i32 == sx && y as i32 == sy {
							Blank
						} else if x == dx && y == dy {
							match m {
								AppliedMove::To(m) if m.is_nari() => {
									kinds[sy as usize][sx as usize].to_nari()
								},
								AppliedMove::To(_) => {
									kinds[sy as usize][sx as usize]
								}
								AppliedMove::Put(m) => {
									KomaKind::from((t.opposite(),m.kind()))
								}
							}
						} else {
							kinds[y as usize][x as usize]
						};

						if Rule::is_mate_with_partial_state_and_point_and_kind(
							t, &ps, x as u32, y as u32, kind
						) {
							return true;
						}
					}
				}
			}
		}
		false
	}


	/// 指した手で王手がかけられているかを返す。既に王手がかかっている状態から指された手の場合、trueを返すとは限らない。
	///
	/// # Arguments
	/// * `state` - 現在(手が指される直前)の盤面
	/// * `teban` - 手を打った側の手番
	/// * `m` - 指された手
	/// 引数が不正な場合の動作は未定義
	pub fn is_oute_move(state:&State,teban:Teban,m:LegalMove) -> bool {
		let (self_board,
			opponent_board,
			flip_self_board,
			flip_opponent_board,
			kaku_board,
			hisha_board,
			kyou_board,
			opponent_ou_position_board,start,sign) = if teban == Teban::Sente {

			(state.part.sente_self_board,
			 state.part.sente_opponent_board,
			 state.part.gote_opponent_board,
			 state.part.gote_self_board,
			 state.part.sente_kaku_board,
			 state.part.sente_hisha_board,
			 BitBoard { merged_bitboard: unsafe { state.part.sente_kyou_board.merged_bitboard & !state.part.sente_nari_board.merged_bitboard } },
			 state.part.sente_opponent_ou_position_board,0,-1)
		} else {

			(state.part.gote_self_board,
			 state.part.gote_opponent_board,
			 state.part.sente_opponent_board,
			 state.part.sente_self_board,
			 state.part.gote_kaku_board,
			 state.part.gote_hisha_board,
			 BitBoard { merged_bitboard: unsafe { state.part.gote_kyou_board.merged_bitboard & !state.part.gote_nari_board.merged_bitboard } },
			 state.part.gote_opponent_ou_position_board,80,1)
		};

		match m {
			LegalMove::To(m) => {
				let from = m.src();
				let kind = state.banmen.0[from as usize % 9][from as usize / 9];
				let board = Rule::gen_candidate_bits(teban, self_board,m.dst(),kind);

				if unsafe { opponent_ou_position_board.merged_bitboard & board.merged_bitboard } != 0 {
					return true;
				}

				let mut kaku_board = kaku_board;
				let mut hisha_board = hisha_board;
				let mut kyou_board = kyou_board;

				match kind {
					KomaKind::SKaku | KomaKind::SKakuN | KomaKind::GKaku | KomaKind::GKakuN => {
						kaku_board = unsafe { BitBoard { merged_bitboard: kaku_board.merged_bitboard & !(2 << from) } };
						kaku_board = unsafe { BitBoard { merged_bitboard: kaku_board.merged_bitboard | (2 << m.dst()) } };
					},
					KomaKind::SHisha | KomaKind::SHishaN | KomaKind::GHisha | KomaKind::GHishaN => {
						hisha_board = unsafe { BitBoard { merged_bitboard: hisha_board.merged_bitboard & !(2 << from) } };
						hisha_board = unsafe { BitBoard { merged_bitboard: hisha_board.merged_bitboard | (2 << m.dst()) } };
					},
					KomaKind::SKyou | KomaKind::GKyou => {
						kyou_board = unsafe { BitBoard { merged_bitboard: kyou_board.merged_bitboard & !(2 << from) } };
						kyou_board = unsafe { BitBoard { merged_bitboard: kyou_board.merged_bitboard | (2 << m.dst()) } };
					},
					_ => ()
				}

				let from = ((start - from as i32) * sign) as u32;

				let self_board = unsafe { BitBoard { merged_bitboard: self_board.merged_bitboard & !(2 << from) } };
				let flip_self_board = unsafe { BitBoard { merged_bitboard: flip_self_board.merged_bitboard & !(2 << (80 - from)) } };

				loop {
					let from = Rule::pop_lsb(&mut kaku_board);

					if from == -1 {
						break;
					}

					let from = ((start - from as i32) * sign) as u32;

					let occ = unsafe { BitBoard { merged_bitboard: self_board.merged_bitboard } };

					let ou_bitboard = opponent_ou_position_board;

					let board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(
						occ,
						opponent_board,
						from
					) | Rule::gen_candidate_bits_by_kaku_to_right_top(
						occ,
						opponent_board,
						from
					);

					if unsafe { board.merged_bitboard & ou_bitboard.merged_bitboard } != 0 {
						return true;
					}

					let occ = unsafe { BitBoard { merged_bitboard: flip_self_board.merged_bitboard } };

					let mut ou_bitboard = opponent_ou_position_board;

					let p = Rule::pop_lsb(&mut ou_bitboard);

					if p == -1 {
						return false;
					}

					let ou_bitboard = BitBoard { merged_bitboard: (2 << (80 - p)) };

					let board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(
						occ,
						flip_opponent_board,
						80 - from
					) | Rule::gen_candidate_bits_by_kaku_to_right_top(
						occ,
						flip_opponent_board,
						80 - from
					);

					if unsafe { board.merged_bitboard & ou_bitboard.merged_bitboard } != 0 {
						return true;
					}
				}

				loop {
					let from = Rule::pop_lsb(&mut hisha_board);

					if from == -1 {
						break;
					}

					let from = ((start - from as i32) * sign) as u32;

					let occ = unsafe { BitBoard { merged_bitboard: flip_self_board.merged_bitboard } };

					let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
						occ,
						flip_opponent_board,
						80 - from
					) | Rule::gen_candidate_bits_by_hisha_to_right(
						occ,
						flip_opponent_board,
						80 - from
					);

					let mut ou_bitboard = opponent_ou_position_board;

					let p = Rule::pop_lsb(&mut ou_bitboard);

					if p == -1 {
						return false;
					}

					let ou_bitboard = BitBoard { merged_bitboard: (2 << (80 - p)) };

					if unsafe { board.merged_bitboard & ou_bitboard.merged_bitboard } != 0 {
						return true;
					}

					let ou_bitboard = opponent_ou_position_board;

					let occ = unsafe { BitBoard { merged_bitboard: self_board.merged_bitboard } };

					let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
						occ,
						opponent_board,
						from
					) | Rule::gen_candidate_bits_by_hisha_to_right(
						occ,
						opponent_board,
						from
					);

					if unsafe { board.merged_bitboard & ou_bitboard.merged_bitboard } != 0 {
						return true;
					}
				}

				loop {
					let from = Rule::pop_lsb(&mut kyou_board);

					if from == -1 {
						break;
					}

					let from = ((start - from as i32) * sign) as u32;

					let occ = unsafe { BitBoard { merged_bitboard: flip_self_board.merged_bitboard } };

					let mut ou_bitboard = opponent_ou_position_board;

					let p = Rule::pop_lsb(&mut ou_bitboard);

					if p == -1 {
						return false;
					}

					let ou_bitboard = BitBoard { merged_bitboard: (2 << (80 - p)) };

					let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
						occ,
						flip_opponent_board,
						80 - from
					);

					if unsafe { board.merged_bitboard & ou_bitboard.merged_bitboard } != 0 {
						return true;
					}
				}

				false
			},
			LegalMove::Put(m) => {
				let kind = From::from((teban,m.kind()));
				let board = Rule::gen_candidate_bits(teban, self_board,m.dst(),kind);

				if unsafe { opponent_ou_position_board.merged_bitboard & board.merged_bitboard } != 0 {
					return true;
				}

				let kind = m.kind();
				let from = ((start - m.dst() as i32) * sign) as u32;

				match kind {
					MochigomaKind::Hisha => {
						let occ = unsafe { BitBoard { merged_bitboard: flip_self_board.merged_bitboard } };

						let mut ou_bitboard = opponent_ou_position_board;

						let p = Rule::pop_lsb(&mut ou_bitboard);

						if p == -1 {
							return false;
						}

						let ou_bitboard = BitBoard { merged_bitboard: (2 << (80 - p)) };

						let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
							occ,
							flip_opponent_board,
							80 - from
						) | Rule::gen_candidate_bits_by_hisha_to_right(
							occ,
							flip_opponent_board,
							80 - from
						);

						if unsafe { board.merged_bitboard & ou_bitboard.merged_bitboard } != 0 {
							return true;
						}

						let ou_bitboard = opponent_ou_position_board;

						let occ = unsafe { BitBoard { merged_bitboard: self_board.merged_bitboard } };

						let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
							occ,
							opponent_board,
							from
						) | Rule::gen_candidate_bits_by_hisha_to_right(
							occ,
							opponent_board,
							from
						);

						if unsafe { board.merged_bitboard & ou_bitboard.merged_bitboard } != 0 {
							true
						} else {
							false
						}
					}
					MochigomaKind::Kaku => {
						let occ = unsafe { BitBoard { merged_bitboard: self_board.merged_bitboard } };

						let ou_bitboard = opponent_ou_position_board;

						let board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(
							occ,
							opponent_board,
							from
						) | Rule::gen_candidate_bits_by_kaku_to_right_top(
							occ,
							opponent_board,
							from
						);

						if unsafe { board.merged_bitboard & ou_bitboard.merged_bitboard } != 0 {
							return true;
						}

						let occ = unsafe { BitBoard { merged_bitboard: flip_self_board.merged_bitboard } };

						let mut ou_bitboard = opponent_ou_position_board;

						let p = Rule::pop_lsb(&mut ou_bitboard);

						if p == -1 {
							return false;
						}

						let ou_bitboard = BitBoard { merged_bitboard: (2 << (80 - p)) };

						let board = Rule::gen_candidate_bits_by_kaku_to_right_bottom(
							occ,
							flip_opponent_board,
							80 - from
						) | Rule::gen_candidate_bits_by_kaku_to_right_top(
							occ,
							flip_opponent_board,
							80 - from
						);

						if unsafe { board.merged_bitboard & ou_bitboard.merged_bitboard } != 0 {
							true
						} else {
							false
						}
					},
					MochigomaKind::Kyou => {
						let occ = unsafe { BitBoard { merged_bitboard: flip_self_board.merged_bitboard } };

						let mut ou_bitboard = opponent_ou_position_board;

						let p = Rule::pop_lsb(&mut ou_bitboard);

						if p == -1 {
							return false;
						}

						let ou_bitboard = BitBoard { merged_bitboard: (2 << (80 - p)) };

						let board = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top(
							occ,
							flip_opponent_board,
							80 - from
						);

						if unsafe { board.merged_bitboard & ou_bitboard.merged_bitboard } != 0 {
							true
						} else {
							false
						}
					}
					_ => false
				}
			}
		}
	}

	/// 駒の効きの数を計算する
	///
	/// # Arguments
	/// * `teban` - 攻め手側の手番（受け側の効きを計算したいときは逆にする）
	/// * `state` - 盤面の状態
	/// * `to` - 効きを調べたい位置
	pub fn control_count(teban:Teban,state:&State,to:Square) -> usize {
		let x = to / 9;
		let y = to - x * 9;
		let mut count = 0;

		let board = if teban == Teban::Sente {
			!BitBoard { merged_bitboard: 1 << (to + 1) }
		} else {
			!BitBoard { merged_bitboard: 1 << (80 - to + 1) }
		};

		for dx in (x-1).max(0)..=(x+1).min(8) {
			for dy in (y-1).max(0)..=(y+1).min(8) {
				let b = Rule::gen_candidate_bits(teban,
													 board,
													 dx as u32 * 9 + dy as u32,
													 state.banmen.0[dy as usize][dx as usize]);
				if unsafe { b.merged_bitboard != 0 } {
					count += 1;
				}
			}
		}

		if teban == Teban::Sente {
			if x > 0 && y < 7 && state.banmen.0[y as usize + 2][x as usize - 1] == KomaKind::SKei {
				count += 1;
			}

			if x < 8 && y < 7 && state.banmen.0[y as usize + 2][x as usize + 1] == KomaKind::SKei {
				count += 1;
			}

			let b = Rule::gen_candidate_bits_by_kaku_to_right_bottom_with_exclude(
				state.part.sente_self_board,
				state.part.sente_opponent_board,
				BitBoard { merged_bitboard: 0 },
				to as u32
			) | Rule::gen_candidate_bits_by_kaku_to_right_top_with_exclude(
				state.part.sente_self_board,
				state.part.sente_opponent_board,
				BitBoard { merged_bitboard: 0 },
				to as u32
			);

			let b = unsafe { b.merged_bitboard & state.part.sente_kaku_board.merged_bitboard };
			let mut b = BitBoard { merged_bitboard: b };

			loop {
				let p = Rule::pop_lsb(&mut b);

				if p == -1 {
					break;
				}

				count += 1;
			}

			let b = Rule::gen_candidate_bits_by_kaku_to_right_bottom_with_exclude(
				state.part.gote_opponent_board,
				state.part.gote_self_board,
				BitBoard { merged_bitboard: 0 },
				80 - to as u32
			) | Rule::gen_candidate_bits_by_kaku_to_right_top_with_exclude(
				state.part.gote_opponent_board,
				state.part.gote_self_board,
				BitBoard { merged_bitboard: 0 },
				80 - to as u32
			);

			let mut kaku_board = state.part.sente_kaku_board;

			loop {
				let p = Rule::pop_lsb(&mut kaku_board);

				if p == -1 {
					break;
				}

				let p = 80 - p + 1;

				if unsafe { (b.merged_bitboard & (1 << p)) != 0 } {
					count += 1;
				}
			}

			let b = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top_with_exclude(
				state.part.gote_opponent_board,
				state.part.gote_self_board,
				BitBoard { merged_bitboard: 0 },
				80 - to as u32
			) | Rule::gen_candidate_bits_by_hisha_to_right_with_exclude(
				state.part.gote_opponent_board,
				state.part.gote_self_board,
				BitBoard { merged_bitboard: 0 },
				80 - to as u32
			);

			let mut hisha_board = state.part.sente_hisha_board;

			loop {
				let p = Rule::pop_lsb(&mut hisha_board);

				if p == -1 {
					break;
				}

				let p = 80 - p + 1;

				if unsafe { (b.merged_bitboard & (1 << p)) != 0 } {
					count += 1;
				}
			}

			let b = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top_with_exclude(
				state.part.sente_self_board,
				state.part.sente_opponent_board,
				BitBoard { merged_bitboard: 0 },
				to as u32
			) | Rule::gen_candidate_bits_by_hisha_to_right_with_exclude(
				state.part.sente_self_board,
				state.part.sente_opponent_board,
				BitBoard { merged_bitboard: 0 },
				to as u32
			);

			{
				let b = unsafe { b.merged_bitboard & state.part.sente_hisha_board.merged_bitboard };
				let mut b = BitBoard { merged_bitboard: b };

				loop {
					let p = Rule::pop_lsb(&mut b);

					if p == -1 {
						break;
					}

					count += 1;
				}
			}

			{
				let b = unsafe {
					b.merged_bitboard & (state.part.sente_kyou_board.merged_bitboard & !state.part.sente_nari_board.merged_bitboard)
				};

				if b != 0 {
					count += 1;
				}
			}
		} else {
			if x < 8 && y > 1 && state.banmen.0[y as usize - 2][x as usize + 1] == KomaKind::GKei {
				count += 1;
			}

			if x > 0 && y > 1 && state.banmen.0[y as usize - 2][x as usize - 1] == KomaKind::GKei {
				count += 1;
			}

			let b = Rule::gen_candidate_bits_by_kaku_to_right_bottom_with_exclude(
				state.part.gote_opponent_board,
				state.part.gote_self_board,
				BitBoard { merged_bitboard: 0 },
				80 - to as u32
			) | Rule::gen_candidate_bits_by_kaku_to_right_top_with_exclude(
				state.part.gote_opponent_board,
				state.part.gote_self_board,
				BitBoard { merged_bitboard: 0 },
				80 - to as u32
			);

			let mut kaku_board = BitBoard { merged_bitboard: unsafe {
				state.part.gote_kaku_board.merged_bitboard & !state.part.gote_nari_board.merged_bitboard
			} };

			loop {
				let p = Rule::pop_lsb(&mut kaku_board);

				if p == -1 {
					break;
				}

				let p = 80 - p + 1;

				if unsafe { (b.merged_bitboard & (1 << p)) != 0 } {
					count += 1;
				}
			}

			let b = Rule::gen_candidate_bits_by_kaku_to_right_bottom_with_exclude(
				state.part.sente_self_board,
				state.part.sente_opponent_board,
				BitBoard { merged_bitboard: 0 },
				to as u32
			) | Rule::gen_candidate_bits_by_kaku_to_right_top_with_exclude(
				state.part.sente_self_board,
				state.part.sente_opponent_board,
				BitBoard { merged_bitboard: 0 },
				to as u32
			);

			let b = unsafe { b.merged_bitboard & state.part.gote_kaku_board.merged_bitboard };
			let mut b = BitBoard { merged_bitboard: b };

			loop {
				let p = Rule::pop_lsb(&mut b);

				if p == -1 {
					break;
				}

				count += 1;
			}

			let b = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top_with_exclude(
				state.part.sente_opponent_board,
				state.part.sente_self_board,
				BitBoard { merged_bitboard: 0 },
				to as u32
			) | Rule::gen_candidate_bits_by_hisha_to_right_with_exclude(
				state.part.sente_opponent_board,
				state.part.sente_self_board,
				BitBoard { merged_bitboard: 0 },
				to as u32
			);

			{
				let b = unsafe { b.merged_bitboard & state.part.gote_hisha_board.merged_bitboard };
				let mut b = BitBoard { merged_bitboard: b };

				loop {
					let p = Rule::pop_lsb(&mut b);

					if p == -1 {
						break;
					}

					count += 1;
				}
			}

			let b = Rule::gen_candidate_bits_by_hisha_or_kyou_to_top_with_exclude(
				state.part.gote_opponent_board,
				state.part.gote_self_board,
				BitBoard { merged_bitboard: 0 },
				80 - to as u32
			) | Rule::gen_candidate_bits_by_hisha_to_right_with_exclude(
				state.part.gote_opponent_board,
				state.part.gote_self_board,
				BitBoard { merged_bitboard: 0 },
				80 - to as u32
			);

			let mut hisha_board = state.part.gote_hisha_board;

			loop {
				let p = Rule::pop_lsb(&mut hisha_board);

				if p == -1 {
					break;
				}

				let p = 80 - p + 1;

				if unsafe { (b.merged_bitboard & (1 << p)) != 0 } {
					count += 1;
				}
			}

			let mut kyou_board = BitBoard { merged_bitboard: unsafe {
				state.part.gote_kyou_board.merged_bitboard & !state.part.gote_nari_board.merged_bitboard
			} };

			loop {
				let p = Rule::pop_lsb(&mut kyou_board);

				if p == -1 {
					break;
				}

				let p = 80 - p + 1;

				if unsafe { (b.merged_bitboard & (1 << p)) != 0 } {
					count += 1;
				}
			}
		}

		count
	}

	/// 駒が成れる手か判定する
	///
	/// # Arguments
	/// * `kind` - 移動する駒の種類
	/// * `from` - 移動元
	/// * `to` - 移動先
	pub fn is_possible_nari(kind:KomaKind,from:Square,to:Square) -> bool {
		let nari_mask = match kind {
			SFu | SKyou | SKei | SGin | SHisha | SKaku => SENTE_NARI_MASK,
			GFu | GKyou | GKei | GGin | GHisha | GKaku => GOTE_NARI_MASK,
			SKin | SOu | SFuN | SKyouN | SKeiN | SGinN | SHishaN | SKakuN => {
				0
			},
			GKin | GOu | GFuN | GKyouN | GKeiN | GGinN | GHishaN | GKakuN => {
				0
			},
			Blank => {
				0
			}
		};

		nari_mask & (1 << to) != 0 || nari_mask & (1 << from) != 0
	}


	/// 現在の王の位置をSquareで返す
	///
	/// # Arguments
	/// * `teban` - 手番
	/// * `state` - 盤面の状態
	pub fn ou_square(teban:Teban,state:&State) -> Square {
		match teban {
			Teban::Sente => {
				let bitboard = state.part.gote_opponent_ou_position_board;

				let (bl,br) = unsafe {
					match bitboard {
						BitBoard { bitboard } => {
							(*bitboard.get_unchecked(0),*bitboard.get_unchecked(1))
						}
					}
				};

				if bl != 0 {
					80 - (bl.trailing_zeros() as Square - 1)
				} else if br != 0 {
					80 - (br.trailing_zeros() as Square + 63)
				} else {
					-1
				}
			},
			Teban::Gote => {
				let bitboard = state.part.sente_opponent_ou_position_board;

				let (bl,br) = unsafe {
					match bitboard {
						BitBoard { bitboard } => {
							(*bitboard.get_unchecked(0),*bitboard.get_unchecked(1))
						}
					}
				};

				if bl != 0 {
					bl.trailing_zeros() as Square - 1
				} else if br != 0 {
					br.trailing_zeros() as Square + 63
				} else {
					-1
				}
			}
		}
	}

	/// 千日手検出用マップの更新関数
	///
	/// # Arguments
	/// * `teban` - 手を列挙したい手番
	/// * `mhash` - 局面を表すハッシュ（第一キー)
	/// * `shash` - 局面を表すハッシュ（第二キー)
	/// * `kyokumen_map` - 千日手検出用のマップ
	pub fn update_sennichite_map(_:&State,teban:Teban,mhash:u64,shash:u64,
									kyokumen_map:&mut KyokumenMap<u64,u32>) {
		match kyokumen_map.get(teban,&mhash,&shash) {
			Some(&c) => {
				kyokumen_map.insert(teban,mhash,shash,c+1);
			},
			None => {
				kyokumen_map.insert(teban,mhash,shash,1);
			}
		}
	}

	/// 現在の局面が千日手か否かを返す
	///
	/// # Arguments
	/// * `teban` - 手を列挙したい手番
	/// * `mhash` - 局面を表すハッシュ（第一キー)
	/// * `shash` - 局面を表すハッシュ（第二キー)
	/// * `kyokumen_map` - 千日手検出用のマップ
	pub fn is_sennichite(_:&State,teban:Teban,mhash:u64,shash:u64,
									kyokumen_map:&KyokumenMap<u64,u32>) -> bool {
		match kyokumen_map.get(teban,&mhash,&shash) {
			Some(&c) if c >= 3 => {
				true
			},
			_ => false
		}
	}

	/// 連続王手の千日手検出用マップの更新関数
	///
	/// # Arguments
	/// * `teban` - 手を列挙したい手番
	/// * `mhash` - 局面を表すハッシュ（第一キー)
	/// * `shash` - 局面を表すハッシュ（第二キー)
	/// * `oute_kyokumen_map` - 千日手検出用のマップ
	pub fn update_sennichite_by_oute_map(state:&State,teban:Teban,mhash:u64,shash:u64,
									oute_kyokumen_map:&mut KyokumenMap<u64,u32>) {

		if Rule::is_mate(teban,state) {
			let count = oute_kyokumen_map.get(teban, &mhash, &shash).map(|&c| c).unwrap_or(0);

			oute_kyokumen_map.insert(teban, mhash, shash, count + 1);
		} else {
			oute_kyokumen_map.clear(teban);
		}
	}

	/// 現在の局面が連続王手の千日手か否かを返す
	///
	/// # Arguments
	/// * `teban` - 手を列挙したい手番
	/// * `mhash` - 局面を表すハッシュ（第一キー)
	/// * `shash` - 局面を表すハッシュ（第二キー)
	/// * `oute_kyokumen_map` - 千日手検出用のマップ
	pub fn is_sennichite_by_oute(state:&State,teban:Teban,mhash:u64,shash:u64,
									oute_kyokumen_map:&KyokumenMap<u64,u32>)
		-> bool {

		if Rule::is_mate(teban,state) {
			let count = oute_kyokumen_map.get(teban, &mhash, &shash).map(|&c| c).unwrap_or(0);

			count > 0
		} else {
			false
		}
	}

	/// 現在の持ち時間を更新して返す(フィッシャークロックルール対応)
	/// # Arguments
	/// * `limit` - 持ち時間
	/// * `teban` - 現在の手番
	/// * `consumed` - 手番中の現在までの経過時間
	pub fn update_time_limit(limit:&UsiGoTimeLimit,teban:Teban,consumed:Duration) -> UsiGoTimeLimit {
		match teban {
			Teban::Sente => {
				if let &UsiGoTimeLimit::Limit(Some((ls,lg)),byoyomi_of_inc) = limit {
					let diff = consumed.as_secs() as u32 * 1000 + consumed.subsec_nanos() / 1000000;
					let inc = match byoyomi_of_inc {
						Some(UsiGoByoyomiOrInc::Inc(inc,_)) if ls > diff => {
							inc
						},
						Some(UsiGoByoyomiOrInc::Inc(inc,_)) => {
							inc - (diff - ls)
						},
						_ => {
							0
						}
					};
					let ls = if ls >= diff {
						ls - diff + inc
					} else {
						0
					};

					UsiGoTimeLimit::Limit(Some((ls as u32,lg)),byoyomi_of_inc)
				} else {
					limit.clone()
				}
			},
			Teban::Gote => {
				if let &UsiGoTimeLimit::Limit(Some((ls,lg)),byoyomi_of_inc) = limit {
					let diff = consumed.as_secs() as u32 * 1000 + consumed.subsec_nanos() / 1000000;
					let inc = match byoyomi_of_inc {
						Some(UsiGoByoyomiOrInc::Inc(_,inc)) if lg > diff => {
							inc
						},
						Some(UsiGoByoyomiOrInc::Inc(_,inc)) => {
							inc - (diff - lg)
						},
						_ => {
							0
						}
					};
					let lg = if lg >= diff {
						lg - diff + inc
					} else {
						0
					};

					UsiGoTimeLimit::Limit(Some((ls, lg as u32)),byoyomi_of_inc)
				} else {
					limit.clone()
				}
			}
		}
	}

	/// 持ち駒の状態を平手初期局面の時の駒を全部持ち駒にした状態で返す。
	///
	/// 返されるのは`HashMap<MochigomaKind,u32>`なので先手後手それぞれについて呼び出す必要がある
	pub fn filled_mochigoma_hashmap() -> HashMap<MochigomaKind,u32> {
		let mut m:HashMap<MochigomaKind,u32> = HashMap::new();

		m.insert(MochigomaKind::Fu, 9);
		m.insert(MochigomaKind::Kyou, 2);
		m.insert(MochigomaKind::Kei, 2);
		m.insert(MochigomaKind::Gin, 2);
		m.insert(MochigomaKind::Kin, 2);
		m.insert(MochigomaKind::Kaku, 1);
		m.insert(MochigomaKind::Hisha, 1);

		m
	}
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn const_test_sente_nari_mask() {
		for i in 0..128 {
			let x = i / 9;
			let y = i - x * 9;

			if x < 9 && y <= 2 {
				assert!(SENTE_NARI_MASK & 1 << i != 0);
			} else {
				assert!(SENTE_NARI_MASK & 1 << i == 0);
			}
		}
	}

	#[test]
	fn const_test_gote_nari_mask() {
		for i in 0..128 {
			let x = i / 9;
			let y = i - x * 9;

			if x < 9 && y >= 6 {
				assert!(GOTE_NARI_MASK & 1 << i != 0);
			} else {
				assert!(GOTE_NARI_MASK & 1 << i == 0);
			}
		}
	}

	#[test]
	fn const_test_deny_move_sente_fu_and_kyou_mask() {
		for i in 0..128 {
			let x = i / 9;
			let y = i - x * 9;

			if x < 9 && y == 0 {
				assert!(DENY_MOVE_SENTE_FU_AND_KYOU_MASK & 1 << i != 0);
			} else {
				assert!(DENY_MOVE_SENTE_FU_AND_KYOU_MASK & 1 << i == 0);
			}
		}
	}

	#[test]
	fn const_test_deny_move_gote_fu_and_kyou_mask() {
		for i in 0..128 {
			let x = i / 9;
			let y = i - x * 9;

			if x < 9 && y == 8 {
				assert!(DENY_MOVE_GOTE_FU_AND_KYOU_MASK & 1 << i != 0);
			} else {
				assert!(DENY_MOVE_GOTE_FU_AND_KYOU_MASK & 1 << i == 0);
			}
		}
	}

	#[test]
	fn const_test_deny_move_deny_move_sente_kei_mask() {
		for i in 0..128 {
			let x = i / 9;
			let y = i - x * 9;

			if x < 9 && y <= 1 {
				assert!(DENY_MOVE_SENTE_KEI_MASK & 1 << i != 0);
			} else {
				assert!(DENY_MOVE_SENTE_KEI_MASK & 1 << i == 0);
			}
		}
	}

	#[test]
	fn const_test_deny_move_deny_move_gote_kei_mask() {
		for i in 0..128 {
			let x = i / 9;
			let y = i - x * 9;

			if x < 9 && y >= 7 {
				assert!(DENY_MOVE_GOTE_KEI_MASK & 1 << i != 0);
			} else {
				assert!(DENY_MOVE_GOTE_KEI_MASK & 1 << i == 0);
			}
		}
	}

	#[test]
	fn const_test_top_mask() {
		for i in 0..128 {
			let x = i / 9;
			let y = i - x * 9;

			if x < 3 && y >= 2 {
				assert!(TOP_MASK & 1 << i != 0);
			} else {
				assert!(TOP_MASK & 1 << i == 0);
			}
		}
	}

	#[test]
	fn const_test_bottom_mask() {
		for i in 0..128 {
			let x = i / 9;
			let y = i - x * 9;

			if x < 3 && y <= 2 {
				assert!(BOTTOM_MASK & 1 << i != 0);
			} else {
				assert!(BOTTOM_MASK & 1 << i == 0);
			}
		}
	}

	#[test]
	fn const_test_right_mask() {
		for i in 0..128 {
			let x = i / 9;

			if x <= 1 {
				assert!(RIGHT_MASK & 1 << i != 0);
			} else {
				assert!(RIGHT_MASK & 1 << i == 0);
			}
		}
	}
}