#![feature(c_variadic)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]

use std::ffi::{c_void, CStr};
use std::io::{Write, ErrorKind};
use std::ptr::null_mut;
use std::sync::atomic::{AtomicI64, Ordering};

extern crate libc;



pub type _IO_wide_data = std::ffi::c_void;
pub type _IO_codecvt = std::ffi::c_void;
pub type _IO_marker = std::ffi::c_void;

pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type size_t = u64;
pub type _IO_FILE = libc::FILE;
pub type time_t = __time_t;
pub type off_t = __off_t;

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
    use crate::{_IO_FILE};
    pub type FILE = _IO_FILE;

    pub use libc::{atoi, strcasecmp, fclose, fopen, fprintf, fputc, fgets, fputs, feof, fflush, fread, fwrite};
    extern "C" {
        pub static mut stdin: *mut FILE;
        pub static mut stdout: *mut FILE;
        pub static mut stderr: *mut FILE;
        pub fn vfprintf(_: *mut FILE, _: *const i8, _: ::std::ffi::VaList) -> i32;
    }
}
pub use libc::{
    atof,
    fscanf,
    malloc,
    realloc,
    free,
    time,
    strlen,
    tolower,
    toupper,
    strchr,
    printf,
    sprintf,
    scanf,
    sscanf,
    puts,
    exit,
    strstr,
    qsort,
    strcpy,
    strcmp,
    isalnum
};

extern "C" {
    pub fn ctime(__timer: *const time_t) -> *mut i8;
}

pub use FileHandle::StdIn as stdin;
pub use FileHandle::StdOut as stdout;
pub use FileHandle::StdErr as stderr;

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
impl std::io::Read for FileHandle {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.file().is_null() {
            return Ok(0);
        }

        let count  = unsafe {
            inner::fread(buf.as_ptr() as *mut std::ffi::c_void, 1, buf.len(), self.file())
        };
        if count <= buf.len() {
            Ok(count as usize)
        } else {
            Err(std::io::Error::new(ErrorKind::Other, "Bytes read is out of bounds"))
        }
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
                buf.len(),
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
    static time: AtomicI64 = AtomicI64::new(100);
    let result = time.fetch_add(1, Ordering::SeqCst) + 1;
    let result = result / 100;
    if __timer != null_mut() {
        *__timer = result;
    }
    return result;
}


pub unsafe fn fclose(__stream: FileHandle) -> i32 {
    inner::fclose(__stream.file())
}
pub unsafe fn fopen(__filename: *const i8, __modes: *const i8) -> FileHandle {
    let f: *mut inner::FILE = inner::fopen(__filename, __modes);
    FileHandle::File(f)
}
pub unsafe extern "C" fn fprintf(__stream: FileHandle, __format: *const i8, args: ...) -> i32 {
    let arg_ptr = args.clone();
    inner::vfprintf((__stream).file(), __format, arg_ptr)
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
    fputc(__c, __stream)
}

pub unsafe fn fread(__ptr: *mut std::ffi::c_void, __size: size_t, __n: size_t, __stream: FileHandle) -> size_t {
    inner::fread(__ptr, __size as _, __n as _, (__stream).file()) as _
}

pub unsafe fn fwrite(__ptr: *const std::ffi::c_void, __size: size_t, __n: size_t, __s: FileHandle) -> size_t {
    inner::fwrite(__ptr, __size as _, __n as _, (__s).file()) as _
}
pub unsafe fn c_time(timer: i64) -> &'static str{
    &CStr::from_ptr(ctime(&timer)).to_str().unwrap()
}