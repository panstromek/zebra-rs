use crate::src::libc;
use crate::src::stubs::{vfprintf, ctime, fprintf, time, fopen, stderr, exit};
use crate::src::zebra::_IO_FILE;

pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type time_t = __time_t;
/*
   File:       error.h

   Created:    June 13, 1998

   Modified:   August 1, 2002

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The interface to the error handler.
*/
/*
   File:       error.c

   Created:    June 13, 1998

   Modified:   November 12, 2001

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The text-based error handler.
*/
/* not Windows CE */
#[no_mangle]
pub unsafe extern "C" fn fatal_error(mut format: *const i8,
                                     mut args: ...) {
    let mut stream = 0 as *mut FILE;
    let mut timer: time_t = 0;
    let mut arg_ptr: ::std::ffi::VaListImpl;
    arg_ptr = args.clone();
    fprintf(stderr, b"\n%s: \x00" as *const u8 as *const i8,
            b"Fatal error\x00" as *const u8 as *const i8);
    vfprintf(stderr, format, arg_ptr.as_va_list());
    stream =
        fopen(b"zebra.err\x00" as *const u8 as *const i8,
              b"a\x00" as *const u8 as *const i8);
    if !stream.is_null() {
        time(&mut timer);
        fprintf(stream,
                b"%s @ %s\n  \x00" as *const u8 as *const i8,
                b"Fatal error\x00" as *const u8 as *const i8,
                ctime(&mut timer));
        arg_ptr = args.clone();
        vfprintf(stream, format, arg_ptr.as_va_list());
    }
    exit(1 as i32);
}
