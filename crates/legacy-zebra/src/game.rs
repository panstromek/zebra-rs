use std::ffi::{CStr};

use engine::src::counter::{adjust_counter, counter_value, reset_counter};
use engine::src::error::FrontEnd;
use engine::src::game::{BoardSource, ComputeMoveLogger, ComputeMoveOutput, engine_global_setup, EvaluatedMove, FileBoardSource, generic_compute_move, generic_game_init, EvaluatedList};
use engine::src::hash::{determine_hash_values, find_hash, HashEntry};
use engine::src::moves::{make_move, unmake_move};
use engine::src::search::{create_eval_info, disc_count, float_move, force_return, sort_moves, SearchState};
use engine::src::thordb::ThorDatabase;
use engine::src::zebra::EvalResult::{DRAWN_POSITION, LOST_POSITION, UNSOLVED_POSITION, WON_POSITION};
use engine::src::zebra::EvalType::{EXACT_EVAL, PASS_EVAL, UNDEFINED_EVAL};
use engine::src::zebra::EvaluationType;
use libc_wrapper::{stdout, time, time_t, c_time};

use crate::src::display::{display_optimal_line, produce_eval_text, display_state, TO_SQUARE};
use crate::src::error::{LibcFatalError};
use crate::src::getcoeff::{load_coeff_adjustments, new_coeff_source};
use crate::src::thordb::LegacyThor;
use crate::src::zebra::FullState;
use engine::src::globals::BoardState;
use std::fs::{File, OpenOptions};
use std::io::{Read, BufReader, BufRead, Write};
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::SeqCst;

use engine::src::timer::Timer;

#[macro_use]
use crate::send_status;

pub static use_log_file: AtomicI32 = AtomicI32::new(1);
/*
  TOGGLE_STATUS_LOG
  Enable/disable the use of logging all the output that the
  text version of Zebra would output to the screen.
*/

pub fn toggle_status_log(write_log: i32) {
    use_log_file.store(write_log, SeqCst);
}

/*
   GLOBAL_SETUP
   Initialize the different sub-systems.
*/

pub unsafe fn global_setup(use_random: i32, hash_bits: i32, mut g_state: &mut FullState) {
    setup_log_file();
    let coeff_adjustments = load_coeff_adjustments();

    let file_name = match std::env::var("COEFFS_PATH") {
        Ok(var) => var,
        Err(_) => "./coeffs2.bin".to_owned(),
    };
    engine_global_setup::<_, LibcFatalError>(
        use_random, hash_bits, coeff_adjustments, new_coeff_source(file_name.as_ref()),
        &mut g_state.search, &mut g_state.hash, &mut g_state.timer,
        &mut g_state.coeff, &mut g_state.random, &mut g_state.stable, &mut g_state.prob_cut);
}

pub struct LogFileHandler {
    log_file: File,
}

