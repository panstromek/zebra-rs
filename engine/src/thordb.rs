
pub trait ThorDatabase {
    fn choose_thor_opening_move_report(freq_sum: i32, match_count: i32, move_list: &[thordb_types::C2RustUnnamed; 64]);
    fn get_thor_game_move(index: i32, move_number: i32) -> i32;
    fn database_search(in_board: &[i32], side_to_move: i32);
    fn get_match_count() -> i32;
    fn get_black_win_count() -> i32;
    fn get_draw_count() -> i32;
    fn get_white_win_count() -> i32;
    fn get_black_median_score() -> i32;
    fn get_black_average_score() -> f64;
    fn choose_thor_opening_move(in_board: &[i32], side_to_move: i32, echo: i32) -> i32;
}
