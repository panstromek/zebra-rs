use crate::src::zebra::FullState;

pub struct LearnState {
    pub learn_depth: i32,
    pub cutoff_empty: i32,
    pub game_move: [i16; 61],
    pub binary_database: i32,
    pub database_name: [i8; 256],
}

/*
   File:          learn.h

   Created:       November 29, 1997

   Modified:      November 18, 2001

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The interface to the learning module.
*/

impl LearnState {
    pub const fn new() -> Self {
        LearnState {
            learn_depth: 0,
            cutoff_empty: 0,
            game_move: [0; 61],
            binary_database:  0,
            database_name: [0; 256]
        }
    }
    /*
       CLEAR_STORED_GAME
       Remove all stored moves.
    */
    pub fn clear_stored_game(&mut self) {
        let mut i: i32 = 0;
        i = 0;
        while i <= 60 {
            self.game_move[i as usize] = -(1) as i16;
            i += 1
        };
    }
    /*
       STORE_MOVE
       Mark the move MOVE as being played after DISKS_PLAYED moves
       had been played.
    */

    pub fn store_move(&mut self, disks_played_0: i32, move_0: i8) {
        self.game_move[disks_played_0 as usize] = move_0 as i16;
    }
    /*
       SET_LEARNING_PARAMETERS
       Specify the depth to which deviations are checked for and the number
       of empty squares at which the game is considered over.
    */

    pub fn set_learning_parameters(&mut self, depth: i32, cutoff: i32) {
        self.learn_depth = depth;
        self.cutoff_empty = cutoff;
    }
    /*
       GAME_LEARNABLE
       Checks if the current game can be learned - i.e. if the moves of the
       game are available and the game was finished or contains enough
       moves to be learned anyway.
    */

    pub fn game_learnable(&self, finished: i32, move_count_0: i32) -> i32 {
        let mut i: i32 = 0;
        let mut moves_available: i32 = 0;
        moves_available = 1;
        i = 0;
        while i < move_count_0 && i < 60 - self.cutoff_empty {
            if self.game_move[i as usize] as i32 == -(1) {
                moves_available = 0
            }
            i += 1
        }
        return (moves_available != 0 &&
            (finished != 0 ||
                move_count_0 >= 60 - self.cutoff_empty)) as
            i32;
    }

    pub fn get_stored_move(&self, index: i32) -> i16 {
        if index > 60 {
            const ILLEGAL: i16 = -1;
            return ILLEGAL;
        }
        return self.game_move[index as usize];
    }
}
