#![allow(dead_code,  non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value, register_tool)]

use engine::src::moves::{disks_played, make_move, valid_move, move_count, generate_all};
use engine::src::globals::board;
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
use engine::src::error::LibcFatalError;

extern "C" {
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn exp(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
             -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
              -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
             -> *mut libc::c_char;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
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
pub type size_t = libc::c_ulong;
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub move_0: libc::c_int,
    pub score: libc::c_int,
    pub prob: libc::c_int,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
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
static mut VERBOSE: libc::c_int = 1 as libc::c_int;
unsafe extern "C" fn read_game(mut stream: *mut FILE,
                               mut game_moves: *mut libc::c_int,
                               mut game_length: *mut libc::c_int) {
    let mut buffer: [libc::c_char; 1000] = [0; 1000];
    if fgets(buffer.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 1000]>() as libc::c_ulong as
                 libc::c_int, stream).is_null() {
        return
    } else {
        let mut i: libc::c_int = 0;
        let mut ch: *mut libc::c_char = buffer.as_mut_ptr();
        while *(*__ctype_b_loc()).offset(*ch as libc::c_int as isize) as
            libc::c_int &
            _ISalnum as libc::c_int as libc::c_ushort as libc::c_int !=
            0 {
            ch = ch.offset(1)
        }
        *ch = 0 as libc::c_int as libc::c_char;
        *game_length =
            strlen(buffer.as_mut_ptr()).wrapping_div(2 as libc::c_int as
                libc::c_ulong) as
                libc::c_int;
        if *game_length > 60 as libc::c_int ||
            strlen(buffer.as_mut_ptr()).wrapping_rem(2 as libc::c_int as
                libc::c_ulong) ==
                1 as libc::c_int as libc::c_ulong {
            fprintf(stderr,
                    b"Bad move string %s.\n\x00" as *const u8 as
                        *const libc::c_char, buffer.as_mut_ptr());
            exit(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < *game_length {
            let mut col: libc::c_int =
                ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong {
                        if 0 != 0 {
                            let mut __c: libc::c_int =
                                buffer[(2 as libc::c_int * i) as usize] as
                                    libc::c_int;
                            __res =
                                if __c < -(128 as libc::c_int) ||
                                    __c > 255 as libc::c_int {
                                    __c
                                } else {
                                    *(*__ctype_tolower_loc()).offset(__c as
                                        isize)
                                }
                        } else {
                            __res =
                                tolower(buffer[(2 as libc::c_int * i) as
                                    usize] as libc::c_int)
                        }
                    } else {
                        __res =
                            *(*__ctype_tolower_loc()).offset(buffer[(2 as
                                libc::c_int
                                * i)
                                as
                                usize]
                                as
                                libc::c_int
                                as isize)
                    }
                    __res
                }) - 'a' as i32 + 1 as libc::c_int;
            let mut row: libc::c_int =
                buffer[(2 as libc::c_int * i + 1 as libc::c_int) as usize] as
                    libc::c_int - '0' as i32;
            if col < 1 as libc::c_int || col > 8 as libc::c_int ||
                row < 1 as libc::c_int || row > 8 as libc::c_int {
                fprintf(stderr,
                        b"Unexpected character in move string\x00" as
                            *const u8 as *const libc::c_char);
            }
            *game_moves.offset(i as isize) = 10 as libc::c_int * row + col;
            i += 1
        }
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
                 -> libc::c_int {
    let mut rand_prob: libc::c_double = 0.;
    let hash_bits: libc::c_int = 20 as libc::c_int;
    let earliest_dev: libc::c_int = 38 as libc::c_int;
    let latest_dev: libc::c_int = 52 as libc::c_int;
    let mut first_allowed_dev: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut last_was_pass: libc::c_int = 0;
    let mut restart: libc::c_int = 0;
    let mut in_branch: libc::c_int = 0;
    let mut games_read: libc::c_int = 0;
    let mut game_length: libc::c_int = 0;
    let mut game_moves: [libc::c_int; 60] = [0; 60];
    let mut stream: *mut FILE = 0 as *mut FILE;
    if argc != 3 as libc::c_int ||
        sscanf(*argv.offset(2 as libc::c_int as isize),
               b"%lf\x00" as *const u8 as *const libc::c_char,
               &mut rand_prob as *mut libc::c_double) != 1 as libc::c_int {
        fputs(b"Usage:\n  enddev <game file> <randomization prob.>\n\x00" as
                  *const u8 as *const libc::c_char, stderr);
        exit(1 as libc::c_int);
    }
    stream =
        fopen(*argv.offset(1 as libc::c_int as isize),
              b"r\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() {
        fprintf(stderr,
                b"Cannot open %s for reading.\n\x00" as *const u8 as
                    *const libc::c_char,
                *argv.offset(1 as libc::c_int as isize));
        exit(1 as libc::c_int);
    }
    init_learn(b"book.bin\x00" as *const u8 as *const libc::c_char,
               1 as libc::c_int);
    global_setup(0 as libc::c_int, hash_bits);
    games_read = 0 as libc::c_int;
    first_allowed_dev = earliest_dev;
    last_was_pass = 0 as libc::c_int;
    restart = 1 as libc::c_int;
    in_branch = 0 as libc::c_int;
    loop  {
        if restart != 0 {
            if in_branch != 0 {
                game_length = disks_played
            } else {
                read_game(stream, game_moves.as_mut_ptr(), &mut game_length);
                if games_read % 1000 as libc::c_int == 0 as libc::c_int {
                    fprintf(stderr,
                            b"%d games processed\n\x00" as *const u8 as
                                *const libc::c_char, games_read);
                }
                if feof(stream) != 0 { break ; }
                games_read += 1;
                first_allowed_dev = earliest_dev
            }
            game_init(0 as *const libc::c_char, &mut side_to_move);
            setup_hash(1 as libc::c_int);
            last_was_pass = 0 as libc::c_int;
            restart = 0 as libc::c_int;
            in_branch = 0 as libc::c_int
        }
        if disc_count(0 as libc::c_int) + disc_count(2 as libc::c_int) ==
            disks_played + 4 as libc::c_int {
        } else {
            __assert_fail(b"disc_count( BLACKSQ ) + disc_count( WHITESQ ) == disks_played + 4\x00"
                              as *const u8 as *const libc::c_char,
                          b"enddev.c\x00" as *const u8 as *const libc::c_char,
                          134 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 23],
                              &[libc::c_char; 23]>(b"int main(int, char **)\x00")).as_ptr());
        }
        determine_hash_values(side_to_move, board.as_mut_ptr());
        generate_all(side_to_move);
        if move_count[disks_played as usize] == 0 as libc::c_int {
            if last_was_pass != 0 {
                /* Game over? */
                restart = 1 as libc::c_int; /* Must pass. */
                if in_branch != 0 {
                    let mut i: libc::c_int = 0;
                    i = 0 as libc::c_int;
                    while i < disks_played {
                        printf(b"%c%c\x00" as *const u8 as
                                   *const libc::c_char,
                               'a' as i32 +
                                   game_moves[i as usize] % 10 as libc::c_int
                                   - 1 as libc::c_int,
                               '0' as i32 +
                                   game_moves[i as usize] /
                                       10 as libc::c_int);
                        i += 1
                    }
                    puts(b"\x00" as *const u8 as *const libc::c_char);
                }
            } else {
                side_to_move =
                    0 as libc::c_int + 2 as libc::c_int - side_to_move;
                last_was_pass = 1 as libc::c_int
            }
        } else {
            let mut move_0: libc::c_int = 0;
            start_move(100000 as libc::c_int as libc::c_double,
                       0 as libc::c_int as libc::c_double,
                       disks_played + 4 as libc::c_int);
            if in_branch != 0 {
                let mut ev_info: EvaluationType =
                    EvaluationType{type_0: MIDGAME_EVAL,
                        res: WON_POSITION,
                        score: 0,
                        confidence: 0.,
                        search_depth: 0,
                        is_book: 0,};
                move_0 =
                    compute_move(side_to_move, 0 as libc::c_int,
                                 100000 as libc::c_int, 0 as libc::c_int,
                                 0 as libc::c_int, 0 as libc::c_int,
                                 8 as libc::c_int, 60 as libc::c_int,
                                 60 as libc::c_int, 1 as libc::c_int,
                                 &mut ev_info)
            } else {
                move_0 = game_moves[disks_played as usize];
                if disks_played >= first_allowed_dev &&
                    disks_played <= latest_dev &&
                    (0.0001f64 *
                        ((my_random() >> 9 as libc::c_int) %
                            10000 as libc::c_int as libc::c_long) as
                            libc::c_double) < rand_prob {
                    let mut i_0: libc::c_int = 0;
                    let mut best_score: libc::c_int = 0;
                    let mut accum_prob: libc::c_int = 0;
                    let mut total_prob: libc::c_int = 0;
                    let mut rand_val: libc::c_int = 0;
                    let mut choices: [C2RustUnnamed_0; 60] =
                        [C2RustUnnamed_0{move_0: 0, score: 0, prob: 0,}; 60];
                    if VERBOSE != 0 {
                        fprintf(stderr,
                                b"Evaluating moves in game %d after %d moves:\n\x00"
                                    as *const u8 as *const libc::c_char,
                                games_read, disks_played);
                    }
                    extended_compute_move::<LibcFatalError>(side_to_move, 0 as libc::c_int,
                                          0 as libc::c_int, 8 as libc::c_int,
                                          60 as libc::c_int,
                                          60 as libc::c_int);
                    if get_evaluated_count() ==
                        move_count[disks_played as usize] {
                    } else {
                        __assert_fail(b"get_evaluated_count() == move_count[disks_played]\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"enddev.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      186 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 23],
                                          &[libc::c_char; 23]>(b"int main(int, char **)\x00")).as_ptr());
                    }
                    best_score = -(12345678 as libc::c_int);
                    i_0 = 0 as libc::c_int;
                    while i_0 < get_evaluated_count() {
                        let mut ev_info_0: EvaluatedMove = get_evaluated(i_0);
                        choices[i_0 as usize].move_0 = ev_info_0.move_0;
                        choices[i_0 as usize].score =
                            ev_info_0.eval.score / 128 as libc::c_int;
                        best_score =
                            if choices[i_0 as usize].score > best_score {
                                choices[i_0 as usize].score
                            } else { best_score };
                        i_0 += 1
                    }
                    total_prob = 0 as libc::c_int;
                    i_0 = 0 as libc::c_int;
                    while i_0 < get_evaluated_count() {
                        choices[i_0 as usize].prob =
                            (100000 as libc::c_int as libc::c_double *
                                exp((choices[i_0 as usize].score -
                                    best_score) as libc::c_double *
                                    0.2f64) +
                                1 as libc::c_int as libc::c_double) as
                                libc::c_int;
                        if choices[i_0 as usize].move_0 == move_0 {
                            /* Encourage deviations. */
                            choices[i_0 as usize].prob =
                                choices[i_0 as usize].prob / 2 as libc::c_int
                        }
                        total_prob += choices[i_0 as usize].prob;
                        i_0 += 1
                    }
                    if VERBOSE != 0 {
                        i_0 = 0 as libc::c_int;
                        while i_0 < get_evaluated_count() {
                            fprintf(stderr,
                                    b"  %c%c  %+3d    p=%.03f\n\x00" as
                                        *const u8 as *const libc::c_char,
                                    'a' as i32 +
                                        choices[i_0 as usize].move_0 %
                                            10 as libc::c_int -
                                        1 as libc::c_int,
                                    '0' as i32 +
                                        choices[i_0 as usize].move_0 /
                                            10 as libc::c_int,
                                    choices[i_0 as usize].score,
                                    choices[i_0 as usize].prob as
                                        libc::c_double /
                                        total_prob as libc::c_double);
                            i_0 += 1
                        }
                    }
                    rand_val =
                        ((my_random() >> 4 as libc::c_int) %
                            (total_prob + 1 as libc::c_int) as libc::c_long)
                            as libc::c_int;
                    accum_prob = 0 as libc::c_int;
                    i_0 = 0 as libc::c_int;
                    loop  {
                        accum_prob += choices[i_0 as usize].prob;
                        if !(accum_prob < rand_val) { break ; }
                        i_0 += 1
                    }
                    if i_0 < move_count[disks_played as usize] {
                    } else {
                        __assert_fail(b"i < move_count[disks_played]\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"enddev.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      218 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 23],
                                          &[libc::c_char; 23]>(b"int main(int, char **)\x00")).as_ptr());
                    }
                    move_0 = choices[i_0 as usize].move_0;
                    if VERBOSE != 0 {
                        fprintf(stderr,
                                b"  %c%c chosen, %c%c in game\n\x00" as
                                    *const u8 as *const libc::c_char,
                                'a' as i32 + move_0 % 10 as libc::c_int -
                                    1 as libc::c_int,
                                '0' as i32 + move_0 / 10 as libc::c_int,
                                'a' as i32 +
                                    game_moves[disks_played as usize] %
                                        10 as libc::c_int - 1 as libc::c_int,
                                '0' as i32 +
                                    game_moves[disks_played as usize] /
                                        10 as libc::c_int);
                    }
                    if move_0 != game_moves[disks_played as usize] {
                        in_branch = 1 as libc::c_int;
                        first_allowed_dev = disks_played + 1 as libc::c_int;
                        if VERBOSE != 0 {
                            fputs(b"  branching\n\x00" as *const u8 as
                                      *const libc::c_char, stderr);
                        }
                    }
                }
            }
            if valid_move(move_0, side_to_move) == 0 {
                fprintf(stderr,
                        b"Game #%d contains illegal move %d @ #%d.\n\x00" as
                            *const u8 as *const libc::c_char, games_read,
                        move_0, disks_played);
                display_board(stderr, board.as_mut_ptr(), side_to_move,
                              0 as libc::c_int, 0 as libc::c_int,
                              0 as libc::c_int);
                exit(1 as libc::c_int);
            }
            game_moves[disks_played as usize] = move_0;
            if make_move(side_to_move, move_0, 1 as libc::c_int) ==
                0 as libc::c_int {
                fprintf(stderr,
                        b"Internal error: \'Legal\' move flips no discs.\n\x00"
                            as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
            side_to_move = 0 as libc::c_int + 2 as libc::c_int - side_to_move;
            last_was_pass = 0 as libc::c_int
        }
    }
    fprintf(stderr,
            b"%d games processed\n\x00" as *const u8 as *const libc::c_char,
            games_read);
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
