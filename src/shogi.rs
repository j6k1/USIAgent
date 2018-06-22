use std::fmt;
use std::fmt::Formatter;
use std::error::Error;
use std::collections::HashMap;
use std::time::{Instant,Duration};

use TryFrom;
use error::*;
use hash::*;
use command::*;

use Validate;
use Find;
use MaxIndex;
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
impl MaxIndex for KomaKind {
	fn max_index() -> usize {
		KomaKind::Blank as usize
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
impl<'a> TryFrom<&'a str,String> for Move {
	fn try_from(s: &'a str) -> Result<Move, TypeConvertError<String>> {
		match s {
			s if s.len() < 4 => {
				return Err(TypeConvertError::SyntaxError(String::from(
					"Invalid SFEN character string (number of characters of move expression is invalid)")));
			},
			_ => (),
		};

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
					let mochigoma = match c {
						'R' => MochigomaKind::Hisha,
						'B' => MochigomaKind::Kaku,
						'G' => MochigomaKind::Kin,
						'S' => MochigomaKind::Gin,
						'N' => MochigomaKind::Kei,
						'L' => MochigomaKind::Kyou,
						'P' => MochigomaKind::Fu,
						_ => {
							return Err(TypeConvertError::LogicError(String::from(
								"Logic error in the move analysis phase of the SFEN string analysis process."
							)));
						}
					};
					match chars.next() {
						Some(c) => match c {
							'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
								let x = c as u32 - '0' as u32;

								match chars.next() {
									Some(c) => {
										let y = match c {
											'a' => 1,
											'b' => 2,
											'c' => 3,
											'd' => 4,
											'e' => 5,
											'f' => 6,
											'g' => 7,
											'h' => 8,
											'i' => 9,
											_ => {
												return Err(TypeConvertError::SyntaxError(
													String::from(
														"Invalid SFEN character string (The format of the move is illegal)"
												)));
											}
										};

										match chars.next() {
											Some(_) => {
												return Err(TypeConvertError::LogicError(String::from(
													"Logic error in the move analysis phase of the SFEN string analysis process."
												)));
											},
											None => (),
										};
										Move::Put(mochigoma, KomaDstPutPosition(x,y))
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
					}
				},
				'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
					let x = c as u32 - '0' as u32;

					let src = match chars.next() {
						Some(c) => {
							let y = match c {
								'a' => 1,
								'b' => 2,
								'c' => 3,
								'd' => 4,
								'e' => 5,
								'f' => 6,
								'g' => 7,
								'h' => 8,
								'i' => 9,
								_ => {
									return Err(TypeConvertError::SyntaxError(String::from(
										"Invalid SFEN character string (The format of the move is illegal)"
									)));
								}
							};
							KomaSrcPosition(x,y)
						},
						None => {
							return Err(TypeConvertError::SyntaxError(
								String::from(
									"Invalid SFEN character string (The format of the move is illegal)"
							)));
						}
					};
					match chars.next() {
						Some(c) => match c {
							'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
								let x = c as u32 - '0' as u32;

								match chars.next() {
									Some(c) => {
										let y = match c {
											'a' => 1,
											'b' => 2,
											'c' => 3,
											'd' => 4,
											'e' => 5,
											'f' => 6,
											'g' => 7,
											'h' => 8,
											'i' => 9,
											_ => {
												return Err(TypeConvertError::SyntaxError(
													String::from(
														"Invalid SFEN character string (The format of the move is illegal)"
												)));
											}
										};

										match chars.next() {
											Some('+')  => Move::To(src, KomaDstToPosition(x,y,true)),
											None => Move::To(src, KomaDstToPosition(x,y,false)),
											_ => {
												return Err(TypeConvertError::SyntaxError(
													String::from(
														"Invalid SFEN character string (The format of the move is illegal)"
												)));
											}
										}
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
pub enum Move {
	To(KomaSrcPosition,KomaDstToPosition),
	Put(MochigomaKind,KomaDstPutPosition),
}
impl ToSfen<ToMoveStringConvertError> for Vec<Move> {
	fn to_sfen(&self) -> Result<String, ToMoveStringConvertError> {
		let mut strs:Vec<String> = Vec::with_capacity(self.len());

		for m in self {
			strs.push(MoveStringCreator::str_from(m)?);
		}

		Ok(strs.join(" "))
	}
}
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub enum LegalMove {
	To(KomaSrcPosition,KomaDstToPosition,Option<ObtainKind>),
	Put(MochigomaKind,KomaDstPutPosition),
}
impl LegalMove {
	pub fn to_move(&self) -> Move {
		match self  {
			&LegalMove::To(ref ms, ref md, _) => Move::To(*ms,*md),
			&LegalMove::Put(ref mk, ref md) => Move::Put(*mk,*md),
		}
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
			"+S" => KomaKind::SGinN,
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
			"+s" => KomaKind::GGinN,
			"+l" => KomaKind::GKyouN,
			"+p" => KomaKind::GFuN,
			_ => return Err(TypeConvertError::SyntaxError(s)),
		})
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
pub trait ToSfen<E> where E: Error + fmt::Display {
	fn to_sfen(&self) -> Result<String,E>;
}
impl ToSfen<TypeConvertError<String>> for Banmen {
	fn to_sfen(&self) -> Result<String,TypeConvertError<String>> {
		let mut s = String::new();

		match self {
			&Banmen(ref kinds) => {
				for y in 0..9 {
					let mut n = 0;
					for x in 0..9 {
						match kinds[y][x] {
							KomaKind::Blank => {
								n += 1;
							},
							k => {
								if n > 0 {
									s.push((n + '0' as u8) as char);
									n = 0;
								}

								match k {
									KomaKind::SOu => s.push('K'),
									KomaKind::SHisha => s.push('R'),
									KomaKind::SKaku => s.push('B'),
									KomaKind::SKin => s.push('G'),
									KomaKind::SGin => s.push('S'),
									KomaKind::SKei => s.push('N'),
									KomaKind::SKyou => s.push('L'),
									KomaKind::SFu => s.push('P'),
									KomaKind::SHishaN => {
										s.push('+');
										s.push('R');
									},
									KomaKind::SKakuN => {
										s.push('+');
										s.push('B');
									},
									KomaKind::SGinN => {
										s.push('+');
										s.push('S');
									},
									KomaKind::SKeiN => {
										s.push('+');
										s.push('N');
									},
									KomaKind::SKyouN => {
										s.push('+');
										s.push('L');
									},
									KomaKind::SFuN => {
										s.push('+');
										s.push('P');
									},
									KomaKind::GOu => s.push('k'),
									KomaKind::GHisha => s.push('r'),
									KomaKind::GKaku => s.push('b'),
									KomaKind::GKin => s.push('g'),
									KomaKind::GGin => s.push('s'),
									KomaKind::GKei => s.push('n'),
									KomaKind::GKyou => s.push('l'),
									KomaKind::GFu => s.push('p'),
									KomaKind::GHishaN => {
										s.push('+');
										s.push('r');
									},
									KomaKind::GKakuN => {
										s.push('+');
										s.push('b');
									},
									KomaKind::GGinN => {
										s.push('+');
										s.push('s');
									},
									KomaKind::GKeiN => {
										s.push('+');
										s.push('n');
									},
									KomaKind::GKyouN => {
										s.push('+');
										s.push('l');
									},
									KomaKind::GFuN => {
										s.push('+');
										s.push('p');
									},
									KomaKind::Blank => (),
								}
							}
						}
					}
					if n > 0 {
						s.push((n + '0' as u8) as char);
					}
					s.push('/');
				}
				s.pop();
			}
		}
		Ok(s)
	}
}
pub enum NextMove {
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
	&[NextMove::Once(0,-1),
		NextMove::Once(-1,-1),
		NextMove::Once(1,-1),
		NextMove::Once(-1,1),
		NextMove::Once(1,1)
	],
	// 金
	&[NextMove::Once(0,-1),
		NextMove::Once(-1,-1),
		NextMove::Once(1,-1),
		NextMove::Once(-1,0),
		NextMove::Once(1,0),
		NextMove::Once(0,1)
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
	&[NextMove::Once(0,-1),
		NextMove::Once(-1,-1),
		NextMove::Once(1,-1),
		NextMove::Once(-1,0),
		NextMove::Once(1,0),
		NextMove::Once(0,1),
		NextMove::Once(-1,1),
		NextMove::Once(1,1)
	],
	// 成歩
	&[NextMove::Once(0,-1),
		NextMove::Once(-1,-1),
		NextMove::Once(1,-1),
		NextMove::Once(-1,0),
		NextMove::Once(1,0),
		NextMove::Once(0,1)
	],
	// 成香
	&[NextMove::Once(0,-1),
		NextMove::Once(-1,-1),
		NextMove::Once(1,-1),
		NextMove::Once(-1,0),
		NextMove::Once(1,0),
		NextMove::Once(0,1)
	],
	// 成桂
	&[NextMove::Once(0,-1),
		NextMove::Once(-1,-1),
		NextMove::Once(1,-1),
		NextMove::Once(-1,0),
		NextMove::Once(1,0),
		NextMove::Once(0,1)
	],
	// 成銀
	&[NextMove::Once(0,-1),
		NextMove::Once(-1,-1),
		NextMove::Once(1,-1),
		NextMove::Once(-1,0),
		NextMove::Once(1,0),
		NextMove::Once(0,1)
	],
	// 成角
	&[NextMove::Repeat(-1,-1),
		NextMove::Repeat(1,-1),
		NextMove::Repeat(-1,1),
		NextMove::Repeat(1,1),
		NextMove::Once(0,-1),
		NextMove::Once(0,1),
		NextMove::Once(-1,0),
		NextMove::Once(1,0)
	],
	// 成飛
	&[NextMove::Once(-1,-1),
		NextMove::Once(1,-1),
		NextMove::Once(-1,1),
		NextMove::Once(1,1),
		NextMove::Repeat(0,-1),
		NextMove::Repeat(0,1),
		NextMove::Repeat(-1,0),
		NextMove::Repeat(1,0)
	],
];
impl Banmen {
	pub fn legal_moves_with_point_and_kind(&self,t:&Teban,x:u32,y:u32,kind:KomaKind)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		let kinds = match self {
			&Banmen(ref kinds) => kinds,
		};

		let x:i32 = x as i32;
		let y:i32 = y as i32;

		match *t {
			Teban::Sente if kind < KomaKind::GFu => {
				let mv = CANDIDATE[kind as usize];

				for m in mv {
					match m {
						&NextMove::Once(mx,my) => {
							if x + mx >= 0 && x + mx < 9 && y + my >= 0 && y + my < 9 {
								let dx = x + mx;
								let dy = y + my;
								match kinds[dy as usize][dx as usize] {
									KomaKind::Blank if  (kind == SFu && dy == 0) ||
														(kind == SKei && dy <= 2) => {
										mvs.push(LegalMove::To(
											KomaSrcPosition(9 - x as u32, (y + 1) as u32),
											KomaDstToPosition(
												9 - dx as u32, dy as u32 + 1, true),None));
									},
									KomaKind::Blank => {
										if  kind < SOu &&
											kind != KomaKind::SKin && dy <= 2 {

											mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, true),None));
										}
										mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),None));
									},
									dst if dst >= KomaKind::GFu &&
										((kind == SFu && dy == 0) || (kind == SKei && dy <= 1)) => {
										let obtained = match ObtainKind::try_from(dst) {
											Ok(obtained) => Some(obtained),
											Err(_) => None,
										};

										mvs.push(LegalMove::To(
											KomaSrcPosition(9 - x as u32, (y + 1) as u32),
											KomaDstToPosition(
												9 - dx as u32, dy as u32 + 1, true),obtained));
									},
									dst if dst >= KomaKind::GFu => {
										let obtained = match ObtainKind::try_from(dst) {
											Ok(obtained) => Some(obtained),
											Err(_) => None,
										};

										if  kind < SOu &&
											kind != KomaKind::SKin && dy <= 2 {

											mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, true),obtained));
										}

										mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),obtained));
									},
									_ => (),
								}
							}
						},
						&NextMove::Repeat(mx,my) => {
							let mut dx = x;
							let mut dy = y;

							while dx + mx >= 0 && dx + mx < 9 && dy + my >= 0 && dy + my < 9 {
								dx = dx + mx;
								dy = dy + my;

								match kinds[dy as usize][dx as usize] {
									KomaKind::Blank if kind == SKyou && dy == 0 => {
										mvs.push(LegalMove::To(
											KomaSrcPosition(9 - x as u32, (y + 1) as u32),
											KomaDstToPosition(
												9 - dx as u32, dy as u32 + 1, true),None));
									},
									KomaKind::Blank => {
										if  kind < KomaKind::SOu &&
											kind != KomaKind::SKin && dy <= 2 {

											mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, true),None));
										}
										mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),None));
									},
									dst if dst < KomaKind::GFu => {
										break;
									},
									dst if dst >= KomaKind::GFu && kind == SKyou && dy == 0 => {
										let obtained = match ObtainKind::try_from(dst) {
											Ok(obtained) => Some(obtained),
											Err(_) => None,
										};
										mvs.push(LegalMove::To(
											KomaSrcPosition(9 - x as u32, (y + 1) as u32),
											KomaDstToPosition(
												9 - dx as u32, dy as u32 + 1, true),obtained));
										break;
									},
									dst if dst >= KomaKind::GFu => {
										let obtained = match ObtainKind::try_from(dst) {
											Ok(obtained) => Some(obtained),
											Err(_) => None,
										};

										if  kind < KomaKind::SOu &&
											kind != KomaKind::SKin && dy <= 2 {

											mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, true),obtained));
										}

										mvs.push(LegalMove::To(
											KomaSrcPosition(9 - x as u32, (y + 1) as u32),
											KomaDstToPosition(
												9 - dx as u32, dy as u32 + 1, false),obtained));
										break;
									},
									_ => (),
								}
							}
						}
					}
				}
			},
			Teban::Gote if kind >= KomaKind::GFu && kind < KomaKind::Blank => {
				let mv = CANDIDATE[kind as usize - KomaKind::GFu as usize];
				for m in mv {
					match m {
						&NextMove::Once(mx,my) => {
							let mx = -mx;
							let my = -my;
							if x + mx >= 0 && x + mx < 9 && y + my >= 0 && y + my < 9 {
								let dx = x + mx;
								let dy = y + my;
								match kinds[dy as usize][dx as usize] {
									KomaKind::Blank if  (kind == GFu && dy == 8) ||
														(kind == GKei && dy >= 7) => {
										mvs.push(LegalMove::To(
											KomaSrcPosition(9 - x as u32, (y + 1) as u32),
											KomaDstToPosition(
												9 - dx as u32, dy as u32 + 1, true),None));
									},
									KomaKind::Blank => {
										if  kind < KomaKind::GOu &&
											kind != KomaKind::GKin && dy >= 6 {

											mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, true),None));
										}
										mvs.push(LegalMove::To(
											KomaSrcPosition(9 - x as u32, (y + 1) as u32),
											KomaDstToPosition(
												9 - dx as u32, dy as u32 + 1, false),None));
									},
									dst if dst < KomaKind::GFu &&
										((kind == GFu && dy == 8) || (kind == GKei && dy >= 7)) => {
										let obtained = match ObtainKind::try_from(dst) {
											Ok(obtained) => Some(obtained),
											Err(_) => None,
										};

										mvs.push(LegalMove::To(
											KomaSrcPosition(9 - x as u32, (y + 1) as u32),
											KomaDstToPosition(
												9 - dx as u32, dy as u32 + 1, true),obtained));
									},
									dst if dst < KomaKind::GFu => {
										let obtained = match ObtainKind::try_from(dst) {
											Ok(obtained) => Some(obtained),
											Err(_) => None,
										};

										if  kind < KomaKind::GOu &&
											kind != KomaKind::GKin && dy >= 6 {

											mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, true),obtained));
										}

										mvs.push(LegalMove::To(
											KomaSrcPosition(9 - x as u32, (y + 1) as u32),
											KomaDstToPosition(
												9 - dx as u32, dy as u32 + 1, false),obtained));
									},
									_ => (),
								}
							}
						},
						&NextMove::Repeat(mx,my) => {
							let mx = -mx;
							let my = -my;
							let mut dx = x;
							let mut dy = y;

							while dx + mx >= 0 && dx + mx < 9 && dy + my >= 0 && dy + my < 9 {
								dx = dx + mx;
								dy = dy + my;

								match kinds[dy as usize][dx as usize] {
									KomaKind::Blank if kind == GKyou && dy == 8 => {
										mvs.push(LegalMove::To(
											KomaSrcPosition(9 - x as u32, (y + 1) as u32),
											KomaDstToPosition(
												9 - dx as u32, dy as u32 + 1, true),None));
									},
									KomaKind::Blank => {
										if  kind < KomaKind::GOu &&
											kind != KomaKind::GKin && dy >= 6 {

											mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, true),None));
										}
										mvs.push(LegalMove::To(
											KomaSrcPosition(9 - x as u32, (y + 1) as u32),
											KomaDstToPosition(
												9 - dx as u32, dy as u32 + 1, false),None));
									},
									dst if dst >= KomaKind::GFu => {
										break;
									},
									dst if dst < KomaKind::GFu &&
										kind == GKyou && dy == 8 => {
										let obtained = match ObtainKind::try_from(dst) {
											Ok(obtained) => Some(obtained),
											Err(_) => None,
										};
										mvs.push(LegalMove::To(
											KomaSrcPosition(9 - x as u32, (y + 1) as u32),
											KomaDstToPosition(
												9 - dx as u32, dy as u32 + 1, true),obtained));
										break;
									},
									dst if dst < KomaKind::GFu => {
										let obtained = match ObtainKind::try_from(dst) {
											Ok(obtained) => Some(obtained),
											Err(_) => None,
										};

										if  kind < KomaKind::GOu &&
											kind != KomaKind::GKin && dy >= 6 {

											mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, true),obtained));
										}

										mvs.push(LegalMove::To(
											KomaSrcPosition(9 - x as u32, (y + 1) as u32),
											KomaDstToPosition(
												9 - dx as u32, dy as u32 + 1, false),obtained));
										break;
									},
									_ => (),
								}
							}
						}
					}
				}
			},
			_ => (),
		}
		mvs
	}

	pub fn legal_moves_with_point(&self,t:&Teban,x:u32,y:u32)
		-> Vec<LegalMove> {
		match self {
			&Banmen(ref kinds) => {
				self.legal_moves_with_point_and_kind(t,x,y,kinds[y as usize][x as usize])
			}
		}
	}
	pub fn legal_moves_with_src(&self,t:&Teban,src:KomaSrcPosition)
		-> Vec<LegalMove> {
		match src {
			KomaSrcPosition(x,y) => self.legal_moves_with_point(t, 9 - x, y - 1)
		}
	}

	pub fn legal_moves_with_dst_to(&self,t:&Teban,dst:KomaDstToPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstToPosition(x,y,_) => self.legal_moves_with_point(t, 9 - x, y - 1)
		}
	}

	pub fn legal_moves_with_dst_put(&self,t:&Teban,dst:KomaDstPutPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstPutPosition(x,y) => self.legal_moves_with_point(t, 9 - x, y - 1)
		}
	}

	pub fn legal_moves(&self,t:&Teban)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		match self {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match *t {
							Teban::Sente => (x,y),
							Teban::Gote => (8 - x, 8 - y),
						};
						mvs.append(&mut self.legal_moves_with_point(t, x as u32, y as u32));
					}
				}
			}
		}
		mvs
	}

	pub fn legal_moves_all(&self,t:&Teban,mc:&MochigomaCollections)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		match self {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match *t {
							Teban::Sente => (x,y),
							Teban::Gote => (8 - x, 8- y),
						};
						mvs.append(&mut self.legal_moves_with_point(t, x as u32, y as u32));
					}
				}
			}
		}
		mvs.append(&mut mc.legal_moves(t, self));
		mvs
	}

	pub fn win_only_moves_with_point_and_kind(&self,t:&Teban,x:u32,y:u32,kind:KomaKind)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		let kinds = match self {
			&Banmen(ref kinds) => kinds,
		};

		let x:i32 = x as i32;
		let y:i32 = y as i32;

		let ou = match *t {
			Teban::Sente => KomaKind::GOu,
			Teban::Gote => KomaKind::SOu,
		};

		let target = match self.find(&ou) {
			Some(ref r) => r[0],
			None => {
				return mvs;
			}
		};

		let (dx,dy) = match target {
			KomaPosition(x,y) => ((9 - x) as i32,(y - 1) as i32),
		};

		match *t {
			Teban::Sente if kind < KomaKind::GFu => {

				match kind {
					KomaKind::SFu |
						KomaKind::SGin |
						KomaKind::SKin |
						KomaKind::SOu |
						KomaKind::SFuN |
						KomaKind::SKyouN |
						KomaKind::SKeiN |
						KomaKind::SGinN => {

						if (dx - x).abs() > 1 || (dy - y).abs() > 1 {
							return mvs;
						}

						self.legal_moves_with_point_and_kind(t, x as u32, y as u32, kind)
							.into_iter().filter(|m| {
								match m {
									&LegalMove::To(_,_,Some(o)) if o == ObtainKind::Ou => true,
									_ => false,
								}
							}).collect::<Vec<LegalMove>>()
					},
					KomaKind::SKyou => {
						if dy > y || dx != x {
							return mvs;
						}

						let mut ty:i32 = y;

						while ty > dy {
							ty = ty - 1;

							if kinds[ty as usize][x as usize] == ou {
								break;
							}

							if kinds[ty as usize][x as usize] != KomaKind::Blank {
								return mvs;
							}
						}

						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - x as u32, ty as u32 + 1, false),
								Some(ObtainKind::Ou),
						));

						if ty < 3 {
							mvs.push(
								LegalMove::To(
									KomaSrcPosition(9 - x as u32,y as u32 + 1),
									KomaDstToPosition(9 - x as u32,ty as u32 + 1, true),
									Some(ObtainKind::Ou),
							));
						}
						mvs
					},
					KomaKind::SKei => {
						self.legal_moves_with_point_and_kind(t, x as u32, y as u32, kind)
							.into_iter().filter(|m| {
								match m {
									&LegalMove::To(_,_,Some(o)) if o == ObtainKind::Ou => true,
									_ => false,
								}
							}).collect::<Vec<LegalMove>>()
					},
					KomaKind::SKaku => {
						let mut tx:i32 = x;
						let mut ty:i32 = y;

						if dx - x < 0 && dx - x == dy - y {
							while tx > dx {
								tx = tx - 1;
								ty = ty - 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx - x == dy - y {
							while tx < dx {
								tx = tx + 1;
								ty = ty + 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx - x < 0 && -(dx - x) == dy - y {
							while tx > dx {
								tx = tx - 1;
								ty = ty + 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if -(dx - x) == dy - y {
							while tx < dx {
								tx = tx + 1;
								ty = ty - 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else {
							return mvs;
						}

						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - tx as u32,ty as u32 + 1,false),
								Some(ObtainKind::Ou),
						));

						if ty < 3 {
							mvs.push(
								LegalMove::To(
									KomaSrcPosition(9 - x as u32,y as u32 + 1),
									KomaDstToPosition(9 - tx as u32,ty as u32 + 1,true),
									Some(ObtainKind::Ou),
							));
						}
						mvs
					},
					KomaKind::SHisha => {
						let mut tx:i32 = x;
						let mut ty:i32 = y;

						if dy - y < 0 && dx == x {
							while ty > dy {
								ty = ty - 1;

								if kinds[ty as usize][x as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx == x {
							while ty < dy {
								ty = ty + 1;

								if kinds[ty as usize][x as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx - x < 0 && dy == y {
							while tx > dx {
								tx = tx - 1;

								if kinds[y as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dy == y {
							while tx < dx {
								tx = tx + 1;

								if kinds[y as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else {
							return mvs;
						}

						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - tx as u32,ty as u32 + 1,false),
								Some(ObtainKind::Ou),
						));

						if ty < 3 {
							mvs.push(
								LegalMove::To(
									KomaSrcPosition(9 - x as u32,y as u32 + 1),
									KomaDstToPosition(9 - tx as u32,ty as u32 + 1,true),
									Some(ObtainKind::Ou),
							));
						}
						mvs
					},
					KomaKind::SKakuN => {
						let mut tx:i32 = x;
						let mut ty:i32 = y;

						if (dx - x).abs() <= 1 && (dy - y).abs() <= 1 {
							tx = dx;
							ty = dy;
						} else if dx - x < 0 && dx - x == dy - y {
							while tx > dx {
								tx = tx - 1;
								ty = ty - 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx - x == dy - y {
							while tx < dx {
								tx = tx + 1;
								ty = ty + 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx - x < 0 && -(dx - x) == dy - y {
							while tx > dx {
								tx = tx - 1;
								ty = ty + 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if -(dx - x) == dy - y {
							while tx < dx {
								tx = tx + 1;
								ty = ty - 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else {
							return mvs;
						}

						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - tx as u32,ty as u32 + 1,false),
								Some(ObtainKind::Ou),
						));

						mvs
					},
					KomaKind::SHishaN => {
						let mut tx:i32 = x;
						let mut ty:i32 = y;

						if (dx - x).abs() <= 1 && (dy - y).abs() <= 1 {
							tx = dx;
							ty = dy;
						} else if dy - y < 0 && dx == x {
							while ty > dy {
								ty = ty - 1;

								if kinds[ty as usize][x as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx == x {
							while ty < dy {
								ty = ty + 1;

								if kinds[ty as usize][x as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx - x < 0 && dy == y {
							while tx > dx {
								tx = tx - 1;

								if kinds[y as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dy == y {
							while tx < dx {
								tx = tx + 1;

								if kinds[y as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else {
							return mvs;
						}

						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - tx as u32,ty as u32 + 1,false),
								Some(ObtainKind::Ou),
						));

						mvs
					},
					_ => mvs,
				}
			},
			Teban::Gote if kind >= KomaKind::GFu && kind < KomaKind::Blank => {
				match kind {
					KomaKind::GFu |
						KomaKind::GGin |
						KomaKind::GKin |
						KomaKind::GOu |
						KomaKind::GFuN |
						KomaKind::GKyouN |
						KomaKind::GKeiN |
						KomaKind::GGinN => {

						if (dx - x).abs() > 1 || (dy - y).abs() > 1 {
							return mvs;
						}

						self.legal_moves_with_point_and_kind(t, x as u32, y as u32, kind)
							.into_iter().filter(|m| {
								match m {
									&LegalMove::To(_,_,Some(o)) if o == ObtainKind::Ou => true,
									_ => false,
								}
							}).collect::<Vec<LegalMove>>()
					}
					KomaKind::GKyou => {
						if dy < y || dx != x {
							return mvs;
						}

						let mut ty:i32 = y;

						while ty < dy {
							ty = ty + 1;

							if kinds[ty as usize][x as usize] == ou {
								break;
							}

							if kinds[ty as usize][x as usize] != KomaKind::Blank {
								return mvs;
							}
						}

						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - x as u32,ty as u32 + 1,false),
								Some(ObtainKind::Ou),
						));

						if ty >= 6 {
							mvs.push(
								LegalMove::To(
									KomaSrcPosition(9 - x as u32,y as u32 + 1),
									KomaDstToPosition(9 - x as u32,ty as u32 + 1,true),
									Some(ObtainKind::Ou),
							));
						}
						mvs
					},
					KomaKind::GKei => {
						self.legal_moves_with_point_and_kind(t, x as u32, y as u32, kind)
							.into_iter().filter(|m| {
								match m {
									&LegalMove::To(_,_,Some(o)) if o == ObtainKind::Ou => true,
									_ => false,
								}
							}).collect::<Vec<LegalMove>>()
					},
					KomaKind::GKaku => {
						let mut tx:i32 = x;
						let mut ty:i32 = y;

						if dx - x < 0 && dx - x == dy - y {
							while tx > dx {
								tx = tx - 1;
								ty = ty - 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx - x == dy - y {
							while tx < dx {
								tx = tx + 1;
								ty = ty + 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx - x < 0 && -(dx - x) == dy - y {
							while tx > dx {
								tx = tx - 1;
								ty = ty + 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if -(dx - x) == dy - y {
							while tx < dx {
								tx = tx + 1;
								ty = ty - 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else {
							return mvs;
						}

						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - tx as u32,ty as u32 + 1,false),
								Some(ObtainKind::Ou),
						));

						if ty >= 6 {
							mvs.push(
								LegalMove::To(
									KomaSrcPosition(9 - x as u32,y as u32 + 1),
									KomaDstToPosition(9 - tx as u32,ty as u32 + 1,true),
									Some(ObtainKind::Ou),
							));
						}
						mvs
					},
					KomaKind::GHisha => {
						let mut tx:i32 = x;
						let mut ty:i32 = y;

						if dy - y < 0 && dx == x {
							while ty > dy {
								ty = ty - 1;

								if kinds[ty as usize][x as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx == x {
							while ty < dy {
								ty = ty + 1;

								if kinds[ty as usize][x as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx - x < 0 && dy == y {
							while tx > dx {
								tx = tx - 1;

								if kinds[y as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dy == y {
							while tx < dx {
								tx = tx + 1;

								if kinds[y as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else {
							return mvs;
						}

						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - tx as u32,ty as u32 + 1,false),
								Some(ObtainKind::Ou),
						));

						if ty >= 6 {
							mvs.push(
								LegalMove::To(
									KomaSrcPosition(9 - x as u32,y as u32 + 1),
									KomaDstToPosition(9 - tx as u32,ty as u32 + 1,true),
									Some(ObtainKind::Ou),
							));
						}
						mvs
					},
					KomaKind::GKakuN => {
						let mut tx:i32 = x;
						let mut ty:i32 = y;

						if (dx - x).abs() <= 1 && (dy - y).abs() <= 1 {
							tx = dx;
							ty = dy;
						} else if dx - x < 0 && dx - x == dy - y {
							while tx > dx {
								tx = tx - 1;
								ty = ty - 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx - x == dy - y {
							while tx < dx {
								tx = tx + 1;
								ty = ty + 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx - x < 0 && -(dx - x) == dy - y {
							while tx > dx {
								tx = tx - 1;
								ty = ty + 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if -(dx - x) == dy - y {
							while tx < dx {
								tx = tx + 1;
								ty = ty - 1;

								if kinds[ty as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else {
							return mvs;
						}

						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - tx as u32,ty as u32 + 1,false),
								Some(ObtainKind::Ou),
						));

						mvs
					},
					KomaKind::GHishaN => {
						let mut tx:i32 = x;
						let mut ty:i32 = y;

						if (dx - x).abs() <= 1 && (dy - y).abs() <= 1 {
							tx = dx;
							ty = dy;
						} else if dy - y < 0 && dx == x {
							while ty > dy {
								ty = ty - 1;

								if kinds[ty as usize][x as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx == x {
							while ty < dy {
								ty = ty + 1;

								if kinds[ty as usize][x as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dx - x < 0 && dy == y {
							while tx > dx {
								tx = tx - 1;

								if kinds[y as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else if dy == y {
							while tx < dx {
								tx = tx + 1;

								if kinds[y as usize][tx as usize] == ou {
									break;
								}

								if kinds[ty as usize][tx as usize] != KomaKind::Blank {
									return mvs;
								}
							}
						} else {
							return mvs;
						}

						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - tx as u32,ty as u32 + 1,false),
								Some(ObtainKind::Ou),
						));

						mvs
					},
					_ => mvs,
				}
			},
			_ => mvs,
		}
	}

	pub fn win_only_moves_with_point(&self,t:&Teban,x:u32,y:u32)
		-> Vec<LegalMove> {
		match self {
			&Banmen(ref kinds) => {
				self.win_only_moves_with_point_and_kind(t,x,y,kinds[y as usize][x as usize])
			}
		}
	}

	pub fn win_only_moves_with_src(&self,t:&Teban,src:KomaSrcPosition)
		-> Vec<LegalMove> {
		match src {
			KomaSrcPosition(x,y) => self.win_only_moves_with_point(t, 9 - x, y - 1)
		}
	}

	pub fn win_only_moves_with_dst_to(&self,t:&Teban,dst:KomaDstToPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstToPosition(x,y,_) => self.win_only_moves_with_point(t, 9 - x, y - 1)
		}
	}

	pub fn win_only_moves_with_dst_put(&self,t:&Teban,dst:KomaDstPutPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstPutPosition(x,y) => self.win_only_moves_with_point(t, 9 - x, y - 1)
		}
	}

	pub fn win_only_moves(&self,t:&Teban)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		match self {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match *t {
							Teban::Sente => (x,y),
							Teban::Gote => (8 - x, 8 - y),
						};
						mvs.append(&mut self.win_only_moves_with_point(t, x as u32, y as u32));
					}
				}
			}
		}
		mvs
	}

	pub fn oute_only_moves_with_point(&self,t:&Teban,mc:&MochigomaCollections,x:u32,y:u32)
		-> Vec<LegalMove> {
		self.legal_moves_with_point(t, x, y)
			.into_iter().filter(|m| {
					match m {
						&LegalMove::To(_,_,Some(ObtainKind::Ou)) => true,
						&LegalMove::To(ref s,ref d,_) => {
							match self.apply_move_none_check(t,mc,&Move::To(*s,*d)) {
								(ref b,_,_) => b.win_only_moves(t).len() > 0
							}
						},
						_ => false,
					}
			}).collect::<Vec<LegalMove>>()
	}

	pub fn oute_only_moves(&self,t:&Teban,mc:&MochigomaCollections)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		match self {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match *t {
							Teban::Sente => (x,y),
							Teban::Gote => (8 - x, 8- y),
						};
						mvs.append(&mut self.oute_only_moves_with_point(t, mc, x as u32, y as u32));
					}
				}
			}
		}
		mvs
	}

	pub fn oute_only_moves_all(&self,t:&Teban,mc:&MochigomaCollections)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		match self {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match *t {
							Teban::Sente => (x,y),
							Teban::Gote => (8 - x, 8- y),
						};
						mvs.append(&mut self.oute_only_moves_with_point(t, mc, x as u32, y as u32));
					}
				}
			}
		}
		mvs.append(&mut mc.oute_only_moves(t, self));
		mvs
	}

	pub fn respond_oute_only_moves_all(&self,t:&Teban,mc:&MochigomaCollections)
		-> Vec<LegalMove> {
		self.legal_moves_all(t, mc)
			.into_iter().filter(|m| {
					match m {
						&LegalMove::To(_,_,Some(ObtainKind::Ou)) => true,
						&LegalMove::To(ref s,ref d,_) => {
							match self.apply_move_none_check(t,mc,&Move::To(*s,*d)) {
								(ref b,_,_) => b.win_only_moves(&t.opposite()).len() == 0
							}
						},
						_ => false,
					}
			}).collect::<Vec<LegalMove>>()
	}

	pub fn apply_move_none_check(&self,t:&Teban,mc:&MochigomaCollections,m:&Move)
		-> (Banmen,MochigomaCollections,Option<MochigomaKind>) {

		let mut kinds = match *self {
			Banmen(ref kinds) => kinds.clone(),
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
								let c =match mc.get(&k) {
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

	pub fn apply_valid_move(&self,t:&Teban,mc:&MochigomaCollections,m:&Move)
		-> Result<(Banmen,MochigomaCollections,Option<MochigomaKind>),ShogiError> {

		match m {
			&Move::To(s,d) => {
				let mvs = self.legal_moves(t);

				match mvs.find(&(s,d)) {
					Some(_) => {
						Ok(self.apply_move_none_check(t,mc,m))
					},
					None => {
						Err(ShogiError::InvalidState(String::from(
							"This is not legal move."
						)))
					}
				}
			},
			&Move::Put(k,d) => {
				let mvs = mc.legal_moves(t,self);

				match mvs.find(&(k,d)) {
					Some(_) => {
						Ok(self.apply_move_none_check(t,mc,m))
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

	pub fn apply_moves(&self,mut teban:Teban,
						mut mc:MochigomaCollections,
						m:&Vec<Move>,mut mhash:u64,mut shash:u64,
						mut kyokumen_hash_map:TwoKeyHashMap<u32>,
						hasher:&KyokumenHash)
		-> (Teban,Banmen,MochigomaCollections,u64,u64,TwoKeyHashMap<u32>) {

		let mut banmen = self.clone();

		for m in m {
			match banmen.apply_move_none_check(&teban,&mc,&m) {
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

	pub fn is_nyugyoku_win(&self,t:&Teban,mc:&MochigomaCollections,limit:&(Option<Instant>,u32)) -> bool {
		if self.win_only_moves(&t.opposite()).len() > 0 {
			return false
		}

		if let &(Some(limit),inc) = limit {
			if limit + Duration::from_millis(inc as u64) > Instant::now() {
				return false;
			}
		}

		let ou = match *t {
			Teban::Sente => KomaKind::SOu,
			Teban::Gote => KomaKind::GOu,
		};

		let oy = match self.find(&ou) {
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

		match self {
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

	pub fn responded_oute(&self,t:&Teban,mc:&MochigomaCollections,m:&Move,nm:&Move)
		-> Result<bool,SelfMatchRunningError> {

		let o = t.opposite();

		if !match m {
			&Move::To(_,ref dst) if self.win_only_moves_with_dst_to(&o, *dst).len() == 0 => false,
			&Move::Put(_,ref dst) if self.win_only_moves_with_dst_put(&o, *dst).len() == 0 => false,
			_ => true,
		} {
			return Err(SelfMatchRunningError::InvalidState(String::from(
				"The argument m is not Move of oute."
			)));
		}

		let (kind,x,y) = match m {
			&Move::To(_,KomaDstToPosition(dx,dy,_)) => {
				match self {
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
				self.legal_moves_all(t, mc).into_iter().filter(|m| {
					match m {
						&LegalMove::To(_,_,Some(ObtainKind::Ou)) => true,
						&LegalMove::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,_),_) => {
							let (sx,sy) = ((9 - sx) as usize, (sy - 1) as usize);
							let (dx,dy) = ((9 - dx) as usize, (dy - 1) as usize);

							let ou = match *t {
								Teban::Sente => KomaKind::SOu,
								Teban::Gote => KomaKind::GOu,
							};

							match self {
								&Banmen(ref kinds) => {
									if kinds[sy][sx] == ou {
										true
									} else {
										let (tx,ty) = match self.find(&ou) {
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

							let (tx,ty) = match self.find(&ou) {
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
				self.legal_moves_all(t, mc).into_iter().filter(|m| {
					match m {
						&LegalMove::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,_),_) => {
							let (dx,dy) = ((9 - dx) as usize, (dy - 1) as usize);
							let (sx,sy) = ((9 - sx) as usize, (sy - 1) as usize);

							let ou = match *t {
								Teban::Sente => KomaKind::SOu,
								Teban::Gote => KomaKind::GOu,
							};

							match self {
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
}
impl Find<KomaPosition,Move> for Vec<LegalMove> {
	fn find(&self,query:&KomaPosition) -> Option<Move> {
		let (x,y) = match query {
			&KomaPosition(x,y) => (x,y)
		};

		for m in self {
			match m {
				&LegalMove::To(ref ms, ref md, _) => {
					match md {
						&KomaDstToPosition(dx,dy,_) => {
							if x == dx && y == dy {
								return Some(Move::To(*ms,*md));
							}
						}
					}
				},
				&LegalMove::Put(ref mk, ref md) => {
					match md {
						&KomaDstPutPosition(dx,dy) => {
							if x == dx && y == dy {
								return Some(Move::Put(*mk,*md));
							}
						}
					}
				}
			}
		}

		None
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
impl Find<(MochigomaKind,KomaDstPutPosition),Move> for Vec<LegalMove> {
	fn find(&self,query:&(MochigomaKind,KomaDstPutPosition)) -> Option<Move> {
		match query {
			&(ref k, ref d) => {
				for m in self {
					match m {
						&LegalMove::Put(ref mk, ref md) => {
							if k == mk && d == md {
								return Some(Move::Put(*k,*d));
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
impl Find<(KomaSrcPosition,KomaDstToPosition),Move> for Vec<LegalMove> {
	fn find(&self,query:&(KomaSrcPosition,KomaDstToPosition)) -> Option<Move> {
		match query {
			&(ref s, ref d) => {
				for m in self {
					match m {
						&LegalMove::To(ref ms, ref md, _) => {
							if s == ms && d == md {
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
impl Find<ObtainKind,Vec<Move>> for Vec<LegalMove> {
	fn find(&self,query:&ObtainKind) -> Option<Vec<Move>> {
		let mut mvs:Vec<Move> = Vec::new();

		for m in self {
			match m {
				&LegalMove::To(ref ms, ref md, Some(ref o)) => {
					if *o == *query {
						mvs.push(Move::To(*ms,*md));
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
	pub fn legal_moves(&self,t:&Teban,b:&Banmen) -> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		match *t {
			Teban::Sente => {
				match *self {
					MochigomaCollections::Pair(ref ms, _) => {
						match b {
							&Banmen(ref kinds) => {
								for y in 0..kinds.len() {
									for x in 0..kinds[y].len() {
										for m in &MOCHIGOMA_KINDS {
											match ms.get(&m) {
												None | Some(&0) => {
													continue;
												},
												Some(_) => (),
											}
											match m {
												&MochigomaKind::Fu => {
													match kinds[y][x] {
														KomaKind::Blank if y > 0 => {
															let mut nifu = false;

															for oy in 0..y {
																match kinds[oy][x] {
																	KomaKind::SFu => nifu = true,
																	_ => (),
																}
															}

															for oy in (y+1)..9 {
																match kinds[oy][x] {
																	KomaKind::SFu => nifu = true,
																	_ => (),
																}
															}

															if !nifu {
																mvs.push(
																	LegalMove::Put(*m,KomaDstPutPosition(
																	9 - x as u32, y as u32 + 1)));
															}
														},
														_ => (),
													}
												},
												&MochigomaKind::Kyou if y == 0 => (),
												&MochigomaKind::Kei if y <= 1 => (),
												_ => {
													match kinds[y][x] {
														KomaKind::Blank => {
															mvs.push(
																LegalMove::Put(*m,KomaDstPutPosition(
																9 - x as u32, y as u32 + 1)));
														},
														_ => (),
													}
												}
											}
										}
									}
								}
							}
						}
					},
					MochigomaCollections::Empty => (),
				}
			},
			Teban::Gote => {
				match *self {
					MochigomaCollections::Pair(_, ref mg) => {
						match b {
							&Banmen(ref kinds) => {
								for y in 0..kinds.len() {
									for x in 0..kinds[y].len() {
										let (x,y) = (8 - x, 8 - y);
										for m in &MOCHIGOMA_KINDS {
											match mg.get(&m) {
												None | Some(&0) => {
													continue;
												},
												Some(_) => (),
											}
											match m {
												&MochigomaKind::Fu => {
													match kinds[y][x] {
														KomaKind::Blank if y < 8 => {
															let mut nifu = false;

															for oy in 0..y {
																match kinds[oy][x] {
																	KomaKind::GFu => nifu = true,
																	_ => (),
																}
															}

															for oy in (y+1)..9 {
																match kinds[oy][x] {
																	KomaKind::GFu => nifu = true,
																	_ => (),
																}
															}

															if !nifu {
																mvs.push(LegalMove::Put(
																		*m,KomaDstPutPosition(
																		9 - x as u32, y as u32 + 1)));
															}
														},
														_ => (),
													}
												},
												&MochigomaKind::Kyou if y == 8 => (),
												&MochigomaKind::Kei if y >= 7 => (),
												_ => {
													match kinds[y][x] {
														KomaKind::Blank => {
															mvs.push(LegalMove::Put(
																	*m,KomaDstPutPosition(
																	9 - x as u32, y as u32 + 1)));
														},
														_ => (),
													}
												}
											}
										}
									}
								}
							}
						}
					},
					MochigomaCollections::Empty => (),
				}
			}
		}
		mvs
	}

	pub fn oute_only_moves(&self,t:&Teban,b:&Banmen) -> Vec<LegalMove> {
		self.legal_moves(t, b)
			.into_iter().filter(|m| {
				match m {
					&LegalMove::Put(k,KomaDstPutPosition(x,y)) => {
						b.win_only_moves_with_point_and_kind(t, 9 - x, y - 1, KomaKind::from((*t,k))).len() > 0
					},
					_ => false,
				}
			}).collect::<Vec<LegalMove>>()
	}
}
impl fmt::Debug for Banmen {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Banmen(ref v) => write!(f, "Banmen[{}]", v.iter()
												.map(|k| format!("{:?}", k))
												.collect::<Vec<String>>().join(" "))
		}
	}
}
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
impl<'a> TryFrom<&'a str,String> for Banmen {
	fn try_from(s: &'a str) -> Result<Banmen, TypeConvertError<String>> {
		let mut chars = s.chars();

		let mut banmen:[[KomaKind; 9]; 9] = [[KomaKind::Blank; 9]; 9];

		let mut x = 0;
		let mut y = 0;

		while let Some(c) = chars.next() {
			let mut s = String::new();

			s.push(match y {
				y if y >= 9=> {
					return Err(TypeConvertError::SyntaxError(
							String::from("Invalid SFEN character string (pieces outside the range of the board)")));
				},
				_ => c,
			});

			match c {
				_ if x > 9 => {
					return Err(TypeConvertError::LogicError(
						String::from("Logic error of SFEN character string parsing process.")));
				},
				'/' => if x == 9 {
					y += 1; x = 0;
				},
				_ if x == 9 => {
					return Err(TypeConvertError::SyntaxError(
						String::from("Invalid SFEN string (line separator '/' not found)")));
				},
				'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' if x + ((c as u32) - ('0' as u32)) > 9 => {
					return Err(TypeConvertError::SyntaxError(
							String::from("Invalid SFEN character string (pieces outside the range of the board)")));
				},
				'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
					x += (c as u32) - ('0' as u32);
				},
				'+' => match chars.next() {
					None => {
						return Err(TypeConvertError::SyntaxError(
							String::from("Invalid SFEN character string (illegal expression of piece)")));
					},
					Some(n) => {
						s.push(n);
						banmen[y as usize][x as usize] = KomaKind::try_from(s)?;
						x += 1;
					}
				},
				_ =>  {
					banmen[y as usize][x as usize] = KomaKind::try_from(s)?;
					x += 1;
				}
			}
		}

		Ok(Banmen(banmen))
	}
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
}
impl TryFrom<KomaKind,String> for ObtainKind {
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
			KomaKind::SFuN => ObtainKind::Fu,
			KomaKind::SKyouN => ObtainKind::Kyou,
			KomaKind::SKeiN => ObtainKind::Kei,
			KomaKind::SGinN => ObtainKind::Gin,
			KomaKind::SKakuN => ObtainKind::Kaku,
			KomaKind::SHishaN => ObtainKind::Hisha,
			KomaKind::GFu => ObtainKind::Fu,
			KomaKind::GKyou => ObtainKind::Kyou,
			KomaKind::GKei => ObtainKind::Kei,
			KomaKind::GGin => ObtainKind::Gin,
			KomaKind::GKin => ObtainKind::Kin,
			KomaKind::GKaku => ObtainKind::Kaku,
			KomaKind::GHisha => ObtainKind::Hisha,
			KomaKind::GOu => ObtainKind::Ou,
			KomaKind::GFuN => ObtainKind::Fu,
			KomaKind::GKyouN => ObtainKind::Kyou,
			KomaKind::GKeiN => ObtainKind::Kei,
			KomaKind::GGinN => ObtainKind::Gin,
			KomaKind::GKakuN => ObtainKind::Kaku,
			KomaKind::GHishaN => ObtainKind::Hisha,
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
impl TryFrom<ObtainKind,String> for MochigomaKind {
	fn try_from(o:ObtainKind) -> Result<MochigomaKind,TypeConvertError<String>> {
		Ok(match o {
			ObtainKind::Fu => MochigomaKind::Fu,
			ObtainKind::Kyou => MochigomaKind::Kyou,
			ObtainKind::Kei => MochigomaKind::Kei,
			ObtainKind::Gin => MochigomaKind::Gin,
			ObtainKind::Kin => MochigomaKind::Kin,
			ObtainKind::Kaku => MochigomaKind::Kaku,
			ObtainKind::Hisha => MochigomaKind::Hisha,
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
impl ToSfen<TypeConvertError<String>> for Teban {
	fn to_sfen(&self) -> Result<String, TypeConvertError<String>> {
		Ok(match *self {
			Teban::Sente => String::from("b"),
			Teban::Gote => String::from("w"),
		})
	}
}