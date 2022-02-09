use crate::bitboard::{BitBoard, non_iterative_popcount};

/*
   File:          bitbmob.c

   Modified:      November 18, 2005

   Authors:       Gunnar Andersson (gunnar@radagast.se)
              Toshihiko Okuhara

   Contents:      Count feasible moves in the bitboard.

   This piece of software is released under the GPL.
   See the file COPYING for more information.
*/

fn generate_all_c(my_bits: BitBoard, opp_bits: BitBoard) -> BitBoard {
   // mm6
   let mut moves = BitBoard { high: 0, low: 0 }; // mm4
   let mut opp_inner_bits = BitBoard { high: 0, low: 0 }; // mm5
   let mut flip_bits = BitBoard { high: 0, low: 0 }; // mm1
   let mut adjacent_opp_bits = BitBoard { high: 0, low: 0 }; // mm3
   opp_inner_bits.high = opp_bits.high & 0x7e7e7e7e as u32; /* 0 m7&o6 m6&o5 .. m2&o1 0 */
   opp_inner_bits.low = opp_bits.low & 0x7e7e7e7e as u32; /* 0 m7&o6 (m6&o5)|(m7&o6&o5) .. (m2&o1)|(m3&o2&o1) 0 */
   flip_bits.high = my_bits.high >> 1 & opp_inner_bits.high; /* 0 o7&o6 o6&o5 o5&o4 o4&o3 o3&o2 o2&o1 0 */
   flip_bits.low = my_bits.low >> 1 & opp_inner_bits.low; /* 0 m7&o6 (m6&o5)|(m7&o6&o5) ..|(m7&o6&o5&o4) ..|(m6&o5&o4&o3)|(m7&o6&o5&o4&o3) .. */
   flip_bits.high |= flip_bits.high >> 1 & opp_inner_bits.high;
   flip_bits.low |= flip_bits.low >> 1 & opp_inner_bits.low;
   adjacent_opp_bits.high = opp_inner_bits.high & opp_inner_bits.high >> 1;
   adjacent_opp_bits.low = opp_inner_bits.low & opp_inner_bits.low >> 1;
   flip_bits.high |= flip_bits.high >> 2 & adjacent_opp_bits.high;
   flip_bits.low |= flip_bits.low >> 2 & adjacent_opp_bits.low;
   flip_bits.high |= flip_bits.high >> 2 & adjacent_opp_bits.high;
   flip_bits.low |= flip_bits.low >> 2 & adjacent_opp_bits.low;
   moves.high = flip_bits.high >> 1;
   moves.low = flip_bits.low >> 1;
   flip_bits.high = my_bits.high << 1 & opp_inner_bits.high;
   flip_bits.low = my_bits.low << 1 & opp_inner_bits.low;
   flip_bits.high |= flip_bits.high << 1 & opp_inner_bits.high;
   flip_bits.low |= flip_bits.low << 1 & opp_inner_bits.low;
   adjacent_opp_bits.high = opp_inner_bits.high & opp_inner_bits.high << 1;
   adjacent_opp_bits.low = opp_inner_bits.low & opp_inner_bits.low << 1;
   flip_bits.high |= flip_bits.high << 2 & adjacent_opp_bits.high;
   flip_bits.low |= flip_bits.low << 2 & adjacent_opp_bits.low;
   flip_bits.high |= flip_bits.high << 2 & adjacent_opp_bits.high;
   flip_bits.low |= flip_bits.low << 2 & adjacent_opp_bits.low;
   moves.high |= flip_bits.high << 1;
   moves.low |= flip_bits.low << 1;
   flip_bits.high = my_bits.high >> 8 & opp_bits.high;
   flip_bits.low = (my_bits.low >> 8 | my_bits.high << 24) & opp_bits.low;
   flip_bits.high |= flip_bits.high >> 8 & opp_bits.high;
   flip_bits.low |= (flip_bits.low >> 8 | flip_bits.high << 24) & opp_bits.low;
   adjacent_opp_bits.high = opp_bits.high & opp_bits.high >> 8;
   adjacent_opp_bits.low = opp_bits.low & (opp_bits.low >> 8 | opp_bits.high << 24);
   flip_bits.high |= flip_bits.high >> 16 & adjacent_opp_bits.high;
   flip_bits.low |= (flip_bits.low >> 16 | flip_bits.high << 16) & adjacent_opp_bits.low;
   flip_bits.high |= flip_bits.high >> 16 & adjacent_opp_bits.high;
   flip_bits.low |= (flip_bits.low >> 16 | flip_bits.high << 16) & adjacent_opp_bits.low;
   moves.high |= flip_bits.high >> 8;
   moves.low |= flip_bits.low >> 8 | flip_bits.high << 24;
   flip_bits.high = (my_bits.high << 8 | my_bits.low >> 24) & opp_bits.high;
   flip_bits.low = my_bits.low << 8 & opp_bits.low;
   flip_bits.high |= (flip_bits.high << 8 | flip_bits.low >> 24) & opp_bits.high;
   flip_bits.low |= flip_bits.low << 8 & opp_bits.low;
   adjacent_opp_bits.high = opp_bits.high & (opp_bits.high << 8 | opp_bits.low >> 24);
   adjacent_opp_bits.low = opp_bits.low & opp_bits.low << 8;
   flip_bits.high |= (flip_bits.high << 16 | flip_bits.low >> 16) & adjacent_opp_bits.high;
   flip_bits.low |= flip_bits.low << 16 & adjacent_opp_bits.low;
   flip_bits.high |= (flip_bits.high << 16 | flip_bits.low >> 16) & adjacent_opp_bits.high;
   flip_bits.low |= flip_bits.low << 16 & adjacent_opp_bits.low;
   moves.high |= flip_bits.high << 8 | flip_bits.low >> 24;
   moves.low |= flip_bits.low << 8;
   flip_bits.high = my_bits.high >> 7 & opp_inner_bits.high;
   flip_bits.low = (my_bits.low >> 7 | my_bits.high << 25) & opp_inner_bits.low;
   flip_bits.high |= flip_bits.high >> 7 & opp_inner_bits.high;
   flip_bits.low |= (flip_bits.low >> 7 | flip_bits.high << 25) & opp_inner_bits.low;
   adjacent_opp_bits.high = opp_inner_bits.high & opp_inner_bits.high >> 7;
   adjacent_opp_bits.low = opp_inner_bits.low & (opp_inner_bits.low >> 7 | opp_inner_bits.high << 25);
   flip_bits.high |= flip_bits.high >> 14 & adjacent_opp_bits.high;
   flip_bits.low |= (flip_bits.low >> 14 | flip_bits.high << 18) & adjacent_opp_bits.low;
   flip_bits.high |= flip_bits.high >> 14 & adjacent_opp_bits.high;
   flip_bits.low |= (flip_bits.low >> 14 | flip_bits.high << 18) & adjacent_opp_bits.low;
   moves.high |= flip_bits.high >> 7;
   moves.low |= flip_bits.low >> 7 | flip_bits.high << 25;
   flip_bits.high = (my_bits.high << 7 | my_bits.low >> 25) & opp_inner_bits.high;
   flip_bits.low = my_bits.low << 7 & opp_inner_bits.low;
   flip_bits.high |= (flip_bits.high << 7 | flip_bits.low >> 25) & opp_inner_bits.high;
   flip_bits.low |= flip_bits.low << 7 & opp_inner_bits.low;
   adjacent_opp_bits.high = opp_inner_bits.high & (opp_inner_bits.high << 7 | opp_inner_bits.low >> 25);
   adjacent_opp_bits.low = opp_inner_bits.low & opp_inner_bits.low << 7;
   flip_bits.high |= (flip_bits.high << 14 | flip_bits.low >> 18) & adjacent_opp_bits.high;
   flip_bits.low |= flip_bits.low << 14 & adjacent_opp_bits.low;
   flip_bits.high |= (flip_bits.high << 14 | flip_bits.low >> 18) & adjacent_opp_bits.high;
   flip_bits.low |= flip_bits.low << 14 & adjacent_opp_bits.low;
   moves.high |= flip_bits.high << 7 | flip_bits.low >> 25;
   moves.low |= flip_bits.low << 7;
   flip_bits.high = my_bits.high >> 9 & opp_inner_bits.high;
   flip_bits.low = (my_bits.low >> 9 | my_bits.high << 23) & opp_inner_bits.low;
   flip_bits.high |= flip_bits.high >> 9 & opp_inner_bits.high;
   flip_bits.low |= (flip_bits.low >> 9 | flip_bits.high << 23) & opp_inner_bits.low;
   adjacent_opp_bits.high = opp_inner_bits.high & opp_inner_bits.high >> 9;
   adjacent_opp_bits.low = opp_inner_bits.low & (opp_inner_bits.low >> 9 | opp_inner_bits.high << 23);
   flip_bits.high |= flip_bits.high >> 18 & adjacent_opp_bits.high;
   flip_bits.low |= (flip_bits.low >> 18 | flip_bits.high << 14) & adjacent_opp_bits.low;
   flip_bits.high |= flip_bits.high >> 18 & adjacent_opp_bits.high;
   flip_bits.low |= (flip_bits.low >> 18 | flip_bits.high << 14) & adjacent_opp_bits.low;
   moves.high |= flip_bits.high >> 9;
   moves.low |= flip_bits.low >> 9 | flip_bits.high << 23;
   flip_bits.high = (my_bits.high << 9 | my_bits.low >> 23) & opp_inner_bits.high;
   flip_bits.low = my_bits.low << 9 & opp_inner_bits.low;
   flip_bits.high |= (flip_bits.high << 9 | flip_bits.low >> 23) & opp_inner_bits.high;
   flip_bits.low |= flip_bits.low << 9 & opp_inner_bits.low;
   adjacent_opp_bits.high = opp_inner_bits.high & (opp_inner_bits.high << 9 | opp_inner_bits.low >> 23);
   adjacent_opp_bits.low = opp_inner_bits.low & opp_inner_bits.low << 9;
   flip_bits.high |= (flip_bits.high << 18 | flip_bits.low >> 14) & adjacent_opp_bits.high;
   flip_bits.low |= flip_bits.low << 18 & adjacent_opp_bits.low;
   flip_bits.high |= (flip_bits.high << 18 | flip_bits.low >> 14) & adjacent_opp_bits.high;
   flip_bits.low |= flip_bits.low << 18 & adjacent_opp_bits.low;
   moves.high |= flip_bits.high << 9 | flip_bits.low >> 23;
   moves.low |= flip_bits.low << 9;
   moves.high &= !(my_bits.high | opp_bits.high);
   moves.low &= !(my_bits.low | opp_bits.low);
   return moves;
}

