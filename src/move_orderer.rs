//! 探索時の手の並び替えの機能を実装する
use rule::{LegalMove, SquareToPoint, State};
use see::calc_see;
use shogi::{KomaKind, Teban};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// 指し手の並び替え順
/// ※降順に並び変えるので優先度の低い物から列挙する
pub enum MoveOrder {
    BadCaptures(i32),
    Quiet(i64),
    KillerMoves,
    GoodCaptures(i32),
}
/// 指し手並び変え機の実装
#[derive(Debug,Clone)]
pub struct MoveOrderer {
    killer_moves:Vec<[Option<LegalMove>; 2]>,
    usage_killer_moves:Vec<u8>,
    history:[[[i64;81]; 22]; 2],
    counter_moves: [[[Option<LegalMove>;81]; 22]; 2]
}
impl MoveOrderer {
    /// MoveOrdererのインスタンスを生成するコンストラクタ
    ///
    /// # Arguments
    /// * `max_ply` - 現在の最大探索深さ
    #[inline]
    pub fn new(max_ply: usize) -> MoveOrderer {
        MoveOrderer {
            killer_moves: vec![[None; 2]; max_ply+1],
            usage_killer_moves: vec![0; max_ply+1],
            history: [[[0;81]; 22]; 2],
            counter_moves: [[[None;81]; 22]; 2]
        }
    }

    /// Killer Moveの更新
    ///
    /// # Arguments
    /// * `ply` - 現在の探索深さ
    /// * `m` - 登録する候補手
    #[inline]
    pub fn update_killer(&mut self, ply: usize, m: LegalMove) {
        if self.usage_killer_moves[ply] >= 1 {
            self.killer_moves[ply][1] = self.killer_moves[ply][0];
            self.killer_moves[ply][0] = Some(m);
        } else if self.usage_killer_moves[ply] == 0 {
            self.killer_moves[ply][0] = Some(m);
        }

        if self.usage_killer_moves[ply] < 2 {
            self.usage_killer_moves[ply] += 1;
        }
    }

    /// Historyの更新
    ///
    /// # Arguments
    /// * `teban` - 手番
    /// * `state` - 盤面の状態
    /// * `m` - 候補手
    /// * `depth` - 現在の残り探索深さ
    #[inline]
    pub fn update_improve_history(
        &mut self, teban: Teban, state: &State, m: LegalMove, depth: u32
    ) {
        let to = match m {
            LegalMove::To(m) => {
                m.dst()
            },
            LegalMove::Put(m) => {
                m.dst()
            }
        };

        self.history[teban as usize][self.calc_piece_index(teban,state,m)][to as usize] += (depth * depth) as i64;
    }

    /// Historyの更新
    ///
    /// # Arguments
    /// * `teban` - 手番
    /// * `state` - 盤面の状態
    /// * `m` - 候補手
    /// * `depth` - 現在の残り探索深さ
    #[inline]
    pub fn update_degrade_history(
        &mut self, teban: Teban, state: &State, m: LegalMove, depth: u32
    ) {
        let to = match m {
            LegalMove::To(m) => {
                m.dst()
            },
            LegalMove::Put(m) => {
                m.dst()
            }
        };

        self.history[teban as usize][self.calc_piece_index(teban,state,m)][to as usize] -= depth as i64;
    }

    /// Counter Moveの更新
    ///
    /// # Arguments
    /// *
    /// * `m` - 登録する候補手
    /// * `teban` - 手の手番
    /// * `kind` - 駒の種類（LegalMove::Putの場合はKomaKind::Blankを渡す）
    #[inline]
    pub fn update_counter_move(&mut self, m: LegalMove, teban: Teban, kind: KomaKind) {
        match m {
            LegalMove::To(mv) if teban == Teban::Sente => {
                let index = if kind == KomaKind::Blank {
                    21
                } else {
                    kind as usize
                };

                self.counter_moves[teban as usize][index][mv.dst() as usize] = Some(m);
            },
            LegalMove::To(mv) => {
                let index = if kind == KomaKind::Blank {
                    21
                } else {
                    kind as usize - KomaKind::GFu as usize
                };

                self.counter_moves[teban as usize][index][mv.dst() as usize] = Some(m);
            },
            LegalMove::Put(mv) => {
                self.counter_moves[teban as usize][mv.kind() as usize][mv.dst() as usize] = Some(m);
            }
        }
    }

    /// 駒の種類をMoveOrdererで使う内部インデックスに変換する
    ///
    /// # Arguments
    /// * `teban` - 手番
    /// * `state` - 盤面の状態
    /// * `m` - 候補手
    #[inline]
    fn calc_piece_index(&self, teban: Teban, state: &State, m: LegalMove) -> usize {
        match m {
            LegalMove::To(m) => {
                if teban == Teban::Sente {
                    let (x,y) = m.src().square_to_point();
                    let kind = state.get_banmen().0[y as usize][x as usize];

                    if kind == KomaKind::Blank {
                        21
                    } else {
                        kind as usize + if m.is_nari() {
                            8
                        } else {
                            0
                        }
                    }
                } else {
                    let (x,y) = m.src().square_to_point();
                    let kind = state.get_banmen().0[y as usize][x as usize];

                    if kind == KomaKind::Blank {
                        21
                    } else {
                        kind as usize - KomaKind::GFu as usize + if m.is_nari() {
                            8
                        } else {
                            0
                        }
                    }
                }
            },
            LegalMove::Put(m) => {
                let kind = m.kind();

                kind as usize + 14
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
    #[inline]
    pub fn ordering<I: Iterator<Item=LegalMove>>(
        &self, it: I, ply: u32, teban: Teban, state: &State, prev_move: Option<LegalMove>, prev_kind: KomaKind
    ) -> impl Iterator<Item=LegalMove> {
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
                    if self.usage_killer_moves[ply as usize] > 0 &&
                        (self.killer_moves[ply as usize][0].map(|k| k == m).unwrap_or(false) ||
                            self.killer_moves[ply as usize][1].map(|k| k == m).unwrap_or(false)) {
                        mvs.push((MoveOrder::KillerMoves,m));
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
                            let index = if teban == Teban::Sente {
                                if prev_kind == KomaKind::Blank {
                                    21
                                } else {
                                    prev_kind as usize
                                }
                            } else {
                                if prev_kind == KomaKind::Blank {
                                    21
                                } else {
                                    prev_kind as usize - KomaKind::GFu as usize
                                }
                            };

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
                                    8000
                                } else {
                                    0
                                }
                            }).unwrap_or(0)
                        };

                        mvs.push((
                            MoveOrder::Quiet(self.history[teban as usize][self.calc_piece_index(teban, state, m)][to as usize] + bonus),
                            m
                        ))
                    }
                }
            }
        }

        mvs.sort_by(|a,b| b.0.cmp(&a.0));

        mvs.into_iter().map(|(_,m)| m)
    }
}