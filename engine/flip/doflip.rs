use core::mem;

use engine_traits::Offset;

use crate::unflip::FlipStack;

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
   File:          doflip.c

   Modified:      November 15, 2005

   Authors:       Gunnar Andersson (gunnar@radagast.se)
              Toshihiko Okuhara

   Contents:      Low-level code which flips the discs (if any) affected
                  by a potential move, with or without updating the
          hash code.

   This piece of software is released under the GPL.
   See the file COPYING for more information.
*/
/* The board split into nine regions. */
static board_region: [i8; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 2, 2, 2, 2, 3, 3, 0,
    0, 1, 1, 2, 2, 2, 2, 3, 3, 0,
    0, 4, 4, 5, 5, 5, 5, 6, 6, 0,
    0, 4, 4, 5, 5, 5, 5, 6, 6, 0,
    0, 4, 4, 5, 5, 5, 5, 6, 6, 0,
    0, 4, 4, 5, 5, 5, 5, 6, 6, 0,
    0, 7, 7, 8, 8, 8, 8, 9, 9, 0,
    0, 7, 7, 8, 8, 8, 8, 9, 9, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];

fn DrctnlFlips_six(sq: usize, inc: isize, color: i32, oppcol: i32, board: &mut [i32; 128],  global_flip_stack: &mut [u8; 2048] , mut t_flip_stack: usize) -> usize {
    let mut pt:isize = sq as isize + inc;


    if board[pt as usize] == oppcol {
        pt += inc;
        if board[pt as usize] == oppcol {
            pt += inc;
            if board[pt as usize] == oppcol {
                pt += inc;
                if board[pt as usize] == oppcol {
                    pt += inc;
                    if board[pt as usize] == oppcol {
                        pt += inc;
                        if board[pt as usize] == oppcol {
                            pt += inc;
                        }
                    }
                }
            }
        }
        if board[pt as usize] == color {
            pt -= inc;
            loop {
                board[pt as usize] = color;
                let fresh0 = t_flip_stack;
                t_flip_stack = t_flip_stack.offset(1);
                global_flip_stack[fresh0 ] = (pt as _);
                pt = pt - inc;
                if pt == sq as isize {
                    break;
                }
            };
        }
    }
    t_flip_stack
}



fn DrctnlFlips_four(sq: usize, inc: isize, color: i32, oppcol: i32, board: &mut [i32; 128], global_flip_stack: &mut [u8; 2048], mut t_flip_stack: usize) -> usize {
    let mut pt: isize = sq as isize + inc;


    if board[pt as usize] == oppcol {
        pt += inc;
        if board[pt as usize] == oppcol {
            pt += inc;
            if board[pt as usize] == oppcol {
                pt += inc;
                if board[pt as usize] == oppcol {
                    pt += inc;
                }
            }
        }
        if board[pt as usize] == color {
            pt -= inc;
            loop {
                board[pt as usize] = color;
                let fresh3 = t_flip_stack;
                t_flip_stack = t_flip_stack.offset(1);
                global_flip_stack[fresh3] = (pt as _);

                pt -= inc;
                if pt == sq as isize { break; }
            };
        }
    }
    return t_flip_stack
}

