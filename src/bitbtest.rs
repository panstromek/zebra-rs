use crate::src::bitboard::BitBoard;
use crate::src::libc;
/*
   File:          bitbtest.c

   Modified:      November 24, 2005

   Authors:       Gunnar Andersson (gunnar@radagast.se)
              Toshihiko Okuhara

   Contents:      Count flips and returns new_my_bits in bb_flips.

   This piece of software is released under the GPL.
   See the file COPYING for more information.
*/

pub static mut bb_flips: BitBoard = BitBoard{high: 0, low: 0,};
static mut right_contiguous: [u8; 64] =
    [0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 2 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 3 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 2 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 4 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 2 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 3 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 2 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 5 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 2 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 3 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 2 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 4 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 2 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 3 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 2 as i32 as u8,
     0 as i32 as u8, 1 as i32 as u8,
     0 as i32 as u8, 6 as i32 as u8];
static mut left_contiguous: [u8; 64] =
    [0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     1 as i32 as u8, 1 as i32 as u8,
     1 as i32 as u8, 1 as i32 as u8,
     1 as i32 as u8, 1 as i32 as u8,
     1 as i32 as u8, 1 as i32 as u8,
     1 as i32 as u8, 1 as i32 as u8,
     1 as i32 as u8, 1 as i32 as u8,
     1 as i32 as u8, 1 as i32 as u8,
     1 as i32 as u8, 1 as i32 as u8,
     2 as i32 as u8, 2 as i32 as u8,
     2 as i32 as u8, 2 as i32 as u8,
     2 as i32 as u8, 2 as i32 as u8,
     2 as i32 as u8, 2 as i32 as u8,
     3 as i32 as u8, 3 as i32 as u8,
     3 as i32 as u8, 3 as i32 as u8,
     4 as i32 as u8, 4 as i32 as u8,
     5 as i32 as u8, 6 as i32 as u8];
static mut right_flip: [u32; 7] =
    [0x1 as u32, 0x3 as u32, 0x7 as u32,
     0xf as u32, 0x1f as u32, 0x3f as u32,
     0x7f as u32];
static mut lsb_mask: [u32; 4] =
    [0xff as u32, 0xffff as u32, 0xffffff as u32,
     0xffffffff as u32];
static mut msb_mask: [u32; 4] =
    [0xff000000 as u32, 0xffff0000 as u32,
     0xffffff00 as u32, 0xffffffff as u32];
static mut pop_count: [u8; 64] =
    [0 as i32 as u8, 1 as i32 as u8,
     1 as i32 as u8, 2 as i32 as u8,
     1 as i32 as u8, 2 as i32 as u8,
     2 as i32 as u8, 3 as i32 as u8,
     1 as i32 as u8, 2 as i32 as u8,
     2 as i32 as u8, 3 as i32 as u8,
     2 as i32 as u8, 3 as i32 as u8,
     3 as i32 as u8, 4 as i32 as u8,
     1 as i32 as u8, 2 as i32 as u8,
     2 as i32 as u8, 3 as i32 as u8,
     2 as i32 as u8, 3 as i32 as u8,
     3 as i32 as u8, 4 as i32 as u8,
     2 as i32 as u8, 3 as i32 as u8,
     3 as i32 as u8, 4 as i32 as u8,
     3 as i32 as u8, 4 as i32 as u8,
     4 as i32 as u8, 5 as i32 as u8,
     1 as i32 as u8, 2 as i32 as u8,
     2 as i32 as u8, 3 as i32 as u8,
     2 as i32 as u8, 3 as i32 as u8,
     3 as i32 as u8, 4 as i32 as u8,
     2 as i32 as u8, 3 as i32 as u8,
     3 as i32 as u8, 4 as i32 as u8,
     3 as i32 as u8, 4 as i32 as u8,
     4 as i32 as u8, 5 as i32 as u8,
     2 as i32 as u8, 3 as i32 as u8,
     3 as i32 as u8, 4 as i32 as u8,
     3 as i32 as u8, 4 as i32 as u8,
     4 as i32 as u8, 5 as i32 as u8,
     3 as i32 as u8, 4 as i32 as u8,
     4 as i32 as u8, 5 as i32 as u8,
     4 as i32 as u8, 5 as i32 as u8,
     5 as i32 as u8, 6 as i32 as u8];
static mut c_frontier: [u8; 62] =
    [0 as i32 as u8, 0x1 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x10 as i32 as u8,
     0x11 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x1 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x20 as i32 as u8,
     0x21 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x1 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x10 as i32 as u8,
     0x11 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x1 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x40 as i32 as u8,
     0x41 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x1 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x10 as i32 as u8,
     0x11 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x1 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x20 as i32 as u8,
     0x21 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x1 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x10 as i32 as u8,
     0x11 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x1 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x80 as i32 as u8,
     0x81 as i32 as u8];
static mut d_frontier: [u8; 60] =
    [0 as i32 as u8, 0 as i32 as u8,
     0x2 as i32 as u8, 0x1 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x20 as i32 as u8,
     0x20 as i32 as u8,
     0x22 as i32 as u8,
     0x21 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x2 as i32 as u8,
     0x1 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x40 as i32 as u8,
     0x40 as i32 as u8,
     0x42 as i32 as u8,
     0x41 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x2 as i32 as u8,
     0x1 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x20 as i32 as u8,
     0x20 as i32 as u8,
     0x22 as i32 as u8,
     0x21 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x2 as i32 as u8,
     0x1 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x82 as i32 as u8,
     0x81 as i32 as u8];
static mut e_frontier: [u8; 56] =
    [0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x4 as i32 as u8, 0x4 as i32 as u8,
     0x2 as i32 as u8, 0x1 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x40 as i32 as u8,
     0x40 as i32 as u8,
     0x40 as i32 as u8,
     0x40 as i32 as u8,
     0x44 as i32 as u8,
     0x44 as i32 as u8,
     0x42 as i32 as u8,
     0x41 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x4 as i32 as u8,
     0x4 as i32 as u8, 0x2 as i32 as u8,
     0x1 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x84 as i32 as u8,
     0x84 as i32 as u8,
     0x82 as i32 as u8,
     0x81 as i32 as u8];
static mut f_flip: [u8; 160] =
    [0 as i32 as u8, 0xf as i32 as u8,
     0xe as i32 as u8, 0 as i32 as u8,
     0xc as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x8 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x7 as i32 as u8,
     0x6 as i32 as u8, 0 as i32 as u8,
     0x4 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x3 as i32 as u8,
     0x2 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x1 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x4 as i32 as u8, 0x5 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x8 as i32 as u8, 0xb as i32 as u8,
     0xa as i32 as u8, 0 as i32 as u8,
     0xc as i32 as u8, 0xd as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x8 as i32 as u8, 0x8 as i32 as u8,
     0x8 as i32 as u8, 0x8 as i32 as u8,
     0x4 as i32 as u8, 0x4 as i32 as u8,
     0x2 as i32 as u8, 0x1 as i32 as u8,
     0x10 as i32 as u8,
     0x17 as i32 as u8,
     0x16 as i32 as u8, 0 as i32 as u8,
     0x14 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x18 as i32 as u8,
     0x1b as i32 as u8,
     0x1a as i32 as u8, 0 as i32 as u8,
     0x1c as i32 as u8,
     0x1d as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x88 as i32 as u8,
     0x88 as i32 as u8,
     0x88 as i32 as u8,
     0x88 as i32 as u8,
     0x84 as i32 as u8,
     0x84 as i32 as u8,
     0x82 as i32 as u8,
     0x81 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0x20 as i32 as u8,
     0x2f as i32 as u8,
     0x2e as i32 as u8, 0 as i32 as u8,
     0x2c as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x28 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x30 as i32 as u8,
     0x37 as i32 as u8,
     0x36 as i32 as u8, 0 as i32 as u8,
     0x34 as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8, 0 as i32 as u8,
     0x38 as i32 as u8,
     0x3b as i32 as u8,
     0x3a as i32 as u8, 0 as i32 as u8,
     0x3c as i32 as u8,
     0x3d as i32 as u8, 0 as i32 as u8,
     0 as i32 as u8];
