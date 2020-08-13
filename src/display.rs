use crate::src::stubs::{sprintf, floor, fprintf, vsprintf, strlen, ceil, fputs, fputc, exit, abs, strcpy, getc, stdout, stdin};
use crate::src::safemem::safe_malloc;
use crate::src::timer::get_real_timer;
use crate::src::search::{full_pv, full_pv_depth, disc_count};
use crate::src::globals::{white_moves, black_moves, pv_depth};
use crate::src::zebra::{EvaluationType, _IO_FILE};
pub use engine::src::display::*;

pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut std::ffi::c_void,
    pub reg_save_area: *mut std::ffi::c_void,
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type va_list = __builtin_va_list;
pub type size_t = u64;
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;

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
    let mut ch: i8 = 0;
    ch = getc(stdin) as i8;
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

pub unsafe fn display_board(mut stream: *mut FILE,
                                       mut board: *mut i32,
                                       mut side_to_move: i32,
                                       mut give_game_score: i32,
                                       mut give_time: i32,
                                       mut give_evals: i32) {
    let mut buffer: [i8; 16] = [0; 16];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut written: i32 = 0;
    let mut first_row: i32 = 0;
    let mut row: i32 = 0;
    if side_to_move == 0 as i32 {
        first_row =
            if 0 as i32 > current_row - 8 as i32 {
                0 as i32
            } else { (current_row) - 8 as i32 }
    } else {
        first_row =
            if 0 as i32 > current_row - 7 as i32 {
                0 as i32
            } else { (current_row) - 7 as i32 }
    }
    buffer[15 as i32 as usize] = 0 as i32 as i8;
    fputs(b"\n\x00" as *const u8 as *const i8, stream);
    fprintf(stream,
            b"%s   a b c d e f g h\n\x00" as *const u8 as *const i8,
            b"      \x00" as *const u8 as *const i8);
    fputs(b"\n\x00" as *const u8 as *const i8, stream);
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 0 as i32;
        while j < 15 as i32 {
            buffer[j as usize] = ' ' as i32 as i8;
            j += 1
        }
        j = 1 as i32;
        while j <= 8 as i32 {
            match *board.offset((10 as i32 * i + j) as isize) {
                0 => {
                    buffer[(2 as i32 * (j - 1 as i32)) as
                               usize] = '*' as i32 as i8
                }
                2 => {
                    buffer[(2 as i32 * (j - 1 as i32)) as
                               usize] = 'O' as i32 as i8
                }
                _ => {
                    buffer[(2 as i32 * (j - 1 as i32)) as
                               usize] = ' ' as i32 as i8
                }
            }
            j += 1
        }
        fprintf(stream,
                b"%s%d  %s      \x00" as *const u8 as *const i8,
                b"      \x00" as *const u8 as *const i8, i,
                buffer.as_mut_ptr());
        written = 0 as i32;
        if i == 1 as i32 {
            written +=
                fprintf(stream,
                        b"%-9s\x00" as *const u8 as *const i8,
                        b"Black\x00" as *const u8 as *const i8);
            if !black_player.is_null() {
                written +=
                    fprintf(stream,
                            b"%s\x00" as *const u8 as *const i8,
                            black_player)
            }
        }
        if i == 2 as i32 && give_time != 0 {
            written +=
                fprintf(stream,
                        b"         %02d:%02d\x00" as *const u8 as
                            *const i8,
                        black_time / 60 as i32,
                        black_time % 60 as i32)
        }
        if i == 3 as i32 {
            if side_to_move == 0 as i32 {
                written +=
                    fprintf(stream,
                            b" (*)  \x00" as *const u8 as *const i8)
            } else if give_evals != 0 && black_eval != 0.0f64 {
                if black_eval >= 0.0f64 && black_eval <= 1.0f64 {
                    written +=
                        fprintf(stream,
                                b"%-6.2f\x00" as *const u8 as
                                    *const i8, black_eval)
                } else {
                    written +=
                        fprintf(stream,
                                b"%+-6.2f\x00" as *const u8 as
                                    *const i8, black_eval)
                }
            } else {
                written +=
                    fprintf(stream,
                            b"      \x00" as *const u8 as *const i8)
            }
            written +=
                fprintf(stream,
                        b"   %d %s\x00" as *const u8 as *const i8,
                        disc_count(0 as i32),
                        b"discs\x00" as *const u8 as *const i8)
        }
        if i == 5 as i32 {
            written +=
                fprintf(stream,
                        b"%-9s\x00" as *const u8 as *const i8,
                        b"White\x00" as *const u8 as *const i8);
            if !white_player.is_null() {
                written +=
                    fprintf(stream,
                            b"%s\x00" as *const u8 as *const i8,
                            white_player)
            }
        }
        if i == 6 as i32 && give_time != 0 {
            written +=
                fprintf(stream,
                        b"         %02d:%02d\x00" as *const u8 as
                            *const i8,
                        white_time / 60 as i32,
                        white_time % 60 as i32)
        }
        if i == 7 as i32 {
            if side_to_move == 2 as i32 {
                written +=
                    fprintf(stream,
                            b" (O)  \x00" as *const u8 as *const i8)
            } else if give_evals != 0 && white_eval != 0.0f64 {
                if white_eval >= 0.0f64 && white_eval <= 1.0f64 {
                    written +=
                        fprintf(stream,
                                b"%-6.2f\x00" as *const u8 as
                                    *const i8, white_eval)
                } else {
                    written +=
                        fprintf(stream,
                                b"%+-6.2f\x00" as *const u8 as
                                    *const i8, white_eval)
                }
            } else {
                written +=
                    fprintf(stream,
                            b"      \x00" as *const u8 as *const i8)
            }
            written +=
                fprintf(stream,
                        b"   %d %s\x00" as *const u8 as *const i8,
                        disc_count(2 as i32),
                        b"discs\x00" as *const u8 as *const i8)
        }
        if give_game_score != 0 {
            fprintf(stream, b"%*s\x00" as *const u8 as *const i8,
                    22 as i32 - written,
                    b"\x00" as *const u8 as *const i8);
            row = first_row + (i - 1 as i32);
            if row < current_row ||
                   row == current_row && side_to_move == 2 as i32 {
                fprintf(stream,
                        b"%2d. \x00" as *const u8 as *const i8,
                        row + 1 as i32);
                if black_moves[row as usize] == -(1 as i32) {
                    fprintf(stream,
                            b"- \x00" as *const u8 as *const i8);
                } else {
                    fprintf(stream,
                            b"%c%c\x00" as *const u8 as *const i8,
                            'a' as i32 +
                                black_moves[row as usize] % 10 as i32
                                - 1 as i32,
                            '0' as i32 +
                                black_moves[row as usize] /
                                    10 as i32);
                }
                fprintf(stream,
                        b"  \x00" as *const u8 as *const i8);
                if row < current_row ||
                       row == current_row && side_to_move == 0 as i32
                   {
                    if white_moves[row as usize] == -(1 as i32) {
                        fprintf(stream,
                                b"- \x00" as *const u8 as
                                    *const i8);
                    } else {
                        fprintf(stream,
                                b"%c%c\x00" as *const u8 as
                                    *const i8,
                                'a' as i32 +
                                    white_moves[row as usize] %
                                        10 as i32 - 1 as i32,
                                '0' as i32 +
                                    white_moves[row as usize] /
                                        10 as i32);
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

pub unsafe fn display_move(mut stream: *mut FILE,
                                      mut move_0: i32) {
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

pub unsafe fn display_optimal_line(mut stream: *mut FILE) {
    let mut i: i32 = 0;
    if full_pv_depth == 0 as i32 { return }
    fprintf(stream, b"%s: \x00" as *const u8 as *const i8,
            b"PV\x00" as *const u8 as *const i8);
    i = 0 as i32;
    while i < full_pv_depth {
        if i % 25 as i32 != 0 as i32 {
            fputc(' ' as i32, stream);
        } else if i > 0 as i32 {
            fprintf(stream,
                    b"\n    \x00" as *const u8 as *const i8);
        }
        display_move(stream, full_pv[i as usize]);
        i += 1
    }
    fputs(b"\n\x00" as *const u8 as *const i8, stream);
}
/*
  SEND_STATUS
  Store information about the last completed search.
*/

pub unsafe extern "C" fn send_status(mut format: *const i8,
                                     mut args: ...) {
    let mut written: i32 = 0;
    let mut arg_ptr: ::std::ffi::VaListImpl;
    arg_ptr = args.clone();
    written =
        vsprintf(status_buffer.as_mut_ptr().offset(status_pos as isize),
                 format, arg_ptr.as_va_list());
    status_pos += written;
    status_modified = 1 as i32;
}
/*
  SEND_STATUS_TIME
  Sends the amount of time elapsed to SEND_STATUS.
  The purpose of this function is to unify the format for
  the time string.
*/

pub unsafe fn send_status_time(mut elapsed_time: f64) {
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

pub unsafe fn send_status_nodes(mut node_count: f64) {
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

pub unsafe fn send_status_pv(mut pv: *mut i32,
                                        mut max_depth: i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i <
              (if max_depth < 5 as i32 {
                   max_depth
               } else { 5 as i32 }) {
        if i < pv_depth[0 as i32 as usize] {
            send_status(b"%c%c \x00" as *const u8 as *const i8,
                        'a' as i32 +
                            *pv.offset(i as isize) % 10 as i32 -
                            1 as i32,
                        '0' as i32 +
                            *pv.offset(i as isize) / 10 as i32);
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

pub unsafe fn display_status(mut stream: *mut FILE,
                                        mut allow_repeat: i32) {
    if (status_pos != 0 as i32 || allow_repeat != 0) &&
           strlen(status_buffer.as_mut_ptr()) >
               0 as i32 as u64 {
        fprintf(stream, b"%s\n\x00" as *const u8 as *const i8,
                status_buffer.as_mut_ptr());
        strcpy(stored_status_buffer.as_mut_ptr(), status_buffer.as_mut_ptr());
    }
    status_pos = 0 as i32;
}
/*
  SEND_SWEEP
  Store information about the current search.
*/

pub unsafe extern "C" fn send_sweep(mut format: *const i8,
                                    mut args: ...) {
    let mut written: i32 = 0;
    let mut arg_ptr: ::std::ffi::VaListImpl;
    arg_ptr = args.clone();
    written =
        vsprintf(sweep_buffer.as_mut_ptr().offset(sweep_pos as isize), format,
                 arg_ptr.as_va_list());
    sweep_pos += written;
    sweep_modified = 1 as i32;
}
/*
  DISPLAY_SWEEP
  Display and clear the current search information.
*/

pub unsafe fn display_sweep(mut stream: *mut FILE) {
    if sweep_pos != 0 as i32 {
        fprintf(stream, b"%s\n\x00" as *const u8 as *const i8,
                sweep_buffer.as_mut_ptr());
    }
    sweep_modified = 0 as i32;
}
/*
  DISPLAY_BUFFERS
  If an update has happened and the last display was long enough ago,
  output relevant buffers.
*/
#[no_mangle]
pub unsafe extern "C" fn display_buffers() {
    let mut timer: f64 = 0.;
    let mut new_interval: f64 = 0.;
    timer = get_real_timer();
    if timer - last_output >= interval2 || timed_buffer_management == 0 {
        display_status(stdout, 0 as i32);
        status_modified = 0 as i32;
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
  PRODUCE_EVAL_TEXT
  Convert a result descriptor into a string intended for output.
*/

pub unsafe fn produce_eval_text(mut eval_info: &EvaluationType,
                                           mut short_output: i32)
 -> *mut i8 {
    let mut buffer = 0 as *mut i8;
    let mut disk_diff: f64 = 0.;
    let mut len: i32 = 0;
    let mut int_confidence: i32 = 0;
    buffer = safe_malloc(32 as i32 as size_t) as *mut i8;
    len = 0 as i32;
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
            *buffer.offset(0 as i32 as isize) =
                0 as i32 as i8;
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
    return buffer;
}
