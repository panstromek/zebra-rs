#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]

/*
   File:          patterns.c

   Created:       July 4, 1997

   Modified:      December 25, 1999

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The patterns.
*/


pub const pow3: [i32; 10] =
    [1 as i32, 3 as i32, 9 as i32, 27 as i32,
     81 as i32, 243 as i32, 729 as i32,
     2187 as i32, 6561 as i32, 19683 as i32];
/* Connections between the squares and the bit masks */

pub static mut row_no: [i32; 100] = [0; 100];

pub static mut row_index: [i32; 100] = [0; 100];

pub static mut col_no: [i32; 100] = [0; 100];

pub static mut col_index: [i32; 100] = [0; 100];

pub static mut color_pattern: [i32; 3] = [0; 3];
/* The patterns describing the current state of the board. */

pub static mut row_pattern: [i32; 8] = [0; 8];

pub static mut col_pattern: [i32; 8] = [0; 8];
/* Symmetry maps */

pub const flip8: [i32; 6561] = transformation_setup();
/* Bit masks which represent dependencies between discs and patterns */

pub static mut depend_lo: [u32; 100] = [0; 100];

pub static mut depend_hi: [u32; 100] = [0; 100];
/* Bit masks that show what patterns have been modified */

pub static mut modified_lo: u32 = 0;

