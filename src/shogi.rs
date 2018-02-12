use std::fmt;
use std::fmt::Formatter;
use TryFrom;
use error::*;
use Validate;
use Find;
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
	pub fn legal_moves_with_point(&self,t:&Teban,x:u32,y:u32)
		-> Vec<(KomaSrcPosition,KomaDstToPosition,Option<ObtainKind>)> {
		let mut mvs:Vec<(KomaSrcPosition,KomaDstToPosition,Option<ObtainKind>)> = Vec::new();

		let kinds = match *self {
			Banmen(ref kinds) => kinds,
		};

		let x:i32 = x as i32;
		let y:i32 = y as i32;

		match *t {
			Teban::Sente if kinds[y as usize][x as usize] < KomaKind::GFu => {
				let kind = kinds[y as usize][x as usize];
				let mv = CANDIDATE[kind as usize];

				for m in mv {
					match m {
						&NextMove::Once(mx,my) => {
							if x + mx >= 0 && x + mx < 9 && y + my >= 0 && y + my < 9 {
								let dx = x + mx;
								let dy = y + my;
								match kinds[my as usize][mx as usize] {
									KomaKind::Blank => {
										mvs.push((
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),None));
										if  kind < SOu &&
											kind != KomaKind::SKin &&
											kind != KomaKind::SGin && dy >= 6 {

											mvs.push((
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

										mvs.push((
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),obtained));
										if  kind < SOu &&
											kind != KomaKind::SKin &&
											kind != KomaKind::SGin && dy >= 6 {

											mvs.push((
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

								match kinds[my as usize][mx as usize] {
									KomaKind::Blank => {
										mvs.push((
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),None));
										if  kind < KomaKind::SOu &&
											kind != KomaKind::SKin &&
											kind != KomaKind::SGin && dy >= 6 {

											mvs.push((
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

										mvs.push((
												KomaSrcPosition(9 - (x + 1) as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),obtained));
										if  kind < KomaKind::SOu &&
											kind != KomaKind::SKin &&
											kind != KomaKind::SGin && dy >= 6 {

											mvs.push((
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
			Teban::Gote if kinds[y as usize][x as usize] < KomaKind::Blank => {
				let kind = kinds[y as usize][x as usize];
				let mv = CANDIDATE[kind as usize - KomaKind::GFu as usize];
				for m in mv {
					match m {
						&NextMove::Once(mx,my) => {
							let mx = -mx;
							let my = -my;
							if x + mx >= 0 && x + mx < 9 && y + my >= 0 && y + my < 9 {
								let dx = x + mx;
								let dy = y + my;
								match kinds[my as usize][mx as usize] {
									KomaKind::Blank => {
										mvs.push((
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),None));
										if  kind < KomaKind::GOu &&
											kind != KomaKind::GKin &&
											kind != KomaKind::GGin && dy <= 2 {

											mvs.push((
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

										mvs.push((
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),obtained));
										if  kind < KomaKind::GOu &&
											kind != KomaKind::GKin &&
											kind != KomaKind::GGin && dy <= 2 {

											mvs.push((
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

								match kinds[my as usize][mx as usize] {
									KomaKind::Blank => {
										mvs.push((
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),None));
										if  kind < KomaKind::GOu &&
											kind != KomaKind::GKin &&
											kind != KomaKind::GGin && dy <= 2 {

											mvs.push((
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

										mvs.push((
												KomaSrcPosition(9 - x as u32, (y + 1) as u32),
												KomaDstToPosition(
													9 - dx as u32, dy as u32 + 1, false),obtained));
										if  kind < KomaKind::GOu &&
											kind != KomaKind::GKin &&
											kind != KomaKind::GGin && dy <= 2 {

											mvs.push((
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

	pub fn legal_moves_with_src(&self,t:&Teban,src:KomaSrcPosition)
		-> Vec<(KomaSrcPosition,KomaDstToPosition,Option<ObtainKind>)> {
		match src {
			KomaSrcPosition(x,y) => self.legal_moves_with_point(t, 9 - x, y)
		}
	}

	pub fn legal_moves_with_dst_to(&self,t:&Teban,dst:KomaDstToPosition)
		-> Vec<(KomaSrcPosition,KomaDstToPosition,Option<ObtainKind>)> {
		match dst {
			KomaDstToPosition(x,y,_) => self.legal_moves_with_point(t, 9 - x, y)
		}
	}

	pub fn legal_moves_with_dst_put(&self,t:&Teban,dst:KomaDstPutPosition)
		-> Vec<(KomaSrcPosition,KomaDstToPosition,Option<ObtainKind>)> {
		match dst {
			KomaDstPutPosition(x,y) => self.legal_moves_with_point(t, 9 - x, y)
		}
	}

	pub fn legal_moves(&self,t:&Teban)
		-> Vec<(KomaSrcPosition,KomaDstToPosition,Option<ObtainKind>)> {
		let mut mvs:Vec<(KomaSrcPosition,KomaDstToPosition,Option<ObtainKind>)> = Vec::new();

		match *self {
			Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						mvs.append(&mut self.legal_moves_with_point(t, x as u32, y as u32));
					}
				}
			}
		}
		mvs
	}

	pub fn apply_move_none_check(&self,t:&Teban,mc:&MochigomaCollections,m:&Move)
		-> (Banmen,MochigomaCollections) {

		let mut kinds = match *self {
			Banmen(ref kinds) => kinds.clone(),
		};

		let nmc = match m {
			&Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,n)) => {
				let k = kinds[sy as usize][(9 - sx) as usize];

				kinds[sy as usize][(9 - sx) as usize] = KomaKind::Blank;

				match kinds[dy as usize][(9 - dx) as usize] {
					KomaKind::Blank => {
						kinds[dy as usize][(9 - dx) as usize] = match n {
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
						mc.clone()
					},
					dst => {
						let obtained = match ObtainKind::try_from(dst) {
							Ok(obtained) => {
								match obtained {
									ObtainKind::Fu => Some(MochigomaKind::Fu),
									ObtainKind::Kyou => Some(MochigomaKind::Kyou),
									ObtainKind::Kei => Some(MochigomaKind::Kei),
									ObtainKind::Gin => Some(MochigomaKind::Gin),
									ObtainKind::Kin => Some(MochigomaKind::Kin),
									ObtainKind::Kaku => Some(MochigomaKind::Kaku),
									ObtainKind::Hisha => Some(MochigomaKind::Hisha),
									_ => None,
								}
							},
							Err(_) => None,
						};

						kinds[dy as usize][(9 - dx) as usize] = match n {
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
												ms.push(obtained);
												MochigomaCollections::Pair(ms,mg.clone())
											},
											Teban::Gote => {
												let mut mg = mg.clone();
												mg.push(obtained);
												MochigomaCollections::Pair(ms.clone(),mg)
											}
										}
									},
									&MochigomaCollections::Empty => {
										match *t {
											Teban::Sente => {
												let mut ms:Vec<MochigomaKind> = Vec::new();
												ms.push(obtained);
												MochigomaCollections::Pair(ms,Vec::new())
											},
											Teban::Gote => {
												let mut mg:Vec<MochigomaKind> = Vec::new();
												mg.push(obtained);
												MochigomaCollections::Pair(Vec::new(),mg)
											}
										}
									}
								}
							},
							None => {
								mc.clone()
							}
						}
					}
				}
			},
			&Move::Put(k,KomaDstPutPosition(dx,dy)) => {
				kinds[dy as usize][(9 - dx) as usize] = KomaKind::from((*t,k));
				mc.clone()
			}
		};

		(Banmen(kinds),nmc)
	}

	pub fn apply_valid_move(&self,t:&Teban,mc:&MochigomaCollections,m:&Move)
		-> Result<(Banmen,MochigomaCollections),ShogiError> {

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
impl Find<KomaPosition,Move> for Vec<(KomaSrcPosition,KomaDstToPosition,Option<ObtainKind>)> {
	fn find(&self,query:&KomaPosition) -> Option<Move> {
		let (x,y) = match query {
			&KomaPosition(x,y) => (x,y)
		};

		for m in self {
			match m {
				&(ref ms, ref md, _) => {
					match md {
						&KomaDstToPosition(dx,dy,_) => {
							if x == dx && y == dy {
								return Some(Move::To(*ms,*md));
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
							r.push(KomaPosition(9 - x as u32, y as u32));
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
impl Find<(MochigomaKind,KomaDstPutPosition),Move> for Vec<(MochigomaKind,KomaDstPutPosition)> {
	fn find(&self,query:&(MochigomaKind,KomaDstPutPosition)) -> Option<Move> {
		match query {
			&(ref k, ref d) => {
				for m in self {
					match m {
						&(ref mk, ref md) => {
							if k == mk && d == md {
								return Some(Move::Put(*k,*d));
							}
						}
					}
				}
			}
		}

		None
	}
}
impl Find<(KomaSrcPosition,KomaDstToPosition),Move> for Vec<(KomaSrcPosition,KomaDstToPosition,Option<ObtainKind>)> {
	fn find(&self,query:&(KomaSrcPosition,KomaDstToPosition)) -> Option<Move> {
		match query {
			&(ref s, ref d) => {
				for m in self {
					match m {
						&(ref ms, ref md, _) => {
							if s == ms && d == md {
								return Some(Move::To(*s,*d));
							}
						}
					}
				}
			}
		}

		None
	}
}
#[derive(Debug)]
pub enum MochigomaCollections {
	Empty,
	Pair(Vec<MochigomaKind>,Vec<MochigomaKind>),
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
	pub fn legal_moves(&self,t:&Teban,b:&Banmen) -> Vec<(MochigomaKind,KomaDstPutPosition)> {
		let mut mvs:Vec<(MochigomaKind,KomaDstPutPosition)> = Vec::new();

		match *t {
			Teban::Sente => {
				match *self {
					MochigomaCollections::Pair(ref ms, _) => {
						for m in ms {
							match b {
								&Banmen(ref kinds) => {
									for y in 0..kinds.len() {
										for x in 0..kinds[y].len() {
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
																mvs.push((*m,KomaDstPutPosition(
																			9 - x as u32, y as u32 + 1)));
															}
														},
														_ => (),
													}
												},
												_ => {
													match kinds[y][x] {
														KomaKind::Blank => {
															mvs.push((*m,KomaDstPutPosition(
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
						for m in mg {
							match b {
								&Banmen(ref kinds) => {
									for y in 0..kinds.len() {
										for x in 0..kinds[y].len() {
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
																mvs.push((*m,KomaDstPutPosition(
																			9 - x as u32, y as u32 + 1)));
															}
														},
														_ => (),
													}
												},
												_ => {
													match kinds[y][x] {
														KomaKind::Blank => {
															mvs.push((*m,KomaDstPutPosition(
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