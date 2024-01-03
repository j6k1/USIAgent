#[cfg(target_feature = "bmi")]
pub fn blsr64(v:u64) -> u64 {
    _blsr_u64(v)
}
#[cfg(not(target_feature = "bmi"))]
#[inline(always)]
pub fn blsr64(v:u64) -> u64 {
    let v = v & (v - 1);
    v
}
#[inline(always)]
pub fn pop_lsb(bits:&mut u64) -> u64 {
    let index = lsb64(*bits);

    *bits = blsr64(*bits);

    index
}
#[inline(always)]
pub fn lsb64(bits:u64) -> u64 {
    bits.trailing_zeros() as u64
}