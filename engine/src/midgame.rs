use crate::{
    src::{
        search::{search_state, force_return, hash_expand_pv, get_ponder_move, create_eval_info, inherit_move_lists, disc_count, reorder_move_list},
        counter::{counter_value, adjust_counter},
        moves::{valid_move, unmake_move, make_move, moves_state,  unmake_move_no_hash, make_move_no_hash},
        hash::{find_hash, HashEntry, hash_state},
        globals::board_state,
        eval::terminal_evaluation,
        probcut::prob_cut,
        zebra::{EvaluationType}
    }
};
use crate::src::getcoeff::{coeff_state, pattern_evaluation, CoeffState};
use crate::src::stubs::abs;
use crate::src::timer::{check_panic_abort, above_recommended, extended_above_recommended, g_timer};
use crate::src::hash::add_hash;
use crate::src::error::FrontEnd;
use crate::src::zebra::EvalResult::{UNSOLVED_POSITION, LOST_POSITION, WON_POSITION};
use crate::src::zebra::EvalType::{MIDGAME_EVAL, EXACT_EVAL, UNDEFINED_EVAL};
use crate::src::myrandom::{random_instance, MyRandom};
use crate::src::search::SearchState;
use crate::src::moves::{MovesState, generate_all};
use crate::src::globals::BoardState;


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
pub struct MidgameState {
    pub allow_midgame_hash_probe: i32,
    pub allow_midgame_hash_update: i32,
    pub best_mid_move: i32,
    pub best_mid_root_move: i32,
    pub midgame_abort: i32,
    pub do_check_midgame_abort: i32,
    pub counter_phase: i32,
    pub apply_perturbation: i32,
    pub perturbation_amplitude: i32,
    pub stage_reached: [i32; 62],
    pub stage_score: [i32; 62],
    pub score_perturbation: [i32; 100],
    pub feas_index_list: [[i32; 64]; 64],
}

pub static mut midgame_state: MidgameState = MidgameState {
    allow_midgame_hash_probe: 0,
    allow_midgame_hash_update: 0,
    best_mid_move: 0,
    best_mid_root_move: 0,
    midgame_abort: 0,
    do_check_midgame_abort: 1,
    counter_phase: 0,
    apply_perturbation: 1,
    perturbation_amplitude: 0,
    stage_reached: [0; 62],
    stage_score: [0; 62],
    score_perturbation: [0; 100],
    feas_index_list: [[0; 64]; 64],
};

impl MidgameState {
    /*
      CLEAR_MIDGAME_ABORT
      IS_MIDGAME_ABORT
      SET_MIDGAME_ABORT
      TOGGLE_MIDGAME_ABORT_CHECK
      These functions handle the midgame abort system which kicks in
      when it is estimated that the next iteration in the iterative
      deepening would take too long.
    */

    pub fn clear_midgame_abort(&mut self) {
        self.midgame_abort = 0;
    }

    pub fn is_midgame_abort(&mut self) -> i32 {
        return self.midgame_abort;
    }

    pub fn set_midgame_abort(&mut self) {
        self.midgame_abort = self.do_check_midgame_abort;
    }

    pub fn toggle_midgame_abort_check(&mut self, toggle: i32) {
        self.do_check_midgame_abort = toggle;
    }
    /*
       TOGGLE_MIDGAME_HASH_USAGE
       Toggles hash table access in the midgame search on/off.
    */

    pub fn toggle_midgame_hash_usage(&mut self, allow_read:
                                            i32,
                                            allow_write:
                                            i32) {
        self.allow_midgame_hash_probe = allow_read;
        self.allow_midgame_hash_update = allow_write;
    }

    /*
      SET_PERTURBATION
      Set the amplitude of the score perturbation applied by
      CALCULATE_PERTURBATION.
    */

    pub fn set_perturbation(&mut self, amplitude: i32) {
        self.perturbation_amplitude = amplitude;
    }
    /*
      TOGGLE_PERTURBATION_USAGE
      Toggle usage of score perturbations on/off.
    */

    pub fn toggle_perturbation_usage(&mut self, toggle: i32) {
        self.apply_perturbation = toggle;
    }
}
/*
  ADVANCE_MOVE
  Swaps a move and its predecessor in the move list if it's
  not already first in the list.
*/
pub fn advance_move(index: i32, search: &mut SearchState, moves: &mut MovesState) {
    let mut temp_move: i32 = 0;
    if index > 0 as i32 {
        temp_move = search.sorted_move_order[moves.disks_played as usize][index as usize];
        search.sorted_move_order[moves.disks_played as usize][index as usize] =
            search.sorted_move_order[moves.disks_played as
                usize][(index - 1 as i32) as usize];
        search.sorted_move_order[moves.disks_played as
            usize][(index - 1 as i32) as usize] =
            temp_move
    };
}
/*
  midgame_c__update_best_list
*/
pub fn midgame_c__update_best_list(best_list: &mut [i32; 4], move_0: i32, best_list_index: i32, best_list_length: i32) {
    let mut i: i32 = 0;
    if best_list_index < best_list_length {
        i = best_list_index;
        while i >= 1 as i32 {
            best_list[i as usize] = best_list[(i - 1 as i32) as usize];
            i -= 1
        }
    } else {
        i = 3;
        while i >= 1 as i32 {
            best_list[i as usize] = best_list[(i - 1 as i32) as usize];
            i -= 1
        }
    }
    best_list[0] = move_0;
}


/*
  STATIC_OR_TERMINAL_EVALUATION
  Invokes the proper evaluation function depending on whether the
  board is filled or not.
*/

pub fn static_or_terminal_evaluation(side_to_move: i32, moves_state_: &MovesState, board_state_: &mut BoardState, search_state_: &mut SearchState, coeff_state_: &mut CoeffState) -> i32 {
    if moves_state_.disks_played == 60 as i32 {
        terminal_evaluation(board_state_.get_piece_counts(side_to_move, moves_state_.disks_played), &mut search_state_.evaluations)
    } else {
        search_state_.evaluations.lo = search_state_.evaluations.lo.wrapping_add(1);
        pattern_evaluation(side_to_move, board_state_, moves_state_, coeff_state_)
    }
}

/*
   SETUP_MIDGAME
   Sets up some search parameters.
*/

