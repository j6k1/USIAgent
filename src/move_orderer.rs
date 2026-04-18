//! 探索時の手の並び替えの機能を実装する

use std::fmt::Debug;
use std::marker::PhantomData;
use error::InvalidInputError;
use rule::{LegalMove, Rule, SquareToPoint, State};
use see::calc_see;
use shogi::{KomaKind, Teban};
use shogi::KomaKind::GFu;
use shogi::Teban::{Gote, Sente};
use stats::StatsEntry;

const CM_BONUS:i32 = 64;
const MAIN_HISTORY_WEIGHT: i32 = 2;
const PIECE_TO_SQUARE_WEIGHT: i32 = 2;
const FOLLOW_UP_WEIGHT: i32 = 1;
const COUNTER_MOVE_WEIGHT: i32 = 1;
pub const HISTORY_SCALE: i32 = 256;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// 指し手の並び替え順
/// ※降順に並び変えるので優先度の低い物から列挙する
pub enum MoveOrder {
    BadCaptures(i32),
    Quiet(i32),
    Checks,
    KillerMoves(usize),
    GoodCaptures(i32),
    PV,
    TT
}
pub(crate) mod private {
    use rule::{LegalMove, State};
    use shogi::Teban;

    pub trait QuietSeeEffectBase {
        fn effect(teban: Teban, state: &State, m:LegalMove, score:i32) -> i32;
        fn see(teban: Teban, state: &State, m:LegalMove) -> i32;
    }
}
/// QuietSeeがスコアに与える作用の実装
pub trait QuietSeeEffect: private::QuietSeeEffectBase {
}
impl<T> QuietSeeEffect for T where T: private::QuietSeeEffectBase {
}
/// QuietSeeを使わない
#[derive(Debug, Clone, Copy)]
pub struct UnusedQuietSee;
impl private::QuietSeeEffectBase for UnusedQuietSee {
    fn effect(_: Teban, _: &State, _: LegalMove, score:i32) -> i32 {
        score
    }

    fn see(_: Teban, _: &State, _: LegalMove) -> i32 {
        0
    }
}
/// QuietSeeをFACTORの値で割る
#[derive(Debug, Clone, Copy)]
pub struct DivideFactor<const FACTOR:usize>;
impl<const FACTOR:usize> private::QuietSeeEffectBase for DivideFactor<FACTOR> {
    fn effect(teban: Teban, state: &State, m: LegalMove, score: i32) -> i32 {
        (score * FACTOR as i32 + calc_see(teban, state, m)) / FACTOR as i32
    }

    fn see(teban: Teban, state: &State, m: LegalMove) -> i32 {
        calc_see(teban,state,m)
    }
}
/// 指し手並び変え機の実装
#[derive(Debug,Clone)]
pub struct MoveOrderer<E: QuietSeeEffect + Clone + Debug> {
    killer_moves:Vec<[Option<LegalMove>; 2]>,
    usage_killer_moves:Vec<u8>,
    history:Box<[[StatsEntry<16384>;81]; 28]>,
    follow_up_history:Box<[[[[StatsEntry<16384>; 81]; 14]; 81]; 14]>,
    continuation_history:Vec<[[StatsEntry<16384>; 81]; 28]>,
    piece_to_square:Box<[[StatsEntry<16384>; 81]; 14]>,
    counter_moves: Box<[[[Option<LegalMove>;81]; 14]; 2]>,
    max_ply: usize,
    effect:PhantomData<E>,
}
impl<E: QuietSeeEffect + Clone + Debug> MoveOrderer<E> {
    /// MoveOrdererのインスタンスを生成するコンストラクタ
    ///
    /// # Arguments
    /// * `max_ply` - 現在の最大探索深さ
    #[inline]
    pub fn new(max_ply: usize) -> MoveOrderer<E> {
        MoveOrderer {
            killer_moves: vec![[None; 2]; max_ply+1],
            usage_killer_moves: vec![0; max_ply+1],
            history: Box::new([[StatsEntry::new(0);81]; 28]),
            follow_up_history: Box::new([[[[StatsEntry::new(0); 81]; 14]; 81]; 14]),
            continuation_history: vec![[[StatsEntry::new(0);81]; 28]; max_ply+1],
            piece_to_square:Box::new([[StatsEntry::new(0);81]; 14]),
            counter_moves: Box::new([[[None;81]; 14]; 2]),
            max_ply: max_ply,
            effect:PhantomData::<E>,
        }
    }

