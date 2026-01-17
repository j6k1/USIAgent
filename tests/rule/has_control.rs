use usiagent::shogi::*;
use usiagent::rule::Rule;
use usiagent::rule::State;

#[allow(unused)]
use usiagent::shogi::KomaKind::{
    SFu, SKyou, SKei, SGin, SKin, SKaku, SHisha, SOu,
    SFuN, SKyouN, SKeiN, SGinN, SKakuN, SHishaN,
    GFu, GKyou, GKei, GGin, GKin, GKaku, GHisha, GOu,
    GFuN, GKyouN, GKeiN, GGinN, GKakuN, GHishaN,
    Blank
};

#[inline]
fn idx(x:u32,y:u32) -> i32 { (x*9 + y) as i32 }

#[inline]
fn set_piece(b:&mut Banmen, x:u32, y:u32, k:KomaKind) { b.0[y as usize][x as usize] = k; }

#[inline]
fn bit_for_pos(x:u32,y:u32) -> u128 { 1u128 << (idx(x,y) + 1) }

#[inline]
fn bb_from_positions(ps:&[(u32,u32)]) -> u128 {
    let mut m: u128 = 0;
    for &(x,y) in ps { m |= bit_for_pos(x,y); }
    m
}

fn blank() -> Banmen { Banmen([[Blank;9];9]) }

// ---------- SENTE tests ----------
#[test]
fn has_control_bits_sente_fu_variants() {
    let target = (4,4);
    // empty target
    let mut b = blank();
    set_piece(&mut b, 4,5, SFu);
    let s = State::new(b);
    let got = Rule::has_control_bits_sente_fu(&s, idx(target.0,target.1));
    let expect = bb_from_positions(&[(4,5)]);
    assert_eq!(got, expect, "empty target");

    // friendly on target
    let mut b = blank();
    set_piece(&mut b, 4,4, SKin);
    set_piece(&mut b, 4,5, SFu);
    let s = State::new(b);
    let got = Rule::has_control_bits_sente_fu(&s, idx(target.0,target.1));
    assert_eq!(got, expect, "friendly occupied");

    // opponent on target
    let mut b = blank();
    set_piece(&mut b, 4,4, GFu);
    set_piece(&mut b, 4,5, SFu);
    let s = State::new(b);
    let got = Rule::has_control_bits_sente_fu(&s, idx(target.0,target.1));
    assert_eq!(got, expect, "opponent occupied");
}