pub static mut modified_hi: u32 = 0;
/*
   TRANSFORMATION_SET_UP
   Calculate the various symmetry and color transformations.
*/
const fn transformation_setup() -> [i32; 6561] {
    let mut flip8_: [i32; 6561] = [0;6561];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut row: [i32; 10] = [0; 10];
    /* Build the pattern tables for 8*1-patterns */
    i = 0 as i32;
    while i < 8 as i32 { row[i as usize] = 0 as i32; i += 1 }
    i = 0 as i32;
    while i < 6561 as i32 {
        /* Create the symmetry map */
        flip8_[i as usize] = 0 as i32;
        j = 0 as i32;
        while j < 8 as i32 {
            flip8_[i as usize] +=
                row[j as usize] * pow3[(7 as i32 - j) as usize];
            j += 1
        }
        /* Next configuration */
        j = 0 as i32;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                     j < 8 as i32) {
                break ;
            }
        }
        i += 1
    };
    return flip8_;
}
/*
  ADD_SINGLE
  Mark board position POS as depending on pattern # MASK.
*/
unsafe fn add_single(mask: i32, pos: i32) {
    if mask < 32 as i32 {
        depend_lo[pos as usize] |=
            ((1 as i32) << mask) as u32
    } else {
        depend_hi[pos as usize] |=
            ((1 as i32) << mask - 32 as i32) as u32
    };
}
/*
  ADD_MULTIPLE
  Mark board positions POS, POS+STEP, ..., POS+(COUNT-1)STEP as
  depending on pattern # MASK.
*/
unsafe fn add_multiple(mask: i32, pos: i32,
                                  count: i32,
                                  step: i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
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
    add_multiple(0 as i32, 11 as i32, 8 as i32,
                 10 as i32);
    add_single(0 as i32, 22 as i32);
    add_single(0 as i32, 72 as i32);
    /* A-file+2X: h1-h8 + g2,g7 */
    add_multiple(1 as i32, 18 as i32, 8 as i32,
                 10 as i32);
    add_single(1 as i32, 27 as i32);
    add_single(1 as i32, 77 as i32);
    /* A-file+2X: a1-h1 + b2,g2 */
    add_multiple(2 as i32, 11 as i32, 8 as i32,
                 1 as i32);
    add_single(2 as i32, 22 as i32);
    add_single(2 as i32, 27 as i32);
    /* A-file+2X: a8-h8 + b7,g7 */
    add_multiple(3 as i32, 81 as i32, 8 as i32,
                 1 as i32);
    add_single(3 as i32, 72 as i32);
    add_single(3 as i32, 77 as i32);
    /* B-file: b1-b8 */
    add_multiple(4 as i32, 12 as i32, 8 as i32,
                 10 as i32);
    /* B-file: g1-g8 */
    add_multiple(5 as i32, 17 as i32, 8 as i32,
                 10 as i32);
    /* B-file: a2-h2 */
    add_multiple(6 as i32, 21 as i32, 8 as i32,
                 1 as i32);
    /* B-file: a7-h7 */
    add_multiple(7 as i32, 71 as i32, 8 as i32,
                 1 as i32);
    /* C-file: c1-c8 */
    add_multiple(8 as i32, 13 as i32, 8 as i32,
                 10 as i32);
    /* C-file: f1-f8 */
    add_multiple(9 as i32, 16 as i32, 8 as i32,
                 10 as i32);
    /* C-file: a3-h3 */
    add_multiple(10 as i32, 31 as i32, 8 as i32,
                 1 as i32);
    /* C-file: a6-h6 */
    add_multiple(11 as i32, 61 as i32, 8 as i32,
                 1 as i32);
    /* D-file: d1-d8 */
    add_multiple(12 as i32, 14 as i32, 8 as i32,
                 10 as i32);
    /* D-file: e1-e8 */
    add_multiple(13 as i32, 15 as i32, 8 as i32,
                 10 as i32);
    /* D-file: a4-h4 */
    add_multiple(14 as i32, 41 as i32, 8 as i32,
                 1 as i32);
    /* D-file: a5-h5 */
    add_multiple(15 as i32, 51 as i32, 8 as i32,
                 1 as i32);
    /* Diag8: a1-h8 */
    add_multiple(16 as i32, 11 as i32, 8 as i32,
                 11 as i32);
    /* Diag8: h1-a8 */
    add_multiple(17 as i32, 18 as i32, 8 as i32,
                 9 as i32);
    /* Diag7: b1-h7 */
    add_multiple(18 as i32, 12 as i32, 7 as i32,
                 11 as i32);
    /* Diag7: a2-g8 */
    add_multiple(19 as i32, 21 as i32, 7 as i32,
                 11 as i32);
    /* Diag7: a7-g1 */
    add_multiple(20 as i32, 17 as i32, 7 as i32,
                 9 as i32);
    /* Diag7: b8-h2 */
    add_multiple(21 as i32, 28 as i32, 7 as i32,
                 9 as i32);
    /* Diag6: c1-h6 */
    add_multiple(22 as i32, 13 as i32, 6 as i32,
                 11 as i32);
    /* Diag6: a3-f8 */
    add_multiple(23 as i32, 31 as i32, 6 as i32,
                 11 as i32);
    /* Diag6: a6-f1 */
    add_multiple(24 as i32, 16 as i32, 6 as i32,
                 9 as i32);
    /* Diag6: c8-h3 */
    add_multiple(25 as i32, 38 as i32, 6 as i32,
                 9 as i32);
    /* Diag5: d1-h5 */
    add_multiple(26 as i32, 14 as i32, 5 as i32,
                 11 as i32);
    /* Diag5: a4-e8 */
    add_multiple(27 as i32, 41 as i32, 5 as i32,
                 11 as i32);
    /* Diag5: a5-e1 */
    add_multiple(28 as i32, 15 as i32, 5 as i32,
                 9 as i32);
    /* Diag5: d8-h4 */
    add_multiple(29 as i32, 48 as i32, 5 as i32,
                 9 as i32);
    /* Diag4: e1-h4 */
    add_multiple(30 as i32, 15 as i32, 4 as i32,
                 11 as i32);
    /* Diag4: a5-d8 */
    add_multiple(31 as i32, 51 as i32, 4 as i32,
                 11 as i32);
    /* Diag4: a4-d1 */
    add_multiple(32 as i32, 14 as i32, 4 as i32,
                 9 as i32);
    /* Diag4: e8-h5 */
    add_multiple(33 as i32, 58 as i32, 4 as i32,
                 9 as i32);
    /* Corner3x3: a1-c1 + a2-c2 + a3-c3 */
    add_multiple(34 as i32, 11 as i32, 3 as i32,
                 1 as i32);
    add_multiple(34 as i32, 21 as i32, 3 as i32,
                 1 as i32);
    add_multiple(34 as i32, 31 as i32, 3 as i32,
                 1 as i32);
    /* Corner3x3: a8-c8 + a7-c7 + a6-c6 */
    add_multiple(35 as i32, 81 as i32, 3 as i32,
                 1 as i32);
    add_multiple(35 as i32, 71 as i32, 3 as i32,
                 1 as i32);
    add_multiple(35 as i32, 61 as i32, 3 as i32,
                 1 as i32);
    /* Corner3x3: f1-h1 + f2-h2 + f3-h3 */
    add_multiple(36 as i32, 18 as i32, 3 as i32,
                 -(1 as i32));
    add_multiple(36 as i32, 28 as i32, 3 as i32,
                 -(1 as i32));
    add_multiple(36 as i32, 38 as i32, 3 as i32,
                 -(1 as i32));
    /* Corner3x3: f8-h8 + f7-h7 + f6-h6 */
    add_multiple(37 as i32, 88 as i32, 3 as i32,
                 -(1 as i32));
    add_multiple(37 as i32, 78 as i32, 3 as i32,
                 -(1 as i32));
    add_multiple(37 as i32, 68 as i32, 3 as i32,
                 -(1 as i32));
    /* Corner4x2: a1-d1 + a2-d2 */
    add_multiple(38 as i32, 11 as i32, 4 as i32,
                 1 as i32);
    add_multiple(38 as i32, 21 as i32, 4 as i32,
                 1 as i32);
    /* Corner4x2: a8-d8 + a7-d7 */
    add_multiple(39 as i32, 81 as i32, 4 as i32,
                 1 as i32);
    add_multiple(39 as i32, 71 as i32, 4 as i32,
                 1 as i32);
    /* Corner4x2: e1-h1 + e2-h2 */
    add_multiple(40 as i32, 18 as i32, 4 as i32,
                 -(1 as i32));
    add_multiple(40 as i32, 28 as i32, 4 as i32,
                 -(1 as i32));
    /* Corner4x2: e8-h8 + e7-h7 */
    add_multiple(41 as i32, 88 as i32, 4 as i32,
                 -(1 as i32));
    add_multiple(41 as i32, 78 as i32, 4 as i32,
                 -(1 as i32));
    /* Corner4x2: a1-a4 + b1-b4 */
    add_multiple(42 as i32, 11 as i32, 4 as i32,
                 10 as i32);
    add_multiple(42 as i32, 12 as i32, 4 as i32,
                 10 as i32);
    /* Corner4x2: h1-h4 + g1-g4 */
    add_multiple(43 as i32, 18 as i32, 4 as i32,
                 10 as i32);
    add_multiple(43 as i32, 17 as i32, 4 as i32,
                 10 as i32);
    /* Corner4x2: a8-a5 + b8-b5 */
    add_multiple(44 as i32, 81 as i32, 4 as i32,
                 -(10 as i32));
    add_multiple(44 as i32, 82 as i32, 4 as i32,
                 -(10 as i32));
    /* Corner4x2: h8-h5 + g8-g5 */
    add_multiple(45 as i32, 88 as i32, 4 as i32,
                 -(10 as i32));
    add_multiple(45 as i32, 87 as i32, 4 as i32,
                 -(10 as i32));
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
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            pos = 10 as i32 * i + j;
            row_no[pos as usize] = i - 1 as i32;
            row_index[pos as usize] = j - 1 as i32;
            col_no[pos as usize] = j - 1 as i32;
            col_index[pos as usize] = i - 1 as i32;
            j += 1
        }
        i += 1
    }
    pattern_dependency();
    /* These values needed for compatibility with the old book format */
    color_pattern[0 as i32 as usize] = 1 as i32;
    color_pattern[2 as i32 as usize] = 2 as i32;
}
/*
   COMPUTE_LINE_PATTERNS
   Translate the current board configuration into patterns.
*/

pub unsafe fn compute_line_patterns(in_board:
                                                   *mut i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut mask: i32 = 0;
    i = 0 as i32;
    while i < 8 as i32 {
        row_pattern[i as usize] = 0 as i32;
        col_pattern[i as usize] = 0 as i32;
        i += 1
    }
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            pos = 10 as i32 * i + j;
            if *in_board.offset(pos as isize) == 1 as i32 {
                mask = 0 as i32
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
