use crate::src::zebra::EvaluationType;
use crate::src::counter::{adjust_counter, counter_value, reset_counter, add_counter};
use crate::src::search::{nodes, total_time, total_evaluations, total_nodes, setup_search, disc_count, complete_pv, get_ponder_move, evaluations, set_current_eval, create_eval_info, root_eval, force_return, clear_pv, evals, clear_ponder_move, set_ponder_move, float_move, sort_moves};
use crate::src::globals::{pv_depth, pv, board, score_sheet_row, black_moves, piece_count};
use crate::src::osfbook::{clear_osf, get_book_move, fill_move_alternatives, check_forced_opening, g_book};
use crate::src::getcoeff::{clear_coeffs, post_init_coeffs, eval_adjustment, init_coeffs_calculate_patterns, process_coeffs_from_fn_source, init_memory_handler, CoeffAdjustments, remove_coeffs};
use crate::src::hash::{free_hash, determine_hash_values, init_hash, find_hash, HashEntry};
use crate::src::timer::{clear_ponder_times, init_timer, time_t, clear_panic_abort, get_elapsed_time, is_panic_abort, determine_move_time, toggle_abort_check, ponder_depth, add_ponder_time, get_real_timer, start_move};
use crate::src::end::{setup_end, end_game};
use crate::src::midgame::{setup_midgame, is_midgame_abort, middle_game, toggle_midgame_hash_usage, toggle_midgame_abort_check, clear_midgame_abort, calculate_perturbation};
use crate::src::moves::{disks_played, valid_move, move_list, move_count, generate_all, unmake_move, make_move};
use crate::src::stable::init_stable;
use crate::src::probcut::{init_probcut, end_mpc_depth, use_end_cut, mpc_cut};
use crate::src::myrandom::{my_srandom, my_random};
use crate::src::stubs::{abs};
use crate::src::error::{FrontEnd};
use crate::src::display::{echo, display_pv};
use crate::src::thordb::{ThorDatabase};
use engine_traits::CoeffSource;
use flip::unflip::init_flip_stack;
use crate::src::zebra::EvalResult::{WON_POSITION, UNSOLVED_POSITION};
use crate::src::zebra::EvalType::{MIDGAME_EVAL, INTERRUPTED_EVAL, UNDEFINED_EVAL, FORCED_EVAL, PASS_EVAL, EXACT_EVAL, WLD_EVAL};


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
/*
  SET_FORCED_OPENING
  Specifies an opening line that Zebra is forced to follow when playing.
*/
pub static mut forced_opening: *const i8 = 0 as *const i8;
pub static mut last_time_used: f64 = 0.;
pub static mut max_depth_reached: i32 = 0;
pub static mut use_log_file: i32 = 1;
pub static mut play_human_openings: i32 = 1;

/*
  TOGGLE_THOR_MATCH_OPENINGS
  Specifies whether matching Thor games are used as opening book
  before resorting to the usual opening book.
*/
pub static mut play_thor_match_openings: i32 = 1;
pub static mut game_evaluated_count: i32 = 0;
pub static mut komi: i32 = 0;
pub static mut prefix_move: i32 = 0;
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

pub unsafe fn toggle_status_log(write_log: i32) {
    use_log_file = write_log;
}

/*
  SET_KOMI
  Set the endgame komi value.
*/

pub unsafe fn set_komi(in_komi: i32) {
    komi = in_komi;
}
/*
  TOGGLE_HUMAN_OPENINGS
  Specifies whether the Thor statistics should be queried for
  openings moves before resorting to the usual opening book.
*/

pub unsafe fn toggle_human_openings(toggle: i32) {
    play_human_openings = toggle;
}

/*
  COMPARE_EVAL
  Comparison function for two evals.  Same return value conventions
  as QuickSort.
*/
pub fn compare_eval(mut e1: EvaluationType, mut e2: EvaluationType) -> i32 {
    if e1.type_0 == WLD_EVAL || e1.type_0 == EXACT_EVAL {
        if e1.score > 0 {
            e1.score += 100000
        }
    }
    if e2.type_0 == WLD_EVAL || e2.type_0 == EXACT_EVAL {
        if e2.score > 0 {
            e2.score += 100000
        }
    }
    e1.score - e2.score
}

