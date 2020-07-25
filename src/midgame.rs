use crate::{
    src::{
        search::{root_eval, force_return, hash_expand_pv, get_ponder_move, nodes, create_eval_info, inherit_move_lists, disc_count, evaluations, evals, sorted_move_order, reorder_move_list},
        timer::{frozen_ponder_depth, extended_above_recommended, above_recommended, is_panic_abort, get_elapsed_time, last_panic_check, check_panic_abort},
        display::{send_status, send_status_time, send_status_pv, send_status_nodes, produce_eval_text, clear_status, display_sweep, echo, send_sweep, clear_sweep, display_buffers},
        counter::{counter_value, adjust_counter},
        stubs::{free, sprintf, abs, stdout},
        libc,
        moves::{valid_move, disks_played, unmake_move, make_move, move_list, move_count, generate_all, unmake_move_no_hash, make_move_no_hash},
        hash::{find_hash, HashEntry, hash_flip_color2, hash2, hash_flip_color1, hash1, add_hash_extended, add_hash},
        globals::{piece_count, board, pv, pv_depth},
        getcoeff::pattern_evaluation,
        eval::terminal_evaluation,
        probcut::mpc_cut,
        myrandom::my_random,
        zebra::{EvaluationType}
    }
};
pub use engine::src::midgame::*;

#[no_mangle]
pub unsafe extern "C" fn midgame_display_simple_ponder_move(move_0: i32) {
    send_sweep(b"%c%c\x00" as *const u8 as *const i8,
               'a' as i32 + move_0 % 10 as i32 -
                   1 as i32,
               '0' as i32 + move_0 / 10 as i32);
}

#[no_mangle]
pub unsafe extern "C" fn midgame_display_initial_ponder_move(alpha: i32, beta: i32, buffer: &mut [i8; 32]) {

    if alpha <= -(29000 as i32) && beta >= 29000 as i32 {
        sprintf(buffer.as_mut_ptr(),
                b"[-inf,inf]:\x00" as *const u8 as *const i8);
    } else if alpha <= -(29000 as i32) &&
        beta < 29000 as i32 {
        sprintf(buffer.as_mut_ptr(),
                b"[-inf,%.1f]:\x00" as *const u8 as *const i8,
                beta as f64 / 128.0f64);
    } else if alpha > -(29000 as i32) &&
        beta >= 29000 as i32 {
        sprintf(buffer.as_mut_ptr(),
                b"[%.1f,inf]:\x00" as *const u8 as *const i8,
                alpha as f64 / 128.0f64);
    } else {
        sprintf(buffer.as_mut_ptr(),
                b"[%.1f,%.1f]:\x00" as *const u8 as *const i8,
                alpha as f64 / 128.0f64,
                beta as f64 / 128.0f64);
    }
    clear_sweep();
    send_sweep(b"%-14s \x00" as *const u8 as *const i8,
               buffer.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn midgame_display_ponder_move(
    max_depth: i32, alpha: i32, beta:  i32,
    curr_val: i32, searched:  i32, update_pv:  i32) {

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

#[no_mangle]
pub unsafe extern "C" fn midgame_display_status(mut side_to_move: i32, mut max_depth: i32, eval_info: *mut EvaluationType, mut eval_str: *mut i8, mut node_val: f64, mut depth: i32) {
    clear_status();
    send_status(b"--> \x00" as *const u8 as *const i8);
    if is_panic_abort() != 0 || force_return != 0 {
        send_status(b"*\x00" as *const u8 as *const i8);
    } else {
        send_status(b" \x00" as *const u8 as *const i8);
    }
    send_status(b"%2d  \x00" as *const u8 as *const i8,
                depth);
    eval_str = produce_eval_text(*eval_info, 1 as i32);
    send_status(b"%-10s  \x00" as *const u8 as *const i8,
                eval_str);
    free(eval_str as *mut libc::c_void);
    node_val = counter_value(&mut nodes);
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
    send_status_pv(pv[0 as i32 as usize].as_mut_ptr(),
                   max_depth);
    send_status_time(get_elapsed_time());
    if get_elapsed_time() != 0.0f64 {
        send_status(b"%6.0f %s\x00" as *const u8 as
                        *const i8,
                    node_val / (get_elapsed_time() + 0.001f64),
                    b"nps\x00" as *const u8 as *const i8);
    }
}
