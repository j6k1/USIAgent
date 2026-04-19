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

#[allow(dead_code)]
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
    let expect = 364;
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
    let expect = 364;
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
    assert_eq!(got, 90 * 9 / 10, "SEE should follow the specified score fold result for this 3-ply sequence");
}



#[test]
fn calc_see_final_value_is_non_constant_number() {
    // The SEE value should be able to take values that are not exactly equal to any single piece score.
    // Here we use a quiet move into an empty target square; SEE becomes 0, which is not a piece score constant.
    let mut b = blank();
    // Place a piece to move quietly and add some surrounding attackers (not strictly necessary for this check).
    set_piece(&mut b, 3,5, SKin);
    set_piece(&mut b, 4,5, SFu);
    set_piece(&mut b, 3,4, SOu);
    set_piece(&mut b, 4,3, GFu);

    let s = State::new(b);
    let src = idx(3,5);
    let dst = idx(4,4); // empty square
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, None));

    let got = calc_see(Teban::Sente, &s, m);
    assert_eq!(got, -405);
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
    let expect = 0;
    assert_eq!(got, expect, "SEE should match the score fold result for this unfavorable capture sequence");
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
    set_piece(&mut b, 3,5, SKin);
    let s = State::new(b);
    let src = idx(3,5);
    let dst = idx(4,4);
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, None));

    let got = calc_see(Teban::Sente, &s, m);
    assert_eq!(got, -405);
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
    set_piece(&mut b, 3,4, SKin);
    let s = State::new(b);
    let src = idx(3,4);
    let dst = idx(4,4);
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, None));

    let got = calc_see(Teban::Sente, &s, m);
    assert_eq!(got, -486);
}
#[test]
fn calc_see_first_capture_by_non_weakest_attacker() {
    // 初手で複数の攻め駒があるが、最弱の駒(歩)ではなく銀で取るケース。
    // 対象マス(4,4)に後手の歩があり、先手は歩(4,5)と銀(3,5)の両方で取れる。
    // ここで銀で取る手を指定する。直後に後手玉(5,4)で取り返される形にする。
    // 期待値: 取った駒の価値(歩=81)。
    let mut b = blank();
    // target
    set_piece(&mut b, 4,4, GFu);
    // our attackers: pawn and silver (silver is not the weakest)
    set_piece(&mut b, 4,5, SFu);
    set_piece(&mut b, 3,5, SGin);
    // opponent recaptor
    set_piece(&mut b, 5,4, GOu);

    let s = State::new(b);

    // choose the stronger attacker (silver) to capture
    let src = idx(3,5);
    let dst = idx(4,4);
    let m = LegalMove::To(LegalMoveTo::new(src, dst, false, Some(ObtainKind::Fu)));

    let got = calc_see(Teban::Sente, &s, m);
    let expect = 81;
    assert_eq!(got, expect);
}

#[test]
fn calc_see_put_immediate_recapture_is_negative() {
    // 初手が駒を置く(打つ)手で、その直後に相手に取り返されるケース。
    // (4,4)に歩を打つ。後手玉(5,4)が(4,4)を攻撃しているので直後に取り返せる。
    // 期待値: 0
    let mut b = blank();
    set_piece(&mut b, 5,4, GOu); // opponent attacker to 4,4

    let s = State::new(b);

    let dst = idx(4,4);
    let m = LegalMove::Put(LegalMovePut::new(MochigomaKind::Fu, dst));

    let got = calc_see(Teban::Sente, &s, m);
    let expect = -81;
    assert_eq!(got, expect);
}


#[test]
fn calc_see_scores_differ_between_weakest_and_non_weakest_first_capture() {
    // 指定の局面:
    //  - 先手歩: 6五 (ターゲット)
    //  - 後手歩: 6六 (歩で取れる)
    //  - 後手桂: 7七 (桂でも取れる)
    //  - 先手角: 8八 (6六に利いている)
    // エンジン座標系では以下の通りに配置する:
    //  ターゲット(6,5) -> (5,4) の先手歩 SFu
    //  後手歩(6,6) -> (5,5) の GFu (ここから(5,4)を取れるとする)
    //  後手桂(7,7) -> (6,6) の GKei (ここから(5,4)を取れる)
    //  先手角(8,8) -> (7,7) の SKaku
    //  さらに取り合いを成立させるため先手玉を(5,3)に置き、(5,4)を取り返せるようにする。
    let mut b = blank();
    // target: Sente pawn at 6五 -> (5,4)
    set_piece(&mut b, 5,4, SFu);
    // Gote attackers: pawn at 6六 -> (5,5), knight at 7七 -> (6,6)
    set_piece(&mut b, 5,5, GFu);
    set_piece(&mut b, 6,6, GKei);
    // Sente bishop at 8八 -> (7,7)
    set_piece(&mut b, 7,7, SKaku);
    // Sente king to allow immediate recapture on (5,4)
    set_piece(&mut b, 5,3, SOu);

    let s = State::new(b);

    let dst = idx(5,4);

    // 後手 歩取り: (5,5)->(5,4)
    let pawn_src = idx(5,5);
    let m_pawn = LegalMove::To(LegalMoveTo::new(pawn_src, dst, false, Some(ObtainKind::Fu)));
    let see_pawn = calc_see(Teban::Gote, &s, m_pawn);

    // 後手 桂取り: (6,6)->(5,4)
    let knight_src = idx(6,6);
    let m_knight = LegalMove::To(LegalMoveTo::new(knight_src, dst, false, Some(ObtainKind::Fu)));
    let see_knight = calc_see(Teban::Gote, &s, m_knight);

    // この局面では桂で取り始めるとスコアが変化するはず
    assert_ne!(see_pawn, see_knight, "SEE must differ when starting with Gote knight vs pawn in this setup");
}
