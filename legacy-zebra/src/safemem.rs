/*
   File:       safemem.h

   Created:    August 30, 1998

   Modified:   January 25, 2000

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The interface to the safer version of malloc.
*/
/*
   File:            safemem.c

   Created:         August 30, 1998

   Modified:        November 1, 2000

   Author:          Gunnar Andersson (gunnar@radagast.se)

   Contents:        Provides safer memory allocation than malloc().
*/

use std::ffi::c_void;
use crate::src::error::{fatal_error_2};
use libc_wrapper::{malloc, realloc};

pub unsafe fn safe_malloc(size: u64) -> *mut c_void {
    let block = malloc(size);
    if block.is_null() {
        fatal_error_2(b"%s %d\n\x00" as *const u8 as *const i8,
                      b"Memory allocation failure when allocating\x00" as
                          *const u8 as *const i8, size);
    }
    block
}

pub unsafe fn safe_realloc(ptr: *mut c_void, size: u64) -> *mut c_void {
    let block = realloc(ptr, size);
    if block.is_null() {
        fatal_error_2(b"%s %d\n\x00" as *const u8 as *const i8,
                      b"Memory allocation failure when allocating\x00" as
                          *const u8 as *const i8, size);
    }
    block
}
