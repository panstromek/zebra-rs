use flip::unflip::FlipStack;

use crate::src::counter::CounterType;
use crate::src::error::FrontEnd;
use crate::src::globals::{Board, BoardState};
use crate::src::hash::{determine_hash_values, find_hash, HashEntry, HashState};
use crate::src::moves::{make_move, MovesState, unmake_move};
use crate::src::zebra::{EvalResult, EvalType, EvaluationType};
use crate::src::zebra::EvalResult::{LOST_POSITION, UNSOLVED_POSITION, WON_POSITION};
use crate::src::zebra::EvalType::{MIDGAME_EVAL, UNINITIALIZED_EVAL};

/*
   File:          search.c

   Created:       July 1, 1997

   Modified:      January 2, 2003

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      Common search routines and variables.
*/


pub struct SearchState {
    pub total_time: f64,
    pub root_eval: i32,
    pub full_pv_depth: i32,
    pub full_pv: [i8; 120],
    pub list_inherited: [i32; 62],
    pub sorted_move_order: [[i8; 64]; 64],
    /* 61*60 used */
    pub evals: [Board; 61],
    pub nodes: CounterType,
    pub total_nodes: CounterType,
    pub evaluations: CounterType,
    pub total_evaluations: CounterType,
    pub pondered_move: i8,
    pub negate_eval: i32,
    pub last_eval: EvaluationType,
}

impl SearchState {
    pub const fn new() -> Self {
        Self {
            total_time: 0.,
            root_eval: 0,
            full_pv_depth: 0,
            full_pv: [0; 120],
            list_inherited: [0; 62],
            sorted_move_order: [[0; 64]; 64],
            evals: [[0; 128]; 61],
            nodes: CounterType { hi: 0, lo: 0 },
            total_nodes: CounterType { hi: 0, lo: 0 },
            evaluations: CounterType { hi: 0, lo: 0 },
            total_evaluations: CounterType { hi: 0, lo: 0 },
            pondered_move: 0,
            negate_eval: 0,
            last_eval:  EvaluationType::new(),
        }
    }
}

pub static force_return: i32 = 0;

/* When no other information is available, JCW's endgame
   priority order is used also in the midgame. */

pub static position_list: [i8; 100] =
    [11, 18, 81, 88, 13, 16, 31, 38, 61,
        68, 83, 86, 33, 36, 63,
        66, 14, 15, 41, 48, 51,
        58, 84, 85, 34, 35, 43,
        46, 53, 56, 64, 65, 24,
        25, 42, 47, 52, 57, 74,
        75, 23, 26, 32, 37, 62, 67, 73,
        76, 12, 17, 21, 28, 71, 78, 82,
        87, 22, 27, 72, 77, 44, 45, 54, 45, 0, 1, 2,
        3, 4, 5, 6, 7, 8, 9, 19,
        29, 39, 49, 59, 69, 79,
        89, 10, 20, 30, 40, 50, 60, 70, 80,
        90, 91, 92, 93, 94, 95,
        96, 97, 98, 99];

/*
  INIT_MOVE_LISTS
  Initalize the self-organizing move lists.
*/
fn init_move_lists(sorted_move_order_: &mut [[i8; 64]; 64]) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i <= 60 as i32 {
        j = 0;
        while j < 60 as i32 {
            sorted_move_order_[i as usize][j as usize] = position_list[j as usize];
            j += 1
        }
        i += 1
    }
}
/* The time spent searching during the game. */
/* The value of the root position from the last midgame or
   endgame search. Can contain strange values if an event
   occurred. */
/* Event flag which forces the search to abort immediately when set. */
/* The number of positions evaluated during the current search. */
/* The number of positions evaluated during the entire game. */
/* Holds the number of nodes searched during the current search. */
/* Holds the total number of nodes searched during the entire game. */
/* The last available evaluations for all possible moves at all
   possible game stages. */
/* Move lists */
/* 61*60 used */
/* The principal variation including passes */
/* JCW's move order */
/*
  INHERIT_MOVE_LISTS
  If possible, initialize the move list corresponding to STAGE
  moves being played with an earlier move list from a stage
  corresponding to the same parity (i.e., in practice side to move).
*/

