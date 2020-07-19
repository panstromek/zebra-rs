use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitBoard {
    pub high: libc::c_uint,
    pub low: libc::c_uint,
}
/*
   File:          bitbtest.c

   Modified:      November 24, 2005

   Authors:       Gunnar Andersson (gunnar@radagast.se)
              Toshihiko Okuhara

   Contents:      Count flips and returns new_my_bits in bb_flips.

   This piece of software is released under the GPL.
   See the file COPYING for more information.
*/
#[no_mangle]
pub static mut bb_flips: BitBoard = BitBoard{high: 0, low: 0,};
static mut right_contiguous: [libc::c_uchar; 64] =
    [0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 5 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 6 as libc::c_int as libc::c_uchar];
static mut left_contiguous: [libc::c_uchar; 64] =
    [0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     4 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     5 as libc::c_int as libc::c_uchar, 6 as libc::c_int as libc::c_uchar];
static mut right_flip: [libc::c_uint; 7] =
    [0x1 as libc::c_uint, 0x3 as libc::c_uint, 0x7 as libc::c_uint,
     0xf as libc::c_uint, 0x1f as libc::c_uint, 0x3f as libc::c_uint,
     0x7f as libc::c_uint];
static mut lsb_mask: [libc::c_uint; 4] =
    [0xff as libc::c_uint, 0xffff as libc::c_uint, 0xffffff as libc::c_uint,
     0xffffffff as libc::c_uint];
static mut msb_mask: [libc::c_uint; 4] =
    [0xff000000 as libc::c_uint, 0xffff0000 as libc::c_uint,
     0xffffff00 as libc::c_uint, 0xffffffff as libc::c_uint];
static mut pop_count: [libc::c_uchar; 64] =
    [0 as libc::c_int as libc::c_uchar, 1 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     4 as libc::c_int as libc::c_uchar, 5 as libc::c_int as libc::c_uchar,
     1 as libc::c_int as libc::c_uchar, 2 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     4 as libc::c_int as libc::c_uchar, 5 as libc::c_int as libc::c_uchar,
     2 as libc::c_int as libc::c_uchar, 3 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     4 as libc::c_int as libc::c_uchar, 5 as libc::c_int as libc::c_uchar,
     3 as libc::c_int as libc::c_uchar, 4 as libc::c_int as libc::c_uchar,
     4 as libc::c_int as libc::c_uchar, 5 as libc::c_int as libc::c_uchar,
     4 as libc::c_int as libc::c_uchar, 5 as libc::c_int as libc::c_uchar,
     5 as libc::c_int as libc::c_uchar, 6 as libc::c_int as libc::c_uchar];
static mut c_frontier: [libc::c_uchar; 62] =
    [0 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x10 as libc::c_int as libc::c_uchar,
     0x11 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x20 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x10 as libc::c_int as libc::c_uchar,
     0x11 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x40 as libc::c_int as libc::c_uchar,
     0x41 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x10 as libc::c_int as libc::c_uchar,
     0x11 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x20 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x10 as libc::c_int as libc::c_uchar,
     0x11 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x80 as libc::c_int as libc::c_uchar,
     0x81 as libc::c_int as libc::c_uchar];
static mut d_frontier: [libc::c_uchar; 60] =
    [0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x40 as libc::c_int as libc::c_uchar,
     0x40 as libc::c_int as libc::c_uchar,
     0x42 as libc::c_int as libc::c_uchar,
     0x41 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x20 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x80 as libc::c_int as libc::c_uchar,
     0x80 as libc::c_int as libc::c_uchar,
     0x82 as libc::c_int as libc::c_uchar,
     0x81 as libc::c_int as libc::c_uchar];
static mut e_frontier: [libc::c_uchar; 56] =
    [0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x4 as libc::c_int as libc::c_uchar, 0x4 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x40 as libc::c_int as libc::c_uchar,
     0x40 as libc::c_int as libc::c_uchar,
     0x40 as libc::c_int as libc::c_uchar,
     0x40 as libc::c_int as libc::c_uchar,
     0x44 as libc::c_int as libc::c_uchar,
     0x44 as libc::c_int as libc::c_uchar,
     0x42 as libc::c_int as libc::c_uchar,
     0x41 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x4 as libc::c_int as libc::c_uchar,
     0x4 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x80 as libc::c_int as libc::c_uchar,
     0x80 as libc::c_int as libc::c_uchar,
     0x80 as libc::c_int as libc::c_uchar,
     0x80 as libc::c_int as libc::c_uchar,
     0x84 as libc::c_int as libc::c_uchar,
     0x84 as libc::c_int as libc::c_uchar,
     0x82 as libc::c_int as libc::c_uchar,
     0x81 as libc::c_int as libc::c_uchar];
static mut f_flip: [libc::c_uchar; 160] =
    [0 as libc::c_int as libc::c_uchar, 0xf as libc::c_int as libc::c_uchar,
     0xe as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0xc as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x8 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x7 as libc::c_int as libc::c_uchar,
     0x6 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x4 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x3 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x4 as libc::c_int as libc::c_uchar, 0x5 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x8 as libc::c_int as libc::c_uchar, 0xb as libc::c_int as libc::c_uchar,
     0xa as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0xc as libc::c_int as libc::c_uchar, 0xd as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x8 as libc::c_int as libc::c_uchar, 0x8 as libc::c_int as libc::c_uchar,
     0x8 as libc::c_int as libc::c_uchar, 0x8 as libc::c_int as libc::c_uchar,
     0x4 as libc::c_int as libc::c_uchar, 0x4 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x10 as libc::c_int as libc::c_uchar,
     0x17 as libc::c_int as libc::c_uchar,
     0x16 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x14 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x18 as libc::c_int as libc::c_uchar,
     0x1b as libc::c_int as libc::c_uchar,
     0x1a as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x1c as libc::c_int as libc::c_uchar,
     0x1d as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x80 as libc::c_int as libc::c_uchar,
     0x80 as libc::c_int as libc::c_uchar,
     0x80 as libc::c_int as libc::c_uchar,
     0x80 as libc::c_int as libc::c_uchar,
     0x80 as libc::c_int as libc::c_uchar,
     0x80 as libc::c_int as libc::c_uchar,
     0x80 as libc::c_int as libc::c_uchar,
     0x80 as libc::c_int as libc::c_uchar,
     0x88 as libc::c_int as libc::c_uchar,
     0x88 as libc::c_int as libc::c_uchar,
     0x88 as libc::c_int as libc::c_uchar,
     0x88 as libc::c_int as libc::c_uchar,
     0x84 as libc::c_int as libc::c_uchar,
     0x84 as libc::c_int as libc::c_uchar,
     0x82 as libc::c_int as libc::c_uchar,
     0x81 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x20 as libc::c_int as libc::c_uchar,
     0x2f as libc::c_int as libc::c_uchar,
     0x2e as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x2c as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x28 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x30 as libc::c_int as libc::c_uchar,
     0x37 as libc::c_int as libc::c_uchar,
     0x36 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x34 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x38 as libc::c_int as libc::c_uchar,
     0x3b as libc::c_int as libc::c_uchar,
     0x3a as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x3c as libc::c_int as libc::c_uchar,
     0x3d as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar];
