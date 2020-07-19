use crate::src::libc;
use crate::src::game::{FILE, time_t, size_t};
use crate::src::osfbook::__compar_fn_t;
use crate::src::getcoeff::gzFile;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    pub fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    pub static mut stdout: *mut FILE;
    #[no_mangle]
    pub fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
                 -> *mut FILE;
    #[no_mangle]
    pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
                   -> libc::c_int;
    #[no_mangle]
    pub fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    pub fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
                  -> libc::c_int;
    #[no_mangle]
    pub fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
                 -> *mut libc::c_char;
    #[no_mangle]
    pub fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    #[no_mangle]
    pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    pub fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    pub fn strstr(_: *const libc::c_char, _: *const libc::c_char)
                  -> *mut libc::c_char;
    #[no_mangle]
    pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
                      -> libc::c_int;
    #[no_mangle]
    pub fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    pub fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    #[no_mangle]
    pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    pub fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    pub fn toupper(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    pub static mut stderr: *mut FILE;
    #[no_mangle]
    pub fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;

    #[no_mangle]
    pub fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
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
    pub fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
              -> *mut libc::c_char;
    #[no_mangle]
    pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    pub static mut stdin: *mut FILE;
    #[no_mangle]
    pub fn vsprintf(_: *mut libc::c_char, _: *const libc::c_char,
                _: ::std::ffi::VaList) -> libc::c_int;
    #[no_mangle]
    pub fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    pub fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    pub fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
                -> libc::c_int;
    #[no_mangle]
    pub fn gzgetc(file: gzFile) -> libc::c_int;
    #[no_mangle]
    pub fn gzclose(file: gzFile) -> libc::c_int;
    #[no_mangle]
    pub fn gzopen(_: *const libc::c_char, _: *const libc::c_char) -> gzFile;
}
