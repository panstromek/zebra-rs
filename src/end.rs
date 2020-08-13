pub use engine::src::end::*;
use engine::{
    src:: {
        search::{nodes, get_ponder_move, set_current_eval},
        hash::{HashEntry},
        counter::{counter_value},
        globals::{pv},
    }
};
use crate::{
    src::{
        stubs::{printf, free, fflush, sprintf, puts, stdout},
        display::{display_status, echo, send_status, send_status_time, send_status_pv, send_status_nodes, produce_eval_text, clear_status, display_sweep, send_sweep, clear_sweep},
        timer::{get_elapsed_time},
        zebra::{EvaluationType, _IO_FILE}
    }
};

pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;

#[no_mangle]
pub unsafe extern "C" fn after_update_best_list_verbose(best_list: *mut i32) {
    printf(b"      After:  \x00" as *const u8 as *const i8);
    let mut i = 0 as i32;
    while i < 4 as i32 {
        printf(b"%2d \x00" as *const u8 as *const i8,
               *best_list.offset(i as isize));
        i += 1
    }
    puts(b"\x00" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C"  fn before_update_best_list_verbose(best_list: *mut i32, mut move_0: i32, mut best_list_index: i32, best_list_length: *mut i32) {
    let mut i: i32 = 0;
    printf(b"move=%2d  index=%d  length=%d      \x00" as *const u8 as
               *const i8, move_0, best_list_index,
           *best_list_length);
    printf(b"Before:  \x00" as *const u8 as *const i8);
    i = 0 as i32;
    while i < 4 as i32 {
        printf(b"%2d \x00" as *const u8 as *const i8,
               *best_list.offset(i as isize));
        i += 1
    }
}

static mut buffer: [i8; 16] = [0; 16];

#[no_mangle]
pub unsafe extern "C" fn end_tree_search_output_some_second_stats(alpha: i32, beta: i32, curr_val: i32, update_pv: i32, move_index: i32) {
    if update_pv != 0 {
        end_tree_search_some_pv_stats_report(alpha, beta, curr_val)
    }
    send_sweep(b" \x00" as *const u8 as *const i8);
    if update_pv != 0 && move_index > 0 as i32 && echo != 0 {
        display_sweep(stdout);
    }
}

#[no_mangle]
pub unsafe extern "C" fn end_tree_search_some_pv_stats_report(alpha: i32, beta: i32, curr_val: i32) {
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
        true_found = 1 as i32;
        true_val = curr_val
    }
}

#[no_mangle]
pub unsafe extern "C" fn end_tree_search_level_0_ponder_0_short_report(move_0: i32, first: i32) {
    if first != 0 {
        send_sweep(b"%-10s \x00" as *const u8 as *const i8,
                   buffer.as_mut_ptr());
    }
    send_sweep(b"%c%c\x00" as *const u8 as *const i8,
               'a' as i32 + move_0 % 10 as i32 -
                   1 as i32,
               '0' as i32 + move_0 / 10 as i32);
}

#[no_mangle]
pub unsafe extern "C" fn end_tree_search_output_some_stats(entry: &HashEntry) {
    /* Output some stats */
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

#[no_mangle]
pub unsafe extern "C" fn end_tree_search_level_0_ponder_0_report(alpha: i32, beta: i32, result: i32) {
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

#[no_mangle]
pub unsafe extern "C" fn end_tree_search_level_0_report(alpha: i32, beta: i32) {
    sprintf(buffer.as_mut_ptr(), b"[%d,%d]:\x00" as *const u8 as *const i8, alpha, beta);
    clear_sweep();
}
/*
  SEND_SOLVE_STATUS
  Displays endgame results - partial or full.
*/
#[no_mangle]
pub unsafe extern "C" fn send_solve_status(mut empties: i32,
                                       _side_to_move: i32,
                                       mut eval_info: *mut EvaluationType) {
    let mut eval_str = 0 as *mut i8;
    let mut node_val: f64 = 0.;
    set_current_eval(*eval_info);
    clear_status();
    send_status(b"-->  %2d  \x00" as *const u8 as *const i8,
                empties);
    eval_str = produce_eval_text(&*eval_info, 1 as i32);
    send_status(b"%-10s  \x00" as *const u8 as *const i8, eval_str);
    free(eval_str as *mut std::ffi::c_void);
    node_val = counter_value(&mut nodes);
    send_status_nodes(node_val);
    if get_ponder_move() != 0 {
        send_status(b"{%c%c} \x00" as *const u8 as *const i8,
                    'a' as i32 + get_ponder_move() % 10 as i32 -
                        1 as i32,
                    '0' as i32 + get_ponder_move() / 10 as i32);
    }
    send_status_pv(pv[0 as i32 as usize].as_mut_ptr(), empties);
    send_status_time(get_elapsed_time());
    if get_elapsed_time() > 0.0001f64 {
        send_status(b"%6.0f %s  \x00" as *const u8 as *const i8,
                    node_val / (get_elapsed_time() + 0.0001f64),
                    b"nps\x00" as *const u8 as *const i8);
    };
}

#[no_mangle]
pub unsafe extern "C"  fn end_report_panic_abort_2() {
    printf(b"%s %.1f %c %s\n\x00" as *const u8 as
               *const i8,
           b"Panic abort after\x00" as *const u8 as
               *const i8, get_elapsed_time(),
           's' as i32,
           b"in selective search\x00" as *const u8 as
               *const i8);
}

#[no_mangle]
pub unsafe extern "C" fn end_report_semi_panic_abort_3() {
    printf(b"%s %.1f %c %s\n\x00" as *const u8 as
               *const i8,
           b"Semi-panic abort after\x00" as *const u8 as
               *const i8, get_elapsed_time(),
           's' as i32,
           b"in WLD search\x00" as *const u8 as
               *const i8);
}

#[no_mangle]
pub unsafe extern "C" fn end_report_semi_panic_abort_2() {
    printf(b"%s %.1f %c %s\n\x00" as *const u8 as *const i8,
           b"Semi-panic abort after\x00" as *const u8 as
               *const i8, get_elapsed_time(), 's' as i32,
           b"in exact search\x00" as *const u8 as
               *const i8);
}

#[no_mangle]
pub unsafe extern "C" fn end_report_panic_abort() {
    printf(b"%s %.1f %c %s\n\x00" as *const u8 as
               *const i8,
           b"Panic abort after\x00" as *const u8 as
               *const i8, get_elapsed_time(),
           's' as i32,
           b"in WLD search\x00" as *const u8 as
               *const i8);
}

#[no_mangle]
pub unsafe extern "C" fn end_report_semi_panic_abort() {
    printf(b"%s %.1f %c %s\n\x00" as *const u8 as
               *const i8,
           b"Semi-panic abort after\x00" as *const u8 as
               *const i8, get_elapsed_time(), // FIXME resolve if we should extract this as param??
           's' as i32,
           b"in selective search\x00" as *const u8 as
               *const i8);
}

#[no_mangle]
pub unsafe extern "C" fn end_display_zero_status() {
    display_status(stdout, 0 as i32);
}
