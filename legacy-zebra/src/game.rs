use std::ffi::{CStr, CString};

use engine::src::counter::{adjust_counter, counter_value, reset_counter};
use engine::src::error::FrontEnd;
use engine::src::game::{BoardSource, CandidateMove, compare_eval, ComputeMoveLogger, ComputeMoveOutput, engine_global_setup, EvaluatedMove, FileBoardSource, generic_compute_move, generic_game_init};
use engine::src::getcoeff::pattern_evaluation;
use engine::src::hash::{determine_hash_values, find_hash, HashEntry};
use engine::src::moves::{generate_all, make_move, unmake_move};
use engine::src::osfbook::{fill_move_alternatives, get_book_move};
use engine::src::search::{create_eval_info, disc_count, float_move, force_return, sort_moves, SearchState};
use engine::src::stubs::abs;
use engine::src::thordb::ThorDatabase;
use engine::src::zebra::EvalResult::{DRAWN_POSITION, LOST_POSITION, UNSOLVED_POSITION, WON_POSITION};
use engine::src::zebra::EvalType::{EXACT_EVAL, MIDGAME_EVAL, PASS_EVAL, UNDEFINED_EVAL, WLD_EVAL};
use engine::src::zebra::EvaluationType;
use libc_wrapper::{ctime, fclose, FileHandle, fopen, fprintf, printf, puts, stdout, strcpy, time, time_t};

use crate::src::display::{clear_status, display_board, display_optimal_line, display_status, produce_eval_text, send_status_nodes, send_status_pv, send_status_time, display_state, send_status_1, send_status_2, send_status_0};
use crate::src::error::{FE, LibcFatalError};
use crate::src::getcoeff::{load_coeff_adjustments, new_z_lib_source};
use crate::src::thordb::LegacyThor;
use crate::src::zebra::FullState;
use engine::src::globals::BoardState;
use std::fs::{File, read, OpenOptions};
use std::io::{Read, BufReader, BufRead, Write};
use std::iter::FromIterator;
use engine::src::timer::Timer;

pub static mut log_file_path: [i8; 2048] = [0; 2048];
pub static mut prefix_move: i32 = 0;
pub static mut evaluated_list: [EvaluatedMove; 60] = [EvaluatedMove {
    eval: EvaluationType {
        type_0: MIDGAME_EVAL,
        res: WON_POSITION,
        score: 0,
        confidence: 0.,
        search_depth: 0,
        is_book: 0,
    },
    side_to_move: 0,
    move_0: 0,
    pv_depth: 0,
    pv: [0; 60],
}; 60];

pub unsafe fn get_evaluated(index: i32)
                            -> EvaluatedMove {
    return evaluated_list[index as usize];
}
pub static mut game_evaluated_count: i32 = 0;

/*
  GET_EVALUATED_COUNT
  GET_EVALUATED
  Accessor functions for the data structure filled by extended_compute_move().
*/

pub unsafe fn get_evaluated_count() -> i32 {
    return game_evaluated_count;
}

pub static mut use_log_file: i32 = 1;
/*
  TOGGLE_STATUS_LOG
  Enable/disable the use of logging all the output that the
  text version of Zebra would output to the screen.
*/

pub unsafe fn toggle_status_log(write_log: i32) {
    use_log_file = write_log;
}

/*
   GLOBAL_SETUP
   Initialize the different sub-systems.
*/

pub unsafe fn global_setup(use_random: i32,
                                      hash_bits: i32, mut g_state: &mut FullState) {
    LogFileHandler::on_global_setup();
    let coeff_adjustments = load_coeff_adjustments();

    let file_name = if let Ok(var) = std::env::var("COEFFS_PATH") {
        CString::new(var).unwrap()
    } else {
        CString::new("./coeffs2.bin").unwrap()
    };
    let FullState {
        ref mut g_config,
        ref mut learn_state,
        ref mut midgame_state,
        ref mut game_state,
        ref mut end_g,
        ref mut coeff_state,
        ref mut g_timer,
        ref mut moves_state,
        ref mut stable_state,
        ref mut board_state,
        ref mut hash_state,
        ref mut random_instance,
        ref mut g_book,
        ref mut prob_cut,
        ref mut search_state,
        ref mut flip_stack_,
    } : &mut FullState = g_state;
    engine_global_setup::<_,LibcFatalError>(use_random, hash_bits, coeff_adjustments, new_z_lib_source(file_name.as_ref()),
                                            search_state
                                            , hash_state
                                            , g_timer
                                            , coeff_state
                                            , random_instance
                                            , stable_state
                                            , prob_cut);
}

pub struct LogFileHandler {
    log_file: FileHandle,
}
impl LogFileHandler {
    fn on_global_setup() {
        unsafe { setup_log_file() }
    }
}
unsafe fn setup_log_file() {
    /* Clear the log file. No error handling done. */
    strcpy(log_file_path.as_mut_ptr(),
           b"zebra.log\x00" as *const u8 as *const i8);
    if use_log_file != 0 {
        let log_file =
            fopen(log_file_path.as_mut_ptr(),
                  b"w\x00" as *const u8 as *const i8);
        if !log_file.is_null() {
            let mut timer_: time_t = 0;
            time(&mut timer_);
            fprintf(log_file,
                    b"%s %s\n\x00" as *const u8 as *const i8,
                    b"Log file created\x00" as *const u8 as
                        *const i8, ctime(&mut timer_));
            fprintf(log_file,
                    b"%s %s %s\n\x00" as *const u8 as *const i8,
                    b"Engine compiled\x00" as *const u8 as
                        *const i8,
                    b"Jul  2 2020\x00" as *const u8 as *const i8,
                    b"19:33:59\x00" as *const u8 as *const i8);
            fclose(log_file);
        }
    }
}

pub struct BasicBoardFileSource {
    stream: BufReader<File>
}
impl FileBoardSource for BasicBoardFileSource {
    fn open(file_name: &CStr) -> Option<BasicBoardFileSource> {
        Some(BasicBoardFileSource {
            stream: BufReader::new(File::open(file_name.to_str().ok()?).ok()?)
        })
    }
}
impl BoardSource for BasicBoardFileSource {
    // These methods and this whole scheme of loading the data honestly doesn't make any sense
    // but I don't want to refactor it for the better at the moment.
    fn fill_board_buffer(&mut self, buffer: &mut String) {
        self.stream.read_line(buffer);
    }

    fn fill_buffer_with_side_to_move(&mut self, buffer: &mut Vec<u8>) {
        self.stream.read(&mut buffer.as_mut_slice()[0..10]);
    }

    fn report_unrecognized_character(unrecognized: i8) {
        unsafe {
            printf(b"%s \'%c\' %s\n\x00" as *const u8 as
                       *const i8,
                   b"Unrecognized character\x00" as *const u8 as
                       *const i8,
                   unrecognized as i32,
                   b"in game file\x00" as *const u8 as
                       *const i8);
        }
    }
}
/*
   GAME_INIT
   Prepare the relevant data structures so that a game
   can be played. The position is read from the file
   specified by FILE_NAME.
*/

