extern crate usiagent;

use usiagent::TryFrom;
use usiagent::Find;
use usiagent::shogi::*;
use usiagent::error::*;
use usiagent::rule;

#[test]
fn test_try_from_komakind_to_obtainkind() {
	const KOMAKIND_AND_OBTAINKINDS:[(KomaKind,ObtainKind); 28] = [
		(KomaKind::SFu,ObtainKind::Fu),
		(KomaKind::SKyou,ObtainKind::Kyou),
		(KomaKind::SKei,ObtainKind::Kei),
		(KomaKind::SGin,ObtainKind::Gin),
		(KomaKind::SKin,ObtainKind::Kin),
		(KomaKind::SKaku,ObtainKind::Kaku),
		(KomaKind::SHisha,ObtainKind::Hisha),
		(KomaKind::SOu,ObtainKind::Ou),
		(KomaKind::SFuN,ObtainKind::FuN),
		(KomaKind::SKyouN,ObtainKind::KyouN),
		(KomaKind::SKeiN,ObtainKind::KeiN),
		(KomaKind::SGinN,ObtainKind::GinN),
		(KomaKind::SKakuN,ObtainKind::KakuN),
		(KomaKind::SHishaN,ObtainKind::HishaN),
		(KomaKind::GFu,ObtainKind::Fu),
		(KomaKind::GKyou,ObtainKind::Kyou),
		(KomaKind::GKei,ObtainKind::Kei),
		(KomaKind::GGin,ObtainKind::Gin),
		(KomaKind::GKin,ObtainKind::Kin),
		(KomaKind::GKaku,ObtainKind::Kaku),
		(KomaKind::GHisha,ObtainKind::Hisha),
		(KomaKind::GOu,ObtainKind::Ou),
		(KomaKind::GFuN,ObtainKind::FuN),
		(KomaKind::GKyouN,ObtainKind::KyouN),
		(KomaKind::GKeiN,ObtainKind::KeiN),
		(KomaKind::GGinN,ObtainKind::GinN),
		(KomaKind::GKakuN,ObtainKind::KakuN),
		(KomaKind::GHishaN,ObtainKind::HishaN),
	];

	for pair in &KOMAKIND_AND_OBTAINKINDS {
		assert_eq!(pair.1,ObtainKind::try_from(pair.0).unwrap());
	}

	match ObtainKind::try_from(KomaKind::Blank) {
		Err(TypeConvertError::LogicError(msg)) => {
			assert_eq!(String::from("Can not  to convert Blank to ObtainKind."), msg);
		},
		_ => assert!(false),
	}
}
#[test]
fn test_try_from_obtainkind_to_mochigomakind() {
	const OBTAINKIND_AND_MOCHIGOMAKINDS:[
		(ObtainKind,MochigomaKind); 13] = [
		(ObtainKind::Fu,MochigomaKind::Fu),
		(ObtainKind::Kyou,MochigomaKind::Kyou),
		(ObtainKind::Kei,MochigomaKind::Kei),
		(ObtainKind::Gin,MochigomaKind::Gin),
		(ObtainKind::Kin,MochigomaKind::Kin),
		(ObtainKind::Kaku,MochigomaKind::Kaku),
		(ObtainKind::Hisha,MochigomaKind::Hisha),
		(ObtainKind::FuN,MochigomaKind::Fu),
		(ObtainKind::KyouN,MochigomaKind::Kyou),
		(ObtainKind::KeiN,MochigomaKind::Kei),
		(ObtainKind::GinN,MochigomaKind::Gin),
		(ObtainKind::KakuN,MochigomaKind::Kaku),
		(ObtainKind::HishaN,MochigomaKind::Hisha),
	];

	for pair in &OBTAINKIND_AND_MOCHIGOMAKINDS {
		assert_eq!(pair.1,MochigomaKind::try_from(pair.0).unwrap());
	}

	match MochigomaKind::try_from(ObtainKind::Ou) {
		Err(TypeConvertError::LogicError(msg)) => {
			assert_eq!(String::from("Can not  to convert Ou to MochigomaKind."), msg);
		},
		_ => assert!(false),
	}
}
#[test]
fn test_find_obtainkind_from_legal_moves() {
	const OBTAINKINDS:[ObtainKind; 13] = [
		ObtainKind::Fu,
		ObtainKind::Kyou,
		ObtainKind::Kei,
		ObtainKind::Gin,
		ObtainKind::Kin,
		ObtainKind::Kaku,
		ObtainKind::Hisha,
		ObtainKind::FuN,
		ObtainKind::KyouN,
		ObtainKind::KeiN,
		ObtainKind::GinN,
		ObtainKind::KakuN,
		ObtainKind::HishaN,
	];

	for k in &OBTAINKINDS {
		let mut mvs:Vec<rule::LegalMove> = Vec::new();

		mvs.push(rule::LegalMove::To(KomaSrcPosition(1,4),KomaDstToPosition(1,3,false),Some(*k)));

		assert_eq!(vec![Move::To(KomaSrcPosition(1,4),KomaDstToPosition(1,3,false))],
			mvs.find(k).unwrap()
		);
	}

	for k in &OBTAINKINDS {
		let mut mvs:Vec<rule::LegalMove> = Vec::new();

		mvs.push(rule::LegalMove::To(KomaSrcPosition(1,4),KomaDstToPosition(1,3,false),Some(*k)));
		mvs.push(rule::LegalMove::To(KomaSrcPosition(2,4),KomaDstToPosition(2,3,false),Some(*k)));

		assert_eq!(
			vec![Move::To(KomaSrcPosition(1,4),KomaDstToPosition(1,3,false)),
				 Move::To(KomaSrcPosition(2,4),KomaDstToPosition(2,3,false))],
			mvs.find(k).unwrap()
		);
	}

	for k in OBTAINKINDS.iter().skip(1) {
		let mut mvs:Vec<rule::LegalMove> = Vec::new();

		mvs.push(rule::LegalMove::To(KomaSrcPosition(1,4),KomaDstToPosition(1,3,false),Some(*k)));

		assert_eq!(None,mvs.find(&ObtainKind::Fu));
	}

	let mut mvs:Vec<rule::LegalMove> = Vec::new();

	mvs.push(rule::LegalMove::To(KomaSrcPosition(1,4),KomaDstToPosition(1,3,false),Some(ObtainKind::Fu)));

	for k in OBTAINKINDS.iter().skip(1) {
		assert_eq!(None,mvs.find(k));
	}
}