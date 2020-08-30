use libc_wrapper::{vfprintf, ctime, fprintf, time, fopen, stderr, exit, strchr, strdup, toupper, tolower, strlen, free, malloc, realloc, puts, printf, putc, sprintf, fflush, time_t, stdout};
use engine::src::error::{FrontEnd, FatalError};
use engine::src::hash::HashEntry;
use engine::src::thordb::C2RustUnnamed;
use engine::src::zebra::EvaluationType;
use crate::src::thordb::{sort_thor_games};
use crate::src::osfbook::{print_move_alternatives};
use std::ffi::c_void;
use engine::{
    src:: {
        search::{get_ponder_move, set_current_eval},
        counter::{counter_value},
    }
};
use crate::{
    src::{
        display::{display_status, send_status, send_status_time,
                  send_status_pv, send_status_nodes, produce_eval_text, display_sweep, send_sweep},
    }
};
use engine::src::display::{clear_status, echo, clear_sweep, interval2, interval1,
                           last_output, sweep_modified, status_modified, timed_buffer_management};
use engine::src::timer::{get_elapsed_time, is_panic_abort, get_real_timer};
use engine::src::search::{hash_expand_pv};

use engine::src::game::CandidateMove;
use engine::src::counter::CounterType;
use engine::src::end::best_move;

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

pub unsafe extern "C" fn fatal_error(format: *const i8, args: ...) -> ! {
    let mut timer: time_t = 0;
    let mut arg_ptr = args.clone();
    fprintf(stderr, b"\n%s: \x00" as *const u8 as *const i8,
            b"Fatal error\x00" as *const u8 as *const i8);
    vfprintf(stderr, format, arg_ptr.as_va_list());
    let stream =
        fopen(b"zebra.err\x00" as *const u8 as *const i8,
              b"a\x00" as *const u8 as *const i8);
    if !stream.is_null() {
        time(&mut timer);
        fprintf(stream,
                b"%s @ %s\n  \x00" as *const u8 as *const i8,
                b"Fatal error\x00" as *const u8 as *const i8,
                ctime(&mut timer));
        arg_ptr = args.clone();
        vfprintf(stream, format, arg_ptr.as_va_list());
    }
    exit(1 as i32);
}

pub struct LibcFatalError; // FIXME rename this, it's not only error anymore
pub type FE = LibcFatalError;


impl FrontEnd for LibcFatalError {
    /*
      DISPLAY_BUFFERS
      If an update has happened and the last display was long enough ago,
      output relevant buffers.
    */
    fn display_buffers() {
        unsafe {
            let timer = get_real_timer::<FE>();
            if timer - last_output >= interval2 || timed_buffer_management == 0 {
                display_status(stdout, 0 as i32);
                status_modified = 0 as i32;
                if timer - last_output >= interval2 {
                    if sweep_modified != 0 { display_sweep(stdout); }
                    last_output = timer;
                    /* Display the sweep at Fibonacci-spaced times */
                    let new_interval = interval1 + interval2;
                    interval1 = interval2;
                    interval2 = new_interval
                }
            };
        }
    }