pub unsafe fn game_init(file_name: *const i8, side_to_move: &mut i32, g_state: &mut FullState) {
    let file_name = (!file_name.is_null()).then(|| CStr::from_ptr(file_name));
    let FullState {
        ref mut g_config,
        ref mut learn_state,
        ref mut midgame_state,
        ref mut game_state,
        ref mut end_g,
        ref mut coeff_state,
        ref mut g_timer,
        ref mut moves_state,
        ref mut stable_state,
        ref mut board_state,
        ref mut hash_state,
        ref mut random_instance,
        ref mut g_book,
        ref mut prob_cut,
        ref mut search_state,
        ref mut flip_stack_,
    } : &mut FullState = g_state;
    generic_game_init::<BasicBoardFileSource, LibcFatalError>(file_name, side_to_move,
                                                              flip_stack_,
                                                              search_state,
                                                              board_state,
                                                              hash_state,
                                                              g_timer,
                                                              end_g,
                                                              midgame_state,
                                                              coeff_state,
                                                              moves_state,
                                                              random_instance,
                                                              g_book,
                                                              stable_state,
                                                              game_state);
}
/*
  PONDER_MOVE
  Perform searches in response to the opponent's next move.
  The results are not returned, but the hash table is filled
  with useful scores and moves.
*/

pub struct LibcPonderMoveReport;
impl LibcFatalError {
    fn report_move_evals(expect_count: i32, move_list_item: &[i32; 64], evals_item: &[i32; 128]) {
        let mut i = 0;
        while i < expect_count {
            let move__ = move_list_item[i as usize];
            let move_eval = evals_item[move__ as usize];
            unsafe {
                printf(b"%c%c %-6.2f  \x00" as *const u8 as *const i8,
                       'a' as i32 + move__ % 10 as i32 - 1 as i32,
                       '0' as i32 + move__ / 10 as i32, move_eval as f64 / 128.0f64);
            }
            if i % 7 as i32 == 6 as i32 || i == expect_count - 1 as i32 {
                unsafe { write!(stdout, "\n"); }
            }
            i += 1
        }
    }

    fn report_hash_move(hash_move: i32) {
        unsafe {
            printf(b"%s=%d\n\x00" as *const u8 as *const i8,
                   b"hash move\x00" as *const u8 as *const i8, hash_move);
        }
    }
}

pub unsafe fn ponder_move<
    L: ComputeMoveLogger,
    Out: ComputeMoveOutput,
    FE: FrontEnd,
    Thor: ThorDatabase>(side_to_move: i32,
                           _book: i32,
                           mid: i32,
                           exact: i32,
                           wld: i32, display_pv: i32, mut echo:i32, g_state: &mut FullState) {
    type Rep = LibcFatalError;
    let FullState {
        ref mut g_config,
        ref mut learn_state,
        ref mut midgame_state,
        ref mut game_state,
        ref mut end_g,
        ref mut coeff_state,
        ref mut g_timer,
        ref mut moves_state,
        ref mut stable_state,
        ref mut board_state,
        ref mut hash_state,
        ref mut random_instance,
        ref mut g_book,
        ref mut prob_cut,
        ref mut search_state,
        ref mut flip_stack_,
    } : &mut FullState = g_state;
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
    g_timer.toggle_abort_check(0 as i32);
    midgame_state.toggle_midgame_abort_check(0 as i32);
     g_timer.start_move(0 as i32 as f64,
                     0 as i32 as f64,
                     disc_count(0 as i32, &board_state.board) + disc_count(2 as i32, &board_state.board));
     g_timer.clear_ponder_times();
    determine_hash_values(side_to_move, &board_state.board,  hash_state);
    reset_counter(&mut search_state.nodes);
    /* Find the scores for the moves available to the opponent. */
    let mut hash_move = 0;
    find_hash(&mut entry, 1 as i32, hash_state);
    if entry.draft as i32 != 0 as i32 {
        hash_move = entry.move_0[0]
    } else {
        find_hash(&mut entry, 0 as i32, hash_state);
        if entry.draft as i32 != 0 as i32 {
            hash_move = entry.move_0[0]
        }
    }
    let stored_echo = echo;
    echo = 0;
    engine::src::game::compute_move::<L, Out, FE, Thor>(side_to_move, 0 as i32, 0 as i32,
                                     0 as i32, 0 as i32, 0 as i32,
                                     if (8 as i32) < mid {
                                         8 as i32
                                     } else { mid }, 0 as i32, 0 as i32,
                                     0 as i32, &mut eval_info, display_pv, echo,   flip_stack_,
                                                         search_state,
                                                         board_state,
                                                         hash_state,
                                                         g_timer,
                                                         end_g,
                                                         midgame_state,
                                                         coeff_state,
                                                         moves_state,
                                                         random_instance,
                                                         g_book,
                                                         stable_state,
                                                         game_state, prob_cut);
    echo = stored_echo;
    /* Sort the opponents on the score and push the table move (if any)
       to the front of the list */
    if force_return != 0 {
        expect_count = 0 as i32
    } else {
        sort_moves(moves_state.move_count[moves_state.disks_played as usize],  moves_state, &search_state);
        float_move(hash_move, moves_state.move_count[moves_state.disks_played as usize], moves_state);
        expect_count = moves_state.move_count[moves_state.disks_played as usize];
        i = 0;
        while i < expect_count {
            expect_list[i as usize] =
                moves_state.move_list[moves_state.disks_played as usize][i as usize];
            i += 1
        }
        Rep::report_hash_move(hash_move);
        let move_list_item = &moves_state.move_list[moves_state.disks_played as usize];
        let evals_item = &search_state.evals[moves_state.disks_played as usize];
        Rep::report_move_evals(expect_count, move_list_item, evals_item);
    }
    /* Go through the expected moves in order and prepare responses. */
    let mut best_pv_depth = 0;
    let mut i = 0;
    while force_return == 0 && i < expect_count {
        move_start_time =  g_timer.get_real_timer();
        let move_0 = expect_list[i as usize];
        search_state.set_ponder_move(move_0);
        this_move = expect_list[i as usize];
        prefix_move = this_move;
        make_move(side_to_move, this_move, 1 as i32 , moves_state,  board_state,  hash_state,  flip_stack_ );
        engine::src::game::compute_move::<L, Out, FE, Thor>(0 as i32 + 2 as i32 - side_to_move,
                                         0 as i32, 0 as i32, 0 as i32,
                                         1 as i32, 0 as i32, mid, exact, wld,
                                         0 as i32, &mut eval_info, display_pv, echo,    flip_stack_,
                                                             search_state,
                                                             board_state,
                                                             hash_state,
                                                             g_timer,
                                                             end_g,
                                                             midgame_state,
                                                             coeff_state,
                                                             moves_state,
                                                             random_instance,
                                                             g_book,
                                                             stable_state,
                                                             game_state,  prob_cut);
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board,  moves_state,  hash_state,  flip_stack_);
        };
        search_state.clear_ponder_move();
        move_stop_time =  g_timer.get_real_timer();
        let move_0 = expect_list[i as usize];
        let time_0 = move_stop_time - move_start_time;
        g_timer.add_ponder_time(move_0, time_0);
        g_timer.ponder_depth[expect_list[i as usize] as usize] =
            if g_timer.ponder_depth[expect_list[i as usize] as usize] >
                game_state.max_depth_reached - 1 as i32 {
                g_timer.ponder_depth[expect_list[i as usize] as usize]
            } else { (game_state.max_depth_reached) - 1 as i32 };
        if i == 0 as i32 && force_return == 0 {
            /* Store the PV for the first move */
            best_pv_depth = board_state.pv_depth[0];
            j = 0;
            while j < board_state.pv_depth[0] {
                best_pv[j as usize] =
                    board_state.pv[0][j as usize];
                j += 1
            }
        }
        i += 1
    }
    /* Make sure the PV looks reasonable when leaving - either by
       clearing it altogether or, preferrably, using the stored PV for
       the first move if it is available. */
    game_state.max_depth_reached += 1;
    prefix_move = 0;
    if best_pv_depth == 0 as i32 {
        board_state.pv_depth[0] = 0 as i32
    } else {
        board_state.pv_depth[0] =
            best_pv_depth + 1 as i32;
        board_state.pv[0][0] =
            expect_list[0];
        i = 0;
        while i < best_pv_depth {
            board_state.pv[0][(i + 1 as i32) as usize] =
                best_pv[i as usize];
            i += 1
        }
    }
    /* Don't forget to enable the time control mechanisms when leaving */
    g_timer.toggle_abort_check(1 as i32);
    midgame_state.toggle_midgame_abort_check(1 as i32);
}
/*
  GET_SEARCH_STATISTICS
  Returns some statistics about the last search made.
*/

