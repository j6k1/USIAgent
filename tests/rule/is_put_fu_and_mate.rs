use usiagent::shogi::*;
use usiagent::shogi::MochigomaCollections;
use usiagent::rule::Rule;
use usiagent::rule::State;

use super::*;

#[test]
fn test_is_put_fu_and_mate_sente() {
	let blank_banmen = Banmen([[Blank; 9]; 9]);

	let m = Move::Put(MochigomaKind::Fu,KomaDstPutPosition(9-4,1+1));

	let position_and_kinds:Vec<Vec<(usize,usize,KomaKind)>> = vec![
		vec![
			(3,0,GFu),(3,1,GKyou),(4,0,GOu),(5,0,GFu),(5,1,GKyou),(4,8,SKyou)
		],
		vec![
			(3,0,GFu),(3,1,GKyou),(4,0,GOu),(5,0,GFu),(5,1,GKyou),(4,2,SKin)
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

	let answer:[bool; 5] = [
		true,true,false,false,false
	];

	for (pk,answer) in position_and_kinds.iter().zip(&answer) {
		let mut banmen = blank_banmen.clone();

		for pk in pk {
			banmen.0[pk.1][pk.0] = pk.2;
		}

		let mut state = State::new(banmen);

		let mut ms:Mochigoma = Mochigoma::new();
		let mg:Mochigoma = Mochigoma::new();

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
			(8-3,8-0,SFu),(8-3,8-1,SKyou),(8-4,8-0,SOu),(8-5,8-0,SFu),(8-5,8-1,SKyou),(8-4,8-2,GKin)
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

	let answer:[bool; 5] = [
		true,true,false,false,false
	];

	for (pk,answer) in position_and_kinds.iter().zip(&answer) {
		let mut banmen = blank_banmen.clone();

		for pk in pk {
			banmen.0[pk.1][pk.0] = pk.2;
		}

		let mut state = State::new(banmen);

		let mut mg:Mochigoma = Mochigoma::new();
		let ms:Mochigoma = Mochigoma::new();

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
