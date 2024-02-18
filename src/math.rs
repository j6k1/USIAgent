//! 乱数生成などの実装を定義するモジュール
/// 高速な乱数生成器の実装
pub struct Prng {
    s:u64
}
impl Prng {
    /// 乱数生成器を初期化して返す
    ///
    /// # Arguments
    /// * `seed` - 乱数シード
    pub fn new(seed:u64) -> Prng {
        Prng {
            s:seed
        }
    }

    /// u64型の乱数を生成して返す
    pub fn rnd64(&mut self) -> u64 {
        self.s ^= self.s >> 12;
        self.s ^= self.s << 25;
        self.s ^= self.s >> 27;
        return (self.s as u128 * 2685821657736338717u128) as u64;
    }

    /// 0~n-1の範囲の乱数を生成して返す
    ///
    /// # Argumants
    /// * `n` - 乱数の値の上限（n自身は含まない）
    pub fn rnd(&mut self,n:u64) -> u64 {
        self.rnd64() % n
    }
}