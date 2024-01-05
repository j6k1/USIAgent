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
    mvs:[MaybeUninit<T>; 593],
    current:u16,
    count:u16
}
impl<T> RandomPicker<T> where T: Copy {
    pub fn new(r:Prng) -> RandomPicker<T> {
        RandomPicker {
            rnd:r,
            mvs:[MaybeUninit::uninit(); 593],
            current:0,
            count:0
        }
    }
}
impl<T> MovePicker<T> for RandomPicker<T> where T: Copy {
    #[inline]
    fn push(&mut self,m:T) -> Result<u16, LimitSizeError> {
        if self.count == MOVE_MAX {
            Err(LimitSizeError(self.count as usize))
        } else {
            unsafe { self.mvs.get_unchecked_mut(self.count as usize).write(m) };

            self.count += 1;

            Ok(self.count)
        }
    }

    #[inline]
    fn reset(&mut self) {
        self.current = 0;
        self.count = 0;
    }

    #[inline]
    fn len(&self) -> usize {
        self.count as usize
    }
}
impl<T> Iterator for RandomPicker<T> where T: Copy {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.count {
            None
        } else {
            let index = self.current;

            let swap_index = self.rnd.rnd(self.count as u64 - index as u64) as u16 + index;

            self.mvs.swap(index as usize,swap_index as usize);

            self.current += 1;

            Some(unsafe { (*self.mvs.get_unchecked(index as usize)).assume_init() })
        }
    }
}
impl<T> From<RandomPicker<T>> for Vec<T> where T: Copy {
    fn from(value: RandomPicker<T>) -> Self {
        let mut mvs = Vec::with_capacity(MOVE_MAX as usize);

        for i in 0..value.count {
            mvs.push(unsafe { value.mvs[i as usize].assume_init() });
        }
        mvs
    }
}