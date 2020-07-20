use crate::src::moves::{unmake_move, make_move, disks_played, move_list};
use crate::src::hash::{hash_flip_color2, hash2, hash_flip_color1, hash1, find_hash, determine_hash_values, HashEntry};
use crate::src::globals::{board, Board, pv_depth, pv};
use crate::src::error::fatal_error;
use crate::src::stubs::{printf, puts};
use crate::src::counter::CounterType;
use crate::src::zebra::EvaluationType;
pub use ::engine::src::search::*;

/*
   File:          search.c

   Created:       July 1, 1997

   Modified:      January 2, 2003

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      Common search routines and variables.
*/
/* Global variables */

/* Local variables */
static mut pondered_move: i32 = 0 as i32;
static mut negate_eval: i32 = 0;
static mut last_eval: EvaluationType =
    EvaluationType{type_0: MIDGAME_EVAL,
                   res: WON_POSITION,
                   score: 0,
                   confidence: 0.,
                   search_depth: 0,
                   is_book: 0,};
/*
  INIT_MOVE_LISTS
  Initalize the self-organizing move lists.
*/
unsafe fn init_move_lists() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0 as i32;
    while i <= 60 as i32 {
        j = 0 as i32;
        while j < 60 as i32 {
            sorted_move_order[i as usize][j as usize] =
                position_list[j as usize];
            j += 1
        }
        i += 1
    }
    i = 0 as i32;
    while i <= 60 as i32 {
        list_inherited[i as usize] = 0 as i32;
        i += 1
    };
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

pub unsafe fn inherit_move_lists(mut stage: i32) {
    let mut i: i32 = 0;
    let mut last: i32 = 0;

    // FIXME
    //  Index out of bounds here - reproducer:
    //  cargo run  -- -l 20 10 0 20 10 0 -r 0
    if list_inherited[stage as usize] != 0 { return }
    list_inherited[stage as usize] = 1 as i32;
    if stage == 0 as i32 { return }
    last = stage - 2 as i32;
    while last >= 0 as i32 && list_inherited[last as usize] == 0 {
        last -= 2 as i32
    }
    if last < 0 as i32 { return }
    i = 0 as i32;
    while i < 60 as i32 {
        sorted_move_order[stage as usize][i as usize] =
            sorted_move_order[last as usize][i as usize];
        i += 1
    };
}
/*
  REORDER_MOVE_LIST
  Move the empty squares to the front of the move list.  Empty squares
  high up in the ranking are kept in place as they probably are empty
  in many variations in the tree.
*/

pub unsafe fn reorder_move_list(mut stage: i32) {
    let dont_touch = 24 as i32;
    let mut i: i32 = 0;
    let mut move_0: i32 = 0;
    let mut empty_pos: i32 = 0;
    let mut nonempty_pos: i32 = 0;
    let mut empty_buffer: [i32; 60] = [0; 60];
    let mut nonempty_buffer: [i32; 60] = [0; 60];
    empty_pos = 0 as i32;
    i = 0 as i32;
    while i < 60 as i32 {
        move_0 = sorted_move_order[stage as usize][i as usize];
        if board[move_0 as usize] == 1 as i32 || i < dont_touch {
            empty_buffer[empty_pos as usize] = move_0;
            empty_pos += 1
        }
        i += 1
    }
    nonempty_pos = 60 as i32 - 1 as i32;
    i = 60 as i32 - 1 as i32;
    while i >= 0 as i32 {
        move_0 = sorted_move_order[stage as usize][i as usize];
        if board[move_0 as usize] != 1 as i32 && i >= dont_touch {
            nonempty_buffer[nonempty_pos as usize] = move_0;
            nonempty_pos -= 1
        }
        i -= 1
    }
    i = 0 as i32;
    while i < empty_pos {
        sorted_move_order[stage as usize][i as usize] =
            empty_buffer[i as usize];
        i += 1
    }
    i = empty_pos;
    while i < 60 as i32 {
        sorted_move_order[stage as usize][i as usize] =
            nonempty_buffer[i as usize];
        i += 1
    };
}
/*
   SETUP_SEARCH
   Initialize the history of the game in the search driver.
*/

