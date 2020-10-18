/*
   File:           unflip.c

   Created:        February 26, 1999

   Modified:       July 12, 1999

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Low-level code to flip back the discs flipped by a move.
*/

use engine_traits::Offset;

pub static mut global_flip_stack: [usize; 2048] = [0; 2048];

pub static mut flip_stack: usize = 0;
/*
   File:          unflip.h

   Created:       February 26, 1999

   Modified:      December 25, 1999

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      Low-level macro code to flip back the discs
                  flipped by a move.
*/
/*
  UNDOFLIPS
  Flip back the FLIP_COUNT topmost squares on GLOBALFLIPSTACK.
  This seems to be the fastest way to do it and doesn't matter
  much anyway as this function consumes a negligible percentage
  of the time.
*/

pub unsafe fn UndoFlips(board: &mut [i32; 128], flip_count: i32, oppcol: i32) {
    let mut UndoFlips__flip_count = flip_count;
    let UndoFlips__oppcol = oppcol;
    if UndoFlips__flip_count & 1 as i32 != 0 {
        UndoFlips__flip_count -= 1;
        flip_stack = flip_stack -1;
        board[global_flip_stack[flip_stack]] = UndoFlips__oppcol
    }
    while UndoFlips__flip_count != 0 {
        UndoFlips__flip_count -= 2 as i32;
        flip_stack = flip_stack - 1;
        board[global_flip_stack[flip_stack]] = UndoFlips__oppcol;
        flip_stack = flip_stack - 1;
        board[global_flip_stack[flip_stack]] = UndoFlips__oppcol
    };
}
/*
  INIT_FLIP_STACK
  Reset the flip stack.
*/

pub unsafe fn init_flip_stack() {
    flip_stack = 0;
}
