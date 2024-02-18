//! ビットボード操作のためのユーティリティ関数群
#[cfg(target_feature = "bmi")]
pub fn blsr64(v:u64) -> u64 {
    _blsr_u64(v)
}
#[cfg(not(target_feature = "bmi"))]
#[inline(always)]
/// 最下位ビットを0にリセットして返す
///
/// # Arguments
/// * `v` - 操作対象のビット列
pub fn blsr64(v:u64) -> u64 {
    let v = v & (v - 1);
    v
}
/// 最下位ビットのビット位置を取り出して最下位ビットをリセットする。
///
/// # Arguments
/// * `bits` - ビット位置を取り出すビット列
#[inline]
pub fn pop_lsb(bits:&mut u64) -> u64 {
    let index = lsb64(*bits);

    *bits = blsr64(*bits);

    index
}
/// 最下位ビットのビット位置を取り出す
///
/// # Arguments
/// * `bits` - ビット位置を取り出すビット列
#[inline(always)]
pub fn lsb64(bits:u64) -> u64 {
    bits.trailing_zeros() as u64
}