#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitBoard {
    pub high: u32,
    pub low: u32,
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

pub unsafe fn non_iterative_popcount(mut n1: u32,
                                                mut n2: u32)
 -> u32 {
    n1 = n1.wrapping_sub(n1 >> 1 as i32 & 0x55555555 as u32);
    n2 = n2.wrapping_sub(n2 >> 1 as i32 & 0x55555555 as u32);
    n1 =
        (n1 &
             0x33333333 as
                 u32).wrapping_add(n1 >> 2 as i32 &
                                                0x33333333 as u32);
    n2 =
        (n2 &
             0x33333333 as
                 u32).wrapping_add(n2 >> 2 as i32 &
                                                0x33333333 as u32);
    n1 = n1.wrapping_add(n1 >> 4 as i32) & 0xf0f0f0f as u32;
    n2 = n2.wrapping_add(n2 >> 4 as i32) & 0xf0f0f0f as u32;
    return n1.wrapping_add(n2).wrapping_mul(0x1010101 as u32) >>
               24 as i32;
}
/*
  ITERATIVE_POPCOUNT
  Counts the number of bits set in a 64-bit integer.
  This is done using an iterative procedure which loops
  a number of times equal to the number of bits set,
  hence this function is fast when the number of bits
  set is low.
*/

pub unsafe fn iterative_popcount(mut n1: u32,
                                            mut n2: u32)
 -> u32 {
    let mut n: u32 = 0;
    n = 0 as i32 as u32;
    while n1 != 0 as i32 as u32 {
        n = n.wrapping_add(1);
        n1 &= n1.wrapping_sub(1 as i32 as u32)
    }
    while n2 != 0 as i32 as u32 {
        n = n.wrapping_add(1);
        n2 &= n2.wrapping_sub(1 as i32 as u32)
    }
    return n;
}
/*
  BIT_REVERSE_32
  Returns the bit-reverse of a 32-bit integer.
*/

pub unsafe fn bit_reverse_32(mut val: u32)
 -> u32 {
    val =
        val >> 1 as i32 & 0x55555555 as i32 as u32 |
            val << 1 as i32 & 0xaaaaaaaa as u32;
    val =
        val >> 2 as i32 & 0x33333333 as i32 as u32 |
            val << 2 as i32 & 0xcccccccc as u32;
    val =
        val >> 4 as i32 & 0xf0f0f0f as i32 as u32 |
            val << 4 as i32 & 0xf0f0f0f0 as u32;
    val =
        val >> 8 as i32 & 0xff00ff as i32 as u32 |
            val << 8 as i32 & 0xff00ff00 as u32;
    val =
        val >> 16 as i32 & 0xffff as i32 as u32 |
            val << 16 as i32 & 0xffff0000 as u32;
    return val;
}
/*
  SET_BITBOARDS
  Converts the vector board representation to the bitboard representation.
*/

pub unsafe fn set_bitboards(mut board: *mut i32,
                                       mut side_to_move: i32,
                                       mut my_out: *mut BitBoard,
                                       mut opp_out: *mut BitBoard) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut mask: u32 = 0;
    let mut my_bits = BitBoard{high: 0, low: 0,};
    let mut opp_bits = BitBoard{high: 0, low: 0,};
    my_bits.high = 0 as i32 as u32;
    my_bits.low = 0 as i32 as u32;
    opp_bits.high = 0 as i32 as u32;
    opp_bits.low = 0 as i32 as u32;
    mask = 1 as i32 as u32;
    i = 1 as i32;
    while i <= 4 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            pos = 10 as i32 * i + j;
            if *board.offset(pos as isize) == side_to_move {
                my_bits.low |= mask
            } else if *board.offset(pos as isize) ==
                          0 as i32 + 2 as i32 - side_to_move {
                opp_bits.low |= mask
            }
            j += 1;
            mask <<= 1 as i32
        }
        i += 1
    }
    mask = 1 as i32 as u32;
    i = 5 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            pos = 10 as i32 * i + j;
            if *board.offset(pos as isize) == side_to_move {
                my_bits.high |= mask
            } else if *board.offset(pos as isize) ==
                          0 as i32 + 2 as i32 - side_to_move {
                opp_bits.high |= mask
            }
            j += 1;
            mask <<= 1 as i32
        }
        i += 1
    }
    *my_out = my_bits;
    *opp_out = opp_bits;
}

pub unsafe fn init_bitboard() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            let mut pos = 10 as i32 * i + j;
            let mut shift =
                (8 as i32 * (i - 1 as i32) +
                     (j - 1 as i32)) as u32;
            if shift < 32 as i32 as u32 {
                square_mask[pos as usize].low =
                    ((1 as u64) << shift) as u32;
                square_mask[pos as usize].high =
                    0 as i32 as u32
            } else {
                square_mask[pos as usize].low =
                    0 as i32 as u32;
                square_mask[pos as usize].high =
                    ((1 as u64) <<
                         shift.wrapping_sub(32 as i32 as
                                                u32)) as u32
            }
            j += 1
        }
        i += 1
    };
}
