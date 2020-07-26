use crate::src::zebra::EvaluationType;
use crate::src::counter::{adjust_counter, counter_value, reset_counter};
use crate::src::search::{nodes, total_time, total_evaluations, total_nodes, setup_search, disc_count};
use crate::src::globals::{pv_depth, pv, board, score_sheet_row, black_moves};
use crate::src::osfbook::clear_osf;
use crate::src::getcoeff::clear_coeffs;
use crate::src::hash::{free_hash, determine_hash_values};
use crate::src::unflip::init_flip_stack;
use crate::src::timer::clear_ponder_times;
use crate::src::eval::init_eval;
use crate::src::end::setup_end;
use crate::src::midgame::setup_midgame;
use crate::src::moves::disks_played;


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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EvaluatedMove {
    pub eval: EvaluationType,
    pub side_to_move: i32,
    pub move_0: i32,
    pub pv_depth: i32,
    pub pv: [i32; 60],
}
pub const BOOK_MOVE: C2RustUnnamed = 1;
pub type C2RustUnnamed = u32;
pub const ENDGAME_MOVE: C2RustUnnamed = 3;
pub const MIDGAME_MOVE: C2RustUnnamed = 2;
pub const INTERRUPTED_MOVE: C2RustUnnamed = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CandidateMove {
    pub move_0: i32,
    pub score: i32,
    pub flags: i32,
    pub parent_flags: i32,
}

/* The maximum length of any system path. */
pub static mut forced_opening: *const i8 = 0 as *const i8;
pub static mut last_time_used: f64 = 0.;
pub static mut max_depth_reached: i32 = 0;
pub static mut use_log_file: i32 = 1 as i32;
pub static mut play_human_openings: i32 = 1 as i32;
pub static mut play_thor_match_openings: i32 = 1 as i32;
pub static mut game_evaluated_count: i32 = 0;
pub static mut komi: i32 = 0 as i32;
pub static mut prefix_move: i32 = 0 as i32;
pub static mut endgame_performed: [i32; 3] = [0; 3];
pub static mut evaluated_list: [EvaluatedMove; 60] =
    [EvaluatedMove{eval:
    EvaluationType{type_0: MIDGAME_EVAL,
        res: WON_POSITION,
        score: 0,
        confidence: 0.,
        search_depth: 0,
        is_book: 0,},
        side_to_move: 0,
        move_0: 0,
        pv_depth: 0,
        pv: [0; 60],}; 60];
/*
  TOGGLE_STATUS_LOG
  Enable/disable the use of logging all the output that the
  text version of Zebra would output to the screen.
*/

pub unsafe fn toggle_status_log(mut write_log: i32) {
    use_log_file = write_log;
}

/*
  SET_KOMI
  Set the endgame komi value.
*/

pub unsafe fn set_komi(mut in_komi: i32) {
    komi = in_komi;
}
/*
  TOGGLE_HUMAN_OPENINGS
  Specifies whether the Thor statistics should be queried for
  openings moves before resorting to the usual opening book.
*/

pub unsafe fn toggle_human_openings(mut toggle: i32) {
    play_human_openings = toggle;
}
/*
  TOGGLE_THOR_MATCH_OPENINGS
  Specifies whether matching Thor games are used as opening book
  before resorting to the usual opening book.
*/

pub unsafe fn toggle_thor_match_openings(mut toggle: i32) {
    play_thor_match_openings = toggle;
}
/*
  SET_FORCED_OPENING
  Specifies an opening line that Zebra is forced to follow when playing.
*/

pub unsafe fn set_forced_opening(mut opening_str:
                                 *const i8) {
    forced_opening = opening_str;
}

/*
  COMPARE_EVAL
  Comparison function for two evals.  Same return value conventions
  as QuickSort.
*/
pub unsafe fn compare_eval(mut e1: EvaluationType,
                       mut e2: EvaluationType) -> i32 {
    if e1.type_0 as u32 == WLD_EVAL as i32 as u32 ||
        e1.type_0 as u32 ==
            EXACT_EVAL as i32 as u32 {
        if e1.score > 0 as i32 { e1.score += 100000 as i32 }
    }
    if e2.type_0 as u32 == WLD_EVAL as i32 as u32 ||
        e2.type_0 as u32 ==
            EXACT_EVAL as i32 as u32 {
        if e2.score > 0 as i32 { e2.score += 100000 as i32 }
    }
    return e1.score - e2.score;
}

