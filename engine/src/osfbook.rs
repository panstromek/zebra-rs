use crate::{
    src::{
        search::{get_ponder_move, create_eval_info, root_eval, disc_count, nodes},
        moves::{unmake_move, make_move, generate_specific, disks_played, move_list, move_count, generate_all, unmake_move_no_hash, make_move_no_hash},
        opname::opening_list,
        hash::{determine_hash_values, clear_hash_drafts},
        game::{CandidateMove},
        myrandom::{my_random, my_srandom},
        globals::{board, piece_count, pv, pv_depth},
        midgame::{toggle_midgame_abort_check, toggle_midgame_hash_usage},
        eval::toggle_experimental,
        getcoeff::remove_coeffs,
        counter::reset_counter,
        patterns::{col_pattern, flip8, row_pattern, compute_line_patterns},
        zebra::{EvaluationType}
    }
};
use crate::src::stubs::{abs, ceil, floor, free, strlen, tolower};
use crate::src::error::fatal_error;
use crate::src::safemem::{safe_malloc, safe_realloc};
use std::ffi::c_void;
use crate::src::midgame::tree_search;
use crate::src::timer::{last_panic_check, clear_panic_abort};

extern "C" {
    #[no_mangle]
    fn report_do_evaluate(evaluation_stage_: i32);
}

pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = u64;

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


pub type DrawMode = u32;
pub const OPPONENT_WINS: DrawMode = 3;
pub const WHITE_WINS: DrawMode = 2;
pub const BLACK_WINS: DrawMode = 1;
pub const NEUTRAL: DrawMode = 0;
pub type GameMode = u32;
pub const PUBLIC_GAME: GameMode = 1;
pub const PRIVATE_GAME: GameMode = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BookNode {
    pub hash_val1: i32,
    pub hash_val2: i32,
    pub black_minimax_score: i16,
    pub white_minimax_score: i16,
    pub best_alternative_move: i16,
    pub alternative_score: i16,
    pub flags: u16,
}
pub static mut deviation_bonus: f64 = 0.;
pub static mut search_depth: i32 = 0;
pub static mut node_table_size: i32 = 0;
pub static mut hash_table_size: i32 = 0;
pub static mut total_game_count: i32 = 0;
pub static mut book_node_count: i32 = 0;
pub static mut evaluated_count: i32 = 0;
pub static mut evaluation_stage: i32 = 0;
pub static mut max_eval_count: i32 = 0;
pub static mut max_batch_size: i32 = 0;
pub static mut exhausted_node_count: i32 = 0;
pub static mut max_slack: i32 = 0;
pub static mut low_deviation_threshold: i32 = 0;
pub static mut high_deviation_threshold: i32 = 0;
pub static mut min_eval_span: i32 = 0;
pub static mut max_eval_span: i32 = 0;
pub static mut min_negamax_span: i32 = 0;
pub static mut max_negamax_span: i32 = 0;
pub static mut leaf_count: i32 = 0;
pub static mut bad_leaf_count: i32 = 0;
pub static mut really_bad_leaf_count: i32 = 0;
pub static mut unreachable_count: i32 = 0;
pub static mut candidate_count: i32 = 0;
pub static mut force_black: i32 = 0;
pub static mut force_white: i32 = 0;
pub static mut used_slack: [i32; 3] = [0; 3];
pub static mut b1_b1_map: [i32; 100] = [0; 100];
pub static mut g1_b1_map: [i32; 100] = [0; 100];
pub static mut g8_b1_map: [i32; 100] = [0; 100];
pub static mut b8_b1_map: [i32; 100] = [0; 100];
pub static mut a2_b1_map: [i32; 100] = [0; 100];
pub static mut a7_b1_map: [i32; 100] = [0; 100];
pub static mut h7_b1_map: [i32; 100] = [0; 100];
pub static mut h2_b1_map: [i32; 100] = [0; 100];
pub static mut exact_count: [i32; 61] = [0; 61];
pub static mut wld_count: [i32; 61] = [0; 61];
pub static mut exhausted_count: [i32; 61] = [0; 61];
pub static mut common_count: [i32; 61] = [0; 61];
pub static mut symmetry_map: [*mut i32; 8] =
    [0 as *const i32 as *mut i32; 8];
pub static mut inv_symmetry_map: [*mut i32; 8] =
    [0 as *const i32 as *mut i32; 8];
pub static mut line_hash: [[[i32; 6561]; 8]; 2] = [[[0; 6561]; 8]; 2];
pub static mut book_hash_table: *mut i32 =
    0 as *const i32 as *mut i32;
pub static mut draw_mode: DrawMode = OPPONENT_WINS;
pub static mut game_mode: GameMode = PRIVATE_GAME;
pub static mut node: *mut BookNode = 0 as *const BookNode as *mut BookNode;
pub static mut candidate_list: [CandidateMove; 60] =
    [CandidateMove{move_0: 0, score: 0, flags: 0, parent_flags: 0,}; 60];


/*
   SELECT_HASH_SLOT
   Finds a slot in the hash table for the node INDEX
   using linear probing.
*/
pub unsafe fn select_hash_slot(mut index: i32) {
    let mut slot: i32 = 0;
    slot = (*node.offset(index as isize)).hash_val1 % hash_table_size;
    while *book_hash_table.offset(slot as isize) != -(1 as i32) {
        slot = (slot + 1 as i32) % hash_table_size
    }
    *book_hash_table.offset(slot as isize) = index;
}
/*
   PROBE_HASH_TABLE
   Search for a certain hash code in the hash table.
*/
pub unsafe fn probe_hash_table(mut val1: i32,
                           mut val2: i32) -> i32 {
    let mut slot: i32 = 0;
    if hash_table_size == 0 as i32 {
        return -(1 as i32)
    } else {
        slot = val1 % hash_table_size;
        while *book_hash_table.offset(slot as isize) != -(1 as i32) &&
            ((*node.offset(*book_hash_table.offset(slot as isize) as
                isize)).hash_val2 != val2 ||
                (*node.offset(*book_hash_table.offset(slot as isize) as
                    isize)).hash_val1 != val1) {
            slot = (slot + 1 as i32) % hash_table_size
        }
        return slot
    };
}
/*
   CREATE_HASH_REFERENCEE
   Takes the node list and fills the hash table with indices
   into the node list.
*/
pub unsafe fn create_hash_reference() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < hash_table_size {
        *book_hash_table.offset(i as isize) = -(1 as i32);
        i += 1
    }
    i = 0 as i32;
    while i < book_node_count { select_hash_slot(i); i += 1 };
}
/*
   PREPARE_HASH
   Compute the position hash codes.
*/
pub unsafe fn prepare_hash() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    /* The hash keys are static, hence the same keys must be
       produced every time the program is run. */
    my_srandom(0 as i32);
    i = 0 as i32;
    while i < 2 as i32 {
        j = 0 as i32;
        while j < 8 as i32 {
            k = 0 as i32;
            while k < 6561 as i32 {
                line_hash[i as usize][j as usize][k as usize] =
                    if my_random() % 2 as i32 as i64 != 0 {
                        my_random()
                    } else { -my_random() } as i32;
                k += 1
            }
            j += 1
        }
        i += 1
    }
    hash_table_size = 0 as i32;
}

