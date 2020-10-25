use crate::src::moves::disks_played;
use crate::src::globals::piece_count___;
use crate::src::search::search_state;


/*
  TERMINAL_EVALUATION
  Evaluates the position when no player has any legal moves.
*/

pub unsafe fn terminal_evaluation(side_to_move: i32) -> i32 {
    let eval_counter = &mut search_state.evaluations;
    let my_discs = piece_count___[side_to_move as usize][disks_played as usize];
    let opp_discs = piece_count___[(2 - side_to_move) as usize][disks_played as usize];

    eval_counter.lo = eval_counter.lo.wrapping_add(1);
    let disc_diff = if my_discs > opp_discs {
         64 - 2 * opp_discs
    } else if opp_discs > my_discs {
         2 * my_discs - 64
    } else {
         0
    };
    return if disc_diff > 0 {
        29000 + disc_diff
    } else if disc_diff == 0 {
        0
    } else {
        -29000 + disc_diff
    };
}