pub fn setup_midgame(state: &mut MidgameState, random: &mut MyRandom) {
    state.allow_midgame_hash_probe = 1;
    state.allow_midgame_hash_update = 1;
    state.stage_reached = [0; 62];
    calculate_perturbation(state, random);
}
/*
  CALCULATE_PERTURBATION
  Determines the score perturbations (if any) to the root moves.
*/

pub fn calculate_perturbation(state: &mut MidgameState, random: &mut MyRandom) {
    let random = random;
    if state.apply_perturbation == 0 || state.perturbation_amplitude == 0 {
        state.score_perturbation = [0; 100];
    } else {
        let shift = state.perturbation_amplitude / 2;
        let mut i = 0;
        while i < state.score_perturbation.len()  {
            state.score_perturbation[i as usize] = abs(random.my_random() as i32) % state.perturbation_amplitude - shift;
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
pub unsafe fn protected_one_ply_search<FE: FrontEnd>(side_to_move: i32, echo:i32)
                                                     -> i32 {
    let mut i: i32 = 0;
    let mut move_0: i32 = 0;
    let mut depth_one_score: i32 = 0;
    let mut depth_two_score: i32 = 0;
    let mut best_score_restricted: i32 = 0;
    let mut best_score_unrestricted: i32 = 0;
    let mut best_move_restricted: i32 = 0;
    let mut best_move_unrestricted: i32 = 0;
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
    best_score_restricted = -(12345678 as i32);
    best_score_unrestricted = -(12345678 as i32);
    best_move_restricted = 0;
    best_move_unrestricted = 0;
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        search_state.nodes.lo = search_state.nodes.lo.wrapping_add(1);
        move_0 = moves_state.move_list[moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, move_0, 1 as i32);
        search_state.evaluations.lo = search_state.evaluations.lo.wrapping_add(1);
        let side_to_move_argument = 0 as i32 + 2 as i32 -
            side_to_move;
        depth_one_score =
            -pattern_evaluation(side_to_move_argument, &mut board_state, &moves_state, &mut coeff_state);
        depth_two_score =
            -tree_search::<FE>(1 as i32, 2 as i32,
                         0 as i32 + 2 as i32 - side_to_move,
                         -(12345678 as i32), 12345678 as i32,
                         0 as i32, 0 as i32,
                         0 as i32, echo);
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
    board_state.pv_depth[0] = 1;
    if best_score_restricted > -(12345678 as i32) {
        /* No immediate loss */
        board_state.pv[0][0] =
            best_move_restricted;
        return best_score_restricted
    } else {
        board_state.pv[0][0] =
            best_move_unrestricted;
        return best_score_unrestricted
    };
}

/*
   TREE_SEARCH
   The recursive tree search function. It uses negascout for
   tree pruning.
*/

pub unsafe fn tree_search<FE: FrontEnd>(level: i32,
                                        max_depth: i32,
                                        side_to_move: i32,
                                        alpha: i32,
                                        beta: i32,
                                        allow_hash: i32,
                                        allow_mpc: i32,
                                        void_legal: i32, echo: i32)
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
    let mpc_cut_ = &prob_cut.mpc_cut;
    if level >= max_depth {
        search_state.nodes.lo = search_state.nodes.lo.wrapping_add(1);
        return static_or_terminal_evaluation(side_to_move, &moves_state, &mut board_state, &mut search_state, &mut coeff_state)
    }
    remains = max_depth - level;
    if remains < 3 as i32 {
        curr_val =
            fast_tree_search::<FE>(level, max_depth, side_to_move, alpha, beta,
                             allow_hash, void_legal);
        board_state.pv_depth[level as usize] = level + 1 as i32;
        board_state.pv[level as usize][level as usize] = midgame_state.best_mid_move;
        return curr_val
    }
    search_state.nodes.lo = search_state.nodes.lo.wrapping_add(1);
    /* Check the hash table */
    use_hash =
        (remains >= 2 as i32 && 1 as i32 != 0 &&
            allow_hash != 0) as i32;
    if 1 as i32 != 0 && allow_mpc != 0 {
        selectivity = 1 as i32
    } else { selectivity = 0 as i32 }
    if use_hash != 0 && midgame_state.allow_midgame_hash_probe != 0 {
        find_hash(&mut entry, 0 as i32, &mut hash_state);
        if entry.draft as i32 >= remains &&
            entry.selectivity as i32 <= selectivity &&
            valid_move(entry.move_0[0],
                       side_to_move) != 0 &&
            entry.flags as i32 & 8 as i32 != 0 &&
            (entry.flags as i32 & 4 as i32 != 0 ||
                entry.flags as i32 & 1 as i32 != 0 &&
                    entry.eval >= beta ||
                entry.flags as i32 & 2 as i32 != 0 &&
                    entry.eval <= alpha) {
            board_state.pv[level as usize][level as usize] =
                entry.move_0[0];
            board_state.pv_depth[level as usize] = level + 1 as i32;
            return entry.eval
        }
    }
    hash_hit =
        (use_hash != 0 && midgame_state.allow_midgame_hash_probe != 0) as i32;
    if hash_hit != 0 {
        hash_move = entry.move_0[0]
    } else { hash_move = 44 as i32 }
    pre_search_done = 0;
    /* Use multi-prob-cut to selectively prune the tree */
    if 1 as i32 != 0 && allow_mpc != 0 && remains <= 22 as i32
    {
        let mut alpha_test = 1;
        let mut beta_test = 1;
        cut = 0;
        while cut < mpc_cut_[remains as usize].cut_tries {
            /* Determine the fail-high and fail-low bounds */
            let bias =
                mpc_cut_[remains as
                    usize].bias[cut as usize][moves_state.disks_played as usize];
            let window =
                mpc_cut_[remains as
                    usize].window[cut as
                    usize][moves_state.disks_played as usize];
            let alpha_bound = alpha + bias - window;
            let beta_bound = beta + bias + window;
            /* Don't use an MPC cut which results in the full-width depth
            being less than some predefined constant */
            shallow_remains =
                mpc_cut_[remains as usize].cut_depth[cut as usize];
            if !(level + shallow_remains < 8 as i32) {
                if shallow_remains > 1 as i32 {
                    /* "Deep" shallow search */
                    if cut == 0 as i32 {
                        /* Use static eval to decide if a one- or two-sided
                       MPC test is to be performed. */
                        search_state.evaluations.lo = search_state.evaluations.lo.wrapping_add(1);
                        let static_eval =
                            pattern_evaluation(side_to_move, &mut board_state, &moves_state, &mut coeff_state);
                        if static_eval <= alpha_bound {
                            beta_test = 0 as i32
                        } else if static_eval >= beta_bound {
                            alpha_test = 0 as i32
                        }
                    }
                    assert!(alpha_test != 0 || beta_test != 0);
                    if alpha_test != 0 && beta_test != 0 {
                        /* Test for likely fail-low or likely fail-high. */
                        let shallow_val =
                            tree_search::<FE>(level, level + shallow_remains,
                                        side_to_move, alpha_bound, beta_bound,
                                        allow_hash, 0 as i32,
                                        void_legal, echo);
                        if shallow_val >= beta_bound {
                            if use_hash != 0 && midgame_state.allow_midgame_hash_update != 0
                            {
                                add_hash(&mut hash_state,0 as i32, beta,
                                         board_state.pv[level as usize][level as usize],
                                         8 as i32 | 1 as i32,
                                         remains, selectivity);
                            }
                            return beta
                        } else if shallow_val <= alpha_bound {
                            if use_hash != 0 && midgame_state.allow_midgame_hash_update != 0
                            {
                                add_hash(&mut hash_state,0 as i32, alpha,
                                         board_state.pv[level as usize][level as usize],
                                         8 as i32 | 2 as i32,
                                         remains, selectivity);
                            }
                            return alpha
                        } else {
                            /* Use information learned from the failed cut test to decide
                           if a one or a two-sided test is to be performed next. */
                            let mid =
                                (alpha_bound + beta_bound) / 2 as i32;
                            let low_threshold =
                                (2 as i32 * mid + alpha_bound) /
                                    3 as i32;
                            let high_threshold =
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
                        if tree_search::<FE>(level, level + shallow_remains,
                                       side_to_move,
                                       beta_bound - 1 as i32,
                                       beta_bound, allow_hash,
                                       0 as i32, void_legal, echo) >=
                            beta_bound {
                            if use_hash != 0 && midgame_state.allow_midgame_hash_update != 0
                            {
                                add_hash(&mut hash_state,0 as i32, beta,
                                         board_state.pv[level as usize][level as usize],
                                         8 as i32 | 1 as i32,
                                         remains, selectivity);
                            }
                            return beta
                        }
                    } else if alpha_test != 0 {
                        /* Fail-low with high probability? */
                        if tree_search::<FE>(level, level + shallow_remains,
                                       side_to_move, alpha_bound,
                                       alpha_bound + 1 as i32,
                                       allow_hash, 0 as i32,
                                       void_legal, echo) <= alpha_bound {
                            if use_hash != 0 && midgame_state.allow_midgame_hash_update != 0
                            {
                                add_hash(&mut hash_state,0 as i32, alpha,
                                         board_state.pv[level as usize][level as usize],
                                         8 as i32 | 2 as i32,
                                         remains, selectivity);
                            }
                            return alpha
                        }
                    }
                } else {
                    /* All-in-one MPC one-ply search and move ordering */
                    moves_state.move_count[moves_state.disks_played as usize] = 0;
                    best = alpha_bound;
                    empties_remaining = 60 as i32 - moves_state.disks_played;
                    move_index = 0;
                    while move_index < 60 as i32 {
                        move_0 =
                            search_state.sorted_move_order[moves_state.disks_played as
                                usize][move_index as usize];
                        if board_state.board[move_0 as usize] == 1 as i32 {
                            if make_move_no_hash(side_to_move, move_0) !=
                                0 as i32 {
                                let side_to_move_argument = 0 as
                                    i32
                                    +
                                    2 as
                                        i32
                                    -
                                    side_to_move;
                                curr_val =
                                    -static_or_terminal_evaluation(side_to_move_argument, &moves_state, &mut board_state, &mut search_state, &mut coeff_state);
                                unmake_move_no_hash(side_to_move, move_0);
                                search_state.nodes.lo = search_state.nodes.lo.wrapping_add(1);
                                if curr_val > best {
                                    best = curr_val;
                                    if best >= beta_bound {
                                        if use_hash != 0 &&
                                            midgame_state.allow_midgame_hash_update != 0
                                        {
                                            add_hash(&mut hash_state,0 as i32, beta,
                                                     board_state.pv[level as
                                                         usize][level as
                                                         usize],
                                                     8 as i32 |
                                                         1 as i32,
                                                     remains, selectivity);
                                        }
                                        return beta
                                    }
                                }
                                search_state.evals[moves_state.disks_played as usize][move_0 as usize]
                                    = curr_val;
                                if move_0 == hash_move {
                                    /* Always try hash table move first */
                                    search_state.evals[moves_state.disks_played as
                                        usize][move_0 as usize] +=
                                        10000 as i32
                                }
                                midgame_state.feas_index_list[moves_state.disks_played as
                                    usize][moves_state.move_count[moves_state.disks_played
                                    as
                                    usize]
                                    as usize] =
                                    move_index;
                                moves_state.move_count[moves_state.disks_played as usize] += 1
                            }
                            empties_remaining -= 1;
                            if empties_remaining == 0 as i32 {
                                break ;
                            }
                        }
                        move_index += 1
                    }
                    if best == alpha_bound &&
                        moves_state.move_count[moves_state.disks_played as usize] >
                            0 as i32 {
                        if use_hash != 0 && midgame_state.allow_midgame_hash_update != 0 {
                            add_hash(&mut hash_state,0 as i32, alpha,
                                     board_state.pv[level as usize][level as usize],
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
    searched = 0;
    best = -(12345678 as i32);
    best_move_index = -(1 as i32);
    curr_alpha = alpha;
    best_list_length = 0;
    i = 0;
    while i < 4 as i32 {
        best_list[i as usize] = 0;
        i += 1
    }
    if pre_search_done == 0 {
        moves_state.move_count[moves_state.disks_played as usize] = 0;
        if hash_hit != 0 {
            i = 0;
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
    i = 0;
    best_list_index = 0;
    loop
    /* Try the hash table move(s) first if feasible */
    {
        if pre_search_done == 0 && best_list_index < best_list_length {
            moves_state.move_count[moves_state.disks_played as usize] += 1;
            move_index = 0;
            while search_state.sorted_move_order[moves_state.disks_played as
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
                empties_remaining = 60 as i32 - moves_state.disks_played;
                move_index = 0;
                while move_index < 60 as i32 {
                    let mut already_checked: i32 = 0;
                    move_0 =
                        search_state.sorted_move_order[moves_state.disks_played as
                            usize][move_index as usize];
                    already_checked = 0;
                    j = 0;
                    while j < best_list_length {
                        if move_0 == best_list[j as usize] {
                            already_checked = 1 as i32
                        }
                        j += 1
                    }
                    if board_state.board[move_0 as usize] == 1 as i32 {
                        if already_checked == 0 &&
                            make_move(side_to_move, move_0,
                                      1 as i32) != 0 as i32
                        {
                            curr_val =
                                -tree_search::<FE>(level + 1 as i32,
                                             level + pre_depth,
                                             0 as i32 +
                                                 2 as i32 -
                                                 side_to_move,
                                             -(12345678 as i32),
                                             -pre_best, 0 as i32,
                                             0 as i32,
                                             1 as i32, echo);
                            pre_best =
                                if pre_best > curr_val {
                                    pre_best
                                } else { curr_val };
                            unmake_move(side_to_move, move_0);
                            search_state.evals[moves_state.disks_played as usize][move_0 as usize] =
                                curr_val;
                            midgame_state.feas_index_list[moves_state.disks_played as
                                usize][moves_state.move_count[moves_state.disks_played
                                as
                                usize]
                                as usize] =
                                move_index;
                            moves_state.move_count[moves_state.disks_played as usize] += 1
                        }
                        empties_remaining -= 1;
                        if empties_remaining == 0 as i32 { break ; }
                    }
                    move_index += 1
                }
                pre_search_done = 1 as i32
            }
            if i == moves_state.move_count[moves_state.disks_played as usize] { break ; }
            best_index = i;
            best_score =
                search_state.evals[moves_state.disks_played as
                    usize][search_state.sorted_move_order[moves_state.disks_played as
                    usize][midgame_state.feas_index_list[moves_state.disks_played
                    as
                    usize][i
                    as
                    usize]
                    as usize] as
                    usize];
            j = i + 1 as i32;
            while j < moves_state.move_count[moves_state.disks_played as usize] {
                let mut cand_move: i32 = 0;
                cand_move =
                    search_state.sorted_move_order[moves_state.disks_played as
                        usize][midgame_state.feas_index_list[moves_state.disks_played
                        as
                        usize][j
                        as
                        usize]
                        as usize];
                if search_state.evals[moves_state.disks_played as usize][cand_move as usize] >
                    best_score {
                    best_score =
                        search_state.evals[moves_state.disks_played as usize][cand_move as usize];
                    best_index = j
                }
                j += 1
            }
            move_index =
                midgame_state.feas_index_list[moves_state.disks_played as usize][best_index as usize];
            midgame_state.feas_index_list[moves_state.disks_played as usize][best_index as usize] =
                midgame_state.feas_index_list[moves_state.disks_played as usize][i as usize]
        }
        move_0 =
            search_state.sorted_move_order[moves_state.disks_played as usize][move_index as usize];
        midgame_state.counter_phase = midgame_state.counter_phase + 1 as i32 & 63 as i32;
        if midgame_state.counter_phase == 0 as i32 {
            let mut node_val: f64 = 0.;
            adjust_counter(&mut search_state.nodes);
            node_val = counter_value(&mut search_state.nodes);
            if node_val - g_timer.last_panic_check >=
                100000 as i32 as f64 {
                /* Time abort? */
                g_timer.last_panic_check = node_val;
                check_panic_abort::<FE>();
                /* Display available search information */
                if echo != 0 { FE::display_buffers(); }
                /* Check for events */
                if g_timer.is_panic_abort() != 0 || force_return != 0 {
                    return -(27000 as i32)
                }
            }
        }
        make_move(side_to_move, move_0, 1 as i32);
        update_pv = 0;
        if searched == 0 as i32 {
            curr_val =
                -tree_search::<FE>(level + 1 as i32, max_depth,
                             0 as i32 + 2 as i32 -
                                 side_to_move, -beta, -curr_alpha, allow_hash,
                             allow_mpc, 1 as i32, echo);
            best = curr_val;
            best_move_index = move_index;
            update_pv = 1 as i32
        } else {
            curr_alpha = if best > curr_alpha { best } else { curr_alpha };
            curr_val =
                -tree_search::<FE>(level + 1 as i32, max_depth,
                             0 as i32 + 2 as i32 -
                                 side_to_move,
                             -(curr_alpha + 1 as i32), -curr_alpha,
                             allow_hash, allow_mpc, 1 as i32, echo);
            if curr_val > curr_alpha && curr_val < beta {
                curr_val =
                    -tree_search::<FE>(level + 1 as i32, max_depth,
                                 0 as i32 + 2 as i32 -
                                     side_to_move, -beta,
                                 12345678 as i32, allow_hash,
                                 allow_mpc, 1 as i32, echo);
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
        if g_timer.is_panic_abort() != 0 || force_return != 0 {
            return -(27000 as i32)
        }
        search_state.evals[moves_state.disks_played as usize][move_0 as usize] = curr_val;
        if update_pv != 0 {
            midgame_c__update_best_list(&mut best_list, move_0,
                                        best_list_index, best_list_length);
            board_state.pv[level as usize][level as usize] = move_0;
            board_state.pv_depth[level as usize] =
                board_state.pv_depth[(level + 1 as i32) as usize];
            j = level + 1 as i32;
            while j < board_state.pv_depth[(level + 1 as i32) as usize] {
                board_state.pv[level as usize][j as usize] =
                    board_state.pv[(level + 1 as i32) as usize][j as usize];
                j += 1
            }
        }
        if best >= beta {
            advance_move(move_index, &mut search_state, &mut moves_state);
            if use_hash != 0 && midgame_state.allow_midgame_hash_update != 0 {
                hash_state.add_hash_extended(0 as i32, best,
                                  &best_list,
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
    if moves_state.move_count[moves_state.disks_played as usize] > 0 as i32 {
        advance_move(best_move_index, &mut search_state, &mut moves_state);
        if use_hash != 0 && midgame_state.allow_midgame_hash_update != 0 {
            if best > alpha {
                hash_state.add_hash_extended(0 as i32, best,
                                  &best_list,
                                  8 as i32 | 4 as i32,
                                  remains, selectivity);
            } else {
                hash_state.add_hash_extended(0 as i32, best,
                                  &best_list,
                                  8 as i32 | 2 as i32,
                                  remains, selectivity);
            }
        }
        return best
    } else if void_legal != 0 {
        /* No feasible moves */
        hash_state.hash1 ^= hash_state.hash_flip_color1;
        hash_state.hash2 ^= hash_state.hash_flip_color2;
        curr_val =
            -tree_search::<FE>(level, max_depth,
                         0 as i32 + 2 as i32 - side_to_move,
                         -beta, -alpha, allow_hash, allow_mpc,
                         0 as i32, echo);
        hash_state.hash1 ^= hash_state.hash_flip_color1;
        hash_state.hash2 ^= hash_state.hash_flip_color2;
        return curr_val
    } else {
        board_state.pv_depth[level as usize] = level;
        return terminal_evaluation(board_state.get_piece_counts(side_to_move, moves_state.disks_played), &mut search_state.evaluations)
    };
}

/*
   FAST_TREE_SEARCH
   The recursive tree search function. It uses negascout for
   tree pruning.
*/
unsafe fn fast_tree_search<FE: FrontEnd>(level: i32,
                                         max_depth: i32,
                                         side_to_move: i32,
                                         alpha: i32,
                                         beta: i32,
                                         allow_hash: i32,
                                         void_legal: i32)
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
    search_state.nodes.lo = search_state.nodes.lo.wrapping_add(1);
    if level >= max_depth {
        return static_or_terminal_evaluation(side_to_move, &moves_state, &mut board_state, &mut search_state, &mut coeff_state)
    }
    /* Check the hash table */
    remains = max_depth - level;
    use_hash =
        (remains >= 2 as i32 && 1 as i32 != 0 &&
            allow_hash != 0) as i32;
    if use_hash != 0 && midgame_state.allow_midgame_hash_probe != 0 {
        find_hash(&mut entry, 0 as i32, &mut hash_state);
        if entry.draft as i32 >= remains &&
            entry.selectivity as i32 == 0 as i32 &&
            valid_move(entry.move_0[0],
                       side_to_move) != 0 &&
            entry.flags as i32 & 8 as i32 != 0 &&
            (entry.flags as i32 & 4 as i32 != 0 ||
                entry.flags as i32 & 1 as i32 != 0 &&
                    entry.eval >= beta ||
                entry.flags as i32 & 2 as i32 != 0 &&
                    entry.eval <= alpha) {
            midgame_state.best_mid_move = entry.move_0[0];
            return entry.eval
        }
    }
    /* Reorder the move lists now and then to keep the empty squares up front */
    if search_state.nodes.lo & 4095 as i32 as u32 == 0 as i32 as u32 {
        reorder_move_list(&board_state.board, &mut search_state.sorted_move_order[moves_state.disks_played as usize]);
    }
    /* Search */
    first = 1;
    best_move = -(1 as i32);
    best_move_index = -(1 as i32);
    best = -(12345678 as i32);
    if remains == 1 as i32 {
        /* Plain alpha-beta last ply */
        empties_remaining = 60 as i32 - moves_state.disks_played;
        move_index = 0;
        while move_index < 60 as i32 {
            move_0 =
                search_state.sorted_move_order[moves_state.disks_played as usize][move_index as usize];
            if board_state.board[move_0 as usize] == 1 as i32 {
                if make_move_no_hash(side_to_move, move_0) != 0 as i32
                {
                    let side_to_move_argument = 0 as i32 +
                        2 as i32 -
                        side_to_move;
                    curr_val =
                        -static_or_terminal_evaluation(side_to_move_argument, &moves_state, &mut board_state, &mut search_state, &mut coeff_state);
                    unmake_move_no_hash(side_to_move, move_0);
                    search_state.nodes.lo = search_state.nodes.lo.wrapping_add(1);
                    if curr_val > best {
                        best = curr_val;
                        best_move_index = move_index;
                        best_move = move_0;
                        if curr_val >= beta {
                            advance_move(move_index, &mut search_state, &mut moves_state);
                            midgame_state.best_mid_move = best_move;
                            if use_hash != 0 && midgame_state.allow_midgame_hash_update != 0
                            {
                                add_hash(&mut hash_state,0 as i32, best, best_move,
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
        empties_remaining = 60 as i32 - moves_state.disks_played;
        move_index = 0;
        while move_index < 60 as i32 {
            move_0 =
                search_state.sorted_move_order[moves_state.disks_played as usize][move_index as usize];
            if board_state.board[move_0 as usize] == 1 as i32 {
                if make_move(side_to_move, move_0, new_use_hash) !=
                    0 as i32 {
                    if first != 0 {
                        curr_val =
                            -fast_tree_search::<FE>(level + 1 as i32,
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
                            -fast_tree_search::<FE>(level + 1 as i32,
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
                                -fast_tree_search::<FE>(level + 1 as i32,
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
                        advance_move(move_index, &mut search_state, &mut moves_state);
                        midgame_state.best_mid_move = best_move;
                        if use_hash != 0 && midgame_state.allow_midgame_hash_update != 0 {
                            add_hash(&mut hash_state,0 as i32, best, best_move,
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
        advance_move(best_move_index, &mut search_state, &mut moves_state);
        midgame_state.best_mid_move = best_move;
        if use_hash != 0 && midgame_state.allow_midgame_hash_update != 0 {
            if best > alpha {
                add_hash(&mut hash_state,0 as i32, best, best_move,
                         8 as i32 | 4 as i32, remains,
                         0 as i32);
            } else {
                add_hash(&mut hash_state,0 as i32, best, best_move,
                         8 as i32 | 2 as i32, remains,
                         0 as i32);
            }
        }
        return best
    } else if void_legal != 0 {
        /* I pass, other player's turn now */
        hash_state.hash1 ^= hash_state.hash_flip_color1;
        hash_state.hash2 ^= hash_state.hash_flip_color2;
        curr_val =
            -fast_tree_search::<FE>(level, max_depth,
                              0 as i32 + 2 as i32 -
                                  side_to_move, -beta, -alpha, allow_hash,
                              0 as i32);
        hash_state.hash1 ^= hash_state.hash_flip_color1;
        hash_state.hash2 ^= hash_state.hash_flip_color2;
        return curr_val
    } else {
        /* Both players had to pass ==> evaluate board as final */
        curr_val = terminal_evaluation(board_state.get_piece_counts(side_to_move, moves_state.disks_played), &mut search_state.evaluations);
        return curr_val
    };
}

/*
  PERTURB_SCORE
  Perturbs SCORE by PERTURBATION if it doesn't appear to be
  a midgame win.
*/
pub unsafe fn perturb_score(score: i32,
                        perturbation: i32)
                        -> i32 {
    if abs(score) < 29000 as i32 - 4000 as i32 {
        return score + perturbation
    } else { return score };
}


/*
   ROOT_TREE_SEARCH
   The recursive tree search function that is to be called only
   for the root of the search tree.
*/

pub unsafe fn root_tree_search<FE: FrontEnd>(level: i32,
                                             max_depth: i32,
                                             side_to_move: i32,
                                             alpha: i32,
                                             beta: i32,
                                             allow_hash: i32,
                                             allow_mpc: i32,
                                             void_legal: i32, echo: i32)
                                             -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut curr_val: i32 = 0;
    let mut best: i32 = 0;
    let mut pre_best: i32 = 0;
    let mut searched: i32 = 0;
    let mut move_0: i32 = 0;
    let mut move_index: i32 = 0;
    let mut best_move_index: i32 = 0;
    let mut hash_hit: i32 = 0;
    let mut pre_depth: i32 = 0;
    let mut update_pv: i32 = 0;
    let mut remains: i32 = 0;
    let mut use_hash: i32 = 0;
    let mut pre_search_done: i32 = 0;
    let mut curr_alpha: i32 = 0;
    let mut best_index: i32 = 0;
    let mut best_score: i32 = 0;
    let mut best_list_index: i32 = 0;
    let mut best_list_length: i32 = 0;
    let mut selectivity: i32 = 0;
    let mut offset: i32 = 0;
    let mut best_list: [i32; 4] = [0; 4];
    let mut entry =
        HashEntry{key1: 0,
            key2: 0,
            eval: 0,
            move_0: [0; 4],
            draft: 0,
            selectivity: 0,
            flags: 0,};
    remains = max_depth - level;
    search_state.nodes.lo = search_state.nodes.lo.wrapping_add(1);
    use_hash =
        (remains >= 2 as i32 && 1 as i32 != 0 &&
            allow_hash != 0) as i32;
    if 1 as i32 != 0 && allow_mpc != 0 {
        selectivity = 1 as i32
    } else { selectivity = 0 as i32 }
    /* Hash strategy at the root: Only use hash table information for
       move ordering purposes.  This guarantees that score perturbation
       is applied for all moves. */
    hash_hit = 0;
    if use_hash != 0 && midgame_state.allow_midgame_hash_probe != 0 {
        find_hash(&mut entry, 0 as i32, &mut hash_state);
        if entry.draft as i32 != 0 as i32 {
            hash_hit = 1 as i32
        }
    }
    pre_search_done = 0;
    if get_ponder_move() == 0 {
        FE::midgame_display_initial_ponder_move(alpha, beta);
    }
    /* Full negascout search */
    searched = 0;
    best = -(12345678 as i32);
    best_move_index = -(1 as i32);
    curr_alpha = alpha;
    best_list_length = 0;
    i = 0;
    while i < 4 as i32 {
        best_list[i as usize] = 0;
        i += 1
    }
    if pre_search_done == 0 {
        moves_state.move_count[moves_state.disks_played as usize] = 0;
        if hash_hit != 0 {
            i = 0;
            while i < 4 as i32 {
                if valid_move(entry.move_0[i as usize], side_to_move) != 0 {
                    let fresh1 = best_list_length;
                    best_list_length = best_list_length + 1;
                    best_list[fresh1 as usize] = entry.move_0[i as usize]
                }
                i += 1
            }
        }
    }
    i = 0;
    best_list_index = 0;
    loop
    /* Try the hash table move(s) first if feasible */
    {
        if pre_search_done == 0 && best_list_index < best_list_length {
            moves_state.move_count[moves_state.disks_played as usize] += 1;
            move_index = 0;
            while search_state.sorted_move_order[moves_state.disks_played as
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
                move_index = 0;
                while move_index < 60 as i32 {
                    let mut already_checked: i32 = 0;
                    move_0 =
                        search_state.sorted_move_order[moves_state.disks_played as
                            usize][move_index as usize];
                    already_checked = 0;
                    j = 0;
                    while j < best_list_length {
                        if move_0 == best_list[j as usize] {
                            already_checked = 1 as i32
                        }
                        j += 1
                    }
                    if already_checked == 0 &&
                        board_state.board[move_0 as usize] == 1 as i32 &&
                        make_move(side_to_move, move_0, 1 as i32)
                            != 0 as i32 {
                        curr_val =
                            -tree_search::<FE>(level + 1 as i32,
                                         level + pre_depth,
                                         0 as i32 + 2 as i32 -
                                             side_to_move,
                                         -(12345678 as i32),
                                         -pre_best, 0 as i32,
                                         0 as i32, 1 as i32, echo);
                        pre_best =
                            if pre_best > curr_val {
                                pre_best
                            } else { curr_val };
                        unmake_move(side_to_move, move_0);
                        search_state.evals[moves_state.disks_played as usize][move_0 as usize] =
                            curr_val;
                        midgame_state.feas_index_list[moves_state.disks_played as
                            usize][moves_state.move_count[moves_state.disks_played as
                            usize] as
                            usize] = move_index;
                        moves_state.move_count[moves_state.disks_played as usize] += 1
                    }
                    move_index += 1
                }
                pre_search_done = 1 as i32
            }
            if i == moves_state.move_count[moves_state.disks_played as usize] { break ; }
            best_index = i;
            best_score =
                search_state.evals[moves_state.disks_played as
                    usize][search_state.sorted_move_order[moves_state.disks_played as
                    usize][midgame_state.feas_index_list[moves_state.disks_played
                    as
                    usize][i
                    as
                    usize]
                    as usize] as
                    usize];
            j = i + 1 as i32;
            while j < moves_state.move_count[moves_state.disks_played as usize] {
                let mut cand_move: i32 = 0;
                cand_move =
                    search_state.sorted_move_order[moves_state.disks_played as
                        usize][midgame_state.feas_index_list[moves_state.disks_played
                        as
                        usize][j
                        as
                        usize]
                        as usize];
                if search_state.evals[moves_state.disks_played as usize][cand_move as usize] >
                    best_score {
                    best_score =
                        search_state.evals[moves_state.disks_played as usize][cand_move as usize];
                    best_index = j
                }
                j += 1
            }
            move_index =
                midgame_state.feas_index_list[moves_state.disks_played as usize][best_index as usize];
            midgame_state.feas_index_list[moves_state.disks_played as usize][best_index as usize] =
                midgame_state.feas_index_list[moves_state.disks_played as usize][i as usize]
        }
        move_0 =
            search_state.sorted_move_order[moves_state.disks_played as usize][move_index as usize];
        if get_ponder_move() == 0 {
            FE::midgame_display_simple_ponder_move(move_0);
        }
        make_move(side_to_move, move_0, 1 as i32);
        update_pv = 0;
        offset = midgame_state.score_perturbation[move_0 as usize];
        if searched == 0 as i32 {
            curr_val =
                perturb_score(-tree_search::<FE>(level + 1 as i32,
                                           max_depth,
                                           0 as i32 + 2 as i32
                                               - side_to_move,
                                           -(beta - offset),
                                           -(curr_alpha - offset), allow_hash,
                                           allow_mpc, 1 as i32, echo),
                              offset);
            best = curr_val;
            best_move_index = move_index;
            update_pv = 1;
            midgame_state.best_mid_root_move = move_0
        } else {
            curr_alpha = if best > curr_alpha { best } else { curr_alpha };
            curr_val =
                perturb_score(-tree_search::<FE>(level + 1 as i32,
                                           max_depth,
                                           0 as i32 + 2 as i32
                                               - side_to_move,
                                           -(curr_alpha - offset +
                                               1 as i32),
                                           -(curr_alpha - offset), allow_hash,
                                           allow_mpc, 1 as i32, echo),
                              offset);
            if curr_val > curr_alpha && curr_val < beta {
                curr_val =
                    perturb_score(-tree_search::<FE>(level + 1 as i32,
                                               max_depth,
                                               0 as i32 +
                                                   2 as i32 -
                                                   side_to_move,
                                               -(beta - offset),
                                               12345678 as i32,
                                               allow_hash, allow_mpc,
                                               1 as i32, echo), offset);
                if curr_val > best {
                    best = curr_val;
                    best_move_index = move_index;
                    update_pv = 1;
                    if g_timer.is_panic_abort() == 0 && force_return == 0 {
                        midgame_state.best_mid_root_move = move_0
                    }
                }
            } else if curr_val > best {
                best = curr_val;
                best_move_index = move_index;
                update_pv = 1 as i32
            }
        }
        unmake_move(side_to_move, move_0);
        if g_timer.is_panic_abort() != 0 || force_return != 0 {
            return -(27000 as i32)
        }
        search_state.evals[moves_state.disks_played as usize][move_0 as usize] = curr_val;
        if get_ponder_move() == 0 {
            FE::midgame_display_ponder_move(max_depth, alpha, beta, curr_val, searched, update_pv)
        }
        if update_pv != 0 {
            midgame_c__update_best_list(&mut best_list, move_0,
                                        best_list_index, best_list_length);
            board_state.pv[level as usize][level as usize] = move_0;
            board_state.pv_depth[level as usize] =
                board_state.pv_depth[(level + 1 as i32) as usize];
            j = level + 1 as i32;
            while j < board_state.pv_depth[(level + 1 as i32) as usize] {
                board_state.pv[level as usize][j as usize] =
                    board_state.pv[(level + 1 as i32) as usize][j as usize];
                j += 1
            }
        }
        if best >= beta {
            advance_move(move_index, &mut search_state, &mut moves_state);
            if use_hash != 0 && midgame_state.allow_midgame_hash_update != 0 {
                hash_state.add_hash_extended(0 as i32, best,
                                  &best_list,
                                  8 as i32 | 1 as i32,
                                  remains, selectivity);
            }
            return best
        }
        /* For symmetry reasons, the score for any move is the score of the
           position for the initial position. */
        if moves_state.disks_played == 0 as i32 {
            hash_state.add_hash_extended(0 as i32, best, &best_list,
                              8 as i32 | 4 as i32, remains,
                              selectivity);
            return best
        }
        searched += 1;
        i += 1;
        best_list_index += 1
    }
    /* Post-processing */
    if moves_state.move_count[moves_state.disks_played as usize] > 0 as i32 {
        advance_move(best_move_index, &mut search_state, &mut moves_state);
        if use_hash != 0 && midgame_state.allow_midgame_hash_update != 0 {
            if best > alpha {
                hash_state.add_hash_extended(0 as i32, best,
                                  &best_list,
                                  8 as i32 | 4 as i32,
                                  remains, selectivity);
            } else {
                hash_state.add_hash_extended(0 as i32, best,
                                  &best_list,
                                  8 as i32 | 2 as i32,
                                  remains, selectivity);
            }
        }
        return best
    } else if void_legal != 0 {
        /* No feasible moves */
        hash_state.hash1 ^= hash_state.hash_flip_color1;
        hash_state.hash2 ^= hash_state.hash_flip_color2;
        curr_val =
            -root_tree_search::<FE>(level, max_depth,
                              0 as i32 + 2 as i32 -
                                  side_to_move, -beta, -alpha, allow_hash,
                              allow_mpc, 0 as i32, echo);
        hash_state.hash1 ^= hash_state.hash_flip_color1;
        hash_state.hash2 ^= hash_state.hash_flip_color2;
        return curr_val
    } else {
        board_state.pv_depth[level as usize] = level;
        return terminal_evaluation(board_state.get_piece_counts(side_to_move, moves_state.disks_played), &mut search_state.evaluations)
    };
}


/*
   MIDDLE_GAME
   side_to_move = the side whose turn it is to move
*/

pub unsafe fn middle_game<FE : FrontEnd>(side_to_move: i32,
                                         max_depth: i32,
                                         update_evals: i32,
                                         eval_info: &mut EvaluationType, echo:i32)
                                         -> i32 {
    let mut adjusted_val: i32;
    let mut alpha: i32;
    let mut beta: i32;
    let mut full_length_line: i32;
    let mut entry =
        HashEntry{key1: 0,
            key2: 0,
            eval: 0,
            move_0: [0; 4],
            draft: 0,
            selectivity: 0,
            flags: 0,};
    g_timer.last_panic_check = 0.0f64;
    midgame_state.counter_phase = 0;
    board_state.piece_count[0][moves_state.disks_played as usize] =
        disc_count(0 as i32, &board_state.board);
    board_state.piece_count[2][moves_state.disks_played as usize] =
        disc_count(2 as i32, &board_state.board);
    let base_stage =
        disc_count(0 as i32, &board_state.board) + disc_count(2 as i32, &board_state.board) -
            4 as i32;
    let mut val = 0;
    let mut old_val = --(27000 as i32);
    let enable_mpc = (max_depth >= 9 as i32) as i32;
    let initial_depth =
        if 1 as i32 > max_depth - 2 as i32 {
            1 as i32
        } else { (max_depth) - 2 as i32 };
    let initial_depth = max_depth;
    *eval_info =
        create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION, 0 as i32,
                         0.0f64, 0 as i32, 0 as i32);
    let mut depth = initial_depth;
    while depth <= max_depth {
        alpha = -(12345678 as i32);
        beta = 12345678;
        inherit_move_lists(moves_state.disks_played + max_depth, &mut search_state.sorted_move_order, &mut search_state.list_inherited);
        /* The actual search */
        if depth == 1 as i32 {
            /* Fix to make it harder to wipe out depth-1 Zebra */
            val = protected_one_ply_search::<FE>(side_to_move, echo)
        } else if enable_mpc != 0 {
            val =
                root_tree_search::<FE>(0 as i32, depth, side_to_move, alpha,
                                 beta, 1 as i32, 1 as i32,
                                 1 as i32, echo);
            if force_return == 0 && g_timer.is_panic_abort() == 0 &&
                (val <= alpha || val >= beta) {
                val =
                    root_tree_search::<FE>(0 as i32, depth, side_to_move,
                                     -(12345678 as i32),
                                     12345678 as i32,
                                     1 as i32, 1 as i32,
                                     1 as i32, echo)
            }
        } else {
            val =
                root_tree_search::<FE>(0 as i32, depth, side_to_move, alpha,
                                 beta, 1 as i32, 0 as i32,
                                 1 as i32, echo);
            if g_timer.is_panic_abort() == 0 && force_return == 0 {
                if val <= alpha {
                    val =
                        root_tree_search::<FE>(0 as i32, depth,
                                         side_to_move,
                                         -(29000 as i32), alpha,
                                         1 as i32, 0 as i32,
                                         1 as i32, echo)
                } else if val >= beta {
                    val =
                        root_tree_search::<FE>(0 as i32, depth,
                                         side_to_move, beta,
                                         29000 as i32,
                                         1 as i32, 0 as i32,
                                         1 as i32, echo)
                }
            }
        }
        /* Adjust scores and PV if search is aborted */
        if g_timer.is_panic_abort() != 0 || force_return != 0 {
            board_state.pv[0][0] =
                midgame_state.best_mid_root_move;
            board_state.pv_depth[0] = 1;
            hash_expand_pv(side_to_move, 0 as i32, 4 as i32,
                           12345678 as i32);
            if base_stage + depth - 2 as i32 >= 0 as i32 &&
                midgame_state.stage_reached[(base_stage + depth - 2 as i32) as
                    usize] != 0 {
                val =
                    midgame_state.stage_score[(base_stage + depth - 2 as i32) as
                        usize];
                if side_to_move == 2 as i32 { val = -val }
            } else { val = old_val }
        }
        /* Check if the search info corresponds to a variation of
           depth exactly DEPTH which would mean that the search
           gives new score information */
        full_length_line = 0;
        find_hash(&mut entry, 0 as i32, &mut hash_state);
        if force_return == 0 && g_timer.is_panic_abort() == 0 &&
            entry.draft as i32 != 0 as i32 &&
            valid_move(entry.move_0[0],
                       side_to_move) != 0 &&
            entry.draft as i32 == depth {
            full_length_line = 1 as i32
        }
        /* Update the stored scores */
        if (midgame_state.stage_reached[(base_stage + depth) as usize] == 0 ||
            full_length_line != 0) && update_evals != 0 {
            midgame_state.stage_reached[(base_stage + depth) as usize] = 1;
            if side_to_move == 0 as i32 {
                midgame_state.stage_score[(base_stage + depth) as usize] = val
            } else { midgame_state.stage_score[(base_stage + depth) as usize] = -val }
        }
        /* Adjust the eval for oscillations odd/even by simply averaging the
           last two stages (if they are available). */
        if midgame_state.stage_reached[(base_stage + depth) as usize] != 0 &&
            midgame_state.stage_reached[(base_stage + depth - 1 as i32) as usize]
                != 0 && update_evals != 0 {
            if side_to_move == 0 as i32 {
                adjusted_val =
                    (midgame_state.stage_score[(base_stage + depth) as usize] +
                        midgame_state.stage_score[(base_stage + depth - 1 as i32)
                            as usize]) / 2 as i32
            } else {
                adjusted_val =
                    -(midgame_state.stage_score[(base_stage + depth) as usize] +
                        midgame_state.stage_score[(base_stage + depth - 1 as i32)
                            as usize]) / 2 as i32
            }
        } else if depth == initial_depth {
            adjusted_val = val
        } else { adjusted_val = (val + old_val) / 2 as i32 }
        /* In case the search reached the end of the game, the score
           must be converted into an endgame score */
        if val >= 29000 as i32 {
            *eval_info =
                create_eval_info(EXACT_EVAL, WON_POSITION,
                                 (val - 29000 as i32) *
                                     128 as i32, 0.0f64, depth,
                                 0 as i32)
        } else if val <= -(29000 as i32) {
            *eval_info =
                create_eval_info(EXACT_EVAL, LOST_POSITION,
                                 (val + 29000 as i32) *
                                     128 as i32, 0.0f64, depth,
                                 0 as i32)
        } else {
            *eval_info =
                create_eval_info(MIDGAME_EVAL, UNSOLVED_POSITION,
                                 adjusted_val, 0.0f64, depth,
                                 0 as i32)
        }
        /* Display and store search info */
        if depth == max_depth {
            FE::midgame_display_status(side_to_move, max_depth, eval_info, depth,
                                       force_return != 0, &mut search_state.nodes,
                                       &mut board_state.pv[0], board_state.pv_depth[0])
        }
        if g_timer.is_panic_abort() != 0 || force_return != 0 { break ; }
        /* Check if search time or adjusted search time are long enough
           for the search to be discontinued */
        old_val = adjusted_val;
        if midgame_state.do_check_midgame_abort != 0 {
            if above_recommended::<FE>() != 0 ||
                extended_above_recommended::<FE>() != 0 &&
                    depth >= g_timer.frozen_ponder_depth {
                midgame_state.set_midgame_abort();
                break ;
            }
        }
        depth += 1
    }
    search_state.root_eval = val;
    return board_state.pv[0][0];
}
