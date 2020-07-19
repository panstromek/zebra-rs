
/*
   File:          patterns.c

   Created:       July 4, 1997

   Modified:      December 25, 1999

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The patterns.
*/
/* Global variables */
use crate::src::libc;


pub static mut pow3: [libc::c_int; 10] =
    [1 as libc::c_int, 3 as libc::c_int, 9 as libc::c_int, 27 as libc::c_int,
     81 as libc::c_int, 243 as libc::c_int, 729 as libc::c_int,
     2187 as libc::c_int, 6561 as libc::c_int, 19683 as libc::c_int];
/* Connections between the squares and the bit masks */

pub static mut row_no: [libc::c_int; 100] = [0; 100];

pub static mut row_index: [libc::c_int; 100] = [0; 100];

pub static mut col_no: [libc::c_int; 100] = [0; 100];

pub static mut col_index: [libc::c_int; 100] = [0; 100];

pub static mut color_pattern: [libc::c_int; 3] = [0; 3];
/* The patterns describing the current state of the board. */

pub static mut row_pattern: [libc::c_int; 8] = [0; 8];

pub static mut col_pattern: [libc::c_int; 8] = [0; 8];
/* Symmetry maps */

pub static mut flip8: [libc::c_int; 6561] = [0; 6561];
/* Bit masks which represent dependencies between discs and patterns */

pub static mut depend_lo: [libc::c_uint; 100] = [0; 100];

pub static mut depend_hi: [libc::c_uint; 100] = [0; 100];
/* Bit masks that show what patterns have been modified */

pub static mut modified_lo: libc::c_uint = 0;

