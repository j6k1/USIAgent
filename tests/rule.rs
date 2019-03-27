extern crate usiagent;

use std::collections::HashMap;
use std::time::{Instant};

use usiagent::TryFrom;
use usiagent::Find;
use usiagent::shogi::*;
use usiagent::protocol::*;
use usiagent::event::*;
use usiagent::error::*;
use usiagent::rule;
use usiagent::rule::Rule;
use usiagent::rule::State;
use usiagent::rule::BANMEN_START_POS;
use usiagent::hash::*;

#[allow(unused)]
use usiagent::shogi::KomaKind::{
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

#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
enum LegalMove {
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
#[test]
fn test_legal_moves_banmen_with_fu_corner_sente() {
	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![
		],
		vec![
			((0,1),(0,0,true),None),
		],
		vec![
			((0,2),(0,1,true),None),
			((0,2),(0,1,false),None)
		],
		vec![
		],
		vec![
			((8,1),(8,0,true),None)
		],
		vec![
			((8,2),(8,1,true),None),
			((8,2),(8,1,false),None)
		]
	];

	const POSITIONS:[(usize,usize); 6] = [
		(0,0),(0,1),(0,2),
		(8,0),(8,1),(8,2)
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SFu;

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}

}
#[test]
fn test_legal_moves_banmen_with_fu_corner_gote() {
	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![
		],
		vec![
			((0,1),(0,0,true),None),
		],
		vec![
			((0,2),(0,1,true),None),
			((0,2),(0,1,false),None)
		],
		vec![
		],
		vec![
			((8,1),(8,0,true),None)
		],
		vec![
			((8,2),(8,1,true),None),
			((8,2),(8,1,false),None)
		]
	];

	const POSITIONS:[(usize,usize); 6] = [
		(0,0),(0,1),(0,2),
		(8,0),(8,1),(8,2)
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),_) => {
					LegalMove::from(((8 - sx, 8 - sy),(8- dx, 8 - dy, nari),None))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();


	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-p.1][8-p.0] = GFu;

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}

}
#[test]
fn test_legal_moves_banmen_with_fu_nari_border_sente() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((4,3),(4,2,true),Some(ObtainKind::Fu)),
		((4,3),(4,2,false),Some(ObtainKind::Fu)),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 0..9 {
		wall_banmen.0[2][x] = GFu;
	}

	let mut banmen = wall_banmen.clone();

	banmen.0[3][4] = SFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_border_gote() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((4,3),(4,2,true),Some(ObtainKind::Fu)),
		((4,3),(4,2,false),Some(ObtainKind::Fu)),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((8 - sx,8 - sy),(8 - dx,8 - dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 0..9 {
		wall_banmen.0[6][x] = SFu;
	}

	let mut banmen = wall_banmen.clone();

	banmen.0[5][4] = GFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)
}
#[test]
fn test_legal_moves_banmen_with_kaku_all_position_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = SKaku;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_all_position_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-y][8-x] = GKaku;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_occupied_corner_self_sente() {
	const POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((0,0),(1,1)),
		((0,8),(1,7)),
		((8,0),(7,1)),
		((8,8),(7,7)),
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();
		let (c,p) = *p;

		banmen.0[c.1][c.0] = SFu;
		banmen.0[p.1][p.0] = SKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_occupied_corner_opponent_sente() {
	const POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((0,0),(1,1)),
		((0,8),(1,7)),
		((8,0),(7,1)),
		((8,8),(7,7)),
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();
		let (c,p) = *p;

		banmen.0[c.1][c.0] = SFu;
		banmen.0[p.1][p.0] = GKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_occupied_corner_self_gote() {
	const POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((0,0),(1,1)),
		((0,8),(1,7)),
		((8,0),(7,1)),
		((8,8),(7,7)),
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();
		let (c,p) = *p;

		banmen.0[c.1][c.0] = GFu;
		banmen.0[p.1][p.0] = GKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_occupied_corner_opponent_gote() {
	const POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((0,0),(1,1)),
		((0,8),(1,7)),
		((8,0),(7,1)),
		((8,8),(7,7)),
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();
		let (c,p) = *p;

		banmen.0[c.1][c.0] = GFu;
		banmen.0[p.1][p.0] = SKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_occupied_corner2_sente() {
	const KINDS:[KomaKind; 2] = [SFu,GFu];

	const POSITIONS:[((usize,usize),(usize,usize),(usize,usize)); 4] = [
		((0,0),(1,1),(2,2)),
		((0,8),(1,7),(2,6)),
		((8,0),(7,1),(6,2)),
		((8,8),(7,7),(6,6)),
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();
		let (c1,c2,p) = *p;

		for k1 in &KINDS {
			for k2 in &KINDS {
				banmen.0[c1.1][c1.0] = *k1;
				banmen.0[c2.1][c2.0] = *k2;
				banmen.0[p.1][p.0] = SKaku;

				assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
					Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_occupied_corner2_gote() {
	const KINDS:[KomaKind; 2] = [GFu,SFu];

	const POSITIONS:[((usize,usize),(usize,usize),(usize,usize)); 4] = [
		((0,0),(1,1),(2,2)),
		((0,8),(1,7),(2,6)),
		((8,0),(7,1),(6,2)),
		((8,8),(7,7),(6,6)),
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();
		let (c1,c2,p) = *p;

		for k1 in &KINDS {
			for k2 in &KINDS {
				banmen.0[c1.1][c1.0] = *k1;
				banmen.0[c2.1][c2.0] = *k2;
				banmen.0[p.1][p.0] = GKaku;

				assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
					Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_left_wall_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][0] = SFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][1] = SKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_right_wall_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][8] = SFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][7] = SKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_top_wall_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[0][x] = SFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[1][x] = SKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_bottom_wall_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[8][x] = SFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[7][x] = SKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_left_wall_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][0] = GFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][1] = SKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_right_wall_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][8] = GFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][7] = SKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_top_wall_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[0][x] = GFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[1][x] = SKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_bottom_wall_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[8][x] = GFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[7][x] = SKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_left_wall_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][0] = GFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][1] = GKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_right_wall_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][8] = GFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][7] = GKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_top_wall_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[0][x] = GFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[1][x] = GKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_bottom_wall_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[8][x] = GFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[7][x] = GKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_left_wall_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][0] = SFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][1] = GKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_right_wall_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][8] = SFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][7] = GKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_top_wall_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[0][x] = SFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[1][x] = GKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_bottom_wall_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[8][x] = SFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[7][x] = GKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_left_wall_inside_sente() {
	const KOMAKINDS:[KomaKind; 2] = [SFu,GFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for y in 1..8 {
				wall_banmen.0[y][0] = *k1;
				wall_banmen.0[y][1] = *k2;
			}

			for y in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[y][2] = SKaku;

				assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
					Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_right_wall_inside_sente() {
	const KOMAKINDS:[KomaKind; 2] = [SFu,GFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for y in 1..8 {
				wall_banmen.0[y][8] = *k1;
				wall_banmen.0[y][7] = *k2;
			}

			for y in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[y][6] = SKaku;

				assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
					Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_top_wall_inside_sente() {
	const KOMAKINDS:[KomaKind; 2] = [SFu,GFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for x in 1..8 {
				wall_banmen.0[0][x] = *k1;
				wall_banmen.0[1][x] = *k2;
			}

			for x in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[2][x] = SKaku;

				assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
					Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_bottom_wall_inside_sente() {
	const KOMAKINDS:[KomaKind; 2] = [SFu,GFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for x in 1..8 {
				wall_banmen.0[8][x] = *k1;
				wall_banmen.0[7][x] = *k2;
			}

			for x in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[6][x] = SKaku;

				assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
					Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_left_wall_inside_gote() {
	const KOMAKINDS:[KomaKind; 2] = [GFu,SFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for y in 1..8 {
				wall_banmen.0[y][0] = *k1;
				wall_banmen.0[y][1] = *k2;
			}

			for y in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[y][2] = GKaku;

				assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
					Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_right_wall_inside_gote() {
	const KOMAKINDS:[KomaKind; 2] = [GFu,SFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for y in 1..8 {
				wall_banmen.0[y][8] = *k1;
				wall_banmen.0[y][7] = *k2;
			}

			for y in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[y][6] = GKaku;

				assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
					Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_top_wall_inside_gote() {
	const KOMAKINDS:[KomaKind; 2] = [GFu,SFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for x in 1..8 {
				wall_banmen.0[0][x] = *k1;
				wall_banmen.0[1][x] = *k2;
			}

			for x in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[2][x] = GKaku;

				assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
					Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_bottom_wall_inside_gote() {
	const KOMAKINDS:[KomaKind; 2] = [GFu,SFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for x in 1..8 {
				wall_banmen.0[8][x] = *k1;
				wall_banmen.0[7][x] = *k2;
			}

			for x in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[6][x] = GKaku;

				assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
					Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_border_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 0..9 {
		wall_banmen.0[2][x] = GFu;
	}

	let mut banmen = wall_banmen.clone();

	banmen.0[4][4] = SKaku;

	assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
		Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_border_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 0..9 {
		wall_banmen.0[6][x] = SFu;
	}

	let mut banmen = wall_banmen.clone();

	banmen.0[4][4] = GKaku;

	assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
		Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_banmen_with_kakun_all_position_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = SKakuN;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_all_position_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-y][8-x] = GKakuN;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_occupied_corner_self_sente() {
	const POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((0,0),(1,1)),
		((0,8),(1,7)),
		((8,0),(7,1)),
		((8,8),(7,7)),
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();
		let (c,p) = *p;

		banmen.0[c.1][c.0] = SFu;
		banmen.0[p.1][p.0] = SKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_occupied_corner_opponent_sente() {
	const POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((0,0),(1,1)),
		((0,8),(1,7)),
		((8,0),(7,1)),
		((8,8),(7,7)),
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();
		let (c,p) = *p;

		banmen.0[c.1][c.0] = SFu;
		banmen.0[p.1][p.0] = GKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_occupied_corner_self_gote() {
	const POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((0,0),(1,1)),
		((0,8),(1,7)),
		((8,0),(7,1)),
		((8,8),(7,7)),
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();
		let (c,p) = *p;

		banmen.0[c.1][c.0] = GFu;
		banmen.0[p.1][p.0] = GKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_occupied_corner_opponent_gote() {
	const POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((0,0),(1,1)),
		((0,8),(1,7)),
		((8,0),(7,1)),
		((8,8),(7,7)),
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();
		let (c,p) = *p;

		banmen.0[c.1][c.0] = GFu;
		banmen.0[p.1][p.0] = SKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_occupied_corner2_sente() {
	const KINDS:[KomaKind; 2] = [SFu,GFu];

	const POSITIONS:[((usize,usize),(usize,usize),(usize,usize)); 4] = [
		((0,0),(1,1),(2,2)),
		((0,8),(1,7),(2,6)),
		((8,0),(7,1),(6,2)),
		((8,8),(7,7),(6,6)),
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();
		let (c1,c2,p) = *p;

		for k1 in &KINDS {
			for k2 in &KINDS {
				banmen.0[c1.1][c1.0] = *k1;
				banmen.0[c2.1][c2.0] = *k2;
				banmen.0[p.1][p.0] = SKakuN;

				assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
					Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_occupied_corner2_gote() {
	const KINDS:[KomaKind; 2] = [GFu,SFu];

	const POSITIONS:[((usize,usize),(usize,usize),(usize,usize)); 4] = [
		((0,0),(1,1),(2,2)),
		((0,8),(1,7),(2,6)),
		((8,0),(7,1),(6,2)),
		((8,8),(7,7),(6,6)),
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();
		let (c1,c2,p) = *p;

		for k1 in &KINDS {
			for k2 in &KINDS {
				banmen.0[c1.1][c1.0] = *k1;
				banmen.0[c2.1][c2.0] = *k2;
				banmen.0[p.1][p.0] = GKakuN;

				assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
					Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_left_wall_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][0] = SFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][1] = SKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_right_wall_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][8] = SFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][7] = SKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_top_wall_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[0][x] = SFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[1][x] = SKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_bottom_wall_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[8][x] = SFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[7][x] = SKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_left_wall_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][0] = GFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][1] = SKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_right_wall_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][8] = GFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][7] = SKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_top_wall_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[0][x] = GFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[1][x] = SKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_bottom_wall_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[8][x] = GFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[7][x] = SKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_left_wall_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][0] = GFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][1] = GKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_right_wall_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][8] = GFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][7] = GKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_top_wall_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[0][x] = GFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[1][x] = GKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_bottom_wall_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[8][x] = GFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[7][x] = GKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_left_wall_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][0] = SFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][1] = GKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_right_wall_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for y in 1..8 {
		wall_banmen.0[y][8] = SFu;
	}

	for y in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[y][7] = GKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_top_wall_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[0][x] = SFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[1][x] = GKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_bottom_wall_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 1..8 {
		wall_banmen.0[8][x] = SFu;
	}

	for x in 2..7 {
		let mut banmen = wall_banmen.clone();

		banmen.0[7][x] = GKakuN;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_left_wall_inside_sente() {
	const KOMAKINDS:[KomaKind; 2] = [SFu,GFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for y in 1..8 {
				wall_banmen.0[y][0] = *k1;
				wall_banmen.0[y][1] = *k2;
			}

			for y in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[y][2] = SKakuN;

				assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
					Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_right_wall_inside_sente() {
	const KOMAKINDS:[KomaKind; 2] = [SFu,GFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for y in 1..8 {
				wall_banmen.0[y][8] = *k1;
				wall_banmen.0[y][7] = *k2;
			}

			for y in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[y][6] = SKakuN;

				assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
					Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_top_wall_inside_sente() {
	const KOMAKINDS:[KomaKind; 2] = [SFu,GFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for x in 1..8 {
				wall_banmen.0[0][x] = *k1;
				wall_banmen.0[1][x] = *k2;
			}

			for x in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[2][x] = SKakuN;

				assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
					Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_bottom_wall_inside_sente() {
	const KOMAKINDS:[KomaKind; 2] = [SFu,GFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for x in 1..8 {
				wall_banmen.0[8][x] = *k1;
				wall_banmen.0[7][x] = *k2;
			}

			for x in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[6][x] = SKakuN;

				assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
					Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_left_wall_inside_gote() {
	const KOMAKINDS:[KomaKind; 2] = [GFu,SFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for y in 1..8 {
				wall_banmen.0[y][0] = *k1;
				wall_banmen.0[y][1] = *k2;
			}

			for y in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[y][2] = GKakuN;

				assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
					Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_right_wall_inside_gote() {
	const KOMAKINDS:[KomaKind; 2] = [GFu,SFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for y in 1..8 {
				wall_banmen.0[y][8] = *k1;
				wall_banmen.0[y][7] = *k2;
			}

			for y in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[y][6] = GKakuN;

				assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
					Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_top_wall_inside_gote() {
	const KOMAKINDS:[KomaKind; 2] = [GFu,SFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for x in 1..8 {
				wall_banmen.0[0][x] = *k1;
				wall_banmen.0[1][x] = *k2;
			}

			for x in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[2][x] = GKakuN;

				assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
					Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_bottom_wall_inside_gote() {
	const KOMAKINDS:[KomaKind; 2] = [GFu,SFu];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k1 in &KOMAKINDS {
		for k2 in &KOMAKINDS {
			let mut wall_banmen = blank_banmen.clone();

			for x in 1..8 {
				wall_banmen.0[8][x] = *k1;
				wall_banmen.0[7][x] = *k2;
			}

			for x in 3..6 {
				let mut banmen = wall_banmen.clone();

				banmen.0[6][x] = GKakuN;

				assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
					Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kakun_nari_border_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 0..9 {
		wall_banmen.0[2][x] = GFu;
	}

	let mut banmen = wall_banmen.clone();

	banmen.0[4][4] = SKakuN;

	assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
		Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_banmen_with_kakun_nari_border_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 0..9 {
		wall_banmen.0[6][x] = SFu;
	}

	let mut banmen = wall_banmen.clone();

	banmen.0[4][4] = GKakuN;

	assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
		Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_contiguous_sente() {
	const KOMAKINDS:[KomaKind; 2] = [SFu,GFu];

	const OFFSETS:[(i32,i32); 4] = [
		(-1,0),(1,0),(0,-1),(0,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &KOMAKINDS {
		for o in &OFFSETS {
			let mut banmen = blank_banmen.clone();

			banmen.0[4][4] = SKakuN;

			banmen.0[(4 + o.1) as usize][(4 + o.0) as usize] = *k;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_contiguous_gote() {
	const KOMAKINDS:[KomaKind; 2] = [GFu,SFu];

	const OFFSETS:[(i32,i32); 4] = [
		(-1,0),(1,0),(0,-1),(0,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &KOMAKINDS {
		for o in &OFFSETS {
			let mut banmen = blank_banmen.clone();

			banmen.0[4][4] = GKakuN;

			banmen.0[(4 + o.1) as usize][(4 + o.0) as usize] = *k;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_next_occupied_lefttop_to_rightbottom_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..8 {
		for x in 0..8 {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = SKaku;
			banmen.0[y+1][x+1] = GFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_next_occupied_righttop_to_leftbottom_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..8 {
		for x in (1..9).rev() {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = SKaku;
			banmen.0[y+1][x-1] = GFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_next_occupied_lefttop_to_rightbottom_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..8 {
		for x in 0..8 {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = GKaku;
			banmen.0[y+1][x+1] = SFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_next_occupied_righttop_to_leftbottom_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..8 {
		for x in (1..9).rev() {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = GKaku;
			banmen.0[y+1][x-1] = SFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_prev_occupied_lefttop_to_rightbottom_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 1..9 {
		for x in 1..9 {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = SKaku;
			banmen.0[y-1][x-1] = GFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_prev_occupied_righttop_to_leftbottom_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 1..9 {
		for x in (0..8).rev() {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = SKaku;
			banmen.0[y-1][x+1] = GFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_prev_occupied_lefttop_to_rightbottom_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 1..9 {
		for x in 1..9 {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = GKaku;
			banmen.0[y-1][x-1] = SFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_prev_occupied_righttop_to_leftbottom_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 1..9 {
		for x in (0..8).rev() {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = GKaku;
			banmen.0[y-1][x+1] = SFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_next_occupied_lefttop_to_rightbottom_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..8 {
		for x in 0..8 {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = SKakuN;
			banmen.0[y+1][x+1] = GFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_next_occupied_righttop_to_leftbottom_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..8 {
		for x in (1..9).rev() {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = SKakuN;
			banmen.0[y+1][x-1] = GFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_next_occupied_lefttop_to_rightbottom_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..8 {
		for x in 0..8 {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = GKakuN;
			banmen.0[y+1][x+1] = SFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_next_occupied_righttop_to_leftbottom_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..8 {
		for x in (1..9).rev() {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = GKakuN;
			banmen.0[y+1][x-1] = SFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_prev_occupied_lefttop_to_rightbottom_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 1..9 {
		for x in 1..9 {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = SKakuN;
			banmen.0[y-1][x-1] = GFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_prev_occupied_righttop_to_leftbottom_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 1..9 {
		for x in (0..8).rev() {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = SKakuN;
			banmen.0[y-1][x+1] = GFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_prev_occupied_lefttop_to_rightbottom_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 1..9 {
		for x in 1..9 {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = GKakuN;
			banmen.0[y-1][x-1] = SFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_prev_occupied_righttop_to_leftbottom_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 1..9 {
		for x in (0..8).rev() {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][x] = GKakuN;
			banmen.0[y-1][x+1] = SFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_7_squares_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut banmen = blank_banmen.clone();

		banmen.0[y][6] = SKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_8_squares_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut banmen = blank_banmen.clone();

		banmen.0[y][7] = SKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_7_squares_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut banmen = blank_banmen.clone();

		banmen.0[y][6] = GKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_8_squares_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut banmen = blank_banmen.clone();

		banmen.0[y][7] = GKaku;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_7_squares_and_contiguous_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OFFSETS:[(i32,i32); 4] = [
		(-1,0),
		(0,-1),
		(0,1),
		(1,0)
	];

	for y in 0..9 {
		for o in &OFFSETS {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][6] = SKakuN;

			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 {
				continue;
			}

			banmen.0[(y as i32+o.1) as usize][(6+o.0) as usize] = SFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_8_squares_and_contiguous_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OFFSETS:[(i32,i32); 4] = [
		(-1,0),
		(0,-1),
		(0,1),
		(1,0)
	];

	for y in 0..9 {
		for o in &OFFSETS {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][7] = SKakuN;

			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 {
				continue;
			}

			banmen.0[(y as i32+o.1) as usize][(6+o.0) as usize] = SFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_7_squares_and_contiguous_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OFFSETS:[(i32,i32); 4] = [
		(-1,0),
		(0,-1),
		(0,1),
		(1,0)
	];

	for y in 0..9 {
		for o in &OFFSETS {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][6] = GKakuN;

			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 {
				continue;
			}

			banmen.0[(y as i32+o.1) as usize][(6+o.0) as usize] = GFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_nari_8_squares_and_contiguous_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OFFSETS:[(i32,i32); 4] = [
		(-1,0),
		(0,-1),
		(0,1),
		(1,0)
	];

	for y in 0..9 {
		for o in &OFFSETS {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][7] = GKakuN;

			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 {
				continue;
			}

			banmen.0[(y as i32+o.1) as usize][(6+o.0) as usize] = GFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_corner_sente() {
	const POSITIONS:[(usize,usize); 16] = [
		(0,0),(0,1),(1,0),(1,1),
		(0,7),(0,8),(1,7),(1,8),
		(7,0),(7,1),(8,0),(8,1),
		(7,7),(7,8),(8,7),(8,8)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SHisha;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_corner_gote() {
	const POSITIONS:[(usize,usize); 16] = [
		(0,0),(0,1),(1,0),(1,1),
		(0,7),(0,8),(1,7),(1,8),
		(7,0),(7,1),(8,0),(8,1),
		(7,7),(7,8),(8,7),(8,8)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &POSITIONS {
		let mut banmen = blank_banmen.clone();

		banmen.0[8 - p.1][8 - p.0] = GHisha;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_nari_border_sente() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		banmen.0[2][x] = GFu;
	}

	banmen.0[3][4] = SHisha;

	assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
		Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_banmen_with_hisha_nari_border_gote() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		banmen.0[6][x] = SFu;
	}

	banmen.0[5][4] = GHisha;

	assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
		Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_banmen_with_hisha_dst_occupied_self_sente() {
	const OFFSETS:[(i32,i32); 8] = [
		(-2,0),(-1,0),(0,-2),(0,-1),(0,1),(0,2),(1,0),(2,0)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for o in &OFFSETS {
		let mut banmen = blank_banmen.clone();

		banmen.0[4][4] = SHisha;

		banmen.0[(4+o.1) as usize][(4+o.0) as usize] = SFu;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_dst_occupied_opponent_sente() {
	const OFFSETS:[(i32,i32); 8] = [
		(-2,0),(-1,0),(0,-2),(0,-1),(0,1),(0,2),(1,0),(2,0)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for o in &OFFSETS {
		let mut banmen = blank_banmen.clone();

		banmen.0[4][4] = SHisha;

		banmen.0[(4+o.1) as usize][(4+o.0) as usize] = GFu;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_dst_occupied_self_gote() {
	const OFFSETS:[(i32,i32); 8] = [
		(-2,0),(-1,0),(0,-2),(0,-1),(0,1),(0,2),(1,0),(2,0)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for o in &OFFSETS {
		let mut banmen = blank_banmen.clone();

		banmen.0[4][4] = GHisha;

		banmen.0[(4+o.1) as usize][(4+o.0) as usize] = GFu;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_dst_occupied_opponent_gote() {
	const OFFSETS:[(i32,i32); 8] = [
		(-2,0),(-1,0),(0,-2),(0,-1),(0,1),(0,2),(1,0),(2,0)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for o in &OFFSETS {
		let mut banmen = blank_banmen.clone();

		banmen.0[4][4] = GHisha;

		banmen.0[(4+o.1) as usize][(4+o.0) as usize] = SFu;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_nari_7_squares_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut banmen = blank_banmen.clone();

		banmen.0[y][6] = SHisha;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_nari_8_squares_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut banmen = blank_banmen.clone();

		banmen.0[y][7] = SHisha;

		assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_nari_7_squares_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut banmen = blank_banmen.clone();

		banmen.0[y][6] = GHisha;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_nari_8_squares_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut banmen = blank_banmen.clone();

		banmen.0[y][7] = GHisha;

		assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_nari_7_squares_and_contiguous_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OFFSETS:[(i32,i32); 4] = [
		(-1,-1),
		(-1,1),
		(1,-1),
		(1,1)
	];

	for y in 0..9 {
		for o in &OFFSETS {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][6] = SHishaN;

			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 {
				continue;
			}

			banmen.0[(y as i32+o.1) as usize][(6+o.0) as usize] = SFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_nari_8_squares_and_contiguous_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OFFSETS:[(i32,i32); 4] = [
		(-1,-1),
		(-1,1),
		(1,-1),
		(1,1)
	];

	for y in 0..9 {
		for o in &OFFSETS {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][7] = SHishaN;

			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 {
				continue;
			}

			banmen.0[(y as i32+o.1) as usize][(6+o.0) as usize] = SFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Sente,&banmen),
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_nari_7_squares_and_contiguous_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OFFSETS:[(i32,i32); 4] = [
		(-1,-1),
		(-1,1),
		(1,-1),
		(1,1)
	];

	for y in 0..9 {
		for o in &OFFSETS {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][6] = GHishaN;

			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 {
				continue;
			}

			banmen.0[(y as i32+o.1) as usize][(6+o.0) as usize] = GFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_nari_8_squares_and_contiguous_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OFFSETS:[(i32,i32); 4] = [
		(-1,0),
		(0,-1),
		(0,1),
		(1,0)
	];

	for y in 0..9 {
		for o in &OFFSETS {
			let mut banmen = blank_banmen.clone();

			banmen.0[y][7] = GHishaN;

			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 {
				continue;
			}

			banmen.0[(y as i32+o.1) as usize][(6+o.0) as usize] = GFu;

			assert_eq!(legal_moves_from_banmen(&Teban::Gote,&banmen),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kyou_corner_sente() {
	const POSITIONS:[(usize,usize); 6] = [
		(0,0),(0,1),(0,2),
		(8,0),(8,1),(8,2)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![
		],
		vec![
			((0,1),(0,0,true),None)
		],
		vec![
			((0,2),(0,1,true),None),
			((0,2),(0,1,false),None),
			((0,2),(0,0,true),None)
		],
		vec![
		],
		vec![
			((8,1),(8,0,true),None)
		],
		vec![
			((8,2),(8,1,true),None),
			((8,2),(8,1,false),None),
			((8,2),(8,0,true),None)
		],
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SKyou;

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_kyou_corner_gote() {
	const POSITIONS:[(usize,usize); 6] = [
		(0,0),(0,1),(0,2),
		(8,0),(8,1),(8,2)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![
		],
		vec![
			((0,1),(0,0,true),None)
		],
		vec![
			((0,2),(0,1,true),None),
			((0,2),(0,1,false),None),
			((0,2),(0,0,true),None)
		],
		vec![
		],
		vec![
			((8,1),(8,0,true),None)
		],
		vec![
			((8,2),(8,1,true),None),
			((8,2),(8,1,false),None),
			((8,2),(8,0,true),None)
		],
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),_) => {
					LegalMove::from(((8 - sx, 8 - sy),(8- dx, 8 - dy, nari),None))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8 - p.1][8 - p.0] = GKyou;

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_border_sente() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((4,4),(4,3,false),None),
		((4,4),(4,2,true),Some(ObtainKind::Fu)),
		((4,4),(4,2,false),Some(ObtainKind::Fu))
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][4] = SKyou;
	banmen.0[2][4] = GFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_border_gote() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((4,4),(4,3,false),None),
		((4,4),(4,2,true),Some(ObtainKind::Fu)),
		((4,4),(4,2,false),Some(ObtainKind::Fu))
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((8 - sx, 8 - sy),(8- dx, 8 - dy, nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][4] = GKyou;
	banmen.0[6][4] = SFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_border_occupied_self_sente() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((4,2),(4,1,true),None),
		((4,2),(4,1,false),None),
		((4,4),(4,3,false),None)
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][4] = SKyou;
	banmen.0[2][4] = SFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_border_occupied_self_gote() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((4,2),(4,1,true),None),
		((4,2),(4,1,false),None),
		((4,4),(4,3,false),None)
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((8 - sx, 8 - sy),(8- dx, 8 - dy, nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][4] = GKyou;
	banmen.0[6][4] = GFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)
}
#[test]
fn test_legal_moves_banmen_with_kei_corner_sente() {
	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![],
		vec![],
		vec![((0,2),(1,0,true),None)],
		vec![],
		vec![],
		vec![((1,2),(0,0,true),None),((1,2),(2,0,true),None)],
		vec![],
		vec![],
		vec![((8,2),(7,0,true),None)],
		vec![],
		vec![],
		vec![((7,2),(6,0,true),None),((7,2),(8,0,true),None)]
	];

	const POSITIONS:[(usize,usize); 12] = [
		(0,0),(0,1),(0,2),(1,0),(1,1),(1,2),
		(8,0),(8,1),(8,2),(7,0),(7,1),(7,2)
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SKei;

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_kei_corner_gote() {
	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![],
		vec![],
		vec![((0,2),(1,0,true),None)],
		vec![],
		vec![],
		vec![((1,2),(0,0,true),None),((1,2),(2,0,true),None)],
		vec![],
		vec![],
		vec![((8,2),(7,0,true),None)],
		vec![],
		vec![],
		vec![((7,2),(6,0,true),None),((7,2),(8,0,true),None)]
	];

	const POSITIONS:[(usize,usize); 12] = [
		(0,0),(0,1),(0,2),(1,0),(1,1),(1,2),
		(8,0),(8,1),(8,2),(7,0),(7,1),(7,2)
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),_) => {
					LegalMove::from(((8 - sx, 8 - sy),(8- dx, 8 - dy, nari),None))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-p.1][8-p.0] = GKei;

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_kei_jump_over_wall_self_sente() {
	let mut answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = Vec::new();

	answer.push((0..18).into_iter().map(|x| {
		((x / 2,2),(x / 2,1,(x % 2 == 0)),None)
	}).collect::<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>>());

	answer[0].push(((1,3),(0,1,true),None));
	answer[0].push(((1,3),(2,1,true),None));

	answer.push((0..18).into_iter().map(|x| {
		((x / 2,2),(x / 2,1,(x % 2 == 0)),None)
	}).collect::<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>>());

	answer[1].push(((7,3),(6,1,true),None));
	answer[1].push(((7,3),(8,1,true),None));

	const POSITIONS:[(usize,usize); 2] = [
		(1,3),(7,3)
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SKei;

		for x in 0..9 {
			banmen.0[2][x] = SFu;
		}

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_kei_jump_over_opponent_self_sente() {
	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,3),(0,1,true),None),((1,3),(2,1,true),None)],
		vec![((7,3),(6,1,true),None),((7,3),(8,1,true),None)]
	];

	const POSITIONS:[(usize,usize); 2] = [
		(1,3),(7,3)
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SKei;

		for x in 0..9 {
			banmen.0[2][x] = GFu;
		}

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_kei_jump_over_wall_self_gote() {
	let mut answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = Vec::new();

	answer.push((0..18).rev().into_iter().map(|x| {
		((x / 2,6),(x / 2,7,(x % 2 == 1)),None)
	}).collect::<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>>());

	answer[0].push(((1,5),(2,7,true),None));
	answer[0].push(((1,5),(0,7,true),None));

	answer.push((0..18).rev().into_iter().map(|x| {
		((x / 2,6),(x / 2,7,(x % 2 == 1)),None)
	}).collect::<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>>());

	answer[1].push(((7,5),(8,7,true),None));
	answer[1].push(((7,5),(6,7,true),None));

	const POSITIONS:[(usize,usize); 2] = [
		(1,5),(7,5)
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = GKei;

		for x in 0..9 {
			banmen.0[6][x] = GFu;
		}

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_kei_jump_over_opponent_self_gote() {
	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,5),(2,7,true),None),((1,5),(0,7,true),None)],
		vec![((7,5),(8,7,true),None),((7,5),(6,7,true),None)]
	];

	const POSITIONS:[(usize,usize); 2] = [
		(1,5),(7,5)
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = GKei;

		for x in 0..9 {
			banmen.0[6][x] = SFu;
		}

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_kei_7_squares_sente() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((6,4),(5,2,true),None),
		((6,4),(5,2,false),None),
		((6,4),(7,2,true),None),
		((6,4),(7,2,false),None),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][6] = SKei;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)
}
#[test]
fn test_legal_moves_banmen_with_kei_8_squares_sente() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((7,4),(6,2,true),None),
		((7,4),(6,2,false),None),
		((7,4),(8,2,true),None),
		((7,4),(8,2,false),None),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][7] = SKei;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)
}
#[test]
fn test_legal_moves_banmen_with_kei_7_squares_gote() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((2,4),(3,6,true),None),
		((2,4),(3,6,false),None),
		((2,4),(1,6,true),None),
		((2,4),(1,6,false),None),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][2] = GKei;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)
}
#[test]
fn test_legal_moves_banmen_with_kei_8_squares_gote() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((1,4),(2,6,true),None),
		((1,4),(2,6,false),None),
		((1,4),(0,6,true),None),
		((1,4),(0,6,false),None),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][1] = GKei;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)
}
#[test]
fn test_legal_moves_banmen_with_kei_7_squares_dst_occupied_self_sente() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((7,2),(7,1,true),None),
		((7,2),(7,1,false),None),
		((6,4),(5,2,true),None),
		((6,4),(5,2,false),None),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][6] = SKei;
	banmen.0[2][7] = SFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)

}
#[test]
fn test_legal_moves_banmen_with_kei_7_squares_dst_occupied_opponent_sente() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((6,4),(5,2,true),None),
		((6,4),(5,2,false),None),
		((6,4),(7,2,true),Some(ObtainKind::Fu)),
		((6,4),(7,2,false),Some(ObtainKind::Fu)),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][6] = SKei;
	banmen.0[2][7] = GFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)

}
#[test]
fn test_legal_moves_banmen_with_kei_8_squares_dst_occupied_self_sente() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((6,2),(6,1,true),None),
		((6,2),(6,1,false),None),
		((7,4),(8,2,true),None),
		((7,4),(8,2,false),None),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][7] = SKei;
	banmen.0[2][6] = SFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)

}
#[test]
fn test_legal_moves_banmen_with_kei_8_squares_dst_occupied_opponent_sente() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((7,4),(6,2,true),Some(ObtainKind::Fu)),
		((7,4),(6,2,false),Some(ObtainKind::Fu)),
		((7,4),(8,2,true),None),
		((7,4),(8,2,false),None),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][7] = SKei;
	banmen.0[2][6] = GFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)

}
#[test]
fn test_legal_moves_banmen_with_kei_7_squares_dst_occupied_self_gote() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((1,6),(1,7,true),None),
		((1,6),(1,7,false),None),
		((2,4),(3,6,true),None),
		((2,4),(3,6,false),None),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][2] = GKei;
	banmen.0[6][1] = GFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)

}
#[test]
fn test_legal_moves_banmen_with_kei_7_squares_dst_occupied_opponent_gote() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((2,4),(3,6,true),None),
		((2,4),(3,6,false),None),
		((2,4),(1,6,true),Some(ObtainKind::Fu)),
		((2,4),(1,6,false),Some(ObtainKind::Fu)),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][2] = GKei;
	banmen.0[6][1] = SFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)

}
#[test]
fn test_legal_moves_banmen_with_kei_8_squares_dst_occupied_self_gote() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((2,6),(2,7,true),None),
		((2,6),(2,7,false),None),
		((1,4),(0,6,true),None),
		((1,4),(0,6,false),None),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][1] = GKei;
	banmen.0[6][2] = GFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)

}
#[test]
fn test_legal_moves_banmen_with_kei_8_squares_dst_occupied_opponent_gote() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((1,4),(2,6,true),Some(ObtainKind::Fu)),
		((1,4),(2,6,false),Some(ObtainKind::Fu)),
		((1,4),(0,6,true),None),
		((1,4),(0,6,false),None),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[4][1] = GKei;
	banmen.0[6][2] = SFu;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)
}
#[test]
fn test_legal_moves_banmen_with_gin_corner_sente() {
	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![
			((0,0),(1,1,true),None),
			((0,0),(1,1,false),None)
		],
		vec![
			((0,1),(0,0,true),None),
			((0,1),(0,0,false),None),
			((0,1),(1,0,true),None),
			((0,1),(1,0,false),None),
			((0,1),(1,2,true),None),
			((0,1),(1,2,false),None)
		],
		vec![
			((1,0),(0,1,true),None),
			((1,0),(0,1,false),None),
			((1,0),(2,1,true),None),
			((1,0),(2,1,false),None)
		],
		vec![
			((1,1),(0,0,true),None),
			((1,1),(0,0,false),None),
			((1,1),(0,2,true),None),
			((1,1),(0,2,false),None),
			((1,1),(1,0,true),None),
			((1,1),(1,0,false),None),
			((1,1),(2,0,true),None),
			((1,1),(2,0,false),None),
			((1,1),(2,2,true),None),
			((1,1),(2,2,false),None)
		],
		vec![
			((8,0),(7,1,true),None),
			((8,0),(7,1,false),None)
		],
		vec![
			((8,1),(7,0,true),None),
			((8,1),(7,0,false),None),
			((8,1),(7,2,true),None),
			((8,1),(7,2,false),None),
			((8,1),(8,0,true),None),
			((8,1),(8,0,false),None)
		],
		vec![
			((7,0),(6,1,true),None),
			((7,0),(6,1,false),None),
			((7,0),(8,1,true),None),
			((7,0),(8,1,false),None)
		],
		vec![
			((7,1),(6,0,true),None),
			((7,1),(6,0,false),None),
			((7,1),(6,2,true),None),
			((7,1),(6,2,false),None),
			((7,1),(7,0,true),None),
			((7,1),(7,0,false),None),
			((7,1),(8,0,true),None),
			((7,1),(8,0,false),None),
			((7,1),(8,2,true),None),
			((7,1),(8,2,false),None)
		],
		vec![
			((0,8),(0,7,false),None),
			((0,8),(1,7,false),None)
		],
		vec![
			((0,7),(0,6,false),None),
			((0,7),(1,6,false),None),
			((0,7),(1,8,false),None)
		],
		vec![
			((1,8),(0,7,false),None),
			((1,8),(1,7,false),None),
			((1,8),(2,7,false),None)
		],
		vec![
			((1,7),(0,6,false),None),
			((1,7),(0,8,false),None),
			((1,7),(1,6,false),None),
			((1,7),(2,6,false),None),
			((1,7),(2,8,false),None)
		],
		vec![
			((8,8),(7,7,false),None),
			((8,8),(8,7,false),None)
		],
		vec![
			((8,7),(7,6,false),None),
			((8,7),(7,8,false),None),
			((8,7),(8,6,false),None)
		],
		vec![
			((7,8),(6,7,false),None),
			((7,8),(7,7,false),None),
			((7,8),(8,7,false),None)
		],
		vec![
			((7,7),(6,6,false),None),
			((7,7),(6,8,false),None),
			((7,7),(7,6,false),None),
			((7,7),(8,6,false),None),
			((7,7),(8,8,false),None)
		]
	];

	const POSITIONS:[(usize,usize); 16] = [
		(0,0),(0,1),(1,0),(1,1),
		(8,0),(8,1),(7,0),(7,1),
		(0,8),(0,7),(1,8),(1,7),
		(8,8),(8,7),(7,8),(7,7)
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SGin;

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}

}
#[test]
fn test_legal_moves_banmen_with_gin_corner_gote() {
	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![
			((0,0),(1,1,true),None),
			((0,0),(1,1,false),None)
		],
		vec![
			((0,1),(0,0,true),None),
			((0,1),(0,0,false),None),
			((0,1),(1,0,true),None),
			((0,1),(1,0,false),None),
			((0,1),(1,2,true),None),
			((0,1),(1,2,false),None)
		],
		vec![
			((1,0),(0,1,true),None),
			((1,0),(0,1,false),None),
			((1,0),(2,1,true),None),
			((1,0),(2,1,false),None)
		],
		vec![
			((1,1),(0,0,true),None),
			((1,1),(0,0,false),None),
			((1,1),(0,2,true),None),
			((1,1),(0,2,false),None),
			((1,1),(1,0,true),None),
			((1,1),(1,0,false),None),
			((1,1),(2,0,true),None),
			((1,1),(2,0,false),None),
			((1,1),(2,2,true),None),
			((1,1),(2,2,false),None)
		],
		vec![
			((8,0),(7,1,true),None),
			((8,0),(7,1,false),None)
		],
		vec![
			((8,1),(7,0,true),None),
			((8,1),(7,0,false),None),
			((8,1),(7,2,true),None),
			((8,1),(7,2,false),None),
			((8,1),(8,0,true),None),
			((8,1),(8,0,false),None)
		],
		vec![
			((7,0),(6,1,true),None),
			((7,0),(6,1,false),None),
			((7,0),(8,1,true),None),
			((7,0),(8,1,false),None)
		],
		vec![
			((7,1),(6,0,true),None),
			((7,1),(6,0,false),None),
			((7,1),(6,2,true),None),
			((7,1),(6,2,false),None),
			((7,1),(7,0,true),None),
			((7,1),(7,0,false),None),
			((7,1),(8,0,true),None),
			((7,1),(8,0,false),None),
			((7,1),(8,2,true),None),
			((7,1),(8,2,false),None)
		],
		vec![
			((0,8),(0,7,false),None),
			((0,8),(1,7,false),None)
		],
		vec![
			((0,7),(0,6,false),None),
			((0,7),(1,6,false),None),
			((0,7),(1,8,false),None)
		],
		vec![
			((1,8),(0,7,false),None),
			((1,8),(1,7,false),None),
			((1,8),(2,7,false),None)
		],
		vec![
			((1,7),(0,6,false),None),
			((1,7),(0,8,false),None),
			((1,7),(1,6,false),None),
			((1,7),(2,6,false),None),
			((1,7),(2,8,false),None)
		],
		vec![
			((8,8),(7,7,false),None),
			((8,8),(8,7,false),None)
		],
		vec![
			((8,7),(7,6,false),None),
			((8,7),(7,8,false),None),
			((8,7),(8,6,false),None)
		],
		vec![
			((7,8),(6,7,false),None),
			((7,8),(7,7,false),None),
			((7,8),(8,7,false),None)
		],
		vec![
			((7,7),(6,6,false),None),
			((7,7),(6,8,false),None),
			((7,7),(7,6,false),None),
			((7,7),(8,6,false),None),
			((7,7),(8,8,false),None)
		]
	];

	const POSITIONS:[(usize,usize); 16] = [
		(0,0),(0,1),(1,0),(1,1),
		(8,0),(8,1),(7,0),(7,1),
		(0,8),(0,7),(1,8),(1,7),
		(8,8),(8,7),(7,8),(7,7)
	];


	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),_) => {
					LegalMove::from(((8 - sx, 8 - sy),(8- dx, 8 - dy, nari),None))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();


	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-p.1][8-p.0] = GGin;

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}

}
#[test]
fn test_legal_moves_banmen_with_gin_nari_border_sente() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((4,3),(3,2,true),Some(ObtainKind::Fu)),
		((4,3),(3,2,false),Some(ObtainKind::Fu)),
		((4,3),(3,4,false),None),
		((4,3),(4,2,true),Some(ObtainKind::Fu)),
		((4,3),(4,2,false),Some(ObtainKind::Fu)),
		((4,3),(5,2,true),Some(ObtainKind::Fu)),
		((4,3),(5,2,false),Some(ObtainKind::Fu)),
		((4,3),(5,4,false),None),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((sx,sy),(dx,dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 0..9 {
		wall_banmen.0[2][x] = GFu;
	}

	let mut banmen = wall_banmen.clone();

	banmen.0[3][4] = SGin;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_border_gote() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((4,3),(3,2,true),Some(ObtainKind::Fu)),
		((4,3),(3,2,false),Some(ObtainKind::Fu)),
		((4,3),(3,4,false),None),
		((4,3),(4,2,true),Some(ObtainKind::Fu)),
		((4,3),(4,2,false),Some(ObtainKind::Fu)),
		((4,3),(5,2,true),Some(ObtainKind::Fu)),
		((4,3),(5,2,false),Some(ObtainKind::Fu)),
		((4,3),(5,4,false),None),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			((sx,sy),(dx,dy,nari),o) => {
				LegalMove::from(((8 - sx,8 - sy),(8 - dx,8 - dy,nari),o))
			}
		}
	}).collect::<Vec<LegalMove>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut wall_banmen = blank_banmen.clone();

	for x in 0..9 {
		wall_banmen.0[6][x] = SFu;
	}

	let mut banmen = wall_banmen.clone();

	banmen.0[5][4] = GGin;

	assert_eq!(
		answer,
		Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	)
}
#[test]
fn test_legal_moves_banmen_with_gin_7_squares_sente() {
	const OFFSETS:[(i32,i32); 5] = [
		(-1,-1),(-1,1),(0,-1),(1,-1),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

		for o in &OFFSETS {
			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 6 + o.0 < 0 || 6 + o.0 > 8 {
				continue;
			}

			let nari = (y + o.1) <= 2;

			if nari {
				answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,true),None));
			}
			answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,false),None));
		}

		let answer = answer.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[y as usize][6] = SGin;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_gin_8_squares_sente() {
	const OFFSETS:[(i32,i32); 5] = [
		(-1,-1),(-1,1),(0,-1),(1,-1),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

		for o in &OFFSETS {
			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 7 + o.0 < 0 || 7 + o.0 > 8 {
				continue;
			}

			let nari = (y + o.1) <= 2;

			if nari {
				answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,true),None));
			}
			answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,false),None));
		}

		let answer = answer.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[y as usize][7] = SGin;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_gin_7_squares_gote() {
	const OFFSETS:[(i32,i32); 5] = [
		(-1,-1),(-1,1),(0,-1),(1,-1),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

		for o in &OFFSETS {
			if y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 2 - o.0 < 0 || 2 - o.0 > 8 {
				continue;
			}

			let nari = (y - o.1) >= 6;

			if nari {
				answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,true),None));
			}
			answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,false),None));
		}

		let answer = answer.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[y as usize][2] = GGin;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_gin_8_squares_gote() {
	const OFFSETS:[(i32,i32); 5] = [
		(-1,-1),(-1,1),(0,-1),(1,-1),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

		for o in &OFFSETS {
			if y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 1 - o.0 < 0 || 1 - o.0 > 8 {
				continue;
			}

			let nari = (y - o.1) >= 6;

			if nari {
				answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,true),None));
			}
			answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,false),None));
		}

		let answer = answer.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[y as usize][1] = GGin;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_gin_7_squares_and_contiguous_self_sente() {
	const OFFSETS:[(i32,i32); 5] = [
		(-1,-1),(-1,1),(0,-1),(1,-1),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 2] = [
		(1,-1),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();
			if occ.1 > 0 {
				for o in &OFFSETS {
					if occ == o || y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 6 + o.0 < 0 || 6 + o.0 > 8 {
						continue;
					}

					let nari = (y + o.1) <= 2;

					if nari {
						answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,true),None));
					}
					answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,false),None));
				}

				if y + occ.1 <= 8 {
					let nari = (y + occ.1 - 1) <= 2;

					if nari {
						answer.push((
							((6 + occ.0) as u32,(y + occ.1) as u32),
							((6 + occ.0) as u32,(y + occ.1 - 1) as u32,true),None));
					}

					if y + occ.1 - 1 > 0 {
						answer.push((
								((6 + occ.0) as u32,(y + occ.1) as u32),
								((6 + occ.0) as u32,(y + occ.1 - 1) as u32,false),None));
					}
				}
			} else {
				if y + occ.1 >= 1 {
					let nari = (y + occ.1 - 1) <= 2;

					if nari {
						answer.push((
							((6 + occ.0) as u32,(y + occ.1) as u32),
							((6 + occ.0) as u32,(y + occ.1 - 1) as u32,true),None));
					}

					if y + occ.1 - 1 > 0 {
						answer.push((
								((6 + occ.0) as u32,(y + occ.1) as u32),
								((6 + occ.0) as u32,(y + occ.1 - 1) as u32,false),None));
					}
				}

				for o in &OFFSETS {
					if occ == o || y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 6 + o.0 < 0 || 6 + o.0 > 8 {
						continue;
					}

					let nari = (y + o.1) <= 2;

					if nari {
						answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,true),None));
					}
					answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,false),None));
				}
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y + occ.1 >= 0 && y + occ.1 <= 8 {
				banmen.0[(y + occ.1) as usize][(6 + occ.0) as usize] = SFu;
			}

			banmen.0[y as usize][6] = SGin;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_gin_8_squares_and_contiguous_self_sente() {
	const OFFSETS:[(i32,i32); 5] = [
		(-1,-1),(-1,1),(0,-1),(1,-1),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 2] = [
		(-1,-1),(-1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();
			if occ.1 > 0 {
				for o in &OFFSETS {
					if occ == o || y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 7 + o.0 < 0 || 7 + o.0 > 8 {
						continue;
					}

					let nari = (y + o.1) <= 2;

					if nari {
						answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,true),None));
					}
					answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,false),None));
				}

				if y + occ.1 <= 8 {
					let nari = (y + occ.1 - 1) <= 2;

					if nari {
						answer.push((
							((7 + occ.0) as u32,(y + occ.1) as u32),
							((7 + occ.0) as u32,(y + occ.1 - 1) as u32,true),None));
					}

					if y + occ.1 - 1 > 0 {
						answer.push((
								((7 + occ.0) as u32,(y + occ.1) as u32),
								((7 + occ.0) as u32,(y + occ.1 - 1) as u32,false),None));
					}
				}
			} else {
				if y + occ.1 >= 1 {
					let nari = (y + occ.1 - 1) <= 2;

					if nari {
						answer.push((
							((7 + occ.0) as u32,(y + occ.1) as u32),
							((7 + occ.0) as u32,(y + occ.1 - 1) as u32,true),None));
					}

					if y + occ.1 - 1 > 0 {
						answer.push((
								((7 + occ.0) as u32,(y + occ.1) as u32),
								((7 + occ.0) as u32,(y + occ.1 - 1) as u32,false),None));
					}
				}

				for o in &OFFSETS {
					if occ == o || y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 7 + o.0 < 0 || 7 + o.0 > 8 {
						continue;
					}

					let nari = (y + o.1) <= 2;

					if nari {
						answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,true),None));
					}
					answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,false),None));
				}
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y + occ.1 >= 0 && y + occ.1 <= 8 {
				banmen.0[(y + occ.1) as usize][(7 + occ.0) as usize] = SFu;
			}

			banmen.0[y as usize][7] = SGin;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_gin_7_squares_and_contiguous_self_gote() {
	const OFFSETS:[(i32,i32); 5] = [
		(-1,-1),(-1,1),(0,-1),(1,-1),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 2] = [
		(1,-1),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			if occ.1 > 0 {
				for o in &OFFSETS {
					if occ == o || y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 2 - o.0 < 0 || 2 - o.0 > 8 {
						continue;
					}

					let nari = (y - o.1) >= 6;

					if nari {
						answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,true),None));
					}
					answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,false),None));
				}

				if y as i32 - occ.1 >=  0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6;

					if nari {
						answer.push((
							((2 - occ.0) as u32,(y - occ.1) as u32),
							((2 - occ.0) as u32,(y - occ.1 + 1) as u32,true),None));
					}

					if y - occ.1 + 1 < 8 {
						answer.push((
								((2 - occ.0) as u32,(y - occ.1) as u32),
								((2 - occ.0) as u32,(y - occ.1 + 1) as u32,false),None));
					}
				}
			} else {
				if y as i32 - occ.1 >=  0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6;

					if nari {
						answer.push((
							((2 - occ.0) as u32,(y - occ.1) as u32),
							((2 - occ.0) as u32,(y - occ.1 + 1) as u32,true),None));
					}

					if y - occ.1 + 1 < 8 {
						answer.push((
								((2 - occ.0) as u32,(y - occ.1) as u32),
								((2 - occ.0) as u32,(y - occ.1 + 1) as u32,false),None));
					}
				}

				for o in &OFFSETS {
					if occ == o || y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 2 - o.0 < 0 || 2 - o.0 > 8 {
						continue;
					}

					let nari = (y - o.1) >= 6;

					if nari {
						answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,true),None));
					}
					answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,false),None));
				}
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y - occ.1 >= 0 && y - occ.1 <= 8 {
				banmen.0[(y - occ.1) as usize][(2 - occ.0) as usize] = GFu;
			}

			banmen.0[y as usize][2] = GGin;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_gin_8_squares_and_contiguous_self_gote() {
	const OFFSETS:[(i32,i32); 5] = [
		(-1,-1),(-1,1),(0,-1),(1,-1),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 2] = [
		(-1,-1),(-1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			if occ.1 > 0 {
				for o in &OFFSETS {
					if occ == o || y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 1 - o.0 < 0 || 1 - o.0 > 8 {
						continue;
					}

					let nari = (y - o.1) >= 6;

					if nari {
						answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,true),None));
					}
					answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,false),None));
				}

				if y as i32 - occ.1 >=  0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6;

					if nari {
						answer.push((
							((1 - occ.0) as u32,(y - occ.1) as u32),
							((1 - occ.0) as u32,(y - occ.1 + 1) as u32,true),None));
					}

					if y - occ.1 + 1 < 8 {
						answer.push((
								((1 - occ.0) as u32,(y - occ.1) as u32),
								((1 - occ.0) as u32,(y - occ.1 + 1) as u32,false),None));
					}
				}
			} else {
				if y as i32 - occ.1 >=  0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6;

					if nari {
						answer.push((
							((1 - occ.0) as u32,(y - occ.1) as u32),
							((1 - occ.0) as u32,(y - occ.1 + 1) as u32,true),None));
					}

					if y - occ.1 + 1 < 8 {
						answer.push((
								((1 - occ.0) as u32,(y - occ.1) as u32),
								((1 - occ.0) as u32,(y - occ.1 + 1) as u32,false),None));
					}
				}

				for o in &OFFSETS {
					if occ == o || y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 1 - o.0 < 0 || 1 - o.0 > 8 {
						continue;
					}

					let nari = (y - o.1) >= 6;

					if nari {
						answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,true),None));
					}
					answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,false),None));
				}
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y - occ.1 >= 0 && y - occ.1 <= 8 {
				banmen.0[(y - occ.1) as usize][(1 - occ.0) as usize] = GFu;
			}

			banmen.0[y as usize][1] = GGin;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_gin_7_squares_and_contiguous_opponent_sente() {
	const OFFSETS:[(i32,i32); 5] = [
		(-1,-1),(-1,1),(0,-1),(1,-1),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 2] = [
		(1,-1),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();
			for o in &OFFSETS {
				if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 6 + o.0 < 0 || 6 + o.0 > 8 {
					continue;
				}

				let nari = (y + o.1) <= 2;

				let obtained = if o == occ {
					Some(ObtainKind::Fu)
				} else {
					None
				};

				if nari {
					answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,true),obtained));
				}
				answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,false),obtained));
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y + occ.1 >= 0 && y + occ.1 <= 8 {
				banmen.0[(y + occ.1) as usize][(6 + occ.0) as usize] = GFu;
			}

			banmen.0[y as usize][6] = SGin;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_gin_8_squares_and_contiguous_opponent_sente() {
	const OFFSETS:[(i32,i32); 5] = [
		(-1,-1),(-1,1),(0,-1),(1,-1),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 2] = [
		(-1,-1),(-1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			for o in &OFFSETS {
				if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 7 + o.0 < 0 || 7 + o.0 > 8 {
					continue;
				}

				let nari = (y + o.1) <= 2;

				let obtained = if o == occ {
					Some(ObtainKind::Fu)
				} else {
					None
				};

				if nari {
					answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,true),obtained));
				}
				answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,false),obtained));
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y + occ.1 >= 0 && y + occ.1 <= 8 {
				banmen.0[(y + occ.1) as usize][(7 + occ.0) as usize] = GFu;
			}

			banmen.0[y as usize][7] = SGin;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_gin_7_squares_and_contiguous_opponent_gote() {
	const OFFSETS:[(i32,i32); 5] = [
		(-1,-1),(-1,1),(0,-1),(1,-1),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 2] = [
		(1,-1),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			for o in &OFFSETS {
				if y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 2 - o.0 < 0 || 2 - o.0 > 8 {
					continue;
				}

				let nari = (y - o.1) >= 6;

				let obtained = if o == occ {
					Some(ObtainKind::Fu)
				} else {
					None
				};

				if nari {
					answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,true),obtained));
				}
				answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,false),obtained));
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y - occ.1 >= 0 && y - occ.1 <= 8 {
				banmen.0[(y - occ.1) as usize][(2 - occ.0) as usize] = SFu;
			}

			banmen.0[y as usize][2] = GGin;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_gin_8_squares_and_contiguous_opponent_gote() {
	const OFFSETS:[(i32,i32); 5] = [
		(-1,-1),(-1,1),(0,-1),(1,-1),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 2] = [
		(-1,-1),(-1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			for o in &OFFSETS {
				if y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 1 - o.0 < 0 || 1 - o.0 > 8 {
					continue;
				}

				let nari = (y - o.1) >= 6;

				let obtained = if o == occ {
					Some(ObtainKind::Fu)
				} else {
					None
				};

				if nari {
					answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,true),obtained));
				}
				answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,false),obtained));
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y - occ.1 >= 0 && y - occ.1 <= 8 {
				banmen.0[(y - occ.1) as usize][(1 - occ.0) as usize] = SFu;
			}

			banmen.0[y as usize][1] = GGin;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
