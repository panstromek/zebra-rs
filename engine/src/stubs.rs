pub use std::ffi::c_void;


#[inline(always)]
pub fn abs(num: i32) -> i32 {
    num.abs()
}
type __time_t = i64;
type time_t = __time_t;

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

extern "C" {
    #[no_mangle]
    pub fn malloc(_: u64) -> *mut c_void;
    #[no_mangle]
    pub fn realloc(_: *mut c_void, _: u64) -> *mut c_void;
    #[no_mangle]
    pub fn free(__ptr: *mut c_void);
    #[no_mangle]
    pub fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    pub fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    pub fn tolower(num: i32) -> i32;
    #[no_mangle]
    pub fn toupper(_: i32) -> i32;
    #[no_mangle]
    pub fn strdup(_: *const i8) -> *mut i8;
    #[no_mangle]
    pub fn strchr(_: *const i8, _: i32) -> *mut i8;
}
