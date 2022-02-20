use crate::bitboard::BitBoard;
/*
   File:          bitbtest.c

   Modified:      November 24, 2005

   Authors:       Gunnar Andersson (gunnar@radagast.se)
              Toshihiko Okuhara

   Contents:      Count flips and returns new_my_bits in bb_flips.

   This piece of software is released under the GPL.
   See the file COPYING for more information.
*/

static right_contiguous: [u8; 64] =
    [0 as u8, 1 as u8,
     0 as u8, 2 as u8,
     0 as u8, 1 as u8,
     0 as u8, 3 as u8,
     0 as u8, 1 as u8,
     0 as u8, 2 as u8,
     0 as u8, 1 as u8,
     0 as u8, 4 as u8,
     0 as u8, 1 as u8,
     0 as u8, 2 as u8,
     0 as u8, 1 as u8,
     0 as u8, 3 as u8,
     0 as u8, 1 as u8,
     0 as u8, 2 as u8,
     0 as u8, 1 as u8,
     0 as u8, 5 as u8,
     0 as u8, 1 as u8,
     0 as u8, 2 as u8,
     0 as u8, 1 as u8,
     0 as u8, 3 as u8,
     0 as u8, 1 as u8,
     0 as u8, 2 as u8,
     0 as u8, 1 as u8,
     0 as u8, 4 as u8,
     0 as u8, 1 as u8,
     0 as u8, 2 as u8,
     0 as u8, 1 as u8,
     0 as u8, 3 as u8,
     0 as u8, 1 as u8,
     0 as u8, 2 as u8,
     0 as u8, 1 as u8,
     0 as u8, 6 as u8];
static left_contiguous: [u8; 64] =
    [0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     1 as u8, 1 as u8,
     1 as u8, 1 as u8,
     1 as u8, 1 as u8,
     1 as u8, 1 as u8,
     1 as u8, 1 as u8,
     1 as u8, 1 as u8,
     1 as u8, 1 as u8,
     1 as u8, 1 as u8,
     2 as u8, 2 as u8,
     2 as u8, 2 as u8,
     2 as u8, 2 as u8,
     2 as u8, 2 as u8,
     3 as u8, 3 as u8,
     3 as u8, 3 as u8,
     4 as u8, 4 as u8,
     5 as u8, 6 as u8];
static right_flip: [u32; 7] =
    [0x1 as u32, 0x3 as u32, 0x7 as u32,
     0xf as u32, 0x1f as u32, 0x3f as u32,
     0x7f as u32];
static lsb_mask: [u32; 4] =
    [0xff as u32, 0xffff as u32, 0xffffff as u32,
     0xffffffff as u32];
static  msb_mask: [u32; 4] =
    [0xff000000 as u32, 0xffff0000 as u32,
     0xffffff00 as u32, 0xffffffff as u32];
static  pop_count: [u8; 64] =
    [0 as u8, 1 as u8,
     1 as u8, 2 as u8,
     1 as u8, 2 as u8,
     2 as u8, 3 as u8,
     1 as u8, 2 as u8,
     2 as u8, 3 as u8,
     2 as u8, 3 as u8,
     3 as u8, 4 as u8,
     1 as u8, 2 as u8,
     2 as u8, 3 as u8,
     2 as u8, 3 as u8,
     3 as u8, 4 as u8,
     2 as u8, 3 as u8,
     3 as u8, 4 as u8,
     3 as u8, 4 as u8,
     4 as u8, 5 as u8,
     1 as u8, 2 as u8,
     2 as u8, 3 as u8,
     2 as u8, 3 as u8,
     3 as u8, 4 as u8,
     2 as u8, 3 as u8,
     3 as u8, 4 as u8,
     3 as u8, 4 as u8,
     4 as u8, 5 as u8,
     2 as u8, 3 as u8,
     3 as u8, 4 as u8,
     3 as u8, 4 as u8,
     4 as u8, 5 as u8,
     3 as u8, 4 as u8,
     4 as u8, 5 as u8,
     4 as u8, 5 as u8,
     5 as u8, 6 as u8];
static  c_frontier: [u8; 62] =
    [0 as u8, 0x1 as i32 as u8,
     0 as u8, 0 as u8,
     0x10 as i32 as u8,
     0x11 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x1 as i32 as u8, 0 as u8,
     0 as u8, 0x20 as i32 as u8,
     0x21 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x1 as i32 as u8, 0 as u8,
     0 as u8, 0x10 as i32 as u8,
     0x11 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x1 as i32 as u8, 0 as u8,
     0 as u8, 0x40 as i32 as u8,
     0x41 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x1 as i32 as u8, 0 as u8,
     0 as u8, 0x10 as i32 as u8,
     0x11 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x1 as i32 as u8, 0 as u8,
     0 as u8, 0x20 as i32 as u8,
     0x21 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x1 as i32 as u8, 0 as u8,
     0 as u8, 0x10 as i32 as u8,
     0x11 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x1 as i32 as u8, 0 as u8,
     0 as u8, 0x80 as i32 as u8,
     0x81 as i32 as u8];
static  d_frontier: [u8; 60] =
    [0 as u8, 0 as u8,
     0x2 as i32 as u8, 0x1 as i32 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x20 as i32 as u8,
     0x20 as i32 as u8,
     0x22 as i32 as u8,
     0x21 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0x2 as i32 as u8,
     0x1 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0x40 as i32 as u8,
     0x40 as i32 as u8,
     0x42 as i32 as u8,
     0x41 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0x2 as i32 as u8,
     0x1 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0x20 as i32 as u8,
     0x20 as i32 as u8,
     0x22 as i32 as u8,
     0x21 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0x2 as i32 as u8,
     0x1 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x82 as i32 as u8,
     0x81 as i32 as u8];
static  e_frontier: [u8; 56] =
    [0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x4 as i32 as u8, 0x4 as i32 as u8,
     0x2 as i32 as u8, 0x1 as i32 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x40 as i32 as u8,
     0x40 as i32 as u8,
     0x40 as i32 as u8,
     0x40 as i32 as u8,
     0x44 as i32 as u8,
     0x44 as i32 as u8,
     0x42 as i32 as u8,
     0x41 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0x4 as i32 as u8,
     0x4 as i32 as u8, 0x2 as i32 as u8,
     0x1 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x80 as i32 as u8,
     0x84 as i32 as u8,
     0x84 as i32 as u8,
     0x82 as i32 as u8,
     0x81 as i32 as u8];
static  f_flip: [u8; 160] =
    [0 as u8, 0xf as i32 as u8,
     0xe as i32 as u8, 0 as u8,
     0xc as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x8 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0x7 as i32 as u8,
     0x6 as i32 as u8, 0 as u8,
     0x4 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0x3 as i32 as u8,
     0x2 as i32 as u8, 0 as u8,
     0 as u8, 0x1 as i32 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x4 as i32 as u8, 0x5 as i32 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x8 as i32 as u8, 0xb as i32 as u8,
     0xa as i32 as u8, 0 as u8,
     0xc as i32 as u8, 0xd as i32 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x8 as i32 as u8, 0x8 as i32 as u8,
     0x8 as i32 as u8, 0x8 as i32 as u8,
     0x4 as i32 as u8, 0x4 as i32 as u8,
     0x2 as i32 as u8, 0x1 as i32 as u8,
     0x10 as i32 as u8,
     0x17 as i32 as u8,
     0x16 as i32 as u8, 0 as u8,
     0x14 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x18 as i32 as u8,
     0x1b as i32 as u8,
     0x1a as i32 as u8, 0 as u8,
     0x1c as i32 as u8,
     0x1d as i32 as u8, 0 as u8,
     0 as u8, 0x80 as i32 as u8,
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
     0x81 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0x20 as i32 as u8,
     0x2f as i32 as u8,
     0x2e as i32 as u8, 0 as u8,
     0x2c as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x28 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x30 as i32 as u8,
     0x37 as i32 as u8,
     0x36 as i32 as u8, 0 as u8,
     0x34 as i32 as u8, 0 as u8,
     0 as u8, 0 as u8,
     0x38 as i32 as u8,
     0x3b as i32 as u8,
     0x3a as i32 as u8, 0 as u8,
     0x3c as i32 as u8,
     0x3d as i32 as u8, 0 as u8,
     0 as u8];
