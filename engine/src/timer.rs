use crate::src::stubs::{fabs};
use crate::src::error::FrontEnd;

pub type time_t = i64;

pub trait TimeSource{
    fn time(&self, __timer: &mut time_t) -> time_t;
}

pub struct Timer {
    source: &'static dyn TimeSource,
    frozen_ponder_time: f64,
    panic_value: f64,
    time_per_move: f64,
    start_time: f64,
    total_move_time: f64,
    ponder_time: [f64; 100],
    panic_abort: i32,
    do_check_abort: i32,
    init_time: time_t,
    current_ponder_depth: i32,
    current_ponder_time: f64,
    pub last_panic_check: f64,
    pub ponder_depth: [i32; 100],
    pub frozen_ponder_depth: i32,
}

impl Timer {
    pub fn new(source: &'static dyn TimeSource) -> Self {
        Timer {
            source,
            frozen_ponder_time: 0.,
            panic_value: 0.,
            time_per_move: 0.,
            start_time: 0.,
            total_move_time: 0.,
            ponder_time: [0.; 100],
            panic_abort: 0,
            do_check_abort: 1,
            init_time: 0,
            current_ponder_depth: 0,
            current_ponder_time: 0.,
            last_panic_check: 0.,
            ponder_depth: [0; 100],
            frozen_ponder_depth: 0,
        }
    }
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

impl Timer {
    /*
      SET_DEFAULT_PANIC
      Sets the panic timeout when search immediately must stop.
    */

    fn set_default_panic(&mut self) {
        self.panic_value = self.time_per_move * (1.6f64 / 0.7f64) / self.total_move_time;
    }
    /*
        DETERMINE_MOVE_TIME
        Initializes the timing subsystem and allocates time for the current move.
    */
    pub fn determine_move_time(&mut self, time_left: f64, incr: f64, discs: i32) {
        let timer_ = self;
        let mut time_available: f64 = 0.;
        let mut moves_left: i32 = 0;
        timer_.frozen_ponder_time = timer_.current_ponder_time;
        timer_.frozen_ponder_depth = timer_.current_ponder_depth;
        moves_left =
            if (65 as i32 - discs) / 2 as i32 - 5 as i32 >
                2 as i32 {
                ((65 as i32 - discs) / 2 as i32) -
                    5 as i32
            } else { 2 as i32 };
        time_available =
            time_left + timer_.frozen_ponder_time + moves_left as f64 * incr -
                10.0f64;
        if time_available < 1.0f64 { time_available = 1.0f64 }
        timer_.time_per_move =
            time_available / (moves_left + 1 as i32) as f64 *
                0.7f64;
        if timer_.time_per_move > time_left / 4 as i32 as f64 {
            timer_.time_per_move = time_left / 4 as i32 as f64
        }
        if timer_.time_per_move > time_left + timer_.frozen_ponder_time {
            timer_.time_per_move = time_left / 4 as i32 as f64
        }
        if timer_.time_per_move == 0 as i32 as f64 {
            timer_.time_per_move = 1 as i32 as f64
        }
        timer_.set_default_panic();
    }
    /*
      TOGGLE_ABORT_CHECK
     Enables/disables panic time abort checking.
    */
    pub fn toggle_abort_check(&mut self, enable: i32) {
        self.do_check_abort = enable;
    }
    /*
      IS_PANIC_ABORT
      Returns the current panic status.
    */

    pub fn is_panic_abort(&self) -> i32 {
        return self.panic_abort;
    }
    /*
      CLEAR_PANIC_ABORT
      Resets the panic abort flag.
    */
    pub fn clear_panic_abort(&mut self) {
        self.panic_abort = 0;
    }

    /*
      ADD_PONDER_TIME
      Increases the timer keeping track of the ponder time for
      a certain move.
    */
    pub fn add_ponder_time(&mut self, move_0: i32, time_0: f64) {
        self.ponder_time[move_0 as usize] += time_0;
    }
    /*
      SET_PANIC_THRESHOLD
      Specifies the fraction of the remaining time (VALUE must lie in [0,1])
      before the panic timeout kicks in.
    */
    pub fn set_panic_threshold(&mut self, value: f64) {
        self.panic_value = value;
    }

