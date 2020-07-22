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
use crate::src::stubs::{abs, ceil};


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
