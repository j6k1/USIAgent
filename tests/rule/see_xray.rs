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
fn see_xray_rook_value() {
    let mut b = blank();
    set_piece(&mut b, 4, 4, GGin);
    set_piece(&mut b, 4, 5, SFu);
    set_piece(&mut b, 4, 3, GFu);
    set_piece(&mut b, 5, 4, SKin);
    set_piece(&mut b, 4, 0, GHisha);

    let s = State::new(b);
    let m = LegalMove::To(LegalMoveTo::new(idx(4, 5), idx(4, 4), false, Some(ObtainKind::Gin)));
    let got = calc_see(Teban::Sente, &s, m);

    assert_eq!(got, 364);
}

#[test]
fn see_xray_bishop_value() {
    let mut b = blank();
    set_piece(&mut b, 4, 4, GGin);
    set_piece(&mut b, 4, 5, SFu);
    set_piece(&mut b, 3, 3, GGin);
    set_piece(&mut b, 5, 5, SGin);
    set_piece(&mut b, 1, 1, GKaku);

    let s = State::new(b);
    let m = LegalMove::To(LegalMoveTo::new(idx(4, 5), idx(4, 4), false, Some(ObtainKind::Gin)));
    let got = calc_see(Teban::Sente, &s, m);

    assert_eq!(got, 364);
}

#[test]
fn see_xray_lance_value() {
    let mut b = blank();
    set_piece(&mut b, 4, 4, GGin);
    set_piece(&mut b, 4, 5, SFu);
    set_piece(&mut b, 4, 3, GFu);
    set_piece(&mut b, 5, 4, SKin);
    set_piece(&mut b, 4, 0, GKyou);

    let s = State::new(b);
    let m = LegalMove::To(LegalMoveTo::new(idx(4, 5), idx(4, 4), false, Some(ObtainKind::Gin)));
    let got = calc_see(Teban::Sente, &s, m);

    assert_eq!(got, 364);
}

#[test]
fn see_xray_rook_value_mirrored() {
    let mut b = blank();
    set_piece(&mut b, 4, 4, SGin);
    set_piece(&mut b, 4, 3, GFu);
    set_piece(&mut b, 4, 5, SFu);
    set_piece(&mut b, 5, 4, GKin);
    set_piece(&mut b, 4, 8, SHisha);

    let s = State::new(b);
    let m = LegalMove::To(LegalMoveTo::new(idx(4, 3), idx(4, 4), false, Some(ObtainKind::Gin)));
    let got = calc_see(Teban::Gote, &s, m);

    assert_eq!(got, 364);
}

#[test]
fn see_xray_bishop_value_mirrored() {
    let mut b = blank();
    set_piece(&mut b, 4, 4, SGin);
    set_piece(&mut b, 4, 3, GFu);
    set_piece(&mut b, 3, 5, SGin);
    set_piece(&mut b, 5, 3, GKin);
    set_piece(&mut b, 7, 7, SKaku);

    let s = State::new(b);
    let m = LegalMove::To(LegalMoveTo::new(idx(4, 3), idx(4, 4), false, Some(ObtainKind::Gin)));
    let got = calc_see(Teban::Gote, &s, m);

    assert_eq!(got, 364);
}

#[test]
fn see_xray_lance_value_mirrored() {
    let mut b = blank();
    set_piece(&mut b, 4, 4, SGin);
    set_piece(&mut b, 4, 3, GFu);
    set_piece(&mut b, 4, 5, SFu);
    set_piece(&mut b, 5, 4, GKin);
    set_piece(&mut b, 4, 8, SKyou);

    let s = State::new(b);
    let m = LegalMove::To(LegalMoveTo::new(idx(4, 3), idx(4, 4), false, Some(ObtainKind::Gin)));
    let got = calc_see(Teban::Gote, &s, m);

    assert_eq!(got, 364);
}
