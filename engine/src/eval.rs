use crate::src::moves::disks_played;
use crate::src::globals::piece_count;
use crate::src::search::evaluations;


/*
  TERMINAL_EVALUATION
  Evaluates the position when no player has any legal moves.
*/

pub unsafe fn terminal_evaluation(side_to_move: i32)
 -> i32 {
    let mut disc_diff: i32 = 0;
    let mut my_discs: i32 = 0;
    let mut opp_discs: i32 = 0;
    evaluations.lo = evaluations.lo.wrapping_add(1);
    my_discs = piece_count[side_to_move as usize][disks_played as usize];
    opp_discs =
        piece_count[(0 as i32 + 2 as i32 - side_to_move) as
                        usize][disks_played as usize];
    if my_discs > opp_discs {
        disc_diff = 64 as i32 - 2 as i32 * opp_discs
    } else if opp_discs > my_discs {
        disc_diff = 2 as i32 * my_discs - 64 as i32
    } else { disc_diff = 0 as i32 }
    if disc_diff > 0 as i32 {
        return 29000 as i32 + disc_diff
    } else if disc_diff == 0 as i32 {
        return 0 as i32
    } else { return -(29000 as i32) + disc_diff };
}
