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
use crate::src::error::{FrontEnd};
use std::ffi::c_void;

pub unsafe fn safe_malloc<FE: FrontEnd>(size: u64) -> *mut c_void {
    let block = FE::malloc(size);
    if block.is_null() {
        FE::safe_malloc_failure(size);
    }
    block
}

pub unsafe fn safe_realloc<FE: FrontEnd>(ptr: *mut c_void, size: u64) -> *mut c_void {
    let block = FE::realloc(ptr, size);
    if block.is_null() {
        FE::safe_realloc_failure(size);
    }
    block
}
