use crate::src::stubs::{sprintf, floor, fprintf, vsprintf, strlen, ceil, fputs, fputc, strdup, exit, abs, strcpy, free, getc, stdout, stdin};
use crate::src::safemem::safe_malloc;
use crate::src::timer::get_real_timer;
use crate::src::search::{full_pv, full_pv_depth, disc_count};
use crate::src::globals::{white_moves, black_moves, pv_depth};
use crate::src::libc;
use crate::src::zebra::{EvaluationType, _IO_FILE};
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type EvalType = libc::c_uint;
pub const UNINITIALIZED_EVAL: EvalType = 8;
pub const INTERRUPTED_EVAL: EvalType = 7;
pub const UNDEFINED_EVAL: EvalType = 6;
pub const PASS_EVAL: EvalType = 5;
pub const FORCED_EVAL: EvalType = 4;
pub const SELECTIVE_EVAL: EvalType = 3;
pub const WLD_EVAL: EvalType = 2;
pub const EXACT_EVAL: EvalType = 1;
pub const MIDGAME_EVAL: EvalType = 0;
pub type EvalResult = libc::c_uint;
pub const UNSOLVED_POSITION: EvalResult = 3;
pub const LOST_POSITION: EvalResult = 2;
pub const DRAWN_POSITION: EvalResult = 1;
pub const WON_POSITION: EvalResult = 0;

/*
   File:           display.c

   Created:        July 10, 1997

   Modified:       November 23, 2001

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Some I/O routines.
*/
/* Global variables */

pub static mut echo: libc::c_int = 0;

pub static mut display_pv: libc::c_int = 0;
/* Local variables */
static mut black_player: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut white_player: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut status_buffer: [libc::c_char; 256] = [0; 256];
static mut sweep_buffer: [libc::c_char; 256] = [0; 256];
static mut stored_status_buffer: [libc::c_char; 256] = [0; 256];
static mut black_eval: libc::c_double = 0.0f64;
static mut white_eval: libc::c_double = 0.0f64;
static mut last_output: libc::c_double = 0.0f64;
static mut interval1: libc::c_double = 0.;
static mut interval2: libc::c_double = 0.;
static mut black_time: libc::c_int = 0;
static mut white_time: libc::c_int = 0;
static mut current_row: libc::c_int = 0;
static mut status_modified: libc::c_int = 0 as libc::c_int;
static mut sweep_modified: libc::c_int = 0 as libc::c_int;
static mut timed_buffer_management: libc::c_int = 1 as libc::c_int;
static mut status_pos: libc::c_int = 0;
static mut sweep_pos: libc::c_int = 0;
static mut black_list: *mut libc::c_int =
    0 as *const libc::c_int as *mut libc::c_int;
static mut white_list: *mut libc::c_int =
    0 as *const libc::c_int as *mut libc::c_int;
/*
   File:         display.h

   Created:      July 10, 1997

   Modified:     November 17, 2002

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:     Declarations of the screen output functions.
*/
/* Flag variable, non-zero if output should be written to stdout. */
/* Flag variable, non-zero if the principal variation is to be
   displayed. */
/*
   DUMPCH
   Reads a character off standard input and terminates the program
   if the character typed is ' '.
*/

pub unsafe fn dumpch() {
    let mut ch: libc::c_char = 0;
    ch = getc(stdin) as libc::c_char;
    if ch as libc::c_int == ' ' as i32 { exit(1 as libc::c_int); };
}
/*
  SET_NAMES
  SET_TIMES
  SET_EVALS
  SET_MOVE_LIST
  Specify some information to be output along with the
  board by DISPLAY_BOARD.
*/

pub unsafe fn set_names(mut black_name: *const libc::c_char,
                                   mut white_name: *const libc::c_char) {
    if !black_player.is_null() { free(black_player as *mut libc::c_void); }
    if !white_player.is_null() { free(white_player as *mut libc::c_void); }
    black_player = strdup(black_name);
    white_player = strdup(white_name);
}

pub unsafe fn set_times(mut black: libc::c_int,
                                   mut white: libc::c_int) {
    black_time = black;
    white_time = white;
}

