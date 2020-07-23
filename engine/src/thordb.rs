use crate::src::bitboard::bit_reverse_32;
use crate::src::myrandom::my_random;
use crate::src::patterns::pow3;
use crate::src::moves::dir_mask;
use crate::src::error::fatal_error;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameInfoType {
    pub black_name: *const i8,
    pub white_name: *const i8,
    pub tournament: *const i8,
    pub year: i32,
    pub black_actual_score: i32,
    pub black_corrected_score: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DatabaseInfoType {
    pub year: i32,
    pub count: i32,
}
pub type PlayerFilterType = u32;
pub const WhiteSelectedFilter: PlayerFilterType = 3;
pub const BlackSelectedFilter: PlayerFilterType = 2;
pub const BothSelectedFilter: PlayerFilterType = 1;
pub const EitherSelectedFilter: PlayerFilterType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlayerType {
    pub lex_order: i32,
    pub is_program: i32,
    pub selected: i32,
    pub name: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlayerDatabaseType {
    pub prolog: PrologType,
    pub name_buffer: *mut i8,
    pub count: i32,
    pub player_list: *mut PlayerType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PrologType {
    pub creation_century: i32,
    pub creation_year: i32,
    pub creation_month: i32,
    pub creation_day: i32,
    pub game_count: i32,
    pub item_count: i32,
    pub origin_year: i32,
    pub reserved: i32,
}
pub type DatabaseType = DatabaseType_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DatabaseType_ {
    pub prolog: PrologType,
    pub games: *mut GameType,
    pub count: i32,
    pub next: *mut DatabaseType_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameType {
    pub tournament_no: i16,
    pub black_no: i16,
    pub white_no: i16,
    pub actual_black_score: i16,
    pub perfect_black_score: i16,
    pub moves: [i8; 60],
    pub move_count: i16,
    pub black_disc_count: [i8; 61],
    pub opening: *mut ThorOpeningNode,
    pub database: *mut DatabaseType_,
    pub shape_hi: u32,
    pub shape_lo: u32,
    pub shape_state_hi: i16,
    pub shape_state_lo: i16,
    pub corner_descriptor: u32,
    pub sort_order: i32,
    pub matching_symmetry: i16,
    pub passes_filter: i16,
}
pub type ThorOpeningNode = ThorOpeningNode_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThorOpeningNode_ {
    pub hash1: u32,
    pub hash2: u32,
    pub current_match: i32,
    pub frequency: i32,
    pub matching_symmetry: i32,
    pub child_move: i8,
    pub sibling_move: i8,
    pub child_node: *mut ThorOpeningNode_,
    pub sibling_node: *mut ThorOpeningNode_,
    pub parent_node: *mut ThorOpeningNode_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TournamentType {
    pub lex_order: i32,
    pub selected: i32,
    pub name: *const i8,
}
pub type int_32 = i32;
pub type int_16 = i16;
/* Type definitions */
pub type int_8 = i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TournamentDatabaseType {
    pub prolog: PrologType,
    pub name_buffer: *mut i8,
    pub count: i32,
    pub tournament_list: *mut TournamentType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub move_0: i32,
    pub frequency: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FilterType {
    pub game_categories: i32,
    pub first_year: i32,
    pub last_year: i32,
    pub player_filter: PlayerFilterType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SearchResultType {
    pub average_black_score: f64,
    pub next_move_score: [f64; 100],
    pub match_count: i32,
    pub black_wins: i32,
    pub draws: i32,
    pub white_wins: i32,
    pub median_black_score: i32,
    pub allocation: i32,
    pub next_move_frequency: [i32; 100],
    pub match_list: *mut *mut GameType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThorOpening {
    pub first_unique: i32,
    pub frequency: i32,
    pub move_str: *const i8,
}

pub static mut thor_opening_list: [ThorOpening; 741] =
    [{
        let mut init =
            ThorOpening{first_unique: 0 as i32,
                frequency: 39 as i32,
                move_str:
                b"f5f6d3g5e6\x00" as *const u8 as
                    *const i8,};
        init
    },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f4e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 2 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"c4g5e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"f4e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 2 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"e6f4c3e7f3e3d3e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"d7e3d6e7c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f3d6g5g4e3g6e7g3f7f8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"h5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"g3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"e2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"g6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"e3g4c4d3d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e7d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"d6f3c5g4h3h5g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g3h3c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"e7f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"d7g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"c4c5b6g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"c6b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"b4a5a3c6b5a6b6g4e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"g5e3b3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"e3b3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"d3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"b3e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g5e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"c6b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"e3d3c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"g5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"c6d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"c5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"c4d3c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"c5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"d3e7f3e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"d7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"d6f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"g5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"e3d7g4f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 89 as i32,
                    move_str:
                    b"d6c4f3g3c3c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"g4e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"e2d7c6f7d3g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"g4d3c3h3c4g3g5g6c7c6h5f3h4h6\x00" as *const u8
                        as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e7d7h5f3h4h6c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"h4g6f3h6h5h2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"c6c5d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"h4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 50 as i32,
                    move_str:
                    b"g3g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"d2c4f3f2e2g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"c4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 62 as i32,
                    move_str:
                    b"c5f3g4e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"g5d3c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"c6c5e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e7d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"c5c4e7g4g3g5f3g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"g6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"d7d6f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"g5b5d6g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"e8\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"f3g3d3d6b5c3c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"g5d3c2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"d7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"b5d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"e2f2c3d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"d2f3g4g5d6g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"h3d3c6b3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"d3c3c2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"f3d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f2g4g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 54 as i32,
                    move_str:
                    b"g5g4g6d6h3h5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"d3d7f7c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g5g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"g5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"c6d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 56 as i32,
                    move_str:
                    b"e2f2d2f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"f3g4g5d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"d3c3d2e1c1f2g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"c2d1f1g6d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"f2g4g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"g5g4g6d6h3h5g3c3d3b3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"f7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"d7c6e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"d6b5d3c3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 50 as i32,
                    move_str:
                    b"c6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 56 as i32,
                    move_str:
                    b"d3c3f2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f3e2c3g4c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"e2b5e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"g6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"g4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"g5d6g6h6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 55 as i32,
                    move_str:
                    b"f3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 66 as i32,
                    move_str:
                    b"g4g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"g3c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"g4d6f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"f3c4d6e2h5h6h3h4h7g6f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g6d3f2h4e2h6c6d2b5f7h5e7d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"d2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"h5h6e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"g6f3d3e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 59 as i32,
                    move_str:
                    b"f3g4g6g3d6f7h5h4h3e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"e7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 78 as i32,
                    move_str:
                    b"g6e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"d7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"d3g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 58 as i32,
                    move_str:
                    b"d6f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 55 as i32,
                    move_str:
                    b"g4c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"g6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"c6e7d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"d6g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"e7d7c3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 65 as i32,
                    move_str:
                    b"d3f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"g6f3g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"d3f3d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 70 as i32,
                    move_str:
                    b"c5c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"g5g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 63 as i32,
                    move_str:
                    b"d2e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"f3c5c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 61 as i32,
                    move_str:
                    b"c6e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"f7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 53 as i32,
                    move_str:
                    b"g3d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"f3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"g4e7f7g5f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"g5e7e3g6d3c6c5f3g4h4h6h3f2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"c5d6c6d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"f7h5c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"c4e8g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"d2c3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"b5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"c4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"f3g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f3d6f7d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"c5c6d6f3g4h3h4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"d6h5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"c4c5b5f7h5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 59 as i32,
                    move_str:
                    b"d3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"e8\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"f7h5h4h3c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"d7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"f3f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"d6d3g4g6h5h6h7h4h3c4f3c5c6d7b5\x00" as
                        *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"f7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 56 as i32,
                    move_str:
                    b"f3h3h5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"f3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"g4d3c6d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 57 as i32,
                    move_str:
                    b"g6h5h6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"f3h3c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"c5d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"g6f3h3c4g3h5h6h4f7c2d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"f7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"f3d3c3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g3f3d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"g4d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"f3d3c3f7d7d8g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 57 as i32,
                    move_str:
                    b"c4d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"c7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e2g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"c5c4g3c6d6d7c7g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"d3g4g3g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"d7g6g4h5e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 50 as i32,
                    move_str:
                    b"e8\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"d6f3g6g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g4e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"g3e3g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"c5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"f7g6g4h5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 98 as i32,
                    move_str:
                    b"h5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"h6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"d6d8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"e8h6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"h5d3c6e3d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 51 as i32,
                    move_str:
                    b"d7c4g4h4e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"c4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 61 as i32,
                    move_str:
                    b"e3d3f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"g4g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"h4f3g6h3h2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 81 as i32,
                    move_str:
                    b"f2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"h3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"g4g6e3h4f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"f3g3d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"h3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"h4e3d6c3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"f3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"e8c5g4h4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 71 as i32,
                    move_str:
                    b"c5f3g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"g3h3h5g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"e8h5g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"h6e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"g6g4e7e3f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"f3d6f7h3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"f7h5e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"h6h5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"f7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"c6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"e7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"d6e3g6f7h5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"f3g6e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 87 as i32,
                    move_str:
                    b"f7h5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 53 as i32,
                    move_str:
                    b"c5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"d7e7f3g6g4f7f8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"g4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"g3e3g4c6d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"c5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"e7g6c5c4f3h5f7g4h6h7d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"d7f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 54 as i32,
                    move_str:
                    b"f7f8d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"g4f3h5e3d3h3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"h6h7e3d3h3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"f7g4f3h5h4h3g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 56 as i32,
                    move_str:
                    b"h6h7c5g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 73 as i32,
                    move_str:
                    b"c6f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"h4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"g6f7g5e7d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"h5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e7d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"d6e3f3g4g5d3e2c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"d3g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"g5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"d3g4f3g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"g4f7e8h3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 63 as i32,
                    move_str:
                    b"g5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"c6e7f8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"c6g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"c5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 58 as i32,
                    move_str:
                    b"g5d6c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"c5f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g4g5e3f3f2h4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 66 as i32,
                    move_str:
                    b"g3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f3e3c4g3h5f7e7d3c2b4d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"h5g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f2d3c3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 67 as i32,
                    move_str:
                    b"h4h6e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 57 as i32,
                    move_str:
                    b"f3e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"h3h2h6g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 59 as i32,
                    move_str:
                    b"f3h5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 55 as i32,
                    move_str:
                    b"g3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"h4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"g7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 2 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"f7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 1 as i32,
                    frequency: 51 as i32,
                    move_str:
                    b"d6c3g5c6b6e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"f6e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 51 as i32,
                    move_str:
                    b"c5c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"d3e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"f6d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"g6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"f4e6f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"d3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"f6g5f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"e6e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"d7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"c4e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"f3d3c4e3e2g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"e3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"d3f3g4g3e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"b3c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"c4g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"c6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"f3d3f4f6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"f4d3c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"d3e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"c4b5b4f4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"f4e3f6c6c5d7e7e6b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 57 as i32,
                    move_str:
                    b"c7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f3g4f6g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"g3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 56 as i32,
                    move_str:
                    b"e2d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 65 as i32,
                    move_str:
                    b"e6f6g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"c6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 66 as i32,
                    move_str:
                    b"b4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 62 as i32,
                    move_str:
                    b"c5f6e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 56 as i32,
                    move_str:
                    b"b5b4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"b4b3e6c6b5e3f2a6a5d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 55 as i32,
                    move_str:
                    b"c6c2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"b5c6e6e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"c2e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"d7c7e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"c6e6b5a5b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"a6a3b6a4a2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 55 as i32,
                    move_str:
                    b"e3e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 57 as i32,
                    move_str:
                    b"b5e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"c6b3e6e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"e3f3b3e6b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f3f6e3g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"e6e3g6b6a5d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"f6g5d7g3b6e7a5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 18 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e7e2g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 18 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"d2b6a5e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"c2b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"a5e3c2a4b3a6b6d2a3a2e1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"c6e6a3b3d2c2c1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"e3b5b6b3a6a5a3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"b3c2f6a3b5b6e3f2e2b4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"e6b4f3e3e2g4d2g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"g4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"d2e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"c6b6b4f6g3b5a3e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"b4b5d2a3a4a5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e3a6c1d7e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"b6f3f6f7g5d7c7g4e1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 24 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"g6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 21 as i32,
                    frequency: 60 as i32,
                    move_str:
                    b"e2d1f1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 21 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"e1e2f1f2g5d7c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 19 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"e1e2f6f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 53 as i32,
                    move_str:
                    b"g4a5f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"f3g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"e1\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"c7f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 106 as i32,
                    move_str:
                    b"f7c7f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"b6a6c8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"a6e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"b6d2e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"d2e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"b4e3e6f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"b6c6b5f6a5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"a6a5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"a4f3a6a3d2c1f6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"c1\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"c6f6a5a4b5a6d7d8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 19 as i32,
                    frequency: 55 as i32,
                    move_str:
                    b"c7e7e8b6f8g3f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 23 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"d8a2a7g3f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 24 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"g3f7g5d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 27 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"d1c1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 21 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"b6f7e8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 56 as i32,
                    move_str:
                    b"d7b5e7c7b6f8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 55 as i32,
                    move_str:
                    b"g4f6b6a6e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"g6g5f3d7f7e7a3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"e2d2f3b6g6a5a4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"b5d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"c6e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"d2a3e6a4b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 51 as i32,
                    move_str:
                    b"e3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e6b5e3f3g4b6a6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"a5g6g5h6g4h5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"a6a3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"a5d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g6a4a3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"a6a3b6a4a2e3f3g4c1f2g3h4h3h2f1\x00" as
                        *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 20 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"g3g4e3f3h3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"a7b6c7g5a2c1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"e3d2c6f2e6b4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"b4f6b5e6f2f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"b5e2e6f3c1a6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f2e6f3c1a3a4a5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"a3e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"a6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"a4b5b6g4e2a3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"a3f2e6b5a4f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"g3g4f3c1b5a4a6d1e2f2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"b5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"f3e6e1d1c1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"g4f3e6f7g3c1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 18 as i32,
                    frequency: 50 as i32,
                    move_str:
                    b"b5b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 19 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"a4a6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"b6b4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"b4c6g6f2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"e6b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 53 as i32,
                    move_str:
                    b"d2e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"e6b4g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 61 as i32,
                    move_str:
                    b"d1a3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"d2c6b5e6b4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 70 as i32,
                    move_str:
                    b"b6b4b5e6c2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"e3c2b4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 59 as i32,
                    move_str:
                    b"b4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"e2e3c2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"d2b4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"c6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 50 as i32,
                    move_str:
                    b"e6f6e3c5g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"c6b6c7b5d7e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"e7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"d7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 53 as i32,
                    move_str:
                    b"b5d7e7b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 59 as i32,
                    move_str:
                    b"b4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"b3c2e3d2c5b4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 57 as i32,
                    move_str:
                    b"b4c5c6d7b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"d2e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"e2c5c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"e3f3c5b4a3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 58 as i32,
                    move_str:
                    b"f6g5e3f3g4h3g3f2e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e6g3g6f7h5h4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"e2d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"g6f7h6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e2h5c5g4g3f2d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"e6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"f3g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"e3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"c6e3d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 50 as i32,
                    move_str:
                    b"e6f7e3c5d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"h5h6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"c6c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"g6c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 57 as i32,
                    move_str:
                    b"c7c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"d7c5g3f3c6e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e3c6b4d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"d7e3c5f3e7h5e2c6d2c2b3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"f7e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"c7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e7c5e3f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 50 as i32,
                    move_str:
                    b"f3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"f7c5b6e3c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 55 as i32,
                    move_str:
                    b"c5e3f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 61 as i32,
                    move_str:
                    b"f3f7e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 63 as i32,
                    move_str:
                    b"b5e3g4h4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"b4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"e3g4f7e7g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"b5e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"g6e3e6c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"b4c2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"f3e6e3g5g6g4h4h5h6g3h3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f3e3g5g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"b4b5e6e7g5c5e2g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"f2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"e2e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"g5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 54 as i32,
                    move_str:
                    b"g4g5e3h3g3f2e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"e6g3g6f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"g3c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"e6b3d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"e6e7e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"c6g6g5f7g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f8f7d2c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"c5b4f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g5d2d7f8e8f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e3d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"d7g6g5f7e2c8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"h6e3c8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"c5e3g3g4h5f8d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"e8h6d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"c5c6c8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"h6d2c8c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"b6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"g4d2c8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 63 as i32,
                    move_str:
                    b"f8h6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 56 as i32,
                    move_str:
                    b"c5b6d8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"c6b5c7f8e8f7h6d2g4c8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"c6f7e8h6d2c8c7b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"f8h6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 82 as i32,
                    move_str:
                    b"f7c5c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"d8f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"c5c6c7c8f7g8b5b6b4a3a4e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 19 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"b3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"b6a5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 18 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"b5e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"f8f7g5h6h4g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"g5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"c5b6d8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"b5e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"c6g6g5f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"f7c5e3g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"b4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 80 as i32,
                    move_str:
                    b"e2g3f2g4d2d8g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"g4g3f2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"d2f2g6c7g5g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"b6g6e3e2f1d2e1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 64 as i32,
                    move_str:
                    b"d1\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g4g5c6h5f8d8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"g5c6d8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"g4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"g5c6e8b5e3d2b3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"f8\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"g5e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"b5e3g6g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 72 as i32,
                    move_str:
                    b"b4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"c6c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"f3f4b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 55 as i32,
                    move_str:
                    b"b3d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"c5c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"b4b6c6b5e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"c6b6d7e8c2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"f4e6c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"c5f4f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 66 as i32,
                    move_str:
                    b"c6f4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 2 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"c4g5c6b6f6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"c5b6b5a6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"c3e3b5f6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 50 as i32,
                    move_str:
                    b"e6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"f6f4e6f7g4e7c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"f3h5h6h7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"e3g6d7g3c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"d7e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"d7c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"d3b4c3e3b5a4f2c2a6f6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"f3e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"d3e6c5b4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"f6f4f3d3c3g6e3e6c5e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"e6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 72 as i32,
                    move_str:
                    b"g6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"g6d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 67 as i32,
                    move_str:
                    b"f4c5f6g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"d3e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"e6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 51 as i32,
                    move_str:
                    b"f6g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"f3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"f3f4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 54 as i32,
                    move_str:
                    b"d3c3f4e3f3g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"c5b4b3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"b3c2f6a3b5b6e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"e6b4f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"c6b4b5d2e3a6c1b6f3f6f7g5d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 21 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"e1e2f1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"b4e3e6b6c6b5a4f3a6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"c6f6a5a4b5a6d7c7e7e8b6f8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 23 as i32,
                    frequency: 64 as i32,
                    move_str:
                    b"d8g3f7g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"d7b5e7c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"g4f6b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"c6d2e6b5e3f3a5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"a5a6a3b6a4a2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"a7b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"e3d2c6b4b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"a4b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"a3g3g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"g4f3e6f7b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"d2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"e6f6e3c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"b3e2e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"f6g5e6f7d7c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"d7e3c5f3e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"c5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 50 as i32,
                    move_str:
                    b"f3e3b4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 68 as i32,
                    move_str:
                    b"g4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"e6e7c6g6g5f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"f8f7d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"d7g6g5f7e2c5e3g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"e8c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"f8h6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"c5b6c6b5c7f8e8f7h6d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"c6f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"f7c5c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"d8c5c6c7c8f7g8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"f8f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"c5c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"f7c5e3e2g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"g4d2f2g6c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"b6g6e3e2f1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"g5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"b3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"c5b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"f4d2f6d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"e2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"e3c6e6f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"f3c2f6e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"e6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"d7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"c6g4d2e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"e6d2g4b6b5e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"g3d1c3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"c7a5f2d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"c3b4c1f2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"c7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 73 as i32,
                    move_str:
                    b"d7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"b4b3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"b5d2d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"d2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"e2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"g4g6g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"e6c3d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"f6e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"c3d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 79 as i32,
                    move_str:
                    b"e6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"d7f6d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"b4c3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"d7f4e3f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"e6f6e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f4e3f6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f3c3c5d2c6g4f7e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"d7g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"b4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"b4b3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"b4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"g4e7c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"g3d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 57 as i32,
                    move_str:
                    b"c6f6g5g6e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"c6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"b3b4f4f6g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 2 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"c5f6e6f4c6e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"f4d3f6e6d7f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 67 as i32,
                    move_str:
                    b"c4c3c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"e3e6f6g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 53 as i32,
                    move_str:
                    b"c3e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"e3g4g5e2c4f3e6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"e6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"f3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"e6f6e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 58 as i32,
                    move_str:
                    b"c4e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 76 as i32,
                    move_str:
                    b"g3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"g3e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 62 as i32,
                    move_str:
                    b"e6c4e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"f6e6g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"c6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"e3f6f3c4d3e6c3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 61 as i32,
                    move_str:
                    b"e2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"e6d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"e2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 54 as i32,
                    move_str:
                    b"e6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 74 as i32,
                    move_str:
                    b"c6d3f6g4c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 66 as i32,
                    move_str:
                    b"e6f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"d7g3b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"c4b4c3d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"b5b3b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"b3b5a4a2a3a5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"g5c3b5a5a4a3b6c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 21 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"a6f7f2f3e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 23 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"e2d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 23 as i32,
                    frequency: 55 as i32,
                    move_str:
                    b"d2e2f1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"e2f2d2c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 18 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"f3d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 18 as i32,
                    frequency: 65 as i32,
                    move_str:
                    b"e7c7b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 20 as i32,
                    frequency: 72 as i32,
                    move_str:
                    b"f7g6f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 57 as i32,
                    move_str:
                    b"b6b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 56 as i32,
                    move_str:
                    b"c3b5a6g5a5a4a3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 18 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"g6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 91 as i32,
                    move_str:
                    b"g4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"a3g5c3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"e2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 77 as i32,
                    move_str:
                    b"g5c3b4b3b5a5a4a3b6a6f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"a4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"e2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"a3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"e2b3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"b5g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"d2c7b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"d2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"f7g6e7f8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"e2c8f2c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"d2e7f2e1f3c8h3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 18 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"e2f1c8f3c7h3b6g4c1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 18 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"c8f3c7h3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"g6g5h6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"e7\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"c3f7d2e7f2e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e7c3g4f8f7g6c8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 50 as i32,
                    move_str:
                    b"g5f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"g5f8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"g4c4b4g3g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"b3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"g5c3b3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 58 as i32,
                    move_str:
                    b"b4b3b5a5a4a3b6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"e2f3d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"f7g6e7f8h6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 15 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"d2c1f2e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 17 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"e2f2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 18 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"b5g3f3h4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"e7f2e2f3c1g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 19 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"f1c8f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 18 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"c8f3c7d8e2b8f1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 23 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"g3e8f1h2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 23 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f1e8g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 22 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"e8g3e2h4f1e1\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 26 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"h5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 25 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"f1\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 21 as i32,
                    frequency: 70 as i32,
                    move_str:
                    b"g6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g6g5e7f8f7h6h5c3h7g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"c3f7d2e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 73 as i32,
                    move_str:
                    b"g3g5h4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"g6g5h5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"e7c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"h3g5h4e7f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"e7c7c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 60 as i32,
                    move_str:
                    b"f3g5h6f7f8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"e7d8c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"c7c4f8b6b5a6a4g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"d8e8g8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"d8\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 13 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"f3b6b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 14 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"d8c8f7f8b8e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"b8e2b6e8f8f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"b6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"f3c4c7d8c8f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 16 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"b8\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 35 as i32,
                    move_str:
                    b"c3c4c7d8c8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 51 as i32,
                    move_str:
                    b"f3c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"g5e6f6g3c4b4b3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"d7f3e7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 61 as i32,
                    move_str:
                    b"f6f3g6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 60 as i32,
                    move_str:
                    b"f3g4g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"c4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 65 as i32,
                    move_str:
                    b"e2d2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 62 as i32,
                    move_str:
                    b"e6f7g4g5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 11 as i32,
                    frequency: 53 as i32,
                    move_str:
                    b"c4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"g5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"e7f6d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"f6g4g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"b5c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"c4c3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"e2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"f3g6e6f6d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"f6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"g5e6f6d7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 58 as i32,
                    move_str:
                    b"d3c4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"c3d3c4b3c2b4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"e6f7f3b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"c4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"d7e8c8c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"c7f6e7e8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 61 as i32,
                    move_str:
                    b"f6b5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"e7d8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"e7f6g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 10 as i32,
                    frequency: 51 as i32,
                    move_str:
                    b"d7d8c7c8f8b8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 12 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"f8c8c7b8g8\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 9 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"d3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"f6g3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 58 as i32,
                    move_str:
                    b"g4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 8 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"d7c7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"g5f6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"f3d3f7\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 52 as i32,
                    move_str:
                    b"d3c4f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"f6e6d3f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 44 as i32,
                    move_str:
                    b"d7e6f6d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"g5f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 68 as i32,
                    move_str:
                    b"c4d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 45 as i32,
                    move_str:
                    b"e6c6d3f3g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"d3f3e2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"e6g5c4c3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 41 as i32,
                    move_str:
                    b"c3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"f3g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 58 as i32,
                    move_str:
                    b"e3c4d3c6g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"d7f6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 73 as i32,
                    move_str:
                    b"c6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 40 as i32,
                    move_str:
                    b"g5\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"e7c6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"b4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 2 as i32,
                    frequency: 49 as i32,
                    move_str:
                    b"c6f6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"f4d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 42 as i32,
                    move_str:
                    b"f3c5\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 51 as i32,
                    move_str:
                    b"e6f6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"g5d3c4e3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"e3f6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"c5e3f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 47 as i32,
                    move_str:
                    b"b4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 2 as i32,
                    frequency: 56 as i32,
                    move_str:
                    b"c7f4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 37 as i32,
                    move_str:
                    b"f3\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 1 as i32,
                    frequency: 46 as i32,
                    move_str:
                    b"f4e3f6d3c5d6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"c4f3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"e2f2\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 6 as i32,
                    frequency: 38 as i32,
                    move_str:
                    b"f3g4\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 7 as i32,
                    frequency: 43 as i32,
                    move_str:
                    b"f2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 4 as i32,
                    frequency: 68 as i32,
                    move_str:
                    b"e6\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 33 as i32,
                    move_str:
                    b"d6f3g5f6\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 48 as i32,
                    move_str:
                    b"g4\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 5 as i32,
                    frequency: 36 as i32,
                    move_str:
                    b"e2\x00" as *const u8 as *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 34 as i32,
                    move_str:
                    b"f2e2f6d3\x00" as *const u8 as
                        *const i8,};
            init
        },
        {
            let mut init =
                ThorOpening{first_unique: 3 as i32,
                    frequency: 39 as i32,
                    move_str:
                    b"d2e2f6d3\x00" as *const u8 as
                        *const i8,};
            init
        }];

/* Local variables */
pub static mut thor_game_count: i32 = 0;
pub static mut thor_database_count: i32 = 0;
pub static mut thor_side_to_move: i32 = 0;
pub static mut thor_sort_criteria_count: i32 = 0;
pub static mut thor_games_sorted: i32 = 0;
pub static mut thor_games_filtered: i32 = 0;
pub static mut thor_row_pattern: [i32; 8] = [0; 8];
pub static mut thor_col_pattern: [i32; 8] = [0; 8];
pub static mut thor_board: [i32; 100] = [0; 100];
pub static mut b1_b1_map: [i32; 100] = [0; 100];
pub static mut g1_b1_map: [i32; 100] = [0; 100];
pub static mut g8_b1_map: [i32; 100] = [0; 100];
pub static mut b8_b1_map: [i32; 100] = [0; 100];
pub static mut a2_b1_map: [i32; 100] = [0; 100];
pub static mut a7_b1_map: [i32; 100] = [0; 100];
pub static mut h7_b1_map: [i32; 100] = [0; 100];
pub static mut h2_b1_map: [i32; 100] = [0; 100];
pub static mut primary_hash: [[u32; 6561]; 8] = [[0; 6561]; 8];
pub static mut secondary_hash: [[u32; 6561]; 8] = [[0; 6561]; 8];
pub static mut symmetry_map: [*mut i32; 8] =
    [0 as *const i32 as *mut i32; 8];
pub static mut inv_symmetry_map: [*mut i32; 8] =
    [0 as *const i32 as *mut i32; 8];
pub static mut move_mask_hi: [u32; 100] = [0; 100];
pub static mut move_mask_lo: [u32; 100] = [0; 100];
pub static mut unmove_mask_hi: [u32; 100] = [0; 100];
pub static mut unmove_mask_lo: [u32; 100] = [0; 100];
pub static mut database_head: *mut DatabaseType =
    0 as *const DatabaseType as *mut DatabaseType;
pub static mut players: PlayerDatabaseType =
    PlayerDatabaseType{prolog:
    PrologType{creation_century: 0,
        creation_year: 0,
        creation_month: 0,
        creation_day: 0,
        game_count: 0,
        item_count: 0,
        origin_year: 0,
        reserved: 0,},
        name_buffer:
        0 as *const i8 as *mut i8,
        count: 0,
        player_list:
        0 as *const PlayerType as *mut PlayerType,};
pub static mut thor_search: SearchResultType =
    SearchResultType{average_black_score: 0.,
        next_move_score: [0.; 100],
        match_count: 0,
        black_wins: 0,
        draws: 0,
        white_wins: 0,
        median_black_score: 0,
        allocation: 0,
        next_move_frequency: [0; 100],
        match_list:
        0 as *const *mut GameType as *mut *mut GameType,};
pub static mut tournaments: TournamentDatabaseType =
    TournamentDatabaseType{prolog:
    PrologType{creation_century: 0,
        creation_year: 0,
        creation_month: 0,
        creation_day: 0,
        game_count: 0,
        item_count: 0,
        origin_year: 0,
        reserved: 0,},
        name_buffer:
        0 as *const i8 as *mut i8,
        count: 0,
        tournament_list:
        0 as *const TournamentType as
            *mut TournamentType,};
pub static mut root_node: *mut ThorOpeningNode =
    0 as *const ThorOpeningNode as *mut ThorOpeningNode;
pub static mut default_sort_order: [i32; 5] =
    [2 as i32, 3 as i32, 1 as i32, 5 as i32,
        4 as i32];
pub static mut thor_sort_order: [i32; 10] = [0; 10];
pub static mut filter: FilterType =
    FilterType{game_categories: 0,
        first_year: 0,
        last_year: 0,
        player_filter: EitherSelectedFilter,};

/*
  CLEAR_THOR_BOARD
*/
pub unsafe fn clear_thor_board() {
    let mut pos: i32 = 0;
    pos = 11 as i32;
    while pos <= 88 as i32 {
        thor_board[pos as usize] = 1 as i32;
        pos += 1
    }
    thor_board[54 as i32 as usize] = 0 as i32;
    thor_board[45 as i32 as usize] =
        thor_board[54 as i32 as usize];
    thor_board[55 as i32 as usize] = 2 as i32;
    thor_board[44 as i32 as usize] =
        thor_board[55 as i32 as usize];
}
/*
  PREPARE_THOR_BOARD
  Mark the positions outside the board as OUTSIDE.
*/
pub unsafe fn prepare_thor_board() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    i = 0 as i32;
    while i < 10 as i32 {
        j = 0 as i32;
        pos = 10 as i32 * i;
        while j < 10 as i32 {
            if i == 0 as i32 || i == 9 as i32 ||
                j == 0 as i32 || j == 9 as i32 {
                thor_board[pos as usize] = 3 as i32
            }
            j += 1;
            pos += 1
        }
        i += 1
    };
}
/*
  DIRECTIONAL_FLIP_COUNT
  Count the number of discs flipped in the direction given by INC
  when SQ is played by COLOR and flip those discs.
*/
pub unsafe fn directional_flip_count(mut sq: i32,
                                 mut inc: i32,
                                 mut color: i32,
                                 mut oppcol: i32)
                                 -> i32 {
    let mut count = 1 as i32;
    let mut pt = sq + inc;
    if thor_board[pt as usize] == oppcol {
        pt += inc;
        if thor_board[pt as usize] == oppcol {
            count += 1;
            pt += inc;
            if thor_board[pt as usize] == oppcol {
                count += 1;
                pt += inc;
                if thor_board[pt as usize] == oppcol {
                    count += 1;
                    pt += inc;
                    if thor_board[pt as usize] == oppcol {
                        count += 1;
                        pt += inc;
                        if thor_board[pt as usize] == oppcol {
                            count += 1;
                            pt += inc
                        }
                    }
                }
            }
        }
        if thor_board[pt as usize] == color {
            let mut g = count;
            loop  {
                pt -= inc;
                thor_board[pt as usize] = color;
                g -= 1;
                if !(g != 0) { break ; }
            }
            return count
        }
    }
    return 0 as i32;
}
/*
  DIRECTIONAL_FLIP_ANY
  Returns 1 if SQ is feasible for COLOR in the direction given by INC
  and flip the discs which are flipped if SQ is played.
*/
pub unsafe fn directional_flip_any(mut sq: i32,
                               mut inc: i32,
                               mut color: i32,
                               mut oppcol: i32)
                               -> i32 {
    let mut pt = sq + inc;
    if thor_board[pt as usize] == oppcol {
        pt += inc;
        if thor_board[pt as usize] == oppcol {
            pt += inc;
            if thor_board[pt as usize] == oppcol {
                pt += inc;
                if thor_board[pt as usize] == oppcol {
                    pt += inc;
                    if thor_board[pt as usize] == oppcol {
                        pt += inc;
                        if thor_board[pt as usize] == oppcol { pt += inc }
                    }
                }
            }
        }
        if thor_board[pt as usize] == color {
            pt -= inc;
            loop  {
                thor_board[pt as usize] = color;
                pt -= inc;
                if !(pt != sq) { break ; }
            }
            return 1 as i32
        }
    }
    return 0 as i32;
}
/*
  COUNT_FLIPS
  Returns the number of discs flipped if SQNUM is played by COLOR
  and flips those discs (if there are any).
*/
pub unsafe fn count_flips(mut sqnum: i32,
                      mut color: i32,
                      mut oppcol: i32) -> i32 {
    let mut count: i32 = 0;
    let mut mask: i32 = 0;
    count = 0 as i32;
    mask = dir_mask[sqnum as usize];
    if mask & 128 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, -(11 as i32), color, oppcol)
    }
    if mask & 64 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, 11 as i32, color, oppcol)
    }
    if mask & 32 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, -(10 as i32), color, oppcol)
    }
    if mask & 16 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, 10 as i32, color, oppcol)
    }
    if mask & 8 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, -(9 as i32), color, oppcol)
    }
    if mask & 4 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, 9 as i32, color, oppcol)
    }
    if mask & 2 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, -(1 as i32), color, oppcol)
    }
    if mask & 1 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, 1 as i32, color, oppcol)
    }
    return count;
}
/*
  ANY_FLIPS
  Returns 1 if SQNUM flips any discs for COLOR, otherwise 0, and
  flips those discs.
*/
pub unsafe fn any_flips(mut sqnum: i32, mut color: i32,
                    mut oppcol: i32) -> i32 {
    let mut count: i32 = 0;
    let mut mask: i32 = 0;
    count = 0 as i32;
    mask = dir_mask[sqnum as usize];
    if mask & 128 as i32 != 0 {
        count |=
            directional_flip_any(sqnum, -(11 as i32), color, oppcol)
    }
    if mask & 64 as i32 != 0 {
        count |= directional_flip_any(sqnum, 11 as i32, color, oppcol)
    }
    if mask & 32 as i32 != 0 {
        count |=
            directional_flip_any(sqnum, -(10 as i32), color, oppcol)
    }
    if mask & 16 as i32 != 0 {
        count |= directional_flip_any(sqnum, 10 as i32, color, oppcol)
    }
    if mask & 8 as i32 != 0 {
        count |=
            directional_flip_any(sqnum, -(9 as i32), color, oppcol)
    }
    if mask & 4 as i32 != 0 {
        count |= directional_flip_any(sqnum, 9 as i32, color, oppcol)
    }
    if mask & 2 as i32 != 0 {
        count |=
            directional_flip_any(sqnum, -(1 as i32), color, oppcol)
    }
    if mask & 1 as i32 != 0 {
        count |= directional_flip_any(sqnum, 1 as i32, color, oppcol)
    }
    return count;
}
/*
  COMPUTE_THOR_PATTERNS
  Computes the row and column patterns.

*/
pub unsafe fn compute_thor_patterns(mut in_board: *mut i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    i = 0 as i32;
    while i < 8 as i32 {
        thor_row_pattern[i as usize] = 0 as i32;
        thor_col_pattern[i as usize] = 0 as i32;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        j = 0 as i32;
        pos = 10 as i32 * i + 11 as i32;
        while j < 8 as i32 {
            thor_row_pattern[i as usize] +=
                pow3[j as usize] * *in_board.offset(pos as isize);
            thor_col_pattern[j as usize] +=
                pow3[i as usize] * *in_board.offset(pos as isize);
            j += 1;
            pos += 1
        }
        i += 1
    };
}
/*
  GET_CORNER_MASK
  Returns an 32-bit mask for the corner configuration. The rotation
  which minimizes the numerical value is chosen.
  The mask is to be interpreted as follows: There are two bits
  for each corner; 00 means empty, 01 means black and 10 means white.
  The bit blocks are given in the order h8h1a8a1 (MSB to LSB).
  Furthermore, this 8-bit value is put in the leftmost byte if
  all four corners have been played, in the rightmost byte if only
  one corner has been played (obvious generalization for one or two
  corners).
*/
pub unsafe fn get_corner_mask(mut disc_a1: i32,
                          mut disc_a8: i32,
                          mut disc_h1: i32,
                          mut disc_h8: i32)
                          -> u32 {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut mask_a1: i32 = 0;
    let mut mask_a8: i32 = 0;
    let mut mask_h1: i32 = 0;
    let mut mask_h8: i32 = 0;
    let mut out_mask: u32 = 0;
    let mut config: [u32; 8] = [0; 8];
    mask_a1 = 0 as i32;
    if disc_a1 == 0 as i32 {
        mask_a1 = 1 as i32
    } else if disc_a1 == 2 as i32 { mask_a1 = 2 as i32 }
    mask_a8 = 0 as i32;
    if disc_a8 == 0 as i32 {
        mask_a8 = 1 as i32
    } else if disc_a8 == 2 as i32 { mask_a8 = 2 as i32 }
    mask_h1 = 0 as i32;
    if disc_h1 == 0 as i32 {
        mask_h1 = 1 as i32
    } else if disc_h1 == 2 as i32 { mask_h1 = 2 as i32 }
    mask_h8 = 0 as i32;
    if disc_h8 == 0 as i32 {
        mask_h8 = 1 as i32
    } else if disc_h8 == 2 as i32 { mask_h8 = 2 as i32 }
    count = 0 as i32;
    if disc_a1 != 1 as i32 { count += 1 }
    if disc_a8 != 1 as i32 { count += 1 }
    if disc_h1 != 1 as i32 { count += 1 }
    if disc_h8 != 1 as i32 { count += 1 }
    if count == 0 as i32 { return 0 as i32 as u32 }
    config[0 as i32 as usize] =
        (mask_a1 + 4 as i32 * mask_a8 + 16 as i32 * mask_h1 +
            64 as i32 * mask_h8) as u32;
    config[1 as i32 as usize] =
        (mask_a1 + 4 as i32 * mask_h1 + 16 as i32 * mask_a8 +
            64 as i32 * mask_h8) as u32;
    config[2 as i32 as usize] =
        (mask_a8 + 4 as i32 * mask_a1 + 16 as i32 * mask_h8 +
            64 as i32 * mask_h1) as u32;
    config[3 as i32 as usize] =
        (mask_a8 + 4 as i32 * mask_h8 + 16 as i32 * mask_a1 +
            64 as i32 * mask_h1) as u32;
    config[4 as i32 as usize] =
        (mask_h1 + 4 as i32 * mask_h8 + 16 as i32 * mask_a1 +
            64 as i32 * mask_a8) as u32;
    config[5 as i32 as usize] =
        (mask_h1 + 4 as i32 * mask_a1 + 16 as i32 * mask_h8 +
            64 as i32 * mask_a8) as u32;
    config[6 as i32 as usize] =
        (mask_h8 + 4 as i32 * mask_h1 + 16 as i32 * mask_a8 +
            64 as i32 * mask_a1) as u32;
    config[7 as i32 as usize] =
        (mask_h8 + 4 as i32 * mask_a8 + 16 as i32 * mask_h1 +
            64 as i32 * mask_a1) as u32;
    out_mask = config[0 as i32 as usize];
    i = 1 as i32;
    while i < 8 as i32 {
        out_mask =
            if out_mask < config[i as usize] {
                out_mask
            } else { config[i as usize] };
        i += 1
    }
    return out_mask << 8 as i32 * (count - 1 as i32);
}

