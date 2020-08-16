#![allow(dead_code,  non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, extern_types)]

use engine::src::game::{global_terminate, set_komi, toggle_human_openings, toggle_status_log};
use engine::src::myrandom::my_srandom;
use engine::src::stubs::{time, strlen};
use engine::src::thordb::init_thor_database;
use engine::src::display::{echo, display_pv, set_move_list, set_evals, set_names};
use engine::src::counter::{counter_value, add_counter, reset_counter, CounterType};
use engine::src::timer::{get_real_timer, determine_move_time, start_move};
use engine::src::search::{full_pv, full_pv_depth, nodes, disc_count};
use engine::src::moves::disks_played;
use engine::src::globals::{board, score_sheet_row, white_moves, black_moves};
use engine::src::hash::setup_hash;
use engine::src::osfbook::{set_deviation_value, reset_book_search, set_slack};
use c2rust_out::src::zebra::{_IO_FILE};
use c2rust_out::src::learn::init_learn;
use c2rust_out::src::game::{global_setup, compute_move, game_init};
use c2rust_out::src::stubs::strstr;
use c2rust_out::src::display::{display_move, display_board};
use engine::src::zebra::EvaluationType;
use engine::src::error::LibcFatalError;

extern "C" {
    #[no_mangle]
    static mut stdout: *mut FILE;
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
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
                  -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type time_t = __time_t;

/* The basic board type. One index for each position;
   a1=11, h1=18, a8=81, h8=88. */
pub type Board = [libc::c_int; 128];
/*
   File:          search.h

   Created:       July 1, 1997

   Modified:      August 1, 2002

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The interface to common search routines and variables.
*/
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
   File:           zebra.c

   Created:        June 5, 1997

   Modified:       December 25, 2005

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       The module which controls the operation of standalone Zebra.
*/
/* Get rid of some ugly warnings by disallowing usage of the
   macro version of tolower (not time-critical anyway). */
/* Local variables */
/* ------------------- Function prototypes ---------------------- */
/* Administrative routines */
unsafe extern "C" fn run_endgame_script(mut in_file_name: *const libc::c_char,
                                        mut out_file_name:
                                        *const libc::c_char,
                                        mut display_line: libc::c_int) {
    let mut script_nodes: CounterType = CounterType{hi: 0, lo: 0,};
    let mut eval_info: EvaluationType =
        EvaluationType{type_0: MIDGAME_EVAL,
            res: WON_POSITION,
            score: 0,
            confidence: 0.,
            search_depth: 0,
            is_book: 0,};
    let mut comment: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer: [libc::c_char; 256] = [0; 256];
    let mut board_string: [libc::c_char; 256] = [0; 256];
    let mut stm_string: [libc::c_char; 256] = [0; 256];
    let mut start_time: libc::c_double = 0.;
    let mut stop_time: libc::c_double = 0.;
    let mut search_start: libc::c_double = 0.;
    let mut search_stop: libc::c_double = 0.;
    let mut max_search: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut book: libc::c_int = 0;
    let mut mid: libc::c_int = 0;
    let mut exact: libc::c_int = 0;
    let mut wld: libc::c_int = 0;
    let mut my_time: libc::c_int = 0;
    let mut my_incr: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut move_0: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut timed_search: libc::c_int = 0;
    let mut scanned: libc::c_int = 0;
    let mut token: libc::c_int = 0;
    let mut position_count: libc::c_int = 0;
    let mut script_stream: *mut FILE = 0 as *mut FILE;
    let mut output_stream: *mut FILE = 0 as *mut FILE;
    /* Open the files and get the number of positions */
    script_stream =
        fopen(in_file_name, b"r\x00" as *const u8 as *const libc::c_char);
    if script_stream.is_null() {
        printf(b"\nCan\'t open script file \'%s\' - aborting\n\n\x00" as
                   *const u8 as *const libc::c_char, in_file_name);
        exit(1 as libc::c_int);
    }
    output_stream =
        fopen(out_file_name, b"w\x00" as *const u8 as *const libc::c_char);
    if output_stream.is_null() {
        printf(b"\nCan\'t create output file \'%s\' - aborting\n\n\x00" as
                   *const u8 as *const libc::c_char, out_file_name);
        exit(1 as libc::c_int);
    }
    fclose(output_stream);
    /* Initialize display subsystem and search parameters */
    set_names(b"\x00" as *const u8 as *const libc::c_char,
              b"\x00" as *const u8 as *const libc::c_char);
    set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                  score_sheet_row);
    set_evals(0.0f64, 0.0f64);
    i = 0 as libc::c_int;
    while i < 60 as libc::c_int {
        black_moves[i as usize] = -(1 as libc::c_int);
        white_moves[i as usize] = -(1 as libc::c_int);
        i += 1
    }
    my_time = 100000000 as libc::c_int;
    my_incr = 0 as libc::c_int;
    timed_search = 0 as libc::c_int;
    book = use_book;
    mid = 60 as libc::c_int;
    if wld_only != 0 {
        exact = 0 as libc::c_int
    } else { exact = 60 as libc::c_int }
    wld = 60 as libc::c_int;
    toggle_status_log(0 as libc::c_int);
    reset_counter(&mut script_nodes);
    position_count = 0 as libc::c_int;
    max_search = -0.0f64;
    start_time = get_real_timer();
    /* Scan through the script file */
    i = 0 as libc::c_int;
    loop  {
        let mut pass_count: libc::c_int = 0 as libc::c_int;
        /* Check if the line is a comment or an end marker */
        fgets(buffer.as_mut_ptr(), 256 as libc::c_int, script_stream);
        if feof(script_stream) != 0 { break ; }
        if buffer[0 as libc::c_int as usize] as libc::c_int == '%' as i32 {
            /* Comment */
            output_stream =
                fopen(out_file_name,
                      b"a\x00" as *const u8 as *const libc::c_char);
            if output_stream.is_null() {
                printf(b"\nCan\'t append to output file \'%s\' - aborting\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       out_file_name);
                exit(1 as libc::c_int);
            }
            fputs(buffer.as_mut_ptr(), output_stream);
            fclose(output_stream);
            if strstr(buffer.as_mut_ptr(),
                      b"% End of the endgame script\x00" as *const u8 as
                          *const libc::c_char) == buffer.as_mut_ptr() {
                break ;
            }
        } else {
            if feof(script_stream) != 0 {
                printf(b"\nEOF encountered when reading position #%d - aborting\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       i + 1 as libc::c_int);
                exit(1 as libc::c_int);
            }
            /* Parse the script line containing board and side to move */
            game_init(0 as *const libc::c_char, &mut side_to_move);
            set_slack(0.0f64 as libc::c_int);
            toggle_human_openings(0 as libc::c_int);
            reset_book_search();
            set_deviation_value(0 as libc::c_int, 60 as libc::c_int, 0.0f64);
            setup_hash(1 as libc::c_int);
            position_count += 1;
            scanned =
                sscanf(buffer.as_mut_ptr(),
                       b"%s %s\x00" as *const u8 as *const libc::c_char,
                       board_string.as_mut_ptr(), stm_string.as_mut_ptr());
            if scanned != 2 as libc::c_int {
                printf(b"\nError parsing line %d - aborting\n\n\x00" as
                           *const u8 as *const libc::c_char,
                       i + 1 as libc::c_int);
                exit(1 as libc::c_int);
            }
            if strlen(stm_string.as_mut_ptr()) !=
                1 as libc::c_int as libc::c_ulong {
                printf(b"\nAmbiguous side to move on line %d - aborting\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       i + 1 as libc::c_int);
                exit(1 as libc::c_int);
            }
            match stm_string[0 as libc::c_int as usize] as libc::c_int {
                79 | 48 => { side_to_move = 2 as libc::c_int }
                42 | 88 => { side_to_move = 0 as libc::c_int }
                _ => {
                    printf(b"\nBad side-to-move indicator on line %d - aborting\n\n\x00"
                               as *const u8 as *const libc::c_char,
                           i + 1 as libc::c_int);
                }
            }
            if strlen(board_string.as_mut_ptr()) !=
                64 as libc::c_int as libc::c_ulong {
                printf(b"\nBoard on line %d doesn\'t contain 64 positions - aborting\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       i + 1 as libc::c_int);
                exit(1 as libc::c_int);
            }
            token = 0 as libc::c_int;
            row = 1 as libc::c_int;
            while row <= 8 as libc::c_int {
                col = 1 as libc::c_int;
                while col <= 8 as libc::c_int {
                    pos = 10 as libc::c_int * row + col;
                    match board_string[token as usize] as libc::c_int {
                        42 | 88 | 120 => {
                            board[pos as usize] = 0 as libc::c_int
                        }
                        79 | 48 | 111 => {
                            board[pos as usize] = 2 as libc::c_int
                        }
                        45 | 46 => { board[pos as usize] = 1 as libc::c_int }
                        _ => {
                            printf(b"\nBad character \'%c\' in board on line %d - aborting\n\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   board_string[token as usize] as
                                       libc::c_int, i + 1 as libc::c_int);
                        }
                    }
                    token += 1;
                    col += 1
                }
                row += 1
            }
            disks_played =
                disc_count(0 as libc::c_int) + disc_count(2 as libc::c_int) -
                    4 as libc::c_int;
            /* Search the position */
            if echo != 0 {
                set_move_list(black_moves.as_mut_ptr(),
                              white_moves.as_mut_ptr(), score_sheet_row);
                display_board(stdout, board.as_mut_ptr(), side_to_move,
                              1 as libc::c_int, 0 as libc::c_int,
                              1 as libc::c_int);
            }
            search_start = get_real_timer();
            start_move(my_time as libc::c_double, my_incr as libc::c_double,
                       disks_played + 4 as libc::c_int);
            determine_move_time(my_time as libc::c_double,
                                my_incr as libc::c_double,
                                disks_played + 4 as libc::c_int);
            pass_count = 0 as libc::c_int;
            move_0 =
                compute_move(side_to_move, 1 as libc::c_int, my_time, my_incr,
                             timed_search, book, mid, exact, wld,
                             1 as libc::c_int, &mut eval_info);
            if move_0 == -(1 as libc::c_int) {
                pass_count += 1;
                side_to_move =
                    0 as libc::c_int + 2 as libc::c_int - side_to_move;
                move_0 =
                    compute_move(side_to_move, 1 as libc::c_int, my_time,
                                 my_incr, timed_search, book, mid, exact, wld,
                                 1 as libc::c_int, &mut eval_info);
                if move_0 == -(1 as libc::c_int) {
                    /* Both pass, game over. */
                    let mut my_discs: libc::c_int = disc_count(side_to_move);
                    let mut opp_discs: libc::c_int =
                        disc_count(0 as libc::c_int + 2 as libc::c_int -
                            side_to_move);
                    if my_discs > opp_discs {
                        my_discs = 64 as libc::c_int - opp_discs
                    } else if opp_discs > my_discs {
                        opp_discs = 64 as libc::c_int - my_discs
                    } else {
                        opp_discs = 32 as libc::c_int;
                        my_discs = opp_discs
                    }
                    eval_info.score =
                        128 as libc::c_int * (my_discs - opp_discs);
                    pass_count += 1
                }
            }
            score = eval_info.score / 128 as libc::c_int;
            search_stop = get_real_timer();
            if search_stop - search_start > max_search {
                max_search = search_stop - search_start
            }
            add_counter(&mut script_nodes, &mut nodes);
            output_stream =
                fopen(out_file_name,
                      b"a\x00" as *const u8 as *const libc::c_char);
            if output_stream.is_null() {
                printf(b"\nCan\'t append to output file \'%s\' - aborting\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       out_file_name);
                exit(1 as libc::c_int);
            }
            if wld_only != 0 {
                if side_to_move == 0 as libc::c_int {
                    if score > 0 as libc::c_int {
                        fputs(b"Black win\x00" as *const u8 as
                                  *const libc::c_char, output_stream);
                    } else if score == 0 as libc::c_int {
                        fputs(b"Draw\x00" as *const u8 as *const libc::c_char,
                              output_stream);
                    } else {
                        fputs(b"White win\x00" as *const u8 as
                                  *const libc::c_char, output_stream);
                    }
                } else if score > 0 as libc::c_int {
                    fputs(b"White win\x00" as *const u8 as
                              *const libc::c_char, output_stream);
                } else if score == 0 as libc::c_int {
                    fputs(b"Draw\x00" as *const u8 as *const libc::c_char,
                          output_stream);
                } else {
                    fputs(b"Black win\x00" as *const u8 as
                              *const libc::c_char, output_stream);
                }
            } else if side_to_move == 0 as libc::c_int {
                fprintf(output_stream,
                        b"%2d - %2d\x00" as *const u8 as *const libc::c_char,
                        32 as libc::c_int + score / 2 as libc::c_int,
                        32 as libc::c_int - score / 2 as libc::c_int);
            } else {
                fprintf(output_stream,
                        b"%2d - %2d\x00" as *const u8 as *const libc::c_char,
                        32 as libc::c_int - score / 2 as libc::c_int,
                        32 as libc::c_int + score / 2 as libc::c_int);
            }
            if display_line != 0 && pass_count != 2 as libc::c_int {
                fputs(b"   \x00" as *const u8 as *const libc::c_char,
                      output_stream);
                if pass_count == 1 as libc::c_int {
                    fputs(b" --\x00" as *const u8 as *const libc::c_char,
                          output_stream);
                }
                j = 0 as libc::c_int;
                while j < full_pv_depth {
                    fputs(b" \x00" as *const u8 as *const libc::c_char,
                          output_stream);
                    display_move(output_stream, full_pv[j as usize]);
                    j += 1
                }
            }
            comment =
                strstr(buffer.as_mut_ptr(),
                       b"%\x00" as *const u8 as *const libc::c_char);
            if !comment.is_null() {
                /* Copy comment to output file */
                fprintf(output_stream,
                        b"      %s\x00" as *const u8 as *const libc::c_char,
                        comment);
            } else {
                fputs(b"\n\x00" as *const u8 as *const libc::c_char,
                      output_stream);
            }
            fclose(output_stream);
            if echo != 0 {
                puts(b"\n\n\n\x00" as *const u8 as *const libc::c_char);
            }
        }
        i += 1
    }
    /* Clean up and terminate */
    fclose(script_stream);
    stop_time = get_real_timer();
    printf(b"Total positions solved:   %d\n\x00" as *const u8 as
               *const libc::c_char, position_count);
    printf(b"Total time:               %.1f s\n\x00" as *const u8 as
               *const libc::c_char, stop_time - start_time);
    printf(b"Total nodes:              %.0f\n\x00" as *const u8 as
               *const libc::c_char, counter_value(&mut script_nodes));
    puts(b"\x00" as *const u8 as *const libc::c_char);
    printf(b"Average time for solve:   %.1f s\n\x00" as *const u8 as
               *const libc::c_char,
           (stop_time - start_time) / position_count as libc::c_double);
    printf(b"Maximum time for solve:   %.1f s\n\x00" as *const u8 as
               *const libc::c_char, max_search);
    puts(b"\x00" as *const u8 as *const libc::c_char);
    printf(b"Average speed:            %.0f nps\n\x00" as *const u8 as
               *const libc::c_char,
           counter_value(&mut script_nodes) / (stop_time - start_time));
    puts(b"\x00" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
                 -> libc::c_int {
    let mut game_file_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut script_in_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut script_out_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut arg_index: libc::c_int = 0;
    let mut help: libc::c_int = 0;
    let mut hash_bits: libc::c_int = 0;
    let mut use_random: libc::c_int = 0;
    let mut run_script: libc::c_int = 0;
    let mut script_optimal_line: libc::c_int = 0 as libc::c_int;
    let mut komi: libc::c_int = 0;
    let mut timer: time_t = 0;
    printf(b"\nscrZebra (c) 1997-2005 Gunnar Andersson, compile date %s at %s\n\n\x00"
               as *const u8 as *const libc::c_char,
           b"Aug  9 2020\x00" as *const u8 as *const libc::c_char,
           b"20:20:01\x00" as *const u8 as *const libc::c_char);
    use_random = 1 as libc::c_int;
    wait = 0 as libc::c_int;
    echo = 1 as libc::c_int;
    display_pv = 1 as libc::c_int;
    use_learning = 0 as libc::c_int;
    use_thor = 0 as libc::c_int;
    skill[2 as libc::c_int as usize] = -(1 as libc::c_int);
    skill[0 as libc::c_int as usize] = skill[2 as libc::c_int as usize];
    hash_bits = 18 as libc::c_int;
    game_file_name = 0 as *const libc::c_char;
    log_file_name = 0 as *mut libc::c_char;
    run_script = 0 as libc::c_int;
    script_out_file = 0 as *const libc::c_char;
    script_in_file = script_out_file;
    komi = 0 as libc::c_int;
    player_time[2 as libc::c_int as usize] = 10000000.0f64;
    player_time[0 as libc::c_int as usize] =
        player_time[2 as libc::c_int as usize];
    player_increment[2 as libc::c_int as usize] = 0.0f64;
    player_increment[0 as libc::c_int as usize] =
        player_increment[2 as libc::c_int as usize];
    let mut current_block_37: u64;
    arg_index = 1 as libc::c_int;
    help = 0 as libc::c_int;
    while arg_index < argc && help == 0 {
        if strcasecmp(*argv.offset(arg_index as isize),
                      b"-e\x00" as *const u8 as *const libc::c_char) == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_37 = 4808432441040389987;
            } else {
                echo = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-h\x00" as *const u8 as *const libc::c_char) ==
            0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_37 = 4808432441040389987;
            } else {
                hash_bits = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-b\x00" as *const u8 as *const libc::c_char) ==
            0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_37 = 4808432441040389987;
            } else {
                use_book = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-wld\x00" as *const u8 as *const libc::c_char)
            == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_37 = 4808432441040389987;
            } else {
                wld_only = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-line\x00" as *const u8 as *const libc::c_char)
            == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_37 = 4808432441040389987;
            } else {
                script_optimal_line = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-script\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if arg_index + 2 as libc::c_int >= argc {
                help = 1 as libc::c_int;
                current_block_37 = 4808432441040389987;
            } else {
                arg_index += 1;
                script_in_file = *argv.offset(arg_index as isize);
                arg_index += 1;
                script_out_file = *argv.offset(arg_index as isize);
                run_script = 1 as libc::c_int;
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-komi\x00" as *const u8 as *const libc::c_char)
            == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_37 = 4808432441040389987;
            } else {
                komi = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else {
            help = 1 as libc::c_int;
            current_block_37 = 13303144130133872306;
        }
        match current_block_37 {
            13303144130133872306 => {
                if arg_index >= argc { help = 1 as libc::c_int }
            }
            _ => { }
        }
        arg_index += 1
    }
    if run_script == 0 { help = 1 as libc::c_int }
    if komi != 0 as libc::c_int {
        if wld_only == 0 {
            puts(b"Komi can only be applied to WLD solves.\x00" as *const u8
                as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        set_komi(komi);
    }
    if help != 0 {
        puts(b"Usage:\x00" as *const u8 as *const libc::c_char);
        puts(b"  scrzebra [-e ...] [-h ...] [-wld ...] [-line ...] [-b ...] [-komi ...] -script ...\x00"
            as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -e <echo?>\x00" as *const u8 as *const libc::c_char);
        printf(b"    Toggles screen output on/off (default %d).\n\n\x00" as
                   *const u8 as *const libc::c_char, 1 as libc::c_int);
        puts(b"  -h <bits in hash key>\x00" as *const u8 as
            *const libc::c_char);
        printf(b"    Size of hash table is 2^{this value} (default %d).\n\n\x00"
                   as *const u8 as *const libc::c_char, 18 as libc::c_int);
        puts(b"  -script <script file> <output file>\x00" as *const u8 as
            *const libc::c_char);
        puts(b"    Solves all positions in script file for exact score.\n\x00"
            as *const u8 as *const libc::c_char);
        puts(b"  -wld <only solve WLD?>\x00" as *const u8 as
            *const libc::c_char);
        printf(b"    Toggles WLD only solve on/off (default %d).\n\n\x00" as
                   *const u8 as *const libc::c_char, 0 as libc::c_int);
        puts(b"  -line <output line?>\x00" as *const u8 as
            *const libc::c_char);
        printf(b"    Toggles output of optimal line on/off (default %d).\n\n\x00"
                   as *const u8 as *const libc::c_char, 0 as libc::c_int);
        puts(b"  -b <use book?>\x00" as *const u8 as *const libc::c_char);
        printf(b"    Toggles usage of opening book on/off (default %d).\n\x00"
                   as *const u8 as *const libc::c_char, 0 as libc::c_int);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -komi <komi>\x00" as *const u8 as *const libc::c_char);
        puts(b"    Number of discs that white has to win with (only WLD).\x00"
            as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if hash_bits < 1 as libc::c_int {
        printf(b"Hash table key must contain at least 1 bit\n\x00" as
            *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    global_setup(use_random, hash_bits);
    init_thor_database::<LibcFatalError>();
    if use_book != 0 {
        init_learn(b"book.bin\x00" as *const u8 as *const libc::c_char,
                   1 as libc::c_int);
    }
    if use_random != 0 && 1 as libc::c_int == 0 {
        time(&mut timer);
        my_srandom(timer as libc::c_int);
    } else { my_srandom(1 as libc::c_int); }
    if run_script != 0 {
        run_endgame_script(script_in_file, script_out_file,
                           script_optimal_line);
    }
    global_terminate();
    return 0 as libc::c_int;
}
static mut use_thor: libc::c_int = 0;
static mut use_learning: libc::c_int = 0;
static mut wld_only: libc::c_int = 0 as libc::c_int;
static mut use_book: libc::c_int = 0 as libc::c_int;
static mut wait: libc::c_int = 0;
static mut skill: [libc::c_int; 3] = [0; 3];
static mut player_increment: [libc::c_double; 3] = [0.; 3];
static mut player_time: [libc::c_double; 3] = [0.; 3];
static mut log_file_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;

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
