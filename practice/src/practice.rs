#![allow(dead_code,  non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]

use engine::src::globals::{score_sheet_row, white_moves, black_moves, board};
use engine::src::moves::{make_move, disks_played, valid_move, generate_all, unmake_move};
use engine::src::game::{get_evaluated, get_evaluated_count, toggle_human_openings};
use engine::src::osfbook::{get_hash, find_opening_name};
use engine::src::display::{set_move_list, set_names};
use c2rust_out::src::display::{produce_eval_text, display_board};
use c2rust_out::src::game::{extended_compute_move, game_init};
use c2rust_out::src::osfbook::{read_binary_database, init_osf};
use c2rust_out::src::zebra::_IO_FILE;
use c2rust_out::src::error::{LibcFatalError, FE};
use engine::src::error::FrontEnd;

extern "C" {

    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type Board = [libc::c_int; 128];
pub type EvalType = libc::c_uint;
pub const UNINITIALIZED_EVAL: EvalType = 8;
pub const INTERRUPTED_EVAL: EvalType = 7;
pub const UNDEFINED_EVAL: EvalType = 6;
pub const PASS_EVAL: EvalType = 5;
pub const FORCED_EVAL: EvalType = 4;
pub const SELECTIVE_EVAL: EvalType = 3;
pub const WLD_EVAL: EvalType = 2;
pub const EXACT_EVAL: EvalType = 1;
pub const MIDGAME_EVAL: EvalType = 0;
pub type EvalResult = libc::c_uint;
pub const UNSOLVED_POSITION: EvalResult = 3;
pub const LOST_POSITION: EvalResult = 2;
pub const DRAWN_POSITION: EvalResult = 1;
pub const WON_POSITION: EvalResult = 0;

#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int) as libc::c_int;
}
/*
   File:         practice.c

   Created:      January 29, 1998

   Modified:     July 12, 1999

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:     A small utility which enables the user to browse
                 an opening book file.
*/
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
                 -> libc::c_int {
    let mut book_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opening_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut move_string: [libc::c_char; 10] = [0; 10];
    let mut i: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut quit: libc::c_int = 0;
    let mut repeat: libc::c_int = 0;
    let mut command: libc::c_int = 0;
    let mut move_0: libc::c_int = 0;
    let mut old_stm: [libc::c_int; 61] = [0; 61];
    let mut move_list: [libc::c_int; 61] = [0; 61];
    let mut row: [libc::c_int; 61] = [0; 61];
    if argc == 2 as libc::c_int {
        book_name = *argv.offset(1 as libc::c_int as isize)
    } else if argc == 1 as libc::c_int {
        book_name =
            strdup(b"book.bin\x00" as *const u8 as *const libc::c_char)
    } else {
        puts(b"Usage:\n  [practice <book file>]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"\nDefault book file is book.bin\n\x00" as *const u8 as
            *const libc::c_char);
        puts(b"Commands: When prompted for a move, a legal move may\x00" as
            *const u8 as *const libc::c_char);
        puts(b"          a number of moves to take back must be entered.\x00"
            as *const u8 as *const libc::c_char);
        puts(b"To exit the program, type \'quit\'.\x00" as *const u8 as
            *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        printf(b"Gunnar Andersson, %s\n\x00" as *const u8 as
                   *const libc::c_char,
               b"Aug  9 2020\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    init_osf(1 as libc::c_int);
    read_binary_database(book_name);
    game_init(0 as *const libc::c_char, &mut side_to_move);
    toggle_human_openings(0 as libc::c_int);
    set_names::<LibcFatalError>(b"\x00" as *const u8 as *const libc::c_char,
              b"\x00" as *const u8 as *const libc::c_char);
    quit = 0 as libc::c_int;
    while quit == 0 {
        let mut val0: libc::c_int = 0;
        let mut val1: libc::c_int = 0;
        let mut orientation: libc::c_int = 0;
        set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                      score_sheet_row);
        opening_name = find_opening_name();
        if !opening_name.is_null() {
            printf(b"\nOpening: %s\n\x00" as *const u8 as *const libc::c_char,
                   opening_name);
        }
        get_hash(&mut val0, &mut val1, &mut orientation);
        display_board(stdout, board.as_mut_ptr(), side_to_move,
                      1 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
        printf(b"Book hash: %d %d (%d)\n\n\x00" as *const u8 as
                   *const libc::c_char, val0, val1, orientation);
        extended_compute_move::<LibcFatalError>(side_to_move, 0 as libc::c_int,
                              1 as libc::c_int, 6 as libc::c_int,
                              16 as libc::c_int, 18 as libc::c_int);
        printf(b"Scores for the %d moves:\n\x00" as *const u8 as
                   *const libc::c_char, get_evaluated_count());
        i = 0 as libc::c_int;
        while i < get_evaluated_count() {
            buffer =
                produce_eval_text(&get_evaluated(i).eval, 0 as libc::c_int);
            printf(b"   %c%c   %s\n\x00" as *const u8 as *const libc::c_char,
                   'a' as i32 + get_evaluated(i).move_0 % 10 as libc::c_int -
                       1 as libc::c_int,
                   '0' as i32 + get_evaluated(i).move_0 / 10 as libc::c_int,
                   buffer);
            FE::free(buffer as *mut libc::c_void);
            i += 1
        }
        puts(b"\x00" as *const u8 as *const libc::c_char);
        loop  {
            repeat = 0 as libc::c_int;
            if side_to_move == 0 as libc::c_int {
                printf(b"Black move: \x00" as *const u8 as
                    *const libc::c_char);
            } else {
                printf(b"White move: \x00" as *const u8 as
                    *const libc::c_char);
            }
            fflush(stdout);
            scanf(b"%s\x00" as *const u8 as *const libc::c_char,
                  move_string.as_mut_ptr());
            if strcmp(move_string.as_mut_ptr(),
                      b"quit\x00" as *const u8 as *const libc::c_char) == 0 {
                quit = 1 as libc::c_int
            } else {
                command = atoi(move_string.as_mut_ptr());
                if command >= 1 as libc::c_int && command <= disks_played {
                    i = 1 as libc::c_int;
                    while i <= command {
                        unmake_move(old_stm[(disks_played - 1 as libc::c_int)
                            as usize],
                                    move_list[(disks_played -
                                        1 as libc::c_int) as
                                        usize]);
                        i += 1
                    }
                    side_to_move = old_stm[disks_played as usize];
                    score_sheet_row = row[disks_played as usize]
                } else if command != 0 as libc::c_int {
                    printf(b"Can\'t back up %d moves\n\n\x00" as *const u8 as
                               *const libc::c_char, command);
                    repeat = 1 as libc::c_int
                } else {
                    generate_all(side_to_move);
                    move_0 =
                        move_string[0 as libc::c_int as usize] as libc::c_int
                            - 'a' as i32 + 1 as libc::c_int +
                            10 as libc::c_int *
                                (move_string[1 as libc::c_int as usize] as
                                    libc::c_int - '0' as i32);
                    if move_string[0 as libc::c_int as usize] as libc::c_int
                        >= 'a' as i32 &&
                        move_string[0 as libc::c_int as usize] as
                            libc::c_int <= 'h' as i32 &&
                        move_string[1 as libc::c_int as usize] as
                            libc::c_int >= '1' as i32 &&
                        move_string[1 as libc::c_int as usize] as
                            libc::c_int <= '8' as i32 &&
                        valid_move(move_0, side_to_move) != 0 {
                        old_stm[disks_played as usize] = side_to_move;
                        row[disks_played as usize] = score_sheet_row;
                        move_list[disks_played as usize] = move_0;
                        make_move(side_to_move, move_0, 1 as libc::c_int);
                        if side_to_move == 0 as libc::c_int {
                            score_sheet_row += 1;
                            black_moves[score_sheet_row as usize] = move_0
                        } else {
                            white_moves[score_sheet_row as usize] = move_0
                        }
                        side_to_move =
                            0 as libc::c_int + 2 as libc::c_int - side_to_move
                    } else {
                        puts(b"Move infeasible\n\x00" as *const u8 as
                            *const libc::c_char);
                        repeat = 1 as libc::c_int
                    }
                }
            }
            if !(repeat != 0) { break ; }
        }
    }
    return 0 as libc::c_int;
}

pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
