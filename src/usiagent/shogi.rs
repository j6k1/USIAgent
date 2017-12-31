use std::fmt;
use std::fmt::Formatter;
use usiagent::TryFrom;
use usiagent::error::*;
use usiagent::Validate;

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
	GKakuN,
	GHishaN,
	Blank,
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum KomaSrcPosition {
	Ban(u32,u32),
	Mochigoma(MochigomaKind),
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum KomaDstPosition {
	Ban(u32,u32),
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub struct Move(pub KomaSrcPosition, pub KomaDstPosition);
impl TryFrom<String> for KomaKind {
	fn try_from(s: String) -> Result<KomaKind, TypeConvertError<String>> {
		Ok(match &*s {
			"K" => KomaKind::SOu,
			"R" => KomaKind::SHisha,
			"B" => KomaKind::SKaku,
			"G" => KomaKind::SKin,
			"S" => KomaKind::SGin,
			"N" => KomaKind::SKei,
			"L" => KomaKind::SKyou,
			"P" => KomaKind::SFu,
			"+R" => KomaKind::SHishaN,
			"+B" => KomaKind::SKakuN,
			"+N" => KomaKind::SKeiN,
			"+L" => KomaKind::SKyouN,
			"+P" => KomaKind::SFuN,
			"k" => KomaKind::GOu,
			"r" => KomaKind::GHisha,
			"b" => KomaKind::GKaku,
			"g" => KomaKind::GKin,
			"s" => KomaKind::GGin,
			"n" => KomaKind::GKei,
			"l" => KomaKind::GKyou,
			"p" => KomaKind::GFu,
			"+r" => KomaKind::GHishaN,
			"+b" => KomaKind::GKakuN,
			"+n" => KomaKind::GKeiN,
			"+l" => KomaKind::GKyouN,
			"+p" => KomaKind::GFuN,
			_ => return Err(TypeConvertError::SyntaxError(s)),
		})
	}
}
pub struct Banmen([KomaKind; 81]);
impl fmt::Debug for Banmen {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Banmen(ref v) => write!(f, "Banmen[{}]", v.iter()
												.map(|k| format!("{:?}", k))
												.collect::<Vec<String>>().join(" "))
		}
	}
}
impl TryFrom<String> for Banmen {
	fn try_from(s: String) -> Result<Banmen, TypeConvertError<String>> {
		let mut chars = s.chars();

		let mut banmen:[KomaKind; 81] = [KomaKind::Blank; 81];

		let mut i = 0;
		let mut j = 0;

		while let Some(c) = chars.next() {
			let mut s = String::new();

			s.push(match j {
				j if j >= 9=> {
					return Err(TypeConvertError::SyntaxError(
							String::from("Invalid SFEN character string (pieces outside the range of the board)")));
				},
				_ => c,
			});

			match c {
				_ if i > 9 => {
					return Err(TypeConvertError::LogicError(
						String::from("Logic error of SFEN character string parsing process.")));
				},
				'/' => if i == 9 {
					j += 1; i = 0;
				},
				_ if i == 9 => {
					return Err(TypeConvertError::SyntaxError(
						String::from("Invalid SFEN string (line separator '/' not found)")));
				},
				'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' if i + ((c as u32) - ('0' as u32)) > 9 => {
					return Err(TypeConvertError::SyntaxError(
							String::from("Invalid SFEN character string (pieces outside the range of the board)")));
				},
				'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
					i += (c as u32) - ('0' as u32);
				},
				'+' => match chars.next() {
					None => {
						return Err(TypeConvertError::SyntaxError(
							String::from("Invalid SFEN character string (illegal expression of piece)")));
					},
					Some(n) => {
						s.push(n);
						banmen[j as usize * 9 + i as usize] = KomaKind::try_from(s)?;
						i += 1;
					}
				},
				_ =>  {
					banmen[j as usize * 9 + i as usize] = KomaKind::try_from(s)?;
					i += 1;
				}
			}
		}

		Ok(Banmen(banmen))
	}
}
impl Validate for KomaSrcPosition {
	fn validate(&self) -> bool {
		match *self {
			KomaSrcPosition::Mochigoma(_) => true,
			KomaSrcPosition::Ban(x, y) => x < 9 && y < 9,
		}
	}
}
impl Validate for KomaDstPosition {
	fn validate(&self) -> bool {
		match *self {
			KomaDstPosition::Ban(x, y) => x < 9 && y < 9,
		}
	}
}
impl Validate for Move {
	fn validate(&self) -> bool {
		match *self {
			Move(ref s, ref d) => s.validate() && d.validate()
		}
	}
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum MochigomaKind {
	Fu = 0,
	Kyou,
	Kei,
	Gin,
	Kin,
	Kaku,
	Hisha,
	Ou,
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum Kyokumen {
	Ban(Teban,KomaKind,u32,u32),
	MochigomaKind(Teban,MochigomaKind),
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