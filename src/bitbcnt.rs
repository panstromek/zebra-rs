
/*
   File:          bitbcnt.c

   Modified:      November 24, 2005

   Authors:       Gunnar Andersson (gunnar@radagast.se)
          Toshihiko Okuhara

   Contents:      Count flips for the last move.

   This piece of software is released under the GPL.
   See the file COPYING for more information.
*/
use crate::src::libc;

// NULL
// #include "bitboard.h"
static mut right_count: [libc::c_char; 128] =
    [0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     5 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     6 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     5 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char];
static mut left_count: [libc::c_char; 128] =
    [0 as libc::c_int as libc::c_char, 6 as libc::c_int as libc::c_char,
     5 as libc::c_int as libc::c_char, 5 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char];
static mut center_count: [libc::c_char; 256] =
    [0 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 5 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 6 as libc::c_int as libc::c_char,
     5 as libc::c_int as libc::c_char, 5 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 5 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char];
unsafe fn CountFlips_bitboard_a1(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 0 as libc::c_int + 1 as libc::c_int &
                         0x7f as libc::c_int as libc::c_uint) as usize];
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x1010100 as
                                   libc::c_uint).wrapping_add((my_bits_high &
                                                                   0x1010101
                                                                       as
                                                                       libc::c_uint)
                                                                  <<
                                                                  4 as
                                                                      libc::c_int).wrapping_mul(0x1020408
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x8040200 as
                                   libc::c_uint).wrapping_add(my_bits_high &
                                                                  0x80402010
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_h1(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Left */
    flipped =
        left_count[(my_bits_low << 7 as libc::c_int - 7 as libc::c_int &
                        0x7f as libc::c_int as libc::c_uint) as usize];
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_low &
                              0x10204000 as
                                  libc::c_uint).wrapping_add(my_bits_high &
                                                                 0x1020408 as
                                                                     libc::c_uint).wrapping_mul(0x1010101
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[(((my_bits_low & 0x80808000 as libc::c_uint) >>
                               4 as
                                   libc::c_int).wrapping_add(my_bits_high &
                                                                 0x80808080 as
                                                                     libc::c_uint).wrapping_mul(0x204081
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_a8(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 24 as libc::c_int + 1 as libc::c_int) as
                        usize];
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_high &
                               0x20408 as
                                   libc::c_uint).wrapping_add(my_bits_low &
                                                                  0x10204080
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[(((my_bits_high & 0x10101 as libc::c_uint) <<
                              4 as
                                  libc::c_int).wrapping_add(my_bits_low &
                                                                0x1010101 as
                                                                    libc::c_uint).wrapping_mul(0x1020408
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_h8(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Left */
    flipped =
        left_count[(my_bits_high >> 31 as libc::c_int - 7 as libc::c_int &
                        0x7f as libc::c_int as libc::c_uint) as usize];
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x808080 as
                                  libc::c_uint).wrapping_add((my_bits_low &
                                                                  0x80808080
                                                                      as
                                                                      libc::c_uint)
                                                                 >>
                                                                 4 as
                                                                     libc::c_int).wrapping_mul(0x204081
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x402010 as
                                  libc::c_uint).wrapping_add(my_bits_low &
                                                                 0x8040201 as
                                                                     libc::c_uint).wrapping_mul(0x1010101
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_b1(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 1 as libc::c_int + 1 as libc::c_int &
                         0x3f as libc::c_int as libc::c_uint) as usize];
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x2020200 as
                                   libc::c_uint).wrapping_add((my_bits_high &
                                                                   0x2020202
                                                                       as
                                                                       libc::c_uint)
                                                                  <<
                                                                  4 as
                                                                      libc::c_int).wrapping_mul(0x810204
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x10080400 as
                                   libc::c_uint).wrapping_add(my_bits_high &
                                                                  0x804020 as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_g1(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Left */
    flipped =
        left_count[(my_bits_low << 7 as libc::c_int - 6 as libc::c_int &
                        0x7e as libc::c_int as libc::c_uint) as usize];
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_low &
                              0x8102000 as
                                  libc::c_uint).wrapping_add(my_bits_high &
                                                                 0x10204 as
                                                                     libc::c_uint).wrapping_mul(0x2020202
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[(((my_bits_low & 0x40404000 as libc::c_uint) >>
                               4 as
                                   libc::c_int).wrapping_add(my_bits_high &
                                                                 0x40404040 as
                                                                     libc::c_uint).wrapping_mul(0x408102
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_a2(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 8 as libc::c_int + 1 as libc::c_int &
                         0x7f as libc::c_int as libc::c_uint) as usize];
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x1010000 as
                                   libc::c_uint).wrapping_add((my_bits_high &
                                                                   0x1010101
                                                                       as
                                                                       libc::c_uint)
                                                                  <<
                                                                  4 as
                                                                      libc::c_int).wrapping_mul(0x1020408
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x4020000 as
                                   libc::c_uint).wrapping_add(my_bits_high &
                                                                  0x40201008
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_h2(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Left */
    flipped =
        left_count[(my_bits_low >> 15 as libc::c_int - 7 as libc::c_int &
                        0x7f as libc::c_int as libc::c_uint) as usize];
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_low &
                              0x20400000 as
                                  libc::c_uint).wrapping_add(my_bits_high &
                                                                 0x2040810 as
                                                                     libc::c_uint).wrapping_mul(0x1010101
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[(((my_bits_low & 0x80800000 as libc::c_uint) >>
                               4 as
                                   libc::c_int).wrapping_add(my_bits_high &
                                                                 0x80808080 as
                                                                     libc::c_uint).wrapping_mul(0x204081
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_a7(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 16 as libc::c_int + 1 as libc::c_int &
                         0x7f as libc::c_int as libc::c_uint) as usize];
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_high &
                               0x204 as
                                   libc::c_uint).wrapping_add(my_bits_low &
                                                                  0x8102040 as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[(((my_bits_high & 0x101 as libc::c_uint) <<
                              4 as
                                  libc::c_int).wrapping_add(my_bits_low &
                                                                0x1010101 as
                                                                    libc::c_uint).wrapping_mul(0x2040810
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_h7(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Left */
    flipped =
        left_count[(my_bits_high >> 23 as libc::c_int - 7 as libc::c_int &
                        0x7f as libc::c_int as libc::c_uint) as usize];
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x8080 as
                                  libc::c_uint).wrapping_add((my_bits_low &
                                                                  0x80808080
                                                                      as
                                                                      libc::c_uint)
                                                                 >>
                                                                 4 as
                                                                     libc::c_int).wrapping_mul(0x408102
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x4020 as
                                  libc::c_uint).wrapping_add(my_bits_low &
                                                                 0x10080402 as
                                                                     libc::c_uint).wrapping_mul(0x1010101
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_b8(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 25 as libc::c_int + 1 as libc::c_int) as
                        usize];
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_high &
                               0x40810 as
                                   libc::c_uint).wrapping_add(my_bits_low &
                                                                  0x20408000
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[(((my_bits_high & 0x20202 as libc::c_uint) <<
                              4 as
                                  libc::c_int).wrapping_add(my_bits_low &
                                                                0x2020202 as
                                                                    libc::c_uint).wrapping_mul(0x810204
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_g8(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Left */
    flipped =
        left_count[(my_bits_high >> 30 as libc::c_int - 7 as libc::c_int &
                        0x7e as libc::c_int as libc::c_uint) as usize];
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x404040 as
                                  libc::c_uint).wrapping_add((my_bits_low &
                                                                  0x40404040
                                                                      as
                                                                      libc::c_uint)
                                                                 >>
                                                                 4 as
                                                                     libc::c_int).wrapping_mul(0x408102
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x201008 as
                                  libc::c_uint).wrapping_add(my_bits_low &
                                                                 0x4020100 as
                                                                     libc::c_uint).wrapping_mul(0x2020202
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_b2(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 9 as libc::c_int + 1 as libc::c_int &
                         0x3f as libc::c_int as libc::c_uint) as usize];
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x2020000 as
                                   libc::c_uint).wrapping_add((my_bits_high &
                                                                   0x2020202
                                                                       as
                                                                       libc::c_uint)
                                                                  <<
                                                                  4 as
                                                                      libc::c_int).wrapping_mul(0x810204
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x8040000 as
                                   libc::c_uint).wrapping_add(my_bits_high &
                                                                  0x80402010
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_g2(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Left */
    flipped =
        left_count[(my_bits_low >> 14 as libc::c_int - 7 as libc::c_int &
                        0x7e as libc::c_int as libc::c_uint) as usize];
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_low &
                              0x10200000 as
                                  libc::c_uint).wrapping_add(my_bits_high &
                                                                 0x1020408 as
                                                                     libc::c_uint).wrapping_mul(0x2020202
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[(((my_bits_low & 0x40400000 as libc::c_uint) >>
                               4 as
                                   libc::c_int).wrapping_add(my_bits_high &
                                                                 0x40404040 as
                                                                     libc::c_uint).wrapping_mul(0x408102
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_b7(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 17 as libc::c_int + 1 as libc::c_int &
                         0x3f as libc::c_int as libc::c_uint) as usize];
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_high &
                               0x408 as
                                   libc::c_uint).wrapping_add(my_bits_low &
                                                                  0x10204080
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[(((my_bits_high & 0x202 as libc::c_uint) <<
                              4 as
                                  libc::c_int).wrapping_add(my_bits_low &
                                                                0x2020202 as
                                                                    libc::c_uint).wrapping_mul(0x1020408
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_g7(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Left */
    flipped =
        left_count[(my_bits_high >> 22 as libc::c_int - 7 as libc::c_int &
                        0x7e as libc::c_int as libc::c_uint) as usize];
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x4040 as
                                  libc::c_uint).wrapping_add((my_bits_low &
                                                                  0x40404040
                                                                      as
                                                                      libc::c_uint)
                                                                 >>
                                                                 4 as
                                                                     libc::c_int).wrapping_mul(0x810204
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x2010 as
                                  libc::c_uint).wrapping_add(my_bits_low &
                                                                 0x8040201 as
                                                                     libc::c_uint).wrapping_mul(0x2020202
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_c1(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down */
    flipped =
        right_count[((my_bits_low &
                          0x4040400 as
                              libc::c_uint).wrapping_add((my_bits_high &
                                                              0x4040404 as
                                                                  libc::c_uint)
                                                             <<
                                                             4 as
                                                                 libc::c_int).wrapping_mul(0x408102
                                                                                               as
                                                                                               libc::c_uint)
                         >> 25 as libc::c_int) as usize];
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x20100800 as
                                   libc::c_uint).wrapping_add(my_bits_high &
                                                                  0x8040 as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 27 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             right_count[(my_bits_low >> 2 as libc::c_int + 1 as libc::c_int &
                              0x1f as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x3 as libc::c_uint == 0x1 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x10200 as libc::c_uint ==
                  0x10000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_f1(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down */
    flipped =
        right_count[((my_bits_high &
                          0x20202020 as
                              libc::c_uint).wrapping_add(my_bits_low >>
                                                             4 as libc::c_int
                                                             &
                                                             0x2020200 as
                                                                 libc::c_uint).wrapping_mul(0x810204
                                                                                                as
                                                                                                libc::c_uint)
                         >> 25 as libc::c_int) as usize];
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_low &
                              0x4081000 as
                                  libc::c_uint).wrapping_add(my_bits_high &
                                                                 0x102 as
                                                                     libc::c_uint).wrapping_mul(0x4040404
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_low << 7 as libc::c_int - 5 as libc::c_int &
                             0x7c as libc::c_int as libc::c_uint) as usize] as
                 libc::c_int) as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0xc0 as libc::c_uint == 0x80 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x804000 as libc::c_uint ==
                  0x800000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_a3(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down */
    flipped =
        right_count[((my_bits_low &
                          0x1000000 as
                              libc::c_uint).wrapping_add((my_bits_high &
                                                              0x1010101 as
                                                                  libc::c_uint)
                                                             <<
                                                             4 as
                                                                 libc::c_int).wrapping_mul(0x1020408
                                                                                               as
                                                                                               libc::c_uint)
                         >> 27 as libc::c_int) as usize];
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x2000000 as
                                   libc::c_uint).wrapping_add(my_bits_high &
                                                                  0x20100804
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             right_count[(my_bits_low >> 16 as libc::c_int + 1 as libc::c_int
                              & 0x7f as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x204 as libc::c_uint == 0x4 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x101 as libc::c_uint == 0x1 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_h3(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down */
    flipped =
        right_count[((my_bits_high &
                          0x80808080 as
                              libc::c_uint).wrapping_add((my_bits_low &
                                                              0x80000000 as
                                                                  libc::c_uint)
                                                             >>
                                                             4 as
                                                                 libc::c_int).wrapping_mul(0x204081
                                                                                               as
                                                                                               libc::c_uint)
                         >> 27 as libc::c_int) as usize];
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_low &
                              0x40000000 as
                                  libc::c_uint).wrapping_add(my_bits_high &
                                                                 0x4081020 as
                                                                     libc::c_uint).wrapping_mul(0x1010101
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_low >> 23 as libc::c_int - 7 as libc::c_int &
                             0x7f as libc::c_int as libc::c_uint) as usize] as
                 libc::c_int) as libc::c_char;
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x4020 as libc::c_uint == 0x20 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x8080 as libc::c_uint == 0x80 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_a6(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Up */
    flipped =
        left_count[((my_bits_low &
                         0x1010101 as
                             libc::c_uint).wrapping_add((my_bits_high &
                                                             0x1 as
                                                                 libc::c_uint)
                                                            <<
                                                            4 as
                                                                libc::c_int).wrapping_mul(0x4081020
                                                                                              as
                                                                                              libc::c_uint)
                        >> 24 as libc::c_int) as usize];
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_high &
                               0x2 as
                                   libc::c_uint).wrapping_add(my_bits_low &
                                                                  0x4081020 as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             right_count[(my_bits_high >> 8 as libc::c_int + 1 as libc::c_int
                              & 0x7f as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x4020000 as libc::c_uint ==
                  0x4000000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x1010000 as libc::c_uint ==
                  0x1000000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_h6(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Up */
    flipped =
        left_count[((my_bits_high &
                         0x80 as
                             libc::c_uint).wrapping_add((my_bits_low &
                                                             0x80808080 as
                                                                 libc::c_uint)
                                                            >>
                                                            4 as
                                                                libc::c_int).wrapping_mul(0x810204
                                                                                              as
                                                                                              libc::c_uint)
                        >> 24 as libc::c_int) as usize];
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x40 as
                                  libc::c_uint).wrapping_add(my_bits_low &
                                                                 0x20100804 as
                                                                     libc::c_uint).wrapping_mul(0x1010101
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_high >> 15 as libc::c_int - 7 as libc::c_int
                             & 0x7f as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x20400000 as libc::c_uint ==
                  0x20000000 as libc::c_uint) as libc::c_int as libc::c_char
                 as libc::c_int) as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x80800000 as libc::c_uint ==
                  0x80000000 as libc::c_uint) as libc::c_int as libc::c_char
                 as libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_c8(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Up */
    flipped =
        left_count[((my_bits_low &
                         0x4040404 as
                             libc::c_uint).wrapping_add((my_bits_high &
                                                             0x40404 as
                                                                 libc::c_uint)
                                                            <<
                                                            4 as
                                                                libc::c_int).wrapping_mul(0x408102
                                                                                              as
                                                                                              libc::c_uint)
                        >> 24 as libc::c_int) as usize];
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_high &
                               0x81020 as
                                   libc::c_uint).wrapping_add(my_bits_low &
                                                                  0x40800000
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 27 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             right_count[(my_bits_high >>
                              26 as libc::c_int + 1 as libc::c_int) as usize]
                 as libc::c_int) as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x3000000 as libc::c_uint ==
                  0x1000000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x20100 as libc::c_uint == 0x100 as libc::c_uint)
                 as libc::c_int as libc::c_char as libc::c_int) as
            libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_f8(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Up */
    flipped =
        left_count[((my_bits_high &
                         0x202020 as
                             libc::c_uint).wrapping_add((my_bits_low &
                                                             0x20202020 as
                                                                 libc::c_uint)
                                                            >>
                                                            4 as
                                                                libc::c_int).wrapping_mul(0x810204
                                                                                              as
                                                                                              libc::c_uint)
                        >> 24 as libc::c_int) as usize];
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x100804 as
                                  libc::c_uint).wrapping_add(my_bits_low &
                                                                 0x2010000 as
                                                                     libc::c_uint).wrapping_mul(0x4040404
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_high >> 29 as libc::c_int - 7 as libc::c_int
                             & 0x7c as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0xc0000000 as libc::c_uint ==
                  0x80000000 as libc::c_uint) as libc::c_int as libc::c_char
                 as libc::c_int) as libc::c_char;
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x408000 as libc::c_uint ==
                  0x8000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_d1(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xf7 as
                               libc::c_uint).wrapping_add(my_bits_low &
                                                              0x7 as
                                                                  libc::c_uint)
                          >> 0 as libc::c_int) as usize];
    /* Down left / Down right */
    flipped =
        (flipped as libc::c_int +
             center_count[(((my_bits_low &
                                 0x1020400 as libc::c_int as libc::c_uint) <<
                                1 as
                                    libc::c_int).wrapping_add(my_bits_low &
                                                                  0x40201000
                                                                      as
                                                                      libc::c_uint).wrapping_add(my_bits_high
                                                                                                     &
                                                                                                     0x80
                                                                                                         as
                                                                                                         libc::c_uint).wrapping_mul(0x1010101
                                                                                                                                        as
                                                                                                                                        libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x8080800 as
                                   libc::c_uint).wrapping_add((my_bits_high &
                                                                   0x8080808
                                                                       as
                                                                       libc::c_uint)
                                                                  <<
                                                                  4 as
                                                                      libc::c_int).wrapping_mul(0x204081
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_e1(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xef as
                               libc::c_uint).wrapping_add(my_bits_low &
                                                              0xf as
                                                                  libc::c_uint)
                          >> 1 as libc::c_int) as usize];
    /* Down left / Down right */
    flipped =
        (flipped as libc::c_int +
             center_count[(((my_bits_low &
                                 0x2040800 as
                                     libc::c_uint).wrapping_add(my_bits_high &
                                                                    0x1 as
                                                                        libc::c_uint)
                                <<
                                1 as
                                    libc::c_int).wrapping_add(my_bits_low &
                                                                  0x80402000
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[(((my_bits_low & 0x10101000 as libc::c_uint) >>
                               4 as
                                   libc::c_int).wrapping_add(my_bits_high &
                                                                 0x10101010 as
                                                                     libc::c_uint).wrapping_mul(0x1020408
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_a4(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 24 as libc::c_int + 1 as libc::c_int) as
                        usize];
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x20408 as
                                   libc::c_uint).wrapping_mul(0x1010100 as
                                                                  libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down / Up */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_low &
                                0x10101 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x1010101
                                                                        as
                                                                        libc::c_uint)
                                                                   <<
                                                                   3 as
                                                                       libc::c_int).wrapping_mul(0x2040810
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_high &
                               0x10080402 as
                                   libc::c_uint).wrapping_mul(0x1010101 as
                                                                  libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_h4(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down / Up */
    flipped =
        center_count[((my_bits_high &
                           0x80808080 as
                               libc::c_uint).wrapping_add((my_bits_low &
                                                               0x808080 as
                                                                   libc::c_uint)
                                                              >>
                                                              3 as
                                                                  libc::c_int).wrapping_mul(0x204081
                                                                                                as
                                                                                                libc::c_uint)
                          >> 24 as libc::c_int) as usize];
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x8102040 as
                                  libc::c_uint).wrapping_mul(0x1010101 as
                                                                 libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_low >> 31 as libc::c_int - 7 as libc::c_int &
                             0x7f as libc::c_int as libc::c_uint) as usize] as
                 libc::c_int) as libc::c_char;
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_low &
                              0x402010 as
                                  libc::c_uint).wrapping_mul(0x1010100 as
                                                                 libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_a5(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 0 as libc::c_int + 1 as libc::c_int &
                         0x7f as libc::c_int as libc::c_uint) as usize];
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x2040810 as
                                   libc::c_uint).wrapping_mul(0x1010101 as
                                                                  libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down / Up */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_low &
                                0x1010101 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x1010100
                                                                        as
                                                                        libc::c_uint)
                                                                   <<
                                                                   3 as
                                                                       libc::c_int).wrapping_mul(0x2040810
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_high &
                               0x8040200 as
                                   libc::c_uint).wrapping_mul(0x10101 as
                                                                  libc::c_uint)
                              >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_h5(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down / Up */
    flipped =
        center_count[((my_bits_high &
                           0x80808000 as
                               libc::c_uint).wrapping_add((my_bits_low &
                                                               0x80808080 as
                                                                   libc::c_uint)
                                                              >>
                                                              3 as
                                                                  libc::c_int).wrapping_mul(0x204081
                                                                                                as
                                                                                                libc::c_uint)
                          >> 25 as libc::c_int) as usize];
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_low &
                              0x40201008 as
                                  libc::c_uint).wrapping_mul(0x1010101 as
                                                                 libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_high << 7 as libc::c_int - 7 as libc::c_int &
                             0x7f as libc::c_int as libc::c_uint) as usize] as
                 libc::c_int) as libc::c_char;
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x10204000 as
                                  libc::c_uint).wrapping_mul(0x10101 as
                                                                 libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_d8(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xf7000000 as
                               libc::c_uint).wrapping_add(my_bits_high &
                                                              0x7000000 as
                                                                  libc::c_uint)
                          >> 24 as libc::c_int) as usize];
    /* Up right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x102040 as
                                    libc::c_uint).wrapping_add(my_bits_low &
                                                                   0x80000000
                                                                       as
                                                                       libc::c_uint).wrapping_add((my_bits_high
                                                                                                       &
                                                                                                       0x40201
                                                                                                           as
                                                                                                           libc::c_uint)
                                                                                                      <<
                                                                                                      1
                                                                                                          as
                                                                                                          libc::c_int).wrapping_mul(0x1010101
                                                                                                                                        as
                                                                                                                                        libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[(((my_bits_high & 0x80808 as libc::c_uint) <<
                              4 as
                                  libc::c_int).wrapping_add(my_bits_low &
                                                                0x8080808 as
                                                                    libc::c_uint).wrapping_mul(0x204081
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_e8(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xef000000 as
                               libc::c_uint).wrapping_add(my_bits_high &
                                                              0xf000000 as
                                                                  libc::c_uint)
                          >> 25 as libc::c_int) as usize];
    /* Up right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x204080 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x80402 as
                                                                        libc::c_uint).wrapping_add(my_bits_low
                                                                                                       &
                                                                                                       0x1000000
                                                                                                           as
                                                                                                           libc::c_uint)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x101010 as
                                  libc::c_uint).wrapping_add((my_bits_low &
                                                                  0x10101010
                                                                      as
                                                                      libc::c_uint)
                                                                 >>
                                                                 4 as
                                                                     libc::c_int).wrapping_mul(0x1020408
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_c2(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down */
    flipped =
        right_count[((my_bits_low &
                          0x4040000 as
                              libc::c_uint).wrapping_add((my_bits_high &
                                                              0x4040404 as
                                                                  libc::c_uint)
                                                             <<
                                                             4 as
                                                                 libc::c_int).wrapping_mul(0x408102
                                                                                               as
                                                                                               libc::c_uint)
                         >> 26 as libc::c_int) as usize];
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x10080000 as
                                   libc::c_uint).wrapping_add(my_bits_high &
                                                                  0x804020 as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 27 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             right_count[(my_bits_low >> 10 as libc::c_int + 1 as libc::c_int
                              & 0x1f as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x300 as libc::c_uint == 0x100 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x1020000 as libc::c_uint ==
                  0x1000000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_f2(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down */
    flipped =
        right_count[((my_bits_high &
                          0x20202020 as
                              libc::c_uint).wrapping_add((my_bits_low &
                                                              0x20200000 as
                                                                  libc::c_uint)
                                                             >>
                                                             4 as
                                                                 libc::c_int).wrapping_mul(0x810204
                                                                                               as
                                                                                               libc::c_uint)
                         >> 26 as libc::c_int) as usize];
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_low &
                              0x8100000 as
                                  libc::c_uint).wrapping_add(my_bits_high &
                                                                 0x10204 as
                                                                     libc::c_uint).wrapping_mul(0x4040404
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_low >> 13 as libc::c_int - 7 as libc::c_int &
                             0x7c as libc::c_int as libc::c_uint) as usize] as
                 libc::c_int) as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0xc000 as libc::c_uint == 0x8000 as libc::c_uint)
                 as libc::c_int as libc::c_char as libc::c_int) as
            libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x80400000 as libc::c_uint ==
                  0x80000000 as libc::c_uint) as libc::c_int as libc::c_char
                 as libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_b3(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down */
    flipped =
        right_count[((my_bits_low &
                          0x2000000 as
                              libc::c_uint).wrapping_add((my_bits_high &
                                                              0x2020202 as
                                                                  libc::c_uint)
                                                             <<
                                                             4 as
                                                                 libc::c_int).wrapping_mul(0x810204
                                                                                               as
                                                                                               libc::c_uint)
                         >> 27 as libc::c_int) as usize];
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x4000000 as
                                   libc::c_uint).wrapping_add(my_bits_high &
                                                                  0x40201008
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             right_count[(my_bits_low >> 17 as libc::c_int + 1 as libc::c_int
                              & 0x3f as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x408 as libc::c_uint == 0x8 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x202 as libc::c_uint == 0x2 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_g3(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down */
    flipped =
        right_count[((my_bits_high &
                          0x40404040 as
                              libc::c_uint).wrapping_add((my_bits_low &
                                                              0x40000000 as
                                                                  libc::c_uint)
                                                             >>
                                                             4 as
                                                                 libc::c_int).wrapping_mul(0x408102
                                                                                               as
                                                                                               libc::c_uint)
                         >> 27 as libc::c_int) as usize];
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_low &
                              0x20000000 as
                                  libc::c_uint).wrapping_add(my_bits_high &
                                                                 0x2040810 as
                                                                     libc::c_uint).wrapping_mul(0x2020202
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_low >> 22 as libc::c_int - 7 as libc::c_int &
                             0x7e as libc::c_int as libc::c_uint) as usize] as
                 libc::c_int) as libc::c_char;
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x2010 as libc::c_uint == 0x10 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x4040 as libc::c_uint == 0x40 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_b6(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Up */
    flipped =
        left_count[((my_bits_low &
                         0x2020202 as
                             libc::c_uint).wrapping_add((my_bits_high &
                                                             0x2 as
                                                                 libc::c_uint)
                                                            <<
                                                            4 as
                                                                libc::c_int).wrapping_mul(0x2040810
                                                                                              as
                                                                                              libc::c_uint)
                        >> 24 as libc::c_int) as usize];
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_high &
                               0x4 as
                                   libc::c_uint).wrapping_add(my_bits_low &
                                                                  0x8102040 as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             right_count[(my_bits_high >> 9 as libc::c_int + 1 as libc::c_int
                              & 0x3f as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x8040000 as libc::c_uint ==
                  0x8000000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x2020000 as libc::c_uint ==
                  0x2000000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_g6(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Up */
    flipped =
        left_count[((my_bits_high &
                         0x40 as
                             libc::c_uint).wrapping_add((my_bits_low &
                                                             0x40404040 as
                                                                 libc::c_uint)
                                                            >>
                                                            4 as
                                                                libc::c_int).wrapping_mul(0x1020408
                                                                                              as
                                                                                              libc::c_uint)
                        >> 24 as libc::c_int) as usize];
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x20 as
                                  libc::c_uint).wrapping_add(my_bits_low &
                                                                 0x10080402 as
                                                                     libc::c_uint).wrapping_mul(0x2020202
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_high >> 14 as libc::c_int - 7 as libc::c_int
                             & 0x7e as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x10200000 as libc::c_uint ==
                  0x10000000 as libc::c_uint) as libc::c_int as libc::c_char
                 as libc::c_int) as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x40400000 as libc::c_uint ==
                  0x40000000 as libc::c_uint) as libc::c_int as libc::c_char
                 as libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_c7(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Up */
    flipped =
        left_count[((my_bits_low &
                         0x4040404 as
                             libc::c_uint).wrapping_add((my_bits_high &
                                                             0x404 as
                                                                 libc::c_uint)
                                                            <<
                                                            4 as
                                                                libc::c_int).wrapping_mul(0x810204
                                                                                              as
                                                                                              libc::c_uint)
                        >> 24 as libc::c_int) as usize];
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_high &
                               0x810 as
                                   libc::c_uint).wrapping_add(my_bits_low &
                                                                  0x20408000
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 27 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             right_count[(my_bits_high >> 18 as libc::c_int + 1 as libc::c_int
                              & 0x1f as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x30000 as libc::c_uint ==
                  0x10000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x201 as libc::c_uint == 0x1 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_f7(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Up */
    flipped =
        left_count[((my_bits_high &
                         0x2020 as
                             libc::c_uint).wrapping_add((my_bits_low &
                                                             0x20202020 as
                                                                 libc::c_uint)
                                                            >>
                                                            4 as
                                                                libc::c_int).wrapping_mul(0x1020408
                                                                                              as
                                                                                              libc::c_uint)
                        >> 24 as libc::c_int) as usize];
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x1008 as
                                  libc::c_uint).wrapping_add(my_bits_low &
                                                                 0x4020100 as
                                                                     libc::c_uint).wrapping_mul(0x4040404
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_high >> 21 as libc::c_int - 7 as libc::c_int
                             & 0x7c as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0xc00000 as libc::c_uint ==
                  0x800000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x4080 as libc::c_uint == 0x80 as libc::c_uint)
                 as libc::c_int as libc::c_char as libc::c_int) as
            libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_d2(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xf700 as
                               libc::c_uint).wrapping_add(my_bits_low &
                                                              0x700 as
                                                                  libc::c_uint)
                          >> 8 as libc::c_int) as usize];
    /* Down left / Down right */
    flipped =
        (flipped as libc::c_int +
             center_count[(((my_bits_low &
                                 0x2040000 as
                                     libc::c_uint).wrapping_add(my_bits_high &
                                                                    0x1 as
                                                                        libc::c_uint)
                                <<
                                1 as
                                    libc::c_int).wrapping_add(my_bits_low &
                                                                  0x20100000
                                                                      as
                                                                      libc::c_uint).wrapping_add(my_bits_high
                                                                                                     &
                                                                                                     0x8040
                                                                                                         as
                                                                                                         libc::c_uint).wrapping_mul(0x1010101
                                                                                                                                        as
                                                                                                                                        libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x8080000 as
                                   libc::c_uint).wrapping_add((my_bits_high &
                                                                   0x8080808
                                                                       as
                                                                       libc::c_uint)
                                                                  <<
                                                                  4 as
                                                                      libc::c_int).wrapping_mul(0x204081
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_e2(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xef00 as
                               libc::c_uint).wrapping_add(my_bits_low &
                                                              0xf00 as
                                                                  libc::c_uint)
                          >> 9 as libc::c_int) as usize];
    /* Down left / Down right */
    flipped =
        (flipped as libc::c_int +
             center_count[(((my_bits_low &
                                 0x4080000 as
                                     libc::c_uint).wrapping_add(my_bits_high &
                                                                    0x102 as
                                                                        libc::c_uint)
                                <<
                                1 as
                                    libc::c_int).wrapping_add(my_bits_high &
                                                                  0x80 as
                                                                      libc::c_uint).wrapping_add(my_bits_low
                                                                                                     &
                                                                                                     0x40200000
                                                                                                         as
                                                                                                         libc::c_uint).wrapping_mul(0x1010101
                                                                                                                                        as
                                                                                                                                        libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[(((my_bits_low & 0x10100000 as libc::c_uint) >>
                               4 as
                                   libc::c_int).wrapping_add(my_bits_high &
                                                                 0x10101010 as
                                                                     libc::c_uint).wrapping_mul(0x1020408
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_b4(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 25 as libc::c_int + 1 as libc::c_int) as
                        usize];
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x40810 as
                                   libc::c_uint).wrapping_mul(0x1010100 as
                                                                  libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down / Up */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_low &
                                0x20202 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x2020202
                                                                        as
                                                                        libc::c_uint)
                                                                   <<
                                                                   3 as
                                                                       libc::c_int).wrapping_mul(0x1020408
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_high &
                               0x20100804 as
                                   libc::c_uint).wrapping_mul(0x1010101 as
                                                                  libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_g4(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down / Up */
    flipped =
        center_count[((my_bits_high &
                           0x40404040 as
                               libc::c_uint).wrapping_add((my_bits_low &
                                                               0x404040 as
                                                                   libc::c_uint)
                                                              >>
                                                              3 as
                                                                  libc::c_int).wrapping_mul(0x408102
                                                                                                as
                                                                                                libc::c_uint)
                          >> 24 as libc::c_int) as usize];
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_low &
                              0x201008 as
                                  libc::c_uint).wrapping_mul(0x2020200 as
                                                                 libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_low >> 30 as libc::c_int - 7 as libc::c_int &
                             0x7e as libc::c_int as libc::c_uint) as usize] as
                 libc::c_int) as libc::c_char;
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x4081020 as
                                  libc::c_uint).wrapping_mul(0x2020202 as
                                                                 libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_b5(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 1 as libc::c_int + 1 as libc::c_int &
                         0x3f as libc::c_int as libc::c_uint) as usize];
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x4081020 as
                                   libc::c_uint).wrapping_mul(0x1010101 as
                                                                  libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down / Up */
    flipped =
        (flipped as libc::c_int +
             center_count[(((my_bits_high & 0x2020200 as libc::c_uint) <<
                                3 as
                                    libc::c_int).wrapping_add(my_bits_low &
                                                                  0x2020202 as
                                                                      libc::c_uint).wrapping_mul(0x1020408
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_high &
                               0x10080400 as
                                   libc::c_uint).wrapping_mul(0x10101 as
                                                                  libc::c_uint)
                              >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_g5(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down / Up */
    flipped =
        center_count[((my_bits_high &
                           0x40404000 as
                               libc::c_uint).wrapping_add((my_bits_low &
                                                               0x40404040 as
                                                                   libc::c_uint)
                                                              >>
                                                              3 as
                                                                  libc::c_int).wrapping_mul(0x408102
                                                                                                as
                                                                                                libc::c_uint)
                          >> 25 as libc::c_int) as usize];
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x8102000 as
                                  libc::c_uint).wrapping_mul(0x20202 as
                                                                 libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_high << 7 as libc::c_int - 6 as libc::c_int &
                             0x7f as libc::c_int as libc::c_uint) as usize] as
                 libc::c_int) as libc::c_char;
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_low &
                              0x20100804 as
                                  libc::c_uint).wrapping_mul(0x2020202 as
                                                                 libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_d7(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xf70000 as
                               libc::c_uint).wrapping_add(my_bits_high &
                                                              0x70000 as
                                                                  libc::c_uint)
                          >> 16 as libc::c_int) as usize];
    /* Up right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x1020 as
                                    libc::c_uint).wrapping_add(my_bits_low &
                                                                   0x40800000
                                                                       as
                                                                       libc::c_uint).wrapping_add((my_bits_high
                                                                                                       &
                                                                                                       0x402
                                                                                                           as
                                                                                                           libc::c_uint).wrapping_add(my_bits_low
                                                                                                                                          &
                                                                                                                                          0x1000000
                                                                                                                                              as
                                                                                                                                              libc::c_uint)
                                                                                                      <<
                                                                                                      1
                                                                                                          as
                                                                                                          libc::c_int).wrapping_mul(0x1010101
                                                                                                                                        as
                                                                                                                                        libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[(((my_bits_high & 0x808 as libc::c_uint) <<
                              4 as
                                  libc::c_int).wrapping_add(my_bits_low &
                                                                0x8080808 as
                                                                    libc::c_uint).wrapping_mul(0x408102
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_e7(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xef0000 as
                               libc::c_uint).wrapping_add(my_bits_high &
                                                              0xf0000 as
                                                                  libc::c_uint)
                          >> 17 as libc::c_int) as usize];
    /* Up right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x2040 as
                                    libc::c_uint).wrapping_add(my_bits_low &
                                                                   0x80000000
                                                                       as
                                                                       libc::c_uint).wrapping_add((my_bits_high
                                                                                                       &
                                                                                                       0x804
                                                                                                           as
                                                                                                           libc::c_uint).wrapping_add(my_bits_low
                                                                                                                                          &
                                                                                                                                          0x2010000
                                                                                                                                              as
                                                                                                                                              libc::c_uint)
                                                                                                      <<
                                                                                                      1
                                                                                                          as
                                                                                                          libc::c_int).wrapping_mul(0x1010101
                                                                                                                                        as
                                                                                                                                        libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x1010 as
                                  libc::c_uint).wrapping_add((my_bits_low &
                                                                  0x10101010
                                                                      as
                                                                      libc::c_uint)
                                                                 >>
                                                                 4 as
                                                                     libc::c_int).wrapping_mul(0x2040810
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_c3(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down */
    flipped =
        right_count[((my_bits_low &
                          0x4000000 as
                              libc::c_uint).wrapping_add((my_bits_high &
                                                              0x4040404 as
                                                                  libc::c_uint)
                                                             <<
                                                             4 as
                                                                 libc::c_int).wrapping_mul(0x408102
                                                                                               as
                                                                                               libc::c_uint)
                         >> 27 as libc::c_int) as usize];
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x8000000 as
                                   libc::c_uint).wrapping_add(my_bits_high &
                                                                  0x80402010
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 27 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             right_count[(my_bits_low >> 18 as libc::c_int + 1 as libc::c_int
                              & 0x1f as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x810 as libc::c_uint == 0x10 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x404 as libc::c_uint == 0x4 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x201 as libc::c_uint == 0x1 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x30000 as libc::c_uint ==
                  0x10000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    /* Down left */
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(!my_bits_low >> 25 as libc::c_int &
                                            my_bits_high >> 0 as libc::c_int &
                                            1 as libc::c_int as libc::c_uint)
            as libc::c_char as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_f3(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down */
    flipped =
        right_count[((my_bits_high &
                          0x20202020 as
                              libc::c_uint).wrapping_add((my_bits_low &
                                                              0x20000000 as
                                                                  libc::c_uint)
                                                             >>
                                                             4 as
                                                                 libc::c_int).wrapping_mul(0x810204
                                                                                               as
                                                                                               libc::c_uint)
                         >> 27 as libc::c_int) as usize];
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_low &
                              0x10000000 as
                                  libc::c_uint).wrapping_add(my_bits_high &
                                                                 0x1020408 as
                                                                     libc::c_uint).wrapping_mul(0x4040404
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_low >> 21 as libc::c_int - 7 as libc::c_int &
                             0x7c as libc::c_int as libc::c_uint) as usize] as
                 libc::c_int) as libc::c_char;
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x1008 as libc::c_uint == 0x8 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x2020 as libc::c_uint == 0x20 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x4080 as libc::c_uint == 0x80 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0xc00000 as libc::c_uint ==
                  0x800000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    /* Down right */
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(!my_bits_low >> 30 as libc::c_int &
                                            my_bits_high >> 7 as libc::c_int &
                                            1 as libc::c_int as libc::c_uint)
            as libc::c_char as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_c6(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Up */
    flipped =
        left_count[((my_bits_low &
                         0x4040404 as
                             libc::c_uint).wrapping_add((my_bits_high &
                                                             0x4 as
                                                                 libc::c_uint)
                                                            <<
                                                            4 as
                                                                libc::c_int).wrapping_mul(0x1020408
                                                                                              as
                                                                                              libc::c_uint)
                        >> 24 as libc::c_int) as usize];
    /* Up right */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_high &
                               0x8 as
                                   libc::c_uint).wrapping_add(my_bits_low &
                                                                  0x10204080
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                              >> 27 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             right_count[(my_bits_high >> 10 as libc::c_int + 1 as libc::c_int
                              & 0x1f as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x10080000 as libc::c_uint ==
                  0x10000000 as libc::c_uint) as libc::c_int as libc::c_char
                 as libc::c_int) as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x4040000 as libc::c_uint ==
                  0x4000000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x1020000 as libc::c_uint ==
                  0x1000000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x300 as libc::c_uint == 0x100 as libc::c_uint)
                 as libc::c_int as libc::c_char as libc::c_int) as
            libc::c_char;
    /* Up left */
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(!my_bits_high >> 1 as libc::c_int &
                                            my_bits_low >> 24 as libc::c_int &
                                            1 as libc::c_int as libc::c_uint)
            as libc::c_char as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_f6(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Up */
    flipped =
        left_count[((my_bits_high &
                         0x20 as
                             libc::c_uint).wrapping_add((my_bits_low &
                                                             0x20202020 as
                                                                 libc::c_uint)
                                                            >>
                                                            4 as
                                                                libc::c_int).wrapping_mul(0x2040810
                                                                                              as
                                                                                              libc::c_uint)
                        >> 24 as libc::c_int) as usize];
    /* Up left */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x10 as
                                  libc::c_uint).wrapping_add(my_bits_low &
                                                                 0x8040201 as
                                                                     libc::c_uint).wrapping_mul(0x4040404
                                                                                                    as
                                                                                                    libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_high >> 13 as libc::c_int - 7 as libc::c_int
                             & 0x7c as libc::c_int as libc::c_uint) as usize]
                 as libc::c_int) as libc::c_char;
    /* Down left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x8100000 as libc::c_uint ==
                  0x8000000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x20200000 as libc::c_uint ==
                  0x20000000 as libc::c_uint) as libc::c_int as libc::c_char
                 as libc::c_int) as libc::c_char;
    /* Down right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x80400000 as libc::c_uint ==
                  0x80000000 as libc::c_uint) as libc::c_int as libc::c_char
                 as libc::c_int) as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0xc000 as libc::c_uint == 0x8000 as libc::c_uint)
                 as libc::c_int as libc::c_char as libc::c_int) as
            libc::c_char;
    /* Up right */
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(!my_bits_high >> 6 as libc::c_int &
                                            my_bits_low >> 31 as libc::c_int &
                                            1 as libc::c_int as libc::c_uint)
            as libc::c_char as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_d3(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xf70000 as
                               libc::c_uint).wrapping_add(my_bits_low &
                                                              0x70000 as
                                                                  libc::c_uint)
                          >> 16 as libc::c_int) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_low &
                                0x4001020 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x102 as
                                                                        libc::c_uint)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x1010102
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[((my_bits_low &
                               0x8000000 as
                                   libc::c_uint).wrapping_add((my_bits_high &
                                                                   0x8080808
                                                                       as
                                                                       libc::c_uint)
                                                                  <<
                                                                  4 as
                                                                      libc::c_int).wrapping_mul(0x204081
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 27 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x808 as libc::c_uint == 0x8 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Down right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_low &
                                0x10000402 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x804020
                                                                        as
                                                                        libc::c_uint)
                                                                   >>
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x2020201
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_e3(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xef0000 as
                               libc::c_uint).wrapping_add(my_bits_low &
                                                              0xf0000 as
                                                                  libc::c_uint)
                          >> 17 as libc::c_int) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_low &
                                0x8002040 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x10204 as
                                                                        libc::c_uint)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x1010102
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             right_count[(((my_bits_low & 0x10000000 as libc::c_uint) >>
                               4 as
                                   libc::c_int).wrapping_add(my_bits_high &
                                                                 0x10101010 as
                                                                     libc::c_uint).wrapping_mul(0x1020408
                                                                                                    as
                                                                                                    libc::c_uint)
                              >> 27 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x1010 as libc::c_uint == 0x10 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Down right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_low &
                                0x20000804 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x8040 as
                                                                        libc::c_uint)
                                                                   >>
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x2020201
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_c4(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_low >> 26 as libc::c_int + 1 as libc::c_int) as
                        usize];
    /* Left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0x3000000 as libc::c_uint ==
                  0x1000000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    /* Down left / Up right */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_low &
                                0x81020 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x102 as
                                                                        libc::c_uint)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x2020202
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down / Up */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_low &
                                0x40404 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x4040404
                                                                        as
                                                                        libc::c_uint)
                                                                   <<
                                                                   3 as
                                                                       libc::c_int).wrapping_mul(0x810204
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x40201008 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x20100 as
                                                                        libc::c_uint)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x2020202
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_f4(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down left / Up right */
    flipped =
        center_count[((my_bits_low &
                           0x408000 as
                               libc::c_uint).wrapping_add((my_bits_high &
                                                               0x2040810 as
                                                                   libc::c_uint)
                                                              <<
                                                              1 as
                                                                  libc::c_int).wrapping_mul(0x1010101
                                                                                                as
                                                                                                libc::c_uint)
                          >> 26 as libc::c_int) as usize];
    /* Down / Up */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x20202020 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x202020
                                                                        as
                                                                        libc::c_uint)
                                                                   >>
                                                                   3 as
                                                                       libc::c_int).wrapping_mul(0x810204
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x8040 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x100804
                                                                        as
                                                                        libc::c_uint)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_low >> 29 as libc::c_int - 7 as libc::c_int &
                             0x7c as libc::c_int as libc::c_uint) as usize] as
                 libc::c_int) as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_low & 0xc0000000 as libc::c_uint ==
                  0x80000000 as libc::c_uint) as libc::c_int as libc::c_char
                 as libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_c5(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right */
    flipped =
        right_count[(my_bits_high >> 2 as libc::c_int + 1 as libc::c_int &
                         0x1f as libc::c_int as libc::c_uint) as usize];
    /* Left */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x3 as libc::c_uint == 0x1 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    /* Down left / Up right */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_low &
                                0x8102040 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x10200 as
                                                                        libc::c_uint)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x2020202
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down / Up */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_low &
                                0x4040404 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x4040400
                                                                        as
                                                                        libc::c_uint)
                                                                   <<
                                                                   3 as
                                                                       libc::c_int).wrapping_mul(0x810204
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x20100800 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x2010000
                                                                        as
                                                                        libc::c_uint)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x2020202
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_f5(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Down left / Up right */
    flipped =
        center_count[((my_bits_low &
                           0x40800000 as
                               libc::c_uint).wrapping_add((my_bits_high &
                                                               0x4081000 as
                                                                   libc::c_uint)
                                                              <<
                                                              1 as
                                                                  libc::c_int).wrapping_mul(0x1010101
                                                                                                as
                                                                                                libc::c_uint)
                          >> 26 as libc::c_int) as usize];
    /* Down / Up */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x20202000 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x20202020
                                                                        as
                                                                        libc::c_uint)
                                                                   >>
                                                                   3 as
                                                                       libc::c_int).wrapping_mul(0x810204
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x804000 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x10080402
                                                                        as
                                                                        libc::c_uint)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 26 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Left */
    flipped =
        (flipped as libc::c_int +
             left_count[(my_bits_high << 7 as libc::c_int - 5 as libc::c_int &
                             0x7c as libc::c_int as libc::c_uint) as usize] as
                 libc::c_int) as libc::c_char;
    /* Right */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0xc0 as libc::c_uint == 0x80 as libc::c_uint) as
                 libc::c_int as libc::c_char as libc::c_int) as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_d6(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xf700 as
                               libc::c_uint).wrapping_add(my_bits_high &
                                                              0x700 as
                                                                  libc::c_uint)
                          >> 8 as libc::c_int) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x2040010 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x20408000
                                                                        as
                                                                        libc::c_uint)
                                                                   >>
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x1020202
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x8080000 as libc::c_uint ==
                  0x8000000 as libc::c_uint) as libc::c_int as libc::c_char as
                 libc::c_int) as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[(((my_bits_high & 0x8 as libc::c_uint) <<
                              4 as
                                  libc::c_int).wrapping_add(my_bits_low &
                                                                0x8080808 as
                                                                    libc::c_uint).wrapping_mul(0x810204
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x20100004 as libc::c_int as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x2010000
                                                                        as
                                                                        libc::c_uint)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x2010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_e6(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xef00 as
                               libc::c_uint).wrapping_add(my_bits_high &
                                                              0xf00 as
                                                                  libc::c_uint)
                          >> 9 as libc::c_int) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x4080020 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x40800000
                                                                        as
                                                                        libc::c_uint)
                                                                   >>
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x1020202
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down */
    flipped =
        (flipped as libc::c_int +
             (my_bits_high & 0x10100000 as libc::c_uint ==
                  0x10000000 as libc::c_uint) as libc::c_int as libc::c_char
                 as libc::c_int) as libc::c_char;
    /* Up */
    flipped =
        (flipped as libc::c_int +
             left_count[((my_bits_high &
                              0x10 as
                                  libc::c_uint).wrapping_add((my_bits_low &
                                                                  0x10101010
                                                                      as
                                                                      libc::c_uint)
                                                                 >>
                                                                 4 as
                                                                     libc::c_int).wrapping_mul(0x4081020
                                                                                                   as
                                                                                                   libc::c_uint)
                             >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x40200008 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x4020100
                                                                        as
                                                                        libc::c_uint)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x2010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_d4(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xf7000000 as
                               libc::c_uint).wrapping_add(my_bits_low &
                                                              0x7000000 as
                                                                  libc::c_uint)
                          >> 24 as libc::c_int) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as libc::c_int +
             center_count[(((my_bits_high & 0x10204 as libc::c_uint) <<
                                1 as
                                    libc::c_int).wrapping_add(my_bits_low &
                                                                  0x102040 as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down / Up */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_low &
                                0x80808 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x8080808
                                                                        as
                                                                        libc::c_uint)
                                                                   <<
                                                                   3 as
                                                                       libc::c_int).wrapping_mul(0x408102
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x80402010 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x40201 as
                                                                        libc::c_uint)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_e4(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_low &
                           0xef000000 as
                               libc::c_uint).wrapping_add(my_bits_low &
                                                              0xf000000 as
                                                                  libc::c_uint)
                          >> 25 as libc::c_int) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x1020408 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x204080
                                                                        as
                                                                        libc::c_uint)
                                                                   >>
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down / Up */
    flipped =
        (flipped as libc::c_int +
             center_count[(((my_bits_low & 0x101010 as libc::c_uint) >>
                                3 as
                                    libc::c_int).wrapping_add(my_bits_high &
                                                                  0x10101010
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1020408
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[(((my_bits_high & 0x804020 as libc::c_uint) >>
                                1 as
                                    libc::c_int).wrapping_add(my_bits_low &
                                                                  0x80402 as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_d5(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xf7 as
                               libc::c_uint).wrapping_add(my_bits_high &
                                                              0x7 as
                                                                  libc::c_uint)
                          >> 0 as libc::c_int) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as libc::c_int +
             center_count[(((my_bits_high & 0x1020400 as libc::c_uint) <<
                                1 as
                                    libc::c_int).wrapping_add(my_bits_low &
                                                                  0x10204080
                                                                      as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down / Up */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_low &
                                0x8080808 as
                                    libc::c_uint).wrapping_add((my_bits_high &
                                                                    0x8080800
                                                                        as
                                                                        libc::c_uint)
                                                                   <<
                                                                   3 as
                                                                       libc::c_int).wrapping_mul(0x408102
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x40201000 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x4020100
                                                                        as
                                                                        libc::c_uint)
                                                                   <<
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}
unsafe fn CountFlips_bitboard_e5(mut my_bits_high: libc::c_uint,
                                            mut my_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_char = 0;
    /* Right / Left */
    flipped =
        center_count[((my_bits_high &
                           0xef as
                               libc::c_uint).wrapping_add(my_bits_high &
                                                              0xf as
                                                                  libc::c_uint)
                          >> 1 as libc::c_int) as usize];
    /* Down left / Up right */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x2040800 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x20408000
                                                                        as
                                                                        libc::c_uint)
                                                                   >>
                                                                   1 as
                                                                       libc::c_int).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down / Up */
    flipped =
        (flipped as libc::c_int +
             center_count[((my_bits_high &
                                0x10101000 as
                                    libc::c_uint).wrapping_add((my_bits_low &
                                                                    0x10101010
                                                                        as
                                                                        libc::c_uint)
                                                                   >>
                                                                   3 as
                                                                       libc::c_int).wrapping_mul(0x1020408
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 25 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    /* Down right / Up left */
    flipped =
        (flipped as libc::c_int +
             center_count[(((my_bits_high & 0x80402000 as libc::c_uint) >>
                                1 as
                                    libc::c_int).wrapping_add(my_bits_low &
                                                                  0x8040201 as
                                                                      libc::c_uint).wrapping_mul(0x1010101
                                                                                                     as
                                                                                                     libc::c_uint)
                               >> 24 as libc::c_int) as usize] as libc::c_int)
            as libc::c_char;
    return flipped as libc::c_int;
}

pub static mut CountFlips_bitboard:
           [Option<unsafe fn(_: libc::c_uint, _: libc::c_uint)
                       -> libc::c_int>; 78] =
    unsafe {
        [Some(CountFlips_bitboard_a1 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_b1 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_c1 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_d1 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_e1 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_f1 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_g1 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_h1 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(CountFlips_bitboard_a2 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_b2 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_c2 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_d2 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_e2 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_f2 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_g2 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_h2 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(CountFlips_bitboard_a3 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_b3 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_c3 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_d3 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_e3 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_f3 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_g3 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_h3 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(CountFlips_bitboard_a4 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_b4 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_c4 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_d4 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_e4 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_f4 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_g4 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_h4 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(CountFlips_bitboard_a5 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_b5 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_c5 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_d5 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_e5 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_f5 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_g5 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_h5 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(CountFlips_bitboard_a6 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_b6 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_c6 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_d6 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_e6 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_f6 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_g6 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_h6 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(CountFlips_bitboard_a7 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_b7 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_c7 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_d7 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_e7 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_f7 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_g7 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_h7 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(CountFlips_bitboard_a8 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_b8 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_c8 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_d8 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_e8 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_f8 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_g8 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(CountFlips_bitboard_h8 as
                  unsafe fn(_: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int)]
    };
