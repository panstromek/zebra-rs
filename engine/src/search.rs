use crate::src::globals::Board;
use crate::src::counter::CounterType;

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

pub static mut total_time: f64 = 0.;

pub static mut root_eval: i32 = 0;

pub static mut force_return: i32 = 0;

pub static mut full_pv_depth: i32 = 0;

pub static mut full_pv: [i32; 120] = [0; 120];

pub static mut list_inherited: [i32; 61] = [0; 61];

pub static mut sorted_move_order: [[i32; 64]; 64] = [[0; 64]; 64];
/* 61*60 used */

pub static mut evals: [Board; 61] = [[0; 128]; 61];

pub static mut nodes: CounterType = CounterType{hi: 0, lo: 0,};

pub static mut total_nodes: CounterType = CounterType{hi: 0, lo: 0,};

pub static mut evaluations: CounterType = CounterType{hi: 0, lo: 0,};

pub static mut total_evaluations: CounterType = CounterType{hi: 0, lo: 0,};
/* When no other information is available, JCW's endgame
   priority order is used also in the midgame. */

pub static mut position_list: [i32; 100] =
    [11 as i32, 18 as i32, 81 as i32,
        88 as i32, 13 as i32, 16 as i32,
        31 as i32, 38 as i32, 61 as i32,
        68 as i32, 83 as i32, 86 as i32,
        33 as i32, 36 as i32, 63 as i32,
        66 as i32, 14 as i32, 15 as i32,
        41 as i32, 48 as i32, 51 as i32,
        58 as i32, 84 as i32, 85 as i32,
        34 as i32, 35 as i32, 43 as i32,
        46 as i32, 53 as i32, 56 as i32,
        64 as i32, 65 as i32, 24 as i32,
        25 as i32, 42 as i32, 47 as i32,
        52 as i32, 57 as i32, 74 as i32,
        75 as i32, 23 as i32, 26 as i32,
        32 as i32, 37 as i32, 62 as i32,
        67 as i32, 73 as i32, 76 as i32,
        12 as i32, 17 as i32, 21 as i32,
        28 as i32, 71 as i32, 78 as i32,
        82 as i32, 87 as i32, 22 as i32,
        27 as i32, 72 as i32, 77 as i32,
        44 as i32, 45 as i32, 54 as i32,
        45 as i32, 0 as i32, 1 as i32, 2 as i32,
        3 as i32, 4 as i32, 5 as i32, 6 as i32,
        7 as i32, 8 as i32, 9 as i32, 19 as i32,
        29 as i32, 39 as i32, 49 as i32,
        59 as i32, 69 as i32, 79 as i32,
        89 as i32, 10 as i32, 20 as i32,
        30 as i32, 40 as i32, 50 as i32,
        60 as i32, 70 as i32, 80 as i32,
        90 as i32, 91 as i32, 92 as i32,
        93 as i32, 94 as i32, 95 as i32,
        96 as i32, 97 as i32, 98 as i32,
        99 as i32];