pub static mut modified_hi: libc::c_uint = 0;
/*
   TRANSFORMATION_SET_UP
   Calculate the various symmetry and color transformations.
*/
unsafe fn transformation_setup() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut row: [libc::c_int; 10] = [0; 10];
    /* Build the pattern tables for 8*1-patterns */
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int { row[i as usize] = 0 as libc::c_int; i += 1 }
    i = 0 as libc::c_int;
    while i < 6561 as libc::c_int {
        /* Create the symmetry map */
        flip8[i as usize] = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            flip8[i as usize] +=
                row[j as usize] * pow3[(7 as libc::c_int - j) as usize];
            j += 1
        }
        /* Next configuration */
        j = 0 as libc::c_int;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as libc::c_int {
                row[j as usize] = 0 as libc::c_int
            }
            j += 1;
            if !(row[(j - 1 as libc::c_int) as usize] == 0 as libc::c_int &&
                     j < 8 as libc::c_int) {
                break ;
            }
        }
        i += 1
    };
}
/*
  ADD_SINGLE
  Mark board position POS as depending on pattern # MASK.
*/
unsafe fn add_single(mut mask: libc::c_int, mut pos: libc::c_int) {
    if mask < 32 as libc::c_int {
        depend_lo[pos as usize] |=
            ((1 as libc::c_int) << mask) as libc::c_uint
    } else {
        depend_hi[pos as usize] |=
            ((1 as libc::c_int) << mask - 32 as libc::c_int) as libc::c_uint
    };
}
/*
  ADD_MULTIPLE
  Mark board positions POS, POS+STEP, ..., POS+(COUNT-1)STEP as
  depending on pattern # MASK.
*/
unsafe fn add_multiple(mut mask: libc::c_int, mut pos: libc::c_int,
                                  mut count: libc::c_int,
                                  mut step: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < count { add_single(mask, pos + i * step); i += 1 };
}
/*
  PATTERN_DEPENDENCY
  Fill the dependency masks for each square with the bit masks
  for the patterns which it depends.
  Note: The definitions of the patterns and their corresponding name
        must match the order given in endmacro.c.
*/
unsafe fn pattern_dependency() {
    /* A-file+2X: a1-a8 + b2,b7 */
    add_multiple(0 as libc::c_int, 11 as libc::c_int, 8 as libc::c_int,
                 10 as libc::c_int);
    add_single(0 as libc::c_int, 22 as libc::c_int);
    add_single(0 as libc::c_int, 72 as libc::c_int);
    /* A-file+2X: h1-h8 + g2,g7 */
    add_multiple(1 as libc::c_int, 18 as libc::c_int, 8 as libc::c_int,
                 10 as libc::c_int);
    add_single(1 as libc::c_int, 27 as libc::c_int);
    add_single(1 as libc::c_int, 77 as libc::c_int);
    /* A-file+2X: a1-h1 + b2,g2 */
    add_multiple(2 as libc::c_int, 11 as libc::c_int, 8 as libc::c_int,
                 1 as libc::c_int);
    add_single(2 as libc::c_int, 22 as libc::c_int);
    add_single(2 as libc::c_int, 27 as libc::c_int);
    /* A-file+2X: a8-h8 + b7,g7 */
    add_multiple(3 as libc::c_int, 81 as libc::c_int, 8 as libc::c_int,
                 1 as libc::c_int);
    add_single(3 as libc::c_int, 72 as libc::c_int);
    add_single(3 as libc::c_int, 77 as libc::c_int);
    /* B-file: b1-b8 */
    add_multiple(4 as libc::c_int, 12 as libc::c_int, 8 as libc::c_int,
                 10 as libc::c_int);
    /* B-file: g1-g8 */
    add_multiple(5 as libc::c_int, 17 as libc::c_int, 8 as libc::c_int,
                 10 as libc::c_int);
    /* B-file: a2-h2 */
    add_multiple(6 as libc::c_int, 21 as libc::c_int, 8 as libc::c_int,
                 1 as libc::c_int);
    /* B-file: a7-h7 */
    add_multiple(7 as libc::c_int, 71 as libc::c_int, 8 as libc::c_int,
                 1 as libc::c_int);
    /* C-file: c1-c8 */
    add_multiple(8 as libc::c_int, 13 as libc::c_int, 8 as libc::c_int,
                 10 as libc::c_int);
    /* C-file: f1-f8 */
    add_multiple(9 as libc::c_int, 16 as libc::c_int, 8 as libc::c_int,
                 10 as libc::c_int);
    /* C-file: a3-h3 */
    add_multiple(10 as libc::c_int, 31 as libc::c_int, 8 as libc::c_int,
                 1 as libc::c_int);
    /* C-file: a6-h6 */
    add_multiple(11 as libc::c_int, 61 as libc::c_int, 8 as libc::c_int,
                 1 as libc::c_int);
    /* D-file: d1-d8 */
    add_multiple(12 as libc::c_int, 14 as libc::c_int, 8 as libc::c_int,
                 10 as libc::c_int);
    /* D-file: e1-e8 */
    add_multiple(13 as libc::c_int, 15 as libc::c_int, 8 as libc::c_int,
                 10 as libc::c_int);
    /* D-file: a4-h4 */
    add_multiple(14 as libc::c_int, 41 as libc::c_int, 8 as libc::c_int,
                 1 as libc::c_int);
    /* D-file: a5-h5 */
    add_multiple(15 as libc::c_int, 51 as libc::c_int, 8 as libc::c_int,
                 1 as libc::c_int);
    /* Diag8: a1-h8 */
    add_multiple(16 as libc::c_int, 11 as libc::c_int, 8 as libc::c_int,
                 11 as libc::c_int);
    /* Diag8: h1-a8 */
    add_multiple(17 as libc::c_int, 18 as libc::c_int, 8 as libc::c_int,
                 9 as libc::c_int);
    /* Diag7: b1-h7 */
    add_multiple(18 as libc::c_int, 12 as libc::c_int, 7 as libc::c_int,
                 11 as libc::c_int);
    /* Diag7: a2-g8 */
    add_multiple(19 as libc::c_int, 21 as libc::c_int, 7 as libc::c_int,
                 11 as libc::c_int);
    /* Diag7: a7-g1 */
    add_multiple(20 as libc::c_int, 17 as libc::c_int, 7 as libc::c_int,
                 9 as libc::c_int);
    /* Diag7: b8-h2 */
    add_multiple(21 as libc::c_int, 28 as libc::c_int, 7 as libc::c_int,
                 9 as libc::c_int);
    /* Diag6: c1-h6 */
    add_multiple(22 as libc::c_int, 13 as libc::c_int, 6 as libc::c_int,
                 11 as libc::c_int);
    /* Diag6: a3-f8 */
    add_multiple(23 as libc::c_int, 31 as libc::c_int, 6 as libc::c_int,
                 11 as libc::c_int);
    /* Diag6: a6-f1 */
    add_multiple(24 as libc::c_int, 16 as libc::c_int, 6 as libc::c_int,
                 9 as libc::c_int);
    /* Diag6: c8-h3 */
    add_multiple(25 as libc::c_int, 38 as libc::c_int, 6 as libc::c_int,
                 9 as libc::c_int);
    /* Diag5: d1-h5 */
    add_multiple(26 as libc::c_int, 14 as libc::c_int, 5 as libc::c_int,
                 11 as libc::c_int);
    /* Diag5: a4-e8 */
    add_multiple(27 as libc::c_int, 41 as libc::c_int, 5 as libc::c_int,
                 11 as libc::c_int);
    /* Diag5: a5-e1 */
    add_multiple(28 as libc::c_int, 15 as libc::c_int, 5 as libc::c_int,
                 9 as libc::c_int);
    /* Diag5: d8-h4 */
    add_multiple(29 as libc::c_int, 48 as libc::c_int, 5 as libc::c_int,
                 9 as libc::c_int);
    /* Diag4: e1-h4 */
    add_multiple(30 as libc::c_int, 15 as libc::c_int, 4 as libc::c_int,
                 11 as libc::c_int);
    /* Diag4: a5-d8 */
    add_multiple(31 as libc::c_int, 51 as libc::c_int, 4 as libc::c_int,
                 11 as libc::c_int);
    /* Diag4: a4-d1 */
    add_multiple(32 as libc::c_int, 14 as libc::c_int, 4 as libc::c_int,
                 9 as libc::c_int);
    /* Diag4: e8-h5 */
    add_multiple(33 as libc::c_int, 58 as libc::c_int, 4 as libc::c_int,
                 9 as libc::c_int);
    /* Corner3x3: a1-c1 + a2-c2 + a3-c3 */
    add_multiple(34 as libc::c_int, 11 as libc::c_int, 3 as libc::c_int,
                 1 as libc::c_int);
    add_multiple(34 as libc::c_int, 21 as libc::c_int, 3 as libc::c_int,
                 1 as libc::c_int);
    add_multiple(34 as libc::c_int, 31 as libc::c_int, 3 as libc::c_int,
                 1 as libc::c_int);
    /* Corner3x3: a8-c8 + a7-c7 + a6-c6 */
    add_multiple(35 as libc::c_int, 81 as libc::c_int, 3 as libc::c_int,
                 1 as libc::c_int);
    add_multiple(35 as libc::c_int, 71 as libc::c_int, 3 as libc::c_int,
                 1 as libc::c_int);
    add_multiple(35 as libc::c_int, 61 as libc::c_int, 3 as libc::c_int,
                 1 as libc::c_int);
    /* Corner3x3: f1-h1 + f2-h2 + f3-h3 */
    add_multiple(36 as libc::c_int, 18 as libc::c_int, 3 as libc::c_int,
                 -(1 as libc::c_int));
    add_multiple(36 as libc::c_int, 28 as libc::c_int, 3 as libc::c_int,
                 -(1 as libc::c_int));
    add_multiple(36 as libc::c_int, 38 as libc::c_int, 3 as libc::c_int,
                 -(1 as libc::c_int));
    /* Corner3x3: f8-h8 + f7-h7 + f6-h6 */
    add_multiple(37 as libc::c_int, 88 as libc::c_int, 3 as libc::c_int,
                 -(1 as libc::c_int));
    add_multiple(37 as libc::c_int, 78 as libc::c_int, 3 as libc::c_int,
                 -(1 as libc::c_int));
    add_multiple(37 as libc::c_int, 68 as libc::c_int, 3 as libc::c_int,
                 -(1 as libc::c_int));
    /* Corner4x2: a1-d1 + a2-d2 */
    add_multiple(38 as libc::c_int, 11 as libc::c_int, 4 as libc::c_int,
                 1 as libc::c_int);
    add_multiple(38 as libc::c_int, 21 as libc::c_int, 4 as libc::c_int,
                 1 as libc::c_int);
    /* Corner4x2: a8-d8 + a7-d7 */
    add_multiple(39 as libc::c_int, 81 as libc::c_int, 4 as libc::c_int,
                 1 as libc::c_int);
    add_multiple(39 as libc::c_int, 71 as libc::c_int, 4 as libc::c_int,
                 1 as libc::c_int);
    /* Corner4x2: e1-h1 + e2-h2 */
    add_multiple(40 as libc::c_int, 18 as libc::c_int, 4 as libc::c_int,
                 -(1 as libc::c_int));
    add_multiple(40 as libc::c_int, 28 as libc::c_int, 4 as libc::c_int,
                 -(1 as libc::c_int));
    /* Corner4x2: e8-h8 + e7-h7 */
    add_multiple(41 as libc::c_int, 88 as libc::c_int, 4 as libc::c_int,
                 -(1 as libc::c_int));
    add_multiple(41 as libc::c_int, 78 as libc::c_int, 4 as libc::c_int,
                 -(1 as libc::c_int));
    /* Corner4x2: a1-a4 + b1-b4 */
    add_multiple(42 as libc::c_int, 11 as libc::c_int, 4 as libc::c_int,
                 10 as libc::c_int);
    add_multiple(42 as libc::c_int, 12 as libc::c_int, 4 as libc::c_int,
                 10 as libc::c_int);
    /* Corner4x2: h1-h4 + g1-g4 */
    add_multiple(43 as libc::c_int, 18 as libc::c_int, 4 as libc::c_int,
                 10 as libc::c_int);
    add_multiple(43 as libc::c_int, 17 as libc::c_int, 4 as libc::c_int,
                 10 as libc::c_int);
    /* Corner4x2: a8-a5 + b8-b5 */
    add_multiple(44 as libc::c_int, 81 as libc::c_int, 4 as libc::c_int,
                 -(10 as libc::c_int));
    add_multiple(44 as libc::c_int, 82 as libc::c_int, 4 as libc::c_int,
                 -(10 as libc::c_int));
    /* Corner4x2: h8-h5 + g8-g5 */
    add_multiple(45 as libc::c_int, 88 as libc::c_int, 4 as libc::c_int,
                 -(10 as libc::c_int));
    add_multiple(45 as libc::c_int, 87 as libc::c_int, 4 as libc::c_int,
                 -(10 as libc::c_int));
}
/* Connections between the squares and the bit masks */
/* The patterns describing the current state of the board. */
/* Symmetry maps */
/* Masks which represent dependencies between discs and patterns */
/* Bit masks that show what patterns have been modified */
/*
   INIT_PATTERNS
   Pre-computes some tables needed for fast pattern access.
*/

