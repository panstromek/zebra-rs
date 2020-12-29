use libc_wrapper::{sprintf, fprintf, vsprintf, fputs, fputc, exit, strcpy, getc, stdin, FILE, size_t, strdup};
use crate::src::error::{FE};
use engine::src::error::FrontEnd;
use engine::src::stubs::{floor, abs, ceil};
use crate::src::safemem::safe_malloc;
use engine::src::zebra::EvaluationType;


use engine::src::search::disc_count;
use engine::src::timer::get_real_timer;
use std::ffi::c_void;

static mut stored_status_buffer: [i8; 256] = [0; 256];

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

pub unsafe fn set_names(black_name: *const i8, white_name: *const i8) {
    if !black_player.is_null() { FE::free(black_player as *mut c_void); }
    if !white_player.is_null() { FE::free(white_player as *mut c_void); }
    black_player = strdup(black_name);
    white_player = strdup(white_name);
}

pub unsafe fn set_times(black: i32, white: i32) {
    black_time = black;
    white_time = white;
}

pub unsafe fn set_evals(black: f64, white: f64) {
    black_eval = black;
    white_eval = white;
}

pub unsafe fn set_move_list(_black: *mut i32, _white: *mut i32, row: i32) {
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

/*
   File:           display.c

   Created:        July 10, 1997

   Modified:       November 23, 2001

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Some I/O routines.
*/
/* Global variables */

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
    let ch = getc(stdin) as i8;
    if ch as i32 == ' ' as i32 { exit(1 as i32); };
}
/*
   DISPLAY_BOARD
   side_to_move = the player whose turn it is
   black_moves = a list of black moves so far
   white_moves = a list of white moves so far
   current_row = the row of the score sheet

   The board is displayed using '*' for black and 'O' for white.
*/
pub unsafe fn display_board(stream: *mut FILE, board: &[i32; 128],
                                 side_to_move: i32, give_game_score: i32,
                                 give_time: i32, give_evals: i32, current_row_: i32,
                                 black_player_: *mut i8, black_time_: i32, black_eval_: f64,
                                 white_player_: *mut i8, white_time_: i32, white_eval_: f64,
                                 black_moves_: &[i32; 60], white_moves_: &[i32; 60]) {
    let mut buffer: [i8; 16] = [0; 16];
    let mut j;
    let mut written;
    let first_row;
    let mut row;
    if side_to_move == 0 {
        first_row = if 0 > current_row_ - 8 {
            0
        } else {
            current_row_ - 8
        }
    } else {
        first_row = if 0 > current_row_ - 7 {
            0
        } else {
            current_row_ - 7
        }
    }
    buffer[15] = 0;
    fputs(b"\n\x00" as *const u8 as *const i8, stream);
    fprintf(stream, b"%s   a b c d e f g h\n\x00" as *const u8 as *const i8,
            b"      \x00" as *const u8 as *const i8);
    fputs(b"\n\x00" as *const u8 as *const i8, stream);
    let mut i = 1;
    while i <= 8 {
        j = 0;
        while j < 15 {
            buffer[j as usize] = ' ' as i32 as i8;
            j += 1
        }
        j = 1;
        while j <= 8 {
            match board[(10 * i + j) as usize] {
                0 => {
                    buffer[(2 * (j - 1)) as usize] = '*' as i32 as i8
                }
                2 => {
                    buffer[(2 * (j - 1)) as usize] = 'O' as i32 as i8
                }
                _ => {
                    buffer[(2 * (j - 1)) as usize] = ' ' as i32 as i8
                }
            }
            j += 1
        }
        fprintf(stream, b"%s%d  %s      \x00" as *const u8 as *const i8,
                b"      \x00" as *const u8 as *const i8, i, buffer.as_mut_ptr());
        written = 0;
        if i == 1 {
            written += fprintf(stream,
                        b"%-9s\x00" as *const u8 as *const i8,
                        b"Black\x00" as *const u8 as *const i8);
            if !black_player_.is_null() {
                written += fprintf(stream, b"%s\x00" as *const u8 as *const i8, black_player_)
            }
        }
        if i == 2 && give_time != 0 {
            written += fprintf(stream, b"         %02d:%02d\x00" as *const u8 as *const i8,
                        black_time_ / 60, black_time_ % 60)
        }
        if i == 3 {
            if side_to_move == 0 {
                written += fprintf(stream, b" (*)  \x00" as *const u8 as *const i8)
            } else if give_evals != 0 && black_eval_ != 0.0f64 {
                if black_eval_ >= 0.0f64 && black_eval_ <= 1.0f64 {
                    written += fprintf(stream, b"%-6.2f\x00" as *const u8 as *const i8, black_eval_)
                } else {
                    written += fprintf(stream, b"%+-6.2f\x00" as *const u8 as *const i8, black_eval_)
                }
            } else {
                written += fprintf(stream, b"      \x00" as *const u8 as *const i8)
            }
            written += fprintf(stream, b"   %d %s\x00" as *const u8 as *const i8,
                        disc_count(0, &board),
                        b"discs\x00" as *const u8 as *const i8)
        }
        if i == 5 {
            written += fprintf(stream,
                        b"%-9s\x00" as *const u8 as *const i8,
                        b"White\x00" as *const u8 as *const i8);
            if !white_player_.is_null() {
                written += fprintf(stream, b"%s\x00" as *const u8 as *const i8, white_player_)
            }
        }
        if i == 6 && give_time != 0 {
            written += fprintf(stream, b"         %02d:%02d\x00" as *const u8 as *const i8,
                               white_time_ / 60, white_time_ % 60)
        }
        if i == 7 {
            if side_to_move == 2 {
                written += fprintf(stream, b" (O)  \x00" as *const u8 as *const i8)
            } else if give_evals != 0 && white_eval_ != 0.0f64 {
                if white_eval_ >= 0.0f64 && white_eval_ <= 1.0f64 {
                    written += fprintf(stream, b"%-6.2f\x00" as *const u8 as *const i8, white_eval_)
                } else {
                    written += fprintf(stream, b"%+-6.2f\x00" as *const u8 as *const i8, white_eval_)
                }
            } else {
                written += fprintf(stream, b"      \x00" as *const u8 as *const i8)
            }
            written += fprintf(stream, b"   %d %s\x00" as *const u8 as *const i8,
                        disc_count(2, &board),
                        b"discs\x00" as *const u8 as *const i8)
        }
        if give_game_score != 0 {
            fprintf(stream, b"%*s\x00" as *const u8 as *const i8,
                    22 - written, b"\x00" as *const u8 as *const i8);
            row = first_row + (i - 1);
            if row < current_row_ || row == current_row_ && side_to_move == 2 {
                fprintf(stream, b"%2d. \x00" as *const u8 as *const i8, row + 1);
                if black_moves_[row as usize] == -1 {
                    fprintf(stream, b"- \x00" as *const u8 as *const i8);
                } else {
                    fprintf(stream, b"%c%c\x00" as *const u8 as *const i8,
                            'a' as i32 + black_moves_[row as usize] % 10 - 1,
                            '0' as i32 + black_moves_[row as usize] / 10);
                }
                fprintf(stream, b"  \x00" as *const u8 as *const i8);
                if row < current_row_ || row == current_row_ && side_to_move == 0 {
                    if white_moves_[row as usize] == -1 {
                        fprintf(stream, b"- \x00" as *const u8 as *const i8);
                    } else {
                        fprintf(stream, b"%c%c\x00" as *const u8 as *const i8,
                                'a' as i32 + white_moves_[row as usize] % 10 - 1,
                                '0' as i32 + white_moves_[row as usize] / 10);
                    }
                }
            }
        }
        fputs(b"\n\x00" as *const u8 as *const i8, stream);
        i += 1
    }
    fputs(b"\n\x00" as *const u8 as *const i8, stream);
}
/*
  DISPLAY_MOVE
  Outputs a move or a pass to STREAM.
*/