pub unsafe fn get_search_statistics(max_depth: &mut i32, node_count: &mut f64, g_state: &mut FullState) {
    let FullState {
        ref mut game_state,
        ref mut search_state,
        ..
    } : &mut FullState = g_state;
    *max_depth = game_state.max_depth_reached;
    if prefix_move != 0 {
        *max_depth += 1
    }
    adjust_counter(&mut search_state.nodes);
    *node_count = counter_value(&mut search_state.nodes);
}


/*
  GET_PV
  Returns the principal variation.
*/

pub unsafe fn get_pv(destin: &mut [i32], g_state: &mut FullState) -> i32 {
    let FullState {
       ref  mut g_config,
       ref  mut learn_state,
       ref  mut midgame_state,
       ref  mut game_state,
       ref  mut end_g,
       ref  mut coeff_state,
       ref  mut g_timer,
       ref  mut moves_state,
       ref  mut stable_state,
       ref  mut board_state,
       ref  mut hash_state,
       ref  mut random_instance,
       ref  mut g_book,
       ref  mut prob_cut,
       ref  mut search_state,
       ref  mut flip_stack_,
    } : &mut FullState = g_state;
    let mut i = 0;
    return if prefix_move == 0 {
        i = 0;
        while i < board_state.pv_depth[0] {
            destin[i as usize] = board_state.pv[0][i as usize];
            i += 1
        }
        board_state.pv_depth[0]
    } else {
        destin[0] = prefix_move;
        i = 0;
        while i < board_state.pv_depth[0] {
            destin[(i + 1 as i32) as usize] = board_state.pv[0][i as usize];
            i += 1
        }
        board_state.pv_depth[0] + 1
    };
}
/*
  EXTENDED_COMPUTE_MOVE
  This wrapper on top of compute_move() calculates the evaluation
  of all moves available as opposed to upper bounds for all moves
  except for the best.
*/

