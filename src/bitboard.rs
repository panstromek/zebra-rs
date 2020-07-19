use crate::src::libc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitBoard {
    pub high: libc::c_uint,
    pub low: libc::c_uint,
}
/*
   File:          bitboard.c

   Created:       November 21, 1999

   Modified:      November 24, 2005

   Authors:       Gunnar Andersson (gunnar@radagast.se)
                  Toshihiko Okuhara

   Contents:      Basic bitboard manipulations
*/

pub static mut square_mask: [BitBoard; 100] =
    [BitBoard{high: 0, low: 0,}; 100];
/*
  NON_ITERATIVE_POPCOUNT
  Counts the number of bits set in a 64-bit integer.
  This is done using some bitfiddling tricks.
*/

pub unsafe extern "C" fn non_iterative_popcount(mut n1: libc::c_uint,
                                                mut n2: libc::c_uint)
 -> libc::c_uint {
    n1 = n1.wrapping_sub(n1 >> 1 as libc::c_int & 0x55555555 as libc::c_uint);
    n2 = n2.wrapping_sub(n2 >> 1 as libc::c_int & 0x55555555 as libc::c_uint);
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
    n2 = n2.wrapping_add(n2 >> 4 as libc::c_int) & 0xf0f0f0f as libc::c_uint;
    return n1.wrapping_add(n2).wrapping_mul(0x1010101 as libc::c_uint) >>
               24 as libc::c_int;
}
/*
  ITERATIVE_POPCOUNT
  Counts the number of bits set in a 64-bit integer.
  This is done using an iterative procedure which loops
  a number of times equal to the number of bits set,
  hence this function is fast when the number of bits
  set is low.
*/

pub unsafe extern "C" fn iterative_popcount(mut n1: libc::c_uint,
                                            mut n2: libc::c_uint)
 -> libc::c_uint {
    let mut n: libc::c_uint = 0;
    n = 0 as libc::c_int as libc::c_uint;
    while n1 != 0 as libc::c_int as libc::c_uint {
        n = n.wrapping_add(1);
        n1 &= n1.wrapping_sub(1 as libc::c_int as libc::c_uint)
    }
    while n2 != 0 as libc::c_int as libc::c_uint {
        n = n.wrapping_add(1);
        n2 &= n2.wrapping_sub(1 as libc::c_int as libc::c_uint)
    }
    return n;
}
/*
  BIT_REVERSE_32
  Returns the bit-reverse of a 32-bit integer.
*/

pub unsafe extern "C" fn bit_reverse_32(mut val: libc::c_uint)
 -> libc::c_uint {
    val =
        val >> 1 as libc::c_int & 0x55555555 as libc::c_int as libc::c_uint |
            val << 1 as libc::c_int & 0xaaaaaaaa as libc::c_uint;
    val =
        val >> 2 as libc::c_int & 0x33333333 as libc::c_int as libc::c_uint |
            val << 2 as libc::c_int & 0xcccccccc as libc::c_uint;
    val =
        val >> 4 as libc::c_int & 0xf0f0f0f as libc::c_int as libc::c_uint |
            val << 4 as libc::c_int & 0xf0f0f0f0 as libc::c_uint;
    val =
        val >> 8 as libc::c_int & 0xff00ff as libc::c_int as libc::c_uint |
            val << 8 as libc::c_int & 0xff00ff00 as libc::c_uint;
    val =
        val >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint |
            val << 16 as libc::c_int & 0xffff0000 as libc::c_uint;
    return val;
}
/*
  SET_BITBOARDS
  Converts the vector board representation to the bitboard representation.
*/

pub unsafe extern "C" fn set_bitboards(mut board: *mut libc::c_int,
                                       mut side_to_move: libc::c_int,
                                       mut my_out: *mut BitBoard,
                                       mut opp_out: *mut BitBoard) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut mask: libc::c_uint = 0;
    let mut my_bits = BitBoard{high: 0, low: 0,};
    let mut opp_bits = BitBoard{high: 0, low: 0,};
    my_bits.high = 0 as libc::c_int as libc::c_uint;
    my_bits.low = 0 as libc::c_int as libc::c_uint;
    opp_bits.high = 0 as libc::c_int as libc::c_uint;
    opp_bits.low = 0 as libc::c_int as libc::c_uint;
    mask = 1 as libc::c_int as libc::c_uint;
    i = 1 as libc::c_int;
    while i <= 4 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            pos = 10 as libc::c_int * i + j;
            if *board.offset(pos as isize) == side_to_move {
                my_bits.low |= mask
            } else if *board.offset(pos as isize) ==
                          0 as libc::c_int + 2 as libc::c_int - side_to_move {
                opp_bits.low |= mask
            }
            j += 1;
            mask <<= 1 as libc::c_int
        }
        i += 1
    }
    mask = 1 as libc::c_int as libc::c_uint;
    i = 5 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            pos = 10 as libc::c_int * i + j;
            if *board.offset(pos as isize) == side_to_move {
                my_bits.high |= mask
            } else if *board.offset(pos as isize) ==
                          0 as libc::c_int + 2 as libc::c_int - side_to_move {
                opp_bits.high |= mask
            }
            j += 1;
            mask <<= 1 as libc::c_int
        }
        i += 1
    }
    *my_out = my_bits;
    *opp_out = opp_bits;
}

pub unsafe extern "C" fn init_bitboard() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            let mut pos = 10 as libc::c_int * i + j;
            let mut shift =
                (8 as libc::c_int * (i - 1 as libc::c_int) +
                     (j - 1 as libc::c_int)) as libc::c_uint;
            if shift < 32 as libc::c_int as libc::c_uint {
                square_mask[pos as usize].low =
                    ((1 as libc::c_ulong) << shift) as libc::c_uint;
                square_mask[pos as usize].high =
                    0 as libc::c_int as libc::c_uint
            } else {
                square_mask[pos as usize].low =
                    0 as libc::c_int as libc::c_uint;
                square_mask[pos as usize].high =
                    ((1 as libc::c_ulong) <<
                         shift.wrapping_sub(32 as libc::c_int as
                                                libc::c_uint)) as libc::c_uint
            }
            j += 1
        }
        i += 1
    };
}
