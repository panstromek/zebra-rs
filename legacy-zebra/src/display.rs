use std::ffi::{c_void, CStr};

use engine::src::search::disc_count;
use engine::src::stubs::{abs, ceil, floor};
use engine::src::zebra::EvaluationType;
use libc_wrapper::{exit, FileHandle, getc, sprintf, stdin};

use crate::src::error::FE;
use crate::src::zebra::FullState;
use engine::src::timer::Timer;
use std::io::Write;
use std::convert::TryInto;
use std::fmt::{Formatter};

pub struct DisplayState {
    pub black_player: &'static str,
    pub white_player: &'static str,
    pub status_buffer: Vec<u8>,
    pub sweep_buffer: Vec<u8>,
    pub black_eval: f64,
    pub white_eval: f64,
    pub last_output: f64,
    pub interval1: f64,
    pub interval2: f64,
    pub black_time: i32,
    pub white_time: i32,
    pub current_row: i32,
    pub status_modified: i32,
    pub sweep_modified: i32,
    pub timed_buffer_management: i32,
    pub status_pos: i32,
    pub sweep_pos: i32,
}

pub static mut display_state: DisplayState = DisplayState {
    black_player: "",
    white_player: "",
    status_buffer: Vec::new(), //TODO I want with capacity here - original also had just array. we should weight some perf implications of both approaches I guess
    sweep_buffer: Vec::new(), //TODO I want with capacity here - original also had just array. we should weight some perf implications of both approaches I guess
    black_eval: 0.0f64,
    white_eval: 0.0f64,
    last_output: 0.0f64,
    interval1: 0.,
    interval2: 0.,
    black_time: 0,
    white_time: 0,
    current_row: 0,
    status_modified: 0,
    sweep_modified: 0,
    timed_buffer_management: 1,
    status_pos: 0,
    sweep_pos: 0,
};

pub struct Square (u8, u8);
impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char(char::from(self.0))?;
        f.write_char(char::from(self.1))
    }
}

#[allow(non_snake_case)]
pub fn TO_SQUARE(move_: i32) -> Square {
    Square(
        b'a' + (move_ % 10) as u8 - 1,
        b'0' + (move_ / 10) as u8,
    )
}
/*
  SET_NAMES
  SET_TIMES
  SET_EVALS
  SET_MOVE_LIST
  Specify some information to be output along with the
  board by DISPLAY_BOARD.
*/

pub unsafe fn set_names(black_name: &'static str, white_name: &'static str) {
    display_state.black_player = black_name;
    display_state.white_player = white_name;
}

pub unsafe fn set_times(black: i32, white: i32) {
    display_state.black_time = black;
    display_state.white_time = white;
}

pub unsafe fn set_evals(black: f64, white: f64) {
    display_state.black_eval = black;
    display_state.white_eval = white;
}

pub unsafe fn set_move_list(row: i32) {
    display_state.current_row = row;
}

/*
  CLEAR_STATUS
  Clear the current status information.
*/

pub unsafe fn clear_status() {
    display_state.status_pos = 0;
    display_state.status_buffer.clear();
    display_state.status_modified = 1;
}

/*
  CLEAR_SWEEP
  Clear the search information.
*/

pub unsafe fn clear_sweep() {
    display_state.sweep_pos = 0;
    display_state.sweep_buffer.clear();
    display_state.sweep_modified = 1;
}

/*
  TOGGLE_SMART_BUFFER_MANAGEMENT
  Allow the user between timed, "smart", buffer management
  and the simple "you asked for it, you got it"-approach which
  displays everything that is fed to the buffer.
*/

pub unsafe fn toggle_smart_buffer_management(use_smart: i32) {
    display_state.timed_buffer_management = use_smart;
}
/*
  RESET_BUFFER_DISPLAY
  Clear all buffers and initialize time variables.
*/