pub fn inherit_move_lists(stage: i32, sorted_move_order_: &mut [[i8; 64]; 64], list_inherited_: &mut [i32; 62]) {
    let mut i: i32 = 0;
    let mut last: i32 = 0;
    if stage >= 61 || stage < 0 {
        return;
    }
    if list_inherited_[stage as usize] != 0 { return }
    list_inherited_[stage as usize] = 1;
    if stage == 0 { return }
    last = stage - 2 as i32;
    while last >= 0 as i32 && list_inherited_[last as usize] == 0 {
        last -= 2 as i32
    }
    if last < 0 { return }
    i = 0;
    while i < 60 {
        sorted_move_order_[stage as usize][i as usize] = sorted_move_order_[last as usize][i as usize];
        i += 1
    };
}
/*
  REORDER_MOVE_LIST
  Move the empty squares to the front of the move list.  Empty squares
  high up in the ranking are kept in place as they probably are empty
  in many variations in the tree.
*/

pub fn reorder_move_list(board_: & crate::src::globals::Board, stage_sorted_move_order: &mut [i8; 64]) {
    let dont_touch = 24;
    let mut i: i32 = 0;
    let mut move_0= 0;
    let mut empty_pos: i32 = 0;
    let mut nonempty_pos: i32 = 0;
    let mut empty_buffer: [i8; 60] = [0; 60];
    let mut nonempty_buffer: [i8; 60] = [0; 60];
    empty_pos = 0;
    i = 0;
    while i < 60 {
        move_0 = stage_sorted_move_order[i as usize];
        if board_[move_0 as usize] == 1 || i < dont_touch {
            empty_buffer[empty_pos as usize] = move_0;
            empty_pos += 1
        }
        i += 1
    }
    nonempty_pos = 60 as i32 - 1 as i32;
    i = 60 as i32 - 1 as i32;
    while i >= 0 as i32 {
        move_0 = stage_sorted_move_order[i as usize];
        if board_[move_0 as usize] != 1 as i32 && i >= dont_touch {
            nonempty_buffer[nonempty_pos as usize] = move_0;
            nonempty_pos -= 1
        }
        i -= 1
    }
    i = 0;
    while i < empty_pos {
        stage_sorted_move_order[i as usize] = empty_buffer[i as usize];
        i += 1
    }
    i = empty_pos;
    while i < 60 as i32 {
        stage_sorted_move_order[i as usize] = nonempty_buffer[i as usize];
        i += 1
    };
}
/*
   SETUP_SEARCH
   Initialize the history of the game in the search driver.
*/

pub fn setup_search(state: &mut SearchState) {
    init_move_lists(&mut state.sorted_move_order);
    state.list_inherited = [0; 62];
    create_eval_info(UNINITIALIZED_EVAL, UNSOLVED_POSITION, 0 as i32,
                     0.0f64, 0 as i32, 0 as i32);
    state.negate_eval = 0;
}
/*
   DISC_COUNT
   side_to_move = the player whose disks are to be counted
   Returns the number of disks of a specified color.
*/

pub const fn disc_count(side_to_move: i32, board_: & crate::src::globals::Board) -> i32 {
    let mut j = 0;
    let mut sum = 0;
    let mut i = 1;
    while i <= 8 {
        j = 10 * i + 1;
        while j <= 10 * i + 8 {
            if board_[j] == side_to_move { sum += 1 }
            j += 1
        }
        i += 1
    }
    sum
}
/*
   SORT_MOVES
   Sort the available in decreasing order based on the results
   from a shallow search.
*/

pub fn sort_moves(list_size: i32, moves: &mut MovesState, search: &SearchState) {
    loop {
        let mut modified = 0;
        let mut i = 0;
        while i < list_size - 1 {
            if search.evals[moves.disks_played as usize][moves.move_list[moves.disks_played as usize][i as usize] as usize] <
                search.evals[moves.disks_played as usize][moves.move_list[moves.disks_played as usize][(i + 1) as usize] as usize] {
                modified = 1;
                let temp_move = moves.move_list[moves.disks_played as usize][i as usize];
                moves.move_list[moves.disks_played as usize][i as usize] = moves.move_list[moves.disks_played as usize][(i + 1 as i32) as usize];
                moves.move_list[moves.disks_played as usize][(i + 1 as i32) as usize] = temp_move
            }
            i += 1
        }
        if modified == 0 { break; }
    };
}
/*
  SELECT_MOVE
  Finds the best move in the move list neglecting the first FIRST moves.
  Moves this move to the front of the sub-list.
*/

