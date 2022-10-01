use engine::{
    src:: {
        counter::counter_value,
    }
};
use engine::src::counter::CounterType;
use engine::src::error::{FatalError, FrontEnd};
use engine::src::game::CandidateMove;
use engine::src::hash::{HashEntry, HashState};
use engine::src::search::{hash_expand_pv, SearchState};

use engine::src::zebra::{EvaluationType};
use libc_wrapper::{stderr, stdout, time, time_t, c_time};
use thordb_types::C2RustUnnamed;
#[macro_use]
use crate::send_status;
#[macro_use]
use crate::send_sweep;

use crate::{
    src::{
        display::{produce_eval_text},
    }
};
use crate::src::display::{display_state, TO_SQUARE};
use crate::src::osfbook::print_move_alternatives;

use engine::src::timer::Timer;
use engine::src::osfbook::Book;
use engine::src::globals::BoardState;
use engine::src::moves::MovesState;
use flip::unflip::FlipStack;

static mut buffer: String = String::new();

/*
   File:       error.h

   Created:    June 13, 1998

   Modified:   August 1, 2002

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The interface to the error handler.
*/
/*
   File:       error.c

   Created:    June 13, 1998

   Modified:   November 12, 2001

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The text-based error handler.
*/

#[macro_export]
macro_rules! fatal_error {
    ($($t:tt)*) => {
        $crate::src::error::fatal_error_(format_args!($($t)*))
    };
}

use std::io::Write;
pub fn fatal_error_(args: std::fmt::Arguments<'_>) -> ! {
    let mut timer: time_t = 0;
    eprint!("\nFatal error: ");
    unsafe {
        stderr.write_fmt(args);
        if let Ok(mut stream) = std::fs::OpenOptions::new().append(true).write(true).create(true).open("zebra.err") {
            time(&mut timer);
            write!(stream, "{} @ {}\n  ", "Fatal error", c_time(timer));
            stream.write_fmt(args);
        }
    }
    std::process::exit(1);
}

pub struct LibcFatalError; // FIXME rename this, it's not only error anymore
pub type FE = LibcFatalError;

impl LibcFatalError {
    pub fn choose_thor_opening_move_report(freq_sum: i32, match_count: i32, move_list: &[C2RustUnnamed; 64]) {
        write!(stdout, "{}:        ", "Thor database");
        let mut i = 0;
        while i < match_count {
            write!(stdout, "{}: {:4.1}%    ", TO_SQUARE(move_list[i as usize].move_0),
                   100.0f64 * move_list[i as usize].frequency as f64 / freq_sum as f64);
            if i % 6 == 4 {
                write!(stdout, "\n");
            }
            i += 1
        }
        if match_count % 6 != 5 {
            write!(stdout, "\n");
        }
    }

    pub fn memory_allocation_failure(block_count_: i32) -> ! {
        fatal_error!("{} @ #{}\n", "Memory allocation failure", block_count_);
    }

    pub fn error_in_map(i: i32, pos: i32, symmetry_map_item: i32) -> ! {
        fatal_error!("Error in map {}: inv(map({}))={}\n", i, pos, symmetry_map_item);
    }

    pub fn error_in_map_thor(i: i32, pos: i32, to_report: i32) -> ! {
        fatal_error!("Error in map {}: inv(map({}))={}\n", i, pos, to_report);
    }
}
impl FrontEnd for LibcFatalError {
    fn reset_buffer_display(g_timer:&Timer) {
        unsafe { display_state.reset_buffer_display(g_timer) }
    }
    /*
      DISPLAY_BUFFERS
      If an update has happened and the last display was long enough ago,
      output relevant buffers.
    */
    fn display_buffers(g_timer: &Timer) {
        unsafe {
            let timer =  g_timer.get_real_timer();
            let ds = &mut display_state;
            if timer - ds.last_output >= ds.interval2 || ds.timed_buffer_management == 0 {
                ds.display_status(&mut stdout, 0);
                ds.status_modified = 0;
                if timer - ds.last_output >= ds.interval2 {
                    if ds.sweep_modified != 0 { ds.display_sweep(&mut stdout); }
                    ds.last_output = timer;
                    /* Display the sweep at Fibonacci-spaced times */
                    let new_interval = ds.interval1 + ds.interval2;
                    ds.interval1 = ds.interval2;
                    ds.interval2 = new_interval
                }
            };
        }
    }

    fn after_update_best_list_verbose(best_list: &[i8; 4]) {
        write!(stdout, "      After:  ");
        let mut i = 0;
        while i < 4 {
            write!(stdout, "{:2} ", best_list[i] as i32);
            i += 1
        }
        write!(stdout, "\n");
    }