pub fn bitboard_mobility(my_bits: BitBoard,
                                           opp_bits: BitBoard)
 -> i32 {
    let mut moves = BitBoard{high: 0, low: 0,};
    let mut count: u32 = 0;
    moves = generate_all_c(my_bits, opp_bits);
    count = non_iterative_popcount(moves.high, moves.low);
    return count as i32;
}
/*
   File:          bitbmob.h

   Created:       November 22, 1999

   Modified:      December 25, 2002

   Authors:       Gunnar Andersson (gunnar@radagast.se)

   Contents:
*/

pub fn weighted_mobility(my_bits: BitBoard,
                                           opp_bits: BitBoard)
 -> i32 {
    let mut n1: u32 = 0; /* corner bonus for A1/H1/A8/H8 */
    let mut n2: u32 = 0;
    let mut moves = BitBoard{high: 0, low: 0,};
    moves = generate_all_c(my_bits, opp_bits);
    n1 =
        moves.high.wrapping_sub(moves.high >> 1 &
                                    0x15555555 as
                                        u32).wrapping_add(moves.high
                                                                       &
                                                                       0x1000000
                                                                           as
                                                                           u32);
    n2 =
        moves.low.wrapping_sub(moves.low >> 1 &
                                   0x55555515 as
                                       u32).wrapping_add(moves.low &
                                                                      0x1 as
                                                                          u32);
    n1 =
        (n1 &
             0x33333333 as
                 u32).wrapping_add(n1 >> 2 &
                                                0x33333333 as u32);
    n2 =
        (n2 &
             0x33333333 as
                 u32).wrapping_add(n2 >> 2 &
                                                0x33333333 as u32);
    n1 = n1.wrapping_add(n1 >> 4) & 0xf0f0f0f as u32;
    n1 =
        n1.wrapping_add(n2.wrapping_add(n2 >> 4) &
                            0xf0f0f0f as u32);
    return (n1.wrapping_mul(0x1010101 as u32) >>
                24).wrapping_mul(128 as
                                                  u32) as
               i32;
}