pub unsafe fn extended_compute_move<FE: FrontEnd>(side_to_move: i32,
                                                  book_only: i32,
                                                  mut book: i32,
                                                  mut mid: i32,
                                                  mut exact: i32,
                                                  mut wld: i32, mut echo: i32, g_state: &mut FullState)
                                                  -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut index: i32 = 0;
    let mut changed: i32 = 0;
    let mut this_move: i32 = 0;
    let mut disc_diff: i32 = 0;
    let mut corrected_diff: i32 = 0;
    let mut best_move: i32 = 0;
    let mut temp_move: i32 = 0;
    let mut best_score: i32 = 0;
    let mut best_pv_depth: i32 = 0;
    let mut stored_echo: i32 = 0;
    let mut shallow_eval: i32 = 0;
    let mut empties: i32 = 0;
    let mut current_mid: i32 = 0;
    let mut current_exact: i32 = 0;
    let mut current_wld: i32 = 0;
    let mut first_iteration: i32 = 0;
    let mut unsearched: i32 = 0;
    let mut unsearched_count: i32 = 0;
    let mut unsearched_move: [i32; 61] = [0; 61];
    let mut best_pv: [i32; 60] = [0; 60];
    let mut transform1: [u32; 60] = [0; 60];
    let mut transform2: [u32; 60] = [0; 60];
    let mut book_move =
        CandidateMove{move_0: 0, score: 0, flags: 0, parent_flags: 0,};
    let mut temp =
        EvaluatedMove{eval:
                          EvaluationType{type_0: MIDGAME_EVAL,
                                         res: WON_POSITION,
                                         score: 0,
                                         confidence: 0.,
                                         search_depth: 0,
                                         is_book: 0,},
                      side_to_move: 0,
                      move_0: 0,
                      pv_depth: 0,
                      pv: [0; 60],};
    let mut book_eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut res = WON_POSITION;
    /* Disable all time control mechanisms and randomization */
    (g_state.g_timer).toggle_abort_check(0 as i32);
    (g_state.midgame_state).toggle_midgame_abort_check(0 as i32);
    (g_state.midgame_state).toggle_perturbation_usage(0 as i32);
     (g_state.g_timer).start_move(0 as i32 as f64,
                                        0 as i32 as f64,
                                        disc_count(0 as i32, &(g_state.board_state).board) + disc_count(2 as i32, &(g_state.board_state).board));
    (g_state.g_timer).clear_ponder_times();
    determine_hash_values(side_to_move, &(g_state.board_state).board, (&mut g_state.hash_state));
    empties = 60 as i32 - (g_state.moves_state).disks_played;
    best_move = 0;
    game_evaluated_count = 0;
    reset_counter(&mut (g_state.search_state).nodes);
    generate_all(side_to_move, (&mut g_state.moves_state), &(g_state.search_state), &(g_state.board_state).board);
    if book_only != 0 || book != 0 {
        /* Evaluations for database moves */
        let mut flags = 0;
        if empties <= exact {
            flags = 16 as i32
        } else if empties <= wld { flags = 4 as i32 }
        fill_move_alternatives::<FE>(side_to_move, flags,
                                     (&mut g_state.g_book),
                                     &mut (g_state.board_state),
                                     &mut (g_state.moves_state),
                                     &mut (g_state.search_state),
                                     &mut (g_state.flip_stack_),
                                     &mut (g_state.hash_state));
        game_evaluated_count = (g_state.g_book).get_candidate_count();
        i = 0;
        while i < game_evaluated_count {
            let mut child_flags: i32 = 0;
            book_move = (g_state.g_book).get_candidate(i);
            evaluated_list[i as usize].side_to_move = side_to_move;
            evaluated_list[i as usize].move_0 = book_move.move_0;
            evaluated_list[i as usize].pv_depth = 1;
            evaluated_list[i as usize].pv[0] =
                book_move.move_0;
            evaluated_list[i as usize].eval =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 book_move.score, 0.0f64, 0 as i32,
                                 1 as i32);
            child_flags = book_move.flags & book_move.parent_flags;
            if child_flags & (16 as i32 | 4 as i32) != 0 {
                if child_flags & 16 as i32 != 0 {
                    evaluated_list[i as usize].eval.type_0 = EXACT_EVAL
                } else { evaluated_list[i as usize].eval.type_0 = WLD_EVAL }
                if book_move.score > 0 as i32 {
                    evaluated_list[i as usize].eval.res = WON_POSITION;
                    /* Normalize the scores so that e.g. 33-31 becomes +256 */
                    evaluated_list[i as usize].eval.score -=
                        30000 as i32;
                    evaluated_list[i as usize].eval.score *=
                        128 as i32
                } else if book_move.score == 0 as i32 {
                    evaluated_list[i as usize].eval.res = DRAWN_POSITION
                } else {
                    /* score < 0 */
                    evaluated_list[i as usize].eval.res = LOST_POSITION;
                    /* Normalize the scores so that e.g. 30-34 becomes -512 */
                    evaluated_list[i as usize].eval.score +=
                        30000 as i32;
                    evaluated_list[i as usize].eval.score *=
                        128 as i32
                }
            } else { evaluated_list[i as usize].eval.type_0 = MIDGAME_EVAL }
            i += 1
        }
    }
    if book_only != 0 {
        /* Only book moves are to be considered */
        if game_evaluated_count > 0 as i32 {
            best_move =
                 get_book_move::<FE>(side_to_move, 0 as i32,
                                     &mut book_eval_info, echo,
                                     &mut (g_state.board_state),
                                     &mut (g_state.g_book),
                                     &(g_state.search_state),
                                     &mut (g_state.moves_state),
                                     &mut (g_state.hash_state),
                                     &mut (g_state.random_instance),
                                     &mut (g_state.flip_stack_));
            let eval = book_eval_info;
            (g_state.search_state).set_current_eval(eval);
        } else {
            (g_state.board_state).pv_depth[0] = 0;
            best_move = -(1 as i32);
            book_eval_info =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 0 as i32, 0.0f64, 0 as i32,
                                 0 as i32);
            let eval = book_eval_info;
            (g_state.search_state).set_current_eval(eval);
        }
    } else {
        /* Make searches for moves not in the database */
        let mut shallow_depth: i32 = 0;
        let empties_0 = 60 as i32 - (g_state.moves_state).disks_played;
        book = 0;
        best_score = -(12345678 as i32);
        if game_evaluated_count > 0 as i32 {
            /* Book PV available */
            best_score = evaluated_list[0].eval.score;
            best_move = evaluated_list[0].move_0
        }
        let negate = 1 as i32;
        (g_state.search_state).negate_current_eval(negate);
        /* Store the available moves, clear their evaluations and sort
           them on shallow evaluation. */
        if empties_0 < 12 as i32 {
            shallow_depth = 1 as i32
        } else {
            let max_depth =
                if mid > (if exact > wld { exact } else { wld }) {
                    mid
                } else if exact > wld { exact } else { wld };
            if max_depth >= 16 as i32 {
                shallow_depth = 6 as i32
            } else { shallow_depth = 4 as i32 }
        }
        unsearched_count = 0;
        i = 0;
        while i < (g_state.moves_state).move_count[(g_state.moves_state).disks_played as usize] {
            this_move = (g_state.moves_state).move_list[(g_state.moves_state).disks_played as usize][i as usize];
            unsearched = 1;
            j = 0;
            while j < game_evaluated_count {
                if evaluated_list[j as usize].move_0 == this_move {
                    unsearched = 0 as i32
                }
                j += 1
            }
            if !(unsearched == 0) {
                unsearched_move[unsearched_count as usize] = this_move;
                unsearched_count += 1;
                make_move(side_to_move, this_move, 1 as i32, (&mut g_state.moves_state), (&mut g_state.board_state), (&mut g_state.hash_state), (&mut g_state.flip_stack_));
                let side_to_move_argument = 0 as i32 +
                    2 as i32 -
                    side_to_move;
                if shallow_depth == 1 as i32 {
                    /* Compute move doesn't allow depth 0 */
                    (g_state.search_state).evaluations.lo = (g_state.search_state).evaluations.lo.wrapping_add(1);
                    shallow_eval =
                        -pattern_evaluation(side_to_move_argument, (&mut g_state.board_state), &(g_state.moves_state), (&mut g_state.coeff_state))
                } else {
                    let mut shallow_info =
                        EvaluationType{type_0: MIDGAME_EVAL,
                                       res: WON_POSITION,
                                       score: 0,
                                       confidence: 0.,
                                       search_depth: 0,
                                       is_book: 0,};
                    compute_move(0 as i32 + 2 as i32 -
                                     side_to_move, 0 as i32,
                                 0 as i32, 0 as i32,
                                 0 as i32, book,
                                 shallow_depth - 1 as i32,
                                 0 as i32, 0 as i32,
                                 1 as i32, &mut shallow_info, g_state);
                    if shallow_info.type_0 as u32 ==
                           PASS_EVAL as i32 as u32 {
                        /* Don't allow pass */
                        compute_move(side_to_move, 0 as i32,
                                     0 as i32, 0 as i32,
                                     0 as i32, book,
                                     shallow_depth - 1 as i32,
                                     0 as i32, 0 as i32,
                                     1 as i32, &mut shallow_info, g_state);
                        if shallow_info.type_0 as u32 ==
                               PASS_EVAL as i32 as u32 {
                            /* Game over */
                            disc_diff =
                                disc_count(side_to_move, &(g_state.board_state).board) -
                                    disc_count(0 as i32 +
                                                   2 as i32 -
                                                   side_to_move, &(g_state.board_state).board);
                            if disc_diff > 0 as i32 {
                                corrected_diff =
                                    64 as i32 -
                                        2 as i32 *
                                            disc_count(0 as i32 +
                                                           2 as i32 -
                                                           side_to_move, &(g_state.board_state).board)
                            } else if disc_diff == 0 as i32 {
                                corrected_diff = 0 as i32
                            } else {
                                corrected_diff =
                                    2 as i32 *
                                        disc_count(side_to_move, &(g_state.board_state).board) -
                                        64 as i32
                            }
                            shallow_eval = 128 as i32 * corrected_diff
                        } else { shallow_eval = shallow_info.score }
                    } else {
                        /* Sign-correct the score produced */
                        shallow_eval = -shallow_info.score
                    }
                }
                let move_0 = this_move;
                {
                    unmake_move(side_to_move, move_0, &mut (g_state.board_state).board, (&mut g_state.moves_state), (&mut g_state.hash_state), (&mut g_state.flip_stack_));
                };
                (g_state.search_state).evals[(g_state.moves_state).disks_played as usize][this_move as usize] =
                    shallow_eval
            }
            i += 1
        }
        loop  {
            changed = 0;
            i = 0;
            while i < unsearched_count - 1 as i32 {
                if (g_state.search_state).evals[(g_state.moves_state).disks_played as
                             usize][unsearched_move[i as usize] as usize] <
                       (g_state.search_state).evals[(g_state.moves_state).disks_played as
                                 usize][unsearched_move[(i + 1 as i32)
                                                            as usize] as
                                            usize] {
                    temp_move = unsearched_move[i as usize];
                    unsearched_move[i as usize] =
                        unsearched_move[(i + 1 as i32) as usize];
                    unsearched_move[(i + 1 as i32) as usize] =
                        temp_move;
                    changed = 1 as i32
                }
                i += 1
            }
            if !(changed != 0) { break ; }
        }
        /* Initialize the entire list as being empty */
        i = 0;
        index = game_evaluated_count;
        while i < unsearched_count {
            evaluated_list[index as usize].side_to_move = side_to_move;
            evaluated_list[index as usize].move_0 =
                unsearched_move[i as usize];
            evaluated_list[index as usize].eval =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 0 as i32, 0.0f64, 0 as i32,
                                 0 as i32);
            evaluated_list[index as usize].pv_depth = 1;
            evaluated_list[index as usize].pv[0] =
                unsearched_move[i as usize];
            if empties_0 > (if wld > exact { wld } else { exact }) {
                transform1[i as usize] =
                    abs((g_state.random_instance).my_random() as i32) as u32;
                transform2[i as usize] =
                    abs((g_state.random_instance).my_random() as i32) as u32
            } else {
                transform1[i as usize] = 0;
                transform2[i as usize] = 0 as i32 as u32
            }
            i += 1;
            index += 1
        }
        stored_echo = echo;
        echo = 0;
        best_pv_depth = 0;
        if mid == 1 as i32 {
            /* compute_move won't be called */
            (g_state.board_state).pv_depth[0] = 0;
            (g_state.board_state).piece_count[0][(g_state.moves_state).disks_played as usize] =
                disc_count(0 as i32, &(g_state.board_state).board);
            (g_state.board_state).piece_count[2][(g_state.moves_state).disks_played as usize] =
                disc_count(2 as i32, &(g_state.board_state).board)
        }
        /* Perform iterative deepening if the search depth is large enough */
        if exact > empties_0 { exact = empties_0 }
        if exact < 12 as i32 || empties_0 > exact {
            current_exact = exact
        } else {
            current_exact =
                8 as i32 + exact % 2 as i32 - 2 as i32
        }
        if wld > empties_0 { wld = empties_0 }
        if wld < 14 as i32 || empties_0 > wld {
            current_wld = wld
        } else {
            current_wld =
                10 as i32 + wld % 2 as i32 - 2 as i32
        }
        if (empties_0 == exact || empties_0 == wld) &&
               empties_0 > 16 as i32 &&
               mid < empties_0 - 12 as i32 {
            mid = empties_0 - 12 as i32
        }
        if mid < 10 as i32 {
            current_mid = mid
        } else {
            current_mid =
                6 as i32 + mid % 2 as i32 - 2 as i32
        }
        first_iteration = 1;
        loop  {
            if current_mid < mid {
                current_mid += 2 as i32;
                /* Avoid performing deep midgame searches if the endgame
                   is reached anyway. */
                if empties_0 <= wld &&
                       current_mid + 7 as i32 >= empties_0 {
                    current_wld = wld;
                    current_mid = mid
                }
                if empties_0 <= exact &&
                       current_mid + 7 as i32 >= empties_0 {
                    current_exact = exact;
                    current_mid = mid
                }
            } else if current_wld < wld {
                current_wld = wld
            } else { current_exact = exact }
            i = 0;
            while i < unsearched_count && force_return == 0 {
                let mut this_eval =
                    EvaluationType{type_0: MIDGAME_EVAL,
                                   res: WON_POSITION,
                                   score: 0,
                                   confidence: 0.,
                                   search_depth: 0,
                                   is_book: 0,};
                this_move = unsearched_move[i as usize];
                /* Locate the current move in the list.  This has to be done
                   because the moves might have been reordered during the
                   iterative deepening. */
                index = 0;
                while evaluated_list[index as usize].move_0 != this_move {
                    index += 1
                }
                /* To avoid strange effects when browsing back and forth through
                   a game during the midgame, rehash the hash transformation masks
                   for each move unless the endgame is reached */
                (g_state.hash_state).set_hash_transformation(transform1[i as usize],
                                                             transform2[i as usize]);
                /* Determine the score for the ith move */
                prefix_move = this_move;
                make_move(side_to_move, this_move, 1 as i32, (&mut g_state.moves_state), (&mut g_state.board_state), (&mut g_state.hash_state), (&mut g_state.flip_stack_));
                if current_mid == 1 as i32 {
                    /* compute_move doesn't like 0-ply searches */
                    (g_state.search_state).evaluations.lo = (g_state.search_state).evaluations.lo.wrapping_add(1);
                    let side_to_move_argument = 0 as i32 + 2 as i32
                        - side_to_move;
                    shallow_eval =
                        pattern_evaluation(side_to_move_argument, (&mut g_state.board_state), &(g_state.moves_state), (&mut g_state.coeff_state));
                    this_eval =
                        create_eval_info(MIDGAME_EVAL, UNSOLVED_POSITION,
                                         shallow_eval, 0.0f64,
                                         0 as i32, 0 as i32)
                } else {
                    compute_move(0 as i32 + 2 as i32 -
                                     side_to_move, 0 as i32,
                                 0 as i32, 0 as i32,
                                 0 as i32, book,
                                 current_mid - 1 as i32,
                                 current_exact - 1 as i32,
                                 current_wld - 1 as i32,
                                 1 as i32, &mut this_eval, g_state);
                }
                if force_return != 0 {
                    /* Clear eval and exit search immediately */
                    this_eval =
                        create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                         0 as i32, 0.0f64,
                                         0 as i32, 0 as i32);
                    let move_0 = this_move;
                    {
                        unmake_move(side_to_move, move_0, &mut (g_state.board_state).board, &mut (g_state.moves_state), &mut (g_state.hash_state), &mut (g_state.flip_stack_));
                    };
                    break ;
                } else {
                    if this_eval.type_0 as u32 ==
                           PASS_EVAL as i32 as u32 {
                        /* Don't allow pass */
                        if current_mid == 1 as i32 {
                            /* compute_move doesn't like 0-ply searches */
                            (g_state.search_state).evaluations.lo = (g_state.search_state).evaluations.lo.wrapping_add(1);
                            shallow_eval = pattern_evaluation(side_to_move, &mut (g_state.board_state), &(g_state.moves_state), &mut (g_state.coeff_state));
                            this_eval =
                                create_eval_info(MIDGAME_EVAL,
                                                 UNSOLVED_POSITION,
                                                 shallow_eval, 0.0f64,
                                                 0 as i32,
                                                 0 as i32)
                        } else {
                            compute_move(side_to_move, 0 as i32,
                                         0 as i32, 0 as i32,
                                         0 as i32, book,
                                         current_mid - 1 as i32,
                                         current_exact - 1 as i32,
                                         current_wld - 1 as i32,
                                         1 as i32, &mut this_eval, g_state);
                        }
                        if this_eval.type_0 as u32 ==
                               PASS_EVAL as i32 as u32 {
                            /* Game over */
                            disc_diff =
                                disc_count(side_to_move, &(g_state.board_state).board) -
                                    disc_count(0 as i32 +
                                                   2 as i32 -
                                                   side_to_move, &(g_state.board_state).board);
                            if disc_diff > 0 as i32 {
                                corrected_diff =
                                    64 as i32 -
                                        2 as i32 *
                                            disc_count(0 as i32 +
                                                           2 as i32 -
                                                           side_to_move, &(g_state.board_state).board);
                                res = WON_POSITION
                            } else if disc_diff == 0 as i32 {
                                corrected_diff = 0;
                                res = DRAWN_POSITION
                            } else {
                                corrected_diff =
                                    2 as i32 *
                                        disc_count(side_to_move, &(g_state.board_state).board) -
                                        64 as i32;
                                res = LOST_POSITION
                            }
                            this_eval =
                                create_eval_info(EXACT_EVAL, res,
                                                 128 as i32 *
                                                     corrected_diff, 0.0f64,
                                                 60 as i32 -
                                                     (g_state.moves_state).disks_played,
                                                 0 as i32)
                        }
                    } else {
                        /* Sign-correct the score produced */
                        this_eval.score = -this_eval.score;
                        if this_eval.res as u32 ==
                               WON_POSITION as i32 as u32 {
                            this_eval.res = LOST_POSITION
                        } else if this_eval.res as u32 ==
                                      LOST_POSITION as i32 as
                                          u32 {
                            this_eval.res = WON_POSITION
                        }
                    }
                    if force_return != 0 { break ; }
                    evaluated_list[index as usize].eval = this_eval;
                    /* Store the PV corresponding to the move */
                    evaluated_list[index as usize].pv_depth =
                        (g_state.board_state).pv_depth[0] +
                            1 as i32;
                    evaluated_list[index as
                                       usize].pv[0] =
                        this_move;
                    j = 0;
                    while j < (g_state.board_state).pv_depth[0] {
                        evaluated_list[index as
                                           usize].pv[(j + 1 as i32) as
                                                         usize] =
                            (g_state.board_state).pv[0][j as usize];
                        j += 1
                    }
                    /* Store the PV corresponding to the best move */
                    if evaluated_list[index as usize].eval.score > best_score
                       {
                        best_score =
                            evaluated_list[index as usize].eval.score;
                        best_move = this_move;
                        best_pv_depth = (g_state.board_state).pv_depth[0];
                        j = 0;
                        while j < best_pv_depth {
                            best_pv[j as usize] =
                                (g_state.board_state).pv[0][j as usize];
                            j += 1
                        }
                    }
                    let move_0 = this_move;
                    {
                        unmake_move(side_to_move, move_0, &mut (g_state.board_state).board, &mut (g_state.moves_state), &mut (g_state.hash_state), &mut (g_state.flip_stack_));
                    };
                    /* Sort the moves evaluated */
                    if first_iteration != 0 { game_evaluated_count += 1 }
                    if force_return == 0 {
                        loop  {
                            changed = 0;
                            j = 0;
                            while j < game_evaluated_count - 1 as i32
                                  {
                                if compare_eval(evaluated_list[j as
                                                                   usize].eval,
                                                evaluated_list[(j +
                                                                    1 as
                                                                        i32)
                                                                   as
                                                                   usize].eval)
                                       < 0 as i32 {
                                    changed = 1;
                                    temp = evaluated_list[j as usize];
                                    evaluated_list[j as usize] =
                                        evaluated_list[(j + 1 as i32)
                                                           as usize];
                                    evaluated_list[(j + 1 as i32) as
                                                       usize] = temp
                                }
                                j += 1
                            }
                            if !(changed != 0) { break ; }
                        }
                    }
                    i += 1
                }
            }
            first_iteration = 0;
            /* Reorder the moves after each iteration.  Each move is moved to
            the front of the list, starting with the bad moves and ending
             with the best move.  This ensures that unsearched_move will be
             sorted w.r.t. the order in evaluated_list. */
            i = game_evaluated_count - 1 as i32;
            while i >= 0 as i32 {
                let this_move_0 = evaluated_list[i as usize].move_0;
                j = 0;
                while j != unsearched_count &&
                          unsearched_move[j as usize] != this_move_0 {
                    j += 1
                }
                if !(j == unsearched_count) {
                    /* Move the move to the front of the list. */
                    while j >= 1 as i32 {
                        unsearched_move[j as usize] =
                            unsearched_move[(j - 1 as i32) as usize];
                        j -= 1
                    }
                    unsearched_move[0] = this_move_0
                }
                /* Must be book move, skip */
                i -= 1
            }
            if !(force_return == 0 &&
                     (current_mid != mid || current_exact != exact ||
                          current_wld != wld)) {
                break ;
            }
        }
        echo = stored_echo;
        game_evaluated_count = (g_state.moves_state).move_count[(g_state.moves_state).disks_played as usize];
        /* Make sure that the PV and the score correspond to the best move */
        (g_state.board_state).pv_depth[0] =
            best_pv_depth + 1 as i32;
        (g_state.board_state).pv[0][0] = best_move;
        i = 0;
        while i < best_pv_depth {
            (g_state.board_state).pv[0][(i + 1 as i32) as usize] =
                best_pv[i as usize];
            i += 1
        }
        let negate = 0 as i32;
        (g_state.search_state).negate_current_eval(negate);
        if (g_state.moves_state).move_count[(g_state.moves_state).disks_played as usize] > 0 as i32 {
            let eval_argument = evaluated_list[0].eval;
            (g_state.search_state).set_current_eval(eval_argument);
        }
    }
    /* Reset the hash transformation masks prior to leaving */
    (g_state.hash_state).set_hash_transformation(0 as i32 as u32,
                                                 0 as i32 as u32);
    /* Don't forget to enable the time control mechanisms when leaving */
    (g_state.g_timer).toggle_abort_check(1 as i32);
    (g_state.midgame_state).toggle_midgame_abort_check(1 as i32);
    (g_state.midgame_state).toggle_perturbation_usage(1 as i32);
    (g_state.game_state).max_depth_reached += 1;
    prefix_move = 0;
    return best_move;
}
/*
  PERFORM_EXTENDED_SOLVE
  Calculates exact score or WLD status for the move ACTUAL_MOVE as
  well as for the best move in the position (if it is any other move).
*/