pub unsafe fn set_evals(mut black: libc::c_double,
                                   mut white: libc::c_double) {
    black_eval = black;
    white_eval = white;
}

pub unsafe fn set_move_list(mut black: *mut libc::c_int,
                                       mut white: *mut libc::c_int,
                                       mut row: libc::c_int) {
    black_list = black;
    white_list = white;
    current_row = row;
}
/*
   DISPLAY_BOARD
   side_to_move = the player whose turn it is
   black_moves = a list of black moves so far
   white_moves = a list of white moves so far
   current_row = the row of the score sheet

   The board is displayed using '*' for black and 'O' for white.
*/

pub unsafe fn display_board(mut stream: *mut FILE,
                                       mut board: *mut libc::c_int,
                                       mut side_to_move: libc::c_int,
                                       mut give_game_score: libc::c_int,
                                       mut give_time: libc::c_int,
                                       mut give_evals: libc::c_int) {
    let mut buffer: [libc::c_char; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut written: libc::c_int = 0;
    let mut first_row: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    if side_to_move == 0 as libc::c_int {
        first_row =
            if 0 as libc::c_int > current_row - 8 as libc::c_int {
                0 as libc::c_int
            } else { (current_row) - 8 as libc::c_int }
    } else {
        first_row =
            if 0 as libc::c_int > current_row - 7 as libc::c_int {
                0 as libc::c_int
            } else { (current_row) - 7 as libc::c_int }
    }
    buffer[15 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    fputs(b"\n\x00" as *const u8 as *const libc::c_char, stream);
    fprintf(stream,
            b"%s   a b c d e f g h\n\x00" as *const u8 as *const libc::c_char,
            b"      \x00" as *const u8 as *const libc::c_char);
    fputs(b"\n\x00" as *const u8 as *const libc::c_char, stream);
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 15 as libc::c_int {
            buffer[j as usize] = ' ' as i32 as libc::c_char;
            j += 1
        }
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            match *board.offset((10 as libc::c_int * i + j) as isize) {
                0 => {
                    buffer[(2 as libc::c_int * (j - 1 as libc::c_int)) as
                               usize] = '*' as i32 as libc::c_char
                }
                2 => {
                    buffer[(2 as libc::c_int * (j - 1 as libc::c_int)) as
                               usize] = 'O' as i32 as libc::c_char
                }
                _ => {
                    buffer[(2 as libc::c_int * (j - 1 as libc::c_int)) as
                               usize] = ' ' as i32 as libc::c_char
                }
            }
            j += 1
        }
        fprintf(stream,
                b"%s%d  %s      \x00" as *const u8 as *const libc::c_char,
                b"      \x00" as *const u8 as *const libc::c_char, i,
                buffer.as_mut_ptr());
        written = 0 as libc::c_int;
        if i == 1 as libc::c_int {
            written +=
                fprintf(stream,
                        b"%-9s\x00" as *const u8 as *const libc::c_char,
                        b"Black\x00" as *const u8 as *const libc::c_char);
            if !black_player.is_null() {
                written +=
                    fprintf(stream,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            black_player)
            }
        }
        if i == 2 as libc::c_int && give_time != 0 {
            written +=
                fprintf(stream,
                        b"         %02d:%02d\x00" as *const u8 as
                            *const libc::c_char,
                        black_time / 60 as libc::c_int,
                        black_time % 60 as libc::c_int)
        }
        if i == 3 as libc::c_int {
            if side_to_move == 0 as libc::c_int {
                written +=
                    fprintf(stream,
                            b" (*)  \x00" as *const u8 as *const libc::c_char)
            } else if give_evals != 0 && black_eval != 0.0f64 {
                if black_eval >= 0.0f64 && black_eval <= 1.0f64 {
                    written +=
                        fprintf(stream,
                                b"%-6.2f\x00" as *const u8 as
                                    *const libc::c_char, black_eval)
                } else {
                    written +=
                        fprintf(stream,
                                b"%+-6.2f\x00" as *const u8 as
                                    *const libc::c_char, black_eval)
                }
            } else {
                written +=
                    fprintf(stream,
                            b"      \x00" as *const u8 as *const libc::c_char)
            }
            written +=
                fprintf(stream,
                        b"   %d %s\x00" as *const u8 as *const libc::c_char,
                        disc_count(0 as libc::c_int),
                        b"discs\x00" as *const u8 as *const libc::c_char)
        }
        if i == 5 as libc::c_int {
            written +=
                fprintf(stream,
                        b"%-9s\x00" as *const u8 as *const libc::c_char,
                        b"White\x00" as *const u8 as *const libc::c_char);
            if !white_player.is_null() {
                written +=
                    fprintf(stream,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            white_player)
            }
        }
        if i == 6 as libc::c_int && give_time != 0 {
            written +=
                fprintf(stream,
                        b"         %02d:%02d\x00" as *const u8 as
                            *const libc::c_char,
                        white_time / 60 as libc::c_int,
                        white_time % 60 as libc::c_int)
        }
        if i == 7 as libc::c_int {
            if side_to_move == 2 as libc::c_int {
                written +=
                    fprintf(stream,
                            b" (O)  \x00" as *const u8 as *const libc::c_char)
            } else if give_evals != 0 && white_eval != 0.0f64 {
                if white_eval >= 0.0f64 && white_eval <= 1.0f64 {
                    written +=
                        fprintf(stream,
                                b"%-6.2f\x00" as *const u8 as
                                    *const libc::c_char, white_eval)
                } else {
                    written +=
                        fprintf(stream,
                                b"%+-6.2f\x00" as *const u8 as
                                    *const libc::c_char, white_eval)
                }
            } else {
                written +=
                    fprintf(stream,
                            b"      \x00" as *const u8 as *const libc::c_char)
            }
            written +=
                fprintf(stream,
                        b"   %d %s\x00" as *const u8 as *const libc::c_char,
                        disc_count(2 as libc::c_int),
                        b"discs\x00" as *const u8 as *const libc::c_char)
        }
        if give_game_score != 0 {
            fprintf(stream, b"%*s\x00" as *const u8 as *const libc::c_char,
                    22 as libc::c_int - written,
                    b"\x00" as *const u8 as *const libc::c_char);
            row = first_row + (i - 1 as libc::c_int);
            if row < current_row ||
                   row == current_row && side_to_move == 2 as libc::c_int {
                fprintf(stream,
                        b"%2d. \x00" as *const u8 as *const libc::c_char,
                        row + 1 as libc::c_int);
                if black_moves[row as usize] == -(1 as libc::c_int) {
                    fprintf(stream,
                            b"- \x00" as *const u8 as *const libc::c_char);
                } else {
                    fprintf(stream,
                            b"%c%c\x00" as *const u8 as *const libc::c_char,
                            'a' as i32 +
                                black_moves[row as usize] % 10 as libc::c_int
                                - 1 as libc::c_int,
                            '0' as i32 +
                                black_moves[row as usize] /
                                    10 as libc::c_int);
                }
                fprintf(stream,
                        b"  \x00" as *const u8 as *const libc::c_char);
                if row < current_row ||
                       row == current_row && side_to_move == 0 as libc::c_int
                   {
                    if white_moves[row as usize] == -(1 as libc::c_int) {
                        fprintf(stream,
                                b"- \x00" as *const u8 as
                                    *const libc::c_char);
                    } else {
                        fprintf(stream,
                                b"%c%c\x00" as *const u8 as
                                    *const libc::c_char,
                                'a' as i32 +
                                    white_moves[row as usize] %
                                        10 as libc::c_int - 1 as libc::c_int,
                                '0' as i32 +
                                    white_moves[row as usize] /
                                        10 as libc::c_int);
                    }
                }
            }
        }
        fputs(b"\n\x00" as *const u8 as *const libc::c_char, stream);
        i += 1
    }
    fputs(b"\n\x00" as *const u8 as *const libc::c_char, stream);
}
/*
  DISPLAY_MOVE
  Outputs a move or a pass to STREAM.
*/

