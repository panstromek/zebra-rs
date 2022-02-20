
/*
   File:          bitbcnt.c

   Modified:      November 24, 2005

   Authors:       Gunnar Andersson (gunnar@radagast.se)
          Toshihiko Okuhara

   Contents:      Count flips for the last move.

   This piece of software is released under the GPL.
   See the file COPYING for more information.
*/

// NULL
// #include "bitboard.h"
static right_count: [i8; 128] =
    [0 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     3 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     4 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     3 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     5 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     3 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     4 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     3 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     6 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     3 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     4 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     3 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     5 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     3 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     4 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8,
     3 as i8, 0 as i8,
     1 as i8, 0 as i8,
     2 as i8, 0 as i8,
     1 as i8, 0 as i8];
static left_count: [i8; 128] =
    [0 as i8, 6 as i8,
     5 as i8, 5 as i8,
     4 as i8, 4 as i8,
     4 as i8, 4 as i8,
     3 as i8, 3 as i8,
     3 as i8, 3 as i8,
     3 as i8, 3 as i8,
     3 as i8, 3 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8];
static center_count: [i8; 256] =
    [0 as i8, 3 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 3 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     1 as i8, 4 as i8,
     3 as i8, 3 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 3 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     2 as i8, 5 as i8,
     4 as i8, 4 as i8,
     3 as i8, 3 as i8,
     3 as i8, 3 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     0 as i8, 3 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     1 as i8, 4 as i8,
     3 as i8, 3 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 3 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     3 as i8, 6 as i8,
     5 as i8, 5 as i8,
     4 as i8, 4 as i8,
     4 as i8, 4 as i8,
     3 as i8, 3 as i8,
     3 as i8, 3 as i8,
     3 as i8, 3 as i8,
     3 as i8, 3 as i8,
     0 as i8, 3 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     1 as i8, 4 as i8,
     3 as i8, 3 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 3 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     2 as i8, 5 as i8,
     4 as i8, 4 as i8,
     3 as i8, 3 as i8,
     3 as i8, 3 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     0 as i8, 3 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     1 as i8, 4 as i8,
     3 as i8, 3 as i8,
     2 as i8, 2 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 3 as i8,
     2 as i8, 2 as i8,
     1 as i8, 1 as i8,
     1 as i8, 1 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8,
     0 as i8, 0 as i8];