pub unsafe fn display_move(stream: *mut FILE,
                                      move_0: i32) {
    if move_0 == -(1 as i32) {
        fprintf(stream, b"--\x00" as *const u8 as *const i8);
    } else {
        fprintf(stream, b"%c%c\x00" as *const u8 as *const i8,
                'a' as i32 + move_0 % 10 as i32 - 1 as i32,
                '0' as i32 + move_0 / 10 as i32);
    };
}
/*
   DISPLAY_OPTIMAL_LINE
   Displays the principal variation found during the tree search.
*/

pub unsafe fn display_optimal_line(stream: *mut FILE, full_pv_depth_: i32, full_pv_: [i32; 120]) {
    let mut i: i32 = 0;
    if full_pv_depth_ == 0 as i32 { return }
    fprintf(stream, b"%s: \x00" as *const u8 as *const i8,
            b"PV\x00" as *const u8 as *const i8);
    i = 0;
    while i < full_pv_depth_ {
        if i % 25 as i32 != 0 as i32 {
            fputc(' ' as i32, stream);
        } else if i > 0 as i32 {
            fprintf(stream,
                    b"\n    \x00" as *const u8 as *const i8);
        }
        display_move(stream, full_pv_[i as usize]);
        i += 1
    }
    fputs(b"\n\x00" as *const u8 as *const i8, stream);
}
/*
  SEND_STATUS
  Store information about the last completed search.
*/

