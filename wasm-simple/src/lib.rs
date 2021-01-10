#![allow(unused)]
#![allow(non_upper_case_globals)]

extern crate console_error_panic_hook;

use std::panic;
use wasm_bindgen::prelude::*;
use engine::src::zebra::{set_default_engine_globals, EvaluationType, engine_play_game, ZebraFrontend, InitialMoveSource, DumpHandler, engine_play_game_async, Config, INITIAL_CONFIG};
use engine::src::game::{engine_global_setup, global_terminate, BoardSource, FileBoardSource, ComputeMoveLogger, ComputeMoveOutput, CandidateMove};
use engine::src::error::{FrontEnd, FatalError};
use wasm_bindgen::__rt::core::ffi::c_void;
use engine::src::hash::HashEntry;
use wasm_bindgen::__rt::core::ptr::null_mut;
use engine::src::learn::Learner;
use wasm_bindgen::__rt::std::ffi::CStr;
use std::convert::{TryFrom, TryInto};
use engine::src::counter::CounterType;
use engine_traits::CoeffSource;
use flate2_coeff_source::Flate2Source;
use flip::unflip;
use engine::src::myrandom;
use std::error::Error;
use thiserror::Error;
use thordb_types::C2RustUnnamed;
use engine::src::thordb::ThorDatabase;

extern crate engine;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn js_time() -> f64;
    #[wasm_bindgen(js_namespace = zebra)]
    fn display_board(board: &[i32]);
    #[wasm_bindgen(catch)]
    pub async fn get_move_from_js(side_to_move: i32) -> Result<JsValue, JsValue>;
}

#[derive(Error, Debug)]
enum ZebraError {
    #[error("Move recieved from JS is not a number.")]
    MoveIsNotANumber,
    #[error("Move from JS not recieved")]
    MissingValue,
}

async fn get_move_from_wasm(side_to_move: i32) -> Result<i32, Box<dyn Error>> {
    Ok(get_move_from_js(side_to_move).await
        .map_err(|_e| ZebraError::MissingValue)?
        .as_f64()
        .ok_or(ZebraError::MoveIsNotANumber)? as i32
    )
}
macro_rules! c_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
static COEFFS: &[u8; 1336662] = include_bytes!("./../../coeffs2.bin");
static mut config : Config = INITIAL_CONFIG;

#[wasm_bindgen]
pub fn set_skills(
    black_skill: i32,
    black_exact_skill: i32,
    black_wld_skill: i32,
    white_skill: i32,
    white_exact_skill: i32,
    white_wld_skill: i32,
) {
    unsafe {
        // black
        config.skill[0] = black_skill;
        config.exact_skill[0] = black_exact_skill;
        config.wld_skill[0] = black_wld_skill;

        // white
        config.skill[2] = white_skill;
        config.exact_skill[2] = white_exact_skill;
        config.wld_skill[2] = white_wld_skill;
    }
}


#[wasm_bindgen]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    unsafe {
        set_default_engine_globals(&mut config);
        config.use_book = 0;
        let coeffs = Flate2Source::new_from_data(COEFFS);

        engine_global_setup::<_, WasmFrontend>(0, 18, None, coeffs);
        // init_thor_database::<WasmFrontend>();

        let x = 1 as i32;
        engine::src::zebra::random_instance.my_srandom(x);

        // FIXME don't run this init code on every start - my set_skills doesn't work because of that
        if config.skill[0] < 0 {
            config.skill[0] = 6;
            config.exact_skill[0] = 6;
            config.wld_skill[0] = 6;
        }
        if config.skill[2] < 0 {
            config.skill[2] = 0;
            config.exact_skill[2] = 0;
            config.wld_skill[2] = 0;
        }

    }
}
#[wasm_bindgen]
pub fn terminate() {
    // I never call this and it's probably pointless..., just putting it here so it is there
    unsafe { global_terminate(); }
}

#[wasm_bindgen]
pub async fn start_game() {
    unsafe {
        let repeat = 1;
        // let mut move_file = LibcFileMoveSource::open(move_file_name);

        engine_play_game_async::<
            WasmFrontend, WasmInitialMoveSource, WasmFrontend, WasmBoardSource,
            WasmComputeMoveLogger, WasmFrontend, WasmLearner, WasmFrontend, WasmThor,
            _, _
        >(null_mut(),
          null_mut(),
          repeat,
          null_mut(),
          None,
          false, false,
          get_move_from_wasm, &mut config).await;
    }
    c_log!("Zebra ended");
}

struct WasmThor;
impl ThorDatabase for WasmThor {
    fn choose_thor_opening_move_report(freq_sum: i32, match_count: i32, move_list: &[thordb_types::C2RustUnnamed; 64]) {
        unimplemented!()
    }

