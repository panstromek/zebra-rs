#![allow(dead_code,  non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value, register_tool)]

use engine::src::moves::{disks_played, make_move, valid_move, move_count, generate_all};
use engine::src::globals::{board, white_moves, black_moves};
use engine::src::myrandom::my_random;
use engine::src::game::{get_evaluated_count, get_evaluated, EvaluatedMove};
use engine::src::zebra::EvaluationType;
use engine::src::timer::start_move;
use engine::src::hash::{determine_hash_values, setup_hash};
use engine::src::search::disc_count;
use c2rust_out::src::display::display_board;
use c2rust_out::src::game::{extended_compute_move, compute_move, game_init, global_setup};
use c2rust_out::src::learn::init_learn;
use c2rust_out::src::zebra::_IO_FILE;
use c2rust_out::src::error::{LibcFatalError, FE};
use engine::src::error::FrontEnd;
use engine::src::display::{current_row, white_time, black_time, black_eval, white_eval, black_player, white_player};

extern "C" {
    #[no_mangle]
    fn __assert_fail(__assertion: *const i8,
                     __file: *const i8, __line: u32,
                     __function: *const i8) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const u16;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn exp(_: f64) -> f64;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fopen(__filename: *const i8, __modes: *const i8)
             -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn printf(_: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn sscanf(_: *const i8, _: *const i8, _: ...)
              -> i32;
    #[no_mangle]
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE)
             -> *mut i8;
    #[no_mangle]
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    #[no_mangle]
    fn puts(__s: *const i8) -> i32;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn exit(_: i32) -> !;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
}
pub type __int32_t = i32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = u64;
pub type _IO_lock_t = ();
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
pub struct C2RustUnnamed_0 {
    pub move_0: i32,
    pub score: i32,
    pub prob: i32,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else { __c };
}
/*
   File:         enddev.c

   Created:      September 1, 2002

   Modified:

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:
*/
static mut VERBOSE: i32 = 1 as i32;
unsafe extern "C" fn read_game(mut stream: *mut FILE,
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
        *ch = 0 as i32 as i8;
        *game_length =
              FE::strlen(buffer.as_mut_ptr()).wrapping_div(2 as i32 as
                u64) as
                i32;
        if *game_length > 60 as i32 ||
              FE::strlen(buffer.as_mut_ptr()).wrapping_rem(2 as i32 as
                u64) ==
                1 as i32 as u64 {
            fprintf(stderr,
                    b"Bad move string %s.\n\x00" as *const u8 as
                        *const i8, buffer.as_mut_ptr());
            exit(1 as i32);
        }
        i = 0 as i32;
        while i < *game_length {
            let mut col: i32 =
                ({
                    let mut __res: i32 = 0;
                    if ::std::mem::size_of::<i8>() as u64
                        > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 =
                                buffer[(2 as i32 * i) as usize] as
                                    i32;
                            __res =
                                if __c < -(128 as i32) ||
                                    __c > 255 as i32 {
                                    __c
                                } else {
                                    *(*__ctype_tolower_loc()).offset(__c as
                                        isize)
                                }
                        } else {
                            __res =
                               FE::tolower(buffer[(2 as i32 * i) as
                                    usize] as i32)
                        }
                    } else {
                        __res =
                            *(*__ctype_tolower_loc()).offset(buffer[(2 as
                                i32
                                * i)
                                as
                                usize]
                                as
                                i32
                                as isize)
                    }
                    __res
                }) - 'a' as i32 + 1 as i32;
            let mut row: i32 =
                buffer[(2 as i32 * i + 1 as i32) as usize] as
                    i32 - '0' as i32;
            if col < 1 as i32 || col > 8 as i32 ||
                row < 1 as i32 || row > 8 as i32 {
                fprintf(stderr,
                        b"Unexpected character in move string\x00" as
                            *const u8 as *const i8);
            }
            *game_moves.offset(i as isize) = 10 as i32 * row + col;
            i += 1
        }
    };
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8)
                 -> i32 {
    let mut rand_prob: f64 = 0.;
    let hash_bits: i32 = 20 as i32;
    let earliest_dev: i32 = 38 as i32;
    let latest_dev: i32 = 52 as i32;
    let mut first_allowed_dev: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut last_was_pass: i32 = 0;
    let mut restart: i32 = 0;
    let mut in_branch: i32 = 0;
    let mut games_read: i32 = 0;
    let mut game_length: i32 = 0;
    let mut game_moves: [i32; 60] = [0; 60];
    let mut stream: *mut FILE = 0 as *mut FILE;
    if argc != 3 as i32 ||
        sscanf(*argv.offset(2 as i32 as isize),
               b"%lf\x00" as *const u8 as *const i8,
               &mut rand_prob as *mut f64) != 1 as i32 {
        fputs(b"Usage:\n  enddev <game file> <randomization prob.>\n\x00" as
                  *const u8 as *const i8, stderr);
        exit(1 as i32);
    }
    stream =
        fopen(*argv.offset(1 as i32 as isize),
              b"r\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fprintf(stderr,
                b"Cannot open %s for reading.\n\x00" as *const u8 as
                    *const i8,
                *argv.offset(1 as i32 as isize));
        exit(1 as i32);
    }
    init_learn(b"book.bin\x00" as *const u8 as *const i8,
               1 as i32);
    global_setup(0 as i32, hash_bits);
    games_read = 0 as i32;
    first_allowed_dev = earliest_dev;
    last_was_pass = 0 as i32;
    restart = 1 as i32;
    in_branch = 0 as i32;
    loop  {
        if restart != 0 {
            if in_branch != 0 {
                game_length = disks_played
            } else {
                read_game(stream, game_moves.as_mut_ptr(), &mut game_length);
                if games_read % 1000 as i32 == 0 as i32 {
                    fprintf(stderr,
                            b"%d games processed\n\x00" as *const u8 as
                                *const i8, games_read);
                }
                if feof(stream) != 0 { break ; }
                games_read += 1;
                first_allowed_dev = earliest_dev
            }
            game_init(0 as *const i8, &mut side_to_move);
            setup_hash(1 as i32);
            last_was_pass = 0 as i32;
            restart = 0 as i32;
            in_branch = 0 as i32
        }
        if disc_count(0 as i32) + disc_count(2 as i32) ==
            disks_played + 4 as i32 {
        } else {
            __assert_fail(b"disc_count( BLACKSQ ) + disc_count( WHITESQ ) == disks_played + 4\x00"
                              as *const u8 as *const i8,
                          b"enddev.c\x00" as *const u8 as *const i8,
                          134 as i32 as u32,
                          (*::std::mem::transmute::<&[u8; 23],
                              &[i8; 23]>(b"int main(int, char **)\x00")).as_ptr());
        }
        determine_hash_values(side_to_move, board.as_mut_ptr());
        generate_all(side_to_move);
        if move_count[disks_played as usize] == 0 as i32 {
            if last_was_pass != 0 {
                /* Game over? */
                restart = 1 as i32; /* Must pass. */
                if in_branch != 0 {
                    let mut i: i32 = 0;
                    i = 0 as i32;
                    while i < disks_played {
                        printf(b"%c%c\x00" as *const u8 as
                                   *const i8,
                               'a' as i32 +
                                   game_moves[i as usize] % 10 as i32
                                   - 1 as i32,
                               '0' as i32 +
                                   game_moves[i as usize] /
                                       10 as i32);
                        i += 1
                    }
                    puts(b"\x00" as *const u8 as *const i8);
                }
            } else {
                side_to_move =
                    0 as i32 + 2 as i32 - side_to_move;
                last_was_pass = 1 as i32
            }
        } else {
            let mut move_0: i32 = 0;
            start_move::<FE>(100000 as i32 as f64,
                       0 as i32 as f64,
                       disks_played + 4 as i32);
            if in_branch != 0 {
                let mut ev_info: EvaluationType =
                    EvaluationType{type_0: MIDGAME_EVAL,
                        res: WON_POSITION,
                        score: 0,
                        confidence: 0.,
                        search_depth: 0,
                        is_book: 0,};
                move_0 =
                    compute_move(side_to_move, 0 as i32,
                                 100000 as i32, 0 as i32,
                                 0 as i32, 0 as i32,
                                 8 as i32, 60 as i32,
                                 60 as i32, 1 as i32,
                                 &mut ev_info)
            } else {
                move_0 = game_moves[disks_played as usize];
                if disks_played >= first_allowed_dev &&
                    disks_played <= latest_dev &&
                    (0.0001f64 *
                        ((my_random() >> 9 as i32) %
                            10000 as i32 as i64) as
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
                                games_read, disks_played);
                    }
                    extended_compute_move::<LibcFatalError>(side_to_move, 0 as i32,
                                          0 as i32, 8 as i32,
                                          60 as i32,
                                          60 as i32);
                    if get_evaluated_count() ==
                        move_count[disks_played as usize] {
                    } else {
                        __assert_fail(b"get_evaluated_count() == move_count[disks_played]\x00"
                                          as *const u8 as *const i8,
                                      b"enddev.c\x00" as *const u8 as
                                          *const i8,
                                      186 as i32 as u32,
                                      (*::std::mem::transmute::<&[u8; 23],
                                          &[i8; 23]>(b"int main(int, char **)\x00")).as_ptr());
                    }
                    best_score = -(12345678 as i32);
                    i_0 = 0 as i32;
                    while i_0 < get_evaluated_count() {
                        let mut ev_info_0: EvaluatedMove = get_evaluated(i_0);
                        choices[i_0 as usize].move_0 = ev_info_0.move_0;
                        choices[i_0 as usize].score =
                            ev_info_0.eval.score / 128 as i32;
                        best_score =
                            if choices[i_0 as usize].score > best_score {
                                choices[i_0 as usize].score
                            } else { best_score };
                        i_0 += 1
                    }
                    total_prob = 0 as i32;
                    i_0 = 0 as i32;
                    while i_0 < get_evaluated_count() {
                        choices[i_0 as usize].prob =
                            (100000 as i32 as f64 *
                                exp((choices[i_0 as usize].score -
                                    best_score) as f64 *
                                    0.2f64) +
                                1 as i32 as f64) as
                                i32;
                        if choices[i_0 as usize].move_0 == move_0 {
                            /* Encourage deviations. */
                            choices[i_0 as usize].prob =
                                choices[i_0 as usize].prob / 2 as i32
                        }
                        total_prob += choices[i_0 as usize].prob;
                        i_0 += 1
                    }
                    if VERBOSE != 0 {
                        i_0 = 0 as i32;
                        while i_0 < get_evaluated_count() {
                            fprintf(stderr,
                                    b"  %c%c  %+3d    p=%.03f\n\x00" as
                                        *const u8 as *const i8,
                                    'a' as i32 +
                                        choices[i_0 as usize].move_0 %
                                            10 as i32 -
                                        1 as i32,
                                    '0' as i32 +
                                        choices[i_0 as usize].move_0 /
                                            10 as i32,
                                    choices[i_0 as usize].score,
                                    choices[i_0 as usize].prob as
                                        f64 /
                                        total_prob as f64);
                            i_0 += 1
                        }
                    }
                    rand_val =
                        ((my_random() >> 4 as i32) %
                            (total_prob + 1 as i32) as i64)
                            as i32;
                    accum_prob = 0 as i32;
                    i_0 = 0 as i32;
                    loop  {
                        accum_prob += choices[i_0 as usize].prob;
                        if !(accum_prob < rand_val) { break ; }
                        i_0 += 1
                    }
                    if i_0 < move_count[disks_played as usize] {
                    } else {
                        __assert_fail(b"i < move_count[disks_played]\x00" as
                                          *const u8 as *const i8,
                                      b"enddev.c\x00" as *const u8 as
                                          *const i8,
                                      218 as i32 as u32,
                                      (*::std::mem::transmute::<&[u8; 23],
                                          &[i8; 23]>(b"int main(int, char **)\x00")).as_ptr());
                    }
                    move_0 = choices[i_0 as usize].move_0;
                    if VERBOSE != 0 {
                        fprintf(stderr,
                                b"  %c%c chosen, %c%c in game\n\x00" as
                                    *const u8 as *const i8,
                                'a' as i32 + move_0 % 10 as i32 -
                                    1 as i32,
                                '0' as i32 + move_0 / 10 as i32,
                                'a' as i32 +
                                    game_moves[disks_played as usize] %
                                        10 as i32 - 1 as i32,
                                '0' as i32 +
                                    game_moves[disks_played as usize] /
                                        10 as i32);
                    }
                    if move_0 != game_moves[disks_played as usize] {
                        in_branch = 1 as i32;
                        first_allowed_dev = disks_played + 1 as i32;
                        if VERBOSE != 0 {
                            fputs(b"  branching\n\x00" as *const u8 as
                                      *const i8, stderr);
                        }
                    }
                }
            }
            if valid_move(move_0, side_to_move) == 0 {
                fprintf(stderr,
                        b"Game #%d contains illegal move %d @ #%d.\n\x00" as
                            *const u8 as *const i8, games_read,
                        move_0, disks_played);
                display_board(stderr, board.as_mut_ptr(), side_to_move,
                              0 as i32, 0 as i32,
                              0 as i32, current_row,
                              black_player, black_time, black_eval,
                              white_player, white_time, white_eval,
                              &black_moves, &white_moves);
                exit(1 as i32);
            }
            game_moves[disks_played as usize] = move_0;
            if make_move(side_to_move, move_0, 1 as i32) ==
                0 as i32 {
                fprintf(stderr,
                        b"Internal error: \'Legal\' move flips no discs.\n\x00"
                            as *const u8 as *const i8);
                exit(1 as i32);
            }
            side_to_move = 0 as i32 + 2 as i32 - side_to_move;
            last_was_pass = 0 as i32
        }
    }
    fprintf(stderr,
            b"%d games processed\n\x00" as *const u8 as *const i8,
            games_read);
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