pub unsafe extern "C" fn send_status(format: *const i8, args: ...) {
    let mut arg_ptr = args.clone();
    let written =
        vsprintf(status_buffer.as_mut_ptr().offset(status_pos as isize),
                 format, arg_ptr.as_va_list());
    status_pos += written;
    status_modified = 1;
}
/*
  SEND_STATUS_TIME
  Sends the amount of time elapsed to SEND_STATUS.
  The purpose of this function is to unify the format for
  the time string.
*/

pub unsafe fn send_status_time(elapsed_time: f64) {
    if elapsed_time < 10000.0f64 {
        send_status(b"%6.1f %c\x00" as *const u8 as *const i8,
                    elapsed_time, 's' as i32);
    } else {
        send_status(b"%6d %c\x00" as *const u8 as *const i8,
                    ceil(elapsed_time) as i32, 's' as i32);
    }
    send_status(b"  \x00" as *const u8 as *const i8);
}
/*
  SEND_STATUS_NODES
  Pipes the number of nodes searched to SEND_STATUS.
  The purpose of this function is to unify the format for
  the number of nodes.
*/

pub unsafe fn send_status_nodes(node_count: f64) {
    if node_count < 1.0e8f64 {
        send_status(b"%8.0f  \x00" as *const u8 as *const i8,
                    node_count);
    } else if node_count < 1.0e10f64 {
        send_status(b"%7.0f%c  \x00" as *const u8 as *const i8,
                    node_count / 1000.0f64, 'k' as i32);
    } else if node_count < 1.0e13f64 {
        send_status(b"%7.0f%c  \x00" as *const u8 as *const i8,
                    node_count / 1000000.0f64, 'M' as i32);
    } else {
        send_status(b"%7.0f%c  \x00" as *const u8 as *const i8,
                    node_count / 1000000000.0f64, 'G' as i32);
    };
}
/*
  SEND_STATUS_PV
  Pipes the principal variation to SEND_STATUS.
*/

pub unsafe fn send_status_pv(pv: &[i32; 64], max_depth: i32, pv_depth_zero: i32) {
    let mut i = 0;
    while i <
              (if max_depth < 5 as i32 {
                   max_depth
               } else { 5 as i32 }) {
        if i < pv_depth_zero {
            send_status(b"%c%c \x00" as *const u8 as *const i8,
                        'a' as i32 +
                            pv[i as usize] % 10 as i32 -
                            1 as i32,
                        '0' as i32 +
                            pv[i as usize] / 10 as i32);
        } else {
            send_status(b"   \x00" as *const u8 as *const i8);
        }
        i += 1
    }
    send_status(b" \x00" as *const u8 as *const i8);
}
/*
  DISPLAY_STATUS
  Output and clear the stored status information.
*/

pub unsafe fn display_status(stream: *mut FILE,
                                        allow_repeat: i32) {
    if (status_pos != 0 as i32 || allow_repeat != 0) &&
             FE::strlen(status_buffer.as_mut_ptr()) >
               0 as i32 as u64 {
        fprintf(stream, b"%s\n\x00" as *const u8 as *const i8,
                status_buffer.as_mut_ptr());
        strcpy(stored_status_buffer.as_mut_ptr(), status_buffer.as_mut_ptr());
    }
    status_pos = 0;
}
/*
  SEND_SWEEP
  Store information about the current search.
*/