pub unsafe fn init_patterns() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    transformation_setup();
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            pos = 10 as libc::c_int * i + j;
            row_no[pos as usize] = i - 1 as libc::c_int;
            row_index[pos as usize] = j - 1 as libc::c_int;
            col_no[pos as usize] = j - 1 as libc::c_int;
            col_index[pos as usize] = i - 1 as libc::c_int;
            j += 1
        }
        i += 1
    }
    pattern_dependency();
    /* These values needed for compatibility with the old book format */
    color_pattern[0 as libc::c_int as usize] = 1 as libc::c_int;
    color_pattern[2 as libc::c_int as usize] = 2 as libc::c_int;
}
/*
   COMPUTE_LINE_PATTERNS
   Translate the current board configuration into patterns.
*/

pub unsafe fn compute_line_patterns(mut in_board:
                                                   *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        row_pattern[i as usize] = 0 as libc::c_int;
        col_pattern[i as usize] = 0 as libc::c_int;
        i += 1
    }
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            pos = 10 as libc::c_int * i + j;
            if *in_board.offset(pos as isize) == 1 as libc::c_int {
                mask = 0 as libc::c_int
            } else {
                mask = color_pattern[*in_board.offset(pos as isize) as usize]
            }
            row_pattern[row_no[pos as usize] as usize] +=
                mask * pow3[row_index[pos as usize] as usize];
            col_pattern[col_no[pos as usize] as usize] +=
                mask * pow3[col_index[pos as usize] as usize];
            j += 1
        }
        i += 1
    };
}