fn test_legal_moves_banmen_with_kin_corner_sente_impl(kind:KomaKind) {
	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![
			((0,0),(0,1,false),None),
			((0,0),(1,0,false),None)
		],
		vec![
			((0,1),(0,0,false),None),
			((0,1),(0,2,false),None),
			((0,1),(1,0,false),None),
			((0,1),(1,1,false),None)
		],
		vec![
			((1,0),(0,0,false),None),
			((1,0),(1,1,false),None),
			((1,0),(2,0,false),None)
		],
		vec![
			((1,1),(0,0,false),None),
			((1,1),(0,1,false),None),
			((1,1),(1,0,false),None),
			((1,1),(1,2,false),None),
			((1,1),(2,0,false),None),
			((1,1),(2,1,false),None)
		],
		vec![
			((8,0),(7,0,false),None),
			((8,0),(8,1,false),None)
		],
		vec![
			((8,1),(7,0,false),None),
			((8,1),(7,1,false),None),
			((8,1),(8,0,false),None),
			((8,1),(8,2,false),None)
		],
		vec![
			((7,0),(6,0,false),None),
			((7,0),(7,1,false),None),
			((7,0),(8,0,false),None),
		],
		vec![
			((7,1),(6,0,false),None),
			((7,1),(6,1,false),None),
			((7,1),(7,0,false),None),
			((7,1),(7,2,false),None),
			((7,1),(8,0,false),None),
			((7,1),(8,1,false),None),
		],
		vec![
			((0,8),(0,7,false),None),
			((0,8),(1,7,false),None),
			((0,8),(1,8,false),None)
		],
		vec![
			((0,7),(0,6,false),None),
			((0,7),(0,8,false),None),
			((0,7),(1,6,false),None),
			((0,7),(1,7,false),None),
		],
		vec![
			((1,8),(0,7,false),None),
			((1,8),(0,8,false),None),
			((1,8),(1,7,false),None),
			((1,8),(2,7,false),None),
			((1,8),(2,8,false),None)
		],
		vec![
			((1,7),(0,6,false),None),
			((1,7),(0,7,false),None),
			((1,7),(1,6,false),None),
			((1,7),(1,8,false),None),
			((1,7),(2,6,false),None),
			((1,7),(2,7,false),None),
		],
		vec![
			((8,8),(7,7,false),None),
			((8,8),(7,8,false),None),
			((8,8),(8,7,false),None),
		],
		vec![
			((8,7),(7,6,false),None),
			((8,7),(7,7,false),None),
			((8,7),(8,6,false),None),
			((8,7),(8,8,false),None)
		],
		vec![
			((7,8),(6,7,false),None),
			((7,8),(6,8,false),None),
			((7,8),(7,7,false),None),
			((7,8),(8,7,false),None),
			((7,8),(8,8,false),None)
		],
		vec![
			((7,7),(6,6,false),None),
			((7,7),(6,7,false),None),
			((7,7),(7,6,false),None),
			((7,7),(7,8,false),None),
			((7,7),(8,6,false),None),
			((7,7),(8,7,false),None),
		]
	];

	const POSITIONS:[(usize,usize); 16] = [
		(0,0),(0,1),(1,0),(1,1),
		(8,0),(8,1),(7,0),(7,1),
		(0,8),(0,7),(1,8),(1,7),
		(8,8),(8,7),(7,8),(7,7)
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = kind;

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}

}
fn test_legal_moves_banmen_with_kin_corner_gote_impl(kind:KomaKind) {
	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![
			((0,0),(0,1,false),None),
			((0,0),(1,0,false),None),
		],
		vec![
			((0,1),(0,0,false),None),
			((0,1),(0,2,false),None),
			((0,1),(1,0,false),None),
			((0,1),(1,1,false),None),
		],
		vec![
			((1,0),(0,0,false),None),
			((1,0),(1,1,false),None),
			((1,0),(2,0,false),None),
		],
		vec![
			((1,1),(0,0,false),None),
			((1,1),(0,1,false),None),
			((1,1),(1,0,false),None),
			((1,1),(1,2,false),None),
			((1,1),(2,0,false),None),
			((1,1),(2,1,false),None),
		],
		vec![
			((8,0),(7,0,false),None),
			((8,0),(8,1,false),None)
		],
		vec![
			((8,1),(7,0,false),None),
			((8,1),(7,1,false),None),
			((8,1),(8,0,false),None),
			((8,1),(8,2,false),None)
		],
		vec![
			((7,0),(6,0,false),None),
			((7,0),(7,1,false),None),
			((7,0),(8,0,false),None),
		],
		vec![
			((7,1),(6,0,false),None),
			((7,1),(6,1,false),None),
			((7,1),(7,0,false),None),
			((7,1),(7,2,false),None),
			((7,1),(8,0,false),None),
			((7,1),(8,1,false),None),
		],
		vec![
			((0,8),(0,7,false),None),
			((0,8),(1,7,false),None),
			((0,8),(1,8,false),None)
		],
		vec![
			((0,7),(0,6,false),None),
			((0,7),(0,8,false),None),
			((0,7),(1,6,false),None),
			((0,7),(1,7,false),None),
		],
		vec![
			((1,8),(0,7,false),None),
			((1,8),(0,8,false),None),
			((1,8),(1,7,false),None),
			((1,8),(2,7,false),None),
			((1,8),(2,8,false),None)
		],
		vec![
			((1,7),(0,6,false),None),
			((1,7),(0,7,false),None),
			((1,7),(1,6,false),None),
			((1,7),(1,8,false),None),
			((1,7),(2,6,false),None),
			((1,7),(2,7,false),None),
		],
		vec![
			((8,8),(7,7,false),None),
			((8,8),(7,8,false),None),
			((8,8),(8,7,false),None),
		],
		vec![
			((8,7),(7,6,false),None),
			((8,7),(7,7,false),None),
			((8,7),(8,6,false),None),
			((8,7),(8,8,false),None)
		],
		vec![
			((7,8),(6,7,false),None),
			((7,8),(6,8,false),None),
			((7,8),(7,7,false),None),
			((7,8),(8,7,false),None),
			((7,8),(8,8,false),None)
		],
		vec![
			((7,7),(6,6,false),None),
			((7,7),(6,7,false),None),
			((7,7),(7,6,false),None),
			((7,7),(7,8,false),None),
			((7,7),(8,6,false),None),
			((7,7),(8,7,false),None),
		]
	];

	const POSITIONS:[(usize,usize); 16] = [
		(0,0),(0,1),(1,0),(1,1),
		(8,0),(8,1),(7,0),(7,1),
		(0,8),(0,7),(1,8),(1,7),
		(8,8),(8,7),(7,8),(7,7)
	];


	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),_) => {
					LegalMove::from(((8 - sx, 8 - sy),(8- dx, 8 - dy, nari),None))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();


	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-p.1][8-p.0] = kind;

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
fn test_legal_moves_banmen_with_kin_7_squares_sente_impl(kind:KomaKind) {
	const OFFSETS:[(i32,i32); 6] = [
		(-1,-1),(-1,0),(0,-1),(0,1),(1,-1),(1,0)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

		for o in &OFFSETS {
			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 6 + o.0 < 0 || 6 + o.0 > 8 {
				continue;
			}
			answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,false),None));
		}

		let answer = answer.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[y as usize][6] = kind;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