pub unsafe extern "C" fn send_sweep(format: *const i8,
                                    args: ...) {
    let mut arg_ptr = args.clone();
    let written =
        vsprintf(sweep_buffer.as_mut_ptr().offset(sweep_pos as isize), format,
                 arg_ptr.as_va_list());
    sweep_pos += written;
    sweep_modified = 1;
}
/*
  DISPLAY_SWEEP
  Display and clear the current search information.
*/

pub unsafe fn display_sweep(stream: *mut FILE) {
    if sweep_pos != 0 as i32 {
        fprintf(stream, b"%s\n\x00" as *const u8 as *const i8,
                sweep_buffer.as_mut_ptr());
    }
    sweep_modified = 0;
}
/*
  PRODUCE_EVAL_TEXT
  Convert a result descriptor into a string intended for output.
*/

pub unsafe fn produce_eval_text(eval_info: &EvaluationType,
                                           short_output: i32)
 -> *mut i8 {
    let disk_diff: f64;
    let int_confidence: i32;
    let buffer = safe_malloc(32 as i32 as size_t) as *mut i8;
    let mut len = 0;
    match eval_info.type_0 as u32 {
        0 => {
            if eval_info.score >= 29000 as i32 {
                len =
                    sprintf(buffer,
                            b"Win\x00" as *const u8 as *const i8)
            } else if eval_info.score <= -(29000 as i32) {
                len =
                    sprintf(buffer,
                            b"Loss\x00" as *const u8 as *const i8)
            } else {
                disk_diff = eval_info.score as f64 / 128.0f64;
                if short_output != 0 {
                    len =
                        sprintf(buffer,
                                b"%+.2f\x00" as *const u8 as
                                    *const i8, disk_diff)
                } else {
                    len =
                        sprintf(buffer,
                                b"%+.2f %s\x00" as *const u8 as
                                    *const i8, disk_diff,
                                b"discs\x00" as *const u8 as
                                    *const i8)
                }
            }
        }
        1 => {
            if short_output != 0 {
                len =
                    sprintf(buffer,
                            b"%+d\x00" as *const u8 as *const i8,
                            eval_info.score >> 7 as i32)
            } else if eval_info.score > 0 as i32 {
                len =
                    sprintf(buffer,
                            b"%s %d-%d\x00" as *const u8 as
                                *const i8,
                            b"Win by\x00" as *const u8 as *const i8,
                            32 as i32 +
                                (eval_info.score >> 8 as i32),
                            32 as i32 -
                                (eval_info.score >> 8 as i32))
            } else if eval_info.score < 0 as i32 {
                len =
                    sprintf(buffer,
                            b"%s %d-%d\x00" as *const u8 as
                                *const i8,
                            b"Loss by\x00" as *const u8 as
                                *const i8,
                            32 as i32 -
                                (abs(eval_info.score) >> 8 as i32),
                            32 as i32 +
                                (abs(eval_info.score) >> 8 as i32))
            } else {
                len =
                    sprintf(buffer,
                            b"Draw\x00" as *const u8 as *const i8)
            }
        }
        2 => {
            if short_output != 0 {
                match eval_info.res as u32 {
                    0 => {
                        len =
                            sprintf(buffer,
                                    b"Win\x00" as *const u8 as
                                        *const i8)
                    }
                    1 => {
                        len =
                            sprintf(buffer,
                                    b"Draw\x00" as *const u8 as
                                        *const i8)
                    }
                    2 => {
                        len =
                            sprintf(buffer,
                                    b"Loss\x00" as *const u8 as
                                        *const i8)
                    }
                    3 => {
                        len =
                            sprintf(buffer,
                                    b"???\x00" as *const u8 as
                                        *const i8)
                    }
                    _ => { }
                }
            } else {
                match eval_info.res as u32 {
                    0 => {
                        if eval_info.score !=
                               1 as i32 * 128 as i32 {
                            /* Lower bound on win */
                            len =
                                sprintf(buffer,
                                        b"%s %d-%d\x00" as *const u8 as
                                            *const i8,
                                        b"Win by at least\x00" as *const u8 as
                                            *const i8,
                                        32 as i32 +
                                            (eval_info.score >>
                                                 8 as i32),
                                        32 as i32 -
                                            (eval_info.score >>
                                                 8 as i32))
                        } else {
                            len =
                                sprintf(buffer,
                                        b"Win\x00" as *const u8 as
                                            *const i8)
                        }
                    }
                    1 => {
                        len =
                            sprintf(buffer,
                                    b"Draw\x00" as *const u8 as
                                        *const i8)
                    }
                    2 => {
                        if eval_info.score !=
                               -(1 as i32) * 128 as i32 {
                            /* Upper bound on win */
                            len =
                                sprintf(buffer,
                                        b"%s %d-%d\x00" as *const u8 as
                                            *const i8,
                                        b"Loss by at least\x00" as *const u8
                                            as *const i8,
                                        32 as i32 -
                                            (abs(eval_info.score) >>
                                                 8 as i32),
                                        32 as i32 +
                                            (abs(eval_info.score) >>
                                                 8 as i32))
                        } else {
                            len =
                                sprintf(buffer,
                                        b"Loss\x00" as *const u8 as
                                            *const i8)
                        }
                    }
                    3 => {
                        len =
                            sprintf(buffer,
                                    b"???\x00" as *const u8 as
                                        *const i8)
                    }
                    _ => { }
                }
            }
        }
        3 => {
            int_confidence =
                floor(eval_info.confidence * 100.0f64) as i32;
            match eval_info.res as u32 {
                0 => {
                    if eval_info.score !=
                           1 as i32 * 128 as i32 {
                        len =
                            sprintf(buffer,
                                    b"%+d @ %d%%\x00" as *const u8 as
                                        *const i8,
                                    eval_info.score / 128 as i32,
                                    int_confidence)
                    } else {
                        len =
                            sprintf(buffer,
                                    b"%s @ %d%%\x00" as *const u8 as
                                        *const i8,
                                    b"Win\x00" as *const u8 as
                                        *const i8, int_confidence)
                    }
                }
                1 => {
                    len =
                        sprintf(buffer,
                                b"%s @ %d%%\x00" as *const u8 as
                                    *const i8,
                                b"Draw\x00" as *const u8 as
                                    *const i8, int_confidence)
                }
                2 => {
                    if eval_info.score !=
                           -(1 as i32) * 128 as i32 {
                        len =
                            sprintf(buffer,
                                    b"%+d @ %d%%\x00" as *const u8 as
                                        *const i8,
                                    eval_info.score >> 7 as i32,
                                    int_confidence)
                    } else {
                        len =
                            sprintf(buffer,
                                    b"%s @ %d%%\x00" as *const u8 as
                                        *const i8,
                                    b"Loss\x00" as *const u8 as
                                        *const i8, int_confidence)
                    }
                }
                3 => {
                    if eval_info.score == 0 as i32 {
                        len =
                            sprintf(buffer,
                                    b"Draw @ %d%%\x00" as *const u8 as
                                        *const i8, int_confidence)
                    } else {
                        len =
                            sprintf(buffer,
                                    b"%+d @ %d%%\x00" as *const u8 as
                                        *const i8,
                                    eval_info.score / 128 as i32,
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
                            b"-\x00" as *const u8 as *const i8)
            } else {
                len =
                    sprintf(buffer,
                            b"forced\x00" as *const u8 as *const i8)
            }
        }
        5 => {
            if short_output != 0 {
                len =
                    sprintf(buffer,
                            b"-\x00" as *const u8 as *const i8)
            } else {
                len =
                    sprintf(buffer,
                            b"pass\x00" as *const u8 as *const i8)
            }
        }
        7 => {
            len =
                sprintf(buffer,
                        b"incompl\x00" as *const u8 as *const i8)
        }
        6 => {
            /* We really want to perform len = sprintf( buffer, "" ); */
            *buffer = 0;
            len = 0 as i32
        }
        8 => {
            len =
                sprintf(buffer, b"--\x00" as *const u8 as *const i8)
        }
        _ => { }
    }
    if eval_info.is_book != 0 {
        len +=
            sprintf(buffer.offset(len as isize),
                    b" (%s)\x00" as *const u8 as *const i8,
                    b"book\x00" as *const u8 as *const i8)
    }
    buffer
}
