//! 探索の最適化に用いるSEEの計算を実装する

use bitboard::BitBoard;
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
    let mut state = state.clone();

    let mut current_score = 0;

    let target = match m {
        LegalMove::To(m) => m.dst(),
        LegalMove::Put(m) => m.dst(),
    };

    if m.obtained().is_some() {
        let (x,y) = target.square_to_point();

        let kind = state.get_banmen().0[y as usize][x as usize];

        // 初手で取った駒のスコアを設定
        current_score += PIECE_SCORE_MAP[kind as usize];
    }

    let mut score = current_score;

    let (
        self_fu_bb, mut self_kyou_bb, self_kei_bb,
        self_gin_bb, self_kin_bb, self_nari_kin_bb,
        mut self_kaku_bb, mut self_hisha_bb,
        mut self_kaku_nari_bb, mut self_hisha_nari_bb, self_ou_bb,
        opponent_fu_bb, mut opponent_kyou_bb, opponent_kei_bb,
        opponent_gin_bb, opponent_kin_bb, opponent_nari_kin_bb,
        mut opponent_kaku_bb, mut opponent_hisha_bb,
        mut opponent_kaku_nari_bb, mut opponent_hisha_nari_bb,
        opponent_ou_bb
    ) = if teban == Teban::Sente {
        let opponent_fu_bb = Rule::has_control_bits_gote_fu(&state, target as Square);
        let opponent_kyou_bb = Rule::has_control_bits_gote_kyou(&state, target as Square);
        let opponent_kei_bb = Rule::has_control_bits_gote_kei(&state, target as Square);
        let opponent_gin_bb = Rule::has_control_bits_gote_gin(&state, target as Square);
        let opponent_kin_bb = Rule::has_control_bits_gote_kin(&state, target as Square);
        let opponent_nari_kin_bb = Rule::has_control_bits_gote_nari_kin(&state, target as Square);
        let opponent_kaku_bb = Rule::has_control_bits_gote_kaku(&state, target as Square);
        let opponent_hisha_bb = Rule::has_control_bits_gote_hisha(&state, target as Square);
        let opponent_kaku_nari_bb = Rule::has_control_bits_gote_kaku_nari(&state, target as Square);
        let opponent_hisha_nari_bb = Rule::has_control_bits_gote_hisha_nari(&state, target as Square);
        let opponent_ou_bb = Rule::has_control_bits_gote_ou(&state, target as Square);

        let self_fu_bb = Rule::has_control_bits_sente_fu(&state, target as Square);
        let self_kyou_bb = Rule::has_control_bits_sente_kyou(&state, target as Square);
        let self_kei_bb = Rule::has_control_bits_sente_kei(&state, target as Square);
        let self_gin_bb = Rule::has_control_bits_sente_gin(&state, target as Square);
        let self_kin_bb = Rule::has_control_bits_sente_kin(&state, target as Square);
        let self_nari_kin_bb = Rule::has_control_bits_sente_nari_kin(&state, target as Square);
        let self_kaku_bb = Rule::has_control_bits_sente_kaku(&state, target as Square);
        let self_hisha_bb = Rule::has_control_bits_sente_hisha(&state, target as Square);
        let self_kaku_nari_bb = Rule::has_control_bits_sente_kaku_nari(&state, target as Square);
        let self_hisha_nari_bb = Rule::has_control_bits_sente_hisha_nari(&state, target as Square);
        let self_ou_bb = Rule::has_control_bits_sente_ou(&state, target as Square);

        (
            self_fu_bb, self_kyou_bb, self_kei_bb, self_gin_bb, self_kin_bb, self_nari_kin_bb, self_kaku_bb, self_hisha_bb,
            self_kaku_nari_bb, self_hisha_nari_bb, self_ou_bb,
            opponent_fu_bb, opponent_kyou_bb, opponent_kei_bb, opponent_gin_bb, opponent_kin_bb, opponent_nari_kin_bb,
            opponent_kaku_bb, opponent_hisha_bb,
            opponent_kaku_nari_bb, opponent_hisha_nari_bb, opponent_ou_bb
        )
    } else {
        let opponent_fu_bb = Rule::has_control_bits_sente_fu(&state, target as Square);
        let opponent_kyou_bb = Rule::has_control_bits_sente_kyou(&state, target as Square);
        let opponent_kei_bb = Rule::has_control_bits_sente_kei(&state, target as Square);
        let opponent_gin_bb = Rule::has_control_bits_sente_gin(&state, target as Square);
        let opponent_kin_bb = Rule::has_control_bits_sente_kin(&state, target as Square);
        let opponent_nari_kin_bb = Rule::has_control_bits_sente_nari_kin(&state, target as Square);
        let opponent_kaku_bb = Rule::has_control_bits_sente_kaku(&state, target as Square);
        let opponent_hisha_bb = Rule::has_control_bits_sente_hisha(&state, target as Square);
        let opponent_kaku_nari_bb = Rule::has_control_bits_sente_kaku_nari(&state, target as Square);
        let opponent_hisha_nari_bb = Rule::has_control_bits_sente_hisha_nari(&state, target as Square);
        let opponent_ou_bb = Rule::has_control_bits_sente_ou(&state, target as Square);

        let self_fu_bb = Rule::has_control_bits_gote_fu(&state, target as Square);
        let self_kyou_bb = Rule::has_control_bits_gote_kyou(&state, target as Square);
        let self_kei_bb = Rule::has_control_bits_gote_kei(&state, target as Square);
        let self_gin_bb = Rule::has_control_bits_gote_gin(&state, target as Square);
        let self_kin_bb = Rule::has_control_bits_gote_kin(&state, target as Square);
        let self_nari_kin_bb = Rule::has_control_bits_gote_nari_kin(&state, target as Square);
        let self_kaku_bb = Rule::has_control_bits_gote_kaku(&state, target as Square);
        let self_hisha_bb = Rule::has_control_bits_gote_hisha(&state, target as Square);
        let self_kaku_nari_bb = Rule::has_control_bits_gote_kaku_nari(&state, target as Square);
        let self_hisha_nari_bb = Rule::has_control_bits_gote_hisha_nari(&state, target as Square);
        let self_ou_bb = Rule::has_control_bits_gote_ou(&state, target as Square);

        (
            self_fu_bb, self_kyou_bb, self_kei_bb, self_gin_bb, self_kin_bb, self_nari_kin_bb, self_kaku_bb, self_hisha_bb,
            self_kaku_nari_bb, self_hisha_nari_bb, self_ou_bb,
            opponent_fu_bb, opponent_kyou_bb, opponent_kei_bb, opponent_gin_bb, opponent_kin_bb, opponent_nari_kin_bb,
            opponent_kaku_bb, opponent_hisha_bb,
            opponent_kaku_nari_bb, opponent_hisha_nari_bb, opponent_ou_bb
        )
    };

    // 飛車、角、香車以外は取り合い中に効きが増えることはないし効きが消えることもないので
    // イテレータを一度取ったらあとは読み出すだけ
    let mut self_fu_it = self_fu_bb.iter();
    let mut self_kyou_it = self_kyou_bb.iter();
    let mut self_kei_it = self_kei_bb.iter();
    let mut self_gin_it = self_gin_bb.iter();
    let mut self_kin_it = self_kin_bb.iter();
    let mut self_nari_kin_it = self_nari_kin_bb.iter();
    let mut self_kaku_it = self_kaku_bb.iter();
    let mut self_hisha_it = self_hisha_bb.iter();
    let mut self_kaku_nari_it = self_kaku_nari_bb.iter();
    let mut self_hisha_nari_it = self_hisha_nari_bb.iter();
    let mut self_ou_it = self_ou_bb.iter();

    let mut opponent_fu_it = opponent_fu_bb.iter();
    let mut opponent_kyou_it = opponent_kyou_bb.iter();
    let mut opponent_kei_it = opponent_kei_bb.iter();
    let mut opponent_gin_it = opponent_gin_bb.iter();
    let mut opponent_kin_it = opponent_kin_bb.iter();
    let mut opponent_nari_kin_it = opponent_nari_kin_bb.iter();
    let mut opponent_kaku_it = opponent_kaku_bb.iter();
    let mut opponent_hisha_it = opponent_hisha_bb.iter();
    let mut opponent_kaku_nari_it = opponent_kaku_nari_bb.iter();
    let mut opponent_hisha_nari_it = opponent_hisha_nari_bb.iter();
    let mut opponent_ou_it = opponent_ou_bb.iter();

    let mut gain = vec![];

    let mut isself = true;

    // 逆伝播
    #[inline]
    fn update_gain(gain:&mut Vec<i32>, current_score:&mut i32, mut score:i32, next_score:i32) -> i32 {
        if gain.len() == 0 {
            score = *current_score;
            gain.push(score);
        } else {
            score = *current_score - score;
            gain.push(-score);
        }
        *current_score = next_score;

        score
    }

    // 動かした駒をoccupiedから取り除く。そのうえで飛車、角、香車の効きを手番側相手番側ともに再列挙
    #[inline]
    fn pull_occupied(state:&mut State, teban: Teban, target: Square, p:Square,
        self_kyou_bb: &mut BitBoard, self_kaku_bb: &mut BitBoard, self_hisha_bb: &mut BitBoard,
        self_kaku_nari_bb: &mut BitBoard, self_hisha_nari_bb: &mut BitBoard,
        opponent_kyou_bb: &mut BitBoard, opponent_kaku_bb: &mut BitBoard, opponent_hisha_bb: &mut BitBoard,
        opponent_kaku_nari_bb: &mut BitBoard, opponent_hisha_nari_bb: &mut BitBoard,
    ) {
        match teban {
            Teban::Sente => {
                // ビットボードからターゲットのマスの駒のビットを取り除く
                state.part.sente_opponent_board ^= 1 << (target + 1);
                state.part.gote_self_board ^= 1 << (80 - target + 1);

                // ビットボードからターゲットのマスへ動かした駒のビットを取り除く
                state.part.sente_self_board ^= 1 << (p + 1);
                state.part.gote_opponent_board ^= 1 << (80 - p + 1);

                // 手番側の飛車、角、香車の効きを再計算
                *self_kyou_bb = Rule::has_control_bits_sente_kyou(state, target as Square);
                *self_kaku_bb = Rule::has_control_bits_sente_kaku(state, target as Square);
                *self_hisha_bb = Rule::has_control_bits_sente_hisha(state, target as Square);
                *self_kaku_nari_bb = Rule::has_control_bits_sente_kaku_nari(state, target as Square);
                *self_hisha_nari_bb = Rule::has_control_bits_sente_hisha_nari(state, target as Square);

                // 非手番側の飛車、角、香車の効きを再計算
                *opponent_kyou_bb = Rule::has_control_bits_gote_kyou(state, target as Square);
                *opponent_kaku_bb = Rule::has_control_bits_gote_kaku(state, target as Square);
                *opponent_hisha_bb = Rule::has_control_bits_gote_hisha(state, target as Square);
                *opponent_kaku_nari_bb = Rule::has_control_bits_gote_kaku_nari(state, target as Square);
                *opponent_hisha_nari_bb = Rule::has_control_bits_gote_hisha_nari(state, target as Square);
            },
            Teban::Gote => {
                // ビットボードからターゲットのマスの駒のビットを取り除く
                state.part.gote_opponent_board ^= 1 << (80 - target + 1);
                state.part.sente_self_board ^= 1 << (target + 1);

                // ビットボードからターゲットのマスへ動かした駒のビットを取り除く
                state.part.gote_self_board ^= 1 << (80 - p + 1);
                state.part.sente_opponent_board ^= 1 << (p + 1);

                // 手番側の飛車、角、香車の効きを再計算
                *self_kyou_bb = Rule::has_control_bits_gote_kyou(state, target as Square);
                *self_kaku_bb = Rule::has_control_bits_gote_kaku(state, target as Square);
                *self_hisha_bb = Rule::has_control_bits_gote_hisha(state, target as Square);
                *self_kaku_nari_bb = Rule::has_control_bits_gote_kaku_nari(state, target as Square);
                *self_hisha_nari_bb = Rule::has_control_bits_gote_hisha_nari(state, target as Square);

                // 非手番側の飛車、角、香車の効きを再計算
                *opponent_kyou_bb = Rule::has_control_bits_sente_kyou(state, target as Square);
                *opponent_kaku_bb = Rule::has_control_bits_sente_kaku(state, target as Square);
                *opponent_hisha_bb = Rule::has_control_bits_sente_hisha(state, target as Square);
                *opponent_kaku_nari_bb = Rule::has_control_bits_sente_kaku_nari(state, target as Square);
                *opponent_hisha_nari_bb = Rule::has_control_bits_sente_hisha_nari(state, target as Square);
            }
        }
    }

    loop {
        if isself {
            if let Some(p) =self_fu_it.next() {
                pull_occupied(&mut state,teban,target as Square,
                              p as Square,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb);
                self_kyou_it = self_kyou_bb.iter();
                self_kaku_it = self_kaku_bb.iter();
                self_hisha_it = self_hisha_bb.iter();
                self_kaku_nari_it = self_kaku_nari_bb.iter();
                self_hisha_nari_it = self_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, FU_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = self_kyou_it.next() {
                self_kyou_bb ^= 1 << (p + 1);

                pull_occupied(&mut state,teban,target as Square,
                              p as Square,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb);
                self_kyou_it = self_kyou_bb.iter();
                self_kaku_it = self_kaku_bb.iter();
                self_hisha_it = self_hisha_bb.iter();
                self_kaku_nari_it = self_kaku_nari_bb.iter();
                self_hisha_nari_it = self_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, KYOU_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = self_kei_it.next() {
                pull_occupied(&mut state,teban,target as Square,
                              p as Square,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb);
                self_kyou_it = self_kyou_bb.iter();
                self_kaku_it = self_kaku_bb.iter();
                self_hisha_it = self_hisha_bb.iter();
                self_kaku_nari_it = self_kaku_nari_bb.iter();
                self_hisha_nari_it = self_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, KEI_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = self_gin_it.next() {
                pull_occupied(&mut state,teban,target as Square,
                              p as Square,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb);
                self_kyou_it = self_kyou_bb.iter();
                self_kaku_it = self_kaku_bb.iter();
                self_hisha_it = self_hisha_bb.iter();
                self_kaku_nari_it = self_kaku_nari_bb.iter();
                self_hisha_nari_it = self_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, GIN_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = self_kin_it.next() {
                pull_occupied(&mut state,teban,target as Square,
                              p as Square,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb);
                self_kyou_it = self_kyou_bb.iter();
                self_kaku_it = self_kaku_bb.iter();
                self_hisha_it = self_hisha_bb.iter();
                self_kaku_nari_it = self_kaku_nari_bb.iter();
                self_hisha_nari_it = self_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, KIN_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = self_nari_kin_it.next() {
                pull_occupied(&mut state,teban,target as Square,
                              p as Square,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb);
                self_kyou_it = self_kyou_bb.iter();
                self_kaku_it = self_kaku_bb.iter();
                self_hisha_it = self_hisha_bb.iter();
                self_kaku_nari_it = self_kaku_nari_bb.iter();
                self_hisha_nari_it = self_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, KIN_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = self_kaku_it.next() {
                self_kaku_bb ^= 1 << (p + 1);

                pull_occupied(&mut state,teban,target as Square,
                              p as Square,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb);
                self_kyou_it = self_kyou_bb.iter();
                self_kaku_it = self_kaku_bb.iter();
                self_hisha_it = self_hisha_bb.iter();
                self_kaku_nari_it = self_kaku_nari_bb.iter();
                self_hisha_nari_it = self_hisha_nari_bb.iter();

                update_gain(&mut gain, &mut current_score, score, KAKU_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = self_hisha_it.next() {
                self_hisha_bb ^= 1 << (p + 1);

                pull_occupied(&mut state,teban,target as Square,
                              p as Square,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb);
                self_kyou_it = self_kyou_bb.iter();
                self_kaku_it = self_kaku_bb.iter();
                self_hisha_it = self_hisha_bb.iter();
                self_kaku_nari_it = self_kaku_nari_bb.iter();
                self_hisha_nari_it = self_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, HISHA_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = self_kaku_nari_it.next() {
                self_kaku_nari_bb ^= 1 << (p + 1);

                pull_occupied(&mut state,teban,target as Square,
                              p as Square,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb);
                self_kyou_it = self_kyou_bb.iter();
                self_kaku_it = self_kaku_bb.iter();
                self_hisha_it = self_hisha_bb.iter();
                self_kaku_nari_it = self_kaku_nari_bb.iter();
                self_hisha_nari_it = self_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, KAKU_NARI_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = self_hisha_nari_it.next() {
                self_hisha_nari_bb ^= 1 << (p + 1);

                pull_occupied(&mut state,teban,target as Square,
                              p as Square,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb);
                self_kyou_it = self_kyou_bb.iter();
                self_kaku_it = self_kaku_bb.iter();
                self_hisha_it = self_hisha_bb.iter();
                self_kaku_nari_it = self_kaku_nari_bb.iter();
                self_hisha_nari_it = self_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, HISHA_NARI_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = self_ou_it.next() {
                pull_occupied(&mut state,teban,target as Square,
                              p as Square,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb);
                self_kyou_it = self_kyou_bb.iter();
                self_kaku_it = self_kaku_bb.iter();
                self_hisha_it = self_hisha_bb.iter();
                self_kaku_nari_it = self_kaku_nari_bb.iter();
                self_hisha_nari_it = self_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, OU_SCORE);
                isself = !isself;
                continue;
            }

            break;
        } else {
            if let Some(p) = opponent_fu_it.next() {
                pull_occupied(&mut state,teban.opposite(),target as Square,
                              p as Square,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb);
                opponent_kyou_it = opponent_kyou_bb.iter();
                opponent_kaku_it = opponent_kaku_bb.iter();
                opponent_hisha_it = opponent_hisha_bb.iter();
                opponent_kaku_nari_it = opponent_kaku_nari_bb.iter();
                opponent_hisha_nari_it = opponent_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, FU_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = opponent_kyou_it.next() {
                opponent_kyou_bb ^= 1 << (p + 1);

                pull_occupied(&mut state,teban.opposite(),target as Square,
                              p as Square,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb);
                opponent_kyou_it = opponent_kyou_bb.iter();
                opponent_kaku_it = opponent_kaku_bb.iter();
                opponent_hisha_it = opponent_hisha_bb.iter();
                opponent_kaku_nari_it = opponent_kaku_nari_bb.iter();
                opponent_hisha_nari_it = opponent_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, KYOU_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = opponent_kei_it.next() {
                pull_occupied(&mut state,teban.opposite(),target as Square,
                              p as Square,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb);
                opponent_kyou_it = opponent_kyou_bb.iter();
                opponent_kaku_it = opponent_kaku_bb.iter();
                opponent_hisha_it = opponent_hisha_bb.iter();
                opponent_kaku_nari_it = opponent_kaku_nari_bb.iter();
                opponent_hisha_nari_it = opponent_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, KEI_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = opponent_gin_it.next() {
                pull_occupied(&mut state,teban.opposite(),target as Square,
                              p as Square,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb);
                opponent_kyou_it = opponent_kyou_bb.iter();
                opponent_kaku_it = opponent_kaku_bb.iter();
                opponent_hisha_it = opponent_hisha_bb.iter();
                opponent_kaku_nari_it = opponent_kaku_nari_bb.iter();
                opponent_hisha_nari_it = opponent_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, GIN_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = opponent_kin_it.next() {
                pull_occupied(&mut state,teban.opposite(),target as Square,
                              p as Square,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb);
                opponent_kyou_it = opponent_kyou_bb.iter();
                opponent_kaku_it = opponent_kaku_bb.iter();
                opponent_hisha_it = opponent_hisha_bb.iter();
                opponent_kaku_nari_it = opponent_kaku_nari_bb.iter();
                opponent_hisha_nari_it = opponent_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, KIN_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = opponent_nari_kin_it.next() {
                pull_occupied(&mut state,teban.opposite(),target as Square,
                              p as Square,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb);
                opponent_kyou_it = opponent_kyou_bb.iter();
                opponent_kaku_it = opponent_kaku_bb.iter();
                opponent_hisha_it = opponent_hisha_bb.iter();
                opponent_kaku_nari_it = opponent_kaku_nari_bb.iter();
                opponent_hisha_nari_it = opponent_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, KIN_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = opponent_kaku_it.next() {
                opponent_kaku_bb ^= 1 << (p + 1);

                pull_occupied(&mut state,teban.opposite(),target as Square,
                              p as Square,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb);
                opponent_kyou_it = opponent_kyou_bb.iter();
                opponent_kaku_it = opponent_kaku_bb.iter();
                opponent_hisha_it = opponent_hisha_bb.iter();
                opponent_kaku_nari_it = opponent_kaku_nari_bb.iter();
                opponent_hisha_nari_it = opponent_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, KAKU_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = opponent_hisha_it.next() {
                opponent_hisha_bb ^= 1 << (p + 1);

                pull_occupied(&mut state,teban.opposite(),target as Square,
                              p as Square,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb);
                opponent_kyou_it = opponent_kyou_bb.iter();
                opponent_kaku_it = opponent_kaku_bb.iter();
                opponent_hisha_it = opponent_hisha_bb.iter();
                opponent_kaku_nari_it = opponent_kaku_nari_bb.iter();
                opponent_hisha_nari_it = opponent_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, HISHA_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = opponent_kaku_nari_it.next() {
                opponent_kaku_nari_bb ^= 1 << (p + 1);

                pull_occupied(&mut state,teban.opposite(),target as Square,
                              p as Square,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb);
                opponent_kyou_it = opponent_kyou_bb.iter();
                opponent_kaku_it = opponent_kaku_bb.iter();
                opponent_hisha_it = opponent_hisha_bb.iter();
                opponent_kaku_nari_it = opponent_kaku_nari_bb.iter();
                opponent_hisha_nari_it = opponent_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, KAKU_NARI_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = opponent_hisha_nari_it.next() {
                opponent_hisha_nari_bb ^= 1 << (p + 1);

                pull_occupied(&mut state,teban.opposite(),target as Square,
                              p as Square,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb);
                opponent_kyou_it = opponent_kyou_bb.iter();
                opponent_kaku_it = opponent_kaku_bb.iter();
                opponent_hisha_it = opponent_hisha_bb.iter();
                opponent_kaku_nari_it = opponent_kaku_nari_bb.iter();
                opponent_hisha_nari_it = opponent_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, HISHA_NARI_SCORE);
                isself = !isself;
                continue;
            }

            if let Some(p) = opponent_ou_it.next() {
                pull_occupied(&mut state,teban.opposite(),target as Square,
                              p as Square,
                              &mut opponent_kyou_bb,
                              &mut opponent_kaku_bb,
                              &mut opponent_hisha_bb,
                              &mut opponent_kaku_nari_bb,
                              &mut opponent_hisha_nari_bb,
                              &mut self_kyou_bb,
                              &mut self_kaku_bb,
                              &mut self_hisha_bb,
                              &mut self_kaku_nari_bb,
                              &mut self_hisha_nari_bb);
                opponent_kyou_it = opponent_kyou_bb.iter();
                opponent_kaku_it = opponent_kaku_bb.iter();
                opponent_hisha_it = opponent_hisha_bb.iter();
                opponent_kaku_nari_it = opponent_kaku_nari_bb.iter();
                opponent_hisha_nari_it = opponent_hisha_nari_bb.iter();

                score = update_gain(&mut gain, &mut current_score, score, OU_SCORE);
                isself = !isself;
                continue;
            }

            break;
        }
    }

    if gain.is_empty() {
        return 0;
    }

    let mut i = gain.len() - 1;

    while i > 0 {
        gain[i-1] = (gain[i-1]).min(-gain[i]);
        i -= 1;
    }

    -gain[0]
}
