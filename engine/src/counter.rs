#[derive(Copy, Clone)]
#[repr(C)]
pub struct CounterType {
    pub hi: u32,
    pub lo: u32,
}
/*
  RESET_COUNTER
*/

pub fn reset_counter(counter: &mut CounterType) {
    counter.lo = 0;
    counter.hi = 0;
}
/*
  ADJUST_COUNTER
  Makes sure that the LO part of the counter only contains 8 decimal digits.
*/

pub fn adjust_counter(counter: &mut CounterType) {
    while counter.lo >= 100000000 {
        counter.lo = counter.lo.wrapping_sub(100000000);
        counter.hi = counter.hi.wrapping_add(1)
    };
}
/*
  COUNTER_VALUE
  Converts a counter to a single floating-point value.
*/

pub fn counter_value(counter: &mut CounterType)
 -> f64 {
    adjust_counter(counter);
    100000000f64 * counter.hi as f64 + counter.lo as f64
}
/*
  ADD_COUNTER
  Adds the value of the counter TERM to the counter SUM.
*/
pub fn add_counter(sum: &mut CounterType, term: &CounterType) {
    sum.lo = sum.lo.wrapping_add(term.lo);
    sum.hi = sum.hi.wrapping_add(term.hi);
    adjust_counter(sum);
}
