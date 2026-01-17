use usiagent::see::calc_see;
use usiagent::rule::{LegalMove, State};
use usiagent::rule::{LegalMoveTo, LegalMovePut};
use usiagent::shogi::*;
use usiagent::shogi::KomaKind::*;

#[inline]
fn idx(x:u32,y:u32) -> u32 { x*9 + y }

#[inline]
fn set_piece(b:&mut Banmen, x:u32, y:u32, k:KomaKind) { b.0[y as usize][x as usize] = k; }

#[inline]
fn blank() -> Banmen { Banmen([[Blank;9];9]) }

fn piece_scores() -> [i32; 13] {
    // The piece score constants used inside see.rs (scaled by 9/10)
    [
        90 * 9 / 10,     // FU
        315 * 9 / 10,    // KYOU
        405 * 9 / 10,    // KEI
        495 * 9 / 10,    // GIN
        540 * 9 / 10,    // KIN / NARI-KIN
        855 * 9 / 10,    // KAKU
        990 * 9 / 10,    // HISHA
        945 * 9 / 10,    // KAKU_NARI
        1395 * 9 / 10,   // HISHA_NARI
        15000 * 9 / 10,  // OU
        // Duplicates for convenience
        540 * 9 / 10,
        540 * 9 / 10,
        540 * 9 / 10,
    ]
}

#[test]
fn calc_see_capture_no_opponent_attackers_sente() {
    // Sente pawn captures Gote silver on a square not attacked by Gote
    // With the revised SEE (return current_score when no recapture), SEE equals the value of the captured piece (silver).
    let mut b = blank();
    // target at (4,4) contains Gote silver
    set_piece(&mut b, 4,4, GGin);
    // Sente pawn ready to capture from (4,5)
    set_piece(&mut b, 4,5, SFu);

    let s = State::new(b);

    let src = idx(4,5);
    let dst = idx(4,4);
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, Some(ObtainKind::Gin)));

    let got = calc_see(Teban::Sente, &s, m);
    assert_eq!(got, 495 * 9 / 10);
}

#[test]
fn calc_see_capture_no_opponent_attackers_gote() {
    // Gote pawn captures Sente pawn on a square not attacked by Sente
    // With the revised SEE (return current_score when no recapture), SEE equals the value of the captured piece (pawn = 90*9/10).
    let mut b = blank();
    // target at (4,4) contains Sente pawn
    set_piece(&mut b, 4,4, SFu);
    // Gote pawn ready to capture from (4,3)
    set_piece(&mut b, 4,3, GFu);

    let s = State::new(b);

    let src = idx(4,3);
    let dst = idx(4,4);
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, Some(ObtainKind::Fu)));

    let got = calc_see(Teban::Gote, &s, m);
    assert_eq!(got, 90 * 9 / 10);
}

#[test]
fn calc_see_non_capture_returns_zero() {
    // Move to empty square; SEE should be 0
    let mut b = blank();
    set_piece(&mut b, 0,0, SKin);
    // move to (0,1) which is empty
    let s = State::new(b);

    let src = idx(0,0);
    let dst = idx(0,1);
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, None));

    let got = calc_see(Teban::Sente, &s, m);
    assert_eq!(got, 0);
}

#[test]
fn calc_see_put_on_safe_square_is_zero() {
    // Drop a pawn on an empty and unattacked square; SEE should be 0
    let b = blank();
    let s = State::new(b);

    let dst = idx(4,4);
    let m = LegalMove::Put(LegalMovePut::new(MochigomaKind::Fu, dst));

    let got = calc_see(Teban::Sente, &s, m);
    assert_eq!(got, 0);
}

#[test]
fn calc_see_capture_with_one_opponent_attacker_min_fold_sente() {
    // Sente captures a silver on 4,4, but Gote king also attacks 4,4.
    // With one opponent attacker (a single recapture), the SEE result should be
    // captured pawn value minus captured silver value (i.e., pawn - silver), per spec.
    let mut b = blank();
    // target contains Gote silver
    set_piece(&mut b, 4,4, GGin);
    // Sente pawn that captures
    set_piece(&mut b, 4,5, SFu);
    // Gote king adjacent, attacking the target
    set_piece(&mut b, 5,4, GOu);

    let s = State::new(b);

    let src = idx(4,5);
    let dst = idx(4,4);
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, Some(ObtainKind::Gin)));

    let got = calc_see(Teban::Sente, &s, m);
    let expect = 90 * 9 / 10 - 495 * 9 / 10; // pawn - silver
    assert_eq!(got, expect);
}

#[test]
fn calc_see_capture_with_one_opponent_attacker_min_fold_gote() {
    // Gote captures a silver on 4,4, Sente king also attacks 4,4.
    // With one opponent attacker (a single recapture), the SEE result should be
    // captured pawn value minus captured silver value (pawn - silver), per spec.
    let mut b = blank();
    // target contains Sente silver
    set_piece(&mut b, 4,4, SGin);
    // Gote pawn that captures from (4,3)
    set_piece(&mut b, 4,3, GFu);
    // Sente king adjacent, attacking the target
    set_piece(&mut b, 3,4, SOu);

    let s = State::new(b);

    let src = idx(4,3);
    let dst = idx(4,4);
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, Some(ObtainKind::Gin)));

    let got = calc_see(Teban::Gote, &s, m);
    let expect = 90 * 9 / 10 - 495 * 9 / 10; // pawn - silver
    assert_eq!(got, expect);
}


