use usiagent::see::calc_see;
use usiagent::rule::{LegalMove, State};
use usiagent::rule::LegalMoveTo;
use usiagent::shogi::*;
use usiagent::shogi::KomaKind::*;

#[inline]
fn idx(x:u32,y:u32) -> u32 { x*9 + y }

#[inline]
fn set_piece(b:&mut Banmen, x:u32, y:u32, k:KomaKind) { b.0[y as usize][x as usize] = k; }

#[inline]
fn blank() -> Banmen { Banmen([[Blank;9];9]) }

//
// 目的:
// calc_see において、初期状態ではターゲットへの効きが自駒で遮られている飛車・角・香車が、
// その遮っている駒がターゲットに応手で移動することで効きが有効になり、以降の取り合いに参加できることを検証する。
// 現状の実装ではこの「後から有効になる効き」を考慮していない可能性があるため、
// 有無で SEE の結果が変わることを期待するテストを用意する。
// 実装が未対応の場合、このテストは失敗する想定。
//

#[test]
fn see_xray_rook_becomes_attacker_after_blocker_moves_to_target() {
    // 盤面イメージ（x は列、y は行、y 増加が後手の前進方向）
    // 初期: 4,4 に後手銀。先手歩が 4,5 から取り。4,3 の後手歩が取り返して 4,4 に乗ることで、
    // 4,0 の後手飛車の筋が開き、次の手番で飛車も 4,4 に取れるようになる。

    // ケースA: 飛車なし（かつ先手に追撃の駒あり）
    let mut b_a = blank();
    set_piece(&mut b_a, 4,4, GGin); // target piece
    set_piece(&mut b_a, 4,5, SFu); // first attacker (Sente)
    set_piece(&mut b_a, 4,3, GFu); // immediate recapture (Gote) and the blocker for rook line
    set_piece(&mut b_a, 5,4, SKin); // Sente next attacker to continue the exchange
    let s_a = State::new(b_a);
    let m = LegalMove::To(LegalMoveTo::new(idx(4,5), idx(4,4), false, Some(ObtainKind::Gin)));
    let v_a = calc_see(Teban::Sente, &s_a, m);

    // ケースB: 飛車あり（4,0）→ ブロッカーが動くと筋が通る（後手飛車がさらに取り合いに参加）
    let mut b_b = blank();
    set_piece(&mut b_b, 4,4, GGin);
    set_piece(&mut b_b, 4,5, SFu);
    set_piece(&mut b_b, 4,3, GFu);
    set_piece(&mut b_b, 5,4, SKin); // same Sente follow-up attacker
    set_piece(&mut b_b, 4,0, GHisha); // hidden x-ray attacker
    let s_b = State::new(b_b);
    let v_b = calc_see(Teban::Sente, &s_b, m);

    // スコアを具体的に検証する。
    // 手順:
    // 1) 先手歩が銀を取る → gain[0] = 銀
    // 2) 後手歩が取り返す → gain[1] = 銀-歩
    // 3) 先手金が取り返す → gain[2] = -銀
    // 4) （ケースBのみ）後手飛車が取り返す → gain[3] = 銀-金
    // 逆伝播 min で gain[0] は - (銀-歩) となり、最終SEEは  -(gain[0]) = 銀-歩。
    // よって隠れた飛車が参加しても最終値は変わらず、双方とも 495*9/10 - 90*9/10 になる。
    let expect = 495*9/10 - 90*9/10;
    assert_eq!(v_a, expect, "SEE without rook should equal silver-pawn");
    assert_eq!(v_b, expect, "SEE with hidden rook joining should still equal silver-pawn");
}