pub unsafe fn setup_search() {
    init_move_lists();
    create_eval_info(UNINITIALIZED_EVAL, UNSOLVED_POSITION, 0 as i32,
                     0.0f64, 0 as i32, 0 as i32);
    negate_eval = 0 as i32;
}
/*
   DISC_COUNT
   side_to_move = the player whose disks are to be counted
   Returns the number of disks of a specified color.
*/

pub unsafe fn disc_count(mut side_to_move: i32)
 -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut sum: i32 = 0;
    sum = 0 as i32;
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 10 as i32 * i + 1 as i32;
        while j <= 10 as i32 * i + 8 as i32 {
            if board[j as usize] == side_to_move { sum += 1 }
            j += 1
        }
        i += 1
    }
    return sum;
}
/*
   SORT_MOVES
   Sort the available in decreasing order based on the results
   from a shallow search.
*/

pub unsafe fn sort_moves(mut list_size: i32) {
    let mut i: i32 = 0;
    let mut modified: i32 = 0;
    let mut temp_move: i32 = 0;
    loop  {
        modified = 0 as i32;
        i = 0 as i32;
        while i < list_size - 1 as i32 {
            if evals[disks_played as
                         usize][move_list[disks_played as usize][i as usize]
                                    as usize] <
                   evals[disks_played as
                             usize][move_list[disks_played as
                                                  usize][(i +
                                                              1 as
                                                                  i32)
                                                             as usize] as
                                        usize] {
                modified = 1 as i32;
                temp_move = move_list[disks_played as usize][i as usize];
                move_list[disks_played as usize][i as usize] =
                    move_list[disks_played as
                                  usize][(i + 1 as i32) as usize];
                move_list[disks_played as
                              usize][(i + 1 as i32) as usize] =
                    temp_move
            }
            i += 1
        }
        if !(modified != 0) { break ; }
    };
}
/*
  SELECT_MOVE
  Finds the best move in the move list neglecting the first FIRST moves.
  Moves this move to the front of the sub-list.
*/

pub unsafe fn select_move(mut first: i32,
                                     mut list_size: i32)
 -> i32 {
    let mut i: i32 = 0;
    let mut temp_move: i32 = 0;
    let mut best: i32 = 0;
    let mut best_eval: i32 = 0;
    best = first;
    best_eval =
        evals[disks_played as
                  usize][move_list[disks_played as usize][first as usize] as
                             usize];
    i = first + 1 as i32;
    while i < list_size {
        if evals[disks_played as
                     usize][move_list[disks_played as usize][i as usize] as
                                usize] > best_eval {
            best = i;
            best_eval =
                evals[disks_played as
                          usize][move_list[disks_played as usize][i as usize]
                                     as usize]
        }
        i += 1
    }
    if best != first {
        temp_move = move_list[disks_played as usize][first as usize];
        move_list[disks_played as usize][first as usize] =
            move_list[disks_played as usize][best as usize];
        move_list[disks_played as usize][best as usize] = temp_move
    }
    return move_list[disks_played as usize][first as usize];
}
/*
  FLOAT_MOVE
  "Float" a move which is believed to be good to the top
  of the list of available moves.
  Return 1 if the move was found, 0 otherwise.
*/

pub unsafe fn float_move(mut move_0: i32,
                                    mut list_size: i32)
 -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0 as i32;
    while i < list_size {
        if move_list[disks_played as usize][i as usize] == move_0 {
            j = i;
            while j >= 1 as i32 {
                move_list[disks_played as usize][j as usize] =
                    move_list[disks_played as
                                  usize][(j - 1 as i32) as usize];
                j -= 1
            }
            move_list[disks_played as usize][0 as i32 as usize] =
                move_0;
            return 1 as i32
        }
        i += 1
    }
    return 0 as i32;
}
/*
   STORE_PV
   Saves the principal variation (the first row of the PV matrix).
*/

