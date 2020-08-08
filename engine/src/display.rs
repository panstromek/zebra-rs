use crate::src::stubs::{free, strdup};
use std::ffi::c_void;
use crate::src::timer::get_real_timer;

extern "C" {
    #[no_mangle]
    pub fn display_buffers();
}
pub type EvalType = u32;
pub const UNINITIALIZED_EVAL: EvalType = 8;
pub const INTERRUPTED_EVAL: EvalType = 7;
pub const UNDEFINED_EVAL: EvalType = 6;
pub const PASS_EVAL: EvalType = 5;
pub const FORCED_EVAL: EvalType = 4;
pub const SELECTIVE_EVAL: EvalType = 3;
pub const WLD_EVAL: EvalType = 2;
pub const EXACT_EVAL: EvalType = 1;
pub const MIDGAME_EVAL: EvalType = 0;
pub type EvalResult = u32;
pub const UNSOLVED_POSITION: EvalResult = 3;
pub const LOST_POSITION: EvalResult = 2;
pub const DRAWN_POSITION: EvalResult = 1;
pub const WON_POSITION: EvalResult = 0;



pub static mut echo: i32 = 0;
pub static mut display_pv: i32 = 0;


/* Local variables */
pub static mut black_player: *mut i8 = 0 as *const i8 as *mut i8;
pub static mut white_player: *mut i8 = 0 as *const i8 as *mut i8;
pub static mut status_buffer: [i8; 256] = [0; 256];
pub static mut sweep_buffer: [i8; 256] = [0; 256];
pub static mut stored_status_buffer: [i8; 256] = [0; 256];
pub static mut black_eval: f64 = 0.0f64;
pub static mut white_eval: f64 = 0.0f64;
pub static mut last_output: f64 = 0.0f64;
pub static mut interval1: f64 = 0.;
pub static mut interval2: f64 = 0.;
pub static mut black_time: i32 = 0;
pub static mut white_time: i32 = 0;
pub static mut current_row: i32 = 0;
pub static mut status_modified: i32 = 0 as i32;
pub static mut sweep_modified: i32 = 0 as i32;
pub static mut timed_buffer_management: i32 = 1 as i32;
pub static mut status_pos: i32 = 0;
pub static mut sweep_pos: i32 = 0;
pub static mut black_list: *mut i32 = 0 as *const i32 as *mut i32;
pub static mut white_list: *mut i32 = 0 as *const i32 as *mut i32;


/*
  SET_NAMES
  SET_TIMES
  SET_EVALS
  SET_MOVE_LIST
  Specify some information to be output along with the
  board by DISPLAY_BOARD.
*/

pub unsafe fn set_names(mut black_name: *const i8,
                        mut white_name: *const i8) {
    if !black_player.is_null() { free(black_player as *mut c_void); }
    if !white_player.is_null() { free(white_player as *mut c_void); }
    black_player = strdup(black_name);
    white_player = strdup(white_name);
}

pub unsafe fn set_times(mut black: i32,
                        mut white: i32) {
    black_time = black;
    white_time = white;
}

pub unsafe fn set_evals(mut black: f64,
                        mut white: f64) {
    black_eval = black;
    white_eval = white;
}

pub unsafe fn set_move_list(mut black: *mut i32,
                            mut white: *mut i32,
                            mut row: i32) {
    black_list = black;
    white_list = white;
    current_row = row;
}

/*
  CLEAR_STATUS
  Clear the current status information.
*/

pub unsafe fn clear_status() {
    status_pos = 0 as i32;
    status_buffer[0 as i32 as usize] =
        0 as i32 as i8;
    status_modified = 1 as i32;
}

pub unsafe fn get_last_status() -> *const i8 {
    return stored_status_buffer.as_mut_ptr();
}

/*
  CLEAR_SWEEP
  Clear the search information.
*/

pub unsafe fn clear_sweep() {
    sweep_pos = 0 as i32;
    sweep_buffer[0 as i32 as usize] =
        0 as i32 as i8;
    sweep_modified = 1 as i32;
}

/*
  TOGGLE_SMART_BUFFER_MANAGEMENT
  Allow the user between timed, "smart", buffer management
  and the simple "you asked for it, you got it"-approach which
  displays everything that is fed to the buffer.
*/

pub unsafe fn toggle_smart_buffer_management(mut use_smart:
                                             i32) {
    timed_buffer_management = use_smart;
}
/*
  RESET_BUFFER_DISPLAY
  Clear all buffers and initialize time variables.
*/

pub unsafe fn reset_buffer_display() {
    /* The first two Fibonacci numbers */
    clear_status();
    clear_sweep();
    interval1 = 0.0f64;
    interval2 = 1.0f64;
    last_output = get_real_timer();
}