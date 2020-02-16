pub trait Blsr<Rhs = Self> {
	type Output;
	fn blsr(&self) -> Self::Output;
}
pub trait LzCnt<Rhs = Self> {
	type Output;
	fn lzcnt(&self) -> Self::Output;
}
pub trait TzCnt<Rhs = Self> {
	type Output;
	fn tzcnt(&self) -> Self::Output;
}

impl Blsr for u64 {
	type Output = u64;

	#[cfg(all(target_arch = "x86_64",target_feature = "bmi1"))]
	#[inline(always)]
	fn blsr(&self) -> u64 {
		use std::arch::x86_64::_blsr_u64;

		unsafe {
			_blsr_u64(*self)
		}
	}

	#[cfg(not(all(target_arch = "x86_64",target_feature = "bmi1")))]
	#[inline(always)]
	fn blsr(&self) -> u64 {
		*self & (*self - 1)
	}
}
impl TzCnt for u64 {
	type Output = u32;

	#[cfg(all(target_arch = "x86_64",target_feature = "bmi1"))]
	#[inline(always)]
	fn tzcnt(&self) -> u32 {
		use std::arch::x86_64::_tzcnt_u64;

		unsafe {
			_tzcnt_u64(*self) as u32
		}
	}

	#[cfg(not(all(target_arch = "x86_64",target_feature = "bmi1")))]
	#[inline(always)]
	fn tzcnt(&self) -> u32 {
		self.trailing_zeros()
	}
}
impl LzCnt for u64 {
	type Output = u32;

	#[cfg(target_arch = "x86_64")]
	#[inline(always)]
	fn lzcnt(&self) -> u32 {
		use std::arch::x86_64::_lzcnt_u64;

		unsafe {
			_lzcnt_u64(*self) as u32
		}
	}

	#[cfg(not(target_arch = "x86_64"))]
	#[inline(always)]
	fn lzcnt(&self) -> u32 {
		self.leading_zeros()
	}
}