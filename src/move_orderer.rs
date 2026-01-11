use rule::{LegalMove, SquareToPoint, State};
use see::calc_see;
use shogi::{KomaKind, Teban};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// 指し手の並び替え順
/// ※降順に並び変えるので優先度の低い物から列挙する
pub enum MoveOrder {
    History(i64),
    KillerMoves,
    GoodCaptures(i32),
}
/// 指し手並び変え機の実装
pub struct MoveOrderer {
    killer_moves:Vec<[Option<LegalMove>; 2]>,
    usage_killer_moves:Vec<u8>,
    history:[[[i64;81]; 21]; 2],
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
            history: [[[0;81]; 21]; 2],
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
    }

    /// Historyの更新
    ///
    /// # Arguments
    /// * `teban` - 手番
    /// * `state` - 盤面の状態
    /// * `m` - 候補手
    /// * `depth` - 現在の探索深さ
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
    /// * `depth` - 現在の探索深さ
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

                    kind as usize
                } else {
                    let (x,y) = m.src().square_to_point();
                    let kind = state.get_banmen().0[y as usize][x as usize];

                    kind as usize - KomaKind::GFu as usize
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
    #[inline]
    pub fn ordering<I: Iterator<Item=LegalMove>>(&self, it: I, ply: u32, teban: Teban, state: &State) -> impl Iterator<Item=LegalMove> {
        let mut mvs = vec![];

        for m in it {
            match m {
                LegalMove::To(mv) if mv.obtained().is_some() => {
                    let see = calc_see(teban,state,m);

                    mvs.push((MoveOrder::GoodCaptures(see),m));
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

                        mvs.push((
                            MoveOrder::History(self.history[teban as usize][self.calc_piece_index(teban,state,m)][to as usize]),
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