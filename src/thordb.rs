use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn bit_reverse_32(val: libc::c_uint) -> libc::c_uint;
    /*
   File:       error.h

   Created:    June 13, 1998

   Modified:   August 1, 2002

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The interface to the error handler.
*/
    #[no_mangle]
    fn fatal_error(format: *const libc::c_char, _: ...);
    /* Directional flip masks for all board positions. */
    #[no_mangle]
    static dir_mask: [libc::c_int; 100];
    #[no_mangle]
    fn my_random() -> libc::c_long;
    /*
   File:           patterns.h

   Created:        July 4, 1997

   Modified:       August 1, 2002

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       The patterns.
*/
    /* Predefined two-bit patterns. */
    /* Board patterns used in position evaluation */
    #[no_mangle]
    static mut pow3: [libc::c_int; 10];
    /*
   File:       safemem.h

   Created:    August 30, 1998

   Modified:   January 25, 2000

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The interface to the safer version of malloc.
*/
    #[no_mangle]
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn safe_realloc(ptr: *mut libc::c_void, size: size_t)
     -> *mut libc::c_void;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameInfoType {
    pub black_name: *const libc::c_char,
    pub white_name: *const libc::c_char,
    pub tournament: *const libc::c_char,
    pub year: libc::c_int,
    pub black_actual_score: libc::c_int,
    pub black_corrected_score: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DatabaseInfoType {
    pub year: libc::c_int,
    pub count: libc::c_int,
}
pub type PlayerFilterType = libc::c_uint;
pub const WhiteSelectedFilter: PlayerFilterType = 3;
pub const BlackSelectedFilter: PlayerFilterType = 2;
pub const BothSelectedFilter: PlayerFilterType = 1;
pub const EitherSelectedFilter: PlayerFilterType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlayerType {
    pub lex_order: libc::c_int,
    pub is_program: libc::c_int,
    pub selected: libc::c_int,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlayerDatabaseType {
    pub prolog: PrologType,
    pub name_buffer: *mut libc::c_char,
    pub count: libc::c_int,
    pub player_list: *mut PlayerType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PrologType {
    pub creation_century: libc::c_int,
    pub creation_year: libc::c_int,
    pub creation_month: libc::c_int,
    pub creation_day: libc::c_int,
    pub game_count: libc::c_int,
    pub item_count: libc::c_int,
    pub origin_year: libc::c_int,
    pub reserved: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThorOpeningNode_ {
    pub hash1: libc::c_uint,
    pub hash2: libc::c_uint,
    pub current_match: libc::c_int,
    pub frequency: libc::c_int,
    pub matching_symmetry: libc::c_int,
    pub child_move: libc::c_char,
    pub sibling_move: libc::c_char,
    pub child_node: *mut ThorOpeningNode_,
    pub sibling_node: *mut ThorOpeningNode_,
    pub parent_node: *mut ThorOpeningNode_,
}
pub type DatabaseType = DatabaseType_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DatabaseType_ {
    pub prolog: PrologType,
    pub games: *mut GameType,
    pub count: libc::c_int,
    pub next: *mut DatabaseType_,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameType {
    pub tournament_no: libc::c_short,
    pub black_no: libc::c_short,
    pub white_no: libc::c_short,
    pub actual_black_score: libc::c_short,
    pub perfect_black_score: libc::c_short,
    pub moves: [libc::c_char; 60],
    pub move_count: libc::c_short,
    pub black_disc_count: [libc::c_char; 61],
    pub opening: *mut ThorOpeningNode,
    pub database: *mut DatabaseType_,
    pub shape_hi: libc::c_uint,
    pub shape_lo: libc::c_uint,
    pub shape_state_hi: libc::c_short,
    pub shape_state_lo: libc::c_short,
    pub corner_descriptor: libc::c_uint,
    pub sort_order: libc::c_int,
    pub matching_symmetry: libc::c_short,
    pub passes_filter: libc::c_short,
}
pub type ThorOpeningNode = ThorOpeningNode_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TournamentType {
    pub lex_order: libc::c_int,
    pub selected: libc::c_int,
    pub name: *const libc::c_char,
}
pub type int_32 = libc::c_int;
pub type int_16 = libc::c_short;
/* Type definitions */
pub type int_8 = libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TournamentDatabaseType {
    pub prolog: PrologType,
    pub name_buffer: *mut libc::c_char,
    pub count: libc::c_int,
    pub tournament_list: *mut TournamentType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub move_0: libc::c_int,
    pub frequency: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FilterType {
    pub game_categories: libc::c_int,
    pub first_year: libc::c_int,
    pub last_year: libc::c_int,
    pub player_filter: PlayerFilterType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SearchResultType {
    pub average_black_score: libc::c_double,
    pub next_move_score: [libc::c_double; 100],
    pub match_count: libc::c_int,
    pub black_wins: libc::c_int,
    pub draws: libc::c_int,
    pub white_wins: libc::c_int,
    pub median_black_score: libc::c_int,
    pub allocation: libc::c_int,
    pub next_move_frequency: [libc::c_int; 100],
    pub match_list: *mut *mut GameType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThorOpening {
    pub first_unique: libc::c_int,
    pub frequency: libc::c_int,
    pub move_str: *const libc::c_char,
}
#[no_mangle]
pub static mut thor_opening_list: [ThorOpening; 741] =
    [{
         let mut init =
             ThorOpening{first_unique: 0 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"f5f6d3g5e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f4e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 2 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"c4g5e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"f4e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 2 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"e6f4c3e7f3e3d3e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"d7e3d6e7c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f3d6g5g4e3g6e7g3f7f8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"h5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"g3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"e2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"g6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"e3g4c4d3d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e7d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"d6f3c5g4h3h5g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g3h3c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"e7f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"d7g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"c4c5b6g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"c6b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"b4a5a3c6b5a6b6g4e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"g5e3b3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"e3b3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"d3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"b3e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g5e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"c6b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"e3d3c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"g5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"c6d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"c5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"c4d3c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"c5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"d3e7f3e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"d7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"d6f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"g5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"e3d7g4f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 89 as libc::c_int,
                         move_str:
                             b"d6c4f3g3c3c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"g4e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"e2d7c6f7d3g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"g4d3c3h3c4g3g5g6c7c6h5f3h4h6\x00" as *const u8
                                 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e7d7h5f3h4h6c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"h4g6f3h6h5h2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"c6c5d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"h4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 50 as libc::c_int,
                         move_str:
                             b"g3g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"d2c4f3f2e2g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"c4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 62 as libc::c_int,
                         move_str:
                             b"c5f3g4e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"g5d3c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"c6c5e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e7d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"c5c4e7g4g3g5f3g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"g6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"d7d6f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"g5b5d6g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"e8\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"f3g3d3d6b5c3c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"g5d3c2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"d7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"b5d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"e2f2c3d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"d2f3g4g5d6g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"h3d3c6b3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"d3c3c2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"f3d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f2g4g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 54 as libc::c_int,
                         move_str:
                             b"g5g4g6d6h3h5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"d3d7f7c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g5g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"g5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"c6d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 56 as libc::c_int,
                         move_str:
                             b"e2f2d2f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"f3g4g5d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"d3c3d2e1c1f2g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"c2d1f1g6d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"f2g4g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"g5g4g6d6h3h5g3c3d3b3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"f7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"d7c6e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"d6b5d3c3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 50 as libc::c_int,
                         move_str:
                             b"c6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 56 as libc::c_int,
                         move_str:
                             b"d3c3f2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f3e2c3g4c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"e2b5e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"g6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"g4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"g5d6g6h6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 55 as libc::c_int,
                         move_str:
                             b"f3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 66 as libc::c_int,
                         move_str:
                             b"g4g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"g3c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"g4d6f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"f3c4d6e2h5h6h3h4h7g6f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g6d3f2h4e2h6c6d2b5f7h5e7d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"d2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"h5h6e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"g6f3d3e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 59 as libc::c_int,
                         move_str:
                             b"f3g4g6g3d6f7h5h4h3e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"e7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 78 as libc::c_int,
                         move_str:
                             b"g6e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"d7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"d3g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 58 as libc::c_int,
                         move_str:
                             b"d6f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 55 as libc::c_int,
                         move_str:
                             b"g4c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"g6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"c6e7d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"d6g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"e7d7c3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 65 as libc::c_int,
                         move_str:
                             b"d3f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"g6f3g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"d3f3d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 70 as libc::c_int,
                         move_str:
                             b"c5c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"g5g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 63 as libc::c_int,
                         move_str:
                             b"d2e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"f3c5c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 61 as libc::c_int,
                         move_str:
                             b"c6e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"f7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 53 as libc::c_int,
                         move_str:
                             b"g3d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"f3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"g4e7f7g5f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"g5e7e3g6d3c6c5f3g4h4h6h3f2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"c5d6c6d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"f7h5c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"c4e8g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"d2c3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"b5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"c4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"f3g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f3d6f7d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"c5c6d6f3g4h3h4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"d6h5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"c4c5b5f7h5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 59 as libc::c_int,
                         move_str:
                             b"d3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"e8\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"f7h5h4h3c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"d7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"f3f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"d6d3g4g6h5h6h7h4h3c4f3c5c6d7b5\x00" as
                                 *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"f7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 56 as libc::c_int,
                         move_str:
                             b"f3h3h5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"f3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"g4d3c6d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 57 as libc::c_int,
                         move_str:
                             b"g6h5h6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"f3h3c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"c5d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"g6f3h3c4g3h5h6h4f7c2d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"f7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"f3d3c3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g3f3d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"g4d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"f3d3c3f7d7d8g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 57 as libc::c_int,
                         move_str:
                             b"c4d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"c7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e2g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"c5c4g3c6d6d7c7g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"d3g4g3g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"d7g6g4h5e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 50 as libc::c_int,
                         move_str:
                             b"e8\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"d6f3g6g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g4e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"g3e3g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"c5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"f7g6g4h5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 98 as libc::c_int,
                         move_str:
                             b"h5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"h6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"d6d8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"e8h6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"h5d3c6e3d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 51 as libc::c_int,
                         move_str:
                             b"d7c4g4h4e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"c4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 61 as libc::c_int,
                         move_str:
                             b"e3d3f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"g4g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"h4f3g6h3h2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 81 as libc::c_int,
                         move_str:
                             b"f2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"h3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"g4g6e3h4f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"f3g3d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"h3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"h4e3d6c3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"f3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"e8c5g4h4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 71 as libc::c_int,
                         move_str:
                             b"c5f3g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"g3h3h5g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"e8h5g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"h6e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"g6g4e7e3f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"f3d6f7h3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"f7h5e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"h6h5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"f7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"c6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"e7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"d6e3g6f7h5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"f3g6e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 87 as libc::c_int,
                         move_str:
                             b"f7h5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 53 as libc::c_int,
                         move_str:
                             b"c5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"d7e7f3g6g4f7f8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"g4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"g3e3g4c6d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"c5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"e7g6c5c4f3h5f7g4h6h7d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"d7f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 54 as libc::c_int,
                         move_str:
                             b"f7f8d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"g4f3h5e3d3h3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"h6h7e3d3h3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"f7g4f3h5h4h3g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 56 as libc::c_int,
                         move_str:
                             b"h6h7c5g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 73 as libc::c_int,
                         move_str:
                             b"c6f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"h4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"g6f7g5e7d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"h5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e7d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"d6e3f3g4g5d3e2c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"d3g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"g5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"d3g4f3g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"g4f7e8h3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 63 as libc::c_int,
                         move_str:
                             b"g5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"c6e7f8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"c6g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"c5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 58 as libc::c_int,
                         move_str:
                             b"g5d6c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"c5f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g4g5e3f3f2h4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 66 as libc::c_int,
                         move_str:
                             b"g3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f3e3c4g3h5f7e7d3c2b4d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"h5g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f2d3c3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 67 as libc::c_int,
                         move_str:
                             b"h4h6e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 57 as libc::c_int,
                         move_str:
                             b"f3e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"h3h2h6g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 59 as libc::c_int,
                         move_str:
                             b"f3h5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 55 as libc::c_int,
                         move_str:
                             b"g3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"h4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"g7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 2 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"f7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 1 as libc::c_int,
                         frequency: 51 as libc::c_int,
                         move_str:
                             b"d6c3g5c6b6e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"f6e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 51 as libc::c_int,
                         move_str:
                             b"c5c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"d3e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"f6d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"g6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"f4e6f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"d3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"f6g5f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"e6e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"d7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"c4e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"f3d3c4e3e2g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"e3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"d3f3g4g3e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"b3c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"c4g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"c6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"f3d3f4f6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"f4d3c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"d3e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"c4b5b4f4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"f4e3f6c6c5d7e7e6b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 57 as libc::c_int,
                         move_str:
                             b"c7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f3g4f6g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"g3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 56 as libc::c_int,
                         move_str:
                             b"e2d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 65 as libc::c_int,
                         move_str:
                             b"e6f6g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"c6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 66 as libc::c_int,
                         move_str:
                             b"b4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 62 as libc::c_int,
                         move_str:
                             b"c5f6e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 56 as libc::c_int,
                         move_str:
                             b"b5b4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"b4b3e6c6b5e3f2a6a5d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 55 as libc::c_int,
                         move_str:
                             b"c6c2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"b5c6e6e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"c2e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"d7c7e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"c6e6b5a5b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"a6a3b6a4a2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 55 as libc::c_int,
                         move_str:
                             b"e3e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 57 as libc::c_int,
                         move_str:
                             b"b5e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"c6b3e6e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"e3f3b3e6b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f3f6e3g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"e6e3g6b6a5d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"f6g5d7g3b6e7a5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 18 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e7e2g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 18 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"d2b6a5e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"c2b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"a5e3c2a4b3a6b6d2a3a2e1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"c6e6a3b3d2c2c1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"e3b5b6b3a6a5a3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"b3c2f6a3b5b6e3f2e2b4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"e6b4f3e3e2g4d2g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"g4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"d2e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"c6b6b4f6g3b5a3e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"b4b5d2a3a4a5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e3a6c1d7e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"b6f3f6f7g5d7c7g4e1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 24 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"g6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 21 as libc::c_int,
                         frequency: 60 as libc::c_int,
                         move_str:
                             b"e2d1f1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 21 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"e1e2f1f2g5d7c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 19 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"e1e2f6f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 53 as libc::c_int,
                         move_str:
                             b"g4a5f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"f3g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"e1\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"c7f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 106 as libc::c_int,
                         move_str:
                             b"f7c7f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"b6a6c8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"a6e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"b6d2e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"d2e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"b4e3e6f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"b6c6b5f6a5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"a6a5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"a4f3a6a3d2c1f6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"c1\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"c6f6a5a4b5a6d7d8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 19 as libc::c_int,
                         frequency: 55 as libc::c_int,
                         move_str:
                             b"c7e7e8b6f8g3f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 23 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"d8a2a7g3f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 24 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"g3f7g5d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 27 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"d1c1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 21 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"b6f7e8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 56 as libc::c_int,
                         move_str:
                             b"d7b5e7c7b6f8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 55 as libc::c_int,
                         move_str:
                             b"g4f6b6a6e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"g6g5f3d7f7e7a3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"e2d2f3b6g6a5a4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"b5d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"c6e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"d2a3e6a4b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 51 as libc::c_int,
                         move_str:
                             b"e3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e6b5e3f3g4b6a6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"a5g6g5h6g4h5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"a6a3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"a5d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g6a4a3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"a6a3b6a4a2e3f3g4c1f2g3h4h3h2f1\x00" as
                                 *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 20 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"g3g4e3f3h3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"a7b6c7g5a2c1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"e3d2c6f2e6b4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"b4f6b5e6f2f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"b5e2e6f3c1a6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f2e6f3c1a3a4a5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"a3e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"a6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"a4b5b6g4e2a3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"a3f2e6b5a4f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"g3g4f3c1b5a4a6d1e2f2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"b5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"f3e6e1d1c1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"g4f3e6f7g3c1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 18 as libc::c_int,
                         frequency: 50 as libc::c_int,
                         move_str:
                             b"b5b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 19 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"a4a6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"b6b4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"b4c6g6f2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"e6b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 53 as libc::c_int,
                         move_str:
                             b"d2e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"e6b4g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 61 as libc::c_int,
                         move_str:
                             b"d1a3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"d2c6b5e6b4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 70 as libc::c_int,
                         move_str:
                             b"b6b4b5e6c2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"e3c2b4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 59 as libc::c_int,
                         move_str:
                             b"b4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"e2e3c2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"d2b4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"c6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 50 as libc::c_int,
                         move_str:
                             b"e6f6e3c5g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"c6b6c7b5d7e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"e7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"d7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 53 as libc::c_int,
                         move_str:
                             b"b5d7e7b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 59 as libc::c_int,
                         move_str:
                             b"b4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"b3c2e3d2c5b4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 57 as libc::c_int,
                         move_str:
                             b"b4c5c6d7b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"d2e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"e2c5c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"e3f3c5b4a3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 58 as libc::c_int,
                         move_str:
                             b"f6g5e3f3g4h3g3f2e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e6g3g6f7h5h4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"e2d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"g6f7h6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e2h5c5g4g3f2d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"e6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"f3g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"e3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"c6e3d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 50 as libc::c_int,
                         move_str:
                             b"e6f7e3c5d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"h5h6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"c6c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"g6c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 57 as libc::c_int,
                         move_str:
                             b"c7c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"d7c5g3f3c6e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e3c6b4d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"d7e3c5f3e7h5e2c6d2c2b3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"f7e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"c7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e7c5e3f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 50 as libc::c_int,
                         move_str:
                             b"f3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"f7c5b6e3c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 55 as libc::c_int,
                         move_str:
                             b"c5e3f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 61 as libc::c_int,
                         move_str:
                             b"f3f7e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 63 as libc::c_int,
                         move_str:
                             b"b5e3g4h4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"b4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"e3g4f7e7g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"b5e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"g6e3e6c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"b4c2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"f3e6e3g5g6g4h4h5h6g3h3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f3e3g5g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"b4b5e6e7g5c5e2g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"f2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"e2e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"g5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 54 as libc::c_int,
                         move_str:
                             b"g4g5e3h3g3f2e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"e6g3g6f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"g3c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"e6b3d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"e6e7e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"c6g6g5f7g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f8f7d2c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"c5b4f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g5d2d7f8e8f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e3d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"d7g6g5f7e2c8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"h6e3c8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"c5e3g3g4h5f8d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"e8h6d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"c5c6c8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"h6d2c8c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"b6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"g4d2c8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 63 as libc::c_int,
                         move_str:
                             b"f8h6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 56 as libc::c_int,
                         move_str:
                             b"c5b6d8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"c6b5c7f8e8f7h6d2g4c8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"c6f7e8h6d2c8c7b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"f8h6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 82 as libc::c_int,
                         move_str:
                             b"f7c5c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"d8f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"c5c6c7c8f7g8b5b6b4a3a4e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 19 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"b3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"b6a5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 18 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"b5e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"f8f7g5h6h4g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"g5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"c5b6d8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"b5e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"c6g6g5f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"f7c5e3g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"b4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 80 as libc::c_int,
                         move_str:
                             b"e2g3f2g4d2d8g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"g4g3f2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"d2f2g6c7g5g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"b6g6e3e2f1d2e1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 64 as libc::c_int,
                         move_str:
                             b"d1\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g4g5c6h5f8d8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"g5c6d8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"g4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"g5c6e8b5e3d2b3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"f8\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"g5e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"b5e3g6g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 72 as libc::c_int,
                         move_str:
                             b"b4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"c6c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"f3f4b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 55 as libc::c_int,
                         move_str:
                             b"b3d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"c5c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"b4b6c6b5e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"c6b6d7e8c2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"f4e6c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"c5f4f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 66 as libc::c_int,
                         move_str:
                             b"c6f4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 2 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"c4g5c6b6f6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"c5b6b5a6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"c3e3b5f6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 50 as libc::c_int,
                         move_str:
                             b"e6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"f6f4e6f7g4e7c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"f3h5h6h7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"e3g6d7g3c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"d7e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"d7c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"d3b4c3e3b5a4f2c2a6f6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"f3e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"d3e6c5b4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"f6f4f3d3c3g6e3e6c5e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"e6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 72 as libc::c_int,
                         move_str:
                             b"g6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"g6d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 67 as libc::c_int,
                         move_str:
                             b"f4c5f6g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"d3e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"e6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 51 as libc::c_int,
                         move_str:
                             b"f6g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"f3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"f3f4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 54 as libc::c_int,
                         move_str:
                             b"d3c3f4e3f3g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"c5b4b3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"b3c2f6a3b5b6e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"e6b4f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"c6b4b5d2e3a6c1b6f3f6f7g5d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 21 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"e1e2f1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"b4e3e6b6c6b5a4f3a6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"c6f6a5a4b5a6d7c7e7e8b6f8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 23 as libc::c_int,
                         frequency: 64 as libc::c_int,
                         move_str:
                             b"d8g3f7g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"d7b5e7c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"g4f6b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"c6d2e6b5e3f3a5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"a5a6a3b6a4a2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"a7b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"e3d2c6b4b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"a4b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"a3g3g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"g4f3e6f7b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"d2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"e6f6e3c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"b3e2e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"f6g5e6f7d7c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"d7e3c5f3e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"c5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 50 as libc::c_int,
                         move_str:
                             b"f3e3b4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 68 as libc::c_int,
                         move_str:
                             b"g4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"e6e7c6g6g5f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"f8f7d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"d7g6g5f7e2c5e3g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"e8c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"f8h6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"c5b6c6b5c7f8e8f7h6d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"c6f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"f7c5c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"d8c5c6c7c8f7g8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"f8f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"c5c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"f7c5e3e2g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"g4d2f2g6c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"b6g6e3e2f1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"g5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"b3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"c5b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"f4d2f6d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"e2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"e3c6e6f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"f3c2f6e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"e6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"d7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"c6g4d2e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"e6d2g4b6b5e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"g3d1c3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"c7a5f2d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"c3b4c1f2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"c7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 73 as libc::c_int,
                         move_str:
                             b"d7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"b4b3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"b5d2d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"d2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"e2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"g4g6g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"e6c3d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"f6e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"c3d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 79 as libc::c_int,
                         move_str:
                             b"e6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"d7f6d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"b4c3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"d7f4e3f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"e6f6e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f4e3f6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f3c3c5d2c6g4f7e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"d7g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"b4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"b4b3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"b4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"g4e7c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"g3d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 57 as libc::c_int,
                         move_str:
                             b"c6f6g5g6e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"c6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"b3b4f4f6g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 2 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"c5f6e6f4c6e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"f4d3f6e6d7f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 67 as libc::c_int,
                         move_str:
                             b"c4c3c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"e3e6f6g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 53 as libc::c_int,
                         move_str:
                             b"c3e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"e3g4g5e2c4f3e6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"e6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"f3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"e6f6e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 58 as libc::c_int,
                         move_str:
                             b"c4e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 76 as libc::c_int,
                         move_str:
                             b"g3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"g3e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 62 as libc::c_int,
                         move_str:
                             b"e6c4e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"f6e6g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"c6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"e3f6f3c4d3e6c3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 61 as libc::c_int,
                         move_str:
                             b"e2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"e6d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"e2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 54 as libc::c_int,
                         move_str:
                             b"e6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 74 as libc::c_int,
                         move_str:
                             b"c6d3f6g4c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 66 as libc::c_int,
                         move_str:
                             b"e6f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"d7g3b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"c4b4c3d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"b5b3b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"b3b5a4a2a3a5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"g5c3b5a5a4a3b6c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 21 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"a6f7f2f3e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 23 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"e2d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 23 as libc::c_int,
                         frequency: 55 as libc::c_int,
                         move_str:
                             b"d2e2f1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"e2f2d2c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 18 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"f3d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 18 as libc::c_int,
                         frequency: 65 as libc::c_int,
                         move_str:
                             b"e7c7b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 20 as libc::c_int,
                         frequency: 72 as libc::c_int,
                         move_str:
                             b"f7g6f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 57 as libc::c_int,
                         move_str:
                             b"b6b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 56 as libc::c_int,
                         move_str:
                             b"c3b5a6g5a5a4a3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 18 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"g6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 91 as libc::c_int,
                         move_str:
                             b"g4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"a3g5c3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"e2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 77 as libc::c_int,
                         move_str:
                             b"g5c3b4b3b5a5a4a3b6a6f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"a4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"e2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"a3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"e2b3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"b5g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"d2c7b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"d2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"f7g6e7f8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"e2c8f2c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"d2e7f2e1f3c8h3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 18 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"e2f1c8f3c7h3b6g4c1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 18 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"c8f3c7h3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"g6g5h6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"e7\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"c3f7d2e7f2e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e7c3g4f8f7g6c8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 50 as libc::c_int,
                         move_str:
                             b"g5f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"g5f8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"g4c4b4g3g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"b3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"g5c3b3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 58 as libc::c_int,
                         move_str:
                             b"b4b3b5a5a4a3b6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"e2f3d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"f7g6e7f8h6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 15 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"d2c1f2e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 17 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"e2f2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 18 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"b5g3f3h4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"e7f2e2f3c1g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 19 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"f1c8f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 18 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"c8f3c7d8e2b8f1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 23 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"g3e8f1h2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 23 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f1e8g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 22 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"e8g3e2h4f1e1\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 26 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"h5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 25 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"f1\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 21 as libc::c_int,
                         frequency: 70 as libc::c_int,
                         move_str:
                             b"g6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g6g5e7f8f7h6h5c3h7g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"c3f7d2e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 73 as libc::c_int,
                         move_str:
                             b"g3g5h4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"g6g5h5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"e7c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"h3g5h4e7f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"e7c7c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 60 as libc::c_int,
                         move_str:
                             b"f3g5h6f7f8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"e7d8c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"c7c4f8b6b5a6a4g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"d8e8g8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"d8\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 13 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"f3b6b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 14 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"d8c8f7f8b8e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"b8e2b6e8f8f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"b6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"f3c4c7d8c8f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 16 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"b8\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 35 as libc::c_int,
                         move_str:
                             b"c3c4c7d8c8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 51 as libc::c_int,
                         move_str:
                             b"f3c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"g5e6f6g3c4b4b3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"d7f3e7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 61 as libc::c_int,
                         move_str:
                             b"f6f3g6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 60 as libc::c_int,
                         move_str:
                             b"f3g4g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"c4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 65 as libc::c_int,
                         move_str:
                             b"e2d2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 62 as libc::c_int,
                         move_str:
                             b"e6f7g4g5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 11 as libc::c_int,
                         frequency: 53 as libc::c_int,
                         move_str:
                             b"c4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"g5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"e7f6d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"f6g4g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"b5c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"c4c3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"e2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"f3g6e6f6d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"f6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"g5e6f6d7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 58 as libc::c_int,
                         move_str:
                             b"d3c4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"c3d3c4b3c2b4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"e6f7f3b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"c4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"d7e8c8c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"c7f6e7e8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 61 as libc::c_int,
                         move_str:
                             b"f6b5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"e7d8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"e7f6g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 10 as libc::c_int,
                         frequency: 51 as libc::c_int,
                         move_str:
                             b"d7d8c7c8f8b8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 12 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"f8c8c7b8g8\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 9 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"d3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"f6g3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 58 as libc::c_int,
                         move_str:
                             b"g4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 8 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"d7c7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"g5f6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"f3d3f7\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 52 as libc::c_int,
                         move_str:
                             b"d3c4f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"f6e6d3f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 44 as libc::c_int,
                         move_str:
                             b"d7e6f6d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"g5f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 68 as libc::c_int,
                         move_str:
                             b"c4d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 45 as libc::c_int,
                         move_str:
                             b"e6c6d3f3g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"d3f3e2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"e6g5c4c3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 41 as libc::c_int,
                         move_str:
                             b"c3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"f3g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 58 as libc::c_int,
                         move_str:
                             b"e3c4d3c6g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"d7f6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 73 as libc::c_int,
                         move_str:
                             b"c6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 40 as libc::c_int,
                         move_str:
                             b"g5\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"e7c6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"b4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 2 as libc::c_int,
                         frequency: 49 as libc::c_int,
                         move_str:
                             b"c6f6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"f4d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 42 as libc::c_int,
                         move_str:
                             b"f3c5\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 51 as libc::c_int,
                         move_str:
                             b"e6f6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"g5d3c4e3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"e3f6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"c5e3f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 47 as libc::c_int,
                         move_str:
                             b"b4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 2 as libc::c_int,
                         frequency: 56 as libc::c_int,
                         move_str:
                             b"c7f4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 37 as libc::c_int,
                         move_str:
                             b"f3\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 1 as libc::c_int,
                         frequency: 46 as libc::c_int,
                         move_str:
                             b"f4e3f6d3c5d6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"c4f3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"e2f2\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 6 as libc::c_int,
                         frequency: 38 as libc::c_int,
                         move_str:
                             b"f3g4\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 7 as libc::c_int,
                         frequency: 43 as libc::c_int,
                         move_str:
                             b"f2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 4 as libc::c_int,
                         frequency: 68 as libc::c_int,
                         move_str:
                             b"e6\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 33 as libc::c_int,
                         move_str:
                             b"d6f3g5f6\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 48 as libc::c_int,
                         move_str:
                             b"g4\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 5 as libc::c_int,
                         frequency: 36 as libc::c_int,
                         move_str:
                             b"e2\x00" as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 34 as libc::c_int,
                         move_str:
                             b"f2e2f6d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     },
     {
         let mut init =
             ThorOpening{first_unique: 3 as libc::c_int,
                         frequency: 39 as libc::c_int,
                         move_str:
                             b"d2e2f6d3\x00" as *const u8 as
                                 *const libc::c_char,};
         init
     }];
/* Local variables */
static mut thor_game_count: libc::c_int = 0;
static mut thor_database_count: libc::c_int = 0;
static mut thor_side_to_move: libc::c_int = 0;
static mut thor_sort_criteria_count: libc::c_int = 0;
static mut thor_games_sorted: libc::c_int = 0;
static mut thor_games_filtered: libc::c_int = 0;
static mut thor_row_pattern: [libc::c_int; 8] = [0; 8];
static mut thor_col_pattern: [libc::c_int; 8] = [0; 8];
static mut thor_board: [libc::c_int; 100] = [0; 100];
static mut b1_b1_map: [libc::c_int; 100] = [0; 100];
static mut g1_b1_map: [libc::c_int; 100] = [0; 100];
static mut g8_b1_map: [libc::c_int; 100] = [0; 100];
static mut b8_b1_map: [libc::c_int; 100] = [0; 100];
static mut a2_b1_map: [libc::c_int; 100] = [0; 100];
static mut a7_b1_map: [libc::c_int; 100] = [0; 100];
static mut h7_b1_map: [libc::c_int; 100] = [0; 100];
static mut h2_b1_map: [libc::c_int; 100] = [0; 100];
static mut primary_hash: [[libc::c_uint; 6561]; 8] = [[0; 6561]; 8];
static mut secondary_hash: [[libc::c_uint; 6561]; 8] = [[0; 6561]; 8];
static mut symmetry_map: [*mut libc::c_int; 8] =
    [0 as *const libc::c_int as *mut libc::c_int; 8];
static mut inv_symmetry_map: [*mut libc::c_int; 8] =
    [0 as *const libc::c_int as *mut libc::c_int; 8];
static mut move_mask_hi: [libc::c_uint; 100] = [0; 100];
static mut move_mask_lo: [libc::c_uint; 100] = [0; 100];
static mut unmove_mask_hi: [libc::c_uint; 100] = [0; 100];
static mut unmove_mask_lo: [libc::c_uint; 100] = [0; 100];
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
                           0 as *const libc::c_char as *mut libc::c_char,
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
                               0 as *const libc::c_char as *mut libc::c_char,
                           count: 0,
                           tournament_list:
                               0 as *const TournamentType as
                                   *mut TournamentType,};
static mut root_node: *mut ThorOpeningNode =
    0 as *const ThorOpeningNode as *mut ThorOpeningNode;
static mut default_sort_order: [libc::c_int; 5] =
    [2 as libc::c_int, 3 as libc::c_int, 1 as libc::c_int, 5 as libc::c_int,
     4 as libc::c_int];
static mut thor_sort_order: [libc::c_int; 10] = [0; 10];
static mut filter: FilterType =
    FilterType{game_categories: 0,
               first_year: 0,
               last_year: 0,
               player_filter: EitherSelectedFilter,};
/*
  CLEAR_THOR_BOARD
*/
unsafe extern "C" fn clear_thor_board() {
    let mut pos: libc::c_int = 0;
    pos = 11 as libc::c_int;
    while pos <= 88 as libc::c_int {
        thor_board[pos as usize] = 1 as libc::c_int;
        pos += 1
    }
    thor_board[54 as libc::c_int as usize] = 0 as libc::c_int;
    thor_board[45 as libc::c_int as usize] =
        thor_board[54 as libc::c_int as usize];
    thor_board[55 as libc::c_int as usize] = 2 as libc::c_int;
    thor_board[44 as libc::c_int as usize] =
        thor_board[55 as libc::c_int as usize];
}
/*
  PREPARE_THOR_BOARD
  Mark the positions outside the board as OUTSIDE.
*/
unsafe extern "C" fn prepare_thor_board() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        j = 0 as libc::c_int;
        pos = 10 as libc::c_int * i;
        while j < 10 as libc::c_int {
            if i == 0 as libc::c_int || i == 9 as libc::c_int ||
                   j == 0 as libc::c_int || j == 9 as libc::c_int {
                thor_board[pos as usize] = 3 as libc::c_int
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
unsafe extern "C" fn directional_flip_count(mut sq: libc::c_int,
                                            mut inc: libc::c_int,
                                            mut color: libc::c_int,
                                            mut oppcol: libc::c_int)
 -> libc::c_int {
    let mut count = 1 as libc::c_int;
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
    return 0 as libc::c_int;
}
/*
  DIRECTIONAL_FLIP_ANY
  Returns 1 if SQ is feasible for COLOR in the direction given by INC
  and flip the discs which are flipped if SQ is played.
*/
unsafe extern "C" fn directional_flip_any(mut sq: libc::c_int,
                                          mut inc: libc::c_int,
                                          mut color: libc::c_int,
                                          mut oppcol: libc::c_int)
 -> libc::c_int {
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
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/*
  COUNT_FLIPS
  Returns the number of discs flipped if SQNUM is played by COLOR
  and flips those discs (if there are any).
*/
unsafe extern "C" fn count_flips(mut sqnum: libc::c_int,
                                 mut color: libc::c_int,
                                 mut oppcol: libc::c_int) -> libc::c_int {
    let mut count: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    count = 0 as libc::c_int;
    mask = dir_mask[sqnum as usize];
    if mask & 128 as libc::c_int != 0 {
        count +=
            directional_flip_count(sqnum, -(11 as libc::c_int), color, oppcol)
    }
    if mask & 64 as libc::c_int != 0 {
        count +=
            directional_flip_count(sqnum, 11 as libc::c_int, color, oppcol)
    }
    if mask & 32 as libc::c_int != 0 {
        count +=
            directional_flip_count(sqnum, -(10 as libc::c_int), color, oppcol)
    }
    if mask & 16 as libc::c_int != 0 {
        count +=
            directional_flip_count(sqnum, 10 as libc::c_int, color, oppcol)
    }
    if mask & 8 as libc::c_int != 0 {
        count +=
            directional_flip_count(sqnum, -(9 as libc::c_int), color, oppcol)
    }
    if mask & 4 as libc::c_int != 0 {
        count +=
            directional_flip_count(sqnum, 9 as libc::c_int, color, oppcol)
    }
    if mask & 2 as libc::c_int != 0 {
        count +=
            directional_flip_count(sqnum, -(1 as libc::c_int), color, oppcol)
    }
    if mask & 1 as libc::c_int != 0 {
        count +=
            directional_flip_count(sqnum, 1 as libc::c_int, color, oppcol)
    }
    return count;
}
/*
  ANY_FLIPS
  Returns 1 if SQNUM flips any discs for COLOR, otherwise 0, and
  flips those discs.
*/
unsafe extern "C" fn any_flips(mut sqnum: libc::c_int, mut color: libc::c_int,
                               mut oppcol: libc::c_int) -> libc::c_int {
    let mut count: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    count = 0 as libc::c_int;
    mask = dir_mask[sqnum as usize];
    if mask & 128 as libc::c_int != 0 {
        count |=
            directional_flip_any(sqnum, -(11 as libc::c_int), color, oppcol)
    }
    if mask & 64 as libc::c_int != 0 {
        count |= directional_flip_any(sqnum, 11 as libc::c_int, color, oppcol)
    }
    if mask & 32 as libc::c_int != 0 {
        count |=
            directional_flip_any(sqnum, -(10 as libc::c_int), color, oppcol)
    }
    if mask & 16 as libc::c_int != 0 {
        count |= directional_flip_any(sqnum, 10 as libc::c_int, color, oppcol)
    }
    if mask & 8 as libc::c_int != 0 {
        count |=
            directional_flip_any(sqnum, -(9 as libc::c_int), color, oppcol)
    }
    if mask & 4 as libc::c_int != 0 {
        count |= directional_flip_any(sqnum, 9 as libc::c_int, color, oppcol)
    }
    if mask & 2 as libc::c_int != 0 {
        count |=
            directional_flip_any(sqnum, -(1 as libc::c_int), color, oppcol)
    }
    if mask & 1 as libc::c_int != 0 {
        count |= directional_flip_any(sqnum, 1 as libc::c_int, color, oppcol)
    }
    return count;
}
/*
  COMPUTE_THOR_PATTERNS
  Computes the row and column patterns.

*/
unsafe extern "C" fn compute_thor_patterns(mut in_board: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        thor_row_pattern[i as usize] = 0 as libc::c_int;
        thor_col_pattern[i as usize] = 0 as libc::c_int;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        j = 0 as libc::c_int;
        pos = 10 as libc::c_int * i + 11 as libc::c_int;
        while j < 8 as libc::c_int {
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
unsafe extern "C" fn get_corner_mask(mut disc_a1: libc::c_int,
                                     mut disc_a8: libc::c_int,
                                     mut disc_h1: libc::c_int,
                                     mut disc_h8: libc::c_int)
 -> libc::c_uint {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut mask_a1: libc::c_int = 0;
    let mut mask_a8: libc::c_int = 0;
    let mut mask_h1: libc::c_int = 0;
    let mut mask_h8: libc::c_int = 0;
    let mut out_mask: libc::c_uint = 0;
    let mut config: [libc::c_uint; 8] = [0; 8];
    mask_a1 = 0 as libc::c_int;
    if disc_a1 == 0 as libc::c_int {
        mask_a1 = 1 as libc::c_int
    } else if disc_a1 == 2 as libc::c_int { mask_a1 = 2 as libc::c_int }
    mask_a8 = 0 as libc::c_int;
    if disc_a8 == 0 as libc::c_int {
        mask_a8 = 1 as libc::c_int
    } else if disc_a8 == 2 as libc::c_int { mask_a8 = 2 as libc::c_int }
    mask_h1 = 0 as libc::c_int;
    if disc_h1 == 0 as libc::c_int {
        mask_h1 = 1 as libc::c_int
    } else if disc_h1 == 2 as libc::c_int { mask_h1 = 2 as libc::c_int }
    mask_h8 = 0 as libc::c_int;
    if disc_h8 == 0 as libc::c_int {
        mask_h8 = 1 as libc::c_int
    } else if disc_h8 == 2 as libc::c_int { mask_h8 = 2 as libc::c_int }
    count = 0 as libc::c_int;
    if disc_a1 != 1 as libc::c_int { count += 1 }
    if disc_a8 != 1 as libc::c_int { count += 1 }
    if disc_h1 != 1 as libc::c_int { count += 1 }
    if disc_h8 != 1 as libc::c_int { count += 1 }
    if count == 0 as libc::c_int { return 0 as libc::c_int as libc::c_uint }
    config[0 as libc::c_int as usize] =
        (mask_a1 + 4 as libc::c_int * mask_a8 + 16 as libc::c_int * mask_h1 +
             64 as libc::c_int * mask_h8) as libc::c_uint;
    config[1 as libc::c_int as usize] =
        (mask_a1 + 4 as libc::c_int * mask_h1 + 16 as libc::c_int * mask_a8 +
             64 as libc::c_int * mask_h8) as libc::c_uint;
    config[2 as libc::c_int as usize] =
        (mask_a8 + 4 as libc::c_int * mask_a1 + 16 as libc::c_int * mask_h8 +
             64 as libc::c_int * mask_h1) as libc::c_uint;
    config[3 as libc::c_int as usize] =
        (mask_a8 + 4 as libc::c_int * mask_h8 + 16 as libc::c_int * mask_a1 +
             64 as libc::c_int * mask_h1) as libc::c_uint;
    config[4 as libc::c_int as usize] =
        (mask_h1 + 4 as libc::c_int * mask_h8 + 16 as libc::c_int * mask_a1 +
             64 as libc::c_int * mask_a8) as libc::c_uint;
    config[5 as libc::c_int as usize] =
        (mask_h1 + 4 as libc::c_int * mask_a1 + 16 as libc::c_int * mask_h8 +
             64 as libc::c_int * mask_a8) as libc::c_uint;
    config[6 as libc::c_int as usize] =
        (mask_h8 + 4 as libc::c_int * mask_h1 + 16 as libc::c_int * mask_a8 +
             64 as libc::c_int * mask_a1) as libc::c_uint;
    config[7 as libc::c_int as usize] =
        (mask_h8 + 4 as libc::c_int * mask_a8 + 16 as libc::c_int * mask_h1 +
             64 as libc::c_int * mask_a1) as libc::c_uint;
    out_mask = config[0 as libc::c_int as usize];
    i = 1 as libc::c_int;
    while i < 8 as libc::c_int {
        out_mask =
            if out_mask < config[i as usize] {
                out_mask
            } else { config[i as usize] };
        i += 1
    }
    return out_mask << 8 as libc::c_int * (count - 1 as libc::c_int);
}
/*
  PLAY_THROUGH_GAME
  Play the MAX_MOVES first moves of GAME and update THOR_BOARD
  and THOR_SIDE_TO_MOVE to represent the position after those moves.
*/
unsafe extern "C" fn play_through_game(mut game: *mut GameType,
                                       mut max_moves: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut move_0: libc::c_int = 0;
    let mut flipped: libc::c_int = 0;
    clear_thor_board();
    thor_side_to_move = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < max_moves {
        move_0 = abs((*game).moves[i as usize] as libc::c_int);
        flipped =
            any_flips(move_0, thor_side_to_move,
                      0 as libc::c_int + 2 as libc::c_int -
                          thor_side_to_move);
        if flipped != 0 {
            thor_board[move_0 as usize] = thor_side_to_move;
            thor_side_to_move =
                0 as libc::c_int + 2 as libc::c_int - thor_side_to_move
        } else {
            thor_side_to_move =
                0 as libc::c_int + 2 as libc::c_int - thor_side_to_move;
            flipped =
                any_flips(move_0, thor_side_to_move,
                          0 as libc::c_int + 2 as libc::c_int -
                              thor_side_to_move);
            if flipped != 0 {
                thor_board[move_0 as usize] = thor_side_to_move;
                thor_side_to_move =
                    0 as libc::c_int + 2 as libc::c_int - thor_side_to_move
            } else { return 0 as libc::c_int }
        }
        i += 1
    }
    return 1 as libc::c_int;
}
/*
  PREPARE_GAME
  Performs off-line analysis of GAME to speed up subsequent requests.
  The main result is that the number of black discs on the board after
  each of the moves is stored.
*/
unsafe extern "C" fn prepare_game(mut game: *mut GameType) {
    let mut i: libc::c_int = 0;
    let mut move_0: libc::c_int = 0;
    let mut done: libc::c_int = 0;
    let mut flipped: libc::c_int = 0;
    let mut opening_match: libc::c_int = 0;
    let mut moves_played: libc::c_int = 0;
    let mut disc_count: [libc::c_int; 3] = [0; 3];
    let mut corner_descriptor: libc::c_uint = 0;
    let mut opening = 0 as *mut ThorOpeningNode;
    let mut child = 0 as *mut ThorOpeningNode;
    /* Play through the game and count the number of black discs
       at each stage. */
    clear_thor_board();
    disc_count[2 as libc::c_int as usize] = 2 as libc::c_int;
    disc_count[0 as libc::c_int as usize] =
        disc_count[2 as libc::c_int as usize];
    thor_side_to_move = 0 as libc::c_int;
    corner_descriptor = 0 as libc::c_int as libc::c_uint;
    moves_played = 0 as libc::c_int;
    done = 0 as libc::c_int;
    loop  {
        /* Store the number of black discs. */
        (*game).black_disc_count[moves_played as usize] =
            disc_count[0 as libc::c_int as usize] as libc::c_char;
        /* Make the move, update the board and disc count,
           and change the sign for white moves */
        move_0 = (*game).moves[moves_played as usize] as libc::c_int;
        flipped =
            count_flips(move_0, thor_side_to_move,
                        0 as libc::c_int + 2 as libc::c_int -
                            thor_side_to_move);
        if flipped != 0 {
            thor_board[move_0 as usize] = thor_side_to_move;
            disc_count[thor_side_to_move as usize] +=
                flipped + 1 as libc::c_int;
            disc_count[(0 as libc::c_int + 2 as libc::c_int -
                            thor_side_to_move) as usize] -= flipped;
            if thor_side_to_move == 2 as libc::c_int {
                (*game).moves[moves_played as usize] =
                    -((*game).moves[moves_played as usize] as libc::c_int) as
                        libc::c_char
            }
            thor_side_to_move =
                0 as libc::c_int + 2 as libc::c_int - thor_side_to_move;
            moves_played += 1
        } else {
            thor_side_to_move =
                0 as libc::c_int + 2 as libc::c_int - thor_side_to_move;
            flipped =
                count_flips(move_0, thor_side_to_move,
                            0 as libc::c_int + 2 as libc::c_int -
                                thor_side_to_move);
            if flipped != 0 {
                thor_board[move_0 as usize] = thor_side_to_move;
                disc_count[thor_side_to_move as usize] +=
                    flipped + 1 as libc::c_int;
                disc_count[(0 as libc::c_int + 2 as libc::c_int -
                                thor_side_to_move) as usize] -= flipped;
                if thor_side_to_move == 2 as libc::c_int {
                    (*game).moves[moves_played as usize] =
                        -((*game).moves[moves_played as usize] as libc::c_int)
                            as libc::c_char
                }
                thor_side_to_move =
                    0 as libc::c_int + 2 as libc::c_int - thor_side_to_move;
                moves_played += 1
            } else { done = 1 as libc::c_int }
        }
        /* Update the corner descriptor if necessary */
        if move_0 == 11 as libc::c_int || move_0 == 18 as libc::c_int ||
               move_0 == 81 as libc::c_int || move_0 == 88 as libc::c_int {
            corner_descriptor |=
                get_corner_mask(thor_board[11 as libc::c_int as usize],
                                thor_board[81 as libc::c_int as usize],
                                thor_board[18 as libc::c_int as usize],
                                thor_board[88 as libc::c_int as usize])
        }
        if !(done == 0 && moves_played < 60 as libc::c_int) { break ; }
    }
    (*game).black_disc_count[moves_played as usize] =
        disc_count[0 as libc::c_int as usize] as libc::c_char;
    (*game).move_count = moves_played as libc::c_short;
    i = moves_played + 1 as libc::c_int;
    while i <= 60 as libc::c_int {
        (*game).black_disc_count[i as usize] =
            -(1 as libc::c_int) as libc::c_char;
        i += 1
    }
    /* Find the longest opening which coincides with the game */
    opening = root_node;
    i = 0 as libc::c_int;
    opening_match = 1 as libc::c_int;
    while opening_match != 0 {
        move_0 = (*opening).child_move as libc::c_int;
        child = (*opening).child_node;
        while !child.is_null() &&
                  move_0 != abs((*game).moves[i as usize] as libc::c_int) {
            move_0 = (*child).sibling_move as libc::c_int;
            child = (*child).sibling_node
        }
        if child.is_null() {
            opening_match = 0 as libc::c_int
        } else { opening = child; i += 1 }
    }
    (*game).opening = opening;
    /* Initialize the shape state */
    (*game).shape_lo =
        ((3 as libc::c_int) << 27 as libc::c_int) as libc::c_uint;
    (*game).shape_hi =
        ((3 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
    (*game).shape_state_hi = 0 as libc::c_int as libc::c_short;
    (*game).shape_state_lo = 0 as libc::c_int as libc::c_short;
    /* Store the corner descriptor */
    (*game).corner_descriptor = corner_descriptor;
}
/*
  GET_INT_8
  Reads an 8-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
unsafe extern "C" fn get_int_8(mut stream: *mut FILE, mut value: *mut int_8)
 -> libc::c_int {
    let mut actually_read: libc::c_int = 0;
    actually_read =
        fread(value as *mut libc::c_void,
              ::std::mem::size_of::<int_8>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream) as libc::c_int;
    return (actually_read == 1 as libc::c_int) as libc::c_int;
}
/*
  GET_INT_16
  Reads a 16-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
unsafe extern "C" fn get_int_16(mut stream: *mut FILE, mut value: *mut int_16)
 -> libc::c_int {
    let mut actually_read: libc::c_int = 0;
    actually_read =
        fread(value as *mut libc::c_void,
              ::std::mem::size_of::<int_16>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream) as libc::c_int;
    return (actually_read == 1 as libc::c_int) as libc::c_int;
}
/*
  GET_INT_32
  Reads a 32-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
unsafe extern "C" fn get_int_32(mut stream: *mut FILE, mut value: *mut int_32)
 -> libc::c_int {
    let mut actually_read: libc::c_int = 0;
    actually_read =
        fread(value as *mut libc::c_void,
              ::std::mem::size_of::<int_32>() as libc::c_ulong,
              1 as libc::c_int as size_t, stream) as libc::c_int;
    return (actually_read == 1 as libc::c_int) as libc::c_int;
}
/*
  TOURNAMENT_NAME
  Returns the name of the INDEXth tournament if available.
*/
unsafe extern "C" fn tournament_name(mut index: libc::c_int)
 -> *const libc::c_char {
    if index < 0 as libc::c_int || index >= tournaments.count {
        return b"<Not available>\x00" as *const u8 as *const libc::c_char
    } else {
        return tournaments.name_buffer.offset((26 as libc::c_int * index) as
                                                  isize)
    };
}
/*
  TOURNAMENT_LEX_ORDER
  Returns the index into the lexicographical order of the
  INDEXth tournament if available, otherwise the last
  index + 1.
*/
unsafe extern "C" fn tournament_lex_order(mut index: libc::c_int)
 -> libc::c_int {
    if index < 0 as libc::c_int || index >= tournaments.count {
        return tournaments.count
    } else {
        return (*tournaments.tournament_list.offset(index as isize)).lex_order
    };
}
/*
  GET_PLAYER_NAME
  Returns the name of the INDEXth player if available.
*/
#[no_mangle]
pub unsafe extern "C" fn get_player_name(mut index: libc::c_int)
 -> *const libc::c_char {
    if index < 0 as libc::c_int || index >= players.count {
        return b"< Not available >\x00" as *const u8 as *const libc::c_char
    } else {
        return players.name_buffer.offset((20 as libc::c_int * index) as
                                              isize)
    };
}
/*
  GET_PLAYER_COUNT
  Returns the number of players in the database.
*/
#[no_mangle]
pub unsafe extern "C" fn get_player_count() -> libc::c_int {
    return players.count;
}
/*
  PLAYER_LEX_ORDER
  Returns the index into the lexicographical order of the
  INDEXth player if available, otherwise the last index + 1.
*/
unsafe extern "C" fn player_lex_order(mut index: libc::c_int) -> libc::c_int {
    if index < 0 as libc::c_int || index >= players.count {
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
unsafe extern "C" fn read_prolog(mut stream: *mut FILE,
                                 mut prolog: *mut PrologType) -> libc::c_int {
    let mut success: libc::c_int = 0;
    let mut byte_val: int_8 = 0;
    let mut word_val: int_16 = 0;
    let mut longint_val: int_32 = 0;
    success = get_int_8(stream, &mut byte_val);
    (*prolog).creation_century = byte_val as libc::c_int;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            libc::c_int;
    (*prolog).creation_year = byte_val as libc::c_int;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            libc::c_int;
    (*prolog).creation_month = byte_val as libc::c_int;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            libc::c_int;
    (*prolog).creation_day = byte_val as libc::c_int;
    success =
        (success != 0 && get_int_32(stream, &mut longint_val) != 0) as
            libc::c_int;
    (*prolog).game_count = longint_val;
    success =
        (success != 0 && get_int_16(stream, &mut word_val) != 0) as
            libc::c_int;
    (*prolog).item_count = word_val as libc::c_int;
    success =
        (success != 0 && get_int_16(stream, &mut word_val) != 0) as
            libc::c_int;
    (*prolog).origin_year = word_val as libc::c_int;
    success =
        (success != 0 && get_int_32(stream, &mut longint_val) != 0) as
            libc::c_int;
    (*prolog).reserved = longint_val;
    return success;
}
/*
  THOR_COMPARE_TOURNAMENTS
  Lexicographically compares the names of two tournaments
  represented by pointers.
*/
unsafe extern "C" fn thor_compare_tournaments(mut t1: *const libc::c_void,
                                              mut t2: *const libc::c_void)
 -> libc::c_int {
    let mut tournament1 = *(t1 as *mut *mut TournamentType);
    let mut tournament2 = *(t2 as *mut *mut TournamentType);
    return strcmp((*tournament1).name, (*tournament2).name);
}
/*
  SORT_TOURNAMENT_DATABASE
  Computes the lexicographic order of all tournaments in the database.
*/
unsafe extern "C" fn sort_tournament_database() {
    let mut tournament_buffer = 0 as *mut *mut TournamentType;
    let mut i: libc::c_int = 0;
    tournament_buffer =
        safe_malloc((tournaments.count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TournamentType>()
                                                         as libc::c_ulong)) as
            *mut *mut TournamentType;
    i = 0 as libc::c_int;
    while i < tournaments.count {
        let ref mut fresh0 = *tournament_buffer.offset(i as isize);
        *fresh0 =
            &mut *tournaments.tournament_list.offset(i as isize) as
                *mut TournamentType;
        i += 1
    }
    qsort(tournament_buffer as *mut libc::c_void, tournaments.count as size_t,
          ::std::mem::size_of::<*mut TournamentType>() as libc::c_ulong,
          Some(thor_compare_tournaments as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    i = 0 as libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn read_tournament_database(mut file_name:
                                                      *const libc::c_char)
 -> libc::c_int {
    let mut stream = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut success: libc::c_int = 0;
    let mut actually_read: libc::c_int = 0;
    let mut buffer_size: libc::c_int = 0;
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() { return 0 as libc::c_int }
    if read_prolog(stream, &mut tournaments.prolog) == 0 {
        fclose(stream);
        return 0 as libc::c_int
    }
    tournaments.count = tournaments.prolog.item_count;
    buffer_size = 26 as libc::c_int * tournaments.prolog.item_count;
    tournaments.name_buffer =
        safe_realloc(tournaments.name_buffer as *mut libc::c_void,
                     buffer_size as size_t) as *mut libc::c_char;
    actually_read =
        fread(tournaments.name_buffer as *mut libc::c_void,
              1 as libc::c_int as size_t, buffer_size as size_t, stream) as
            libc::c_int;
    success = (actually_read == buffer_size) as libc::c_int;
    fclose(stream);
    if success != 0 {
        tournaments.tournament_list =
            safe_realloc(tournaments.tournament_list as *mut libc::c_void,
                         (tournaments.count as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<TournamentType>()
                                                              as
                                                              libc::c_ulong))
                as *mut TournamentType;
        i = 0 as libc::c_int;
        while i < tournaments.count {
            let ref mut fresh1 =
                (*tournaments.tournament_list.offset(i as isize)).name;
            *fresh1 = tournament_name(i);
            (*tournaments.tournament_list.offset(i as isize)).selected =
                1 as libc::c_int;
            i += 1
        }
        sort_tournament_database();
        thor_games_sorted = 0 as libc::c_int;
        thor_games_filtered = 0 as libc::c_int
    }
    return success;
}
/*
  GET_TOURNAMENT_NAME
  Returns the name of the INDEXth tournament if available.
*/
#[no_mangle]
pub unsafe extern "C" fn get_tournament_name(mut index: libc::c_int)
 -> *const libc::c_char {
    if index < 0 as libc::c_int || index >= tournaments.count {
        return b"< Not available >\x00" as *const u8 as *const libc::c_char
    } else {
        return tournaments.name_buffer.offset((26 as libc::c_int * index) as
                                                  isize)
    };
}
/*
  GET_TOURNAMENT_COUNT
  Returns the number of players in the database.
*/
#[no_mangle]
pub unsafe extern "C" fn get_tournament_count() -> libc::c_int {
    return tournaments.count;
}
/*
  THOR_COMPARE_PLAYERS
  Lexicographically compares the names of two players
  represented by pointers.
*/
unsafe extern "C" fn thor_compare_players(mut p1: *const libc::c_void,
                                          mut p2: *const libc::c_void)
 -> libc::c_int {
    let mut ch: libc::c_char = 0;
    let mut buffer1: [libc::c_char; 20] = [0; 20];
    let mut buffer2: [libc::c_char; 20] = [0; 20];
    let mut i: libc::c_int = 0;
    let mut player1 = *(p1 as *mut *mut PlayerType);
    let mut player2 = *(p2 as *mut *mut PlayerType);
    i = 0 as libc::c_int;
    loop  {
        ch = *(*player1).name.offset(i as isize);
        buffer1[i as usize] = tolower(ch as libc::c_int) as libc::c_char;
        i += 1;
        if !(ch as libc::c_int != 0 as libc::c_int) { break ; }
    }
    if buffer1[0 as libc::c_int as usize] as libc::c_int == '?' as i32 {
        /* Put unknown players LAST */
        buffer1[0 as libc::c_int as usize] = '~' as i32 as libc::c_char
    }
    i = 0 as libc::c_int;
    loop  {
        ch = *(*player2).name.offset(i as isize);
        buffer2[i as usize] = tolower(ch as libc::c_int) as libc::c_char;
        i += 1;
        if !(ch as libc::c_int != 0 as libc::c_int) { break ; }
    }
    if buffer2[0 as libc::c_int as usize] as libc::c_int == '?' as i32 {
        /* Put unknown players LAST */
        buffer2[0 as libc::c_int as usize] = '~' as i32 as libc::c_char
    }
    return strcmp(buffer1.as_mut_ptr(), buffer2.as_mut_ptr());
}
/*
  SORT_PLAYER_DATABASE
  Computes the lexicographic order of all players in the database.
*/
unsafe extern "C" fn sort_player_database() {
    let mut player_buffer = 0 as *mut *mut PlayerType;
    let mut i: libc::c_int = 0;
    player_buffer =
        safe_malloc((players.count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut PlayerType>()
                                                         as libc::c_ulong)) as
            *mut *mut PlayerType;
    i = 0 as libc::c_int;
    while i < players.count {
        let ref mut fresh2 = *player_buffer.offset(i as isize);
        *fresh2 =
            &mut *players.player_list.offset(i as isize) as *mut PlayerType;
        i += 1
    }
    qsort(player_buffer as *mut libc::c_void, players.count as size_t,
          ::std::mem::size_of::<*mut PlayerType>() as libc::c_ulong,
          Some(thor_compare_players as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    i = 0 as libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn read_player_database(mut file_name:
                                                  *const libc::c_char)
 -> libc::c_int {
    let mut stream = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut success: libc::c_int = 0;
    let mut actually_read: libc::c_int = 0;
    let mut buffer_size: libc::c_int = 0;
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() { return 0 as libc::c_int }
    if read_prolog(stream, &mut players.prolog) == 0 {
        fclose(stream);
        return 0 as libc::c_int
    }
    players.count = players.prolog.item_count;
    buffer_size = 20 as libc::c_int * players.count;
    players.name_buffer =
        safe_realloc(players.name_buffer as *mut libc::c_void,
                     buffer_size as size_t) as *mut libc::c_char;
    actually_read =
        fread(players.name_buffer as *mut libc::c_void,
              1 as libc::c_int as size_t, buffer_size as size_t, stream) as
            libc::c_int;
    success = (actually_read == buffer_size) as libc::c_int;
    fclose(stream);
    if success != 0 {
        players.player_list =
            safe_realloc(players.player_list as *mut libc::c_void,
                         (players.count as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<PlayerType>()
                                                              as
                                                              libc::c_ulong))
                as *mut PlayerType;
        i = 0 as libc::c_int;
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
                    1 as libc::c_int
            } else {
                (*players.player_list.offset(i as isize)).is_program =
                    0 as libc::c_int
            }
            (*players.player_list.offset(i as isize)).selected =
                1 as libc::c_int;
            i += 1
        }
        sort_player_database();
        thor_games_sorted = 0 as libc::c_int;
        thor_games_filtered = 0 as libc::c_int
    }
    return success;
}
/*
  READ_GAME
  Reads a game from STREAM in GAME and prepares the game
  for database questions. Returns TRUE upon success,
  otherwise FALSE.
*/
unsafe extern "C" fn read_game(mut stream: *mut FILE, mut game: *mut GameType)
 -> libc::c_int {
    let mut success: libc::c_int = 0;
    let mut actually_read: libc::c_int = 0;
    let mut byte_val: int_8 = 0;
    let mut word_val: int_16 = 0;
    success = get_int_16(stream, &mut word_val);
    (*game).tournament_no = word_val;
    success =
        (success != 0 && get_int_16(stream, &mut word_val) != 0) as
            libc::c_int;
    (*game).black_no = word_val;
    success =
        (success != 0 && get_int_16(stream, &mut word_val) != 0) as
            libc::c_int;
    (*game).white_no = word_val;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            libc::c_int;
    (*game).actual_black_score = byte_val as libc::c_short;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            libc::c_int;
    (*game).perfect_black_score = byte_val as libc::c_short;
    actually_read =
        fread(&mut (*game).moves as *mut [libc::c_char; 60] as
                  *mut libc::c_void, 1 as libc::c_int as size_t,
              60 as libc::c_int as size_t, stream) as libc::c_int;
    prepare_game(game);
    return (success != 0 && actually_read == 60 as libc::c_int) as
               libc::c_int;
}
/*
  READ_GAME_DATABASE
  Reads a game database from FILE_NAME.
*/
#[no_mangle]
pub unsafe extern "C" fn read_game_database(mut file_name:
                                                *const libc::c_char)
 -> libc::c_int {
    let mut stream = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut success: libc::c_int = 0;
    let mut old_database_head = 0 as *mut DatabaseType;
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() { return 0 as libc::c_int }
    old_database_head = database_head;
    database_head =
        safe_malloc(::std::mem::size_of::<DatabaseType>() as libc::c_ulong) as
            *mut DatabaseType;
    (*database_head).next = old_database_head;
    if read_prolog(stream, &mut (*database_head).prolog) == 0 {
        fclose(stream);
        return 0 as libc::c_int
    }
    success = 1 as libc::c_int;
    (*database_head).count = (*database_head).prolog.game_count;
    (*database_head).games =
        safe_malloc(((*database_head).count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<GameType>()
                                                         as libc::c_ulong)) as
            *mut GameType;
    i = 0 as libc::c_int;
    while i < (*database_head).count {
        success =
            (success != 0 &&
                 read_game(stream,
                           &mut *(*database_head).games.offset(i as isize)) !=
                     0) as libc::c_int;
        let ref mut fresh4 =
            (*(*database_head).games.offset(i as isize)).database;
        *fresh4 = database_head;
        i += 1
    }
    thor_database_count += 1;
    thor_game_count += (*database_head).count;
    thor_games_sorted = 0 as libc::c_int;
    thor_games_filtered = 0 as libc::c_int;
    fclose(stream);
    return success;
}
/*
  GAME_DATABASE_ALREADY_LOADED
  Checks if the game database in FILE_NAME already exists in memory
  (the only thing actually checked is the prolog but this suffices
  according to the specification of the database format).
*/
#[no_mangle]
pub unsafe extern "C" fn game_database_already_loaded(mut file_name:
                                                          *const libc::c_char)
 -> libc::c_int {
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
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() { return 0 as libc::c_int }
    if read_prolog(stream, &mut new_prolog) == 0 {
        fclose(stream);
        return 0 as libc::c_int
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
            return 1 as libc::c_int
        }
        current_db = (*current_db).next
    }
    return 0 as libc::c_int;
}
/*
  GET_DATABASE_COUNT
  Returns the number of game databases currently loaded.
*/
#[no_mangle]
pub unsafe extern "C" fn get_database_count() -> libc::c_int {
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
#[no_mangle]
pub unsafe extern "C" fn get_database_info(mut info: *mut DatabaseInfoType) {
    let mut i: libc::c_int = 0;
    let mut change: libc::c_int = 0;
    let mut temp = DatabaseInfoType{year: 0, count: 0,};
    let mut current_db = 0 as *mut DatabaseType;
    current_db = database_head;
    i = 0 as libc::c_int;
    while i < thor_database_count {
        (*info.offset(i as isize)).year = (*current_db).prolog.origin_year;
        (*info.offset(i as isize)).count = (*current_db).count;
        current_db = (*current_db).next;
        i += 1
    }
    loop 
         /* Sort the list */
         {
        change = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < thor_database_count - 1 as libc::c_int {
            if (*info.offset(i as isize)).year >
                   (*info.offset((i + 1 as libc::c_int) as isize)).year {
                change = 1 as libc::c_int;
                temp = *info.offset(i as isize);
                *info.offset(i as isize) =
                    *info.offset((i + 1 as libc::c_int) as isize);
                *info.offset((i + 1 as libc::c_int) as isize) = temp
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
unsafe extern "C" fn print_game(mut stream: *mut FILE,
                                mut game: *mut GameType,
                                mut display_moves: libc::c_int) {
    let mut i: libc::c_int = 0;
    fprintf(stream, b"%s  %d\n\x00" as *const u8 as *const libc::c_char,
            tournament_name((*game).tournament_no as libc::c_int),
            (*(*game).database).prolog.origin_year);
    fprintf(stream, b"%s %s %s\n\x00" as *const u8 as *const libc::c_char,
            get_player_name((*game).black_no as libc::c_int),
            b"vs\x00" as *const u8 as *const libc::c_char,
            get_player_name((*game).white_no as libc::c_int));
    fprintf(stream, b"%d - %d   \x00" as *const u8 as *const libc::c_char,
            (*game).actual_black_score as libc::c_int,
            64 as libc::c_int - (*game).actual_black_score as libc::c_int);
    fprintf(stream,
            b"[ %d - %d %s ]\n\x00" as *const u8 as *const libc::c_char,
            (*game).perfect_black_score as libc::c_int,
            64 as libc::c_int - (*game).perfect_black_score as libc::c_int,
            b"perfect\x00" as *const u8 as *const libc::c_char);
    if display_moves != 0 {
        i = 0 as libc::c_int;
        while i < 60 as libc::c_int {
            fprintf(stream, b" %d\x00" as *const u8 as *const libc::c_char,
                    abs((*game).moves[i as usize] as libc::c_int));
            if i % 20 as libc::c_int == 19 as libc::c_int {
                fputs(b"\n\x00" as *const u8 as *const libc::c_char, stream);
            }
            i += 1
        }
    }
    fputs(b"\n\x00" as *const u8 as *const libc::c_char, stream);
}
/*
  COMPUTE_PARTIAL_HASH
  Computes the primary and secondary hash values for the
  unit element in the rotation group.
*/
unsafe extern "C" fn compute_partial_hash(mut hash_val1: *mut libc::c_uint,
                                          mut hash_val2: *mut libc::c_uint) {
    let mut i: libc::c_int = 0;
    *hash_val1 = 0 as libc::c_int as libc::c_uint;
    *hash_val2 = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
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
unsafe extern "C" fn compute_full_primary_hash(mut hash_val:
                                                   *mut libc::c_uint) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *hash_val.offset(i as isize) = 0 as libc::c_int as libc::c_uint;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        /* b1 -> b1 */
        *hash_val.offset(0 as libc::c_int as isize) ^=
            primary_hash[i as usize][thor_row_pattern[i as usize] as usize];
        /* b8 -> b1 */
        *hash_val.offset(1 as libc::c_int as isize) ^=
            primary_hash[i as
                             usize][thor_row_pattern[(7 as libc::c_int - i) as
                                                         usize] as usize];
        /* a2 -> b1 */
        *hash_val.offset(2 as libc::c_int as isize) ^=
            primary_hash[i as usize][thor_col_pattern[i as usize] as usize];
        /* h2 -> b1 */
        *hash_val.offset(3 as libc::c_int as isize) ^=
            primary_hash[i as
                             usize][thor_col_pattern[(7 as libc::c_int - i) as
                                                         usize] as usize];
        i += 1
    }
    /* g1 -> b1 */
    *hash_val.offset(4 as libc::c_int as isize) =
        bit_reverse_32(*hash_val.offset(0 as libc::c_int as isize));
    /* g8 -> b1 */
    *hash_val.offset(5 as libc::c_int as isize) =
        bit_reverse_32(*hash_val.offset(1 as libc::c_int as isize));
    /* a7 -> b1 */
    *hash_val.offset(6 as libc::c_int as isize) =
        bit_reverse_32(*hash_val.offset(2 as libc::c_int as isize));
    /* h7 -> b1 */
    *hash_val.offset(7 as libc::c_int as isize) =
        bit_reverse_32(*hash_val.offset(3 as libc::c_int as isize));
}
unsafe extern "C" fn compute_full_secondary_hash(mut hash_val:
                                                     *mut libc::c_uint) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *hash_val.offset(i as isize) = 0 as libc::c_int as libc::c_uint;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        /* b1 -> b1 */
        *hash_val.offset(0 as libc::c_int as isize) ^=
            secondary_hash[i as usize][thor_row_pattern[i as usize] as usize];
        /* b8 -> b1 */
        *hash_val.offset(1 as libc::c_int as isize) ^=
            secondary_hash[i as
                               usize][thor_row_pattern[(7 as libc::c_int - i)
                                                           as usize] as
                                          usize];
        /* a2 -> b1 */
        *hash_val.offset(2 as libc::c_int as isize) ^=
            secondary_hash[i as usize][thor_col_pattern[i as usize] as usize];
        /* h2 -> b1 */
        *hash_val.offset(3 as libc::c_int as isize) ^=
            secondary_hash[i as
                               usize][thor_col_pattern[(7 as libc::c_int - i)
                                                           as usize] as
                                          usize];
        i += 1
    }
    /* g1 -> b1 */
    *hash_val.offset(4 as libc::c_int as isize) =
        bit_reverse_32(*hash_val.offset(0 as libc::c_int as isize));
    /* g8 -> b1 */
    *hash_val.offset(5 as libc::c_int as isize) =
        bit_reverse_32(*hash_val.offset(1 as libc::c_int as isize));
    /* a7 -> b1 */
    *hash_val.offset(6 as libc::c_int as isize) =
        bit_reverse_32(*hash_val.offset(2 as libc::c_int as isize));
    /* h7 -> b1 */
    *hash_val.offset(7 as libc::c_int as isize) =
        bit_reverse_32(*hash_val.offset(3 as libc::c_int as isize));
}
/*
  PRIMARY_HASH_LOOKUP
  Checks if any of the rotations of the current pattern set
  match the primary hash code TARGET_HASH.
*/
unsafe extern "C" fn primary_hash_lookup(mut target_hash: libc::c_uint)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut hit_mask: libc::c_int = 0;
    let mut hash_val: [libc::c_uint; 8] = [0; 8];
    compute_full_primary_hash(hash_val.as_mut_ptr());
    hit_mask = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if hash_val[i as usize] == target_hash {
            hit_mask |= (1 as libc::c_int) << i
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
unsafe extern "C" fn secondary_hash_lookup(mut target_hash: libc::c_uint)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut hit_mask: libc::c_int = 0;
    let mut hash_val: [libc::c_uint; 8] = [0; 8];
    compute_full_secondary_hash(hash_val.as_mut_ptr());
    hit_mask = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if hash_val[i as usize] == target_hash {
            hit_mask |= (1 as libc::c_int) << i
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
unsafe extern "C" fn thor_compare(mut g1: *const libc::c_void,
                                  mut g2: *const libc::c_void)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut game1 = *(g1 as *mut *mut GameType);
    let mut game2 = *(g2 as *mut *mut GameType);
    i = 0 as libc::c_int;
    while i < thor_sort_criteria_count {
        match thor_sort_order[i as usize] {
            1 => {
                result =
                    (*(*game2).database).prolog.origin_year -
                        (*(*game1).database).prolog.origin_year
            }
            2 => {
                result =
                    player_lex_order((*game1).black_no as libc::c_int) -
                        player_lex_order((*game2).black_no as libc::c_int)
            }
            3 => {
                result =
                    player_lex_order((*game1).white_no as libc::c_int) -
                        player_lex_order((*game2).white_no as libc::c_int)
            }
            4 => {
                result =
                    tournament_lex_order((*game1).tournament_no as
                                             libc::c_int) -
                        tournament_lex_order((*game2).tournament_no as
                                                 libc::c_int)
            }
            5 => {
                result =
                    (*game1).actual_black_score as libc::c_int -
                        (*game2).actual_black_score as libc::c_int
            }
            6 => {
                result =
                    (*game2).actual_black_score as libc::c_int -
                        (*game1).actual_black_score as libc::c_int
            }
            0 | _ => {
                /* Really can't happen */
                result =
                    (*(*game1).database).prolog.origin_year -
                        (*(*game2).database).prolog.origin_year
            }
        }
        if result != 0 as libc::c_int { return result }
        i += 1
    }
    /* If control reaches this point, the two games couldn't be
       distinguished by the current search criteria. */
    return 0 as libc::c_int;
}
/*
  FILTER_DATABASE
  Applies the current filter rules to the database DB.
*/
unsafe extern "C" fn filter_database(mut db: *mut DatabaseType) {
    let mut i: libc::c_int = 0;
    let mut category: libc::c_int = 0;
    let mut passes_filter: libc::c_int = 0;
    let mut year: libc::c_int = 0;
    let mut game = 0 as *mut GameType;
    i = 0 as libc::c_int;
    while i < (*db).count {
        game = &mut *(*db).games.offset(i as isize) as *mut GameType;
        passes_filter = 1 as libc::c_int;
        /* Apply the tournament filter */
        if passes_filter != 0 &&
               (*tournaments.tournament_list.offset((*game).tournament_no as
                                                        isize)).selected == 0
           {
            passes_filter = 0 as libc::c_int
        }
        /* Apply the year filter */
        if passes_filter != 0 {
            year = (*(*game).database).prolog.origin_year;
            if year < filter.first_year || year > filter.last_year {
                passes_filter = 0 as libc::c_int
            }
        }
        /* Apply the player filter */
        if passes_filter != 0 {
            match filter.player_filter as libc::c_uint {
                0 => {
                    if (*players.player_list.offset((*game).black_no as
                                                        isize)).selected == 0
                           &&
                           (*players.player_list.offset((*game).white_no as
                                                            isize)).selected
                               == 0 {
                        passes_filter = 0 as libc::c_int
                    }
                }
                1 => {
                    if (*players.player_list.offset((*game).black_no as
                                                        isize)).selected == 0
                           ||
                           (*players.player_list.offset((*game).white_no as
                                                            isize)).selected
                               == 0 {
                        passes_filter = 0 as libc::c_int
                    }
                }
                2 => {
                    if (*players.player_list.offset((*game).black_no as
                                                        isize)).selected == 0
                       {
                        passes_filter = 0 as libc::c_int
                    }
                }
                3 => {
                    if (*players.player_list.offset((*game).white_no as
                                                        isize)).selected == 0
                       {
                        passes_filter = 0 as libc::c_int
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
                    category = 4 as libc::c_int
                } else { category = 2 as libc::c_int }
            } else if (*players.player_list.offset((*game).white_no as
                                                       isize)).is_program != 0
             {
                category = 2 as libc::c_int
            } else { category = 1 as libc::c_int }
            passes_filter = category & filter.game_categories
        }
        (*game).passes_filter = passes_filter as libc::c_short;
        i += 1
    };
}
/*
  FILTER_ALL_DATABASES
  Applies the current filter rules to all databases.
*/
unsafe extern "C" fn filter_all_databases() {
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
#[no_mangle]
pub unsafe extern "C" fn set_player_filter(mut selected: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < players.count {
        (*players.player_list.offset(i as isize)).selected =
            *selected.offset(i as isize);
        i += 1
    }
    thor_games_filtered = 0 as libc::c_int;
}
/*
  SET_PLAYER_FILTER_TYPE
  Specifies whether it suffices for a game to contain one selected
  player or if both players have to be selected for it be displayed.
*/
#[no_mangle]
pub unsafe extern "C" fn set_player_filter_type(mut player_filter:
                                                    PlayerFilterType) {
    filter.player_filter = player_filter;
}
/*
  SET_TOURNAMENT_FILTER
  Specify what tournaments to search for. The boolean vector SELECTED
  must contain at least TOURNAMENTS.COUNT values - check with
  GET_TOURNAMENT_COUNT() if necessary.
*/
#[no_mangle]
pub unsafe extern "C" fn set_tournament_filter(mut selected:
                                                   *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < tournaments.count {
        (*tournaments.tournament_list.offset(i as isize)).selected =
            *selected.offset(i as isize);
        i += 1
    }
    thor_games_filtered = 0 as libc::c_int;
}
/*
  SET_YEAR_FILTER
  Specify the interval of years to which the search will be confined.
*/
#[no_mangle]
pub unsafe extern "C" fn set_year_filter(mut first_year: libc::c_int,
                                         mut last_year: libc::c_int) {
    filter.first_year = first_year;
    filter.last_year = last_year;
    thor_games_filtered = 0 as libc::c_int;
}
/*
  SPECIFY_GAME_CATEGORIES
  Specify the types of games in the database that are displayed
  if they match the position probed for. The input is the binary
  OR of the flags for the types enabled.
*/
#[no_mangle]
pub unsafe extern "C" fn specify_game_categories(mut categories:
                                                     libc::c_int) {
    if categories != filter.game_categories {
        filter.game_categories = categories;
        thor_games_filtered = 0 as libc::c_int
    };
}
/*
  SORT_THOR_GAMES
  Sorts the COUNT first games in the list THOR_SEARCH.MATCH_LIST.
  The first THOR_SORT_CRITERIA_COUNT entries of THOR_SORT_ORDER are
  used (in order) to sort the matches.
*/
unsafe extern "C" fn sort_thor_games(mut count: libc::c_int) {
    if count <= 1 as libc::c_int {
        /* No need to sort 0 or 1 games. */
        return
    }
    qsort(thor_search.match_list as *mut libc::c_void, count as size_t,
          ::std::mem::size_of::<*mut GameType>() as libc::c_ulong,
          Some(thor_compare as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
}
/*
  SPECIFY_THOR_SORT_ORDER
  Specifies that in subsequent calls to SORT_THOR_MATCHES,
  the COUNT first elements of SORT_ORDER are to be used
  (in decreasing order of priority).
  Note: If there aren't (at least) COUNT elements at the location
        to which SORT_ORDER points, a crash is likely.
*/
#[no_mangle]
pub unsafe extern "C" fn specify_thor_sort_order(mut count: libc::c_int,
                                                 mut sort_order:
                                                     *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    /* Truncate the input vector if it is too long */
    count = if count < 10 as libc::c_int { count } else { 10 as libc::c_int };
    /* Check if the new order coincides with the old order */
    if count != thor_sort_criteria_count {
        thor_games_sorted = 0 as libc::c_int
    } else {
        i = 0 as libc::c_int;
        while i < count {
            if *sort_order.offset(i as isize) != thor_sort_order[i as usize] {
                thor_games_sorted = 0 as libc::c_int
            }
            i += 1
        }
    }
    thor_sort_criteria_count = count;
    i = 0 as libc::c_int;
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
unsafe extern "C" fn recursive_opening_scan(mut node: *mut ThorOpeningNode,
                                            mut depth: libc::c_int,
                                            mut moves_played: libc::c_int,
                                            mut primary_hash_0:
                                                *mut libc::c_uint,
                                            mut secondary_hash_0:
                                                *mut libc::c_uint) {
    let mut i: libc::c_int = 0;
    let mut match_0: libc::c_int = 0;
    let mut matching_symmetry: libc::c_int = 0;
    let mut child = 0 as *mut ThorOpeningNode;
    /* Determine the status of the current node */
    if depth < moves_played {
        (*node).matching_symmetry = 0 as libc::c_int;
        (*node).current_match = 0 as libc::c_int
    } else if depth == moves_played {
        /* Check the hash codes */
        match_0 = 0 as libc::c_int;
        matching_symmetry = 0 as libc::c_int;
        i = 7 as libc::c_int;
        while i >= 0 as libc::c_int {
            if (*node).hash1 == *primary_hash_0.offset(i as isize) &&
                   (*node).hash2 == *secondary_hash_0.offset(i as isize) {
                match_0 = 1 as libc::c_int;
                matching_symmetry = i
            }
            i -= 1
        }
        if match_0 != 0 {
            (*node).matching_symmetry = matching_symmetry;
            (*node).current_match = 1 as libc::c_int
        } else { (*node).current_match = 2 as libc::c_int }
    } else {
        /* depth > moves_played */
        (*node).current_match = (*(*node).parent_node).current_match;
        (*node).matching_symmetry = (*(*node).parent_node).matching_symmetry
    }
    /* Recursively search the childen */
    child = (*node).child_node;
    while !child.is_null() {
        recursive_opening_scan(child, depth + 1 as libc::c_int, moves_played,
                               primary_hash_0, secondary_hash_0);
        child = (*child).sibling_node
    };
}
/*
  OPENING_SCAN
  Fills the opening tree with information on how well
  the current pattern configuration matches the openings.
*/
unsafe extern "C" fn opening_scan(mut moves_played: libc::c_int) {
    let mut primary_hash_0: [libc::c_uint; 8] = [0; 8];
    let mut secondary_hash_0: [libc::c_uint; 8] = [0; 8];
    compute_full_primary_hash(primary_hash_0.as_mut_ptr());
    compute_full_secondary_hash(secondary_hash_0.as_mut_ptr());
    recursive_opening_scan(root_node, 0 as libc::c_int, moves_played,
                           primary_hash_0.as_mut_ptr(),
                           secondary_hash_0.as_mut_ptr());
}
/*
  RECURSIVE_FREQUENCY_COUNT
  Recursively fills frequency table FREQ_COUNT which is to contain
  the number of times each move has been played according to the
  trimmed set of openings from the Thor database.
*/
unsafe extern "C" fn recursive_frequency_count(mut node: *mut ThorOpeningNode,
                                               mut freq_count:
                                                   *mut libc::c_int,
                                               mut depth: libc::c_int,
                                               mut moves_played: libc::c_int,
                                               mut symmetries:
                                                   *mut libc::c_int,
                                               mut primary_hash_0:
                                                   *mut libc::c_uint,
                                               mut secondary_hash_0:
                                                   *mut libc::c_uint) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut child_move: libc::c_int = 0;
    let mut child = 0 as *mut ThorOpeningNode;
    if depth == moves_played {
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            j = *symmetries.offset(i as isize);
            if (*node).hash1 == *primary_hash_0.offset(j as isize) &&
                   (*node).hash2 == *secondary_hash_0.offset(j as isize) {
                child_move = (*node).child_move as libc::c_int;
                child = (*node).child_node;
                while !child.is_null() {
                    *freq_count.offset(*inv_symmetry_map[j as
                                                             usize].offset(child_move
                                                                               as
                                                                               isize)
                                           as isize) += (*child).frequency;
                    child_move = (*child).sibling_move as libc::c_int;
                    child = (*child).sibling_node
                }
                break ;
            } else { i += 1 }
        }
    } else if depth < moves_played {
        child = (*node).child_node;
        while !child.is_null() {
            recursive_frequency_count(child, freq_count,
                                      depth + 1 as libc::c_int, moves_played,
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
#[no_mangle]
pub unsafe extern "C" fn choose_thor_opening_move(mut in_board:
                                                      *mut libc::c_int,
                                                  mut side_to_move:
                                                      libc::c_int,
                                                  mut echo: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut temp_symm: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut disc_count: libc::c_int = 0;
    let mut freq_sum: libc::c_int = 0;
    let mut acc_freq_sum: libc::c_int = 0;
    let mut random_move: libc::c_int = 0;
    let mut random_value: libc::c_int = 0;
    let mut match_count: libc::c_int = 0;
    let mut symmetries: [libc::c_int; 8] = [0; 8];
    let mut freq_count: [libc::c_int; 100] = [0; 100];
    let mut primary_hash_0: [libc::c_uint; 8] = [0; 8];
    let mut secondary_hash_0: [libc::c_uint; 8] = [0; 8];
    let mut move_list: [C2RustUnnamed; 64] =
        [C2RustUnnamed{move_0: 0, frequency: 0,}; 64];
    let mut temp = C2RustUnnamed{move_0: 0, frequency: 0,};
    disc_count = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        pos = 10 as libc::c_int * i + 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            freq_count[pos as usize] = 0 as libc::c_int;
            if *in_board.offset(pos as isize) != 1 as libc::c_int {
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
    if side_to_move == 0 as libc::c_int &&
           disc_count % 2 as libc::c_int == 1 as libc::c_int {
        return -(1 as libc::c_int)
    }
    if side_to_move == 2 as libc::c_int &&
           disc_count % 2 as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    /* Create a random permutation of the symmetries to avoid the same
       symmetry always being chosen in e.g. the initial position */
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int { symmetries[i as usize] = i; i += 1 }
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        j = i + abs(my_random() as libc::c_int) % (8 as libc::c_int - i);
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
                              0 as libc::c_int, disc_count - 4 as libc::c_int,
                              symmetries.as_mut_ptr(),
                              primary_hash_0.as_mut_ptr(),
                              secondary_hash_0.as_mut_ptr());
    freq_sum = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        pos = 10 as libc::c_int * i + 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            freq_sum += freq_count[pos as usize];
            j += 1;
            pos += 1
        }
        i += 1
    }
    if freq_sum > 0 as libc::c_int {
        /* Position found in Thor opening tree */
        /* Create a list of the moves chosen from the position and also
           randomly select one of them. Probability for each move is
           proportional to the frequency of that move being played here. */
        random_value = abs(my_random() as libc::c_int) % freq_sum;
        random_move = -(1 as libc::c_int);
        acc_freq_sum = 0 as libc::c_int;
        match_count = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= 8 as libc::c_int {
            j = 1 as libc::c_int;
            pos = 10 as libc::c_int * i + 1 as libc::c_int;
            while j <= 8 as libc::c_int {
                if freq_count[pos as usize] > 0 as libc::c_int {
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
            i = 0 as libc::c_int;
            while i < match_count {
                j = 0 as libc::c_int;
                while j < match_count - 1 as libc::c_int {
                    if move_list[j as usize].frequency <
                           move_list[(j + 1 as libc::c_int) as
                                         usize].frequency {
                        temp = move_list[j as usize];
                        move_list[j as usize] =
                            move_list[(j + 1 as libc::c_int) as usize];
                        move_list[(j + 1 as libc::c_int) as usize] = temp
                    }
                    j += 1
                }
                i += 1
            }
            printf(b"%s:        \x00" as *const u8 as *const libc::c_char,
                   b"Thor database\x00" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < match_count {
                printf(b"%c%c: %4.1f%%    \x00" as *const u8 as
                           *const libc::c_char,
                       'a' as i32 +
                           move_list[i as usize].move_0 % 10 as libc::c_int -
                           1 as libc::c_int,
                       '0' as i32 +
                           move_list[i as usize].move_0 / 10 as libc::c_int,
                       100.0f64 *
                           move_list[i as usize].frequency as libc::c_double /
                           freq_sum as libc::c_double);
                if i % 6 as libc::c_int == 4 as libc::c_int {
                    puts(b"\x00" as *const u8 as *const libc::c_char);
                }
                i += 1
            }
            if match_count % 6 as libc::c_int != 5 as libc::c_int {
                puts(b"\x00" as *const u8 as *const libc::c_char);
            }
        }
        return random_move
    }
    return -(1 as libc::c_int);
}
/*
  POSITION_MATCH
  Returns TRUE if the position after MOVE_COUNT moves of GAME, with
  SIDE_TO_MOVE being the player to move, matches the hash codes
  IN_HASH1 and IN_HASH2, otherwise FALSE.
*/
unsafe extern "C" fn position_match(mut game: *mut GameType,
                                    mut move_count: libc::c_int,
                                    mut side_to_move: libc::c_int,
                                    mut shape_lo: *mut libc::c_uint,
                                    mut shape_hi: *mut libc::c_uint,
                                    mut corner_mask: libc::c_uint,
                                    mut in_hash1: libc::c_uint,
                                    mut in_hash2: libc::c_uint)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut shape_match: libc::c_int = 0;
    let mut primary_hit_mask: libc::c_int = 0;
    let mut secondary_hit_mask: libc::c_int = 0;
    /* Verify that the number of moves and the side-to-move status
       are correct */
    if move_count >= (*game).move_count as libc::c_int {
        if move_count > (*game).move_count as libc::c_int {
            /* Too many moves! */
            return 0 as libc::c_int
        }
        /* No side-to-move status to check if the game is over */
    } else if side_to_move == 0 as libc::c_int {
        /* Black to move */
        if ((*game).moves[move_count as usize] as libc::c_int) <
               0 as libc::c_int {
            /* White to move in the game */
            return 0 as libc::c_int
        }
    } else if (*game).moves[move_count as usize] as libc::c_int >
                  0 as libc::c_int {
        /* White to move */
        /* Black to move in the game */
        return 0 as libc::c_int
    }
    /* Check if the opening information suffices to
       determine if the position matches or not. */
    if (*(*game).opening).current_match == 1 as libc::c_int {
        (*game).matching_symmetry =
            (*(*game).opening).matching_symmetry as libc::c_short;
        return 1 as libc::c_int
    } else {
        if (*(*game).opening).current_match == 2 as libc::c_int {
            return 0 as libc::c_int
        }
    }
    /* Check if the lower 32 bits of the shape state coincide */
    if ((*game).shape_state_lo as libc::c_int) < move_count {
        i = (*game).shape_state_lo as libc::c_int;
        while i < move_count {
            (*game).shape_lo |=
                move_mask_lo[abs((*game).moves[i as usize] as libc::c_int) as
                                 usize];
            i += 1
        }
        (*game).shape_state_lo = move_count as libc::c_short
    } else if (*game).shape_state_lo as libc::c_int > move_count {
        i = (*game).shape_state_lo as libc::c_int - 1 as libc::c_int;
        while i >= move_count {
            (*game).shape_lo &=
                !move_mask_lo[abs((*game).moves[i as usize] as libc::c_int) as
                                  usize];
            i -= 1
        }
        (*game).shape_state_lo = move_count as libc::c_short
    }
    shape_match = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        shape_match |=
            ((*game).shape_lo == *shape_lo.offset(i as isize)) as libc::c_int;
        i += 1
    }
    if shape_match == 0 { return 0 as libc::c_int }
    /* Check if the upper 32 bits of the shape state coincide */
    if ((*game).shape_state_hi as libc::c_int) < move_count {
        i = (*game).shape_state_hi as libc::c_int;
        while i < move_count {
            (*game).shape_hi |=
                move_mask_hi[abs((*game).moves[i as usize] as libc::c_int) as
                                 usize];
            i += 1
        }
        (*game).shape_state_hi = move_count as libc::c_short
    } else if (*game).shape_state_hi as libc::c_int > move_count {
        i = (*game).shape_state_hi as libc::c_int - 1 as libc::c_int;
        while i >= move_count {
            (*game).shape_hi &=
                !move_mask_hi[abs((*game).moves[i as usize] as libc::c_int) as
                                  usize];
            i -= 1
        }
        (*game).shape_state_hi = move_count as libc::c_short
    }
    shape_match = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        shape_match |=
            ((*game).shape_hi == *shape_hi.offset(i as isize)) as libc::c_int;
        i += 1
    }
    if shape_match == 0 { return 0 as libc::c_int }
    /* Check if the corner mask is compatible with that of the game */
    if corner_mask & !(*game).corner_descriptor != 0 {
        return 0 as libc::c_int
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
                i = 0 as libc::c_int;
                while i < 8 as libc::c_int {
                    if primary_hit_mask & secondary_hit_mask &
                           (1 as libc::c_int) << i != 0 {
                        (*game).matching_symmetry = i as libc::c_short;
                        return 1 as libc::c_int
                    }
                    i += 1
                }
            }
        }
    }
    return 0 as libc::c_int;
}
/*
  DATABASE_SEARCH
  Determines what positions in the Thor database match the position
  given by IN_BOARD with SIDE_TO_MOVE being the player whose turn it is.
*/
#[no_mangle]
pub unsafe extern "C" fn database_search(mut in_board: *mut libc::c_int,
                                         mut side_to_move: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    let mut move_count: libc::c_int = 0;
    let mut symmetry: libc::c_int = 0;
    let mut next_move: libc::c_int = 0;
    let mut disc_count: [libc::c_int; 3] = [0; 3];
    let mut frequency: [libc::c_int; 65] = [0; 65];
    let mut cumulative: [libc::c_int; 65] = [0; 65];
    let mut target_hash1: libc::c_uint = 0;
    let mut target_hash2: libc::c_uint = 0;
    let mut corner_mask: libc::c_uint = 0;
    let mut shape_lo: [libc::c_uint; 8] = [0; 8];
    let mut shape_hi: [libc::c_uint; 8] = [0; 8];
    let mut current_db = 0 as *mut DatabaseType;
    let mut game = 0 as *mut GameType;
    /* We need a player and a tournament database. */
    if players.count == 0 as libc::c_int ||
           tournaments.count == 0 as libc::c_int {
        thor_search.match_count = 0 as libc::c_int;
        return
    }
    /* Make sure there's memory allocated if all positions
       in all databases match the position */
    if thor_search.allocation == 0 as libc::c_int {
        thor_search.match_list =
            safe_malloc((thor_game_count as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut GameType>()
                                                             as
                                                             libc::c_ulong))
                as *mut *mut GameType;
        thor_search.allocation = thor_game_count
    } else if thor_search.allocation < thor_game_count {
        free(thor_search.match_list as *mut libc::c_void);
        thor_search.match_list =
            safe_malloc((thor_game_count as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut GameType>()
                                                             as
                                                             libc::c_ulong))
                as *mut *mut GameType;
        thor_search.allocation = thor_game_count
    }
    /* If necessary, filter all games in the database */
    if thor_games_filtered == 0 {
        filter_all_databases();
        thor_games_filtered = 1 as libc::c_int
    }
    /* If necessary, sort all games in the database */
    if thor_games_sorted == 0 {
        current_db = database_head;
        i = 0 as libc::c_int;
        while !current_db.is_null() {
            j = 0 as libc::c_int;
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
        j = 0 as libc::c_int;
        while j < thor_game_count {
            (**thor_search.match_list.offset(j as isize)).sort_order = j;
            j += 1
        }
        thor_games_sorted = 1 as libc::c_int
    }
    /* Determine disc count, hash codes, patterns and opening
       for the position */
    disc_count[2 as libc::c_int as usize] = 0 as libc::c_int;
    disc_count[0 as libc::c_int as usize] =
        disc_count[2 as libc::c_int as usize];
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        pos = 10 as libc::c_int * i + 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            if *in_board.offset(pos as isize) == 0 as libc::c_int {
                disc_count[0 as libc::c_int as usize] += 1
            } else if *in_board.offset(pos as isize) == 2 as libc::c_int {
                disc_count[2 as libc::c_int as usize] += 1
            }
            j += 1;
            pos += 1
        }
        i += 1
    }
    move_count =
        disc_count[0 as libc::c_int as usize] +
            disc_count[2 as libc::c_int as usize] - 4 as libc::c_int;
    compute_thor_patterns(in_board);
    compute_partial_hash(&mut target_hash1, &mut target_hash2);
    opening_scan(move_count);
    /* Determine the shape masks */
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        shape_lo[i as usize] = 0 as libc::c_int as libc::c_uint;
        shape_hi[i as usize] = 0 as libc::c_int as libc::c_uint;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        j = 0 as libc::c_int;
        pos = 10 as libc::c_int * i + 11 as libc::c_int;
        while j < 8 as libc::c_int {
            if *in_board.offset(pos as isize) != 1 as libc::c_int {
                index = 8 as libc::c_int * i + j;
                if index < 32 as libc::c_int {
                    shape_lo[0 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index) as libc::c_uint
                } else {
                    shape_hi[0 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index - 32 as libc::c_int) as
                            libc::c_uint
                }
                index = 8 as libc::c_int * i + (7 as libc::c_int - j);
                if index < 32 as libc::c_int {
                    shape_lo[1 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index) as libc::c_uint
                } else {
                    shape_hi[1 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index - 32 as libc::c_int) as
                            libc::c_uint
                }
                index = 8 as libc::c_int * j + i;
                if index < 32 as libc::c_int {
                    shape_lo[2 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index) as libc::c_uint
                } else {
                    shape_hi[2 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index - 32 as libc::c_int) as
                            libc::c_uint
                }
                index = 8 as libc::c_int * j + (7 as libc::c_int - i);
                if index < 32 as libc::c_int {
                    shape_lo[3 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index) as libc::c_uint
                } else {
                    shape_hi[3 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index - 32 as libc::c_int) as
                            libc::c_uint
                }
                index = 8 as libc::c_int * (7 as libc::c_int - i) + j;
                if index < 32 as libc::c_int {
                    shape_lo[4 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index) as libc::c_uint
                } else {
                    shape_hi[4 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index - 32 as libc::c_int) as
                            libc::c_uint
                }
                index =
                    8 as libc::c_int * (7 as libc::c_int - i) +
                        (7 as libc::c_int - j);
                if index < 32 as libc::c_int {
                    shape_lo[5 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index) as libc::c_uint
                } else {
                    shape_hi[5 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index - 32 as libc::c_int) as
                            libc::c_uint
                }
                index = 8 as libc::c_int * (7 as libc::c_int - j) + i;
                if index < 32 as libc::c_int {
                    shape_lo[6 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index) as libc::c_uint
                } else {
                    shape_hi[6 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index - 32 as libc::c_int) as
                            libc::c_uint
                }
                index =
                    8 as libc::c_int * (7 as libc::c_int - j) +
                        (7 as libc::c_int - i);
                if index < 32 as libc::c_int {
                    shape_lo[7 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index) as libc::c_uint
                } else {
                    shape_hi[7 as libc::c_int as usize] |=
                        ((1 as libc::c_int) << index - 32 as libc::c_int) as
                            libc::c_uint
                }
            }
            j += 1;
            pos += 1
        }
        i += 1
    }
    /* Get the corner mask */
    corner_mask =
        get_corner_mask(*in_board.offset(11 as libc::c_int as isize),
                        *in_board.offset(81 as libc::c_int as isize),
                        *in_board.offset(18 as libc::c_int as isize),
                        *in_board.offset(88 as libc::c_int as isize));
    /* Query the database about all positions in all databases.
       Only games which pass the currently applied filter are scanned.
       Also compute the frequency table and the next move table.
       To speed up sorting the games, the match table is first filled
       with NULLs and when a matching game is found, a pointer to it is
       inserted at a position determined by the field SORT_ORDER
       in the game. As this index is unique, no over-write
       can occur. */
    thor_search.match_count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < thor_game_count {
        let ref mut fresh6 = *thor_search.match_list.offset(i as isize);
        *fresh6 = 0 as *mut GameType;
        i += 1
    }
    i = 0 as libc::c_int;
    while i <= 64 as libc::c_int {
        frequency[i as usize] = 0 as libc::c_int;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        thor_search.next_move_frequency[i as usize] = 0 as libc::c_int;
        thor_search.next_move_score[i as usize] = 0.0f64;
        i += 1
    }
    current_db = database_head;
    while !current_db.is_null() {
        i = 0 as libc::c_int;
        while i < (*current_db).count {
            game =
                &mut *(*current_db).games.offset(i as isize) as *mut GameType;
            if (*game).passes_filter != 0 {
                if disc_count[0 as libc::c_int as usize] ==
                       (*game).black_disc_count[move_count as usize] as
                           libc::c_int {
                    if position_match(game, move_count, side_to_move,
                                      shape_lo.as_mut_ptr(),
                                      shape_hi.as_mut_ptr(), corner_mask,
                                      target_hash1, target_hash2) != 0 {
                        let ref mut fresh7 =
                            *thor_search.match_list.offset((*game).sort_order
                                                               as isize);
                        *fresh7 = game;
                        symmetry = (*game).matching_symmetry as libc::c_int;
                        if move_count < (*game).move_count as libc::c_int {
                            next_move =
                                *symmetry_map[symmetry as
                                                  usize].offset(abs((*game).moves[move_count
                                                                                      as
                                                                                      usize]
                                                                        as
                                                                        libc::c_int)
                                                                    as isize);
                            thor_search.next_move_frequency[next_move as
                                                                usize] += 1;
                            if (*game).actual_black_score as libc::c_int ==
                                   32 as libc::c_int {
                                thor_search.next_move_score[next_move as
                                                                usize] +=
                                    0.5f64
                            } else if (*game).actual_black_score as
                                          libc::c_int > 32 as libc::c_int {
                                if side_to_move == 0 as libc::c_int {
                                    thor_search.next_move_score[next_move as
                                                                    usize] +=
                                        1.0f64
                                }
                            } else if side_to_move == 2 as libc::c_int {
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
    if thor_search.match_count > 0 as libc::c_int &&
           thor_search.match_count < thor_game_count {
        i = 0 as libc::c_int;
        j = 0 as libc::c_int;
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
    sum = 0 as libc::c_int;
    i = 0 as libc::c_int;
    thor_search.white_wins = 0 as libc::c_int;
    while i <= 31 as libc::c_int {
        thor_search.white_wins += frequency[i as usize];
        sum += i * frequency[i as usize];
        i += 1
    }
    thor_search.draws = frequency[32 as libc::c_int as usize];
    sum += 32 as libc::c_int * frequency[32 as libc::c_int as usize];
    i = 33 as libc::c_int;
    thor_search.black_wins = 0 as libc::c_int;
    while i <= 64 as libc::c_int {
        thor_search.black_wins += frequency[i as usize];
        sum += i * frequency[i as usize];
        i += 1
    }
    if thor_search.match_count == 0 as libc::c_int {
        /* Average of 0 values is pointless */
        thor_search.average_black_score = 32.0f64
    } else {
        thor_search.average_black_score =
            sum as libc::c_double / thor_search.match_count as libc::c_double
    }
    /* Determine the median score */
    if thor_search.match_count == 0 as libc::c_int {
        /* ...and so is median of 0 values */
        thor_search.median_black_score = 32 as libc::c_int
    } else {
        cumulative[0 as libc::c_int as usize] =
            frequency[0 as libc::c_int as usize];
        i = 1 as libc::c_int;
        while i <= 64 as libc::c_int {
            cumulative[i as usize] =
                cumulative[(i - 1 as libc::c_int) as usize] +
                    frequency[i as usize];
            i += 1
        }
        /* Median is average between first value for which cumulative
           frequency reaches 50% and first value for which it is
           strictly larger than 50%. This definition works regardless
           of the parity of the number of values.
           By definition of median, both loops terminate for indices <= 64. */
        i = 0 as libc::c_int;
        while 2 as libc::c_int * cumulative[i as usize] <
                  thor_search.match_count {
            i += 1
        }
        j = i;
        while 2 as libc::c_int * cumulative[j as usize] <
                  thor_search.match_count + 1 as libc::c_int {
            j += 1
        }
        thor_search.median_black_score = (i + j) / 2 as libc::c_int
    };
}
/*
  GET_THOR_GAME
  Returns all available information about the INDEXth game
  in the list of matching games generated by DATABASE_SEARCH.
*/
#[no_mangle]
pub unsafe extern "C" fn get_thor_game(mut index: libc::c_int)
 -> GameInfoType {
    let mut info =
        GameInfoType{black_name: 0 as *const libc::c_char,
                     white_name: 0 as *const libc::c_char,
                     tournament: 0 as *const libc::c_char,
                     year: 0,
                     black_actual_score: 0,
                     black_corrected_score: 0,};
    let mut game = 0 as *mut GameType;
    if index < 0 as libc::c_int || index >= thor_search.match_count {
        /* Bad index, so fill with empty values */
        info.black_name = b"\x00" as *const u8 as *const libc::c_char;
        info.white_name = b"\x00" as *const u8 as *const libc::c_char;
        info.tournament = b"\x00" as *const u8 as *const libc::c_char;
        info.year = 0 as libc::c_int;
        info.black_actual_score = 32 as libc::c_int;
        info.black_corrected_score = 32 as libc::c_int
    } else {
        /* Copy name fields etc */
        game = *thor_search.match_list.offset(index as isize);
        info.black_name = get_player_name((*game).black_no as libc::c_int);
        info.white_name = get_player_name((*game).white_no as libc::c_int);
        info.tournament =
            tournament_name((*game).tournament_no as libc::c_int);
        info.year = (*(*game).database).prolog.origin_year;
        info.black_actual_score = (*game).actual_black_score as libc::c_int;
        info.black_corrected_score =
            (*game).perfect_black_score as libc::c_int
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
#[no_mangle]
pub unsafe extern "C" fn get_thor_game_moves(mut index: libc::c_int,
                                             mut move_count: *mut libc::c_int,
                                             mut moves: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut game = 0 as *mut GameType;
    if index < 0 as libc::c_int || index >= thor_search.match_count {
        /* Bad index, so fill with empty values */
        *move_count = 0 as libc::c_int
    } else {
        game = *thor_search.match_list.offset(index as isize);
        *move_count = (*game).move_count as libc::c_int;
        match (*game).matching_symmetry as libc::c_int {
            0 | 2 | 5 | 7 => {
                /* Symmetries that preserve the initial position. */
                i = 0 as libc::c_int;
                while i < (*game).move_count as libc::c_int {
                    *moves.offset(i as isize) =
                        *symmetry_map[(*game).matching_symmetry as
                                          usize].offset(abs((*game).moves[i as
                                                                              usize]
                                                                as
                                                                libc::c_int)
                                                            as isize);
                    i += 1
                }
            }
            _ => {
                /* Symmetries that reverse the initial position. */
                i = 0 as libc::c_int;
                while i < (*game).move_count as libc::c_int {
                    *moves.offset(i as isize) =
                        abs((*game).moves[i as usize] as libc::c_int);
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
#[no_mangle]
pub unsafe extern "C" fn get_thor_game_move_count(mut index: libc::c_int)
 -> libc::c_int {
    if index < 0 as libc::c_int || index >= thor_search.match_count {
        /* Bad index */
        return -(1 as libc::c_int)
    } else {
        return (**thor_search.match_list.offset(index as isize)).move_count as
                   libc::c_int
    };
}
/*
  GET_THOR_GAME_MOVE
  Returns the MOVE_NUMBERth move in the INDEXth game
  in the list of matching games generated by DATABASE_SEARCH.
*/
#[no_mangle]
pub unsafe extern "C" fn get_thor_game_move(mut index: libc::c_int,
                                            mut move_number: libc::c_int)
 -> libc::c_int {
    if index < 0 as libc::c_int || index >= thor_search.match_count {
        return -(1 as libc::c_int)
    } else {
        let mut game = *thor_search.match_list.offset(index as isize);
        if move_number < 0 as libc::c_int ||
               move_number >= (*game).move_count as libc::c_int {
            return -(1 as libc::c_int)
        } else {
            return *symmetry_map[(*game).matching_symmetry as
                                     usize].offset(abs((*game).moves[move_number
                                                                         as
                                                                         usize]
                                                           as libc::c_int) as
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
#[no_mangle]
pub unsafe extern "C" fn get_total_game_count() -> libc::c_int {
    return thor_game_count;
}
#[no_mangle]
pub unsafe extern "C" fn get_match_count() -> libc::c_int {
    return thor_search.match_count;
}
#[no_mangle]
pub unsafe extern "C" fn get_black_win_count() -> libc::c_int {
    return thor_search.black_wins;
}
#[no_mangle]
pub unsafe extern "C" fn get_draw_count() -> libc::c_int {
    return thor_search.draws;
}
#[no_mangle]
pub unsafe extern "C" fn get_white_win_count() -> libc::c_int {
    return thor_search.white_wins;
}
#[no_mangle]
pub unsafe extern "C" fn get_black_median_score() -> libc::c_int {
    return thor_search.median_black_score;
}
#[no_mangle]
pub unsafe extern "C" fn get_black_average_score() -> libc::c_double {
    return thor_search.average_black_score;
}
#[no_mangle]
pub unsafe extern "C" fn get_move_frequency(mut move_0: libc::c_int)
 -> libc::c_int {
    return thor_search.next_move_frequency[move_0 as usize];
}
#[no_mangle]
pub unsafe extern "C" fn get_move_win_rate(mut move_0: libc::c_int)
 -> libc::c_double {
    if thor_search.next_move_frequency[move_0 as usize] == 0 as libc::c_int {
        return 0.0f64
    } else {
        return thor_search.next_move_score[move_0 as usize] /
                   thor_search.next_move_frequency[move_0 as usize] as
                       libc::c_double
    };
}
/*
  PRINT_THOR_MATCHES
  Outputs the MAX_GAMES first games found by the latest
  database search to STREAM.
*/
#[no_mangle]
pub unsafe extern "C" fn print_thor_matches(mut stream: *mut FILE,
                                            mut max_games: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <
              (if thor_search.match_count < max_games {
                   thor_search.match_count
               } else { max_games }) {
        if i == 0 as libc::c_int {
            fputs(b"\n\x00" as *const u8 as *const libc::c_char, stream);
        }
        print_game(stream, *thor_search.match_list.offset(i as isize),
                   0 as libc::c_int);
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
unsafe extern "C" fn init_thor_hash() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut row: [libc::c_int; 10] = [0; 10];
    let mut flip_row: [libc::c_int; 6561] = [0; 6561];
    let mut buffer: [libc::c_int; 6561] = [0; 6561];
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int { row[i as usize] = 0 as libc::c_int; i += 1 }
    i = 0 as libc::c_int;
    while i < 6561 as libc::c_int {
        flip_row[i as usize] = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            flip_row[i as usize] +=
                row[j as usize] * pow3[(7 as libc::c_int - j) as usize];
            j += 1
        }
        /* Next configuration */
        j = 0 as libc::c_int;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as libc::c_int {
                row[j as usize] = 0 as libc::c_int
            }
            j += 1;
            if !(row[(j - 1 as libc::c_int) as usize] == 0 as libc::c_int &&
                     j < 8 as libc::c_int) {
                break ;
            }
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 6561 as libc::c_int {
            buffer[j as usize] = abs(my_random() as libc::c_int);
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 6561 as libc::c_int {
            primary_hash[i as usize][j as usize] =
                buffer[j as usize] as libc::c_uint &
                    0xffff0000 as libc::c_uint |
                    bit_reverse_32(buffer[flip_row[j as usize] as usize] as
                                       libc::c_uint) &
                        0xffff as libc::c_int as libc::c_uint;
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 6561 as libc::c_int {
            buffer[j as usize] = abs(my_random() as libc::c_int);
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 6561 as libc::c_int {
            secondary_hash[i as usize][j as usize] =
                buffer[j as usize] as libc::c_uint &
                    0xffff0000 as libc::c_uint |
                    bit_reverse_32(buffer[flip_row[j as usize] as usize] as
                                       libc::c_uint) &
                        0xffff as libc::c_int as libc::c_uint;
            j += 1
        }
        i += 1
    };
}
/*
  INIT_MOVE_MASKS
  Initializes the shape bit masks for each of the possible moves.
*/
unsafe extern "C" fn init_move_masks() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        pos = 10 as libc::c_int * i + 11 as libc::c_int;
        while j < 8 as libc::c_int {
            index = 8 as libc::c_int * i + j;
            move_mask_lo[pos as usize] =
                ((1 as libc::c_int) << index) as libc::c_uint;
            move_mask_hi[pos as usize] = 0 as libc::c_int as libc::c_uint;
            unmove_mask_lo[pos as usize] =
                !((1 as libc::c_int) << index) as libc::c_uint;
            unmove_mask_hi[pos as usize] =
                !(0 as libc::c_int) as libc::c_uint;
            j += 1;
            pos += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        pos = 10 as libc::c_int * i + 51 as libc::c_int;
        while j < 8 as libc::c_int {
            index = 8 as libc::c_int * i + j;
            move_mask_lo[pos as usize] = 0 as libc::c_int as libc::c_uint;
            move_mask_hi[pos as usize] =
                ((1 as libc::c_int) << index) as libc::c_uint;
            unmove_mask_lo[pos as usize] =
                !(0 as libc::c_int) as libc::c_uint;
            unmove_mask_hi[pos as usize] =
                !((1 as libc::c_int) << index) as libc::c_uint;
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
unsafe extern "C" fn init_symmetry_maps() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            pos = 10 as libc::c_int * i + j;
            b1_b1_map[pos as usize] = pos;
            g1_b1_map[pos as usize] =
                10 as libc::c_int * i + (9 as libc::c_int - j);
            g8_b1_map[pos as usize] =
                10 as libc::c_int * (9 as libc::c_int - i) +
                    (9 as libc::c_int - j);
            b8_b1_map[pos as usize] =
                10 as libc::c_int * (9 as libc::c_int - i) + j;
            a2_b1_map[pos as usize] = 10 as libc::c_int * j + i;
            a7_b1_map[pos as usize] =
                10 as libc::c_int * j + (9 as libc::c_int - i);
            h7_b1_map[pos as usize] =
                10 as libc::c_int * (9 as libc::c_int - j) +
                    (9 as libc::c_int - i);
            h2_b1_map[pos as usize] =
                10 as libc::c_int * (9 as libc::c_int - j) + i;
            j += 1
        }
        i += 1
    }
    symmetry_map[0 as libc::c_int as usize] = b1_b1_map.as_mut_ptr();
    inv_symmetry_map[0 as libc::c_int as usize] = b1_b1_map.as_mut_ptr();
    symmetry_map[1 as libc::c_int as usize] = b8_b1_map.as_mut_ptr();
    inv_symmetry_map[1 as libc::c_int as usize] = b8_b1_map.as_mut_ptr();
    symmetry_map[2 as libc::c_int as usize] = a2_b1_map.as_mut_ptr();
    inv_symmetry_map[2 as libc::c_int as usize] = a2_b1_map.as_mut_ptr();
    symmetry_map[3 as libc::c_int as usize] = h2_b1_map.as_mut_ptr();
    inv_symmetry_map[3 as libc::c_int as usize] = a7_b1_map.as_mut_ptr();
    symmetry_map[4 as libc::c_int as usize] = g1_b1_map.as_mut_ptr();
    inv_symmetry_map[4 as libc::c_int as usize] = g1_b1_map.as_mut_ptr();
    symmetry_map[5 as libc::c_int as usize] = g8_b1_map.as_mut_ptr();
    inv_symmetry_map[5 as libc::c_int as usize] = g8_b1_map.as_mut_ptr();
    symmetry_map[6 as libc::c_int as usize] = a7_b1_map.as_mut_ptr();
    inv_symmetry_map[6 as libc::c_int as usize] = h2_b1_map.as_mut_ptr();
    symmetry_map[7 as libc::c_int as usize] = h7_b1_map.as_mut_ptr();
    inv_symmetry_map[7 as libc::c_int as usize] = h7_b1_map.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            k = 1 as libc::c_int;
            while k <= 8 as libc::c_int {
                pos = 10 as libc::c_int * j + k;
                if *inv_symmetry_map[i as
                                         usize].offset(*symmetry_map[i as
                                                                         usize].offset(pos
                                                                                           as
                                                                                           isize)
                                                           as isize) != pos {
                    fatal_error(b"Error in map %d: inv(map(%d))=%d\n\x00" as
                                    *const u8 as *const libc::c_char, i, pos,
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
unsafe extern "C" fn new_thor_opening_node(mut parent: *mut ThorOpeningNode)
 -> *mut ThorOpeningNode {
    let mut node = 0 as *mut ThorOpeningNode;
    node =
        safe_malloc(::std::mem::size_of::<ThorOpeningNode>() as libc::c_ulong)
            as *mut ThorOpeningNode;
    (*node).child_move = 0 as libc::c_int as libc::c_char;
    (*node).sibling_move = 0 as libc::c_int as libc::c_char;
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
unsafe extern "C" fn calculate_opening_frequency(mut node:
                                                     *mut ThorOpeningNode)
 -> libc::c_int {
    let mut sum: libc::c_int = 0;
    let mut child = 0 as *mut ThorOpeningNode;
    child = (*node).child_node;
    if child.is_null() {
        return (*node).frequency
    } else {
        sum = 0 as libc::c_int;
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
unsafe extern "C" fn build_thor_opening_tree() {
    let mut thor_move_list: [libc::c_char; 61] = [0; 61];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut move_0: libc::c_int = 0;
    let mut branch_depth: libc::c_int = 0;
    let mut end_depth: libc::c_int = 0;
    let mut flipped: libc::c_int = 0;
    let mut hash1: libc::c_uint = 0;
    let mut hash2: libc::c_uint = 0;
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
    node_list[0 as libc::c_int as usize] = root_node;
    /* Add each of the openings to the tree */
    i = 0 as libc::c_int;
    while i < 741 as libc::c_int {
        branch_depth = thor_opening_list[i as usize].first_unique;
        end_depth =
            (branch_depth as
                 libc::c_ulong).wrapping_add(strlen(thor_opening_list[i as
                                                                          usize].move_str).wrapping_div(2
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong))
                as libc::c_int;
        j = 0 as libc::c_int;
        while j < end_depth - branch_depth {
            thor_move_list[(branch_depth + j) as usize] =
                (10 as libc::c_int *
                     (*thor_opening_list[i as
                                             usize].move_str.offset((2 as
                                                                         libc::c_int
                                                                         * j +
                                                                         1 as
                                                                             libc::c_int)
                                                                        as
                                                                        isize)
                          as libc::c_int - '0' as i32) +
                     (*thor_opening_list[i as
                                             usize].move_str.offset((2 as
                                                                         libc::c_int
                                                                         * j)
                                                                        as
                                                                        isize)
                          as libc::c_int - 'a' as i32 + 1 as libc::c_int)) as
                    libc::c_char;
            j += 1
        }
        /* Play through the moves common with the previous line
           and the first deviation */
        clear_thor_board();
        thor_side_to_move = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j <= branch_depth {
            move_0 = thor_move_list[j as usize] as libc::c_int;
            flipped =
                any_flips(move_0, thor_side_to_move,
                          0 as libc::c_int + 2 as libc::c_int -
                              thor_side_to_move);
            if flipped != 0 {
                thor_board[move_0 as usize] = thor_side_to_move;
                thor_side_to_move =
                    0 as libc::c_int + 2 as libc::c_int - thor_side_to_move
            } else {
                thor_side_to_move =
                    0 as libc::c_int + 2 as libc::c_int - thor_side_to_move;
                flipped =
                    any_flips(move_0, thor_side_to_move,
                              0 as libc::c_int + 2 as libc::c_int -
                                  thor_side_to_move);
                if flipped != 0 {
                    thor_board[move_0 as usize] = thor_side_to_move;
                    thor_side_to_move =
                        0 as libc::c_int + 2 as libc::c_int -
                            thor_side_to_move
                } else {
                    puts(b"This COULD happen (1) in BUILD_THOR_OPENING_TREE\x00"
                             as *const u8 as *const libc::c_char);
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
        node_list[(branch_depth + 1 as libc::c_int) as usize] = new_child;
        /* Play through the rest of the moves and create new nodes for each
           of the resulting positions */
        j = branch_depth + 1 as libc::c_int;
        while j < end_depth {
            move_0 = thor_move_list[j as usize] as libc::c_int;
            flipped =
                any_flips(move_0, thor_side_to_move,
                          0 as libc::c_int + 2 as libc::c_int -
                              thor_side_to_move);
            if flipped != 0 {
                thor_board[move_0 as usize] = thor_side_to_move;
                thor_side_to_move =
                    0 as libc::c_int + 2 as libc::c_int - thor_side_to_move
            } else {
                thor_side_to_move =
                    0 as libc::c_int + 2 as libc::c_int - thor_side_to_move;
                flipped =
                    any_flips(move_0, thor_side_to_move,
                              0 as libc::c_int + 2 as libc::c_int -
                                  thor_side_to_move);
                if flipped != 0 {
                    thor_board[move_0 as usize] = thor_side_to_move;
                    thor_side_to_move =
                        0 as libc::c_int + 2 as libc::c_int -
                            thor_side_to_move
                } else {
                    puts(b"This COULD happen (2) in BUILD_THOR_OPENING_TREE\x00"
                             as *const u8 as *const libc::c_char);
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
            node_list[(j + 1 as libc::c_int) as usize] = new_child;
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
#[no_mangle]
pub unsafe extern "C" fn get_thor_game_size() -> libc::c_int {
    return ::std::mem::size_of::<GameType>() as libc::c_ulong as libc::c_int;
}
/*
  INIT_THOR_DATABASE
  Performs the basic initializations of the Thor database interface.
  Before any operation on the database may be performed, this function
  must be called.
*/
#[no_mangle]
pub unsafe extern "C" fn init_thor_database() {
    let mut i: libc::c_int = 0; /* "infinity" */
    thor_game_count = 0 as libc::c_int;
    thor_database_count = 0 as libc::c_int;
    thor_search.match_list = 0 as *mut *mut GameType;
    thor_search.allocation = 0 as libc::c_int;
    thor_search.match_count = 0 as libc::c_int;
    thor_search.black_wins = 0 as libc::c_int;
    thor_search.draws = 0 as libc::c_int;
    thor_search.white_wins = 0 as libc::c_int;
    thor_search.median_black_score = 0 as libc::c_int;
    thor_search.average_black_score = 0.0f64;
    thor_sort_criteria_count = 5 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        thor_sort_order[i as usize] = default_sort_order[i as usize];
        i += 1
    }
    database_head = 0 as *mut DatabaseType;
    players.name_buffer = 0 as *mut libc::c_char;
    players.player_list = 0 as *mut PlayerType;
    players.count = 0 as libc::c_int;
    tournaments.name_buffer = 0 as *mut libc::c_char;
    tournaments.tournament_list = 0 as *mut TournamentType;
    tournaments.count = 0 as libc::c_int;
    thor_games_sorted = 0 as libc::c_int;
    thor_games_filtered = 0 as libc::c_int;
    init_move_masks();
    init_symmetry_maps();
    init_thor_hash();
    prepare_thor_board();
    build_thor_opening_tree();
    filter.game_categories =
        1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int;
    filter.player_filter = EitherSelectedFilter;
    filter.first_year = -((1 as libc::c_int) << 25 as libc::c_int);
    filter.last_year = (1 as libc::c_int) << 25 as libc::c_int;
}