/*
  TOURNAMENT_NAME
  Returns the name of the INDEXth tournament if available.
*/
pub unsafe fn tournament_name(mut index: i32)
                          -> *const i8 {
    if index < 0 as i32 || index >= tournaments.count {
        return b"<Not available>\x00" as *const u8 as *const i8
    } else {
        return tournaments.name_buffer.offset((26 as i32 * index) as
            isize)
    };
}

/*
  GET_PLAYER_NAME
  Returns the name of the INDEXth player if available.
*/

pub unsafe fn get_player_name(mut index: i32)
                              -> *const i8 {
    if index < 0 as i32 || index >= players.count {
        return b"< Not available >\x00" as *const u8 as *const i8
    } else {
        return players.name_buffer.offset((20 as i32 * index) as
            isize)
    };
}
/*
  GET_PLAYER_COUNT
  Returns the number of players in the database.
*/

pub unsafe fn get_player_count() -> i32 {
    return players.count;
}
/*
  PLAYER_LEX_ORDER
  Returns the index into the lexicographical order of the
  INDEXth player if available, otherwise the last index + 1.
*/
pub unsafe fn player_lex_order(mut index: i32) -> i32 {
    if index < 0 as i32 || index >= players.count {
        return players.count
    } else { return (*players.player_list.offset(index as isize)).lex_order };
}