/*
  GET_EVALUATED_COUNT
  GET_EVALUATED
  Accessor functions for the data structure filled by extended_compute_move().
*/

pub unsafe fn get_evaluated_count() -> i32 {
    return game_evaluated_count;
}

pub unsafe fn get_evaluated(index: i32)
                            -> EvaluatedMove {
    return evaluated_list[index as usize];
}
/*
  GET_SEARCH_STATISTICS
  Returns some statistics about the last search made.
*/

pub unsafe fn get_search_statistics(max_depth: &mut i32, node_count: &mut f64) {
    *max_depth = max_depth_reached;
    if prefix_move != 0 {
        *max_depth += 1
    }
    adjust_counter(&mut nodes);
    *node_count = counter_value(&mut nodes);
}


/*
  GET_PV
  Returns the principal variation.
*/

pub unsafe fn get_pv(destin: &mut [i32]) -> i32 {
    let mut i = 0;
    return if prefix_move == 0 {
        i = 0;
        while i < pv_depth[0] {
            destin[i as usize] = pv[0][i as usize];
            i += 1
        }
        pv_depth[0]
    } else {
        destin[0] = prefix_move;
        i = 0;
        while i < pv_depth[0] {
            destin[(i + 1 as i32) as usize] = pv[0][i as usize];
            i += 1
        }
        pv_depth[0] + 1
    };
}
/*
   GLOBAL_TERMINATE
   Free all dynamically allocated memory.
*/

pub unsafe fn global_terminate() {
    free_hash();
    clear_coeffs();
    clear_osf(&mut g_book);
}

pub unsafe fn engine_game_init() {
    setup_search();
    setup_midgame();
    setup_end();
    clear_ponder_times();
    reset_counter(&mut total_nodes);
    reset_counter(&mut total_evaluations);
    init_flip_stack();
    total_time = 0.0f64;
    max_depth_reached = 0;
    last_time_used = 0.0f64;
    endgame_performed[2] = 0;
    endgame_performed[0] = endgame_performed[2];
}

pub const fn create_fresh_board() -> [i32; 128] {
    let mut board_ = [0; 128];
    let mut i = 0;
    while i < 10 {
        let mut j = 0;
        while j < 10 {
            let pos = 10 * i + j;
            if i == 0 || i == 9 || j == 0 || j == 9 {
                board_[pos] = 3
            } else {
                board_[pos] = 1
            }
            j += 1
        }
        i += 1
    }
    board_
}

pub unsafe fn setup_game_finalize(side_to_move:  &mut i32) {
    disks_played = disc_count(0, &board) + disc_count(2, &board) - 4;
    determine_hash_values(*side_to_move, &board);
    /* Make the game score look right */
    if *side_to_move == 0 as i32 {
        score_sheet_row = -(1 as i32)
    } else {
        black_moves[0] = -(1 as i32);
        score_sheet_row = 0 as i32
    };
}


pub unsafe fn setup_non_file_based_game(side_to_move: &mut i32) {
    board = create_fresh_board();
    board[54] = 0;
    board[45] = 0;
    board[55] = 2;
    board[44] = 2;
    *side_to_move = 0;
    setup_game_finalize(side_to_move);
}


pub unsafe fn engine_global_setup<S:CoeffSource, FE: FrontEnd>(
    use_random: i32, hash_bits: i32, coeff_adjustments:
    Option<CoeffAdjustments>, coeffs: S) {
    let mut timer: time_t = 0;
    if use_random != 0 {
        FE::time(&mut timer);
        my_srandom(timer as i32);
    } else { my_srandom(1 as i32); }
    init_hash(hash_bits);

    // inlined init_coeffs
    init_memory_handler();
    process_coeffs_from_fn_source::<FE, _>(coeffs);
    init_coeffs_calculate_patterns();
    if let Some(adjusts) = coeff_adjustments {
        eval_adjustment(adjusts.disc_adjust, adjusts.edge_adjust, adjusts.corner_adjust, adjusts.x_adjust);
    };
    post_init_coeffs();

    init_timer::<FE>();
    init_probcut(&mut mpc_cut, &mut use_end_cut, &mut end_mpc_depth);
    init_stable();
    setup_search();
}

