use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use usiagent::math::Prng;
use usiagent::movepick::RandomPicker;
use usiagent::protocol::{ParsePosition, PositionParser, ToSfen};
use usiagent::rule::{Checks, Rule, State};

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
            Path::new("data").join("random").join("generatemoves").join("answer_has_blocking_move.txt")
        ).unwrap()).lines()).enumerate() {

        let mut expected = answer.unwrap().split(',').into_iter().map(|m| m.to_string()).collect::<Vec<String>>();

        expected.sort();

        let expected = expected.join(" ");

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
