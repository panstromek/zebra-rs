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
use crate::src::getcoeff::pattern_evaluation;
use crate::src::stubs::abs;
use crate::src::timer::{is_panic_abort, last_panic_check, check_panic_abort};
use crate::src::hash::add_hash;
use crate::src::display::{echo, display_buffers};

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


/*
  STATIC_OR_TERMINAL_EVALUATION
  Invokes the proper evaluation function depending on whether the
  board is filled or not.
*/
pub unsafe fn static_or_terminal_evaluation(mut side_to_move:
                                            i32)
                                            -> i32 {
    if disks_played == 60 as i32 {
        return terminal_evaluation(side_to_move)
    } else {
        evaluations.lo = evaluations.lo.wrapping_add(1);
        return pattern_evaluation(side_to_move)
    };
}

/*
   SETUP_MIDGAME
   Sets up some search parameters.
*/

pub unsafe fn setup_midgame() {
    let mut i: i32 = 0;
    allow_midgame_hash_probe = 1 as i32;
    allow_midgame_hash_update = 1 as i32;
    i = 0 as i32;
    while i <= 60 as i32 {
        stage_reached[i as usize] = 0 as i32;
        i += 1
    }
    calculate_perturbation();
}
/*
  CALCULATE_PERTURBATION
  Determines the score perturbations (if any) to the root moves.
*/

pub unsafe fn calculate_perturbation() {
    let mut i: i32 = 0;
    let mut shift: i32 = 0;
    if apply_perturbation == 0 || perturbation_amplitude == 0 as i32 {
        i = 0 as i32;
        while i < 100 as i32 {
            score_perturbation[i as usize] = 0 as i32;
            i += 1
        }
    } else {
        shift = perturbation_amplitude / 2 as i32;
        i = 0 as i32;
        while i < 100 as i32 {
            score_perturbation[i as usize] =
                abs(my_random() as i32) % perturbation_amplitude -
                    shift;
            i += 1
        }
    };
}
/*
  PROTECTED_ONE_PLY_SEARCH
  Chooses the move maximizing the static evaluation function
  while avoiding all moves which allow an immediate loss
  (if that is possible).
*/
pub unsafe fn protected_one_ply_search(mut side_to_move: i32)
                                   -> i32 {
    let mut i: i32 = 0;
    let mut move_0: i32 = 0;
    let mut depth_one_score: i32 = 0;
    let mut depth_two_score: i32 = 0;
    let mut best_score_restricted: i32 = 0;
    let mut best_score_unrestricted: i32 = 0;
    let mut best_move_restricted: i32 = 0;
    let mut best_move_unrestricted: i32 = 0;
    generate_all(side_to_move);
    best_score_restricted = -(12345678 as i32);
    best_score_unrestricted = -(12345678 as i32);
    best_move_restricted = 0 as i32;
    best_move_unrestricted = 0 as i32;
    i = 0 as i32;
    while i < move_count[disks_played as usize] {
        nodes.lo = nodes.lo.wrapping_add(1);
        move_0 = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, move_0, 1 as i32);
        evaluations.lo = evaluations.lo.wrapping_add(1);
        depth_one_score =
            -pattern_evaluation(0 as i32 + 2 as i32 -
                side_to_move);
        depth_two_score =
            -tree_search(1 as i32, 2 as i32,
                         0 as i32 + 2 as i32 - side_to_move,
                         -(12345678 as i32), 12345678 as i32,
                         0 as i32, 0 as i32,
                         0 as i32);
        unmake_move(side_to_move, move_0);
        if depth_one_score > best_score_unrestricted {
            best_score_unrestricted = depth_one_score;
            best_move_unrestricted = move_0
        }
        if depth_two_score > -(29000 as i32) &&
            depth_one_score > best_score_restricted {
            best_score_restricted = depth_one_score;
            best_move_restricted = move_0
        }
        i += 1
    }
    pv_depth[0 as i32 as usize] = 1 as i32;
    if best_score_restricted > -(12345678 as i32) {
        /* No immediate loss */
        pv[0 as i32 as usize][0 as i32 as usize] =
            best_move_restricted;
        return best_score_restricted
    } else {
        pv[0 as i32 as usize][0 as i32 as usize] =
            best_move_unrestricted;
        return best_score_unrestricted
    };
}

