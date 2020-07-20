use crate::src::stubs::{atoi, printf, scanf};
use crate::src::doflip::{DoFlips_no_hash, hash_update2, hash_update1, DoFlips_hash};
use crate::src::cntflip::AnyFlips_compact;
use crate::src::globals::{board, piece_count};
use crate::src::unflip::flip_stack;
use crate::src::hash::{hash_stored2, hash2, hash_stored1, hash1, hash_put_value2, hash_put_value1};
use crate::src::search::sorted_move_order;
pub use engine::src::moves::*;
/*
   File:           globals.h

   Created:        June 30, 1997

   Modified:       January 8, 2000

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Global state variables.
*/
/* The basic board type. One index for each position;
   a1=11, h1=18, a8=81, h8=88. */
/*
   File:              moves.c

   Created:           June 30, 1997

   Modified:          April 24, 2001

   Author:            Gunnar Andersson (gunnar@radagast.se)

   Contents:          The move generator.
*/

/*
   GET_MOVE
   Prompts the user to enter a move and checks if the move is legal.
*/

pub unsafe fn get_move(mut side_to_move: i32)
 -> i32 {
    let mut buffer: [i8; 255] = [0; 255];
    let mut ready = 0 as i32;
    let mut curr_move: i32 = 0;
    while ready == 0 {
        if side_to_move == 0 as i32 {
            printf(b"%s: \x00" as *const u8 as *const i8,
                   b"Black move\x00" as *const u8 as *const i8);
        } else {
            printf(b"%s: \x00" as *const u8 as *const i8,
                   b"White move\x00" as *const u8 as *const i8);
        }
        scanf(b"%s\x00" as *const u8 as *const i8,
              buffer.as_mut_ptr());
        curr_move = atoi(buffer.as_mut_ptr());
        ready = valid_move(curr_move, side_to_move);
        if ready == 0 {
            curr_move =
                buffer[0 as i32 as usize] as i32 - 'a' as i32
                    + 1 as i32 +
                    10 as i32 *
                        (buffer[1 as i32 as usize] as i32 -
                             '0' as i32);
            ready = valid_move(curr_move, side_to_move)
        }
    }
    return curr_move;
}
