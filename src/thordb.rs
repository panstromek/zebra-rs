use crate::src::libc;
use crate::src::stubs::{puts, strlen, abs, fputs, free, printf, qsort, fprintf, fclose, fopen, fread, strchr, strcmp, tolower};
use crate::src::safemem::{safe_malloc, safe_realloc};
use crate::src::error::fatal_error;
use crate::src::bitboard::bit_reverse_32;
use crate::src::myrandom::my_random;
use crate::src::patterns::pow3;
use crate::src::moves::dir_mask;
use crate::src::zebra::_IO_FILE;

pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type __compar_fn_t
    =
    Option<unsafe fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> i32>;
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
static mut thor_game_count: i32 = 0;
static mut thor_database_count: i32 = 0;
static mut thor_side_to_move: i32 = 0;
static mut thor_sort_criteria_count: i32 = 0;
static mut thor_games_sorted: i32 = 0;
static mut thor_games_filtered: i32 = 0;
static mut thor_row_pattern: [i32; 8] = [0; 8];
static mut thor_col_pattern: [i32; 8] = [0; 8];
static mut thor_board: [i32; 100] = [0; 100];
static mut b1_b1_map: [i32; 100] = [0; 100];
static mut g1_b1_map: [i32; 100] = [0; 100];
static mut g8_b1_map: [i32; 100] = [0; 100];
static mut b8_b1_map: [i32; 100] = [0; 100];
static mut a2_b1_map: [i32; 100] = [0; 100];
static mut a7_b1_map: [i32; 100] = [0; 100];
static mut h7_b1_map: [i32; 100] = [0; 100];
static mut h2_b1_map: [i32; 100] = [0; 100];
static mut primary_hash: [[u32; 6561]; 8] = [[0; 6561]; 8];
static mut secondary_hash: [[u32; 6561]; 8] = [[0; 6561]; 8];
static mut symmetry_map: [*mut i32; 8] =
    [0 as *const i32 as *mut i32; 8];
static mut inv_symmetry_map: [*mut i32; 8] =
    [0 as *const i32 as *mut i32; 8];
static mut move_mask_hi: [u32; 100] = [0; 100];
static mut move_mask_lo: [u32; 100] = [0; 100];
static mut unmove_mask_hi: [u32; 100] = [0; 100];
static mut unmove_mask_lo: [u32; 100] = [0; 100];
static mut database_head: *mut DatabaseType =
    0 as *const DatabaseType as *mut DatabaseType;
static mut players: PlayerDatabaseType =
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
static mut thor_search: SearchResultType =
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
static mut tournaments: TournamentDatabaseType =
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
static mut root_node: *mut ThorOpeningNode =
    0 as *const ThorOpeningNode as *mut ThorOpeningNode;
static mut default_sort_order: [i32; 5] =
    [2 as i32, 3 as i32, 1 as i32, 5 as i32,
     4 as i32];
static mut thor_sort_order: [i32; 10] = [0; 10];
static mut filter: FilterType =
    FilterType{game_categories: 0,
               first_year: 0,
               last_year: 0,
               player_filter: EitherSelectedFilter,};