/*
   INIT_BOOK_TREE
   Initializes the node tree by creating the root of the tree.
*/
pub unsafe fn init_book_tree() {
    book_node_count = 0 as i32;
    node = 0 as *mut BookNode;
}

/*
   CLEAR_NODE_DEPTH
   Changes the flags of a node so that the search depth
   bits are cleared.
*/
pub unsafe fn clear_node_depth(mut index: i32) {
    let mut depth: i32 = 0;
    depth =
        (*node.offset(index as isize)).flags as i32 >>
            10 as i32;
    let ref mut fresh0 = (*node.offset(index as isize)).flags;
    *fresh0 =
        (*fresh0 as i32 ^ depth << 10 as i32) as
            u16;
}
/*
   GET_NODE_DEPTH
*/
pub unsafe fn get_node_depth(mut index: i32) -> i32 {
    return (*node.offset(index as isize)).flags as i32 >>
        10 as i32;
}
/*
   SET_NODE_DEPTH
   Marks a node as being searched to a certain depth.
*/
pub unsafe fn set_node_depth(mut index: i32,
                         mut depth: i32) {
    let ref mut fresh1 = (*node.offset(index as isize)).flags;
    *fresh1 =
        (*fresh1 as i32 | depth << 10 as i32) as
            u16;
}

/*
  SET_MAX_BATCH_SIZE
  Specify the maximum number of nodes to evaluate.
*/

pub unsafe fn set_max_batch_size(mut size: i32) {
    max_batch_size = size;
}
/*
   SET_DEVIATION_VALUE
   Sets the number of disks where a penalty is incurred if
   the deviation from the book line comes later than that
   stage; also set the punishment per move after the threshold.
*/

pub unsafe fn set_deviation_value(mut low_threshold: i32,
                                  mut high_threshold: i32,
                                  mut bonus: f64) {
    low_deviation_threshold = low_threshold;
    high_deviation_threshold = high_threshold;
    deviation_bonus = bonus;
}

/*
   RESET_BOOK_SEARCH
   Sets the used slack count to zero.
*/

pub unsafe fn reset_book_search() {
    used_slack[0 as i32 as usize] = 0.0f64 as i32;
    used_slack[2 as i32 as usize] = 0.0f64 as i32;
}
/*
   SET_SLACK
   Sets the total amount of negamaxed evaluation that
   the program is willing to trade for randomness.
*/

pub unsafe fn set_slack(mut slack: i32) {
    max_slack = slack;
}
/*
  SET_DRAW_MODE
  Specifies how book draws should be treated.
*/

pub unsafe fn set_draw_mode(mut mode: DrawMode) {
    draw_mode = mode;
}
/*
  SET_GAME_MODE
  Specifies if the book is in private or public mode.
*/

pub unsafe fn set_game_mode(mut mode: GameMode) {
    game_mode = mode;
}
/*
  SET_BLACK_FORCE
  SET_WHITE_FORCE
  Specifies if the moves for either of the players are to
  be forced when recursing the tree.
*/

pub unsafe fn set_black_force(mut force: i32) {
    force_black = force;
}

pub unsafe fn set_white_force(mut force: i32) {
    force_white = force;
}

/*
  GET_CANDIDATE_COUNT
  GET_CANDIDATE
  Accessor functions for the data structure created by
  FILL_MOVE_ALTERNATIVES.
*/

pub unsafe fn get_candidate_count() -> i32 {
    return candidate_count;
}

pub unsafe fn get_candidate(mut index: i32)
                            -> CandidateMove {
    return candidate_list[index as usize];
}


/*
   GET_HASH
   Return the hash values for the current board position.
   The position is rotated so that the 64-bit hash value
   is minimized among all rotations. This ensures detection
   of all transpositions.
   See also init_maps().
*/

