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

pub static square_mask: [BitBoard; 100] = create_square_mask();
/*
  NON_ITERATIVE_POPCOUNT
  Counts the number of bits set in a 64-bit integer.
  This is done using some bitfiddling tricks.
*/

pub fn non_iterative_popcount(mut n1: u32, mut n2: u32) -> u32 {
    n1 = n1.wrapping_sub(n1 >> 1 & 0x55555555);
    n2 = n2.wrapping_sub(n2 >> 1 & 0x55555555);
    n1 = (n1 & 0x33333333).wrapping_add(n1 >> 2 & 0x33333333);
    n2 = (n2 & 0x33333333).wrapping_add(n2 >> 2 & 0x33333333);
    n1 = n1.wrapping_add(n1 >> 4) & 0xf0f0f0f;
    n2 = n2.wrapping_add(n2 >> 4) & 0xf0f0f0f;
    return n1.wrapping_add(n2).wrapping_mul(0x1010101) >> 24;
}
/*
  BIT_REVERSE_32
  Returns the bit-reverse of a 32-bit integer.
*/

pub fn bit_reverse_32(mut val: u32)
 -> u32 {
    val =
        val >> 1 & 0x55555555 as i32 as u32 |
            val << 1 & 0xaaaaaaaa as u32;
    val =
        val >> 2 & 0x33333333 as i32 as u32 |
            val << 2 & 0xcccccccc as u32;
    val =
        val >> 4 & 0xf0f0f0f as i32 as u32 |
            val << 4 & 0xf0f0f0f0 as u32;
    val =
        val >> 8 & 0xff00ff as i32 as u32 |
            val << 8 & 0xff00ff00 as u32;
    val =
        val >> 16 & 0xffff as i32 as u32 |
            val << 16 & 0xffff0000 as u32;
    return val;
}
/*
  SET_BITBOARDS
  Converts the vector board representation to the bitboard representation.
*/

pub fn set_bitboards(board: &[i32; 128],
                                       side_to_move: i32,
                                       my_out: &mut BitBoard,
                                       opp_out: &mut BitBoard) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut mask: u32 = 0;
    let mut my_bits = BitBoard{high: 0, low: 0,};
    let mut opp_bits = BitBoard{high: 0, low: 0,};
    my_bits.high = 0;
    my_bits.low = 0;
    opp_bits.high = 0;
    opp_bits.low = 0;
    mask = 1;
    i = 1;
    while i <= 4 {
        j = 1;
        while j <= 8 {
            pos = 10 * i + j;
            if board[pos as usize] == side_to_move {
                my_bits.low |= mask
            } else if board[pos as usize] ==
                          0 + 2 - side_to_move {
                opp_bits.low |= mask
            }
            j += 1;
            mask <<= 1
        }
        i += 1
    }
    mask = 1;
    i = 5;
    while i <= 8 {
        j = 1;
        while j <= 8 {
            pos = 10 * i + j;
            if board[pos as usize] == side_to_move {
                my_bits.high |= mask
            } else if board[pos as usize] ==
                          0 + 2 - side_to_move {
                opp_bits.high |= mask
            }
            j += 1;
            mask <<= 1
        }
        i += 1
    }
    *my_out = my_bits;
    *opp_out = opp_bits;
}

const fn create_square_mask() -> [BitBoard; 100] {
    let mut square_mask_: [BitBoard; 100] = [BitBoard{high: 0, low: 0,}; 100];
    let mut j = 0;
    let mut i = 1;
    while i <= 8 {
        j = 1;
        while j <= 8 {
            let pos = 10 * i + j;
            let shift =
                (8 * (i - 1) +
                     (j - 1)) as u32;
            if shift < 32 {
                square_mask_[pos as usize].low =
                    ((1 as u64) << shift) as u32;
                square_mask_[pos as usize].high =
                    0
            } else {
                square_mask_[pos as usize].low = 0;
                square_mask_[pos as usize].high =
                    ((1 as u64) <<
                         shift.wrapping_sub(32)) as u32
            }
            j += 1
        }
        i += 1
    };
    square_mask_
}
