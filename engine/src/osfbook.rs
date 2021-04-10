use engine_traits::Offset;
use flip::unflip::FlipStack;

use crate::{
    src::{
        counter::reset_counter,
        game::CandidateMove,
        getcoeff::remove_coeffs,
        hash::clear_hash_drafts,
        moves::{generate_specific, make_move, make_move_no_hash, unmake_move_no_hash},
        opname::opening_list,
        patterns::{compute_line_patterns, flip8},
        search::{create_eval_info, disc_count},
        zebra::EvaluationType
    }
};
use crate::src::error::FrontEnd;
use crate::src::game::ForcedOpening;
use crate::src::game::setup_non_file_based_game;
use crate::src::globals::{Board, BoardState};
use crate::src::hash::{add_hash, HashState, setup_hash};
use crate::src::midgame::tree_search;
use crate::src::moves::{generate_all, MovesState, unmake_move};
use crate::src::myrandom::MyRandom;
use crate::src::search::SearchState;
use crate::src::stubs::{abs, ceil, floor};
use crate::src::zebra::{DrawMode, GameMode};
use crate::src::zebra::DrawMode::{BLACK_WINS, OPPONENT_WINS, WHITE_WINS};
use crate::src::zebra::EvalResult::{DRAWN_POSITION, LOST_POSITION, UNSOLVED_POSITION, WON_POSITION};
use crate::src::zebra::EvalType::{EXACT_EVAL, MIDGAME_EVAL, UNDEFINED_EVAL, WLD_EVAL};
use crate::src::zebra::GameMode::PRIVATE_GAME;

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