    fn before_update_best_list_verbose(best_list: &[i8; 4], move_0: i8, best_list_index: i32, best_list_length: i32) {
        write!(stdout, "move={:2}  index={}  length={}      ", move_0 as i32, best_list_index, best_list_length);
        write!(stdout, "Before:  ");
        let mut i = 0;
        while i < 4 {
            write!(stdout, "{:2} ", best_list[i] as i32);
            i += 1
        }
    }

    fn end_tree_search_output_some_second_stats(alpha: i32, beta: i32, curr_val: i32, update_pv: i32, move_index: i32, echo: i32) {
        unsafe {
            if update_pv != 0 {
                Self::end_tree_search_some_pv_stats_report(alpha, beta, curr_val)
            }
            send_sweep!(display_state, " ");
            if update_pv != 0 && move_index > 0 && echo != 0 {
                display_state.display_sweep(&mut stdout);
            }
        }
    }

    fn end_tree_search_some_pv_stats_report(alpha: i32, beta: i32, curr_val: i32) {
        unsafe {
            let ds = &mut display_state;
            if curr_val <= alpha {
                send_sweep!(ds, "<{}", curr_val + 1);
            } else if curr_val >= beta {
                send_sweep!(ds, ">{}", curr_val - 1);
            } else {
                send_sweep!(ds, "={}", curr_val);
            }
        }
    }

    fn end_tree_search_level_0_ponder_0_short_report(move_0: i8, first: i32) {
        unsafe {
            let ds = &mut display_state;
            if first != 0 {
                send_sweep!(ds, "{:<10} ", buffer);
            }
            send_sweep!(ds, "{}", TO_SQUARE(move_0));
        }
    }

    fn end_tree_search_output_some_stats(entry: &HashEntry) {
        /* Output some stats */
        unsafe {
            let ds = &mut display_state;
            send_sweep!(ds, "{}", TO_SQUARE(entry.move_0[0]));
            if entry.flags as i32 & 16 != 0 &&
                entry.flags as i32 & 4 != 0 {
                send_sweep!(ds, "={}", entry.eval);
            } else if entry.flags as i32 & 16 != 0
                &&
                entry.flags as i32 & 1 !=
                    0 {
                send_sweep!(ds, ">{}", entry.eval - 1);
            } else {
                send_sweep!(ds, "<{}", entry.eval + 1);
            }
            stdout.flush();
        }
    }

     fn end_tree_search_level_0_ponder_0_report(alpha: i32, beta: i32, result: i32, best_move_: i32) {
         unsafe {
             let ds = &mut display_state;
             send_sweep!(ds, "{:<10} ", buffer);
             send_sweep!(ds, "{}", TO_SQUARE(best_move_));
             if result <= alpha {
                 send_sweep!(ds, "<{}", result + 1 );
             } else if result >= beta {
                 send_sweep!(ds, ">{}", result - 1);
             } else {
                 send_sweep!(ds, "={}", result);
             }
         }
    }

    fn end_tree_search_level_0_report(alpha: i32, beta: i32) {
        unsafe {
            use std::fmt::Write;
            buffer.clear();
            write!(buffer, "[{},{}]:", alpha, beta);
            display_state.clear_sweep();
        }
    }
    /*
      SEND_SOLVE_STATUS
      Displays endgame results - partial or full.
    */
    fn send_solve_status(empties: i32, _side_to_move: i32, eval_info: &EvaluationType,
                          pv_zero: &mut [i8; 64],
                         pv_depth_zero: i32,
                         mut g_timer: &Timer,
                         mut search_state: &mut SearchState) {
        unsafe {
            let eval = *eval_info;
            search_state.set_current_eval(eval);
            let ds = &mut display_state;
            ds.clear_status();
            send_status!(ds, "-->  {:2}  ", empties);
            let mut eval_str = produce_eval_text(&*eval_info, 1);
            send_status!(ds, "{:<10}  ", eval_str);
            let nodes_counter: &mut CounterType = &mut search_state.nodes;
            let node_val = counter_value(nodes_counter);
            ds.send_status_nodes(node_val);
            if search_state.get_ponder_move() != 0 {
                send_status!(ds, "{{{}}} ",TO_SQUARE(search_state.get_ponder_move()));
            }
            ds.send_status_pv(pv_zero, empties, pv_depth_zero);
            ds.send_status_time( g_timer.get_elapsed_time());
            if  g_timer.get_elapsed_time() > 0.0001f64 {
                send_status!(ds, "{:6.0} {}  ",
                            node_val / ( g_timer.get_elapsed_time() + 0.0001f64),
                            "nps");
            };
        }
    }