pub unsafe fn store_pv(mut pv_buffer: *mut i32,
                                  mut depth_buffer: *mut i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < pv_depth[0 as i32 as usize] {
        *pv_buffer.offset(i as isize) =
            pv[0 as i32 as usize][i as usize];
        i += 1
    }
    *depth_buffer = pv_depth[0 as i32 as usize];
}
/*
   RESTORE_PV
   Put the stored principal variation back into the PV matrix.
*/

pub unsafe fn restore_pv(mut pv_buffer: *mut i32,
                                    mut depth_buffer: i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < depth_buffer {
        pv[0 as i32 as usize][i as usize] =
            *pv_buffer.offset(i as isize);
        i += 1
    }
    pv_depth[0 as i32 as usize] = depth_buffer;
}
/*
  CLEAR_PV
  Clears the principal variation.
*/

pub unsafe fn clear_pv() {
    pv_depth[0 as i32 as usize] = 0 as i32;
}
/*
  COMPLETE_PV
  Complete the principal variation with passes (if any there are any).
*/

pub unsafe fn complete_pv(mut side_to_move: i32) {
    let mut i: i32 = 0;
    let mut actual_side_to_move: [i32; 60] = [0; 60];
    full_pv_depth = 0 as i32;
    i = 0 as i32;
    while i < pv_depth[0 as i32 as usize] {
        if make_move(side_to_move, pv[0 as i32 as usize][i as usize],
                     1 as i32) != 0 {
            actual_side_to_move[i as usize] = side_to_move;
            full_pv[full_pv_depth as usize] =
                pv[0 as i32 as usize][i as usize];
            full_pv_depth += 1
        } else {
            full_pv[full_pv_depth as usize] = -(1 as i32);
            full_pv_depth += 1;
            side_to_move = 0 as i32 + 2 as i32 - side_to_move;
            if make_move(side_to_move,
                         pv[0 as i32 as usize][i as usize],
                         1 as i32) != 0 {
                actual_side_to_move[i as usize] = side_to_move;
                full_pv[full_pv_depth as usize] =
                    pv[0 as i32 as usize][i as usize];
                full_pv_depth += 1
            } else {
                let mut j: i32 = 0;
                printf(b"pv_depth[0] = %d\n\x00" as *const u8 as
                           *const i8,
                       pv_depth[0 as i32 as usize]);
                j = 0 as i32;
                while j < pv_depth[0 as i32 as usize] {
                    printf(b"%c%c \x00" as *const u8 as *const i8,
                           'a' as i32 +
                               pv[0 as i32 as usize][j as usize] %
                                   10 as i32 - 1 as i32,
                           '0' as i32 +
                               pv[0 as i32 as usize][j as usize] /
                                   10 as i32);
                    j += 1
                }
                puts(b"\x00" as *const u8 as *const i8);
                printf(b"i=%d\n\x00" as *const u8 as *const i8, i);
                fatal_error(b"Error in PV completion\x00" as *const u8 as
                                *const i8);
            }
        }
        side_to_move = 0 as i32 + 2 as i32 - side_to_move;
        i += 1
    }
    i = pv_depth[0 as i32 as usize] - 1 as i32;
    while i >= 0 as i32 {
        unmake_move(actual_side_to_move[i as usize],
                    pv[0 as i32 as usize][i as usize]);
        i -= 1
    };
}
/*
  HASH_EXPAND_PV
  Pad the existing PV with the move sequence suggested by the hash table.
*/

