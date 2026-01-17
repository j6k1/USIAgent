use usiagent::rule::Rule;
use usiagent::rule::State;
use usiagent::shogi::*;
use usiagent::shogi::KomaKind::*;

#[inline]
fn idx(x:u32,y:u32) -> i32 { (x*9 + y) as i32 }

#[inline]
fn set_piece(b:&mut Banmen, x:u32, y:u32, k:KomaKind) { b.0[y as usize][x as usize] = k; }

fn blank() -> Banmen { Banmen([[Blank;9];9]) }

// This file adds regression tests for a bug where has_control/has_control_bits
// mistakenly count control for promoted-to-gold pieces (Tokin, Narikyou, Narikei, Narigin)
// on squares that are only reachable by their UNPROMOTED movement.
// Promoted-gold movers must follow gold's move set only.

// ---------- SENTE side ----------

#[test]
fn bug_sente_narikyou_should_not_control_far_forward_like_lance() {
    // Target square: (4,4)
    // Place a promoted lance (narikyou) at (4,6). If it were an unpromoted lance,
    // it could slide to (4,4). As a gold-equivalent, it cannot move two steps.
    let mut b = blank();
    set_piece(&mut b, 4,6, SKyouN);
    let s = State::new(b);
    // Bits API should return empty
    let bb = Rule::has_control_bits_sente_nari_kin(&s, idx(4,4));
    assert_eq!(<(u64,u64)>::from(bb), (0u64, 0u64), "promoted lance must not control 4,4 from 4,6 via lance sliding");
    // Boolean API should be false
    assert!(!Rule::has_control_sente_nari_kin(&s, idx(4,4)));
}

#[test]
fn bug_sente_narikei_should_not_control_knight_jump() {
    // Target: (3,2) and (5,2)
    // Place a promoted knight at (4,4). Unpromoted knight could jump to (3,2) or (5,2),
    // but promoted gold cannot.
    let mut b = blank();
    set_piece(&mut b, 4,4, SKeiN);
    let s = State::new(b);
    let bb1 = Rule::has_control_bits_sente_nari_kin(&s, idx(3,2));
    assert_eq!(<(u64,u64)>::from(bb1), (0u64, 0u64), "promoted knight must not control 3,2 via knight jump");
    assert!(!Rule::has_control_sente_nari_kin(&s, idx(3,2)));

    let bb2 = Rule::has_control_bits_sente_nari_kin(&s, idx(5,2));
    assert_eq!(<(u64,u64)>::from(bb2), (0u64, 0u64), "promoted knight must not control 5,2 via knight jump");
    assert!(!Rule::has_control_sente_nari_kin(&s, idx(5,2)));
}

#[test]
fn bug_sente_narigin_should_control_forward_diagonals() {
    // Target: (4,4)
    // For Sente, the squares (3,5) and (5,5) are forward-diagonals to the target (4,4).
    // Promoted silver (gold move set) DOES control 4,4 from these squares.
    for &(x,y) in &[(3u32,5u32), (5,5)] {
        let mut b = blank();
        set_piece(&mut b, x,y, SGinN);
        let s = State::new(b);
        let bb = Rule::has_control_bits_sente_nari_kin(&s, idx(4,4));
        // Bitboard should be non-zero (function returns Sente-perspective bitboard of attackers)
        assert_ne!(<(u64,u64)>::from(bb), (0u64, 0u64), "Sente promoted silver at ({},{}) should control 4,4 (forward-diagonal)", x, y);
        assert!(Rule::has_control_sente_nari_kin(&s, idx(4,4)), "Sente promoted silver at ({},{}) should control 4,4", x, y);
    }
}

// ---------- GOTE side ----------

#[test]
fn bug_gote_narikyou_should_not_control_far_forward_like_lance() {
    // For Gote, forward direction towards increasing y as used in existing tests.
    // Place GKyouN at (4,2). If unpromoted, it could slide to (4,4); promoted gold cannot move two.
    let mut b = blank();
    set_piece(&mut b, 4,2, GKyouN);
    let s = State::new(b);
    let bb = Rule::has_control_bits_gote_nari_kin(&s, idx(4,4));
    assert_eq!(<(u64,u64)>::from(bb), (0u64, 0u64), "Gote promoted lance must not control 4,4 from 4,2 via lance sliding");
    assert!(!Rule::has_control_gote_nari_kin(&s, idx(4,4)));
}

#[test]
fn bug_gote_narikei_should_not_control_knight_jump() {
    // Place GKeiN at (4,4). Unpromoted Gote knight could attack (3,6) or (5,6).
    let mut b = blank();
    set_piece(&mut b, 4,4, GKeiN);
    let s = State::new(b);
    for &(x,y) in &[(3u32,6u32), (5,6)] {
        let bb = Rule::has_control_bits_gote_nari_kin(&s, idx(x,y));
        assert_eq!(<(u64,u64)>::from(bb), (0u64, 0u64), "Gote promoted knight must not control {},{} via knight jump", x, y);
        assert!(!Rule::has_control_gote_nari_kin(&s, idx(x,y)));
    }
}

#[test]
fn bug_gote_narigin_should_control_forward_diagonals() {
    // For Gote, the squares (3,3) and (5,3) are forward-diagonals to the target (4,4).
    // Promoted silver (gold move set) DOES control 4,4 from these squares.
    for &(x,y) in &[(3u32,3u32), (5,3)] {
        let mut b = blank();
        set_piece(&mut b, x,y, GGinN);
        let s = State::new(b);
        let bb = Rule::has_control_bits_gote_nari_kin(&s, idx(4,4));
        // Bitboard should be non-zero (function returns Sente-perspective bitboard of attackers)
        assert_ne!(<(u64,u64)>::from(bb), (0u64, 0u64), "Gote promoted silver at ({},{}) should control 4,4 (forward-diagonal)", x, y);
        assert!(Rule::has_control_gote_nari_kin(&s, idx(4,4)), "Gote promoted silver at ({},{}) should control 4,4", x, y);
    }
}
