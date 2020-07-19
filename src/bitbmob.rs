
use crate::src::bitboard::{BitBoard, non_iterative_popcount};
use crate::src::libc;

/*
   File:          bitbmob.c

   Modified:      November 18, 2005

   Authors:       Gunnar Andersson (gunnar@radagast.se)
              Toshihiko Okuhara

   Contents:      Count feasible moves in the bitboard.

   This piece of software is released under the GPL.
   See the file COPYING for more information.
*/

pub unsafe extern "C" fn init_mmx() { }
unsafe extern "C" fn generate_all_c(my_bits: BitBoard, opp_bits: BitBoard)
 -> BitBoard {
    // mm6
    let mut moves = BitBoard{high: 0, low: 0,}; // mm4
    let mut opp_inner_bits = BitBoard{high: 0, low: 0,}; // mm5
    let mut flip_bits = BitBoard{high: 0, low: 0,}; // mm1
    let mut adjacent_opp_bits = BitBoard{high: 0, low: 0,}; // mm3
    opp_inner_bits.high =
        opp_bits.high &
            0x7e7e7e7e as libc::c_uint; /* 0 m7&o6 m6&o5 .. m2&o1 0 */
    opp_inner_bits.low =
        opp_bits.low &
            0x7e7e7e7e as
                libc::c_uint; /* 0 m7&o6 (m6&o5)|(m7&o6&o5) .. (m2&o1)|(m3&o2&o1) 0 */
    flip_bits.high =
        my_bits.high >> 1 as libc::c_int &
            opp_inner_bits.high; /* 0 o7&o6 o6&o5 o5&o4 o4&o3 o3&o2 o2&o1 0 */
    flip_bits.low =
        my_bits.low >> 1 as libc::c_int &
            opp_inner_bits.low; /* 0 m7&o6 (m6&o5)|(m7&o6&o5) ..|(m7&o6&o5&o4) ..|(m6&o5&o4&o3)|(m7&o6&o5&o4&o3) .. */
    flip_bits.high |=
        flip_bits.high >> 1 as libc::c_int & opp_inner_bits.high;
    flip_bits.low |= flip_bits.low >> 1 as libc::c_int & opp_inner_bits.low;
    adjacent_opp_bits.high =
        opp_inner_bits.high & opp_inner_bits.high >> 1 as libc::c_int;
    adjacent_opp_bits.low =
        opp_inner_bits.low & opp_inner_bits.low >> 1 as libc::c_int;
    flip_bits.high |=
        flip_bits.high >> 2 as libc::c_int & adjacent_opp_bits.high;
    flip_bits.low |=
        flip_bits.low >> 2 as libc::c_int & adjacent_opp_bits.low;
    flip_bits.high |=
        flip_bits.high >> 2 as libc::c_int & adjacent_opp_bits.high;
    flip_bits.low |=
        flip_bits.low >> 2 as libc::c_int & adjacent_opp_bits.low;
    moves.high = flip_bits.high >> 1 as libc::c_int;
    moves.low = flip_bits.low >> 1 as libc::c_int;
    flip_bits.high = my_bits.high << 1 as libc::c_int & opp_inner_bits.high;
    flip_bits.low = my_bits.low << 1 as libc::c_int & opp_inner_bits.low;
    flip_bits.high |=
        flip_bits.high << 1 as libc::c_int & opp_inner_bits.high;
    flip_bits.low |= flip_bits.low << 1 as libc::c_int & opp_inner_bits.low;
    adjacent_opp_bits.high =
        opp_inner_bits.high & opp_inner_bits.high << 1 as libc::c_int;
    adjacent_opp_bits.low =
        opp_inner_bits.low & opp_inner_bits.low << 1 as libc::c_int;
    flip_bits.high |=
        flip_bits.high << 2 as libc::c_int & adjacent_opp_bits.high;
    flip_bits.low |=
        flip_bits.low << 2 as libc::c_int & adjacent_opp_bits.low;
    flip_bits.high |=
        flip_bits.high << 2 as libc::c_int & adjacent_opp_bits.high;
    flip_bits.low |=
        flip_bits.low << 2 as libc::c_int & adjacent_opp_bits.low;
    moves.high |= flip_bits.high << 1 as libc::c_int;
    moves.low |= flip_bits.low << 1 as libc::c_int;
    flip_bits.high = my_bits.high >> 8 as libc::c_int & opp_bits.high;
    flip_bits.low =
        (my_bits.low >> 8 as libc::c_int | my_bits.high << 24 as libc::c_int)
            & opp_bits.low;
    flip_bits.high |= flip_bits.high >> 8 as libc::c_int & opp_bits.high;
    flip_bits.low |=
        (flip_bits.low >> 8 as libc::c_int |
             flip_bits.high << 24 as libc::c_int) & opp_bits.low;
    adjacent_opp_bits.high =
        opp_bits.high & opp_bits.high >> 8 as libc::c_int;
    adjacent_opp_bits.low =
        opp_bits.low &
            (opp_bits.low >> 8 as libc::c_int |
                 opp_bits.high << 24 as libc::c_int);
    flip_bits.high |=
        flip_bits.high >> 16 as libc::c_int & adjacent_opp_bits.high;
    flip_bits.low |=
        (flip_bits.low >> 16 as libc::c_int |
             flip_bits.high << 16 as libc::c_int) & adjacent_opp_bits.low;
    flip_bits.high |=
        flip_bits.high >> 16 as libc::c_int & adjacent_opp_bits.high;
    flip_bits.low |=
        (flip_bits.low >> 16 as libc::c_int |
             flip_bits.high << 16 as libc::c_int) & adjacent_opp_bits.low;
    moves.high |= flip_bits.high >> 8 as libc::c_int;
    moves.low |=
        flip_bits.low >> 8 as libc::c_int |
            flip_bits.high << 24 as libc::c_int;
    flip_bits.high =
        (my_bits.high << 8 as libc::c_int | my_bits.low >> 24 as libc::c_int)
            & opp_bits.high;
    flip_bits.low = my_bits.low << 8 as libc::c_int & opp_bits.low;
    flip_bits.high |=
        (flip_bits.high << 8 as libc::c_int |
             flip_bits.low >> 24 as libc::c_int) & opp_bits.high;
    flip_bits.low |= flip_bits.low << 8 as libc::c_int & opp_bits.low;
    adjacent_opp_bits.high =
        opp_bits.high &
            (opp_bits.high << 8 as libc::c_int |
                 opp_bits.low >> 24 as libc::c_int);
    adjacent_opp_bits.low = opp_bits.low & opp_bits.low << 8 as libc::c_int;
    flip_bits.high |=
        (flip_bits.high << 16 as libc::c_int |
             flip_bits.low >> 16 as libc::c_int) & adjacent_opp_bits.high;
    flip_bits.low |=
        flip_bits.low << 16 as libc::c_int & adjacent_opp_bits.low;
    flip_bits.high |=
        (flip_bits.high << 16 as libc::c_int |
             flip_bits.low >> 16 as libc::c_int) & adjacent_opp_bits.high;
    flip_bits.low |=
        flip_bits.low << 16 as libc::c_int & adjacent_opp_bits.low;
    moves.high |=
        flip_bits.high << 8 as libc::c_int |
            flip_bits.low >> 24 as libc::c_int;
    moves.low |= flip_bits.low << 8 as libc::c_int;
    flip_bits.high = my_bits.high >> 7 as libc::c_int & opp_inner_bits.high;
    flip_bits.low =
        (my_bits.low >> 7 as libc::c_int | my_bits.high << 25 as libc::c_int)
            & opp_inner_bits.low;
    flip_bits.high |=
        flip_bits.high >> 7 as libc::c_int & opp_inner_bits.high;
    flip_bits.low |=
        (flip_bits.low >> 7 as libc::c_int |
             flip_bits.high << 25 as libc::c_int) & opp_inner_bits.low;
    adjacent_opp_bits.high =
        opp_inner_bits.high & opp_inner_bits.high >> 7 as libc::c_int;
    adjacent_opp_bits.low =
        opp_inner_bits.low &
            (opp_inner_bits.low >> 7 as libc::c_int |
                 opp_inner_bits.high << 25 as libc::c_int);
    flip_bits.high |=
        flip_bits.high >> 14 as libc::c_int & adjacent_opp_bits.high;
    flip_bits.low |=
        (flip_bits.low >> 14 as libc::c_int |
             flip_bits.high << 18 as libc::c_int) & adjacent_opp_bits.low;
    flip_bits.high |=
        flip_bits.high >> 14 as libc::c_int & adjacent_opp_bits.high;
    flip_bits.low |=
        (flip_bits.low >> 14 as libc::c_int |
             flip_bits.high << 18 as libc::c_int) & adjacent_opp_bits.low;
    moves.high |= flip_bits.high >> 7 as libc::c_int;
    moves.low |=
        flip_bits.low >> 7 as libc::c_int |
            flip_bits.high << 25 as libc::c_int;
    flip_bits.high =
        (my_bits.high << 7 as libc::c_int | my_bits.low >> 25 as libc::c_int)
            & opp_inner_bits.high;
    flip_bits.low = my_bits.low << 7 as libc::c_int & opp_inner_bits.low;
    flip_bits.high |=
        (flip_bits.high << 7 as libc::c_int |
             flip_bits.low >> 25 as libc::c_int) & opp_inner_bits.high;
    flip_bits.low |= flip_bits.low << 7 as libc::c_int & opp_inner_bits.low;
    adjacent_opp_bits.high =
        opp_inner_bits.high &
            (opp_inner_bits.high << 7 as libc::c_int |
                 opp_inner_bits.low >> 25 as libc::c_int);
    adjacent_opp_bits.low =
        opp_inner_bits.low & opp_inner_bits.low << 7 as libc::c_int;
    flip_bits.high |=
        (flip_bits.high << 14 as libc::c_int |
             flip_bits.low >> 18 as libc::c_int) & adjacent_opp_bits.high;
    flip_bits.low |=
        flip_bits.low << 14 as libc::c_int & adjacent_opp_bits.low;
    flip_bits.high |=
        (flip_bits.high << 14 as libc::c_int |
             flip_bits.low >> 18 as libc::c_int) & adjacent_opp_bits.high;
    flip_bits.low |=
        flip_bits.low << 14 as libc::c_int & adjacent_opp_bits.low;
    moves.high |=
        flip_bits.high << 7 as libc::c_int |
            flip_bits.low >> 25 as libc::c_int;
    moves.low |= flip_bits.low << 7 as libc::c_int;
    flip_bits.high = my_bits.high >> 9 as libc::c_int & opp_inner_bits.high;
    flip_bits.low =
        (my_bits.low >> 9 as libc::c_int | my_bits.high << 23 as libc::c_int)
            & opp_inner_bits.low;
    flip_bits.high |=
        flip_bits.high >> 9 as libc::c_int & opp_inner_bits.high;
    flip_bits.low |=
        (flip_bits.low >> 9 as libc::c_int |
             flip_bits.high << 23 as libc::c_int) & opp_inner_bits.low;
    adjacent_opp_bits.high =
        opp_inner_bits.high & opp_inner_bits.high >> 9 as libc::c_int;
    adjacent_opp_bits.low =
        opp_inner_bits.low &
            (opp_inner_bits.low >> 9 as libc::c_int |
                 opp_inner_bits.high << 23 as libc::c_int);
    flip_bits.high |=
        flip_bits.high >> 18 as libc::c_int & adjacent_opp_bits.high;
    flip_bits.low |=
        (flip_bits.low >> 18 as libc::c_int |
             flip_bits.high << 14 as libc::c_int) & adjacent_opp_bits.low;
    flip_bits.high |=
        flip_bits.high >> 18 as libc::c_int & adjacent_opp_bits.high;
    flip_bits.low |=
        (flip_bits.low >> 18 as libc::c_int |
             flip_bits.high << 14 as libc::c_int) & adjacent_opp_bits.low;
    moves.high |= flip_bits.high >> 9 as libc::c_int;
    moves.low |=
        flip_bits.low >> 9 as libc::c_int |
            flip_bits.high << 23 as libc::c_int;
    flip_bits.high =
        (my_bits.high << 9 as libc::c_int | my_bits.low >> 23 as libc::c_int)
            & opp_inner_bits.high;
    flip_bits.low = my_bits.low << 9 as libc::c_int & opp_inner_bits.low;
    flip_bits.high |=
        (flip_bits.high << 9 as libc::c_int |
             flip_bits.low >> 23 as libc::c_int) & opp_inner_bits.high;
    flip_bits.low |= flip_bits.low << 9 as libc::c_int & opp_inner_bits.low;
    adjacent_opp_bits.high =
        opp_inner_bits.high &
            (opp_inner_bits.high << 9 as libc::c_int |
                 opp_inner_bits.low >> 23 as libc::c_int);
    adjacent_opp_bits.low =
        opp_inner_bits.low & opp_inner_bits.low << 9 as libc::c_int;
    flip_bits.high |=
        (flip_bits.high << 18 as libc::c_int |
             flip_bits.low >> 14 as libc::c_int) & adjacent_opp_bits.high;
    flip_bits.low |=
        flip_bits.low << 18 as libc::c_int & adjacent_opp_bits.low;
    flip_bits.high |=
        (flip_bits.high << 18 as libc::c_int |
             flip_bits.low >> 14 as libc::c_int) & adjacent_opp_bits.high;
    flip_bits.low |=
        flip_bits.low << 18 as libc::c_int & adjacent_opp_bits.low;
    moves.high |=
        flip_bits.high << 9 as libc::c_int |
            flip_bits.low >> 23 as libc::c_int;
    moves.low |= flip_bits.low << 9 as libc::c_int;
    moves.high &= !(my_bits.high | opp_bits.high);
    moves.low &= !(my_bits.low | opp_bits.low);
    return moves;
}

