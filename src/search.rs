use crate::src::moves::{unmake_move, make_move};
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
/* Global variables */
/*
  COMPLETE_PV
  Complete the principal variation with passes (if any there are any).
*/

pub unsafe fn complete_pv(mut side_to_move: i32) {
    let mut i: i32 = 0;
    let mut actual_side_to_move: [i32; 60] = [0; 60];
    full_pv_depth = 0 as i32;
    i = 0 as i32;
    while i < pv_depth[0 as i32 as usize] {
        if make_move(side_to_move, pv[0 as i32 as usize][i as usize],
                     1 as i32) != 0 {
            actual_side_to_move[i as usize] = side_to_move;
            full_pv[full_pv_depth as usize] =
                pv[0 as i32 as usize][i as usize];
            full_pv_depth += 1
        } else {
            full_pv[full_pv_depth as usize] = -(1 as i32);
            full_pv_depth += 1;
            side_to_move = 0 as i32 + 2 as i32 - side_to_move;
            if make_move(side_to_move,
                         pv[0 as i32 as usize][i as usize],
                         1 as i32) != 0 {
                actual_side_to_move[i as usize] = side_to_move;
                full_pv[full_pv_depth as usize] =
                    pv[0 as i32 as usize][i as usize];
                full_pv_depth += 1
            } else {
                let mut j: i32 = 0;
                printf(b"pv_depth[0] = %d\n\x00" as *const u8 as
                           *const i8,
                       pv_depth[0 as i32 as usize]);
                j = 0 as i32;
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
        }
        side_to_move = 0 as i32 + 2 as i32 - side_to_move;
        i += 1
    }
    i = pv_depth[0 as i32 as usize] - 1 as i32;
    while i >= 0 as i32 {
        unmake_move(actual_side_to_move[i as usize],
                    pv[0 as i32 as usize][i as usize]);
        i -= 1
    };
}
