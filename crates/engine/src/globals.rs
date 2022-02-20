
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
    pub pv: [[i8; 64]; 64],
    pub pv_depth: [i32; 64],
    pub score_sheet_row: i32,
    pub piece_count: [[i32; 64]; 3],
    pub black_moves: [i8; 60],
    pub white_moves: [i8; 60],
    pub board: Board,
}

pub struct PieceCounts {
    pub my_discs: i32,
    pub opp_discs: i32,
}

impl BoardState {
    pub const fn new() -> Self {
        BoardState {
            pv: [[0; 64]; 64],
            pv_depth: [0; 64],
            score_sheet_row: 0,
            piece_count: [[0; 64]; 3],
            black_moves: [0; 60],
            white_moves: [0; 60],
            board: [0; 128],
        }
    }
    pub fn get_piece_counts(&self, side_to_move: i32, disks_played: i32) -> PieceCounts {
        let my_discs = self.piece_count[side_to_move as usize][disks_played as usize];
        let opp_discs = self.piece_count[(2 - side_to_move) as usize][disks_played as usize];
        PieceCounts {
            my_discs,
            opp_discs,
        }
    }

    /*
      CLEAR_PV
      Clears the principal variation.
    */
    pub fn clear_pv(&mut self) {
        self.pv_depth[0] = 0;
    }
}
