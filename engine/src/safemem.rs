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
use crate::src::stubs::{malloc, realloc};
use crate::src::error::fatal_error;
use std::ffi::c_void;

pub type size_t = u64;
pub unsafe fn safe_malloc(size: size_t) -> *mut c_void {
    let mut block = 0 as *mut c_void;
    block = malloc(size);
    if block.is_null() {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Memory allocation failure when allocating\x00" as
                        *const u8 as *const i8, size);
    }
    return block;
}

pub unsafe fn safe_realloc(ptr: *mut c_void,
                                      size: size_t) -> *mut c_void {
    let mut block = 0 as *mut c_void;
    block = realloc(ptr, size);
    if block.is_null() {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Memory allocation failure when allocating\x00" as
                        *const u8 as *const i8, size);
    }
    return block;
}
