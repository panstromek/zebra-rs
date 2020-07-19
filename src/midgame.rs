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
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
    /* The value of the root position from the last midgame or
   endgame search. Can contain strange values if an event
   occurred. */
    #[no_mangle]
    static mut root_eval: libc::c_int;
    /* Event flag which forces the search to abort immediately when set. */
    #[no_mangle]
    static mut force_return: libc::c_int;
    /* The number of positions evaluated during the current search. */
    #[no_mangle]
    static mut evaluations: CounterType;
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
    fn inherit_move_lists(stage: libc::c_int);
    #[no_mangle]
    fn reorder_move_list(stage: libc::c_int);
    #[no_mangle]
    fn disc_count(side_to_move: libc::c_int) -> libc::c_int;
    /*
   File:         display.h

   Created:      July 10, 1997

   Modified:     November 17, 2002

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:     Declarations of the screen output functions.
*/
    /* Flag variable, non-zero if output should be written to stdout. */
    /* Flag variable, non-zero if the principal variation is to be
   displayed. */
    #[no_mangle]
    fn send_status_nodes(node_count: libc::c_double);
    /* Holds the current board position. Updated as the search progresses,
   but all updates must be reversed when the search stops. */
    #[no_mangle]
    static mut board: Board;
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
    static mut echo: libc::c_int;
    #[no_mangle]
    fn send_status(format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn send_status_time(elapsed_time: libc::c_double);
    #[no_mangle]
    fn send_status_pv(pv_0: *mut libc::c_int, max_depth: libc::c_int);
    #[no_mangle]
    fn counter_value(counter: *mut CounterType) -> libc::c_double;
    #[no_mangle]
    fn adjust_counter(counter: *mut CounterType);
    #[no_mangle]
    static mut pv: [[libc::c_int; 64]; 64];
    #[no_mangle]
    static mut pv_depth: [libc::c_int; 64];
    #[no_mangle]
    static mut piece_count: [[libc::c_int; 64]; 3];
    #[no_mangle]
    fn clear_status();
    #[no_mangle]
    fn send_sweep(format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn clear_sweep();
    #[no_mangle]
    fn display_sweep(stream: *mut FILE);
    #[no_mangle]
    fn display_buffers();
    #[no_mangle]
    fn produce_eval_text(eval_info: EvaluationType, short_output: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn terminal_evaluation(side_to_move: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pattern_evaluation(side_to_move: libc::c_int) -> libc::c_int;
    /* The 64-bit hash key. */
    #[no_mangle]
    static mut hash1: libc::c_uint;
    #[no_mangle]
    static mut hash2: libc::c_uint;
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
    #[no_mangle]
    fn generate_all(side_to_move: libc::c_int);
    #[no_mangle]
    fn make_move(side_to_move: libc::c_int, move_0: libc::c_int,
                 update_hash: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn make_move_no_hash(side_to_move: libc::c_int, move_0: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn unmake_move(side_to_move: libc::c_int, move_0: libc::c_int);
    #[no_mangle]
    fn unmake_move_no_hash(side_to_move: libc::c_int, move_0: libc::c_int);
    #[no_mangle]
    fn valid_move(move_0: libc::c_int, side_to_move: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn my_random() -> libc::c_long;
    #[no_mangle]
    static mut mpc_cut: [DepthInfo; 23];
    #[no_mangle]
    static mut frozen_ponder_depth: libc::c_int;
    /* Holds the value of the variable NODES the last time the
   timer module was called to check if a panic abort occured. */
    #[no_mangle]
    static mut last_panic_check: libc::c_double;
    #[no_mangle]
    fn check_panic_abort();
    #[no_mangle]
    fn is_panic_abort() -> libc::c_int;
    #[no_mangle]
    fn get_elapsed_time() -> libc::c_double;
    #[no_mangle]
    fn above_recommended() -> libc::c_int;
    #[no_mangle]
    fn extended_above_recommended() -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub struct CounterType {
    pub hi: libc::c_uint,
    pub lo: libc::c_uint,
}
pub type Board = [libc::c_int; 128];
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
pub struct HashEntry {
    pub key1: libc::c_uint,
    pub key2: libc::c_uint,
    pub eval: libc::c_int,
    pub move_0: [libc::c_int; 4],
    pub draft: libc::c_short,
    pub selectivity: libc::c_short,
    pub flags: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DepthInfo {
    pub cut_tries: libc::c_int,
    pub cut_depth: [libc::c_int; 2],
    pub bias: [[libc::c_int; 61]; 2],
    pub window: [[libc::c_int; 61]; 2],
}
/* Default aspiration window parameters. These values are currently
   really huge as usage of a small windows tends to slow down
   the search. */
static mut allow_midgame_hash_probe: libc::c_int = 0;
static mut allow_midgame_hash_update: libc::c_int = 0;
static mut best_mid_move: libc::c_int = 0;
static mut best_mid_root_move: libc::c_int = 0;
static mut midgame_abort: libc::c_int = 0;
static mut do_check_midgame_abort: libc::c_int = 1 as libc::c_int;
static mut counter_phase: libc::c_int = 0;
static mut apply_perturbation: libc::c_int = 1 as libc::c_int;
static mut perturbation_amplitude: libc::c_int = 0 as libc::c_int;
static mut stage_reached: [libc::c_int; 61] = [0; 61];
static mut stage_score: [libc::c_int; 61] = [0; 61];
static mut score_perturbation: [libc::c_int; 100] = [0; 100];
static mut feas_index_list: [[libc::c_int; 64]; 64] = [[0; 64]; 64];
/*
   SETUP_MIDGAME
   Sets up some search parameters.
*/
#[no_mangle]
pub unsafe extern "C" fn setup_midgame() {
    let mut i: libc::c_int = 0;
    allow_midgame_hash_probe = 1 as libc::c_int;
    allow_midgame_hash_update = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i <= 60 as libc::c_int {
        stage_reached[i as usize] = 0 as libc::c_int;
        i += 1
    }
    calculate_perturbation();
}
/*
  CLEAR_MIDGAME_ABORT
  IS_MIDGAME_ABORT
  SET_MIDGAME_ABORT
  TOGGLE_MIDGAME_ABORT_CHECK
  These functions handle the midgame abort system which kicks in
  when it is estimated that the next iteration in the iterative
  deepening would take too long.
*/
#[no_mangle]
pub unsafe extern "C" fn clear_midgame_abort() {
    midgame_abort = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_midgame_abort() -> libc::c_int {
    return midgame_abort;
}
#[no_mangle]
pub unsafe extern "C" fn set_midgame_abort() {
    midgame_abort = do_check_midgame_abort;
}
#[no_mangle]
pub unsafe extern "C" fn toggle_midgame_abort_check(mut toggle: libc::c_int) {
    do_check_midgame_abort = toggle;
}
/*
   TOGGLE_MIDGAME_HASH_USAGE
   Toggles hash table access in the midgame search on/off.
*/
#[no_mangle]
pub unsafe extern "C" fn toggle_midgame_hash_usage(mut allow_read:
                                                       libc::c_int,
                                                   mut allow_write:
                                                       libc::c_int) {
    allow_midgame_hash_probe = allow_read;
    allow_midgame_hash_update = allow_write;
}
/*
  CALCULATE_PERTURBATION
  Determines the score perturbations (if any) to the root moves.
*/
#[no_mangle]
pub unsafe extern "C" fn calculate_perturbation() {
    let mut i: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    if apply_perturbation == 0 || perturbation_amplitude == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 100 as libc::c_int {
            score_perturbation[i as usize] = 0 as libc::c_int;
            i += 1
        }
    } else {
        shift = perturbation_amplitude / 2 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 100 as libc::c_int {
            score_perturbation[i as usize] =
                abs(my_random() as libc::c_int) % perturbation_amplitude -
                    shift;
            i += 1
        }
    };
}
/*
  SET_PERTURBATION
  Set the amplitude of the score perturbation applied by
  CALCULATE_PERTURBATION.
*/
#[no_mangle]
pub unsafe extern "C" fn set_perturbation(mut amplitude: libc::c_int) {
    perturbation_amplitude = amplitude;
}
/*
  TOGGLE_PERTURBATION_USAGE
  Toggle usage of score perturbations on/off.
*/
#[no_mangle]
pub unsafe extern "C" fn toggle_perturbation_usage(mut toggle: libc::c_int) {
    apply_perturbation = toggle;
}
/*
  ADVANCE_MOVE
  Swaps a move and its predecessor in the move list if it's
  not already first in the list.
*/
unsafe extern "C" fn advance_move(mut index: libc::c_int) {
    let mut temp_move: libc::c_int = 0;
    if index > 0 as libc::c_int {
        temp_move = sorted_move_order[disks_played as usize][index as usize];
        sorted_move_order[disks_played as usize][index as usize] =
            sorted_move_order[disks_played as
                                  usize][(index - 1 as libc::c_int) as usize];
        sorted_move_order[disks_played as
                              usize][(index - 1 as libc::c_int) as usize] =
            temp_move
    };
}
/*
  STATIC_OR_TERMINAL_EVALUATION
  Invokes the proper evaluation function depending on whether the
  board is filled or not.
*/
unsafe extern "C" fn static_or_terminal_evaluation(mut side_to_move:
                                                       libc::c_int)
 -> libc::c_int {
    if disks_played == 60 as libc::c_int {
        return terminal_evaluation(side_to_move)
    } else {
        evaluations.lo = evaluations.lo.wrapping_add(1);
        return pattern_evaluation(side_to_move)
    };
}
/*
   FAST_TREE_SEARCH
   The recursive tree search function. It uses negascout for
   tree pruning.
*/
unsafe extern "C" fn fast_tree_search(mut level: libc::c_int,
                                      mut max_depth: libc::c_int,
                                      mut side_to_move: libc::c_int,
                                      mut alpha: libc::c_int,
                                      mut beta: libc::c_int,
                                      mut allow_hash: libc::c_int,
                                      mut void_legal: libc::c_int)
 -> libc::c_int {
    let mut curr_val: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut move_index: libc::c_int = 0;
    let mut move_0: libc::c_int = 0;
    let mut best_move_index: libc::c_int = 0;
    let mut best_move: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut remains: libc::c_int = 0;
    let mut use_hash: libc::c_int = 0;
    let mut new_use_hash: libc::c_int = 0;
    let mut curr_alpha: libc::c_int = 0;
    let mut empties_remaining: libc::c_int = 0;
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
        (remains >= 2 as libc::c_int && 1 as libc::c_int != 0 &&
             allow_hash != 0) as libc::c_int;
    if use_hash != 0 && allow_midgame_hash_probe != 0 {
        find_hash(&mut entry, 0 as libc::c_int);
        if entry.draft as libc::c_int >= remains &&
               entry.selectivity as libc::c_int == 0 as libc::c_int &&
               valid_move(entry.move_0[0 as libc::c_int as usize],
                          side_to_move) != 0 &&
               entry.flags as libc::c_int & 8 as libc::c_int != 0 &&
               (entry.flags as libc::c_int & 4 as libc::c_int != 0 ||
                    entry.flags as libc::c_int & 1 as libc::c_int != 0 &&
                        entry.eval >= beta ||
                    entry.flags as libc::c_int & 2 as libc::c_int != 0 &&
                        entry.eval <= alpha) {
            best_mid_move = entry.move_0[0 as libc::c_int as usize];
            return entry.eval
        }
    }
    /* Reorder the move lists now and then to keep the empty squares up front */
    if nodes.lo & 4095 as libc::c_int as libc::c_uint ==
           0 as libc::c_int as libc::c_uint {
        reorder_move_list(disks_played);
    }
    /* Search */
    first = 1 as libc::c_int;
    best_move = -(1 as libc::c_int);
    best_move_index = -(1 as libc::c_int);
    best = -(12345678 as libc::c_int);
    if remains == 1 as libc::c_int {
        /* Plain alpha-beta last ply */
        empties_remaining = 60 as libc::c_int - disks_played;
        move_index = 0 as libc::c_int;
        while move_index < 60 as libc::c_int {
            move_0 =
                sorted_move_order[disks_played as usize][move_index as usize];
            if board[move_0 as usize] == 1 as libc::c_int {
                if make_move_no_hash(side_to_move, move_0) != 0 as libc::c_int
                   {
                    curr_val =
                        -static_or_terminal_evaluation(0 as libc::c_int +
                                                           2 as libc::c_int -
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
                                add_hash(0 as libc::c_int, best, best_move,
                                         8 as libc::c_int | 1 as libc::c_int,
                                         remains, 0 as libc::c_int);
                            }
                            return best
                        }
                    }
                    first = 0 as libc::c_int
                }
                empties_remaining -= 1;
                if empties_remaining == 0 as libc::c_int { break ; }
            }
            move_index += 1
        }
    } else {
        /* Principal variation search for deeper searches */
        new_use_hash =
            (remains >= 2 as libc::c_int + 1 as libc::c_int && use_hash != 0)
                as libc::c_int;
        curr_alpha = alpha;
        empties_remaining = 60 as libc::c_int - disks_played;
        move_index = 0 as libc::c_int;
        while move_index < 60 as libc::c_int {
            move_0 =
                sorted_move_order[disks_played as usize][move_index as usize];
            if board[move_0 as usize] == 1 as libc::c_int {
                if make_move(side_to_move, move_0, new_use_hash) !=
                       0 as libc::c_int {
                    if first != 0 {
                        curr_val =
                            -fast_tree_search(level + 1 as libc::c_int,
                                              max_depth,
                                              0 as libc::c_int +
                                                  2 as libc::c_int -
                                                  side_to_move, -beta,
                                              -curr_alpha, allow_hash,
                                              1 as libc::c_int);
                        best = curr_val;
                        best_move = move_0;
                        best_move_index = move_index
                    } else {
                        curr_alpha =
                            if best > curr_alpha { best } else { curr_alpha };
                        curr_val =
                            -fast_tree_search(level + 1 as libc::c_int,
                                              max_depth,
                                              0 as libc::c_int +
                                                  2 as libc::c_int -
                                                  side_to_move,
                                              -(curr_alpha +
                                                    1 as libc::c_int),
                                              -curr_alpha, allow_hash,
                                              1 as libc::c_int);
                        if curr_val > curr_alpha && curr_val < beta {
                            curr_val =
                                -fast_tree_search(level + 1 as libc::c_int,
                                                  max_depth,
                                                  0 as libc::c_int +
                                                      2 as libc::c_int -
                                                      side_to_move, -beta,
                                                  12345678 as libc::c_int,
                                                  allow_hash,
                                                  1 as libc::c_int)
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
                            add_hash(0 as libc::c_int, best, best_move,
                                     8 as libc::c_int | 1 as libc::c_int,
                                     remains, 0 as libc::c_int);
                        }
                        return best
                    }
                    first = 0 as libc::c_int
                }
                empties_remaining -= 1;
                if empties_remaining == 0 as libc::c_int { break ; }
            }
            move_index += 1
        }
    }
    if first == 0 {
        advance_move(best_move_index);
        best_mid_move = best_move;
        if use_hash != 0 && allow_midgame_hash_update != 0 {
            if best > alpha {
                add_hash(0 as libc::c_int, best, best_move,
                         8 as libc::c_int | 4 as libc::c_int, remains,
                         0 as libc::c_int);
            } else {
                add_hash(0 as libc::c_int, best, best_move,
                         8 as libc::c_int | 2 as libc::c_int, remains,
                         0 as libc::c_int);
            }
        }
        return best
    } else if void_legal != 0 {
        /* I pass, other player's turn now */
        hash1 ^= hash_flip_color1;
        hash2 ^= hash_flip_color2;
        curr_val =
            -fast_tree_search(level, max_depth,
                              0 as libc::c_int + 2 as libc::c_int -
                                  side_to_move, -beta, -alpha, allow_hash,
                              0 as libc::c_int);
        hash1 ^= hash_flip_color1;
        hash2 ^= hash_flip_color2;
        return curr_val
    } else {
        /* Both players had to pass ==> evaluate board as final */
        curr_val = terminal_evaluation(side_to_move);
        return curr_val
    };
}
/*
  midgame_c__update_best_list
*/
unsafe extern "C" fn midgame_c__update_best_list(mut best_list:
                                                     *mut libc::c_int,
                                                 mut move_0: libc::c_int,
                                                 mut best_list_index:
                                                     libc::c_int,
                                                 mut best_list_length:
                                                     libc::c_int) {
    let mut i: libc::c_int = 0;
    if best_list_index < best_list_length {
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
    }
    *best_list.offset(0 as libc::c_int as isize) = move_0;
}
/*
   TREE_SEARCH
   The recursive tree search function. It uses negascout for
   tree pruning.
*/
#[no_mangle]
pub unsafe extern "C" fn tree_search(mut level: libc::c_int,
                                     mut max_depth: libc::c_int,
                                     mut side_to_move: libc::c_int,
                                     mut alpha: libc::c_int,
                                     mut beta: libc::c_int,
                                     mut allow_hash: libc::c_int,
                                     mut allow_mpc: libc::c_int,
                                     mut void_legal: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut curr_val: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut pre_best: libc::c_int = 0;
    let mut searched: libc::c_int = 0;
    let mut move_0: libc::c_int = 0;
    let mut hash_move: libc::c_int = 0;
    let mut move_index: libc::c_int = 0;
    let mut best_move_index: libc::c_int = 0;
    let mut empties_remaining: libc::c_int = 0;
    let mut hash_hit: libc::c_int = 0;
    let mut pre_depth: libc::c_int = 0;
    let mut update_pv: libc::c_int = 0;
    let mut remains: libc::c_int = 0;
    let mut shallow_remains: libc::c_int = 0;
    let mut use_hash: libc::c_int = 0;
    let mut pre_search_done: libc::c_int = 0;
    let mut curr_alpha: libc::c_int = 0;
    let mut best_index: libc::c_int = 0;
    let mut best_score: libc::c_int = 0;
    let mut best_list_index: libc::c_int = 0;
    let mut best_list_length: libc::c_int = 0;
    let mut selectivity: libc::c_int = 0;
    let mut cut: libc::c_int = 0;
    let mut best_list: [libc::c_int; 4] = [0; 4];
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
    if remains < 3 as libc::c_int {
        curr_val =
            fast_tree_search(level, max_depth, side_to_move, alpha, beta,
                             allow_hash, void_legal);
        pv_depth[level as usize] = level + 1 as libc::c_int;
        pv[level as usize][level as usize] = best_mid_move;
        return curr_val
    }
    nodes.lo = nodes.lo.wrapping_add(1);
    /* Check the hash table */
    use_hash =
        (remains >= 2 as libc::c_int && 1 as libc::c_int != 0 &&
             allow_hash != 0) as libc::c_int;
    if 1 as libc::c_int != 0 && allow_mpc != 0 {
        selectivity = 1 as libc::c_int
    } else { selectivity = 0 as libc::c_int }
    if use_hash != 0 && allow_midgame_hash_probe != 0 {
        find_hash(&mut entry, 0 as libc::c_int);
        if entry.draft as libc::c_int >= remains &&
               entry.selectivity as libc::c_int <= selectivity &&
               valid_move(entry.move_0[0 as libc::c_int as usize],
                          side_to_move) != 0 &&
               entry.flags as libc::c_int & 8 as libc::c_int != 0 &&
               (entry.flags as libc::c_int & 4 as libc::c_int != 0 ||
                    entry.flags as libc::c_int & 1 as libc::c_int != 0 &&
                        entry.eval >= beta ||
                    entry.flags as libc::c_int & 2 as libc::c_int != 0 &&
                        entry.eval <= alpha) {
            pv[level as usize][level as usize] =
                entry.move_0[0 as libc::c_int as usize];
            pv_depth[level as usize] = level + 1 as libc::c_int;
            return entry.eval
        }
    }
    hash_hit =
        (use_hash != 0 && allow_midgame_hash_probe != 0) as libc::c_int;
    if hash_hit != 0 {
        hash_move = entry.move_0[0 as libc::c_int as usize]
    } else { hash_move = 44 as libc::c_int }
    pre_search_done = 0 as libc::c_int;
    /* Use multi-prob-cut to selectively prune the tree */
    if 1 as libc::c_int != 0 && allow_mpc != 0 && remains <= 22 as libc::c_int
       {
        let mut alpha_test = 1 as libc::c_int;
        let mut beta_test = 1 as libc::c_int;
        cut = 0 as libc::c_int;
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
            if !(level + shallow_remains < 8 as libc::c_int) {
                if shallow_remains > 1 as libc::c_int {
                    /* "Deep" shallow search */
                    if cut == 0 as libc::c_int {
                        /* Use static eval to decide if a one- or two-sided
                       MPC test is to be performed. */
                        evaluations.lo = evaluations.lo.wrapping_add(1);
                        let mut static_eval =
                            pattern_evaluation(side_to_move);
                        if static_eval <= alpha_bound {
                            beta_test = 0 as libc::c_int
                        } else if static_eval >= beta_bound {
                            alpha_test = 0 as libc::c_int
                        }
                    }
                    if alpha_test != 0 || beta_test != 0 {
                    } else {
                        __assert_fail(b"alpha_test || beta_test\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"midgame.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      563 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 56],
                                                                &[libc::c_char; 56]>(b"int tree_search(int, int, int, int, int, int, int, int)\x00")).as_ptr());
                    }
                    if alpha_test != 0 && beta_test != 0 {
                        /* Test for likely fail-low or likely fail-high. */
                        let mut shallow_val =
                            tree_search(level, level + shallow_remains,
                                        side_to_move, alpha_bound, beta_bound,
                                        allow_hash, 0 as libc::c_int,
                                        void_legal);
                        if shallow_val >= beta_bound {
                            if use_hash != 0 && allow_midgame_hash_update != 0
                               {
                                add_hash(0 as libc::c_int, beta,
                                         pv[level as usize][level as usize],
                                         8 as libc::c_int | 1 as libc::c_int,
                                         remains, selectivity);
                            }
                            return beta
                        } else if shallow_val <= alpha_bound {
                            if use_hash != 0 && allow_midgame_hash_update != 0
                               {
                                add_hash(0 as libc::c_int, alpha,
                                         pv[level as usize][level as usize],
                                         8 as libc::c_int | 2 as libc::c_int,
                                         remains, selectivity);
                            }
                            return alpha
                        } else {
                            /* Use information learned from the failed cut test to decide
                           if a one or a two-sided test is to be performed next. */
                            let mut mid =
                                (alpha_bound + beta_bound) / 2 as libc::c_int;
                            let mut low_threshold =
                                (2 as libc::c_int * mid + alpha_bound) /
                                    3 as libc::c_int;
                            let mut high_threshold =
                                (2 as libc::c_int * mid + beta_bound) /
                                    3 as libc::c_int;
                            if shallow_val <= low_threshold {
                                beta_test = 0 as libc::c_int
                            } else {
                                if !(shallow_val >= high_threshold) {
                                    break ;
                                }
                                alpha_test = 0 as libc::c_int
                            }
                            /* Unlikely that there is any selective cutoff. */
                        }
                    } else if beta_test != 0 {
                        /* Fail-high with high probability? */
                        if tree_search(level, level + shallow_remains,
                                       side_to_move,
                                       beta_bound - 1 as libc::c_int,
                                       beta_bound, allow_hash,
                                       0 as libc::c_int, void_legal) >=
                               beta_bound {
                            if use_hash != 0 && allow_midgame_hash_update != 0
                               {
                                add_hash(0 as libc::c_int, beta,
                                         pv[level as usize][level as usize],
                                         8 as libc::c_int | 1 as libc::c_int,
                                         remains, selectivity);
                            }
                            return beta
                        }
                    } else if alpha_test != 0 {
                        /* Fail-low with high probability? */
                        if tree_search(level, level + shallow_remains,
                                       side_to_move, alpha_bound,
                                       alpha_bound + 1 as libc::c_int,
                                       allow_hash, 0 as libc::c_int,
                                       void_legal) <= alpha_bound {
                            if use_hash != 0 && allow_midgame_hash_update != 0
                               {
                                add_hash(0 as libc::c_int, alpha,
                                         pv[level as usize][level as usize],
                                         8 as libc::c_int | 2 as libc::c_int,
                                         remains, selectivity);
                            }
                            return alpha
                        }
                    }
                } else {
                    /* All-in-one MPC one-ply search and move ordering */
                    move_count[disks_played as usize] = 0 as libc::c_int;
                    best = alpha_bound;
                    empties_remaining = 60 as libc::c_int - disks_played;
                    move_index = 0 as libc::c_int;
                    while move_index < 60 as libc::c_int {
                        move_0 =
                            sorted_move_order[disks_played as
                                                  usize][move_index as usize];
                        if board[move_0 as usize] == 1 as libc::c_int {
                            if make_move_no_hash(side_to_move, move_0) !=
                                   0 as libc::c_int {
                                curr_val =
                                    -static_or_terminal_evaluation(0 as
                                                                       libc::c_int
                                                                       +
                                                                       2 as
                                                                           libc::c_int
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
                                            add_hash(0 as libc::c_int, beta,
                                                     pv[level as
                                                            usize][level as
                                                                       usize],
                                                     8 as libc::c_int |
                                                         1 as libc::c_int,
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
                                        10000 as libc::c_int
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
                            if empties_remaining == 0 as libc::c_int {
                                break ;
                            }
                        }
                        move_index += 1
                    }
                    if best == alpha_bound &&
                           move_count[disks_played as usize] >
                               0 as libc::c_int {
                        if use_hash != 0 && allow_midgame_hash_update != 0 {
                            add_hash(0 as libc::c_int, alpha,
                                     pv[level as usize][level as usize],
                                     8 as libc::c_int | 2 as libc::c_int,
                                     remains, selectivity);
                        }
                        return alpha
                    }
                    pre_search_done = 1 as libc::c_int
                }
            }
            cut += 1
        }
    }
    /* Full negascout search */
    searched = 0 as libc::c_int;
    best = -(12345678 as libc::c_int);
    best_move_index = -(1 as libc::c_int);
    curr_alpha = alpha;
    best_list_length = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        best_list[i as usize] = 0 as libc::c_int;
        i += 1
    }
    if pre_search_done == 0 {
        move_count[disks_played as usize] = 0 as libc::c_int;
        if hash_hit != 0 {
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                if valid_move(entry.move_0[i as usize], side_to_move) != 0 {
                    let fresh0 = best_list_length;
                    best_list_length = best_list_length + 1;
                    best_list[fresh0 as usize] = entry.move_0[i as usize]
                }
                i += 1
            }
        }
    }
    i = 0 as libc::c_int;
    best_list_index = 0 as libc::c_int;
    loop 
         /* Try the hash table move(s) first if feasible */
         {
        if pre_search_done == 0 && best_list_index < best_list_length {
            move_count[disks_played as usize] += 1;
            move_index = 0 as libc::c_int;
            while sorted_move_order[disks_played as
                                        usize][move_index as usize] !=
                      best_list[best_list_index as usize] {
                move_index += 1
            }
        } else {
            /* Otherwise use information from shallow searches */
            if pre_search_done == 0 {
                if remains < 10 as libc::c_int {
                    pre_depth = 1 as libc::c_int
                } else { pre_depth = 2 as libc::c_int }
                pre_best = -(12345678 as libc::c_int);
                empties_remaining = 60 as libc::c_int - disks_played;
                move_index = 0 as libc::c_int;
                while move_index < 60 as libc::c_int {
                    let mut already_checked: libc::c_int = 0;
                    move_0 =
                        sorted_move_order[disks_played as
                                              usize][move_index as usize];
                    already_checked = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    while j < best_list_length {
                        if move_0 == best_list[j as usize] {
                            already_checked = 1 as libc::c_int
                        }
                        j += 1
                    }
                    if board[move_0 as usize] == 1 as libc::c_int {
                        if already_checked == 0 &&
                               make_move(side_to_move, move_0,
                                         1 as libc::c_int) != 0 as libc::c_int
                           {
                            curr_val =
                                -tree_search(level + 1 as libc::c_int,
                                             level + pre_depth,
                                             0 as libc::c_int +
                                                 2 as libc::c_int -
                                                 side_to_move,
                                             -(12345678 as libc::c_int),
                                             -pre_best, 0 as libc::c_int,
                                             0 as libc::c_int,
                                             1 as libc::c_int);
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
                        if empties_remaining == 0 as libc::c_int { break ; }
                    }
                    move_index += 1
                }
                pre_search_done = 1 as libc::c_int
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
            j = i + 1 as libc::c_int;
            while j < move_count[disks_played as usize] {
                let mut cand_move: libc::c_int = 0;
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
        counter_phase = counter_phase + 1 as libc::c_int & 63 as libc::c_int;
        if counter_phase == 0 as libc::c_int {
            let mut node_val: libc::c_double = 0.;
            adjust_counter(&mut nodes);
            node_val = counter_value(&mut nodes);
            if node_val - last_panic_check >=
                   100000 as libc::c_int as libc::c_double {
                /* Time abort? */
                last_panic_check = node_val;
                check_panic_abort();
                /* Display available search information */
                if echo != 0 { display_buffers(); }
                /* Check for events */
                handle_event(1 as libc::c_int, 0 as libc::c_int,
                             1 as libc::c_int);
                if is_panic_abort() != 0 || force_return != 0 {
                    return -(27000 as libc::c_int)
                }
            }
        }
        make_move(side_to_move, move_0, 1 as libc::c_int);
        update_pv = 0 as libc::c_int;
        if searched == 0 as libc::c_int {
            curr_val =
                -tree_search(level + 1 as libc::c_int, max_depth,
                             0 as libc::c_int + 2 as libc::c_int -
                                 side_to_move, -beta, -curr_alpha, allow_hash,
                             allow_mpc, 1 as libc::c_int);
            best = curr_val;
            best_move_index = move_index;
            update_pv = 1 as libc::c_int
        } else {
            curr_alpha = if best > curr_alpha { best } else { curr_alpha };
            curr_val =
                -tree_search(level + 1 as libc::c_int, max_depth,
                             0 as libc::c_int + 2 as libc::c_int -
                                 side_to_move,
                             -(curr_alpha + 1 as libc::c_int), -curr_alpha,
                             allow_hash, allow_mpc, 1 as libc::c_int);
            if curr_val > curr_alpha && curr_val < beta {
                curr_val =
                    -tree_search(level + 1 as libc::c_int, max_depth,
                                 0 as libc::c_int + 2 as libc::c_int -
                                     side_to_move, -beta,
                                 12345678 as libc::c_int, allow_hash,
                                 allow_mpc, 1 as libc::c_int);
                if curr_val > best {
                    best = curr_val;
                    best_move_index = move_index;
                    update_pv = 1 as libc::c_int
                }
            } else if curr_val > best {
                best = curr_val;
                best_move_index = move_index;
                update_pv = 1 as libc::c_int
            }
        }
        unmake_move(side_to_move, move_0);
        if is_panic_abort() != 0 || force_return != 0 {
            return -(27000 as libc::c_int)
        }
        evals[disks_played as usize][move_0 as usize] = curr_val;
        if update_pv != 0 {
            midgame_c__update_best_list(best_list.as_mut_ptr(), move_0,
                                        best_list_index, best_list_length);
            pv[level as usize][level as usize] = move_0;
            pv_depth[level as usize] =
                pv_depth[(level + 1 as libc::c_int) as usize];
            j = level + 1 as libc::c_int;
            while j < pv_depth[(level + 1 as libc::c_int) as usize] {
                pv[level as usize][j as usize] =
                    pv[(level + 1 as libc::c_int) as usize][j as usize];
                j += 1
            }
        }
        if best >= beta {
            advance_move(move_index);
            if use_hash != 0 && allow_midgame_hash_update != 0 {
                add_hash_extended(0 as libc::c_int, best,
                                  best_list.as_mut_ptr(),
                                  8 as libc::c_int | 1 as libc::c_int,
                                  remains, selectivity);
            }
            return best
        }
        searched += 1;
        i += 1;
        best_list_index += 1
    }
    /* Post-processing */
    if move_count[disks_played as usize] > 0 as libc::c_int {
        advance_move(best_move_index);
        if use_hash != 0 && allow_midgame_hash_update != 0 {
            if best > alpha {
                add_hash_extended(0 as libc::c_int, best,
                                  best_list.as_mut_ptr(),
                                  8 as libc::c_int | 4 as libc::c_int,
                                  remains, selectivity);
            } else {
                add_hash_extended(0 as libc::c_int, best,
                                  best_list.as_mut_ptr(),
                                  8 as libc::c_int | 2 as libc::c_int,
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
                         0 as libc::c_int + 2 as libc::c_int - side_to_move,
                         -beta, -alpha, allow_hash, allow_mpc,
                         0 as libc::c_int);
        hash1 ^= hash_flip_color1;
        hash2 ^= hash_flip_color2;
        return curr_val
    } else {
        pv_depth[level as usize] = level;
        return terminal_evaluation(side_to_move)
    };
}
/*
  PERTURB_SCORE
  Perturbs SCORE by PERTURBATION if it doesn't appear to be
  a midgame win.
*/
unsafe extern "C" fn perturb_score(mut score: libc::c_int,
                                   mut perturbation: libc::c_int)
 -> libc::c_int {
    if abs(score) < 29000 as libc::c_int - 4000 as libc::c_int {
        return score + perturbation
    } else { return score };
}
/*
   ROOT_TREE_SEARCH
   The recursive tree search function that is to be called only
   for the root of the search tree.
*/
#[no_mangle]
pub unsafe extern "C" fn root_tree_search(mut level: libc::c_int,
                                          mut max_depth: libc::c_int,
                                          mut side_to_move: libc::c_int,
                                          mut alpha: libc::c_int,
                                          mut beta: libc::c_int,
                                          mut allow_hash: libc::c_int,
                                          mut allow_mpc: libc::c_int,
                                          mut void_legal: libc::c_int)
 -> libc::c_int {
    let mut buffer: [libc::c_char; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut curr_val: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut pre_best: libc::c_int = 0;
    let mut searched: libc::c_int = 0;
    let mut move_0: libc::c_int = 0;
    let mut move_index: libc::c_int = 0;
    let mut best_move_index: libc::c_int = 0;
    let mut hash_hit: libc::c_int = 0;
    let mut pre_depth: libc::c_int = 0;
    let mut update_pv: libc::c_int = 0;
    let mut remains: libc::c_int = 0;
    let mut use_hash: libc::c_int = 0;
    let mut pre_search_done: libc::c_int = 0;
    let mut curr_alpha: libc::c_int = 0;
    let mut best_index: libc::c_int = 0;
    let mut best_score: libc::c_int = 0;
    let mut best_list_index: libc::c_int = 0;
    let mut best_list_length: libc::c_int = 0;
    let mut selectivity: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut best_list: [libc::c_int; 4] = [0; 4];
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    remains = max_depth - level;
    nodes.lo = nodes.lo.wrapping_add(1);
    use_hash =
        (remains >= 2 as libc::c_int && 1 as libc::c_int != 0 &&
             allow_hash != 0) as libc::c_int;
    if 1 as libc::c_int != 0 && allow_mpc != 0 {
        selectivity = 1 as libc::c_int
    } else { selectivity = 0 as libc::c_int }
    /* Hash strategy at the root: Only use hash table information for
       move ordering purposes.  This guarantees that score perturbation
       is applied for all moves. */
    hash_hit = 0 as libc::c_int;
    if use_hash != 0 && allow_midgame_hash_probe != 0 {
        find_hash(&mut entry, 0 as libc::c_int);
        if entry.draft as libc::c_int != 0 as libc::c_int {
            hash_hit = 1 as libc::c_int
        }
    }
    pre_search_done = 0 as libc::c_int;
    if get_ponder_move() == 0 {
        if alpha <= -(29000 as libc::c_int) && beta >= 29000 as libc::c_int {
            sprintf(buffer.as_mut_ptr(),
                    b"[-inf,inf]:\x00" as *const u8 as *const libc::c_char);
        } else if alpha <= -(29000 as libc::c_int) &&
                      beta < 29000 as libc::c_int {
            sprintf(buffer.as_mut_ptr(),
                    b"[-inf,%.1f]:\x00" as *const u8 as *const libc::c_char,
                    beta as libc::c_double / 128.0f64);
        } else if alpha > -(29000 as libc::c_int) &&
                      beta >= 29000 as libc::c_int {
            sprintf(buffer.as_mut_ptr(),
                    b"[%.1f,inf]:\x00" as *const u8 as *const libc::c_char,
                    alpha as libc::c_double / 128.0f64);
        } else {
            sprintf(buffer.as_mut_ptr(),
                    b"[%.1f,%.1f]:\x00" as *const u8 as *const libc::c_char,
                    alpha as libc::c_double / 128.0f64,
                    beta as libc::c_double / 128.0f64);
        }
        clear_sweep();
        send_sweep(b"%-14s \x00" as *const u8 as *const libc::c_char,
                   buffer.as_mut_ptr());
    }
    /* Full negascout search */
    searched = 0 as libc::c_int;
    best = -(12345678 as libc::c_int);
    best_move_index = -(1 as libc::c_int);
    curr_alpha = alpha;
    best_list_length = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        best_list[i as usize] = 0 as libc::c_int;
        i += 1
    }
    if pre_search_done == 0 {
        move_count[disks_played as usize] = 0 as libc::c_int;
        if hash_hit != 0 {
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                if valid_move(entry.move_0[i as usize], side_to_move) != 0 {
                    let fresh1 = best_list_length;
                    best_list_length = best_list_length + 1;
                    best_list[fresh1 as usize] = entry.move_0[i as usize]
                }
                i += 1
            }
        }
    }
    i = 0 as libc::c_int;
    best_list_index = 0 as libc::c_int;
    loop 
         /* Try the hash table move(s) first if feasible */
         {
        if pre_search_done == 0 && best_list_index < best_list_length {
            move_count[disks_played as usize] += 1;
            move_index = 0 as libc::c_int;
            while sorted_move_order[disks_played as
                                        usize][move_index as usize] !=
                      best_list[best_list_index as usize] {
                move_index += 1
            }
        } else {
            /* Otherwise use information from shallow searches */
            if pre_search_done == 0 {
                if remains < 10 as libc::c_int {
                    pre_depth = 1 as libc::c_int
                } else { pre_depth = 2 as libc::c_int }
                pre_best = -(12345678 as libc::c_int);
                move_index = 0 as libc::c_int;
                while move_index < 60 as libc::c_int {
                    let mut already_checked: libc::c_int = 0;
                    move_0 =
                        sorted_move_order[disks_played as
                                              usize][move_index as usize];
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
                           make_move(side_to_move, move_0, 1 as libc::c_int)
                               != 0 as libc::c_int {
                        curr_val =
                            -tree_search(level + 1 as libc::c_int,
                                         level + pre_depth,
                                         0 as libc::c_int + 2 as libc::c_int -
                                             side_to_move,
                                         -(12345678 as libc::c_int),
                                         -pre_best, 0 as libc::c_int,
                                         0 as libc::c_int, 1 as libc::c_int);
                        pre_best =
                            if pre_best > curr_val {
                                pre_best
                            } else { curr_val };
                        unmake_move(side_to_move, move_0);
                        evals[disks_played as usize][move_0 as usize] =
                            curr_val;
                        feas_index_list[disks_played as
                                            usize][move_count[disks_played as
                                                                  usize] as
                                                       usize] = move_index;
                        move_count[disks_played as usize] += 1
                    }
                    move_index += 1
                }
                pre_search_done = 1 as libc::c_int
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
            j = i + 1 as libc::c_int;
            while j < move_count[disks_played as usize] {
                let mut cand_move: libc::c_int = 0;
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
        if get_ponder_move() == 0 {
            send_sweep(b"%c%c\x00" as *const u8 as *const libc::c_char,
                       'a' as i32 + move_0 % 10 as libc::c_int -
                           1 as libc::c_int,
                       '0' as i32 + move_0 / 10 as libc::c_int);
        }
        make_move(side_to_move, move_0, 1 as libc::c_int);
        update_pv = 0 as libc::c_int;
        offset = score_perturbation[move_0 as usize];
        if searched == 0 as libc::c_int {
            curr_val =
                perturb_score(-tree_search(level + 1 as libc::c_int,
                                           max_depth,
                                           0 as libc::c_int + 2 as libc::c_int
                                               - side_to_move,
                                           -(beta - offset),
                                           -(curr_alpha - offset), allow_hash,
                                           allow_mpc, 1 as libc::c_int),
                              offset);
            best = curr_val;
            best_move_index = move_index;
            update_pv = 1 as libc::c_int;
            best_mid_root_move = move_0
        } else {
            curr_alpha = if best > curr_alpha { best } else { curr_alpha };
            curr_val =
                perturb_score(-tree_search(level + 1 as libc::c_int,
                                           max_depth,
                                           0 as libc::c_int + 2 as libc::c_int
                                               - side_to_move,
                                           -(curr_alpha - offset +
                                                 1 as libc::c_int),
                                           -(curr_alpha - offset), allow_hash,
                                           allow_mpc, 1 as libc::c_int),
                              offset);
            if curr_val > curr_alpha && curr_val < beta {
                curr_val =
                    perturb_score(-tree_search(level + 1 as libc::c_int,
                                               max_depth,
                                               0 as libc::c_int +
                                                   2 as libc::c_int -
                                                   side_to_move,
                                               -(beta - offset),
                                               12345678 as libc::c_int,
                                               allow_hash, allow_mpc,
                                               1 as libc::c_int), offset);
                if curr_val > best {
                    best = curr_val;
                    best_move_index = move_index;
                    update_pv = 1 as libc::c_int;
                    if is_panic_abort() == 0 && force_return == 0 {
                        best_mid_root_move = move_0
                    }
                }
            } else if curr_val > best {
                best = curr_val;
                best_move_index = move_index;
                update_pv = 1 as libc::c_int
            }
        }
        unmake_move(side_to_move, move_0);
        if is_panic_abort() != 0 || force_return != 0 {
            return -(27000 as libc::c_int)
        }
        evals[disks_played as usize][move_0 as usize] = curr_val;
        if get_ponder_move() == 0 {
            if update_pv != 0 {
                if curr_val <= alpha {
                    send_sweep(b"<%.2f\x00" as *const u8 as
                                   *const libc::c_char,
                               (curr_val + 1 as libc::c_int) as libc::c_double
                                   / 128.0f64);
                } else if curr_val >= beta {
                    send_sweep(b">%.2f\x00" as *const u8 as
                                   *const libc::c_char,
                               (curr_val - 1 as libc::c_int) as libc::c_double
                                   / 128.0f64);
                } else {
                    send_sweep(b"=%.2f\x00" as *const u8 as
                                   *const libc::c_char,
                               curr_val as libc::c_double / 128.0f64);
                }
            }
            send_sweep(b" \x00" as *const u8 as *const libc::c_char);
            if update_pv != 0 && searched > 0 as libc::c_int && echo != 0 &&
                   max_depth >= 10 as libc::c_int {
                display_sweep(stdout);
            }
        }
        if update_pv != 0 {
            midgame_c__update_best_list(best_list.as_mut_ptr(), move_0,
                                        best_list_index, best_list_length);
            pv[level as usize][level as usize] = move_0;
            pv_depth[level as usize] =
                pv_depth[(level + 1 as libc::c_int) as usize];
            j = level + 1 as libc::c_int;
            while j < pv_depth[(level + 1 as libc::c_int) as usize] {
                pv[level as usize][j as usize] =
                    pv[(level + 1 as libc::c_int) as usize][j as usize];
                j += 1
            }
        }
        if best >= beta {
            advance_move(move_index);
            if use_hash != 0 && allow_midgame_hash_update != 0 {
                add_hash_extended(0 as libc::c_int, best,
                                  best_list.as_mut_ptr(),
                                  8 as libc::c_int | 1 as libc::c_int,
                                  remains, selectivity);
            }
            return best
        }
        /* For symmetry reasons, the score for any move is the score of the
           position for the initial position. */
        if disks_played == 0 as libc::c_int {
            add_hash_extended(0 as libc::c_int, best, best_list.as_mut_ptr(),
                              8 as libc::c_int | 4 as libc::c_int, remains,
                              selectivity);
            return best
        }
        searched += 1;
        i += 1;
        best_list_index += 1
    }
    /* Post-processing */
    if move_count[disks_played as usize] > 0 as libc::c_int {
        advance_move(best_move_index);
        if use_hash != 0 && allow_midgame_hash_update != 0 {
            if best > alpha {
                add_hash_extended(0 as libc::c_int, best,
                                  best_list.as_mut_ptr(),
                                  8 as libc::c_int | 4 as libc::c_int,
                                  remains, selectivity);
            } else {
                add_hash_extended(0 as libc::c_int, best,
                                  best_list.as_mut_ptr(),
                                  8 as libc::c_int | 2 as libc::c_int,
                                  remains, selectivity);
            }
        }
        return best
    } else if void_legal != 0 {
        /* No feasible moves */
        hash1 ^= hash_flip_color1;
        hash2 ^= hash_flip_color2;
        curr_val =
            -root_tree_search(level, max_depth,
                              0 as libc::c_int + 2 as libc::c_int -
                                  side_to_move, -beta, -alpha, allow_hash,
                              allow_mpc, 0 as libc::c_int);
        hash1 ^= hash_flip_color1;
        hash2 ^= hash_flip_color2;
        return curr_val
    } else {
        pv_depth[level as usize] = level;
        return terminal_evaluation(side_to_move)
    };
}
/*
  PROTECTED_ONE_PLY_SEARCH
  Chooses the move maximizing the static evaluation function
  while avoiding all moves which allow an immediate loss
  (if that is possible).
*/
unsafe extern "C" fn protected_one_ply_search(mut side_to_move: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut move_0: libc::c_int = 0;
    let mut depth_one_score: libc::c_int = 0;
    let mut depth_two_score: libc::c_int = 0;
    let mut best_score_restricted: libc::c_int = 0;
    let mut best_score_unrestricted: libc::c_int = 0;
    let mut best_move_restricted: libc::c_int = 0;
    let mut best_move_unrestricted: libc::c_int = 0;
    generate_all(side_to_move);
    best_score_restricted = -(12345678 as libc::c_int);
    best_score_unrestricted = -(12345678 as libc::c_int);
    best_move_restricted = 0 as libc::c_int;
    best_move_unrestricted = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < move_count[disks_played as usize] {
        nodes.lo = nodes.lo.wrapping_add(1);
        move_0 = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, move_0, 1 as libc::c_int);
        evaluations.lo = evaluations.lo.wrapping_add(1);
        depth_one_score =
            -pattern_evaluation(0 as libc::c_int + 2 as libc::c_int -
                                    side_to_move);
        depth_two_score =
            -tree_search(1 as libc::c_int, 2 as libc::c_int,
                         0 as libc::c_int + 2 as libc::c_int - side_to_move,
                         -(12345678 as libc::c_int), 12345678 as libc::c_int,
                         0 as libc::c_int, 0 as libc::c_int,
                         0 as libc::c_int);
        unmake_move(side_to_move, move_0);
        if depth_one_score > best_score_unrestricted {
            best_score_unrestricted = depth_one_score;
            best_move_unrestricted = move_0
        }
        if depth_two_score > -(29000 as libc::c_int) &&
               depth_one_score > best_score_restricted {
            best_score_restricted = depth_one_score;
            best_move_restricted = move_0
        }
        i += 1
    }
    pv_depth[0 as libc::c_int as usize] = 1 as libc::c_int;
    if best_score_restricted > -(12345678 as libc::c_int) {
        /* No immediate loss */
        pv[0 as libc::c_int as usize][0 as libc::c_int as usize] =
            best_move_restricted;
        return best_score_restricted
    } else {
        pv[0 as libc::c_int as usize][0 as libc::c_int as usize] =
            best_move_unrestricted;
        return best_score_unrestricted
    };
}
/*
   MIDDLE_GAME
   side_to_move = the side whose turn it is to move
*/
#[no_mangle]
pub unsafe extern "C" fn middle_game(mut side_to_move: libc::c_int,
                                     mut max_depth: libc::c_int,
                                     mut update_evals: libc::c_int,
                                     mut eval_info: *mut EvaluationType)
 -> libc::c_int {
    let mut eval_str =
        0 as *mut libc::c_char; /* Disable I.D. in this function */
    let mut node_val: libc::c_double = 0.;
    let mut val: libc::c_int = 0;
    let mut old_val: libc::c_int = 0;
    let mut adjusted_val: libc::c_int = 0;
    let mut initial_depth: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut alpha: libc::c_int = 0;
    let mut beta: libc::c_int = 0;
    let mut enable_mpc: libc::c_int = 0;
    let mut base_stage: libc::c_int = 0;
    let mut full_length_line: libc::c_int = 0;
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    last_panic_check = 0.0f64;
    counter_phase = 0 as libc::c_int;
    piece_count[0 as libc::c_int as usize][disks_played as usize] =
        disc_count(0 as libc::c_int);
    piece_count[2 as libc::c_int as usize][disks_played as usize] =
        disc_count(2 as libc::c_int);
    base_stage =
        disc_count(0 as libc::c_int) + disc_count(2 as libc::c_int) -
            4 as libc::c_int;
    val = 0 as libc::c_int;
    old_val = --(27000 as libc::c_int);
    enable_mpc = (max_depth >= 9 as libc::c_int) as libc::c_int;
    initial_depth =
        if 1 as libc::c_int > max_depth - 2 as libc::c_int {
            1 as libc::c_int
        } else { (max_depth) - 2 as libc::c_int };
    initial_depth = max_depth;
    *eval_info =
        create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION, 0 as libc::c_int,
                         0.0f64, 0 as libc::c_int, 0 as libc::c_int);
    depth = initial_depth;
    while depth <= max_depth {
        alpha = -(12345678 as libc::c_int);
        beta = 12345678 as libc::c_int;
        inherit_move_lists(disks_played + max_depth);
        /* The actual search */
        if depth == 1 as libc::c_int {
            /* Fix to make it harder to wipe out depth-1 Zebra */
            val = protected_one_ply_search(side_to_move)
        } else if enable_mpc != 0 {
            val =
                root_tree_search(0 as libc::c_int, depth, side_to_move, alpha,
                                 beta, 1 as libc::c_int, 1 as libc::c_int,
                                 1 as libc::c_int);
            if force_return == 0 && is_panic_abort() == 0 &&
                   (val <= alpha || val >= beta) {
                val =
                    root_tree_search(0 as libc::c_int, depth, side_to_move,
                                     -(12345678 as libc::c_int),
                                     12345678 as libc::c_int,
                                     1 as libc::c_int, 1 as libc::c_int,
                                     1 as libc::c_int)
            }
        } else {
            val =
                root_tree_search(0 as libc::c_int, depth, side_to_move, alpha,
                                 beta, 1 as libc::c_int, 0 as libc::c_int,
                                 1 as libc::c_int);
            if is_panic_abort() == 0 && force_return == 0 {
                if val <= alpha {
                    val =
                        root_tree_search(0 as libc::c_int, depth,
                                         side_to_move,
                                         -(29000 as libc::c_int), alpha,
                                         1 as libc::c_int, 0 as libc::c_int,
                                         1 as libc::c_int)
                } else if val >= beta {
                    val =
                        root_tree_search(0 as libc::c_int, depth,
                                         side_to_move, beta,
                                         29000 as libc::c_int,
                                         1 as libc::c_int, 0 as libc::c_int,
                                         1 as libc::c_int)
                }
            }
        }
        /* Adjust scores and PV if search is aborted */
        if is_panic_abort() != 0 || force_return != 0 {
            pv[0 as libc::c_int as usize][0 as libc::c_int as usize] =
                best_mid_root_move;
            pv_depth[0 as libc::c_int as usize] = 1 as libc::c_int;
            hash_expand_pv(side_to_move, 0 as libc::c_int, 4 as libc::c_int,
                           12345678 as libc::c_int);
            if base_stage + depth - 2 as libc::c_int >= 0 as libc::c_int &&
                   stage_reached[(base_stage + depth - 2 as libc::c_int) as
                                     usize] != 0 {
                val =
                    stage_score[(base_stage + depth - 2 as libc::c_int) as
                                    usize];
                if side_to_move == 2 as libc::c_int { val = -val }
            } else { val = old_val }
        }
        /* Check if the search info corresponds to a variation of
           depth exactly DEPTH which would mean that the search
           gives new score information */
        full_length_line = 0 as libc::c_int;
        find_hash(&mut entry, 0 as libc::c_int);
        if force_return == 0 && is_panic_abort() == 0 &&
               entry.draft as libc::c_int != 0 as libc::c_int &&
               valid_move(entry.move_0[0 as libc::c_int as usize],
                          side_to_move) != 0 &&
               entry.draft as libc::c_int == depth {
            full_length_line = 1 as libc::c_int
        }
        /* Update the stored scores */
        if (stage_reached[(base_stage + depth) as usize] == 0 ||
                full_length_line != 0) && update_evals != 0 {
            stage_reached[(base_stage + depth) as usize] = 1 as libc::c_int;
            if side_to_move == 0 as libc::c_int {
                stage_score[(base_stage + depth) as usize] = val
            } else { stage_score[(base_stage + depth) as usize] = -val }
        }
        /* Adjust the eval for oscillations odd/even by simply averaging the
           last two stages (if they are available). */
        if stage_reached[(base_stage + depth) as usize] != 0 &&
               stage_reached[(base_stage + depth - 1 as libc::c_int) as usize]
                   != 0 && update_evals != 0 {
            if side_to_move == 0 as libc::c_int {
                adjusted_val =
                    (stage_score[(base_stage + depth) as usize] +
                         stage_score[(base_stage + depth - 1 as libc::c_int)
                                         as usize]) / 2 as libc::c_int
            } else {
                adjusted_val =
                    -(stage_score[(base_stage + depth) as usize] +
                          stage_score[(base_stage + depth - 1 as libc::c_int)
                                          as usize]) / 2 as libc::c_int
            }
        } else if depth == initial_depth {
            adjusted_val = val
        } else { adjusted_val = (val + old_val) / 2 as libc::c_int }
        /* In case the search reached the end of the game, the score
           must be converted into an endgame score */
        if val >= 29000 as libc::c_int {
            *eval_info =
                create_eval_info(EXACT_EVAL, WON_POSITION,
                                 (val - 29000 as libc::c_int) *
                                     128 as libc::c_int, 0.0f64, depth,
                                 0 as libc::c_int)
        } else if val <= -(29000 as libc::c_int) {
            *eval_info =
                create_eval_info(EXACT_EVAL, LOST_POSITION,
                                 (val + 29000 as libc::c_int) *
                                     128 as libc::c_int, 0.0f64, depth,
                                 0 as libc::c_int)
        } else {
            *eval_info =
                create_eval_info(MIDGAME_EVAL, UNSOLVED_POSITION,
                                 adjusted_val, 0.0f64, depth,
                                 0 as libc::c_int)
        }
        /* Display and store search info */
        if depth == max_depth {
            clear_status();
            send_status(b"--> \x00" as *const u8 as *const libc::c_char);
            if is_panic_abort() != 0 || force_return != 0 {
                send_status(b"*\x00" as *const u8 as *const libc::c_char);
            } else {
                send_status(b" \x00" as *const u8 as *const libc::c_char);
            }
            send_status(b"%2d  \x00" as *const u8 as *const libc::c_char,
                        depth);
            eval_str = produce_eval_text(*eval_info, 1 as libc::c_int);
            send_status(b"%-10s  \x00" as *const u8 as *const libc::c_char,
                        eval_str);
            free(eval_str as *mut libc::c_void);
            node_val = counter_value(&mut nodes);
            send_status_nodes(node_val);
            if get_ponder_move() != 0 {
                send_status(b"{%c%c} \x00" as *const u8 as
                                *const libc::c_char,
                            'a' as i32 + get_ponder_move() % 10 as libc::c_int
                                - 1 as libc::c_int,
                            '0' as i32 +
                                get_ponder_move() / 10 as libc::c_int);
            }
            hash_expand_pv(side_to_move, 0 as libc::c_int, 4 as libc::c_int,
                           12345678 as libc::c_int);
            send_status_pv(pv[0 as libc::c_int as usize].as_mut_ptr(),
                           max_depth);
            send_status_time(get_elapsed_time());
            if get_elapsed_time() != 0.0f64 {
                send_status(b"%6.0f %s\x00" as *const u8 as
                                *const libc::c_char,
                            node_val / (get_elapsed_time() + 0.001f64),
                            b"nps\x00" as *const u8 as *const libc::c_char);
            }
        }
        if is_panic_abort() != 0 || force_return != 0 { break ; }
        /* Check if search time or adjusted search time are long enough
           for the search to be discontinued */
        old_val = adjusted_val;
        if do_check_midgame_abort != 0 {
            if above_recommended() != 0 ||
                   extended_above_recommended() != 0 &&
                       depth >= frozen_ponder_depth {
                set_midgame_abort();
                break ;
            }
        }
        depth += 1
    }
    root_eval = val;
    return pv[0 as libc::c_int as usize][0 as libc::c_int as usize];
}
