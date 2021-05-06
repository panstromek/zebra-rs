#![allow(unused)]
#![allow(non_upper_case_globals)]

extern crate console_error_panic_hook;

use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::panic;

use thiserror::Error;
use wasm_bindgen::__rt::core::ffi::c_void;
use wasm_bindgen::__rt::core::ptr::null_mut;
use wasm_bindgen::__rt::std::ffi::CStr;
use wasm_bindgen::prelude::*;

use engine::src::counter::CounterType;
use engine::src::error::{FatalError, FrontEnd};
use engine::src::game::{BoardSource, CandidateMove, ComputeMoveLogger, ComputeMoveOutput, engine_global_setup, FileBoardSource, GameState};
use engine::src::hash::{HashEntry, HashState};
use engine::src::learn::{Learner, LearnState};
use engine::src::myrandom;
use engine::src::thordb::ThorDatabase;
use engine::src::zebra::{Config, EvaluationType, INITIAL_CONFIG, InitialMoveSource, set_default_engine_globals, ZebraFrontend, FullState, PlayGame, next_state, PlayGameState, MoveAttempt};
use engine_traits::CoeffSource;
use flate2_coeff_source::Flate2Source;
use flip::unflip;
use thordb_types::C2RustUnnamed;
use flip::unflip::FlipStack;
use engine::src::search::{SearchState, disc_count};
use engine::src::probcut::ProbCut;
use engine::src::osfbook::Book;
use engine::src::myrandom::MyRandom;
use engine::src::globals::BoardState;
use engine::src::stable::StableState;
use engine::src::moves::{MovesState, unmake_move, generate_all};
use engine::src::timer::{Timer, TimeSource};
use engine::src::getcoeff::CoeffState;
use engine::src::end::End;
use engine::src::midgame::MidgameState;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn js_time() -> f64;
    #[wasm_bindgen(js_namespace = zebra)]
    fn display_board(board: &[i32]);
}

macro_rules! c_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
static COEFFS: &[u8; 1336662] = include_bytes!("./../../../coeffs2.bin");


#[wasm_bindgen]
pub fn initialize() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

pub struct JsTimeSource;

impl TimeSource for JsTimeSource {
    fn time(&self, __timer: &mut i64) -> i64 {
        let time = js_time() as i64;
        *__timer = time;
        return time;
    }
}

static time_source: JsTimeSource = JsTimeSource {};

#[wasm_bindgen]
pub struct ZebraGame {
    game: Box<PlayGame<WasmInitialMoveSource>>
}

#[wasm_bindgen]
impl ZebraGame {
    #[wasm_bindgen]
    pub fn new() -> Self {
        let coeffs = Flate2Source::new_from_data(COEFFS);
        //
        let mut zebra = ZebraGame {
            game: Box::new(PlayGame::new(None, Vec::new(), 1, None,
                                         (FullState::new(&JsTimeSource))))
        };

        let state = &mut zebra.game.g_state;

        engine_global_setup::<_, WasmFrontend>(0, 18, None, coeffs,
                                               &mut state.search_state
                                               , &mut state.hash_state
                                               , &mut state.g_timer
                                               , &mut state.coeff_state
                                               , &mut state.random_instance
                                               , &mut state.stable_state
                                               , &mut state.prob_cut);

        set_default_engine_globals(&mut state.g_config);
        state.g_config.use_book = 0;
        state.g_config.use_thor = false;

        // // init_thor_database::<WasmFrontend>();
        //
        let x = 1 as i32;
        state.random_instance.my_srandom(x);
        if state.g_config.skill[0] < 0 {
            state.g_config.skill[0] = 6;
            state.g_config.exact_skill[0] = 6;
            state.g_config.wld_skill[0] = 6;
        }
        if state.g_config.skill[2] < 0 {
            state.g_config.skill[2] = 0;
            state.g_config.exact_skill[2] = 0;
            state.g_config.wld_skill[2] = 0;
        }
        return zebra;
    }

    #[wasm_bindgen]
    pub fn play_until_next_interaction(&mut self, move_attempt: Option<i32>) -> InteractionRequest {
        let mut move_attempt = move_attempt
            .map(|num| MoveAttempt(num, num));

        loop {
            match self.next_state(move_attempt.take()) {
                None => { continue; }
                Some(interaction) => return interaction
            }
        }
    }

    #[wasm_bindgen]
    pub fn side_to_move(&self) -> i32 {
        self.game.side_to_move
    }

