use std::collections::HashMap;
use std::time::{Instant,Duration};

use shogi::*;
use hash::*;
use error::*;
use event::*;

use shogi::KomaKind::{SFu,SKyou,SKei,SGin,SKin,SKaku,SHisha,SOu,GFu,GKyou,GKei,GGin,GKin,GKaku,GHisha,GOu,Blank};
use TryFrom;
use Find;

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
	pub fn legal_moves_with_point_and_kind(t:&Teban,banmen:&Banmen,x:u32,y:u32,kind:KomaKind)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		let kinds = match banmen {
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

	pub fn legal_moves_with_point(t:&Teban,banmen:&Banmen,x:u32,y:u32)
		-> Vec<LegalMove> {
		match banmen {
			&Banmen(ref kinds) => {
				Rule::legal_moves_with_point_and_kind(t,banmen,x,y,kinds[y as usize][x as usize])
			}
		}
	}
	pub fn legal_moves_with_src(t:&Teban,banmen:&Banmen,src:KomaSrcPosition)
		-> Vec<LegalMove> {
		match src {
			KomaSrcPosition(x,y) => Rule::legal_moves_with_point(t, banmen, 9 - x, y - 1)
		}
	}

	pub fn legal_moves_with_dst_to(t:&Teban,banmen:&Banmen,dst:KomaDstToPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstToPosition(x,y,_) => Rule::legal_moves_with_point(t, banmen, 9 - x, y - 1)
		}
	}

	pub fn legal_moves_with_dst_put(t:&Teban,banmen:&Banmen,dst:KomaDstPutPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstPutPosition(x,y) => Rule::legal_moves_with_point(t, banmen, 9 - x, y - 1)
		}
	}

	pub fn legal_moves_from_banmen(t:&Teban,banmen:&Banmen)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		match banmen {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match *t {
							Teban::Sente => (x,y),
							Teban::Gote => (8 - x, 8 - y),
						};
						mvs.append(&mut Rule::legal_moves_with_point(t, banmen, x as u32, y as u32));
					}
				}
			}
		}
		mvs
	}

	pub fn legal_moves_from_mochigoma(t:&Teban,mc:&MochigomaCollections,b:&Banmen) -> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		match *t {
			Teban::Sente => {
				match *mc {
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
				match *mc {
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

	pub fn legal_moves_all(t:&Teban,banmen:&Banmen,mc:&MochigomaCollections)
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
						mvs.append(&mut Rule::legal_moves_with_point(t, banmen, x as u32, y as u32));
					}
				}
			}
		}
		mvs.append(&mut Rule::legal_moves_from_mochigoma(t, mc, banmen));
		mvs
	}


	pub fn win_only_moves_with_point_and_kind(t:&Teban,banmen:&Banmen,x:u32,y:u32,kind:KomaKind)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		let kinds = match banmen {
			&Banmen(ref kinds) => kinds,
		};

		let x:i32 = x as i32;
		let y:i32 = y as i32;

		let ou = match *t {
			Teban::Sente => KomaKind::GOu,
			Teban::Gote => KomaKind::SOu,
		};

		let target = match banmen.find(&ou) {
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

						Rule::legal_moves_with_point_and_kind(t, banmen, x as u32, y as u32, kind)
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
						Rule::legal_moves_with_point_and_kind(t, banmen, x as u32, y as u32, kind)
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

						Rule::legal_moves_with_point_and_kind(t, banmen, x as u32, y as u32, kind)
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
						Rule::legal_moves_with_point_and_kind(t, banmen, x as u32, y as u32, kind)
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

	pub fn win_only_moves_with_point(t:&Teban,banmen:&Banmen,x:u32,y:u32)
		-> Vec<LegalMove> {
		match banmen {
			&Banmen(ref kinds) => {
				Rule::win_only_moves_with_point_and_kind(t,banmen,x,y,kinds[y as usize][x as usize])
			}
		}
	}

	pub fn win_only_moves_with_src(t:&Teban,banmen:&Banmen,src:KomaSrcPosition)
		-> Vec<LegalMove> {
		match src {
			KomaSrcPosition(x,y) => Rule::win_only_moves_with_point(t,banmen, 9 - x, y - 1)
		}
	}

	pub fn win_only_moves_with_dst_to(t:&Teban,banmen:&Banmen,dst:KomaDstToPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstToPosition(x,y,_) => Rule::win_only_moves_with_point(t, banmen, 9 - x, y - 1)
		}
	}

	pub fn win_only_moves_with_dst_put(t:&Teban,banmen:&Banmen,dst:KomaDstPutPosition)
		-> Vec<LegalMove> {
		match dst {
			KomaDstPutPosition(x,y) => Rule::win_only_moves_with_point(t, banmen, 9 - x, y - 1)
		}
	}

	pub fn win_only_moves(t:&Teban,banmen:&Banmen)
		-> Vec<LegalMove> {
		let mut mvs:Vec<LegalMove> = Vec::new();

		match banmen {
			&Banmen(ref kinds) => {
				for y in 0..kinds.len() {
					for x in 0..kinds[y].len() {
						let (x,y) = match *t {
							Teban::Sente => (x,y),
							Teban::Gote => (8 - x, 8 - y),
						};
						mvs.append(&mut Rule::win_only_moves_with_point(t, banmen, x as u32, y as u32));
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
						mut kyokumen_hash_map:TwoKeyHashMap<u32>,
						hasher:&KyokumenHash)
		-> (Teban,Banmen,MochigomaCollections,u64,u64,TwoKeyHashMap<u32>) {

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
									kyokumen_hash_map:&mut TwoKeyHashMap<u32>) -> bool {
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
									oute_kyokumen_hash_map:&mut Option<TwoKeyHashMap<u32>>)
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
}