fn test_legal_moves_banmen_with_kin_8_squares_sente_impl(kind:KomaKind) {
	const OFFSETS:[(i32,i32); 6] = [
		(-1,-1),(-1,0),(0,-1),(0,1),(1,-1),(1,0)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

		for o in &OFFSETS {
			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 7 + o.0 < 0 || 7 + o.0 > 8 {
				continue;
			}
			answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,false),None));
		}

		let answer = answer.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[y as usize][7] = kind;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
fn test_legal_moves_banmen_with_kin_7_squares_gote_impl(kind:KomaKind) {
	const OFFSETS:[(i32,i32); 6] = [
		(-1,-1),(-1,0),(0,-1),(0,1),(1,-1),(1,0)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

		for o in &OFFSETS {
			if y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 2 - o.0 < 0 || 2 - o.0 > 8 {
				continue;
			}
			answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,false),None));
		}

		let answer = answer.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[y as usize][2] = kind;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
fn test_legal_moves_banmen_with_kin_8_squares_gote_impl(kind:KomaKind) {
	const OFFSETS:[(i32,i32); 6] = [
		(-1,-1),(-1,0),(0,-1),(0,1),(1,-1),(1,0)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

		for o in &OFFSETS {
			if y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 1 - o.0 < 0 || 1 - o.0 > 8 {
				continue;
			}
			answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,false),None));
		}

		let answer = answer.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[y as usize][1] = kind;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