// //
fn TestFlips_bitboard_a1( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 0 + 1
                              & 0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 0 + 1;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down */
    if opp_bits_low &
           ((1) << 0 + 8) as
               u32 != 0 {
        if !opp_bits_low & 0x1010100 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    0 + 8 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    0 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    0 + 8 * 6 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3) as usize] &
                    0x1010101 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x1010100 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 &
                    0x1010100 as u32;
            my_bits_low |= t | t >> 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     0 +
                                                         8 |
                                                     t >>
                                                         0 +
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1) << 0 + 9) as
               u32 != 0 {
        if !opp_bits_low & 0x8040200 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    0 + 9 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    0 + 9 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    0 + 9 * 6 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3) as usize] &
                    0x80402010 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x8040200 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 9 &
                    0x8040200 as u32;
            my_bits_low |= t | t >> 9;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     0 +
                                                         9 |
                                                     t >>
                                                         0 +
                                                             9
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_low |= 0x1 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_h1( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 7 - 6 &
                             0x3f as i32 as u32) as usize] as
            i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 7;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 7 + 7) as
               u32 != 0 {
        if !opp_bits_low & 0x10204000 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    7 + 7 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    7 + 7 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    7 + 7 * 6 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3) as usize] &
                    0x1020408 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x10204000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 7 &
                    0x10204000 as u32;
            my_bits_low |= t | t >> 7;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     7 +
                                                         7 |
                                                     t >>
                                                         7 +
                                                             7
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1) << 7 + 8) as
               u32 != 0 {
        if !opp_bits_low & 0x80808000 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    7 + 8 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    7 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    7 + 8 * 6 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3) as usize] &
                    0x80808080 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x80808000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 &
                    0x80808000 as u32;
            my_bits_low |= t | t >> 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     7 +
                                                         8 |
                                                     t >>
                                                         7 +
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_low |= 0x80 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_a8( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >>
                              24 + 1 &
                              0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 24 + 1;
    // FIXME see if this is intended
    let negated = ((my_bits_high & fl) as i32).wrapping_neg();
    t =
        (negated >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 24 - 7) as
               u32 != 0 {
        if !opp_bits_high & 0x20408 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    24 + 32 -
                        7 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    24 + 32 -
                        7 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    24 + 32 -
                        7 * 6;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3) as usize] &
                    0x10204080 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x20408 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 7 &
                    0x20408 as u32;
            my_bits_high |= t | t << 7;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     24 -
                                                         7 |
                                                     t >>
                                                         24 -
                                                             7
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1) << 24 - 8) as
               u32 != 0 {
        if !opp_bits_high & 0x10101 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    24 + 32 -
                        8 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    24 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    24 + 32 -
                        8 * 6;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3) as usize] &
                    0x1010101 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x10101 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 &
                    0x10101 as u32;
            my_bits_high |= t | t << 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     24 -
                                                         8 |
                                                     t >>
                                                         24 -
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_high |= 0x1000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_h8( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 31 - 6
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 31;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 31 - 8) as
               u32 != 0 {
        if !opp_bits_high & 0x808080 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    31 + 32 -
                        8 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    31 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    31 + 32 -
                        8 * 6;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3) as usize] &
                    0x80808080 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x808080 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 &
                    0x808080 as u32;
            my_bits_high |= t | t << 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     31 -
                                                         8 |
                                                     t >>
                                                         31 -
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1) << 31 - 9) as
               u32 != 0 {
        if !opp_bits_high & 0x402010 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    31 + 32 -
                        9 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    31 + 32 -
                        9 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    31 + 32 -
                        9 * 6;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3) as usize] &
                    0x8040201 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x402010 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 9 &
                    0x402010 as u32;
            my_bits_high |= t | t << 9;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     31 -
                                                         9 |
                                                     t >>
                                                         31 -
                                                             9
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_high |= 0x80000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_b1( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 1 + 1
                              & 0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 1 + 1;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down */
    if opp_bits_low &
           ((1) << 1 + 8) as
               u32 != 0 {
        if !opp_bits_low & 0x2020200 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    1 + 8 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    1 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    1 + 8 * 6 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3) as usize] &
                    0x2020202 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x2020200 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 &
                    0x2020200 as u32;
            my_bits_low |= t | t >> 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     1 +
                                                         8 |
                                                     t >>
                                                         1 +
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1) << 1 + 9) as
               u32 != 0 {
        if !opp_bits_low & 0x10080400 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    1 + 9 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    1 + 9 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3) as usize] &
                    0x804020 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x10080400 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 9 &
                    0x10080400 as u32;
            my_bits_low |= t | t >> 9;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     1 +
                                                         9 |
                                                     t >>
                                                         1 +
                                                             9
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_low |= 0x2 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_g1( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 6 - 6 &
                             0x3e as i32 as u32) as usize] as
            i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 6;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 6 + 7) as
               u32 != 0 {
        if !opp_bits_low & 0x8102000 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    6 + 7 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    6 + 7 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3) as usize] &
                    0x10204 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x8102000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 7 &
                    0x8102000 as u32;
            my_bits_low |= t | t >> 7;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     6 +
                                                         7 |
                                                     t >>
                                                         6 +
                                                             7
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1) << 6 + 8) as
               u32 != 0 {
        if !opp_bits_low & 0x40404000 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    6 + 8 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    6 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    6 + 8 * 6 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3) as usize] &
                    0x40404040 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x40404000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 &
                    0x40404000 as u32;
            my_bits_low |= t | t >> 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     6 +
                                                         8 |
                                                     t >>
                                                         6 +
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_low |= 0x40 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_a2( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 8 + 1
                              & 0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 8 + 1;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down */
    if opp_bits_low &
           ((1) << 8 + 8) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    8 + 8 * 2) as
                   u32 != 0 {
            t =
                opp_bits_high >>
                    8 + 8 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    8 + 8 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    8 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x1010101 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) << 8 + 8
                         |
                         (1) <<
                             8 +
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    8 + 8 * 2 &
                    1 as u32;
            my_bits_low |= t << 8 + 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1) << 8 + 9) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    8 + 9 * 2) as
                   u32 != 0 {
            t =
                opp_bits_high >>
                    8 + 9 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    8 + 9 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    8 + 9 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x40201008 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) << 8 + 9
                         |
                         (1) <<
                             8 +
                                 9 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    8 + 9 * 2 &
                    1 as u32;
            my_bits_low |= t << 8 + 9;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_low |= 0x100 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_h2( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 15 - 6
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 15;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 15 + 7) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    15 + 7 * 2)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    15 + 7 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    15 + 7 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    15 + 7 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x2040810 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) <<
                         15 + 7 |
                         (1) <<
                             15 +
                                 7 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    15 + 7 * 2 &
                    1 as u32;
            my_bits_low |= t << 15 + 7;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1) << 15 + 8) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    15 + 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    15 + 8 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    15 + 8 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    15 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x80808080 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) <<
                         15 + 8 |
                         (1) <<
                             15 +
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    15 + 8 * 2 &
                    1 as u32;
            my_bits_low |= t << 15 + 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_low |= 0x8000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_a7( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >>
                              16 + 1 &
                              0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 16 + 1;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 16 - 7) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    16 - 7 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    16 + 32 -
                        7 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    16 + 32 -
                        7 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    16 + 32 -
                        7 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x8102040 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         16 - 7 |
                         (1) <<
                             16 -
                                 7 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    16 - 7 * 2 &
                    1 as u32;
            my_bits_high |= t << 16 - 7;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1) << 16 - 8) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    16 - 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    16 + 32 -
                        8 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    16 + 32 -
                        8 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    16 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x1010101 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         16 - 8 |
                         (1) <<
                             16 -
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    16 - 8 * 2 &
                    1 as u32;
            my_bits_high |= t << 16 - 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_high |= 0x10000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_h7( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 23 - 6
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 23;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 23 - 8) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    23 - 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    23 + 32 -
                        8 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    23 + 32 -
                        8 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    23 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x80808080 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         23 - 8 |
                         (1) <<
                             23 -
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    23 - 8 * 2 &
                    1 as u32;
            my_bits_high |= t << 23 - 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1) << 23 - 9) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    23 - 9 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    23 + 32 -
                        9 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    23 + 32 -
                        9 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    23 + 32 -
                        9 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x10080402 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         23 - 9 |
                         (1) <<
                             23 -
                                 9 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    23 - 9 * 2 &
                    1 as u32;
            my_bits_high |= t << 23 - 9;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_high |= 0x800000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_b8( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >>
                              25 + 1 &
                              0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 25 + 1;
    // FIXME find out if this was the correct assumption in the original code
    //  because there is UB in the original (negation can overflow there)
    let negated = ((my_bits_high & fl) as i32).wrapping_neg();
    t =
        (negated >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 25 - 7) as
               u32 != 0 {
        if !opp_bits_high & 0x40810 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    25 + 32 -
                        7 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    25 + 32 -
                        7 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3) as usize] &
                    0x20408000 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x40810 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 7 &
                    0x40810 as u32;
            my_bits_high |= t | t << 7;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     25 -
                                                         7 |
                                                     t >>
                                                         25 -
                                                             7
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1) << 25 - 8) as
               u32 != 0 {
        if !opp_bits_high & 0x20202 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    25 + 32 -
                        8 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    25 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    25 + 32 -
                        8 * 6;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3) as usize] &
                    0x2020202 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x20202 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 &
                    0x20202 as u32;
            my_bits_high |= t | t << 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     25 -
                                                         8 |
                                                     t >>
                                                         25 -
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_high |= 0x2000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_g8( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 30 - 6
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 30;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 30 - 8) as
               u32 != 0 {
        if !opp_bits_high & 0x404040 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    30 + 32 -
                        8 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    30 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    30 + 32 -
                        8 * 6;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3) as usize] &
                    0x40404040 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x404040 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 &
                    0x404040 as u32;
            my_bits_high |= t | t << 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     30 -
                                                         8 |
                                                     t >>
                                                         30 -
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1) << 30 - 9) as
               u32 != 0 {
        if !opp_bits_high & 0x201008 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    30 + 32 -
                        9 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    30 + 32 -
                        9 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3) as usize] &
                    0x4020100 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x201008 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 9 &
                    0x201008 as u32;
            my_bits_high |= t | t << 9;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     30 -
                                                         9 |
                                                     t >>
                                                         30 -
                                                             9
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_high |= 0x40000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_b2( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 9 + 1
                              & 0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 9 + 1;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down */
    if opp_bits_low &
           ((1) << 9 + 8) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    9 + 8 * 2) as
                   u32 != 0 {
            t =
                opp_bits_high >>
                    9 + 8 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    9 + 8 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    9 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x2020202 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) << 9 + 8
                         |
                         (1) <<
                             9 +
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    9 + 8 * 2 &
                    1 as u32;
            my_bits_low |= t << 9 + 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1) << 9 + 9) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    9 + 9 * 2) as
                   u32 != 0 {
            t =
                opp_bits_high >>
                    9 + 9 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    9 + 9 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    9 + 9 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x80402010 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) << 9 + 9
                         |
                         (1) <<
                             9 +
                                 9 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    9 + 9 * 2 &
                    1 as u32;
            my_bits_low |= t << 9 + 9;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_low |= 0x200 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_g2( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 14 - 6
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 14;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 14 + 7) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    14 + 7 * 2)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    14 + 7 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    14 + 7 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    14 + 7 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x1020408 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) <<
                         14 + 7 |
                         (1) <<
                             14 +
                                 7 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    14 + 7 * 2 &
                    1 as u32;
            my_bits_low |= t << 14 + 7;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1) << 14 + 8) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    14 + 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    14 + 8 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    14 + 8 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    14 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x40404040 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) <<
                         14 + 8 |
                         (1) <<
                             14 +
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    14 + 8 * 2 &
                    1 as u32;
            my_bits_low |= t << 14 + 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_low |= 0x4000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_b7( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >>
                              17 + 1 &
                              0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 17 + 1;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 17 - 7) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    17 - 7 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    17 + 32 -
                        7 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    17 + 32 -
                        7 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    17 + 32 -
                        7 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x10204080 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         17 - 7 |
                         (1) <<
                             17 -
                                 7 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    17 - 7 * 2 &
                    1 as u32;
            my_bits_high |= t << 17 - 7;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1) << 17 - 8) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    17 - 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    17 + 32 -
                        8 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    17 + 32 -
                        8 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    17 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x2020202 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         17 - 8 |
                         (1) <<
                             17 -
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    17 - 8 * 2 &
                    1 as u32;
            my_bits_high |= t << 17 - 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_high |= 0x20000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_g7( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 22 - 6
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 22;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 22 - 8) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    22 - 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    22 + 32 -
                        8 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    22 + 32 -
                        8 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    22 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x40404040 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         22 - 8 |
                         (1) <<
                             22 -
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    22 - 8 * 2 &
                    1 as u32;
            my_bits_high |= t << 22 - 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1) << 22 - 9) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    22 - 9 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    22 + 32 -
                        9 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    22 + 32 -
                        9 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    22 + 32 -
                        9 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x8040201 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         22 - 9 |
                         (1) <<
                             22 -
                                 9 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    22 - 9 * 2 &
                    1 as u32;
            my_bits_high |= t << 22 - 9;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_high |= 0x400000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_c1( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_low >> 2 - 1 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        2 -
                            2).wrapping_add(28 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 2 - 1;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_low & my_bits_low >> 7 &
            ((1) << 2 + 7) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            2 +
                                                7) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1) << 2 + 8) as
               u32 != 0 {
        if !opp_bits_low & 0x4040400 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    2 + 8 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    2 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    2 + 8 * 6 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3) as usize] &
                    0x4040404 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x4040400 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 &
                    0x4040400 as u32;
            my_bits_low |= t | t >> 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     2 +
                                                         8 |
                                                     t >>
                                                         2 +
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1) << 2 + 9) as
               u32 != 0 {
        if !opp_bits_low & 0x20100800 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    2 + 9 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    2 + 9 * 5 -
                        32 |
                    ((1) <<
                         2 +
                             9 * 4 -
                             32) as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x20100800 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 9 &
                    0x20100800 as u32;
            my_bits_low |= t | t >> 9;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     2 +
                                                         9 |
                                                     t >>
                                                         2 +
                                                             9
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_low |= 0x4 as i32 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_f1( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_low >> 5 - 4 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_low >> 5 - 5) as
                   usize] as u32;
    my_bits_low |= fl << 5 - 4;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 5 + 7) as
               u32 != 0 {
        if !opp_bits_low & 0x4081000 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    5 + 7 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    5 + 7 * 5 -
                        32 |
                    ((1) <<
                         5 +
                             7 * 4 -
                             32) as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x4081000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 7 &
                    0x4081000 as u32;
            my_bits_low |= t | t >> 7;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     5 +
                                                         7 |
                                                     t >>
                                                         5 +
                                                             7
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1) << 5 + 8) as
               u32 != 0 {
        if !opp_bits_low & 0x20202000 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    5 + 8 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    5 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    5 + 8 * 6 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3) as usize] &
                    0x20202020 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x20202000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 &
                    0x20202000 as u32;
            my_bits_low |= t | t >> 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     5 +
                                                         8 |
                                                     t >>
                                                         5 +
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down right */
    t =
        opp_bits_low & my_bits_low >> 9 &
            ((1) << 5 + 9) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            5 +
                                                9) as
            i32 as i32;
    my_bits_low |= 0x20 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_a3( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 16 + 1
                              & 0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 16 + 1;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 &
            ((1) << 16 - 7) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            16 -
                                                7) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1) << 16 + 8) as
               u32 != 0 {
        t =
            opp_bits_high >>
                16 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                16 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                16 + 8 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x1010101 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 16 + 8)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 &
            ((1) << 16 - 8) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            16 -
                                                8) as
            i32 as i32;
    /* Down right */
    if opp_bits_low &
           ((1) << 16 + 9) as
               u32 != 0 {
        t =
            opp_bits_high >>
                16 + 9 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                16 + 9 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                16 + 9 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x20100804 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 16 + 9)
                    as u32;
            flipped += contig
        }
    }
    my_bits_low |= 0x10000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_h3( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 23 - 6
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 23;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 23 + 7) as
               u32 != 0 {
        t =
            opp_bits_high >>
                23 + 7 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                23 + 7 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                23 + 7 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x4081020 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 23 + 7)
                    as u32;
            flipped += contig
        }
    }
    /* Down */
    if opp_bits_low &
           ((1) << 23 + 8) as
               u32 != 0 {
        t =
            opp_bits_high >>
                23 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                23 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                23 + 8 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x80808080 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 23 + 8)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 &
            ((1) << 23 - 8) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            23 -
                                                8) as
            i32 as i32;
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 &
            ((1) << 23 - 9) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            23 -
                                                9) as
            i32 as i32;
    my_bits_low |= 0x800000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_a6( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >> 8 + 1
                              & 0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 8 + 1;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 8 - 7) as
               u32 != 0 {
        t =
            opp_bits_low >>
                8 + 32 -
                    7 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                8 + 32 -
                    7 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                8 + 32 -
                    7 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x4081020 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 8 - 7) as
                    u32;
            flipped += contig
        }
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 &
            ((1) << 8 + 8) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            8 +
                                                8) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 8 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                8 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                8 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                8 + 32 -
                    8 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x1010101 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 8 - 8) as
                    u32;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 &
            ((1) << 8 + 9) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            8 +
                                                9) as
            i32 as i32;
    my_bits_high |= 0x100 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_h6( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 15 - 6
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 15;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 &
            ((1) << 15 + 7) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            15 +
                                                7) as
            i32 as i32;
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 &
            ((1) << 15 + 8) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            15 +
                                                8) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 15 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                15 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                15 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                15 + 32 -
                    8 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x80808080 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 15 - 8)
                    as u32;
            flipped += contig
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1) << 15 - 9) as
               u32 != 0 {
        t =
            opp_bits_low >>
                15 + 32 -
                    9 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                15 + 32 -
                    9 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                15 + 32 -
                    9 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x20100804 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 15 - 9)
                    as u32;
            flipped += contig
        }
    }
    my_bits_high |= 0x8000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_c8( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_high >> 26 - 1 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        26 -
                            2).wrapping_add(28 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 26 - 1;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 26 - 7) as
               u32 != 0 {
        if !opp_bits_high & 0x81020 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    26 + 32 -
                        7 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    26 + 32 -
                        7 * 5 |
                    ((1) <<
                         26 + 32 -
                             7 * 4) as
                        u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x81020 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 7 &
                    0x81020 as u32;
            my_bits_high |= t | t << 7;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     26 -
                                                         7 |
                                                     t >>
                                                         26 -
                                                             7
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1) << 26 - 8) as
               u32 != 0 {
        if !opp_bits_high & 0x40404 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    26 + 32 -
                        8 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    26 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    26 + 32 -
                        8 * 6;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3) as usize] &
                    0x4040404 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x40404 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 &
                    0x40404 as u32;
            my_bits_high |= t | t << 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     26 -
                                                         8 |
                                                     t >>
                                                         26 -
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up left */
    t =
        opp_bits_high & my_bits_high << 9 &
            ((1) << 26 - 9) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            26 -
                                                9) as
            i32 as i32;
    my_bits_high |= 0x4000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_f8( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_high >> 29 - 4 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_high >> 29 - 5) as
                   usize] as u32;
    my_bits_high |= fl << 29 - 4;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    t =
        opp_bits_high & my_bits_high << 7 &
            ((1) << 29 - 7) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            29 -
                                                7) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 29 - 8) as
               u32 != 0 {
        if !opp_bits_high & 0x202020 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    29 + 32 -
                        8 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    29 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    29 + 32 -
                        8 * 6;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3) as usize] &
                    0x20202020 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x202020 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 &
                    0x202020 as u32;
            my_bits_high |= t | t << 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     29 -
                                                         8 |
                                                     t >>
                                                         29 -
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1) << 29 - 9) as
               u32 != 0 {
        if !opp_bits_high & 0x100804 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    29 + 32 -
                        9 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    29 + 32 -
                        9 * 5 |
                    ((1) <<
                         29 + 32 -
                             9 * 4) as
                        u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x100804 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 9 &
                    0x100804 as u32;
            my_bits_high |= t | t << 9;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     29 -
                                                         9 |
                                                     t >>
                                                         29 -
                                                             9
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_high |= 0x20000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_d1( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_low >> 3 - 2 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        3 -
                            3).wrapping_add(24 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 3 - 2;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 3 + 7) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low >> 7 &
                0x20400 as u32;
        my_bits_low |= t.wrapping_add(t >> 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 3 +
                                                     7 |
                                                 t >>
                                                     3 +
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_low &
           ((1) << 3 + 8) as
               u32 != 0 {
        if !opp_bits_low & 0x8080800 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    3 + 8 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    3 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    3 + 8 * 6 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3) as usize] &
                    0x8080808 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x8080800 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 &
                    0x8080800 as u32;
            my_bits_low |= t | t >> 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     3 +
                                                         8 |
                                                     t >>
                                                         3 +
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1) << 3 + 9) as
               u32 != 0 {
        if !opp_bits_low & 0x40201000 as u32 ==
               0 as u32 {
            t =
                ((my_bits_high <<
                      31 -
                          (3 +
                               9 * 4 -
                               32)) as i32 >>
                     31) as u32;
            my_bits_low |= 0x40201000 as u32 & t;
            flipped =
                (flipped as
                     u32).wrapping_add(3 as
                                                    u32 & t) as
                    i32 as i32
        } else {
            t =
                opp_bits_low & my_bits_low >> 9 &
                    0x40201000 as u32;
            my_bits_low |= t | t >> 9;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     3 +
                                                         9 |
                                                     t >>
                                                         3 +
                                                             9
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_low |= 0x8 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_e1( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_low >> 4 - 3 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        4 -
                            4).wrapping_add(16 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 4 - 3;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 4 + 7) as
               u32 != 0 {
        if !opp_bits_low & 0x2040800 as u32 ==
               0 as u32 {
            t =
                ((my_bits_high <<
                      31 -
                          (4 +
                               7 * 4 -
                               32)) as i32 >>
                     31) as u32;
            my_bits_low |= 0x2040800 as u32 & t;
            flipped =
                (flipped as
                     u32).wrapping_add(3 as
                                                    u32 & t) as
                    i32 as i32
        } else {
            t =
                opp_bits_low & my_bits_low >> 7 &
                    0x2040800 as u32;
            my_bits_low |= t | t >> 7;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     4 +
                                                         7 |
                                                     t >>
                                                         4 +
                                                             7
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1) << 4 + 8) as
               u32 != 0 {
        if !opp_bits_low & 0x10101000 as u32 ==
               0 as u32 {
            t =
                opp_bits_high >>
                    4 + 8 * 4 -
                        32 & 1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    4 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    4 + 8 * 6 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 3) as usize] &
                    0x10101010 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x10101000 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 &
                    0x10101000 as u32;
            my_bits_low |= t | t >> 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     4 +
                                                         8 |
                                                     t >>
                                                         4 +
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1) << 4 + 9) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low >> 9 &
                0x402000 as u32;
        my_bits_low |= t.wrapping_add(t >> 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 4 +
                                                     9 |
                                                 t >>
                                                     4 +
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x10 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_a4( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 24 + 1
                              & 0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 24 + 1;
    // FIXME verify that original behaviour is intended to be wrapping
    let negated = ((my_bits_low & fl) as i32).wrapping_neg();
    t =
        (negated >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_low &
           ((1) << 24 - 7) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 7 &
                0x20400 as u32;
        my_bits_low |= t.wrapping_add(t << 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 24 -
                                                     7 |
                                                 t >>
                                                     24 -
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1) <<
                24 + 8 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                24 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                24 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x1010101 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) << 24 - 8) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 &
                0x10100 as u32;
        my_bits_low |= t.wrapping_add(t << 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 24 -
                                                     8 |
                                                 t >>
                                                     24 -
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1) <<
                24 + 9 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                24 + 9 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                24 + 9 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x10080402 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    my_bits_low |= 0x1000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_h4( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 31 - 6
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 31;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_high &
           ((1) <<
                31 + 7 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                31 + 7 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                31 + 7 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x8102040 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1) <<
                31 + 8 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                31 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                31 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x80808080 as u32;
        // fixme VERIFY overflow
        let negated = ((my_bits_high & fl) as i32).wrapping_neg();
        t =
            (negated >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) << 31 - 8) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 &
                0x808000 as u32;
        my_bits_low |= t.wrapping_add(t << 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 31 -
                                                     8 |
                                                 t >>
                                                     31 -
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up left */
    if opp_bits_low &
           ((1) << 31 - 9) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 9 &
                0x402000 as u32;
        my_bits_low |= t.wrapping_add(t << 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 31 -
                                                     9 |
                                                 t >>
                                                     31 -
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x80000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_a5( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >> 0 + 1
                              & 0x3f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 0 + 1;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_low &
           ((1) <<
                0 + 32 - 7) as
               u32 != 0 {
        t =
            opp_bits_low >>
                0 + 32 -
                    7 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                0 + 32 -
                    7 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x2040810 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1) << 0 + 8) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 &
                0x10100 as u32;
        my_bits_high |= t.wrapping_add(t >> 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 0 +
                                                     8 |
                                                 t >>
                                                     0 +
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) <<
                0 + 32 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                0 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                0 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x1010101 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1) << 0 + 9) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 &
                0x40200 as u32;
        my_bits_high |= t.wrapping_add(t >> 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 0 +
                                                     9 |
                                                 t >>
                                                     0 +
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    my_bits_high |= 0x1 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_h5( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 7 - 6
                             & 0x3f as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 7;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_high &
           ((1) << 7 + 7) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 &
                0x204000 as u32;
        my_bits_high |= t.wrapping_add(t >> 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 7 +
                                                     7 |
                                                 t >>
                                                     7 +
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1) << 7 + 8) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 &
                0x808000 as u32;
        my_bits_high |= t.wrapping_add(t >> 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 7 +
                                                     8 |
                                                 t >>
                                                     7 +
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) <<
                7 + 32 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                7 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                7 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x80808080 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up left */
    if opp_bits_low &
           ((1) <<
                7 + 32 - 9) as
               u32 != 0 {
        t =
            opp_bits_low >>
                7 + 32 -
                    9 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                7 + 32 -
                    9 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x40201008 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    my_bits_high |= 0x80 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_d8( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_high >> 27 - 2 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        27 -
                            3).wrapping_add(24 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 27 - 2;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 27 - 7) as
               u32 != 0 {
        if !opp_bits_high & 0x102040 as u32 ==
               0 as u32 {
            t =
                ((my_bits_low <<
                      31 -
                          (27 + 32 -
                               7 * 4)) as
                     i32 >> 31) as u32;
            my_bits_high |= 0x102040 as u32 & t;
            flipped =
                (flipped as
                     u32).wrapping_add(3 as
                                                    u32 & t) as
                    i32 as i32
        } else {
            t =
                opp_bits_high & my_bits_high << 7 &
                    0x102040 as u32;
            my_bits_high |= t | t << 7;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     27 -
                                                         7 |
                                                     t >>
                                                         27 -
                                                             7
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1) << 27 - 8) as
               u32 != 0 {
        if !opp_bits_high & 0x80808 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    27 + 32 -
                        8 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    27 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    27 + 32 -
                        8 * 6;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3) as usize] &
                    0x8080808 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x80808 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 &
                    0x80808 as u32;
            my_bits_high |= t | t << 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     27 -
                                                         8 |
                                                     t >>
                                                         27 -
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1) << 27 - 9) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high << 9 &
                0x40200 as u32;
        my_bits_high |= t.wrapping_add(t << 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 27 -
                                                     9 |
                                                 t >>
                                                     27 -
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    my_bits_high |= 0x8000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_e8( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_high >> 28 - 3 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        28 -
                            4).wrapping_add(16 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 28 - 3;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 28 - 7) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high << 7 &
                0x204000 as u32;
        my_bits_high |= t.wrapping_add(t << 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 28 -
                                                     7 |
                                                 t >>
                                                     28 -
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_high &
           ((1) << 28 - 8) as
               u32 != 0 {
        if !opp_bits_high & 0x101010 as u32 ==
               0 as u32 {
            t =
                opp_bits_low >>
                    28 + 32 -
                        8 * 4 &
                    1 as u32;
            contig =
                (3 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    28 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    28 + 32 -
                        8 * 6;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 3) as usize] &
                    0x10101010 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x101010 as u32;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 &
                    0x101010 as u32;
            my_bits_high |= t | t << 8;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     28 -
                                                         8 |
                                                     t >>
                                                         28 -
                                                             8
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1) << 28 - 9) as
               u32 != 0 {
        if !opp_bits_high & 0x80402 as u32 ==
               0 as u32 {
            t =
                ((my_bits_low <<
                      31 -
                          (28 + 32 -
                               9 * 4)) as
                     i32 >> 31) as u32;
            my_bits_high |= 0x80402 as u32 & t;
            flipped =
                (flipped as
                     u32).wrapping_add(3 as
                                                    u32 & t) as
                    i32 as i32
        } else {
            t =
                opp_bits_high & my_bits_high << 9 &
                    0x80402 as u32;
            my_bits_high |= t | t << 9;
            flipped =
                (flipped as
                     u32).wrapping_add((t >>
                                                     28 -
                                                         9 |
                                                     t >>
                                                         28 -
                                                             9
                                                                 *
                                                                 2
                                                             -
                                                             1)
                                                    &
                                                    3 as
                                                        u32) as
                    i32 as i32
        }
    }
    my_bits_high |= 0x10000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_c2( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_low >> 10 - 1 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        10 -
                            2).wrapping_add(28 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 10 - 1;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_low & my_bits_low >> 7 &
            ((1) << 10 + 7) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            10 +
                                                7) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1) << 10 + 8) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    10 + 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    10 + 8 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    10 + 8 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    10 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x4040404 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) <<
                         10 + 8 |
                         (1) <<
                             10 +
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    10 + 8 * 2 &
                    1 as u32;
            my_bits_low |= t << 10 + 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1) << 10 + 9) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    10 + 9 * 2)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    10 + 9 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    10 + 9 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x804020 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) <<
                         10 + 9 |
                         (1) <<
                             10 +
                                 9 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    10 + 9 * 2 &
                    1 as u32;
            my_bits_low |= t << 10 + 9;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_low |= 0x400 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_f2( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_low >> 13 - 4 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_low >> 13 - 5) as
                   usize] as u32;
    my_bits_low |= fl << 13 - 4;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 13 + 7) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    13 + 7 * 2)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    13 + 7 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    13 + 7 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x10204 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) <<
                         13 + 7 |
                         (1) <<
                             13 +
                                 7 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    13 + 7 * 2 &
                    1 as u32;
            my_bits_low |= t << 13 + 7;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1) << 13 + 8) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    13 + 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    13 + 8 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    13 + 8 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    13 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x20202020 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) <<
                         13 + 8 |
                         (1) <<
                             13 +
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    13 + 8 * 2 &
                    1 as u32;
            my_bits_low |= t << 13 + 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down right */
    t =
        opp_bits_low & my_bits_low >> 9 &
            ((1) << 13 + 9) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            13 +
                                                9) as
            i32 as i32;
    my_bits_low |= 0x2000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_b3( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 17 + 1
                              & 0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 17 + 1;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 &
            ((1) << 17 - 7) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            17 -
                                                7) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1) << 17 + 8) as
               u32 != 0 {
        t =
            opp_bits_high >>
                17 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                17 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                17 + 8 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x2020202 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 17 + 8)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 &
            ((1) << 17 - 8) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            17 -
                                                8) as
            i32 as i32;
    /* Down right */
    if opp_bits_low &
           ((1) << 17 + 9) as
               u32 != 0 {
        t =
            opp_bits_high >>
                17 + 9 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                17 + 9 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                17 + 9 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x40201008 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 17 + 9)
                    as u32;
            flipped += contig
        }
    }
    my_bits_low |= 0x20000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_g3( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 22 - 6
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 22;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 22 + 7) as
               u32 != 0 {
        t =
            opp_bits_high >>
                22 + 7 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                22 + 7 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                22 + 7 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x2040810 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 22 + 7)
                    as u32;
            flipped += contig
        }
    }
    /* Down */
    if opp_bits_low &
           ((1) << 22 + 8) as
               u32 != 0 {
        t =
            opp_bits_high >>
                22 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                22 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                22 + 8 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x40404040 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 22 + 8)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 &
            ((1) << 22 - 8) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            22 -
                                                8) as
            i32 as i32;
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 &
            ((1) << 22 - 9) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            22 -
                                                9) as
            i32 as i32;
    my_bits_low |= 0x400000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_b6( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >> 9 + 1
                              & 0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 9 + 1;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 9 - 7) as
               u32 != 0 {
        t =
            opp_bits_low >>
                9 + 32 -
                    7 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                9 + 32 -
                    7 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                9 + 32 -
                    7 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x8102040 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 9 - 7) as
                    u32;
            flipped += contig
        }
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 &
            ((1) << 9 + 8) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            9 +
                                                8) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 9 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                9 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                9 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                9 + 32 -
                    8 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x2020202 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 9 - 8) as
                    u32;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 &
            ((1) << 9 + 9) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            9 +
                                                9) as
            i32 as i32;
    my_bits_high |= 0x200 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_g6( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 14 - 6
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 14;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 &
            ((1) << 14 + 7) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            14 +
                                                7) as
            i32 as i32;
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 &
            ((1) << 14 + 8) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            14 +
                                                8) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 14 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                14 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                14 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                14 + 32 -
                    8 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x40404040 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 14 - 8)
                    as u32;
            flipped += contig
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1) << 14 - 9) as
               u32 != 0 {
        t =
            opp_bits_low >>
                14 + 32 -
                    9 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                14 + 32 -
                    9 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                14 + 32 -
                    9 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x10080402 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 14 - 9)
                    as u32;
            flipped += contig
        }
    }
    my_bits_high |= 0x4000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_c7( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_high >> 18 - 1 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        18 -
                            2).wrapping_add(28 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 18 - 1;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 18 - 7) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    18 - 7 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    18 + 32 -
                        7 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    18 + 32 -
                        7 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x20408000 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         18 - 7 |
                         (1) <<
                             18 -
                                 7 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    18 - 7 * 2 &
                    1 as u32;
            my_bits_high |= t << 18 - 7;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1) << 18 - 8) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    18 - 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    18 + 32 -
                        8 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    18 + 32 -
                        8 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    18 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x4040404 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         18 - 8 |
                         (1) <<
                             18 -
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    18 - 8 * 2 &
                    1 as u32;
            my_bits_high |= t << 18 - 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up left */
    t =
        opp_bits_high & my_bits_high << 9 &
            ((1) << 18 - 9) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            18 -
                                                9) as
            i32 as i32;
    my_bits_high |= 0x40000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_f7( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_high >> 21 - 4 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_high >> 21 - 5) as
                   usize] as u32;
    my_bits_high |= fl << 21 - 4;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    t =
        opp_bits_high & my_bits_high << 7 &
            ((1) << 21 - 7) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            21 -
                                                7) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 21 - 8) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    21 - 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    21 + 32 -
                        8 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    21 + 32 -
                        8 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    21 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x20202020 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         21 - 8 |
                         (1) <<
                             21 -
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    21 - 8 * 2 &
                    1 as u32;
            my_bits_high |= t << 21 - 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1) << 21 - 9) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    21 - 9 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    21 + 32 -
                        9 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    21 + 32 -
                        9 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x4020100 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         21 - 9 |
                         (1) <<
                             21 -
                                 9 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    21 - 9 * 2 &
                    1 as u32;
            my_bits_high |= t << 21 - 9;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_high |= 0x200000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_d2( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_low >> 11 - 2 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        11 -
                            3).wrapping_add(24 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 11 - 2;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 11 + 7) as
               u32 != 0 {
        t =
            opp_bits_low &
                (my_bits_low >> 7 |
                     my_bits_high << 32 - 7) &
                0x2040000 as u32;
        my_bits_low |= t.wrapping_add(t >> 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 11 +
                                                     7 |
                                                 t >>
                                                     11 +
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_low &
           ((1) << 11 + 8) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    11 + 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    11 + 8 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    11 + 8 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    11 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x8080808 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) <<
                         11 + 8 |
                         (1) <<
                             11 +
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    11 + 8 * 2 &
                    1 as u32;
            my_bits_low |= t << 11 + 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1) << 11 + 9) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    11 + 9 * 2)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    11 + 9 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    11 + 9 * 4 -
                        32 |
                    ((1) <<
                         11 +
                             9 * 3 -
                             32) as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) <<
                         11 + 9 |
                         (1) <<
                             11 +
                                 9 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    11 + 9 * 2 &
                    1 as u32;
            my_bits_low |= t << 11 + 9;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_low |= 0x800 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_e2( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_low >> 12 - 3 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        12 -
                            4).wrapping_add(16 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 12 - 3;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 12 + 7) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    12 + 7 * 2)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    12 + 7 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    12 + 7 * 4 -
                        32 |
                    ((1) <<
                         12 +
                             7 * 3 -
                             32) as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) <<
                         12 + 7 |
                         (1) <<
                             12 +
                                 7 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    12 + 7 * 2 &
                    1 as u32;
            my_bits_low |= t << 12 + 7;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down */
    if opp_bits_low &
           ((1) << 12 + 8) as
               u32 != 0 {
        if opp_bits_low &
               ((1) <<
                    12 + 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_high >>
                    12 + 8 * 3 -
                        32 & 1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_high >>
                    12 + 8 * 4 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_high >>
                    12 + 8 * 5 -
                        32;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                lsb_mask[(contig - 2) as usize] &
                    0x10101010 as u32;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1) <<
                         12 + 8 |
                         (1) <<
                             12 +
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    12 + 8 * 2 &
                    1 as u32;
            my_bits_low |= t << 12 + 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1) << 12 + 9) as
               u32 != 0 {
        t =
            opp_bits_low &
                (my_bits_low >> 9 |
                     my_bits_high << 32 - 9) &
                0x40200000 as u32;
        my_bits_low |= t.wrapping_add(t >> 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 12 +
                                                     9 |
                                                 t >>
                                                     12 +
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x1000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_b4( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 25 + 1
                              & 0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 25 + 1;
    let negated = ((my_bits_low & fl) as i32).wrapping_neg();
    t =
        (negated >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_low &
           ((1) << 25 - 7) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 7 &
                0x40800 as u32;
        my_bits_low |= t.wrapping_add(t << 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 25 -
                                                     7 |
                                                 t >>
                                                     25 -
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1) <<
                25 + 8 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                25 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                25 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x2020202 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) << 25 - 8) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 &
                0x20200 as u32;
        my_bits_low |= t.wrapping_add(t << 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 25 -
                                                     8 |
                                                 t >>
                                                     25 -
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1) <<
                25 + 9 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                25 + 9 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                25 + 9 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x20100804 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    my_bits_low |= 0x2000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_g4( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 30 - 6
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 30;
    t =
        (-((my_bits_low & fl) as i32) >> 31) as
            u32;
    my_bits_low |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_high &
           ((1) <<
                30 + 7 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                30 + 7 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                30 + 7 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x4081020 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1) <<
                30 + 8 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                30 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                30 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x40404040 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) << 30 - 8) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 &
                0x404000 as u32;
        my_bits_low |= t.wrapping_add(t << 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 30 -
                                                     8 |
                                                 t >>
                                                     30 -
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up left */
    if opp_bits_low &
           ((1) << 30 - 9) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 9 &
                0x201000 as u32;
        my_bits_low |= t.wrapping_add(t << 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 30 -
                                                     9 |
                                                 t >>
                                                     30 -
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x40000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_b5( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >> 1 + 1
                              & 0x1f as i32 as u32) as usize]
            as i32;
    fl = right_flip[contig as usize] << 1 + 1;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Up right */
    if opp_bits_low &
           ((1) <<
                1 + 32 - 7) as
               u32 != 0 {
        t =
            opp_bits_low >>
                1 + 32 -
                    7 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                1 + 32 -
                    7 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x4081020 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1) << 1 + 8) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 &
                0x20200 as u32;
        my_bits_high |= t.wrapping_add(t >> 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 1 +
                                                     8 |
                                                 t >>
                                                     1 +
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) <<
                1 + 32 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                1 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                1 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x2020202 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1) << 1 + 9) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 &
                0x80400 as u32;
        my_bits_high |= t.wrapping_add(t >> 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 1 +
                                                     9 |
                                                 t >>
                                                     1 +
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    my_bits_high |= 0x2 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_g5( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 6 - 6
                             & 0x3e as i32 as u32) as usize]
            as i32;
    fl =
        (0x80000000 as u32 as i32 >> contig) as u32
            >> 32 - 6;
    t =
        (-((my_bits_high & fl) as i32) >> 31) as
            u32;
    my_bits_high |= fl & t;
    flipped = (contig as u32 & t) as i32;
    /* Down left */
    if opp_bits_high &
           ((1) << 6 + 7) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 &
                0x102000 as u32;
        my_bits_high |= t.wrapping_add(t >> 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 6 +
                                                     7 |
                                                 t >>
                                                     6 +
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1) << 6 + 8) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 &
                0x404000 as u32;
        my_bits_high |= t.wrapping_add(t >> 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 6 +
                                                     8 |
                                                 t >>
                                                     6 +
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) <<
                6 + 32 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                6 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                6 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x40404040 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up left */
    if opp_bits_low &
           ((1) <<
                6 + 32 - 9) as
               u32 != 0 {
        t =
            opp_bits_low >>
                6 + 32 -
                    9 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                6 + 32 -
                    9 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x20100804 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    my_bits_high |= 0x40 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_d7( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_high >> 19 - 2 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        19 -
                            3).wrapping_add(24 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 19 - 2;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 19 - 7) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    19 - 7 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    19 + 32 -
                        7 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    19 + 32 -
                        7 * 4 |
                    ((1) <<
                         19 + 32 -
                             7 * 3) as
                        u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         19 - 7 |
                         (1) <<
                             19 -
                                 7 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    19 - 7 * 2 &
                    1 as u32;
            my_bits_high |= t << 19 - 7;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up */
    if opp_bits_high &
           ((1) << 19 - 8) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    19 - 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    19 + 32 -
                        8 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    19 + 32 -
                        8 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    19 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x8080808 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         19 - 8 |
                         (1) <<
                             19 -
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    19 - 8 * 2 &
                    1 as u32;
            my_bits_high |= t << 19 - 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1) << 19 - 9) as
               u32 != 0 {
        t =
            opp_bits_high &
                (my_bits_high << 9 |
                     my_bits_low >> 32 - 9) &
                0x402 as u32;
        my_bits_high |= t.wrapping_add(t << 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 19 -
                                                     9 |
                                                 t >>
                                                     19 -
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    my_bits_high |= 0x80000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_e7( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_high >> 20 - 3 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        20 -
                            4).wrapping_add(16 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 20 - 3;
    flipped = pop_count[fl as usize] as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 20 - 7) as
               u32 != 0 {
        t =
            opp_bits_high &
                (my_bits_high << 7 |
                     my_bits_low >> 32 - 7) &
                0x2040 as u32;
        my_bits_high |= t.wrapping_add(t << 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 20 -
                                                     7 |
                                                 t >>
                                                     20 -
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_high &
           ((1) << 20 - 8) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    20 - 8 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    20 + 32 -
                        8 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t &=
                opp_bits_low >>
                    20 + 32 -
                        8 * 4;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t &=
                opp_bits_low >>
                    20 + 32 -
                        8 * 5;
            contig =
                (contig as u32).wrapping_add(t) as i32 as
                    i32;
            t =
                msb_mask[(contig - 2) as usize] &
                    0x10101010 as u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         20 - 8 |
                         (1) <<
                             20 -
                                 8 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    20 - 8 * 2 &
                    1 as u32;
            my_bits_high |= t << 20 - 8;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1) << 20 - 9) as
               u32 != 0 {
        if opp_bits_high &
               ((1) <<
                    20 - 9 * 2)
                   as u32 != 0 {
            t =
                opp_bits_low >>
                    20 + 32 -
                        9 * 3 &
                    1 as u32;
            contig =
                (2 as u32).wrapping_add(t) as
                    i32;
            t =
                t <<
                    20 + 32 -
                        9 * 4 |
                    ((1) <<
                         20 + 32 -
                             9 * 3) as
                        u32;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1) <<
                         20 - 9 |
                         (1) <<
                             20 -
                                 9 * 2) as
                        u32;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    20 - 9 * 2 &
                    1 as u32;
            my_bits_high |= t << 20 - 9;
            flipped =
                (flipped as u32).wrapping_add(t) as i32 as
                    i32
        }
    }
    my_bits_high |= 0x100000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_c3( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_low >> 18 - 1 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        18 -
                            2).wrapping_add(28 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 18 - 1;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_low & my_bits_high << 32 - 7 &
            ((1) << 18 + 7) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            18 +
                                                7) as
            i32 as i32;
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 &
            ((1) << 18 - 7) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            18 -
                                                7) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1) << 18 + 8) as
               u32 != 0 {
        t =
            opp_bits_high >>
                18 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                18 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                18 + 8 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x4040404 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 18 + 8)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 &
            ((1) << 18 - 8) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            18 -
                                                8) as
            i32 as i32;
    /* Down right */
    if opp_bits_low &
           ((1) << 18 + 9) as
               u32 != 0 {
        t =
            opp_bits_high >>
                18 + 9 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                18 + 9 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                18 + 9 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x80402010 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 18 + 9)
                    as u32;
            flipped += contig
        }
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 &
            ((1) << 18 - 9) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            18 -
                                                9) as
            i32 as i32;
    my_bits_low |= 0x40000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_f3( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_low >> 21 - 4 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_low >> 21 - 5) as
                   usize] as u32;
    my_bits_low |= fl << 21 - 4;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 21 + 7) as
               u32 != 0 {
        t =
            opp_bits_high >>
                21 + 7 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                21 + 7 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                21 + 7 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x1020408 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 21 + 7)
                    as u32;
            flipped += contig
        }
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 &
            ((1) << 21 - 7) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            21 -
                                                7) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1) << 21 + 8) as
               u32 != 0 {
        t =
            opp_bits_high >>
                21 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                21 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                21 + 8 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x20202020 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 21 + 8)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 &
            ((1) << 21 - 8) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            21 -
                                                8) as
            i32 as i32;
    /* Down right */
    t =
        opp_bits_low & my_bits_high << 32 - 9 &
            ((1) << 21 + 9) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            21 +
                                                9) as
            i32 as i32;
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 &
            ((1) << 21 - 9) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            21 -
                                                9) as
            i32 as i32;
    my_bits_low |= 0x200000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_c6( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_high >> 10 - 1 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        10 -
                            2).wrapping_add(28 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 10 - 1;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 &
            ((1) << 10 + 7) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            10 +
                                                7) as
            i32 as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 10 - 7) as
               u32 != 0 {
        t =
            opp_bits_low >>
                10 + 32 -
                    7 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                10 + 32 -
                    7 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                10 + 32 -
                    7 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x10204080 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 10 - 7)
                    as u32;
            flipped += contig
        }
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 &
            ((1) << 10 + 8) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            10 +
                                                8) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 10 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                10 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                10 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                10 + 32 -
                    8 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x4040404 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 10 - 8)
                    as u32;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 &
            ((1) << 10 + 9) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            10 +
                                                9) as
            i32 as i32;
    /* Up left */
    t =
        opp_bits_high & my_bits_low >> 32 - 9 &
            ((1) << 10 - 9) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            10 -
                                                9) as
            i32 as i32;
    my_bits_high |= 0x400 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_f6( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_high >> 13 - 4 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_high >> 13 - 5) as
                   usize] as u32;
    my_bits_high |= fl << 13 - 4;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 &
            ((1) << 13 + 7) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            13 +
                                                7) as
            i32 as i32;
    /* Up right */
    t =
        opp_bits_high & my_bits_low >> 32 - 7 &
            ((1) << 13 - 7) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            13 -
                                                7) as
            i32 as i32;
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 &
            ((1) << 13 + 8) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            13 +
                                                8) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 13 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                13 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                13 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                13 + 32 -
                    8 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x20202020 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 13 - 8)
                    as u32;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 &
            ((1) << 13 + 9) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            13 +
                                                9) as
            i32 as i32;
    /* Up left */
    if opp_bits_high &
           ((1) << 13 - 9) as
               u32 != 0 {
        t =
            opp_bits_low >>
                13 + 32 -
                    9 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                13 + 32 -
                    9 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                13 + 32 -
                    9 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x8040201 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 13 - 9)
                    as u32;
            flipped += contig
        }
    }
    my_bits_high |= 0x2000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_d3( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_low >> 19 - 2 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        19 -
                            3).wrapping_add(24 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 19 - 2;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 19 + 7) as
               u32 != 0 {
        fl =
            my_bits_high << 32 - 7 &
                ((1) << 19 + 7)
                    as u32;
        t =
            opp_bits_high & my_bits_high >> 7 &
                ((1) <<
                     19 + 7 * 2 -
                         32) as u32;
        my_bits_low |=
            fl.wrapping_add(t << 32 - 7);
        my_bits_high |= t;
        flipped =
            (flipped as
                 u32).wrapping_add(fl >>
                                                19 +
                                                    7 |
                                                t >>
                                                    19 +
                                                        7 *
                                                            2 -
                                                        32 -
                                                        1) as
                i32 as i32
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 &
            ((1) << 19 - 7) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            19 -
                                                7) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1) << 19 + 8) as
               u32 != 0 {
        t =
            opp_bits_high >>
                19 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                19 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                19 + 8 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x8080808 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 19 + 8)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 &
            ((1) << 19 - 8) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            19 -
                                                8) as
            i32 as i32;
    /* Down right */
    if opp_bits_low &
           ((1) << 19 + 9) as
               u32 != 0 {
        t =
            opp_bits_high >>
                19 + 9 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                19 + 9 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x804020 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 19 + 9)
                    as u32;
            flipped += contig
        }
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 &
            ((1) << 19 - 9) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            19 -
                                                9) as
            i32 as i32;
    my_bits_low |= 0x80000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_e3( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_low >> 20 - 3 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        20 -
                            4).wrapping_add(16 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 20 - 3;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_low &
           ((1) << 20 + 7) as
               u32 != 0 {
        t =
            opp_bits_high >>
                20 + 7 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                20 + 7 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x10204 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 20 + 7)
                    as u32;
            flipped += contig
        }
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 &
            ((1) << 20 - 7) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            20 -
                                                7) as
            i32 as i32;
    /* Down */
    if opp_bits_low &
           ((1) << 20 + 8) as
               u32 != 0 {
        t =
            opp_bits_high >>
                20 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                20 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_high >>
                20 + 8 * 4 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            lsb_mask[(contig - 1) as usize] &
                0x10101010 as u32;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1) << 20 + 8)
                    as u32;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 &
            ((1) << 20 - 8) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            20 -
                                                8) as
            i32 as i32;
    /* Down right */
    if opp_bits_low &
           ((1) << 20 + 9) as
               u32 != 0 {
        fl =
            my_bits_high << 32 - 9 &
                ((1) << 20 + 9)
                    as u32;
        t =
            opp_bits_high & my_bits_high >> 9 &
                ((1) <<
                     20 + 9 * 2 -
                         32) as u32;
        my_bits_low |=
            fl.wrapping_add(t << 32 - 9);
        my_bits_high |= t;
        flipped =
            (flipped as
                 u32).wrapping_add(fl >>
                                                20 +
                                                    9 |
                                                t >>
                                                    20 +
                                                        9 *
                                                            2 -
                                                        32 -
                                                        1) as
                i32 as i32
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 &
            ((1) << 20 - 9) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            20 -
                                                9) as
            i32 as i32;
    my_bits_low |= 0x100000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_c4( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_low >> 26 - 1 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        26 -
                            2).wrapping_add(28 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 26 - 1;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 &
            ((1) <<
                 26 + 7 - 32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            26 +
                                                7 -
                                                32) as
            i32 as i32;
    /* Up right */
    if opp_bits_low &
           ((1) << 26 - 7) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 7 &
                0x81000 as u32;
        my_bits_low |= t.wrapping_add(t << 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 26 -
                                                     7 |
                                                 t >>
                                                     26 -
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1) <<
                26 + 8 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                26 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                26 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x4040404 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) << 26 - 8) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 &
                0x40400 as u32;
        my_bits_low |= t.wrapping_add(t << 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 26 -
                                                     8 |
                                                 t >>
                                                     26 -
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1) <<
                26 + 9 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                26 + 9 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                26 + 9 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x40201008 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 &
            ((1) << 26 - 9) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            26 -
                                                9) as
            i32 as i32;
    my_bits_low |= 0x4000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_f4( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_low >> 29 - 4 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_low >> 29 - 5) as
                   usize] as u32;
    my_bits_low |= fl << 29 - 4;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_high &
           ((1) <<
                29 + 7 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                29 + 7 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                29 + 7 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x2040810 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 &
            ((1) << 29 - 7) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            29 -
                                                7) as
            i32 as i32;
    /* Down */
    if opp_bits_high &
           ((1) <<
                29 + 8 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                29 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                29 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x20202020 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) << 29 - 8) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 &
                0x202000 as u32;
        my_bits_low |= t.wrapping_add(t << 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 29 -
                                                     8 |
                                                 t >>
                                                     29 -
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 &
            ((1) <<
                 29 + 9 - 32) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            29 +
                                                9 -
                                                32) as
            i32 as i32;
    /* Up left */
    if opp_bits_low &
           ((1) << 29 - 9) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 9 &
                0x100800 as u32;
        my_bits_low |= t.wrapping_add(t << 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 29 -
                                                     9 |
                                                 t >>
                                                     29 -
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x20000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_c5( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_high >> 2 - 1 &
                        0x3d as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        2 -
                            2).wrapping_add(28 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 2 - 1;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 &
            ((1) << 2 + 7) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            2 +
                                                7) as
            i32 as i32;
    /* Up right */
    if opp_bits_low &
           ((1) <<
                2 + 32 - 7) as
               u32 != 0 {
        t =
            opp_bits_low >>
                2 + 32 -
                    7 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                2 + 32 -
                    7 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x8102040 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1) << 2 + 8) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 &
                0x40400 as u32;
        my_bits_high |= t.wrapping_add(t >> 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 2 +
                                                     8 |
                                                 t >>
                                                     2 +
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) <<
                2 + 32 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                2 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                2 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x4040404 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1) << 2 + 9) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 &
                0x100800 as u32;
        my_bits_high |= t.wrapping_add(t >> 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 2 +
                                                     9 |
                                                 t >>
                                                     2 +
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 &
            ((1) <<
                 2 + 32 - 9) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            2 +
                                                32 -
                                                9) as
            i32 as i32;
    my_bits_high |= 0x4 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_f5( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_high >> 5 - 4 &
                    0x2f as i32 as
                        u32).wrapping_add(64 as
                                                       u32) as usize]
            as u32;
    fl =
        f_flip[(t & my_bits_high >> 5 - 5) as
                   usize] as u32;
    my_bits_high |= fl << 5 - 4;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_high &
           ((1) << 5 + 7) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 &
                0x81000 as u32;
        my_bits_high |= t.wrapping_add(t >> 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 5 +
                                                     7 |
                                                 t >>
                                                     5 +
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 &
            ((1) <<
                 5 + 32 - 7) as
                u32;
    my_bits_low |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            5 +
                                                32 -
                                                7) as
            i32 as i32;
    /* Down */
    if opp_bits_high &
           ((1) << 5 + 8) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 &
                0x202000 as u32;
        my_bits_high |= t.wrapping_add(t >> 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 5 +
                                                     8 |
                                                 t >>
                                                     5 +
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) <<
                5 + 32 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                5 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                5 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x20202020 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 &
            ((1) << 5 + 9) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            5 +
                                                9) as
            i32 as i32;
    /* Up left */
    if opp_bits_low &
           ((1) <<
                5 + 32 - 9) as
               u32 != 0 {
        t =
            opp_bits_low >>
                5 + 32 -
                    9 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                5 + 32 -
                    9 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x10080402 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    my_bits_high |= 0x20 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_d6( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_high >> 11 - 2 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        11 -
                            3).wrapping_add(24 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 11 - 2;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 &
            ((1) << 11 + 7) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            11 +
                                                7) as
            i32 as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 11 - 7) as
               u32 != 0 {
        t =
            opp_bits_low >>
                11 + 32 -
                    7 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                11 + 32 -
                    7 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x20408000 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 11 - 7)
                    as u32;
            flipped += contig
        }
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 &
            ((1) << 11 + 8) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            11 +
                                                8) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 11 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                11 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                11 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                11 + 32 -
                    8 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x8080808 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 11 - 8)
                    as u32;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 &
            ((1) << 11 + 9) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            11 +
                                                9) as
            i32 as i32;
    /* Up left */
    if opp_bits_high &
           ((1) << 11 - 9) as
               u32 != 0 {
        fl =
            my_bits_low >> 32 - 9 &
                ((1) << 11 - 9)
                    as u32;
        t =
            opp_bits_low & my_bits_low << 9 &
                ((1) <<
                     11 + 32 -
                         9 * 2) as u32;
        my_bits_high |=
            fl.wrapping_add(t >> 32 - 9);
        my_bits_low |= t;
        flipped =
            (flipped as
                 u32).wrapping_add(fl >>
                                                11 -
                                                    9 |
                                                t >>
                                                    11 +
                                                        32 -
                                                        9 *
                                                            2 -
                                                        1) as
                i32 as i32
    }
    my_bits_high |= 0x800 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_e6( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_high >> 12 - 3 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        12 -
                            4).wrapping_add(16 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 12 - 3;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 &
            ((1) << 12 + 7) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            12 +
                                                7) as
            i32 as i32;
    /* Up right */
    if opp_bits_high &
           ((1) << 12 - 7) as
               u32 != 0 {
        fl =
            my_bits_low >> 32 - 7 &
                ((1) << 12 - 7)
                    as u32;
        t =
            opp_bits_low & my_bits_low << 7 &
                ((1) <<
                     12 + 32 -
                         7 * 2) as u32;
        my_bits_high |=
            fl.wrapping_add(t >> 32 - 7);
        my_bits_low |= t;
        flipped =
            (flipped as
                 u32).wrapping_add(fl >>
                                                12 -
                                                    7 |
                                                t >>
                                                    12 +
                                                        32 -
                                                        7 *
                                                            2 -
                                                        1) as
                i32 as i32
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 &
            ((1) << 12 + 8) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            12 +
                                                8) as
            i32 as i32;
    /* Up */
    if opp_bits_high &
           ((1) << 12 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                12 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                12 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t &=
            opp_bits_low >>
                12 + 32 -
                    8 * 4;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x10101010 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 12 - 8)
                    as u32;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 &
            ((1) << 12 + 9) as
                u32;
    my_bits_high |= t;
    flipped =
        (flipped as
             u32).wrapping_add(t >>
                                            12 +
                                                9) as
            i32 as i32;
    /* Up left */
    if opp_bits_high &
           ((1) << 12 - 9) as
               u32 != 0 {
        t =
            opp_bits_low >>
                12 + 32 -
                    9 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                12 + 32 -
                    9 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        t =
            msb_mask[(contig - 1) as usize] &
                0x4020100 as u32;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1) << 12 - 9)
                    as u32;
            flipped += contig
        }
    }
    my_bits_high |= 0x1000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_d4( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_low >> 27 - 2 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        27 -
                            3).wrapping_add(24 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 27 - 2;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    t =
        opp_bits_high &
            (opp_bits_high << 7 |
                 ((1) <<
                      27 + 7 -
                          32) as u32) &
            my_bits_high >> 7 & 0x204 as u32;
    my_bits_high |= t.wrapping_add(t >> 7);
    flipped =
        (flipped as
             u32).wrapping_add((t >>
                                             27 +
                                                 7 -
                                                 32 |
                                             t >>
                                                 27 +
                                                     7 *
                                                         2 -
                                                     32 -
                                                     1) &
                                            3 as u32)
            as i32 as i32;
    /* Up right */
    if opp_bits_low &
           ((1) << 27 - 7) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 7 &
                0x102000 as u32;
        my_bits_low |= t.wrapping_add(t << 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 27 -
                                                     7 |
                                                 t >>
                                                     27 -
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1) <<
                27 + 8 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                27 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                27 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x8080808 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) << 27 - 8) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 &
                0x80800 as u32;
        my_bits_low |= t.wrapping_add(t << 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 27 -
                                                     8 |
                                                 t >>
                                                     27 -
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1) <<
                27 + 9 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                27 + 9 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                27 + 9 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x80402010 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up left */
    if opp_bits_low &
           ((1) << 27 - 9) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 9 &
                0x40200 as u32;
        my_bits_low |= t.wrapping_add(t << 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 27 -
                                                     9 |
                                                 t >>
                                                     27 -
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x8000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_e4( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_low >> 28 - 3 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        28 -
                            4).wrapping_add(16 as
                                                              u32) as
                   usize] as u32;
    my_bits_low |= fl << 28 - 3;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_high &
           ((1) <<
                28 + 7 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                28 + 7 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                28 + 7 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x1020408 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up right */
    if opp_bits_low &
           ((1) << 28 - 7) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 7 &
                0x204000 as u32;
        my_bits_low |= t.wrapping_add(t << 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 28 -
                                                     7 |
                                                 t >>
                                                     28 -
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1) <<
                28 + 8 - 32) as
               u32 != 0 {
        t =
            opp_bits_high >>
                28 + 8 * 2 -
                    32 & 1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_high >>
                28 + 8 * 3 -
                    32;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = lsb_mask[contig as usize] & 0x10101010 as u32;
        t =
            (-((my_bits_high & fl) as i32) >> 31) as
                u32;
        my_bits_high |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) << 28 - 8) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 8 &
                0x101000 as u32;
        my_bits_low |= t.wrapping_add(t << 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 28 -
                                                     8 |
                                                 t >>
                                                     28 -
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Down right */
    t =
        opp_bits_high &
            (opp_bits_high << 9 |
                 ((1) <<
                      28 + 9 -
                          32) as u32) &
            my_bits_high >> 9 & 0x4020 as u32;
    my_bits_high |= t.wrapping_add(t >> 9);
    flipped =
        (flipped as
             u32).wrapping_add((t >>
                                             28 +
                                                 9 -
                                                 32 |
                                             t >>
                                                 28 +
                                                     9 *
                                                         2 -
                                                     32 -
                                                     1) &
                                            3 as u32)
            as i32 as i32;
    /* Up left */
    if opp_bits_low &
           ((1) << 28 - 9) as
               u32 != 0 {
        t =
            opp_bits_low & my_bits_low << 9 &
                0x80400 as u32;
        my_bits_low |= t.wrapping_add(t << 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 28 -
                                                     9 |
                                                 t >>
                                                     28 -
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    my_bits_low |= 0x10000000 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_d5( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_high >> 3 - 2 &
                        0x3b as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        3 -
                            3).wrapping_add(24 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 3 - 2;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_high &
           ((1) << 3 + 7) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 &
                0x20400 as u32;
        my_bits_high |= t.wrapping_add(t >> 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 3 +
                                                     7 |
                                                 t >>
                                                     3 +
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up right */
    if opp_bits_low &
           ((1) <<
                3 + 32 - 7) as
               u32 != 0 {
        t =
            opp_bits_low >>
                3 + 32 -
                    7 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                3 + 32 -
                    7 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x10204080 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down */
    if opp_bits_high &
           ((1) << 3 + 8) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 &
                0x80800 as u32;
        my_bits_high |= t.wrapping_add(t >> 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 3 +
                                                     8 |
                                                 t >>
                                                     3 +
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) <<
                3 + 32 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                3 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                3 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x8080808 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1) << 3 + 9) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 &
                0x201000 as u32;
        my_bits_high |= t.wrapping_add(t >> 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 3 +
                                                     9 |
                                                 t >>
                                                     3 +
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up left */
    t =
        opp_bits_low &
            (opp_bits_low >> 9 |
                 ((1) <<
                      3 + 32 - 9)
                     as u32) & my_bits_low << 9 &
            0x4020000 as u32;
    my_bits_low |= t.wrapping_add(t << 9);
    flipped =
        (flipped as
             u32).wrapping_add((t >>
                                             3 +
                                                 32 -
                                                 9 |
                                             t >>
                                                 3 +
                                                     32 -
                                                     9 *
                                                         2 -
                                                     1) &
                                            3 as u32)
            as i32 as i32;
    my_bits_high |= 0x8 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}
fn TestFlips_bitboard_e5( mut my_bits_high: u32,
                                           mut my_bits_low: u32,
                                           opp_bits_high: u32,
                                           opp_bits_low: u32)
 -> (i32, BitBoard) {
    let mut flipped: i32 = 0;
    let mut contig: i32 = 0;
    let mut t: u32 = 0;
    let mut fl: u32 = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_high >> 4 - 3 &
                        0x37 as i32 as u32) as usize] as
            u32;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        4 -
                            4).wrapping_add(16 as
                                                              u32) as
                   usize] as u32;
    my_bits_high |= fl << 4 - 3;
    flipped = pop_count[fl as usize] as i32;
    /* Down left */
    if opp_bits_high &
           ((1) << 4 + 7) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 &
                0x40800 as u32;
        my_bits_high |= t.wrapping_add(t >> 7);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 4 +
                                                     7 |
                                                 t >>
                                                     4 +
                                                         7 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up right */
    t =
        opp_bits_low &
            (opp_bits_low >> 7 |
                 ((1) <<
                      4 + 32 - 7)
                     as u32) & my_bits_low << 7 &
            0x20400000 as u32;
    my_bits_low |= t.wrapping_add(t << 7);
    flipped =
        (flipped as
             u32).wrapping_add((t >>
                                             4 +
                                                 32 -
                                                 7 |
                                             t >>
                                                 4 +
                                                     32 -
                                                     7 *
                                                         2 -
                                                     1) &
                                            3 as u32)
            as i32 as i32;
    /* Down */
    if opp_bits_high &
           ((1) << 4 + 8) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 &
                0x101000 as u32;
        my_bits_high |= t.wrapping_add(t >> 8);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 4 +
                                                     8 |
                                                 t >>
                                                     4 +
                                                         8 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up */
    if opp_bits_low &
           ((1) <<
                4 + 32 - 8) as
               u32 != 0 {
        t =
            opp_bits_low >>
                4 + 32 -
                    8 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                4 + 32 -
                    8 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x10101010 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    /* Down right */
    if opp_bits_high &
           ((1) << 4 + 9) as
               u32 != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 &
                0x402000 as u32;
        my_bits_high |= t.wrapping_add(t >> 9);
        flipped =
            (flipped as
                 u32).wrapping_add((t >>
                                                 4 +
                                                     9 |
                                                 t >>
                                                     4 +
                                                         9 *
                                                             2
                                                         - 1) &
                                                3 as
                                                    u32) as
                i32 as i32
    }
    /* Up left */
    if opp_bits_low &
           ((1) <<
                4 + 32 - 9) as
               u32 != 0 {
        t =
            opp_bits_low >>
                4 + 32 -
                    9 * 2 &
                1 as u32;
        contig =
            (1 as u32).wrapping_add(t) as i32;
        t &=
            opp_bits_low >>
                4 + 32 -
                    9 * 3;
        contig =
            (contig as u32).wrapping_add(t) as i32 as
                i32;
        fl = msb_mask[contig as usize] & 0x8040201 as u32;
        t =
            (-((my_bits_low & fl) as i32) >> 31) as
                u32;
        my_bits_low |= fl & t;
        flipped =
            (flipped as u32).wrapping_add(contig as u32 & t)
                as i32 as i32
    }
    my_bits_high |= 0x10 as u32;
    return (flipped, BitBoard{ high: my_bits_high, low: my_bits_low });
}

