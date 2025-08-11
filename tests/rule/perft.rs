use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, AddAssign};
use std::path::Path;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use usiagent::math::Prng;
use usiagent::movepick::{MovePicker, RandomPicker};
use usiagent::protocol::{PositionParser};
use usiagent::rule::{Evasions, EvasionsAll, LegalMove, NonEvasionsAll, Rule};
use usiagent::rule::State;
use usiagent::shogi::{MochigomaCollections, Teban};

#[derive(Default,Debug,PartialEq,Eq)]
struct PerftResult {
    pub nodes:usize,
    pub captures:usize,
    pub promotions:usize,
    pub checks:usize,
    pub mates:usize
}
impl Add for PerftResult {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        PerftResult {
            nodes: self.nodes + rhs.nodes,
            captures: self.captures + rhs.captures,
            promotions: self.promotions + rhs.promotions,
            checks: self.checks + rhs.checks,
            mates: self.mates + rhs.mates
        }
    }
}
impl AddAssign for PerftResult {
    fn add_assign(&mut self, rhs: Self) {
        *self = PerftResult {
            nodes: self.nodes + rhs.nodes,
            captures: self.captures + rhs.captures,
            promotions: self.promotions + rhs.promotions,
            checks: self.checks + rhs.checks,
            mates: self.mates + rhs.mates
        }
    }
}
impl From<String> for PerftResult {
    fn from(value: String) -> Self {
        let fields: Vec<_> = value.split(',').map(|f| f.parse().unwrap()).collect();

        PerftResult {
            nodes: fields[0],
            captures: fields[1],
            promotions: fields[2],
            checks: fields[3],
            mates: fields[4]
        }
    }
}
trait PerftSolver {
    fn perft(&self,teban:Teban,state:&State,mc:&MochigomaCollections,m:Option<LegalMove>,depth:usize) -> PerftResult;
}
struct PerftSolverByNonEvasions;
struct PerftSolverByEvasions;

impl PerftSolver for PerftSolverByNonEvasions {
    fn perft(&self,teban:Teban,state: &State, mc:&MochigomaCollections, m: Option<LegalMove>, depth: usize) -> PerftResult {
        let mut result = PerftResult::default();

        if depth == 0 {
            result.nodes += 1;

            match m {
                Some(LegalMove::To(m)) => {
                    if m.obtained().is_some() {
                        result.captures += 1;
                    }
                    if m.is_nari() {
                        result.promotions += 1;
                    }
                },
                _ => ()
            };

            if Rule::in_check(teban.opposite(),state) {
                result.checks += 1;

                let mut rng = rand::thread_rng();
                let mut rng = XorShiftRng::from_seed(rng.gen());

                let mut buffer = RandomPicker::new(Prng::new(rng.gen()));

                Rule::generate_moves_all::<Evasions>(teban,state,mc,&mut buffer).unwrap();

                if buffer.len() == 0 {
                    result.mates += 1;
                }
            }
        } else {
            let mut rng = rand::thread_rng();
            let mut rng = XorShiftRng::from_seed(rng.gen());

            let mut buffer = RandomPicker::new(Prng::new(rng.gen()));

            Rule::generate_moves_all::<NonEvasionsAll>(teban,state,mc,&mut buffer).unwrap();

            for m in buffer {
                let next = Rule::apply_move_none_check(state, teban, mc, m.to_applied_move());

                match next {
                    (state, mc, _) => {
                        result += self.perft(teban.opposite(),&state,&mc,Some(m),depth - 1);
                    }
                }
            }
        }

        result
    }
}

impl PerftSolver for PerftSolverByEvasions {
    fn perft(&self,teban:Teban,state: &State, mc:&MochigomaCollections, m: Option<LegalMove>, depth: usize) -> PerftResult {
        let mut result = PerftResult::default();

        if depth == 0 {
            result.nodes += 1;

            match m {
                Some(LegalMove::To(m)) => {
                    if m.obtained().is_some() {
                        result.captures += 1;
                    }
                    if m.is_nari() {
                        result.promotions += 1;
                    }
                },
                _ => ()
            };

            if Rule::in_check(teban.opposite(),state) {
                result.checks += 1;

                let mut rng = rand::thread_rng();
                let mut rng = XorShiftRng::from_seed(rng.gen());

                let mut buffer = RandomPicker::new(Prng::new(rng.gen()));

                Rule::generate_moves_all::<Evasions>(teban,state,mc,&mut buffer).unwrap();

                if buffer.len() == 0 {
                    result.mates += 1;
                }
            }
        } else {
            let mut rng = rand::thread_rng();
            let mut rng = XorShiftRng::from_seed(rng.gen());

            let mut buffer = RandomPicker::new(Prng::new(rng.gen()));

            if Rule::in_check(teban,state) {
                Rule::generate_moves_all::<EvasionsAll>(teban,state,mc,&mut buffer).unwrap();
            } else {
                Rule::generate_moves_all::<NonEvasionsAll>(teban,state,mc,&mut buffer).unwrap();
            }

            for m in buffer {
                let next = Rule::apply_move_none_check(state, teban, mc, m.to_applied_move());

                match next {
                    (state, mc, _) => {
                        result += self.perft(teban.opposite(),&state,&mc,Some(m),depth - 1);
                    }
                }
            }
        }

        result
    }
}
#[ignore]
#[test]
fn test_perft() {
    let position_parser = PositionParser::new();

    for (n,(sfen,answer)) in BufReader::new(
        File::open(
            Path::new("data").join("floodgate").join("generatemoves").join("kyokumen_sfen_uniq.txt")
        ).unwrap()).lines().zip(BufReader::new(
        File::open(
            Path::new("data").join("floodgate").join("generatemoves").join("answer_perft_uniq.txt")
        ).unwrap()).lines()).enumerate() {

        let expected = PerftResult::from(answer.unwrap());

        let sfen = format!("sfen {}",sfen.unwrap());

        let (teban, banmen, mc, _, _) = position_parser.parse(&sfen.split(' ').collect::<Vec<&str>>()).unwrap().extract();

        let state = State::new(banmen);

        let solver = PerftSolverByNonEvasions;

        let result = solver.perft(teban,&state,&mc,None,7);

        if &expected != &result {
            println!("line {}: {}",n, sfen);
        }

        assert_eq!(expected, result);
    }
}
#[ignore]
#[test]
fn test_perft_by_evasions() {
    let position_parser = PositionParser::new();

    for (n,(sfen,answer)) in BufReader::new(
        File::open(
            Path::new("data").join("floodgate").join("generatemoves").join("kyokumen_sfen_uniq.txt")
        ).unwrap()).lines().zip(BufReader::new(
        File::open(
            Path::new("data").join("floodgate").join("generatemoves").join("answer_perft_by_evasions_uniq.txt")
        ).unwrap()).lines()).enumerate() {

        let expected = PerftResult::from(answer.unwrap());

        let sfen = format!("sfen {}",sfen.unwrap());

        let (teban, banmen, mc, _, _) = position_parser.parse(&sfen.split(' ').collect::<Vec<&str>>()).unwrap().extract();

        let state = State::new(banmen);

        let solver = PerftSolverByEvasions;

        let result = solver.perft(teban,&state,&mc,None,7);

        if &expected != &result {
            println!("line {}: {}",n, sfen);
        }

        assert_eq!(expected, result);
    }
}