use std::fmt;
use std::fmt::Formatter;
use std::collections::HashMap;

use TryFrom;
use rule::AppliedMove;
use error::*;

use Find;
use MaxIndex;
/// 盤面上の駒の種別
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum KomaKind {
	/// 先手歩
	SFu = 0,
	/// 先手香
	SKyou,
	/// 先手桂
	SKei,
	/// 先手銀
	SGin,
	/// 先手金
	SKin,
	/// 先手角
	SKaku,
	/// 先手飛車
	SHisha,
	/// 王
	SOu,
	/// 先手と金
	SFuN,
	/// 先手成り香
	SKyouN,
	/// 先手成り桂
	SKeiN,
	/// 先手成銀
	SGinN,
	/// 先手馬
	SKakuN,
	/// 先手龍
	SHishaN,
	/// 後手歩
	GFu,
	/// 後手香
	GKyou,
	/// 後手桂
	GKei,
	/// 後手銀
	GGin,
	/// 後手金
	GKin,
	/// 後手角
	GKaku,
	/// 後手飛車
	GHisha,
	/// 玉
	GOu,
	/// 後手と金
	GFuN,
	/// 後手成り香
	GKyouN,
	/// 後手成り桂
	GKeiN,
	/// 後手成銀
	GGinN,
	/// 後手馬
	GKakuN,
	/// 後手龍
	GHishaN,
	/// 駒無し
	Blank,
}
impl KomaKind {
	/// 成駒のKomaKindを取得
	pub fn to_nari(&self) -> KomaKind {
		match *self {
			KomaKind::SFu => {
				KomaKind::SFuN
			},
			KomaKind::SKyou => {
				KomaKind::SKyouN
			},
			KomaKind::SKei => {
				KomaKind::SKeiN
			},
			KomaKind::SGin => {
				KomaKind::SGinN
			},
			KomaKind::SHisha => {
				KomaKind::SHishaN
			},
			KomaKind::SKaku => {
				KomaKind::SKakuN
			},
			KomaKind::GFu => {
				KomaKind::GFuN
			},
			KomaKind::GKyou => {
				KomaKind::GKyouN
			},
			KomaKind::GKei => {
				KomaKind::GKeiN
			},
			KomaKind::GGin => {
				KomaKind::GGinN
			},
			KomaKind::GHisha => {
				KomaKind::GHishaN
			},
			KomaKind::GKaku => {
				KomaKind::GKakuN
			},
			kind => kind
		}
	}
	/// 駒が成っているか否か
	pub fn is_nari(&self) -> bool {
		match *self {
			KomaKind::SFuN | KomaKind::SKyouN | KomaKind::SKeiN | KomaKind::SGinN | KomaKind::SHishaN | KomaKind::SKakuN |
			KomaKind::GFuN | KomaKind::GKyouN | KomaKind::GKeiN | KomaKind::GGinN | KomaKind::GHishaN | KomaKind::GKakuN => {
				true
			},
			_ => false
		}
	}
}
impl MaxIndex for KomaKind {
	fn max_index() -> usize {
		KomaKind::Blank as usize
	}
}
#[derive(PartialEq, Eq)]
pub struct Banmen(pub [[KomaKind; 9]; 9]);
impl Clone for Banmen {
	fn clone(&self) -> Banmen {
		match self {
			&Banmen(ref kinds) => Banmen(kinds.clone())
		}
	}
}
impl Banmen {
}
impl fmt::Debug for Banmen {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Banmen(ref v) => write!(f, "Banmen[\n{}\n]",
									v.iter()
									.map(|&row| {
										format!("  [{}]", row.iter().map(|&k| format!("{:?}", k)).collect::<Vec<String>>().join(", "))
									})
									.collect::<Vec<String>>().join("\n"))
		}
	}
}
impl Find<KomaKind,Vec<KomaPosition>> for Banmen {
	fn find(&self,query:&KomaKind) -> Option<Vec<KomaPosition>> {
		let mut r:Vec<KomaPosition> = Vec::new();

		match self {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						if *query == kinds[y][x] {
							r.push(KomaPosition(9 - x as u32, y as u32 + 1));
						}
					}
				}
			}
		}

		if r.len() > 0 {
			Some(r)
		} else {
			None
		}
	}
}
/// 手番
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum Teban {
	/// 先手
	Sente,
	/// 後手
	Gote,
}
impl Teban {
	/// 相手の手番を取得
	pub fn opposite(&self) -> Teban {
		match *self {
			Teban::Sente => Teban::Gote,
			Teban::Gote => Teban::Sente,
		}
	}
}
/// 駒の位置
/// `KomaPosition(x,y)`,`x`は右側から1originのインデックス、`y`は上側から1originのインデックス
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub struct KomaPosition(pub u32,pub u32);
/// 駒の位置
/// `KomaSrcPosition(x,y)`,`x`は右側から1originのインデックス、`y`は上側から1originのインデックス
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub struct KomaSrcPosition(pub u32,pub u32);
/// 駒の位置
/// `KomaDstToPosition(x,y,b)`,`x`は右側から1originのインデックス、`y`は上側から1originのインデックス`b`は成るか否か
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub struct KomaDstToPosition(pub u32,pub u32,pub bool);
/// 駒の位置
/// `KomaDstPutPosition(x,y)`,`x`は右側から1originのインデックス、`y`は上側から1originのインデックス
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub struct KomaDstPutPosition(pub u32,pub u32);
/// 指し手
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum Move {
	/// 盤面上の駒を移動
	To(KomaSrcPosition,KomaDstToPosition),
	/// 持ち駒を置く
	Put(MochigomaKind,KomaDstPutPosition),
}
impl Move {
	/// 指し手を`AppliedMove`に変換(フレームワーク側で実装している手の適用処理に渡すために型変換が必要)
	pub fn to_applied_move(&self) -> AppliedMove {
		AppliedMove::from(*self)
	}
}
/// 持ち駒
#[derive(Debug,Eq)]
pub enum MochigomaCollections {
	/// 持ち駒が先手後手とも無し
	Empty,
	/// 先手後手それぞれの持ち駒を`HashMap<MochigomaKind,u32>`で表現
	Pair(HashMap<MochigomaKind,u32>,HashMap<MochigomaKind,u32>),
}
impl Clone for MochigomaCollections {
	fn clone(&self) -> MochigomaCollections {
		match *self {
			MochigomaCollections::Empty => MochigomaCollections::Empty,
			MochigomaCollections::Pair(ref ms, ref mg) => {
				MochigomaCollections::Pair(ms.clone(),mg.clone())
			}
		}
	}
}
impl PartialEq for MochigomaCollections {
	fn eq(&self, other: &Self) -> bool {
		match self {
			&MochigomaCollections::Empty => {
				match other {
					&MochigomaCollections::Empty => {
						true
					}
					&MochigomaCollections::Pair(ref ms,ref mg) => {
						ms.is_empty() && mg.is_empty()
					}
				}
			},
			&MochigomaCollections::Pair(ref ms, ref mg) => {
				match other {
					&MochigomaCollections::Empty => {
						ms.is_empty() && mg.is_empty()
					}
					&MochigomaCollections::Pair(ref oms,ref omg) => {
						ms == oms && mg == omg
					}
				}
			}
		}
	}
}
impl MochigomaCollections {
	/// MochigomaCollectionsを生成
	/// 先手と後手それぞれの持ち駒を`HashMap<MochigomaKind,u32>`で渡す
	pub fn new(ms:HashMap<MochigomaKind,u32>,mg:HashMap<MochigomaKind,u32>) -> MochigomaCollections {
		if ms.len() == 0 && mg.len() == 0 {
			MochigomaCollections::Empty
		} else {
			MochigomaCollections::Pair(ms,mg)
		}
	}

