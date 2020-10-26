
/*
   File:          unflip.h

   Created:       February 26, 1999

   Modified:      December 25, 1999

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      Low-level macro code to flip back the discs
                  flipped by a move.
*/

/*
   File:           unflip.c

   Created:        February 26, 1999

   Modified:       July 12, 1999

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Low-level code to flip back the discs flipped by a move.
*/

use engine_traits::Offset;

pub struct FlipStack {
    pub global_flip_stack: [usize; 2048],
    pub flip_stack: usize,
}
impl FlipStack {
    /*
      INIT_FLIP_STACK
      Reset the flip stack.
    */
    pub fn init_flip_stack(&mut self) {
        self.flip_stack = 0;
    }
    /*
      UNDOFLIPS
      Flip back the FLIP_COUNT topmost squares on GLOBALFLIPSTACK.
      This seems to be the fastest way to do it and doesn't matter
      much anyway as this function consumes a negligible percentage
      of the time.
    */

    pub fn UndoFlips(&mut self, board: &mut [i32; 128], flip_count: i32, oppcol: i32) {
        let mut UndoFlips__flip_count = flip_count;
        let UndoFlips__oppcol = oppcol;
        if UndoFlips__flip_count & 1 as i32 != 0 {
            UndoFlips__flip_count -= 1;
            self.flip_stack = self.flip_stack -1;
            board[self.global_flip_stack[self.flip_stack]] = UndoFlips__oppcol
        }
        while UndoFlips__flip_count != 0 {
            UndoFlips__flip_count -= 2 as i32;
            self.flip_stack = self.flip_stack - 1;
            board[self.global_flip_stack[self.flip_stack]] = UndoFlips__oppcol;
            self.flip_stack = self.flip_stack - 1;
            board[self.global_flip_stack[self.flip_stack]] = UndoFlips__oppcol
        };
    }
}

pub static mut flip_stack_: FlipStack = FlipStack {
    global_flip_stack: [0; 2048],
    flip_stack: 0,
};