pub struct Book {
    pub deviation_bonus: f64,
    pub search_depth: i32,
    pub node_table_size: i32,
    pub hash_table_size: i32,
    pub total_game_count: i32,
    pub book_node_count: i32,
    pub evaluated_count: i32,
    pub evaluation_stage: i32,
    pub max_eval_count: i32,
    pub max_batch_size: i32,
    pub exhausted_node_count: i32,
    pub max_slack: i32,
    pub low_deviation_threshold: i32,
    pub high_deviation_threshold: i32,
    pub min_eval_span: i32,
    pub max_eval_span: i32,
    pub min_negamax_span: i32,
    pub max_negamax_span: i32,
    pub leaf_count: i32,
    pub bad_leaf_count: i32,
    pub really_bad_leaf_count: i32,
    pub unreachable_count: i32,
    pub candidate_count: i32,
    pub force_black: i32,
    pub force_white: i32,
    pub used_slack: [i32; 3],
    pub exact_count: [i32; 61],
    pub wld_count: [i32; 61],
    pub exhausted_count: [i32; 61],
    pub common_count: [i32; 61],
    pub symmetry_map: [&'static [i8]; 8],
    pub inv_symmetry_map: [&'static [i8]; 8],
    pub line_hash: [[[i32; 6561]; 8]; 2],
    pub book_hash_table: Vec<i32>,
    pub draw_mode: DrawMode,
    pub game_mode: GameMode,
    pub node: Vec<BookNode>,
    pub candidate_list: [CandidateMove; 60]
}
pub struct BookMaps {
    pub b1_b1_map: [i8; 100],
    pub g1_b1_map: [i8; 100],
    pub g8_b1_map: [i8; 100],
    pub b8_b1_map: [i8; 100],
    pub a2_b1_map: [i8; 100],
    pub a7_b1_map: [i8; 100],
    pub h7_b1_map: [i8; 100],
    pub h2_b1_map: [i8; 100],
}
pub static BOOK_MAPS: BookMaps = create_book_maps();

pub const fn create_book_maps() -> BookMaps {
    let mut maps = BookMaps {
        //FIXME it seems like these maps are only initialized in legacy-zebra, not in the engine,
        // why is that? Is it my mistake?
        b1_b1_map: [0; 100],
        g1_b1_map: [0; 100],
        g8_b1_map: [0; 100],
        b8_b1_map: [0; 100],
        a2_b1_map: [0; 100],
        a7_b1_map: [0; 100],
        h7_b1_map: [0; 100],
        h2_b1_map: [0; 100],
    };
    let mut i = 0;
    let mut j = 0;
    let mut pos = 0;
    i = 1;
    while i <= 8 {
        j = 1;
        while j <= 8 {
            pos = 10  * i + j;
            maps.b1_b1_map[pos as usize] = pos;
            maps.g1_b1_map[pos as usize] = 10 * i + (9 - j);
            maps.g8_b1_map[pos as usize] = 10 * (9 - i) + (9 - j);
            maps.b8_b1_map[pos as usize] = 10 * (9 - i) + j;
            maps.a2_b1_map[pos as usize] = 10 * j + i;
            maps.a7_b1_map[pos as usize] = 10 * j + (9 - i);
            maps.h7_b1_map[pos as usize] = 10 * (9 - j) + (9 - i);
            maps.h2_b1_map[pos as usize] = 10 * (9 - j) + i;
            j += 1
        }
        i += 1
    };

    // Following block should be equivalent of this commented out block from the original code
    // I don't really understand its original purpose, though

    // let mut i = 0;
    // while i < 8 as i32 {
    //     *book.symmetry_map[i as usize] = 0;
    //     i += 1
    // }

    maps.b1_b1_map[0] = 0;
    maps.g1_b1_map[0] = 0;
    maps.g8_b1_map[0] = 0;
    maps.b8_b1_map[0] = 0;
    maps.a2_b1_map[0] = 0;
    maps.a7_b1_map[0] = 0;
    maps.h7_b1_map[0] = 0;
    maps.h2_b1_map[0] = 0;
    maps
}

impl Book {
    pub const fn new() -> Self {
        Book {
            deviation_bonus: 0.,
            search_depth: 0,
            node_table_size: 0,
            hash_table_size: 0,
            total_game_count: 0,
            book_node_count: 0,
            evaluated_count: 0,
            evaluation_stage: 0,
            max_eval_count: 0,
            max_batch_size: 0,
            exhausted_node_count: 0,
            max_slack: 0,
            low_deviation_threshold: 0,
            high_deviation_threshold: 0,
            min_eval_span: 0,
            max_eval_span: 0,
            min_negamax_span: 0,
            max_negamax_span: 0,
            leaf_count: 0,
            bad_leaf_count: 0,
            really_bad_leaf_count: 0,
            unreachable_count: 0,
            candidate_count: 0,
            force_black: 0,
            force_white: 0,
            used_slack: [0; 3],
            exact_count: [0; 61],
            wld_count: [0; 61],
            exhausted_count: [0; 61],
            common_count: [0; 61],
            symmetry_map: [&[]; 8],
            inv_symmetry_map: [&[]; 8],
            line_hash: [[[0; 6561]; 8]; 2],
            book_hash_table: Vec::new(),
            draw_mode: OPPONENT_WINS,
            game_mode: PRIVATE_GAME,
            node: Vec::new(),
            candidate_list: [CandidateMove { move_0: 0, score: 0, flags: 0, parent_flags: 0 }; 60]
        }
    }
}


/*
   PROBE_HASH_TABLE
   Search for a certain hash code in the hash table.
*/
pub fn probe_hash_table(val1: i32, val2: i32, book: &mut Book) -> i32 {
    let book = book;
    if (book).hash_table_size == 0 as i32 {
        -(1 as i32)
    } else {
        let mut slot = val1 % book.hash_table_size;
        while *book.book_hash_table.offset(slot as isize) != -(1 as i32) &&
            ((*book.node.offset(*book.book_hash_table.offset(slot as isize) as
                isize)).hash_val2 != val2 ||
                (*book.node.offset(*book.book_hash_table.offset(slot as isize) as
                    isize)).hash_val1 != val1) {
            slot = (slot + 1 as i32) % book.hash_table_size
        }
        slot
    }
}

/*
   CLEAR_NODE_DEPTH
   Changes the flags of a node so that the search depth
   bits are cleared.
*/
pub fn clear_node_depth(index: i32, book: &mut Book) {
    let mut depth: i32 = 0;
    depth =
        (*book.node.offset(index as isize)).flags as i32 >>
            10 as i32;
    let ref mut fresh0 = (*book.node.offset(index as isize)).flags;
    *fresh0 =
        (*fresh0 as i32 ^ depth << 10 as i32) as
            u16;
}
/*
   GET_NODE_DEPTH
*/
pub fn get_node_depth(index: i32, book: &mut Book) -> i32 {
    return (*book.node.offset(index as isize)).flags as i32 >> 10 as i32;
}
/*
   SET_NODE_DEPTH
   Marks a node as being searched to a certain depth.
*/
pub fn set_node_depth(index: i32, depth: i32, book: &mut Book) {
    let ref mut fresh1 = (*book.node.offset(index as isize)).flags;
    *fresh1 = (*fresh1 as i32 | depth << 10 as i32) as u16;
}

/*
  SET_MAX_BATCH_SIZE
  Specify the maximum number of nodes to evaluate.
*/

pub fn set_max_batch_size(size: i32, book: &mut Book) {
    book.max_batch_size = size;
}
/*
   SET_DEVIATION_VALUE
   Sets the number of disks where a penalty is incurred if
   the deviation from the book line comes later than that
   stage; also set the punishment per move after the threshold.
*/

pub fn set_deviation_value(low_threshold: i32, high_threshold: i32, bonus: f64, book: &mut Book) {
    book.low_deviation_threshold = low_threshold;
    book.high_deviation_threshold = high_threshold;
    book.deviation_bonus = bonus;
}

/*
   RESET_BOOK_SEARCH
   Sets the used slack count to zero.
*/

pub fn reset_book_search(book: &mut Book) {
    book.used_slack[0] = 0.0f64 as i32;
    book.used_slack[2] = 0.0f64 as i32;
}

impl Book {
    /*
       g_book.SET_SLACK(
       Sets the total amount of negamaxed evaluation that
       the program is willing to trade for randomness.
    */

    pub fn set_slack(&mut self, slack: i32) {
        self.max_slack = slack;
    }
    /*
      g_book.(
      Specifies how book draws should be treated.
    */

    pub fn set_draw_mode(&mut self, mode: DrawMode) {
        self.draw_mode = mode;
    }
    /*
      g_book.(
      Specifies if the book is in private or public mode.
    */

    pub fn set_game_mode(&mut self, mode: GameMode) {
        self.game_mode = mode;
    }
    /*
      g_book.(
      g_book.(
      Specifies if the moves for either of the players are to
      be forced when recursing the tree.
    */

    pub fn set_black_force(&mut self, force: i32) {
        self.force_black = force;
    }

    pub fn set_white_force(&mut self, force: i32) {
        self.force_white = force;
    }

    /*
      g_book.(
      GET_CANDIDATE
      Accessor functions for the data structure created by
      FILL_MOVE_ALTERNATIVES.
    */

    pub fn get_candidate_count(&self) -> i32 {
        return self.candidate_count;
    }

    pub fn get_candidate(&self, index: i32) -> CandidateMove {
        return self.candidate_list[index as usize];
    }
}

/*
   GET_HASH
   Return the hash values for the current board position.
   The position is rotated so that the 64-bit hash value
   is minimized among all rotations. This ensures detection
   of all transpositions.
   See also init_maps().
*/


pub fn get_hash(val0: &mut i32, val1: &mut i32, orientation: &mut i32, book: & Book, board1: &Board) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut min_map: i32 = 0;
    let mut min_hash0: i32 = 0;
    let mut min_hash1: i32 = 0;
    let mut out: [[i32; 2]; 8] = [[0; 2]; 8];
    /* Calculate the 8 different 64-bit hash values for the
       different rotations. */
    let mut row_pattern: [i32; 8] = [0;8];
    let mut col_pattern: [i32; 8] = [0; 8];
    compute_line_patterns(board1, &mut row_pattern, &mut col_pattern);
    i = 0;
    while i < 8 as i32 {
        j = 0;
        while j < 2 as i32 {
            out[i as usize][j as usize] = 0;
            j += 1
        }
        i += 1
    }
    i = 0;
    while i < 8 as i32 {
        /* b1 -> b1 */
        out[0][0] ^=
            book.line_hash[0][i as
                usize][row_pattern[i as usize] as usize];
        out[0][1] ^=
            book.line_hash[1][i as
                usize][row_pattern[i as usize] as usize];
        /* g1 -> b1 */
        out[1][0] ^=
            book.line_hash[0][i as
                usize][flip8[row_pattern[i as usize] as
                usize] as usize];
        out[1][1] ^=
            book.line_hash[1][i as
                usize][flip8[row_pattern[i as usize] as
                usize] as usize];
        /* g8 -> b1 */
        out[2][0] ^=
            book.line_hash[0][i as
                usize][flip8[row_pattern[(7 as
                i32
                - i) as
                usize] as
                usize] as usize];
        out[2][1] ^=
            book.line_hash[1][i as
                usize][flip8[row_pattern[(7 as
                i32
                - i) as
                usize] as
                usize] as usize];
        /* b8 -> b1 */
        out[3][0] ^=
            book.line_hash[0][i as
                usize][row_pattern[(7 as i32 - i)
                as usize] as
                usize];
        out[3][1] ^=
            book.line_hash[1][i as
                usize][row_pattern[(7 as i32 - i)
                as usize] as
                usize];
        /* a2 -> b1 */
        out[4][0] ^=
            book.line_hash[0][i as
                usize][col_pattern[i as usize] as usize];
        out[4][1] ^=
            book.line_hash[1][i as
                usize][col_pattern[i as usize] as usize];
        /* a7 -> b1 */
        out[5][0] ^=
            book.line_hash[0][i as
                usize][flip8[col_pattern[i as usize] as
                usize] as usize];
        out[5][1] ^=
            book.line_hash[1][i as
                usize][flip8[col_pattern[i as usize] as
                usize] as usize];
        /* h7 -> b1 */
        out[6][0] ^=
            book.line_hash[0][i as
                usize][flip8[col_pattern[(7 as
                i32
                - i) as
                usize] as
                usize] as usize];
        out[6][1] ^=
            book.line_hash[1][i as
                usize][flip8[col_pattern[(7 as
                i32
                - i) as
                usize] as
                usize] as usize];
        /* h2 -> b1 */
        out[7][0] ^=
            book.line_hash[0][i as
                usize][col_pattern[(7 as i32 - i)
                as usize] as
                usize];
        out[7][1] ^=
            book.line_hash[1][i as
                usize][col_pattern[(7 as i32 - i)
                as usize] as
                usize];
        i += 1
    }
    /* Find the rotation minimizing the hash index.
       If two hash indices are equal, map number is implicitly used
       as tie-breaker. */
    min_map = 0;
    min_hash0 = out[0][0];
    min_hash1 = out[0][1];
    i = 1;
    while i < 8 as i32 {
        if out[i as usize][0] < min_hash0 ||
            out[i as usize][0] == min_hash0 &&
                out[i as usize][1] < min_hash1 {
            min_map = i;
            min_hash0 = out[i as usize][0];
            min_hash1 = out[i as usize][1]
        }
        i += 1
    }
    *val0 = abs(min_hash0);
    *val1 = abs(min_hash1);
    *orientation = min_map;
}
impl Book {
/*
   SET_SEARCH_DEPTH
   When finding move alternatives, searches to depth DEPTH
   will be performed.
*/

pub fn set_search_depth(&mut self, depth: i32) {
    self.search_depth = depth;
}
/*
  SET_EVAL_SPAN
  Specify the evaluation value interval where nodes are re-evaluated.
*/

pub fn set_eval_span(&mut self, min_span: f64,
                            max_span: f64) {
    self.min_eval_span = ceil(min_span * 128.0f64) as i32;
    self.max_eval_span = ceil(max_span * 128.0f64) as i32;
}
/*
  SET_NEGAMAX_SPAN
  Specify the negamax value interval where nodes are re-evaluated.
*/

pub fn set_negamax_span(&mut self, min_span: f64, max_span: f64) {
    self.min_negamax_span = ceil(min_span * 128.0f64) as i32;
    self.max_negamax_span = ceil(max_span * 128.0f64) as i32;
}
}
/*
   ADJUST_SCORE
   Tweak a score as to encourage early deviations.
*/
pub fn adjust_score(score: i32, side_to_move: i32, book: &mut Book, disks_played_: i32) -> i32 {
    let mut adjustment: i32 = 0;
    let mut adjust_steps: i32 = 0;
    adjust_steps = book.high_deviation_threshold - disks_played_;
    if adjust_steps < 0 as i32 {
        adjustment = 0 as i32
    } else {
        if disks_played_ < book.low_deviation_threshold {
            adjust_steps = book.high_deviation_threshold - book.low_deviation_threshold
        }
        adjustment =
            floor(adjust_steps as f64 * book.deviation_bonus * 128.0f64)
                as i32;
        if side_to_move == 2 as i32 { adjustment = -adjustment }
    }
    return score + adjustment;
}


/*
  FIND_OPENING_NAME
  Searches the opening name database read by READ_OPENING_LIST
  and returns a pointer to the name if the position was found,
  NULL otherwise.
*/

pub fn find_opening_name(book: &Book, board: &Board) -> Option<&'static [u8]> {
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    get_hash(&mut val1, &mut val2, &mut orientation, book, board);
    let mut i = 0;
    while i < opening_list.len() {
        if val1 == opening_list[i].hash_val1 &&
            val2 == opening_list[i].hash_val2 {
            return Some(opening_list[i].name)
        }
        i += 1
    }
    None
}

