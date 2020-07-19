use crate::{
    src::{
        search::{get_ponder_move, create_eval_info, root_eval, disc_count, nodes},
        display::{send_status, echo},
        moves::{unmake_move, make_move, generate_specific, disks_played, move_list, move_count, generate_all, unmake_move_no_hash, make_move_no_hash},
        opname::opening_list,
        hash::{setup_hash, determine_hash_values, add_hash, clear_hash_drafts},
        game::{global_setup, game_init, CandidateMove},
        stubs::*,
        libc,
        myrandom::{my_random, my_srandom},
        error::fatal_error,
        globals::{board, piece_count, pv, pv_depth},
        midgame::{toggle_midgame_abort_check, toggle_midgame_hash_usage, tree_search},
        timer::{toggle_abort_check, clear_panic_abort, last_panic_check},
        eval::toggle_experimental,
        safemem::{safe_malloc, safe_realloc},
        autop::toggle_event_status,
        end::end_game,
        getcoeff::remove_coeffs,
        counter::reset_counter,
        patterns::{col_pattern, flip8, row_pattern, compute_line_patterns},
        zebra::{EvaluationType, _IO_FILE}
    }
};


pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
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
pub type size_t = libc::c_ulong;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type time_t = __time_t;
pub type __compar_fn_t
    =
    Option<unsafe fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;

/*
   File:           globals.h

   Created:        June 30, 1997

   Modified:       January 8, 2000

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Global state variables.
*/
/* The basic board type. One index for each position;
   a1=11, h1=18, a8=81, h8=88. */

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


pub type DrawMode = libc::c_uint;
pub const OPPONENT_WINS: DrawMode = 3;
pub const WHITE_WINS: DrawMode = 2;
pub const BLACK_WINS: DrawMode = 1;
pub const NEUTRAL: DrawMode = 0;
pub type GameMode = libc::c_uint;
pub const PUBLIC_GAME: GameMode = 1;
pub const PRIVATE_GAME: GameMode = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BookNode {
    pub hash_val1: libc::c_int,
    pub hash_val2: libc::c_int,
    pub black_minimax_score: libc::c_short,
    pub white_minimax_score: libc::c_short,
    pub best_alternative_move: libc::c_short,
    pub alternative_score: libc::c_short,
    pub flags: libc::c_ushort,
}
static mut deviation_bonus: libc::c_double = 0.;
static mut search_depth: libc::c_int = 0;
static mut node_table_size: libc::c_int = 0;
static mut hash_table_size: libc::c_int = 0;
static mut total_game_count: libc::c_int = 0;
static mut book_node_count: libc::c_int = 0;
static mut evaluated_count: libc::c_int = 0;
static mut evaluation_stage: libc::c_int = 0;
static mut max_eval_count: libc::c_int = 0;
static mut max_batch_size: libc::c_int = 0;
static mut exhausted_node_count: libc::c_int = 0;
static mut max_slack: libc::c_int = 0;
static mut low_deviation_threshold: libc::c_int = 0;
static mut high_deviation_threshold: libc::c_int = 0;
static mut min_eval_span: libc::c_int = 0;
static mut max_eval_span: libc::c_int = 0;
static mut min_negamax_span: libc::c_int = 0;
static mut max_negamax_span: libc::c_int = 0;
static mut leaf_count: libc::c_int = 0;
static mut bad_leaf_count: libc::c_int = 0;
static mut really_bad_leaf_count: libc::c_int = 0;
static mut unreachable_count: libc::c_int = 0;
static mut candidate_count: libc::c_int = 0;
static mut force_black: libc::c_int = 0;
static mut force_white: libc::c_int = 0;
static mut used_slack: [libc::c_int; 3] = [0; 3];
static mut b1_b1_map: [libc::c_int; 100] = [0; 100];
static mut g1_b1_map: [libc::c_int; 100] = [0; 100];
static mut g8_b1_map: [libc::c_int; 100] = [0; 100];
static mut b8_b1_map: [libc::c_int; 100] = [0; 100];
static mut a2_b1_map: [libc::c_int; 100] = [0; 100];
static mut a7_b1_map: [libc::c_int; 100] = [0; 100];
static mut h7_b1_map: [libc::c_int; 100] = [0; 100];
static mut h2_b1_map: [libc::c_int; 100] = [0; 100];
static mut exact_count: [libc::c_int; 61] = [0; 61];
static mut wld_count: [libc::c_int; 61] = [0; 61];
static mut exhausted_count: [libc::c_int; 61] = [0; 61];
static mut common_count: [libc::c_int; 61] = [0; 61];
static mut symmetry_map: [*mut libc::c_int; 8] =
    [0 as *const libc::c_int as *mut libc::c_int; 8];
static mut inv_symmetry_map: [*mut libc::c_int; 8] =
    [0 as *const libc::c_int as *mut libc::c_int; 8];
static mut line_hash: [[[libc::c_int; 6561]; 8]; 2] = [[[0; 6561]; 8]; 2];
static mut book_hash_table: *mut libc::c_int =
    0 as *const libc::c_int as *mut libc::c_int;
static mut draw_mode: DrawMode = OPPONENT_WINS;
static mut game_mode: GameMode = PRIVATE_GAME;
static mut node: *mut BookNode = 0 as *const BookNode as *mut BookNode;
static mut candidate_list: [CandidateMove; 60] =
    [CandidateMove{move_0: 0, score: 0, flags: 0, parent_flags: 0,}; 60];
/*
   INIT_MAPS
   Initializes the 8 symmetry maps.
   Notice that the order of these MUST coincide with the returned
   orientation value from get_hash() OR YOU WILL LOSE BIG.
*/
unsafe fn init_maps() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            pos = 10 as libc::c_int * i + j;
            b1_b1_map[pos as usize] = pos;
            g1_b1_map[pos as usize] =
                10 as libc::c_int * i + (9 as libc::c_int - j);
            g8_b1_map[pos as usize] =
                10 as libc::c_int * (9 as libc::c_int - i) +
                    (9 as libc::c_int - j);
            b8_b1_map[pos as usize] =
                10 as libc::c_int * (9 as libc::c_int - i) + j;
            a2_b1_map[pos as usize] = 10 as libc::c_int * j + i;
            a7_b1_map[pos as usize] =
                10 as libc::c_int * j + (9 as libc::c_int - i);
            h7_b1_map[pos as usize] =
                10 as libc::c_int * (9 as libc::c_int - j) +
                    (9 as libc::c_int - i);
            h2_b1_map[pos as usize] =
                10 as libc::c_int * (9 as libc::c_int - j) + i;
            j += 1
        }
        i += 1
    }
    symmetry_map[0 as libc::c_int as usize] = b1_b1_map.as_mut_ptr();
    inv_symmetry_map[0 as libc::c_int as usize] = b1_b1_map.as_mut_ptr();
    symmetry_map[1 as libc::c_int as usize] = g1_b1_map.as_mut_ptr();
    inv_symmetry_map[1 as libc::c_int as usize] = g1_b1_map.as_mut_ptr();
    symmetry_map[2 as libc::c_int as usize] = g8_b1_map.as_mut_ptr();
    inv_symmetry_map[2 as libc::c_int as usize] = g8_b1_map.as_mut_ptr();
    symmetry_map[3 as libc::c_int as usize] = b8_b1_map.as_mut_ptr();
    inv_symmetry_map[3 as libc::c_int as usize] = b8_b1_map.as_mut_ptr();
    symmetry_map[4 as libc::c_int as usize] = a2_b1_map.as_mut_ptr();
    inv_symmetry_map[4 as libc::c_int as usize] = a2_b1_map.as_mut_ptr();
    symmetry_map[5 as libc::c_int as usize] = a7_b1_map.as_mut_ptr();
    inv_symmetry_map[5 as libc::c_int as usize] = h2_b1_map.as_mut_ptr();
    symmetry_map[6 as libc::c_int as usize] = h7_b1_map.as_mut_ptr();
    inv_symmetry_map[6 as libc::c_int as usize] = h7_b1_map.as_mut_ptr();
    symmetry_map[7 as libc::c_int as usize] = h2_b1_map.as_mut_ptr();
    inv_symmetry_map[7 as libc::c_int as usize] = a7_b1_map.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        *symmetry_map[i as usize].offset(0 as libc::c_int as isize) =
            0 as libc::c_int;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            k = 1 as libc::c_int;
            while k <= 8 as libc::c_int {
                pos = 10 as libc::c_int * j + k;
                if *inv_symmetry_map[i as
                                         usize].offset(*symmetry_map[i as
                                                                         usize].offset(pos
                                                                                           as
                                                                                           isize)
                                                           as isize) != pos {
                    fatal_error(b"Error in map %d: inv(map(%d))=%d\n\x00" as
                                    *const u8 as *const libc::c_char, i, pos,
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
   SELECT_HASH_SLOT
   Finds a slot in the hash table for the node INDEX
   using linear probing.
*/
unsafe fn select_hash_slot(mut index: libc::c_int) {
    let mut slot: libc::c_int = 0;
    slot = (*node.offset(index as isize)).hash_val1 % hash_table_size;
    while *book_hash_table.offset(slot as isize) != -(1 as libc::c_int) {
        slot = (slot + 1 as libc::c_int) % hash_table_size
    }
    *book_hash_table.offset(slot as isize) = index;
}
/*
   PROBE_HASH_TABLE
   Search for a certain hash code in the hash table.
*/
unsafe fn probe_hash_table(mut val1: libc::c_int,
                                      mut val2: libc::c_int) -> libc::c_int {
    let mut slot: libc::c_int = 0;
    if hash_table_size == 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        slot = val1 % hash_table_size;
        while *book_hash_table.offset(slot as isize) != -(1 as libc::c_int) &&
                  ((*node.offset(*book_hash_table.offset(slot as isize) as
                                     isize)).hash_val2 != val2 ||
                       (*node.offset(*book_hash_table.offset(slot as isize) as
                                         isize)).hash_val1 != val1) {
            slot = (slot + 1 as libc::c_int) % hash_table_size
        }
        return slot
    };
}
/*
   CREATE_HASH_REFERENCEE
   Takes the node list and fills the hash table with indices
   into the node list.
*/
unsafe fn create_hash_reference() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < hash_table_size {
        *book_hash_table.offset(i as isize) = -(1 as libc::c_int);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < book_node_count { select_hash_slot(i); i += 1 };
}
/*
   REBUILD_HASH_TABLE
   Resize the hash table for a requested number of nodes.
*/
unsafe fn rebuild_hash_table(mut requested_items: libc::c_int) {
    let mut new_size: libc::c_int = 0;
    let mut new_memory: libc::c_int = 0;
    new_size = 2 as libc::c_int * requested_items;
    new_memory =
        (new_size as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                             as libc::c_ulong) as libc::c_int;
    if hash_table_size == 0 as libc::c_int {
        book_hash_table =
            safe_malloc(new_memory as size_t) as *mut libc::c_int
    } else {
        book_hash_table =
            safe_realloc(book_hash_table as *mut libc::c_void,
                         new_memory as size_t) as *mut libc::c_int
    }
    if book_hash_table.is_null() {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const libc::c_char,
                    b"Book hash table: Failed to allocate\x00" as *const u8 as
                        *const libc::c_char, new_memory, new_size);
    }
    hash_table_size = new_size;
    create_hash_reference();
}
/*
   PREPARE_HASH
   Compute the position hash codes.
*/
unsafe fn prepare_hash() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    /* The hash keys are static, hence the same keys must be
       produced every time the program is run. */
    my_srandom(0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            k = 0 as libc::c_int;
            while k < 6561 as libc::c_int {
                line_hash[i as usize][j as usize][k as usize] =
                    if my_random() % 2 as libc::c_int as libc::c_long != 0 {
                        my_random()
                    } else { -my_random() } as libc::c_int;
                k += 1
            }
            j += 1
        }
        i += 1
    }
    hash_table_size = 0 as libc::c_int;
}
/*
   GET_HASH
   Return the hash values for the current board position.
   The position is rotated so that the 64-bit hash value
   is minimized among all rotations. This ensures detection
   of all transpositions.
   See also init_maps().
*/
/*
   GET_HASH
   Return the hash values for the current board position.
   The position is rotated so that the 64-bit hash value
   is minimized among all rotations. This ensures detection
   of all transpositions.
   See also init_maps().
*/

