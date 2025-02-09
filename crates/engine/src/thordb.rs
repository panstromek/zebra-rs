use crate::src::myrandom::MyRandom;
use crate::src::timer::Timer;

pub trait ThorDatabase {
    fn get_thor_game_move(&self, index: i32, move_number: i32) -> i32;
    fn database_search(&mut self, in_board: &[i32], side_to_move: i32);
    fn get_match_count(&self) -> i32;
    fn get_black_win_count(&self) -> i32;
    fn get_draw_count(&self) -> i32;
    fn get_white_win_count(&self) -> i32;
    fn get_black_median_score(&self) -> i32;
    fn get_black_average_score(&self) -> f64;
    fn choose_thor_opening_move(&mut self, in_board: &[i32], side_to_move: i32, echo: i32, random: &mut MyRandom) -> i32;
    fn load_thor_files(&mut self, g_timer: &mut Timer);
}
