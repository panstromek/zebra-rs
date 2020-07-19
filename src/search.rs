/*
   File:           globals.h

   Created:        June 30, 1997

   Modified:       January 8, 2000

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Global state variables.
*/
use crate::src::libc;
use crate::src::moves::{unmake_move, make_move, disks_played, move_list};
use crate::src::hash::{hash_flip_color2, hash2, hash_flip_color1, hash1, find_hash, determine_hash_values, HashEntry};
use crate::src::globals::{board, Board, pv_depth, pv};
use crate::src::error::fatal_error;
use crate::src::stubs::{printf, puts};
use crate::src::counter::CounterType;
use crate::src::zebra::EvaluationType;

pub type EvalType = libc::c_uint;
pub const UNINITIALIZED_EVAL: EvalType = 8;
pub const INTERRUPTED_EVAL: EvalType = 7;
pub const UNDEFINED_EVAL: EvalType = 6;
pub const PASS_EVAL: EvalType = 5;
pub const FORCED_EVAL: EvalType = 4;
pub const SELECTIVE_EVAL: EvalType = 3;
pub const WLD_EVAL: EvalType = 2;
pub const EXACT_EVAL: EvalType = 1;
pub const MIDGAME_EVAL: EvalType = 0;
pub type EvalResult = libc::c_uint;
pub const UNSOLVED_POSITION: EvalResult = 3;
pub const LOST_POSITION: EvalResult = 2;
pub const DRAWN_POSITION: EvalResult = 1;
pub const WON_POSITION: EvalResult = 0;

/*
   File:          search.c

   Created:       July 1, 1997

   Modified:      January 2, 2003

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      Common search routines and variables.
*/
/* Global variables */

pub static mut total_time: libc::c_double = 0.;

pub static mut root_eval: libc::c_int = 0;

pub static mut force_return: libc::c_int = 0;

pub static mut full_pv_depth: libc::c_int = 0;

pub static mut full_pv: [libc::c_int; 120] = [0; 120];

pub static mut list_inherited: [libc::c_int; 61] = [0; 61];

pub static mut sorted_move_order: [[libc::c_int; 64]; 64] = [[0; 64]; 64];
/* 61*60 used */

pub static mut evals: [Board; 61] = [[0; 128]; 61];

pub static mut nodes: CounterType = CounterType{hi: 0, lo: 0,};

pub static mut total_nodes: CounterType = CounterType{hi: 0, lo: 0,};

pub static mut evaluations: CounterType = CounterType{hi: 0, lo: 0,};

pub static mut total_evaluations: CounterType = CounterType{hi: 0, lo: 0,};
/* When no other information is available, JCW's endgame
   priority order is used also in the midgame. */

pub static mut position_list: [libc::c_int; 100] =
    [11 as libc::c_int, 18 as libc::c_int, 81 as libc::c_int,
     88 as libc::c_int, 13 as libc::c_int, 16 as libc::c_int,
     31 as libc::c_int, 38 as libc::c_int, 61 as libc::c_int,
     68 as libc::c_int, 83 as libc::c_int, 86 as libc::c_int,
     33 as libc::c_int, 36 as libc::c_int, 63 as libc::c_int,
     66 as libc::c_int, 14 as libc::c_int, 15 as libc::c_int,
     41 as libc::c_int, 48 as libc::c_int, 51 as libc::c_int,
     58 as libc::c_int, 84 as libc::c_int, 85 as libc::c_int,
     34 as libc::c_int, 35 as libc::c_int, 43 as libc::c_int,
     46 as libc::c_int, 53 as libc::c_int, 56 as libc::c_int,
     64 as libc::c_int, 65 as libc::c_int, 24 as libc::c_int,
     25 as libc::c_int, 42 as libc::c_int, 47 as libc::c_int,
     52 as libc::c_int, 57 as libc::c_int, 74 as libc::c_int,
     75 as libc::c_int, 23 as libc::c_int, 26 as libc::c_int,
     32 as libc::c_int, 37 as libc::c_int, 62 as libc::c_int,
     67 as libc::c_int, 73 as libc::c_int, 76 as libc::c_int,
     12 as libc::c_int, 17 as libc::c_int, 21 as libc::c_int,
     28 as libc::c_int, 71 as libc::c_int, 78 as libc::c_int,
     82 as libc::c_int, 87 as libc::c_int, 22 as libc::c_int,
     27 as libc::c_int, 72 as libc::c_int, 77 as libc::c_int,
     44 as libc::c_int, 45 as libc::c_int, 54 as libc::c_int,
     45 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int, 2 as libc::c_int,
     3 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int,
     7 as libc::c_int, 8 as libc::c_int, 9 as libc::c_int, 19 as libc::c_int,
     29 as libc::c_int, 39 as libc::c_int, 49 as libc::c_int,
     59 as libc::c_int, 69 as libc::c_int, 79 as libc::c_int,
     89 as libc::c_int, 10 as libc::c_int, 20 as libc::c_int,
     30 as libc::c_int, 40 as libc::c_int, 50 as libc::c_int,
     60 as libc::c_int, 70 as libc::c_int, 80 as libc::c_int,
     90 as libc::c_int, 91 as libc::c_int, 92 as libc::c_int,
     93 as libc::c_int, 94 as libc::c_int, 95 as libc::c_int,
     96 as libc::c_int, 97 as libc::c_int, 98 as libc::c_int,
     99 as libc::c_int];
