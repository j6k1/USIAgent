//! 巻き戻し可能な局面情報の実装

use std::mem::MaybeUninit;
use bitboard::BitBoard;
use error::InvalidStateError;
use rule::{LegalMove, Rule, SquareToPoint, State};
use shogi::{KomaKind, MochigomaCollections, Teban};

#[derive(Debug,Clone,Copy)]
struct UndoItem {
    /// 最後に適用された手
    pub mv:LegalMove,
    /// 最後に手が適用された時の手番
    pub teban:Teban,
    /// 最後に手が適用される直前の持ち駒の状態
    pub mc:MochigomaCollections,
    /// 最後に手が適用された時の移動元の駒種
    pub from_kind:KomaKind,
    /// 最後に手が適用される直前に移動先にいた駒の駒種
    pub before_to_kind:KomaKind,
    /// 先手視点の先手側の駒のビットボード
    pub sente_self_board:BitBoard,
    /// 先手視点の後手側の駒のビットボード
    pub sente_opponent_board:BitBoard,
    /// 後手視点の後手側の駒のビットボード
    pub gote_self_board:BitBoard,
    /// 後手視点の先手側の駒のビットボード
    pub gote_opponent_board:BitBoard,
    /// 先手側の飛車角香が効いていてその位置にある駒が動くことで素抜けが発生する可能性のある駒の位置のビットボード
    pub sente_pin_board:BitBoard,
    /// 後手側の飛車角香が効いていてその位置にある駒が動くことで素抜けが発生する可能性のある駒の位置のビットボード
    pub gote_pin_board:BitBoard,
    /// 先手側の駒のうち、後手側の玉に王手をかけている駒の位置のビットボード
    pub sente_checked_board:BitBoard,
    /// 後手側の駒のうち、先手側の王に王手をかけている駒の位置のビットボード
    pub gote_checked_board:BitBoard,
}
/// 手を適用した後適用前の状態に巻き戻し可能な盤面の状態
#[derive(Debug,Clone)]
pub struct Position<const N: usize> {
    state:State,
    mc:MochigomaCollections,
    undo_items:[MaybeUninit<UndoItem>;N],
    curernt_index:usize
}
impl<const N: usize> Position<N> {
    /// `Position`の生成
    ///
    /// # Arguments
    /// * `state` - 盤面の初期状態
    /// * `mc` - 持ち駒の初期状態
    #[inline]
    pub fn new(state:State,mc:MochigomaCollections) -> Position<N> {
        Position {
            state,
            mc,
            undo_items:[MaybeUninit::uninit(); N],
            curernt_index:0
        }
    }

    /// 現在の盤面の状態への参照を返す
    #[inline]
    pub fn get_state(&self) -> &State {
        &self.state
    }

    /// 現在の持ち駒の状態への参照を返す
    #[inline]
    pub fn get_mc(&self) -> &MochigomaCollections {
        &self.mc
    }