    #[wasm_bindgen]
    pub fn undo(&mut self) -> Option<i32> {
        // let end = self.game.move_vec.iter()
        //     .position(|m| m == &0)?;
        // if end < 2 {
        //     return None;
        // }
        // let first = *self.game.move_vec.get(end - 2)?;
        // let second = *self.game.move_vec.get(end - 1)?;
        //
        // let move_0 = first as i32 - 'a' as i32 + 1 + 10 * (second as i32 - '0' as i32);
        //
        // let side_to_move = *self.game.g_state.board_state.board.get(move_0 as usize)?;
        //
        // unmake_move(side_to_move, move_0,
        //             &mut self.game.g_state.board_state.board,
        //             &mut self.game.g_state.moves_state,
        //             &mut self.game.g_state.hash_state,
        //             &mut self.game.g_state.flip_stack_);
        //
        // self.game.move_vec[end - 2] = 0;
        // self.game.move_vec[end - 1] = 0;
        // if self.game.g_state.board_state.score_sheet_row > 0 {
        //     self.game.g_state.board_state.score_sheet_row -= 1;
        //     let score_sheet_row = self.game.g_state.board_state.score_sheet_row;
        //     if side_to_move == 2 {
        //         self.game.g_state.board_state.white_moves[score_sheet_row as usize] = -1;
        //     } else {
        //         self.game.g_state.board_state.black_moves[score_sheet_row as usize] = -1;
        //     }
        // }
        // self.game.side_to_move = side_to_move;
        // display_board(&self.game.g_state.board_state.board);

        // Ported from from droidzebra/reversatile C code
        let mut human_can_move = false;
        let mut curr_move;
        const BLACKSQ: i32 = 0;
        const WHITESQ: i32 = 2;
        const PASS: i32 = -1;
        #[allow(non_snake_case)]
        fn OPP(color: i32) -> i32 {
            ((BLACKSQ + WHITESQ) - (color))
        }
        let side_to_move = &mut self.game.side_to_move;
        let score_sheet_row = &mut self.game.g_state.board_state.score_sheet_row;

        if *score_sheet_row == 0 && *side_to_move == BLACKSQ {
            generate_all(*side_to_move,
                         &mut self.game.g_state.moves_state,
                         &mut self.game.g_state.search_state,
                         &mut self.game.g_state.board_state.board);
            display_board(&self.game.g_state.board_state.board);

            return None;
        }
        // TODO setting
        let auto_make_forced_moves = false;

        // _droidzebra_undo_stack_push(disks_played);
        let mut white_moves = &mut self.game.g_state.board_state.white_moves;
        let mut black_moves = &mut self.game.g_state.board_state.black_moves;
        loop {
            *side_to_move = OPP(*side_to_move);

            if *side_to_move == WHITESQ {
                *score_sheet_row -= 1;
            }
            let score_sheet_row = *score_sheet_row;
            human_can_move =
                self.game.g_state.g_config.skill[(*side_to_move) as usize] == 0 &&
                    !(
                        (auto_make_forced_moves && self.game.g_state.moves_state.move_count[self.game.g_state.moves_state.disks_played as usize - 1] == 1)
                            || (*side_to_move==WHITESQ && white_moves[score_sheet_row as usize]==PASS)
                            || (*side_to_move==BLACKSQ && black_moves[score_sheet_row as usize]==PASS)
                    );

            if *side_to_move == WHITESQ {
                curr_move = white_moves[score_sheet_row as usize];
                if white_moves[score_sheet_row as usize]!=PASS {
                    unmake_move(WHITESQ,
                                white_moves[score_sheet_row as usize],
                                &mut self.game.g_state.board_state.board,
                                &mut self.game.g_state.moves_state,
                                &mut self.game.g_state.hash_state,
                                &mut self.game.g_state.flip_stack_
                    );

                }
                white_moves[score_sheet_row as usize] = PASS;
            } else {
                curr_move = black_moves[score_sheet_row as usize];
                if black_moves[score_sheet_row as usize] != PASS {
                    unmake_move(BLACKSQ, black_moves[score_sheet_row as usize],
                                &mut self.game.g_state.board_state.board,
                                &mut self.game.g_state.moves_state,
                                &mut self.game.g_state.hash_state,
                                &mut self.game.g_state.flip_stack_);
                }
                black_moves[score_sheet_row as usize] = PASS;
            }
            if !(!(score_sheet_row == 0 && *side_to_move == BLACKSQ) && !human_can_move) {
                break;
            }
        }
        match self.game.state {
            PlayGameState::GettingMove { provided_move_count, move_start, .. } => {
                self.game.state = PlayGameState::GettingMove {
                    provided_move_count, move_start, side_to_move: *side_to_move
                }
            }
            _ => {}
        }

        generate_all(*side_to_move,
                     &mut self.game.g_state.moves_state,
                     &mut self.game.g_state.search_state,
                     &mut self.game.g_state.board_state.board);
        // set_move_list?
        // TODO find and display opening name?
        display_board(&self.game.g_state.board_state.board);

        // Where does this fn + field come from?
        // It wasn't in the original C code but it's in the Android C code
        // clear_endgame_performed();
        Some(1)
    }