pub unsafe fn hash_expand_pv(mut side_to_move: i32,
                                        mut mode: i32,
                                        mut flags: i32,
                                        mut max_selectivity: i32) {
    let mut i: i32 = 0;
    let mut pass_count: i32 = 0;
    let mut new_pv_depth: i32 = 0;
    let mut new_pv: [i32; 61] = [0; 61];
    let mut new_side_to_move: [i32; 61] = [0; 61];
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    determine_hash_values(side_to_move, board.as_mut_ptr());
    new_pv_depth = 0 as i32;
    pass_count = 0 as i32;
    while pass_count < 2 as i32 {
        new_side_to_move[new_pv_depth as usize] = side_to_move;
        if new_pv_depth < pv_depth[0 as i32 as usize] &&
               new_pv_depth == 0 as i32 {
            if board[pv[0 as i32 as usize][new_pv_depth as usize] as
                         usize] == 1 as i32 &&
                   make_move(side_to_move,
                             pv[0 as i32 as
                                    usize][new_pv_depth as usize],
                             1 as i32) != 0 {
                new_pv[new_pv_depth as usize] =
                    pv[0 as i32 as usize][new_pv_depth as usize];
                new_pv_depth += 1;
                pass_count = 0 as i32
            } else {
                hash1 ^= hash_flip_color1;
                hash2 ^= hash_flip_color2;
                pass_count += 1
            }
        } else {
            find_hash(&mut entry, mode);
            if entry.draft as i32 != 0 as i32 &&
                   entry.flags as i32 & flags != 0 &&
                   entry.selectivity as i32 <= max_selectivity &&
                   board[entry.move_0[0 as i32 as usize] as usize] ==
                       1 as i32 &&
                   make_move(side_to_move,
                             entry.move_0[0 as i32 as usize],
                             1 as i32) != 0 {
                new_pv[new_pv_depth as usize] =
                    entry.move_0[0 as i32 as usize];
                new_pv_depth += 1;
                pass_count = 0 as i32
            } else {
                hash1 ^= hash_flip_color1;
                hash2 ^= hash_flip_color2;
                pass_count += 1
            }
        }
        side_to_move = 0 as i32 + 2 as i32 - side_to_move
    }
    i = new_pv_depth - 1 as i32;
    while i >= 0 as i32 {
        unmake_move(new_side_to_move[i as usize], new_pv[i as usize]);
        i -= 1
    }
    i = 0 as i32;
    while i < new_pv_depth {
        pv[0 as i32 as usize][i as usize] = new_pv[i as usize];
        i += 1
    }
    pv_depth[0 as i32 as usize] = new_pv_depth;
}
/*
  SET_PONDER_MOVE
  CLEAR_PONDER_MOVE
  GET_PONDER_MOVE
  A value of 0 denotes a normal search while anything else means
  that the search is performed given that the move indicated has
  been made.
*/

pub unsafe fn set_ponder_move(mut move_0: i32) {
    pondered_move = move_0;
}

pub unsafe fn clear_ponder_move() {
    pondered_move = 0 as i32;
}

pub unsafe fn get_ponder_move() -> i32 {
    return pondered_move;
}
/*
  CREATE_EVAL_INFO
  Creates a result descriptor given all the information available
  about the last search.
*/

pub unsafe fn create_eval_info(mut in_type: EvalType,
                                          mut in_res: EvalResult,
                                          mut in_score: i32,
                                          mut in_conf: f64,
                                          mut in_depth: i32,
                                          mut in_book: i32)
 -> EvaluationType {
    let mut out =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    out.type_0 = in_type;
    out.res = in_res;
    out.score = in_score;
    out.confidence = in_conf;
    out.search_depth = in_depth;
    out.is_book = in_book;
    return out;
}
/*
  PRODUCE_COMPACT_EVAL
  Converts a result descriptor into a number between -99.99 and 99.99 a la GGS.
*/

pub unsafe fn produce_compact_eval(mut eval_info: EvaluationType)
 -> f64 {
    let mut eval: f64 = 0.;
    's_97:
        {
            let mut current_block_17: u64;
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
                _ => { break 's_97 ; }
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

pub unsafe fn set_current_eval(mut eval: EvaluationType) {
    last_eval = eval;
    if negate_eval != 0 {
        last_eval.score = -last_eval.score;
        if last_eval.res as u32 ==
               WON_POSITION as i32 as u32 {
            last_eval.res = LOST_POSITION
        } else if last_eval.res as u32 ==
                      LOST_POSITION as i32 as u32 {
            last_eval.res = WON_POSITION
        }
    };
}

pub unsafe fn get_current_eval() -> EvaluationType {
    return last_eval;
}

pub unsafe fn negate_current_eval(mut negate: i32) {
    negate_eval = negate;
}