/*
  GET_EVALUATED_COUNT
  GET_EVALUATED
  Accessor functions for the data structure filled by extended_compute_move().
*/

pub unsafe fn get_evaluated_count() -> i32 {
    return game_evaluated_count;
}

pub unsafe fn get_evaluated(mut index: i32)
                            -> EvaluatedMove {
    return evaluated_list[index as usize];
}
/*
  GET_SEARCH_STATISTICS
  Returns some statistics about the last search made.
*/

pub unsafe fn get_search_statistics(mut max_depth:
                                    *mut i32,
                                    mut node_count:
                                    *mut f64) {
    *max_depth = max_depth_reached;
    if prefix_move != 0 as i32 { *max_depth += 1 }
    adjust_counter(&mut nodes);
    *node_count = counter_value(&mut nodes);
}


/*
  GET_PV
  Returns the principal variation.
*/

pub unsafe fn get_pv(mut destin: *mut i32) -> i32 {
    let mut i: i32 = 0;
    if prefix_move == 0 as i32 {
        i = 0 as i32;
        while i < pv_depth[0 as i32 as usize] {
            *destin.offset(i as isize) =
                pv[0 as i32 as usize][i as usize];
            i += 1
        }
        return pv_depth[0 as i32 as usize]
    } else {
        *destin.offset(0 as i32 as isize) = prefix_move;
        i = 0 as i32;
        while i < pv_depth[0 as i32 as usize] {
            *destin.offset((i + 1 as i32) as isize) =
                pv[0 as i32 as usize][i as usize];
            i += 1
        }
        return pv_depth[0 as i32 as usize] + 1 as i32
    };
}
/*
   GLOBAL_TERMINATE
   Free all dynamically allocated memory.
*/

pub unsafe fn global_terminate() {
    free_hash();
    clear_coeffs();
    clear_osf();
}

pub unsafe fn engine_game_init() {
    setup_search();
    setup_midgame();
    setup_end();
    init_eval();
    clear_ponder_times();
    reset_counter(&mut total_nodes);
    reset_counter(&mut total_evaluations);
    init_flip_stack();
    total_time = 0.0f64;
    max_depth_reached = 0 as i32;
    last_time_used = 0.0f64;
    endgame_performed[2 as i32 as usize] = 0 as i32;
    endgame_performed[0 as i32 as usize] =
        endgame_performed[2 as i32 as usize];
}

pub unsafe fn setup_game_clear_board() {
    let mut i = 0 as i32;
    while i < 10 as i32 {
        let mut j = 0 as i32;
        while j < 10 as i32 {
            let mut pos = 10 as i32 * i + j;
            if i == 0 as i32 || i == 9 as i32 ||
                j == 0 as i32 || j == 9 as i32 {
                board[pos as usize] = 3 as i32
            } else { board[pos as usize] = 1 as i32 }
            j += 1
        }
        i += 1
    }
}

pub unsafe fn setup_game_board_normal(side_to_move: *mut i32) {
    board[54 as i32 as usize] = 0 as i32;
    board[45 as i32 as usize] = board[54 as i32 as usize];
    board[55 as i32 as usize] = 2 as i32;
    board[44 as i32 as usize] = board[55 as i32 as usize];
    *side_to_move = 0 as i32
}

pub unsafe fn setup_game_finalize(side_to_move:  *mut i32) {
    disks_played =
        disc_count(0 as i32) + disc_count(2 as i32) -
            4 as i32;
    determine_hash_values(*side_to_move, board.as_mut_ptr());
    /* Make the game score look right */
    if *side_to_move == 0 as i32 {
        score_sheet_row = -(1 as i32)
    } else {
        black_moves[0 as i32 as usize] = -(1 as i32);
        score_sheet_row = 0 as i32
    };
}


pub unsafe fn setup_non_file_based_game(mut side_to_move: *mut i32) {
    setup_game_clear_board();
    setup_game_board_normal(side_to_move);
    setup_game_finalize(side_to_move);
}
