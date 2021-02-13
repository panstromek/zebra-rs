

use std::ffi::{c_void, CString};

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

use engine::src::zebra::{EvaluationType, Config};
use libc_wrapper::{ctime, exit, fflush, fopen, fprintf, free, malloc, printf, putc, puts, realloc, sprintf, stderr, stdout, strchr, strdup, strlen, time, time_t, tolower, toupper, FileHandle};
use thordb_types::C2RustUnnamed;

use crate::{
    src::{
        display::{display_status, display_sweep, produce_eval_text,
                  send_status_nodes, send_status_pv, send_status_time, send_sweep_1},
    }
};
use crate::src::display::{clear_status, clear_sweep, reset_buffer_display, display_state, send_status_1, send_status_2, send_status_0, send_sweep_0, send_sweep_2, CFormat};
use crate::src::osfbook::print_move_alternatives;
use crate::src::thordb::sort_thor_games;
use crate::src::zebra::FullState;
use engine::src::timer::Timer;
use engine::src::osfbook::Book;
use engine::src::globals::BoardState;
use engine::src::moves::MovesState;
use flip::unflip::FlipStack;

static mut buffer: [i8; 16] = [0; 16];

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

pub unsafe fn fatal_error_3<T: CFormat, U: CFormat, V: CFormat>(format: *const i8, arg: T, arg2: U, arg3: V) -> ! {
    fatal_error(|stream| {
        fprintf(stream, format, arg, arg2, arg3);
    })
}

pub unsafe fn fatal_error_2<T: CFormat, U: CFormat>(format: *const i8, arg: T, arg2: U) -> ! {
    fatal_error(|stream| {
        fprintf(stream, format, arg, arg2);
    })
}

pub unsafe fn fatal_error_1<T: CFormat>(format: *const i8, arg: T) -> ! {
    fatal_error(|stream| {
        fprintf(stream, format, arg);
    })
}

pub unsafe fn fatal_error_0(format: *const i8) -> ! {
    fatal_error(|stream| {
        fprintf(stream, format);
    })
}

unsafe fn fatal_error(mut variadic_printer: impl FnMut(FileHandle)) -> ! {
    let mut timer: time_t = 0;
    eprint!("\nFatal error: ");

    variadic_printer(stderr);
    let stream =
        fopen(b"zebra.err\x00" as *const u8 as *const i8,
              b"a\x00" as *const u8 as *const i8);
    if !stream.is_null() {
        time(&mut timer);
        fprintf(stream,
                b"%s @ %s\n  \x00" as *const u8 as *const i8,
                b"Fatal error\x00" as *const u8 as *const i8,
                ctime(&mut timer));
        variadic_printer(stream);
    }
    std::process::exit(1);
}

pub struct LibcFatalError; // FIXME rename this, it's not only error anymore
pub type FE = LibcFatalError;

impl LibcFatalError {
    pub fn choose_thor_opening_move_report(freq_sum: i32, match_count: i32, move_list: &[C2RustUnnamed; 64]) {
        unsafe {
            printf(b"%s:        \x00" as *const u8 as *const i8,
                   b"Thor database\x00" as *const u8 as *const i8);
            let mut i = 0;
            while i < match_count {
                printf(b"%c%c: %4.1f%%    \x00" as *const u8 as
                           *const i8,
                       'a' as i32 +
                           move_list[i as usize].move_0 % 10 as i32 -
                           1 as i32,
                       '0' as i32 +
                           move_list[i as usize].move_0 / 10 as i32,
                       100.0f64 *
                           move_list[i as usize].frequency as f64 /
                           freq_sum as f64);
                if i % 6 as i32 == 4 as i32 {
                    puts(b"\x00" as *const u8 as *const i8);
                }
                i += 1
            }
            if match_count % 6 as i32 != 5 as i32 {
                puts(b"\x00" as *const u8 as *const i8);
            }
        }
    }
    #[inline(always)]
    pub fn sort_thor_games(count: i32) {
        unsafe { sort_thor_games(count) }
    }
    pub fn memory_allocation_failure(block_count_: i32) -> ! {
        unsafe {
            fatal_error_2(b"%s @ #%d\n\x00" as *const u8 as *const i8,
                        b"Memory allocation failure\x00" as *const u8 as
                            *const i8, block_count_);
        }
    }