pub unsafe fn reset_buffer_display(g_timer:&mut Timer) {
    /* The first two Fibonacci numbers */
    clear_status();
    clear_sweep();
    display_state.interval1 = 0.0f64;
    display_state.interval2 = 1.0f64;
    display_state.last_output =  g_timer.get_real_timer();
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
pub fn display_board(mut stream: &mut dyn std::io::Write, board: &[i32; 128],
                            side_to_move: i32, give_game_score: i32,
                            give_time: i32, give_evals: i32, current_row_: i32,
                            black_player_: &'static str, black_time_: i32, black_eval_: f64,
                            white_player_: &'static str, white_time_: i32, white_eval_: f64,
                            black_moves_: &[i32; 60], white_moves_: &[i32; 60]) {
    use std::fmt::Write;
    let mut buffer: [u8; 16] = [0; 16];
    let mut j;
    let mut written:usize;
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
    write!(stream, "\n");
    write!(stream, "{}   a b c d e f g h\n", "      ");
    write!(stream, "\n");
    let mut i = 1;
    while i <= 8 {
        j = 0;
        while j < 15 {
            buffer[j as usize] = ' ' as i32 as u8;
            j += 1
        }
        j = 1;
        while j <= 8 {
            match board[(10 * i + j) as usize] {
                0 => {
                    buffer[(2 * (j - 1)) as usize] = '*' as i32 as u8
                }
                2 => {
                    buffer[(2 * (j - 1)) as usize] = 'O' as i32 as u8
                }
                _ => {
                    buffer[(2 * (j - 1)) as usize] = ' ' as i32 as u8
                }
            }
            j += 1
        }
        write!(stream, "{}{}  {}      ",
                "      ", i, CStr::from_bytes_with_nul(&buffer).unwrap().to_str().unwrap());
        let mut buf = String::with_capacity(22);
        if i == 1 {
            write!(buf,
                        "{:<9}",
                        "Black");
            if !black_player_.is_empty() {
                write!(buf, "{}", black_player_ );
            }
        }
        if i == 2 && give_time != 0 {
            write!(buf, "         {:02}:{:02}",
                        black_time_ / 60, black_time_ % 60);
        }
        if i == 3 {
            if side_to_move == 0 {
                write!(buf, " (*)  ");
            } else if give_evals != 0 && black_eval_ != 0.0f64 {
                if black_eval_ >= 0.0f64 && black_eval_ <= 1.0f64 {
                    write!(buf, "{:<6.2}", black_eval_);
                } else {
                    write!(buf, "{:<+6.2}", black_eval_);
                }
            } else {
                write!(buf, "      ");
            }
            write!(buf, "   {} {}",
                        disc_count(0, &board),
                        "discs");
        }
        if i == 5 {
            write!(buf,
                        "{:<9}",
                        "White");
            if !white_player_.is_empty() {
                write!(buf, "{}", white_player_);
            }
        }
        if i == 6 && give_time != 0 {
            write!(buf, "         {:02}:{:02}",
                               white_time_ / 60, white_time_ % 60);
        }
        if i == 7 {
            if side_to_move == 2 {
                write!(buf, " (O)  ");
            } else if give_evals != 0 && white_eval_ != 0.0f64 {
                if white_eval_ >= 0.0f64 && white_eval_ <= 1.0f64 {
                    write!(buf, "{:<6.2}", white_eval_);
                } else {
                    write!(buf, "{:<+6.2}", white_eval_);
                }
            } else {
                write!(buf, "      ");
            }
            write!(buf, "   {} {}",
                        disc_count(2, &board),
                        "discs");
        }
        stream.write(&buf.as_bytes());
        if give_game_score != 0 {
            let written = buf.len();
            write!(stream, "{:width$}", "",
                    width = (22 - written));
            row = first_row + (i - 1);
            if row < current_row_ || row == current_row_ && side_to_move == 2 {
                write!(stream, "{:2}. ", row + 1);
                if black_moves_[row as usize] == -1 {
                    write!(stream, "- ");
                } else {
                    write!(stream, "{}{}",
                            char::from('a' as u8 + (black_moves_[row as usize] % 10) as u8  - 1),
                            char::from('0' as u8 + (black_moves_[row as usize] / 10) as u8));
                }
                write!(stream, "  ");
                if row < current_row_ || row == current_row_ && side_to_move == 0 {
                    if white_moves_[row as usize] == -1 {
                        write!(stream, "- ");
                    } else {
                        write!(stream, "{}{}",
                                char::from('a' as u8 + (white_moves_[row as usize] % 10) as u8  - 1) ,
                                char::from('0' as u8 + (white_moves_[row as usize] / 10) as u8 ));
                    }
                }
            }
        }
        write!(stream, "\n");
        i += 1
    }
    write!(stream, "\n");
}
/*
  DISPLAY_MOVE
  Outputs a move or a pass to STREAM.
*/

pub fn display_move(stream: &mut dyn Write,
                                      move_0: i32) {
    if move_0 == -(1 as i32) {
        write!(stream, "--");
    } else {
        write!(stream, "{}", TO_SQUARE(move_0));
    };
}
/*
   DISPLAY_OPTIMAL_LINE
   Displays the principal variation found during the tree search.
*/

pub fn display_optimal_line(stream: &mut dyn Write, full_pv_depth_: i32, full_pv_: &[i32; 120]) {
    let mut i: i32 = 0;
    if full_pv_depth_ == 0 as i32 { return }
    write!(stream, "PV: ");
    i = 0;
    while i < full_pv_depth_ {
        if i % 25 as i32 != 0 as i32 {
            write!(stream, " ");
        } else if i > 0 as i32 {
            write!(stream, "\n    ");
        }
        display_move(stream, full_pv_[i as usize]);
        i += 1
    }
    write!(stream, "\n");
}

pub unsafe trait CFormat: Copy {}
unsafe impl CFormat for i32 {}
unsafe impl CFormat for f64 {}
unsafe impl CFormat for *mut i8 {}
unsafe impl CFormat for *const i8 {}
unsafe impl CFormat for u64 {}

impl DisplayState {
    pub fn write_status_fmt(&mut self, args: std::fmt::Arguments<'_>) {
        let len_before = self.status_buffer.len();
        self.status_buffer.write_fmt(args);
        let len_after = self.status_buffer.len();
        let written = len_after - len_before;
        self.status_pos += written as i32;
        self.status_modified = 1;
    }
    pub fn write_sweep_fmt(&mut self, args: std::fmt::Arguments<'_>) {
        let len_before = self.sweep_buffer.len();
        self.sweep_buffer.write_fmt(args);
        let len_after = self.sweep_buffer.len();
        let written = len_after - len_before;
        self.sweep_pos += written as i32;
        self.sweep_modified = 1;
    }
}

/*
  SEND_STATUS
  Store information about the last completed search.
*/

#[macro_export]
macro_rules! send_status {
    ($display_state:ident, $($t:tt)*) => {
        $display_state.write_status_fmt(format_args!($($t)*));
    };
}


/*
  SEND_SWEEP
  Store information about the current search.
*/

#[macro_export]
macro_rules! send_sweep {
    ($display_state:ident, $($t:tt)*) => {
        $display_state.write_sweep_fmt(format_args!($($t)*));
    };
}

/*
  SEND_STATUS_TIME
  Sends the amount of time elapsed to SEND_STATUS.
  The purpose of this function is to unify the format for
  the time string.
*/

pub unsafe fn send_status_time(elapsed_time: f64) {
    if elapsed_time < 10000.0f64 {
        send_status!(display_state, "{:6.1} {}", elapsed_time, 's' );
    } else {
        send_status!(display_state, "{:6} {}", ceil(elapsed_time), 's');
    }
    send_status!(display_state, "  ");
}
/*
  SEND_STATUS_NODES
  Pipes the number of nodes searched to SEND_STATUS.
  The purpose of this function is to unify the format for
  the number of nodes.
*/

pub unsafe fn send_status_nodes(node_count: f64) {
    if node_count < 1.0e8f64 {
        send_status!(display_state, "{:8.0}  ", node_count);
    } else if node_count < 1.0e10f64 {
        send_status!(display_state, "{:7.0}{}  ", node_count / 1000.0f64, 'k');
    } else if node_count < 1.0e13f64 {
        send_status!(display_state, "{:7.0}{}  ", node_count / 1000000.0f64, 'M');
    } else {
        send_status!(display_state, "{:7.0}{}  ", node_count / 1000000000.0f64, 'G');
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
            send_status!(display_state, "{} ", TO_SQUARE(pv[i as usize]));
        } else {
            send_status!(display_state, "   ");
        }
        i += 1
    }
    send_status!(display_state, " ");
}
/*
  DISPLAY_STATUS
  Output and clear the stored status information.
*/