pub unsafe fn perform_extended_solve(side_to_move: i32,
                                                actual_move: i32,
                                                book: i32,
                                                exact_solve:
                                                    i32, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut mid: i32 = 0;
    let mut wld: i32 = 0;
    let mut exact: i32 = 0;
    let mut best_move: i32 = 0;
    let mut disc_diff: i32 = 0;
    let mut corrected_diff: i32 = 0;
    let mut temp =
        EvaluatedMove{eval:
                          EvaluationType{type_0: MIDGAME_EVAL,
                                         res: WON_POSITION,
                                         score: 0,
                                         confidence: 0.,
                                         search_depth: 0,
                                         is_book: 0,},
                      side_to_move: 0,
                      move_0: 0,
                      pv_depth: 0,
                      pv: [0; 60],};
    let mut res = WON_POSITION;
    /* Disable all time control mechanisms */
    ( g_state.g_timer).toggle_abort_check(0 as i32);
    ( g_state.midgame_state).toggle_midgame_abort_check(0 as i32);
    ( g_state.midgame_state).toggle_perturbation_usage(0 as i32);
     ( g_state.g_timer).start_move(0 as i32 as f64,
                                         0 as i32 as f64,
                                         disc_count(0 as i32, &( g_state.board_state).board) + disc_count(2 as i32, &( g_state.board_state).board));
     ( g_state.g_timer).clear_ponder_times();
    determine_hash_values(side_to_move, &( g_state.board_state).board, &mut ( g_state.hash_state));
    reset_counter(&mut ( g_state.search_state).nodes);
    /* Set search depths that result in Zebra solving after a brief
       midgame analysis */
    mid = 60;
    wld = 60;
    if exact_solve != 0 {
        exact = 60 as i32
    } else { exact = 0 as i32 }
    game_evaluated_count = 1;
    /* Calculate the score for the preferred move */
    evaluated_list[0].side_to_move = side_to_move;
    evaluated_list[0].move_0 = actual_move;
    evaluated_list[0].eval =
        create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION, 0 as i32,
                         0.0f64, 0 as i32, 0 as i32);
    evaluated_list[0].pv_depth = 1;
    evaluated_list[0].pv[0] =
        actual_move;
    prefix_move = actual_move;
    let negate = 1 as i32;
    ( g_state.search_state).negate_current_eval(negate);
    make_move(side_to_move, actual_move, 1 as i32, &mut ( g_state.moves_state), &mut ( g_state.board_state), &mut ( g_state.hash_state), &mut ( g_state.flip_stack_));
    compute_move(0 as i32 + 2 as i32 - side_to_move,
                 0 as i32, 0 as i32, 0 as i32,
                 0 as i32, book, mid - 1 as i32,
                 exact - 1 as i32, wld - 1 as i32,
                 1 as i32,
                 &mut (*evaluated_list.as_mut_ptr()).eval, g_state);
    if evaluated_list[0].eval.type_0 as u32
           == PASS_EVAL as i32 as u32 {
        /* Don't allow pass */
        compute_move(side_to_move, 0 as i32, 0 as i32,
                     0 as i32, 0 as i32, book,
                     mid - 1 as i32, exact - 1 as i32,
                     wld - 1 as i32, 1 as i32,
                     &mut (*evaluated_list.as_mut_ptr()).eval, g_state);
        if evaluated_list[0].eval.type_0 as
               u32 == PASS_EVAL as i32 as u32 {
            /* Game has ended */
            disc_diff =
                disc_count(side_to_move, &( g_state.board_state).board) -
                    disc_count(0 as i32 + 2 as i32 -
                                   side_to_move, &( g_state.board_state).board);
            if disc_diff > 0 as i32 {
                corrected_diff =
                    64 as i32 -
                        2 as i32 *
                            disc_count(0 as i32 + 2 as i32 -
                                           side_to_move, &( g_state.board_state).board);
                res = WON_POSITION
            } else if disc_diff == 0 as i32 {
                corrected_diff = 0;
                res = DRAWN_POSITION
            } else {
                corrected_diff =
                    2 as i32 * disc_count(side_to_move, &( g_state.board_state).board) -
                        64 as i32;
                res = LOST_POSITION
            }
            evaluated_list[0].eval =
                create_eval_info(EXACT_EVAL, res,
                                 128 as i32 * corrected_diff, 0.0f64,
                                 60 as i32 - ( g_state.moves_state).disks_played,
                                 0 as i32)
        }
    } else {
        /* Sign-correct the score produced */
        evaluated_list[0].eval.score =
            -evaluated_list[0].eval.score;
        if evaluated_list[0].eval.res as u32
               == WON_POSITION as i32 as u32 {
            evaluated_list[0].eval.res = LOST_POSITION
        } else if evaluated_list[0].eval.res as
                      u32 ==
                      LOST_POSITION as i32 as u32 {
            evaluated_list[0].eval.res = WON_POSITION
        }
    }
    if force_return != 0 {
        evaluated_list[0].eval =
            create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                             0 as i32, 0.0f64, 0 as i32,
                             0 as i32)
    } else {
        evaluated_list[0].pv_depth =
            ( g_state.board_state).pv_depth[0] + 1 as i32;
        evaluated_list[0].pv[0] = actual_move;
        i = 0;
        while i < ( g_state.board_state).pv_depth[0] {
            evaluated_list[0].pv[(i + 1 as i32) as usize] =
                ( g_state.board_state).pv[0][i as usize];
            i += 1
        }
    }
    let move_0 = actual_move;
    {
        unmake_move(side_to_move, move_0, &mut ( g_state.board_state).board, &mut ( g_state.moves_state), &mut ( g_state.hash_state), &mut ( g_state.flip_stack_));
    };
    prefix_move = 0;
    let negate = 0 as i32;
    ( g_state.search_state).negate_current_eval(negate);
    ( g_state.game_state).max_depth_reached += 1;
    /* Compute the score for the best move and store it in the move list
       if it isn't ACTUAL_MOVE */
    best_move =
        compute_move(side_to_move, 0 as i32, 0 as i32,
                     0 as i32, 0 as i32, book, mid, exact,
                     wld, 1 as i32,
                     &mut (*evaluated_list.as_mut_ptr().offset(1)).eval, g_state);
    if force_return == 0 && best_move != actual_move {
        /* Move list will contain best move first and then the actual move */
        game_evaluated_count = 2;
        evaluated_list[1].side_to_move = side_to_move;
        evaluated_list[1].move_0 = best_move;
        evaluated_list[1].pv_depth =
            ( g_state.board_state).pv_depth[0];
        i = 0;
        while i < ( g_state.board_state).pv_depth[0] {
            evaluated_list[1].pv[i as usize] =
                ( g_state.board_state).pv[0][i as usize];
            i += 1
        }
        temp = evaluated_list[0];
        evaluated_list[0] =
            evaluated_list[1];
        evaluated_list[1] = temp
    }
    /* The PV and current eval should when correspond to the best move
       when leaving */
    ( g_state.board_state).pv_depth[0] =
        evaluated_list[0].pv_depth;
    i = 0;
    while i < ( g_state.board_state).pv_depth[0] {
        ( g_state.board_state).pv[0][i as usize] =
            evaluated_list[0].pv[i as usize];
        i += 1
    }
    let eval_argument = evaluated_list[0].eval;
    ( g_state.search_state).set_current_eval(eval_argument);
    /* Don't forget to enable the time control mechanisms when leaving */
    ( g_state.g_timer).toggle_abort_check(1 as i32);
    ( g_state.midgame_state).toggle_midgame_abort_check(1 as i32);
    ( g_state.midgame_state).toggle_perturbation_usage(0 as i32);
}