    pub fn error_in_map(i: i32, pos: i32, symmetry_map_item: i32) -> ! {
        unsafe {
            fatal_error_3(b"Error in map %d: inv(map(%d))=%d\n\x00" as
                            *const u8 as *const i8, i, pos, symmetry_map_item);
        }
    }

    pub fn error_in_map_thor(i: i32, pos: i32, to_report: i32) -> ! {
        unsafe {
            fatal_error_3(b"Error in map %d: inv(map(%d))=%d\n\x00" as
                            *const u8 as *const i8, i, pos,
                        to_report);
        }
    }

}
impl FrontEnd for LibcFatalError {
    fn reset_buffer_display(g_timer:&mut Timer) {
        unsafe { reset_buffer_display(g_timer) }
    }
    /*
      DISPLAY_BUFFERS
      If an update has happened and the last display was long enough ago,
      output relevant buffers.
    */
    fn display_buffers(g_timer: &mut Timer) {
        unsafe {
            let timer =  g_timer.get_real_timer();
            if timer - display_state.last_output >= display_state.interval2 || display_state.timed_buffer_management == 0 {
                display_status(stdout, 0 as i32);
                display_state.status_modified = 0;
                if timer - display_state.last_output >= display_state.interval2 {
                    if display_state.sweep_modified != 0 { display_sweep(stdout); }
                    display_state.last_output = timer;
                    /* Display the sweep at Fibonacci-spaced times */
                    let new_interval = display_state.interval1 + display_state.interval2;
                    display_state.interval1 = display_state.interval2;
                    display_state.interval2 = new_interval
                }
            };
        }
    }

    fn after_update_best_list_verbose(best_list: &mut [i32; 4]) {
        unsafe {
            // let best_list = best_list.as_mut_ptr();
            printf(b"      After:  \x00" as *const u8 as *const i8);
            let mut i = 0;
            while i < 4 {
                printf(b"%2d \x00" as *const u8 as *const i8, best_list[i]);
                i += 1
            }
            puts(b"\x00" as *const u8 as *const i8);
        }
    }
    fn before_update_best_list_verbose(best_list: &mut [i32; 4], move_0: i32, best_list_index: i32, best_list_length: &mut i32) {
        unsafe {
            printf(b"move=%2d  index=%d  length=%d      \x00" as *const u8 as
                       *const i8, move_0, best_list_index, *best_list_length);
            printf(b"Before:  \x00" as *const u8 as *const i8);
            let mut i = 0;
            while i < 4 {
                printf(b"%2d \x00" as *const u8 as *const i8, best_list[i]);
                i += 1
            }
        }
    }

    fn end_tree_search_output_some_second_stats(alpha: i32, beta: i32, curr_val: i32, update_pv: i32, move_index: i32, echo: i32) {
        unsafe {
            if update_pv != 0 {
                Self::end_tree_search_some_pv_stats_report(alpha, beta, curr_val)
            }
            send_sweep_0(b" \x00" as *const u8 as *const i8);
            if update_pv != 0 && move_index > 0 as i32 && echo != 0 {
                display_sweep(stdout);
            }
        }
    }

    fn end_tree_search_some_pv_stats_report(alpha: i32, beta: i32, curr_val: i32) {
        unsafe {
            if curr_val <= alpha {
                send_sweep_1(b"<%d\x00" as *const u8 as *const i8,
                             curr_val + 1 as i32);
            } else if curr_val >= beta {
                send_sweep_1(b">%d\x00" as *const u8 as *const i8,
                             curr_val - 1 as i32);
            } else {
                send_sweep_1(b"=%d\x00" as *const u8 as *const i8,
                             curr_val);
            }
        }
    }

    fn end_tree_search_level_0_ponder_0_short_report(move_0: i32, first: i32) {
        unsafe {
            if first != 0 {
                send_sweep_1(b"%-10s \x00" as *const u8 as *const i8,
                             buffer.as_mut_ptr());
            }
            send_sweep_2(b"%c%c\x00" as *const u8 as *const i8,
                         'a' as i32 + move_0 % 10 as i32 -
                           1 as i32,
                         '0' as i32 + move_0 / 10 as i32);
        }
    }