fn test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_sente_impl(kind:KomaKind) {
	const OFFSETS:[(i32,i32); 6] = [
		(-1,-1),(-1,0),(0,-1),(0,1),(1,-1),(1,0)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(1,-1),(1,0),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();
			if occ.1 >= 0 {
				for o in &OFFSETS {
					if occ == o || y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 6 + o.0 < 0 || 6 + o.0 > 8 {
						continue;
					}
					answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,false),None));
				}

				if y + occ.1 <= 8 && y + occ.1 >= 1 {
					let nari = (y + occ.1 - 1) <= 2;

					if nari {
						answer.push((
							((6 + occ.0) as u32,(y + occ.1) as u32),
							((6 + occ.0) as u32,(y + occ.1 - 1) as u32,true),None));
					}

					if y + occ.1 - 1 > 0 {
						answer.push((
								((6 + occ.0) as u32,(y + occ.1) as u32),
								((6 + occ.0) as u32,(y + occ.1 - 1) as u32,false),None));
					}
				}
			} else {
				if y + occ.1 >= 1 && y + occ.1 >= 1 {
					let nari = (y + occ.1 - 1) <= 2;

					if nari {
						answer.push((
							((6 + occ.0) as u32,(y + occ.1) as u32),
							((6 + occ.0) as u32,(y + occ.1 - 1) as u32,true),None));
					}

					if y + occ.1 - 1 > 0 {
						answer.push((
								((6 + occ.0) as u32,(y + occ.1) as u32),
								((6 + occ.0) as u32,(y + occ.1 - 1) as u32,false),None));
					}
				}

				for o in &OFFSETS {
					if occ == o || y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 6 + o.0 < 0 || 6 + o.0 > 8 {
						continue;
					}
					answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,false),None));
				}
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y + occ.1 >= 0 && y + occ.1 <= 8 {
				banmen.0[(y + occ.1) as usize][(6 + occ.0) as usize] = SFu;
			}

			banmen.0[y as usize][6] = kind;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
fn test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_sente_impl(kind:KomaKind) {
	const OFFSETS:[(i32,i32); 6] = [
		(-1,-1),(-1,0),(0,-1),(0,1),(1,-1),(1,0)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(-1,-1),(-1,0),(-1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();
			if occ.1 > 0 {
				for o in &OFFSETS {
					if occ == o || y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 7 + o.0 < 0 || 7 + o.0 > 8 {
						continue;
					}
					answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,false),None));
				}

				if y + occ.1 <= 8 && y + occ.1 >= 1 {
					let nari = (y + occ.1 - 1) <= 2;

					if nari {
						answer.push((
							((7 + occ.0) as u32,(y + occ.1) as u32),
							((7 + occ.0) as u32,(y + occ.1 - 1) as u32,true),None));
					}

					if y + occ.1 - 1 >0 {
						answer.push((
							((7 + occ.0) as u32,(y + occ.1) as u32),
							((7 + occ.0) as u32,(y + occ.1 - 1) as u32,false),None));
					}
				}
			} else {
				if y + occ.1 >= 1 && y + occ.1 >= 1 {
					let nari = (y + occ.1 - 1) <= 2;

					if nari {
						answer.push((
							((7 + occ.0) as u32,(y + occ.1) as u32),
							((7 + occ.0) as u32,(y + occ.1 - 1) as u32,true),None));
					}

					if y + occ.1 - 1 >0 {
						answer.push((
							((7 + occ.0) as u32,(y + occ.1) as u32),
							((7 + occ.0) as u32,(y + occ.1 - 1) as u32,false),None));
					}
				}

				for o in &OFFSETS {
					if occ == o || y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 7 + o.0 < 0 || 7 + o.0 > 8 {
						continue;
					}
					answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,false),None));
				}
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y + occ.1 >= 0 && y + occ.1 <= 8 {
				banmen.0[(y + occ.1) as usize][(7 + occ.0) as usize] = SFu;
			}

			banmen.0[y as usize][7] = kind;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
fn test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_gote_impl(kind:KomaKind) {
	const OFFSETS:[(i32,i32); 6] = [
		(-1,-1),(-1,0),(0,-1),(0,1),(1,-1),(1,0)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(1,-1),(1,0),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			if occ.1 >= 0 {
				for o in &OFFSETS {
					if occ == o || y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 2 - o.0 < 0 || 2 - o.0 > 8 {
						continue;
					}
					answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,false),None));
				}

				if y as i32 - occ.1 >=  0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6;

					if nari {
						answer.push((
							((2 - occ.0) as u32,(y - occ.1) as u32),
							((2 - occ.0) as u32,(y - occ.1 + 1) as u32,true),None));
					}

					if y - occ.1 + 1 < 8 {
						answer.push((
							((2 - occ.0) as u32,(y - occ.1) as u32),
							((2 - occ.0) as u32,(y - occ.1 + 1) as u32,false),None));
					}
				}
			} else {
				if y as i32 - occ.1 >= 0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6;

					if nari {
						answer.push((
							((2 - occ.0) as u32,(y - occ.1) as u32),
							((2 - occ.0) as u32,(y - occ.1 + 1) as u32,true),None));
					}

					if y - occ.1 + 1 < 8 {
						answer.push((
								((2 - occ.0) as u32,(y - occ.1) as u32),
								((2 - occ.0) as u32,(y - occ.1 + 1) as u32,false),None));
					}
				}

				for o in &OFFSETS {
					if occ == o || y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 2 - o.0 < 0 || 2 - o.0 > 8 {
						continue;
					}
					answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,false),None));
				}
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y - occ.1 >= 0 && y - occ.1 <= 8 {
				banmen.0[(y - occ.1) as usize][(2 - occ.0) as usize] = GFu;
			}

			banmen.0[y as usize][2] = kind;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
fn test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_gote_impl(kind:KomaKind) {
	const OFFSETS:[(i32,i32); 6] = [
		(-1,-1),(-1,0),(0,-1),(0,1),(1,-1),(1,0)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(-1,-1),(-1,0),(-1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			if occ.1 > 0 {
				for o in &OFFSETS {
					if occ == o || y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 1 - o.0 < 0 || 1 - o.0 > 8 {
						continue;
					}
					answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,false),None));
				}

				if y as i32 - occ.1 >=  0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6;

					if nari {
						answer.push((
							((1 - occ.0) as u32,(y - occ.1) as u32),
							((1 - occ.0) as u32,(y - occ.1 + 1) as u32,true),None));
					}

					if y - occ.1 + 1 < 8 {
						answer.push((
								((1 - occ.0) as u32,(y - occ.1) as u32),
								((1 - occ.0) as u32,(y - occ.1 + 1) as u32,false),None));
					}
				}
			} else {
				if y as i32 - occ.1 >= 0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6;

					if nari {
						answer.push((
							((1 - occ.0) as u32,(y - occ.1) as u32),
							((1 - occ.0) as u32,(y - occ.1 + 1) as u32,true),None));
					}

					if y - occ.1 + 1 < 8 {
						answer.push((
								((1 - occ.0) as u32,(y - occ.1) as u32),
								((1 - occ.0) as u32,(y - occ.1 + 1) as u32,false),None));
					}
				}

				for o in &OFFSETS {
					if occ == o || y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 1 - o.0 < 0 || 1 - o.0 > 8 {
						continue;
					}
					answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,false),None));
				}
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y - occ.1 >= 0 && y - occ.1 <= 8 {
				banmen.0[(y - occ.1) as usize][(1 - occ.0) as usize] = GFu;
			}

			banmen.0[y as usize][1] = kind;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