    #[wasm_bindgen]
    pub fn get_board(&self) -> Box<[i32]> {
        self.game.g_state.board_state.board.into()
    }
    #[wasm_bindgen]
    pub fn set_skills(
        &mut self,
        black_skill: i32,
        black_exact_skill: i32,
        black_wld_skill: i32,
        white_skill: i32,
        white_exact_skill: i32,
        white_wld_skill: i32,
    ) {
        let state = &mut self.game.g_state;
        state.g_config.skill[0] = black_skill;
        state.g_config.exact_skill[0] = black_exact_skill;
        state.g_config.wld_skill[0] = black_wld_skill;
        state.g_config.skill[2] = white_skill;
        state.g_config.exact_skill[2] = white_exact_skill;
        state.g_config.wld_skill[2] = white_wld_skill;
    }
}

impl ZebraGame {
    pub fn next_state(&mut self, mut move_attempt: Option<MoveAttempt>) -> Option<InteractionRequest> {
        let mut play_state = &mut self.game;
        let state = next_state::<
            WasmFrontend, WasmInitialMoveSource, WasmBoardSource, WasmComputeMoveLogger, WasmFrontend, WasmFrontend, WasmThor
        >(&mut play_state, move_attempt.take());
        match state {
            PlayGameState::GetPass { provided_move_count } => {
                // TODO signal this to frontend
                display_board(&play_state.g_state.board_state.board);

                return Some(InteractionRequest::Pass);
            }
            PlayGameState::GettingMove { provided_move_count, move_start, side_to_move } => {
                display_board(&play_state.g_state.board_state.board);

                // TODO signal that we need move
                // move_attempt =  Some(MoveAttempt(res.0, res.1))
                return Some(InteractionRequest::Move);
            }
            PlayGameState::AfterGameReport { node_val, eval_val } => {
                display_board(&play_state.g_state.board_state.board);

                // TODO report game score?
                // TODO display
                let black_disc_count = disc_count(0, &play_state.g_state.board_state.board);
                let white_disc_count = disc_count(2, &play_state.g_state.board_state.board);
                let total_time_ = play_state.g_state.search_state.total_time;
                report_after_game_ended(node_val, eval_val, black_disc_count, white_disc_count, total_time_);
            }
            PlayGameState::End => {
                display_board(&play_state.g_state.board_state.board);
                return Some(InteractionRequest::End);
            }
            PlayGameState::NeedsDump { .. } => {
                display_board(&play_state.g_state.board_state.board);
            }
            _ => {}
        };
        None
    }
}

#[wasm_bindgen]
pub enum InteractionRequest {
    Move,
    Pass,
    End,
}

struct WasmThor;

impl ThorDatabase for WasmThor {
    fn choose_thor_opening_move_report(freq_sum: i32, match_count: i32, move_list: &[thordb_types::C2RustUnnamed; 64]) {

    }

    fn get_thor_game_move(index: i32, move_number: i32) -> i32 {
        0
    }

    fn database_search(in_board: &[i32], side_to_move: i32) {
        //
    }

    fn get_match_count() -> i32 {
        //
        0
    }

    fn get_black_win_count() -> i32 {
        0
    }

    fn get_draw_count() -> i32 {
        0
    }

    fn get_white_win_count() -> i32 {
        0
    }

    fn get_black_median_score() -> i32 {
        0
    }

    fn get_black_average_score() -> f64 {
        0.0
    }

    fn choose_thor_opening_move(in_board: &[i32], side_to_move: i32, echo: i32, random: &mut MyRandom) -> i32 {
        0
    }
}

struct WasmLearner;

impl ComputeMoveOutput for WasmFrontend {
    fn display_out_optimal_line(search_state: &SearchState) {
        // c_log!("Display out optimal line")
    }

    fn send_move_type_0_status(interrupted_depth: i32, info: &EvaluationType, counter_value: f64, timer: &mut Timer, board_state: &BoardState) {

    }

    fn display_status_out() {
        // c_log!("todo display_status_out")
    }

    fn echo_ponder_move_4(curr_move: i32, ponder_move: i32) {

    }

