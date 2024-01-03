use usiagent::shogi::*;
use usiagent::shogi::MochigomaCollections;
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
fn test_respond_oute_only_moves_all_sente() {
	let opponents:Vec<(KomaKind,u32,u32)> = vec![
		(GKyou,4,2),
		(GKei,3,5),
		(GKyou,4,2),
	];

	let mvs:Vec<Vec<Move>> = vec![
		vec![ Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,7+1)) ],
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

	base_banmen.0[6][4] = Blank;
	base_banmen.0[0][0] = Blank;

	for (o,(mvs,answer)) in opponents.into_iter().zip(mvs.into_iter().zip(answer.into_iter())) {
		let mut banmen = base_banmen.clone();

		banmen.0[o.2 as usize][o.1 as usize] = o.0;

		let mut ms:Mochigoma = Mochigoma::new();

		ms.insert(MochigomaKind::Fu,1);

		let mut mg:Mochigoma = Mochigoma::new();

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

		assert_eq!(sort_legal_mvs_legacy_move(Teban::Sente,state.get_banmen(),answer),
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

	let mc = MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new());

	let state = State::new(banmen);

	assert_eq!(sort_legal_mvs_legacy(Teban::Sente,state.get_banmen(),answer.into_iter().map(|m| LegalMove::from(m)).collect::<Vec<LegalMove>>()),
		Rule::respond_oute_only_moves_all(Teban::Sente,&state,&mc).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_respond_oute_only_moves_all_no_move_sente() {
	let mvs:Vec<Move> = vec![
		Move::To(KomaSrcPosition(8,6),KomaDstToPosition(8,7,true)),
		Move::To(KomaSrcPosition(8,8),KomaDstToPosition(7,9,false)),
		Move::Put(MochigomaKind::Kin,KomaDstPutPosition(7,8))
	];

	let banmen = Banmen([
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,GGin,GGin,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,SOu,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SKyou,SKei,Blank,Blank,Blank,Blank,Blank,Blank,Blank]
	]);

	let ms:Mochigoma = Mochigoma::new();

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Kin,1);

	let mut mc = MochigomaCollections::Pair(ms,mg);

	let mut state = State::new(banmen.clone());

	let mut teban = Teban::Gote;

	for m in mvs {
		match Rule::apply_move_none_check(&state,teban,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
				teban = teban.opposite();
			}
		}
	}

	assert_eq!(0,Rule::respond_oute_only_moves_all(teban,&state,&mc).into_iter().map(|m| {
		m.to_applied_move().to_move()
	}).collect::<Vec<Move>>().len());
}
#[test]
fn test_respond_oute_only_moves_all_gote() {
	let opponents:Vec<(KomaKind,u32,u32)> = vec![
		(SKyou,8-4,8-2),
		(SKei,8-3,8-5),
		(SKyou,8-4,8-2),
	];

	let mvs:Vec<Vec<Move>> = vec![
		vec![ Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,7+1)) ],
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

	base_banmen.0[8-6][8-4] = Blank;
	base_banmen.0[8-0][8-0] = Blank;

	for (o,(mvs,answer)) in opponents.into_iter().zip(mvs.into_iter().zip(answer.into_iter())) {
		let mut banmen = base_banmen.clone();

		banmen.0[o.2 as usize][o.1 as usize] = o.0;

		let mut ms:Mochigoma = Mochigoma::new();

		ms.insert(MochigomaKind::Fu,1);

		let mut mg:Mochigoma = Mochigoma::new();

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

		assert_eq!(sort_legal_mvs_legacy_move(Teban::Gote,state.get_banmen(),answer),
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

	let mc = MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new());

	let state = State::new(banmen);

	assert_eq!(answer.into_iter().map(|m| LegalMove::from(m)).collect::<Vec<LegalMove>>(),
		Rule::respond_oute_only_moves_all(Teban::Gote,&state,&mc).into_iter().map(|m| {
			LegalMove::from(m)
		}).collect::<Vec<LegalMove>>()
	);
}
#[test]
fn test_respond_oute_only_moves_all_no_move_gote() {
	let mvs:Vec<Move> = vec![
		Move::To(KomaSrcPosition(2,4),KomaDstToPosition(2,3,true)),
		Move::To(KomaSrcPosition(2,2),KomaDstToPosition(3,1,false)),
		Move::Put(MochigomaKind::Kin,KomaDstPutPosition(3,2))
	];

	let banmen = Banmen([
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKei,GKyou],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GOu,Blank],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,Blank,Blank,Blank,Blank,Blank,SGin,SGin,Blank],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank]
	]);

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Kin,1);

	let mg:Mochigoma = Mochigoma::new();
	let mut mc = MochigomaCollections::Pair(ms,mg);

	let mut state = State::new(banmen.clone());

	let mut teban = Teban::Sente;

	for m in mvs {
		match Rule::apply_move_none_check(&state,teban,&mc,m.to_applied_move()) {
			(next,nmc,_) => {
				state = next;
				mc = nmc;
				teban = teban.opposite();
			}
		}
	}

	assert_eq!(0,Rule::respond_oute_only_moves_all(teban,&state,&mc).into_iter().map(|m| {
		m.to_applied_move().to_move()
	}).collect::<Vec<Move>>().len());
}
