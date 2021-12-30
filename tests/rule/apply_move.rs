use usiagent::shogi::*;
use usiagent::rule::Rule;
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
	let mut omc = MochigomaCollections::Empty.into();

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
fn test_apply_move_none_check_mochigoma_not_empty_sente() {
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
	banmen.0[2][0] = Blank;

	let mut state = State::new(banmen.clone());

	let mut teban = Teban::Sente;

	let mut ms:Mochigoma = Mochigoma::new();
	let mg:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Fu, 1);

	let mut omc = MochigomaCollections::Pair(ms,mg).into();

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
	let mut omc = MochigomaCollections::Empty.into();

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
fn test_apply_move_none_check_mochigoma_not_empty_gote() {
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
	banmen.0[6][8] = Blank;

	let mut state = State::new(banmen.clone());
	let mut teban = Teban::Gote;

	let ms:Mochigoma = Mochigoma::new();
	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Fu, 1);

	let mut omc = MochigomaCollections::Pair(ms,mg).into();

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
	let mut mc = MochigomaCollections::Empty.into();

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

	assert_eq!(legal_moves_all(&Teban::Sente,&banmen,&mc.clone().into()),
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
	let mut mc = MochigomaCollections::Empty.into();

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

	assert_eq!(legal_moves_all(&Teban::Gote,&banmen,&mc.clone().into()),
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
		let mut ms:Mochigoma = Mochigoma::new();
		let mg:Mochigoma = Mochigoma::new();

		ms.insert(MochigomaKind::Fu, 2);
		ms.insert(MochigomaKind::Kyou,1);

		let mut omc = MochigomaCollections::Pair(ms,mg).into();

		match apply_move_none_check(&banmen,&teban,&omc,m) {
			(next,nmc,_) => {
				banmen = next;
				omc = nmc;
			}
		}

		let teban = Teban::Sente;
		let mut ms:Mochigoma = Mochigoma::new();
		let mg:Mochigoma = Mochigoma::new();

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
		let ms:Mochigoma = Mochigoma::new();
		let mut mg:Mochigoma = Mochigoma::new();

		mg.insert(MochigomaKind::Fu, 2);
		mg.insert(MochigomaKind::Kyou,1);

		let mut omc = MochigomaCollections::Pair(ms,mg).into();

		match apply_move_none_check(&banmen,&teban,&omc,m) {
			(next,nmc,_) => {
				banmen = next;
				omc = nmc;
			}
		}

		let teban = Teban::Gote;
		let ms:Mochigoma = Mochigoma::new();
		let mut mg:Mochigoma = Mochigoma::new();

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
	let mut omc = MochigomaCollections::Empty.into();

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
	let mut omc = MochigomaCollections::Empty.into();

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
fn test_apply_valid_move_valid_to_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	const POSITIONS:[(u32,u32); 5] = [
		(4,4),(2,3),(6,3),(4,2),(4,1)
	];

	let mvs:Vec<Vec<Vec<(u32,u32,bool)>>> = vec![
		// 歩
		vec![
			vec![
				(4,3,false)
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
				(4,3,false),
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
				).is_ok(), "apply_valid_move returned Err: kind = {:?}, move = {:?}.", k,m.to_move());

				let mut banmen = blank_banmen.clone();

				banmen.0[p.1 as usize][p.0 as usize] = *k;
				banmen.0[dy as usize][dx as usize] = GFu;

				let state = State::new(banmen);

				assert!(Rule::apply_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					m
				).is_ok(), "apply_valid_move to returned Err: kind = {:?}, move = {:?}.", k,m.to_move());
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
				(4,3,false)
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
				(4,3,false),
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
				).is_ok(), "apply_valid_move returned Err: kind = {:?}, move = {:?}.", k,m.to_move());

				let mut banmen = blank_banmen.clone();

				banmen.0[8 - p.1 as usize][8 - p.0 as usize] = *k;
				banmen.0[8 - dy as usize][8 - dx as usize] = SFu;

				let state = State::new(banmen);

				assert!(Rule::apply_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					m
				).is_ok(), "apply_valid_move returned Err: kind = {:?}, move = {:?}.", k,m.to_move());
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
		).is_ok(), "apply_valid_move returned Err: move = {:?}.", m.to_move());
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
		).is_ok(), "apply_valid_move returned Err move = {:?}.", m.to_move());
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
		).is_ok(), "apply_valid_move returned Err: move = {:?}.", m.to_move());
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
		).is_ok(), "apply_valid_move returned Err: move = {:?}.", m.to_move());
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
			).is_ok(), "apply_valid_move returned Err: move = {:?}.", m.to_move());
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
			).is_ok(), "apply_valid_move returned Err: move = {:?}.", m.to_move());
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
			).is_ok(), "apply_valid_move returned Err: move = {:?}.", m.to_move());
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
			).is_ok(), "apply_valid_move returned Err: move = {:?}.", m.to_move());
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
			).is_ok(), "apply_valid_move returned Err: move = {:?}.", m.to_move());
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
			).is_ok(), "apply_valid_move returned Err: move = {:?}.", m.to_move());
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
			).is_ok(), "apply_valid_move returned Err: move = {:?}.", m.to_move());
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
			).is_ok(), "apply_valid_move returned Err: move = {:?}.", m.to_move());
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
		).is_err(), "apply_valid_move returned Ok: move = {:?}.", m.to_move());
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
		).is_err(), "apply_valid_move returned Ok: move = {:?}.", m.to_move());
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
		).is_err(), "apply_valid_move returned Ok: move = {:?}.", m.to_move());
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
		).is_err(), "apply_valid_move returned Ok: move = {:?}.", m.to_move());
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
			).is_err(), "apply_valid_move returned Ok: move = {:?}.", m.to_move());
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
			).is_err(), "apply_valid_move returned Ok: move = {:?}.", m.to_move());
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
			).is_err(), "apply_valid_move returned Ok: move = {:?}.", m.to_move());
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
			).is_err(), "apply_valid_move returned Ok: move = {:?}.", m.to_move());
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
			).is_err(), "apply_valid_move returned Ok: move = {:?}.", m.to_move());
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
			).is_err(), "apply_valid_move returned Ok: move = {:?}.", m.to_move());
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
			).is_err(), "apply_valid_move returned Ok: move = {:?}.", m.to_move());
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
			).is_err(), "apply_valid_move returned Ok: move = {:?}.", m.to_move());
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
				).is_err(), "apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k, mv.to_move());

				let mv = rule::LegalMove::To(rule::LegalMoveTo::new(
												((m.0).0 * 9 + (m.0).1) as u32,
												((m.1).0 * 9 + (m.1).1) as u32,false,None)).to_applied_move();

				assert!(Rule::apply_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					mv
				).is_err(), "apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k, mv.to_move());
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
				).is_err(), "apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k, mv.to_move());

				let mv = rule::LegalMove::To(rule::LegalMoveTo::new(
												((m.0).0 * 9 + (m.0).1) as u32,
												((m.1).0 * 9 + (m.1).1) as u32,false,None)).to_applied_move();

				assert!(Rule::apply_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					mv
				).is_err(), "apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k, mv.to_move());
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
				).is_err(), "apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k,m.to_move());
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
				).is_err(), "apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k,m.to_move());
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
				).is_err(), "apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k,mv.to_move());

				let mv = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-p.0,p.1+1),KomaDstToPosition(9-dx,dy+1,false)));

				assert!(Rule::apply_valid_move(&state,
					Teban::Sente,
					&MochigomaCollections::Empty,
					mv
				).is_err(), "apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k,mv.to_move());
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
				).is_err(), "apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k,mv.to_move());

				let mv = rule::AppliedMove::from(Move::To(KomaSrcPosition(9-(8-p.0),(8-p.1)+1),KomaDstToPosition(9-(8-dx),(8-dy)+1,false)));

				assert!(Rule::apply_valid_move(&state,
					Teban::Gote,
					&MochigomaCollections::Empty,
					mv
				).is_err(), "apply_valid_move returned Ok: kind = {:?}, move = {:?}.", k,mv.to_move());
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
		let mut ms:Mochigoma = Mochigoma::new();

		ms.insert(*kind, 1);

		let mc = MochigomaCollections::Pair(ms,Mochigoma::new());

		for y in 0..9 {
			for x in 0..9 {
				if banmen.0[y][x] == Blank && *deny_line <= y as u32 {
					let m = rule::AppliedMove::from(Move::Put(*kind,KomaDstPutPosition(9 - x as u32,y as u32 + 1)));

					assert!(Rule::apply_valid_move(&state,
						Teban::Sente,
						&mc,
						m
					).is_ok(), "apply_valid_move returned Err: kind = {:?}, move = {:?}.", kind,m.to_move());
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
		let mut mg:Mochigoma = Mochigoma::new();

		mg.insert(*kind, 1);

		let mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

		for y in 0..9 {
			for x in 0..9 {
				let (x,y) = (8-x,8-y);

				if banmen.0[y][x] == Blank && (8 - *deny_line) >= y as u32 {
					let m = rule::AppliedMove::from(Move::Put(*kind,KomaDstPutPosition(9 - x as u32,y as u32 + 1)));

					assert!(Rule::apply_valid_move(&state,
						Teban::Gote,
						&mc,
						m
					).is_ok(), "apply_valid_move returned Err: kind = {:?}, move = {:?}.", kind,m.to_move());
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
		let mut ms:Mochigoma = Mochigoma::new();

		ms.insert(*kind, 1);

		let mc = MochigomaCollections::Pair(ms,Mochigoma::new());

		for y in 0..9 {
			for x in 0..9 {
				if banmen.0[y][x] != Blank || *deny_line > y as u32 {
					let m = rule::AppliedMove::from(Move::Put(*kind,KomaDstPutPosition(9 - x as u32,y as u32 + 1)));

					assert!(Rule::apply_valid_move(&state,
						Teban::Sente,
						&mc,
						m
					).is_err(), "apply_valid_move returned Ok: kind = {:?}, move = {:?}.", kind,m.to_move());
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
		let mut mg:Mochigoma = Mochigoma::new();

		mg.insert(*kind, 1);

		let mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

		for y in 0..9 {
			for x in 0..9 {
				let (x,y) = (8-x,8-y);

				if banmen.0[y][x] != Blank || (8 - *deny_line) < y as u32 {
					let m = rule::AppliedMove::from(Move::Put(*kind,KomaDstPutPosition(9 - x as u32,y as u32 + 1)));

					assert!(Rule::apply_valid_move(&state,
						Teban::Gote,
						&mc,
						m
					).is_err(), "apply_valid_move returned Ok: kind = {:?}, move = {:?}.", kind,m.to_move());
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
	let oute_kyokumen_map:KyokumenMap<u64,u32> = KyokumenMap::new();
	let hasher = KyokumenHash::new();

	let (imhash, ishash) = hasher.calc_initial_hash(&BANMEN_START_POS,&Mochigoma::new(),&Mochigoma::new());

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
		 _,
		 _) = Rule::apply_moves(state,teban,mc,&mvs,imhash,ishash,kyokumen_map,oute_kyokumen_map,&hasher);

	let (amhash, ashash) = hasher.calc_initial_hash(&after_banmen,&Mochigoma::new(),&Mochigoma::new());

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

	let (imhash, ishash) = hasher.calc_initial_hash(&BANMEN_START_POS,&Mochigoma::new(),&Mochigoma::new());

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
											 	|teban,banmen,mc,m,o,r:(u64,u64)| {
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
	let (amhash, ashash) = hasher.calc_initial_hash(&after_banmen,&Mochigoma::new(),&Mochigoma::new());

	assert_eq!(amhash,mhash);
	assert_eq!(ashash,shash);

	assert!(mhash != imhash);
	assert!(mhash != ishash);
}