pub fn DoFlips_no_hash(sqnum: i8, color: i32,
                              board: &mut [i32; 128], flip_stack__: &mut FlipStack)
                              -> i32 {
    let opp_color = 2 - color;
    let flip_stack = &mut flip_stack__.flip_stack;
    let global_flip_stack = &mut flip_stack__.global_flip_stack;
    let mut t_flip_stack = *flip_stack;
    let old_flip_stack = t_flip_stack;
    let sq = sqnum as usize;
    match board_region[sqnum as usize] as i32 {
        1 => {
            t_flip_stack = DrctnlFlips_six(sq, 1, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_six(sq, 11, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_six(sq, 10, color, opp_color, board, global_flip_stack, t_flip_stack);
        }

        2 => {
            t_flip_stack = DrctnlFlips_four(sq, 1, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, 11, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_six(sq, 10, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, 9, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, -1, color, opp_color, board, global_flip_stack, t_flip_stack);
        }
        3 => {
            t_flip_stack = DrctnlFlips_six(sq, 10, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_six(sq, 9, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_six(sq, -1, color, opp_color, board, global_flip_stack, t_flip_stack);
        }
        4 => {
            t_flip_stack = DrctnlFlips_four(sq, -10, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, -9, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_six(sq, 1, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, 11, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, 10, color, opp_color, board, global_flip_stack, t_flip_stack);
        }
        5 => {
            t_flip_stack = DrctnlFlips_four(sq, -11, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, -10, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, -9, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, 1, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, 11, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, 10, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, 9, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, -1, color, opp_color, board, global_flip_stack, t_flip_stack);
        }
        6 => {
            t_flip_stack = DrctnlFlips_four(sq, -10, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, -11, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_six(sq, -1, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, 9, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, 10, color, opp_color, board, global_flip_stack, t_flip_stack);
        }
        7 => {
            t_flip_stack = DrctnlFlips_six(sq, -10, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_six(sq, -9, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_six(sq, 1, color, opp_color, board, global_flip_stack, t_flip_stack);
        }
        8 => {
            t_flip_stack = DrctnlFlips_four(sq, -1, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, -11, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_six(sq, -10, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, -9, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_four(sq, 1, color, opp_color, board, global_flip_stack, t_flip_stack);
        }
        9 => {
            t_flip_stack = DrctnlFlips_six(sq, -10, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_six(sq, -11, color, opp_color, board, global_flip_stack, t_flip_stack);
            t_flip_stack = DrctnlFlips_six(sq, -1, color, opp_color, board, global_flip_stack, t_flip_stack);
        }

        _ => { }
    }
    *flip_stack = t_flip_stack;
    return (t_flip_stack - old_flip_stack) as i32;
}


fn DrctnlFlipsHash_four( sq: usize, inc: isize, color: i32, oppcol: i32, board: &mut [i32; 128],
                         global_flip_stack: &mut [u8; 2048] , mut t_flip_stack: usize ,
                         t_hash_update2: &mut i32,  t_hash_update1: &mut i32,
                         hash_flip1: &mut [u32; 128], hash_flip2: &mut [u32; 128]) -> usize {
    let mut pt: isize = sq as isize + inc;


    if board[pt as usize] == oppcol {
        pt += inc;
        if board[pt as usize] == oppcol {
            pt += inc;
            if board[pt as usize] == oppcol {
                pt += inc;
                if board[pt as usize] == oppcol {
                    pt += inc;
                }
            }
        }
        if board[pt as usize] == color {
            pt -= inc;
            loop {
                *t_hash_update1 =
                    (*t_hash_update1 as u32 ^
                        hash_flip1[pt as usize]) as
                        i32;
                *t_hash_update2 =
                    (*t_hash_update2 as u32 ^
                        hash_flip2[pt as usize]) as
                        i32;
                board[pt as usize] = color;
                let fresh40 = t_flip_stack;
                t_flip_stack = t_flip_stack.offset(1);
                global_flip_stack[fresh40] = (pt as _);
                pt -= inc;
                if !(pt != sq as isize) { break; }
            };
        }
    }
    t_flip_stack
}


fn DrctnlFlipsHash_six( sq: usize, inc: isize, color: i32, oppcol: i32, board: &mut [i32; 128],
                        global_flip_stack: &mut [u8; 2048] , mut t_flip_stack: usize,
                        t_hash_update2: &mut i32,  t_hash_update1: &mut i32,
                        hash_flip1: &mut [u32; 128], hash_flip2: &mut [u32; 128],
) -> usize {
    let mut pt: isize = sq as isize + inc;

    if board[pt as usize] == oppcol {
        pt += inc;
        if board[pt as usize] == oppcol {
            pt += inc;
            if board[pt as usize] == oppcol {
                pt += inc;
                if board[pt as usize] == oppcol {
                    pt += inc;
                    if board[pt as usize] == oppcol {
                        pt += inc;
                        if board[pt as usize] == oppcol {
                            pt += inc;
                        }
                    }
                }
            }
        }
        if board[pt as usize] == color {
            pt -= inc;
            loop {
                *t_hash_update1 =
                    (*t_hash_update1 as u32 ^
                        hash_flip1[pt as usize]) as
                        i32;
                *t_hash_update2 =
                    (*t_hash_update2 as u32 ^
                        hash_flip2[pt as usize]) as
                        i32;
                board[pt as usize] = color;
                let fresh40 = t_flip_stack;
                t_flip_stack = t_flip_stack.offset(1);
                global_flip_stack[fresh40] = (pt as _);
                pt -= inc;
                if !(sq as isize != pt) { break; }
            };
        }
    }
    t_flip_stack
}

/*
   doflip.h

   Automatically created by ENDMACRO on Fri Feb 26 20:29:42 1999

   Last modified:   October 25, 2005
*/

pub fn DoFlips_hash(sqnum: i8, color: i32, board: &mut [i32; 128],
                           hash_flip1: &mut [u32; 128], hash_flip2: &mut [u32; 128], stack: &mut FlipStack) -> (i32, u32, u32) {
    let opp_color = 2 - color;

    let flip_stack = &mut  stack.flip_stack;
    let global_flip_stack = &mut stack.global_flip_stack;

    let mut t_flip_stack = *flip_stack;
    let old_flip_stack = t_flip_stack;
    let mut t_hash_update2 = 0;
    let mut t_hash_update1 = t_hash_update2;
    let mut sq = sqnum as usize;
    match board_region[sqnum as usize] as i32 {
        1 => {
            t_flip_stack = DrctnlFlipsHash_six(sq, 1, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_six(sq, 11, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_six(sq, 10, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
        }
        2 => {
            t_flip_stack = DrctnlFlipsHash_four(sq, 1, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, 11, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_six(sq, 10, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, 9, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, -1, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
        }
        3 => {
            t_flip_stack = DrctnlFlipsHash_six(sq, 10, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_six(sq, 9, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_six(sq, -1, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
        }
        4 => {
            t_flip_stack = DrctnlFlipsHash_four(sq, -10, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, -9, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_six(sq, 1, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, 11, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, 10, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
        }
        5 => {
            t_flip_stack = DrctnlFlipsHash_four(sq, -11, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, -10, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, -9, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, 1, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, 11, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, 10, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, 9, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, -1, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
        }
        6 => {
            t_flip_stack = DrctnlFlipsHash_four(sq, -10, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, -11, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_six(sq, -1, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, 9, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, 10, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
        }
        7 => {
            t_flip_stack = DrctnlFlipsHash_six(sq, -10, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_six(sq, -9, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_six(sq, 1, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
        }
        8 => {
            t_flip_stack = DrctnlFlipsHash_four(sq, -1, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, -11, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_six(sq, -10, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, -9, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_four(sq, 1, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
        }
        9 => {
            t_flip_stack = DrctnlFlipsHash_six(sq, -10, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_six(sq, -11, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
            t_flip_stack = DrctnlFlipsHash_six(sq, -1, color, opp_color, board, global_flip_stack, t_flip_stack, &mut t_hash_update2, &mut t_hash_update1, hash_flip1, hash_flip2);
        }
        _   => { }
    };
    *flip_stack = t_flip_stack;
    return ((t_flip_stack - old_flip_stack) as i32, t_hash_update1 as u32, t_hash_update2 as u32);
}
