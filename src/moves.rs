use crate::src::stubs::{atoi, printf, scanf};
pub use engine::src::moves::*;

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
pub unsafe fn get_move(side_to_move: i32) -> i32 {
    let mut buffer: [i8; 255] = [0; 255];
    let mut ready = 0 as i32;
    let mut curr_move: i32 = 0;
    while ready == 0 {
        prompt_get_move(side_to_move, &mut buffer);
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
    curr_move
}

fn prompt_get_move(side_to_move: i32, buffer: &mut [i8; 255]) -> i32 {
    unsafe {
        if side_to_move == 0 as i32 {
            printf(b"%s: \x00" as *const u8 as *const i8,
                   b"Black move\x00" as *const u8 as *const i8);
        } else {
            printf(b"%s: \x00" as *const u8 as *const i8,
                   b"White move\x00" as *const u8 as *const i8);
        }
        scanf(b"%s\x00" as *const u8 as *const i8, buffer.as_mut_ptr());
        atoi(buffer.as_mut_ptr())
    }
}
