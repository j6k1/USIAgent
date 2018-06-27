use std::fmt;
use std::error::Error;

use shogi::*;
use command::*;
use error::*;
use TryFrom;
use Validate;

#[derive(Debug)]
pub enum UsiOutput {
	Command(Vec<String>),
}
impl UsiOutput {
	pub fn try_from(cmd: &UsiCommand) -> Result<UsiOutput, UsiOutputCreateError> {
		Ok(UsiOutput::Command(cmd.to_usi_command()?))
	}
}
pub trait ToSfen<E> where E: Error + fmt::Display {
	fn to_sfen(&self) -> Result<String,E>;
}
pub trait ToUsiCommand<T,E> where T: fmt::Debug, E: fmt::Debug + Error {
	fn to_usi_command(&self) -> Result<T,E>;
}
trait DanCharFromNum {
	fn char_from(n: u32) -> Result<char, DanConvertError>;
}
struct DanCharCreator {

}
impl DanCharFromNum for DanCharCreator {
	fn char_from(n: u32) -> Result<char, DanConvertError> {
		const DAN_MAP:[char; 9] = ['a','b','c','d','e','f','g','h','i'];

		match n {
			n if n > 9 => Err(DanConvertError(n)),
			n => Ok(DAN_MAP[n as usize - 1]),
		}
	}
}
trait KomaStrFromKind<T> {
	fn str_from(k:T) -> String;
}
const MOCHIGOMA_MAP:[char; 8] = ['P','L','N','S','G','B','R','K'];

