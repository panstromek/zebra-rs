use crate::src::zebra::EvaluationType;
use crate::src::hash::HashEntry;
use std::ffi::c_void;
use crate::src::timer::time_t;
use crate::src::game::CandidateMove;
use crate::src::counter::CounterType;

pub trait FrontEnd : FatalError {
    fn reset_buffer_display();
    fn display_buffers();
    fn after_update_best_list_verbose(best_list: &mut [i32; 4]);
    fn before_update_best_list_verbose(best_list: &mut [i32; 4], move_0: i32, best_list_index: i32, best_list_length: &mut i32);
    fn end_tree_search_output_some_second_stats(alpha: i32, beta: i32, curr_val: i32, update_pv: i32, move_index: i32);
    fn end_tree_search_some_pv_stats_report(alpha: i32, beta: i32, curr_val: i32);
    fn end_tree_search_level_0_ponder_0_short_report(move_0: i32, first: i32);
    fn end_tree_search_output_some_stats(entry: &HashEntry);
    fn end_tree_search_level_0_ponder_0_report(alpha: i32, beta: i32, result: i32, best_move_: i32);
    fn end_tree_search_level_0_report(alpha: i32, beta: i32);
    fn send_solve_status(empties: i32, side_to_move: i32, eval_info: &mut EvaluationType, counter: &mut CounterType, pv_zero: &mut [i32; 64], pv_depth_zero: i32);
    fn end_report_panic_abort_2(elapsed_time: f64);
    fn end_report_semi_panic_abort_3(elapsed_time: f64);
    fn end_report_semi_panic_abort_2(elapsed_time: f64);
    fn end_report_panic_abort(elapsed_time: f64);
    fn end_report_semi_panic_abort(elapsed_time: f64);
    fn end_display_zero_status();
    fn handle_fatal_pv_error(i: i32, pv_0_depth: i32, pv_0: &[i32; 64]);
    fn time(__timer: &mut time_t) -> time_t;
    unsafe fn strlen(_: *const i8) -> u64;
    fn tolower(num: i32) -> i32;
    unsafe fn strdup(_: *const i8) -> *mut i8;
    fn report_do_evaluate(evaluation_stage_: i32);
    fn report_unwanted_book_draw(this_move: i32);
    fn report_in_get_book_move_1(side_to_move: i32, remaining_slack: i32);
    fn report_in_get_book_move_2(chosen_score: i32, chosen_index: i32, flags: &i32, x: &[CandidateMove; 60]);
    fn midgame_display_simple_ponder_move(move_0: i32);
    fn midgame_display_initial_ponder_move(alpha: i32, beta: i32);
    fn midgame_display_ponder_move(
        max_depth: i32, alpha: i32, beta: i32,
        curr_val: i32, searched: i32, update_pv: i32);
    fn midgame_display_status(side_to_move: i32, max_depth: i32,
                              eval_info: &EvaluationType, depth: i32, force_return_: bool,
                              counter: &mut CounterType, pv_zero: &mut [i32; 64], pv_depth_zero: i32);
    fn report_mirror_symetry_error(count: i32, i: i32, first_mirror_offset: i32, first_item: i32, second_item: i32);
    fn thordb_report_flipped_0_first();
    fn thordb_report_flipped_0_second();
    fn choose_thor_opening_move_report(freq_sum: i32, match_count: i32, move_list: &[thordb_types::C2RustUnnamed; 64]);
    fn sort_thor_games(count: i32);
}
pub trait FatalError {
    fn invalid_move(curr_move: i32) -> !;
    fn unrecognized_character(unrecognized: i8) -> !;
    unsafe fn cannot_open_game_file(file_name: *const i8) -> !;
    fn memory_allocation_failure(block_count_: i32) -> !;
    fn invalid_move_in_move_sequence(curr_move: i32) -> !;
    fn error_in_map(i: i32, pos: i32, symmetry_map_item: i32) -> !;
    fn internal_error_in_book_code() -> !;
    fn error_in_map_thor(i: i32, pos: i32, to_report: i32) -> !;
    fn unexpected_character_in_a_move_string() -> !;
    fn invalid_move_string_provided() -> !;
}
