use crate::src::globals::PieceCounts;
use crate::src::counter::CounterType;


/*
  TERMINAL_EVALUATION
  Evaluates the position when no player has any legal moves.
*/

pub fn terminal_evaluation(counts: PieceCounts, eval_counter: &mut CounterType) -> i32 {
    let PieceCounts { my_discs, opp_discs } = counts;

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
