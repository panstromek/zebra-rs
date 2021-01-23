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


pub const fn pow3(n: usize) -> i32 {
    3i32.wrapping_pow(n as _)
}
/* Connections between the squares and the bit masks */

/* These values needed for compatibility with the old book format */
// color_pattern[0] = 1;
// color_pattern[2] = 2;
pub static color_pattern: [i32; 3] = [1, 0, 2];
/* The patterns describing the current state of the board. */
/* Symmetry maps */

pub static flip8: [i16; 6561] = transformation_setup();
/* Bit masks which represent dependencies between discs and patterns */

/*
   TRANSFORMATION_SET_UP
   Calculate the various symmetry and color transformations.
*/
const fn transformation_setup() -> [i16; 6561] {
    pub const pow3: [i16; 10] = [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683];

    let mut flip8_: [i16; 6561] = [0;6561];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut row: [i16; 10] = [0; 10];
    /* Build the pattern tables for 8*1-patterns */
    i = 0;
    while i < 8 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 6561 as i32 {
        /* Create the symmetry map */
        flip8_[i as usize] = 0;
        j = 0;
        while j < 8 as i32 {
            flip8_[i as usize] += row[j as usize] * pow3[(7 as i32 - j) as usize];
            j += 1
        }
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 {
                row[j as usize] = 0
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0  &&
                     j < 8 as i32) {
                break ;
            }
        }
        i += 1
    };
    return flip8_;
}
/*
   INIT_PATTERNS
   Pre-computes some tables needed for fast pattern access.
*/

const fn init_patterns() -> [[u8; 100]; 4] {
    let mut row_no_: [u8; 100] = [0; 100];
    let mut row_index_: [u8; 100] = [0; 100];
    let mut col_no_: [u8; 100] = [0; 100];
    let mut col_index_: [u8; 100] = [0; 100];

    let mut i = 1u8;
    while i <= 8 {
        let mut j = 1;
        while j <= 8  {
            let pos = (10 * i + j) as usize;
            row_no_[pos] = i - 1;
            row_index_[pos] = j - 1;
            col_no_[pos] = j - 1;
            col_index_[pos] = i - 1;
            j += 1
        }
        i += 1
    }
    return [
        row_no_,
        row_index_,
        col_no_,
        col_index_
    ]
}
/*
   COMPUTE_LINE_PATTERNS
   Translate the current board configuration into patterns.
*/

pub fn compute_line_patterns(in_board: &[i32; 128], row_pattern_: &mut [i32; 8], col_pattern_: &mut [i32; 8]) {
    static patterns: [[u8; 100]; 4] = init_patterns();
    static row_no: &[u8; 100] = &patterns[0];
    static row_index: &[u8; 100] = &patterns[1];
    static col_no: &[u8; 100] = &patterns[2];
    static col_index: &[u8; 100] = &patterns[3];

    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut mask: i32 = 0;
    *row_pattern_ = [0; 8];
    *col_pattern_ = [0; 8];
    i = 1;
    while i <= 8 as i32 {
        j = 1;
        while j <= 8 as i32 {
            pos = 10 as i32 * i + j;
            if in_board[pos as usize] == 1 as i32 {
                mask = 0 as i32
            } else {
                mask = color_pattern[in_board[pos as usize] as usize]
            }
            row_pattern_[row_no[pos as usize] as usize] +=
                mask * pow3(row_index[pos as usize] as usize);
            col_pattern_[col_no[pos as usize] as usize] +=
                mask * pow3(col_index[pos as usize] as usize);
            j += 1
        }
        i += 1
    };
}
