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
		let mut ms = Mochigoma::new();

		ms.insert(MochigomaKind::Kyou, 1);

		let mut mc = MochigomaCollections::Pair(ms,Mochigoma::new());

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
		let mut ms = Mochigoma::new();

		ms.insert(MochigomaKind::Kyou, 1);

		let mut mc = MochigomaCollections::Pair(ms,Mochigoma::new());


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
		let mut mg = Mochigoma::new();

		mg.insert(MochigomaKind::Kyou, 1);

		let mut mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

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
		let mut mg = Mochigoma::new();

		mg.insert(MochigomaKind::Kyou, 1);

		let mut mc = MochigomaCollections::Pair(Mochigoma::new(),mg);


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
		let mut ms = Mochigoma::new();

		ms.insert(MochigomaKind::Kaku, 1);

		let mut mc = MochigomaCollections::Pair(ms,Mochigoma::new());

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
		let mut ms = Mochigoma::new();

		ms.insert(MochigomaKind::Kaku, 1);

		let mut mc = MochigomaCollections::Pair(ms,Mochigoma::new());


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
		let mut mg = Mochigoma::new();

		mg.insert(MochigomaKind::Kaku, 1);

		let mut mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

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
		let mut mg = Mochigoma::new();

		mg.insert(MochigomaKind::Kaku, 1);

		let mut mc = MochigomaCollections::Pair(Mochigoma::new(),mg);


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
		let mut ms = Mochigoma::new();

		ms.insert(MochigomaKind::Hisha, 1);

		let mut mc = MochigomaCollections::Pair(ms,Mochigoma::new());

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
		let mut ms = Mochigoma::new();

		ms.insert(MochigomaKind::Hisha, 1);

		let mut mc = MochigomaCollections::Pair(ms,Mochigoma::new());


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
		let mut mg = Mochigoma::new();

		mg.insert(MochigomaKind::Hisha, 1);

		let mut mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

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
		let mut mg = Mochigoma::new();

		mg.insert(MochigomaKind::Hisha, 1);

		let mut mc = MochigomaCollections::Pair(Mochigoma::new(),mg);


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

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(kind,1);

	let mc = MochigomaCollections::Pair(ms,Mochigoma::new());

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

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(kind,1);

	let mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

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

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Kei,1);

	let mc = MochigomaCollections::Pair(ms,Mochigoma::new());

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

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Kei,1);

	let mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

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

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Fu,1);

	let mc = MochigomaCollections::Pair(ms,Mochigoma::new());

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

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Fu,1);

	let mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

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

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Kyou,1);

	let mc = MochigomaCollections::Pair(ms,Mochigoma::new());

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

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Kyou,1);

	let mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

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

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Kei,1);

	let mc = MochigomaCollections::Pair(ms,Mochigoma::new());

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

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Kei,1);

	let mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

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

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Gin,1);

	let mc = MochigomaCollections::Pair(ms,Mochigoma::new());

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

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Gin,1);

	let mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

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

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,Mochigoma::new());

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

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

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

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Kaku,1);

	let mc = MochigomaCollections::Pair(ms,Mochigoma::new());

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

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Kaku,1);

	let mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

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

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Hisha,1);

	let mc = MochigomaCollections::Pair(ms,Mochigoma::new());

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

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Hisha,1);

	let mc = MochigomaCollections::Pair(Mochigoma::new(),mg);

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