    fn end_report_panic_abort_2(elapsed_time: f64) {
        write!(stdout, "{} {:.1} {} {}\n", "Panic abort after", elapsed_time, 's', "in selective search");
    }

    fn end_report_semi_panic_abort_3(elapsed_time: f64) {
        write!(stdout, "{} {:.1} {} {}\n", "Semi-panic abort after", elapsed_time, 's', "in WLD search");
    }

    fn end_report_semi_panic_abort_2(elapsed_time: f64) {
        write!(stdout, "{} {:.1} {} {}\n", "Semi-panic abort after", elapsed_time, 's', "in exact search");
    }

    fn end_report_panic_abort(elapsed_time: f64) {
        write!(stdout, "{} {:.1} {} {}\n", "Panic abort after", elapsed_time, 's', "in WLD search");
    }

    fn end_report_semi_panic_abort(elapsed_time: f64) {
        write!(stdout, "{} {:.1} {} {}\n", "Semi-panic abort after", elapsed_time, 's', "in selective search");
    }

    fn end_display_zero_status() {
        unsafe {
            display_state.display_status(&mut stdout, 0);
        }
    }

    fn handle_fatal_pv_error(i: i32, pv_0_depth: i32, pv_0: &[i8; 64]) {
        write!(stdout, "pv_depth[0] = {}\n", pv_0_depth);
        let mut j = 0;
        while j < pv_0_depth {
            write!(stdout, "{} ", TO_SQUARE(pv_0[j as usize]));
            j += 1
        }
        write!(stdout, "\n");
        write!(stdout, "i={}\n", i);
        fatal_error!("Error in PV completion");
    }

    fn report_unwanted_book_draw(this_move: i32) {
        write!(stdout, "{} leads to an unwanted book draw\n", TO_SQUARE(this_move));
    }

    fn report_in_get_book_move_1(side_to_move: i32, remaining_slack: i32, board_state: &mut BoardState, g_book: &mut Book) {
        write!(stdout, "Slack left is {:.2}. ", remaining_slack as f64 / 128.0f64);
        print_move_alternatives(side_to_move, board_state, g_book);
    }

    fn report_in_get_book_move_2(chosen_score: i32, chosen_index: i32, flags: &i32, candidate_list_: &[CandidateMove; 60], search_state: & SearchState) {
        unsafe {
            let ds = &mut display_state;
            send_status!(ds, "-->   Book     ");
            if flags & 16 != 0 {
                send_status!(ds, "{:+3} (exact)   ",
                            chosen_score / 128);
            } else if flags & 4 != 0 {
                send_status!(ds, "{:+3} (WLD)     ",
                            chosen_score / 128);
            } else {
                send_status!(ds, "{:+6.2}        ",
                            chosen_score as f64 / 128.0f64);
            }
            if search_state.get_ponder_move() != 0 {
                send_status!(ds, "{{{}}} ",TO_SQUARE(search_state.get_ponder_move()));
            }
            send_status!(ds, "{}", TO_SQUARE(candidate_list_[chosen_index as usize].move_0));
        }
    }
    fn midgame_display_simple_ponder_move(move_0: i8) {
        unsafe {
            send_sweep!(display_state, "{}", TO_SQUARE(move_0));
        }
    }

    fn midgame_display_initial_ponder_move(alpha: i32, beta: i32) {
        use std::fmt::Write;
        let mut buffer_ = String::with_capacity(32);
        unsafe {
            if alpha <= -(29000) && beta >= 29000 {
                write!(buffer_,
                        "[-inf,inf]:");
            } else if alpha <= -(29000) &&
                beta < 29000 {
                write!(buffer_,
                        "[-inf,{:.1}]:",
                        beta as f64 / 128.0f64);
            } else if alpha > -(29000) &&
                beta >= 29000 {
                write!(buffer_,
                        "[{:.1},inf]:",
                        alpha as f64 / 128.0f64);
            } else {
                write!(buffer_,
                        "[{:.1},{:.1}]:",
                        alpha as f64 / 128.0f64,
                        beta as f64 / 128.0f64);
            }
            let ds = &mut display_state;
            ds.clear_sweep();
            send_sweep!(ds, "{:<14} ", buffer_);
        }
    }