pub unsafe fn display_status(mut stream: FileHandle, allow_repeat: i32) {
    if !display_state.status_buffer.is_empty() || allow_repeat != 0 {
        write_buffer(&mut stream, display_state.status_buffer.as_mut())
    }
    display_state.status_pos = 0;
}

fn write_buffer(stream: &mut FileHandle, buf: &mut Vec<u8>) {
    if buf.len() > 0 {
        buf.push(b'\n');
        stream.write(buf);
        buf.pop();
    }
}
/*
  DISPLAY_SWEEP
  Display and clear the current search information.
*/

pub unsafe fn display_sweep(mut stream: FileHandle) {
    if !display_state.sweep_buffer.is_empty() {
        write_buffer(&mut stream,  display_state.sweep_buffer.as_mut());
    }
    display_state.sweep_modified = 0;
}
/*
  PRODUCE_EVAL_TEXT
  Convert a result descriptor into a string intended for output.
*/

pub fn produce_eval_text(eval_info: &EvaluationType,
                                           short_output: i32)
 -> String {

    use std::fmt::Write;
    // assert!(buffer.len < 19); // this is true in all tests now, so we allocate just 18 bytes.
    //  TODO allocate on stack for perf?
    let mut buffer =  String::with_capacity(18);
    let disk_diff: f64;
    let int_confidence: i32;
    match eval_info.type_0 as u32 {
        0 => if eval_info.score >= 29000 as i32 {
            write!(buffer, "Win");
        } else if eval_info.score <= -(29000 as i32) {
            write!(buffer, "Loss");
        } else {
            disk_diff = eval_info.score as f64 / 128.0f64;
            if short_output != 0 {
                write!(buffer, "{:+.2}", disk_diff);
            } else {
                write!(buffer, "{:+.2} {}", disk_diff, "discs");
            }
        },
        1 => if short_output != 0 {
            write!(buffer, "{:+}", eval_info.score >> 7);
        } else if eval_info.score > 0 {
            write!(buffer,
                        "{} {}-{}",
                        "Win by",
                        32 + (eval_info.score >> 8),
                        32 - (eval_info.score >> 8));
        } else if eval_info.score < 0 {
            write!(buffer, "{} {}-{}",
                        "Loss by",
                        32 - (abs(eval_info.score) >> 8),
                        32 + (abs(eval_info.score) >> 8));
        } else {
            write!(buffer, "Draw");
        },
        2 => if short_output != 0 {
            match eval_info.res as u32 {
                0 => { write!(buffer, "Win"); },
                1 => { write!(buffer, "Draw"); },
                2 => { write!(buffer, "Loss"); },
                3 => { write!(buffer, "???"); },
                _ => {}
            }
        } else {
            match eval_info.res as u32 {
                0 => if eval_info.score != 1 * 128 {
                    /* Lower bound on win */
                    write!(
                        buffer,
                        "{} {}-{}",
                        "Win by at least",
                        32 + (eval_info.score >> 8),
                        32 - (eval_info.score >> 8));
                } else {
                    write!(buffer, "Win");
                },
                1 => { write!(buffer, "Draw"); },
                2 => if eval_info.score != -128 {
                    /* Upper bound on win */
                    write!(buffer, "{} {}-{}",
                                  "Loss by at least",
                                  32 - (abs(eval_info.score) >> 8),
                                  32 + (abs(eval_info.score) >> 8));
                } else {
                    write!(buffer, "Loss");
                },
                3 => { write!(buffer, "???"); },
                _ => {}
            }
        },
        3 => {
            int_confidence = floor(eval_info.confidence * 100.0f64) as i32;
            match eval_info.res as u32 {
                0 => if eval_info.score != 128 {
                    write!(buffer, "{:+} @ {}%", eval_info.score / 128, int_confidence);
                } else {
                    write!(buffer, "{} @ {}%", "Win", int_confidence);
                },
                1 => {
                    write!(buffer, "{} @ {}%", "Draw", int_confidence);
                },
                2 => if eval_info.score != -128 {
                    write!(buffer, "{:+} @ {}%", eval_info.score >> 7 as i32, int_confidence);
                } else {
                    write!(buffer,
                                  "{} @ {}%",
                                  "Loss",
                                  int_confidence);
                },
                3 => if eval_info.score == 0 as i32 {
                    write!(buffer, "Draw @ {}%", int_confidence);
                } else {
                    write!(buffer, "{:+} @ {}%", eval_info.score / 128, int_confidence);
                },
                _ => { }
            }
        }
        4 => if short_output != 0 {
            write!(buffer, "-");
        } else {
            write!(buffer, "forced");
        },
        5 => if short_output != 0 {
            write!(buffer, "-");
        } else {
            write!(buffer, "pass");
        },
        7 => { write!(buffer, "incompl"); },
        6 => {
            buffer.clear();
        }
        8 => { write!(buffer, "--"); },
        _ => {}
    }
    if eval_info.is_book != 0 {
        write!(buffer, " ({})", "book");
    }
    return buffer
}
