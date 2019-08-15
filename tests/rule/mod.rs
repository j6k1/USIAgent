use std::collections::HashMap;
use std::time::{Instant,Duration};

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
fn test_legal_moves_banmen_with_kaku_leave_opponent_area_sente() {
	const POSITIONS:[(u32,u32); 2] = [
		(2,2),(6,2)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,2),(1,1,true),None),((2,2),(1,1,false),None),
			 ((2,2),(0,0,true),None),((2,2),(0,0,false),None),
			 ((2,2),(3,1,true),None),((2,2),(3,1,false),None),
			 ((2,2),(4,0,true),None),((2,2),(4,0,false),None),
			 ((2,2),(1,3,true),None),((2,2),(1,3,false),None),
			 ((2,2),(0,4,true),None),((2,2),(0,4,false),None),
			 ((2,2),(3,3,true),None),((2,2),(3,3,false),None),
			 ((2,2),(4,4,true),None),((2,2),(4,4,false),None),
			 ((2,2),(5,5,true),None),((2,2),(5,5,false),None),
			 ((2,2),(6,6,true),None),((2,2),(6,6,false),None),
			 ((2,2),(7,7,true),None),((2,2),(7,7,false),None),
			 ((2,2),(8,8,true),None),((2,2),(8,8,false),None),
		],
		vec![((6,2),(5,1,true),None),((6,2),(5,1,false),None),
			 ((6,2),(4,0,true),None),((6,2),(4,0,false),None),
			 ((6,2),(7,1,true),None),((6,2),(7,1,false),None),
			 ((6,2),(8,0,true),None),((6,2),(8,0,false),None),
			 ((6,2),(5,3,true),None),((6,2),(5,3,false),None),
			 ((6,2),(4,4,true),None),((6,2),(4,4,false),None),
			 ((6,2),(3,5,true),None),((6,2),(3,5,false),None),
			 ((6,2),(2,6,true),None),((6,2),(2,6,false),None),
			 ((6,2),(1,7,true),None),((6,2),(1,7,false),None),
			 ((6,2),(0,8,true),None),((6,2),(0,8,false),None),
			 ((6,2),(7,3,true),None),((6,2),(7,3,false),None),
			 ((6,2),(8,4,true),None),((6,2),(8,4,false),None),
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,answer) in POSITIONS.iter().zip(&answer) {
		let answer = answer.into_iter().map(|&m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[p.1 as usize][p.0 as usize] = SKaku;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_leave_opponent_area_gote() {
	const POSITIONS:[(u32,u32); 2] = [
		(2,2),(6,2)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,2),(1,1,true),None),((2,2),(1,1,false),None),
			 ((2,2),(0,0,true),None),((2,2),(0,0,false),None),
			 ((2,2),(3,1,true),None),((2,2),(3,1,false),None),
			 ((2,2),(4,0,true),None),((2,2),(4,0,false),None),
			 ((2,2),(1,3,true),None),((2,2),(1,3,false),None),
			 ((2,2),(0,4,true),None),((2,2),(0,4,false),None),
			 ((2,2),(3,3,true),None),((2,2),(3,3,false),None),
			 ((2,2),(4,4,true),None),((2,2),(4,4,false),None),
			 ((2,2),(5,5,true),None),((2,2),(5,5,false),None),
			 ((2,2),(6,6,true),None),((2,2),(6,6,false),None),
			 ((2,2),(7,7,true),None),((2,2),(7,7,false),None),
			 ((2,2),(8,8,true),None),((2,2),(8,8,false),None),
		],
		vec![((6,2),(5,1,true),None),((6,2),(5,1,false),None),
			 ((6,2),(4,0,true),None),((6,2),(4,0,false),None),
			 ((6,2),(7,1,true),None),((6,2),(7,1,false),None),
			 ((6,2),(8,0,true),None),((6,2),(8,0,false),None),
			 ((6,2),(5,3,true),None),((6,2),(5,3,false),None),
			 ((6,2),(4,4,true),None),((6,2),(4,4,false),None),
			 ((6,2),(3,5,true),None),((6,2),(3,5,false),None),
			 ((6,2),(2,6,true),None),((6,2),(2,6,false),None),
			 ((6,2),(1,7,true),None),((6,2),(1,7,false),None),
			 ((6,2),(0,8,true),None),((6,2),(0,8,false),None),
			 ((6,2),(7,3,true),None),((6,2),(7,3,false),None),
			 ((6,2),(8,4,true),None),((6,2),(8,4,false),None),
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,answer) in POSITIONS.iter().zip(&answer) {
		let answer = answer.into_iter().map(|&m| {
			match m {
				((sx,sy),(dx,dy,n),o) => {
					LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,n),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[8-p.1 as usize][8-p.0 as usize] = GKaku;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
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
fn test_legal_moves_banmen_with_hisha_leave_opponent_area_sente() {
	const POSITIONS:[(u32,u32); 2] = [
		(2,2),(6,2)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,2),(2,1,true),None),((2,2),(2,1,false),None),
			 ((2,2),(2,0,true),None),((2,2),(2,0,false),None),
			 ((2,2),(2,3,true),None),((2,2),(2,3,false),None),
			 ((2,2),(2,4,true),None),((2,2),(2,4,false),None),
			 ((2,2),(2,5,true),None),((2,2),(2,5,false),None),
			 ((2,2),(2,6,true),None),((2,2),(2,6,false),None),
			 ((2,2),(2,7,true),None),((2,2),(2,7,false),None),
			 ((2,2),(2,8,true),None),((2,2),(2,8,false),None),
			 ((2,2),(1,2,true),None),((2,2),(1,2,false),None),
			 ((2,2),(0,2,true),None),((2,2),(0,2,false),None),
			 ((2,2),(3,2,true),None),((2,2),(3,2,false),None),
			 ((2,2),(4,2,true),None),((2,2),(4,2,false),None),
			 ((2,2),(5,2,true),None),((2,2),(5,2,false),None),
			 ((2,2),(6,2,true),None),((2,2),(6,2,false),None),
			 ((2,2),(7,2,true),None),((2,2),(7,2,false),None),
			 ((2,2),(8,2,true),None),((2,2),(8,2,false),None),
		],
		vec![((6,2),(6,1,true),None),((6,2),(6,1,false),None),
			 ((6,2),(6,0,true),None),((6,2),(6,0,false),None),
			 ((6,2),(6,3,true),None),((6,2),(6,3,false),None),
			 ((6,2),(6,4,true),None),((6,2),(6,4,false),None),
			 ((6,2),(6,5,true),None),((6,2),(6,5,false),None),
			 ((6,2),(6,6,true),None),((6,2),(6,6,false),None),
			 ((6,2),(6,7,true),None),((6,2),(6,7,false),None),
			 ((6,2),(6,8,true),None),((6,2),(6,8,false),None),
			 ((6,2),(5,2,true),None),((6,2),(5,2,false),None),
			 ((6,2),(4,2,true),None),((6,2),(4,2,false),None),
			 ((6,2),(3,2,true),None),((6,2),(3,2,false),None),
			 ((6,2),(2,2,true),None),((6,2),(2,2,false),None),
			 ((6,2),(1,2,true),None),((6,2),(1,2,false),None),
			 ((6,2),(0,2,true),None),((6,2),(0,2,false),None),
			 ((6,2),(7,2,true),None),((6,2),(7,2,false),None),
			 ((6,2),(8,2,true),None),((6,2),(8,2,false),None),
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,answer) in POSITIONS.iter().zip(&answer) {
		let answer = answer.into_iter().map(|&m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[p.1 as usize][p.0 as usize] = SHisha;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_leave_opponent_area_gote() {
	const POSITIONS:[(u32,u32); 2] = [
		(2,2),(6,2)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,2),(2,1,true),None),((2,2),(2,1,false),None),
			 ((2,2),(2,0,true),None),((2,2),(2,0,false),None),
			 ((2,2),(2,3,true),None),((2,2),(2,3,false),None),
			 ((2,2),(2,4,true),None),((2,2),(2,4,false),None),
			 ((2,2),(2,5,true),None),((2,2),(2,5,false),None),
			 ((2,2),(2,6,true),None),((2,2),(2,6,false),None),
			 ((2,2),(2,7,true),None),((2,2),(2,7,false),None),
			 ((2,2),(2,8,true),None),((2,2),(2,8,false),None),
			 ((2,2),(1,2,true),None),((2,2),(1,2,false),None),
			 ((2,2),(0,2,true),None),((2,2),(0,2,false),None),
			 ((2,2),(3,2,true),None),((2,2),(3,2,false),None),
			 ((2,2),(4,2,true),None),((2,2),(4,2,false),None),
			 ((2,2),(5,2,true),None),((2,2),(5,2,false),None),
			 ((2,2),(6,2,true),None),((2,2),(6,2,false),None),
			 ((2,2),(7,2,true),None),((2,2),(7,2,false),None),
			 ((2,2),(8,2,true),None),((2,2),(8,2,false),None),
		],
		vec![((6,2),(6,1,true),None),((6,2),(6,1,false),None),
			 ((6,2),(6,0,true),None),((6,2),(6,0,false),None),
			 ((6,2),(6,3,true),None),((6,2),(6,3,false),None),
			 ((6,2),(6,4,true),None),((6,2),(6,4,false),None),
			 ((6,2),(6,5,true),None),((6,2),(6,5,false),None),
			 ((6,2),(6,6,true),None),((6,2),(6,6,false),None),
			 ((6,2),(6,7,true),None),((6,2),(6,7,false),None),
			 ((6,2),(6,8,true),None),((6,2),(6,8,false),None),
			 ((6,2),(5,2,true),None),((6,2),(5,2,false),None),
			 ((6,2),(4,2,true),None),((6,2),(4,2,false),None),
			 ((6,2),(3,2,true),None),((6,2),(3,2,false),None),
			 ((6,2),(2,2,true),None),((6,2),(2,2,false),None),
			 ((6,2),(1,2,true),None),((6,2),(1,2,false),None),
			 ((6,2),(0,2,true),None),((6,2),(0,2,false),None),
			 ((6,2),(7,2,true),None),((6,2),(7,2,false),None),
			 ((6,2),(8,2,true),None),((6,2),(8,2,false),None),
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,answer) in POSITIONS.iter().zip(&answer) {
		let answer = answer.into_iter().map(|&m| {
			match m {
				((sx,sy),(dx,dy,n),o) => {
					LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,n),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[8-p.1 as usize][8-p.0 as usize] = GHisha;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
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

			let nari = (y + o.1) <= 2 || y <= 2;

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

			let nari = (y + o.1) <= 2 || y <= 2;

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

			let nari = (y - o.1) >= 6 || y >= 6;

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

			let nari = (y - o.1) >= 6 || y >= 6;

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

					let nari = (y + o.1) <= 2 || y <= 2;

					if nari {
						answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,true),None));
					}
					answer.push(((6,y as u32),((6+o.0) as u32,(y+o.1) as u32,false),None));
				}

				if y + occ.1 <= 8 {
					let nari = (y + occ.1 - 1) <= 2 || (y + occ.1) <= 2;

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
					let nari = (y + occ.1 - 1) <= 2 || (y + occ.1) <= 2;

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

					let nari = (y + o.1) <= 2 || y <= 2;

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

					let nari = (y + o.1) <= 2 || y <= 2;

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

					let nari = (y + o.1) <= 2 || y <= 2;

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

					let nari = (y - o.1) >= 6 || y >= 6;

					if nari {
						answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,true),None));
					}
					answer.push(((2,y as u32),((2-o.0) as u32,(y-o.1) as u32,false),None));
				}

				if y as i32 - occ.1 >=  0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6 || (y - occ.1) >= 6;

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
					let nari = (y - occ.1 + 1) >= 6 || (y - occ.1) >= 6;

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

					let nari = (y - o.1) >= 6 || y >= 6;

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

					let nari = (y - o.1) >= 6 || y >= 6;

					if nari {
						answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,true),None));
					}
					answer.push(((1,y as u32),((1-o.0) as u32,(y-o.1) as u32,false),None));
				}

				if y as i32 - occ.1 >=  0 && y as i32 - occ.1 <= 7 {
					let nari = (y - occ.1 + 1) >= 6 || (y - occ.1) >= 6;

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
					let nari = (y - occ.1 + 1) >= 6 || (y - occ.1) >= 6;

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

					let nari = (y - o.1) >= 6 || y >= 6;

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

				let nari = (y + o.1) <= 2 || y <= 2;

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

				let nari = (y + o.1) <= 2 || y <= 2;

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

				let nari = (y - o.1) >= 6 || y >= 6;

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

				let nari = (y - o.1) >= 6 || y >= 6;

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
#[test]
fn test_legal_moves_banmen_with_gin_leave_opponent_area_sente() {
	const POSITIONS:[(u32,u32); 2] = [
		(2,2),(6,2)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,2),(1,1,true),None),((2,2),(1,1,false),None),
			 ((2,2),(1,3,true),None),((2,2),(1,3,false),None),
			 ((2,2),(2,1,true),None),((2,2),(2,1,false),None),
			 ((2,2),(3,1,true),None),((2,2),(3,1,false),None),
			 ((2,2),(3,3,true),None),((2,2),(3,3,false),None),
		],
		vec![((6,2),(5,1,true),None),((6,2),(5,1,false),None),
			 ((6,2),(5,3,true),None),((6,2),(5,3,false),None),
			 ((6,2),(6,1,true),None),((6,2),(6,1,false),None),
			 ((6,2),(7,1,true),None),((6,2),(7,1,false),None),
			 ((6,2),(7,3,true),None),((6,2),(7,3,false),None),
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,answer) in POSITIONS.iter().zip(&answer) {
		let answer = answer.into_iter().map(|&m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[p.1 as usize][p.0 as usize] = SGin;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
	}
}
#[test]
fn test_legal_moves_banmen_with_gin_leave_opponent_area_gote() {
	const POSITIONS:[(u32,u32); 2] = [
		(2,2),(6,2)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,2),(1,1,true),None),((2,2),(1,1,false),None),
			 ((2,2),(1,3,true),None),((2,2),(1,3,false),None),
			 ((2,2),(2,1,true),None),((2,2),(2,1,false),None),
			 ((2,2),(3,1,true),None),((2,2),(3,1,false),None),
			 ((2,2),(3,3,true),None),((2,2),(3,3,false),None),
		],
		vec![((6,2),(5,1,true),None),((6,2),(5,1,false),None),
			 ((6,2),(5,3,true),None),((6,2),(5,3,false),None),
			 ((6,2),(6,1,true),None),((6,2),(6,1,false),None),
			 ((6,2),(7,1,true),None),((6,2),(7,1,false),None),
			 ((6,2),(7,3,true),None),((6,2),(7,3,false),None),
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,answer) in POSITIONS.iter().zip(&answer) {
		let answer = answer.into_iter().map(|&m| {
			match m {
				((sx,sy),(dx,dy,n),o) => {
					LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,n),o))
				}
			}
		}).collect::<Vec<LegalMove>>();

		let mut banmen = blank_banmen.clone();

		banmen.0[8-p.1 as usize][8-p.0 as usize] = GGin;

		assert_eq!(
			answer,
			Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		)
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
fn test_legal_moves_from_mochigoma_with_fu_after_change_state_sente() {
	let mvs:Vec<Vec<(Teban,Move)>> = vec![
		vec![
			(Teban::Sente,Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-7,6+1)))
		],
		vec![
			(Teban::Sente,Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-6,0+1)))
		],
		vec![
			(Teban::Sente,Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-7,0+1)))
		],
		vec![
			(Teban::Sente,Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-7,3+1))),
			(Teban::Sente,Move::To(KomaSrcPosition(9-7,3+1),KomaDstToPosition(9-7,2+1,true)))
		],
		vec![
			(Teban::Sente,Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-7,6+1))),
			(Teban::Gote,Move::To(KomaSrcPosition(9-7,1+1),KomaDstToPosition(9-7,6+1,false)))
		]
	];

	let answer = [
		0,5,0,4,5
	];

	for (mvs,answer) in mvs.iter().zip(&answer) {
		let mut banmen = rule::BANMEN_START_POS.clone();

		banmen.0[6][6] = Blank;
		banmen.0[6][7] = Blank;
		banmen.0[8][6] = Blank;
		banmen.0[2][7] = Blank;
		banmen.0[0][7] = Blank;

		let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

		ms.insert(MochigomaKind::Fu, 2);

		let mut mc:MochigomaCollections = MochigomaCollections::Pair(ms,HashMap::new());

		let mut state = State::new(banmen.clone());

		for (t,m) in mvs {
			match Rule::apply_move_none_check(&state,*t,&mc,m.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}
		}

		assert_eq!(*answer,
			Rule::legal_moves_from_mochigoma(Teban::Sente,&mc,&state).into_iter().filter(|m| {
				match m {
					rule::LegalMove::Put(m) => m.dst() / 9 == 7,
					_ => false
				}
			}).count());
	}
}
#[test]
fn test_legal_moves_from_mochigoma_with_fu_after_change_state_gote() {
	let mvs:Vec<Vec<(Teban,Move)>> = vec![
		vec![
			(Teban::Gote,Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-(8-7),(8-6)+1)))
		],
		vec![
			(Teban::Gote,Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-(8-6),(8-0)+1)))
		],
		vec![
			(Teban::Gote,Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-(8-7),(8-0)+1)))
		],
		vec![
			(Teban::Gote,Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-(8-7),(8-3)+1))),
			(Teban::Gote,Move::To(KomaSrcPosition(9-(8-7),(8-3)+1),KomaDstToPosition(9-(8-7),(8-2)+1,true)))
		],
		vec![
			(Teban::Gote,Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-(8-7),(8-6)+1))),
			(Teban::Sente,Move::To(KomaSrcPosition(9-(8-7),(8-1)+1),KomaDstToPosition(9-(8-7),(8-6)+1,false)))
		]
	];

	let answer = [
		0,5,0,4,5
	];

	for (mvs,answer) in mvs.iter().zip(&answer) {
		let mut banmen = rule::BANMEN_START_POS.clone();

		banmen.0[8-6][8-6] = Blank;
		banmen.0[8-6][8-7] = Blank;
		banmen.0[8-8][8-6] = Blank;
		banmen.0[8-2][8-7] = Blank;
		banmen.0[8-0][8-7] = Blank;

		let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

		mg.insert(MochigomaKind::Fu, 2);

		let mut mc:MochigomaCollections = MochigomaCollections::Pair(HashMap::new(),mg);

		let mut state = State::new(banmen.clone());

		for (t,m) in mvs {
			match Rule::apply_move_none_check(&state,*t,&mc,m.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}
		}

		assert_eq!(*answer,
			Rule::legal_moves_from_mochigoma(Teban::Gote,&mc,&state).into_iter().filter(|m| {
				match m {
					rule::LegalMove::Put(m) => m.dst() / 9 == 1,
					_ => false
				}
			}).count());
	}
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
fn test_win_only_moves_some_moves_sente_impl(ox:u32,oy:u32,positions:Vec<(u32,u32,bool)>,kind:KomaKind) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = GOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer = if p.2 {
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
fn test_win_only_moves_some_moves_gote_impl(ox:u32,oy:u32,positions:Vec<(u32,u32,bool)>,kind:KomaKind) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = SOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer = if p.2 {
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
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5,false)],SFu)
}
#[test]
fn test_win_only_moves_none_moves_with_fu_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(3,5),(4,6),(5,5)],SFu)
}
#[test]
fn test_win_only_moves_nari_moves_with_fu_sente() {
	test_win_only_moves_some_moves_sente_impl(4,2,vec![(4,3,true)],SFu)
}
#[test]
fn test_win_only_moves_some_moves_with_fu_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5,false)],GFu)
}
#[test]
fn test_win_only_moves_none_moves_with_fu_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-3,8-5),(8-4,8-6),(8-5,8-5)],GFu)
}
#[test]
fn test_win_only_moves_nari_moves_with_fu_gote() {
	test_win_only_moves_some_moves_gote_impl(4,8-2,vec![(8-4,8-3,true)],GFu)
}
#[test]
fn test_win_only_moves_some_moves_with_gin_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5,false),(3,5,false),(5,5,false),(3,3,false),(5,3,false)],SGin)
}
#[test]
fn test_win_only_moves_none_moves_with_gin_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(4,3),(3,4),(5,4),(4,6)],SGin)
}
#[test]
fn test_win_only_moves_nari_moves_with_gin_sente() {
	test_win_only_moves_some_moves_sente_impl(4,2,vec![(4,3,true),(3,3,true),(5,3,true),(3,1,true),(5,1,true)],SGin)
}
#[test]
fn test_win_only_moves_some_moves_with_gin_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-3,false),(8-5,8-3,false)],GGin)
}
#[test]
fn test_win_only_moves_none_moves_with_gin_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-4,8-3),(8-3,8-4),(8-5,8-4),(8-4,8-6)],GGin)
}
#[test]
fn test_win_only_moves_nari_moves_with_gin_gote() {
	test_win_only_moves_some_moves_gote_impl(4,8-2,vec![(8-4,8-3,true),(8-3,8-3,true),(8-5,8-3,true),(8-3,8-1,true),(8-5,8-1,true)],GGin)
}
#[test]
fn test_win_only_moves_some_moves_with_kin_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5,false),(3,5,false),(5,5,false),(3,4,false),(5,4,false),(4,3,false)],SKin)
}
#[test]
fn test_win_only_moves_none_moves_with_kin_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(3,3),(5,3),(4,6)],SKin)
}
#[test]
fn test_win_only_moves_some_moves_with_kin_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-4,false),(8-5,8-4,false),(8-4,8-3,false)],GKin)
}
#[test]
fn test_win_only_moves_none_moves_with_kin_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-3,8-3),(8-5,8-3),(8-4,8-6)],GKin)
}
#[test]
fn test_win_only_moves_some_moves_with_ou_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5,false),(3,5,false),(5,5,false),(3,4,false),(5,4,false),(3,3,false),(4,3,false),(5,3,false)],SOu)
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
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-4,false),(8-5,8-4,false),(8-3,8-3,false),(8-4,8-3,false),(8-5,8-3,false)],GOu)
}
#[test]
fn test_win_only_moves_some_moves_with_fu_nari_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5,false),(3,5,false),(5,5,false),(3,4,false),(5,4,false),(4,3,false)],SFuN)
}
#[test]
fn test_win_only_moves_none_moves_with_fu_nari_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(3,3),(5,3),(4,6)],SFuN)
}
#[test]
fn test_win_only_moves_some_moves_with_fu_nari_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-4,false),(8-5,8-4,false),(8-4,8-3,false)],GFuN)
}
#[test]
fn test_win_only_moves_none_moves_with_fu_nari_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-3,8-3),(8-5,8-3),(8-4,8-6)],GFuN)
}
#[test]
fn test_win_only_moves_some_moves_with_gin_nari_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5,false),(3,5,false),(5,5,false),(3,4,false),(5,4,false),(4,3,false)],SGinN)
}
#[test]
fn test_win_only_moves_none_moves_with_gin_nari_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(3,3),(5,3),(4,6)],SGinN)
}
#[test]
fn test_win_only_moves_some_moves_with_gin_nari_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-4,false),(8-5,8-4,false),(8-4,8-3,false)],GGinN)
}
#[test]
fn test_win_only_moves_none_moves_with_gin_nari_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-3,8-3),(8-5,8-3),(8-4,8-6)],GGinN)
}
#[test]
fn test_win_only_moves_some_moves_with_kyou_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,8,false)],SKyou)
}
#[test]
fn test_win_only_moves_none_moves_with_kyou_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(3,8),(5,8)],SKyou)
}
#[test]
fn test_win_only_moves_nari_moves_with_kyou_sente() {
	test_win_only_moves_some_moves_sente_impl(4,2,vec![(4,8,true)],SKyou)
}
#[test]
fn test_win_only_moves_some_moves_with_kyou_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(4,0,false)],GKyou)
}
#[test]
fn test_win_only_moves_none_moves_with_kyou_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-3,0),(8-5,0)],GKyou)
}
#[test]
fn test_win_only_moves_nari_moves_with_kyou_gote() {
	test_win_only_moves_some_moves_gote_impl(4,8-2,vec![(4,0,true)],GKyou)
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
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(4,5,false),(3,5,false),(5,5,false),(3,4,false),(5,4,false),(4,3,false)],SKyouN)
}
#[test]
fn test_win_only_moves_none_moves_with_kyou_nari_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(3,3),(5,3),(4,6)],SKyouN)
}
#[test]
fn test_win_only_moves_some_moves_with_kyou_nari_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-4,8-5,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-4,false),(8-5,8-4,false),(8-4,8-3,false)],GKyouN)
}
#[test]
fn test_win_only_moves_none_moves_with_kyou_nari_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-3,8-3),(8-5,8-3),(8-4,8-6)],GKyouN)
}
#[test]
fn test_win_only_moves_some_moves_with_kei_sente() {
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(3,6,false),(5,6,false)],SKei)
}
#[test]
fn test_win_only_moves_none_moves_with_kei_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(4,6)],SKei)
}
#[test]
fn test_win_only_moves_nari_moves_with_kei_sente() {
	test_win_only_moves_some_moves_sente_impl(4,2,vec![(3,4,true),(5,4,true)],SKei)
}
#[test]
fn test_win_only_moves_some_moves_with_kei_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8-3,8-6,false)],GKei)
}
#[test]
fn test_win_only_moves_none_moves_with_kei_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(4,8-6)],GKei)
}
#[test]
fn test_win_only_moves_nari_moves_with_kei_gote() {
	test_win_only_moves_some_moves_gote_impl(4,8-2,vec![(8-3,8-4,true),(8-5,8-4,true)],GKei)
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
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(0,0,true),(0,8,false),(8,0,true),(8,8,false)],SKaku)
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(1,0),(0,7),(7,0),(7,8)],SKaku)
}
#[test]
fn test_win_only_moves_nari_moves_with_kaku_sente() {
	test_win_only_moves_some_moves_sente_impl(4,2,vec![(2,0,true),(2,4,true),(6,0,true),(6,4,true)],SKaku)
}
#[test]
fn test_win_only_moves_some_moves_with_kaku_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8,8,true),(8,0,false),(0,8,true),(0,0,false)],GKaku)
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-1,8),(8,8-7),(8-7,8),(8-7,8)],GKaku)
}
#[test]
fn test_win_only_moves_nari_moves_with_kaku_gote() {
	test_win_only_moves_some_moves_gote_impl(4,8-2,vec![(8-2,8,true),(8-2,8-4,true,),(8-6,8,true),(8-6,8-4,true)],GKaku)
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
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(0,4,false),(4,0,true),(8,4,false),(4,8,false)],SHisha)
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(0,3),(3,0),(8,5),(5,8)],SHisha)
}
#[test]
fn test_win_only_moves_nari_moves_with_hisha_sente() {
	test_win_only_moves_some_moves_sente_impl(4,2,vec![(0,2,true),(4,0,true),(8,2,true),(4,8,true)],SHisha)
}
#[test]
fn test_win_only_moves_some_moves_with_hisha_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8,8-4,false),(8-4,8,true),(0,8-4,false),(8-4,0,false)],GHisha)
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_gote() {
	test_win_only_moves_none_moves_gote_impl(4,4,vec![(8-0,8-3),(8-3,8-0),(8-8,8-5),(8-5,8-8)],GHisha)
}
#[test]
fn test_win_only_moves_nari_moves_with_hisha_gote() {
	test_win_only_moves_some_moves_gote_impl(4,8-2,vec![(8-0,8-2,true),(8-4,8-0,true),(8-8,8-2,true),(8-4,8-8,true)],GHisha)
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
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(0,0,false),(0,8,false),(8,0,false),(8,8,false),(4,5,false),(3,4,false),(5,4,false),(4,3,false)],SKakuN)
}
#[test]
fn test_win_only_moves_none_moves_with_kaku_nari_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(1,0),(0,7),(7,0),(7,8),(4,6),(2,4),(6,4),(4,2)],SKakuN)
}
#[test]
fn test_win_only_moves_some_moves_with_kaku_nari_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8,8,false),(8,0,false),(0,8,false),(0,0,false),(8-4,8-5,false),(8-3,8-4,false),(8-5,8-4,false),(8-4,8-3,false)],GKakuN)
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
	test_win_only_moves_some_moves_sente_impl(4,4,vec![(0,4,false),(4,0,false),(8,4,false),(4,8,false),(3,5,false),(5,5,false),(3,3,false),(5,3,false)],SHishaN)
}
#[test]
fn test_win_only_moves_none_moves_with_hisha_nari_sente() {
	test_win_only_moves_none_moves_sente_impl(4,4,vec![(0,3),(3,0),(8,5),(5,8),(2,6),(6,6),(2,2),(6,2)],SHishaN)
}
#[test]
fn test_win_only_moves_some_moves_with_hisha_nari_gote() {
	test_win_only_moves_some_moves_gote_impl(4,4,vec![(8,8-4,false),(8-4,8,false),(0,8-4,false),(8-4,0,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-3,false),(8-5,8-3,false)],GHishaN)
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
fn test_oute_only_moves_win_only_result_some_moves_sente_impl(ox:u32,oy:u32,positions:Vec<(u32,u32,bool)>,kind:KomaKind) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = GOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer = if p.2 {
			vec![
				((p.0,p.1),(ox,oy,true),Some(ObtainKind::Ou)),
				((p.0,p.1),(ox,oy,false),Some(ObtainKind::Ou))
			]
		} else {
			vec![
				((p.0,p.1),(ox,oy,false),Some(ObtainKind::Ou))
			]
		};

		assert_eq!(answer.clone().into_iter().map(|m| LegalMove::from(m)).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| LegalMove::from(m)).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_oute_only_moves_win_only_result_some_moves_gote_impl(ox:u32,oy:u32,positions:Vec<(u32,u32,bool)>,kind:KomaKind) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = SOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer = if p.2 {
			vec![
				((p.0,p.1),(ox,oy,true),Some(ObtainKind::Ou)),
				((p.0,p.1),(ox,oy,false),Some(ObtainKind::Ou))
			]
		} else {
			vec![
				((p.0,p.1),(ox,oy,false),Some(ObtainKind::Ou))
			]
		};

		assert_eq!(answer.clone().into_iter().map(|m| LegalMove::from(m)).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| LegalMove::from(m)).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_fu_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5,false)],SFu)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_fu_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,2,vec![(4,3,true)],SFu)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_fu_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5,false)],GFu)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_fu_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,8-2,vec![(8-4,8-3,true)],GFu)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_gin_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5,false),(3,5,false),(5,5,false),(3,3,false),(5,3,false)],SGin)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_gin_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,2,vec![(4,3,true),(3,3,true),(5,3,true),(3,1,true),(5,1,true)],SGin)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_gin_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-3,false),(8-5,8-3,false)],GGin)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_gin_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,8-2,vec![(8-4,8-3,true),(8-3,8-3,true),(8-5,8-3,true),(8-3,8-1,true),(8-5,8-1,true)],GGin)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kin_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5,false),(3,5,false),(5,5,false),(3,4,false),(5,4,false),(4,3,false)],SKin)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kin_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-4,false),(8-5,8-4,false),(8-4,8-3,false)],GKin)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_ou_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5,false),(3,5,false),(5,5,false),(3,4,false),(5,4,false),(3,3,false),(4,3,false),(5,3,false)],SOu)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_ou_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-4,false),(8-5,8-4,false),(8-3,8-3,false),(8-4,8-3,false),(8-5,8-3,false)],GOu)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_fu_nari_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5,false),(3,5,false),(5,5,false),(3,4,false),(5,4,false),(4,3,false)],SFuN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_fu_nari_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-4,false),(8-5,8-4,false),(8-4,8-3,false)],GFuN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_gin_nari_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5,false),(3,5,false),(5,5,false),(3,4,false),(5,4,false),(4,3,false)],SGinN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_gin_nari_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-4,false),(8-5,8-4,false),(8-4,8-3,false)],GGinN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kyou_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,8,false)],SKyou)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_kyou_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,2,vec![(4,8,true)],SKyou)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kyou_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(4,0,false)],GKyou)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_kyou_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,8-2,vec![(4,0,true)],GKyou)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kyou_nari_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(4,5,false),(3,5,false),(5,5,false),(3,4,false),(5,4,false),(4,3,false)],SKyouN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kyou_nari_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-4,8-5,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-4,false),(8-5,8-4,false),(8-4,8-3,false)],GKyouN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kei_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(3,6,false),(5,6,false)],SKei)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_kei_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,2,vec![(3,4,true),(5,4,true)],SKei)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kei_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8-3,8-6,false)],GKei)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_kei_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,8-2,vec![(8-3,8-4,true),(8-5,8-4,true)],GKei)
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

			assert_eq!(answer.clone().into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>(),
				Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);

			assert_eq!(answer.into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>(),
				Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
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

			assert_eq!(answer.clone().into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>(),
				Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);

			assert_eq!(answer.into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>(),
				Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kaku_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(0,0,true),(0,8,false),(8,0,true),(8,8,false)],SKaku)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_kaku_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,2,vec![(2,0,true),(2,4,true),(6,0,true),(6,4,true)],SKaku)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kaku_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8,8,true),(8,0,false),(0,8,true),(0,0,false)],GKaku)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_kaku_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,8-2,vec![(8-2,8,true),(8-2,8-4,true),(8-6,8,true),(8-6,8-4,true)],GKaku)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_hisha_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(0,4,false),(4,0,true),(8,4,false),(4,8,false)],SHisha)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_hisha_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,2,vec![(0,2,true),(4,0,true),(8,2,true),(4,8,true)],SHisha)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_hisha_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8,8-4,false),(8-4,8,true),(0,8-4,false),(8-4,0,false)],GHisha)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kaku_nari_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(0,0,false),(0,8,false),(8,0,false),(8,8,false),(4,5,false),(3,4,false),(5,4,false),(4,3,false)],SKakuN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_kaku_nari_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8,8,false),(8,0,false),(0,8,false),(0,0,false),(8-4,8-5,false),(8-3,8-4,false),(8-5,8-4,false),(8-4,8-3,false)],GKakuN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_hisha_nari_sente() {
	test_oute_only_moves_win_only_result_some_moves_sente_impl(4,4,vec![(0,4,false),(4,0,false),(8,4,false),(4,8,false),(3,5,false),(5,5,false),(3,3,false),(5,3,false)],SHishaN)
}
#[test]
fn test_oute_only_moves_win_only_result_some_moves_with_hisha_nari_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,4,vec![(8,8-4,false),(8-4,8,false),(0,8-4,false),(8-4,0,false),(8-3,8-5,false),(8-5,8-5,false),(8-3,8-3,false),(8-5,8-3,false)],GHishaN)
}
#[test]
fn test_oute_only_moves_win_only_result_nari_moves_with_hisha_gote() {
	test_oute_only_moves_win_only_result_some_moves_gote_impl(4,8-2,vec![(8-0,8-2,true),(8-4,8-0,true),(8-8,8-2,true),(8-4,8-8,true)],GHisha)
}
fn test_oute_only_moves_none_moves_sente_impl(ox:u32,oy:u32,positions:Vec<(u32,u32)>,kind:KomaKind) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = GOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_oute_only_moves_none_moves_gote_impl(ox:u32,oy:u32,positions:Vec<(u32,u32)>,kind:KomaKind) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for p in &positions {
		let mut banmen = blank_banmen.clone();

		banmen.0[oy as usize][ox as usize] = SOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kyou_occupied_self_sente() {
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

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kyou_occupied_self_gote() {
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

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_fu_sente() {
	test_oute_only_moves_none_moves_sente_impl(4,4,vec![(3,5),(4,7),(5,5)],SFu)
}
#[test]
fn test_oute_only_moves_none_moves_with_gin_sente() {
	test_oute_only_moves_none_moves_sente_impl(4,4,vec![(4,3),(1,4),(7,4),(4,7)],SGin)
}
#[test]
fn test_oute_only_moves_none_moves_with_fu_gote() {
	test_oute_only_moves_none_moves_gote_impl(4,4,vec![(8-3,8-5),(8-4,8-7),(8-5,8-5)],GFu)
}
#[test]
fn test_oute_only_moves_none_moves_with_gin_gote() {
	test_oute_only_moves_none_moves_gote_impl(4,4,vec![(8-4,8-3),(8-1,8-4),(8-7,8-4),(8-4,8-7)],GGin)
}
#[test]
fn test_oute_only_moves_none_moves_with_kin_sente() {
	test_oute_only_moves_none_moves_sente_impl(4,4,vec![(2,2),(6,2),(4,7)],SKin)
}
#[test]
fn test_oute_only_moves_none_moves_with_kin_gote() {
	test_oute_only_moves_none_moves_gote_impl(4,4,vec![(8-2,8-2),(8-6,8-2),(8-4,8-7)],GKin)
}
#[test]
fn test_oute_only_moves_none_moves_with_ou_gote() {
	test_oute_only_moves_none_moves_gote_impl(4,4,vec![(4,7)],GOu)
}
#[test]
fn test_oute_only_moves_none_moves_with_fu_nari_sente() {
	test_oute_only_moves_none_moves_sente_impl(4,4,vec![(2,2),(6,2),(4,7)],SFuN)
}
#[test]
fn test_oute_only_moves_none_moves_with_fu_nari_gote() {
	test_oute_only_moves_none_moves_gote_impl(4,4,vec![(8-2,8-2),(8-6,8-2),(8-4,8-7)],GFuN)
}
#[test]
fn test_oute_only_moves_none_moves_with_gin_nari_sente() {
	test_oute_only_moves_none_moves_sente_impl(4,4,vec![(2,2),(6,2),(4,7)],SGinN)
}
#[test]
fn test_oute_only_moves_none_moves_with_gin_nari_gote() {
	test_oute_only_moves_none_moves_gote_impl(4,4,vec![(8-2,8-2),(8-6,8-2),(8-4,8-7)],GGinN)
}
#[test]
fn test_oute_only_moves_none_moves_with_kyou_sente() {
	test_oute_only_moves_none_moves_sente_impl(4,4,vec![(3,8),(5,8)],SKyou)
}
#[test]
fn test_oute_only_moves_none_moves_with_kyou_gote() {
	test_oute_only_moves_none_moves_gote_impl(4,4,vec![(8-3,0),(8-5,0)],GKyou)
}
#[test]
fn test_oute_only_moves_none_moves_with_kyou_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 2] = [
		(0,8),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 2] = [
		((0,6),(0,7)),((8,6),(8,7))
	];

	const OU_POSITIONS:[(usize,usize); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKyou;
		banmen.0[(o.0).1][(o.0).0] = SGin;
		banmen.0[(o.1).1][(o.1).0] = SGin;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![];

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
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

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 2] = [
		((0,6),(0,7)),((8,6),(8,7))
	];

	const OU_POSITIONS:[(usize,usize); 2] = [
		(0,0),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKyou;
		banmen.0[8-(o.0).1][8-(o.0).0] = GGin;
		banmen.0[8-(o.1).1][8-(o.1).0] = GGin;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![];

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kyou_nari_sente() {
	test_oute_only_moves_none_moves_sente_impl(4,4,vec![(2,2),(6,2),(4,7)],SKyouN)
}
#[test]
fn test_oute_only_moves_none_moves_with_kyou_nari_gote() {
	test_oute_only_moves_none_moves_gote_impl(4,4,vec![(8-2,8-2),(8-6,8-2),(8-4,8-7)],GKyouN)
}
#[test]
fn test_oute_only_moves_none_moves_with_kei_sente() {
	test_oute_only_moves_none_moves_sente_impl(4,4,vec![(4,6)],SKei)
}
#[test]
fn test_oute_only_moves_none_moves_with_kei_gote() {
	test_oute_only_moves_none_moves_gote_impl(4,4,vec![(4,8-6)],GKei)
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((1,1),(2,2)),((2,6),(3,5)),((7,1),(6,2)),((7,7),(6,6))
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(5,5),(2,6),(3,5),(3,3)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKaku;
		banmen.0[(o.0).1][(o.0).0] = SGin;
		banmen.0[(o.1).1][(o.1).0] = SGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((1,1),(2,2)),((2,6),(3,5)),((7,1),(6,2)),((6,6),(5,5))
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(5,5),(7,1),(6,2),(2,2)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKaku;
		banmen.0[8-(o.0).1][8-(o.0).0] = GGin;
		banmen.0[8-(o.1).1][8-(o.1).0] = GGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((2,2),(3,3)),((2,6),(3,5)),((5,3),(6,2)),((5,5),(6,6))
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(6,6),(6,2),(2,6),(2,2)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKaku;
		banmen.0[(o.0).1][(o.0).0] = GGin;
		banmen.0[(o.1).1][(o.1).0] = GGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((3,3),(2,2)),((3,5),(2,6)),((5,3),(6,2)),((5,5),(6,6))
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKaku;
		banmen.0[8-(o.0).1][8-(o.0).0] = SGin;
		banmen.0[8-(o.1).1][8-(o.1).0] = SGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_turn_move_and_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(0,8),(8,0),(8,8),(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 8] = [
		(1,1),(1,7),(6,2),(7,7),(6,2),(2,2),(7,7),(6,2)
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,0),(0,0),(8,8),(8,0),(8,0),(0,0),(8,8),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKaku;
		banmen.0[o.1][o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_turn_move_and_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(0,8),(8,0),(8,8),(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 8] = [
		(1,1),(1,7),(7,1),(7,7),(6,2),(2,2),(6,6),(6,2)
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,0),(0,0),(8,8),(8,0),(8,0),(0,0),(8,8),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKaku;
		banmen.0[8-o.1][8-o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_turn_move_and_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(0,8),(8,0),(8,8),(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 8] = [
		(1,1),(1,7),(7,1),(7,7),(6,2),(2,2),(6,6),(6,2)
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,0),(0,0),(8,8),(8,0),(8,0),(0,0),(8,8),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKaku;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_turn_move_and_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(0,8),(8,0),(8,8),(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 8] = [
		(1,1),(1,7),(7,1),(7,7),(7,1),(1,1),(7,7),(7,1)
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,0),(0,0),(8,8),(8,0),(8,0),(0,0),(8,8),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKaku;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 5] = [
		(1,8),(7,0),(1,0),(8,7),(0,7)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 5] = [
		((1,3),(1,4)),((7,5),(7,4)),((1,5),(1,4)),((4,7),(3,7)),((4,7),(5,7))
	];

	const OU_POSITIONS:[(usize,usize); 5] = [
		(1,0),(7,8),(1,8),(0,7),(8,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHisha;
		banmen.0[(o.0).1][(o.0).0] = SGin;
		banmen.0[(o.1).1][(o.1).0] = SGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 5] = [
		(1,8),(7,0),(1,0),(8,7),(0,7)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 5] = [
		((1,3),(1,4)),((7,5),(7,4)),((1,5),(1,4)),((4,7),(3,7)),((5,7),(4,7))
	];

	const OU_POSITIONS:[(usize,usize); 5] = [
		(1,0),(7,8),(1,8),(0,7),(8,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHisha;
		banmen.0[8-(o.0).1][8-(o.0).0] = GGin;
		banmen.0[8-(o.1).1][8-(o.1).0] = GGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 5] = [
		(1,8),(7,0),(1,0),(8,7),(0,7)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 5] = [
		((1,3),(1,4)),((7,5),(7,4)),((1,5),(1,4)),((4,7),(3,7)),((4,7),(5,7))
	];

	const OU_POSITIONS:[(usize,usize); 5] = [
		(1,0),(7,8),(1,8),(0,7),(8,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHisha;
		banmen.0[(o.0).1][(o.0).0] = GGin;
		banmen.0[(o.1).1][(o.1).0] = GGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 5] = [
		(1,8),(7,0),(1,0),(8,7),(0,7)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 5] = [
		((1,3),(1,4)),((7,5),(7,4)),((1,5),(1,4)),((4,7),(3,7)),((5,7),(4,7))
	];

	const OU_POSITIONS:[(usize,usize); 5] = [
		(1,0),(7,8),(1,8),(0,7),(8,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHisha;
		banmen.0[8-(o.0).1][8-(o.0).0] = SGin;
		banmen.0[8-(o.1).1][8-(o.1).0] = SGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_turn_move_and_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(8,0),(0,8),(8,8),(0,0),(8,0),(0,8),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 8] = [
		((0,1),(1,0)),((8,1),(7,0)),((0,1),(1,8)),((0,3),(3,0)),((5,8),(8,5)),((3,8),(0,5)),((5,0),(8,3)),((3,0),(0,3))
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,8),(0,8),(8,0),(0,0),(8,8),(0,8),(8,0),(0,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHisha;
		banmen.0[(o.0).1][(o.0).0] = SGin;
		banmen.0[(o.1).1][(o.1).0] = SGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_turn_move_and_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(8,0),(0,8),(8,8),(0,0),(8,0),(0,8),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 8] = [
		((0,1),(1,0)),((8,1),(7,0)),((0,1),(1,8)),((0,3),(3,0)),((5,8),(8,5)),((3,8),(0,5)),((5,0),(8,3)),((3,0),(0,3))
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,8),(0,8),(8,0),(0,0),(8,8),(0,8),(8,0),(0,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHisha;
		banmen.0[8-(o.0).1][8-(o.0).0] = GGin;
		banmen.0[8-(o.1).1][8-(o.1).0] = GGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_turn_move_and_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(8,0),(0,8),(8,8),(0,0),(8,0),(0,8),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 8] = [
		((0,1),(1,0)),((8,1),(7,0)),((0,1),(1,8)),((0,3),(3,0)),((5,8),(8,5)),((3,8),(0,5)),((5,0),(8,3)),((3,0),(0,3))
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,8),(0,8),(8,0),(0,0),(8,8),(0,8),(8,0),(0,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHisha;
		banmen.0[(o.0).1][(o.0).0] = GGin;
		banmen.0[(o.1).1][(o.1).0] = GGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_turn_move_and_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(8,0),(0,8),(8,8),(0,0),(8,0),(0,8),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 8] = [
		((0,1),(1,0)),((8,1),(7,0)),((0,1),(1,8)),((0,3),(3,0)),((5,8),(8,5)),((3,8),(0,5)),((5,0),(8,3)),((3,0),(0,3))
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,8),(0,8),(8,0),(0,0),(8,8),(0,8),(8,0),(0,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHisha;
		banmen.0[8-(o.0).1][8-(o.0).0] = SGin;
		banmen.0[8-(o.1).1][8-(o.1).0] = SGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_nari_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((3,3),(2,2)),((3,5),(2,6)),((5,3),(6,2)),((5,5),(6,6))
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKakuN;
		banmen.0[(o.0).1][(o.0).0] = SFu;
		banmen.0[(o.1).1][(o.1).0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_nari_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((3,3),(2,2)),((3,5),(2,6)),((5,3),(6,2)),((5,5),(6,6))
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKakuN;
		banmen.0[8-(o.0).1][8-(o.0).0] = GFu;
		banmen.0[8-(o.1).1][8-(o.1).0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_nari_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((3,3),(2,2)),((3,5),(2,6)),((5,3),(6,2)),((5,5),(6,6))
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKakuN;
		banmen.0[(o.0).1][(o.0).0] = GGin;
		banmen.0[(o.1).1][(o.1).0] = GGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_nari_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((3,3),(2,2)),((3,5),(2,6)),((5,3),(6,2)),((5,5),(6,6))
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(4,4),(4,4),(4,4),(4,4)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKakuN;
		banmen.0[8-(o.0).1][8-(o.0).0] = SGin;
		banmen.0[8-(o.1).1][8-(o.1).0] = SGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_nari_turn_move_and_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(0,8),(8,0),(8,8),(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 8] = [
		(2,2),(1,7),(6,2),(7,7),(6,2),(2,2),(7,7),(6,2)
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,0),(0,0),(8,8),(8,0),(8,0),(0,0),(8,8),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKakuN;
		banmen.0[o.1][o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_nari_turn_move_and_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(0,8),(8,0),(8,8),(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 8] = [
		(1,1),(1,7),(7,1),(7,7),(6,2),(2,2),(6,6),(6,2)
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,0),(0,0),(8,8),(8,0),(8,0),(0,0),(8,8),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKakuN;
		banmen.0[8-o.1][8-o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_nari_turn_move_and_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(0,8),(8,0),(8,8),(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 8] = [
		(1,1),(1,7),(7,1),(7,7),(6,2),(2,2),(6,6),(6,2)
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,0),(0,0),(8,8),(8,0),(8,0),(0,0),(8,8),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKakuN;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_kaku_nari_turn_move_and_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(0,8),(8,0),(8,8),(0,0),(0,8),(8,0),(8,8)
	];

	const OCC_POSITIONS:[(usize,usize); 8] = [
		(1,1),(1,7),(7,1),(7,7),(7,1),(1,1),(7,7),(7,1)
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,0),(0,0),(8,8),(8,0),(8,0),(0,0),(8,8),(8,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKakuN;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_nari_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((1,4),(1,3)),((7,4),(7,5)),((1,5),(1,6)),((1,7),(2,7))
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHishaN;
		banmen.0[(o.0).1][(o.0).0] = SFu;
		banmen.0[(o.1).1][(o.1).0] = SFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![];

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
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

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((1,4),(1,3)),((7,4),(7,5)),((1,5),(1,6)),((1,7),(2,7))
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHishaN;
		banmen.0[8-(o.0).1][8-(o.0).0] = GFu;
		banmen.0[8-(o.1).1][8-(o.1).0] = GFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![];

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
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

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((1,4),(1,3)),((7,4),(7,5)),((1,5),(1,6)),((1,7),(2,7))
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHishaN;
		banmen.0[(o.0).1][(o.0).0] = GFu;
		banmen.0[(o.1).1][(o.1).0] = GFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![];

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
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

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 4] = [
		((1,4),(1,3)),((7,4),(7,5)),((1,5),(1,6)),((1,7),(2,7))
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,8),(1,8),(0,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHishaN;
		banmen.0[8-(o.0).1][8-(o.0).0] = SFu;
		banmen.0[8-(o.1).1][8-(o.1).0] = SFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![];

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_nari_turn_move_and_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(8,0),(0,8),(8,8),(0,0),(8,0),(0,8),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 8] = [
		((0,1),(1,0)),((8,1),(7,0)),((0,1),(1,8)),((0,3),(3,0)),((5,8),(8,5)),((3,8),(0,5)),((5,0),(8,3)),((3,0),(0,3))
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,8),(0,8),(8,0),(0,0),(8,8),(0,8),(8,0),(0,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHishaN;
		banmen.0[(o.0).1][(o.0).0] = SGin;
		banmen.0[(o.1).1][(o.1).0] = SGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_nari_turn_move_and_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(8,0),(0,8),(8,8),(0,0),(8,0),(0,8),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 8] = [
		((0,1),(1,0)),((8,1),(7,0)),((0,1),(1,8)),((0,3),(3,0)),((5,8),(8,5)),((3,8),(0,5)),((5,0),(8,3)),((3,0),(0,3))
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,8),(0,8),(8,0),(0,0),(8,8),(0,8),(8,0),(0,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHishaN;
		banmen.0[8-(o.0).1][8-(o.0).0] = GGin;
		banmen.0[8-(o.1).1][8-(o.1).0] = GGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_nari_turn_move_and_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(8,0),(0,8),(8,8),(0,0),(8,0),(0,8),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 8] = [
		((0,1),(1,0)),((8,1),(7,0)),((0,1),(1,8)),((0,3),(3,0)),((5,8),(8,5)),((3,8),(0,5)),((5,0),(8,3)),((3,0),(0,3))
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,8),(0,8),(8,0),(0,0),(8,8),(0,8),(8,0),(0,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHishaN;
		banmen.0[(o.0).1][(o.0).0] = GGin;
		banmen.0[(o.1).1][(o.1).0] = GGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_none_moves_with_hisha_nari_turn_move_and_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 8] = [
		(0,0),(8,0),(0,8),(8,8),(0,0),(8,0),(0,8),(8,8)
	];

	const OCC_POSITIONS:[((usize,usize),(usize,usize)); 8] = [
		((0,1),(1,0)),((8,1),(7,0)),((0,1),(1,8)),((0,3),(3,0)),((5,8),(8,5)),((3,8),(0,5)),((5,0),(8,3)),((3,0),(0,3))
	];

	const OU_POSITIONS:[(usize,usize); 8] = [
		(8,8),(0,8),(8,0),(0,0),(8,8),(0,8),(8,0),(0,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHishaN;
		banmen.0[8-(o.0).1][8-(o.0).0] = SGin;
		banmen.0[8-(o.1).1][8-(o.1).0] = SGin;

		let answer:Vec<LegalMove> = vec![];

		assert_eq!(answer.clone(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer,
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_impl(
	occ_positions:Vec<Vec<(usize,usize)>>,occ_kind:KomaKind,positions:Vec<(usize,usize)>,kind:KomaKind
) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,o) in positions.iter().zip(&occ_positions) {
		let mut banmen = blank_banmen.clone();

		banmen.0[4][4] = GOu;

		banmen.0[p.1][p.0] = kind;

		let answer:Vec<LegalMove> = vec![];

		for op in o {
			banmen.0[op.1][op.0] = occ_kind;
		}

		assert_eq!(answer,
			Rule::oute_only_moves_with_point(Teban::Sente,&State::new(banmen.clone()),
				&MochigomaCollections::Empty, p.0 as u32,p.1 as u32).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_impl(
	occ_positions:Vec<Vec<(usize,usize)>>,occ_kind:KomaKind,positions:Vec<(usize,usize)>,kind:KomaKind
) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,o) in positions.iter().zip(&occ_positions) {
		let mut banmen = blank_banmen.clone();

		banmen.0[4][4] = GOu;

		banmen.0[p.1][p.0] = kind;

		let answer:Vec<LegalMove> = vec![];

		for op in o {
			banmen.0[op.1][op.0] = occ_kind;
		}

		assert_eq!(answer,
			Rule::oute_only_moves_with_point(Teban::Gote,&State::new(banmen.clone()),
				&MochigomaCollections::Empty,p.0 as u32,p.1 as u32).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_kin_impl(kind:KomaKind) {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_impl(
		vec![
			vec![(3,5),(4,5),(5,5)],
			vec![(3,5),(4,5),(5,5)],
			vec![(3,5),(4,5),(5,5)],
			vec![(3,5),(3,4),(3,3)],
			vec![(4,3)],
			vec![(5,5),(5,4),(5,3)]
		],
		SGin,
		vec![
			(3,6),(4,6),(5,6),(2,4),(4,2),(6,4),
		],
		kind
	)
}
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_kin_impl(kind:KomaKind) {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_impl(
		(vec![
			vec![(3,5),(4,5),(5,5)],
			vec![(3,5),(4,5),(5,5)],
			vec![(3,5),(4,5),(5,5)],
			vec![(3,5),(3,4),(3,3)],
			vec![(4,3)],
			vec![(5,5),(5,4),(5,3)]
		]).into_iter().map(|o| o.into_iter().map(|op| {
			(8 - op.0, 8- op.1)
		}).collect::<Vec<(usize,usize)>>()).collect::<Vec<Vec<(usize,usize)>>>(),
		GGin,
		(vec![
			(3,6),(4,6),(5,6),(2,4),(4,2),(6,4),
		]).into_iter().map(|p| (8 - p.0, 8 - p.1)).collect::<Vec<(usize,usize)>>(),
		kind
	)
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_gin() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_impl(
		vec![
			vec![(3,5),(4,5),(5,5)],
			vec![(3,5),(4,5),(5,5)],
			vec![(3,5),(4,5),(5,5)],
			vec![(3,3)],
			vec![(5,3)]
		],
		SKin,
		vec![
			(3,6),(4,6),(5,6),(2,2),(6,2),
		],
		SGin
	)
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_gin() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_impl(
		(vec![
			vec![(3,5),(4,5),(5,5)],
			vec![(3,5),(4,5),(5,5)],
			vec![(3,5),(4,5),(5,5)],
			vec![(3,3)],
			vec![(5,3)]
		]).into_iter().map(|o| o.into_iter().map(|op| {
			(8 - op.0, 8- op.1)
		}).collect::<Vec<(usize,usize)>>()).collect::<Vec<Vec<(usize,usize)>>>(),
		GKin,
		(vec![
			(3,6),(4,6),(5,6),(2,2),(6,2),
		]).into_iter().map(|p| (8 - p.0, 8 - p.1)).collect::<Vec<(usize,usize)>>(),
		GGin
	)
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_kei() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_impl(
		vec![
			vec![(3,6)],
			vec![(5,6)],
		],
		SKin,
		vec![
			(1,8),(7,8)
		],
		SKei
	)
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_kei() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_impl(
		(vec![
			vec![(3,6)],
			vec![(5,6)],
		]).into_iter().map(|o| o.into_iter().map(|op| {
			(8 - op.0, 8- op.1)
		}).collect::<Vec<(usize,usize)>>()).collect::<Vec<Vec<(usize,usize)>>>(),
		GKin,
		(vec![
			(1,8),(7,8)
		]).into_iter().map(|p| (8 - p.0, 8 - p.1)).collect::<Vec<(usize,usize)>>(),
		GKei
	)
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_kaku_nari() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_impl(
		vec![
			vec![(4,5),(3,5),(5,5)],
			vec![(3,4),(3,3),(3,5)],
			vec![(5,4),(5,3),(5,5)],
			vec![(4,3),(3,3),(5,3)]
		],
		SKin,
		vec![
			(4,6),(2,4),(6,4),(4,2)
		],
		SKakuN
	)
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_kaku_nari() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_impl(
		(vec![
			vec![(4,5),(3,5),(5,5)],
			vec![(3,4),(3,3),(3,5)],
			vec![(5,4),(5,3),(5,5)],
			vec![(4,3),(3,3),(5,3)]
		]).into_iter().map(|o| o.into_iter().map(|op| {
			(8 - op.0, 8- op.1)
		}).collect::<Vec<(usize,usize)>>()).collect::<Vec<Vec<(usize,usize)>>>(),
		GKin,
		(vec![
			(4,6),(2,4),(6,4),(4,2)
		]).into_iter().map(|p| (8 - p.0, 8 - p.1)).collect::<Vec<(usize,usize)>>(),
		GKakuN
	)
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_hisha_nari() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_impl(
		vec![
			vec![(3,5),(2,4),(4,6)],
			vec![(5,5),(6,6),(4,6)],
			vec![(3,3),(2,4),(4,2)],
			vec![(5,3),(6,4),(4,2)]
		],
		SKin,
		vec![
			(2,6),(6,6),(2,2),(6,2)
		],
		SHishaN
	)
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_hisha_nari() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_impl(
		(vec![
			vec![(3,5),(2,4),(4,6)],
			vec![(5,5),(6,6),(4,6)],
			vec![(3,3),(2,4),(4,2)],
			vec![(5,3),(6,4),(6,4)]
		]).into_iter().map(|o| o.into_iter().map(|op| {
			(8 - op.0, 8- op.1)
		}).collect::<Vec<(usize,usize)>>()).collect::<Vec<Vec<(usize,usize)>>>(),
		GKin,
		(vec![
			(2,6),(6,6),(2,2),(6,2)
		]).into_iter().map(|p| (8 - p.0, 8 - p.1)).collect::<Vec<(usize,usize)>>(),
		GHishaN
	)
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_fu_nari() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_kin_impl(SFuN);
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_fu_nari() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_kin_impl(GFuN);
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_kyou_nari() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_kin_impl(SKyouN);
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_kyou_nari() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_kin_impl(GKyouN);
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_kei_nari() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_kin_impl(SKeiN);
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_kei_nari() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_kin_impl(GKeiN);
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_gin_nari() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_sente_kin_impl(SGinN);
}
#[test]
fn test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_gin_nari() {
	test_oute_only_moves_with_point_none_moves_with_occupied_self_gote_kin_impl(GGinN);
}
#[test]
fn test_oute_only_moves_with_kyou_occupied_opponent_sente() {
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

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = if p.1 <= 2 || o.1 <= 2 {
			vec![
				((p.0 as u32, p.1 as u32),(o.0 as u32, o.1 as u32, true),Some(ObtainKind::Fu)),
				((p.0 as u32, p.1 as u32),(o.0 as u32, o.1 as u32, false),Some(ObtainKind::Fu)),
			]
		} else {
			vec![
				((p.0 as u32, p.1 as u32),(o.0 as u32, o.1 as u32, false),Some(ObtainKind::Fu))
			]
		};

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kyou_occupied_opponent_gote() {
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

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = if (8 - p.1) >= 6 || (8 - o.1) >= 6 {
			vec![
				((8-p.0 as u32, 8-p.1 as u32),(8-o.0 as u32, 8-o.1 as u32,true),Some(ObtainKind::Fu)),
				((8-p.0 as u32, 8-p.1 as u32),(8-o.0 as u32, 8-o.1 as u32,false),Some(ObtainKind::Fu)),
			]
		} else {
			vec![
				((8-p.0 as u32, 8-p.1 as u32),(8-o.0 as u32, 8-o.1 as u32,false),Some(ObtainKind::Fu))
			]
		};

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kyou_open_path_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 2] = [
		(1,7),(7,7)
	];

	const OU_POSITIONS:[(usize,usize); 2] = [
		(1,0),(7,0)
	];

	const OCC_POSITIONS:[(usize,usize); 2] = [
		(1,4),(7,4)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((7,4),(6,3,false),None),((7,4),(6,4,false),None),((7,4),(8,3,false),None),((7,4),(8,4,false),None)],
	];

	for (((p,t),o),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKyou;
		banmen.0[o.1][o.0] = SKin;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kyou_open_path_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 2] = [
		(1,7),(7,7)
	];

	const OU_POSITIONS:[(usize,usize); 2] = [
		(1,0),(7,0)
	];

	const OCC_POSITIONS:[(usize,usize); 2] = [
		(1,4),(7,4)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((7,4),(6,3,false),None),((7,4),(6,4,false),None),((7,4),(8,3,false),None),((7,4),(8,4,false),None)],
	];

	for (((p,t),o),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKyou;
		banmen.0[8-o.1][8-o.0] = GKin;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kyou_open_path_put_and_to_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OU_POSITIONS:[(usize,usize); 2] = [
		(1,0),(7,0)
	];

	const OCC_POSITIONS:[(usize,usize); 2] = [
		(1,4),(7,4)
	];

	const MOVES:[Move; 2] = [
		Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-1,7+1)),
		Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-7,7+1)),
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((7,4),(6,3,false),None),((7,4),(6,4,false),None),((7,4),(8,3,false),None),((7,4),(8,4,false),None)],
	];

	for (((m,t),o),answer) in MOVES.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[o.1][o.0] = SKin;

		let mut state = State::new(banmen.clone());
		let mut ms = HashMap::new();

		ms.insert(MochigomaKind::Kyou, 1);

		let mut mc = MochigomaCollections::Pair(ms,HashMap::new());

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut ms = HashMap::new();

		ms.insert(MochigomaKind::Kyou, 1);

		let mut mc = MochigomaCollections::Pair(ms,HashMap::new());


		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kyou_open_path_put_and_to_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OU_POSITIONS:[(usize,usize); 2] = [
		(1,0),(7,0)
	];

	const OCC_POSITIONS:[(usize,usize); 2] = [
		(1,4),(7,4)
	];

	const MOVES:[Move; 2] = [
		Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-(8-1),(8-7)+1)),
		Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-(8-7),(8-7)+1)),
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((7,4),(6,3,false),None),((7,4),(6,4,false),None),((7,4),(8,3,false),None),((7,4),(8,4,false),None)],
	];

	for (((m,t),o),answer) in MOVES.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-o.1][8-o.0] = GKin;

		let mut state = State::new(banmen.clone());
		let mut mg = HashMap::new();

		mg.insert(MochigomaKind::Kyou, 1);

		let mut mc = MochigomaCollections::Pair(HashMap::new(),mg);

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut mg = HashMap::new();

		mg.insert(MochigomaKind::Kyou, 1);

		let mut mc = MochigomaCollections::Pair(HashMap::new(),mg);


		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(7,2),(1,6),(7,6),(2,1)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(8,1),(0,7),(8,7),(1,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKaku;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = if p.1 <= 2 || o.1 <= 2 {
			vec![
				((p.0 as u32, p.1 as u32),(o.0 as u32, o.1 as u32, true),Some(ObtainKind::Fu)),
				((p.0 as u32, p.1 as u32),(o.0 as u32, o.1 as u32, false),Some(ObtainKind::Fu)),
			]
		} else {
			vec![
				((p.0 as u32, p.1 as u32),(o.0 as u32, o.1 as u32, false),Some(ObtainKind::Fu))
			]
		};

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,0),(1,0),(8,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(7,2),(1,6),(7,6),(2,1)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(8,1),(0,7),(8,7),(1,0)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKaku;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = if (8 - p.1) >= 6 || (8 - o.1) >= 6 {
			vec![
				((8-p.0 as u32, 8-p.1 as u32),(8-o.0 as u32, 8-o.1 as u32,true),Some(ObtainKind::Fu)),
				((8-p.0 as u32, 8-p.1 as u32),(8-o.0 as u32, 8-o.1 as u32,false),Some(ObtainKind::Fu)),
			]
		} else {
			vec![
				((8-p.0 as u32, 8-p.1 as u32),(8-o.0 as u32, 8-o.1 as u32,false),Some(ObtainKind::Fu))
			]
		};

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_turn_move_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,8),(0,1),(8,1)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,0),(5,4,true),None),((1,0),(5,4,false),None)],
		vec![((7,0),(3,4,true),None),((7,0),(3,4,false),None)],
		vec![((0,7),(3,4,false),None)],
		vec![((8,7),(5,4,false),None)],
	];

	for ((p,t),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKaku;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_turn_move_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,8),(0,1),(8,1)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,0),(5,4,true),None),((1,0),(5,4,false),None)],
		vec![((7,0),(3,4,true),None),((7,0),(3,4,false),None)],
		vec![((0,7),(3,4,false),None)],
		vec![((8,7),(5,4,false),None)],
	];

	for ((p,t),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKaku;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_nari_turn_move_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,8),(0,1),(8,1)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,0),(5,4,false),None)],
		vec![((7,0),(3,4,false),None)],
		vec![((0,7),(3,4,false),None)],
		vec![((8,7),(5,4,false),None)],
	];

	for ((p,t),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKakuN;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_nari_turn_move_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,8),(7,8),(0,1),(8,1)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,0),(5,4,false),None)],
		vec![((7,0),(3,4,false),None)],
		vec![((0,7),(3,4,false),None)],
		vec![((8,7),(5,4,false),None)],
	];

	for ((p,t),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKakuN;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_nari_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(4,2),(2,4),(6,4),(4,6)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(4,3),(3,4),(5,4),(4,5)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![
			((4,2),(3,3,false),None),
			((4,2),(5,3,false),None),
			((4,2),(4,3,false),Some(ObtainKind::Fu)),
		],
		vec![
			((2,4),(3,3,false),None),
			((2,4),(3,5,false),None),
			((2,4),(3,4,false),Some(ObtainKind::Fu)),
		],
		vec![
			((6,4),(5,3,false),None),
			((6,4),(5,5,false),None),
			((6,4),(5,4,false),Some(ObtainKind::Fu)),
		],
		vec![
			((4,6),(3,5,false),None),
			((4,6),(5,5,false),None),
			((4,6),(4,5,false),Some(ObtainKind::Fu)),
		]
	];

	for ((p,o),answer) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[4][4] = GOu;
		banmen.0[p.1][p.0] = SKakuN;
		banmen.0[o.1][o.0] = GFu;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_nari_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(4,2),(2,4),(6,4),(4,6)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(4,3),(3,4),(5,4),(4,5)
	];


	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![
			((4,2),(3,3,false),None),
			((4,2),(5,3,false),None),
			((4,2),(4,3,false),Some(ObtainKind::Fu)),
		],
		vec![
			((2,4),(3,3,false),None),
			((2,4),(3,5,false),None),
			((2,4),(3,4,false),Some(ObtainKind::Fu)),
		],
		vec![
			((6,4),(5,3,false),None),
			((6,4),(5,5,false),None),
			((6,4),(5,4,false),Some(ObtainKind::Fu)),
		],
		vec![
			((4,6),(3,5,false),None),
			((4,6),(5,5,false),None),
			((4,6),(4,5,false),Some(ObtainKind::Fu)),
		]
	];

	for ((p,o),answer) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[4][4] = SOu;
		banmen.0[8-p.1][8-p.0] = GKakuN;
		banmen.0[8-o.1][8-o.0] = SFu;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_open_path_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(8,7),(0,7),(7,0),(1,0)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,2),(5,2),(2,5),(6,5)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((3,2),(3,1,true),None),((3,2),(3,1,false),None)],
		vec![((5,2),(5,1,true),None),((5,2),(5,1,false),None)],
		vec![((2,5),(2,4,false),None)],
		vec![((6,5),(6,4,false),None)],
	];

	for (((p,t),o),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKaku;
		banmen.0[o.1][o.0] = SFu;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_open_path_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(8,7),(0,7),(7,0),(1,0)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,2),(5,2),(2,5),(6,5)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((3,2),(3,1,true),None),((3,2),(3,1,false),None)],
		vec![((5,2),(5,1,true),None),((5,2),(5,1,false),None)],
		vec![((2,5),(2,4,false),None)],
		vec![((6,5),(6,4,false),None)],
	];

	for (((p,t),o),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKaku;
		banmen.0[8-o.1][8-o.0] = GFu;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_open_path_put_and_to_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OU_POSITIONS:[(usize,usize); 4] = [
		(8,7),(0,7),(7,0),(1,0)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,2),(5,2),(2,5),(6,5)
	];

	const MOVES:[Move; 4] = [
		Move::Put(MochigomaKind::Kaku,KomaDstPutPosition(9-1,0+1)),
		Move::Put(MochigomaKind::Kaku,KomaDstPutPosition(9-7,0+1)),
		Move::Put(MochigomaKind::Kaku,KomaDstPutPosition(9-0,7+1)),
		Move::Put(MochigomaKind::Kaku,KomaDstPutPosition(9-8,7+1)),
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((3,2),(3,1,true),None),((3,2),(3,1,false),None)],
		vec![((5,2),(5,1,true),None),((5,2),(5,1,false),None)],
		vec![((2,5),(2,4,false),None)],
		vec![((6,5),(6,4,false),None)],
	];

	for (((m,t),o),answer) in MOVES.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[o.1][o.0] = SFu;

		let mut state = State::new(banmen.clone());
		let mut ms = HashMap::new();

		ms.insert(MochigomaKind::Kaku, 1);

		let mut mc = MochigomaCollections::Pair(ms,HashMap::new());

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut ms = HashMap::new();

		ms.insert(MochigomaKind::Kaku, 1);

		let mut mc = MochigomaCollections::Pair(ms,HashMap::new());


		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_open_path_put_and_to_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OU_POSITIONS:[(usize,usize); 4] = [
		(8,7),(0,7),(7,0),(1,0)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,2),(5,2),(2,5),(6,5)
	];

	const MOVES:[Move; 4] = [
		Move::Put(MochigomaKind::Kaku,KomaDstPutPosition(9-(8-1),(8-0)+1)),
		Move::Put(MochigomaKind::Kaku,KomaDstPutPosition(9-(8-7),(8-0)+1)),
		Move::Put(MochigomaKind::Kaku,KomaDstPutPosition(9-(8-0),(8-7)+1)),
		Move::Put(MochigomaKind::Kaku,KomaDstPutPosition(9-(8-8),(8-7)+1)),
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((3,2),(3,1,true),None),((3,2),(3,1,false),None)],
		vec![((5,2),(5,1,true),None),((5,2),(5,1,false),None)],
		vec![((2,5),(2,4,false),None)],
		vec![((6,5),(6,4,false),None)],
	];

	for (((m,t),o),answer) in MOVES.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-o.1][8-o.0] = GFu;

		let mut state = State::new(banmen.clone());
		let mut mg = HashMap::new();

		mg.insert(MochigomaKind::Kaku, 1);

		let mut mc = MochigomaCollections::Pair(HashMap::new(),mg);

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut mg = HashMap::new();

		mg.insert(MochigomaKind::Kaku, 1);

		let mut mc = MochigomaCollections::Pair(HashMap::new(),mg);


		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_open_path_to_and_to_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(7,0),(0,1),(6,7),(8,1)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(7,6),(0,7),(7,0),(1,0)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(6,5),(2,5),(5,2),(3,2)
	];

	const MOVES:[Move; 4] = [
		Move::To(KomaSrcPosition(9-7,0+1),KomaDstToPosition(9-4,3+1,false)),
		Move::To(KomaSrcPosition(9-0,1+1),KomaDstToPosition(9-3,4+1,false)),
		Move::To(KomaSrcPosition(9-6,7+1),KomaDstToPosition(9-3,4+1,false)),
		Move::To(KomaSrcPosition(9-8,1+1),KomaDstToPosition(9-5,4+1,false)),
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((6,5),(6,4,false),None)],
		vec![((2,5),(2,4,false),None)],
		vec![((5,2),(5,1,true),None),((5,2),(5,1,false),None)],
		vec![((3,2),(3,1,true),None),((3,2),(3,1,false),None)],
	];

	for ((((p,m),t),o),answer) in POSITIONS.iter().zip(&MOVES).zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[o.1][o.0] = SFu;
		banmen.0[p.1][p.0] = SKaku;

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_open_path_to_and_to_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(7,0),(0,1),(6,7),(8,1)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(7,6),(0,7),(7,0),(1,0)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(6,5),(2,5),(5,2),(3,2)
	];

	const MOVES:[Move; 4] = [
		Move::To(KomaSrcPosition(9-(8-7),(8-0)+1),KomaDstToPosition(9-(8-4),(8-3)+1,false)),
		Move::To(KomaSrcPosition(9-(8-0),(8-1)+1),KomaDstToPosition(9-(8-3),(8-4)+1,false)),
		Move::To(KomaSrcPosition(9-(8-6),(8-7)+1),KomaDstToPosition(9-(8-3),(8-4)+1,false)),
		Move::To(KomaSrcPosition(9-(8-8),(8-1)+1),KomaDstToPosition(9-(8-5),(8-4)+1,false)),
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((6,5),(6,4,false),None)],
		vec![((2,5),(2,4,false),None)],
		vec![((5,2),(5,1,true),None),((5,2),(5,1,false),None)],
		vec![((3,2),(3,1,true),None),((3,2),(3,1,false),None)],
	];

	for ((((p,m),t),o),answer) in POSITIONS.iter().zip(&MOVES).zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-o.1][8-o.0] = GFu;
		banmen.0[8-p.1][8-p.0] = GKaku;

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_open_path_to_nari_and_to_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(7,0),(0,1),(6,7),(8,1)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(7,6),(0,7),(7,0),(1,0)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(6,5),(2,5),(5,2),(3,2)
	];

	const MOVES:[Move; 4] = [
		Move::To(KomaSrcPosition(9-7,0+1),KomaDstToPosition(9-4,3+1,true)),
		Move::To(KomaSrcPosition(9-0,1+1),KomaDstToPosition(9-3,4+1,true)),
		Move::To(KomaSrcPosition(9-6,7+1),KomaDstToPosition(9-3,4+1,true)),
		Move::To(KomaSrcPosition(9-8,1+1),KomaDstToPosition(9-5,4+1,true)),
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((6,5),(6,4,false),None)],
		vec![((2,5),(2,4,false),None)],
		vec![((5,2),(5,1,true),None),((5,2),(5,1,false),None)],
		vec![((3,2),(3,1,true),None),((3,2),(3,1,false),None)],
	];

	for ((((p,m),t),o),answer) in POSITIONS.iter().zip(&MOVES).zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[o.1][o.0] = SFu;
		banmen.0[p.1][p.0] = SKaku;

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_open_path_to_nari_and_to_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(7,0),(0,1),(6,7),(8,1)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(7,6),(0,7),(7,0),(1,0)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(6,5),(2,5),(5,2),(3,2)
	];

	const MOVES:[Move; 4] = [
		Move::To(KomaSrcPosition(9-(8-7),(8-0)+1),KomaDstToPosition(9-(8-4),(8-3)+1,true)),
		Move::To(KomaSrcPosition(9-(8-0),(8-1)+1),KomaDstToPosition(9-(8-3),(8-4)+1,true)),
		Move::To(KomaSrcPosition(9-(8-6),(8-7)+1),KomaDstToPosition(9-(8-3),(8-4)+1,true)),
		Move::To(KomaSrcPosition(9-(8-8),(8-1)+1),KomaDstToPosition(9-(8-5),(8-4)+1,true)),
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((6,5),(6,4,false),None)],
		vec![((2,5),(2,4,false),None)],
		vec![((5,2),(5,1,true),None),((5,2),(5,1,false),None)],
		vec![((3,2),(3,1,true),None),((3,2),(3,1,false),None)],
	];

	for ((((p,m),t),o),answer) in POSITIONS.iter().zip(&MOVES).zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-o.1][8-o.0] = GFu;
		banmen.0[8-p.1][8-p.0] = GKaku;

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_nari_open_path_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(8,7),(0,7),(7,0),(1,0)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,2),(5,2),(2,5),(6,5)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((3,2),(3,1,true),None),((3,2),(3,1,false),None)],
		vec![((5,2),(5,1,true),None),((5,2),(5,1,false),None)],
		vec![((2,5),(2,4,false),None)],
		vec![((6,5),(6,4,false),None)],
	];

	for (((p,t),o),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SKakuN;
		banmen.0[o.1][o.0] = SFu;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_nari_open_path_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(8,7),(0,7),(7,0),(1,0)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(3,2),(5,2),(2,5),(6,5)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((3,2),(3,1,true),None),((3,2),(3,1,false),None)],
		vec![((5,2),(5,1,true),None),((5,2),(5,1,false),None)],
		vec![((2,5),(2,4,false),None)],
		vec![((6,5),(6,4,false),None)],
	];

	for (((p,t),o),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GKakuN;
		banmen.0[8-o.1][8-o.0] = GFu;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_nari_open_path_to_and_to_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(7,0),(0,1),(6,7),(8,1)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(7,6),(0,7),(7,0),(1,0)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(6,5),(2,5),(5,2),(3,2)
	];

	const MOVES:[Move; 4] = [
		Move::To(KomaSrcPosition(9-7,0+1),KomaDstToPosition(9-4,3+1,false)),
		Move::To(KomaSrcPosition(9-0,1+1),KomaDstToPosition(9-3,4+1,false)),
		Move::To(KomaSrcPosition(9-6,7+1),KomaDstToPosition(9-3,4+1,false)),
		Move::To(KomaSrcPosition(9-8,1+1),KomaDstToPosition(9-5,4+1,false)),
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((6,5),(6,4,false),None)],
		vec![((2,5),(2,4,false),None)],
		vec![((5,2),(5,1,true),None),((5,2),(5,1,false),None)],
		vec![((3,2),(3,1,true),None),((3,2),(3,1,false),None)],
	];

	for ((((p,m),t),o),answer) in POSITIONS.iter().zip(&MOVES).zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[o.1][o.0] = SFu;
		banmen.0[p.1][p.0] = SKakuN;

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kaku_nari_open_path_to_and_to_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(7,0),(0,1),(6,7),(8,1)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(7,6),(0,7),(7,0),(1,0)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(6,5),(2,5),(5,2),(3,2)
	];

	const MOVES:[Move; 4] = [
		Move::To(KomaSrcPosition(9-(8-7),(8-0)+1),KomaDstToPosition(9-(8-4),(8-3)+1,false)),
		Move::To(KomaSrcPosition(9-(8-0),(8-1)+1),KomaDstToPosition(9-(8-3),(8-4)+1,false)),
		Move::To(KomaSrcPosition(9-(8-6),(8-7)+1),KomaDstToPosition(9-(8-3),(8-4)+1,false)),
		Move::To(KomaSrcPosition(9-(8-8),(8-1)+1),KomaDstToPosition(9-(8-5),(8-4)+1,false)),
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((6,5),(6,4,false),None)],
		vec![((2,5),(2,4,false),None)],
		vec![((5,2),(5,1,true),None),((5,2),(5,1,false),None)],
		vec![((3,2),(3,1,true),None),((3,2),(3,1,false),None)],
	];

	for ((((p,m),t),o),answer) in POSITIONS.iter().zip(&MOVES).zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-o.1][8-o.0] = GFu;
		banmen.0[8-p.1][8-p.0] = GKakuN;

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_turn_move_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(7,8),(0,8),(7,1),(1,1)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,0),(1,8,true),None),((1,0),(1,8,false),None),((1,0),(7,0,true),None),((1,0),(7,0,false),None)],
		vec![((7,0),(7,8,true),None),((7,0),(7,8,false),None),((7,0),(0,0,true),None),((7,0),(0,0,false),None)],
		vec![((0,7),(0,1,true),None),((0,7),(0,1,false),None),((0,7),(7,7,false),None)],
		vec![((8,7),(8,1,true),None),((8,7),(8,1,false),None),((8,7),(1,7,false),None)],
	];

	for ((p,t),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHisha;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_turn_move_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(7,8),(0,8),(7,1),(1,1)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,0),(1,8,true),None),((1,0),(1,8,false),None),((1,0),(7,0,true),None),((1,0),(7,0,false),None)],
		vec![((7,0),(7,8,true),None),((7,0),(7,8,false),None),((7,0),(0,0,true),None),((7,0),(0,0,false),None)],
		vec![((0,7),(0,1,true),None),((0,7),(0,1,false),None),((0,7),(7,7,false),None)],
		vec![((8,7),(8,1,true),None),((8,7),(8,1,false),None),((8,7),(1,7,false),None)],
	];

	for ((p,t),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHisha;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_nari_turn_move_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(7,8),(0,8),(7,1),(1,1)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,0),(1,8,false),None),((1,0),(7,0,false),None)],
		vec![((7,0),(7,8,false),None),((7,0),(0,0,false),None)],
		vec![((0,7),(0,1,false),None),((0,7),(7,7,false),None)],
		vec![((8,7),(8,1,false),None),((8,7),(1,7,false),None)]
	];

	for ((p,t),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHishaN;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_nari_turn_move_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(7,8),(0,8),(7,1),(1,1)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,0),(1,8,false),None),((1,0),(7,0,false),None)],
		vec![((7,0),(7,8,false),None),((7,0),(0,0,false),None)],
		vec![((0,7),(0,1,false),None),((0,7),(7,7,false),None)],
		vec![((8,7),(8,1,false),None),((8,7),(1,7,false),None)]
	];

	for ((p,t),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHishaN;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_occupied_opponent_sente() {
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

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = if p.1 <= 2 || o.1 <= 2 {
			vec![
				((p.0 as u32, p.1 as u32),(o.0 as u32, o.1 as u32, true),Some(ObtainKind::Fu)),
				((p.0 as u32, p.1 as u32),(o.0 as u32, o.1 as u32, false),Some(ObtainKind::Fu)),
			]
		} else {
			vec![
				((p.0 as u32, p.1 as u32),(o.0 as u32, o.1 as u32, false),Some(ObtainKind::Fu))
			]
		};

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 5] = [
		(1,8),(7,0),(1,0),(8,7),(0,7)
	];

	const OCC_POSITIONS:[(usize,usize); 5] = [
		(1,1),(7,7),(1,7),(7,7),(1,7)
	];

	const OU_POSITIONS:[(usize,usize); 5] = [
		(1,0),(7,8),(1,8),(0,7),(8,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHisha;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = if (8 - p.1) >= 6 || (8 - o.1) >= 6 {
			vec![
				((8-p.0 as u32, 8-p.1 as u32),(8-o.0 as u32, 8-o.1 as u32,true),Some(ObtainKind::Fu)),
				((8-p.0 as u32, 8-p.1 as u32),(8-o.0 as u32, 8-o.1 as u32,false),Some(ObtainKind::Fu)),
			]
		} else {
			vec![
				((8-p.0 as u32, 8-p.1 as u32),(8-o.0 as u32, 8-o.1 as u32,false),Some(ObtainKind::Fu))
			]
		};

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_nari_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 5] = [
		(1,8),(7,0),(1,0),(8,7),(0,7)
	];

	const OCC_POSITIONS:[(usize,usize); 5] = [
		(1,1),(7,7),(1,7),(7,7),(1,7)
	];

	const OU_POSITIONS:[(usize,usize); 5] = [
		(1,0),(7,8),(1,8),(0,7),(8,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHishaN;
		banmen.0[o.1][o.0] = GFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
			((p.0 as u32,p.1 as u32),(o.0 as u32,o.1 as u32,false),Some(ObtainKind::Fu))
		];

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_nari_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 5] = [
		(1,8),(7,0),(1,0),(8,7),(0,7)
	];

	const OCC_POSITIONS:[(usize,usize); 5] = [
		(1,1),(7,7),(1,7),(7,7),(1,7)
	];

	const OU_POSITIONS:[(usize,usize); 5] = [
		(1,0),(7,8),(1,8),(0,7),(8,7)
	];

	for ((p,o),t) in POSITIONS.iter().zip(&OCC_POSITIONS).zip(&OU_POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHishaN;
		banmen.0[8-o.1][8-o.0] = SFu;

		let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
			((8-p.0 as u32,8-p.1 as u32),(8-o.0 as u32,8-o.1 as u32,false),Some(ObtainKind::Fu)),
		];

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_open_path_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,7),(0,0),(0,0),(0,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,4),(4,0),(0,4),(5,7)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((4,0),(4,1,false),None)],
		vec![((0,4),(1,3,false),None),((0,4),(1,4,false),None)],
		vec![((5,7),(4,6,false),None),((5,7),(5,6,false),None),((5,7),(5,8,false),None),((5,7),(6,6,false),None)],
	];

	for (((p,t),o),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHisha;
		banmen.0[o.1][o.0] = SKin;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_open_path_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,7),(0,0),(0,0),(0,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,4),(4,0),(0,4),(5,7)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((4,0),(4,1,false),None)],
		vec![((0,4),(1,3,false),None),((0,4),(1,4,false),None)],
		vec![((5,7),(4,6,false),None),((5,7),(5,6,false),None),((5,7),(5,8,false),None),((5,7),(6,6,false),None)],
	];

	for (((p,t),o),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHisha;
		banmen.0[8-o.1][8-o.0] = GKin;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_open_path_put_and_to_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,7),(0,0),(0,0),(0,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,4),(4,0),(0,4),(5,7)
	];

	const MOVES:[Move; 4] = [
		Move::Put(MochigomaKind::Hisha,KomaDstPutPosition(9-1,0+1)),
		Move::Put(MochigomaKind::Hisha,KomaDstPutPosition(9-7,0+1)),
		Move::Put(MochigomaKind::Hisha,KomaDstPutPosition(9-0,7+1)),
		Move::Put(MochigomaKind::Hisha,KomaDstPutPosition(9-8,7+1)),
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((4,0),(4,1,false),None)],
		vec![((0,4),(1,3,false),None),((0,4),(1,4,false),None)],
		vec![((5,7),(4,6,false),None),((5,7),(5,6,false),None),((5,7),(5,8,false),None),((5,7),(6,6,false),None)],
	];

	for (((m,t),o),answer) in MOVES.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[o.1][o.0] = SKin;

		let mut state = State::new(banmen.clone());
		let mut ms = HashMap::new();

		ms.insert(MochigomaKind::Hisha, 1);

		let mut mc = MochigomaCollections::Pair(ms,HashMap::new());

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut ms = HashMap::new();

		ms.insert(MochigomaKind::Hisha, 1);

		let mut mc = MochigomaCollections::Pair(ms,HashMap::new());


		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_open_path_put_and_to_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,7),(0,0),(0,0),(0,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,4),(4,0),(0,4),(5,7)
	];

	const MOVES:[Move; 4] = [
		Move::Put(MochigomaKind::Hisha,KomaDstPutPosition(9-(8-1),(8-0)+1)),
		Move::Put(MochigomaKind::Hisha,KomaDstPutPosition(9-(8-7),(8-0)+1)),
		Move::Put(MochigomaKind::Hisha,KomaDstPutPosition(9-(8-0),(8-7)+1)),
		Move::Put(MochigomaKind::Hisha,KomaDstPutPosition(9-(8-8),(8-7)+1)),
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((4,0),(4,1,false),None)],
		vec![((0,4),(1,3,false),None),((0,4),(1,4,false),None)],
		vec![((5,7),(4,6,false),None),((5,7),(5,6,false),None),((5,7),(5,8,false),None),((5,7),(6,6,false),None)],
	];

	for (((m,t),o),answer) in MOVES.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-o.1][8-o.0] = GKin;

		let mut state = State::new(banmen.clone());
		let mut mg = HashMap::new();

		mg.insert(MochigomaKind::Hisha, 1);

		let mut mc = MochigomaCollections::Pair(HashMap::new(),mg);

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut mg = HashMap::new();

		mg.insert(MochigomaKind::Hisha, 1);

		let mut mc = MochigomaCollections::Pair(HashMap::new(),mg);


		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_open_path_to_and_to_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(8,0),(7,7),(7,7),(8,0)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,7),(0,0),(0,0),(0,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,4),(4,0),(0,4),(5,7)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((4,0),(4,1,false),None)],
		vec![((0,4),(1,3,false),None),((0,4),(1,4,false),None)],
		vec![((5,7),(4,6,false),None),((5,7),(5,6,false),None),((5,7),(5,8,false),None),((5,7),(6,6,false),None)],
	];

	const MOVES:[Move; 4] = [
		Move::To(KomaSrcPosition(9-8,0+1),KomaDstToPosition(9-1,0+1,false)),
		Move::To(KomaSrcPosition(9-7,7+1),KomaDstToPosition(9-7,0+1,false)),
		Move::To(KomaSrcPosition(9-7,7+1),KomaDstToPosition(9-0,7+1,false)),
		Move::To(KomaSrcPosition(9-8,0+1),KomaDstToPosition(9-8,7+1,false)),
	];

	for ((((p,m),t),o),answer) in POSITIONS.iter().zip(&MOVES).zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[o.1][o.0] = SKin;
		banmen.0[p.1][p.0] = SHisha;

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_open_path_to_and_to_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(8,0),(7,7),(7,7),(8,0)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,7),(0,0),(0,0),(0,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,4),(4,0),(0,4),(5,7)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((4,0),(4,1,false),None)],
		vec![((0,4),(1,3,false),None),((0,4),(1,4,false),None)],
		vec![((5,7),(4,6,false),None),((5,7),(5,6,false),None),((5,7),(5,8,false),None),((5,7),(6,6,false),None)],
	];

	const MOVES:[Move; 4] = [
		Move::To(KomaSrcPosition(9-(8-8),(8-0)+1),KomaDstToPosition(9-(8-1),(8-0)+1,false)),
		Move::To(KomaSrcPosition(9-(8-7),(8-7)+1),KomaDstToPosition(9-(8-7),(8-0)+1,false)),
		Move::To(KomaSrcPosition(9-(8-7),(8-7)+1),KomaDstToPosition(9-(8-0),(8-7)+1,false)),
		Move::To(KomaSrcPosition(9-(8-8),(8-0)+1),KomaDstToPosition(9-(8-8),(8-7)+1,false)),
	];

	for ((((p,m),t),o),answer) in POSITIONS.iter().zip(&MOVES).zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-o.1][8-o.0] = GKin;
		banmen.0[8-p.1][8-p.0] = GHisha;

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_open_path_to_nari_and_to_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(8,0),(7,7),(7,7),(8,0)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,7),(0,0),(0,0),(0,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,4),(4,0),(0,4),(5,7)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((4,0),(4,1,false),None)],
		vec![((0,4),(1,3,false),None),((0,4),(1,4,false),None)],
		vec![((5,7),(4,6,false),None),((5,7),(5,6,false),None),((5,7),(5,8,false),None),((5,7),(6,6,false),None)],
	];

	const MOVES:[Move; 4] = [
		Move::To(KomaSrcPosition(9-8,0+1),KomaDstToPosition(9-1,0+1,true)),
		Move::To(KomaSrcPosition(9-7,7+1),KomaDstToPosition(9-7,0+1,true)),
		Move::To(KomaSrcPosition(9-7,7+1),KomaDstToPosition(9-0,7+1,true)),
		Move::To(KomaSrcPosition(9-8,0+1),KomaDstToPosition(9-8,7+1,true)),
	];

	for ((((p,m),t),o),answer) in POSITIONS.iter().zip(&MOVES).zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHishaN;
		banmen.0[o.1][o.0] = SKin;

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_open_path_to_nari_and_to_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(8,0),(7,7),(7,7),(8,0)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,7),(0,0),(0,0),(0,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,4),(4,0),(0,4),(5,7)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((4,0),(4,1,false),None)],
		vec![((0,4),(1,3,false),None),((0,4),(1,4,false),None)],
		vec![((5,7),(4,6,false),None),((5,7),(5,6,false),None),((5,7),(5,8,false),None),((5,7),(6,6,false),None)],
	];

	const MOVES:[Move; 4] = [
		Move::To(KomaSrcPosition(9-(8-8),(8-0)+1),KomaDstToPosition(9-(8-1),(8-0)+1,true)),
		Move::To(KomaSrcPosition(9-(8-7),(8-7)+1),KomaDstToPosition(9-(8-7),(8-0)+1,true)),
		Move::To(KomaSrcPosition(9-(8-7),(8-7)+1),KomaDstToPosition(9-(8-0),(8-7)+1,true)),
		Move::To(KomaSrcPosition(9-(8-8),(8-0)+1),KomaDstToPosition(9-(8-8),(8-7)+1,true)),
	];

	for ((((p,m),t),o),answer) in POSITIONS.iter().zip(&MOVES).zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-o.1][8-o.0] = GKin;
		banmen.0[8-p.1][8-p.0] = GHisha;

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		let mut state = State::new(banmen.clone());
		let mut mc = MochigomaCollections::Empty;

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_nari_open_path_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,7),(0,0),(0,0),(0,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,4),(4,0),(0,4),(5,7)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((4,0),(4,1,false),None)],
		vec![((0,4),(1,3,false),None),((0,4),(1,4,false),None)],
		vec![((5,7),(4,6,false),None),((5,7),(5,6,false),None),((5,7),(5,8,false),None),((5,7),(6,6,false),None)],
	];

	for (((p,t),o),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[t.1][t.0] = GOu;
		banmen.0[p.1][p.0] = SHishaN;
		banmen.0[o.1][o.0] = SKin;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_hisha_nari_open_path_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(usize,usize); 4] = [
		(1,0),(7,0),(0,7),(8,7)
	];

	const OU_POSITIONS:[(usize,usize); 4] = [
		(1,7),(0,0),(0,0),(0,7)
	];

	const OCC_POSITIONS:[(usize,usize); 4] = [
		(1,4),(4,0),(0,4),(5,7)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((1,4),(0,3,false),None),((1,4),(0,4,false),None),((1,4),(2,3,false),None),((1,4),(2,4,false),None)],
		vec![((4,0),(4,1,false),None)],
		vec![((0,4),(1,3,false),None),((0,4),(1,4,false),None)],
		vec![((5,7),(4,6,false),None),((5,7),(5,6,false),None),((5,7),(5,8,false),None),((5,7),(6,6,false),None)],
	];

	for (((p,t),o),answer) in POSITIONS.iter().zip(&OU_POSITIONS).zip(&OCC_POSITIONS).zip(answer.into_iter()) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-t.1][8-t.0] = SOu;
		banmen.0[8-p.1][8-p.0] = GHishaN;
		banmen.0[8-o.1][8-o.0] = GKin;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_fu_sente() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = GOu;

	banmen.0[6][4] = SFu;


	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((4,6),(4,5,false),None),
	];

	assert_eq!(answer.clone().into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>(),
		Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);

	assert_eq!(answer.into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>(),
		Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_with_fu_gote() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = SOu;

	banmen.0[8-6][8-4] = GFu;


	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((8-4,8-6),(8-4,8-5,false),None),
	];

	assert_eq!(answer.clone().into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>(),
		Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);

	assert_eq!(answer.into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>(),
		Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_with_fu_nari_move_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let positions:Vec<(u32,u32)> = vec![
		(3,3),(4,3),(5,3)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((3,3),(3,2,true),None)],
		vec![((4,3),(4,2,true),None),((4,3),(4,2,false),None)],
		vec![((5,3),(5,2,true),None)],
	];

	for (p,answer) in positions.iter().zip(answer) {
		let mut banmen = blank_banmen.clone();
		banmen.0[1][4] = GOu;

		banmen.0[p.1 as usize][p.0 as usize] = SFu;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_fu_nari_move_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let positions:Vec<(u32,u32)> = vec![
		(3,3),(4,3),(5,3)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((3,3),(3,2,true),None)],
		vec![((4,3),(4,2,true),None),((4,3),(4,2,false),None)],
		vec![((5,3),(5,2,true),None)],
	];

	for (p,answer) in positions.iter().zip(answer) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-1][4] = SOu;

		banmen.0[8-p.1 as usize][8-p.0 as usize] = GFu;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kei_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let positions:Vec<(u32,u32)> = vec![
		(2,8),(6,8)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,8),(3,6,false),None)],
		vec![((6,8),(5,6,false),None)],
	];

	for (p,answer) in positions.iter().zip(answer) {
		let mut banmen = blank_banmen.clone();

		banmen.0[4][4] = GOu;

		banmen.0[p.1 as usize][p.0 as usize] = SKei;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kei_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let positions:Vec<(u32,u32)> = vec![
		(2,8),(6,8)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,8),(3,6,false),None)],
		vec![((6,8),(5,6,false),None)],
	];

	for (p,answer) in positions.iter().zip(answer) {
		let mut banmen = blank_banmen.clone();

		banmen.0[4][4] = SOu;

		banmen.0[8-p.1 as usize][8-p.0 as usize] = GKei;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kei_nari_move_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let positions:Vec<(u32,u32)> = vec![
		(2,4),(6,4),(3,4),(5,4)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,4),(3,2,true),None)],
		vec![((6,4),(5,2,true),None)],
		vec![((3,4),(4,2,true),None)],
		vec![((5,4),(4,2,true),None)],
	];

	for (p,answer) in positions.iter().zip(answer) {
		let mut banmen = blank_banmen.clone();

		banmen.0[1][4] = GOu;

		banmen.0[p.1 as usize][p.0 as usize] = SKei;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kei_nari_move_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let positions:Vec<(u32,u32)> = vec![
		(2,4),(6,4),(3,4),(5,4)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,4),(3,2,true),None)],
		vec![((6,4),(5,2,true),None)],
		vec![((3,4),(4,2,true),None)],
		vec![((5,4),(4,2,true),None)],
	];

	for (p,answer) in positions.iter().zip(answer) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-1][8-4] = SOu;

		banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GKei;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_gin_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let positions:Vec<(u32,u32)> = vec![
		(2,8),(3,8),(4,8),(5,8),(6,8),(2,4),(6,4)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,8),(3,7,false),None)],
		vec![((3,8),(3,7,false),None),((3,8),(4,7,false),None)],
		vec![((4,8),(3,7,false),None),((4,8),(4,7,false),None),((4,8),(5,7,false),None)],
		vec![((5,8),(4,7,false),None),((5,8),(5,7,false),None)],
		vec![((6,8),(5,7,false),None)],
		vec![((2,4),(3,5,false),None)],
		vec![((6,4),(5,5,false),None)],
	];

	for (p,answer) in positions.iter().zip(answer) {
		let mut banmen = blank_banmen.clone();

		banmen.0[6][4] = GOu;

		banmen.0[p.1 as usize][p.0 as usize] = SGin;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_gin_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let positions:Vec<(u32,u32)> = vec![
		(2,8),(3,8),(4,8),(5,8),(6,8),(2,4),(6,4)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,8),(3,7,false),None)],
		vec![((3,8),(3,7,false),None),((3,8),(4,7,false),None)],
		vec![((4,8),(3,7,false),None),((4,8),(4,7,false),None),((4,8),(5,7,false),None)],
		vec![((5,8),(4,7,false),None),((5,8),(5,7,false),None)],
		vec![((6,8),(5,7,false),None)],
		vec![((2,4),(3,5,false),None)],
		vec![((6,4),(5,5,false),None)],
	];

	for (p,answer) in positions.iter().zip(answer) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-6][8-4] = SOu;

		banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GGin;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_gin_nari_move_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let positions:Vec<(u32,u32)> = vec![
		(2,3),(3,3),(4,3),(5,3),(6,3),(2,2),(6,2),(2,1),(6,1)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,3),(3,2,true),None),((2,3),(3,2,false),None)],
		vec![((3,3),(3,2,true),None),((3,3),(3,2,false),None),((3,3),(4,2,true),None),((3,3),(4,2,false),None)],
		vec![((4,3),(3,2,true),None),((4,3),(3,2,false),None),((4,3),(4,2,true),None),((4,3),(4,2,false),None),((4,3),(5,2,true),None),((4,3),(5,2,false),None)],
		vec![((5,3),(4,2,true),None),((5,3),(4,2,false),None),((5,3),(5,2,true),None),((5,3),(5,2,false),None)],
		vec![((6,3),(5,2,true),None),((6,3),(5,2,false),None)],
		vec![((2,2),(3,1,true),None)],
		vec![((6,2),(5,1,true),None)],
		vec![((2,1),(3,0,false),None),((2,1),(3,2,true),None),((2,1),(3,2,false),None)],
		vec![((6,1),(5,0,false),None),((6,1),(5,2,true),None),((6,1),(5,2,false),None)],
	];

	for (p,answer) in positions.iter().zip(answer) {
		let mut banmen = blank_banmen.clone();

		banmen.0[1][4] = GOu;

		banmen.0[p.1 as usize][p.0 as usize] = SGin;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_gin_nari_move_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let positions:Vec<(u32,u32)> = vec![
		(2,3),(3,3),(4,3),(5,3),(6,3),(2,2),(6,2),(2,1),(6,1)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,3),(3,2,true),None),((2,3),(3,2,false),None)],
		vec![((3,3),(3,2,true),None),((3,3),(3,2,false),None),((3,3),(4,2,true),None),((3,3),(4,2,false),None)],
		vec![((4,3),(3,2,true),None),((4,3),(3,2,false),None),((4,3),(4,2,true),None),((4,3),(4,2,false),None),((4,3),(5,2,true),None),((4,3),(5,2,false),None)],
		vec![((5,3),(4,2,true),None),((5,3),(4,2,false),None),((5,3),(5,2,true),None),((5,3),(5,2,false),None)],
		vec![((6,3),(5,2,true),None),((6,3),(5,2,false),None)],
		vec![((2,2),(3,1,true),None)],
		vec![((6,2),(5,1,true),None)],
		vec![((2,1),(3,0,false),None),((2,1),(3,2,true),None),((2,1),(3,2,false),None)],
		vec![((6,1),(5,0,false),None),((6,1),(5,2,true),None),((6,1),(5,2,false),None)],
	];

	for (p,answer) in positions.iter().zip(answer) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-1][8-4] = SOu;

		banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GGin;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_oute_only_moves_with_kin_sente_impl(kind:KomaKind) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let positions:Vec<(u32,u32)> = vec![
		(2,8),(3,8),(4,8),(5,8),(6,8),(2,6),(6,6),(4,4)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,8),(3,7,false),None)],
		vec![((3,8),(3,7,false),None),((3,8),(4,7,false),None)],
		vec![((4,8),(3,7,false),None),((4,8),(4,7,false),None),((4,8),(5,7,false),None)],
		vec![((5,8),(4,7,false),None),((5,8),(5,7,false),None)],
		vec![((6,8),(5,7,false),None)],
		vec![((2,6),(3,6,false),None)],
		vec![((6,6),(5,6,false),None)],
		vec![((4,4),(4,5,false),None)],
	];

	for (p,answer) in positions.iter().zip(answer) {
		let mut banmen = blank_banmen.clone();

		banmen.0[6][4] = GOu;

		banmen.0[p.1 as usize][p.0 as usize] = kind;

		assert_eq!(answer.clone().into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Sente,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
fn test_oute_only_moves_with_kin_gote_impl(kind:KomaKind) {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let positions:Vec<(u32,u32)> = vec![
		(2,8),(3,8),(4,8),(5,8),(6,8),(2,6),(6,6),(4,4)
	];

	let answer:Vec<Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)>> = vec![
		vec![((2,8),(3,7,false),None)],
		vec![((3,8),(3,7,false),None),((3,8),(4,7,false),None)],
		vec![((4,8),(3,7,false),None),((4,8),(4,7,false),None),((4,8),(5,7,false),None)],
		vec![((5,8),(4,7,false),None),((5,8),(5,7,false),None)],
		vec![((6,8),(5,7,false),None)],
		vec![((2,6),(3,6,false),None)],
		vec![((6,6),(5,6,false),None)],
		vec![((4,4),(4,5,false),None)],
	];

	for (p,answer) in positions.iter().zip(answer) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-6][8-4] = SOu;

		banmen.0[8 - p.1 as usize][8 - p.0 as usize] = kind;

		assert_eq!(answer.clone().into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_from_banmen(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);

		assert_eq!(answer.into_iter().map(|m| {
				match m {
					((sx,sy),(dx,dy,nari),o) => {
						LegalMove::from(((8-sx,8-sy),(8-dx,8-dy,nari),o))
					}
				}
			}).collect::<Vec<LegalMove>>(),
			Rule::oute_only_moves_all(Teban::Gote,&State::new(banmen.clone()),&MochigomaCollections::Empty).into_iter().map(|m| {
				LegalMove::from(m)
			}).collect::<Vec<LegalMove>>()
		);
	}
}
#[test]
fn test_oute_only_moves_with_kin_sente() {
	test_oute_only_moves_with_kin_sente_impl(SKin);
}
#[test]
fn test_oute_only_moves_with_fu_nari_sente() {
	test_oute_only_moves_with_kin_sente_impl(SFuN);
}
#[test]
fn test_oute_only_moves_with_kyou_nari_sente() {
	test_oute_only_moves_with_kin_sente_impl(SKyouN);
}
#[test]
fn test_oute_only_moves_with_kei_nari_sente() {
	test_oute_only_moves_with_kin_sente_impl(SKeiN);
}
#[test]
fn test_oute_only_moves_with_gin_nari_sente() {
	test_oute_only_moves_with_kin_sente_impl(SGinN);
}
#[test]
fn test_oute_only_moves_with_kin_gonte() {
	test_oute_only_moves_with_kin_gote_impl(GKin);
}
#[test]
fn test_oute_only_moves_with_fu_nari_gote() {
	test_oute_only_moves_with_kin_gote_impl(GFuN);
}
#[test]
fn test_oute_only_moves_with_kyou_nari_gote() {
	test_oute_only_moves_with_kin_gote_impl(GKyouN);
}
#[test]
fn test_oute_only_moves_with_kei_nari_gote() {
	test_oute_only_moves_with_kin_gote_impl(GKeiN);
}
#[test]
fn test_oute_only_moves_with_gin_nari_gote() {
	test_oute_only_moves_with_kin_gote_impl(GGinN);
}
fn test_oute_only_moves_from_mochigoma_none_moves_sente_impl(kind:MochigomaKind) {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = GOu;
	banmen.0[5][3] = SFu;
	banmen.0[5][4] = SFu;
	banmen.0[5][5] = SFu;
	banmen.0[4][3] = SKin;
	banmen.0[4][5] = SKin;
	banmen.0[3][3] = SGin;
	banmen.0[3][4] = SGin;
	banmen.0[3][5] = SKei;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(kind,1);

	let mc = MochigomaCollections::Pair(ms,HashMap::new());

	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![];

	let answer = answer.into_iter().map(|m| {
		LegalMove::from(m)
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
fn test_oute_only_moves_from_mochigoma_none_moves_gote_impl(kind:MochigomaKind) {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[8-4][8-4] = SOu;
	banmen.0[8-5][8-3] = GFu;
	banmen.0[8-5][8-4] = GFu;
	banmen.0[8-5][8-5] = GFu;
	banmen.0[8-4][8-3] = GKin;
	banmen.0[8-4][8-5] = GKin;
	banmen.0[8-3][8-3] = GGin;
	banmen.0[8-3][8-4] = GGin;
	banmen.0[8-3][8-5] = GKei;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(kind,1);

	let mc = MochigomaCollections::Pair(HashMap::new(),mg);

	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![];

	let answer = answer.into_iter().map(|m| {
		LegalMove::from(m)
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_sente_fu() {
	test_oute_only_moves_from_mochigoma_none_moves_sente_impl(MochigomaKind::Fu);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_sente_kyou() {
	test_oute_only_moves_from_mochigoma_none_moves_sente_impl(MochigomaKind::Kyou);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_sente_gin() {
	test_oute_only_moves_from_mochigoma_none_moves_sente_impl(MochigomaKind::Gin);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_sente_kin() {
	test_oute_only_moves_from_mochigoma_none_moves_sente_impl(MochigomaKind::Kin);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_sente_kaku() {
	test_oute_only_moves_from_mochigoma_none_moves_sente_impl(MochigomaKind::Kaku);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_sente_hisha() {
	test_oute_only_moves_from_mochigoma_none_moves_sente_impl(MochigomaKind::Hisha);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_gote_fu() {
	test_oute_only_moves_from_mochigoma_none_moves_gote_impl(MochigomaKind::Fu);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_gote_kyou() {
	test_oute_only_moves_from_mochigoma_none_moves_gote_impl(MochigomaKind::Kyou);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_gote_gin() {
	test_oute_only_moves_from_mochigoma_none_moves_gote_impl(MochigomaKind::Gin);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_gote_kin() {
	test_oute_only_moves_from_mochigoma_none_moves_gote_impl(MochigomaKind::Kin);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_gote_kaku() {
	test_oute_only_moves_from_mochigoma_none_moves_gote_impl(MochigomaKind::Kaku);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_gote_hisha() {
	test_oute_only_moves_from_mochigoma_none_moves_gote_impl(MochigomaKind::Hisha);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_sente_kei() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = GOu;
	banmen.0[5][3] = SFu;
	banmen.0[5][4] = SFu;
	banmen.0[5][5] = SFu;
	banmen.0[6][3] = SKyou;
	banmen.0[6][4] = SKyou;
	banmen.0[6][5] = SKyou;
	banmen.0[4][3] = SKin;
	banmen.0[4][5] = SKin;
	banmen.0[3][3] = SGin;
	banmen.0[3][4] = SGin;
	banmen.0[3][5] = SKei;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Kei,1);

	let mc = MochigomaCollections::Pair(ms,HashMap::new());

	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![];

	let answer = answer.into_iter().map(|m| {
		LegalMove::from(m)
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_none_moves_gote_kei() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[8-4][8-4] = SOu;

	banmen.0[8-5][8-3] = GFu;
	banmen.0[8-5][8-4] = GFu;
	banmen.0[8-5][8-5] = GFu;
	banmen.0[8-6][8-3] = GKyou;
	banmen.0[8-6][8-4] = GKyou;
	banmen.0[8-6][8-5] = GKyou;
	banmen.0[8-4][8-3] = GKin;
	banmen.0[8-4][8-5] = GKin;
	banmen.0[8-3][8-3] = GGin;
	banmen.0[8-3][8-4] = GGin;
	banmen.0[8-3][8-5] = GKei;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Kei,1);

	let mc = MochigomaCollections::Pair(HashMap::new(),mg);

	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![];

	let answer = answer.into_iter().map(|m| {
		LegalMove::from(m)
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_fu_sente() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = GOu;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Fu,1);

	let mc = MochigomaCollections::Pair(ms,HashMap::new());

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Fu,(4,5))
	];

	let answer = answer.into_iter().map(|m| {
		LegalMove::from(m)
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_fu_gote() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = SOu;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Fu,1);

	let mc = MochigomaCollections::Pair(HashMap::new(),mg);

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Fu,(4,5))
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			(kind,(dx,dy)) => {
				LegalMove::from((kind,(8-dx,8-dy)))
			}
		}
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_kyou_sente() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = GOu;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Kyou,1);

	let mc = MochigomaCollections::Pair(ms,HashMap::new());

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Kyou,(4,5)),
		(MochigomaKind::Kyou,(4,6)),
		(MochigomaKind::Kyou,(4,7)),
		(MochigomaKind::Kyou,(4,8))
	];

	let answer = answer.into_iter().map(|m| {
		LegalMove::from(m)
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_kyou_gote() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = SOu;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Kyou,1);

	let mc = MochigomaCollections::Pair(HashMap::new(),mg);

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Kyou,(4,5)),
		(MochigomaKind::Kyou,(4,6)),
		(MochigomaKind::Kyou,(4,7)),
		(MochigomaKind::Kyou,(4,8))
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			(kind,(dx,dy)) => {
				LegalMove::from((kind,(8-dx,8-dy)))
			}
		}
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_kei_sente() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = GOu;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Kei,1);

	let mc = MochigomaCollections::Pair(ms,HashMap::new());

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Kei,(3,6)),
		(MochigomaKind::Kei,(5,6))
	];

	let answer = answer.into_iter().map(|m| {
		LegalMove::from(m)
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_kei_gote() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = SOu;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Kei,1);

	let mc = MochigomaCollections::Pair(HashMap::new(),mg);

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Kei,(3,6)),
		(MochigomaKind::Kei,(5,6))
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			(kind,(dx,dy)) => {
				LegalMove::from((kind,(8-dx,8-dy)))
			}
		}
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_gin_sente() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = GOu;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Gin,1);

	let mc = MochigomaCollections::Pair(ms,HashMap::new());

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Gin,(3,3)),
		(MochigomaKind::Gin,(3,5)),
		(MochigomaKind::Gin,(4,5)),
		(MochigomaKind::Gin,(5,3)),
		(MochigomaKind::Gin,(5,5)),
	];

	let answer = answer.into_iter().map(|m| {
		LegalMove::from(m)
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_gin_gote() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = SOu;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Gin,1);

	let mc = MochigomaCollections::Pair(HashMap::new(),mg);

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Gin,(3,3)),
		(MochigomaKind::Gin,(3,5)),
		(MochigomaKind::Gin,(4,5)),
		(MochigomaKind::Gin,(5,3)),
		(MochigomaKind::Gin,(5,5)),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			(kind,(dx,dy)) => {
				LegalMove::from((kind,(8-dx,8-dy)))
			}
		}
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_kin_sente() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = GOu;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,HashMap::new());

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Kin,(3,4)),
		(MochigomaKind::Kin,(3,5)),
		(MochigomaKind::Kin,(4,3)),
		(MochigomaKind::Kin,(4,5)),
		(MochigomaKind::Kin,(5,4)),
		(MochigomaKind::Kin,(5,5)),
	];

	let answer = answer.into_iter().map(|m| {
		LegalMove::from(m)
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_kin_gote() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = SOu;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(HashMap::new(),mg);

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Kin,(3,4)),
		(MochigomaKind::Kin,(3,5)),
		(MochigomaKind::Kin,(4,3)),
		(MochigomaKind::Kin,(4,5)),
		(MochigomaKind::Kin,(5,4)),
		(MochigomaKind::Kin,(5,5)),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			(kind,(dx,dy)) => {
				LegalMove::from((kind,(8-dx,8-dy)))
			}
		}
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_kaku_sente() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = GOu;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Kaku,1);

	let mc = MochigomaCollections::Pair(ms,HashMap::new());

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Kaku,(0,0)),
		(MochigomaKind::Kaku,(0,8)),
		(MochigomaKind::Kaku,(1,1)),
		(MochigomaKind::Kaku,(1,7)),
		(MochigomaKind::Kaku,(2,2)),
		(MochigomaKind::Kaku,(2,6)),
		(MochigomaKind::Kaku,(3,3)),
		(MochigomaKind::Kaku,(3,5)),
		(MochigomaKind::Kaku,(5,3)),
		(MochigomaKind::Kaku,(5,5)),
		(MochigomaKind::Kaku,(6,2)),
		(MochigomaKind::Kaku,(6,6)),
		(MochigomaKind::Kaku,(7,1)),
		(MochigomaKind::Kaku,(7,7)),
		(MochigomaKind::Kaku,(8,0)),
		(MochigomaKind::Kaku,(8,8)),
	];

	let answer = answer.into_iter().map(|m| {
		LegalMove::from(m)
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_kaku_gote() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = SOu;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Kaku,1);

	let mc = MochigomaCollections::Pair(HashMap::new(),mg);

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Kaku,(0,0)),
		(MochigomaKind::Kaku,(0,8)),
		(MochigomaKind::Kaku,(1,1)),
		(MochigomaKind::Kaku,(1,7)),
		(MochigomaKind::Kaku,(2,2)),
		(MochigomaKind::Kaku,(2,6)),
		(MochigomaKind::Kaku,(3,3)),
		(MochigomaKind::Kaku,(3,5)),
		(MochigomaKind::Kaku,(5,3)),
		(MochigomaKind::Kaku,(5,5)),
		(MochigomaKind::Kaku,(6,2)),
		(MochigomaKind::Kaku,(6,6)),
		(MochigomaKind::Kaku,(7,1)),
		(MochigomaKind::Kaku,(7,7)),
		(MochigomaKind::Kaku,(8,0)),
		(MochigomaKind::Kaku,(8,8)),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			(kind,(dx,dy)) => {
				LegalMove::from((kind,(8-dx,8-dy)))
			}
		}
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_hisha_sente() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = GOu;

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Hisha,1);

	let mc = MochigomaCollections::Pair(ms,HashMap::new());

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Hisha,(0,4)),
		(MochigomaKind::Hisha,(1,4)),
		(MochigomaKind::Hisha,(2,4)),
		(MochigomaKind::Hisha,(3,4)),
		(MochigomaKind::Hisha,(4,0)),
		(MochigomaKind::Hisha,(4,1)),
		(MochigomaKind::Hisha,(4,2)),
		(MochigomaKind::Hisha,(4,3)),
		(MochigomaKind::Hisha,(4,5)),
		(MochigomaKind::Hisha,(4,6)),
		(MochigomaKind::Hisha,(4,7)),
		(MochigomaKind::Hisha,(4,8)),
		(MochigomaKind::Hisha,(5,4)),
		(MochigomaKind::Hisha,(6,4)),
		(MochigomaKind::Hisha,(7,4)),
		(MochigomaKind::Hisha,(8,4)),
	];

	let answer = answer.into_iter().map(|m| {
		LegalMove::from(m)
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_oute_only_moves_from_mochigoma_hisha_gote() {
	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[4][4] = SOu;

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Hisha,1);

	let mc = MochigomaCollections::Pair(HashMap::new(),mg);

	let answer:Vec<(MochigomaKind,(u32,u32))> = vec![
		(MochigomaKind::Hisha,(0,4)),
		(MochigomaKind::Hisha,(1,4)),
		(MochigomaKind::Hisha,(2,4)),
		(MochigomaKind::Hisha,(3,4)),
		(MochigomaKind::Hisha,(4,0)),
		(MochigomaKind::Hisha,(4,1)),
		(MochigomaKind::Hisha,(4,2)),
		(MochigomaKind::Hisha,(4,3)),
		(MochigomaKind::Hisha,(4,5)),
		(MochigomaKind::Hisha,(4,6)),
		(MochigomaKind::Hisha,(4,7)),
		(MochigomaKind::Hisha,(4,8)),
		(MochigomaKind::Hisha,(5,4)),
		(MochigomaKind::Hisha,(6,4)),
		(MochigomaKind::Hisha,(7,4)),
		(MochigomaKind::Hisha,(8,4)),
	];

	let answer = answer.into_iter().map(|m| {
		match m {
			(kind,(dx,dy)) => {
				LegalMove::from((kind,(8-dx,8-dy)))
			}
		}
	}).collect::<Vec<LegalMove>>();

	assert_eq!(answer,
		Rule::oute_only_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen)).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_respond_oute_only_moves_all_sente() {
	let mvs:Vec<Vec<Move>> = vec![
		vec![ Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,7+1)) ],
		vec![]
	];

	let answer:Vec<Vec<Move>> =  vec![
		vec![
			Move::To(KomaSrcPosition(9-7,7+1),KomaDstToPosition(9-4,7+1,false)),
			Move::To(KomaSrcPosition(9-3,8+1),KomaDstToPosition(9-4,7+1,false)),
			Move::To(KomaSrcPosition(9-4,8+1),KomaDstToPosition(9-3,7+1,false)),
			Move::To(KomaSrcPosition(9-4,8+1),KomaDstToPosition(9-5,7+1,false)),
			Move::To(KomaSrcPosition(9-5,8+1),KomaDstToPosition(9-4,7+1,false)),
		],
		vec![
			Move::To(KomaSrcPosition(9-7,7+1),KomaDstToPosition(9-4,7+1,false)),
			Move::To(KomaSrcPosition(9-3,8+1),KomaDstToPosition(9-4,7+1,false)),
			Move::To(KomaSrcPosition(9-4,8+1),KomaDstToPosition(9-3,7+1,false)),
			Move::To(KomaSrcPosition(9-4,8+1),KomaDstToPosition(9-5,7+1,false)),
			Move::To(KomaSrcPosition(9-5,8+1),KomaDstToPosition(9-4,7+1,false)),
			Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,3+1)),
			Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,4+1)),
			Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,5+1)),
			Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,6+1)),
			Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,7+1)),
		]
	];

	let mut base_banmen = BANMEN_START_POS.clone();

	base_banmen.0[2][4] = GKyou;
	base_banmen.0[6][4] = Blank;
	base_banmen.0[0][0] = Blank;

	for (mvs,answer) in mvs.into_iter().zip(answer.into_iter()) {
		let mut banmen = base_banmen.clone();

		let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

		ms.insert(MochigomaKind::Fu,1);

		let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

		mg.insert(MochigomaKind::Fu,1);

		let mut mc = MochigomaCollections::Pair(ms,mg);

		let mut state = State::new(banmen.clone());

		for m in mvs {
			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}
		}

		assert_eq!(answer,
			Rule::respond_oute_only_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
				m.to_applied_move().to_move()
			}).collect::<Vec<Move>>()
		);
	}
}
#[test]
fn test_respond_oute_only_moves_all_win_move_sente() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((4,1),(4,0,false),Some(ObtainKind::Ou))
	];

	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[8][4] = SOu;
	banmen.0[6][3] = GKyou;
	banmen.0[6][4] = GKyou;
	banmen.0[6][5] = GKyou;
	banmen.0[0][4] = GOu;
	banmen.0[1][4] = SKin;

	let mc = MochigomaCollections::Pair(HashMap::new(),HashMap::new());

	let state = State::new(banmen);

	assert_eq!(answer.into_iter().map(|m| LegalMove::from(m)).collect::<Vec<LegalMove>>(),
		Rule::respond_oute_only_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_respond_oute_only_moves_all_gote() {
	let mvs:Vec<Vec<Move>> = vec![
		vec![ Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,7+1)) ],
		vec![]
	];

	let answer:Vec<Vec<Move>> =  vec![
		vec![
			Move::To(KomaSrcPosition(9-7,7+1),KomaDstToPosition(9-4,7+1,false)),
			Move::To(KomaSrcPosition(9-3,8+1),KomaDstToPosition(9-4,7+1,false)),
			Move::To(KomaSrcPosition(9-4,8+1),KomaDstToPosition(9-3,7+1,false)),
			Move::To(KomaSrcPosition(9-4,8+1),KomaDstToPosition(9-5,7+1,false)),
			Move::To(KomaSrcPosition(9-5,8+1),KomaDstToPosition(9-4,7+1,false)),
		],
		vec![
			Move::To(KomaSrcPosition(9-7,7+1),KomaDstToPosition(9-4,7+1,false)),
			Move::To(KomaSrcPosition(9-3,8+1),KomaDstToPosition(9-4,7+1,false)),
			Move::To(KomaSrcPosition(9-4,8+1),KomaDstToPosition(9-3,7+1,false)),
			Move::To(KomaSrcPosition(9-4,8+1),KomaDstToPosition(9-5,7+1,false)),
			Move::To(KomaSrcPosition(9-5,8+1),KomaDstToPosition(9-4,7+1,false)),
			Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,3+1)),
			Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,4+1)),
			Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,5+1)),
			Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,6+1)),
			Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,7+1)),
		]
	];

	let mut base_banmen = BANMEN_START_POS.clone();

	base_banmen.0[8-2][8-4] = SKyou;
	base_banmen.0[8-6][8-4] = Blank;
	base_banmen.0[8-0][8-0] = Blank;

	for (mvs,answer) in mvs.into_iter().zip(answer.into_iter()) {
		let mut banmen = base_banmen.clone();

		let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

		ms.insert(MochigomaKind::Fu,1);

		let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

		mg.insert(MochigomaKind::Fu,1);

		let mut mc = MochigomaCollections::Pair(ms,mg);

		let mut state = State::new(banmen.clone());

		let mvs = mvs.into_iter().map(|m| {
			match m {
				Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,n)) => {
					let sx = 9 - sx;
					let sy = sy - 1;
					let dx = 9 - dx;
					let dy = dy - 1;

					let sx = 8 - sx;
					let sy = 8 - sy;
					let dx = 8 - dx;
					let dy = 8 - dy;

					Move::To(KomaSrcPosition(9-sx,sy+1),KomaDstToPosition(9-dx,dy+1,n))
				},
				Move::Put(kind,KomaDstPutPosition(dx,dy)) => {
					let dx = 9 - dx;
					let dy = dy - 1;

					let dx = 8 - dx;
					let dy = 8 - dy;

					Move::Put(kind,KomaDstPutPosition(9-dx,dy+1))
				}
			}
		}).collect::<Vec<Move>>();

		for m in mvs {
			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}
		}

		let answer = answer.into_iter().map(|m| {
			match m {
				Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,n)) => {
					let sx = 9 - sx;
					let sy = sy - 1;
					let dx = 9 - dx;
					let dy = dy - 1;

					let sx = 8 - sx;
					let sy = 8 - sy;
					let dx = 8 - dx;
					let dy = 8 - dy;

					Move::To(KomaSrcPosition(9-sx,sy+1),KomaDstToPosition(9-dx,dy+1,n))
				},
				Move::Put(kind,KomaDstPutPosition(dx,dy)) => {
					let dx = 9 - dx;
					let dy = dy - 1;

					let dx = 8 - dx;
					let dy = 8 - dy;

					Move::Put(kind,KomaDstPutPosition(9-dx,dy+1))
				}
			}
		}).collect::<Vec<Move>>();

		assert_eq!(answer,
			Rule::respond_oute_only_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
				m.to_applied_move().to_move()
			}).collect::<Vec<Move>>()
		);
	}
}
#[test]
fn test_respond_oute_only_moves_all_win_move_gote() {
	let answer:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((8-4,8-1),(8-4,8-0,false),Some(ObtainKind::Ou))
	];

	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[8-8][8-4] = GOu;
	banmen.0[8-6][8-3] = SKyou;
	banmen.0[8-6][8-4] = SKyou;
	banmen.0[8-6][8-5] = SKyou;
	banmen.0[8-0][8-4] = SOu;
	banmen.0[8-1][8-4] = GKin;

	let mc = MochigomaCollections::Pair(HashMap::new(),HashMap::new());

	let state = State::new(banmen);

	assert_eq!(answer.into_iter().map(|m| LegalMove::from(m)).collect::<Vec<LegalMove>>(),
		Rule::respond_oute_only_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
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
			Move::To(KomaSrcPosition(9-2,0+1),KomaDstToPosition(9-2,1+1,false)),
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
			Move::To(KomaSrcPosition(9-2,0+1),KomaDstToPosition(9-2,1+1,false)),
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
#[test]
fn test_apply_move_to_partial_state_none_check_sente() {
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
			Move::To(KomaSrcPosition(9-2,0+1),KomaDstToPosition(9-2,1+1,false)),
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

	let answer_state = State::new(banmen.clone());
	let answer = answer_state.get_part();

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

	assert_eq!(answer,
		state.get_part()
	);
}
#[test]
fn test_apply_move_to_partial_state_none_check_gote() {
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
			Move::To(KomaSrcPosition(9-2,0+1),KomaDstToPosition(9-2,1+1,false)),
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

	let answer_state = State::new(banmen.clone());
	let answer = answer_state.get_part();

	assert_eq!(answer,
		state.get_part()
	);
}
#[test]
fn test_is_valid_move_valid_to_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 5] = [
		(4,4),(2,3),(6,3),(4,2),(4,1)
	];

	let mvs:Vec<Vec<Vec<(u32,u32,bool)>>> = vec![
		// 歩
		vec![
			vec![
				(4,3,true),(4,3,false)
			],
			vec![
				(2,2,true),(2,2,false)
			],
			vec![
				(6,2,true),(6,2,false)
			],
			vec![
				(4,1,true),(4,1,false)
			],
			vec![
				(4,1,true)
			]
		],
		// 香車
		vec![
			vec![
				(4,3,true),(4,3,false),
				(4,2,true),(4,2,false),
				(4,1,true),(4,1,false),
				(4,0,true),
			],
			vec![
				(2,2,true),(2,2,false),
				(2,1,true),(2,1,false),
				(2,0,true),
			],
			vec![
				(6,2,true),(6,2,false),
				(6,1,true),(6,1,false),
				(6,0,true),
			],
			vec![
				(4,1,true),(4,1,false),
				(4,0,true)
			],
			vec![
				(4,0,true)
			]
		],
		// 桂馬
		vec![
			vec![
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			],
			vec![
				(1,1,true),
				(3,1,true)
			],
			vec![
				(4,1,true),
				(8,1,true)
			],
			vec![
				(2,0,true),
				(6,0,true)
			],
			vec![]
		],
		// 銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,5,false),
				(5,5,false)
			],
			vec![
				(1,2,true),(1,2,false),
				(2,2,true),(2,2,false),
				(3,2,true),(3,2,false),
				(1,4,false),
				(3,4,false)
			],
			vec![
				(5,2,true),(5,2,false),
				(6,2,true),(6,2,false),
				(7,2,true),(7,2,false),
				(5,4,false),
				(7,4,false)
			],
			vec![
				(3,1,true),(3,1,false),
				(4,1,true),(4,1,false),
				(5,1,true),(5,1,false),
				(3,3,true),(3,3,false),
				(5,3,true),(5,3,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(4,0,true),(4,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			]
		],
		// 金,成歩,成香,成桂,成銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(4,5,false)
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(2,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(6,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(4,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(4,2,false)
			]
		],
		// 角
		vec![
			vec![
				(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false),
				(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false)
			],
			vec![
				(1,2,true),(1,2,false),(0,1,true),(0,1,false),
				(3,2,true),(3,2,false),(4,1,true),(4,1,false),(5,0,true),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false)
			],
			vec![
				(5,2,true),(5,2,false),(4,1,true),(4,1,false),(3,0,true),(3,0,false),
				(7,2,true),(7,2,false),(8,1,true),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false)
			],
			vec![
				(3,1,true),(3,1,false),(2,0,true),(2,0,false),
				(5,1,true),(5,1,false),(6,0,true),(6,0,false),
				(3,3,true),(3,3,false),(2,4,true),(2,4,false),(1,5,true),(1,5,false),(0,6,true),(0,6,false),
				(5,3,true),(5,3,false),(6,4,true),(6,4,false),(7,5,true),(7,5,false),(8,4,true),(8,4,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),(2,3,true),(2,3,false),(1,4,true),(1,4,false),(0,5,true),(0,5,false),
				(5,2,true),(5,2,false),(6,3,true),(6,3,false),(7,4,true),(7,4,false),(8,5,true),(8,5,false)
			]
		],
		// 飛車
		vec![
			vec![
				(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false)
			],
			vec![
				(2,2,true),(2,2,false),(2,1,true),(2,1,false),(2,0,true),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false)
			],
			vec![
				(6,2,true),(6,2,false),(6,1,true),(6,1,false),(6,0,true),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false)
			],
			vec![
				(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,2,true),(3,2,false),(2,2,true),(2,2,false),(1,2,true),(1,2,false),(0,2,true),(0,2,false),
				(5,2,true),(5,2,false),(6,2,true),(6,2,false),(7,2,true),(7,2,false),(8,2,true),(8,2,false),
			],
			vec![
				(4,0,true),(4,0,false),
				(4,2,true),(4,2,false),(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,1,true),(3,1,false),(2,1,true),(2,1,false),(1,1,true),(1,1,false),(0,1,true),(0,1,false),
				(5,1,true),(5,1,false),(6,1,true),(6,1,false),(7,1,true),(7,1,false),(8,1,true),(8,1,false),
			]
		],
		// 成角
		vec![
			vec![
				(3,3,false),(2,2,false),(1,1,false),(0,0,false),
				(5,3,false),(6,2,false),(7,1,false),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false),
				(4,3,false),(4,5,false),(3,4,false),(5,4,false)
			],
			vec![
				(1,2,false),(0,1,false),
				(3,2,false),(4,1,false),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false),
				(2,2,false),(2,4,false),(1,3,false),(3,3,false),
			],
			vec![
				(5,2,false),(4,1,false),(3,0,false),
				(7,2,false),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false),
				(6,2,false),(6,4,false),(5,3,false),(7,3,false),
			],
			vec![
				(3,1,false),(2,0,false),
				(5,1,false),(6,0,false),
				(3,3,false),(2,4,false),(1,5,false),(0,6,false),
				(5,3,false),(6,4,false),(7,5,false),(8,4,false),
				(4,1,false),(4,3,false),(3,2,false),(5,2,false)
			],
			vec![
				(3,0,false),
				(5,0,false),
				(3,2,false),(2,3,false),(1,4,false),(0,5,false),
				(5,2,false),(6,3,false),(7,4,false),(8,5,false),
				(4,0,false),(4,2,false),(3,1,false),(5,1,false)
			]
		],
		// 成飛
		vec![
			vec![
				(4,3,false),(4,2,false),(4,1,false),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false),
				(3,3,false),(5,3,false),(3,5,false),(5,5,false),
			],
			vec![
				(2,2,false),(2,1,false),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false),
				(1,2,false),(3,2,false),(1,4,false),(3,4,false)
			],
			vec![
				(6,2,false),(6,1,false),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false),
				(5,2,false),(7,2,false),(5,4,false),(7,4,false)
			],
			vec![
				(4,1,false),(4,0,false),
				(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,2,false),(2,2,false),(1,2,false),(0,2,false),
				(5,2,false),(6,2,false),(7,2,false),(8,2,false),
				(3,1,false),(5,1,false),(3,3,false),(5,3,false)
			],
			vec![
				(4,0,false),
				(4,2,false),(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,1,false),(2,1,false),(1,1,false),(0,1,false),
				(5,1,false),(6,1,false),(7,1,false),(8,1,false),
				(3,0,false),(5,0,false),(3,1,false),(5,1,false)
			]
		],
		// 王
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(3,5,false),
				(4,5,false),
				(5,5,false),
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(1,4,false),
				(2,4,false),
				(3,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(5,4,false),
				(6,4,false),
				(7,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(3,3,false),
				(4,3,false),
				(5,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(3,2,false),
				(4,2,false),
				(5,2,false)
			]
		]
	];

	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ SFu ],
		vec![ SKyou ],
		vec![ SKei ],
		vec![ SGin ],
		vec![ SKin,SFuN,SKyouN,SKeiN,SGinN ],
		vec![ SKaku ],
		vec![ SHisha ],
		vec![ SKakuN ],
		vec![ SHishaN ],
		vec![ SOu ]
	];

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for ((k,p),mvs) in kinds.iter().zip(&POSITIONS).zip(mvs) {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[p.1 as usize][p.0 as usize] = *k;

				let dx = m.0;
				let dy = m.1;

				let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-dx,dy+1,m.2)));

				let state = State::new(banmen);

				assert!(Rule::is_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					m
				), format!("is_valid_move: kind = {:?}, move = {:?} is false.", k,m.to_move()));

				let mut banmen = blank_banmen.clone();

				banmen.0[p.1 as usize][p.0 as usize] = *k;
				banmen.0[dy as usize][dx as usize] = GFu;

				let state = State::new(banmen);

				assert!(Rule::is_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					m
				), format!("is_valid_move to occupied: kind = {:?}, move = {:?} is false.", k,m.to_move()));
			}
		}
	}
}
#[test]
fn test_is_valid_move_valid_to_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 5] = [
		(4,4),(2,3),(6,3),(4,2),(4,1)
	];

	let mvs:Vec<Vec<Vec<(u32,u32,bool)>>> = vec![
		// 歩
		vec![
			vec![
				(4,3,true),(4,3,false)
			],
			vec![
				(2,2,true),(2,2,false)
			],
			vec![
				(6,2,true),(6,2,false)
			],
			vec![
				(4,1,true),(4,1,false)
			],
			vec![
				(4,1,true)
			]
		],
		// 香車
		vec![
			vec![
				(4,3,true),(4,3,false),
				(4,2,true),(4,2,false),
				(4,1,true),(4,1,false),
				(4,0,true),
			],
			vec![
				(2,2,true),(2,2,false),
				(2,1,true),(2,1,false),
				(2,0,true),
			],
			vec![
				(6,2,true),(6,2,false),
				(6,1,true),(6,1,false),
				(6,0,true),
			],
			vec![
				(4,1,true),(4,1,false),
				(4,0,true)
			],
			vec![
				(4,0,true)
			]
		],
		// 桂馬
		vec![
			vec![
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			],
			vec![
				(1,1,true),
				(3,1,true)
			],
			vec![
				(4,1,true),
				(8,1,true)
			],
			vec![
				(2,0,true),
				(6,0,true)
			],
			vec![]
		],
		// 銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,5,false),
				(5,5,false)
			],
			vec![
				(1,2,true),(1,2,false),
				(2,2,true),(2,2,false),
				(3,2,true),(3,2,false),
				(1,4,false),
				(3,4,false)
			],
			vec![
				(5,2,true),(5,2,false),
				(6,2,true),(6,2,false),
				(7,2,true),(7,2,false),
				(5,4,false),
				(7,4,false)
			],
			vec![
				(3,1,true),(3,1,false),
				(4,1,true),(4,1,false),
				(5,1,true),(5,1,false),
				(3,3,true),(3,3,false),
				(5,3,true),(5,3,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(4,0,true),(4,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			]
		],
		// 金,成歩,成香,成桂,成銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(4,5,false)
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(2,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(6,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(4,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(4,2,false)
			]
		],
		// 角
		vec![
			vec![
				(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false),
				(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false)
			],
			vec![
				(1,2,true),(1,2,false),(0,1,true),(0,1,false),
				(3,2,true),(3,2,false),(4,1,true),(4,1,false),(5,0,true),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false)
			],
			vec![
				(5,2,true),(5,2,false),(4,1,true),(4,1,false),(3,0,true),(3,0,false),
				(7,2,true),(7,2,false),(8,1,true),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false)
			],
			vec![
				(3,1,true),(3,1,false),(2,0,true),(2,0,false),
				(5,1,true),(5,1,false),(6,0,true),(6,0,false),
				(3,3,true),(3,3,false),(2,4,true),(2,4,false),(1,5,true),(1,5,false),(0,6,true),(0,6,false),
				(5,3,true),(5,3,false),(6,4,true),(6,4,false),(7,5,true),(7,5,false),(8,4,true),(8,4,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),(2,3,true),(2,3,false),(1,4,true),(1,4,false),(0,5,true),(0,5,false),
				(5,2,true),(5,2,false),(6,3,true),(6,3,false),(7,4,true),(7,4,false),(8,5,true),(8,5,false)
			]
		],
		// 飛車
		vec![
			vec![
				(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false)
			],
			vec![
				(2,2,true),(2,2,false),(2,1,true),(2,1,false),(2,0,true),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false)
			],
			vec![
				(6,2,true),(6,2,false),(6,1,true),(6,1,false),(6,0,true),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false)
			],
			vec![
				(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,2,true),(3,2,false),(2,2,true),(2,2,false),(1,2,true),(1,2,false),(0,2,true),(0,2,false),
				(5,2,true),(5,2,false),(6,2,true),(6,2,false),(7,2,true),(7,2,false),(8,2,true),(8,2,false),
			],
			vec![
				(4,0,true),(4,0,false),
				(4,2,true),(4,2,false),(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,1,true),(3,1,false),(2,1,true),(2,1,false),(1,1,true),(1,1,false),(0,1,true),(0,1,false),
				(5,1,true),(5,1,false),(6,1,true),(6,1,false),(7,1,true),(7,1,false),(8,1,true),(8,1,false),
			]
		],
		// 成角
		vec![
			vec![
				(3,3,false),(2,2,false),(1,1,false),(0,0,false),
				(5,3,false),(6,2,false),(7,1,false),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false),
				(4,3,false),(4,5,false),(3,4,false),(5,4,false)
			],
			vec![
				(1,2,false),(0,1,false),
				(3,2,false),(4,1,false),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false),
				(2,2,false),(2,4,false),(1,3,false),(3,3,false),
			],
			vec![
				(5,2,false),(4,1,false),(3,0,false),
				(7,2,false),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false),
				(6,2,false),(6,4,false),(5,3,false),(7,3,false),
			],
			vec![
				(3,1,false),(2,0,false),
				(5,1,false),(6,0,false),
				(3,3,false),(2,4,false),(1,5,false),(0,6,false),
				(5,3,false),(6,4,false),(7,5,false),(8,4,false),
				(4,1,false),(4,3,false),(3,2,false),(5,2,false)
			],
			vec![
				(3,0,false),
				(5,0,false),
				(3,2,false),(2,3,false),(1,4,false),(0,5,false),
				(5,2,false),(6,3,false),(7,4,false),(8,5,false),
				(4,0,false),(4,2,false),(3,1,false),(5,1,false)
			]
		],
		// 成飛
		vec![
			vec![
				(4,3,false),(4,2,false),(4,1,false),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false),
				(3,3,false),(5,3,false),(3,5,false),(5,5,false),
			],
			vec![
				(2,2,false),(2,1,false),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false),
				(1,2,false),(3,2,false),(1,4,false),(3,4,false)
			],
			vec![
				(6,2,false),(6,1,false),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false),
				(5,2,false),(7,2,false),(5,4,false),(7,4,false)
			],
			vec![
				(4,1,false),(4,0,false),
				(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,2,false),(2,2,false),(1,2,false),(0,2,false),
				(5,2,false),(6,2,false),(7,2,false),(8,2,false),
				(3,1,false),(5,1,false),(3,3,false),(5,3,false)
			],
			vec![
				(4,0,false),
				(4,2,false),(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,1,false),(2,1,false),(1,1,false),(0,1,false),
				(5,1,false),(6,1,false),(7,1,false),(8,1,false),
				(3,0,false),(5,0,false),(3,1,false),(5,1,false)
			]
		],
		// 王
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(3,5,false),
				(4,5,false),
				(5,5,false),
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(1,4,false),
				(2,4,false),
				(3,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(5,4,false),
				(6,4,false),
				(7,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(3,3,false),
				(4,3,false),
				(5,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(3,2,false),
				(4,2,false),
				(5,2,false)
			]
		]
	];

	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ GFu ],
		vec![ GKyou ],
		vec![ GKei ],
		vec![ GGin ],
		vec![ GKin,GFuN,GKyouN,GKeiN,GGinN ],
		vec![ GKaku ],
		vec![ GHisha ],
		vec![ GKakuN ],
		vec![ GHishaN ],
		vec![ GOu ]
	];

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for ((k,p),mvs) in kinds.iter().zip(&POSITIONS).zip(mvs) {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

				let dx = m.0;
				let dy = m.1;

				let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-dx),(8-dy)+1,m.2)));

				let state = State::new(banmen);

				assert!(Rule::is_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					m
				), format!("is_valid_move: kind = {:?}, move = {:?} is false.", k,m.to_move()));

				let mut banmen = blank_banmen.clone();

				banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;
				banmen.0[8 - dy as usize][8 - dx as usize] = SFu;

				let state = State::new(banmen);

				assert!(Rule::is_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					m
				), format!("is_valid_move to occupied: kind = {:?}, move = {:?} is false.", k,m.to_move()));
			}
		}
	}
}
#[test]
fn test_is_valid_move_valid_with_kyou_opponent_occupied_sente() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,4,false),(4,3,false),(4,2,true),(4,2,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[5][4] = SKyou;
		banmen.0[2][4] = GFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-4,5+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

		let state = State::new(banmen);

		assert!(Rule::is_valid_move(&state,
			Teban::Sente,
			&MochigomaCollections::Empty,
			m
		), format!("is_valid_move: move = {:?} is false.", m.to_move()));
	}
}
#[test]
fn test_is_valid_move_valid_with_kyou_opponent_occupied_gote() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,4,false),(4,3,false),(4,2,true),(4,2,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-5][8-4] = GKyou;
		banmen.0[8-2][8-4] = SFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-4),(8-5)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

		let state = State::new(banmen);

		assert!(Rule::is_valid_move(&state,
			Teban::Gote,
			&MochigomaCollections::Empty,
			m
		), format!("is_valid_move: move = {:?} is false.", m.to_move()));
	}
}
#[test]
fn test_is_valid_move_valid_with_kyou_self_occupied_sente() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,4,false),(4,3,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[5][4] = SKyou;
		banmen.0[2][4] = SFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-4,5+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

		let state = State::new(banmen);

		assert!(Rule::is_valid_move(&state,
			Teban::Sente,
			&MochigomaCollections::Empty,
			m
		), format!("is_valid_move: move = {:?} is false.", m.to_move()));
	}
}
#[test]
fn test_is_valid_move_valid_with_kyou_self_occupied_gote() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,4,false),(4,3,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-5][8-4] = GKyou;
		banmen.0[8-2][8-4] = GFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-4),(8-5)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

		let state = State::new(banmen);

		assert!(Rule::is_valid_move(&state,
			Teban::Gote,
			&MochigomaCollections::Empty,
			m
		), format!("is_valid_move: move = {:?} is false.", m.to_move()));
	}
}
#[test]
fn test_is_valid_move_valid_with_kaku_opponent_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(2,2,true),(2,2,false),(3,3,true),(3,3,false),(4,4,true),(4,4,false)
		],
		vec![
			(2,6,false),(3,5,false),(4,4,false)
		],
		vec![
			(6,2,true),(6,2,false),(5,3,true),(5,3,false),(4,4,true),(4,4,false)
		],
		vec![
			(6,6,false),(5,5,false),(4,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SKaku;
			banmen.0[4][4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::is_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is false.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_valid_with_kaku_opponent_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(2,2,true),(2,2,false),(3,3,true),(3,3,false),(4,4,true),(4,4,false)
		],
		vec![
			(2,6,false),(3,5,false),(4,4,false)
		],
		vec![
			(6,2,true),(6,2,false),(5,3,true),(5,3,false),(4,4,true),(4,4,false)
		],
		vec![
			(6,6,false),(5,5,false),(4,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GKaku;
			banmen.0[4][4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::is_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is false.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_valid_with_kaku_self_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(2,2,true),(2,2,false),(3,3,true),(3,3,false)
		],
		vec![
			(2,6,false),(3,5,false)
		],
		vec![
			(6,2,true),(6,2,false),(5,3,true),(5,3,false)
		],
		vec![
			(6,6,false),(5,5,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SKaku;
			banmen.0[4][4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::is_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is false.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_valid_with_kaku_self_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(2,2,true),(2,2,false),(3,3,true),(3,3,false)
		],
		vec![
			(2,6,false),(3,5,false)
		],
		vec![
			(6,2,true),(6,2,false),(5,3,true),(5,3,false)
		],
		vec![
			(6,6,false),(5,5,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GKaku;
			banmen.0[4][4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::is_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is false.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_valid_with_hisha_opponent_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,6,false),(4,5,false),(4,4,false)
		],
		vec![
			(2,4,false),(3,4,false),(4,4,false)
		],
		vec![
			(4,2,true),(4,2,false),(4,3,true),(4,3,false),(4,4,true),(4,4,false)
		],
		vec![
			(6,4,false),(5,4,false),(4,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SHisha;
			banmen.0[4][4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::is_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is false.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_valid_with_hisha_opponent_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,6,false),(4,5,false),(4,4,false)
		],
		vec![
			(2,4,false),(3,4,false),(4,4,false)
		],
		vec![
			(4,2,true),(4,2,false),(4,3,true),(4,3,false),(4,4,true),(4,4,false)
		],
		vec![
			(6,4,false),(5,4,false),(4,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GHisha;
			banmen.0[8-4][8-4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::is_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is false.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_valid_with_hisha_self_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,6,false),(4,5,false)
		],
		vec![
			(2,4,false),(3,4,false)
		],
		vec![
			(4,2,true),(4,2,false),(4,3,true),(4,3,false)
		],
		vec![
			(6,4,false),(5,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SHisha;
			banmen.0[4][4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::is_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is false.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_valid_with_hisha_self_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,6,false),(4,5,false)
		],
		vec![
			(2,4,false),(3,4,false)
		],
		vec![
			(4,2,true),(4,2,false),(4,3,true),(4,3,false)
		],
		vec![
			(6,4,false),(5,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GHisha;
			banmen.0[8-4][8-4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::is_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is false.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_invalid_with_kyou_opponent_occupied_sente() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,1,true),(4,1,false),(4,0,true),(4,0,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[5][4] = SKyou;
		banmen.0[2][4] = GFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-4,5+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

		let state = State::new(banmen);

		assert!(!Rule::is_valid_move(&state,
			Teban::Sente,
			&MochigomaCollections::Empty,
			m
		), format!("is_valid_move: move = {:?} is true.", m.to_move()));
	}
}
#[test]
fn test_is_valid_move_invalid_with_kyou_opponent_occupied_gote() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,1,true),(4,1,false),(4,0,true),(4,0,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-5][8-4] = GKyou;
		banmen.0[8-2][8-4] = SFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-4),(8-5)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

		let state = State::new(banmen);

		assert!(!Rule::is_valid_move(&state,
			Teban::Gote,
			&MochigomaCollections::Empty,
			m
		), format!("is_valid_move: move = {:?} is true.", m.to_move()));
	}
}
#[test]
fn test_is_valid_move_invalid_with_kyou_self_occupied_sente() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[5][4] = SKyou;
		banmen.0[2][4] = SFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-4,5+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

		let state = State::new(banmen);

		assert!(!Rule::is_valid_move(&state,
			Teban::Sente,
			&MochigomaCollections::Empty,
			m
		), format!("is_valid_move: move = {:?} is true.", m.to_move()));
	}
}
#[test]
fn test_is_valid_move_invalid_with_kyou_self_occupied_gote() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-5][8-4] = GKyou;
		banmen.0[8-2][8-4] = GFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-4),(8-5)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

		let state = State::new(banmen);

		assert!(!Rule::is_valid_move(&state,
			Teban::Gote,
			&MochigomaCollections::Empty,
			m
		), format!("is_valid_move: move = {:?} is true.", m.to_move()));
	}
}
#[test]
fn test_is_valid_move_invalid_with_kaku_opponent_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(5,5,true),(5,5,false),(6,6,true),(6,6,false),(7,7,true),(7,7,false),(8,8,true),(8,8,false)
		],
		vec![
			(5,3,true),(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false)
		],
		vec![
			(3,5,true),(3,5,false),(2,6,true),(2,6,false),(1,7,true),(1,7,false),(0,8,true),(0,8,false)
		],
		vec![
			(3,3,true),(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SKaku;
			banmen.0[4][4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(!Rule::is_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is true.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_invalid_with_kaku_opponent_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(5,5,true),(5,5,false),(6,6,true),(6,6,false),(7,7,true),(7,7,false),(8,8,true),(8,8,false)
		],
		vec![
			(5,3,true),(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false)
		],
		vec![
			(3,5,true),(3,5,false),(2,6,true),(2,6,false),(1,7,true),(1,7,false),(0,8,true),(0,8,false)
		],
		vec![
			(3,3,true),(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GKaku;
			banmen.0[4][4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(!Rule::is_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is true.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_invalid_with_kaku_self_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,4,true),(4,4,false),(5,5,true),(5,5,false),(6,6,true),(6,6,false),(7,7,true),(7,7,false),(8,8,true),(8,8,false)
		],
		vec![
			(4,4,true),(4,4,false),(5,3,true),(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false)
		],
		vec![
			(4,4,true),(4,4,false),(3,5,true),(3,5,false),(2,6,true),(2,6,false),(1,7,true),(1,7,false),(0,8,true),(0,8,false)
		],
		vec![
			(4,4,true),(4,4,false),(3,3,true),(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SKaku;
			banmen.0[4][4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(!Rule::is_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is true.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_invalid_with_kaku_self_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,4,true),(4,4,false),(5,5,true),(5,5,false),(6,6,true),(6,6,false),(7,7,true),(7,7,false),(8,8,true),(8,8,false)
		],
		vec![
			(4,4,true),(4,4,false),(5,3,true),(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false)
		],
		vec![
			(4,4,true),(4,4,false),(3,5,true),(3,5,false),(2,6,true),(2,6,false),(1,7,true),(1,7,false),(0,8,true),(0,8,false)
		],
		vec![
			(4,4,true),(4,4,false),(3,3,true),(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GKaku;
			banmen.0[4][4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(!Rule::is_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is true.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_invalid_with_hisha_opponent_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,3,true),(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false)
		],
		vec![
			(5,4,true),(5,4,false),(6,4,true),(6,4,false),(7,4,true),(7,4,false),(8,4,true),(8,4,false)
		],
		vec![
			(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false)
		],
		vec![
			(3,4,true),(3,4,false),(2,4,true),(2,4,false),(1,4,true),(1,4,false),(0,4,true),(0,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SHisha;
			banmen.0[4][4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(!Rule::is_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is true.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_invalid_with_hisha_opponent_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,3,true),(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false)
		],
		vec![
			(5,4,true),(5,4,false),(6,4,true),(6,4,false),(7,4,true),(7,4,false),(8,4,true),(8,4,false)
		],
		vec![
			(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false)
		],
		vec![
			(3,4,true),(3,4,false),(2,4,true),(2,4,false),(1,4,true),(1,4,false),(0,4,true),(0,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GHisha;
			banmen.0[8-4][8-4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(!Rule::is_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is true.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_invalid_with_hisha_self_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,4,true),(4,4,false),(4,3,true),(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false)
		],
		vec![
			(4,4,true),(4,4,false),(5,4,true),(5,4,false),(6,4,true),(6,4,false),(7,4,true),(7,4,false),(8,4,true),(8,4,false)
		],
		vec![
			(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false)
		],
		vec![
			(4,4,true),(4,4,false),(3,4,true),(3,4,false),(2,4,true),(2,4,false),(1,4,true),(1,4,false),(0,4,true),(0,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SHisha;
			banmen.0[4][4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(!Rule::is_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is true.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_invalid_with_hisha_self_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,4,true),(4,4,false),(4,3,true),(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false)
		],
		vec![
			(4,4,true),(4,4,false),(5,4,true),(5,4,false),(6,4,true),(6,4,false),(7,4,true),(7,4,false),(8,4,true),(8,4,false)
		],
		vec![
			(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false)
		],
		vec![
			(4,4,true),(4,4,false),(3,4,true),(3,4,false),(2,4,true),(2,4,false),(1,4,true),(1,4,false),(0,4,true),(0,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GHisha;
			banmen.0[8-4][8-4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(!Rule::is_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			), format!("is_valid_move: move = {:?} is true.", m.to_move()));
		}
	}
}
#[test]
fn test_is_valid_move_invalid_to_outside_sente() {
	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ SFu ],
		vec![ SKyou ],
		vec![ SKei ],
		vec![ SGin ],
		vec![ SKin,SFuN,SKyouN,SKeiN,SGinN ],
		vec![ SKaku ],
		vec![ SHisha ],
		vec![ SKakuN ],
		vec![ SHishaN ]
	];

	let mvs:Vec<Vec<((u32,u32),(u32,u32))>> = vec![
		vec![
			((8,0),(7,8))
		],
		vec![
			((8,1),(7,8))
		],
		vec![
			((7,1),(7,8))
		],
		vec![
			((0,0),(1,8)),((0,0),(2,8)),
			((0,8),(2,0)),
			((8,0),(6,8)),((8,0),(7,8)),((8,0),(8,8)),((8,0),(9,1)),
			((8,8),(9,7)),((8,8),(9,9)),((8,8),(8,0))
		],
		vec![
			((0,0),(1,8)),((0,0),(2,8)),
			((0,8),(1,0)),
			((8,0),(6,8)),((8,0),(7,8)),((8,0),(8,8)),((8,0),(9,0)),
			((8,8),(7,0)),((8,8),(9,7)),((8,8),(9,8))
		],
		vec![
			((0,0),(2,8)),
			((0,8),(2,0)),
			((8,0),(6,8)),((8,0),(8,8)),((8,0),(9,1)),
			((8,8),(6,0)),((8,8),(9,7)),((8,8),(9,9))
		],
		vec![
			((0,8),(1,0)),
			((8,0),(7,8)),((8,0),(9,0)),
			((8,8),(7,0)),((8,8),(8,9)),((8,8),(9,8))
		],
		vec![
			((0,0),(2,8)),
			((0,8),(1,0)),((0,8),(2,0)),
			((8,0),(7,8)),((8,0),(9,0)),((8,0),(6,8)),((8,0),(8,8)),((8,0),(9,1)),
			((8,8),(7,0)),((8,8),(8,9)),((8,8),(9,8)),((8,8),(6,0)),((8,8),(9,7)),((8,8),(9,9))
		],
		vec![
			((0,0),(2,8)),
			((0,8),(1,0)),((0,8),(2,0)),
			((8,0),(7,8)),((8,0),(9,0)),((8,0),(6,8)),((8,0),(9,1)),
			((8,8),(7,0)),((8,8),(8,9)),((8,8),(9,8)),((8,8),(6,0)),((8,8),(9,7)),((8,8),(9,9))
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for k in kinds {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[(m.0).1 as usize][(m.0).0 as usize] = *k;

				let mv = rule::LegalMove::To(rule::LegalMoveTo::new(
												((m.0).0 * 9 + (m.0).1) as u32,
												((m.1).0 * 9 + (m.1).1) as u32,true,None)).to_applied_move();

				let state = State::new(banmen);

				assert!(!Rule::is_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					mv
				), format!("is_valid_move: kind = {:?}, move = {:?} is true.", k, mv.to_move()));

				let mv = rule::LegalMove::To(rule::LegalMoveTo::new(
												((m.0).0 * 9 + (m.0).1) as u32,
												((m.1).0 * 9 + (m.1).1) as u32,false,None)).to_applied_move();

				assert!(!Rule::is_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					mv
				), format!("is_valid_move: kind = {:?}, move = {:?} is true.", k, mv.to_move()));
			}
		}
	}
}
#[test]
fn test_is_valid_move_invalid_to_outside_gote() {
	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ GFu ],
		vec![ GKyou ],
		vec![ GKei ],
		vec![ GGin ],
		vec![ GKin,GFuN,GKyouN,GKeiN,GGinN ],
		vec![ GKaku ],
		vec![ GHisha ],
		vec![ GKakuN ],
		vec![ GHishaN ]
	];

	let mvs:Vec<Vec<((u32,u32),(u32,u32))>> = vec![
		vec![
			((7,8),(8,0)),((8,8),(9,0))
		],
		vec![
			((7,7),(8,0)),((8,7),(9,0))
		],
		vec![
			((8,7),(6,0)),((8,7),(9,9))
		],
		vec![
			((0,0),(0,8)),
			((0,8),(1,0)),((0,8),(2,0)),
			((8,0),(6,8)),((8,0),(8,8)),((8,0),(9,1)),
			((8,8),(8,0)),((8,8),(9,0)),((8,8),(9,9)),((8,8),(9,7))
		],
		vec![
			((0,8),(1,0)),((0,8),(2,0)),
			((8,0),(7,8)),((8,0),(9,0)),((8,0),(9,1)),
			((8,8),(8,0)),((8,8),(9,0)),((8,8),(9,8)),((8,8),(9,9))
		],
		vec![
			((0,0),(2,8)),
			((0,8),(2,0)),
			((8,0),(6,8)),((8,0),(8,8)),((8,0),(9,1)),
			((8,8),(6,0)),((8,8),(9,7)),((8,8),(9,9))
		],
		vec![
			((0,8),(1,0)),
			((8,0),(7,8)),((8,0),(9,0)),
			((8,8),(7,0)),((8,8),(8,9)),((8,8),(9,8))
		],
		vec![
			((0,0),(2,8)),
			((0,8),(1,0)),((0,8),(2,0)),
			((8,0),(7,8)),((8,0),(9,0)),((8,0),(6,8)),((8,0),(8,8)),((8,0),(9,1)),
			((8,8),(7,0)),((8,8),(8,9)),((8,8),(9,8)),((8,8),(6,0)),((8,8),(9,7)),((8,8),(9,9))
		],
		vec![
			((0,0),(2,8)),
			((0,8),(1,0)),((0,8),(2,0)),
			((8,0),(7,8)),((8,0),(9,0)),((8,0),(6,8)),((8,0),(9,1)),
			((8,8),(7,0)),((8,8),(8,9)),((8,8),(9,8)),((8,8),(6,0)),((8,8),(9,7)),((8,8),(9,9))
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for k in kinds {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[8 - (m.0).1 as usize][8 - (m.0).0 as usize] = *k;

				let mv = rule::LegalMove::To(rule::LegalMoveTo::new(
												((m.0).0 * 9 + (m.0).1) as u32,
												((m.1).0 * 9 + (m.1).1) as u32,true,None)).to_applied_move();

				let state = State::new(banmen);

				assert!(!Rule::is_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					mv
				), format!("is_valid_move: kind = {:?}, move = {:?} is true.", k, mv.to_move()));

				let mv = rule::LegalMove::To(rule::LegalMoveTo::new(
												((m.0).0 * 9 + (m.0).1) as u32,
												((m.1).0 * 9 + (m.1).1) as u32,false,None)).to_applied_move();

				assert!(!Rule::is_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					mv
				), format!("is_valid_move: kind = {:?}, move = {:?} is true.", k, mv.to_move()));
			}
		}
	}
}
#[test]
fn test_is_valid_move_invalid_to_self_occupied_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 5] = [
		(4,4),(2,3),(6,3),(4,2),(4,1)
	];

	let mvs:Vec<Vec<Vec<(u32,u32,bool)>>> = vec![
		// 歩
		vec![
			vec![
				(4,3,true),(4,3,false)
			],
			vec![
				(2,2,true),(2,2,false)
			],
			vec![
				(6,2,true),(6,2,false)
			],
			vec![
				(4,1,true),(4,1,false)
			],
			vec![
				(4,1,true)
			]
		],
		// 香車
		vec![
			vec![
				(4,3,true),(4,3,false),
				(4,2,true),(4,2,false),
				(4,1,true),(4,1,false),
				(4,0,true),
			],
			vec![
				(2,2,true),(2,2,false),
				(2,1,true),(2,1,false),
				(2,0,true),
			],
			vec![
				(6,2,true),(6,2,false),
				(6,1,true),(6,1,false),
				(6,0,true),
			],
			vec![
				(4,1,true),(4,1,false),
				(4,0,true)
			],
			vec![
				(4,0,true)
			]
		],
		// 桂馬
		vec![
			vec![
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			],
			vec![
				(1,1,true),
				(3,1,true)
			],
			vec![
				(4,1,true),
				(8,1,true)
			],
			vec![
				(2,0,true),
				(6,0,true)
			],
			vec![]
		],
		// 銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,5,false),
				(5,5,false)
			],
			vec![
				(1,2,true),(1,2,false),
				(2,2,true),(2,2,false),
				(3,2,true),(3,2,false),
				(1,4,false),
				(3,4,false)
			],
			vec![
				(5,2,true),(5,2,false),
				(6,2,true),(6,2,false),
				(7,2,true),(7,2,false),
				(5,4,false),
				(7,4,false)
			],
			vec![
				(3,1,true),(3,1,false),
				(4,1,true),(4,1,false),
				(5,1,true),(5,1,false),
				(3,3,true),(3,3,false),
				(5,3,true),(5,3,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(4,0,true),(4,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			]
		],
		// 金,成歩,成香,成桂,成銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(4,5,false)
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(2,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(6,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(4,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(4,2,false)
			]
		],
		// 角
		vec![
			vec![
				(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false),
				(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false)
			],
			vec![
				(1,2,true),(1,2,false),(0,1,true),(0,1,false),
				(3,2,true),(3,2,false),(4,1,true),(4,1,false),(5,0,true),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false)
			],
			vec![
				(5,2,true),(5,2,false),(4,1,true),(4,1,false),(3,0,true),(3,0,false),
				(7,2,true),(7,2,false),(8,1,true),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false)
			],
			vec![
				(3,1,true),(3,1,false),(2,0,true),(2,0,false),
				(5,1,true),(5,1,false),(6,0,true),(6,0,false),
				(3,3,true),(3,3,false),(2,4,true),(2,4,false),(1,5,true),(1,5,false),(0,6,true),(0,6,false),
				(5,3,true),(5,3,false),(6,4,true),(6,4,false),(7,5,true),(7,5,false),(8,4,true),(8,4,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),(2,3,true),(2,3,false),(1,4,true),(1,4,false),(0,5,true),(0,5,false),
				(5,2,true),(5,2,false),(6,3,true),(6,3,false),(7,4,true),(7,4,false),(8,5,true),(8,5,false)
			]
		],
		// 飛車
		vec![
			vec![
				(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false)
			],
			vec![
				(2,2,true),(2,2,false),(2,1,true),(2,1,false),(2,0,true),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false)
			],
			vec![
				(6,2,true),(6,2,false),(6,1,true),(6,1,false),(6,0,true),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false)
			],
			vec![
				(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,2,true),(3,2,false),(2,2,true),(2,2,false),(1,2,true),(1,2,false),(0,2,true),(0,2,false),
				(5,2,true),(5,2,false),(6,2,true),(6,2,false),(7,2,true),(7,2,false),(8,2,true),(8,2,false),
			],
			vec![
				(4,0,true),(4,0,false),
				(4,2,true),(4,2,false),(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,1,true),(3,1,false),(2,1,true),(2,1,false),(1,1,true),(1,1,false),(0,1,true),(0,1,false),
				(5,1,true),(5,1,false),(6,1,true),(6,1,false),(7,1,true),(7,1,false),(8,1,true),(8,1,false),
			]
		],
		// 成角
		vec![
			vec![
				(3,3,false),(2,2,false),(1,1,false),(0,0,false),
				(5,3,false),(6,2,false),(7,1,false),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false),
				(4,3,false),(4,5,false),(3,4,false),(5,4,false)
			],
			vec![
				(1,2,false),(0,1,false),
				(3,2,false),(4,1,false),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false),
				(2,2,false),(2,4,false),(1,3,false),(3,3,false),
			],
			vec![
				(5,2,false),(4,1,false),(3,0,false),
				(7,2,false),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false),
				(6,2,false),(6,4,false),(5,3,false),(7,3,false),
			],
			vec![
				(3,1,false),(2,0,false),
				(5,1,false),(6,0,false),
				(3,3,false),(2,4,false),(1,5,false),(0,6,false),
				(5,3,false),(6,4,false),(7,5,false),(8,4,false),
				(4,1,false),(4,3,false),(3,2,false),(5,2,false)
			],
			vec![
				(3,0,false),
				(5,0,false),
				(3,2,false),(2,3,false),(1,4,false),(0,5,false),
				(5,2,false),(6,3,false),(7,4,false),(8,5,false),
				(4,0,false),(4,2,false),(3,1,false),(5,1,false)
			]
		],
		// 成飛
		vec![
			vec![
				(4,3,false),(4,2,false),(4,1,false),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false),
				(3,3,false),(5,3,false),(3,5,false),(5,5,false),
			],
			vec![
				(2,2,false),(2,1,false),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false),
				(1,2,false),(3,2,false),(1,4,false),(3,4,false)
			],
			vec![
				(6,2,false),(6,1,false),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false),
				(5,2,false),(7,2,false),(5,4,false),(7,4,false)
			],
			vec![
				(4,1,false),(4,0,false),
				(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,2,false),(2,2,false),(1,2,false),(0,2,false),
				(5,2,false),(6,2,false),(7,2,false),(8,2,false),
				(3,1,false),(5,1,false),(3,3,false),(5,3,false)
			],
			vec![
				(4,0,false),
				(4,2,false),(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,1,false),(2,1,false),(1,1,false),(0,1,false),
				(5,1,false),(6,1,false),(7,1,false),(8,1,false),
				(3,0,false),(5,0,false),(3,1,false),(5,1,false)
			]
		],
		// 王
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(3,5,false),
				(4,5,false),
				(5,5,false),
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(1,4,false),
				(2,4,false),
				(3,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(5,4,false),
				(6,4,false),
				(7,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(3,3,false),
				(4,3,false),
				(5,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(3,2,false),
				(4,2,false),
				(5,2,false)
			]
		]
	];

	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ SFu ],
		vec![ SKyou ],
		vec![ SKei ],
		vec![ SGin ],
		vec![ SKin,SFuN,SKyouN,SKeiN,SGinN ],
		vec![ SKaku ],
		vec![ SHisha ],
		vec![ SKakuN ],
		vec![ SHishaN ],
		vec![ SOu ]
	];

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for ((k,p),mvs) in kinds.iter().zip(&POSITIONS).zip(mvs) {
			for m in mvs {
				let dx = m.0;
				let dy = m.1;
				let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-dx,dy+1,m.2)));
				let mut banmen = blank_banmen.clone();

				banmen.0[p.1 as usize][p.0 as usize] = *k;
				banmen.0[dy as usize][dx as usize] = SFu;

				let state = State::new(banmen);

				assert!(!Rule::is_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					m
				), format!("is_valid_move to occupied: kind = {:?}, move = {:?} is true.", k,m.to_move()));
			}
		}
	}
}
#[test]
fn test_is_valid_move_invalid_to_self_occupied_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 5] = [
		(4,4),(2,3),(6,3),(4,2),(4,1)
	];

	let mvs:Vec<Vec<Vec<(u32,u32,bool)>>> = vec![
		// 歩
		vec![
			vec![
				(4,3,true),(4,3,false)
			],
			vec![
				(2,2,true),(2,2,false)
			],
			vec![
				(6,2,true),(6,2,false)
			],
			vec![
				(4,1,true),(4,1,false)
			],
			vec![
				(4,1,true)
			]
		],
		// 香車
		vec![
			vec![
				(4,3,true),(4,3,false),
				(4,2,true),(4,2,false),
				(4,1,true),(4,1,false),
				(4,0,true),
			],
			vec![
				(2,2,true),(2,2,false),
				(2,1,true),(2,1,false),
				(2,0,true),
			],
			vec![
				(6,2,true),(6,2,false),
				(6,1,true),(6,1,false),
				(6,0,true),
			],
			vec![
				(4,1,true),(4,1,false),
				(4,0,true)
			],
			vec![
				(4,0,true)
			]
		],
		// 桂馬
		vec![
			vec![
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			],
			vec![
				(1,1,true),
				(3,1,true)
			],
			vec![
				(4,1,true),
				(8,1,true)
			],
			vec![
				(2,0,true),
				(6,0,true)
			],
			vec![]
		],
		// 銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,5,false),
				(5,5,false)
			],
			vec![
				(1,2,true),(1,2,false),
				(2,2,true),(2,2,false),
				(3,2,true),(3,2,false),
				(1,4,false),
				(3,4,false)
			],
			vec![
				(5,2,true),(5,2,false),
				(6,2,true),(6,2,false),
				(7,2,true),(7,2,false),
				(5,4,false),
				(7,4,false)
			],
			vec![
				(3,1,true),(3,1,false),
				(4,1,true),(4,1,false),
				(5,1,true),(5,1,false),
				(3,3,true),(3,3,false),
				(5,3,true),(5,3,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(4,0,true),(4,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			]
		],
		// 金,成歩,成香,成桂,成銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(4,5,false)
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(2,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(6,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(4,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(4,2,false)
			]
		],
		// 角
		vec![
			vec![
				(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false),
				(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false)
			],
			vec![
				(1,2,true),(1,2,false),(0,1,true),(0,1,false),
				(3,2,true),(3,2,false),(4,1,true),(4,1,false),(5,0,true),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false)
			],
			vec![
				(5,2,true),(5,2,false),(4,1,true),(4,1,false),(3,0,true),(3,0,false),
				(7,2,true),(7,2,false),(8,1,true),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false)
			],
			vec![
				(3,1,true),(3,1,false),(2,0,true),(2,0,false),
				(5,1,true),(5,1,false),(6,0,true),(6,0,false),
				(3,3,true),(3,3,false),(2,4,true),(2,4,false),(1,5,true),(1,5,false),(0,6,true),(0,6,false),
				(5,3,true),(5,3,false),(6,4,true),(6,4,false),(7,5,true),(7,5,false),(8,4,true),(8,4,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),(2,3,true),(2,3,false),(1,4,true),(1,4,false),(0,5,true),(0,5,false),
				(5,2,true),(5,2,false),(6,3,true),(6,3,false),(7,4,true),(7,4,false),(8,5,true),(8,5,false)
			]
		],
		// 飛車
		vec![
			vec![
				(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false)
			],
			vec![
				(2,2,true),(2,2,false),(2,1,true),(2,1,false),(2,0,true),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false)
			],
			vec![
				(6,2,true),(6,2,false),(6,1,true),(6,1,false),(6,0,true),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false)
			],
			vec![
				(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,2,true),(3,2,false),(2,2,true),(2,2,false),(1,2,true),(1,2,false),(0,2,true),(0,2,false),
				(5,2,true),(5,2,false),(6,2,true),(6,2,false),(7,2,true),(7,2,false),(8,2,true),(8,2,false),
			],
			vec![
				(4,0,true),(4,0,false),
				(4,2,true),(4,2,false),(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,1,true),(3,1,false),(2,1,true),(2,1,false),(1,1,true),(1,1,false),(0,1,true),(0,1,false),
				(5,1,true),(5,1,false),(6,1,true),(6,1,false),(7,1,true),(7,1,false),(8,1,true),(8,1,false),
			]
		],
		// 成角
		vec![
			vec![
				(3,3,false),(2,2,false),(1,1,false),(0,0,false),
				(5,3,false),(6,2,false),(7,1,false),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false),
				(4,3,false),(4,5,false),(3,4,false),(5,4,false)
			],
			vec![
				(1,2,false),(0,1,false),
				(3,2,false),(4,1,false),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false),
				(2,2,false),(2,4,false),(1,3,false),(3,3,false),
			],
			vec![
				(5,2,false),(4,1,false),(3,0,false),
				(7,2,false),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false),
				(6,2,false),(6,4,false),(5,3,false),(7,3,false),
			],
			vec![
				(3,1,false),(2,0,false),
				(5,1,false),(6,0,false),
				(3,3,false),(2,4,false),(1,5,false),(0,6,false),
				(5,3,false),(6,4,false),(7,5,false),(8,4,false),
				(4,1,false),(4,3,false),(3,2,false),(5,2,false)
			],
			vec![
				(3,0,false),
				(5,0,false),
				(3,2,false),(2,3,false),(1,4,false),(0,5,false),
				(5,2,false),(6,3,false),(7,4,false),(8,5,false),
				(4,0,false),(4,2,false),(3,1,false),(5,1,false)
			]
		],
		// 成飛
		vec![
			vec![
				(4,3,false),(4,2,false),(4,1,false),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false),
				(3,3,false),(5,3,false),(3,5,false),(5,5,false),
			],
			vec![
				(2,2,false),(2,1,false),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false),
				(1,2,false),(3,2,false),(1,4,false),(3,4,false)
			],
			vec![
				(6,2,false),(6,1,false),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false),
				(5,2,false),(7,2,false),(5,4,false),(7,4,false)
			],
			vec![
				(4,1,false),(4,0,false),
				(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,2,false),(2,2,false),(1,2,false),(0,2,false),
				(5,2,false),(6,2,false),(7,2,false),(8,2,false),
				(3,1,false),(5,1,false),(3,3,false),(5,3,false)
			],
			vec![
				(4,0,false),
				(4,2,false),(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,1,false),(2,1,false),(1,1,false),(0,1,false),
				(5,1,false),(6,1,false),(7,1,false),(8,1,false),
				(3,0,false),(5,0,false),(3,1,false),(5,1,false)
			]
		],
		// 王
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(3,5,false),
				(4,5,false),
				(5,5,false),
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(1,4,false),
				(2,4,false),
				(3,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(5,4,false),
				(6,4,false),
				(7,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(3,3,false),
				(4,3,false),
				(5,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(3,2,false),
				(4,2,false),
				(5,2,false)
			]
		]
	];

	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ GFu ],
		vec![ GKyou ],
		vec![ GKei ],
		vec![ GGin ],
		vec![ GKin,GFuN,GKyouN,GKeiN,GGinN ],
		vec![ GKaku ],
		vec![ GHisha ],
		vec![ GKakuN ],
		vec![ GHishaN ],
		vec![ GOu ]
	];

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for ((k,p),mvs) in kinds.iter().zip(&POSITIONS).zip(mvs) {
			for m in mvs {
				let dx = m.0;
				let dy = m.1;
				let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-dx),(8-dy)+1,m.2)));
				let mut banmen = blank_banmen.clone();

				banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;
				banmen.0[8 - dy as usize][8 - dx as usize] = GFu;

				let state = State::new(banmen);

				assert!(!Rule::is_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					m
				), format!("is_valid_move to occupied: kind = {:?}, move = {:?} is true.", k,m.to_move()));
			}
		}
	}
}
#[test]
fn test_is_valid_move_invalid_to_invalid_direction_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 5] = [
		(4,4),(2,3),(6,3),(4,2),(4,1)
	];

	let mvs:Vec<Vec<Vec<(u32,u32)>>> = vec![
		// 歩
		vec![
			vec![
				(3,3),(3,4),(3,5),(4,5),(5,3),(5,4),(5,5)
			],
			vec![
				(1,2),(1,3),(1,4),(2,4),(3,2),(3,3),(3,4)
			],
			vec![
				(5,2),(5,3),(5,4),(6,4),(7,2),(7,3),(7,4)
			],
			vec![
				(2,1),(2,2),(3,3),(4,3),(5,1),(5,2),(5,3)
			],
			vec![
				(2,0),(2,1),(3,2),(4,2),(5,0),(5,1),(5,2)
			]
		],
		// 香車
		vec![
			vec![
				(4,5),(4,6),(4,7),(4,8)
			],
			vec![
				(2,5),(2,6),(2,7),(2,8)
			],
			vec![
				(6,5),(6,6),(6,7),(6,8)
			],
			vec![
				(4,3),(4,4),(4,5),(4,6),(4,7),(4,8)
			],
			vec![
				(4,2),(4,3),(4,4),(4,5),(4,6),(4,7),(4,8)
			]
		],
		// 桂馬
		vec![
			vec![
				(3,3),(3,4),(3,5),(4,3),(4,5),(5,3),(5,4),(5,5)
			],
			vec![
				(1,3),(1,4),(1,5),(2,3),(2,5),(3,3),(3,4),(3,5)
			],
			vec![
				(5,3),(5,4),(5,5),(6,3),(6,5),(7,3),(7,4),(7,5)
			],
			vec![
				(3,1),(3,2),(3,3),(4,1),(4,3),(5,1),(5,2),(5,3)
			],
			vec![
				(3,0),(3,1),(3,2),(4,0),(4,2),(5,0),(5,1),(5,2)
			]
		],
		// 銀
		vec![
			vec![
				(3,4),(4,5),(5,4)
			],
			vec![
				(1,4),(2,5),(3,4)
			],
			vec![
				(5,4),(6,5),(7,4)
			],
			vec![
				(3,2),(4,3),(5,2)
			],
			vec![
				(3,1),(4,2),(5,1)
			]
		],
		// 金,成歩,成香,成桂,成銀
		vec![
			vec![
				(3,5),(5,5)
			],
			vec![
				(1,5),(3,5)
			],
			vec![
				(5,5),(7,5)
			],
			vec![
				(3,3),(5,3)
			],
			vec![
				(3,2),(5,2)
			]
		],
		// 角
		vec![
			vec![
				(3,4),(4,3),(4,5),(5,4)
			],
			vec![
				(1,4),(2,3),(2,5),(3,4)
			],
			vec![
				(5,4),(6,3),(6,5),(7,4)
			],
			vec![
				(3,2),(4,1),(4,3),(5,2)
			],
			vec![
				(3,1),(4,0),(4,2),(5,1)
			]
		],
		// 飛車
		vec![
			vec![
				(3,3),(3,5),(5,3),(5,5)
			],
			vec![
				(1,3),(1,5),(3,3),(3,5)
			],
			vec![
				(5,3),(5,5),(7,3),(7,5)
			],
			vec![
				(3,1),(3,3),(5,1),(5,3)
			],
			vec![
				(3,0),(3,2),(5,0),(5,2)
			]
		],
		// 成角
		vec![
			vec![
				(2,3),(2,4),(2,5),(3,2),(3,6),(4,2),(4,6),(5,2),(5,6),(6,3),(6,4),(6,5)
			],
			vec![
				(0,3),(0,4),(0,5),(1,2),(1,6),(2,2),(2,6),(3,2),(3,6),(4,3),(4,4),(4,5)
			],
			vec![
				(4,3),(4,4),(4,5),(5,2),(5,6),(6,2),(6,6),(7,2),(7,6),(8,3),(8,4),(8,5)
			],
			vec![
				(2,1),(2,2),(2,3),(3,0),(3,4),(4,0),(4,4),(5,0),(5,4),(6,1),(6,2),(6,3)
			],
			vec![
				(2,0),(2,1),(2,2),(3,3),(4,3),(5,3),(6,0),(6,1),(6,2)
			]
		],
		// 成飛
		vec![
			vec![
				(2,2),(2,3),(2,5),(2,6),(3,2),(3,6),(5,2),(5,6),(6,2),(6,3),(6,5),(6,6)
			],
			vec![
				(0,2),(0,4),(0,5),(0,6),(1,2),(1,6),(2,6),(3,2),(3,6),(4,2),(4,4),(4,5),(4,6)
			],
			vec![
				(4,2),(4,4),(4,5),(4,6),(5,6),(6,2),(7,2),(7,6),(8,2),(8,4),(8,5),(8,6)
			],
			vec![
				(2,0),(2,1),(2,3),(2,4),(3,0),(3,4),(5,0),(5,4),(6,0),(6,1),(6,3),(6,4)
			],
			vec![
				(2,0),(2,2),(2,3),(3,3),(5,3),(6,0),(6,2),(6,3)
			]
		],
		// 王
		vec![
			vec![
				(2,2),(2,3),(2,4),(2,5),(2,6),(3,2),(3,6),(4,2),(4,6),(5,2),(5,6),(6,2),(6,3),(6,4),(6,5),(6,6)
			],
			vec![
				(0,2),(0,3),(0,4),(0,5),(0,6),(1,2),(1,6),(2,2),(2,6),(3,2),(3,6),(4,2),(4,3),(4,4),(4,5),(4,6)
			],
			vec![
				(4,2),(4,3),(4,4),(4,5),(4,6),(5,2),(5,6),(6,2),(6,6),(7,2),(7,6),(8,2),(8,3),(8,4),(8,5),(8,6)
			],
			vec![
				(2,0),(2,1),(2,2),(2,3),(2,4),(3,0),(3,4),(4,0),(4,4),(5,0),(5,4),(6,0),(6,1),(6,2),(6,3),(6,4)
			],
			vec![
				(2,0),(2,1),(2,2),(2,3),(3,3),(4,3),(5,3),(6,0),(6,1),(6,2),(6,3)
			]
		]
	];

	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ SFu ],
		vec![ SKyou ],
		vec![ SKei ],
		vec![ SGin ],
		vec![ SKin,SFuN,SKyouN,SKeiN,SGinN ],
		vec![ SKaku ],
		vec![ SHisha ],
		vec![ SKakuN ],
		vec![ SHishaN ],
		vec![ SOu ]
	];

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for ((k,p),mvs) in kinds.iter().zip(&POSITIONS).zip(mvs) {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[p.1 as usize][p.0 as usize] = *k;

				let state = State::new(banmen);

				let dx = m.0;
				let dy = m.1;

				let mv = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-dx,dy+1,true)));

				assert!(!Rule::is_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					mv
				), format!("is_valid_move to occupied: kind = {:?}, move = {:?} is true.", k,mv.to_move()));

				let mv = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-dx,dy+1,false)));

				assert!(!Rule::is_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					mv
				), format!("is_valid_move to occupied: kind = {:?}, move = {:?} is true.", k,mv.to_move()));
			}
		}
	}
}
#[test]
fn test_is_valid_move_invalid_to_invalid_direction_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 5] = [
		(4,4),(2,3),(6,3),(4,2),(4,1)
	];

	let mvs:Vec<Vec<Vec<(u32,u32)>>> = vec![
		// 歩
		vec![
			vec![
				(3,3),(3,4),(3,5),(4,5),(5,3),(5,4),(5,5)
			],
			vec![
				(1,2),(1,3),(1,4),(2,4),(3,2),(3,3),(3,4)
			],
			vec![
				(5,2),(5,3),(5,4),(6,4),(7,2),(7,3),(7,4)
			],
			vec![
				(2,1),(2,2),(3,3),(4,3),(5,1),(5,2),(5,3)
			],
			vec![
				(2,0),(2,1),(3,2),(4,2),(5,0),(5,1),(5,2)
			]
		],
		// 香車
		vec![
			vec![
				(4,5),(4,6),(4,7),(4,8)
			],
			vec![
				(2,5),(2,6),(2,7),(2,8)
			],
			vec![
				(6,5),(6,6),(6,7),(6,8)
			],
			vec![
				(4,3),(4,4),(4,5),(4,6),(4,7),(4,8)
			],
			vec![
				(4,2),(4,3),(4,4),(4,5),(4,6),(4,7),(4,8)
			]
		],
		// 桂馬
		vec![
			vec![
				(3,3),(3,4),(3,5),(4,3),(4,5),(5,3),(5,4),(5,5)
			],
			vec![
				(1,3),(1,4),(1,5),(2,3),(2,5),(3,3),(3,4),(3,5)
			],
			vec![
				(5,3),(5,4),(5,5),(6,3),(6,5),(7,3),(7,4),(7,5)
			],
			vec![
				(3,1),(3,2),(3,3),(4,1),(4,3),(5,1),(5,2),(5,3)
			],
			vec![
				(3,0),(3,1),(3,2),(4,0),(4,2),(5,0),(5,1),(5,2)
			]
		],
		// 銀
		vec![
			vec![
				(3,4),(4,5),(5,4)
			],
			vec![
				(1,4),(2,5),(3,4)
			],
			vec![
				(5,4),(6,5),(7,4)
			],
			vec![
				(3,2),(4,3),(5,2)
			],
			vec![
				(3,1),(4,2),(5,1)
			]
		],
		// 金,成歩,成香,成桂,成銀
		vec![
			vec![
				(3,5),(5,5)
			],
			vec![
				(1,5),(3,5)
			],
			vec![
				(5,5),(7,5)
			],
			vec![
				(3,3),(5,3)
			],
			vec![
				(3,2),(5,2)
			]
		],
		// 角
		vec![
			vec![
				(3,4),(4,3),(4,5),(5,4)
			],
			vec![
				(1,4),(2,3),(2,5),(3,4)
			],
			vec![
				(5,4),(6,3),(6,5),(7,4)
			],
			vec![
				(3,2),(4,1),(4,3),(5,2)
			],
			vec![
				(3,1),(4,0),(4,2),(5,1)
			]
		],
		// 飛車
		vec![
			vec![
				(3,3),(3,5),(5,3),(5,5)
			],
			vec![
				(1,3),(1,5),(3,3),(3,5)
			],
			vec![
				(5,3),(5,5),(7,3),(7,5)
			],
			vec![
				(3,1),(3,3),(5,1),(5,3)
			],
			vec![
				(3,0),(3,2),(5,0),(5,2)
			]
		],
		// 成角
		vec![
			vec![
				(2,3),(2,4),(2,5),(3,2),(3,6),(4,2),(4,6),(5,2),(5,6),(6,3),(6,4),(6,5)
			],
			vec![
				(0,3),(0,4),(0,5),(1,2),(1,6),(2,2),(2,6),(3,2),(3,6),(4,3),(4,4),(4,5)
			],
			vec![
				(4,3),(4,4),(4,5),(5,2),(5,6),(6,2),(6,6),(7,2),(7,6),(8,3),(8,4),(8,5)
			],
			vec![
				(2,1),(2,2),(2,3),(3,0),(3,4),(4,0),(4,4),(5,0),(5,4),(6,1),(6,2),(6,3)
			],
			vec![
				(2,0),(2,1),(2,2),(3,3),(4,3),(5,3),(6,0),(6,1),(6,2)
			]
		],
		// 成飛
		vec![
			vec![
				(2,2),(2,3),(2,5),(2,6),(3,2),(3,6),(5,2),(5,6),(6,2),(6,3),(6,5),(6,6)
			],
			vec![
				(0,2),(0,4),(0,5),(0,6),(1,2),(1,6),(2,6),(3,2),(3,6),(4,2),(4,4),(4,5),(4,6)
			],
			vec![
				(4,2),(4,4),(4,5),(4,6),(5,6),(6,2),(7,2),(7,6),(8,2),(8,4),(8,5),(8,6)
			],
			vec![
				(2,0),(2,1),(2,3),(2,4),(3,0),(3,4),(5,0),(5,4),(6,0),(6,1),(6,3),(6,4)
			],
			vec![
				(2,0),(2,2),(2,3),(3,3),(5,3),(6,0),(6,2),(6,3)
			]
		],
		// 王
		vec![
			vec![
				(2,2),(2,3),(2,4),(2,5),(2,6),(3,2),(3,6),(4,2),(4,6),(5,2),(5,6),(6,2),(6,3),(6,4),(6,5),(6,6)
			],
			vec![
				(0,2),(0,3),(0,4),(0,5),(0,6),(1,2),(1,6),(2,2),(2,6),(3,2),(3,6),(4,2),(4,3),(4,4),(4,5),(4,6)
			],
			vec![
				(4,2),(4,3),(4,4),(4,5),(4,6),(5,2),(5,6),(6,2),(6,6),(7,2),(7,6),(8,2),(8,3),(8,4),(8,5),(8,6)
			],
			vec![
				(2,0),(2,1),(2,2),(2,3),(2,4),(3,0),(3,4),(4,0),(4,4),(5,0),(5,4),(6,0),(6,1),(6,2),(6,3),(6,4)
			],
			vec![
				(2,0),(2,1),(2,2),(2,3),(3,3),(4,3),(5,3),(6,0),(6,1),(6,2),(6,3)
			]
		]
	];

	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ GFu ],
		vec![ GKyou ],
		vec![ GKei ],
		vec![ GGin ],
		vec![ GKin,GFuN,GKyouN,GKeiN,GGinN ],
		vec![ GKaku ],
		vec![ GHisha ],
		vec![ GKakuN ],
		vec![ GHishaN ],
		vec![ GOu ]
	];

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for ((k,p),mvs) in kinds.iter().zip(&POSITIONS).zip(mvs) {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

				let state = State::new(banmen);

				let dx = m.0;
				let dy = m.1;

				let mv = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-dx),(8-dy)+1,true)));

				assert!(!Rule::is_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					mv
				), format!("is_valid_move to occupied: kind = {:?}, move = {:?} is true.", k,mv.to_move()));

				let mv = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-dx),(8-dy)+1,false)));

				assert!(!Rule::is_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					mv
				), format!("is_valid_move to occupied: kind = {:?}, move = {:?} is true.", k,mv.to_move()));
			}
		}
	}
}
#[test]
fn test_is_valid_move_put_valid_sente() {
	let kinds:[MochigomaKind; 7] = [
		MochigomaKind::Fu,
		MochigomaKind::Kyou,
		MochigomaKind::Kei,
		MochigomaKind::Gin,
		MochigomaKind::Kin,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha
	];

	let deny_line:[u32; 7] = [
		1,
		1,
		2,
		0,
		0,
		0,
		0
	];

	let mut banmen = BANMEN_START_POS.clone();

	banmen.0[0][0] = Blank;
	banmen.0[1][0] = GKyou;
	banmen.0[0][1] = Blank;
	banmen.0[2][2] = GKei;

	let state = State::new(banmen.clone());

	for (kind,deny_line) in kinds.iter().zip(&deny_line) {
		let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

		ms.insert(*kind, 1);

		let mc = MochigomaCollections::Pair(ms,HashMap::new());

		for y in 0..9 {
			for x in 0..9 {
				if banmen.0[y][x] == Blank && *deny_line <= y as u32 {
					let m = rule::AppliedMove::from(Move::Put(*kind,KomaDstPutPosition(9 - x as u32,y as u32 + 1)));

					assert!(Rule::is_valid_move(&state,
						Teban::Sente,
						&mc,
						m
					), format!("is_valid_move to occupied: kind = {:?}, move = {:?} is false.", kind,m.to_move()));
				}
			}
		}
	}
}
#[test]
fn test_is_valid_move_put_valid_gote() {
	let kinds:[MochigomaKind; 7] = [
		MochigomaKind::Fu,
		MochigomaKind::Kyou,
		MochigomaKind::Kei,
		MochigomaKind::Gin,
		MochigomaKind::Kin,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha
	];

	let deny_line:[u32; 7] = [
		1,
		1,
		2,
		0,
		0,
		0,
		0
	];

	let mut banmen = BANMEN_START_POS.clone();

	banmen.0[8-0][8-0] = Blank;
	banmen.0[8-1][8-0] = SKyou;
	banmen.0[8-0][8-1] = Blank;
	banmen.0[8-2][8-2] = SKei;

	let state = State::new(banmen.clone());

	for (kind,deny_line) in kinds.iter().zip(&deny_line) {
		let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

		mg.insert(*kind, 1);

		let mc = MochigomaCollections::Pair(HashMap::new(),mg);

		for y in 0..9 {
			for x in 0..9 {
				let (x,y) = (8-x,8-y);

				if banmen.0[y][x] == Blank && (8 - *deny_line) >= y as u32 {
					let m = rule::AppliedMove::from(Move::Put(*kind,KomaDstPutPosition(9 - x as u32,y as u32 + 1)));

					assert!(Rule::is_valid_move(&state,
						Teban::Gote,
						&mc,
						m
					), format!("is_valid_move to occupied: kind = {:?}, move = {:?} is false.", kind,m.to_move()));
				}
			}
		}
	}
}
#[test]
fn test_is_valid_move_put_invalid_sente() {
	let kinds:[MochigomaKind; 7] = [
		MochigomaKind::Fu,
		MochigomaKind::Kyou,
		MochigomaKind::Kei,
		MochigomaKind::Gin,
		MochigomaKind::Kin,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha
	];

	let deny_line:[u32; 7] = [
		1,
		1,
		2,
		0,
		0,
		0,
		0
	];

	let mut banmen = BANMEN_START_POS.clone();

	banmen.0[0][0] = Blank;
	banmen.0[1][0] = GKyou;
	banmen.0[0][1] = Blank;
	banmen.0[2][2] = GKei;

	let state = State::new(banmen.clone());

	for (kind,deny_line) in kinds.iter().zip(&deny_line) {
		let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

		ms.insert(*kind, 1);

		let mc = MochigomaCollections::Pair(ms,HashMap::new());

		for y in 0..9 {
			for x in 0..9 {
				if banmen.0[y][x] != Blank || *deny_line > y as u32 {
					let m = rule::AppliedMove::from(Move::Put(*kind,KomaDstPutPosition(9 - x as u32,y as u32 + 1)));

					assert!(!Rule::is_valid_move(&state,
						Teban::Sente,
						&mc,
						m
					), format!("is_valid_move to occupied: kind = {:?}, move = {:?} is true.", kind,m.to_move()));
				}
			}
		}
	}
}
#[test]
fn test_is_valid_move_put_invalid_gote() {
	let kinds:[MochigomaKind; 7] = [
		MochigomaKind::Fu,
		MochigomaKind::Kyou,
		MochigomaKind::Kei,
		MochigomaKind::Gin,
		MochigomaKind::Kin,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha
	];

	let deny_line:[u32; 7] = [
		1,
		1,
		2,
		0,
		0,
		0,
		0
	];

	let mut banmen = BANMEN_START_POS.clone();

	banmen.0[8-0][8-0] = Blank;
	banmen.0[8-1][8-0] = SKyou;
	banmen.0[8-0][8-1] = Blank;
	banmen.0[8-2][8-2] = SKei;

	let state = State::new(banmen.clone());

	for (kind,deny_line) in kinds.iter().zip(&deny_line) {
		let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

		mg.insert(*kind, 1);

		let mc = MochigomaCollections::Pair(HashMap::new(),mg);

		for y in 0..9 {
			for x in 0..9 {
				let (x,y) = (8-x,8-y);

				if banmen.0[y][x] != Blank || (8 - *deny_line) < y as u32 {
					let m = rule::AppliedMove::from(Move::Put(*kind,KomaDstPutPosition(9 - x as u32,y as u32 + 1)));

					assert!(!Rule::is_valid_move(&state,
						Teban::Gote,
						&mc,
						m
					), format!("is_valid_move to occupied: kind = {:?}, move = {:?} is true.", kind,m.to_move()));
				}
			}
		}
	}
}
#[test]
fn test_apply_valid_move_valid_to_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 5] = [
		(4,4),(2,3),(6,3),(4,2),(4,1)
	];

	let mvs:Vec<Vec<Vec<(u32,u32,bool)>>> = vec![
		// 歩
		vec![
			vec![
				(4,3,true),(4,3,false)
			],
			vec![
				(2,2,true),(2,2,false)
			],
			vec![
				(6,2,true),(6,2,false)
			],
			vec![
				(4,1,true),(4,1,false)
			],
			vec![
				(4,1,true)
			]
		],
		// 香車
		vec![
			vec![
				(4,3,true),(4,3,false),
				(4,2,true),(4,2,false),
				(4,1,true),(4,1,false),
				(4,0,true),
			],
			vec![
				(2,2,true),(2,2,false),
				(2,1,true),(2,1,false),
				(2,0,true),
			],
			vec![
				(6,2,true),(6,2,false),
				(6,1,true),(6,1,false),
				(6,0,true),
			],
			vec![
				(4,1,true),(4,1,false),
				(4,0,true)
			],
			vec![
				(4,0,true)
			]
		],
		// 桂馬
		vec![
			vec![
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			],
			vec![
				(1,1,true),
				(3,1,true)
			],
			vec![
				(4,1,true),
				(8,1,true)
			],
			vec![
				(2,0,true),
				(6,0,true)
			],
			vec![]
		],
		// 銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,5,false),
				(5,5,false)
			],
			vec![
				(1,2,true),(1,2,false),
				(2,2,true),(2,2,false),
				(3,2,true),(3,2,false),
				(1,4,false),
				(3,4,false)
			],
			vec![
				(5,2,true),(5,2,false),
				(6,2,true),(6,2,false),
				(7,2,true),(7,2,false),
				(5,4,false),
				(7,4,false)
			],
			vec![
				(3,1,true),(3,1,false),
				(4,1,true),(4,1,false),
				(5,1,true),(5,1,false),
				(3,3,true),(3,3,false),
				(5,3,true),(5,3,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(4,0,true),(4,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			]
		],
		// 金,成歩,成香,成桂,成銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(4,5,false)
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(2,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(6,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(4,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(4,2,false)
			]
		],
		// 角
		vec![
			vec![
				(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false),
				(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false)
			],
			vec![
				(1,2,true),(1,2,false),(0,1,true),(0,1,false),
				(3,2,true),(3,2,false),(4,1,true),(4,1,false),(5,0,true),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false)
			],
			vec![
				(5,2,true),(5,2,false),(4,1,true),(4,1,false),(3,0,true),(3,0,false),
				(7,2,true),(7,2,false),(8,1,true),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false)
			],
			vec![
				(3,1,true),(3,1,false),(2,0,true),(2,0,false),
				(5,1,true),(5,1,false),(6,0,true),(6,0,false),
				(3,3,true),(3,3,false),(2,4,true),(2,4,false),(1,5,true),(1,5,false),(0,6,true),(0,6,false),
				(5,3,true),(5,3,false),(6,4,true),(6,4,false),(7,5,true),(7,5,false),(8,4,true),(8,4,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),(2,3,true),(2,3,false),(1,4,true),(1,4,false),(0,5,true),(0,5,false),
				(5,2,true),(5,2,false),(6,3,true),(6,3,false),(7,4,true),(7,4,false),(8,5,true),(8,5,false)
			]
		],
		// 飛車
		vec![
			vec![
				(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false)
			],
			vec![
				(2,2,true),(2,2,false),(2,1,true),(2,1,false),(2,0,true),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false)
			],
			vec![
				(6,2,true),(6,2,false),(6,1,true),(6,1,false),(6,0,true),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false)
			],
			vec![
				(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,2,true),(3,2,false),(2,2,true),(2,2,false),(1,2,true),(1,2,false),(0,2,true),(0,2,false),
				(5,2,true),(5,2,false),(6,2,true),(6,2,false),(7,2,true),(7,2,false),(8,2,true),(8,2,false),
			],
			vec![
				(4,0,true),(4,0,false),
				(4,2,true),(4,2,false),(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,1,true),(3,1,false),(2,1,true),(2,1,false),(1,1,true),(1,1,false),(0,1,true),(0,1,false),
				(5,1,true),(5,1,false),(6,1,true),(6,1,false),(7,1,true),(7,1,false),(8,1,true),(8,1,false),
			]
		],
		// 成角
		vec![
			vec![
				(3,3,false),(2,2,false),(1,1,false),(0,0,false),
				(5,3,false),(6,2,false),(7,1,false),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false),
				(4,3,false),(4,5,false),(3,4,false),(5,4,false)
			],
			vec![
				(1,2,false),(0,1,false),
				(3,2,false),(4,1,false),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false),
				(2,2,false),(2,4,false),(1,3,false),(3,3,false),
			],
			vec![
				(5,2,false),(4,1,false),(3,0,false),
				(7,2,false),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false),
				(6,2,false),(6,4,false),(5,3,false),(7,3,false),
			],
			vec![
				(3,1,false),(2,0,false),
				(5,1,false),(6,0,false),
				(3,3,false),(2,4,false),(1,5,false),(0,6,false),
				(5,3,false),(6,4,false),(7,5,false),(8,4,false),
				(4,1,false),(4,3,false),(3,2,false),(5,2,false)
			],
			vec![
				(3,0,false),
				(5,0,false),
				(3,2,false),(2,3,false),(1,4,false),(0,5,false),
				(5,2,false),(6,3,false),(7,4,false),(8,5,false),
				(4,0,false),(4,2,false),(3,1,false),(5,1,false)
			]
		],
		// 成飛
		vec![
			vec![
				(4,3,false),(4,2,false),(4,1,false),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false),
				(3,3,false),(5,3,false),(3,5,false),(5,5,false),
			],
			vec![
				(2,2,false),(2,1,false),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false),
				(1,2,false),(3,2,false),(1,4,false),(3,4,false)
			],
			vec![
				(6,2,false),(6,1,false),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false),
				(5,2,false),(7,2,false),(5,4,false),(7,4,false)
			],
			vec![
				(4,1,false),(4,0,false),
				(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,2,false),(2,2,false),(1,2,false),(0,2,false),
				(5,2,false),(6,2,false),(7,2,false),(8,2,false),
				(3,1,false),(5,1,false),(3,3,false),(5,3,false)
			],
			vec![
				(4,0,false),
				(4,2,false),(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,1,false),(2,1,false),(1,1,false),(0,1,false),
				(5,1,false),(6,1,false),(7,1,false),(8,1,false),
				(3,0,false),(5,0,false),(3,1,false),(5,1,false)
			]
		],
		// 王
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(3,5,false),
				(4,5,false),
				(5,5,false),
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(1,4,false),
				(2,4,false),
				(3,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(5,4,false),
				(6,4,false),
				(7,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(3,3,false),
				(4,3,false),
				(5,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(3,2,false),
				(4,2,false),
				(5,2,false)
			]
		]
	];

	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ SFu ],
		vec![ SKyou ],
		vec![ SKei ],
		vec![ SGin ],
		vec![ SKin,SFuN,SKyouN,SKeiN,SGinN ],
		vec![ SKaku ],
		vec![ SHisha ],
		vec![ SKakuN ],
		vec![ SHishaN ],
		vec![ SOu ]
	];

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for ((k,p),mvs) in kinds.iter().zip(&POSITIONS).zip(mvs) {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[p.1 as usize][p.0 as usize] = *k;

				let dx = m.0;
				let dy = m.1;

				let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-dx,dy+1,m.2)));

				let state = State::new(banmen);

				assert!(Rule::apply_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					m
				).is_ok(), format!("apply_valid_move returned Err: kind = {:?}, move = {:?}.", k,m.to_move()));

				let mut banmen = blank_banmen.clone();

				banmen.0[p.1 as usize][p.0 as usize] = *k;
				banmen.0[dy as usize][dx as usize] = GFu;

				let state = State::new(banmen);

				assert!(Rule::apply_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					m
				).is_ok(), format!("apply_valid_move to returned Err: kind = {:?}, move = {:?}.", k,m.to_move()));
			}
		}
	}
}
#[test]
fn test_apply_valid_move_to_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 5] = [
		(4,4),(2,3),(6,3),(4,2),(4,1)
	];

	let mvs:Vec<Vec<Vec<(u32,u32,bool)>>> = vec![
		// 歩
		vec![
			vec![
				(4,3,true),(4,3,false)
			],
			vec![
				(2,2,true),(2,2,false)
			],
			vec![
				(6,2,true),(6,2,false)
			],
			vec![
				(4,1,true),(4,1,false)
			],
			vec![
				(4,1,true)
			]
		],
		// 香車
		vec![
			vec![
				(4,3,true),(4,3,false),
				(4,2,true),(4,2,false),
				(4,1,true),(4,1,false),
				(4,0,true),
			],
			vec![
				(2,2,true),(2,2,false),
				(2,1,true),(2,1,false),
				(2,0,true),
			],
			vec![
				(6,2,true),(6,2,false),
				(6,1,true),(6,1,false),
				(6,0,true),
			],
			vec![
				(4,1,true),(4,1,false),
				(4,0,true)
			],
			vec![
				(4,0,true)
			]
		],
		// 桂馬
		vec![
			vec![
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			],
			vec![
				(1,1,true),
				(3,1,true)
			],
			vec![
				(4,1,true),
				(8,1,true)
			],
			vec![
				(2,0,true),
				(6,0,true)
			],
			vec![]
		],
		// 銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,5,false),
				(5,5,false)
			],
			vec![
				(1,2,true),(1,2,false),
				(2,2,true),(2,2,false),
				(3,2,true),(3,2,false),
				(1,4,false),
				(3,4,false)
			],
			vec![
				(5,2,true),(5,2,false),
				(6,2,true),(6,2,false),
				(7,2,true),(7,2,false),
				(5,4,false),
				(7,4,false)
			],
			vec![
				(3,1,true),(3,1,false),
				(4,1,true),(4,1,false),
				(5,1,true),(5,1,false),
				(3,3,true),(3,3,false),
				(5,3,true),(5,3,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(4,0,true),(4,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			]
		],
		// 金,成歩,成香,成桂,成銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(4,5,false)
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(2,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(6,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(4,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(4,2,false)
			]
		],
		// 角
		vec![
			vec![
				(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false),
				(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false)
			],
			vec![
				(1,2,true),(1,2,false),(0,1,true),(0,1,false),
				(3,2,true),(3,2,false),(4,1,true),(4,1,false),(5,0,true),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false)
			],
			vec![
				(5,2,true),(5,2,false),(4,1,true),(4,1,false),(3,0,true),(3,0,false),
				(7,2,true),(7,2,false),(8,1,true),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false)
			],
			vec![
				(3,1,true),(3,1,false),(2,0,true),(2,0,false),
				(5,1,true),(5,1,false),(6,0,true),(6,0,false),
				(3,3,true),(3,3,false),(2,4,true),(2,4,false),(1,5,true),(1,5,false),(0,6,true),(0,6,false),
				(5,3,true),(5,3,false),(6,4,true),(6,4,false),(7,5,true),(7,5,false),(8,4,true),(8,4,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),(2,3,true),(2,3,false),(1,4,true),(1,4,false),(0,5,true),(0,5,false),
				(5,2,true),(5,2,false),(6,3,true),(6,3,false),(7,4,true),(7,4,false),(8,5,true),(8,5,false)
			]
		],
		// 飛車
		vec![
			vec![
				(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false)
			],
			vec![
				(2,2,true),(2,2,false),(2,1,true),(2,1,false),(2,0,true),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false)
			],
			vec![
				(6,2,true),(6,2,false),(6,1,true),(6,1,false),(6,0,true),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false)
			],
			vec![
				(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,2,true),(3,2,false),(2,2,true),(2,2,false),(1,2,true),(1,2,false),(0,2,true),(0,2,false),
				(5,2,true),(5,2,false),(6,2,true),(6,2,false),(7,2,true),(7,2,false),(8,2,true),(8,2,false),
			],
			vec![
				(4,0,true),(4,0,false),
				(4,2,true),(4,2,false),(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,1,true),(3,1,false),(2,1,true),(2,1,false),(1,1,true),(1,1,false),(0,1,true),(0,1,false),
				(5,1,true),(5,1,false),(6,1,true),(6,1,false),(7,1,true),(7,1,false),(8,1,true),(8,1,false),
			]
		],
		// 成角
		vec![
			vec![
				(3,3,false),(2,2,false),(1,1,false),(0,0,false),
				(5,3,false),(6,2,false),(7,1,false),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false),
				(4,3,false),(4,5,false),(3,4,false),(5,4,false)
			],
			vec![
				(1,2,false),(0,1,false),
				(3,2,false),(4,1,false),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false),
				(2,2,false),(2,4,false),(1,3,false),(3,3,false),
			],
			vec![
				(5,2,false),(4,1,false),(3,0,false),
				(7,2,false),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false),
				(6,2,false),(6,4,false),(5,3,false),(7,3,false),
			],
			vec![
				(3,1,false),(2,0,false),
				(5,1,false),(6,0,false),
				(3,3,false),(2,4,false),(1,5,false),(0,6,false),
				(5,3,false),(6,4,false),(7,5,false),(8,4,false),
				(4,1,false),(4,3,false),(3,2,false),(5,2,false)
			],
			vec![
				(3,0,false),
				(5,0,false),
				(3,2,false),(2,3,false),(1,4,false),(0,5,false),
				(5,2,false),(6,3,false),(7,4,false),(8,5,false),
				(4,0,false),(4,2,false),(3,1,false),(5,1,false)
			]
		],
		// 成飛
		vec![
			vec![
				(4,3,false),(4,2,false),(4,1,false),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false),
				(3,3,false),(5,3,false),(3,5,false),(5,5,false),
			],
			vec![
				(2,2,false),(2,1,false),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false),
				(1,2,false),(3,2,false),(1,4,false),(3,4,false)
			],
			vec![
				(6,2,false),(6,1,false),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false),
				(5,2,false),(7,2,false),(5,4,false),(7,4,false)
			],
			vec![
				(4,1,false),(4,0,false),
				(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,2,false),(2,2,false),(1,2,false),(0,2,false),
				(5,2,false),(6,2,false),(7,2,false),(8,2,false),
				(3,1,false),(5,1,false),(3,3,false),(5,3,false)
			],
			vec![
				(4,0,false),
				(4,2,false),(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,1,false),(2,1,false),(1,1,false),(0,1,false),
				(5,1,false),(6,1,false),(7,1,false),(8,1,false),
				(3,0,false),(5,0,false),(3,1,false),(5,1,false)
			]
		],
		// 王
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(3,5,false),
				(4,5,false),
				(5,5,false),
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(1,4,false),
				(2,4,false),
				(3,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(5,4,false),
				(6,4,false),
				(7,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(3,3,false),
				(4,3,false),
				(5,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(3,2,false),
				(4,2,false),
				(5,2,false)
			]
		]
	];

	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ GFu ],
		vec![ GKyou ],
		vec![ GKei ],
		vec![ GGin ],
		vec![ GKin,GFuN,GKyouN,GKeiN,GGinN ],
		vec![ GKaku ],
		vec![ GHisha ],
		vec![ GKakuN ],
		vec![ GHishaN ],
		vec![ GOu ]
	];

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for ((k,p),mvs) in kinds.iter().zip(&POSITIONS).zip(mvs) {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

				let dx = m.0;
				let dy = m.1;

				let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-dx),(8-dy)+1,m.2)));

				let state = State::new(banmen);

				assert!(Rule::apply_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					m
				).is_ok(), format!("apply_valid_move returned Err: kind = {:?}, move = {:?}.", k,m.to_move()));

				let mut banmen = blank_banmen.clone();

				banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;
				banmen.0[8 - dy as usize][8 - dx as usize] = SFu;

				let state = State::new(banmen);

				assert!(Rule::apply_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					m
				).is_ok(), format!("apply_valid_move returned Err: kind = {:?}, move = {:?}.", k,m.to_move()));
			}
		}
	}
}
#[test]
fn test_apply_valid_move_with_kyou_opponent_occupied_sente() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,4,false),(4,3,false),(4,2,true),(4,2,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[5][4] = SKyou;
		banmen.0[2][4] = GFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-4,5+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

		let state = State::new(banmen);

		assert!(Rule::apply_valid_move(&state,
			Teban::Sente,
			&MochigomaCollections::Empty,
			m
		).is_ok(), format!("apply_valid_move returned Err: move = {:?}.", m.to_move()));
	}
}
#[test]
fn test_apply_valid_move_with_kyou_opponent_occupied_gote() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,4,false),(4,3,false),(4,2,true),(4,2,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-5][8-4] = GKyou;
		banmen.0[8-2][8-4] = SFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-4),(8-5)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

		let state = State::new(banmen);

		assert!(Rule::apply_valid_move(&state,
			Teban::Gote,
			&MochigomaCollections::Empty,
			m
		).is_ok(), format!("apply_valid_move returned Err move = {:?}.", m.to_move()));
	}
}
#[test]
fn test_apply_valid_move_valid_with_kyou_self_occupied_sente() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,4,false),(4,3,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[5][4] = SKyou;
		banmen.0[2][4] = SFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-4,5+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

		let state = State::new(banmen);

		assert!(Rule::apply_valid_move(&state,
			Teban::Sente,
			&MochigomaCollections::Empty,
			m
		).is_ok(), format!("apply_valid_move returned Err: move = {:?}.", m.to_move()));
	}
}
#[test]
fn test_apply_valid_move_valid_with_kyou_self_occupied_gote() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,4,false),(4,3,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-5][8-4] = GKyou;
		banmen.0[8-2][8-4] = GFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-4),(8-5)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

		let state = State::new(banmen);

		assert!(Rule::apply_valid_move(&state,
			Teban::Gote,
			&MochigomaCollections::Empty,
			m
		).is_ok(), format!("apply_valid_move returned Err: move = {:?}.", m.to_move()));
	}
}
#[test]
fn test_apply_valid_move_valid_with_kaku_opponent_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(2,2,true),(2,2,false),(3,3,true),(3,3,false),(4,4,true),(4,4,false)
		],
		vec![
			(2,6,false),(3,5,false),(4,4,false)
		],
		vec![
			(6,2,true),(6,2,false),(5,3,true),(5,3,false),(4,4,true),(4,4,false)
		],
		vec![
			(6,6,false),(5,5,false),(4,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SKaku;
			banmen.0[4][4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			).is_ok(), format!("apply_valid_move returned Err: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_valid_with_kaku_opponent_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(2,2,true),(2,2,false),(3,3,true),(3,3,false),(4,4,true),(4,4,false)
		],
		vec![
			(2,6,false),(3,5,false),(4,4,false)
		],
		vec![
			(6,2,true),(6,2,false),(5,3,true),(5,3,false),(4,4,true),(4,4,false)
		],
		vec![
			(6,6,false),(5,5,false),(4,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GKaku;
			banmen.0[4][4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			).is_ok(), format!("apply_valid_move returned Err: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_valid_with_kaku_self_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(2,2,true),(2,2,false),(3,3,true),(3,3,false)
		],
		vec![
			(2,6,false),(3,5,false)
		],
		vec![
			(6,2,true),(6,2,false),(5,3,true),(5,3,false)
		],
		vec![
			(6,6,false),(5,5,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SKaku;
			banmen.0[4][4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			).is_ok(), format!("apply_valid_move returned Err: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_valid_with_kaku_self_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(2,2,true),(2,2,false),(3,3,true),(3,3,false)
		],
		vec![
			(2,6,false),(3,5,false)
		],
		vec![
			(6,2,true),(6,2,false),(5,3,true),(5,3,false)
		],
		vec![
			(6,6,false),(5,5,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GKaku;
			banmen.0[4][4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			).is_ok(), format!("apply_valid_move returned Err: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_valid_with_hisha_opponent_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,6,false),(4,5,false),(4,4,false)
		],
		vec![
			(2,4,false),(3,4,false),(4,4,false)
		],
		vec![
			(4,2,true),(4,2,false),(4,3,true),(4,3,false),(4,4,true),(4,4,false)
		],
		vec![
			(6,4,false),(5,4,false),(4,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SHisha;
			banmen.0[4][4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			).is_ok(), format!("apply_valid_move returned Err: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_valid_with_hisha_opponent_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,6,false),(4,5,false),(4,4,false)
		],
		vec![
			(2,4,false),(3,4,false),(4,4,false)
		],
		vec![
			(4,2,true),(4,2,false),(4,3,true),(4,3,false),(4,4,true),(4,4,false)
		],
		vec![
			(6,4,false),(5,4,false),(4,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GHisha;
			banmen.0[8-4][8-4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			).is_ok(), format!("apply_valid_move returned Err: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_valid_with_hisha_self_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,6,false),(4,5,false)
		],
		vec![
			(2,4,false),(3,4,false)
		],
		vec![
			(4,2,true),(4,2,false),(4,3,true),(4,3,false)
		],
		vec![
			(6,4,false),(5,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SHisha;
			banmen.0[4][4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			).is_ok(), format!("apply_valid_move returned Err: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_valid_with_hisha_self_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,6,false),(4,5,false)
		],
		vec![
			(2,4,false),(3,4,false)
		],
		vec![
			(4,2,true),(4,2,false),(4,3,true),(4,3,false)
		],
		vec![
			(6,4,false),(5,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GHisha;
			banmen.0[8-4][8-4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			).is_ok(), format!("apply_valid_move returned Err: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_with_kyou_opponent_occupied_sente() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,1,true),(4,1,false),(4,0,true),(4,0,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[5][4] = SKyou;
		banmen.0[2][4] = GFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-4,5+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

		let state = State::new(banmen);

		assert!(Rule::apply_valid_move(&state,
			Teban::Sente,
			&MochigomaCollections::Empty,
			m
		).is_err(), format!("apply_valid_move returned Ok: move = {:?}.", m.to_move()));
	}
}
#[test]
fn test_apply_valid_move_invalid_with_kyou_opponent_occupied_gote() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,1,true),(4,1,false),(4,0,true),(4,0,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-5][8-4] = GKyou;
		banmen.0[8-2][8-4] = SFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-4),(8-5)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

		let state = State::new(banmen);

		assert!(Rule::apply_valid_move(&state,
			Teban::Gote,
			&MochigomaCollections::Empty,
			m
		).is_err(), format!("apply_valid_move returned Ok: move = {:?}.", m.to_move()));
	}
}
#[test]
fn test_apply_valid_move_invalid_with_kyou_self_occupied_sente() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[5][4] = SKyou;
		banmen.0[2][4] = SFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-4,5+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

		let state = State::new(banmen);

		assert!(Rule::apply_valid_move(&state,
			Teban::Sente,
			&MochigomaCollections::Empty,
			m
		).is_err(), format!("apply_valid_move returned Ok: move = {:?}.", m.to_move()));
	}
}
#[test]
fn test_apply_valid_move_invalid_with_kyou_self_occupied_gote() {
	let mvs:Vec<(u32,u32,bool)> = vec![
		(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for m in &mvs {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-5][8-4] = GKyou;
		banmen.0[8-2][8-4] = GFu;

		let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-4),(8-5)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

		let state = State::new(banmen);

		assert!(Rule::apply_valid_move(&state,
			Teban::Gote,
			&MochigomaCollections::Empty,
			m
		).is_err(), format!("apply_valid_move returned Ok: move = {:?}.", m.to_move()));
	}
}
#[test]
fn test_apply_valid_move_invalid_with_kaku_opponent_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(5,5,true),(5,5,false),(6,6,true),(6,6,false),(7,7,true),(7,7,false),(8,8,true),(8,8,false)
		],
		vec![
			(5,3,true),(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false)
		],
		vec![
			(3,5,true),(3,5,false),(2,6,true),(2,6,false),(1,7,true),(1,7,false),(0,8,true),(0,8,false)
		],
		vec![
			(3,3,true),(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SKaku;
			banmen.0[4][4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			).is_err(), format!("apply_valid_move returned Ok: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_with_kaku_opponent_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(5,5,true),(5,5,false),(6,6,true),(6,6,false),(7,7,true),(7,7,false),(8,8,true),(8,8,false)
		],
		vec![
			(5,3,true),(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false)
		],
		vec![
			(3,5,true),(3,5,false),(2,6,true),(2,6,false),(1,7,true),(1,7,false),(0,8,true),(0,8,false)
		],
		vec![
			(3,3,true),(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GKaku;
			banmen.0[4][4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			).is_err(), format!("apply_valid_move returned Ok: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_with_kaku_self_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,4,true),(4,4,false),(5,5,true),(5,5,false),(6,6,true),(6,6,false),(7,7,true),(7,7,false),(8,8,true),(8,8,false)
		],
		vec![
			(4,4,true),(4,4,false),(5,3,true),(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false)
		],
		vec![
			(4,4,true),(4,4,false),(3,5,true),(3,5,false),(2,6,true),(2,6,false),(1,7,true),(1,7,false),(0,8,true),(0,8,false)
		],
		vec![
			(4,4,true),(4,4,false),(3,3,true),(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SKaku;
			banmen.0[4][4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			).is_err(), format!("apply_valid_move returned Ok: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_with_kaku_self_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(1,1),(1,7),(7,1),(7,7)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,4,true),(4,4,false),(5,5,true),(5,5,false),(6,6,true),(6,6,false),(7,7,true),(7,7,false),(8,8,true),(8,8,false)
		],
		vec![
			(4,4,true),(4,4,false),(5,3,true),(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false)
		],
		vec![
			(4,4,true),(4,4,false),(3,5,true),(3,5,false),(2,6,true),(2,6,false),(1,7,true),(1,7,false),(0,8,true),(0,8,false)
		],
		vec![
			(4,4,true),(4,4,false),(3,3,true),(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GKaku;
			banmen.0[4][4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			).is_err(), format!("apply_valid_move returned Ok: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_with_hisha_opponent_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,3,true),(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false)
		],
		vec![
			(5,4,true),(5,4,false),(6,4,true),(6,4,false),(7,4,true),(7,4,false),(8,4,true),(8,4,false)
		],
		vec![
			(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false)
		],
		vec![
			(3,4,true),(3,4,false),(2,4,true),(2,4,false),(1,4,true),(1,4,false),(0,4,true),(0,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SHisha;
			banmen.0[4][4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			).is_err(), format!("apply_valid_move returned Ok: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_with_hisha_opponent_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,3,true),(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false)
		],
		vec![
			(5,4,true),(5,4,false),(6,4,true),(6,4,false),(7,4,true),(7,4,false),(8,4,true),(8,4,false)
		],
		vec![
			(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false)
		],
		vec![
			(3,4,true),(3,4,false),(2,4,true),(2,4,false),(1,4,true),(1,4,false),(0,4,true),(0,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GHisha;
			banmen.0[8-4][8-4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			).is_err(), format!("apply_valid_move returned Ok: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_with_hisha_self_occupied_sente() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,4,true),(4,4,false),(4,3,true),(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false)
		],
		vec![
			(4,4,true),(4,4,false),(5,4,true),(5,4,false),(6,4,true),(6,4,false),(7,4,true),(7,4,false),(8,4,true),(8,4,false)
		],
		vec![
			(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false)
		],
		vec![
			(4,4,true),(4,4,false),(3,4,true),(3,4,false),(2,4,true),(2,4,false),(1,4,true),(1,4,false),(0,4,true),(0,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[p.1 as usize][p.0 as usize] = SHisha;
			banmen.0[4][4] = SFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Sente,
				&MochigomaCollections::Empty,
				m
			).is_err(), format!("apply_valid_move returned Ok: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_with_hisha_self_occupied_gote() {
	const POSITIONS:[(u32,u32); 4] = [
		(4,7),(1,4),(4,1),(7,4)
	];

	let mvs:Vec<Vec<(u32,u32,bool)>> = vec![
		vec![
			(4,4,true),(4,4,false),(4,3,true),(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false)
		],
		vec![
			(4,4,true),(4,4,false),(5,4,true),(5,4,false),(6,4,true),(6,4,false),(7,4,true),(7,4,false),(8,4,true),(8,4,false)
		],
		vec![
			(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false)
		],
		vec![
			(4,4,true),(4,4,false),(3,4,true),(3,4,false),(2,4,true),(2,4,false),(1,4,true),(1,4,false),(0,4,true),(0,4,false)
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (p,mvs) in POSITIONS.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = GHisha;
			banmen.0[8-4][8-4] = GFu;

			let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,m.2)));

			let state = State::new(banmen);

			assert!(Rule::apply_valid_move(&state,
				Teban::Gote,
				&MochigomaCollections::Empty,
				m
			).is_err(), format!("apply_valid_move returned Ok: move = {:?}.", m.to_move()));
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_to_outside_sente() {
	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ SFu ],
		vec![ SKyou ],
		vec![ SKei ],
		vec![ SGin ],
		vec![ SKin,SFuN,SKyouN,SKeiN,SGinN ],
		vec![ SKaku ],
		vec![ SHisha ],
		vec![ SKakuN ],
		vec![ SHishaN ]
	];

	let mvs:Vec<Vec<((u32,u32),(u32,u32))>> = vec![
		vec![
			((8,0),(7,8))
		],
		vec![
			((8,1),(7,8))
		],
		vec![
			((7,1),(7,8))
		],
		vec![
			((0,0),(1,8)),((0,0),(2,8)),
			((0,8),(2,0)),
			((8,0),(6,8)),((8,0),(7,8)),((8,0),(8,8)),((8,0),(9,1)),
			((8,8),(9,7)),((8,8),(9,9)),((8,8),(8,0))
		],
		vec![
			((0,0),(1,8)),((0,0),(2,8)),
			((0,8),(1,0)),
			((8,0),(6,8)),((8,0),(7,8)),((8,0),(8,8)),((8,0),(9,0)),
			((8,8),(7,0)),((8,8),(9,7)),((8,8),(9,8))
		],
		vec![
			((0,0),(2,8)),
			((0,8),(2,0)),
			((8,0),(6,8)),((8,0),(8,8)),((8,0),(9,1)),
			((8,8),(6,0)),((8,8),(9,7)),((8,8),(9,9))
		],
		vec![
			((0,8),(1,0)),
			((8,0),(7,8)),((8,0),(9,0)),
			((8,8),(7,0)),((8,8),(8,9)),((8,8),(9,8))
		],
		vec![
			((0,0),(2,8)),
			((0,8),(1,0)),((0,8),(2,0)),
			((8,0),(7,8)),((8,0),(9,0)),((8,0),(6,8)),((8,0),(8,8)),((8,0),(9,1)),
			((8,8),(7,0)),((8,8),(8,9)),((8,8),(9,8)),((8,8),(6,0)),((8,8),(9,7)),((8,8),(9,9))
		],
		vec![
			((0,0),(2,8)),
			((0,8),(1,0)),((0,8),(2,0)),
			((8,0),(7,8)),((8,0),(9,0)),((8,0),(6,8)),((8,0),(9,1)),
			((8,8),(7,0)),((8,8),(8,9)),((8,8),(9,8)),((8,8),(6,0)),((8,8),(9,7)),((8,8),(9,9))
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for k in kinds {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[(m.0).1 as usize][(m.0).0 as usize] = *k;

				let mv = rule::LegalMove::To(rule::LegalMoveTo::new(
												((m.0).0 * 9 + (m.0).1) as u32,
												((m.1).0 * 9 + (m.1).1) as u32,true,None)).to_applied_move();

				let state = State::new(banmen);

				assert!(Rule::apply_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					mv
				).is_err(), format!("apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k, mv.to_move()));

				let mv = rule::LegalMove::To(rule::LegalMoveTo::new(
												((m.0).0 * 9 + (m.0).1) as u32,
												((m.1).0 * 9 + (m.1).1) as u32,false,None)).to_applied_move();

				assert!(Rule::apply_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					mv
				).is_err(), format!("apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k, mv.to_move()));
			}
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_to_outside_gote() {
	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ GFu ],
		vec![ GKyou ],
		vec![ GKei ],
		vec![ GGin ],
		vec![ GKin,GFuN,GKyouN,GKeiN,GGinN ],
		vec![ GKaku ],
		vec![ GHisha ],
		vec![ GKakuN ],
		vec![ GHishaN ]
	];

	let mvs:Vec<Vec<((u32,u32),(u32,u32))>> = vec![
		vec![
			((7,8),(8,0)),((8,8),(9,0))
		],
		vec![
			((7,7),(8,0)),((8,7),(9,0))
		],
		vec![
			((8,7),(6,0)),((8,7),(9,9))
		],
		vec![
			((0,0),(0,8)),
			((0,8),(1,0)),((0,8),(2,0)),
			((8,0),(6,8)),((8,0),(8,8)),((8,0),(9,1)),
			((8,8),(8,0)),((8,8),(9,0)),((8,8),(9,9)),((8,8),(9,7))
		],
		vec![
			((0,8),(1,0)),((0,8),(2,0)),
			((8,0),(7,8)),((8,0),(9,0)),((8,0),(9,1)),
			((8,8),(8,0)),((8,8),(9,0)),((8,8),(9,8)),((8,8),(9,9))
		],
		vec![
			((0,0),(2,8)),
			((0,8),(2,0)),
			((8,0),(6,8)),((8,0),(8,8)),((8,0),(9,1)),
			((8,8),(6,0)),((8,8),(9,7)),((8,8),(9,9))
		],
		vec![
			((0,8),(1,0)),
			((8,0),(7,8)),((8,0),(9,0)),
			((8,8),(7,0)),((8,8),(8,9)),((8,8),(9,8))
		],
		vec![
			((0,0),(2,8)),
			((0,8),(1,0)),((0,8),(2,0)),
			((8,0),(7,8)),((8,0),(9,0)),((8,0),(6,8)),((8,0),(8,8)),((8,0),(9,1)),
			((8,8),(7,0)),((8,8),(8,9)),((8,8),(9,8)),((8,8),(6,0)),((8,8),(9,7)),((8,8),(9,9))
		],
		vec![
			((0,0),(2,8)),
			((0,8),(1,0)),((0,8),(2,0)),
			((8,0),(7,8)),((8,0),(9,0)),((8,0),(6,8)),((8,0),(9,1)),
			((8,8),(7,0)),((8,8),(8,9)),((8,8),(9,8)),((8,8),(6,0)),((8,8),(9,7)),((8,8),(9,9))
		]
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for k in kinds {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[8 - (m.0).1 as usize][8 - (m.0).0 as usize] = *k;

				let mv = rule::LegalMove::To(rule::LegalMoveTo::new(
												((m.0).0 * 9 + (m.0).1) as u32,
												((m.1).0 * 9 + (m.1).1) as u32,true,None)).to_applied_move();

				let state = State::new(banmen);

				assert!(Rule::apply_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					mv
				).is_err(), format!("apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k, mv.to_move()));

				let mv = rule::LegalMove::To(rule::LegalMoveTo::new(
												((m.0).0 * 9 + (m.0).1) as u32,
												((m.1).0 * 9 + (m.1).1) as u32,false,None)).to_applied_move();

				assert!(Rule::apply_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					mv
				).is_err(), format!("apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k, mv.to_move()));
			}
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_to_self_occupied_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 5] = [
		(4,4),(2,3),(6,3),(4,2),(4,1)
	];

	let mvs:Vec<Vec<Vec<(u32,u32,bool)>>> = vec![
		// 歩
		vec![
			vec![
				(4,3,true),(4,3,false)
			],
			vec![
				(2,2,true),(2,2,false)
			],
			vec![
				(6,2,true),(6,2,false)
			],
			vec![
				(4,1,true),(4,1,false)
			],
			vec![
				(4,1,true)
			]
		],
		// 香車
		vec![
			vec![
				(4,3,true),(4,3,false),
				(4,2,true),(4,2,false),
				(4,1,true),(4,1,false),
				(4,0,true),
			],
			vec![
				(2,2,true),(2,2,false),
				(2,1,true),(2,1,false),
				(2,0,true),
			],
			vec![
				(6,2,true),(6,2,false),
				(6,1,true),(6,1,false),
				(6,0,true),
			],
			vec![
				(4,1,true),(4,1,false),
				(4,0,true)
			],
			vec![
				(4,0,true)
			]
		],
		// 桂馬
		vec![
			vec![
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			],
			vec![
				(1,1,true),
				(3,1,true)
			],
			vec![
				(4,1,true),
				(8,1,true)
			],
			vec![
				(2,0,true),
				(6,0,true)
			],
			vec![]
		],
		// 銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,5,false),
				(5,5,false)
			],
			vec![
				(1,2,true),(1,2,false),
				(2,2,true),(2,2,false),
				(3,2,true),(3,2,false),
				(1,4,false),
				(3,4,false)
			],
			vec![
				(5,2,true),(5,2,false),
				(6,2,true),(6,2,false),
				(7,2,true),(7,2,false),
				(5,4,false),
				(7,4,false)
			],
			vec![
				(3,1,true),(3,1,false),
				(4,1,true),(4,1,false),
				(5,1,true),(5,1,false),
				(3,3,true),(3,3,false),
				(5,3,true),(5,3,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(4,0,true),(4,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			]
		],
		// 金,成歩,成香,成桂,成銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(4,5,false)
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(2,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(6,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(4,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(4,2,false)
			]
		],
		// 角
		vec![
			vec![
				(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false),
				(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false)
			],
			vec![
				(1,2,true),(1,2,false),(0,1,true),(0,1,false),
				(3,2,true),(3,2,false),(4,1,true),(4,1,false),(5,0,true),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false)
			],
			vec![
				(5,2,true),(5,2,false),(4,1,true),(4,1,false),(3,0,true),(3,0,false),
				(7,2,true),(7,2,false),(8,1,true),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false)
			],
			vec![
				(3,1,true),(3,1,false),(2,0,true),(2,0,false),
				(5,1,true),(5,1,false),(6,0,true),(6,0,false),
				(3,3,true),(3,3,false),(2,4,true),(2,4,false),(1,5,true),(1,5,false),(0,6,true),(0,6,false),
				(5,3,true),(5,3,false),(6,4,true),(6,4,false),(7,5,true),(7,5,false),(8,4,true),(8,4,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),(2,3,true),(2,3,false),(1,4,true),(1,4,false),(0,5,true),(0,5,false),
				(5,2,true),(5,2,false),(6,3,true),(6,3,false),(7,4,true),(7,4,false),(8,5,true),(8,5,false)
			]
		],
		// 飛車
		vec![
			vec![
				(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false)
			],
			vec![
				(2,2,true),(2,2,false),(2,1,true),(2,1,false),(2,0,true),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false)
			],
			vec![
				(6,2,true),(6,2,false),(6,1,true),(6,1,false),(6,0,true),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false)
			],
			vec![
				(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,2,true),(3,2,false),(2,2,true),(2,2,false),(1,2,true),(1,2,false),(0,2,true),(0,2,false),
				(5,2,true),(5,2,false),(6,2,true),(6,2,false),(7,2,true),(7,2,false),(8,2,true),(8,2,false),
			],
			vec![
				(4,0,true),(4,0,false),
				(4,2,true),(4,2,false),(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,1,true),(3,1,false),(2,1,true),(2,1,false),(1,1,true),(1,1,false),(0,1,true),(0,1,false),
				(5,1,true),(5,1,false),(6,1,true),(6,1,false),(7,1,true),(7,1,false),(8,1,true),(8,1,false),
			]
		],
		// 成角
		vec![
			vec![
				(3,3,false),(2,2,false),(1,1,false),(0,0,false),
				(5,3,false),(6,2,false),(7,1,false),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false),
				(4,3,false),(4,5,false),(3,4,false),(5,4,false)
			],
			vec![
				(1,2,false),(0,1,false),
				(3,2,false),(4,1,false),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false),
				(2,2,false),(2,4,false),(1,3,false),(3,3,false),
			],
			vec![
				(5,2,false),(4,1,false),(3,0,false),
				(7,2,false),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false),
				(6,2,false),(6,4,false),(5,3,false),(7,3,false),
			],
			vec![
				(3,1,false),(2,0,false),
				(5,1,false),(6,0,false),
				(3,3,false),(2,4,false),(1,5,false),(0,6,false),
				(5,3,false),(6,4,false),(7,5,false),(8,4,false),
				(4,1,false),(4,3,false),(3,2,false),(5,2,false)
			],
			vec![
				(3,0,false),
				(5,0,false),
				(3,2,false),(2,3,false),(1,4,false),(0,5,false),
				(5,2,false),(6,3,false),(7,4,false),(8,5,false),
				(4,0,false),(4,2,false),(3,1,false),(5,1,false)
			]
		],
		// 成飛
		vec![
			vec![
				(4,3,false),(4,2,false),(4,1,false),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false),
				(3,3,false),(5,3,false),(3,5,false),(5,5,false),
			],
			vec![
				(2,2,false),(2,1,false),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false),
				(1,2,false),(3,2,false),(1,4,false),(3,4,false)
			],
			vec![
				(6,2,false),(6,1,false),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false),
				(5,2,false),(7,2,false),(5,4,false),(7,4,false)
			],
			vec![
				(4,1,false),(4,0,false),
				(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,2,false),(2,2,false),(1,2,false),(0,2,false),
				(5,2,false),(6,2,false),(7,2,false),(8,2,false),
				(3,1,false),(5,1,false),(3,3,false),(5,3,false)
			],
			vec![
				(4,0,false),
				(4,2,false),(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,1,false),(2,1,false),(1,1,false),(0,1,false),
				(5,1,false),(6,1,false),(7,1,false),(8,1,false),
				(3,0,false),(5,0,false),(3,1,false),(5,1,false)
			]
		],
		// 王
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(3,5,false),
				(4,5,false),
				(5,5,false),
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(1,4,false),
				(2,4,false),
				(3,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(5,4,false),
				(6,4,false),
				(7,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(3,3,false),
				(4,3,false),
				(5,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(3,2,false),
				(4,2,false),
				(5,2,false)
			]
		]
	];

	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ SFu ],
		vec![ SKyou ],
		vec![ SKei ],
		vec![ SGin ],
		vec![ SKin,SFuN,SKyouN,SKeiN,SGinN ],
		vec![ SKaku ],
		vec![ SHisha ],
		vec![ SKakuN ],
		vec![ SHishaN ],
		vec![ SOu ]
	];

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for ((k,p),mvs) in kinds.iter().zip(&POSITIONS).zip(mvs) {
			for m in mvs {
				let dx = m.0;
				let dy = m.1;
				let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-dx,dy+1,m.2)));
				let mut banmen = blank_banmen.clone();

				banmen.0[p.1 as usize][p.0 as usize] = *k;
				banmen.0[dy as usize][dx as usize] = SFu;

				let state = State::new(banmen);

				assert!(Rule::apply_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					m
				).is_err(), format!("apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k,m.to_move()));
			}
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_to_self_occupied_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 5] = [
		(4,4),(2,3),(6,3),(4,2),(4,1)
	];

	let mvs:Vec<Vec<Vec<(u32,u32,bool)>>> = vec![
		// 歩
		vec![
			vec![
				(4,3,true),(4,3,false)
			],
			vec![
				(2,2,true),(2,2,false)
			],
			vec![
				(6,2,true),(6,2,false)
			],
			vec![
				(4,1,true),(4,1,false)
			],
			vec![
				(4,1,true)
			]
		],
		// 香車
		vec![
			vec![
				(4,3,true),(4,3,false),
				(4,2,true),(4,2,false),
				(4,1,true),(4,1,false),
				(4,0,true),
			],
			vec![
				(2,2,true),(2,2,false),
				(2,1,true),(2,1,false),
				(2,0,true),
			],
			vec![
				(6,2,true),(6,2,false),
				(6,1,true),(6,1,false),
				(6,0,true),
			],
			vec![
				(4,1,true),(4,1,false),
				(4,0,true)
			],
			vec![
				(4,0,true)
			]
		],
		// 桂馬
		vec![
			vec![
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			],
			vec![
				(1,1,true),
				(3,1,true)
			],
			vec![
				(4,1,true),
				(8,1,true)
			],
			vec![
				(2,0,true),
				(6,0,true)
			],
			vec![]
		],
		// 銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,5,false),
				(5,5,false)
			],
			vec![
				(1,2,true),(1,2,false),
				(2,2,true),(2,2,false),
				(3,2,true),(3,2,false),
				(1,4,false),
				(3,4,false)
			],
			vec![
				(5,2,true),(5,2,false),
				(6,2,true),(6,2,false),
				(7,2,true),(7,2,false),
				(5,4,false),
				(7,4,false)
			],
			vec![
				(3,1,true),(3,1,false),
				(4,1,true),(4,1,false),
				(5,1,true),(5,1,false),
				(3,3,true),(3,3,false),
				(5,3,true),(5,3,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(4,0,true),(4,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),
				(5,2,true),(5,2,false)
			]
		],
		// 金,成歩,成香,成桂,成銀
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(4,5,false)
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(2,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(6,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(4,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(4,2,false)
			]
		],
		// 角
		vec![
			vec![
				(3,3,false),(2,2,true),(2,2,false),(1,1,true),(1,1,false),(0,0,true),(0,0,false),
				(5,3,false),(6,2,true),(6,2,false),(7,1,true),(7,1,false),(8,0,true),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false)
			],
			vec![
				(1,2,true),(1,2,false),(0,1,true),(0,1,false),
				(3,2,true),(3,2,false),(4,1,true),(4,1,false),(5,0,true),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false)
			],
			vec![
				(5,2,true),(5,2,false),(4,1,true),(4,1,false),(3,0,true),(3,0,false),
				(7,2,true),(7,2,false),(8,1,true),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false)
			],
			vec![
				(3,1,true),(3,1,false),(2,0,true),(2,0,false),
				(5,1,true),(5,1,false),(6,0,true),(6,0,false),
				(3,3,true),(3,3,false),(2,4,true),(2,4,false),(1,5,true),(1,5,false),(0,6,true),(0,6,false),
				(5,3,true),(5,3,false),(6,4,true),(6,4,false),(7,5,true),(7,5,false),(8,4,true),(8,4,false)
			],
			vec![
				(3,0,true),(3,0,false),
				(5,0,true),(5,0,false),
				(3,2,true),(3,2,false),(2,3,true),(2,3,false),(1,4,true),(1,4,false),(0,5,true),(0,5,false),
				(5,2,true),(5,2,false),(6,3,true),(6,3,false),(7,4,true),(7,4,false),(8,5,true),(8,5,false)
			]
		],
		// 飛車
		vec![
			vec![
				(4,3,false),(4,2,true),(4,2,false),(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false)
			],
			vec![
				(2,2,true),(2,2,false),(2,1,true),(2,1,false),(2,0,true),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false)
			],
			vec![
				(6,2,true),(6,2,false),(6,1,true),(6,1,false),(6,0,true),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false)
			],
			vec![
				(4,1,true),(4,1,false),(4,0,true),(4,0,false),
				(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,2,true),(3,2,false),(2,2,true),(2,2,false),(1,2,true),(1,2,false),(0,2,true),(0,2,false),
				(5,2,true),(5,2,false),(6,2,true),(6,2,false),(7,2,true),(7,2,false),(8,2,true),(8,2,false),
			],
			vec![
				(4,0,true),(4,0,false),
				(4,2,true),(4,2,false),(4,3,true),(4,3,false),(4,4,true),(4,4,false),(4,5,true),(4,5,false),(4,6,true),(4,6,false),(4,7,true),(4,7,false),(4,8,true),(4,8,false),
				(3,1,true),(3,1,false),(2,1,true),(2,1,false),(1,1,true),(1,1,false),(0,1,true),(0,1,false),
				(5,1,true),(5,1,false),(6,1,true),(6,1,false),(7,1,true),(7,1,false),(8,1,true),(8,1,false),
			]
		],
		// 成角
		vec![
			vec![
				(3,3,false),(2,2,false),(1,1,false),(0,0,false),
				(5,3,false),(6,2,false),(7,1,false),(8,0,false),
				(3,5,false),(2,6,false),(1,7,false),(0,8,false),
				(5,5,false),(6,6,false),(7,7,false),(8,8,false),
				(4,3,false),(4,5,false),(3,4,false),(5,4,false)
			],
			vec![
				(1,2,false),(0,1,false),
				(3,2,false),(4,1,false),(5,0,false),
				(1,4,false),(0,5,false),
				(3,4,false),(4,5,false),(5,6,false),(6,7,false),(7,8,false),
				(2,2,false),(2,4,false),(1,3,false),(3,3,false),
			],
			vec![
				(5,2,false),(4,1,false),(3,0,false),
				(7,2,false),(8,1,false),
				(5,4,false),(4,5,false),(3,6,false),(2,7,false),(1,8,false),
				(7,4,false),(8,4,false),
				(6,2,false),(6,4,false),(5,3,false),(7,3,false),
			],
			vec![
				(3,1,false),(2,0,false),
				(5,1,false),(6,0,false),
				(3,3,false),(2,4,false),(1,5,false),(0,6,false),
				(5,3,false),(6,4,false),(7,5,false),(8,4,false),
				(4,1,false),(4,3,false),(3,2,false),(5,2,false)
			],
			vec![
				(3,0,false),
				(5,0,false),
				(3,2,false),(2,3,false),(1,4,false),(0,5,false),
				(5,2,false),(6,3,false),(7,4,false),(8,5,false),
				(4,0,false),(4,2,false),(3,1,false),(5,1,false)
			]
		],
		// 成飛
		vec![
			vec![
				(4,3,false),(4,2,false),(4,1,false),(4,0,false),
				(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(5,4,false),(6,4,false),(7,4,false),(8,4,false),
				(3,3,false),(5,3,false),(3,5,false),(5,5,false),
			],
			vec![
				(2,2,false),(2,1,false),(2,0,false),
				(2,4,false),(2,5,false),(2,6,false),(2,7,false),(2,8,false),
				(1,4,false),(0,4,false),
				(3,3,false),(4,3,false),(5,3,false),(6,3,false),(7,3,false),(8,3,false),
				(1,2,false),(3,2,false),(1,4,false),(3,4,false)
			],
			vec![
				(6,2,false),(6,1,false),(6,0,false),
				(6,4,false),(6,5,false),(6,6,false),(6,7,false),(6,8,false),
				(5,4,false),(4,4,false),(3,4,false),(2,4,false),(1,4,false),(0,4,false),
				(7,3,false),(8,3,false),
				(5,2,false),(7,2,false),(5,4,false),(7,4,false)
			],
			vec![
				(4,1,false),(4,0,false),
				(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,2,false),(2,2,false),(1,2,false),(0,2,false),
				(5,2,false),(6,2,false),(7,2,false),(8,2,false),
				(3,1,false),(5,1,false),(3,3,false),(5,3,false)
			],
			vec![
				(4,0,false),
				(4,2,false),(4,3,false),(4,4,false),(4,5,false),(4,6,false),(4,7,false),(4,8,false),
				(3,1,false),(2,1,false),(1,1,false),(0,1,false),
				(5,1,false),(6,1,false),(7,1,false),(8,1,false),
				(3,0,false),(5,0,false),(3,1,false),(5,1,false)
			]
		],
		// 王
		vec![
			vec![
				(3,3,false),
				(4,3,false),
				(5,3,false),
				(3,4,false),
				(5,4,false),
				(3,5,false),
				(4,5,false),
				(5,5,false),
			],
			vec![
				(1,2,false),
				(2,2,false),
				(3,2,false),
				(1,3,false),
				(3,3,false),
				(1,4,false),
				(2,4,false),
				(3,4,false)
			],
			vec![
				(5,2,false),
				(6,2,false),
				(7,2,false),
				(5,3,false),
				(7,3,false),
				(5,4,false),
				(6,4,false),
				(7,4,false)
			],
			vec![
				(3,1,false),
				(4,1,false),
				(5,1,false),
				(3,2,false),
				(5,2,false),
				(3,3,false),
				(4,3,false),
				(5,3,false)
			],
			vec![
				(3,0,false),
				(4,0,false),
				(5,0,false),
				(3,1,false),
				(5,1,false),
				(3,2,false),
				(4,2,false),
				(5,2,false)
			]
		]
	];

	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ GFu ],
		vec![ GKyou ],
		vec![ GKei ],
		vec![ GGin ],
		vec![ GKin,GFuN,GKyouN,GKeiN,GGinN ],
		vec![ GKaku ],
		vec![ GHisha ],
		vec![ GKakuN ],
		vec![ GHishaN ],
		vec![ GOu ]
	];

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for ((k,p),mvs) in kinds.iter().zip(&POSITIONS).zip(mvs) {
			for m in mvs {
				let dx = m.0;
				let dy = m.1;
				let m = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-dx),(8-dy)+1,m.2)));
				let mut banmen = blank_banmen.clone();

				banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;
				banmen.0[8 - dy as usize][8 - dx as usize] = GFu;

				let state = State::new(banmen);

				assert!(Rule::apply_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					m
				).is_err(), format!("apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k,m.to_move()));
			}
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_to_invalid_direction_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 5] = [
		(4,4),(2,3),(6,3),(4,2),(4,1)
	];

	let mvs:Vec<Vec<Vec<(u32,u32)>>> = vec![
		// 歩
		vec![
			vec![
				(3,3),(3,4),(3,5),(4,5),(5,3),(5,4),(5,5)
			],
			vec![
				(1,2),(1,3),(1,4),(2,4),(3,2),(3,3),(3,4)
			],
			vec![
				(5,2),(5,3),(5,4),(6,4),(7,2),(7,3),(7,4)
			],
			vec![
				(2,1),(2,2),(3,3),(4,3),(5,1),(5,2),(5,3)
			],
			vec![
				(2,0),(2,1),(3,2),(4,2),(5,0),(5,1),(5,2)
			]
		],
		// 香車
		vec![
			vec![
				(4,5),(4,6),(4,7),(4,8)
			],
			vec![
				(2,5),(2,6),(2,7),(2,8)
			],
			vec![
				(6,5),(6,6),(6,7),(6,8)
			],
			vec![
				(4,3),(4,4),(4,5),(4,6),(4,7),(4,8)
			],
			vec![
				(4,2),(4,3),(4,4),(4,5),(4,6),(4,7),(4,8)
			]
		],
		// 桂馬
		vec![
			vec![
				(3,3),(3,4),(3,5),(4,3),(4,5),(5,3),(5,4),(5,5)
			],
			vec![
				(1,3),(1,4),(1,5),(2,3),(2,5),(3,3),(3,4),(3,5)
			],
			vec![
				(5,3),(5,4),(5,5),(6,3),(6,5),(7,3),(7,4),(7,5)
			],
			vec![
				(3,1),(3,2),(3,3),(4,1),(4,3),(5,1),(5,2),(5,3)
			],
			vec![
				(3,0),(3,1),(3,2),(4,0),(4,2),(5,0),(5,1),(5,2)
			]
		],
		// 銀
		vec![
			vec![
				(3,4),(4,5),(5,4)
			],
			vec![
				(1,4),(2,5),(3,4)
			],
			vec![
				(5,4),(6,5),(7,4)
			],
			vec![
				(3,2),(4,3),(5,2)
			],
			vec![
				(3,1),(4,2),(5,1)
			]
		],
		// 金,成歩,成香,成桂,成銀
		vec![
			vec![
				(3,5),(5,5)
			],
			vec![
				(1,5),(3,5)
			],
			vec![
				(5,5),(7,5)
			],
			vec![
				(3,3),(5,3)
			],
			vec![
				(3,2),(5,2)
			]
		],
		// 角
		vec![
			vec![
				(3,4),(4,3),(4,5),(5,4)
			],
			vec![
				(1,4),(2,3),(2,5),(3,4)
			],
			vec![
				(5,4),(6,3),(6,5),(7,4)
			],
			vec![
				(3,2),(4,1),(4,3),(5,2)
			],
			vec![
				(3,1),(4,0),(4,2),(5,1)
			]
		],
		// 飛車
		vec![
			vec![
				(3,3),(3,5),(5,3),(5,5)
			],
			vec![
				(1,3),(1,5),(3,3),(3,5)
			],
			vec![
				(5,3),(5,5),(7,3),(7,5)
			],
			vec![
				(3,1),(3,3),(5,1),(5,3)
			],
			vec![
				(3,0),(3,2),(5,0),(5,2)
			]
		],
		// 成角
		vec![
			vec![
				(2,3),(2,4),(2,5),(3,2),(3,6),(4,2),(4,6),(5,2),(5,6),(6,3),(6,4),(6,5)
			],
			vec![
				(0,3),(0,4),(0,5),(1,2),(1,6),(2,2),(2,6),(3,2),(3,6),(4,3),(4,4),(4,5)
			],
			vec![
				(4,3),(4,4),(4,5),(5,2),(5,6),(6,2),(6,6),(7,2),(7,6),(8,3),(8,4),(8,5)
			],
			vec![
				(2,1),(2,2),(2,3),(3,0),(3,4),(4,0),(4,4),(5,0),(5,4),(6,1),(6,2),(6,3)
			],
			vec![
				(2,0),(2,1),(2,2),(3,3),(4,3),(5,3),(6,0),(6,1),(6,2)
			]
		],
		// 成飛
		vec![
			vec![
				(2,2),(2,3),(2,5),(2,6),(3,2),(3,6),(5,2),(5,6),(6,2),(6,3),(6,5),(6,6)
			],
			vec![
				(0,2),(0,4),(0,5),(0,6),(1,2),(1,6),(2,6),(3,2),(3,6),(4,2),(4,4),(4,5),(4,6)
			],
			vec![
				(4,2),(4,4),(4,5),(4,6),(5,6),(6,2),(7,2),(7,6),(8,2),(8,4),(8,5),(8,6)
			],
			vec![
				(2,0),(2,1),(2,3),(2,4),(3,0),(3,4),(5,0),(5,4),(6,0),(6,1),(6,3),(6,4)
			],
			vec![
				(2,0),(2,2),(2,3),(3,3),(5,3),(6,0),(6,2),(6,3)
			]
		],
		// 王
		vec![
			vec![
				(2,2),(2,3),(2,4),(2,5),(2,6),(3,2),(3,6),(4,2),(4,6),(5,2),(5,6),(6,2),(6,3),(6,4),(6,5),(6,6)
			],
			vec![
				(0,2),(0,3),(0,4),(0,5),(0,6),(1,2),(1,6),(2,2),(2,6),(3,2),(3,6),(4,2),(4,3),(4,4),(4,5),(4,6)
			],
			vec![
				(4,2),(4,3),(4,4),(4,5),(4,6),(5,2),(5,6),(6,2),(6,6),(7,2),(7,6),(8,2),(8,3),(8,4),(8,5),(8,6)
			],
			vec![
				(2,0),(2,1),(2,2),(2,3),(2,4),(3,0),(3,4),(4,0),(4,4),(5,0),(5,4),(6,0),(6,1),(6,2),(6,3),(6,4)
			],
			vec![
				(2,0),(2,1),(2,2),(2,3),(3,3),(4,3),(5,3),(6,0),(6,1),(6,2),(6,3)
			]
		]
	];

	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ SFu ],
		vec![ SKyou ],
		vec![ SKei ],
		vec![ SGin ],
		vec![ SKin,SFuN,SKyouN,SKeiN,SGinN ],
		vec![ SKaku ],
		vec![ SHisha ],
		vec![ SKakuN ],
		vec![ SHishaN ],
		vec![ SOu ]
	];

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for ((k,p),mvs) in kinds.iter().zip(&POSITIONS).zip(mvs) {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[p.1 as usize][p.0 as usize] = *k;

				let state = State::new(banmen);

				let dx = m.0;
				let dy = m.1;

				let mv = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-dx,dy+1,true)));

				assert!(Rule::apply_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					mv
				).is_err(), format!("apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k,mv.to_move()));

				let mv = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-dx,dy+1,false)));

				assert!(Rule::apply_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					mv
				).is_err(), format!("apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k,mv.to_move()));
			}
		}
	}
}
#[test]
fn test_apply_valid_move_invalid_to_invalid_direction_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 5] = [
		(4,4),(2,3),(6,3),(4,2),(4,1)
	];

	let mvs:Vec<Vec<Vec<(u32,u32)>>> = vec![
		// 歩
		vec![
			vec![
				(3,3),(3,4),(3,5),(4,5),(5,3),(5,4),(5,5)
			],
			vec![
				(1,2),(1,3),(1,4),(2,4),(3,2),(3,3),(3,4)
			],
			vec![
				(5,2),(5,3),(5,4),(6,4),(7,2),(7,3),(7,4)
			],
			vec![
				(2,1),(2,2),(3,3),(4,3),(5,1),(5,2),(5,3)
			],
			vec![
				(2,0),(2,1),(3,2),(4,2),(5,0),(5,1),(5,2)
			]
		],
		// 香車
		vec![
			vec![
				(4,5),(4,6),(4,7),(4,8)
			],
			vec![
				(2,5),(2,6),(2,7),(2,8)
			],
			vec![
				(6,5),(6,6),(6,7),(6,8)
			],
			vec![
				(4,3),(4,4),(4,5),(4,6),(4,7),(4,8)
			],
			vec![
				(4,2),(4,3),(4,4),(4,5),(4,6),(4,7),(4,8)
			]
		],
		// 桂馬
		vec![
			vec![
				(3,3),(3,4),(3,5),(4,3),(4,5),(5,3),(5,4),(5,5)
			],
			vec![
				(1,3),(1,4),(1,5),(2,3),(2,5),(3,3),(3,4),(3,5)
			],
			vec![
				(5,3),(5,4),(5,5),(6,3),(6,5),(7,3),(7,4),(7,5)
			],
			vec![
				(3,1),(3,2),(3,3),(4,1),(4,3),(5,1),(5,2),(5,3)
			],
			vec![
				(3,0),(3,1),(3,2),(4,0),(4,2),(5,0),(5,1),(5,2)
			]
		],
		// 銀
		vec![
			vec![
				(3,4),(4,5),(5,4)
			],
			vec![
				(1,4),(2,5),(3,4)
			],
			vec![
				(5,4),(6,5),(7,4)
			],
			vec![
				(3,2),(4,3),(5,2)
			],
			vec![
				(3,1),(4,2),(5,1)
			]
		],
		// 金,成歩,成香,成桂,成銀
		vec![
			vec![
				(3,5),(5,5)
			],
			vec![
				(1,5),(3,5)
			],
			vec![
				(5,5),(7,5)
			],
			vec![
				(3,3),(5,3)
			],
			vec![
				(3,2),(5,2)
			]
		],
		// 角
		vec![
			vec![
				(3,4),(4,3),(4,5),(5,4)
			],
			vec![
				(1,4),(2,3),(2,5),(3,4)
			],
			vec![
				(5,4),(6,3),(6,5),(7,4)
			],
			vec![
				(3,2),(4,1),(4,3),(5,2)
			],
			vec![
				(3,1),(4,0),(4,2),(5,1)
			]
		],
		// 飛車
		vec![
			vec![
				(3,3),(3,5),(5,3),(5,5)
			],
			vec![
				(1,3),(1,5),(3,3),(3,5)
			],
			vec![
				(5,3),(5,5),(7,3),(7,5)
			],
			vec![
				(3,1),(3,3),(5,1),(5,3)
			],
			vec![
				(3,0),(3,2),(5,0),(5,2)
			]
		],
		// 成角
		vec![
			vec![
				(2,3),(2,4),(2,5),(3,2),(3,6),(4,2),(4,6),(5,2),(5,6),(6,3),(6,4),(6,5)
			],
			vec![
				(0,3),(0,4),(0,5),(1,2),(1,6),(2,2),(2,6),(3,2),(3,6),(4,3),(4,4),(4,5)
			],
			vec![
				(4,3),(4,4),(4,5),(5,2),(5,6),(6,2),(6,6),(7,2),(7,6),(8,3),(8,4),(8,5)
			],
			vec![
				(2,1),(2,2),(2,3),(3,0),(3,4),(4,0),(4,4),(5,0),(5,4),(6,1),(6,2),(6,3)
			],
			vec![
				(2,0),(2,1),(2,2),(3,3),(4,3),(5,3),(6,0),(6,1),(6,2)
			]
		],
		// 成飛
		vec![
			vec![
				(2,2),(2,3),(2,5),(2,6),(3,2),(3,6),(5,2),(5,6),(6,2),(6,3),(6,5),(6,6)
			],
			vec![
				(0,2),(0,4),(0,5),(0,6),(1,2),(1,6),(2,6),(3,2),(3,6),(4,2),(4,4),(4,5),(4,6)
			],
			vec![
				(4,2),(4,4),(4,5),(4,6),(5,6),(6,2),(7,2),(7,6),(8,2),(8,4),(8,5),(8,6)
			],
			vec![
				(2,0),(2,1),(2,3),(2,4),(3,0),(3,4),(5,0),(5,4),(6,0),(6,1),(6,3),(6,4)
			],
			vec![
				(2,0),(2,2),(2,3),(3,3),(5,3),(6,0),(6,2),(6,3)
			]
		],
		// 王
		vec![
			vec![
				(2,2),(2,3),(2,4),(2,5),(2,6),(3,2),(3,6),(4,2),(4,6),(5,2),(5,6),(6,2),(6,3),(6,4),(6,5),(6,6)
			],
			vec![
				(0,2),(0,3),(0,4),(0,5),(0,6),(1,2),(1,6),(2,2),(2,6),(3,2),(3,6),(4,2),(4,3),(4,4),(4,5),(4,6)
			],
			vec![
				(4,2),(4,3),(4,4),(4,5),(4,6),(5,2),(5,6),(6,2),(6,6),(7,2),(7,6),(8,2),(8,3),(8,4),(8,5),(8,6)
			],
			vec![
				(2,0),(2,1),(2,2),(2,3),(2,4),(3,0),(3,4),(4,0),(4,4),(5,0),(5,4),(6,0),(6,1),(6,2),(6,3),(6,4)
			],
			vec![
				(2,0),(2,1),(2,2),(2,3),(3,3),(4,3),(5,3),(6,0),(6,1),(6,2),(6,3)
			]
		]
	];

	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ GFu ],
		vec![ GKyou ],
		vec![ GKei ],
		vec![ GGin ],
		vec![ GKin,GFuN,GKyouN,GKeiN,GGinN ],
		vec![ GKaku ],
		vec![ GHisha ],
		vec![ GKakuN ],
		vec![ GHishaN ],
		vec![ GOu ]
	];

	for (kinds,mvs) in kinds.iter().zip(&mvs) {
		for ((k,p),mvs) in kinds.iter().zip(&POSITIONS).zip(mvs) {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

				let state = State::new(banmen);

				let dx = m.0;
				let dy = m.1;

				let mv = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-dx),(8-dy)+1,true)));

				assert!(Rule::apply_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					mv
				).is_err(), format!("apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k,mv.to_move()));

				let mv = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-dx),(8-dy)+1,false)));

				assert!(Rule::apply_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					mv
				).is_err(), format!("apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k,mv.to_move()));
			}
		}
	}
}
#[test]
fn test_apply_valid_move_put_valid_sente() {
	let kinds:[MochigomaKind; 7] = [
		MochigomaKind::Fu,
		MochigomaKind::Kyou,
		MochigomaKind::Kei,
		MochigomaKind::Gin,
		MochigomaKind::Kin,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha
	];

	let deny_line:[u32; 7] = [
		1,
		1,
		2,
		0,
		0,
		0,
		0
	];

	let mut banmen = BANMEN_START_POS.clone();

	banmen.0[0][0] = Blank;
	banmen.0[1][0] = GKyou;
	banmen.0[0][1] = Blank;
	banmen.0[2][2] = GKei;

	let state = State::new(banmen.clone());

	for (kind,deny_line) in kinds.iter().zip(&deny_line) {
		let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

		ms.insert(*kind, 1);

		let mc = MochigomaCollections::Pair(ms,HashMap::new());

		for y in 0..9 {
			for x in 0..9 {
				if banmen.0[y][x] == Blank && *deny_line <= y as u32 {
					let m = rule::AppliedMove::from(Move::Put(*kind,KomaDstPutPosition(9 - x as u32,y as u32 + 1)));

					assert!(Rule::apply_valid_move(&state,
						Teban::Sente,
						&mc,
						m
					).is_ok(), format!("apply_valid_move returned Err: kind = {:?}, move = {:?}.", kind,m.to_move()));
				}
			}
		}
	}
}
#[test]
fn test_apply_valid_move_put_valid_gote() {
	let kinds:[MochigomaKind; 7] = [
		MochigomaKind::Fu,
		MochigomaKind::Kyou,
		MochigomaKind::Kei,
		MochigomaKind::Gin,
		MochigomaKind::Kin,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha
	];

	let deny_line:[u32; 7] = [
		1,
		1,
		2,
		0,
		0,
		0,
		0
	];

	let mut banmen = BANMEN_START_POS.clone();

	banmen.0[8-0][8-0] = Blank;
	banmen.0[8-1][8-0] = SKyou;
	banmen.0[8-0][8-1] = Blank;
	banmen.0[8-2][8-2] = SKei;

	let state = State::new(banmen.clone());

	for (kind,deny_line) in kinds.iter().zip(&deny_line) {
		let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

		mg.insert(*kind, 1);

		let mc = MochigomaCollections::Pair(HashMap::new(),mg);

		for y in 0..9 {
			for x in 0..9 {
				let (x,y) = (8-x,8-y);

				if banmen.0[y][x] == Blank && (8 - *deny_line) >= y as u32 {
					let m = rule::AppliedMove::from(Move::Put(*kind,KomaDstPutPosition(9 - x as u32,y as u32 + 1)));

					assert!(Rule::apply_valid_move(&state,
						Teban::Gote,
						&mc,
						m
					).is_ok(), format!("apply_valid_move returned Err: kind = {:?}, move = {:?}.", kind,m.to_move()));
				}
			}
		}
	}
}
#[test]
fn test_apply_valid_move_put_invalid_sente() {
	let kinds:[MochigomaKind; 7] = [
		MochigomaKind::Fu,
		MochigomaKind::Kyou,
		MochigomaKind::Kei,
		MochigomaKind::Gin,
		MochigomaKind::Kin,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha
	];

	let deny_line:[u32; 7] = [
		1,
		1,
		2,
		0,
		0,
		0,
		0
	];

	let mut banmen = BANMEN_START_POS.clone();

	banmen.0[0][0] = Blank;
	banmen.0[1][0] = GKyou;
	banmen.0[0][1] = Blank;
	banmen.0[2][2] = GKei;

	let state = State::new(banmen.clone());

	for (kind,deny_line) in kinds.iter().zip(&deny_line) {
		let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

		ms.insert(*kind, 1);

		let mc = MochigomaCollections::Pair(ms,HashMap::new());

		for y in 0..9 {
			for x in 0..9 {
				if banmen.0[y][x] != Blank || *deny_line > y as u32 {
					let m = rule::AppliedMove::from(Move::Put(*kind,KomaDstPutPosition(9 - x as u32,y as u32 + 1)));

					assert!(Rule::apply_valid_move(&state,
						Teban::Sente,
						&mc,
						m
					).is_err(), format!("apply_valid_move returned Ok: kind = {:?}, move = {:?}.", kind,m.to_move()));
				}
			}
		}
	}
}
#[test]
fn test_apply_valid_move_put_invalid_gote() {
	let kinds:[MochigomaKind; 7] = [
		MochigomaKind::Fu,
		MochigomaKind::Kyou,
		MochigomaKind::Kei,
		MochigomaKind::Gin,
		MochigomaKind::Kin,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha
	];

	let deny_line:[u32; 7] = [
		1,
		1,
		2,
		0,
		0,
		0,
		0
	];

	let mut banmen = BANMEN_START_POS.clone();

	banmen.0[8-0][8-0] = Blank;
	banmen.0[8-1][8-0] = SKyou;
	banmen.0[8-0][8-1] = Blank;
	banmen.0[8-2][8-2] = SKei;

	let state = State::new(banmen.clone());

	for (kind,deny_line) in kinds.iter().zip(&deny_line) {
		let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

		mg.insert(*kind, 1);

		let mc = MochigomaCollections::Pair(HashMap::new(),mg);

		for y in 0..9 {
			for x in 0..9 {
				let (x,y) = (8-x,8-y);

				if banmen.0[y][x] != Blank || (8 - *deny_line) < y as u32 {
					let m = rule::AppliedMove::from(Move::Put(*kind,KomaDstPutPosition(9 - x as u32,y as u32 + 1)));

					assert!(Rule::apply_valid_move(&state,
						Teban::Gote,
						&mc,
						m
					).is_err(), format!("apply_valid_move returned Ok: kind = {:?}, move = {:?}.", kind,m.to_move()));
				}
			}
		}
	}
}
#[test]
fn test_apply_moves() {
	let mut after_banmen = BANMEN_START_POS.clone();

	after_banmen.0[6][8] = Blank;
	after_banmen.0[4][8] = SFu;
	after_banmen.0[8][1] = Blank;
	after_banmen.0[6][2] = SKei;
	after_banmen.0[5][2] = SFu;

	after_banmen.0[8-6][8-8] = Blank;
	after_banmen.0[8-4][8-8] = GFu;
	after_banmen.0[8-8][8-1] = Blank;
	after_banmen.0[8-6][8-2] = GKei;
	after_banmen.0[8-5][8-2] = GFu;

	let mvs:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((8,6),(8,5,false),None),
		((8-8,8-6),(8-8,8-5,false),None),
		((8,5),(8,4,false),None),
		((8-8,8-5),(8-8,8-4,false),None),
		((2,6),(2,5,false),None),
		((8-2,8-6),(8-2,8-5,false),None),
		((1,8),(2,6,false),None),
		((8-1,8-8),(8-2,8-6,false),None),
	];

	let kyokumen_map:KyokumenMap<u64,u32> = KyokumenMap::new();
	let hasher = KyokumenHash::new();

	let (imhash, ishash) = hasher.calc_initial_hash(&BANMEN_START_POS,&HashMap::new(),&HashMap::new());

	let mvs = mvs.into_iter().map(|m| {
		rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(m.0).0,(m.0).1+1),KomaDstToPosition(9-(m.1).0,(m.1).1+1,(m.1).2)))
	}).collect::<Vec<rule::AppliedMove>>();

	let state = State::new(BANMEN_START_POS.clone());
	let teban = Teban::Sente;
	let mc = MochigomaCollections::Empty;

	let (_,
		 _,
		 _,
		 mhash,
		 shash,
		 _) = Rule::apply_moves(state,teban,mc,&mvs,imhash,ishash,kyokumen_map,&hasher);

	let (amhash, ashash) = hasher.calc_initial_hash(&after_banmen,&HashMap::new(),&HashMap::new());

	assert_eq!(amhash,mhash);
	assert_eq!(ashash,shash);

	assert!(mhash != imhash);
	assert!(mhash != ishash);
}
#[test]
fn test_apply_moves_with_callback() {
	let mut after_banmen = BANMEN_START_POS.clone();

	after_banmen.0[6][8] = Blank;
	after_banmen.0[4][8] = SFu;
	after_banmen.0[8][1] = Blank;
	after_banmen.0[6][2] = SKei;
	after_banmen.0[5][2] = SFu;

	after_banmen.0[8-6][8-8] = Blank;
	after_banmen.0[8-4][8-8] = GFu;
	after_banmen.0[8-8][8-1] = Blank;
	after_banmen.0[8-6][8-2] = GKei;
	after_banmen.0[8-5][8-2] = GFu;

	let mvs:Vec<((u32,u32),(u32,u32,bool),Option<ObtainKind>)> = vec![
		((8,6),(8,5,false),None),
		((8-8,8-6),(8-8,8-5,false),None),
		((8,5),(8,4,false),None),
		((8-8,8-5),(8-8,8-4,false),None),
		((2,6),(2,5,false),None),
		((8-2,8-6),(8-2,8-5,false),None),
		((1,8),(2,6,false),None),
		((8-1,8-8),(8-2,8-6,false),None),
	];

	let hasher = KyokumenHash::new();

	let (imhash, ishash) = hasher.calc_initial_hash(&BANMEN_START_POS,&HashMap::new(),&HashMap::new());

	let mvs = mvs.into_iter().map(|m| {
		rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(m.0).0,(m.0).1+1),KomaDstToPosition(9-(m.1).0,(m.1).1+1,(m.1).2)))
	}).collect::<Vec<rule::AppliedMove>>();

	let state = State::new(BANMEN_START_POS.clone());
	let teban = Teban::Sente;
	let mc = MochigomaCollections::Empty;

	let (_, _, _, r) = Rule::apply_moves_with_callback(state,
											 	teban,
											 	mc,
											 	&mvs,
											 	(imhash,ishash),
											 	|banmen,teban,mc,m,o,r:(u64,u64)| {
			let (mhash,shash) = r;

			if let Some(m) = m {
				let mhash = hasher.calc_main_hash(mhash,teban,banmen,mc,*m,o);
				let shash = hasher.calc_sub_hash(shash,teban,banmen,mc,*m,o);

				(mhash,shash)
			} else {
				(mhash,shash)
			}
		 });

	let (mhash,shash) = r;
	let (amhash, ashash) = hasher.calc_initial_hash(&after_banmen,&HashMap::new(),&HashMap::new());

	assert_eq!(amhash,mhash);
	assert_eq!(ashash,shash);

	assert!(mhash != imhash);
	assert!(mhash != ishash);
}
#[test]
fn test_is_nyugyoku_win_win_sente() {
	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,Blank,SKin,Blank,SKin,Blank],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SFuN],
		[Blank,SGin,SHishaN,SFuN,SFuN,Blank,Blank,Blank,SOu],
		[GFu,Blank,Blank,Blank,Blank,SKakuN,Blank,Blank,Blank],
		[Blank,SHishaN,SFu,SKakuN,Blank,Blank,Blank,Blank,Blank],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,Blank,Blank,Blank,Blank,Blank,Blank,GOu,Blank]
	]);

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Fu,7);
	ms.insert(MochigomaKind::Kyou,2);
	ms.insert(MochigomaKind::Gin,1);
	ms.insert(MochigomaKind::Kei,1);
	mg.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&Some(Instant::now())));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,SHishaN,Blank,Blank,Blank,SGin,Blank,SHishaN],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SFuN,SKakuN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&Some(Instant::now())));

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(HashMap::new(),HashMap::new()),&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(HashMap::new(),HashMap::new()),&Some(Instant::now())));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,SHishaN,Blank,Blank,Blank,SGin,Blank,SKakuN],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SFuN,SHishaN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&Some(Instant::now())));

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(HashMap::new(),HashMap::new()),&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(HashMap::new(),HashMap::new()),&Some(Instant::now())));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SHishaN],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SFuN,SKakuN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Hisha,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(ms,HashMap::new()),&None));

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Hisha,1);
	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(ms,HashMap::new()),&Some(Instant::now())));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,SHishaN,Blank,Blank,Blank,SGin,Blank,Blank],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SFuN,SHishaN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);


	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Kaku,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(ms,HashMap::new()),&None));

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Kaku,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(ms,HashMap::new()),&Some(Instant::now())));
}
#[test]
fn test_is_nyugyoku_win_lose_sente() {
	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,Blank,SKin,Blank,SKin,Blank],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SFuN],
		[Blank,SGin,SHishaN,SFuN,SFuN,Blank,Blank,Blank,SOu],
		[GFu,Blank,Blank,Blank,Blank,SKakuN,Blank,Blank,Blank],
		[Blank,SHishaN,SFu,SKakuN,Blank,Blank,Blank,Blank,Blank],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,Blank,Blank,Blank,Blank,Blank,Blank,GOu,Blank]
	]);

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Fu,7);
	ms.insert(MochigomaKind::Kyou,2);
	ms.insert(MochigomaKind::Gin,1);
	ms.insert(MochigomaKind::Kei,1);
	mg.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&Some(Instant::now() + Duration::from_secs(60))));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,SHishaN,Blank,Blank,Blank,SGin,Blank,SHishaN],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SFuN,SKakuN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,
			&MochigomaCollections::Empty,&Some(Instant::now() + Duration::from_secs(60))));

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,
			&MochigomaCollections::Pair(HashMap::new(),HashMap::new()),&Some(Instant::now() + Duration::from_secs(60))));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SHishaN],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SFuN,SKakuN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Hisha,1);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,
			&MochigomaCollections::Pair(ms,HashMap::new()),&Some(Instant::now() + Duration::from_secs(60))));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,Blank,SKin,Blank,SKin,Blank],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SFuN],
		[Blank,Blank,SHishaN,SFuN,SFuN,Blank,Blank,Blank,SOu],
		[GFu,SGin,Blank,Blank,Blank,SKakuN,Blank,Blank,Blank],
		[Blank,SHishaN,SFu,SKakuN,Blank,Blank,Blank,Blank,Blank],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,Blank,Blank,Blank,Blank,Blank,Blank,GOu,Blank]
	]);

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Fu,7);
	ms.insert(MochigomaKind::Kyou,2);
	ms.insert(MochigomaKind::Gin,1);
	ms.insert(MochigomaKind::Kei,1);
	mg.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&Some(Instant::now())));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&None));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,Blank,SKin,Blank,SKin,Blank],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SFuN],
		[Blank,SGin,SFu,SFuN,SFuN,Blank,Blank,Blank,SOu],
		[GFu,Blank,Blank,Blank,Blank,SKakuN,Blank,Blank,Blank],
		[Blank,SHishaN,SHishaN,SKakuN,Blank,Blank,Blank,Blank,Blank],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,Blank,Blank,Blank,Blank,Blank,Blank,GOu,Blank]
	]);

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Fu,7);
	ms.insert(MochigomaKind::Kyou,2);
	ms.insert(MochigomaKind::Gin,1);
	ms.insert(MochigomaKind::Kei,1);
	mg.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&Some(Instant::now())));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&None));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,SHishaN,Blank,Blank,Blank,SGin,Blank,SHishaN],
		[Blank,Blank,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,SGin,Blank,Blank,SFuN,SKakuN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&None));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&Some(Instant::now())));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,SHishaN,Blank,Blank,Blank,SGin,Blank,SFuN],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SHishaN,SKakuN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&None));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&Some(Instant::now())));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,Blank,SKin,Blank,SKin,Blank],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SFuN],
		[Blank,SGin,SHishaN,SFuN,SFuN,Blank,Blank,Blank,Blank],
		[GFu,Blank,Blank,Blank,Blank,SKakuN,Blank,Blank,SOu],
		[Blank,SHishaN,SFu,SKakuN,Blank,Blank,Blank,Blank,Blank],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,Blank,Blank,Blank,Blank,Blank,Blank,GOu,Blank]
	]);

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	ms.insert(MochigomaKind::Fu,7);
	ms.insert(MochigomaKind::Kyou,2);
	ms.insert(MochigomaKind::Gin,1);
	ms.insert(MochigomaKind::Kei,1);
	mg.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&None));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&Some(Instant::now())));
}
#[test]
fn test_is_nyugyoku_win_win_gote() {
	let banmen = Banmen([
		[Blank,SOu,Blank,Blank,Blank,Blank,Blank,Blank,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[Blank,Blank,Blank,Blank,Blank,GKakuN,GFu,GHishaN,Blank],
		[Blank,Blank,Blank,GKakuN,Blank,Blank,Blank,GGin,SFu],
		[GOu,Blank,Blank,Blank,GFuN,GFuN,GHishaN,Blank,Blank],
		[GFuN,Blank,GGin,Blank,Blank,Blank,Blank,GKyouN,Blank],
		[Blank,GKin,Blank,GKin,Blank,GFuN,GFuN,GFuN,GFuN]
	]);

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Fu,7);
	mg.insert(MochigomaKind::Kyou,2);
	mg.insert(MochigomaKind::Gin,1);
	mg.insert(MochigomaKind::Kei,1);
	ms.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&Some(Instant::now())));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GKakuN,GFuN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GHishaN,Blank,GGin,Blank,Blank,Blank,GHishaN,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&Some(Instant::now())));

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(HashMap::new(),HashMap::new()),&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(HashMap::new(),HashMap::new()),&Some(Instant::now())));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GHishaN,GFuN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GKakuN,Blank,GGin,Blank,Blank,Blank,GHishaN,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&Some(Instant::now())));

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(HashMap::new(),HashMap::new()),&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(HashMap::new(),HashMap::new()),&Some(Instant::now())));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GKakuN,GFuN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GHishaN,Blank,GGin,Blank,Blank,Blank,Blank,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Hisha,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(HashMap::new(),mg),&None));

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Hisha,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(HashMap::new(),mg),&Some(Instant::now())));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GHishaN,GFuN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,Blank,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GKakuN,Blank,GGin,Blank,Blank,Blank,GHishaN,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Kaku,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(HashMap::new(),mg),&None));

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Kaku,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(HashMap::new(),mg),&Some(Instant::now())));
}
#[test]
fn test_is_nyugyoku_win_lose_gote() {
	let banmen = Banmen([
		[Blank,SOu,Blank,Blank,Blank,Blank,Blank,Blank,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[Blank,Blank,Blank,Blank,Blank,GKakuN,GFu,GHishaN,Blank],
		[Blank,Blank,Blank,GKakuN,Blank,Blank,Blank,GGin,SFu],
		[GOu,Blank,Blank,Blank,GFuN,GFuN,GHishaN,Blank,Blank],
		[GFuN,Blank,GGin,Blank,Blank,Blank,Blank,GKyouN,Blank],
		[Blank,GKin,Blank,GKin,Blank,GFuN,GFuN,GFuN,GFuN]
	]);

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Fu,7);
	mg.insert(MochigomaKind::Kyou,2);
	mg.insert(MochigomaKind::Gin,1);
	mg.insert(MochigomaKind::Kei,1);
	ms.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&Some(Instant::now() + Duration::from_secs(60))));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GKakuN,GFuN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GHishaN,Blank,GGin,Blank,Blank,Blank,GHishaN,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,
			&MochigomaCollections::Empty,&Some(Instant::now() + Duration::from_secs(60))));

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,
			&MochigomaCollections::Pair(HashMap::new(),HashMap::new()),&Some(Instant::now() + Duration::from_secs(60))));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GKakuN,GFuN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GHishaN,Blank,GGin,Blank,Blank,Blank,Blank,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Hisha,1);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,
			&MochigomaCollections::Pair(HashMap::new(),mg),&Some(Instant::now() + Duration::from_secs(60))));

	let banmen = Banmen([
		[Blank,SOu,Blank,Blank,Blank,Blank,Blank,Blank,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[Blank,Blank,Blank,Blank,Blank,GKakuN,GFu,GHishaN,Blank],
		[Blank,Blank,Blank,GKakuN,Blank,Blank,GKyouN,GGin,SFu],
		[GOu,Blank,Blank,Blank,GFuN,GFuN,GHishaN,Blank,Blank],
		[GFuN,Blank,GGin,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,GKin,Blank,GKin,Blank,GFuN,GFuN,GFuN,GFuN]
	]);

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Fu,7);
	mg.insert(MochigomaKind::Kyou,2);
	mg.insert(MochigomaKind::Gin,1);
	mg.insert(MochigomaKind::Kei,1);
	ms.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&Some(Instant::now())));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&None));

	let banmen = Banmen([
		[Blank,SOu,Blank,Blank,Blank,Blank,Blank,Blank,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[Blank,Blank,Blank,Blank,Blank,GKakuN,GHishaN,GHishaN,Blank],
		[Blank,Blank,Blank,GKakuN,Blank,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,Blank,Blank,GFuN,GFuN,GFu,GGin,Blank],
		[GFuN,Blank,GGin,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,GKin,Blank,GKin,Blank,GFuN,GFuN,GFuN,GFuN]
	]);

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Fu,7);
	mg.insert(MochigomaKind::Kyou,2);
	mg.insert(MochigomaKind::Gin,1);
	mg.insert(MochigomaKind::Kei,1);
	ms.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&Some(Instant::now())));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&None));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GKakuN,GFuN,Blank,GKyouN,GGin,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,Blank,Blank],
		[GHishaN,Blank,GGin,Blank,Blank,Blank,GHishaN,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&None));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&Some(Instant::now())));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GKakuN,GHishaN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GFuN,Blank,GGin,Blank,Blank,Blank,GHishaN,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&None));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&Some(Instant::now())));

	let banmen = Banmen([
		[Blank,SOu,Blank,Blank,Blank,Blank,Blank,Blank,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[Blank,Blank,Blank,Blank,Blank,GKakuN,GFu,GHishaN,Blank],
		[GOu,Blank,Blank,GKakuN,Blank,Blank,Blank,GKyouN,SFu],
		[Blank,Blank,Blank,Blank,GFuN,GFuN,GHishaN,GGin,Blank],
		[GFuN,Blank,GGin,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,GKin,Blank,GKin,Blank,GFuN,GFuN,GFuN,GFuN]
	]);

	let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
	let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

	mg.insert(MochigomaKind::Fu,7);
	mg.insert(MochigomaKind::Kyou,2);
	mg.insert(MochigomaKind::Gin,1);
	mg.insert(MochigomaKind::Kei,1);
	ms.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&None));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&Some(Instant::now())));
}
#[test]
fn test_responded_oute_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mvs:Vec<Move> = vec![
		Move::To(KomaSrcPosition(9-5,8+1),KomaDstToPosition(9-4,7+1,false)),
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,7+1)),
		Move::To(KomaSrcPosition(9-4,8+1),KomaDstToPosition(9-4,7+1,false)),
		Move::To(KomaSrcPosition(9-4,8+1),KomaDstToPosition(9-5,7+1,false)),
		Move::To(KomaSrcPosition(9-3,8+1),KomaDstToPosition(9-3,7+1,false)),
		Move::To(KomaSrcPosition(9-4,8+1),KomaDstToPosition(9-4,7+1,false)),
	];

	let position_and_kinds:Vec<Vec<(usize,usize,KomaKind)>> = vec![
		vec![
			(5,8,SKin),(4,8,SOu),(3,8,SKin),(4,7,GKin)
		],
		vec![
			(5,8,SKin),(4,8,SOu),(3,8,SKin),(4,0,GKyou)
		],
		vec![
			(5,8,SKin),(4,8,SOu),(3,8,SKin),(4,7,GKin)
		],
		vec![
			(5,8,SKin),(4,8,SOu),(3,8,SKin),(4,0,GKyou)
		],
		vec![
			(5,8,SKin),(4,8,SOu),(3,8,SKin),(4,7,GKin)
		],
		vec![
			(5,8,SKin),(4,8,SOu),(3,8,SKin),(4,7,GKin),(4,0,GKyou)
		],
	];

	let answer:[bool; 6] = [
		true,true,true,true,false,false
	];

	for ((pk,m),answer) in position_and_kinds.iter().zip(&mvs).zip(&answer) {
		let mut banmen = blank_banmen.clone();

		for pk in pk {
			banmen.0[pk.1][pk.0] = pk.2;
		}

		let state = State::new(banmen);

		let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
		let mg:HashMap<MochigomaKind,u32> = HashMap::new();

		ms.insert(MochigomaKind::Fu,1);

		match Rule::responded_oute(&state,Teban::Sente,&MochigomaCollections::Pair(ms,mg),m.to_applied_move()) {
			Ok(r) => {
				assert_eq!(*answer,r, "assertion failed: `(left == right), move = {:?}, {:?}",m,state.get_banmen());
			},
			Err(_) => {
				assert!(false, "responded_oute returned Err (no mate). {:?}", state.get_banmen());
			}
		}
	}
}
#[test]
fn test_responded_oute_error_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mvs:Vec<Move> = vec![
		Move::To(KomaSrcPosition(9-5,8+1),KomaDstToPosition(9-4,7+1,false)),
	];

	let position_and_kinds:Vec<Vec<(usize,usize,KomaKind)>> = vec![
		vec![
			(5,8,SKin),(4,8,SOu),(3,8,SKin),(4,6,GKin)
		]
	];

	for (pk,m) in position_and_kinds.iter().zip(&mvs) {
		let mut banmen = blank_banmen.clone();

		for pk in pk {
			banmen.0[pk.1][pk.0] = pk.2;
		}

		let state = State::new(banmen);

		let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
		let mg:HashMap<MochigomaKind,u32> = HashMap::new();

		ms.insert(MochigomaKind::Fu,1);

		match Rule::responded_oute(&state,Teban::Sente,&MochigomaCollections::Pair(ms,mg),m.to_applied_move()) {
			Ok(_) => {
				assert!(false, "responded_oute returned Ok. {:?}", state.get_banmen());
			},
			Err(_) => {
				assert!(true);
			}
		}
	}
}
#[test]
fn test_responded_oute_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mvs:Vec<Move> = vec![
		Move::To(KomaSrcPosition(9-(8-5),(8-8)+1),KomaDstToPosition(9-(8-4),(8-7)+1,false)),
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-(8-4),(8-7)+1)),
		Move::To(KomaSrcPosition(9-(8-4),(8-8)+1),KomaDstToPosition(9-(8-4),(8-7)+1,false)),
		Move::To(KomaSrcPosition(9-(8-4),(8-8)+1),KomaDstToPosition(9-(8-5),(8-7)+1,false)),
		Move::To(KomaSrcPosition(9-(8-3),(8-8)+1),KomaDstToPosition(9-(8-3),(8-7)+1,false)),
		Move::To(KomaSrcPosition(9-(8-4),(8-8)+1),KomaDstToPosition(9-(8-4),(8-7)+1,false)),
	];

	let position_and_kinds:Vec<Vec<(usize,usize,KomaKind)>> = vec![
		vec![
			(8-5,8-8,GKin),(8-4,8-8,GOu),(8-3,8-8,GKin),(8-4,8-7,SKin)
		],
		vec![
			(8-5,8-8,GKin),(8-4,8-8,GOu),(8-3,8-8,GKin),(8-4,8-0,SKyou)
		],
		vec![
			(8-5,8-8,GKin),(8-4,8-8,GOu),(8-3,8-8,GKin),(8-4,8-7,SKin)
		],
		vec![
			(8-5,8-8,GKin),(8-4,8-8,GOu),(8-3,8-8,GKin),(8-4,8-0,SKyou)
		],
		vec![
			(8-5,8-8,GKin),(8-4,8-8,GOu),(8-3,8-8,GKin),(8-4,8-7,SKin)
		],
		vec![
			(8-5,8-8,GKin),(8-4,8-8,GOu),(8-3,8-8,GKin),(8-4,8-7,SKin),(8-4,8-0,SKyou)
		],
	];

	let answer:[bool; 6] = [
		true,true,true,true,false,false
	];

	for ((pk,m),answer) in position_and_kinds.iter().zip(&mvs).zip(&answer) {
		let mut banmen = blank_banmen.clone();

		for pk in pk {
			banmen.0[pk.1][pk.0] = pk.2;
		}

		let state = State::new(banmen);

		let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();
		let ms:HashMap<MochigomaKind,u32> = HashMap::new();

		mg.insert(MochigomaKind::Fu,1);

		match Rule::responded_oute(&state,Teban::Gote,&MochigomaCollections::Pair(ms,mg),m.to_applied_move()) {
			Ok(r) => {
				assert_eq!(*answer,r, "assertion failed: `(left == right), move = {:?}, {:?}",m,state.get_banmen());
			},
			Err(_) => {
				assert!(false, "responded_oute returned Err (no mate). {:?}", state.get_banmen());
			}
		}
	}
}
#[test]
fn test_responded_oute_error_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mvs:Vec<Move> = vec![
		Move::To(KomaSrcPosition(9-(8-5),(8-8)+1),KomaDstToPosition(9-(8-4),(8-7)+1,false)),
	];

	let position_and_kinds:Vec<Vec<(usize,usize,KomaKind)>> = vec![
		vec![
			(8-5,8-8,GKin),(8-4,8-8,GOu),(8-3,8-8,GKin),(8-4,8-6,SKin)
		]
	];

	for (pk,m) in position_and_kinds.iter().zip(&mvs) {
		let mut banmen = blank_banmen.clone();

		for pk in pk {
			banmen.0[pk.1][pk.0] = pk.2;
		}

		let state = State::new(banmen);

		let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();
		let ms:HashMap<MochigomaKind,u32> = HashMap::new();

		mg.insert(MochigomaKind::Fu,1);

		match Rule::responded_oute(&state,Teban::Gote,&MochigomaCollections::Pair(ms,mg),m.to_applied_move()) {
			Ok(_) => {
				assert!(false, "responded_oute returned Ok. {:?}", state.get_banmen());
			},
			Err(_) => {
				assert!(true);
			}
		}
	}
}
#[test]
fn test_is_put_fu_and_mate_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let m = Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,1+1));

	let position_and_kinds:Vec<Vec<(usize,usize,KomaKind)>> = vec![
		vec![
			(3,0,GFu),(3,1,GKyou),(4,0,GOu),(5,0,GFu),(5,1,GKyou),(4,8,SKyou)
		],
		vec![
			(3,0,GKin),(3,1,GKyou),(4,0,GOu),(5,0,GKin),(5,1,GKyou),(4,8,SKyou)
		],
		vec![
			(3,0,GFu),(3,1,GKyou),(4,0,GOu),(5,1,GKyou),(4,8,SKyou)
		],
		vec![
			(3,0,GFu),(3,1,GKyou),(4,0,GOu),(5,0,GKyou),(5,1,GGin)
		]
	];

	let answer:[bool; 4] = [
		true,false,false,false
	];

	for (pk,answer) in position_and_kinds.iter().zip(&answer) {
		let mut banmen = blank_banmen.clone();

		for pk in pk {
			banmen.0[pk.1][pk.0] = pk.2;
		}

		let mut state = State::new(banmen);

		let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
		let mg:HashMap<MochigomaKind,u32> = HashMap::new();

		ms.insert(MochigomaKind::Fu,1);

		let mut mc = MochigomaCollections::Pair(ms,mg);

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(*answer,Rule::is_put_fu_and_mate(
								&state,Teban::Sente,&mc,m.to_applied_move()),
								"assertion failed: `(left == right), move = {:?}, {:?}",m,state.get_banmen())
	}
}
#[test]
fn test_is_put_fu_and_mate_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let m = Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-(8-4),(8-1)+1));

	let position_and_kinds:Vec<Vec<(usize,usize,KomaKind)>> = vec![
		vec![
			(8-3,8-0,SFu),(8-3,8-1,SKyou),(8-4,8-0,SOu),(8-5,8-0,SFu),(8-5,8-1,SKyou),(8-4,8-8,GKyou)
		],
		vec![
			(8-3,8-0,SKin),(8-3,8-1,SKyou),(8-4,8-0,SOu),(8-5,8-0,SKin),(8-5,8-1,SKyou),(8-4,8-8,GKyou)
		],
		vec![
			(8-3,8-0,SFu),(8-3,8-1,SKyou),(8-4,8-0,SOu),(8-5,8-1,SKyou),(8-4,8-8,GKyou)
		],
		vec![
			(8-3,8-0,SFu),(8-3,8-1,SKyou),(8-4,8-0,SOu),(8-5,8-0,SKyou),(8-5,8-1,SGin)
		]
	];

	let answer:[bool; 4] = [
		true,false,false,false
	];

	for (pk,answer) in position_and_kinds.iter().zip(&answer) {
		let mut banmen = blank_banmen.clone();

		for pk in pk {
			banmen.0[pk.1][pk.0] = pk.2;
		}

		let mut state = State::new(banmen);

		let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();
		let ms:HashMap<MochigomaKind,u32> = HashMap::new();

		mg.insert(MochigomaKind::Fu,1);

		let mut mc = MochigomaCollections::Pair(ms,mg);

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		assert_eq!(*answer,Rule::is_put_fu_and_mate(
								&state,Teban::Gote,&mc,m.to_applied_move()),
								"assertion failed: `(left == right), move = {:?}, {:?}",m,state.get_banmen())
	}
}
#[test]
fn test_is_win_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mvs:Vec<Move> = vec![
		Move::To(KomaSrcPosition(9-4,1+1),KomaDstToPosition(9-4,0+1,false)),
		Move::To(KomaSrcPosition(9-4,2+1),KomaDstToPosition(9-4,1+1,false)),
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,1+1))
	];

	let position_and_kinds:Vec<Vec<(usize,usize,KomaKind)>> = vec![
		vec![ (4,0,GOu),(4,1,SFu) ],
		vec![ (4,0,GOu),(4,2,SFu) ],
		vec![ (4,0,GOu),(4,2,SFu) ]
	];

	let answer:[bool; 3] = [
		true,false,false
	];

	for ((pk,m),answer) in position_and_kinds.iter().zip(&mvs).zip(&answer) {
		let mut banmen = blank_banmen.clone();

		for pk in pk {
			banmen.0[pk.1][pk.0] = pk.2;
		}

		let mut state = State::new(banmen);

		assert_eq!(*answer,Rule::is_win(
								&state,Teban::Sente,m.to_applied_move()),
								"assertion failed: `(left == right), move = {:?}, {:?}",m,state.get_banmen())
	}
}
#[test]
fn test_is_win_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mvs:Vec<Move> = vec![
		Move::To(KomaSrcPosition(9-(8-4),(8-1)+1),KomaDstToPosition(9-(8-4),(8-0)+1,false)),
		Move::To(KomaSrcPosition(9-(8-4),(8-2)+1),KomaDstToPosition(9-(8-4),(8-1)+1,false)),
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-(8-4),(8-1)+1))
	];

	let position_and_kinds:Vec<Vec<(usize,usize,KomaKind)>> = vec![
		vec![ (8-4,8-0,SOu),(8-4,8-1,GFu) ],
		vec![ (8-4,8-0,SOu),(8-4,8-2,GFu) ],
		vec![ (8-4,8-0,SOu),(8-4,8-2,GFu) ]
	];

	let answer:[bool; 3] = [
		true,false,false
	];

	for ((pk,m),answer) in position_and_kinds.iter().zip(&mvs).zip(&answer) {
		let mut banmen = blank_banmen.clone();

		for pk in pk {
			banmen.0[pk.1][pk.0] = pk.2;
		}

		let mut state = State::new(banmen);

		assert_eq!(*answer,Rule::is_win(
								&state,Teban::Gote,m.to_applied_move()),
								"assertion failed: `(left == right), move = {:?}, {:?}",m,state.get_banmen())
	}
}
#[test]
fn test_is_mate_to_sente() {
	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ SFu ],
		vec![ SKei ],
		vec![ SGin ],
		vec![ SKin, SFuN, SKyouN, SKeiN, SGinN ],
		vec![ SKaku ],
		vec![ SHisha ],
		vec![ SKakuN ],
		vec![ SHishaN ],
		vec![ SOu ]
	];

	let positions:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,6)],
		vec![(2,8),(6,8)],
		vec![(2,2),(2,6),(4,6),(6,2),(6,6)],
		vec![(2,4),(2,6),(4,2),(4,6),(6,4),(6,6)],
		vec![(0,4),(4,0),(4,8),(8,4)],
		vec![(1,1),(1,7),(7,1),(7,7)],
		vec![(0,4),(4,0),(4,8),(8,4),(2,4),(4,2),(4,6),(6,4)],
		vec![(1,1),(1,7),(7,1),(7,7),(2,2),(2,6),(6,2),(6,6)],
		vec![(2,2),(2,4),(2,6),(4,2),(4,6),(6,2),(6,4),(6,6)]
	];

	let mvs:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,5)],
		vec![(3,6),(5,6)],
		vec![(3,3),(3,5),(4,5),(5,3),(5,5)],
		vec![(3,4),(3,5),(4,3),(4,5),(5,4),(5,5)],
		vec![(2,2),(6,2),(6,6),(2,6)],
		vec![(4,1),(1,4),(7,4),(4,7)],
		vec![(2,2),(6,2),(6,6),(2,6),(3,4),(4,3),(4,5),(5,4)],
		vec![(4,1),(1,4),(7,4),(4,7),(3,3),(3,5),(5,3),(5,5)],
		vec![(3,3),(3,4),(3,5),(4,3),(4,5),(5,3),(5,4),(5,5)],
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for ((p,kinds),mvs) in positions.iter().zip(&kinds).zip(&mvs) {
		for (k,p) in kinds.iter().zip(p) {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[3][4] = GOu;
				banmen.0[p.1 as usize][p.0 as usize] = *k;

				let mut state = State::new(banmen);

				let mut mc = MochigomaCollections::Empty;

				let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

				match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
					(next,nmc,_) => {
						state = next;
						mc = nmc;
					}
				}

				let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

				match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
					(next,_,_) => {
						state = next;
					}
				}

				assert!(Rule::is_mate(Teban::Sente,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
			}
		}
	}
}
#[test]
fn test_is_mate_put_sente() {
	let kinds:Vec<MochigomaKind> = vec![
		MochigomaKind::Fu,
		MochigomaKind::Kyou,
		MochigomaKind::Kei,
		MochigomaKind::Gin,
		MochigomaKind::Kin,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha,
	];

	let mvs:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,5)],
		vec![(4,8)],
		vec![(3,6),(5,6)],
		vec![(3,3),(3,5),(4,5),(5,3),(5,5)],
		vec![(3,4),(3,5),(4,3),(4,5),(5,4),(5,5)],
		vec![(2,2),(6,2),(6,6),(2,6)],
		vec![(4,1),(1,4),(7,4),(4,7)],
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (k,mvs) in kinds.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::Put(*k,KomaDstPutPosition(9-m.0,m.1+1));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(Rule::is_mate(Teban::Sente,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_kyou_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[3][4] = GOu;
	banmen.0[8][4] = SKyou;
	banmen.0[5][4] = GKin;

	let mut state = State::new(banmen);

	let mut mc = MochigomaCollections::Empty;

	let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

	match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
		(next,nmc,_) => {
			state = next;
			mc = nmc;
		}
	}

	let mv = Move::To(KomaSrcPosition(9-4,5+1),KomaDstToPosition(9-3,5+1,false));

	match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
		(next,_,_) => {
			state = next;
		}
	}

	assert!(Rule::is_mate(Teban::Sente,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
}
#[test]
fn test_is_mate_with_kaku_2step_sente() {
	let kinds:Vec<KomaKind> = vec![ SKaku, SKakuN ];

	let positions:Vec<(u32,u32)> = vec![
		(0,4),(4,0),(4,8),(8,4)
	];

	let mvs:Vec<(u32,u32)> = vec![
		(2,2),(6,2),(6,6),(2,6)
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(3,3),(5,3),(5,5),(3,5)
	];

	let occ_mvs:Vec<(u32,u32)> = vec![
		(3,2),(5,2),(5,6),(3,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),(op,om)) in positions.iter().zip(&mvs).zip(occ_positions.iter().zip(&occ_mvs)) {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = *k;

			banmen.0[op.1 as usize][op.0 as usize] = GKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-op.0,op.1+1),KomaDstToPosition(9-om.0,om.1+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(Rule::is_mate(Teban::Sente,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_hisha_2step_sente() {
	let kinds:Vec<KomaKind> = vec![ SHisha, SHishaN ];

	let positions:Vec<(u32,u32)> = vec![
		(1,1),(1,7),(7,1),(7,7),
	];

	let mvs:Vec<(u32,u32)> = vec![
		(4,1),(1,4),(7,4),(4,7),
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(4,2),(2,4),(6,4),(4,6)
	];

	let occ_mvs:Vec<(u32,u32)> = vec![
		(3,2),(2,5),(6,6),(3,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),(op,om)) in positions.iter().zip(&mvs).zip(occ_positions.iter().zip(&occ_mvs)) {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = *k;

			banmen.0[op.1 as usize][op.0 as usize] = GKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-op.0,op.1+1),KomaDstToPosition(9-om.0,om.1+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(Rule::is_mate(Teban::Sente,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_no_mate_to_sente() {
	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ SFu ],
		vec![ SKei ],
		vec![ SGin ],
		vec![ SKin, SFuN, SKyouN, SKeiN, SGinN ],
		vec![ SKaku ],
		vec![ SHisha ],
		vec![ SKakuN ],
		vec![ SHishaN ],
		vec![ SOu ]
	];

	let positions:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,7)],
		vec![(3,8),(5,8)],
		vec![(2,3),(3,2),(5,2),(6,3)],
		vec![(3,2),(5,2)],
		vec![(2,2),(6,2),(6,6),(2,6)],
		vec![(4,1),(1,4),(7,4),(4,7)],
		vec![(2,2),(6,2),(6,6),(2,6),(3,4),(4,3),(4,5),(5,4)],
		vec![(4,1),(1,4),(7,4),(4,7),(3,3),(3,5),(5,3),(5,5)],
		vec![(1,1),(1,4),(1,7),(4,1),(4,7),(7,1),(7,4),(7,7)]
	];

	let mvs:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,6)],
		vec![(4,6)],
		vec![(3,4),(4,3),(4,3),(5,4)],
		vec![(3,3),(5,3)],
		vec![(0,4),(4,0),(4,8),(8,4)],
		vec![(1,1),(1,7),(7,1),(7,7)],
		vec![(0,4),(4,0),(4,8),(8,4),(2,4),(4,2),(4,6),(6,4)],
		vec![(1,1),(1,7),(7,1),(7,7),(2,2),(2,6),(6,2),(6,6)],
		vec![(2,2),(2,4),(2,6),(4,2),(4,6),(6,2),(6,4),(6,6)],
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for ((p,kinds),mvs) in positions.iter().zip(&kinds).zip(&mvs) {
		for (k,p) in kinds.iter().zip(p) {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[3][4] = GOu;
				banmen.0[p.1 as usize][p.0 as usize] = *k;

				let mut state = State::new(banmen);

				let mut mc = MochigomaCollections::Empty;

				let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

				match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
					(next,nmc,_) => {
						state = next;
						mc = nmc;
					}
				}

				let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

				match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
					(next,_,_) => {
						state = next;
					}
				}

				assert!(!Rule::is_mate(Teban::Sente,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
			}
		}
	}
}
#[test]
fn test_is_mate_no_mate_put_sente() {
	let kinds:Vec<MochigomaKind> = vec![
		MochigomaKind::Fu,
		MochigomaKind::Kei,
		MochigomaKind::Gin,
		MochigomaKind::Kin,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha,
	];

	let mvs:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,6)],
		vec![(4,6)],
		vec![(3,4),(4,3),(4,3),(5,4)],
		vec![(3,3),(5,3)],
		vec![(0,4),(4,0),(4,8),(8,4)],
		vec![(1,1),(1,7),(7,1),(7,7)],
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (k,mvs) in kinds.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::Put(*k,KomaDstPutPosition(9-m.0,m.1+1));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(!Rule::is_mate(Teban::Sente,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_no_mate_with_kyou_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[3][4] = GOu;
	banmen.0[8][4] = SKyou;
	banmen.0[6][4] = SKin;

	let mut state = State::new(banmen);

	let mc = MochigomaCollections::Empty;

	let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

	match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
		(next,_,_) => {
			state = next;
		}
	}

	assert!(!Rule::is_mate(Teban::Sente,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
}
#[test]
fn test_is_mate_no_mate_with_kaku_occupied_self_sente() {
	let kinds:Vec<KomaKind> = vec![ SKaku, SKakuN ];

	let positions:Vec<(u32,u32)> = vec![
		(0,4),(4,0),(4,8),(8,4)
	];

	let mvs:Vec<(u32,u32)> = vec![
		(2,2),(6,2),(6,6),(2,6)
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(3,3),(5,3),(5,5),(3,5)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = *k;

			banmen.0[op.1 as usize][op.0 as usize] = SFu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(!Rule::is_mate(Teban::Sente,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_no_mate_with_hisha_occupied_self_sente() {
	let kinds:Vec<KomaKind> = vec![ SHisha, SHishaN ];

	let positions:Vec<(u32,u32)> = vec![
		(1,1),(1,7),(7,1),(7,7),
	];

	let mvs:Vec<(u32,u32)> = vec![
		(4,1),(1,4),(7,4),(4,7),
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(4,2),(2,4),(6,4),(4,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = *k;

			banmen.0[op.1 as usize][op.0 as usize] = SKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(!Rule::is_mate(Teban::Sente,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_no_mate_with_kyou_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[3][4] = GOu;
	banmen.0[8][4] = SKyou;
	banmen.0[6][4] = GKin;

	let mut state = State::new(banmen);

	let mc = MochigomaCollections::Empty;

	let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

	match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
		(next,_,_) => {
			state = next;
		}
	}

	assert!(!Rule::is_mate(Teban::Sente,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
}
#[test]
fn test_is_mate_no_mate_with_kaku_occupied_opponent_sente() {
	let kinds:Vec<KomaKind> = vec![ SKaku, SKakuN ];

	let positions:Vec<(u32,u32)> = vec![
		(0,4),(4,0),(4,8),(8,4)
	];

	let mvs:Vec<(u32,u32)> = vec![
		(2,2),(6,2),(6,6),(2,6)
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(3,3),(5,3),(5,5),(3,5)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = *k;

			banmen.0[op.1 as usize][op.0 as usize] = GFu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(!Rule::is_mate(Teban::Sente,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_no_mate_with_hisha_occupied_opponent_sente() {
	let kinds:Vec<KomaKind> = vec![ SHisha, SHishaN ];

	let positions:Vec<(u32,u32)> = vec![
		(1,1),(1,7),(7,1),(7,7),
	];

	let mvs:Vec<(u32,u32)> = vec![
		(4,1),(1,4),(7,4),(4,7),
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(4,2),(2,4),(6,4),(4,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = *k;

			banmen.0[op.1 as usize][op.0 as usize] = GKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(!Rule::is_mate(Teban::Sente,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_to_gote() {
	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ GFu ],
		vec![ GKei ],
		vec![ GGin ],
		vec![ GKin, GFuN, GKyouN, GKeiN, GGinN ],
		vec![ GKaku ],
		vec![ GHisha ],
		vec![ GKakuN ],
		vec![ GHishaN ],
		vec![ GOu ]
	];

	let positions:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,6)],
		vec![(2,8),(6,8)],
		vec![(2,2),(2,6),(4,6),(6,2),(6,6)],
		vec![(2,4),(2,6),(4,2),(4,6),(6,4),(6,6)],
		vec![(0,4),(4,0),(4,8),(8,4)],
		vec![(1,1),(1,7),(7,1),(7,7)],
		vec![(0,4),(4,0),(4,8),(8,4),(2,4),(4,2),(4,6),(6,4)],
		vec![(1,1),(1,7),(7,1),(7,7),(2,2),(2,6),(6,2),(6,6)],
		vec![(2,2),(2,4),(2,6),(4,2),(4,6),(6,2),(6,4),(6,6)]
	];

	let mvs:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,5)],
		vec![(3,6),(5,6)],
		vec![(3,3),(3,5),(4,5),(5,3),(5,5)],
		vec![(3,4),(3,5),(4,3),(4,5),(5,4),(5,5)],
		vec![(2,2),(6,2),(6,6),(2,6)],
		vec![(4,1),(1,4),(7,4),(4,7)],
		vec![(2,2),(6,2),(6,6),(2,6),(3,4),(4,3),(4,5),(5,4)],
		vec![(4,1),(1,4),(7,4),(4,7),(3,3),(3,5),(5,3),(5,5)],
		vec![(3,3),(3,4),(3,5),(4,3),(4,5),(5,3),(5,4),(5,5)],
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for ((p,kinds),mvs) in positions.iter().zip(&kinds).zip(&mvs) {
		for (k,p) in kinds.iter().zip(p) {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[8-3][8-4] = SOu;
				banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

				let mut state = State::new(banmen);

				let mut mc = MochigomaCollections::Empty;

				let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

				match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
					(next,nmc,_) => {
						state = next;
						mc = nmc;
					}
				}

				let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

				match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
					(next,_,_) => {
						state = next;
					}
				}

				assert!(Rule::is_mate(Teban::Gote,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
			}
		}
	}
}
#[test]
fn test_is_mate_put_gote() {
	let kinds:Vec<MochigomaKind> = vec![
		MochigomaKind::Fu,
		MochigomaKind::Kyou,
		MochigomaKind::Kei,
		MochigomaKind::Gin,
		MochigomaKind::Kin,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha,
	];

	let mvs:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,5)],
		vec![(4,8)],
		vec![(3,6),(5,6)],
		vec![(3,3),(3,5),(4,5),(5,3),(5,5)],
		vec![(3,4),(3,5),(4,3),(4,5),(5,4),(5,5)],
		vec![(2,2),(6,2),(6,6),(2,6)],
		vec![(4,1),(1,4),(7,4),(4,7)],
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (k,mvs) in kinds.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::Put(*k,KomaDstPutPosition(9-(8-m.0),(8-m.1)+1));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(Rule::is_mate(Teban::Gote,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_kyou_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[8-3][8-4] = SOu;
	banmen.0[8-8][8-4] = GKyou;
	banmen.0[8-5][8-4] = SKin;

	let mut state = State::new(banmen);

	let mut mc = MochigomaCollections::Empty;

	let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

	match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
		(next,nmc,_) => {
			state = next;
			mc = nmc;
		}
	}

	let mv = Move::To(KomaSrcPosition(9-(8-4),(8-5)+1),KomaDstToPosition(9-(8-3),(8-5)+1,false));

	match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
		(next,_,_) => {
			state = next;
		}
	}

	assert!(Rule::is_mate(Teban::Gote,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
}
#[test]
fn test_is_mate_with_kaku_2step_gote() {
	let kinds:Vec<KomaKind> = vec![ GKaku, GKakuN ];

	let positions:Vec<(u32,u32)> = vec![
		(0,4),(4,0),(4,8),(8,4)
	];

	let mvs:Vec<(u32,u32)> = vec![
		(2,2),(6,2),(6,6),(2,6)
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(3,3),(5,3),(5,5),(3,5)
	];

	let occ_mvs:Vec<(u32,u32)> = vec![
		(3,2),(5,2),(5,6),(3,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),(op,om)) in positions.iter().zip(&mvs).zip(occ_positions.iter().zip(&occ_mvs)) {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;
			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

			banmen.0[8 - op.1 as usize][8 - op.0 as usize] = SKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-op.0),(8-op.1)+1),KomaDstToPosition(9-(8-om.0),(8-om.1)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(Rule::is_mate(Teban::Gote,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_hisha_2step_gote() {
	let kinds:Vec<KomaKind> = vec![ GHisha, GHishaN ];

	let positions:Vec<(u32,u32)> = vec![
		(1,1),(1,7),(7,1),(7,7),
	];

	let mvs:Vec<(u32,u32)> = vec![
		(4,1),(1,4),(7,4),(4,7),
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(4,2),(2,4),(6,4),(4,6)
	];

	let occ_mvs:Vec<(u32,u32)> = vec![
		(3,2),(2,5),(6,6),(3,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),(op,om)) in positions.iter().zip(&mvs).zip(occ_positions.iter().zip(&occ_mvs)) {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;
			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

			banmen.0[8 - op.1 as usize][8 - op.0 as usize] = SKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-op.0),(8-op.1)+1),KomaDstToPosition(9-(8-om.0),(8-om.1)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(Rule::is_mate(Teban::Gote,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_no_mate_to_gote() {
	let kinds:Vec<Vec<KomaKind>> = vec![
		vec![ GFu ],
		vec![ GKei ],
		vec![ GGin ],
		vec![ GKin, GFuN, GKyouN, GKeiN, GGinN ],
		vec![ GKaku ],
		vec![ GHisha ],
		vec![ GKakuN ],
		vec![ GHishaN ],
		vec![ GOu ]
	];

	let positions:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,7)],
		vec![(3,8),(5,8)],
		vec![(2,3),(3,2),(5,2),(6,3)],
		vec![(3,2),(5,2)],
		vec![(2,2),(6,2),(6,6),(2,6)],
		vec![(4,1),(1,4),(7,4),(4,7)],
		vec![(2,2),(6,2),(6,6),(2,6),(3,4),(4,3),(4,5),(5,4)],
		vec![(4,1),(1,4),(7,4),(4,7),(3,3),(3,5),(5,3),(5,5)],
		vec![(1,1),(1,4),(1,7),(4,1),(4,7),(7,1),(7,4),(7,7)]
	];

	let mvs:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,6)],
		vec![(4,6)],
		vec![(3,4),(4,3),(4,3),(5,4)],
		vec![(3,3),(5,3)],
		vec![(0,4),(4,0),(4,8),(8,4)],
		vec![(1,1),(1,7),(7,1),(7,7)],
		vec![(0,4),(4,0),(4,8),(8,4),(2,4),(4,2),(4,6),(6,4)],
		vec![(1,1),(1,7),(7,1),(7,7),(2,2),(2,6),(6,2),(6,6)],
		vec![(2,2),(2,4),(2,6),(4,2),(4,6),(6,2),(6,4),(6,6)],
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for ((p,kinds),mvs) in positions.iter().zip(&kinds).zip(&mvs) {
		for (k,p) in kinds.iter().zip(p) {
			for m in mvs {
				let mut banmen = blank_banmen.clone();

				banmen.0[8-3][8-4] = SOu;
				banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

				let mut state = State::new(banmen);

				let mut mc = MochigomaCollections::Empty;

				let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

				match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
					(next,nmc,_) => {
						state = next;
						mc = nmc;
					}
				}

				let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

				match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
					(next,_,_) => {
						state = next;
					}
				}

				assert!(!Rule::is_mate(Teban::Gote,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
			}
		}
	}
}
#[test]
fn test_is_mate_no_mate_put_gote() {
	let kinds:Vec<MochigomaKind> = vec![
		MochigomaKind::Fu,
		MochigomaKind::Kei,
		MochigomaKind::Gin,
		MochigomaKind::Kin,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha,
	];

	let mvs:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,6)],
		vec![(4,6)],
		vec![(3,4),(4,3),(4,3),(5,4)],
		vec![(3,3),(5,3)],
		vec![(0,4),(4,0),(4,8),(8,4)],
		vec![(1,1),(1,7),(7,1),(7,7)],
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (k,mvs) in kinds.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::Put(*k,KomaDstPutPosition(9-(8-m.0),(8-m.1)+1));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(!Rule::is_mate(Teban::Gote,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_no_mate_with_kyou_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[8-3][8-4] = SOu;
	banmen.0[8-8][8-4] = GKyou;
	banmen.0[8-6][8-4] = GKin;

	let mut state = State::new(banmen);

	let mc = MochigomaCollections::Empty;

	let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

	match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
		(next,_,_) => {
			state = next;
		}
	}

	assert!(!Rule::is_mate(Teban::Gote,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
}
#[test]
fn test_is_mate_no_mate_with_kaku_occupied_self_gote() {
	let kinds:Vec<KomaKind> = vec![ GKaku, GKakuN ];

	let positions:Vec<(u32,u32)> = vec![
		(0,4),(4,0),(4,8),(8,4)
	];

	let mvs:Vec<(u32,u32)> = vec![
		(2,2),(6,2),(6,6),(2,6)
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(3,3),(5,3),(5,5),(3,5)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;
			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

			banmen.0[8 - op.1 as usize][8 - op.0 as usize] = GFu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(!Rule::is_mate(Teban::Gote,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_no_mate_with_hisha_occupied_self_gote() {
	let kinds:Vec<KomaKind> = vec![ GHisha, GHishaN ];

	let positions:Vec<(u32,u32)> = vec![
		(1,1),(1,7),(7,1),(7,7),
	];

	let mvs:Vec<(u32,u32)> = vec![
		(4,1),(1,4),(7,4),(4,7),
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(4,2),(2,4),(6,4),(4,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;
			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

			banmen.0[8 - op.1 as usize][8 - op.0 as usize] = GKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(!Rule::is_mate(Teban::Gote,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_no_mate_with_kyou_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[8-3][8-4] = SOu;
	banmen.0[8-8][8-4] = GKyou;
	banmen.0[8-6][8-4] = SKin;

	let mut state = State::new(banmen);

	let mc = MochigomaCollections::Empty;

	let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

	match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
		(next,_,_) => {
			state = next;
		}
	}

	assert!(!Rule::is_mate(Teban::Gote,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
}
#[test]
fn test_is_mate_no_mate_with_kaku_occupied_opponent_gote() {
	let kinds:Vec<KomaKind> = vec![ GKaku, GKakuN ];

	let positions:Vec<(u32,u32)> = vec![
		(0,4),(4,0),(4,8),(8,4)
	];

	let mvs:Vec<(u32,u32)> = vec![
		(2,2),(6,2),(6,6),(2,6)
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(3,3),(5,3),(5,5),(3,5)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;
			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

			banmen.0[8 - op.1 as usize][8 - op.0 as usize] = SFu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(!Rule::is_mate(Teban::Gote,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_no_mate_with_hisha_occupied_opponent_gote() {
	let kinds:Vec<KomaKind> = vec![ GHisha, GHishaN ];

	let positions:Vec<(u32,u32)> = vec![
		(1,1),(1,7),(7,1),(7,7),
	];

	let mvs:Vec<(u32,u32)> = vec![
		(4,1),(1,4),(7,4),(4,7),
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(4,2),(2,4),(6,4),(4,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;
			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

			banmen.0[8 - op.1 as usize][8 - op.0 as usize] = SKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			assert!(!Rule::is_mate(Teban::Gote,&state),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_with_kyou_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[3][4] = GOu;
	banmen.0[5][4] = GKin;

	let mut state = State::new(banmen);

	let mut mc = MochigomaCollections::Empty;

	let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

	match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
		(next,nmc,_) => {
			state = next;
			mc = nmc;
		}
	}

	let mv = Move::To(KomaSrcPosition(9-4,5+1),KomaDstToPosition(9-3,5+1,false));

	match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
		(next,nmc,_) => {
			state = next;
			mc = nmc;
		}
	}

	let mv = Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-4,8+1));

	let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Sente,&mc,mv.to_applied_move());

	assert!(Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Sente,&ps),
						"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_with_kaku_sente() {
	let kinds:Vec<KomaKind> = vec![ SKaku, SKakuN ];

	let positions:Vec<(u32,u32)> = vec![
		(0,4),(4,0),(4,8),(8,4)
	];

	let mvs:Vec<(u32,u32)> = vec![
		(2,2),(6,2),(6,6),(2,6)
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(3,3),(5,3),(5,5),(3,5)
	];

	let occ_mvs:Vec<(u32,u32)> = vec![
		(3,2),(5,2),(5,6),(3,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),(op,om)) in positions.iter().zip(&mvs).zip(occ_positions.iter().zip(&occ_mvs)) {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = *k;

			banmen.0[op.1 as usize][op.0 as usize] = GKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-op.0,op.1+1),KomaDstToPosition(9-om.0,om.1+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Sente,&mc,mv.to_applied_move());

			assert!(Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Sente,&ps),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_with_hisha_sente() {
	let kinds:Vec<KomaKind> = vec![ SHisha, SHishaN ];

	let positions:Vec<(u32,u32)> = vec![
		(1,1),(1,7),(7,1),(7,7),
	];

	let mvs:Vec<(u32,u32)> = vec![
		(4,1),(1,4),(7,4),(4,7),
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(4,2),(2,4),(6,4),(4,6)
	];

	let occ_mvs:Vec<(u32,u32)> = vec![
		(3,2),(2,5),(6,6),(3,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),(op,om)) in positions.iter().zip(&mvs).zip(occ_positions.iter().zip(&occ_mvs)) {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = *k;

			banmen.0[op.1 as usize][op.0 as usize] = GKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-op.0,op.1+1),KomaDstToPosition(9-om.0,om.1+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Sente,&mc,mv.to_applied_move());

			assert!(Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Sente,&ps),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_put_sente() {
	let kinds:Vec<MochigomaKind> = vec![
		MochigomaKind::Kyou,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha,
	];

	let mvs:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,8)],
		vec![(2,2),(6,2),(6,6),(2,6)],
		vec![(4,1),(1,4),(7,4),(4,7)],
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (k,mvs) in kinds.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::Put(*k,KomaDstPutPosition(9-m.0,m.1+1));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Sente,&mc,mv.to_applied_move());

			assert!(Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Sente,&ps),
								"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_no_mate_with_kyou_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[3][4] = GOu;
	banmen.0[6][4] = SKin;

	let mut state = State::new(banmen);

	let mut mc = MochigomaCollections::Empty;

	let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

	match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
		(next,nmc,_) => {
			state = next;
			mc = nmc;
		}
	}

	let mv = Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-4,8+1));

	let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Sente,&mc,mv.to_applied_move());

	assert!(!Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Sente,&ps),
						"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_no_mate_with_kaku_occupied_self_sente() {
	let kinds:Vec<KomaKind> = vec![ SKaku, SKakuN ];

	let positions:Vec<(u32,u32)> = vec![
		(0,4),(4,0),(4,8),(8,4)
	];

	let mvs:Vec<(u32,u32)> = vec![
		(2,2),(6,2),(6,6),(2,6)
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(3,3),(5,3),(5,5),(3,5)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = *k;

			banmen.0[op.1 as usize][op.0 as usize] = SFu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Sente,&mc,mv.to_applied_move());

			assert!(!Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Sente,&ps),
								"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_apply_move_to_partial_state_none_check_no_mate_with_hisha_occupied_self_sente() {
	let kinds:Vec<KomaKind> = vec![ SHisha, SHishaN ];

	let positions:Vec<(u32,u32)> = vec![
		(1,1),(1,7),(7,1),(7,7),
	];

	let mvs:Vec<(u32,u32)> = vec![
		(4,1),(1,4),(7,4),(4,7),
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(4,2),(2,4),(6,4),(4,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = *k;

			banmen.0[op.1 as usize][op.0 as usize] = SKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Sente,&mc,mv.to_applied_move());

			assert!(!Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Sente,&ps),
								"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_no_mate_with_kyou_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[3][4] = GOu;
	banmen.0[6][4] = GKin;

	let mut state = State::new(banmen);

	let mut mc = MochigomaCollections::Empty;

	let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

	match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
		(next,nmc,_) => {
			state = next;
			mc = nmc;
		}
	}

	let mv = Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-4,8+1));

	let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Sente,&mc,mv.to_applied_move());

	assert!(!Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Sente,&ps),
						"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_no_mate_with_kaku_occupied_opponent_sente() {
	let kinds:Vec<KomaKind> = vec![ SKaku, SKakuN ];

	let positions:Vec<(u32,u32)> = vec![
		(0,4),(4,0),(4,8),(8,4)
	];

	let mvs:Vec<(u32,u32)> = vec![
		(2,2),(6,2),(6,6),(2,6)
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(3,3),(5,3),(5,5),(3,5)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = *k;

			banmen.0[op.1 as usize][op.0 as usize] = GFu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Sente,&mc,mv.to_applied_move());

			assert!(!Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Sente,&ps),
								"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_no_mate_with_hisha_occupied_opponent_sente() {
	let kinds:Vec<KomaKind> = vec![ SHisha, SHishaN ];

	let positions:Vec<(u32,u32)> = vec![
		(1,1),(1,7),(7,1),(7,7),
	];

	let mvs:Vec<(u32,u32)> = vec![
		(4,1),(1,4),(7,4),(4,7),
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(4,2),(2,4),(6,4),(4,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[3][4] = GOu;
			banmen.0[p.1 as usize][p.0 as usize] = *k;

			banmen.0[op.1 as usize][op.0 as usize] = GKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-4,3+1),KomaDstToPosition(9-4,4+1,false));

			match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-m.0,m.1+1,false));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Sente,&mc,mv.to_applied_move());

			assert!(!Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Sente,&ps),
								"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_with_kyou_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[8-3][8-4] = SOu;
	banmen.0[8-5][8-4] = SKin;

	let mut state = State::new(banmen);

	let mut mc = MochigomaCollections::Empty;

	let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

	match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
		(next,nmc,_) => {
			state = next;
			mc = nmc;
		}
	}

	let mv = Move::To(KomaSrcPosition(9-(8-4),(8-5)+1),KomaDstToPosition(9-(8-3),(8-5)+1,false));

	match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
		(next,nmc,_) => {
			state = next;
			mc = nmc;
		}
	}

	let mv = Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-(8-4),(8-8)+1));

	let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Gote,&mc,mv.to_applied_move());

	assert!(Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Gote,&ps),
						"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_with_kaku_gote() {
	let kinds:Vec<KomaKind> = vec![ GKaku, GKakuN ];

	let positions:Vec<(u32,u32)> = vec![
		(0,4),(4,0),(4,8),(8,4)
	];

	let mvs:Vec<(u32,u32)> = vec![
		(2,2),(6,2),(6,6),(2,6)
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(3,3),(5,3),(5,5),(3,5)
	];

	let occ_mvs:Vec<(u32,u32)> = vec![
		(3,2),(5,2),(5,6),(3,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),(op,om)) in positions.iter().zip(&mvs).zip(occ_positions.iter().zip(&occ_mvs)) {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;
			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

			banmen.0[8 - op.1 as usize][8 - op.0 as usize] = SKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-op.0),(8-op.1)+1),KomaDstToPosition(9-(8-om.0),(8-om.1)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Gote,&mc,mv.to_applied_move());

			assert!(Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Gote,&ps),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_with_hisha_gote() {
	let kinds:Vec<KomaKind> = vec![ GHisha, GHishaN ];

	let positions:Vec<(u32,u32)> = vec![
		(1,1),(1,7),(7,1),(7,7),
	];

	let mvs:Vec<(u32,u32)> = vec![
		(4,1),(1,4),(7,4),(4,7),
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(4,2),(2,4),(6,4),(4,6)
	];

	let occ_mvs:Vec<(u32,u32)> = vec![
		(3,2),(2,5),(6,6),(3,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),(op,om)) in positions.iter().zip(&mvs).zip(occ_positions.iter().zip(&occ_mvs)) {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;
			banmen.0[8-p.1 as usize][8-p.0 as usize] = *k;

			banmen.0[8-op.1 as usize][8-op.0 as usize] = SKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-op.0),(8-op.1)+1),KomaDstToPosition(9-(8-om.0),(8-om.1)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,_,_) => {
					state = next;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Gote,&mc,mv.to_applied_move());

			assert!(Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Gote,&ps),"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_put_gote() {
	let kinds:Vec<MochigomaKind> = vec![
		MochigomaKind::Kyou,
		MochigomaKind::Kaku,
		MochigomaKind::Hisha,
	];

	let mvs:Vec<Vec<(u32,u32)>> = vec![
		vec![(4,8)],
		vec![(2,2),(6,2),(6,6),(2,6)],
		vec![(4,1),(1,4),(7,4),(4,7)],
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for (k,mvs) in kinds.iter().zip(&mvs) {
		for m in mvs {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::Put(*k,KomaDstPutPosition(9-(8-m.0),(8-m.1)+1));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Gote,&mc,mv.to_applied_move());

			assert!(Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Gote,&ps),
								"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_no_mate_with_kyou_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[8-3][8-4] = SOu;
	banmen.0[8-6][8-4] = GKin;

	let mut state = State::new(banmen);

	let mut mc = MochigomaCollections::Empty;

	let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

	match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
		(next,nmc,_) => {
			state = next;
			mc = nmc;
		}
	}

	let mv = Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-(8-4),(8-8)+1));

	let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Gote,&mc,mv.to_applied_move());

	assert!(!Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Gote,&ps),
						"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_no_mate_with_kaku_occupied_self_gote() {
	let kinds:Vec<KomaKind> = vec![ GKaku, GKakuN ];

	let positions:Vec<(u32,u32)> = vec![
		(0,4),(4,0),(4,8),(8,4)
	];

	let mvs:Vec<(u32,u32)> = vec![
		(2,2),(6,2),(6,6),(2,6)
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(3,3),(5,3),(5,5),(3,5)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;
			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

			banmen.0[8 - op.1 as usize][8 - op.0 as usize] = GFu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Gote,&mc,mv.to_applied_move());

			assert!(!Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Gote,&ps),
								"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_apply_move_to_partial_state_none_check_no_mate_with_hisha_occupied_self_gote() {
	let kinds:Vec<KomaKind> = vec![ GHisha, GHishaN ];

	let positions:Vec<(u32,u32)> = vec![
		(1,1),(1,7),(7,1),(7,7),
	];

	let mvs:Vec<(u32,u32)> = vec![
		(4,1),(1,4),(7,4),(4,7),
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(4,2),(2,4),(6,4),(4,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;
			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

			banmen.0[8 - op.1 as usize][8 - op.0 as usize] = GKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Gote,&mc,mv.to_applied_move());

			assert!(!Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Gote,&ps),
								"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_no_mate_with_kyou_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let mut banmen = blank_banmen.clone();

	banmen.0[8-3][8-4] = SOu;
	banmen.0[8-6][8-4] = SKin;

	let mut state = State::new(banmen);

	let mut mc = MochigomaCollections::Empty;

	let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

	match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
		(next,nmc,_) => {
			state = next;
			mc = nmc;
		}
	}

	let mv = Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-(8-4),(8-8)+1));

	let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Gote,&mc,mv.to_applied_move());

	assert!(!Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Gote,&ps),
						"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_no_mate_with_kaku_occupied_opponent_gote() {
	let kinds:Vec<KomaKind> = vec![ GKaku, GKakuN ];

	let positions:Vec<(u32,u32)> = vec![
		(0,4),(4,0),(4,8),(8,4)
	];

	let mvs:Vec<(u32,u32)> = vec![
		(2,2),(6,2),(6,6),(2,6)
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(3,3),(5,3),(5,5),(3,5)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;
			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

			banmen.0[8 - op.1 as usize][8 - op.0 as usize] = SFu;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Gote,&mc,mv.to_applied_move());

			assert!(!Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Gote,&ps),
								"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_partial_state_repeat_move_kinds_no_mate_with_hisha_occupied_opponent_gote() {
	let kinds:Vec<KomaKind> = vec![ GHisha, GHishaN ];

	let positions:Vec<(u32,u32)> = vec![
		(1,1),(1,7),(7,1),(7,7),
	];

	let mvs:Vec<(u32,u32)> = vec![
		(4,1),(1,4),(7,4),(4,7),
	];

	let occ_positions:Vec<(u32,u32)> = vec![
		(4,2),(2,4),(6,4),(4,6)
	];

	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for k in &kinds {
		for ((p,m),op) in positions.iter().zip(&mvs).zip(&occ_positions) {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-3][8-4] = SOu;
			banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;

			banmen.0[8 - op.1 as usize][8 - op.0 as usize] = SKin;

			let mut state = State::new(banmen);

			let mut mc = MochigomaCollections::Empty;

			let mv = Move::To(KomaSrcPosition(9-(8-4),(8-3)+1),KomaDstToPosition(9-(8-4),(8-4)+1,false));

			match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mv.to_applied_move()) {
				(next,nmc,_) => {
					state = next;
					mc = nmc;
				}
			}

			let mv = Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-m.0),(8-m.1)+1,false));

			let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Gote,&mc,mv.to_applied_move());

			assert!(!Rule::is_mate_with_partial_state_repeat_move_kinds(Teban::Gote,&ps),
								"assertion failed, move = {:?}, {:?}",mv,state.get_banmen());
		}
	}
}
#[test]
fn test_is_mate_with_partial_state_and_old_banmen_and_opponent_move_sente() {
	let mate_mvs:Vec<Move> = vec![
		Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-4,8+1)),
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,1+1)),
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,2+1)),
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,1+1)),
	];

	let mvs:Vec<Move>  = vec![
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,1+1)),
		Move::To(KomaSrcPosition(9-5,0+1),KomaDstToPosition(9-4,1+1,false)),
		Move::To(KomaSrcPosition(9-3,0+1),KomaDstToPosition(9-3,1+1,false)),
		Move::To(KomaSrcPosition(9-3,0+1),KomaDstToPosition(9-3,1+1,false))
	];

	let answer:Vec<bool> = vec![
		false,
		false,
		false,
		true
	];

	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[0][3] = GKin;
	banmen.0[0][4] = GOu;
	banmen.0[0][5] = GKin;

	for ((m,answer),mm) in mvs.iter().zip(&answer).zip(&mate_mvs) {
		let mut state = State::new(banmen.clone());

		let mut ms:HashMap<MochigomaKind,u32> = HashMap::new();
		let mg:HashMap<MochigomaKind,u32> = HashMap::new();

		ms.insert(MochigomaKind::Fu, 1);

		let mut mc = MochigomaCollections::Pair(ms,mg);

		match Rule::apply_move_none_check(&state,Teban::Sente,&mc,mm.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Gote,&mc,m.to_applied_move());

		assert_eq!(*answer,
					Rule::is_mate_with_partial_state_and_old_banmen_and_opponent_move(
						Teban::Sente,state.get_banmen(),&ps,m.to_applied_move()));
	}
}
#[test]
fn test_is_mate_with_partial_state_and_old_banmen_and_opponent_move_gote() {
	let mate_mvs:Vec<Move> = vec![
		Move::Put(MochigomaKind::Kyou,KomaDstPutPosition(9-(8-4),(8-8)+1)),
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-(8-4),(8-1)+1)),
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-(8-4),(8-2)+1)),
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-(8-4),(8-1)+1)),
	];

	let mvs:Vec<Move>  = vec![
		Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-(8-4),(8-1)+1)),
		Move::To(KomaSrcPosition(9-(8-5),(8-0)+1),KomaDstToPosition(9-(8-4),(8-1)+1,false)),
		Move::To(KomaSrcPosition(9-(8-3),(8-0)+1),KomaDstToPosition(9-(8-3),(8-1)+1,false)),
		Move::To(KomaSrcPosition(9-(8-3),(8-0)+1),KomaDstToPosition(9-(8-3),(8-1)+1,false))
	];

	let answer:Vec<bool> = vec![
		false,
		false,
		false,
		true
	];

	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[8-0][8-3] = SKin;
	banmen.0[8-0][8-4] = SOu;
	banmen.0[8-0][8-5] = SKin;

	for ((m,answer),mm) in mvs.iter().zip(&answer).zip(&mate_mvs) {
		let mut state = State::new(banmen.clone());

		let ms:HashMap<MochigomaKind,u32> = HashMap::new();
		let mut mg:HashMap<MochigomaKind,u32> = HashMap::new();

		mg.insert(MochigomaKind::Fu, 1);

		let mut mc = MochigomaCollections::Pair(ms,mg);

		match Rule::apply_move_none_check(&state,Teban::Gote,&mc,mm.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
			}
		}

		let ps = Rule::apply_move_to_partial_state_none_check(&state,Teban::Sente,&mc,m.to_applied_move());

		assert_eq!(*answer,
					Rule::is_mate_with_partial_state_and_old_banmen_and_opponent_move(
						Teban::Gote,state.get_banmen(),&ps,m.to_applied_move()));
	}
}
#[test]
fn test_is_sennichite_sente() {
	let mvs:Vec<((u32,u32),(u32,u32))> = vec![
		((7,7),(4,7)),((4,7),(7,7)),((7,7),(4,7)),((4,7),(7,7)),((7,7),(4,7))
	];

	let mut kyokumen_map:KyokumenMap<u64,u32> = KyokumenMap::new();
	let hasher = KyokumenHash::new();

	let (mut mhash, mut shash) = hasher.calc_initial_hash(&BANMEN_START_POS,&HashMap::new(),&HashMap::new());

	let mut state = State::new(BANMEN_START_POS.clone());

	assert!(!Rule::is_sennichite(&state,Teban::Sente,mhash,shash,&kyokumen_map));

	Rule::update_sennichite_map(&state,Teban::Sente,mhash,shash,&mut kyokumen_map);

	let mvs = mvs.into_iter().map(|m| {
		rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(m.0).0,(m.0).1+1),KomaDstToPosition(9-(m.1).0,(m.1).1+1,false)))
	}).collect::<Vec<rule::AppliedMove>>();

	let teban = Teban::Sente;
	let mut mc = MochigomaCollections::Empty;

	for m in mvs {
		match Rule::apply_move_none_check(&state,teban,&mc,m) {
			(next,nmc,o) => {
				mhash = hasher.calc_main_hash(mhash,teban,state.get_banmen(),&mc,m,&o);
				shash = hasher.calc_sub_hash(shash,teban,state.get_banmen(),&mc,m,&o);

				mc = nmc;
				state = next;

				assert!(!Rule::is_sennichite(&state,teban,mhash,shash,&kyokumen_map));

				Rule::update_sennichite_map(&state,teban,mhash,shash,&mut kyokumen_map);
			}
		}
	}

	let m = Move::To(KomaSrcPosition(9-4,7+1),KomaDstToPosition(9-7,7+1,false)).to_applied_move();

	match Rule::apply_move_none_check(&state,Teban::Sente,&mc,m) {
		(_,_,_) => {
			mhash = hasher.calc_main_hash(mhash,Teban::Sente,state.get_banmen(),&mc,m,&None);
			shash = hasher.calc_sub_hash(shash,Teban::Sente,state.get_banmen(),&mc,m,&None);
		}
	}

	assert!(Rule::is_sennichite(&state,Teban::Sente,mhash,shash,&kyokumen_map));
}
#[test]
fn test_is_sennichite_gote() {
	let mvs:Vec<((u32,u32),(u32,u32))> = vec![
		((7,7),(4,7)),((4,7),(7,7)),((7,7),(4,7)),((4,7),(7,7)),((7,7),(4,7))
	];

	let mut kyokumen_map:KyokumenMap<u64,u32> = KyokumenMap::new();
	let hasher = KyokumenHash::new();

	let (mut mhash, mut shash) = hasher.calc_initial_hash(&BANMEN_START_POS,&HashMap::new(),&HashMap::new());

	let mut state = State::new(BANMEN_START_POS.clone());

	assert!(!Rule::is_sennichite(&state,Teban::Gote,mhash,shash,&kyokumen_map));
	Rule::update_sennichite_map(&state,Teban::Gote,mhash,shash,&mut kyokumen_map);

	let mvs = mvs.into_iter().map(|m| {
		rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-(m.0).0),(8-(m.0).1)+1),KomaDstToPosition(9-(8-(m.1).0),(8-(m.1).1)+1,false)))
	}).collect::<Vec<rule::AppliedMove>>();

	let teban = Teban::Gote;
	let mut mc = MochigomaCollections::Empty;

	for m in mvs {
		match Rule::apply_move_none_check(&state,teban,&mc,m) {
			(next,nmc,o) => {
				mhash = hasher.calc_main_hash(mhash,teban,state.get_banmen(),&mc,m,&o);
				shash = hasher.calc_sub_hash(shash,teban,state.get_banmen(),&mc,m,&o);

				mc = nmc;
				state = next;

				assert!(!Rule::is_sennichite(&state,teban,mhash,shash,&kyokumen_map));

				Rule::update_sennichite_map(&state,teban,mhash,shash,&mut kyokumen_map)
			}
		}
	}

	let m = Move::To(KomaSrcPosition(9-(8-4),(8-7)+1),KomaDstToPosition(9-(8-7),(8-7)+1,false)).to_applied_move();

	match Rule::apply_move_none_check(&state,Teban::Gote,&mc,m) {
		(_,_,_) => {
			mhash = hasher.calc_main_hash(mhash,Teban::Gote,state.get_banmen(),&mc,m,&None);
			shash = hasher.calc_sub_hash(shash,Teban::Gote,state.get_banmen(),&mc,m,&None);
		}
	}

	assert!(Rule::is_sennichite(&state,Teban::Gote,mhash,shash,&kyokumen_map));
}
#[test]
fn test_is_sennichite_by_oute_sente() {
	let mvs:Vec<((u32,u32),(u32,u32))> = vec![
		((4,8),(5,8)),((4,0),(5,0)),((5,8),(4,8))
	];

	let hasher = KyokumenHash::new();

	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[8][4] = GOu;
	banmen.0[0][8] = SHisha;

	let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&HashMap::new(),&HashMap::new());

	let mut state = State::new(banmen);
	let mut teban = Teban::Sente;
	let mut mc = MochigomaCollections::Empty;

	let mut kyokumen_map:KyokumenMap<u64,u32> = KyokumenMap::new();

	let m = Move::To(KomaSrcPosition(9-8,0+1),KomaDstToPosition(9-4,0+1,false)).to_applied_move();

	match Rule::apply_move_none_check(&state,teban,&mc,m) {
		(next,nmc,_) => {
			mhash = hasher.calc_main_hash(mhash,teban,state.get_banmen(),&mc,m,&None);
			shash = hasher.calc_sub_hash(shash,teban,state.get_banmen(),&mc,m,&None);

			state = next;
			mc = nmc;

			assert!(!Rule::is_sennichite_by_oute(&state,teban,mhash,shash,&kyokumen_map));

			Rule::update_sennichite_by_oute_map(&state,teban,mhash,shash,&mut kyokumen_map);

			teban = teban.opposite();
		}
	}

	let mvs = mvs.into_iter().map(|m| {
		rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(m.0).0,(m.0).1+1),KomaDstToPosition(9-(m.1).0,(m.1).1+1,false)))
	}).collect::<Vec<rule::AppliedMove>>();

	for m in mvs {
		match Rule::apply_move_none_check(&state,teban,&mc,m) {
			(next,nmc,o) => {
				mhash = hasher.calc_main_hash(mhash,teban,state.get_banmen(),&mc,m,&o);
				shash = hasher.calc_sub_hash(shash,teban,state.get_banmen(),&mc,m,&o);

				mc = nmc;
				state = next;

				Rule::update_sennichite_by_oute_map(&state,teban,mhash,shash,&mut kyokumen_map);

				teban = teban.opposite();
			}
		}
	}

	let m = Move::To(KomaSrcPosition(9-5,0+1),KomaDstToPosition(9-4,0+1,false)).to_applied_move();

	match Rule::apply_move_none_check(&state,teban,&mc,m) {
		(next,_,o) => {
			mhash = hasher.calc_main_hash(mhash,teban,state.get_banmen(),&mc,m,&o);
			shash = hasher.calc_sub_hash(shash,teban,state.get_banmen(),&mc,m,&o);

			state = next;

			Rule::update_sennichite_by_oute_map(&state,teban,mhash,shash,&mut kyokumen_map);
		}
	}

	assert!(Rule::is_sennichite_by_oute(&state,Teban::Sente,mhash,shash,&kyokumen_map));
}
#[test]
fn test_is_sennichite_by_oute_gote() {
	let mvs:Vec<((u32,u32),(u32,u32))> = vec![
		((4,8),(5,8)),((4,0),(5,0)),((5,8),(4,8))
	];

	let hasher = KyokumenHash::new();

	let mut banmen = Banmen([[Blank; 9]; 9]);

	banmen.0[8-8][8-4] = SOu;
	banmen.0[8-0][8-8] = GHisha;

	let (mut mhash, mut shash) = hasher.calc_initial_hash(&banmen,&HashMap::new(),&HashMap::new());

	let mut state = State::new(banmen);
	let mut teban = Teban::Gote;
	let mut mc = MochigomaCollections::Empty;

	let mut kyokumen_map:KyokumenMap<u64,u32> = KyokumenMap::new();

	let m = Move::To(KomaSrcPosition(9-(8-8),(8-0)+1),KomaDstToPosition(9-(8-4),(8-0)+1,false)).to_applied_move();

	match Rule::apply_move_none_check(&state,teban,&mc,m) {
		(next,nmc,_) => {
			mhash = hasher.calc_main_hash(mhash,teban,state.get_banmen(),&mc,m,&None);
			shash = hasher.calc_sub_hash(shash,teban,state.get_banmen(),&mc,m,&None);

			state = next;
			mc = nmc;

			assert!(!Rule::is_sennichite_by_oute(&state,teban,mhash,shash,&kyokumen_map));

			Rule::update_sennichite_by_oute_map(&state,teban,mhash,shash,&mut kyokumen_map);
			teban = teban.opposite();
		}
	}

	let mvs = mvs.into_iter().map(|m| {
		rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-(m.0).0),(8-(m.0).1)+1),KomaDstToPosition(9-(8-(m.1).0),(8-(m.1).1)+1,false)))
	}).collect::<Vec<rule::AppliedMove>>();

	for m in mvs {
		match Rule::apply_move_none_check(&state,teban,&mc,m) {
			(next,nmc,o) => {
				mhash = hasher.calc_main_hash(mhash,teban,state.get_banmen(),&mc,m,&o);
				shash = hasher.calc_sub_hash(shash,teban,state.get_banmen(),&mc,m,&o);

				mc = nmc;
				state = next;

				Rule::update_sennichite_by_oute_map(&state,teban,mhash,shash,&mut kyokumen_map);

				teban = teban.opposite();
			}
		}
	}

	let m = Move::To(KomaSrcPosition(9-(8-5),(8-0)+1),KomaDstToPosition(9-(8-4),(8-0)+1,false)).to_applied_move();

	match Rule::apply_move_none_check(&state,teban,&mc,m) {
		(next,_,o) => {
			mhash = hasher.calc_main_hash(mhash,teban,state.get_banmen(),&mc,m,&o);
			shash = hasher.calc_sub_hash(shash,teban,state.get_banmen(),&mc,m,&o);

			state = next;

			Rule::update_sennichite_by_oute_map(&state,teban,mhash,shash,&mut kyokumen_map);
		}
	}

	assert!(Rule::is_sennichite_by_oute(&state,Teban::Gote,mhash,shash,&kyokumen_map));
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
										kind != KomaKind::SKin && (dy <= 2 || y <= 2) {

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
										kind != KomaKind::SKin && (dy <= 2 || y <= 2) {

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
										kind != KomaKind::SKin && (dy <= 2 || y <= 2) {

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
										kind != KomaKind::SKin && (dy <= 2 || y <= 2) {

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
										kind != KomaKind::GKin && (dy >= 6 || y >= 6) {

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
										kind != KomaKind::GKin && (dy >= 6 || y >= 6) {

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
										kind != KomaKind::GKin && (dy >= 6 || y >= 6) {

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
										kind != KomaKind::GKin && (dy >= 6 || y >= 6) {

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

					if ty < 3 || y < 3 {
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

					if ty < 3 || y < 3 {
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

					if ty >= 6 || y >= 6 {
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

					if ty >= 6 || y >= 6 {
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
				mhash = hasher.calc_main_hash(mhash,teban,&banmen,&mc,m,&o);
				shash = hasher.calc_sub_hash(shash,teban,&banmen,&mc,m,&o);

				mc = nmc;
				teban = teban.opposite();
				banmen = next;

				match kyokumen_hash_map.get(&mhash,&shash) {
					Some(&c) => {
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
		Some(&c) if c >= 3 => {
			return false;
		},
		Some(&c) => {
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