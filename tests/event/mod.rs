use usiagent::shogi::*;
use usiagent::event::*;
use usiagent::error::*;
use usiagent::TryFrom;

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
fn test_moved_try_from() {
	let input_and_expected:Vec<(KomaKind,Result<Moved, TypeConvertError<String>>)> = vec![
		(SFu,Ok(Moved::To(MovedKind::Fu,(9,1),(8,2),false))),
		(SKyou,Ok(Moved::To(MovedKind::Kyou,(9,1),(8,2),false))),
		(SKei,Ok(Moved::To(MovedKind::Kei,(9,1),(8,2),false))),
		(SGin,Ok(Moved::To(MovedKind::Gin,(9,1),(8,2),false))),
		(SKin,Ok(Moved::To(MovedKind::Kin,(9,1),(8,2),false))),
		(SKaku,Ok(Moved::To(MovedKind::Kaku,(9,1),(8,2),false))),
		(SHisha,Ok(Moved::To(MovedKind::Hisha,(9,1),(8,2),false))),
		(SOu,Ok(Moved::To(MovedKind::SOu,(9,1),(8,2),false))),
		(SFuN,Ok(Moved::To(MovedKind::FuN,(9,1),(8,2),false))),
		(SKyouN,Ok(Moved::To(MovedKind::KyouN,(9,1),(8,2),false))),
		(SKeiN,Ok(Moved::To(MovedKind::KeiN,(9,1),(8,2),false))),
		(SGinN,Ok(Moved::To(MovedKind::GinN,(9,1),(8,2),false))),
		(SKakuN,Ok(Moved::To(MovedKind::KakuN,(9,1),(8,2),false))),
		(SHishaN,Ok(Moved::To(MovedKind::HishaN,(9,1),(8,2),false))),
		(GFu,Ok(Moved::To(MovedKind::Fu,(9,1),(8,2),false))),
		(GKyou,Ok(Moved::To(MovedKind::Kyou,(9,1),(8,2),false))),
		(GKei,Ok(Moved::To(MovedKind::Kei,(9,1),(8,2),false))),
		(GGin,Ok(Moved::To(MovedKind::Gin,(9,1),(8,2),false))),
		(GKin,Ok(Moved::To(MovedKind::Kin,(9,1),(8,2),false))),
		(GKaku,Ok(Moved::To(MovedKind::Kaku,(9,1),(8,2),false))),
		(GHisha,Ok(Moved::To(MovedKind::Hisha,(9,1),(8,2),false))),
		(GOu,Ok(Moved::To(MovedKind::GOu,(9,1),(8,2),false))),
		(GFuN,Ok(Moved::To(MovedKind::FuN,(9,1),(8,2),false))),
		(GKyouN,Ok(Moved::To(MovedKind::KyouN,(9,1),(8,2),false))),
		(GKeiN,Ok(Moved::To(MovedKind::KeiN,(9,1),(8,2),false))),
		(GGinN,Ok(Moved::To(MovedKind::GinN,(9,1),(8,2),false))),
		(GKakuN,Ok(Moved::To(MovedKind::KakuN,(9,1),(8,2),false))),
		(GHishaN,Ok(Moved::To(MovedKind::HishaN,(9,1),(8,2),false))),
		(Blank,Ok(Moved::To(MovedKind::Blank,(9,1),(8,2),false))),
	];

	for (k,r) in input_and_expected.into_iter() {
		let mut banmen = Banmen([[Blank; 9]; 9]);
		banmen.0[0][0] = k;

		assert_eq!(Moved::try_from((&banmen,&Move::To(KomaSrcPosition(9,1),KomaDstToPosition(8,2,false)))),r);
	}

	let input_and_expected:Vec<(KomaKind,Result<Moved, TypeConvertError<String>>)> = vec![
		(SFu,Ok(Moved::To(MovedKind::Fu,(9,1),(8,2),true))),
		(SKyou,Ok(Moved::To(MovedKind::Kyou,(9,1),(8,2),true))),
		(SKei,Ok(Moved::To(MovedKind::Kei,(9,1),(8,2),true))),
		(SGin,Ok(Moved::To(MovedKind::Gin,(9,1),(8,2),true))),
		(SKin,Ok(Moved::To(MovedKind::Kin,(9,1),(8,2),true))),
		(SKaku,Ok(Moved::To(MovedKind::Kaku,(9,1),(8,2),true))),
		(SHisha,Ok(Moved::To(MovedKind::Hisha,(9,1),(8,2),true))),
		(SOu,Ok(Moved::To(MovedKind::SOu,(9,1),(8,2),true))),
		(SFuN,Ok(Moved::To(MovedKind::FuN,(9,1),(8,2),true))),
		(SKyouN,Ok(Moved::To(MovedKind::KyouN,(9,1),(8,2),true))),
		(SKeiN,Ok(Moved::To(MovedKind::KeiN,(9,1),(8,2),true))),
		(SGinN,Ok(Moved::To(MovedKind::GinN,(9,1),(8,2),true))),
		(SKakuN,Ok(Moved::To(MovedKind::KakuN,(9,1),(8,2),true))),
		(SHishaN,Ok(Moved::To(MovedKind::HishaN,(9,1),(8,2),true))),
		(GFu,Ok(Moved::To(MovedKind::Fu,(9,1),(8,2),true))),
		(GKyou,Ok(Moved::To(MovedKind::Kyou,(9,1),(8,2),true))),
		(GKei,Ok(Moved::To(MovedKind::Kei,(9,1),(8,2),true))),
		(GGin,Ok(Moved::To(MovedKind::Gin,(9,1),(8,2),true))),
		(GKin,Ok(Moved::To(MovedKind::Kin,(9,1),(8,2),true))),
		(GKaku,Ok(Moved::To(MovedKind::Kaku,(9,1),(8,2),true))),
		(GHisha,Ok(Moved::To(MovedKind::Hisha,(9,1),(8,2),true))),
		(GOu,Ok(Moved::To(MovedKind::GOu,(9,1),(8,2),true))),
		(GFuN,Ok(Moved::To(MovedKind::FuN,(9,1),(8,2),true))),
		(GKyouN,Ok(Moved::To(MovedKind::KyouN,(9,1),(8,2),true))),
		(GKeiN,Ok(Moved::To(MovedKind::KeiN,(9,1),(8,2),true))),
		(GGinN,Ok(Moved::To(MovedKind::GinN,(9,1),(8,2),true))),
		(GKakuN,Ok(Moved::To(MovedKind::KakuN,(9,1),(8,2),true))),
		(GHishaN,Ok(Moved::To(MovedKind::HishaN,(9,1),(8,2),true))),
		(Blank,Ok(Moved::To(MovedKind::Blank,(9,1),(8,2),true))),
	];

	for (k,r) in input_and_expected.into_iter() {
		let mut banmen = Banmen([[Blank; 9]; 9]);
		banmen.0[0][0] = k;

		assert_eq!(Moved::try_from((&banmen,&Move::To(KomaSrcPosition(9,1),KomaDstToPosition(8,2,true)))),r);
	}

	let input_and_expected:Vec<(MochigomaKind,Result<Moved, TypeConvertError<String>>)> = vec![
		(MochigomaKind::Fu,Ok(Moved::Put(MochigomaKind::Fu,(9,1)))),
		(MochigomaKind::Kyou,Ok(Moved::Put(MochigomaKind::Kyou,(9,1)))),
		(MochigomaKind::Kei,Ok(Moved::Put(MochigomaKind::Kei,(9,1)))),
		(MochigomaKind::Gin,Ok(Moved::Put(MochigomaKind::Gin,(9,1)))),
		(MochigomaKind::Kin,Ok(Moved::Put(MochigomaKind::Kin,(9,1)))),
		(MochigomaKind::Hisha,Ok(Moved::Put(MochigomaKind::Hisha,(9,1)))),
		(MochigomaKind::Kaku,Ok(Moved::Put(MochigomaKind::Kaku,(9,1))))
	];


	for (k,r) in input_and_expected.into_iter() {
		let banmen = Banmen([[Blank; 9]; 9]);

		assert_eq!(Moved::try_from((&banmen,&Move::Put(k,KomaDstPutPosition(9,1)))),r);
	}
}
#[test]
fn test_moved_display() {
	let input_and_expected:Vec<(Moved,&'static str)> = vec![

	];

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(format!("{}",i).as_str(),r);
	}
}