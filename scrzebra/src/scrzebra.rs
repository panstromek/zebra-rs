#![allow(dead_code,  non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, extern_types)]

use engine::src::game::{global_terminate, set_komi, toggle_human_openings, toggle_status_log};
use engine::src::myrandom::my_srandom;
use engine::src::thordb::init_thor_database;
use engine::src::display::{echo, display_pv, set_move_list, set_evals, set_names, current_row, black_player, black_time, black_eval, white_eval, white_time, white_player};
use engine::src::counter::{counter_value, add_counter, reset_counter, CounterType};
use engine::src::timer::{get_real_timer, determine_move_time, start_move};
use engine::src::search::{full_pv, full_pv_depth, nodes, disc_count};
use engine::src::moves::disks_played;
use engine::src::globals::{board, score_sheet_row, white_moves, black_moves};
use engine::src::hash::setup_hash;
use engine::src::osfbook::{set_deviation_value, reset_book_search, set_slack};
use c2rust_out::src::learn::init_learn;
use c2rust_out::src::game::{global_setup, compute_move, game_init};
use c2rust_out::src::display::{display_move, display_board};
use engine::src::zebra::EvaluationType;
use c2rust_out::src::error::{LibcFatalError, FE};
use engine::src::error::FrontEnd;
use libc_wrapper::{strstr, FILE};

extern "C" {
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> i32;
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
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8,
              __base: i32) -> i64;
    #[no_mangle]
    fn exit(_: i32) -> !;
    #[no_mangle]
    fn strcasecmp(_: *const i8, _: *const i8)
                  -> i32;
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type size_t = u64;
pub type _IO_lock_t = ();
pub type time_t = __time_t;

/* The basic board type. One index for each position;
   a1=11, h1=18, a8=81, h8=88. */
pub type Board = [i32; 128];
/*
   File:          search.h

   Created:       July 1, 1997

   Modified:      August 1, 2002

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The interface to common search routines and variables.
*/
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

