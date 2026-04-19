use usiagent::see::calc_see;
use usiagent::rule::{LegalMove, LegalMoveTo, State};
use usiagent::shogi::*;
use usiagent::shogi::KomaKind::*;

#[inline]
fn idx(x: u32, y: u32) -> u32 { x * 9 + y }

#[inline]
fn set_piece(b: &mut Banmen, x: u32, y: u32, k: KomaKind) { b.0[y as usize][x as usize] = k; }

#[inline]
fn blank() -> Banmen { Banmen([[Blank; 9]; 9]) }

#[test]
fn see_xray_rook_changes_result() {
    let mut a = blank();
    set_piece(&mut a, 4, 4, GGin);
    set_piece(&mut a, 4, 5, SFu);
    set_piece(&mut a, 4, 3, GFu);
    set_piece(&mut a, 5, 4, SKin);
    let s_a = State::new(a);
    let m = LegalMove::To(LegalMoveTo::new(idx(4, 5), idx(4, 4), false, Some(ObtainKind::Gin)));
    let v_a = calc_see(Teban::Sente, &s_a, m);

    let mut b = blank();
    set_piece(&mut b, 4, 4, GGin);
    set_piece(&mut b, 4, 5, SFu);
    set_piece(&mut b, 4, 3, GFu);
    set_piece(&mut b, 5, 4, SKin);
    set_piece(&mut b, 4, 0, GHisha);
    let s_b = State::new(b);
    let v_b = calc_see(Teban::Sente, &s_b, m);

    assert_ne!(v_a, v_b);
}

#[test]
fn see_xray_bishop_changes_result() {
    let mut a = blank();
    set_piece(&mut a, 4, 4, GGin);
    set_piece(&mut a, 4, 5, SFu);
    set_piece(&mut a, 3, 3, GGin);
    set_piece(&mut a, 5, 5, SGin);
    let s_a = State::new(a);
    let m = LegalMove::To(LegalMoveTo::new(idx(4, 5), idx(4, 4), false, Some(ObtainKind::Gin)));
    let v_a = calc_see(Teban::Sente, &s_a, m);

    let mut b = blank();
    set_piece(&mut b, 4, 4, GGin);
    set_piece(&mut b, 4, 5, SFu);
    set_piece(&mut b, 3, 3, GGin);
    set_piece(&mut b, 5, 5, SGin);
    set_piece(&mut b, 1, 1, GKaku);
    let s_b = State::new(b);
    let v_b = calc_see(Teban::Sente, &s_b, m);

    assert_ne!(v_a, v_b);
}

#[test]
fn see_xray_lance_changes_result() {
    let mut a = blank();
    set_piece(&mut a, 4, 4, GGin);
    set_piece(&mut a, 4, 5, SFu);
    set_piece(&mut a, 4, 3, GFu);
    set_piece(&mut a, 5, 4, SKin);
    let s_a = State::new(a);
    let m = LegalMove::To(LegalMoveTo::new(idx(4, 5), idx(4, 4), false, Some(ObtainKind::Gin)));
    let v_a = calc_see(Teban::Sente, &s_a, m);

    let mut b = blank();
    set_piece(&mut b, 4, 4, GGin);
    set_piece(&mut b, 4, 5, SFu);
    set_piece(&mut b, 4, 3, GFu);
    set_piece(&mut b, 5, 4, SKin);
    set_piece(&mut b, 4, 0, GKyou);
    let s_b = State::new(b);
    let v_b = calc_see(Teban::Sente, &s_b, m);

    assert_ne!(v_a, v_b);
}

#[test]
fn see_xray_rook_changes_result_mirrored() {
    let mut a = blank();
    set_piece(&mut a, 4, 4, SGin);
    set_piece(&mut a, 4, 3, GFu);
    set_piece(&mut a, 4, 5, SFu);
    set_piece(&mut a, 5, 4, GKin);
    let s_a = State::new(a);
    let m = LegalMove::To(LegalMoveTo::new(idx(4, 3), idx(4, 4), false, Some(ObtainKind::Gin)));
    let v_a = calc_see(Teban::Gote, &s_a, m);

    let mut b = blank();
    set_piece(&mut b, 4, 4, SGin);
    set_piece(&mut b, 4, 3, GFu);
    set_piece(&mut b, 4, 5, SFu);
    set_piece(&mut b, 5, 4, GKin);
    set_piece(&mut b, 4, 8, SHisha);
    let s_b = State::new(b);
    let v_b = calc_see(Teban::Gote, &s_b, m);

    assert_ne!(v_a, v_b);
}