    fn echo_ponder_move_2(curr_move: i32, ponder_move: i32) {

    }

    fn echo_ponder_move(curr_move: i32, ponder_move: i32) {

    }

    fn echo_compute_move_2(info: &EvaluationType, disk: i32) {

    }

    fn echo_compute_move_1(info: &EvaluationType) {

    }
}

struct WasmComputeMoveLogger;

impl ComputeMoveLogger for WasmComputeMoveLogger {
    fn log_moves_generated(logger: &mut Self, moves_generated: i32, move_list_for_disks_played: &[i32; 64]) {
        //
    }

    fn log_best_move_pass(logger: &mut Self) {
        //
    }

    fn log_best_move(logger: &mut Self, best_move: i32) {
        //
    }

    fn log_chosen_move(logger: &mut Self, curr_move: i32, info: &EvaluationType) {
        //
    }

    fn log_status(logger: &mut Self) {
        //
    }

    fn log_optimal_line(logger: &mut Self, search_state: &SearchState) {
        //
    }

    fn close_logger(logger: &mut Self) {
        //
    }

    fn log_board(logger: &mut Self, board_: &BoardState, side_to_move_: i32) {
        //
    }

    fn create_log_file_if_needed() -> Option<Self> where Self: Sized {
        None
    }
}

struct WasmInitialMoveSource;

impl InitialMoveSource for WasmInitialMoveSource {
    fn fill_line_buffer(&mut self, line_buffer: &mut [u8]) {
        line_buffer[0] = 0;
    }
}

fn report_after_game_ended(node_val: f64, eval_val: f64, black_disc_count: i32, white_disc_count: i32, total_time_: f64) {
    c_log!("\nBlack: {}   White: {}", black_disc_count, white_disc_count);
    c_log!("Nodes searched:        {}", node_val);
    c_log!("Positions evaluated:   {}", eval_val);
    c_log!("Total time: {} s", total_time_);
}

struct WasmFrontend;

impl ZebraFrontend for WasmFrontend {
    fn set_evals(black: f64, white: f64) {
        //
    }

    fn set_move_list(row: i32) {
        //
    }

    fn set_names(white_is_player: bool, black_is_player: bool) {
        //
    }

    fn report_engine_override() {
        c_log!("Engine override")
    }

    fn before_get_move() {
        // this function is kinda nonsense in the original
    }
    //
    //
    // fn report_skill_levels(black_level: i32, white_level: i32) {
    //     c_log!("\n");
    //     c_log!("Black level: {}\n", black_level);
    //     c_log!("White level: {}\n", white_level);
    // }
    //
    // fn report_thor_matching_games_stats(total_search_time: f64, thor_position_count: i32, database_time: f64) {
    //     c_log!("{} matching games  ({} s search time, {} s total)\n",
    //                    thor_position_count, database_time, total_search_time);
    // }
    //
    // fn report_thor_stats(black_win_count: i32, draw_count: i32, white_win_count: i32, black_median_score: i32, black_average_score: f64) {
    //     c_log!("{} black wins, {} draws, {} white wins\n",
    //            black_win_count, draw_count,
    //            white_win_count);
    //     c_log!("Median score {}-{}",
    //            black_median_score,
    //            64 as i32 -                   black_median_score);
    //     c_log!(", average score {}-{}\n",
    //            black_average_score,
    //            64.0f64 - black_average_score);
    // }
    //
    // fn report_opening_name(opening_name: &CStr) {
    //     c_log!("\nOpening: {}\n", opening_name.to_str().unwrap() );
    // }

    fn report_book_randomness(slack_: f64) {
        c_log!("Book randomness: {} disks\n", slack_);
    }

    fn load_thor_files(g_timer: &mut Timer) {

    }

    fn print_move_alternatives(side_to_move: i32, board_state: &mut BoardState, g_book: &mut Book) {

    }
}
// #define TO_SQUARE(index)        'a'+(index % 10)-1,'0'+(index / 10)
macro_rules! to_square {
    ($index:expr) => {
       ( ('a' as u8 +($index as u8 % 10 as u8) - 1 as u8) as char, ('0' as u8 +($index as u8 / 10 as u8)) as char )
    };
}
const LOG_PONDER_MOVE: bool = false;

impl FrontEnd for WasmFrontend {
    fn reset_buffer_display(g_timer: &mut Timer) {
        //
    }

    fn display_buffers(g_timer: &mut Timer) {
        // TODO?
    }

    fn after_update_best_list_verbose(best_list: &mut [i32; 4]) {}

    fn before_update_best_list_verbose(best_list: &mut [i32; 4], move_0: i32, best_list_index: i32, best_list_length: &mut i32) {}

