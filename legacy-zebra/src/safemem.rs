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
use libc_wrapper::malloc;
#[macro_use]
use crate::fatal_error;

pub unsafe fn safe_malloc(size: u64) -> *mut c_void {
    let block = malloc(size);
    if block.is_null() {
        fatal_error!("{} {}\n", "Memory allocation failure when allocating", size);
    }
    block
}