#[test]
fn see_xray_bishop_changes_result_mirrored() {
    let mut a = blank();
    set_piece(&mut a, 4, 4, SGin);
    set_piece(&mut a, 4, 3, GFu);
    set_piece(&mut a, 3, 5, SGin);
    set_piece(&mut a, 5, 3, GKin);
    let s_a = State::new(a);
    let m = LegalMove::To(LegalMoveTo::new(idx(4, 3), idx(4, 4), false, Some(ObtainKind::Gin)));
    let v_a = calc_see(Teban::Gote, &s_a, m);

    let mut b = blank();
    set_piece(&mut b, 4, 4, SGin);
    set_piece(&mut b, 4, 3, GFu);
    set_piece(&mut b, 3, 5, SGin);
    set_piece(&mut b, 5, 3, GKin);
    set_piece(&mut b, 7, 7, SKaku);
    let s_b = State::new(b);
    let v_b = calc_see(Teban::Gote, &s_b, m);

    assert_ne!(v_a, v_b);
}

#[test]
fn see_xray_lance_changes_result_mirrored() {
    let mut a = blank();
    set_piece(&mut a, 4, 4, SGin);
    set_piece(&mut a, 4, 3, GFu);
    set_piece(&mut a, 4, 5, SFu);
    set_piece(&mut a, 5, 4, GKin);
    let s_a = State::new(a);
    let m = LegalMove::To(LegalMoveTo::new(idx(4, 3), idx(4, 4), false, Some(ObtainKind::Gin)));
    let v_a = calc_see(Teban::Gote, &s_a, m);

    let mut b = blank();
    set_piece(&mut b, 4, 4, SGin);
    set_piece(&mut b, 4, 3, GFu);
    set_piece(&mut b, 4, 5, SFu);
    set_piece(&mut b, 5, 4, GKin);
    set_piece(&mut b, 4, 8, SKyou);
    let s_b = State::new(b);
    let v_b = calc_see(Teban::Gote, &s_b, m);

    assert_ne!(v_a, v_b);
}

#[test]
fn see_xray_line_opens_from_gote_capture() {
    let mut a = blank();
    set_piece(&mut a, 5, 5, SFu);
    set_piece(&mut a, 5, 4, GFu);
    set_piece(&mut a, 4, 3, GGin);
    let s_a = State::new(a);
    let m = LegalMove::To(LegalMoveTo::new(idx(5, 4), idx(5, 5), false, Some(ObtainKind::Fu)));
    let v_a = calc_see(Teban::Gote, &s_a, m);

    let mut b = blank();
    set_piece(&mut b, 6, 6, SKaku);
    set_piece(&mut b, 5, 5, SFu);
    set_piece(&mut b, 5, 4, GFu);
    set_piece(&mut b, 4, 3, GGin);
    let s_b = State::new(b);
    let v_b = calc_see(Teban::Gote, &s_b, m);

    assert_ne!(v_a, v_b);
}

#[test]
fn see_xray_line_opens_from_sente_capture_mirrored() {
    let mut a = blank();
    set_piece(&mut a, 5, 3, GFu);
    set_piece(&mut a, 5, 4, SFu);
    set_piece(&mut a, 4, 5, SGin);
    let s_a = State::new(a);
    let m = LegalMove::To(LegalMoveTo::new(idx(5, 4), idx(5, 3), false, Some(ObtainKind::Fu)));
    let v_a = calc_see(Teban::Sente, &s_a, m);

    let mut b = blank();
    set_piece(&mut b, 6, 2, GKaku);
    set_piece(&mut b, 5, 3, GFu);
    set_piece(&mut b, 5, 4, SFu);
    set_piece(&mut b, 4, 5, SGin);
    let s_b = State::new(b);
    let v_b = calc_see(Teban::Sente, &s_b, m);

    assert_ne!(v_a, v_b);
}