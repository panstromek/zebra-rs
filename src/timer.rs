use crate::src::libc;
use crate::src::stubs::{printf, fabs, time};

pub type __time_t = libc::c_long;
pub type time_t = __time_t;
/* Global variables */

pub static mut last_panic_check: libc::c_double = 0.;

pub static mut ponder_depth: [libc::c_int; 100] = [0; 100];

pub static mut current_ponder_depth: libc::c_int = 0;

pub static mut frozen_ponder_depth: libc::c_int = 0;
/* Local variables */

pub static mut current_ponder_time: libc::c_double = 0.;

pub static mut frozen_ponder_time: libc::c_double = 0.;
static mut panic_value: libc::c_double = 0.;
static mut time_per_move: libc::c_double = 0.;
static mut start_time: libc::c_double = 0.;
static mut total_move_time: libc::c_double = 0.;
static mut ponder_time: [libc::c_double; 100] = [0.; 100];
static mut panic_abort: libc::c_int = 0;
static mut do_check_abort: libc::c_int = 1 as libc::c_int;
static mut init_time: time_t = 0;
/*
  RESET_REAL_TIMER
*/

pub unsafe extern "C" fn reset_real_timer() { time(&mut init_time); }
/*
  INIT_TIMER
  Initializes the timer. This is really only needed when
  CRON_SUPPORTED is defined; in this case the cron daemon
  is used for timing.
*/

pub unsafe extern "C" fn init_timer() { reset_real_timer(); }
/*
  GET_REAL_TIMER
  Returns the time passed since the last call to init_timer() or reset_timer().
*/

pub unsafe extern "C" fn get_real_timer() -> libc::c_double {
    let mut curr_time: time_t = 0;
    time(&mut curr_time);
    return (curr_time - init_time) as libc::c_double;
}
/*
  SET_DEFAULT_PANIC
  Sets the panic timeout when search immediately must stop.
*/

pub unsafe extern "C" fn set_default_panic() {
    panic_value = time_per_move * (1.6f64 / 0.7f64) / total_move_time;
}
/*
   File:          timer.h

   Created:       September 28, 1997

   Modified:      August 1, 2002

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The time control mechanism.
*/
/* Holds the value of the variable NODES the last time the
   timer module was called to check if a panic abort occured. */
/*
  DETERMINE_MOVE_TIME
  Initializes the timing subsystem and allocates time for the current move.
*/

pub unsafe extern "C" fn determine_move_time(mut time_left: libc::c_double,
                                             mut incr: libc::c_double,
                                             mut discs: libc::c_int) {
    let mut time_available: libc::c_double = 0.;
    let mut moves_left: libc::c_int = 0;
    frozen_ponder_time = current_ponder_time;
    frozen_ponder_depth = current_ponder_depth;
    moves_left =
        if (65 as libc::c_int - discs) / 2 as libc::c_int - 5 as libc::c_int >
               2 as libc::c_int {
            ((65 as libc::c_int - discs) / 2 as libc::c_int) -
                5 as libc::c_int
        } else { 2 as libc::c_int };
    time_available =
        time_left + frozen_ponder_time + moves_left as libc::c_double * incr -
            10.0f64;
    if time_available < 1.0f64 { time_available = 1.0f64 }
    time_per_move =
        time_available / (moves_left + 1 as libc::c_int) as libc::c_double *
            0.7f64;
    if time_per_move > time_left / 4 as libc::c_int as libc::c_double {
        time_per_move = time_left / 4 as libc::c_int as libc::c_double
    }
    if time_per_move > time_left + frozen_ponder_time {
        time_per_move = time_left / 4 as libc::c_int as libc::c_double
    }
    if time_per_move == 0 as libc::c_int as libc::c_double {
        time_per_move = 1 as libc::c_int as libc::c_double
    }
    set_default_panic();
}
/*
  GET_ELAPSED_TIME
  Returns the time passed since START_MOVE was called.
  This is the actual time, not adjusted for pondering.
*/

pub unsafe extern "C" fn get_elapsed_time() -> libc::c_double {
    return fabs(get_real_timer() - start_time);
}
/*
  START_MOVE
*/