    fn end_tree_search_output_some_stats(entry: &HashEntry) {
        /* Output some stats */
        unsafe {
            send_sweep_2(b"%c%c\x00" as *const u8 as *const i8,
                         'a' as i32 +
                           entry.move_0[0] %
                               10 as i32 - 1 as i32,
                         '0' as i32 +
                           entry.move_0[0] /
                               10 as i32);
            if entry.flags as i32 & 16 as i32 != 0 &&
                entry.flags as i32 & 4 as i32 != 0 {
                send_sweep_1(b"=%d\x00" as *const u8 as *const i8,
                             entry.eval);
            } else if entry.flags as i32 & 16 as i32 != 0
                &&
                entry.flags as i32 & 1 as i32 !=
                    0 {
                send_sweep_1(b">%d\x00" as *const u8 as *const i8,
                             entry.eval - 1 as i32);
            } else {
                send_sweep_1(b"<%d\x00" as *const u8 as *const i8,
                             entry.eval + 1 as i32);
            }
            fflush(stdout);
        }
    }

     fn end_tree_search_level_0_ponder_0_report(alpha: i32, beta: i32, result: i32, best_move_: i32) {
         unsafe {
             send_sweep_1(b"%-10s \x00" as *const u8 as *const i8,
                          buffer.as_mut_ptr());
             send_sweep_2(b"%c%c\x00" as *const u8 as *const i8,
                          'a' as i32 + best_move_ % 10 as i32 -
                            1 as i32,
                          '0' as i32 + best_move_ / 10 as i32);
             if result <= alpha {
                 send_sweep_1(b"<%d\x00" as *const u8 as *const i8,
                              result + 1 as i32);
             } else if result >= beta {
                 send_sweep_1(b">%d\x00" as *const u8 as *const i8,
                              result - 1 as i32);
             } else {
                 send_sweep_1(b"=%d\x00" as *const u8 as *const i8,
                              result);
             }
         }
    }

    fn end_tree_search_level_0_report(alpha: i32, beta: i32) {
        unsafe {
            sprintf(buffer.as_mut_ptr(), b"[%d,%d]:\x00" as *const u8 as *const i8, alpha, beta);
            clear_sweep();
        }
    }
    /*
      SEND_SOLVE_STATUS
      Displays endgame results - partial or full.
    */
    fn send_solve_status(empties: i32, _side_to_move: i32, eval_info: &mut EvaluationType,
                          pv_zero: &mut [i32; 64],
                         pv_depth_zero: i32,
                         mut g_timer: &mut Timer,
                         mut search_state: &mut SearchState) {
        unsafe {
            let eval = *eval_info;
            search_state.set_current_eval(eval);
            clear_status();
            send_status_1(b"-->  %2d  \x00" as *const u8 as *const i8, empties);
            let mut eval_str_ = produce_eval_text(&*eval_info, 1 as i32);
            let eval_str = eval_str_.as_mut_ptr();
            send_status_1(b"%-10s  \x00" as *const u8 as *const i8, eval_str);
            let nodes_counter: &mut CounterType = &mut search_state.nodes;
            let node_val = counter_value(nodes_counter);
            send_status_nodes(node_val);
            if search_state.get_ponder_move() != 0 {
                send_status_2(b"{%c%c} \x00" as *const u8 as *const i8,
                            'a' as i32 + search_state.get_ponder_move() % 10 as i32 -
                                1 as i32,
                            '0' as i32 + search_state.get_ponder_move() / 10 as i32);
            }
            send_status_pv(pv_zero, empties, pv_depth_zero);
            send_status_time( g_timer.get_elapsed_time());
            if  g_timer.get_elapsed_time() > 0.0001f64 {
                send_status_2(b"%6.0f %s  \x00" as *const u8 as *const i8,
                            node_val / ( g_timer.get_elapsed_time() + 0.0001f64),
                            b"nps\x00" as *const u8 as *const i8);
            };
        }
    }

    fn end_report_panic_abort_2(elapsed_time: f64) {
        unsafe {
            printf(b"%s %.1f %c %s\n\x00" as *const u8 as *const i8,
                   b"Panic abort after\x00" as *const u8 as *const i8, elapsed_time, 's' as i32,
                   b"in selective search\x00" as *const u8 as *const i8);
        }
    }