/*
  GET_TOURNAMENT_NAME
  Returns the name of the INDEXth tournament if available.
*/

pub unsafe fn get_tournament_name(mut index: i32)
                                  -> *const i8 {
    if index < 0 as i32 || index >= tournaments.count {
        return b"< Not available >\x00" as *const u8 as *const i8
    } else {
        return tournaments.name_buffer.offset((26 as i32 * index) as
            isize)
    };
}
/*
  GET_TOURNAMENT_COUNT
  Returns the number of players in the database.
*/

pub unsafe fn get_tournament_count() -> i32 {
    return tournaments.count;
}

/*
  GET_DATABASE_COUNT
  Returns the number of game databases currently loaded.
*/

pub unsafe fn get_database_count() -> i32 {
    return thor_database_count;
}
/*
  GET_DATABASE_INFO
  Fills the vector INFO with the origin years and number of games of
  all game databases loaded.
  Enough memory must have been allocated prior to this function being
  called, that this is the case can be checked by calling GET_DATABASE_COUNT
  above.
*/

pub unsafe fn get_database_info(mut info: *mut DatabaseInfoType) {
    let mut i: i32 = 0;
    let mut change: i32 = 0;
    let mut temp = DatabaseInfoType{year: 0, count: 0,};
    let mut current_db = 0 as *mut DatabaseType;
    current_db = database_head;
    i = 0 as i32;
    while i < thor_database_count {
        (*info.offset(i as isize)).year = (*current_db).prolog.origin_year;
        (*info.offset(i as isize)).count = (*current_db).count;
        current_db = (*current_db).next;
        i += 1
    }
    loop
    /* Sort the list */
    {
        change = 0 as i32;
        i = 0 as i32;
        while i < thor_database_count - 1 as i32 {
            if (*info.offset(i as isize)).year >
                (*info.offset((i + 1 as i32) as isize)).year {
                change = 1 as i32;
                temp = *info.offset(i as isize);
                *info.offset(i as isize) =
                    *info.offset((i + 1 as i32) as isize);
                *info.offset((i + 1 as i32) as isize) = temp
            }
            i += 1
        }
        if !(change != 0) { break ; }
    };
}