	/// 持ち駒は先手後手とも空か？
	pub fn is_empty(&self) -> bool {
		match self {
			&MochigomaCollections::Empty => true,
			&MochigomaCollections::Pair(ref ms, ref mg) => {
				(ms.is_empty() && mg.is_empty()) ||
				(ms.values().fold(0,|acc,&c| acc + c) == 0 && mg.values().fold(0,|acc,&c| acc + c) == 0)
			}
		}
	}
}
/// 獲った駒の種別
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum ObtainKind {
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
	/// 王または玉
	Ou,
	/// と金
	FuN,
	/// 成り香
	KyouN,
	/// 成り桂
	KeiN,
	/// 成銀
	GinN,
	/// 馬
	KakuN,
	/// 龍
	HishaN,
}
impl TryFrom<KomaKind,TypeConvertError<String>> for ObtainKind {
	fn try_from(kind:KomaKind) -> Result<ObtainKind,TypeConvertError<String>> {
		Ok(match kind {
			KomaKind::SFu => ObtainKind::Fu,
			KomaKind::SKyou => ObtainKind::Kyou,
			KomaKind::SKei => ObtainKind::Kei,
			KomaKind::SGin => ObtainKind::Gin,
			KomaKind::SKin => ObtainKind::Kin,
			KomaKind::SKaku => ObtainKind::Kaku,
			KomaKind::SHisha => ObtainKind::Hisha,
			KomaKind::SOu => ObtainKind::Ou,
			KomaKind::SFuN => ObtainKind::FuN,
			KomaKind::SKyouN => ObtainKind::KyouN,
			KomaKind::SKeiN => ObtainKind::KeiN,
			KomaKind::SGinN => ObtainKind::GinN,
			KomaKind::SKakuN => ObtainKind::KakuN,
			KomaKind::SHishaN => ObtainKind::HishaN,
			KomaKind::GFu => ObtainKind::Fu,
			KomaKind::GKyou => ObtainKind::Kyou,
			KomaKind::GKei => ObtainKind::Kei,
			KomaKind::GGin => ObtainKind::Gin,
			KomaKind::GKin => ObtainKind::Kin,
			KomaKind::GKaku => ObtainKind::Kaku,
			KomaKind::GHisha => ObtainKind::Hisha,
			KomaKind::GOu => ObtainKind::Ou,
			KomaKind::GFuN => ObtainKind::FuN,
			KomaKind::GKyouN => ObtainKind::KyouN,
			KomaKind::GKeiN => ObtainKind::KeiN,
			KomaKind::GGinN => ObtainKind::GinN,
			KomaKind::GKakuN => ObtainKind::KakuN,
			KomaKind::GHishaN => ObtainKind::HishaN,
			KomaKind::Blank => {
				return Err(TypeConvertError::LogicError(String::from("Can not  to convert Blank to ObtainKind.")));
			}
		})
	}
}
impl TryFrom<KomaKind,TypeConvertError<String>> for MochigomaKind {
	fn try_from(kind:KomaKind) -> Result<MochigomaKind,TypeConvertError<String>> {
		Ok(match kind {
			KomaKind::SFu => MochigomaKind::Fu,
			KomaKind::SKyou => MochigomaKind::Kyou,
			KomaKind::SKei => MochigomaKind::Kei,
			KomaKind::SGin => MochigomaKind::Gin,
			KomaKind::SKin => MochigomaKind::Kin,
			KomaKind::SKaku => MochigomaKind::Kaku,
			KomaKind::SHisha => MochigomaKind::Hisha,
			KomaKind::SFuN => MochigomaKind::Fu,
			KomaKind::SKyouN => MochigomaKind::Kyou,
			KomaKind::SKeiN => MochigomaKind::Kei,
			KomaKind::SGinN => MochigomaKind::Gin,
			KomaKind::SKakuN => MochigomaKind::Kaku,
			KomaKind::SHishaN => MochigomaKind::Hisha,
			KomaKind::GFu => MochigomaKind::Fu,
			KomaKind::GKyou => MochigomaKind::Kyou,
			KomaKind::GKei => MochigomaKind::Kei,
			KomaKind::GGin => MochigomaKind::Gin,
			KomaKind::GKin => MochigomaKind::Kin,
			KomaKind::GKaku => MochigomaKind::Kaku,
			KomaKind::GHisha => MochigomaKind::Hisha,
			KomaKind::GFuN => MochigomaKind::Fu,
			KomaKind::GKyouN => MochigomaKind::Kyou,
			KomaKind::GKeiN => MochigomaKind::Kei,
			KomaKind::GGinN => MochigomaKind::Gin,
			KomaKind::GKakuN => MochigomaKind::Kaku,
			KomaKind::GHishaN => MochigomaKind::Hisha,
			KomaKind::SOu | KomaKind::GOu => {
				return Err(TypeConvertError::LogicError(String::from("Can not  to convert SOu or GOu to MochigomaKind.")));
			},
			KomaKind::Blank => {
				return Err(TypeConvertError::LogicError(String::from("Can not  to convert Blank to MochigomaKind.")));
			}
		})
	}
}
// 持ち駒の種別
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug, Hash)]
pub enum MochigomaKind {
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
}
impl TryFrom<ObtainKind,TypeConvertError<String>> for MochigomaKind {
	fn try_from(o:ObtainKind) -> Result<MochigomaKind,TypeConvertError<String>> {
		Ok(match o {
			ObtainKind::Fu | ObtainKind::FuN => MochigomaKind::Fu,
			ObtainKind::Kyou | ObtainKind::KyouN=> MochigomaKind::Kyou,
			ObtainKind::Kei | ObtainKind::KeiN => MochigomaKind::Kei,
			ObtainKind::Gin | ObtainKind::GinN => MochigomaKind::Gin,
			ObtainKind::Kin => MochigomaKind::Kin,
			ObtainKind::Kaku | ObtainKind::KakuN => MochigomaKind::Kaku,
			ObtainKind::Hisha | ObtainKind::HishaN => MochigomaKind::Hisha,
			ObtainKind::Ou => {
				return Err(TypeConvertError::LogicError(String::from("Can not  to convert Ou to MochigomaKind.")));
			}
		})
	}
}
impl MaxIndex for MochigomaKind {
	fn max_index() -> usize {
		MochigomaKind::Hisha as usize
	}
}
/// 持ち駒の種別の配列
pub const MOCHIGOMA_KINDS:[MochigomaKind; 7] = [
	MochigomaKind::Fu,
	MochigomaKind::Kyou,
	MochigomaKind::Kei,
	MochigomaKind::Gin,
	MochigomaKind::Kin,
	MochigomaKind::Kaku,
	MochigomaKind::Hisha,
];
impl From<(Teban,MochigomaKind)> for KomaKind {
	fn from(tk:(Teban,MochigomaKind)) -> KomaKind {
		match tk {
			(Teban::Sente,k) => {
				match k {
					MochigomaKind::Fu => KomaKind::SFu,
					MochigomaKind::Kyou => KomaKind::SKyou,
					MochigomaKind::Kei => KomaKind::SKei,
					MochigomaKind::Gin => KomaKind::SGin,
					MochigomaKind::Kin => KomaKind::SKin,
					MochigomaKind::Kaku => KomaKind::SKaku,
					MochigomaKind::Hisha => KomaKind::SHisha,
				}
			},
			(Teban::Gote,k) => {
				match k {
					MochigomaKind::Fu => KomaKind::GFu,
					MochigomaKind::Kyou => KomaKind::GKyou,
					MochigomaKind::Kei => KomaKind::GKei,
					MochigomaKind::Gin => KomaKind::GGin,
					MochigomaKind::Kin => KomaKind::GKin,
					MochigomaKind::Kaku => KomaKind::GKaku,
					MochigomaKind::Hisha => KomaKind::GHisha,
				}
			}
		}
	}
}
