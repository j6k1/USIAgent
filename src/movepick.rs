use std::mem::MaybeUninit;
use error::LimitSizeError;
use math::Prng;

pub const MOVE_MAX:u16 = 593;

pub trait MovePicker<T> : Iterator<Item =T> {
    fn push(&mut self,m:T) -> Result<u16,LimitSizeError>;
    fn reset(&mut self);
    fn len(&self) -> usize;
}
pub struct RandomPicker<T> {
    rnd:Prng,
    mvs:[T; 593],
    current:u16,
    count:u16
}
impl<T> RandomPicker<T> where T: Copy {
    pub fn new(r:Prng) -> RandomPicker<T> {
        RandomPicker {
            rnd:r,
            mvs:[unsafe { MaybeUninit::zeroed().assume_init() }; 593],
            current:0,
            count:0
        }
    }
}
impl<T> MovePicker<T> for RandomPicker<T> where T: Copy {
    fn push(&mut self,m:T) -> Result<u16, LimitSizeError> {
        if self.count == MOVE_MAX {
            Err(LimitSizeError(self.count as usize))
        } else {
            self.mvs[self.count as usize] = m;

            self.count += 1;
            Ok(self.count)
        }
    }

    fn reset(&mut self) {
        self.current = 0;
        self.count = 0;
    }

    fn len(&self) -> usize {
        self.count as usize
    }
}
impl<T> Iterator for RandomPicker<T> where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current + 1 >= self.count {
            None
        } else {
            let index = self.current;

            let swap_index = self.rnd.rnd(self.count as u64 - index as u64) as u16 + index;

            let tmp = self.mvs[index as usize];
            self.mvs[index as usize] = self.mvs[swap_index as usize];
            self.mvs[swap_index as usize] = tmp;

            self.current += 1;

            Some(self.mvs[index as usize])
        }
    }
}
impl<T> From<RandomPicker<T>> for Vec<T> where T: Copy {
    fn from(value: RandomPicker<T>) -> Self {
        let mut mvs = Vec::with_capacity(MOVE_MAX as usize);

        for i in 0..value.count {
            mvs.push(value.mvs[i as usize]);
        }
        mvs
    }
}