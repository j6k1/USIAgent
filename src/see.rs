//! 探索の最適化に用いるSEEの計算を実装する
use rule::{LegalMove, Rule, Square, SquareToPoint, State};
use shogi::{Teban};

const FU_SCORE:i32 = 90 * 9 / 10;
const KYOU_SCORE:i32 = 315 * 9 / 10;
const KEI_SCORE:i32 = 405 * 9 / 10;
const GIN_SCORE:i32 = 495 * 9 / 10;
const KIN_SCORE:i32 = 540 * 9 / 10;
const KAKU_SCORE:i32 = 855 * 9 / 10;
const HISHA_SCORE:i32 = 990 * 9 / 10;
const KAKU_NARI_SCORE:i32 = 945 * 9 / 10;
const HISHA_NARI_SCORE:i32 = 1395 * 9 / 10;
const OU_SCORE:i32 = 15000 * 9 / 10;

const PIECE_SCORE_MAP:[i32; 29] = [
    90 * 9 / 10,
    315 * 9 / 10,
    405 * 9 / 10,
    495 * 9 / 10,
    540 * 9 / 10,
    855 * 9 / 10,
    990 * 9 / 10,
    15000 * 9 / 10,
    540 * 9 / 10,
    540 * 9 / 10,
    540 * 9 / 10,
    540 * 9 / 10,
    945 * 9 / 10,
    1395 * 9 / 10,
    90 * 9 / 10,
    315 * 9 / 10,
    405 * 9 / 10,
    495 * 9 / 10,
    540 * 9 / 10,
    855 * 9 / 10,
    990 * 9 / 10,
    15000 * 9 / 10,
    540 * 9 / 10,
    540 * 9 / 10,
    540 * 9 / 10,
    540 * 9 / 10,
    945 * 9 / 10,
    1395 * 9 / 10,
    0
];

