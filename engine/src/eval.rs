use crate::src::moves::disks_played;
use crate::src::globals::piece_count;
use crate::src::search::evaluations;


/*
   File:           eval.c

   Created:        July 1, 1997

   Modified:       September 22, 1999

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Control mechanisms for board evaluation.
                   No knowledge in this module though.
*/
/* Local variables */
static mut use_experimental: i32 = 0;
/*
   TOGGLE_EXPERIMENTAL
   Toggles usage of novelties in the evaluation function on/off.
*/

pub unsafe fn toggle_experimental(mut use_0: i32) {
    use_experimental = use_0;
}
/*
  EXPERIMENTAL_EVAL
  Returns 1 if the experimental eval (if there is such) is used,
  0 otherwise.
*/

pub unsafe fn experimental_eval() -> i32 {
    return use_experimental;
}
/*
  INIT_EVAL
  Reset the evaluation module.
*/

pub unsafe fn init_eval() { }
/*
   File:           eval.h

   Created:        July 1, 1997

   Modified:       September 15, 2001

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       The interface to the evaluation function.
*/
/* An evaluation indicating a won midgame position where no
   player has any moves available. */
/* An eval so high it must have originated from a midgame win
   disturbed by some randomness. */
/*
  TERMINAL_EVALUATION
  Evaluates the position when no player has any legal moves.
*/

pub unsafe fn terminal_evaluation(mut side_to_move: i32)
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