pub trait BoardSource {
    fn fill_board_buffer(&mut self, buffer: &mut [i8; 70]);
    fn fill_buffer_with_side_to_move(&mut self, buffer: &mut [i8; 70]);
    fn report_unrecognized_character(unrecognized: i8);
}


pub unsafe fn process_board_source<S: BoardSource, FE: FrontEnd>(side_to_move: &mut i32, mut file_source: S) {
    let mut buffer: [i8; 70] = [0; 70];
    file_source.fill_board_buffer(&mut buffer);
    let mut token = 0;
    let mut i = 1;
    while i <= 8 as i32 {
        let mut j = 1;
        while j <= 8 as i32 {
            let pos = 10 as i32 * i + j;
            match buffer[token as usize] as i32 {
                42 | 88 => { board[pos as usize] = 0 as i32 }
                79 | 48 => { board[pos as usize] = 2 as i32 }
                45 | 46 => {}
                _ => {
                    let unrecognized = buffer[pos as usize];
                    S::report_unrecognized_character(unrecognized);
                }
            }
            token += 1;
            j += 1
        }
        i += 1
    }
    file_source.fill_buffer_with_side_to_move(&mut buffer);
    if buffer[0] as i32 == 'B' as i32 {
        *side_to_move = 0 as i32
    } else if buffer[0] as i32 ==
        'W' as i32 {
        *side_to_move = 2 as i32
    } else {
        let unrecognized = buffer[0];
        FE::unrecognized_character(unrecognized);
    }
}


pub trait FileBoardSource : BoardSource {
    unsafe fn open(file_name: *const i8) -> Option<Self> where Self: Sized;
}

pub unsafe fn setup_file_based_game<S: FileBoardSource, FE: FrontEnd>(file_name: *const i8, side_to_move: &mut i32) {
    board = create_fresh_board();
    assert!(!file_name.is_null());
    match S::open(file_name) {
        Some(file_source) => process_board_source::<_, FE>(side_to_move, file_source),
        None => {
            FE::cannot_open_game_file(file_name);
        },
    };
    setup_game_finalize(side_to_move);
}

pub unsafe fn generic_setup_game<Source: FileBoardSource, FE: FrontEnd>(file_name: *const i8, side_to_move: &mut i32) {
    if file_name.is_null() {
        setup_non_file_based_game(side_to_move);
    } else {
        setup_file_based_game::<Source, FE>(file_name, side_to_move);
    }
}

pub unsafe fn generic_game_init<Source: FileBoardSource, FE: FrontEnd>(file_name: *const i8, side_to_move: &mut i32) {
    generic_setup_game::<Source, FE>(file_name, side_to_move);
    engine_game_init();
}

