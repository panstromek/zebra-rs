#![feature(c_variadic)]
// #![feature(extern_types)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::ffi::{c_void, CString, CStr};
extern crate libc;



pub type _IO_wide_data = std::ffi::c_void;
pub type _IO_codecvt = std::ffi::c_void;
pub type _IO_marker = std::ffi::c_void;

pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut std::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type time_t = __time_t;
pub type off_t = __off_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: u32,
    pub next: *mut u8,
    pub pos: off_t,
}
pub type gzFile = *mut gzFile_s;
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type __compar_fn_t =  Option<unsafe extern "C" fn(_: *const c_void, _: *const c_void) -> i32>;

/// #SAFETY implementor must ensure that this pointer is valid
pub trait AsConstPtr<T: Sized> {
     fn as_const_ptr(self) -> *const T;
}
impl AsConstPtr<i8> for *const i8 {
    fn as_const_ptr(self) -> *const i8 {
        self
    }
}
impl AsConstPtr<i8> for *mut i8 {
    fn as_const_ptr(self) -> *const i8 {
        self
    }
}

impl AsConstPtr<i8> for &CStr {
    fn as_const_ptr(self) -> *const i8 {
        self.as_ptr()
    }
}

pub unsafe fn atoi<T: AsConstPtr<i8>>(__nptr: T) -> i32 {
    inner::atoi(__nptr.as_const_ptr())
}

pub unsafe fn atof<T: AsConstPtr<i8>>(__nptr: T) -> f64 {
    inner::atof(__nptr.as_const_ptr())
}

pub unsafe fn strcasecmp<T: AsConstPtr<i8>, U: AsConstPtr<i8>>(a: T, b: U) -> i32
{
    inner::strcasecmp(a.as_const_ptr(), b.as_const_ptr())
}

mod inner  {
    extern "C" {
        pub fn atoi(__nptr: *const i8) -> i32;
        pub fn atof(__nptr: *const i8) -> f64;
        pub fn strcasecmp(_: *const i8, _: *const i8) -> i32;

    }
}
extern "C" {
    pub fn malloc(_: u64) -> *mut c_void;
    pub fn realloc(_: *mut c_void, _: u64) -> *mut c_void;

    pub fn free(__ptr: *mut c_void);
    pub fn time(__timer: *mut time_t) -> time_t;
    pub fn strlen(_: *const i8) -> u64;
    pub fn tolower(num: i32) -> i32;
    pub fn toupper(_: i32) -> i32;
    pub fn strdup(_: *const i8) -> *mut i8;
    pub fn strchr(_: *const i8, _: i32) -> *mut i8;

    pub static mut stdout: *mut FILE;
    pub fn fclose(__stream: *mut FILE) -> i32;
    pub fn fopen(__filename: *const i8, __modes: *const i8)
                 -> *mut FILE;
    pub fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    pub fn printf(_: *const i8, _: ...) -> i32;
    pub fn sprintf(_: *mut i8, _: *const i8, _: ...)
                   -> i32;
    pub fn scanf(_: *const i8, _: ...) -> i32;
    pub fn sscanf(_: *const i8, _: *const i8, _: ...)
                  -> i32;
    pub fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    pub fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE)
                 -> *mut i8;
    pub fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    pub fn puts(__s: *const i8) -> i32;
    pub fn feof(__stream: *mut FILE) -> i32;
    pub fn exit(_: i32) -> !;
    pub fn strstr(_: *const i8, _: *const i8)
                  -> *mut i8;
    pub fn ctime(__timer: *const time_t) -> *mut i8;
    pub fn __ctype_b_loc() -> *mut *const u16;
    pub static mut stderr: *mut FILE;
    pub fn fflush(__stream: *mut FILE) -> i32;
    pub fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    pub fn putc(__c: i32, __stream: *mut FILE) -> i32;
    pub fn fread(__ptr: *mut std::ffi::c_void, __size: size_t, __n: size_t,
                 __stream: *mut FILE) -> size_t;
    pub fn fwrite(__ptr: *const std::ffi::c_void, __size: size_t, __n: size_t,
                  __s: *mut FILE) -> size_t;
    pub fn qsort(__base: *mut std::ffi::c_void, __nmemb: size_t, __size: size_t,
                 __compar: __compar_fn_t);
    pub fn strcpy(_: *mut i8, _: *const i8)
                  -> *mut i8;
    pub fn strcmp(_: *const i8, _: *const i8) -> i32;
    pub static mut stdin: *mut FILE;
    pub fn vsprintf(_: *mut i8, _: *const i8,
                    _: ::std::ffi::VaList) -> i32;
    pub fn getc(__stream: *mut FILE) -> i32;
    pub fn vfprintf(_: *mut FILE, _: *const i8, _: ::std::ffi::VaList)
                    -> i32;
    pub fn gzgetc(file: gzFile) -> i32;
    pub fn gzclose(file: gzFile) -> i32;
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