pub unsafe fn display_move(mut stream: *mut FILE,
                                      mut move_0: libc::c_int) {
    if move_0 == -(1 as libc::c_int) {
        fprintf(stream, b"--\x00" as *const u8 as *const libc::c_char);
    } else {
        fprintf(stream, b"%c%c\x00" as *const u8 as *const libc::c_char,
                'a' as i32 + move_0 % 10 as libc::c_int - 1 as libc::c_int,
                '0' as i32 + move_0 / 10 as libc::c_int);
    };
}
/*
   DISPLAY_OPTIMAL_LINE
   Displays the principal variation found during the tree search.
*/

pub unsafe fn display_optimal_line(mut stream: *mut FILE) {
    let mut i: libc::c_int = 0;
    if full_pv_depth == 0 as libc::c_int { return }
    fprintf(stream, b"%s: \x00" as *const u8 as *const libc::c_char,
            b"PV\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < full_pv_depth {
        if i % 25 as libc::c_int != 0 as libc::c_int {
            fputc(' ' as i32, stream);
        } else if i > 0 as libc::c_int {
            fprintf(stream,
                    b"\n    \x00" as *const u8 as *const libc::c_char);
        }
        display_move(stream, full_pv[i as usize]);
        i += 1
    }
    fputs(b"\n\x00" as *const u8 as *const libc::c_char, stream);
}
/*
  SEND_STATUS
  Store information about the last completed search.
*/

