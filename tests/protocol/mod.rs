use std::collections::HashMap;

use usiagent::TryFrom;
use usiagent::shogi::*;
use usiagent::protocol::*;
use usiagent::error::*;
use usiagent::event::*;
use usiagent::command::*;

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

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(Move::try_from(&i),r);
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

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(KomaKind::try_from(i),r);
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

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(Teban::try_from(i),r);
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

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(Banmen::try_from(&i),r);
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

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(MochigomaCollections::try_from(i),r);
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
		(&["sfen","lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL","w","-","1","moves","3c3d","7g7f","2b7g+"],
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
					Move::To(KomaSrcPosition(2,2),KomaDstToPosition(7,7,true))
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

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(parser.parse(i),r);
	}
}
#[test]
fn test_go_parser_parse() {
	let input_and_expected:Vec<(&'static [&'static str],Result<UsiGo,TypeConvertError<String>>)> = vec![
		(&["mate","infinite"],Ok(UsiGo::Mate(UsiGoMateTimeLimit::Infinite))),
		(&["mate","1"],Ok(UsiGo::Mate(UsiGoMateTimeLimit::Limit(1)))),
		(&["mate"],Ok(UsiGo::Mate(UsiGoMateTimeLimit::None))),
		(&["infinite"],Ok(UsiGo::Go(UsiGoTimeLimit::Infinite))),

		(&["btime","1","wtime","2"],Ok(UsiGo::Go(UsiGoTimeLimit::Limit(Some((1,2)),None)))),
		(&["wtime","2","btime","1"],Ok(UsiGo::Go(UsiGoTimeLimit::Limit(Some((1,2)),None)))),

		(&["binc","1","winc","2"],Ok(UsiGo::Go(UsiGoTimeLimit::Limit(None,Some(UsiGoByoyomiOrInc::Inc(1,2)))))),
		(&["winc","2","binc","1"],Ok(UsiGo::Go(UsiGoTimeLimit::Limit(None,Some(UsiGoByoyomiOrInc::Inc(1,2)))))),

		(&["byoyomi","1"],Ok(UsiGo::Go(UsiGoTimeLimit::Limit(None,Some(UsiGoByoyomiOrInc::Byoyomi(1)))))),

		(&["btime","1","wtime","2","binc","3","winc","4"],Ok(UsiGo::Go(UsiGoTimeLimit::Limit(Some((1,2)),Some(UsiGoByoyomiOrInc::Inc(3,4)))))),
		(&["wtime","2","btime","1","winc","4","binc","3"],Ok(UsiGo::Go(UsiGoTimeLimit::Limit(Some((1,2)),Some(UsiGoByoyomiOrInc::Inc(3,4)))))),

		(&["btime","1","wtime","2","byoyomi","3"],Ok(UsiGo::Go(UsiGoTimeLimit::Limit(Some((1,2)),Some(UsiGoByoyomiOrInc::Byoyomi(3)))))),
		(&["wtime","2","btime","1","byoyomi","3"],Ok(UsiGo::Go(UsiGoTimeLimit::Limit(Some((1,2)),Some(UsiGoByoyomiOrInc::Byoyomi(3)))))),

		(&["ponder","btime","1","wtime","2"],Ok(UsiGo::Ponder(UsiGoTimeLimit::Limit(Some((1,2)),None)))),
		(&["ponder","wtime","2","btime","1"],Ok(UsiGo::Ponder(UsiGoTimeLimit::Limit(Some((1,2)),None)))),

		(&["ponder","binc","1","winc","2"],Ok(UsiGo::Ponder(UsiGoTimeLimit::Limit(None,Some(UsiGoByoyomiOrInc::Inc(1,2)))))),
		(&["ponder","winc","2","binc","1"],Ok(UsiGo::Ponder(UsiGoTimeLimit::Limit(None,Some(UsiGoByoyomiOrInc::Inc(1,2)))))),

		(&["ponder","byoyomi","1"],Ok(UsiGo::Ponder(UsiGoTimeLimit::Limit(None,Some(UsiGoByoyomiOrInc::Byoyomi(1)))))),

		(&["ponder","btime","1","wtime","2","binc","3","winc","4"],Ok(UsiGo::Ponder(UsiGoTimeLimit::Limit(Some((1,2)),Some(UsiGoByoyomiOrInc::Inc(3,4)))))),
		(&["ponder","wtime","2","btime","1","winc","4","binc","3"],Ok(UsiGo::Ponder(UsiGoTimeLimit::Limit(Some((1,2)),Some(UsiGoByoyomiOrInc::Inc(3,4)))))),

		(&["ponder","btime","1","wtime","2","byoyomi","3"],Ok(UsiGo::Ponder(UsiGoTimeLimit::Limit(Some((1,2)),Some(UsiGoByoyomiOrInc::Byoyomi(3)))))),
		(&["ponder","wtime","2","btime","1","byoyomi","3"],Ok(UsiGo::Ponder(UsiGoTimeLimit::Limit(Some((1,2)),Some(UsiGoByoyomiOrInc::Byoyomi(3)))))),

		(&["mate","1","2"],Err(TypeConvertError::SyntaxError(String::from(
			"The format of the position command input is invalid. (go mate has too many parameters)"
		)))),
		(&["infinite","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The format of the position command input is invalid."
		)))),
		(&["btime"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (There is no value for item)"
		)))),
		(&["btime","a"],Err(TypeConvertError::SyntaxError(String::from(
			"Failed parse string to integer."
		)))),
		(&["btime","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Insufficient parameters)"
		)))),
		(&["btime","1","btime","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Unexpected parameter 'btime')"
		)))),
		(&["btime","1","wtime"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (There is no value for item)"
		)))),
		(&["btime","1","wtime","a"],Err(TypeConvertError::SyntaxError(String::from(
			"Failed parse string to integer."
		)))),
		(&["btime","1","wtime","1","btime","1","wtime","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Duplicate parameters)"
		)))),
		(&["binc"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (There is no value for item)"
		)))),
		(&["binc","a"],Err(TypeConvertError::SyntaxError(String::from(
			"Failed parse string to integer."
		)))),
		(&["binc","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Insufficient parameters)"
		)))),
		(&["binc","1","binc","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Unexpected parameter 'binc')"
		)))),
		(&["binc","1","winc"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (There is no value for item)"
		)))),
		(&["binc","1","winc","a"],Err(TypeConvertError::SyntaxError(String::from(
			"Failed parse string to integer."
		)))),
		(&["binc","1","winc","1","binc","1","winc","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Duplicate parameters)"
		)))),
		(&["binc","1","winc","1","byoyomi","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Duplicate parameters)"
		)))),
		(&["byoyomi"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (There is no value for item)"
		)))),
		(&["byoyomi","a"],Err(TypeConvertError::SyntaxError(String::from(
			"Failed parse string to integer."
		)))),
		(&["byoyomi","1","byoyomi","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Duplicate parameters)"
		)))),
		(&["byoyomi","1","binc","1","winc","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Duplicate parameters)"
		)))),
		(&["hoge","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Unknown parameter)"
		)))),
		(&["ponder","infinite","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The format of the position command input is invalid."
		)))),
		(&["ponder","btime"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (There is no value for item)"
		)))),
		(&["ponder","btime","a"],Err(TypeConvertError::SyntaxError(String::from(
			"Failed parse string to integer."
		)))),
		(&["ponder","btime","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Insufficient parameters)"
		)))),
		(&["ponder","btime","1","btime","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Unexpected parameter 'btime')"
		)))),
		(&["ponder","btime","1","wtime"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (There is no value for item)"
		)))),
		(&["ponder","btime","1","wtime","a"],Err(TypeConvertError::SyntaxError(String::from(
			"Failed parse string to integer."
		)))),
		(&["ponder","btime","1","wtime","1","btime","1","wtime","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Duplicate parameters)"
		)))),
		(&["ponder","binc"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (There is no value for item)"
		)))),
		(&["ponder","binc","a"],Err(TypeConvertError::SyntaxError(String::from(
			"Failed parse string to integer."
		)))),
		(&["ponder","binc","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Insufficient parameters)"
		)))),
		(&["ponder","binc","1","binc","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Unexpected parameter 'binc')"
		)))),
		(&["ponder","binc","1","winc"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (There is no value for item)"
		)))),
		(&["ponder","binc","1","winc","a"],Err(TypeConvertError::SyntaxError(String::from(
			"Failed parse string to integer."
		)))),
		(&["ponder","binc","1","winc","1","binc","1","winc","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Duplicate parameters)"
		)))),
		(&["ponder","binc","1","winc","1","byoyomi","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Duplicate parameters)"
		)))),
		(&["ponder","byoyomi"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (There is no value for item)"
		)))),
		(&["ponder","byoyomi","a"],Err(TypeConvertError::SyntaxError(String::from(
			"Failed parse string to integer."
		)))),
		(&["ponder","byoyomi","1","byoyomi","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Duplicate parameters)"
		)))),
		(&["ponder","byoyomi","1","binc","1","winc","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Duplicate parameters)"
		)))),
		(&["ponder","hoge","1"],Err(TypeConvertError::SyntaxError(String::from(
			"The input form of the go command is invalid. (Unknown parameter)"
		)))),
	];

	let parser = GoParser::new();

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(parser.parse(i),r);
	}
}
#[test]
fn test_banmen_to_sfen() {
	let input_and_expected:Vec<(Banmen,Result<String,TypeConvertError<String>>)> = vec![
		(Banmen([
			[GKyouN,GKeiN,GGinN,GKin,GOu,GKin,GGin,GKei,GKyou],
			[Blank,GHishaN,Blank,Blank,Blank,Blank,Blank,GKakuN,Blank],
			[GFuN,GFuN,GFuN,GFuN,GFuN,GFu,GFu,GFu,GFu],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[SFu,SFu,SFu,SFu,SFuN,SFuN,SFuN,SFuN,SFuN],
			[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
			[SKyou,SKei,SGin,SKin,SOu,SKin,SGinN,SKeiN,SKyouN]
		]),Ok(String::from("+l+n+sgkgsnl/1+r5+b1/+p+p+p+p+ppppp/9/9/9/PPPP+P+P+P+P+P/1B5R1/LNSGKG+S+N+L"))),
		(Banmen([
			[GKyouN,GKeiN,GGinN,GKin,GOu,GKin,GGin,GKei,GKyou],
			[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
			[GFuN,GFuN,GFuN,GFuN,GFuN,GFu,GFu,GFu,GFu],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[SFu,SFu,SFu,SFu,SFuN,SFuN,SFuN,SFuN,SFuN],
			[Blank,SKakuN,Blank,Blank,Blank,Blank,Blank,SHishaN,Blank],
			[SKyou,SKei,SGin,SKin,SOu,SKin,SGinN,SKeiN,SKyouN]
		]),Ok(String::from("+l+n+sgkgsnl/1r5b1/+p+p+p+p+ppppp/9/9/9/PPPP+P+P+P+P+P/1+B5+R1/LNSGKG+S+N+L"))),
		(Banmen([
			[GFu,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,GFu,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,GFu,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,GFu,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,GFu,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,GFu,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,GFu,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,GFu,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,GFu]
		]),Ok(String::from("p8/1p7/2p6/3p5/4p4/5p3/6p2/7p1/8p"))),
	];

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(i.to_sfen(),r);
	}
}
#[test]
fn test_teban_to_sfen() {
	let input_and_expected:Vec<(Teban,Result<String,TypeConvertError<String>>)> = vec![
		(Teban::Sente,Ok(String::from("b"))),
		(Teban::Gote,Ok(String::from("w"))),
	];

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(i.to_sfen(),r);
	}
}
#[test]
fn test_mochigomacollections_to_sfen() {
	let input_and_expected:Vec<(MochigomaCollections,Result<String, TypeConvertError<String>>)> = vec![
		(MochigomaCollections::Pair(HashMap::new(),HashMap::new()),Ok(String::from("-"))),
		(MochigomaCollections::Empty,Ok(String::from("-"))),
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
		})),Ok(String::from("2R2B2G2S2N2L9P2r2b2g2s2n2l9p"))),
		(MochigomaCollections::Pair(vec![
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
		}),HashMap::new()),Ok(String::from("4R4B4G4S4N4L18P"))),
		(MochigomaCollections::Pair(HashMap::new(), vec![
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
		})),Ok(String::from("4r4b4g4s4n4l18p"))),
		(MochigomaCollections::Pair(vec![
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
		})),Ok(String::from("RBGSNLPrbgsnlp"))),
		(MochigomaCollections::Pair(vec![
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
		}),HashMap::new()),Ok(String::from("RBGSNLP"))),
		(MochigomaCollections::Pair(HashMap::new(), vec![
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
		})),Ok(String::from("rbgsnlp"))),
	];

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(i.to_sfen(),r);
	}
}
#[test]
fn test_teban_banmen_mc_moves_to_sfen() {
	let input_and_expected:Vec<(
		(Teban,Banmen,MochigomaCollections,Vec<Move>),Result<String,SfenStringConvertError>)> = vec![

		((Teban::Sente,Banmen([
			[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
			[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
			[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
			[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
			[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
		]),MochigomaCollections::Empty,vec![]),Ok(String::from("startpos"))),
		((Teban::Sente,Banmen([
			[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
			[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
			[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
			[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
			[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
		]),MochigomaCollections::Pair(HashMap::new(),HashMap::new()),vec![]),Ok(String::from("startpos"))),
		((Teban::Gote,Banmen([
			[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
			[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
			[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
			[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
			[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
		]),MochigomaCollections::Pair(HashMap::new(),HashMap::new()),vec![]),
		Ok(String::from("sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1"))),
		((Teban::Sente,Banmen([
			[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
			[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
			[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
			[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
			[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
		]),MochigomaCollections::Empty,vec![
			Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))
		]),Ok(String::from("startpos moves 1g1f"))),
		((Teban::Sente,Banmen([
			[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
			[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
			[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
			[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
			[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
		]),MochigomaCollections::Pair(HashMap::new(),HashMap::new()),vec![
			Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))
		]),Ok(String::from("startpos moves 1g1f"))),
		((Teban::Sente,Banmen([
			[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
			[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
			[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
			[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
			[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
		]),MochigomaCollections::Empty,vec![
			Move::To(KomaSrcPosition(7,7),KomaDstToPosition(7,6,false)),
			Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false)),
			Move::To(KomaSrcPosition(8,8),KomaDstToPosition(3,3,true)),
		]),Ok(String::from("startpos moves 7g7f 3c3d 8h3c+"))),
		((Teban::Sente,Banmen([
			[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
			[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
			[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
			[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
			[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
		]),MochigomaCollections::Pair(HashMap::new(),HashMap::new()),vec![
			Move::To(KomaSrcPosition(7,7),KomaDstToPosition(7,6,false)),
			Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false)),
			Move::To(KomaSrcPosition(8,8),KomaDstToPosition(3,3,true)),
		]),Ok(String::from("startpos moves 7g7f 3c3d 8h3c+"))),
		((Teban::Gote,Banmen([
			[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
			[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
			[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
			[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
			[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
		]),MochigomaCollections::Pair(HashMap::new(),HashMap::new()),vec![
			Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false))
		]),
		Ok(String::from("sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1 moves 3c3d"))),
		((Teban::Gote,Banmen([
			[GKyou,GKei,GGin,GKin,GOu,GKin,GGin,GKei,GKyou],
			[Blank,GHisha,Blank,Blank,Blank,Blank,Blank,GKaku,Blank],
			[GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu,GFu],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank,Blank],
			[SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu,SFu],
			[Blank,SKaku,Blank,Blank,Blank,Blank,Blank,SHisha,Blank],
			[SKyou,SKei,SGin,SKin,SOu,SKin,SGin,SKei,SKyou],
		]),MochigomaCollections::Pair(HashMap::new(),HashMap::new()),vec![
			Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false)),
			Move::To(KomaSrcPosition(7,7),KomaDstToPosition(7,6,false)),
			Move::To(KomaSrcPosition(2,2),KomaDstToPosition(7,7,true))
		]),
		Ok(String::from("sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1 moves 3c3d 7g7f 2b7g+"))),
	];

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(i.to_sfen(),r);
	}
}
#[test]
fn test_checkmate_to_sfen() {
	let input_and_expected:Vec<(CheckMate,Result<String,UsiOutputCreateError>)> = vec![
		(CheckMate::Moves(vec![
			Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))
		]),Ok(String::from("1g1f"))),
		(CheckMate::Moves(vec![
			Move::To(KomaSrcPosition(7,7),KomaDstToPosition(7,6,false)),
			Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false)),
			Move::To(KomaSrcPosition(8,8),KomaDstToPosition(3,3,true))
		]),Ok(String::from("7g7f 3c3d 8h3c+"))),
		(CheckMate::NotiImplemented,Ok(String::from("notimplemented"))),
		(CheckMate::Timeout,Ok(String::from("timeout"))),
		(CheckMate::Nomate,Ok(String::from("nomate"))),

		(CheckMate::Abort,Err(UsiOutputCreateError::AbortedError)),
		(CheckMate::Moves(vec![]),Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))),
		(CheckMate::Moves(vec![
			Move::To(KomaSrcPosition(10,1),KomaDstToPosition(9,1,false))
		]),Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))),
		(CheckMate::Moves(vec![
			Move::To(KomaSrcPosition(1,10),KomaDstToPosition(2,10,false))
		]),Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))),
	];

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(i.to_sfen(),r);
	}
}
#[test]
fn test_bestmove_to_sfen() {
	let input_and_expected:Vec<(BestMove,Result<String,ToMoveStringConvertError>)> = vec![
		(BestMove::Move(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),None),Ok(String::from("1g1f"))),
		(BestMove::Move(Move::To(KomaSrcPosition(7,7),KomaDstToPosition(7,6,false)),
			Some(Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false)))
		),Ok(String::from("7g7f ponder 3c3d"))),
		(BestMove::Resign,Ok(String::from("resign"))),
		(BestMove::Win,Ok(String::from("win"))),

		(BestMove::Abort,Err(ToMoveStringConvertError::AbortedError)),
	];

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(i.to_sfen(),r);
	}
}
#[test]
fn test_usiinfo_subcommand_vec_to_usi_command() {
	let input_and_expected:Vec<(Vec<UsiInfoSubCommand>,Result<String,UsiOutputCreateError>)> = vec![
		(vec![
			UsiInfoSubCommand::Depth(1),
			UsiInfoSubCommand::SelDepth(3),
			UsiInfoSubCommand::Time(10000),
			UsiInfoSubCommand::Nodes(1000000),
			UsiInfoSubCommand::Pv(vec![
				Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
				Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),
				Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false))
			]),
			UsiInfoSubCommand::MultiPv(1),
			UsiInfoSubCommand::Score(UsiScore::Cp(-100)),
			UsiInfoSubCommand::CurMove(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))),
			UsiInfoSubCommand::Hashfull(10000),
			UsiInfoSubCommand::Nps(100)
		],Ok(String::from("depth 1 seldepth 3 time 10000 nodes 1000000 score cp -100 curmove 1g1f hashfull 10000 nps 100 multipv 1 pv 1g1f 9c9d 1f1e"))),
		(vec![
			UsiInfoSubCommand::Depth(1),
			UsiInfoSubCommand::SelDepth(3),
			UsiInfoSubCommand::Time(10000),
			UsiInfoSubCommand::Nodes(1000000),
			UsiInfoSubCommand::Pv(vec![
				Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
				Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),
				Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false))
			]),
			UsiInfoSubCommand::Score(UsiScore::Cp(-100)),
			UsiInfoSubCommand::CurMove(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))),
			UsiInfoSubCommand::Hashfull(10000),
			UsiInfoSubCommand::Nps(100)
		],Ok(String::from("depth 1 seldepth 3 time 10000 nodes 1000000 score cp -100 curmove 1g1f hashfull 10000 nps 100 pv 1g1f 9c9d 1f1e"))),
		(vec![
			UsiInfoSubCommand::Depth(1),
			UsiInfoSubCommand::SelDepth(3),
			UsiInfoSubCommand::Time(10000),
			UsiInfoSubCommand::Nodes(1000000),
			UsiInfoSubCommand::Str(String::from("hellow!")),
			UsiInfoSubCommand::Score(UsiScore::Cp(-100)),
			UsiInfoSubCommand::CurMove(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))),
			UsiInfoSubCommand::Hashfull(10000),
			UsiInfoSubCommand::Nps(100)
		],Ok(String::from("depth 1 seldepth 3 time 10000 nodes 1000000 string hellow! score cp -100 curmove 1g1f hashfull 10000 nps 100"))),
		(vec![
			UsiInfoSubCommand::Pv(vec![
				Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
				Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),
				Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false))
			]),
			UsiInfoSubCommand::Str(String::from("hellow!")),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"specified pv and str with together"
		)))),
		(vec![
			UsiInfoSubCommand::Str(String::from("hellow!")),
			UsiInfoSubCommand::Pv(vec![
				Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
				Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),
				Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false))
			]),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"specified pv and str with together"
		)))),
		(vec![
			UsiInfoSubCommand::SelDepth(3),
			UsiInfoSubCommand::Depth(1),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"seldepth must be specified immediately after depth"
		)))),
		(vec![
			UsiInfoSubCommand::Depth(1),
			UsiInfoSubCommand::Nodes(1),
			UsiInfoSubCommand::SelDepth(3),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"seldepth must be specified immediately after depth"
		)))),
		(vec![
			UsiInfoSubCommand::SelDepth(3),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"seldepth must be specified immediately after depth"
		)))),
		(vec![
			UsiInfoSubCommand::Pv(vec![
			]),
		],Err(UsiOutputCreateError::InvalidStateError(String::from("pv")))),
		(vec![
			UsiInfoSubCommand::Pv(vec![
				Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
				Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),
				Move::To(KomaSrcPosition(1,6),KomaDstToPosition(10,7,false))
			]),
		],Err(UsiOutputCreateError::InvalidStateError(String::from("pv")))),
		(vec![
			UsiInfoSubCommand::CurMove(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(10,8,false)))
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"parameter of curmove is invalid"
		)))),
		(vec![
			UsiInfoSubCommand::Depth(1),
			UsiInfoSubCommand::Depth(2),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"The same subcommand is specified more than once"
		)))),
		(vec![
			UsiInfoSubCommand::Depth(1),
			UsiInfoSubCommand::SelDepth(1),
			UsiInfoSubCommand::Nodes(1),
			UsiInfoSubCommand::SelDepth(2),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"The same subcommand is specified more than once"
		)))),
		(vec![
			UsiInfoSubCommand::Time(1),
			UsiInfoSubCommand::Time(2),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"The same subcommand is specified more than once"
		)))),
		(vec![
			UsiInfoSubCommand::Nodes(1),
			UsiInfoSubCommand::Nodes(2),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"The same subcommand is specified more than once"
		)))),
		(vec![
			UsiInfoSubCommand::Score(UsiScore::Cp(-100)),
			UsiInfoSubCommand::Score(UsiScore::CpUpper(100)),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"The same subcommand is specified more than once"
		)))),
		(vec![
			UsiInfoSubCommand::CurMove(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))),
			UsiInfoSubCommand::CurMove(Move::To(KomaSrcPosition(9,7),KomaDstToPosition(9,6,false))),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"The same subcommand is specified more than once"
		)))),
		(vec![
			UsiInfoSubCommand::Hashfull(1),
			UsiInfoSubCommand::Hashfull(2),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"The same subcommand is specified more than once"
		)))),
		(vec![
			UsiInfoSubCommand::Nps(1),
			UsiInfoSubCommand::Nps(2),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"The same subcommand is specified more than once"
		)))),
		(vec![
			UsiInfoSubCommand::Str(String::from("hellow!")),
			UsiInfoSubCommand::Str(String::from("hellow!!!!!")),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"The same subcommand is specified more than once"
		)))),
		(vec![
			UsiInfoSubCommand::Pv(vec![
				Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
				Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),
				Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false))
			]),
			UsiInfoSubCommand::Pv(vec![
				Move::To(KomaSrcPosition(7,7),KomaDstToPosition(7,6,false)),
			]),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"The same subcommand is specified more than once"
		)))),
		(vec![
			UsiInfoSubCommand::MultiPv(1),
			UsiInfoSubCommand::MultiPv(2),
			UsiInfoSubCommand::Pv(vec![
				Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
				Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),
				Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false))
			]),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"The same subcommand is specified more than once"
		)))),
		(vec![
			UsiInfoSubCommand::MultiPv(1),
		],Err(UsiOutputCreateError::InvalidInfoCommand(String::from(
			"multipv must be specified along with pv"
		)))),
	];

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(i.to_usi_command(),r);
	}
}
#[test]
fn test_usiinfo_subcommand_to_usi_command() {
	let input_and_expected:Vec<(UsiInfoSubCommand,Result<String,UsiOutputCreateError>)> = vec![
		(UsiInfoSubCommand::Depth(1),Ok(String::from("depth 1"))),
		(UsiInfoSubCommand::SelDepth(1),Ok(String::from("seldepth 1"))),
		(UsiInfoSubCommand::Time(100),Ok(String::from("time 100"))),
		(UsiInfoSubCommand::Nodes(1000),Ok(String::from("nodes 1000"))),
		(UsiInfoSubCommand::MultiPv(1),Ok(String::from("multipv 1"))),
		(UsiInfoSubCommand::Pv(vec![
			Move::To(KomaSrcPosition(7,7),KomaDstToPosition(7,6,false)),
		]),Ok(String::from("pv 7g7f"))),
		(UsiInfoSubCommand::Pv(vec![
			Move::To(KomaSrcPosition(7,7),KomaDstToPosition(7,6,false)),
			Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false)),
			Move::To(KomaSrcPosition(8,8),KomaDstToPosition(3,3,true))
		]),Ok(String::from("pv 7g7f 3c3d 8h3c+"))),
		(UsiInfoSubCommand::Score(UsiScore::Cp(1000)),Ok(String::from("score cp 1000"))),
		(UsiInfoSubCommand::Score(UsiScore::CpUpper(1000)),Ok(String::from("score cp 1000 upperbound"))),
		(UsiInfoSubCommand::Score(UsiScore::CpLower(1000)),Ok(String::from("score cp 1000 lowerbound"))),
		(UsiInfoSubCommand::Score(UsiScore::Mate(UsiScoreMate::Num(1000))),Ok(String::from("score mate 1000"))),
		(UsiInfoSubCommand::Score(UsiScore::Mate(UsiScoreMate::Plus)),Ok(String::from("score mate +"))),
		(UsiInfoSubCommand::Score(UsiScore::Mate(UsiScoreMate::Minus)),Ok(String::from("score mate -"))),
		(UsiInfoSubCommand::Score(UsiScore::MateUpper(1000)),Ok(String::from("score mate 1000 upperbound"))),
		(UsiInfoSubCommand::Score(UsiScore::MateLower(1000)),Ok(String::from("score mate 1000 lowerbound"))),
		(UsiInfoSubCommand::CurMove(Move::To(KomaSrcPosition(7,7),KomaDstToPosition(7,6,false)))
		,Ok(String::from("curmove 7g7f"))),
		(UsiInfoSubCommand::Hashfull(10000),Ok(String::from("hashfull 10000"))),
		(UsiInfoSubCommand::Nps(10000),Ok(String::from("nps 10000"))),
		(UsiInfoSubCommand::Str(String::from("hellow world!")),Ok(String::from("string hellow world!"))),

		(UsiInfoSubCommand::Pv(vec![]),Err(UsiOutputCreateError::InvalidStateError(String::from("pv")))),
		(UsiInfoSubCommand::Pv(vec![
			Move::To(KomaSrcPosition(10,7),KomaDstToPosition(1,6,false)),
		]),Err(UsiOutputCreateError::InvalidStateError(String::from("pv")))),
		(UsiInfoSubCommand::Pv(vec![
			Move::To(KomaSrcPosition(7,10),KomaDstToPosition(1,6,false)),
		]),Err(UsiOutputCreateError::InvalidStateError(String::from("pv")))),
	];

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(i.to_usi_command(),r);
	}
}
#[test]
fn test_usi_opt_type_to_usi_command() {
	let input_and_expected:Vec<(UsiOptType,Result<String,UsiOutputCreateError>)> = vec![
		(UsiOptType::Check(Some(true)),Ok(String::from("check default true"))),
		(UsiOptType::Check(Some(false)),Ok(String::from("check default false"))),
		(UsiOptType::Check(None),Ok(String::from("check"))),
		(UsiOptType::Spin(1,1000,Some(100)),Ok(String::from("spin default 100 min 1 max 1000"))),
		(UsiOptType::Spin(1,1000,None),Ok(String::from("spin min 1 max 1000"))),
		(UsiOptType::Combo(Some(String::from("d")),["aaa","bbb","d"].into_iter().map(|v| v.to_string()).collect::<Vec<String>>()),
		Ok(String::from("combo default d var aaa var bbb var d"))),
		(UsiOptType::Combo(None,["aaa","bbb","d"].into_iter().map(|v| v.to_string()).collect::<Vec<String>>()),
		Ok(String::from("combo var aaa var bbb var d"))),
		(UsiOptType::Button,Ok(String::from("button"))),
		(UsiOptType::String(Some(String::from(""))),Ok(String::from("string default <empty>"))),
		(UsiOptType::String(Some(String::from("aaa"))),Ok(String::from("string default aaa"))),
		(UsiOptType::String(None),Ok(String::from("string"))),
		(UsiOptType::FileName(Some(String::from(""))),Ok(String::from("filename default <empty>"))),
		(UsiOptType::FileName(Some(String::from("aaa"))),Ok(String::from("filename default aaa"))),
		(UsiOptType::FileName(None),Ok(String::from("filename"))),

		(UsiOptType::Combo(Some(String::from("d")),vec![]),Err(UsiOutputCreateError::InvalidStateError(String::from("There is no selection item of combo")))),
		(UsiOptType::Combo(None,vec![]),Err(UsiOutputCreateError::InvalidStateError(String::from("There is no selection item of combo")))),
	];

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(i.to_usi_command(),r);
	}
}
#[test]
fn test_usi_command_to_usi_command() {
	let input_and_expected:Vec<(UsiCommand,Result<Vec<String>,UsiOutputCreateError>)> = vec![
		(UsiCommand::UsiOk,Ok(vec![String::from("usiok")])),
		(UsiCommand::UsiId(String::from("testengine"),String::from("j6k1")),Ok(vec![
			String::from("id name testengine"),String::from("id author j6k1")
		])),
		(UsiCommand::UsiReadyOk,Ok(vec![String::from("readyok")])),
		(UsiCommand::UsiBestMove(BestMove::Move(Move::To(KomaSrcPosition(7,7),KomaDstToPosition(7,6,false)),None)),Ok(vec![
			String::from("bestmove 7g7f")
		])),
		(UsiCommand::UsiInfo(vec![
			UsiInfoSubCommand::Depth(1),
			UsiInfoSubCommand::SelDepth(3),
			UsiInfoSubCommand::Time(10000),
			UsiInfoSubCommand::Nodes(1000000),
			UsiInfoSubCommand::Pv(vec![
				Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false)),
				Move::To(KomaSrcPosition(9,3),KomaDstToPosition(9,4,false)),
				Move::To(KomaSrcPosition(1,6),KomaDstToPosition(1,5,false))
			]),
			UsiInfoSubCommand::MultiPv(1),
			UsiInfoSubCommand::Score(UsiScore::Cp(-100)),
			UsiInfoSubCommand::CurMove(Move::To(KomaSrcPosition(1,7),KomaDstToPosition(1,6,false))),
			UsiInfoSubCommand::Hashfull(10000),
			UsiInfoSubCommand::Nps(100)
		]),Ok(vec![
			String::from("info depth 1 seldepth 3 time 10000 nodes 1000000 score cp -100 curmove 1g1f hashfull 10000 nps 100 multipv 1 pv 1g1f 9c9d 1f1e")
		])),
		(UsiCommand::UsiOption(String::from("item"),UsiOptType::String(Some(String::from("aaa")))),Ok(vec![
			String::from("option name item type string default aaa")
		])),
		(UsiCommand::UsiCheckMate(CheckMate::Moves(vec![
			Move::To(KomaSrcPosition(7,7),KomaDstToPosition(7,6,false)),
			Move::To(KomaSrcPosition(3,3),KomaDstToPosition(3,4,false)),
			Move::To(KomaSrcPosition(8,8),KomaDstToPosition(3,3,true))
		])),Ok(vec![String::from("checkmate 7g7f 3c3d 8h3c+")])),

	];

	for (i,r) in input_and_expected.into_iter() {
		assert_eq!(i.to_usi_command(),r);
	}
}