pub unsafe fn generic_compute_move<L: ComputeMoveLogger, Out: ComputeMoveOutput, FE: FrontEnd, Thor: ThorDatabase>(side_to_move: i32,
                                                                                               update_all: i32,
                                                                                               my_time: i32,
                                                                                               my_incr: i32,
                                                                                               timed_depth: i32,
                                                                                               book: i32,
                                                                                               mut mid: i32,
                                                                                               exact: i32,
                                                                                               wld: i32,
                                                                                               search_forced: i32,
                                                                                               eval_info: &mut EvaluationType,
                                                                                               logger: &mut Option<L>)
                                                                                               -> i32 {
    let mut book_eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
            res: WON_POSITION,
            score: 0,
            confidence: 0.,
            search_depth: 0,
            is_book: 0,};
    let mut mid_eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
            res: WON_POSITION,
            score: 0,
            confidence: 0.,
            search_depth: 0,
            is_book: 0,};
    let mut end_eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
            res: WON_POSITION,
            score: 0,
            confidence: 0.,
            search_depth: 0,
            is_book: 0,};
    let mut midgame_diff: f64 = 0.;
    let mut midgame_depth: i32 = 0;
    let mut max_depth: i32 = 0;
    let mut endgame_reached: i32 = 0;
    let mut offset: i32 = 0;

    if let Some(logger) = logger {
        let board_ = &board;
        let side_to_move_ = side_to_move;
        L::log_board(logger, board_, side_to_move_);
    }
    /* Initialize various components of the move system */
    piece_count[0][disks_played as usize] =
        disc_count(0 as i32, &board);
    piece_count[2][disks_played as usize] =
        disc_count(2 as i32, &board);
    generate_all(side_to_move);
    determine_hash_values(side_to_move, &board);
    calculate_perturbation();
    if let Some(logger) = logger {
        let moves_generated = move_count[disks_played as usize];
        let move_list_for_disks_played = &move_list[disks_played as usize];

        L::log_moves_generated(logger, moves_generated, move_list_for_disks_played);
    }
    if update_all != 0 {
        reset_counter(&mut evaluations);
        reset_counter(&mut nodes);
    }
    let mut i = 0;
    while i < 100 as i32 {
        evals[disks_played as usize][i as usize] = 0;
        i += 1
    }
    max_depth_reached = 1;
    let empties = 60 as i32 - disks_played;
    FE::reset_buffer_display();
    determine_move_time(my_time as f64, my_incr as f64,
                        disks_played + 4 as i32);
    if get_ponder_move() == 0 { clear_ponder_times(); }
    remove_coeffs(disks_played);
    /* No feasible moves? */
    if move_count[disks_played as usize] == 0 as i32 {
        *eval_info =
            create_eval_info(PASS_EVAL, UNSOLVED_POSITION,
                             0.0f64 as i32, 0.0f64, 0 as i32,
                             0 as i32);
        set_current_eval(*eval_info);
        if echo != 0 {
            let info = &*eval_info;
            Out::echo_compute_move_1(info);
        }
        if let Some(logger) = logger {
            L::log_best_move_pass(logger);
        }
        last_time_used = 0.0f64;
        clear_pv();
        return -(1 as i32)
    }
    /* If there is only one move available:
       Don't waste any time, unless told so or very close to the end,
       searching the position. */
    if empties > 60 as i32 &&
        move_count[disks_played as usize] == 1 as i32 &&
        search_forced == 0 {
        /* Forced move */
        *eval_info =
            create_eval_info(FORCED_EVAL, UNSOLVED_POSITION,
                             0.0f64 as i32, 0.0f64, 0 as i32,
                             0 as i32);
        set_current_eval(*eval_info);
        if echo != 0 {
            let info = &*eval_info;
            let disk = move_list[disks_played as usize][0];
            Out::echo_compute_move_2(info, disk);
        }
        if let Some(logger) = logger {
            let best_move = move_list[disks_played as usize][0];
            L::log_best_move(logger, best_move);
        }
        last_time_used = 0.0f64;
        return move_list[disks_played as usize][0]
    }
    /* Mark the search as interrupted until a successful search
       has been performed. */
    let mut move_type = INTERRUPTED_MOVE;
    let mut interrupted_depth = 0;
    let mut curr_move = move_list[disks_played as usize][0];
    /* Check the opening book for midgame moves */
    let mut book_move_found = 0;
    let mut midgame_move = -(1 as i32);
    if !forced_opening.is_null() {
        /* Check if the position fits the currently forced opening */
        curr_move = check_forced_opening::<FE>(side_to_move, forced_opening);
        if curr_move != -(1 as i32) {
            book_eval_info =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 0 as i32, 0.0f64, 0 as i32,
                                 1 as i32);
            midgame_move = curr_move;
            book_move_found = 1;
            move_type = BOOK_MOVE;
            if echo != 0 {
                let ponder_move = get_ponder_move();
                Out::echo_ponder_move(curr_move, ponder_move);
            }
            clear_pv();
            pv_depth[0] = 1;
            pv[0][0] =
                curr_move
        }
    }
    if book_move_found == 0 && play_thor_match_openings != 0 {
        /* Optionally use the Thor database as opening book. */
        let threshold = 2;
        Thor::database_search(&board, side_to_move);
        if Thor::get_match_count() >= threshold {
            let game_index =
                ((my_random() >> 8 as i32) %
                    Thor::get_match_count() as i64) as i32;
            curr_move = Thor::get_thor_game_move(game_index, disks_played);
            if valid_move(curr_move, side_to_move) != 0 {
                book_eval_info =
                    create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                     0 as i32, 0.0f64,
                                     0 as i32, 1 as i32);
                midgame_move = curr_move;
                book_move_found = 1;
                move_type = BOOK_MOVE;
                if echo != 0 {
                    let ponder_move = get_ponder_move();
                    Out::echo_ponder_move_2(curr_move, ponder_move);
                }
                clear_pv();
                pv_depth[0] = 1;
                pv[0][0] =
                    curr_move
            } else {
                FE::invalid_move(curr_move);
            }
        }
    }
    if book_move_found == 0 && play_human_openings != 0 && book != 0 {
        /* Check Thor statistics for a move */
        curr_move =
            Thor::choose_thor_opening_move(&board, side_to_move,
                                     0 as i32);
        if curr_move != -(1 as i32) {
            book_eval_info =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 0 as i32, 0.0f64, 0 as i32,
                                 1 as i32);
            midgame_move = curr_move;
            book_move_found = 1;
            move_type = BOOK_MOVE;
            if echo != 0 {
                let ponder_move = get_ponder_move();
                Out::echo_ponder_move_4(curr_move, ponder_move);
            }
            clear_pv();
            pv_depth[0] = 1;
            pv[0][0] =
                curr_move
        }
    }
    if book_move_found == 0 && book != 0 {
        /* Check ordinary opening book */
        let mut flags = 0;
        if empties <= 30 as i32 {
            if empties <= wld { flags = 4 as i32 }
            if empties <= exact { flags = 16 as i32 }
        }
        fill_move_alternatives::<FE>(side_to_move, flags);
        curr_move =
             get_book_move::<FE>(side_to_move, update_all, &mut book_eval_info);
        if curr_move != -(1 as i32) {
            set_current_eval(book_eval_info);
            midgame_move = curr_move;
            book_move_found = 1;
            move_type = BOOK_MOVE;
            Out::display_status_out();
        }
    }
    /* Use iterative deepening in the midgame searches until the endgame
       is reached. If an endgame search already has been performed,
       make a much more shallow midgame search. Also perform much more
       shallow searches when there is no time limit and hence no danger
       starting to solve only to get interrupted. */
    if timed_depth == 0 && empties <= (if exact > wld { exact } else { wld })
    {
        mid =
            if (if (if mid < empties - 7 as i32 {
                mid
            } else { (empties) - 7 as i32 }) <
                28 as i32 {
                if mid < empties - 7 as i32 {
                    mid
                } else { (empties) - 7 as i32 }
            } else { 28 as i32 }) > 2 as i32 {
                if (if mid < empties - 7 as i32 {
                    mid
                } else { (empties) - 7 as i32 }) <
                    28 as i32 {
                    if mid < empties - 7 as i32 {
                        mid
                    } else { (empties) - 7 as i32 }
                } else { 28 as i32 }
            } else { 2 as i32 }
    }
    endgame_reached =
        (timed_depth == 0 && endgame_performed[side_to_move as usize] != 0) as
            i32;
    if book_move_found == 0 && endgame_reached == 0 {
        clear_panic_abort();
        clear_midgame_abort();
        toggle_midgame_abort_check(update_all);
        toggle_midgame_hash_usage(1 as i32, 1 as i32);
        if timed_depth != 0 {
            max_depth = 64 as i32
        } else if empties <= (if exact > wld { exact } else { wld }) {
            max_depth =
                if (if (if mid < empties - 12 as i32 {
                    mid
                } else { (empties) - 12 as i32 }) <
                    18 as i32 {
                    if mid < empties - 12 as i32 {
                        mid
                    } else { (empties) - 12 as i32 }
                } else { 18 as i32 }) > 2 as i32 {
                    if (if mid < empties - 12 as i32 {
                        mid
                    } else { (empties) - 12 as i32 }) <
                        18 as i32 {
                        if mid < empties - 12 as i32 {
                            mid
                        } else { (empties) - 12 as i32 }
                    } else { 18 as i32 }
                } else { 2 as i32 }
        } else { max_depth = mid }
        midgame_depth =
            if (2 as i32) < max_depth {
                2 as i32
            } else { max_depth };
        loop  {
            max_depth_reached = midgame_depth;
            midgame_move =
                middle_game::<FE>(side_to_move, midgame_depth, update_all,
                            &mut mid_eval_info);
            set_current_eval(mid_eval_info);
            midgame_diff =
                1.3f64 * mid_eval_info.score as f64 / 128.0f64;
            if side_to_move == 0 as i32 {
                midgame_diff -= komi as f64
            } else { midgame_diff += komi as f64 }
            if timed_depth != 0 {
                /* Check if the endgame zone has been reached */
                offset = 7;
                /* These constants were chosen rather arbitrarily but intend
                   to make Zebra solve earlier if the position is lopsided. */
                if is_panic_abort() != 0 { offset -= 1 }
                if endgame_performed[side_to_move as usize] != 0 {
                    offset += 2 as i32
                }
                if midgame_depth + offset + 27 as i32 >=
                    2 as i32 * empties ||
                    midgame_depth + 7 as i32 >= empties {
                    endgame_reached = 1 as i32
                }
            }
            midgame_depth += 1;
            if !(is_panic_abort() == 0 && is_midgame_abort() == 0 &&
                force_return == 0 && midgame_depth <= max_depth &&
                midgame_depth + disks_played <= 61 as i32 &&
                endgame_reached == 0) {
                break ;
            }
        }
        if echo != 0 { Out::display_status_out(); }
        if abs(mid_eval_info.score) == abs(-(27000 as i32)) {
            move_type = INTERRUPTED_MOVE;
            interrupted_depth = midgame_depth - 1 as i32
            /* compensate for increment */
        } else { move_type = MIDGAME_MOVE }
    }
    let mut curr_move = midgame_move;
    /* If the endgame has been reached, solve the position */
    if force_return == 0 {
        if timed_depth != 0 && endgame_reached != 0 ||
            timed_depth != 0 && book_move_found != 0 &&
                disks_played >= 60 as i32 - 30 as i32 ||
            timed_depth == 0 &&
                empties <= (if exact > wld { exact } else { wld }) {
            max_depth_reached = empties;
            clear_panic_abort();
            if timed_depth != 0 {
                curr_move =
                   end_game::<FE>(side_to_move,
                             (disks_played < 60 as i32 - exact) as
                                 i32, 0 as i32, book, komi,
                             &mut end_eval_info)
            } else if empties <= exact {
                curr_move =
                   end_game::<FE>(side_to_move, 0 as i32, 0 as i32,
                             book, komi, &mut end_eval_info)
            } else {
                curr_move =
                   end_game::<FE>(side_to_move, 1 as i32, 0 as i32,
                             book, komi, &mut end_eval_info)
            }
            set_current_eval(end_eval_info);
            if abs(root_eval) == abs(-(27000 as i32)) {
                move_type = INTERRUPTED_MOVE
            } else { move_type = ENDGAME_MOVE }
            if update_all != 0 {
                endgame_performed[side_to_move as usize] = 1 as i32
            }
        }
    }
    match move_type as u32 {
        0 => {
            *eval_info =
                create_eval_info(INTERRUPTED_EVAL, UNSOLVED_POSITION,
                                 0.0f64 as i32, 0.0f64,
                                 0 as i32, 0 as i32);
            let info = &*eval_info;
            let counter_value = counter_value(&mut nodes);
            let elapsed_time = get_elapsed_time::<FE>();
            Out::send_move_type_0_status(interrupted_depth, info, counter_value, elapsed_time);
        }
        1 => { *eval_info = book_eval_info }
        2 => { *eval_info = mid_eval_info }
        3 => { *eval_info = end_eval_info }
        _ => { }
    }
    set_current_eval(*eval_info);
    last_time_used = get_elapsed_time::<FE>();
    if update_all != 0 {
        total_time += last_time_used;
        add_counter(&mut total_evaluations, &mut evaluations);
        add_counter(&mut total_nodes, &mut nodes);
    }
    clear_panic_abort();
    /* Write the contents of the status buffer to the log file. */
    if move_type as u32 == BOOK_MOVE as i32 as u32 {
        if let Some(logger) = logger {
            let info = &*eval_info;
            L::log_chosen_move(logger, curr_move, info);
        }
    } else if let Some(logger) = logger {
        L::log_status(logger);
    }
    /* Write the principal variation, if available, to the log file
       and, optionally, to screen. */
    if get_ponder_move() == 0 {
        complete_pv::<FE>(side_to_move);
        if display_pv != 0 && echo != 0 { Out::display_out_optimal_line(); }
        if let Some(logger) = logger { L::log_optimal_line(logger); }
    }
    if let Some(logger) = logger {
        L::close_logger(logger);
    }
    return curr_move;
}