    fn midgame_display_ponder_move(max_depth: i32, alpha: i32, beta: i32, curr_val: i32,
                                   searched: i32, update_pv: i32, echo: i32) {
        unsafe {
            let ds = &mut display_state;
            if update_pv != 0 {
                if curr_val <= alpha {
                    send_sweep!(ds, "<{:.2}",
                                 (curr_val + 1) as f64
                                   / 128.0f64);
                } else if curr_val >= beta {
                    send_sweep!(ds, ">{:.2}",
                                 (curr_val - 1) as f64
                                   / 128.0f64);
                } else {
                    send_sweep!(ds, "={:.2}",
                                 curr_val as f64 / 128.0f64);
                }
            }
            send_sweep!(ds, " ");
            if update_pv != 0 && searched > 0 && echo != 0 &&
                max_depth >= 10 {
                ds.display_sweep(&mut stdout);
            }
        }
    }

     fn midgame_display_status(side_to_move: i32, max_depth: i32,
                               eval_info: &EvaluationType, depth: i32,
                               force_return_: bool,
                               mut g_timer: &mut Timer,
                               mut moves_state: &mut MovesState,
                               mut board_state: &mut BoardState,
                               mut hash_state: &mut HashState,
                               mut search_state: &mut SearchState,
                               mut flip_stack_: &mut FlipStack
     ) {
         let mut nodes_counter: &mut CounterType = &mut search_state.nodes;
         unsafe {
             let ds = &mut display_state;
             ds.clear_status();
             send_status!(ds, "--> ");
             if g_timer.is_panic_abort() != 0 || force_return_ {
                 send_status!(ds, "*");
             } else {
                 send_status!(ds, " ");
             }
             send_status!(ds, "{:2}  ",
                         depth);
             let mut eval_str = produce_eval_text(eval_info, 1);
             send_status!(ds, "{:<10}  ",
                         eval_str);
             let node_val = counter_value(nodes_counter);
             ds.send_status_nodes(node_val);
             if search_state.get_ponder_move() != 0 {
                 send_status!(ds, "{{{}}} ",TO_SQUARE(search_state.get_ponder_move()));
             }
             hash_expand_pv(side_to_move, 0, 4, 12345678, &mut board_state, &mut hash_state, &mut moves_state, &mut flip_stack_);
             let mut pv_zero: &mut [i8; 64] = &mut board_state.pv[0];
             let mut pv_depth_zero: i32 = board_state.pv_depth[0];

             ds.send_status_pv(pv_zero, max_depth, pv_depth_zero);
             ds.send_status_time( g_timer.get_elapsed_time());
             if  g_timer.get_elapsed_time() != 0.0f64 {
                 send_status!(ds, "{:6.0} {}",
                             node_val / ( g_timer.get_elapsed_time() + 0.001f64),
                             "nps");
             }
         }
    }

    fn report_mirror_symetry_error(count: i32, i: i32, first_mirror_offset: i32, first_item: i32, second_item: i32) {
        write!(stdout, "{} @ {} <--> {} of {}\n", "Mirror symmetry error", i, first_mirror_offset, count);
        write!(stdout, "{} <--> {}\n", first_item, second_item);
    }
}

impl LibcFatalError {
    pub fn thordb_report_flipped_0_first() {
        write!(stdout, "This COULD happen (1) in BUILD_THOR_OPENING_TREE");
    }
    pub fn thordb_report_flipped_0_second() {
        write!(stdout, "This COULD happen (2) in BUILD_THOR_OPENING_TREE");
    }

    pub fn report_do_evaluate(evaluation_stage_: i32) {
        write!(stdout, "|");
        if evaluation_stage_ % 5 == 0 {
            write!(stdout, " {}% ", 4 * evaluation_stage_);
        }
        stdout.flush();
    }
}

impl FatalError for LibcFatalError {
    fn invalid_move(curr_move: i8) -> ! {
        fatal_error!("Thor book move {} is invalid!", curr_move);
    }

    fn unrecognized_character(unrecognized: i8) -> ! {
        fatal_error!("{} '{}' {}\n", "Unrecognized character", char::from(unrecognized as u8), "in game file");
    }

    fn cannot_open_game_file(file_name: &str) -> ! {
        fatal_error!("{} '{}'\n", "Cannot open game file" , file_name);
    }

    fn invalid_move_in_move_sequence(curr_move: i8) -> ! {
        fatal_error!("Invalid move {} in move sequence", TO_SQUARE(curr_move));
    }

    fn internal_error_in_book_code() -> ! {
        fatal_error!("Internal error in book code.");
    }

    fn unexpected_character_in_a_move_string() -> ! {
        fatal_error!("Unexpected character in move string");
    }

    fn invalid_move_string_provided() -> ! {
        fatal_error!("Invalid move string provided");
    }
    fn initial_squares_are_empty() -> ! {
        fatal_error!("Initial squares (d4, e4, d5, e5) from the board file should not be empty.\n");
    }
}
