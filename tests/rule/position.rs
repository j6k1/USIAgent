use usiagent::rule::*;
use usiagent::shogi::{Banmen, Teban};
use usiagent::shogi::KomaKind::{Blank, SOu, GOu};

#[test]
fn test_sente_ou_square() {
    let positions:Vec<(Square,Square)> = vec![(2,0),(2,8),(3,0),(3,8),(6,0),(6,8),(7,0),(7,8)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &(x,y) in positions.iter() {
        let mut banmen = blank_banmen.clone();

        banmen.0[y as usize][x as usize] = SOu;

        assert_eq!(Rule::ou_square(Teban::Sente,&State::new(banmen)),x * 9 + y);
    }
}
#[test]
fn test_gote_ou_square() {
    let positions:Vec<(Square,Square)> = vec![(2,0),(2,8),(3,0),(3,8),(6,0),(6,8),(7,0),(7,8)];

    let blank_banmen = Banmen([[Blank; 9]; 9]);

    for &(x,y) in positions.iter() {
        let mut banmen = blank_banmen.clone();

        banmen.0[y as usize][x as usize] = GOu;

        assert_eq!(Rule::ou_square(Teban::Gote,&State::new(banmen)),x * 9 + y);
    }
}