/*
   COMPUTE_MOVE
   Returns the best move in a position given search parameters.
*/
pub unsafe fn compute_move(side_to_move: i32,
                           update_all: i32,
                           my_time: i32,
                           my_incr: i32,
                           timed_depth: i32,
                           book: i32,
                           mid: i32,
                           exact: i32,
                           wld: i32,
                           search_forced: i32,
                           eval_info: &mut EvaluationType, g_state: &mut FullState)
                           -> i32 {
    let mut g_config = (&mut g_state.g_config);
    let mut learn_state = (&mut g_state.learn_state);
    let mut midgame_state = (&mut g_state.midgame_state);
    let mut game_state = (&mut g_state.game_state);
    let mut end_g = (&mut g_state.end_g);
    let mut coeff_state = (&mut g_state.coeff_state);
    let mut g_timer = (&mut g_state.g_timer);
    let mut moves_state = (&mut g_state.moves_state);
    let mut stable_state = (&mut g_state.stable_state);
    let mut board_state = (&mut g_state.board_state);
    let mut hash_state = (&mut g_state.hash_state);
    let mut random_instance = (&mut g_state.random_instance);
    let mut g_book = (&mut g_state.g_book);
    let mut prob_cut = (&mut g_state.prob_cut);
    let mut search_state = (&mut g_state.search_state);
    let mut flip_stack_ = (&mut g_state.flip_stack_);

    return generic_compute_move::<LogFileHandler, LibcZebraOutput, LibcFatalError, LegacyThor>(side_to_move, update_all, my_time,
                                my_incr, timed_depth,
                                book, mid,
                                exact, wld,
                                search_forced, eval_info, &mut LogFileHandler::create_log_file_if_needed(), g_config.display_pv, g_config.echo,   &mut flip_stack_,
                                                                                               &mut search_state,
                                                                                               &mut board_state,
                                                                                               &mut hash_state,
                                                                                               &mut g_timer,
                                                                                               &mut end_g,
                                                                                               &mut midgame_state,
                                                                                               &mut coeff_state,
                                                                                               &mut moves_state,
                                                                                               &mut random_instance,
                                                                                               &mut g_book,
                                                                                               &mut stable_state,
                                                                                               &mut game_state, &mut prob_cut);
}