/*
  CLEAR_THOR_BOARD
*/
unsafe fn clear_thor_board() {
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
unsafe fn prepare_thor_board() {
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
unsafe fn directional_flip_count(mut sq: i32,
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
unsafe fn directional_flip_any(mut sq: i32,
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
unsafe fn count_flips(mut sqnum: i32,
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
unsafe fn any_flips(mut sqnum: i32, mut color: i32,
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
unsafe fn compute_thor_patterns(mut in_board: *mut i32) {
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
unsafe fn get_corner_mask(mut disc_a1: i32,
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
  PLAY_THROUGH_GAME
  Play the MAX_MOVES first moves of GAME and update THOR_BOARD
  and THOR_SIDE_TO_MOVE to represent the position after those moves.
*/
unsafe fn play_through_game(mut game: *mut GameType,
                                       mut max_moves: i32)
 -> i32 {
    let mut i: i32 = 0;
    let mut move_0: i32 = 0;
    let mut flipped: i32 = 0;
    clear_thor_board();
    thor_side_to_move = 0 as i32;
    i = 0 as i32;
    while i < max_moves {
        move_0 = abs((*game).moves[i as usize] as i32);
        flipped =
            any_flips(move_0, thor_side_to_move,
                      0 as i32 + 2 as i32 -
                          thor_side_to_move);
        if flipped != 0 {
            thor_board[move_0 as usize] = thor_side_to_move;
            thor_side_to_move =
                0 as i32 + 2 as i32 - thor_side_to_move
        } else {
            thor_side_to_move =
                0 as i32 + 2 as i32 - thor_side_to_move;
            flipped =
                any_flips(move_0, thor_side_to_move,
                          0 as i32 + 2 as i32 -
                              thor_side_to_move);
            if flipped != 0 {
                thor_board[move_0 as usize] = thor_side_to_move;
                thor_side_to_move =
                    0 as i32 + 2 as i32 - thor_side_to_move
            } else { return 0 as i32 }
        }
        i += 1
    }
    return 1 as i32;
}
/*
  PREPARE_GAME
  Performs off-line analysis of GAME to speed up subsequent requests.
  The main result is that the number of black discs on the board after
  each of the moves is stored.
*/
unsafe fn prepare_game(mut game: *mut GameType) {
    let mut i: i32 = 0;
    let mut move_0: i32 = 0;
    let mut done: i32 = 0;
    let mut flipped: i32 = 0;
    let mut opening_match: i32 = 0;
    let mut moves_played: i32 = 0;
    let mut disc_count: [i32; 3] = [0; 3];
    let mut corner_descriptor: u32 = 0;
    let mut opening = 0 as *mut ThorOpeningNode;
    let mut child = 0 as *mut ThorOpeningNode;
    /* Play through the game and count the number of black discs
       at each stage. */
    clear_thor_board();
    disc_count[2 as i32 as usize] = 2 as i32;
    disc_count[0 as i32 as usize] =
        disc_count[2 as i32 as usize];
    thor_side_to_move = 0 as i32;
    corner_descriptor = 0 as i32 as u32;
    moves_played = 0 as i32;
    done = 0 as i32;
    loop  {
        /* Store the number of black discs. */
        (*game).black_disc_count[moves_played as usize] =
            disc_count[0 as i32 as usize] as i8;
        /* Make the move, update the board and disc count,
           and change the sign for white moves */
        move_0 = (*game).moves[moves_played as usize] as i32;
        flipped =
            count_flips(move_0, thor_side_to_move,
                        0 as i32 + 2 as i32 -
                            thor_side_to_move);
        if flipped != 0 {
            thor_board[move_0 as usize] = thor_side_to_move;
            disc_count[thor_side_to_move as usize] +=
                flipped + 1 as i32;
            disc_count[(0 as i32 + 2 as i32 -
                            thor_side_to_move) as usize] -= flipped;
            if thor_side_to_move == 2 as i32 {
                (*game).moves[moves_played as usize] =
                    -((*game).moves[moves_played as usize] as i32) as
                        i8
            }
            thor_side_to_move =
                0 as i32 + 2 as i32 - thor_side_to_move;
            moves_played += 1
        } else {
            thor_side_to_move =
                0 as i32 + 2 as i32 - thor_side_to_move;
            flipped =
                count_flips(move_0, thor_side_to_move,
                            0 as i32 + 2 as i32 -
                                thor_side_to_move);
            if flipped != 0 {
                thor_board[move_0 as usize] = thor_side_to_move;
                disc_count[thor_side_to_move as usize] +=
                    flipped + 1 as i32;
                disc_count[(0 as i32 + 2 as i32 -
                                thor_side_to_move) as usize] -= flipped;
                if thor_side_to_move == 2 as i32 {
                    (*game).moves[moves_played as usize] =
                        -((*game).moves[moves_played as usize] as i32)
                            as i8
                }
                thor_side_to_move =
                    0 as i32 + 2 as i32 - thor_side_to_move;
                moves_played += 1
            } else { done = 1 as i32 }
        }
        /* Update the corner descriptor if necessary */
        if move_0 == 11 as i32 || move_0 == 18 as i32 ||
               move_0 == 81 as i32 || move_0 == 88 as i32 {
            corner_descriptor |=
                get_corner_mask(thor_board[11 as i32 as usize],
                                thor_board[81 as i32 as usize],
                                thor_board[18 as i32 as usize],
                                thor_board[88 as i32 as usize])
        }
        if !(done == 0 && moves_played < 60 as i32) { break ; }
    }
    (*game).black_disc_count[moves_played as usize] =
        disc_count[0 as i32 as usize] as i8;
    (*game).move_count = moves_played as i16;
    i = moves_played + 1 as i32;
    while i <= 60 as i32 {
        (*game).black_disc_count[i as usize] =
            -(1 as i32) as i8;
        i += 1
    }
    /* Find the longest opening which coincides with the game */
    opening = root_node;
    i = 0 as i32;
    opening_match = 1 as i32;
    while opening_match != 0 {
        move_0 = (*opening).child_move as i32;
        child = (*opening).child_node;
        while !child.is_null() &&
                  move_0 != abs((*game).moves[i as usize] as i32) {
            move_0 = (*child).sibling_move as i32;
            child = (*child).sibling_node
        }
        if child.is_null() {
            opening_match = 0 as i32
        } else { opening = child; i += 1 }
    }
    (*game).opening = opening;
    /* Initialize the shape state */
    (*game).shape_lo =
        ((3 as i32) << 27 as i32) as u32;
    (*game).shape_hi =
        ((3 as i32) << 3 as i32) as u32;
    (*game).shape_state_hi = 0 as i32 as i16;
    (*game).shape_state_lo = 0 as i32 as i16;
    /* Store the corner descriptor */
    (*game).corner_descriptor = corner_descriptor;
}
/*
  GET_INT_8
  Reads an 8-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
unsafe fn get_int_8(mut stream: *mut FILE, mut value: *mut int_8)
 -> i32 {
    let mut actually_read: i32 = 0;
    actually_read =
        fread(value as *mut libc::c_void,
              ::std::mem::size_of::<int_8>() as u64,
              1 as i32 as size_t, stream) as i32;
    return (actually_read == 1 as i32) as i32;
}
/*
  GET_INT_16
  Reads a 16-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
unsafe fn get_int_16(mut stream: *mut FILE, mut value: *mut int_16)
 -> i32 {
    let mut actually_read: i32 = 0;
    actually_read =
        fread(value as *mut libc::c_void,
              ::std::mem::size_of::<int_16>() as u64,
              1 as i32 as size_t, stream) as i32;
    return (actually_read == 1 as i32) as i32;
}
/*
  GET_INT_32
  Reads a 32-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
unsafe fn get_int_32(mut stream: *mut FILE, mut value: *mut int_32)
 -> i32 {
    let mut actually_read: i32 = 0;
    actually_read =
        fread(value as *mut libc::c_void,
              ::std::mem::size_of::<int_32>() as u64,
              1 as i32 as size_t, stream) as i32;
    return (actually_read == 1 as i32) as i32;
}
/*
  TOURNAMENT_NAME
  Returns the name of the INDEXth tournament if available.
*/
unsafe fn tournament_name(mut index: i32)
 -> *const i8 {
    if index < 0 as i32 || index >= tournaments.count {
        return b"<Not available>\x00" as *const u8 as *const i8
    } else {
        return tournaments.name_buffer.offset((26 as i32 * index) as
                                                  isize)
    };
}
/*
  TOURNAMENT_LEX_ORDER
  Returns the index into the lexicographical order of the
  INDEXth tournament if available, otherwise the last
  index + 1.
*/
unsafe fn tournament_lex_order(mut index: i32)
 -> i32 {
    if index < 0 as i32 || index >= tournaments.count {
        return tournaments.count
    } else {
        return (*tournaments.tournament_list.offset(index as isize)).lex_order
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
unsafe fn player_lex_order(mut index: i32) -> i32 {
    if index < 0 as i32 || index >= players.count {
        return players.count
    } else { return (*players.player_list.offset(index as isize)).lex_order };
}
/*
  READ_PROLOG
  Reads the prolog from STREAM into PROLOG. As the prolog is common
  for all the three database types (game, player, tournament) also
  values which aren't used are read.
  Returns TRUE upon success, otherwise FALSE.
*/
unsafe fn read_prolog(mut stream: *mut FILE,
                                 mut prolog: *mut PrologType) -> i32 {
    let mut success: i32 = 0;
    let mut byte_val: int_8 = 0;
    let mut word_val: int_16 = 0;
    let mut longint_val: int_32 = 0;
    success = get_int_8(stream, &mut byte_val);
    (*prolog).creation_century = byte_val as i32;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            i32;
    (*prolog).creation_year = byte_val as i32;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            i32;
    (*prolog).creation_month = byte_val as i32;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            i32;
    (*prolog).creation_day = byte_val as i32;
    success =
        (success != 0 && get_int_32(stream, &mut longint_val) != 0) as
            i32;
    (*prolog).game_count = longint_val;
    success =
        (success != 0 && get_int_16(stream, &mut word_val) != 0) as
            i32;
    (*prolog).item_count = word_val as i32;
    success =
        (success != 0 && get_int_16(stream, &mut word_val) != 0) as
            i32;
    (*prolog).origin_year = word_val as i32;
    success =
        (success != 0 && get_int_32(stream, &mut longint_val) != 0) as
            i32;
    (*prolog).reserved = longint_val;
    return success;
}
/*
  THOR_COMPARE_TOURNAMENTS
  Lexicographically compares the names of two tournaments
  represented by pointers.
*/
unsafe fn thor_compare_tournaments(mut t1: *const libc::c_void,
                                              mut t2: *const libc::c_void)
 -> i32 {
    let mut tournament1 = *(t1 as *mut *mut TournamentType);
    let mut tournament2 = *(t2 as *mut *mut TournamentType);
    return strcmp((*tournament1).name, (*tournament2).name);
}
/*
  SORT_TOURNAMENT_DATABASE
  Computes the lexicographic order of all tournaments in the database.
*/
unsafe fn sort_tournament_database() {
    let mut tournament_buffer = 0 as *mut *mut TournamentType;
    let mut i: i32 = 0;
    tournament_buffer =
        safe_malloc((tournaments.count as
                         u64).wrapping_mul(::std::mem::size_of::<*mut TournamentType>()
                                                         as u64)) as
            *mut *mut TournamentType;
    i = 0 as i32;
    while i < tournaments.count {
        let ref mut fresh0 = *tournament_buffer.offset(i as isize);
        *fresh0 =
            &mut *tournaments.tournament_list.offset(i as isize) as
                *mut TournamentType;
        i += 1
    }
    qsort(tournament_buffer as *mut libc::c_void, tournaments.count as size_t,
          ::std::mem::size_of::<*mut TournamentType>() as u64,
          Some(thor_compare_tournaments as
                   unsafe fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> i32));
    i = 0 as i32;
    while i < tournaments.count {
        (**tournament_buffer.offset(i as isize)).lex_order = i;
        i += 1
    }
    free(tournament_buffer as *mut libc::c_void);
}
/*
  READ_TOURNAMENT_DATABASE
  Reads the tournament database from FILE_NAME.
  Returns TRUE if all went well, otherwise FALSE.
*/

pub unsafe fn read_tournament_database(mut file_name:
                                                      *const i8)
 -> i32 {
    let mut stream = 0 as *mut FILE;
    let mut i: i32 = 0;
    let mut success: i32 = 0;
    let mut actually_read: i32 = 0;
    let mut buffer_size: i32 = 0;
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() { return 0 as i32 }
    if read_prolog(stream, &mut tournaments.prolog) == 0 {
        fclose(stream);
        return 0 as i32
    }
    tournaments.count = tournaments.prolog.item_count;
    buffer_size = 26 as i32 * tournaments.prolog.item_count;
    tournaments.name_buffer =
        safe_realloc(tournaments.name_buffer as *mut libc::c_void,
                     buffer_size as size_t) as *mut i8;
    actually_read =
        fread(tournaments.name_buffer as *mut libc::c_void,
              1 as i32 as size_t, buffer_size as size_t, stream) as
            i32;
    success = (actually_read == buffer_size) as i32;
    fclose(stream);
    if success != 0 {
        tournaments.tournament_list =
            safe_realloc(tournaments.tournament_list as *mut libc::c_void,
                         (tournaments.count as
                              u64).wrapping_mul(::std::mem::size_of::<TournamentType>()
                                                              as
                                                              u64))
                as *mut TournamentType;
        i = 0 as i32;
        while i < tournaments.count {
            let ref mut fresh1 =
                (*tournaments.tournament_list.offset(i as isize)).name;
            *fresh1 = tournament_name(i);
            (*tournaments.tournament_list.offset(i as isize)).selected =
                1 as i32;
            i += 1
        }
        sort_tournament_database();
        thor_games_sorted = 0 as i32;
        thor_games_filtered = 0 as i32
    }
    return success;
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
  THOR_COMPARE_PLAYERS
  Lexicographically compares the names of two players
  represented by pointers.
*/
unsafe fn thor_compare_players(mut p1: *const libc::c_void,
                                          mut p2: *const libc::c_void)
 -> i32 {
    let mut ch: i8 = 0;
    let mut buffer1: [i8; 20] = [0; 20];
    let mut buffer2: [i8; 20] = [0; 20];
    let mut i: i32 = 0;
    let mut player1 = *(p1 as *mut *mut PlayerType);
    let mut player2 = *(p2 as *mut *mut PlayerType);
    i = 0 as i32;
    loop  {
        ch = *(*player1).name.offset(i as isize);
        buffer1[i as usize] = tolower(ch as i32) as i8;
        i += 1;
        if !(ch as i32 != 0 as i32) { break ; }
    }
    if buffer1[0 as i32 as usize] as i32 == '?' as i32 {
        /* Put unknown players LAST */
        buffer1[0 as i32 as usize] = '~' as i32 as i8
    }
    i = 0 as i32;
    loop  {
        ch = *(*player2).name.offset(i as isize);
        buffer2[i as usize] = tolower(ch as i32) as i8;
        i += 1;
        if !(ch as i32 != 0 as i32) { break ; }
    }
    if buffer2[0 as i32 as usize] as i32 == '?' as i32 {
        /* Put unknown players LAST */
        buffer2[0 as i32 as usize] = '~' as i32 as i8
    }
    return strcmp(buffer1.as_mut_ptr(), buffer2.as_mut_ptr());
}
/*
  SORT_PLAYER_DATABASE
  Computes the lexicographic order of all players in the database.
*/
unsafe fn sort_player_database() {
    let mut player_buffer = 0 as *mut *mut PlayerType;
    let mut i: i32 = 0;
    player_buffer =
        safe_malloc((players.count as
                         u64).wrapping_mul(::std::mem::size_of::<*mut PlayerType>()
                                                         as u64)) as
            *mut *mut PlayerType;
    i = 0 as i32;
    while i < players.count {
        let ref mut fresh2 = *player_buffer.offset(i as isize);
        *fresh2 =
            &mut *players.player_list.offset(i as isize) as *mut PlayerType;
        i += 1
    }
    qsort(player_buffer as *mut libc::c_void, players.count as size_t,
          ::std::mem::size_of::<*mut PlayerType>() as u64,
          Some(thor_compare_players as
                   unsafe fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> i32));
    i = 0 as i32;
    while i < players.count {
        (**player_buffer.offset(i as isize)).lex_order = i;
        i += 1
    }
    free(player_buffer as *mut libc::c_void);
}
/*
  READ_PLAYER_DATABASE
  Reads the player database from FILE_NAME.
  Returns TRUE if all went well, otherwise FALSE.
*/

pub unsafe fn read_player_database(mut file_name:
                                                  *const i8)
 -> i32 {
    let mut stream = 0 as *mut FILE;
    let mut i: i32 = 0;
    let mut success: i32 = 0;
    let mut actually_read: i32 = 0;
    let mut buffer_size: i32 = 0;
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() { return 0 as i32 }
    if read_prolog(stream, &mut players.prolog) == 0 {
        fclose(stream);
        return 0 as i32
    }
    players.count = players.prolog.item_count;
    buffer_size = 20 as i32 * players.count;
    players.name_buffer =
        safe_realloc(players.name_buffer as *mut libc::c_void,
                     buffer_size as size_t) as *mut i8;
    actually_read =
        fread(players.name_buffer as *mut libc::c_void,
              1 as i32 as size_t, buffer_size as size_t, stream) as
            i32;
    success = (actually_read == buffer_size) as i32;
    fclose(stream);
    if success != 0 {
        players.player_list =
            safe_realloc(players.player_list as *mut libc::c_void,
                         (players.count as
                              u64).wrapping_mul(::std::mem::size_of::<PlayerType>()
                                                              as
                                                              u64))
                as *mut PlayerType;
        i = 0 as i32;
        while i < players.count {
            let ref mut fresh3 =
                (*players.player_list.offset(i as isize)).name;
            *fresh3 = get_player_name(i);
            /* By convention, names of computer programs always contain
            parenthesis within which the name of the creator of the
             program is given. E.g. "Zebra (andersson)", "Sethos()". */
            if !strchr((*players.player_list.offset(i as isize)).name,
                       '(' as i32).is_null() {
                (*players.player_list.offset(i as isize)).is_program =
                    1 as i32
            } else {
                (*players.player_list.offset(i as isize)).is_program =
                    0 as i32
            }
            (*players.player_list.offset(i as isize)).selected =
                1 as i32;
            i += 1
        }
        sort_player_database();
        thor_games_sorted = 0 as i32;
        thor_games_filtered = 0 as i32
    }
    return success;
}
/*
  READ_GAME
  Reads a game from STREAM in GAME and prepares the game
  for database questions. Returns TRUE upon success,
  otherwise FALSE.
*/
unsafe fn read_game(mut stream: *mut FILE, mut game: *mut GameType)
 -> i32 {
    let mut success: i32 = 0;
    let mut actually_read: i32 = 0;
    let mut byte_val: int_8 = 0;
    let mut word_val: int_16 = 0;
    success = get_int_16(stream, &mut word_val);
    (*game).tournament_no = word_val;
    success =
        (success != 0 && get_int_16(stream, &mut word_val) != 0) as
            i32;
    (*game).black_no = word_val;
    success =
        (success != 0 && get_int_16(stream, &mut word_val) != 0) as
            i32;
    (*game).white_no = word_val;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            i32;
    (*game).actual_black_score = byte_val as i16;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            i32;
    (*game).perfect_black_score = byte_val as i16;
    actually_read =
        fread(&mut (*game).moves as *mut [i8; 60] as
                  *mut libc::c_void, 1 as i32 as size_t,
              60 as i32 as size_t, stream) as i32;
    prepare_game(game);
    return (success != 0 && actually_read == 60 as i32) as
               i32;
}
/*
  READ_GAME_DATABASE
  Reads a game database from FILE_NAME.
*/

pub unsafe fn read_game_database(mut file_name:
                                                *const i8)
 -> i32 {
    let mut stream = 0 as *mut FILE;
    let mut i: i32 = 0;
    let mut success: i32 = 0;
    let mut old_database_head = 0 as *mut DatabaseType;
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() { return 0 as i32 }
    old_database_head = database_head;
    database_head =
        safe_malloc(::std::mem::size_of::<DatabaseType>() as u64) as
            *mut DatabaseType;
    (*database_head).next = old_database_head;
    if read_prolog(stream, &mut (*database_head).prolog) == 0 {
        fclose(stream);
        return 0 as i32
    }
    success = 1 as i32;
    (*database_head).count = (*database_head).prolog.game_count;
    (*database_head).games =
        safe_malloc(((*database_head).count as
                         u64).wrapping_mul(::std::mem::size_of::<GameType>()
                                                         as u64)) as
            *mut GameType;
    i = 0 as i32;
    while i < (*database_head).count {
        success =
            (success != 0 &&
                 read_game(stream,
                           &mut *(*database_head).games.offset(i as isize)) !=
                     0) as i32;
        let ref mut fresh4 =
            (*(*database_head).games.offset(i as isize)).database;
        *fresh4 = database_head;
        i += 1
    }
    thor_database_count += 1;
    thor_game_count += (*database_head).count;
    thor_games_sorted = 0 as i32;
    thor_games_filtered = 0 as i32;
    fclose(stream);
    return success;
}
/*
  GAME_DATABASE_ALREADY_LOADED
  Checks if the game database in FILE_NAME already exists in memory
  (the only thing actually checked is the prolog but this suffices
  according to the specification of the database format).
*/

pub unsafe fn game_database_already_loaded(mut file_name:
                                                          *const i8)
 -> i32 {
    let mut stream = 0 as *mut FILE;
    let mut current_db = 0 as *mut DatabaseType;
    let mut new_prolog =
        PrologType{creation_century: 0,
                   creation_year: 0,
                   creation_month: 0,
                   creation_day: 0,
                   game_count: 0,
                   item_count: 0,
                   origin_year: 0,
                   reserved: 0,};
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() { return 0 as i32 }
    if read_prolog(stream, &mut new_prolog) == 0 {
        fclose(stream);
        return 0 as i32
    }
    fclose(stream);
    current_db = database_head;
    while !current_db.is_null() {
        if (*current_db).prolog.creation_century ==
               new_prolog.creation_century &&
               (*current_db).prolog.creation_year == new_prolog.creation_year
               &&
               (*current_db).prolog.creation_month ==
                   new_prolog.creation_month &&
               (*current_db).prolog.creation_day == new_prolog.creation_day &&
               (*current_db).prolog.game_count == new_prolog.game_count &&
               (*current_db).prolog.item_count == new_prolog.item_count &&
               (*current_db).prolog.origin_year ==
                   (*current_db).prolog.origin_year {
            return 1 as i32
        }
        current_db = (*current_db).next
    }
    return 0 as i32;
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
  PRINT_GAME
  Outputs the information about the game GAME to STREAM.
  The flag DISPLAY_MOVES specifies if the moves of the
  game are to be output or not.
*/
unsafe fn print_game(mut stream: *mut FILE,
                                mut game: *mut GameType,
                                mut display_moves: i32) {
    let mut i: i32 = 0;
    fprintf(stream, b"%s  %d\n\x00" as *const u8 as *const i8,
            tournament_name((*game).tournament_no as i32),
            (*(*game).database).prolog.origin_year);
    fprintf(stream, b"%s %s %s\n\x00" as *const u8 as *const i8,
            get_player_name((*game).black_no as i32),
            b"vs\x00" as *const u8 as *const i8,
            get_player_name((*game).white_no as i32));
    fprintf(stream, b"%d - %d   \x00" as *const u8 as *const i8,
            (*game).actual_black_score as i32,
            64 as i32 - (*game).actual_black_score as i32);
    fprintf(stream,
            b"[ %d - %d %s ]\n\x00" as *const u8 as *const i8,
            (*game).perfect_black_score as i32,
            64 as i32 - (*game).perfect_black_score as i32,
            b"perfect\x00" as *const u8 as *const i8);
    if display_moves != 0 {
        i = 0 as i32;
        while i < 60 as i32 {
            fprintf(stream, b" %d\x00" as *const u8 as *const i8,
                    abs((*game).moves[i as usize] as i32));
            if i % 20 as i32 == 19 as i32 {
                fputs(b"\n\x00" as *const u8 as *const i8, stream);
            }
            i += 1
        }
    }
    fputs(b"\n\x00" as *const u8 as *const i8, stream);
}
/*
  COMPUTE_PARTIAL_HASH
  Computes the primary and secondary hash values for the
  unit element in the rotation group.
*/
unsafe fn compute_partial_hash(mut hash_val1: *mut u32,
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
unsafe fn compute_full_primary_hash(mut hash_val:
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
unsafe fn compute_full_secondary_hash(mut hash_val:
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
unsafe fn primary_hash_lookup(mut target_hash: u32)
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
unsafe fn secondary_hash_lookup(mut target_hash: u32)
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
  THOR_COMPARE
  Compares two games from a list of pointers to games.
  Only to be called by QSORT. A full comparison is
  performed using the priority order from THOR_SORT_ORDER.
*/
unsafe fn thor_compare(mut g1: *const libc::c_void,
                                  mut g2: *const libc::c_void)
 -> i32 {
    let mut i: i32 = 0;
    let mut result: i32 = 0;
    let mut game1 = *(g1 as *mut *mut GameType);
    let mut game2 = *(g2 as *mut *mut GameType);
    i = 0 as i32;
    while i < thor_sort_criteria_count {
        match thor_sort_order[i as usize] {
            1 => {
                result =
                    (*(*game2).database).prolog.origin_year -
                        (*(*game1).database).prolog.origin_year
            }
            2 => {
                result =
                    player_lex_order((*game1).black_no as i32) -
                        player_lex_order((*game2).black_no as i32)
            }
            3 => {
                result =
                    player_lex_order((*game1).white_no as i32) -
                        player_lex_order((*game2).white_no as i32)
            }
            4 => {
                result =
                    tournament_lex_order((*game1).tournament_no as
                                             i32) -
                        tournament_lex_order((*game2).tournament_no as
                                                 i32)
            }
            5 => {
                result =
                    (*game1).actual_black_score as i32 -
                        (*game2).actual_black_score as i32
            }
            6 => {
                result =
                    (*game2).actual_black_score as i32 -
                        (*game1).actual_black_score as i32
            }
            0 | _ => {
                /* Really can't happen */
                result =
                    (*(*game1).database).prolog.origin_year -
                        (*(*game2).database).prolog.origin_year
            }
        }
        if result != 0 as i32 { return result }
        i += 1
    }
    /* If control reaches this point, the two games couldn't be
       distinguished by the current search criteria. */
    return 0 as i32;
}
/*
  FILTER_DATABASE
  Applies the current filter rules to the database DB.
*/
unsafe fn filter_database(mut db: *mut DatabaseType) {
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
unsafe fn filter_all_databases() {
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
  SORT_THOR_GAMES
  Sorts the COUNT first games in the list THOR_SEARCH.MATCH_LIST.
  The first THOR_SORT_CRITERIA_COUNT entries of THOR_SORT_ORDER are
  used (in order) to sort the matches.
*/
unsafe fn sort_thor_games(mut count: i32) {
    if count <= 1 as i32 {
        /* No need to sort 0 or 1 games. */
        return
    }
    qsort(thor_search.match_list as *mut libc::c_void, count as size_t,
          ::std::mem::size_of::<*mut GameType>() as u64,
          Some(thor_compare as
                   unsafe fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> i32));
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
unsafe fn recursive_opening_scan(mut node: *mut ThorOpeningNode,
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
unsafe fn opening_scan(mut moves_played: i32) {
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
unsafe fn recursive_frequency_count(mut node: *mut ThorOpeningNode,
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
  CHOOSE_THOR_OPENING_MOVE
  Computes frequencies for all moves from the given position,
  display these and chooses one if from a distribution skewed
  towards common moves. (If no moves are found, PASS is returned.)
*/

pub unsafe fn choose_thor_opening_move(mut in_board:
                                                      *mut i32,
                                                  mut side_to_move:
                                                      i32,
                                                  mut echo: i32)
 -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut temp_symm: i32 = 0;
    let mut pos: i32 = 0;
    let mut disc_count: i32 = 0;
    let mut freq_sum: i32 = 0;
    let mut acc_freq_sum: i32 = 0;
    let mut random_move: i32 = 0;
    let mut random_value: i32 = 0;
    let mut match_count: i32 = 0;
    let mut symmetries: [i32; 8] = [0; 8];
    let mut freq_count: [i32; 100] = [0; 100];
    let mut primary_hash_0: [u32; 8] = [0; 8];
    let mut secondary_hash_0: [u32; 8] = [0; 8];
    let mut move_list: [C2RustUnnamed; 64] =
        [C2RustUnnamed{move_0: 0, frequency: 0,}; 64];
    let mut temp = C2RustUnnamed{move_0: 0, frequency: 0,};
    disc_count = 0 as i32;
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        pos = 10 as i32 * i + 1 as i32;
        while j <= 8 as i32 {
            freq_count[pos as usize] = 0 as i32;
            if *in_board.offset(pos as isize) != 1 as i32 {
                disc_count += 1
            }
            j += 1;
            pos += 1
        }
        i += 1
    }
    /* Check that the parity of the board coincides with standard
       Othello - if this is not the case, the Thor opening lines are useless
       as they don't contain any passes. */
    if side_to_move == 0 as i32 &&
           disc_count % 2 as i32 == 1 as i32 {
        return -(1 as i32)
    }
    if side_to_move == 2 as i32 &&
           disc_count % 2 as i32 == 0 as i32 {
        return -(1 as i32)
    }
    /* Create a random permutation of the symmetries to avoid the same
       symmetry always being chosen in e.g. the initial position */
    i = 0 as i32;
    while i < 8 as i32 { symmetries[i as usize] = i; i += 1 }
    i = 0 as i32;
    while i < 7 as i32 {
        j = i + abs(my_random() as i32) % (8 as i32 - i);
        temp_symm = symmetries[i as usize];
        symmetries[i as usize] = symmetries[j as usize];
        symmetries[j as usize] = temp_symm;
        i += 1
    }
    /* Calculate frequencies for all moves */
    compute_thor_patterns(in_board);
    compute_full_primary_hash(primary_hash_0.as_mut_ptr());
    compute_full_secondary_hash(secondary_hash_0.as_mut_ptr());
    recursive_frequency_count(root_node, freq_count.as_mut_ptr(),
                              0 as i32, disc_count - 4 as i32,
                              symmetries.as_mut_ptr(),
                              primary_hash_0.as_mut_ptr(),
                              secondary_hash_0.as_mut_ptr());
    freq_sum = 0 as i32;
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        pos = 10 as i32 * i + 1 as i32;
        while j <= 8 as i32 {
            freq_sum += freq_count[pos as usize];
            j += 1;
            pos += 1
        }
        i += 1
    }
    if freq_sum > 0 as i32 {
        /* Position found in Thor opening tree */
        /* Create a list of the moves chosen from the position and also
           randomly select one of them. Probability for each move is
           proportional to the frequency of that move being played here. */
        random_value = abs(my_random() as i32) % freq_sum;
        random_move = -(1 as i32);
        acc_freq_sum = 0 as i32;
        match_count = 0 as i32;
        i = 1 as i32;
        while i <= 8 as i32 {
            j = 1 as i32;
            pos = 10 as i32 * i + 1 as i32;
            while j <= 8 as i32 {
                if freq_count[pos as usize] > 0 as i32 {
                    move_list[match_count as usize].move_0 = pos;
                    move_list[match_count as usize].frequency =
                        freq_count[pos as usize];
                    match_count += 1;
                    if acc_freq_sum < random_value &&
                           acc_freq_sum + freq_count[pos as usize] >=
                               random_value {
                        random_move = pos
                    }
                    acc_freq_sum += freq_count[pos as usize]
                }
                j += 1;
                pos += 1
            }
            i += 1
        }
        /* Optionally display the database moves sorted on frequency */
        if echo != 0 {
            i = 0 as i32;
            while i < match_count {
                j = 0 as i32;
                while j < match_count - 1 as i32 {
                    if move_list[j as usize].frequency <
                           move_list[(j + 1 as i32) as
                                         usize].frequency {
                        temp = move_list[j as usize];
                        move_list[j as usize] =
                            move_list[(j + 1 as i32) as usize];
                        move_list[(j + 1 as i32) as usize] = temp
                    }
                    j += 1
                }
                i += 1
            }
            printf(b"%s:        \x00" as *const u8 as *const i8,
                   b"Thor database\x00" as *const u8 as *const i8);
            i = 0 as i32;
            while i < match_count {
                printf(b"%c%c: %4.1f%%    \x00" as *const u8 as
                           *const i8,
                       'a' as i32 +
                           move_list[i as usize].move_0 % 10 as i32 -
                           1 as i32,
                       '0' as i32 +
                           move_list[i as usize].move_0 / 10 as i32,
                       100.0f64 *
                           move_list[i as usize].frequency as f64 /
                           freq_sum as f64);
                if i % 6 as i32 == 4 as i32 {
                    puts(b"\x00" as *const u8 as *const i8);
                }
                i += 1
            }
            if match_count % 6 as i32 != 5 as i32 {
                puts(b"\x00" as *const u8 as *const i8);
            }
        }
        return random_move
    }
    return -(1 as i32);
}
/*
  POSITION_MATCH
  Returns TRUE if the position after MOVE_COUNT moves of GAME, with
  SIDE_TO_MOVE being the player to move, matches the hash codes
  IN_HASH1 and IN_HASH2, otherwise FALSE.
*/
unsafe fn position_match(mut game: *mut GameType,
                                    mut move_count: i32,
                                    mut side_to_move: i32,
                                    mut shape_lo: *mut u32,
                                    mut shape_hi: *mut u32,
                                    mut corner_mask: u32,
                                    mut in_hash1: u32,
                                    mut in_hash2: u32)
 -> i32 {
    let mut i: i32 = 0;
    let mut shape_match: i32 = 0;
    let mut primary_hit_mask: i32 = 0;
    let mut secondary_hit_mask: i32 = 0;
    /* Verify that the number of moves and the side-to-move status
       are correct */
    if move_count >= (*game).move_count as i32 {
        if move_count > (*game).move_count as i32 {
            /* Too many moves! */
            return 0 as i32
        }
        /* No side-to-move status to check if the game is over */
    } else if side_to_move == 0 as i32 {
        /* Black to move */
        if ((*game).moves[move_count as usize] as i32) <
               0 as i32 {
            /* White to move in the game */
            return 0 as i32
        }
    } else if (*game).moves[move_count as usize] as i32 >
                  0 as i32 {
        /* White to move */
        /* Black to move in the game */
        return 0 as i32
    }
    /* Check if the opening information suffices to
       determine if the position matches or not. */
    if (*(*game).opening).current_match == 1 as i32 {
        (*game).matching_symmetry =
            (*(*game).opening).matching_symmetry as i16;
        return 1 as i32
    } else {
        if (*(*game).opening).current_match == 2 as i32 {
            return 0 as i32
        }
    }
    /* Check if the lower 32 bits of the shape state coincide */
    if ((*game).shape_state_lo as i32) < move_count {
        i = (*game).shape_state_lo as i32;
        while i < move_count {
            (*game).shape_lo |=
                move_mask_lo[abs((*game).moves[i as usize] as i32) as
                                 usize];
            i += 1
        }
        (*game).shape_state_lo = move_count as i16
    } else if (*game).shape_state_lo as i32 > move_count {
        i = (*game).shape_state_lo as i32 - 1 as i32;
        while i >= move_count {
            (*game).shape_lo &=
                !move_mask_lo[abs((*game).moves[i as usize] as i32) as
                                  usize];
            i -= 1
        }
        (*game).shape_state_lo = move_count as i16
    }
    shape_match = 0 as i32;
    i = 0 as i32;
    while i < 8 as i32 {
        shape_match |=
            ((*game).shape_lo == *shape_lo.offset(i as isize)) as i32;
        i += 1
    }
    if shape_match == 0 { return 0 as i32 }
    /* Check if the upper 32 bits of the shape state coincide */
    if ((*game).shape_state_hi as i32) < move_count {
        i = (*game).shape_state_hi as i32;
        while i < move_count {
            (*game).shape_hi |=
                move_mask_hi[abs((*game).moves[i as usize] as i32) as
                                 usize];
            i += 1
        }
        (*game).shape_state_hi = move_count as i16
    } else if (*game).shape_state_hi as i32 > move_count {
        i = (*game).shape_state_hi as i32 - 1 as i32;
        while i >= move_count {
            (*game).shape_hi &=
                !move_mask_hi[abs((*game).moves[i as usize] as i32) as
                                  usize];
            i -= 1
        }
        (*game).shape_state_hi = move_count as i16
    }
    shape_match = 0 as i32;
    i = 0 as i32;
    while i < 8 as i32 {
        shape_match |=
            ((*game).shape_hi == *shape_hi.offset(i as isize)) as i32;
        i += 1
    }
    if shape_match == 0 { return 0 as i32 }
    /* Check if the corner mask is compatible with that of the game */
    if corner_mask & !(*game).corner_descriptor != 0 {
        return 0 as i32
    }
    /* Otherwise play through the moves of the game until the
       number of discs is correct and check if the hash
       functions match the given hash values for at least one
       rotation (common to the two hash functions). */
    if play_through_game(game, move_count) != 0 {
        compute_thor_patterns(thor_board.as_mut_ptr());
        primary_hit_mask = primary_hash_lookup(in_hash1);
        if primary_hit_mask != 0 {
            secondary_hit_mask = secondary_hash_lookup(in_hash2);
            if primary_hit_mask & secondary_hit_mask != 0 {
                i = 0 as i32;
                while i < 8 as i32 {
                    if primary_hit_mask & secondary_hit_mask &
                           (1 as i32) << i != 0 {
                        (*game).matching_symmetry = i as i16;
                        return 1 as i32
                    }
                    i += 1
                }
            }
        }
    }
    return 0 as i32;
}
/*
  DATABASE_SEARCH
  Determines what positions in the Thor database match the position
  given by IN_BOARD with SIDE_TO_MOVE being the player whose turn it is.
*/

pub unsafe fn database_search(mut in_board: *mut i32,
                                         mut side_to_move: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut index: i32 = 0;
    let mut pos: i32 = 0;
    let mut sum: i32 = 0;
    let mut move_count: i32 = 0;
    let mut symmetry: i32 = 0;
    let mut next_move: i32 = 0;
    let mut disc_count: [i32; 3] = [0; 3];
    let mut frequency: [i32; 65] = [0; 65];
    let mut cumulative: [i32; 65] = [0; 65];
    let mut target_hash1: u32 = 0;
    let mut target_hash2: u32 = 0;
    let mut corner_mask: u32 = 0;
    let mut shape_lo: [u32; 8] = [0; 8];
    let mut shape_hi: [u32; 8] = [0; 8];
    let mut current_db = 0 as *mut DatabaseType;
    let mut game = 0 as *mut GameType;
    /* We need a player and a tournament database. */
    if players.count == 0 as i32 ||
           tournaments.count == 0 as i32 {
        thor_search.match_count = 0 as i32;
        return
    }
    /* Make sure there's memory allocated if all positions
       in all databases match the position */
    if thor_search.allocation == 0 as i32 {
        thor_search.match_list =
            safe_malloc((thor_game_count as
                             u64).wrapping_mul(::std::mem::size_of::<*mut GameType>()
                                                             as
                                                             u64))
                as *mut *mut GameType;
        thor_search.allocation = thor_game_count
    } else if thor_search.allocation < thor_game_count {
        free(thor_search.match_list as *mut libc::c_void);
        thor_search.match_list =
            safe_malloc((thor_game_count as
                             u64).wrapping_mul(::std::mem::size_of::<*mut GameType>()
                                                             as
                                                             u64))
                as *mut *mut GameType;
        thor_search.allocation = thor_game_count
    }
    /* If necessary, filter all games in the database */
    if thor_games_filtered == 0 {
        filter_all_databases();
        thor_games_filtered = 1 as i32
    }
    /* If necessary, sort all games in the database */
    if thor_games_sorted == 0 {
        current_db = database_head;
        i = 0 as i32;
        while !current_db.is_null() {
            j = 0 as i32;
            while j < (*current_db).count {
                let ref mut fresh5 =
                    *thor_search.match_list.offset(i as isize);
                *fresh5 =
                    &mut *(*current_db).games.offset(j as isize) as
                        *mut GameType;
                i += 1;
                j += 1
            }
            current_db = (*current_db).next
        }
        sort_thor_games(thor_game_count);
        j = 0 as i32;
        while j < thor_game_count {
            (**thor_search.match_list.offset(j as isize)).sort_order = j;
            j += 1
        }
        thor_games_sorted = 1 as i32
    }
    /* Determine disc count, hash codes, patterns and opening
       for the position */
    disc_count[2 as i32 as usize] = 0 as i32;
    disc_count[0 as i32 as usize] =
        disc_count[2 as i32 as usize];
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        pos = 10 as i32 * i + 1 as i32;
        while j <= 8 as i32 {
            if *in_board.offset(pos as isize) == 0 as i32 {
                disc_count[0 as i32 as usize] += 1
            } else if *in_board.offset(pos as isize) == 2 as i32 {
                disc_count[2 as i32 as usize] += 1
            }
            j += 1;
            pos += 1
        }
        i += 1
    }
    move_count =
        disc_count[0 as i32 as usize] +
            disc_count[2 as i32 as usize] - 4 as i32;
    compute_thor_patterns(in_board);
    compute_partial_hash(&mut target_hash1, &mut target_hash2);
    opening_scan(move_count);
    /* Determine the shape masks */
    i = 0 as i32;
    while i < 8 as i32 {
        shape_lo[i as usize] = 0 as i32 as u32;
        shape_hi[i as usize] = 0 as i32 as u32;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        j = 0 as i32;
        pos = 10 as i32 * i + 11 as i32;
        while j < 8 as i32 {
            if *in_board.offset(pos as isize) != 1 as i32 {
                index = 8 as i32 * i + j;
                if index < 32 as i32 {
                    shape_lo[0 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[0 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * i + (7 as i32 - j);
                if index < 32 as i32 {
                    shape_lo[1 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[1 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * j + i;
                if index < 32 as i32 {
                    shape_lo[2 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[2 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * j + (7 as i32 - i);
                if index < 32 as i32 {
                    shape_lo[3 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[3 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * (7 as i32 - i) + j;
                if index < 32 as i32 {
                    shape_lo[4 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[4 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index =
                    8 as i32 * (7 as i32 - i) +
                        (7 as i32 - j);
                if index < 32 as i32 {
                    shape_lo[5 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[5 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * (7 as i32 - j) + i;
                if index < 32 as i32 {
                    shape_lo[6 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[6 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index =
                    8 as i32 * (7 as i32 - j) +
                        (7 as i32 - i);
                if index < 32 as i32 {
                    shape_lo[7 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[7 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
            }
            j += 1;
            pos += 1
        }
        i += 1
    }
    /* Get the corner mask */
    corner_mask =
        get_corner_mask(*in_board.offset(11 as i32 as isize),
                        *in_board.offset(81 as i32 as isize),
                        *in_board.offset(18 as i32 as isize),
                        *in_board.offset(88 as i32 as isize));
    /* Query the database about all positions in all databases.
       Only games which pass the currently applied filter are scanned.
       Also compute the frequency table and the next move table.
       To speed up sorting the games, the match table is first filled
       with NULLs and when a matching game is found, a pointer to it is
       inserted at a position determined by the field SORT_ORDER
       in the game. As this index is unique, no over-write
       can occur. */
    thor_search.match_count = 0 as i32;
    i = 0 as i32;
    while i < thor_game_count {
        let ref mut fresh6 = *thor_search.match_list.offset(i as isize);
        *fresh6 = 0 as *mut GameType;
        i += 1
    }
    i = 0 as i32;
    while i <= 64 as i32 {
        frequency[i as usize] = 0 as i32;
        i += 1
    }
    i = 0 as i32;
    while i < 100 as i32 {
        thor_search.next_move_frequency[i as usize] = 0 as i32;
        thor_search.next_move_score[i as usize] = 0.0f64;
        i += 1
    }
    current_db = database_head;
    while !current_db.is_null() {
        i = 0 as i32;
        while i < (*current_db).count {
            game =
                &mut *(*current_db).games.offset(i as isize) as *mut GameType;
            if (*game).passes_filter != 0 {
                if disc_count[0 as i32 as usize] ==
                       (*game).black_disc_count[move_count as usize] as
                           i32 {
                    if position_match(game, move_count, side_to_move,
                                      shape_lo.as_mut_ptr(),
                                      shape_hi.as_mut_ptr(), corner_mask,
                                      target_hash1, target_hash2) != 0 {
                        let ref mut fresh7 =
                            *thor_search.match_list.offset((*game).sort_order
                                                               as isize);
                        *fresh7 = game;
                        symmetry = (*game).matching_symmetry as i32;
                        if move_count < (*game).move_count as i32 {
                            next_move =
                                *symmetry_map[symmetry as
                                                  usize].offset(abs((*game).moves[move_count
                                                                                      as
                                                                                      usize]
                                                                        as
                                                                        i32)
                                                                    as isize);
                            thor_search.next_move_frequency[next_move as
                                                                usize] += 1;
                            if (*game).actual_black_score as i32 ==
                                   32 as i32 {
                                thor_search.next_move_score[next_move as
                                                                usize] +=
                                    0.5f64
                            } else if (*game).actual_black_score as
                                          i32 > 32 as i32 {
                                if side_to_move == 0 as i32 {
                                    thor_search.next_move_score[next_move as
                                                                    usize] +=
                                        1.0f64
                                }
                            } else if side_to_move == 2 as i32 {
                                thor_search.next_move_score[next_move as
                                                                usize] +=
                                    1.0f64
                            }
                        }
                        frequency[(*game).actual_black_score as usize] += 1;
                        thor_search.match_count += 1
                    }
                }
            }
            i += 1
        }
        current_db = (*current_db).next
    }
    /* Remove the NULLs from the list of matching games if there are any.
       This gives a sorted list. */
    if thor_search.match_count > 0 as i32 &&
           thor_search.match_count < thor_game_count {
        i = 0 as i32;
        j = 0 as i32;
        while i < thor_search.match_count {
            if !(*thor_search.match_list.offset(j as isize)).is_null() {
                let ref mut fresh8 =
                    *thor_search.match_list.offset(i as isize);
                *fresh8 = *thor_search.match_list.offset(j as isize);
                i += 1
            }
            j += 1
        }
    }
    /* Count the number of black wins, draws and white wins.
       Also determine the average score. */
    sum = 0 as i32;
    i = 0 as i32;
    thor_search.white_wins = 0 as i32;
    while i <= 31 as i32 {
        thor_search.white_wins += frequency[i as usize];
        sum += i * frequency[i as usize];
        i += 1
    }
    thor_search.draws = frequency[32 as i32 as usize];
    sum += 32 as i32 * frequency[32 as i32 as usize];
    i = 33 as i32;
    thor_search.black_wins = 0 as i32;
    while i <= 64 as i32 {
        thor_search.black_wins += frequency[i as usize];
        sum += i * frequency[i as usize];
        i += 1
    }
    if thor_search.match_count == 0 as i32 {
        /* Average of 0 values is pointless */
        thor_search.average_black_score = 32.0f64
    } else {
        thor_search.average_black_score =
            sum as f64 / thor_search.match_count as f64
    }
    /* Determine the median score */
    if thor_search.match_count == 0 as i32 {
        /* ...and so is median of 0 values */
        thor_search.median_black_score = 32 as i32
    } else {
        cumulative[0 as i32 as usize] =
            frequency[0 as i32 as usize];
        i = 1 as i32;
        while i <= 64 as i32 {
            cumulative[i as usize] =
                cumulative[(i - 1 as i32) as usize] +
                    frequency[i as usize];
            i += 1
        }
        /* Median is average between first value for which cumulative
           frequency reaches 50% and first value for which it is
           strictly larger than 50%. This definition works regardless
           of the parity of the number of values.
           By definition of median, both loops terminate for indices <= 64. */
        i = 0 as i32;
        while 2 as i32 * cumulative[i as usize] <
                  thor_search.match_count {
            i += 1
        }
        j = i;
        while 2 as i32 * cumulative[j as usize] <
                  thor_search.match_count + 1 as i32 {
            j += 1
        }
        thor_search.median_black_score = (i + j) / 2 as i32
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
  GET_THOR_GAME_MOVES
  Returns the moves, and number of moves, in the INDEXth game
  in the list of matching games generated by DATABASE_SEARCH.
  The game will not necessarily have the same rotational symmetry
  as the position searched for with database_search(); this depends
  on what rotation that gave a match.
*/

pub unsafe fn get_thor_game_moves(mut index: i32,
                                             mut move_count: *mut i32,
                                             mut moves: *mut i32) {
    let mut i: i32 = 0;
    let mut game = 0 as *mut GameType;
    if index < 0 as i32 || index >= thor_search.match_count {
        /* Bad index, so fill with empty values */
        *move_count = 0 as i32
    } else {
        game = *thor_search.match_list.offset(index as isize);
        *move_count = (*game).move_count as i32;
        match (*game).matching_symmetry as i32 {
            0 | 2 | 5 | 7 => {
                /* Symmetries that preserve the initial position. */
                i = 0 as i32;
                while i < (*game).move_count as i32 {
                    *moves.offset(i as isize) =
                        *symmetry_map[(*game).matching_symmetry as
                                          usize].offset(abs((*game).moves[i as
                                                                              usize]
                                                                as
                                                                i32)
                                                            as isize);
                    i += 1
                }
            }
            _ => {
                /* Symmetries that reverse the initial position. */
                i = 0 as i32;
                while i < (*game).move_count as i32 {
                    *moves.offset(i as isize) =
                        abs((*game).moves[i as usize] as i32);
                    i += 1
                }
            }
        }
    };
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
  GET_THOR_GAME_MOVE
  Returns the MOVE_NUMBERth move in the INDEXth game
  in the list of matching games generated by DATABASE_SEARCH.
*/

pub unsafe fn get_thor_game_move(mut index: i32,
                                            mut move_number: i32)
 -> i32 {
    if index < 0 as i32 || index >= thor_search.match_count {
        return -(1 as i32)
    } else {
        let mut game = *thor_search.match_list.offset(index as isize);
        if move_number < 0 as i32 ||
               move_number >= (*game).move_count as i32 {
            return -(1 as i32)
        } else {
            return *symmetry_map[(*game).matching_symmetry as
                                     usize].offset(abs((*game).moves[move_number
                                                                         as
                                                                         usize]
                                                           as i32) as
                                                       isize)
        }
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
  PRINT_THOR_MATCHES
  Outputs the MAX_GAMES first games found by the latest
  database search to STREAM.
*/

pub unsafe fn print_thor_matches(mut stream: *mut FILE,
                                            mut max_games: i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i <
              (if thor_search.match_count < max_games {
                   thor_search.match_count
               } else { max_games }) {
        if i == 0 as i32 {
            fputs(b"\n\x00" as *const u8 as *const i8, stream);
        }
        print_game(stream, *thor_search.match_list.offset(i as isize),
                   0 as i32);
        i += 1
    };
}
/*
  INIT_THOR_HASH
  Computes hash codes for each of the 6561 configurations of the 8 different
  rows. A special feature of the codes is the relation

     hash[flip[pattern]] == reverse[hash[pattern]]

  which speeds up the computation of the hash functions.
*/
unsafe fn init_thor_hash() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut row: [i32; 10] = [0; 10];
    let mut flip_row: [i32; 6561] = [0; 6561];
    let mut buffer: [i32; 6561] = [0; 6561];
    i = 0 as i32;
    while i < 8 as i32 { row[i as usize] = 0 as i32; i += 1 }
    i = 0 as i32;
    while i < 6561 as i32 {
        flip_row[i as usize] = 0 as i32;
        j = 0 as i32;
        while j < 8 as i32 {
            flip_row[i as usize] +=
                row[j as usize] * pow3[(7 as i32 - j) as usize];
            j += 1
        }
        /* Next configuration */
        j = 0 as i32;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                     j < 8 as i32) {
                break ;
            }
        }
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        j = 0 as i32;
        while j < 6561 as i32 {
            buffer[j as usize] = abs(my_random() as i32);
            j += 1
        }
        j = 0 as i32;
        while j < 6561 as i32 {
            primary_hash[i as usize][j as usize] =
                buffer[j as usize] as u32 &
                    0xffff0000 as u32 |
                    bit_reverse_32(buffer[flip_row[j as usize] as usize] as
                                       u32) &
                        0xffff as i32 as u32;
            j += 1
        }
        j = 0 as i32;
        while j < 6561 as i32 {
            buffer[j as usize] = abs(my_random() as i32);
            j += 1
        }
        j = 0 as i32;
        while j < 6561 as i32 {
            secondary_hash[i as usize][j as usize] =
                buffer[j as usize] as u32 &
                    0xffff0000 as u32 |
                    bit_reverse_32(buffer[flip_row[j as usize] as usize] as
                                       u32) &
                        0xffff as i32 as u32;
            j += 1
        }
        i += 1
    };
}
/*
  INIT_MOVE_MASKS
  Initializes the shape bit masks for each of the possible moves.
*/
unsafe fn init_move_masks() {
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
  INIT_SYMMETRY_MAPS
  Initializes the mappings which the 8 elements in the board
  symmetry group induce (and their inverses).
  Note: The order of the mappings must coincide with the order
        in which they are calculated in COMPUTE_FULL_PRIMARY_HASH()
    and COMPUTE_FULL_SECONDARY_HASH().
*/
unsafe fn init_symmetry_maps() {
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
/*
  NEW_THOR_OPENING_NODE
  Creates and initializes a new node for use in the opening tree.
*/
unsafe fn new_thor_opening_node(mut parent: *mut ThorOpeningNode)
 -> *mut ThorOpeningNode {
    let mut node = 0 as *mut ThorOpeningNode;
    node =
        safe_malloc(::std::mem::size_of::<ThorOpeningNode>() as u64)
            as *mut ThorOpeningNode;
    (*node).child_move = 0 as i32 as i8;
    (*node).sibling_move = 0 as i32 as i8;
    (*node).child_node = 0 as *mut ThorOpeningNode_;
    (*node).sibling_node = 0 as *mut ThorOpeningNode_;
    (*node).parent_node = parent;
    return node;
}
/*
  CALCULATE_OPENING_FREQUENCY
  Calculates and returns the number of lines in the Thor opening base
  that match the line defined by NODE.
*/
unsafe fn calculate_opening_frequency(mut node:
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
  BUILD_THOR_OPENING_TREE
  Builds the opening tree from the statically computed
  structure THOR_OPENING_LIST (see thorop.c).
*/
unsafe fn build_thor_opening_tree() {
    let mut thor_move_list: [i8; 61] = [0; 61];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut move_0: i32 = 0;
    let mut branch_depth: i32 = 0;
    let mut end_depth: i32 = 0;
    let mut flipped: i32 = 0;
    let mut hash1: u32 = 0;
    let mut hash2: u32 = 0;
    let mut parent = 0 as *mut ThorOpeningNode;
    let mut last_child = 0 as *mut ThorOpeningNode;
    let mut new_child = 0 as *mut ThorOpeningNode;
    let mut node_list: [*mut ThorOpeningNode; 61] =
        [0 as *mut ThorOpeningNode; 61];
    /* Create the root node and compute its hash value */
    root_node = new_thor_opening_node(0 as *mut ThorOpeningNode);
    clear_thor_board();
    compute_thor_patterns(thor_board.as_mut_ptr());
    compute_partial_hash(&mut hash1, &mut hash2);
    (*root_node).hash1 = hash1;
    (*root_node).hash2 = hash2;
    node_list[0 as i32 as usize] = root_node;
    /* Add each of the openings to the tree */
    i = 0 as i32;
    while i < 741 as i32 {
        branch_depth = thor_opening_list[i as usize].first_unique;
        end_depth =
            (branch_depth as
                 u64).wrapping_add(strlen(thor_opening_list[i as
                                                                          usize].move_str).wrapping_div(2
                                                                                                            as
                                                                                                            i32
                                                                                                            as
                                                                                                            u64))
                as i32;
        j = 0 as i32;
        while j < end_depth - branch_depth {
            thor_move_list[(branch_depth + j) as usize] =
                (10 as i32 *
                     (*thor_opening_list[i as
                                             usize].move_str.offset((2 as
                                                                         i32
                                                                         * j +
                                                                         1 as
                                                                             i32)
                                                                        as
                                                                        isize)
                          as i32 - '0' as i32) +
                     (*thor_opening_list[i as
                                             usize].move_str.offset((2 as
                                                                         i32
                                                                         * j)
                                                                        as
                                                                        isize)
                          as i32 - 'a' as i32 + 1 as i32)) as
                    i8;
            j += 1
        }
        /* Play through the moves common with the previous line
           and the first deviation */
        clear_thor_board();
        thor_side_to_move = 0 as i32;
        j = 0 as i32;
        while j <= branch_depth {
            move_0 = thor_move_list[j as usize] as i32;
            flipped =
                any_flips(move_0, thor_side_to_move,
                          0 as i32 + 2 as i32 -
                              thor_side_to_move);
            if flipped != 0 {
                thor_board[move_0 as usize] = thor_side_to_move;
                thor_side_to_move =
                    0 as i32 + 2 as i32 - thor_side_to_move
            } else {
                thor_side_to_move =
                    0 as i32 + 2 as i32 - thor_side_to_move;
                flipped =
                    any_flips(move_0, thor_side_to_move,
                              0 as i32 + 2 as i32 -
                                  thor_side_to_move);
                if flipped != 0 {
                    thor_board[move_0 as usize] = thor_side_to_move;
                    thor_side_to_move =
                        0 as i32 + 2 as i32 -
                            thor_side_to_move
                } else {
                    puts(b"This COULD happen (1) in BUILD_THOR_OPENING_TREE\x00"
                             as *const u8 as *const i8);
                }
            }
            j += 1
        }
        /* Create the branch from the previous node */
        parent = node_list[branch_depth as usize];
        new_child = new_thor_opening_node(parent);
        compute_thor_patterns(thor_board.as_mut_ptr());
        compute_partial_hash(&mut hash1, &mut hash2);
        (*new_child).hash1 = hash1;
        (*new_child).hash2 = hash2;
        if (*parent).child_node.is_null() {
            (*parent).child_node = new_child;
            (*parent).child_move = thor_move_list[branch_depth as usize]
        } else {
            last_child = (*parent).child_node;
            while !(*last_child).sibling_node.is_null() {
                last_child = (*last_child).sibling_node
            }
            (*last_child).sibling_node = new_child;
            (*last_child).sibling_move = thor_move_list[branch_depth as usize]
        }
        node_list[(branch_depth + 1 as i32) as usize] = new_child;
        /* Play through the rest of the moves and create new nodes for each
           of the resulting positions */
        j = branch_depth + 1 as i32;
        while j < end_depth {
            move_0 = thor_move_list[j as usize] as i32;
            flipped =
                any_flips(move_0, thor_side_to_move,
                          0 as i32 + 2 as i32 -
                              thor_side_to_move);
            if flipped != 0 {
                thor_board[move_0 as usize] = thor_side_to_move;
                thor_side_to_move =
                    0 as i32 + 2 as i32 - thor_side_to_move
            } else {
                thor_side_to_move =
                    0 as i32 + 2 as i32 - thor_side_to_move;
                flipped =
                    any_flips(move_0, thor_side_to_move,
                              0 as i32 + 2 as i32 -
                                  thor_side_to_move);
                if flipped != 0 {
                    thor_board[move_0 as usize] = thor_side_to_move;
                    thor_side_to_move =
                        0 as i32 + 2 as i32 -
                            thor_side_to_move
                } else {
                    puts(b"This COULD happen (2) in BUILD_THOR_OPENING_TREE\x00"
                             as *const u8 as *const i8);
                }
            }
            parent = new_child;
            new_child = new_thor_opening_node(parent);
            compute_thor_patterns(thor_board.as_mut_ptr());
            compute_partial_hash(&mut hash1, &mut hash2);
            (*new_child).hash1 = hash1;
            (*new_child).hash2 = hash2;
            (*parent).child_node = new_child;
            (*parent).child_move = thor_move_list[j as usize];
            node_list[(j + 1 as i32) as usize] = new_child;
            j += 1
        }
        (*new_child).frequency = thor_opening_list[i as usize].frequency;
        i += 1
    }
    /* Calculate opening frequencies also for interior nodes */
    calculate_opening_frequency(root_node);
}
/*
  GET_THOR_GAME_SIZE
  Returns the amount of memory which each game in the database takes.
*/

pub unsafe fn get_thor_game_size() -> i32 {
    return ::std::mem::size_of::<GameType>() as u64 as i32;
}
/*
  INIT_THOR_DATABASE
  Performs the basic initializations of the Thor database interface.
  Before any operation on the database may be performed, this function
  must be called.
*/

pub unsafe fn init_thor_database() {
    let mut i: i32 = 0; /* "infinity" */
    thor_game_count = 0 as i32;
    thor_database_count = 0 as i32;
    thor_search.match_list = 0 as *mut *mut GameType;
    thor_search.allocation = 0 as i32;
    thor_search.match_count = 0 as i32;
    thor_search.black_wins = 0 as i32;
    thor_search.draws = 0 as i32;
    thor_search.white_wins = 0 as i32;
    thor_search.median_black_score = 0 as i32;
    thor_search.average_black_score = 0.0f64;
    thor_sort_criteria_count = 5 as i32;
    i = 0 as i32;
    while i < 5 as i32 {
        thor_sort_order[i as usize] = default_sort_order[i as usize];
        i += 1
    }
    database_head = 0 as *mut DatabaseType;
    players.name_buffer = 0 as *mut i8;
    players.player_list = 0 as *mut PlayerType;
    players.count = 0 as i32;
    tournaments.name_buffer = 0 as *mut i8;
    tournaments.tournament_list = 0 as *mut TournamentType;
    tournaments.count = 0 as i32;
    thor_games_sorted = 0 as i32;
    thor_games_filtered = 0 as i32;
    init_move_masks();
    init_symmetry_maps();
    init_thor_hash();
    prepare_thor_board();
    build_thor_opening_tree();
    filter.game_categories =
        1 as i32 | 2 as i32 | 4 as i32;
    filter.player_filter = EitherSelectedFilter;
    filter.first_year = -((1 as i32) << 25 as i32);
    filter.last_year = (1 as i32) << 25 as i32;
}
