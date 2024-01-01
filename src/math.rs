pub struct Prng {
    s:u64
}
impl Prng {
    pub fn new(seed:u64) -> Prng {
        Prng {
            s:seed
        }
    }

    pub fn rnd64(&mut self) -> u64 {
        self.s ^= self.s >> 12;
        self.s ^= self.s << 25;
        self.s ^= self.s >> 27;
        return (self.s as u128 * 2685821657736338717u128) as u64;
    }

    pub fn rnd(&mut self,n:u64) -> u64 {
        self.rnd64() % n
    }
}