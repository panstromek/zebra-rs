#![feature(c_variadic)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::ffi::{c_void, CStr};
use std::io::Write;
use std::ptr::null_mut;

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

pub unsafe fn strcasecmp<T: AsConstPtr<i8>, U: AsConstPtr<i8>>(a: T, b: U) -> i32
{
    inner::strcasecmp(a.as_const_ptr(), b.as_const_ptr())
}

mod inner  {
    use crate::{_IO_FILE, size_t};
    pub type FILE = _IO_FILE;

    extern "C" {
        pub fn atoi(__nptr: *const i8) -> i32;
        pub fn strcasecmp(_: *const i8, _: *const i8) -> i32;

        pub static mut stdin: *mut FILE;
        pub fn getc(__stream: *mut FILE) -> i32;
        pub static mut stdout: *mut FILE;
        pub fn fclose(__stream: *mut FILE) -> i32;
        pub fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
        pub fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
        pub fn fputc(__c: i32, __stream: *mut FILE) -> i32;
        pub fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
        pub fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
        pub fn feof(__stream: *mut FILE) -> i32;
        pub static mut stderr: *mut FILE;
        pub fn fflush(__stream: *mut FILE) -> i32;
        pub fn putc(__c: i32, __stream: *mut FILE) -> i32;
        pub fn fread(__ptr: *mut std::ffi::c_void, __size: size_t, __n: size_t,
                     __stream: *mut FILE) -> size_t;
        pub fn fwrite(__ptr: *const std::ffi::c_void, __size: size_t, __n: size_t,
                      __s: *mut FILE) -> size_t;

        pub fn vfprintf(_: *mut FILE, _: *const i8, _: ::std::ffi::VaList)
                        -> i32;
    }
}
extern "C" {
    pub fn fscanf(_: *mut inner::FILE, _: *const i8, _: ...) -> i32;
    pub fn malloc(_: u64) -> *mut c_void;
    pub fn realloc(_: *mut c_void, _: u64) -> *mut c_void;

    pub fn free(__ptr: *mut c_void);
    pub fn time(__timer: *mut time_t) -> time_t;
    pub fn strlen(_: *const i8) -> u64;
    pub fn tolower(num: i32) -> i32;
    pub fn toupper(_: i32) -> i32;
    pub fn strchr(_: *const i8, _: i32) -> *mut i8;


    pub fn printf(_: *const i8, _: ...) -> i32;
    pub fn sprintf(_: *mut i8, _: *const i8, _: ...)
                   -> i32;
    pub fn scanf(_: *const i8, _: ...) -> i32;
    pub fn sscanf(_: *const i8, _: *const i8, _: ...)
                  -> i32;
    pub fn puts(__s: *const i8) -> i32;


    pub fn exit(_: i32) -> !;
    pub fn strstr(_: *const i8, _: *const i8)
                  -> *mut i8;
    pub fn ctime(__timer: *const time_t) -> *mut i8;
    pub fn __ctype_b_loc() -> *mut *const u16;

    pub fn qsort(__base: *mut std::ffi::c_void, __nmemb: size_t, __size: size_t,
                 __compar: __compar_fn_t);
    pub fn strcpy(_: *mut i8, _: *const i8)
                  -> *mut i8;
    pub fn strcmp(_: *const i8, _: *const i8) -> i32;
    pub fn gzgetc(file: gzFile) -> i32;
    pub fn gzclose(file: gzFile) -> i32;
    pub fn gzopen(_: *const i8, _: *const i8) -> gzFile;
}
pub static mut stdin: FileHandle = FileHandle::StdIn;
pub static mut stdout: FileHandle = FileHandle::StdOut;
pub static mut stderr: FileHandle = FileHandle::StdErr;

#[derive(Copy, Clone)]
#[repr(C)]
pub enum FileHandle {
    StdIn,
    StdOut,
    StdErr,
    File(*mut inner::FILE)
}
impl FileHandle {
    pub fn file(self) -> *mut inner::FILE {
        match self {
            Self::File(f) => f,
            FileHandle::StdIn => { unsafe { inner::stdin } }
            FileHandle::StdOut => { unsafe { inner::stdout } }
            FileHandle::StdErr => { unsafe { inner::stderr } }
        }
    }
    pub fn is_null(self) -> bool {
        if let Self::File(val) = self {
            return val.is_null();
        }
        return false;
    }

    pub fn null() -> Self {
        Self::File(null_mut())
    }
}
impl Write for FileHandle {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.is_null() {
            return Ok(0)
        }
        let written = unsafe {
            inner::fwrite(
                buf.as_ptr() as *const std::ffi::c_void,
                1,
                buf.len() as u64,
                self.file(),
            )
        };
        // TODO return result based on written
        Ok(written as usize)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if !self.is_null() {
            let _success = unsafe { inner::fflush(self.file()) };
        }
        return Ok(()) // todo return error based on success
    }
}

#[no_mangle]
pub unsafe extern "C" fn __wrap_time(__timer: *mut time_t) -> time_t {
    static mut time: time_t = 100;
    time += 1;
    let result = time / 100;
    if __timer != null_mut() {
        *__timer = result;
    }
    return result;
}

pub unsafe fn getc(__stream: FileHandle) -> i32 {
    return inner::getc((__stream).file())
}

pub unsafe fn fclose(__stream: FileHandle) -> i32 {
    inner::fclose(__stream.file())
}
pub unsafe fn fopen(__filename: *const i8, __modes: *const i8) -> FileHandle {
    let f: *mut inner::FILE = inner::fopen(__filename, __modes);
    FileHandle::File(f)
}
pub unsafe extern "C" fn fprintf(__stream: FileHandle, __format: *const i8, args: ...) -> i32 {
    let mut arg_ptr = args.clone();
    inner::vfprintf((__stream).file(), __format, arg_ptr.as_va_list())
}
pub unsafe fn fputc(__c: i32, __stream: FileHandle) -> i32 {
    inner::fputc(__c, (__stream).file())
}

pub unsafe fn fgets(__s: *mut i8, __n: i32, __stream: FileHandle) -> *mut i8 {
    inner::fgets(__s, __n, (__stream).file())
}

pub unsafe fn fputs(__s: *const i8, __stream: FileHandle) -> i32 {
    inner::fputs(__s, (__stream).file())
}

pub unsafe fn feof(__stream: FileHandle) -> i32 {
    inner::feof((__stream).file())
}

pub unsafe fn fflush(__stream: FileHandle) -> i32 {
    inner::fflush((__stream).file())
}

pub unsafe fn putc(__c: i32, __stream: FileHandle) -> i32 {
    inner::putc(__c, (__stream).file())
}

pub unsafe fn fread(__ptr: *mut std::ffi::c_void, __size: size_t, __n: size_t, __stream: FileHandle) -> size_t {
    inner::fread(__ptr, __size, __n, (__stream).file())
}

pub unsafe fn fwrite(__ptr: *const std::ffi::c_void, __size: size_t, __n: size_t, __s: FileHandle) -> size_t {
    inner::fwrite(__ptr, __size, __n, (__s).file())
}
pub fn c_time(timer: i64) -> &'static str{
    &unsafe { CStr::from_ptr(ctime(&timer)) }.to_str().unwrap()
}