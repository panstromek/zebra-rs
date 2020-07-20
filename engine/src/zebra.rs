
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