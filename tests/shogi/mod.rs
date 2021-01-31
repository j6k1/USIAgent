use std::collections::HashMap;

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

		mvs.push(rule::LegalMove::To(rule::LegalMoveTo::new((9-4)*9,(9-3)*9,false,Some(*k))));

		assert_eq!(vec![Move::To(KomaSrcPosition(4,1),KomaDstToPosition(3,1,false))],
			mvs.find(k).unwrap()
		);
	}

	for k in &OBTAINKINDS {
		let mut mvs:Vec<rule::LegalMove> = Vec::new();

		mvs.push(rule::LegalMove::To(rule::LegalMoveTo::new((9-4)*9,(9-3)*9,false,Some(*k))));
		mvs.push(rule::LegalMove::To(rule::LegalMoveTo::new((9-4)*9+1,(9-3)*9+1,false,Some(*k))));

		assert_eq!(
			vec![Move::To(KomaSrcPosition(4,1),KomaDstToPosition(3,1,false)),
				 Move::To(KomaSrcPosition(4,2),KomaDstToPosition(3,2,false))],
			mvs.find(k).unwrap()
		);
	}

	for k in OBTAINKINDS.iter().skip(1) {
		let mut mvs:Vec<rule::LegalMove> = Vec::new();

		mvs.push(rule::LegalMove::To(rule::LegalMoveTo::new((9-4)*9,(9-3)*9,false,Some(*k))));

		assert_eq!(None,mvs.find(&ObtainKind::Fu));
	}

	let mut mvs:Vec<rule::LegalMove> = Vec::new();

	mvs.push(rule::LegalMove::To(rule::LegalMoveTo::new((9-4)*9,(9-3)*9,false,Some(ObtainKind::Fu))));

	for k in OBTAINKINDS.iter().skip(1) {
		assert_eq!(None,mvs.find(k));
	}
}
#[test]
fn test_mochigoma_collections_eq() {
	let input_and_expected:Vec<(MochigomaCollections,MochigomaCollections,bool)> = vec![
		(MochigomaCollections::Empty,MochigomaCollections::Empty,true),
		(MochigomaCollections::Pair(HashMap::new(),HashMap::new()),MochigomaCollections::Empty,true),
		(MochigomaCollections::Pair(HashMap::new(),vec![
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),MochigomaCollections::Pair(HashMap::new(),vec![
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),true),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),true),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,0),
			(MochigomaKind::Kaku,0),
			(MochigomaKind::Kin,0),
			(MochigomaKind::Gin,0),
			(MochigomaKind::Kei,0),
			(MochigomaKind::Kyou,0),
			(MochigomaKind::Fu,0),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,0),
			(MochigomaKind::Kaku,0),
			(MochigomaKind::Kin,0),
			(MochigomaKind::Gin,0),
			(MochigomaKind::Kei,0),
			(MochigomaKind::Kyou,0),
			(MochigomaKind::Fu,0),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),MochigomaCollections::Empty,true),
		(MochigomaCollections::Empty, MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,0),
			(MochigomaKind::Kaku,0),
			(MochigomaKind::Kin,0),
			(MochigomaKind::Gin,0),
			(MochigomaKind::Kei,0),
			(MochigomaKind::Kyou,0),
			(MochigomaKind::Fu,0),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,0),
			(MochigomaKind::Kaku,0),
			(MochigomaKind::Kin,0),
			(MochigomaKind::Gin,0),
			(MochigomaKind::Kei,0),
			(MochigomaKind::Kyou,0),
			(MochigomaKind::Fu,0),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),true),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,0),
			(MochigomaKind::Kin,4),
			(MochigomaKind::Gin,0),
			(MochigomaKind::Kei,4),
			(MochigomaKind::Kyou,0),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})), MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kin,4),
			(MochigomaKind::Kei,4),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),true),
		(MochigomaCollections::Pair(vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,0),
			(MochigomaKind::Kin,4),
			(MochigomaKind::Gin,0),
			(MochigomaKind::Kei,4),
			(MochigomaKind::Kyou,0),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})), MochigomaCollections::Pair(vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kin,4),
			(MochigomaKind::Kei,4),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),true),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,0),
			(MochigomaKind::Kin,4),
			(MochigomaKind::Gin,0),
			(MochigomaKind::Kei,4),
			(MochigomaKind::Kyou,0),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,0),
			(MochigomaKind::Kin,4),
			(MochigomaKind::Gin,0),
			(MochigomaKind::Kei,4),
			(MochigomaKind::Kyou,0),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})), MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kin,4),
			(MochigomaKind::Kei,4),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kin,4),
			(MochigomaKind::Kei,4),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),true),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Fu,9),
			(MochigomaKind::Kyou,2),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
			(MochigomaKind::Kei,2),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
			(MochigomaKind::Gin,2),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),true),
		(MochigomaCollections::Empty,MochigomaCollections::Pair(vec![
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),HashMap::new()),false),
		(MochigomaCollections::Empty,MochigomaCollections::Pair(HashMap::new(),vec![
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),HashMap::new()),MochigomaCollections::Empty,false),
		(MochigomaCollections::Pair(HashMap::new(),vec![
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),MochigomaCollections::Empty,false),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),HashMap::new()),MochigomaCollections::Pair(HashMap::new(),vec![
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(HashMap::new(),vec![
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),MochigomaCollections::Pair(HashMap::new(),vec![
			(MochigomaKind::Fu,2),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),HashMap::new()),MochigomaCollections::Pair(vec![
			(MochigomaKind::Fu,2),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),HashMap::new()),false),
		(MochigomaCollections::Pair(HashMap::new(),vec![
			(MochigomaKind::Hisha,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),MochigomaCollections::Pair(HashMap::new(),vec![
			(MochigomaKind::Hisha,2),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),HashMap::new()),MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),HashMap::new()),false),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,1),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,1),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,2),
			(MochigomaKind::Kaku,2),
			(MochigomaKind::Kin,2),
			(MochigomaKind::Gin,2),
			(MochigomaKind::Kei,2),
			(MochigomaKind::Kyou,2),
			(MochigomaKind::Fu,9),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
	];

	for (i,(l,r,a)) in input_and_expected.into_iter().enumerate() {
		assert_eq!(l == r,a,"l = {:?}, r = {:?} index = {}",l,r,i);
	}
}
#[test]
fn test_mochigoma_collections_is_empty() {
	let input_and_expected:Vec<(MochigomaCollections,bool)> = vec![
		(MochigomaCollections::Empty,true),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,0),
			(MochigomaKind::Kaku,0),
			(MochigomaKind::Kin,0),
			(MochigomaKind::Gin,0),
			(MochigomaKind::Kei,0),
			(MochigomaKind::Kyou,0),
			(MochigomaKind::Fu,0),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,0),
			(MochigomaKind::Kaku,0),
			(MochigomaKind::Kin,0),
			(MochigomaKind::Gin,0),
			(MochigomaKind::Kei,0),
			(MochigomaKind::Kyou,0),
			(MochigomaKind::Fu,0),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),true),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Kaku,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Kin,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),(MochigomaCollections::Pair(vec![
			(MochigomaKind::Gin,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Kei,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Kyou,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Kaku,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Kin,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Gin,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Kei,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Kyou,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
		(MochigomaCollections::Pair(vec![
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})),false),
	];

	for (i,(mc,a)) in input_and_expected.into_iter().enumerate() {
		assert_eq!(mc.is_empty(),a,"index = {}",i);
	}
}