/*
  COMPUTE_PARTIAL_HASH
  Computes the primary and secondary hash values for the
  unit element in the rotation group.
*/
pub unsafe fn compute_partial_hash(mut hash_val1: *mut u32,
                               mut hash_val2: *mut u32) {
    let mut i: i32 = 0;
    *hash_val1 = 0 as i32 as u32;
    *hash_val2 = 0 as i32 as u32;
    i = 0 as i32;
    while i < 8 as i32 {
        *hash_val1 ^=
            primary_hash[i as usize][thor_row_pattern[i as usize] as usize];
        *hash_val2 ^=
            secondary_hash[i as usize][thor_row_pattern[i as usize] as usize];
        i += 1
    };
}
/*
  COMPUTE_FULL_PRIMARY_HASH
  COMPUTE_FULL_SECONDARY_HASH
  Compute the primary and secondary hash codes respectively
  for all elements in the rotation group.
  Note: The order of the hash codes must coincide with the
        definitions in INIT_SYMMETRY_MAPS().
*/
pub unsafe fn compute_full_primary_hash(mut hash_val:
                                    *mut u32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 4 as i32 {
        *hash_val.offset(i as isize) = 0 as i32 as u32;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        /* b1 -> b1 */
        *hash_val.offset(0 as i32 as isize) ^=
            primary_hash[i as usize][thor_row_pattern[i as usize] as usize];
        /* b8 -> b1 */
        *hash_val.offset(1 as i32 as isize) ^=
            primary_hash[i as
                usize][thor_row_pattern[(7 as i32 - i) as
                usize] as usize];
        /* a2 -> b1 */
        *hash_val.offset(2 as i32 as isize) ^=
            primary_hash[i as usize][thor_col_pattern[i as usize] as usize];
        /* h2 -> b1 */
        *hash_val.offset(3 as i32 as isize) ^=
            primary_hash[i as
                usize][thor_col_pattern[(7 as i32 - i) as
                usize] as usize];
        i += 1
    }
    /* g1 -> b1 */
    *hash_val.offset(4 as i32 as isize) =
        bit_reverse_32(*hash_val.offset(0 as i32 as isize));
    /* g8 -> b1 */
    *hash_val.offset(5 as i32 as isize) =
        bit_reverse_32(*hash_val.offset(1 as i32 as isize));
    /* a7 -> b1 */
    *hash_val.offset(6 as i32 as isize) =
        bit_reverse_32(*hash_val.offset(2 as i32 as isize));
    /* h7 -> b1 */
    *hash_val.offset(7 as i32 as isize) =
        bit_reverse_32(*hash_val.offset(3 as i32 as isize));
}
pub unsafe fn compute_full_secondary_hash(mut hash_val:
                                      *mut u32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 4 as i32 {
        *hash_val.offset(i as isize) = 0 as i32 as u32;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        /* b1 -> b1 */
        *hash_val.offset(0 as i32 as isize) ^=
            secondary_hash[i as usize][thor_row_pattern[i as usize] as usize];
        /* b8 -> b1 */
        *hash_val.offset(1 as i32 as isize) ^=
            secondary_hash[i as
                usize][thor_row_pattern[(7 as i32 - i)
                as usize] as
                usize];
        /* a2 -> b1 */
        *hash_val.offset(2 as i32 as isize) ^=
            secondary_hash[i as usize][thor_col_pattern[i as usize] as usize];
        /* h2 -> b1 */
        *hash_val.offset(3 as i32 as isize) ^=
            secondary_hash[i as
                usize][thor_col_pattern[(7 as i32 - i)
                as usize] as
                usize];
        i += 1
    }
    /* g1 -> b1 */
    *hash_val.offset(4 as i32 as isize) =
        bit_reverse_32(*hash_val.offset(0 as i32 as isize));
    /* g8 -> b1 */
    *hash_val.offset(5 as i32 as isize) =
        bit_reverse_32(*hash_val.offset(1 as i32 as isize));
    /* a7 -> b1 */
    *hash_val.offset(6 as i32 as isize) =
        bit_reverse_32(*hash_val.offset(2 as i32 as isize));
    /* h7 -> b1 */
    *hash_val.offset(7 as i32 as isize) =
        bit_reverse_32(*hash_val.offset(3 as i32 as isize));
}