    fn end_tree_search_output_some_second_stats(alpha: i32, beta: i32, curr_val: i32, update_pv: i32, move_index: i32, echo: i32) {}


    fn end_tree_search_some_pv_stats_report(alpha: i32, beta: i32, curr_val: i32) {
    }

    fn end_tree_search_level_0_ponder_0_short_report(move_0: i32, first: i32) {}

    fn end_tree_search_output_some_stats(entry: &HashEntry) {}

    fn end_tree_search_level_0_ponder_0_report(alpha: i32, beta: i32, result: i32, best_move_: i32) {}

    fn end_tree_search_level_0_report(alpha: i32, beta: i32) {}

    fn send_solve_status(empties: i32, _side_to_move: i32, eval_info: &mut EvaluationType, pv_zero: &mut [i32; 64], pv_depth_zero: i32, g_timer: &mut Timer, search_state: &mut SearchState) {
        // c_log!("Solve status TODO")
    }

    fn end_report_panic_abort_2(elapsed_time: f64) {

    }

    fn end_report_semi_panic_abort_3(elapsed_time: f64) {

    }

    fn end_report_semi_panic_abort_2(elapsed_time: f64) {

    }

    fn end_report_panic_abort(elapsed_time: f64) {

    }

    fn end_report_semi_panic_abort(elapsed_time: f64) {

    }

    fn end_display_zero_status() {
        // c_log!("end_display_zero_status")
    }

    fn handle_fatal_pv_error(i: i32, pv_0_depth: i32, pv_0: &[i32; 64]) {
        panic!("FATAL PV ERROR");
    }

    fn report_unwanted_book_draw(this_move: i32) {

    }

    fn report_in_get_book_move_1(side_to_move: i32, remaining_slack: i32, board_state: &mut BoardState, g_book: &mut Book) {

    }

    fn report_in_get_book_move_2(chosen_score: i32, chosen_index: i32, flags: &i32, x: &[CandidateMove; 60], search_state: &SearchState) {

    }


    fn midgame_display_simple_ponder_move(move_0: i32) {
        // Maybe I'm gonna need this??
        if LOG_PONDER_MOVE {
            c_log!("{}{}", ('a' as u8 + move_0 as u8 % 10 - 1) as char,
                   ('0' as u8 + move_0 as u8 / 10) as char);
        }
    }

    fn midgame_display_initial_ponder_move(alpha: i32, beta: i32) {
        if LOG_PONDER_MOVE {
            c_log!("pondering move [{},{}] ", alpha, beta)
        }
    }

    fn midgame_display_ponder_move(max_depth: i32, alpha: i32, beta: i32, curr_val: i32, searched: i32, update_pv: i32, echo: i32) {
        //
    }

    fn midgame_display_status(side_to_move: i32, max_depth: i32, eval_info: &EvaluationType, depth: i32, force_return_: bool, g_timer: &mut Timer, moves_state: &mut MovesState, board_state: &mut BoardState, hash_state: &mut HashState, search_state: &mut SearchState, flip_stack_: &mut FlipStack) {
        //
    }

    fn report_mirror_symetry_error(count: i32, i: i32, first_mirror_offset: i32, first_item: i32, second_item: i32) {
        //
    }
}

impl FatalError for WasmFrontend {
    fn invalid_move(curr_move: i32) -> ! {
        unimplemented!()
    }

    fn unrecognized_character(unrecognized: i8) -> ! {
        unimplemented!()
    }

    fn cannot_open_game_file(file_name: &str) -> ! {
        unimplemented!()
    }

    fn invalid_move_in_move_sequence(curr_move: i32) -> ! {
        unimplemented!()
    }

    fn internal_error_in_book_code() -> ! {
        unimplemented!()
    }

    fn unexpected_character_in_a_move_string() -> ! {
        unimplemented!()
    }

    fn invalid_move_string_provided() -> ! {
        unimplemented!()
    }

    fn initial_squares_are_empty() -> ! { unimplemented!() }
}

struct WasmBoardSource;

impl FileBoardSource for WasmBoardSource {
    fn open(file_name: &CStr) -> Option<Self> where Self: Sized {
//        todo??
        None
    }
}

impl BoardSource for WasmBoardSource {
    fn fill_board_buffer(&mut self, buffer: &mut String) {
        unimplemented!()
    }

    fn fill_buffer_with_side_to_move(&mut self, buffer: &mut Vec<u8>) {
        unimplemented!()
    }

    fn report_unrecognized_character(unrecognized: i8) {
        unimplemented!()
    }
}