fn CountFlips_bitboard_a1(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 0 + 1 &
                         0x7f as i32 as u32) as usize];
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x1010100 as
                                   u32).wrapping_add((my_bits_high &
                                                                   0x1010101
                                                                       as
                                                                       u32)
                                                                  <<
                                                                  4).wrapping_mul(0x1020408
                                                                                                    as
                                                                                                    u32)
                              >> 25) as usize] as i32)
            as i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x8040200 as
                                   u32).wrapping_add(my_bits_high &
                                                                  0x80402010
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 25) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_h1(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Left */
    flipped =
        left_count[(my_bits_low << 7 - 7 &
                        0x7f as i32 as u32) as usize];
    /* Down left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_low &
                              0x10204000 as
                                  u32).wrapping_add(my_bits_high &
                                                                 0x1020408 as
                                                                     u32).wrapping_mul(0x1010101
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[(((my_bits_low & 0x80808000 as u32) >>
                               4).wrapping_add(my_bits_high &
                                                                 0x80808080 as
                                                                     u32).wrapping_mul(0x204081
                                                                                                    as
                                                                                                    u32)
                              >> 25) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_a8(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 24 + 1) as
                        usize];
    /* Up right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_high &
                               0x20408 as
                                   u32).wrapping_add(my_bits_low &
                                                                  0x10204080
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 25) as usize] as i32)
            as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[(((my_bits_high & 0x10101 as u32) <<
                              4).wrapping_add(my_bits_low &
                                                                0x1010101 as
                                                                    u32).wrapping_mul(0x1020408
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_h8(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Left */
    flipped =
        left_count[(my_bits_high >> 31 - 7 &
                        0x7f as i32 as u32) as usize];
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x808080 as
                                  u32).wrapping_add((my_bits_low &
                                                                  0x80808080
                                                                      as
                                                                      u32)
                                                                 >>
                                                                 4).wrapping_mul(0x204081
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Up left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x402010 as
                                  u32).wrapping_add(my_bits_low &
                                                                 0x8040201 as
                                                                     u32).wrapping_mul(0x1010101
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_b1(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 1 + 1 &
                         0x3f as i32 as u32) as usize];
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x2020200 as
                                   u32).wrapping_add((my_bits_high &
                                                                   0x2020202
                                                                       as
                                                                       u32)
                                                                  <<
                                                                  4).wrapping_mul(0x810204
                                                                                                    as
                                                                                                    u32)
                              >> 25) as usize] as i32)
            as i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x10080400 as
                                   u32).wrapping_add(my_bits_high &
                                                                  0x804020 as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 26) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_g1(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Left */
    flipped =
        left_count[(my_bits_low << 7 - 6 &
                        0x7e as i32 as u32) as usize];
    /* Down left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_low &
                              0x8102000 as
                                  u32).wrapping_add(my_bits_high &
                                                                 0x10204 as
                                                                     u32).wrapping_mul(0x2020202
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[(((my_bits_low & 0x40404000 as u32) >>
                               4).wrapping_add(my_bits_high &
                                                                 0x40404040 as
                                                                     u32).wrapping_mul(0x408102
                                                                                                    as
                                                                                                    u32)
                              >> 25) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_a2(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 8 + 1 &
                         0x7f as i32 as u32) as usize];
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x1010000 as
                                   u32).wrapping_add((my_bits_high &
                                                                   0x1010101
                                                                       as
                                                                       u32)
                                                                  <<
                                                                  4).wrapping_mul(0x1020408
                                                                                                    as
                                                                                                    u32)
                              >> 26) as usize] as i32)
            as i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x4020000 as
                                   u32).wrapping_add(my_bits_high &
                                                                  0x40201008
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 25) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_h2(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Left */
    flipped =
        left_count[(my_bits_low >> 15 - 7 &
                        0x7f as i32 as u32) as usize];
    /* Down left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_low &
                              0x20400000 as
                                  u32).wrapping_add(my_bits_high &
                                                                 0x2040810 as
                                                                     u32).wrapping_mul(0x1010101
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[(((my_bits_low & 0x80800000 as u32) >>
                               4).wrapping_add(my_bits_high &
                                                                 0x80808080 as
                                                                     u32).wrapping_mul(0x204081
                                                                                                    as
                                                                                                    u32)
                              >> 26) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_a7(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 16 + 1 &
                         0x7f as i32 as u32) as usize];
    /* Up right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_high &
                               0x204 as
                                   u32).wrapping_add(my_bits_low &
                                                                  0x8102040 as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 25) as usize] as i32)
            as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[(((my_bits_high & 0x101 as u32) <<
                              4).wrapping_add(my_bits_low &
                                                                0x1010101 as
                                                                    u32).wrapping_mul(0x2040810
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_h7(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Left */
    flipped =
        left_count[(my_bits_high >> 23 - 7 &
                        0x7f as i32 as u32) as usize];
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x8080 as
                                  u32).wrapping_add((my_bits_low &
                                                                  0x80808080
                                                                      as
                                                                      u32)
                                                                 >>
                                                                 4).wrapping_mul(0x408102
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Up left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x4020 as
                                  u32).wrapping_add(my_bits_low &
                                                                 0x10080402 as
                                                                     u32).wrapping_mul(0x1010101
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_b8(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 25 + 1) as
                        usize];
    /* Up right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_high &
                               0x40810 as
                                   u32).wrapping_add(my_bits_low &
                                                                  0x20408000
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 26) as usize] as i32)
            as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[(((my_bits_high & 0x20202 as u32) <<
                              4).wrapping_add(my_bits_low &
                                                                0x2020202 as
                                                                    u32).wrapping_mul(0x810204
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_g8(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Left */
    flipped =
        left_count[(my_bits_high >> 30 - 7 &
                        0x7e as i32 as u32) as usize];
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x404040 as
                                  u32).wrapping_add((my_bits_low &
                                                                  0x40404040
                                                                      as
                                                                      u32)
                                                                 >>
                                                                 4).wrapping_mul(0x408102
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Up left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x201008 as
                                  u32).wrapping_add(my_bits_low &
                                                                 0x4020100 as
                                                                     u32).wrapping_mul(0x2020202
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_b2(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 9 + 1 &
                         0x3f as i32 as u32) as usize];
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x2020000 as
                                   u32).wrapping_add((my_bits_high &
                                                                   0x2020202
                                                                       as
                                                                       u32)
                                                                  <<
                                                                  4).wrapping_mul(0x810204
                                                                                                    as
                                                                                                    u32)
                              >> 26) as usize] as i32)
            as i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x8040000 as
                                   u32).wrapping_add(my_bits_high &
                                                                  0x80402010
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 26) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_g2(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Left */
    flipped =
        left_count[(my_bits_low >> 14 - 7 &
                        0x7e as i32 as u32) as usize];
    /* Down left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_low &
                              0x10200000 as
                                  u32).wrapping_add(my_bits_high &
                                                                 0x1020408 as
                                                                     u32).wrapping_mul(0x2020202
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[(((my_bits_low & 0x40400000 as u32) >>
                               4).wrapping_add(my_bits_high &
                                                                 0x40404040 as
                                                                     u32).wrapping_mul(0x408102
                                                                                                    as
                                                                                                    u32)
                              >> 26) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_b7(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 17 + 1 &
                         0x3f as i32 as u32) as usize];
    /* Up right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_high &
                               0x408 as
                                   u32).wrapping_add(my_bits_low &
                                                                  0x10204080
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 26) as usize] as i32)
            as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[(((my_bits_high & 0x202 as u32) <<
                              4).wrapping_add(my_bits_low &
                                                                0x2020202 as
                                                                    u32).wrapping_mul(0x1020408
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_g7(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Left */
    flipped =
        left_count[(my_bits_high >> 22 - 7 &
                        0x7e as i32 as u32) as usize];
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x4040 as
                                  u32).wrapping_add((my_bits_low &
                                                                  0x40404040
                                                                      as
                                                                      u32)
                                                                 >>
                                                                 4).wrapping_mul(0x810204
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Up left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x2010 as
                                  u32).wrapping_add(my_bits_low &
                                                                 0x8040201 as
                                                                     u32).wrapping_mul(0x2020202
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_c1(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down */
    flipped =
        right_count[((my_bits_low &
                          0x4040400 as
                              u32).wrapping_add((my_bits_high &
                                                              0x4040404 as
                                                                  u32)
                                                             <<
                                                             4).wrapping_mul(0x408102
                                                                                               as
                                                                                               u32)
                         >> 25) as usize];
    /* Down right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x20100800 as
                                   u32).wrapping_add(my_bits_high &
                                                                  0x8040 as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 27) as usize] as i32)
            as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             right_count[(my_bits_low >> 2 + 1 &
                              0x1f as i32 as u32) as usize]
                 as i32) as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x3 as u32 == 0x1 as u32) as
                 i32 as i8 as i32) as i8;
    /* Down left */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x10200 as u32 ==
                  0x10000 as u32) as i32 as i8 as
                 i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_f1(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down */
    flipped =
        right_count[((my_bits_high &
                          0x20202020 as
                              u32).wrapping_add(my_bits_low >>
                                                             4
                                                             &
                                                             0x2020200 as
                                                                 u32).wrapping_mul(0x810204
                                                                                                as
                                                                                                u32)
                         >> 25) as usize];
    /* Down left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_low &
                              0x4081000 as
                                  u32).wrapping_add(my_bits_high &
                                                                 0x102 as
                                                                     u32).wrapping_mul(0x4040404
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_low << 7 - 5 &
                             0x7c as i32 as u32) as usize] as
                 i32) as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0xc0 as u32 == 0x80 as u32) as
                 i32 as i8 as i32) as i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x804000 as u32 ==
                  0x800000 as u32) as i32 as i8 as
                 i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_a3(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down */
    flipped =
        right_count[((my_bits_low &
                          0x1000000 as
                              u32).wrapping_add((my_bits_high &
                                                              0x1010101 as
                                                                  u32)
                                                             <<
                                                             4).wrapping_mul(0x1020408
                                                                                               as
                                                                                               u32)
                         >> 27) as usize];
    /* Down right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x2000000 as
                                   u32).wrapping_add(my_bits_high &
                                                                  0x20100804
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 25) as usize] as i32)
            as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             right_count[(my_bits_low >> 16 + 1
                              & 0x7f as i32 as u32) as usize]
                 as i32) as i8;
    /* Up right */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x204 as u32 == 0x4 as u32) as
                 i32 as i8 as i32) as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x101 as u32 == 0x1 as u32) as
                 i32 as i8 as i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_h3(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down */
    flipped =
        right_count[((my_bits_high &
                          0x80808080 as
                              u32).wrapping_add((my_bits_low &
                                                              0x80000000 as
                                                                  u32)
                                                             >>
                                                             4).wrapping_mul(0x204081
                                                                                               as
                                                                                               u32)
                         >> 27) as usize];
    /* Down left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_low &
                              0x40000000 as
                                  u32).wrapping_add(my_bits_high &
                                                                 0x4081020 as
                                                                     u32).wrapping_mul(0x1010101
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_low >> 23 - 7 &
                             0x7f as i32 as u32) as usize] as
                 i32) as i8;
    /* Up left */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x4020 as u32 == 0x20 as u32) as
                 i32 as i8 as i32) as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x8080 as u32 == 0x80 as u32) as
                 i32 as i8 as i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_a6(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Up */
    flipped =
        left_count[((my_bits_low &
                         0x1010101 as
                             u32).wrapping_add((my_bits_high &
                                                             0x1 as
                                                                 u32)
                                                            <<
                                                            4).wrapping_mul(0x4081020
                                                                                              as
                                                                                              u32)
                        >> 24) as usize];
    /* Up right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_high &
                               0x2 as
                                   u32).wrapping_add(my_bits_low &
                                                                  0x4081020 as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 25) as usize] as i32)
            as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             right_count[(my_bits_high >> 8 + 1
                              & 0x7f as i32 as u32) as usize]
                 as i32) as i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x4020000 as u32 ==
                  0x4000000 as u32) as i32 as i8 as
                 i32) as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x1010000 as u32 ==
                  0x1000000 as u32) as i32 as i8 as
                 i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_h6(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Up */
    flipped =
        left_count[((my_bits_high &
                         0x80 as
                             u32).wrapping_add((my_bits_low &
                                                             0x80808080 as
                                                                 u32)
                                                            >>
                                                            4).wrapping_mul(0x810204
                                                                                              as
                                                                                              u32)
                        >> 24) as usize];
    /* Up left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x40 as
                                  u32).wrapping_add(my_bits_low &
                                                                 0x20100804 as
                                                                     u32).wrapping_mul(0x1010101
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_high >> 15 - 7
                             & 0x7f as i32 as u32) as usize]
                 as i32) as i8;
    /* Down left */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x20400000 as u32 ==
                  0x20000000 as u32) as i32 as i8
                 as i32) as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x80800000 as u32 ==
                  0x80000000 as u32) as i32 as i8
                 as i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_c8(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Up */
    flipped =
        left_count[((my_bits_low &
                         0x4040404 as
                             u32).wrapping_add((my_bits_high &
                                                             0x40404 as
                                                                 u32)
                                                            <<
                                                            4).wrapping_mul(0x408102
                                                                                              as
                                                                                              u32)
                        >> 24) as usize];
    /* Up right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_high &
                               0x81020 as
                                   u32).wrapping_add(my_bits_low &
                                                                  0x40800000
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 27) as usize] as i32)
            as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             right_count[(my_bits_high >>
                              26 + 1) as usize]
                 as i32) as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x3000000 as u32 ==
                  0x1000000 as u32) as i32 as i8 as
                 i32) as i8;
    /* Up left */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x20100 as u32 == 0x100 as u32)
                 as i32 as i8 as i32) as
            i8;
    return flipped as i32;
}
fn CountFlips_bitboard_f8(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Up */
    flipped =
        left_count[((my_bits_high &
                         0x202020 as
                             u32).wrapping_add((my_bits_low &
                                                             0x20202020 as
                                                                 u32)
                                                            >>
                                                            4).wrapping_mul(0x810204
                                                                                              as
                                                                                              u32)
                        >> 24) as usize];
    /* Up left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x100804 as
                                  u32).wrapping_add(my_bits_low &
                                                                 0x2010000 as
                                                                     u32).wrapping_mul(0x4040404
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_high >> 29 - 7
                             & 0x7c as i32 as u32) as usize]
                 as i32) as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0xc0000000 as u32 ==
                  0x80000000 as u32) as i32 as i8
                 as i32) as i8;
    /* Up right */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x408000 as u32 ==
                  0x8000 as u32) as i32 as i8 as
                 i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_d1(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xf7 as
                               u32).wrapping_add(my_bits_low &
                                                              0x7 as
                                                                  u32)
                          >> 0) as usize];
    /* Down left / Down right */
    flipped =
        (flipped as i32 +
             center_count[(((my_bits_low &
                                 0x1020400 as i32 as u32) <<
                                1).wrapping_add(my_bits_low &
                                                                  0x40201000
                                                                      as
                                                                      u32).wrapping_add(my_bits_high
                                                                                                     &
                                                                                                     0x80
                                                                                                         as
                                                                                                         u32).wrapping_mul(0x1010101
                                                                                                                                        as
                                                                                                                                        u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x8080800 as
                                   u32).wrapping_add((my_bits_high &
                                                                   0x8080808
                                                                       as
                                                                       u32)
                                                                  <<
                                                                  4).wrapping_mul(0x204081
                                                                                                    as
                                                                                                    u32)
                              >> 25) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_e1(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xef as
                               u32).wrapping_add(my_bits_low &
                                                              0xf as
                                                                  u32)
                          >> 1) as usize];
    /* Down left / Down right */
    flipped =
        (flipped as i32 +
             center_count[(((my_bits_low &
                                 0x2040800 as
                                     u32).wrapping_add(my_bits_high &
                                                                    0x1 as
                                                                        u32)
                                <<
                                1).wrapping_add(my_bits_low &
                                                                  0x80402000
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                               >> 25) as usize] as i32)
            as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[(((my_bits_low & 0x10101000 as u32) >>
                               4).wrapping_add(my_bits_high &
                                                                 0x10101010 as
                                                                     u32).wrapping_mul(0x1020408
                                                                                                    as
                                                                                                    u32)
                              >> 25) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_a4(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 24 + 1) as
                        usize];
    /* Up right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x20408 as
                                   u32).wrapping_mul(0x1010100 as
                                                                  u32)
                              >> 25) as usize] as i32)
            as i8;
    /* Down / Up */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_low &
                                0x10101 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x1010101
                                                                        as
                                                                        u32)
                                                                   <<
                                                                   3).wrapping_mul(0x2040810
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_high &
                               0x10080402 as
                                   u32).wrapping_mul(0x1010101 as
                                                                  u32)
                              >> 25) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_h4(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down / Up */
    flipped =
        center_count[((my_bits_high &
                           0x80808080 as
                               u32).wrapping_add((my_bits_low &
                                                               0x808080 as
                                                                   u32)
                                                              >>
                                                              3).wrapping_mul(0x204081
                                                                                                as
                                                                                                u32)
                          >> 24) as usize];
    /* Down left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x8102040 as
                                  u32).wrapping_mul(0x1010101 as
                                                                 u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_low >> 31 - 7 &
                             0x7f as i32 as u32) as usize] as
                 i32) as i8;
    /* Up left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_low &
                              0x402010 as
                                  u32).wrapping_mul(0x1010100 as
                                                                 u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_a5(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 0 + 1 &
                         0x7f as i32 as u32) as usize];
    /* Up right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x2040810 as
                                   u32).wrapping_mul(0x1010101 as
                                                                  u32)
                              >> 25) as usize] as i32)
            as i8;
    /* Down / Up */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_low &
                                0x1010101 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x1010100
                                                                        as
                                                                        u32)
                                                                   <<
                                                                   3).wrapping_mul(0x2040810
                                                                                                     as
                                                                                                     u32)
                               >> 25) as usize] as i32)
            as i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_high &
                               0x8040200 as
                                   u32).wrapping_mul(0x10101 as
                                                                  u32)
                              >> 25) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_h5(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down / Up */
    flipped =
        center_count[((my_bits_high &
                           0x80808000 as
                               u32).wrapping_add((my_bits_low &
                                                               0x80808080 as
                                                                   u32)
                                                              >>
                                                              3).wrapping_mul(0x204081
                                                                                                as
                                                                                                u32)
                          >> 25) as usize];
    /* Up left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_low &
                              0x40201008 as
                                  u32).wrapping_mul(0x1010101 as
                                                                 u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_high << 7 - 7 &
                             0x7f as i32 as u32) as usize] as
                 i32) as i8;
    /* Down left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x10204000 as
                                  u32).wrapping_mul(0x10101 as
                                                                 u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_d8(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xf7000000 as
                               u32).wrapping_add(my_bits_high &
                                                              0x7000000 as
                                                                  u32)
                          >> 24) as usize];
    /* Up right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x102040 as
                                    u32).wrapping_add(my_bits_low &
                                                                   0x80000000
                                                                       as
                                                                       u32).wrapping_add((my_bits_high
                                                                                                       &
                                                                                                       0x40201
                                                                                                           as
                                                                                                           u32)
                                                                                                      <<
                                                                                                      1).wrapping_mul(0x1010101
                                                                                                                                        as
                                                                                                                                        u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[(((my_bits_high & 0x80808 as u32) <<
                              4).wrapping_add(my_bits_low &
                                                                0x8080808 as
                                                                    u32).wrapping_mul(0x204081
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_e8(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xef000000 as
                               u32).wrapping_add(my_bits_high &
                                                              0xf000000 as
                                                                  u32)
                          >> 25) as usize];
    /* Up right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x204080 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x80402 as
                                                                        u32).wrapping_add(my_bits_low
                                                                                                       &
                                                                                                       0x1000000
                                                                                                           as
                                                                                                           u32)
                                                                   <<
                                                                   1).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                               >> 25) as usize] as i32)
            as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x101010 as
                                  u32).wrapping_add((my_bits_low &
                                                                  0x10101010
                                                                      as
                                                                      u32)
                                                                 >>
                                                                 4).wrapping_mul(0x1020408
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_c2(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down */
    flipped =
        right_count[((my_bits_low &
                          0x4040000 as
                              u32).wrapping_add((my_bits_high &
                                                              0x4040404 as
                                                                  u32)
                                                             <<
                                                             4).wrapping_mul(0x408102
                                                                                               as
                                                                                               u32)
                         >> 26) as usize];
    /* Down right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x10080000 as
                                   u32).wrapping_add(my_bits_high &
                                                                  0x804020 as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 27) as usize] as i32)
            as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             right_count[(my_bits_low >> 10 + 1
                              & 0x1f as i32 as u32) as usize]
                 as i32) as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x300 as u32 == 0x100 as u32) as
                 i32 as i8 as i32) as i8;
    /* Down left */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x1020000 as u32 ==
                  0x1000000 as u32) as i32 as i8 as
                 i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_f2(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down */
    flipped =
        right_count[((my_bits_high &
                          0x20202020 as
                              u32).wrapping_add((my_bits_low &
                                                              0x20200000 as
                                                                  u32)
                                                             >>
                                                             4).wrapping_mul(0x810204
                                                                                               as
                                                                                               u32)
                         >> 26) as usize];
    /* Down left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_low &
                              0x8100000 as
                                  u32).wrapping_add(my_bits_high &
                                                                 0x10204 as
                                                                     u32).wrapping_mul(0x4040404
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_low >> 13 - 7 &
                             0x7c as i32 as u32) as usize] as
                 i32) as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0xc000 as u32 == 0x8000 as u32)
                 as i32 as i8 as i32) as
            i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x80400000 as u32 ==
                  0x80000000 as u32) as i32 as i8
                 as i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_b3(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down */
    flipped =
        right_count[((my_bits_low &
                          0x2000000 as
                              u32).wrapping_add((my_bits_high &
                                                              0x2020202 as
                                                                  u32)
                                                             <<
                                                             4).wrapping_mul(0x810204
                                                                                               as
                                                                                               u32)
                         >> 27) as usize];
    /* Down right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x4000000 as
                                   u32).wrapping_add(my_bits_high &
                                                                  0x40201008
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 26) as usize] as i32)
            as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             right_count[(my_bits_low >> 17 + 1
                              & 0x3f as i32 as u32) as usize]
                 as i32) as i8;
    /* Up right */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x408 as u32 == 0x8 as u32) as
                 i32 as i8 as i32) as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x202 as u32 == 0x2 as u32) as
                 i32 as i8 as i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_g3(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down */
    flipped =
        right_count[((my_bits_high &
                          0x40404040 as
                              u32).wrapping_add((my_bits_low &
                                                              0x40000000 as
                                                                  u32)
                                                             >>
                                                             4).wrapping_mul(0x408102
                                                                                               as
                                                                                               u32)
                         >> 27) as usize];
    /* Down left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_low &
                              0x20000000 as
                                  u32).wrapping_add(my_bits_high &
                                                                 0x2040810 as
                                                                     u32).wrapping_mul(0x2020202
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_low >> 22 - 7 &
                             0x7e as i32 as u32) as usize] as
                 i32) as i8;
    /* Up left */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x2010 as u32 == 0x10 as u32) as
                 i32 as i8 as i32) as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x4040 as u32 == 0x40 as u32) as
                 i32 as i8 as i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_b6(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Up */
    flipped =
        left_count[((my_bits_low &
                         0x2020202 as
                             u32).wrapping_add((my_bits_high &
                                                             0x2 as
                                                                 u32)
                                                            <<
                                                            4).wrapping_mul(0x2040810
                                                                                              as
                                                                                              u32)
                        >> 24) as usize];
    /* Up right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_high &
                               0x4 as
                                   u32).wrapping_add(my_bits_low &
                                                                  0x8102040 as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 26) as usize] as i32)
            as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             right_count[(my_bits_high >> 9 + 1
                              & 0x3f as i32 as u32) as usize]
                 as i32) as i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x8040000 as u32 ==
                  0x8000000 as u32) as i32 as i8 as
                 i32) as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x2020000 as u32 ==
                  0x2000000 as u32) as i32 as i8 as
                 i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_g6(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Up */
    flipped =
        left_count[((my_bits_high &
                         0x40 as
                             u32).wrapping_add((my_bits_low &
                                                             0x40404040 as
                                                                 u32)
                                                            >>
                                                            4).wrapping_mul(0x1020408
                                                                                              as
                                                                                              u32)
                        >> 24) as usize];
    /* Up left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x20 as
                                  u32).wrapping_add(my_bits_low &
                                                                 0x10080402 as
                                                                     u32).wrapping_mul(0x2020202
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_high >> 14 - 7
                             & 0x7e as i32 as u32) as usize]
                 as i32) as i8;
    /* Down left */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x10200000 as u32 ==
                  0x10000000 as u32) as i32 as i8
                 as i32) as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x40400000 as u32 ==
                  0x40000000 as u32) as i32 as i8
                 as i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_c7(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Up */
    flipped =
        left_count[((my_bits_low &
                         0x4040404 as
                             u32).wrapping_add((my_bits_high &
                                                             0x404 as
                                                                 u32)
                                                            <<
                                                            4).wrapping_mul(0x810204
                                                                                              as
                                                                                              u32)
                        >> 24) as usize];
    /* Up right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_high &
                               0x810 as
                                   u32).wrapping_add(my_bits_low &
                                                                  0x20408000
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 27) as usize] as i32)
            as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             right_count[(my_bits_high >> 18 + 1
                              & 0x1f as i32 as u32) as usize]
                 as i32) as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x30000 as u32 ==
                  0x10000 as u32) as i32 as i8 as
                 i32) as i8;
    /* Up left */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x201 as u32 == 0x1 as u32) as
                 i32 as i8 as i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_f7(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Up */
    flipped =
        left_count[((my_bits_high &
                         0x2020 as
                             u32).wrapping_add((my_bits_low &
                                                             0x20202020 as
                                                                 u32)
                                                            >>
                                                            4).wrapping_mul(0x1020408
                                                                                              as
                                                                                              u32)
                        >> 24) as usize];
    /* Up left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x1008 as
                                  u32).wrapping_add(my_bits_low &
                                                                 0x4020100 as
                                                                     u32).wrapping_mul(0x4040404
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_high >> 21 - 7
                             & 0x7c as i32 as u32) as usize]
                 as i32) as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0xc00000 as u32 ==
                  0x800000 as u32) as i32 as i8 as
                 i32) as i8;
    /* Up right */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x4080 as u32 == 0x80 as u32)
                 as i32 as i8 as i32) as
            i8;
    return flipped as i32;
}
fn CountFlips_bitboard_d2(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xf700 as
                               u32).wrapping_add(my_bits_low &
                                                              0x700 as
                                                                  u32)
                          >> 8) as usize];
    /* Down left / Down right */
    flipped =
        (flipped as i32 +
             center_count[(((my_bits_low &
                                 0x2040000 as
                                     u32).wrapping_add(my_bits_high &
                                                                    0x1 as
                                                                        u32)
                                <<
                                1).wrapping_add(my_bits_low &
                                                                  0x20100000
                                                                      as
                                                                      u32).wrapping_add(my_bits_high
                                                                                                     &
                                                                                                     0x8040
                                                                                                         as
                                                                                                         u32).wrapping_mul(0x1010101
                                                                                                                                        as
                                                                                                                                        u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x8080000 as
                                   u32).wrapping_add((my_bits_high &
                                                                   0x8080808
                                                                       as
                                                                       u32)
                                                                  <<
                                                                  4).wrapping_mul(0x204081
                                                                                                    as
                                                                                                    u32)
                              >> 26) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_e2(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xef00 as
                               u32).wrapping_add(my_bits_low &
                                                              0xf00 as
                                                                  u32)
                          >> 9) as usize];
    /* Down left / Down right */
    flipped =
        (flipped as i32 +
             center_count[(((my_bits_low &
                                 0x4080000 as
                                     u32).wrapping_add(my_bits_high &
                                                                    0x102 as
                                                                        u32)
                                <<
                                1).wrapping_add(my_bits_high &
                                                                  0x80 as
                                                                      u32).wrapping_add(my_bits_low
                                                                                                     &
                                                                                                     0x40200000
                                                                                                         as
                                                                                                         u32).wrapping_mul(0x1010101
                                                                                                                                        as
                                                                                                                                        u32)
                               >> 25) as usize] as i32)
            as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[(((my_bits_low & 0x10100000 as u32) >>
                               4).wrapping_add(my_bits_high &
                                                                 0x10101010 as
                                                                     u32).wrapping_mul(0x1020408
                                                                                                    as
                                                                                                    u32)
                              >> 26) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_b4(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 25 + 1) as
                        usize];
    /* Up right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x40810 as
                                   u32).wrapping_mul(0x1010100 as
                                                                  u32)
                              >> 26) as usize] as i32)
            as i8;
    /* Down / Up */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_low &
                                0x20202 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x2020202
                                                                        as
                                                                        u32)
                                                                   <<
                                                                   3).wrapping_mul(0x1020408
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_high &
                               0x20100804 as
                                   u32).wrapping_mul(0x1010101 as
                                                                  u32)
                              >> 26) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_g4(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down / Up */
    flipped =
        center_count[((my_bits_high &
                           0x40404040 as
                               u32).wrapping_add((my_bits_low &
                                                               0x404040 as
                                                                   u32)
                                                              >>
                                                              3).wrapping_mul(0x408102
                                                                                                as
                                                                                                u32)
                          >> 24) as usize];
    /* Up left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_low &
                              0x201008 as
                                  u32).wrapping_mul(0x2020200 as
                                                                 u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_low >> 30 - 7 &
                             0x7e as i32 as u32) as usize] as
                 i32) as i8;
    /* Down left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x4081020 as
                                  u32).wrapping_mul(0x2020202 as
                                                                 u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_b5(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 1 + 1 &
                         0x3f as i32 as u32) as usize];
    /* Up right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x4081020 as
                                   u32).wrapping_mul(0x1010101 as
                                                                  u32)
                              >> 26) as usize] as i32)
            as i8;
    /* Down / Up */
    flipped =
        (flipped as i32 +
             center_count[(((my_bits_high & 0x2020200 as u32) <<
                                3).wrapping_add(my_bits_low &
                                                                  0x2020202 as
                                                                      u32).wrapping_mul(0x1020408
                                                                                                     as
                                                                                                     u32)
                               >> 25) as usize] as i32)
            as i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_high &
                               0x10080400 as
                                   u32).wrapping_mul(0x10101 as
                                                                  u32)
                              >> 26) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_g5(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down / Up */
    flipped =
        center_count[((my_bits_high &
                           0x40404000 as
                               u32).wrapping_add((my_bits_low &
                                                               0x40404040 as
                                                                   u32)
                                                              >>
                                                              3).wrapping_mul(0x408102
                                                                                                as
                                                                                                u32)
                          >> 25) as usize];
    /* Down left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x8102000 as
                                  u32).wrapping_mul(0x20202 as
                                                                 u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_high << 7 - 6 &
                             0x7f as i32 as u32) as usize] as
                 i32) as i8;
    /* Up left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_low &
                              0x20100804 as
                                  u32).wrapping_mul(0x2020202 as
                                                                 u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_d7(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xf70000 as
                               u32).wrapping_add(my_bits_high &
                                                              0x70000 as
                                                                  u32)
                          >> 16) as usize];
    /* Up right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x1020 as
                                    u32).wrapping_add(my_bits_low &
                                                                   0x40800000
                                                                       as
                                                                       u32).wrapping_add((my_bits_high
                                                                                                       &
                                                                                                       0x402
                                                                                                           as
                                                                                                           u32).wrapping_add(my_bits_low
                                                                                                                                          &
                                                                                                                                          0x1000000
                                                                                                                                              as
                                                                                                                                              u32)
                                                                                                      <<
                                                                                                      1).wrapping_mul(0x1010101
                                                                                                                                        as
                                                                                                                                        u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[(((my_bits_high & 0x808 as u32) <<
                              4).wrapping_add(my_bits_low &
                                                                0x8080808 as
                                                                    u32).wrapping_mul(0x408102
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_e7(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xef0000 as
                               u32).wrapping_add(my_bits_high &
                                                              0xf0000 as
                                                                  u32)
                          >> 17) as usize];
    /* Up right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x2040 as
                                    u32).wrapping_add(my_bits_low &
                                                                   0x80000000
                                                                       as
                                                                       u32).wrapping_add((my_bits_high
                                                                                                       &
                                                                                                       0x804
                                                                                                           as
                                                                                                           u32).wrapping_add(my_bits_low
                                                                                                                                          &
                                                                                                                                          0x2010000
                                                                                                                                              as
                                                                                                                                              u32)
                                                                                                      <<
                                                                                                      1).wrapping_mul(0x1010101
                                                                                                                                        as
                                                                                                                                        u32)
                               >> 25) as usize] as i32)
            as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x1010 as
                                  u32).wrapping_add((my_bits_low &
                                                                  0x10101010
                                                                      as
                                                                      u32)
                                                                 >>
                                                                 4).wrapping_mul(0x2040810
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_c3(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down */
    flipped =
        right_count[((my_bits_low &
                          0x4000000 as
                              u32).wrapping_add((my_bits_high &
                                                              0x4040404 as
                                                                  u32)
                                                             <<
                                                             4).wrapping_mul(0x408102
                                                                                               as
                                                                                               u32)
                         >> 27) as usize];
    /* Down right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x8000000 as
                                   u32).wrapping_add(my_bits_high &
                                                                  0x80402010
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 27) as usize] as i32)
            as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             right_count[(my_bits_low >> 18 + 1
                              & 0x1f as i32 as u32) as usize]
                 as i32) as i8;
    /* Up right */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x810 as u32 == 0x10 as u32) as
                 i32 as i8 as i32) as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x404 as u32 == 0x4 as u32) as
                 i32 as i8 as i32) as i8;
    /* Up left */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x201 as u32 == 0x1 as u32) as
                 i32 as i8 as i32) as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x30000 as u32 ==
                  0x10000 as u32) as i32 as i8 as
                 i32) as i8;
    /* Down left */
    flipped =
        (flipped as
             u32).wrapping_add(!my_bits_low >> 25 &
                                            my_bits_high >> 0 &
                                            1)
            as i8 as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_f3(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down */
    flipped =
        right_count[((my_bits_high &
                          0x20202020 as
                              u32).wrapping_add((my_bits_low &
                                                              0x20000000 as
                                                                  u32)
                                                             >>
                                                             4).wrapping_mul(0x810204
                                                                                               as
                                                                                               u32)
                         >> 27) as usize];
    /* Down left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_low &
                              0x10000000 as
                                  u32).wrapping_add(my_bits_high &
                                                                 0x1020408 as
                                                                     u32).wrapping_mul(0x4040404
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_low >> 21 - 7 &
                             0x7c as i32 as u32) as usize] as
                 i32) as i8;
    /* Up left */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x1008 as u32 == 0x8 as u32) as
                 i32 as i8 as i32) as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x2020 as u32 == 0x20 as u32) as
                 i32 as i8 as i32) as i8;
    /* Up right */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x4080 as u32 == 0x80 as u32) as
                 i32 as i8 as i32) as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0xc00000 as u32 ==
                  0x800000 as u32) as i32 as i8 as
                 i32) as i8;
    /* Down right */
    flipped =
        (flipped as
             u32).wrapping_add(!my_bits_low >> 30 &
                                            my_bits_high >> 7 &
                                            1)
            as i8 as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_c6(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Up */
    flipped =
        left_count[((my_bits_low &
                         0x4040404 as
                             u32).wrapping_add((my_bits_high &
                                                             0x4 as
                                                                 u32)
                                                            <<
                                                            4).wrapping_mul(0x1020408
                                                                                              as
                                                                                              u32)
                        >> 24) as usize];
    /* Up right */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_high &
                               0x8 as
                                   u32).wrapping_add(my_bits_low &
                                                                  0x10204080
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                              >> 27) as usize] as i32)
            as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             right_count[(my_bits_high >> 10 + 1
                              & 0x1f as i32 as u32) as usize]
                 as i32) as i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x10080000 as u32 ==
                  0x10000000 as u32) as i32 as i8
                 as i32) as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x4040000 as u32 ==
                  0x4000000 as u32) as i32 as i8 as
                 i32) as i8;
    /* Down left */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x1020000 as u32 ==
                  0x1000000 as u32) as i32 as i8 as
                 i32) as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x300 as u32 == 0x100 as u32)
                 as i32 as i8 as i32) as
            i8;
    /* Up left */
    flipped =
        (flipped as
             u32).wrapping_add(!my_bits_high >> 1 &
                                            my_bits_low >> 24 &
                                            1)
            as i8 as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_f6(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Up */
    flipped =
        left_count[((my_bits_high &
                         0x20 as
                             u32).wrapping_add((my_bits_low &
                                                             0x20202020 as
                                                                 u32)
                                                            >>
                                                            4).wrapping_mul(0x2040810
                                                                                              as
                                                                                              u32)
                        >> 24) as usize];
    /* Up left */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x10 as
                                  u32).wrapping_add(my_bits_low &
                                                                 0x8040201 as
                                                                     u32).wrapping_mul(0x4040404
                                                                                                    as
                                                                                                    u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_high >> 13 - 7
                             & 0x7c as i32 as u32) as usize]
                 as i32) as i8;
    /* Down left */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x8100000 as u32 ==
                  0x8000000 as u32) as i32 as i8 as
                 i32) as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x20200000 as u32 ==
                  0x20000000 as u32) as i32 as i8
                 as i32) as i8;
    /* Down right */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x80400000 as u32 ==
                  0x80000000 as u32) as i32 as i8
                 as i32) as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0xc000 as u32 == 0x8000 as u32)
                 as i32 as i8 as i32) as
            i8;
    /* Up right */
    flipped =
        (flipped as
             u32).wrapping_add(!my_bits_high >> 6 &
                                            my_bits_low >> 31 &
                                            1)
            as i8 as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_d3(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xf70000 as
                               u32).wrapping_add(my_bits_low &
                                                              0x70000 as
                                                                  u32)
                          >> 16) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_low &
                                0x4001020 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x102 as
                                                                        u32)
                                                                   <<
                                                                   1).wrapping_mul(0x1010102
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[((my_bits_low &
                               0x8000000 as
                                   u32).wrapping_add((my_bits_high &
                                                                   0x8080808
                                                                       as
                                                                       u32)
                                                                  <<
                                                                  4).wrapping_mul(0x204081
                                                                                                    as
                                                                                                    u32)
                              >> 27) as usize] as i32)
            as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x808 as u32 == 0x8 as u32) as
                 i32 as i8 as i32) as i8;
    /* Down right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_low &
                                0x10000402 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x804020
                                                                        as
                                                                        u32)
                                                                   >>
                                                                   1).wrapping_mul(0x2020201
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_e3(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xef0000 as
                               u32).wrapping_add(my_bits_low &
                                                              0xf0000 as
                                                                  u32)
                          >> 17) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_low &
                                0x8002040 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x10204 as
                                                                        u32)
                                                                   <<
                                                                   1).wrapping_mul(0x1010102
                                                                                                     as
                                                                                                     u32)
                               >> 25) as usize] as i32)
            as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             right_count[(((my_bits_low & 0x10000000 as u32) >>
                               4).wrapping_add(my_bits_high &
                                                                 0x10101010 as
                                                                     u32).wrapping_mul(0x1020408
                                                                                                    as
                                                                                                    u32)
                              >> 27) as usize] as i32)
            as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x1010 as u32 == 0x10 as u32) as
                 i32 as i8 as i32) as i8;
    /* Down right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_low &
                                0x20000804 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x8040 as
                                                                        u32)
                                                                   >>
                                                                   1).wrapping_mul(0x2020201
                                                                                                     as
                                                                                                     u32)
                               >> 25) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_c4(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 26 + 1) as
                        usize];
    /* Left */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0x3000000 as u32 ==
                  0x1000000 as u32) as i32 as i8 as
                 i32) as i8;
    /* Down left / Up right */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_low &
                                0x81020 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x102 as
                                                                        u32)
                                                                   <<
                                                                   1).wrapping_mul(0x2020202
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down / Up */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_low &
                                0x40404 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x4040404
                                                                        as
                                                                        u32)
                                                                   <<
                                                                   3).wrapping_mul(0x810204
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x40201008 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x20100 as
                                                                        u32)
                                                                   <<
                                                                   1).wrapping_mul(0x2020202
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_f4(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down left / Up right */
    flipped =
        center_count[((my_bits_low &
                           0x408000 as
                               u32).wrapping_add((my_bits_high &
                                                               0x2040810 as
                                                                   u32)
                                                              <<
                                                              1).wrapping_mul(0x1010101
                                                                                                as
                                                                                                u32)
                          >> 26) as usize];
    /* Down / Up */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x20202020 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x202020
                                                                        as
                                                                        u32)
                                                                   >>
                                                                   3).wrapping_mul(0x810204
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x8040 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x100804
                                                                        as
                                                                        u32)
                                                                   <<
                                                                   1).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                               >> 26) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_low >> 29 - 7 &
                             0x7c as i32 as u32) as usize] as
                 i32) as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             (my_bits_low & 0xc0000000 as u32 ==
                  0x80000000 as u32) as i32 as i8
                 as i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_c5(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 2 + 1 &
                         0x1f as i32 as u32) as usize];
    /* Left */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x3 as u32 == 0x1 as u32) as
                 i32 as i8 as i32) as i8;
    /* Down left / Up right */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_low &
                                0x8102040 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x10200 as
                                                                        u32)
                                                                   <<
                                                                   1).wrapping_mul(0x2020202
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down / Up */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_low &
                                0x4040404 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x4040400
                                                                        as
                                                                        u32)
                                                                   <<
                                                                   3).wrapping_mul(0x810204
                                                                                                     as
                                                                                                     u32)
                               >> 25) as usize] as i32)
            as i8;
    /* Down right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x20100800 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x2010000
                                                                        as
                                                                        u32)
                                                                   <<
                                                                   1).wrapping_mul(0x2020202
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_f5(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Down left / Up right */
    flipped =
        center_count[((my_bits_low &
                           0x40800000 as
                               u32).wrapping_add((my_bits_high &
                                                               0x4081000 as
                                                                   u32)
                                                              <<
                                                              1).wrapping_mul(0x1010101
                                                                                                as
                                                                                                u32)
                          >> 26) as usize];
    /* Down / Up */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x20202000 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x20202020
                                                                        as
                                                                        u32)
                                                                   >>
                                                                   3).wrapping_mul(0x810204
                                                                                                     as
                                                                                                     u32)
                               >> 25) as usize] as i32)
            as i8;
    /* Down right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x804000 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x10080402
                                                                        as
                                                                        u32)
                                                                   <<
                                                                   1).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                               >> 26) as usize] as i32)
            as i8;
    /* Left */
    flipped =
        (flipped as i32 +
             left_count[(my_bits_high << 7 - 5 &
                             0x7c as i32 as u32) as usize] as
                 i32) as i8;
    /* Right */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0xc0 as u32 == 0x80 as u32) as
                 i32 as i8 as i32) as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_d6(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xf700 as
                               u32).wrapping_add(my_bits_high &
                                                              0x700 as
                                                                  u32)
                          >> 8) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x2040010 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x20408000
                                                                        as
                                                                        u32)
                                                                   >>
                                                                   1).wrapping_mul(0x1020202
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x8080000 as u32 ==
                  0x8000000 as u32) as i32 as i8 as
                 i32) as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[(((my_bits_high & 0x8 as u32) <<
                              4).wrapping_add(my_bits_low &
                                                                0x8080808 as
                                                                    u32).wrapping_mul(0x810204
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Down right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x20100004 as i32 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x2010000
                                                                        as
                                                                        u32)
                                                                   <<
                                                                   1).wrapping_mul(0x2010101
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_e6(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xef00 as
                               u32).wrapping_add(my_bits_high &
                                                              0xf00 as
                                                                  u32)
                          >> 9) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x4080020 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x40800000
                                                                        as
                                                                        u32)
                                                                   >>
                                                                   1).wrapping_mul(0x1020202
                                                                                                     as
                                                                                                     u32)
                               >> 25) as usize] as i32)
            as i8;
    /* Down */
    flipped =
        (flipped as i32 +
             (my_bits_high & 0x10100000 as u32 ==
                  0x10000000 as u32) as i32 as i8
                 as i32) as i8;
    /* Up */
    flipped =
        (flipped as i32 +
             left_count[((my_bits_high &
                              0x10 as
                                  u32).wrapping_add((my_bits_low &
                                                                  0x10101010
                                                                      as
                                                                      u32)
                                                                 >>
                                                                 4).wrapping_mul(0x4081020
                                                                                                   as
                                                                                                   u32)
                             >> 24) as usize] as i32)
            as i8;
    /* Down right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x40200008 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x4020100
                                                                        as
                                                                        u32)
                                                                   <<
                                                                   1).wrapping_mul(0x2010101
                                                                                                     as
                                                                                                     u32)
                               >> 25) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_d4(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xf7000000 as
                               u32).wrapping_add(my_bits_low &
                                                              0x7000000 as
                                                                  u32)
                          >> 24) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as i32 +
             center_count[(((my_bits_high & 0x10204 as u32) <<
                                1).wrapping_add(my_bits_low &
                                                                  0x102040 as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down / Up */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_low &
                                0x80808 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x8080808
                                                                        as
                                                                        u32)
                                                                   <<
                                                                   3).wrapping_mul(0x408102
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x80402010 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x40201 as
                                                                        u32)
                                                                   <<
                                                                   1).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_e4(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xef000000 as
                               u32).wrapping_add(my_bits_low &
                                                              0xf000000 as
                                                                  u32)
                          >> 25) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x1020408 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x204080
                                                                        as
                                                                        u32)
                                                                   >>
                                                                   1).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down / Up */
    flipped =
        (flipped as i32 +
             center_count[(((my_bits_low & 0x101010 as u32) >>
                                3).wrapping_add(my_bits_high &
                                                                  0x10101010
                                                                      as
                                                                      u32).wrapping_mul(0x1020408
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down right / Up left */
    flipped =
        (flipped as i32 +
             center_count[(((my_bits_high & 0x804020 as u32) >>
                                1).wrapping_add(my_bits_low &
                                                                  0x80402 as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_d5(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xf7 as
                               u32).wrapping_add(my_bits_high &
                                                              0x7 as
                                                                  u32)
                          >> 0) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as i32 +
             center_count[(((my_bits_high & 0x1020400 as u32) <<
                                1).wrapping_add(my_bits_low &
                                                                  0x10204080
                                                                      as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down / Up */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_low &
                                0x8080808 as
                                    u32).wrapping_add((my_bits_high &
                                                                    0x8080800
                                                                        as
                                                                        u32)
                                                                   <<
                                                                   3).wrapping_mul(0x408102
                                                                                                     as
                                                                                                     u32)
                               >> 25) as usize] as i32)
            as i8;
    /* Down right / Up left */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x40201000 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x4020100
                                                                        as
                                                                        u32)
                                                                   <<
                                                                   1).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}
fn CountFlips_bitboard_e5(my_bits_high: u32,
                                            my_bits_low: u32)
 -> i32 {
    let mut flipped: i8 = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xef as
                               u32).wrapping_add(my_bits_high &
                                                              0xf as
                                                                  u32)
                          >> 1) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x2040800 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x20408000
                                                                        as
                                                                        u32)
                                                                   >>
                                                                   1).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    /* Down / Up */
    flipped =
        (flipped as i32 +
             center_count[((my_bits_high &
                                0x10101000 as
                                    u32).wrapping_add((my_bits_low &
                                                                    0x10101010
                                                                        as
                                                                        u32)
                                                                   >>
                                                                   3).wrapping_mul(0x1020408
                                                                                                     as
                                                                                                     u32)
                               >> 25) as usize] as i32)
            as i8;
    /* Down right / Up left */
    flipped =
        (flipped as i32 +
             center_count[(((my_bits_high & 0x80402000 as u32) >>
                                1).wrapping_add(my_bits_low &
                                                                  0x8040201 as
                                                                      u32).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     u32)
                               >> 24) as usize] as i32)
            as i8;
    return flipped as i32;
}