#[test]
fn calc_see_three_ply_exchange_returns_zero() {
    // Three-ply capture chain validating new gain accumulation rule:
    // gain[0] = captured(GFu)=81
    // gain[1] = -(captured(Gote recaptures our pawn)=81 - gain[0]=81) = 0
    // gain[2] = -(captured(we recapture Gote pawn)=81 + gain[1]=0) = -81
    // Backprop: gain[1]=min(0,81)=0; gain[0]=min(81,0)=0 => final SEE = 0
    let mut b = blank();
    set_piece(&mut b, 4,4, GFu); // initial target piece
    set_piece(&mut b, 4,5, SFu); // Sente pawn captures first
    set_piece(&mut b, 4,3, GFu); // Gote pawn recaptures
    set_piece(&mut b, 3,4, SOu); // Sente king recaptures

    let s = State::new(b);
    let src = idx(4,5);
    let dst = idx(4,4);
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, Some(ObtainKind::Fu)));

    let got = calc_see(Teban::Sente, &s, m);
    assert_eq!(got, 0, "SEE should be 0 in a symmetric three-ply pawn exchange chain");
}



#[test]
fn calc_see_final_value_is_non_constant_number() {
    // The SEE value should be able to take values that are not exactly equal to any single piece score.
    // Here we use a quiet move into an empty target square; SEE becomes 0, which is not a piece score constant.
    let mut b = blank();
    // Place a piece to move quietly and add some surrounding attackers (not strictly necessary for this check).
    set_piece(&mut b, 0,0, SKin);
    set_piece(&mut b, 4,5, SFu);
    set_piece(&mut b, 3,4, SOu);
    set_piece(&mut b, 4,3, GFu);

    let s = State::new(b);
    let src = idx(0,0);
    let dst = idx(4,4); // empty square
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, None));

    let got = calc_see(Teban::Sente, &s, m);
    let pcs = piece_scores();
    assert_eq!(got, 0, "SEE for quiet move to empty square should be exactly 0");
    assert!(!pcs.iter().any(|&v| v == got), "SEE should not equal any single piece score, got {}", got);
}

#[test]
fn calc_see_unfavorable_capture_returns_captured_pawn_score() {
    // Build an exchange that should be losing for the side to move.
    // According to the revised SEE spec and clarification, the net SEE should be 0 here.
    let mut b = blank();
    set_piece(&mut b, 4,4, GFu);  // target pawn
    set_piece(&mut b, 4,5, SFu);  // Sente pawn captures first
    // Opponent strong counter-attackers
    set_piece(&mut b, 3,5, GGin); // Gote silver
    set_piece(&mut b, 5,4, GOu);  // Gote king
    // Few Sente defenders
    set_piece(&mut b, 3,4, SOu);  // Sente king only

    let s = State::new(b);
    let src = idx(4,5);
    let dst = idx(4,4);
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, Some(ObtainKind::Fu)));

    let got = calc_see(Teban::Sente, &s, m);
    let expect = 0; // Unfavorable capture sequence should evaluate to 0 in this case
    assert_eq!(got, expect, "SEE should be 0 for this unfavorable capture sequence");
}

#[test]
fn calc_see_quiet_move_can_be_positive() {
    // Quiet move into a square with more of our attackers than opponent's -> should be favorable.
    // Target (4,4) is empty.
    let mut b = blank();
    // Our attackers
    set_piece(&mut b, 4,5, SFu);  // attacks 4,4
    set_piece(&mut b, 3,4, SOu);  // attacks 4,4
    set_piece(&mut b, 5,3, SGin); // attacks 4,4
    // Opponent attackers (fewer)
    set_piece(&mut b, 4,3, GFu);  // attacks 4,4

    // Make any quiet move to 4,4 (source square is irrelevant for SEE other than dst/obtain)
    set_piece(&mut b, 0,0, SKin);
    let s = State::new(b);
    let src = idx(0,0);
    let dst = idx(4,4);
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, None));

    let got = calc_see(Teban::Sente, &s, m);
    assert_eq!(got, 0, "SEE for favorable quiet move (empty target) should be exactly 0");
}

#[test]
fn calc_see_quiet_move_to_empty_square_is_zero() {
    // Quiet move into a square dominated by opponent attacks -> should be unfavorable (negative).
    let mut b = blank();
    // Opponent attackers (many)
    set_piece(&mut b, 4,3, GFu);  // attacks 4,4
    set_piece(&mut b, 3,5, GGin); // attacks 4,4
    set_piece(&mut b, 5,4, GOu);  // attacks 4,4
    // Our attackers (few)
    set_piece(&mut b, 4,5, SFu);  // attacks 4,4

    // Place a moving piece to perform a quiet move
    set_piece(&mut b, 0,0, SKin);
    let s = State::new(b);
    let src = idx(0,0);
    let dst = idx(4,4);
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, None));

    let got = calc_see(Teban::Sente, &s, m);
    assert_eq!(got, 0, "SEE for unfavorable quiet move (empty target) should be exactly 0");
}



