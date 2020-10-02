use std::ffi::c_void;
use crate::src::timer::get_real_timer;
use crate::src::error::FrontEnd;

pub static mut echo: i32 = 0;
pub static mut display_pv: i32 = 0;


/* Local variables */
pub static mut black_player: *mut i8 = 0 as *const i8 as *mut i8;
pub static mut white_player: *mut i8 = 0 as *const i8 as *mut i8;
pub static mut status_buffer: [i8; 256] = [0; 256];
pub static mut sweep_buffer: [i8; 256] = [0; 256];
pub static mut black_eval: f64 = 0.0f64;
pub static mut white_eval: f64 = 0.0f64;
pub static mut last_output: f64 = 0.0f64;
pub static mut interval1: f64 = 0.;
pub static mut interval2: f64 = 0.;
pub static mut black_time: i32 = 0;
pub static mut white_time: i32 = 0;
pub static mut current_row: i32 = 0;
pub static mut status_modified: i32 = 0;
pub static mut sweep_modified: i32 = 0;
pub static mut timed_buffer_management: i32 = 1;
pub static mut status_pos: i32 = 0;
pub static mut sweep_pos: i32 = 0;

/*
  SET_NAMES
  SET_TIMES
  SET_EVALS
  SET_MOVE_LIST
  Specify some information to be output along with the
  board by DISPLAY_BOARD.
*/

pub unsafe fn set_names<FE: FrontEnd>(black_name: *const i8, white_name: *const i8) {
    if !black_player.is_null() { FE::free(black_player as *mut c_void); }
    if !white_player.is_null() { FE::free(white_player as *mut c_void); }
    black_player = FE::strdup(black_name);
    white_player = FE::strdup(white_name);
}

pub unsafe fn set_times(black: i32, white: i32) {
    black_time = black;
    white_time = white;
}

pub unsafe fn set_evals(black: f64, white: f64) {
    black_eval = black;
    white_eval = white;
}

pub unsafe fn set_move_list(black: *mut i32, white: *mut i32, row: i32) {
    current_row = row;
}

/*
  CLEAR_STATUS
  Clear the current status information.
*/

pub unsafe fn clear_status() {
    status_pos = 0;
    status_buffer[0] = 0;
    status_modified = 1;
}

/*
  CLEAR_SWEEP
  Clear the search information.
*/

pub unsafe fn clear_sweep() {
    sweep_pos = 0;
    sweep_buffer[0] = 0;
    sweep_modified = 1;
}

/*
  TOGGLE_SMART_BUFFER_MANAGEMENT
  Allow the user between timed, "smart", buffer management
  and the simple "you asked for it, you got it"-approach which
  displays everything that is fed to the buffer.
*/

pub unsafe fn toggle_smart_buffer_management(use_smart: i32) {
    timed_buffer_management = use_smart;
}
/*
  RESET_BUFFER_DISPLAY
  Clear all buffers and initialize time variables.
*/

pub unsafe fn reset_buffer_display<FE: FrontEnd>() {
    /* The first two Fibonacci numbers */
    clear_status();
    clear_sweep();
    interval1 = 0.0f64;
    interval2 = 1.0f64;
    last_output = get_real_timer::<FE>();
}