fn null(_: u32, _: u32, _: u32, _: u32) -> (i32, BitBoard) {
    // perf??
    unreachable!()
}
pub static TestFlips_bitboard: [fn(_: u32, _: u32, _: u32, _: u32) -> (i32, BitBoard); 78] = [
    (TestFlips_bitboard_a1),
    (TestFlips_bitboard_b1),
    (TestFlips_bitboard_c1),
    (TestFlips_bitboard_d1),
    (TestFlips_bitboard_e1),
    (TestFlips_bitboard_f1),
    (TestFlips_bitboard_g1),
    (TestFlips_bitboard_h1), null, null,
    (TestFlips_bitboard_a2),
    (TestFlips_bitboard_b2),
    (TestFlips_bitboard_c2),
    (TestFlips_bitboard_d2),
    (TestFlips_bitboard_e2),
    (TestFlips_bitboard_f2),
    (TestFlips_bitboard_g2),
    (TestFlips_bitboard_h2), null, null,
    (TestFlips_bitboard_a3),
    (TestFlips_bitboard_b3),
    (TestFlips_bitboard_c3),
    (TestFlips_bitboard_d3),
    (TestFlips_bitboard_e3),
    (TestFlips_bitboard_f3),
    (TestFlips_bitboard_g3),
    (TestFlips_bitboard_h3), null, null,
    (TestFlips_bitboard_a4),
    (TestFlips_bitboard_b4),
    (TestFlips_bitboard_c4),
    (TestFlips_bitboard_d4),
    (TestFlips_bitboard_e4),
    (TestFlips_bitboard_f4),
    (TestFlips_bitboard_g4),
    (TestFlips_bitboard_h4), null, null,
    (TestFlips_bitboard_a5),
    (TestFlips_bitboard_b5),
    (TestFlips_bitboard_c5),
    (TestFlips_bitboard_d5),
    (TestFlips_bitboard_e5),
    (TestFlips_bitboard_f5),
    (TestFlips_bitboard_g5),
    (TestFlips_bitboard_h5), null, null,
    (TestFlips_bitboard_a6),
    (TestFlips_bitboard_b6),
    (TestFlips_bitboard_c6),
    (TestFlips_bitboard_d6),
    (TestFlips_bitboard_e6),
    (TestFlips_bitboard_f6),
    (TestFlips_bitboard_g6),
    (TestFlips_bitboard_h6), null, null,
    (TestFlips_bitboard_a7),
    (TestFlips_bitboard_b7),
    (TestFlips_bitboard_c7),
    (TestFlips_bitboard_d7),
    (TestFlips_bitboard_e7),
    (TestFlips_bitboard_f7),
    (TestFlips_bitboard_g7),
    (TestFlips_bitboard_h7), null, null,
    (TestFlips_bitboard_a8),
    (TestFlips_bitboard_b8),
    (TestFlips_bitboard_c8),
    (TestFlips_bitboard_d8),
    (TestFlips_bitboard_e8),
    (TestFlips_bitboard_f8),
    (TestFlips_bitboard_g8),
    (TestFlips_bitboard_h8)
];