     fn end_report_semi_panic_abort_3(elapsed_time: f64) {
         unsafe {
             printf(b"%s %.1f %c %s\n\x00" as *const u8 as
                        *const i8,
                    b"Semi-panic abort after\x00" as *const u8 as
                        *const i8, elapsed_time,
                    's' as i32,
                    b"in WLD search\x00" as *const u8 as
                        *const i8);
         }
    }

    fn end_report_semi_panic_abort_2(elapsed_time: f64) {
        unsafe {
            printf(b"%s %.1f %c %s\n\x00" as *const u8 as *const i8,
                   b"Semi-panic abort after\x00" as *const u8 as
                       *const i8, elapsed_time, 's' as i32,
                   b"in exact search\x00" as *const u8 as
                       *const i8);
        }
    }

    fn end_report_panic_abort(elapsed_time: f64) {
        unsafe {
            printf(b"%s %.1f %c %s\n\x00" as *const u8 as
                       *const i8,
                   b"Panic abort after\x00" as *const u8 as
                       *const i8, elapsed_time,
                   's' as i32,
                   b"in WLD search\x00" as *const u8 as
                       *const i8);
        }
    }

    fn end_report_semi_panic_abort(elapsed_time: f64) {
        unsafe {
            printf(b"%s %.1f %c %s\n\x00" as *const u8 as
                       *const i8,
                   b"Semi-panic abort after\x00" as *const u8 as
                       *const i8, elapsed_time,
                   's' as i32,
                   b"in selective search\x00" as *const u8 as
                       *const i8);
        }
    }

    fn end_display_zero_status() {
        unsafe {
            display_status(stdout, 0 as i32);
        }
    }

    fn handle_fatal_pv_error(i: i32, pv_0_depth: i32, pv_0: &[i32; 64]) {
        unsafe {
            printf(b"pv_depth[0] = %d\n\x00" as *const u8 as
                       *const i8,
                   pv_0_depth);
            let mut j = 0;
            while j < pv_0_depth {
                printf(b"%c%c \x00" as *const u8 as *const i8,
                       'a' as i32 +
                           pv_0[j as usize] %
                               10 as i32 - 1 as i32,
                       '0' as i32 +
                           pv_0[j as usize] /
                               10 as i32);
                j += 1
            }
            puts(b"\x00" as *const u8 as *const i8);
            printf(b"i=%d\n\x00" as *const u8 as *const i8, i);
            fatal_error_0(b"Error in PV completion\x00" as *const u8 as
                *const i8);
        }
    }

    fn report_unwanted_book_draw(this_move: i32) {
        unsafe {
            printf(b"%c%c leads to an unwanted book draw\n\x00" as *const u8 as *const i8, 'a' as i32 + this_move % 10 as i32 - 1 as i32, '0' as i32 + this_move / 10 as i32);
        }
    }

    fn report_in_get_book_move_1(side_to_move: i32, remaining_slack: i32, board_state: &mut BoardState, g_book: &mut Book) {
        unsafe {
            printf(b"Slack left is %.2f. \x00" as *const u8 as
                       *const i8,
                   remaining_slack as f64 / 128.0f64);
            print_move_alternatives(side_to_move,board_state, g_book );
        }
    }
    fn report_in_get_book_move_2(chosen_score: i32, chosen_index: i32, flags: &i32, candidate_list_: &[CandidateMove; 60], search_state: & SearchState) {
        unsafe {
            send_status_0(b"-->   Book     \x00" as *const u8 as
                *const i8);
            if flags & 16 as i32 != 0 {
                send_status_1(b"%+3d (exact)   \x00" as *const u8 as
                                *const i8,
                            chosen_score / 128 as i32);
            } else if flags & 4 as i32 != 0 {
                send_status_1(b"%+3d (WLD)     \x00" as *const u8 as
                                *const i8,
                            chosen_score / 128 as i32);
            } else {
                send_status_1(b"%+6.2f        \x00" as *const u8 as
                                *const i8,
                            chosen_score as f64 / 128.0f64);
            }
            if search_state.get_ponder_move() != 0 {
                send_status_2(b"{%c%c} \x00" as *const u8 as *const i8,
                            'a' as i32 + search_state.get_ponder_move() % 10 as i32 -
                                1 as i32,
                            '0' as i32 + search_state.get_ponder_move() / 10 as i32);
            }
            send_status_2(b"%c%c\x00" as *const u8 as *const i8,
                        'a' as i32 +
                            candidate_list_[chosen_index as usize].move_0 %
                                10 as i32 - 1 as i32,
                        '0' as i32 +
                            candidate_list_[chosen_index as usize].move_0 /
                                10 as i32);
        }
    }
    fn midgame_display_simple_ponder_move(move_0: i32) {
        unsafe {
            send_sweep_2(b"%c%c\x00" as *const u8 as *const i8,
                         'a' as i32 + move_0 % 10 as i32 -
                           1 as i32,
                         '0' as i32 + move_0 / 10 as i32);
        }
    }