fn test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_sente_impl(kind:KomaKind) {
	const OFFSETS:[(i32,i32); 6] = [
		(-1,-1),(-1,0),(0,-1),(0,1),(1,-1),(1,0)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(1,1),(1,0),(1,-1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();
			for o in &OFFSETS {
				if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 6 + o.0 < 0 || 6 + o.0 > 8 {
					continue;
				}

				let obtained = if o == occ {
					Some(ObtainKind::Fu)
				} else {
					None
				};
				answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,false),obtained));
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y + occ.1 >= 0 && y + occ.1 <= 8 {
				banmen.0[(y + occ.1) as usize][(6 + occ.0) as usize] = GFu;
			}

			banmen.0[y as usize][6] = kind;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
fn test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_sente_impl(kind:KomaKind) {
	const OFFSETS:[(i32,i32); 6] = [
		(-1,-1),(-1,0),(0,-1),(0,1),(1,-1),(1,0)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(-1,-1),(-1,0),(-1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			for o in &OFFSETS {
				if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 7 + o.0 < 0 || 7 + o.0 > 8 {
					continue;
				}

				let obtained = if o == occ {
					Some(ObtainKind::Fu)
				} else {
					None
				};
				answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,false),obtained));
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y + occ.1 >= 0 && y + occ.1 <= 8 {
				banmen.0[(y + occ.1) as usize][(7 + occ.0) as usize] = GFu;
			}

			banmen.0[y as usize][7] = kind;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
fn test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_gote_impl(kind:KomaKind) {
	const OFFSETS:[(i32,i32); 6] = [
		(-1,-1),(-1,0),(0,-1),(0,1),(1,-1),(1,0)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(1,-1),(1,0),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			for o in &OFFSETS {
				if y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 2 - o.0 < 0 || 2 - o.0 > 8 {
					continue;
				}

				let obtained = if o == occ {
					Some(ObtainKind::Fu)
				} else {
					None
				};
				answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,false),obtained));
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y - occ.1 >= 0 && y - occ.1 <= 8 {
				banmen.0[(y - occ.1) as usize][(2 - occ.0) as usize] = SFu;
			}

			banmen.0[y as usize][2] = kind;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
fn test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_gote_impl(kind:KomaKind) {
	const OFFSETS:[(i32,i32); 6] = [
		(-1,-1),(-1,0),(0,-1),(0,1),(1,-1),(1,0)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(-1,1),(-1,0),(-1,-1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			for o in &OFFSETS {
				if y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 1 - o.0 < 0 || 1 - o.0 > 8 {
					continue;
				}

				let obtained = if o == occ {
					Some(ObtainKind::Fu)
				} else {
					None
				};
				answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,false),obtained));
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y - occ.1 >= 0 && y - occ.1 <= 8 {
				banmen.0[(y - occ.1) as usize][(1 - occ.0) as usize] = SFu;
			}

			banmen.0[y as usize][1] = kind;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kin_corner_sente() {
	test_legal_moves_banmen_with_kin_corner_sente_impl(SKin)
}
#[test]
fn test_legal_moves_banmen_with_kin_corner_gote() {
	test_legal_moves_banmen_with_kin_corner_gote_impl(GKin)
}
#[test]
fn test_legal_moves_banmen_with_kin_7_squares_sente() {
	test_legal_moves_banmen_with_kin_7_squares_sente_impl(SKin)
}
#[test]
fn test_legal_moves_banmen_with_kin_8_squares_sente() {
	test_legal_moves_banmen_with_kin_8_squares_sente_impl(SKin)
}
#[test]
fn test_legal_moves_banmen_with_kin_7_squares_gote() {
	test_legal_moves_banmen_with_kin_7_squares_gote_impl(GKin)
}
#[test]
fn test_legal_moves_banmen_with_kin_8_squares_gote() {
	test_legal_moves_banmen_with_kin_8_squares_gote_impl(GKin)
}
#[test]
fn test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_sente() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_sente_impl(SKin)
}
#[test]
fn test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_sente() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_sente_impl(SKin)
}
#[test]
fn test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_gote() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_gote_impl(GKin)
}
#[test]
fn test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_gote() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_gote_impl(GKin)
}
#[test]
fn test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_sente() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_sente_impl(SKin)
}
#[test]
fn test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_sente() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_sente_impl(SKin)
}
#[test]
fn test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_gote() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_gote_impl(GKin)
}
#[test]
fn test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_gote() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_gote_impl(GKin)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_corner_sente() {
	test_legal_moves_banmen_with_kin_corner_sente_impl(SFuN)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_corner_gote() {
	test_legal_moves_banmen_with_kin_corner_gote_impl(GFuN)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_7_squares_sente() {
	test_legal_moves_banmen_with_kin_7_squares_sente_impl(SFuN)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_8_squares_sente() {
	test_legal_moves_banmen_with_kin_8_squares_sente_impl(SFuN)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_7_squares_gote() {
	test_legal_moves_banmen_with_kin_7_squares_gote_impl(GFuN)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_8_squares_gote() {
	test_legal_moves_banmen_with_kin_8_squares_gote_impl(GFuN)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_7_squares_and_contiguous_self_sente() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_sente_impl(SFuN)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_8_squares_and_contiguous_self_sente() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_sente_impl(SFuN)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_7_squares_and_contiguous_self_gote() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_gote_impl(GFuN)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_8_squares_and_contiguous_self_gote() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_gote_impl(GFuN)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_7_squares_and_contiguous_opponent_sente() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_sente_impl(SFuN)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_8_squares_and_contiguous_opponent_sente() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_sente_impl(SFuN)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_7_squares_and_contiguous_opponent_gote() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_gote_impl(GFuN)
}
#[test]
fn test_legal_moves_banmen_with_fu_nari_8_squares_and_contiguous_opponent_gote() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_gote_impl(GFuN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_corner_sente() {
	test_legal_moves_banmen_with_kin_corner_sente_impl(SKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_corner_gote() {
	test_legal_moves_banmen_with_kin_corner_gote_impl(GKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_7_squares_sente() {
	test_legal_moves_banmen_with_kin_7_squares_sente_impl(SKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_8_squares_sente() {
	test_legal_moves_banmen_with_kin_8_squares_sente_impl(SKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_7_squares_gote() {
	test_legal_moves_banmen_with_kin_7_squares_gote_impl(GKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_8_squares_gote() {
	test_legal_moves_banmen_with_kin_8_squares_gote_impl(GKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_7_squares_and_contiguous_self_sente() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_sente_impl(SKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_8_squares_and_contiguous_self_sente() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_sente_impl(SKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_7_squares_and_contiguous_self_gote() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_gote_impl(GKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_8_squares_and_contiguous_self_gote() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_gote_impl(GKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_7_squares_and_contiguous_opponent_sente() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_sente_impl(SKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_8_squares_and_contiguous_opponent_sente() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_sente_impl(SKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_7_squares_and_contiguous_opponent_gote() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_gote_impl(GKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kyou_nari_8_squares_and_contiguous_opponent_gote() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_gote_impl(GKyouN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_corner_sente() {
	test_legal_moves_banmen_with_kin_corner_sente_impl(SKeiN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_corner_gote() {
	test_legal_moves_banmen_with_kin_corner_gote_impl(GKeiN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_7_squares_sente() {
	test_legal_moves_banmen_with_kin_7_squares_sente_impl(SKeiN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_8_squares_sente() {
	test_legal_moves_banmen_with_kin_8_squares_sente_impl(SKeiN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_7_squares_gote() {
	test_legal_moves_banmen_with_kin_7_squares_gote_impl(GKeiN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_8_squares_gote() {
	test_legal_moves_banmen_with_kin_8_squares_gote_impl(GKeiN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_7_squares_and_contiguous_self_sente() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_sente_impl(SKeiN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_8_squares_and_contiguous_self_sente() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_sente_impl(SKeiN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_7_squares_and_contiguous_self_gote() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_gote_impl(GKeiN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_8_squares_and_contiguous_self_gote() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_gote_impl(GKeiN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_7_squares_and_contiguous_opponent_sente() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_sente_impl(SKeiN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_8_squares_and_contiguous_opponent_sente() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_sente_impl(SKeiN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_7_squares_and_contiguous_opponent_gote() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_gote_impl(GKeiN)
}
#[test]
fn test_legal_moves_banmen_with_kei_nari_8_squares_and_contiguous_opponent_gote() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_gote_impl(GKeiN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_corner_sente() {
	test_legal_moves_banmen_with_kin_corner_sente_impl(SGinN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_corner_gote() {
	test_legal_moves_banmen_with_kin_corner_gote_impl(GGinN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_7_squares_sente() {
	test_legal_moves_banmen_with_kin_7_squares_sente_impl(SGinN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_8_squares_sente() {
	test_legal_moves_banmen_with_kin_8_squares_sente_impl(SGinN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_7_squares_gote() {
	test_legal_moves_banmen_with_kin_7_squares_gote_impl(GGinN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_8_squares_gote() {
	test_legal_moves_banmen_with_kin_8_squares_gote_impl(GGinN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_7_squares_and_contiguous_self_sente() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_sente_impl(SGinN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_8_squares_and_contiguous_self_sente() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_sente_impl(SGinN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_7_squares_and_contiguous_self_gote() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_self_gote_impl(GGinN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_8_squares_and_contiguous_self_gote() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_self_gote_impl(GGinN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_7_squares_and_contiguous_opponent_sente() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_sente_impl(SGinN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_8_squares_and_contiguous_opponent_sente() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_sente_impl(SGinN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_7_squares_and_contiguous_opponent_gote() {
	test_legal_moves_banmen_with_kin_7_squares_and_contiguous_opponent_gote_impl(GGinN)
}
#[test]
fn test_legal_moves_banmen_with_gin_nari_8_squares_and_contiguous_opponent_gote() {
	test_legal_moves_banmen_with_kin_8_squares_and_contiguous_opponent_gote_impl(GGinN)
}
#[test]
fn test_legal_moves_banmen_with_ou_corner_sente() {
	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![
			((0,0),(0,1,false),None),
			((0,0),(1,0,false),None),
			((0,0),(1,1,false),None)
		],
		vec![
			((0,1),(0,0,false),None),
			((0,1),(0,2,false),None),
			((0,1),(1,0,false),None),
			((0,1),(1,1,false),None),
			((0,1),(1,2,false),None)
		],
		vec![
			((1,0),(0,0,false),None),
			((1,0),(0,1,false),None),
			((1,0),(1,1,false),None),
			((1,0),(2,0,false),None),
			((1,0),(2,1,false),None)
		],
		vec![
			((1,1),(0,0,false),None),
			((1,1),(0,1,false),None),
			((1,1),(0,2,false),None),
			((1,1),(1,0,false),None),
			((1,1),(1,2,false),None),
			((1,1),(2,0,false),None),
			((1,1),(2,1,false),None),
			((1,1),(2,2,false),None)
		],
		vec![
			((8,0),(7,0,false),None),
			((8,0),(7,1,false),None),
			((8,0),(8,1,false),None),
		],
		vec![
			((8,1),(7,0,false),None),
			((8,1),(7,1,false),None),
			((8,1),(7,2,false),None),
			((8,1),(8,0,false),None),
			((8,1),(8,2,false),None),
		],
		vec![
			((7,0),(6,0,false),None),
			((7,0),(6,1,false),None),
			((7,0),(7,1,false),None),
			((7,0),(8,0,false),None),
			((7,0),(8,1,false),None)
		],
		vec![
			((7,1),(6,0,false),None),
			((7,1),(6,1,false),None),
			((7,1),(6,2,false),None),
			((7,1),(7,0,false),None),
			((7,1),(7,2,false),None),
			((7,1),(8,0,false),None),
			((7,1),(8,1,false),None),
			((7,1),(8,2,false),None)
		],
		vec![
			((0,8),(0,7,false),None),
			((0,8),(1,7,false),None),
			((0,8),(1,8,false),None)
		],
		vec![
			((0,7),(0,6,false),None),
			((0,7),(0,8,false),None),
			((0,7),(1,6,false),None),
			((0,7),(1,7,false),None),
			((0,7),(1,8,false),None)
		],
		vec![
			((1,8),(0,7,false),None),
			((1,8),(0,8,false),None),
			((1,8),(1,7,false),None),
			((1,8),(2,7,false),None),
			((1,8),(2,8,false),None)
		],
		vec![
			((1,7),(0,6,false),None),
			((1,7),(0,7,false),None),
			((1,7),(0,8,false),None),
			((1,7),(1,6,false),None),
			((1,7),(1,8,false),None),
			((1,7),(2,6,false),None),
			((1,7),(2,7,false),None),
			((1,7),(2,8,false),None)
		],
		vec![
			((8,8),(7,7,false),None),
			((8,8),(7,8,false),None),
			((8,8),(8,7,false),None),
		],
		vec![
			((8,7),(7,6,false),None),
			((8,7),(7,7,false),None),
			((8,7),(7,8,false),None),
			((8,7),(8,6,false),None),
			((8,7),(8,8,false),None),
		],
		vec![
			((7,8),(6,7,false),None),
			((7,8),(6,8,false),None),
			((7,8),(7,7,false),None),
			((7,8),(8,7,false),None),
			((7,8),(8,8,false),None)
		],
		vec![
			((7,7),(6,6,false),None),
			((7,7),(6,7,false),None),
			((7,7),(6,8,false),None),
			((7,7),(7,6,false),None),
			((7,7),(7,8,false),None),
			((7,7),(8,6,false),None),
			((7,7),(8,7,false),None),
			((7,7),(8,8,false),None)
		]
	];

	const POSITIONS:[(usize,usize); 16] = [
		(0,0),(0,1),(1,0),(1,1),
		(8,0),(8,1),(7,0),(7,1),
		(0,8),(0,7),(1,8),(1,7),
		(8,8),(8,7),(7,8),(7,7)
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SOu;

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}

}
#[test]
fn test_legal_moves_banmen_with_ou_corner_gote() {
	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![
			((0,0),(0,1,false),None),
			((0,0),(1,0,false),None),
			((0,0),(1,1,false),None)
		],
		vec![
			((0,1),(0,0,false),None),
			((0,1),(0,2,false),None),
			((0,1),(1,0,false),None),
			((0,1),(1,1,false),None),
			((0,1),(1,2,false),None)
		],
		vec![
			((1,0),(0,0,false),None),
			((1,0),(0,1,false),None),
			((1,0),(1,1,false),None),
			((1,0),(2,0,false),None),
			((1,0),(2,1,false),None)
		],
		vec![
			((1,1),(0,0,false),None),
			((1,1),(0,1,false),None),
			((1,1),(0,2,false),None),
			((1,1),(1,0,false),None),
			((1,1),(1,2,false),None),
			((1,1),(2,0,false),None),
			((1,1),(2,1,false),None),
			((1,1),(2,2,false),None)
		],
		vec![
			((8,0),(7,0,false),None),
			((8,0),(7,1,false),None),
			((8,0),(8,1,false),None),
		],
		vec![
			((8,1),(7,0,false),None),
			((8,1),(7,1,false),None),
			((8,1),(7,2,false),None),
			((8,1),(8,0,false),None),
			((8,1),(8,2,false),None),
		],
		vec![
			((7,0),(6,0,false),None),
			((7,0),(6,1,false),None),
			((7,0),(7,1,false),None),
			((7,0),(8,0,false),None),
			((7,0),(8,1,false),None)
		],
		vec![
			((7,1),(6,0,false),None),
			((7,1),(6,1,false),None),
			((7,1),(6,2,false),None),
			((7,1),(7,0,false),None),
			((7,1),(7,2,false),None),
			((7,1),(8,0,false),None),
			((7,1),(8,1,false),None),
			((7,1),(8,2,false),None)
		],
		vec![
			((0,8),(0,7,false),None),
			((0,8),(1,7,false),None),
			((0,8),(1,8,false),None)
		],
		vec![
			((0,7),(0,6,false),None),
			((0,7),(0,8,false),None),
			((0,7),(1,6,false),None),
			((0,7),(1,7,false),None),
			((0,7),(1,8,false),None)
		],
		vec![
			((1,8),(0,7,false),None),
			((1,8),(0,8,false),None),
			((1,8),(1,7,false),None),
			((1,8),(2,7,false),None),
			((1,8),(2,8,false),None)
		],
		vec![
			((1,7),(0,6,false),None),
			((1,7),(0,7,false),None),
			((1,7),(0,8,false),None),
			((1,7),(1,6,false),None),
			((1,7),(1,8,false),None),
			((1,7),(2,6,false),None),
			((1,7),(2,7,false),None),
			((1,7),(2,8,false),None)
		],
		vec![
			((8,8),(7,7,false),None),
			((8,8),(7,8,false),None),
			((8,8),(8,7,false),None),
		],
		vec![
			((8,7),(7,6,false),None),
			((8,7),(7,7,false),None),
			((8,7),(7,8,false),None),
			((8,7),(8,6,false),None),
			((8,7),(8,8,false),None),
		],
		vec![
			((7,8),(6,7,false),None),
			((7,8),(6,8,false),None),
			((7,8),(7,7,false),None),
			((7,8),(8,7,false),None),
			((7,8),(8,8,false),None)
		],
		vec![
			((7,7),(6,6,false),None),
			((7,7),(6,7,false),None),
			((7,7),(6,8,false),None),
			((7,7),(7,6,false),None),
			((7,7),(7,8,false),None),
			((7,7),(8,6,false),None),
			((7,7),(8,7,false),None),
			((7,7),(8,8,false),None)
		]
	];

	const POSITIONS:[(usize,usize); 16] = [
		(0,0),(0,1),(1,0),(1,1),
		(8,0),(8,1),(7,0),(7,1),
		(0,8),(0,7),(1,8),(1,7),
		(8,8),(8,7),(7,8),(7,7)
	];

	let answer = answer.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),_) => {
					LegalMove::from(((8 - sx, 8 - sy),(8- dx, 8 - dy, nari),None))
				}
			}
		}).collect::<Vec<LegalMove>>()
	}).collect::<Vec<Vec<LegalMove>>>();


	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (a,p) in answer.iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-p.1][8-p.0] = GOu;

		assert_eq!(
			a,
			&Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}

}
#[test]
fn test_legal_moves_banmen_with_ou_7_squares_sente() {
	const OFFSETS:[(i32,i32); 8] = [
		(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

		for o in &OFFSETS {
			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 6 + o.0 < 0 || 6 + o.0 > 8 {
				continue;
			}
			answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,false),None));
		}

		let answer = answer.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[y as usize][6] = SOu;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_ou_8_squares_sente() {
	const OFFSETS:[(i32,i32); 8] = [
		(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

		for o in &OFFSETS {
			if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 7 + o.0 < 0 || 7 + o.0 > 8 {
				continue;
			}
			answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,false),None));
		}

		let answer = answer.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[y as usize][7] = SOu;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_ou_7_squares_gote() {
	const OFFSETS:[(i32,i32); 8] = [
		(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

		for o in &OFFSETS {
			if y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 2 - o.0 < 0 || 2 - o.0 > 8 {
				continue;
			}
			answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,false),None));
		}

		let answer = answer.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[y as usize][2] = GOu;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_ou_8_squares_gote() {
	const OFFSETS:[(i32,i32); 8] = [
		(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

		for o in &OFFSETS {
			if y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 1 - o.0 < 0 || 1 - o.0 > 8 {
				continue;
			}
			answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,false),None));
		}

		let answer = answer.into_iter().map(|m| {
			match m {
				((sx,sy),(dx,dy,nari),o) => {
					LegalMove::from(((sx,sy),(dx,dy,nari),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[y as usize][1] = GOu;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_ou_7_squares_and_contiguous_self_sente() {
	const OFFSETS:[(i32,i32); 8] = [
		(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(1,-1),(1,0),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();
			if occ.1 >= 0 {
				for o in &OFFSETS {
					if occ == o || y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 6 + o.0 < 0 || 6 + o.0 > 8 {
						continue;
					}
					answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,false),None));
				}

				if y + occ.1 <= 8 && y + occ.1 >= 1 {
					let nari = (y + occ.1 - 1) <= 2;

					if nari {
						answer.push((
							((6 + occ.0) as u32,(y + occ.1) as u32),
							((6 + occ.0) as u32,(y + occ.1 - 1) as u32,true),None));
					}

					if y + occ.1 - 1 > 0 {
						answer.push((
								((6 + occ.0) as u32,(y + occ.1) as u32),
								((6 + occ.0) as u32,(y + occ.1 - 1) as u32,false),None));
					}
				}
			} else {
				if y + occ.1 >= 1 && y + occ.1 >= 1 {
					let nari = (y + occ.1 - 1) <= 2;

					if nari {
						answer.push((
							((6 + occ.0) as u32,(y + occ.1) as u32),
							((6 + occ.0) as u32,(y + occ.1 - 1) as u32,true),None));
					}

					if y + occ.1 - 1 > 0 {
						answer.push((
								((6 + occ.0) as u32,(y + occ.1) as u32),
								((6 + occ.0) as u32,(y + occ.1 - 1) as u32,false),None));
					}
				}

				for o in &OFFSETS {
					if occ == o || y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 6 + o.0 < 0 || 6 + o.0 > 8 {
						continue;
					}
					answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,false),None));
				}
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y + occ.1 >= 0 && y + occ.1 <= 8 {
				banmen.0[(y + occ.1) as usize][(6 + occ.0) as usize] = SFu;
			}

			banmen.0[y as usize][6] = SOu;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_ou_8_squares_and_contiguous_self_sente() {
	const OFFSETS:[(i32,i32); 8] = [
		(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(-1,-1),(-1,0),(-1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();
			if occ.1 > 0 {
				for o in &OFFSETS {
					if occ == o || y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 7 + o.0 < 0 || 7 + o.0 > 8 {
						continue;
					}
					answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,false),None));
				}

				if y + occ.1 <= 8 && y + occ.1 >= 1 {
					let nari = (y + occ.1 - 1) <= 2;

					if nari {
						answer.push((
							((7 + occ.0) as u32,(y + occ.1) as u32),
							((7 + occ.0) as u32,(y + occ.1 - 1) as u32,true),None));
					}

					if y + occ.1 - 1 >0 {
						answer.push((
							((7 + occ.0) as u32,(y + occ.1) as u32),
							((7 + occ.0) as u32,(y + occ.1 - 1) as u32,false),None));
					}
				}
			} else {
				if y + occ.1 >= 1 && y + occ.1 >= 1 {
					let nari = (y + occ.1 - 1) <= 2;

					if nari {
						answer.push((
							((7 + occ.0) as u32,(y + occ.1) as u32),
							((7 + occ.0) as u32,(y + occ.1 - 1) as u32,true),None));
					}

					if y + occ.1 - 1 >0 {
						answer.push((
							((7 + occ.0) as u32,(y + occ.1) as u32),
							((7 + occ.0) as u32,(y + occ.1 - 1) as u32,false),None));
					}
				}

				for o in &OFFSETS {
					if occ == o || y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 7 + o.0 < 0 || 7 + o.0 > 8 {
						continue;
					}
					answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,false),None));
				}
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y + occ.1 >= 0 && y + occ.1 <= 8 {
				banmen.0[(y + occ.1) as usize][(7 + occ.0) as usize] = SFu;
			}

			banmen.0[y as usize][7] = SOu;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_ou_7_squares_and_contiguous_self_gote() {
	const OFFSETS:[(i32,i32); 8] = [
		(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(1,-1),(1,0),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			if occ.1 >= 0 {
				for o in &OFFSETS {
					if occ == o || y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 2 - o.0 < 0 || 2 - o.0 > 8 {
						continue;
					}
					answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,false),None));
				}

				if y as i32 - occ.1 >=  0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6;

					if nari {
						answer.push((
							((2 - occ.0) as u32,(y - occ.1) as u32),
							((2 - occ.0) as u32,(y - occ.1 + 1) as u32,true),None));
					}

					if y - occ.1 + 1 < 8 {
						answer.push((
							((2 - occ.0) as u32,(y - occ.1) as u32),
							((2 - occ.0) as u32,(y - occ.1 + 1) as u32,false),None));
					}
				}
			} else {
				if y as i32 - occ.1 >= 0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6;

					if nari {
						answer.push((
							((2 - occ.0) as u32,(y - occ.1) as u32),
							((2 - occ.0) as u32,(y - occ.1 + 1) as u32,true),None));
					}

					if y - occ.1 + 1 < 8 {
						answer.push((
								((2 - occ.0) as u32,(y - occ.1) as u32),
								((2 - occ.0) as u32,(y - occ.1 + 1) as u32,false),None));
					}
				}

				for o in &OFFSETS {
					if occ == o || y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 2 - o.0 < 0 || 2 - o.0 > 8 {
						continue;
					}
					answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,false),None));
				}
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y - occ.1 >= 0 && y - occ.1 <= 8 {
				banmen.0[(y - occ.1) as usize][(2 - occ.0) as usize] = GFu;
			}

			banmen.0[y as usize][2] = GOu;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_ou_8_squares_and_contiguous_self_gote() {
	const OFFSETS:[(i32,i32); 8] = [
		(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(-1,-1),(-1,0),(-1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			if occ.1 > 0 {
				for o in &OFFSETS {
					if occ == o || y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 1 - o.0 < 0 || 1 - o.0 > 8 {
						continue;
					}
					answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,false),None));
				}

				if y as i32 - occ.1 >=  0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6;

					if nari {
						answer.push((
							((1 - occ.0) as u32,(y - occ.1) as u32),
							((1 - occ.0) as u32,(y - occ.1 + 1) as u32,true),None));
					}

					if y - occ.1 + 1 < 8 {
						answer.push((
								((1 - occ.0) as u32,(y - occ.1) as u32),
								((1 - occ.0) as u32,(y - occ.1 + 1) as u32,false),None));
					}
				}
			} else {
				if y as i32 - occ.1 >= 0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6;

					if nari {
						answer.push((
							((1 - occ.0) as u32,(y - occ.1) as u32),
							((1 - occ.0) as u32,(y - occ.1 + 1) as u32,true),None));
					}

					if y - occ.1 + 1 < 8 {
						answer.push((
								((1 - occ.0) as u32,(y - occ.1) as u32),
								((1 - occ.0) as u32,(y - occ.1 + 1) as u32,false),None));
					}
				}

				for o in &OFFSETS {
					if occ == o || y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 1 - o.0 < 0 || 1 - o.0 > 8 {
						continue;
					}
					answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,false),None));
				}
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y - occ.1 >= 0 && y - occ.1 <= 8 {
				banmen.0[(y - occ.1) as usize][(1 - occ.0) as usize] = GFu;
			}

			banmen.0[y as usize][1] = GOu;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_ou_7_squares_and_contiguous_opponent_sente() {
	const OFFSETS:[(i32,i32); 8] = [
		(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(1,1),(1,0),(1,-1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();
			for o in &OFFSETS {
				if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 6 + o.0 < 0 || 6 + o.0 > 8 {
					continue;
				}

				let obtained = if o == occ {
					Some(ObtainKind::Fu)
				} else {
					None
				};
				answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,false),obtained));
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y + occ.1 >= 0 && y + occ.1 <= 8 {
				banmen.0[(y + occ.1) as usize][(6 + occ.0) as usize] = GFu;
			}

			banmen.0[y as usize][6] = SOu;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_ou_8_squares_and_contiguous_opponent_sente() {
	const OFFSETS:[(i32,i32); 8] = [
		(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(-1,-1),(-1,0),(-1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			for o in &OFFSETS {
				if y as i32 + o.1 < 0 || y as i32 + o.1 > 8 || 7 + o.0 < 0 || 7 + o.0 > 8 {
					continue;
				}

				let obtained = if o == occ {
					Some(ObtainKind::Fu)
				} else {
					None
				};
				answer.push(((7,y as u32),((7+o.0) as u32,(y+o.1) as u32,false),obtained));
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y + occ.1 >= 0 && y + occ.1 <= 8 {
				banmen.0[(y + occ.1) as usize][(7 + occ.0) as usize] = GFu;
			}

			banmen.0[y as usize][7] = SOu;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_ou_7_squares_and_contiguous_opponent_gote() {
	const OFFSETS:[(i32,i32); 8] = [
		(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(1,-1),(1,0),(1,1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			for o in &OFFSETS {
				if y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 2 - o.0 < 0 || 2 - o.0 > 8 {
					continue;
				}

				let obtained = if o == occ {
					Some(ObtainKind::Fu)
				} else {
					None
				};
				answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,false),obtained));
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y - occ.1 >= 0 && y - occ.1 <= 8 {
				banmen.0[(y - occ.1) as usize][(2 - occ.0) as usize] = SFu;
			}

			banmen.0[y as usize][2] = GOu;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_ou_8_squares_and_contiguous_opponent_gote() {
	const OFFSETS:[(i32,i32); 8] = [
		(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
	];

	const OCC_OFFSETS:[(i32,i32); 3] = [
		(-1,1),(-1,0),(-1,-1)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for y in 0..9 {
		for occ in &OCC_OFFSETS {
			let mut answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = Vec::new();

			for o in &OFFSETS {
				if y as i32 - o.1 < 0 || y as i32 - o.1 > 8 || 1 - o.0 < 0 || 1 - o.0 > 8 {
					continue;
				}

				let obtained = if o == occ {
					Some(ObtainKind::Fu)
				} else {
					None
				};
				answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,false),obtained));
			}

			let answer = answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((sx,sy),(dx,dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>();

			let mut banmen = blank_banmen.clone();

			if y - occ.1 >= 0 && y - occ.1 <= 8 {
				banmen.0[(y - occ.1) as usize][(1 - occ.0) as usize] = SFu;
			}

			banmen.0[y as usize][1] = GOu;

			assert_eq!(
				answer,
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			)
		}
	}
}
#[test]
fn test_legal_moves_from_mochigoma_with_fu_sente() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[6][8] = Blank;
	banmen.0[0][8] = Blank;
	banmen.0[3][8] = GKyou;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Fu, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,HashMap::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_fu_gote() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8-6][8-8] = Blank;
	banmen.0[8-0][8-8] = Blank;
	banmen.0[8-3][8-8] = SKyou;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Fu, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(HashMap::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_kyou_sente() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[0][8] = Blank;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Kyou, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,HashMap::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_kyou_gote() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8-0][8-8] = Blank;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Kyou, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(HashMap::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_kei_sente() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[2][8] = Blank;
	banmen.0[3][8] = GFu;
	banmen.0[8][7] = Blank;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Kei, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,HashMap::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_kei_gote() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8-2][8-8] = Blank;
	banmen.0[8-3][8-8] = SFu;
	banmen.0[8-8][8-7] = Blank;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Kei, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(HashMap::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_gin_sente() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8][6] = Blank;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Gin, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,HashMap::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_gin_gote() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8-8][8-6] = Blank;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Gin, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(HashMap::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_kin_sente() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8][5] = Blank;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Kin, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,HashMap::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_kin_gote() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8-8][8-5] = Blank;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Kin, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(HashMap::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_hisha_sente() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[7][7] = Blank;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Hisha, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,HashMap::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_hisha_gote() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8-7][8-7] = Blank;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Hisha, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(HashMap::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_kaku_sente() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[7][1] = Blank;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Kaku, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,HashMap::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_kaku_gote() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8-7][8-1] = Blank;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Kaku, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(HashMap::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc,&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_all_sente() {
	const INITIAL_SFEN:&'static str = "sfen l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w RGgsn5p 1";

	let position_parser = PositionParser::new();

	let (_, banmen, mc, _, _) = match position_parser.parse(&INITIAL_SFEN.split(" ").collect::<Vec<&str>>()).unwrap() {
		position => match position {
			SystemEvent::Position(teban, p, n, m) => {
				let(banmen,mc) = match p {
					UsiInitialPosition::Startpos => {
						(rule::BANMEN_START_POS.clone(), MochigomaCollections::Pair(HashMap::new(),HashMap::new()))
					},
					UsiInitialPosition::Sfen(ref b,MochigomaCollections::Pair(ref ms,ref mg)) => {
						(b.clone(),MochigomaCollections::Pair(ms.clone(),mg.clone()))
					},
					UsiInitialPosition::Sfen(ref b,MochigomaCollections::Empty) => {
						(b.clone(),MochigomaCollections::Pair(HashMap::new(),HashMap::new()))
					}
				};
				(teban,banmen,mc,n,m)
			},
			_ => {
				panic!("invalid state.");
			}
		}
	};

	assert_eq!(legal_moves_all(&Teban::Sente,&banmen,&mc),
		Rule::legal_moves_all(Teban::Sente,&State::new(banmen.clone()),&mc).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_all_gote() {
	const INITIAL_SFEN:&'static str = "sfen l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w RGgsn5p 1";

	let position_parser = PositionParser::new();

	let (_, banmen, mc, _, _) = match position_parser.parse(&INITIAL_SFEN.split(" ").collect::<Vec<&str>>()).unwrap() {
		position => match position {
			SystemEvent::Position(teban, p, n, m) => {
				let(banmen,mc) = match p {
					UsiInitialPosition::Startpos => {
						(rule::BANMEN_START_POS.clone(), MochigomaCollections::Pair(HashMap::new(),HashMap::new()))
					},
					UsiInitialPosition::Sfen(ref b,MochigomaCollections::Pair(ref ms,ref mg)) => {
						(b.clone(),MochigomaCollections::Pair(ms.clone(),mg.clone()))
					},
					UsiInitialPosition::Sfen(ref b,MochigomaCollections::Empty) => {
						(b.clone(),MochigomaCollections::Pair(HashMap::new(),HashMap::new()))
					}
				};
				(teban,banmen,mc,n,m)
			},
			_ => {
				panic!("invalid state.");
			}
		}
	};

	assert_eq!(legal_moves_all(&Teban::Gote,&banmen,&mc),
		Rule::legal_moves_all(Teban::Gote,&State::new(banmen.clone()),&mc).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
fn test_win_only_moves_some_moves_sente_impl(ox:u32,oy:u32,positions:Vec<(u32,u32)>,kind:KomaKind,nari:bool) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = GOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer = if nari {
			vec![
				((p.0,p.1),(ox,oy,true),Some(ObtainKind::Ou)),
				((p.0,p.1),(ox,oy,false),Some(ObtainKind::Ou))
			]
		} else {
			vec![
				((p.0,p.1),(ox,oy,false),Some(ObtainKind::Ou))
			]
		};

		assert_eq!(answer.into_iter().map(|m| LegalMove::from(m)).collect::<Vec<LegalMove>>(),
			Rule::win_only_moves(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_win_only_moves_some_moves_gote_impl(ox:u32,oy:u32,positions:Vec<(u32,u32)>,kind:KomaKind,nari:bool) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = SOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer = if nari {
			vec![
				((p.0,p.1),(ox,oy,true),Some(ObtainKind::Ou)),
				((p.0,p.1),(ox,oy,false),Some(ObtainKind::Ou))
			]
		} else {
			vec![
				((p.0,p.1),(ox,oy,false),Some(ObtainKind::Ou))
			]
		};

		assert_eq!(answer.into_iter().map(|m| LegalMove::from(m)).collect::<Vec<LegalMove>>(),
			Rule::win_only_moves(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_win_only_moves_none_moves_sente_impl(ox:u32,oy:u32,positions:Vec<(u32,u32)>,kind:KomaKind) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = GOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_win_only_moves_none_moves_gote_impl(ox:u32,oy:u32,positions:Vec<(u32,u32)>,kind:KomaKind) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = SOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_some_moves_with_fu_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5)],SFu,false)
}
#[test]
fn test_win_only_moves_none_moves_with_fu_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(3,5),(4,6),(5,5)],SFu)
}
#[test]
fn test_win_only_moves_nari_moves_with_fu_sente() {
	test_win_only_moves_some_moves_sente_impl(4,2,vec![(4,3)],SFu,true)
}
#[test]
fn test_win_only_moves_some_moves_with_fu_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5)],GFu,false)
}
#[test]
fn test_win_only_moves_none_moves_with_fu_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-3,8-5),(8-4,8-6),(8-5,8-5)],GFu)
}
#[test]
fn test_win_only_moves_nari_moves_with_fu_gote() {
	test_win_only_moves_some_moves_gote_impl(4,8-2,vec![(8-4,8-3)],GFu,true)
}
#[test]
fn test_win_only_moves_some_moves_with_gin_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5),(3,5),(5,5),(3,3),(5,3)],SGin,false)
}
#[test]
fn test_win_only_moves_none_moves_with_gin_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(4,3),(3,4),(5,4),(4,6)],SGin)
}
#[test]
fn test_win_only_moves_nari_moves_with_gin_sente() {
	test_win_only_moves_some_moves_sente_impl(4,2,vec![(4,3),(3,3),(5,3),(3,1),(5,1)],SGin,true)
}
#[test]
fn test_win_only_moves_some_moves_with_gin_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5),(8-3,8-5),(8-5,8-5),(8-3,8-3),(8-5,8-3)],GGin,false)
}
#[test]
fn test_win_only_moves_none_moves_with_gin_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-4,8-3),(8-3,8-4),(8-5,8-4),(8-4,8-6)],GGin)
}
#[test]
fn test_win_only_moves_nari_moves_with_gin_gote() {
	test_win_only_moves_some_moves_gote_impl(4,8-2,vec![(8-4,8-3),(8-3,8-3),(8-5,8-3),(8-3,8-1),(8-5,8-1)],GGin,true)
}
#[test]
fn test_win_only_moves_some_moves_with_kin_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5),(3,5),(5,5),(3,4),(5,4),(4,3)],SKin,false)
}
#[test]
fn test_win_only_moves_none_moves_with_kin_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(3,3),(5,3),(4,6)],SKin)
}
#[test]
fn test_win_only_moves_some_moves_with_kin_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5),(8-3,8-5),(8-5,8-5),(8-3,8-4),(8-5,8-4),(8-4,8-3)],GKin,false)
}
#[test]
fn test_win_only_moves_none_moves_with_kin_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-3,8-3),(8-5,8-3),(8-4,8-6)],GKin)
}
#[test]
fn test_win_only_moves_some_moves_with_ou_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5),(3,5),(5,5),(3,4),(5,4),(3,3),(4,3),(5,3)],SOu,false)
}
#[test]
fn test_win_only_moves_none_moves_with_ou_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(4,6)],SOu)
}
#[test]
fn test_win_only_moves_none_moves_with_ou_gote() {
	test_win_only_moves_none_moves_gote_impl(8-4,8-4,vec![(8-4,8-6)],GOu)
}
#[test]
fn test_win_only_moves_some_moves_with_ou_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5),(8-3,8-5),(8-5,8-5),(8-3,8-4),(8-5,8-4),(8-3,8-3),(8-4,8-3),(8-5,8-3)],GOu,false)
}
#[test]
fn test_win_only_moves_some_moves_with_fu_nari_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5),(3,5),(5,5),(3,4),(5,4),(4,3)],SFuN,false)
}
#[test]
fn test_win_only_moves_none_moves_with_fu_nari_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(3,3),(5,3),(4,6)],SFuN)
}
#[test]
fn test_win_only_moves_some_moves_with_fu_nari_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5),(8-3,8-5),(8-5,8-5),(8-3,8-4),(8-5,8-4),(8-4,8-3)],GFuN,false)
}
#[test]
fn test_win_only_moves_none_moves_with_fu_nari_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-3,8-3),(8-5,8-3),(8-4,8-6)],GFuN)
}
#[test]
fn test_win_only_moves_some_moves_with_gin_nari_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5),(3,5),(5,5),(3,4),(5,4),(4,3)],SGinN,false)
}
#[test]
fn test_win_only_moves_none_moves_with_gin_nari_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(3,3),(5,3),(4,6)],SGinN)
}
#[test]
fn test_win_only_moves_some_moves_with_gin_nari_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5),(8-3,8-5),(8-5,8-5),(8-3,8-4),(8-5,8-4),(8-4,8-3)],GGinN,false)
}
#[test]
fn test_win_only_moves_none_moves_with_gin_nari_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-3,8-3),(8-5,8-3),(8-4,8-6)],GGinN)
}
#[test]
fn test_win_only_moves_some_moves_with_kyou_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,8)],SKyou,false)
}
#[test]
fn test_win_only_moves_none_moves_with_kyou_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(3,8),(5,8)],SKyou)
}
#[test]
fn test_win_only_moves_nari_moves_with_kyou_sente() {
	test_win_only_moves_some_moves_sente_impl(4,2,vec![(4,8)],SKyou,true)
}
#[test]
fn test_win_only_moves_some_moves_with_kyou_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(4,0)],GKyou,false)
}
#[test]
fn test_win_only_moves_none_moves_with_kyou_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-3,0),(8-5,0)],GKyou)
}
#[test]
fn test_win_only_moves_nari_moves_with_kyou_gote() {
	test_win_only_moves_some_moves_gote_impl(4,8-2,vec![(4,0)],GKyou,true)
}
#[test]
fn test_win_only_moves_none_moves_with_kyou_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 2] = [
		(0,8),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 2] = [
		(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKyou;
		banmen.0[o.1][o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_kyou_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 2] = [
		(0,8),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 2] = [
		(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKyou;
		banmen.0[8-o.1][8-o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_kyou_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 2] = [
		(0,8),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 2] = [
		(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKyou;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_kyou_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 2] = [
		(0,8),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 2] = [
		(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKyou;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_some_moves_with_kyou_nari_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5),(3,5),(5,5),(3,4),(5,4),(4,3)],SKyouN,false)
}
#[test]
fn test_win_only_moves_none_moves_with_kyou_nari_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(3,3),(5,3),(4,6)],SKyouN)
}
#[test]
fn test_win_only_moves_some_moves_with_kyou_nari_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5),(8-3,8-5),(8-5,8-5),(8-3,8-4),(8-5,8-4),(8-4,8-3)],GKyouN,false)
}
#[test]
fn test_win_only_moves_none_moves_with_kyou_nari_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-3,8-3),(8-5,8-3),(8-4,8-6)],GKyouN)
}
#[test]
fn test_win_only_moves_some_moves_with_kei_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(3,6),(5,6)],SKei,false)
}
#[test]
fn test_win_only_moves_none_moves_with_kei_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(4,6)],SKei)
}
#[test]
fn test_win_only_moves_nari_moves_with_kei_sente() {
	test_win_only_moves_some_moves_sente_impl(4,2,vec![(3,4),(5,4)],SKei,true)
}
#[test]
fn test_win_only_moves_some_moves_with_kei_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-3,8-6)],GKei,false)
}
#[test]
fn test_win_only_moves_none_moves_with_kei_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(4,8-6)],GKei)
}
#[test]
fn test_win_only_moves_nari_moves_with_kei_gote() {
	test_win_only_moves_some_moves_gote_impl(4,8-2,vec![(8-3,8-4),(8-5,8-4)],GKei,true)
}
#[test]
fn test_win_only_moves_some_moves_with_kei_jump_over_wall_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 2] = [
		(1,2),(7,2)
	];

	const OCC_POSITIONS:[(u32,u32); 2] = [
		(1,1),(7,1)
	];

	const OCC_KINDS:[KomaKind; 2] = [
		SFu,
		GFu
	];

	const OU_POSITIONS:[(u32,u32); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		for k in &OCC_KINDS {
			let mut banmen = blank_banmen.clone();

			banmen.0[t.1 as usize][t.0 as usize] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = SKei;
			banmen.0[o.1 as usize][o.0 as usize] = *k;

			let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
				((p.0,p.1),(t.0,t.1,true),Some(ObtainKind::Ou))
			];

			assert_eq!(answer.into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>(),
				Rule::win_only_moves(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_win_only_moves_some_moves_with_kei_jump_over_wall_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 2] = [
		(1,2),(7,2)
	];

	const OCC_POSITIONS:[(u32,u32); 2] = [
		(1,1),(7,1)
	];

	const OCC_KINDS:[KomaKind; 2] = [
		GFu,
		SFu
	];

	const OU_POSITIONS:[(u32,u32); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		for k in &OCC_KINDS {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - t.1 as usize][8 - t.0 as usize] = SOu;
			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GKei;
			banmen.0[8 - o.1 as usize][8 - o.0 as usize] = *k;

			let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
				((8 - p.0,8 - p.1),(8 - t.0,8 - t.1,true),Some(ObtainKind::Ou))
			];

			assert_eq!(answer.into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>(),
				Rule::win_only_moves(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_win_only_moves_some_moves_with_kaku_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(0,0),(0,8),(8,0),(8,8)],SKaku,false)
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(1,0),(0,7),(7,0),(7,8)],SKaku)
}
#[test]
fn test_win_only_moves_nari_moves_with_kaku_sente() {
	test_win_only_moves_some_moves_sente_impl(4,2,vec![(2,0),(2,4),(6,0),(6,4)],SKaku,true)
}
#[test]
fn test_win_only_moves_some_moves_with_kaku_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8,8),(8,0),(0,8),(0,0)],GKaku,false)
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-1,8),(8,8-7),(8-7,8),(8-7,8)],GKaku)
}
#[test]
fn test_win_only_moves_nari_moves_with_kaku_gote() {
	test_win_only_moves_some_moves_gote_impl(4,8-2,vec![(8-2,8),(8-2,8-4),(8-6,8),(8-6,8-4)],GKaku,true)
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKaku;
		banmen.0[o.1][o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKaku;
		banmen.0[8-o.1][8-o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKaku;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKaku;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_some_moves_with_hisha_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(0,4),(4,0),(8,4),(4,8)],SHisha,false)
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(0,3),(3,0),(8,5),(5,8)],SHisha)
}
#[test]
fn test_win_only_moves_nari_moves_with_hisha_sente() {
	test_win_only_moves_some_moves_sente_impl(4,2,vec![(0,2),(4,0),(8,2),(4,8)],SHisha,true)
}
#[test]
fn test_win_only_moves_some_moves_with_hisha_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8,8-4),(8-4,8),(0,8-4),(8-4,0)],GHisha,false)
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-0,8-3),(8-3,8-0),(8-8,8-5),(8-5,8-8)],GHisha)
}
#[test]
fn test_win_only_moves_nari_moves_with_hisha_gote() {
	test_win_only_moves_some_moves_gote_impl(4,8-2,vec![(8-0,8-2),(8-4,8-0),(8-8,8-2),(8-4,8-8)],GHisha,true)
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,2),(7,6),(1,6),(1,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHisha;
		banmen.0[o.1][o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,2),(7,6),(1,6),(1,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHisha;
		banmen.0[8-o.1][8-o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,1),(7,7),(1,7),(7,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHisha;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,1),(7,7),(1,7),(7,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHisha;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_some_moves_with_kaku_nari_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(0,0),(0,8),(8,0),(8,8),(4,5),(3,4),(5,4),(4,3)],SKakuN,false)
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_nari_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(1,0),(0,7),(7,0),(7,8),(4,6),(2,4),(6,4),(4,2)],SKakuN)
}
#[test]
fn test_win_only_moves_some_moves_with_kaku_nari_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8,8),(8,0),(0,8),(0,0),(8-4,8-5),(8-3,8-4),(8-5,8-4),(8-4,8-3)],GKakuN,false)
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_nari_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-1,8),(8,8-7),(8-7,8),(8-7,8),(8-4,8-6),(8-2,8-4),(8-6,8-4),(8-4,8-2)],GKakuN)
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_nari_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKakuN;
		banmen.0[o.1][o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_nari_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKakuN;
		banmen.0[8-o.1][8-o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_nari_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKakuN;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_nari_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKakuN;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_some_moves_with_hisha_nari_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(0,4),(4,0),(8,4),(4,8),(3,5),(5,5),(3,3),(5,3)],SHishaN,false)
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_nari_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(0,3),(3,0),(8,5),(5,8),(2,6),(6,6),(2,2),(6,2)],SHishaN)
}
#[test]
fn test_win_only_moves_some_moves_with_hisha_nari_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8,8-4),(8-4,8),(0,8-4),(8-4,0),(8-3,8-5),(8-5,8-5),(8-3,8-3),(8-5,8-3)],GHishaN,false)
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_nari_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-0,8-3),(8-3,8-0),(8-8,8-5),(8-5,8-8),(8-2,8-6),(8-6,8-6),(8-2,8-2),(8-6,8-2)],GHishaN)
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_nari_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,2),(7,6),(1,6),(1,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHishaN;
		banmen.0[o.1][o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_nari_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,2),(7,6),(1,6),(1,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHishaN;
		banmen.0[8-o.1][8-o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_nari_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,1),(7,7),(1,7),(7,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHishaN;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_nari_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,1),(7,7),(1,7),(7,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHishaN;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::win_only_moves(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_oute_only_moves_win_only_result_some_moves_sente_impl(ox:u32,oy:u32,positions:Vec<(u32,u32)>,kind:KomaKind,nari:bool) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = GOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer = if nari {
			vec![
				((p.0,p.1),(ox,oy,true),Some(ObtainKind::Ou)),
				((p.0,p.1),(ox,oy,false),Some(ObtainKind::Ou))
			]
		} else {
			vec![
				((p.0,p.1),(ox,oy,false),Some(ObtainKind::Ou))
			]
		};

		assert_eq!(answer.into_iter().map(|m| LegalMove::from(m)).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_oute_only_moves_win_only_result_some_moves_gote_impl(ox:u32,oy:u32,positions:Vec<(u32,u32)>,kind:KomaKind,nari:bool) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = SOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer = if nari {
			vec![
				((p.0,p.1),(ox,oy,true),Some(ObtainKind::Ou)),
				((p.0,p.1),(ox,oy,false),Some(ObtainKind::Ou))
			]
		} else {
			vec![
				((p.0,p.1),(ox,oy,false),Some(ObtainKind::Ou))
			]
		};

		assert_eq!(answer.into_iter().map(|m| LegalMove::from(m)).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_oute_only_moves_win_only_result_none_moves_sente_impl(ox:u32,oy:u32,positions:Vec<(u32,u32)>,kind:KomaKind) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = GOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_oute_only_moves_win_only_result_none_moves_gote_impl(ox:u32,oy:u32,positions:Vec<(u32,u32)>,kind:KomaKind) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = SOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_fu_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5)],SFu,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_fu_sente() {
	test_oute_only_moves_win_only_result_none_moves_sente_impl(4,4,vec![(3,5),(4,6),(5,5)],SFu)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_fu_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,2,vec![(4,3)],SFu,true)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_fu_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5)],GFu,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_fu_gote() {
	test_oute_only_moves_win_only_result_none_moves_gote_impl(4,4,vec![(8-3,8-5),(8-4,8-6),(8-5,8-5)],GFu)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_fu_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,8-2,vec![(8-4,8-3)],GFu,true)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_gin_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5),(3,5),(5,5),(3,3),(5,3)],SGin,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_gin_sente() {
	test_oute_only_moves_win_only_result_none_moves_sente_impl(4,4,vec![(4,3),(3,4),(5,4),(4,6)],SGin)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_gin_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,2,vec![(4,3),(3,3),(5,3),(3,1),(5,1)],SGin,true)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_gin_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5),(8-3,8-5),(8-5,8-5),(8-3,8-3),(8-5,8-3)],GGin,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_gin_gote() {
	test_oute_only_moves_win_only_result_none_moves_gote_impl(4,4,vec![(8-4,8-3),(8-3,8-4),(8-5,8-4),(8-4,8-6)],GGin)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_gin_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,8-2,vec![(8-4,8-3),(8-3,8-3),(8-5,8-3),(8-3,8-1),(8-5,8-1)],GGin,true)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kin_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5),(3,5),(5,5),(3,4),(5,4),(4,3)],SKin,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kin_sente() {
	test_oute_only_moves_win_only_result_none_moves_sente_impl(4,4,vec![(3,3),(5,3),(4,6)],SKin)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kin_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5),(8-3,8-5),(8-5,8-5),(8-3,8-4),(8-5,8-4),(8-4,8-3)],GKin,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kin_gote() {
	test_oute_only_moves_win_only_result_none_moves_gote_impl(4,4,vec![(8-3,8-3),(8-5,8-3),(8-4,8-6)],GKin)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_ou_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5),(3,5),(5,5),(3,4),(5,4),(3,3),(4,3),(5,3)],SOu,false)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_ou_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5),(8-3,8-5),(8-5,8-5),(8-3,8-4),(8-5,8-4),(8-3,8-3),(8-4,8-3),(8-5,8-3)],GOu,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_ou_gote() {
	test_oute_only_moves_win_only_result_none_moves_gote_impl(4,4,vec![(4,6)],GOu)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_fu_nari_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5),(3,5),(5,5),(3,4),(5,4),(4,3)],SFuN,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_fu_nari_sente() {
	test_oute_only_moves_win_only_result_none_moves_sente_impl(4,4,vec![(3,3),(5,3),(4,6)],SFuN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_fu_nari_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5),(8-3,8-5),(8-5,8-5),(8-3,8-4),(8-5,8-4),(8-4,8-3)],GFuN,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_fu_nari_gote() {
	test_oute_only_moves_win_only_result_none_moves_gote_impl(4,4,vec![(8-3,8-3),(8-5,8-3),(8-4,8-6)],GFuN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_gin_nari_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5),(3,5),(5,5),(3,4),(5,4),(4,3)],SGinN,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_gin_nari_sente() {
	test_oute_only_moves_win_only_result_none_moves_sente_impl(4,4,vec![(3,3),(5,3),(4,6)],SGinN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_gin_nari_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5),(8-3,8-5),(8-5,8-5),(8-3,8-4),(8-5,8-4),(8-4,8-3)],GGinN,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_gin_nari_gote() {
	test_oute_only_moves_win_only_result_none_moves_gote_impl(4,4,vec![(8-3,8-3),(8-5,8-3),(8-4,8-6)],GGinN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kyou_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,8)],SKyou,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kyou_sente() {
	test_oute_only_moves_win_only_result_none_moves_sente_impl(4,4,vec![(3,8),(5,8)],SKyou)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_kyou_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,2,vec![(4,8)],SKyou,true)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kyou_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(4,0)],GKyou,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kyou_gote() {
	test_oute_only_moves_win_only_result_none_moves_gote_impl(4,4,vec![(8-3,0),(8-5,0)],GKyou)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_kyou_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,8-2,vec![(4,0)],GKyou,true)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kyou_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 2] = [
		(0,8),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 2] = [
		(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKyou;
		banmen.0[o.1][o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kyou_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 2] = [
		(0,8),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 2] = [
		(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKyou;
		banmen.0[8-o.1][8-o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_moves_with_kyou_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 2] = [
		(0,8),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 2] = [
		(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKyou;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
			((p.1 as u32, p.0 as u32),(o.1 as u32, o.0 as u32, true),Some(ObtainKind::Fu)),
			((p.1 as u32, p.0 as u32),(o.1 as u32, o.0 as u32, false),Some(ObtainKind::Fu)),
		];

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kyou_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 2] = [
		(0,8),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 2] = [
		(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKyou;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
			((8-p.1 as u32, 8-p.0 as u32),(8-o.1 as u32, 8-o.0 as u32,true),Some(ObtainKind::Fu)),
			((8-p.1 as u32, 8-p.0 as u32),(8-o.1 as u32, 8-o.0 as u32,false),Some(ObtainKind::Fu)),
		];

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kyou_nari_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5),(3,5),(5,5),(3,4),(5,4),(4,3)],SKyouN,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kyou_nari_sente() {
	test_oute_only_moves_win_only_result_none_moves_sente_impl(4,4,vec![(3,3),(5,3),(4,6)],SKyouN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kyou_nari_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5),(8-3,8-5),(8-5,8-5),(8-3,8-4),(8-5,8-4),(8-4,8-3)],GKyouN,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kyou_nari_gote() {
	test_oute_only_moves_win_only_result_none_moves_gote_impl(4,4,vec![(8-3,8-3),(8-5,8-3),(8-4,8-6)],GKyouN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kei_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(3,6),(5,6)],SKei,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kei_sente() {
	test_oute_only_moves_win_only_result_none_moves_sente_impl(4,4,vec![(4,6)],SKei)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_kei_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,2,vec![(3,4),(5,4)],SKei,true)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kei_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-3,8-6)],GKei,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kei_gote() {
	test_oute_only_moves_win_only_result_none_moves_gote_impl(4,4,vec![(4,8-6)],GKei)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_kei_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,8-2,vec![(8-3,8-4),(8-5,8-4)],GKei,true)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kei_jump_over_wall_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 2] = [
		(1,2),(7,2)
	];

	const OCC_POSITIONS:[(u32,u32); 2] = [
		(1,1),(7,1)
	];

	const OCC_KINDS:[KomaKind; 2] = [
		SFu,
		GFu
	];

	const OU_POSITIONS:[(u32,u32); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		for k in &OCC_KINDS {
			let mut banmen = blank_banmen.clone();

			banmen.0[t.1 as usize][t.0 as usize] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = SKei;
			banmen.0[o.1 as usize][o.0 as usize] = *k;

			let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
				((p.0,p.1),(t.0,t.1,true),Some(ObtainKind::Ou))
			];

			assert_eq!(answer.into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>(),
				Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kei_jump_over_wall_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 2] = [
		(1,2),(7,2)
	];

	const OCC_POSITIONS:[(u32,u32); 2] = [
		(1,1),(7,1)
	];

	const OCC_KINDS:[KomaKind; 2] = [
		GFu,
		SFu
	];

	const OU_POSITIONS:[(u32,u32); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		for k in &OCC_KINDS {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - t.1 as usize][8 - t.0 as usize] = SOu;
			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GKei;
			banmen.0[8 - o.1 as usize][8 - o.0 as usize] = *k;

			let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
				((8 - p.0,8 - p.1),(8 - t.0,8 - t.1,true),Some(ObtainKind::Ou))
			];

			assert_eq!(answer.into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>(),
				Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kaku_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(0,0),(0,8),(8,0),(8,8)],SKaku,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kaku_sente() {
	test_oute_only_moves_win_only_result_none_moves_sente_impl(4,4,vec![(1,0),(0,7),(7,0),(7,8)],SKaku)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_kaku_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,2,vec![(2,0),(2,4),(6,0),(6,4)],SKaku,true)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kaku_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8,8),(8,0),(0,8),(0,0)],GKaku,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kaku_gote() {
	test_oute_only_moves_win_only_result_none_moves_gote_impl(4,4,vec![(8-1,8),(8,8-7),(8-7,8),(8-7,8)],GKaku)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_kaku_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,8-2,vec![(8-2,8),(8-2,8-4),(8-6,8),(8-6,8-4)],GKaku,true)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kaku_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKaku;
		banmen.0[o.1][o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kaku_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKaku;
		banmen.0[8-o.1][8-o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kaku_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKaku;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kaku_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(5,3),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKaku;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_hisha_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(0,4),(4,0),(8,4),(4,8)],SHisha,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_hisha_sente() {
	test_oute_only_moves_win_only_result_none_moves_sente_impl(4,4,vec![(0,3),(3,0),(8,5),(5,8)],SHisha)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_hisha_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,2,vec![(0,2),(4,0),(8,2),(4,8)],SHisha,true)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_hisha_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8,8-4),(8-4,8),(0,8-4),(8-4,0)],GHisha,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_hisha_gote() {
	test_oute_only_moves_win_only_result_none_moves_gote_impl(4,4,vec![(8-0,8-3),(8-3,8-0),(8-8,8-5),(8-5,8-8)],GHisha)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_hisha_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,8-2,vec![(8-0,8-2),(8-4,8-0),(8-8,8-2),(8-4,8-8)],GHisha,true)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_hisha_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,2),(7,6),(1,6),(1,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHisha;
		banmen.0[o.1][o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_hisha_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,2),(7,6),(1,6),(1,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHisha;
		banmen.0[8-o.1][8-o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_moves_with_hisha_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,1),(7,7),(1,7),(7,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHisha;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
			((p.1 as u32, p.0 as u32),(o.1 as u32, o.0 as u32, true),Some(ObtainKind::Fu)),
			((p.1 as u32, p.0 as u32),(o.1 as u32, o.0 as u32, false),Some(ObtainKind::Fu)),
		];

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,1),(7,7),(1,7),(7,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHisha;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
			((8-p.1 as u32, 8-p.0 as u32),(8-o.1 as u32, 8-o.0 as u32,true),Some(ObtainKind::Fu)),
			((8-p.1 as u32, 8-p.0 as u32),(8-o.1 as u32, 8-o.0 as u32,false),Some(ObtainKind::Fu)),
		];

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kaku_nari_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(0,0),(0,8),(8,0),(8,8),(4,5),(3,4),(5,4),(4,3)],SKakuN,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kaku_nari_sente() {
	test_oute_only_moves_win_only_result_none_moves_sente_impl(4,4,vec![(1,0),(0,7),(7,0),(7,8),(4,6),(2,4),(6,4),(4,2)],SKakuN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kaku_nari_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8,8),(8,0),(0,8),(0,0),(8-4,8-5),(8-3,8-4),(8-5,8-4),(8-4,8-3)],GKakuN,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kaku_nari_gote() {
	test_oute_only_moves_win_only_result_none_moves_gote_impl(4,4,vec![(8-1,8),(8,8-7),(8-7,8),(8-7,8),(8-4,8-6),(8-2,8-4),(8-6,8-4),(8-4,8-2)],GKakuN)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kaku_nari_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKakuN;
		banmen.0[o.1][o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kaku_nari_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKakuN;
		banmen.0[8-o.1][8-o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kaku_nari_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(3,5),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKakuN;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_kaku_nari_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,3),(5,3),(5,3),(5,5)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKakuN;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer,
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_hisha_nari_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(0,4),(4,0),(8,4),(4,8),(3,5),(5,5),(3,3),(5,3)],SHishaN,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_hisha_nari_sente() {
	test_oute_only_moves_win_only_result_none_moves_sente_impl(4,4,vec![(0,3),(3,0),(8,5),(5,8),(2,6),(6,6),(2,2),(6,2)],SHishaN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_hisha_nari_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8,8-4),(8-4,8),(0,8-4),(8-4,0),(8-3,8-5),(8-5,8-5),(8-3,8-3),(8-5,8-3)],GHishaN,false)
}
#[test]
fn test_oute_only_moves_win_only_result_none_moves_with_hisha_nari_gote() {
	test_oute_only_moves_win_only_result_none_moves_gote_impl(4,4,vec![(8-0,8-3),(8-3,8-0),(8-8,8-5),(8-5,8-8),(8-2,8-6),(8-6,8-6),(8-2,8-2),(8-6,8-2)],GHishaN)
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_nari_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,2),(7,6),(1,6),(1,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHishaN;
		banmen.0[o.1][o.0] = SFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
			((p.1 as u32, p.0 as u32),(o.1 as u32, o.0 as u32, true),Some(ObtainKind::Fu)),
			((p.1 as u32, p.0 as u32),(o.1 as u32, o.0 as u32, false),Some(ObtainKind::Fu)),
		];

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_nari_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,2),(7,6),(1,6),(1,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHishaN;
		banmen.0[8-o.1][8-o.0] = GFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
			((8-t.1 as u32,8-t.0 as u32),(8-o.1 as u32, 8-o.0 as u32,true), Some(ObtainKind::Fu)),
			((8-t.1 as u32,8-t.0 as u32),(8-o.1 as u32, 8-o.0 as u32,false), Some(ObtainKind::Fu)),
		];

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_nari_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,1),(7,7),(1,7),(7,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHishaN;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
			((p.1 as u32,p.0 as u32),(o.1 as u32,o.0 as u32,true),Some(ObtainKind::Fu)),
			((p.1 as u32,p.0 as u32),(o.1 as u32,o.0 as u32,false),Some(ObtainKind::Fu)),
		];

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_nari_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,1),(7,7),(1,7),(7,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHishaN;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
			((8-p.1 as u32,8-p.0 as u32),(8-o.1 as u32,8-o.0 as u32,true),Some(ObtainKind::Fu)),
			((8-p.1 as u32,8-p.0 as u32),(8-o.1 as u32,8-o.0 as u32,false),Some(ObtainKind::Fu)),
		];

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_apply_move_none_check_sente() {
	let mvs:Vec<Vec<Move>> = vec![
		vec![
			Move::To(KomaSrcPosition(9-2,6+1),KomaDstToPosition(9-2,5+1,false)),
			Move::To(KomaSrcPosition(9-1,7+1),KomaDstToPosition(9-3,5+1,false)),
			Move::To(KomaSrcPosition(9-7,6+1),KomaDstToPosition(9-7,5+1,false)),
			Move::To(KomaSrcPosition(9-2,5+1),KomaDstToPosition(9-2,4+1,false)),
			Move::To(KomaSrcPosition(9-2,4+1),KomaDstToPosition(9-2,3+1,false)),
			Move::To(KomaSrcPosition(9-2,3+1),KomaDstToPosition(9-2,2+1,false))
		],
		vec![
			Move::To(KomaSrcPosition(9-2,1+0),KomaDstToPosition(9-2,1+1,false)),
			Move::To(KomaSrcPosition(9-2,1+1),KomaDstToPosition(9-2,2+1,false))
		]
	];

	let mut banmen = BANMEN_START_POS.clone();
	let mut state = State::new(banmen.clone());

	let mut teban = Teban::Sente;
	let mut omc = MochigomaCollections::Empty;

	for m in &mvs {
		for m in m {
			match apply_move_none_check(&banmen,&teban,&omc,m) {
				(next,nmc,_) => {
					banmen = next;
					omc = nmc;
				}
			}
		}
		teban = teban.opposite();
	}

	let mut teban = Teban::Sente;
	let mut mc = MochigomaCollections::Empty;

	for m in &mvs {
		for m in m {
			match Rule::apply_move_none_check(&state,teban,&mc,m.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}
		}
		teban = teban.opposite();
	}

	assert_eq!(legal_moves_all(&Teban::Sente,&banmen,&omc),
		Rule::legal_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_apply_move_none_check_gote() {
	let mvs:Vec<Vec<Move>> = vec![
		vec![
			Move::To(KomaSrcPosition(9-2,6+1),KomaDstToPosition(9-2,5+1,false)),
			Move::To(KomaSrcPosition(9-1,7+1),KomaDstToPosition(9-3,5+1,false)),
			Move::To(KomaSrcPosition(9-7,6+1),KomaDstToPosition(9-7,5+1,false)),
			Move::To(KomaSrcPosition(9-2,5+1),KomaDstToPosition(9-2,4+1,false)),
			Move::To(KomaSrcPosition(9-2,4+1),KomaDstToPosition(9-2,3+1,false)),
			Move::To(KomaSrcPosition(9-2,3+1),KomaDstToPosition(9-2,2+1,false))
		],
		vec![
			Move::To(KomaSrcPosition(9-2,1+0),KomaDstToPosition(9-2,1+1,false)),
			Move::To(KomaSrcPosition(9-2,1+1),KomaDstToPosition(9-2,2+1,false))
		]
	];

	let mvs = mvs.into_iter().map(|mvs| {
		mvs.into_iter().map(|m| {
			match m {
				Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,n)) => {
					Move::To(KomaSrcPosition(10-sx,10-sy),KomaDstToPosition(10-dx,10-dy,n))
				},
				Move::Put(k,KomaDstPutPosition(dx,dy)) => {
					Move::Put(k,KomaDstPutPosition(10-dx,10-dy))
				}
			}
		}).collect::<Vec<Move>>()
	}).collect::<Vec<Vec<Move>>>();

	let mut banmen = BANMEN_START_POS.clone();
	let mut state = State::new(banmen.clone());
	let mut teban = Teban::Gote;
	let mut omc = MochigomaCollections::Empty;

	for m in &mvs {
		for m in m {
			match apply_move_none_check(&banmen,&teban,&omc,m) {
				(next,nmc,_) => {
					banmen = next;
					omc = nmc;
				}
			}
		}
		teban = teban.opposite();
	}
	let mut teban = Teban::Gote;
	let mut mc = MochigomaCollections::Empty;

	for m in &mvs {
		for m in m {
			match Rule::apply_move_none_check(&state,teban,&mc,m.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}
		}
		teban = teban.opposite();
	}

	assert_eq!(legal_moves_all(&Teban::Gote,&banmen,&omc),
		Rule::legal_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_apply_move_none_check_nari_move_sente() {
	let mvs:Vec<Move> = vec![
		Move::To(KomaSrcPosition(9-1,7+1),KomaDstToPosition(9-3,5+1,false)),
		Move::To(KomaSrcPosition(9-7,6+1),KomaDstToPosition(9-7,5+1,false)),
		Move::To(KomaSrcPosition(9-2,5+1),KomaDstToPosition(9-2,4+1,false)),
		Move::To(KomaSrcPosition(9-2,4+1),KomaDstToPosition(9-2,3+1,false)),
		Move::To(KomaSrcPosition(9-2,3+1),KomaDstToPosition(9-2,2+1,true))
	];

	let mut banmen = BANMEN_START_POS.clone();
	let mut state = State::new(banmen.clone());
	let teban = Teban::Sente;
	let mut mc = MochigomaCollections::Empty;

	for m in &mvs {
		match apply_move_none_check(&banmen,&teban,&mc,m) {
			(next,nmc,_) => {
				banmen = next;
				mc = nmc;
			}
		}
	}
	let teban = Teban::Sente;
	let mut mc = MochigomaCollections::Empty;

	for m in &mvs {
		match Rule::apply_move_none_check(&state,teban,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}
	}

	assert_eq!(legal_moves_all(&Teban::Sente,&banmen,&mc),
		Rule::legal_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_apply_move_none_check_nari_move_gote() {
	let mvs:Vec<Move> = vec![
		Move::To(KomaSrcPosition(9-1,7+1),KomaDstToPosition(9-3,5+1,false)),
		Move::To(KomaSrcPosition(9-7,6+1),KomaDstToPosition(9-7,5+1,false)),
		Move::To(KomaSrcPosition(9-2,5+1),KomaDstToPosition(9-2,4+1,false)),
		Move::To(KomaSrcPosition(9-2,4+1),KomaDstToPosition(9-2,3+1,false)),
		Move::To(KomaSrcPosition(9-2,3+1),KomaDstToPosition(9-2,2+1,true))
	];

	let mvs = mvs.into_iter().map(|m| {
		match m {
			Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,n)) => {
				Move::To(KomaSrcPosition(10-sx,10-sy),KomaDstToPosition(10-dx,10-dy,n))
			},
			Move::Put(k,KomaDstPutPosition(dx,dy)) => {
				Move::Put(k,KomaDstPutPosition(10-dx,10-dy))
			}
		}
	}).collect::<Vec<Move>>();

	let mut banmen = BANMEN_START_POS.clone();
	let mut state = State::new(banmen.clone());
	let teban = Teban::Gote;
	let mut mc = MochigomaCollections::Empty;

	for m in &mvs {
		match apply_move_none_check(&banmen,&teban,&mc,m) {
			(next,nmc,_) => {
				banmen = next;
				mc = nmc;
			}
		}
	}
	let teban = Teban::Gote;
	let mut mc = MochigomaCollections::Empty;

	for m in &mvs {
		match Rule::apply_move_none_check(&state,teban,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}
	}

	assert_eq!(legal_moves_all(&Teban::Gote,&banmen,&mc),
		Rule::legal_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_apply_move_none_check_put_move_sente() {
	let mvs:Vec<Move> = vec![
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-7,4+1)),
		Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-7,4+1)),
	];

	for m in &mvs {
		let mut banmen = BANMEN_START_POS.clone();

		banmen.0[6][8] = Blank;
		banmen.0[6][7] = Blank;
		banmen.0[8][8] = Blank;

		let mut state = State::new(banmen.clone());

		let teban = Teban::Sente;
		let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
		let mg:HashMap<MochigomaKind,u32> = HashMap::new();

		ms.insert(MochigomaKind::Fu, 2);
		ms.insert(MochigomaKind::Kyou,1);

		let mut omc = MochigomaCollections::Pair(ms,mg);

		match apply_move_none_check(&banmen,&teban,&omc,m) {
			(next,nmc,_) => {
				banmen = next;
				omc = nmc;
			}
		}

		let teban = Teban::Sente;
		let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
		let mg:HashMap<MochigomaKind,u32> = HashMap::new();

		ms.insert(MochigomaKind::Fu, 2);
		ms.insert(MochigomaKind::Kyou,1);

		let mut mc = MochigomaCollections::Pair(ms,mg);

		match Rule::apply_move_none_check(&state,teban,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(legal_moves_all(&Teban::Sente,&banmen,&omc),
			Rule::legal_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_apply_move_none_check_put_move_gote() {
	let mvs:Vec<Move> = vec![
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-7,4+1)),
		Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-7,4+1)),
	];

	let mvs = mvs.into_iter().map(|m| {
		match m {
			Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,n)) => {
				Move::To(KomaSrcPosition(10-sx,10-sy),KomaDstToPosition(10-dx,10-dy,n))
			},
			Move::Put(k,KomaDstPutPosition(dx,dy)) => {
				Move::Put(k,KomaDstPutPosition(10-dx,10-dy))
			}
		}
	}).collect::<Vec<Move>>();

	for m in &mvs {
		let mut banmen = BANMEN_START_POS.clone();

		banmen.0[6][8] = Blank;
		banmen.0[6][7] = Blank;
		banmen.0[8][8] = Blank;

		let mut state = State::new(banmen.clone());

		let teban = Teban::Gote;
		let ms:HashMap<MochigomaKind,u32> = HashMap::new();
		let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

		mg.insert(MochigomaKind::Fu, 2);
		mg.insert(MochigomaKind::Kyou,1);

		let mut omc = MochigomaCollections::Pair(ms,mg);

		match apply_move_none_check(&banmen,&teban,&omc,m) {
			(next,nmc,_) => {
				banmen = next;
				omc = nmc;
			}
		}

		let teban = Teban::Gote;
		let ms:HashMap<MochigomaKind,u32> = HashMap::new();
		let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

		mg.insert(MochigomaKind::Fu, 2);
		mg.insert(MochigomaKind::Kyou,1);

		let mut mc = MochigomaCollections::Pair(ms,mg);

		match Rule::apply_move_none_check(&state,teban,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(legal_moves_all(&Teban::Gote,&banmen,&omc),
			Rule::legal_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
impl From<rule::LegalMove> for LegalMove {
	fn from(m:rule::LegalMove) -> LegalMove {
		match m {
			rule::LegalMove::To(m) => {
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

				LegalMove::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,n),m.obtained())
			},
			rule::LegalMove::Put(m) => {
				let dst = m.dst();
				let kind = m.kind();
				let dx = dst / 9;
				let dy = dst - dx * 9;
				let dx = 9 - dx;
				let dy = dy + 1;

				LegalMove::Put(kind,KomaDstPutPosition(dx,dy))
			}
		}
	}
}
impl From<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> for LegalMove {
	fn from(to:((u32,u32),(u32,u32,bool),Option<ObtainKind>)) -> LegalMove {
		match to {
			((sx,sy),(dx,dy,nari),obtained) => {
				LegalMove::To(
					KomaSrcPosition(9 - sx, sy + 1),
					KomaDstToPosition(9 - dx, dy + 1, nari),
					obtained
				)
			}
		}
	}
}
impl From<(MochigomaKind,(u32,u32))> for LegalMove {
	fn from(put:(MochigomaKind,(u32,u32))) -> LegalMove {
		match put {
			(k,(x,y)) => {
				LegalMove::Put(k,KomaDstPutPosition(9 - x, y + 1))
			}
		}
	}
}
fn find_from_move_to(mvs:&Vec<LegalMove>,query:&(KomaSrcPosition,KomaDstToPosition)) -> Option<Move> {
	match query {
		&(ref s, ref d) => {
			for m in mvs {
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
#[allow(dead_code)]
fn find_from_move_put(mvs:&Vec<LegalMove>,query:&(MochigomaKind,KomaDstPutPosition)) -> Option<Move> {
	match query {
		&(ref k, ref d) => {
			for m in mvs {
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
#[allow(dead_code)]
fn legal_moves_with_point_and_kind(t:&Teban,banmen:&Banmen,x:u32,y:u32,kind:KomaKind)
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
													(kind == SKei && dy <= 1) => {
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
#[allow(dead_code)]
fn legal_moves_with_point(t:&Teban,banmen:&Banmen,x:u32,y:u32)
	-> Vec<LegalMove> {
	match banmen {
		&Banmen(ref kinds) => {
			legal_moves_with_point_and_kind(t,banmen,x,y,kinds[y as usize][x as usize])
		}
	}
}
#[allow(dead_code)]
fn legal_moves_with_src(t:&Teban,banmen:&Banmen,src:KomaSrcPosition)
	-> Vec<LegalMove> {
	match src {
		KomaSrcPosition(x,y) => legal_moves_with_point(t, banmen, 9 - x, y - 1)
	}
}
#[allow(dead_code)]
fn legal_moves_with_dst_to(t:&Teban,banmen:&Banmen,dst:KomaDstToPosition)
	-> Vec<LegalMove> {
	match dst {
		KomaDstToPosition(x,y,_) => legal_moves_with_point(t, banmen, 9 - x, y - 1)
	}
}
#[allow(dead_code)]
fn legal_moves_with_dst_put(t:&Teban,banmen:&Banmen,dst:KomaDstPutPosition)
	-> Vec<LegalMove> {
	match dst {
		KomaDstPutPosition(x,y) => legal_moves_with_point(t, banmen, 9 - x, y - 1)
	}
}
#[allow(dead_code)]
fn legal_moves_from_banmen(t:&Teban,banmen:&Banmen)
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
					mvs.append(&mut legal_moves_with_point(t, banmen, x as u32, y as u32));
				}
			}
		}
	}
	mvs
}
#[allow(dead_code)]
fn legal_moves_from_mochigoma(t:&Teban,mc:&MochigomaCollections,b:&Banmen) -> Vec<LegalMove> {
	let mut mvs:Vec<LegalMove> = Vec::new();

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
						match b {
							&Banmen(ref kinds) => {
								for x in 0..9 {
									for y in 0..9 {
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
					for m in &MOCHIGOMA_KINDS {
						match mg.get(&m) {
							None | Some(&0) => {
								continue;
							},
							Some(_) => (),
						}
						match b {
							&Banmen(ref kinds) => {
								for x in 0..9 {
									for y in 0..9 {
										let (x,y) = (8 - x, 8 - y);
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
#[allow(dead_code)]
fn legal_moves_all(t:&Teban,banmen:&Banmen,mc:&MochigomaCollections)
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
					mvs.append(&mut legal_moves_with_point(t, banmen, x as u32, y as u32));
				}
			}
		}
	}
	mvs.append(&mut legal_moves_from_mochigoma(t, mc, banmen));
	mvs
}
#[allow(dead_code)]
fn win_only_moves_with_point_and_kind(t:&Teban,banmen:&Banmen,x:u32,y:u32,kind:KomaKind)
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

					legal_moves_with_point_and_kind(t, banmen, x as u32, y as u32, kind)
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

					if ty < 3 {
						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - x as u32,ty as u32 + 1, true),
								Some(ObtainKind::Ou),
						));
					}

					mvs.push(
						LegalMove::To(
							KomaSrcPosition(9 - x as u32,y as u32 + 1),
							KomaDstToPosition(9 - x as u32, ty as u32 + 1, false),
							Some(ObtainKind::Ou),
					));
					mvs
				},
				KomaKind::SKei => {
					legal_moves_with_point_and_kind(t, banmen, x as u32, y as u32, kind)
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

					if ty < 3 {
						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - tx as u32,ty as u32 + 1,true),
								Some(ObtainKind::Ou),
						));
					}

					mvs.push(
						LegalMove::To(
							KomaSrcPosition(9 - x as u32,y as u32 + 1),
							KomaDstToPosition(9 - tx as u32,ty as u32 + 1,false),
							Some(ObtainKind::Ou),
					));
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

					if ty < 3 {
						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - tx as u32,ty as u32 + 1,true),
								Some(ObtainKind::Ou),
						));
					}

					mvs.push(
						LegalMove::To(
							KomaSrcPosition(9 - x as u32,y as u32 + 1),
							KomaDstToPosition(9 - tx as u32,ty as u32 + 1,false),
							Some(ObtainKind::Ou),
					));
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

					legal_moves_with_point_and_kind(t, banmen, x as u32, y as u32, kind)
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

					if ty >= 6 {
						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - x as u32,ty as u32 + 1,true),
								Some(ObtainKind::Ou),
						));
					}

					mvs.push(
						LegalMove::To(
							KomaSrcPosition(9 - x as u32,y as u32 + 1),
							KomaDstToPosition(9 - x as u32,ty as u32 + 1,false),
							Some(ObtainKind::Ou),
					));
					mvs
				},
				KomaKind::GKei => {
					legal_moves_with_point_and_kind(t, banmen, x as u32, y as u32, kind)
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

					if ty >= 6 {
						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - tx as u32,ty as u32 + 1,true),
								Some(ObtainKind::Ou),
						));
					}

					mvs.push(
						LegalMove::To(
							KomaSrcPosition(9 - x as u32,y as u32 + 1),
							KomaDstToPosition(9 - tx as u32,ty as u32 + 1,false),
							Some(ObtainKind::Ou),
					));
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

					if ty >= 6 {
						mvs.push(
							LegalMove::To(
								KomaSrcPosition(9 - x as u32,y as u32 + 1),
								KomaDstToPosition(9 - tx as u32,ty as u32 + 1,true),
								Some(ObtainKind::Ou),
						));
					}

					mvs.push(
						LegalMove::To(
							KomaSrcPosition(9 - x as u32,y as u32 + 1),
							KomaDstToPosition(9 - tx as u32,ty as u32 + 1,false),
							Some(ObtainKind::Ou),
					));
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
#[allow(dead_code)]
fn win_only_moves_with_point(t:&Teban,banmen:&Banmen,x:u32,y:u32)
	-> Vec<LegalMove> {
	match banmen {
		&Banmen(ref kinds) => {
			win_only_moves_with_point_and_kind(t,banmen,x,y,kinds[y as usize][x as usize])
		}
	}
}
#[allow(dead_code)]
fn win_only_moves_with_src(t:&Teban,banmen:&Banmen,src:KomaSrcPosition)
	-> Vec<LegalMove> {
	match src {
		KomaSrcPosition(x,y) => win_only_moves_with_point(t,banmen, 9 - x, y - 1)
	}
}
#[allow(dead_code)]
fn win_only_moves_with_dst_to(t:&Teban,banmen:&Banmen,dst:KomaDstToPosition)
	-> Vec<LegalMove> {
	match dst {
		KomaDstToPosition(x,y,_) => win_only_moves_with_point(t, banmen, 9 - x, y - 1)
	}
}
#[allow(dead_code)]
fn win_only_moves_with_dst_put(t:&Teban,banmen:&Banmen,dst:KomaDstPutPosition)
	-> Vec<LegalMove> {
	match dst {
		KomaDstPutPosition(x,y) => win_only_moves_with_point(t, banmen, 9 - x, y - 1)
	}
}
#[allow(dead_code)]
fn win_only_moves(t:&Teban,banmen:&Banmen)
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
					mvs.append(&mut win_only_moves_with_point(t, banmen, x as u32, y as u32));
				}
			}
		}
	}
	mvs
}
#[allow(dead_code)]
fn oute_only_moves_with_point(t:&Teban,banmen:&Banmen,mc:&MochigomaCollections,x:u32,y:u32)
	-> Vec<LegalMove> {
	legal_moves_with_point(t, banmen, x, y)
		.into_iter().filter(|m| {
				match m {
					&LegalMove::To(_,_,Some(ObtainKind::Ou)) => true,
					&LegalMove::To(ref s,ref d,_) => {
						match apply_move_none_check(banmen, t, mc,&Move::To(*s,*d)) {
							(ref b,_,_) => win_only_moves(t,b).len() > 0
						}
					},
					_ => false,
				}
		}).collect::<Vec<LegalMove>>()
}
#[allow(dead_code)]
fn oute_only_moves_from_banmen(t:&Teban,banmen:&Banmen,mc:&MochigomaCollections)
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
					mvs.append(&mut oute_only_moves_with_point(t, banmen, mc, x as u32, y as u32));
				}
			}
		}
	}
	mvs
}
#[allow(dead_code)]
fn oute_only_moves_from_mochigoma(t:&Teban,mc:&MochigomaCollections,b:&Banmen) -> Vec<LegalMove> {
	legal_moves_from_mochigoma(t, mc, b)
		.into_iter().filter(|m| {
			match m {
				&LegalMove::Put(k,KomaDstPutPosition(x,y)) => {
					win_only_moves_with_point_and_kind(t, b, 9 - x, y - 1, KomaKind::from((*t,k))).len() > 0
				},
				_ => false,
			}
		}).collect::<Vec<LegalMove>>()
}
#[allow(dead_code)]
fn oute_only_moves_all(t:&Teban,banmen:&Banmen,mc:&MochigomaCollections)
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
					mvs.append(&mut oute_only_moves_with_point(t, banmen, mc, x as u32, y as u32));
				}
			}
		}
	}
	mvs.append(&mut oute_only_moves_from_mochigoma(t, mc, banmen));
	mvs
}
#[allow(dead_code)]
fn respond_oute_only_moves_all(t:&Teban,banmen:&Banmen,mc:&MochigomaCollections)
	-> Vec<LegalMove> {
	legal_moves_all(t, banmen, mc)
		.into_iter().filter(|m| {
				match m {
					&LegalMove::To(_,_,Some(ObtainKind::Ou)) => true,
					&LegalMove::To(ref s,ref d,_) => {
						match apply_move_none_check(banmen,t,mc,&Move::To(*s,*d)) {
							(ref b,_,_) => win_only_moves(&t.opposite(),b).len() == 0
						}
					},
					_ => false,
				}
		}).collect::<Vec<LegalMove>>()
}
#[allow(dead_code)]
fn apply_move_none_check(banmen:&Banmen,t:&Teban,mc:&MochigomaCollections,m:&Move)
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
#[allow(dead_code)]
fn apply_valid_move(banmen:&Banmen,t:&Teban,mc:&MochigomaCollections,m:&Move)
	-> Result<(Banmen,MochigomaCollections,Option<MochigomaKind>),ShogiError> {

	match m {
		&Move::To(s,d) => {
			let mvs = legal_moves_from_banmen(t,banmen);

			match find_from_move_to(&mvs,&(s,d)) {
				Some(_) => {
					Ok(apply_move_none_check(banmen,t,mc,m))
				},
				None => {
					Err(ShogiError::InvalidState(String::from(
						"This is not legal move."
					)))
				}
			}
		},
		&Move::Put(k,d) => {
			let mvs = legal_moves_from_mochigoma(t,mc,banmen);

			match find_from_move_put(&mvs,&(k,d)) {
				Some(_) => {
					Ok(apply_move_none_check(banmen,t,mc,m))
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
#[allow(dead_code)]
fn apply_moves(banmen:&Banmen,mut teban:Teban,
					mut mc:MochigomaCollections,
					m:&Vec<Move>,mut mhash:u64,mut shash:u64,
					mut kyokumen_hash_map:TwoKeyHashMap<u64,u32>,
					hasher:&KyokumenHash<u64>)
	-> (Teban,Banmen,MochigomaCollections,u64,u64,TwoKeyHashMap<u64,u32>) {

	let mut banmen = banmen.clone();

	for m in m {
		match apply_move_none_check(&banmen,&teban,&mc,&m) {
			(next,nmc,o) => {
				let m = m.to_applied_move();
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
#[allow(dead_code)]
fn apply_moves_with_callback<T,F>(
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
		match apply_move_none_check(&banmen,&teban,&mc,m) {
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
#[allow(dead_code)]
fn is_nyugyoku_win(banmen:&Banmen,t:&Teban,mc:&MochigomaCollections,limit:&Option<Instant>) -> bool {
	if win_only_moves(&t.opposite(),banmen).len() > 0 {
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
#[allow(dead_code)]
fn responded_oute(banmen:&Banmen,t:&Teban,mc:&MochigomaCollections,m:&Move,nm:&Move)
	-> Result<bool,SelfMatchRunningError> {

	let o = t.opposite();

	if !match m {
		&Move::To(_,ref dst) if win_only_moves_with_dst_to(&o, banmen, *dst).len() == 0 => false,
		&Move::Put(_,ref dst) if win_only_moves_with_dst_put(&o, banmen, *dst).len() == 0 => false,
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
			legal_moves_all(t, banmen, mc).into_iter().filter(|m| {
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
			legal_moves_all(t, banmen, mc).into_iter().filter(|m| {
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
			find_from_move_to(&mvs,&(s,d)).is_some()
		},
		&Move::Put(k,d) => {
			find_from_move_put(&mvs,&(k,d)).is_some()
		}
	})
}
#[allow(dead_code)]
fn is_put_fu_and_mate(banmen:&Banmen,teban:&Teban,mc:&MochigomaCollections,m:&Move) -> bool {
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

			is_oute && legal_moves_all(&teban, banmen, &mc).into_iter().filter(|m| {
				match m {
					&LegalMove::To(_,_,Some(ObtainKind::Ou)) => true,
					m @ _ => {
						match apply_move_none_check(banmen,&teban,&mc,&m.to_move()) {
							(ref b,_,_) => win_only_moves(&teban.opposite(),b).len() == 0
						}
					},
				}
			}).count() == 0
		},
		_ => false,
	}
}
#[allow(dead_code)]
fn is_win(banmen:&Banmen,teban:&Teban,m:&Move) -> bool {
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
#[allow(dead_code)]
fn check_sennichite(_:&Banmen,mhash:u64,shash:u64,
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
#[allow(dead_code)]
fn check_sennichite_by_oute(banmen:&Banmen,teban:&Teban,mhash:u64,shash:u64,
								oute_kyokumen_hash_map:&mut Option<TwoKeyHashMap<u64,u32>>)
	-> bool {

	match *oute_kyokumen_hash_map {
		None if win_only_moves(&teban,banmen).len() > 0 => {
			let mut m = TwoKeyHashMap::new();
			m.insert(mhash,shash,1);
			*oute_kyokumen_hash_map = Some(m);
		},
		Some(ref mut m) => {
			if win_only_moves(&teban,banmen).len() > 0 {
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
