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
use crate::src::stubs::*;
use crate::src::error::fatal_error;
use crate::src::libc;

pub type size_t = libc::c_ulong;
pub unsafe fn safe_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut block = 0 as *mut libc::c_void;
    block = malloc(size);
    if block.is_null() {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const libc::c_char,
                    b"Memory allocation failure when allocating\x00" as
                        *const u8 as *const libc::c_char, size);
    }
    return block;
}

pub unsafe fn safe_realloc(mut ptr: *mut libc::c_void,
                                      mut size: size_t) -> *mut libc::c_void {
    let mut block = 0 as *mut libc::c_void;
    block = realloc(ptr, size);
    if block.is_null() {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const libc::c_char,
                    b"Memory allocation failure when allocating\x00" as
                        *const u8 as *const libc::c_char, size);
    }
    return block;
}
