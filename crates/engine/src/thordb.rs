use crate::src::myrandom::MyRandom;

pub trait ThorDatabase {
    fn get_thor_game_move(index: i32, move_number: i32) -> i32;
    fn database_search(in_board: &[i32], side_to_move: i32);
    fn get_match_count() -> i32;
    fn get_black_win_count() -> i32;
    fn get_draw_count() -> i32;
    fn get_white_win_count() -> i32;
    fn get_black_median_score() -> i32;
    fn get_black_average_score() -> f64;
    fn choose_thor_opening_move(in_board: &[i32], side_to_move: i32, echo: i32, random: &mut MyRandom) -> i32;
}
