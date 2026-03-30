use std::convert::TryFrom;
use std::ops::{Add, AddAssign};
use error::InvalidParameterError;
use rule::{LegalMove, LegalMoveTo, Rule, SquareToPoint, State};
use shogi::{KomaKind, Teban};
use shogi::KomaKind::GFu;

const LOW_PLY_HISTORY_SIZE:usize = 5;

#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord)]
pub struct StatsEntry<const D:i32> {
    entry:i32,
    update_count:u32
}
impl<const D:i32> StatsEntry<D> {
    #[inline]
    pub fn new(entry:i32) -> StatsEntry<D> {
        StatsEntry {
            entry:entry,
            update_count:0,
        }
    }
}
impl<const D:i32> Add<i32> for StatsEntry<D> {
    type Output = Self;

    #[inline]
    fn add(self,bonus:i32) -> Self::Output {
        let clamped_bonus = bonus.clamp(-D,D);

        StatsEntry {
            entry: clamped_bonus - self.entry * clamped_bonus.abs() / D,
            update_count: self.update_count + 1
        }
    }
}
impl<const D:i32> AddAssign<i32> for StatsEntry<D> {
    #[inline]
    fn add_assign(&mut self, bonus:i32) {
        *self = *self + bonus;
    }
}
impl<const D:i32> From<StatsEntry<D>> for i32 {
    fn from(entry:StatsEntry<D>) -> i32 {
        entry.entry
    }
}
#[derive(Clone,Debug)]
pub struct StatsHistory {
    max_ply:usize,
    main_history:[[StatsEntry<7183>; 32768]; 2],
    pawn_history:[[StatsEntry<8192>; 81]; 14],
    capture_history:[[[StatsEntry<10692>; 7]; 81]; 14],
    continuation_history:[Vec<[[StatsEntry<30000>; 81]; 14]>; 2],
    low_ply_history:[[StatsEntry<7183>; 32768]; 5],
}
impl StatsHistory {
    #[inline]
    pub fn new(max_ply:usize) -> StatsHistory {
        StatsHistory {
            max_ply:max_ply,
            main_history:[[StatsEntry::new(68); 32768]; 2],
            pawn_history:[[StatsEntry::new(-1238); 81]; 14],
            capture_history:[[[StatsEntry::new(-689); 7]; 81]; 14],
            continuation_history:[
                vec![[[StatsEntry::new(-529); 81]; 14]; max_ply+1],
                vec![[[StatsEntry::new(-529); 81]; 14]; max_ply+1],
            ],
            low_ply_history:[[StatsEntry::new(97); 32768]; 5],
        }
    }

    #[inline]
    pub fn startup(&mut self) {
        self.continuation_history = [
            vec![[[StatsEntry::new(-529); 81]; 14]; self.max_ply+1],
            vec![[[StatsEntry::new(-529); 81]; 14]; self.max_ply+1],
        ];
        self.low_ply_history = [[StatsEntry::new(97); 32768]; 5];
    }

    #[inline]
    fn move_to_index(&self, m:LegalMove) -> usize {
        match m {
            LegalMove::To(m) => ((m.is_nari() as usize) << 14) | ((m.src() as usize) << 7) | m.dst() as usize,
            LegalMove::Put(m) => ((m.kind() as usize + 81) << 7) | (m.dst() as usize),
        }
    }

    #[inline]
    fn move_to_moved_piece(&self, kind: KomaKind, teban: Teban, m:LegalMove) -> Result<usize,InvalidParameterError> {
        match m {
            LegalMove::To(m) => {
                if kind == KomaKind::Blank {
                    Err(InvalidParameterError::new(format!("The piece being moved is invalid ({:?})",kind)))
                } else {
                    match teban {
                        Teban::Sente if kind < KomaKind::GFu && m.is_nari() => {
                            Ok(kind.to_nari() as usize)
                        },
                        Teban::Sente if kind < KomaKind::GFu => {
                            Ok(kind as usize)
                        },
                        Teban::Gote if kind >= KomaKind::GFu && m.is_nari() => {
                            Ok(kind.is_nari() as usize - KomaKind::GFu as usize)
                        },
                        Teban::Gote if kind >= KomaKind::GFu => {
                            Ok(kind as usize - KomaKind::GFu as usize)
                        },
                        _ => {
                            Err(InvalidParameterError::new(format!("The piece being moved is invalid ({:?})",kind)))
                        }
                    }
                }
            },
            LegalMove::Put(m) => {
                match KomaKind::try_from((teban,m.kind())) {
                    Ok(kind) => Ok(kind as usize),
                    Err(e) => {
                        Err(InvalidParameterError::new(format!("The piece being moved is invalid ({:?})",m.kind())))
                    }
                }
            }
        }
    }
    const CONTNUATION_HISTORY_BONUSES:[i32; 6] = [1157,648,288,576,140,441];
    #[inline]
    pub fn update_continuation_history(&mut self, ply: usize, teban: Teban, state: &State, kind:KomaKind, m:LegalMove, bonus:i32)
        -> Result<(),InvalidParameterError> {
        let moved_piece = self.move_to_moved_piece(kind, teban, m)?;

        self.continuation_history[teban as usize][ply][moved_piece][m.dst() as usize] += bonus;

        Ok(())
    }

