use usiagent::rule::Rule;
use usiagent::rule::State;
use usiagent::shogi::*;
use usiagent::shogi::KomaKind::*;
use usiagent::bitboard::BitBoard;

#[inline]
fn idx(x:u32,y:u32) -> i32 { (x*9 + y) as i32 }

#[inline]
fn set_piece(b:&mut Banmen, x:u32, y:u32, k:KomaKind) { b.0[y as usize][x as usize] = k; }

#[inline]
fn bit_for_pos(x:u32,y:u32) -> u128 { 1u128 << ((x*9 + y) + 1) }

#[inline]
fn bb_from_positions(ps:&[(u32,u32)]) -> u128 {
    let mut m: u128 = 0;
    for &(x,y) in ps { m |= bit_for_pos(x,y); }
    m
}

fn blank() -> Banmen { Banmen([[Blank;9];9]) }

// ---------------- GHisha (Gote rook) comprehensive coverage ----------------

#[test]
fn has_control_bits_gote_hisha_unblocked_lines() {
    // Target at (4,4). Place rooks on same file/rank with no blockers.
    let mut b = blank();
    for &(x,y) in &[(4,0),(4,8),(0,4),(8,4)] { set_piece(&mut b, x,y, GHisha); }
    let s = State::new(b);
    let bb = Rule::has_control_bits_gote_hisha(&s, idx(4,4));
    let expect = bb_from_positions(&[(4,0),(4,8),(0,4),(8,4)]);
    assert_eq!(<(u64,u64)>::from(bb), <(u64,u64)>::from(BitBoard::from(expect)), "unblocked rook lines should all control");

    // Friendly piece occupying target still counts as control
    let mut b = blank();
    set_piece(&mut b, 4,4, GKin);
    for &(x,y) in &[(4,0),(4,8),(0,4),(8,4)] { set_piece(&mut b, x,y, GHisha); }
    let s = State::new(b);
    let bb2 = Rule::has_control_bits_gote_hisha(&s, idx(4,4));
    assert_eq!(<(u64,u64)>::from(bb2), <(u64,u64)>::from(BitBoard::from(expect)), "friendly-occupied target must still report attackers");
}

#[test]
fn has_control_bits_gote_hisha_blocked_by_friendly_and_enemy() {
    // Blockers between rook and target should remove control regardless of side
    // Case 1: Friendly blocker (GKin) at (4,2) blocks GHisha at (4,0)
    let mut b = blank();
    set_piece(&mut b, 4,0, GHisha);
    set_piece(&mut b, 4,2, GKin); // blocker between (4,0) and (4,4)
    let s = State::new(b);
    let bb = Rule::has_control_bits_gote_hisha(&s, idx(4,4));
    assert_eq!(<(u64,u64)>::from(bb), (0,0), "friendly blocker should cut rook file control");

    // Case 2: Enemy blocker (SKin) at (2,4) blocks GHisha at (0,4)
    let mut b = blank();
    set_piece(&mut b, 0,4, GHisha);
    set_piece(&mut b, 2,4, SKin); // blocker on rank
    let s = State::new(b);
    let bb = Rule::has_control_bits_gote_hisha(&s, idx(4,4));
    assert_eq!(<(u64,u64)>::from(bb), (0,0), "opponent blocker should cut rook rank control");
}

#[test]
fn has_control_gote_hisha_boolean_matches_bits() {
    // Mix of blocked and unblocked rooks
    let mut b = blank();
    set_piece(&mut b, 4,0, GHisha); // unblocked
    set_piece(&mut b, 0,4, GHisha); // will be blocked by SKin
    set_piece(&mut b, 2,4, SKin);   // blocker
    let s = State::new(b);

    let bb = Rule::has_control_bits_gote_hisha(&s, idx(4,4));
    let bits = <(u64,u64)>::from(bb);
    let bool_has = Rule::has_control_gote_hisha(&s, idx(4,4));
    // bits must be non-zero because (4,0) rook controls, and boolean must be true
    assert_ne!(bits, (0,0));
    assert!(bool_has);
}

// ---------------- GKaku (Gote bishop) comprehensive coverage ----------------

#[test]
fn has_control_bits_gote_kaku_unblocked_diagonals_and_non_diagonals() {
    // Unblocked diagonals to (4,4)
    let mut b = blank();
    for &(x,y) in &[(1,7),(7,7),(1,1),(7,1)] { set_piece(&mut b, x,y, GKaku); }
    // Add also some non-diagonal bishops which should NOT control
    for &(x,y) in &[(4,0),(0,4),(8,4),(4,8)] { set_piece(&mut b, x,y, GKaku); }
    let s = State::new(b);

    let bb = Rule::has_control_bits_gote_kaku(&s, idx(4,4));
    // Expect only the four diagonals to appear
    let expect = bb_from_positions(&[(1,7),(7,7),(1,1),(7,1)]);
    assert_eq!(<(u64,u64)>::from(bb), <(u64,u64)>::from(BitBoard::from(expect)));
}