pub fn select_move(first: i32, list_size: i32, search_state_: &mut SearchState, moves_state_: &mut MovesState) -> i8 {
    let mut temp_move = 0;
    let mut best = 0;
    let mut best_eval: i32 = 0;
    best = first;
    best_eval =
        search_state_.evals[moves_state_.disks_played as
            usize][moves_state_.move_list[moves_state_.disks_played as usize][first as usize] as
            usize];
    let mut i = first + 1;
    while i < list_size {
        if search_state_.evals[moves_state_.disks_played as usize][moves_state_.move_list[moves_state_.disks_played as usize][i as usize] as usize] > best_eval {
            best = i;
            best_eval = search_state_.evals[moves_state_.disks_played as usize][moves_state_.move_list[moves_state_.disks_played as usize][i as usize] as usize]
        }
        i += 1
    }
    if best != first {
        temp_move = moves_state_.move_list[moves_state_.disks_played as usize][first as usize];
        moves_state_.move_list[moves_state_.disks_played as usize][first as usize] = moves_state_.move_list[moves_state_.disks_played as usize][best as usize];
        moves_state_.move_list[moves_state_.disks_played as usize][best as usize] = temp_move
    }
    return moves_state_.move_list[moves_state_.disks_played as usize][first as usize];
}
/*
  FLOAT_MOVE
  "Float" a move which is believed to be good to the top
  of the list of available moves.
  Return 1 if the move was found, 0 otherwise.
*/

pub fn float_move(move_0: i8, list_size: i32, state: &mut MovesState) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < list_size {
        if state.move_list[state.disks_played as usize][i as usize] == move_0 {
            j = i;
            while j >= 1 as i32 {
                state.move_list[state.disks_played as usize][j as usize] = state.move_list[state.disks_played as usize][(j - 1) as usize];
                j -= 1
            }
            state.move_list[state.disks_played as usize][0] = move_0;
            return 1 as i32
        }
        i += 1
    }
    return 0;
}
/*
   STORE_PV
   Saves the principal variation (the first row of the PV matrix).
*/

pub fn store_pv(pv_buffer: &mut [i8], depth_buffer: &mut i32, state: &BoardState) {
    let mut i = 0;
    while i < state.pv_depth[0] {
        pv_buffer[(i as usize)] = state.pv[0][i as usize];
        i += 1
    }
    *depth_buffer = state.pv_depth[0];
}
/*
   RESTORE_PV
   Put the stored principal variation back into the PV matrix.
*/

pub fn restore_pv(pv_buffer: &[i8], depth_buffer: i32, state: &mut BoardState) {
    let mut i: i32 = 0;
    i = 0;
    while i < depth_buffer {
        state.pv[0][i as usize] = pv_buffer[i as usize];
        i += 1
    }
    state.pv_depth[0] = depth_buffer;
}
/*
  SET_PONDER_MOVE
  CLEAR_PONDER_MOVE
  GET_PONDER_MOVE
  A value of 0 denotes a normal search while anything else means
  that the search is performed given that the move indicated has
  been made.
*/
impl SearchState {
    pub fn set_ponder_move(&mut self, move_0: i8) {
        self.pondered_move = move_0;
    }

    pub fn clear_ponder_move(&mut self) {
        self.pondered_move = 0;
    }

    pub fn get_ponder_move(&self) -> i8 {
        return self.pondered_move;
    }
}

/*
  CREATE_EVAL_INFO
  Creates a result descriptor given all the information available
  about the last search.
*/