    /*
      RESET_REAL_TIMER
    */
    pub fn reset_real_timer(&mut self) { self.source.time(&mut self.init_time); }
    /*
      INIT_TIMER
      Initializes the timer. This is really only needed when
      CRON_SUPPORTED is defined; in this case the cron daemon
      is used for timing.
    */
    pub fn init_timer(&mut self) { self.reset_real_timer(); }

    /*
      CLEAR_PONDER_TIMES
      Clears the ponder times for all board positions and resets
      the time associated with the last move actually made.
    */

    pub fn clear_ponder_times(&mut self) {
        let timer = self;
        let mut i: i32 = 0;
        i = 0;
        while i < 100 as i32 {
            timer.ponder_time[i as usize] = 0.0f64;
            timer.ponder_depth[i as usize] = 0;
            i += 1
        }
        timer.current_ponder_time = 0.0f64;
        timer.current_ponder_depth = 0;
    }
    /*
      GET_REAL_TIMER
      Returns the time passed since the last call to timer.init_timer() or reset_timer().
    */

    pub fn get_real_timer(&self) -> f64 {
        let mut curr_time: time_t = 0;
        self.source.time(&mut curr_time);
        return (curr_time - self.init_time) as f64;
    }

    /*
      GET_ELAPSED_TIME
      Returns the time passed since START_MOVE was called.
      This is the actual time, not adjusted for pondering.
    */

    pub fn get_elapsed_time(&self) -> f64 {
        return fabs(self.get_real_timer() - self.start_time);
    }
    /*
      START_MOVE
    */

    pub fn start_move(&mut self, in_total_time: f64,
                                    _increment: f64,
                                    _discs: i32) {
        /*
          This is a possible approach in time control games with increment:
            available_time = in_total_time + increment * (65 - discs) / 2.0;
          Some correction might be necessary anyway, so let's skip it for now.
        */
        /* This won't work well in games with time increment, but never mind */
        self.total_move_time =
            if in_total_time - 10.0f64 > 0.1f64 {
                (in_total_time) - 10.0f64
            } else { 0.1f64 };
        self.panic_abort = 0;
        self.start_time = self.get_real_timer();
    }
    /*
      CHECK_PANIC_ABORT
      Checks if the alotted time has been used up and in this case
      sets the timer.panic_abort flags.
    */

    pub fn check_panic_abort(&mut self) {
        let mut curr_time: f64 = 0.;
        let mut adjusted_total_time: f64 = 0.;
        curr_time = self.get_elapsed_time();
        adjusted_total_time = self.total_move_time;
        if self.do_check_abort != 0 && curr_time >= self.panic_value * adjusted_total_time {
            self.panic_abort = 1 as i32
        };
    }
    /*
      CHECK_THRESHOLD
      Checks if a certain fraction of the panic time has been used.
    */

    pub fn check_threshold(&self, threshold: f64)
                                         -> i32 {
        let mut curr_time: f64 = 0.;
        let mut adjusted_total_time: f64 = 0.;
        curr_time = self.get_elapsed_time();
        adjusted_total_time = self.total_move_time;
        return (self.do_check_abort != 0 &&
            curr_time >= self.panic_value * threshold * adjusted_total_time) as
            i32;
    }
    /*
      ABOVE_RECOMMENDED
      EXTENDED_ABOVE_RECOMMENDED
      Checks if the time spent searching is greater than the threshold
      where no new iteration should be started.
      The extended version takes the ponder time into account.
    */

    pub fn above_recommended(&self) -> i32 {
        return (self.get_elapsed_time() >= self.time_per_move) as i32;
    }

    pub fn extended_above_recommended(&self) -> i32 {
        return (self.get_elapsed_time() + self.frozen_ponder_time >= 1.5f64 * self.time_per_move)
            as i32;
    }
}

// (clear_ponder_times|get_real_timer|get_elapsed_time|start_move|check_panic_abort|check_threshold|above_recommended|extended_above_recommended)