pub unsafe fn get_hash(mut val0: *mut i32,
                       mut val1: *mut i32,
                       mut orientation: *mut i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut min_map: i32 = 0;
    let mut min_hash0: i32 = 0;
    let mut min_hash1: i32 = 0;
    let mut out: [[i32; 2]; 8] = [[0; 2]; 8];
    /* Calculate the 8 different 64-bit hash values for the
       different rotations. */
    compute_line_patterns(board.as_mut_ptr());
    i = 0 as i32;
    while i < 8 as i32 {
        j = 0 as i32;
        while j < 2 as i32 {
            out[i as usize][j as usize] = 0 as i32;
            j += 1
        }
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        /* b1 -> b1 */
        out[0 as i32 as usize][0 as i32 as usize] ^=
            line_hash[0 as i32 as
                usize][i as
                usize][row_pattern[i as usize] as usize];
        out[0 as i32 as usize][1 as i32 as usize] ^=
            line_hash[1 as i32 as
                usize][i as
                usize][row_pattern[i as usize] as usize];
        /* g1 -> b1 */
        out[1 as i32 as usize][0 as i32 as usize] ^=
            line_hash[0 as i32 as
                usize][i as
                usize][flip8[row_pattern[i as usize] as
                usize] as usize];
        out[1 as i32 as usize][1 as i32 as usize] ^=
            line_hash[1 as i32 as
                usize][i as
                usize][flip8[row_pattern[i as usize] as
                usize] as usize];
        /* g8 -> b1 */
        out[2 as i32 as usize][0 as i32 as usize] ^=
            line_hash[0 as i32 as
                usize][i as
                usize][flip8[row_pattern[(7 as
                i32
                - i) as
                usize] as
                usize] as usize];
        out[2 as i32 as usize][1 as i32 as usize] ^=
            line_hash[1 as i32 as
                usize][i as
                usize][flip8[row_pattern[(7 as
                i32
                - i) as
                usize] as
                usize] as usize];
        /* b8 -> b1 */
        out[3 as i32 as usize][0 as i32 as usize] ^=
            line_hash[0 as i32 as
                usize][i as
                usize][row_pattern[(7 as i32 - i)
                as usize] as
                usize];
        out[3 as i32 as usize][1 as i32 as usize] ^=
            line_hash[1 as i32 as
                usize][i as
                usize][row_pattern[(7 as i32 - i)
                as usize] as
                usize];
        /* a2 -> b1 */
        out[4 as i32 as usize][0 as i32 as usize] ^=
            line_hash[0 as i32 as
                usize][i as
                usize][col_pattern[i as usize] as usize];
        out[4 as i32 as usize][1 as i32 as usize] ^=
            line_hash[1 as i32 as
                usize][i as
                usize][col_pattern[i as usize] as usize];
        /* a7 -> b1 */
        out[5 as i32 as usize][0 as i32 as usize] ^=
            line_hash[0 as i32 as
                usize][i as
                usize][flip8[col_pattern[i as usize] as
                usize] as usize];
        out[5 as i32 as usize][1 as i32 as usize] ^=
            line_hash[1 as i32 as
                usize][i as
                usize][flip8[col_pattern[i as usize] as
                usize] as usize];
        /* h7 -> b1 */
        out[6 as i32 as usize][0 as i32 as usize] ^=
            line_hash[0 as i32 as
                usize][i as
                usize][flip8[col_pattern[(7 as
                i32
                - i) as
                usize] as
                usize] as usize];
        out[6 as i32 as usize][1 as i32 as usize] ^=
            line_hash[1 as i32 as
                usize][i as
                usize][flip8[col_pattern[(7 as
                i32
                - i) as
                usize] as
                usize] as usize];
        /* h2 -> b1 */
        out[7 as i32 as usize][0 as i32 as usize] ^=
            line_hash[0 as i32 as
                usize][i as
                usize][col_pattern[(7 as i32 - i)
                as usize] as
                usize];
        out[7 as i32 as usize][1 as i32 as usize] ^=
            line_hash[1 as i32 as
                usize][i as
                usize][col_pattern[(7 as i32 - i)
                as usize] as
                usize];
        i += 1
    }
    /* Find the rotation minimizing the hash index.
       If two hash indices are equal, map number is implicitly used
       as tie-breaker. */
    min_map = 0 as i32;
    min_hash0 = out[0 as i32 as usize][0 as i32 as usize];
    min_hash1 = out[0 as i32 as usize][1 as i32 as usize];
    i = 1 as i32;
    while i < 8 as i32 {
        if out[i as usize][0 as i32 as usize] < min_hash0 ||
            out[i as usize][0 as i32 as usize] == min_hash0 &&
                out[i as usize][1 as i32 as usize] < min_hash1 {
            min_map = i;
            min_hash0 = out[i as usize][0 as i32 as usize];
            min_hash1 = out[i as usize][1 as i32 as usize]
        }
        i += 1
    }
    *val0 = abs(min_hash0);
    *val1 = abs(min_hash1);
    *orientation = min_map;
}

