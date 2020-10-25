
/*
   File:           globals.h

   Created:        June 30, 1997

   Modified:       January 8, 2000

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Global state variables.
*/
/* The basic board type. One index for each position;
   a1=11, h1=18, a8=81, h8=88. */

pub type Board = [i32; 128];
/*
   File:       globals.c

   Created:    June 30, 1997

   Modified:   October 30, 2001

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   Global state variables.
*/
/* Global variables */

pub static mut pv___: [[i32; 64]; 64] = [[0; 64]; 64];

pub static mut pv_depth___: [i32; 64] = [0; 64];

pub static mut score_sheet_row___: i32 = 0;

pub static mut piece_count___: [[i32; 64]; 3] = [[0; 64]; 3];

pub static mut black_moves___: [i32; 60] = [0; 60];

pub static mut white_moves___: [i32; 60] = [0; 60];

pub static mut board___: Board = [0; 128];