    fn get_thor_game_move(index: i32, move_number: i32) -> i32 {
        unimplemented!()
    }

    fn database_search(in_board: &[i32], side_to_move: i32) {
        unimplemented!()
    }

    fn get_match_count() -> i32 {
        unimplemented!()
    }

    fn get_black_win_count() -> i32 {
        unimplemented!()
    }

    fn get_draw_count() -> i32 {
        unimplemented!()
    }

    fn get_white_win_count() -> i32 {
        unimplemented!()
    }

    fn get_black_median_score() -> i32 {
        unimplemented!()
    }

    fn get_black_average_score() -> f64 {
        unimplemented!()
    }

    fn choose_thor_opening_move(in_board: &[i32], side_to_move: i32, echo: i32) -> i32 {
        unimplemented!()
    }
}
struct WasmLearner;

impl Learner for WasmLearner {
    fn learn_game(game_length: i32, private_game: i32, save_database: i32) {
        unimplemented!()
    }
}

impl ComputeMoveOutput for WasmFrontend {
    fn display_out_optimal_line() {
        c_log!("Display out optimal line")
    }

    fn send_move_type_0_status(interrupted_depth: i32, info: &EvaluationType, counter_value: f64, elapsed_time: f64) {
        unimplemented!()
    }

    fn display_status_out() {
        c_log!("todo display_status_out")
    }

    fn echo_ponder_move_4(curr_move: i32, ponder_move: i32) {
        unimplemented!()
    }

    fn echo_ponder_move_2(curr_move: i32, ponder_move: i32) {
        unimplemented!()
    }

    fn echo_ponder_move(curr_move: i32, ponder_move: i32) {
        unimplemented!()
    }

    fn echo_compute_move_2(info: &EvaluationType, disk: i32) {
        unimplemented!()
    }

    fn echo_compute_move_1(info: &EvaluationType) {
        unimplemented!()
    }
}

struct WasmComputeMoveLogger;

impl ComputeMoveLogger for WasmComputeMoveLogger {
    fn log_moves_generated(logger: &mut Self, moves_generated: i32, move_list_for_disks_played: &[i32; 64]) {
        unimplemented!()
    }

    fn log_best_move_pass(logger: &mut Self) {
        unimplemented!()
    }

    fn log_best_move(logger: &mut Self, best_move: i32) {
        unimplemented!()
    }

    fn log_chosen_move(logger: &mut Self, curr_move: i32, info: &EvaluationType) {
        unimplemented!()
    }

    fn log_status(logger: &mut Self) {
        unimplemented!()
    }

    fn log_optimal_line(logger: &mut Self) {
        unimplemented!()
    }

    fn close_logger(logger: &mut Self) {
        unimplemented!()
    }

    fn log_board(logger: &mut Self, board_: &[i32; 128], side_to_move_: i32) {
        unimplemented!()
    }

    fn create(log_file_path_: &mut [i8]) -> Option<Self> where Self: Sized {
        None
    }

    fn create_log_file_if_needed() -> Option<Self> where Self: Sized {
        None
    }
}

struct WasmBoardSource;

impl FileBoardSource for WasmBoardSource {
    fn open(file_name: &CStr) -> Option<Self> where Self: Sized {
//        todo??
        None
    }
}

impl BoardSource for WasmBoardSource {
    fn fill_board_buffer(&mut self, buffer: &mut [i8; 70]) {
        buffer[0] = 0
    }

    fn fill_buffer_with_side_to_move(&mut self, buffer: &mut [i8; 70]) {
        unimplemented!()
    }

    fn report_unrecognized_character(unrecognized: i8) {
        unimplemented!()
    }
}

struct WasmInitialMoveSource;

impl InitialMoveSource for WasmInitialMoveSource {
    fn fill_line_buffer(&mut self, line_buffer: &mut [u8]) {
        line_buffer[0] = 0;
    }
}

struct WasmFrontend;

impl DumpHandler for WasmFrontend {
    fn dump_position(side_to_move: i32, board_: &[i32; 128]) {
        c_log!("dump position called")
        // this probably isn't needed in web, in original it stores a position in a file
    }

    fn dump_game_score(side_to_move: i32, score_sheet_row_: i32, black_moves_: &[i32; 60], white_moves_: &[i32; 60]) {
        c_log!("dump_game_score")
    }
}

impl ZebraFrontend for WasmFrontend {
    fn set_evals(black: f64, white: f64) {
        // unimplemented!()
    }

    fn set_move_list(row: i32) {
        // unimplemented!()
    }

    fn set_names(white_is_player: bool, black_is_player: bool) {
        // unimplemented!()
    }

    fn set_times(black: i32, white: i32) {
        // unimplemented!()
    }