pub unsafe extern "C" fn bitboard_mobility(my_bits: BitBoard,
                                           opp_bits: BitBoard)
 -> libc::c_int {
    let mut moves = BitBoard{high: 0, low: 0,};
    let mut count: libc::c_uint = 0;
    moves = generate_all_c(my_bits, opp_bits);
    count = non_iterative_popcount(moves.high, moves.low);
    return count as libc::c_int;
}
/*
   File:          bitbmob.h

   Created:       November 22, 1999

   Modified:      December 25, 2002

   Authors:       Gunnar Andersson (gunnar@radagast.se)

   Contents:
*/

pub unsafe extern "C" fn weighted_mobility(my_bits: BitBoard,
                                           opp_bits: BitBoard)
 -> libc::c_int {
    let mut n1: libc::c_uint = 0; /* corner bonus for A1/H1/A8/H8 */
    let mut n2: libc::c_uint = 0;
    let mut moves = BitBoard{high: 0, low: 0,};
    moves = generate_all_c(my_bits, opp_bits);
    n1 =
        moves.high.wrapping_sub(moves.high >> 1 as libc::c_int &
                                    0x15555555 as
                                        libc::c_uint).wrapping_add(moves.high
                                                                       &
                                                                       0x1000000
                                                                           as
                                                                           libc::c_uint);
    n2 =
        moves.low.wrapping_sub(moves.low >> 1 as libc::c_int &
                                   0x55555515 as
                                       libc::c_uint).wrapping_add(moves.low &
                                                                      0x1 as
                                                                          libc::c_uint);
    n1 =
        (n1 &
             0x33333333 as
                 libc::c_uint).wrapping_add(n1 >> 2 as libc::c_int &
                                                0x33333333 as libc::c_uint);
    n2 =
        (n2 &
             0x33333333 as
                 libc::c_uint).wrapping_add(n2 >> 2 as libc::c_int &
                                                0x33333333 as libc::c_uint);
    n1 = n1.wrapping_add(n1 >> 4 as libc::c_int) & 0xf0f0f0f as libc::c_uint;
    n1 =
        n1.wrapping_add(n2.wrapping_add(n2 >> 4 as libc::c_int) &
                            0xf0f0f0f as libc::c_uint);
    return (n1.wrapping_mul(0x1010101 as libc::c_uint) >>
                24 as
                    libc::c_int).wrapping_mul(128 as libc::c_int as
                                                  libc::c_uint) as
               libc::c_int;
}