unsafe fn setup_log_file() {
    /* Clear the log file. No error handling done. */
    if use_log_file.load(SeqCst) != 0 {
        if let Ok(mut log_file) = OpenOptions::new().write(true).truncate(true).create(true).open("zebra.log") {
            let mut timer_: time_t = 0;
            time(&mut timer_);
            write!(log_file, "{} {}\n", "Log file created", c_time(timer_));
            write!(log_file, "{} {} {}\n", "Engine compiled", "Jul  2 2020", "19:33:59");
            log_file.flush();
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
        write!(stdout, "{} '{}' {}\n", "Unrecognized character", unrecognized as u8 as char, "in game file");
    }
}

pub fn game_init(side_to_move: &mut i32, g_state: &mut FullState) {
    generic_game_init::<BasicBoardFileSource, LibcFatalError>(None, side_to_move, g_state);
}
/*
  PONDER_MOVE
  Perform searches in response to the opponent's next move.
  The results are not returned, but the hash table is filled
  with useful scores and moves.
*/

fn report_move_evals(expect_count: i32, move_list_item: &[i8; 64], evals_item: &[i32; 128]) {
    let mut i = 0;
    while i < expect_count {
        let move__ = move_list_item[i as usize] as i32;
        let move_eval = evals_item[move__ as usize];
        write!(stdout, "{} {:<6.2}  ", TO_SQUARE(move__), move_eval as f64 / 128.0f64);
        if i % 7 == 6 || i == expect_count - 1 {
            write!(stdout, "\n");
        }
        i += 1
    }
}

fn report_hash_move(hash_move: i8) {
    write!(stdout, "{}={}\n", "hash move", hash_move as i32);
}

pub fn ponder_move<
    L: ComputeMoveLogger,
    Out: ComputeMoveOutput,
    FE: FrontEnd,
    Thor: ThorDatabase>(side_to_move: i32,
                           _book: i32,
                           mid: i32,
                           exact: i32,
                           wld: i32, display_pv: i32, mut echo:i32, g_state: &mut FullState, thor: &mut Thor) {
    type Rep = LibcFatalError;

    let mut eval_info =EvaluationType::new();
    let mut entry = HashEntry::new();
    let mut move_start_time: f64 = 0.;
    let mut move_stop_time: f64 = 0.;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut this_move = 0;
    let mut expect_count: i32 = 0;
    let mut expect_list: [i8; 64] = [0; 64];
    let mut best_pv: [i8; 61] = [0; 61];
    /* Disable all time control mechanisms as it's the opponent's
       time we're using */
    &mut g_state.timer.toggle_abort_check(0);
    &mut g_state.midgame.toggle_midgame_abort_check(0);
     &mut g_state.timer.start_move(0 as f64,
                                   0 as f64,
                                   disc_count(0, &&mut g_state.board.board) + disc_count(2, &&mut g_state.board.board));
     &mut g_state.timer.clear_ponder_times();
    determine_hash_values(side_to_move, &&mut g_state.board.board, &mut g_state.hash);
    reset_counter(&mut &mut g_state.search.nodes);
    /* Find the scores for the moves available to the opponent. */
    let mut hash_move = 0;
    find_hash(&mut entry, 1, &mut g_state.hash);
    if entry.draft as i32 != 0 {
        hash_move = entry.move_0[0]
    } else {
        find_hash(&mut entry, 0, &mut g_state.hash);
        if entry.draft as i32 != 0 {
            hash_move = entry.move_0[0]
        }
    }
    let stored_echo = echo;
    echo = 0;
    engine::src::game::compute_move::<L, Out, FE, Thor>(side_to_move, 0, 0,
                                     0, 0, 0,
                                     if (8) < mid {
                                         8
                                     } else { mid }, 0, 0,
                                     0, &mut eval_info, display_pv, echo, g_state, thor);
    echo = stored_echo;
    /* Sort the opponents on the score and push the table move (if any)
       to the front of the list */
    if force_return != 0 {
        expect_count = 0
    } else {
        sort_moves(g_state.moves.move_count[g_state.moves.disks_played as usize], &mut g_state.moves, &&mut g_state.search);
        float_move(hash_move, g_state.moves.move_count[g_state.moves.disks_played as usize], &mut g_state.moves);
        expect_count = g_state.moves.move_count[g_state.moves.disks_played as usize];
        i = 0;
        while i < expect_count {
            expect_list[i as usize] = g_state.moves.move_list[ g_state.moves.disks_played as usize][i as usize];
            i += 1
        }
        report_hash_move(hash_move);
        let move_list_item = &&mut g_state.moves.move_list[g_state.moves.disks_played as usize];
        let evals_item = &&mut g_state.search.evals[g_state.moves.disks_played as usize];
        report_move_evals(expect_count, move_list_item, evals_item);
    }
    /* Go through the expected moves in order and prepare responses. */
    let mut best_pv_depth = 0;
    let mut i = 0;
    while force_return == 0 && i < expect_count {
        move_start_time = g_state.timer.get_real_timer();
        let move_0 = expect_list[i as usize];
        g_state.search.set_ponder_move(move_0);
        this_move = expect_list[i as usize];
        g_state.game.prefix_move = this_move;
        make_move(side_to_move, this_move, 1, &mut g_state.moves, &mut g_state.board, &mut g_state.hash, &mut g_state.flip_stack);
        engine::src::game::compute_move::<L, Out, FE, Thor>(0 + 2 - side_to_move,
                                         0, 0, 0,
                                         1, 0, mid, exact, wld,
                                         0, &mut eval_info, display_pv, echo, g_state, thor);
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut g_state.board.board, &mut g_state.moves, &mut g_state.hash, &mut g_state.flip_stack);
        };
        g_state.search.clear_ponder_move();
        move_stop_time =  g_state.timer.get_real_timer();
        let move_0 = expect_list[i as usize];
        let time_0 = move_stop_time - move_start_time;
        g_state.timer.add_ponder_time(move_0, time_0);
        g_state.timer.ponder_depth[expect_list[i as usize] as usize] = if g_state.timer.ponder_depth[expect_list[i as usize] as usize] > g_state.game.max_depth_reached - 1 {
            g_state.timer.ponder_depth[expect_list[i as usize] as usize]
        } else {
            (g_state.game.max_depth_reached) - 1
        };
        if i == 0 && force_return == 0 {
            /* Store the PV for the first move */
            best_pv_depth = g_state.board.pv_depth[0];
            j = 0;
            while j < g_state.board.pv_depth[0] {
                best_pv[j as usize] = g_state.board.pv[0][j as usize];
                j += 1
            }
        }
        i += 1
    }
    /* Make sure the PV looks reasonable when leaving - either by
       clearing it altogether or, preferrably, using the stored PV for
       the first move if it is available. */
    g_state.game.max_depth_reached += 1;
    g_state.game.prefix_move = 0;
    if best_pv_depth == 0 {
        g_state.board.pv_depth[0] = 0
    } else {
        g_state.board.pv_depth[0] = best_pv_depth + 1;
        g_state.board.pv[0][0] = expect_list[0];
        i = 0;
        while i < best_pv_depth {
            g_state.board.pv[0][(i + 1) as usize] = best_pv[i as usize];
            i += 1
        }
    }
    /* Don't forget to enable the time control mechanisms when leaving */
    &mut g_state.timer.toggle_abort_check(1);
    &mut g_state.midgame.toggle_midgame_abort_check(1);
}
/*
  GET_SEARCH_STATISTICS
  Returns some statistics about the last search made.
*/

