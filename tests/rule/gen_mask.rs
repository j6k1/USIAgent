use usiagent::bitboard::BitBoard;
use usiagent::rule::State;
use usiagent::shogi::{Banmen, KomaKind, Teban};
use usiagent::shogi::KomaKind::Blank;
#[allow(dead_code)]
#[inline]
fn idx(x:u32,y:u32) -> u32 { x*9 + y }

#[inline]
fn set_piece(b:&mut Banmen, x:u32, y:u32, k:KomaKind) { b.0[y as usize][x as usize] = k; }

#[inline]
fn blank() -> Banmen { Banmen([[Blank;9];9]) }

#[test]
fn gen_ou_surrounding_mask_test()
{
    let test_data = vec![
        (Teban::Sente,vec![], BitBoard::default()),
        (Teban::Sente,vec![(0,0,KomaKind::SOu)],BitBoard::from(0b000000011_000000010u128)),
        (Teban::Gote,vec![(0,0,KomaKind::GOu)],BitBoard::from(0b000000011_000000010u128)),
        (Teban::Sente,vec![(0,8,KomaKind::SOu)],BitBoard::from(0b110000000_010000000u128)),
        (Teban::Gote,vec![(0,8,KomaKind::GOu)],BitBoard::from(0b110000000_010000000u128)),
        (Teban::Sente,vec![(8,0,KomaKind::SOu)],BitBoard::from(0b000000010_000000011u128 << 63)),
        (Teban::Gote,vec![(8,0,KomaKind::GOu)],BitBoard::from(0b000000010_000000011u128 << 63)),
        (Teban::Sente,vec![(8,8,KomaKind::SOu)],BitBoard::from(0b010000000_110000000u128 << 63)),
        (Teban::Gote,vec![(8,8,KomaKind::GOu)],BitBoard::from(0b010000000_110000000u128 << 63)),
        (Teban::Sente,vec![(4,4,KomaKind::SOu)],BitBoard::from(0b000111000_000101000_000111000u128 << 27)),
        (Teban::Gote,vec![(4,4,KomaKind::GOu)],BitBoard::from(0b000111000_000101000_000111000u128 << 27)),
        (Teban::Sente,vec![(4,4,KomaKind::SFu)],BitBoard::default()),
        (Teban::Gote,vec![(4,4,KomaKind::GFu)],BitBoard::default()),
    ];

    for (i,(t,pieces,expected)) in test_data.into_iter().enumerate() {
        let mut b = blank();
        for &(x,y,k) in pieces.iter() {
            set_piece(&mut b,x,y,k);
        }
        let state = State::new(b);

        assert_eq!(usiagent::rule::Rule::gen_ou_surrounding_mask(t,state.get_part()),expected << 1,"index = {}",i);
        assert_eq!(usiagent::rule::Rule::gen_ou_surrounding_mask(t.opposite(),state.get_part()),BitBoard::default(),"index = {}",i);
    }
}