pub fn create_eval_info(in_type: EvalType, in_res: EvalResult, in_score: i32, in_conf: f64, in_depth: i32, in_book: i32) -> EvaluationType {
    EvaluationType {
        type_0: in_type,
        res: in_res,
        score: in_score,
        confidence: in_conf,
        search_depth: in_depth,
        is_book: in_book,
    }
}
/*
  PRODUCE_COMPACT_EVAL
  Converts a result descriptor into a number between -99.99 and 99.99 a la GGS.
*/

pub fn produce_compact_eval(eval_info: EvaluationType) -> f64 {
    let mut eval: f64 = 0.;
        {
            let current_block_17: u64;
            match eval_info.type_0 as u32 {
                0 => {
                    /*
        eval = eval_info.search_depth + logistic_map( eval_info.score );
        if ( eval_info.is_book )
          eval = -eval;
          */
                    eval = eval_info.score as f64 / 128.0f64;
                    return eval
                }
                1 => { return eval_info.score as f64 / 128.0f64 }
                2 => {
                    match eval_info.res as u32 {
                        0 => {
                            if eval_info.score >
                                2 as i32 * 128 as i32 {
                                /* Win by more than 2 */
                                return eval_info.score as f64 /
                                    128.0f64 - 0.01f64
                            } else { return 1.99f64 }
                        }
                        1 => { return 0.0f64 }
                        2 => {
                            if eval_info.score <
                                -(2 as i32) * 128 as i32 {
                                /* Loss by more than 2 */
                                return eval_info.score as f64 /
                                    128.0f64 + 0.01f64
                            } else { return -1.99f64 }
                        }
                        3 => { return 0.0f64 }
                        _ => { }
                    }
                    current_block_17 = 13171200747117244060;
                }
                3 => { current_block_17 = 13171200747117244060; }
                4 | 5 | 7 | 6 | 8 => {
                    current_block_17 = 12692146724533637300;
                }
                _ => {
                    return 0.0f64;
                }
            }
            match current_block_17 {
                13171200747117244060 => {
                    match eval_info.res as u32 {
                        0 => { return 1.0f64 + eval_info.confidence }
                        1 => { return -1.0f64 + eval_info.confidence }
                        2 => { return -1.0f64 - eval_info.confidence }
                        3 => {
                            return eval_info.score as f64 /
                                128.0f64
                        }
                        _ => { }
                    }
                }
                _ => { }
            }
            return 0.0f64
        }
    return 0.0f64;
    /* This statement shouldn't be reached */
}
/*
  SET_CURRENT_EVAL
  GET_CURRENT_EVAL
  NEGATE_CURRENT_EVAL
  Mutator and accessor functions for the global variable
  holding the last available position evaluation.
*/
impl SearchState {
    pub fn set_current_eval(&mut self, eval: EvaluationType) {
        self.last_eval = eval;
        if self.negate_eval != 0 {
            self.last_eval.score = -self.last_eval.score;
            if self.last_eval.res as u32 ==
                WON_POSITION as i32 as u32 {
                self.last_eval.res = LOST_POSITION
            } else if self.last_eval.res as u32 ==
                LOST_POSITION as i32 as u32 {
                self.last_eval.res = WON_POSITION
            }
        };
    }

    pub fn get_current_eval(&self) -> EvaluationType {
        return self.last_eval;
    }

    pub fn negate_current_eval(&mut self, negate: i32) {
        self.negate_eval = negate;
    }
}

/*
  HASH_EXPAND_PV
  Pad the existing PV with the move sequence suggested by the hash table.
*/