fn null(_: u32, _: u32) -> i32 {
 // perf??
 unreachable!()
}
pub static CountFlips_bitboard: [fn(_: u32, _: u32) -> i32; 78] =
 [(CountFlips_bitboard_a1),
  (CountFlips_bitboard_b1 ),
  (CountFlips_bitboard_c1 ),
  (CountFlips_bitboard_d1 ),
  (CountFlips_bitboard_e1 ),
  (CountFlips_bitboard_f1 ),
  (CountFlips_bitboard_g1 ),
  (CountFlips_bitboard_h1 ), null, null,
  (CountFlips_bitboard_a2 ),
  (CountFlips_bitboard_b2 ),
  (CountFlips_bitboard_c2 ),
  (CountFlips_bitboard_d2 ),
  (CountFlips_bitboard_e2 ),
  (CountFlips_bitboard_f2 ),
  (CountFlips_bitboard_g2 ),
  (CountFlips_bitboard_h2 ), null, null,
  (CountFlips_bitboard_a3 ),
  (CountFlips_bitboard_b3 ),
  (CountFlips_bitboard_c3 ),
  (CountFlips_bitboard_d3 ),
  (CountFlips_bitboard_e3 ),
  (CountFlips_bitboard_f3 ),
  (CountFlips_bitboard_g3 ),
  (CountFlips_bitboard_h3 ), null, null,
  (CountFlips_bitboard_a4 ),
  (CountFlips_bitboard_b4 ),
  (CountFlips_bitboard_c4 ),
  (CountFlips_bitboard_d4 ),
  (CountFlips_bitboard_e4 ),
  (CountFlips_bitboard_f4 ),
  (CountFlips_bitboard_g4 ),
  (CountFlips_bitboard_h4 ), null, null,
  (CountFlips_bitboard_a5 ),
  (CountFlips_bitboard_b5 ),
  (CountFlips_bitboard_c5 ),
  (CountFlips_bitboard_d5 ),
  (CountFlips_bitboard_e5 ),
  (CountFlips_bitboard_f5 ),
  (CountFlips_bitboard_g5 ),
  (CountFlips_bitboard_h5 ), null, null,
  (CountFlips_bitboard_a6 ),
  (CountFlips_bitboard_b6 ),
  (CountFlips_bitboard_c6 ),
  (CountFlips_bitboard_d6 ),
  (CountFlips_bitboard_e6 ),
  (CountFlips_bitboard_f6 ),
  (CountFlips_bitboard_g6 ),
  (CountFlips_bitboard_h6 ), null, null,
  (CountFlips_bitboard_a7 ),
  (CountFlips_bitboard_b7 ),
  (CountFlips_bitboard_c7 ),
  (CountFlips_bitboard_d7 ),
  (CountFlips_bitboard_e7 ),
  (CountFlips_bitboard_f7 ),
  (CountFlips_bitboard_g7 ),
  (CountFlips_bitboard_h7 ), null, null,
  (CountFlips_bitboard_a8 ),
  (CountFlips_bitboard_b8 ),
  (CountFlips_bitboard_c8 ),
  (CountFlips_bitboard_d8 ),
  (CountFlips_bitboard_e8 ),
  (CountFlips_bitboard_f8 ),
  (CountFlips_bitboard_g8 ),
  (CountFlips_bitboard_h8 )];
