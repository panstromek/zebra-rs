pub use std::ffi::c_void;
use std::alloc::{Layout, LayoutErr};

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

pub unsafe fn malloc(len: u64) -> *mut c_void {
    // This is a really primitive version
    // of malloc that just uses a Vec to allocate the storage
    // We could do better but this is just a stub so let's be ok with it
    // we use u64 because that is the alignment malloc usually gives you
    // as "sufficient"
    let mut vec: Vec<u64> = Vec::new();
    let size = std::mem::size_of::<u64>();
    // division will truncate the length if it's not a multiple of size_of::u64,
    // so we need to add one to round it up
    let count = len as usize / size + 2;
    vec.resize(count, 0u64);
    vec[0] = (count) as u64;
    // we store the size in the first byte because
    // we need it in realloc or free
    let raw = Box::into_raw(vec.into_boxed_slice());
    (raw as *mut u64).offset(1) as *mut c_void
}

pub unsafe fn realloc(mem: *mut c_void, num_bytes: u64) -> *mut c_void {
    let start = (mem as *mut u64).offset(-1);
    let size = (*start) as usize;
    let mut vec = Vec::from_raw_parts(start, size, size);

    let size = num_bytes as usize / std::mem::size_of::<u64>() + 2;
    vec.resize(size, 0u64);
    vec[0] = (size) as u64;
    let raw = Box::into_raw(vec.into_boxed_slice());
    (raw as *mut u64).offset(1) as *mut c_void
}

pub unsafe fn free(mem: *mut c_void) {
    let start = (mem as *mut u64).offset(-1);
    let size = (*start) as usize;
    let vec = Vec::from_raw_parts(start, size, size);
    drop(vec)
}
