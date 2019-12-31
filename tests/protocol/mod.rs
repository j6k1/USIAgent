use std::collections::HashMap;

use usiagent::TryFrom;
use usiagent::shogi::*;
use usiagent::protocol::*;
use usiagent::error::*;
use usiagent::event::UsiInitialPosition;
use usiagent::rule;
use usiagent::rule::BANMEN_START_POS;

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
fn test_move_try_from() {
	let input_and_expected:Vec<(&'static str,Result<Move, TypeConvertError<String>>)> = vec![
		("",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (number of characters of move expression is invalid)")))),
		("Z*1a",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (The format of the move is illegal)"
		)))),
		("P",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (number of characters of move expression is invalid)")))),
		("P+",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (number of characters of move expression is invalid)")))),
		("P+1a",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN string (there no '*' after the name)")))),
		("P*",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (number of characters of move expression is invalid)")))),
		("P*1",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (number of characters of move expression is invalid)")))),
		("P*aa",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (The format of the move is illegal)"
		)))),
		("P*1j",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (The format of the move is illegal)"
		)))),
		("P*10",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (The format of the move is illegal)"
		)))),
		("P*0a",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (The format of the move is illegal)"
		)))),
		("R*1a",Ok(Move::Put(MochigomaKind::Hisha, KomaDstPutPosition(1,1)))),
		("B*2b",Ok(Move::Put(MochigomaKind::Kaku, KomaDstPutPosition(2,2)))),
		("G*3c",Ok(Move::Put(MochigomaKind::Kin, KomaDstPutPosition(3,3)))),
		("S*4d",Ok(Move::Put(MochigomaKind::Gin, KomaDstPutPosition(4,4)))),
		("N*5e",Ok(Move::Put(MochigomaKind::Kei, KomaDstPutPosition(5,5)))),
		("L*6f",Ok(Move::Put(MochigomaKind::Kyou, KomaDstPutPosition(6,6)))),
		("P*7g",Ok(Move::Put(MochigomaKind::Fu, KomaDstPutPosition(7,7)))),
		("P*8h",Ok(Move::Put(MochigomaKind::Fu, KomaDstPutPosition(8,8)))),
		("P*9i",Ok(Move::Put(MochigomaKind::Fu, KomaDstPutPosition(9,9)))),

		("1",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (number of characters of move expression is invalid)")))),
		("1a",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (number of characters of move expression is invalid)")))),
		("1a1",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (number of characters of move expression is invalid)")))),
		("0a1a",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (The format of the move is illegal)"
		)))),
		("111a",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (The format of the move is illegal)")))),
		("1aia",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (The format of the move is illegal)"
		)))),
		("1a19",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (The format of the move is illegal)"
		)))),
		("1a0a",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (The format of the move is illegal)"
		)))),
		("1a2b*",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (The format of the move is illegal)"
		)))),
		("1a9i",Ok(Move::To(KomaSrcPosition(1,1),KomaDstToPosition(9,9,false)))),
		("2b8h",Ok(Move::To(KomaSrcPosition(2,2),KomaDstToPosition(8,8,false)))),
		("3c7g",Ok(Move::To(KomaSrcPosition(3,3),KomaDstToPosition(7,7,false)))),
		("4d6f",Ok(Move::To(KomaSrcPosition(4,4),KomaDstToPosition(6,6,false)))),
		("5e1a",Ok(Move::To(KomaSrcPosition(5,5),KomaDstToPosition(1,1,false)))),
		("1a5e",Ok(Move::To(KomaSrcPosition(1,1),KomaDstToPosition(5,5,false)))),
		("6f4d",Ok(Move::To(KomaSrcPosition(6,6),KomaDstToPosition(4,4,false)))),
		("7g3c",Ok(Move::To(KomaSrcPosition(7,7),KomaDstToPosition(3,3,false)))),
		("8h2b",Ok(Move::To(KomaSrcPosition(8,8),KomaDstToPosition(2,2,false)))),
		("9i1a",Ok(Move::To(KomaSrcPosition(9,9),KomaDstToPosition(1,1,false)))),

		("1a9i+",Ok(Move::To(KomaSrcPosition(1,1),KomaDstToPosition(9,9,true)))),
		("2b8h+",Ok(Move::To(KomaSrcPosition(2,2),KomaDstToPosition(8,8,true)))),
		("3c7g+",Ok(Move::To(KomaSrcPosition(3,3),KomaDstToPosition(7,7,true)))),
		("4d6f+",Ok(Move::To(KomaSrcPosition(4,4),KomaDstToPosition(6,6,true)))),
		("5e1a+",Ok(Move::To(KomaSrcPosition(5,5),KomaDstToPosition(1,1,true)))),
		("1a5e+",Ok(Move::To(KomaSrcPosition(1,1),KomaDstToPosition(5,5,true)))),
		("6f4d+",Ok(Move::To(KomaSrcPosition(6,6),KomaDstToPosition(4,4,true)))),
		("7g3c+",Ok(Move::To(KomaSrcPosition(7,7),KomaDstToPosition(3,3,true)))),
		("8h2b+",Ok(Move::To(KomaSrcPosition(8,8),KomaDstToPosition(2,2,true)))),
		("9i1a+",Ok(Move::To(KomaSrcPosition(9,9),KomaDstToPosition(1,1,true)))),
	];

	for (i,e) in input_and_expected.into_iter() {
		assert_eq!(Move::try_from(&i),e);
	}
}
#[test]
fn test_komakind_try_from() {
	let input_and_expected:Vec<(&'static str,Result<KomaKind, TypeConvertError<String>>)> = vec![
		("K", Ok(KomaKind::SOu)),
		("R", Ok(KomaKind::SHisha)),
		("B", Ok(KomaKind::SKaku)),
		("G", Ok(KomaKind::SKin)),
		("S", Ok(KomaKind::SGin)),
		("N", Ok(KomaKind::SKei)),
		("L", Ok(KomaKind::SKyou)),
		("P", Ok(KomaKind::SFu)),
		("+R", Ok(KomaKind::SHishaN)),
		("+B", Ok(KomaKind::SKakuN)),
		("+N", Ok(KomaKind::SKeiN)),
		("+S", Ok(KomaKind::SGinN)),
		("+L", Ok(KomaKind::SKyouN)),
		("+P", Ok(KomaKind::SFuN)),
		("k", Ok(KomaKind::GOu)),
		("r", Ok(KomaKind::GHisha)),
		("b", Ok(KomaKind::GKaku)),
		("g", Ok(KomaKind::GKin)),
		("s", Ok(KomaKind::GGin)),
		("n", Ok(KomaKind::GKei)),
		("l", Ok(KomaKind::GKyou)),
		("p", Ok(KomaKind::GFu)),
		("+r", Ok(KomaKind::GHishaN)),
		("+b", Ok(KomaKind::GKakuN)),
		("+n", Ok(KomaKind::GKeiN)),
		("+s", Ok(KomaKind::GGinN)),
		("+l", Ok(KomaKind::GKyouN)),
		("+p", Ok(KomaKind::GFuN)),
		("a", Err(TypeConvertError::SyntaxError(String::from(
			"Invalid SFEN character string (a)"
		)))),
		("*p", Err(TypeConvertError::SyntaxError(String::from(
			"Invalid SFEN character string (*p)"
		)))),
	];

	for (i,e) in input_and_expected.into_iter() {
		assert_eq!(KomaKind::try_from(i.to_string()),e);
	}
}
#[test]
fn test_teban_try_from() {
	let input_and_expected:Vec<(&'static str,Result<Teban, TypeConvertError<String>>)> = vec![
		("b", Ok(Teban::Sente)),
		("w", Ok(Teban::Gote)),
		("a", Err(TypeConvertError::SyntaxError(String::from(
			"It is an illegal character string as a character string representing a turn."
		))))
	];

	for (i,e) in input_and_expected.into_iter() {
		assert_eq!(Teban::try_from(i),e);
	}
}
#[test]
fn test_banmen_try_from() {
	let input_and_expected:Vec<(&'static str,Result<Banmen, TypeConvertError<String>>)> = vec![
		("+l+n+sgkgsnl/1+r5+b1/+p+p+p+p+ppppp/9/9/9/PPPP+P+P+P+P+P/1B5R1/LNSGKG+S+N+L",Ok(Banmen([
			[GKyouN,GKeiN,GGinN,GKin,GOu,GKin,GGin,GKei,GKyou],
			[Blank,GHishaN,Blank,Blank,Blank,Blank,Blank,GKakuN,Blank],
			[GFuN,GFuN,GFuN,GFuN,GFuN,GFu,GFu,GFu,GFu],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[SFu,SFu,SFu,SFu,SFuN,SFuN,SFuN,SFuN,SFuN],
			[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
			[SKyou,SKei,SGin,SKin,SOu,SKin,SGinN,SKeiN,SKyouN]
		]))),
		("+l+n+sgkgsnl/1r5b1/+p+p+p+p+ppppp/9/9/9/PPPP+P+P+P+P+P/1+B5+R1/LNSGKG+S+N+L",Ok(Banmen([
			[GKyouN,GKeiN,GGinN,GKin,GOu,GKin,GGin,GKei,GKyou],
			[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
			[GFuN,GFuN,GFuN,GFuN,GFuN,GFu,GFu,GFu,GFu],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[SFu,SFu,SFu,SFu,SFuN,SFuN,SFuN,SFuN,SFuN],
			[Blank,SKakuN,Blank,Blank,Blank,Blank,Blank,SHishaN,Blank],
			[SKyou,SKei,SGin,SKin,SOu,SKin,SGinN,SKeiN,SKyouN]
		]))),
		("p8/1p7/2p6/3p5/4p4/5p3/6p2/7p1/8p",Ok(Banmen([
			[GFu,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,GFu,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,GFu,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,GFu,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,GFu,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,GFu,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,GFu,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GFu,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,GFu]
		]))),

		("+",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (illegal expression of piece)")))),
		("P+",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (illegal expression of piece)")))),
		("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/+",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (illegal expression of piece)")))),
		("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPP1/1B5R1/P+",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (illegal expression of piece)")))),
		("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNa",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (a)")))),
		("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSN+a",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (+a)")))),
		("lnsgkgsnl/1r5b1/ppppppppp/0/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (0 is specified for the number of blank)")))),
		("lnsgkgsnl/1r5b1/ppppppp2/p0p7/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (0 is specified for the number of blank)")))),
		("lnsgkgsnl/1r5b1/pppppppp2/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (pieces outside the range of the board)")))),
		("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPP1P/1B5R1/LNSGKGSNL",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN string (line separator '/' not found)")))),
		("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPP2/1B5R1/LNSGKGSNL/PP7",Err(TypeConvertError::SyntaxError(String::from(
		"Invalid SFEN character string (pieces outside the range of the board)")))),
	];

	for (i,e) in input_and_expected.into_iter() {
		assert_eq!(Banmen::try_from(&i),e);
	}
}
#[test]
fn test_mochigomacollections_try_from() {
	let input_and_expected:Vec<(&'static str,Result<MochigomaCollections, TypeConvertError<String>>)> = vec![
		("-",Ok(MochigomaCollections::Pair(HashMap::new(),HashMap::new()))),
		("2R2B2G2S2N2L9P2r2b2g2s2n2l9p",Ok(MochigomaCollections::Pair(vec![
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
		})))),
		("4R4B4G4S4N4L18P",Ok(MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,4),
			(MochigomaKind::Kaku,4),
			(MochigomaKind::Kin,4),
			(MochigomaKind::Gin,4),
			(MochigomaKind::Kei,4),
			(MochigomaKind::Kyou,4),
			(MochigomaKind::Fu,18),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),HashMap::new()))),
		("4r4b4g4s4n4l18p",Ok(MochigomaCollections::Pair(HashMap::new(), vec![
			(MochigomaKind::Hisha,4),
			(MochigomaKind::Kaku,4),
			(MochigomaKind::Kin,4),
			(MochigomaKind::Gin,4),
			(MochigomaKind::Kei,4),
			(MochigomaKind::Kyou,4),
			(MochigomaKind::Fu,18),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})))),
		("RBGSNLPrbgsnlp",Ok(MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,1),
			(MochigomaKind::Kaku,1),
			(MochigomaKind::Kin,1),
			(MochigomaKind::Gin,1),
			(MochigomaKind::Kei,1),
			(MochigomaKind::Kyou,1),
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),vec![
			(MochigomaKind::Hisha,1),
			(MochigomaKind::Kaku,1),
			(MochigomaKind::Kin,1),
			(MochigomaKind::Gin,1),
			(MochigomaKind::Kei,1),
			(MochigomaKind::Kyou,1),
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})))),
		("RBGSNLP",Ok(MochigomaCollections::Pair(vec![
			(MochigomaKind::Hisha,1),
			(MochigomaKind::Kaku,1),
			(MochigomaKind::Kin,1),
			(MochigomaKind::Gin,1),
			(MochigomaKind::Kei,1),
			(MochigomaKind::Kyou,1),
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		}),HashMap::new()))),
		("rbgsnlp",Ok(MochigomaCollections::Pair(HashMap::new(), vec![
			(MochigomaKind::Hisha,1),
			(MochigomaKind::Kaku,1),
			(MochigomaKind::Kin,1),
			(MochigomaKind::Gin,1),
			(MochigomaKind::Kei,1),
			(MochigomaKind::Kyou,1),
			(MochigomaKind::Fu,1),
		].into_iter().fold(HashMap::new(), |mut acc,(k,n)| {
			acc.insert(k,n);
			acc
		})))),

		("0P", Err(TypeConvertError::SyntaxError(String::from(
			"Invalid SFEN character string (the number of pieces is illegal.)."
		)))),
		("1P", Err(TypeConvertError::SyntaxError(String::from(
			"Invalid SFEN character string (the number of pieces is illegal.)."
		)))),
		("A", Err(TypeConvertError::SyntaxError(String::from(
			"Invalid SFEN character string (illegal representation character string of the piece)"
		)))),
		("2A", Err(TypeConvertError::SyntaxError(String::from(
			"Invalid SFEN character string (illegal representation character string of the piece)"
		)))),
		("2", Err(TypeConvertError::SyntaxError(String::from(
			"Invalid SFEN character string (The type of piece is empty)"
		)))),
	];

	for (i,e) in input_and_expected.into_iter() {
		assert_eq!(MochigomaCollections::try_from(i),e);
	}
}
#[test]
fn test_position_parser_parse() {
	let input_and_expected:Vec<(&'static [&'static str],Result<PositionParseResult,TypeConvertError<String>>)> = vec![
		(&["startpos"],Ok(PositionParseResult(Teban::Sente,UsiInitialPosition::Startpos,1,vec![]))),
		(&["startpos","moves","1g1f"],Ok(PositionParseResult(Teban::Sente,UsiInitialPosition::Startpos,1,vec![
			Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))
		]))),
		(&["startpos","moves","7g7f","3c3d","8h3c+"],Ok(PositionParseResult(Teban::Sente,UsiInitialPosition::Startpos,1,vec![
			Move::To(KomaSrcPosition(7,7),KomaDstToPosition(7,6,false)),
			Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false)),
			Move::To(KomaSrcPosition(8,8),KomaDstToPosition(3,3,true)),
		]))),

		(&["sfen","lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL","w","-","1"],
			Ok(PositionParseResult(Teban::Gote,
				UsiInitialPosition::Sfen(Banmen([
					[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
					[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
					[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
					[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
					[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
					[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
					[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
					[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
					[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
				]),MochigomaCollections::Empty),
				1,vec![])
			)
		),
		(&["sfen","lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL","w","-","1","moves","3c3d"],
			Ok(PositionParseResult(Teban::Gote,
				UsiInitialPosition::Sfen(Banmen([
					[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
					[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
					[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
					[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
					[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
					[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
					[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
					[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
					[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
				]),MochigomaCollections::Empty),
				1,vec![
					Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false))
				])
			)
		),
		(&["sfen","lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL","w","-","1","moves","3c3d","7g7f","2b7g"],
			Ok(PositionParseResult(Teban::Gote,
				UsiInitialPosition::Sfen(Banmen([
					[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
					[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
					[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
					[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
					[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
					[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
					[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
					[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
					[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
				]),MochigomaCollections::Empty),
				1,vec![
					Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false)),
					Move::To(KomaSrcPosition(7,7),KomaDstToPosition(7,6,false)),
					Move::To(KomaSrcPosition(2,2),KomaDstToPosition(7,7,false))
				])
			)
		),

		(&[],Err(TypeConvertError::SyntaxError(String::from(
			"The format of the position command input is invalid."
		)))),
		(&["hoge"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the position command is invalid. (Insufficient parameters)"
		)))),
		(&["startpos","hoge"],Err(TypeConvertError::SyntaxError(String::from(
			"The format of the position command input is invalid."
		)))),
		(&["startpos","hoge","1g1f"],Err(TypeConvertError::SyntaxError(String::from(
			"The format of the position command input is invalid."
		)))),
		(&["startpos","moves"],Err(TypeConvertError::SyntaxError(String::from(
			"The format of the position command input is invalid."
		)))),
		(&["sfen","lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL","w","-","1","hoge"],
			Err(TypeConvertError::SyntaxError(String::from(
			"The format of the position command input is invalid."
		)))),
		(&["sfen","lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL","w","-","1","hoge","1g1f"],
			Err(TypeConvertError::SyntaxError(String::from(
			"The format of the position command input is invalid."
		)))),
		(&["sfen","lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL","w","-","1","moves"],
			Err(TypeConvertError::SyntaxError(String::from(
			"The format of the position command input is invalid."
		)))),
		(&["sfen","lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL","w","-"],
			Err(TypeConvertError::SyntaxError(String::from(
			"The format of the position command input is invalid."
		)))),
	];

	let parser = PositionParser::new();

	for (i,e) in input_and_expected.into_iter() {
		assert_eq!(parser.parse(i),e);
	}
}
