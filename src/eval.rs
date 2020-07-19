use ::libc;
extern "C" {
    /* The number of positions evaluated during the current search. */
    #[no_mangle]
    static mut evaluations: CounterType;
    #[no_mangle]
    static mut piece_count: [[libc::c_int; 64]; 3];
    /*
   File:           moves.h

   Created:        June 30, 1997

   Modified:       August 1, 2002

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       The move generator's interface.
*/
    /* The number of disks played from the initial position.
   Must match the current status of the BOARD variable. */
    #[no_mangle]
    static mut disks_played: libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CounterType {
    pub hi: libc::c_uint,
    pub lo: libc::c_uint,
}
/*
   File:           eval.c

   Created:        July 1, 1997

   Modified:       September 22, 1999

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Control mechanisms for board evaluation.
                   No knowledge in this module though.
*/
/* Local variables */
static mut use_experimental: libc::c_int = 0;
/*
   TOGGLE_EXPERIMENTAL
   Toggles usage of novelties in the evaluation function on/off.
*/
#[no_mangle]
pub unsafe extern "C" fn toggle_experimental(mut use_0: libc::c_int) {
    use_experimental = use_0;
}
/*
  EXPERIMENTAL_EVAL
  Returns 1 if the experimental eval (if there is such) is used,
  0 otherwise.
*/
#[no_mangle]
pub unsafe extern "C" fn experimental_eval() -> libc::c_int {
    return use_experimental;
}
/*
  INIT_EVAL
  Reset the evaluation module.
*/
#[no_mangle]
pub unsafe extern "C" fn init_eval() { }
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
#[no_mangle]
pub unsafe extern "C" fn terminal_evaluation(mut side_to_move: libc::c_int)
 -> libc::c_int {
    let mut disc_diff: libc::c_int = 0;
    let mut my_discs: libc::c_int = 0;
    let mut opp_discs: libc::c_int = 0;
    evaluations.lo = evaluations.lo.wrapping_add(1);
    my_discs = piece_count[side_to_move as usize][disks_played as usize];
    opp_discs =
        piece_count[(0 as libc::c_int + 2 as libc::c_int - side_to_move) as
                        usize][disks_played as usize];
    if my_discs > opp_discs {
        disc_diff = 64 as libc::c_int - 2 as libc::c_int * opp_discs
    } else if opp_discs > my_discs {
        disc_diff = 2 as libc::c_int * my_discs - 64 as libc::c_int
    } else { disc_diff = 0 as libc::c_int }
    if disc_diff > 0 as libc::c_int {
        return 29000 as libc::c_int + disc_diff
    } else if disc_diff == 0 as libc::c_int {
        return 0 as libc::c_int
    } else { return -(29000 as libc::c_int) + disc_diff };
}
