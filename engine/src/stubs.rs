#[inline(always)]
pub fn abs(num: i32) -> i32 {
    num.abs()
}

// FIXME verify: are these replacements equivalent to their libc counterparts???
//  if not, does it matter??
#[inline(always)]
pub fn floor(num: f64) -> f64{
    num.floor()
}
#[inline(always)]
pub fn fabs(num: f64) -> f64 {
    num.abs()
}
#[inline(always)]
pub fn ceil(num: f64) -> f64 {
    num.ceil()
}
