use std::fmt;
use std::fmt::Formatter;
use std::collections::HashMap;

use TryFrom;
use rule::AppliedMove;
use error::*;

use Find;
use MaxIndex;

#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum KomaKind {
	SFu = 0,
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
	Blank,
}
impl KomaKind {
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
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum Teban {
	Sente,
	Gote,
}
impl Teban {
	pub fn opposite(&self) -> Teban {
		match *self {
			Teban::Sente => Teban::Gote,
			Teban::Gote => Teban::Sente,
		}
	}
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub struct KomaPosition(pub u32,pub u32);
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub struct KomaSrcPosition(pub u32,pub u32);
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub struct KomaDstToPosition(pub u32,pub u32,pub bool);
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub struct KomaDstPutPosition(pub u32,pub u32);
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum Move {
	To(KomaSrcPosition,KomaDstToPosition),
	Put(MochigomaKind,KomaDstPutPosition),
}
impl Move {
	pub fn to_applied_move(&self) -> AppliedMove {
		AppliedMove::from(*self)
	}
}
#[derive(Debug)]
pub enum MochigomaCollections {
	Empty,
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
impl MochigomaCollections {
	pub fn new(ms:HashMap<MochigomaKind,u32>,mg:HashMap<MochigomaKind,u32>) -> MochigomaCollections {
		if ms.len() == 0 && mg.len() == 0 {
			MochigomaCollections::Empty
		} else {
			MochigomaCollections::Pair(ms,mg)
		}
	}
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum ObtainKind {
	Fu = 0,
	Kyou,
	Kei,
	Gin,
	Kin,
	Kaku,
	Hisha,
	Ou,
	FuN,
	KyouN,
	KeiN,
	GinN,
	KakuN,
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
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug, Hash)]
pub enum MochigomaKind {
	Fu = 0,
	Kyou,
	Kei,
	Gin,
	Kin,
	Kaku,
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
