use crate::{
    src::{
        search::{root_eval, force_return, hash_expand_pv, get_ponder_move, nodes, create_eval_info, inherit_move_lists, disc_count, evaluations, evals, sorted_move_order, reorder_move_list},
        counter::{counter_value, adjust_counter},
        moves::{valid_move, disks_played, unmake_move, make_move, move_list, move_count, generate_all, unmake_move_no_hash, make_move_no_hash},
        hash::{find_hash, HashEntry, hash_flip_color2, hash2, hash_flip_color1, hash1, add_hash_extended},
        globals::{piece_count, board, pv, pv_depth},
        eval::terminal_evaluation,
        probcut::mpc_cut,
        myrandom::my_random,
        zebra::{EvaluationType}
    }
};

pub type EvalType = u32;
pub const UNINITIALIZED_EVAL: EvalType = 8;
pub const INTERRUPTED_EVAL: EvalType = 7;
pub const UNDEFINED_EVAL: EvalType = 6;
pub const PASS_EVAL: EvalType = 5;
pub const FORCED_EVAL: EvalType = 4;
pub const SELECTIVE_EVAL: EvalType = 3;
pub const WLD_EVAL: EvalType = 2;
pub const EXACT_EVAL: EvalType = 1;
pub const MIDGAME_EVAL: EvalType = 0;
pub type EvalResult = u32;
pub const UNSOLVED_POSITION: EvalResult = 3;
pub const LOST_POSITION: EvalResult = 2;
pub const DRAWN_POSITION: EvalResult = 1;
pub const WON_POSITION: EvalResult = 0;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct DepthInfo {
    pub cut_tries: i32,
    pub cut_depth: [i32; 2],
    pub bias: [[i32; 61]; 2],
    pub window: [[i32; 61]; 2],
}


/* Default aspiration window parameters. These values are currently
   really huge as usage of a small windows tends to slow down
   the search. */
pub static mut allow_midgame_hash_probe: i32 = 0;
pub static mut allow_midgame_hash_update: i32 = 0;
pub static mut best_mid_move: i32 = 0;
pub static mut best_mid_root_move: i32 = 0;
pub static mut midgame_abort: i32 = 0;
pub static mut do_check_midgame_abort: i32 = 1 as i32;
pub static mut counter_phase: i32 = 0;
pub static mut apply_perturbation: i32 = 1 as i32;
pub static mut perturbation_amplitude: i32 = 0 as i32;
pub static mut stage_reached: [i32; 61] = [0; 61];
pub static mut stage_score: [i32; 61] = [0; 61];
pub static mut score_perturbation: [i32; 100] = [0; 100];
pub static mut feas_index_list: [[i32; 64]; 64] = [[0; 64]; 64];


/*
  CLEAR_MIDGAME_ABORT
  IS_MIDGAME_ABORT
  SET_MIDGAME_ABORT
  TOGGLE_MIDGAME_ABORT_CHECK
  These functions handle the midgame abort system which kicks in
  when it is estimated that the next iteration in the iterative
  deepening would take too long.
*/

pub unsafe fn clear_midgame_abort() {
    midgame_abort = 0 as i32;
}

pub unsafe fn is_midgame_abort() -> i32 {
    return midgame_abort;
}

pub unsafe fn set_midgame_abort() {
    midgame_abort = do_check_midgame_abort;
}

pub unsafe fn toggle_midgame_abort_check(mut toggle: i32) {
    do_check_midgame_abort = toggle;
}
/*
   TOGGLE_MIDGAME_HASH_USAGE
   Toggles hash table access in the midgame search on/off.
*/

pub unsafe fn toggle_midgame_hash_usage(mut allow_read:
                                        i32,
                                        mut allow_write:
                                        i32) {
    allow_midgame_hash_probe = allow_read;
    allow_midgame_hash_update = allow_write;
}

/*
  SET_PERTURBATION
  Set the amplitude of the score perturbation applied by
  CALCULATE_PERTURBATION.
*/

pub unsafe fn set_perturbation(mut amplitude: i32) {
    perturbation_amplitude = amplitude;
}
/*
  TOGGLE_PERTURBATION_USAGE
  Toggle usage of score perturbations on/off.
*/

pub unsafe fn toggle_perturbation_usage(mut toggle: i32) {
    apply_perturbation = toggle;
}

/*
  ADVANCE_MOVE
  Swaps a move and its predecessor in the move list if it's
  not already first in the list.
*/
pub unsafe fn advance_move(mut index: i32) {
    let mut temp_move: i32 = 0;
    if index > 0 as i32 {
        temp_move = sorted_move_order[disks_played as usize][index as usize];
        sorted_move_order[disks_played as usize][index as usize] =
            sorted_move_order[disks_played as
                usize][(index - 1 as i32) as usize];
        sorted_move_order[disks_played as
            usize][(index - 1 as i32) as usize] =
            temp_move
    };
}
/*
  midgame_c__update_best_list
*/
pub unsafe fn midgame_c__update_best_list(mut best_list:
                                      *mut i32,
                                      mut move_0: i32,
                                      mut best_list_index:
                                      i32,
                                      mut best_list_length:
                                      i32) {
    let mut i: i32 = 0;
    if best_list_index < best_list_length {
        i = best_list_index;
        while i >= 1 as i32 {
            *best_list.offset(i as isize) =
                *best_list.offset((i - 1 as i32) as isize);
            i -= 1
        }
    } else {
        i = 3 as i32;
        while i >= 1 as i32 {
            *best_list.offset(i as isize) =
                *best_list.offset((i - 1 as i32) as isize);
            i -= 1
        }
    }
    *best_list.offset(0 as i32 as isize) = move_0;
}
