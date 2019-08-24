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