    #[inline]
    pub fn update_continuation_histories(&mut self, ply: usize, teban: Teban, in_check: bool, kind:KomaKind, m:LegalMove, bonus:i32)
        -> Result<(),InvalidParameterError> {
        let kind = self.move_to_moved_piece(kind, teban, m)?;

        for (i,(h,&w)) in self.continuation_history[teban as usize].iter_mut()
            .take(ply as usize + 1)
            .rev().skip(1)
            .take(6).zip(Self::CONTNUATION_HISTORY_BONUSES.iter()).enumerate() {

            if in_check && i > 1 {
                break;
            }

            h[kind][m.dst() as usize] += (bonus * w / 1024) + 88 * (i < 1) as i32;
        }

        Ok(())
    }

    #[inline]
    pub fn update_quiet_histories(&mut self, ply: usize, teban: Teban, state: &State, kind: KomaKind, m: LegalMove, bonus:i32)
        -> Result<(),InvalidParameterError> {
        self.main_history[teban as usize][self.move_to_index(m)] += bonus;

        if ply < LOW_PLY_HISTORY_SIZE {
            self.low_ply_history[teban as usize][self.move_to_index(m)] += bonus * 761 / 1024;
        }

        self.update_continuation_histories(ply, teban, Rule::in_check(teban,state), kind, m, bonus * 955 / 1024)?;

        self.pawn_history[self.move_to_moved_piece(kind, teban, m)?][m.dst() as usize] += bonus * if bonus > 0 {
            850
        } else {
            550
        } / 1024;

        Ok(())
    }

    #[inline]
    pub fn update_quiet_histories_by_static_eval(&mut self, teban: Teban, tt_hit: bool, prev_kind: KomaKind, prev_move: LegalMove,
                                                static_eval: i32, prev_static_eval: i32)
        -> Result<(),InvalidParameterError> {
        let eval_diff = (-prev_static_eval + static_eval).clamp(-200, 156) + 58;

        self.main_history[teban.opposite() as usize][self.move_to_index(prev_move)] += eval_diff * 9;

        if !tt_hit && prev_kind != KomaKind::SFu && prev_kind != KomaKind::GFu && !prev_move.is_nari() {
            self.pawn_history[self.move_to_moved_piece(prev_kind, teban, prev_move)?][prev_move.dst() as usize] += eval_diff * 14;
        }

        Ok(())
    }

