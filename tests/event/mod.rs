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
fn test_movekind_display() {
	let input_and_expected:Vec<(MovedKind,&'static str)> = vec![
		(MovedKind::Fu,"歩"),
		(MovedKind::Kyou,"香"),
		(MovedKind::Kei,"桂"),
		(MovedKind::Gin,"銀"),
		(MovedKind::Kin,"金"),
		(MovedKind::Kaku,"角"),
		(MovedKind::Hisha,"飛"),
		(MovedKind::SOu,"王"),
		(MovedKind::GOu,"玉"),
		(MovedKind::FuN,"成歩"),
		(MovedKind::KyouN,"成香"),
		(MovedKind::KeiN,"成桂"),
		(MovedKind::GinN,"成銀"),
		(MovedKind::KakuN,"馬"),
		(MovedKind::HishaN,"龍"),
		(MovedKind::Blank,"駒無し"),
	];

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(format!("{}",i).as_str(),r);
	}
}
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
		(Moved::To(MovedKind::SOu,(9,0),(9,1),false),"9零王 -> 9一 （不正な手です）"),
		(Moved::To(MovedKind::Gin,(9,0),(8,1),true),"9零銀 -> 8一成 （不正な手です）"),
		(Moved::To(MovedKind::SOu,(9,1),(9,0),false),"9一王 -> 9零 （不正な手です）"),
		(Moved::To(MovedKind::Gin,(9,1),(8,0),true),"9一銀 -> 8零成 （不正な手です）"),
		(Moved::To(MovedKind::SOu,(0,1),(1,2),false),"0一王 -> 1二 （不正な手です）"),
		(Moved::To(MovedKind::Kaku,(0,1),(1,2),true),"0一角 -> 1二成 （不正な手です）"),
		(Moved::To(MovedKind::SOu,(9,10),(8,9),false),"9,10王 -> 8,9 （不正な手です）"),
		(Moved::To(MovedKind::Kaku,(9,10),(8,9),true),"9,10角 -> 8,9成 （不正な手です）"),
		(Moved::To(MovedKind::SOu,(1,9),(2,10),false),"1,9王 -> 2,10 （不正な手です）"),
		(Moved::To(MovedKind::Kaku,(1,9),(2,10),true),"1,9角 -> 2,10成 （不正な手です）"),
		(Moved::Put(MochigomaKind::Fu,(10,1)),"10一歩 （不正な手です）"),
		(Moved::Put(MochigomaKind::Kyou,(10,1)),"10一香 （不正な手です）"),
		(Moved::Put(MochigomaKind::Kei,(10,1)),"10一桂 （不正な手です）"),
		(Moved::Put(MochigomaKind::Gin,(10,1)),"10一銀 （不正な手です）"),
		(Moved::Put(MochigomaKind::Kin,(10,1)),"10一金 （不正な手です）"),
		(Moved::Put(MochigomaKind::Kaku,(10,1)),"10一角 （不正な手です）"),
		(Moved::Put(MochigomaKind::Hisha,(10,1)),"10一飛 （不正な手です）"),
		(Moved::Put(MochigomaKind::Fu,(9,0)),"9零歩 （不正な手です）"),
		(Moved::Put(MochigomaKind::Kyou,(9,0)),"9零香 （不正な手です）"),
		(Moved::Put(MochigomaKind::Kei,(9,0)),"9零桂 （不正な手です）"),
		(Moved::Put(MochigomaKind::Gin,(9,0)),"9零銀 （不正な手です）"),
		(Moved::Put(MochigomaKind::Kin,(9,0)),"9零金 （不正な手です）"),
		(Moved::Put(MochigomaKind::Kaku,(9,0)),"9零角 （不正な手です）"),
		(Moved::Put(MochigomaKind::Hisha,(9,0)),"9零飛 （不正な手です）"),
		(Moved::Put(MochigomaKind::Fu,(9,10)),"9,10歩 （不正な手です）"),
		(Moved::Put(MochigomaKind::Kyou,(9,10)),"9,10香 （不正な手です）"),
		(Moved::Put(MochigomaKind::Kei,(9,10)),"9,10桂 （不正な手です）"),
		(Moved::Put(MochigomaKind::Gin,(9,10)),"9,10銀 （不正な手です）"),
		(Moved::Put(MochigomaKind::Kin,(9,10)),"9,10金 （不正な手です）"),
		(Moved::Put(MochigomaKind::Kaku,(9,10)),"9,10角 （不正な手です）"),
		(Moved::Put(MochigomaKind::Hisha,(9,10)),"9,10飛 （不正な手です）"),

		(Moved::To(MovedKind::Fu,(9,1),(8,2),true),"9一歩 -> 8二成"),
		(Moved::To(MovedKind::Kyou,(9,1),(8,2),true),"9一香 -> 8二成"),
		(Moved::To(MovedKind::Kei,(9,1),(8,2),true),"9一桂 -> 8二成"),
		(Moved::To(MovedKind::Gin,(9,1),(8,2),true),"9一銀 -> 8二成"),
		(Moved::To(MovedKind::Kin,(9,1),(8,2),true),"9一金 -> 8二成 （不正な手です）"),
		(Moved::To(MovedKind::Kaku,(9,1),(8,2),true),"9一角 -> 8二成"),
		(Moved::To(MovedKind::Hisha,(9,1),(8,2),true),"9一飛 -> 8二成"),
		(Moved::To(MovedKind::SOu,(9,1),(8,2),true),"9一王 -> 8二成 （不正な手です）"),
		(Moved::To(MovedKind::GOu,(9,1),(8,2),true),"9一玉 -> 8二成 （不正な手です）"),
		(Moved::To(MovedKind::FuN,(9,1),(8,2),false),"9一成歩 -> 8二"),
		(Moved::To(MovedKind::FuN,(9,1),(8,2),true),"9一成歩 -> 8二成 （不正な手です）"),
		(Moved::To(MovedKind::KyouN,(9,1),(8,2),false),"9一成香 -> 8二"),
		(Moved::To(MovedKind::KyouN,(9,1),(8,2),true),"9一成香 -> 8二成 （不正な手です）"),
		(Moved::To(MovedKind::KeiN,(9,1),(8,2),false),"9一成桂 -> 8二"),
		(Moved::To(MovedKind::KeiN,(9,1),(8,2),true),"9一成桂 -> 8二成 （不正な手です）"),
		(Moved::To(MovedKind::GinN,(9,1),(8,2),false),"9一成銀 -> 8二"),
		(Moved::To(MovedKind::GinN,(9,1),(8,2),true),"9一成銀 -> 8二成 （不正な手です）"),
		(Moved::To(MovedKind::KakuN,(9,1),(8,2),false),"9一馬 -> 8二"),
		(Moved::To(MovedKind::KakuN,(9,1),(8,2),true),"9一馬 -> 8二成 （不正な手です）"),
		(Moved::To(MovedKind::HishaN,(9,1),(8,2),false),"9一龍 -> 8二"),
		(Moved::To(MovedKind::HishaN,(9,1),(8,2),true),"9一龍 -> 8二成 （不正な手です）"),
		(Moved::To(MovedKind::Fu,(9,1),(8,2),false),"9一歩 -> 8二"),
		(Moved::To(MovedKind::Kyou,(9,1),(8,2),false),"9一香 -> 8二"),
		(Moved::To(MovedKind::Kei,(9,1),(8,2),false),"9一桂 -> 8二"),
		(Moved::To(MovedKind::Gin,(9,1),(8,2),false),"9一銀 -> 8二"),
		(Moved::To(MovedKind::Kin,(9,1),(8,2),false),"9一金 -> 8二"),
		(Moved::To(MovedKind::Kaku,(9,1),(8,2),false),"9一角 -> 8二"),
		(Moved::To(MovedKind::Hisha,(9,1),(8,2),false),"9一飛 -> 8二"),
		(Moved::To(MovedKind::SOu,(9,1),(8,2),false),"9一王 -> 8二"),
		(Moved::To(MovedKind::GOu,(9,1),(8,2),false),"9一玉 -> 8二"),
		(Moved::To(MovedKind::Blank,(9,1),(8,2),false),"9一駒無し -> 8二 （不正な手です）"),
		(Moved::To(MovedKind::Blank,(9,1),(8,2),true),"9一駒無し -> 8二成 （不正な手です）"),
		(Moved::Put(MochigomaKind::Fu,(9,1)),"9一歩"),
		(Moved::Put(MochigomaKind::Kyou,(9,1)),"9一香"),
		(Moved::Put(MochigomaKind::Kei,(9,1)),"9一桂"),
		(Moved::Put(MochigomaKind::Gin,(9,1)),"9一銀"),
		(Moved::Put(MochigomaKind::Kin,(9,1)),"9一金"),
		(Moved::Put(MochigomaKind::Kaku,(9,1)),"9一角"),
		(Moved::Put(MochigomaKind::Hisha,(9,1)),"9一飛"),
	];

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(format!("{}",i).as_str(),r);
	}
}