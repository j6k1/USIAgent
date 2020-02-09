use std::collections::HashMap;

use usiagent::shogi::*;
use usiagent::rule::Rule;
use usiagent::rule::State;

use super::*;

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
		vec![
			(5,8,SKin),(4,8,SOu),(3,8,SKin),(4,7,GKin),(4,6,GKin)
		],
	];

	let answer:[bool; 7] = [
		true,true,true,true,false,false,false
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
		vec![
			(8-5,8-8,GKin),(8-4,8-8,GOu),(8-3,8-8,GKin),(8-4,8-7,SKin),(8-4,8-6,SKin)
		],
	];

	let answer:[bool; 7] = [
		true,true,true,true,false,false,false
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
