use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CounterType {
    pub hi: libc::c_uint,
    pub lo: libc::c_uint,
}
/*
  RESET_COUNTER
*/
#[no_mangle]
pub unsafe extern "C" fn reset_counter(mut counter: *mut CounterType) {
    (*counter).lo = 0 as libc::c_int as libc::c_uint;
    (*counter).hi = 0 as libc::c_int as libc::c_uint;
}
/*
  ADJUST_COUNTER
  Makes sure that the LO part of the counter only contains 8 decimal digits.
*/
#[no_mangle]
pub unsafe extern "C" fn adjust_counter(mut counter: *mut CounterType) {
    while (*counter).lo >= 100000000 as libc::c_int as libc::c_uint {
        (*counter).lo =
            (*counter).lo.wrapping_sub(100000000 as libc::c_int as
                                           libc::c_uint);
        (*counter).hi = (*counter).hi.wrapping_add(1)
    };
}
/*
  COUNTER_VALUE
  Converts a counter to a single floating-point value.
*/
#[no_mangle]
pub unsafe extern "C" fn counter_value(mut counter: *mut CounterType)
 -> libc::c_double {
    adjust_counter(counter);
    return 100000000 as libc::c_int as libc::c_double *
               (*counter).hi as libc::c_double +
               (*counter).lo as libc::c_double;
}
/*
  ADD_COUNTER
  Adds the value of the counter TERM to the counter SUM.
*/
#[no_mangle]
pub unsafe extern "C" fn add_counter(mut sum: *mut CounterType,
                                     mut term: *mut CounterType) {
    (*sum).lo = (*sum).lo.wrapping_add((*term).lo);
    (*sum).hi = (*sum).hi.wrapping_add((*term).hi);
    adjust_counter(sum);
}