pub unsafe fn get_hash(mut val0: *mut libc::c_int,
                                  mut val1: *mut libc::c_int,
                                  mut orientation: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut min_map: libc::c_int = 0;
    let mut min_hash0: libc::c_int = 0;
    let mut min_hash1: libc::c_int = 0;
    let mut out: [[libc::c_int; 2]; 8] = [[0; 2]; 8];
    /* Calculate the 8 different 64-bit hash values for the
       different rotations. */
    compute_line_patterns(board.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            out[i as usize][j as usize] = 0 as libc::c_int;
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        /* b1 -> b1 */
        out[0 as libc::c_int as usize][0 as libc::c_int as usize] ^=
            line_hash[0 as libc::c_int as
                          usize][i as
                                     usize][row_pattern[i as usize] as usize];
        out[0 as libc::c_int as usize][1 as libc::c_int as usize] ^=
            line_hash[1 as libc::c_int as
                          usize][i as
                                     usize][row_pattern[i as usize] as usize];
        /* g1 -> b1 */
        out[1 as libc::c_int as usize][0 as libc::c_int as usize] ^=
            line_hash[0 as libc::c_int as
                          usize][i as
                                     usize][flip8[row_pattern[i as usize] as
                                                      usize] as usize];
        out[1 as libc::c_int as usize][1 as libc::c_int as usize] ^=
            line_hash[1 as libc::c_int as
                          usize][i as
                                     usize][flip8[row_pattern[i as usize] as
                                                      usize] as usize];
        /* g8 -> b1 */
        out[2 as libc::c_int as usize][0 as libc::c_int as usize] ^=
            line_hash[0 as libc::c_int as
                          usize][i as
                                     usize][flip8[row_pattern[(7 as
                                                                   libc::c_int
                                                                   - i) as
                                                                  usize] as
                                                      usize] as usize];
        out[2 as libc::c_int as usize][1 as libc::c_int as usize] ^=
            line_hash[1 as libc::c_int as
                          usize][i as
                                     usize][flip8[row_pattern[(7 as
                                                                   libc::c_int
                                                                   - i) as
                                                                  usize] as
                                                      usize] as usize];
        /* b8 -> b1 */
        out[3 as libc::c_int as usize][0 as libc::c_int as usize] ^=
            line_hash[0 as libc::c_int as
                          usize][i as
                                     usize][row_pattern[(7 as libc::c_int - i)
                                                            as usize] as
                                                usize];
        out[3 as libc::c_int as usize][1 as libc::c_int as usize] ^=
            line_hash[1 as libc::c_int as
                          usize][i as
                                     usize][row_pattern[(7 as libc::c_int - i)
                                                            as usize] as
                                                usize];
        /* a2 -> b1 */
        out[4 as libc::c_int as usize][0 as libc::c_int as usize] ^=
            line_hash[0 as libc::c_int as
                          usize][i as
                                     usize][col_pattern[i as usize] as usize];
        out[4 as libc::c_int as usize][1 as libc::c_int as usize] ^=
            line_hash[1 as libc::c_int as
                          usize][i as
                                     usize][col_pattern[i as usize] as usize];
        /* a7 -> b1 */
        out[5 as libc::c_int as usize][0 as libc::c_int as usize] ^=
            line_hash[0 as libc::c_int as
                          usize][i as
                                     usize][flip8[col_pattern[i as usize] as
                                                      usize] as usize];
        out[5 as libc::c_int as usize][1 as libc::c_int as usize] ^=
            line_hash[1 as libc::c_int as
                          usize][i as
                                     usize][flip8[col_pattern[i as usize] as
                                                      usize] as usize];
        /* h7 -> b1 */
        out[6 as libc::c_int as usize][0 as libc::c_int as usize] ^=
            line_hash[0 as libc::c_int as
                          usize][i as
                                     usize][flip8[col_pattern[(7 as
                                                                   libc::c_int
                                                                   - i) as
                                                                  usize] as
                                                      usize] as usize];
        out[6 as libc::c_int as usize][1 as libc::c_int as usize] ^=
            line_hash[1 as libc::c_int as
                          usize][i as
                                     usize][flip8[col_pattern[(7 as
                                                                   libc::c_int
                                                                   - i) as
                                                                  usize] as
                                                      usize] as usize];
        /* h2 -> b1 */
        out[7 as libc::c_int as usize][0 as libc::c_int as usize] ^=
            line_hash[0 as libc::c_int as
                          usize][i as
                                     usize][col_pattern[(7 as libc::c_int - i)
                                                            as usize] as
                                                usize];
        out[7 as libc::c_int as usize][1 as libc::c_int as usize] ^=
            line_hash[1 as libc::c_int as
                          usize][i as
                                     usize][col_pattern[(7 as libc::c_int - i)
                                                            as usize] as
                                                usize];
        i += 1
    }
    /* Find the rotation minimizing the hash index.
       If two hash indices are equal, map number is implicitly used
       as tie-breaker. */
    min_map = 0 as libc::c_int;
    min_hash0 = out[0 as libc::c_int as usize][0 as libc::c_int as usize];
    min_hash1 = out[0 as libc::c_int as usize][1 as libc::c_int as usize];
    i = 1 as libc::c_int;
    while i < 8 as libc::c_int {
        if out[i as usize][0 as libc::c_int as usize] < min_hash0 ||
               out[i as usize][0 as libc::c_int as usize] == min_hash0 &&
                   out[i as usize][1 as libc::c_int as usize] < min_hash1 {
            min_map = i;
            min_hash0 = out[i as usize][0 as libc::c_int as usize];
            min_hash1 = out[i as usize][1 as libc::c_int as usize]
        }
        i += 1
    }
    *val0 = abs(min_hash0);
    *val1 = abs(min_hash1);
    *orientation = min_map;
}
/*
   SET_ALLOCATION
   Changes the number of nodes for which memory is allocated.
*/
unsafe fn set_allocation(mut size: libc::c_int) {
    if node.is_null() {
        node =
            safe_malloc((size as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<BookNode>()
                                                             as
                                                             libc::c_ulong))
                as *mut BookNode
    } else {
        node =
            safe_realloc(node as *mut libc::c_void,
                         (size as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<BookNode>()
                                                              as
                                                              libc::c_ulong))
                as *mut BookNode
    }
    if node.is_null() {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const libc::c_char,
                    b"Book node list: Failed to allocate\x00" as *const u8 as
                        *const libc::c_char,
                    (size as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<BookNode>()
                                                         as libc::c_ulong),
                    size);
    }
    node_table_size = size;
    if node_table_size as libc::c_double >
           0.80f64 * hash_table_size as libc::c_double {
        rebuild_hash_table(node_table_size);
    };
}
/*
   INCREASE_ALLOCATION
   Allocate more memory for the book tree.
*/
unsafe fn increase_allocation() {
    set_allocation(node_table_size + 50000 as libc::c_int);
}
/*
   CREATE_BOOK_NODE
   Creates a new book node without any connections whatsoever
   to the rest of the tree.
*/
unsafe fn create_BookNode(mut val1: libc::c_int,
                                     mut val2: libc::c_int,
                                     mut flags: libc::c_ushort)
 -> libc::c_int {
    let mut index: libc::c_int = 0;
    if book_node_count == node_table_size { increase_allocation(); }
    index = book_node_count;
    (*node.offset(index as isize)).hash_val1 = val1;
    (*node.offset(index as isize)).hash_val2 = val2;
    (*node.offset(index as isize)).black_minimax_score =
        9999 as libc::c_int as libc::c_short;
    (*node.offset(index as isize)).white_minimax_score =
        9999 as libc::c_int as libc::c_short;
    (*node.offset(index as isize)).best_alternative_move =
        -(1 as libc::c_int) as libc::c_short;
    (*node.offset(index as isize)).alternative_score =
        9999 as libc::c_int as libc::c_short;
    (*node.offset(index as isize)).flags = flags;
    select_hash_slot(index);
    book_node_count += 1;
    return index;
}
/*
   INIT_BOOK_TREE
   Initializes the node tree by creating the root of the tree.
*/
unsafe fn init_book_tree() {
    book_node_count = 0 as libc::c_int;
    node = 0 as *mut BookNode;
}
/*
   PREPATE_TREE_TRAVERSAL
   Prepares all relevant data structures for a tree search
   or traversal.
*/
unsafe fn prepare_tree_traversal() {
    let mut side_to_move: libc::c_int = 0;
    toggle_experimental(0 as libc::c_int);
    game_init(0 as *const libc::c_char, &mut side_to_move);
    toggle_midgame_hash_usage(1 as libc::c_int, 1 as libc::c_int);
    toggle_abort_check(0 as libc::c_int);
    toggle_midgame_abort_check(0 as libc::c_int);
}
/*
   CLEAR_NODE_DEPTH
   Changes the flags of a node so that the search depth
   bits are cleared.
*/
unsafe fn clear_node_depth(mut index: libc::c_int) {
    let mut depth: libc::c_int = 0;
    depth =
        (*node.offset(index as isize)).flags as libc::c_int >>
            10 as libc::c_int;
    let ref mut fresh0 = (*node.offset(index as isize)).flags;
    *fresh0 =
        (*fresh0 as libc::c_int ^ depth << 10 as libc::c_int) as
            libc::c_ushort;
}
/*
   GET_NODE_DEPTH
*/
unsafe fn get_node_depth(mut index: libc::c_int) -> libc::c_int {
    return (*node.offset(index as isize)).flags as libc::c_int >>
               10 as libc::c_int;
}
/*
   SET_NODE_DEPTH
   Marks a node as being searched to a certain depth.
*/
unsafe fn set_node_depth(mut index: libc::c_int,
                                    mut depth: libc::c_int) {
    let ref mut fresh1 = (*node.offset(index as isize)).flags;
    *fresh1 =
        (*fresh1 as libc::c_int | depth << 10 as libc::c_int) as
            libc::c_ushort;
}
/*
   ADJUST_SCORE
   Tweak a score as to encourage early deviations.
*/
unsafe fn adjust_score(mut score: libc::c_int,
                                  mut side_to_move: libc::c_int)
 -> libc::c_int {
    let mut adjustment: libc::c_int = 0;
    let mut adjust_steps: libc::c_int = 0;
    adjust_steps = high_deviation_threshold - disks_played;
    if adjust_steps < 0 as libc::c_int {
        adjustment = 0 as libc::c_int
    } else {
        if disks_played < low_deviation_threshold {
            adjust_steps = high_deviation_threshold - low_deviation_threshold
        }
        adjustment =
            floor(adjust_steps as libc::c_double * deviation_bonus * 128.0f64)
                as libc::c_int;
        if side_to_move == 2 as libc::c_int { adjustment = -adjustment }
    }
    return score + adjustment;
}
/*
   DO_MINIMAX
   Calculates the minimax value of node INDEX.
*/
unsafe fn do_minimax(mut index: libc::c_int,
                                mut black_score: *mut libc::c_int,
                                mut white_score: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut child: libc::c_int = 0;
    let mut child_black_score: libc::c_int = 0;
    let mut child_white_score: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut this_move: libc::c_int = 0;
    let mut alternative_move: libc::c_int = 0;
    let mut alternative_move_found: libc::c_int = 0;
    let mut child_count: libc::c_int = 0;
    let mut best_black_child_val: libc::c_int = 0;
    let mut best_white_child_val: libc::c_int = 0;
    let mut worst_black_child_val: libc::c_int = 0;
    let mut worst_white_child_val: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    let mut best_black_score: libc::c_short = 0;
    let mut best_white_score: libc::c_short = 0;
    /* If the node has been visited AND it is a midgame node, meaning
       that the minimax values are not to be tweaked, return the
       stored values. */
    if (*node.offset(index as isize)).flags as libc::c_int & 8 as libc::c_int
           == 0 {
        if (*node.offset(index as isize)).flags as libc::c_int &
               (4 as libc::c_int | 16 as libc::c_int) == 0 {
            *black_score =
                (*node.offset(index as isize)).black_minimax_score as
                    libc::c_int;
            *white_score =
                (*node.offset(index as isize)).white_minimax_score as
                    libc::c_int;
            return
        }
    }
    /* Correct WLD solved nodes corresponding to draws to be represented
       as full solved and make sure full solved nodes are marked as
       WLD solved as well */
    if (*node.offset(index as isize)).flags as libc::c_int & 4 as libc::c_int
           != 0 &&
           (*node.offset(index as isize)).black_minimax_score as libc::c_int
               == 0 as libc::c_int &&
           (*node.offset(index as isize)).white_minimax_score as libc::c_int
               == 0 as libc::c_int {
        let ref mut fresh2 = (*node.offset(index as isize)).flags;
        *fresh2 =
            (*fresh2 as libc::c_int | 16 as libc::c_int) as libc::c_ushort
    }
    if (*node.offset(index as isize)).flags as libc::c_int & 16 as libc::c_int
           != 0 &&
           (*node.offset(index as isize)).flags as libc::c_int &
               4 as libc::c_int == 0 {
        let ref mut fresh3 = (*node.offset(index as isize)).flags;
        *fresh3 =
            (*fresh3 as libc::c_int | 4 as libc::c_int) as libc::c_ushort
    }
    /* Recursively minimax all children of the node */
    if (*node.offset(index as isize)).flags as libc::c_int & 1 as libc::c_int
           != 0 {
        side_to_move = 0 as libc::c_int
    } else { side_to_move = 2 as libc::c_int }
    best_black_child_val = -(99999 as libc::c_int);
    best_white_child_val = -(99999 as libc::c_int);
    worst_black_child_val = 99999 as libc::c_int;
    worst_white_child_val = 99999 as libc::c_int;
    if (*node.offset(index as isize)).alternative_score as libc::c_int !=
           9999 as libc::c_int {
        best_black_score =
            adjust_score((*node.offset(index as isize)).alternative_score as
                             libc::c_int, side_to_move) as libc::c_short;
        best_white_score = best_black_score;
        worst_black_child_val = best_black_score as libc::c_int;
        best_black_child_val = worst_black_child_val;
        worst_white_child_val = best_white_score as libc::c_int;
        best_white_child_val = worst_white_child_val;
        alternative_move_found = 0 as libc::c_int;
        alternative_move =
            (*node.offset(index as isize)).best_alternative_move as
                libc::c_int;
        if alternative_move > 0 as libc::c_int {
            get_hash(&mut val1, &mut val2, &mut orientation);
            alternative_move =
                *inv_symmetry_map[orientation as
                                      usize].offset(alternative_move as isize)
        }
    } else {
        alternative_move_found = 1 as libc::c_int;
        alternative_move = 0 as libc::c_int;
        if side_to_move == 0 as libc::c_int {
            best_black_score = -(32000 as libc::c_int) as libc::c_short;
            best_white_score = -(32000 as libc::c_int) as libc::c_short
        } else {
            best_black_score = 32000 as libc::c_int as libc::c_short;
            best_white_score = 32000 as libc::c_int as libc::c_short
        }
    }
    generate_all(side_to_move);
    child_count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < move_count[disks_played as usize] {
        piece_count[0 as libc::c_int as usize][disks_played as usize] =
            disc_count(0 as libc::c_int);
        piece_count[2 as libc::c_int as usize][disks_played as usize] =
            disc_count(2 as libc::c_int);
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as libc::c_int);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as libc::c_int) {
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
            if side_to_move == 0 as libc::c_int {
                best_black_score =
                    if child_black_score > best_black_score as libc::c_int {
                        child_black_score
                    } else { best_black_score as libc::c_int } as
                        libc::c_short;
                best_white_score =
                    if child_white_score > best_white_score as libc::c_int {
                        child_white_score
                    } else { best_white_score as libc::c_int } as
                        libc::c_short
            } else {
                best_black_score =
                    if child_black_score < best_black_score as libc::c_int {
                        child_black_score
                    } else { best_black_score as libc::c_int } as
                        libc::c_short;
                best_white_score =
                    if child_white_score < best_white_score as libc::c_int {
                        child_white_score
                    } else { best_white_score as libc::c_int } as
                        libc::c_short
            }
            child_count += 1
        } else if alternative_move_found == 0 && this_move == alternative_move
         {
            alternative_move_found = 1 as libc::c_int
        }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    if alternative_move_found == 0 {
        /* The was-to-be deviation now leads to a position in the database,
           hence it can no longer be used. */
        (*node.offset(index as isize)).alternative_score =
            9999 as libc::c_int as libc::c_short;
        (*node.offset(index as isize)).best_alternative_move =
            -(1 as libc::c_int) as libc::c_short
    }
    /* Try to infer the WLD status from the children */
    if (*node.offset(index as isize)).flags as libc::c_int &
           (16 as libc::c_int | 4 as libc::c_int) == 0 &&
           child_count > 0 as libc::c_int {
        if side_to_move == 0 as libc::c_int {
            if best_black_child_val >= 30000 as libc::c_int &&
                   best_white_child_val >= 30000 as libc::c_int {
                /* Black win */
                let ref mut fresh4 =
                    (*node.offset(index as isize)).white_minimax_score;
                *fresh4 =
                    if best_black_child_val < best_white_child_val {
                        best_black_child_val
                    } else { best_white_child_val } as libc::c_short;
                (*node.offset(index as isize)).black_minimax_score = *fresh4;
                let ref mut fresh5 = (*node.offset(index as isize)).flags;
                *fresh5 =
                    (*fresh5 as libc::c_int | 4 as libc::c_int) as
                        libc::c_ushort
            } else if best_black_child_val <= -(30000 as libc::c_int) &&
                          best_white_child_val <= -(30000 as libc::c_int) {
                /* Black loss */
                let ref mut fresh6 =
                    (*node.offset(index as isize)).white_minimax_score;
                *fresh6 =
                    if best_black_child_val > best_white_child_val {
                        best_black_child_val
                    } else { best_white_child_val } as libc::c_short;
                (*node.offset(index as isize)).black_minimax_score = *fresh6;
                let ref mut fresh7 = (*node.offset(index as isize)).flags;
                *fresh7 =
                    (*fresh7 as libc::c_int | 4 as libc::c_int) as
                        libc::c_ushort
            }
        } else if worst_black_child_val <= -(30000 as libc::c_int) &&
                      worst_white_child_val <= -(30000 as libc::c_int) {
            /* White win */
            let ref mut fresh8 =
                (*node.offset(index as isize)).white_minimax_score;
            *fresh8 =
                if worst_black_child_val > worst_white_child_val {
                    worst_black_child_val
                } else { worst_white_child_val } as libc::c_short;
            (*node.offset(index as isize)).black_minimax_score = *fresh8;
            let ref mut fresh9 = (*node.offset(index as isize)).flags;
            *fresh9 =
                (*fresh9 as libc::c_int | 4 as libc::c_int) as libc::c_ushort
        } else if worst_black_child_val >= 30000 as libc::c_int &&
                      worst_white_child_val >= 30000 as libc::c_int {
            /* White loss */
            let ref mut fresh10 =
                (*node.offset(index as isize)).white_minimax_score;
            *fresh10 =
                if worst_black_child_val < worst_white_child_val {
                    worst_black_child_val
                } else { worst_white_child_val } as libc::c_short;
            (*node.offset(index as isize)).black_minimax_score = *fresh10;
            let ref mut fresh11 = (*node.offset(index as isize)).flags;
            *fresh11 =
                (*fresh11 as libc::c_int | 4 as libc::c_int) as libc::c_ushort
        }
    }
    /* Tweak the minimax scores for draws to give the right
       draw avoidance behavior */
    if (*node.offset(index as isize)).flags as libc::c_int &
           (16 as libc::c_int | 4 as libc::c_int) != 0 {
        *black_score =
            (*node.offset(index as isize)).black_minimax_score as libc::c_int;
        *white_score =
            (*node.offset(index as isize)).white_minimax_score as libc::c_int;
        if (*node.offset(index as isize)).black_minimax_score as libc::c_int
               == 0 as libc::c_int &&
               (*node.offset(index as isize)).white_minimax_score as
                   libc::c_int == 0 as libc::c_int {
            /* Is it a position in which a draw should be avoided? */
            if game_mode as libc::c_uint ==
                   PRIVATE_GAME as libc::c_int as libc::c_uint ||
                   (*node.offset(index as isize)).flags as libc::c_int &
                       32 as libc::c_int == 0 {
                match draw_mode as libc::c_uint {
                    1 => {
                        *black_score =
                            30000 as libc::c_int - 1 as libc::c_int;
                        *white_score = 30000 as libc::c_int - 1 as libc::c_int
                    }
                    2 => {
                        *black_score =
                            -(30000 as libc::c_int - 1 as libc::c_int);
                        *white_score =
                            -(30000 as libc::c_int - 1 as libc::c_int)
                    }
                    3 => {
                        *black_score =
                            -(30000 as libc::c_int - 1 as libc::c_int);
                        *white_score = 30000 as libc::c_int - 1 as libc::c_int
                    }
                    0 | _ => { }
                }
            }
        }
    } else {
        let ref mut fresh12 =
            (*node.offset(index as isize)).black_minimax_score;
        *fresh12 = best_black_score;
        *black_score = *fresh12 as libc::c_int;
        let ref mut fresh13 =
            (*node.offset(index as isize)).white_minimax_score;
        *fresh13 = best_white_score;
        *white_score = *fresh13 as libc::c_int
    }
    let ref mut fresh14 = (*node.offset(index as isize)).flags;
    *fresh14 = (*fresh14 as libc::c_int ^ 8 as libc::c_int) as libc::c_ushort;
}
/*
   MINIMAX_TREE
   Calculates the minimax values of all nodes in the tree.
*/

pub unsafe fn minimax_tree() {
    let mut i: libc::c_int = 0;
    let mut dummy_black_score: libc::c_int = 0;
    let mut dummy_white_score: libc::c_int = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    printf(b"Calculating minimax value... \x00" as *const u8 as
               *const libc::c_char);
    fflush(stdout);
    prepare_tree_traversal();
    time(&mut start_time);
    /* Mark all nodes as not traversed */
    i = 0 as libc::c_int;
    while i < book_node_count {
        let ref mut fresh15 = (*node.offset(i as isize)).flags;
        *fresh15 =
            (*fresh15 as libc::c_int | 8 as libc::c_int) as libc::c_ushort;
        i += 1
    }
    do_minimax(0 as libc::c_int, &mut dummy_black_score,
               &mut dummy_white_score);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const libc::c_char,
           (stop_time - start_time) as libc::c_int);
    puts(b"\x00" as *const u8 as *const libc::c_char);
}
/*
  NEGA_SCOUT
  This wrapper on top of TREE_SEARCH is used by EVALUATE_NODE
  to search the possible deviations.
*/
unsafe fn nega_scout(mut depth: libc::c_int,
                                mut allow_mpc: libc::c_int,
                                mut side_to_move: libc::c_int,
                                mut allowed_count: libc::c_int,
                                mut allowed_moves: *mut libc::c_int,
                                mut alpha: libc::c_int, mut beta: libc::c_int,
                                mut best_score: *mut libc::c_int,
                                mut best_index: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut curr_alpha: libc::c_int = 0;
    let mut curr_depth: libc::c_int = 0;
    let mut low_score: libc::c_int = 0;
    let mut high_score: libc::c_int = 0;
    let mut best_move: libc::c_int = 0;
    let mut current_score: libc::c_int = 0;
    reset_counter(&mut nodes);
    low_score = -(12345678 as libc::c_int);
    /* To avoid spurious hash table entries to take out the effect
       of the averaging done, the hash table drafts are changed prior
       to each node being searched. */
    clear_hash_drafts();
    determine_hash_values(side_to_move, board.as_mut_ptr());
    /* First determine the best move in the current position
       and its score when searched to depth DEPTH.
       This is done using standard negascout with iterative deepening. */
    curr_depth = 2 as libc::c_int - depth % 2 as libc::c_int;
    while curr_depth <= depth {
        low_score = -(12345678 as libc::c_int);
        curr_alpha = -(12345678 as libc::c_int);
        i = 0 as libc::c_int;
        while i < allowed_count {
            make_move(side_to_move, *allowed_moves.offset(i as isize),
                      1 as libc::c_int);
            piece_count[0 as libc::c_int as usize][disks_played as usize] =
                disc_count(0 as libc::c_int);
            piece_count[2 as libc::c_int as usize][disks_played as usize] =
                disc_count(2 as libc::c_int);
            last_panic_check = 0.0f64;
            if i == 0 as libc::c_int {
                current_score =
                    -tree_search(1 as libc::c_int, curr_depth,
                                 0 as libc::c_int + 2 as libc::c_int -
                                     side_to_move, -(12345678 as libc::c_int),
                                 12345678 as libc::c_int, 1 as libc::c_int,
                                 allow_mpc, 1 as libc::c_int);
                low_score = current_score;
                *best_index = i
            } else {
                curr_alpha =
                    if low_score > curr_alpha {
                        low_score
                    } else { curr_alpha };
                current_score =
                    -tree_search(1 as libc::c_int, curr_depth,
                                 0 as libc::c_int + 2 as libc::c_int -
                                     side_to_move,
                                 -(curr_alpha + 1 as libc::c_int),
                                 -curr_alpha, 1 as libc::c_int, allow_mpc,
                                 1 as libc::c_int);
                if current_score > curr_alpha {
                    current_score =
                        -tree_search(1 as libc::c_int, curr_depth,
                                     0 as libc::c_int + 2 as libc::c_int -
                                         side_to_move,
                                     -(12345678 as libc::c_int),
                                     12345678 as libc::c_int,
                                     1 as libc::c_int, allow_mpc,
                                     1 as libc::c_int);
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
        while j >= 1 as libc::c_int {
            *allowed_moves.offset(j as isize) =
                *allowed_moves.offset((j - 1 as libc::c_int) as isize);
            j -= 1
        }
        *allowed_moves.offset(0 as libc::c_int as isize) = best_move;
        *best_index = 0 as libc::c_int;
        curr_depth += 2 as libc::c_int
    }
    /* Then find the score for the best move when searched
       to depth DEPTH+1 */
    make_move(side_to_move, *allowed_moves.offset(*best_index as isize),
              1 as libc::c_int);
    piece_count[0 as libc::c_int as usize][disks_played as usize] =
        disc_count(0 as libc::c_int);
    piece_count[2 as libc::c_int as usize][disks_played as usize] =
        disc_count(2 as libc::c_int);
    last_panic_check = 0.0f64;
    high_score =
        -tree_search(1 as libc::c_int, depth + 1 as libc::c_int,
                     0 as libc::c_int + 2 as libc::c_int - side_to_move,
                     -(12345678 as libc::c_int), 12345678 as libc::c_int,
                     1 as libc::c_int, allow_mpc, 1 as libc::c_int);
    unmake_move(side_to_move, *allowed_moves.offset(*best_index as isize));
    /* To remove the oscillations between odd and even search depths
       the score for the deviation is the average between the two scores. */
    *best_score = (low_score + high_score) / 2 as libc::c_int;
}
/*
   EVALUATE_NODE
   Applies a search to a predetermined depth to find the best
   alternative move in a position.
   Note: This function assumes that generate_all() has been
         called prior to it being called.
*/
unsafe fn evaluate_node(mut index: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut alternative_move_count: libc::c_int = 0;
    let mut this_move: libc::c_int = 0;
    let mut best_move: libc::c_int = 0;
    let mut child: libc::c_int = 0;
    let mut allow_mpc: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut best_index: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    let mut feasible_move: [libc::c_int; 64] = [0; 64];
    let mut best_score: libc::c_int = 0;
    /* Don't evaluate nodes that already have been searched deep enough */
    depth = get_node_depth(index);
    if depth >= search_depth &&
           (*node.offset(index as isize)).alternative_score as libc::c_int !=
               9999 as libc::c_int {
        return
    }
    /* If the node has been evaluated and its score is outside the
       eval and minimax windows, bail out. */
    if (*node.offset(index as isize)).alternative_score as libc::c_int !=
           9999 as libc::c_int {
        if abs((*node.offset(index as isize)).alternative_score as
                   libc::c_int) < min_eval_span ||
               abs((*node.offset(index as isize)).alternative_score as
                       libc::c_int) > max_eval_span {
            return
        }
        if abs((*node.offset(index as isize)).black_minimax_score as
                   libc::c_int) < min_negamax_span ||
               abs((*node.offset(index as isize)).black_minimax_score as
                       libc::c_int) > max_negamax_span {
            return
        }
    }
    if (*node.offset(index as isize)).flags as libc::c_int & 1 as libc::c_int
           != 0 {
        side_to_move = 0 as libc::c_int
    } else { side_to_move = 2 as libc::c_int }
    remove_coeffs(disks_played - 8 as libc::c_int);
    clear_panic_abort();
    piece_count[0 as libc::c_int as usize][disks_played as usize] =
        disc_count(0 as libc::c_int);
    piece_count[2 as libc::c_int as usize][disks_played as usize] =
        disc_count(2 as libc::c_int);
    /* Find the moves which haven't been tried from this position */
    alternative_move_count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < move_count[disks_played as usize] {
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as libc::c_int);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child == -(1 as libc::c_int) {
            let fresh16 = alternative_move_count;
            alternative_move_count = alternative_move_count + 1;
            feasible_move[fresh16 as usize] = this_move
        }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    if alternative_move_count == 0 as libc::c_int {
        /* There weren't any such moves */
        exhausted_node_count += 1;
        (*node.offset(index as isize)).best_alternative_move =
            -(2 as libc::c_int) as libc::c_short;
        (*node.offset(index as isize)).alternative_score =
            9999 as libc::c_int as libc::c_short
    } else {
        /* Find the best of those moves */
        allow_mpc = (search_depth >= 9 as libc::c_int) as libc::c_int;
        nega_scout(search_depth, allow_mpc, side_to_move,
                   alternative_move_count, feasible_move.as_mut_ptr(),
                   -(12345678 as libc::c_int), 12345678 as libc::c_int,
                   &mut best_score, &mut best_index);
        best_move = feasible_move[best_index as usize];
        evaluated_count += 1;
        if side_to_move == 0 as libc::c_int {
            (*node.offset(index as isize)).alternative_score =
                best_score as libc::c_short
        } else {
            (*node.offset(index as isize)).alternative_score =
                -best_score as libc::c_short
        }
        get_hash(&mut val1, &mut val2, &mut orientation);
        (*node.offset(index as isize)).best_alternative_move =
            *symmetry_map[orientation as usize].offset(best_move as isize) as
                libc::c_short
    }
    clear_node_depth(index);
    set_node_depth(index, search_depth);
}
/*
   DO_EVALUATE
   Recursively makes sure a subtree is evaluated to
   the specified depth.
*/
unsafe fn do_evaluate(mut index: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut child: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut this_move: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    if evaluated_count >= max_eval_count { return }
    if (*node.offset(index as isize)).flags as libc::c_int & 8 as libc::c_int
           == 0 {
        return
    }
    if (*node.offset(index as isize)).flags as libc::c_int & 1 as libc::c_int
           != 0 {
        side_to_move = 0 as libc::c_int
    } else { side_to_move = 2 as libc::c_int }
    generate_all(side_to_move);
    if (*node.offset(index as isize)).flags as libc::c_int &
           (16 as libc::c_int | 4 as libc::c_int) == 0 {
        evaluate_node(index);
    }
    if evaluated_count >=
           (evaluation_stage + 1 as libc::c_int) * max_eval_count /
               25 as libc::c_int {
        evaluation_stage += 1;
        putc('|' as i32, stdout);
        if evaluation_stage % 5 as libc::c_int == 0 as libc::c_int {
            printf(b" %d%% \x00" as *const u8 as *const libc::c_char,
                   4 as libc::c_int * evaluation_stage);
        }
        fflush(stdout);
    }
    i = 0 as libc::c_int;
    while i < move_count[disks_played as usize] {
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as libc::c_int);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as libc::c_int) { do_evaluate(child); }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    let ref mut fresh17 = (*node.offset(index as isize)).flags;
    *fresh17 = (*fresh17 as libc::c_int ^ 8 as libc::c_int) as libc::c_ushort;
}
/*
   EVALUATE_TREE
   Finds the most promising deviations from all nodes in the tree.
*/

pub unsafe fn evaluate_tree() {
    let mut i: libc::c_int = 0;
    let mut feasible_count: libc::c_int = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    prepare_tree_traversal();
    exhausted_node_count = 0 as libc::c_int;
    evaluated_count = 0 as libc::c_int;
    evaluation_stage = 0 as libc::c_int;
    time(&mut start_time);
    feasible_count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < book_node_count {
        let ref mut fresh18 = (*node.offset(i as isize)).flags;
        *fresh18 =
            (*fresh18 as libc::c_int | 8 as libc::c_int) as libc::c_ushort;
        if ((*node.offset(i as isize)).alternative_score as libc::c_int ==
                9999 as libc::c_int ||
                get_node_depth(i) < search_depth &&
                    abs((*node.offset(i as isize)).alternative_score as
                            libc::c_int) >= min_eval_span &&
                    abs((*node.offset(i as isize)).alternative_score as
                            libc::c_int) <= max_eval_span &&
                    abs((*node.offset(i as isize)).black_minimax_score as
                            libc::c_int) >= min_negamax_span &&
                    abs((*node.offset(i as isize)).black_minimax_score as
                            libc::c_int) <= max_negamax_span) &&
               (*node.offset(i as isize)).flags as libc::c_int &
                   (4 as libc::c_int | 16 as libc::c_int) == 0 {
            feasible_count += 1
        }
        i += 1
    }
    max_eval_count =
        if feasible_count < max_batch_size {
            feasible_count
        } else { max_batch_size };
    printf(b"Evaluating to depth %d. \x00" as *const u8 as
               *const libc::c_char, search_depth);
    if min_eval_span > 0 as libc::c_int ||
           max_eval_span < 1000 as libc::c_int * 128 as libc::c_int {
        printf(b"Eval interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const libc::c_char,
               min_eval_span as libc::c_double / 128.0f64,
               max_eval_span as libc::c_double / 128.0f64);
    }
    if min_negamax_span > 0 as libc::c_int ||
           max_negamax_span < 1000 as libc::c_int * 128 as libc::c_int {
        printf(b"Negamax interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const libc::c_char,
               min_negamax_span as libc::c_double / 128.0f64,
               max_negamax_span as libc::c_double / 128.0f64);
    }
    if max_eval_count == feasible_count {
        printf(b"\n%d relevant nodes.\x00" as *const u8 as
                   *const libc::c_char, feasible_count);
    } else {
        printf(b"\nMax batch size is %d.\x00" as *const u8 as
                   *const libc::c_char, max_batch_size);
    }
    puts(b"\x00" as *const u8 as *const libc::c_char);
    printf(b"Progress: \x00" as *const u8 as *const libc::c_char);
    fflush(stdout);
    if feasible_count > 0 as libc::c_int { do_evaluate(0 as libc::c_int); }
    time(&mut stop_time);
    printf(b"(took %d s)\n\x00" as *const u8 as *const libc::c_char,
           (stop_time - start_time) as libc::c_int);
    printf(b"%d nodes evaluated \x00" as *const u8 as *const libc::c_char,
           evaluated_count);
    printf(b"(%d exhausted nodes ignored)\n\x00" as *const u8 as
               *const libc::c_char, exhausted_node_count);
    puts(b"\x00" as *const u8 as *const libc::c_char);
}
/*
   DO_VALIDATE
   Recursively makes sure a subtree doesn't contain any midgame
   node without a deviation move.
*/
unsafe fn do_validate(mut index: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut child: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut this_move: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    if evaluated_count >= max_eval_count { return }
    if (*node.offset(index as isize)).flags as libc::c_int & 8 as libc::c_int
           == 0 {
        return
    }
    if (*node.offset(index as isize)).flags as libc::c_int & 1 as libc::c_int
           != 0 {
        side_to_move = 0 as libc::c_int
    } else { side_to_move = 2 as libc::c_int }
    generate_all(side_to_move);
    if (*node.offset(index as isize)).flags as libc::c_int &
           (16 as libc::c_int | 4 as libc::c_int) == 0 &&
           (*node.offset(index as isize)).alternative_score as libc::c_int ==
               9999 as libc::c_int &&
           (*node.offset(index as isize)).best_alternative_move as libc::c_int
               != -(2 as libc::c_int) {
        evaluate_node(index);
    }
    i = 0 as libc::c_int;
    while i < move_count[disks_played as usize] {
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as libc::c_int);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as libc::c_int) { do_validate(child); }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    let ref mut fresh19 = (*node.offset(index as isize)).flags;
    *fresh19 = (*fresh19 as libc::c_int ^ 8 as libc::c_int) as libc::c_ushort;
}
/*
  VALIDATE_TREE
  Makes sure all nodes are either exhausted, solved or have a deviation.
  The number of positions evaluated is returned.
*/

pub unsafe fn validate_tree() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut feasible_count: libc::c_int = 0;
    prepare_tree_traversal();
    exhausted_node_count = 0 as libc::c_int;
    evaluated_count = 0 as libc::c_int;
    evaluation_stage = 0 as libc::c_int;
    feasible_count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < book_node_count {
        if (*node.offset(i as isize)).flags as libc::c_int &
               (4 as libc::c_int | 16 as libc::c_int) == 0 &&
               (*node.offset(i as isize)).alternative_score as libc::c_int ==
                   9999 as libc::c_int &&
               (*node.offset(i as isize)).best_alternative_move as libc::c_int
                   != -(2 as libc::c_int) {
            feasible_count += 1
        }
        i += 1
    }
    max_eval_count =
        if feasible_count < max_batch_size {
            feasible_count
        } else { max_batch_size };
    if feasible_count > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < book_node_count {
            let ref mut fresh20 = (*node.offset(i as isize)).flags;
            *fresh20 =
                (*fresh20 as libc::c_int | 8 as libc::c_int) as
                    libc::c_ushort;
            i += 1
        }
        do_validate(0 as libc::c_int);
    }
    return evaluated_count;
}
/*
  FILL_ENDGAME_HASH
  Recursively transfer information from all solved nodes in the
  book hash table to the game hash table.
*/

