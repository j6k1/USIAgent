//! 探索時の手の並び替えの機能を実装する

use std::fmt::Debug;
use std::marker::PhantomData;
use error::InvalidInputError;
use rule::{LegalMove, Rule, SquareToPoint, State};
use see::calc_see;
use shogi::{KomaKind, Teban};
use shogi::KomaKind::GFu;
use shogi::Teban::{Gote, Sente};

const CM_BONUS:i64 = 4800;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// 指し手の並び替え順
/// ※降順に並び変えるので優先度の低い物から列挙する
pub enum MoveOrder {
    BadCaptures(i32),
    Quiet(i64),
    KillerMoves(i32),
    Checks,
    GoodCaptures(i32),
}
pub(crate) mod private {
    use rule::{LegalMove, State};
    use shogi::Teban;

    pub trait QuietSeeEffectBase {
        fn effect(teban: Teban, state: &State, m:LegalMove, score:i64) -> i64;
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
    fn effect(_: Teban, _: &State, _: LegalMove, score:i64) -> i64 {
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
    fn effect(teban: Teban, state: &State, m: LegalMove, score: i64) -> i64 {
        (score * FACTOR as i64 + calc_see(teban, state, m) as i64) / FACTOR as i64
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
    history:[[[i64;81]; 21]; 2],
    counter_moves: [[[Option<LegalMove>;81]; 21]; 2],
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
            history: [[[0;81]; 21]; 2],
            counter_moves: [[[None;81]; 21]; 2],
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
    pub fn update_killer(&mut self, ply: usize, m: LegalMove) -> Result<(),InvalidInputError> {
        if ply > self.max_ply {
            return Err(InvalidInputError(String::from("ply value exceeds max_ply.")));
        } else if m.obtained().is_some() {
            return Err(InvalidInputError(String::from("Move that captures a piece cannot be registered in the Killer Move.")));
        }

        if let LegalMove::To(mv) = m {
            if mv.is_nari() {
                return Err(InvalidInputError(String::from("Promotion move is not included in the killer moves.")));
            }
        }

        if self.usage_killer_moves[ply] >= 1 {
            self.killer_moves[ply][1] = self.killer_moves[ply][0];
            self.killer_moves[ply][0] = Some(m);
        } else if self.usage_killer_moves[ply] == 0 {
            self.killer_moves[ply][0] = Some(m);
        }

        if self.usage_killer_moves[ply] < 2 {
            self.usage_killer_moves[ply] += 1;
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
    pub fn update_improve_history(
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

        self.history[teban as usize][self.calc_piece_index(teban,state,m)?][to as usize] += (depth * depth) as i64;

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

        self.history[teban as usize][self.calc_piece_index(teban,state,m)?][to as usize] -= depth as i64;

        Ok(())
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
    fn calc_piece_index(&self, teban: Teban, state: &State, m: LegalMove) -> Result<usize,InvalidInputError> {
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

                    Ok(kind as usize + if m.is_nari() {
                        8
                    } else {
                        0
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

                    Ok(kind as usize - KomaKind::GFu as usize + if m.is_nari() {
                        8
                    } else {
                        0
                    })
                }
            },
            LegalMove::Put(m) => {
                let kind = m.kind();

                Ok(kind as usize + 14)
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
    /// * `prev_move` - 直前に差された手
    /// * `prev_kind` - 直前に差された手の駒種（LegaLMove::Putの場合はKomaKind::Blank）
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidInputError`] prev_kindとteban.opposite()の駒種が一致していない
    ///
    /// [`InvalidInputError`]: ../error/struct.InvalidInputError.html
    #[inline]
    pub fn ordering<I: Iterator<Item=LegalMove>>(
        &self, it: I, ply: u32, teban: Teban, state: &State, prev_move: Option<LegalMove>, prev_kind: KomaKind
    ) -> Result<impl Iterator<Item=LegalMove>,InvalidInputError> {
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
            match m {
                LegalMove::To(mv) if mv.obtained().is_some() => {
                    let see = calc_see(teban,state,m);

                    if see >= 0 {
                        mvs.push((MoveOrder::GoodCaptures(see),m));
                    } else {
                        mvs.push((MoveOrder::BadCaptures(see),m));
                    }
                },
                _ => {
                    if Rule::is_oute_move(state,teban,m) {
                        mvs.push((MoveOrder::Checks,m));
                    } else if self.usage_killer_moves[ply as usize] > 0 &&
                        (self.killer_moves[ply as usize][0].map(|k| k == m).unwrap_or(false) ||
                            self.killer_moves[ply as usize][1].map(|k| k == m).unwrap_or(false)) {
                        let see = E::see(teban,state,m);

                        mvs.push((MoveOrder::KillerMoves(see),m));
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
                                21
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
                                        CM_BONUS
                                    } else {
                                        0
                                    }
                                }).unwrap_or(0)
                            } else {
                                0
                            }
                        };

                        let s = self.history[teban as usize][self.calc_piece_index(teban, state, m)?][to as usize] + bonus;
                        let s = E::effect(teban,state,m,s);
                        mvs.push((
                            MoveOrder::Quiet(s),
                            m
                        ))
                    }
                }
            }
        }

        mvs.sort_by(|a,b| b.0.cmp(&a.0));

        Ok(mvs.into_iter().map(|(_,m)| m))
    }
}