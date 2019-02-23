use std::collections::HashMap;
use std::time::{Instant,Duration};
use std::ops::BitOr;
use std::ops::Not;

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
use TryFrom;
use Find;

impl From<u32> for ObtainKind {
	fn from(k:u32) -> ObtainKind {
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
impl From<u32> for MochigomaKind {
	fn from(k:u32) -> MochigomaKind {
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
type Square = i32;
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum LegalMove {
	To(LegalMoveTo),
	Put(LegalMovePut),
}
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct LegalMoveTo(u32);
impl LegalMoveTo {
	pub fn new(src:u32,to:u32,nari:bool,obtaind:Option<ObtainKind>) -> LegalMoveTo {
		let n:u32 = if nari {
			1
		} else {
			0
		};

		LegalMoveTo(
			obtaind.map_or(0, |o| o as u32 + 1) << 15 |
			n << 14 |
			(to << 7) & 0b1111111 |
			src & 0b1111111
		)
	}

	#[inline]
	pub fn src(&self) -> u32 {
		self.0 & 0b1111111
	}

	#[inline]
	pub fn dst(&self) -> u32 {
		(self.0 >> 7) & 0b1111111
	}

	#[inline]
	pub fn is_nari(&self) -> bool {
		(self.0 & 1 << 14) != 0
	}

	#[inline]
	pub fn obtained(&self) -> Option<ObtainKind> {
		let o:u32 = self.0 >> 15;

		if o == 0 {
			None
		} else {
			Some(ObtainKind::from(o-1))
		}
	}
}
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct LegalMovePut(u32);
impl LegalMovePut {
	pub fn new(kind:MochigomaKind,to:u32) -> LegalMovePut {
		LegalMovePut(
			(to << 3) & 0b1111111 |
			(kind as u32) & 0b111
		)
	}

	#[inline]
	pub fn dst(&self) -> u32 {
		(self.0 >> 3) & 0b1111111
	}

	#[inline]
	pub fn kind(&self) -> MochigomaKind {
		MochigomaKind::from(self.0 & 0b111)
	}
}
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum AppliedMove {
	To(AppliedMoveTo),
	Put(AppliedMovePut)
}
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct AppliedMoveTo(u32);
impl AppliedMoveTo {
	#[inline]
	pub fn src(&self) -> u32 {
		self.0 & 0b1111111
	}

	#[inline]
	pub fn dst(&self) -> u32 {
		(self.0 >> 7) & 0b1111111
	}

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
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct AppliedMovePut(u32);
impl AppliedMovePut {
	#[inline]
	pub fn to(&self) -> u32 {
		(self.0 >> 3) & 0b1111111
	}

	#[inline]
	pub fn kind(&self) -> MochigomaKind {
		MochigomaKind::from(self.0 & 0b111)
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
					(dst << 7) & 0b1111111 |
					src & 0b1111111
				))
			},
			Move::Put(kind,KomaDstPutPosition(x,y)) => {
				let x = 9 - x;
				let y = y - 1;

				let dst = x * 9 + y;

				AppliedMove::Put(AppliedMovePut(
					(dst << 3) & 0b1111111 |
					(kind as u32) & 0111
				))
			}
		}
	}
}
impl LegalMove {
	pub fn to_applied_move(&self) -> AppliedMove {
		AppliedMove::from(*self)
	}
}
impl From<LegalMove> for Move {
	fn from(m:LegalMove) -> Move {
		match m {
			LegalMove::To(m) => {
				let src = m.src();
				let dst = m.dst();
				let n = m.is_nari();
				let sx = src / 9;
				let sy = src - sx * 9;
				let dx = dst / 9;
				let dy = dst - dx * 9;
				let sx = 9 - sx;
				let sy = sy + 1;
				let dx = 9 - dx;
				let dy = dy + 1;

				Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,n))
			},
			LegalMove::Put(m) => {
				let dst = m.dst();
				let kind = m.kind();
				let dx = dst / 9;
				let dy = dst - dx * 9;
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
#[derive(Clone, Copy)]
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
pub struct State {
	banmen:Banmen,
	sente_self_board:BitBoard,
	sente_opponent_board:BitBoard,
	gote_self_board:BitBoard,
	gote_opponent_board:BitBoard,
	diag_board:BitBoard,
	rotate_board:BitBoard,
	sente_hisha_board:BitBoard,
	gote_hisha_board:BitBoard,
	sente_kaku_board:BitBoard,
	gote_kaku_board:BitBoard,
	sente_kyou_board:BitBoard,
	gote_kyou_board:BitBoard,
	sente_fu_board:BitBoard,
	gote_fu_board:BitBoard,
	sente_ou_position_board:BitBoard,
	gote_ou_position_board:BitBoard
}
impl State {
	pub fn new(banmen:Banmen) -> State {
		let mut sente_self_board:u128 = 0;
		let mut sente_opponent_board:u128 = 0;
		let mut gote_self_board:u128 = 0;
		let mut gote_opponent_board:u128 = 0;
		let mut diag_board:u128 = 0;
		let mut rotate_board:u128 = 0;
		let mut sente_hisha_board:u128 = 0;
		let mut gote_hisha_board:u128 = 0;
		let mut sente_kaku_board:u128 = 0;
		let mut gote_kaku_board:u128 = 0;
		let mut sente_kyou_board:u128 = 0;
		let mut gote_kyou_board:u128 = 0;
		let mut sente_fu_board:u128 = 0;
		let mut gote_fu_board:u128 = 0;
		let mut sente_ou_position_board:u128 = 0;
		let mut gote_ou_position_board:u128 = 0;

		match banmen {
			Banmen(ref kinds) => {
				for y in 0..9 {
					for x in 0..9 {
						let kind = kinds[y][x];
						match kind {
							SFu => sente_fu_board ^= 1 << (x * 9 + y + 1),
							SKyou => sente_kyou_board ^= 1 << (x * 9 + y + 1),
							SKaku => sente_kaku_board ^= 1 << (x * 9 + y + 1),
							SHisha => sente_hisha_board ^= 1 << (x * 9 + y + 1),
							SOu => sente_ou_position_board ^= 1 << (x * 9 + y + 1),
							GFu => gote_fu_board ^= 1 << (x * 9 + y + 1),
							GKyou => gote_kyou_board ^= 1 << (x * 9 + y + 1),
							GKaku => gote_kaku_board ^= 1 << (x * 9 + y + 1),
							GHisha => gote_hisha_board ^= 1 << (x * 9 + y + 1),
							GOu => gote_ou_position_board ^= 1 << (x * 9 + y + 1),
							_ => (),
						}

						if kind < GFu {
							sente_self_board ^= 1 << (x * 9 + y + 1);
							gote_opponent_board ^= 1 << ((8 - x) * 9 + (8 - y) + 1);
						} else if kind >= GFu && kind < Blank {
							gote_self_board ^= 1 << ((8 - x) * 9 + (8- y) + 1);
							sente_opponent_board ^= 1 << (x * 9 + y + 1);
						}

						let i = y * 9 + x;

						let li = DIAG_LEFT_ROTATE_MAP[i];

						let lmask = if li != -1 {
							1 << li
						} else {
							0
						};

						let ri = DIAG_RIGHT_ROTATE_MAP[i];

						let rmask = if ri != -1 {
							1 << ri + 64
						} else {
							0
						};

						diag_board ^= lmask | rmask;

						let (x,y) = {
							(8 - y,x)
						};

						rotate_board ^= 1 << (x * 9 + y + 1);
					}
				}
			}
		}

		State {
			banmen:banmen,
			sente_self_board:BitBoard{ merged_bitboard: sente_self_board },
			sente_opponent_board:BitBoard{ merged_bitboard: sente_opponent_board },
			gote_self_board:BitBoard{ merged_bitboard: gote_self_board },
			gote_opponent_board:BitBoard{ merged_bitboard: gote_opponent_board },
			diag_board:BitBoard{ merged_bitboard: diag_board },
			rotate_board:BitBoard{ merged_bitboard: rotate_board },
			sente_hisha_board:BitBoard{ merged_bitboard: sente_hisha_board },
			gote_hisha_board:BitBoard{ merged_bitboard: gote_hisha_board },
			sente_kaku_board:BitBoard{ merged_bitboard: sente_kaku_board },
			gote_kaku_board:BitBoard{ merged_bitboard: gote_kaku_board },
			sente_kyou_board:BitBoard{ merged_bitboard: sente_kyou_board },
			gote_kyou_board:BitBoard{ merged_bitboard: gote_kyou_board },
			sente_fu_board:BitBoard{ merged_bitboard: sente_fu_board },
			gote_fu_board:BitBoard{ merged_bitboard: gote_fu_board },
			sente_ou_position_board:BitBoard{ merged_bitboard: sente_ou_position_board },
			gote_ou_position_board:BitBoard{ merged_bitboard: gote_ou_position_board }
		}
	}

	pub fn map_banmen<F,T>(&self,mut f:F) -> T where F: FnMut(&Banmen) -> T {
		f(&self.banmen)
	}
}
const CANDIDATE_BITS:[u128; 14] = [
	// 歩
	0b000000000_010000000,
	// 桂馬(合法手の計算ではこの値は利用しない)
	0b0,
	// 桂馬
	0b100000000_000000000_100000000,
	// 銀
	0b010100000_010000000_010100000,
	// 金
	0b011000000_010100000_011000000,
	// 角(合法手の計算ではこの値は利用しない)
	0b0,
	// 飛車(合法手の計算ではこの値は利用しない)
	0b0,
	// 王
	0b011100000_010100000_011100000,
	// 成歩
	0b011000000_010100000_011000000,
	// 成香
	0b011000000_010100000_011000000,
	// 成桂
	0b011000000_010100000_011000000,
	// 成銀
	0b011000000_010100000_011000000,
	// 成角(一マスだけ進める手だけここに定義)
	0b001000000_010100000_001000000,
	// 成飛(一マスだけ進める手だけここに定義)
	0b010100000_000000000_010100000
];
const TOP_MASK: u128 = 0b001111111_001111111_001111111;
const BOTTOM_MASK: u128 = 0b111000000_111000000_111000000;
const RIGHT_MASK: u128 = 0b111111111_111111111_000000000;
const DIAG_LEFT_ROTATE_MAP:[i32; 81] = [
	-1,-1,-1,-1,-1,-1,-1,-1,-1,
	-1,21,28,34,39,43,46,48,-1,
	-1,15,22,29,35,40,44,47,-1,
	-1,10,16,23,30,36,41,45,-1,
	-1, 6,11,17,24,31,37,42,-1,
	-1, 3, 7,12,18,25,32,38,-1,
	-1, 1, 4, 8,13,19,26,33,-1,
	-1, 0, 2, 5, 9,14,20,27,-1,
	-1,-1,-1,-1,-1,-1,-1,-1,-1
];
const DIAG_RIGHT_ROTATE_MAP:[i32; 81] = [
	-1,-1,-1,-1,-1,-1,-1,-1,-1,
	-1, 0, 1, 3, 6,10,15,21,-1,
	-1, 2, 4, 7,11,16,22,28,-1,
	-1, 5, 8,12,17,23,29,34,-1,
	-1, 9,13,18,24,30,35,39,-1,
	-1,14,19,25,31,36,40,43,-1,
	-1,20,26,32,37,41,44,46,-1,
	-1,27,33,38,42,45,47,48,-1,
	-1,-1,-1,-1,-1,-1,-1,-1,-1
];
const DIAG_RIGHT_BITBOARD_SLIDE_INFO: [(i32,u32,u32); 81] = [
	(-1, 0, 1),(-1, 0, 2),( 0, 0, 3),( 1, 0, 4),( 3, 0, 5),( 6, 0, 6),(10, 0, 7),(15, 0, 8),(21, 0, 9),
	(-1, 1, 2),( 0, 1, 3),( 1, 2, 4),( 3, 1, 5),( 6, 1, 6),(10, 1, 7),(15, 1, 8),(21, 1, 9),(28, 0, 8),
	( 0, 2, 3),( 1, 2, 4),( 3, 2, 5),( 6, 2, 6),(10, 2, 7),(15, 2, 8),(21, 2, 9),(28, 1, 8),(34, 0, 7),
	( 1, 3, 4),( 3, 3, 5),( 6, 3, 6),(10, 3, 7),(15, 3, 8),(21, 3, 9),(28, 2, 8),(34, 1, 7),(39, 0, 6),
	( 3, 4, 5),( 6, 4, 6),(10, 4, 7),(15, 4, 8),(21, 4, 9),(28, 3, 8),(34, 2, 7),(39, 1, 6),(43, 0, 5),
	( 6, 5, 6),(10, 5, 7),(15, 5, 8),(21, 5, 9),(28, 4, 8),(34, 3, 7),(39, 2, 6),(43, 1, 5),(46, 0, 4),
	(10, 6, 7),(15, 6, 8),(21, 6, 9),(28, 5, 8),(34, 4, 7),(39, 3, 6),(43, 2, 5),(46, 1, 4),(48, 0, 3),
	(15, 7, 8),(21, 7, 9),(28, 6, 8),(34, 5, 7),(39, 4, 6),(43, 3, 5),(46, 2, 4),(48, 1, 3),(-1, 0, 2),
	(21, 8, 9),(28, 7, 8),(34, 6, 7),(39, 5, 6),(43, 4, 5),(46, 3, 4),(48, 2, 3),(-1, 0, 2),(-1, 0, 1)
];
const DIAG_BITBOARD_MASK: [u64; 81] = [
	0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,
	0b0,0b1,0b11,0b111,0b1111,0b11111,0b111111,0b1111111,0b0,
	0b0,0b11,0b111,0b1111,0b11111,0b111111,0b1111111,0b111111,0b0,
	0b0,0b111,0b1111,0b11111,0b111111,0b1111111,0b111111,0b11111,0b0,
	0b0,0b1111,0b11111,0b111111,0b1111111,0b111111,0b11111,0b1111,0b0,
	0b0,0b11111,0b111111,0b1111111,0b111111,0b11111,0b1111,0b111,0b0,
	0b0,0b111111,0b1111111,0b111111,0b11111,0b1111,0b111,0b11,0b0,
	0b0,0b1111111,0b111111,0b11111,0b1111,0b111,0b11,0b1,0b0,
	0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0,0b0
];
const SENTE_NARI_MASK: u128 = 0b000000111_000000111_000000111_000000111_000000111_000000111_000000111_000000111_000000111;
const GOTE_NARI_MASK: u128 = 0b111000000_111000000_111000000_111000000_111000000_111000000_111000000_111000000_111000000;
const DENY_MOVE_SENTE_FU_AND_KYOU_MASK: u128 = 0b000000001_000000001_000000001_000000001_000000001_000000001_000000001_000000001_000000001;
const DENY_MOVE_SENTE_KEI_MASK: u128 = 0b000000011_000000011_000000011_000000011_000000011_000000011_000000011_000000011_000000011;
const DENY_MOVE_GOTE_FU_AND_KYOU_MASK: u128 = 0b100000000_100000000_100000000_100000000_100000000_100000000_100000000_100000000_100000000;
const DENY_MOVE_GOTE_KEI_MASK: u128 = 0b110000000_110000000_110000000_110000000_110000000_110000000_110000000_110000000_110000000;
enum NextMove {
	Once(i32,i32),
	Repeat(i32,i32),
}
const CANDIDATE:[&[NextMove]; 14] = [
	// 歩
	&[NextMove::Once(0,-1)],
	// 香車
	&[NextMove::Repeat(0,-1)],
	// 桂馬
	&[NextMove::Once(-1,-2),NextMove::Once(1,-2)],
	// 銀
	&[NextMove::Once(-1,-1),
		NextMove::Once(-1,1),
		NextMove::Once(0,-1),
		NextMove::Once(1,-1),
		NextMove::Once(1,1)
	],
	// 金
	&[NextMove::Once(-1,-1),
		NextMove::Once(-1,0),
		NextMove::Once(0,-1),
		NextMove::Once(0,1),
		NextMove::Once(1,-1),
		NextMove::Once(1,0)
	],
	// 角
	&[NextMove::Repeat(-1,-1),
		NextMove::Repeat(1,-1),
		NextMove::Repeat(-1,1),
		NextMove::Repeat(1,1)
	],
	// 飛車
	&[NextMove::Repeat(0,-1),
		NextMove::Repeat(0,1),
		NextMove::Repeat(-1,0),
		NextMove::Repeat(1,0)
	],
	// 王
	&[NextMove::Once(-1,-1),
		NextMove::Once(-1,0),
		NextMove::Once(-1,1),
		NextMove::Once(0,-1),
		NextMove::Once(0,1),
		NextMove::Once(1,-1),
		NextMove::Once(1,0),
		NextMove::Once(1,1)
	],
	// 成歩
	&[NextMove::Once(-1,-1),
		NextMove::Once(-1,0),
		NextMove::Once(0,-1),
		NextMove::Once(0,1),
		NextMove::Once(1,-1),
		NextMove::Once(1,0)
	],
	// 成香
	&[NextMove::Once(-1,-1),
		NextMove::Once(-1,0),
		NextMove::Once(0,-1),
		NextMove::Once(0,1),
		NextMove::Once(1,-1),
		NextMove::Once(1,0)
	],
	// 成桂
	&[NextMove::Once(-1,-1),
		NextMove::Once(-1,0),
		NextMove::Once(0,-1),
		NextMove::Once(0,1),
		NextMove::Once(1,-1),
		NextMove::Once(1,0)
	],
	// 成銀
	&[NextMove::Once(-1,-1),
		NextMove::Once(-1,0),
		NextMove::Once(0,-1),
		NextMove::Once(0,1),
		NextMove::Once(1,-1),
		NextMove::Once(1,0)
	],
	// 成角
	&[NextMove::Repeat(-1,-1),
		NextMove::Repeat(1,-1),
		NextMove::Repeat(-1,1),
		NextMove::Repeat(1,1),
		NextMove::Once(-1,0),
		NextMove::Once(0,-1),
		NextMove::Once(0,1),
		NextMove::Once(1,0)
	],
	// 成飛
	&[NextMove::Repeat(0,-1),
		NextMove::Repeat(0,1),
		NextMove::Repeat(-1,0),
		NextMove::Repeat(1,0),
		NextMove::Once(-1,-1),
		NextMove::Once(-1,1),
		NextMove::Once(1,-1),
		NextMove::Once(1,1)
	],
];
/// 左上を(0,0)とした位置
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
pub trait Validate {
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
pub struct Rule {

}
impl Rule {
	pub fn legal_moves_once_with_point_and_kind_and_bitboard_and_buffer(
		teban:Teban,self_occupied:BitBoard,from:u32,kind:KomaKind,mvs:&mut Vec<Square>
	) {
		let from = if teban == Teban::Sente {
			from
		} else {
			80 - from
		};

		let x = from / 9;
		let y = from - x * 9;

		let mut mask = CANDIDATE_BITS[kind as usize];

		if y == 0 || ((kind == SKei || kind == GKei) && y <= 1) {
			mask = mask & TOP_MASK;
		} else if y == 8 {
			mask = mask & BOTTOM_MASK;
		}

		if x == 8 {
			mask = mask & RIGHT_MASK;
		}

		let mask = mask as u128;
		let self_occupied = unsafe {
			match self_occupied {
				BitBoard { merged_bitboard } => {
					merged_bitboard
				}
			}
		};

		let mut board = !self_occupied;

		if from < 10 {
			board &= mask >> (11 - from - 1);
		} else if from == 10 {
			board &= mask;
		} else {
			board &= mask << (from - 11 + 1);
		}

		let mut board = BitBoard { merged_bitboard: board };

		loop {
			let p = Rule::pop_lsb(&mut board);

			if p == -1 {
				break;
			} else if teban == Teban::Sente {
				mvs.push(p);
			} else {
				mvs.push(80 - p);
			}
		}
	}

	#[inline]
	pub fn legal_moves_once_with_point_and_kind_and_bitboard(
		teban:Teban,self_occupied:BitBoard,from:u32,kind:KomaKind
	) -> Vec<Square> {
		let mut mvs:Vec<Square> = Vec::with_capacity(8);

		Rule::legal_moves_once_with_point_and_kind_and_bitboard_and_buffer(teban,self_occupied,from,kind,&mut mvs);

		mvs
	}

	pub fn legal_moves_sente_kaku_with_point_and_kind_and_bitboard(
		self_occupied:BitBoard,diag_bitboard:BitBoard,from:u32,kind:KomaKind
	) -> Vec<Square> {
		let mut mvs:Vec<Square> = Vec::with_capacity(20);

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(0)
		};

		let count = Rule::calc_to_left_top_move_count_of_kaku(board, from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to -= 10;
				mvs.push(to as Square);
				c += 1;
			}

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			to -= 10;

			if self_occupied & 1 << (to + 1) == 0 {
				mvs.push(to as Square);
			}
		}

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(1)
		};

		let count = Rule::calc_to_right_top_move_count_of_kaku(board, from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to -= 8;
				mvs.push(to as Square);
				c += 1;
			}

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			to -= 8;

			if self_occupied & 1 << (to + 1) == 0 {
				mvs.push(to as Square);
			}
		}

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(1)
		};