#[test]
fn has_control_bits_gote_kaku_blocked_by_both_sides() {
    // Place bishop on diagonal but insert blockers near target
    // Friendly blocker
    let mut b = blank();
    set_piece(&mut b, 1,7, GKaku);
    set_piece(&mut b, 3,5, GFu); // between (1,7) and (4,4)
    let s = State::new(b);
    let bb = Rule::has_control_bits_gote_kaku(&s, idx(4,4));
    assert_eq!(<(u64,u64)>::from(bb), (0,0), "friendly blocker should cut bishop diagonal");

    // Opponent blocker
    let mut b = blank();
    set_piece(&mut b, 7,7, GKaku);
    set_piece(&mut b, 5,5, SFu);
    let s = State::new(b);
    let bb = Rule::has_control_bits_gote_kaku(&s, idx(4,4));
    assert_eq!(<(u64,u64)>::from(bb), (0,0), "opponent blocker should cut bishop diagonal");
}

#[test]
fn has_control_gote_kaku_boolean_matches_bits() {
    let mut b = blank();
    set_piece(&mut b, 1,7, GKaku); // unblocked
    set_piece(&mut b, 7,7, GKaku); // will be blocked
    set_piece(&mut b, 5,5, SKin);  // blocker for 7,7
    let s = State::new(b);

    let bb = Rule::has_control_bits_gote_kaku(&s, idx(4,4));
    let bits = <(u64,u64)>::from(bb);
    let bool_has = Rule::has_control_gote_kaku(&s, idx(4,4));
    assert_ne!(bits, (0,0), "at least one bishop should control");
    assert!(bool_has, "boolean must agree with bits when non-zero");
}

// ---------------- GKyou (Gote lance) comprehensive coverage ----------------

#[test]
fn has_control_bits_gote_kyou_forward_direction_only() {
    // For Gote, forward is increasing y. From (4,0) it should reach (4,4). From (4,8) it should not.
    let mut b = blank();
    set_piece(&mut b, 4,0, GKyou); // should control
    set_piece(&mut b, 4,8, GKyou); // should NOT control (behind target)
    set_piece(&mut b, 0,4, GKyou); // same rank -> should NOT control
    let s = State::new(b);

    let bb = Rule::has_control_bits_gote_kyou(&s, idx(4,4));
    let expect = bb_from_positions(&[(4,0)]);
    assert_eq!(<(u64,u64)>::from(bb), <(u64,u64)>::from(BitBoard::from(expect)));
}

#[test]
fn has_control_bits_gote_kyou_blockers_and_friendly_target() {
    // Friendly blocker on file should cut control
    let mut b = blank();
    set_piece(&mut b, 4,0, GKyou);
    set_piece(&mut b, 4,2, GFu); // blocker
    let s = State::new(b);
    let bb = Rule::has_control_bits_gote_kyou(&s, idx(4,4));
    assert_eq!(<(u64,u64)>::from(bb), (0,0));

    // Opponent blocker
    let mut b = blank();
    set_piece(&mut b, 4,0, GKyou);
    set_piece(&mut b, 4,3, SFu); // blocker just before target
    let s = State::new(b);
    let bb = Rule::has_control_bits_gote_kyou(&s, idx(4,4));
    assert_eq!(<(u64,u64)>::from(bb), (0,0));

    // Friendly target occupancy still counts as control when unblocked
    let mut b = blank();
    set_piece(&mut b, 4,4, GKin);
    set_piece(&mut b, 4,0, GKyou);
    let s = State::new(b);
    let bb = Rule::has_control_bits_gote_kyou(&s, idx(4,4));
    let expect = bb_from_positions(&[(4,0)]);
    assert_eq!(<(u64,u64)>::from(bb), <(u64,u64)>::from(BitBoard::from(expect)));
}

#[test]
fn has_control_gote_kyou_boolean_matches_bits() {
    let mut b = blank();
    set_piece(&mut b, 4,0, GKyou);
    let s = State::new(b);
    let bb = Rule::has_control_bits_gote_kyou(&s, idx(4,4));
    let bits = <(u64,u64)>::from(bb);
    assert_ne!(bits, (0,0));
    assert!(Rule::has_control_gote_kyou(&s, idx(4,4)));
}
