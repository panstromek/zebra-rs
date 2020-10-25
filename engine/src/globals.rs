
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

pub struct BoardState {
    pub pv: [[i32; 64]; 64],
    pub pv_depth: [i32; 64],
    pub score_sheet_row: i32,
    pub piece_count: [[i32; 64]; 3],
    pub black_moves: [i32; 60],
    pub white_moves: [i32; 60],
    pub board: Board,
}

pub static mut board_state: BoardState = BoardState {
    pv: [[0; 64]; 64],
    pv_depth: [0; 64],
    score_sheet_row: 0,
    piece_count: [[0; 64]; 3],
    black_moves: [0; 60],
    white_moves: [0; 60],
    board: [0; 128],
};