/*
   TREE_SEARCH
   The recursive tree search function. It uses negascout for
   tree pruning.
*/

pub unsafe fn tree_search(mut level: i32,
                          mut max_depth: i32,
                          mut side_to_move: i32,
                          mut alpha: i32,
                          mut beta: i32,
                          mut allow_hash: i32,
                          mut allow_mpc: i32,
                          mut void_legal: i32)
                          -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut curr_val: i32 = 0;
    let mut best: i32 = 0;
    let mut pre_best: i32 = 0;
    let mut searched: i32 = 0;
    let mut move_0: i32 = 0;
    let mut hash_move: i32 = 0;
    let mut move_index: i32 = 0;
    let mut best_move_index: i32 = 0;
    let mut empties_remaining: i32 = 0;
    let mut hash_hit: i32 = 0;
    let mut pre_depth: i32 = 0;
    let mut update_pv: i32 = 0;
    let mut remains: i32 = 0;
    let mut shallow_remains: i32 = 0;
    let mut use_hash: i32 = 0;
    let mut pre_search_done: i32 = 0;
    let mut curr_alpha: i32 = 0;
    let mut best_index: i32 = 0;
    let mut best_score: i32 = 0;
    let mut best_list_index: i32 = 0;
    let mut best_list_length: i32 = 0;
    let mut selectivity: i32 = 0;
    let mut cut: i32 = 0;
    let mut best_list: [i32; 4] = [0; 4];
    let mut entry =
        HashEntry{key1: 0,
            key2: 0,
            eval: 0,
            move_0: [0; 4],
            draft: 0,
            selectivity: 0,
            flags: 0,};
    if level >= max_depth {
        nodes.lo = nodes.lo.wrapping_add(1);
        return static_or_terminal_evaluation(side_to_move)
    }
    remains = max_depth - level;
    if remains < 3 as i32 {
        curr_val =
            fast_tree_search(level, max_depth, side_to_move, alpha, beta,
                             allow_hash, void_legal);
        pv_depth[level as usize] = level + 1 as i32;
        pv[level as usize][level as usize] = best_mid_move;
        return curr_val
    }
    nodes.lo = nodes.lo.wrapping_add(1);
    /* Check the hash table */
    use_hash =
        (remains >= 2 as i32 && 1 as i32 != 0 &&
            allow_hash != 0) as i32;
    if 1 as i32 != 0 && allow_mpc != 0 {
        selectivity = 1 as i32
    } else { selectivity = 0 as i32 }
    if use_hash != 0 && allow_midgame_hash_probe != 0 {
        find_hash(&mut entry, 0 as i32);
        if entry.draft as i32 >= remains &&
            entry.selectivity as i32 <= selectivity &&
            valid_move(entry.move_0[0 as i32 as usize],
                       side_to_move) != 0 &&
            entry.flags as i32 & 8 as i32 != 0 &&
            (entry.flags as i32 & 4 as i32 != 0 ||
                entry.flags as i32 & 1 as i32 != 0 &&
                    entry.eval >= beta ||
                entry.flags as i32 & 2 as i32 != 0 &&
                    entry.eval <= alpha) {
            pv[level as usize][level as usize] =
                entry.move_0[0 as i32 as usize];
            pv_depth[level as usize] = level + 1 as i32;
            return entry.eval
        }
    }
    hash_hit =
        (use_hash != 0 && allow_midgame_hash_probe != 0) as i32;
    if hash_hit != 0 {
        hash_move = entry.move_0[0 as i32 as usize]
    } else { hash_move = 44 as i32 }
    pre_search_done = 0 as i32;
    /* Use multi-prob-cut to selectively prune the tree */
    if 1 as i32 != 0 && allow_mpc != 0 && remains <= 22 as i32
    {
        let mut alpha_test = 1 as i32;
        let mut beta_test = 1 as i32;
        cut = 0 as i32;
        while cut < mpc_cut[remains as usize].cut_tries {
            /* Determine the fail-high and fail-low bounds */
            let mut bias =
                mpc_cut[remains as
                    usize].bias[cut as usize][disks_played as usize];
            let mut window =
                mpc_cut[remains as
                    usize].window[cut as
                    usize][disks_played as usize];
            let mut alpha_bound = alpha + bias - window;
            let mut beta_bound = beta + bias + window;
            /* Don't use an MPC cut which results in the full-width depth
            being less than some predefined constant */
            shallow_remains =
                mpc_cut[remains as usize].cut_depth[cut as usize];
            if !(level + shallow_remains < 8 as i32) {
                if shallow_remains > 1 as i32 {
                    /* "Deep" shallow search */
                    if cut == 0 as i32 {
                        /* Use static eval to decide if a one- or two-sided
                       MPC test is to be performed. */
                        evaluations.lo = evaluations.lo.wrapping_add(1);
                        let mut static_eval =
                            pattern_evaluation(side_to_move);
                        if static_eval <= alpha_bound {
                            beta_test = 0 as i32
                        } else if static_eval >= beta_bound {
                            alpha_test = 0 as i32
                        }
                    }
                    assert!(alpha_test != 0 || beta_test != 0);
                    if alpha_test != 0 && beta_test != 0 {
                        /* Test for likely fail-low or likely fail-high. */
                        let mut shallow_val =
                            tree_search(level, level + shallow_remains,
                                        side_to_move, alpha_bound, beta_bound,
                                        allow_hash, 0 as i32,
                                        void_legal);
                        if shallow_val >= beta_bound {
                            if use_hash != 0 && allow_midgame_hash_update != 0
                            {
                                add_hash(0 as i32, beta,
                                         pv[level as usize][level as usize],
                                         8 as i32 | 1 as i32,
                                         remains, selectivity);
                            }
                            return beta
                        } else if shallow_val <= alpha_bound {
                            if use_hash != 0 && allow_midgame_hash_update != 0
                            {
                                add_hash(0 as i32, alpha,
                                         pv[level as usize][level as usize],
                                         8 as i32 | 2 as i32,
                                         remains, selectivity);
                            }
                            return alpha
                        } else {
                            /* Use information learned from the failed cut test to decide
                           if a one or a two-sided test is to be performed next. */
                            let mut mid =
                                (alpha_bound + beta_bound) / 2 as i32;
                            let mut low_threshold =
                                (2 as i32 * mid + alpha_bound) /
                                    3 as i32;
                            let mut high_threshold =
                                (2 as i32 * mid + beta_bound) /
                                    3 as i32;
                            if shallow_val <= low_threshold {
                                beta_test = 0 as i32
                            } else {
                                if !(shallow_val >= high_threshold) {
                                    break ;
                                }
                                alpha_test = 0 as i32
                            }
                            /* Unlikely that there is any selective cutoff. */
                        }
                    } else if beta_test != 0 {
                        /* Fail-high with high probability? */
                        if tree_search(level, level + shallow_remains,
                                       side_to_move,
                                       beta_bound - 1 as i32,
                                       beta_bound, allow_hash,
                                       0 as i32, void_legal) >=
                            beta_bound {
                            if use_hash != 0 && allow_midgame_hash_update != 0
                            {
                                add_hash(0 as i32, beta,
                                         pv[level as usize][level as usize],
                                         8 as i32 | 1 as i32,
                                         remains, selectivity);
                            }
                            return beta
                        }
                    } else if alpha_test != 0 {
                        /* Fail-low with high probability? */
                        if tree_search(level, level + shallow_remains,
                                       side_to_move, alpha_bound,
                                       alpha_bound + 1 as i32,
                                       allow_hash, 0 as i32,
                                       void_legal) <= alpha_bound {
                            if use_hash != 0 && allow_midgame_hash_update != 0
                            {
                                add_hash(0 as i32, alpha,
                                         pv[level as usize][level as usize],
                                         8 as i32 | 2 as i32,
                                         remains, selectivity);
                            }
                            return alpha
                        }
                    }
                } else {
                    /* All-in-one MPC one-ply search and move ordering */
                    move_count[disks_played as usize] = 0 as i32;
                    best = alpha_bound;
                    empties_remaining = 60 as i32 - disks_played;
                    move_index = 0 as i32;
                    while move_index < 60 as i32 {
                        move_0 =
                            sorted_move_order[disks_played as
                                usize][move_index as usize];
                        if board[move_0 as usize] == 1 as i32 {
                            if make_move_no_hash(side_to_move, move_0) !=
                                0 as i32 {
                                curr_val =
                                    -static_or_terminal_evaluation(0 as
                                        i32
                                        +
                                        2 as
                                            i32
                                        -
                                        side_to_move);
                                unmake_move_no_hash(side_to_move, move_0);
                                nodes.lo = nodes.lo.wrapping_add(1);
                                if curr_val > best {
                                    best = curr_val;
                                    if best >= beta_bound {
                                        if use_hash != 0 &&
                                            allow_midgame_hash_update != 0
                                        {
                                            add_hash(0 as i32, beta,
                                                     pv[level as
                                                         usize][level as
                                                         usize],
                                                     8 as i32 |
                                                         1 as i32,
                                                     remains, selectivity);
                                        }
                                        return beta
                                    }
                                }
                                evals[disks_played as usize][move_0 as usize]
                                    = curr_val;
                                if move_0 == hash_move {
                                    /* Always try hash table move first */
                                    evals[disks_played as
                                        usize][move_0 as usize] +=
                                        10000 as i32
                                }
                                feas_index_list[disks_played as
                                    usize][move_count[disks_played
                                    as
                                    usize]
                                    as usize] =
                                    move_index;
                                move_count[disks_played as usize] += 1
                            }
                            empties_remaining -= 1;
                            if empties_remaining == 0 as i32 {
                                break ;
                            }
                        }
                        move_index += 1
                    }
                    if best == alpha_bound &&
                        move_count[disks_played as usize] >
                            0 as i32 {
                        if use_hash != 0 && allow_midgame_hash_update != 0 {
                            add_hash(0 as i32, alpha,
                                     pv[level as usize][level as usize],
                                     8 as i32 | 2 as i32,
                                     remains, selectivity);
                        }
                        return alpha
                    }
                    pre_search_done = 1 as i32
                }
            }
            cut += 1
        }
    }
    /* Full negascout search */
    searched = 0 as i32;
    best = -(12345678 as i32);
    best_move_index = -(1 as i32);
    curr_alpha = alpha;
    best_list_length = 0 as i32;
    i = 0 as i32;
    while i < 4 as i32 {
        best_list[i as usize] = 0 as i32;
        i += 1
    }
    if pre_search_done == 0 {
        move_count[disks_played as usize] = 0 as i32;
        if hash_hit != 0 {
            i = 0 as i32;
            while i < 4 as i32 {
                if valid_move(entry.move_0[i as usize], side_to_move) != 0 {
                    let fresh0 = best_list_length;
                    best_list_length = best_list_length + 1;
                    best_list[fresh0 as usize] = entry.move_0[i as usize]
                }
                i += 1
            }
        }
    }
    i = 0 as i32;
    best_list_index = 0 as i32;
    loop
    /* Try the hash table move(s) first if feasible */
    {
        if pre_search_done == 0 && best_list_index < best_list_length {
            move_count[disks_played as usize] += 1;
            move_index = 0 as i32;
            while sorted_move_order[disks_played as
                usize][move_index as usize] !=
                best_list[best_list_index as usize] {
                move_index += 1
            }
        } else {
            /* Otherwise use information from shallow searches */
            if pre_search_done == 0 {
                if remains < 10 as i32 {
                    pre_depth = 1 as i32
                } else { pre_depth = 2 as i32 }
                pre_best = -(12345678 as i32);
                empties_remaining = 60 as i32 - disks_played;
                move_index = 0 as i32;
                while move_index < 60 as i32 {
                    let mut already_checked: i32 = 0;
                    move_0 =
                        sorted_move_order[disks_played as
                            usize][move_index as usize];
                    already_checked = 0 as i32;
                    j = 0 as i32;
                    while j < best_list_length {
                        if move_0 == best_list[j as usize] {
                            already_checked = 1 as i32
                        }
                        j += 1
                    }
                    if board[move_0 as usize] == 1 as i32 {
                        if already_checked == 0 &&
                            make_move(side_to_move, move_0,
                                      1 as i32) != 0 as i32
                        {
                            curr_val =
                                -tree_search(level + 1 as i32,
                                             level + pre_depth,
                                             0 as i32 +
                                                 2 as i32 -
                                                 side_to_move,
                                             -(12345678 as i32),
                                             -pre_best, 0 as i32,
                                             0 as i32,
                                             1 as i32);
                            pre_best =
                                if pre_best > curr_val {
                                    pre_best
                                } else { curr_val };
                            unmake_move(side_to_move, move_0);
                            evals[disks_played as usize][move_0 as usize] =
                                curr_val;
                            feas_index_list[disks_played as
                                usize][move_count[disks_played
                                as
                                usize]
                                as usize] =
                                move_index;
                            move_count[disks_played as usize] += 1
                        }
                        empties_remaining -= 1;
                        if empties_remaining == 0 as i32 { break ; }
                    }
                    move_index += 1
                }
                pre_search_done = 1 as i32
            }
            if i == move_count[disks_played as usize] { break ; }
            best_index = i;
            best_score =
                evals[disks_played as
                    usize][sorted_move_order[disks_played as
                    usize][feas_index_list[disks_played
                    as
                    usize][i
                    as
                    usize]
                    as usize] as
                    usize];
            j = i + 1 as i32;
            while j < move_count[disks_played as usize] {
                let mut cand_move: i32 = 0;
                cand_move =
                    sorted_move_order[disks_played as
                        usize][feas_index_list[disks_played
                        as
                        usize][j
                        as
                        usize]
                        as usize];
                if evals[disks_played as usize][cand_move as usize] >
                    best_score {
                    best_score =
                        evals[disks_played as usize][cand_move as usize];
                    best_index = j
                }
                j += 1
            }
            move_index =
                feas_index_list[disks_played as usize][best_index as usize];
            feas_index_list[disks_played as usize][best_index as usize] =
                feas_index_list[disks_played as usize][i as usize]
        }
        move_0 =
            sorted_move_order[disks_played as usize][move_index as usize];
        counter_phase = counter_phase + 1 as i32 & 63 as i32;
        if counter_phase == 0 as i32 {
            let mut node_val: f64 = 0.;
            adjust_counter(&mut nodes);
            node_val = counter_value(&mut nodes);
            if node_val - last_panic_check >=
                100000 as i32 as f64 {
                /* Time abort? */
                last_panic_check = node_val;
                check_panic_abort();
                /* Display available search information */
                if echo != 0 { display_buffers(); }
                /* Check for events */
                if is_panic_abort() != 0 || force_return != 0 {
                    return -(27000 as i32)
                }
            }
        }
        make_move(side_to_move, move_0, 1 as i32);
        update_pv = 0 as i32;
        if searched == 0 as i32 {
            curr_val =
                -tree_search(level + 1 as i32, max_depth,
                             0 as i32 + 2 as i32 -
                                 side_to_move, -beta, -curr_alpha, allow_hash,
                             allow_mpc, 1 as i32);
            best = curr_val;
            best_move_index = move_index;
            update_pv = 1 as i32
        } else {
            curr_alpha = if best > curr_alpha { best } else { curr_alpha };
            curr_val =
                -tree_search(level + 1 as i32, max_depth,
                             0 as i32 + 2 as i32 -
                                 side_to_move,
                             -(curr_alpha + 1 as i32), -curr_alpha,
                             allow_hash, allow_mpc, 1 as i32);
            if curr_val > curr_alpha && curr_val < beta {
                curr_val =
                    -tree_search(level + 1 as i32, max_depth,
                                 0 as i32 + 2 as i32 -
                                     side_to_move, -beta,
                                 12345678 as i32, allow_hash,
                                 allow_mpc, 1 as i32);
                if curr_val > best {
                    best = curr_val;
                    best_move_index = move_index;
                    update_pv = 1 as i32
                }
            } else if curr_val > best {
                best = curr_val;
                best_move_index = move_index;
                update_pv = 1 as i32
            }
        }
        unmake_move(side_to_move, move_0);
        if is_panic_abort() != 0 || force_return != 0 {
            return -(27000 as i32)
        }
        evals[disks_played as usize][move_0 as usize] = curr_val;
        if update_pv != 0 {
            midgame_c__update_best_list(best_list.as_mut_ptr(), move_0,
                                        best_list_index, best_list_length);
            pv[level as usize][level as usize] = move_0;
            pv_depth[level as usize] =
                pv_depth[(level + 1 as i32) as usize];
            j = level + 1 as i32;
            while j < pv_depth[(level + 1 as i32) as usize] {
                pv[level as usize][j as usize] =
                    pv[(level + 1 as i32) as usize][j as usize];
                j += 1
            }
        }
        if best >= beta {
            advance_move(move_index);
            if use_hash != 0 && allow_midgame_hash_update != 0 {
                add_hash_extended(0 as i32, best,
                                  best_list.as_mut_ptr(),
                                  8 as i32 | 1 as i32,
                                  remains, selectivity);
            }
            return best
        }
        searched += 1;
        i += 1;
        best_list_index += 1
    }
    /* Post-processing */
    if move_count[disks_played as usize] > 0 as i32 {
        advance_move(best_move_index);
        if use_hash != 0 && allow_midgame_hash_update != 0 {
            if best > alpha {
                add_hash_extended(0 as i32, best,
                                  best_list.as_mut_ptr(),
                                  8 as i32 | 4 as i32,
                                  remains, selectivity);
            } else {
                add_hash_extended(0 as i32, best,
                                  best_list.as_mut_ptr(),
                                  8 as i32 | 2 as i32,
                                  remains, selectivity);
            }
        }
        return best
    } else if void_legal != 0 {
        /* No feasible moves */
        hash1 ^= hash_flip_color1;
        hash2 ^= hash_flip_color2;
        curr_val =
            -tree_search(level, max_depth,
                         0 as i32 + 2 as i32 - side_to_move,
                         -beta, -alpha, allow_hash, allow_mpc,
                         0 as i32);
        hash1 ^= hash_flip_color1;
        hash2 ^= hash_flip_color2;
        return curr_val
    } else {
        pv_depth[level as usize] = level;
        return terminal_evaluation(side_to_move)
    };
}