/*
   DO_COMPRESS
   Compresses the subtree below the current node.
*/
pub unsafe fn do_compress(mut index: i32,
                      mut node_order: *mut i32,
                      mut child_count: *mut i16,
                      mut node_index: *mut i32,
                      mut child_list: *mut i16,
                      mut child_index: *mut i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut child: i32 = 0;
    let mut valid_child_count: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut found: i32 = 0;
    let mut local_child_list: [i32; 64] = [0; 64];
    let mut this_move: i16 = 0;
    let mut local_child_move: [i16; 64] = [0; 64];
    if (*node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    *node_order.offset(*node_index as isize) = index;
    if (*node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    valid_child_count = 0 as i32;
    generate_all(side_to_move);
    i = 0 as i32;
    while i < move_count[disks_played as usize] {
        this_move =
            move_list[disks_played as usize][i as usize] as i16;
        make_move(side_to_move, this_move as i32, 1 as i32);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as i32) &&
            (*node.offset(child as isize)).flags as i32 &
                8 as i32 != 0 {
            j = 0 as i32;
            found = 0 as i32;
            while j < valid_child_count {
                if child == local_child_list[j as usize] {
                    found = 1 as i32
                }
                j += 1
            }
            if found == 0 {
                local_child_list[valid_child_count as usize] = child;
                local_child_move[valid_child_count as usize] = this_move;
                valid_child_count += 1;
                *child_list.offset(*child_index as isize) = this_move;
                *child_index += 1
            }
        }
        unmake_move(side_to_move, this_move as i32);
        i += 1
    }
    *child_count.offset(*node_index as isize) =
        valid_child_count as i16;
    *node_index += 1;
    i = 0 as i32;
    while i < valid_child_count {
        this_move = local_child_move[i as usize];
        make_move(side_to_move, this_move as i32, 1 as i32);
        do_compress(local_child_list[i as usize], node_order, child_count,
                    node_index, child_list, child_index);
        unmake_move(side_to_move, this_move as i32);
        i += 1
    }
    let ref mut fresh44 = (*node.offset(index as isize)).flags;
    *fresh44 = (*fresh44 as i32 ^ 8 as i32) as u16;
}

/*
   SET_SEARCH_DEPTH
   When finding move alternatives, searches to depth DEPTH
   will be performed.
*/

pub unsafe fn set_search_depth(mut depth: i32) {
    search_depth = depth;
}
/*
  SET_EVAL_SPAN
  Specify the evaluation value interval where nodes are re-evaluated.
*/

pub unsafe fn set_eval_span(mut min_span: f64,
                            mut max_span: f64) {
    min_eval_span = ceil(min_span * 128.0f64) as i32;
    max_eval_span = ceil(max_span * 128.0f64) as i32;
}
/*
  SET_NEGAMAX_SPAN
  Specify the negamax value interval where nodes are re-evaluated.
*/

pub unsafe fn set_negamax_span(mut min_span: f64,
                               mut max_span: f64) {
    min_negamax_span = ceil(min_span * 128.0f64) as i32;
    max_negamax_span = ceil(max_span * 128.0f64) as i32;
}
/*
   ADJUST_SCORE
   Tweak a score as to encourage early deviations.
*/
pub unsafe fn adjust_score(mut score: i32,
                       mut side_to_move: i32)
                       -> i32 {
    let mut adjustment: i32 = 0;
    let mut adjust_steps: i32 = 0;
    adjust_steps = high_deviation_threshold - disks_played;
    if adjust_steps < 0 as i32 {
        adjustment = 0 as i32
    } else {
        if disks_played < low_deviation_threshold {
            adjust_steps = high_deviation_threshold - low_deviation_threshold
        }
        adjustment =
            floor(adjust_steps as f64 * deviation_bonus * 128.0f64)
                as i32;
        if side_to_move == 2 as i32 { adjustment = -adjustment }
    }
    return score + adjustment;
}
/*
   DO_MINIMAX
   Calculates the minimax value of node INDEX.
*/
pub unsafe fn do_minimax(mut index: i32,
                         mut black_score: *mut i32,
                         mut white_score: *mut i32) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut child_black_score: i32 = 0;
    let mut child_white_score: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut alternative_move: i32 = 0;
    let mut alternative_move_found: i32 = 0;
    let mut child_count: i32 = 0;
    let mut best_black_child_val: i32 = 0;
    let mut best_white_child_val: i32 = 0;
    let mut worst_black_child_val: i32 = 0;
    let mut worst_white_child_val: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut best_black_score: i16 = 0;
    let mut best_white_score: i16 = 0;
    /* If the node has been visited AND it is a midgame node, meaning
       that the minimax values are not to be tweaked, return the
       stored values. */
    if (*node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        if (*node.offset(index as isize)).flags as i32 &
            (4 as i32 | 16 as i32) == 0 {
            *black_score =
                (*node.offset(index as isize)).black_minimax_score as
                    i32;
            *white_score =
                (*node.offset(index as isize)).white_minimax_score as
                    i32;
            return
        }
    }
    /* Correct WLD solved nodes corresponding to draws to be represented
       as full solved and make sure full solved nodes are marked as
       WLD solved as well */
    if (*node.offset(index as isize)).flags as i32 & 4 as i32
        != 0 &&
        (*node.offset(index as isize)).black_minimax_score as i32
            == 0 as i32 &&
        (*node.offset(index as isize)).white_minimax_score as i32
            == 0 as i32 {
        let ref mut fresh2 = (*node.offset(index as isize)).flags;
        *fresh2 =
            (*fresh2 as i32 | 16 as i32) as u16
    }
    if (*node.offset(index as isize)).flags as i32 & 16 as i32
        != 0 &&
        (*node.offset(index as isize)).flags as i32 &
            4 as i32 == 0 {
        let ref mut fresh3 = (*node.offset(index as isize)).flags;
        *fresh3 =
            (*fresh3 as i32 | 4 as i32) as u16
    }
    /* Recursively minimax all children of the node */
    if (*node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    best_black_child_val = -(99999 as i32);
    best_white_child_val = -(99999 as i32);
    worst_black_child_val = 99999 as i32;
    worst_white_child_val = 99999 as i32;
    if (*node.offset(index as isize)).alternative_score as i32 !=
        9999 as i32 {
        best_black_score =
            adjust_score((*node.offset(index as isize)).alternative_score as
                             i32, side_to_move) as i16;
        best_white_score = best_black_score;
        worst_black_child_val = best_black_score as i32;
        best_black_child_val = worst_black_child_val;
        worst_white_child_val = best_white_score as i32;
        best_white_child_val = worst_white_child_val;
        alternative_move_found = 0 as i32;
        alternative_move =
            (*node.offset(index as isize)).best_alternative_move as
                i32;
        if alternative_move > 0 as i32 {
            get_hash(&mut val1, &mut val2, &mut orientation);
            alternative_move =
                *inv_symmetry_map[orientation as
                    usize].offset(alternative_move as isize)
        }
    } else {
        alternative_move_found = 1 as i32;
        alternative_move = 0 as i32;
        if side_to_move == 0 as i32 {
            best_black_score = -(32000 as i32) as i16;
            best_white_score = -(32000 as i32) as i16
        } else {
            best_black_score = 32000 as i32 as i16;
            best_white_score = 32000 as i32 as i16
        }
    }
    generate_all(side_to_move);
    child_count = 0 as i32;
    i = 0 as i32;
    while i < move_count[disks_played as usize] {
        piece_count[0 as i32 as usize][disks_played as usize] =
            disc_count(0 as i32);
        piece_count[2 as i32 as usize][disks_played as usize] =
            disc_count(2 as i32);
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_minimax(child, &mut child_black_score, &mut child_white_score);
            best_black_child_val =
                if best_black_child_val > child_black_score {
                    best_black_child_val
                } else { child_black_score };
            best_white_child_val =
                if best_white_child_val > child_white_score {
                    best_white_child_val
                } else { child_white_score };
            worst_black_child_val =
                if worst_black_child_val < child_black_score {
                    worst_black_child_val
                } else { child_black_score };
            worst_white_child_val =
                if worst_white_child_val < child_white_score {
                    worst_white_child_val
                } else { child_white_score };
            if side_to_move == 0 as i32 {
                best_black_score =
                    if child_black_score > best_black_score as i32 {
                        child_black_score
                    } else { best_black_score as i32 } as
                        i16;
                best_white_score =
                    if child_white_score > best_white_score as i32 {
                        child_white_score
                    } else { best_white_score as i32 } as
                        i16
            } else {
                best_black_score =
                    if child_black_score < best_black_score as i32 {
                        child_black_score
                    } else { best_black_score as i32 } as
                        i16;
                best_white_score =
                    if child_white_score < best_white_score as i32 {
                        child_white_score
                    } else { best_white_score as i32 } as
                        i16
            }
            child_count += 1
        } else if alternative_move_found == 0 && this_move == alternative_move
        {
            alternative_move_found = 1 as i32
        }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    if alternative_move_found == 0 {
        /* The was-to-be deviation now leads to a position in the database,
           hence it can no longer be used. */
        (*node.offset(index as isize)).alternative_score =
            9999 as i32 as i16;
        (*node.offset(index as isize)).best_alternative_move =
            -(1 as i32) as i16
    }
    /* Try to infer the WLD status from the children */
    if (*node.offset(index as isize)).flags as i32 &
        (16 as i32 | 4 as i32) == 0 &&
        child_count > 0 as i32 {
        if side_to_move == 0 as i32 {
            if best_black_child_val >= 30000 as i32 &&
                best_white_child_val >= 30000 as i32 {
                /* Black win */
                let ref mut fresh4 =
                    (*node.offset(index as isize)).white_minimax_score;
                *fresh4 =
                    if best_black_child_val < best_white_child_val {
                        best_black_child_val
                    } else { best_white_child_val } as i16;
                (*node.offset(index as isize)).black_minimax_score = *fresh4;
                let ref mut fresh5 = (*node.offset(index as isize)).flags;
                *fresh5 =
                    (*fresh5 as i32 | 4 as i32) as
                        u16
            } else if best_black_child_val <= -(30000 as i32) &&
                best_white_child_val <= -(30000 as i32) {
                /* Black loss */
                let ref mut fresh6 =
                    (*node.offset(index as isize)).white_minimax_score;
                *fresh6 =
                    if best_black_child_val > best_white_child_val {
                        best_black_child_val
                    } else { best_white_child_val } as i16;
                (*node.offset(index as isize)).black_minimax_score = *fresh6;
                let ref mut fresh7 = (*node.offset(index as isize)).flags;
                *fresh7 =
                    (*fresh7 as i32 | 4 as i32) as
                        u16
            }
        } else if worst_black_child_val <= -(30000 as i32) &&
            worst_white_child_val <= -(30000 as i32) {
            /* White win */
            let ref mut fresh8 =
                (*node.offset(index as isize)).white_minimax_score;
            *fresh8 =
                if worst_black_child_val > worst_white_child_val {
                    worst_black_child_val
                } else { worst_white_child_val } as i16;
            (*node.offset(index as isize)).black_minimax_score = *fresh8;
            let ref mut fresh9 = (*node.offset(index as isize)).flags;
            *fresh9 =
                (*fresh9 as i32 | 4 as i32) as u16
        } else if worst_black_child_val >= 30000 as i32 &&
            worst_white_child_val >= 30000 as i32 {
            /* White loss */
            let ref mut fresh10 =
                (*node.offset(index as isize)).white_minimax_score;
            *fresh10 =
                if worst_black_child_val < worst_white_child_val {
                    worst_black_child_val
                } else { worst_white_child_val } as i16;
            (*node.offset(index as isize)).black_minimax_score = *fresh10;
            let ref mut fresh11 = (*node.offset(index as isize)).flags;
            *fresh11 =
                (*fresh11 as i32 | 4 as i32) as u16
        }
    }
    /* Tweak the minimax scores for draws to give the right
       draw avoidance behavior */
    if (*node.offset(index as isize)).flags as i32 &
        (16 as i32 | 4 as i32) != 0 {
        *black_score =
            (*node.offset(index as isize)).black_minimax_score as i32;
        *white_score =
            (*node.offset(index as isize)).white_minimax_score as i32;
        if (*node.offset(index as isize)).black_minimax_score as i32
            == 0 as i32 &&
            (*node.offset(index as isize)).white_minimax_score as
                i32 == 0 as i32 {
            /* Is it a position in which a draw should be avoided? */
            if game_mode as u32 ==
                PRIVATE_GAME as i32 as u32 ||
                (*node.offset(index as isize)).flags as i32 &
                    32 as i32 == 0 {
                match draw_mode as u32 {
                    1 => {
                        *black_score =
                            30000 as i32 - 1 as i32;
                        *white_score = 30000 as i32 - 1 as i32
                    }
                    2 => {
                        *black_score =
                            -(30000 as i32 - 1 as i32);
                        *white_score =
                            -(30000 as i32 - 1 as i32)
                    }
                    3 => {
                        *black_score =
                            -(30000 as i32 - 1 as i32);
                        *white_score = 30000 as i32 - 1 as i32
                    }
                    0 | _ => { }
                }
            }
        }
    } else {
        let ref mut fresh12 =
            (*node.offset(index as isize)).black_minimax_score;
        *fresh12 = best_black_score;
        *black_score = *fresh12 as i32;
        let ref mut fresh13 =
            (*node.offset(index as isize)).white_minimax_score;
        *fresh13 = best_white_score;
        *white_score = *fresh13 as i32
    }
    let ref mut fresh14 = (*node.offset(index as isize)).flags;
    *fresh14 = (*fresh14 as i32 ^ 8 as i32) as u16;
}

/*
   INIT_MAPS
   Initializes the 8 symmetry maps.
   Notice that the order of these MUST coincide with the returned
   orientation value from get_hash() OR YOU WILL LOSE BIG.
*/
pub unsafe fn init_maps() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut pos: i32 = 0;
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            pos = 10 as i32 * i + j;
            b1_b1_map[pos as usize] = pos;
            g1_b1_map[pos as usize] =
                10 as i32 * i + (9 as i32 - j);
            g8_b1_map[pos as usize] =
                10 as i32 * (9 as i32 - i) +
                    (9 as i32 - j);
            b8_b1_map[pos as usize] =
                10 as i32 * (9 as i32 - i) + j;
            a2_b1_map[pos as usize] = 10 as i32 * j + i;
            a7_b1_map[pos as usize] =
                10 as i32 * j + (9 as i32 - i);
            h7_b1_map[pos as usize] =
                10 as i32 * (9 as i32 - j) +
                    (9 as i32 - i);
            h2_b1_map[pos as usize] =
                10 as i32 * (9 as i32 - j) + i;
            j += 1
        }
        i += 1
    }
    symmetry_map[0 as i32 as usize] = b1_b1_map.as_mut_ptr();
    inv_symmetry_map[0 as i32 as usize] = b1_b1_map.as_mut_ptr();
    symmetry_map[1 as i32 as usize] = g1_b1_map.as_mut_ptr();
    inv_symmetry_map[1 as i32 as usize] = g1_b1_map.as_mut_ptr();
    symmetry_map[2 as i32 as usize] = g8_b1_map.as_mut_ptr();
    inv_symmetry_map[2 as i32 as usize] = g8_b1_map.as_mut_ptr();
    symmetry_map[3 as i32 as usize] = b8_b1_map.as_mut_ptr();
    inv_symmetry_map[3 as i32 as usize] = b8_b1_map.as_mut_ptr();
    symmetry_map[4 as i32 as usize] = a2_b1_map.as_mut_ptr();
    inv_symmetry_map[4 as i32 as usize] = a2_b1_map.as_mut_ptr();
    symmetry_map[5 as i32 as usize] = a7_b1_map.as_mut_ptr();
    inv_symmetry_map[5 as i32 as usize] = h2_b1_map.as_mut_ptr();
    symmetry_map[6 as i32 as usize] = h7_b1_map.as_mut_ptr();
    inv_symmetry_map[6 as i32 as usize] = h7_b1_map.as_mut_ptr();
    symmetry_map[7 as i32 as usize] = h2_b1_map.as_mut_ptr();
    inv_symmetry_map[7 as i32 as usize] = a7_b1_map.as_mut_ptr();
    i = 0 as i32;
    while i < 8 as i32 {
        *symmetry_map[i as usize].offset(0 as i32 as isize) =
            0 as i32;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            k = 1 as i32;
            while k <= 8 as i32 {
                pos = 10 as i32 * j + k;
                if *inv_symmetry_map[i as
                    usize].offset(*symmetry_map[i as
                    usize].offset(pos
                    as
                    isize)
                    as isize) != pos {
                    fatal_error(b"Error in map %d: inv(map(%d))=%d\n\x00" as
                                    *const u8 as *const i8, i, pos,
                                *inv_symmetry_map[i as
                                    usize].offset(*symmetry_map[i
                                    as
                                    usize].offset(pos
                                    as
                                    isize)
                                    as
                                    isize));
                }
                k += 1
            }
            j += 1
        }
        i += 1
    };
}


