use usiagent::bitboard::BitBoard;
use usiagent::rule::{State, H_MASK, KAKU_TO_RIGHT_BOTTOM_MASK_MAP, KAKU_TO_RIGHT_TOP_MASK_MAP, V_MASK};
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
#[test]
fn test_gen_control_bits_by_kyou() {
    let test_data = vec![
        (vec![((4,6,KomaKind::SKyou))],4,6,BitBoard::from(0b000111111u128 << 36))
    ];

    for (i,(pieces,sx,sy,expected)) in test_data.into_iter().enumerate() {
        let mut b = blank();
        for &(x,y,k) in pieces.iter() {
            set_piece(&mut b,x,y,k);
        }
        let state = State::new(b);

        assert_eq!(usiagent::rule::Rule::gen_control_bits_by_kyou(
            state.get_part().gote_opponent_board,
            state.get_part().gote_self_board,
            idx(sx,sy)
        ).reverse(),
        expected << 1,"index = {}",i);
    }
}
#[test]
fn test_gen_control_bits_by_kaku() {
    let test_data = vec![
        (vec![((3,3,KomaKind::SKaku))],3,3,
         (BitBoard::from(KAKU_TO_RIGHT_BOTTOM_MASK_MAP[idx(3,3) as usize]) |
          BitBoard::from(KAKU_TO_RIGHT_TOP_MASK_MAP[idx(3,3) as usize])) & !(1 << idx(3,3))
        ),
        (vec![((3,5,KomaKind::SKaku))],3,5,
         (BitBoard::from(KAKU_TO_RIGHT_BOTTOM_MASK_MAP[idx(3,5) as usize]) |
             BitBoard::from(KAKU_TO_RIGHT_TOP_MASK_MAP[idx(3,5) as usize])) & !(1 << idx(3,5))
        ),
        (vec![((5,3,KomaKind::SKaku))],5,3,
         (BitBoard::from(KAKU_TO_RIGHT_BOTTOM_MASK_MAP[idx(5,3) as usize]) |
             BitBoard::from(KAKU_TO_RIGHT_TOP_MASK_MAP[idx(5,3) as usize])) & !(1 << idx(5,3))
        ),
        (vec![((5,5,KomaKind::SKaku))],5,5,
         (BitBoard::from(KAKU_TO_RIGHT_BOTTOM_MASK_MAP[idx(5,5) as usize]) |
             BitBoard::from(KAKU_TO_RIGHT_TOP_MASK_MAP[idx(5,5) as usize])) & !(1 << idx(5,5))
        ),
    ];

    for (i,(pieces,sx,sy,expected)) in test_data.into_iter().enumerate() {
        let mut b = blank();
        for &(x,y,k) in pieces.iter() {
            set_piece(&mut b,x,y,k);
        }
        let state = State::new(b);

        assert_eq!(usiagent::rule::Rule::gen_control_bits_by_kaku(
            state.get_part().sente_self_board,
            state.get_part().sente_opponent_board,
            state.get_part().gote_opponent_board,
            state.get_part().gote_self_board,
            idx(sx,sy)
        ), expected << 1,"index = {}",i);
    }
}
#[test]
fn test_gen_control_bits_by_hisha() {
    let test_data = vec![
        (vec![((3,3,KomaKind::SHisha))],3,3,(
            BitBoard::from(V_MASK << (3 * 9)) | BitBoard::from(H_MASK << 3)
        ) & !(1 << idx(3,3))),
        (vec![((3,5,KomaKind::SHisha))],3,5,(
            BitBoard::from(V_MASK << (3 * 9)) | BitBoard::from(H_MASK << 5)
        ) & !(1 << idx(3,5))),
        (vec![((5,3,KomaKind::SHisha))],5,3,(
            BitBoard::from(V_MASK << (5 * 9)) | BitBoard::from(H_MASK << 3)
        ) & !(1 << idx(5,3))),
        (vec![((5,5,KomaKind::SHisha))],5,5,(
            BitBoard::from(V_MASK << (5 * 9)) | BitBoard::from(H_MASK << 5)
        ) & !(1 << idx(5,5)))
    ];

    for (i,(pieces,sx,sy,expected)) in test_data.into_iter().enumerate() {
        let mut b = blank();
        for &(x,y,k) in pieces.iter() {
            set_piece(&mut b,x,y,k);
        }
        let state = State::new(b);

        assert_eq!(usiagent::rule::Rule::gen_control_bits_by_hisha(
            state.get_part().sente_self_board,
            state.get_part().sente_opponent_board,
            state.get_part().gote_opponent_board,
            state.get_part().gote_self_board,
            idx(sx,sy)
        ), expected << 1,"index = {}",i);
    }
}