/*
  PRIMARY_HASH_LOOKUP
  Checks if any of the rotations of the current pattern set
  match the primary hash code TARGET_HASH.
*/
pub unsafe fn primary_hash_lookup(mut target_hash: u32)
                              -> i32 {
    let mut i: i32 = 0;
    let mut hit_mask: i32 = 0;
    let mut hash_val: [u32; 8] = [0; 8];
    compute_full_primary_hash(hash_val.as_mut_ptr());
    hit_mask = 0 as i32;
    i = 0 as i32;
    while i < 8 as i32 {
        if hash_val[i as usize] == target_hash {
            hit_mask |= (1 as i32) << i
        }
        i += 1
    }
    return hit_mask;
}
/*
  SECONDARY_HASH_LOOKUP
  Checks if any of the rotations of the current pattern set
  match the secondary hash code TARGET_HASH.
*/
pub unsafe fn secondary_hash_lookup(mut target_hash: u32)
                                -> i32 {
    let mut i: i32 = 0;
    let mut hit_mask: i32 = 0;
    let mut hash_val: [u32; 8] = [0; 8];
    compute_full_secondary_hash(hash_val.as_mut_ptr());
    hit_mask = 0 as i32;
    i = 0 as i32;
    while i < 8 as i32 {
        if hash_val[i as usize] == target_hash {
            hit_mask |= (1 as i32) << i
        }
        i += 1
    }
    return hit_mask;
}