    fn midgame_display_initial_ponder_move(alpha: i32, beta: i32) {
        let buffer_: &mut [i8; 32] = &mut [0; 32];
        unsafe {
            if alpha <= -(29000 as i32) && beta >= 29000 as i32 {
                sprintf(buffer_.as_mut_ptr(),
                        b"[-inf,inf]:\x00" as *const u8 as *const i8);
            } else if alpha <= -(29000 as i32) &&
                beta < 29000 as i32 {
                sprintf(buffer_.as_mut_ptr(),
                        b"[-inf,%.1f]:\x00" as *const u8 as *const i8,
                        beta as f64 / 128.0f64);
            } else if alpha > -(29000 as i32) &&
                beta >= 29000 as i32 {
                sprintf(buffer_.as_mut_ptr(),
                        b"[%.1f,inf]:\x00" as *const u8 as *const i8,
                        alpha as f64 / 128.0f64);
            } else {
                sprintf(buffer_.as_mut_ptr(),
                        b"[%.1f,%.1f]:\x00" as *const u8 as *const i8,
                        alpha as f64 / 128.0f64,
                        beta as f64 / 128.0f64);
            }
            clear_sweep();
            send_sweep_1(b"%-14s \x00" as *const u8 as *const i8,
                         buffer_.as_mut_ptr());
        }
    }

    fn midgame_display_ponder_move(max_depth: i32, alpha: i32, beta: i32, curr_val: i32,
                                   searched: i32, update_pv: i32, echo: i32) {
        unsafe {

            if update_pv != 0 {
                if curr_val <= alpha {
                    send_sweep_1(b"<%.2f\x00" as *const u8 as
                                   *const i8,
                                 (curr_val + 1 as i32) as f64
                                   / 128.0f64);
                } else if curr_val >= beta {
                    send_sweep_1(b">%.2f\x00" as *const u8 as
                                   *const i8,
                                 (curr_val - 1 as i32) as f64
                                   / 128.0f64);
                } else {
                    send_sweep_1(b"=%.2f\x00" as *const u8 as
                                   *const i8,
                                 curr_val as f64 / 128.0f64);
                }
            }
            send_sweep_0(b" \x00" as *const u8 as *const i8);
            if update_pv != 0 && searched > 0 as i32 && echo != 0 &&
                max_depth >= 10 as i32 {
                display_sweep(stdout);
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
             clear_status();
             send_status_0(b"--> \x00" as *const u8 as *const i8);
             if g_timer.is_panic_abort() != 0 || force_return_ {
                 send_status_0(b"*\x00" as *const u8 as *const i8);
             } else {
                 send_status_0(b" \x00" as *const u8 as *const i8);
             }
             send_status_1(b"%2d  \x00" as *const u8 as *const i8,
                         depth);
             let mut eval_str_ = produce_eval_text(eval_info, 1 as i32);
             let eval_str = eval_str_.as_mut_ptr();
             send_status_1(b"%-10s  \x00" as *const u8 as *const i8,
                         eval_str);
             let node_val = counter_value(nodes_counter);
             send_status_nodes(node_val);
             if search_state.get_ponder_move() != 0 {
                 send_status_2(b"{%c%c} \x00" as *const u8 as
                                 *const i8,
                             'a' as i32 + search_state.get_ponder_move() % 10 as i32
                                 - 1 as i32,
                             '0' as i32 +
                                 search_state.get_ponder_move() / 10 as i32);
             }
             hash_expand_pv(side_to_move, 0 as i32, 4 as i32, 12345678 as i32, &mut board_state, &mut hash_state, &mut moves_state, &mut flip_stack_);
             let mut pv_zero: &mut [i32; 64] = &mut board_state.pv[0];
             let mut pv_depth_zero: i32 = board_state.pv_depth[0];

             send_status_pv(pv_zero, max_depth, pv_depth_zero);
             send_status_time( g_timer.get_elapsed_time());
             if  g_timer.get_elapsed_time() != 0.0f64 {
                 send_status_2(b"%6.0f %s\x00" as *const u8 as
                                 *const i8,
                             node_val / ( g_timer.get_elapsed_time() + 0.001f64),
                             b"nps\x00" as *const u8 as *const i8);
             }

         }
    }

    fn report_mirror_symetry_error(count: i32, i: i32, first_mirror_offset: i32, first_item: i32, second_item: i32) {
        unsafe {
            printf(b"%s @ %d <--> %d of %d\n\x00" as *const u8 as
                       *const i8,
                   b"Mirror symmetry error\x00" as *const u8 as
                       *const i8, i, first_mirror_offset,
                   count);
            printf(b"%d <--> %d\n\x00" as *const u8 as
                       *const i8,
                   first_item,
                   second_item);
        }
    }
}
impl LibcFatalError {
    pub fn thordb_report_flipped_0_first() {
        unsafe {
            puts(b"This COULD happen (1) in BUILD_THOR_OPENING_TREE\x00" as *const u8 as *const i8);
        }
    }
    pub fn thordb_report_flipped_0_second() {
        unsafe {
            puts(b"This COULD happen (2) in BUILD_THOR_OPENING_TREE\x00" as *const u8 as *const i8);
        }
    }

