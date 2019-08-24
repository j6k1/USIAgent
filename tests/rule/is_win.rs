use usiagent::shogi::*;
use usiagent::rule::Rule;
use usiagent::rule::State;

use super::*;

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
