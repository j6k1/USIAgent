use std::collections::HashMap;

use usiagent::shogi::*;
use usiagent::rule::Rule;
use usiagent::rule::State;

use super::*;

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
