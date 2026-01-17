use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

use usiagent::see::calc_see;
use usiagent::rule::{LegalMove, LegalMoveTo, State};
use usiagent::shogi::*;
use usiagent::shogi::KomaKind::*;

#[inline]
fn idx(x:u32,y:u32) -> u32 { x*9 + y }

#[inline]
fn set_piece(b:&mut Banmen, x:u32, y:u32, k:KomaKind) { b.0[y as usize][x as usize] = k; }

#[inline]
fn blank() -> Banmen { Banmen([[Blank;9];9]) }

fn run_with_timeout<F>(f: F, timeout_ms: u64)
where
    F: FnOnce(Sender<()>) + Send + 'static,
{
    let (tx, rx) = channel();
    thread::spawn(move || {
        f(tx);
    });
    match rx.recv_timeout(Duration::from_millis(timeout_ms)) {
        Ok(_) => { /* finished in time */ }
        Err(_) => panic!("calc_see did not return within {} ms (possible infinite loop)", timeout_ms),
    }
}

#[test]
fn see_should_not_infinite_loop_when_xray_rook_rejoins_after_pull_occupied() {
    // Scenario revised per requirement:
    // 1) Sente rook (slider) captures on (4,4) from (4,8) taking a Gote silver.
    // 2) Gote rook (slider) on (4,0) can immediately recapture on (4,4).
    // This ensures both the initial capture and the opponent recapture are by sliders.
    let mut b = blank();
    set_piece(&mut b, 4,4, GGin);    // target piece to be captured
    set_piece(&mut b, 4,8, SHisha);  // Sente rook will capture to (4,4)
    set_piece(&mut b, 4,0, GHisha);  // Gote rook can recapture to (4,4)

    let s = State::new(b);
    // Sente rook moves from (4,8) to (4,4), capturing GGin
    let m = LegalMove::To(LegalMoveTo::new(idx(4,8), idx(4,4), false, Some(ObtainKind::Gin)));

    run_with_timeout(move |done| {
        let _ = calc_see(Teban::Sente, &s, m);
        let _ = done.send(());
    }, 1500);
}

#[test]
fn see_should_not_infinite_loop_when_xray_bishop_rejoins_after_pull_occupied() {
    // Scenario revised per requirement:
    // 1) Sente bishop (slider) captures on (4,4) from (1,1), taking a Gote silver.
    // 2) Gote bishop (slider) on (7,7) can immediately recapture on (4,4).
    // Both initial capture and opponent recapture are by bishops.
    let mut b = blank();
    set_piece(&mut b, 4,4, GGin);    // target piece
    set_piece(&mut b, 1,1, SKaku);   // Sente bishop captures along diagonal to (4,4)
    set_piece(&mut b, 7,7, GKaku);   // Gote bishop can recapture along opposite diagonal

    let s = State::new(b);
    let m = LegalMove::To(LegalMoveTo::new(idx(1,1), idx(4,4), false, Some(ObtainKind::Gin)));

    run_with_timeout(move |done| {
        let _ = calc_see(Teban::Sente, &s, m);
        let _ = done.send(());
    }, 1500);
}

#[test]
fn see_should_not_infinite_loop_when_xray_lance_rejoins_after_pull_occupied() {
    // Scenario revised per requirement:
    // 1) Sente lance (slider) captures on (4,4) from (4,8), taking a Gote silver.
    // 2) Gote lance (slider) on (4,0) can immediately recapture on (4,4).
    let mut b = blank();
    set_piece(&mut b, 4,4, GGin);   // target piece
    set_piece(&mut b, 4,8, SKyou);  // Sente lance captures downward to (4,4)
    set_piece(&mut b, 4,0, GKyou);  // Gote lance can recapture upward to (4,4)

    let s = State::new(b);
    let m = LegalMove::To(LegalMoveTo::new(idx(4,8), idx(4,4), false, Some(ObtainKind::Gin)));

    run_with_timeout(move |done| {
        let _ = calc_see(Teban::Sente, &s, m);
        let _ = done.send(());
    }, 1500);
}
