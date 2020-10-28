use crate::src::moves::moves_state;
use crate::src::globals::{board_state, PieceCounts};
use crate::src::search::search_state;


/*
  TERMINAL_EVALUATION
  Evaluates the position when no player has any legal moves.
*/

pub unsafe fn terminal_evaluation(side_to_move: i32) -> i32 {
    let eval_counter = &mut search_state.evaluations;
    let disks_played = moves_state.disks_played;
    let state = &board_state;
    let PieceCounts {
        my_discs, opp_discs
    } = state.get_piece_counts(side_to_move, disks_played);

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