    fn report_some_thor_scores(black_win_count: i32, draw_count: i32, white_win_count: i32, black_median_score: i32, black_average_score: f64) {
        unimplemented!()
    }

    fn report_some_thor_stats(total_search_time: f64, thor_position_count: i32, db_search_time: f64) {
        unimplemented!()
    }

    fn display_board_after_thor(side_to_move: i32, give_time_: i32, board_: &[i32; 128], black_moves_: &[i32; 60], white_moves_: &[i32; 60]) {
        display_board(board_);

        // unimplemented!()
    }

    fn print_out_thor_matches(thor_max_games_: i32) {
        unimplemented!()
    }

    fn log_game_ending(log_file_name_: &CStr, move_vec: &[i8; 121], first_side_to_move: i32, second_side_to_move: i32) {
        unimplemented!()
    }

    fn get_pass() {
        unimplemented!()
    }

    fn report_engine_override() {
        unimplemented!()
    }

    fn prompt_get_move(side_to_move: i32, buffer: &mut [i8; 255]) -> i32 {
        unimplemented!()
    }

    fn before_get_move() {
        // this function is kinda nonsense in the original
    }

    fn report_after_game_ended(node_val: f64, eval_val: f64, black_disc_count: i32, white_disc_count: i32, total_time_: f64) {
        c_log!("\nBlack: {}   White: {}", black_disc_count, white_disc_count);
        c_log!("Nodes searched:        {}", node_val);
        c_log!("Positions evaluated:   {}", eval_val);
        c_log!("Total time: {} s", total_time_);
    }

    fn report_skill_levels(black_level: i32, white_level: i32) {
        c_log!("\n");
        c_log!("Black level: {}\n", black_level);
        c_log!("White level: {}\n", white_level);
    }

    fn report_thor_matching_games_stats(total_search_time: f64, thor_position_count: i32, database_time: f64) {
        c_log!("{} matching games  ({} s search time, {} s total)\n",
                       thor_position_count, database_time, total_search_time);
    }

    fn report_thor_stats(black_win_count: i32, draw_count: i32, white_win_count: i32, black_median_score: i32, black_average_score: f64) {
        c_log!("{} black wins, {} draws, {} white wins\n",
               black_win_count, draw_count,
               white_win_count);
        c_log!("Median score {}-{}",
               black_median_score,
               64 as i32 -                   black_median_score);
        c_log!(", average score {}-{}\n",
               black_average_score,
               64.0f64 - black_average_score);
    }

    fn report_opening_name(opening_name: &CStr) {
        c_log!("\nOpening: {}\n", opening_name.to_str().unwrap() );
    }

    fn report_book_randomness(slack_: f64) {
        c_log!("Book randomness: {} disks\n", slack_);
    }

    fn load_thor_files() {
        c_log!("load_thor_files - ignored \n");
    }

    fn print_move_alternatives(side_to_move: i32) {
        unimplemented!()
    }

    fn dumpch() {
        unimplemented!()
    }
}
// #define TO_SQUARE(index)        'a'+(index % 10)-1,'0'+(index / 10)
macro_rules! to_square {
    ($index:expr) => {
       ( ('a' as u8 +($index as u8 % 10 as u8) - 1 as u8) as char, ('0' as u8 +($index as u8 / 10 as u8)) as char )
    };
}

impl FrontEnd for WasmFrontend {
    fn reset_buffer_display() {}

    fn display_buffers() {
        c_log!("Display Buffers called - not sure what I am supposed to show here")
        // unimplemented!()
    }

    fn after_update_best_list_verbose(best_list: &mut [i32; 4]) {
        unimplemented!()
    }

    fn before_update_best_list_verbose(best_list: &mut [i32; 4], move_0: i32, best_list_index: i32, best_list_length: &mut i32) {
        unimplemented!()
    }

    fn end_tree_search_output_some_second_stats(alpha: i32, beta: i32, curr_val: i32, update_pv: i32, move_index: i32) {
        unimplemented!()
    }

    fn end_tree_search_some_pv_stats_report(alpha: i32, beta: i32, curr_val: i32) {
        unimplemented!()
    }

    fn end_tree_search_level_0_ponder_0_short_report(move_0: i32, first: i32) {
        unimplemented!()
    }

    fn end_tree_search_output_some_stats(entry: &HashEntry) {
        unimplemented!()
    }

    fn end_tree_search_level_0_ponder_0_report(alpha: i32, beta: i32, result: i32, best_move_: i32) {
        c_log!("end_tree_search_level_0_ponder_0_report TODO")
    }

    fn end_tree_search_level_0_report(alpha: i32, beta: i32) {
        c_log!("[{},{}]", alpha, beta);
    }

