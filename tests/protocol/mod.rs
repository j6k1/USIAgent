use usiagent::TryFrom;
use usiagent::shogi::*;
use usiagent::protocol::*;
use usiagent::error::*;
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