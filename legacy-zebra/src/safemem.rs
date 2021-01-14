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
use crate::src::error::LibcFatalError;

pub unsafe fn safe_malloc(size: u64) -> *mut c_void {
    let block = LibcFatalError::malloc(size);
    if block.is_null() {
        LibcFatalError::safe_malloc_failure(size);
    }
    block
}

pub unsafe fn safe_realloc(ptr: *mut c_void, size: u64) -> *mut c_void {
    let block = LibcFatalError::realloc(ptr, size);
    if block.is_null() {
        LibcFatalError::safe_realloc_failure(size);
    }
    block
}
