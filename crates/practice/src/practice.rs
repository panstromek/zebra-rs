#![allow(unused_must_use)]

use engine::src::moves::{generate_all, make_move, unmake_move, valid_move};
use engine::src::osfbook::{find_opening_name, get_hash};
use legacy_zebra::src::display::{produce_eval_text, display_state, TO_SQUARE};
use legacy_zebra::src::error::{LibcFatalError};
use legacy_zebra::src::game::{extended_compute_move, game_init};
use legacy_zebra::src::osfbook::{init_osf, read_binary_database};
use legacy_zebra::src::zebra::{LibcTimeSource, Atoi};
use libc_wrapper::{stdout, scanf};
use std::ffi::{CStr, CString};
use engine::src::zebra::FullState;
use std::io::Write;
use legacy_zebra::src::thordb::LegacyThor;
/*
   File:         practice.c

   Created:      January 29, 1998

   Modified:     July 12, 1999

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:     A small utility which enables the user to browse
                 an opening book file.
*/
unsafe fn main_0(args: Vec<String>) -> i32 {
    let book_name: &str;
    let mut i;
    let mut side_to_move: i32 = 0;
    let mut quit;
    let mut repeat;
    let mut command;
    let mut move_0: i8;
    let mut old_stm: [i32; 61] = [0; 61];
    let mut move_list: [i8; 61] = [0; 61];
    let mut row: [i32; 61] = [0; 61];
    static SRC: LibcTimeSource = LibcTimeSource {};
    let mut full_state = FullState::new(&SRC);
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
        std::process::exit(1);
    }
    init_osf(1, g_state);
    let book_name = CString::new(book_name).unwrap();
    read_binary_database(book_name.as_ptr(), &mut g_state.g_book);
    game_init(&mut side_to_move, g_state);
    (g_state.game).toggle_human_openings(0);
    {
        let mut ds = display_state.lock().unwrap();
        ds.set_names("", "");
    }
    quit = 0;
    let mut thor = LegacyThor::new();
    while quit == 0 {
        let mut val0: i32 = 0;
        let mut val1: i32 = 0;
        let mut orientation: i32 = 0;
        {
            let mut ds = display_state.lock().unwrap();
            ds.set_move_list((g_state.board).score_sheet_row);
        }
        let opening_name = find_opening_name(&mut (g_state.g_book), &(g_state.board).board);
        if let Some(opening_name) = opening_name {
            write!(stdout, "\nOpening: {}\n",
                   CStr::from_bytes_with_nul(opening_name).unwrap().to_str().unwrap());
        }
        let val0___ = &mut val0;
        let val1___ = &mut val1;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut (g_state.g_book), &(g_state.board).board);
        {
            let mut ds = display_state.lock().unwrap();
            ds.display_board(&mut stdout, &(g_state.board).board, side_to_move,
                             1, 0, 0,
                             &(g_state.board).black_moves, &(g_state.board).white_moves
            );
        }
        write!(stdout, "Book hash: {} {} ({})\n\n", val0, val1, orientation);
        let evaluated_list = extended_compute_move::<LibcFatalError>(side_to_move, 0,
                                                                     1, 6,
                                                                     16, 18, (g_state.config).echo, g_state, &mut thor);
        write!(stdout, "Scores for the {} moves:\n", evaluated_list.get_evaluated_count());
        i = 0;
        while i < evaluated_list.get_evaluated_count() {
            let eval_str_ = produce_eval_text(&evaluated_list.get_evaluated(i).eval, 0);
            write!(stdout, "   {}   {}\n", TO_SQUARE(evaluated_list.get_evaluated(i).move_0), eval_str_);

            i += 1
        }
        write!(stdout, "\n");
        loop  {
            repeat = 0;
            if side_to_move == 0 {
                write!(stdout, "Black move: ");
            } else {
                write!(stdout, "White move: ");
            }
            stdout.flush();
            let mut move_string: [u8; 10] = [0; 10];
            scanf(b"%s\x00" as *const u8 as *const i8, move_string.as_mut_ptr());
            if move_string.split(|&byte| byte == b'\x00').next().map_or(false, |s| s == b"quit") {
                quit = 1
            } else {
                command = move_string.atoi();

                if command >= 1 && command <= g_state.moves.disks_played {
                    i = 1;
                    while i <= command {
                        unmake_move(old_stm[(g_state.moves.disks_played - 1) as usize],
                                    move_list[(g_state.moves.disks_played - 1) as usize],
                                    &mut (g_state.board).board, &mut (g_state.moves),
                                    &mut (g_state.hash), &mut (g_state.flip_stack));
                        i += 1
                    }
                    side_to_move = old_stm[(g_state.moves).disks_played as usize];
                    (g_state.board).score_sheet_row = row[(g_state.moves).disks_played as usize]
                } else if command != 0 {
                    write!(stdout, "Can\'t back up {} moves\n\n", command);
                    repeat = 1
                } else {
                    generate_all(side_to_move, &mut (g_state.moves), &(g_state.search), &(g_state.board).board);
                    move_0 = (move_string[0] as i32 - 'a' as i32 + 1 + 10 * (move_string[1] as i32 - '0' as i32)) as i8;
                    if move_string[0] as i32 >= 'a' as i32 &&
                        move_string[0] as i32 <= 'h' as i32 &&
                        move_string[1] as i32 >= '1' as i32 &&
                        move_string[1] as i32 <= '8' as i32 &&
                        valid_move(move_0, side_to_move, &g_state.board.board) != 0 {
                        old_stm[g_state.moves.disks_played as usize] = side_to_move;
                        row[g_state.moves.disks_played as usize] = g_state.board.score_sheet_row;
                        move_list[g_state.moves.disks_played as usize] = move_0;
                        make_move(side_to_move, move_0, 1,
                                  &mut g_state.moves,
                                  &mut g_state.board,
                                  &mut g_state.hash,
                                  &mut g_state.flip_stack);
                        if side_to_move == 0 {
                            g_state.board.score_sheet_row += 1;
                            g_state.board.black_moves[g_state.board.score_sheet_row as usize] = move_0
                        } else {
                            g_state.board.white_moves[g_state.board.score_sheet_row as usize] = move_0
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