pub unsafe fn fill_endgame_hash(mut cutoff: libc::c_int,
                                           mut level: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut this_index: libc::c_int = 0;
    let mut child_index: libc::c_int = 0;
    let mut matching_move: libc::c_int = 0;
    let mut signed_score: libc::c_int = 0;
    let mut bound: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut this_move: libc::c_int = 0;
    let mut is_full: libc::c_int = 0;
    let mut is_wld: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    if level >= 5 as libc::c_int { return }
    get_hash(&mut val1, &mut val2, &mut orientation);
    slot = probe_hash_table(val1, val2);
    this_index = *book_hash_table.offset(slot as isize);
    /* If the position wasn't found in the hash table, return. */
    if slot == -(1 as libc::c_int) ||
           *book_hash_table.offset(slot as isize) == -(1 as libc::c_int) {
        return
    }
    /* Check the status of the node */
    is_full =
        (*node.offset(this_index as isize)).flags as libc::c_int &
            16 as libc::c_int;
    is_wld =
        (*node.offset(this_index as isize)).flags as libc::c_int &
            4 as libc::c_int;
    /* Match the status of the node with those of the children and
       recursively treat the entire subtree of the node */
    if (*node.offset(*book_hash_table.offset(slot as isize) as isize)).flags
           as libc::c_int & 1 as libc::c_int != 0 {
        side_to_move = 0 as libc::c_int
    } else { side_to_move = 2 as libc::c_int }
    matching_move = -(1 as libc::c_int);
    generate_all(side_to_move);
    i = 0 as libc::c_int;
    while i < move_count[disks_played as usize] {
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as libc::c_int);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child_index = *book_hash_table.offset(slot as isize);
        if child_index != -(1 as libc::c_int) {
            if disks_played < 60 as libc::c_int - cutoff {
                fill_endgame_hash(cutoff, level + 1 as libc::c_int);
            }
            if is_full != 0 {
                /* Any child with matching exact score? */
                if (*node.offset(child_index as isize)).flags as libc::c_int &
                       16 as libc::c_int != 0 &&
                       (*node.offset(child_index as
                                         isize)).black_minimax_score as
                           libc::c_int ==
                           (*node.offset(this_index as
                                             isize)).black_minimax_score as
                               libc::c_int {
                    matching_move = this_move
                }
            } else if is_wld != 0 {
                /* Any child with matching WLD results? */
                if (*node.offset(child_index as isize)).flags as libc::c_int &
                       (16 as libc::c_int | 4 as libc::c_int) != 0 {
                    if side_to_move == 0 as libc::c_int {
                        if (*node.offset(child_index as
                                             isize)).black_minimax_score as
                               libc::c_int >=
                               (*node.offset(this_index as
                                                 isize)).black_minimax_score
                                   as libc::c_int {
                            matching_move = this_move
                        }
                    } else if (*node.offset(child_index as
                                                isize)).black_minimax_score as
                                  libc::c_int <=
                                  (*node.offset(this_index as
                                                    isize)).black_minimax_score
                                      as libc::c_int {
                        matching_move = this_move
                    }
                }
            }
        }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    if matching_move != -(1 as libc::c_int) {
        /* Store the information */
        signed_score =
            (*node.offset(this_index as isize)).black_minimax_score as
                libc::c_int;
        if side_to_move == 2 as libc::c_int { signed_score = -signed_score }
        if signed_score > 30000 as libc::c_int {
            signed_score -= 30000 as libc::c_int
        } else if signed_score < -(30000 as libc::c_int) {
            signed_score += 30000 as libc::c_int
        } else if abs(signed_score) == 30000 as libc::c_int - 1 as libc::c_int
         {
            signed_score = 0 as libc::c_int
        }
        if is_full != 0 {
            bound = 4 as libc::c_int
        } else if signed_score >= 0 as libc::c_int {
            bound = 1 as libc::c_int
        } else { bound = 2 as libc::c_int }
        add_hash(1 as libc::c_int, signed_score, matching_move,
                 16 as libc::c_int | bound, 60 as libc::c_int - disks_played,
                 0 as libc::c_int);
    };
}
/*
   DO_EXAMINE
   Add the properties of node INDEX to the statistics being gathered
   and recursively traverse the subtree of the node, doing the same
   thing in all nodes.
*/
unsafe fn do_examine(mut index: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut child: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut this_move: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    let mut child_count: libc::c_int = 0;
    let mut child_move: [libc::c_int; 64] = [0; 64];
    let mut child_node: [libc::c_int; 64] = [0; 64];
    if (*node.offset(index as isize)).flags as libc::c_int & 8 as libc::c_int
           == 0 {
        return
    }
    if (*node.offset(index as isize)).flags as libc::c_int & 16 as libc::c_int
           != 0 {
        exact_count[disks_played as usize] += 1
    } else if (*node.offset(index as isize)).flags as libc::c_int &
                  4 as libc::c_int != 0 {
        wld_count[disks_played as usize] += 1
    } else if (*node.offset(index as isize)).best_alternative_move as
                  libc::c_int == -(2 as libc::c_int) {
        exhausted_count[disks_played as usize] += 1
    } else { common_count[disks_played as usize] += 1 }
    /* Examine all the children of the node */
    if (*node.offset(index as isize)).flags as libc::c_int & 1 as libc::c_int
           != 0 {
        side_to_move = 0 as libc::c_int
    } else { side_to_move = 2 as libc::c_int }
    generate_all(side_to_move);
    child_count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < move_count[disks_played as usize] {
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as libc::c_int);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as libc::c_int) {
            child_move[child_count as usize] = this_move;
            child_node[child_count as usize] = child;
            child_count += 1
        }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    if child_count == 0 as libc::c_int {
        leaf_count += 1;
        if (*node.offset(index as isize)).flags as libc::c_int &
               16 as libc::c_int == 0 {
            bad_leaf_count += 1
        }
        if (*node.offset(index as isize)).flags as libc::c_int &
               4 as libc::c_int == 0 {
            really_bad_leaf_count += 1
        }
    } else {
        let mut current_block_38: u64;
        i = 0 as libc::c_int;
        while i < child_count {
            if side_to_move == 0 as libc::c_int {
                if force_black != 0 &&
                       (*node.offset(child_node[i as usize] as
                                         isize)).black_minimax_score as
                           libc::c_int !=
                           (*node.offset(index as isize)).black_minimax_score
                               as libc::c_int {
                    current_block_38 = 2873832966593178012;
                } else { current_block_38 = 10891380440665537214; }
            } else if force_white != 0 &&
                          (*node.offset(child_node[i as usize] as
                                            isize)).white_minimax_score as
                              libc::c_int !=
                              (*node.offset(index as
                                                isize)).white_minimax_score as
                                  libc::c_int {
                current_block_38 = 2873832966593178012;
            } else { current_block_38 = 10891380440665537214; }
            match current_block_38 {
                10891380440665537214 => {
                    this_move = child_move[i as usize];
                    make_move(side_to_move, this_move, 1 as libc::c_int);
                    do_examine(child_node[i as usize]);
                    unmake_move(side_to_move, this_move);
                }
                _ => { }
            }
            i += 1
        }
    }
    let ref mut fresh21 = (*node.offset(index as isize)).flags;
    *fresh21 = (*fresh21 as libc::c_int ^ 8 as libc::c_int) as libc::c_ushort;
}
/*
   EXAMINE_TREE
   Generates some statistics about the book tree.
*/

pub unsafe fn examine_tree() {
    let mut i: libc::c_int = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    printf(b"Examining tree... \x00" as *const u8 as *const libc::c_char);
    fflush(stdout);
    prepare_tree_traversal();
    time(&mut start_time);
    i = 0 as libc::c_int;
    while i <= 60 as libc::c_int {
        exact_count[i as usize] = 0 as libc::c_int;
        wld_count[i as usize] = 0 as libc::c_int;
        exhausted_count[i as usize] = 0 as libc::c_int;
        common_count[i as usize] = 0 as libc::c_int;
        i += 1
    }
    unreachable_count = 0 as libc::c_int;
    leaf_count = 0 as libc::c_int;
    bad_leaf_count = 0 as libc::c_int;
    /* Mark all nodes as not traversed and examine the tree */
    i = 0 as libc::c_int;
    while i < book_node_count {
        let ref mut fresh22 = (*node.offset(i as isize)).flags;
        *fresh22 =
            (*fresh22 as libc::c_int | 8 as libc::c_int) as libc::c_ushort;
        i += 1
    }
    do_examine(0 as libc::c_int);
    /* Any nodes not reached by the walkthrough? */
    i = 0 as libc::c_int;
    while i < book_node_count {
        if (*node.offset(i as isize)).flags as libc::c_int & 8 as libc::c_int
               != 0 {
            unreachable_count += 1;
            let ref mut fresh23 = (*node.offset(i as isize)).flags;
            *fresh23 =
                (*fresh23 as libc::c_int ^ 8 as libc::c_int) as libc::c_ushort
        }
        i += 1
    }
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const libc::c_char,
           (stop_time - start_time) as libc::c_int);
    puts(b"\x00" as *const u8 as *const libc::c_char);
}
unsafe fn int_compare(mut i1: *const libc::c_void,
                                 mut i2: *const libc::c_void) -> libc::c_int {
    return *(i1 as *mut libc::c_int) - *(i2 as *mut libc::c_int);
}
/*
   BOOK_STATISTICS
   Describe the status of the nodes in the tree.
*/

