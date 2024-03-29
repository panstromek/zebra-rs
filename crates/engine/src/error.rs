use crate::src::zebra::{EvaluationType, Config};
use crate::src::hash::{HashEntry, HashState};
use std::ffi::c_void;
use crate::src::timer::{time_t, Timer};
use crate::src::game::CandidateMove;
use crate::src::counter::CounterType;
use crate::src::search::SearchState;
use crate::src::osfbook::Book;
use crate::src::globals::BoardState;
use crate::src::moves::MovesState;
use flip::unflip::FlipStack;

pub trait FrontEnd : FatalError {
    fn reset_buffer_display(g_timer:&Timer);
    fn display_buffers(g_timer: &Timer);
    fn after_update_best_list_verbose(best_list: &[i8; 4]);
    fn before_update_best_list_verbose(best_list: &[i8; 4], move_0: i8, best_list_index: i32, best_list_length: i32);
    fn end_tree_search_output_some_second_stats(alpha: i32, beta: i32, curr_val: i32, update_pv: i32, move_index: i32, echo: i32);
    fn end_tree_search_some_pv_stats_report(alpha: i32, beta: i32, curr_val: i32);
    fn end_tree_search_level_0_ponder_0_short_report(move_0: i8, first: i32);
    fn end_tree_search_output_some_stats(entry: &HashEntry);
    fn end_tree_search_level_0_ponder_0_report(alpha: i32, beta: i32, result: i32, best_move_: i32);
    fn end_tree_search_level_0_report(alpha: i32, beta: i32);
    fn send_solve_status(empties: i32, _side_to_move: i32, eval_info: &EvaluationType,
                         pv_zero: &mut [i8; 64],
                         pv_depth_zero: i32,
                         g_timer: &Timer,
                         search_state: &mut SearchState);
    fn end_report_panic_abort_2(elapsed_time: f64);
    fn end_report_semi_panic_abort_3(elapsed_time: f64);
    fn end_report_semi_panic_abort_2(elapsed_time: f64);
    fn end_report_panic_abort(elapsed_time: f64);
    fn end_report_semi_panic_abort(elapsed_time: f64);
    fn end_display_zero_status();
    fn handle_fatal_pv_error(i: i32, pv_0_depth: i32, pv_0: &[i8; 64]);
    fn report_unwanted_book_draw(this_move: i32);
    fn report_in_get_book_move_1(side_to_move: i32, remaining_slack: i32, board_state: &mut BoardState, g_book: &mut Book);
    fn report_in_get_book_move_2(chosen_score: i32, chosen_index: i32, flags: &i32, x: &[CandidateMove; 60], search_state: &SearchState);
    fn midgame_display_simple_ponder_move(move_0: i8);
    fn midgame_display_initial_ponder_move(alpha: i32, beta: i32);
    fn midgame_display_ponder_move(
        max_depth: i32, alpha: i32, beta: i32,
        curr_val: i32, searched: i32, update_pv: i32, echo: i32);
    fn midgame_display_status(side_to_move: i32, max_depth: i32,
                              eval_info: &EvaluationType, depth: i32,
                              force_return_: bool,
                              g_timer: &mut Timer,
                              moves_state: &mut MovesState,
                              board_state: &mut BoardState,
                              hash_state: &mut HashState,
                              search_state: &mut SearchState,
                              flip_stack_: &mut FlipStack);
    fn report_mirror_symetry_error(count: i32, i: i32, first_mirror_offset: i32, first_item: i32, second_item: i32);
}
pub trait FatalError {
    fn invalid_move(curr_move: i8) -> !;
    fn unrecognized_character(unrecognized: i8) -> !;
    fn cannot_open_game_file(file_name: &str) -> !;
    fn invalid_move_in_move_sequence(curr_move: i8) -> !;
    fn internal_error_in_book_code() -> !;
    fn unexpected_character_in_a_move_string() -> !;
    fn invalid_move_string_provided() -> !;
    fn initial_squares_are_empty() -> !;
}