/*
   REBUILD_HASH_TABLE
   Resize the hash table for a requested number of nodes.
*/
pub unsafe fn rebuild_hash_table(mut requested_items: i32) {
    let mut new_size: i32 = 0;
    let mut new_memory: i32 = 0;
    new_size = 2 as i32 * requested_items;
    new_memory =
        (new_size as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64) as i32;
    if hash_table_size == 0 as i32 {
        book_hash_table =
            safe_malloc(new_memory as size_t) as *mut i32
    } else {
        book_hash_table =
            safe_realloc(book_hash_table as *mut c_void,
                         new_memory as size_t) as *mut i32
    }
    if book_hash_table.is_null() {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Book hash table: Failed to allocate\x00" as *const u8 as
                        *const i8, new_memory, new_size);
    }
    hash_table_size = new_size;
    create_hash_reference();
}



/*
   SET_ALLOCATION
   Changes the number of nodes for which memory is allocated.
*/
pub unsafe fn set_allocation(mut size: i32) {
    if node.is_null() {
        node =
            safe_malloc((size as
                u64).wrapping_mul(::std::mem::size_of::<BookNode>()
                as
                u64))
                as *mut BookNode
    } else {
        node =
            safe_realloc(node as *mut c_void,
                         (size as
                             u64).wrapping_mul(::std::mem::size_of::<BookNode>()
                             as
                             u64))
                as *mut BookNode
    }
    if node.is_null() {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Book node list: Failed to allocate\x00" as *const u8 as
                        *const i8,
                    (size as
                        u64).wrapping_mul(::std::mem::size_of::<BookNode>()
                        as u64),
                    size);
    }
    node_table_size = size;
    if node_table_size as f64 >
        0.80f64 * hash_table_size as f64 {
        rebuild_hash_table(node_table_size);
    };
}
/*
   INCREASE_ALLOCATION
   Allocate more memory for the book tree.
*/
pub unsafe fn increase_allocation() {
    set_allocation(node_table_size + 50000 as i32);
}
/*
   CREATE_BOOK_NODE
   Creates a new book node without any connections whatsoever
   to the rest of the tree.
*/
pub unsafe fn create_BookNode(mut val1: i32,
                          mut val2: i32,
                          mut flags: u16)
                          -> i32 {
    let mut index: i32 = 0;
    if book_node_count == node_table_size { increase_allocation(); }
    index = book_node_count;
    (*node.offset(index as isize)).hash_val1 = val1;
    (*node.offset(index as isize)).hash_val2 = val2;
    (*node.offset(index as isize)).black_minimax_score =
        9999 as i32 as i16;
    (*node.offset(index as isize)).white_minimax_score =
        9999 as i32 as i16;
    (*node.offset(index as isize)).best_alternative_move =
        -(1 as i32) as i16;
    (*node.offset(index as isize)).alternative_score =
        9999 as i32 as i16;
    (*node.offset(index as isize)).flags = flags;
    select_hash_slot(index);
    book_node_count += 1;
    return index;
}


