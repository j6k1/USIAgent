//! 合法手を生成するために利用するビットボード関連の実装
use std::fmt;
use std::fmt::Formatter;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr, Sub};
use bits::pop_lsb;
use rule::Square;

/// 合法手を生成するために内部で利用するビットボード
#[derive(Clone, Copy)]
pub union BitBoard {
    merged_bitboard:u128,
    bitboard:[u64; 2]
}
impl From<u128> for BitBoard {
    fn from(bits: u128) -> Self {
        BitBoard {
            merged_bitboard: bits
        }
    }
}
impl From<(u64,u64)> for BitBoard {
    fn from((low,high): (u64, u64)) -> Self {
        BitBoard {
            bitboard: [low,high]
        }
    }
}
impl Default for BitBoard {
    fn default() -> Self {
        BitBoard {
            merged_bitboard: 0
        }
    }
}
impl From<BitBoard> for (u64,u64) {
    fn from(bitboard: BitBoard) -> Self {
        unsafe { (*bitboard.bitboard.get_unchecked(0), *bitboard.bitboard.get_unchecked(1)) }
    }
}
impl BitOr for BitBoard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        unsafe {
            BitBoard { merged_bitboard: self.merged_bitboard | rhs.merged_bitboard }
        }
    }
}
impl BitAnd for BitBoard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe {
            BitBoard { merged_bitboard: self.merged_bitboard & rhs.merged_bitboard }
        }
    }
}
impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = unsafe {
            BitBoard { merged_bitboard: self.merged_bitboard & rhs.merged_bitboard }
        }
    }
}
impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = unsafe {
            BitBoard { merged_bitboard: self.merged_bitboard | rhs.merged_bitboard }
        }
    }
}
impl BitXor for BitBoard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        unsafe {
            BitBoard { merged_bitboard: self.merged_bitboard ^ rhs.merged_bitboard }
        }
    }
}
impl BitXorAssign for BitBoard {

    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        unsafe {
            self.merged_bitboard ^= rhs.merged_bitboard;
        }
    }
}
impl Not for BitBoard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        unsafe {
            BitBoard { merged_bitboard: !self.merged_bitboard }
        }
    }
}
impl Sub for BitBoard {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        unsafe {
            BitBoard { merged_bitboard: self.merged_bitboard - rhs.merged_bitboard }
        }
    }
}
impl PartialEq for BitBoard {
    #[inline]
    fn eq(&self,other:&BitBoard) -> bool {
        unsafe { self.merged_bitboard == other.merged_bitboard }
    }
}
impl BitOr<u128> for BitBoard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: u128) -> Self {
        unsafe {
            BitBoard { merged_bitboard: self.merged_bitboard | rhs }
        }
    }
}
impl BitAnd<u128> for BitBoard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: u128) -> Self::Output {
        unsafe {
            BitBoard { merged_bitboard: self.merged_bitboard & rhs }
        }
    }
}
impl BitAndAssign<u128> for BitBoard {
    fn bitand_assign(&mut self, rhs: u128) {
        *self = unsafe {
            BitBoard { merged_bitboard: self.merged_bitboard & rhs }
        }
    }
}
impl BitOrAssign<u128> for BitBoard {
    fn bitor_assign(&mut self, rhs: u128) {
        *self = unsafe {
            BitBoard { merged_bitboard: self.merged_bitboard | rhs }
        }
    }
}
impl BitXor<u128> for BitBoard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: u128) -> Self {
        unsafe {
            BitBoard { merged_bitboard: self.merged_bitboard ^ rhs }
        }
    }
}
impl BitXorAssign<u128> for BitBoard {

    #[inline]
    fn bitxor_assign(&mut self, rhs: u128) {
        unsafe {
            self.merged_bitboard ^= rhs;
        }
    }
}
impl Sub<u128> for BitBoard {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: u128) -> Self::Output {
        unsafe {
            BitBoard { merged_bitboard: self.merged_bitboard - rhs }
        }
    }
}
impl Shl<u128> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: u128) -> Self::Output {
        BitBoard { merged_bitboard: unsafe { self.merged_bitboard } << rhs }
    }
}
impl Shr<u128> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: u128) -> Self::Output {
        BitBoard { merged_bitboard: unsafe { self.merged_bitboard } >> rhs }
    }
}
impl PartialEq<u128> for BitBoard {
    #[inline]
    fn eq(&self,other:&u128) -> bool {
        unsafe { self.merged_bitboard == *other }
    }
}
impl Eq for BitBoard {}
impl fmt::Debug for BitBoard {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", unsafe { self.merged_bitboard })
    }
}
impl BitAnd<BitBoard> for u128 {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, rhs: BitBoard) -> Self::Output {
        unsafe {
            BitBoard { merged_bitboard: self & rhs.merged_bitboard }
        }
    }
}
/// ビットボードの最下位ビットを取り出すイテレータ
pub struct PopLsbIterByCallback<F> {
    callback:F
}
impl BitBoard {
    /// ビットボード内の下位方向から見たビット位置を取り出して返すイテレータを生成する
    #[inline]
    pub fn iter(self) -> impl Iterator<Item = Square> {
        let br = (unsafe { *self.bitboard.get_unchecked(0) } == 0) as usize;
        let bl = (unsafe { *self.bitboard.get_unchecked(1) } == 0) as usize;

        let mut index = br + (bl & br);

        let mut board = self;

        PopLsbIterByCallback {
            callback: move || {
                if index == 2 {
                    None
                } else {
                    let p = unsafe { board.bitboard.get_unchecked_mut(index) };

                    if *p == 0 {
                        return None;
                    }

                    let s = pop_lsb(&mut *p) as Square + index as Square * 64 - 1;

                    index += ((*p) == 0) as usize;

                    Some(s as Square)
                }
            }
        }
    }

    /// ビットボードのビット列を反転させ、下位から82bit分の範囲に収まるように位置を調整する
    #[inline]
    pub fn reverse(self) -> BitBoard {
        BitBoard {
            merged_bitboard: unsafe { self.merged_bitboard.reverse_bits() >> 45 }
        }
    }

    /// ビットボード内の立っているビットの個数を返す
    #[inline]
    pub fn bitcount(self) -> usize {
        unsafe {
            self.bitboard.get_unchecked(0).count_ones() as usize + self.bitboard.get_unchecked(1).count_ones() as usize
        }
    }

    /// BitBoardに対するオーバーフローを許容する減算
    #[inline]
    pub fn wrapping_sub(self,rhs: u32) -> BitBoard {
        BitBoard {
            merged_bitboard: unsafe { self.merged_bitboard.wrapping_sub(rhs as u128) }
        }
    }
}
impl<F> Iterator for PopLsbIterByCallback<F> where F: FnMut() -> Option<Square> {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        (self.callback)()
    }
}