struct KomaStringCreator {

}
impl KomaStrFromKind<MochigomaKind> for KomaStringCreator {
	fn str_from(k:MochigomaKind) -> String {
		format!("{}",MOCHIGOMA_MAP[k as usize])
	}
}
trait MoveStringFrom {
	fn str_from(m:&Move) -> Result<String, ToMoveStringConvertError>;
}
struct MoveStringCreator {

}
impl MoveStringFrom for MoveStringCreator {
	fn str_from(m:&Move) -> Result<String, ToMoveStringConvertError> {
		match m {
			&Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,false)) => {
				Ok(format!("{}{}{}{}", sx, DanCharCreator::char_from(sy)?, dx, DanCharCreator::char_from(dy)?))
			},
			&Move::To(KomaSrcPosition(sx,sy),KomaDstToPosition(dx,dy,true)) => {
				Ok(format!("{}{}{}{}+", sx, DanCharCreator::char_from(sy)?, dx, DanCharCreator::char_from(dy)?))
			},
			&Move::Put(k,KomaDstPutPosition(x,y)) => {
				Ok(format!("{}*{}{}", KomaStringCreator::str_from(k), x, DanCharCreator::char_from(y)?))
			},
		}
	}
}
impl<'a> TryFrom<&'a str,String> for Move {
	fn try_from(s: &'a str) -> Result<Move, TypeConvertError<String>> {
		match s {
			s if s.len() < 4 => {
				return Err(TypeConvertError::SyntaxError(String::from(
					"Invalid SFEN character string (number of characters of move expression is invalid)")));
			},
			_ => (),
		};

		let mut chars = s.chars();

		Ok(match chars.next() {
			Some(c) => match c {
				'R' | 'B' | 'G' | 'S' | 'N' | 'L' | 'P' => {
					match chars.next() {
						Some('*') => (),
						None | Some(_) => {
							return Err(TypeConvertError::SyntaxError(
								String::from("Invalid SFEN string (there no '*' after the name)")));
						}
					}
					let mochigoma = match c {
						'R' => MochigomaKind::Hisha,
						'B' => MochigomaKind::Kaku,
						'G' => MochigomaKind::Kin,
						'S' => MochigomaKind::Gin,
						'N' => MochigomaKind::Kei,
						'L' => MochigomaKind::Kyou,
						'P' => MochigomaKind::Fu,
						_ => {
							return Err(TypeConvertError::LogicError(String::from(
								"Logic error in the move analysis phase of the SFEN string analysis process."
							)));
						}
					};
					match chars.next() {
						Some(c) => match c {
							'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
								let x = c as u32 - '0' as u32;

								match chars.next() {
									Some(c) => {
										let y = match c {
											'a' => 1,
											'b' => 2,
											'c' => 3,
											'd' => 4,
											'e' => 5,
											'f' => 6,
											'g' => 7,
											'h' => 8,
											'i' => 9,
											_ => {
												return Err(TypeConvertError::SyntaxError(
													String::from(
														"Invalid SFEN character string (The format of the move is illegal)"
												)));
											}
										};

										match chars.next() {
											Some(_) => {
												return Err(TypeConvertError::LogicError(String::from(
													"Logic error in the move analysis phase of the SFEN string analysis process."
												)));
											},
											None => (),
										};
										Move::Put(mochigoma, KomaDstPutPosition(x,y))
									},
									None => {
										return Err(TypeConvertError::SyntaxError(
											String::from(
												"Invalid SFEN character string (The format of the move is illegal)"
										)));
									}
								}
							},
							_ => {
								return Err(TypeConvertError::SyntaxError(
										String::from(
											"Invalid SFEN character string (The format of the move is illegal)"
										)));
							}
						},
						None => {
							return Err(TypeConvertError::SyntaxError(
										String::from(
											"Invalid SFEN character string (The format of the hand is illegal)"
										)));
						}
					}
				},
				'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
					let x = c as u32 - '0' as u32;

					let src = match chars.next() {
						Some(c) => {
							let y = match c {
								'a' => 1,
								'b' => 2,
								'c' => 3,
								'd' => 4,
								'e' => 5,
								'f' => 6,
								'g' => 7,
								'h' => 8,
								'i' => 9,
								_ => {
									return Err(TypeConvertError::SyntaxError(String::from(
										"Invalid SFEN character string (The format of the move is illegal)"
									)));
								}
							};
							KomaSrcPosition(x,y)
						},
						None => {
							return Err(TypeConvertError::SyntaxError(
								String::from(
									"Invalid SFEN character string (The format of the move is illegal)"
							)));
						}
					};
					match chars.next() {
						Some(c) => match c {
							'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
								let x = c as u32 - '0' as u32;

								match chars.next() {
									Some(c) => {
										let y = match c {
											'a' => 1,
											'b' => 2,
											'c' => 3,
											'd' => 4,
											'e' => 5,
											'f' => 6,
											'g' => 7,
											'h' => 8,
											'i' => 9,
											_ => {
												return Err(TypeConvertError::SyntaxError(
													String::from(
														"Invalid SFEN character string (The format of the move is illegal)"
												)));
											}
										};

										match chars.next() {
											Some('+')  => Move::To(src, KomaDstToPosition(x,y,true)),
											None => Move::To(src, KomaDstToPosition(x,y,false)),
											_ => {
												return Err(TypeConvertError::SyntaxError(
													String::from(
														"Invalid SFEN character string (The format of the move is illegal)"
												)));
											}
										}
									},
									None => {
										return Err(TypeConvertError::SyntaxError(
											String::from(
												"Invalid SFEN character string (The format of the move is illegal)"
										)));
									}
								}
							},
							_ => {
								return Err(TypeConvertError::SyntaxError(
										String::from(
											"Invalid SFEN character string (The format of the move is illegal)"
										)));
							}
						},
						None => {
							return Err(TypeConvertError::SyntaxError(
										String::from(
											"Invalid SFEN character string (The format of the hand is illegal)"
										)));
						}
					}
				},
				_ => {
					return Err(TypeConvertError::SyntaxError(
							String::from(
								"Invalid SFEN character string (The format of the move is illegal)"
							)));
				}
			},
			None => {
				return Err(TypeConvertError::SyntaxError(
							String::from(
								"Invalid SFEN character string (The format of the hand is illegal)"
							)));
			}
		})
	}
}
impl ToSfen<ToMoveStringConvertError> for Vec<Move> {
	fn to_sfen(&self) -> Result<String, ToMoveStringConvertError> {
		let mut strs:Vec<String> = Vec::with_capacity(self.len());

		for m in self {
			strs.push(MoveStringCreator::str_from(m)?);
		}

		Ok(strs.join(" "))
	}
}
impl TryFrom<String,String> for KomaKind {
	fn try_from(s: String) -> Result<KomaKind, TypeConvertError<String>> {
		Ok(match &*s {
			"K" => KomaKind::SOu,
			"R" => KomaKind::SHisha,
			"B" => KomaKind::SKaku,
			"G" => KomaKind::SKin,
			"S" => KomaKind::SGin,
			"N" => KomaKind::SKei,
			"L" => KomaKind::SKyou,
			"P" => KomaKind::SFu,
			"+R" => KomaKind::SHishaN,
			"+B" => KomaKind::SKakuN,
			"+N" => KomaKind::SKeiN,
			"+S" => KomaKind::SGinN,
			"+L" => KomaKind::SKyouN,
			"+P" => KomaKind::SFuN,
			"k" => KomaKind::GOu,
			"r" => KomaKind::GHisha,
			"b" => KomaKind::GKaku,
			"g" => KomaKind::GKin,
			"s" => KomaKind::GGin,
			"n" => KomaKind::GKei,
			"l" => KomaKind::GKyou,
			"p" => KomaKind::GFu,
			"+r" => KomaKind::GHishaN,
			"+b" => KomaKind::GKakuN,
			"+n" => KomaKind::GKeiN,
			"+s" => KomaKind::GGinN,
			"+l" => KomaKind::GKyouN,
			"+p" => KomaKind::GFuN,
			_ => return Err(TypeConvertError::SyntaxError(s)),
		})
	}
}
impl<'a> TryFrom<&'a str,String> for Banmen {
	fn try_from(s: &'a str) -> Result<Banmen, TypeConvertError<String>> {
		let mut chars = s.chars();

		let mut banmen:[[KomaKind; 9]; 9] = [[KomaKind::Blank; 9]; 9];

		let mut x = 0;
		let mut y = 0;

		while let Some(c) = chars.next() {
			let mut s = String::new();

			s.push(match y {
				y if y >= 9=> {
					return Err(TypeConvertError::SyntaxError(
							String::from("Invalid SFEN character string (pieces outside the range of the board)")));
				},
				_ => c,
			});

			match c {
				_ if x > 9 => {
					return Err(TypeConvertError::LogicError(
						String::from("Logic error of SFEN character string parsing process.")));
				},
				'/' => if x == 9 {
					y += 1; x = 0;
				},
				_ if x == 9 => {
					return Err(TypeConvertError::SyntaxError(
						String::from("Invalid SFEN string (line separator '/' not found)")));
				},
				'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' if x + ((c as u32) - ('0' as u32)) > 9 => {
					return Err(TypeConvertError::SyntaxError(
							String::from("Invalid SFEN character string (pieces outside the range of the board)")));
				},
				'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
					x += (c as u32) - ('0' as u32);
				},
				'+' => match chars.next() {
					None => {
						return Err(TypeConvertError::SyntaxError(
							String::from("Invalid SFEN character string (illegal expression of piece)")));
					},
					Some(n) => {
						s.push(n);
						banmen[y as usize][x as usize] = KomaKind::try_from(s)?;
						x += 1;
					}
				},
				_ =>  {
					banmen[y as usize][x as usize] = KomaKind::try_from(s)?;
					x += 1;
				}
			}
		}