pub fn get_search_statistics(max_depth: &mut i32, node_count: &mut f64, g_state: &mut FullState) {
    let FullState {
        game: ref mut game_state,
        search: ref mut search_state,
        ..
    } : &mut FullState = g_state;
    *max_depth = game_state.max_depth_reached;
    if game_state.prefix_move != 0 {
        *max_depth += 1
    }
    adjust_counter(&mut search_state.nodes);
    *node_count = counter_value(&mut search_state.nodes);
}


/*
  GET_PV
  Returns the principal variation.
*/

pub fn get_pv(destin: &mut [i8], g_state: &mut FullState) -> i32 {
    let FullState { game: ref mut game_state, board: ref mut board_state, .. } = g_state;
    let mut i = 0;
    return if game_state.prefix_move == 0 {
        i = 0;
        while i < board_state.pv_depth[0] {
            destin[i as usize] = board_state.pv[0][i as usize];
            i += 1
        }
        board_state.pv_depth[0]
    } else {
        destin[0] = game_state.prefix_move;
        i = 0;
        while i < board_state.pv_depth[0] {
            destin[(i + 1) as usize] = board_state.pv[0][i as usize];
            i += 1
        }
        board_state.pv_depth[0] + 1
    };
}
pub fn extended_compute_move<FE: FrontEnd>(
    side_to_move: i32, book_only: i32, book: i32, mid: i32, exact: i32, wld: i32, echo: i32, g_state: &mut FullState,
    thor: &mut LegacyThor
)
    -> EvaluatedList {
    engine::src::game::extended_compute_move::<LogFileHandler, LibcZebraOutput, FE, LegacyThor, _>(
        side_to_move, book_only, book, mid, exact, wld, echo, g_state, |_| (), || false, thor
    )
}
/*
  PERFORM_EXTENDED_SOLVE
  Calculates exact score or WLD status for the move ACTUAL_MOVE as
  well as for the best move in the position (if it is any other move).
*/

