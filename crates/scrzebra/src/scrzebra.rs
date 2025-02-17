#![allow(dead_code,  non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut, unused_must_use)]

use engine::src::counter::{add_counter, counter_value, CounterType, reset_counter};
use engine::src::hash::setup_hash;
use engine::src::osfbook::{reset_book_search, set_deviation_value};
use engine::src::search::disc_count;
use engine::src::zebra::EvaluationType;
use legacy_zebra::src::display::{display_move, display_state};
use legacy_zebra::src::game::{legacy_compute_move, game_init, global_setup, toggle_status_log};
use legacy_zebra::src::learn::init_learn;
use legacy_zebra::src::thordb::{init_thor_database, LegacyThor};
use legacy_zebra::src::zebra::{FullState, LibcTimeSource};
use libc_wrapper::{FileHandle, strlen, strstr, time, fclose, fopen, fprintf, printf, sscanf,
                   fgets, fputs, puts, feof, exit, strcasecmp, stdout, atoi};
use std::io::Write;

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
unsafe fn run_endgame_script(mut in_file_name: *const i8,
                             mut out_file_name: *const i8,
                             mut display_line: i32,
                             g_state: &mut FullState, mut thor: LegacyThor) {
    let mut script_nodes = CounterType { hi: 0, lo: 0 };
    let mut eval_info =  EvaluationType::new();
    let mut comment = 0 as *mut i8;
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
    let mut move_0 = 0;
    let mut score: i32 = 0;
    let mut timed_search: i32 = 0;
    let mut scanned: i32 = 0;
    let mut token: i32 = 0;
    let mut position_count: i32 = 0;
    let mut script_stream = FileHandle::null();
    let mut output_stream = FileHandle::null();
    /* If the played move is the best, output the already calculated
               score for the best move - that way we avoid a subtle problem:
               Suppose (N-1)-ply move is X but N-ply move is Y, where Y is
               the best move. Then averaging the corresponding scores won't
               coincide with the N-ply averaged score for Y. */
    /* Open the files and get the number of positions */
    script_stream =
        fopen(in_file_name, b"r\x00" as *const u8 as *const i8);
    if script_stream.is_null() {
        printf(b"\nCan\'t open script file \'%s\' - aborting\n\n\x00" as
                   *const u8 as *const i8, in_file_name);
        exit(1);
    }
    output_stream =
        fopen(out_file_name, b"w\x00" as *const u8 as *const i8);
    if output_stream.is_null() {
        printf(b"\nCan\'t create output file \'%s\' - aborting\n\n\x00" as
                   *const u8 as *const i8, out_file_name);
        exit(1);
    }
    fclose(output_stream);
    /* Initialize display subsystem and search parameters */
    display_state.set_names("", "");
    display_state.set_move_list(g_state.board.score_sheet_row);
    display_state.set_evals(0.0f64, 0.0f64);
    i = 0;
    while i < 60 {
        g_state.board.black_moves[i as usize] = -1;
        g_state.board.white_moves[i as usize] = -1;
        i += 1
    }
    my_time = 100000000;
    my_incr = 0;
    timed_search = 0;
    book = use_book;
    mid = 60;
    if wld_only != 0 {
        exact = 0
    } else { exact = 60 }
    wld = 60;
    toggle_status_log(0);
    reset_counter(&mut script_nodes);
    position_count = 0;
    max_search = -0.0f64;
    start_time =  g_state.timer.get_real_timer();
    /* Scan through the script file */
    i = 0;
    loop  {
        let mut pass_count = 0;
        /* Check if the line is a comment or an end marker */
        fgets(buffer.as_mut_ptr(), 256, script_stream);
        if feof(script_stream) != 0 { break ; }
        if buffer[0] as i32 == '%' as i32 {
            /* Comment */
            output_stream =
                fopen(out_file_name,
                      b"a\x00" as *const u8 as *const i8);
            if output_stream.is_null() {
                printf(b"\nCan\'t append to output file \'%s\' - aborting\n\n\x00"
                           as *const u8 as *const i8,
                       out_file_name);
                exit(1);
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
                write!(stdout, "\nEOF encountered when reading position #{} - aborting\n\n", i + 1);
                exit(1);
            }
            /* Parse the script line containing board and side to move */
            game_init(&mut side_to_move, g_state);
            g_state.g_book.set_slack(0.0f64 as i32);
            g_state.game.toggle_human_openings(0);
            reset_book_search(&mut g_state.g_book);
            set_deviation_value(0, 60, 0.0f64, &mut g_state.g_book);
            setup_hash(1, &mut g_state.hash, &mut g_state.random);
            position_count += 1;
            scanned =
                sscanf(buffer.as_mut_ptr(),
                       b"%s %s\x00" as *const u8 as *const i8,
                       board_string.as_mut_ptr(), stm_string.as_mut_ptr());
            if scanned != 2 {
                write!(stdout, "\nError parsing line {} - aborting\n\n", i + 1);
                exit(1);
            }
            if   strlen(stm_string.as_mut_ptr()) != 1 {
                write!(stdout, "\nAmbiguous side to move on line {} - aborting\n\n", i + 1);
                exit(1);
            }
            match stm_string[0] as i32 {
                79 | 48 => { side_to_move = 2 }
                42 | 88 => { side_to_move = 0 }
                _ => {
                    write!(stdout, "\nBad side-to-move indicator on line {} - aborting\n\n", i + 1);
                }
            }
            if   strlen(board_string.as_mut_ptr()) != 64 {
                write!(stdout, "\nBoard on line {} doesn\'t contain 64 positions - aborting\n\n", i + 1);
                exit(1);
            }
            token = 0;
            row = 1;
            while row <= 8 {
                col = 1;
                while col <= 8 {
                    pos = 10 * row + col;
                    match board_string[token as usize] as i32 {
                        42 | 88 | 120 => {
                            g_state.board.board[pos as usize] = 0
                        }
                        79 | 48 | 111 => {
                            g_state.board.board[pos as usize] = 2
                        }
                        45 | 46 => { g_state.board.board[pos as usize] = 1 }
                        _ => {
                            write!(stdout, "\nBad character \'{}\' in board on line {} - aborting\n\n",
                                   char::from(board_string[token as usize] as u8), i + 1);
                        }
                    }
                    token += 1;
                    col += 1
                }
                row += 1
            }
            g_state.moves.disks_played =
                disc_count(0, &g_state.board.board) + disc_count(2, &g_state.board.board) -
                    4;
            /* Search the position */
            if g_state.config.echo != 0 {
                display_state.set_move_list(g_state.board.score_sheet_row);
                display_state.display_board(&mut stdout, &g_state.board.board, side_to_move,
                                            1, 0,
                                            1,
                                            &g_state.board.black_moves, &g_state.board.white_moves);
            }
            search_start =  g_state.timer.get_real_timer();
            g_state.timer.start_move(my_time as f64, my_incr as f64,
                                     g_state.moves.disks_played + 4);
            g_state.timer.determine_move_time(my_time as f64,
                                              my_incr as f64,
                                              g_state.moves.disks_played + 4);
            pass_count = 0;
            move_0 =
                legacy_compute_move(side_to_move, 1, my_time, my_incr,
                                    timed_search, book, mid, exact, wld,
                                    1, &mut eval_info, g_state, &mut thor);
            if move_0 == -(1) {
                pass_count += 1;
                side_to_move =
                    0 + 2 - side_to_move;
                move_0 =
                    legacy_compute_move(side_to_move, 1, my_time,
                                        my_incr, timed_search, book, mid, exact, wld,
                                        1, &mut eval_info, g_state, &mut thor);
                if move_0 == -(1) {
                    /* Both pass, game over. */
                    let mut my_discs = disc_count(side_to_move, &g_state.board.board);
                    let mut opp_discs =
                        disc_count(0 + 2 - side_to_move, &g_state.board.board);
                    if my_discs > opp_discs {
                        my_discs = 64 - opp_discs
                    } else if opp_discs > my_discs {
                        opp_discs = 64 - my_discs
                    } else {
                        opp_discs = 32;
                        my_discs = opp_discs
                    }
                    eval_info.score =
                        128 * (my_discs - opp_discs);
                    pass_count += 1
                }
            }
            score = eval_info.score / 128;
            search_stop =  g_state.timer.get_real_timer();
            if search_stop - search_start > max_search {
                max_search = search_stop - search_start
            }
            add_counter(&mut script_nodes, &mut g_state.search.nodes);
            output_stream =
                fopen(out_file_name,
                      b"a\x00" as *const u8 as *const i8);
            if output_stream.is_null() {
                printf(b"\nCan\'t append to output file \'%s\' - aborting\n\n\x00"
                           as *const u8 as *const i8,
                       out_file_name);
                exit(1);
            }
            if wld_only != 0 {
                if side_to_move == 0 {
                    if score > 0 {
                        fputs(b"Black win\x00" as *const u8 as
                                  *const i8, output_stream);
                    } else if score == 0 {
                        fputs(b"Draw\x00" as *const u8 as *const i8,
                              output_stream);
                    } else {
                        fputs(b"White win\x00" as *const u8 as
                                  *const i8, output_stream);
                    }
                } else if score > 0 {
                    fputs(b"White win\x00" as *const u8 as
                              *const i8, output_stream);
                } else if score == 0 {
                    fputs(b"Draw\x00" as *const u8 as *const i8,
                          output_stream);
                } else {
                    fputs(b"Black win\x00" as *const u8 as
                              *const i8, output_stream);
                }
            } else if side_to_move == 0 {
                fprintf(output_stream,
                        b"%2d - %2d\x00" as *const u8 as *const i8,
                        32 + score / 2,
                        32 - score / 2);
            } else {
                fprintf(output_stream,
                        b"%2d - %2d\x00" as *const u8 as *const i8,
                        32 - score / 2,
                        32 + score / 2);
            }
            if display_line != 0 && pass_count != 2 {
                fputs(b"   \x00" as *const u8 as *const i8,
                      output_stream);
                if pass_count == 1 {
                    fputs(b" --\x00" as *const u8 as *const i8,
                          output_stream);
                }
                j = 0;
                while j < g_state.search.full_pv_depth {
                    fputs(b" \x00" as *const u8 as *const i8,
                          output_stream);
                    display_move(&mut output_stream, g_state.search.full_pv[j as usize]);
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
            if g_state.config.echo != 0 {
                puts(b"\n\n\n\x00" as *const u8 as *const i8);
            }
        }
        i += 1
    }
    /* Clean up and terminate */
    fclose(script_stream);
    stop_time =  (&mut g_state.timer).get_real_timer();
    write!(stdout, "Total positions solved:   {}\n", position_count);
    write!(stdout, "Total time:               {:.1} s\n", stop_time - start_time);
    write!(stdout, "Total nodes:              {:.0}\n", counter_value(&mut script_nodes));
    write!(stdout, "\n");
    write!(stdout, "Average time for solve:   {:.1} s\n", (stop_time - start_time) / position_count as f64);
    write!(stdout, "Maximum time for solve:   {:.1} s\n", max_search);
    write!(stdout, "\n");
    write!(stdout, "Average speed:            {:.0} nps\n",
           counter_value(&mut script_nodes) / (stop_time - start_time));
    write!(stdout, "\n");
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8)
                 -> i32 {
    let mut script_in_file: *const i8 = 0 as *const i8;
    let mut script_out_file: *const i8 = 0 as *const i8;
    let mut arg_index: i32 = 0;
    let mut help: i32 = 0;
    let mut hash_bits: i32 = 0;
    let mut use_random: i32 = 0;
    let mut run_script: i32 = 0;
    let mut script_optimal_line: i32 = 0;
    let mut komi: i32 = 0;
    let mut timer: time_t = 0;
    printf(b"\nscrZebra (c) 1997-2005 Gunnar Andersson, compile date %s at %s\n\n\x00"
               as *const u8 as *const i8,
           b"Aug  9 2020\x00" as *const u8 as *const i8,
           b"20:20:01\x00" as *const u8 as *const i8);
    use_random = 1;
    wait = 0;
    static src: LibcTimeSource = LibcTimeSource {};
    let mut full_state = FullState::new(&src);
    let g_state: &mut FullState = &mut full_state;
    let mut g_config = &mut g_state.config;
    let mut game_state = &mut g_state.game;
    g_config.echo = 1;
    g_config.display_pv = 1;
    use_learning = 0;
    use_thor = 0;
    skill[2] = -(1);
    skill[0] = skill[2];
    hash_bits = 18;
    log_file_name = 0 as *mut i8;
    run_script = 0;
    script_out_file = 0 as *const i8;
    script_in_file = script_out_file;
    komi = 0;
    player_time[2] = 10000000.0f64;
    player_time[0] =
        player_time[2];
    player_increment[2] = 0.0f64;
    player_increment[0] =
        player_increment[2];
    let mut current_block_37: u64;
    arg_index = 1;
    help = 0;
    while arg_index < argc && help == 0 {
        if strcasecmp(*argv.offset(arg_index as isize),
                      b"-e\x00" as *const u8 as *const i8) == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_37 = 4808432441040389987;
            } else {
                g_config.echo = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-h\x00" as *const u8 as *const i8) ==
            0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
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
                help = 1;
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
                help = 1;
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
                help = 1;
                current_block_37 = 4808432441040389987;
            } else {
                script_optimal_line = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-script\x00" as *const u8 as
                                 *const i8) == 0 {
            if arg_index + 2 >= argc {
                help = 1;
                current_block_37 = 4808432441040389987;
            } else {
                arg_index += 1;
                script_in_file = *argv.offset(arg_index as isize);
                arg_index += 1;
                script_out_file = *argv.offset(arg_index as isize);
                run_script = 1;
                current_block_37 = 13303144130133872306;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-komi\x00" as *const u8 as *const i8)
            == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_37 = 4808432441040389987;
            } else {
                komi = atoi(*argv.offset(arg_index as isize));
                current_block_37 = 13303144130133872306;
            }
        } else {
            help = 1;
            current_block_37 = 13303144130133872306;
        }
        match current_block_37 {
            13303144130133872306 => {
                if arg_index >= argc { help = 1 }
            }
            _ => { }
        }
        arg_index += 1
    }
    if run_script == 0 { help = 1 }
    if komi != 0 {
        if wld_only == 0 {
            puts(b"Komi can only be applied to WLD solves.\x00" as *const u8
                as *const i8);
            exit(1);
        }
        game_state.set_komi(komi);
    }
    if help != 0 {
        puts(b"Usage:\x00" as *const u8 as *const i8);
        puts(b"  scrzebra [-e ...] [-h ...] [-wld ...] [-line ...] [-b ...] [-komi ...] -script ...\x00"
            as *const u8 as *const i8);
        write!(stdout, "\n");
        puts(b"  -e <echo?>\x00" as *const u8 as *const i8);
        printf(b"    Toggles screen output on/off (default %d).\n\n\x00" as
                   *const u8 as *const i8, 1);
        puts(b"  -h <bits in hash key>\x00" as *const u8 as
            *const i8);
        printf(b"    Size of hash table is 2^{this value} (default %d).\n\n\x00"
                   as *const u8 as *const i8, 18);
        puts(b"  -script <script file> <output file>\x00" as *const u8 as
            *const i8);
        puts(b"    Solves all positions in script file for exact score.\n\x00"
            as *const u8 as *const i8);
        puts(b"  -wld <only solve WLD?>\x00" as *const u8 as
            *const i8);
        printf(b"    Toggles WLD only solve on/off (default %d).\n\n\x00" as
                   *const u8 as *const i8, 0);
        puts(b"  -line <output line?>\x00" as *const u8 as
            *const i8);
        printf(b"    Toggles output of optimal line on/off (default %d).\n\n\x00"
                   as *const u8 as *const i8, 0);
        puts(b"  -b <use book?>\x00" as *const u8 as *const i8);
        printf(b"    Toggles usage of opening book on/off (default %d).\n\x00"
                   as *const u8 as *const i8, 0);
        write!(stdout, "\n");
        puts(b"  -komi <komi>\x00" as *const u8 as *const i8);
        puts(b"    Number of discs that white has to win with (only WLD).\x00"
            as *const u8 as *const i8);
        write!(stdout, "\n");
        write!(stdout, "\n");
        exit(1);
    }
    if hash_bits < 1 {
        printf(b"Hash table key must contain at least 1 bit\n\x00" as
            *const u8 as *const i8);
        exit(1);
    }
    global_setup(use_random, hash_bits, g_state);
    let mut thor = LegacyThor::new();
    init_thor_database(&mut thor, &mut g_state.random);
    if use_book != 0 {
        init_learn(b"book.bin\x00" as *const u8 as *const i8,
                   1, g_state);
    }
    if use_random != 0 && 1 == 0 {
        time(&mut timer);
        let x = timer as i32;
        g_state.random.my_srandom(x);
    } else {
        let x = 1;
        g_state.random.my_srandom(x); }
    if run_script != 0 {
        run_endgame_script(script_in_file, script_out_file,
                           script_optimal_line, g_state, thor);
    }
    0
}
static mut use_thor: i32 = 0;
static mut use_learning: i32 = 0;
static mut wld_only: i32 = 0;
static mut use_book: i32 = 0;
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
