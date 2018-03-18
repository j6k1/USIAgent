use std::fmt;
use std::fmt::Formatter;
use std::collections::HashMap;
use TryFrom;
use error::*;
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
pub struct Banmen(pub [[KomaKind; 9]; 9]);
impl Clone for Banmen {
	fn clone(&self) -> Banmen {
		match self {
			&Banmen(ref kinds) => Banmen(kinds.clone())
		}
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
									KomaKind::Blank => {
										mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),None));
										if  kind < SOu &&
											kind != KomaKind::SKin && dy <= 2 {

											mvs.push(LegalMove::To(
													KomaSrcPosition(9 - x as u32, (y + 1) as u32),
													KomaDstToPosition(
														9 - dx as u32, dy as u32 + 1, true),None));
										}
									},
									dst if dst >= KomaKind::GFu => {
										let obtained = match ObtainKind::try_from(dst) {
											Ok(obtained) => Some(obtained),
											Err(_) => None,
										};

										mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),obtained));
										if  kind < SOu &&
											kind != KomaKind::SKin && dy <= 2 {

											mvs.push(LegalMove::To(
													KomaSrcPosition(9 - x as u32, (y + 1) as u32),
													KomaDstToPosition(
														9 - dx as u32, dy as u32 + 1, true),obtained));
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
								dx = dx + mx;
								dy = dy + my;

								match kinds[dy as usize][dx as usize] {
									KomaKind::Blank => {
										mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),None));
										if  kind < KomaKind::SOu &&
											kind != KomaKind::SKin && dy <= 2 {

											mvs.push(LegalMove::To(
													KomaSrcPosition(9 - x as u32, (y + 1) as u32),
													KomaDstToPosition(
														9 - dx as u32, dy as u32 + 1, true),None));
										}
									},
									dst if dst < KomaKind::GFu => {
										break;
									},
									dst if dst >= KomaKind::GFu => {
										let obtained = match ObtainKind::try_from(dst) {
											Ok(obtained) => Some(obtained),
											Err(_) => None,
										};

										mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),obtained));
										if  kind < KomaKind::SOu &&
											kind != KomaKind::SKin && dy <= 2 {

											mvs.push(LegalMove::To(
													KomaSrcPosition(9 - x as u32, (y + 1) as u32),
													KomaDstToPosition(
														9 - dx as u32, dy as u32 + 1, true),obtained));
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
									KomaKind::Blank => {
										mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),None));
										if  kind < KomaKind::GOu &&
											kind != KomaKind::GKin && dy >= 6 {

											mvs.push(LegalMove::To(
													KomaSrcPosition(9 - x as u32, (y + 1) as u32),
													KomaDstToPosition(
														9 - dx as u32, dy as u32 + 1, true),None));
										}
									},
									dst if dst < KomaKind::GFu => {
										let obtained = match ObtainKind::try_from(dst) {
											Ok(obtained) => Some(obtained),
											Err(_) => None,
										};

										mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),obtained));
										if  kind < KomaKind::GOu &&
											kind != KomaKind::GKin && dy >= 6 {

											mvs.push(LegalMove::To(
													KomaSrcPosition(9 - x as u32, (y + 1) as u32),
													KomaDstToPosition(
														9 - dx as u32, dy as u32 + 1, true),obtained));
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
								dx = dx + mx;
								dy = dy + my;

								match kinds[dy as usize][dx as usize] {
									KomaKind::Blank => {
										mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),None));
										if  kind < KomaKind::GOu &&
											kind != KomaKind::GKin && dy >= 6 {

											mvs.push(LegalMove::To(
													KomaSrcPosition(9 - x as u32, (y + 1) as u32),
													KomaDstToPosition(
														9 - dx as u32, dy as u32 + 1, true),None));
										}
									},
									dst if dst >= KomaKind::GFu => {
										break;
									},
									dst if dst < KomaKind::GFu => {
										let obtained = match ObtainKind::try_from(dst) {
											Ok(obtained) => Some(obtained),
											Err(_) => None,
										};

										mvs.push(LegalMove::To(
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),obtained));
										if  kind < KomaKind::GOu &&
											kind != KomaKind::GKin && dy >= 6 {

											mvs.push(LegalMove::To(
													KomaSrcPosition(9 - x as u32, (y + 1) as u32),
													KomaDstToPosition(
														9 - dx as u32, dy as u32 + 1, true),obtained));
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
														KomaKind::Blank => {
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
														KomaKind::Blank => {
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
pub const BANMEN_START_POS:[[KomaKind; 9]; 9] = [
	[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
	[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
	[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
	[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
	[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
	[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
	[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
	[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
	[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
];
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