/*
  FILTER_DATABASE
  Applies the current filter rules to the database DB.
*/
pub unsafe fn filter_database(mut db: *mut DatabaseType) {
    let mut i: i32 = 0;
    let mut category: i32 = 0;
    let mut passes_filter: i32 = 0;
    let mut year: i32 = 0;
    let mut game = 0 as *mut GameType;
    i = 0 as i32;
    while i < (*db).count {
        game = &mut *(*db).games.offset(i as isize) as *mut GameType;
        passes_filter = 1 as i32;
        /* Apply the tournament filter */
        if passes_filter != 0 &&
            (*tournaments.tournament_list.offset((*game).tournament_no as
                isize)).selected == 0
        {
            passes_filter = 0 as i32
        }
        /* Apply the year filter */
        if passes_filter != 0 {
            year = (*(*game).database).prolog.origin_year;
            if year < filter.first_year || year > filter.last_year {
                passes_filter = 0 as i32
            }
        }
        /* Apply the player filter */
        if passes_filter != 0 {
            match filter.player_filter as u32 {
                0 => {
                    if (*players.player_list.offset((*game).black_no as
                        isize)).selected == 0
                        &&
                        (*players.player_list.offset((*game).white_no as
                            isize)).selected
                            == 0 {
                        passes_filter = 0 as i32
                    }
                }
                1 => {
                    if (*players.player_list.offset((*game).black_no as
                        isize)).selected == 0
                        ||
                        (*players.player_list.offset((*game).white_no as
                            isize)).selected
                            == 0 {
                        passes_filter = 0 as i32
                    }
                }
                2 => {
                    if (*players.player_list.offset((*game).black_no as
                        isize)).selected == 0
                    {
                        passes_filter = 0 as i32
                    }
                }
                3 => {
                    if (*players.player_list.offset((*game).white_no as
                        isize)).selected == 0
                    {
                        passes_filter = 0 as i32
                    }
                }
                _ => { }
            }
        }
        /* Apply the game type filter */
        if passes_filter != 0 {
            if (*players.player_list.offset((*game).black_no as
                isize)).is_program != 0 {
                if (*players.player_list.offset((*game).white_no as
                    isize)).is_program != 0 {
                    category = 4 as i32
                } else { category = 2 as i32 }
            } else if (*players.player_list.offset((*game).white_no as
                isize)).is_program != 0
            {
                category = 2 as i32
            } else { category = 1 as i32 }
            passes_filter = category & filter.game_categories
        }
        (*game).passes_filter = passes_filter as i16;
        i += 1
    };
}
/*
  FILTER_ALL_DATABASES
  Applies the current filter rules to all databases.
*/
pub unsafe fn filter_all_databases() {
    let mut current_db = 0 as *mut DatabaseType;
    current_db = database_head;
    while !current_db.is_null() {
        filter_database(current_db);
        current_db = (*current_db).next
    };
}