		Ok(Banmen(banmen))
	}
}
impl<'a> TryFrom<&'a str,String> for Teban {
	fn try_from(s: &'a str) -> Result<Teban, TypeConvertError<String>> {
		Ok(match s {
			"b" => Teban::Sente,
			"w" => Teban::Gote,
			_ => {
				return Err(TypeConvertError::SyntaxError(String::from(
					"It is an illegal character string as a character string representing a turn."
				)));
			}
		})
	}
}
impl ToSfen<TypeConvertError<String>> for Banmen {
	fn to_sfen(&self) -> Result<String,TypeConvertError<String>> {
		let mut s = String::new();

		match self {
			&Banmen(ref kinds) => {
				for y in 0..9 {
					let mut n = 0;
					for x in 0..9 {
						match kinds[y][x] {
							KomaKind::Blank => {
								n += 1;
							},
							k => {
								if n > 0 {
									s.push((n + '0' as u8) as char);
									n = 0;
								}

								match k {
									KomaKind::SOu => s.push('K'),
									KomaKind::SHisha => s.push('R'),
									KomaKind::SKaku => s.push('B'),
									KomaKind::SKin => s.push('G'),
									KomaKind::SGin => s.push('S'),
									KomaKind::SKei => s.push('N'),
									KomaKind::SKyou => s.push('L'),
									KomaKind::SFu => s.push('P'),
									KomaKind::SHishaN => {
										s.push('+');
										s.push('R');
									},
									KomaKind::SKakuN => {
										s.push('+');
										s.push('B');
									},
									KomaKind::SGinN => {
										s.push('+');
										s.push('S');
									},
									KomaKind::SKeiN => {
										s.push('+');
										s.push('N');
									},
									KomaKind::SKyouN => {
										s.push('+');
										s.push('L');
									},
									KomaKind::SFuN => {
										s.push('+');
										s.push('P');
									},
									KomaKind::GOu => s.push('k'),
									KomaKind::GHisha => s.push('r'),
									KomaKind::GKaku => s.push('b'),
									KomaKind::GKin => s.push('g'),
									KomaKind::GGin => s.push('s'),
									KomaKind::GKei => s.push('n'),
									KomaKind::GKyou => s.push('l'),
									KomaKind::GFu => s.push('p'),
									KomaKind::GHishaN => {
										s.push('+');
										s.push('r');
									},
									KomaKind::GKakuN => {
										s.push('+');
										s.push('b');
									},
									KomaKind::GGinN => {
										s.push('+');
										s.push('s');
									},
									KomaKind::GKeiN => {
										s.push('+');
										s.push('n');
									},
									KomaKind::GKyouN => {
										s.push('+');
										s.push('l');
									},
									KomaKind::GFuN => {
										s.push('+');
										s.push('p');
									},
									KomaKind::Blank => (),
								}
							}
						}
					}
					if n > 0 {
						s.push((n + '0' as u8) as char);
					}
					s.push('/');
				}
				s.pop();
			}
		}
		Ok(s)
	}
}
impl ToSfen<TypeConvertError<String>> for Teban {
	fn to_sfen(&self) -> Result<String, TypeConvertError<String>> {
		Ok(match *self {
			Teban::Sente => String::from("b"),
			Teban::Gote => String::from("w"),
		})
	}
}
impl ToSfen<TypeConvertError<String>> for MochigomaCollections {
	fn to_sfen(&self) -> Result<String, TypeConvertError<String>> {
		let mut sfen = String::new();

		match self {
			&MochigomaCollections::Empty => {
				Ok(String::from("-"))
			},
			&MochigomaCollections::Pair(ref ms, ref mg) => {
				const SFEN_MOCHIGOMA_KINDS_SENTE:[(char,MochigomaKind); 7] = [
					('R', MochigomaKind::Hisha),
					('B', MochigomaKind::Kaku),
					('G', MochigomaKind::Kin),
					('S', MochigomaKind::Gin),
					('N', MochigomaKind::Kei),
					('L', MochigomaKind::Kyou),
					('P', MochigomaKind::Fu),
				];

				for &(c,k) in &SFEN_MOCHIGOMA_KINDS_SENTE {
					if let Some(n) = ms.get(&k) {
						if *n > 0 {
							sfen.push(c);
							sfen.push_str(&n.to_string());
						}
					}
				}

				const SFEN_MOCHIGOMA_KINDS_GOTE:[(char,MochigomaKind); 7] = [
					('r', MochigomaKind::Hisha),
					('b', MochigomaKind::Kaku),
					('g', MochigomaKind::Kin),
					('s', MochigomaKind::Gin),
					('n', MochigomaKind::Kei),
					('l', MochigomaKind::Kyou),
					('p', MochigomaKind::Fu),
				];

				for &(c,k) in &SFEN_MOCHIGOMA_KINDS_GOTE {
					if let Some(n) = mg.get(&k) {
						if *n > 0 {
							sfen.push(c);
							sfen.push_str(&n.to_string());
						}
					}
				}

				if sfen.len() == 0 {
					Ok(String::from("-"))
				} else {
					Ok(sfen)
				}
			}
		}
	}
}
impl ToSfen<SfenStringConvertError> for (Teban,Banmen,MochigomaCollections,Vec<Move>) {
	fn to_sfen(&self) -> Result<String,SfenStringConvertError> {
		Ok(match self {
			&(ref t, ref b, ref mc, ref m) if m.len() > 0 => {
				let mc = mc.to_sfen()?;

				if *t == Teban::Sente && mc == "-" && *b == BANMEN_START_POS {
					format!("startpos moves {}", m.to_sfen()?)
				} else {
					format!("sfen {} {} {} 1 moves {}", b.to_sfen()?, t.to_sfen()?, mc, m.to_sfen()?)
				}
			},
			&(ref t, ref b, ref mc, _) => {
				let mc = mc.to_sfen()?;

				if *t == Teban::Sente && mc == "-" && *b == BANMEN_START_POS {
					format!("startpos")
				} else {
					format!("sfen {} {} {} 1", b.to_sfen()?, t.to_sfen()?, mc)
				}
			}
		})
	}
}
impl ToSfen<UsiOutputCreateError> for CheckMate {
	fn to_sfen(&self) -> Result<String, UsiOutputCreateError> {
		Ok(match *self {
			CheckMate::Moves(ref v) if v.len() < 1 => {
				return Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))
			},
			CheckMate::Moves(ref v) => {
				let mut mv:Vec<String> = Vec::with_capacity(v.len());
				for m in v {
					match *m {
						ref m if !m.validate() => {
							return Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))
						},
						ref m => {
							mv.push(MoveStringCreator::str_from(m)?);
						}
					}
				}
				mv.join(" ")
			},
			CheckMate::NotiImplemented => format!("notimplemented"),
			CheckMate::Timeout => format!("timeout"),
			CheckMate::Nomate => format!("nomate"),
			CheckMate::Abort => {
				return Err(UsiOutputCreateError::AbortedError);
			}
		})
	}
}
impl ToSfen<ToMoveStringConvertError> for BestMove {
	fn to_sfen(&self) -> Result<String, ToMoveStringConvertError> {
		match *self {
			BestMove::Resign => Ok(String::from("resign")),
			BestMove::Win => Ok(String::from("win")),
			BestMove::Move(ref m,None) => Ok(MoveStringCreator::str_from(m)?),
			BestMove::Move(ref m,Some(ref pm)) => {
				Ok(format!("{} ponder {}",
						MoveStringCreator::str_from(m)?,
						MoveStringCreator::str_from(pm)?))

			},
			BestMove::Abort => {
				Err(ToMoveStringConvertError::AbortedError)
			}
		}
	}
}
impl ToUsiCommand<String,UsiOutputCreateError> for Vec<UsiInfoSubCommand> {
	fn to_usi_command(&self) -> Result<String, UsiOutputCreateError> {
		let mut strs:Vec<String> = Vec::with_capacity(self.len());

		for cmd in self {
			strs.push(cmd.to_usi_command()?);
		}

		Ok(strs.join(" "))
	}
}
impl ToUsiCommand<String,UsiOutputCreateError> for UsiInfoSubCommand {
	fn to_usi_command(&self) -> Result<String, UsiOutputCreateError> {
		Ok(match *self {
			UsiInfoSubCommand::Depth(d) => format!("depth {}", d),
			UsiInfoSubCommand::SelDepth(d) => format!("seldepth {}", d),
			UsiInfoSubCommand::Time(t) => format!("time {}",t),
			UsiInfoSubCommand::Nodes(n) => format!("nodes {}", n),
			UsiInfoSubCommand::Pv(ref v) if v.len() < 1 => {
				return Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))
			},
			UsiInfoSubCommand::Pv(ref v) => {
				let mut mv:Vec<String> = Vec::with_capacity(v.len());
				for m in v {
					match *m {
						ref m if !m.validate() => {
							return Err(UsiOutputCreateError::InvalidStateError(String::from("checkmate")))
						},
						ref m => {
							mv.push(MoveStringCreator::str_from(m)?);
						}
					}
				}
				mv.join(" ")
			},
			UsiInfoSubCommand::Score(UsiScore::Cp(cp)) => format!("score cp {}", cp),
			UsiInfoSubCommand::Score(UsiScore::CpUpper(cp)) => {
				format!("score cp {} upperbound", cp)
			},
			UsiInfoSubCommand::Score(UsiScore::CpLower(cp)) => {
				format!("score cp {} lowerbound", cp)
			},
			UsiInfoSubCommand::Score(UsiScore::Mate(UsiScoreMate::Num(n))) => format!("score mate {}",n),
			UsiInfoSubCommand::Score(UsiScore::Mate(UsiScoreMate::Plus)) => format!("score mate +"),
			UsiInfoSubCommand::Score(UsiScore::Mate(UsiScoreMate::Minus)) => format!("score mate -"),
			UsiInfoSubCommand::Score(UsiScore::MateUpper(n)) => {
				format!("score mate {} upperbound",n)
			},
			UsiInfoSubCommand::Score(UsiScore::MateLower(n)) => {
				format!("score mate {} lowerbound",n)
			},
			UsiInfoSubCommand::CurMove(ref m) => {
				MoveStringCreator::str_from(m)?
			},
			UsiInfoSubCommand::Hashfull(v) => format!("hashfull {}", v),
			UsiInfoSubCommand::Nps(v) => format!("nps {}",v),
			UsiInfoSubCommand::Str(ref s) => format!("string {}",s),
		})
	}
}
impl ToUsiCommand<String,UsiOutputCreateError> for UsiOptType {
	fn to_usi_command(&self) -> Result<String, UsiOutputCreateError> {
		Ok(match *self {
			UsiOptType::Check(Some(b)) if b => format!("check default true"),
			UsiOptType::Check(Some(_)) => format!("check default false"),
			UsiOptType::Check(None) => format!("check"),
			UsiOptType::Spin(min, max,Some(d)) => format!("spin default {} min {} max {}",d,min,max),
			UsiOptType::Spin(min, max,None) => format!("spin min {} max {}", min, max),
			UsiOptType::Combo(Some(_), ref v) if v.len() < 1 => {
				return Err(UsiOutputCreateError::InvalidStateError(String::from("There is no selection item of combo")))
			},
			UsiOptType::Combo(Some(ref d), ref v) => {
				format!("combo default {} {}", d,
					v.iter().map(|va| format!("var {}", va)).collect::<Vec<String>>().join(" "))
			},
			UsiOptType::Combo(None, ref v) => {
				format!("combo {}", v.iter()
									.map(|va| format!("var {}", va)).collect::<Vec<String>>().join(" "))
			},
			UsiOptType::Button => format!("button"),
			UsiOptType::String(Some(ref s)) if s.is_empty() => format!("string default <empty>"),
			UsiOptType::String(Some(ref s)) => format!("string default {}", s),
			UsiOptType::String(None) => format!("string"),
			UsiOptType::FileName(Some(ref s)) if s.is_empty() => format!("filename <empty>"),
			UsiOptType::FileName(Some(ref s)) => format!("filename {}", s),
			UsiOptType::FileName(None) => format!("filename"),
		})
	}
}
impl ToUsiCommand<Vec<String>,UsiOutputCreateError> for UsiCommand {
	fn to_usi_command(&self) -> Result<Vec<String>, UsiOutputCreateError> {
		Ok(match *self {
			UsiCommand::UsiOk => vec![String::from("usiok")],
			UsiCommand::UsiId(ref name, ref author) => {
				vec![format!("id name {}", name), format!("id author {}", author)]
			},
			UsiCommand::UsiReadyOk => vec![String::from("readyok")],
			UsiCommand::UsiBestMove(ref m) => vec![format!("bestmove {}", m.to_sfen()?)],
			UsiCommand::UsiInfo(ref i) => vec![format!("info {}", i.to_usi_command()?)],
			UsiCommand::UsiOption(ref s,ref opt) => vec![format!("option name {} type {}",s,opt.to_usi_command()?)],
			UsiCommand::UsiCheckMate(ref c) => vec![format!("checkmate {}", c.to_sfen()?)],
		})
	}
}