    fn report_ponder_time(current_ponder_time_: f64, current_ponder_depth_: i32) {
        unsafe {
            printf(b"Ponder time: %.1f s\n\x00" as *const u8 as *const i8, current_ponder_time_);
            printf(b"Ponder depth: %d\n\x00" as *const u8 as *const i8, current_ponder_depth_);
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

    fn end_tree_search_output_some_second_stats(alpha: i32, beta: i32, curr_val: i32, update_pv: i32, move_index: i32) {
        unsafe {
            if update_pv != 0 {
                Self::end_tree_search_some_pv_stats_report(alpha, beta, curr_val)
            }
            send_sweep(b" \x00" as *const u8 as *const i8);
            if update_pv != 0 && move_index > 0 as i32 && echo != 0 {
                display_sweep(stdout);
            }
        }
    }

    fn end_tree_search_some_pv_stats_report(alpha: i32, beta: i32, curr_val: i32) {
        unsafe {
            if curr_val <= alpha {
                send_sweep(b"<%d\x00" as *const u8 as *const i8,
                           curr_val + 1 as i32);
            } else if curr_val >= beta {
                send_sweep(b">%d\x00" as *const u8 as *const i8,
                           curr_val - 1 as i32);
            } else {
                send_sweep(b"=%d\x00" as *const u8 as *const i8,
                           curr_val);

                // TODO wtf are these???? they are not used...
            /*
                pub static mut true_found: i32 = 0;
                pub static mut true_val: i32 = 0;
                ****
                true_found = 1 as i32;
                true_val = curr_val;
            */
            }
        }
    }

    fn end_tree_search_level_0_ponder_0_short_report(move_0: i32, first: i32) {
        unsafe {
            if first != 0 {
                send_sweep(b"%-10s \x00" as *const u8 as *const i8,
                           buffer.as_mut_ptr());
            }
            send_sweep(b"%c%c\x00" as *const u8 as *const i8,
                       'a' as i32 + move_0 % 10 as i32 -
                           1 as i32,
                       '0' as i32 + move_0 / 10 as i32);
        }
    }

    fn end_tree_search_output_some_stats(entry: &HashEntry) {
        /* Output some stats */
        unsafe {
            send_sweep(b"%c%c\x00" as *const u8 as *const i8,
                       'a' as i32 +
                           entry.move_0[0 as i32 as usize] %
                               10 as i32 - 1 as i32,
                       '0' as i32 +
                           entry.move_0[0 as i32 as usize] /
                               10 as i32);
            if entry.flags as i32 & 16 as i32 != 0 &&
                entry.flags as i32 & 4 as i32 != 0 {
                send_sweep(b"=%d\x00" as *const u8 as *const i8,
                           entry.eval);
            } else if entry.flags as i32 & 16 as i32 != 0
                &&
                entry.flags as i32 & 1 as i32 !=
                    0 {
                send_sweep(b">%d\x00" as *const u8 as *const i8,
                           entry.eval - 1 as i32);
            } else {
                send_sweep(b"<%d\x00" as *const u8 as *const i8,
                           entry.eval + 1 as i32);
            }
            fflush(stdout);
        }
    }

     fn end_tree_search_level_0_ponder_0_report(alpha: i32, beta: i32, result: i32) {
         unsafe {
             send_sweep(b"%-10s \x00" as *const u8 as *const i8,
                        buffer.as_mut_ptr());
             send_sweep(b"%c%c\x00" as *const u8 as *const i8,
                        'a' as i32 + best_move % 10 as i32 -
                            1 as i32,
                        '0' as i32 + best_move / 10 as i32);
             if result <= alpha {
                 send_sweep(b"<%d\x00" as *const u8 as *const i8,
                            result + 1 as i32);
             } else if result >= beta {
                 send_sweep(b">%d\x00" as *const u8 as *const i8,
                            result - 1 as i32);
             } else {
                 send_sweep(b"=%d\x00" as *const u8 as *const i8,
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
                         nodes_counter: &mut CounterType, pv_zero: &mut [i32; 64], pv_depth_zero: i32) {
        unsafe {
            set_current_eval(*eval_info);
            clear_status();
            send_status(b"-->  %2d  \x00" as *const u8 as *const i8, empties);
            let eval_str = produce_eval_text(&*eval_info, 1 as i32);
            send_status(b"%-10s  \x00" as *const u8 as *const i8, eval_str);
            free(eval_str as *mut std::ffi::c_void);
            let node_val = counter_value(nodes_counter);
            send_status_nodes(node_val);
            if get_ponder_move() != 0 {
                send_status(b"{%c%c} \x00" as *const u8 as *const i8,
                            'a' as i32 + get_ponder_move() % 10 as i32 -
                                1 as i32,
                            '0' as i32 + get_ponder_move() / 10 as i32);
            }
            send_status_pv(pv_zero, empties, pv_depth_zero);
            send_status_time(get_elapsed_time::<FE>());
            if get_elapsed_time::<FE>() > 0.0001f64 {
                send_status(b"%6.0f %s  \x00" as *const u8 as *const i8,
                            node_val / (get_elapsed_time::<FE>() + 0.0001f64),
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
            let mut j = 0 as i32;
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
            fatal_error(b"Error in PV completion\x00" as *const u8 as
                *const i8);
        }
    }

    #[inline(always)]
    unsafe fn malloc(size: u64) -> *mut c_void {
        unsafe { malloc(size) }
    }
    #[inline(always)]
    unsafe fn realloc(ptr: *mut c_void, size: u64) -> *mut c_void {
        unsafe { realloc(ptr, size) }
    }
    #[inline(always)]
    unsafe fn free(__ptr: *mut c_void) {
        unsafe { free(__ptr) }
    }
    #[inline(always)]
    fn time(__timer: &mut i64) -> i64 {
        unsafe { time(__timer) }
    }
    #[inline(always)]
    unsafe fn strlen(s: *const i8) -> u64 {
        unsafe { strlen(s) }
    }
    #[inline(always)]
    fn tolower(num: i32) -> i32 {
        unsafe { tolower(num) }
    }
    #[inline(always)]
    unsafe fn strdup(s: *const i8) -> *mut i8 {
        unsafe { strdup(s) }
    }
    fn report_do_evaluate(evaluation_stage_: i32) {
        unsafe {
            putc('|' as i32, stdout);
            if evaluation_stage_ % 5 as i32 == 0 as i32 {
                printf(b" %d%% \x00" as *const u8 as *const i8,
                       4 as i32 * evaluation_stage_);
            }
            fflush(stdout);
        }
    }
    fn report_unwanted_book_draw(this_move: i32) {
        unsafe {
            printf(b"%c%c leads to an unwanted book draw\n\x00" as *const u8 as *const i8, 'a' as i32 + this_move % 10 as i32 - 1 as i32, '0' as i32 + this_move / 10 as i32);
        }
    }

    fn report_in_get_book_move_1(side_to_move: i32, remaining_slack: i32) {
        unsafe {
            printf(b"Slack left is %.2f. \x00" as *const u8 as
                       *const i8,
                   remaining_slack as f64 / 128.0f64);
            print_move_alternatives(side_to_move);
        }
    }
    fn report_in_get_book_move_2(chosen_score: i32, chosen_index: i32, flags: &i32, candidate_list_: &[CandidateMove; 60]) {
        unsafe {
            send_status(b"-->   Book     \x00" as *const u8 as
                *const i8);
            if flags & 16 as i32 != 0 {
                send_status(b"%+3d (exact)   \x00" as *const u8 as
                                *const i8,
                            chosen_score / 128 as i32);
            } else if flags & 4 as i32 != 0 {
                send_status(b"%+3d (WLD)     \x00" as *const u8 as
                                *const i8,
                            chosen_score / 128 as i32);
            } else {
                send_status(b"%+6.2f        \x00" as *const u8 as
                                *const i8,
                            chosen_score as f64 / 128.0f64);
            }
            if get_ponder_move() != 0 {
                send_status(b"{%c%c} \x00" as *const u8 as *const i8,
                            'a' as i32 + get_ponder_move() % 10 as i32 -
                                1 as i32,
                            '0' as i32 + get_ponder_move() / 10 as i32);
            }
            send_status(b"%c%c\x00" as *const u8 as *const i8,
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
            send_sweep(b"%c%c\x00" as *const u8 as *const i8,
                       'a' as i32 + move_0 % 10 as i32 -
                           1 as i32,
                       '0' as i32 + move_0 / 10 as i32);
        }
    }

    fn midgame_display_initial_ponder_move(alpha: i32, beta: i32, buffer_: &mut [i8; 32]) {
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
            send_sweep(b"%-14s \x00" as *const u8 as *const i8,
                       buffer_.as_mut_ptr());
        }
    }

    fn midgame_display_ponder_move(max_depth: i32, alpha: i32, beta: i32, curr_val: i32,
                                   searched: i32, update_pv: i32) {
        unsafe {
            if update_pv != 0 {
                if curr_val <= alpha {
                    send_sweep(b"<%.2f\x00" as *const u8 as
                                   *const i8,
                               (curr_val + 1 as i32) as f64
                                   / 128.0f64);
                } else if curr_val >= beta {
                    send_sweep(b">%.2f\x00" as *const u8 as
                                   *const i8,
                               (curr_val - 1 as i32) as f64
                                   / 128.0f64);
                } else {
                    send_sweep(b"=%.2f\x00" as *const u8 as
                                   *const i8,
                               curr_val as f64 / 128.0f64);
                }
            }
            send_sweep(b" \x00" as *const u8 as *const i8);
            if update_pv != 0 && searched > 0 as i32 && echo != 0 &&
                max_depth >= 10 as i32 {
                display_sweep(stdout);
            }
        }
    }

     fn midgame_display_status(side_to_move: i32, max_depth: i32,
                               eval_info: &EvaluationType, depth: i32,
                               force_return_: bool, nodes_counter: &mut CounterType,
                               pv_zero: &mut [i32; 64], pv_depth_zero: i32) {
         unsafe {
             clear_status();
             send_status(b"--> \x00" as *const u8 as *const i8);
             if is_panic_abort() != 0 || force_return_ {
                 send_status(b"*\x00" as *const u8 as *const i8);
             } else {
                 send_status(b" \x00" as *const u8 as *const i8);
             }
             send_status(b"%2d  \x00" as *const u8 as *const i8,
                         depth);
             let eval_str = produce_eval_text(eval_info, 1 as i32);
             send_status(b"%-10s  \x00" as *const u8 as *const i8,
                         eval_str);
             free(eval_str as *mut std::ffi::c_void);
             let node_val = counter_value(nodes_counter);
             send_status_nodes(node_val);
             if get_ponder_move() != 0 {
                 send_status(b"{%c%c} \x00" as *const u8 as
                                 *const i8,
                             'a' as i32 + get_ponder_move() % 10 as i32
                                 - 1 as i32,
                             '0' as i32 +
                                 get_ponder_move() / 10 as i32);
             }
             hash_expand_pv(side_to_move, 0 as i32, 4 as i32,
                            12345678 as i32);
             send_status_pv(pv_zero, max_depth, pv_depth_zero);
             send_status_time(get_elapsed_time::<FE>());
             if get_elapsed_time::<FE>() != 0.0f64 {
                 send_status(b"%6.0f %s\x00" as *const u8 as
                                 *const i8,
                             node_val / (get_elapsed_time::<FE>() + 0.001f64),
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
    fn thordb_report_flipped_0_first() {
        unsafe {
            puts(b"This COULD happen (1) in BUILD_THOR_OPENING_TREE\x00" as *const u8 as *const i8);
        }
    }
    fn thordb_report_flipped_0_second() {
        unsafe {
            puts(b"This COULD happen (2) in BUILD_THOR_OPENING_TREE\x00" as *const u8 as *const i8);
        }
    }
    fn choose_thor_opening_move_report(freq_sum: i32, match_count: i32, move_list: &[C2RustUnnamed; 64]) {
        unsafe {
            printf(b"%s:        \x00" as *const u8 as *const i8,
                   b"Thor database\x00" as *const u8 as *const i8);
            let mut i = 0 as i32;
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
    fn sort_thor_games(count: i32) {
        unsafe { sort_thor_games(count) }
    }
}

impl FatalError for LibcFatalError {
  fn invalid_move(curr_move: i32) -> ! {
    unsafe {
      fatal_error(b"Thor book move %d is invalid!\x00" as *const u8
                      as *const i8, curr_move);
    }
  }

 fn unrecognized_character(unrecognized: i8) -> ! {
  unsafe {
    fatal_error(b"%s \'%c\' %s\n\x00" as *const u8 as
                    *const i8,
                b"Unrecognized character\x00" as *const u8 as
                    *const i8,
                unrecognized as i32,
                b"in game file\x00" as *const u8 as
                    *const i8);
  }
}

unsafe fn cannot_open_game_file(file_name: *const i8) -> ! {
  fatal_error(b"%s \'%s\'\n\x00" as *const u8 as
                  *const i8,
              b"Cannot open game file\x00" as *const u8 as
                  *const i8, file_name);
}


 fn memory_allocation_failure(block_count_: i32) -> ! {
  unsafe {
    fatal_error(b"%s @ #%d\n\x00" as *const u8 as *const i8,
                b"Memory allocation failure\x00" as *const u8 as
                    *const i8, block_count_);
  }
}

fn invalid_move_in_move_sequence(curr_move: i32) -> ! {
  unsafe {
    fatal_error(b"Invalid move %c%c in move sequence\x00"
                    as *const u8 as *const i8,
                'a' as i32 + curr_move % 10 as i32
                    - 1 as i32,
                '0' as i32 +
                    curr_move / 10 as i32);
  }
}

 fn error_in_map(i: i32, pos: i32, symmetry_map_item: i32) -> ! {
  unsafe {
    fatal_error(b"Error in map %d: inv(map(%d))=%d\n\x00" as
                    *const u8 as *const i8, i, pos, symmetry_map_item);
  }
}

 fn internal_error_in_book_code() -> ! {
    unsafe {
        fatal_error(b"Internal error in book code.\x00" as *const u8 as
            *const i8);
    }
}

 fn book_node_list_allocation_failure(size: i32, to_report: u64) -> ! {
    unsafe {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Book node list: Failed to allocate\x00" as *const u8 as
                        *const i8,
                    to_report,
                    size);
    }
}

 fn book_hash_table_allocaiton_failure(new_size: i32, new_memory: i32) -> ! {
    unsafe {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Book hash table: Failed to allocate\x00" as *const u8 as
                        *const i8, new_memory, new_size);
    }
}

 fn safe_malloc_failure(size: u64) -> ! {
    unsafe {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Memory allocation failure when allocating\x00" as
                        *const u8 as *const i8, size);
    }
}

 fn safe_realloc_failure(size: u64) -> ! {
    unsafe {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Memory allocation failure when allocating\x00" as
                        *const u8 as *const i8, size);
    }
}


 fn error_in_map_thor(i: i32, pos: i32, to_report: i32) -> ! {
    unsafe {
        fatal_error(b"Error in map %d: inv(map(%d))=%d\n\x00" as
                        *const u8 as *const i8, i, pos,
                    to_report);
    }
}

 fn unexpected_character_in_a_move_string() -> ! {
    unsafe {
        fatal_error(b"Unexpected character in move string\x00" as *const u8 as *const i8);
    }
}

 fn invalid_move_string_provided() -> ! {
    unsafe {
        fatal_error(b"Invalid move string provided\x00" as *const u8 as *const i8);
    }
}
}
