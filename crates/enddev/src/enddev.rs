#![allow(dead_code,  non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut, unused_must_use)]
#![feature(extern_types, label_break_value, register_tool)]

use engine::src::game::EvaluatedMove;
use engine::src::hash::{determine_hash_values, setup_hash};
use engine::src::moves::{generate_all, make_move, valid_move};
use engine::src::search::disc_count;
use engine::src::zebra::{EvaluationType, FullState};
use legacy_zebra::src::display::{display_state};
use legacy_zebra::src::error::{LibcFatalError};
use legacy_zebra::src::game::{legacy_compute_move, extended_compute_move, game_init, global_setup};
use legacy_zebra::src::learn::init_learn;
use libc_wrapper::{_IO_FILE, stdout, fprintf, fputs, printf, feof, fopen, sscanf, tolower, strlen, __ctype_b_loc, fgets, stderr, FileHandle, exit};
use legacy_zebra::src::zebra::LibcTimeSource;
use std::io::Write;

pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub type FILE = _IO_FILE;
/*
   File:           globals.h

   Created:        June 30, 1997

   Modified:       January 8, 2000

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Global state variables.
*/
/* The basic board type. One index for each position;
   a1=11, h1=18, a8=81, h8=88. */
pub type Board = [i32; 128];


#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub move_0: i32,
    pub score: i32,
    pub prob: i32,
}
/*
   File:         enddev.c

   Created:      September 1, 2002

   Modified:

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:
*/
static mut VERBOSE: i32 = 1;
unsafe extern "C" fn read_game(mut stream: FileHandle,
                               mut game_moves: *mut i32,
                               mut game_length: *mut i32) {
    let mut buffer: [i8; 1000] = [0; 1000];
    if fgets(buffer.as_mut_ptr(),
             ::std::mem::size_of::<[i8; 1000]>() as u64 as
                 i32, stream).is_null() {
        return
    } else {
        let mut i: i32 = 0;
        let mut ch: *mut i8 = buffer.as_mut_ptr();
        while *(*__ctype_b_loc()).offset(*ch as i32 as isize) as
            i32 &
            _ISalnum as i32 as u16 as i32 !=
            0 {
            ch = ch.offset(1)
        }
        *ch = 0;
        *game_length = strlen(buffer.as_mut_ptr()).wrapping_div(2) as i32;
        if *game_length > 60 || strlen(buffer.as_mut_ptr()).wrapping_rem(2) == 1 {
            fprintf(stderr,
                    b"Bad move string %s.\n\x00" as *const u8 as
                        *const i8, buffer.as_mut_ptr());
            exit(1);
        }
        i = 0;
        while i < *game_length {
            let mut col: i32 = tolower(buffer[(2 * i) as usize] as i32) - 'a' as i32 + 1;
            let mut row: i32 = buffer[(2 * i + 1) as usize] as i32 - '0' as i32;
            if col < 1 || col > 8 ||
                row < 1 || row > 8 {
                fprintf(stderr,
                        b"Unexpected character in move string\x00" as
                            *const u8 as *const i8);
            }
            *game_moves.offset(i as isize) = 10 * row + col;
            i += 1
        }
    };
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8)
                 -> i32 {
    let mut rand_prob: f64 = 0.;
    let hash_bits: i32 = 20;
    let earliest_dev: i32 = 38;
    let latest_dev: i32 = 52;
    let mut first_allowed_dev: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut last_was_pass: i32 = 0;
    let mut restart: i32 = 0;
    let mut in_branch: i32 = 0;
    let mut games_read: i32 = 0;
    let mut game_length: i32 = 0;
    let mut game_moves: [i32; 60] = [0; 60];
    if argc != 3 ||
        sscanf(*argv.offset(2),
               b"%lf\x00" as *const u8 as *const i8,
               &mut rand_prob as *mut f64) != 1 {
        fputs(b"Usage:\n  enddev <game file> <randomization prob.>\n\x00" as
                  *const u8 as *const i8, stderr);
        exit(1);
    }
    let stream =
        fopen(*argv.offset(1),
              b"r\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fprintf(stderr,
                b"Cannot open %s for reading.\n\x00" as *const u8 as
                    *const i8,
                *argv.offset(1));
        exit(1);
    }
    static src: LibcTimeSource = LibcTimeSource {};
    let mut full_state = FullState::new(&src);
    let g_state: &mut FullState = &mut full_state;
    init_learn(b"book.bin\x00" as *const u8 as *const i8,
               1, g_state);
    global_setup(0, hash_bits, g_state);
    games_read = 0;
    first_allowed_dev = earliest_dev;
    last_was_pass = 0;
    restart = 1;
    in_branch = 0;
    loop  {
        if restart != 0 {
            if in_branch != 0 {
                game_length = g_state.moves.disks_played
            } else {
                read_game(stream, game_moves.as_mut_ptr(), &mut game_length);
                if games_read % 1000 == 0 {
                    fprintf(stderr,
                            b"%d games processed\n\x00" as *const u8 as
                                *const i8, games_read);
                }
                if feof(stream) != 0 { break ; }
                games_read += 1;
                first_allowed_dev = earliest_dev
            }
            game_init(&mut side_to_move, g_state);
            setup_hash(1, &mut g_state.hash, &mut g_state.random);
            last_was_pass = 0;
            restart = 0;
            in_branch = 0
        }
        assert_eq!(disc_count(0, &g_state.board.board) + disc_count(2, &g_state.board.board), g_state.moves.disks_played + 4);
        determine_hash_values(side_to_move, &g_state.board.board, &mut g_state.hash);
        generate_all(side_to_move, &mut g_state.moves, &g_state.search, &g_state.board.board);
        if g_state.moves.move_count[g_state.moves.disks_played as usize] == 0 {
            if last_was_pass != 0 {
                /* Game over? */
                restart = 1; /* Must pass. */
                if in_branch != 0 {
                    let mut i: i32 = 0;
                    i = 0;
                    while i < g_state.moves.disks_played {
                        printf(b"%c%c\x00" as *const u8 as
                                   *const i8,
                               'a' as i32 +
                                   game_moves[i as usize] % 10
                                   - 1,
                               '0' as i32 +
                                   game_moves[i as usize] /
                                       10);
                        i += 1
                    }
                    write!(stdout, "\n");
                }
            } else {
                side_to_move =
                    0 + 2 - side_to_move;
                last_was_pass = 1
            }
        } else {
            let mut move_0: i32 = 0;
             g_state.timer.start_move(100000 as f64,
                                      0 as f64,
                                      g_state.moves.disks_played + 4);
            if in_branch != 0 {
                let mut ev_info: EvaluationType =  EvaluationType::new();
                move_0 =
                    legacy_compute_move(side_to_move, 0,
                                        100000, 0,
                                        0, 0,
                                        8, 60,
                                        60, 1,
                                        &mut ev_info, g_state) as i32
            } else {
                move_0 = game_moves[g_state.moves.disks_played as usize];
                if g_state.moves.disks_played >= first_allowed_dev &&
                    g_state.moves.disks_played <= latest_dev &&
                    (0.0001f64 *
                        ((g_state.random.my_random() >> 9) %
                            10000 as i64) as
                            f64) < rand_prob {
                    let mut i_0: i32 = 0;
                    let mut best_score: i32 = 0;
                    let mut accum_prob: i32 = 0;
                    let mut total_prob: i32 = 0;
                    let mut rand_val: i32 = 0;
                    let mut choices: [C2RustUnnamed_0; 60] =
                        [C2RustUnnamed_0{move_0: 0, score: 0, prob: 0,}; 60];
                    if VERBOSE != 0 {
                        fprintf(stderr,
                                b"Evaluating moves in game %d after %d moves:\n\x00"
                                    as *const u8 as *const i8,
                                games_read, g_state.moves.disks_played);
                    }
                    let evaluated_list = extended_compute_move::<LibcFatalError>(side_to_move, 0,
                                                                                 0, 8,
                                                                                 60,
                                                                                 60, g_state.config.echo, g_state);
                    assert_eq!(evaluated_list.get_evaluated_count(), g_state.moves.move_count[g_state.moves.disks_played as usize]);
                    best_score = -(12345678);
                    i_0 = 0;
                    while i_0 < evaluated_list.get_evaluated_count() {
                        let mut ev_info_0: EvaluatedMove = evaluated_list.get_evaluated(i_0);
                        choices[i_0 as usize].move_0 = ev_info_0.move_0 as i32;
                        choices[i_0 as usize].score =
                            ev_info_0.eval.score / 128;
                        best_score =
                            if choices[i_0 as usize].score > best_score {
                                choices[i_0 as usize].score
                            } else { best_score };
                        i_0 += 1
                    }
                    total_prob = 0;
                    i_0 = 0;
                    while i_0 < evaluated_list.get_evaluated_count() {
                        choices[i_0 as usize].prob =
                            (100000 as f64 * ((choices[i_0 as usize].score - best_score) as f64 * 0.2f64).exp() + 1_f64) as i32;
                        if choices[i_0 as usize].move_0 == move_0 {
                            /* Encourage deviations. */
                            choices[i_0 as usize].prob =
                                choices[i_0 as usize].prob / 2
                        }
                        total_prob += choices[i_0 as usize].prob;
                        i_0 += 1
                    }
                    if VERBOSE != 0 {
                        i_0 = 0;
                        while i_0 < evaluated_list.get_evaluated_count() {
                            fprintf(stderr,
                                    b"  %c%c  %+3d    p=%.03f\n\x00" as
                                        *const u8 as *const i8,
                                    'a' as i32 +
                                        choices[i_0 as usize].move_0 %
                                            10 -
                                        1,
                                    '0' as i32 +
                                        choices[i_0 as usize].move_0 /
                                            10,
                                    choices[i_0 as usize].score,
                                    choices[i_0 as usize].prob as
                                        f64 /
                                        total_prob as f64);
                            i_0 += 1
                        }
                    }
                    rand_val =
                        ((g_state.random.my_random() >> 4) %
                            (total_prob + 1) as i64)
                            as i32;
                    accum_prob = 0;
                    i_0 = 0;
                    loop  {
                        accum_prob += choices[i_0 as usize].prob;
                        if !(accum_prob < rand_val) { break ; }
                        i_0 += 1
                    }
                    assert!(i_0 < g_state.moves.move_count[g_state.moves.disks_played as usize]);
                    move_0 = choices[i_0 as usize].move_0;
                    if VERBOSE != 0 {
                        fprintf(stderr,
                                b"  %c%c chosen, %c%c in game\n\x00" as
                                    *const u8 as *const i8,
                                'a' as i32 + move_0 % 10 -
                                    1,
                                '0' as i32 + move_0 / 10,
                                'a' as i32 +
                                    game_moves[g_state.moves.disks_played as usize] %
                                        10 - 1,
                                '0' as i32 +
                                    game_moves[g_state.moves.disks_played as usize] /
                                        10);
                    }
                    if move_0 != game_moves[g_state.moves.disks_played as usize] {
                        in_branch = 1;
                        first_allowed_dev = g_state.moves.disks_played + 1;
                        if VERBOSE != 0 {
                            fputs(b"  branching\n\x00" as *const u8 as
                                      *const i8, stderr);
                        }
                    }
                }
            }
            if valid_move(move_0 as i8, side_to_move, &g_state.board.board) == 0 {
                fprintf(stderr,
                        b"Game #%d contains illegal move %d @ #%d.\n\x00" as
                            *const u8 as *const i8, games_read,
                        move_0, g_state.moves.disks_played);
                display_state.display_board(&mut libc_wrapper::stderr, &g_state.board.board, side_to_move,
                                            0, 0, 0,
                                            &g_state.board.black_moves, &g_state.board.white_moves);
                exit(1);
            }
            game_moves[g_state.moves.disks_played as usize] = move_0;
            if make_move(side_to_move, move_0 as i8, 1, &mut g_state.moves, &mut g_state.board, &mut g_state.hash, &mut &mut g_state.flip_stack) ==
                0 {
                fprintf(stderr,
                        b"Internal error: \'Legal\' move flips no discs.\n\x00"
                            as *const u8 as *const i8);
                exit(1);
            }
            side_to_move = 0 + 2 - side_to_move;
            last_was_pass = 0
        }
    }
    fprintf(stderr,
            b"%d games processed\n\x00" as *const u8 as *const i8,
            games_read);
    return 0;
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