/*
  SET_PLAYER_FILTER
  Specify what players to search for. The boolean vector SELECTED
  must contain at least PLAYERS.COUNT values - check with
  GET_PLAYER_COUNT() if necessary.
*/

pub unsafe fn set_player_filter(mut selected: *mut i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < players.count {
        (*players.player_list.offset(i as isize)).selected =
            *selected.offset(i as isize);
        i += 1
    }
    thor_games_filtered = 0 as i32;
}
/*
  SET_PLAYER_FILTER_TYPE
  Specifies whether it suffices for a game to contain one selected
  player or if both players have to be selected for it be displayed.
*/

pub unsafe fn set_player_filter_type(mut player_filter:
                                     PlayerFilterType) {
    filter.player_filter = player_filter;
}
/*
  SET_TOURNAMENT_FILTER
  Specify what tournaments to search for. The boolean vector SELECTED
  must contain at least TOURNAMENTS.COUNT values - check with
  GET_TOURNAMENT_COUNT() if necessary.
*/

pub unsafe fn set_tournament_filter(mut selected:
                                    *mut i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < tournaments.count {
        (*tournaments.tournament_list.offset(i as isize)).selected =
            *selected.offset(i as isize);
        i += 1
    }
    thor_games_filtered = 0 as i32;
}
/*
  SET_YEAR_FILTER
  Specify the interval of years to which the search will be confined.
*/

pub unsafe fn set_year_filter(mut first_year: i32,
                              mut last_year: i32) {
    filter.first_year = first_year;
    filter.last_year = last_year;
    thor_games_filtered = 0 as i32;
}
/*
  SPECIFY_GAME_CATEGORIES
  Specify the types of games in the database that are displayed
  if they match the position probed for. The input is the binary
  OR of the flags for the types enabled.
*/

pub unsafe fn specify_game_categories(mut categories:
                                      i32) {
    if categories != filter.game_categories {
        filter.game_categories = categories;
        thor_games_filtered = 0 as i32
    };
}
/*
  SPECIFY_THOR_SORT_ORDER
  Specifies that in subsequent calls to SORT_THOR_MATCHES,
  the COUNT first elements of SORT_ORDER are to be used
  (in decreasing order of priority).
  Note: If there aren't (at least) COUNT elements at the location
        to which SORT_ORDER points, a crash is likely.
*/

pub unsafe fn specify_thor_sort_order(mut count: i32,
                                      mut sort_order:
                                      *mut i32) {
    let mut i: i32 = 0;
    /* Truncate the input vector if it is too long */
    count = if count < 10 as i32 { count } else { 10 as i32 };
    /* Check if the new order coincides with the old order */
    if count != thor_sort_criteria_count {
        thor_games_sorted = 0 as i32
    } else {
        i = 0 as i32;
        while i < count {
            if *sort_order.offset(i as isize) != thor_sort_order[i as usize] {
                thor_games_sorted = 0 as i32
            }
            i += 1
        }
    }
    thor_sort_criteria_count = count;
    i = 0 as i32;
    while i < count {
        thor_sort_order[i as usize] = *sort_order.offset(i as isize);
        i += 1
    };
}

/*
  RECURSIVE_OPENING_SCAN
  Performs a preorder traversal of the opening tree rooted
  at NODE and checks which opening nodes are compatible
  with the primary and secondary hash codes from the 8 different
  rotations.
*/
pub unsafe fn recursive_opening_scan(mut node: *mut ThorOpeningNode,
                                 mut depth: i32,
                                 mut moves_played: i32,
                                 mut primary_hash_0:
                                 *mut u32,
                                 mut secondary_hash_0:
                                 *mut u32) {
    let mut i: i32 = 0;
    let mut match_0: i32 = 0;
    let mut matching_symmetry: i32 = 0;
    let mut child = 0 as *mut ThorOpeningNode;
    /* Determine the status of the current node */
    if depth < moves_played {
        (*node).matching_symmetry = 0 as i32;
        (*node).current_match = 0 as i32
    } else if depth == moves_played {
        /* Check the hash codes */
        match_0 = 0 as i32;
        matching_symmetry = 0 as i32;
        i = 7 as i32;
        while i >= 0 as i32 {
            if (*node).hash1 == *primary_hash_0.offset(i as isize) &&
                (*node).hash2 == *secondary_hash_0.offset(i as isize) {
                match_0 = 1 as i32;
                matching_symmetry = i
            }
            i -= 1
        }
        if match_0 != 0 {
            (*node).matching_symmetry = matching_symmetry;
            (*node).current_match = 1 as i32
        } else { (*node).current_match = 2 as i32 }
    } else {
        /* depth > moves_played */
        (*node).current_match = (*(*node).parent_node).current_match;
        (*node).matching_symmetry = (*(*node).parent_node).matching_symmetry
    }
    /* Recursively search the childen */
    child = (*node).child_node;
    while !child.is_null() {
        recursive_opening_scan(child, depth + 1 as i32, moves_played,
                               primary_hash_0, secondary_hash_0);
        child = (*child).sibling_node
    };
}

/*
  OPENING_SCAN
  Fills the opening tree with information on how well
  the current pattern configuration matches the openings.
*/
pub unsafe fn opening_scan(mut moves_played: i32) {
    let mut primary_hash_0: [u32; 8] = [0; 8];
    let mut secondary_hash_0: [u32; 8] = [0; 8];
    compute_full_primary_hash(primary_hash_0.as_mut_ptr());
    compute_full_secondary_hash(secondary_hash_0.as_mut_ptr());
    recursive_opening_scan(root_node, 0 as i32, moves_played,
                           primary_hash_0.as_mut_ptr(),
                           secondary_hash_0.as_mut_ptr());
}
/*
  RECURSIVE_FREQUENCY_COUNT
  Recursively fills frequency table FREQ_COUNT which is to contain
  the number of times each move has been played according to the
  trimmed set of openings from the Thor database.
*/
pub unsafe fn recursive_frequency_count(mut node: *mut ThorOpeningNode,
                                    mut freq_count:
                                    *mut i32,
                                    mut depth: i32,
                                    mut moves_played: i32,
                                    mut symmetries:
                                    *mut i32,
                                    mut primary_hash_0:
                                    *mut u32,
                                    mut secondary_hash_0:
                                    *mut u32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut child_move: i32 = 0;
    let mut child = 0 as *mut ThorOpeningNode;
    if depth == moves_played {
        i = 0 as i32;
        while i < 8 as i32 {
            j = *symmetries.offset(i as isize);
            if (*node).hash1 == *primary_hash_0.offset(j as isize) &&
                (*node).hash2 == *secondary_hash_0.offset(j as isize) {
                child_move = (*node).child_move as i32;
                child = (*node).child_node;
                while !child.is_null() {
                    *freq_count.offset(*inv_symmetry_map[j as
                        usize].offset(child_move
                        as
                        isize)
                        as isize) += (*child).frequency;
                    child_move = (*child).sibling_move as i32;
                    child = (*child).sibling_node
                }
                break ;
            } else { i += 1 }
        }
    } else if depth < moves_played {
        child = (*node).child_node;
        while !child.is_null() {
            recursive_frequency_count(child, freq_count,
                                      depth + 1 as i32, moves_played,
                                      symmetries, primary_hash_0,
                                      secondary_hash_0);
            child = (*child).sibling_node
        }
    };
}

/*
  GET_THOR_GAME
  Returns all available information about the INDEXth game
  in the list of matching games generated by DATABASE_SEARCH.
*/