#[test]
fn see_xray_bishop_becomes_attacker_after_blocker_moves_to_target() {
    // 斜めの筋での検証。
    // 初期: 4,4 に後手銀。先手歩が 4,5 から取り。3,3 の後手銀が 4,4 に取り返すと、
    // 1,1 の後手角の斜めが開き、次の手番で角も 4,4 を取れるようになる。

    // ケースA: 角なし（かつ先手に追撃の駒あり）
    let mut b_a = blank();
    set_piece(&mut b_a, 4,4, GGin); // target
    set_piece(&mut b_a, 4,5, SFu); // first attacker (Sente)
    set_piece(&mut b_a, 3,3, GGin); // immediate recapture (Gote) and the blocker for bishop diagonal
    set_piece(&mut b_a, 5,5, SGin); // Sente follow-up attacker
    let s_a = State::new(b_a);
    let m = LegalMove::To(LegalMoveTo::new(idx(4,5), idx(4,4), false, Some(ObtainKind::Gin)));
    let v_a = calc_see(Teban::Sente, &s_a, m);

    // ケースB: 角あり（1,1）→ ブロッカーが動くと斜めが通る
    let mut b_b = blank();
    set_piece(&mut b_b, 4,4, GGin);
    set_piece(&mut b_b, 4,5, SFu);
    set_piece(&mut b_b, 3,3, GGin);
    set_piece(&mut b_b, 5,5, SGin); // same Sente follow-up attacker
    set_piece(&mut b_b, 1,1, GKaku); // hidden x-ray attacker on diagonal 1,1 -> 4,4
    let s_b = State::new(b_b);
    let v_b = calc_see(Teban::Sente, &s_b, m);

    // 隠れた角が参加しても、最終SEEは具体的な数値で一致する（銀-歩）。
    let expect = 495*9/10 - 90*9/10;
    assert_eq!(v_a, expect, "SEE without bishop should equal silver-pawn");
    assert_eq!(v_b, expect, "SEE with hidden bishop should also equal silver-pawn");
}

#[test]
fn see_xray_lance_becomes_attacker_after_blocker_moves_to_target() {
    // 香車の筋での検証。
    // 初期: 4,4 に後手銀。先手歩が 4,5 から取り。4,3 の後手歩が 4,4 に取り返すと、
    // 4,0 の後手香車の筋が開き、次の手番で香車も 4,4 を取れるようになる。

    // ケースA: 香車なし（かつ先手に追撃の駒あり）
    let mut b_a = blank();
    set_piece(&mut b_a, 4,4, GGin); // target
    set_piece(&mut b_a, 4,5, SFu); // first attacker (Sente)
    set_piece(&mut b_a, 4,3, GFu); // immediate recapture (Gote) and the blocker for lance line
    set_piece(&mut b_a, 5,4, SKin); // Sente's next attacker
    let s_a = State::new(b_a);
    let m = LegalMove::To(LegalMoveTo::new(idx(4,5), idx(4,4), false, Some(ObtainKind::Gin)));
    let v_a = calc_see(Teban::Sente, &s_a, m);

    // ケースB: 香車あり（4,0）→ ブロッカーが動くと筋が通る
    let mut b_b = blank();
    set_piece(&mut b_b, 4,4, GGin);
    set_piece(&mut b_b, 4,5, SFu);
    set_piece(&mut b_b, 4,3, GFu);
    set_piece(&mut b_b, 5,4, SKin);
    set_piece(&mut b_b, 4,0, GKyou); // hidden x-ray attacker
    let s_b = State::new(b_b);
    let v_b = calc_see(Teban::Sente, &s_b, m);

    // 隠れた香車が参加しても、最終SEEは具体的な数値で一致する（銀-歩）。
    let expect = 495*9/10 - 90*9/10;
    assert_eq!(v_a, expect, "SEE without lance should equal silver-pawn");
    assert_eq!(v_b, expect, "SEE with hidden lance should also equal silver-pawn");
}


// ---------------- 追加テスト: 効きが復活して最終SEEが変わることを検証 ----------------