    #[inline]
    pub fn update_all_stats(&mut self, ply: usize, depth: u32,
                            teban: Teban, state: &State,
                            move_count: usize,
                            prev_in_check: bool,
                            best_move_kind: KomaKind,
                            best_move: LegalMove, tt_move: Option<LegalMove>,
                            quiets_searched: &[LegalMove],
                            captures_searched: &[LegalMoveTo],
                            tt_hit: bool,
                            prev_kind: KomaKind, prev_move:Option<LegalMove>) -> Result<(),InvalidParameterError> {
        let best_move_moved_piece  = self.move_to_moved_piece(best_move_kind, teban, best_move)?;

        let bonus = (121 * depth as i32 - 77).min(1633) + 375 * tt_move.map(|m| m == best_move).unwrap_or(false) as i32;
        let malus = (825 * depth as i32- 1962159) - 16 * move_count as i32;

        if let Some(o) = best_move.obtained() {
            self.capture_history[best_move_moved_piece][best_move.dst() as usize][o as usize] += bonus * 1482 / 1024;
        } else {
    self.update_quiet_histories(ply, teban, state, best_move_kind, best_move, bonus * 881 / 1024);

            for &m in quiets_searched {
                match m {
                    LegalMove::To(mv) => {
                        let (x,y) = mv.src().square_to_point();
                        let moved_piece = state.get_banmen().0[y as usize][x as usize];

                        self.update_quiet_histories(ply, teban, state, moved_piece, m,  -malus * 1083 / 1024)?;
                    },
                    LegalMove::Put(mv) => {
                        self.update_quiet_histories(ply, teban, state, KomaKind::Blank, m,  -malus * 1083 / 1024)?;
                    }
                }
            }
        }

        if let Some(prev_move) = prev_move {
            if tt_hit && prev_move.obtained().is_some() {
                self.update_continuation_histories(ply - 1, teban.opposite(), prev_in_check, prev_kind, prev_move, -malus * 614 / 1024);
            }
        }

        for &m in captures_searched {
            if let Some(o) = m.obtained() {
                let (x,y) = m.src().square_to_point();
                let moved_piece = self.move_to_moved_piece(state.get_banmen().0[y as usize][x as usize],teban,LegalMove::To(m))?;

                self.capture_history[moved_piece][m.dst() as usize][o as usize] += -malus * 1397 / 1024;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn update_quiet_histories_by_fail_high_tt_move(&mut self, ply: usize, depth: u32, teban: Teban, state: &State, tt_kind: KomaKind, tt_move: LegalMove)
        -> Result<(),InvalidParameterError> {
        self.update_quiet_histories(ply, teban, state, tt_kind, tt_move,  (130 * depth as i32 - 71).min(1043))
    }

    #[inline]
    pub fn update_quiet_histories_when_fail_low<T>(&mut self, ply: usize, stat_score: i32,
                                                   teban: Teban, state: &State,
                                                   move_count: usize, depth: u32,
                                                   best_value: T, static_eval: i32, prev_static_eval: i32
                                                   prev_in_check: bool,
                                                   prev_kind: KomaKind, prev_move: LegalMove)
        -> Result<(),InvalidParameterError> where T: Ord + From<i32> {
        if prev_kind == KomaKind::Blank {
            return Err(InvalidParameterError::new(format!("The piece being moved is invalid ({:?})",prev_kind)))
        }

        let mut bonus_scale = -228;

        bonus_scale -= stat_score / 104;
        bonus_scale += (63 * depth as i32).min(508);
        bonus_scale += 184 * (move_count > 8) as i32;
        bonus_scale += 143 * (!Rule::in_check(teban,state) && best_value <= T::from(static_eval - 92)) as i32;
        bonus_scale += 149 * (!prev_in_check && best_value <= T::from(-(static_eval - 70))) as i32;

        bonus_scale = bonus_scale.max(0);

        let scaled_bonus = (144 * depth as i32 - 92).min(1365) * bonus_scale;

        self.update_continuation_histories(ply - 1, teban.opposite(), prev_in_check, prev_kind, prev_move, scaled_bonus * 400 / 32768)?;

        self.main_history[teban.opposite() as usize][self.move_to_index(prev_move)] += scaled_bonus * 220 / 32768;

        if prev_kind != KomaKind::SFu && prev_kind != KomaKind::GFu && !prev_move.is_nari() {
            let kind_index = if prev_kind < GFu {
                prev_kind as usize
            } else {
                prev_kind as usize - GFu as usize
            };

            self.pawn_history[kind_index][prev_move.dst() as usize] += scaled_bonus * 1164 / 32768;
        }

        Ok(())
    }

    #[inline]
    pub fn update_capture_histories_when_fail_low(&mut self, prev_kind: KomaKind, prev_move: LegalMove)
        -> Result<(),InvalidParameterError> {
        if prev_kind == KomaKind::Blank {
            return Err(InvalidParameterError::new(format!("The piece being moved is invalid ({:?})",prev_kind)))
        }

        if let Some(o) = prev_move.obtained() {
            let kind_index = if prev_kind < GFu {
                prev_kind as usize
            } else {
                prev_kind as usize - GFu as usize
            };

            self.capture_history[kind_index][prev_move.dst() as usize][o as usize] += 964;

            Ok(())
        } else {
            Err(InvalidParameterError::new(String::from("This is a non-capture move.")))
        }
    }
}