#[test]
fn has_control_bits_sente_kyou_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 4,8, SKyou);
    let s = State::new(b);
    let got = Rule::has_control_bits_sente_kyou(&s, idx(target.0,target.1));
    let expect = bb_from_positions(&[(4,8)]);
    assert_eq!(got, expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SKin);
    set_piece(&mut b, 4,8, SKyou);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_kyou(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GFu);
    set_piece(&mut b, 4,8, SKyou);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_kyou(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_sente_kei_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 3,6, SKei);
    set_piece(&mut b, 5,6, SKei);
    let s = State::new(b);
    let expect = bb_from_positions(&[(3,6),(5,6)]);
    assert_eq!(Rule::has_control_bits_sente_kei(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SKin);
    set_piece(&mut b, 3,6, SKei);
    set_piece(&mut b, 5,6, SKei);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_kei(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GFu);
    set_piece(&mut b, 3,6, SKei);
    set_piece(&mut b, 5,6, SKei);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_kei(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_sente_gin_variants() {
    let target = (4,4);
    let mut b = blank();
    for &(x,y) in [(4,5),(3,3),(5,3)].iter() { set_piece(&mut b, x,y, SGin); }
    let s = State::new(b);
    let expect = bb_from_positions(&[(4,5),(3,3),(5,3)]);
    assert_eq!(Rule::has_control_bits_sente_gin(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SKin);
    for &(x,y) in [(4,5),(3,3),(5,3)].iter() { set_piece(&mut b, x,y, SGin); }
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_gin(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GFu);
    for &(x,y) in [(4,5),(3,3),(5,3)].iter() { set_piece(&mut b, x,y, SGin); }
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_gin(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_sente_kin_variants() {
    let target = (4,4);
    let mut b = blank();
    for &(x,y) in [(4,5),(3,5),(5,5)].iter() { set_piece(&mut b, x,y, SKin); }
    let s = State::new(b);
    let expect = bb_from_positions(&[(4,5),(3,5),(5,5)]);
    assert_eq!(Rule::has_control_bits_sente_kin(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SKin);
    for &(x,y) in [(4,5),(3,5),(5,5)].iter() { set_piece(&mut b, x,y, SKin); }
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_kin(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GFu);
    for &(x,y) in [(4,5),(3,5),(5,5)].iter() { set_piece(&mut b, x,y, SKin); }
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_kin(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_sente_kaku_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 1,1, SKaku);
    set_piece(&mut b, 7,1, SKaku);
    let s = State::new(b);
    let expect = bb_from_positions(&[(1,1),(7,1)]);
    assert_eq!(Rule::has_control_bits_sente_kaku(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SKin);
    set_piece(&mut b, 1,1, SKaku);
    set_piece(&mut b, 7,1, SKaku);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_kaku(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GFu);
    set_piece(&mut b, 1,1, SKaku);
    set_piece(&mut b, 7,1, SKaku);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_kaku(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_sente_hisha_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 4,8, SHisha);
    set_piece(&mut b, 0,4, SHisha);
    let s = State::new(b);
    let expect = bb_from_positions(&[(4,8),(0,4)]);
    assert_eq!(Rule::has_control_bits_sente_hisha(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SKin);
    set_piece(&mut b, 4,8, SHisha);
    set_piece(&mut b, 0,4, SHisha);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_hisha(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GFu);
    set_piece(&mut b, 4,8, SHisha);
    set_piece(&mut b, 0,4, SHisha);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_hisha(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_sente_ou_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 4,5, SOu);
    let s = State::new(b);
    let expect = bb_from_positions(&[(4,5)]);
    assert_eq!(Rule::has_control_bits_sente_ou(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SKin);
    set_piece(&mut b, 4,5, SOu);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_ou(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GFu);
    set_piece(&mut b, 4,5, SOu);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_ou(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_sente_nari_kin_variants() {
    let target = (4,4);
    for &kind in [SFuN, SKyouN, SKeiN, SGinN].iter() {
        // empty
        let mut b = blank();
        set_piece(&mut b, 4,5, kind);
        let s = State::new(b);
        let expect = bb_from_positions(&[(4,5)]);
        assert_eq!(Rule::has_control_bits_sente_nari_kin(&s, idx(target.0,target.1)), expect);
        // friendly
        let mut b = blank();
        set_piece(&mut b, 4,4, SKin);
        set_piece(&mut b, 4,5, kind);
        let s = State::new(b);
        assert_eq!(Rule::has_control_bits_sente_nari_kin(&s, idx(4,4)), expect);
        // opponent
        let mut b = blank();
        set_piece(&mut b, 4,4, GFu);
        set_piece(&mut b, 4,5, kind);
        let s = State::new(b);
        assert_eq!(Rule::has_control_bits_sente_nari_kin(&s, idx(4,4)), expect);
    }
}

#[test]
fn has_control_bits_sente_kaku_nari_variants() {
    let target = (4,4);
    // place馬 adjacent so the one-step mask surely reaches target
    let mut b = blank();
    set_piece(&mut b, 3,4, SKakuN);
    let s = State::new(b);
    let expect = bb_from_positions(&[(3,4)]);
    assert_eq!(Rule::has_control_bits_sente_kaku_nari(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SKin);
    set_piece(&mut b, 3,4, SKakuN);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_kaku_nari(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GFu);
    set_piece(&mut b, 3,4, SKakuN);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_kaku_nari(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_sente_hisha_nari_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 4,5, SHishaN);
    let s = State::new(b);
    let expect = bb_from_positions(&[(4,5)]);
    assert_eq!(Rule::has_control_bits_sente_hisha_nari(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SKin);
    set_piece(&mut b, 4,5, SHishaN);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_hisha_nari(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GFu);
    set_piece(&mut b, 4,5, SHishaN);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_sente_hisha_nari(&s, idx(4,4)), expect);
}

// ---------- GOTE tests ----------
#[test]
fn has_control_bits_gote_fu_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 4,3, GFu);
    let s = State::new(b);
    let expect = bb_from_positions(&[(4,3)]);
    assert_eq!(Rule::has_control_bits_gote_fu(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GKin);
    set_piece(&mut b, 4,3, GFu);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_fu(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SFu);
    set_piece(&mut b, 4,3, GFu);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_fu(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_gote_kyou_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 4,0, GKyou);
    let s = State::new(b);
    let expect = bb_from_positions(&[(4,0)]);
    assert_eq!(Rule::has_control_bits_gote_kyou(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GKin);
    set_piece(&mut b, 4,0, GKyou);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_kyou(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SFu);
    set_piece(&mut b, 4,0, GKyou);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_kyou(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_gote_kei_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 3,2, GKei);
    set_piece(&mut b, 5,2, GKei);
    let s = State::new(b);
    let expect = bb_from_positions(&[(3,2),(5,2)]);
    assert_eq!(Rule::has_control_bits_gote_kei(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GKin);
    set_piece(&mut b, 3,2, GKei);
    set_piece(&mut b, 5,2, GKei);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_kei(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SFu);
    set_piece(&mut b, 3,2, GKei);
    set_piece(&mut b, 5,2, GKei);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_kei(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_gote_gin_variants() {
    let target = (4,4);
    let mut b = blank();
    for &(x,y) in [(4,3),(3,5),(5,5)].iter() { set_piece(&mut b, x,y, GGin); }
    let s = State::new(b);
    let expect = bb_from_positions(&[(4,3),(3,5),(5,5)]);
    assert_eq!(Rule::has_control_bits_gote_gin(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GKin);
    for &(x,y) in [(4,3),(3,5),(5,5)].iter() { set_piece(&mut b, x,y, GGin); }
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_gin(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SFu);
    for &(x,y) in [(4,3),(3,5),(5,5)].iter() { set_piece(&mut b, x,y, GGin); }
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_gin(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_gote_kin_variants() {
    let target = (4,4);
    let mut b = blank();
    for &(x,y) in [(4,3),(3,4),(5,4)].iter() { set_piece(&mut b, x,y, GKin); }
    let s = State::new(b);
    let expect = bb_from_positions(&[(4,3),(3,4),(5,4)]);
    assert_eq!(Rule::has_control_bits_gote_kin(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GKin);
    for &(x,y) in [(4,3),(3,4),(5,4)].iter() { set_piece(&mut b, x,y, GKin); }
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_kin(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SFu);
    for &(x,y) in [(4,3),(3,4),(5,4)].iter() { set_piece(&mut b, x,y, GKin); }
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_kin(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_gote_kaku_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 1,7, GKaku);
    set_piece(&mut b, 7,7, GKaku);
    let s = State::new(b);
    let expect = bb_from_positions(&[(1,7),(7,7)]);
    assert_eq!(Rule::has_control_bits_gote_kaku(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GKin);
    set_piece(&mut b, 1,7, GKaku);
    set_piece(&mut b, 7,7, GKaku);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_kaku(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SFu);
    set_piece(&mut b, 1,7, GKaku);
    set_piece(&mut b, 7,7, GKaku);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_kaku(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_gote_hisha_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 4,0, GHisha);
    set_piece(&mut b, 8,4, GHisha);
    let s = State::new(b);
    let expect = bb_from_positions(&[(4,0),(8,4)]);
    assert_eq!(Rule::has_control_bits_gote_hisha(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GKin);
    set_piece(&mut b, 4,0, GHisha);
    set_piece(&mut b, 8,4, GHisha);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_hisha(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SFu);
    set_piece(&mut b, 4,0, GHisha);
    set_piece(&mut b, 8,4, GHisha);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_hisha(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_gote_ou_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 4,3, GOu);
    let s = State::new(b);
    let expect = bb_from_positions(&[(4,3)]);
    assert_eq!(Rule::has_control_bits_gote_ou(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GKin);
    set_piece(&mut b, 4,3, GOu);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_ou(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SFu);
    set_piece(&mut b, 4,3, GOu);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_ou(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_gote_nari_kin_variants() {
    let target = (4,4);
    for &kind in [GFuN, GKyouN, GKeiN, GGinN].iter() {
        let mut b = blank();
        set_piece(&mut b, 4,3, kind);
        let s = State::new(b);
        let expect = bb_from_positions(&[(4,3)]);
        assert_eq!(Rule::has_control_bits_gote_nari_kin(&s, idx(target.0,target.1)), expect);

        let mut b = blank();
        set_piece(&mut b, 4,4, GKin);
        set_piece(&mut b, 4,3, kind);
        let s = State::new(b);
        assert_eq!(Rule::has_control_bits_gote_nari_kin(&s, idx(4,4)), expect);

        let mut b = blank();
        set_piece(&mut b, 4,4, SFu);
        set_piece(&mut b, 4,3, kind);
        let s = State::new(b);
        assert_eq!(Rule::has_control_bits_gote_nari_kin(&s, idx(4,4)), expect);
    }
}

#[test]
fn has_control_bits_gote_kaku_nari_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 5,4, GKakuN);
    let s = State::new(b);
    let expect = bb_from_positions(&[(5,4)]);
    assert_eq!(Rule::has_control_bits_gote_kaku_nari(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GKin);
    set_piece(&mut b, 5,4, GKakuN);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_kaku_nari(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SFu);
    set_piece(&mut b, 5,4, GKakuN);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_kaku_nari(&s, idx(4,4)), expect);
}

#[test]
fn has_control_bits_gote_hisha_nari_variants() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 4,3, GHishaN);
    let s = State::new(b);
    let expect = bb_from_positions(&[(4,3)]);
    assert_eq!(Rule::has_control_bits_gote_hisha_nari(&s, idx(target.0,target.1)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, GKin);
    set_piece(&mut b, 4,3, GHishaN);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_hisha_nari(&s, idx(4,4)), expect);

    let mut b = blank();
    set_piece(&mut b, 4,4, SFu);
    set_piece(&mut b, 4,3, GHishaN);
    let s = State::new(b);
    assert_eq!(Rule::has_control_bits_gote_hisha_nari(&s, idx(4,4)), expect);
}

// Additional coverage: promoted rook (dragon) diagonal one-step should control target.
#[test]
fn has_control_bits_sente_hisha_nari_diagonal() {
    let target = (4,4);
    let mut b = blank();
    // Dragon placed diagonally adjacent so only the diagonal king-like step applies
    set_piece(&mut b, 3,3, SHishaN);
    let s = State::new(b);
    let expect = bb_from_positions(&[(3,3)]);
    assert_eq!(Rule::has_control_bits_sente_hisha_nari(&s, idx(target.0,target.1)), expect);
}

#[test]
fn has_control_bits_gote_hisha_nari_diagonal() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 5,5, GHishaN);
    let s = State::new(b);
    let expect = bb_from_positions(&[(5,5)]);
    assert_eq!(Rule::has_control_bits_gote_hisha_nari(&s, idx(target.0,target.1)), expect);
}

// Additional coverage: promoted bishop (horse) must retain diagonal sliding control.
#[test]
fn has_control_bits_sente_kaku_nari_diagonal_slide() {
    let target = (4,4);
    let mut b = blank();
    // Place horse on clear diagonal; it should slide like a bishop
    set_piece(&mut b, 2,2, SKakuN);
    let s = State::new(b);
    let expect = bb_from_positions(&[(2,2)]);
    assert_eq!(Rule::has_control_bits_sente_kaku_nari(&s, idx(target.0,target.1)), expect);
}

#[test]
fn has_control_bits_gote_kaku_nari_diagonal_slide() {
    let target = (4,4);
    let mut b = blank();
    set_piece(&mut b, 6,6, GKakuN);
    let s = State::new(b);
    let expect = bb_from_positions(&[(6,6)]);
    assert_eq!(Rule::has_control_bits_gote_kaku_nari(&s, idx(target.0,target.1)), expect);
}


// ---------- Boolean has_control() tests moved from src/rule/has_control.rs ----------

#[test]
fn has_control_sente_fu_variants() {
    let target = (4,4);
    // empty
    let mut b = blank(); set_piece(&mut b, 4,5, SFu);
    assert!(Rule::has_control_sente_fu(&State::new(b), idx(target.0,target.1)));
    // friendly on target -> false
    let mut b = blank(); set_piece(&mut b, 4,4, SKin); set_piece(&mut b, 4,5, SFu);
    assert!(Rule::has_control_sente_fu(&State::new(b), idx(target.0,target.1)));
    // opponent on target -> true
    let mut b = blank(); set_piece(&mut b, 4,4, GFu); set_piece(&mut b, 4,5, SFu);
    assert!(Rule::has_control_sente_fu(&State::new(b), idx(target.0,target.1)));
}

#[test]
fn has_control_sente_kyou_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 4,8, SKyou);
    assert!(Rule::has_control_sente_kyou(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, SKin); set_piece(&mut b, 4,8, SKyou);
    assert!(Rule::has_control_sente_kyou(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, GFu); set_piece(&mut b, 4,8, SKyou);
    assert!(Rule::has_control_sente_kyou(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_sente_kei_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 3,6, SKei); set_piece(&mut b, 5,6, SKei);
    assert!(Rule::has_control_sente_kei(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, SKin); set_piece(&mut b, 3,6, SKei); set_piece(&mut b, 5,6, SKei);
    assert!(Rule::has_control_sente_kei(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, GFu); set_piece(&mut b, 3,6, SKei); set_piece(&mut b, 5,6, SKei);
    assert!(Rule::has_control_sente_kei(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_sente_gin_variants() {
    let target = (4,4);
    let mut b = blank(); for &(x,y) in [(4,5),(3,3),(5,3)].iter() { set_piece(&mut b,x,y, SGin); }
    assert!(Rule::has_control_sente_gin(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, SKin); for &(x,y) in [(4,5),(3,3),(5,3)].iter(){ set_piece(&mut b,x,y, SGin);} 
    assert!(Rule::has_control_sente_gin(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, GFu); for &(x,y) in [(4,5),(3,3),(5,3)].iter(){ set_piece(&mut b,x,y, SGin);} 
    assert!(Rule::has_control_sente_gin(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_sente_kin_variants() {
    let target = (4,4);
    let mut b = blank(); for &(x,y) in [(4,5),(3,5),(5,5)].iter() { set_piece(&mut b,x,y, SKin);} 
    assert!(Rule::has_control_sente_kin(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, SKin); for &(x,y) in [(4,5),(3,5),(5,5)].iter(){ set_piece(&mut b,x,y, SKin);} 
    assert!(Rule::has_control_sente_kin(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, GFu); for &(x,y) in [(4,5),(3,5),(5,5)].iter(){ set_piece(&mut b,x,y, SKin);} 
    assert!(Rule::has_control_sente_kin(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_sente_kaku_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 1,1, SKaku); set_piece(&mut b, 7,1, SKaku);
    assert!(Rule::has_control_sente_kaku(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, SKin); set_piece(&mut b, 1,1, SKaku); set_piece(&mut b, 7,1, SKaku);
    assert!(Rule::has_control_sente_kaku(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, GFu); set_piece(&mut b, 1,1, SKaku); set_piece(&mut b, 7,1, SKaku);
    assert!(Rule::has_control_sente_kaku(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_sente_hisha_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 4,8, SHisha); set_piece(&mut b, 0,4, SHisha);
    assert!(Rule::has_control_sente_hisha(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, SKin); set_piece(&mut b, 4,8, SHisha); set_piece(&mut b, 0,4, SHisha);
    assert!(Rule::has_control_sente_hisha(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, GFu); set_piece(&mut b, 4,8, SHisha); set_piece(&mut b, 0,4, SHisha);
    assert!(Rule::has_control_sente_hisha(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_sente_ou_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 4,5, SOu);
    assert!(Rule::has_control_sente_ou(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, SKin); set_piece(&mut b, 4,5, SOu);
    assert!(Rule::has_control_sente_ou(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, GFu); set_piece(&mut b, 4,5, SOu);
    assert!(Rule::has_control_sente_ou(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_sente_nari_kin_variants() {
    let target = (4,4);
    for &k in [SFuN, SKyouN, SKeiN, SGinN].iter() {
        let mut b = blank(); set_piece(&mut b, 4,5, k);
        assert!(Rule::has_control_sente_nari_kin(&State::new(b), idx(target.0,target.1)));
        let mut b = blank(); set_piece(&mut b, 4,4, SKin); set_piece(&mut b, 4,5, k);
        assert!(Rule::has_control_sente_nari_kin(&State::new(b), idx(4,4)));
        let mut b = blank(); set_piece(&mut b, 4,4, GFu); set_piece(&mut b, 4,5, k);
        assert!(Rule::has_control_sente_nari_kin(&State::new(b), idx(4,4)));
    }
}

#[test]
fn has_control_sente_kaku_nari_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 3,4, SKakuN);
    assert!(Rule::has_control_sente_kaku_nari(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, SKin); set_piece(&mut b, 3,4, SKakuN);
    assert!(Rule::has_control_sente_kaku_nari(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, GFu); set_piece(&mut b, 3,4, SKakuN);
    assert!(Rule::has_control_sente_kaku_nari(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_sente_hisha_nari_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 4,5, SHishaN);
    assert!(Rule::has_control_sente_hisha_nari(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, SKin); set_piece(&mut b, 4,5, SHishaN);
    assert!(Rule::has_control_sente_hisha_nari(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, GFu); set_piece(&mut b, 4,5, SHishaN);
    assert!(Rule::has_control_sente_hisha_nari(&State::new(b), idx(4,4)));
}

// ---------------- GOTE ----------------
#[test]
fn has_control_gote_fu_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 4,3, GFu);
    assert!(Rule::has_control_gote_fu(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, GKin); set_piece(&mut b, 4,3, GFu);
    assert!(Rule::has_control_gote_fu(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, SFu); set_piece(&mut b, 4,3, GFu);
    assert!(Rule::has_control_gote_fu(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_gote_kyou_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 4,0, GKyou);
    assert!(Rule::has_control_gote_kyou(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, GKin); set_piece(&mut b, 4,0, GKyou);
    assert!(Rule::has_control_gote_kyou(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, SFu); set_piece(&mut b, 4,0, GKyou);
    assert!(Rule::has_control_gote_kyou(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_gote_kei_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 3,2, GKei); set_piece(&mut b, 5,2, GKei);
    assert!(Rule::has_control_gote_kei(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, GKin); set_piece(&mut b, 3,2, GKei); set_piece(&mut b, 5,2, GKei);
    assert!(Rule::has_control_gote_kei(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, SFu); set_piece(&mut b, 3,2, GKei); set_piece(&mut b, 5,2, GKei);
    assert!(Rule::has_control_gote_kei(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_gote_gin_variants() {
    let target = (4,4);
    let mut b = blank(); for &(x,y) in [(4,3),(3,5),(5,5)].iter() { set_piece(&mut b,x,y, GGin);} 
    assert!(Rule::has_control_gote_gin(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, GKin); for &(x,y) in [(4,3),(3,5),(5,5)].iter(){ set_piece(&mut b,x,y, GGin);} 
    assert!(Rule::has_control_gote_gin(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, SFu); for &(x,y) in [(4,3),(3,5),(5,5)].iter(){ set_piece(&mut b,x,y, GGin);} 
    assert!(Rule::has_control_gote_gin(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_gote_kin_variants() {
    let target = (4,4);
    let mut b = blank(); for &(x,y) in [(4,3),(3,4),(5,4)].iter() { set_piece(&mut b,x,y, GKin);} 
    assert!(Rule::has_control_gote_kin(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, GKin); for &(x,y) in [(4,3),(3,4),(5,4)].iter(){ set_piece(&mut b,x,y, GKin);} 
    assert!(Rule::has_control_gote_kin(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, SFu); for &(x,y) in [(4,3),(3,4),(5,4)].iter(){ set_piece(&mut b,x,y, GKin);} 
    assert!(Rule::has_control_gote_kin(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_gote_kaku_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 1,7, GKaku); set_piece(&mut b, 7,7, GKaku);
    assert!(Rule::has_control_gote_kaku(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, GKin); set_piece(&mut b, 1,7, GKaku); set_piece(&mut b, 7,7, GKaku);
    assert!(Rule::has_control_gote_kaku(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, SFu); set_piece(&mut b, 1,7, GKaku); set_piece(&mut b, 7,7, GKaku);
    assert!(Rule::has_control_gote_kaku(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_gote_hisha_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 4,0, GHisha); set_piece(&mut b, 8,4, GHisha);
    assert!(Rule::has_control_gote_hisha(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, GKin); set_piece(&mut b, 4,0, GHisha); set_piece(&mut b, 8,4, GHisha);
    assert!(Rule::has_control_gote_hisha(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, SFu); set_piece(&mut b, 4,0, GHisha); set_piece(&mut b, 8,4, GHisha);
    assert!(Rule::has_control_gote_hisha(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_gote_ou_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 4,3, GOu);
    assert!(Rule::has_control_gote_ou(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, GKin); set_piece(&mut b, 4,3, GOu);
    assert!(Rule::has_control_gote_ou(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, SFu); set_piece(&mut b, 4,3, GOu);
    assert!(Rule::has_control_gote_ou(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_gote_nari_kin_variants() {
    let target = (4,4);
    for &k in [GFuN, GKyouN, GKeiN, GGinN].iter() {
        let mut b = blank(); set_piece(&mut b, 4,3, k);
        assert!(Rule::has_control_gote_nari_kin(&State::new(b), idx(target.0,target.1)));
        let mut b = blank(); set_piece(&mut b, 4,4, GKin); set_piece(&mut b, 4,3, k);
        assert!(Rule::has_control_gote_nari_kin(&State::new(b), idx(4,4)));
        let mut b = blank(); set_piece(&mut b, 4,4, SFu); set_piece(&mut b, 4,3, k);
        assert!(Rule::has_control_gote_nari_kin(&State::new(b), idx(4,4)));
    }
}

#[test]
fn has_control_gote_kaku_nari_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 5,4, GKakuN);
    assert!(Rule::has_control_gote_kaku_nari(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, GKin); set_piece(&mut b, 5,4, GKakuN);
    assert!(Rule::has_control_gote_kaku_nari(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, SFu); set_piece(&mut b, 5,4, GKakuN);
    assert!(Rule::has_control_gote_kaku_nari(&State::new(b), idx(4,4)));
}

#[test]
fn has_control_gote_hisha_nari_variants() {
    let target = (4,4);
    let mut b = blank(); set_piece(&mut b, 4,3, GHishaN);
    assert!(Rule::has_control_gote_hisha_nari(&State::new(b), idx(target.0,target.1)));
    let mut b = blank(); set_piece(&mut b, 4,4, GKin); set_piece(&mut b, 4,3, GHishaN);
    assert!(Rule::has_control_gote_hisha_nari(&State::new(b), idx(4,4)));
    let mut b = blank(); set_piece(&mut b, 4,4, SFu); set_piece(&mut b, 4,3, GHishaN);
    assert!(Rule::has_control_gote_hisha_nari(&State::new(b), idx(4,4)));
}
