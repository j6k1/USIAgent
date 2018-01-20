use std::fmt;
use std::fmt::Formatter;
use TryFrom;
use error::*;
use Validate;
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
pub struct KomaSrcPosition(pub u32,pub u32);
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub struct KomaDstToPosition(pub u32,pub u32,pub bool);
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
pub struct KomaDstPutPosition(pub u32,pub u32);
impl<'a> TryFrom<&'a str,String> for Move {
	fn try_from(s: &'a str) -> Result<Move, TypeConvertError<String>> {
		match s {
			s if s.len() != 4 => {
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
								let x = c as u32;

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
					let x = c as u32;

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
								let x = c as u32;

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
pub enum NextMove {
	Once(i32,i32),
	Repeat(i32,i32),
}
const CANDIDATE:[&[NextMove]; 13] = [
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
	pub fn legal_moves(&self) -> Vec<KomaDstToPosition> {
		let mut mvs:Vec<KomaDstToPosition> = Vec::new();

		match *self {
			Banmen(ref kinds) => {
				for i in 0..kinds.len() {
					match kinds[i] {
						KomaKind::Blank => (),
						kind if kind < KomaKind::GFu => {
							let mv = CANDIDATE[kind as usize];
							for m in mv {
								let x = (i % 9) as i32;
								let y = (i / 9) as i32;

								match m {
									&NextMove::Once(mx,my) => {
										if x + mx >= 0 && x + mx < 9 && y + my >= 0 && y + my < 9 {
											let p = (i as i32 + mx + my * 9) as usize;
											let dx = x + mx;
											let dy = y + my;
											match kinds[p] {
												KomaKind::Blank => {
													mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, false));
													if  kind < SOu &&
														kind != KomaKind::SKin &&
														kind != KomaKind::SGin && dy <= 2 {

														mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, true));
													}
												},
												dst if dst >= KomaKind::GFu => {
													mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, false));
													if  kind < SOu &&
														kind != KomaKind::SKin &&
														kind != KomaKind::SGin && dy <= 2 {

														mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, true));
													}
												},
												_ => (),
											}
										}
									},
									&NextMove::Repeat(mx,my) => {
										let mut dx = x;
										let mut dy = y;

										while dx + mx >= 0 && dx + mx < 9 && dy + my >= 0 && dy + my < 9 {
											let p = (i as i32 + mx + my * 9) as usize;
											dx = dx + mx;
											dy = dy + my;

											match kinds[p] {
												KomaKind::Blank => {
													mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, false));
													if  kind < KomaKind::SOu &&
														kind != KomaKind::SKin &&
														kind != KomaKind::SGin && dy <= 2 {

														mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, true));
													}
												},
												dst if dst >= KomaKind::GFu => {
													mvs.push(KomaDstToPosition(dx as u32, dy as u32, false));
													if  kind < KomaKind::SOu &&
														kind != KomaKind::SKin &&
														kind != KomaKind::SGin && dy <= 2 {

														mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, true));
													}
													break;
												},
												_ => (),
											}
										}
									}
								}
							}
						},
						kind => {
							let mv = CANDIDATE[kind as usize - KomaKind::GFu as usize];
							for m in mv {
								let x = (i % 9) as i32;
								let y = (i / 9) as i32;

								match m {
									&NextMove::Once(mx,my) => {
										let mx = -mx;
										let my = -my;
										if x + mx >= 0 && x + mx < 9 && y + my >= 0 && y + my < 9 {
											let p = (i as i32 + mx + my * 9) as usize;
											let dx = x + mx;
											let dy = y + my;
											match kinds[p] {
												KomaKind::Blank => {
													mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, false));
													if  kind < KomaKind::GOu &&
														kind != KomaKind::GKin &&
														kind != KomaKind::GGin && dy >= 6 {

														mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, true));
													}
												},
												dst if dst < KomaKind::GFu => {
													mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, false));
													if  kind < KomaKind::GOu &&
														kind != KomaKind::GKin &&
														kind != KomaKind::GGin && dy >= 6 {

														mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, true));
													}
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
											let p = (i as i32 + mx + my * 9) as usize;
											dx = dx + mx;
											dy = dy + my;

											match kinds[p] {
												KomaKind::Blank => {
													mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, false));
													if  kind < KomaKind::GOu &&
														kind != KomaKind::GKin &&
														kind != KomaKind::GGin && dy >= 6 {

														mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, true));
													}
												},
												dst if dst < KomaKind::GFu => {
													mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, false));
													if  kind < KomaKind::GOu &&
														kind != KomaKind::GKin &&
														kind != KomaKind::GGin && dy >= 6 {

														mvs.push(KomaDstToPosition(dx as u32 + 1, dy as u32 + 1, true));
													}
													break;
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
			}
		}
		mvs
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
pub const BANMEN_START_POS:[KomaKind; 81] = [
	GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou,
	Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank,
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
			KomaSrcPosition(x, y) => x < 9 && y < 9,
		}
	}
}
impl Validate for KomaDstToPosition {
	fn validate(&self) -> bool {
		match *self {
			KomaDstToPosition(x, y, _) => x < 9 && y < 9,
		}
	}
}
impl Validate for KomaDstPutPosition {
	fn validate(&self) -> bool {
		match *self {
			KomaDstPutPosition(x, y) => x < 9 && y < 9,
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