pub fn perform_extended_solve(side_to_move: i32,
                                                actual_move: i8,
                                                book: i32,
                                                exact_solve:
                                                    i32, g_state: &mut FullState, thor: &mut LegacyThor) {
    let mut i: i32 = 0;
    let mut mid: i32 = 0;
    let mut wld: i32 = 0;
    let mut exact: i32 = 0;
    let mut best_move = 0;
    let mut disc_diff: i32 = 0;
    let mut corrected_diff: i32 = 0;
    let mut temp = EvaluatedMove::new();
    let mut res = WON_POSITION;
    /* Disable all time control mechanisms */
    ( g_state.timer).toggle_abort_check(0);
    ( g_state.midgame).toggle_midgame_abort_check(0);
    ( g_state.midgame).toggle_perturbation_usage(0);
     ( g_state.timer).start_move(0 as f64,
                                 0 as f64,
                                 disc_count(0, &( g_state.board).board) + disc_count(2, &( g_state.board).board));
     ( g_state.timer).clear_ponder_times();
    determine_hash_values(side_to_move, &( g_state.board).board, &mut ( g_state.hash));
    reset_counter(&mut ( g_state.search).nodes);
    /* Set search depths that result in Zebra solving after a brief
       midgame analysis */
    mid = 60;
    wld = 60;
    if exact_solve != 0 {
        exact = 60
    } else { exact = 0 }
    let mut game_evaluated_count = 1;
    let mut evaluated_list: [EvaluatedMove; 60] = [EvaluatedMove::new(); 60];
    /* Calculate the score for the preferred move */
    evaluated_list[0].side_to_move = side_to_move;
    evaluated_list[0].move_0 = actual_move;
    evaluated_list[0].eval =
        create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION, 0,
                         0.0f64, 0, 0);
    evaluated_list[0].pv_depth = 1;
    evaluated_list[0].pv[0] =
        actual_move;
    g_state.game.prefix_move = actual_move;
    let negate = 1;
    ( g_state.search).negate_current_eval(negate);
    make_move(side_to_move, actual_move, 1, &mut ( g_state.moves), &mut ( g_state.board), &mut ( g_state.hash), &mut ( g_state.flip_stack));
    legacy_compute_move(0 + 2 - side_to_move,
                        0, 0, 0,
                        0, book, mid - 1,
                        exact - 1, wld - 1,
                        1,
                        &mut evaluated_list[0].eval, g_state, thor);
    if evaluated_list[0].eval.type_0 as u32 == PASS_EVAL as i32 as u32 {
        /* Don't allow pass */
        legacy_compute_move(side_to_move, 0, 0,
                            0, 0, book,
                            mid - 1, exact - 1,
                            wld - 1, 1,
                            &mut evaluated_list[0].eval, g_state, thor);
        if evaluated_list[0].eval.type_0 as u32 == PASS_EVAL as i32 as u32 {
            /* Game has ended */
            disc_diff = disc_count(side_to_move, &(g_state.board).board) -
                disc_count(0 + 2 - side_to_move, &(g_state.board).board);
            if disc_diff > 0 {
                corrected_diff = 64 - 2 * disc_count(0 + 2 - side_to_move, &(g_state.board).board);
                res = WON_POSITION
            } else if disc_diff == 0 {
                corrected_diff = 0;
                res = DRAWN_POSITION
            } else {
                corrected_diff = 2 * disc_count(side_to_move, &(g_state.board).board) - 64;
                res = LOST_POSITION
            }
            evaluated_list[0].eval = create_eval_info(
                EXACT_EVAL, res,
                128 * corrected_diff, 0.0f64,
                60 - (g_state.moves).disks_played, 0)
        }
    } else {
        /* Sign-correct the score produced */
        evaluated_list[0].eval.score = -evaluated_list[0].eval.score;
        if evaluated_list[0].eval.res == WON_POSITION {
            evaluated_list[0].eval.res = LOST_POSITION
        } else if evaluated_list[0].eval.res == LOST_POSITION {
            evaluated_list[0].eval.res = WON_POSITION
        }
    }
    if force_return != 0 {
        evaluated_list[0].eval = create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                                  0, 0.0f64, 0, 0)
    } else {
        evaluated_list[0].pv_depth = (g_state.board).pv_depth[0] + 1;
        evaluated_list[0].pv[0] = actual_move;
        i = 0;
        while i < ( g_state.board).pv_depth[0] {
            evaluated_list[0].pv[(i + 1) as usize] = (g_state.board).pv[0][i as usize];
            i += 1
        }
    }
    unmake_move(side_to_move, actual_move, &mut (g_state.board).board, &mut (g_state.moves), &mut (g_state.hash), &mut (g_state.flip_stack));
    g_state.game.prefix_move = 0;
    let negate = 0;
    ( g_state.search).negate_current_eval(negate);
    ( g_state.game).max_depth_reached += 1;
    /* Compute the score for the best move and store it in the move list
       if it isn't ACTUAL_MOVE */
    best_move = legacy_compute_move(side_to_move, 0, 0,
                                    0, 0, book, mid, exact,
                                    wld, 1, &mut evaluated_list[1].eval, g_state, thor);
    if force_return == 0 && best_move != actual_move {
        /* Move list will contain best move first and then the actual move */
        game_evaluated_count = 2;
        evaluated_list[1].side_to_move = side_to_move;
        evaluated_list[1].move_0 = best_move;
        evaluated_list[1].pv_depth = (g_state.board).pv_depth[0];
        i = 0;
        while i < (g_state.board).pv_depth[0] {
            evaluated_list[1].pv[i as usize] = (g_state.board).pv[0][i as usize];
            i += 1
        }
        temp = evaluated_list[0];
        evaluated_list[0] = evaluated_list[1];
        evaluated_list[1] = temp
    }
    /* The PV and current eval should when correspond to the best move
       when leaving */
    (g_state.board).pv_depth[0] = evaluated_list[0].pv_depth;
    i = 0;
    while i < ( g_state.board).pv_depth[0] {
        ( g_state.board).pv[0][i as usize] = evaluated_list[0].pv[i as usize];
        i += 1
    }
    ( g_state.search).set_current_eval(evaluated_list[0].eval);
    /* Don't forget to enable the time control mechanisms when leaving */
    ( g_state.timer).toggle_abort_check(1);
    ( g_state.midgame).toggle_midgame_abort_check(1);
    ( g_state.midgame).toggle_perturbation_usage(0);
}