pub unsafe extern "C" fn send_status(mut format: *const libc::c_char,
                                     mut args: ...) {
    let mut written: libc::c_int = 0;
    let mut arg_ptr: ::std::ffi::VaListImpl;
    arg_ptr = args.clone();
    written =
        vsprintf(status_buffer.as_mut_ptr().offset(status_pos as isize),
                 format, arg_ptr.as_va_list());
    status_pos += written;
    status_modified = 1 as libc::c_int;
}
/*
  SEND_STATUS_TIME
  Sends the amount of time elapsed to SEND_STATUS.
  The purpose of this function is to unify the format for
  the time string.
*/

pub unsafe fn send_status_time(mut elapsed_time: libc::c_double) {
    if elapsed_time < 10000.0f64 {
        send_status(b"%6.1f %c\x00" as *const u8 as *const libc::c_char,
                    elapsed_time, 's' as i32);
    } else {
        send_status(b"%6d %c\x00" as *const u8 as *const libc::c_char,
                    ceil(elapsed_time) as libc::c_int, 's' as i32);
    }
    send_status(b"  \x00" as *const u8 as *const libc::c_char);
}
/*
  SEND_STATUS_NODES
  Pipes the number of nodes searched to SEND_STATUS.
  The purpose of this function is to unify the format for
  the number of nodes.
*/

pub unsafe fn send_status_nodes(mut node_count: libc::c_double) {
    if node_count < 1.0e8f64 {
        send_status(b"%8.0f  \x00" as *const u8 as *const libc::c_char,
                    node_count);
    } else if node_count < 1.0e10f64 {
        send_status(b"%7.0f%c  \x00" as *const u8 as *const libc::c_char,
                    node_count / 1000.0f64, 'k' as i32);
    } else if node_count < 1.0e13f64 {
        send_status(b"%7.0f%c  \x00" as *const u8 as *const libc::c_char,
                    node_count / 1000000.0f64, 'M' as i32);
    } else {
        send_status(b"%7.0f%c  \x00" as *const u8 as *const libc::c_char,
                    node_count / 1000000000.0f64, 'G' as i32);
    };
}
/*
  SEND_STATUS_PV
  Pipes the principal variation to SEND_STATUS.
*/

pub unsafe fn send_status_pv(mut pv: *mut libc::c_int,
                                        mut max_depth: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <
              (if max_depth < 5 as libc::c_int {
                   max_depth
               } else { 5 as libc::c_int }) {
        if i < pv_depth[0 as libc::c_int as usize] {
            send_status(b"%c%c \x00" as *const u8 as *const libc::c_char,
                        'a' as i32 +
                            *pv.offset(i as isize) % 10 as libc::c_int -
                            1 as libc::c_int,
                        '0' as i32 +
                            *pv.offset(i as isize) / 10 as libc::c_int);
        } else {
            send_status(b"   \x00" as *const u8 as *const libc::c_char);
        }
        i += 1
    }
    send_status(b" \x00" as *const u8 as *const libc::c_char);
}
/*
  CLEAR_STATUS
  Clear the current status information.
*/