		let count = Rule::calc_to_left_bottom_move_count_of_kaku(board, from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to +=8;
				mvs.push(to as Square);
				c += 1;
			}

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			to += 8;

			if self_occupied & 1 << (to + 1) == 0 {
				mvs.push(to as Square);
			}
		}

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(0)
		};

		let count = Rule::calc_to_right_bottom_move_count_of_kaku(board, from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to += 10;
				mvs.push(to as Square);
				c += 1;
			}

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			to += 10;

			if self_occupied & 1 << (to + 1) == 0 {
				mvs.push(to as Square);
			}
		}

		if kind == SKakuN {
			Rule::legal_moves_once_with_point_and_kind_and_bitboard_and_buffer(
				Teban::Sente,self_occupied,from,kind,&mut mvs
			);
		}

		mvs
	}

	pub fn legal_moves_gote_kaku_with_point_and_kind_and_bitboard(
		self_occupied:BitBoard,diag_bitboard:BitBoard,from:u32,kind:KomaKind
	) -> Vec<Square> {
		let mut mvs:Vec<Square> = Vec::with_capacity(20);

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(0)
		};

		let count = Rule::calc_to_right_bottom_move_count_of_kaku(board, from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to += 10;
				mvs.push(to as Square);
				c += 1;
			}

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			to += 10;

			if self_occupied & 1 << (80 - to + 1) == 0 {
				mvs.push(to as Square);
			}
		}

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(1)
		};

		let count = Rule::calc_to_left_bottom_move_count_of_kaku(board, from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to += 8;
				mvs.push(to as Square);
				c += 1;
			}

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			to += 8;

			if self_occupied & 1 << (80 - to + 1) == 0 {
				mvs.push(to as Square);
			}
		}

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(1)
		};

		let count = Rule::calc_to_right_top_move_count_of_kaku(board, from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to -= 8;
				mvs.push(to as Square);
				c += 1;
			}

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			to -= 8;

			if self_occupied & 1 << (80 - to + 1) == 0 {
				mvs.push(to as Square);
			}
		}

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(0)
		};

		let count = Rule::calc_to_left_top_move_count_of_kaku(board, from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to -= 10;
				mvs.push(to as Square);
				c += 1;
			}

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			to -= 10;

			if self_occupied & 1 << (80 - to + 1) == 0 {
				mvs.push(to as Square);
			}
		}

		if kind == GKakuN {
			Rule::legal_moves_once_with_point_and_kind_and_bitboard_and_buffer(
				Teban::Gote,self_occupied,from,kind,&mut mvs
			);
		}

		mvs
	}

	pub fn calc_to_bottom_move_count_of_kaku(diag_bitboard:u64,from:u32) -> u32 {
		let (row_offset,offset,row_width) = match DIAG_RIGHT_BITBOARD_SLIDE_INFO[from as usize] {
			(row_offset,offset,row_width) => {
				(row_offset,offset,row_width)
			}
		};

		if row_offset == -1 {
			return 0;
		} else {
			let row = if row_offset == 0 {
				diag_bitboard & DIAG_BITBOARD_MASK[from as usize]
			} else {
				(diag_bitboard >> row_offset) & DIAG_BITBOARD_MASK[from as usize]
			};

			let row = row >> offset + 1;

			let l = if row == 0 {
				row_width - 1 - offset
			} else {
				row.trailing_zeros() + 1
			};

			l
		}
	}

	#[inline]
	pub fn calc_to_left_bottom_move_count_of_kaku(r_diag_bitboard:u64,from:u32) -> u32 {
		Rule::calc_to_bottom_move_count_of_kaku(r_diag_bitboard,from)
	}

	#[inline]
	pub fn calc_to_right_bottom_move_count_of_kaku(l_diag_bitboard:u64,from:u32) -> u32 {
		let x = from / 9;
		let y = from - x * 9;

		let (x,y) = {
			(y,8-x)
		};

		let from = y * 9 + x;

		Rule::calc_to_bottom_move_count_of_kaku(l_diag_bitboard,from)
	}

	pub fn calc_to_top_move_count_of_kaku(diag_bitboard:u64,from:u32) -> u32 {
		let (row_offset,mask_row_offset,offset,row_width) = match DIAG_RIGHT_BITBOARD_SLIDE_INFO[from as usize] {
			(row_offset,offset,row_width) => {
				if row_offset == -1 {
					return 0;
				}

				(
					64 - row_offset - row_width as i32,
					64 - row_offset,
					row_width - 1 - offset,
					row_width
				)
			}
		};

		let row = (diag_bitboard << row_offset) & (DIAG_BITBOARD_MASK[from as usize] << mask_row_offset);

		let row = row << offset + 1;

		let l = if row == 0 {
			row_width - 1 - offset
		} else {
			row.leading_zeros() + 1
		};

		l
	}

	#[inline]
	pub fn calc_to_right_top_move_count_of_kaku(r_diag_bitboard:u64,from:u32) -> u32 {
		Rule::calc_to_top_move_count_of_kaku(r_diag_bitboard,from)
	}

	#[inline]
	pub fn calc_to_left_top_move_count_of_kaku(l_diag_bitboard:u64,from:u32) -> u32 {
		let x = from / 9;
		let y = from - x * 9;

		let (x,y) = {
			(y,8-x)
		};

		let from = y * 9 + x;

		Rule::calc_to_top_move_count_of_kaku(l_diag_bitboard,from)
	}

	pub fn legal_moves_sente_hisha_with_point_and_kind_and_bitboard(
		self_occupied:BitBoard,bitboard:BitBoard,rotate_bitboard:BitBoard,from:u32,kind:KomaKind
	) -> Vec<Square> {
		let mut mvs:Vec<Square> = Vec::with_capacity(20);

		let count = Rule::calc_to_top_move_count_of_hisha(bitboard,from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to -= 1;
				mvs.push(to as Square);
				c += 1;
			}

			to -= 1;

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			if self_occupied & 1 << (to + 1) != 0 {
				mvs.push(to as Square);
			}
		}

		let count = Rule::calc_to_bottom_move_count_of_hisha(bitboard,from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to += 1;
				mvs.push(to as Square);
				c += 1;
			}

			to += 1;

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			if self_occupied & 1 << (to + 1) != 0 {
				mvs.push(to as Square);
			}
		}

		let count = Rule::calc_to_left_move_count_of_hisha(rotate_bitboard,from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to -= 9;
				mvs.push(to as Square);
				c += 1;
			}

			to -= 9;

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			if self_occupied & 1 << (to + 1) != 0 {
				mvs.push(to as Square);
			}
		}

		let count = Rule::calc_to_right_move_count_of_hisha(rotate_bitboard,from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to += 9;
				mvs.push(to as Square);
				c += 1;
			}

			to += 9;

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			if self_occupied & 1 << (to + 1) != 0 {
				mvs.push(to as Square);
			}
		}

		if kind == SHishaN {
			Rule::legal_moves_once_with_point_and_kind_and_bitboard_and_buffer(
				Teban::Sente,self_occupied,from,kind,&mut mvs
			);
		}

		mvs
	}

	pub fn legal_moves_gote_hisha_with_point_and_kind_and_bitboard(
		self_occupied:BitBoard,bitboard:BitBoard,rotate_bitboard:BitBoard,from:u32,kind:KomaKind
	) -> Vec<Square> {
		let mut mvs:Vec<Square> = Vec::with_capacity(20);

		let count = Rule::calc_to_bottom_move_count_of_hisha(bitboard,from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to += 1;
				mvs.push(to as Square);
				c += 1;
			}

			to += 1;

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			if self_occupied & 1 << (80 - to + 1) != 0 {
				mvs.push(to as Square);
			}
		}

		let count = Rule::calc_to_top_move_count_of_hisha(bitboard,from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to -= 1;
				mvs.push(to as Square);
				c += 1;
			}

			to -= 1;

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			if self_occupied & 1 << (80 - to + 1) != 0 {
				mvs.push(to as Square);
			}
		}

		let count = Rule::calc_to_right_move_count_of_hisha(rotate_bitboard,from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to += 9;
				mvs.push(to as Square);
				c += 1;
			}

			to += 9;

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			if self_occupied & 1 << (80 - to + 1) != 0 {
				mvs.push(to as Square);
			}
		}

		let count = Rule::calc_to_left_move_count_of_hisha(rotate_bitboard,from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to -= 9;
				mvs.push(to as Square);
				c += 1;
			}

			to -= 9;

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			if self_occupied & 1 << (80 - to + 1) != 0 {
				mvs.push(to as Square);
			}
		}

		if kind == GHishaN {
			Rule::legal_moves_once_with_point_and_kind_and_bitboard_and_buffer(
				Teban::Gote,self_occupied,from,kind,&mut mvs
			);
		}

		mvs
	}

	#[inline]
	pub fn calc_to_bottom_move_count_of_hisha(bitboard:BitBoard,from:u32) -> u32 {
		Rule::calc_back_move_repeat_count(bitboard,from)
	}

	#[inline]
	pub fn calc_to_top_move_count_of_hisha(bitboard:BitBoard,from:u32) -> u32 {
		Rule::calc_forward_move_repeat_count(bitboard,from)
	}

	#[inline]
	pub fn calc_to_left_move_count_of_hisha(bitboard:BitBoard,from:u32) -> u32 {
		Rule::calc_back_move_repeat_count(bitboard,from)
	}

	#[inline]
	pub fn calc_to_right_move_count_of_hisha(bitboard:BitBoard,from:u32) -> u32 {
		Rule::calc_forward_move_repeat_count(bitboard,from)
	}

	pub fn legal_moves_sente_kyou_with_point_and_kind_and_bitboard(
		self_occupied:BitBoard,bitboard:BitBoard,from:u32
	) -> Vec<Square> {
		let mut mvs:Vec<Square> = Vec::with_capacity(8);

		let count = Rule::calc_forward_move_repeat_count(bitboard,from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to -= 1;
				mvs.push(to as Square);
				c += 1;
			}

			to -= 1;

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			if self_occupied & 1 << (to + 1) != 0 {
				mvs.push(to as Square);
			}
		}

		mvs
	}

	pub fn legal_moves_gote_kyou_with_point_and_kind_and_bitboard(
		self_occupied:BitBoard,bitboard:BitBoard,from:u32
	) -> Vec<Square> {
		let mut mvs:Vec<Square> = Vec::with_capacity(8);

		let count = Rule::calc_back_move_repeat_count(bitboard,from);

		if count > 0 {
			let mut c = 1;
			let mut to = from;

			while c < count {
				to += 1;
				mvs.push(to as Square);
				c += 1;
			}

			to += 1;

			let self_occupied = unsafe {
				self_occupied.merged_bitboard
			};

			if self_occupied & 1 << (80 - to + 1) != 0 {
				mvs.push(to as Square);
			}
		}

		mvs
	}

	pub fn calc_back_move_repeat_count(bitboard:BitBoard,from:u32) -> u32 {
		let board_x = from / 9;
		let board_y = from - board_x * 9;

		let board = unsafe {
			BitBoard {
				merged_bitboard: (bitboard.merged_bitboard << board_x * 9) & 0b111111111
			}
		};

		let board = unsafe { *board.bitboard.get_unchecked(0) };
		let board = board >> (board_y + 1);

		if board == 0 {
			8 - board_y
		} else {
			board.trailing_zeros() + 1
		}
	}

	pub fn calc_forward_move_repeat_count(bitboard:BitBoard,from:u32) -> u32 {
		let board_x = from / 9;
		let board_y = from - board_x * 9;

		let board = unsafe {
			BitBoard {
				merged_bitboard: (
					(bitboard.merged_bitboard << (127 - 8 - board_x * 9 - 1))
				) & 0b111111111 << 119
			}
		};

		let board = unsafe { *board.bitboard.get_unchecked(1) };
		let board = board << (8 - board_y + 1);

		if board == 0 {
			board_y
		} else {
			board.leading_zeros() + 1
		}
	}

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
				*(bitboard.bitboard.get_unchecked_mut(0)) &= *bitboard.bitboard.get_unchecked(0) - 1;
			}

			return p - 1;
		} else if br != 0 {
			let p = br.trailing_zeros() as Square;
			unsafe {
				*(bitboard.bitboard.get_unchecked_mut(1)) &= *bitboard.bitboard.get_unchecked(1) - 1;
			}

			return p + 63;
		} else {
			return -1;
		}
	}

	pub fn legal_moves_with_point_and_kind(
		t:&Teban,state:&State,x:u32,y:u32,kind:KomaKind
	) -> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		Rule::legal_moves_with_point_and_kind_and_buffer(t,state,x,y,kind,&mut mvs);

		mvs
	}

	pub fn legal_moves_with_point_and_kind_and_buffer(
		t:&Teban,state:&State,x:u32,y:u32,kind:KomaKind,mvs:&mut Vec<LegalMove>
	) {
		let from = x * 9 + y;

		let kinds = match &state.banmen {
			&Banmen(ref kinds) => kinds
		};

		match kind {
			SFu => {
				for m in Rule::legal_moves_once_with_point_and_kind_and_bitboard(
					*t,state.sente_self_board,from,kind
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_bitboard = unsafe { state.sente_opponent_board.merged_bitboard };

					let o = if opponent_bitboard & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					if SENTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					if DENY_MOVE_SENTE_FU_AND_KYOU_MASK & to_mask == 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
					}
				}
			},
			SKyou => {
				let bitboard = state.sente_self_board | state.sente_opponent_board;

				for m in Rule::legal_moves_sente_kyou_with_point_and_kind_and_bitboard(
					state.sente_self_board, bitboard, from
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_bitboard = unsafe { state.sente_opponent_board.merged_bitboard };

					let o = if opponent_bitboard & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					if SENTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					if DENY_MOVE_SENTE_FU_AND_KYOU_MASK & to_mask == 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
					}
				}
			}
			SKei => {
				for m in Rule::legal_moves_once_with_point_and_kind_and_bitboard(
					*t,state.sente_self_board,from,kind
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_bitboard = unsafe { state.sente_opponent_board.merged_bitboard };

					let o = if opponent_bitboard & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					if SENTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					if DENY_MOVE_SENTE_KEI_MASK & to_mask == 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
					}
				}
			},
			SKaku | SKakuN => {
				for m in Rule::legal_moves_sente_kaku_with_point_and_kind_and_bitboard(
					state.sente_self_board, state.diag_board, from, kind
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_bitboard = unsafe { state.sente_opponent_board.merged_bitboard };

					let o = if opponent_bitboard & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					if kind == SKaku && SENTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}
					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			SHisha | SHishaN => {
				let bitboard = state.sente_self_board | state.sente_opponent_board;

				for m in Rule::legal_moves_sente_hisha_with_point_and_kind_and_bitboard(
					state.sente_self_board, bitboard, state.rotate_board, from, kind
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_bitboard = unsafe { state.sente_opponent_board.merged_bitboard };

					let o = if opponent_bitboard & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					if kind == SHisha && SENTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}
					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			SGin | SOu =>  {
				for m in Rule::legal_moves_once_with_point_and_kind_and_bitboard(
					*t,state.sente_self_board,from,kind
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_bitboard = unsafe { state.sente_opponent_board.merged_bitboard };

					let o = if opponent_bitboard & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					if SENTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}
					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			SFuN | SKyouN | SKeiN | SGinN | SKin => {
				for m in Rule::legal_moves_once_with_point_and_kind_and_bitboard(
					*t,state.sente_self_board,from,kind
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_bitboard = unsafe { state.sente_opponent_board.merged_bitboard };

					let o = if opponent_bitboard & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			GFu => {
				for m in Rule::legal_moves_once_with_point_and_kind_and_bitboard(
					*t,state.gote_self_board,from,kind
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_board = unsafe { state.sente_self_board.merged_bitboard };

					let o = if opponent_board & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					if GOTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					if DENY_MOVE_GOTE_FU_AND_KYOU_MASK & to_mask == 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
					}
				}
			},
			GKyou => {
				let bitboard = state.sente_self_board | state.sente_opponent_board;

				for m in Rule::legal_moves_gote_kyou_with_point_and_kind_and_bitboard(
					state.gote_self_board, bitboard, from
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_board = unsafe { state.sente_self_board.merged_bitboard };

					let o = if opponent_board & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					if GOTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					if DENY_MOVE_GOTE_FU_AND_KYOU_MASK & to_mask == 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
					}
				}
			},
			GKei => {
				for m in Rule::legal_moves_once_with_point_and_kind_and_bitboard(
					*t,state.gote_self_board,from,kind
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_board = unsafe { state.sente_self_board.merged_bitboard };

					let o = if opponent_board & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					if GOTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					if DENY_MOVE_GOTE_KEI_MASK & to_mask == 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
					}
				}
			},
			GKaku | GKakuN => {
				for m in Rule::legal_moves_gote_kaku_with_point_and_kind_and_bitboard(
					state.gote_self_board, state.diag_board, from, kind
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_board = unsafe { state.sente_self_board.merged_bitboard };

					let o = if opponent_board & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					if kind == GKaku && GOTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}
					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			GHisha | GHishaN => {
				let bitboard = state.sente_self_board | state.sente_opponent_board;

				for m in Rule::legal_moves_gote_hisha_with_point_and_kind_and_bitboard(
					state.gote_self_board, bitboard, state.rotate_board, from, kind
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_board = unsafe { state.sente_self_board.merged_bitboard };

					let o = if opponent_board & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					if kind == GHisha && GOTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}
					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			GGin | GOu =>  {
				for m in Rule::legal_moves_once_with_point_and_kind_and_bitboard(
					*t,state.gote_self_board,from,kind
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_board = unsafe { state.sente_self_board.merged_bitboard };

					let o = if opponent_board & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					if GOTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}
					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			GFuN | GKyouN | GKeiN | GGinN | GKin => {
				for m in Rule::legal_moves_once_with_point_and_kind_and_bitboard(
					*t,state.gote_self_board,from,kind
				) {
					let to = m as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_board = unsafe { state.sente_self_board.merged_bitboard };

					let o = if opponent_board & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			Blank => (),
		}
	}

	pub fn legal_moves_with_point(t:&Teban,state:&State,x:u32,y:u32)
		-> Vec<LegalMove> {
		match &state.banmen {
			&Banmen(ref kinds) => {
				Rule::legal_moves_with_point_and_kind(t,state,x,y,kinds[y as usize][x as usize])
			}
		}
	}
	pub fn legal_moves_with_src(t:&Teban,state:&State,src:KomaSrcPosition)
		-> Vec<LegalMove> {
		match src {
			KomaSrcPosition(x,y) => Rule::legal_moves_with_point(t, state, 9 - x, y - 1)
		}
	}

	pub fn legal_moves_with_dst_to(t:&Teban,state:&State,dst:KomaDstToPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstToPosition(x,y,_) => Rule::legal_moves_with_point(t, state, 9 - x, y - 1)
		}
	}

	pub fn legal_moves_with_dst_put(t:&Teban,state:&State,dst:KomaDstPutPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstPutPosition(x,y) => Rule::legal_moves_with_point(t, state, 9 - x, y - 1)
		}
	}


	pub fn legal_moves_from_banmen(t:&Teban,state:&State)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		Rule::legal_moves_from_banmen_with_buffer(t,state,&mut mvs);

		mvs
	}

	pub fn legal_moves_from_banmen_with_buffer(t:&Teban,state:&State,mvs:&mut Vec<LegalMove>) {
		match &state.banmen {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match *t {
							Teban::Sente => (x,y),
							Teban::Gote => (8-x,8-y),
						};

						Rule::legal_moves_with_point_and_kind_and_buffer(
							t, state, x as u32, y as u32, kinds[y][x], &mut mvs
						);
					}
				}
			}
		}
	}

	pub fn legal_moves_from_mochigoma(t:&Teban,mc:&MochigomaCollections,state:&State)
		-> Vec<LegalMove> {

		let mut mvs:Vec<LegalMove> = Vec::new();

		Rule::legal_moves_from_mochigoma_with_buffer(t,mc,state,&mut mvs);

		mvs
	}

	pub fn legal_moves_from_mochigoma_with_buffer(
		t:&Teban,mc:&MochigomaCollections,state:&State,mvs:&mut Vec<LegalMove>
	) {
		match *t {
			Teban::Sente => {
				match *mc {
					MochigomaCollections::Pair(ref ms, _) => {
						for m in &MOCHIGOMA_KINDS {
							match ms.get(&m) {
								None | Some(&0) => {
									continue;
								},
								Some(_) => (),
							}

							match *m {
								MochigomaKind::Fu | MochigomaKind::Kyou => {
									let candidate_bitboard = {
										state.sente_self_board | state.sente_opponent_board
									};

									let mut candidate_bitboard = !candidate_bitboard;

									while let p = Rule::pop_lsb(&mut candidate_bitboard) {
										if p == -1 {
											break;
										}

										let p_mask = 1 << p;

										if DENY_MOVE_SENTE_FU_AND_KYOU_MASK & p_mask != 0 {
											continue;
										}

										mvs.push(LegalMove::Put(LegalMovePut::new(*m,p as u32)));
									}
								},
								MochigomaKind::Kei => {
									let candidate_bitboard = {
										state.sente_self_board | state.sente_opponent_board
									};

									let mut candidate_bitboard = !candidate_bitboard;

									while let p = Rule::pop_lsb(&mut candidate_bitboard) {
										if p == -1 {
											break;
										}

										let p_mask = 1 << p;

										if DENY_MOVE_SENTE_KEI_MASK & p_mask != 0 {
											continue;
										}

										mvs.push(LegalMove::Put(LegalMovePut::new(*m,p as u32)));
									}
								},
								_ => {
									let candidate_bitboard = {
										state.sente_self_board | state.sente_opponent_board
									};

									let mut candidate_bitboard = !candidate_bitboard;

									while let p = Rule::pop_lsb(&mut candidate_bitboard) {
										if p == -1 {
											break;
										}

										let p_mask = 1 << p;

										mvs.push(LegalMove::Put(LegalMovePut::new(*m,p as u32)));
									}
								}
							}
						}
					},
					MochigomaCollections::Empty => (),
				}
			},
			Teban::Gote => {
				match *mc {
					MochigomaCollections::Pair(_, ref mg) => {
						for m in &MOCHIGOMA_KINDS {
							match mg.get(&m) {
								None | Some(&0) => {
									continue;
								},
								Some(_) => (),
							}

							match *m {
								MochigomaKind::Fu | MochigomaKind::Kyou => {
									let candidate_bitboard = {
										state.gote_self_board | state.gote_opponent_board
									};

									let mut candidate_bitboard = !candidate_bitboard;

									while let p = Rule::pop_lsb(&mut candidate_bitboard) {
										if p == -1 {
											break;
										}

										let p = 80 - p;

										let p_mask = 1 << p;

										if DENY_MOVE_GOTE_FU_AND_KYOU_MASK & p_mask != 0 {
											continue;
										}

										mvs.push(LegalMove::Put(LegalMovePut::new(*m,p as u32)));
									}
								},
								MochigomaKind::Kei => {
									let candidate_bitboard = {
										state.gote_self_board | state.gote_opponent_board
									};

									let mut candidate_bitboard = !candidate_bitboard;

									while let p = Rule::pop_lsb(&mut candidate_bitboard) {
										if p == -1 {
											break;
										}

										let p = 80 - p;

										let p_mask = 1 << p;

										if DENY_MOVE_GOTE_KEI_MASK & p_mask != 0 {
											continue;
										}

										mvs.push(LegalMove::Put(LegalMovePut::new(*m,p as u32)));
									}
								},
								_ => {
									let candidate_bitboard = {
										state.gote_self_board | state.gote_opponent_board
									};

									let mut candidate_bitboard = !candidate_bitboard;

									while let p = Rule::pop_lsb(&mut candidate_bitboard) {
										if p == -1 {
											break;
										}

										let p = 80 - p;

										let p_mask = 1 << p;

										mvs.push(LegalMove::Put(LegalMovePut::new(*m,p as u32)));
									}
								}
							}
						}
					},
					MochigomaCollections::Empty => (),
				}
			}
		}
	}

	pub fn legal_moves_all(t:&Teban,state:&State,mc:&MochigomaCollections)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		Rule::legal_moves_from_banmen_with_buffer(t, state, &mut mvs);
		Rule::legal_moves_from_mochigoma_with_buffer(t, mc, state, &mut mvs);
		mvs
	}

	pub fn win_only_move_once_with_point_and_kind_and_bitboard(
		teban:Teban,self_occupied:BitBoard,self_ou_bitboard:BitBoard,from:u32,kind:KomaKind
	) -> Option<Square> {
		let from = if teban == Teban::Sente {
			from
		} else {
			80 - from
		};

		let x = from / 9;
		let y = from - x * 9;

		let mut mask = CANDIDATE_BITS[kind as usize];

		if y == 0 || ((kind == SKei || kind == GKei) && y <= 1) {
			mask = mask & TOP_MASK;
		} else if y == 8 {
			mask = mask & BOTTOM_MASK;
		}

		if x == 8 {
			mask = mask & RIGHT_MASK;
		}

		let mask = mask as u128;
		let self_occupied = unsafe {
			match self_occupied {
				BitBoard { merged_bitboard } => {
					merged_bitboard
				}
			}
		};

		let mut board = !self_occupied;

		if from < 10 {
			board &= mask >> (11 - from - 1);
		} else if from == 10 {
			board &= mask;
		} else {
			board &= mask << (from - 11 + 1);
		}

		let self_ou_bitboard = unsafe {
			self_ou_bitboard.merged_bitboard
		};

		if self_ou_bitboard & board != 0 {
			let mut board = BitBoard { merged_bitboard: self_ou_bitboard };
			let p = Rule::pop_lsb(&mut board);

			match teban {
				Teban::Sente => Some(p),
				Teban::Gote => Some(80 - p),
			}
		} else {
			None
		}
	}

	pub fn win_only_move_sente_kaku_with_point_and_kind_and_bitboard(
		self_occupied:BitBoard,self_ou_bitboard:BitBoard,diag_bitboard:BitBoard,from:u32,kind:KomaKind
	) -> Option<Square> {
		let self_ou_bitboard_raw = unsafe {
			self_ou_bitboard.merged_bitboard
		};

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(0)
		};

		let count = Rule::calc_to_left_top_move_count_of_kaku(board, from);

		if count > 0 {
			let p = from - 10 * count;

			if self_ou_bitboard_raw & (1 << p) != 0 {
				return Some(p as Square);
			}
		}

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(1)
		};

		let count = Rule::calc_to_right_top_move_count_of_kaku(board, from);

		if count > 0 {
			let p = from - 8 * count;

			if self_ou_bitboard_raw & (1 << p) != 0 {
				return Some(p as Square);
			}
		}

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(1)
		};

		let count = Rule::calc_to_left_bottom_move_count_of_kaku(board, from);

		if count > 0 {
			let p = from + 8 * count;

			if self_ou_bitboard_raw & (1 << p) != 0 {
				return Some(p as Square);
			}
		}

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(0)
		};

		let count = Rule::calc_to_right_bottom_move_count_of_kaku(board, from);

		if count > 0 {
			let p = from + 10 * count;

			if self_ou_bitboard_raw & (1 << p) != 0 {
				return Some(p as Square);
			}
		}

		if kind == SKakuN {
			Rule::win_only_move_once_with_point_and_kind_and_bitboard(
				Teban::Sente,self_occupied,self_ou_bitboard,from,kind
			)
		} else {
			None
		}
	}

	pub fn win_only_move_gote_kaku_with_point_and_kind_and_bitboard(
		self_occupied:BitBoard,self_ou_bitboard:BitBoard,diag_bitboard:BitBoard,from:u32,kind:KomaKind
	) -> Option<Square> {
		let self_ou_bitboard_raw = unsafe {
			self_ou_bitboard.merged_bitboard
		};

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(0)
		};

		let count = Rule::calc_to_right_bottom_move_count_of_kaku(board, from);

		if count > 0 {
			let p = from + 10 * count;

			if self_ou_bitboard_raw & (1 << p) != 0 {
				return Some(p as Square);
			}
		}

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(1)
		};

		let count = Rule::calc_to_left_bottom_move_count_of_kaku(board, from);

		if count > 0 {
			let p = from + 8 * count;

			if self_ou_bitboard_raw & (1 << (80 - p)) != 0 {
				return Some(p as Square);
			}
		}

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(1)
		};

		let count = Rule::calc_to_right_top_move_count_of_kaku(board, from);

		if count > 0 {
			let p = from - 8 * count;

			if self_ou_bitboard_raw & (1 << (80 - p)) != 0 {
				return Some(p as Square);
			}
		}

		let board = unsafe {
			*diag_bitboard.bitboard.get_unchecked(0)
		};

		let count = Rule::calc_to_left_top_move_count_of_kaku(board, from);

		if count > 0 {
			let p = from - 10 * count;

			if self_ou_bitboard_raw & (1 << (80 - p)) != 0 {
				return Some(p as Square);
			}
		}

		if kind == GKakuN {
			Rule::win_only_move_once_with_point_and_kind_and_bitboard(
				Teban::Gote,self_occupied,self_ou_bitboard,from,kind
			)
		} else {
			None
		}
	}


	pub fn win_only_move_sente_hisha_with_point_and_kind_and_bitboard(
		self_occupied:BitBoard,self_ou_bitboard:BitBoard,bitboard:BitBoard,rotate_bitboard:BitBoard,from:u32,kind:KomaKind
	) -> Option<Square> {
		let self_ou_bitboard_raw = unsafe {
			self_ou_bitboard.merged_bitboard
		};

		let count = Rule::calc_to_top_move_count_of_hisha(bitboard,from);

		if count > 0 {
			let p = from - count;

			if self_ou_bitboard_raw & (1 << p) != 0 {
				return Some(p as Square);
			}
		}

		let count = Rule::calc_to_bottom_move_count_of_hisha(bitboard,from);

		if count > 0 {
			let p = from + count;

			if self_ou_bitboard_raw & (1 << p) != 0 {
				return Some(p as Square);
			}
		}

		let count = Rule::calc_to_left_move_count_of_hisha(rotate_bitboard,from);

		if count > 0 {
			let p = from - 9 * count;

			if self_ou_bitboard_raw & (1 << p) != 0 {
				return Some(p as Square);
			}
		}

		let count = Rule::calc_to_right_move_count_of_hisha(rotate_bitboard,from);

		if count > 0 {
			let p = from + 9 * count;

			if self_ou_bitboard_raw & (1 << p) != 0 {
				return Some(p as Square);
			}
		}

		if kind == SHishaN {
			Rule::win_only_move_once_with_point_and_kind_and_bitboard(
				Teban::Sente,self_occupied,self_ou_bitboard,from,kind
			)
		} else {
			None
		}
	}

	pub fn win_only_move_gote_hisha_with_point_and_kind_and_bitboard(
		self_occupied:BitBoard,self_ou_bitboard:BitBoard,bitboard:BitBoard,rotate_bitboard:BitBoard,from:u32,kind:KomaKind
	) -> Option<Square> {
		let self_ou_bitboard_raw = unsafe {
			self_ou_bitboard.merged_bitboard
		};

		let count = Rule::calc_to_bottom_move_count_of_hisha(bitboard,from);

		if count > 0 {
			let p = from + count;

			if self_ou_bitboard_raw & (1 << (80 - p)) != 0 {
				return Some(p as Square);
			}
		}

		let count = Rule::calc_to_top_move_count_of_hisha(bitboard,from);

		if count > 0 {
			let p = from - count;

			if self_ou_bitboard_raw & (1 << (80 - p)) != 0 {
				return Some(p as Square);
			}
		}

		let count = Rule::calc_to_right_move_count_of_hisha(rotate_bitboard,from);

		if count > 0 {
			let p = from + 9 * count;

			if self_ou_bitboard_raw & (1 << (80 - p)) != 0 {
				return Some(p as Square);
			}
		}

		let count = Rule::calc_to_left_move_count_of_hisha(rotate_bitboard,from);

		if count > 0 {
			let p = from - 9 * count;

			if self_ou_bitboard_raw & (1 << (80 - p)) != 0 {
				return Some(p as Square);
			}
		}

		if kind == GHishaN {
			Rule::win_only_move_once_with_point_and_kind_and_bitboard(
				Teban::Gote,self_occupied,self_ou_bitboard,from,kind
			)
		} else {
			None
		}
	}


	pub fn win_only_move_sente_kyou_with_point_and_kind_and_bitboard(
		self_occupied:BitBoard,self_ou_bitboard:BitBoard,bitboard:BitBoard,from:u32
	) -> Option<Square> {
		let self_ou_bitboard = unsafe {
			self_ou_bitboard.merged_bitboard
		};

		let count = Rule::calc_forward_move_repeat_count(bitboard,from);

		if count > 0 {
			let p = from - count;

			if self_ou_bitboard & (1 << p) != 0 {
				return Some(p as Square);
			}
		}

		None
	}

	pub fn win_only_move_gote_kyou_with_point_and_kind_and_bitboard(
		self_occupied:BitBoard,self_ou_bitboard:BitBoard,bitboard:BitBoard,from:u32
	) -> Option<Square> {
		let self_ou_bitboard = unsafe {
			self_ou_bitboard.merged_bitboard
		};

		let count = Rule::calc_back_move_repeat_count(bitboard,from);

		if count > 0 {
			let p = from + count;

			if self_ou_bitboard & (1 << (80 - p)) != 0 {
				return Some(p as Square);
			}
		}

		None
	}

	pub fn win_only_moves_with_point_and_kind(
		t:&Teban,state:&State,x:u32,y:u32,kind:KomaKind
	) -> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();
		Rule::win_only_moves_with_point_and_kind_and_buffer(t, state, x, y, kind, &mut mvs);
		mvs
	}

	pub fn win_only_moves_with_point_and_kind_and_buffer(
		t:&Teban,state:&State,x:u32,y:u32,kind:KomaKind,mvs:&mut Vec<LegalMove>
	) {
		let from = x * 9 + y;

		let kinds = match &state.banmen {
			&Banmen(ref kinds) => kinds
		};

		match kind {
			SFu => {
				if let Some(p) = Rule::win_only_move_once_with_point_and_kind_and_bitboard(
					*t,state.sente_self_board,state.sente_ou_position_board,from,kind
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let o = Some(ObtainKind::Ou);

					if SENTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					if DENY_MOVE_SENTE_FU_AND_KYOU_MASK & to_mask == 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
					}
				}
			},
			SKyou => {
				let bitboard = state.sente_self_board | state.sente_opponent_board;

				if let Some(p) = Rule::win_only_move_sente_kyou_with_point_and_kind_and_bitboard(
					state.sente_self_board, state.sente_ou_position_board, bitboard, from
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let o = Some(ObtainKind::Ou);

					if SENTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					if DENY_MOVE_SENTE_FU_AND_KYOU_MASK & to_mask == 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
					}
				}
			}
			SKei => {
				if let Some(p) = Rule::win_only_move_once_with_point_and_kind_and_bitboard(
					*t,state.sente_self_board,state.sente_ou_position_board,from,kind
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let o = Some(ObtainKind::Ou);

					if SENTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					if DENY_MOVE_SENTE_KEI_MASK & to_mask == 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
					}
				}
			},
			SKaku | SKakuN => {
				if let Some(p) = Rule::win_only_move_sente_kaku_with_point_and_kind_and_bitboard(
					state.sente_self_board, state.sente_ou_position_board, state.diag_board, from, kind
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let o = Some(ObtainKind::Ou);

					if kind == SKaku && SENTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}
					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			SHisha | SHishaN => {
				let bitboard = state.sente_self_board | state.sente_opponent_board;

				if let Some(p) = Rule::win_only_move_sente_hisha_with_point_and_kind_and_bitboard(
					state.sente_self_board, state.sente_ou_position_board, bitboard, state.rotate_board, from, kind
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let o = Some(ObtainKind::Ou);

					if kind == SHisha && SENTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}
					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			SGin | SOu =>  {
				if let Some(p) = Rule::win_only_move_once_with_point_and_kind_and_bitboard(
					*t,state.sente_self_board,state.sente_ou_position_board,from,kind
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let o = Some(ObtainKind::Ou);

					if SENTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			SFuN | SKyouN | SKeiN | SGinN | SKin => {
				if let Some(p) = Rule::win_only_move_once_with_point_and_kind_and_bitboard(
					*t,state.sente_self_board,state.sente_ou_position_board,from,kind
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let o = Some(ObtainKind::Ou);

					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			GFu => {
				if let Some(p) = Rule::win_only_move_once_with_point_and_kind_and_bitboard(
					*t,state.gote_self_board,state.gote_ou_position_board,from,kind
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let o = Some(ObtainKind::Ou);

					if GOTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					if DENY_MOVE_GOTE_FU_AND_KYOU_MASK & to_mask == 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
					}
				}
			},
			GKyou => {
				let bitboard = state.sente_self_board | state.sente_opponent_board;

				if let Some(p) = Rule::win_only_move_gote_kyou_with_point_and_kind_and_bitboard(
					state.gote_self_board, state.gote_ou_position_board, bitboard, from
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let o = Some(ObtainKind::Ou);

					if GOTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					if DENY_MOVE_GOTE_FU_AND_KYOU_MASK & to_mask == 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
					}
				}
			},
			GKei => {
				if let Some(p) = Rule::win_only_move_once_with_point_and_kind_and_bitboard(
					*t,state.gote_self_board,state.gote_ou_position_board,from,kind
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let o = Some(ObtainKind::Ou);

					if GOTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					if DENY_MOVE_GOTE_KEI_MASK & to_mask == 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
					}
				}
			},
			GKaku | GKakuN => {
				if let Some(p) = Rule::win_only_move_gote_kaku_with_point_and_kind_and_bitboard(
					state.gote_self_board, state.gote_ou_position_board, state.diag_board, from, kind
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_board = unsafe { state.sente_self_board.merged_bitboard };

					let o = if opponent_board & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					if kind == GKaku && GOTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}
					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			GHisha | GHishaN => {
				let bitboard = state.sente_self_board | state.sente_opponent_board;

				if let Some(p) = Rule::win_only_move_gote_hisha_with_point_and_kind_and_bitboard(
					state.gote_self_board, state.gote_ou_position_board, bitboard, state.rotate_board, from, kind
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let o = Some(ObtainKind::Ou);

					if kind == GHisha && GOTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}

					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			GGin | GOu =>  {
				if let Some(p) = Rule::win_only_move_once_with_point_and_kind_and_bitboard(
					*t,state.gote_self_board,state.gote_ou_position_board,from,kind
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let o = Some(ObtainKind::Ou);

					if GOTE_NARI_MASK & to_mask != 0 {
						mvs.push(LegalMove::To(LegalMoveTo::new(from, to, true, o)));
					}
					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			GFuN | GKyouN | GKeiN | GGinN | GKin => {
				if let Some(p) = Rule::win_only_move_once_with_point_and_kind_and_bitboard(
					*t,state.gote_self_board,state.gote_ou_position_board,from,kind
				) {
					let to = p as u32;

					let to_mask = if to > 0 {
						1 << to
					} else {
						1
					};

					let opponent_board = unsafe { state.sente_self_board.merged_bitboard };

					let o = if opponent_board & to_mask != 0 {
						match ObtainKind::try_from(kinds[y as usize][x as usize]) {
							Ok(obtained) => Some(obtained),
							Err(_) => None,
						}
					} else {
						None
					};

					mvs.push(LegalMove::To(LegalMoveTo::new(from, to, false, o)));
				}
			},
			Blank => (),
		}
	}

	pub fn win_only_moves_with_point(t:&Teban,state:&State,x:u32,y:u32)
		-> Vec<LegalMove> {
		match &state.banmen {
			&Banmen(ref kinds) => {
				Rule::win_only_moves_with_point_and_kind(t,state,x,y,kinds[y as usize][x as usize])
			}
		}
	}

	pub fn win_only_moves_with_src(t:&Teban,state:&State,src:KomaSrcPosition)
		-> Vec<LegalMove> {
		match src {
			KomaSrcPosition(x,y) => Rule::win_only_moves_with_point(t,state, 9 - x, y - 1)
		}
	}

	pub fn win_only_moves_with_dst_to(t:&Teban,state:&State,dst:KomaDstToPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstToPosition(x,y,_) => Rule::win_only_moves_with_point(t, state, 9 - x, y - 1)
		}
	}

	pub fn win_only_moves_with_dst_put(t:&Teban,state:&State,dst:KomaDstPutPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstPutPosition(x,y) => Rule::win_only_moves_with_point(t, state, 9 - x, y - 1)
		}
	}

	pub fn win_only_moves(t:&Teban,state:&State)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		match &state.banmen {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match *t {
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


	pub fn oute_only_moves_with_point(t:&Teban,banmen:&Banmen,mc:&MochigomaCollections,x:u32,y:u32)
		-> Vec<LegalMove> {
		Rule::legal_moves_with_point(t, banmen, x, y)
			.into_iter().filter(|m| {
					match m {
						&LegalMove::To(_,_,Some(ObtainKind::Ou)) => true,
						&LegalMove::To(ref s,ref d,_) => {
							match Rule::apply_move_none_check(banmen, t, mc,&Move::To(*s,*d)) {
								(ref b,_,_) => Rule::win_only_moves(t,b).len() > 0
							}
						},
						_ => false,
					}
			}).collect::<Vec<LegalMove>>()
	}

	pub fn oute_only_moves_from_banmen(t:&Teban,banmen:&Banmen,mc:&MochigomaCollections)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		match banmen {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match *t {
							Teban::Sente => (x,y),
							Teban::Gote => (8 - x, 8- y),
						};
						mvs.append(&mut Rule::oute_only_moves_with_point(t, banmen, mc, x as u32, y as u32));
					}
				}
			}
		}
		mvs
	}

	pub fn oute_only_moves_from_mochigoma(t:&Teban,mc:&MochigomaCollections,b:&Banmen) -> Vec<LegalMove> {
		Rule::legal_moves_from_mochigoma(t, mc, b)
			.into_iter().filter(|m| {
				match m {
					&LegalMove::Put(k,KomaDstPutPosition(x,y)) => {
						Rule::win_only_moves_with_point_and_kind(t, b, 9 - x, y - 1, KomaKind::from((*t,k))).len() > 0
					},
					_ => false,
				}
			}).collect::<Vec<LegalMove>>()
	}

	pub fn oute_only_moves_all(t:&Teban,banmen:&Banmen,mc:&MochigomaCollections)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		match banmen {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match *t {
							Teban::Sente => (x,y),
							Teban::Gote => (8 - x, 8- y),
						};
						mvs.append(&mut Rule::oute_only_moves_with_point(t, banmen, mc, x as u32, y as u32));
					}
				}
			}
		}
		mvs.append(&mut Rule::oute_only_moves_from_mochigoma(t, mc, banmen));
		mvs
	}

	pub fn respond_oute_only_moves_all(t:&Teban,banmen:&Banmen,mc:&MochigomaCollections)
		-> Vec<LegalMove> {
		Rule::legal_moves_all(t, banmen, mc)
			.into_iter().filter(|m| {
					match m {
						&LegalMove::To(_,_,Some(ObtainKind::Ou)) => true,
						&LegalMove::To(ref s,ref d,_) => {
							match Rule::apply_move_none_check(banmen,t,mc,&Move::To(*s,*d)) {
								(ref b,_,_) => Rule::win_only_moves(&t.opposite(),b).len() == 0
							}
						},
						_ => false,
					}
			}).collect::<Vec<LegalMove>>()
	}

	pub fn apply_move_none_check(banmen:&Banmen,t:&Teban,mc:&MochigomaCollections,m:&Move)
		-> (Banmen,MochigomaCollections,Option<MochigomaKind>) {

		let mut kinds = match banmen {
			&Banmen(ref kinds) => kinds.clone(),
		};

		let (nmc,obtained) = match m {
			&Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,n)) => {
				let k = kinds[(sy - 1) as usize][(9 - sx) as usize];

				kinds[(sy - 1) as usize][(9 - sx) as usize] = KomaKind::Blank;

				match kinds[(dy - 1) as usize][(9 - dx) as usize] {
					KomaKind::Blank => {
						kinds[(dy - 1) as usize][(9 - dx) as usize] = match n {
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

						kinds[(dy - 1) as usize][(9 - dx) as usize] = match n {
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
										match *t {
											Teban::Sente => {
												let mut ms = ms.clone();

												let count = match ms.get(&obtained) {
													Some(count) => count+1,
													None => 1,
												};

												ms.insert(obtained,count);

												(MochigomaCollections::Pair(ms,mg.clone()),Some(obtained))
											},
											Teban::Gote => {
												let mut mg = mg.clone();

												let count = match mg.get(&obtained) {
													Some(count) => count+1,
													None => 1,
												};

												mg.insert(obtained,count);

												(MochigomaCollections::Pair(ms.clone(),mg),Some(obtained))
											}
										}
									},
									&MochigomaCollections::Empty => {
										match *t {
											Teban::Sente => {
												let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

												ms.insert(obtained,1);
												(MochigomaCollections::Pair(ms,HashMap::new()),Some(obtained))
											},
											Teban::Gote => {
												let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();
												mg.insert(obtained,1);
												(MochigomaCollections::Pair(HashMap::new(),mg),Some(obtained))
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
			&Move::Put(k,KomaDstPutPosition(dx,dy)) => {
				kinds[(dy - 1) as usize][(9 - dx) as usize] = KomaKind::from((*t,k));

				let mut mc = mc.clone();

				match t {
					&Teban::Sente => {
						match mc {
							MochigomaCollections::Pair(ref mut mc,_) => {
								let c = match mc.get(&k) {
									Some(c) => {
										c-1
									},
									None => 0,
								};
								mc.insert(k,c);
							},
							_ => (),
						}
					},
					&Teban::Gote => {
						match mc {
							MochigomaCollections::Pair(_,ref mut mc) => {
								let c = match mc.get(&k) {
									Some(c) => {
										c-1
									},
									None => 0
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

	pub fn apply_valid_move(banmen:&Banmen,t:&Teban,mc:&MochigomaCollections,m:&Move)
		-> Result<(Banmen,MochigomaCollections,Option<MochigomaKind>),ShogiError> {

		match m {
			&Move::To(s,d) => {
				let mvs = Rule::legal_moves_from_banmen(t,banmen);

				match mvs.find(&(s,d)) {
					Some(_) => {
						Ok(Rule::apply_move_none_check(banmen,t,mc,m))
					},
					None => {
						Err(ShogiError::InvalidState(String::from(
							"This is not legal move."
						)))
					}
				}
			},
			&Move::Put(k,d) => {
				let mvs = Rule::legal_moves_from_mochigoma(t,mc,banmen);

				match mvs.find(&(k,d)) {
					Some(_) => {
						Ok(Rule::apply_move_none_check(banmen,t,mc,m))
					},
					None => {
						Err(ShogiError::InvalidState(String::from(
							"This is not legal move."
						)))
					}
				}
			}
		}
	}

	pub fn apply_moves(banmen:&Banmen,mut teban:Teban,
						mut mc:MochigomaCollections,
						m:&Vec<Move>,mut mhash:u64,mut shash:u64,
						mut kyokumen_hash_map:TwoKeyHashMap<u64,u32>,
						hasher:&KyokumenHash<u64>)
		-> (Teban,Banmen,MochigomaCollections,u64,u64,TwoKeyHashMap<u64,u32>) {

		let mut banmen = banmen.clone();

		for m in m {
			match Rule::apply_move_none_check(&banmen,&teban,&mc,&m) {
				(next,nmc,o) => {
					mhash = hasher.calc_main_hash(mhash,&teban,&banmen,&mc,m,&o);
					shash = hasher.calc_sub_hash(shash,&teban,&banmen,&mc,m,&o);

					mc = nmc;
					teban = teban.opposite();
					banmen = next;

					match kyokumen_hash_map.get(&mhash,&shash) {
						Some(c) => {
							kyokumen_hash_map.insert(mhash,shash,c+1);
						},
						None => {
							kyokumen_hash_map.insert(mhash,shash,1);
						}
					}
				}
			}
		}

		(teban,banmen,mc,mhash,shash,kyokumen_hash_map)
	}


	pub fn apply_moves_with_callback<T,F>(
						banmen:&Banmen,
						mut teban:Teban,
						mut mc:MochigomaCollections,
						m:&Vec<Move>,mut r:T,mut f:F)
		-> (Teban,Banmen,MochigomaCollections,T)
		where F: FnMut(&Banmen,&Teban,
						&MochigomaCollections,&Option<Move>,
						&Option<MochigomaKind>,T) -> T {

		let mut banmen = banmen.clone();

		for m in m {
			match Rule::apply_move_none_check(&banmen,&teban,&mc,m) {
				(next,nmc,o) => {
					r = f(&banmen,&teban,&mc,&Some(*m),&o,r);
					banmen = next;
					mc = nmc;
					teban = teban.opposite();
				}
			}
		}

		r = f(&banmen,&teban,&mc,&None,&None,r);

		(teban,banmen,mc,r)
	}

	pub fn is_nyugyoku_win(banmen:&Banmen,t:&Teban,mc:&MochigomaCollections,limit:&Option<Instant>) -> bool {
		if Rule::win_only_moves(&t.opposite(),banmen).len() > 0 {
			return false
		}

		if let &Some(limit) = limit {
			if limit > Instant::now() {
				return false;
			}
		}

		let ou = match *t {
			Teban::Sente => KomaKind::SOu,
			Teban::Gote => KomaKind::GOu,
		};

		let oy = match banmen.find(&ou) {
			Some(ref v) if v.len() > 0 => {
				match v[0] {
					KomaPosition(_,oy) => {
						(oy - 1) as usize
					}
				}
			},
			_ => {
				return false;
			}
		};

		match banmen {
			&Banmen(ref kinds) => {
				match *t {
					Teban::Sente => {
						match mc {
							&MochigomaCollections::Pair(ref mc, _) => {
								oy <= 2 && kinds.iter().enumerate().map(|(y,row)| {
									if y <  3 {
										row.iter().map(|k| {
											match *k {
												KomaKind::SHisha | KomaKind::SHishaN |
												KomaKind::SKaku | KomaKind::SKakuN => {
													5
												},
												KomaKind::SOu => {
													0
												},
												k if k < KomaKind::GFu => {
													1
												},
												_ => {
													0
												}
											}
										}).fold(0, |sum,s| sum + s)
									} else {
										0
									}
								}).fold(0, |sum,s| sum + s) + (&MOCHIGOMA_KINDS).iter().map(|k| {
									match k {
										&MochigomaKind::Hisha | &MochigomaKind::Kaku => {
											mc.get(k).map_or(0, |n| *n * 5)
										},
										_ => {
											mc.get(k).map_or(0, |n| *n)
										}
									}
								}).fold(0, |sum,s| sum + s) >= 28 && kinds.iter().enumerate().map(|(y,row)| {
									if y < 3 {
										row.iter().map(|k| {
											match *k {
												KomaKind::SOu => false,
												k if k < KomaKind::GFu => true,
												_ => false,
											}
										}).count()
									} else {
										0
									}
								}).fold(0, |sum,s| sum + s) >= 10
							},
							&MochigomaCollections::Empty => {
								oy <= 2 && kinds.iter().enumerate().map(|(y,row)| {
									if y < 3 {
										row.iter().map(|k| {
											match *k {
												KomaKind::SHisha | KomaKind::SHishaN |
												KomaKind::SKaku | KomaKind::SKakuN => {
													5
												},
												KomaKind::SOu => {
													0
												},
												k if k < KomaKind::GFu => {
													1
												},
												_ => {
													0
												}
											}
										}).fold(0, |sum,s| sum + s)
									} else {
										0
									}
								}).fold(0, |sum,s| sum + s)  >= 28 && kinds.iter().enumerate().map(|(y,row)| {
									if y < 3 {
										row.iter().map(|k| {
											match *k {
												KomaKind::SOu => false,
												k if k < KomaKind::GFu => true,
												_ => false,
											}
										}).count()
									} else {
										0
									}
								}).fold(0, |sum,s| sum + s) >= 10
							}
						}
					},
					Teban::Gote => {
						match mc {
							&MochigomaCollections::Pair(_, ref mc) => {
								oy >= 6 && kinds.iter().enumerate().map(|(y,row)| {
									if y >= 6 {
										row.iter().map(|k| {
											match *k {
												KomaKind::GHisha | KomaKind::GHishaN |
												KomaKind::GKaku | KomaKind::GKakuN => {
													5
												},
												KomaKind::GOu | KomaKind::Blank=> {
													0
												},
												k if k >= KomaKind::GFu => {
													1
												},
												_ => {
													0
												}
											}
										}).fold(0, |sum,s| sum + s)
									} else {
										0
									}
								}).fold(0, |sum,s| sum + s) + (&MOCHIGOMA_KINDS).iter().map(|k| {
									match k {
										&MochigomaKind::Hisha | &MochigomaKind::Kaku => {
											mc.get(k).map_or(0, |n| *n * 5)
										},
										_ => {
											mc.get(k).map_or(0, |n| *n)
										}
									}
								}).fold(0, |sum,s| sum + s) >= 27 && kinds.iter().enumerate().map(|(y,row)| {
									if y >= 6 {
										row.iter().map(|k| {
											match *k {
												KomaKind::GOu | KomaKind::Blank => false,
												k if k >= KomaKind::GFu => true,
												_ => false,
											}
										}).count()
									} else {
										0
									}
								}).fold(0, |sum,s| sum + s) >= 10
							},
							&MochigomaCollections::Empty => {
								oy >= 6 && kinds.iter().enumerate().map(|(y,row)| {
									if y >= 6 {
										row.iter().map(|k| {
											match *k {
												KomaKind::GHisha | KomaKind::GHishaN |
												KomaKind::GKaku | KomaKind::GKakuN => {
													5
												},
												KomaKind::GOu | KomaKind::Blank=> {
													0
												},
												k if k >= KomaKind::GFu => {
													1
												},
												_ => {
													0
												}
											}
										}).count()
									} else {
										0
									}
								}).fold(0, |sum,s| sum + s) >= 27 && kinds.iter().enumerate().map(|(y,row)| {
									if y >= 6 {
										row.iter().map(|k| {
											match *k {
												KomaKind::GOu | KomaKind::Blank => false,
												k if k >= KomaKind::GFu => true,
												_ => false,
											}
										}).count()
									} else {
										0
									}
								}).count() >= 10
							}
						}
					}
				}
			}
		}
	}

	pub fn responded_oute(banmen:&Banmen,t:&Teban,mc:&MochigomaCollections,m:&Move,nm:&Move)
		-> Result<bool,SelfMatchRunningError> {

		let o = t.opposite();

		if !match m {
			&Move::To(_,ref dst) if Rule::win_only_moves_with_dst_to(&o, banmen, *dst).len() == 0 => false,
			&Move::Put(_,ref dst) if Rule::win_only_moves_with_dst_put(&o, banmen, *dst).len() == 0 => false,
			_ => true,
		} {
			return Err(SelfMatchRunningError::InvalidState(String::from(
				"The argument m is not Move of oute."
			)));
		}

		let (kind,x,y) = match m {
			&Move::To(_,KomaDstToPosition(dx,dy,_)) => {
				match banmen {
					&Banmen(ref kinds) => {
						let (dx,dy) = ((9 - dx) as usize,(dy - 1) as usize);
						(kinds[dy][dx],dx,dy)
					}
				}
			},
			&Move::Put(k,KomaDstPutPosition(dx,dy)) => {
				(KomaKind::from((*t,k)),(9 - dx) as usize, (dy - 1) as usize)
			}
		};

		let mvs = match kind {
			KomaKind::SKyou | KomaKind::GKyou |
			KomaKind::SHisha | KomaKind::GHisha |
			KomaKind::SHishaN | KomaKind::GHishaN |
			KomaKind::SKaku | KomaKind::GKaku |
			KomaKind::SKakuN | KomaKind::GKakuN => {
				Rule::legal_moves_all(t, banmen, mc).into_iter().filter(|m| {
					match m {
						&LegalMove::To(_,_,Some(ObtainKind::Ou)) => true,
						&LegalMove::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,_),_) => {
							let (sx,sy) = ((9 - sx) as usize, (sy - 1) as usize);
							let (dx,dy) = ((9 - dx) as usize, (dy - 1) as usize);

							let ou = match *t {
								Teban::Sente => KomaKind::SOu,
								Teban::Gote => KomaKind::GOu,
							};

							match banmen {
								&Banmen(ref kinds) => {
									if kinds[sy][sx] == ou {
										true
									} else {
										let (tx,ty) = match banmen.find(&ou) {
											Some(ref v) if v.len() > 0 => {
												match v[0] {
													KomaPosition(ox,oy) => {
														((9 - ox) as usize, (oy - 1) as usize)
													},
												}
											},
											_ => {
												return false;
											}
										};

										if dx == x && dy == y {
											true
										} else if tx - x == 0 && ty < y {
											dx == x && dy <= y && dy > ty
										} else if tx - x == 0 {
											dx == x && dy >= y && dy < ty
										} else if ty - y == 0 && tx < x {
											dy == y && dx <= x && dx > tx
										} else if ty - y == 0 {
											dy == y && dx >= x && dx < tx
										} else if tx < x && ty < y {
											(tx as i32 - dx as i32).abs() == (ty as i32 - dy as i32).abs() &&
													dx <= x && dx > tx &&
													dy <= y && dy > ty
										} else if tx > x && ty < y {
											(tx as i32 - dx as i32).abs() == (ty as i32 - dy as i32).abs() &&
													dx >= x && dx < tx &&
													dy <= y && dy < ty
										} else if tx < x && ty > y {
											(tx as i32 - dx as i32).abs() == (ty as i32 - dy as i32).abs() &&
													dx <= x && dx > tx &&
													dy >= y && dy < ty
										} else if tx > x && ty > y{
											(tx as i32 - dx as i32).abs() == (ty as i32 - dy as i32).abs() &&
													dx >= x && dx < tx &&
													dy >= y && dy < ty
										} else {
											false
										}
									}
								}
							}
						},
						&LegalMove::Put(_,KomaDstPutPosition(dx,dy)) => {
							let (dx,dy) = ((9 - dx) as usize, (dy - 1) as usize);
							let (dx,dy) = ((9 - dx) as usize, (dy - 1) as usize);

							let ou = match *t {
								Teban::Sente => KomaKind::SOu,
								Teban::Gote => KomaKind::GOu,
							};

							let (tx,ty) = match banmen.find(&ou) {
								Some(ref v) if v.len() > 0 => {
									match v[0] {
										KomaPosition(ox,oy) => {
											((9 - ox) as usize, (oy - 1) as usize)
										}
									}
								},
								_ => {
									return false;
								}
							};

							if tx - x == 0 && ty < y {
								dx == x && dy <= y && dy > ty
							} else if tx - x == 0 {
								dx == x && dy >= y && dy < ty
							} else if ty - y == 0 && tx < x {
								dy == y && dx <= x && dx > tx
							} else if ty - y == 0 {
								dy == y && dx >= x && dx < tx
							} else if tx < x && ty < y {
								(tx as i32 - dx as i32).abs() == (ty as i32 - dy as i32).abs() &&
										dx <= x && dx > tx &&
										dy <= y && dy > ty
							} else if tx > x && ty < y {
								(tx as i32 - dx as i32).abs() == (ty as i32 - dy as i32).abs() &&
										dx >= x && dx < tx &&
										dy <= y && dy < ty
							} else if tx < x && ty > y {
								(tx as i32 - dx as i32).abs() == (ty as i32 - dy as i32).abs() &&
										dx <= x && dx > tx &&
										dy >= y && dy < ty
							} else if tx > x && ty > y{
								(tx as i32 - dx as i32).abs() == (ty as i32 - dy as i32).abs() &&
										dx >= x && dx < tx &&
										dy >= y && dy < ty
							} else {
								false
							}
						}
					}
				}).collect::<Vec<LegalMove>>()
			},
			_ => {
				Rule::legal_moves_all(t, banmen, mc).into_iter().filter(|m| {
					match m {
						&LegalMove::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,_),_) => {
							let (dx,dy) = ((9 - dx) as usize, (dy - 1) as usize);
							let (sx,sy) = ((9 - sx) as usize, (sy - 1) as usize);

							let ou = match *t {
								Teban::Sente => KomaKind::SOu,
								Teban::Gote => KomaKind::GOu,
							};

							match banmen {
								&Banmen(ref kinds) => {
									kinds[sy][sx] == ou || (dx == x && dy == y)
								}
							}
						},
						_ => false
					}
				}).collect::<Vec<LegalMove>>()
			}
		};

		Ok(match nm {
			&Move::To(s,d) => {
				mvs.find(&(s,d)).is_some()
			},
			&Move::Put(k,d) => {
				mvs.find(&(k,d)).is_some()
			}
		})
	}

	pub fn is_put_fu_and_mate(banmen:&Banmen,teban:&Teban,mc:&MochigomaCollections,m:&Move) -> bool {
		match *m {
			Move::Put(MochigomaKind::Fu,KomaDstPutPosition(dx,dy)) => {
				let dx = 9 - dx;
				let dy = dy - 1;

				let ou = match teban {
					&Teban::Sente => KomaKind::GOu,
					&Teban::Gote => KomaKind::SOu,
				};

				let (ox,oy) = match banmen.find(&ou) {
					Some(ref v) if v.len() > 0 => {
						match v[0] {
							KomaPosition(ox,oy) => {
								((9 - ox) as i32, (oy - 1) as i32)
							}
						}
					},
					_ => {
						(-1,-1)
					}
				};

				let is_oute = match teban {
					&Teban::Sente if oy != -1 && ox != -1 => dy == (oy + 1) as u32 && ox as u32 == dx,
					&Teban::Gote if oy != -1 && ox != -1  => dy == (oy - 1) as u32 && ox as u32 == dx,
					_ => false,
				};

				is_oute && Rule::legal_moves_all(&teban, banmen, &mc).into_iter().filter(|m| {
					match m {
						&LegalMove::To(_,_,Some(ObtainKind::Ou)) => true,
						m @ _ => {
							match Rule::apply_move_none_check(banmen,&teban,&mc,&m.to_move()) {
								(ref b,_,_) => Rule::win_only_moves(&teban.opposite(),b).len() == 0
							}
						},
					}
				}).count() == 0
			},
			_ => false,
		}
	}

	pub fn is_win(banmen:&Banmen,teban:&Teban,m:&Move) -> bool {
		match m {
			&Move::To(_,KomaDstToPosition(dx,dy,_)) => {
				match banmen {
					&Banmen(ref kinds) => {
						match teban {
							&Teban::Sente => {
								kinds[dy as usize - 1][9 - dx as usize] == KomaKind::GOu
							},
							&Teban::Gote => {
								kinds[dy as usize - 1][9 - dx as usize] == KomaKind::SOu
							}
						}
					}
				}
			},
			_ => false,
		}
	}

	pub fn check_sennichite(_:&Banmen,mhash:u64,shash:u64,
									kyokumen_hash_map:&mut TwoKeyHashMap<u64,u32>) -> bool {
		match kyokumen_hash_map.get(&mhash,&shash) {
			Some(c) if c >= 3 => {
				return false;
			},
			Some(c) => {
				kyokumen_hash_map.insert(mhash,shash,c+1);
			},
			None => {
				kyokumen_hash_map.insert(mhash,shash,1);
			}
		}

		return true;
	}

	pub fn check_sennichite_by_oute(banmen:&Banmen,teban:&Teban,mhash:u64,shash:u64,
									oute_kyokumen_hash_map:&mut Option<TwoKeyHashMap<u64,u32>>)
		-> bool {

		match *oute_kyokumen_hash_map {
			None if Rule::win_only_moves(&teban,banmen).len() > 0 => {
				let mut m = TwoKeyHashMap::new();
				m.insert(mhash,shash,1);
				*oute_kyokumen_hash_map = Some(m);
			},
			Some(ref mut m) => {
				if Rule::win_only_moves(&teban,banmen).len() > 0 {
					if let Some(_) = m.get(&mhash,&shash) {
						return false;
					}

					m.insert(mhash,shash,1);
				}
			},
			ref mut m => {
				*m = None;
			}
		}

		true
	}

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

			if i == 0 {
				assert!(SENTE_NARI_MASK & 1 != 0);
			} else if x < 9 && y <= 2 {
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

			if i == 0 {
				assert!(GOTE_NARI_MASK & 1 == 0);
			} else if x < 9 && y >= 6 {
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

			if i == 0 {
				assert!(DENY_MOVE_SENTE_FU_AND_KYOU_MASK & 1 != 0);
			} else if x < 9 && y == 0 {
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

			if i == 0 {
				assert!(DENY_MOVE_GOTE_FU_AND_KYOU_MASK & 1 == 0);
			} else if x < 9 && y == 8 {
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

			if i == 0 {
				assert!(DENY_MOVE_SENTE_KEI_MASK & 1 != 0);
			} else if x < 9 && y <= 1 {
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

			if i == 0 {
				assert!(DENY_MOVE_GOTE_KEI_MASK & 1 == 0);
			} else if x < 9 && y >= 7 {
				assert!(DENY_MOVE_GOTE_KEI_MASK & 1 << i != 0);
			} else {
				assert!(DENY_MOVE_GOTE_KEI_MASK & 1 << i == 0);
			}
		}
	}
}