pub unsafe extern "C" fn start_move(mut in_total_time: libc::c_double,
                                    mut increment: libc::c_double,
                                    mut discs: libc::c_int) {
    /*
      This is a possible approach in time control games with increment:
        available_time = in_total_time + increment * (65 - discs) / 2.0;
      Some correction might be necessary anyway, so let's skip it for now.
    */
    /* This won't work well in games with time increment, but never mind */
    total_move_time =
        if in_total_time - 10.0f64 > 0.1f64 {
            (in_total_time) - 10.0f64
        } else { 0.1f64 };
    panic_abort = 0 as libc::c_int;
    start_time = get_real_timer();
}
/*
  SET_PANIC_THRESHOLD
  Specifies the fraction of the remaining time (VALUE must lie in [0,1])
  before the panic timeout kicks in.
*/

pub unsafe extern "C" fn set_panic_threshold(mut value: libc::c_double) {
    panic_value = value;
}
/*
  CHECK_PANIC_ABORT
  Checks if the alotted time has been used up and in this case
  sets the PANIC_ABORT flags.
*/

pub unsafe extern "C" fn check_panic_abort() {
    let mut curr_time: libc::c_double = 0.;
    let mut adjusted_total_time: libc::c_double = 0.;
    curr_time = get_elapsed_time();
    adjusted_total_time = total_move_time;
    if do_check_abort != 0 && curr_time >= panic_value * adjusted_total_time {
        panic_abort = 1 as libc::c_int
    };
}
/*
  CHECK_THRESHOLD
  Checks if a certain fraction of the panic time has been used.
*/

pub unsafe extern "C" fn check_threshold(mut threshold: libc::c_double)
 -> libc::c_int {
    let mut curr_time: libc::c_double = 0.;
    let mut adjusted_total_time: libc::c_double = 0.;
    curr_time = get_elapsed_time();
    adjusted_total_time = total_move_time;
    return (do_check_abort != 0 &&
                curr_time >= panic_value * threshold * adjusted_total_time) as
               libc::c_int;
}
/*
  TOGGLE_ABORT_CHECK
  Enables/disables panic time abort checking.
*/

pub unsafe extern "C" fn toggle_abort_check(mut enable: libc::c_int) {
    do_check_abort = enable;
}
/*
  CLEAR_PANIC_ABORT
  Resets the panic abort flag.
*/

pub unsafe extern "C" fn clear_panic_abort() {
    panic_abort = 0 as libc::c_int;
}
/*
  IS_PANIC_ABORT
  Returns the current panic status.
*/

pub unsafe extern "C" fn is_panic_abort() -> libc::c_int {
    return panic_abort;
}
/*
  CLEAR_PONDER_TIMES
  Clears the ponder times for all board positions and resets
  the time associated with the last move actually made.
*/

pub unsafe extern "C" fn clear_ponder_times() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        ponder_time[i as usize] = 0.0f64;
        ponder_depth[i as usize] = 0 as libc::c_int;
        i += 1
    }
    current_ponder_time = 0.0f64;
    current_ponder_depth = 0 as libc::c_int;
}
/*
  ADD_PONDER_TIME
  Increases the timer keeping track of the ponder time for
  a certain move.
*/

pub unsafe extern "C" fn add_ponder_time(mut move_0: libc::c_int,
                                         mut time_0: libc::c_double) {
    ponder_time[move_0 as usize] += time_0;
}
/*
  ADJUST_CURRENT_PONDER_TIME
  The ponder time for the move actually made in the position where
  pondering was made is stored.
*/

pub unsafe extern "C" fn adjust_current_ponder_time(mut move_0: libc::c_int) {
    current_ponder_time = ponder_time[move_0 as usize];
    current_ponder_depth = ponder_depth[move_0 as usize];
    printf(b"Ponder time: %.1f s\n\x00" as *const u8 as *const libc::c_char,
           current_ponder_time);
    printf(b"Ponder depth: %d\n\x00" as *const u8 as *const libc::c_char,
           current_ponder_depth);
}
/*
  ABOVE_RECOMMENDED
  EXTENDED_ABOVE_RECOMMENDED
  Checks if the time spent searching is greater than the threshold
  where no new iteration should be started.
  The extended version takes the ponder time into account.
*/

pub unsafe extern "C" fn above_recommended() -> libc::c_int {
    return (get_elapsed_time() >= time_per_move) as libc::c_int;
}

pub unsafe extern "C" fn extended_above_recommended() -> libc::c_int {
    return (get_elapsed_time() + frozen_ponder_time >= 1.5f64 * time_per_move)
               as libc::c_int;
}
