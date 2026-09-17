#[inline(always)]
pub fn align_up(a: u64, b: u64) -> u64 {
    (a + b - 1) & !(b - 1)
}

#[inline(always)]
pub fn align_down(a: u64, b: u64) -> u64 {
    (a) & !((b) - 1)
}

#[inline(always)]
pub fn align(a: u64, b: u64) -> u64 {
    align_up(a, b)
}
