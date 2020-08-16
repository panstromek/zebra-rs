
use crate::src::globals::{pv_depth, pv};
use crate::src::error::fatal_error;
use crate::src::stubs::{printf, puts};
pub use ::engine::src::search::*;

/*
   File:          search.c

   Created:       July 1, 1997

   Modified:      January 2, 2003

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      Common search routines and variables.
*/
pub unsafe fn handle_fatal_pv_error(i: i32) {
    printf(b"pv_depth[0] = %d\n\x00" as *const u8 as
               *const i8,
           pv_depth[0 as i32 as usize]);
    let mut j = 0 as i32;
    while j < pv_depth[0 as i32 as usize] {
        printf(b"%c%c \x00" as *const u8 as *const i8,
               'a' as i32 +
                   pv[0 as i32 as usize][j as usize] %
                       10 as i32 - 1 as i32,
               '0' as i32 +
                   pv[0 as i32 as usize][j as usize] /
                       10 as i32);
        j += 1
    }
    puts(b"\x00" as *const u8 as *const i8);
    printf(b"i=%d\n\x00" as *const u8 as *const i8, i);
    fatal_error(b"Error in PV completion\x00" as *const u8 as
        *const i8);
}
