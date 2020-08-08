use crate::src::moves::{make_move, generate_all, disks_played, move_count};
use crate::src::end::{get_earliest_wld_solve, get_earliest_full_solve};

pub static mut database_name: [i8; 256] = [0; 256];
pub static mut binary_database: i32 = 0;
pub static mut learn_depth: i32 = 0;
pub static mut cutoff_empty: i32 = 0;
pub static mut game_move: [i16; 61] = [0; 61];
/*
   File:          learn.h

   Created:       November 29, 1997

   Modified:      November 18, 2001

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The interface to the learning module.
*/
/*
   CLEAR_STORED_GAME
   Remove all stored moves.
*/

pub unsafe fn clear_stored_game() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i <= 60 as i32 {
        game_move[i as usize] = -(1 as i32) as i16;
        i += 1
    };
}
/*
   STORE_MOVE
   Mark the move MOVE as being played after DISKS_PLAYED moves
   had been played.
*/

pub unsafe fn store_move(mut disks_played_0: i32,
                         mut move_0: i32) {
    game_move[disks_played_0 as usize] = move_0 as i16;
}
/*
   SET_LEARNING_PARAMETERS
   Specify the depth to which deviations are checked for and the number
   of empty squares at which the game is considered over.
*/

pub unsafe fn set_learning_parameters(mut depth: i32,
                                      mut cutoff: i32) {
    learn_depth = depth;
    cutoff_empty = cutoff;
}

/*
   GAME_LEARNABLE
   Checks if the current game can be learned - i.e. if the moves of the
   game are available and the game was finished or contains enough
   moves to be learned anyway.
*/

pub unsafe fn game_learnable(mut finished: i32,
                             mut move_count_0: i32)
                             -> i32 {
    let mut i: i32 = 0;
    let mut moves_available: i32 = 0;
    moves_available = 1 as i32;
    i = 0 as i32;
    while i < move_count_0 && i < 60 as i32 - cutoff_empty {
        if game_move[i as usize] as i32 == -(1 as i32) {
            moves_available = 0 as i32
        }
        i += 1
    }
    return (moves_available != 0 &&
        (finished != 0 ||
            move_count_0 >= 60 as i32 - cutoff_empty)) as
        i32;
}

pub trait Learner {
    fn learn_game(game_length: i32, private_game: i32, save_database: i32);
}