    /// Killer Moveの更新
    ///
    /// # Arguments
    /// * `ply` - 現在の探索深さ
    /// * `m` - 登録する候補手
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidInputError`] plyがコンストラクタで指定したmax_plyの値を超えている
    ///                         mが駒を取る手
    ///                         mが成る手
    ///
    /// [`InvalidInputError`]: ../error/struct.InvalidInputError.html
    #[inline]
    pub fn update_killer(&mut self, ply: u32, m: LegalMove) -> Result<(),InvalidInputError> {
        if (ply as usize) > self.max_ply {
            return Err(InvalidInputError(String::from("ply value exceeds max_ply.")));
        } else if m.obtained().is_some() {
            return Err(InvalidInputError(String::from("Move that captures a piece cannot be registered in the Killer Move.")));
        }

        if let LegalMove::To(mv) = m {
            if mv.is_nari() {
                return Err(InvalidInputError(String::from("Promotion move is not included in the killer moves.")));
            }
        }

        if self.usage_killer_moves[ply as usize] >= 1 {
            self.killer_moves[ply as usize][1] = self.killer_moves[ply as usize][0];
            self.killer_moves[ply as usize][0] = Some(m);
        } else if self.usage_killer_moves[ply as usize] == 0 {
            self.killer_moves[ply as usize][0] = Some(m);
        }

        if self.usage_killer_moves[ply as usize] < 2 {
            self.usage_killer_moves[ply as usize] += 1;
        }

        Ok(())
    }

    /// 手がKiller Moveであるか判定
    ///
    /// # Arguments
    /// * `ply` - 現在の探索深さ
    /// * `m` - Killer Moveか判定する候補手
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidInputError`] plyがコンストラクタで指定したmax_plyの値を超えている
    ///
    /// [`InvalidInputError`]: ../error/struct.InvalidInputError.html
    #[inline]
    pub fn is_killer(&self, ply: u32, m: LegalMove) -> Result<bool,InvalidInputError> {
        if (ply as usize) > self.max_ply {
            return Err(InvalidInputError(String::from("ply value exceeds max_ply.")));
        }

        Ok(if self.usage_killer_moves[ply as usize] == 0 {
            false
        } else if self.killer_moves[ply as usize][0] == Some(m) {
            true
        } else if self.killer_moves[ply as usize][1] == Some(m) {
            true
        } else {
            false
        })
    }

    /// Historyの更新
    ///
    /// # Arguments
    /// * `teban` - 手番
    /// * `state` - 盤面の状態
    /// * `m` - 候補手
    /// * `depth` - 現在の残り探索深さ
    /// * `ply` - 現在の探索深さ
    /// * `move_history` - 直近の手の履歴
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidInputError`] mが駒を取る手
    ///                         mの移動元に駒がない、mの移動元の駒種がteban側の駒種でない
    ///
    /// [`InvalidInputError`]: ../error/struct.InvalidInputError.html
    #[inline]
    pub fn update_improve_history(
        &mut self, teban: Teban, state: &State, m: LegalMove, depth: u32, ply: u32, move_history: &[Option<(u8,u8)>]
    ) -> Result<(),InvalidInputError> {
        if m.obtained().is_some() {
            return Err(InvalidInputError(String::from("Move that captures a piece cannot be registered in the history.")));
        }

        let to = match m {
            LegalMove::To(m) => {
                m.dst()
            },
            LegalMove::Put(m) => {
                m.dst()
            }
        };

        let bonus = depth as i32 * 6 * HISTORY_SCALE;
        let piece_index = self.calc_piece_index(teban,state,m)?;

        self.history[piece_index][to as usize] += bonus;

        self.piece_to_square[self.calc_piece_us_index(teban, piece_index)?][to as usize] += bonus;

        let next_piece = piece_index;
        let next_to = to;

        for h in move_history.iter().rev().take(1) {
            if let &Some((p,t)) = h {
                self.follow_up_history[p as usize][t as usize][self.calc_piece_us_index(teban, next_piece)?][next_to as usize] += bonus;
            }
        }

        let in_check = Rule::in_check(teban,state);

        for (i,h) in self.continuation_history.iter_mut()
            .take(ply as usize + 1).rev().skip(1)
            .take(6).enumerate() {

            if in_check && i >= 2 {
                break;
            }

            h[piece_index][to as usize] += (bonus >> i) + (i == 0) as i32 * 88;
        }

        Ok(())
    }

    /// Historyの更新
    ///
    /// # Arguments
    /// * `teban` - 手番
    /// * `state` - 盤面の状態
    /// * `m` - 候補手
    /// * `depth` - 現在の残り探索深さ
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidInputError`] mが駒を取る手
    ///                         mの移動元に駒がない、mの移動元の駒種がteban側の駒種でない
    ///
    /// [`InvalidInputError`]: ../error/struct.InvalidInputError.html
    #[inline]
    pub fn update_degrade_history(
        &mut self, teban: Teban, state: &State, m: LegalMove, depth: u32
    ) -> Result<(),InvalidInputError> {
        if m.obtained().is_some() {
            return Err(InvalidInputError(String::from("Move that captures a piece cannot be registered in the history.")));
        }

        let to = match m {
            LegalMove::To(m) => {
                m.dst()
            },
            LegalMove::Put(m) => {
                m.dst()
            }
        };

        let bonus = -(depth as i32 * 2 * HISTORY_SCALE);

        self.history[self.calc_piece_index(teban,state,m)?][to as usize] += bonus;

        Ok(())
    }

    /// Historyの参照
    ///
    /// # Arguments
    /// * `teban` - 手番
    /// * `state` - 盤面の状態
    /// * `m` - 調べる手
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidInputError`] mが駒を取る手
    ///                         mの移動元に駒がない、mの移動元の駒種がteban側の駒種でない
    ///
    /// [`InvalidInputError`]: ../error/struct.InvalidInputError.html
    #[inline]
    pub fn look_up_history(&self, teban: Teban, state: &State, m: LegalMove)
        -> Result<i32,InvalidInputError> {
        if m.obtained().is_some() {
            return Err(InvalidInputError(String::from("The move that captured the piece is not recorded in the history.")));
        }

        let to = match m {
            LegalMove::To(m) => {
                m.dst()
            },
            LegalMove::Put(m) => {
                m.dst()
            }
        };

        Ok(self.history[self.calc_piece_index(teban,state,m)?][to as usize].value())
    }

    /// Counter Moveの更新
    ///
    /// # Arguments
    /// *
    /// * `m` - 登録する候補手
    /// * `teban` - mの手番
    /// * `prev_move` - 直前に差された手
    /// * `prev_kind` - 直前に差された手の駒の種類（LegalMove::Putの場合はKomaKind::Blankを渡す）
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidInputError`] mが駒を取る手
    ///                         mが成る手である
    ///                         prev_kindとteban.opposite()の駒種が一致していない
    ///
    /// [`InvalidInputError`]: ../error/struct.InvalidInputError.html
    #[inline]
    pub fn update_counter_move(&mut self, m: LegalMove, teban: Teban, prev_move: LegalMove, prev_kind: KomaKind)
        -> Result<(),InvalidInputError> {
        if m.obtained().is_some() {
            return Err(InvalidInputError(String::from("Move that captures a piece cannot be registered in the Counter Move.")));
        }

        if let LegalMove::To(mv) = m {
            if mv.is_nari() {
                return Err(InvalidInputError(String::from("Promotion move is not included in the counter moves.")));
            }
        }

        match prev_move {
            LegalMove::To(mv) if teban == Teban::Sente => {
                if prev_kind == KomaKind::Blank {
                    return Ok(());
                }

                if prev_kind < KomaKind::GFu {
                    return Err(InvalidInputError(String::from(
                        "The previous move was made by the Gote player, but the piece type in prev_kind belongs to the Sente player."
                    )));
                }

                let index = prev_kind as usize - KomaKind::GFu as usize;

                self.counter_moves[teban.opposite() as usize][index][mv.dst() as usize] = Some(m);
            },
            LegalMove::To(mv) => {
                if prev_kind == KomaKind::Blank {
                    return Ok(());
                }

                if prev_kind >= KomaKind::GFu {
                    return Err(InvalidInputError(String::from(
                        "The previous move was made by the Sente player, but the piece type in prev_kind belongs to the Gote player."
                    )));
                }

                let index = prev_kind as usize;

                self.counter_moves[teban.opposite() as usize][index][mv.dst() as usize] = Some(m);
            },
            LegalMove::Put(mv) => {
                self.counter_moves[teban.opposite() as usize][mv.kind() as usize][mv.dst() as usize] = Some(m);
            }
        }

        Ok(())
    }

    /// 手がCounter Moveであるか判定
    ///
    /// # Arguments
    /// *
    /// * `m` - Counter Moveか判定する候補手
    /// * `teban` - mの手番
    /// * `prev_move` - 直前に差された手
    /// * `prev_kind` - 直前に差された手の駒の種類（LegalMove::Putの場合はKomaKind::Blankを渡す）
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidInputError`] prev_kindとteban.opposite()の駒種が一致していない
    ///
    /// [`InvalidInputError`]: ../error/struct.InvalidInputError.html
    #[inline]
    pub fn is_counter_move(&self, m: LegalMove, teban: Teban, prev_move: LegalMove, prev_kind: KomaKind) -> Result<bool,InvalidInputError> {
        if m.obtained().is_some() {
            return Ok(false);
        }

        if let LegalMove::To(mv) = m {
            if mv.is_nari() {
                return Ok(false);
            }
        }

        match prev_move {
            LegalMove::To(mv) if teban == Teban::Sente => {
                if prev_kind == KomaKind::Blank {
                    return Ok(false);
                }

                if prev_kind < KomaKind::GFu {
                    return Err(InvalidInputError(String::from(
                        "The previous move was made by the Gote player, but the piece type in prev_kind belongs to the Sente player."
                    )));
                }

                let index = prev_kind as usize - KomaKind::GFu as usize;

                Ok(self.counter_moves[teban.opposite() as usize][index][mv.dst() as usize] == Some(m))
            },
            LegalMove::To(mv) => {
                if prev_kind == KomaKind::Blank {
                    return Ok(false);
                }

                if prev_kind >= KomaKind::GFu {
                    return Err(InvalidInputError(String::from(
                        "The previous move was made by the Sente player, but the piece type in prev_kind belongs to the Gote player."
                    )));
                }

                let index = prev_kind as usize;

                Ok(self.counter_moves[teban.opposite() as usize][index][mv.dst() as usize] == Some(m))
            },
            LegalMove::Put(mv) => {
                Ok(self.counter_moves[teban.opposite() as usize][mv.kind() as usize][mv.dst() as usize] == Some(m))
            }
        }
    }

    /// 駒の種類をMoveOrdererで使う内部インデックスに変換する
    ///
    /// # Arguments
    /// * `teban` - 手番
    /// * `state` - 盤面の状態
    /// * `m` - 候補手
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidInputError`] mとtebanの駒種が一致していない
    ///
    /// [`InvalidInputError`]: ../error/struct.InvalidInputError.html
    #[inline]
    pub fn calc_piece_index(&self, teban: Teban, state: &State, m: LegalMove) -> Result<usize,InvalidInputError> {
        match m {
            LegalMove::To(m) => {
                if teban == Teban::Sente {
                    let (x,y) = m.src().square_to_point();
                    let kind = state.get_banmen().0[y as usize][x as usize];

                    if kind >= GFu {
                        return Err(InvalidInputError(String::from(
                            "The piece type of the piece moved during Sente's turn is Gote."
                        )));
                    } else if kind == KomaKind::Blank {
                        return Err(InvalidInputError(String::from("There are no pieces on the move origin.")));
                    }

                    Ok(if m.is_nari() {
                        kind.to_nari() as usize
                    } else {
                        kind as usize
                    })
                } else {
                    let (x,y) = m.src().square_to_point();
                    let kind = state.get_banmen().0[y as usize][x as usize];

                    if kind < GFu {
                        return Err(InvalidInputError(String::from(
                            "The piece type of the piece moved during Gote's turn is Sente."
                        )));
                    } else if kind == KomaKind::Blank {
                        return Err(InvalidInputError(String::from("There are no pieces on the move origin.")));
                    }

                    Ok(if m.is_nari() {
                        kind.to_nari() as usize
                    } else {
                        kind as usize
                    })
                }
            },
            LegalMove::Put(m) => {
                let kind = KomaKind::from((teban,m.kind()));

                Ok(kind as usize)
            }
        }
    }

    /// 駒の先手後手を区別したインデックスを手番側のインデックスに変換する
    ///
    /// # Arguments
    /// * `teban` - 手番
    /// * `kind` - 駒の種類ことの先手後手区別ありのインデックス
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidInputError`] kindとtebanの駒種が一致していない
    ///
    /// [`InvalidInputError`]: ../error/struct.InvalidInputError.html
    #[inline]
    pub fn calc_piece_us_index(&self, teban: Teban, kind: usize) -> Result<usize,InvalidInputError> {
        if kind == KomaKind::Blank as usize {
            Err(InvalidInputError(String::from("There are no pieces on the move origin.")))
        } else if teban == Teban::Sente && kind >= KomaKind::GFu as usize { Err(InvalidInputError(String::from(
                "The piece type of the piece moved during Sente's turn is Gote."
            )))
        } else if teban == Teban::Gote && kind < KomaKind::GFu as usize {
            Err(InvalidInputError(String::from(
                "The piece type of the piece moved during Gote's turn is Sente."
            )))
        } else {
            Ok(if teban == Teban::Gote {
                kind - KomaKind::GFu as usize
            } else {
                kind
            })
        }
    }

    /// 統計をクリアする関数
    pub fn clear(&mut self) {
        self.killer_moves = vec![[None; 2]; self.max_ply+1];
        self.usage_killer_moves = vec![0; self.max_ply+1];

        for h in self.history.iter_mut() {
            h.fill(StatsEntry::new(0));
        }

        for h in self.follow_up_history.iter_mut() {
            for h in h.iter_mut() {
                for h in h.iter_mut() {
                    h.fill(StatsEntry::new(0));
                }
            }
        }

        for h in self.continuation_history.iter_mut() {
            for h in h.iter_mut() {
                h.fill(StatsEntry::new(0));
            }
        }

        for h in self.piece_to_square.iter_mut() {
            h.fill(StatsEntry::new(0));
        }

        for h in self.counter_moves.iter_mut() {
            for h in h.iter_mut() {
                h.fill(None);
            }
        }
    }

    /// 探索の開始時に呼ぶ関数（goコマンド受信時）
    ///
    /// # Arguments
    /// * `ply` - 次の探索深さ
    ///
    pub fn startup(&mut self) {
        for h in self.continuation_history.iter_mut() {
            for h in h.iter_mut() {
                h.fill(StatsEntry::new(0));
            }
        }
    }
    /// 指し手を並び変える関数
    ///
    /// # Arguments
    /// * `it` - 候補手を列挙するイテレータ
    /// * `ply` - 現在の探索深さ
    /// * `teban` - 手番
    /// * `state` - 盤面の状態
    /// * `tt_move` - 置換表から取得した手
    /// * `pv` - 現在のPV
    /// * `prev_move` - 直前に差された手
    /// * `prev_kind` - 直前に差された手の駒種（LegaLMove::Putの場合はKomaKind::Blank）
    /// * `move_history` - 直近の手の履歴
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidInputError`] prev_kindとteban.opposite()の駒種が一致していない
    ///
    /// [`InvalidInputError`]: ../error/struct.InvalidInputError.html
    #[inline]
    pub fn ordering<I: Iterator<Item=LegalMove>>(
        &self, it: I, ply: u32, teban: Teban, state: &State,
        tt_move:Option<LegalMove>,pv:Option<LegalMove>,
        prev_move: Option<LegalMove>, prev_kind: KomaKind, move_history: &[Option<(u8,u8)>]
    ) -> Result<impl Iterator<Item=(LegalMove,i32)> + Clone,InvalidInputError> {
        if teban.opposite() == Sente && prev_kind >= KomaKind::GFu && prev_kind < KomaKind::Blank {
            return Err(InvalidInputError(String::from(
                "The move specified for the Sente player's turn was designated as the Gote player's move."
            )));
        } else if teban.opposite() == Gote && prev_kind < KomaKind::GFu {
            return Err(InvalidInputError(String::from(
                "The move specified for the Gote player's turn was designated as the Sente player's move."
            )))
        } else if ply as usize > self.max_ply {
            return Err(InvalidInputError(String::from(
                "ply value exceeds max_ply."
            )));
        }

        let mut mvs = vec![];

        for m in it {
            if tt_move.map(|tt_m| tt_m == m).unwrap_or(false) {
                let see = if m.obtained().is_some() {
                    calc_see(teban,state,m)
                } else {
                    0
                };

                mvs.push((MoveOrder::TT,m,see));
            } else if pv.map(|pv| pv == m).unwrap_or(false) {
                let see = if m.obtained().is_some() {
                    calc_see(teban,state,m)
                } else {
                    0
                };

                mvs.push((MoveOrder::PV,m,see));
            } else {
                match m {
                    LegalMove::To(mv) if mv.obtained().is_some() => {
                        let see = calc_see(teban,state,m);

                        if see >= 0 {
                            mvs.push((MoveOrder::GoodCaptures(see),m,see));
                        } else {
                            mvs.push((MoveOrder::BadCaptures(see),m,see));
                        }
                    },
                    _ => {
                        if Rule::is_oute_move(state,teban,m) {
                            let see = E::see(teban,state,m);

                            mvs.push((MoveOrder::Checks,m,see));
                        } else if self.killer_moves[ply as usize][0].map(|k| k == m).unwrap_or(false) {
                            let see = E::see(teban,state,m);

                            mvs.push((MoveOrder::KillerMoves(1),m,see));
                        } else if self.killer_moves[ply as usize][1].map(|k| k == m).unwrap_or(false) {
                            let see = E::see(teban,state,m);

                            mvs.push((MoveOrder::KillerMoves(0),m,see));
                        } else {
                            let to = match m {
                                LegalMove::To(m) => {
                                    m.dst()
                                },
                                LegalMove::Put(m) => {
                                    m.dst()
                                }
                            };

                            let bonus = {
                                let index = if teban.opposite() == Teban::Sente {
                                    prev_kind as usize
                                } else if prev_kind >= KomaKind::GFu && prev_kind < KomaKind::Blank {
                                    prev_kind as usize - KomaKind::GFu as usize
                                } else {
                                    14
                                };

                                if prev_kind != KomaKind::Blank {
                                    prev_move.map(|prev_move| {
                                        let dst = match prev_move {
                                            LegalMove::To(m) => {
                                                m.dst()
                                            },
                                            LegalMove::Put(m) => {
                                                m.dst()
                                            }
                                        };

                                        if self.counter_moves[teban.opposite() as usize][index][dst as usize].map(|cm| {
                                            m == cm
                                        }).unwrap_or(false) {
                                            CM_BONUS * COUNTER_MOVE_WEIGHT
                                        } else {
                                            0
                                        }
                                    }).unwrap_or(0)
                                } else {
                                    0
                                }
                            };

                            let piece_index = self.calc_piece_index(teban,state,m)?;

                            assert!(piece_index < 28);

                            let mut s = MAIN_HISTORY_WEIGHT * self.history[piece_index][to as usize].value();

                            s += PIECE_TO_SQUARE_WEIGHT * self.piece_to_square[self.calc_piece_us_index(teban, piece_index)?][to as usize].value();

                            for h in move_history.iter().rev().take(1) {
                                if let &Some((p, t)) = h {
                                    let h = self.follow_up_history[p as usize][t as usize][self.calc_piece_us_index(teban, piece_index)?][to as usize].value();

                                    s += FOLLOW_UP_WEIGHT * h;
                                }
                            }

                            for (i,h) in self.continuation_history.iter()
                                                                                     .take(ply as usize + 1)
                                                                                     .rev().skip(1)
                                                                                     .take(6).enumerate() {
                                if i == 4 {
                                    continue;
                                }
                                s += h[piece_index][to as usize].value() / (1 << 5);
                            }

                            s = s + bonus;
                            s = E::effect(teban,state,m,s);

                            let see = E::see(teban,state,m);

                            mvs.push((
                                MoveOrder::Quiet(s),
                                m,
                                see
                            ))
                        }
                    }
                }
            }
        }

        mvs.sort_by(|a,b| b.0.cmp(&a.0));

        Ok(mvs.into_iter().map(|(_,m,see)| (m,see)))
    }
}