// //
unsafe extern "C" fn TestFlips_bitboard_a1(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 0 as libc::c_int + 1 as libc::c_int
                              & 0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 0 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x1010100 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    0 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    0 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    0 as libc::c_int + 8 as libc::c_int * 6 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x1010101 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x1010100 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as libc::c_int &
                    0x1010100 as libc::c_uint;
            my_bits_low |= t | t >> 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     0 as libc::c_int +
                                                         8 as libc::c_int |
                                                     t >>
                                                         0 as libc::c_int +
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 0 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x8040200 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    0 as libc::c_int + 9 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    0 as libc::c_int + 9 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    0 as libc::c_int + 9 as libc::c_int * 6 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x80402010 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x8040200 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 9 as libc::c_int &
                    0x8040200 as libc::c_uint;
            my_bits_low |= t | t >> 9 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     0 as libc::c_int +
                                                         9 as libc::c_int |
                                                     t >>
                                                         0 as libc::c_int +
                                                             9 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    my_bits_low |= 0x1 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_h1(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 7 as libc::c_int - 6 as libc::c_int &
                             0x3f as libc::c_int as libc::c_uint) as usize] as
            libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 7 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 7 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x10204000 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    7 as libc::c_int + 7 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    7 as libc::c_int + 7 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    7 as libc::c_int + 7 as libc::c_int * 6 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x1020408 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x10204000 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 7 as libc::c_int &
                    0x10204000 as libc::c_uint;
            my_bits_low |= t | t >> 7 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     7 as libc::c_int +
                                                         7 as libc::c_int |
                                                     t >>
                                                         7 as libc::c_int +
                                                             7 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 7 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x80808000 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    7 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    7 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    7 as libc::c_int + 8 as libc::c_int * 6 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x80808080 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x80808000 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as libc::c_int &
                    0x80808000 as libc::c_uint;
            my_bits_low |= t | t >> 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     7 as libc::c_int +
                                                         8 as libc::c_int |
                                                     t >>
                                                         7 as libc::c_int +
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    my_bits_low |= 0x80 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_a8(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >>
                              24 as libc::c_int + 1 as libc::c_int &
                              0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 24 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 24 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x20408 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    24 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    24 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    24 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 6 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x10204080 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x20408 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 7 as libc::c_int &
                    0x20408 as libc::c_uint;
            my_bits_high |= t | t << 7 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     24 as libc::c_int -
                                                         7 as libc::c_int |
                                                     t >>
                                                         24 as libc::c_int -
                                                             7 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 24 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x10101 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    24 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    24 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    24 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 6 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x1010101 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x10101 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as libc::c_int &
                    0x10101 as libc::c_uint;
            my_bits_high |= t | t << 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     24 as libc::c_int -
                                                         8 as libc::c_int |
                                                     t >>
                                                         24 as libc::c_int -
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    my_bits_high |= 0x1000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_h8(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 31 as libc::c_int - 6 as libc::c_int
                             & 0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 31 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 31 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x808080 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    31 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    31 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    31 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 6 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x80808080 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x808080 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as libc::c_int &
                    0x808080 as libc::c_uint;
            my_bits_high |= t | t << 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     31 as libc::c_int -
                                                         8 as libc::c_int |
                                                     t >>
                                                         31 as libc::c_int -
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 31 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x402010 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    31 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    31 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    31 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 6 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x8040201 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x402010 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 9 as libc::c_int &
                    0x402010 as libc::c_uint;
            my_bits_high |= t | t << 9 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     31 as libc::c_int -
                                                         9 as libc::c_int |
                                                     t >>
                                                         31 as libc::c_int -
                                                             9 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    my_bits_high |= 0x80000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_b1(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 1 as libc::c_int + 1 as libc::c_int
                              & 0x1f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 1 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 1 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x2020200 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    1 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    1 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    1 as libc::c_int + 8 as libc::c_int * 6 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x2020202 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x2020200 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as libc::c_int &
                    0x2020200 as libc::c_uint;
            my_bits_low |= t | t >> 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     1 as libc::c_int +
                                                         8 as libc::c_int |
                                                     t >>
                                                         1 as libc::c_int +
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 1 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x10080400 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    1 as libc::c_int + 9 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    1 as libc::c_int + 9 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x804020 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x10080400 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 9 as libc::c_int &
                    0x10080400 as libc::c_uint;
            my_bits_low |= t | t >> 9 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     1 as libc::c_int +
                                                         9 as libc::c_int |
                                                     t >>
                                                         1 as libc::c_int +
                                                             9 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    my_bits_low |= 0x2 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_g1(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 6 as libc::c_int - 6 as libc::c_int &
                             0x3e as libc::c_int as libc::c_uint) as usize] as
            libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 6 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 6 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x8102000 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    6 as libc::c_int + 7 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    6 as libc::c_int + 7 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x10204 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x8102000 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 7 as libc::c_int &
                    0x8102000 as libc::c_uint;
            my_bits_low |= t | t >> 7 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     6 as libc::c_int +
                                                         7 as libc::c_int |
                                                     t >>
                                                         6 as libc::c_int +
                                                             7 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 6 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x40404000 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    6 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    6 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    6 as libc::c_int + 8 as libc::c_int * 6 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x40404040 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x40404000 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as libc::c_int &
                    0x40404000 as libc::c_uint;
            my_bits_low |= t | t >> 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     6 as libc::c_int +
                                                         8 as libc::c_int |
                                                     t >>
                                                         6 as libc::c_int +
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    my_bits_low |= 0x40 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_a2(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 8 as libc::c_int + 1 as libc::c_int
                              & 0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 8 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 8 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    8 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int) as
                   libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    8 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    8 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    8 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x1010101 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) << 8 as libc::c_int + 8 as libc::c_int
                         |
                         (1 as libc::c_int) <<
                             8 as libc::c_int +
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    8 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 8 as libc::c_int + 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 8 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    8 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int) as
                   libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    8 as libc::c_int + 9 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    8 as libc::c_int + 9 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    8 as libc::c_int + 9 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x40201008 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) << 8 as libc::c_int + 9 as libc::c_int
                         |
                         (1 as libc::c_int) <<
                             8 as libc::c_int +
                                 9 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    8 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 8 as libc::c_int + 9 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    my_bits_low |= 0x100 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_h2(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 15 as libc::c_int - 6 as libc::c_int
                             & 0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 15 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 15 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    15 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    15 as libc::c_int + 7 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    15 as libc::c_int + 7 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    15 as libc::c_int + 7 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x2040810 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) <<
                         15 as libc::c_int + 7 as libc::c_int |
                         (1 as libc::c_int) <<
                             15 as libc::c_int +
                                 7 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    15 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 15 as libc::c_int + 7 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 15 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    15 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    15 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    15 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    15 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x80808080 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) <<
                         15 as libc::c_int + 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             15 as libc::c_int +
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    15 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 15 as libc::c_int + 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    my_bits_low |= 0x8000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_a7(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >>
                              16 as libc::c_int + 1 as libc::c_int &
                              0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 16 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 16 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    16 as libc::c_int - 7 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    16 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    16 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    16 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x8102040 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         16 as libc::c_int - 7 as libc::c_int |
                         (1 as libc::c_int) <<
                             16 as libc::c_int -
                                 7 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    16 as libc::c_int - 7 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 16 as libc::c_int - 7 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 16 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    16 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    16 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    16 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    16 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x1010101 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         16 as libc::c_int - 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             16 as libc::c_int -
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    16 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 16 as libc::c_int - 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    my_bits_high |= 0x10000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_h7(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 23 as libc::c_int - 6 as libc::c_int
                             & 0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 23 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 23 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    23 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    23 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    23 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    23 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x80808080 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         23 as libc::c_int - 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             23 as libc::c_int -
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    23 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 23 as libc::c_int - 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 23 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    23 as libc::c_int - 9 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    23 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    23 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    23 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x10080402 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         23 as libc::c_int - 9 as libc::c_int |
                         (1 as libc::c_int) <<
                             23 as libc::c_int -
                                 9 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    23 as libc::c_int - 9 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 23 as libc::c_int - 9 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    my_bits_high |= 0x800000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_b8(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >>
                              25 as libc::c_int + 1 as libc::c_int &
                              0x1f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 25 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 25 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x40810 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    25 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    25 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x20408000 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x40810 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 7 as libc::c_int &
                    0x40810 as libc::c_uint;
            my_bits_high |= t | t << 7 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     25 as libc::c_int -
                                                         7 as libc::c_int |
                                                     t >>
                                                         25 as libc::c_int -
                                                             7 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 25 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x20202 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    25 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    25 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    25 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 6 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x2020202 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x20202 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as libc::c_int &
                    0x20202 as libc::c_uint;
            my_bits_high |= t | t << 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     25 as libc::c_int -
                                                         8 as libc::c_int |
                                                     t >>
                                                         25 as libc::c_int -
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    my_bits_high |= 0x2000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_g8(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 30 as libc::c_int - 6 as libc::c_int
                             & 0x3e as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 30 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 30 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x404040 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    30 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    30 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    30 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 6 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x40404040 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x404040 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as libc::c_int &
                    0x404040 as libc::c_uint;
            my_bits_high |= t | t << 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     30 as libc::c_int -
                                                         8 as libc::c_int |
                                                     t >>
                                                         30 as libc::c_int -
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 30 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x201008 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    30 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    30 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x4020100 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x201008 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 9 as libc::c_int &
                    0x201008 as libc::c_uint;
            my_bits_high |= t | t << 9 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     30 as libc::c_int -
                                                         9 as libc::c_int |
                                                     t >>
                                                         30 as libc::c_int -
                                                             9 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    my_bits_high |= 0x40000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_b2(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 9 as libc::c_int + 1 as libc::c_int
                              & 0x1f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 9 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 9 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    9 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int) as
                   libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    9 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    9 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    9 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x2020202 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) << 9 as libc::c_int + 8 as libc::c_int
                         |
                         (1 as libc::c_int) <<
                             9 as libc::c_int +
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    9 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 9 as libc::c_int + 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 9 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    9 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int) as
                   libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    9 as libc::c_int + 9 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    9 as libc::c_int + 9 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    9 as libc::c_int + 9 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x80402010 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) << 9 as libc::c_int + 9 as libc::c_int
                         |
                         (1 as libc::c_int) <<
                             9 as libc::c_int +
                                 9 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    9 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 9 as libc::c_int + 9 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    my_bits_low |= 0x200 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_g2(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 14 as libc::c_int - 6 as libc::c_int
                             & 0x3e as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 14 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 14 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    14 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    14 as libc::c_int + 7 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    14 as libc::c_int + 7 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    14 as libc::c_int + 7 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x1020408 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) <<
                         14 as libc::c_int + 7 as libc::c_int |
                         (1 as libc::c_int) <<
                             14 as libc::c_int +
                                 7 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    14 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 14 as libc::c_int + 7 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 14 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    14 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    14 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    14 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    14 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x40404040 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) <<
                         14 as libc::c_int + 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             14 as libc::c_int +
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    14 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 14 as libc::c_int + 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    my_bits_low |= 0x4000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_b7(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >>
                              17 as libc::c_int + 1 as libc::c_int &
                              0x1f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 17 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 17 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    17 as libc::c_int - 7 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    17 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    17 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    17 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x10204080 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         17 as libc::c_int - 7 as libc::c_int |
                         (1 as libc::c_int) <<
                             17 as libc::c_int -
                                 7 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    17 as libc::c_int - 7 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 17 as libc::c_int - 7 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 17 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    17 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    17 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    17 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    17 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x2020202 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         17 as libc::c_int - 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             17 as libc::c_int -
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    17 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 17 as libc::c_int - 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    my_bits_high |= 0x20000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_g7(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 22 as libc::c_int - 6 as libc::c_int
                             & 0x3e as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 22 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 22 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    22 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    22 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    22 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    22 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x40404040 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         22 as libc::c_int - 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             22 as libc::c_int -
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    22 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 22 as libc::c_int - 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 22 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    22 as libc::c_int - 9 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    22 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    22 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    22 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x8040201 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         22 as libc::c_int - 9 as libc::c_int |
                         (1 as libc::c_int) <<
                             22 as libc::c_int -
                                 9 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    22 as libc::c_int - 9 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 22 as libc::c_int - 9 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    my_bits_high |= 0x400000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_c1(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_low >> 2 as libc::c_int - 1 as libc::c_int &
                        0x3d as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        2 as libc::c_int -
                            2 as
                                libc::c_int).wrapping_add(28 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 2 as libc::c_int - 1 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    t =
        opp_bits_low & my_bits_low >> 7 as libc::c_int &
            ((1 as libc::c_int) << 2 as libc::c_int + 7 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            2 as libc::c_int +
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 2 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x4040400 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    2 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    2 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    2 as libc::c_int + 8 as libc::c_int * 6 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x4040404 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x4040400 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as libc::c_int &
                    0x4040400 as libc::c_uint;
            my_bits_low |= t | t >> 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     2 as libc::c_int +
                                                         8 as libc::c_int |
                                                     t >>
                                                         2 as libc::c_int +
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 2 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x20100800 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    2 as libc::c_int + 9 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t =
                t <<
                    2 as libc::c_int + 9 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int |
                    ((1 as libc::c_int) <<
                         2 as libc::c_int +
                             9 as libc::c_int * 4 as libc::c_int -
                             32 as libc::c_int) as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x20100800 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 9 as libc::c_int &
                    0x20100800 as libc::c_uint;
            my_bits_low |= t | t >> 9 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     2 as libc::c_int +
                                                         9 as libc::c_int |
                                                     t >>
                                                         2 as libc::c_int +
                                                             9 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    my_bits_low |= 0x4 as libc::c_int as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_f1(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_low >> 5 as libc::c_int - 4 as libc::c_int &
                    0x2f as libc::c_int as
                        libc::c_uint).wrapping_add(64 as libc::c_int as
                                                       libc::c_uint) as usize]
            as libc::c_uint;
    fl =
        f_flip[(t & my_bits_low >> 5 as libc::c_int - 5 as libc::c_int) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 5 as libc::c_int - 4 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 5 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x4081000 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    5 as libc::c_int + 7 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t =
                t <<
                    5 as libc::c_int + 7 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int |
                    ((1 as libc::c_int) <<
                         5 as libc::c_int +
                             7 as libc::c_int * 4 as libc::c_int -
                             32 as libc::c_int) as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x4081000 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 7 as libc::c_int &
                    0x4081000 as libc::c_uint;
            my_bits_low |= t | t >> 7 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     5 as libc::c_int +
                                                         7 as libc::c_int |
                                                     t >>
                                                         5 as libc::c_int +
                                                             7 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 5 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x20202000 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    5 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    5 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    5 as libc::c_int + 8 as libc::c_int * 6 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x20202020 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x20202000 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as libc::c_int &
                    0x20202000 as libc::c_uint;
            my_bits_low |= t | t >> 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     5 as libc::c_int +
                                                         8 as libc::c_int |
                                                     t >>
                                                         5 as libc::c_int +
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Down right */
    t =
        opp_bits_low & my_bits_low >> 9 as libc::c_int &
            ((1 as libc::c_int) << 5 as libc::c_int + 9 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            5 as libc::c_int +
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_low |= 0x20 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_a3(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 16 as libc::c_int + 1 as libc::c_int
                              & 0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 16 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as libc::c_int &
            ((1 as libc::c_int) << 16 as libc::c_int - 7 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            16 as libc::c_int -
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 16 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                16 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                16 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                16 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x1010101 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 16 as libc::c_int + 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as libc::c_int &
            ((1 as libc::c_int) << 16 as libc::c_int - 8 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            16 as libc::c_int -
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 16 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                16 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                16 as libc::c_int + 9 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                16 as libc::c_int + 9 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x20100804 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 16 as libc::c_int + 9 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    my_bits_low |= 0x10000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_h3(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 23 as libc::c_int - 6 as libc::c_int
                             & 0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 23 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 23 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                23 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                23 as libc::c_int + 7 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                23 as libc::c_int + 7 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x4081020 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 23 as libc::c_int + 7 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 23 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                23 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                23 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                23 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x80808080 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 23 as libc::c_int + 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as libc::c_int &
            ((1 as libc::c_int) << 23 as libc::c_int - 8 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            23 as libc::c_int -
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as libc::c_int &
            ((1 as libc::c_int) << 23 as libc::c_int - 9 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            23 as libc::c_int -
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_low |= 0x800000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_a6(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >> 8 as libc::c_int + 1 as libc::c_int
                              & 0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 8 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 8 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                8 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                8 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                8 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x4081020 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 8 as libc::c_int - 7 as libc::c_int) as
                    libc::c_uint;
            flipped += contig
        }
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as libc::c_int &
            ((1 as libc::c_int) << 8 as libc::c_int + 8 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            8 as libc::c_int +
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 8 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                8 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                8 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                8 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x1010101 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 8 as libc::c_int - 8 as libc::c_int) as
                    libc::c_uint;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as libc::c_int &
            ((1 as libc::c_int) << 8 as libc::c_int + 9 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            8 as libc::c_int +
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_high |= 0x100 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_h6(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 15 as libc::c_int - 6 as libc::c_int
                             & 0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 15 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as libc::c_int &
            ((1 as libc::c_int) << 15 as libc::c_int + 7 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            15 as libc::c_int +
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as libc::c_int &
            ((1 as libc::c_int) << 15 as libc::c_int + 8 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            15 as libc::c_int +
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 15 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                15 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                15 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                15 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x80808080 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 15 as libc::c_int - 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 15 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                15 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                15 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                15 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x20100804 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 15 as libc::c_int - 9 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    my_bits_high |= 0x8000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_c8(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_high >> 26 as libc::c_int - 1 as libc::c_int &
                        0x3d as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        26 as libc::c_int -
                            2 as
                                libc::c_int).wrapping_add(28 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 26 as libc::c_int - 1 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 26 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x81020 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    26 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t =
                t <<
                    26 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 5 as libc::c_int |
                    ((1 as libc::c_int) <<
                         26 as libc::c_int + 32 as libc::c_int -
                             7 as libc::c_int * 4 as libc::c_int) as
                        libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x81020 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 7 as libc::c_int &
                    0x81020 as libc::c_uint;
            my_bits_high |= t | t << 7 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     26 as libc::c_int -
                                                         7 as libc::c_int |
                                                     t >>
                                                         26 as libc::c_int -
                                                             7 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 26 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x40404 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    26 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    26 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    26 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 6 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x4040404 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x40404 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as libc::c_int &
                    0x40404 as libc::c_uint;
            my_bits_high |= t | t << 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     26 as libc::c_int -
                                                         8 as libc::c_int |
                                                     t >>
                                                         26 as libc::c_int -
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Up left */
    t =
        opp_bits_high & my_bits_high << 9 as libc::c_int &
            ((1 as libc::c_int) << 26 as libc::c_int - 9 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            26 as libc::c_int -
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_high |= 0x4000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_f8(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_high >> 29 as libc::c_int - 4 as libc::c_int &
                    0x2f as libc::c_int as
                        libc::c_uint).wrapping_add(64 as libc::c_int as
                                                       libc::c_uint) as usize]
            as libc::c_uint;
    fl =
        f_flip[(t & my_bits_high >> 29 as libc::c_int - 5 as libc::c_int) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 29 as libc::c_int - 4 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Up right */
    t =
        opp_bits_high & my_bits_high << 7 as libc::c_int &
            ((1 as libc::c_int) << 29 as libc::c_int - 7 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            29 as libc::c_int -
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 29 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x202020 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    29 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    29 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    29 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 6 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x20202020 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x202020 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as libc::c_int &
                    0x202020 as libc::c_uint;
            my_bits_high |= t | t << 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     29 as libc::c_int -
                                                         8 as libc::c_int |
                                                     t >>
                                                         29 as libc::c_int -
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 29 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x100804 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    29 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t =
                t <<
                    29 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 5 as libc::c_int |
                    ((1 as libc::c_int) <<
                         29 as libc::c_int + 32 as libc::c_int -
                             9 as libc::c_int * 4 as libc::c_int) as
                        libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x100804 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 9 as libc::c_int &
                    0x100804 as libc::c_uint;
            my_bits_high |= t | t << 9 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     29 as libc::c_int -
                                                         9 as libc::c_int |
                                                     t >>
                                                         29 as libc::c_int -
                                                             9 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    my_bits_high |= 0x20000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_d1(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_low >> 3 as libc::c_int - 2 as libc::c_int &
                        0x3b as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        3 as libc::c_int -
                            3 as
                                libc::c_int).wrapping_add(24 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 3 as libc::c_int - 2 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 3 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low >> 7 as libc::c_int &
                0x20400 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t >> 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 3 as libc::c_int +
                                                     7 as libc::c_int |
                                                 t >>
                                                     3 as libc::c_int +
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 3 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x8080800 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    3 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    3 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    3 as libc::c_int + 8 as libc::c_int * 6 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x8080808 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x8080800 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as libc::c_int &
                    0x8080800 as libc::c_uint;
            my_bits_low |= t | t >> 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     3 as libc::c_int +
                                                         8 as libc::c_int |
                                                     t >>
                                                         3 as libc::c_int +
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 3 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x40201000 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                ((my_bits_high <<
                      31 as libc::c_int -
                          (3 as libc::c_int +
                               9 as libc::c_int * 4 as libc::c_int -
                               32 as libc::c_int)) as libc::c_int >>
                     31 as libc::c_int) as libc::c_uint;
            my_bits_low |= 0x40201000 as libc::c_uint & t;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add(3 as libc::c_int as
                                                    libc::c_uint & t) as
                    libc::c_int as libc::c_int
        } else {
            t =
                opp_bits_low & my_bits_low >> 9 as libc::c_int &
                    0x40201000 as libc::c_uint;
            my_bits_low |= t | t >> 9 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     3 as libc::c_int +
                                                         9 as libc::c_int |
                                                     t >>
                                                         3 as libc::c_int +
                                                             9 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    my_bits_low |= 0x8 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_e1(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_low >> 4 as libc::c_int - 3 as libc::c_int &
                        0x37 as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        4 as libc::c_int -
                            4 as
                                libc::c_int).wrapping_add(16 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 4 as libc::c_int - 3 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 4 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x2040800 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                ((my_bits_high <<
                      31 as libc::c_int -
                          (4 as libc::c_int +
                               7 as libc::c_int * 4 as libc::c_int -
                               32 as libc::c_int)) as libc::c_int >>
                     31 as libc::c_int) as libc::c_uint;
            my_bits_low |= 0x2040800 as libc::c_uint & t;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add(3 as libc::c_int as
                                                    libc::c_uint & t) as
                    libc::c_int as libc::c_int
        } else {
            t =
                opp_bits_low & my_bits_low >> 7 as libc::c_int &
                    0x2040800 as libc::c_uint;
            my_bits_low |= t | t >> 7 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     4 as libc::c_int +
                                                         7 as libc::c_int |
                                                     t >>
                                                         4 as libc::c_int +
                                                             7 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 4 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_low & 0x10101000 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_high >>
                    4 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    4 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    4 as libc::c_int + 8 as libc::c_int * 6 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x10101010 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |= 0x10101000 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_low & my_bits_low >> 8 as libc::c_int &
                    0x10101000 as libc::c_uint;
            my_bits_low |= t | t >> 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     4 as libc::c_int +
                                                         8 as libc::c_int |
                                                     t >>
                                                         4 as libc::c_int +
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 4 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low >> 9 as libc::c_int &
                0x402000 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t >> 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 4 as libc::c_int +
                                                     9 as libc::c_int |
                                                 t >>
                                                     4 as libc::c_int +
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    my_bits_low |= 0x10 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_a4(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 24 as libc::c_int + 1 as libc::c_int
                              & 0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 24 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up right */
    if opp_bits_low &
           ((1 as libc::c_int) << 24 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 7 as libc::c_int &
                0x20400 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 24 as libc::c_int -
                                                     7 as libc::c_int |
                                                 t >>
                                                     24 as libc::c_int -
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                24 as libc::c_int + 8 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                24 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                24 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x1010101 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) << 24 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as libc::c_int &
                0x10100 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 24 as libc::c_int -
                                                     8 as libc::c_int |
                                                 t >>
                                                     24 as libc::c_int -
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down right */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                24 as libc::c_int + 9 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                24 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                24 as libc::c_int + 9 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x10080402 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    my_bits_low |= 0x1000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_h4(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 31 as libc::c_int - 6 as libc::c_int
                             & 0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 31 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down left */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                31 as libc::c_int + 7 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                31 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                31 as libc::c_int + 7 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x8102040 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                31 as libc::c_int + 8 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                31 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                31 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x80808080 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) << 31 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as libc::c_int &
                0x808000 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 31 as libc::c_int -
                                                     8 as libc::c_int |
                                                 t >>
                                                     31 as libc::c_int -
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up left */
    if opp_bits_low &
           ((1 as libc::c_int) << 31 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 9 as libc::c_int &
                0x402000 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 31 as libc::c_int -
                                                     9 as libc::c_int |
                                                 t >>
                                                     31 as libc::c_int -
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    my_bits_low |= 0x80000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_a5(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >> 0 as libc::c_int + 1 as libc::c_int
                              & 0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 0 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up right */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                0 as libc::c_int + 32 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                0 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                0 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x2040810 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as libc::c_int &
                0x10100 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 0 as libc::c_int +
                                                     8 as libc::c_int |
                                                 t >>
                                                     0 as libc::c_int +
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                0 as libc::c_int + 32 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                0 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                0 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x1010101 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Down right */
    if opp_bits_high &
           ((1 as libc::c_int) << 0 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 as libc::c_int &
                0x40200 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 0 as libc::c_int +
                                                     9 as libc::c_int |
                                                 t >>
                                                     0 as libc::c_int +
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    my_bits_high |= 0x1 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_h5(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 7 as libc::c_int - 6 as libc::c_int
                             & 0x3f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 7 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down left */
    if opp_bits_high &
           ((1 as libc::c_int) << 7 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 as libc::c_int &
                0x204000 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 7 as libc::c_int +
                                                     7 as libc::c_int |
                                                 t >>
                                                     7 as libc::c_int +
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) << 7 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as libc::c_int &
                0x808000 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 7 as libc::c_int +
                                                     8 as libc::c_int |
                                                 t >>
                                                     7 as libc::c_int +
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                7 as libc::c_int + 32 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                7 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                7 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x80808080 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up left */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                7 as libc::c_int + 32 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                7 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                7 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x40201008 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    my_bits_high |= 0x80 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_d8(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_high >> 27 as libc::c_int - 2 as libc::c_int &
                        0x3b as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        27 as libc::c_int -
                            3 as
                                libc::c_int).wrapping_add(24 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 27 as libc::c_int - 2 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 27 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x102040 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                ((my_bits_low <<
                      31 as libc::c_int -
                          (27 as libc::c_int + 32 as libc::c_int -
                               7 as libc::c_int * 4 as libc::c_int)) as
                     libc::c_int >> 31 as libc::c_int) as libc::c_uint;
            my_bits_high |= 0x102040 as libc::c_uint & t;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add(3 as libc::c_int as
                                                    libc::c_uint & t) as
                    libc::c_int as libc::c_int
        } else {
            t =
                opp_bits_high & my_bits_high << 7 as libc::c_int &
                    0x102040 as libc::c_uint;
            my_bits_high |= t | t << 7 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     27 as libc::c_int -
                                                         7 as libc::c_int |
                                                     t >>
                                                         27 as libc::c_int -
                                                             7 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 27 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x80808 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    27 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    27 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    27 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 6 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x8080808 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x80808 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as libc::c_int &
                    0x80808 as libc::c_uint;
            my_bits_high |= t | t << 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     27 as libc::c_int -
                                                         8 as libc::c_int |
                                                     t >>
                                                         27 as libc::c_int -
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 27 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high << 9 as libc::c_int &
                0x40200 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t << 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 27 as libc::c_int -
                                                     9 as libc::c_int |
                                                 t >>
                                                     27 as libc::c_int -
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    my_bits_high |= 0x8000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_e8(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_high >> 28 as libc::c_int - 3 as libc::c_int &
                        0x37 as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        28 as libc::c_int -
                            4 as
                                libc::c_int).wrapping_add(16 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 28 as libc::c_int - 3 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 28 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high << 7 as libc::c_int &
                0x204000 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t << 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 28 as libc::c_int -
                                                     7 as libc::c_int |
                                                 t >>
                                                     28 as libc::c_int -
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 28 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x101010 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                opp_bits_low >>
                    28 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (3 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    28 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    28 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 6 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 3 as libc::c_int) as usize] &
                    0x10101010 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |= 0x101010 as libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                opp_bits_high & my_bits_high << 8 as libc::c_int &
                    0x101010 as libc::c_uint;
            my_bits_high |= t | t << 8 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     28 as libc::c_int -
                                                         8 as libc::c_int |
                                                     t >>
                                                         28 as libc::c_int -
                                                             8 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 28 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        if !opp_bits_high & 0x80402 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            t =
                ((my_bits_low <<
                      31 as libc::c_int -
                          (28 as libc::c_int + 32 as libc::c_int -
                               9 as libc::c_int * 4 as libc::c_int)) as
                     libc::c_int >> 31 as libc::c_int) as libc::c_uint;
            my_bits_high |= 0x80402 as libc::c_uint & t;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add(3 as libc::c_int as
                                                    libc::c_uint & t) as
                    libc::c_int as libc::c_int
        } else {
            t =
                opp_bits_high & my_bits_high << 9 as libc::c_int &
                    0x80402 as libc::c_uint;
            my_bits_high |= t | t << 9 as libc::c_int;
            flipped =
                (flipped as
                     libc::c_uint).wrapping_add((t >>
                                                     28 as libc::c_int -
                                                         9 as libc::c_int |
                                                     t >>
                                                         28 as libc::c_int -
                                                             9 as libc::c_int
                                                                 *
                                                                 2 as
                                                                     libc::c_int
                                                             -
                                                             1 as libc::c_int)
                                                    &
                                                    3 as libc::c_int as
                                                        libc::c_uint) as
                    libc::c_int as libc::c_int
        }
    }
    my_bits_high |= 0x10000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_c2(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_low >> 10 as libc::c_int - 1 as libc::c_int &
                        0x3d as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        10 as libc::c_int -
                            2 as
                                libc::c_int).wrapping_add(28 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 10 as libc::c_int - 1 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    t =
        opp_bits_low & my_bits_low >> 7 as libc::c_int &
            ((1 as libc::c_int) << 10 as libc::c_int + 7 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            10 as libc::c_int +
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 10 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    10 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    10 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    10 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    10 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x4040404 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) <<
                         10 as libc::c_int + 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             10 as libc::c_int +
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    10 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 10 as libc::c_int + 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 10 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    10 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    10 as libc::c_int + 9 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    10 as libc::c_int + 9 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x804020 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) <<
                         10 as libc::c_int + 9 as libc::c_int |
                         (1 as libc::c_int) <<
                             10 as libc::c_int +
                                 9 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    10 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 10 as libc::c_int + 9 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    my_bits_low |= 0x400 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_f2(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_low >> 13 as libc::c_int - 4 as libc::c_int &
                    0x2f as libc::c_int as
                        libc::c_uint).wrapping_add(64 as libc::c_int as
                                                       libc::c_uint) as usize]
            as libc::c_uint;
    fl =
        f_flip[(t & my_bits_low >> 13 as libc::c_int - 5 as libc::c_int) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 13 as libc::c_int - 4 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 13 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    13 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    13 as libc::c_int + 7 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    13 as libc::c_int + 7 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x10204 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) <<
                         13 as libc::c_int + 7 as libc::c_int |
                         (1 as libc::c_int) <<
                             13 as libc::c_int +
                                 7 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    13 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 13 as libc::c_int + 7 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 13 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    13 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    13 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    13 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    13 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x20202020 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) <<
                         13 as libc::c_int + 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             13 as libc::c_int +
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    13 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 13 as libc::c_int + 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Down right */
    t =
        opp_bits_low & my_bits_low >> 9 as libc::c_int &
            ((1 as libc::c_int) << 13 as libc::c_int + 9 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            13 as libc::c_int +
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_low |= 0x2000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_b3(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 17 as libc::c_int + 1 as libc::c_int
                              & 0x1f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 17 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as libc::c_int &
            ((1 as libc::c_int) << 17 as libc::c_int - 7 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            17 as libc::c_int -
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 17 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                17 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                17 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                17 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x2020202 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 17 as libc::c_int + 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as libc::c_int &
            ((1 as libc::c_int) << 17 as libc::c_int - 8 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            17 as libc::c_int -
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 17 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                17 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                17 as libc::c_int + 9 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                17 as libc::c_int + 9 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x40201008 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 17 as libc::c_int + 9 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    my_bits_low |= 0x20000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_g3(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 22 as libc::c_int - 6 as libc::c_int
                             & 0x3e as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 22 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 22 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                22 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                22 as libc::c_int + 7 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                22 as libc::c_int + 7 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x2040810 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 22 as libc::c_int + 7 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 22 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                22 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                22 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                22 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x40404040 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 22 as libc::c_int + 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as libc::c_int &
            ((1 as libc::c_int) << 22 as libc::c_int - 8 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            22 as libc::c_int -
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as libc::c_int &
            ((1 as libc::c_int) << 22 as libc::c_int - 9 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            22 as libc::c_int -
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_low |= 0x400000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_b6(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >> 9 as libc::c_int + 1 as libc::c_int
                              & 0x1f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 9 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 9 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                9 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                9 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                9 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x8102040 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 9 as libc::c_int - 7 as libc::c_int) as
                    libc::c_uint;
            flipped += contig
        }
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as libc::c_int &
            ((1 as libc::c_int) << 9 as libc::c_int + 8 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            9 as libc::c_int +
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 9 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                9 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                9 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                9 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x2020202 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 9 as libc::c_int - 8 as libc::c_int) as
                    libc::c_uint;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as libc::c_int &
            ((1 as libc::c_int) << 9 as libc::c_int + 9 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            9 as libc::c_int +
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_high |= 0x200 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_g6(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 14 as libc::c_int - 6 as libc::c_int
                             & 0x3e as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 14 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as libc::c_int &
            ((1 as libc::c_int) << 14 as libc::c_int + 7 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            14 as libc::c_int +
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as libc::c_int &
            ((1 as libc::c_int) << 14 as libc::c_int + 8 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            14 as libc::c_int +
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 14 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                14 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                14 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                14 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x40404040 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 14 as libc::c_int - 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 14 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                14 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                14 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                14 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x10080402 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 14 as libc::c_int - 9 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    my_bits_high |= 0x4000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_c7(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_high >> 18 as libc::c_int - 1 as libc::c_int &
                        0x3d as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        18 as libc::c_int -
                            2 as
                                libc::c_int).wrapping_add(28 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 18 as libc::c_int - 1 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 18 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    18 as libc::c_int - 7 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    18 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    18 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x20408000 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         18 as libc::c_int - 7 as libc::c_int |
                         (1 as libc::c_int) <<
                             18 as libc::c_int -
                                 7 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    18 as libc::c_int - 7 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 18 as libc::c_int - 7 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 18 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    18 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    18 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    18 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    18 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x4040404 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         18 as libc::c_int - 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             18 as libc::c_int -
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    18 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 18 as libc::c_int - 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Up left */
    t =
        opp_bits_high & my_bits_high << 9 as libc::c_int &
            ((1 as libc::c_int) << 18 as libc::c_int - 9 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            18 as libc::c_int -
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_high |= 0x40000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_f7(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_high >> 21 as libc::c_int - 4 as libc::c_int &
                    0x2f as libc::c_int as
                        libc::c_uint).wrapping_add(64 as libc::c_int as
                                                       libc::c_uint) as usize]
            as libc::c_uint;
    fl =
        f_flip[(t & my_bits_high >> 21 as libc::c_int - 5 as libc::c_int) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 21 as libc::c_int - 4 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Up right */
    t =
        opp_bits_high & my_bits_high << 7 as libc::c_int &
            ((1 as libc::c_int) << 21 as libc::c_int - 7 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            21 as libc::c_int -
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 21 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    21 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    21 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    21 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    21 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x20202020 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         21 as libc::c_int - 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             21 as libc::c_int -
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    21 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 21 as libc::c_int - 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 21 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    21 as libc::c_int - 9 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    21 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    21 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x4020100 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         21 as libc::c_int - 9 as libc::c_int |
                         (1 as libc::c_int) <<
                             21 as libc::c_int -
                                 9 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    21 as libc::c_int - 9 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 21 as libc::c_int - 9 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    my_bits_high |= 0x200000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_d2(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_low >> 11 as libc::c_int - 2 as libc::c_int &
                        0x3b as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        11 as libc::c_int -
                            3 as
                                libc::c_int).wrapping_add(24 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 11 as libc::c_int - 2 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 11 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low &
                (my_bits_low >> 7 as libc::c_int |
                     my_bits_high << 32 as libc::c_int - 7 as libc::c_int) &
                0x2040000 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t >> 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 11 as libc::c_int +
                                                     7 as libc::c_int |
                                                 t >>
                                                     11 as libc::c_int +
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 11 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    11 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    11 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    11 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    11 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x8080808 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) <<
                         11 as libc::c_int + 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             11 as libc::c_int +
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    11 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 11 as libc::c_int + 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 11 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    11 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    11 as libc::c_int + 9 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t =
                t <<
                    11 as libc::c_int + 9 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int |
                    ((1 as libc::c_int) <<
                         11 as libc::c_int +
                             9 as libc::c_int * 3 as libc::c_int -
                             32 as libc::c_int) as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) <<
                         11 as libc::c_int + 9 as libc::c_int |
                         (1 as libc::c_int) <<
                             11 as libc::c_int +
                                 9 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    11 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 11 as libc::c_int + 9 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    my_bits_low |= 0x800 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_e2(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_low >> 12 as libc::c_int - 3 as libc::c_int &
                        0x37 as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        12 as libc::c_int -
                            4 as
                                libc::c_int).wrapping_add(16 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 12 as libc::c_int - 3 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 12 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    12 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    12 as libc::c_int + 7 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t =
                t <<
                    12 as libc::c_int + 7 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int |
                    ((1 as libc::c_int) <<
                         12 as libc::c_int +
                             7 as libc::c_int * 3 as libc::c_int -
                             32 as libc::c_int) as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) <<
                         12 as libc::c_int + 7 as libc::c_int |
                         (1 as libc::c_int) <<
                             12 as libc::c_int +
                                 7 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    12 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 12 as libc::c_int + 7 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 12 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_low &
               ((1 as libc::c_int) <<
                    12 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_high >>
                    12 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                        32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    12 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_high >>
                    12 as libc::c_int + 8 as libc::c_int * 5 as libc::c_int -
                        32 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                lsb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x10101010 as libc::c_uint;
            if my_bits_high & t != 0 {
                my_bits_high |= t;
                my_bits_low |=
                    ((1 as libc::c_int) <<
                         12 as libc::c_int + 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             12 as libc::c_int +
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_low >>
                    12 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_low |= t << 12 as libc::c_int + 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 12 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low &
                (my_bits_low >> 9 as libc::c_int |
                     my_bits_high << 32 as libc::c_int - 9 as libc::c_int) &
                0x40200000 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t >> 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 12 as libc::c_int +
                                                     9 as libc::c_int |
                                                 t >>
                                                     12 as libc::c_int +
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    my_bits_low |= 0x1000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_b4(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_low >> 25 as libc::c_int + 1 as libc::c_int
                              & 0x1f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 25 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up right */
    if opp_bits_low &
           ((1 as libc::c_int) << 25 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 7 as libc::c_int &
                0x40800 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 25 as libc::c_int -
                                                     7 as libc::c_int |
                                                 t >>
                                                     25 as libc::c_int -
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                25 as libc::c_int + 8 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                25 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                25 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x2020202 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) << 25 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as libc::c_int &
                0x20200 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 25 as libc::c_int -
                                                     8 as libc::c_int |
                                                 t >>
                                                     25 as libc::c_int -
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down right */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                25 as libc::c_int + 9 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                25 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                25 as libc::c_int + 9 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x20100804 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    my_bits_low |= 0x2000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_g4(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_low >> 30 as libc::c_int - 6 as libc::c_int
                             & 0x3e as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 30 as libc::c_int;
    t =
        (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_low |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down left */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                30 as libc::c_int + 7 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                30 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                30 as libc::c_int + 7 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x4081020 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                30 as libc::c_int + 8 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                30 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                30 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x40404040 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) << 30 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as libc::c_int &
                0x404000 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 30 as libc::c_int -
                                                     8 as libc::c_int |
                                                 t >>
                                                     30 as libc::c_int -
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up left */
    if opp_bits_low &
           ((1 as libc::c_int) << 30 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 9 as libc::c_int &
                0x201000 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 30 as libc::c_int -
                                                     9 as libc::c_int |
                                                 t >>
                                                     30 as libc::c_int -
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    my_bits_low |= 0x40000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_b5(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right */
    contig =
        right_contiguous[(opp_bits_high >> 1 as libc::c_int + 1 as libc::c_int
                              & 0x1f as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl = right_flip[contig as usize] << 1 as libc::c_int + 1 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Up right */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                1 as libc::c_int + 32 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                1 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                1 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x4081020 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) << 1 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as libc::c_int &
                0x20200 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 1 as libc::c_int +
                                                     8 as libc::c_int |
                                                 t >>
                                                     1 as libc::c_int +
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                1 as libc::c_int + 32 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                1 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                1 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x2020202 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Down right */
    if opp_bits_high &
           ((1 as libc::c_int) << 1 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 as libc::c_int &
                0x80400 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 1 as libc::c_int +
                                                     9 as libc::c_int |
                                                 t >>
                                                     1 as libc::c_int +
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    my_bits_high |= 0x2 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_g5(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Left */
    contig =
        left_contiguous[(opp_bits_high >> 6 as libc::c_int - 6 as libc::c_int
                             & 0x3e as libc::c_int as libc::c_uint) as usize]
            as libc::c_int;
    fl =
        (0x80000000 as libc::c_uint as libc::c_int >> contig) as libc::c_uint
            >> 32 as libc::c_int - 6 as libc::c_int;
    t =
        (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
            libc::c_uint;
    my_bits_high |= fl & t;
    flipped = (contig as libc::c_uint & t) as libc::c_int;
    /* Down left */
    if opp_bits_high &
           ((1 as libc::c_int) << 6 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 as libc::c_int &
                0x102000 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 6 as libc::c_int +
                                                     7 as libc::c_int |
                                                 t >>
                                                     6 as libc::c_int +
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) << 6 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as libc::c_int &
                0x404000 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 6 as libc::c_int +
                                                     8 as libc::c_int |
                                                 t >>
                                                     6 as libc::c_int +
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                6 as libc::c_int + 32 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                6 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                6 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x40404040 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up left */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                6 as libc::c_int + 32 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                6 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                6 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x20100804 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    my_bits_high |= 0x40 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_d7(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_high >> 19 as libc::c_int - 2 as libc::c_int &
                        0x3b as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        19 as libc::c_int -
                            3 as
                                libc::c_int).wrapping_add(24 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 19 as libc::c_int - 2 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 19 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    19 as libc::c_int - 7 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    19 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t =
                t <<
                    19 as libc::c_int + 32 as libc::c_int -
                        7 as libc::c_int * 4 as libc::c_int |
                    ((1 as libc::c_int) <<
                         19 as libc::c_int + 32 as libc::c_int -
                             7 as libc::c_int * 3 as libc::c_int) as
                        libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         19 as libc::c_int - 7 as libc::c_int |
                         (1 as libc::c_int) <<
                             19 as libc::c_int -
                                 7 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    19 as libc::c_int - 7 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 19 as libc::c_int - 7 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 19 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    19 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    19 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    19 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    19 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x8080808 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         19 as libc::c_int - 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             19 as libc::c_int -
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    19 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 19 as libc::c_int - 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 19 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high &
                (my_bits_high << 9 as libc::c_int |
                     my_bits_low >> 32 as libc::c_int - 9 as libc::c_int) &
                0x402 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t << 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 19 as libc::c_int -
                                                     9 as libc::c_int |
                                                 t >>
                                                     19 as libc::c_int -
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    my_bits_high |= 0x80000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_e7(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_high >> 20 as libc::c_int - 3 as libc::c_int &
                        0x37 as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        20 as libc::c_int -
                            4 as
                                libc::c_int).wrapping_add(16 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 20 as libc::c_int - 3 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 20 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high &
                (my_bits_high << 7 as libc::c_int |
                     my_bits_low >> 32 as libc::c_int - 7 as libc::c_int) &
                0x2040 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t << 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 20 as libc::c_int -
                                                     7 as libc::c_int |
                                                 t >>
                                                     20 as libc::c_int -
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 20 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    20 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    20 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    20 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 4 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t &=
                opp_bits_low >>
                    20 as libc::c_int + 32 as libc::c_int -
                        8 as libc::c_int * 5 as libc::c_int;
            contig =
                (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int;
            t =
                msb_mask[(contig - 2 as libc::c_int) as usize] &
                    0x10101010 as libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         20 as libc::c_int - 8 as libc::c_int |
                         (1 as libc::c_int) <<
                             20 as libc::c_int -
                                 8 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    20 as libc::c_int - 8 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 20 as libc::c_int - 8 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 20 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        if opp_bits_high &
               ((1 as libc::c_int) <<
                    20 as libc::c_int - 9 as libc::c_int * 2 as libc::c_int)
                   as libc::c_uint != 0 {
            t =
                opp_bits_low >>
                    20 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 3 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            contig =
                (2 as libc::c_int as libc::c_uint).wrapping_add(t) as
                    libc::c_int;
            t =
                t <<
                    20 as libc::c_int + 32 as libc::c_int -
                        9 as libc::c_int * 4 as libc::c_int |
                    ((1 as libc::c_int) <<
                         20 as libc::c_int + 32 as libc::c_int -
                             9 as libc::c_int * 3 as libc::c_int) as
                        libc::c_uint;
            if my_bits_low & t != 0 {
                my_bits_low |= t;
                my_bits_high |=
                    ((1 as libc::c_int) <<
                         20 as libc::c_int - 9 as libc::c_int |
                         (1 as libc::c_int) <<
                             20 as libc::c_int -
                                 9 as libc::c_int * 2 as libc::c_int) as
                        libc::c_uint;
                flipped += contig
            }
        } else {
            t =
                my_bits_high >>
                    20 as libc::c_int - 9 as libc::c_int * 2 as libc::c_int &
                    1 as libc::c_int as libc::c_uint;
            my_bits_high |= t << 20 as libc::c_int - 9 as libc::c_int;
            flipped =
                (flipped as libc::c_uint).wrapping_add(t) as libc::c_int as
                    libc::c_int
        }
    }
    my_bits_high |= 0x100000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_c3(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_low >> 18 as libc::c_int - 1 as libc::c_int &
                        0x3d as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        18 as libc::c_int -
                            2 as
                                libc::c_int).wrapping_add(28 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 18 as libc::c_int - 1 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    t =
        opp_bits_low & my_bits_high << 32 as libc::c_int - 7 as libc::c_int &
            ((1 as libc::c_int) << 18 as libc::c_int + 7 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            18 as libc::c_int +
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as libc::c_int &
            ((1 as libc::c_int) << 18 as libc::c_int - 7 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            18 as libc::c_int -
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 18 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                18 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                18 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                18 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x4040404 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 18 as libc::c_int + 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as libc::c_int &
            ((1 as libc::c_int) << 18 as libc::c_int - 8 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            18 as libc::c_int -
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 18 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                18 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                18 as libc::c_int + 9 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                18 as libc::c_int + 9 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x80402010 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 18 as libc::c_int + 9 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as libc::c_int &
            ((1 as libc::c_int) << 18 as libc::c_int - 9 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            18 as libc::c_int -
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_low |= 0x40000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_f3(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_low >> 21 as libc::c_int - 4 as libc::c_int &
                    0x2f as libc::c_int as
                        libc::c_uint).wrapping_add(64 as libc::c_int as
                                                       libc::c_uint) as usize]
            as libc::c_uint;
    fl =
        f_flip[(t & my_bits_low >> 21 as libc::c_int - 5 as libc::c_int) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 21 as libc::c_int - 4 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 21 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                21 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                21 as libc::c_int + 7 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                21 as libc::c_int + 7 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x1020408 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 21 as libc::c_int + 7 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as libc::c_int &
            ((1 as libc::c_int) << 21 as libc::c_int - 7 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            21 as libc::c_int -
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 21 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                21 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                21 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                21 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x20202020 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 21 as libc::c_int + 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as libc::c_int &
            ((1 as libc::c_int) << 21 as libc::c_int - 8 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            21 as libc::c_int -
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down right */
    t =
        opp_bits_low & my_bits_high << 32 as libc::c_int - 9 as libc::c_int &
            ((1 as libc::c_int) << 21 as libc::c_int + 9 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            21 as libc::c_int +
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as libc::c_int &
            ((1 as libc::c_int) << 21 as libc::c_int - 9 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            21 as libc::c_int -
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_low |= 0x200000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_c6(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_high >> 10 as libc::c_int - 1 as libc::c_int &
                        0x3d as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        10 as libc::c_int -
                            2 as
                                libc::c_int).wrapping_add(28 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 10 as libc::c_int - 1 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as libc::c_int &
            ((1 as libc::c_int) << 10 as libc::c_int + 7 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            10 as libc::c_int +
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 10 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                10 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                10 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                10 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x10204080 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 10 as libc::c_int - 7 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as libc::c_int &
            ((1 as libc::c_int) << 10 as libc::c_int + 8 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            10 as libc::c_int +
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 10 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                10 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                10 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                10 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x4040404 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 10 as libc::c_int - 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as libc::c_int &
            ((1 as libc::c_int) << 10 as libc::c_int + 9 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            10 as libc::c_int +
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up left */
    t =
        opp_bits_high & my_bits_low >> 32 as libc::c_int - 9 as libc::c_int &
            ((1 as libc::c_int) << 10 as libc::c_int - 9 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            10 as libc::c_int -
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_high |= 0x400 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_f6(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_high >> 13 as libc::c_int - 4 as libc::c_int &
                    0x2f as libc::c_int as
                        libc::c_uint).wrapping_add(64 as libc::c_int as
                                                       libc::c_uint) as usize]
            as libc::c_uint;
    fl =
        f_flip[(t & my_bits_high >> 13 as libc::c_int - 5 as libc::c_int) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 13 as libc::c_int - 4 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as libc::c_int &
            ((1 as libc::c_int) << 13 as libc::c_int + 7 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            13 as libc::c_int +
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up right */
    t =
        opp_bits_high & my_bits_low >> 32 as libc::c_int - 7 as libc::c_int &
            ((1 as libc::c_int) << 13 as libc::c_int - 7 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            13 as libc::c_int -
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as libc::c_int &
            ((1 as libc::c_int) << 13 as libc::c_int + 8 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            13 as libc::c_int +
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 13 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                13 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                13 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                13 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x20202020 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 13 as libc::c_int - 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as libc::c_int &
            ((1 as libc::c_int) << 13 as libc::c_int + 9 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            13 as libc::c_int +
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 13 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                13 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                13 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                13 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x8040201 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 13 as libc::c_int - 9 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    my_bits_high |= 0x2000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_d3(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_low >> 19 as libc::c_int - 2 as libc::c_int &
                        0x3b as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        19 as libc::c_int -
                            3 as
                                libc::c_int).wrapping_add(24 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 19 as libc::c_int - 2 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 19 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        fl =
            my_bits_high << 32 as libc::c_int - 7 as libc::c_int &
                ((1 as libc::c_int) << 19 as libc::c_int + 7 as libc::c_int)
                    as libc::c_uint;
        t =
            opp_bits_high & my_bits_high >> 7 as libc::c_int &
                ((1 as libc::c_int) <<
                     19 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int -
                         32 as libc::c_int) as libc::c_uint;
        my_bits_low |=
            fl.wrapping_add(t << 32 as libc::c_int - 7 as libc::c_int);
        my_bits_high |= t;
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add(fl >>
                                                19 as libc::c_int +
                                                    7 as libc::c_int |
                                                t >>
                                                    19 as libc::c_int +
                                                        7 as libc::c_int *
                                                            2 as libc::c_int -
                                                        32 as libc::c_int -
                                                        1 as libc::c_int) as
                libc::c_int as libc::c_int
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as libc::c_int &
            ((1 as libc::c_int) << 19 as libc::c_int - 7 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            19 as libc::c_int -
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 19 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                19 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                19 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                19 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x8080808 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 19 as libc::c_int + 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as libc::c_int &
            ((1 as libc::c_int) << 19 as libc::c_int - 8 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            19 as libc::c_int -
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 19 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                19 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                19 as libc::c_int + 9 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x804020 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 19 as libc::c_int + 9 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as libc::c_int &
            ((1 as libc::c_int) << 19 as libc::c_int - 9 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            19 as libc::c_int -
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_low |= 0x80000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_e3(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_low >> 20 as libc::c_int - 3 as libc::c_int &
                        0x37 as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        20 as libc::c_int -
                            4 as
                                libc::c_int).wrapping_add(16 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 20 as libc::c_int - 3 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_low &
           ((1 as libc::c_int) << 20 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                20 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                20 as libc::c_int + 7 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x10204 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 20 as libc::c_int + 7 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as libc::c_int &
            ((1 as libc::c_int) << 20 as libc::c_int - 7 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            20 as libc::c_int -
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down */
    if opp_bits_low &
           ((1 as libc::c_int) << 20 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                20 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                20 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_high >>
                20 as libc::c_int + 8 as libc::c_int * 4 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            lsb_mask[(contig - 1 as libc::c_int) as usize] &
                0x10101010 as libc::c_uint;
        if my_bits_high & t != 0 {
            my_bits_high |= t;
            my_bits_low |=
                ((1 as libc::c_int) << 20 as libc::c_int + 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Up */
    t =
        opp_bits_low & my_bits_low << 8 as libc::c_int &
            ((1 as libc::c_int) << 20 as libc::c_int - 8 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            20 as libc::c_int -
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down right */
    if opp_bits_low &
           ((1 as libc::c_int) << 20 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        fl =
            my_bits_high << 32 as libc::c_int - 9 as libc::c_int &
                ((1 as libc::c_int) << 20 as libc::c_int + 9 as libc::c_int)
                    as libc::c_uint;
        t =
            opp_bits_high & my_bits_high >> 9 as libc::c_int &
                ((1 as libc::c_int) <<
                     20 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int -
                         32 as libc::c_int) as libc::c_uint;
        my_bits_low |=
            fl.wrapping_add(t << 32 as libc::c_int - 9 as libc::c_int);
        my_bits_high |= t;
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add(fl >>
                                                20 as libc::c_int +
                                                    9 as libc::c_int |
                                                t >>
                                                    20 as libc::c_int +
                                                        9 as libc::c_int *
                                                            2 as libc::c_int -
                                                        32 as libc::c_int -
                                                        1 as libc::c_int) as
                libc::c_int as libc::c_int
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as libc::c_int &
            ((1 as libc::c_int) << 20 as libc::c_int - 9 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            20 as libc::c_int -
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_low |= 0x100000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_c4(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_low >> 26 as libc::c_int - 1 as libc::c_int &
                        0x3d as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        26 as libc::c_int -
                            2 as
                                libc::c_int).wrapping_add(28 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 26 as libc::c_int - 1 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as libc::c_int &
            ((1 as libc::c_int) <<
                 26 as libc::c_int + 7 as libc::c_int - 32 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            26 as libc::c_int +
                                                7 as libc::c_int -
                                                32 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up right */
    if opp_bits_low &
           ((1 as libc::c_int) << 26 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 7 as libc::c_int &
                0x81000 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 26 as libc::c_int -
                                                     7 as libc::c_int |
                                                 t >>
                                                     26 as libc::c_int -
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                26 as libc::c_int + 8 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                26 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                26 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x4040404 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) << 26 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as libc::c_int &
                0x40400 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 26 as libc::c_int -
                                                     8 as libc::c_int |
                                                 t >>
                                                     26 as libc::c_int -
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down right */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                26 as libc::c_int + 9 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                26 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                26 as libc::c_int + 9 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x40201008 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as libc::c_int &
            ((1 as libc::c_int) << 26 as libc::c_int - 9 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            26 as libc::c_int -
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_low |= 0x4000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_f4(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_low >> 29 as libc::c_int - 4 as libc::c_int &
                    0x2f as libc::c_int as
                        libc::c_uint).wrapping_add(64 as libc::c_int as
                                                       libc::c_uint) as usize]
            as libc::c_uint;
    fl =
        f_flip[(t & my_bits_low >> 29 as libc::c_int - 5 as libc::c_int) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 29 as libc::c_int - 4 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                29 as libc::c_int + 7 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                29 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                29 as libc::c_int + 7 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x2040810 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as libc::c_int &
            ((1 as libc::c_int) << 29 as libc::c_int - 7 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            29 as libc::c_int -
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                29 as libc::c_int + 8 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                29 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                29 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x20202020 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) << 29 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as libc::c_int &
                0x202000 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 29 as libc::c_int -
                                                     8 as libc::c_int |
                                                 t >>
                                                     29 as libc::c_int -
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as libc::c_int &
            ((1 as libc::c_int) <<
                 29 as libc::c_int + 9 as libc::c_int - 32 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            29 as libc::c_int +
                                                9 as libc::c_int -
                                                32 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up left */
    if opp_bits_low &
           ((1 as libc::c_int) << 29 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 9 as libc::c_int &
                0x100800 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 29 as libc::c_int -
                                                     9 as libc::c_int |
                                                 t >>
                                                     29 as libc::c_int -
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    my_bits_low |= 0x20000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_c5(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        c_frontier[(opp_bits_high >> 2 as libc::c_int - 1 as libc::c_int &
                        0x3d as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        2 as libc::c_int -
                            2 as
                                libc::c_int).wrapping_add(28 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 2 as libc::c_int - 1 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as libc::c_int &
            ((1 as libc::c_int) << 2 as libc::c_int + 7 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            2 as libc::c_int +
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up right */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                2 as libc::c_int + 32 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                2 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                2 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x8102040 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) << 2 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as libc::c_int &
                0x40400 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 2 as libc::c_int +
                                                     8 as libc::c_int |
                                                 t >>
                                                     2 as libc::c_int +
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                2 as libc::c_int + 32 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                2 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                2 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x4040404 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Down right */
    if opp_bits_high &
           ((1 as libc::c_int) << 2 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 as libc::c_int &
                0x100800 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 2 as libc::c_int +
                                                     9 as libc::c_int |
                                                 t >>
                                                     2 as libc::c_int +
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up left */
    t =
        opp_bits_low & my_bits_low << 9 as libc::c_int &
            ((1 as libc::c_int) <<
                 2 as libc::c_int + 32 as libc::c_int - 9 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            2 as libc::c_int +
                                                32 as libc::c_int -
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    my_bits_high |= 0x4 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_f5(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        f_flip[(opp_bits_high >> 5 as libc::c_int - 4 as libc::c_int &
                    0x2f as libc::c_int as
                        libc::c_uint).wrapping_add(64 as libc::c_int as
                                                       libc::c_uint) as usize]
            as libc::c_uint;
    fl =
        f_flip[(t & my_bits_high >> 5 as libc::c_int - 5 as libc::c_int) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 5 as libc::c_int - 4 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_high &
           ((1 as libc::c_int) << 5 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 as libc::c_int &
                0x81000 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 5 as libc::c_int +
                                                     7 as libc::c_int |
                                                 t >>
                                                     5 as libc::c_int +
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up right */
    t =
        opp_bits_low & my_bits_low << 7 as libc::c_int &
            ((1 as libc::c_int) <<
                 5 as libc::c_int + 32 as libc::c_int - 7 as libc::c_int) as
                libc::c_uint;
    my_bits_low |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            5 as libc::c_int +
                                                32 as libc::c_int -
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) << 5 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as libc::c_int &
                0x202000 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 5 as libc::c_int +
                                                     8 as libc::c_int |
                                                 t >>
                                                     5 as libc::c_int +
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                5 as libc::c_int + 32 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                5 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                5 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x20202020 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as libc::c_int &
            ((1 as libc::c_int) << 5 as libc::c_int + 9 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            5 as libc::c_int +
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up left */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                5 as libc::c_int + 32 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                5 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                5 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x10080402 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    my_bits_high |= 0x20 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_d6(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_high >> 11 as libc::c_int - 2 as libc::c_int &
                        0x3b as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        11 as libc::c_int -
                            3 as
                                libc::c_int).wrapping_add(24 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 11 as libc::c_int - 2 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as libc::c_int &
            ((1 as libc::c_int) << 11 as libc::c_int + 7 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            11 as libc::c_int +
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 11 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                11 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                11 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x20408000 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 11 as libc::c_int - 7 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as libc::c_int &
            ((1 as libc::c_int) << 11 as libc::c_int + 8 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            11 as libc::c_int +
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 11 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                11 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                11 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                11 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x8080808 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 11 as libc::c_int - 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as libc::c_int &
            ((1 as libc::c_int) << 11 as libc::c_int + 9 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            11 as libc::c_int +
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 11 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        fl =
            my_bits_low >> 32 as libc::c_int - 9 as libc::c_int &
                ((1 as libc::c_int) << 11 as libc::c_int - 9 as libc::c_int)
                    as libc::c_uint;
        t =
            opp_bits_low & my_bits_low << 9 as libc::c_int &
                ((1 as libc::c_int) <<
                     11 as libc::c_int + 32 as libc::c_int -
                         9 as libc::c_int * 2 as libc::c_int) as libc::c_uint;
        my_bits_high |=
            fl.wrapping_add(t >> 32 as libc::c_int - 9 as libc::c_int);
        my_bits_low |= t;
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add(fl >>
                                                11 as libc::c_int -
                                                    9 as libc::c_int |
                                                t >>
                                                    11 as libc::c_int +
                                                        32 as libc::c_int -
                                                        9 as libc::c_int *
                                                            2 as libc::c_int -
                                                        1 as libc::c_int) as
                libc::c_int as libc::c_int
    }
    my_bits_high |= 0x800 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_e6(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_high >> 12 as libc::c_int - 3 as libc::c_int &
                        0x37 as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        12 as libc::c_int -
                            4 as
                                libc::c_int).wrapping_add(16 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 12 as libc::c_int - 3 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    t =
        opp_bits_high & my_bits_high >> 7 as libc::c_int &
            ((1 as libc::c_int) << 12 as libc::c_int + 7 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            12 as libc::c_int +
                                                7 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up right */
    if opp_bits_high &
           ((1 as libc::c_int) << 12 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        fl =
            my_bits_low >> 32 as libc::c_int - 7 as libc::c_int &
                ((1 as libc::c_int) << 12 as libc::c_int - 7 as libc::c_int)
                    as libc::c_uint;
        t =
            opp_bits_low & my_bits_low << 7 as libc::c_int &
                ((1 as libc::c_int) <<
                     12 as libc::c_int + 32 as libc::c_int -
                         7 as libc::c_int * 2 as libc::c_int) as libc::c_uint;
        my_bits_high |=
            fl.wrapping_add(t >> 32 as libc::c_int - 7 as libc::c_int);
        my_bits_low |= t;
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add(fl >>
                                                12 as libc::c_int -
                                                    7 as libc::c_int |
                                                t >>
                                                    12 as libc::c_int +
                                                        32 as libc::c_int -
                                                        7 as libc::c_int *
                                                            2 as libc::c_int -
                                                        1 as libc::c_int) as
                libc::c_int as libc::c_int
    }
    /* Down */
    t =
        opp_bits_high & my_bits_high >> 8 as libc::c_int &
            ((1 as libc::c_int) << 12 as libc::c_int + 8 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            12 as libc::c_int +
                                                8 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up */
    if opp_bits_high &
           ((1 as libc::c_int) << 12 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                12 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                12 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t &=
            opp_bits_low >>
                12 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 4 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x10101010 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 12 as libc::c_int - 8 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    /* Down right */
    t =
        opp_bits_high & my_bits_high >> 9 as libc::c_int &
            ((1 as libc::c_int) << 12 as libc::c_int + 9 as libc::c_int) as
                libc::c_uint;
    my_bits_high |= t;
    flipped =
        (flipped as
             libc::c_uint).wrapping_add(t >>
                                            12 as libc::c_int +
                                                9 as libc::c_int) as
            libc::c_int as libc::c_int;
    /* Up left */
    if opp_bits_high &
           ((1 as libc::c_int) << 12 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                12 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                12 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        t =
            msb_mask[(contig - 1 as libc::c_int) as usize] &
                0x4020100 as libc::c_uint;
        if my_bits_low & t != 0 {
            my_bits_low |= t;
            my_bits_high |=
                ((1 as libc::c_int) << 12 as libc::c_int - 9 as libc::c_int)
                    as libc::c_uint;
            flipped += contig
        }
    }
    my_bits_high |= 0x1000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_d4(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_low >> 27 as libc::c_int - 2 as libc::c_int &
                        0x3b as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        27 as libc::c_int -
                            3 as
                                libc::c_int).wrapping_add(24 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 27 as libc::c_int - 2 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    t =
        opp_bits_high &
            (opp_bits_high << 7 as libc::c_int |
                 ((1 as libc::c_int) <<
                      27 as libc::c_int + 7 as libc::c_int -
                          32 as libc::c_int) as libc::c_uint) &
            my_bits_high >> 7 as libc::c_int & 0x204 as libc::c_uint;
    my_bits_high |= t.wrapping_add(t >> 7 as libc::c_int);
    flipped =
        (flipped as
             libc::c_uint).wrapping_add((t >>
                                             27 as libc::c_int +
                                                 7 as libc::c_int -
                                                 32 as libc::c_int |
                                             t >>
                                                 27 as libc::c_int +
                                                     7 as libc::c_int *
                                                         2 as libc::c_int -
                                                     32 as libc::c_int -
                                                     1 as libc::c_int) &
                                            3 as libc::c_int as libc::c_uint)
            as libc::c_int as libc::c_int;
    /* Up right */
    if opp_bits_low &
           ((1 as libc::c_int) << 27 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 7 as libc::c_int &
                0x102000 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 27 as libc::c_int -
                                                     7 as libc::c_int |
                                                 t >>
                                                     27 as libc::c_int -
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                27 as libc::c_int + 8 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                27 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                27 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x8080808 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) << 27 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as libc::c_int &
                0x80800 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 27 as libc::c_int -
                                                     8 as libc::c_int |
                                                 t >>
                                                     27 as libc::c_int -
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down right */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                27 as libc::c_int + 9 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                27 as libc::c_int + 9 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                27 as libc::c_int + 9 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x80402010 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up left */
    if opp_bits_low &
           ((1 as libc::c_int) << 27 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 9 as libc::c_int &
                0x40200 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 27 as libc::c_int -
                                                     9 as libc::c_int |
                                                 t >>
                                                     27 as libc::c_int -
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    my_bits_low |= 0x8000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_e4(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_low >> 28 as libc::c_int - 3 as libc::c_int &
                        0x37 as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_low >>
                        28 as libc::c_int -
                            4 as
                                libc::c_int).wrapping_add(16 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_low |= fl << 28 as libc::c_int - 3 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                28 as libc::c_int + 7 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                28 as libc::c_int + 7 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                28 as libc::c_int + 7 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x1020408 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up right */
    if opp_bits_low &
           ((1 as libc::c_int) << 28 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 7 as libc::c_int &
                0x204000 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 28 as libc::c_int -
                                                     7 as libc::c_int |
                                                 t >>
                                                     28 as libc::c_int -
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) <<
                28 as libc::c_int + 8 as libc::c_int - 32 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high >>
                28 as libc::c_int + 8 as libc::c_int * 2 as libc::c_int -
                    32 as libc::c_int & 1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_high >>
                28 as libc::c_int + 8 as libc::c_int * 3 as libc::c_int -
                    32 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = lsb_mask[contig as usize] & 0x10101010 as libc::c_uint;
        t =
            (-((my_bits_high & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_high |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) << 28 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 8 as libc::c_int &
                0x101000 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 28 as libc::c_int -
                                                     8 as libc::c_int |
                                                 t >>
                                                     28 as libc::c_int -
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Down right */
    t =
        opp_bits_high &
            (opp_bits_high << 9 as libc::c_int |
                 ((1 as libc::c_int) <<
                      28 as libc::c_int + 9 as libc::c_int -
                          32 as libc::c_int) as libc::c_uint) &
            my_bits_high >> 9 as libc::c_int & 0x4020 as libc::c_uint;
    my_bits_high |= t.wrapping_add(t >> 9 as libc::c_int);
    flipped =
        (flipped as
             libc::c_uint).wrapping_add((t >>
                                             28 as libc::c_int +
                                                 9 as libc::c_int -
                                                 32 as libc::c_int |
                                             t >>
                                                 28 as libc::c_int +
                                                     9 as libc::c_int *
                                                         2 as libc::c_int -
                                                     32 as libc::c_int -
                                                     1 as libc::c_int) &
                                            3 as libc::c_int as libc::c_uint)
            as libc::c_int as libc::c_int;
    /* Up left */
    if opp_bits_low &
           ((1 as libc::c_int) << 28 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low & my_bits_low << 9 as libc::c_int &
                0x80400 as libc::c_uint;
        my_bits_low |= t.wrapping_add(t << 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 28 as libc::c_int -
                                                     9 as libc::c_int |
                                                 t >>
                                                     28 as libc::c_int -
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    my_bits_low |= 0x10000000 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_d5(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        d_frontier[(opp_bits_high >> 3 as libc::c_int - 2 as libc::c_int &
                        0x3b as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        3 as libc::c_int -
                            3 as
                                libc::c_int).wrapping_add(24 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 3 as libc::c_int - 2 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_high &
           ((1 as libc::c_int) << 3 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 as libc::c_int &
                0x20400 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 3 as libc::c_int +
                                                     7 as libc::c_int |
                                                 t >>
                                                     3 as libc::c_int +
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up right */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                3 as libc::c_int + 32 as libc::c_int - 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                3 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                3 as libc::c_int + 32 as libc::c_int -
                    7 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x10204080 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) << 3 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as libc::c_int &
                0x80800 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 3 as libc::c_int +
                                                     8 as libc::c_int |
                                                 t >>
                                                     3 as libc::c_int +
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                3 as libc::c_int + 32 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                3 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                3 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x8080808 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Down right */
    if opp_bits_high &
           ((1 as libc::c_int) << 3 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 as libc::c_int &
                0x201000 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 3 as libc::c_int +
                                                     9 as libc::c_int |
                                                 t >>
                                                     3 as libc::c_int +
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up left */
    t =
        opp_bits_low &
            (opp_bits_low >> 9 as libc::c_int |
                 ((1 as libc::c_int) <<
                      3 as libc::c_int + 32 as libc::c_int - 9 as libc::c_int)
                     as libc::c_uint) & my_bits_low << 9 as libc::c_int &
            0x4020000 as libc::c_uint;
    my_bits_low |= t.wrapping_add(t << 9 as libc::c_int);
    flipped =
        (flipped as
             libc::c_uint).wrapping_add((t >>
                                             3 as libc::c_int +
                                                 32 as libc::c_int -
                                                 9 as libc::c_int |
                                             t >>
                                                 3 as libc::c_int +
                                                     32 as libc::c_int -
                                                     9 as libc::c_int *
                                                         2 as libc::c_int -
                                                     1 as libc::c_int) &
                                            3 as libc::c_int as libc::c_uint)
            as libc::c_int as libc::c_int;
    my_bits_high |= 0x8 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
unsafe extern "C" fn TestFlips_bitboard_e5(mut my_bits_high: libc::c_uint,
                                           mut my_bits_low: libc::c_uint,
                                           mut opp_bits_high: libc::c_uint,
                                           mut opp_bits_low: libc::c_uint)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut contig: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut fl: libc::c_uint = 0;
    /* Right / Left */
    t =
        e_frontier[(opp_bits_high >> 4 as libc::c_int - 3 as libc::c_int &
                        0x37 as libc::c_int as libc::c_uint) as usize] as
            libc::c_uint;
    fl =
        f_flip[(t &
                    my_bits_high >>
                        4 as libc::c_int -
                            4 as
                                libc::c_int).wrapping_add(16 as libc::c_int as
                                                              libc::c_uint) as
                   usize] as libc::c_uint;
    my_bits_high |= fl << 4 as libc::c_int - 3 as libc::c_int;
    flipped = pop_count[fl as usize] as libc::c_int;
    /* Down left */
    if opp_bits_high &
           ((1 as libc::c_int) << 4 as libc::c_int + 7 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 7 as libc::c_int &
                0x40800 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 7 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 4 as libc::c_int +
                                                     7 as libc::c_int |
                                                 t >>
                                                     4 as libc::c_int +
                                                         7 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up right */
    t =
        opp_bits_low &
            (opp_bits_low >> 7 as libc::c_int |
                 ((1 as libc::c_int) <<
                      4 as libc::c_int + 32 as libc::c_int - 7 as libc::c_int)
                     as libc::c_uint) & my_bits_low << 7 as libc::c_int &
            0x20400000 as libc::c_uint;
    my_bits_low |= t.wrapping_add(t << 7 as libc::c_int);
    flipped =
        (flipped as
             libc::c_uint).wrapping_add((t >>
                                             4 as libc::c_int +
                                                 32 as libc::c_int -
                                                 7 as libc::c_int |
                                             t >>
                                                 4 as libc::c_int +
                                                     32 as libc::c_int -
                                                     7 as libc::c_int *
                                                         2 as libc::c_int -
                                                     1 as libc::c_int) &
                                            3 as libc::c_int as libc::c_uint)
            as libc::c_int as libc::c_int;
    /* Down */
    if opp_bits_high &
           ((1 as libc::c_int) << 4 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 8 as libc::c_int &
                0x101000 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 8 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 4 as libc::c_int +
                                                     8 as libc::c_int |
                                                 t >>
                                                     4 as libc::c_int +
                                                         8 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                4 as libc::c_int + 32 as libc::c_int - 8 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                4 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                4 as libc::c_int + 32 as libc::c_int -
                    8 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x10101010 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    /* Down right */
    if opp_bits_high &
           ((1 as libc::c_int) << 4 as libc::c_int + 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_high & my_bits_high >> 9 as libc::c_int &
                0x402000 as libc::c_uint;
        my_bits_high |= t.wrapping_add(t >> 9 as libc::c_int);
        flipped =
            (flipped as
                 libc::c_uint).wrapping_add((t >>
                                                 4 as libc::c_int +
                                                     9 as libc::c_int |
                                                 t >>
                                                     4 as libc::c_int +
                                                         9 as libc::c_int *
                                                             2 as libc::c_int
                                                         - 1 as libc::c_int) &
                                                3 as libc::c_int as
                                                    libc::c_uint) as
                libc::c_int as libc::c_int
    }
    /* Up left */
    if opp_bits_low &
           ((1 as libc::c_int) <<
                4 as libc::c_int + 32 as libc::c_int - 9 as libc::c_int) as
               libc::c_uint != 0 {
        t =
            opp_bits_low >>
                4 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 2 as libc::c_int &
                1 as libc::c_int as libc::c_uint;
        contig =
            (1 as libc::c_int as libc::c_uint).wrapping_add(t) as libc::c_int;
        t &=
            opp_bits_low >>
                4 as libc::c_int + 32 as libc::c_int -
                    9 as libc::c_int * 3 as libc::c_int;
        contig =
            (contig as libc::c_uint).wrapping_add(t) as libc::c_int as
                libc::c_int;
        fl = msb_mask[contig as usize] & 0x8040201 as libc::c_uint;
        t =
            (-((my_bits_low & fl) as libc::c_int) >> 31 as libc::c_int) as
                libc::c_uint;
        my_bits_low |= fl & t;
        flipped =
            (flipped as libc::c_uint).wrapping_add(contig as libc::c_uint & t)
                as libc::c_int as libc::c_int
    }
    my_bits_high |= 0x10 as libc::c_uint;
    bb_flips.high = my_bits_high;
    bb_flips.low = my_bits_low;
    return flipped;
}
#[no_mangle]
pub static mut TestFlips_bitboard:
           [Option<unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                        _: libc::c_uint, _: libc::c_uint)
                       -> libc::c_int>; 78] =
    unsafe {
        [Some(TestFlips_bitboard_a1 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_b1 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_c1 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_d1 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_e1 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_f1 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_g1 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_h1 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(TestFlips_bitboard_a2 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_b2 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_c2 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_d2 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_e2 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_f2 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_g2 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_h2 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(TestFlips_bitboard_a3 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_b3 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_c3 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_d3 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_e3 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_f3 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_g3 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_h3 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(TestFlips_bitboard_a4 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_b4 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_c4 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_d4 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_e4 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_f4 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_g4 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_h4 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(TestFlips_bitboard_a5 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_b5 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_c5 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_d5 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_e5 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_f5 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_g5 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_h5 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(TestFlips_bitboard_a6 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_b6 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_c6 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_d6 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_e6 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_f6 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_g6 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_h6 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(TestFlips_bitboard_a7 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_b7 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_c7 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_d7 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_e7 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_f7 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_g7 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_h7 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int), None, None,
         Some(TestFlips_bitboard_a8 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_b8 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_c8 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_d8 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_e8 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_f8 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_g8 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int),
         Some(TestFlips_bitboard_h8 as
                  unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                       _: libc::c_uint, _: libc::c_uint)
                      -> libc::c_int)]
    };