pub unsafe fn clear_status() {
    status_pos = 0 as libc::c_int;
    status_buffer[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_char;
    status_modified = 1 as libc::c_int;
}
/*
  DISPLAY_STATUS
  Output and clear the stored status information.
*/

pub unsafe fn display_status(mut stream: *mut FILE,
                                        mut allow_repeat: libc::c_int) {
    if (status_pos != 0 as libc::c_int || allow_repeat != 0) &&
           strlen(status_buffer.as_mut_ptr()) >
               0 as libc::c_int as libc::c_ulong {
        fprintf(stream, b"%s\n\x00" as *const u8 as *const libc::c_char,
                status_buffer.as_mut_ptr());
        strcpy(stored_status_buffer.as_mut_ptr(), status_buffer.as_mut_ptr());
    }
    status_pos = 0 as libc::c_int;
}

pub unsafe fn get_last_status() -> *const libc::c_char {
    return stored_status_buffer.as_mut_ptr();
}
/*
  SEND_SWEEP
  Store information about the current search.
*/

pub unsafe extern "C" fn send_sweep(mut format: *const libc::c_char,
                                    mut args: ...) {
    let mut written: libc::c_int = 0;
    let mut arg_ptr: ::std::ffi::VaListImpl;
    arg_ptr = args.clone();
    written =
        vsprintf(sweep_buffer.as_mut_ptr().offset(sweep_pos as isize), format,
                 arg_ptr.as_va_list());
    sweep_pos += written;
    sweep_modified = 1 as libc::c_int;
}
/*
  CLEAR_SWEEP
  Clear the search information.
*/

pub unsafe fn clear_sweep() {
    sweep_pos = 0 as libc::c_int;
    sweep_buffer[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_char;
    sweep_modified = 1 as libc::c_int;
}
/*
  DISPLAY_SWEEP
  Display and clear the current search information.
*/

pub unsafe fn display_sweep(mut stream: *mut FILE) {
    if sweep_pos != 0 as libc::c_int {
        fprintf(stream, b"%s\n\x00" as *const u8 as *const libc::c_char,
                sweep_buffer.as_mut_ptr());
    }
    sweep_modified = 0 as libc::c_int;
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
/*
  DISPLAY_BUFFERS
  If an update has happened and the last display was long enough ago,
  output relevant buffers.
*/

pub unsafe fn display_buffers() {
    let mut timer: libc::c_double = 0.;
    let mut new_interval: libc::c_double = 0.;
    timer = get_real_timer();
    if timer - last_output >= interval2 || timed_buffer_management == 0 {
        display_status(stdout, 0 as libc::c_int);
        status_modified = 0 as libc::c_int;
        if timer - last_output >= interval2 {
            if sweep_modified != 0 { display_sweep(stdout); }
            last_output = timer;
            /* Display the sweep at Fibonacci-spaced times */
            new_interval = interval1 + interval2;
            interval1 = interval2;
            interval2 = new_interval
        }
    };
}
/*
  TOGGLE_SMART_BUFFER_MANAGEMENT
  Allow the user between timed, "smart", buffer management
  and the simple "you asked for it, you got it"-approach which
  displays everything that is fed to the buffer.
*/

pub unsafe fn toggle_smart_buffer_management(mut use_smart:
                                                            libc::c_int) {
    timed_buffer_management = use_smart;
}
/*
  PRODUCE_EVAL_TEXT
  Convert a result descriptor into a string intended for output.
*/

pub unsafe fn produce_eval_text(mut eval_info: EvaluationType,
                                           mut short_output: libc::c_int)
 -> *mut libc::c_char {
    let mut buffer = 0 as *mut libc::c_char;
    let mut disk_diff: libc::c_double = 0.;
    let mut len: libc::c_int = 0;
    let mut int_confidence: libc::c_int = 0;
    buffer = safe_malloc(32 as libc::c_int as size_t) as *mut libc::c_char;
    len = 0 as libc::c_int;
    match eval_info.type_0 as libc::c_uint {
        0 => {
            if eval_info.score >= 29000 as libc::c_int {
                len =
                    sprintf(buffer,
                            b"Win\x00" as *const u8 as *const libc::c_char)
            } else if eval_info.score <= -(29000 as libc::c_int) {
                len =
                    sprintf(buffer,
                            b"Loss\x00" as *const u8 as *const libc::c_char)
            } else {
                disk_diff = eval_info.score as libc::c_double / 128.0f64;
                if short_output != 0 {
                    len =
                        sprintf(buffer,
                                b"%+.2f\x00" as *const u8 as
                                    *const libc::c_char, disk_diff)
                } else {
                    len =
                        sprintf(buffer,
                                b"%+.2f %s\x00" as *const u8 as
                                    *const libc::c_char, disk_diff,
                                b"discs\x00" as *const u8 as
                                    *const libc::c_char)
                }
            }
        }
        1 => {
            if short_output != 0 {
                len =
                    sprintf(buffer,
                            b"%+d\x00" as *const u8 as *const libc::c_char,
                            eval_info.score >> 7 as libc::c_int)
            } else if eval_info.score > 0 as libc::c_int {
                len =
                    sprintf(buffer,
                            b"%s %d-%d\x00" as *const u8 as
                                *const libc::c_char,
                            b"Win by\x00" as *const u8 as *const libc::c_char,
                            32 as libc::c_int +
                                (eval_info.score >> 8 as libc::c_int),
                            32 as libc::c_int -
                                (eval_info.score >> 8 as libc::c_int))
            } else if eval_info.score < 0 as libc::c_int {
                len =
                    sprintf(buffer,
                            b"%s %d-%d\x00" as *const u8 as
                                *const libc::c_char,
                            b"Loss by\x00" as *const u8 as
                                *const libc::c_char,
                            32 as libc::c_int -
                                (abs(eval_info.score) >> 8 as libc::c_int),
                            32 as libc::c_int +
                                (abs(eval_info.score) >> 8 as libc::c_int))
            } else {
                len =
                    sprintf(buffer,
                            b"Draw\x00" as *const u8 as *const libc::c_char)
            }
        }
        2 => {
            if short_output != 0 {
                match eval_info.res as libc::c_uint {
                    0 => {
                        len =
                            sprintf(buffer,
                                    b"Win\x00" as *const u8 as
                                        *const libc::c_char)
                    }
                    1 => {
                        len =
                            sprintf(buffer,
                                    b"Draw\x00" as *const u8 as
                                        *const libc::c_char)
                    }
                    2 => {
                        len =
                            sprintf(buffer,
                                    b"Loss\x00" as *const u8 as
                                        *const libc::c_char)
                    }
                    3 => {
                        len =
                            sprintf(buffer,
                                    b"???\x00" as *const u8 as
                                        *const libc::c_char)
                    }
                    _ => { }
                }
            } else {
                match eval_info.res as libc::c_uint {
                    0 => {
                        if eval_info.score !=
                               1 as libc::c_int * 128 as libc::c_int {
                            /* Lower bound on win */
                            len =
                                sprintf(buffer,
                                        b"%s %d-%d\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Win by at least\x00" as *const u8 as
                                            *const libc::c_char,
                                        32 as libc::c_int +
                                            (eval_info.score >>
                                                 8 as libc::c_int),
                                        32 as libc::c_int -
                                            (eval_info.score >>
                                                 8 as libc::c_int))
                        } else {
                            len =
                                sprintf(buffer,
                                        b"Win\x00" as *const u8 as
                                            *const libc::c_char)
                        }
                    }
                    1 => {
                        len =
                            sprintf(buffer,
                                    b"Draw\x00" as *const u8 as
                                        *const libc::c_char)
                    }
                    2 => {
                        if eval_info.score !=
                               -(1 as libc::c_int) * 128 as libc::c_int {
                            /* Upper bound on win */
                            len =
                                sprintf(buffer,
                                        b"%s %d-%d\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Loss by at least\x00" as *const u8
                                            as *const libc::c_char,
                                        32 as libc::c_int -
                                            (abs(eval_info.score) >>
                                                 8 as libc::c_int),
                                        32 as libc::c_int +
                                            (abs(eval_info.score) >>
                                                 8 as libc::c_int))
                        } else {
                            len =
                                sprintf(buffer,
                                        b"Loss\x00" as *const u8 as
                                            *const libc::c_char)
                        }
                    }
                    3 => {
                        len =
                            sprintf(buffer,
                                    b"???\x00" as *const u8 as
                                        *const libc::c_char)
                    }
                    _ => { }
                }
            }
        }
        3 => {
            int_confidence =
                floor(eval_info.confidence * 100.0f64) as libc::c_int;
            match eval_info.res as libc::c_uint {
                0 => {
                    if eval_info.score !=
                           1 as libc::c_int * 128 as libc::c_int {
                        len =
                            sprintf(buffer,
                                    b"%+d @ %d%%\x00" as *const u8 as
                                        *const libc::c_char,
                                    eval_info.score / 128 as libc::c_int,
                                    int_confidence)
                    } else {
                        len =
                            sprintf(buffer,
                                    b"%s @ %d%%\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Win\x00" as *const u8 as
                                        *const libc::c_char, int_confidence)
                    }
                }
                1 => {
                    len =
                        sprintf(buffer,
                                b"%s @ %d%%\x00" as *const u8 as
                                    *const libc::c_char,
                                b"Draw\x00" as *const u8 as
                                    *const libc::c_char, int_confidence)
                }
                2 => {
                    if eval_info.score !=
                           -(1 as libc::c_int) * 128 as libc::c_int {
                        len =
                            sprintf(buffer,
                                    b"%+d @ %d%%\x00" as *const u8 as
                                        *const libc::c_char,
                                    eval_info.score >> 7 as libc::c_int,
                                    int_confidence)
                    } else {
                        len =
                            sprintf(buffer,
                                    b"%s @ %d%%\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Loss\x00" as *const u8 as
                                        *const libc::c_char, int_confidence)
                    }
                }
                3 => {
                    if eval_info.score == 0 as libc::c_int {
                        len =
                            sprintf(buffer,
                                    b"Draw @ %d%%\x00" as *const u8 as
                                        *const libc::c_char, int_confidence)
                    } else {
                        len =
                            sprintf(buffer,
                                    b"%+d @ %d%%\x00" as *const u8 as
                                        *const libc::c_char,
                                    eval_info.score / 128 as libc::c_int,
                                    int_confidence)
                    }
                }
                _ => { }
            }
        }
        4 => {
            if short_output != 0 {
                len =
                    sprintf(buffer,
                            b"-\x00" as *const u8 as *const libc::c_char)
            } else {
                len =
                    sprintf(buffer,
                            b"forced\x00" as *const u8 as *const libc::c_char)
            }
        }
        5 => {
            if short_output != 0 {
                len =
                    sprintf(buffer,
                            b"-\x00" as *const u8 as *const libc::c_char)
            } else {
                len =
                    sprintf(buffer,
                            b"pass\x00" as *const u8 as *const libc::c_char)
            }
        }
        7 => {
            len =
                sprintf(buffer,
                        b"incompl\x00" as *const u8 as *const libc::c_char)
        }
        6 => {
            /* We really want to perform len = sprintf( buffer, "" ); */
            *buffer.offset(0 as libc::c_int as isize) =
                0 as libc::c_int as libc::c_char;
            len = 0 as libc::c_int
        }
        8 => {
            len =
                sprintf(buffer, b"--\x00" as *const u8 as *const libc::c_char)
        }
        _ => { }
    }
    if eval_info.is_book != 0 {
        len +=
            sprintf(buffer.offset(len as isize),
                    b" (%s)\x00" as *const u8 as *const libc::c_char,
                    b"book\x00" as *const u8 as *const libc::c_char)
    }
    return buffer;
}
