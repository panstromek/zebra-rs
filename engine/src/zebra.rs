use crate::src::display::set_names;

pub type Board = [i32; 128];
pub type EvalType = u32;
pub const UNINITIALIZED_EVAL: EvalType = 8;
pub const INTERRUPTED_EVAL: EvalType = 7;
pub const UNDEFINED_EVAL: EvalType = 6;
pub const PASS_EVAL: EvalType = 5;
pub const FORCED_EVAL: EvalType = 4;
pub const SELECTIVE_EVAL: EvalType = 3;
pub const WLD_EVAL: EvalType = 2;
pub const EXACT_EVAL: EvalType = 1;
pub const MIDGAME_EVAL: EvalType = 0;
pub type EvalResult = u32;
pub const UNSOLVED_POSITION: EvalResult = 3;
pub const LOST_POSITION: EvalResult = 2;
pub const DRAWN_POSITION: EvalResult = 1;
pub const WON_POSITION: EvalResult = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EvaluationType {
    pub type_0: EvalType,
    pub res: EvalResult,
    pub score: i32,
    pub confidence: f64,
    pub search_depth: i32,
    pub is_book: i32,
}
pub type DrawMode = u32;
pub const OPPONENT_WINS: DrawMode = 3;
pub const WHITE_WINS: DrawMode = 2;
pub const BLACK_WINS: DrawMode = 1;
pub const NEUTRAL: DrawMode = 0;
pub type GameMode = u32;
pub const PUBLIC_GAME: GameMode = 1;
pub const PRIVATE_GAME: GameMode = 0;

/* Local variables */
pub static mut slack: f64 = 0.25f64;
pub static mut dev_bonus: f64 = 0.0f64;
pub static mut low_thresh: i32 = 0 as i32;
pub static mut high_thresh: i32 = 0 as i32;
pub static mut rand_move_freq: i32 = 0 as i32;
pub static mut tournament: i32 = 0 as i32;
pub static mut tournament_levels: i32 = 0;
pub static mut deviation_depth: i32 = 0;
pub static mut cutoff_empty: i32 = 0;
pub static mut one_position_only: i32 = 0 as i32;
pub static mut use_timer: i32 = 0 as i32;
pub static mut only_analyze: i32 = 0 as i32;
pub static mut thor_max_games: i32 = 0;
pub static mut tournament_skill: [[i32; 3]; 8] = [[0; 3]; 8];
pub static mut wld_skill: [i32; 3] = [0; 3];
pub static mut exact_skill: [i32; 3] = [0; 3];
pub static mut player_time: [f64; 3] = [0.; 3];
pub static mut player_increment: [f64; 3] = [0.; 3];
pub static mut skill: [i32; 3] = [0; 3];
pub static mut wait: i32 = 0;
pub static mut use_book: i32 = 1 as i32;
pub static mut wld_only: i32 = 0 as i32;
pub static mut use_learning: i32 = 0;
pub static mut use_thor: i32 = 0;


/// This trait is unsafe because line buffer is used as a c-style string later
/// so this function needs to ensure that the line_buffer contains at
/// least one null character (there's definitely better way to do this, but I
/// don't want to deviate from the original source for first implementation)
pub unsafe trait InitialMoveSource {
    fn fill_line_buffer(&mut self, line_buffer: &mut [i8; 1000]);
}


pub unsafe fn set_names_from_skills() {
    let mut black_name = 0 as *const i8;
    if skill[0 as i32 as usize] == 0 as i32 {
        black_name = b"Player\x00" as *const u8 as *const i8
    } else {
        black_name = b"Zebra\x00" as *const u8 as *const i8
    }
    let mut white_name = 0 as *const i8;
    if skill[2 as i32 as usize] == 0 as i32 {
        white_name = b"Player\x00" as *const u8 as *const i8
    } else {
        white_name = b"Zebra\x00" as *const u8 as *const i8
    }
    set_names(black_name, white_name);
}