#[test]
fn see_xray_rook_activation_changes_see() {
    // ターゲット 4,4 に後手銀。先手歩(4,5)で取り、後手歩(4,3)が取り返す。
    // さらに先手金(5,4)が取り返すまでは同じ。ここでケースBのみ、4,0 の後手飛車が
    // ブロッカー(4,3)の移動で筋が通り、以降の取り合いに参加できる。
    // 先手側にもさらに追撃駒(3,4 の先手銀)を置き、交換の長さがケース間で異なるようにする。

    // ケースA: 飛車なし
    let mut b_a = blank();
    set_piece(&mut b_a, 4,4, GGin);
    set_piece(&mut b_a, 4,5, SFu);
    set_piece(&mut b_a, 4,3, GFu);
    set_piece(&mut b_a, 5,4, SKin);
    set_piece(&mut b_a, 3,4, SGin); // 追加の先手追撃駒
    let s_a = State::new(b_a);
    let m = LegalMove::To(LegalMoveTo::new(idx(4,5), idx(4,4), false, Some(ObtainKind::Gin)));
    let v_a = calc_see(Teban::Sente, &s_a, m);

    // ケースB: 飛車あり（x-ray で後から参加）
    let mut b_b = blank();
    set_piece(&mut b_b, 4,4, GGin);
    set_piece(&mut b_b, 4,5, SFu);
    set_piece(&mut b_b, 4,3, GFu);
    set_piece(&mut b_b, 5,4, SKin);
    set_piece(&mut b_b, 3,4, SGin);
    set_piece(&mut b_b, 4,0, GHisha);
    let s_b = State::new(b_b);
    let v_b = calc_see(Teban::Sente, &s_b, m);

    // 効き復活により交換が一手以上伸び、最終SEEが変化し得ることを検証する。
    assert_ne!(v_b, v_a, "Hidden rook joining later should change the final SEE value");
}

#[test]
fn see_xray_bishop_activation_changes_see() {
    // 斜め x-ray 版。1,1 の後手角が、ブロッカー(3,3)が 4,4 に動くことで参加できる。

    // ケースA: 角なし
    let mut b_a = blank();
    set_piece(&mut b_a, 4,4, GGin);
    set_piece(&mut b_a, 4,5, SFu);
    set_piece(&mut b_a, 3,3, GGin); // ブロッカー兼取り返し
    set_piece(&mut b_a, 5,5, SKin);
    set_piece(&mut b_a, 6,6, SHisha); // 先手追加攻め駒
    let s_a = State::new(b_a);
    let m = LegalMove::To(LegalMoveTo::new(idx(4,5), idx(4,4), false, Some(ObtainKind::Gin)));
    let v_a = calc_see(Teban::Sente, &s_a, m);

    // ケースB: 角あり（x-ray で後から参加）
    let mut b_b = blank();
    set_piece(&mut b_b, 4,4, GGin);
    set_piece(&mut b_b, 4,5, SFu);
    set_piece(&mut b_b, 3,3, GGin);
    set_piece(&mut b_b, 5,5, SKin);
    set_piece(&mut b_b, 6,6, SHisha);
    set_piece(&mut b_b, 1,1, GKaku);
    let s_b = State::new(b_b);
    let v_b = calc_see(Teban::Sente, &s_b, m);

    assert_ne!(v_b, v_a, "Hidden bishop joining later should change the final SEE value");
}

#[test]
fn see_xray_lance_activation_changes_see() {
    // 縦 x-ray 版。4,0 の後手香が、ブロッカー(4,3)が 4,4 に動くことで参加できる。

    // ケースA: 香なし
    let mut b_a = blank();
    set_piece(&mut b_a, 4,4, GGin);
    set_piece(&mut b_a, 4,5, SFu);
    set_piece(&mut b_a, 4,3, GFu);
    set_piece(&mut b_a, 5,4, SKin);
    set_piece(&mut b_a, 3,4, SGin); // 追加の先手追撃駒
    let s_a = State::new(b_a);
    let m = LegalMove::To(LegalMoveTo::new(idx(4,5), idx(4,4), false, Some(ObtainKind::Gin)));
    let v_a = calc_see(Teban::Sente, &s_a, m);

    // ケースB: 香あり（x-ray で後から参加）
    let mut b_b = blank();
    set_piece(&mut b_b, 4,4, GGin);
    set_piece(&mut b_b, 4,5, SFu);
    set_piece(&mut b_b, 4,3, GFu);
    set_piece(&mut b_b, 5,4, SKin);
    set_piece(&mut b_b, 3,4, SGin);
    set_piece(&mut b_b, 4,0, GKyou);
    let s_b = State::new(b_b);
    let v_b = calc_see(Teban::Sente, &s_b, m);

    assert_ne!(v_b, v_a, "Hidden lance joining later should change the final SEE value");
}
