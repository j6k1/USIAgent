use std::collections::HashSet;

use usiagent::bitboard::BitBoard;
use usiagent::rule::Rule;
use usiagent::shogi::KomaKind::{GKyou, SHisha, SHishaN, SKaku, SKakuN, SKyou};
use usiagent::shogi::{KomaKind, Teban};
use usiagent::superposition::SuperPosition;

#[inline]
fn idx(x: u32, y: u32) -> u32 {
    x * 9 + y
}

#[inline]
fn sq(x: u32, y: u32) -> BitBoard {
    BitBoard::from(1u128 << (idx(x, y) + 1))
}

fn squares(squares: &[(u32, u32)]) -> BitBoard {
    squares
        .iter()
        .fold(BitBoard::default(), |acc, &(x, y)| acc | sq(x, y))
}

fn assert_board(actual: BitBoard, expected: BitBoard) {
    assert_eq!(u128::from(actual), u128::from(expected));
}

fn promoted_extra(teban: Teban, x: u32, y: u32, base: KomaKind, promoted: KomaKind) -> BitBoard {
    let from = idx(x, y);
    Rule::gen_candidate_bits(teban, BitBoard::default(), from, promoted)
        & !Rule::gen_candidate_bits(teban, BitBoard::default(), from, base)
}

fn control_for_piece(teban: Teban, kind: KomaKind, x: u32, y: u32) -> BitBoard {
    match kind {
        SKyou | GKyou => BitBoard::default(),
        SKaku => promoted_extra(teban, x, y, SKaku, SKakuN),
        SHisha => promoted_extra(teban, x, y, SHisha, SHishaN),
        _ => Rule::gen_candidate_bits(teban, BitBoard::default(), idx(x, y), kind),
    }
}

fn add_controls(sp: &mut SuperPosition, pieces: &[(Teban, KomaKind, u32, u32)]) {
    let mut occupied = HashSet::new();

    for &(teban, kind, x, y) in pieces {
        assert!(
            occupied.insert((x, y)),
            "duplicate piece square: ({}, {})",
            x,
            y
        );
        *sp += control_for_piece(teban, kind, x, y);
    }
}

fn remove_controls(sp: &mut SuperPosition, pieces: &[(Teban, KomaKind, u32, u32)]) {
    let mut occupied = HashSet::new();

    for &(teban, kind, x, y) in pieces {
        assert!(
            occupied.insert((x, y)),
            "duplicate piece square: ({}, {})",
            x,
            y
        );
        *sp -= control_for_piece(teban, kind, x, y);
    }
}

#[test]
fn overlapping_control_remains_until_all_sources_are_removed() {
    let rook = [(Teban::Sente, SHisha, 4, 4)];
    let bishop = [(Teban::Sente, SKaku, 3, 4)];
    let rook_control = squares(&[(3, 3), (5, 3), (3, 5), (5, 5)]);
    let bishop_control = squares(&[(2, 4), (4, 4), (3, 3), (3, 5)]);

    let mut sp = SuperPosition::default();

    add_controls(&mut sp, &rook);
    assert_board(sp.to_bitboard(), rook_control);

    add_controls(&mut sp, &bishop);
    assert_board(sp.to_bitboard(), rook_control | bishop_control);

    remove_controls(&mut sp, &rook);
    assert_board(sp.to_bitboard(), bishop_control);

    remove_controls(&mut sp, &bishop);
    assert_board(sp.to_bitboard(), BitBoard::default());
}

#[test]
fn edge_controls_do_not_wrap() {
    let pieces = [(Teban::Sente, SHisha, 0, 0), (Teban::Sente, SKaku, 8, 8)];
    let expected = squares(&[(1, 1), (7, 8), (8, 7)]);

    let mut sp = SuperPosition::default();
    add_controls(&mut sp, &pieces);

    assert_board(sp.to_bitboard(), expected);
}

#[test]
fn lance_controls_are_not_included() {
    let pieces = [(Teban::Sente, SKyou, 4, 8), (Teban::Gote, GKyou, 4, 0)];

    let mut sp = SuperPosition::default();
    add_controls(&mut sp, &pieces);

    assert_board(sp.to_bitboard(), BitBoard::default());
}

#[test]
fn multiple_distinct_controls_can_exist_at_once() {
    let pieces = [(Teban::Sente, SHisha, 4, 4), (Teban::Sente, SKaku, 6, 6)];
    let expected = squares(&[
        (3, 3),
        (5, 3),
        (3, 5),
        (5, 5),
        (5, 6),
        (7, 6),
        (6, 5),
        (6, 7),
    ]);

    let mut sp = SuperPosition::default();
    add_controls(&mut sp, &pieces);

    assert_board(sp.to_bitboard(), expected);
}

#[test]
#[should_panic(expected = "duplicate piece square")]
fn duplicate_piece_squares_are_rejected() {
    let mut sp = SuperPosition::default();
    add_controls(
        &mut sp,
        &[(Teban::Sente, SHisha, 4, 4), (Teban::Sente, SKaku, 4, 4)],
    );
}
