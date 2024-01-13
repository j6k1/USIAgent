use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use usiagent::math::Prng;
use usiagent::movepick::RandomPicker;
use usiagent::shogi::*;
use usiagent::shogi::MochigomaCollections;
use usiagent::rule;
use usiagent::rule::{NonEvasionsAll, Rule};
use usiagent::rule::State;

use super::*;

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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SFu;

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Sente,&banmen,a),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-p.1][8-p.0] = GFu;

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Gote,&banmen,a),
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
		sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
					Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
						LegalMove::from(m)
					}).collect::<Vec<LegalMove>>()
				);
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_all_position_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			for ox in 0..9 {
				for oy in 0..9 {
					if ox == x && oy == y {
						continue;
					}
					let mut banmen = blank_banmen.clone();
					banmen.0[y][x] = SKaku;
					banmen.0[ox][oy] = SFu;

					assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
							   Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
								   LegalMove::from(m)
							   }).collect::<Vec<LegalMove>>()
					);
				}
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_all_position_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			for ox in 0..9 {
				for oy in 0..9 {
					if ox == x && oy == y {
						continue;
					}
					let mut banmen = blank_banmen.clone();

					banmen.0[8-y][8-x] = GKaku;
					banmen.0[8-ox][8-oy] = GFu;

					assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
							   Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
								   LegalMove::from(m)
							   }).collect::<Vec<LegalMove>>()
					);
				}
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_all_position_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			for ox in 0..9 {
				for oy in 0..9 {
					if ox == x && oy == y {
						continue;
					}
					let mut banmen = blank_banmen.clone();
					banmen.0[y][x] = SKaku;
					banmen.0[ox][oy] = GFu;

					assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
							   Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
								   LegalMove::from(m)
							   }).collect::<Vec<LegalMove>>()
					);
				}
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kaku_all_position_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			for ox in 0..9 {
				for oy in 0..9 {
					if ox == x && oy == y {
						continue;
					}
					let mut banmen = blank_banmen.clone();

					banmen.0[8-y][8-x] = GKaku;
					banmen.0[8-ox][8-oy] = SFu;

					assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
							   Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
								   LegalMove::from(m)
							   }).collect::<Vec<LegalMove>>()
					);
				}
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

	assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

	assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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
			sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

				assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

	assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

	assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_all_position_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			let mut banmen = blank_banmen.clone();
			banmen.0[y][x] = SHisha;

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
					   Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
						   LegalMove::from(m)
					   }).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_all_position_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-y][8-x] = GHisha;

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
					   Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
						   LegalMove::from(m)
					   }).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_all_position_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			for ox in 0..9 {
				for oy in 0..9 {
					if ox == x && oy == y {
						continue;
					}
					let mut banmen = blank_banmen.clone();
					banmen.0[y][x] = SHisha;
					banmen.0[ox][oy] = SFu;

					assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
							   Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
								   LegalMove::from(m)
							   }).collect::<Vec<LegalMove>>()
					);
				}
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_all_position_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			for ox in 0..9 {
				for oy in 0..9 {
					if ox == x && oy == y {
						continue;
					}
					let mut banmen = blank_banmen.clone();

					banmen.0[8-y][8-x] = GHisha;
					banmen.0[8-ox][8-oy] = GFu;

					assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
							   Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
								   LegalMove::from(m)
							   }).collect::<Vec<LegalMove>>()
					);
				}
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_all_position_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			for ox in 0..9 {
				for oy in 0..9 {
					if ox == x && oy == y {
						continue;
					}
					let mut banmen = blank_banmen.clone();
					banmen.0[y][x] = SHisha;
					banmen.0[ox][oy] = GFu;

					assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
							   Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
								   LegalMove::from(m)
							   }).collect::<Vec<LegalMove>>()
					);
				}
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_hisha_all_position_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in 0..9 {
		for y in 0..9 {
			for ox in 0..9 {
				for oy in 0..9 {
					if ox == x && oy == y {
						continue;
					}
					let mut banmen = blank_banmen.clone();

					banmen.0[8-y][8-x] = GHisha;
					banmen.0[8-ox][8-oy] = SFu;

					assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
							   Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
								   LegalMove::from(m)
							   }).collect::<Vec<LegalMove>>()
					);
				}
			}
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

	assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

	assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

		assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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
			sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
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

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
				Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
					LegalMove::from(m)
				}).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kyou_all_vertical_position_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in [0,6,7] {
		for y in 0..9 {
			let mut banmen = blank_banmen.clone();
			banmen.0[y][x] = SKyou;

			assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
					   Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
						   LegalMove::from(m)
					   }).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kyou_all_vertical_position_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in [0,6,7] {
		for y in 0..9 {
			let mut banmen = blank_banmen.clone();

			banmen.0[8-y][8-x] = GKyou;

			assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
					   Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
						   LegalMove::from(m)
					   }).collect::<Vec<LegalMove>>()
			);
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kyou_all_vertical_position_occupied_self_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in [0,6,7] {
		for y in 0..9 {
			for ox in 0..9 {
				for oy in 0..9 {
					if ox == x && oy == y {
						continue;
					}
					let mut banmen = blank_banmen.clone();
					banmen.0[y][x] = SKyou;
					banmen.0[ox][oy] = SFu;

					assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
							   Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
								   LegalMove::from(m)
							   }).collect::<Vec<LegalMove>>()
					);
				}
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kyou_all_vertical_position_occupied_self_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in [0,6,7] {
		for y in 0..9 {
			for ox in 0..9 {
				for oy in 0..9 {
					if ox == x && oy == y {
						continue;
					}
					let mut banmen = blank_banmen.clone();

					banmen.0[8-y][8-x] = GKyou;
					banmen.0[8-ox][8-oy] = GFu;

					assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
							   Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
								   LegalMove::from(m)
							   }).collect::<Vec<LegalMove>>()
					);
				}
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kyou_all_vertical_position_occupied_opponent_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in [0,6,7] {
		for y in 0..9 {
			for ox in 0..9 {
				for oy in 0..9 {
					if ox == x && oy == y {
						continue;
					}
					let mut banmen = blank_banmen.clone();
					banmen.0[y][x] = SKyou;
					banmen.0[ox][oy] = GFu;

					assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_from_banmen(&Teban::Sente,&banmen)),
							   Rule::legal_moves_from_banmen(Teban::Sente,&State::new(banmen.clone())).into_iter().map(|m| {
								   LegalMove::from(m)
							   }).collect::<Vec<LegalMove>>()
					);
				}
			}
		}
	}
}
#[test]
fn test_legal_moves_banmen_with_kyou_all_vertical_position_occupied_opponent_gote() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	for x in [0,6,7] {
		for y in 0..9 {
			for ox in 0..9 {
				for oy in 0..9 {
					if ox == x && oy == y {
						continue;
					}
					let mut banmen = blank_banmen.clone();

					banmen.0[8-y][8-x] = GKyou;
					banmen.0[8-ox][8-oy] = SFu;

					assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_from_banmen(&Teban::Gote,&banmen)),
							   Rule::legal_moves_from_banmen(Teban::Gote,&State::new(banmen.clone())).into_iter().map(|m| {
								   LegalMove::from(m)
							   }).collect::<Vec<LegalMove>>()
					);
				}
			}
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SKyou;

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Sente,&banmen,a),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8 - p.1][8 - p.0] = GKyou;

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Gote,&banmen,a),
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
		sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SKei;

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Sente,&banmen,a),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-p.1][8-p.0] = GKei;

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Gote,&banmen,a),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SKei;

		for x in 0..9 {
			banmen.0[2][x] = SFu;
		}

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Sente,&banmen,a),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SKei;

		for x in 0..9 {
			banmen.0[2][x] = GFu;
		}

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Sente,&banmen,a),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = GKei;

		for x in 0..9 {
			banmen.0[6][x] = GFu;
		}

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Gote,&banmen,a),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = GKei;

		for x in 0..9 {
			banmen.0[6][x] = SFu;
		}

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Gote,&banmen,a),
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
		sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SGin;

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Sente,&banmen,a),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-p.1][8-p.0] = GGin;

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Gote,&banmen,a),
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
		sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
		sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = kind;

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Sente,&banmen,a),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-p.1][8-p.0] = kind;

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Gote,&banmen,a),
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
			sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[p.1][p.0] = SOu;

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Sente,&banmen,a),
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

	for (a,p) in answer.into_iter().zip(&POSITIONS) {
		let mut banmen = blank_banmen.clone();

		banmen.0[8-p.1][8-p.0] = GOu;

		assert_eq!(
			&sort_legal_mvs_legacy(Teban::Gote,&banmen,a),
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
			sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
			sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Sente,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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
				sort_legal_mvs_legacy(Teban::Gote,&banmen,answer),
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

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Fu, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,Mochigoma::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc.clone().into(),&banmen),
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

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Fu, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(Mochigoma::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc.clone().into(),&banmen),
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

		let mut ms:Mochigoma = Mochigoma::new();

		ms.insert(MochigomaKind::Fu, 2);

		let mut mc:MochigomaCollections = MochigomaCollections::Pair(ms,Mochigoma::new());

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

		let mut mg:Mochigoma = Mochigoma::new();

		mg.insert(MochigomaKind::Fu, 2);

		let mut mc:MochigomaCollections = MochigomaCollections::Pair(Mochigoma::new(),mg);

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

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Kyou, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,Mochigoma::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc.clone().into(),&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_kyou_gote() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8-0][8-8] = Blank;

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Kyou, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(Mochigoma::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc.clone().into(),&banmen),
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

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Kei, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,Mochigoma::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc.clone().into(),&banmen),
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

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Kei, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(Mochigoma::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc.clone().into(),&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_gin_sente() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8][6] = Blank;

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Gin, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,Mochigoma::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc.clone().into(),&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_gin_gote() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8-8][8-6] = Blank;

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Gin, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(Mochigoma::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc.clone().into(),&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_kin_sente() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8][5] = Blank;

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Kin, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,Mochigoma::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc.clone().into(),&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_kin_gote() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8-8][8-5] = Blank;

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Kin, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(Mochigoma::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc.clone().into(),&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_hisha_sente() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[7][7] = Blank;

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Hisha, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,Mochigoma::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc.clone().into(),&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_hisha_gote() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8-7][8-7] = Blank;

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Hisha, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(Mochigoma::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc.clone().into(),&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Gote,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_kaku_sente() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[7][1] = Blank;

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Kaku, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(ms,Mochigoma::new());

	assert_eq!(legal_moves_from_mochigoma(&Teban::Sente,&mc.clone().into(),&banmen),
		Rule::legal_moves_from_mochigoma(Teban::Sente,&mc,&State::new(banmen.clone())).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_legal_moves_from_mochigoma_with_kaku_gote() {
	let mut banmen = rule::BANMEN_START_POS.clone();

	banmen.0[8-7][8-1] = Blank;

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Kaku, 1);

	let mc:MochigomaCollections = MochigomaCollections::Pair(Mochigoma::new(),mg);

	assert_eq!(legal_moves_from_mochigoma(&Teban::Gote,&mc.clone().into(),&banmen),
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
		position => position.extract()
	};

	assert_eq!(sort_legal_mvs_legacy(Teban::Sente,&banmen,legal_moves_all(&Teban::Sente,&banmen,&mc.clone().into())),
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
		position => position.extract()
	};

	assert_eq!(sort_legal_mvs_legacy(Teban::Gote,&banmen,legal_moves_all(&Teban::Gote,&banmen,&mc.clone().into())),
		Rule::legal_moves_all(Teban::Gote,&State::new(banmen.clone()),&mc).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[ignore]
#[test]
fn test_legal_moves_all_by_nonevasions_all_strategy() {
	let position_parser = PositionParser::new();

	let mut rng = rand::thread_rng();
	let mut rng = XorShiftRng::from_seed(rng.gen());

	let mut buffer = RandomPicker::new(Prng::new(rng.gen()));

	for (n,(sfen,answer)) in BufReader::new(
		File::open(
			Path::new("data").join("floodgate").join("legalmoves").join("kyokumen_sfen.txt")
		).unwrap()).lines().zip(BufReader::new(
		File::open(
				Path::new("data").join("floodgate").join("legalmoves").join("answer_noevasions_all.txt")
		).unwrap()).lines()).enumerate() {

		let mut expected = answer.unwrap().split(' ').into_iter().map(|m| m.to_string()).collect::<Vec<String>>();

		expected.sort();

		let expected = expected.join(" ");

		let sfen = format!("sfen {}",sfen.unwrap());

		let (teban, banmen, mc, _, _) = position_parser.parse(&sfen.split(' ').collect::<Vec<&str>>()).unwrap().extract();

		let state = State::new(banmen);

		Rule::legal_moves_all_by_strategy::<NonEvasionsAll>(teban, &state, &mc, &mut buffer).unwrap();

		let mvs: Vec<usiagent::rule::LegalMove> = (&buffer).into();

		let mut mvs = mvs.into_iter().map(|m| m.to_move().to_sfen().unwrap()).collect::<Vec<String>>();

		mvs.sort();

		let mvs = mvs.join(" ");

		if &expected != &mvs {
			println!("line {}: {}",n, sfen);
		}

		assert_eq!(expected, mvs);
	}
}