/*
   FAST_TREE_SEARCH
   The recursive tree search function. It uses negascout for
   tree pruning.
*/
unsafe fn fast_tree_search(mut level: i32,
                           mut max_depth: i32,
                           mut side_to_move: i32,
                           mut alpha: i32,
                           mut beta: i32,
                           mut allow_hash: i32,
                           mut void_legal: i32)
                           -> i32 {
    let mut curr_val: i32 = 0;
    let mut best: i32 = 0;
    let mut move_index: i32 = 0;
    let mut move_0: i32 = 0;
    let mut best_move_index: i32 = 0;
    let mut best_move: i32 = 0;
    let mut first: i32 = 0;
    let mut remains: i32 = 0;
    let mut use_hash: i32 = 0;
    let mut new_use_hash: i32 = 0;
    let mut curr_alpha: i32 = 0;
    let mut empties_remaining: i32 = 0;
    let mut entry =
        HashEntry{key1: 0,
            key2: 0,
            eval: 0,
            move_0: [0; 4],
            draft: 0,
            selectivity: 0,
            flags: 0,};
    nodes.lo = nodes.lo.wrapping_add(1);
    if level >= max_depth {
        return static_or_terminal_evaluation(side_to_move)
    }
    /* Check the hash table */
    remains = max_depth - level;
    use_hash =
        (remains >= 2 as i32 && 1 as i32 != 0 &&
            allow_hash != 0) as i32;
    if use_hash != 0 && allow_midgame_hash_probe != 0 {
        find_hash(&mut entry, 0 as i32);
        if entry.draft as i32 >= remains &&
            entry.selectivity as i32 == 0 as i32 &&
            valid_move(entry.move_0[0 as i32 as usize],
                       side_to_move) != 0 &&
            entry.flags as i32 & 8 as i32 != 0 &&
            (entry.flags as i32 & 4 as i32 != 0 ||
                entry.flags as i32 & 1 as i32 != 0 &&
                    entry.eval >= beta ||
                entry.flags as i32 & 2 as i32 != 0 &&
                    entry.eval <= alpha) {
            best_mid_move = entry.move_0[0 as i32 as usize];
            return entry.eval
        }
    }
    /* Reorder the move lists now and then to keep the empty squares up front */
    if nodes.lo & 4095 as i32 as u32 ==
        0 as i32 as u32 {
        reorder_move_list(disks_played);
    }
    /* Search */
    first = 1 as i32;
    best_move = -(1 as i32);
    best_move_index = -(1 as i32);
    best = -(12345678 as i32);
    if remains == 1 as i32 {
        /* Plain alpha-beta last ply */
        empties_remaining = 60 as i32 - disks_played;
        move_index = 0 as i32;
        while move_index < 60 as i32 {
            move_0 =
                sorted_move_order[disks_played as usize][move_index as usize];
            if board[move_0 as usize] == 1 as i32 {
                if make_move_no_hash(side_to_move, move_0) != 0 as i32
                {
                    curr_val =
                        -static_or_terminal_evaluation(0 as i32 +
                            2 as i32 -
                            side_to_move);
                    unmake_move_no_hash(side_to_move, move_0);
                    nodes.lo = nodes.lo.wrapping_add(1);
                    if curr_val > best {
                        best = curr_val;
                        best_move_index = move_index;
                        best_move = move_0;
                        if curr_val >= beta {
                            advance_move(move_index);
                            best_mid_move = best_move;
                            if use_hash != 0 && allow_midgame_hash_update != 0
                            {
                                add_hash(0 as i32, best, best_move,
                                         8 as i32 | 1 as i32,
                                         remains, 0 as i32);
                            }
                            return best
                        }
                    }
                    first = 0 as i32
                }
                empties_remaining -= 1;
                if empties_remaining == 0 as i32 { break ; }
            }
            move_index += 1
        }
    } else {
        /* Principal variation search for deeper searches */
        new_use_hash =
            (remains >= 2 as i32 + 1 as i32 && use_hash != 0)
                as i32;
        curr_alpha = alpha;
        empties_remaining = 60 as i32 - disks_played;
        move_index = 0 as i32;
        while move_index < 60 as i32 {
            move_0 =
                sorted_move_order[disks_played as usize][move_index as usize];
            if board[move_0 as usize] == 1 as i32 {
                if make_move(side_to_move, move_0, new_use_hash) !=
                    0 as i32 {
                    if first != 0 {
                        curr_val =
                            -fast_tree_search(level + 1 as i32,
                                              max_depth,
                                              0 as i32 +
                                                  2 as i32 -
                                                  side_to_move, -beta,
                                              -curr_alpha, allow_hash,
                                              1 as i32);
                        best = curr_val;
                        best_move = move_0;
                        best_move_index = move_index
                    } else {
                        curr_alpha =
                            if best > curr_alpha { best } else { curr_alpha };
                        curr_val =
                            -fast_tree_search(level + 1 as i32,
                                              max_depth,
                                              0 as i32 +
                                                  2 as i32 -
                                                  side_to_move,
                                              -(curr_alpha +
                                                  1 as i32),
                                              -curr_alpha, allow_hash,
                                              1 as i32);
                        if curr_val > curr_alpha && curr_val < beta {
                            curr_val =
                                -fast_tree_search(level + 1 as i32,
                                                  max_depth,
                                                  0 as i32 +
                                                      2 as i32 -
                                                      side_to_move, -beta,
                                                  12345678 as i32,
                                                  allow_hash,
                                                  1 as i32)
                        }
                        if curr_val > best {
                            best_move = move_0;
                            best_move_index = move_index;
                            best = curr_val
                        }
                    }
                    unmake_move(side_to_move, move_0);
                    if best >= beta {
                        advance_move(move_index);
                        best_mid_move = best_move;
                        if use_hash != 0 && allow_midgame_hash_update != 0 {
                            add_hash(0 as i32, best, best_move,
                                     8 as i32 | 1 as i32,
                                     remains, 0 as i32);
                        }
                        return best
                    }
                    first = 0 as i32
                }
                empties_remaining -= 1;
                if empties_remaining == 0 as i32 { break ; }
            }
            move_index += 1
        }
    }
    if first == 0 {
        advance_move(best_move_index);
        best_mid_move = best_move;
        if use_hash != 0 && allow_midgame_hash_update != 0 {
            if best > alpha {
                add_hash(0 as i32, best, best_move,
                         8 as i32 | 4 as i32, remains,
                         0 as i32);
            } else {
                add_hash(0 as i32, best, best_move,
                         8 as i32 | 2 as i32, remains,
                         0 as i32);
            }
        }
        return best
    } else if void_legal != 0 {
        /* I pass, other player's turn now */
        hash1 ^= hash_flip_color1;
        hash2 ^= hash_flip_color2;
        curr_val =
            -fast_tree_search(level, max_depth,
                              0 as i32 + 2 as i32 -
                                  side_to_move, -beta, -alpha, allow_hash,
                              0 as i32);
        hash1 ^= hash_flip_color1;
        hash2 ^= hash_flip_color2;
        return curr_val
    } else {
        /* Both players had to pass ==> evaluate board as final */
        curr_val = terminal_evaluation(side_to_move);
        return curr_val
    };
}