/*
  CHECK_FORCED_OPENING
  Checks if the board position fits the provided forced opening line OPENING
  in any rotation; if this is the case, the next move is returned,
  otherwise PASS is returned.
*/

pub fn check_forced_opening<FE: FrontEnd>(side_to_move: i32, opening: ForcedOpening, board: &[i32; 128], disks_played: i32, book: &Book, random: &mut MyRandom) -> i32 {

    let move_count_0 = opening.move_count;
    if move_count_0 <= disks_played { return -(1 as i32) }
    let mut move_0 = &opening.moves;

    let mut local_board: [i32; 100] = [0; 100];
    let move_offset: [i32; 8] =
        [1 as i32, -(1 as i32), 9 as i32,
            -(9 as i32), 10 as i32, -(10 as i32),
            11 as i32, -(11 as i32)];
    let mut j: i32 = 0;
    let mut count: i32 = 0;
    let mut local_side_to_move: i32 = 0;
    let mut same_position: i32 = 0;
    let mut symm_index: i32 = 0;
    let mut symmetry: i32 = 0;

    /* Play through the given opening line until the number of discs
       matches that on the actual board. */
    let mut pos = 11;
    while pos <= 88 as i32 {
        local_board[pos as usize] = 1;
        pos += 1
    }
    local_board[54] = 0;
    local_board[45] =
        local_board[54];
    local_board[55] = 2;
    local_board[44] =
        local_board[55];
    local_side_to_move = 0;
    let mut i = 0;
    while i < disks_played {
        j = 0;
        while j < 8 as i32 {
            pos = move_0[i as usize] + move_offset[j as usize];
            count = 0;
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
    symmetry = abs(random.my_random() as i32) % 8 as i32;
    symm_index = 0;
    while symm_index < 8 as i32 {
        same_position = 1;
        i = 1;
        while i <= 8 as i32 && same_position != 0 {
            j = 1;
            while j <= 8 as i32 {
                pos = 10 as i32 * i + j;
                if board[pos as usize] !=
                    local_board[*book.symmetry_map[symmetry as
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
            return *book.inv_symmetry_map[symmetry as
                usize].offset(move_0[disks_played as
                usize] as isize) as _
        }
        symm_index += 1;
        symmetry = (symmetry + 1 as i32) % 8 as i32
    }
    return -(1 as i32);
}

/*
  FILL_ENDGAME_HASH
  Recursively transfer information from all solved nodes in the
  book hash table to the game hash table.
*/

pub fn fill_endgame_hash(cutoff: i32, level: i32
                                ,book: &mut Book
                                ,board_state_: &mut BoardState
                                ,moves_state_: &mut MovesState
                                ,search_state_: &SearchState
                                ,hash_state_: &mut HashState
                                ,flip_stack: &mut FlipStack
) {
    let mut i: i32 = 0;
    let mut this_index: i32 = 0;
    let mut child_index: i32 = 0;
    let mut matching_move: i32 = 0;
    let mut signed_score: i32 = 0;
    let mut bound: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut is_full: i32 = 0;
    let mut is_wld: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    if level >= 5 as i32 { return }
    let val0___ = &mut val1;
    let val1___ = &mut val2;
    let orientation___ = &mut orientation;
    get_hash(val0___, val1___, orientation___, book, &board_state_.board);
    slot = probe_hash_table(val1, val2, book);
    this_index = *book.book_hash_table.offset(slot as isize);
    /* If the position wasn't found in the hash table, return. */
    if slot == -(1 as i32) ||
        *book.book_hash_table.offset(slot as isize) == -(1 as i32) {
        return
    }
    /* Check the status of the g_book.node */
    is_full =
        (*book.node.offset(this_index as isize)).flags as i32 &
            16 as i32;
    is_wld =
        (*book.node.offset(this_index as isize)).flags as i32 &
            4 as i32;
    /* Match the status of the node with those of the children and
       recursively treat the entire subtree of the node */
    if (*book.node.offset(*book.book_hash_table.offset(slot as isize) as isize)).flags
        as i32 & 1 as i32 != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    matching_move = -(1 as i32);
    generate_all(side_to_move, moves_state_, search_state_, &board_state_.board);
    i = 0;
    while i < moves_state_.move_count[moves_state_.disks_played as usize] {
        this_move = moves_state_.move_list[moves_state_.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32, moves_state_, board_state_, hash_state_, flip_stack );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, book, &board_state_.board);
        slot = probe_hash_table(val1, val2, book);
        child_index = *book.book_hash_table.offset(slot as isize);
        if child_index != -(1 as i32) {
            if moves_state_.disks_played < 60 as i32 - cutoff {
                fill_endgame_hash(cutoff, level + 1 as i32,
                book
                ,board_state_
                ,moves_state_
                ,search_state_
                ,hash_state_
                ,flip_stack
                );
            }
            if is_full != 0 {
                /* Any child with matching exact score? */
                if (*book.node.offset(child_index as isize)).flags as i32 &
                    16 as i32 != 0 &&
                    (*book.node.offset(child_index as
                        isize)).black_minimax_score as
                        i32 ==
                        (*book.node.offset(this_index as
                            isize)).black_minimax_score as
                            i32 {
                    matching_move = this_move
                }
            } else if is_wld != 0 {
                /* Any child with matching WLD results? */
                if (*book.node.offset(child_index as isize)).flags as i32 &
                    (16 as i32 | 4 as i32) != 0 {
                    if side_to_move == 0 as i32 {
                        if (*book.node.offset(child_index as
                            isize)).black_minimax_score as
                            i32 >=
                            (*book.node.offset(this_index as
                                isize)).black_minimax_score
                                as i32 {
                            matching_move = this_move
                        }
                    } else if (*book.node.offset(child_index as
                        isize)).black_minimax_score as
                        i32 <=
                        (*book.node.offset(this_index as
                            isize)).black_minimax_score
                            as i32 {
                        matching_move = this_move
                    }
                }
            }
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state_.board, moves_state_, hash_state_, flip_stack);
        };
        i += 1
    }
    if matching_move != -(1 as i32) {
        /* Store the information */
        signed_score =
            (*book.node.offset(this_index as isize)).black_minimax_score as
                i32;
        if side_to_move == 2 as i32 { signed_score = -signed_score }
        if signed_score > 30000 as i32 {
            signed_score -= 30000 as i32
        } else if signed_score < -(30000 as i32) {
            signed_score += 30000 as i32
        } else if abs(signed_score) == 30000 as i32 - 1 as i32
        {
            signed_score = 0 as i32
        }
        if is_full != 0 {
            bound = 4 as i32
        } else if signed_score >= 0 as i32 {
            bound = 1 as i32
        } else { bound = 2 as i32 }
        add_hash(hash_state_, 1 as i32, signed_score, matching_move,
                 16 as i32 | bound, 60 as i32 - moves_state_.disks_played,
                 0 as i32);
    };
}

/*
  FILL_MOVE_ALTERNATIVES
  Fills the data structure CANDIDATE_LIST with information
  about the book moves available in the current position.
  FLAGS specifies a subset of the flag bits which have to be set
  for a position to be considered. Notice that FLAGS=0 accepts
  any flag combination.
*/

pub fn fill_move_alternatives<FE: FrontEnd>(side_to_move: i32,
                                                   flags: i32,
                                                   book: &mut Book,
                                                   board_state_: &mut BoardState,
                                                   moves_state_: &mut MovesState,
                                                   search_state_: &SearchState,
                                                   flip_stack: &mut FlipStack,
                                                   hash_state_: &mut HashState
) {
    let mut temp =
        CandidateMove{move_0: 0, score: 0, flags: 0, parent_flags: 0,};
    let mut sign: i32 = 0;
    let mut i: i32 = 0;
    let mut slot: i32 = 0;
    let mut changed: i32 = 0;
    let mut index: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut this_move: i32 = 0;
    let mut alternative_move: i32 = 0;
    let mut score: i32 = 0;
    let mut alternative_score: i32 = 0;
    let mut child_feasible: i32 = 0;
    let mut deviation: i32 = 0;
    let mut root_flags: i32 = 0;
    let board1 = &board_state_.board;
    get_hash(&mut val1, &mut val2, &mut orientation, book, board1);
    slot = probe_hash_table(val1, val2, book);
    /* If the position wasn't found in the hash table, return. */
    if slot == -(1 as i32) ||
        *book.book_hash_table.offset(slot as isize) == -(1 as i32) {
        book.candidate_count = 0;
        return
    } else { index = *book.book_hash_table.offset(slot as isize) }
    /* If the position hasn't got the right flag bits set, return. */
    root_flags = (*book.node.offset(index as isize)).flags as i32;
    if flags != 0 as i32 && root_flags & flags == 0 {
        book.candidate_count = 0;
        return
    }
    if side_to_move == 0 as i32 {
        sign = 1 as i32
    } else { sign = -(1 as i32) }
    alternative_move =
        (*book.node.offset(index as isize)).best_alternative_move as i32;
    if alternative_move > 0 as i32 {
        alternative_move =
            *book.inv_symmetry_map[orientation as
                usize].offset(alternative_move as isize) as _;
        alternative_score =
            adjust_score((*book.node.offset(index as isize)).alternative_score as
                             i32, side_to_move, book, moves_state_.disks_played)
    } else { alternative_score = -(12345678 as i32) }
    generate_all(side_to_move, moves_state_, search_state_, &board_state_.board);
    book.candidate_count = 0;
    i = 0;
    while i < moves_state_.move_count[moves_state_.disks_played as usize] {
        this_move = moves_state_.move_list[moves_state_.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32, moves_state_, board_state_, hash_state_, flip_stack);
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, book, &board_state_.board);
        slot = probe_hash_table(val1, val2, book);
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state_.board, moves_state_, hash_state_, flip_stack);
        };
        /* Check if the move leads to a book position and, if it does,
           whether it has the solve status (WLD or FULL) specified by FLAGS. */
        deviation = 0;
        if slot == -(1 as i32) ||
            *book.book_hash_table.offset(slot as isize) == -(1 as i32) {
            if this_move == alternative_move && flags == 0 {
                score = alternative_score;
                child_feasible = 1;
                deviation = 1 as i32
            } else {
                child_feasible = 0;
                score = 0 as i32
            }
        } else if (*book.node.offset(*book.book_hash_table.offset(slot as isize) as
            isize)).flags as i32 & flags != 0
            || flags == 0 {
            if side_to_move == 0 as i32 {
                score =
                    (*book.node.offset(*book.book_hash_table.offset(slot as isize) as
                        isize)).black_minimax_score as
                        i32
            } else {
                score =
                    (*book.node.offset(*book.book_hash_table.offset(slot as isize) as
                        isize)).white_minimax_score as
                        i32
            }
            child_feasible = 1 as i32
        } else { child_feasible = 0; score = 0 as i32 }
        const EMPTY_HASH_SLOT: i32 = -1;
        if child_feasible != 0 && score == 0 as i32 &&
            (*book.node.offset(index as isize)).flags as i32 &
                4 as i32 == 0 &&
            (*book.book_hash_table.offset(slot as isize) != EMPTY_HASH_SLOT) &&
            (*book.node.offset(*book.book_hash_table.offset(slot as isize) as
                isize)).flags as i32 &
                4 as i32 != 0 {
            /* Check if this is a book draw that should be avoided, i.e., one
               where the current position is not solved but the child position
               is solved for a draw, and the draw mode dictates this draw to
               be a bad one. */
            if book.game_mode as u32 ==
                PRIVATE_GAME as i32 as u32 ||
                (*book.node.offset(*book.book_hash_table.offset(slot as isize) as
                    isize)).flags as i32 &
                    32 as i32 == 0 {
                if side_to_move == 0 as i32 {
                    if book.draw_mode as u32 ==
                        WHITE_WINS as i32 as u32 ||
                        book.draw_mode as u32 ==
                            OPPONENT_WINS as i32 as u32 {
                        FE::report_unwanted_book_draw(this_move);
                        child_feasible = 0 as i32
                    }
                } else if book.draw_mode as u32 ==
                    BLACK_WINS as i32 as u32 ||
                    book.draw_mode as u32 ==
                        OPPONENT_WINS as i32 as u32 {
                    FE::report_unwanted_book_draw(this_move);
                    child_feasible = 0 as i32
                }
            }
        }
        if child_feasible != 0 {
            book.candidate_list[book.candidate_count as usize].move_0 =
                moves_state_.move_list[moves_state_.disks_played as usize][i as usize];
            book.candidate_list[book.candidate_count as usize].score = sign * score;
            if deviation != 0 {
                book.candidate_list[book.candidate_count as usize].flags =
                    64 as i32
            } else {
                book.candidate_list[book.candidate_count as usize].flags =
                    (*book.node.offset(*book.book_hash_table.offset(slot as isize) as
                        isize)).flags as i32
            }
            book.candidate_list[book.candidate_count as usize].parent_flags =
                root_flags;
            book.candidate_count += 1
        }
        i += 1
    }
    if book.candidate_count > 0 as i32 {
        loop
        /* Sort the book moves using bubble sort */
        {
            changed = 0;
            i = 0;
            while i < book.candidate_count - 1 as i32 {
                if book.candidate_list[i as usize].score <
                    book.candidate_list[(i + 1 as i32) as usize].score {
                    changed = 1;
                    temp = book.candidate_list[i as usize];
                    book.candidate_list[i as usize] =
                        book.candidate_list[(i + 1 as i32) as usize];
                    book.candidate_list[(i + 1 as i32) as usize] = temp
                }
                i += 1
            }
            if !(changed != 0) { break ; }
        }
    };
}

