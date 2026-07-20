use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use usiagent::bitboard::BitBoard;
use usiagent::math::Prng;
use usiagent::movepick::RandomPicker;
use usiagent::protocol::{ParsePosition, PositionParser, ToSfen};
use usiagent::rule::{Checks, LegalMove, Rule, State};
use usiagent::shogi::{MochigomaCollections, MochigomaKind, ObtainKind, Teban};

#[ignore]
#[test]
fn test_has_blocking_move() {
    let position_parser = PositionParser::new();

    let mut rng = rand::thread_rng();
    let mut rng = XorShiftRng::from_seed(rng.gen());

    let mut rng = Prng::new(rng.gen());

    let mut buffer = RandomPicker::new(Prng::new(rng.rnd64()));

    for (n,(sfen,answer)) in BufReader::new(
        File::open(
            Path::new("data").join("random").join("generatemoves").join("random_kyokumen_sfen_uniq.txt")
        ).unwrap()).lines().zip(BufReader::new(
        File::open(
            Path::new("data").join("random").join("has_blocking").join("answer_has_blocking_move_by_random_move.txt")
        ).unwrap()).lines()).enumerate() {
        let mut expected = answer.unwrap().split(',').into_iter().map(|m| m.to_string()).collect::<Vec<String>>();

        expected.sort();

        let expected = expected.join(",");

        let sfen = format!("sfen {}",sfen.unwrap());

        let (teban, banmen, mc, _, _) = match position_parser.parse(&sfen.split(" ").collect::<Vec<&str>>()) {
            Ok(position) => {
                position.extract()
            },
            Err(e) => {
                panic!("invalid state. {}",e);
            }
        };

        let state = State::new(banmen);

        Rule::generate_moves::<Checks>(teban, &state, &mc, &mut buffer).unwrap();

        let mut answer_buf = Vec::new();

        for m in &mut buffer {
            let has_blocking_move = Rule::has_blocking_move(teban,&state,m);

            answer_buf.push(format!("{} {}",m.to_move().to_sfen().unwrap(),has_blocking_move.to_string()));
        }

        answer_buf.sort();

        let answer = answer_buf.join(",");

        if &expected != &answer {
            println!("line {}: {}",n, sfen);
        }

        assert_eq!(expected, answer);
    }
}
#[ignore]
#[test]
fn test_has_blocking_move_by_floodgate() {
    let position_parser = PositionParser::new();

    let mut rng = rand::thread_rng();
    let mut rng = XorShiftRng::from_seed(rng.gen());

    let mut rng = Prng::new(rng.gen());

    let mut buffer = RandomPicker::new(Prng::new(rng.rnd64()));

    for (n,(sfen,answer)) in BufReader::new(
        File::open(
            Path::new("data").join("floodgate").join("generatemoves").join("kyokumen_sfen_uniq.txt")
        ).unwrap()).lines().zip(BufReader::new(
        File::open(
            Path::new("data").join("floodgate").join("has_blocking").join("answer_has_blocking_move_by_floodgate.txt")
        ).unwrap()).lines()).enumerate() {

        let mut expected = answer.unwrap().split(',').into_iter().map(|m| m.to_string()).collect::<Vec<String>>();

        expected.sort();

        let expected = expected.join(",");

        let sfen = format!("sfen {}",sfen.unwrap());

        let (teban, banmen, mc, _, _) = match position_parser.parse(&sfen.split(" ").collect::<Vec<&str>>()) {
            Ok(position) => {
                position.extract()
            },
            Err(e) => {
                panic!("invalid state. {}",e);
            }
        };

        let state = State::new(banmen);

        Rule::generate_moves::<Checks>(teban, &state, &mc, &mut buffer).unwrap();

        let mut answer_buf = Vec::new();

        for m in &mut buffer {
            let has_blocking_move = Rule::has_blocking_move(teban,&state,m);

            answer_buf.push(format!("{} {}",m.to_move().to_sfen().unwrap(),has_blocking_move.to_string()));
        }

        answer_buf.sort();

        let answer = answer_buf.join(",");

        if &expected != &answer {
            println!("line {}: {}",n, sfen);
        }

        assert_eq!(expected, answer);
    }
}
const PIECES_MAP:[char;7] = ['P','L','N','S','G','B','R'];
#[ignore]
#[test]
fn test_has_blocking_drop() {
    #[inline]
    fn aggregation_can_blocking_drop(buffer:&mut HashSet<MochigomaKind>,
                              teban:Teban,
                              mc:&MochigomaCollections,
                              check_line:BitBoard,
                              self_occupied_board:BitBoard,
                              opponent_occupied_board:BitBoard,
                              self_nari_board:BitBoard,
                              self_fu_board:BitBoard) {
        let mc = match mc {
            MochigomaCollections::Empty => {
                return;
            },
            MochigomaCollections::Pair(mc,_) if teban == Teban::Sente => {
                mc
            },
            MochigomaCollections::Pair(_,mc) => {
                mc
            }
        };

        let mut it = mc.iter();

        let (_, count) = it.next().expect("Could not retrieve item from logic error iterator.");

        if count > 0 && Rule::has_blocking_drop_by_fu(
            check_line,
            self_occupied_board,
            opponent_occupied_board,
            self_nari_board,
            self_fu_board
        ) {
            buffer.insert(MochigomaKind::Fu);
        }

        let (_, count) = it.next().expect("Could not retrieve item from logic error iterator.");

        if count > 0 && Rule::has_blocking_drop_by_kyou(
            check_line,
            self_occupied_board,
            opponent_occupied_board
        ) {
            buffer.insert(MochigomaKind::Kyou);
        }

        let (_, count) = it.next().expect("Could not retrieve item from logic error iterator.");

        if count > 0 && Rule::has_blocking_drop_by_kei(
            check_line,
            self_occupied_board,
            opponent_occupied_board
        ) {
            buffer.insert(MochigomaKind::Kei);
        }

        let (_, count) = it.next().expect("Could not retrieve item from logic error iterator.");

        if count > 0 {
            buffer.insert(MochigomaKind::Gin);
        }

        let (_, count) = it.next().expect("Could not retrieve item from logic error iterator.");

        if count > 0 {
            buffer.insert(MochigomaKind::Kin);
        }

        let (_, count) = it.next().expect("Could not retrieve item from logic error iterator.");

        if count > 0 {
            buffer.insert(MochigomaKind::Kaku);
        }

        let (_, count) = it.next().expect("Could not retrieve item from logic error iterator.");

        if count > 0 {
            buffer.insert(MochigomaKind::Hisha);
        }
    }

    let position_parser = PositionParser::new();

    let mut rng = rand::thread_rng();
    let mut rng = XorShiftRng::from_seed(rng.gen());

    let mut rng = Prng::new(rng.gen());

    let mut buffer = RandomPicker::new(Prng::new(rng.rnd64()));

    for (n,(sfen,answer)) in BufReader::new(
        File::open(
            Path::new("data").join("random").join("generatemoves").join("random_kyokumen_sfen_uniq.txt")
        ).unwrap()).lines().zip(BufReader::new(
        File::open(
            Path::new("data").join("random").join("has_blocking").join("answer_has_blocking_drop_by_random_move.txt")
        ).unwrap()).lines()).enumerate() {
        let mut expected = answer.unwrap().split(',').into_iter().map(|m| m.to_string()).collect::<Vec<String>>();

        expected.sort();

        let expected = expected.join(",");

        let sfen = format!("sfen {}",sfen.unwrap());

        let (teban, banmen, mc, _, _) = match position_parser.parse(&sfen.split(" ").collect::<Vec<&str>>()) {
            Ok(position) => {
                position.extract()
            },
            Err(e) => {
                panic!("invalid state. {}",e);
            }
        };

        let state = State::new(banmen);

        Rule::generate_moves::<Checks>(teban, &state, &mc, &mut buffer).unwrap();

        let mut answer_buf = Vec::new();

        for m in &mut buffer {
            let mut answer_pieaces = HashSet::new();

            if teban == Teban::Sente {
                if state.get_part().sente_checked_board != 0 || state.get_part().gote_checked_board != 0 {
                    answer_buf.push(format!("{} none",m.to_move().to_sfen().unwrap()));
                    continue;
                }

                let mut self_occupied_board = state.get_part().sente_self_board;
                let mut opponent_occupied_board = state.get_part().sente_opponent_board;
                let mut flip_self_occupied_board = state.get_part().gote_opponent_board;
                let mut flip_opponent_occupied_board = state.get_part().gote_self_board;
                let mut flip_opponent_nari_board = state.get_part().gote_nari_board;
                let mut flip_opponent_fu_board = state.get_part().gote_fu_board;

                match m {
                    LegalMove::Put(mv) => {
                        let to = mv.dst();
                        let to_mask = BitBoard::from(1 << (to + 1));

                        self_occupied_board ^= to_mask;
                        flip_self_occupied_board ^= to_mask.reverse();

                        let (check_line, _) = Rule::gen_check_line(teban, &state, m);

                        if check_line == 0 {
                            answer_buf.push(format!("{} none",m.to_move().to_sfen().unwrap()));
                            continue;
                        }

                        aggregation_can_blocking_drop(&mut answer_pieaces, teban.opposite(), &mc,
                                                      check_line,
                                                      flip_opponent_occupied_board,
                                                      flip_self_occupied_board,
                                                      flip_opponent_nari_board.reverse(),
                                                      flip_opponent_fu_board.reverse()
                        );
                    },
                    LegalMove::To(mv) if mv.obtained() == Some(ObtainKind::Ou) => {
                        answer_buf.push(format!("{} none",m.to_move().to_sfen().unwrap()));
                    },
                    LegalMove::To(mv) => {
                        let from = mv.src();

                        let from_mask = BitBoard::from(1 << (from + 1));

                        let to = mv.dst();
                        let to_mask = BitBoard::from(1 << (to + 1));

                        self_occupied_board ^= from_mask | to_mask;
                        flip_self_occupied_board ^= (from_mask | to_mask).reverse();

                        let captured_mask = !mv.obtained().map(|_| (1 << (mv.dst() + 1)).into()).unwrap_or(BitBoard::default());

                        opponent_occupied_board &= captured_mask;
                        flip_opponent_occupied_board &= captured_mask.reverse();
                        flip_opponent_nari_board &= captured_mask;
                        flip_opponent_fu_board &= captured_mask;

                        let (check_line,_) = Rule::gen_check_line(teban,&state,m);

                        if check_line == 0 {
                            answer_buf.push(format!("{} none",m.to_move().to_sfen().unwrap()));
                            continue;
                        }

                        aggregation_can_blocking_drop(&mut answer_pieaces, teban.opposite(), &mc,
                                                      check_line,
                                                      flip_opponent_occupied_board,
                                                      flip_self_occupied_board,
                                                      flip_opponent_nari_board.reverse(),
                                                      flip_opponent_fu_board.reverse()
                        );
                    }
                }
            } else {
                if state.get_part().gote_checked_board != 0 || state.get_part().sente_checked_board != 0 {
                    answer_buf.push(format!("{} none",m.to_move().to_sfen().unwrap()));
                    continue;
                }

                let mut self_occupied_board = state.get_part().gote_self_board;
                let mut opponent_occupied_board = state.get_part().gote_opponent_board;
                let mut flip_self_occupied_board = state.get_part().sente_opponent_board;
                let mut flip_opponent_occupied_board = state.get_part().sente_self_board;
                let mut flip_opponent_nari_board = state.get_part().sente_nari_board;
                let mut flip_opponent_fu_board = state.get_part().sente_fu_board;

                match m {
                    LegalMove::Put(mv) => {
                        let to = mv.dst();
                        let to_mask = BitBoard::from(1 << (to + 1));

                        self_occupied_board ^= to_mask.reverse();
                        flip_self_occupied_board ^= to_mask;

                        let (check_line, _) = Rule::gen_check_line(teban, &state, m);

                        if check_line == 0 {
                            answer_buf.push(format!("{} none",m.to_move().to_sfen().unwrap()));
                            continue;
                        }

                        aggregation_can_blocking_drop(&mut answer_pieaces, teban.opposite(), &mc,
                                                      check_line,
                                                      flip_opponent_occupied_board,
                                                      flip_self_occupied_board,
                                                      flip_opponent_nari_board,
                                                      flip_opponent_fu_board
                        );
                    },
                    LegalMove::To(mv) if mv.obtained() == Some(ObtainKind::Ou) => {
                        answer_buf.push(format!("{} none",m.to_move().to_sfen().unwrap()));
                    },
                    LegalMove::To(mv) => {
                        let from = mv.src();

                        let from_mask = BitBoard::from(1 << (from + 1));

                        let to = mv.dst();
                        let to_mask = BitBoard::from(1 << (to + 1));

                        self_occupied_board ^= (from_mask | to_mask).reverse();
                        flip_self_occupied_board ^= from_mask | to_mask;

                        let captured_mask = !mv.obtained().map(|_| (1 << (mv.dst() + 1)).into()).unwrap_or(BitBoard::default());

                        opponent_occupied_board &= captured_mask.reverse();
                        flip_opponent_occupied_board &= captured_mask;
                        flip_opponent_nari_board &= captured_mask;
                        flip_opponent_fu_board &= captured_mask;

                        let (check_line,_) = Rule::gen_check_line(teban,&state,m);

                        if check_line == 0 {
                            answer_buf.push(format!("{} none",m.to_move().to_sfen().unwrap()));
                            continue;
                        }

                        aggregation_can_blocking_drop(&mut answer_pieaces, teban.opposite(), &mc,
                                                      check_line,
                                                      flip_opponent_occupied_board,
                                                      flip_self_occupied_board,
                                                      flip_opponent_nari_board,
                                                      flip_opponent_fu_board
                        );
                    }
                }
            }

            if answer_pieaces.len() > 0 {
                let mut answer_pieaces = answer_pieaces.into_iter().collect::<Vec<MochigomaKind>>();

                answer_pieaces.sort();
                answer_buf.push(format!("{} exists {}",
                                        m.to_move().to_sfen().unwrap(),
                                        answer_pieaces.into_iter().map(|k| {
                                            PIECES_MAP[k as usize].to_string()
                                        }).collect::<Vec<String>>().join(" ")));
            } else {
                answer_buf.push(format!("{} none", m.to_move().to_sfen().unwrap()));
            }
        }

        answer_buf.sort();

        let answer = answer_buf.join(",");

        if &expected != &answer {
            println!("line {}: {}",n, sfen);
        }

        assert_eq!(expected, answer);
    }
}
#[ignore]
#[test]
fn test_can_blocking_count() {
    let position_parser = PositionParser::new();

    let mut rng = rand::thread_rng();
    let mut rng = XorShiftRng::from_seed(rng.gen());

    let mut rng = Prng::new(rng.gen());

    let mut buffer = RandomPicker::new(Prng::new(rng.rnd64()));

    for (n,(sfen,answer)) in BufReader::new(
        File::open(
            Path::new("data").join("random").join("generatemoves").join("random_kyokumen_sfen_uniq.txt")
        ).unwrap()).lines().zip(BufReader::new(
        File::open(
            Path::new("data").join("random").join("has_blocking").join("answer_can_blocking_count_by_random_move.txt")
        ).unwrap()).lines()).enumerate() {
        let mut expected = answer.unwrap().split(',').into_iter().map(|m| m.to_string()).collect::<Vec<String>>();

        expected.sort();

        let expected = expected.join(",");

        let sfen = format!("sfen {}",sfen.unwrap());

        let (teban, banmen, mc, _, _) = match position_parser.parse(&sfen.split(" ").collect::<Vec<&str>>()) {
            Ok(position) => {
                position.extract()
            },
            Err(e) => {
                panic!("invalid state. {}",e);
            }
        };

        let state = State::new(banmen);

        Rule::generate_moves::<Checks>(teban, &state, &mc, &mut buffer).unwrap();

        let mut answer_buf = Vec::new();

        for m in &mut buffer {
            answer_buf.push(format!("{} {}",m.to_move().to_sfen().unwrap(),Rule::can_blocking_count(teban,&state,&mc,m)));
        }

        answer_buf.sort();

        let answer = answer_buf.join(",");

        if &expected != &answer {
            println!("line {}: {}",n, sfen);
        }

        assert_eq!(expected, answer);
    }
}
#[ignore]
#[test]
fn test_can_blocking_count_by_floodgate() {
    let position_parser = PositionParser::new();

    let mut rng = rand::thread_rng();
    let mut rng = XorShiftRng::from_seed(rng.gen());

    let mut rng = Prng::new(rng.gen());

    let mut buffer = RandomPicker::new(Prng::new(rng.rnd64()));

    for (n,(sfen,answer)) in BufReader::new(
        File::open(
            Path::new("data").join("floodgate").join("generatemoves").join("kyokumen_sfen_uniq.txt")
        ).unwrap()).lines().zip(BufReader::new(
        File::open(
            Path::new("data").join("floodgate").join("has_blocking").join("answer_can_blocking_count_by_floodgate.txt")
        ).unwrap()).lines()).enumerate() {
        let mut expected = answer.unwrap().split(',').into_iter().map(|m| m.to_string()).collect::<Vec<String>>();

        expected.sort();

        let expected = expected.join(",");

        let sfen = format!("sfen {}",sfen.unwrap());

        let (teban, banmen, mc, _, _) = match position_parser.parse(&sfen.split(" ").collect::<Vec<&str>>()) {
            Ok(position) => {
                position.extract()
            },
            Err(e) => {
                panic!("invalid state. {}",e);
            }
        };

        let state = State::new(banmen);

        Rule::generate_moves::<Checks>(teban, &state, &mc, &mut buffer).unwrap();

        let mut answer_buf = Vec::new();

        for m in &mut buffer {
            answer_buf.push(format!("{} {}",m.to_move().to_sfen().unwrap(),Rule::can_blocking_count(teban,&state,&mc,m)));
        }

        answer_buf.sort();

        let answer = answer_buf.join(",");

        if &expected != &answer {
            println!("line {}: {}",n, sfen);
        }

        assert_eq!(expected, answer);
    }
}
