use crate::src::game::{FILE, time_t, size_t};
use crate::src::osfbook::__compar_fn_t;
use crate::src::getcoeff::gzFile;
use engine::src::stubs::{abs, floor, ceil, fabs};
use std::ffi::c_void;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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

    #[no_mangle]
    pub static mut stdout: *mut FILE;
    #[no_mangle]
    pub fn fclose(__stream: *mut FILE) -> i32;
    #[no_mangle]
    pub fn fopen(__filename: *const i8, __modes: *const i8)
                 -> *mut FILE;
    #[no_mangle]
    pub fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    pub fn printf(_: *const i8, _: ...) -> i32;
    #[no_mangle]
    pub fn sprintf(_: *mut i8, _: *const i8, _: ...)
                   -> i32;
    #[no_mangle]
    pub fn scanf(_: *const i8, _: ...) -> i32;
    #[no_mangle]
    pub fn sscanf(_: *const i8, _: *const i8, _: ...)
                  -> i32;
    #[no_mangle]
    pub fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    #[no_mangle]
    pub fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE)
                 -> *mut i8;
    #[no_mangle]
    pub fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    #[no_mangle]
    pub fn puts(__s: *const i8) -> i32;
    #[no_mangle]
    pub fn feof(__stream: *mut FILE) -> i32;
    #[no_mangle]
    pub fn atof(__nptr: *const i8) -> f64;
    #[no_mangle]
    pub fn atoi(__nptr: *const i8) -> i32;
    #[no_mangle]
    pub fn exit(_: i32) -> !;
    #[no_mangle]
    pub fn strstr(_: *const i8, _: *const i8)
                  -> *mut i8;
    #[no_mangle]
    pub fn strcasecmp(_: *const i8, _: *const i8)
                      -> i32;
    #[no_mangle]
    pub fn ctime(__timer: *const time_t) -> *mut i8;
    #[no_mangle]
    pub fn __ctype_b_loc() -> *mut *const u16;
    #[no_mangle]
    pub static mut stderr: *mut FILE;
    #[no_mangle]
    pub fn fflush(__stream: *mut FILE) -> i32;
    #[no_mangle]
    pub fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    pub fn putc(__c: i32, __stream: *mut FILE) -> i32;
    #[no_mangle]
    pub fn fread(__ptr: *mut std::ffi::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    pub fn fwrite(__ptr: *const std::ffi::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    pub fn qsort(__base: *mut std::ffi::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    pub fn strcpy(_: *mut i8, _: *const i8)
              -> *mut i8;
    #[no_mangle]
    pub fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    pub static mut stdin: *mut FILE;
    #[no_mangle]
    pub fn vsprintf(_: *mut i8, _: *const i8,
                _: ::std::ffi::VaList) -> i32;
    #[no_mangle]
    pub fn getc(__stream: *mut FILE) -> i32;
    #[no_mangle]
    pub fn vfprintf(_: *mut FILE, _: *const i8, _: ::std::ffi::VaList)
                -> i32;
    #[no_mangle]
    pub fn gzgetc(file: gzFile) -> i32;
    #[no_mangle]
    pub fn gzclose(file: gzFile) -> i32;
    #[no_mangle]
    pub fn gzopen(_: *const i8, _: *const i8) -> gzFile;
}
// // Use these in rust version
// // If there is a problem with these things, I can always just link to libc if I need to
// #[no_mangle]
// pub unsafe extern "C" fn malloc(len: u64) -> *mut c_void {
//     // This is a really primitive version
//     // of malloc that just uses a Vec to allocate the storage
//     // We could do better but this is just a stub so let's be ok with it
//     // we use u64 because that is the alignment malloc usually gives you
//     // as "sufficient"
//     let mut vec: Vec<u64> = Vec::new();
//     let size = std::mem::size_of::<u64>();
//     // division will truncate the length if it's not a multiple of size_of::u64,
//     // so we need to add one to round it up
//     let count = len as usize / size + 2;
//     vec.resize(count, 0u64);
//     vec[0] = (count) as u64;
//     // we store the size in the first byte because
//     // we need it in realloc or free
//     let raw = Box::into_raw(vec.into_boxed_slice());
//     (raw as *mut u64).offset(1) as *mut c_void
// }
//
// #[no_mangle]
// pub unsafe extern "C" fn realloc(mem: *mut c_void, num_bytes: u64) -> *mut c_void {
//     let start = (mem as *mut u64).offset(-1);
//     let size = (*start) as usize;
//     let mut vec = Vec::from_raw_parts(start, size, size);
//
//     let size = num_bytes as usize / std::mem::size_of::<u64>() + 2;
//     vec.resize(size, 0u64);
//     vec[0] = (size) as u64;
//     let raw = Box::into_raw(vec.into_boxed_slice());
//     (raw as *mut u64).offset(1) as *mut c_void
// }
//
// #[no_mangle]
// pub unsafe extern "C" fn free(mem: *mut c_void) {
//     let start = (mem as *mut u64).offset(-1);
//     let size = (*start) as usize;
//     let vec = Vec::from_raw_parts(start, size, size);
//     drop(vec)
// }


// TODO do a crosscheck with this. So far I only implemented
//  it for ascii and I am not sure if more is needed
// pub fn tolower(num: i32) -> i32 {
//     if num >= 'A' as i32 && num <= 'Z' as i32 {
//         let offset = ('a' as i32) - ('A' as i32);
//         return num - offset;
//     }
//     return num;
// }

// pub unsafe extern "C" fn strlen(string: *const i8) -> u64 {
//     // std::sys::strlen(string)
//     std::ffi::CStr::from_ptr(string).to_bytes().len() as u64
// }