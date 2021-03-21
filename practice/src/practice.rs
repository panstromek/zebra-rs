#![allow(dead_code,  non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut, unused_must_use)]

use engine::src::moves::{generate_all, make_move, unmake_move, valid_move};
use engine::src::osfbook::{find_opening_name, get_hash};
use legacy_zebra::src::display::{display_board, produce_eval_text, set_move_list, set_names, display_state};
use legacy_zebra::src::error::{LibcFatalError};
use legacy_zebra::src::game::{extended_compute_move, game_init, get_evaluated, get_evaluated_count};
use legacy_zebra::src::osfbook::{init_osf, read_binary_database};
use legacy_zebra::src::zebra::{ LibcTimeSource};
use libc_wrapper::{_IO_FILE, stdout, atoi, strcmp, scanf};
use std::ffi::{CStr, CString};
use engine::src::zebra::FullState;
use std::io::Write;

pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type Board = [i32; 128];

/*
   File:         practice.c

   Created:      January 29, 1998

   Modified:     July 12, 1999

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:     A small utility which enables the user to browse
                 an opening book file.
*/
unsafe fn main_0(args: Vec<String>) -> i32 {
    let mut book_name: &str;
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
    static src: LibcTimeSource = LibcTimeSource {};
    let mut full_state = FullState::new(&src);
    let g_state: &mut FullState = &mut full_state;
    let argc = args.len();
    if argc == 2 {
        book_name = &args[1]
    } else if argc == 1 {
        book_name = "book.bin";
    } else {
        writeln!(stdout, "Usage:\n  [practice <book file>]");
        writeln!(stdout, "\nDefault book file is book.bin\n");
        writeln!(stdout, "Commands: When prompted for a move, a legal move may");
        writeln!(stdout, "          a number of moves to take back must be entered.");
        writeln!(stdout, "To exit the program, type \'quit\'.");
        write!(stdout, "\n");
        write!(stdout, "Gunnar Andersson, {}\n", "Aug  9 2020");
        std::process::exit(1 as i32);
    }
    init_osf(1 as i32, g_state);
    let book_name = CString::new(book_name).unwrap();
    read_binary_database(book_name.as_ptr(), &mut g_state.g_book);
    game_init(0 as *const i8, &mut side_to_move, g_state);
    (g_state.game_state).toggle_human_openings(0 as i32);
    set_names("", "");
    quit = 0;
    while quit == 0 {
        let mut val0: i32 = 0;
        let mut val1: i32 = 0;
        let mut orientation: i32 = 0;
        set_move_list((g_state.board_state).score_sheet_row);
        let opening_name = find_opening_name(&mut (g_state.g_book), &(g_state.board_state).board);
        if let Some(opening_name) = opening_name {
            write!(stdout, "\nOpening: {}\n",
                   CStr::from_bytes_with_nul(opening_name).unwrap().to_str().unwrap());
        }
        let val0___ = &mut val0;
        let val1___ = &mut val1;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut (g_state.g_book), &(g_state.board_state).board);
        display_board(&mut stdout, &(g_state.board_state).board, side_to_move,
                      1 as i32, 0 as i32, 0 as i32,
                      display_state.current_row,
                      display_state.black_player, display_state.black_time, display_state.black_eval,
                      display_state.white_player, display_state.white_time, display_state.white_eval,
                      &(g_state.board_state).black_moves, &(g_state.board_state).white_moves
        );
        write!(stdout, "Book hash: {} {} ({})\n\n", val0, val1, orientation);
        extended_compute_move::<LibcFatalError>(side_to_move, 0 as i32,
                                                1 as i32, 6 as i32,
                                                16 as i32, 18 as i32, (g_state.g_config).echo, g_state);
        write!(stdout, "Scores for the {} moves:\n", get_evaluated_count());
        i = 0;
        while i < get_evaluated_count() {
            let mut eval_str_ = produce_eval_text(&get_evaluated(i).eval, 0 as i32);
            write!(stdout, "   {}{}   {}\n",
                   char::from('a' as u8 + (get_evaluated(i).move_0 % 10) as u8 - 1) ,
                   char::from('0' as u8 + (get_evaluated(i).move_0 / 10) as u8),
                CStr::from_ptr(eval_str_.as_ptr()).to_str().unwrap());

            i += 1
        }
        write!(stdout, "\n");
        loop  {
            repeat = 0;
            if side_to_move == 0 as i32 {
                write!(stdout, "Black move: ");
            } else {
                write!(stdout, "White move: ");
            }
            stdout.flush();
            scanf(b"%s\x00" as *const u8 as *const i8, move_string.as_mut_ptr());
            if strcmp(move_string.as_mut_ptr(), b"quit\x00" as *const u8 as *const i8) == 0 {
                quit = 1
            } else {
                command = atoi(move_string.as_mut_ptr());
                if command >= 1 && command <= g_state.moves_state.disks_played {
                    i = 1;
                    while i <= command {
                        let side_to_move = old_stm[(g_state.moves_state.disks_played - 1) as usize];
                        let move_0 = move_list[(g_state.moves_state.disks_played - 1) as usize];
                        {
                            unmake_move(side_to_move, move_0, &mut (g_state.board_state).board, &mut (g_state.moves_state), &mut (g_state.hash_state), &mut (g_state.flip_stack_));
                        };
                        i += 1
                    }
                    side_to_move = old_stm[(g_state.moves_state).disks_played as usize];
                    (g_state.board_state).score_sheet_row = row[(g_state.moves_state).disks_played as usize]
                } else if command != 0 {
                    write!(stdout, "Can\'t back up {} moves\n\n", command);
                    repeat = 1 as i32
                } else {
                    generate_all(side_to_move, &mut (g_state.moves_state), &(g_state.search_state), &(g_state.board_state).board);
                    move_0 = move_string[0] as i32 - 'a' as i32 + 1 + 10 * (move_string[1] as i32 - '0' as i32);
                    if move_string[0] as i32 >= 'a' as i32 &&
                        move_string[0] as i32 <= 'h' as i32 &&
                        move_string[1] as i32 >= '1' as i32 &&
                        move_string[1] as i32 <= '8' as i32 &&
                        valid_move(move_0, side_to_move, &g_state.board_state.board) != 0 {
                        old_stm[g_state.moves_state.disks_played as usize] = side_to_move;
                        row[g_state.moves_state.disks_played as usize] = g_state.board_state.score_sheet_row;
                        move_list[g_state.moves_state.disks_played as usize] = move_0;
                        make_move(side_to_move, move_0, 1 as i32,
                                  &mut g_state.moves_state,
                                  &mut g_state.board_state,
                                  &mut g_state.hash_state,
                                  &mut g_state.flip_stack_);
                        if side_to_move == 0 {
                            g_state.board_state.score_sheet_row += 1;
                            g_state.board_state.black_moves[g_state.board_state.score_sheet_row as usize] = move_0
                        } else {
                            g_state.board_state.white_moves[g_state.board_state.score_sheet_row as usize] = move_0
                        }
                        side_to_move = 2 - side_to_move
                    } else {
                        write!(stdout, "Move infeasible\n\n");
                        repeat = 1
                    }
                }
            }
            if !(repeat != 0) { break; }
        }
    }
    return 0;
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0(::std::env::args().collect()) as i32)
    }
}