/*
   COMPUTE_MOVE
   Returns the best move in a position given search parameters.
*/
pub fn legacy_compute_move(side_to_move: i32,
                           update_all: i32,
                           my_time: i32,
                           my_incr: i32,
                           timed_depth: i32,
                           book: i32,
                           mid: i32,
                           exact: i32,
                           wld: i32,
                           search_forced: i32,
                           eval_info: &mut EvaluationType, g_state: &mut FullState,
                           thor: &mut LegacyThor
)
                           -> i8 {
    return generic_compute_move::<LogFileHandler, LibcZebraOutput, LibcFatalError, LegacyThor>(side_to_move, update_all, my_time,
                                                                                               my_incr, timed_depth,
                                                                                               book, mid,
                                                                                               exact, wld,
                                                                                               search_forced, eval_info,
                                                                                               &mut LogFileHandler::create_log_file_if_needed(),
                                                                                               g_state.config.display_pv,
                                                                                               g_state.config.echo,
                                                                                               g_state, thor);
}

pub struct LibcZebraOutput;
impl ComputeMoveOutput for LibcZebraOutput {
fn display_out_optimal_line(search_state: &SearchState) {
    //FIXME parametrize, this touches global state
     { display_optimal_line(&mut stdout, search_state.full_pv_depth, &search_state.full_pv) }
}

fn send_move_type_0_status(interrupted_depth: i32, info: &EvaluationType, counter_value: f64, timer: &mut Timer, board_state: &BoardState) {
    unsafe {
        let ds = &mut display_state;
        ds.clear_status();
        send_status!(ds, "--> *{:2}", interrupted_depth);
        let mut eval_str = produce_eval_text(info, 1);
        send_status!(ds, "{:>10}  ", eval_str);
        ds.send_status_nodes(counter_value);
        ds.send_status_pv(&board_state.pv[0], interrupted_depth, board_state.pv_depth[0]);
        ds.send_status_time(timer.get_elapsed_time());
        if timer.get_elapsed_time() != 0.0f64 {
            send_status!(ds, "{:6.0} {}", counter_value / (timer.get_elapsed_time() + 0.001f64), "nps" );
        }
    }
}

fn display_status_out() {
    unsafe { display_state.display_status(&mut stdout, 0); }
}

fn echo_ponder_move_4(curr_move: i8, ponder_move: i8) {
   Self::echo_ponder_move_2(curr_move, ponder_move)
}

fn echo_ponder_move_2(curr_move: i8, ponder_move: i8) {
    unsafe {
        let ds = &mut display_state;
        send_status!(ds, "-->   {}        ", "Thor database");
        if ponder_move != 0 {
            send_status!(ds, "{{{}}} ", TO_SQUARE(ponder_move));
        }
        send_status!(ds, "{}", TO_SQUARE(curr_move));
        ds.display_status(&mut stdout, 0);
    }
}

fn echo_ponder_move(curr_move: i8, ponder_move: i8) {
    unsafe {
        let ds = &mut display_state;
        send_status!(ds, "-->   Forced opening move        ");
        if ponder_move != 0 {
            send_status!(ds, "{} ",TO_SQUARE(ponder_move));
        }
        send_status!(ds, "{}",TO_SQUARE(curr_move));
        ds.display_status(&mut stdout, 0);
    }
}

fn echo_compute_move_2(info: &EvaluationType, disk: i8) {
    unsafe {
        let ds = &mut display_state;
        let mut eval_str = produce_eval_text(info, 0);
        send_status!(ds, "-->         ");
        send_status!(ds, "{:<8}  ", eval_str);
        send_status!(ds, "{} ", TO_SQUARE(disk));
        ds.display_status(&mut stdout, 0);
    }
}

fn echo_compute_move_1(info: &EvaluationType) {
    unsafe {
        let ds = &mut display_state;
        let mut eval_str = produce_eval_text(info, 0);
        send_status!(ds, "-->         ");
        send_status!(ds, "{:<8}  ",             eval_str);
        ds.display_status(&mut stdout, 0);
    }
}
}
impl LogFileHandler {
    fn create() -> Option<Self> {
        if let Ok(log_file) = std::fs::OpenOptions::new().append(true).write(true).open("zebra.log")  {
            Some(LogFileHandler { log_file })
        } else {
            None
        }
    }
}
impl ComputeMoveLogger for LogFileHandler {

fn create_log_file_if_needed() -> Option<Self> {
    if use_log_file.load(SeqCst) != 0 {
        Self::create()
    } else {
        None
    }
}

fn log_moves_generated(logger: &mut LogFileHandler, moves_generated: i32, move_list_for_disks_played: &[i8; 64]) {
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
}

fn log_best_move(logger: &mut LogFileHandler, best_move: i8) {
    write!(&mut logger.log_file, "{}: {}{}  ({})\n",
           "Best move",
           char::from('a' as u8 + (best_move % 10) as u8 - 1),
           char::from('0' as u8 + (best_move / 10) as u8),
           "forced");
}

fn log_chosen_move(logger: &mut LogFileHandler, curr_move: i8, info: &EvaluationType) {
    let mut eval_str = produce_eval_text(info, 0);
    write!(logger.log_file, "{}: {}  {}\n", "Move chosen", TO_SQUARE(curr_move), eval_str);
}

fn log_status(logger: &mut LogFileHandler) {
    unsafe { display_state.display_status(&mut logger.log_file, 1); }
}

fn log_optimal_line(logger: &mut LogFileHandler, search_state: &SearchState) {
    display_optimal_line(&mut logger.log_file, search_state.full_pv_depth, &search_state.full_pv);
}

fn close_logger(logger: &mut LogFileHandler) {}

fn log_board(logger: &mut LogFileHandler, board_state: &BoardState, side_to_move_: i32) {
    unsafe {
        display_state.display_board(&mut logger.log_file, &board_state.board, side_to_move_,
                                    0, 0, 0,
                                    &board_state.black_moves, &board_state.white_moves,
        );
    }
}
}