pub unsafe fn get_thor_game(mut index: i32)
                            -> GameInfoType {
    let mut info =
        GameInfoType{black_name: 0 as *const i8,
            white_name: 0 as *const i8,
            tournament: 0 as *const i8,
            year: 0,
            black_actual_score: 0,
            black_corrected_score: 0,};
    let mut game = 0 as *mut GameType;
    if index < 0 as i32 || index >= thor_search.match_count {
        /* Bad index, so fill with empty values */
        info.black_name = b"\x00" as *const u8 as *const i8;
        info.white_name = b"\x00" as *const u8 as *const i8;
        info.tournament = b"\x00" as *const u8 as *const i8;
        info.year = 0 as i32;
        info.black_actual_score = 32 as i32;
        info.black_corrected_score = 32 as i32
    } else {
        /* Copy name fields etc */
        game = *thor_search.match_list.offset(index as isize);
        info.black_name = get_player_name((*game).black_no as i32);
        info.white_name = get_player_name((*game).white_no as i32);
        info.tournament =
            tournament_name((*game).tournament_no as i32);
        info.year = (*(*game).database).prolog.origin_year;
        info.black_actual_score = (*game).actual_black_score as i32;
        info.black_corrected_score =
            (*game).perfect_black_score as i32
    }
    return info;
}

/*
  GET_THOR_GAME_MOVE_COUNT
  Returns the number of moves in the INDEXth game
  in the list of matching games generated by DATABASE_SEARCH.
*/

pub unsafe fn get_thor_game_move_count(mut index: i32)
                                       -> i32 {
    if index < 0 as i32 || index >= thor_search.match_count {
        /* Bad index */
        return -(1 as i32)
    } else {
        return (**thor_search.match_list.offset(index as isize)).move_count as
            i32
    };
}

/*
  GET_TOTAL_GAME_COUNT
  GET_MATCH_COUNT
  GET_BLACK_WIN_COUNT
  GET_DRAW_COUNT
  GET_WHITE_WIN_COUNT
  GET_BLACK_MEDIAN_SCORE
  GET_AVERAGE_BLACK_SCORE
  GET_MOVE_FREQUENCY
  GET_MOVE_WIN_RATE
  Accessor functions which return statistics from the last
  query to DATABASE_SEARCH.
*/

pub unsafe fn get_total_game_count() -> i32 {
    return thor_game_count;
}

pub unsafe fn get_match_count() -> i32 {
    return thor_search.match_count;
}

pub unsafe fn get_black_win_count() -> i32 {
    return thor_search.black_wins;
}

pub unsafe fn get_draw_count() -> i32 {
    return thor_search.draws;
}

pub unsafe fn get_white_win_count() -> i32 {
    return thor_search.white_wins;
}

pub unsafe fn get_black_median_score() -> i32 {
    return thor_search.median_black_score;
}

pub unsafe fn get_black_average_score() -> f64 {
    return thor_search.average_black_score;
}

pub unsafe fn get_move_frequency(mut move_0: i32)
                                 -> i32 {
    return thor_search.next_move_frequency[move_0 as usize];
}

pub unsafe fn get_move_win_rate(mut move_0: i32)
                                -> f64 {
    if thor_search.next_move_frequency[move_0 as usize] == 0 as i32 {
        return 0.0f64
    } else {
        return thor_search.next_move_score[move_0 as usize] /
            thor_search.next_move_frequency[move_0 as usize] as
                f64
    };
}

/*
  INIT_MOVE_MASKS
  Initializes the shape bit masks for each of the possible moves.
*/
pub unsafe fn init_move_masks() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut index: i32 = 0;
    i = 0 as i32;
    while i < 4 as i32 {
        j = 0 as i32;
        pos = 10 as i32 * i + 11 as i32;
        while j < 8 as i32 {
            index = 8 as i32 * i + j;
            move_mask_lo[pos as usize] =
                ((1 as i32) << index) as u32;
            move_mask_hi[pos as usize] = 0 as i32 as u32;
            unmove_mask_lo[pos as usize] =
                !((1 as i32) << index) as u32;
            unmove_mask_hi[pos as usize] =
                !(0 as i32) as u32;
            j += 1;
            pos += 1
        }
        i += 1
    }
    i = 0 as i32;
    while i < 4 as i32 {
        j = 0 as i32;
        pos = 10 as i32 * i + 51 as i32;
        while j < 8 as i32 {
            index = 8 as i32 * i + j;
            move_mask_lo[pos as usize] = 0 as i32 as u32;
            move_mask_hi[pos as usize] =
                ((1 as i32) << index) as u32;
            unmove_mask_lo[pos as usize] =
                !(0 as i32) as u32;
            unmove_mask_hi[pos as usize] =
                !((1 as i32) << index) as u32;
            j += 1;
            pos += 1
        }
        i += 1
    };
}
/*
  CALCULATE_OPENING_FREQUENCY
  Calculates and returns the number of lines in the Thor opening base
  that match the line defined by NODE.
*/
pub unsafe fn calculate_opening_frequency(mut node:
                                      *mut ThorOpeningNode)
                                      -> i32 {
    let mut sum: i32 = 0;
    let mut child = 0 as *mut ThorOpeningNode;
    child = (*node).child_node;
    if child.is_null() {
        return (*node).frequency
    } else {
        sum = 0 as i32;
        loop  {
            sum += calculate_opening_frequency(child);
            child = (*child).sibling_node;
            if child.is_null() { break ; }
        }
        (*node).frequency = sum;
        return sum
    };
}

/*
  GET_THOR_GAME_SIZE
  Returns the amount of memory which each game in the database takes.
*/

pub unsafe fn get_thor_game_size() -> i32 {
    return ::core::mem::size_of::<GameType>() as u64 as i32;
}
/*
  INIT_SYMMETRY_MAPS
  Initializes the mappings which the 8 elements in the board
  symmetry group induce (and their inverses).
  Note: The order of the mappings must coincide with the order
        in which they are calculated in COMPUTE_FULL_PRIMARY_HASH()
    and COMPUTE_FULL_SECONDARY_HASH().
*/
pub unsafe fn init_symmetry_maps() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut pos: i32 = 0;
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            pos = 10 as i32 * i + j;
            b1_b1_map[pos as usize] = pos;
            g1_b1_map[pos as usize] =
                10 as i32 * i + (9 as i32 - j);
            g8_b1_map[pos as usize] =
                10 as i32 * (9 as i32 - i) +
                    (9 as i32 - j);
            b8_b1_map[pos as usize] =
                10 as i32 * (9 as i32 - i) + j;
            a2_b1_map[pos as usize] = 10 as i32 * j + i;
            a7_b1_map[pos as usize] =
                10 as i32 * j + (9 as i32 - i);
            h7_b1_map[pos as usize] =
                10 as i32 * (9 as i32 - j) +
                    (9 as i32 - i);
            h2_b1_map[pos as usize] =
                10 as i32 * (9 as i32 - j) + i;
            j += 1
        }
        i += 1
    }
    symmetry_map[0 as i32 as usize] = b1_b1_map.as_mut_ptr();
    inv_symmetry_map[0 as i32 as usize] = b1_b1_map.as_mut_ptr();
    symmetry_map[1 as i32 as usize] = b8_b1_map.as_mut_ptr();
    inv_symmetry_map[1 as i32 as usize] = b8_b1_map.as_mut_ptr();
    symmetry_map[2 as i32 as usize] = a2_b1_map.as_mut_ptr();
    inv_symmetry_map[2 as i32 as usize] = a2_b1_map.as_mut_ptr();
    symmetry_map[3 as i32 as usize] = h2_b1_map.as_mut_ptr();
    inv_symmetry_map[3 as i32 as usize] = a7_b1_map.as_mut_ptr();
    symmetry_map[4 as i32 as usize] = g1_b1_map.as_mut_ptr();
    inv_symmetry_map[4 as i32 as usize] = g1_b1_map.as_mut_ptr();
    symmetry_map[5 as i32 as usize] = g8_b1_map.as_mut_ptr();
    inv_symmetry_map[5 as i32 as usize] = g8_b1_map.as_mut_ptr();
    symmetry_map[6 as i32 as usize] = a7_b1_map.as_mut_ptr();
    inv_symmetry_map[6 as i32 as usize] = h2_b1_map.as_mut_ptr();
    symmetry_map[7 as i32 as usize] = h7_b1_map.as_mut_ptr();
    inv_symmetry_map[7 as i32 as usize] = h7_b1_map.as_mut_ptr();
    i = 0 as i32;
    while i < 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            k = 1 as i32;
            while k <= 8 as i32 {
                pos = 10 as i32 * j + k;
                if *inv_symmetry_map[i as
                    usize].offset(*symmetry_map[i as
                    usize].offset(pos
                    as
                    isize)
                    as isize) != pos {
                    fatal_error(b"Error in map %d: inv(map(%d))=%d\n\x00" as
                                    *const u8 as *const i8, i, pos,
                                *inv_symmetry_map[i as
                                    usize].offset(*symmetry_map[i
                                    as
                                    usize].offset(pos
                                    as
                                    isize)
                                    as
                                    isize));
                }
                k += 1
            }
            j += 1
        }
        i += 1
    };
}