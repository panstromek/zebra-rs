use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    /*
   File:          autoplay.h

   Created:       May 21, 1998

   Modified:      August 1, 2002

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:
*/
    #[no_mangle]
    fn handle_event(only_passive_events: libc::c_int,
                    allow_delay: libc::c_int, passive_mode: libc::c_int);
    #[no_mangle]
    fn set_bitboards(board_0: *mut libc::c_int, side_to_move: libc::c_int,
                     my_out: *mut BitBoard, opp_out: *mut BitBoard);
    /*
   File:          bitbcnt.h

   Created:       November 22, 1999

   Modified:      November 24, 2005

   Authors:       Gunnar Andersson (gunnar@radagast.se)

   Contents:
*/
    #[no_mangle]
    static CountFlips_bitboard:
           [Option<unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint)
                       -> libc::c_int>; 78];
    /* pv[n][n..<depth>] contains the principal variation from the
   node on recursion depth n on the current recursive call sequence.
   After the search, pv[0][0..<depth>] contains the principal
   variation from the root position. */
    #[no_mangle]
    static mut pv: [[libc::c_int; 64]; 64];
    /* pv_depth[n] contains the depth of the principal variation
   starting at level n in the call sequence.
   After the search, pv[0] holds the depth of the principal variation
   from the root position. */
    #[no_mangle]
    static mut pv_depth: [libc::c_int; 64];
    /* piece_count[col][n] holds the number of disks of color col after
   n moves have been played. */
    #[no_mangle]
    static mut piece_count: [[libc::c_int; 64]; 3];
    /* Holds the current board position. Updated as the search progresses,
   but all updates must be reversed when the search stops. */
    #[no_mangle]
    static mut board: Board;
    /* The value of the root position from the last midgame or
   endgame search. Can contain strange values if an event
   occurred. */
    #[no_mangle]
    static mut root_eval: libc::c_int;
    /* Event flag which forces the search to abort immediately when set. */
    #[no_mangle]
    static mut force_return: libc::c_int;
    /* Holds the number of nodes searched during the current search. */
    #[no_mangle]
    static mut nodes: CounterType;
    /* The last available evaluations for all possible moves at all
   possible game stages. */
    #[no_mangle]
    static mut evals: [Board; 61];
    /* Move lists */
    #[no_mangle]
    static mut sorted_move_order: [[libc::c_int; 64]; 64];
    #[no_mangle]
    fn disc_count(side_to_move: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn select_move(first: libc::c_int, list_size: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn store_pv(pv_buffer: *mut libc::c_int, depth_buffer: *mut libc::c_int);
    #[no_mangle]
    fn restore_pv(pv_buffer: *mut libc::c_int, depth_buffer: libc::c_int);
    #[no_mangle]
    fn get_ponder_move() -> libc::c_int;
    #[no_mangle]
    fn create_eval_info(in_type: EvalType, in_res: EvalResult,
                        in_score: libc::c_int, in_conf: libc::c_double,
                        in_depth: libc::c_int, in_book: libc::c_int)
     -> EvaluationType;
    #[no_mangle]
    fn hash_expand_pv(side_to_move: libc::c_int, mode: libc::c_int,
                      flags: libc::c_int, max_selectivity: libc::c_int);
    #[no_mangle]
    fn set_current_eval(eval: EvaluationType);
    #[no_mangle]
    fn adjust_counter(counter: *mut CounterType);
    #[no_mangle]
    fn counter_value(counter: *mut CounterType) -> libc::c_double;
    /*
   File:          bitbmob.h

   Created:       November 22, 1999

   Modified:      December 25, 2002

   Authors:       Gunnar Andersson (gunnar@radagast.se)

   Contents:
*/
    #[no_mangle]
    fn bitboard_mobility(my_bits: BitBoard, opp_bits: BitBoard)
     -> libc::c_int;
    #[no_mangle]
    fn init_mmx();
    #[no_mangle]
    fn weighted_mobility(my_bits: BitBoard, opp_bits: BitBoard)
     -> libc::c_int;
    /*
   File:          bitbtest.h

   Created:       November 22, 1999

   Modified:      November 24, 2005

   Authors:       Gunnar Andersson (gunnar@radagast.se)

   Contents:
*/
    #[no_mangle]
    static mut bb_flips: BitBoard;
    #[no_mangle]
    static TestFlips_bitboard:
           [Option<unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                                        _: libc::c_uint, _: libc::c_uint)
                       -> libc::c_int>; 78];
    /*
   File:         display.h

   Created:      July 10, 1997

   Modified:     November 17, 2002

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:     Declarations of the screen output functions.
*/
    /* Flag variable, non-zero if output should be written to stdout. */
    #[no_mangle]
    static mut echo: libc::c_int;
    #[no_mangle]
    fn send_status(format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn send_status_time(elapsed_time: libc::c_double);
    #[no_mangle]
    fn send_status_pv(pv_0: *mut libc::c_int, max_depth: libc::c_int);
    #[no_mangle]
    fn send_status_nodes(node_count: libc::c_double);
    #[no_mangle]
    fn clear_status();
    #[no_mangle]
    fn display_status(stream: *mut FILE, allow_repeat: libc::c_int);
    #[no_mangle]
    fn send_sweep(format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn clear_sweep();
    #[no_mangle]
    fn display_sweep(stream: *mut FILE);
    #[no_mangle]
    fn reset_buffer_display();
    #[no_mangle]
    fn display_buffers();
    #[no_mangle]
    fn produce_eval_text(eval_info: EvaluationType, short_output: libc::c_int)
     -> *mut libc::c_char;
    /*
   doflip.h

   Automatically created by ENDMACRO on Fri Feb 26 20:29:42 1999

   Last modified:   October 25, 2005
*/
    #[no_mangle]
    static mut hash_update1: libc::c_uint;
    #[no_mangle]
    static mut hash_update2: libc::c_uint;
    #[no_mangle]
    fn DoFlips_hash(sqnum: libc::c_int, color: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut end_mean: [[libc::c_float; 9]; 61];
    #[no_mangle]
    static mut end_sigma: [[libc::c_float; 9]; 61];
    #[no_mangle]
    static mut end_stats_available: [[libc::c_short; 9]; 61];
    /* The 64-bit hash key. */
    #[no_mangle]
    static mut hash1: libc::c_uint;
    #[no_mangle]
    static mut hash2: libc::c_uint;
    /* 64-bit hash masks used when a disc is played on the board;
   the relation
     hash_put_value?[][] == hash_value?[][] ^ hash_flip_color?
   is guaranteed to hold. */
    #[no_mangle]
    static mut hash_put_value1: [[libc::c_uint; 128]; 3];
    #[no_mangle]
    static mut hash_put_value2: [[libc::c_uint; 128]; 3];
    /* The XOR of the hash_color*, used for disk flipping. */
    #[no_mangle]
    static mut hash_flip_color1: libc::c_uint;
    #[no_mangle]
    static mut hash_flip_color2: libc::c_uint;
    #[no_mangle]
    fn add_hash(reverse_mode: libc::c_int, score: libc::c_int,
                best: libc::c_int, flags: libc::c_int, draft: libc::c_int,
                selectivity: libc::c_int);
    #[no_mangle]
    fn add_hash_extended(reverse_mode: libc::c_int, score: libc::c_int,
                         best: *mut libc::c_int, flags: libc::c_int,
                         draft: libc::c_int, selectivity: libc::c_int);
    #[no_mangle]
    fn find_hash(entry: *mut HashEntry, reverse_mode: libc::c_int);
    #[no_mangle]
    fn toggle_midgame_hash_usage(allow_read: libc::c_int,
                                 allow_write: libc::c_int);
    #[no_mangle]
    fn tree_search(level: libc::c_int, max_depth: libc::c_int,
                   side_to_move: libc::c_int, alpha: libc::c_int,
                   beta: libc::c_int, allow_hash: libc::c_int,
                   allow_mpc: libc::c_int, void_legal: libc::c_int)
     -> libc::c_int;
    /*
   File:           moves.h

   Created:        June 30, 1997

   Modified:       August 1, 2002

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       The move generator's interface.
*/
    /* The number of disks played from the initial position.
   Must match the current status of the BOARD variable. */
    #[no_mangle]
    static mut disks_played: libc::c_int;
    /* The number of moves available after a certain number
   of disks played. */
    #[no_mangle]
    static mut move_count: [libc::c_int; 64];
    /* The actual moves available after a certain number of
   disks played. */
    #[no_mangle]
    static mut move_list: [[libc::c_int; 64]; 64];
    /* Directional flip masks for all board positions. */
    #[no_mangle]
    static dir_mask: [libc::c_int; 100];
    #[no_mangle]
    fn generate_all(side_to_move: libc::c_int);
    #[no_mangle]
    fn make_move(side_to_move: libc::c_int, move_0: libc::c_int,
                 update_hash: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn unmake_move(side_to_move: libc::c_int, move_0: libc::c_int);
    #[no_mangle]
    fn valid_move(move_0: libc::c_int, side_to_move: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn fill_endgame_hash(cutoff: libc::c_int, level: libc::c_int);
    #[no_mangle]
    fn fill_move_alternatives(side_to_move: libc::c_int, flags: libc::c_int);
    #[no_mangle]
    fn get_book_move(side_to_move: libc::c_int, update_slack: libc::c_int,
                     eval_info: *mut EvaluationType) -> libc::c_int;
    #[no_mangle]
    static mut use_end_cut: [libc::c_int; 61];
    #[no_mangle]
    static mut end_mpc_depth: [[libc::c_int; 4]; 61];
    /*
  COUNT_EDGE_STABLE
  Returns the number of stable edge discs for COLOR.
*/
    #[no_mangle]
    fn count_edge_stable(color: libc::c_int, col_bits: BitBoard,
                         opp_bits: BitBoard) -> libc::c_int;
    /*
  COUNT_STABLE
  Returns the number of stable discs for COLOR.
  Note: COUNT_EDGE_STABLE must have been called immediately
        before this function is called *or you lose big*.
*/
    #[no_mangle]
    fn count_stable(color: libc::c_int, col_bits: BitBoard,
                    opp_bits: BitBoard) -> libc::c_int;
    /* Holds the value of the variable NODES the last time the
   timer module was called to check if a panic abort occured. */
    #[no_mangle]
    static mut last_panic_check: libc::c_double;
    #[no_mangle]
    fn set_panic_threshold(value: libc::c_double);
    #[no_mangle]
    fn check_panic_abort();
    #[no_mangle]
    fn check_threshold(threshold: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn clear_panic_abort();
    #[no_mangle]
    fn is_panic_abort() -> libc::c_int;
    #[no_mangle]
    fn get_elapsed_time() -> libc::c_double;
    #[no_mangle]
    fn UndoFlips(flip_count: libc::c_int, oppcol: libc::c_int);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitBoard {
    pub high: libc::c_uint,
    pub low: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CounterType {
    pub hi: libc::c_uint,
    pub lo: libc::c_uint,
}
/*
   File:           globals.h

   Created:        June 30, 1997

   Modified:       January 8, 2000

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Global state variables.
*/
/* The basic board type. One index for each position;
   a1=11, h1=18, a8=81, h8=88. */
pub type Board = [libc::c_int; 128];
/*
   File:          search.h

   Created:       July 1, 1997

   Modified:      August 1, 2002

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The interface to common search routines and variables.
*/
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
/* All information available about a move decision. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EvaluationType {
    pub type_0: EvalType,
    pub res: EvalResult,
    pub score: libc::c_int,
    pub confidence: libc::c_double,
    pub search_depth: libc::c_int,
    pub is_book: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MoveLink {
    pub pred: libc::c_int,
    pub succ: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashEntry {
    pub key1: libc::c_uint,
    pub key2: libc::c_uint,
    pub eval: libc::c_int,
    pub move_0: [libc::c_int; 4],
    pub draft: libc::c_short,
    pub selectivity: libc::c_short,
    pub flags: libc::c_short,
}
pub const DRAW: C2RustUnnamed = 2;
pub type C2RustUnnamed = libc::c_uint;
pub const UNKNOWN: C2RustUnnamed = 3;
pub const LOSS: C2RustUnnamed = 1;
pub const WIN: C2RustUnnamed = 0;
#[no_mangle]
pub static mut end_move_list: [MoveLink; 100] =
    [MoveLink{pred: 0, succ: 0,}; 100];
/* The parities of the regions are in the region_parity bit vector. */
static mut region_parity: libc::c_uint = 0;
/* Pseudo-probabilities corresponding to the percentiles.
   These are taken from the normal distribution; to the percentile
   x corresponds the probability Pr(-x <= Y <= x) where Y is a N(0,1)
   variable. */
static mut confidence: [libc::c_double; 10] =
    [1.000f64, 0.99f64, 0.98f64, 0.954f64, 0.911f64, 0.838f64, 0.729f64,
     0.576f64, 0.383f64, 0.197f64];
/* Percentiles used in the endgame MPC */
static mut end_percentile: [libc::c_double; 10] =
    [100.0f64, 4.0f64, 3.0f64, 2.0f64, 1.7f64, 1.4f64, 1.1f64, 0.8f64, 0.5f64,
     0.25f64];
static mut stability_threshold: [libc::c_int; 19] =
    [65 as libc::c_int, 65 as libc::c_int, 65 as libc::c_int,
     65 as libc::c_int, 65 as libc::c_int, 46 as libc::c_int,
     38 as libc::c_int, 30 as libc::c_int, 24 as libc::c_int,
     24 as libc::c_int, 24 as libc::c_int, 24 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int];
static mut fast_first_mean: [[libc::c_double; 64]; 61] = [[0.; 64]; 61];
static mut fast_first_sigma: [[libc::c_double; 64]; 61] = [[0.; 64]; 61];
static mut best_move: libc::c_int = 0;
static mut best_end_root_move: libc::c_int = 0;
static mut true_found: libc::c_int = 0;
static mut true_val: libc::c_int = 0;
static mut full_output_mode: libc::c_int = 0;
static mut earliest_wld_solve: libc::c_int = 0;
static mut earliest_full_solve: libc::c_int = 0;
static mut fast_first_threshold: [[libc::c_int; 64]; 61] = [[0; 64]; 61];
static mut ff_mob_factor: [libc::c_int; 61] = [0; 61];
static mut neighborhood_mask: [BitBoard; 100] =
    [BitBoard{high: 0, low: 0,}; 100];
#[no_mangle]
pub static mut quadrant_mask: [libc::c_uint; 100] =
    [0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     1 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     1 as libc::c_int as libc::c_uint, 2 as libc::c_int as libc::c_uint,
     2 as libc::c_int as libc::c_uint, 2 as libc::c_int as libc::c_uint,
     2 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     1 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     1 as libc::c_int as libc::c_uint, 2 as libc::c_int as libc::c_uint,
     2 as libc::c_int as libc::c_uint, 2 as libc::c_int as libc::c_uint,
     2 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     1 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     1 as libc::c_int as libc::c_uint, 2 as libc::c_int as libc::c_uint,
     2 as libc::c_int as libc::c_uint, 2 as libc::c_int as libc::c_uint,
     2 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     1 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     1 as libc::c_int as libc::c_uint, 2 as libc::c_int as libc::c_uint,
     2 as libc::c_int as libc::c_uint, 2 as libc::c_int as libc::c_uint,
     2 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 4 as libc::c_int as libc::c_uint,
     4 as libc::c_int as libc::c_uint, 4 as libc::c_int as libc::c_uint,
     4 as libc::c_int as libc::c_uint, 8 as libc::c_int as libc::c_uint,
     8 as libc::c_int as libc::c_uint, 8 as libc::c_int as libc::c_uint,
     8 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 4 as libc::c_int as libc::c_uint,
     4 as libc::c_int as libc::c_uint, 4 as libc::c_int as libc::c_uint,
     4 as libc::c_int as libc::c_uint, 8 as libc::c_int as libc::c_uint,
     8 as libc::c_int as libc::c_uint, 8 as libc::c_int as libc::c_uint,
     8 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 4 as libc::c_int as libc::c_uint,
     4 as libc::c_int as libc::c_uint, 4 as libc::c_int as libc::c_uint,
     4 as libc::c_int as libc::c_uint, 8 as libc::c_int as libc::c_uint,
     8 as libc::c_int as libc::c_uint, 8 as libc::c_int as libc::c_uint,
     8 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 4 as libc::c_int as libc::c_uint,
     4 as libc::c_int as libc::c_uint, 4 as libc::c_int as libc::c_uint,
     4 as libc::c_int as libc::c_uint, 8 as libc::c_int as libc::c_uint,
     8 as libc::c_int as libc::c_uint, 8 as libc::c_int as libc::c_uint,
     8 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint];
/* Number of discs that the side to move at the root has to win with. */
static mut komi_shift: libc::c_int = 0;
/*
  TESTFLIPS_WRAPPER
  Checks if SQ is a valid move by
  (1) verifying that there exists a neighboring opponent disc,
  (2) verifying that the move flips some disc.
*/
unsafe extern "C" fn TestFlips_wrapper(mut sq: libc::c_int,
                                       mut my_bits: BitBoard,
                                       mut opp_bits: BitBoard)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    if neighborhood_mask[sq as usize].high & opp_bits.high |
           neighborhood_mask[sq as usize].low & opp_bits.low !=
           0 as libc::c_int as libc::c_uint {
        flipped =
            TestFlips_bitboard[(sq - 11 as libc::c_int) as
                                   usize].expect("non-null function pointer")(my_bits.high,
                                                                              my_bits.low,
                                                                              opp_bits.high,
                                                                              opp_bits.low)
    } else { flipped = 0 as libc::c_int }
    return flipped;
}
/*
  PREPARE_TO_SOLVE
  Create the list of empty squares.
*/
unsafe extern "C" fn prepare_to_solve(mut board_0: *const libc::c_int) {
    /* fixed square ordering: */
    /* jcw's order, which is the best of 4 tried (according to Warren Smith) */
    static mut worst2best: [libc::c_uchar; 64] =
        [22 as libc::c_int as libc::c_uchar,
         27 as libc::c_int as libc::c_uchar,
         72 as libc::c_int as libc::c_uchar,
         77 as libc::c_int as libc::c_uchar,
         12 as libc::c_int as libc::c_uchar,
         17 as libc::c_int as libc::c_uchar,
         21 as libc::c_int as libc::c_uchar,
         28 as libc::c_int as libc::c_uchar,
         71 as libc::c_int as libc::c_uchar,
         78 as libc::c_int as libc::c_uchar,
         82 as libc::c_int as libc::c_uchar,
         87 as libc::c_int as libc::c_uchar,
         23 as libc::c_int as libc::c_uchar,
         26 as libc::c_int as libc::c_uchar,
         32 as libc::c_int as libc::c_uchar,
         37 as libc::c_int as libc::c_uchar,
         62 as libc::c_int as libc::c_uchar,
         67 as libc::c_int as libc::c_uchar,
         73 as libc::c_int as libc::c_uchar,
         76 as libc::c_int as libc::c_uchar,
         24 as libc::c_int as libc::c_uchar,
         25 as libc::c_int as libc::c_uchar,
         42 as libc::c_int as libc::c_uchar,
         47 as libc::c_int as libc::c_uchar,
         52 as libc::c_int as libc::c_uchar,
         57 as libc::c_int as libc::c_uchar,
         74 as libc::c_int as libc::c_uchar,
         75 as libc::c_int as libc::c_uchar,
         34 as libc::c_int as libc::c_uchar,
         35 as libc::c_int as libc::c_uchar,
         43 as libc::c_int as libc::c_uchar,
         46 as libc::c_int as libc::c_uchar,
         53 as libc::c_int as libc::c_uchar,
         56 as libc::c_int as libc::c_uchar,
         64 as libc::c_int as libc::c_uchar,
         65 as libc::c_int as libc::c_uchar,
         13 as libc::c_int as libc::c_uchar,
         16 as libc::c_int as libc::c_uchar,
         31 as libc::c_int as libc::c_uchar,
         38 as libc::c_int as libc::c_uchar,
         61 as libc::c_int as libc::c_uchar,
         68 as libc::c_int as libc::c_uchar,
         83 as libc::c_int as libc::c_uchar,
         86 as libc::c_int as libc::c_uchar,
         14 as libc::c_int as libc::c_uchar,
         15 as libc::c_int as libc::c_uchar,
         41 as libc::c_int as libc::c_uchar,
         48 as libc::c_int as libc::c_uchar,
         51 as libc::c_int as libc::c_uchar,
         58 as libc::c_int as libc::c_uchar,
         84 as libc::c_int as libc::c_uchar,
         85 as libc::c_int as libc::c_uchar,
         33 as libc::c_int as libc::c_uchar,
         36 as libc::c_int as libc::c_uchar,
         63 as libc::c_int as libc::c_uchar,
         66 as libc::c_int as libc::c_uchar,
         11 as libc::c_int as libc::c_uchar,
         18 as libc::c_int as libc::c_uchar,
         81 as libc::c_int as libc::c_uchar,
         88 as libc::c_int as libc::c_uchar,
         44 as libc::c_int as libc::c_uchar,
         45 as libc::c_int as libc::c_uchar,
         54 as libc::c_int as libc::c_uchar,
         45 as libc::c_int as libc::c_uchar];
    let mut i: libc::c_int = 0;
    let mut last_sq: libc::c_int = 0;
    region_parity = 0 as libc::c_int as libc::c_uint;
    last_sq = 0 as libc::c_int;
    i = 59 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut sq = worst2best[i as usize] as libc::c_int;
        if *board_0.offset(sq as isize) == 1 as libc::c_int {
            end_move_list[last_sq as usize].succ = sq;
            end_move_list[sq as usize].pred = last_sq;
            region_parity ^= quadrant_mask[sq as usize];
            last_sq = sq
        }
        i -= 1
    }
    end_move_list[last_sq as usize].succ = 99 as libc::c_int;
}
/*
  SOLVE_TWO_EMPTY
  SOLVE_THREE_EMPTY
  SOLVE_FOUR_EMPTY
  SOLVE_PARITY
  SOLVE_PARITY_HASH
  SOLVE_PARITY_HASH_HIGH
  These are the core routines of the low level endgame code.
  They all perform the same task: Return the score for the side to move.
  Structural differences:
  * SOLVE_TWO_EMPTY may only be called for *exactly* two empty
  * SOLVE_THREE_EMPTY may only be called for *exactly* three empty
  * SOLVE_FOUR_EMPTY may only be called for *exactly* four empty
  * SOLVE_PARITY uses stability, parity and fixed move ordering
  * SOLVE_PARITY_HASH uses stability, hash table and fixed move ordering
  * SOLVE_PARITY_HASH_HIGH uses stability, hash table and (non-thresholded)
    fastest first
*/
unsafe extern "C" fn solve_two_empty(mut my_bits: BitBoard,
                                     mut opp_bits: BitBoard,
                                     mut sq1: libc::c_int,
                                     mut sq2: libc::c_int,
                                     mut alpha: libc::c_int,
                                     mut beta: libc::c_int,
                                     mut disc_diff: libc::c_int,
                                     mut pass_legal: libc::c_int)
 -> libc::c_int {
    // BitBoard new_opp_bits;
    let mut score = -(12345678 as libc::c_int);
    let mut flipped: libc::c_int = 0;
    let mut ev: libc::c_int = 0;
    nodes.lo = nodes.lo.wrapping_add(1);
    /* Overall strategy: Lazy evaluation whenever possible, i.e., don't
       update bitboards until they are used. Also look at alpha and beta
       in order to perform strength reduction: Feasibility testing is
       faster than counting number of flips. */
    /* Try the first of the two empty squares... */
    flipped = TestFlips_wrapper(sq1, my_bits, opp_bits);
    if flipped != 0 as libc::c_int {
        /* SQ1 feasible for me */
        nodes.lo = nodes.lo.wrapping_add(1);
        ev = disc_diff + 2 as libc::c_int * flipped;
        flipped =
            CountFlips_bitboard[(sq2 - 11 as libc::c_int) as
                                    usize].expect("non-null function pointer")(opp_bits.high
                                                                                   &
                                                                                   !bb_flips.high,
                                                                               opp_bits.low
                                                                                   &
                                                                                   !bb_flips.low);
        if flipped != 0 as libc::c_int {
            ev -= 2 as libc::c_int * flipped
        } else if ev >= 0 as libc::c_int {
            /* He passes, check if SQ2 is feasible for me */
            /* I'm ahead, so EV will increase by at least 2 */
            ev += 2 as libc::c_int;
            if ev < beta {
                /* Only bother if not certain fail-high */
                ev +=
                    2 as libc::c_int *
                        CountFlips_bitboard[(sq2 - 11 as libc::c_int) as
                                                usize].expect("non-null function pointer")(bb_flips.high,
                                                                                           bb_flips.low)
            }
        } else if ev < beta {
            /* Only bother if not fail-high already */
            flipped =
                CountFlips_bitboard[(sq2 - 11 as libc::c_int) as
                                        usize].expect("non-null function pointer")(bb_flips.high,
                                                                                   bb_flips.low);
            if flipped != 0 as libc::c_int {
                /* ELSE: SQ2 will end up empty, game over */
                /* SQ2 feasible for me, game over */
                ev += 2 as libc::c_int * (flipped + 1 as libc::c_int)
            }
        }
        /* Being legal, the first move is the best so far */
        score = ev;
        if score > alpha { if score >= beta { return score } alpha = score }
    }
    /* ...and then the second */
    flipped = TestFlips_wrapper(sq2, my_bits, opp_bits);
    if flipped != 0 as libc::c_int {
        /* SQ2 feasible for me */
        nodes.lo = nodes.lo.wrapping_add(1);
        ev = disc_diff + 2 as libc::c_int * flipped;
        flipped =
            CountFlips_bitboard[(sq1 - 11 as libc::c_int) as
                                    usize].expect("non-null function pointer")(opp_bits.high
                                                                                   &
                                                                                   !bb_flips.high,
                                                                               opp_bits.low
                                                                                   &
                                                                                   !bb_flips.low);
        if flipped != 0 as libc::c_int {
            /* SQ1 feasible for him, game over */
            ev -= 2 as libc::c_int * flipped
        } else if ev >= 0 as libc::c_int {
            /* He passes, check if SQ1 is feasible for me */
            /* I'm ahead, so EV will increase by at least 2 */
            ev += 2 as libc::c_int;
            if ev < beta {
                /* Only bother if not certain fail-high */
                ev +=
                    2 as libc::c_int *
                        CountFlips_bitboard[(sq1 - 11 as libc::c_int) as
                                                usize].expect("non-null function pointer")(bb_flips.high,
                                                                                           bb_flips.low)
            }
        } else if ev < beta {
            /* Only bother if not fail-high already */
            flipped =
                CountFlips_bitboard[(sq1 - 11 as libc::c_int) as
                                        usize].expect("non-null function pointer")(bb_flips.high,
                                                                                   bb_flips.low);
            if flipped != 0 as libc::c_int {
                /* ELSE: SQ1 will end up empty, game over */
                /* SQ1 feasible for me, game over */
                ev += 2 as libc::c_int * (flipped + 1 as libc::c_int)
            }
        }
        /* If the second move is better than the first (if that move was legal),
           its score is the score of the position */
        if ev >= score { return ev }
    }
    /* If both SQ1 and SQ2 are illegal I have to pass,
       otherwise return the best score. */
    if score == -(12345678 as libc::c_int) {
        if pass_legal == 0 {
            /* Two empty squares */
            if disc_diff > 0 as libc::c_int {
                return disc_diff + 2 as libc::c_int
            }
            if disc_diff < 0 as libc::c_int {
                return disc_diff - 2 as libc::c_int
            }
            return 0 as libc::c_int
        } else {
            return -solve_two_empty(opp_bits, my_bits, sq1, sq2, -beta,
                                    -alpha, -disc_diff, 0 as libc::c_int)
        }
    } else { return score };
}
unsafe extern "C" fn solve_three_empty(mut my_bits: BitBoard,
                                       mut opp_bits: BitBoard,
                                       mut sq1: libc::c_int,
                                       mut sq2: libc::c_int,
                                       mut sq3: libc::c_int,
                                       mut alpha: libc::c_int,
                                       mut beta: libc::c_int,
                                       mut disc_diff: libc::c_int,
                                       mut pass_legal: libc::c_int)
 -> libc::c_int {
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut score = -(12345678 as libc::c_int);
    let mut flipped: libc::c_int = 0;
    let mut new_disc_diff: libc::c_int = 0;
    let mut ev: libc::c_int = 0;
    nodes.lo = nodes.lo.wrapping_add(1);
    flipped = TestFlips_wrapper(sq1, my_bits, opp_bits);
    if flipped != 0 as libc::c_int {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as libc::c_int * flipped - 1 as libc::c_int;
        score =
            -solve_two_empty(new_opp_bits, bb_flips, sq2, sq3, -beta, -alpha,
                             new_disc_diff, 1 as libc::c_int);
        if score >= beta {
            return score
        } else { if score > alpha { alpha = score } }
    }
    flipped = TestFlips_wrapper(sq2, my_bits, opp_bits);
    if flipped != 0 as libc::c_int {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as libc::c_int * flipped - 1 as libc::c_int;
        ev =
            -solve_two_empty(new_opp_bits, bb_flips, sq1, sq3, -beta, -alpha,
                             new_disc_diff, 1 as libc::c_int);
        if ev >= beta {
            return ev
        } else {
            if ev > score { score = ev; if score > alpha { alpha = score } }
        }
    }
    flipped = TestFlips_wrapper(sq3, my_bits, opp_bits);
    if flipped != 0 as libc::c_int {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as libc::c_int * flipped - 1 as libc::c_int;
        ev =
            -solve_two_empty(new_opp_bits, bb_flips, sq1, sq2, -beta, -alpha,
                             new_disc_diff, 1 as libc::c_int);
        if ev >= score { return ev }
    }
    if score == -(12345678 as libc::c_int) {
        if pass_legal == 0 {
            /* Three empty squares */
            if disc_diff > 0 as libc::c_int {
                return disc_diff + 3 as libc::c_int
            }
            if disc_diff < 0 as libc::c_int {
                return disc_diff - 3 as libc::c_int
            }
            return 0 as libc::c_int
            /* Can't reach this code, only keep it for symmetry */
        } else {
            return -solve_three_empty(opp_bits, my_bits, sq1, sq2, sq3, -beta,
                                      -alpha, -disc_diff, 0 as libc::c_int)
        }
    }
    return score;
}
unsafe extern "C" fn solve_four_empty(mut my_bits: BitBoard,
                                      mut opp_bits: BitBoard,
                                      mut sq1: libc::c_int,
                                      mut sq2: libc::c_int,
                                      mut sq3: libc::c_int,
                                      mut sq4: libc::c_int,
                                      mut alpha: libc::c_int,
                                      mut beta: libc::c_int,
                                      mut disc_diff: libc::c_int,
                                      mut pass_legal: libc::c_int)
 -> libc::c_int {
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut score = -(12345678 as libc::c_int);
    let mut flipped: libc::c_int = 0;
    let mut new_disc_diff: libc::c_int = 0;
    let mut ev: libc::c_int = 0;
    nodes.lo = nodes.lo.wrapping_add(1);
    flipped = TestFlips_wrapper(sq1, my_bits, opp_bits);
    if flipped != 0 as libc::c_int {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as libc::c_int * flipped - 1 as libc::c_int;
        score =
            -solve_three_empty(new_opp_bits, bb_flips, sq2, sq3, sq4, -beta,
                               -alpha, new_disc_diff, 1 as libc::c_int);
        if score >= beta {
            return score
        } else { if score > alpha { alpha = score } }
    }
    flipped = TestFlips_wrapper(sq2, my_bits, opp_bits);
    if flipped != 0 as libc::c_int {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as libc::c_int * flipped - 1 as libc::c_int;
        ev =
            -solve_three_empty(new_opp_bits, bb_flips, sq1, sq3, sq4, -beta,
                               -alpha, new_disc_diff, 1 as libc::c_int);
        if ev >= beta {
            return ev
        } else {
            if ev > score { score = ev; if score > alpha { alpha = score } }
        }
    }
    flipped = TestFlips_wrapper(sq3, my_bits, opp_bits);
    if flipped != 0 as libc::c_int {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as libc::c_int * flipped - 1 as libc::c_int;
        ev =
            -solve_three_empty(new_opp_bits, bb_flips, sq1, sq2, sq4, -beta,
                               -alpha, new_disc_diff, 1 as libc::c_int);
        if ev >= beta {
            return ev
        } else {
            if ev > score { score = ev; if score > alpha { alpha = score } }
        }
    }
    flipped = TestFlips_wrapper(sq4, my_bits, opp_bits);
    if flipped != 0 as libc::c_int {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as libc::c_int * flipped - 1 as libc::c_int;
        ev =
            -solve_three_empty(new_opp_bits, bb_flips, sq1, sq2, sq3, -beta,
                               -alpha, new_disc_diff, 1 as libc::c_int);
        if ev >= score { return ev }
    }
    if score == -(12345678 as libc::c_int) {
        if pass_legal == 0 {
            /* Four empty squares */
            if disc_diff > 0 as libc::c_int {
                return disc_diff + 4 as libc::c_int
            }
            if disc_diff < 0 as libc::c_int {
                return disc_diff - 4 as libc::c_int
            }
            return 0 as libc::c_int
        } else {
            return -solve_four_empty(opp_bits, my_bits, sq1, sq2, sq3, sq4,
                                     -beta, -alpha, -disc_diff,
                                     0 as libc::c_int)
        }
    }
    return score;
}
unsafe extern "C" fn solve_parity(mut my_bits: BitBoard,
                                  mut opp_bits: BitBoard,
                                  mut alpha: libc::c_int,
                                  mut beta: libc::c_int,
                                  mut color: libc::c_int,
                                  mut empties: libc::c_int,
                                  mut disc_diff: libc::c_int,
                                  mut pass_legal: libc::c_int)
 -> libc::c_int {
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut score = -(12345678 as libc::c_int);
    let mut oppcol = 0 as libc::c_int + 2 as libc::c_int - color;
    let mut ev: libc::c_int = 0;
    let mut flipped: libc::c_int = 0;
    let mut new_disc_diff: libc::c_int = 0;
    let mut sq: libc::c_int = 0;
    let mut old_sq: libc::c_int = 0;
    let mut best_sq = 0 as libc::c_int;
    let mut parity_mask: libc::c_uint = 0;
    nodes.lo = nodes.lo.wrapping_add(1);
    /* Check for stability cutoff */
    if alpha >= stability_threshold[empties as usize] {
        let mut stability_bound: libc::c_int = 0;
        stability_bound =
            64 as libc::c_int -
                2 as libc::c_int *
                    count_edge_stable(oppcol, opp_bits, my_bits);
        if stability_bound <= alpha { return alpha }
        stability_bound =
            64 as libc::c_int -
                2 as libc::c_int * count_stable(oppcol, opp_bits, my_bits);
        if stability_bound < beta {
            beta = stability_bound + 1 as libc::c_int
        }
        if stability_bound <= alpha { return alpha }
    }
    /* Odd parity */
    parity_mask = region_parity;
    if region_parity != 0 as libc::c_int as libc::c_uint {
        /* Is there any region with odd parity? */
        old_sq = 0 as libc::c_int;
        sq = end_move_list[old_sq as usize].succ;
        while sq != 99 as libc::c_int {
            let mut holepar = quadrant_mask[sq as usize];
            if holepar & parity_mask != 0 {
                flipped = TestFlips_wrapper(sq, my_bits, opp_bits);
                if flipped != 0 as libc::c_int {
                    new_opp_bits.high = opp_bits.high & !bb_flips.high;
                    new_opp_bits.low = opp_bits.low & !bb_flips.low;
                    end_move_list[old_sq as usize].succ =
                        end_move_list[sq as usize].succ;
                    new_disc_diff =
                        -disc_diff - 2 as libc::c_int * flipped -
                            1 as libc::c_int;
                    if empties == 5 as libc::c_int {
                        let mut sq1 =
                            end_move_list[0 as libc::c_int as usize].succ;
                        let mut sq2 = end_move_list[sq1 as usize].succ;
                        let mut sq3 = end_move_list[sq2 as usize].succ;
                        let mut sq4 = end_move_list[sq3 as usize].succ;
                        ev =
                            -solve_four_empty(new_opp_bits, bb_flips, sq1,
                                              sq2, sq3, sq4, -beta, -alpha,
                                              new_disc_diff, 1 as libc::c_int)
                    } else {
                        region_parity ^= holepar;
                        ev =
                            -solve_parity(new_opp_bits, bb_flips, -beta,
                                          -alpha, oppcol,
                                          empties - 1 as libc::c_int,
                                          new_disc_diff, 1 as libc::c_int);
                        region_parity ^= holepar
                    }
                    end_move_list[old_sq as usize].succ = sq;
                    if ev > score {
                        if ev > alpha {
                            if ev >= beta { best_move = sq; return ev }
                            alpha = ev
                        }
                        score = ev;
                        best_sq = sq
                    }
                }
            }
            old_sq = sq;
            sq = end_move_list[sq as usize].succ
        }
    }
    /* Even parity */
    parity_mask = !parity_mask;
    old_sq = 0 as libc::c_int;
    sq = end_move_list[old_sq as usize].succ;
    while sq != 99 as libc::c_int {
        let mut holepar_0 = quadrant_mask[sq as usize];
        if holepar_0 & parity_mask != 0 {
            flipped = TestFlips_wrapper(sq, my_bits, opp_bits);
            if flipped != 0 as libc::c_int {
                new_opp_bits.high = opp_bits.high & !bb_flips.high;
                new_opp_bits.low = opp_bits.low & !bb_flips.low;
                end_move_list[old_sq as usize].succ =
                    end_move_list[sq as usize].succ;
                new_disc_diff =
                    -disc_diff - 2 as libc::c_int * flipped -
                        1 as libc::c_int;
                if empties == 5 as libc::c_int {
                    let mut sq1_0 =
                        end_move_list[0 as libc::c_int as usize].succ;
                    let mut sq2_0 = end_move_list[sq1_0 as usize].succ;
                    let mut sq3_0 = end_move_list[sq2_0 as usize].succ;
                    let mut sq4_0 = end_move_list[sq3_0 as usize].succ;
                    ev =
                        -solve_four_empty(new_opp_bits, bb_flips, sq1_0,
                                          sq2_0, sq3_0, sq4_0, -beta, -alpha,
                                          new_disc_diff, 1 as libc::c_int)
                } else {
                    region_parity ^= holepar_0;
                    ev =
                        -solve_parity(new_opp_bits, bb_flips, -beta, -alpha,
                                      oppcol, empties - 1 as libc::c_int,
                                      new_disc_diff, 1 as libc::c_int);
                    region_parity ^= holepar_0
                }
                end_move_list[old_sq as usize].succ = sq;
                if ev > score {
                    if ev > alpha {
                        if ev >= beta { best_move = sq; return ev }
                        alpha = ev
                    }
                    score = ev;
                    best_sq = sq
                }
            }
        }
        old_sq = sq;
        sq = end_move_list[sq as usize].succ
    }
    if score == -(12345678 as libc::c_int) {
        if pass_legal == 0 {
            if disc_diff > 0 as libc::c_int { return disc_diff + empties }
            if disc_diff < 0 as libc::c_int { return disc_diff - empties }
            return 0 as libc::c_int
        } else {
            return -solve_parity(opp_bits, my_bits, -beta, -alpha, oppcol,
                                 empties, -disc_diff, 0 as libc::c_int)
        }
    }
    best_move = best_sq;
    return score;
}
unsafe extern "C" fn solve_parity_hash(mut my_bits: BitBoard,
                                       mut opp_bits: BitBoard,
                                       mut alpha: libc::c_int,
                                       mut beta: libc::c_int,
                                       mut color: libc::c_int,
                                       mut empties: libc::c_int,
                                       mut disc_diff: libc::c_int,
                                       mut pass_legal: libc::c_int)
 -> libc::c_int {
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut score = -(12345678 as libc::c_int);
    let mut oppcol = 0 as libc::c_int + 2 as libc::c_int - color;
    let mut in_alpha = alpha;
    let mut ev: libc::c_int = 0;
    let mut flipped: libc::c_int = 0;
    let mut new_disc_diff: libc::c_int = 0;
    let mut sq: libc::c_int = 0;
    let mut old_sq: libc::c_int = 0;
    let mut best_sq = 0 as libc::c_int;
    let mut parity_mask: libc::c_uint = 0;
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    nodes.lo = nodes.lo.wrapping_add(1);
    find_hash(&mut entry, 1 as libc::c_int);
    if entry.draft as libc::c_int == empties &&
           entry.selectivity as libc::c_int == 0 as libc::c_int &&
           valid_move(entry.move_0[0 as libc::c_int as usize], color) != 0 &&
           entry.flags as libc::c_int & 16 as libc::c_int != 0 &&
           (entry.flags as libc::c_int & 4 as libc::c_int != 0 ||
                entry.flags as libc::c_int & 1 as libc::c_int != 0 &&
                    entry.eval >= beta ||
                entry.flags as libc::c_int & 2 as libc::c_int != 0 &&
                    entry.eval <= alpha) {
        best_move = entry.move_0[0 as libc::c_int as usize];
        return entry.eval
    }
    /* Check for stability cutoff */
    if alpha >= stability_threshold[empties as usize] {
        let mut stability_bound: libc::c_int = 0;
        stability_bound =
            64 as libc::c_int -
                2 as libc::c_int *
                    count_edge_stable(oppcol, opp_bits, my_bits);
        if stability_bound <= alpha { return alpha }
        stability_bound =
            64 as libc::c_int -
                2 as libc::c_int * count_stable(oppcol, opp_bits, my_bits);
        if stability_bound < beta {
            beta = stability_bound + 1 as libc::c_int
        }
        if stability_bound <= alpha { return alpha }
    }
    /* Odd parity. */
    parity_mask = region_parity;
    if region_parity != 0 as libc::c_int as libc::c_uint {
        /* Is there any region with odd parity? */
        old_sq = 0 as libc::c_int;
        sq = end_move_list[old_sq as usize].succ;
        while sq != 99 as libc::c_int {
            let mut holepar = quadrant_mask[sq as usize];
            if holepar & parity_mask != 0 {
                flipped = TestFlips_wrapper(sq, my_bits, opp_bits);
                if flipped != 0 as libc::c_int {
                    new_opp_bits.high = opp_bits.high & !bb_flips.high;
                    new_opp_bits.low = opp_bits.low & !bb_flips.low;
                    region_parity ^= holepar;
                    end_move_list[old_sq as usize].succ =
                        end_move_list[sq as usize].succ;
                    new_disc_diff =
                        -disc_diff - 2 as libc::c_int * flipped -
                            1 as libc::c_int;
                    ev =
                        -solve_parity(new_opp_bits, bb_flips, -beta, -alpha,
                                      oppcol, empties - 1 as libc::c_int,
                                      new_disc_diff, 1 as libc::c_int);
                    region_parity ^= holepar;
                    end_move_list[old_sq as usize].succ = sq;
                    if ev > score {
                        score = ev;
                        if ev > alpha {
                            if ev >= beta {
                                best_move = sq;
                                add_hash(1 as libc::c_int, score, best_move,
                                         16 as libc::c_int | 1 as libc::c_int,
                                         empties, 0 as libc::c_int);
                                return score
                            }
                            alpha = ev
                        }
                        best_sq = sq
                    }
                }
            }
            old_sq = sq;
            sq = end_move_list[sq as usize].succ
        }
    }
    /* Even parity. */
    parity_mask = !parity_mask;
    old_sq = 0 as libc::c_int;
    sq = end_move_list[old_sq as usize].succ;
    while sq != 99 as libc::c_int {
        let mut holepar_0 = quadrant_mask[sq as usize];
        if holepar_0 & parity_mask != 0 {
            flipped = TestFlips_wrapper(sq, my_bits, opp_bits);
            if flipped != 0 as libc::c_int {
                new_opp_bits.high = opp_bits.high & !bb_flips.high;
                new_opp_bits.low = opp_bits.low & !bb_flips.low;
                region_parity ^= holepar_0;
                end_move_list[old_sq as usize].succ =
                    end_move_list[sq as usize].succ;
                new_disc_diff =
                    -disc_diff - 2 as libc::c_int * flipped -
                        1 as libc::c_int;
                ev =
                    -solve_parity(new_opp_bits, bb_flips, -beta, -alpha,
                                  oppcol, empties - 1 as libc::c_int,
                                  new_disc_diff, 1 as libc::c_int);
                region_parity ^= holepar_0;
                end_move_list[old_sq as usize].succ = sq;
                if ev > score {
                    score = ev;
                    if ev > alpha {
                        if ev >= beta {
                            best_move = sq;
                            add_hash(1 as libc::c_int, score, best_move,
                                     16 as libc::c_int | 1 as libc::c_int,
                                     empties, 0 as libc::c_int);
                            return score
                        }
                        alpha = ev
                    }
                    best_sq = sq
                }
            }
        }
        old_sq = sq;
        sq = end_move_list[sq as usize].succ
    }
    if score == -(12345678 as libc::c_int) {
        if pass_legal == 0 {
            if disc_diff > 0 as libc::c_int { return disc_diff + empties }
            if disc_diff < 0 as libc::c_int { return disc_diff - empties }
            return 0 as libc::c_int
        } else {
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2;
            score =
                -solve_parity_hash(opp_bits, my_bits, -beta, -alpha, oppcol,
                                   empties, -disc_diff, 0 as libc::c_int);
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2
        }
    } else {
        best_move = best_sq;
        if score > in_alpha {
            add_hash(1 as libc::c_int, score, best_move,
                     16 as libc::c_int | 4 as libc::c_int, empties,
                     0 as libc::c_int);
        } else {
            add_hash(1 as libc::c_int, score, best_move,
                     16 as libc::c_int | 2 as libc::c_int, empties,
                     0 as libc::c_int);
        }
    }
    return score;
}
unsafe extern "C" fn solve_parity_hash_high(mut my_bits: BitBoard,
                                            mut opp_bits: BitBoard,
                                            mut alpha: libc::c_int,
                                            mut beta: libc::c_int,
                                            mut color: libc::c_int,
                                            mut empties: libc::c_int,
                                            mut disc_diff: libc::c_int,
                                            mut pass_legal: libc::c_int)
 -> libc::c_int {
    /* Move bonuses without and with parity for the squares.
       These are only used when sorting moves in the 9-12 empties
       range and were automatically tuned by OPTIMIZE. */
    static mut move_bonus: [[libc::c_uchar; 128]; 2] =
        [[0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          24 as libc::c_int as libc::c_uchar,
          1 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          25 as libc::c_int as libc::c_uchar,
          25 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          1 as libc::c_int as libc::c_uchar,
          24 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          1 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          1 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          25 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          25 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          25 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          25 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          1 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          1 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          24 as libc::c_int as libc::c_uchar,
          1 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          25 as libc::c_int as libc::c_uchar,
          25 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          1 as libc::c_int as libc::c_uchar,
          24 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar],
         [0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          86 as libc::c_int as libc::c_uchar,
          122 as libc::c_int as libc::c_uchar,
          125 as libc::c_int as libc::c_uchar,
          125 as libc::c_int as libc::c_uchar,
          122 as libc::c_int as libc::c_uchar,
          86 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          86 as libc::c_int as libc::c_uchar,
          117 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          117 as libc::c_int as libc::c_uchar,
          86 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          122 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          122 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          125 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          125 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          125 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          125 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          122 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          122 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          86 as libc::c_int as libc::c_uchar,
          117 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          117 as libc::c_int as libc::c_uchar,
          86 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          86 as libc::c_int as libc::c_uchar,
          122 as libc::c_int as libc::c_uchar,
          125 as libc::c_int as libc::c_uchar,
          125 as libc::c_int as libc::c_uchar,
          122 as libc::c_int as libc::c_uchar,
          86 as libc::c_int as libc::c_uchar,
          128 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar,
          0 as libc::c_int as libc::c_uchar]];
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut best_new_my_bits = BitBoard{high: 0, low: 0,};
    let mut best_new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut i: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut in_alpha = alpha;
    let mut oppcol = 0 as libc::c_int + 2 as libc::c_int - color;
    let mut flipped: libc::c_int = 0;
    let mut best_flipped: libc::c_int = 0;
    let mut new_disc_diff: libc::c_int = 0;
    let mut ev: libc::c_int = 0;
    let mut hash_move: libc::c_int = 0;
    let mut moves: libc::c_int = 0;
    let mut parity: libc::c_int = 0;
    let mut best_value: libc::c_int = 0;
    let mut best_index: libc::c_int = 0;
    let mut pred: libc::c_int = 0;
    let mut succ: libc::c_int = 0;
    let mut sq: libc::c_int = 0;
    let mut old_sq: libc::c_int = 0;
    let mut best_sq = 0 as libc::c_int;
    let mut move_order: [libc::c_int; 64] = [0; 64];
    let mut goodness: [libc::c_int; 64] = [0; 64];
    let mut diff1: libc::c_uint = 0;
    let mut diff2: libc::c_uint = 0;
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    nodes.lo = nodes.lo.wrapping_add(1);
    hash_move = -(1 as libc::c_int);
    find_hash(&mut entry, 1 as libc::c_int);
    if entry.draft as libc::c_int == empties {
        if entry.selectivity as libc::c_int == 0 as libc::c_int &&
               entry.flags as libc::c_int & 16 as libc::c_int != 0 &&
               valid_move(entry.move_0[0 as libc::c_int as usize], color) != 0
               &&
               (entry.flags as libc::c_int & 4 as libc::c_int != 0 ||
                    entry.flags as libc::c_int & 1 as libc::c_int != 0 &&
                        entry.eval >= beta ||
                    entry.flags as libc::c_int & 2 as libc::c_int != 0 &&
                        entry.eval <= alpha) {
            best_move = entry.move_0[0 as libc::c_int as usize];
            return entry.eval
        }
    }
    /* Check for stability cutoff */
    if alpha >= stability_threshold[empties as usize] {
        let mut stability_bound: libc::c_int = 0;
        stability_bound =
            64 as libc::c_int -
                2 as libc::c_int *
                    count_edge_stable(oppcol, opp_bits, my_bits);
        if stability_bound <= alpha { return alpha }
        stability_bound =
            64 as libc::c_int -
                2 as libc::c_int * count_stable(oppcol, opp_bits, my_bits);
        if stability_bound < beta {
            beta = stability_bound + 1 as libc::c_int
        }
        if stability_bound <= alpha { return alpha }
    }
    /* Calculate goodness values for all moves */
    moves = 0 as libc::c_int;
    best_value = -(12345678 as libc::c_int);
    best_index = 0 as libc::c_int;
    best_flipped = 0 as libc::c_int;
    old_sq = 0 as libc::c_int;
    sq = end_move_list[old_sq as usize].succ;
    while sq != 99 as libc::c_int {
        flipped = TestFlips_wrapper(sq, my_bits, opp_bits);
        if flipped != 0 as libc::c_int {
            nodes.lo = nodes.lo.wrapping_add(1);
            new_opp_bits.high = opp_bits.high & !bb_flips.high;
            new_opp_bits.low = opp_bits.low & !bb_flips.low;
            end_move_list[old_sq as usize].succ =
                end_move_list[sq as usize].succ;
            if quadrant_mask[sq as usize] & region_parity != 0 {
                parity = 1 as libc::c_int
            } else { parity = 0 as libc::c_int }
            goodness[moves as usize] =
                move_bonus[parity as usize][sq as usize] as libc::c_int;
            if sq == hash_move {
                goodness[moves as usize] += 128 as libc::c_int
            }
            goodness[moves as usize] -=
                weighted_mobility(new_opp_bits, bb_flips);
            if goodness[moves as usize] > best_value {
                best_value = goodness[moves as usize];
                best_index = moves;
                best_new_my_bits = bb_flips;
                best_new_opp_bits = new_opp_bits;
                best_flipped = flipped
            }
            end_move_list[old_sq as usize].succ = sq;
            move_order[moves as usize] = sq;
            moves += 1
        }
        old_sq = sq;
        sq = end_move_list[sq as usize].succ
    }
    /* Maybe there aren't any legal moves */
    if moves == 0 as libc::c_int {
        /* I have to pass */
        if pass_legal == 0 {
            /* Last move also pass, game over */
            if disc_diff > 0 as libc::c_int { return disc_diff + empties }
            if disc_diff < 0 as libc::c_int { return disc_diff - empties }
            return 0 as libc::c_int
        } else {
            /* Opponent gets the chance to play */
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2;
            score =
                -solve_parity_hash_high(opp_bits, my_bits, -beta, -alpha,
                                        oppcol, empties, -disc_diff,
                                        0 as libc::c_int);
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2;
            return score
        }
    }
    /* Try move with highest goodness value */
    sq = move_order[best_index as usize];
    DoFlips_hash(sq, color);
    board[sq as usize] = color;
    diff1 = hash_update1 ^ hash_put_value1[color as usize][sq as usize];
    diff2 = hash_update2 ^ hash_put_value2[color as usize][sq as usize];
    hash1 ^= diff1;
    hash2 ^= diff2;
    region_parity ^= quadrant_mask[sq as usize];
    pred = end_move_list[sq as usize].pred;
    succ = end_move_list[sq as usize].succ;
    end_move_list[pred as usize].succ = succ;
    end_move_list[succ as usize].pred = pred;
    new_disc_diff =
        -disc_diff - 2 as libc::c_int * best_flipped - 1 as libc::c_int;
    if empties <= 8 as libc::c_int + 1 as libc::c_int {
        score =
            -solve_parity_hash(best_new_opp_bits, best_new_my_bits, -beta,
                               -alpha, oppcol, empties - 1 as libc::c_int,
                               new_disc_diff, 1 as libc::c_int)
    } else {
        score =
            -solve_parity_hash_high(best_new_opp_bits, best_new_my_bits,
                                    -beta, -alpha, oppcol,
                                    empties - 1 as libc::c_int, new_disc_diff,
                                    1 as libc::c_int)
    }
    UndoFlips(best_flipped, oppcol);
    hash1 ^= diff1;
    hash2 ^= diff2;
    board[sq as usize] = 1 as libc::c_int;
    region_parity ^= quadrant_mask[sq as usize];
    end_move_list[pred as usize].succ = sq;
    end_move_list[succ as usize].pred = sq;
    best_sq = sq;
    if score > alpha {
        if score >= beta {
            best_move = best_sq;
            add_hash(1 as libc::c_int, score, best_move,
                     16 as libc::c_int | 1 as libc::c_int, empties,
                     0 as libc::c_int);
            return score
        }
        alpha = score
    }
    /* Play through the rest of the moves */
    move_order[best_index as usize] = move_order[0 as libc::c_int as usize];
    goodness[best_index as usize] = goodness[0 as libc::c_int as usize];
    i = 1 as libc::c_int;
    while i < moves {
        let mut j: libc::c_int = 0;
        best_value = goodness[i as usize];
        best_index = i;
        j = i + 1 as libc::c_int;
        while j < moves {
            if goodness[j as usize] > best_value {
                best_value = goodness[j as usize];
                best_index = j
            }
            j += 1
        }
        sq = move_order[best_index as usize];
        move_order[best_index as usize] = move_order[i as usize];
        goodness[best_index as usize] = goodness[i as usize];
        flipped = TestFlips_wrapper(sq, my_bits, opp_bits);
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        DoFlips_hash(sq, color);
        board[sq as usize] = color;
        diff1 = hash_update1 ^ hash_put_value1[color as usize][sq as usize];
        diff2 = hash_update2 ^ hash_put_value2[color as usize][sq as usize];
        hash1 ^= diff1;
        hash2 ^= diff2;
        region_parity ^= quadrant_mask[sq as usize];
        pred = end_move_list[sq as usize].pred;
        succ = end_move_list[sq as usize].succ;
        end_move_list[pred as usize].succ = succ;
        end_move_list[succ as usize].pred = pred;
        new_disc_diff =
            -disc_diff - 2 as libc::c_int * flipped - 1 as libc::c_int;
        if empties <= 8 as libc::c_int {
            /* Fail-high for opp is likely. */
            ev =
                -solve_parity_hash(new_opp_bits, bb_flips, -beta, -alpha,
                                   oppcol, empties - 1 as libc::c_int,
                                   new_disc_diff, 1 as libc::c_int)
        } else {
            ev =
                -solve_parity_hash_high(new_opp_bits, bb_flips, -beta, -alpha,
                                        oppcol, empties - 1 as libc::c_int,
                                        new_disc_diff, 1 as libc::c_int)
        }
        region_parity ^= quadrant_mask[sq as usize];
        UndoFlips(flipped, oppcol);
        hash1 ^= diff1;
        hash2 ^= diff2;
        board[sq as usize] = 1 as libc::c_int;
        end_move_list[pred as usize].succ = sq;
        end_move_list[succ as usize].pred = sq;
        if ev > score {
            score = ev;
            if ev > alpha {
                if ev >= beta {
                    best_move = sq;
                    add_hash(1 as libc::c_int, score, best_move,
                             16 as libc::c_int | 1 as libc::c_int, empties,
                             0 as libc::c_int);
                    return score
                }
                alpha = ev
            }
            best_sq = sq
        }
        i += 1
    }
    best_move = best_sq;
    if score > in_alpha {
        add_hash(1 as libc::c_int, score, best_move,
                 16 as libc::c_int | 4 as libc::c_int, empties,
                 0 as libc::c_int);
    } else {
        add_hash(1 as libc::c_int, score, best_move,
                 16 as libc::c_int | 2 as libc::c_int, empties,
                 0 as libc::c_int);
    }
    return score;
}
/*
  END_SOLVE
  The search itself. Assumes relevant data structures have been set up with
  PREPARE_TO_SOLVE(). Returns difference between disc count for
  COLOR and disc count for the opponent of COLOR.
*/
unsafe extern "C" fn end_solve(mut my_bits: BitBoard, mut opp_bits: BitBoard,
                               mut alpha: libc::c_int, mut beta: libc::c_int,
                               mut color: libc::c_int,
                               mut empties: libc::c_int,
                               mut discdiff: libc::c_int,
                               mut prevmove: libc::c_int) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if empties <= 8 as libc::c_int {
        result =
            solve_parity(my_bits, opp_bits, alpha, beta, color, empties,
                         discdiff, prevmove)
    } else {
        result =
            solve_parity_hash_high(my_bits, opp_bits, alpha, beta, color,
                                   empties, discdiff, prevmove)
    }
    return result;
}
/*
  UPDATE_BEST_LIST
*/
unsafe extern "C" fn update_best_list(mut best_list: *mut libc::c_int,
                                      mut move_0: libc::c_int,
                                      mut best_list_index: libc::c_int,
                                      mut best_list_length: *mut libc::c_int,
                                      mut verbose: libc::c_int) {
    let mut i: libc::c_int = 0;
    verbose = 0 as libc::c_int;
    if verbose != 0 {
        printf(b"move=%2d  index=%d  length=%d      \x00" as *const u8 as
                   *const libc::c_char, move_0, best_list_index,
               *best_list_length);
        printf(b"Before:  \x00" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            printf(b"%2d \x00" as *const u8 as *const libc::c_char,
                   *best_list.offset(i as isize));
            i += 1
        }
    }
    if best_list_index < *best_list_length {
        i = best_list_index;
        while i >= 1 as libc::c_int {
            *best_list.offset(i as isize) =
                *best_list.offset((i - 1 as libc::c_int) as isize);
            i -= 1
        }
    } else {
        i = 3 as libc::c_int;
        while i >= 1 as libc::c_int {
            *best_list.offset(i as isize) =
                *best_list.offset((i - 1 as libc::c_int) as isize);
            i -= 1
        }
        if *best_list_length < 4 as libc::c_int { *best_list_length += 1 }
    }
    *best_list.offset(0 as libc::c_int as isize) = move_0;
    if verbose != 0 {
        printf(b"      After:  \x00" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            printf(b"%2d \x00" as *const u8 as *const libc::c_char,
                   *best_list.offset(i as isize));
            i += 1
        }
        puts(b"\x00" as *const u8 as *const libc::c_char);
    };
}
/*
  END_TREE_SEARCH
  Plain nega-scout with fastest-first move ordering.
*/
unsafe extern "C" fn end_tree_search(mut level: libc::c_int,
                                     mut max_depth: libc::c_int,
                                     mut my_bits: BitBoard,
                                     mut opp_bits: BitBoard,
                                     mut side_to_move: libc::c_int,
                                     mut alpha: libc::c_int,
                                     mut beta: libc::c_int,
                                     mut selectivity: libc::c_int,
                                     mut selective_cutoff: *mut libc::c_int,
                                     mut void_legal: libc::c_int)
 -> libc::c_int {
    static mut buffer: [libc::c_char; 16] = [0; 16];
    let mut node_val: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut empties: libc::c_int = 0;
    let mut disk_diff: libc::c_int = 0;
    let mut previous_move: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut curr_val: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut move_0: libc::c_int = 0;
    let mut hash_hit: libc::c_int = 0;
    let mut move_index: libc::c_int = 0;
    let mut remains: libc::c_int = 0;
    let mut exp_depth: libc::c_int = 0;
    let mut pre_depth: libc::c_int = 0;
    let mut update_pv: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut use_hash: libc::c_int = 0;
    let mut my_discs: libc::c_int = 0;
    let mut opp_discs: libc::c_int = 0;
    let mut curr_alpha: libc::c_int = 0;
    let mut pre_search_done: libc::c_int = 0;
    let mut mobility: libc::c_int = 0;
    let mut threshold: libc::c_int = 0;
    let mut best_list_index: libc::c_int = 0;
    let mut best_list_length: libc::c_int = 0;
    let mut best_list: [libc::c_int; 4] = [0; 4];
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    let mut mid_entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    let mut stability_bound: libc::c_int = 0;
    if level == 0 as libc::c_int {
        sprintf(buffer.as_mut_ptr(),
                b"[%d,%d]:\x00" as *const u8 as *const libc::c_char, alpha,
                beta);
        clear_sweep();
    }
    remains = max_depth - level;
    *selective_cutoff = 0 as libc::c_int;
    /* Always (almost) check for stability cutoff in this region of search */
    if alpha >= 24 as libc::c_int {
        stability_bound =
            64 as libc::c_int -
                2 as libc::c_int *
                    count_edge_stable(0 as libc::c_int + 2 as libc::c_int -
                                          side_to_move, opp_bits, my_bits);
        if stability_bound <= alpha {
            pv_depth[level as usize] = level;
            return alpha
        }
        stability_bound =
            64 as libc::c_int -
                2 as libc::c_int *
                    count_stable(0 as libc::c_int + 2 as libc::c_int -
                                     side_to_move, opp_bits, my_bits);
        if stability_bound < beta {
            beta = stability_bound + 1 as libc::c_int
        }
        if stability_bound <= alpha {
            pv_depth[level as usize] = level;
            return alpha
        }
    }
    /* Check if the low-level code is to be invoked */
    my_discs = piece_count[side_to_move as usize][disks_played as usize];
    opp_discs =
        piece_count[(0 as libc::c_int + 2 as libc::c_int - side_to_move) as
                        usize][disks_played as usize];
    empties = 64 as libc::c_int - my_discs - opp_discs;
    if remains <= 12 as libc::c_int {
        disk_diff = my_discs - opp_discs;
        if void_legal != 0 {
            /* Is PASS legal or was last move a pass? */
            previous_move = 44 as libc::c_int
        } else {
            previous_move = 0 as libc::c_int
        } /* d4, of course impossible */
        prepare_to_solve(board.as_mut_ptr());
        result =
            end_solve(my_bits, opp_bits, alpha, beta, side_to_move, empties,
                      disk_diff, previous_move);
        pv_depth[level as usize] = level + 1 as libc::c_int;
        pv[level as usize][level as usize] = best_move;
        if level == 0 as libc::c_int && get_ponder_move() == 0 {
            send_sweep(b"%-10s \x00" as *const u8 as *const libc::c_char,
                       buffer.as_mut_ptr());
            send_sweep(b"%c%c\x00" as *const u8 as *const libc::c_char,
                       'a' as i32 + best_move % 10 as libc::c_int -
                           1 as libc::c_int,
                       '0' as i32 + best_move / 10 as libc::c_int);
            if result <= alpha {
                send_sweep(b"<%d\x00" as *const u8 as *const libc::c_char,
                           result + 1 as libc::c_int);
            } else if result >= beta {
                send_sweep(b">%d\x00" as *const u8 as *const libc::c_char,
                           result - 1 as libc::c_int);
            } else {
                send_sweep(b"=%d\x00" as *const u8 as *const libc::c_char,
                           result);
            }
        }
        return result
    }
    /* Otherwise normal search */
    nodes.lo = nodes.lo.wrapping_add(1);
    use_hash = 1 as libc::c_int;
    if use_hash != 0 {
        /* Check for endgame hash table move */
        find_hash(&mut entry, 1 as libc::c_int);
        if entry.draft as libc::c_int == remains &&
               entry.selectivity as libc::c_int <= selectivity &&
               valid_move(entry.move_0[0 as libc::c_int as usize],
                          side_to_move) != 0 &&
               entry.flags as libc::c_int & 16 as libc::c_int != 0 &&
               (entry.flags as libc::c_int & 4 as libc::c_int != 0 ||
                    entry.flags as libc::c_int & 1 as libc::c_int != 0 &&
                        entry.eval >= beta ||
                    entry.flags as libc::c_int & 2 as libc::c_int != 0 &&
                        entry.eval <= alpha) {
            pv[level as usize][level as usize] =
                entry.move_0[0 as libc::c_int as usize];
            pv_depth[level as usize] = level + 1 as libc::c_int;
            if level == 0 as libc::c_int && get_ponder_move() == 0 {
                /* Output some stats */
                send_sweep(b"%c%c\x00" as *const u8 as *const libc::c_char,
                           'a' as i32 +
                               entry.move_0[0 as libc::c_int as usize] %
                                   10 as libc::c_int - 1 as libc::c_int,
                           '0' as i32 +
                               entry.move_0[0 as libc::c_int as usize] /
                                   10 as libc::c_int);
                if entry.flags as libc::c_int & 16 as libc::c_int != 0 &&
                       entry.flags as libc::c_int & 4 as libc::c_int != 0 {
                    send_sweep(b"=%d\x00" as *const u8 as *const libc::c_char,
                               entry.eval);
                } else if entry.flags as libc::c_int & 16 as libc::c_int != 0
                              &&
                              entry.flags as libc::c_int & 1 as libc::c_int !=
                                  0 {
                    send_sweep(b">%d\x00" as *const u8 as *const libc::c_char,
                               entry.eval - 1 as libc::c_int);
                } else {
                    send_sweep(b"<%d\x00" as *const u8 as *const libc::c_char,
                               entry.eval + 1 as libc::c_int);
                }
                fflush(stdout);
            }
            if entry.selectivity as libc::c_int > 0 as libc::c_int {
                *selective_cutoff = 1 as libc::c_int
            }
            return entry.eval
        }
        hash_hit =
            (entry.draft as libc::c_int != 0 as libc::c_int) as libc::c_int;
        /* If not any such found, check for a midgame hash move */
        find_hash(&mut mid_entry, 0 as libc::c_int);
        if mid_entry.draft as libc::c_int != 0 as libc::c_int &&
               mid_entry.flags as libc::c_int & 8 as libc::c_int != 0 {
            if level <= 4 as libc::c_int ||
                   mid_entry.flags as libc::c_int &
                       (4 as libc::c_int | 1 as libc::c_int) != 0 {
                /* Give the midgame move full priority if we're are the root
                   of the tree, no endgame hash move was found and the position
                   isn't in the wipeout zone. */
                if level == 0 as libc::c_int && hash_hit == 0 &&
                       mid_entry.eval < 60 as libc::c_int * 128 as libc::c_int
                   {
                    entry = mid_entry;
                    hash_hit = 1 as libc::c_int
                }
            }
        }
    }
    /* Use endgame multi-prob-cut to selectively prune the tree */
    if 1 as libc::c_int != 0 && level > 2 as libc::c_int &&
           selectivity > 0 as libc::c_int {
        let mut cut: libc::c_int = 0;
        cut = 0 as libc::c_int;
        while cut < use_end_cut[disks_played as usize] {
            let mut shallow_remains =
                end_mpc_depth[disks_played as usize][cut as usize];
            let mut mpc_bias =
                ceil(end_mean[disks_played as usize][shallow_remains as usize]
                         as libc::c_double * 128.0f64) as libc::c_int;
            let mut mpc_window =
                ceil(end_sigma[disks_played as
                                   usize][shallow_remains as usize] as
                         libc::c_double * end_percentile[selectivity as usize]
                         * 128.0f64) as libc::c_int;
            let mut beta_bound =
                128 as libc::c_int * beta + mpc_bias + mpc_window;
            let mut alpha_bound =
                128 as libc::c_int * alpha + mpc_bias - mpc_window;
            let mut shallow_val =
                tree_search(level, level + shallow_remains, side_to_move,
                            alpha_bound, beta_bound, use_hash,
                            0 as libc::c_int, void_legal);
            if shallow_val >= beta_bound {
                if use_hash != 0 {
                    add_hash(1 as libc::c_int, alpha,
                             pv[level as usize][level as usize],
                             16 as libc::c_int | 1 as libc::c_int, remains,
                             selectivity);
                }
                *selective_cutoff = 1 as libc::c_int;
                return beta
            }
            if shallow_val <= alpha_bound {
                if use_hash != 0 {
                    add_hash(1 as libc::c_int, beta,
                             pv[level as usize][level as usize],
                             16 as libc::c_int | 2 as libc::c_int, remains,
                             selectivity);
                }
                *selective_cutoff = 1 as libc::c_int;
                return alpha
            }
            cut += 1
        }
    }
    /* Determine the depth of the shallow search used to find
       achieve good move sorting */
    if remains >= 15 as libc::c_int {
        if remains >= 20 as libc::c_int {
            if remains >= 24 as libc::c_int {
                if remains >= 30 as libc::c_int {
                    pre_depth = 6 as libc::c_int
                } else { pre_depth = 4 as libc::c_int }
            } else { pre_depth = 3 as libc::c_int }
        } else { pre_depth = 2 as libc::c_int }
    } else { pre_depth = 1 as libc::c_int }
    if level == 0 as libc::c_int {
        /* Deeper pre-search from the root */
        pre_depth += 2 as libc::c_int;
        if pre_depth % 2 as libc::c_int == 1 as libc::c_int {
            /* Avoid odd depths from the root */
            pre_depth += 1
        }
    }
    /* The nega-scout search */
    exp_depth = remains;
    first = 1 as libc::c_int;
    best = -(12345678 as libc::c_int);
    pre_search_done = 0 as libc::c_int;
    curr_alpha = alpha;
    /* Initialize the move list and check the hash table move list */
    move_count[disks_played as usize] = 0 as libc::c_int;
    best_list_length = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        best_list[i as usize] = 0 as libc::c_int;
        i += 1
    }
    if hash_hit != 0 {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            if valid_move(entry.move_0[i as usize], side_to_move) != 0 {
                let fresh0 = best_list_length;
                best_list_length = best_list_length + 1;
                best_list[fresh0 as usize] = entry.move_0[i as usize];
                /* Check for ETC among the hash table moves */
                if use_hash != 0 &&
                       make_move(side_to_move, entry.move_0[i as usize],
                                 1 as libc::c_int) != 0 as libc::c_int {
                    let mut etc_entry =
                        HashEntry{key1: 0,
                                  key2: 0,
                                  eval: 0,
                                  move_0: [0; 4],
                                  draft: 0,
                                  selectivity: 0,
                                  flags: 0,};
                    find_hash(&mut etc_entry, 1 as libc::c_int);
                    if etc_entry.flags as libc::c_int & 16 as libc::c_int != 0
                           &&
                           etc_entry.draft as libc::c_int ==
                               empties - 1 as libc::c_int &&
                           etc_entry.selectivity as libc::c_int <= selectivity
                           &&
                           etc_entry.flags as libc::c_int &
                               (2 as libc::c_int | 4 as libc::c_int) != 0 &&
                           etc_entry.eval <= -beta {
                        /* Immediate cutoff from this move, move it up front */
                        j = best_list_length - 1 as libc::c_int;
                        while j >= 1 as libc::c_int {
                            best_list[j as usize] =
                                best_list[(j - 1 as libc::c_int) as usize];
                            j -= 1
                        }
                        best_list[0 as libc::c_int as usize] =
                            entry.move_0[i as usize]
                    }
                    unmake_move(side_to_move, entry.move_0[i as usize]);
                }
            }
            i += 1
        }
    }
    move_index = 0 as libc::c_int;
    best_list_index = 0 as libc::c_int;
    loop  {
        let mut child_selective_cutoff: libc::c_int = 0;
        let mut new_my_bits = BitBoard{high: 0, low: 0,};
        let mut new_opp_bits = BitBoard{high: 0, low: 0,};
        /* Use results of shallow searches to determine the move order */
        if best_list_index < best_list_length {
            move_0 = best_list[best_list_index as usize];
            move_count[disks_played as usize] += 1
        } else {
            if pre_search_done == 0 {
                let mut shallow_index: libc::c_int = 0;
                pre_search_done = 1 as libc::c_int;
                threshold =
                    if (60 as libc::c_int * 128 as libc::c_int) <
                           128 as libc::c_int * alpha +
                               fast_first_threshold[disks_played as
                                                        usize][pre_depth as
                                                                   usize] {
                        (60 as libc::c_int) * 128 as libc::c_int
                    } else {
                        (128 as libc::c_int * alpha) +
                            fast_first_threshold[disks_played as
                                                     usize][pre_depth as
                                                                usize]
                    };
                shallow_index = 0 as libc::c_int;
                while shallow_index < 60 as libc::c_int {
                    let mut already_checked: libc::c_int = 0;
                    move_0 =
                        sorted_move_order[disks_played as
                                              usize][shallow_index as usize];
                    already_checked = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    while j < best_list_length {
                        if move_0 == best_list[j as usize] {
                            already_checked = 1 as libc::c_int
                        }
                        j += 1
                    }
                    if already_checked == 0 &&
                           board[move_0 as usize] == 1 as libc::c_int &&
                           TestFlips_wrapper(move_0, my_bits, opp_bits) >
                               0 as libc::c_int {
                        new_opp_bits.high = opp_bits.high & !bb_flips.high;
                        new_opp_bits.low = opp_bits.low & !bb_flips.low;
                        make_move(side_to_move, move_0, 1 as libc::c_int);
                        curr_val = 0 as libc::c_int;
                        /* Enhanced Transposition Cutoff: It's a good idea to
                           transpose back into a position in the hash table. */
                        if use_hash != 0 {
                            let mut etc_entry_0 =
                                HashEntry{key1: 0,
                                          key2: 0,
                                          eval: 0,
                                          move_0: [0; 4],
                                          draft: 0,
                                          selectivity: 0,
                                          flags: 0,};
                            find_hash(&mut etc_entry_0, 1 as libc::c_int);
                            if etc_entry_0.flags as libc::c_int &
                                   16 as libc::c_int != 0 &&
                                   etc_entry_0.draft as libc::c_int ==
                                       empties - 1 as libc::c_int {
                                curr_val += 384 as libc::c_int;
                                if etc_entry_0.selectivity as libc::c_int <=
                                       selectivity {
                                    if etc_entry_0.flags as libc::c_int &
                                           (2 as libc::c_int |
                                                4 as libc::c_int) != 0 &&
                                           etc_entry_0.eval <= -beta {
                                        curr_val = 10000000 as libc::c_int
                                    }
                                    if etc_entry_0.flags as libc::c_int &
                                           1 as libc::c_int != 0 &&
                                           etc_entry_0.eval >= -alpha {
                                        curr_val -= 640 as libc::c_int
                                    }
                                }
                            }
                        }
                        /* Determine the midgame score. If it is worse than
                           alpha-8, a fail-high is likely so precision in that
                           range is not worth the extra nodes required. */
                        if curr_val != 10000000 as libc::c_int {
                            curr_val -=
                                tree_search(level + 1 as libc::c_int,
                                            level + pre_depth,
                                            0 as libc::c_int +
                                                2 as libc::c_int -
                                                side_to_move,
                                            -(12345678 as libc::c_int),
                                            (-alpha + 8 as libc::c_int) *
                                                128 as libc::c_int,
                                            1 as libc::c_int,
                                            1 as libc::c_int,
                                            1 as libc::c_int)
                        }
                        /* Make the moves which are highly likely to result in
                           fail-high in decreasing order of mobility for the
                           opponent. */
                        if curr_val > threshold ||
                               move_0 ==
                                   mid_entry.move_0[0 as libc::c_int as usize]
                           {
                            if curr_val >
                                   60 as libc::c_int * 128 as libc::c_int {
                                curr_val +=
                                    2 as libc::c_int * 1000000 as libc::c_int
                            } else { curr_val += 1000000 as libc::c_int }
                            if curr_val < 10000000 as libc::c_int {
                                mobility =
                                    bitboard_mobility(new_opp_bits, bb_flips);
                                if curr_val >
                                       2 as libc::c_int *
                                           1000000 as libc::c_int {
                                    curr_val -=
                                        2 as libc::c_int *
                                            ff_mob_factor[(disks_played -
                                                               1 as
                                                                   libc::c_int)
                                                              as usize] *
                                            mobility
                                } else {
                                    curr_val -=
                                        ff_mob_factor[(disks_played -
                                                           1 as libc::c_int)
                                                          as usize] * mobility
                                }
                            }
                        }
                        unmake_move(side_to_move, move_0);
                        evals[disks_played as usize][move_0 as usize] =
                            curr_val;
                        move_list[disks_played as
                                      usize][move_count[disks_played as usize]
                                                 as usize] = move_0;
                        move_count[disks_played as usize] += 1
                    }
                    shallow_index += 1
                }
            }
            if move_index == move_count[disks_played as usize] { break ; }
            move_0 =
                select_move(move_index, move_count[disks_played as usize])
        }
        node_val = counter_value(&mut nodes);
        if node_val - last_panic_check >= 250000.0f64 {
            /* Check for time abort */
            last_panic_check = node_val;
            check_panic_abort();
            /* Output status buffers if in interactive mode */
            if echo != 0 { display_buffers(); }
            /* Check for events */
            handle_event(1 as libc::c_int, 0 as libc::c_int,
                         1 as libc::c_int);
            if is_panic_abort() != 0 || force_return != 0 {
                return -(27000 as libc::c_int)
            }
        }
        if level == 0 as libc::c_int && get_ponder_move() == 0 {
            if first != 0 {
                send_sweep(b"%-10s \x00" as *const u8 as *const libc::c_char,
                           buffer.as_mut_ptr());
            }
            send_sweep(b"%c%c\x00" as *const u8 as *const libc::c_char,
                       'a' as i32 + move_0 % 10 as libc::c_int -
                           1 as libc::c_int,
                       '0' as i32 + move_0 / 10 as libc::c_int);
        }
        make_move(side_to_move, move_0, use_hash);
        TestFlips_wrapper(move_0, my_bits, opp_bits);
        new_my_bits = bb_flips;
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        update_pv = 0 as libc::c_int;
        if first != 0 {
            curr_val =
                -end_tree_search(level + 1 as libc::c_int, level + exp_depth,
                                 new_opp_bits, new_my_bits,
                                 0 as libc::c_int + 2 as libc::c_int -
                                     side_to_move, -beta, -curr_alpha,
                                 selectivity, &mut child_selective_cutoff,
                                 1 as libc::c_int);
            best = curr_val;
            update_pv = 1 as libc::c_int;
            if level == 0 as libc::c_int { best_end_root_move = move_0 }
        } else {
            curr_alpha = if best > curr_alpha { best } else { curr_alpha };
            curr_val =
                -end_tree_search(level + 1 as libc::c_int, level + exp_depth,
                                 new_opp_bits, new_my_bits,
                                 0 as libc::c_int + 2 as libc::c_int -
                                     side_to_move,
                                 -(curr_alpha + 1 as libc::c_int),
                                 -curr_alpha, selectivity,
                                 &mut child_selective_cutoff,
                                 1 as libc::c_int);
            if curr_val > curr_alpha && curr_val < beta {
                if selectivity > 0 as libc::c_int {
                    curr_val =
                        -end_tree_search(level + 1 as libc::c_int,
                                         level + exp_depth, new_opp_bits,
                                         new_my_bits,
                                         0 as libc::c_int + 2 as libc::c_int -
                                             side_to_move, -beta,
                                         12345678 as libc::c_int, selectivity,
                                         &mut child_selective_cutoff,
                                         1 as libc::c_int)
                } else {
                    curr_val =
                        -end_tree_search(level + 1 as libc::c_int,
                                         level + exp_depth, new_opp_bits,
                                         new_my_bits,
                                         0 as libc::c_int + 2 as libc::c_int -
                                             side_to_move, -beta, -curr_val,
                                         selectivity,
                                         &mut child_selective_cutoff,
                                         1 as libc::c_int)
                }
                if curr_val > best {
                    best = curr_val;
                    update_pv = 1 as libc::c_int;
                    if level == 0 as libc::c_int && is_panic_abort() == 0 &&
                           force_return == 0 {
                        best_end_root_move = move_0
                    }
                }
            } else if curr_val > best {
                best = curr_val;
                update_pv = 1 as libc::c_int;
                if level == 0 as libc::c_int && is_panic_abort() == 0 &&
                       force_return == 0 {
                    best_end_root_move = move_0
                }
            }
        }
        if best >= beta {
            /* The other children don't matter in this case. */
            *selective_cutoff = child_selective_cutoff
        } else if child_selective_cutoff != 0 {
            *selective_cutoff = 1 as libc::c_int
        }
        unmake_move(side_to_move, move_0);
        if is_panic_abort() != 0 || force_return != 0 {
            return -(27000 as libc::c_int)
        }
        if level == 0 as libc::c_int && get_ponder_move() == 0 {
            /* Output some stats */
            if update_pv != 0 {
                if curr_val <= alpha {
                    send_sweep(b"<%d\x00" as *const u8 as *const libc::c_char,
                               curr_val + 1 as libc::c_int);
                } else if curr_val >= beta {
                    send_sweep(b">%d\x00" as *const u8 as *const libc::c_char,
                               curr_val - 1 as libc::c_int);
                } else {
                    send_sweep(b"=%d\x00" as *const u8 as *const libc::c_char,
                               curr_val);
                    true_found = 1 as libc::c_int;
                    true_val = curr_val
                }
            }
            send_sweep(b" \x00" as *const u8 as *const libc::c_char);
            if update_pv != 0 && move_index > 0 as libc::c_int && echo != 0 {
                display_sweep(stdout);
            }
        }
        if update_pv != 0 {
            update_best_list(best_list.as_mut_ptr(), move_0, best_list_index,
                             &mut best_list_length,
                             (level == 0 as libc::c_int) as libc::c_int);
            pv[level as usize][level as usize] = move_0;
            pv_depth[level as usize] =
                pv_depth[(level + 1 as libc::c_int) as usize];
            i = level + 1 as libc::c_int;
            while i < pv_depth[(level + 1 as libc::c_int) as usize] {
                pv[level as usize][i as usize] =
                    pv[(level + 1 as libc::c_int) as usize][i as usize];
                i += 1
            }
        }
        if best >= beta {
            /* Fail high */
            if use_hash != 0 {
                add_hash_extended(1 as libc::c_int, best,
                                  best_list.as_mut_ptr(),
                                  16 as libc::c_int | 1 as libc::c_int,
                                  remains,
                                  if *selective_cutoff != 0 {
                                      selectivity
                                  } else { 0 as libc::c_int });
            }
            return best
        }
        if best_list_index >= best_list_length && update_pv == 0 &&
               best_list_length < 4 as libc::c_int {
            let fresh1 = best_list_length;
            best_list_length = best_list_length + 1;
            best_list[fresh1 as usize] = move_0
        }
        first = 0 as libc::c_int;
        move_index += 1;
        best_list_index += 1
    }
    if first == 0 {
        if use_hash != 0 {
            let mut flags = 16 as libc::c_int;
            if best > alpha {
                flags |= 4 as libc::c_int
            } else { flags |= 2 as libc::c_int }
            add_hash_extended(1 as libc::c_int, best, best_list.as_mut_ptr(),
                              flags, remains,
                              if *selective_cutoff != 0 {
                                  selectivity
                              } else { 0 as libc::c_int });
        }
        return best
    } else if void_legal != 0 {
        if use_hash != 0 {
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2
        }
        curr_val =
            -end_tree_search(level, max_depth, opp_bits, my_bits,
                             0 as libc::c_int + 2 as libc::c_int -
                                 side_to_move, -beta, -alpha, selectivity,
                             selective_cutoff, 0 as libc::c_int);
        if use_hash != 0 {
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2
        }
        return curr_val
    } else {
        pv_depth[level as usize] = level;
        my_discs = piece_count[side_to_move as usize][disks_played as usize];
        opp_discs =
            piece_count[(0 as libc::c_int + 2 as libc::c_int - side_to_move)
                            as usize][disks_played as usize];
        disk_diff = my_discs - opp_discs;
        if my_discs > opp_discs {
            return 64 as libc::c_int - 2 as libc::c_int * opp_discs
        } else if my_discs == opp_discs {
            return 0 as libc::c_int
        } else { return -(64 as libc::c_int - 2 as libc::c_int * my_discs) }
    };
}
/*
  END_TREE_WRAPPER
  Wrapper onto END_TREE_SEARCH which applies the knowledge that
  the range of valid scores is [-64,+64].  Komi, if any, is accounted for.
*/
unsafe extern "C" fn end_tree_wrapper(mut level: libc::c_int,
                                      mut max_depth: libc::c_int,
                                      mut side_to_move: libc::c_int,
                                      mut alpha: libc::c_int,
                                      mut beta: libc::c_int,
                                      mut selectivity: libc::c_int,
                                      mut void_legal: libc::c_int)
 -> libc::c_int {
    let mut selective_cutoff: libc::c_int = 0;
    let mut my_bits = BitBoard{high: 0, low: 0,};
    let mut opp_bits = BitBoard{high: 0, low: 0,};
    init_mmx();
    set_bitboards(board.as_mut_ptr(), side_to_move, &mut my_bits,
                  &mut opp_bits);
    return end_tree_search(level, max_depth, my_bits, opp_bits, side_to_move,
                           (if alpha - komi_shift > -(64 as libc::c_int) {
                                (alpha) - komi_shift
                            } else { -(64 as libc::c_int) }),
                           (if beta - komi_shift < 64 as libc::c_int {
                                (beta) - komi_shift
                            } else { 64 as libc::c_int }), selectivity,
                           &mut selective_cutoff, void_legal) + komi_shift;
}
/*
   FULL_EXPAND_PV
   Pad the PV with optimal moves in the low-level phase.
*/
unsafe extern "C" fn full_expand_pv(mut side_to_move: libc::c_int,
                                    mut selectivity: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut pass_count: libc::c_int = 0;
    let mut new_pv_depth: libc::c_int = 0;
    let mut new_pv: [libc::c_int; 61] = [0; 61];
    let mut new_side_to_move: [libc::c_int; 61] = [0; 61];
    new_pv_depth = 0 as libc::c_int;
    pass_count = 0 as libc::c_int;
    while pass_count < 2 as libc::c_int {
        let mut move_0: libc::c_int = 0;
        generate_all(side_to_move);
        if move_count[disks_played as usize] > 0 as libc::c_int {
            let mut empties =
                64 as libc::c_int - disc_count(0 as libc::c_int) -
                    disc_count(2 as libc::c_int);
            end_tree_wrapper(new_pv_depth, empties, side_to_move,
                             -(64 as libc::c_int), 64 as libc::c_int,
                             selectivity, 1 as libc::c_int);
            move_0 = pv[new_pv_depth as usize][new_pv_depth as usize];
            new_pv[new_pv_depth as usize] = move_0;
            new_side_to_move[new_pv_depth as usize] = side_to_move;
            make_move(side_to_move, move_0, 1 as libc::c_int);
            new_pv_depth += 1
        } else {
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2;
            pass_count += 1
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
  SEND_SOLVE_STATUS
  Displays endgame results - partial or full.
*/
unsafe extern "C" fn send_solve_status(mut empties: libc::c_int,
                                       mut side_to_move: libc::c_int,
                                       mut eval_info: *mut EvaluationType) {
    let mut eval_str = 0 as *mut libc::c_char;
    let mut node_val: libc::c_double = 0.;
    set_current_eval(*eval_info);
    clear_status();
    send_status(b"-->  %2d  \x00" as *const u8 as *const libc::c_char,
                empties);
    eval_str = produce_eval_text(*eval_info, 1 as libc::c_int);
    send_status(b"%-10s  \x00" as *const u8 as *const libc::c_char, eval_str);
    free(eval_str as *mut libc::c_void);
    node_val = counter_value(&mut nodes);
    send_status_nodes(node_val);
    if get_ponder_move() != 0 {
        send_status(b"{%c%c} \x00" as *const u8 as *const libc::c_char,
                    'a' as i32 + get_ponder_move() % 10 as libc::c_int -
                        1 as libc::c_int,
                    '0' as i32 + get_ponder_move() / 10 as libc::c_int);
    }
    send_status_pv(pv[0 as libc::c_int as usize].as_mut_ptr(), empties);
    send_status_time(get_elapsed_time());
    if get_elapsed_time() > 0.0001f64 {
        send_status(b"%6.0f %s  \x00" as *const u8 as *const libc::c_char,
                    node_val / (get_elapsed_time() + 0.0001f64),
                    b"nps\x00" as *const u8 as *const libc::c_char);
    };
}
/*
  END_GAME
  Provides an interface to the fast endgame solver.
*/
#[no_mangle]
pub unsafe extern "C" fn end_game(mut side_to_move: libc::c_int,
                                  mut wld: libc::c_int,
                                  mut force_echo: libc::c_int,
                                  mut allow_book: libc::c_int,
                                  mut komi: libc::c_int,
                                  mut eval_info: *mut EvaluationType)
 -> libc::c_int {
    let mut current_confidence: libc::c_double = 0.;
    let mut solve_status = WIN;
    let mut book_move: libc::c_int = 0;
    let mut empties: libc::c_int = 0;
    let mut selectivity: libc::c_int = 0;
    let mut alpha: libc::c_int = 0;
    let mut beta: libc::c_int = 0;
    let mut any_search_result: libc::c_int = 0;
    let mut exact_score_failed: libc::c_int = 0;
    let mut incomplete_search: libc::c_int = 0;
    let mut long_selective_search: libc::c_int = 0;
    let mut old_depth: libc::c_int = 0;
    let mut old_eval: libc::c_int = 0;
    let mut last_window_center: libc::c_int = 0;
    let mut old_pv: [libc::c_int; 64] = [0; 64];
    let mut book_eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    empties =
        64 as libc::c_int - disc_count(0 as libc::c_int) -
            disc_count(2 as libc::c_int);
    /* In komi games, the WLD window is adjusted. */
    if side_to_move == 0 as libc::c_int {
        komi_shift = komi
    } else { komi_shift = -komi }
    /* Check if the position is solved (WLD or exact) in the book. */
    book_move = -(1 as libc::c_int);
    if allow_book != 0 {
        /* Is the exact score known? */
        fill_move_alternatives(side_to_move, 16 as libc::c_int);
        book_move = get_book_move(side_to_move, 0 as libc::c_int, eval_info);
        if book_move != -(1 as libc::c_int) {
            root_eval = (*eval_info).score / 128 as libc::c_int;
            hash_expand_pv(side_to_move, 1 as libc::c_int, 4 as libc::c_int,
                           0 as libc::c_int);
            send_solve_status(empties, side_to_move, eval_info);
            return book_move
        }
        /* Is the WLD status known? */
        fill_move_alternatives(side_to_move, 4 as libc::c_int);
        if komi_shift == 0 as libc::c_int {
            book_move =
                get_book_move(side_to_move, 0 as libc::c_int, eval_info);
            if book_move != -(1 as libc::c_int) {
                if wld != 0 {
                    root_eval = (*eval_info).score / 128 as libc::c_int;
                    hash_expand_pv(side_to_move, 1 as libc::c_int,
                                   4 as libc::c_int | 2 as libc::c_int |
                                       1 as libc::c_int, 0 as libc::c_int);
                    send_solve_status(empties, side_to_move, eval_info);
                    return book_move
                } else { book_eval_info = *eval_info }
            }
        }
        fill_endgame_hash(8 as libc::c_int + 1 as libc::c_int,
                          0 as libc::c_int);
    }
    last_panic_check = 0.0f64;
    solve_status = UNKNOWN;
    old_eval = 0 as libc::c_int;
    /* Prepare for the shallow searches using the midgame eval */
    piece_count[0 as libc::c_int as usize][disks_played as usize] =
        disc_count(0 as libc::c_int);
    piece_count[2 as libc::c_int as usize][disks_played as usize] =
        disc_count(2 as libc::c_int);
    if empties > 32 as libc::c_int {
        set_panic_threshold(0.20f64);
    } else if empties < 22 as libc::c_int {
        set_panic_threshold(0.50f64);
    } else {
        set_panic_threshold(0.50f64 -
                                (empties - 22 as libc::c_int) as
                                    libc::c_double * 0.03f64);
    }
    reset_buffer_display();
    /* Make sure the pre-searches don't mess up the hash table */
    toggle_midgame_hash_usage(1 as libc::c_int, 0 as libc::c_int);
    incomplete_search = 0 as libc::c_int;
    any_search_result = 0 as libc::c_int;
    /* Start off by selective endgame search */
    last_window_center = 0 as libc::c_int;
    if empties > 18 as libc::c_int {
        if wld != 0 {
            selectivity = 9 as libc::c_int;
            while selectivity > 0 as libc::c_int && is_panic_abort() == 0 &&
                      force_return == 0 {
                let mut flags: libc::c_uint = 0;
                let mut res = WON_POSITION;
                alpha = -(1 as libc::c_int);
                beta = 1 as libc::c_int;
                root_eval =
                    end_tree_wrapper(0 as libc::c_int, empties, side_to_move,
                                     alpha, beta, selectivity,
                                     1 as libc::c_int);
                adjust_counter(&mut nodes);
                if is_panic_abort() != 0 || force_return != 0 { break ; }
                any_search_result = 1 as libc::c_int;
                old_eval = root_eval;
                store_pv(old_pv.as_mut_ptr(), &mut old_depth);
                current_confidence = confidence[selectivity as usize];
                flags = 4 as libc::c_int as libc::c_uint;
                if root_eval == 0 as libc::c_int {
                    res = DRAWN_POSITION
                } else {
                    flags |=
                        (2 as libc::c_int | 1 as libc::c_int) as libc::c_uint;
                    if root_eval > 0 as libc::c_int {
                        res = WON_POSITION
                    } else { res = LOST_POSITION }
                }
                *eval_info =
                    create_eval_info(SELECTIVE_EVAL, res,
                                     root_eval * 128 as libc::c_int,
                                     current_confidence, empties,
                                     0 as libc::c_int);
                if full_output_mode != 0 {
                    hash_expand_pv(side_to_move, 1 as libc::c_int,
                                   flags as libc::c_int, selectivity);
                    send_solve_status(empties, side_to_move, eval_info);
                }
                selectivity -= 1
            }
        } else {
            selectivity = 9 as libc::c_int;
            while selectivity > 0 as libc::c_int && is_panic_abort() == 0 &&
                      force_return == 0 {
                alpha = last_window_center - 1 as libc::c_int;
                beta = last_window_center + 1 as libc::c_int;
                root_eval =
                    end_tree_wrapper(0 as libc::c_int, empties, side_to_move,
                                     alpha, beta, selectivity,
                                     1 as libc::c_int);
                if root_eval <= alpha {
                    loop  {
                        last_window_center -= 2 as libc::c_int;
                        alpha = last_window_center - 1 as libc::c_int;
                        beta = last_window_center + 1 as libc::c_int;
                        if is_panic_abort() != 0 || force_return != 0 {
                            break ;
                        }
                        root_eval =
                            end_tree_wrapper(0 as libc::c_int, empties,
                                             side_to_move, alpha, beta,
                                             selectivity, 1 as libc::c_int);
                        if !(root_eval <= alpha) { break ; }
                    }
                    root_eval = last_window_center
                } else if root_eval >= beta {
                    loop  {
                        last_window_center += 2 as libc::c_int;
                        alpha = last_window_center - 1 as libc::c_int;
                        beta = last_window_center + 1 as libc::c_int;
                        if is_panic_abort() != 0 || force_return != 0 {
                            break ;
                        }
                        root_eval =
                            end_tree_wrapper(0 as libc::c_int, empties,
                                             side_to_move, alpha, beta,
                                             selectivity, 1 as libc::c_int);
                        if !(root_eval >= beta) { break ; }
                    }
                    root_eval = last_window_center
                }
                adjust_counter(&mut nodes);
                if is_panic_abort() != 0 || force_return != 0 { break ; }
                last_window_center = root_eval;
                if is_panic_abort() == 0 && force_return == 0 {
                    any_search_result = 1 as libc::c_int;
                    old_eval = root_eval;
                    store_pv(old_pv.as_mut_ptr(), &mut old_depth);
                    current_confidence = confidence[selectivity as usize];
                    *eval_info =
                        create_eval_info(SELECTIVE_EVAL, UNSOLVED_POSITION,
                                         root_eval * 128 as libc::c_int,
                                         current_confidence, empties,
                                         0 as libc::c_int);
                    if full_output_mode != 0 {
                        hash_expand_pv(side_to_move, 1 as libc::c_int,
                                       4 as libc::c_int, selectivity);
                        send_solve_status(empties, side_to_move, eval_info);
                    }
                }
                selectivity -= 1
            }
        }
    } else { selectivity = 0 as libc::c_int }
    /* Check if the selective search took more than 40% of the allocated
         time. If this is the case, there is no point attempting WLD. */
    long_selective_search = check_threshold(0.35f64);
    /* Make sure the panic abort flag is set properly; it must match
       the status of long_selective_search. This is not automatic as
       it is not guaranteed that any selective search was performed. */
    check_panic_abort();
    if is_panic_abort() != 0 || force_return != 0 ||
           long_selective_search != 0 {
        /* Don't try non-selective solve. */
        if any_search_result != 0 {
            if echo != 0 && (is_panic_abort() != 0 || force_return != 0) {
                printf(b"%s %.1f %c %s\n\x00" as *const u8 as
                           *const libc::c_char,
                       b"Semi-panic abort after\x00" as *const u8 as
                           *const libc::c_char, get_elapsed_time(),
                       's' as i32,
                       b"in selective search\x00" as *const u8 as
                           *const libc::c_char);
                if full_output_mode != 0 {
                    let mut flags_0: libc::c_uint = 0;
                    flags_0 = 4 as libc::c_int as libc::c_uint;
                    if solve_status as libc::c_uint !=
                           DRAW as libc::c_int as libc::c_uint {
                        flags_0 |=
                            (2 as libc::c_int | 1 as libc::c_int) as
                                libc::c_uint
                    }
                    hash_expand_pv(side_to_move, 1 as libc::c_int,
                                   flags_0 as libc::c_int, selectivity);
                    send_solve_status(empties, side_to_move, eval_info);
                }
            }
            pv[0 as libc::c_int as usize][0 as libc::c_int as usize] =
                best_end_root_move;
            pv_depth[0 as libc::c_int as usize] = 1 as libc::c_int;
            root_eval = old_eval;
            clear_panic_abort();
        } else {
            if echo != 0 {
                printf(b"%s %.1f %c %s\n\x00" as *const u8 as
                           *const libc::c_char,
                       b"Panic abort after\x00" as *const u8 as
                           *const libc::c_char, get_elapsed_time(),
                       's' as i32,
                       b"in selective search\x00" as *const u8 as
                           *const libc::c_char);
            }
            root_eval = -(27000 as libc::c_int)
        }
        if echo != 0 || force_echo != 0 {
            display_status(stdout, 0 as libc::c_int);
        }
        if book_move != -(1 as libc::c_int) &&
               (book_eval_info.res as libc::c_uint ==
                    WON_POSITION as libc::c_int as libc::c_uint ||
                    book_eval_info.res as libc::c_uint ==
                        DRAWN_POSITION as libc::c_int as libc::c_uint) {
            /* If there is a known win (or mismarked draw) available,
             always play it upon timeout. */
            *eval_info = book_eval_info;
            root_eval = (*eval_info).score / 128 as libc::c_int;
            return book_move
        } else {
            return pv[0 as libc::c_int as usize][0 as libc::c_int as usize]
        }
    }
    /* Start non-selective solve */
    if wld != 0 {
        alpha = -(1 as libc::c_int);
        beta = 1 as libc::c_int
    } else {
        alpha = last_window_center - 1 as libc::c_int;
        beta = last_window_center + 1 as libc::c_int
    }
    root_eval =
        end_tree_wrapper(0 as libc::c_int, empties, side_to_move, alpha, beta,
                         0 as libc::c_int, 1 as libc::c_int);
    adjust_counter(&mut nodes);
    if is_panic_abort() == 0 && force_return == 0 {
        if wld == 0 {
            if root_eval <= alpha {
                let mut ceiling_value = last_window_center - 2 as libc::c_int;
                loop  {
                    alpha = ceiling_value - 1 as libc::c_int;
                    beta = ceiling_value;
                    root_eval =
                        end_tree_wrapper(0 as libc::c_int, empties,
                                         side_to_move, alpha, beta,
                                         0 as libc::c_int, 1 as libc::c_int);
                    if is_panic_abort() != 0 || force_return != 0 { break ; }
                    if root_eval > alpha { break ; }
                    ceiling_value -= 2 as libc::c_int
                }
            } else if root_eval >= beta {
                let mut floor_value = last_window_center + 2 as libc::c_int;
                loop  {
                    alpha = floor_value - 1 as libc::c_int;
                    beta = floor_value + 1 as libc::c_int;
                    root_eval =
                        end_tree_wrapper(0 as libc::c_int, empties,
                                         side_to_move, alpha, beta,
                                         0 as libc::c_int, 1 as libc::c_int);
                    if is_panic_abort() != 0 || force_return != 0 { break ; }
                    if root_eval > alpha {
                    } else {
                        __assert_fail(b"root_eval > alpha\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"end.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      2126 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 56],
                                                                &[libc::c_char; 56]>(b"int end_game(int, int, int, int, int, EvaluationType *)\x00")).as_ptr());
                    }
                    if root_eval < beta { break ; }
                    floor_value += 2 as libc::c_int
                }
            }
        }
        if is_panic_abort() == 0 && force_return == 0 {
            let mut res_0 = WON_POSITION;
            if root_eval < 0 as libc::c_int {
                res_0 = LOST_POSITION
            } else if root_eval == 0 as libc::c_int {
                res_0 = DRAWN_POSITION
            } else { res_0 = WON_POSITION }
            if wld != 0 {
                let mut flags_1: libc::c_uint = 0;
                if root_eval == 0 as libc::c_int {
                    flags_1 = 4 as libc::c_int as libc::c_uint
                } else {
                    flags_1 =
                        (2 as libc::c_int | 1 as libc::c_int) as libc::c_uint
                }
                *eval_info =
                    create_eval_info(WLD_EVAL, res_0,
                                     root_eval * 128 as libc::c_int, 0.0f64,
                                     empties, 0 as libc::c_int);
                if full_output_mode != 0 {
                    hash_expand_pv(side_to_move, 1 as libc::c_int,
                                   flags_1 as libc::c_int, 0 as libc::c_int);
                    send_solve_status(empties, side_to_move, eval_info);
                }
            } else {
                *eval_info =
                    create_eval_info(EXACT_EVAL, res_0,
                                     root_eval * 128 as libc::c_int, 0.0f64,
                                     empties, 0 as libc::c_int);
                if full_output_mode != 0 {
                    hash_expand_pv(side_to_move, 1 as libc::c_int,
                                   4 as libc::c_int, 0 as libc::c_int);
                    send_solve_status(empties, side_to_move, eval_info);
                }
            }
        }
    }
    adjust_counter(&mut nodes);
    /* Check for abort. */
    if is_panic_abort() != 0 || force_return != 0 {
        if any_search_result != 0 {
            if echo != 0 {
                printf(b"%s %.1f %c %s\n\x00" as *const u8 as
                           *const libc::c_char,
                       b"Semi-panic abort after\x00" as *const u8 as
                           *const libc::c_char, get_elapsed_time(),
                       's' as i32,
                       b"in WLD search\x00" as *const u8 as
                           *const libc::c_char);
                if full_output_mode != 0 {
                    let mut flags_2: libc::c_uint = 0;
                    flags_2 = 4 as libc::c_int as libc::c_uint;
                    if root_eval != 0 as libc::c_int {
                        flags_2 |=
                            (2 as libc::c_int | 1 as libc::c_int) as
                                libc::c_uint
                    }
                    hash_expand_pv(side_to_move, 1 as libc::c_int,
                                   flags_2 as libc::c_int, 0 as libc::c_int);
                    send_solve_status(empties, side_to_move, eval_info);
                }
                if echo != 0 || force_echo != 0 {
                    display_status(stdout, 0 as libc::c_int);
                }
            }
            restore_pv(old_pv.as_mut_ptr(), old_depth);
            root_eval = old_eval;
            clear_panic_abort();
        } else {
            if echo != 0 {
                printf(b"%s %.1f %c %s\n\x00" as *const u8 as
                           *const libc::c_char,
                       b"Panic abort after\x00" as *const u8 as
                           *const libc::c_char, get_elapsed_time(),
                       's' as i32,
                       b"in WLD search\x00" as *const u8 as
                           *const libc::c_char);
            }
            root_eval = -(27000 as libc::c_int)
        }
        return pv[0 as libc::c_int as usize][0 as libc::c_int as usize]
    }
    /* Update solve info. */
    store_pv(old_pv.as_mut_ptr(), &mut old_depth);
    old_eval = root_eval;
    if is_panic_abort() == 0 && force_return == 0 &&
           empties > earliest_wld_solve {
        earliest_wld_solve = empties
    }
    /* Check for aborted search. */
    exact_score_failed = 0 as libc::c_int;
    if incomplete_search != 0 {
        if echo != 0 {
            printf(b"%s %.1f %c %s\n\x00" as *const u8 as *const libc::c_char,
                   b"Semi-panic abort after\x00" as *const u8 as
                       *const libc::c_char, get_elapsed_time(), 's' as i32,
                   b"in exact search\x00" as *const u8 as
                       *const libc::c_char);
            if full_output_mode != 0 {
                hash_expand_pv(side_to_move, 1 as libc::c_int,
                               4 as libc::c_int, 0 as libc::c_int);
                send_solve_status(empties, side_to_move, eval_info);
            }
            if echo != 0 || force_echo != 0 {
                display_status(stdout, 0 as libc::c_int);
            }
        }
        pv[0 as libc::c_int as usize][0 as libc::c_int as usize] =
            best_end_root_move;
        pv_depth[0 as libc::c_int as usize] = 1 as libc::c_int;
        root_eval = old_eval;
        exact_score_failed = 1 as libc::c_int;
        clear_panic_abort();
    }
    if abs(root_eval) % 2 as libc::c_int == 1 as libc::c_int {
        if root_eval > 0 as libc::c_int {
            root_eval += 1
        } else { root_eval -= 1 }
    }
    if exact_score_failed == 0 && wld == 0 && empties > earliest_full_solve {
        earliest_full_solve = empties
    }
    if wld == 0 && exact_score_failed == 0 {
        (*eval_info).type_0 = EXACT_EVAL;
        (*eval_info).score = root_eval * 128 as libc::c_int
    }
    if wld == 0 && exact_score_failed == 0 {
        hash_expand_pv(side_to_move, 1 as libc::c_int, 4 as libc::c_int,
                       0 as libc::c_int);
        send_solve_status(empties, side_to_move, eval_info);
    }
    if echo != 0 || force_echo != 0 {
        display_status(stdout, 0 as libc::c_int);
    }
    /* For shallow endgames, we can afford to compute the entire PV
       move by move. */
    if wld == 0 && incomplete_search == 0 && force_return == 0 &&
           empties <= 16 as libc::c_int {
        full_expand_pv(side_to_move, 0 as libc::c_int);
    }
    return pv[0 as libc::c_int as usize][0 as libc::c_int as usize];
}
/*
   SETUP_END
   Prepares the endgame solver for a new game.
   This means clearing a few status fields.
*/
#[no_mangle]
pub unsafe extern "C" fn setup_end() {
    let mut last_mean: libc::c_double = 0.;
    let mut last_sigma: libc::c_double = 0.;
    let mut ff_threshold: [libc::c_double; 61] = [0.; 61];
    let mut prelim_threshold: [[libc::c_double; 64]; 61] = [[0.; 64]; 61];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    static mut dir_shift: [libc::c_int; 8] =
        [1 as libc::c_int, -(1 as libc::c_int), 7 as libc::c_int,
         -(7 as libc::c_int), 8 as libc::c_int, -(8 as libc::c_int),
         9 as libc::c_int, -(9 as libc::c_int)];
    earliest_wld_solve = 0 as libc::c_int;
    earliest_full_solve = 0 as libc::c_int;
    full_output_mode = 1 as libc::c_int;
    /* Calculate the neighborhood masks */
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            /* Create the neighborhood mask for the square POS */
            let mut pos = 10 as libc::c_int * i + j;
            let mut shift =
                8 as libc::c_int * (i - 1 as libc::c_int) +
                    (j - 1 as libc::c_int);
            let mut k: libc::c_uint = 0;
            neighborhood_mask[pos as usize].low =
                0 as libc::c_int as libc::c_uint;
            neighborhood_mask[pos as usize].high =
                0 as libc::c_int as libc::c_uint;
            k = 0 as libc::c_int as libc::c_uint;
            while k < 8 as libc::c_int as libc::c_uint {
                if dir_mask[pos as usize] & (1 as libc::c_int) << k != 0 {
                    let mut neighbor =
                        (shift + dir_shift[k as usize]) as libc::c_uint;
                    if neighbor < 32 as libc::c_int as libc::c_uint {
                        neighborhood_mask[pos as usize].low |=
                            ((1 as libc::c_int) << neighbor) as libc::c_uint
                    } else {
                        neighborhood_mask[pos as usize].high |=
                            ((1 as libc::c_int) <<
                                 neighbor.wrapping_sub(32 as libc::c_int as
                                                           libc::c_uint)) as
                                libc::c_uint
                    }
                }
                k = k.wrapping_add(1)
            }
            j += 1
        }
        i += 1
    }
    /* Set the fastest-first mobility encouragements and thresholds */
    i = 0 as libc::c_int;
    while i <= 60 as libc::c_int {
        ff_mob_factor[i as usize] = 460 as libc::c_int;
        i += 1
    }
    i = 0 as libc::c_int;
    while i <= 60 as libc::c_int {
        ff_threshold[i as usize] = 0.45f64;
        i += 1
    }
    /* Calculate the alpha thresholds for using fastest-first for
       each #empty and shallow search depth. */
    j = 0 as libc::c_int; /* Infinity in disc difference */
    while j <= 8 as libc::c_int {
        last_sigma = 100.0f64;
        last_mean = 0.0f64;
        i = 60 as libc::c_int;
        while i >= 0 as libc::c_int {
            if end_stats_available[i as usize][j as usize] != 0 {
                last_mean =
                    end_mean[i as usize][j as usize] as libc::c_double;
                last_sigma =
                    ff_threshold[i as usize] *
                        end_sigma[i as usize][j as usize] as libc::c_double
            }
            fast_first_mean[i as usize][j as usize] = last_mean;
            fast_first_sigma[i as usize][j as usize] = last_sigma;
            prelim_threshold[i as usize][j as usize] = last_mean + last_sigma;
            i -= 1
        }
        j += 1
    }
    j = 8 as libc::c_int + 1 as libc::c_int;
    while j < 64 as libc::c_int {
        i = 0 as libc::c_int;
        while i <= 60 as libc::c_int {
            prelim_threshold[i as usize][j as usize] =
                prelim_threshold[i as usize][8 as libc::c_int as usize];
            i += 1
        }
        j += 1
    }
    i = 0 as libc::c_int;
    while i <= 60 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 64 as libc::c_int {
            fast_first_threshold[i as usize][j as usize] =
                ceil(prelim_threshold[i as usize][j as usize] * 128.0f64) as
                    libc::c_int;
            j += 1
        }
        i += 1
    };
}
/*
   File:          end.h

   Created:       June 25, 1997

   Modified:      November 24, 2005

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The interface to the endgame solver.
*/
/*
  GET_EARLIEST_WLD_SOLVE
  GET_EARLIEST_FULL_SOLVE
  Return the highest #empty when WLD and full solve respectively
  were completed (not initiated).
*/
#[no_mangle]
pub unsafe extern "C" fn get_earliest_wld_solve() -> libc::c_int {
    return earliest_wld_solve;
}
#[no_mangle]
pub unsafe extern "C" fn get_earliest_full_solve() -> libc::c_int {
    return earliest_full_solve;
}
/*
  SET_OUTPUT_MODE
  Toggles output of intermediate search status on/off.
*/
#[no_mangle]
pub unsafe extern "C" fn set_output_mode(mut full: libc::c_int) {
    full_output_mode = full;
}
