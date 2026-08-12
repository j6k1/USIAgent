//! 複数枚の駒からの効きを重ね合わせて差分更新可能な特殊なビットボードの実装

use std::ops::{AddAssign, SubAssign};
use bitboard::BitBoard;

/// 10枚のビットボードを組み合わせることで盤面上の飛車角香車を除くすべての駒の効き情報を高速に差分更新可能な構造体
#[derive(Clone,Eq,PartialEq,Debug)]
pub struct SuperPosition {
    boards:[BitBoard; 10]
}
impl SuperPosition {
    /// 複数枚のビットボードを重ね合わせて効きのビットボードを生成して返す
    #[inline]
    pub fn to_bitboard(&self) -> BitBoard {
        self.boards[0] | self.boards[1] | self.boards[2] | self.boards[3] |
        self.boards[4] | self.boards[5] | self.boards[6] | self.boards[7] |
        self.boards[8] | self.boards[9]
    }
}
impl Default for SuperPosition {
    #[inline]
    fn default() -> Self {
        SuperPosition {
            boards:[BitBoard::default(); 10]
        }
    }
}
impl AddAssign<BitBoard> for SuperPosition {
    #[inline]
    fn add_assign(&mut self, rhs: BitBoard) {
        let mut rhs = rhs;

        for i in 0..10 {
            let board = self.boards[i] | (self.boards[i] ^ rhs);
            rhs = self.boards[i] & rhs;

            self.boards[i] = board;

            if rhs == BitBoard::default() {
                break;
            }
        }
    }
}
impl SubAssign<BitBoard> for SuperPosition {
    #[inline]
    fn sub_assign(&mut self, rhs: BitBoard) {
        let mut rhs = rhs;

        for i in 0..10 {
            let board = self.boards[i] ^ (self.boards[i] & rhs);
            rhs ^= self.boards[i] & rhs;

            self.boards[i] = board;

            if rhs == BitBoard::default() {
                break;
            }
        }
    }
}
impl From<&SuperPosition> for BitBoard {
    #[inline]
    fn from(sp: &SuperPosition) -> Self {
        sp.to_bitboard()
    }
}