pub unsafe fn book_statistics(mut full_statistics: libc::c_int) {
    let mut strata: [libc::c_double; 11] =
        [0.01f64, 0.02f64, 0.03f64, 0.05f64, 0.10f64, 0.30f64, 0.50f64,
         0.70f64, 0.90f64, 0.99f64, 1.01f64];
    let mut eval_strata: [libc::c_double; 10] = [0.; 10];
    let mut negamax_strata: [libc::c_double; 10] = [0.; 10];
    let mut i: libc::c_int = 0;
    let mut full_solved: libc::c_int = 0;
    let mut wld_solved: libc::c_int = 0;
    let mut unevaluated: libc::c_int = 0;
    let mut eval_count: libc::c_int = 0;
    let mut negamax_count: libc::c_int = 0;
    let mut private_count: libc::c_int = 0;
    let mut this_strata: libc::c_int = 0;
    let mut strata_shift: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut evals = 0 as *mut libc::c_int;
    let mut negamax = 0 as *mut libc::c_int;
    let mut depth: [libc::c_int; 60] = [0; 60];
    let mut total_count: [libc::c_int; 61] = [0; 61];
    evals =
        safe_malloc((book_node_count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                         as libc::c_ulong)) as
            *mut libc::c_int;
    negamax =
        safe_malloc((book_node_count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                         as libc::c_ulong)) as
            *mut libc::c_int;
    wld_solved = 0 as libc::c_int;
    full_solved = wld_solved;
    eval_count = 0 as libc::c_int;
    negamax_count = 0 as libc::c_int;
    private_count = 0 as libc::c_int;
    unevaluated = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 60 as libc::c_int {
        depth[i as usize] = 0 as libc::c_int;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < book_node_count {
        if (*node.offset(i as isize)).flags as libc::c_int & 16 as libc::c_int
               != 0 {
            full_solved += 1
        } else if (*node.offset(i as isize)).flags as libc::c_int &
                      4 as libc::c_int != 0 {
            wld_solved += 1
        } else {
            depth[get_node_depth(i) as usize] += 1;
            if (*node.offset(i as isize)).alternative_score as libc::c_int ==
                   9999 as libc::c_int &&
                   (*node.offset(i as isize)).best_alternative_move as
                       libc::c_int == -(1 as libc::c_int) {
                unevaluated += 1
            } else {
                if (*node.offset(i as isize)).alternative_score as libc::c_int
                       != 9999 as libc::c_int {
                    let fresh24 = eval_count;
                    eval_count = eval_count + 1;
                    *evals.offset(fresh24 as isize) =
                        abs((*node.offset(i as isize)).alternative_score as
                                libc::c_int)
                }
                let fresh25 = negamax_count;
                negamax_count = negamax_count + 1;
                *negamax.offset(fresh25 as isize) =
                    abs((*node.offset(i as isize)).black_minimax_score as
                            libc::c_int)
            }
        }
        if (*node.offset(i as isize)).flags as libc::c_int & 32 as libc::c_int
               != 0 {
            private_count += 1
        }
        i += 1
    }
    qsort(evals as *mut libc::c_void, eval_count as size_t,
          ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
          Some(int_compare as
                   unsafe fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    qsort(negamax as *mut libc::c_void, negamax_count as size_t,
          ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
          Some(int_compare as
                   unsafe fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    puts(b"\x00" as *const u8 as *const libc::c_char);
    printf(b"#nodes:       %d\x00" as *const u8 as *const libc::c_char,
           book_node_count);
    if private_count > 0 as libc::c_int {
        printf(b"  (%d private)\x00" as *const u8 as *const libc::c_char,
               private_count);
    }
    puts(b"\x00" as *const u8 as *const libc::c_char);
    printf(b"#full solved: %d\n\x00" as *const u8 as *const libc::c_char,
           full_solved);
    printf(b"#WLD solved:  %d\n\x00" as *const u8 as *const libc::c_char,
           wld_solved);
    printf(b"#unevaluated: %d\n\n\x00" as *const u8 as *const libc::c_char,
           unevaluated);
    i = 0 as libc::c_int;
    while i <= 59 as libc::c_int {
        if depth[i as usize] > 0 as libc::c_int {
            printf(b"#nodes with %2d-ply deviations: %d\n\x00" as *const u8 as
                       *const libc::c_char, i, depth[i as usize]);
        }
        i += 1
    }
    puts(b"\x00" as *const u8 as *const libc::c_char);
    this_strata = 0 as libc::c_int;
    strata_shift =
        floor(strata[this_strata as usize] * eval_count as libc::c_double) as
            libc::c_int;
    i = 0 as libc::c_int;
    while i < eval_count {
        if i == strata_shift {
            eval_strata[this_strata as usize] =
                *evals.offset(i as isize) as libc::c_double / 128.0f64;
            this_strata += 1;
            strata_shift =
                floor(strata[this_strata as usize] *
                          eval_count as libc::c_double) as libc::c_int
        }
        i += 1
    }
    this_strata = 0 as libc::c_int;
    strata_shift =
        floor(strata[this_strata as usize] * negamax_count as libc::c_double)
            as libc::c_int;
    i = 0 as libc::c_int;
    while i < negamax_count {
        if i == strata_shift {
            negamax_strata[this_strata as usize] =
                *evals.offset(i as isize) as libc::c_double / 128.0f64;
            this_strata += 1;
            strata_shift =
                floor(strata[this_strata as usize] *
                          negamax_count as libc::c_double) as libc::c_int
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        printf(b"%2.0f%%:  \x00" as *const u8 as *const libc::c_char,
               100 as libc::c_int as libc::c_double * strata[i as usize]);
        printf(b"%5.2f   \x00" as *const u8 as *const libc::c_char,
               eval_strata[i as usize]);
        printf(b"%5.2f   \x00" as *const u8 as *const libc::c_char,
               negamax_strata[i as usize]);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        i += 1
    }
    puts(b"\x00" as *const u8 as *const libc::c_char);
    free(negamax as *mut libc::c_void);
    free(evals as *mut libc::c_void);
    if full_statistics != 0 {
        examine_tree();
        first = 61 as libc::c_int;
        last = -(1 as libc::c_int);
        i = 0 as libc::c_int;
        while i <= 60 as libc::c_int {
            total_count[i as usize] =
                exact_count[i as usize] + wld_count[i as usize] +
                    exhausted_count[i as usize] + common_count[i as usize];
            if total_count[i as usize] > 0 as libc::c_int {
                first = if first < i { first } else { i };
                last = if last > i { last } else { i }
            }
            i += 1
        }
        printf(b"%d unreachable nodes\n\n\x00" as *const u8 as
                   *const libc::c_char, unreachable_count);
        printf(b"%d leaf nodes; %d lack exact score and %d lack WLD status\n\x00"
                   as *const u8 as *const libc::c_char, leaf_count,
               bad_leaf_count, really_bad_leaf_count);
        i = first;
        while i <= last {
            printf(b"%2d moves\x00" as *const u8 as *const libc::c_char, i);
            printf(b"   \x00" as *const u8 as *const libc::c_char);
            printf(b"%5d node\x00" as *const u8 as *const libc::c_char,
                   total_count[i as usize]);
            if total_count[i as usize] == 1 as libc::c_int {
                printf(b" :  \x00" as *const u8 as *const libc::c_char);
            } else {
                printf(b"s:  \x00" as *const u8 as *const libc::c_char);
            }
            if common_count[i as usize] > 0 as libc::c_int {
                printf(b"%5d midgame\x00" as *const u8 as *const libc::c_char,
                       common_count[i as usize]);
            } else {
                printf(b"             \x00" as *const u8 as
                           *const libc::c_char);
            }
            printf(b"  \x00" as *const u8 as *const libc::c_char);
            if wld_count[i as usize] > 0 as libc::c_int {
                printf(b"%5d WLD\x00" as *const u8 as *const libc::c_char,
                       wld_count[i as usize]);
            } else {
                printf(b"         \x00" as *const u8 as *const libc::c_char);
            }
            printf(b"  \x00" as *const u8 as *const libc::c_char);
            if exact_count[i as usize] > 0 as libc::c_int {
                printf(b"%5d exact\x00" as *const u8 as *const libc::c_char,
                       exact_count[i as usize]);
            } else {
                printf(b"           \x00" as *const u8 as
                           *const libc::c_char);
            }
            printf(b"  \x00" as *const u8 as *const libc::c_char);
            if exhausted_count[i as usize] > 0 as libc::c_int {
                printf(b"%2d exhausted\x00" as *const u8 as
                           *const libc::c_char, exhausted_count[i as usize]);
            }
            puts(b"\x00" as *const u8 as *const libc::c_char);
            i += 1
        }
        puts(b"\x00" as *const u8 as *const libc::c_char);
    };
}
/*
   DISPLAY_OPTIMAL_LINE
   Outputs the sequence of moves which is optimal according
   to both players.
*/

pub unsafe fn display_doubly_optimal_line(mut original_side_to_move:
                                                         libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut done: libc::c_int = 0;
    let mut show_move: libc::c_int = 0;
    let mut line: libc::c_int = 0;
    let mut root_score: libc::c_int = 0;
    let mut child_score: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut base_orientation: libc::c_int = 0;
    let mut child_orientation: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut this_move: libc::c_int = 0;
    let mut current: libc::c_int = 0;
    let mut child: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    prepare_tree_traversal();
    printf(b"Root evaluation with Zebra playing \x00" as *const u8 as
               *const libc::c_char);
    if original_side_to_move == 0 as libc::c_int {
        root_score =
            (*node.offset(0 as libc::c_int as isize)).black_minimax_score as
                libc::c_int;
        printf(b"black\x00" as *const u8 as *const libc::c_char);
    } else {
        root_score =
            (*node.offset(0 as libc::c_int as isize)).white_minimax_score as
                libc::c_int;
        printf(b"white\x00" as *const u8 as *const libc::c_char);
    }
    printf(b": %+.2f\n\x00" as *const u8 as *const libc::c_char,
           root_score as libc::c_double / 128.0f64);
    current = 0 as libc::c_int;
    puts(b"Preferred line: \x00" as *const u8 as *const libc::c_char);
    line = 0 as libc::c_int;
    done = 0 as libc::c_int;
    show_move = 1 as libc::c_int;
    while (*node.offset(current as isize)).flags as libc::c_int &
              (16 as libc::c_int | 4 as libc::c_int) == 0 && done == 0 {
        if (*node.offset(current as isize)).flags as libc::c_int &
               1 as libc::c_int != 0 {
            side_to_move = 0 as libc::c_int
        } else { side_to_move = 2 as libc::c_int }
        generate_all(side_to_move);
        next = -(1 as libc::c_int);
        this_move = -(1 as libc::c_int);
        i = 0 as libc::c_int;
        while i < move_count[disks_played as usize] {
            get_hash(&mut val1, &mut val2, &mut base_orientation);
            this_move = move_list[disks_played as usize][i as usize];
            make_move(side_to_move, this_move, 1 as libc::c_int);
            get_hash(&mut val1, &mut val2, &mut child_orientation);
            slot = probe_hash_table(val1, val2);
            child = *book_hash_table.offset(slot as isize);
            if child != -(1 as libc::c_int) {
                if original_side_to_move == 0 as libc::c_int {
                    child_score =
                        (*node.offset(child as isize)).black_minimax_score as
                            libc::c_int
                } else {
                    child_score =
                        (*node.offset(child as isize)).white_minimax_score as
                            libc::c_int
                }
                if child_score == root_score { next = child }
            }
            if child != -(1 as libc::c_int) && next == child { break ; }
            unmake_move(side_to_move, this_move);
            i += 1
        }
        if next == -(1 as libc::c_int) {
            done = 1 as libc::c_int;
            if adjust_score((*node.offset(current as isize)).alternative_score
                                as libc::c_int, side_to_move) != root_score {
                puts(b"(failed to find continuation)\x00" as *const u8 as
                         *const libc::c_char);
                show_move = 0 as libc::c_int
            } else {
                this_move =
                    (*node.offset(current as isize)).best_alternative_move as
                        libc::c_int;
                this_move =
                    *inv_symmetry_map[base_orientation as
                                          usize].offset(this_move as isize)
            }
        }
        if show_move != 0 {
            if side_to_move == 0 as libc::c_int {
                line += 1;
                printf(b"%2d. \x00" as *const u8 as *const libc::c_char,
                       line);
            }
            printf(b"%c%c  \x00" as *const u8 as *const libc::c_char,
                   'a' as i32 + this_move % 10 as libc::c_int -
                       1 as libc::c_int,
                   '0' as i32 + this_move / 10 as libc::c_int);
            if side_to_move == 2 as libc::c_int {
                puts(b"\x00" as *const u8 as *const libc::c_char);
            }
            if done != 0 {
                puts(b"(deviation)\x00" as *const u8 as *const libc::c_char);
            }
        }
        current = next
    }
    puts(b"\x00" as *const u8 as *const libc::c_char);
}
/*
  ADD_NEW_GAME
  Adds a new game to the game tree.
*/

pub unsafe fn add_new_game(mut move_count_0: libc::c_int,
                                      mut game_move_list: *mut libc::c_short,
                                      mut min_empties: libc::c_int,
                                      mut max_full_solve: libc::c_int,
                                      mut max_wld_solve: libc::c_int,
                                      mut update_path: libc::c_int,
                                      mut private_game: libc::c_int) {
    let mut dummy_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut stored_echo: libc::c_int = 0;
    let mut dummy_black_score: libc::c_int = 0;
    let mut dummy_white_score: libc::c_int = 0;
    let mut force_eval: libc::c_int = 0;
    let mut midgame_eval_done: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut this_move: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut this_node: libc::c_int = 0;
    let mut last_move_number: libc::c_int = 0;
    let mut first_new_node: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    let mut outcome: libc::c_int = 0;
    let mut visited_node: [libc::c_int; 61] = [0; 61];
    let mut flags: [libc::c_ushort; 61] = [0; 61];
    stored_echo = echo;
    echo = 0 as libc::c_int;
    toggle_event_status(0 as libc::c_int);
    /* First create new nodes for new positions */
    prepare_tree_traversal();
    i = 0 as libc::c_int;
    while i < move_count_0 {
        if *game_move_list.offset(i as isize) as libc::c_int >
               0 as libc::c_int {
            flags[i as usize] = 1 as libc::c_int as libc::c_ushort
        } else { flags[i as usize] = 2 as libc::c_int as libc::c_ushort }
        i += 1
    }
    flags[move_count_0 as usize] = 0 as libc::c_int as libc::c_ushort;
    first_new_node = 61 as libc::c_int;
    this_node = 0 as libc::c_int;
    side_to_move = 0 as libc::c_int;
    last_move_number =
        if move_count_0 < 60 as libc::c_int - min_empties {
            move_count_0
        } else { (60 as libc::c_int) - min_empties };
    i = 0 as libc::c_int;
    while i <= last_move_number {
        /* Look for the position in the hash table */
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        if slot == -(1 as libc::c_int) ||
               *book_hash_table.offset(slot as isize) == -(1 as libc::c_int) {
            this_node = create_BookNode(val1, val2, flags[i as usize]);
            if private_game != 0 {
                let ref mut fresh26 =
                    (*node.offset(this_node as isize)).flags;
                *fresh26 =
                    (*fresh26 as libc::c_int | 32 as libc::c_int) as
                        libc::c_ushort
            }
            if i < first_new_node { first_new_node = i }
        } else { this_node = *book_hash_table.offset(slot as isize) }
        visited_node[i as usize] = this_node;
        /* Make the moves of the game until the cutoff point */
        if i < last_move_number {
            this_move =
                abs(*game_move_list.offset(i as isize) as libc::c_int);
            if *game_move_list.offset(i as isize) as libc::c_int >
                   0 as libc::c_int {
                side_to_move = 0 as libc::c_int
            } else { side_to_move = 2 as libc::c_int }
            if generate_specific(this_move, side_to_move) == 0 {
                puts(b"\x00" as *const u8 as *const libc::c_char);
                printf(b"i=%d, side_to_move=%d, this_move=%d\n\x00" as
                           *const u8 as *const libc::c_char, i, side_to_move,
                       this_move);
                printf(b"last_move_number=%d, move_count=%d\n\x00" as
                           *const u8 as *const libc::c_char, last_move_number,
                       move_count_0);
                j = 0 as libc::c_int;
                while j < move_count_0 {
                    printf(b"%3d \x00" as *const u8 as *const libc::c_char,
                           *game_move_list.offset(j as isize) as libc::c_int);
                    j += 1
                }
                fatal_error(b"%s: %d\n\x00" as *const u8 as
                                *const libc::c_char,
                            b"Invalid move generated\x00" as *const u8 as
                                *const libc::c_char, this_move);
            }
            make_move(side_to_move, this_move, 1 as libc::c_int);
        } else {
            /* No more move to make, only update the player to move */
            side_to_move = 0 as libc::c_int + 2 as libc::c_int - side_to_move
        }
        i += 1
    }
    if last_move_number == move_count_0 {
        /* No cutoff applies */
        let mut black_count: libc::c_int = 0;
        let mut white_count: libc::c_int = 0;
        black_count = disc_count(0 as libc::c_int);
        white_count = disc_count(2 as libc::c_int);
        if black_count > white_count {
            outcome = 64 as libc::c_int - 2 as libc::c_int * white_count
        } else if white_count > black_count {
            outcome = 2 as libc::c_int * black_count - 64 as libc::c_int
        } else { outcome = 0 as libc::c_int }
    } else {
        generate_all(side_to_move);
        determine_hash_values(side_to_move, board.as_mut_ptr());
        if echo != 0 {
            puts(b"\x00" as *const u8 as *const libc::c_char);
            if side_to_move == 0 as libc::c_int {
                printf(b"Full solving with %d empty (black)\n\x00" as
                           *const u8 as *const libc::c_char,
                       60 as libc::c_int - disks_played);
            } else {
                printf(b"Full solving with %d empty (white)\n\x00" as
                           *const u8 as *const libc::c_char,
                       60 as libc::c_int - disks_played);
            }
        }
        end_game(side_to_move, 0 as libc::c_int, 0 as libc::c_int,
                 1 as libc::c_int, 0 as libc::c_int, &mut dummy_info);
        outcome = root_eval;
        if side_to_move == 2 as libc::c_int { outcome = -outcome }
    }
    (*node.offset(this_node as isize)).black_minimax_score =
        outcome as libc::c_short;
    (*node.offset(this_node as isize)).white_minimax_score =
        outcome as libc::c_short;
    if outcome > 0 as libc::c_int {
        let ref mut fresh27 =
            (*node.offset(this_node as isize)).black_minimax_score;
        *fresh27 =
            (*fresh27 as libc::c_int + 30000 as libc::c_int) as libc::c_short;
        let ref mut fresh28 =
            (*node.offset(this_node as isize)).white_minimax_score;
        *fresh28 =
            (*fresh28 as libc::c_int + 30000 as libc::c_int) as libc::c_short
    }
    if outcome < 0 as libc::c_int {
        let ref mut fresh29 =
            (*node.offset(this_node as isize)).black_minimax_score;
        *fresh29 =
            (*fresh29 as libc::c_int - 30000 as libc::c_int) as libc::c_short;
        let ref mut fresh30 =
            (*node.offset(this_node as isize)).white_minimax_score;
        *fresh30 =
            (*fresh30 as libc::c_int - 30000 as libc::c_int) as libc::c_short
    }
    let ref mut fresh31 = (*node.offset(this_node as isize)).flags;
    *fresh31 =
        (*fresh31 as libc::c_int | 16 as libc::c_int) as libc::c_ushort;
    /* Take another pass through the midgame to update move
       alternatives and minimax information if requested. */
    if update_path != 0 {
        prepare_tree_traversal();
        i = 0 as libc::c_int;
        while i < last_move_number {
            this_move =
                abs(*game_move_list.offset(i as isize) as libc::c_int);
            if *game_move_list.offset(i as isize) as libc::c_int >
                   0 as libc::c_int {
                side_to_move = 0 as libc::c_int
            } else { side_to_move = 2 as libc::c_int }
            if generate_specific(this_move, side_to_move) == 0 {
                fatal_error(b"%s: %d\n\x00" as *const u8 as
                                *const libc::c_char,
                            b"Invalid move generated\x00" as *const u8 as
                                *const libc::c_char, this_move);
            }
            make_move(side_to_move, this_move, 1 as libc::c_int);
            i += 1
        }
        if echo != 0 { fflush(stdout); }
        midgame_eval_done = 0 as libc::c_int;
        i = last_move_number - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            this_move =
                abs(*game_move_list.offset(i as isize) as libc::c_int);
            if *game_move_list.offset(i as isize) as libc::c_int >
                   0 as libc::c_int {
                side_to_move = 0 as libc::c_int
            } else { side_to_move = 2 as libc::c_int }
            unmake_move(side_to_move, this_move);
            /* If the game was public, make sure that all nodes that
            previously marked as private nodes are marked as public. */
            this_node = visited_node[i as usize];
            if private_game == 0 &&
                   (*node.offset(this_node as isize)).flags as libc::c_int &
                       32 as libc::c_int != 0 {
                let ref mut fresh32 =
                    (*node.offset(this_node as isize)).flags;
                *fresh32 =
                    (*fresh32 as libc::c_int ^ 32 as libc::c_int) as
                        libc::c_ushort
            }
            if (*node.offset(this_node as isize)).flags as libc::c_int &
                   1 as libc::c_int != 0 {
                side_to_move = 0 as libc::c_int
            } else { side_to_move = 2 as libc::c_int }
            generate_all(side_to_move);
            determine_hash_values(side_to_move, board.as_mut_ptr());
            if disks_played >= 60 as libc::c_int - max_full_solve {
                /* Only solve the position if it hasn't been solved already */
                if (*node.offset(this_node as isize)).flags as libc::c_int &
                       16 as libc::c_int == 0 {
                    end_game(side_to_move, 0 as libc::c_int, 0 as libc::c_int,
                             1 as libc::c_int, 0 as libc::c_int,
                             &mut dummy_info);
                    if side_to_move == 0 as libc::c_int {
                        outcome = root_eval
                    } else { outcome = -root_eval }
                    (*node.offset(this_node as isize)).black_minimax_score =
                        outcome as libc::c_short;
                    (*node.offset(this_node as isize)).white_minimax_score =
                        outcome as libc::c_short;
                    if outcome > 0 as libc::c_int {
                        let ref mut fresh33 =
                            (*node.offset(this_node as
                                              isize)).black_minimax_score;
                        *fresh33 =
                            (*fresh33 as libc::c_int + 30000 as libc::c_int)
                                as libc::c_short;
                        let ref mut fresh34 =
                            (*node.offset(this_node as
                                              isize)).white_minimax_score;
                        *fresh34 =
                            (*fresh34 as libc::c_int + 30000 as libc::c_int)
                                as libc::c_short
                    }
                    if outcome < 0 as libc::c_int {
                        let ref mut fresh35 =
                            (*node.offset(this_node as
                                              isize)).black_minimax_score;
                        *fresh35 =
                            (*fresh35 as libc::c_int - 30000 as libc::c_int)
                                as libc::c_short;
                        let ref mut fresh36 =
                            (*node.offset(this_node as
                                              isize)).white_minimax_score;
                        *fresh36 =
                            (*fresh36 as libc::c_int - 30000 as libc::c_int)
                                as libc::c_short
                    }
                    let ref mut fresh37 =
                        (*node.offset(this_node as isize)).flags;
                    *fresh37 =
                        (*fresh37 as libc::c_int | 16 as libc::c_int) as
                            libc::c_ushort
                }
            } else if disks_played >= 60 as libc::c_int - max_wld_solve {
                /* Only solve the position if its WLD status is unknown */
                if (*node.offset(this_node as isize)).flags as libc::c_int &
                       4 as libc::c_int == 0 {
                    end_game(side_to_move, 1 as libc::c_int, 0 as libc::c_int,
                             1 as libc::c_int, 0 as libc::c_int,
                             &mut dummy_info);
                    if side_to_move == 0 as libc::c_int {
                        outcome = root_eval
                    } else { outcome = -root_eval }
                    (*node.offset(this_node as isize)).black_minimax_score =
                        outcome as libc::c_short;
                    (*node.offset(this_node as isize)).white_minimax_score =
                        outcome as libc::c_short;
                    if outcome > 0 as libc::c_int {
                        let ref mut fresh38 =
                            (*node.offset(this_node as
                                              isize)).black_minimax_score;
                        *fresh38 =
                            (*fresh38 as libc::c_int + 30000 as libc::c_int)
                                as libc::c_short;
                        let ref mut fresh39 =
                            (*node.offset(this_node as
                                              isize)).white_minimax_score;
                        *fresh39 =
                            (*fresh39 as libc::c_int + 30000 as libc::c_int)
                                as libc::c_short
                    }
                    if outcome < 0 as libc::c_int {
                        let ref mut fresh40 =
                            (*node.offset(this_node as
                                              isize)).black_minimax_score;
                        *fresh40 =
                            (*fresh40 as libc::c_int - 30000 as libc::c_int)
                                as libc::c_short;
                        let ref mut fresh41 =
                            (*node.offset(this_node as
                                              isize)).white_minimax_score;
                        *fresh41 =
                            (*fresh41 as libc::c_int - 30000 as libc::c_int)
                                as libc::c_short
                    }
                    let ref mut fresh42 =
                        (*node.offset(this_node as isize)).flags;
                    *fresh42 =
                        (*fresh42 as libc::c_int | 4 as libc::c_int) as
                            libc::c_ushort
                }
            } else {
                force_eval =
                    (i >= first_new_node - 1 as libc::c_int ||
                         (*node.offset(this_node as
                                           isize)).best_alternative_move as
                             libc::c_int ==
                             abs(*game_move_list.offset(i as isize) as
                                     libc::c_int)) as libc::c_int;
                if midgame_eval_done == 0 {
                    printf(b"Evaluating: \x00" as *const u8 as
                               *const libc::c_char);
                    fflush(stdout);
                }
                midgame_eval_done = 1 as libc::c_int;
                if force_eval != 0 { clear_node_depth(this_node); }
                evaluate_node(this_node);
                printf(b"|\x00" as *const u8 as *const libc::c_char);
                fflush(stdout);
            }
            let ref mut fresh43 = (*node.offset(this_node as isize)).flags;
            *fresh43 =
                (*fresh43 as libc::c_int | 8 as libc::c_int) as
                    libc::c_ushort;
            do_minimax(this_node, &mut dummy_black_score,
                       &mut dummy_white_score);
            if (*node.offset(this_node as isize)).flags as libc::c_int &
                   4 as libc::c_int == 0 &&
                   (*node.offset(this_node as isize)).best_alternative_move as
                       libc::c_int == -(1 as libc::c_int) &&
                   (*node.offset(this_node as isize)).alternative_score as
                       libc::c_int == 9999 as libc::c_int {
                /* Minimax discovered that the node hasn't got a deviation any
                   longer because that move has been played. */
                evaluate_node(this_node);
                printf(b"-|-\x00" as *const u8 as *const libc::c_char);
                do_minimax(this_node, &mut dummy_black_score,
                           &mut dummy_white_score);
            }
            i -= 1
        }
        puts(b"\x00" as *const u8 as *const libc::c_char);
    }
    toggle_event_status(1 as libc::c_int);
    echo = stored_echo;
    total_game_count += 1;
}
/*
   BUILD_TREE
   Reads games from the file pointed to by FILE_NAME and
   incorporates them into the game tree.
*/

pub unsafe fn build_tree(mut file_name: *const libc::c_char,
                                    mut max_game_count: libc::c_int,
                                    mut max_diff: libc::c_int,
                                    mut min_empties: libc::c_int) {
    let mut move_string: [libc::c_char; 200] = [0; 200];
    let mut line_buffer: [libc::c_char; 1000] = [0; 1000];
    let mut sign: libc::c_char = 0;
    let mut column: libc::c_char = 0;
    let mut row: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    let mut games_parsed: libc::c_int = 0;
    let mut games_imported: libc::c_int = 0;
    let mut move_count_0: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut game_move_list: [libc::c_short; 60] = [0; 60];
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut stream = 0 as *mut FILE;
    puts(b"Importing game list...\x00" as *const u8 as *const libc::c_char);
    fflush(stdout);
    stream = fopen(file_name, b"r\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                    b"Could not open game file\x00" as *const u8 as
                        *const libc::c_char, file_name);
    }
    time(&mut start_time);
    games_parsed = 0 as libc::c_int;
    games_imported = 0 as libc::c_int;
    loop  {
        fgets(line_buffer.as_mut_ptr(), 998 as libc::c_int, stream);
        sscanf(line_buffer.as_mut_ptr(),
               b"%s %d\x00" as *const u8 as *const libc::c_char,
               move_string.as_mut_ptr(), &mut diff as *mut libc::c_int);
        move_count_0 =
            strlen(move_string.as_mut_ptr()).wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong).wrapping_div(3
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong)
                as libc::c_int;
        games_parsed += 1;
        i = 0 as libc::c_int;
        while i < move_count_0 {
            sscanf(move_string.as_mut_ptr().offset((3 as libc::c_int * i) as
                                                       isize),
                   b"%c%c%c\x00" as *const u8 as *const libc::c_char,
                   &mut sign as *mut libc::c_char,
                   &mut column as *mut libc::c_char,
                   &mut row as *mut libc::c_char);
            game_move_list[i as usize] =
                (10 as libc::c_int * (row as libc::c_int - '0' as i32) +
                     (column as libc::c_int - 'a' as i32 + 1 as libc::c_int))
                    as libc::c_short;
            if sign as libc::c_int == '-' as i32 {
                game_move_list[i as usize] =
                    -(game_move_list[i as usize] as libc::c_int) as
                        libc::c_short
            }
            i += 1
        }
        if abs(diff) <= max_diff {
            add_new_game(move_count_0, game_move_list.as_mut_ptr(),
                         min_empties, 0 as libc::c_int, 0 as libc::c_int,
                         0 as libc::c_int, 0 as libc::c_int);
            printf(b"|\x00" as *const u8 as *const libc::c_char);
            if games_imported % 100 as libc::c_int == 0 as libc::c_int {
                printf(b" --- %d games --- \x00" as *const u8 as
                           *const libc::c_char, games_imported);
            }
            fflush(stdout);
            games_imported += 1
        }
        if !(games_parsed < max_game_count) { break ; }
    }
    time(&mut stop_time);
    fclose(stream);
    printf(b"\ndone (took %d s)\n\x00" as *const u8 as *const libc::c_char,
           (stop_time - start_time) as libc::c_int);
    printf(b"%d games read; %d games imported\n\x00" as *const u8 as
               *const libc::c_char, games_parsed, games_imported);
    printf(b"Games with final difference <= %d were read until %d empties.\n\x00"
               as *const u8 as *const libc::c_char, max_diff, min_empties);
    puts(b"\x00" as *const u8 as *const libc::c_char);
}
/*
   READ_TEXT_DATABASE
   Reads an existing ASCII database file.
*/

pub unsafe fn read_text_database(mut file_name:
                                                *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut magic1: libc::c_int = 0;
    let mut magic2: libc::c_int = 0;
    let mut new_book_node_count: libc::c_int = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut stream = 0 as *mut FILE;
    time(&mut start_time);
    printf(b"Reading text opening database... \x00" as *const u8 as
               *const libc::c_char);
    fflush(stdout);
    stream = fopen(file_name, b"r\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                    b"Could not open database file\x00" as *const u8 as
                        *const libc::c_char, file_name);
    }
    fscanf(stream, b"%d\x00" as *const u8 as *const libc::c_char,
           &mut magic1 as *mut libc::c_int);
    fscanf(stream, b"%d\x00" as *const u8 as *const libc::c_char,
           &mut magic2 as *mut libc::c_int);
    if magic1 != 2718 as libc::c_int || magic2 != 2818 as libc::c_int {
        fatal_error(b"%s: %s\x00" as *const u8 as *const libc::c_char,
                    b"Wrong checksum, might be an old version\x00" as
                        *const u8 as *const libc::c_char, file_name);
    }
    fscanf(stream, b"%d\x00" as *const u8 as *const libc::c_char,
           &mut new_book_node_count as *mut libc::c_int);
    set_allocation(new_book_node_count + 1000 as libc::c_int);
    i = 0 as libc::c_int;
    while i < new_book_node_count {
        fscanf(stream,
               b"%d %d %hd %hd %hd %hd %hd\n\x00" as *const u8 as
                   *const libc::c_char,
               &mut (*node.offset(i as isize)).hash_val1 as *mut libc::c_int,
               &mut (*node.offset(i as isize)).hash_val2 as *mut libc::c_int,
               &mut (*node.offset(i as isize)).black_minimax_score as
                   *mut libc::c_short,
               &mut (*node.offset(i as isize)).white_minimax_score as
                   *mut libc::c_short,
               &mut (*node.offset(i as isize)).best_alternative_move as
                   *mut libc::c_short,
               &mut (*node.offset(i as isize)).alternative_score as
                   *mut libc::c_short,
               &mut (*node.offset(i as isize)).flags as *mut libc::c_ushort);
        i += 1
    }
    book_node_count = new_book_node_count;
    create_hash_reference();
    fclose(stream);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const libc::c_char,
           (stop_time - start_time) as libc::c_int);
    puts(b"\x00" as *const u8 as *const libc::c_char);
}
/*
   READ_BINARY_DATABASE
   Reads a binary database file.
*/

pub unsafe fn read_binary_database(mut file_name:
                                                  *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut new_book_node_count: libc::c_int = 0;
    let mut magic1: libc::c_short = 0;
    let mut magic2: libc::c_short = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut stream = 0 as *mut FILE;
    time(&mut start_time);
    printf(b"Reading binary opening database... \x00" as *const u8 as
               *const libc::c_char);
    fflush(stdout);
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                    b"Could not open database file\x00" as *const u8 as
                        *const libc::c_char, file_name);
    }
    fread(&mut magic1 as *mut libc::c_short as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
          1 as libc::c_int as size_t, stream);
    fread(&mut magic2 as *mut libc::c_short as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
          1 as libc::c_int as size_t, stream);
    if magic1 as libc::c_int != 2718 as libc::c_int ||
           magic2 as libc::c_int != 2818 as libc::c_int {
        fatal_error(b"%s: %s\x00" as *const u8 as *const libc::c_char,
                    b"Wrong checksum, might be an old version\x00" as
                        *const u8 as *const libc::c_char, file_name);
    }
    fread(&mut new_book_node_count as *mut libc::c_int as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
          1 as libc::c_int as size_t, stream);
    set_allocation(new_book_node_count + 1000 as libc::c_int);
    i = 0 as libc::c_int;
    while i < new_book_node_count {
        fread(&mut (*node.offset(i as isize)).hash_val1 as *mut libc::c_int as
                  *mut libc::c_void,
              ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        fread(&mut (*node.offset(i as isize)).hash_val2 as *mut libc::c_int as
                  *mut libc::c_void,
              ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        fread(&mut (*node.offset(i as isize)).black_minimax_score as
                  *mut libc::c_short as *mut libc::c_void,
              ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        fread(&mut (*node.offset(i as isize)).white_minimax_score as
                  *mut libc::c_short as *mut libc::c_void,
              ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        fread(&mut (*node.offset(i as isize)).best_alternative_move as
                  *mut libc::c_short as *mut libc::c_void,
              ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        fread(&mut (*node.offset(i as isize)).alternative_score as
                  *mut libc::c_short as *mut libc::c_void,
              ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        fread(&mut (*node.offset(i as isize)).flags as *mut libc::c_ushort as
                  *mut libc::c_void,
              ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        i += 1
    }
    fclose(stream);
    book_node_count = new_book_node_count;
    create_hash_reference();
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const libc::c_char,
           (stop_time - start_time) as libc::c_int);
}
/*
   MERGE_BINARY_DATABASE
   Merges a binary database file with the current book.
*/

pub unsafe fn merge_binary_database(mut file_name:
                                                   *const libc::c_char) {
    let mut start_time: time_t = 0;
    time(&mut start_time);
    printf(b"Importing binary opening database... \x00" as *const u8 as
               *const libc::c_char);
    fflush(stdout);
    let mut stream =
        fopen(file_name, b"rb\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                    b"Could not open database file\x00" as *const u8 as
                        *const libc::c_char, file_name);
    }
    let mut magic1: libc::c_short = 0;
    let mut magic2: libc::c_short = 0;
    fread(&mut magic1 as *mut libc::c_short as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
          1 as libc::c_int as size_t, stream);
    fread(&mut magic2 as *mut libc::c_short as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
          1 as libc::c_int as size_t, stream);
    if magic1 as libc::c_int != 2718 as libc::c_int ||
           magic2 as libc::c_int != 2818 as libc::c_int {
        fatal_error(b"%s: %s\x00" as *const u8 as *const libc::c_char,
                    b"Wrong checksum, might be an old version\x00" as
                        *const u8 as *const libc::c_char, file_name);
    }
    let mut merge_book_node_count: libc::c_int = 0;
    fread(&mut merge_book_node_count as *mut libc::c_int as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
          1 as libc::c_int as size_t, stream);
    let mut merge_use_count = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < merge_book_node_count {
        let mut merge_node =
            BookNode{hash_val1: 0,
                     hash_val2: 0,
                     black_minimax_score: 0,
                     white_minimax_score: 0,
                     best_alternative_move: 0,
                     alternative_score: 0,
                     flags: 0,};
        /* Read node. */
        fread(&mut merge_node.hash_val1 as *mut libc::c_int as
                  *mut libc::c_void,
              ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        fread(&mut merge_node.hash_val2 as *mut libc::c_int as
                  *mut libc::c_void,
              ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        fread(&mut merge_node.black_minimax_score as *mut libc::c_short as
                  *mut libc::c_void,
              ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        fread(&mut merge_node.white_minimax_score as *mut libc::c_short as
                  *mut libc::c_void,
              ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        fread(&mut merge_node.best_alternative_move as *mut libc::c_short as
                  *mut libc::c_void,
              ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        fread(&mut merge_node.alternative_score as *mut libc::c_short as
                  *mut libc::c_void,
              ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        fread(&mut merge_node.flags as *mut libc::c_ushort as
                  *mut libc::c_void,
              ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        /* Look up node in existing database. */
        let mut slot =
            probe_hash_table(merge_node.hash_val1, merge_node.hash_val2);
        if slot == -(1 as libc::c_int) ||
               *book_hash_table.offset(slot as isize) == -(1 as libc::c_int) {
            /* New position, add it without modifications. */
            let mut this_node =
                create_BookNode(merge_node.hash_val1, merge_node.hash_val2,
                                merge_node.flags);
            *node.offset(this_node as isize) = merge_node;
            merge_use_count += 1
        } else {
            /* Existing position, use the book from the merge file if it contains
            better endgame information. */
            let mut index = *book_hash_table.offset(slot as isize);
            if merge_node.flags as libc::c_int & 16 as libc::c_int != 0 &&
                   (*node.offset(index as isize)).flags as libc::c_int &
                       16 as libc::c_int == 0 ||
                   merge_node.flags as libc::c_int & 4 as libc::c_int != 0 &&
                       (*node.offset(index as isize)).flags as libc::c_int &
                           4 as libc::c_int == 0 {
                *node.offset(index as isize) = merge_node;
                merge_use_count += 1
            }
        }
        i += 1
    }
    fclose(stream);
    /* Make sure the tree is in reasonably good shape after the merge. */
    minimax_tree();
    let mut stop_time: time_t = 0;
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const libc::c_char,
           (stop_time - start_time) as libc::c_int);
    printf(b"Used %d out of %d nodes from the merge file.\x00" as *const u8 as
               *const libc::c_char, merge_use_count, merge_book_node_count);
}
/*
   WRITE_TEXT_DATABASE
   Writes the database to an ASCII file.
*/

pub unsafe fn write_text_database(mut file_name:
                                                 *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut stream = 0 as *mut FILE;
    time(&mut start_time);
    printf(b"Writing text database... \x00" as *const u8 as
               *const libc::c_char);
    fflush(stdout);
    stream = fopen(file_name, b"w\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                    b"Could not create database file\x00" as *const u8 as
                        *const libc::c_char, file_name);
    }
    fprintf(stream, b"%d\n%d\n\x00" as *const u8 as *const libc::c_char,
            2718 as libc::c_int, 2818 as libc::c_int);
    fprintf(stream, b"%d\n\x00" as *const u8 as *const libc::c_char,
            book_node_count);
    i = 0 as libc::c_int;
    while i < book_node_count {
        fprintf(stream,
                b"%d %d %d %d %d %d %d\n\x00" as *const u8 as
                    *const libc::c_char, (*node.offset(i as isize)).hash_val1,
                (*node.offset(i as isize)).hash_val2,
                (*node.offset(i as isize)).black_minimax_score as libc::c_int,
                (*node.offset(i as isize)).white_minimax_score as libc::c_int,
                (*node.offset(i as isize)).best_alternative_move as
                    libc::c_int,
                (*node.offset(i as isize)).alternative_score as libc::c_int,
                (*node.offset(i as isize)).flags as libc::c_int);
        i += 1
    }
    fclose(stream);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const libc::c_char,
           (stop_time - start_time) as libc::c_int);
    puts(b"\x00" as *const u8 as *const libc::c_char);
}
/*
   WRITE_BINARY_DATABASE
   Writes the database to a binary file.
*/

pub unsafe fn write_binary_database(mut file_name:
                                                   *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut magic: libc::c_short = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut stream = 0 as *mut FILE;
    time(&mut start_time);
    printf(b"Writing binary database... \x00" as *const u8 as
               *const libc::c_char);
    fflush(stdout);
    stream = fopen(file_name, b"wb\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                    b"Could not create database file\x00" as *const u8 as
                        *const libc::c_char, file_name);
    }
    magic = 2718 as libc::c_int as libc::c_short;
    fwrite(&mut magic as *mut libc::c_short as *const libc::c_void,
           ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    magic = 2818 as libc::c_int as libc::c_short;
    fwrite(&mut magic as *mut libc::c_short as *const libc::c_void,
           ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    fwrite(&mut book_node_count as *mut libc::c_int as *const libc::c_void,
           ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    i = 0 as libc::c_int;
    while i < book_node_count {
        fwrite(&mut (*node.offset(i as isize)).hash_val1 as *mut libc::c_int
                   as *const libc::c_void,
               ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
               1 as libc::c_int as size_t, stream);
        fwrite(&mut (*node.offset(i as isize)).hash_val2 as *mut libc::c_int
                   as *const libc::c_void,
               ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
               1 as libc::c_int as size_t, stream);
        fwrite(&mut (*node.offset(i as isize)).black_minimax_score as
                   *mut libc::c_short as *const libc::c_void,
               ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
               1 as libc::c_int as size_t, stream);
        fwrite(&mut (*node.offset(i as isize)).white_minimax_score as
                   *mut libc::c_short as *const libc::c_void,
               ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
               1 as libc::c_int as size_t, stream);
        fwrite(&mut (*node.offset(i as isize)).best_alternative_move as
                   *mut libc::c_short as *const libc::c_void,
               ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
               1 as libc::c_int as size_t, stream);
        fwrite(&mut (*node.offset(i as isize)).alternative_score as
                   *mut libc::c_short as *const libc::c_void,
               ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
               1 as libc::c_int as size_t, stream);
        fwrite(&mut (*node.offset(i as isize)).flags as *mut libc::c_ushort as
                   *const libc::c_void,
               ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
               1 as libc::c_int as size_t, stream);
        i += 1
    }
    fclose(stream);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const libc::c_char,
           (stop_time - start_time) as libc::c_int);
    puts(b"\x00" as *const u8 as *const libc::c_char);
}
/*
   DO_COMPRESS
   Compresses the subtree below the current node.
*/
unsafe fn do_compress(mut index: libc::c_int,
                                 mut node_order: *mut libc::c_int,
                                 mut child_count: *mut libc::c_short,
                                 mut node_index: *mut libc::c_int,
                                 mut child_list: *mut libc::c_short,
                                 mut child_index: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut child: libc::c_int = 0;
    let mut valid_child_count: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut local_child_list: [libc::c_int; 64] = [0; 64];
    let mut this_move: libc::c_short = 0;
    let mut local_child_move: [libc::c_short; 64] = [0; 64];
    if (*node.offset(index as isize)).flags as libc::c_int & 8 as libc::c_int
           == 0 {
        return
    }
    *node_order.offset(*node_index as isize) = index;
    if (*node.offset(index as isize)).flags as libc::c_int & 1 as libc::c_int
           != 0 {
        side_to_move = 0 as libc::c_int
    } else { side_to_move = 2 as libc::c_int }
    valid_child_count = 0 as libc::c_int;
    generate_all(side_to_move);
    i = 0 as libc::c_int;
    while i < move_count[disks_played as usize] {
        this_move =
            move_list[disks_played as usize][i as usize] as libc::c_short;
        make_move(side_to_move, this_move as libc::c_int, 1 as libc::c_int);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as libc::c_int) &&
               (*node.offset(child as isize)).flags as libc::c_int &
                   8 as libc::c_int != 0 {
            j = 0 as libc::c_int;
            found = 0 as libc::c_int;
            while j < valid_child_count {
                if child == local_child_list[j as usize] {
                    found = 1 as libc::c_int
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
        unmake_move(side_to_move, this_move as libc::c_int);
        i += 1
    }
    *child_count.offset(*node_index as isize) =
        valid_child_count as libc::c_short;
    *node_index += 1;
    i = 0 as libc::c_int;
    while i < valid_child_count {
        this_move = local_child_move[i as usize];
        make_move(side_to_move, this_move as libc::c_int, 1 as libc::c_int);
        do_compress(local_child_list[i as usize], node_order, child_count,
                    node_index, child_list, child_index);
        unmake_move(side_to_move, this_move as libc::c_int);
        i += 1
    }
    let ref mut fresh44 = (*node.offset(index as isize)).flags;
    *fresh44 = (*fresh44 as libc::c_int ^ 8 as libc::c_int) as libc::c_ushort;
}
/*
   WRITE_COMPRESSED_DATABASE
   Creates and saves a compressed database file.
*/

pub unsafe fn write_compressed_database(mut file_name:
                                                       *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut node_index: libc::c_int = 0;
    let mut child_index: libc::c_int = 0;
    let mut node_order = 0 as *mut libc::c_int;
    let mut child_count = 0 as *mut libc::c_short;
    let mut child = 0 as *mut libc::c_short;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut stream = 0 as *mut FILE;
    time(&mut start_time);
    printf(b"Writing compressed database... \x00" as *const u8 as
               *const libc::c_char);
    fflush(stdout);
    stream = fopen(file_name, b"wb\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                    b"Could not create database file\x00" as *const u8 as
                        *const libc::c_char, file_name);
    }
    prepare_tree_traversal();
    node_order =
        safe_malloc((book_node_count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                         as libc::c_ulong)) as
            *mut libc::c_int;
    child_count =
        safe_malloc((book_node_count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                         as libc::c_ulong)) as
            *mut libc::c_short;
    child =
        malloc((book_node_count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                    as libc::c_ulong)) as
            *mut libc::c_short;
    i = 0 as libc::c_int;
    while i < book_node_count {
        let ref mut fresh45 = (*node.offset(i as isize)).flags;
        *fresh45 =
            (*fresh45 as libc::c_int | 8 as libc::c_int) as libc::c_ushort;
        i += 1
    }
    node_index = 0 as libc::c_int;
    child_index = 0 as libc::c_int;
    do_compress(0 as libc::c_int, node_order, child_count, &mut node_index,
                child, &mut child_index);
    fwrite(&mut book_node_count as *mut libc::c_int as *const libc::c_void,
           ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    fwrite(&mut child_index as *mut libc::c_int as *const libc::c_void,
           ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    fwrite(child_count as *const libc::c_void,
           ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
           book_node_count as size_t, stream);
    fwrite(child as *const libc::c_void,
           ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
           child_index as size_t, stream);
    i = 0 as libc::c_int;
    while i < book_node_count {
        fwrite(&mut (*node.offset(*node_order.offset(i as isize) as
                                      isize)).black_minimax_score as
                   *mut libc::c_short as *const libc::c_void,
               ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
               1 as libc::c_int as size_t, stream);
        fwrite(&mut (*node.offset(*node_order.offset(i as isize) as
                                      isize)).white_minimax_score as
                   *mut libc::c_short as *const libc::c_void,
               ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
               1 as libc::c_int as size_t, stream);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < book_node_count {
        fwrite(&mut (*node.offset(*node_order.offset(i as isize) as
                                      isize)).best_alternative_move as
                   *mut libc::c_short as *const libc::c_void,
               ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
               1 as libc::c_int as size_t, stream);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < book_node_count {
        fwrite(&mut (*node.offset(*node_order.offset(i as isize) as
                                      isize)).alternative_score as
                   *mut libc::c_short as *const libc::c_void,
               ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
               1 as libc::c_int as size_t, stream);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < book_node_count {
        fwrite(&mut (*node.offset(*node_order.offset(i as isize) as
                                      isize)).flags as *mut libc::c_ushort as
                   *const libc::c_void,
               ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
               1 as libc::c_int as size_t, stream);
        i += 1
    }
    fclose(stream);
    free(node_order as *mut libc::c_void);
    free(child_count as *mut libc::c_void);
    free(child as *mut libc::c_void);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const libc::c_char,
           (stop_time - start_time) as libc::c_int);
    puts(b"\x00" as *const u8 as *const libc::c_char);
}
/*
  DO_UNCOMPRESS
  Uncompress the subtree below the current node. This is done
  in preorder.
*/
unsafe fn do_uncompress(mut depth: libc::c_int,
                                   mut stream: *mut FILE,
                                   mut node_index: *mut libc::c_int,
                                   mut child_index: *mut libc::c_int,
                                   mut child_count: *mut libc::c_short,
                                   mut child: *mut libc::c_short,
                                   mut black_score: *mut libc::c_short,
                                   mut white_score: *mut libc::c_short,
                                   mut alt_move: *mut libc::c_short,
                                   mut alt_score: *mut libc::c_short,
                                   mut flags: *mut libc::c_ushort) {
    let mut i: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut saved_child_index: libc::c_int = 0;
    let mut saved_child_count: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    let mut this_move: libc::c_int = 0;
    if *flags.offset(*node_index as isize) as libc::c_int & 1 as libc::c_int
           != 0 {
        side_to_move = 0 as libc::c_int
    } else { side_to_move = 2 as libc::c_int }
    saved_child_count =
        *child_count.offset(*node_index as isize) as libc::c_int;
    saved_child_index = *child_index;
    *child_index += saved_child_count;
    /* Write the data for the current node */
    get_hash(&mut val1, &mut val2, &mut orientation);
    fwrite(&mut val1 as *mut libc::c_int as *const libc::c_void,
           ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    fwrite(&mut val2 as *mut libc::c_int as *const libc::c_void,
           ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    fwrite(&mut *black_score.offset(*node_index as isize) as
               *mut libc::c_short as *const libc::c_void,
           ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    fwrite(&mut *white_score.offset(*node_index as isize) as
               *mut libc::c_short as *const libc::c_void,
           ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    fwrite(&mut *alt_move.offset(*node_index as isize) as *mut libc::c_short
               as *const libc::c_void,
           ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    fwrite(&mut *alt_score.offset(*node_index as isize) as *mut libc::c_short
               as *const libc::c_void,
           ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    fwrite(&mut *flags.offset(*node_index as isize) as *mut libc::c_ushort as
               *const libc::c_void,
           ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    *node_index += 1;
    /* Recursively traverse the children */
    i = 0 as libc::c_int;
    while i < saved_child_count {
        let mut flipped: libc::c_int = 0;
        this_move =
            *child.offset((saved_child_index + i) as isize) as libc::c_int;
        flipped = make_move_no_hash(side_to_move, this_move);
        if flipped == 0 as libc::c_int {
            printf(b"%c%c flips %d discs for %d\n\x00" as *const u8 as
                       *const libc::c_char,
                   'a' as i32 + this_move % 10 as libc::c_int -
                       1 as libc::c_int,
                   '0' as i32 + this_move / 10 as libc::c_int, flipped,
                   side_to_move);
        }
        do_uncompress(depth + 1 as libc::c_int, stream, node_index,
                      child_index, child_count, child, black_score,
                      white_score, alt_move, alt_score, flags);
        unmake_move_no_hash(side_to_move, this_move);
        i += 1
    };
}
/*
  UNPACK_COMPRESSED_DATABASE
  Reads a database compressed with WRITE_COMPRESSED_DATABASE
  and unpacks it into an ordinary .bin file.
*/

pub unsafe fn unpack_compressed_database(mut in_name:
                                                        *const libc::c_char,
                                                    mut out_name:
                                                        *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut dummy: libc::c_int = 0;
    let mut node_count: libc::c_int = 0;
    let mut child_list_size: libc::c_int = 0;
    let mut node_index: libc::c_int = 0;
    let mut child_index: libc::c_int = 0;
    let mut magic: libc::c_short = 0;
    let mut child_count = 0 as *mut libc::c_short;
    let mut child = 0 as *mut libc::c_short;
    let mut black_score = 0 as *mut libc::c_short;
    let mut white_score = 0 as *mut libc::c_short;
    let mut alt_move = 0 as *mut libc::c_short;
    let mut alt_score = 0 as *mut libc::c_short;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut flags = 0 as *mut libc::c_ushort;
    let mut stream = 0 as *mut FILE;
    printf(b"Uncompressing compressed database... \x00" as *const u8 as
               *const libc::c_char);
    fflush(stdout);
    time(&mut start_time);
    /* Read the compressed database */
    stream = fopen(in_name, b"rb\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                    b"Could not open database file\x00" as *const u8 as
                        *const libc::c_char, in_name);
    }
    fread(&mut node_count as *mut libc::c_int as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
          1 as libc::c_int as size_t, stream);
    fread(&mut child_list_size as *mut libc::c_int as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
          1 as libc::c_int as size_t, stream);
    child_count =
        safe_malloc((node_count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                         as libc::c_ulong)) as
            *mut libc::c_short;
    child =
        safe_malloc((child_list_size as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                         as libc::c_ulong)) as
            *mut libc::c_short;
    fread(child_count as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
          node_count as size_t, stream);
    fread(child as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
          child_list_size as size_t, stream);
    black_score =
        safe_malloc((node_count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                         as libc::c_ulong)) as
            *mut libc::c_short;
    white_score =
        safe_malloc((node_count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                         as libc::c_ulong)) as
            *mut libc::c_short;
    alt_move =
        safe_malloc((node_count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                         as libc::c_ulong)) as
            *mut libc::c_short;
    alt_score =
        safe_malloc((node_count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                         as libc::c_ulong)) as
            *mut libc::c_short;
    flags =
        safe_malloc((node_count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_ushort>()
                                                         as libc::c_ulong)) as
            *mut libc::c_ushort;
    i = 0 as libc::c_int;
    while i < node_count {
        fread(&mut *black_score.offset(i as isize) as *mut libc::c_short as
                  *mut libc::c_void,
              ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        fread(&mut *white_score.offset(i as isize) as *mut libc::c_short as
                  *mut libc::c_void,
              ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream);
        i += 1
    }
    fread(alt_move as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
          node_count as size_t, stream);
    fread(alt_score as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
          node_count as size_t, stream);
    fread(flags as *mut libc::c_void,
          ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
          node_count as size_t, stream);
    fclose(stream);
    /* Traverse the tree described by the database and create the .bin file */
    stream = fopen(out_name, b"wb\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                    b"Could not create database file\x00" as *const u8 as
                        *const libc::c_char, out_name);
    }
    toggle_experimental(0 as libc::c_int);
    game_init(0 as *const libc::c_char, &mut dummy);
    toggle_midgame_hash_usage(1 as libc::c_int, 1 as libc::c_int);
    toggle_abort_check(0 as libc::c_int);
    toggle_midgame_abort_check(0 as libc::c_int);
    magic = 2718 as libc::c_int as libc::c_short;
    fwrite(&mut magic as *mut libc::c_short as *const libc::c_void,
           ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    magic = 2818 as libc::c_int as libc::c_short;
    fwrite(&mut magic as *mut libc::c_short as *const libc::c_void,
           ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    fwrite(&mut node_count as *mut libc::c_int as *const libc::c_void,
           ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
           1 as libc::c_int as size_t, stream);
    node_index = 0 as libc::c_int;
    child_index = 0 as libc::c_int;
    do_uncompress(0 as libc::c_int, stream, &mut node_index, &mut child_index,
                  child_count, child, black_score, white_score, alt_move,
                  alt_score, flags);
    fclose(stream);
    /* Free tables */
    free(child_count as *mut libc::c_void);
    free(child as *mut libc::c_void);
    free(black_score as *mut libc::c_void);
    free(white_score as *mut libc::c_void);
    free(alt_move as *mut libc::c_void);
    free(alt_score as *mut libc::c_void);
    free(flags as *mut libc::c_void);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const libc::c_char,
           (stop_time - start_time) as libc::c_int);
    puts(b"\x00" as *const u8 as *const libc::c_char);
}
/*
   SET_SEARCH_DEPTH
   When finding move alternatives, searches to depth DEPTH
   will be performed.
*/

pub unsafe fn set_search_depth(mut depth: libc::c_int) {
    search_depth = depth;
}
/*
  SET_EVAL_SPAN
  Specify the evaluation value interval where nodes are re-evaluated.
*/

pub unsafe fn set_eval_span(mut min_span: libc::c_double,
                                       mut max_span: libc::c_double) {
    min_eval_span = ceil(min_span * 128.0f64) as libc::c_int;
    max_eval_span = ceil(max_span * 128.0f64) as libc::c_int;
}
/*
  SET_NEGAMAX_SPAN
  Specify the negamax value interval where nodes are re-evaluated.
*/

pub unsafe fn set_negamax_span(mut min_span: libc::c_double,
                                          mut max_span: libc::c_double) {
    min_negamax_span = ceil(min_span * 128.0f64) as libc::c_int;
    max_negamax_span = ceil(max_span * 128.0f64) as libc::c_int;
}
/*
  SET_MAX_BATCH_SIZE
  Specify the maximum number of nodes to evaluate.
*/

pub unsafe fn set_max_batch_size(mut size: libc::c_int) {
    max_batch_size = size;
}
/*
   SET_DEVIATION_VALUE
   Sets the number of disks where a penalty is incurred if
   the deviation from the book line comes later than that
   stage; also set the punishment per move after the threshold.
*/

pub unsafe fn set_deviation_value(mut low_threshold: libc::c_int,
                                             mut high_threshold: libc::c_int,
                                             mut bonus: libc::c_double) {
    low_deviation_threshold = low_threshold;
    high_deviation_threshold = high_threshold;
    deviation_bonus = bonus;
}
/*
   RESET_BOOK_SEARCH
   Sets the used slack count to zero.
*/

pub unsafe fn reset_book_search() {
    used_slack[0 as libc::c_int as usize] = 0.0f64 as libc::c_int;
    used_slack[2 as libc::c_int as usize] = 0.0f64 as libc::c_int;
}
/*
   SET_SLACK
   Sets the total amount of negamaxed evaluation that
   the program is willing to trade for randomness.
*/

pub unsafe fn set_slack(mut slack: libc::c_int) {
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

pub unsafe fn set_black_force(mut force: libc::c_int) {
    force_black = force;
}

pub unsafe fn set_white_force(mut force: libc::c_int) {
    force_white = force;
}
/*
  MERGE_POSITION_LIST
  Adds the scores from the positions defined in SCRIPT_FILE and solved
  in OUTPUT_FILE to the book.  The two files are checked for sanity -
  if they don't describe the same set of positions, something has gone awry.
*/

pub unsafe fn merge_position_list(mut script_file:
                                                 *const libc::c_char,
                                             mut output_file:
                                                 *const libc::c_char) {
    let mut script_buffer: [libc::c_char; 1024] = [0; 1024];
    let mut result_buffer: [libc::c_char; 1024] = [0; 1024];
    let mut move_buffer: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut line: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut move_0: libc::c_int = 0;
    let mut wld_only: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut position_count: libc::c_int = 0;
    let mut already_wld_count: libc::c_int = 0;
    let mut already_exact_count: libc::c_int = 0;
    let mut tokens_read: libc::c_int = 0;
    let mut moves_read: libc::c_int = 0;
    let mut new_nodes_created: libc::c_int = 0;
    let mut probable_error: libc::c_int = 0;
    let mut script_stream = 0 as *mut FILE;
    let mut result_stream = 0 as *mut FILE;
    script_stream =
        fopen(script_file, b"r\x00" as *const u8 as *const libc::c_char);
    if script_stream.is_null() {
        fprintf(stderr,
                b"Can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                script_file);
        exit(1 as libc::c_int);
    }
    result_stream =
        fopen(output_file, b"r\x00" as *const u8 as *const libc::c_char);
    if result_stream.is_null() {
        fprintf(stderr,
                b"Can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                output_file);
        exit(1 as libc::c_int);
    }
    prepare_tree_traversal();
    line = 1 as libc::c_int;
    position_count = 0 as libc::c_int;
    already_wld_count = 0 as libc::c_int;
    already_exact_count = 0 as libc::c_int;
    new_nodes_created = 0 as libc::c_int;
    fgets(script_buffer.as_mut_ptr(), 1024 as libc::c_int, script_stream);
    fgets(result_buffer.as_mut_ptr(), 1024 as libc::c_int, result_stream);
    while feof(script_stream) == 0 && feof(result_stream) == 0 {
        let mut ch = 0 as *mut libc::c_char;
        ch =
            script_buffer.as_mut_ptr().offset(strlen(script_buffer.as_mut_ptr())
                                                  as
                                                  isize).offset(-(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize));
        while ch >= script_buffer.as_mut_ptr() &&
                  *(*__ctype_b_loc()).offset(*ch as libc::c_int as isize) as
                      libc::c_int &
                      _ISgraph as libc::c_int as libc::c_ushort as libc::c_int
                      == 0 {
            *ch = 0 as libc::c_int as libc::c_char;
            ch = ch.offset(-1)
        }
        ch =
            result_buffer.as_mut_ptr().offset(strlen(result_buffer.as_mut_ptr())
                                                  as
                                                  isize).offset(-(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize));
        while ch >= result_buffer.as_mut_ptr() &&
                  *(*__ctype_b_loc()).offset(*ch as libc::c_int as isize) as
                      libc::c_int &
                      _ISgraph as libc::c_int as libc::c_ushort as libc::c_int
                      == 0 {
            *ch = 0 as libc::c_int as libc::c_char;
            ch = ch.offset(-1)
        }
        if line % 4 as libc::c_int == 3 as libc::c_int {
            /* The position/result lines */
            position_count += 1;
            /* Parse the board */
            disks_played =
                0 as libc::c_int; /* The initial board contains 4 discs */
            col = 0 as libc::c_int;
            i = 1 as libc::c_int;
            while i <= 8 as libc::c_int {
                j = 1 as libc::c_int;
                while j <= 8 as libc::c_int {
                    pos = 10 as libc::c_int * i + j;
                    match script_buffer[col as usize] as libc::c_int {
                        42 | 88 | 120 => {
                            board[pos as usize] = 0 as libc::c_int;
                            disks_played += 1
                        }
                        79 | 48 | 111 => {
                            board[pos as usize] = 2 as libc::c_int;
                            disks_played += 1
                        }
                        45 | 46 => { board[pos as usize] = 1 as libc::c_int }
                        _ => {
                            fprintf(stderr,
                                    b"\nBad character \'%c\' in board on line %d\n\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    script_buffer[col as usize] as
                                        libc::c_int, line);
                            exit(1 as libc::c_int);
                        }
                    }
                    col += 1;
                    j += 1
                }
                i += 1
            }
            match script_buffer[65 as libc::c_int as usize] as libc::c_int {
                42 | 88 | 120 => { side_to_move = 0 as libc::c_int }
                79 | 48 | 111 => { side_to_move = 2 as libc::c_int }
                _ => {
                    fprintf(stderr,
                            b"\nBad side to move \'%c\' in board on line %d\n\n\x00"
                                as *const u8 as *const libc::c_char,
                            script_buffer[65 as libc::c_int as usize] as
                                libc::c_int, line);
                    exit(1 as libc::c_int);
                }
            }
            disks_played -= 4 as libc::c_int;
            /* Parse the result */
            wld_only = 1 as libc::c_int;
            if strstr(result_buffer.as_mut_ptr(),
                      b"Black win\x00" as *const u8 as *const libc::c_char) ==
                   result_buffer.as_mut_ptr() {
                score = 30000 as libc::c_int + 2 as libc::c_int;
                tokens_read =
                    sscanf(result_buffer.as_mut_ptr(),
                           b"%*s %*s %s\x00" as *const u8 as
                               *const libc::c_char, move_buffer.as_mut_ptr());
                moves_read = tokens_read
            } else if strstr(result_buffer.as_mut_ptr(),
                             b"White win\x00" as *const u8 as
                                 *const libc::c_char) ==
                          result_buffer.as_mut_ptr() {
                score = -(30000 as libc::c_int + 2 as libc::c_int);
                tokens_read =
                    sscanf(result_buffer.as_mut_ptr(),
                           b"%*s %*s %s\x00" as *const u8 as
                               *const libc::c_char, move_buffer.as_mut_ptr());
                moves_read = tokens_read
            } else if strstr(result_buffer.as_mut_ptr(),
                             b"Draw\x00" as *const u8 as *const libc::c_char)
                          == result_buffer.as_mut_ptr() {
                score = 0 as libc::c_int;
                tokens_read =
                    sscanf(result_buffer.as_mut_ptr(),
                           b"%*s %s\x00" as *const u8 as *const libc::c_char,
                           move_buffer.as_mut_ptr());
                moves_read = tokens_read
            } else {
                /* Exact score */
                let mut black_discs: libc::c_int = 0;
                let mut white_discs: libc::c_int = 0;
                wld_only = 0 as libc::c_int;
                tokens_read =
                    sscanf(result_buffer.as_mut_ptr(),
                           b"%d %*s %d %s\x00" as *const u8 as
                               *const libc::c_char,
                           &mut black_discs as *mut libc::c_int,
                           &mut white_discs as *mut libc::c_int,
                           move_buffer.as_mut_ptr());
                moves_read = tokens_read - 2 as libc::c_int;
                score = black_discs - white_discs;
                if score > 0 as libc::c_int {
                    score += 30000 as libc::c_int
                } else if score < 0 as libc::c_int {
                    score -= 30000 as libc::c_int
                }
            }
            /* Set the score for the node corresponding to the position */
            get_hash(&mut val1, &mut val2, &mut orientation);
            slot = probe_hash_table(val1, val2);
            index = *book_hash_table.offset(slot as isize);
            if index == -(1 as libc::c_int) {
                fprintf(stderr,
                        b"Position on line %d not found in book\n\x00" as
                            *const u8 as *const libc::c_char, line);
                exit(0 as libc::c_int);
            }
            probable_error = 0 as libc::c_int;
            if (*node.offset(index as isize)).flags as libc::c_int &
                   4 as libc::c_int != 0 {
                already_wld_count += 1;
                if score > 0 as libc::c_int &&
                       (*node.offset(index as isize)).black_minimax_score as
                           libc::c_int <= 0 as libc::c_int ||
                       score == 0 as libc::c_int &&
                           (*node.offset(index as isize)).black_minimax_score
                               as libc::c_int != 0 as libc::c_int ||
                       score < 0 as libc::c_int &&
                           (*node.offset(index as isize)).black_minimax_score
                               as libc::c_int > 0 as libc::c_int {
                    probable_error = 1 as libc::c_int;
                    fprintf(stderr,
                            b"Line %d: New WLD score %d conflicts with old score %d\n\x00"
                                as *const u8 as *const libc::c_char, line,
                            score,
                            (*node.offset(index as isize)).black_minimax_score
                                as libc::c_int);
                }
            }
            if (*node.offset(index as isize)).flags as libc::c_int &
                   16 as libc::c_int != 0 {
                already_exact_count += 1;
                if wld_only == 0 &&
                       score !=
                           (*node.offset(index as isize)).black_minimax_score
                               as libc::c_int {
                    probable_error = 1 as libc::c_int;
                    fprintf(stderr,
                            b"Line %d: New exact score %d conflicts with old score %d\n\x00"
                                as *const u8 as *const libc::c_char, line,
                            score,
                            (*node.offset(index as isize)).black_minimax_score
                                as libc::c_int);
                }
            }
            if probable_error != 0 || wld_only == 0 ||
                   (*node.offset(index as isize)).flags as libc::c_int &
                       16 as libc::c_int == 0 {
                let ref mut fresh46 =
                    (*node.offset(index as isize)).white_minimax_score;
                *fresh46 = score as libc::c_short;
                (*node.offset(index as isize)).black_minimax_score = *fresh46
            }
            if probable_error != 0 {
                /* Clear the old flags if score was wrong */
                let ref mut fresh47 = (*node.offset(index as isize)).flags;
                *fresh47 =
                    (*fresh47 as libc::c_int &
                         !(4 as libc::c_int | 16 as libc::c_int)) as
                        libc::c_ushort
            }
            if wld_only != 0 {
                let ref mut fresh48 = (*node.offset(index as isize)).flags;
                *fresh48 =
                    (*fresh48 as libc::c_int | 4 as libc::c_int) as
                        libc::c_ushort
            } else {
                let ref mut fresh49 = (*node.offset(index as isize)).flags;
                *fresh49 =
                    (*fresh49 as libc::c_int |
                         (4 as libc::c_int | 16 as libc::c_int)) as
                        libc::c_ushort
            }
            /* Examine the position arising from the PV move; if it exists it
            need only be checked for sanity, otherwise a new node is
             created. */
            if moves_read > 0 as libc::c_int {
                /* Make sure the optimal move leads to a position in the hash table */
                let mut row: libc::c_int = 0;
                let mut col_0: libc::c_int = 0;
                row =
                    move_buffer[1 as libc::c_int as usize] as libc::c_int -
                        '0' as i32;
                col_0 =
                    tolower(move_buffer[0 as libc::c_int as usize] as
                                libc::c_int) - 'a' as i32 + 1 as libc::c_int;
                move_0 = 10 as libc::c_int * row + col_0;
                if row >= 1 as libc::c_int && row <= 8 as libc::c_int &&
                       col_0 >= 1 as libc::c_int && col_0 <= 8 as libc::c_int
                       && make_move_no_hash(side_to_move, move_0) != 0 {
                    let mut new_side_to_move =
                        0 as libc::c_int + 2 as libc::c_int - side_to_move;
                    generate_all(new_side_to_move);
                    if move_count[disks_played as usize] == 0 as libc::c_int {
                        new_side_to_move = side_to_move
                    }
                    get_hash(&mut val1, &mut val2, &mut orientation);
                    slot = probe_hash_table(val1, val2);
                    index = *book_hash_table.offset(slot as isize);
                    if index == -(1 as libc::c_int) {
                        index =
                            create_BookNode(val1, val2,
                                            32 as libc::c_int as
                                                libc::c_ushort);
                        let ref mut fresh50 =
                            (*node.offset(index as
                                              isize)).white_minimax_score;
                        *fresh50 = score as libc::c_short;
                        (*node.offset(index as isize)).black_minimax_score =
                            *fresh50;
                        if new_side_to_move == 0 as libc::c_int {
                            let ref mut fresh51 =
                                (*node.offset(index as isize)).flags;
                            *fresh51 =
                                (*fresh51 as libc::c_int | 1 as libc::c_int)
                                    as libc::c_ushort
                        } else {
                            let ref mut fresh52 =
                                (*node.offset(index as isize)).flags;
                            *fresh52 =
                                (*fresh52 as libc::c_int | 2 as libc::c_int)
                                    as libc::c_ushort
                        }
                        if wld_only != 0 {
                            let ref mut fresh53 =
                                (*node.offset(index as isize)).flags;
                            *fresh53 =
                                (*fresh53 as libc::c_int | 4 as libc::c_int)
                                    as libc::c_ushort
                        } else {
                            let ref mut fresh54 =
                                (*node.offset(index as isize)).flags;
                            *fresh54 =
                                (*fresh54 as libc::c_int |
                                     (4 as libc::c_int | 16 as libc::c_int))
                                    as libc::c_ushort
                        }
                        new_nodes_created += 1
                    } else {
                        /* Position already exists, sanity-check it */
                        probable_error = 0 as libc::c_int;
                        if (*node.offset(index as isize)).flags as libc::c_int
                               & 4 as libc::c_int != 0 {
                            if score > 0 as libc::c_int &&
                                   (*node.offset(index as
                                                     isize)).black_minimax_score
                                       as libc::c_int <= 0 as libc::c_int ||
                                   score == 0 as libc::c_int &&
                                       (*node.offset(index as
                                                         isize)).black_minimax_score
                                           as libc::c_int != 0 as libc::c_int
                                   ||
                                   score < 0 as libc::c_int &&
                                       (*node.offset(index as
                                                         isize)).black_minimax_score
                                           as libc::c_int > 0 as libc::c_int {
                                probable_error = 1 as libc::c_int;
                                fprintf(stderr,
                                        b"Line %d: New child WLD score %d conflicts with old score %d\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, line, score,
                                        (*node.offset(index as
                                                          isize)).black_minimax_score
                                            as libc::c_int);
                            }
                        }
                        if (*node.offset(index as isize)).flags as libc::c_int
                               & 16 as libc::c_int != 0 {
                            if wld_only == 0 &&
                                   score !=
                                       (*node.offset(index as
                                                         isize)).black_minimax_score
                                           as libc::c_int {
                                probable_error = 1 as libc::c_int;
                                fprintf(stderr,
                                        b"Line %d: New child exact score %d conflicts with old score %d\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, line, score,
                                        (*node.offset(index as
                                                          isize)).black_minimax_score
                                            as libc::c_int);
                            }
                        }
                        if probable_error != 0 {
                            /* Correct errors encountered */
                            let ref mut fresh55 =
                                (*node.offset(index as
                                                  isize)).white_minimax_score;
                            *fresh55 = score as libc::c_short;
                            (*node.offset(index as isize)).black_minimax_score
                                = *fresh55;
                            let ref mut fresh56 =
                                (*node.offset(index as isize)).flags;
                            *fresh56 =
                                (*fresh56 as libc::c_int &
                                     !(4 as libc::c_int | 16 as libc::c_int))
                                    as libc::c_ushort;
                            if wld_only != 0 {
                                let ref mut fresh57 =
                                    (*node.offset(index as isize)).flags;
                                *fresh57 =
                                    (*fresh57 as libc::c_int |
                                         4 as libc::c_int) as libc::c_ushort
                            } else {
                                let ref mut fresh58 =
                                    (*node.offset(index as isize)).flags;
                                *fresh58 =
                                    (*fresh58 as libc::c_int |
                                         (4 as libc::c_int |
                                              16 as libc::c_int)) as
                                        libc::c_ushort
                            }
                        }
                    }
                    unmake_move_no_hash(side_to_move, move_0);
                } else {
                    fprintf(stderr,
                            b"Line %d: The PV move \'%s\' is invalid\n\x00" as
                                *const u8 as *const libc::c_char, line,
                            move_buffer.as_mut_ptr());
                    exit(1 as libc::c_int);
                }
            }
        } else if strcmp(script_buffer.as_mut_ptr(),
                         result_buffer.as_mut_ptr()) != 0 {
            fprintf(stderr,
                    b"Script and result files differ unexpectedly on line %d\n\x00"
                        as *const u8 as *const libc::c_char, line);
            exit(1 as libc::c_int);
        }
        fgets(script_buffer.as_mut_ptr(), 1024 as libc::c_int, script_stream);
        fgets(result_buffer.as_mut_ptr(), 1024 as libc::c_int, result_stream);
        line += 1
    }
    line -= 1;
    printf(b"%d lines read from the script and result files\n\x00" as
               *const u8 as *const libc::c_char, line);
    if feof(script_stream) == 0 || feof(result_stream) == 0 {
        puts(b"Warning: The two files don\'t have the same number of lines.\x00"
                 as *const u8 as *const libc::c_char);
    }
    printf(b"%d positions merged with the book\n\x00" as *const u8 as
               *const libc::c_char, position_count);
    printf(b"%d positions were already solved for exact score\n\x00" as
               *const u8 as *const libc::c_char, already_exact_count);
    printf(b"%d positions were already solved WLD\n\x00" as *const u8 as
               *const libc::c_char, already_wld_count);
    printf(b"%d positions had optimal moves leading to new positions\n\x00" as
               *const u8 as *const libc::c_char, new_nodes_created);
    puts(b"\x00" as *const u8 as *const libc::c_char);
    fclose(script_stream);
    fclose(result_stream);
}
/*
  CHECK_FORCED_OPENING
  Checks if the board position fits the provided forced opening line OPENING
  in any rotation; if this is the case, the next move is returned,
  otherwise PASS is returned.
*/

pub unsafe fn check_forced_opening(mut side_to_move: libc::c_int,
                                              mut opening:
                                                  *const libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut move_count_0: libc::c_int = 0;
    let mut local_side_to_move: libc::c_int = 0;
    let mut same_position: libc::c_int = 0;
    let mut symm_index: libc::c_int = 0;
    let mut symmetry: libc::c_int = 0;
    let mut move_0: [libc::c_int; 60] = [0; 60];
    let mut local_board: [libc::c_int; 100] = [0; 100];
    let mut move_offset: [libc::c_int; 8] =
        [1 as libc::c_int, -(1 as libc::c_int), 9 as libc::c_int,
         -(9 as libc::c_int), 10 as libc::c_int, -(10 as libc::c_int),
         11 as libc::c_int, -(11 as libc::c_int)];
    move_count_0 =
        strlen(opening).wrapping_div(2 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    if move_count_0 <= disks_played { return -(1 as libc::c_int) }
    i = 0 as libc::c_int;
    while i < move_count_0 {
        move_0[i as usize] =
            10 as libc::c_int *
                (*opening.offset((2 as libc::c_int * i + 1 as libc::c_int) as
                                     isize) as libc::c_int - '0' as i32) +
                tolower(*opening.offset((2 as libc::c_int * i) as isize) as
                            libc::c_int) - 'a' as i32 + 1 as libc::c_int;
        i += 1
    }
    /* Play through the given opening line until the number of discs
       matches that on the actual board. */
    pos = 11 as libc::c_int;
    while pos <= 88 as libc::c_int {
        local_board[pos as usize] = 1 as libc::c_int;
        pos += 1
    }
    local_board[54 as libc::c_int as usize] = 0 as libc::c_int;
    local_board[45 as libc::c_int as usize] =
        local_board[54 as libc::c_int as usize];
    local_board[55 as libc::c_int as usize] = 2 as libc::c_int;
    local_board[44 as libc::c_int as usize] =
        local_board[55 as libc::c_int as usize];
    local_side_to_move = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < disks_played {
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            pos = move_0[i as usize] + move_offset[j as usize];
            count = 0 as libc::c_int;
            while local_board[pos as usize] ==
                      0 as libc::c_int + 2 as libc::c_int - local_side_to_move
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
            0 as libc::c_int + 2 as libc::c_int - local_side_to_move;
        i += 1
    }
    if local_side_to_move != side_to_move { return -(1 as libc::c_int) }
    /* Check if any of the 8 symmetries make the board after the opening
       line match the current board. The initial symmetry is chosen
       randomly to avoid the same symmetry being chosen all the time.
       This is not a perfect scheme but good enough. */
    symmetry = abs(my_random() as libc::c_int) % 8 as libc::c_int;
    symm_index = 0 as libc::c_int;
    while symm_index < 8 as libc::c_int {
        same_position = 1 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= 8 as libc::c_int && same_position != 0 {
            j = 1 as libc::c_int;
            while j <= 8 as libc::c_int {
                pos = 10 as libc::c_int * i + j;
                if board[pos as usize] !=
                       local_board[*symmetry_map[symmetry as
                                                     usize].offset(pos as
                                                                       isize)
                                       as usize] {
                    same_position = 0 as libc::c_int
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
        symmetry = (symmetry + 1 as libc::c_int) % 8 as libc::c_int
    }
    return -(1 as libc::c_int);
}
/*
  FILL_MOVE_ALTERNATIVES
  Fills the data structure CANDIDATE_LIST with information
  about the book moves available in the current position.
  FLAGS specifies a subset of the flag bits which have to be set
  for a position to be considered. Notice that FLAGS=0 accepts
  any flag combination.
*/

pub unsafe fn fill_move_alternatives(mut side_to_move: libc::c_int,
                                                mut flags: libc::c_int) {
    let mut temp =
        CandidateMove{move_0: 0, score: 0, flags: 0, parent_flags: 0,};
    let mut sign: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut changed: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    let mut this_move: libc::c_int = 0;
    let mut alternative_move: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut alternative_score: libc::c_int = 0;
    let mut child_feasible: libc::c_int = 0;
    let mut deviation: libc::c_int = 0;
    let mut root_flags: libc::c_int = 0;
    get_hash(&mut val1, &mut val2, &mut orientation);
    slot = probe_hash_table(val1, val2);
    /* If the position wasn't found in the hash table, return. */
    if slot == -(1 as libc::c_int) ||
           *book_hash_table.offset(slot as isize) == -(1 as libc::c_int) {
        candidate_count = 0 as libc::c_int;
        return
    } else { index = *book_hash_table.offset(slot as isize) }
    /* If the position hasn't got the right flag bits set, return. */
    root_flags = (*node.offset(index as isize)).flags as libc::c_int;
    if flags != 0 as libc::c_int && root_flags & flags == 0 {
        candidate_count = 0 as libc::c_int;
        return
    }
    if side_to_move == 0 as libc::c_int {
        sign = 1 as libc::c_int
    } else { sign = -(1 as libc::c_int) }
    alternative_move =
        (*node.offset(index as isize)).best_alternative_move as libc::c_int;
    if alternative_move > 0 as libc::c_int {
        alternative_move =
            *inv_symmetry_map[orientation as
                                  usize].offset(alternative_move as isize);
        alternative_score =
            adjust_score((*node.offset(index as isize)).alternative_score as
                             libc::c_int, side_to_move)
    } else { alternative_score = -(12345678 as libc::c_int) }
    generate_all(side_to_move);
    candidate_count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < move_count[disks_played as usize] {
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as libc::c_int);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        unmake_move(side_to_move, this_move);
        /* Check if the move leads to a book position and, if it does,
           whether it has the solve status (WLD or FULL) specified by FLAGS. */
        deviation = 0 as libc::c_int;
        if slot == -(1 as libc::c_int) ||
               *book_hash_table.offset(slot as isize) == -(1 as libc::c_int) {
            if this_move == alternative_move && flags == 0 {
                score = alternative_score;
                child_feasible = 1 as libc::c_int;
                deviation = 1 as libc::c_int
            } else {
                child_feasible = 0 as libc::c_int;
                score = 0 as libc::c_int
            }
        } else if (*node.offset(*book_hash_table.offset(slot as isize) as
                                    isize)).flags as libc::c_int & flags != 0
                      || flags == 0 {
            if side_to_move == 0 as libc::c_int {
                score =
                    (*node.offset(*book_hash_table.offset(slot as isize) as
                                      isize)).black_minimax_score as
                        libc::c_int
            } else {
                score =
                    (*node.offset(*book_hash_table.offset(slot as isize) as
                                      isize)).white_minimax_score as
                        libc::c_int
            }
            child_feasible = 1 as libc::c_int
        } else { child_feasible = 0 as libc::c_int; score = 0 as libc::c_int }
        if child_feasible != 0 && score == 0 as libc::c_int &&
               (*node.offset(index as isize)).flags as libc::c_int &
                   4 as libc::c_int == 0 &&
               (*node.offset(*book_hash_table.offset(slot as isize) as
                                 isize)).flags as libc::c_int &
                   4 as libc::c_int != 0 {
            /* Check if this is a book draw that should be avoided, i.e., one
               where the current position is not solved but the child position
               is solved for a draw, and the draw mode dictates this draw to
               be a bad one. */
            if game_mode as libc::c_uint ==
                   PRIVATE_GAME as libc::c_int as libc::c_uint ||
                   (*node.offset(*book_hash_table.offset(slot as isize) as
                                     isize)).flags as libc::c_int &
                       32 as libc::c_int == 0 {
                if side_to_move == 0 as libc::c_int {
                    if draw_mode as libc::c_uint ==
                           WHITE_WINS as libc::c_int as libc::c_uint ||
                           draw_mode as libc::c_uint ==
                               OPPONENT_WINS as libc::c_int as libc::c_uint {
                        printf(b"%c%c leads to an unwanted book draw\n\x00" as
                                   *const u8 as *const libc::c_char,
                               'a' as i32 + this_move % 10 as libc::c_int -
                                   1 as libc::c_int,
                               '0' as i32 + this_move / 10 as libc::c_int);
                        child_feasible = 0 as libc::c_int
                    }
                } else if draw_mode as libc::c_uint ==
                              BLACK_WINS as libc::c_int as libc::c_uint ||
                              draw_mode as libc::c_uint ==
                                  OPPONENT_WINS as libc::c_int as libc::c_uint
                 {
                    printf(b"%c%c leads to an unwanted book draw\n\x00" as
                               *const u8 as *const libc::c_char,
                           'a' as i32 + this_move % 10 as libc::c_int -
                               1 as libc::c_int,
                           '0' as i32 + this_move / 10 as libc::c_int);
                    child_feasible = 0 as libc::c_int
                }
            }
        }
        if child_feasible != 0 {
            candidate_list[candidate_count as usize].move_0 =
                move_list[disks_played as usize][i as usize];
            candidate_list[candidate_count as usize].score = sign * score;
            if deviation != 0 {
                candidate_list[candidate_count as usize].flags =
                    64 as libc::c_int
            } else {
                candidate_list[candidate_count as usize].flags =
                    (*node.offset(*book_hash_table.offset(slot as isize) as
                                      isize)).flags as libc::c_int
            }
            candidate_list[candidate_count as usize].parent_flags =
                root_flags;
            candidate_count += 1
        }
        i += 1
    }
    if candidate_count > 0 as libc::c_int {
        loop 
             /* Sort the book moves using bubble sort */
             {
            changed = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < candidate_count - 1 as libc::c_int {
                if candidate_list[i as usize].score <
                       candidate_list[(i + 1 as libc::c_int) as usize].score {
                    changed = 1 as libc::c_int;
                    temp = candidate_list[i as usize];
                    candidate_list[i as usize] =
                        candidate_list[(i + 1 as libc::c_int) as usize];
                    candidate_list[(i + 1 as libc::c_int) as usize] = temp
                }
                i += 1
            }
            if !(changed != 0) { break ; }
        }
    };
}
/*
  GET_CANDIDATE_COUNT
  GET_CANDIDATE
  Accessor functions for the data structure created by
  FILL_MOVE_ALTERNATIVES.
*/

pub unsafe fn get_candidate_count() -> libc::c_int {
    return candidate_count;
}

pub unsafe fn get_candidate(mut index: libc::c_int)
 -> CandidateMove {
    return candidate_list[index as usize];
}
/*
   PRINT_MOVE_ALTERNATIVES
   Displays all available book moves from a position.
   FLAGS specifies a subset of the flag bits which have to be set
   for a position to be considered. Notice that FLAGS=0 accepts
   any flag combination.
*/

pub unsafe fn print_move_alternatives(mut side_to_move:
                                                     libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut output_score: libc::c_int = 0;
    if candidate_count > 0 as libc::c_int {
        if side_to_move == 0 as libc::c_int {
            sign = 1 as libc::c_int
        } else { sign = -(1 as libc::c_int) }
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        /* Check that the position is in the opening book after all */
        if slot == -(1 as libc::c_int) ||
               *book_hash_table.offset(slot as isize) == -(1 as libc::c_int) {
            return
        }
        /* Pick the book score corresponding to the player to move and
           remove draw avoidance and the special scores for nodes WLD. */
        if side_to_move == 0 as libc::c_int {
            score =
                (*node.offset(*book_hash_table.offset(slot as isize) as
                                  isize)).black_minimax_score as libc::c_int
        } else {
            score =
                (*node.offset(*book_hash_table.offset(slot as isize) as
                                  isize)).white_minimax_score as libc::c_int
        }
        if score == 30000 as libc::c_int - 1 as libc::c_int ||
               score == -(30000 as libc::c_int - 1 as libc::c_int) {
            score = 0 as libc::c_int
        }
        if score > 30000 as libc::c_int { score -= 30000 as libc::c_int }
        if score < -(30000 as libc::c_int) { score += 30000 as libc::c_int }
        printf(b"Book score is \x00" as *const u8 as *const libc::c_char);
        if (*node.offset(*book_hash_table.offset(slot as isize) as
                             isize)).flags as libc::c_int & 16 as libc::c_int
               != 0 {
            printf(b"%+d (exact score).\x00" as *const u8 as
                       *const libc::c_char, sign * score);
        } else if (*node.offset(*book_hash_table.offset(slot as isize) as
                                    isize)).flags as libc::c_int &
                      4 as libc::c_int != 0 {
            printf(b"%+d (W/L/D solved).\x00" as *const u8 as
                       *const libc::c_char, sign * score);
        } else {
            printf(b"%+.2f.\x00" as *const u8 as *const libc::c_char,
                   (sign * score) as libc::c_double / 128.0f64);
        }
        if (*node.offset(*book_hash_table.offset(slot as isize) as
                             isize)).flags as libc::c_int & 32 as libc::c_int
               != 0 {
            printf(b" Private node.\x00" as *const u8 as *const libc::c_char);
        }
        puts(b"\x00" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < candidate_count {
            printf(b"   %c%c   \x00" as *const u8 as *const libc::c_char,
                   'a' as i32 +
                       candidate_list[i as usize].move_0 % 10 as libc::c_int -
                       1 as libc::c_int,
                   '0' as i32 +
                       candidate_list[i as usize].move_0 / 10 as libc::c_int);
            output_score = candidate_list[i as usize].score;
            if output_score >= 30000 as libc::c_int {
                output_score -= 30000 as libc::c_int
            } else if output_score <= -(30000 as libc::c_int) {
                output_score += 30000 as libc::c_int
            }
            if candidate_list[i as usize].flags & 16 as libc::c_int != 0 {
                printf(b"%+-6d  (exact score)\x00" as *const u8 as
                           *const libc::c_char, output_score);
            } else if candidate_list[i as usize].flags & 4 as libc::c_int != 0
             {
                printf(b"%+-6d  (W/L/D solved)\x00" as *const u8 as
                           *const libc::c_char, output_score);
            } else {
                printf(b"%+-6.2f\x00" as *const u8 as *const libc::c_char,
                       output_score as libc::c_double / 128.0f64);
                if candidate_list[i as usize].flags & 64 as libc::c_int != 0 {
                    printf(b"  (deviation)\x00" as *const u8 as
                               *const libc::c_char);
                }
            }
            puts(b"\x00" as *const u8 as *const libc::c_char);
            i += 1
        }
    };
}
/*
   GET_BOOK_MOVE
   Chooses a book move from the list of candidates
   which don't worsen the negamaxed out-of-book
   evaluation by too much.
*/

pub unsafe fn get_book_move(mut side_to_move: libc::c_int,
                                       mut update_slack: libc::c_int,
                                       mut eval_info: *mut EvaluationType)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut original_side_to_move: libc::c_int = 0;
    let mut remaining_slack: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut chosen_score: libc::c_int = 0;
    let mut best_score: libc::c_int = 0;
    let mut alternative_score: libc::c_int = 0;
    let mut feasible_count: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut chosen_index: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut base_flags: libc::c_int = 0;
    let mut random_point: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    let mut continuation: libc::c_int = 0;
    let mut is_feasible: libc::c_int = 0;
    let mut acc_weight: libc::c_int = 0;
    let mut total_weight: libc::c_int = 0;
    let mut best_move: libc::c_int = 0;
    let mut this_move: libc::c_int = 0;
    let mut alternative_move: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut weight: [libc::c_int; 60] = [0; 60];
    let mut temp_move: [libc::c_int; 60] = [0; 60];
    let mut temp_stm: [libc::c_int; 60] = [0; 60];
    /* Disable opening book randomness unless the move is going to
       be played on the board by Zebra */
    if update_slack != 0 {
        remaining_slack =
            if max_slack - used_slack[side_to_move as usize] >
                   0 as libc::c_int {
                (max_slack) - used_slack[side_to_move as usize]
            } else { 0 as libc::c_int }
    } else { remaining_slack = 0 as libc::c_int }
    if echo != 0 && candidate_count > 0 as libc::c_int &&
           get_ponder_move() == 0 {
        printf(b"Slack left is %.2f. \x00" as *const u8 as
                   *const libc::c_char,
               remaining_slack as libc::c_double / 128.0f64);
        print_move_alternatives(side_to_move);
    }
    /* No book move found? */
    if candidate_count == 0 as libc::c_int { return -(1 as libc::c_int) }
    /* Find the book flags of the original position. */
    get_hash(&mut val1, &mut val2, &mut orientation);
    slot = probe_hash_table(val1, val2);
    if slot == -(1 as libc::c_int) ||
           *book_hash_table.offset(slot as isize) == -(1 as libc::c_int) {
        fatal_error(b"Internal error in book code.\x00" as *const u8 as
                        *const libc::c_char);
    }
    base_flags =
        (*node.offset(*book_hash_table.offset(slot as isize) as isize)).flags
            as libc::c_int;
    /* If we have an endgame score for the position, we only want to
       consult the book if there is at least one move realizing that score. */
    index = *book_hash_table.offset(slot as isize);
    if (*node.offset(index as isize)).flags as libc::c_int & 16 as libc::c_int
           != 0 {
        if candidate_list[0 as libc::c_int as usize].score <
               (*node.offset(index as isize)).black_minimax_score as
                   libc::c_int {
            return -(1 as libc::c_int)
        }
    } else if (*node.offset(index as isize)).flags as libc::c_int &
                  4 as libc::c_int != 0 {
        if (*node.offset(index as isize)).black_minimax_score as libc::c_int >
               0 as libc::c_int &&
               candidate_list[0 as libc::c_int as usize].score <=
                   0 as libc::c_int {
            return -(1 as libc::c_int)
        }
    }
    /* Don't randomize among solved moves */
    score = candidate_list[0 as libc::c_int as usize].score;
    if score >= 30000 as libc::c_int { remaining_slack = 0 as libc::c_int }
    feasible_count = 0 as libc::c_int;
    total_weight = 0 as libc::c_int;
    while feasible_count < candidate_count &&
              candidate_list[feasible_count as usize].score >=
                  score - remaining_slack {
        weight[feasible_count as usize] =
            2 as libc::c_int * remaining_slack + 1 as libc::c_int -
                (score - candidate_list[feasible_count as usize].score);
        total_weight += weight[feasible_count as usize];
        feasible_count += 1
    }
    /* Chose a move at random from the moves which don't worsen
       the position by more than the allowed slack (and, optionally,
       update it). A simple weighting scheme makes the moves with
       scores close to the best move most likely to be chosen. */
    if feasible_count == 1 as libc::c_int {
        chosen_index = 0 as libc::c_int
    } else {
        random_point =
            ((my_random() >> 10 as libc::c_int) %
                 total_weight as libc::c_long) as libc::c_int;
        chosen_index = 0 as libc::c_int;
        acc_weight = weight[chosen_index as usize];
        while random_point > acc_weight {
            chosen_index += 1;
            acc_weight += weight[chosen_index as usize]
        }
    }
    chosen_score = candidate_list[chosen_index as usize].score;
    if update_slack != 0 {
        used_slack[side_to_move as usize] += score - chosen_score
    }
    /* Convert the book score to the normal form.
       Note that this should work also for old-style book values. */
    if chosen_score >= 30000 as libc::c_int {
        chosen_score -= 30000 as libc::c_int;
        if chosen_score <= 64 as libc::c_int {
            chosen_score *= 128 as libc::c_int
        }
    }
    if chosen_score <= -(30000 as libc::c_int) {
        chosen_score += 30000 as libc::c_int;
        if chosen_score >= -(64 as libc::c_int) {
            chosen_score *= 128 as libc::c_int
        }
    }
    /* Return the score via the EvaluationType structure */
    flags = candidate_list[chosen_index as usize].flags;
    *eval_info =
        create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION, chosen_score,
                         0.0f64, 0 as libc::c_int, 1 as libc::c_int);
    if base_flags & (16 as libc::c_int | 4 as libc::c_int) != 0 &&
           flags & (16 as libc::c_int | 4 as libc::c_int) != 0 {
        /* Both the base position and the position after the book move
           are solved. */
        if base_flags & 16 as libc::c_int != 0 &&
               flags & 16 as libc::c_int != 0 {
            (*eval_info).type_0 = EXACT_EVAL
        } else { (*eval_info).type_0 = WLD_EVAL }
        if chosen_score > 0 as libc::c_int {
            (*eval_info).res = WON_POSITION
        } else if chosen_score == 0 as libc::c_int {
            (*eval_info).res = DRAWN_POSITION
        } else { (*eval_info).res = LOST_POSITION }
    } else if flags & 4 as libc::c_int != 0 && chosen_score > 0 as libc::c_int
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
        send_status(b"-->   Book     \x00" as *const u8 as
                        *const libc::c_char);
        if flags & 16 as libc::c_int != 0 {
            send_status(b"%+3d (exact)   \x00" as *const u8 as
                            *const libc::c_char,
                        chosen_score / 128 as libc::c_int);
        } else if flags & 4 as libc::c_int != 0 {
            send_status(b"%+3d (WLD)     \x00" as *const u8 as
                            *const libc::c_char,
                        chosen_score / 128 as libc::c_int);
        } else {
            send_status(b"%+6.2f        \x00" as *const u8 as
                            *const libc::c_char,
                        chosen_score as libc::c_double / 128.0f64);
        }
        if get_ponder_move() != 0 {
            send_status(b"{%c%c} \x00" as *const u8 as *const libc::c_char,
                        'a' as i32 + get_ponder_move() % 10 as libc::c_int -
                            1 as libc::c_int,
                        '0' as i32 + get_ponder_move() / 10 as libc::c_int);
        }
        send_status(b"%c%c\x00" as *const u8 as *const libc::c_char,
                    'a' as i32 +
                        candidate_list[chosen_index as usize].move_0 %
                            10 as libc::c_int - 1 as libc::c_int,
                    '0' as i32 +
                        candidate_list[chosen_index as usize].move_0 /
                            10 as libc::c_int);
    }
    /* Fill the PV structure with the optimal book line */
    original_side_to_move = side_to_move;
    level = 0 as libc::c_int;
    temp_move[0 as libc::c_int as usize] =
        candidate_list[chosen_index as usize].move_0;
    loop  {
        temp_stm[level as usize] = side_to_move;
        make_move(side_to_move, temp_move[level as usize], 1 as libc::c_int);
        level += 1;
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        continuation = 1 as libc::c_int;
        if slot == -(1 as libc::c_int) ||
               *book_hash_table.offset(slot as isize) == -(1 as libc::c_int) {
            continuation = 0 as libc::c_int
        } else {
            alternative_move =
                (*node.offset(*book_hash_table.offset(slot as isize) as
                                  isize)).best_alternative_move as
                    libc::c_int;
            if alternative_move > 0 as libc::c_int {
                alternative_move =
                    *inv_symmetry_map[orientation as
                                          usize].offset(alternative_move as
                                                            isize);
                alternative_score =
                    adjust_score((*node.offset(*book_hash_table.offset(slot as
                                                                           isize)
                                                   as
                                                   isize)).alternative_score
                                     as libc::c_int, side_to_move)
            } else { alternative_score = -(12345678 as libc::c_int) }
            if (*node.offset(*book_hash_table.offset(slot as isize) as
                                 isize)).flags as libc::c_int &
                   1 as libc::c_int != 0 {
                side_to_move = 0 as libc::c_int;
                sign = 1 as libc::c_int
            } else {
                side_to_move = 2 as libc::c_int;
                sign = -(1 as libc::c_int)
            }
            generate_all(side_to_move);
            best_score = -(12345678 as libc::c_int);
            best_move = -(1 as libc::c_int);
            i = 0 as libc::c_int;
            while i < move_count[disks_played as usize] {
                this_move = move_list[disks_played as usize][i as usize];
                make_move(side_to_move, this_move, 1 as libc::c_int);
                get_hash(&mut val1, &mut val2, &mut orientation);
                slot = probe_hash_table(val1, val2);
                unmake_move(side_to_move, this_move);
                if slot == -(1 as libc::c_int) ||
                       *book_hash_table.offset(slot as isize) ==
                           -(1 as libc::c_int) {
                    if this_move == alternative_move {
                        score = alternative_score;
                        is_feasible = 1 as libc::c_int
                    } else { is_feasible = 0 as libc::c_int }
                } else {
                    if original_side_to_move == 0 as libc::c_int {
                        score =
                            (*node.offset(*book_hash_table.offset(slot as
                                                                      isize)
                                              as isize)).black_minimax_score
                                as libc::c_int
                    } else {
                        score =
                            (*node.offset(*book_hash_table.offset(slot as
                                                                      isize)
                                              as isize)).white_minimax_score
                                as libc::c_int
                    }
                    is_feasible = 1 as libc::c_int
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
            if best_move == -(1 as libc::c_int) {
                continuation = 0 as libc::c_int
            } else { temp_move[level as usize] = best_move }
        }
        if !(continuation != 0) { break ; }
    }
    pv_depth[0 as libc::c_int as usize] = level;
    i = 0 as libc::c_int;
    while i < level {
        pv[0 as libc::c_int as usize][i as usize] = temp_move[i as usize];
        i += 1
    }
    loop  {
        level -= 1;
        unmake_move(temp_stm[level as usize], temp_move[level as usize]);
        if !(level > 0 as libc::c_int) { break ; }
    }
    return candidate_list[chosen_index as usize].move_0;
}
/*
  DUPSTR
  A strdup clone.
*/
unsafe fn dupstr(mut str: *const libc::c_char)
 -> *mut libc::c_char {
    let mut new_str =
        malloc(strlen(str).wrapping_add(1 as libc::c_int as libc::c_ulong)) as
            *mut libc::c_char;
    strcpy(new_str, str);
    return new_str;
}
/*
  CONVERT_OPENING_LIST
  Convert a list of openings on Robert Gatliff's format
  to a hash table representation containing the same information.
*/

pub unsafe fn convert_opening_list(mut base_file:
                                                  *const libc::c_char) {
    let mut in_stream =
        0 as *mut FILE; /* Max number of opening names occurring */
    let mut out_stream = 0 as *mut FILE;
    let mut name_start = 0 as *mut libc::c_char;
    let mut scan_ptr = 0 as *mut libc::c_char;
    let mut move_ptr = 0 as *mut libc::c_char;
    let mut source_file_name = 0 as *const libc::c_char;
    let mut header_file_name = 0 as *const libc::c_char;
    let mut parent: [*mut libc::c_char; 1000] =
        [0 as *mut libc::c_char; 1000];
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut move_seq: [libc::c_char; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut opening_count: libc::c_int = 0;
    let mut op_move_count: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    let mut hash_val1: libc::c_int = 0;
    let mut hash_val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    let mut op_move: [libc::c_int; 60] = [0; 60];
    let mut side_to_move: [libc::c_int; 60] = [0; 60];
    let mut timer: time_t = 0;
    in_stream =
        fopen(base_file, b"r\x00" as *const u8 as *const libc::c_char);
    if in_stream.is_null() {
        printf(b"Cannot open opening file \'%s\'\n\x00" as *const u8 as
                   *const libc::c_char, base_file);
        exit(1 as libc::c_int);
    }
    /* Get the number of openings */
    fgets(buffer.as_mut_ptr(), 1023 as libc::c_int, in_stream);
    sscanf(buffer.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
           &mut opening_count as *mut libc::c_int);
    /* Prepare the header file */
    header_file_name = b"opname.h\x00" as *const u8 as *const libc::c_char;
    out_stream =
        fopen(header_file_name, b"w\x00" as *const u8 as *const libc::c_char);
    if out_stream.is_null() {
        printf(b"Cannot create header file \'%s\'\n\x00" as *const u8 as
                   *const libc::c_char, header_file_name);
        exit(1 as libc::c_int);
    }
    time(&mut timer);
    fprintf(out_stream, b"/*\n\x00" as *const u8 as *const libc::c_char);
    fprintf(out_stream, b"   %s\n\n\x00" as *const u8 as *const libc::c_char,
            header_file_name);
    fprintf(out_stream,
            b"   Automatically created by BOOKTOOL on %s\x00" as *const u8 as
                *const libc::c_char, ctime(&mut timer));
    fprintf(out_stream, b"*/\x00" as *const u8 as *const libc::c_char);
    fprintf(out_stream, b"\n\n\n\x00" as *const u8 as *const libc::c_char);
    fputs(b"#ifndef OPNAME_H\n\x00" as *const u8 as *const libc::c_char,
          out_stream);
    fputs(b"#define OPNAME_H\n\n\n\x00" as *const u8 as *const libc::c_char,
          out_stream);
    fprintf(out_stream,
            b"#define OPENING_COUNT       %d\n\n\n\x00" as *const u8 as
                *const libc::c_char, opening_count);
    fputs(b"typedef struct {\n\x00" as *const u8 as *const libc::c_char,
          out_stream);
    fputs(b"  const char *name;\n\x00" as *const u8 as *const libc::c_char,
          out_stream);
    fputs(b"  const char *sequence;\n\x00" as *const u8 as
              *const libc::c_char, out_stream);
    fputs(b"  int hash_val1;\n\x00" as *const u8 as *const libc::c_char,
          out_stream);
    fputs(b"  int hash_val2;\n\x00" as *const u8 as *const libc::c_char,
          out_stream);
    fputs(b"  int level;\n\x00" as *const u8 as *const libc::c_char,
          out_stream);
    fputs(b"} OpeningDescriptor;\n\n\n\x00" as *const u8 as
              *const libc::c_char, out_stream);
    fputs(b"extern OpeningDescriptor opening_list[OPENING_COUNT];\n\x00" as
              *const u8 as *const libc::c_char, out_stream);
    fputs(b"\n\n#endif  /* OPNAME_H */\n\x00" as *const u8 as
              *const libc::c_char, out_stream);
    fclose(out_stream);
    /* Prepare the source file */
    source_file_name = b"opname.c\x00" as *const u8 as *const libc::c_char;
    out_stream =
        fopen(source_file_name, b"w\x00" as *const u8 as *const libc::c_char);
    if out_stream.is_null() {
        printf(b"Cannot create source file \'%s\'\n\x00" as *const u8 as
                   *const libc::c_char, source_file_name);
        exit(1 as libc::c_int);
    }
    time(&mut timer);
    fprintf(out_stream, b"/*\n\x00" as *const u8 as *const libc::c_char);
    fprintf(out_stream, b"   %s\n\n\x00" as *const u8 as *const libc::c_char,
            source_file_name);
    fprintf(out_stream,
            b"   Automatically created by BOOKTOOL on %s\x00" as *const u8 as
                *const libc::c_char, ctime(&mut timer));
    fprintf(out_stream, b"*/\x00" as *const u8 as *const libc::c_char);
    fprintf(out_stream, b"\n\n\n\x00" as *const u8 as *const libc::c_char);
    fprintf(out_stream,
            b"#include \"%s\"\n\n\n\x00" as *const u8 as *const libc::c_char,
            header_file_name);
    fputs(b"OpeningDescriptor opening_list[OPENING_COUNT] = {\n\x00" as
              *const u8 as *const libc::c_char, out_stream);
    /* Read the list of openings */
    prepare_tree_traversal();
    level = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < opening_count {
        fgets(buffer.as_mut_ptr(), 1023 as libc::c_int, in_stream);
        /* Each line in the input file corresponds to one opening.
           First separate the line into opening moves and name. */
        sscanf(buffer.as_mut_ptr(),
               b"%s\x00" as *const u8 as *const libc::c_char,
               move_seq.as_mut_ptr());
        name_start =
            buffer.as_mut_ptr().offset(strlen(move_seq.as_mut_ptr()) as
                                           isize);
        while *(*__ctype_b_loc()).offset(*name_start as libc::c_int as isize)
                  as libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
            name_start = name_start.offset(1)
        }
        scan_ptr = name_start;
        while *(*__ctype_b_loc()).offset(*scan_ptr as libc::c_int as isize) as
                  libc::c_int &
                  _ISprint as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
            scan_ptr = scan_ptr.offset(1)
        }
        *scan_ptr = 0 as libc::c_int as libc::c_char;
        op_move_count =
            strlen(move_seq.as_mut_ptr()).wrapping_div(2 as libc::c_int as
                                                           libc::c_ulong) as
                libc::c_int;
        j = 0 as libc::c_int;
        move_ptr = buffer.as_mut_ptr();
        while j < op_move_count {
            if *(*__ctype_b_loc()).offset(*move_ptr as libc::c_int as isize)
                   as libc::c_int &
                   _ISupper as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                side_to_move[j as usize] = 0 as libc::c_int
            } else { side_to_move[j as usize] = 2 as libc::c_int }
            col =
                toupper(*move_ptr as libc::c_int) - 'A' as i32 +
                    1 as libc::c_int;
            move_ptr = move_ptr.offset(1);
            row = *move_ptr as libc::c_int - '0' as i32;
            move_ptr = move_ptr.offset(1);
            op_move[j as usize] = 10 as libc::c_int * row + col;
            j += 1
        }
        /* Check out how the relation between this openings and the ones
           in the hierachy created to far */
        while level > 0 as libc::c_int &&
                  strstr(move_seq.as_mut_ptr(),
                         parent[(level - 1 as libc::c_int) as usize]) !=
                      move_seq.as_mut_ptr() {
            level -= 1;
            free(parent[level as usize] as *mut libc::c_void);
        }
        parent[level as usize] = dupstr(move_seq.as_mut_ptr());
        level += 1;
        /* Create the board position characteristic for the opening. */
        j = 0 as libc::c_int;
        while j < op_move_count {
            if generate_specific(op_move[j as usize],
                                 side_to_move[j as usize]) == 0 {
                printf(b"Move %d in opening #%d is illegal\n\x00" as *const u8
                           as *const libc::c_char, j + 1 as libc::c_int, i);
                exit(1 as libc::c_int);
            }
            make_move(side_to_move[j as usize], op_move[j as usize],
                      1 as libc::c_int);
            j += 1
        }
        /* Write the code fragment  */
        get_hash(&mut hash_val1, &mut hash_val2, &mut orientation);
        fprintf(out_stream,
                b"   { \"%s\",\n     \"%s\",\n     %d, %d, %d }\x00" as
                    *const u8 as *const libc::c_char, name_start,
                move_seq.as_mut_ptr(), hash_val1, hash_val2,
                level - 1 as libc::c_int);
        if i != opening_count - 1 as libc::c_int {
            fputs(b" ,\n\x00" as *const u8 as *const libc::c_char,
                  out_stream);
        }
        /* Undo the moves */
        j = op_move_count - 1 as libc::c_int;
        while j >= 0 as libc::c_int {
            unmake_move(side_to_move[j as usize], op_move[j as usize]);
            j -= 1
        }
        i += 1
    }
    fputs(b"\n};\n\x00" as *const u8 as *const libc::c_char, out_stream);
    /* Remove the hierarchy data */
    while level > 0 as libc::c_int {
        level -= 1;
        free(parent[level as usize] as *mut libc::c_void);
    }
    fclose(out_stream);
    fclose(in_stream);
}
/*
  FIND_OPENING_NAME
  Searches the opening name database read by READ_OPENING_LIST
  and returns a pointer to the name if the position was found,
  NULL otherwise.
*/

pub unsafe fn find_opening_name() -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    let mut val1: libc::c_int = 0;
    let mut val2: libc::c_int = 0;
    let mut orientation: libc::c_int = 0;
    get_hash(&mut val1, &mut val2, &mut orientation);
    i = 0 as libc::c_int;
    while i < 76 as libc::c_int {
        if val1 == opening_list[i as usize].hash_val1 &&
               val2 == opening_list[i as usize].hash_val2 {
            return opening_list[i as usize].name
        }
        i += 1
    }
    return 0 as *const libc::c_char;
}
/*
   INIT_OSF
   Makes sure all data structures are initialized.
*/

pub unsafe fn init_osf(mut do_global_setup: libc::c_int) {
    init_maps();
    prepare_hash();
    setup_hash(1 as libc::c_int);
    init_book_tree();
    reset_book_search();
    search_depth = 2 as libc::c_int;
    max_slack = 0 as libc::c_int;
    low_deviation_threshold = 60 as libc::c_int;
    high_deviation_threshold = 60 as libc::c_int;
    deviation_bonus = 0.0f64;
    min_eval_span = 0 as libc::c_int;
    max_eval_span = 1000 as libc::c_int * 128 as libc::c_int;
    min_negamax_span = 0 as libc::c_int;
    max_negamax_span = 1000 as libc::c_int * 128 as libc::c_int;
    max_batch_size = 10000000 as libc::c_int;
    force_black = 0 as libc::c_int;
    force_white = 0 as libc::c_int;
    if do_global_setup != 0 {
        global_setup(0 as libc::c_int, 19 as libc::c_int);
    };
}
/*
  CLEAR_OSF
  Free all dynamically allocated memory.
*/

pub unsafe fn clear_osf() {
    free(book_hash_table as *mut libc::c_void);
    book_hash_table = 0 as *mut libc::c_int;
    free(node as *mut libc::c_void);
    node = 0 as *mut BookNode;
}