/*
  FIND_OPENING_NAME
  Searches the opening name database read by READ_OPENING_LIST
  and returns a pointer to the name if the position was found,
  NULL otherwise.
*/

pub unsafe fn find_opening_name() -> *const i8 {
    let mut i: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    get_hash(&mut val1, &mut val2, &mut orientation);
    i = 0 as i32;
    while i < 76 as i32 {
        if val1 == opening_list[i as usize].hash_val1 &&
            val2 == opening_list[i as usize].hash_val2 {
            return opening_list[i as usize].name
        }
        i += 1
    }
    return 0 as *const i8;
}
/*
  CLEAR_OSF
  Free all dynamically allocated memory.
*/

pub unsafe fn clear_osf() {
    free(book_hash_table as *mut c_void);
    book_hash_table = 0 as *mut i32;
    free(node as *mut c_void);
    node = 0 as *mut BookNode;
}


/*
  CHECK_FORCED_OPENING
  Checks if the board position fits the provided forced opening line OPENING
  in any rotation; if this is the case, the next move is returned,
  otherwise PASS is returned.
*/

pub unsafe fn check_forced_opening(mut side_to_move: i32,
                                   mut opening:
                                   *const i8)
                                   -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut count: i32 = 0;
    let mut move_count_0: i32 = 0;
    let mut local_side_to_move: i32 = 0;
    let mut same_position: i32 = 0;
    let mut symm_index: i32 = 0;
    let mut symmetry: i32 = 0;
    let mut move_0: [i32; 60] = [0; 60];
    let mut local_board: [i32; 100] = [0; 100];
    let mut move_offset: [i32; 8] =
        [1 as i32, -(1 as i32), 9 as i32,
            -(9 as i32), 10 as i32, -(10 as i32),
            11 as i32, -(11 as i32)];
    move_count_0 =
        strlen(opening).wrapping_div(2 as i32 as u64) as
            i32;
    if move_count_0 <= disks_played { return -(1 as i32) }
    i = 0 as i32;
    while i < move_count_0 {
        move_0[i as usize] =
            10 as i32 *
                (*opening.offset((2 as i32 * i + 1 as i32) as
                    isize) as i32 - '0' as i32) +
                tolower(*opening.offset((2 as i32 * i) as isize) as
                    i32) - 'a' as i32 + 1 as i32;
        i += 1
    }
    /* Play through the given opening line until the number of discs
       matches that on the actual board. */
    pos = 11 as i32;
    while pos <= 88 as i32 {
        local_board[pos as usize] = 1 as i32;
        pos += 1
    }
    local_board[54 as i32 as usize] = 0 as i32;
    local_board[45 as i32 as usize] =
        local_board[54 as i32 as usize];
    local_board[55 as i32 as usize] = 2 as i32;
    local_board[44 as i32 as usize] =
        local_board[55 as i32 as usize];
    local_side_to_move = 0 as i32;
    i = 0 as i32;
    while i < disks_played {
        j = 0 as i32;
        while j < 8 as i32 {
            pos = move_0[i as usize] + move_offset[j as usize];
            count = 0 as i32;
            while local_board[pos as usize] ==
                0 as i32 + 2 as i32 - local_side_to_move
            {
                pos += move_offset[j as usize];
                count += 1
            }
            if local_board[pos as usize] == local_side_to_move {
                pos -= move_offset[j as usize];
                while pos != move_0[i as usize] {
                    local_board[pos as usize] = local_side_to_move;
                    pos -= move_offset[j as usize]
                }
            }
            j += 1
        }
        local_board[move_0[i as usize] as usize] = local_side_to_move;
        local_side_to_move =
            0 as i32 + 2 as i32 - local_side_to_move;
        i += 1
    }
    if local_side_to_move != side_to_move { return -(1 as i32) }
    /* Check if any of the 8 symmetries make the board after the opening
       line match the current board. The initial symmetry is chosen
       randomly to avoid the same symmetry being chosen all the time.
       This is not a perfect scheme but good enough. */
    symmetry = abs(my_random() as i32) % 8 as i32;
    symm_index = 0 as i32;
    while symm_index < 8 as i32 {
        same_position = 1 as i32;
        i = 1 as i32;
        while i <= 8 as i32 && same_position != 0 {
            j = 1 as i32;
            while j <= 8 as i32 {
                pos = 10 as i32 * i + j;
                if board[pos as usize] !=
                    local_board[*symmetry_map[symmetry as
                        usize].offset(pos as
                        isize)
                        as usize] {
                    same_position = 0 as i32
                }
                j += 1
            }
            i += 1
        }
        if same_position != 0 {
            return *inv_symmetry_map[symmetry as
                usize].offset(move_0[disks_played as
                usize] as
                isize)
        }
        symm_index += 1;
        symmetry = (symmetry + 1 as i32) % 8 as i32
    }
    return -(1 as i32);
}

