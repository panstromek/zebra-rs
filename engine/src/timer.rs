use crate::src::stubs::{fabs};
use crate::src::error::FrontEnd;

pub type time_t = i64;
/* Global variables */

pub static mut last_panic_check: f64 = 0.;

pub static mut ponder_depth: [i32; 100] = [0; 100];

pub static mut current_ponder_depth: i32 = 0;

pub static mut frozen_ponder_depth: i32 = 0;
/* Local variables */

pub static mut current_ponder_time: f64 = 0.;

struct Timer {
    frozen_ponder_time: f64,
    panic_value: f64,
    time_per_move: f64,
    start_time: f64,
    total_move_time: f64,
    ponder_time: [f64; 100],
    panic_abort: i32,
    do_check_abort: i32,
    init_time: time_t,
}

static mut timer: Timer = Timer {
    frozen_ponder_time: 0.,
    panic_value: 0.,
    time_per_move: 0.,
    start_time: 0.,
    total_move_time: 0.,
    ponder_time: [0.; 100],
    panic_abort: 0,
    do_check_abort: 1,
    init_time: 0,
};

/*
  SET_DEFAULT_PANIC
  Sets the panic timeout when search immediately must stop.
*/

pub unsafe fn set_default_panic() {
   timer.panic_value =timer. time_per_move * (1.6f64 / 0.7f64) / timer.total_move_time;
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

pub unsafe fn determine_move_time(time_left: f64,
                                  incr: f64,
                                  discs: i32) {
    let mut time_available: f64 = 0.;
    let mut moves_left: i32 = 0;
   timer.frozen_ponder_time = current_ponder_time;
    frozen_ponder_depth = current_ponder_depth;
    moves_left =
        if (65 as i32 - discs) / 2 as i32 - 5 as i32 >
            2 as i32 {
            ((65 as i32 - discs) / 2 as i32) -
                5 as i32
        } else { 2 as i32 };
    time_available =
        time_left +timer.frozen_ponder_time + moves_left as f64 * incr -
            10.0f64;
    if time_available < 1.0f64 { time_available = 1.0f64 }
   timer. time_per_move =
        time_available / (moves_left + 1 as i32) as f64 *
            0.7f64;
    if timer. time_per_move > time_left / 4 as i32 as f64 {
       timer. time_per_move = time_left / 4 as i32 as f64
    }
    if timer. time_per_move > time_left +timer.frozen_ponder_time {
       timer. time_per_move = time_left / 4 as i32 as f64
    }
    if timer. time_per_move == 0 as i32 as f64 {
       timer. time_per_move = 1 as i32 as f64
    }
    set_default_panic();
}

/*
  TOGGLE_ABORT_CHECK
  Enables/disables panic time abort checking.
*/

pub unsafe fn toggle_abort_check(enable: i32) {
    timer.do_check_abort = enable;
}
/*
  CLEAR_PANIC_ABORT
  Resets the panic abort flag.
*/

pub unsafe fn clear_panic_abort() {
    timer.panic_abort = 0;
}
/*
  IS_PANIC_ABORT
  Returns the current panic status.
*/

pub unsafe fn is_panic_abort() -> i32 {
    return timer.panic_abort;
}
/*
  CLEAR_PONDER_TIMES
  Clears the ponder times for all board positions and resets
  the time associated with the last move actually made.
*/

pub unsafe fn clear_ponder_times() {
    let mut i: i32 = 0;
    i = 0;
    while i < 100 as i32 {
        timer.ponder_time[i as usize] = 0.0f64;
        ponder_depth[i as usize] = 0;
        i += 1
    }
    current_ponder_time = 0.0f64;
    current_ponder_depth = 0;
}
/*
  ADD_PONDER_TIME
  Increases the timer keeping track of the ponder time for
  a certain move.
*/

pub unsafe fn add_ponder_time(move_0: i32,
                              time_0: f64) {
    timer.ponder_time[move_0 as usize] += time_0;
}
/*
  SET_PANIC_THRESHOLD
  Specifies the fraction of the remaining time (VALUE must lie in [0,1])
  before the panic timeout kicks in.
*/

pub unsafe fn set_panic_threshold(value: f64) {
   timer.panic_value = value;
}

/*
  RESET_REAL_TIMER
*/

pub unsafe fn reset_real_timer<FE: FrontEnd>() { FE::time(&mut timer.init_time); }
/*
  timer.init_timeR
  Initializes the timer. This is really only needed when
  CRON_SUPPORTED is defined; in this case the cron daemon
  is used for timing.
*/

pub unsafe fn init_timer<FE: FrontEnd>() { reset_real_timer::<FE>(); }
/*
  GET_REAL_TIMER
  Returns the time passed since the last call to timer.init_timer() or reset_timer().
*/

pub unsafe fn get_real_timer<FE :FrontEnd>() -> f64 {
    let mut curr_time: time_t = 0;
    FE::time(&mut curr_time);
    return (curr_time - timer.init_time) as f64;
}

/*
  GET_ELAPSED_TIME
  Returns the time passed since START_MOVE was called.
  This is the actual time, not adjusted for pondering.
*/

pub unsafe fn get_elapsed_time<FE:FrontEnd>() -> f64 {
    return fabs(get_real_timer::<FE>() -timer.  start_time);
}
/*
  START_MOVE
*/

pub unsafe fn start_move<FE: FrontEnd>(in_total_time: f64,
                         _increment: f64,
                         _discs: i32) {
    /*
      This is a possible approach in time control games with increment:
        available_time = in_total_time + increment * (65 - discs) / 2.0;
      Some correction might be necessary anyway, so let's skip it for now.
    */
    /* This won't work well in games with time increment, but never mind */
    timer.total_move_time =
        if in_total_time - 10.0f64 > 0.1f64 {
            (in_total_time) - 10.0f64
        } else { 0.1f64 };
    timer.panic_abort = 0;
   timer.  start_time = get_real_timer::<FE>();
}
/*
  CHECK_PANIC_ABORT
  Checks if the alotted time has been used up and in this case
  sets the timer.panic_abort flags.
*/

pub unsafe fn check_panic_abort<FE: FrontEnd>() {
    let mut curr_time: f64 = 0.;
    let mut adjusted_total_time: f64 = 0.;
    curr_time = get_elapsed_time::<FE>();
    adjusted_total_time = timer.total_move_time;
    if timer.do_check_abort != 0 && curr_time >=timer.panic_value * adjusted_total_time {
        timer.panic_abort = 1 as i32
    };
}
/*
  CHECK_THRESHOLD
  Checks if a certain fraction of the panic time has been used.
*/

pub unsafe fn check_threshold<FE: FrontEnd>(threshold: f64)
                              -> i32 {
    let mut curr_time: f64 = 0.;
    let mut adjusted_total_time: f64 = 0.;
    curr_time = get_elapsed_time::<FE>();
    adjusted_total_time = timer.total_move_time;
    return (timer.do_check_abort != 0 &&
        curr_time >=timer.panic_value * threshold * adjusted_total_time) as
        i32;
}
/*
  ABOVE_RECOMMENDED
  EXTENDED_ABOVE_RECOMMENDED
  Checks if the time spent searching is greater than the threshold
  where no new iteration should be started.
  The extended version takes the ponder time into account.
*/

pub unsafe fn above_recommended<FE: FrontEnd>() -> i32 {
    return (get_elapsed_time::<FE>() >=timer. time_per_move) as i32;
}

pub unsafe fn extended_above_recommended<FE: FrontEnd>() -> i32 {
    return (get_elapsed_time::<FE>() +timer.frozen_ponder_time >= 1.5f64 *timer. time_per_move)
        as i32;
}
