use crate::src::stubs::{printf, fabs, time};
pub use engine::src::timer::*;
/*
  RESET_REAL_TIMER
*/

pub unsafe fn reset_real_timer() { time(&mut init_time); }
/*
  INIT_TIMER
  Initializes the timer. This is really only needed when
  CRON_SUPPORTED is defined; in this case the cron daemon
  is used for timing.
*/

pub unsafe fn init_timer() { reset_real_timer(); }
/*
  GET_REAL_TIMER
  Returns the time passed since the last call to init_timer() or reset_timer().
*/

pub unsafe fn get_real_timer() -> f64 {
    let mut curr_time: time_t = 0;
    time(&mut curr_time);
    return (curr_time - init_time) as f64;
}

/*
  GET_ELAPSED_TIME
  Returns the time passed since START_MOVE was called.
  This is the actual time, not adjusted for pondering.
*/

pub unsafe fn get_elapsed_time() -> f64 {
    return fabs(get_real_timer() - start_time);
}
/*
  START_MOVE
*/

pub unsafe fn start_move(mut in_total_time: f64,
                                    mut increment: f64,
                                    mut discs: i32) {
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
    panic_abort = 0 as i32;
    start_time = get_real_timer();
}
/*
  CHECK_PANIC_ABORT
  Checks if the alotted time has been used up and in this case
  sets the PANIC_ABORT flags.
*/

pub unsafe fn check_panic_abort() {
    let mut curr_time: f64 = 0.;
    let mut adjusted_total_time: f64 = 0.;
    curr_time = get_elapsed_time();
    adjusted_total_time = total_move_time;
    if do_check_abort != 0 && curr_time >= panic_value * adjusted_total_time {
        panic_abort = 1 as i32
    };
}
/*
  CHECK_THRESHOLD
  Checks if a certain fraction of the panic time has been used.
*/

pub unsafe fn check_threshold(mut threshold: f64)
 -> i32 {
    let mut curr_time: f64 = 0.;
    let mut adjusted_total_time: f64 = 0.;
    curr_time = get_elapsed_time();
    adjusted_total_time = total_move_time;
    return (do_check_abort != 0 &&
                curr_time >= panic_value * threshold * adjusted_total_time) as
               i32;
}
/*
  ADJUST_CURRENT_PONDER_TIME
  The ponder time for the move actually made in the position where
  pondering was made is stored.
*/

pub unsafe fn adjust_current_ponder_time(mut move_0: i32) {
    current_ponder_time = ponder_time[move_0 as usize];
    current_ponder_depth = ponder_depth[move_0 as usize];
    printf(b"Ponder time: %.1f s\n\x00" as *const u8 as *const i8,
           current_ponder_time);
    printf(b"Ponder depth: %d\n\x00" as *const u8 as *const i8,
           current_ponder_depth);
}
/*
  ABOVE_RECOMMENDED
  EXTENDED_ABOVE_RECOMMENDED
  Checks if the time spent searching is greater than the threshold
  where no new iteration should be started.
  The extended version takes the ponder time into account.
*/

pub unsafe fn above_recommended() -> i32 {
    return (get_elapsed_time() >= time_per_move) as i32;
}

pub unsafe fn extended_above_recommended() -> i32 {
    return (get_elapsed_time() + frozen_ponder_time >= 1.5f64 * time_per_move)
               as i32;
}