/*
  NEGA_SCOUT
  This wrapper on top of TREE_SEARCH is used by EVALUATE_NODE
  to search the possible deviations.
*/
pub unsafe fn nega_scout(mut depth: i32,
                     mut allow_mpc: i32,
                     mut side_to_move: i32,
                     mut allowed_count: i32,
                     mut allowed_moves: *mut i32,
                     mut alpha: i32, mut beta: i32,
                     mut best_score: *mut i32,
                     mut best_index: *mut i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut curr_alpha: i32 = 0;
    let mut curr_depth: i32 = 0;
    let mut low_score: i32 = 0;
    let mut high_score: i32 = 0;
    let mut best_move: i32 = 0;
    let mut current_score: i32 = 0;
    reset_counter(&mut nodes);
    low_score = -(12345678 as i32);
    /* To avoid spurious hash table entries to take out the effect
       of the averaging done, the hash table drafts are changed prior
       to each node being searched. */
    clear_hash_drafts();
    determine_hash_values(side_to_move, board.as_mut_ptr());
    /* First determine the best move in the current position
       and its score when searched to depth DEPTH.
       This is done using standard negascout with iterative deepening. */
    curr_depth = 2 as i32 - depth % 2 as i32;
    while curr_depth <= depth {
        low_score = -(12345678 as i32);
        curr_alpha = -(12345678 as i32);
        i = 0 as i32;
        while i < allowed_count {
            make_move(side_to_move, *allowed_moves.offset(i as isize),
                      1 as i32);
            piece_count[0 as i32 as usize][disks_played as usize] =
                disc_count(0 as i32);
            piece_count[2 as i32 as usize][disks_played as usize] =
                disc_count(2 as i32);
            last_panic_check = 0.0f64;
            if i == 0 as i32 {
                current_score =
                    -tree_search(1 as i32, curr_depth,
                                 0 as i32 + 2 as i32 -
                                     side_to_move, -(12345678 as i32),
                                 12345678 as i32, 1 as i32,
                                 allow_mpc, 1 as i32);
                low_score = current_score;
                *best_index = i
            } else {
                curr_alpha =
                    if low_score > curr_alpha {
                        low_score
                    } else { curr_alpha };
                current_score =
                    -tree_search(1 as i32, curr_depth,
                                 0 as i32 + 2 as i32 -
                                     side_to_move,
                                 -(curr_alpha + 1 as i32),
                                 -curr_alpha, 1 as i32, allow_mpc,
                                 1 as i32);
                if current_score > curr_alpha {
                    current_score =
                        -tree_search(1 as i32, curr_depth,
                                     0 as i32 + 2 as i32 -
                                         side_to_move,
                                     -(12345678 as i32),
                                     12345678 as i32,
                                     1 as i32, allow_mpc,
                                     1 as i32);
                    if current_score > low_score {
                        low_score = current_score;
                        *best_index = i
                    }
                } else if current_score > low_score {
                    low_score = current_score;
                    *best_index = i
                }
            }
            unmake_move(side_to_move, *allowed_moves.offset(i as isize));
            i += 1
        }
        /* Float the best move so far to the top of the list */
        best_move = *allowed_moves.offset(*best_index as isize);
        j = *best_index;
        while j >= 1 as i32 {
            *allowed_moves.offset(j as isize) =
                *allowed_moves.offset((j - 1 as i32) as isize);
            j -= 1
        }
        *allowed_moves.offset(0 as i32 as isize) = best_move;
        *best_index = 0 as i32;
        curr_depth += 2 as i32
    }
    /* Then find the score for the best move when searched
       to depth DEPTH+1 */
    make_move(side_to_move, *allowed_moves.offset(*best_index as isize),
              1 as i32);
    piece_count[0 as i32 as usize][disks_played as usize] =
        disc_count(0 as i32);
    piece_count[2 as i32 as usize][disks_played as usize] =
        disc_count(2 as i32);
    last_panic_check = 0.0f64;
    high_score =
        -tree_search(1 as i32, depth + 1 as i32,
                     0 as i32 + 2 as i32 - side_to_move,
                     -(12345678 as i32), 12345678 as i32,
                     1 as i32, allow_mpc, 1 as i32);
    unmake_move(side_to_move, *allowed_moves.offset(*best_index as isize));
    /* To remove the oscillations between odd and even search depths
       the score for the deviation is the average between the two scores. */
    *best_score = (low_score + high_score) / 2 as i32;
}