/*
   COMPUTE_MOVE
   Returns the best move in a position given search parameters.
*/
pub unsafe fn compute_move<L: ComputeMoveLogger, Out: ComputeMoveOutput, FE: FrontEnd, Thor: ThorDatabase>(
    side_to_move: i32,
    update_all: i32,
    my_time: i32,
    my_incr: i32,
    timed_depth: i32,
    book: i32,
    mid: i32,
    exact: i32,
    wld: i32,
    search_forced: i32,
    eval_info: &mut EvaluationType)
    -> i32 {
    return generic_compute_move::<L, Out, FE, Thor>(
        side_to_move, update_all, my_time,
        my_incr, timed_depth,
        book, mid,
        exact, wld,
        search_forced, eval_info, &mut L::create_log_file_if_needed());
}

pub unsafe fn ponder_move<
    L: ComputeMoveLogger,
    Out: ComputeMoveOutput,
    FE: FrontEnd,
    Thor: ThorDatabase,
    Rep: PonderMoveReport>(side_to_move: i32,
                           _book: i32,
                           mid: i32,
                           exact: i32,
                           wld: i32) {
    let mut eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
            res: WON_POSITION,
            score: 0,
            confidence: 0.,
            search_depth: 0,
            is_book: 0,};
    let mut entry =
        HashEntry{key1: 0,
            key2: 0,
            eval: 0,
            move_0: [0; 4],
            draft: 0,
            selectivity: 0,
            flags: 0,};
    let mut move_start_time: f64 = 0.;
    let mut move_stop_time: f64 = 0.;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut this_move: i32 = 0;
    let mut expect_count: i32 = 0;
    let mut expect_list: [i32; 64] = [0; 64];
    let mut best_pv: [i32; 61] = [0; 61];
    /* Disable all time control mechanisms as it's the opponent's
       time we're using */
    toggle_abort_check(0 as i32);
    toggle_midgame_abort_check(0 as i32);
    start_move::<FE>(0 as i32 as f64,
                     0 as i32 as f64,
                     disc_count(0 as i32, &board) + disc_count(2 as i32, &board));
    clear_ponder_times();
    determine_hash_values(side_to_move, &board);
    reset_counter(&mut nodes);
    /* Find the scores for the moves available to the opponent. */
    let mut hash_move = 0;
    find_hash(&mut entry, 1 as i32);
    if entry.draft as i32 != 0 as i32 {
        hash_move = entry.move_0[0]
    } else {
        find_hash(&mut entry, 0 as i32);
        if entry.draft as i32 != 0 as i32 {
            hash_move = entry.move_0[0]
        }
    }
    let stored_echo = echo;
    echo = 0;
    compute_move::<L, Out, FE, Thor>(side_to_move, 0 as i32, 0 as i32,
                 0 as i32, 0 as i32, 0 as i32,
                 if (8 as i32) < mid {
                     8 as i32
                 } else { mid }, 0 as i32, 0 as i32,
                 0 as i32, &mut eval_info);
    echo = stored_echo;
    /* Sort the opponents on the score and push the table move (if any)
       to the front of the list */
    if force_return != 0 {
        expect_count = 0 as i32
    } else {
        sort_moves(move_count[disks_played as usize]);
        float_move(hash_move, move_count[disks_played as usize]);
        expect_count = move_count[disks_played as usize];
        i = 0;
        while i < expect_count {
            expect_list[i as usize] =
                move_list[disks_played as usize][i as usize];
            i += 1
        }
        Rep::report_hash_move(hash_move);
        let move_list_item = &move_list[disks_played as usize];
        let evals_item = &evals[disks_played as usize];
        Rep::report_move_evals(expect_count, move_list_item, evals_item);
    }
    /* Go through the expected moves in order and prepare responses. */
    let mut best_pv_depth = 0;
    let mut i = 0;
    while force_return == 0 && i < expect_count {
        move_start_time = get_real_timer::<FE>();
        set_ponder_move(expect_list[i as usize]);
        this_move = expect_list[i as usize];
        prefix_move = this_move;
        make_move(side_to_move, this_move, 1 as i32);
        compute_move::<L, Out, FE, Thor>(0 as i32 + 2 as i32 - side_to_move,
                                         0 as i32, 0 as i32, 0 as i32,
                     1 as i32, 0 as i32, mid, exact, wld,
                     0 as i32, &mut eval_info);
        unmake_move(side_to_move, this_move);
        clear_ponder_move();
        move_stop_time = get_real_timer::<FE>();
        add_ponder_time(expect_list[i as usize],
                        move_stop_time - move_start_time);
        ponder_depth[expect_list[i as usize] as usize] =
            if ponder_depth[expect_list[i as usize] as usize] >
                max_depth_reached - 1 as i32 {
                ponder_depth[expect_list[i as usize] as usize]
            } else { (max_depth_reached) - 1 as i32 };
        if i == 0 as i32 && force_return == 0 {
            /* Store the PV for the first move */
            best_pv_depth = pv_depth[0];
            j = 0;
            while j < pv_depth[0] {
                best_pv[j as usize] =
                    pv[0][j as usize];
                j += 1
            }
        }
        i += 1
    }
    /* Make sure the PV looks reasonable when leaving - either by
       clearing it altogether or, preferrably, using the stored PV for
       the first move if it is available. */
    max_depth_reached += 1;
    prefix_move = 0;
    if best_pv_depth == 0 as i32 {
        pv_depth[0] = 0 as i32
    } else {
        pv_depth[0] =
            best_pv_depth + 1 as i32;
        pv[0][0] =
            expect_list[0];
        i = 0;
        while i < best_pv_depth {
            pv[0][(i + 1 as i32) as usize] =
                best_pv[i as usize];
            i += 1
        }
    }
    /* Don't forget to enable the time control mechanisms when leaving */
    toggle_abort_check(1 as i32);
    toggle_midgame_abort_check(1 as i32);
}
pub trait PonderMoveReport {
    fn report_move_evals(expect_count: i32, move_list_item: &[i32; 64], evals_item: &[i32; 128]);
    fn report_hash_move(hash_move: i32);
}