    /// 手を適用する
    ///
    /// # Arguments
    /// * `teban` - 手を列挙したい手番
    /// * `mv` - 適用する手
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidStateError`] Undoスタックのサイズを超えて手を適用しようとした
    ///
    /// [`InvalidStateError`]: ../error/struct.LimitSizeError.html
    #[inline]
    pub fn apply_move(&mut self, teban: Teban,mv: LegalMove) -> Result<(),InvalidStateError> {
        if self.curernt_index >= N {
            return Err(InvalidStateError(String::from("Undo stack overflow")));
        }

        let mc = self.mc;

        let from_kind = match mv {
            LegalMove::To(m) => {
                let (sx,sy) = m.src().square_to_point();

                self.state.get_banmen()[sy as usize][sx as usize]
            },
            LegalMove::Put(_) => KomaKind::Blank,
        };

        let (dx,dy) = mv.dst().square_to_point();

        let before_to_kind = self.state.get_banmen()[dy as usize][dx as usize];

        let sente_self_board = self.state.part.sente_self_board;
        let sente_opponent_board = self.state.part.sente_opponent_board;
        let gote_self_board = self.state.part.gote_self_board;
        let gote_opponent_board = self.state.part.gote_opponent_board;

        let sente_pin_board = self.state.part.sente_pin_board;
        let gote_pin_board = self.state.part.gote_pin_board;
        let sente_checked_board = self.state.part.sente_checked_board;
        let gote_checked_board = self.state.part.gote_checked_board;

        Rule::apply_move_to_partial_state_none_check_inplace(&mut self.state,teban,&self.mc,mv.to_applied_move());
        Rule::apply_move_to_banmen_and_mochigoma_none_check_inplace(&mut self.state.banmen,teban,&mut self.mc,mv.to_applied_move());

        let undo_item = UndoItem {
            mv,
            teban,
            mc,
            from_kind,
            before_to_kind,
            sente_pin_board,
            gote_pin_board,
            sente_checked_board,
            gote_checked_board,
            sente_self_board,
            sente_opponent_board,
            gote_self_board,
            gote_opponent_board
        };

        unsafe {
            self.undo_items.get_unchecked_mut(self.curernt_index).write(undo_item);
        }

        self.curernt_index += 1;

        Ok(())
    }

    /// 盤面と持ち駒の状態を直前に適用された手が適用される前の状態に巻き戻す
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidStateError`] Undoスタックが空(盤面は既に初期状態)なのに巻き戻そうとした
    ///
    /// [`InvalidStateError`]: ../error/struct.LimitSizeError.html
    #[inline]
    pub fn undo_move(&mut self) -> Result<(),InvalidStateError> {
        if self.curernt_index == 0 {
            return Err(InvalidStateError(String::from("The undo stack is empty.")));
        }

        self.curernt_index -= 1;

        let undo_item = unsafe {
            self.undo_items.get_unchecked_mut(self.curernt_index).assume_init()
        };

        if undo_item.teban == Teban::Sente {
            match undo_item.mv {
                LegalMove::To(m) => {
                    let to = m.dst();
                    let inverse_to = 80 - to;
                    let from = m.src();
                    let (dx,dy) = to.square_to_point();
                    let (sx,sy) = from.square_to_point();

                    let to_kind = self.state.get_banmen()[dy as usize][dx as usize];

                    let from_kind = undo_item.from_kind;

                    let before_to_kind = undo_item.before_to_kind;

                    self.state.part.sente_nari_board ^= (to_kind.is_nari() as u128) << (to + 1);

                    self.state.part.sente_control_superposition -= Rule::gen_control_bits(to, to_kind);
                    self.state.part.sente_control_superposition += Rule::gen_control_bits(from, from_kind);

                    if before_to_kind != KomaKind::Blank {
                        self.state.part.gote_nari_board ^= (before_to_kind.is_nari() as u128) << (to + 1);
                        self.state.part.gote_control_superposition += Rule::gen_control_bits(inverse_to, before_to_kind);
                    }

                    self.state.part.sente_nari_board ^= (from_kind.is_nari() as u128) << (from + 1);

                    for (kind,p) in [(to_kind,to),(from_kind,from),(before_to_kind,to)] {
                        let mask = if kind == KomaKind::SOu {
                            1 << (80 - p + 1)
                        } else {
                            1 << (p + 1)
                        };

                        match kind {
                            KomaKind::SFu => {
                                self.state.part.sente_fu_board ^= mask;
                            },
                            KomaKind::SKyou => {
                                self.state.part.sente_kyou_board ^= mask;
                            },
                            KomaKind::SKei => {
                                self.state.part.sente_kei_board ^= mask;
                            },
                            KomaKind::SGin => {
                                self.state.part.sente_gin_board ^= mask;
                            },
                            KomaKind::SKin => {
                                self.state.part.sente_kin_board ^= mask;
                            },
                            KomaKind::SKaku => {
                                self.state.part.sente_kaku_board ^= mask;
                            },
                            KomaKind::SHisha => {
                                self.state.part.sente_hisha_board ^= mask;
                            },
                            KomaKind::SOu => {
                                self.state.part.gote_opponent_ou_position_board ^= mask;
                            },
                            KomaKind::SFuN => {
                                self.state.part.sente_fu_board ^= mask;
                            },
                            KomaKind::SKyouN => {
                                self.state.part.sente_kyou_board ^= mask;
                            },
                            KomaKind::SKeiN => {
                                self.state.part.sente_kei_board ^= mask;
                            },
                            KomaKind::SGinN => {
                                self.state.part.sente_gin_board ^= mask;
                            },
                            KomaKind::SKakuN => {
                                self.state.part.sente_kaku_board ^= mask;
                            },
                            KomaKind::SHishaN => {
                                self.state.part.sente_hisha_board ^= mask;
                            },
                            KomaKind::GFu => {
                                self.state.part.gote_fu_board ^= mask;
                            },
                            KomaKind::GKyou => {
                                self.state.part.gote_kyou_board ^= mask;
                            },
                            KomaKind::GKei => {
                                self.state.part.gote_kei_board ^= mask;
                            },
                            KomaKind::GGin => {
                                self.state.part.gote_gin_board ^= mask;
                            },
                            KomaKind::GKin => {
                                self.state.part.gote_kin_board ^= mask;
                            },
                            KomaKind::GKaku => {
                                self.state.part.gote_kaku_board ^= mask;
                            },
                            KomaKind::GHisha => {
                                self.state.part.gote_hisha_board ^= mask;
                            },
                            KomaKind::GOu => {
                                self.state.part.sente_opponent_ou_position_board ^= mask;
                            },
                            KomaKind::GFuN => {
                                self.state.part.gote_fu_board ^= mask;
                            },
                            KomaKind::GKyouN => {
                                self.state.part.gote_kyou_board ^= mask;
                            },
                            KomaKind::GKeiN => {
                                self.state.part.gote_kei_board ^= mask;
                            },
                            KomaKind::GGinN => {
                                self.state.part.gote_gin_board ^= mask;
                            },
                            KomaKind::GKakuN => {
                                self.state.part.gote_kaku_board ^= mask;
                            },
                            KomaKind::GHishaN => {
                                self.state.part.gote_hisha_board ^= mask;
                            },
                            KomaKind::Blank => {}
                        }
                    }

                    self.state.part.sente_control_board = self.state.part.sente_control_superposition.to_bitboard();
                    self.state.part.gote_control_board = self.state.part.gote_control_superposition.to_bitboard();

                    self.state.banmen[dy as usize][dx as usize] = before_to_kind;
                    self.state.banmen[sy as usize][sx as usize] = from_kind;

                    self.mc = undo_item.mc;
                },
                LegalMove::Put(m) => {
                    let p = m.dst();
                    let (dx,dy) = p.square_to_point();

                    let kind = self.state.get_banmen()[dy as usize][dx as usize];

                    let mask = if kind == KomaKind::SOu {
                        1 << (80 - p + 1)
                    } else {
                        1 << (p + 1)
                    };

                    match kind {
                        KomaKind::SFu => {
                            self.state.part.sente_fu_board ^= mask;
                        },
                        KomaKind::SKyou => {
                            self.state.part.sente_kyou_board ^= mask;
                        },
                        KomaKind::SKei => {
                            self.state.part.sente_kei_board ^= mask;
                        },
                        KomaKind::SGin => {
                            self.state.part.sente_gin_board ^= mask;
                        },
                        KomaKind::SKin => {
                            self.state.part.sente_kin_board ^= mask;
                        },
                        KomaKind::SKaku => {
                            self.state.part.sente_kaku_board ^= mask;
                        },
                        KomaKind::SHisha => {
                            self.state.part.sente_hisha_board ^= mask;
                        },
                        KomaKind::SOu => {
                            self.state.part.gote_opponent_ou_position_board ^= mask;
                        },
                        KomaKind::SFuN => {
                            self.state.part.sente_fu_board ^= mask;
                        },
                        KomaKind::SKyouN => {
                            self.state.part.sente_kyou_board ^= mask;
                        },
                        KomaKind::SKeiN => {
                            self.state.part.sente_kei_board ^= mask;
                        },
                        KomaKind::SGinN => {
                            self.state.part.sente_gin_board ^= mask;
                        },
                        KomaKind::SKakuN => {
                            self.state.part.sente_kaku_board ^= mask;
                        },
                        KomaKind::SHishaN => {
                            self.state.part.sente_hisha_board ^= mask;
                        },
                        KomaKind::GFu => {
                            self.state.part.gote_fu_board ^= mask;
                        },
                        KomaKind::GKyou => {
                            self.state.part.gote_kyou_board ^= mask;
                        },
                        KomaKind::GKei => {
                            self.state.part.gote_kei_board ^= mask;
                        },
                        KomaKind::GGin => {
                            self.state.part.gote_gin_board ^= mask;
                        },
                        KomaKind::GKin => {
                            self.state.part.gote_kin_board ^= mask;
                        },
                        KomaKind::GKaku => {
                            self.state.part.gote_kaku_board ^= mask;
                        },
                        KomaKind::GHisha => {
                            self.state.part.gote_hisha_board ^= mask;
                        },
                        KomaKind::GOu => {
                            self.state.part.sente_opponent_ou_position_board ^= mask;
                        },
                        KomaKind::GFuN => {
                            self.state.part.gote_fu_board ^= mask;
                        },
                        KomaKind::GKyouN => {
                            self.state.part.gote_kyou_board ^= mask;
                        },
                        KomaKind::GKeiN => {
                            self.state.part.gote_kei_board ^= mask;
                        },
                        KomaKind::GGinN => {
                            self.state.part.gote_gin_board ^= mask;
                        },
                        KomaKind::GKakuN => {
                            self.state.part.gote_kaku_board ^= mask;
                        },
                        KomaKind::GHishaN => {
                            self.state.part.gote_hisha_board ^= mask;
                        },
                        KomaKind::Blank => {}
                    }

                    self.state.banmen[dy as usize][dx as usize] = KomaKind::Blank;

                    self.mc = undo_item.mc;

                    self.state.part.sente_control_superposition -= Rule::gen_control_bits(p, kind);

                    self.state.part.sente_control_board = self.state.part.sente_control_superposition.to_bitboard();
                    self.state.part.gote_control_board = self.state.part.gote_control_superposition.to_bitboard();
                }
            }
        } else {
            match undo_item.mv {
                LegalMove::To(m) => {
                    let to = m.dst();
                    let inverse_to = 80 - to;
                    let from = m.src();
                    let inverse_from = 80 - from;
                    let (dx,dy) = to.square_to_point();
                    let (sx,sy) = from.square_to_point();

                    let to_kind = self.state.get_banmen()[dy as usize][dx as usize];

                    let from_kind = undo_item.from_kind;

                    let before_to_kind = undo_item.before_to_kind;

                    self.state.part.gote_nari_board ^= (to_kind.is_nari() as u128) << (to + 1);

                    self.state.part.gote_control_superposition -= Rule::gen_control_bits(inverse_to,to_kind);
                    self.state.part.gote_control_superposition += Rule::gen_control_bits(inverse_from, from_kind);

                    if before_to_kind != KomaKind::Blank {
                        self.state.part.sente_nari_board ^= (before_to_kind.is_nari() as u128) << (to + 1);
                        self.state.part.sente_control_superposition += Rule::gen_control_bits(to, before_to_kind);
                    }

                    self.state.part.gote_nari_board ^= (from_kind.is_nari() as u128) << (from + 1);

                    for (kind,p) in [(to_kind,to),(from_kind,from),(before_to_kind,to)] {
                        let mask = if kind == KomaKind::SOu {
                            1 << (80 - p + 1)
                        } else {
                            1 << (p + 1)
                        };

                        match kind {
                            KomaKind::SFu => {
                                self.state.part.sente_fu_board ^= mask;
                            },
                            KomaKind::SKyou => {
                                self.state.part.sente_kyou_board ^= mask;
                            },
                            KomaKind::SKei => {
                                self.state.part.sente_kei_board ^= mask;
                            },
                            KomaKind::SGin => {
                                self.state.part.sente_gin_board ^= mask;
                            },
                            KomaKind::SKin => {
                                self.state.part.sente_kin_board ^= mask;
                            },
                            KomaKind::SKaku => {
                                self.state.part.sente_kaku_board ^= mask;
                            },
                            KomaKind::SHisha => {
                                self.state.part.sente_hisha_board ^= mask;
                            },
                            KomaKind::SOu => {
                                self.state.part.gote_opponent_ou_position_board ^= mask;
                            },
                            KomaKind::SFuN => {
                                self.state.part.sente_fu_board ^= mask;
                            },
                            KomaKind::SKyouN => {
                                self.state.part.sente_kyou_board ^= mask;
                            },
                            KomaKind::SKeiN => {
                                self.state.part.sente_kei_board ^= mask;
                            },
                            KomaKind::SGinN => {
                                self.state.part.sente_gin_board ^= mask;
                            },
                            KomaKind::SKakuN => {
                                self.state.part.sente_kaku_board ^= mask;
                            },
                            KomaKind::SHishaN => {
                                self.state.part.sente_hisha_board ^= mask;
                            },
                            KomaKind::GFu => {
                                self.state.part.gote_fu_board ^= mask;
                            },
                            KomaKind::GKyou => {
                                self.state.part.gote_kyou_board ^= mask;
                            },
                            KomaKind::GKei => {
                                self.state.part.gote_kei_board ^= mask;
                            },
                            KomaKind::GGin => {
                                self.state.part.gote_gin_board ^= mask;
                            },
                            KomaKind::GKin => {
                                self.state.part.gote_kin_board ^= mask;
                            },
                            KomaKind::GKaku => {
                                self.state.part.gote_kaku_board ^= mask;
                            },
                            KomaKind::GHisha => {
                                self.state.part.gote_hisha_board ^= mask;
                            },
                            KomaKind::GOu => {
                                self.state.part.sente_opponent_ou_position_board ^= mask;
                            },
                            KomaKind::GFuN => {
                                self.state.part.gote_fu_board ^= mask;
                            },
                            KomaKind::GKyouN => {
                                self.state.part.gote_kyou_board ^= mask;
                            },
                            KomaKind::GKeiN => {
                                self.state.part.gote_kei_board ^= mask;
                            },
                            KomaKind::GGinN => {
                                self.state.part.gote_gin_board ^= mask;
                            },
                            KomaKind::GKakuN => {
                                self.state.part.gote_kaku_board ^= mask;
                            },
                            KomaKind::GHishaN => {
                                self.state.part.gote_hisha_board ^= mask;
                            },
                            KomaKind::Blank => {}
                        }
                    }

                    self.state.part.sente_control_board = self.state.part.sente_control_superposition.to_bitboard();
                    self.state.part.gote_control_board = self.state.part.gote_control_superposition.to_bitboard();

                    self.state.banmen[dy as usize][dx as usize] = before_to_kind;
                    self.state.banmen[sy as usize][sx as usize] = from_kind;

                    self.mc = undo_item.mc;
                },
                LegalMove::Put(m) => {
                    let p = m.dst();
                    let (dx,dy) = p.square_to_point();

                    let kind = self.state.get_banmen()[dy as usize][dx as usize];

                    let mask = if kind == KomaKind::SOu {
                        1 << (80 - p + 1)
                    } else {
                        1 << (p + 1)
                    };

                    match kind {
                        KomaKind::SFu => {
                            self.state.part.sente_fu_board ^= mask;
                        },
                        KomaKind::SKyou => {
                            self.state.part.sente_kyou_board ^= mask;
                        },
                        KomaKind::SKei => {
                            self.state.part.sente_kei_board ^= mask;
                        },
                        KomaKind::SGin => {
                            self.state.part.sente_gin_board ^= mask;
                        },
                        KomaKind::SKin => {
                            self.state.part.sente_kin_board ^= mask;
                        },
                        KomaKind::SKaku => {
                            self.state.part.sente_kaku_board ^= mask;
                        },
                        KomaKind::SHisha => {
                            self.state.part.sente_hisha_board ^= mask;
                        },
                        KomaKind::SOu => {
                            self.state.part.gote_opponent_ou_position_board ^= mask;
                        },
                        KomaKind::SFuN => {
                            self.state.part.sente_fu_board ^= mask;
                        },
                        KomaKind::SKyouN => {
                            self.state.part.sente_kyou_board ^= mask;
                        },
                        KomaKind::SKeiN => {
                            self.state.part.sente_kei_board ^= mask;
                        },
                        KomaKind::SGinN => {
                            self.state.part.sente_gin_board ^= mask;
                        },
                        KomaKind::SKakuN => {
                            self.state.part.sente_kaku_board ^= mask;
                        },
                        KomaKind::SHishaN => {
                            self.state.part.sente_hisha_board ^= mask;
                        },
                        KomaKind::GFu => {
                            self.state.part.gote_fu_board ^= mask;
                        },
                        KomaKind::GKyou => {
                            self.state.part.gote_kyou_board ^= mask;
                        },
                        KomaKind::GKei => {
                            self.state.part.gote_kei_board ^= mask;
                        },
                        KomaKind::GGin => {
                            self.state.part.gote_gin_board ^= mask;
                        },
                        KomaKind::GKin => {
                            self.state.part.gote_kin_board ^= mask;
                        },
                        KomaKind::GKaku => {
                            self.state.part.gote_kaku_board ^= mask;
                        },
                        KomaKind::GHisha => {
                            self.state.part.gote_hisha_board ^= mask;
                        },
                        KomaKind::GOu => {
                            self.state.part.sente_opponent_ou_position_board ^= mask;
                        },
                        KomaKind::GFuN => {
                            self.state.part.gote_fu_board ^= mask;
                        },
                        KomaKind::GKyouN => {
                            self.state.part.gote_kyou_board ^= mask;
                        },
                        KomaKind::GKeiN => {
                            self.state.part.gote_kei_board ^= mask;
                        },
                        KomaKind::GGinN => {
                            self.state.part.gote_gin_board ^= mask;
                        },
                        KomaKind::GKakuN => {
                            self.state.part.gote_kaku_board ^= mask;
                        },
                        KomaKind::GHishaN => {
                            self.state.part.gote_hisha_board ^= mask;
                        },
                        KomaKind::Blank => {}
                    }

                    self.state.banmen[dy as usize][dx as usize] = KomaKind::Blank;

                    self.mc = undo_item.mc;

                    self.state.part.gote_control_superposition -= Rule::gen_control_bits(80 - p, kind);

                    self.state.part.sente_control_board = self.state.part.sente_control_superposition.to_bitboard();
                    self.state.part.gote_control_board = self.state.part.gote_control_superposition.to_bitboard();
                }
            }
        }

        self.state.part.sente_self_board = undo_item.sente_self_board;
        self.state.part.sente_opponent_board = undo_item.sente_opponent_board;
        self.state.part.gote_self_board = undo_item.gote_self_board;
        self.state.part.gote_opponent_board = undo_item.gote_opponent_board;
        self.state.part.sente_pin_board = undo_item.sente_pin_board;
        self.state.part.gote_pin_board = undo_item.gote_pin_board;
        self.state.part.sente_checked_board = undo_item.sente_checked_board;
        self.state.part.gote_checked_board = undo_item.gote_checked_board;

        Ok(())
    }

    /// 局面の状態を一番最初の時点まで巻き戻す。Undoスタックも空に戻る。
    ///
    /// # Errors
    ///
    /// この関数は以下のエラーを返すケースがあります。
    /// * [`InvalidStateError`] 局面の巻き戻し中にエラーが発生した(現状の実装では実際には発生しない)
    ///
    /// [`InvalidStateError`]: ../error/struct.LimitSizeError.html
    #[inline]
    pub fn rewind(&mut self) -> Result<(),InvalidStateError> {
        while self.curernt_index > 0 {
            self.undo_move()?;
        }

        Ok(())
    }
}