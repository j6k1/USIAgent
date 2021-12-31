use usiagent::shogi::*;
use usiagent::rule::Rule;
use usiagent::rule::State;

use super::*;

#[test]
fn test_is_nyugyoku_win_win_sente() {
	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,Blank,SKin,Blank,SKin,Blank],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SFuN],
		[Blank,SGin,SHishaN,SFuN,SFuN,Blank,Blank,Blank,SOu],
		[GFu,Blank,Blank,Blank,Blank,SKakuN,Blank,Blank,Blank],
		[Blank,SHishaN,SFu,SKakuN,Blank,Blank,Blank,Blank,Blank],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,Blank,Blank,Blank,Blank,Blank,Blank,GOu,Blank]
	]);

	let mut ms:Mochigoma = Mochigoma::new();
	let mut mg:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Fu,7);
	ms.insert(MochigomaKind::Kyou,2);
	ms.insert(MochigomaKind::Gin,1);
	ms.insert(MochigomaKind::Kei,1);
	mg.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&Some(Instant::now())));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,SHishaN,Blank,Blank,Blank,SGin,Blank,SHishaN],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SFuN,SKakuN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&Some(Instant::now())));

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()),&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()),&Some(Instant::now())));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,SHishaN,Blank,Blank,Blank,SGin,Blank,SKakuN],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SFuN,SHishaN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&Some(Instant::now())));

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()),&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()),&Some(Instant::now())));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SHishaN],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SFuN,SKakuN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Hisha,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(ms,Mochigoma::new()),&None));

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Hisha,1);
	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(ms,Mochigoma::new()),&Some(Instant::now())));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,SHishaN,Blank,Blank,Blank,SGin,Blank,Blank],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SFuN,SHishaN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);


	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Kaku,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(ms,Mochigoma::new()),&None));

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Kaku,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Pair(ms,Mochigoma::new()),&Some(Instant::now())));
}
#[test]
fn test_is_nyugyoku_win_lose_sente() {
	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,Blank,SKin,Blank,SKin,Blank],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SFuN],
		[Blank,SGin,SHishaN,SFuN,SFuN,Blank,Blank,Blank,SOu],
		[GFu,Blank,Blank,Blank,Blank,SKakuN,Blank,Blank,Blank],
		[Blank,SHishaN,SFu,SKakuN,Blank,Blank,Blank,Blank,Blank],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,Blank,Blank,Blank,Blank,Blank,Blank,GOu,Blank]
	]);

	let mut ms:Mochigoma = Mochigoma::new();
	let mut mg:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Fu,7);
	ms.insert(MochigomaKind::Kyou,2);
	ms.insert(MochigomaKind::Gin,1);
	ms.insert(MochigomaKind::Kei,1);
	mg.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&Some(Instant::now() + Duration::from_secs(60))));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,SHishaN,Blank,Blank,Blank,SGin,Blank,SHishaN],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SFuN,SKakuN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,
			&MochigomaCollections::Empty,&Some(Instant::now() + Duration::from_secs(60))));

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,
			&MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()),&Some(Instant::now() + Duration::from_secs(60))));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SHishaN],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SFuN,SKakuN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	let mut ms:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Hisha,1);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,
			&MochigomaCollections::Pair(ms,Mochigoma::new()),&Some(Instant::now() + Duration::from_secs(60))));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,Blank,SKin,Blank,SKin,Blank],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SFuN],
		[Blank,Blank,SHishaN,SFuN,SFuN,Blank,Blank,Blank,SOu],
		[GFu,SGin,Blank,Blank,Blank,SKakuN,Blank,Blank,Blank],
		[Blank,SHishaN,SFu,SKakuN,Blank,Blank,Blank,Blank,Blank],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,Blank,Blank,Blank,Blank,Blank,Blank,GOu,Blank]
	]);

	let mut ms:Mochigoma = Mochigoma::new();
	let mut mg:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Fu,7);
	ms.insert(MochigomaKind::Kyou,2);
	ms.insert(MochigomaKind::Gin,1);
	ms.insert(MochigomaKind::Kei,1);
	mg.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&Some(Instant::now())));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&None));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,Blank,SKin,Blank,SKin,Blank],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SFuN],
		[Blank,SGin,SFu,SFuN,SFuN,Blank,Blank,Blank,SOu],
		[GFu,Blank,Blank,Blank,Blank,SKakuN,Blank,Blank,Blank],
		[Blank,SHishaN,SHishaN,SKakuN,Blank,Blank,Blank,Blank,Blank],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,Blank,Blank,Blank,Blank,Blank,Blank,GOu,Blank]
	]);

	let mut ms:Mochigoma = Mochigoma::new();
	let mut mg:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Fu,7);
	ms.insert(MochigomaKind::Kyou,2);
	ms.insert(MochigomaKind::Gin,1);
	ms.insert(MochigomaKind::Kei,1);
	mg.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&Some(Instant::now())));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&None));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,SHishaN,Blank,Blank,Blank,SGin,Blank,SHishaN],
		[Blank,Blank,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,SGin,Blank,Blank,SFuN,SKakuN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&None));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&Some(Instant::now())));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,SFuN,SKin,SFuN,SKin,SFuN],
		[Blank,SKyouN,SHishaN,Blank,Blank,Blank,SGin,Blank,SFuN],
		[Blank,SGin,Blank,SFuN,Blank,Blank,SKakuN,Blank,SOu],
		[GFu,Blank,Blank,Blank,SHishaN,SKakuN,Blank,Blank,Blank],
		[Blank,Blank,SFu,Blank,Blank,SKei,SKyou,SKyou,SKin],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,SGin,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,SFu,SFu,SFu,SFu,SFu,Blank,GOu,Blank]
	]);

	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&None));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&MochigomaCollections::Empty,&Some(Instant::now())));

	let banmen = Banmen([
		[SFuN,SFuN,SFuN,SFuN,Blank,SKin,Blank,SKin,Blank],
		[Blank,SKyouN,Blank,Blank,Blank,Blank,SGin,Blank,SFuN],
		[Blank,SGin,SHishaN,SFuN,SFuN,Blank,Blank,Blank,Blank],
		[GFu,Blank,Blank,Blank,Blank,SKakuN,Blank,Blank,SOu],
		[Blank,SHishaN,SFu,SKakuN,Blank,Blank,Blank,Blank,Blank],
		[SFu,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank,Blank],
		[Blank,Blank,Blank,SGin,GKeiN,Blank,Blank,GKin,GFuN],
		[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GKeiN,Blank],
		[SKyou,Blank,Blank,Blank,Blank,Blank,Blank,GOu,Blank]
	]);

	let mut ms:Mochigoma = Mochigoma::new();
	let mut mg:Mochigoma = Mochigoma::new();

	ms.insert(MochigomaKind::Fu,7);
	ms.insert(MochigomaKind::Kyou,2);
	ms.insert(MochigomaKind::Gin,1);
	ms.insert(MochigomaKind::Kei,1);
	mg.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&None));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&Some(Instant::now())));
}
#[test]
fn test_is_nyugyoku_win_win_gote() {
	let banmen = Banmen([
		[Blank,SOu,Blank,Blank,Blank,Blank,Blank,Blank,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[Blank,Blank,Blank,Blank,Blank,GKakuN,GFu,GHishaN,Blank],
		[Blank,Blank,Blank,GKakuN,Blank,Blank,Blank,GGin,SFu],
		[GOu,Blank,Blank,Blank,GFuN,GFuN,GHishaN,Blank,Blank],
		[GFuN,Blank,GGin,Blank,Blank,Blank,Blank,GKyouN,Blank],
		[Blank,GKin,Blank,GKin,Blank,GFuN,GFuN,GFuN,GFuN]
	]);

	let mut ms:Mochigoma = Mochigoma::new();
	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Fu,7);
	mg.insert(MochigomaKind::Kyou,2);
	mg.insert(MochigomaKind::Gin,1);
	mg.insert(MochigomaKind::Kei,1);
	ms.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&Some(Instant::now())));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GKakuN,GFuN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GHishaN,Blank,GGin,Blank,Blank,Blank,GHishaN,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&Some(Instant::now())));

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()),&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()),&Some(Instant::now())));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GHishaN,GFuN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GKakuN,Blank,GGin,Blank,Blank,Blank,GHishaN,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&Some(Instant::now())));

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()),&None));
	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()),&Some(Instant::now())));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GKakuN,GFuN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GHishaN,Blank,GGin,Blank,Blank,Blank,Blank,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Hisha,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(Mochigoma::new(),mg),&None));

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Hisha,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(Mochigoma::new(),mg),&Some(Instant::now())));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GHishaN,GFuN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,Blank,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GKakuN,Blank,GGin,Blank,Blank,Blank,GHishaN,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Kaku,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(Mochigoma::new(),mg),&None));

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Kaku,1);

	assert!(Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Pair(Mochigoma::new(),mg),&Some(Instant::now())));
}
#[test]
fn test_is_nyugyoku_win_lose_gote() {
	let banmen = Banmen([
		[Blank,SOu,Blank,Blank,Blank,Blank,Blank,Blank,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[Blank,Blank,Blank,Blank,Blank,GKakuN,GFu,GHishaN,Blank],
		[Blank,Blank,Blank,GKakuN,Blank,Blank,Blank,GGin,SFu],
		[GOu,Blank,Blank,Blank,GFuN,GFuN,GHishaN,Blank,Blank],
		[GFuN,Blank,GGin,Blank,Blank,Blank,Blank,GKyouN,Blank],
		[Blank,GKin,Blank,GKin,Blank,GFuN,GFuN,GFuN,GFuN]
	]);

	let mut ms:Mochigoma = Mochigoma::new();
	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Fu,7);
	mg.insert(MochigomaKind::Kyou,2);
	mg.insert(MochigomaKind::Gin,1);
	mg.insert(MochigomaKind::Kei,1);
	ms.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Sente,&mc,&Some(Instant::now() + Duration::from_secs(60))));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GKakuN,GFuN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GHishaN,Blank,GGin,Blank,Blank,Blank,GHishaN,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,
			&MochigomaCollections::Empty,&Some(Instant::now() + Duration::from_secs(60))));

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,
			&MochigomaCollections::Pair(Mochigoma::new(),Mochigoma::new()),&Some(Instant::now() + Duration::from_secs(60))));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GKakuN,GFuN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GHishaN,Blank,GGin,Blank,Blank,Blank,Blank,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Hisha,1);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,
			&MochigomaCollections::Pair(Mochigoma::new(),mg),&Some(Instant::now() + Duration::from_secs(60))));

	let banmen = Banmen([
		[Blank,SOu,Blank,Blank,Blank,Blank,Blank,Blank,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[Blank,Blank,Blank,Blank,Blank,GKakuN,GFu,GHishaN,Blank],
		[Blank,Blank,Blank,GKakuN,Blank,Blank,GKyouN,GGin,SFu],
		[GOu,Blank,Blank,Blank,GFuN,GFuN,GHishaN,Blank,Blank],
		[GFuN,Blank,GGin,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,GKin,Blank,GKin,Blank,GFuN,GFuN,GFuN,GFuN]
	]);

	let mut ms:Mochigoma = Mochigoma::new();
	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Fu,7);
	mg.insert(MochigomaKind::Kyou,2);
	mg.insert(MochigomaKind::Gin,1);
	mg.insert(MochigomaKind::Kei,1);
	ms.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&Some(Instant::now())));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&None));

	let banmen = Banmen([
		[Blank,SOu,Blank,Blank,Blank,Blank,Blank,Blank,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[Blank,Blank,Blank,Blank,Blank,GKakuN,GHishaN,GHishaN,Blank],
		[Blank,Blank,Blank,GKakuN,Blank,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,Blank,Blank,GFuN,GFuN,GFu,GGin,Blank],
		[GFuN,Blank,GGin,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,GKin,Blank,GKin,Blank,GFuN,GFuN,GFuN,GFuN]
	]);

	let mut ms:Mochigoma = Mochigoma::new();
	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Fu,7);
	mg.insert(MochigomaKind::Kyou,2);
	mg.insert(MochigomaKind::Gin,1);
	mg.insert(MochigomaKind::Kei,1);
	ms.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&Some(Instant::now())));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&None));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GKakuN,GFuN,Blank,GKyouN,GGin,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,Blank,Blank],
		[GHishaN,Blank,GGin,Blank,Blank,Blank,GHishaN,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&None));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&Some(Instant::now())));

	let banmen = Banmen([
		[Blank,SOu,Blank,GFu,GFu,GFu,GFu,GFu,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,GGin,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[GKin,GKyou,GKyou,GKei,Blank,Blank,GFu,Blank,Blank],
		[Blank,Blank,Blank,GKakuN,GHishaN,Blank,Blank,GKyouN,SFu],
		[GOu,Blank,GKakuN,Blank,Blank,GFuN,Blank,GGin,Blank],
		[GFuN,Blank,GGin,Blank,Blank,Blank,GHishaN,Blank,Blank],
		[GFuN,GKin,GFuN,GKin,GFuN,GFuN,GFuN,GFuN,GFuN]
	]);

	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&None));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&MochigomaCollections::Empty,&Some(Instant::now())));

	let banmen = Banmen([
		[Blank,SOu,Blank,Blank,Blank,Blank,Blank,Blank,GKyou],
		[Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
		[SFuN,SKin,Blank,Blank,SKeiN,GGin,Blank,Blank,Blank],
		[Blank,Blank,SKeiN,Blank,Blank,Blank,Blank,Blank,GFu],
		[Blank,Blank,Blank,Blank,Blank,GKakuN,GFu,GHishaN,Blank],
		[GOu,Blank,Blank,GKakuN,Blank,Blank,Blank,GKyouN,SFu],
		[Blank,Blank,Blank,Blank,GFuN,GFuN,GHishaN,GGin,Blank],
		[GFuN,Blank,GGin,Blank,Blank,Blank,Blank,Blank,Blank],
		[Blank,GKin,Blank,GKin,Blank,GFuN,GFuN,GFuN,GFuN]
	]);

	let mut ms:Mochigoma = Mochigoma::new();
	let mut mg:Mochigoma = Mochigoma::new();

	mg.insert(MochigomaKind::Fu,7);
	mg.insert(MochigomaKind::Kyou,2);
	mg.insert(MochigomaKind::Gin,1);
	mg.insert(MochigomaKind::Kei,1);
	ms.insert(MochigomaKind::Kin,1);

	let mc = MochigomaCollections::Pair(ms,mg);
	let state = State::new(banmen);

	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&None));
	assert!(!Rule::is_nyugyoku_win(&state,Teban::Gote,&mc,&Some(Instant::now())));
}
