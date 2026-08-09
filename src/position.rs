//! 巻き戻し可能な局面情報の実装

use std::convert::TryFrom;
use error::InvalidStateError;
use rule::{LegalMove, Rule, SquareToPoint, State};
use shogi::{KomaKind, Mochigoma, MochigomaCollections, MochigomaKind, Teban};

#[derive(Debug,Clone)]
struct UndoItem {
    /// 最後に適用された手
    pub mv:LegalMove,
    /// 最後に手が適用された時の手番
    pub teban:Teban,
    /// 先手のpin_boardをxorで巻き戻すためのビットの位置のリスト
    pub sente_pin_changed:Vec<u8>,
    /// 後手のpin_boardをxorで巻き戻すためのビットの位置のリスト
    pub gote_pin_changed:Vec<u8>,
    /// 先手のcheck_boardをxorで巻き戻すためのビットの位置のリスト
    pub sente_checked_changed:Vec<u8>,
    /// 後手のcheck_boardをxorで巻き戻すためのビットの位置のリスト
    pub gote_checked_changed:Vec<u8>
}
#[derive(Debug,Clone)]
pub struct Position {
    state:State,
    mc:MochigomaCollections,
    undo_items:Vec<UndoItem>,
}
impl Position {
    pub fn new(state:State,mc:MochigomaCollections)->Self{
        Self {
            state,
            mc,
            undo_items:Vec::new(),
        }
    }

    pub fn apply_move(&mut self, teban: Teban,mv: LegalMove) {
        let sente_pin_board = self.state.part.sente_pin_board;
        let gote_pin_board = self.state.part.gote_pin_board;
        let sente_checked_board = self.state.part.sente_checked_board;
        let gote_checked_board = self.state.part.gote_checked_board;

        Rule::apply_move_to_banmen_and_mochigoma_none_check_inplace(&mut self.state.banmen,teban,&mut self.mc,mv.to_applied_move());
        Rule::apply_move_to_partial_state_none_check_inplace(&mut self.state,teban,&self.mc,mv.to_applied_move());

        let mut sente_pin_changed = Vec::with_capacity(1);

        for p in (sente_pin_board ^ self.state.part.sente_pin_board).iter() {
            sente_pin_changed.push(p as u8);
        }

        let mut gote_pin_changed = Vec::with_capacity(1);

        for p in (gote_pin_board ^ self.state.part.gote_pin_board).iter() {
            gote_pin_changed.push(p as u8);
        }

        let mut sente_checked_changed = Vec::with_capacity(2);

        for p in (sente_checked_board ^ self.state.part.sente_checked_board).iter() {
            sente_checked_changed.push(p as u8);
        }

        let mut gote_checked_changed = Vec::with_capacity(2);

        for p in (gote_checked_board ^ self.state.part.gote_checked_board).iter() {
            gote_checked_changed.push(p as u8);
        }

        let undo_item = UndoItem {
            mv,
            teban,
            sente_pin_changed,
            gote_pin_changed,
            sente_checked_changed,
            gote_checked_changed,
        };

        self.undo_items.push(undo_item);
    }

