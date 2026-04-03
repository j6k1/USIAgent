use std::convert::{TryFrom};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Index, IndexMut};
use rand::Rng;
use error::{IllegalParameterError};
use rule::{LegalMove, LegalMoveTo, Rule, SquareToPoint, State};
use shogi::{KomaKind, Teban};

pub const LOW_PLY_HISTORY_SIZE:usize = 5;

#[repr(transparent)]
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord)]
pub struct StatsEntry<const D:i32> {
    entry:i32
}
impl<const D:i32> StatsEntry<D> {
    #[inline]
    pub fn new(entry:i32) -> StatsEntry<D> {
        StatsEntry {
            entry:entry
        }
    }
}
impl<const D:i32> Add<i32> for StatsEntry<D> {
    type Output = Self;

    #[inline]
    fn add(self,bonus:i32) -> Self::Output {
        let clamped_bonus = bonus.clamp(-D,D);

        StatsEntry {
            entry: clamped_bonus - self.entry * clamped_bonus.abs() / D
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
#[derive(Clone)]
pub struct StatsHistory {
    max_ply:usize,
    main_history:Box<[[StatsEntry<7183>; 32768]; 2]>,
    pawn_history:Box<[[StatsEntry<8192>; 81]; 28]>,
    capture_history:Box<[[[StatsEntry<10692>; 14]; 81]; 28]>,
    continuation_history:Vec<[[StatsEntry<30000>; 81]; 28]>,
    low_ply_history:Box<[[StatsEntry<7183>; 32768]; 5]>,
}
impl StatsHistory {
    #[inline]
    pub fn new(max_ply:usize) -> StatsHistory {
        StatsHistory {
            max_ply:max_ply,
            main_history:Box::new([[StatsEntry::new(68); 32768]; 2]),
            pawn_history:Box::new([[StatsEntry::new(-1238); 81]; 28]),
            capture_history:Box::new([[[StatsEntry::new(-689); 14]; 81]; 28]),
            continuation_history: vec![[[StatsEntry::new(-529); 81]; 28]; max_ply+1],
            low_ply_history:Box::new([[StatsEntry::new(97); 32768]; 5]),
        }
    }

    #[inline]
    pub fn startup(&mut self) {
        for it in self.continuation_history.iter_mut() {
            for it in it.iter_mut() {
                it.fill(StatsEntry::new(-529));
            }
        }

        for it in self.low_ply_history.iter_mut() {
            it.fill(StatsEntry::new(97));
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        for it in self.main_history.iter_mut() {
            it.fill(StatsEntry::new(68));
        }

        for it in self.pawn_history.iter_mut() {
            it.fill(StatsEntry::new(-1238));
        }

        for it in self.capture_history.iter_mut() {
            for it in it.iter_mut() {
                it.fill(StatsEntry::new(-689));
            }
        }

        for it in self.continuation_history.iter_mut() {
            for it in it.iter_mut() {
                it.fill(StatsEntry::new(-529));
            }
        }

        for it in self.low_ply_history.iter_mut() {
            it.fill(StatsEntry::new(97));
        }
    }
    #[inline]
    fn key(&self, m:LegalMove) -> usize {
        match m {
            LegalMove::To(m) => ((m.is_nari() as usize) << 14) | ((m.src() as usize) << 7) | m.dst() as usize,
            LegalMove::Put(m) => ((m.kind() as usize + 81) << 7) | (m.dst() as usize),
        }
    }

    #[inline]
    fn normalize_kind(&self, teban: Teban, kind:KomaKind, m: LegalMove) -> Result<usize, IllegalParameterError> {
        let kind = if let LegalMove::Put(mv) = m {
            KomaKind::from((teban,mv.kind()))
        } else {
            kind
        };

        if kind == KomaKind::Blank {
            Err(IllegalParameterError::new(format!("The piece being moved is invalid ({:?})", kind))).unwrap()
        } else if m.is_nari() {
            Ok(kind.to_nari() as usize)
        } else {
            Ok(kind as usize)
        }
    }

    #[inline]
    fn normalize_capture_kind(&self, kind:KomaKind, m: LegalMoveTo) -> Result<usize, IllegalParameterError> {
        if kind == KomaKind::Blank {
            Err(IllegalParameterError::new(format!("The piece being moved is invalid ({:?})", kind))).unwrap()
        } else if m.is_nari() {
            Ok(kind.to_nari() as usize)
        } else {
            Ok(kind as usize)
        }
    }

    #[inline]
    fn moved_after_piece(&self, teban: Teban, state: &State, prev_move:LegalMove) -> Result<KomaKind, IllegalParameterError> {
        match prev_move {
            LegalMove::To(m) => {
                let (x,y) = m.dst().square_to_point();

                let kind = state.get_banmen().0[y as usize][x as usize];

                Ok(kind)
            },
            LegalMove::Put(m) => {
                match KomaKind::try_from((teban,m.kind())) {
                    Ok(kind) => Ok(kind),
                    Err(_) => {
                        Err(IllegalParameterError::new(format!("The piece being moved is invalid ({:?})", m.kind())))
                    }
                }
            }
        }
    }

    #[inline]
    fn moved_after_piece_index(&self, teban: Teban, state: &State, prev_move:LegalMove) -> Result<usize, IllegalParameterError> {
        Ok(self.normalize_kind(teban,self.moved_after_piece(teban,state,prev_move)?,prev_move)?)
    }

    #[inline]
    pub fn lookup_main_history(&self, teban: Teban, m:LegalMove) -> Result<i32, IllegalParameterError> {
        let key = self.key(m);

        Ok(self.main_history[teban as usize][key].into())
    }

    #[inline]
    pub fn lookup_low_ply_history(&self, ply: usize, teban: Teban, m:LegalMove) -> Result<i32, IllegalParameterError> {
        if ply as usize >= LOW_PLY_HISTORY_SIZE {
            return Err(IllegalParameterError::new(String::from(
                "ply value exceeds max_ply."
            )));
        }

        let key = self.key(m);

        Ok(self.low_ply_history[teban as usize][key].into())
    }

    #[inline]
    pub fn lookup_pawn_history(&self, teban: Teban, kind: KomaKind, m:LegalMove) -> Result<i32, IllegalParameterError> {
        let kind = self.normalize_kind(teban,kind,m)?;

        Ok(self.pawn_history[kind][m.dst() as usize].into())
    }
    #[inline]
    pub fn continuation_histories_iter(&self, ply: usize, teban: Teban, kind: KomaKind, m:LegalMove)
                                       -> Result<impl Iterator<Item = &StatsEntry<30000>>, IllegalParameterError> {
        if ply as usize > self.max_ply {
            return Err(IllegalParameterError::new(String::from(
                "ply value exceeds max_ply."
            )));
        }

        let kind = self.normalize_kind(teban,kind,m)?;

        Ok(self.continuation_history.iter()
            .take(ply as usize + 1)
            .rev().skip(1)
            .take(6).map(move |h| h[kind].index(m.dst() as usize))
        )
    }

    #[inline]
    pub fn continuation_histories_iter_mut(&mut self, ply: usize, teban: Teban, kind: KomaKind, m:LegalMove)
                                           -> Result<impl Iterator<Item = &mut StatsEntry<30000>>, IllegalParameterError> {
        if ply as usize > self.max_ply {
            return Err(IllegalParameterError::new(String::from(
                "ply value exceeds max_ply."
            )));
        }

        let kind = self.normalize_kind(teban,kind,m)?;

        Ok(self.continuation_history.iter_mut()
            .take(ply as usize + 1)
            .rev().skip(1)
            .take(6).map(move |h| h[kind].index_mut(m.dst() as usize))
        )
    }

    #[inline]
    pub fn lookup_capture_history(&self, teban: Teban, kind: KomaKind, m:LegalMoveTo) -> Result<i32, IllegalParameterError> {
        let kind = self.normalize_capture_kind(kind, m)?;

        if let Some(o) = m.obtained() {
            Ok(self.capture_history[kind as usize][m.dst() as usize][o as usize].into())
        } else {
            Err(IllegalParameterError::new(String::from("This is not a move that captures a piece.")))
        }
    }
    const CONTNUATION_HISTORY_BONUSES:[i32; 6] = [1157,648,288,576,140,441];
    #[inline]
    pub fn update_continuation_history(&mut self, ply: usize, teban: Teban, kind: KomaKind, m:LegalMove, bonus:i32)
        -> Result<(), IllegalParameterError> {
        if ply as usize > self.max_ply {
            return Err(IllegalParameterError::new(String::from(
                "ply value exceeds max_ply."
            )));
        }

        let moved_piece = self.normalize_kind(teban,kind,m)?;

        self.continuation_history[ply][moved_piece][m.dst() as usize] += bonus;

        Ok(())
    }
    #[inline]
    pub fn update_continuation_histories(&mut self, ply: usize, teban: Teban, in_check: bool, kind: KomaKind, m:LegalMove, bonus:i32)
        -> Result<(), IllegalParameterError> {
        if ply as usize > self.max_ply {
            return Err(IllegalParameterError::new(String::from(
                "ply value exceeds max_ply."
            )));
        }

        for (i,(s,&w)) in self.continuation_histories_iter_mut(
            ply,
            teban,
            kind,
            m)?
            .zip(Self::CONTNUATION_HISTORY_BONUSES.iter()).enumerate() {

            if in_check && i > 1 {
                break;
            }

            *s += (bonus * w / 1024) + 88 * (i < 1) as i32;
        }

        Ok(())
    }

    #[inline]
    pub fn update_quiet_histories(&mut self, ply: usize, teban: Teban, state: &State, mut kind: KomaKind, m: LegalMove, bonus:i32)
        -> Result<(), IllegalParameterError> {
        self.main_history[teban as usize][self.key(m)] += bonus;

        if ply < LOW_PLY_HISTORY_SIZE {
            self.low_ply_history[teban as usize][self.key(m)] += bonus * 761 / 1024;
        }

        self.update_continuation_histories(ply, teban, Rule::in_check(teban,state), kind, m, bonus * 955 / 1024)?;

        if let LegalMove::Put(mv) = m {
            kind = KomaKind::from((teban,mv.kind()));
        }

        self.pawn_history[self.normalize_kind(teban,kind,m)?][m.dst() as usize] += bonus * if bonus > 0 {
            850
        } else {
            550
        } / 1024;

        Ok(())
    }

    #[inline]
    pub fn update_quiet_histories_by_static_eval(&mut self, teban: Teban, tt_hit: bool, mut prev_kind: KomaKind, prev_move: LegalMove,
                                                static_eval: i32, prev_static_eval: i32)
        -> Result<(), IllegalParameterError> {
        if let LegalMove::Put(mv) = prev_move {
            prev_kind = KomaKind::from((teban.opposite(),mv.kind()));
        }
        let eval_diff = (-prev_static_eval + static_eval).clamp(-200, 156) + 58;

        self.main_history[teban.opposite() as usize][self.key(prev_move)] += eval_diff * 9;

        if !tt_hit && !prev_move.is_nari() {
            self.pawn_history[self.normalize_kind(teban.opposite(),prev_kind,prev_move)?][prev_move.dst() as usize] += eval_diff * 14;
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
                            prev_kind: KomaKind, prev_move:Option<LegalMove>) -> Result<(), IllegalParameterError> {
        let best_move_moved_piece  = self.normalize_kind(teban,best_move_kind,best_move)?;

        let bonus = (121 * depth as i32 - 77).min(1633) + 375 * tt_move.map(|m| m == best_move).unwrap_or(false) as i32;
        let malus = (825 * depth as i32- 196).min(2159) - 16 * move_count as i32;

        if let Some(o) = best_move.obtained() {
            self.capture_history[best_move_moved_piece][best_move.dst() as usize][o as usize] += bonus * 1482 / 1024;
        } else {
    self.update_quiet_histories(ply, teban, state, best_move_kind, best_move, bonus * 881 / 1024)?;

            for &m in quiets_searched {
                match m {
                    LegalMove::To(mv) => {
                        let (x,y) = mv.src().square_to_point();
                        let moved_piece = state.get_banmen().0[y as usize][x as usize];

                        self.update_quiet_histories(ply, teban, state, moved_piece, m,  -malus * 1083 / 1024)?;
                    },
                    LegalMove::Put(_) => {
                        self.update_quiet_histories(ply, teban, state, KomaKind::Blank, m,  -malus * 1083 / 1024)?;
                    }
                }
            }
        }

        if let Some(prev_move) = prev_move {
            if tt_hit && prev_move.obtained().is_some() {
                self.update_continuation_histories(ply - 1, teban.opposite(), prev_in_check, prev_kind, prev_move, -malus * 614 / 1024)?;
            }
        }

        for &m in captures_searched {
            if let Some(o) = m.obtained() {
                let (x,y) = m.src().square_to_point();
                let moved_piece = self.normalize_capture_kind(state.get_banmen().0[y as usize][x as usize],m)?;

                self.capture_history[moved_piece][m.dst() as usize][o as usize] += -malus * 1397 / 1024;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn update_quiet_histories_by_fail_high_tt_move(&mut self, ply: usize, depth: u32, teban: Teban, state: &State,
                                                       tt_kind: KomaKind, tt_move: LegalMove,
                                                       move_count: usize,
                                                       prev_move: Option<LegalMove>)
        -> Result<(), IllegalParameterError> {
        if tt_move.obtained().is_none() {
            self.update_quiet_histories(ply, teban, state, tt_kind, tt_move,  (130 * depth as i32 - 71).min(1043))?;
        }

        if let Some(prev_move) = prev_move {
            if prev_move.obtained().is_none() && move_count <= 4 {
                self.update_continuation_histories(ply - 1, teban.opposite(), Rule::in_check(teban,state),
                                                      self.moved_after_piece(teban.opposite(),state,prev_move)?,prev_move
                                                      , -2142)?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn update_quiet_histories_when_fail_low<T>(&mut self, ply: usize, depth: u32,
                                                   stat_score: i32,
                                                   teban: Teban, state: &State,
                                                   move_count: usize,
                                                   best_value: T,
                                                   static_eval: i32, prev_static_eval: i32,
                                                   prev_in_check: bool,
                                                   prev_kind: KomaKind,
                                                   prev_move: LegalMove)
        -> Result<(), IllegalParameterError> where T: Ord + From<i32> {
        let mut bonus_scale = -228;

        bonus_scale -= stat_score / 104;
        bonus_scale += (63 * depth as i32).min(508);
        bonus_scale += 184 * (move_count > 8) as i32;
        bonus_scale += 143 * (!Rule::in_check(teban,state) && best_value <= T::from(static_eval - 92)) as i32;
        bonus_scale += 149 * (!prev_in_check && best_value <= T::from(-(prev_static_eval - 70))) as i32;

        bonus_scale = bonus_scale.max(0);

        let scaled_bonus = (144 * depth as i32 - 92).min(1365) * bonus_scale;

        self.update_continuation_histories(ply - 1, teban.opposite(), prev_in_check, prev_kind, prev_move, scaled_bonus * 400 / 32768)?;

        self.main_history[teban.opposite() as usize][self.key(prev_move)] += scaled_bonus * 220 / 32768;

        if prev_kind != KomaKind::SFu && prev_kind != KomaKind::GFu && !prev_move.is_nari() {
            let kind_index = self.moved_after_piece_index(teban.opposite(), state, prev_move)?;

            self.pawn_history[kind_index][prev_move.dst() as usize] += scaled_bonus * 1164 / 32768;
        }

        Ok(())
    }

    #[inline]
    pub fn update_capture_histories_when_fail_low(&mut self, teban: Teban, state: &State, prev_move: LegalMove)
        -> Result<(), IllegalParameterError> {
        if let Some(o) = prev_move.obtained() {
            let prev_kind = self.moved_after_piece_index(teban.opposite(), state, prev_move)?;

            self.capture_history[prev_kind][prev_move.dst() as usize][o as usize] += 964;

            Ok(())
        } else {
            Err(IllegalParameterError::new(String::from("This is a non-capture move.")))
        }
    }
}
impl Debug for StatsHistory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StatsHistory")
    }
}