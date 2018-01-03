use std::fmt;
use std::fmt::Formatter;
use usiagent::TryFrom;
use usiagent::error::*;
use usiagent::Validate;
use self::KomaKind::{SFu,SKyou,SKei,SGin,SKin,SKaku,SHisha,SOu,GFu,GKyou,GKei,GGin,GKin,GKaku,GHisha,GOu,Blank};

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
impl<'a> TryFrom<&'a str,String> for KomaSrcPosition {
	fn try_from(s: &'a str) -> Result<KomaSrcPosition, TypeConvertError<String>> {
		let mut chars = s.chars();

		Ok(match chars.next() {
			Some(c) => match c {
				'R' | 'B' | 'G' | 'S' | 'N' | 'L' | 'P' => {
					match chars.next() {
						Some('*') => (),
						None | Some(_) => {
							return Err(TypeConvertError::SyntaxError(
								String::from("Invalid SFEN string (there no '*' after the name)")));
						}
					}
					match c {
						'R' => KomaSrcPosition::Mochigoma(MochigomaKind::Hisha),
						'B' => KomaSrcPosition::Mochigoma(MochigomaKind::Kaku),
						'G' => KomaSrcPosition::Mochigoma(MochigomaKind::Kin),
						'S' => KomaSrcPosition::Mochigoma(MochigomaKind::Gin),
						'N' => KomaSrcPosition::Mochigoma(MochigomaKind::Kei),
						'L' => KomaSrcPosition::Mochigoma(MochigomaKind::Kyou),
						'P' => KomaSrcPosition::Mochigoma(MochigomaKind::Fu),
						_ => {
							return Err(TypeConvertError::LogicError(String::from(
								"Logic error in the move analysis phase of the SFEN string analysis process."
							)));
						}
					}
				},
				'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
					let x = c as u32 - 1;

					match chars.next() {
						Some(c) => {
							let y = match c {
								'a' => 0,
								'b' => 1,
								'c' => 2,
								'd' => 3,
								'e' => 4,
								'f' => 5,
								'g' => 6,
								'h' => 7,
								'i' => 8,
								_ => {
									return Err(TypeConvertError::SyntaxError(String::from(
										"Invalid SFEN character string (The format of the move is illegal)"
									)));
								}
							};
							KomaSrcPosition::Ban(x,y)
						},
						None => {
							return Err(TypeConvertError::SyntaxError(
								String::from(
									"Invalid SFEN character string (The format of the move is illegal)"
							)));
						}
					}
				},
				_ => {
					return Err(TypeConvertError::SyntaxError(
							String::from(
								"Invalid SFEN character string (The format of the move is illegal)"
							)));
				}
			},
			None => {
				return Err(TypeConvertError::SyntaxError(
							String::from(
								"Invalid SFEN character string (The format of the hand is illegal)"
							)));
			}
		})
	}
}
impl<'a> TryFrom<&'a str, String> for KomaDstPosition {
	fn try_from(s: &'a str) -> Result<KomaDstPosition, TypeConvertError<String>> {
		let mut chars = s.chars();

		Ok(match chars.next() {
			Some(c) => match c {
				'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
					let x = c as u32 - 1;

					match chars.next() {
						Some(c) => {
							let y = match c {
								'a' => 0,
								'b' => 1,
								'c' => 2,
								'd' => 3,
								'e' => 4,
								'f' => 5,
								'g' => 6,
								'h' => 7,
								'i' => 8,
								_ => {
									return Err(TypeConvertError::SyntaxError(
										String::from(
											"Invalid SFEN character string (The format of the move is illegal)"
									)));
								}
							};
							KomaDstPosition::Ban(x,y)
						},
						None => {
							return Err(TypeConvertError::SyntaxError(
								String::from(
									"Invalid SFEN character string (The format of the move is illegal)"
							)));
						}
					}
				},
				_ => {
					return Err(TypeConvertError::SyntaxError(
							String::from(
								"Invalid SFEN character string (The format of the move is illegal)"
							)));
				}
			},
			None => {
				return Err(TypeConvertError::SyntaxError(
							String::from(
								"Invalid SFEN character string (The format of the hand is illegal)"
							)));
			}
		})
	}
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub struct Move(pub KomaSrcPosition, pub KomaDstPosition);
impl<'a> TryFrom<&'a str,String> for Move {
	fn try_from(s: &'a str) -> Result<Move, TypeConvertError<String>> {
		Ok(match s {
			ref s if s.len() != 4 => {
				return Err(TypeConvertError::SyntaxError(String::from(
					"Invalid SFEN character string (number of characters of move expression is invalid)")));
			},
			ref s => Move(KomaSrcPosition::try_from(&s[0..1])?,
						KomaDstPosition::try_from(&s[2..3])?)
		})
	}
}
impl TryFrom<String,String> for KomaKind {
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
pub struct Banmen(pub [KomaKind; 81]);
impl fmt::Debug for Banmen {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Banmen(ref v) => write!(f, "Banmen[{}]", v.iter()
												.map(|k| format!("{:?}", k))
												.collect::<Vec<String>>().join(" "))
		}
	}
}
/// 右上を(0,0)とした位置
pub const BANMEN_START_POS:[KomaKind; 81] = [
	GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou,
	Blank,GKaku,Blank,Blank,Blank,Blank,Blank,GHisha,Blank,
	GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,
	Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,
	Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,
	Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,
	SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,
	Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank,
	SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou,
];
impl<'a> TryFrom<&'a str,String> for Banmen {
	fn try_from(s: &'a str) -> Result<Banmen, TypeConvertError<String>> {
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
impl<'a> TryFrom<&'a str,String> for Teban {
	fn try_from(s: &'a str) -> Result<Teban, TypeConvertError<String>> {
		Ok(match s {
			"b" => Teban::Sente,
			"w" => Teban::Gote,
			_ => {
				return Err(TypeConvertError::SyntaxError(String::from(
					"It is an illegal character string as a character string representing a turn."
				)));
			}
		})
	}
}