    pub fn undo_move(&mut self) -> Result<(),InvalidStateError>{
        if let Some(undo_item) = self.undo_items.pop() {
            match undo_item.mv {
                LegalMove::To(m) => {
                    let to = m.dst();
                    let inverse_to = 80 - to;
                    let from = m.src();
                    let inverse_from = 80 - from;
                    let (sx,sy) = to.square_to_point();
                    let (dx,dy) = from.square_to_point();

                    let to_kind = self.state.get_banmen()[dy as usize][dx as usize];

                    let from_kind = if m.is_nari() {
                        match to_kind {
                            KomaKind::SFuN => KomaKind::SFu,
                            KomaKind::SKyouN => KomaKind::SKyou,
                            KomaKind::SKeiN => KomaKind::SKei,
                            KomaKind::SGinN => KomaKind::SGin,
                            KomaKind::SKakuN => KomaKind::SKaku,
                            KomaKind::SHishaN => KomaKind::SHisha,
                            KomaKind::GFuN => KomaKind::GFu,
                            KomaKind::GKyouN => KomaKind::GKyou,
                            KomaKind::GKeiN => KomaKind::GKei,
                            KomaKind::GGinN => KomaKind::GGin,
                            KomaKind::GKakuN => KomaKind::GKaku,
                            KomaKind::GHishaN => KomaKind::GHisha,
                            _ => to_kind
                        }
                    } else {
                        to_kind
                    };

                    let obtained_kind = if let Some(obtained) = m.obtained() {
                        Some(KomaKind::from((undo_item.teban.opposite(), obtained)))
                    } else {
                        None
                    };

                    if undo_item.teban == Teban::Sente {
                        self.state.part.sente_self_board ^= 1 << (to + 1);
                        self.state.part.gote_opponent_board ^= 1 << (inverse_to + 1);
                        self.state.part.sente_nari_board &= !(1 << (to + 1));
                    } else {
                        self.state.part.gote_self_board ^= 1 << (inverse_to + 1);
                        self.state.part.sente_opponent_board ^= 1 << (to + 1);
                        self.state.part.gote_nari_board &= !(1 << (to + 1));
                    }

                    if obtained_kind.is_some() {
                        if undo_item.teban == Teban::Sente {
                            self.state.part.gote_self_board ^= 1 << (inverse_to + 1);
                            self.state.part.sente_opponent_board ^= 1 << (to + 1);
                            self.state.part.gote_nari_board &= !(1 << (to + 1));
                        } else {
                            self.state.part.sente_self_board ^= 1 << (to + 1);
                            self.state.part.gote_opponent_board ^= 1 << (inverse_to + 1);
                            self.state.part.sente_nari_board &= !(1 << (to + 1));
                        }
                    }

                    if undo_item.teban == Teban::Sente {
                        self.state.part.sente_self_board ^= 1 << (from + 1);
                        self.state.part.gote_opponent_board ^= 1 << (inverse_from + 1);

                        if from_kind >= KomaKind::SFuN && from_kind < KomaKind::GFu {
                            self.state.part.sente_nari_board ^= 1 << (from + 1);
                        }
                    } else {
                        self.state.part.gote_self_board ^= 1 << (inverse_from + 1);
                        self.state.part.sente_opponent_board ^= 1 << (from + 1);

                        if from_kind >= KomaKind::GFuN && from_kind < KomaKind::Blank {
                            self.state.part.gote_nari_board ^= 1 << (from + 1);
                        }
                    }

                    for (kind,p) in [(to_kind,to),(from_kind,from),(obtained_kind.unwrap_or(KomaKind::Blank),to)] {
                        match kind {
                            KomaKind::SFu | KomaKind::SFuN => {
                                self.state.part.sente_fu_board ^= 1 << (p + 1);
                            },
                            KomaKind::SKyou | KomaKind::SKyouN => {
                                self.state.part.sente_kyou_board ^= 1 << (p + 1);
                            },
                            KomaKind::SKei | KomaKind::SKeiN => {
                                self.state.part.sente_kei_board ^= 1 << (p + 1);
                            },
                            KomaKind::SGin | KomaKind::SGinN => {
                                self.state.part.sente_gin_board ^= 1 << (p + 1);
                            },
                            KomaKind::SKin => {
                                self.state.part.sente_kin_board ^= 1 << (p + 1);
                            },
                            KomaKind::SKaku | KomaKind::SKakuN => {
                                self.state.part.sente_kaku_board ^= 1 << (p + 1);
                            },
                            KomaKind::SHisha | KomaKind::SHishaN => {
                                self.state.part.sente_hisha_board ^= 1 << (p + 1);
                            },
                            KomaKind::SOu => {
                                self.state.part.gote_opponent_ou_position_board ^= 1 << (80 - p + 1);
                            },
                            KomaKind::GFu | KomaKind::GFuN => {
                                self.state.part.gote_fu_board ^= 1 << (p + 1);
                            },
                            KomaKind::GKyou | KomaKind::GKyouN => {
                                self.state.part.gote_kyou_board ^= 1 << (p + 1);
                            },
                            KomaKind::GKei | KomaKind::GKeiN => {
                                self.state.part.gote_kei_board ^= 1 << (p + 1);
                            },
                            KomaKind::GGin | KomaKind::GGinN => {
                                self.state.part.gote_gin_board ^= 1 << (p + 1);
                            },
                            KomaKind::GKin => {
                                self.state.part.gote_kin_board ^= 1 << (p + 1);
                            },
                            KomaKind::GKaku | KomaKind::GKakuN => {
                                self.state.part.gote_kaku_board ^= 1 << (p + 1);
                            },
                            KomaKind::GHisha | KomaKind::GHishaN => {
                                self.state.part.gote_hisha_board ^= 1 << (p + 1);
                            },
                            KomaKind::GOu => {
                                self.state.part.sente_opponent_ou_position_board ^= 1 << (p + 1);
                            },
                            KomaKind::Blank => {}
                        }
                    }

                    self.state.banmen[dy as usize][dx as usize] = KomaKind::Blank;
                    self.state.banmen[dy as usize][dx as usize] = obtained_kind.unwrap_or(KomaKind::Blank);
                    self.state.banmen[sy as usize][sx as usize] = from_kind;

                    if let Some(kind) = m.obtained() {
                        match self.mc {
                            MochigomaCollections::Pair(ref mut ms, ref mut mg) => {
                                if let Ok(mk) = MochigomaKind::try_from(kind) {
                                    if undo_item.teban == Teban::Sente {
                                        ms.pull(mk)?;
                                    } else {
                                        mg.pull(mk)?;
                                    }
                                }
                            },
                            MochigomaCollections::Empty => ()
                        }
                    }
                },
                LegalMove::Put(m) => {
                    let p = m.dst();
                    let (dx,dy) = p.square_to_point();

                    let kind = KomaKind::from((undo_item.teban, m.kind()));

                    if undo_item.teban == Teban::Sente {
                        self.state.part.sente_self_board ^= 1 << (p + 1);
                        self.state.part.gote_opponent_board ^= 1 << (80 - p + 1);
                    } else {
                        self.state.part.gote_self_board ^= 1 << (80 - p + 1);
                        self.state.part.sente_opponent_board ^= 1 << (p + 1);
                    }

                    match kind {
                        KomaKind::SFu | KomaKind::SFuN => {
                            self.state.part.sente_fu_board ^= 1 << (p + 1);
                        },
                        KomaKind::SKyou | KomaKind::SKyouN => {
                            self.state.part.sente_kyou_board ^= 1 << (p + 1);
                        },
                        KomaKind::SKei | KomaKind::SKeiN => {
                            self.state.part.sente_kei_board ^= 1 << (p + 1);
                        },
                        KomaKind::SGin | KomaKind::SGinN => {
                            self.state.part.sente_gin_board ^= 1 << (p + 1);
                        },
                        KomaKind::SKin => {
                            self.state.part.sente_kin_board ^= 1 << (p + 1);
                        },
                        KomaKind::SKaku | KomaKind::SKakuN => {
                            self.state.part.sente_kaku_board ^= 1 << (p + 1);
                        },
                        KomaKind::SHisha | KomaKind::SHishaN => {
                            self.state.part.sente_hisha_board ^= 1 << (p + 1);
                        },
                        KomaKind::SOu => {
                            self.state.part.gote_opponent_ou_position_board ^= 1 << (80 - p + 1);
                        },
                        KomaKind::GFu | KomaKind::GFuN => {
                            self.state.part.gote_fu_board ^= 1 << (p + 1);
                        },
                        KomaKind::GKyou | KomaKind::GKyouN => {
                            self.state.part.gote_kyou_board ^= 1 << (p + 1);
                        },
                        KomaKind::GKei | KomaKind::GKeiN => {
                            self.state.part.gote_kei_board ^= 1 << (p + 1);
                        },
                        KomaKind::GGin | KomaKind::GGinN => {
                            self.state.part.gote_gin_board ^= 1 << (p + 1);
                        },
                        KomaKind::GKin => {
                            self.state.part.gote_kin_board ^= 1 << (p + 1);
                        },
                        KomaKind::GKaku | KomaKind::GKakuN => {
                            self.state.part.gote_kaku_board ^= 1 << (p + 1);
                        },
                        KomaKind::GHisha | KomaKind::GHishaN => {
                            self.state.part.gote_hisha_board ^= 1 << (p + 1);
                        },
                        KomaKind::GOu => {
                            self.state.part.sente_opponent_ou_position_board ^= 1 << (p + 1);
                        },
                        KomaKind::Blank => {}
                    }

                    self.state.banmen[dy as usize][dx as usize] = KomaKind::Blank;

                    match &mut self.mc {
                        &mut MochigomaCollections::Pair(ref mut ms, ref mut mg) => {
                            if undo_item.teban == Teban::Sente {
                                ms.put(m.kind());
                            } else {
                                mg.put(m.kind());
                            }
                        },
                        mc @ &mut MochigomaCollections::Empty => {
                            if undo_item.teban == Teban::Sente {
                                let mut ms = Mochigoma::new();

                                ms.put(m.kind());

                                *mc = MochigomaCollections::Pair(ms, Mochigoma::new());
                            } else {
                                let mut mg = Mochigoma::new();

                                mg.put(m.kind());

                                *mc = MochigomaCollections::Pair(Mochigoma::new(), mg);
                            }
                        }
                    }
                }
            }

            for &p in undo_item.sente_pin_changed.iter() {
                self.state.part.sente_pin_board ^= 1 << (p + 1);
            }

            for &p in undo_item.gote_pin_changed.iter() {
                self.state.part.sente_pin_board ^= 1 << (p + 1);
            }

            for &p in undo_item.sente_checked_changed.iter() {
                self.state.part.sente_checked_board ^= 1 << (p + 1);
            }

            for &p in undo_item.gote_checked_changed.iter() {
                self.state.part.gote_checked_board ^= 1 << (p + 1);
            }
        }

        Ok(())
    }
}