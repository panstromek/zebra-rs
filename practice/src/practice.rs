#![allow(dead_code,  non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]

use engine::src::globals::{score_sheet_row, white_moves, black_moves, board};
use engine::src::moves::{make_move, disks_played, valid_move, generate_all, unmake_move};
use engine::src::game::{get_evaluated, get_evaluated_count, toggle_human_openings};
use engine::src::osfbook::{get_hash, find_opening_name};
use legacy_zebra::src::display::{produce_eval_text, display_board, white_eval, white_time, white_player, black_eval, black_time, black_player, current_row, set_move_list, set_names};
use legacy_zebra::src::game::{extended_compute_move, game_init};
use legacy_zebra::src::osfbook::{read_binary_database, init_osf};
use legacy_zebra::src::error::{LibcFatalError, FE};
use libc_wrapper::_IO_FILE;
use engine::src::zebra::g_config;

extern "C" {

    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn printf(_: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn scanf(_: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn puts(__s: *const i8) -> i32;
    #[no_mangle]
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8,
              __base: i32) -> i64;
    #[no_mangle]
    fn free(__ptr: *mut ::std::ffi::c_void);
    #[no_mangle]
    fn exit(_: i32) -> !;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strdup(_: *const i8) -> *mut i8;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type Board = [i32; 128];


#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut ::std::ffi::c_void as *mut *mut i8,
                  10 as i32) as i32;
}
/*
   File:         practice.c

   Created:      January 29, 1998

   Modified:     July 12, 1999

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:     A small utility which enables the user to browse
                 an opening book file.
*/
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8)
                 -> i32 {
    let mut book_name: *mut i8 = 0 as *mut i8;
    let mut buffer: *mut i8 = 0 as *mut i8;
    let mut opening_name: *const i8 = 0 as *const i8;
    let mut move_string: [i8; 10] = [0; 10];
    let mut i: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut quit: i32 = 0;
    let mut repeat: i32 = 0;
    let mut command: i32 = 0;
    let mut move_0: i32 = 0;
    let mut old_stm: [i32; 61] = [0; 61];
    let mut move_list: [i32; 61] = [0; 61];
    let mut row: [i32; 61] = [0; 61];
    if argc == 2 as i32 {
        book_name = *argv.offset(1)
    } else if argc == 1 as i32 {
        book_name =
            strdup(b"book.bin\x00" as *const u8 as *const i8)
    } else {
        puts(b"Usage:\n  [practice <book file>]\x00" as *const u8 as
            *const i8);
        puts(b"\nDefault book file is book.bin\n\x00" as *const u8 as
            *const i8);
        puts(b"Commands: When prompted for a move, a legal move may\x00" as
            *const u8 as *const i8);
        puts(b"          a number of moves to take back must be entered.\x00"
            as *const u8 as *const i8);
        puts(b"To exit the program, type \'quit\'.\x00" as *const u8 as
            *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        printf(b"Gunnar Andersson, %s\n\x00" as *const u8 as
                   *const i8,
               b"Aug  9 2020\x00" as *const u8 as *const i8);
        exit(1 as i32);
    }
    init_osf(1 as i32);
    read_binary_database(book_name);
    game_init(0 as *const i8, &mut side_to_move);
    toggle_human_openings(0 as i32);
    set_names(b"\x00" as *const u8 as *const i8,
              b"\x00" as *const u8 as *const i8);
    quit = 0;
    while quit == 0 {
        let mut val0: i32 = 0;
        let mut val1: i32 = 0;
        let mut orientation: i32 = 0;
        set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                      score_sheet_row);
        opening_name = find_opening_name();
        if !opening_name.is_null() {
            printf(b"\nOpening: %s\n\x00" as *const u8 as *const i8,
                   opening_name);
        }
        get_hash(&mut val0, &mut val1, &mut orientation);
        display_board(stdout, &board, side_to_move,
                      1 as i32, 0 as i32, 0 as i32,
                      current_row,
                      black_player, black_time, black_eval,
                      white_player, white_time, white_eval,
                      &black_moves, &white_moves
        );
        printf(b"Book hash: %d %d (%d)\n\n\x00" as *const u8 as
                   *const i8, val0, val1, orientation);
        extended_compute_move::<LibcFatalError>(side_to_move, 0 as i32,
                              1 as i32, 6 as i32,
                              16 as i32, 18 as i32, g_config.echo);
        printf(b"Scores for the %d moves:\n\x00" as *const u8 as
                   *const i8, get_evaluated_count());
        i = 0;
        while i < get_evaluated_count() {
            buffer =
                produce_eval_text(&get_evaluated(i).eval, 0 as i32);
            printf(b"   %c%c   %s\n\x00" as *const u8 as *const i8,
                   'a' as i32 + get_evaluated(i).move_0 % 10 as i32 -
                       1 as i32,
                   '0' as i32 + get_evaluated(i).move_0 / 10 as i32,
                   buffer);
            FE::free(buffer as *mut ::std::ffi::c_void);
            i += 1
        }
        puts(b"\x00" as *const u8 as *const i8);
        loop  {
            repeat = 0;
            if side_to_move == 0 as i32 {
                printf(b"Black move: \x00" as *const u8 as
                    *const i8);
            } else {
                printf(b"White move: \x00" as *const u8 as
                    *const i8);
            }
            fflush(stdout);
            scanf(b"%s\x00" as *const u8 as *const i8,
                  move_string.as_mut_ptr());
            if strcmp(move_string.as_mut_ptr(),
                      b"quit\x00" as *const u8 as *const i8) == 0 {
                quit = 1 as i32
            } else {
                command = atoi(move_string.as_mut_ptr());
                if command >= 1 as i32 && command <= disks_played {
                    i = 1;
                    while i <= command {
                        unmake_move(old_stm[(disks_played - 1 as i32)
                            as usize],
                                    move_list[(disks_played -
                                        1 as i32) as
                                        usize]);
                        i += 1
                    }
                    side_to_move = old_stm[disks_played as usize];
                    score_sheet_row = row[disks_played as usize]
                } else if command != 0 as i32 {
                    printf(b"Can\'t back up %d moves\n\n\x00" as *const u8 as
                               *const i8, command);
                    repeat = 1 as i32
                } else {
                    generate_all(side_to_move);
                    move_0 =
                        move_string[0] as i32
                            - 'a' as i32 + 1 as i32 +
                            10 as i32 *
                                (move_string[1] as
                                    i32 - '0' as i32);
                    if move_string[0] as i32
                        >= 'a' as i32 &&
                        move_string[0] as
                            i32 <= 'h' as i32 &&
                        move_string[1] as
                            i32 >= '1' as i32 &&
                        move_string[1] as
                            i32 <= '8' as i32 &&
                        valid_move(move_0, side_to_move) != 0 {
                        old_stm[disks_played as usize] = side_to_move;
                        row[disks_played as usize] = score_sheet_row;
                        move_list[disks_played as usize] = move_0;
                        make_move(side_to_move, move_0, 1 as i32);
                        if side_to_move == 0 as i32 {
                            score_sheet_row += 1;
                            black_moves[score_sheet_row as usize] = move_0
                        } else {
                            white_moves[score_sheet_row as usize] = move_0
                        }
                        side_to_move =
                            0 as i32 + 2 as i32 - side_to_move
                    } else {
                        puts(b"Move infeasible\n\x00" as *const u8 as
                            *const i8);
                        repeat = 1 as i32
                    }
                }
            }
            if !(repeat != 0) { break ; }
        }
    }
    return 0 as i32;
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as i32,
                                    args.as_mut_ptr() as
                                        *mut *mut i8) as i32)
    }
}