    fn send_solve_status(empties: i32, side_to_move: i32, eval_info: &mut EvaluationType, counter: &mut CounterType, pv_zero: &mut [i32; 64], pv_depth_zero: i32) {
        c_log!("TODO send_solve_status")
    }

    fn end_report_panic_abort_2(elapsed_time: f64) {
        unimplemented!()
    }

    fn end_report_semi_panic_abort_3(elapsed_time: f64) {
        unimplemented!()
    }

    fn end_report_semi_panic_abort_2(elapsed_time: f64) {
        unimplemented!()
    }

    fn end_report_panic_abort(elapsed_time: f64) {
        unimplemented!()
    }

    fn end_report_semi_panic_abort(elapsed_time: f64) {
        unimplemented!()
    }

    fn end_display_zero_status() {
        c_log!("end_display_zero_status")
    }

    fn handle_fatal_pv_error(i: i32, pv_0_depth: i32, pv_0: &[i32; 64]) {
        unimplemented!()
    }

    fn time(__timer: &mut i64) -> i64 {
        let time = js_time().round() as i64;
        *__timer = time;
        return time;
    }

    fn tolower(num: i32) -> i32 {
        // if num >= 'A' as i32 && num <= 'Z' as i32 {
        //     let offset = ('a' as i32) - ('A' as i32);
        //     return num - offset;
        // }
        // return num;
        (char::from(num as u8).to_ascii_lowercase()) as i32
    }

    fn report_do_evaluate(evaluation_stage_: i32) {
        unimplemented!()
    }

    fn report_unwanted_book_draw(this_move: i32) {
        unimplemented!()
    }

    fn report_in_get_book_move_1(side_to_move: i32, remaining_slack: i32) {
        unimplemented!()
    }

    fn report_in_get_book_move_2(chosen_score: i32, chosen_index: i32, flags: &i32, x: &[CandidateMove; 60]) {
        unimplemented!()
    }

    fn midgame_display_simple_ponder_move(move_0: i32) {
        c_log!("{}{}", ('a' as u8 + move_0 as u8 % 10 - 1) as char,
                   ('0' as u8 + move_0 as u8 / 10) as char);
    }

    //fixme remove this fricking buffer param
    fn midgame_display_initial_ponder_move(alpha: i32, beta: i32) {
        c_log!("pondering move [{},{}] ", alpha, beta)
    }

    fn midgame_display_ponder_move(max_depth: i32, alpha: i32, beta: i32, curr_val: i32, searched: i32, update_pv: i32) {
        c_log!("TODO midgame_display_ponder_move")
    }

    fn midgame_display_status(side_to_move: i32, max_depth: i32, eval_info: &EvaluationType, depth: i32, force_return_: bool, counter: &mut CounterType, pv_zero: &mut [i32; 64], pv_depth_zero: i32) {
        // unimplemented!()
        c_log!("TODO midgame_display_status")
    }

    fn report_mirror_symetry_error(count: i32, i: i32, first_mirror_offset: i32, first_item: i32, second_item: i32) {
        unimplemented!()
    }

    fn thordb_report_flipped_0_first() {
        unimplemented!()
    }

    fn thordb_report_flipped_0_second() {
        unimplemented!()
    }

    fn choose_thor_opening_move_report(freq_sum: i32, match_count: i32, move_list: &[C2RustUnnamed; 64]) {
        c_log!("Thor database:");
        let mut i = 0;
        while i < match_count {
            c_log!("{}{}: {}",
                   ('a' as i32 + move_list[i as usize].move_0 % 10 - 1) as u8 as char,
                   ('0' as i32 + move_list[i as usize].move_0 / 10 as i32) as u8 as char,
                   100.0f64 *move_list[i as usize].frequency as f64 / freq_sum as f64);
            if i % 6 as i32 == 4 as i32 {
                c_log!("");
            }
            i += 1
        }
        if match_count % 6 as i32 != 5 as i32 {
            c_log!("");
        }
    }

    fn sort_thor_games(count: i32) {
        unimplemented!()
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

    fn memory_allocation_failure(block_count_: i32) -> ! {
        unimplemented!()
    }

    fn invalid_move_in_move_sequence(curr_move: i32) -> ! {
        unimplemented!()
    }

    fn error_in_map(i: i32, pos: i32, symmetry_map_item: i32) -> ! {
        unimplemented!()
    }

    fn internal_error_in_book_code() -> ! {
        unimplemented!()
    }

    fn error_in_map_thor(i: i32, pos: i32, to_report: i32) -> ! {
        unimplemented!()
    }

    fn unexpected_character_in_a_move_string() -> ! {
        unimplemented!()
    }

    fn invalid_move_string_provided() -> ! {
        unimplemented!()
    }
}