/*
   GET_BOOK_MOVE
   Chooses a book move from the list of candidates
   which don't worsen the negamaxed out-of-book
   evaluation by too much.
*/

pub fn get_book_move<FE: FrontEnd>(mut side_to_move: i32,
                                          update_slack: i32,
                                          mut eval_info: &mut EvaluationType, echo: i32,
                                          board_state_: &mut BoardState,
                                          book: &mut Book,
                                          search_state_: &SearchState,
                                          moves_state_: &mut MovesState,
                                          hash_state_: &mut HashState,
                                          random: &mut MyRandom,
                                          flip_stack: &mut FlipStack)
                                          -> i32 {

    let mut i: i32 = 0;
    let mut original_side_to_move: i32 = 0;
    let mut remaining_slack: i32 = 0;
    let mut score: i32 = 0;
    let mut chosen_score: i32 = 0;
    let mut best_score: i32 = 0;
    let mut alternative_score: i32 = 0;
    let mut feasible_count: i32 = 0;
    let mut index: i32 = 0;
    let mut chosen_index: i32 = 0;
    let mut flags: i32 = 0;
    let mut base_flags: i32 = 0;
    let mut random_point: i32 = 0;
    let mut level: i32 = 0;
    let mut continuation: i32 = 0;
    let mut is_feasible: i32 = 0;
    let mut acc_weight: i32 = 0;
    let mut total_weight: i32 = 0;
    let mut best_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut alternative_move: i32 = 0;
    let mut sign: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut slot: i32 = 0;
    let mut weight: [i32; 60] = [0; 60];
    let mut temp_move: [i32; 60] = [0; 60];
    let mut temp_stm: [i32; 60] = [0; 60];
    /* Disable opening book randomness unless the move is going to
       be played on the board by Zebra */
    if update_slack != 0 {
        remaining_slack =
            if book.max_slack - book.used_slack[side_to_move as usize] >
                0 as i32 {
                (book.max_slack) - book.used_slack[side_to_move as usize]
            } else { 0 as i32 }
    } else { remaining_slack = 0 as i32 }
    if echo != 0 && book.candidate_count > 0 as i32 &&
        search_state_.get_ponder_move() == 0 {
        FE::report_in_get_book_move_1(side_to_move, remaining_slack, board_state_, book);
    }
    /* No book move found? */
    if book.candidate_count == 0 as i32 { return -(1 as i32) }
    /* Find the book flags of the original position. */
    let val0___ = &mut val1;
    let val1___ = &mut val2;
    let orientation___ = &mut orientation;
    get_hash(val0___, val1___, orientation___, book, &board_state_.board);
    slot = probe_hash_table(val1, val2, book);
    if slot == -(1 as i32) ||
        *book.book_hash_table.offset(slot as isize) == -(1 as i32) {
        FE::internal_error_in_book_code();
    }
    base_flags =
        (*book.node.offset(*book.book_hash_table.offset(slot as isize) as isize)).flags
            as i32;
    /* If we have an endgame score for the position, we only want to
       consult the book if there is at least one move realizing that score. */
    index = *book.book_hash_table.offset(slot as isize);
    if (*book.node.offset(index as isize)).flags as i32 & 16 as i32
        != 0 {
        if book.candidate_list[0].score <
            (*book.node.offset(index as isize)).black_minimax_score as
                i32 {
            return -(1 as i32)
        }
    } else if (*book.node.offset(index as isize)).flags as i32 &
        4 as i32 != 0 {
        if (*book.node.offset(index as isize)).black_minimax_score as i32 >
            0 as i32 &&
            book.candidate_list[0].score <=
                0 as i32 {
            return -(1 as i32)
        }
    }
    /* Don't randomize among solved moves */
    score = book.candidate_list[0].score;
    if score >= 30000 as i32 { remaining_slack = 0 as i32 }
    feasible_count = 0;
    total_weight = 0;
    while feasible_count < book.candidate_count &&
        book.candidate_list[feasible_count as usize].score >=
            score - remaining_slack {
        weight[feasible_count as usize] =
            2 as i32 * remaining_slack + 1 as i32 -
                (score - book.candidate_list[feasible_count as usize].score);
        total_weight += weight[feasible_count as usize];
        feasible_count += 1
    }
    /* Chose a move at random from the moves which don't worsen
       the position by more than the allowed slack (and, optionally,
       update it). A simple weighting scheme makes the moves with
       scores close to the best move most likely to be chosen. */
    if feasible_count == 1 as i32 {
        chosen_index = 0 as i32
    } else {
        random_point =
            ((random.my_random() >> 10 as i32) %
                total_weight as i64) as i32;
        chosen_index = 0;
        acc_weight = weight[chosen_index as usize];
        while random_point > acc_weight {
            chosen_index += 1;
            acc_weight += weight[chosen_index as usize]
        }
    }
    chosen_score = book.candidate_list[chosen_index as usize].score;
    if update_slack != 0 {
        book.used_slack[side_to_move as usize] += score - chosen_score
    }
    /* Convert the book score to the normal form.
       Note that this should work also for old-style book values. */
    if chosen_score >= 30000 as i32 {
        chosen_score -= 30000 as i32;
        if chosen_score <= 64 as i32 {
            chosen_score *= 128 as i32
        }
    }
    if chosen_score <= -(30000 as i32) {
        chosen_score += 30000 as i32;
        if chosen_score >= -(64 as i32) {
            chosen_score *= 128 as i32
        }
    }
    /* Return the score via the EvaluationType structure */
    flags = book.candidate_list[chosen_index as usize].flags;
    *eval_info =
        create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION, chosen_score,
                         0.0f64, 0 as i32, 1 as i32);
    if base_flags & (16 as i32 | 4 as i32) != 0 &&
        flags & (16 as i32 | 4 as i32) != 0 {
        /* Both the base position and the position after the book move
           are solved. */
        if base_flags & 16 as i32 != 0 &&
            flags & 16 as i32 != 0 {
            (*eval_info).type_0 = EXACT_EVAL
        } else { (*eval_info).type_0 = WLD_EVAL }
        if chosen_score > 0 as i32 {
            (*eval_info).res = WON_POSITION
        } else if chosen_score == 0 as i32 {
            (*eval_info).res = DRAWN_POSITION
        } else { (*eval_info).res = LOST_POSITION }
    } else if flags & 4 as i32 != 0 && chosen_score > 0 as i32
    {
        /* The base position is unknown but the move played leads
           to a won position. */
        (*eval_info).type_0 = WLD_EVAL;
        (*eval_info).res = WON_POSITION
    } else {
        /* No endgame information available. */
        (*eval_info).type_0 = MIDGAME_EVAL
    }
    if echo != 0 {
        FE::report_in_get_book_move_2(chosen_score, chosen_index, &flags, &book.candidate_list, search_state_);
    }
    /* Fill the PV structure with the optimal book line */
    original_side_to_move = side_to_move;
    level = 0;
    temp_move[0] =
        book.candidate_list[chosen_index as usize].move_0;
    loop  {
        temp_stm[level as usize] = side_to_move;
        make_move(side_to_move, temp_move[level as usize], 1 as i32, moves_state_, board_state_, hash_state_, flip_stack);
        level += 1;
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, book, &board_state_.board);
        slot = probe_hash_table(val1, val2, book);
        continuation = 1;
        if slot == -(1 as i32) ||
            *book.book_hash_table.offset(slot as isize) == -(1 as i32) {
            continuation = 0 as i32
        } else {
            alternative_move =
                (*book.node.offset(*book.book_hash_table.offset(slot as isize) as
                    isize)).best_alternative_move as
                    i32;
            if alternative_move > 0 as i32 {
                alternative_move =
                    *book.inv_symmetry_map[orientation as
                        usize].offset(alternative_move as
                        isize) as _;
                alternative_score =
                    adjust_score((*book.node.offset(*book.book_hash_table.offset(slot as
                        isize)
                        as
                        isize)).alternative_score
                                     as i32, side_to_move, book, moves_state_.disks_played)
            } else { alternative_score = -(12345678 as i32) }
            if (*book.node.offset(*book.book_hash_table.offset(slot as isize) as
                isize)).flags as i32 &
                1 as i32 != 0 {
                side_to_move = 0;
                sign = 1 as i32
            } else {
                side_to_move = 2;
                sign = -(1 as i32)
            }
            generate_all(side_to_move, moves_state_, search_state_, &board_state_.board);
            best_score = -(12345678 as i32);
            best_move = -(1 as i32);
            i = 0;
            while i < moves_state_.move_count[moves_state_.disks_played as usize] {
                this_move = moves_state_.move_list[moves_state_.disks_played as usize][i as usize];
                make_move(side_to_move, this_move, 1 as i32, moves_state_, board_state_, hash_state_, flip_stack);
                let val0___ = &mut val1;
                let val1___ = &mut val2;
                let orientation___ = &mut orientation;
                get_hash(val0___, val1___, orientation___, book, &board_state_.board);
                slot = probe_hash_table(val1, val2, book);
                let move_0 = this_move;
                {
                    unmake_move(side_to_move, move_0, &mut board_state_.board, moves_state_, hash_state_, flip_stack);
                };
                if slot == -(1 as i32) ||
                    *book.book_hash_table.offset(slot as isize) ==
                        -(1 as i32) {
                    if this_move == alternative_move {
                        score = alternative_score;
                        is_feasible = 1 as i32
                    } else { is_feasible = 0 as i32 }
                } else {
                    if original_side_to_move == 0 as i32 {
                        score =
                            (*book.node.offset(*book.book_hash_table.offset(slot as
                                isize)
                                as isize)).black_minimax_score
                                as i32
                    } else {
                        score =
                            (*book.node.offset(*book.book_hash_table.offset(slot as
                                isize)
                                as isize)).white_minimax_score
                                as i32
                    }
                    is_feasible = 1 as i32
                }
                if is_feasible != 0 {
                    score *= sign;
                    if score > best_score {
                        best_score = score;
                        best_move = this_move
                    }
                }
                i += 1
            }
            if best_move == -(1 as i32) {
                continuation = 0 as i32
            } else { temp_move[level as usize] = best_move }
        }
        if !(continuation != 0) { break ; }
    }
    board_state_.pv_depth[0] = level;
    i = 0;
    while i < level {
        board_state_.pv[0][i as usize] = temp_move[i as usize];
        i += 1
    }
    loop  {
        level -= 1;
        let side_to_move = temp_stm[level as usize];
        let move_0 = temp_move[level as usize];
        {
            unmake_move(side_to_move, move_0, &mut board_state_.board, moves_state_, hash_state_, flip_stack);
        };
        if !(level > 0 as i32) { break ; }
    }
    return book.candidate_list[chosen_index as usize].move_0;
}