pub fn hash_expand_pv(mut side_to_move: i32, mode: i32, flags: i32, max_selectivity: i32,
                           board_state_: &mut BoardState, hash_state_: &mut HashState, moves_state_: &mut MovesState, flip_stack: &mut FlipStack) {
    let mut pass_count = 0;
    let mut new_pv_depth = 0;
    let mut new_pv = [0; 61];
    let mut new_side_to_move = [0; 61];
    let mut entry = HashEntry::new();
    determine_hash_values(side_to_move, &board_state_.board, hash_state_);
    new_pv_depth = 0;
    pass_count = 0;
    while pass_count < 2 {
        new_side_to_move[new_pv_depth] = side_to_move;
        if new_pv_depth < board_state_.pv_depth[0] as usize &&
            new_pv_depth == 0 {
            if board_state_.board[board_state_.pv[0][new_pv_depth] as usize] == 1 &&
                make_move(side_to_move, board_state_.pv[0][new_pv_depth], 1, moves_state_, board_state_, hash_state_, flip_stack) != 0 {
                new_pv[new_pv_depth] =
                    board_state_.pv[0][new_pv_depth];
                new_pv_depth += 1;
                pass_count = 0
            } else {
                hash_state_.hash1 ^= hash_state_.hash_flip_color1;
                hash_state_.hash2 ^= hash_state_.hash_flip_color2;
                pass_count += 1
            }
        } else {
            find_hash(&mut entry, mode, hash_state_);
            if entry.draft as i32 != 0 as i32 &&
                entry.flags as i32 & flags != 0 &&
                entry.selectivity as i32 <= max_selectivity &&
                board_state_.board[entry.move_0[0] as usize] == 1 &&
                make_move(side_to_move, entry.move_0[0] , 1, moves_state_, board_state_, hash_state_, flip_stack) != 0 {
                new_pv[new_pv_depth] = entry.move_0[0];
                new_pv_depth += 1;
                pass_count = 0
            } else {
                hash_state_.hash1 ^= hash_state_.hash_flip_color1;
                hash_state_.hash2 ^= hash_state_.hash_flip_color2;
                pass_count += 1
            }
        }
        side_to_move = 2 - side_to_move
    }
    let mut i = new_pv_depth as i32 - 1 as i32;
    while i >= 0 {
        let side_to_move = new_side_to_move[i as usize];
        let move_0 = new_pv[i as usize];
        {
            unmake_move(side_to_move, move_0, &mut board_state_.board, moves_state_, hash_state_, flip_stack);
        };
        i -= 1
    }
    let mut i = 0;
    while i < new_pv_depth {
        board_state_.pv[0][i] = new_pv[i];
        i += 1
    }
    board_state_.pv_depth[0] = new_pv_depth as i32;
}


/*
  COMPLETE_PV
  Complete the principal variation with passes (if any there are any).
*/

pub fn complete_pv<FE: FrontEnd>(mut side_to_move: i32, search_state_: &mut SearchState, board_state_: &mut BoardState, flip_stack: &mut FlipStack, hash_state_: &mut HashState, moves_state_: &mut MovesState) {
    let mut actual_side_to_move = [0; 60];
    search_state_.full_pv_depth = 0;
    let mut i = 0;
    while i < board_state_.pv_depth[0] {
        if make_move(side_to_move, board_state_.pv[0][i as usize], 1, moves_state_, board_state_, hash_state_, flip_stack) != 0 {
            actual_side_to_move[i as usize] = side_to_move;
            search_state_.full_pv[search_state_.full_pv_depth as usize] = board_state_.pv[0][i as usize];
            search_state_.full_pv_depth += 1
        } else {
            search_state_.full_pv[search_state_.full_pv_depth as usize] = -(1);
            search_state_.full_pv_depth += 1;
            side_to_move = 0 + 2 - side_to_move;
            if make_move(side_to_move, board_state_.pv[0][i as usize], 1, moves_state_, board_state_, hash_state_, flip_stack) != 0 {
                actual_side_to_move[i as usize] = side_to_move;
                search_state_.full_pv[search_state_.full_pv_depth as usize] = board_state_.pv[0][i as usize];
                search_state_.full_pv_depth += 1
            } else {
                let pv_0_depth: i32 = board_state_.pv_depth[0];
                let pv_0: &[i8; 64] = &board_state_.pv[0];
                FE::handle_fatal_pv_error(i, pv_0_depth, pv_0);
            }
        }
        side_to_move = 0 as i32 + 2 as i32 - side_to_move;
        i += 1
    }
    i = board_state_.pv_depth[0] - 1 as i32;
    while i >= 0 as i32 {
        let side_to_move = actual_side_to_move[i as usize];
        let move_0 = board_state_.pv[0][i as usize];
        {
            unmake_move(side_to_move, move_0, &mut board_state_.board, moves_state_, hash_state_, flip_stack);
        };
        i -= 1
    };
}