/*
   EVALUATE_NODE
   Applies a search to a predetermined depth to find the best
   alternative move in a position.
   Note: This function assumes that generate_all() has been
         called prior to it being called.
*/
pub unsafe fn evaluate_node(mut index: i32) {
    let mut i: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut alternative_move_count: i32 = 0;
    let mut this_move: i32 = 0;
    let mut best_move: i32 = 0;
    let mut child: i32 = 0;
    let mut allow_mpc: i32 = 0;
    let mut depth: i32 = 0;
    let mut best_index: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut feasible_move: [i32; 64] = [0; 64];
    let mut best_score: i32 = 0;
    /* Don't evaluate nodes that already have been searched deep enough */
    depth = get_node_depth(index);
    if depth >= search_depth &&
        (*node.offset(index as isize)).alternative_score as i32 !=
            9999 as i32 {
        return
    }
    /* If the node has been evaluated and its score is outside the
       eval and minimax windows, bail out. */
    if (*node.offset(index as isize)).alternative_score as i32 !=
        9999 as i32 {
        if abs((*node.offset(index as isize)).alternative_score as
            i32) < min_eval_span ||
            abs((*node.offset(index as isize)).alternative_score as
                i32) > max_eval_span {
            return
        }
        if abs((*node.offset(index as isize)).black_minimax_score as
            i32) < min_negamax_span ||
            abs((*node.offset(index as isize)).black_minimax_score as
                i32) > max_negamax_span {
            return
        }
    }
    if (*node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    remove_coeffs(disks_played - 8 as i32);
    clear_panic_abort();
    piece_count[0 as i32 as usize][disks_played as usize] =
        disc_count(0 as i32);
    piece_count[2 as i32 as usize][disks_played as usize] =
        disc_count(2 as i32);
    /* Find the moves which haven't been tried from this position */
    alternative_move_count = 0 as i32;
    i = 0 as i32;
    while i < move_count[disks_played as usize] {
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child == -(1 as i32) {
            let fresh16 = alternative_move_count;
            alternative_move_count = alternative_move_count + 1;
            feasible_move[fresh16 as usize] = this_move
        }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    if alternative_move_count == 0 as i32 {
        /* There weren't any such moves */
        exhausted_node_count += 1;
        (*node.offset(index as isize)).best_alternative_move =
            -(2 as i32) as i16;
        (*node.offset(index as isize)).alternative_score =
            9999 as i32 as i16
    } else {
        /* Find the best of those moves */
        allow_mpc = (search_depth >= 9 as i32) as i32;
        nega_scout(search_depth, allow_mpc, side_to_move,
                   alternative_move_count, feasible_move.as_mut_ptr(),
                   -(12345678 as i32), 12345678 as i32,
                   &mut best_score, &mut best_index);
        best_move = feasible_move[best_index as usize];
        evaluated_count += 1;
        if side_to_move == 0 as i32 {
            (*node.offset(index as isize)).alternative_score =
                best_score as i16
        } else {
            (*node.offset(index as isize)).alternative_score =
                -best_score as i16
        }
        get_hash(&mut val1, &mut val2, &mut orientation);
        (*node.offset(index as isize)).best_alternative_move =
            *symmetry_map[orientation as usize].offset(best_move as isize) as
                i16
    }
    clear_node_depth(index);
    set_node_depth(index, search_depth);
}

/*
   DO_VALIDATE
   Recursively makes sure a subtree doesn't contain any midgame
   node without a deviation move.
*/
pub unsafe fn do_validate(mut index: i32) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    if evaluated_count >= max_eval_count { return }
    if (*node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move);
    if (*node.offset(index as isize)).flags as i32 &
        (16 as i32 | 4 as i32) == 0 &&
        (*node.offset(index as isize)).alternative_score as i32 ==
            9999 as i32 &&
        (*node.offset(index as isize)).best_alternative_move as i32
            != -(2 as i32) {
        evaluate_node(index);
    }
    i = 0 as i32;
    while i < move_count[disks_played as usize] {
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as i32) { do_validate(child); }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    let ref mut fresh19 = (*node.offset(index as isize)).flags;
    *fresh19 = (*fresh19 as i32 ^ 8 as i32) as u16;
}

/*
   DO_EVALUATE
   Recursively makes sure a subtree is evaluated to
   the specified depth.
*/
pub unsafe fn do_evaluate(mut index: i32) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    if evaluated_count >= max_eval_count { return }
    if (*node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move);
    if (*node.offset(index as isize)).flags as i32 &
        (16 as i32 | 4 as i32) == 0 {
        evaluate_node(index);
    }
    if evaluated_count >=
        (evaluation_stage + 1 as i32) * max_eval_count /
            25 as i32 {
        evaluation_stage += 1;
        report_do_evaluate(evaluation_stage);
    }
    i = 0 as i32;
    while i < move_count[disks_played as usize] {
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as i32) { do_evaluate(child); }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    let ref mut fresh17 = (*node.offset(index as isize)).flags;
    *fresh17 = (*fresh17 as i32 ^ 8 as i32) as u16;
}


pub unsafe fn compute_feasible_count() -> i32 {
    let mut feasible_count = 0 as i32;
    let mut i = 0 as i32;
    while i < book_node_count {
        let ref mut fresh18 = (*node.offset(i as isize)).flags;
        *fresh18 =
            (*fresh18 as i32 | 8 as i32) as u16;
        if ((*node.offset(i as isize)).alternative_score as i32 ==
            9999 as i32 ||
            get_node_depth(i) < search_depth &&
                abs((*node.offset(i as isize)).alternative_score as
                    i32) >= min_eval_span &&
                abs((*node.offset(i as isize)).alternative_score as
                    i32) <= max_eval_span &&
                abs((*node.offset(i as isize)).black_minimax_score as
                    i32) >= min_negamax_span &&
                abs((*node.offset(i as isize)).black_minimax_score as
                    i32) <= max_negamax_span) &&
            (*node.offset(i as isize)).flags as i32 &
                (4 as i32 | 16 as i32) == 0 {
            feasible_count += 1
        }
        i += 1
    }
    feasible_count
}

// extracted from validate_tree
pub unsafe fn validate_prepared_tree() -> i32 {
    exhausted_node_count = 0 as i32;
    evaluated_count = 0 as i32;
    evaluation_stage = 0 as i32;
    let mut feasible_count = 0 as i32;
    let mut i = 0 as i32;
    while i < book_node_count {
        if (*node.offset(i as isize)).flags as i32 &
            (4 as i32 | 16 as i32) == 0 &&
            (*node.offset(i as isize)).alternative_score as i32 ==
                9999 as i32 &&
            (*node.offset(i as isize)).best_alternative_move as i32
                != -(2 as i32) {
            feasible_count += 1
        }
        i += 1
    }
    max_eval_count =
        if feasible_count < max_batch_size {
            feasible_count
        } else { max_batch_size };
    if feasible_count > 0 as i32 {
        i = 0 as i32;
        while i < book_node_count {
            let ref mut fresh20 = (*node.offset(i as isize)).flags;
            *fresh20 =
                (*fresh20 as i32 | 8 as i32) as
                    u16;
            i += 1
        }
        do_validate(0 as i32);
    }
    return evaluated_count;
}