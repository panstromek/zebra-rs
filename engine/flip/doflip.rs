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

fn DrctnlFlips_six(sq: usize, inc: isize, color: i32, oppcol: i32, board: &mut [i32; 128],  global_flip_stack: &mut [usize; 2048] , mut t_flip_stack: usize) -> usize {
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



fn DrctnlFlips_four(sq: usize, inc: isize, color: i32, oppcol: i32, board: &mut [i32; 128], global_flip_stack: &mut [usize; 2048], mut t_flip_stack: usize) -> usize {
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

pub fn DoFlips_no_hash(sqnum: i32, color: i32,
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
/*
   doflip.h

   Automatically created by ENDMACRO on Fri Feb 26 20:29:42 1999

   Last modified:   October 25, 2005
*/

pub fn DoFlips_hash(sqnum: i32, color: i32, board: &mut [i32; 128],
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
            let mut pt = sq.offset(1);
            if board[pt] == opp_color {
                pt = pt.offset(1);
                if board[pt] == opp_color {
                    pt = pt.offset(1);
                    if board[pt] == opp_color {
                        pt = pt.offset(1);
                        if board[pt] == opp_color {
                            pt = pt.offset(1);
                            if board[pt] == opp_color {
                                pt = pt.offset(1);
                                if board[pt] == opp_color {
                                    pt = pt.offset(1)
                                }
                            }
                        }
                    }
                }
                if board[pt] == color {
                    pt = pt.offset(-(1 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt]) as
                                i32;
                        board[pt] = color;
                        let fresh40 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh40 ] = (pt);
                        pt = pt.offset(-(1 as i32 as isize));
                        if !(pt != sq) { break ; }
                    }
                }
            }
            let mut pt_0 = sq.offset(11);
            if board[pt_0] == opp_color {
                pt_0 = pt_0.offset(11);
                if board[pt_0] == opp_color {
                    pt_0 = pt_0.offset(11);
                    if board[pt_0] == opp_color {
                        pt_0 = pt_0.offset(11);
                        if board[pt_0] == opp_color {
                            pt_0 = pt_0.offset(11);
                            if board[pt_0] == opp_color {
                                pt_0 =
                                    pt_0.offset(11);
                                if board[pt_0] == opp_color {
                                    pt_0 =
                                        pt_0.offset(11)
                                }
                            }
                        }
                    }
                }
                if board[pt_0] == color {
                    pt_0 = pt_0.offset(-(11 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_0]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_0]) as
                                i32;
                        board[pt_0] = color;
                        let fresh41 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh41 ] = (pt_0);
                        pt_0 = pt_0.offset(-(11 as i32 as isize));
                        if !(pt_0 != sq) { break ; }
                    }
                }
            }
            let mut pt_1 = sq.offset(10);
            if board[pt_1] == opp_color {
                pt_1 = pt_1.offset(10);
                if board[pt_1] == opp_color {
                    pt_1 = pt_1.offset(10);
                    if board[pt_1] == opp_color {
                        pt_1 = pt_1.offset(10);
                        if board[pt_1] == opp_color {
                            pt_1 = pt_1.offset(10);
                            if board[pt_1] == opp_color {
                                pt_1 =
                                    pt_1.offset(10);
                                if board[pt_1] == opp_color {
                                    pt_1 =
                                        pt_1.offset(10)
                                }
                            }
                        }
                    }
                }
                if board[pt_1] == color {
                    pt_1 = pt_1.offset(-(10 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_1]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_1]) as
                                i32;
                        board[pt_1] = color;
                        let fresh42 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh42 ] = (pt_1);
                        pt_1 = pt_1.offset(-(10 as i32 as isize));
                        if !(pt_1 != sq) { break ; }
                    }
                }
            }
        }
        2 => {
            let mut pt_2 = sq.offset(1);
            if board[pt_2] == opp_color {
                pt_2 = pt_2.offset(1);
                if board[pt_2] == opp_color {
                    pt_2 = pt_2.offset(1);
                    if board[pt_2] == opp_color {
                        pt_2 = pt_2.offset(1);
                        if board[pt_2] == opp_color {
                            pt_2 = pt_2.offset(1)
                        }
                    }
                }
                if board[pt_2] == color {
                    pt_2 = pt_2.offset(-(1 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_2]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_2]) as
                                i32;
                        board[pt_2] = color;
                        let fresh43 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh43 ] = (pt_2);
                        pt_2 = pt_2.offset(-(1 as i32 as isize));
                        if !(pt_2 != sq) { break ; }
                    }
                }
            }
            let mut pt_3 = sq.offset(11);
            if board[pt_3] == opp_color {
                pt_3 = pt_3.offset(11);
                if board[pt_3] == opp_color {
                    pt_3 = pt_3.offset(11);
                    if board[pt_3] == opp_color {
                        pt_3 = pt_3.offset(11);
                        if board[pt_3] == opp_color {
                            pt_3 = pt_3.offset(11)
                        }
                    }
                }
                if board[pt_3] == color {
                    pt_3 = pt_3.offset(-(11 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_3]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_3]) as
                                i32;
                        board[pt_3] = color;
                        let fresh44 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh44 ] = (pt_3);
                        pt_3 = pt_3.offset(-(11 as i32 as isize));
                        if !(pt_3 != sq) { break ; }
                    }
                }
            }
            let mut pt_4 = sq.offset(10);
            if board[pt_4] == opp_color {
                pt_4 = pt_4.offset(10);
                if board[pt_4] == opp_color {
                    pt_4 = pt_4.offset(10);
                    if board[pt_4] == opp_color {
                        pt_4 = pt_4.offset(10);
                        if board[pt_4] == opp_color {
                            pt_4 = pt_4.offset(10);
                            if board[pt_4] == opp_color {
                                pt_4 =
                                    pt_4.offset(10);
                                if board[pt_4] == opp_color {
                                    pt_4 =
                                        pt_4.offset(10)
                                }
                            }
                        }
                    }
                }
                if board[pt_4] == color {
                    pt_4 = pt_4.offset(-(10 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_4]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_4]) as
                                i32;
                        board[pt_4] = color;
                        let fresh45 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh45 ] = (pt_4);
                        pt_4 = pt_4.offset(-(10 as i32 as isize));
                        if !(pt_4 != sq) { break ; }
                    }
                }
            }
            let mut pt_5 = sq.offset(9);
            if board[pt_5] == opp_color {
                pt_5 = pt_5.offset(9);
                if board[pt_5] == opp_color {
                    pt_5 = pt_5.offset(9);
                    if board[pt_5] == opp_color {
                        pt_5 = pt_5.offset(9);
                        if board[pt_5] == opp_color {
                            pt_5 = pt_5.offset(9)
                        }
                    }
                }
                if board[pt_5] == color {
                    pt_5 = pt_5.offset(-(9 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_5]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_5]) as
                                i32;
                        board[pt_5] = color;
                        let fresh46 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh46 ] = (pt_5);
                        pt_5 = pt_5.offset(-(9 as i32 as isize));
                        if !(pt_5 != sq) { break ; }
                    }
                }
            }
            let mut pt_6 = sq.offset(-(1 as i32) as isize);
            if board[pt_6] == opp_color {
                pt_6 = pt_6.offset(-(1 as i32) as isize);
                if board[pt_6] == opp_color {
                    pt_6 = pt_6.offset(-(1 as i32) as isize);
                    if board[pt_6] == opp_color {
                        pt_6 = pt_6.offset(-(1 as i32) as isize);
                        if board[pt_6] == opp_color {
                            pt_6 = pt_6.offset(-(1 as i32) as isize)
                        }
                    }
                }
                if board[pt_6] == color {
                    pt_6 = pt_6.offset(-(-(1 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_6]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_6]) as
                                i32;
                        board[pt_6] = color;
                        let fresh47 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh47 ] = (pt_6);
                        pt_6 = pt_6.offset(-(-(1 as i32) as isize));
                        if !(pt_6 != sq) { break ; }
                    }
                }
            }
        }
        3 => {
            let mut pt_7 = sq.offset(10);
            if board[pt_7] == opp_color {
                pt_7 = pt_7.offset(10);
                if board[pt_7] == opp_color {
                    pt_7 = pt_7.offset(10);
                    if board[pt_7] == opp_color {
                        pt_7 = pt_7.offset(10);
                        if board[pt_7] == opp_color {
                            pt_7 = pt_7.offset(10);
                            if board[pt_7] == opp_color {
                                pt_7 =
                                    pt_7.offset(10);
                                if board[pt_7] == opp_color {
                                    pt_7 =
                                        pt_7.offset(10)
                                }
                            }
                        }
                    }
                }
                if board[pt_7] == color {
                    pt_7 = pt_7.offset(-(10 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_7]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_7]) as
                                i32;
                        board[pt_7] = color;
                        let fresh48 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh48 ] = (pt_7);
                        pt_7 = pt_7.offset(-(10 as i32 as isize));
                        if !(pt_7 != sq) { break ; }
                    }
                }
            }
            let mut pt_8 = sq.offset(9);
            if board[pt_8] == opp_color {
                pt_8 = pt_8.offset(9);
                if board[pt_8] == opp_color {
                    pt_8 = pt_8.offset(9);
                    if board[pt_8] == opp_color {
                        pt_8 = pt_8.offset(9);
                        if board[pt_8] == opp_color {
                            pt_8 = pt_8.offset(9);
                            if board[pt_8] == opp_color {
                                pt_8 = pt_8.offset(9);
                                if board[pt_8] == opp_color {
                                    pt_8 =
                                        pt_8.offset(9)
                                }
                            }
                        }
                    }
                }
                if board[pt_8] == color {
                    pt_8 = pt_8.offset(-(9 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_8]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_8]) as
                                i32;
                        board[pt_8] = color;
                        let fresh49 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh49 ] = (pt_8);
                        pt_8 = pt_8.offset(-(9 as i32 as isize));
                        if !(pt_8 != sq) { break ; }
                    }
                }
            }
            let mut pt_9 = sq.offset(-(1 as i32) as isize);
            if board[pt_9] == opp_color {
                pt_9 = pt_9.offset(-(1 as i32) as isize);
                if board[pt_9] == opp_color {
                    pt_9 = pt_9.offset(-(1 as i32) as isize);
                    if board[pt_9] == opp_color {
                        pt_9 = pt_9.offset(-(1 as i32) as isize);
                        if board[pt_9] == opp_color {
                            pt_9 = pt_9.offset(-(1 as i32) as isize);
                            if board[pt_9] == opp_color {
                                pt_9 =
                                    pt_9.offset(-(1 as i32) as isize);
                                if board[pt_9] == opp_color {
                                    pt_9 =
                                        pt_9.offset(-(1 as i32) as
                                                        isize)
                                }
                            }
                        }
                    }
                }
                if board[pt_9] == color {
                    pt_9 = pt_9.offset(-(-(1 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_9]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_9]) as
                                i32;
                        board[pt_9] = color;
                        let fresh50 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh50 ] = (pt_9);
                        pt_9 = pt_9.offset(-(-(1 as i32) as isize));
                        if !(pt_9 != sq) { break ; }
                    }
                }
            }
        }
        4 => {
            let mut pt_10 = sq.offset(-(10 as i32) as isize);
            if board[pt_10] == opp_color {
                pt_10 = pt_10.offset(-(10 as i32) as isize);
                if board[pt_10] == opp_color {
                    pt_10 = pt_10.offset(-(10 as i32) as isize);
                    if board[pt_10] == opp_color {
                        pt_10 = pt_10.offset(-(10 as i32) as isize);
                        if board[pt_10] == opp_color {
                            pt_10 =
                                pt_10.offset(-(10 as i32) as isize)
                        }
                    }
                }
                if board[pt_10] == color {
                    pt_10 = pt_10.offset(-(-(10 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_10]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_10]) as
                                i32;
                        board[pt_10] = color;
                        let fresh51 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh51 ] = (pt_10);
                        pt_10 =
                            pt_10.offset(-(-(10 as i32) as isize));
                        if !(pt_10 != sq) { break ; }
                    }
                }
            }
            let mut pt_11 = sq.offset(-(9 as i32) as isize);
            if board[pt_11] == opp_color {
                pt_11 = pt_11.offset(-(9 as i32) as isize);
                if board[pt_11] == opp_color {
                    pt_11 = pt_11.offset(-(9 as i32) as isize);
                    if board[pt_11] == opp_color {
                        pt_11 = pt_11.offset(-(9 as i32) as isize);
                        if board[pt_11] == opp_color {
                            pt_11 = pt_11.offset(-(9 as i32) as isize)
                        }
                    }
                }
                if board[pt_11] == color {
                    pt_11 = pt_11.offset(-(-(9 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_11]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_11]) as
                                i32;
                        board[pt_11] = color;
                        let fresh52 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh52 ] = (pt_11);
                        pt_11 = pt_11.offset(-(-(9 as i32) as isize));
                        if !(pt_11 != sq) { break ; }
                    }
                }
            }
            let mut pt_12 = sq.offset(1);
            if board[pt_12] == opp_color {
                pt_12 = pt_12.offset(1);
                if board[pt_12] == opp_color {
                    pt_12 = pt_12.offset(1);
                    if board[pt_12] == opp_color {
                        pt_12 = pt_12.offset(1);
                        if board[pt_12] == opp_color {
                            pt_12 = pt_12.offset(1);
                            if board[pt_12] == opp_color {
                                pt_12 =
                                    pt_12.offset(1);
                                if board[pt_12] == opp_color {
                                    pt_12 =
                                        pt_12.offset(1)
                                }
                            }
                        }
                    }
                }
                if board[pt_12] == color {
                    pt_12 = pt_12.offset(-(1 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_12]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_12]) as
                                i32;
                        board[pt_12] = color;
                        let fresh53 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh53 ] = (pt_12);
                        pt_12 = pt_12.offset(-(1 as i32 as isize));
                        if !(pt_12 != sq) { break ; }
                    }
                }
            }
            let mut pt_13 = sq.offset(11);
            if board[pt_13] == opp_color {
                pt_13 = pt_13.offset(11);
                if board[pt_13] == opp_color {
                    pt_13 = pt_13.offset(11);
                    if board[pt_13] == opp_color {
                        pt_13 = pt_13.offset(11);
                        if board[pt_13] == opp_color {
                            pt_13 = pt_13.offset(11)
                        }
                    }
                }
                if board[pt_13] == color {
                    pt_13 = pt_13.offset(-(11 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_13]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_13]) as
                                i32;
                        board[pt_13] = color;
                        let fresh54 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh54 ] = (pt_13);
                        pt_13 = pt_13.offset(-(11 as i32 as isize));
                        if !(pt_13 != sq) { break ; }
                    }
                }
            }
            let mut pt_14 = sq.offset(10);
            if board[pt_14] == opp_color {
                pt_14 = pt_14.offset(10);
                if board[pt_14] == opp_color {
                    pt_14 = pt_14.offset(10);
                    if board[pt_14] == opp_color {
                        pt_14 = pt_14.offset(10);
                        if board[pt_14] == opp_color {
                            pt_14 = pt_14.offset(10)
                        }
                    }
                }
                if board[pt_14] == color {
                    pt_14 = pt_14.offset(-(10 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_14]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_14]) as
                                i32;
                        board[pt_14] = color;
                        let fresh55 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh55 ] = (pt_14);
                        pt_14 = pt_14.offset(-(10 as i32 as isize));
                        if !(pt_14 != sq) { break ; }
                    }
                }
            }
        }
        5 => {
            let mut pt_15 = sq.offset(-(11 as i32) as isize);
            if board[pt_15] == opp_color {
                pt_15 = pt_15.offset(-(11 as i32) as isize);
                if board[pt_15] == opp_color {
                    pt_15 = pt_15.offset(-(11 as i32) as isize);
                    if board[pt_15] == opp_color {
                        pt_15 = pt_15.offset(-(11 as i32) as isize);
                        if board[pt_15] == opp_color {
                            pt_15 =
                                pt_15.offset(-(11 as i32) as isize)
                        }
                    }
                }
                if board[pt_15] == color {
                    pt_15 = pt_15.offset(-(-(11 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_15]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_15]) as
                                i32;
                        board[pt_15] = color;
                        let fresh56 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh56 ] = (pt_15);
                        pt_15 =
                            pt_15.offset(-(-(11 as i32) as isize));
                        if !(pt_15 != sq) { break ; }
                    }
                }
            }
            let mut pt_16 = sq.offset(-(10 as i32) as isize);
            if board[pt_16] == opp_color {
                pt_16 = pt_16.offset(-(10 as i32) as isize);
                if board[pt_16] == opp_color {
                    pt_16 = pt_16.offset(-(10 as i32) as isize);
                    if board[pt_16] == opp_color {
                        pt_16 = pt_16.offset(-(10 as i32) as isize);
                        if board[pt_16] == opp_color {
                            pt_16 =
                                pt_16.offset(-(10 as i32) as isize)
                        }
                    }
                }
                if board[pt_16] == color {
                    pt_16 = pt_16.offset(-(-(10 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_16]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_16]) as
                                i32;
                        board[pt_16] = color;
                        let fresh57 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh57 ] = (pt_16);
                        pt_16 =
                            pt_16.offset(-(-(10 as i32) as isize));
                        if !(pt_16 != sq) { break ; }
                    }
                }
            }
            let mut pt_17 = sq.offset(-(9 as i32) as isize);
            if board[pt_17] == opp_color {
                pt_17 = pt_17.offset(-(9 as i32) as isize);
                if board[pt_17] == opp_color {
                    pt_17 = pt_17.offset(-(9 as i32) as isize);
                    if board[pt_17] == opp_color {
                        pt_17 = pt_17.offset(-(9 as i32) as isize);
                        if board[pt_17] == opp_color {
                            pt_17 = pt_17.offset(-(9 as i32) as isize)
                        }
                    }
                }
                if board[pt_17] == color {
                    pt_17 = pt_17.offset(-(-(9 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_17]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_17]) as
                                i32;
                        board[pt_17] = color;
                        let fresh58 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh58 ] = (pt_17);
                        pt_17 = pt_17.offset(-(-(9 as i32) as isize));
                        if !(pt_17 != sq) { break ; }
                    }
                }
            }
            let mut pt_18 = sq.offset(1);
            if board[pt_18] == opp_color {
                pt_18 = pt_18.offset(1);
                if board[pt_18] == opp_color {
                    pt_18 = pt_18.offset(1);
                    if board[pt_18] == opp_color {
                        pt_18 = pt_18.offset(1);
                        if board[pt_18] == opp_color {
                            pt_18 = pt_18.offset(1)
                        }
                    }
                }
                if board[pt_18] == color {
                    pt_18 = pt_18.offset(-(1 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_18]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_18]) as
                                i32;
                        board[pt_18] = color;
                        let fresh59 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh59 ] = (pt_18);
                        pt_18 = pt_18.offset(-(1 as i32 as isize));
                        if !(pt_18 != sq) { break ; }
                    }
                }
            }
            let mut pt_19 = sq.offset(11);
            if board[pt_19] == opp_color {
                pt_19 = pt_19.offset(11);
                if board[pt_19] == opp_color {
                    pt_19 = pt_19.offset(11);
                    if board[pt_19] == opp_color {
                        pt_19 = pt_19.offset(11);
                        if board[pt_19] == opp_color {
                            pt_19 = pt_19.offset(11)
                        }
                    }
                }
                if board[pt_19] == color {
                    pt_19 = pt_19.offset(-(11 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_19]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_19]) as
                                i32;
                        board[pt_19] = color;
                        let fresh60 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh60 ] = (pt_19);
                        pt_19 = pt_19.offset(-(11 as i32 as isize));
                        if !(pt_19 != sq) { break ; }
                    }
                }
            }
            let mut pt_20 = sq.offset(10);
            if board[pt_20] == opp_color {
                pt_20 = pt_20.offset(10);
                if board[pt_20] == opp_color {
                    pt_20 = pt_20.offset(10);
                    if board[pt_20] == opp_color {
                        pt_20 = pt_20.offset(10);
                        if board[pt_20] == opp_color {
                            pt_20 = pt_20.offset(10)
                        }
                    }
                }
                if board[pt_20] == color {
                    pt_20 = pt_20.offset(-(10 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_20]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_20]) as
                                i32;
                        board[pt_20] = color;
                        let fresh61 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh61 ] = (pt_20);
                        pt_20 = pt_20.offset(-(10 as i32 as isize));
                        if !(pt_20 != sq) { break ; }
                    }
                }
            }
            let mut pt_21 = sq.offset(9);
            if board[pt_21] == opp_color {
                pt_21 = pt_21.offset(9);
                if board[pt_21] == opp_color {
                    pt_21 = pt_21.offset(9);
                    if board[pt_21] == opp_color {
                        pt_21 = pt_21.offset(9);
                        if board[pt_21] == opp_color {
                            pt_21 = pt_21.offset(9)
                        }
                    }
                }
                if board[pt_21] == color {
                    pt_21 = pt_21.offset(-(9 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_21]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_21]) as
                                i32;
                        board[pt_21] = color;
                        let fresh62 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh62 ] = (pt_21);
                        pt_21 = pt_21.offset(-(9 as i32 as isize));
                        if !(pt_21 != sq) { break ; }
                    }
                }
            }
            let mut pt_22 = sq.offset(-(1 as i32) as isize);
            if board[pt_22] == opp_color {
                pt_22 = pt_22.offset(-(1 as i32) as isize);
                if board[pt_22] == opp_color {
                    pt_22 = pt_22.offset(-(1 as i32) as isize);
                    if board[pt_22] == opp_color {
                        pt_22 = pt_22.offset(-(1 as i32) as isize);
                        if board[pt_22] == opp_color {
                            pt_22 = pt_22.offset(-(1 as i32) as isize)
                        }
                    }
                }
                if board[pt_22] == color {
                    pt_22 = pt_22.offset(-(-(1 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_22]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_22]) as
                                i32;
                        board[pt_22] = color;
                        let fresh63 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh63 ] = (pt_22);
                        pt_22 = pt_22.offset(-(-(1 as i32) as isize));
                        if !(pt_22 != sq) { break ; }
                    }
                }
            }
        }
        6 => {
            let mut pt_23 = sq.offset(-(10 as i32) as isize);
            if board[pt_23] == opp_color {
                pt_23 = pt_23.offset(-(10 as i32) as isize);
                if board[pt_23] == opp_color {
                    pt_23 = pt_23.offset(-(10 as i32) as isize);
                    if board[pt_23] == opp_color {
                        pt_23 = pt_23.offset(-(10 as i32) as isize);
                        if board[pt_23] == opp_color {
                            pt_23 =
                                pt_23.offset(-(10 as i32) as isize)
                        }
                    }
                }
                if board[pt_23] == color {
                    pt_23 = pt_23.offset(-(-(10 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_23]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_23]) as
                                i32;
                        board[pt_23] = color;
                        let fresh64 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh64 ] = (pt_23);
                        pt_23 =
                            pt_23.offset(-(-(10 as i32) as isize));
                        if !(pt_23 != sq) { break ; }
                    }
                }
            }
            let mut pt_24 = sq.offset(-(11 as i32) as isize);
            if board[pt_24] == opp_color {
                pt_24 = pt_24.offset(-(11 as i32) as isize);
                if board[pt_24] == opp_color {
                    pt_24 = pt_24.offset(-(11 as i32) as isize);
                    if board[pt_24] == opp_color {
                        pt_24 = pt_24.offset(-(11 as i32) as isize);
                        if board[pt_24] == opp_color {
                            pt_24 =
                                pt_24.offset(-(11 as i32) as isize)
                        }
                    }
                }
                if board[pt_24] == color {
                    pt_24 = pt_24.offset(-(-(11 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_24]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_24]) as
                                i32;
                        board[pt_24] = color;
                        let fresh65 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh65 ] = (pt_24);
                        pt_24 =
                            pt_24.offset(-(-(11 as i32) as isize));
                        if !(pt_24 != sq) { break ; }
                    }
                }
            }
            let mut pt_25 = sq.offset(-(1 as i32) as isize);
            if board[pt_25] == opp_color {
                pt_25 = pt_25.offset(-(1 as i32) as isize);
                if board[pt_25] == opp_color {
                    pt_25 = pt_25.offset(-(1 as i32) as isize);
                    if board[pt_25] == opp_color {
                        pt_25 = pt_25.offset(-(1 as i32) as isize);
                        if board[pt_25] == opp_color {
                            pt_25 =
                                pt_25.offset(-(1 as i32) as isize);
                            if board[pt_25] == opp_color {
                                pt_25 =
                                    pt_25.offset(-(1 as i32) as
                                                     isize);
                                if board[pt_25] == opp_color {
                                    pt_25 =
                                        pt_25.offset(-(1 as i32) as
                                                         isize)
                                }
                            }
                        }
                    }
                }
                if board[pt_25] == color {
                    pt_25 = pt_25.offset(-(-(1 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_25]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_25]) as
                                i32;
                        board[pt_25] = color;
                        let fresh66 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh66 ] = (pt_25);
                        pt_25 = pt_25.offset(-(-(1 as i32) as isize));
                        if !(pt_25 != sq) { break ; }
                    }
                }
            }
            let mut pt_26 = sq.offset(9);
            if board[pt_26] == opp_color {
                pt_26 = pt_26.offset(9);
                if board[pt_26] == opp_color {
                    pt_26 = pt_26.offset(9);
                    if board[pt_26] == opp_color {
                        pt_26 = pt_26.offset(9);
                        if board[pt_26] == opp_color {
                            pt_26 = pt_26.offset(9)
                        }
                    }
                }
                if board[pt_26] == color {
                    pt_26 = pt_26.offset(-(9 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_26]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_26]) as
                                i32;
                        board[pt_26] = color;
                        let fresh67 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh67 ] = (pt_26);
                        pt_26 = pt_26.offset(-(9 as i32 as isize));
                        if !(pt_26 != sq) { break ; }
                    }
                }
            }
            let mut pt_27 = sq.offset(10);
            if board[pt_27] == opp_color {
                pt_27 = pt_27.offset(10);
                if board[pt_27] == opp_color {
                    pt_27 = pt_27.offset(10);
                    if board[pt_27] == opp_color {
                        pt_27 = pt_27.offset(10);
                        if board[pt_27] == opp_color {
                            pt_27 = pt_27.offset(10)
                        }
                    }
                }
                if board[pt_27] == color {
                    pt_27 = pt_27.offset(-(10 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_27]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_27]) as
                                i32;
                        board[pt_27] = color;
                        let fresh68 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh68 ] = (pt_27);
                        pt_27 = pt_27.offset(-(10 as i32 as isize));
                        if !(pt_27 != sq) { break ; }
                    }
                }
            }
        }
        7 => {
            let mut pt_28 = sq.offset(-(10 as i32) as isize);
            if board[pt_28] == opp_color {
                pt_28 = pt_28.offset(-(10 as i32) as isize);
                if board[pt_28] == opp_color {
                    pt_28 = pt_28.offset(-(10 as i32) as isize);
                    if board[pt_28] == opp_color {
                        pt_28 = pt_28.offset(-(10 as i32) as isize);
                        if board[pt_28] == opp_color {
                            pt_28 =
                                pt_28.offset(-(10 as i32) as isize);
                            if board[pt_28] == opp_color {
                                pt_28 =
                                    pt_28.offset(-(10 as i32) as
                                                     isize);
                                if board[pt_28] == opp_color {
                                    pt_28 =
                                        pt_28.offset(-(10 as i32) as
                                                         isize)
                                }
                            }
                        }
                    }
                }
                if board[pt_28] == color {
                    pt_28 = pt_28.offset(-(-(10 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_28]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_28]) as
                                i32;
                        board[pt_28] = color;
                        let fresh69 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh69 ] = (pt_28);
                        pt_28 =
                            pt_28.offset(-(-(10 as i32) as isize));
                        if !(pt_28 != sq) { break ; }
                    }
                }
            }
            let mut pt_29 = sq.offset(-(9 as i32) as isize);
            if board[pt_29] == opp_color {
                pt_29 = pt_29.offset(-(9 as i32) as isize);
                if board[pt_29] == opp_color {
                    pt_29 = pt_29.offset(-(9 as i32) as isize);
                    if board[pt_29] == opp_color {
                        pt_29 = pt_29.offset(-(9 as i32) as isize);
                        if board[pt_29] == opp_color {
                            pt_29 =
                                pt_29.offset(-(9 as i32) as isize);
                            if board[pt_29] == opp_color {
                                pt_29 =
                                    pt_29.offset(-(9 as i32) as
                                                     isize);
                                if board[pt_29] == opp_color {
                                    pt_29 =
                                        pt_29.offset(-(9 as i32) as
                                                         isize)
                                }
                            }
                        }
                    }
                }
                if board[pt_29] == color {
                    pt_29 = pt_29.offset(-(-(9 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_29]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_29]) as
                                i32;
                        board[pt_29] = color;
                        let fresh70 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh70 ] = (pt_29);
                        pt_29 = pt_29.offset(-(-(9 as i32) as isize));
                        if !(pt_29 != sq) { break ; }
                    }
                }
            }
            let mut pt_30 = sq.offset(1);
            if board[pt_30] == opp_color {
                pt_30 = pt_30.offset(1);
                if board[pt_30] == opp_color {
                    pt_30 = pt_30.offset(1);
                    if board[pt_30] == opp_color {
                        pt_30 = pt_30.offset(1);
                        if board[pt_30] == opp_color {
                            pt_30 = pt_30.offset(1);
                            if board[pt_30] == opp_color {
                                pt_30 =
                                    pt_30.offset(1);
                                if board[pt_30] == opp_color {
                                    pt_30 =
                                        pt_30.offset(1)
                                }
                            }
                        }
                    }
                }
                if board[pt_30] == color {
                    pt_30 = pt_30.offset(-(1 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_30]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_30]) as
                                i32;
                        board[pt_30] = color;
                        let fresh71 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh71 ] = (pt_30);
                        pt_30 = pt_30.offset(-(1 as i32 as isize));
                        if !(pt_30 != sq) { break ; }
                    }
                }
            }
        }
        8 => {
            let mut pt_31 = sq.offset(-(1 as i32) as isize);
            if board[pt_31] == opp_color {
                pt_31 = pt_31.offset(-(1 as i32) as isize);
                if board[pt_31] == opp_color {
                    pt_31 = pt_31.offset(-(1 as i32) as isize);
                    if board[pt_31] == opp_color {
                        pt_31 = pt_31.offset(-(1 as i32) as isize);
                        if board[pt_31] == opp_color {
                            pt_31 = pt_31.offset(-(1 as i32) as isize)
                        }
                    }
                }
                if board[pt_31] == color {
                    pt_31 = pt_31.offset(-(-(1 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_31]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_31]) as
                                i32;
                        board[pt_31] = color;
                        let fresh72 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh72 ] = (pt_31);
                        pt_31 = pt_31.offset(-(-(1 as i32) as isize));
                        if !(pt_31 != sq) { break ; }
                    }
                }
            }
            let mut pt_32 = sq.offset(-(11 as i32) as isize);
            if board[pt_32] == opp_color {
                pt_32 = pt_32.offset(-(11 as i32) as isize);
                if board[pt_32] == opp_color {
                    pt_32 = pt_32.offset(-(11 as i32) as isize);
                    if board[pt_32] == opp_color {
                        pt_32 = pt_32.offset(-(11 as i32) as isize);
                        if board[pt_32] == opp_color {
                            pt_32 =
                                pt_32.offset(-(11 as i32) as isize)
                        }
                    }
                }
                if board[pt_32] == color {
                    pt_32 = pt_32.offset(-(-(11 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_32]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_32]) as
                                i32;
                        board[pt_32] = color;
                        let fresh73 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh73 ] = (pt_32);
                        pt_32 =
                            pt_32.offset(-(-(11 as i32) as isize));
                        if !(pt_32 != sq) { break ; }
                    }
                }
            }
            let mut pt_33 = sq.offset(-(10 as i32) as isize);
            if board[pt_33] == opp_color {
                pt_33 = pt_33.offset(-(10 as i32) as isize);
                if board[pt_33] == opp_color {
                    pt_33 = pt_33.offset(-(10 as i32) as isize);
                    if board[pt_33] == opp_color {
                        pt_33 = pt_33.offset(-(10 as i32) as isize);
                        if board[pt_33] == opp_color {
                            pt_33 =
                                pt_33.offset(-(10 as i32) as isize);
                            if board[pt_33] == opp_color {
                                pt_33 =
                                    pt_33.offset(-(10 as i32) as
                                                     isize);
                                if board[pt_33] == opp_color {
                                    pt_33 =
                                        pt_33.offset(-(10 as i32) as
                                                         isize)
                                }
                            }
                        }
                    }
                }
                if board[pt_33] == color {
                    pt_33 = pt_33.offset(-(-(10 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_33]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_33]) as
                                i32;
                        board[pt_33] = color;
                        let fresh74 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh74 ] = (pt_33);
                        pt_33 =
                            pt_33.offset(-(-(10 as i32) as isize));
                        if !(pt_33 != sq) { break ; }
                    }
                }
            }
            let mut pt_34 = sq.offset(-(9 as i32) as isize);
            if board[pt_34] == opp_color {
                pt_34 = pt_34.offset(-(9 as i32) as isize);
                if board[pt_34] == opp_color {
                    pt_34 = pt_34.offset(-(9 as i32) as isize);
                    if board[pt_34] == opp_color {
                        pt_34 = pt_34.offset(-(9 as i32) as isize);
                        if board[pt_34] == opp_color {
                            pt_34 = pt_34.offset(-(9 as i32) as isize)
                        }
                    }
                }
                if board[pt_34] == color {
                    pt_34 = pt_34.offset(-(-(9 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_34]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_34]) as
                                i32;
                        board[pt_34] = color;
                        let fresh75 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh75 ] = (pt_34);
                        pt_34 = pt_34.offset(-(-(9 as i32) as isize));
                        if !(pt_34 != sq) { break ; }
                    }
                }
            }
            let mut pt_35 = sq.offset(1);
            if board[pt_35] == opp_color {
                pt_35 = pt_35.offset(1);
                if board[pt_35] == opp_color {
                    pt_35 = pt_35.offset(1);
                    if board[pt_35] == opp_color {
                        pt_35 = pt_35.offset(1);
                        if board[pt_35] == opp_color {
                            pt_35 = pt_35.offset(1)
                        }
                    }
                }
                if board[pt_35] == color {
                    pt_35 = pt_35.offset(-(1 as i32 as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_35]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_35]) as
                                i32;
                        board[pt_35] = color;
                        let fresh76 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh76 ] = (pt_35);
                        pt_35 = pt_35.offset(-(1 as i32 as isize));
                        if !(pt_35 != sq) { break ; }
                    }
                }
            }
        }
        9 => {
            let mut pt_36 = sq.offset(-(10 as i32) as isize);
            if board[pt_36] == opp_color {
                pt_36 = pt_36.offset(-(10 as i32) as isize);
                if board[pt_36] == opp_color {
                    pt_36 = pt_36.offset(-(10 as i32) as isize);
                    if board[pt_36] == opp_color {
                        pt_36 = pt_36.offset(-(10 as i32) as isize);
                        if board[pt_36] == opp_color {
                            pt_36 =
                                pt_36.offset(-(10 as i32) as isize);
                            if board[pt_36] == opp_color {
                                pt_36 =
                                    pt_36.offset(-(10 as i32) as
                                                     isize);
                                if board[pt_36] == opp_color {
                                    pt_36 =
                                        pt_36.offset(-(10 as i32) as
                                                         isize)
                                }
                            }
                        }
                    }
                }
                if board[pt_36] == color {
                    pt_36 = pt_36.offset(-(-(10 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_36]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_36]) as
                                i32;
                        board[pt_36] = color;
                        let fresh77 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh77 ] = (pt_36);
                        pt_36 =
                            pt_36.offset(-(-(10 as i32) as isize));
                        if !(pt_36 != sq) { break ; }
                    }
                }
            }
            let mut pt_37 = sq.offset(-(11 as i32) as isize);
            if board[pt_37] == opp_color {
                pt_37 = pt_37.offset(-(11 as i32) as isize);
                if board[pt_37] == opp_color {
                    pt_37 = pt_37.offset(-(11 as i32) as isize);
                    if board[pt_37] == opp_color {
                        pt_37 = pt_37.offset(-(11 as i32) as isize);
                        if board[pt_37] == opp_color {
                            pt_37 =
                                pt_37.offset(-(11 as i32) as isize);
                            if board[pt_37] == opp_color {
                                pt_37 =
                                    pt_37.offset(-(11 as i32) as
                                                     isize);
                                if board[pt_37] == opp_color {
                                    pt_37 =
                                        pt_37.offset(-(11 as i32) as
                                                         isize)
                                }
                            }
                        }
                    }
                }
                if board[pt_37] == color {
                    pt_37 = pt_37.offset(-(-(11 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_37]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_37]) as
                                i32;
                        board[pt_37] = color;
                        let fresh78 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh78 ] = (pt_37);
                        pt_37 =
                            pt_37.offset(-(-(11 as i32) as isize));
                        if !(pt_37 != sq) { break ; }
                    }
                }
            }
            let mut pt_38 = sq.offset(-(1 as i32) as isize);
            if board[pt_38] == opp_color {
                pt_38 = pt_38.offset(-(1 as i32) as isize);
                if board[pt_38] == opp_color {
                    pt_38 = pt_38.offset(-(1 as i32) as isize);
                    if board[pt_38] == opp_color {
                        pt_38 = pt_38.offset(-(1 as i32) as isize);
                        if board[pt_38] == opp_color {
                            pt_38 =
                                pt_38.offset(-(1 as i32) as isize);
                            if board[pt_38] == opp_color {
                                pt_38 =
                                    pt_38.offset(-(1 as i32) as
                                                     isize);
                                if board[pt_38] == opp_color {
                                    pt_38 =
                                        pt_38.offset(-(1 as i32) as
                                                         isize)
                                }
                            }
                        }
                    }
                }
                if board[pt_38] == color {
                    pt_38 = pt_38.offset(-(-(1 as i32) as isize));
                    loop  {
                        t_hash_update1 =
                            (t_hash_update1 as u32 ^
                                 hash_flip1[pt_38]) as
                                i32;
                        t_hash_update2 =
                            (t_hash_update2 as u32 ^
                                 hash_flip2[pt_38]) as
                                i32;
                        board[pt_38] = color;
                        let fresh79 = t_flip_stack;
                        t_flip_stack = t_flip_stack.offset(1);
                        global_flip_stack[fresh79 ] = (pt_38);
                        pt_38 = pt_38.offset(-(-(1 as i32) as isize));
                        if !(pt_38 != sq) { break ; }
                    }
                }
            }
        }
        _ => { }
    }
    *flip_stack = t_flip_stack;
    return ((t_flip_stack - old_flip_stack) as i32, t_hash_update1 as u32, t_hash_update2 as u32);
}