/// SEEを計算する
///
/// # Arguments
/// * `teban` - 手番
/// * `state` - 局面
/// * `m` - 手
pub fn calc_see(teban: Teban, state:&State, m: LegalMove) -> i32 {
    let mut score = 0;

    let target = match m {
        LegalMove::To(m) => m.dst(),
        LegalMove::Put(m) => m.dst(),
    };

    if let LegalMove::To(m) = m {
        if m.obtained().is_some() {
            let (x,y) = target.square_to_point();

            let kind = state.get_banmen().0[y as usize][x as usize];

            score += PIECE_SCORE_MAP[kind as usize];
        }
    }

    let (
        sfu_bb,skyou_bb,skei_bb,sgin_bb,skin_bb,snkin_bb,
        skaku_bb,shisha_bb,
        skakun_bb,shishan_bb,sou_bb,
        ofu_bb,okyou_bb,okei_bb,ogin_bb,okin_bb,onkin_bb,
        okaku_bb,ohisha_bb,
        okakun_bb,ohishan_bb,oou_bb
    ) = if teban == Teban::Sente {
        let ofu_bb = Rule::has_control_bits_gote_fu(state, target as Square);
        let okyou_bb = Rule::has_control_bits_gote_kyou(state, target as Square);
        let okei_bb = Rule::has_control_bits_gote_kei(state, target as Square);
        let ogin_bb = Rule::has_control_bits_gote_gin(state, target as Square);
        let okin_bb = Rule::has_control_bits_gote_kin(state, target as Square);
        let onkin_bb = Rule::has_control_bits_gote_nari_kin(state, target as Square);
        let okaku_bb = Rule::has_control_bits_gote_kaku(state, target as Square);
        let ohisha_bb = Rule::has_control_bits_gote_hisha(state, target as Square);
        let okakun_bb = Rule::has_control_bits_gote_kaku_nari(state, target as Square);
        let ohishan_bb = Rule::has_control_bits_gote_hisha_nari(state, target as Square);
        let oou_bb = Rule::has_control_bits_gote_ou(state, target as Square);

        if ofu_bb == 0 && okyou_bb == 0 && okei_bb == 0 && ogin_bb == 0 && okin_bb == 0 &&
            onkin_bb == 0 && okaku_bb == 0 && ohisha_bb == 0 && okakun_bb == 0 && ohishan_bb == 0 && oou_bb == 0 {
            return score;
        }

        let sfu_bb = Rule::has_control_bits_sente_fu(state, target as Square);
        let skyou_bb = Rule::has_control_bits_sente_kyou(state, target as Square);
        let skei_bb = Rule::has_control_bits_sente_kei(state, target as Square);
        let sgin_bb = Rule::has_control_bits_sente_gin(state, target as Square);
        let skin_bb = Rule::has_control_bits_sente_kin(state, target as Square);
        let snkin_bb = Rule::has_control_bits_sente_nari_kin(state, target as Square);
        let skaku_bb = Rule::has_control_bits_sente_kaku(state, target as Square);
        let shisha_bb = Rule::has_control_bits_sente_hisha(state, target as Square);
        let skakun_bb = Rule::has_control_bits_sente_kaku_nari(state, target as Square);
        let shishan_bb = Rule::has_control_bits_sente_hisha_nari(state, target as Square);
        let sou_bb = Rule::has_control_bits_sente_ou(state, target as Square);

        (
            sfu_bb,skyou_bb,skei_bb,sgin_bb,skin_bb,snkin_bb,skaku_bb,shisha_bb,
            skakun_bb,shishan_bb,sou_bb,
            ofu_bb,okyou_bb,okei_bb,ogin_bb,okin_bb,onkin_bb,okaku_bb,ohisha_bb,
            okakun_bb,ohishan_bb,oou_bb
        )
    } else {
        let ofu_bb = Rule::has_control_bits_sente_fu(state, target as Square);
        let okyou_bb = Rule::has_control_bits_sente_kyou(state, target as Square);
        let okei_bb = Rule::has_control_bits_sente_kei(state, target as Square);
        let ogin_bb = Rule::has_control_bits_sente_gin(state, target as Square);
        let okin_bb = Rule::has_control_bits_sente_kin(state, target as Square);
        let onkin_bb = Rule::has_control_bits_sente_nari_kin(state, target as Square);
        let okaku_bb = Rule::has_control_bits_sente_kaku(state, target as Square);
        let ohisha_bb = Rule::has_control_bits_sente_hisha(state, target as Square);
        let okakun_bb = Rule::has_control_bits_sente_kaku_nari(state, target as Square);
        let ohishan_bb = Rule::has_control_bits_sente_hisha_nari(state, target as Square);
        let oou_bb = Rule::has_control_bits_sente_ou(state, target as Square);

        if ofu_bb == 0 && okyou_bb == 0 && okei_bb == 0 && ogin_bb == 0 && okin_bb == 0 &&
            onkin_bb == 0 && okaku_bb == 0 && ohisha_bb == 0 && okakun_bb == 0 && ohishan_bb == 0 && oou_bb == 0 {
            return score;
        }

        let sfu_bb = Rule::has_control_bits_gote_fu(state, target as Square);
        let skyou_bb = Rule::has_control_bits_gote_kyou(state, target as Square);
        let skei_bb = Rule::has_control_bits_gote_kei(state, target as Square);
        let sgin_bb = Rule::has_control_bits_gote_gin(state, target as Square);
        let skin_bb = Rule::has_control_bits_gote_kin(state, target as Square);
        let snkin_bb = Rule::has_control_bits_gote_nari_kin(state, target as Square);
        let skaku_bb = Rule::has_control_bits_gote_kaku(state, target as Square);
        let shisha_bb = Rule::has_control_bits_gote_hisha(state, target as Square);
        let skakun_bb = Rule::has_control_bits_gote_kaku_nari(state, target as Square);
        let shishan_bb = Rule::has_control_bits_gote_hisha_nari(state, target as Square);
        let sou_bb = Rule::has_control_bits_gote_ou(state, target as Square);

        (
            sfu_bb,skyou_bb,skei_bb,sgin_bb,skin_bb,snkin_bb,skaku_bb,shisha_bb,
            skakun_bb,shishan_bb,sou_bb,
            ofu_bb,okyou_bb,okei_bb,ogin_bb,okin_bb,onkin_bb,okaku_bb,ohisha_bb,
            okakun_bb,ohishan_bb,oou_bb
        )
    };

    let mut sfu_it = sfu_bb.iter();
    let mut skyou_it = skyou_bb.iter();
    let mut skei_it = skei_bb.iter();
    let mut sgin_it = sgin_bb.iter();
    let mut skin_it = skin_bb.iter();
    let mut snkin_it = snkin_bb.iter();
    let mut skaku_it = skaku_bb.iter();
    let mut shisha_it = shisha_bb.iter();
    let mut skakun_it = skakun_bb.iter();
    let mut shishan_it = shishan_bb.iter();
    let mut sou_it = sou_bb.iter();

    let mut ofu_it = ofu_bb.iter();
    let mut okyou_it = okyou_bb.iter();
    let mut okei_it = okei_bb.iter();
    let mut ogin_it = ogin_bb.iter();
    let mut okin_it = okin_bb.iter();
    let mut onkin_it = onkin_bb.iter();
    let mut okaku_it = okaku_bb.iter();
    let mut ohisha_it = ohisha_bb.iter();
    let mut okakun_it = okakun_bb.iter();
    let mut ohishan_it = ohishan_bb.iter();
    let mut oou_it = oou_bb.iter();

    let mut gain = vec![score];

    let mut isself = false;

    let mut current_score = score;

    #[inline]
    fn update_gain(gain:&mut Vec<i32>, current_score:&mut i32, next_score:i32) {
        let g = gain.last().unwrap();
        gain.push(*current_score - g);
        *current_score = next_score;
    }

    loop {
        if isself {
            if sfu_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, FU_SCORE);
                isself = !isself;
                continue;
            }

            if skyou_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, KYOU_SCORE);
                isself = !isself;
                continue;
            }

            if skei_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, KEI_SCORE);
                isself = !isself;
                continue;
            }

            if sgin_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, GIN_SCORE);
                isself = !isself;
                continue;
            }

            if skin_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, KIN_SCORE);
                isself = !isself;
                continue;
            }

            if snkin_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, KIN_SCORE);
                isself = !isself;
                continue;
            }

            if skaku_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, KAKU_SCORE);
                isself = !isself;
                continue;
            }

            if shisha_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, HISHA_SCORE);
                isself = !isself;
                continue;
            }

            if skakun_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, KAKU_NARI_SCORE);
                isself = !isself;
                continue;
            }

            if shishan_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, HISHA_NARI_SCORE);
                isself = !isself;
                continue;
            }

            if sou_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, OU_SCORE);
                isself = !isself;
                continue;
            }

            break;
        } else {
            if ofu_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, FU_SCORE);
                isself = !isself;
                continue;
            }

            if okyou_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, KYOU_SCORE);
                isself = !isself;
                continue;
            }

            if okei_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, KEI_SCORE);
                isself = !isself;
                continue;
            }

            if ogin_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, GIN_SCORE);
                isself = !isself;
                continue;
            }

            if okin_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, KIN_SCORE);
                isself = !isself;
                continue;
            }

            if onkin_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, KIN_SCORE);
                isself = !isself;
                continue;
            }

            if okaku_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, KAKU_SCORE);
                isself = !isself;
                continue;
            }

            if ohisha_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, HISHA_SCORE);
                isself = !isself;
                continue;
            }

            if okakun_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, KAKU_NARI_SCORE);
                isself = !isself;
                continue;
            }

            if ohishan_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, HISHA_NARI_SCORE);
                isself = !isself;
                continue;
            }

            if oou_it.next().is_some() {
                update_gain(&mut gain, &mut current_score, OU_SCORE);
                isself = !isself;
                continue;
            }

            break;
        }
    }

    let mut i = gain.len() - 1;

    while i > 0 {
        gain[i-1] = (-gain[i]).max(gain[i-1]);
        i -= 1;
    }

    gain[0]
}
