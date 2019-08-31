use std::collections::HashMap;

use usiagent::shogi::*;
use usiagent::rule::Rule;
use usiagent::rule::State;

use super::*;

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
		(next,nmc,_) => {
			mhash = hasher.calc_main_hash(mhash,Teban::Sente,state.get_banmen(),&mc,m,&None);
			shash = hasher.calc_sub_hash(shash,Teban::Sente,state.get_banmen(),&mc,m,&None);
			state = next;
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
		(next,nmc,_) => {
			mhash = hasher.calc_main_hash(mhash,Teban::Gote,state.get_banmen(),&mc,m,&None);
			shash = hasher.calc_sub_hash(shash,Teban::Gote,state.get_banmen(),&mc,m,&None);
			state = next;
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