pub trait ComputeMoveOutput {
    fn display_out_optimal_line();
    fn send_move_type_0_status(interrupted_depth: i32, info: &EvaluationType, counter_value: f64, elapsed_time: f64);
    fn display_status_out();
    fn echo_ponder_move_4(curr_move: i32, ponder_move: i32);
    fn echo_ponder_move_2(curr_move: i32, ponder_move: i32);
    fn echo_ponder_move(curr_move: i32, ponder_move: i32);
    fn echo_compute_move_2(info: &EvaluationType, disk: i32);
    fn echo_compute_move_1(info: &EvaluationType);
}
pub trait ComputeMoveLogger {
    fn log_moves_generated(logger: &mut Self, moves_generated: i32, move_list_for_disks_played: &[i32; 64]);
    fn log_best_move_pass(logger: &mut Self);
    fn log_best_move(logger: &mut Self, best_move: i32);
    fn log_chosen_move(logger: &mut Self, curr_move: i32, info: &EvaluationType);
    fn log_status(logger: &mut Self);
    fn log_optimal_line(logger: &mut Self);
    fn close_logger(logger: &mut Self);
    fn log_board(logger: &mut Self, board_: &[i32; 128], side_to_move_: i32);
    fn create(log_file_path_: &mut [i8]) -> Option<Self> where Self:Sized;
    fn create_log_file_if_needed() -> Option<Self> where Self:Sized;
}