    pub fn report_do_evaluate(evaluation_stage_: i32) {
        unsafe {
            putc('|' as i32, stdout);
            if evaluation_stage_ % 5 as i32 == 0 as i32 {
                printf(b" %d%% \x00" as *const u8 as *const i8,
                       4 as i32 * evaluation_stage_);
            }
            fflush(stdout);
        }
    }
}

impl FatalError for LibcFatalError {
  fn invalid_move(curr_move: i32) -> ! {
    unsafe {
        fatal_error_1(b"Thor book move %d is invalid!\x00" as *const u8
                      as *const i8, curr_move);
    }
  }

 fn unrecognized_character(unrecognized: i8) -> ! {
  unsafe {
      fatal_error_3(b"%s \'%c\' %s\n\x00" as *const u8 as
                    *const i8,
                b"Unrecognized character\x00" as *const u8 as
                    *const i8,
                unrecognized as i32,
                b"in game file\x00" as *const u8 as
                    *const i8);
  }
}

fn cannot_open_game_file(file_name: &str) -> ! {
    let file_name: *const i8 = CString::new(file_name).unwrap().as_c_str().as_ptr();
    unsafe {
        fatal_error_2(b"%s \'%s\'\n\x00" as *const u8 as
                        *const i8,
                    b"Cannot open game file\x00" as *const u8 as
                        *const i8, file_name);
    }
}


fn invalid_move_in_move_sequence(curr_move: i32) -> ! {
  unsafe {
    fatal_error_2(b"Invalid move %c%c in move sequence\x00"
                    as *const u8 as *const i8,
                'a' as i32 + curr_move % 10 as i32
                    - 1 as i32,
                '0' as i32 +
                    curr_move / 10 as i32);
  }
}

 fn internal_error_in_book_code() -> ! {
    unsafe {
        fatal_error_0(b"Internal error in book code.\x00" as *const u8 as
            *const i8);
    }
}

 fn unexpected_character_in_a_move_string() -> ! {
    unsafe {
        fatal_error_0(b"Unexpected character in move string\x00" as *const u8 as *const i8);
    }
}

 fn invalid_move_string_provided() -> ! {
    unsafe {
        fatal_error_0(b"Invalid move string provided\x00" as *const u8 as *const i8);
    }
}
}
