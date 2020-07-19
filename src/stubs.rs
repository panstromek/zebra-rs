use crate::src::libc;
use crate::src::game::{FILE, time_t, size_t};
use crate::src::osfbook::__compar_fn_t;
use crate::src::getcoeff::gzFile;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    pub fn tolower(_: i32) -> i32;
    #[no_mangle]
    pub fn floor(_: f64) -> f64;
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
    pub fn strchr(_: *const i8, _: i32) -> *mut i8;
    #[no_mangle]
    pub fn strstr(_: *const i8, _: *const i8)
                  -> *mut i8;
    #[no_mangle]
    pub fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    pub fn strcasecmp(_: *const i8, _: *const i8)
                      -> i32;
    #[no_mangle]
    pub fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    pub fn ctime(__timer: *const time_t) -> *mut i8;
    #[no_mangle]
    pub fn malloc(_: u64) -> *mut libc::c_void;
    #[no_mangle]
    pub fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    pub fn fabs(_: f64) -> f64;
    #[no_mangle]
    pub fn __ctype_b_loc() -> *mut *const u16;
    #[no_mangle]
    pub fn toupper(_: i32) -> i32;
    #[no_mangle]
    pub fn ceil(_: f64) -> f64;
    #[no_mangle]
    pub static mut stderr: *mut FILE;
    #[no_mangle]
    pub fn fflush(__stream: *mut FILE) -> i32;
    #[no_mangle]
    pub fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;

    #[no_mangle]
    pub fn putc(__c: i32, __stream: *mut FILE) -> i32;
    #[no_mangle]
    pub fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    pub fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    pub fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    pub fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    pub fn abs(_: i32) -> i32;
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
    pub fn strdup(_: *const i8) -> *mut i8;
    #[no_mangle]
    pub fn __assert_fail(__assertion: *const i8,
                     __file: *const i8, __line: u32,
                     __function: *const i8) -> !;
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