// //
unsafe fn TestFlips_bitboard_a1(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 0 as i32 + 1 as i32
                              & 0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 0 as i32 + 1 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 0 as i32 + 8 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x1010100 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    0 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    0 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    0 as i32 + 8 as i32 * 6 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3 as i32) as usize] &
                    0x1010101 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x1010100 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as i32 &
                    0x1010100 as u32;
            my_bits_low |= t | t >> 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     0 as i32 +
                                                         8 as i32 |
                                                     t >>
                                                         0 as i32 +
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 0 as i32 + 9 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x8040200 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    0 as i32 + 9 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    0 as i32 + 9 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    0 as i32 + 9 as i32 * 6 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3 as i32) as usize] &
                    0x80402010 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x8040200 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 9 as i32 &
                    0x8040200 as u32;
            my_bits_low |= t | t >> 9 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     0 as i32 +
                                                         9 as i32 |
                                                     t >>
                                                         0 as i32 +
                                                             9 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_low |= 0x1 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_h1(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 7 as i32 - 6 as i32 &
                             0x3f as i32 as u32) as usize] as
            i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 7 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 7 as i32 + 7 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x10204000 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    7 as i32 + 7 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    7 as i32 + 7 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    7 as i32 + 7 as i32 * 6 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3 as i32) as usize] &
                    0x1020408 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x10204000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 7 as i32 &
                    0x10204000 as u32;
            my_bits_low |= t | t >> 7 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     7 as i32 +
                                                         7 as i32 |
                                                     t >>
                                                         7 as i32 +
                                                             7 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 7 as i32 + 8 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x80808000 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    7 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    7 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    7 as i32 + 8 as i32 * 6 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3 as i32) as usize] &
                    0x80808080 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x80808000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as i32 &
                    0x80808000 as u32;
            my_bits_low |= t | t >> 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     7 as i32 +
                                                         8 as i32 |
                                                     t >>
                                                         7 as i32 +
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_low |= 0x80 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_a8(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >>
                              24 as i32 + 1 as i32 &
                              0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 24 as i32 + 1 as i32;
    // FIXME see if this is intended
    let negated = ((my_bits_high & fl) as i32).wrapping_neg();
    t =
        (negated >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 24 as i32 - 7 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x20408 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    24 as i32 + 32 as i32 -
                        7 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    24 as i32 + 32 as i32 -
                        7 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    24 as i32 + 32 as i32 -
                        7 as i32 * 6 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3 as i32) as usize] &
                    0x10204080 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x20408 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 7 as i32 &
                    0x20408 as u32;
            my_bits_high |= t | t << 7 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     24 as i32 -
                                                         7 as i32 |
                                                     t >>
                                                         24 as i32 -
                                                             7 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 24 as i32 - 8 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x10101 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    24 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    24 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    24 as i32 + 32 as i32 -
                        8 as i32 * 6 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3 as i32) as usize] &
                    0x1010101 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x10101 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as i32 &
                    0x10101 as u32;
            my_bits_high |= t | t << 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     24 as i32 -
                                                         8 as i32 |
                                                     t >>
                                                         24 as i32 -
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_high |= 0x1000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_h8(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 31 as i32 - 6 as i32
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 31 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 31 as i32 - 8 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x808080 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    31 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    31 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    31 as i32 + 32 as i32 -
                        8 as i32 * 6 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3 as i32) as usize] &
                    0x80808080 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x808080 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as i32 &
                    0x808080 as u32;
            my_bits_high |= t | t << 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     31 as i32 -
                                                         8 as i32 |
                                                     t >>
                                                         31 as i32 -
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 31 as i32 - 9 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x402010 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    31 as i32 + 32 as i32 -
                        9 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    31 as i32 + 32 as i32 -
                        9 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    31 as i32 + 32 as i32 -
                        9 as i32 * 6 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3 as i32) as usize] &
                    0x8040201 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x402010 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 9 as i32 &
                    0x402010 as u32;
            my_bits_high |= t | t << 9 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     31 as i32 -
                                                         9 as i32 |
                                                     t >>
                                                         31 as i32 -
                                                             9 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_high |= 0x80000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_b1(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 1 as i32 + 1 as i32
                              & 0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 1 as i32 + 1 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 1 as i32 + 8 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x2020200 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    1 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    1 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    1 as i32 + 8 as i32 * 6 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3 as i32) as usize] &
                    0x2020202 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x2020200 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as i32 &
                    0x2020200 as u32;
            my_bits_low |= t | t >> 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     1 as i32 +
                                                         8 as i32 |
                                                     t >>
                                                         1 as i32 +
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 1 as i32 + 9 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x10080400 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    1 as i32 + 9 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    1 as i32 + 9 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3 as i32) as usize] &
                    0x804020 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x10080400 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 9 as i32 &
                    0x10080400 as u32;
            my_bits_low |= t | t >> 9 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     1 as i32 +
                                                         9 as i32 |
                                                     t >>
                                                         1 as i32 +
                                                             9 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_low |= 0x2 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_g1(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 6 as i32 - 6 as i32 &
                             0x3e as i32 as u32) as usize] as
            i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 6 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 6 as i32 + 7 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x8102000 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    6 as i32 + 7 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    6 as i32 + 7 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3 as i32) as usize] &
                    0x10204 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x8102000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 7 as i32 &
                    0x8102000 as u32;
            my_bits_low |= t | t >> 7 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     6 as i32 +
                                                         7 as i32 |
                                                     t >>
                                                         6 as i32 +
                                                             7 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 6 as i32 + 8 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x40404000 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    6 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    6 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    6 as i32 + 8 as i32 * 6 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3 as i32) as usize] &
                    0x40404040 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x40404000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as i32 &
                    0x40404000 as u32;
            my_bits_low |= t | t >> 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     6 as i32 +
                                                         8 as i32 |
                                                     t >>
                                                         6 as i32 +
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_low |= 0x40 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_a2(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 8 as i32 + 1 as i32
                              & 0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 8 as i32 + 1 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 8 as i32 + 8 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    8 as i32 + 8 as i32 * 2 as i32) as
                   u32 != 0 {
            t =
                opp_bits_high >>
                    8 as i32 + 8 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    8 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    8 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x1010101 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) << 8 as i32 + 8 as i32
                         |
                         (1 as i32) <<
                             8 as i32 +
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    8 as i32 + 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 8 as i32 + 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 8 as i32 + 9 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    8 as i32 + 9 as i32 * 2 as i32) as
                   u32 != 0 {
            t =
                opp_bits_high >>
                    8 as i32 + 9 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    8 as i32 + 9 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    8 as i32 + 9 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x40201008 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) << 8 as i32 + 9 as i32
                         |
                         (1 as i32) <<
                             8 as i32 +
                                 9 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    8 as i32 + 9 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 8 as i32 + 9 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_low |= 0x100 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_h2(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 15 as i32 - 6 as i32
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 15 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 15 as i32 + 7 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    15 as i32 + 7 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    15 as i32 + 7 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    15 as i32 + 7 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    15 as i32 + 7 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x2040810 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) <<
                         15 as i32 + 7 as i32 |
                         (1 as i32) <<
                             15 as i32 +
                                 7 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    15 as i32 + 7 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 15 as i32 + 7 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 15 as i32 + 8 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    15 as i32 + 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    15 as i32 + 8 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    15 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    15 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x80808080 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) <<
                         15 as i32 + 8 as i32 |
                         (1 as i32) <<
                             15 as i32 +
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    15 as i32 + 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 15 as i32 + 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_low |= 0x8000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_a7(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >>
                              16 as i32 + 1 as i32 &
                              0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 16 as i32 + 1 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 16 as i32 - 7 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    16 as i32 - 7 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    16 as i32 + 32 as i32 -
                        7 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    16 as i32 + 32 as i32 -
                        7 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    16 as i32 + 32 as i32 -
                        7 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x8102040 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         16 as i32 - 7 as i32 |
                         (1 as i32) <<
                             16 as i32 -
                                 7 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    16 as i32 - 7 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 16 as i32 - 7 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 16 as i32 - 8 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    16 as i32 - 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    16 as i32 + 32 as i32 -
                        8 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    16 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    16 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x1010101 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         16 as i32 - 8 as i32 |
                         (1 as i32) <<
                             16 as i32 -
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    16 as i32 - 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 16 as i32 - 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_high |= 0x10000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_h7(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 23 as i32 - 6 as i32
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 23 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 23 as i32 - 8 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    23 as i32 - 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    23 as i32 + 32 as i32 -
                        8 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    23 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    23 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x80808080 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         23 as i32 - 8 as i32 |
                         (1 as i32) <<
                             23 as i32 -
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    23 as i32 - 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 23 as i32 - 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 23 as i32 - 9 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    23 as i32 - 9 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    23 as i32 + 32 as i32 -
                        9 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    23 as i32 + 32 as i32 -
                        9 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    23 as i32 + 32 as i32 -
                        9 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x10080402 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         23 as i32 - 9 as i32 |
                         (1 as i32) <<
                             23 as i32 -
                                 9 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    23 as i32 - 9 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 23 as i32 - 9 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_high |= 0x800000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_b8(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >>
                              25 as i32 + 1 as i32 &
                              0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 25 as i32 + 1 as i32;
    // FIXME find out if this was the correct assumption in the original code
    //  because there is UB in the original (negation can overflow there)
    let negated = ((my_bits_high & fl) as i32).wrapping_neg();
    t =
        (negated >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 25 as i32 - 7 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x40810 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    25 as i32 + 32 as i32 -
                        7 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    25 as i32 + 32 as i32 -
                        7 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3 as i32) as usize] &
                    0x20408000 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x40810 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 7 as i32 &
                    0x40810 as u32;
            my_bits_high |= t | t << 7 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     25 as i32 -
                                                         7 as i32 |
                                                     t >>
                                                         25 as i32 -
                                                             7 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 25 as i32 - 8 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x20202 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    25 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    25 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    25 as i32 + 32 as i32 -
                        8 as i32 * 6 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3 as i32) as usize] &
                    0x2020202 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x20202 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as i32 &
                    0x20202 as u32;
            my_bits_high |= t | t << 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     25 as i32 -
                                                         8 as i32 |
                                                     t >>
                                                         25 as i32 -
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_high |= 0x2000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_g8(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 30 as i32 - 6 as i32
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 30 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 30 as i32 - 8 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x404040 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    30 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    30 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    30 as i32 + 32 as i32 -
                        8 as i32 * 6 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3 as i32) as usize] &
                    0x40404040 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x404040 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as i32 &
                    0x404040 as u32;
            my_bits_high |= t | t << 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     30 as i32 -
                                                         8 as i32 |
                                                     t >>
                                                         30 as i32 -
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 30 as i32 - 9 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x201008 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    30 as i32 + 32 as i32 -
                        9 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    30 as i32 + 32 as i32 -
                        9 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3 as i32) as usize] &
                    0x4020100 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x201008 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 9 as i32 &
                    0x201008 as u32;
            my_bits_high |= t | t << 9 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     30 as i32 -
                                                         9 as i32 |
                                                     t >>
                                                         30 as i32 -
                                                             9 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_high |= 0x40000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_b2(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 9 as i32 + 1 as i32
                              & 0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 9 as i32 + 1 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 9 as i32 + 8 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    9 as i32 + 8 as i32 * 2 as i32) as
                   u32 != 0 {
            t =
                opp_bits_high >>
                    9 as i32 + 8 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    9 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    9 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x2020202 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) << 9 as i32 + 8 as i32
                         |
                         (1 as i32) <<
                             9 as i32 +
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    9 as i32 + 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 9 as i32 + 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 9 as i32 + 9 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    9 as i32 + 9 as i32 * 2 as i32) as
                   u32 != 0 {
            t =
                opp_bits_high >>
                    9 as i32 + 9 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    9 as i32 + 9 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    9 as i32 + 9 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x80402010 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) << 9 as i32 + 9 as i32
                         |
                         (1 as i32) <<
                             9 as i32 +
                                 9 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    9 as i32 + 9 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 9 as i32 + 9 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_low |= 0x200 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_g2(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 14 as i32 - 6 as i32
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 14 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 14 as i32 + 7 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    14 as i32 + 7 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    14 as i32 + 7 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    14 as i32 + 7 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    14 as i32 + 7 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x1020408 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) <<
                         14 as i32 + 7 as i32 |
                         (1 as i32) <<
                             14 as i32 +
                                 7 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    14 as i32 + 7 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 14 as i32 + 7 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 14 as i32 + 8 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    14 as i32 + 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    14 as i32 + 8 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    14 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    14 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x40404040 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) <<
                         14 as i32 + 8 as i32 |
                         (1 as i32) <<
                             14 as i32 +
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    14 as i32 + 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 14 as i32 + 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_low |= 0x4000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_b7(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >>
                              17 as i32 + 1 as i32 &
                              0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 17 as i32 + 1 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 17 as i32 - 7 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    17 as i32 - 7 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    17 as i32 + 32 as i32 -
                        7 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    17 as i32 + 32 as i32 -
                        7 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    17 as i32 + 32 as i32 -
                        7 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x10204080 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         17 as i32 - 7 as i32 |
                         (1 as i32) <<
                             17 as i32 -
                                 7 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    17 as i32 - 7 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 17 as i32 - 7 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 17 as i32 - 8 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    17 as i32 - 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    17 as i32 + 32 as i32 -
                        8 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    17 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    17 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x2020202 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         17 as i32 - 8 as i32 |
                         (1 as i32) <<
                             17 as i32 -
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    17 as i32 - 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 17 as i32 - 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_high |= 0x20000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_g7(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 22 as i32 - 6 as i32
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 22 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 22 as i32 - 8 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    22 as i32 - 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    22 as i32 + 32 as i32 -
                        8 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    22 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    22 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x40404040 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         22 as i32 - 8 as i32 |
                         (1 as i32) <<
                             22 as i32 -
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    22 as i32 - 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 22 as i32 - 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 22 as i32 - 9 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    22 as i32 - 9 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    22 as i32 + 32 as i32 -
                        9 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    22 as i32 + 32 as i32 -
                        9 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    22 as i32 + 32 as i32 -
                        9 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x8040201 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         22 as i32 - 9 as i32 |
                         (1 as i32) <<
                             22 as i32 -
                                 9 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    22 as i32 - 9 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 22 as i32 - 9 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_high |= 0x400000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_c1(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_low >> 2 as i32 - 1 as i32 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        2 as i32 -
                            2 as
                                i32).wrapping_add(28 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 2 as i32 - 1 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_low & my_bits_low >> 7 as i32 &
            ((1 as i32) << 2 as i32 + 7 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            2 as i32 +
                                                7 as i32) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 2 as i32 + 8 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x4040400 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    2 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    2 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    2 as i32 + 8 as i32 * 6 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3 as i32) as usize] &
                    0x4040404 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x4040400 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as i32 &
                    0x4040400 as u32;
            my_bits_low |= t | t >> 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     2 as i32 +
                                                         8 as i32 |
                                                     t >>
                                                         2 as i32 +
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 2 as i32 + 9 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x20100800 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    2 as i32 + 9 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    2 as i32 + 9 as i32 * 5 as i32 -
                        32 as i32 |
                    ((1 as i32) <<
                         2 as i32 +
                             9 as i32 * 4 as i32 -
                             32 as i32) as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x20100800 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 9 as i32 &
                    0x20100800 as u32;
            my_bits_low |= t | t >> 9 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     2 as i32 +
                                                         9 as i32 |
                                                     t >>
                                                         2 as i32 +
                                                             9 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_low |= 0x4 as i32 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_f1(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_low >> 5 as i32 - 4 as i32 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as i32 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_low >> 5 as i32 - 5 as i32) as
                   usize] as u32;
    my_bits_low |= fl << 5 as i32 - 4 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 5 as i32 + 7 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x4081000 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    5 as i32 + 7 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    5 as i32 + 7 as i32 * 5 as i32 -
                        32 as i32 |
                    ((1 as i32) <<
                         5 as i32 +
                             7 as i32 * 4 as i32 -
                             32 as i32) as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x4081000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 7 as i32 &
                    0x4081000 as u32;
            my_bits_low |= t | t >> 7 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     5 as i32 +
                                                         7 as i32 |
                                                     t >>
                                                         5 as i32 +
                                                             7 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 5 as i32 + 8 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x20202000 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    5 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    5 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    5 as i32 + 8 as i32 * 6 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3 as i32) as usize] &
                    0x20202020 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x20202000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as i32 &
                    0x20202000 as u32;
            my_bits_low |= t | t >> 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     5 as i32 +
                                                         8 as i32 |
                                                     t >>
                                                         5 as i32 +
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down right */
    t =
        opp_bits_low & my_bits_low >> 9 as i32 &
            ((1 as i32) << 5 as i32 + 9 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            5 as i32 +
                                                9 as i32) as
            i32 as i32;
    my_bits_low |= 0x20 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_a3(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 16 as i32 + 1 as i32
                              & 0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 16 as i32 + 1 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as i32 &
            ((1 as i32) << 16 as i32 - 7 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            16 as i32 -
                                                7 as i32) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 16 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                16 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                16 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                16 as i32 + 8 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x1010101 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 16 as i32 + 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as i32 &
            ((1 as i32) << 16 as i32 - 8 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            16 as i32 -
                                                8 as i32) as
            i32 as i32;
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 16 as i32 + 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                16 as i32 + 9 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                16 as i32 + 9 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                16 as i32 + 9 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x20100804 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 16 as i32 + 9 as i32)
                    as u32;
            flipped += contig
        }
    }
    my_bits_low |= 0x10000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_h3(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 23 as i32 - 6 as i32
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 23 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 23 as i32 + 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                23 as i32 + 7 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                23 as i32 + 7 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                23 as i32 + 7 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x4081020 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 23 as i32 + 7 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 23 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                23 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                23 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                23 as i32 + 8 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x80808080 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 23 as i32 + 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as i32 &
            ((1 as i32) << 23 as i32 - 8 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            23 as i32 -
                                                8 as i32) as
            i32 as i32;
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as i32 &
            ((1 as i32) << 23 as i32 - 9 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            23 as i32 -
                                                9 as i32) as
            i32 as i32;
    my_bits_low |= 0x800000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_a6(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >> 8 as i32 + 1 as i32
                              & 0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 8 as i32 + 1 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 8 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                8 as i32 + 32 as i32 -
                    7 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                8 as i32 + 32 as i32 -
                    7 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                8 as i32 + 32 as i32 -
                    7 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x4081020 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 8 as i32 - 7 as i32) as
                    u32;
            flipped += contig
        }
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as i32 &
            ((1 as i32) << 8 as i32 + 8 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            8 as i32 +
                                                8 as i32) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 8 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                8 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                8 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                8 as i32 + 32 as i32 -
                    8 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x1010101 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 8 as i32 - 8 as i32) as
                    u32;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as i32 &
            ((1 as i32) << 8 as i32 + 9 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            8 as i32 +
                                                9 as i32) as
            i32 as i32;
    my_bits_high |= 0x100 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_h6(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 15 as i32 - 6 as i32
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 15 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as i32 &
            ((1 as i32) << 15 as i32 + 7 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            15 as i32 +
                                                7 as i32) as
            i32 as i32;
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as i32 &
            ((1 as i32) << 15 as i32 + 8 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            15 as i32 +
                                                8 as i32) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 15 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                15 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                15 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                15 as i32 + 32 as i32 -
                    8 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x80808080 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 15 as i32 - 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 15 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                15 as i32 + 32 as i32 -
                    9 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                15 as i32 + 32 as i32 -
                    9 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                15 as i32 + 32 as i32 -
                    9 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x20100804 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 15 as i32 - 9 as i32)
                    as u32;
            flipped += contig
        }
    }
    my_bits_high |= 0x8000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_c8(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_high >> 26 as i32 - 1 as i32 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        26 as i32 -
                            2 as
                                i32).wrapping_add(28 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 26 as i32 - 1 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 26 as i32 - 7 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x81020 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    26 as i32 + 32 as i32 -
                        7 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    26 as i32 + 32 as i32 -
                        7 as i32 * 5 as i32 |
                    ((1 as i32) <<
                         26 as i32 + 32 as i32 -
                             7 as i32 * 4 as i32) as
                        u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x81020 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 7 as i32 &
                    0x81020 as u32;
            my_bits_high |= t | t << 7 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     26 as i32 -
                                                         7 as i32 |
                                                     t >>
                                                         26 as i32 -
                                                             7 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 26 as i32 - 8 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x40404 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    26 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    26 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    26 as i32 + 32 as i32 -
                        8 as i32 * 6 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3 as i32) as usize] &
                    0x4040404 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x40404 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as i32 &
                    0x40404 as u32;
            my_bits_high |= t | t << 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     26 as i32 -
                                                         8 as i32 |
                                                     t >>
                                                         26 as i32 -
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up left */
    t =
        opp_bits_high & my_bits_high << 9 as i32 &
            ((1 as i32) << 26 as i32 - 9 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            26 as i32 -
                                                9 as i32) as
            i32 as i32;
    my_bits_high |= 0x4000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_f8(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_high >> 29 as i32 - 4 as i32 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as i32 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_high >> 29 as i32 - 5 as i32) as
                   usize] as u32;
    my_bits_high |= fl << 29 as i32 - 4 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    t =
        opp_bits_high & my_bits_high << 7 as i32 &
            ((1 as i32) << 29 as i32 - 7 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            29 as i32 -
                                                7 as i32) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 29 as i32 - 8 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x202020 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    29 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    29 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    29 as i32 + 32 as i32 -
                        8 as i32 * 6 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3 as i32) as usize] &
                    0x20202020 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x202020 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as i32 &
                    0x202020 as u32;
            my_bits_high |= t | t << 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     29 as i32 -
                                                         8 as i32 |
                                                     t >>
                                                         29 as i32 -
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 29 as i32 - 9 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x100804 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    29 as i32 + 32 as i32 -
                        9 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    29 as i32 + 32 as i32 -
                        9 as i32 * 5 as i32 |
                    ((1 as i32) <<
                         29 as i32 + 32 as i32 -
                             9 as i32 * 4 as i32) as
                        u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x100804 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 9 as i32 &
                    0x100804 as u32;
            my_bits_high |= t | t << 9 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     29 as i32 -
                                                         9 as i32 |
                                                     t >>
                                                         29 as i32 -
                                                             9 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_high |= 0x20000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_d1(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_low >> 3 as i32 - 2 as i32 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        3 as i32 -
                            3 as
                                i32).wrapping_add(24 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 3 as i32 - 2 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 3 as i32 + 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low >> 7 as i32 &
                0x20400 as u32;
        my_bits_low |= t.wrapping_add(t >> 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 3 as i32 +
                                                     7 as i32 |
                                                 t >>
                                                     3 as i32 +
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 3 as i32 + 8 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x8080800 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    3 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    3 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    3 as i32 + 8 as i32 * 6 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3 as i32) as usize] &
                    0x8080808 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x8080800 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as i32 &
                    0x8080800 as u32;
            my_bits_low |= t | t >> 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     3 as i32 +
                                                         8 as i32 |
                                                     t >>
                                                         3 as i32 +
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 3 as i32 + 9 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x40201000 as u32 ==
               0 as i32 as u32 {
            t =
                ((my_bits_high <<
                      31 as i32 -
                          (3 as i32 +
                               9 as i32 * 4 as i32 -
                               32 as i32)) as i32 >>
                     31 as i32) as u32;
            my_bits_low |= 0x40201000 as u32 & t;
            flipped =
                (flipped as
                     u32).wrapping_add(3 as i32 as
                                                    u32 & t) as
                    i32 as i32
        } else {
            t =
                opp_bits_low & my_bits_low >> 9 as i32 &
                    0x40201000 as u32;
            my_bits_low |= t | t >> 9 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     3 as i32 +
                                                         9 as i32 |
                                                     t >>
                                                         3 as i32 +
                                                             9 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_low |= 0x8 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_e1(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_low >> 4 as i32 - 3 as i32 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        4 as i32 -
                            4 as
                                i32).wrapping_add(16 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 4 as i32 - 3 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 4 as i32 + 7 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x2040800 as u32 ==
               0 as i32 as u32 {
            t =
                ((my_bits_high <<
                      31 as i32 -
                          (4 as i32 +
                               7 as i32 * 4 as i32 -
                               32 as i32)) as i32 >>
                     31 as i32) as u32;
            my_bits_low |= 0x2040800 as u32 & t;
            flipped =
                (flipped as
                     u32).wrapping_add(3 as i32 as
                                                    u32 & t) as
                    i32 as i32
        } else {
            t =
                opp_bits_low & my_bits_low >> 7 as i32 &
                    0x2040800 as u32;
            my_bits_low |= t | t >> 7 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     4 as i32 +
                                                         7 as i32 |
                                                     t >>
                                                         4 as i32 +
                                                             7 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 4 as i32 + 8 as i32) as
               u32 != 0 {
        if !opp_bits_low & 0x10101000 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_high >>
                    4 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    4 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    4 as i32 + 8 as i32 * 6 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3 as i32) as usize] &
                    0x10101010 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x10101000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as i32 &
                    0x10101000 as u32;
            my_bits_low |= t | t >> 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     4 as i32 +
                                                         8 as i32 |
                                                     t >>
                                                         4 as i32 +
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 4 as i32 + 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low >> 9 as i32 &
                0x402000 as u32;
        my_bits_low |= t.wrapping_add(t >> 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 4 as i32 +
                                                     9 as i32 |
                                                 t >>
                                                     4 as i32 +
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x10 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_a4(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 24 as i32 + 1 as i32
                              & 0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 24 as i32 + 1 as i32;
    // FIXME verify that original behaviour is intended to be wrapping
    let negated = ((my_bits_low & fl) as i32).wrapping_neg();
    t =
        (negated >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_low &
           ((1 as i32) << 24 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 7 as i32 &
                0x20400 as u32;
        my_bits_low |= t.wrapping_add(t << 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 24 as i32 -
                                                     7 as i32 |
                                                 t >>
                                                     24 as i32 -
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1 as i32) <<
                24 as i32 + 8 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                24 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                24 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x1010101 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) << 24 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as i32 &
                0x10100 as u32;
        my_bits_low |= t.wrapping_add(t << 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 24 as i32 -
                                                     8 as i32 |
                                                 t >>
                                                     24 as i32 -
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1 as i32) <<
                24 as i32 + 9 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                24 as i32 + 9 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                24 as i32 + 9 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x10080402 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    my_bits_low |= 0x1000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_h4(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 31 as i32 - 6 as i32
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 31 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_high &
           ((1 as i32) <<
                31 as i32 + 7 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                31 as i32 + 7 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                31 as i32 + 7 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x8102040 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1 as i32) <<
                31 as i32 + 8 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                31 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                31 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x80808080 as u32;
        // fixme VERIFY overflow
        let negated = ((my_bits_high & fl) as i32).wrapping_neg();
        t =
            (negated >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) << 31 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as i32 &
                0x808000 as u32;
        my_bits_low |= t.wrapping_add(t << 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 31 as i32 -
                                                     8 as i32 |
                                                 t >>
                                                     31 as i32 -
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up left */
    if opp_bits_low &
           ((1 as i32) << 31 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 9 as i32 &
                0x402000 as u32;
        my_bits_low |= t.wrapping_add(t << 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 31 as i32 -
                                                     9 as i32 |
                                                 t >>
                                                     31 as i32 -
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x80000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_a5(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >> 0 as i32 + 1 as i32
                              & 0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 0 as i32 + 1 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_low &
           ((1 as i32) <<
                0 as i32 + 32 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                0 as i32 + 32 as i32 -
                    7 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                0 as i32 + 32 as i32 -
                    7 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x2040810 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1 as i32) << 0 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as i32 &
                0x10100 as u32;
        my_bits_high |= t.wrapping_add(t >> 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 0 as i32 +
                                                     8 as i32 |
                                                 t >>
                                                     0 as i32 +
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) <<
                0 as i32 + 32 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                0 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                0 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x1010101 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1 as i32) << 0 as i32 + 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 as i32 &
                0x40200 as u32;
        my_bits_high |= t.wrapping_add(t >> 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 0 as i32 +
                                                     9 as i32 |
                                                 t >>
                                                     0 as i32 +
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    my_bits_high |= 0x1 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_h5(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 7 as i32 - 6 as i32
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 7 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_high &
           ((1 as i32) << 7 as i32 + 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 as i32 &
                0x204000 as u32;
        my_bits_high |= t.wrapping_add(t >> 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 7 as i32 +
                                                     7 as i32 |
                                                 t >>
                                                     7 as i32 +
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1 as i32) << 7 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as i32 &
                0x808000 as u32;
        my_bits_high |= t.wrapping_add(t >> 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 7 as i32 +
                                                     8 as i32 |
                                                 t >>
                                                     7 as i32 +
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) <<
                7 as i32 + 32 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                7 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                7 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x80808080 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up left */
    if opp_bits_low &
           ((1 as i32) <<
                7 as i32 + 32 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                7 as i32 + 32 as i32 -
                    9 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                7 as i32 + 32 as i32 -
                    9 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x40201008 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    my_bits_high |= 0x80 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_d8(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_high >> 27 as i32 - 2 as i32 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        27 as i32 -
                            3 as
                                i32).wrapping_add(24 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 27 as i32 - 2 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 27 as i32 - 7 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x102040 as u32 ==
               0 as i32 as u32 {
            t =
                ((my_bits_low <<
                      31 as i32 -
                          (27 as i32 + 32 as i32 -
                               7 as i32 * 4 as i32)) as
                     i32 >> 31 as i32) as u32;
            my_bits_high |= 0x102040 as u32 & t;
            flipped =
                (flipped as
                     u32).wrapping_add(3 as i32 as
                                                    u32 & t) as
                    i32 as i32
        } else {
            t =
                opp_bits_high & my_bits_high << 7 as i32 &
                    0x102040 as u32;
            my_bits_high |= t | t << 7 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     27 as i32 -
                                                         7 as i32 |
                                                     t >>
                                                         27 as i32 -
                                                             7 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 27 as i32 - 8 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x80808 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    27 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    27 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    27 as i32 + 32 as i32 -
                        8 as i32 * 6 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3 as i32) as usize] &
                    0x8080808 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x80808 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as i32 &
                    0x80808 as u32;
            my_bits_high |= t | t << 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     27 as i32 -
                                                         8 as i32 |
                                                     t >>
                                                         27 as i32 -
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 27 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high << 9 as i32 &
                0x40200 as u32;
        my_bits_high |= t.wrapping_add(t << 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 27 as i32 -
                                                     9 as i32 |
                                                 t >>
                                                     27 as i32 -
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    my_bits_high |= 0x8000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_e8(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_high >> 28 as i32 - 3 as i32 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        28 as i32 -
                            4 as
                                i32).wrapping_add(16 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 28 as i32 - 3 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 28 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high << 7 as i32 &
                0x204000 as u32;
        my_bits_high |= t.wrapping_add(t << 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 28 as i32 -
                                                     7 as i32 |
                                                 t >>
                                                     28 as i32 -
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 28 as i32 - 8 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x101010 as u32 ==
               0 as i32 as u32 {
            t =
                opp_bits_low >>
                    28 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32 &
                    1 as i32 as u32;
            contig =
                (3 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    28 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    28 as i32 + 32 as i32 -
                        8 as i32 * 6 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3 as i32) as usize] &
                    0x10101010 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x101010 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as i32 &
                    0x101010 as u32;
            my_bits_high |= t | t << 8 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     28 as i32 -
                                                         8 as i32 |
                                                     t >>
                                                         28 as i32 -
                                                             8 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 28 as i32 - 9 as i32) as
               u32 != 0 {
        if !opp_bits_high & 0x80402 as u32 ==
               0 as i32 as u32 {
            t =
                ((my_bits_low <<
                      31 as i32 -
                          (28 as i32 + 32 as i32 -
                               9 as i32 * 4 as i32)) as
                     i32 >> 31 as i32) as u32;
            my_bits_high |= 0x80402 as u32 & t;
            flipped =
                (flipped as
                     u32).wrapping_add(3 as i32 as
                                                    u32 & t) as
                    i32 as i32
        } else {
            t =
                opp_bits_high & my_bits_high << 9 as i32 &
                    0x80402 as u32;
            my_bits_high |= t | t << 9 as i32;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     28 as i32 -
                                                         9 as i32 |
                                                     t >>
                                                         28 as i32 -
                                                             9 as i32
                                                                 *
                                                                 2 as
                                                                     i32
                                                             -
                                                             1 as i32)
                                                    &
                                                    3 as i32 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_high |= 0x10000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_c2(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_low >> 10 as i32 - 1 as i32 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        10 as i32 -
                            2 as
                                i32).wrapping_add(28 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 10 as i32 - 1 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_low & my_bits_low >> 7 as i32 &
            ((1 as i32) << 10 as i32 + 7 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            10 as i32 +
                                                7 as i32) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 10 as i32 + 8 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    10 as i32 + 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    10 as i32 + 8 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    10 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    10 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x4040404 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) <<
                         10 as i32 + 8 as i32 |
                         (1 as i32) <<
                             10 as i32 +
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    10 as i32 + 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 10 as i32 + 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 10 as i32 + 9 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    10 as i32 + 9 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    10 as i32 + 9 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    10 as i32 + 9 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x804020 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) <<
                         10 as i32 + 9 as i32 |
                         (1 as i32) <<
                             10 as i32 +
                                 9 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    10 as i32 + 9 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 10 as i32 + 9 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_low |= 0x400 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_f2(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_low >> 13 as i32 - 4 as i32 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as i32 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_low >> 13 as i32 - 5 as i32) as
                   usize] as u32;
    my_bits_low |= fl << 13 as i32 - 4 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 13 as i32 + 7 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    13 as i32 + 7 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    13 as i32 + 7 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    13 as i32 + 7 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x10204 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) <<
                         13 as i32 + 7 as i32 |
                         (1 as i32) <<
                             13 as i32 +
                                 7 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    13 as i32 + 7 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 13 as i32 + 7 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 13 as i32 + 8 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    13 as i32 + 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    13 as i32 + 8 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    13 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    13 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x20202020 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) <<
                         13 as i32 + 8 as i32 |
                         (1 as i32) <<
                             13 as i32 +
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    13 as i32 + 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 13 as i32 + 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down right */
    t =
        opp_bits_low & my_bits_low >> 9 as i32 &
            ((1 as i32) << 13 as i32 + 9 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            13 as i32 +
                                                9 as i32) as
            i32 as i32;
    my_bits_low |= 0x2000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_b3(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 17 as i32 + 1 as i32
                              & 0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 17 as i32 + 1 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as i32 &
            ((1 as i32) << 17 as i32 - 7 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            17 as i32 -
                                                7 as i32) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 17 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                17 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                17 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                17 as i32 + 8 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x2020202 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 17 as i32 + 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as i32 &
            ((1 as i32) << 17 as i32 - 8 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            17 as i32 -
                                                8 as i32) as
            i32 as i32;
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 17 as i32 + 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                17 as i32 + 9 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                17 as i32 + 9 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                17 as i32 + 9 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x40201008 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 17 as i32 + 9 as i32)
                    as u32;
            flipped += contig
        }
    }
    my_bits_low |= 0x20000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_g3(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 22 as i32 - 6 as i32
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 22 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 22 as i32 + 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                22 as i32 + 7 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                22 as i32 + 7 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                22 as i32 + 7 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x2040810 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 22 as i32 + 7 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 22 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                22 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                22 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                22 as i32 + 8 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x40404040 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 22 as i32 + 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as i32 &
            ((1 as i32) << 22 as i32 - 8 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            22 as i32 -
                                                8 as i32) as
            i32 as i32;
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as i32 &
            ((1 as i32) << 22 as i32 - 9 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            22 as i32 -
                                                9 as i32) as
            i32 as i32;
    my_bits_low |= 0x400000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_b6(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >> 9 as i32 + 1 as i32
                              & 0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 9 as i32 + 1 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 9 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                9 as i32 + 32 as i32 -
                    7 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                9 as i32 + 32 as i32 -
                    7 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                9 as i32 + 32 as i32 -
                    7 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x8102040 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 9 as i32 - 7 as i32) as
                    u32;
            flipped += contig
        }
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as i32 &
            ((1 as i32) << 9 as i32 + 8 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            9 as i32 +
                                                8 as i32) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 9 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                9 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                9 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                9 as i32 + 32 as i32 -
                    8 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x2020202 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 9 as i32 - 8 as i32) as
                    u32;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as i32 &
            ((1 as i32) << 9 as i32 + 9 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            9 as i32 +
                                                9 as i32) as
            i32 as i32;
    my_bits_high |= 0x200 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_g6(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 14 as i32 - 6 as i32
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 14 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as i32 &
            ((1 as i32) << 14 as i32 + 7 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            14 as i32 +
                                                7 as i32) as
            i32 as i32;
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as i32 &
            ((1 as i32) << 14 as i32 + 8 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            14 as i32 +
                                                8 as i32) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 14 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                14 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                14 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                14 as i32 + 32 as i32 -
                    8 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x40404040 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 14 as i32 - 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 14 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                14 as i32 + 32 as i32 -
                    9 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                14 as i32 + 32 as i32 -
                    9 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                14 as i32 + 32 as i32 -
                    9 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x10080402 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 14 as i32 - 9 as i32)
                    as u32;
            flipped += contig
        }
    }
    my_bits_high |= 0x4000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_c7(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_high >> 18 as i32 - 1 as i32 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        18 as i32 -
                            2 as
                                i32).wrapping_add(28 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 18 as i32 - 1 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 18 as i32 - 7 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    18 as i32 - 7 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    18 as i32 + 32 as i32 -
                        7 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    18 as i32 + 32 as i32 -
                        7 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x20408000 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         18 as i32 - 7 as i32 |
                         (1 as i32) <<
                             18 as i32 -
                                 7 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    18 as i32 - 7 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 18 as i32 - 7 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 18 as i32 - 8 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    18 as i32 - 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    18 as i32 + 32 as i32 -
                        8 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    18 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    18 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x4040404 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         18 as i32 - 8 as i32 |
                         (1 as i32) <<
                             18 as i32 -
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    18 as i32 - 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 18 as i32 - 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up left */
    t =
        opp_bits_high & my_bits_high << 9 as i32 &
            ((1 as i32) << 18 as i32 - 9 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            18 as i32 -
                                                9 as i32) as
            i32 as i32;
    my_bits_high |= 0x40000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_f7(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_high >> 21 as i32 - 4 as i32 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as i32 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_high >> 21 as i32 - 5 as i32) as
                   usize] as u32;
    my_bits_high |= fl << 21 as i32 - 4 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    t =
        opp_bits_high & my_bits_high << 7 as i32 &
            ((1 as i32) << 21 as i32 - 7 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            21 as i32 -
                                                7 as i32) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 21 as i32 - 8 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    21 as i32 - 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    21 as i32 + 32 as i32 -
                        8 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    21 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    21 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x20202020 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         21 as i32 - 8 as i32 |
                         (1 as i32) <<
                             21 as i32 -
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    21 as i32 - 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 21 as i32 - 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 21 as i32 - 9 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    21 as i32 - 9 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    21 as i32 + 32 as i32 -
                        9 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    21 as i32 + 32 as i32 -
                        9 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x4020100 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         21 as i32 - 9 as i32 |
                         (1 as i32) <<
                             21 as i32 -
                                 9 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    21 as i32 - 9 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 21 as i32 - 9 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_high |= 0x200000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_d2(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_low >> 11 as i32 - 2 as i32 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        11 as i32 -
                            3 as
                                i32).wrapping_add(24 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 11 as i32 - 2 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 11 as i32 + 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low &
                (my_bits_low >> 7 as i32 |
                     my_bits_high << 32 as i32 - 7 as i32) &
                0x2040000 as u32;
        my_bits_low |= t.wrapping_add(t >> 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 11 as i32 +
                                                     7 as i32 |
                                                 t >>
                                                     11 as i32 +
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 11 as i32 + 8 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    11 as i32 + 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    11 as i32 + 8 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    11 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    11 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x8080808 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) <<
                         11 as i32 + 8 as i32 |
                         (1 as i32) <<
                             11 as i32 +
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    11 as i32 + 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 11 as i32 + 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 11 as i32 + 9 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    11 as i32 + 9 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    11 as i32 + 9 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    11 as i32 + 9 as i32 * 4 as i32 -
                        32 as i32 |
                    ((1 as i32) <<
                         11 as i32 +
                             9 as i32 * 3 as i32 -
                             32 as i32) as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) <<
                         11 as i32 + 9 as i32 |
                         (1 as i32) <<
                             11 as i32 +
                                 9 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    11 as i32 + 9 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 11 as i32 + 9 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_low |= 0x800 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_e2(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_low >> 12 as i32 - 3 as i32 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        12 as i32 -
                            4 as
                                i32).wrapping_add(16 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 12 as i32 - 3 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 12 as i32 + 7 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    12 as i32 + 7 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    12 as i32 + 7 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    12 as i32 + 7 as i32 * 4 as i32 -
                        32 as i32 |
                    ((1 as i32) <<
                         12 as i32 +
                             7 as i32 * 3 as i32 -
                             32 as i32) as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) <<
                         12 as i32 + 7 as i32 |
                         (1 as i32) <<
                             12 as i32 +
                                 7 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    12 as i32 + 7 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 12 as i32 + 7 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 12 as i32 + 8 as i32) as
               u32 != 0 {
        if opp_bits_low &
               ((1 as i32) <<
                    12 as i32 + 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    12 as i32 + 8 as i32 * 3 as i32 -
                        32 as i32 & 1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    12 as i32 + 8 as i32 * 4 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    12 as i32 + 8 as i32 * 5 as i32 -
                        32 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2 as i32) as usize] &
                    0x10101010 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as i32) <<
                         12 as i32 + 8 as i32 |
                         (1 as i32) <<
                             12 as i32 +
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    12 as i32 + 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_low |= t << 12 as i32 + 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 12 as i32 + 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low &
                (my_bits_low >> 9 as i32 |
                     my_bits_high << 32 as i32 - 9 as i32) &
                0x40200000 as u32;
        my_bits_low |= t.wrapping_add(t >> 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 12 as i32 +
                                                     9 as i32 |
                                                 t >>
                                                     12 as i32 +
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x1000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_b4(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 25 as i32 + 1 as i32
                              & 0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 25 as i32 + 1 as i32;
    let negated = ((my_bits_low & fl) as i32).wrapping_neg();
    t =
        (negated >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_low &
           ((1 as i32) << 25 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 7 as i32 &
                0x40800 as u32;
        my_bits_low |= t.wrapping_add(t << 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 25 as i32 -
                                                     7 as i32 |
                                                 t >>
                                                     25 as i32 -
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1 as i32) <<
                25 as i32 + 8 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                25 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                25 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x2020202 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) << 25 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as i32 &
                0x20200 as u32;
        my_bits_low |= t.wrapping_add(t << 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 25 as i32 -
                                                     8 as i32 |
                                                 t >>
                                                     25 as i32 -
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1 as i32) <<
                25 as i32 + 9 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                25 as i32 + 9 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                25 as i32 + 9 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x20100804 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    my_bits_low |= 0x2000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_g4(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 30 as i32 - 6 as i32
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 30 as i32;
    t =
        (-((my_bits_low & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_high &
           ((1 as i32) <<
                30 as i32 + 7 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                30 as i32 + 7 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                30 as i32 + 7 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x4081020 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1 as i32) <<
                30 as i32 + 8 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                30 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                30 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x40404040 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) << 30 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as i32 &
                0x404000 as u32;
        my_bits_low |= t.wrapping_add(t << 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 30 as i32 -
                                                     8 as i32 |
                                                 t >>
                                                     30 as i32 -
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up left */
    if opp_bits_low &
           ((1 as i32) << 30 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 9 as i32 &
                0x201000 as u32;
        my_bits_low |= t.wrapping_add(t << 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 30 as i32 -
                                                     9 as i32 |
                                                 t >>
                                                     30 as i32 -
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x40000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_b5(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >> 1 as i32 + 1 as i32
                              & 0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 1 as i32 + 1 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_low &
           ((1 as i32) <<
                1 as i32 + 32 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                1 as i32 + 32 as i32 -
                    7 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                1 as i32 + 32 as i32 -
                    7 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x4081020 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1 as i32) << 1 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as i32 &
                0x20200 as u32;
        my_bits_high |= t.wrapping_add(t >> 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 1 as i32 +
                                                     8 as i32 |
                                                 t >>
                                                     1 as i32 +
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) <<
                1 as i32 + 32 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                1 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                1 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x2020202 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1 as i32) << 1 as i32 + 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 as i32 &
                0x80400 as u32;
        my_bits_high |= t.wrapping_add(t >> 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 1 as i32 +
                                                     9 as i32 |
                                                 t >>
                                                     1 as i32 +
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    my_bits_high |= 0x2 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_g5(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 6 as i32 - 6 as i32
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 as i32 - 6 as i32;
    t =
        (-((my_bits_high & fl) as i32) >> 31 as i32) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_high &
           ((1 as i32) << 6 as i32 + 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 as i32 &
                0x102000 as u32;
        my_bits_high |= t.wrapping_add(t >> 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 6 as i32 +
                                                     7 as i32 |
                                                 t >>
                                                     6 as i32 +
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1 as i32) << 6 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as i32 &
                0x404000 as u32;
        my_bits_high |= t.wrapping_add(t >> 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 6 as i32 +
                                                     8 as i32 |
                                                 t >>
                                                     6 as i32 +
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) <<
                6 as i32 + 32 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                6 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                6 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x40404040 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up left */
    if opp_bits_low &
           ((1 as i32) <<
                6 as i32 + 32 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                6 as i32 + 32 as i32 -
                    9 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                6 as i32 + 32 as i32 -
                    9 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x20100804 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    my_bits_high |= 0x40 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_d7(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_high >> 19 as i32 - 2 as i32 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        19 as i32 -
                            3 as
                                i32).wrapping_add(24 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 19 as i32 - 2 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 19 as i32 - 7 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    19 as i32 - 7 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    19 as i32 + 32 as i32 -
                        7 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    19 as i32 + 32 as i32 -
                        7 as i32 * 4 as i32 |
                    ((1 as i32) <<
                         19 as i32 + 32 as i32 -
                             7 as i32 * 3 as i32) as
                        u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         19 as i32 - 7 as i32 |
                         (1 as i32) <<
                             19 as i32 -
                                 7 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    19 as i32 - 7 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 19 as i32 - 7 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 19 as i32 - 8 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    19 as i32 - 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    19 as i32 + 32 as i32 -
                        8 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    19 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    19 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x8080808 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         19 as i32 - 8 as i32 |
                         (1 as i32) <<
                             19 as i32 -
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    19 as i32 - 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 19 as i32 - 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 19 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_high &
                (my_bits_high << 9 as i32 |
                     my_bits_low >> 32 as i32 - 9 as i32) &
                0x402 as u32;
        my_bits_high |= t.wrapping_add(t << 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 19 as i32 -
                                                     9 as i32 |
                                                 t >>
                                                     19 as i32 -
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    my_bits_high |= 0x80000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_e7(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_high >> 20 as i32 - 3 as i32 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        20 as i32 -
                            4 as
                                i32).wrapping_add(16 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 20 as i32 - 3 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 20 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_high &
                (my_bits_high << 7 as i32 |
                     my_bits_low >> 32 as i32 - 7 as i32) &
                0x2040 as u32;
        my_bits_high |= t.wrapping_add(t << 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 20 as i32 -
                                                     7 as i32 |
                                                 t >>
                                                     20 as i32 -
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 20 as i32 - 8 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    20 as i32 - 8 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    20 as i32 + 32 as i32 -
                        8 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    20 as i32 + 32 as i32 -
                        8 as i32 * 4 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    20 as i32 + 32 as i32 -
                        8 as i32 * 5 as i32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2 as i32) as usize] &
                    0x10101010 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         20 as i32 - 8 as i32 |
                         (1 as i32) <<
                             20 as i32 -
                                 8 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    20 as i32 - 8 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 20 as i32 - 8 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 20 as i32 - 9 as i32) as
               u32 != 0 {
        if opp_bits_high &
               ((1 as i32) <<
                    20 as i32 - 9 as i32 * 2 as i32)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    20 as i32 + 32 as i32 -
                        9 as i32 * 3 as i32 &
                    1 as i32 as u32;
            contig =
                (2 as i32 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    20 as i32 + 32 as i32 -
                        9 as i32 * 4 as i32 |
                    ((1 as i32) <<
                         20 as i32 + 32 as i32 -
                             9 as i32 * 3 as i32) as
                        u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as i32) <<
                         20 as i32 - 9 as i32 |
                         (1 as i32) <<
                             20 as i32 -
                                 9 as i32 * 2 as i32) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    20 as i32 - 9 as i32 * 2 as i32 &
                    1 as i32 as u32;
            my_bits_high |= t << 20 as i32 - 9 as i32;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_high |= 0x100000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_c3(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_low >> 18 as i32 - 1 as i32 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        18 as i32 -
                            2 as
                                i32).wrapping_add(28 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 18 as i32 - 1 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_low & my_bits_high << 32 as i32 - 7 as i32 &
            ((1 as i32) << 18 as i32 + 7 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            18 as i32 +
                                                7 as i32) as
            i32 as i32;
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as i32 &
            ((1 as i32) << 18 as i32 - 7 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            18 as i32 -
                                                7 as i32) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 18 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                18 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                18 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                18 as i32 + 8 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x4040404 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 18 as i32 + 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as i32 &
            ((1 as i32) << 18 as i32 - 8 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            18 as i32 -
                                                8 as i32) as
            i32 as i32;
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 18 as i32 + 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                18 as i32 + 9 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                18 as i32 + 9 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                18 as i32 + 9 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x80402010 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 18 as i32 + 9 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as i32 &
            ((1 as i32) << 18 as i32 - 9 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            18 as i32 -
                                                9 as i32) as
            i32 as i32;
    my_bits_low |= 0x40000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_f3(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_low >> 21 as i32 - 4 as i32 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as i32 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_low >> 21 as i32 - 5 as i32) as
                   usize] as u32;
    my_bits_low |= fl << 21 as i32 - 4 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 21 as i32 + 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                21 as i32 + 7 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                21 as i32 + 7 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                21 as i32 + 7 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x1020408 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 21 as i32 + 7 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as i32 &
            ((1 as i32) << 21 as i32 - 7 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            21 as i32 -
                                                7 as i32) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 21 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                21 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                21 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                21 as i32 + 8 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x20202020 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 21 as i32 + 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as i32 &
            ((1 as i32) << 21 as i32 - 8 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            21 as i32 -
                                                8 as i32) as
            i32 as i32;
    /* Down right */
    t =
        opp_bits_low & my_bits_high << 32 as i32 - 9 as i32 &
            ((1 as i32) << 21 as i32 + 9 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            21 as i32 +
                                                9 as i32) as
            i32 as i32;
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as i32 &
            ((1 as i32) << 21 as i32 - 9 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            21 as i32 -
                                                9 as i32) as
            i32 as i32;
    my_bits_low |= 0x200000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_c6(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_high >> 10 as i32 - 1 as i32 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        10 as i32 -
                            2 as
                                i32).wrapping_add(28 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 10 as i32 - 1 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as i32 &
            ((1 as i32) << 10 as i32 + 7 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            10 as i32 +
                                                7 as i32) as
            i32 as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 10 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                10 as i32 + 32 as i32 -
                    7 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                10 as i32 + 32 as i32 -
                    7 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                10 as i32 + 32 as i32 -
                    7 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x10204080 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 10 as i32 - 7 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as i32 &
            ((1 as i32) << 10 as i32 + 8 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            10 as i32 +
                                                8 as i32) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 10 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                10 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                10 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                10 as i32 + 32 as i32 -
                    8 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x4040404 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 10 as i32 - 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as i32 &
            ((1 as i32) << 10 as i32 + 9 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            10 as i32 +
                                                9 as i32) as
            i32 as i32;
    /* Up left */
    t =
        opp_bits_high & my_bits_low >> 32 as i32 - 9 as i32 &
            ((1 as i32) << 10 as i32 - 9 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            10 as i32 -
                                                9 as i32) as
            i32 as i32;
    my_bits_high |= 0x400 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_f6(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_high >> 13 as i32 - 4 as i32 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as i32 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_high >> 13 as i32 - 5 as i32) as
                   usize] as u32;
    my_bits_high |= fl << 13 as i32 - 4 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as i32 &
            ((1 as i32) << 13 as i32 + 7 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            13 as i32 +
                                                7 as i32) as
            i32 as i32;
    /* Up right */
    t =
        opp_bits_high & my_bits_low >> 32 as i32 - 7 as i32 &
            ((1 as i32) << 13 as i32 - 7 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            13 as i32 -
                                                7 as i32) as
            i32 as i32;
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as i32 &
            ((1 as i32) << 13 as i32 + 8 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            13 as i32 +
                                                8 as i32) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 13 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                13 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                13 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                13 as i32 + 32 as i32 -
                    8 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x20202020 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 13 as i32 - 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as i32 &
            ((1 as i32) << 13 as i32 + 9 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            13 as i32 +
                                                9 as i32) as
            i32 as i32;
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 13 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                13 as i32 + 32 as i32 -
                    9 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                13 as i32 + 32 as i32 -
                    9 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                13 as i32 + 32 as i32 -
                    9 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x8040201 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 13 as i32 - 9 as i32)
                    as u32;
            flipped += contig
        }
    }
    my_bits_high |= 0x2000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_d3(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_low >> 19 as i32 - 2 as i32 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        19 as i32 -
                            3 as
                                i32).wrapping_add(24 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 19 as i32 - 2 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 19 as i32 + 7 as i32) as
               u32 != 0 {
        fl =
            my_bits_high << 32 as i32 - 7 as i32 &
                ((1 as i32) << 19 as i32 + 7 as i32)
                    as u32;
        t =
            opp_bits_high & my_bits_high >> 7 as i32 &
                ((1 as i32) <<
                     19 as i32 + 7 as i32 * 2 as i32 -
                         32 as i32) as u32;
        my_bits_low |=
            fl.wrapping_add(t << 32 as i32 - 7 as i32);
        my_bits_high |= t;
        flipped =
            (flipped as
                 u32).wrapping_add(fl >>
                                                19 as i32 +
                                                    7 as i32 |
                                                t >>
                                                    19 as i32 +
                                                        7 as i32 *
                                                            2 as i32 -
                                                        32 as i32 -
                                                        1 as i32) as
                i32 as i32
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as i32 &
            ((1 as i32) << 19 as i32 - 7 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            19 as i32 -
                                                7 as i32) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 19 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                19 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                19 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                19 as i32 + 8 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x8080808 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 19 as i32 + 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as i32 &
            ((1 as i32) << 19 as i32 - 8 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            19 as i32 -
                                                8 as i32) as
            i32 as i32;
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 19 as i32 + 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                19 as i32 + 9 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                19 as i32 + 9 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x804020 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 19 as i32 + 9 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as i32 &
            ((1 as i32) << 19 as i32 - 9 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            19 as i32 -
                                                9 as i32) as
            i32 as i32;
    my_bits_low |= 0x80000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_e3(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_low >> 20 as i32 - 3 as i32 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        20 as i32 -
                            4 as
                                i32).wrapping_add(16 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 20 as i32 - 3 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1 as i32) << 20 as i32 + 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                20 as i32 + 7 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                20 as i32 + 7 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x10204 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 20 as i32 + 7 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as i32 &
            ((1 as i32) << 20 as i32 - 7 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            20 as i32 -
                                                7 as i32) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1 as i32) << 20 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                20 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                20 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                20 as i32 + 8 as i32 * 4 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1 as i32) as usize] &
                0x10101010 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as i32) << 20 as i32 + 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as i32 &
            ((1 as i32) << 20 as i32 - 8 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            20 as i32 -
                                                8 as i32) as
            i32 as i32;
    /* Down right */
    if opp_bits_low &
           ((1 as i32) << 20 as i32 + 9 as i32) as
               u32 != 0 {
        fl =
            my_bits_high << 32 as i32 - 9 as i32 &
                ((1 as i32) << 20 as i32 + 9 as i32)
                    as u32;
        t =
            opp_bits_high & my_bits_high >> 9 as i32 &
                ((1 as i32) <<
                     20 as i32 + 9 as i32 * 2 as i32 -
                         32 as i32) as u32;
        my_bits_low |=
            fl.wrapping_add(t << 32 as i32 - 9 as i32);
        my_bits_high |= t;
        flipped =
            (flipped as
                 u32).wrapping_add(fl >>
                                                20 as i32 +
                                                    9 as i32 |
                                                t >>
                                                    20 as i32 +
                                                        9 as i32 *
                                                            2 as i32 -
                                                        32 as i32 -
                                                        1 as i32) as
                i32 as i32
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as i32 &
            ((1 as i32) << 20 as i32 - 9 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            20 as i32 -
                                                9 as i32) as
            i32 as i32;
    my_bits_low |= 0x100000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_c4(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_low >> 26 as i32 - 1 as i32 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        26 as i32 -
                            2 as
                                i32).wrapping_add(28 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 26 as i32 - 1 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as i32 &
            ((1 as i32) <<
                 26 as i32 + 7 as i32 - 32 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            26 as i32 +
                                                7 as i32 -
                                                32 as i32) as
            i32 as i32;
    /* Up right */
    if opp_bits_low &
           ((1 as i32) << 26 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 7 as i32 &
                0x81000 as u32;
        my_bits_low |= t.wrapping_add(t << 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 26 as i32 -
                                                     7 as i32 |
                                                 t >>
                                                     26 as i32 -
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1 as i32) <<
                26 as i32 + 8 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                26 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                26 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x4040404 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) << 26 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as i32 &
                0x40400 as u32;
        my_bits_low |= t.wrapping_add(t << 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 26 as i32 -
                                                     8 as i32 |
                                                 t >>
                                                     26 as i32 -
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1 as i32) <<
                26 as i32 + 9 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                26 as i32 + 9 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                26 as i32 + 9 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x40201008 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as i32 &
            ((1 as i32) << 26 as i32 - 9 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            26 as i32 -
                                                9 as i32) as
            i32 as i32;
    my_bits_low |= 0x4000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_f4(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_low >> 29 as i32 - 4 as i32 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as i32 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_low >> 29 as i32 - 5 as i32) as
                   usize] as u32;
    my_bits_low |= fl << 29 as i32 - 4 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_high &
           ((1 as i32) <<
                29 as i32 + 7 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                29 as i32 + 7 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                29 as i32 + 7 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x2040810 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as i32 &
            ((1 as i32) << 29 as i32 - 7 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            29 as i32 -
                                                7 as i32) as
            i32 as i32;
    /* Down */
    if opp_bits_high &
           ((1 as i32) <<
                29 as i32 + 8 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                29 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                29 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x20202020 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) << 29 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as i32 &
                0x202000 as u32;
        my_bits_low |= t.wrapping_add(t << 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 29 as i32 -
                                                     8 as i32 |
                                                 t >>
                                                     29 as i32 -
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as i32 &
            ((1 as i32) <<
                 29 as i32 + 9 as i32 - 32 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            29 as i32 +
                                                9 as i32 -
                                                32 as i32) as
            i32 as i32;
    /* Up left */
    if opp_bits_low &
           ((1 as i32) << 29 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 9 as i32 &
                0x100800 as u32;
        my_bits_low |= t.wrapping_add(t << 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 29 as i32 -
                                                     9 as i32 |
                                                 t >>
                                                     29 as i32 -
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x20000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_c5(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_high >> 2 as i32 - 1 as i32 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        2 as i32 -
                            2 as
                                i32).wrapping_add(28 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 2 as i32 - 1 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as i32 &
            ((1 as i32) << 2 as i32 + 7 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            2 as i32 +
                                                7 as i32) as
            i32 as i32;
    /* Up right */
    if opp_bits_low &
           ((1 as i32) <<
                2 as i32 + 32 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                2 as i32 + 32 as i32 -
                    7 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                2 as i32 + 32 as i32 -
                    7 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x8102040 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1 as i32) << 2 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as i32 &
                0x40400 as u32;
        my_bits_high |= t.wrapping_add(t >> 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 2 as i32 +
                                                     8 as i32 |
                                                 t >>
                                                     2 as i32 +
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) <<
                2 as i32 + 32 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                2 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                2 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x4040404 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1 as i32) << 2 as i32 + 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 as i32 &
                0x100800 as u32;
        my_bits_high |= t.wrapping_add(t >> 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 2 as i32 +
                                                     9 as i32 |
                                                 t >>
                                                     2 as i32 +
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as i32 &
            ((1 as i32) <<
                 2 as i32 + 32 as i32 - 9 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            2 as i32 +
                                                32 as i32 -
                                                9 as i32) as
            i32 as i32;
    my_bits_high |= 0x4 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_f5(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_high >> 5 as i32 - 4 as i32 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as i32 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_high >> 5 as i32 - 5 as i32) as
                   usize] as u32;
    my_bits_high |= fl << 5 as i32 - 4 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_high &
           ((1 as i32) << 5 as i32 + 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 as i32 &
                0x81000 as u32;
        my_bits_high |= t.wrapping_add(t >> 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 5 as i32 +
                                                     7 as i32 |
                                                 t >>
                                                     5 as i32 +
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as i32 &
            ((1 as i32) <<
                 5 as i32 + 32 as i32 - 7 as i32) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            5 as i32 +
                                                32 as i32 -
                                                7 as i32) as
            i32 as i32;
    /* Down */
    if opp_bits_high &
           ((1 as i32) << 5 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as i32 &
                0x202000 as u32;
        my_bits_high |= t.wrapping_add(t >> 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 5 as i32 +
                                                     8 as i32 |
                                                 t >>
                                                     5 as i32 +
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) <<
                5 as i32 + 32 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                5 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                5 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x20202020 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as i32 &
            ((1 as i32) << 5 as i32 + 9 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            5 as i32 +
                                                9 as i32) as
            i32 as i32;
    /* Up left */
    if opp_bits_low &
           ((1 as i32) <<
                5 as i32 + 32 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                5 as i32 + 32 as i32 -
                    9 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                5 as i32 + 32 as i32 -
                    9 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x10080402 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    my_bits_high |= 0x20 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_d6(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_high >> 11 as i32 - 2 as i32 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        11 as i32 -
                            3 as
                                i32).wrapping_add(24 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 11 as i32 - 2 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as i32 &
            ((1 as i32) << 11 as i32 + 7 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            11 as i32 +
                                                7 as i32) as
            i32 as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 11 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                11 as i32 + 32 as i32 -
                    7 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                11 as i32 + 32 as i32 -
                    7 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x20408000 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 11 as i32 - 7 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as i32 &
            ((1 as i32) << 11 as i32 + 8 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            11 as i32 +
                                                8 as i32) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 11 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                11 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                11 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                11 as i32 + 32 as i32 -
                    8 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x8080808 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 11 as i32 - 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as i32 &
            ((1 as i32) << 11 as i32 + 9 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            11 as i32 +
                                                9 as i32) as
            i32 as i32;
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 11 as i32 - 9 as i32) as
               u32 != 0 {
        fl =
            my_bits_low >> 32 as i32 - 9 as i32 &
                ((1 as i32) << 11 as i32 - 9 as i32)
                    as u32;
        t =
            opp_bits_low & my_bits_low << 9 as i32 &
                ((1 as i32) <<
                     11 as i32 + 32 as i32 -
                         9 as i32 * 2 as i32) as u32;
        my_bits_high |=
            fl.wrapping_add(t >> 32 as i32 - 9 as i32);
        my_bits_low |= t;
        flipped =
            (flipped as
                 u32).wrapping_add(fl >>
                                                11 as i32 -
                                                    9 as i32 |
                                                t >>
                                                    11 as i32 +
                                                        32 as i32 -
                                                        9 as i32 *
                                                            2 as i32 -
                                                        1 as i32) as
                i32 as i32
    }
    my_bits_high |= 0x800 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_e6(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_high >> 12 as i32 - 3 as i32 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        12 as i32 -
                            4 as
                                i32).wrapping_add(16 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 12 as i32 - 3 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as i32 &
            ((1 as i32) << 12 as i32 + 7 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            12 as i32 +
                                                7 as i32) as
            i32 as i32;
    /* Up right */
    if opp_bits_high &
           ((1 as i32) << 12 as i32 - 7 as i32) as
               u32 != 0 {
        fl =
            my_bits_low >> 32 as i32 - 7 as i32 &
                ((1 as i32) << 12 as i32 - 7 as i32)
                    as u32;
        t =
            opp_bits_low & my_bits_low << 7 as i32 &
                ((1 as i32) <<
                     12 as i32 + 32 as i32 -
                         7 as i32 * 2 as i32) as u32;
        my_bits_high |=
            fl.wrapping_add(t >> 32 as i32 - 7 as i32);
        my_bits_low |= t;
        flipped =
            (flipped as
                 u32).wrapping_add(fl >>
                                                12 as i32 -
                                                    7 as i32 |
                                                t >>
                                                    12 as i32 +
                                                        32 as i32 -
                                                        7 as i32 *
                                                            2 as i32 -
                                                        1 as i32) as
                i32 as i32
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as i32 &
            ((1 as i32) << 12 as i32 + 8 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            12 as i32 +
                                                8 as i32) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1 as i32) << 12 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                12 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                12 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                12 as i32 + 32 as i32 -
                    8 as i32 * 4 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x10101010 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 12 as i32 - 8 as i32)
                    as u32;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as i32 &
            ((1 as i32) << 12 as i32 + 9 as i32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            12 as i32 +
                                                9 as i32) as
            i32 as i32;
    /* Up left */
    if opp_bits_high &
           ((1 as i32) << 12 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                12 as i32 + 32 as i32 -
                    9 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                12 as i32 + 32 as i32 -
                    9 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1 as i32) as usize] &
                0x4020100 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as i32) << 12 as i32 - 9 as i32)
                    as u32;
            flipped += contig
        }
    }
    my_bits_high |= 0x1000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_d4(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_low >> 27 as i32 - 2 as i32 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        27 as i32 -
                            3 as
                                i32).wrapping_add(24 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 27 as i32 - 2 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high &
            (opp_bits_high << 7 as i32 |
                 ((1 as i32) <<
                      27 as i32 + 7 as i32 -
                          32 as i32) as u32) &
            my_bits_high >> 7 as i32 & 0x204 as u32;
    my_bits_high |= t.wrapping_add(t >> 7 as i32);
    flipped =
        (flipped as
             u32).wrapping_add((t >>
                                             27 as i32 +
                                                 7 as i32 -
                                                 32 as i32 |
                                             t >>
                                                 27 as i32 +
                                                     7 as i32 *
                                                         2 as i32 -
                                                     32 as i32 -
                                                     1 as i32) &
                                            3 as i32 as u32)
            as i32 as i32;
    /* Up right */
    if opp_bits_low &
           ((1 as i32) << 27 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 7 as i32 &
                0x102000 as u32;
        my_bits_low |= t.wrapping_add(t << 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 27 as i32 -
                                                     7 as i32 |
                                                 t >>
                                                     27 as i32 -
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1 as i32) <<
                27 as i32 + 8 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                27 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                27 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x8080808 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) << 27 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as i32 &
                0x80800 as u32;
        my_bits_low |= t.wrapping_add(t << 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 27 as i32 -
                                                     8 as i32 |
                                                 t >>
                                                     27 as i32 -
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1 as i32) <<
                27 as i32 + 9 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                27 as i32 + 9 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                27 as i32 + 9 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x80402010 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up left */
    if opp_bits_low &
           ((1 as i32) << 27 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 9 as i32 &
                0x40200 as u32;
        my_bits_low |= t.wrapping_add(t << 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 27 as i32 -
                                                     9 as i32 |
                                                 t >>
                                                     27 as i32 -
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x8000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_e4(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_low >> 28 as i32 - 3 as i32 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        28 as i32 -
                            4 as
                                i32).wrapping_add(16 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 28 as i32 - 3 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_high &
           ((1 as i32) <<
                28 as i32 + 7 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                28 as i32 + 7 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                28 as i32 + 7 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x1020408 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up right */
    if opp_bits_low &
           ((1 as i32) << 28 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 7 as i32 &
                0x204000 as u32;
        my_bits_low |= t.wrapping_add(t << 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 28 as i32 -
                                                     7 as i32 |
                                                 t >>
                                                     28 as i32 -
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1 as i32) <<
                28 as i32 + 8 as i32 - 32 as i32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                28 as i32 + 8 as i32 * 2 as i32 -
                    32 as i32 & 1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                28 as i32 + 8 as i32 * 3 as i32 -
                    32 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x10101010 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) << 28 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as i32 &
                0x101000 as u32;
        my_bits_low |= t.wrapping_add(t << 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 28 as i32 -
                                                     8 as i32 |
                                                 t >>
                                                     28 as i32 -
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Down right */
    t =
        opp_bits_high &
            (opp_bits_high << 9 as i32 |
                 ((1 as i32) <<
                      28 as i32 + 9 as i32 -
                          32 as i32) as u32) &
            my_bits_high >> 9 as i32 & 0x4020 as u32;
    my_bits_high |= t.wrapping_add(t >> 9 as i32);
    flipped =
        (flipped as
             u32).wrapping_add((t >>
                                             28 as i32 +
                                                 9 as i32 -
                                                 32 as i32 |
                                             t >>
                                                 28 as i32 +
                                                     9 as i32 *
                                                         2 as i32 -
                                                     32 as i32 -
                                                     1 as i32) &
                                            3 as i32 as u32)
            as i32 as i32;
    /* Up left */
    if opp_bits_low &
           ((1 as i32) << 28 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 9 as i32 &
                0x80400 as u32;
        my_bits_low |= t.wrapping_add(t << 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 28 as i32 -
                                                     9 as i32 |
                                                 t >>
                                                     28 as i32 -
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x10000000 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_d5(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_high >> 3 as i32 - 2 as i32 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        3 as i32 -
                            3 as
                                i32).wrapping_add(24 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 3 as i32 - 2 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_high &
           ((1 as i32) << 3 as i32 + 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 as i32 &
                0x20400 as u32;
        my_bits_high |= t.wrapping_add(t >> 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 3 as i32 +
                                                     7 as i32 |
                                                 t >>
                                                     3 as i32 +
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up right */
    if opp_bits_low &
           ((1 as i32) <<
                3 as i32 + 32 as i32 - 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                3 as i32 + 32 as i32 -
                    7 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                3 as i32 + 32 as i32 -
                    7 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x10204080 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1 as i32) << 3 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as i32 &
                0x80800 as u32;
        my_bits_high |= t.wrapping_add(t >> 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 3 as i32 +
                                                     8 as i32 |
                                                 t >>
                                                     3 as i32 +
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) <<
                3 as i32 + 32 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                3 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                3 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x8080808 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1 as i32) << 3 as i32 + 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 as i32 &
                0x201000 as u32;
        my_bits_high |= t.wrapping_add(t >> 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 3 as i32 +
                                                     9 as i32 |
                                                 t >>
                                                     3 as i32 +
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up left */
    t =
        opp_bits_low &
            (opp_bits_low >> 9 as i32 |
                 ((1 as i32) <<
                      3 as i32 + 32 as i32 - 9 as i32)
                     as u32) & my_bits_low << 9 as i32 &
            0x4020000 as u32;
    my_bits_low |= t.wrapping_add(t << 9 as i32);
    flipped =
        (flipped as
             u32).wrapping_add((t >>
                                             3 as i32 +
                                                 32 as i32 -
                                                 9 as i32 |
                                             t >>
                                                 3 as i32 +
                                                     32 as i32 -
                                                     9 as i32 *
                                                         2 as i32 -
                                                     1 as i32) &
                                            3 as i32 as u32)
            as i32 as i32;
    my_bits_high |= 0x8 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe fn TestFlips_bitboard_e5(mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           mut opp_bits_high: u32,
                                           mut opp_bits_low: u32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_high >> 4 as i32 - 3 as i32 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        4 as i32 -
                            4 as
                                i32).wrapping_add(16 as i32 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 4 as i32 - 3 as i32;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_high &
           ((1 as i32) << 4 as i32 + 7 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 as i32 &
                0x40800 as u32;
        my_bits_high |= t.wrapping_add(t >> 7 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 4 as i32 +
                                                     7 as i32 |
                                                 t >>
                                                     4 as i32 +
                                                         7 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up right */
    t =
        opp_bits_low &
            (opp_bits_low >> 7 as i32 |
                 ((1 as i32) <<
                      4 as i32 + 32 as i32 - 7 as i32)
                     as u32) & my_bits_low << 7 as i32 &
            0x20400000 as u32;
    my_bits_low |= t.wrapping_add(t << 7 as i32);
    flipped =
        (flipped as
             u32).wrapping_add((t >>
                                             4 as i32 +
                                                 32 as i32 -
                                                 7 as i32 |
                                             t >>
                                                 4 as i32 +
                                                     32 as i32 -
                                                     7 as i32 *
                                                         2 as i32 -
                                                     1 as i32) &
                                            3 as i32 as u32)
            as i32 as i32;
    /* Down */
    if opp_bits_high &
           ((1 as i32) << 4 as i32 + 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as i32 &
                0x101000 as u32;
        my_bits_high |= t.wrapping_add(t >> 8 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 4 as i32 +
                                                     8 as i32 |
                                                 t >>
                                                     4 as i32 +
                                                         8 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1 as i32) <<
                4 as i32 + 32 as i32 - 8 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                4 as i32 + 32 as i32 -
                    8 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                4 as i32 + 32 as i32 -
                    8 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x10101010 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1 as i32) << 4 as i32 + 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 as i32 &
                0x402000 as u32;
        my_bits_high |= t.wrapping_add(t >> 9 as i32);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 4 as i32 +
                                                     9 as i32 |
                                                 t >>
                                                     4 as i32 +
                                                         9 as i32 *
                                                             2 as i32
                                                         - 1 as i32) &
                                                3 as i32 as
                                                    u32) as
                i32 as i32
    }
    /* Up left */
    if opp_bits_low &
           ((1 as i32) <<
                4 as i32 + 32 as i32 - 9 as i32) as
               u32 != 0 {
        t =
            opp_bits_low >>
                4 as i32 + 32 as i32 -
                    9 as i32 * 2 as i32 &
                1 as i32 as u32;
        contig =
            (1 as i32 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                4 as i32 + 32 as i32 -
                    9 as i32 * 3 as i32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x8040201 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31 as i32) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    my_bits_high |= 0x10 as u32;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}

pub static mut TestFlips_bitboard:
           [Option<unsafe fn(_: u32, _: u32,
                                        _: u32, _: u32)
                       -> i32>; 78] =
    unsafe {
        [Some(TestFlips_bitboard_a1 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_b1 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_c1 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_d1 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_e1 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_f1 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_g1 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_h1 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32), None, None,
         Some(TestFlips_bitboard_a2 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_b2 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_c2 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_d2 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_e2 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_f2 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_g2 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_h2 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32), None, None,
         Some(TestFlips_bitboard_a3 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_b3 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_c3 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_d3 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_e3 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_f3 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_g3 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_h3 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32), None, None,
         Some(TestFlips_bitboard_a4 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_b4 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_c4 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_d4 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_e4 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_f4 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_g4 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_h4 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32), None, None,
         Some(TestFlips_bitboard_a5 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_b5 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_c5 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_d5 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_e5 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_f5 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_g5 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_h5 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32), None, None,
         Some(TestFlips_bitboard_a6 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_b6 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_c6 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_d6 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_e6 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_f6 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_g6 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_h6 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32), None, None,
         Some(TestFlips_bitboard_a7 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_b7 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_c7 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_d7 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_e7 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_f7 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_g7 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_h7 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32), None, None,
         Some(TestFlips_bitboard_a8 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_b8 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_c8 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_d8 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_e8 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_f8 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_g8 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32),
         Some(TestFlips_bitboard_h8 as
                  unsafe fn(_: u32, _: u32,
                                       _: u32, _: u32)
                      -> i32)]
    };