/* Local variables */
static mut pondered_move: libc::c_int = 0 as libc::c_int;
static mut negate_eval: libc::c_int = 0;
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
unsafe extern "C" fn init_move_lists() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <= 60 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 60 as libc::c_int {
            sorted_move_order[i as usize][j as usize] =
                position_list[j as usize];
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i <= 60 as libc::c_int {
        list_inherited[i as usize] = 0 as libc::c_int;
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

pub unsafe extern "C" fn inherit_move_lists(mut stage: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut last: libc::c_int = 0;

    // FIXME
    //  Index out of bounds here - reproducer:
    //  cargo run  -- -l 20 10 0 20 10 0 -r 0
    if list_inherited[stage as usize] != 0 { return }
    list_inherited[stage as usize] = 1 as libc::c_int;
    if stage == 0 as libc::c_int { return }
    last = stage - 2 as libc::c_int;
    while last >= 0 as libc::c_int && list_inherited[last as usize] == 0 {
        last -= 2 as libc::c_int
    }
    if last < 0 as libc::c_int { return }
    i = 0 as libc::c_int;
    while i < 60 as libc::c_int {
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

pub unsafe extern "C" fn reorder_move_list(mut stage: libc::c_int) {
    let dont_touch = 24 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut move_0: libc::c_int = 0;
    let mut empty_pos: libc::c_int = 0;
    let mut nonempty_pos: libc::c_int = 0;
    let mut empty_buffer: [libc::c_int; 60] = [0; 60];
    let mut nonempty_buffer: [libc::c_int; 60] = [0; 60];
    empty_pos = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 60 as libc::c_int {
        move_0 = sorted_move_order[stage as usize][i as usize];
        if board[move_0 as usize] == 1 as libc::c_int || i < dont_touch {
            empty_buffer[empty_pos as usize] = move_0;
            empty_pos += 1
        }
        i += 1
    }
    nonempty_pos = 60 as libc::c_int - 1 as libc::c_int;
    i = 60 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        move_0 = sorted_move_order[stage as usize][i as usize];
        if board[move_0 as usize] != 1 as libc::c_int && i >= dont_touch {
            nonempty_buffer[nonempty_pos as usize] = move_0;
            nonempty_pos -= 1
        }
        i -= 1
    }
    i = 0 as libc::c_int;
    while i < empty_pos {
        sorted_move_order[stage as usize][i as usize] =
            empty_buffer[i as usize];
        i += 1
    }
    i = empty_pos;
    while i < 60 as libc::c_int {
        sorted_move_order[stage as usize][i as usize] =
            nonempty_buffer[i as usize];
        i += 1
    };
}
/*
   SETUP_SEARCH
   Initialize the history of the game in the search driver.
*/

pub unsafe extern "C" fn setup_search() {
    init_move_lists();
    create_eval_info(UNINITIALIZED_EVAL, UNSOLVED_POSITION, 0 as libc::c_int,
                     0.0f64, 0 as libc::c_int, 0 as libc::c_int);
    negate_eval = 0 as libc::c_int;
}
/*
   DISC_COUNT
   side_to_move = the player whose disks are to be counted
   Returns the number of disks of a specified color.
*/

pub unsafe extern "C" fn disc_count(mut side_to_move: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    sum = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 10 as libc::c_int * i + 1 as libc::c_int;
        while j <= 10 as libc::c_int * i + 8 as libc::c_int {
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

pub unsafe extern "C" fn sort_moves(mut list_size: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut modified: libc::c_int = 0;
    let mut temp_move: libc::c_int = 0;
    loop  {
        modified = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < list_size - 1 as libc::c_int {
            if evals[disks_played as
                         usize][move_list[disks_played as usize][i as usize]
                                    as usize] <
                   evals[disks_played as
                             usize][move_list[disks_played as
                                                  usize][(i +
                                                              1 as
                                                                  libc::c_int)
                                                             as usize] as
                                        usize] {
                modified = 1 as libc::c_int;
                temp_move = move_list[disks_played as usize][i as usize];
                move_list[disks_played as usize][i as usize] =
                    move_list[disks_played as
                                  usize][(i + 1 as libc::c_int) as usize];
                move_list[disks_played as
                              usize][(i + 1 as libc::c_int) as usize] =
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

pub unsafe extern "C" fn select_move(mut first: libc::c_int,
                                     mut list_size: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut temp_move: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut best_eval: libc::c_int = 0;
    best = first;
    best_eval =
        evals[disks_played as
                  usize][move_list[disks_played as usize][first as usize] as
                             usize];
    i = first + 1 as libc::c_int;
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

pub unsafe extern "C" fn float_move(mut move_0: libc::c_int,
                                    mut list_size: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < list_size {
        if move_list[disks_played as usize][i as usize] == move_0 {
            j = i;
            while j >= 1 as libc::c_int {
                move_list[disks_played as usize][j as usize] =
                    move_list[disks_played as
                                  usize][(j - 1 as libc::c_int) as usize];
                j -= 1
            }
            move_list[disks_played as usize][0 as libc::c_int as usize] =
                move_0;
            return 1 as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*
   STORE_PV
   Saves the principal variation (the first row of the PV matrix).
*/

pub unsafe extern "C" fn store_pv(mut pv_buffer: *mut libc::c_int,
                                  mut depth_buffer: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < pv_depth[0 as libc::c_int as usize] {
        *pv_buffer.offset(i as isize) =
            pv[0 as libc::c_int as usize][i as usize];
        i += 1
    }
    *depth_buffer = pv_depth[0 as libc::c_int as usize];
}
/*
   RESTORE_PV
   Put the stored principal variation back into the PV matrix.
*/

pub unsafe extern "C" fn restore_pv(mut pv_buffer: *mut libc::c_int,
                                    mut depth_buffer: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < depth_buffer {
        pv[0 as libc::c_int as usize][i as usize] =
            *pv_buffer.offset(i as isize);
        i += 1
    }
    pv_depth[0 as libc::c_int as usize] = depth_buffer;
}
/*
  CLEAR_PV
  Clears the principal variation.
*/

pub unsafe extern "C" fn clear_pv() {
    pv_depth[0 as libc::c_int as usize] = 0 as libc::c_int;
}
/*
  COMPLETE_PV
  Complete the principal variation with passes (if any there are any).
*/

pub unsafe extern "C" fn complete_pv(mut side_to_move: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut actual_side_to_move: [libc::c_int; 60] = [0; 60];
    full_pv_depth = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < pv_depth[0 as libc::c_int as usize] {
        if make_move(side_to_move, pv[0 as libc::c_int as usize][i as usize],
                     1 as libc::c_int) != 0 {
            actual_side_to_move[i as usize] = side_to_move;
            full_pv[full_pv_depth as usize] =
                pv[0 as libc::c_int as usize][i as usize];
            full_pv_depth += 1
        } else {
            full_pv[full_pv_depth as usize] = -(1 as libc::c_int);
            full_pv_depth += 1;
            side_to_move = 0 as libc::c_int + 2 as libc::c_int - side_to_move;
            if make_move(side_to_move,
                         pv[0 as libc::c_int as usize][i as usize],
                         1 as libc::c_int) != 0 {
                actual_side_to_move[i as usize] = side_to_move;
                full_pv[full_pv_depth as usize] =
                    pv[0 as libc::c_int as usize][i as usize];
                full_pv_depth += 1
            } else {
                let mut j: libc::c_int = 0;
                printf(b"pv_depth[0] = %d\n\x00" as *const u8 as
                           *const libc::c_char,
                       pv_depth[0 as libc::c_int as usize]);
                j = 0 as libc::c_int;
                while j < pv_depth[0 as libc::c_int as usize] {
                    printf(b"%c%c \x00" as *const u8 as *const libc::c_char,
                           'a' as i32 +
                               pv[0 as libc::c_int as usize][j as usize] %
                                   10 as libc::c_int - 1 as libc::c_int,
                           '0' as i32 +
                               pv[0 as libc::c_int as usize][j as usize] /
                                   10 as libc::c_int);
                    j += 1
                }
                puts(b"\x00" as *const u8 as *const libc::c_char);
                printf(b"i=%d\n\x00" as *const u8 as *const libc::c_char, i);
                fatal_error(b"Error in PV completion\x00" as *const u8 as
                                *const libc::c_char);
            }
        }
        side_to_move = 0 as libc::c_int + 2 as libc::c_int - side_to_move;
        i += 1
    }
    i = pv_depth[0 as libc::c_int as usize] - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        unmake_move(actual_side_to_move[i as usize],
                    pv[0 as libc::c_int as usize][i as usize]);
        i -= 1
    };
}
/*
  HASH_EXPAND_PV
  Pad the existing PV with the move sequence suggested by the hash table.
*/

pub unsafe extern "C" fn hash_expand_pv(mut side_to_move: libc::c_int,
                                        mut mode: libc::c_int,
                                        mut flags: libc::c_int,
                                        mut max_selectivity: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut pass_count: libc::c_int = 0;
    let mut new_pv_depth: libc::c_int = 0;
    let mut new_pv: [libc::c_int; 61] = [0; 61];
    let mut new_side_to_move: [libc::c_int; 61] = [0; 61];
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    determine_hash_values(side_to_move, board.as_mut_ptr());
    new_pv_depth = 0 as libc::c_int;
    pass_count = 0 as libc::c_int;
    while pass_count < 2 as libc::c_int {
        new_side_to_move[new_pv_depth as usize] = side_to_move;
        if new_pv_depth < pv_depth[0 as libc::c_int as usize] &&
               new_pv_depth == 0 as libc::c_int {
            if board[pv[0 as libc::c_int as usize][new_pv_depth as usize] as
                         usize] == 1 as libc::c_int &&
                   make_move(side_to_move,
                             pv[0 as libc::c_int as
                                    usize][new_pv_depth as usize],
                             1 as libc::c_int) != 0 {
                new_pv[new_pv_depth as usize] =
                    pv[0 as libc::c_int as usize][new_pv_depth as usize];
                new_pv_depth += 1;
                pass_count = 0 as libc::c_int
            } else {
                hash1 ^= hash_flip_color1;
                hash2 ^= hash_flip_color2;
                pass_count += 1
            }
        } else {
            find_hash(&mut entry, mode);
            if entry.draft as libc::c_int != 0 as libc::c_int &&
                   entry.flags as libc::c_int & flags != 0 &&
                   entry.selectivity as libc::c_int <= max_selectivity &&
                   board[entry.move_0[0 as libc::c_int as usize] as usize] ==
                       1 as libc::c_int &&
                   make_move(side_to_move,
                             entry.move_0[0 as libc::c_int as usize],
                             1 as libc::c_int) != 0 {
                new_pv[new_pv_depth as usize] =
                    entry.move_0[0 as libc::c_int as usize];
                new_pv_depth += 1;
                pass_count = 0 as libc::c_int
            } else {
                hash1 ^= hash_flip_color1;
                hash2 ^= hash_flip_color2;
                pass_count += 1
            }
        }
        side_to_move = 0 as libc::c_int + 2 as libc::c_int - side_to_move
    }
    i = new_pv_depth - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        unmake_move(new_side_to_move[i as usize], new_pv[i as usize]);
        i -= 1
    }
    i = 0 as libc::c_int;
    while i < new_pv_depth {
        pv[0 as libc::c_int as usize][i as usize] = new_pv[i as usize];
        i += 1
    }
    pv_depth[0 as libc::c_int as usize] = new_pv_depth;
}
/*
  SET_PONDER_MOVE
  CLEAR_PONDER_MOVE
  GET_PONDER_MOVE
  A value of 0 denotes a normal search while anything else means
  that the search is performed given that the move indicated has
  been made.
*/

pub unsafe extern "C" fn set_ponder_move(mut move_0: libc::c_int) {
    pondered_move = move_0;
}

pub unsafe extern "C" fn clear_ponder_move() {
    pondered_move = 0 as libc::c_int;
}

pub unsafe extern "C" fn get_ponder_move() -> libc::c_int {
    return pondered_move;
}
/*
  CREATE_EVAL_INFO
  Creates a result descriptor given all the information available
  about the last search.
*/

pub unsafe extern "C" fn create_eval_info(mut in_type: EvalType,
                                          mut in_res: EvalResult,
                                          mut in_score: libc::c_int,
                                          mut in_conf: libc::c_double,
                                          mut in_depth: libc::c_int,
                                          mut in_book: libc::c_int)
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

pub unsafe extern "C" fn produce_compact_eval(mut eval_info: EvaluationType)
 -> libc::c_double {
    let mut eval: libc::c_double = 0.;
    's_97:
        {
            let mut current_block_17: u64;
            match eval_info.type_0 as libc::c_uint {
                0 => {
                    /*
        eval = eval_info.search_depth + logistic_map( eval_info.score );
        if ( eval_info.is_book )
          eval = -eval;
          */
                    eval = eval_info.score as libc::c_double / 128.0f64;
                    return eval
                }
                1 => { return eval_info.score as libc::c_double / 128.0f64 }
                2 => {
                    match eval_info.res as libc::c_uint {
                        0 => {
                            if eval_info.score >
                                   2 as libc::c_int * 128 as libc::c_int {
                                /* Win by more than 2 */
                                return eval_info.score as libc::c_double /
                                           128.0f64 - 0.01f64
                            } else { return 1.99f64 }
                        }
                        1 => { return 0.0f64 }
                        2 => {
                            if eval_info.score <
                                   -(2 as libc::c_int) * 128 as libc::c_int {
                                /* Loss by more than 2 */
                                return eval_info.score as libc::c_double /
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
                    match eval_info.res as libc::c_uint {
                        0 => { return 1.0f64 + eval_info.confidence }
                        1 => { return -1.0f64 + eval_info.confidence }
                        2 => { return -1.0f64 - eval_info.confidence }
                        3 => {
                            return eval_info.score as libc::c_double /
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

pub unsafe extern "C" fn set_current_eval(mut eval: EvaluationType) {
    last_eval = eval;
    if negate_eval != 0 {
        last_eval.score = -last_eval.score;
        if last_eval.res as libc::c_uint ==
               WON_POSITION as libc::c_int as libc::c_uint {
            last_eval.res = LOST_POSITION
        } else if last_eval.res as libc::c_uint ==
                      LOST_POSITION as libc::c_int as libc::c_uint {
            last_eval.res = WON_POSITION
        }
    };
}

pub unsafe extern "C" fn get_current_eval() -> EvaluationType {
    return last_eval;
}

pub unsafe extern "C" fn negate_current_eval(mut negate: libc::c_int) {
    negate_eval = negate;
}