pub struct LibcZebraOutput;
impl ComputeMoveOutput for LibcZebraOutput {
fn display_out_optimal_line(search_state: &SearchState) {
    //FIXME parametrize, this touches global state
    unsafe { display_optimal_line(&mut stdout, search_state.full_pv_depth, &search_state.full_pv) }
}

fn send_move_type_0_status(interrupted_depth: i32, info: &EvaluationType, counter_value: f64, timer: &mut Timer, board_state: &BoardState) {
    unsafe {
        clear_status();
        send_status_1(b"--> *%2d\x00" as *const u8 as *const i8,
                    interrupted_depth);
        let mut eval_str_ = produce_eval_text(info, 1 as i32);
        let eval_str = eval_str_.as_mut_ptr();
        send_status_1(b"%10s  \x00" as *const u8 as *const i8,
                    eval_str);
        send_status_nodes(counter_value);
        send_status_pv(&board_state.pv[0], interrupted_depth, board_state.pv_depth[0]);
        send_status_time(timer.get_elapsed_time());
        if timer.get_elapsed_time() != 0.0f64 {
            send_status_2(b"%6.0f %s\x00" as *const u8 as
                            *const i8,
                        counter_value /
                            (timer.get_elapsed_time() + 0.001f64),
                        b"nps\x00" as *const u8 as *const i8);
        }
    }
}

fn display_status_out() {
    unsafe { display_status(stdout, 0 as i32); }
}

fn echo_ponder_move_4(curr_move: i32, ponder_move: i32) {
    unsafe {
        send_status_1(b"-->   %s        \x00" as *const u8 as
                        *const i8,
                    b"Thor database\x00" as *const u8 as
                        *const i8);
        if ponder_move != 0 {
            send_status_2(b"{%c%c} \x00" as *const u8 as
                            *const i8,
                        'a' as i32 +
                            ponder_move % 10 as i32 -
                            1 as i32,
                        '0' as i32 +
                            ponder_move / 10 as i32);
        }
        send_status_2(b"%c%c\x00" as *const u8 as *const i8,
                    'a' as i32 + curr_move % 10 as i32 -
                        1 as i32,
                    '0' as i32 + curr_move / 10 as i32);
        display_status(stdout, 0 as i32);
    }
}

fn echo_ponder_move_2(curr_move: i32, ponder_move: i32) {
    unsafe {
        send_status_1(b"-->   %s        \x00" as *const u8 as
                        *const i8,
                    b"Thor database\x00" as *const u8 as
                        *const i8);
        if ponder_move != 0 {
            send_status_2(b"{%c%c} \x00" as *const u8 as
                            *const i8,
                        'a' as i32 +
                            ponder_move % 10 as i32
                            - 1 as i32,
                        '0' as i32 +
                            ponder_move /
                                10 as i32);
        }
        send_status_2(b"%c%c\x00" as *const u8 as
                        *const i8,
                    'a' as i32 + curr_move % 10 as i32 -
                        1 as i32,
                    '0' as i32 + curr_move / 10 as i32);
        display_status(stdout, 0 as i32);
    }
}

fn echo_ponder_move(curr_move: i32, ponder_move: i32) {
    unsafe {
        send_status_0(b"-->   Forced opening move        \x00" as
            *const u8 as *const i8);
        if ponder_move != 0 {
            send_status_2(b"{%c%c} \x00" as *const u8 as
                            *const i8,
                        'a' as i32 +
                            ponder_move % 10 as i32 -
                            1 as i32,
                        '0' as i32 +
                            ponder_move / 10 as i32);
        }
        send_status_2(b"%c%c\x00" as *const u8 as *const i8,
                    'a' as i32 + curr_move % 10 as i32 -
                        1 as i32,
                    '0' as i32 + curr_move / 10 as i32);
        display_status(stdout, 0 as i32);
    }
}

fn echo_compute_move_2(info: &EvaluationType, disk: i32) {
    unsafe {
        let mut eval_str_ = produce_eval_text(info, 0 as i32);
        let eval_str = eval_str_.as_mut_ptr();
        send_status_0(b"-->         \x00" as *const u8 as
            *const i8);
        send_status_1(b"%-8s  \x00" as *const u8 as *const i8,
                    eval_str);
        send_status_2(b"%c%c \x00" as *const u8 as *const i8,
                    'a' as i32 +
                        disk %
                            10 as i32 - 1 as i32,
                    '0' as i32 +
                        disk /
                            10 as i32);
        display_status(stdout, 0 as i32);
    }
}

fn echo_compute_move_1(info: &EvaluationType) {
    unsafe {
        let mut eval_str_ = produce_eval_text(info, 0 as i32);
        let eval_str = eval_str_.as_mut_ptr();
        send_status_0(b"-->         \x00" as *const u8 as
            *const i8);
        send_status_1(b"%-8s  \x00" as *const u8 as *const i8,
                    eval_str);
        display_status(stdout, 0 as i32);
    }
}
}
impl LogFileHandler {
    fn create(log_file_path_: &mut [i8]) -> Option<Self> {
        let log_file = unsafe {
            fopen(log_file_path_.as_mut_ptr(),
                  b"a\x00" as *const u8 as *const i8)
        };
        if !log_file.is_null() {
            let logger = LogFileHandler { log_file };
            Some(logger)
        } else {
            None
        }
    }
}
impl ComputeMoveLogger for LogFileHandler {

fn create_log_file_if_needed() -> Option<Self> {
    unsafe {
        if use_log_file != 0 {
            let log_file_path_ = &mut log_file_path as &mut [i8];
            Self::create(log_file_path_)
        } else {
            None
        }
    }
}

fn log_moves_generated(logger: &mut LogFileHandler, moves_generated: i32, move_list_for_disks_played: &[i32; 64]) {
    write!(&mut logger.log_file, "{} {}: ", moves_generated, "moves generated");
    let mut i = 0;
    while i < moves_generated {
        write!(&mut logger.log_file,
               "{}{} ",
               char::from('a' as u8 + (move_list_for_disks_played[i as usize] % 10) as u8 - 1),
               char::from('0' as u8 + (move_list_for_disks_played[i as usize] / 10) as u8)
        );
        i += 1
    }
    write!(&mut logger.log_file, "\n");
}

fn log_best_move_pass(logger: &mut LogFileHandler) {
    write!(&mut logger.log_file, "{}: {}\n", "Best move", "pass");
    unsafe {
        fclose(logger.log_file);
    }
}

fn log_best_move(logger: &mut LogFileHandler, best_move: i32) {
    write!(&mut logger.log_file, "{}: {}{}  ({})\n",
           "Best move",
           char::from('a' as u8 + (best_move % 10) as u8 - 1),
           char::from('0' as u8 + (best_move / 10) as u8),
           "forced");
    unsafe {
        fclose(logger.log_file);
    }
}

fn log_chosen_move(logger: &mut LogFileHandler, curr_move: i32, info: &EvaluationType) {
    unsafe {
        let mut eval_str_ = produce_eval_text(info, 0 as i32);
        let eval_str = eval_str_.as_mut_ptr();
        fprintf(logger.log_file,
                b"%s: %c%c  %s\n\x00" as *const u8 as *const i8,
                b"Move chosen\x00" as *const u8 as *const i8,
                'a' as i32 + curr_move % 10 as i32 -
                    1 as i32,
                '0' as i32 + curr_move / 10 as i32, eval_str);
    }
}

fn log_status(logger: &mut LogFileHandler) {
    unsafe { display_status(logger.log_file, 1 as i32); }
}

fn log_optimal_line(logger: &mut LogFileHandler, search_state: &SearchState) {
    unsafe { display_optimal_line(&mut logger.log_file, search_state.full_pv_depth, &search_state.full_pv); }
}

fn close_logger(logger: &mut LogFileHandler) {
    if !logger.log_file.is_null() { unsafe { fclose(logger.log_file); } }
}

fn log_board(logger: &mut LogFileHandler, board_state: &BoardState, side_to_move_: i32) {
    let board_ = &board_state.board;
    unsafe {
        display_board(&mut logger.log_file, board_, side_to_move_,
                      0 as i32, 0 as i32, 0 as i32,
                      display_state.current_row,
                      display_state.black_player, display_state.black_time, display_state.black_eval,
                      display_state.white_player, display_state.white_time, display_state.white_eval,
                      &board_state.black_moves, &board_state.white_moves
        );
    }
}
}