#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut ::std::ffi::c_void as *mut *mut i8,
                  10 as i32) as i32;
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
unsafe extern "C" fn run_endgame_script(mut in_file_name: *const i8,
                                        mut out_file_name:
                                        *const i8,
                                        mut display_line: i32) {
    let mut script_nodes: CounterType = CounterType{hi: 0, lo: 0,};
    let mut eval_info: EvaluationType =
        EvaluationType{type_0: MIDGAME_EVAL,
            res: WON_POSITION,
            score: 0,
            confidence: 0.,
            search_depth: 0,
            is_book: 0,};
    let mut comment: *mut i8 = 0 as *mut i8;
    let mut buffer: [i8; 256] = [0; 256];
    let mut board_string: [i8; 256] = [0; 256];
    let mut stm_string: [i8; 256] = [0; 256];
    let mut start_time: f64 = 0.;
    let mut stop_time: f64 = 0.;
    let mut search_start: f64 = 0.;
    let mut search_stop: f64 = 0.;
    let mut max_search: f64 = 0.;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut pos: i32 = 0;
    let mut book: i32 = 0;
    let mut mid: i32 = 0;
    let mut exact: i32 = 0;
    let mut wld: i32 = 0;
    let mut my_time: i32 = 0;
    let mut my_incr: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut move_0: i32 = 0;
    let mut score: i32 = 0;
    let mut timed_search: i32 = 0;
    let mut scanned: i32 = 0;
    let mut token: i32 = 0;
    let mut position_count: i32 = 0;
    let mut script_stream: *mut FILE = 0 as *mut FILE;
    let mut output_stream: *mut FILE = 0 as *mut FILE;
    /* Open the files and get the number of positions */
    script_stream =
        fopen(in_file_name, b"r\x00" as *const u8 as *const i8);
    if script_stream.is_null() {
        printf(b"\nCan\'t open script file \'%s\' - aborting\n\n\x00" as
                   *const u8 as *const i8, in_file_name);
        exit(1 as i32);
    }
    output_stream =
        fopen(out_file_name, b"w\x00" as *const u8 as *const i8);
    if output_stream.is_null() {
        printf(b"\nCan\'t create output file \'%s\' - aborting\n\n\x00" as
                   *const u8 as *const i8, out_file_name);
        exit(1 as i32);
    }
    fclose(output_stream);
    /* Initialize display subsystem and search parameters */
    set_names::<FE>(b"\x00" as *const u8 as *const i8,
              b"\x00" as *const u8 as *const i8);
    set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                  score_sheet_row);
    set_evals(0.0f64, 0.0f64);
    i = 0 as i32;
    while i < 60 as i32 {
        black_moves[i as usize] = -(1 as i32);
        white_moves[i as usize] = -(1 as i32);
        i += 1
    }
    my_time = 100000000 as i32;
    my_incr = 0 as i32;
    timed_search = 0 as i32;
    book = use_book;
    mid = 60 as i32;
    if wld_only != 0 {
        exact = 0 as i32
    } else { exact = 60 as i32 }
    wld = 60 as i32;
    toggle_status_log(0 as i32);
    reset_counter(&mut script_nodes);
    position_count = 0 as i32;
    max_search = -0.0f64;
    start_time = get_real_timer::<FE>();
    /* Scan through the script file */
    i = 0 as i32;
    loop  {
        let mut pass_count: i32 = 0 as i32;
        /* Check if the line is a comment or an end marker */
        fgets(buffer.as_mut_ptr(), 256 as i32, script_stream);
        if feof(script_stream) != 0 { break ; }
        if buffer[0 as i32 as usize] as i32 == '%' as i32 {
            /* Comment */
            output_stream =
                fopen(out_file_name,
                      b"a\x00" as *const u8 as *const i8);
            if output_stream.is_null() {
                printf(b"\nCan\'t append to output file \'%s\' - aborting\n\n\x00"
                           as *const u8 as *const i8,
                       out_file_name);
                exit(1 as i32);
            }
            fputs(buffer.as_mut_ptr(), output_stream);
            fclose(output_stream);
            if strstr(buffer.as_mut_ptr(),
                      b"% End of the endgame script\x00" as *const u8 as
                          *const i8) == buffer.as_mut_ptr() {
                break ;
            }
        } else {
            if feof(script_stream) != 0 {
                printf(b"\nEOF encountered when reading position #%d - aborting\n\n\x00"
                           as *const u8 as *const i8,
                       i + 1 as i32);
                exit(1 as i32);
            }
            /* Parse the script line containing board and side to move */
            game_init(0 as *const i8, &mut side_to_move);
            set_slack(0.0f64 as i32);
            toggle_human_openings(0 as i32);
            reset_book_search();
            set_deviation_value(0 as i32, 60 as i32, 0.0f64);
            setup_hash(1 as i32);
            position_count += 1;
            scanned =
                sscanf(buffer.as_mut_ptr(),
                       b"%s %s\x00" as *const u8 as *const i8,
                       board_string.as_mut_ptr(), stm_string.as_mut_ptr());
            if scanned != 2 as i32 {
                printf(b"\nError parsing line %d - aborting\n\n\x00" as
                           *const u8 as *const i8,
                       i + 1 as i32);
                exit(1 as i32);
            }
            if   FE::strlen(stm_string.as_mut_ptr()) !=
                1 as i32 as u64 {
                printf(b"\nAmbiguous side to move on line %d - aborting\n\n\x00"
                           as *const u8 as *const i8,
                       i + 1 as i32);
                exit(1 as i32);
            }
            match stm_string[0 as i32 as usize] as i32 {
                79 | 48 => { side_to_move = 2 as i32 }
                42 | 88 => { side_to_move = 0 as i32 }
                _ => {
                    printf(b"\nBad side-to-move indicator on line %d - aborting\n\n\x00"
                               as *const u8 as *const i8,
                           i + 1 as i32);
                }
            }
            if   FE::strlen(board_string.as_mut_ptr()) !=
                64 as i32 as u64 {
                printf(b"\nBoard on line %d doesn\'t contain 64 positions - aborting\n\n\x00"
                           as *const u8 as *const i8,
                       i + 1 as i32);
                exit(1 as i32);
            }
            token = 0 as i32;
            row = 1 as i32;
            while row <= 8 as i32 {
                col = 1 as i32;
                while col <= 8 as i32 {
                    pos = 10 as i32 * row + col;
                    match board_string[token as usize] as i32 {
                        42 | 88 | 120 => {
                            board[pos as usize] = 0 as i32
                        }
                        79 | 48 | 111 => {
                            board[pos as usize] = 2 as i32
                        }
                        45 | 46 => { board[pos as usize] = 1 as i32 }
                        _ => {
                            printf(b"\nBad character \'%c\' in board on line %d - aborting\n\n\x00"
                                       as *const u8 as *const i8,
                                   board_string[token as usize] as
                                       i32, i + 1 as i32);
                        }
                    }
                    token += 1;
                    col += 1
                }
                row += 1
            }
            disks_played =
                disc_count(0 as i32) + disc_count(2 as i32) -
                    4 as i32;
            /* Search the position */
            if echo != 0 {
                set_move_list(black_moves.as_mut_ptr(),
                              white_moves.as_mut_ptr(), score_sheet_row);
                display_board(stdout, board.as_mut_ptr(), side_to_move,
                              1 as i32, 0 as i32,
                              1 as i32, current_row,
                              black_player, black_time, black_eval,
                              white_player, white_time, white_eval,
                              &black_moves, &white_moves);
            }
            search_start = get_real_timer::<FE>();
            start_move::<FE>(my_time as f64, my_incr as f64,
                       disks_played + 4 as i32);
            determine_move_time(my_time as f64,
                                my_incr as f64,
                                disks_played + 4 as i32);
            pass_count = 0 as i32;
            move_0 =
                compute_move(side_to_move, 1 as i32, my_time, my_incr,
                             timed_search, book, mid, exact, wld,
                             1 as i32, &mut eval_info);
            if move_0 == -(1 as i32) {
                pass_count += 1;
                side_to_move =
                    0 as i32 + 2 as i32 - side_to_move;
                move_0 =
                    compute_move(side_to_move, 1 as i32, my_time,
                                 my_incr, timed_search, book, mid, exact, wld,
                                 1 as i32, &mut eval_info);
                if move_0 == -(1 as i32) {
                    /* Both pass, game over. */
                    let mut my_discs: i32 = disc_count(side_to_move);
                    let mut opp_discs: i32 =
                        disc_count(0 as i32 + 2 as i32 -
                            side_to_move);
                    if my_discs > opp_discs {
                        my_discs = 64 as i32 - opp_discs
                    } else if opp_discs > my_discs {
                        opp_discs = 64 as i32 - my_discs
                    } else {
                        opp_discs = 32 as i32;
                        my_discs = opp_discs
                    }
                    eval_info.score =
                        128 as i32 * (my_discs - opp_discs);
                    pass_count += 1
                }
            }
            score = eval_info.score / 128 as i32;
            search_stop = get_real_timer::<FE>();
            if search_stop - search_start > max_search {
                max_search = search_stop - search_start
            }
            add_counter(&mut script_nodes, &mut nodes);
            output_stream =
                fopen(out_file_name,
                      b"a\x00" as *const u8 as *const i8);
            if output_stream.is_null() {
                printf(b"\nCan\'t append to output file \'%s\' - aborting\n\n\x00"
                           as *const u8 as *const i8,
                       out_file_name);
                exit(1 as i32);
            }
            if wld_only != 0 {
                if side_to_move == 0 as i32 {
                    if score > 0 as i32 {
                        fputs(b"Black win\x00" as *const u8 as
                                  *const i8, output_stream);
                    } else if score == 0 as i32 {
                        fputs(b"Draw\x00" as *const u8 as *const i8,
                              output_stream);
                    } else {
                        fputs(b"White win\x00" as *const u8 as
                                  *const i8, output_stream);
                    }
                } else if score > 0 as i32 {
                    fputs(b"White win\x00" as *const u8 as
                              *const i8, output_stream);
                } else if score == 0 as i32 {
                    fputs(b"Draw\x00" as *const u8 as *const i8,
                          output_stream);
                } else {
                    fputs(b"Black win\x00" as *const u8 as
                              *const i8, output_stream);
                }
            } else if side_to_move == 0 as i32 {
                fprintf(output_stream,
                        b"%2d - %2d\x00" as *const u8 as *const i8,
                        32 as i32 + score / 2 as i32,
                        32 as i32 - score / 2 as i32);
            } else {
                fprintf(output_stream,
                        b"%2d - %2d\x00" as *const u8 as *const i8,
                        32 as i32 - score / 2 as i32,
                        32 as i32 + score / 2 as i32);
            }
            if display_line != 0 && pass_count != 2 as i32 {
                fputs(b"   \x00" as *const u8 as *const i8,
                      output_stream);
                if pass_count == 1 as i32 {
                    fputs(b" --\x00" as *const u8 as *const i8,
                          output_stream);
                }
                j = 0 as i32;
                while j < full_pv_depth {
                    fputs(b" \x00" as *const u8 as *const i8,
                          output_stream);
                    display_move(output_stream, full_pv[j as usize]);
                    j += 1
                }
            }
            comment =
                strstr(buffer.as_mut_ptr(),
                       b"%\x00" as *const u8 as *const i8);
            if !comment.is_null() {
                /* Copy comment to output file */
                fprintf(output_stream,
                        b"      %s\x00" as *const u8 as *const i8,
                        comment);
            } else {
                fputs(b"\n\x00" as *const u8 as *const i8,
                      output_stream);
            }
            fclose(output_stream);
            if echo != 0 {
                puts(b"\n\n\n\x00" as *const u8 as *const i8);
            }
        }
        i += 1
    }
    /* Clean up and terminate */
    fclose(script_stream);
    stop_time = get_real_timer::<FE>();
    printf(b"Total positions solved:   %d\n\x00" as *const u8 as
               *const i8, position_count);
    printf(b"Total time:               %.1f s\n\x00" as *const u8 as
               *const i8, stop_time - start_time);
    printf(b"Total nodes:              %.0f\n\x00" as *const u8 as
               *const i8, counter_value(&mut script_nodes));
    puts(b"\x00" as *const u8 as *const i8);
    printf(b"Average time for solve:   %.1f s\n\x00" as *const u8 as
               *const i8,
           (stop_time - start_time) / position_count as f64);
    printf(b"Maximum time for solve:   %.1f s\n\x00" as *const u8 as
               *const i8, max_search);
    puts(b"\x00" as *const u8 as *const i8);
    printf(b"Average speed:            %.0f nps\n\x00" as *const u8 as
               *const i8,
           counter_value(&mut script_nodes) / (stop_time - start_time));
    puts(b"\x00" as *const u8 as *const i8);
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8)
                 -> i32 {
    let mut game_file_name: *const i8 = 0 as *const i8;
    let mut script_in_file: *const i8 = 0 as *const i8;
    let mut script_out_file: *const i8 = 0 as *const i8;
    let mut arg_index: i32 = 0;
    let mut help: i32 = 0;
    let mut hash_bits: i32 = 0;
    let mut use_random: i32 = 0;
    let mut run_script: i32 = 0;
    let mut script_optimal_line: i32 = 0 as i32;
    let mut komi: i32 = 0;
    let mut timer: time_t = 0;
    printf(b"\nscrZebra (c) 1997-2005 Gunnar Andersson, compile date %s at %s\n\n\x00"
               as *const u8 as *const i8,
           b"Aug  9 2020\x00" as *const u8 as *const i8,
           b"20:20:01\x00" as *const u8 as *const i8);
    use_random = 1 as i32;
    wait = 0 as i32;
    echo = 1 as i32;
    display_pv = 1 as i32;
    use_learning = 0 as i32;
    use_thor = 0 as i32;
    skill[2 as i32 as usize] = -(1 as i32);
    skill[0 as i32 as usize] = skill[2 as i32 as usize];
    hash_bits = 18 as i32;
    game_file_name = 0 as *const i8;
    log_file_name = 0 as *mut i8;
    run_script = 0 as i32;
    script_out_file = 0 as *const i8;
    script_in_file = script_out_file;
    komi = 0 as i32;
    player_time[2 as i32 as usize] = 10000000.0f64;
    player_time[0 as i32 as usize] =
        player_time[2 as i32 as usize];
    player_increment[2 as i32 as usize] = 0.0f64;
    player_increment[0 as i32 as usize] =
        player_increment[2 as i32 as usize];
    let mut current_block_37: u64;
    arg_index = 1 as i32;
    help = 0 as i32;
    while arg_index < argc && help == 0 {
        if strcasecmp(*argv.offset(arg_index as isize),
                      b"-e\x00" as *const u8 as *const i8) == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as i32;
                current_block_37 = 4808432441040389987;
            } else {
                echo = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-h\x00" as *const u8 as *const i8) ==
            0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as i32;
                current_block_37 = 4808432441040389987;
            } else {
                hash_bits = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-b\x00" as *const u8 as *const i8) ==
            0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as i32;
                current_block_37 = 4808432441040389987;
            } else {
                use_book = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-wld\x00" as *const u8 as *const i8)
            == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as i32;
                current_block_37 = 4808432441040389987;
            } else {
                wld_only = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-line\x00" as *const u8 as *const i8)
            == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as i32;
                current_block_37 = 4808432441040389987;
            } else {
                script_optimal_line = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-script\x00" as *const u8 as
                                 *const i8) == 0 {
            if arg_index + 2 as i32 >= argc {
                help = 1 as i32;
                current_block_37 = 4808432441040389987;
            } else {
                arg_index += 1;
                script_in_file = *argv.offset(arg_index as isize);
                arg_index += 1;
                script_out_file = *argv.offset(arg_index as isize);
                run_script = 1 as i32;
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-komi\x00" as *const u8 as *const i8)
            == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as i32;
                current_block_37 = 4808432441040389987;
            } else {
                komi = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else {
            help = 1 as i32;
            current_block_37 = 13303144130133872306;
        }
        match current_block_37 {
            13303144130133872306 => {
                if arg_index >= argc { help = 1 as i32 }
            }
            _ => { }
        }
        arg_index += 1
    }
    if run_script == 0 { help = 1 as i32 }
    if komi != 0 as i32 {
        if wld_only == 0 {
            puts(b"Komi can only be applied to WLD solves.\x00" as *const u8
                as *const i8);
            exit(1 as i32);
        }
        set_komi(komi);
    }
    if help != 0 {
        puts(b"Usage:\x00" as *const u8 as *const i8);
        puts(b"  scrzebra [-e ...] [-h ...] [-wld ...] [-line ...] [-b ...] [-komi ...] -script ...\x00"
            as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -e <echo?>\x00" as *const u8 as *const i8);
        printf(b"    Toggles screen output on/off (default %d).\n\n\x00" as
                   *const u8 as *const i8, 1 as i32);
        puts(b"  -h <bits in hash key>\x00" as *const u8 as
            *const i8);
        printf(b"    Size of hash table is 2^{this value} (default %d).\n\n\x00"
                   as *const u8 as *const i8, 18 as i32);
        puts(b"  -script <script file> <output file>\x00" as *const u8 as
            *const i8);
        puts(b"    Solves all positions in script file for exact score.\n\x00"
            as *const u8 as *const i8);
        puts(b"  -wld <only solve WLD?>\x00" as *const u8 as
            *const i8);
        printf(b"    Toggles WLD only solve on/off (default %d).\n\n\x00" as
                   *const u8 as *const i8, 0 as i32);
        puts(b"  -line <output line?>\x00" as *const u8 as
            *const i8);
        printf(b"    Toggles output of optimal line on/off (default %d).\n\n\x00"
                   as *const u8 as *const i8, 0 as i32);
        puts(b"  -b <use book?>\x00" as *const u8 as *const i8);
        printf(b"    Toggles usage of opening book on/off (default %d).\n\x00"
                   as *const u8 as *const i8, 0 as i32);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -komi <komi>\x00" as *const u8 as *const i8);
        puts(b"    Number of discs that white has to win with (only WLD).\x00"
            as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        exit(1 as i32);
    }
    if hash_bits < 1 as i32 {
        printf(b"Hash table key must contain at least 1 bit\n\x00" as
            *const u8 as *const i8);
        exit(1 as i32);
    }
    global_setup(use_random, hash_bits);
    init_thor_database::<LibcFatalError>();
    if use_book != 0 {
        init_learn(b"book.bin\x00" as *const u8 as *const i8,
                   1 as i32);
    }
    if use_random != 0 && 1 as i32 == 0 {
        FE::time(&mut timer);
        my_srandom(timer as i32);
    } else { my_srandom(1 as i32); }
    if run_script != 0 {
        run_endgame_script(script_in_file, script_out_file,
                           script_optimal_line);
    }
    global_terminate::<LibcFatalError>();
    return 0 as i32;
}
static mut use_thor: i32 = 0;
static mut use_learning: i32 = 0;
static mut wld_only: i32 = 0 as i32;
static mut use_book: i32 = 0 as i32;
static mut wait: i32 = 0;
static mut skill: [i32; 3] = [0; 3];
static mut player_increment: [f64; 3] = [0.; 3];
static mut player_time: [f64; 3] = [0.; 3];
static mut log_file_name: *mut i8 =
    0 as *const i8 as *mut i8;

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
