#![allow(dead_code, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]

use std::process::exit;
use std::ptr::null_mut;
use engine::src::game::{generic_game_init, global_terminate, game_state};
use crate::src::game::{LibcBoardFileSource, LibcZebraOutput, LogFileHandler, compute_move, global_setup, toggle_status_log};
use crate::src::learn::{LibcLearner, init_learn};
use crate::src::thordb::{read_game_database, read_tournament_database, read_player_database, print_thor_matches, LegacyThor, get_total_game_count, choose_thor_opening_move, get_thor_game_size, init_thor_database};
use crate::src::error::{LibcFatalError, FE, fatal_error};
use engine::src::error::{FrontEnd, FatalError};
use libc_wrapper::{fclose, fputs, fprintf, fopen, fputc, puts, printf, strstr, sscanf, feof, fgets, atoi, scanf, sprintf, ctime, time, strchr, strcasecmp, atof, stdout, strlen};
use engine::src::globals::{board_state};
use engine::src::counter::{counter_value, add_counter, reset_counter, CounterType, adjust_counter};
use engine::src::timer::{g_timer};
use engine::src::search::{disc_count, produce_compact_eval, search_state};
use crate::src::display::{display_move, display_board, dumpch, set_names, set_move_list, set_evals, set_times, toggle_smart_buffer_management, white_eval, white_time, white_player, black_eval, black_time, black_player, current_row};
use engine::src::moves::{make_move, game_in_progress, moves_state, generate_all, valid_move, unmake_move};
use engine::src::hash::{setup_hash, hash_state};
use engine::src::osfbook::{set_deviation_value, reset_book_search, find_opening_name, g_book};
use engine::src::stubs::floor;
use engine::src::getcoeff::{remove_coeffs, coeff_state};
use engine::src::myrandom::{random_instance};
use crate::src::osfbook::print_move_alternatives;
use engine::src::zebra::{set_default_engine_globals, DumpHandler, EvaluationType, ZebraFrontend, engine_play_game, InitialMoveSource, Config, INITIAL_CONFIG, learn_state};
use libc_wrapper::{FILE, time_t};
use engine::src::myrandom;
use flip::unflip;
use engine::src::zebra::EvalResult::{WON_POSITION, LOST_POSITION};
use engine::src::zebra::EvalType::MIDGAME_EVAL;
use engine::src::zebra::DrawMode::{OPPONENT_WINS, WHITE_WINS, BLACK_WINS, NEUTRAL};
use engine::src::zebra::GameMode::{PUBLIC_GAME, PRIVATE_GAME};
use flip::unflip::flip_stack_;
use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::Read;

pub static mut g_config: Config = INITIAL_CONFIG;
/* ------------------- Function prototypes ---------------------- */
/* Administrative routines */
/* ---------------------- Functions ------------------------ */
/*
   MAIN
Interprets the command-line parameters and starts the game.
*/
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8)
 -> i32 {
    printf(b"\nZebra (c) 1997-2005 Gunnar Andersson, compile date %s at %s\n\n\x00"
               as *const u8 as *const i8,
           // TODO add macro or smth for these (it's in the C code)
           b"Jul  2 2020\x00" as *const u8 as *const i8,
           b"19:33:54\x00" as *const u8 as *const i8);
    let mut config = &mut g_config;

    let mut move_sequence = 0 as *const i8;
    let mut move_file_name = 0 as *const i8;
    let mut repeat = 1;
    let mut script_optimal_line = 0;
    let mut timer: time_t = 0;
    let mut use_random = 1;
    let mut hash_bits = 18;
    let mut game_file_name = 0 as *const i8;
    let mut log_file_name = 0 as *mut i8;
    let run_script = 0;
    let script_out_file = 0 as *const i8;
    let script_in_file = script_out_file;
    let mut use_learning = 0;
    let mut use_thor = 0;
    set_default_engine_globals(config);
    let mut current_block_107: u64;
    let mut arg_index = 1;
    let mut help = 0;
    while arg_index < argc && help == 0 {
        if strcasecmp(*argv.offset(arg_index as isize),
                      b"-e\x00" as *const u8 as *const i8) == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                g_config.echo = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-h\x00" as *const u8 as *const i8) ==
                      0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                hash_bits = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-l\x00" as *const u8 as *const i8) ==
                      0 {
            config.tournament = 0;
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                config.skill[0] =
                    atoi(*argv.offset(arg_index as isize));
                if config.skill[0] > 0 as i32 {
                    if arg_index + 2 as i32 >= argc {
                        help = 1;
                        current_block_107 = 2668756484064249700;
                    } else {
                        arg_index += 1;
                        config.exact_skill[0] =
                            atoi(*argv.offset(arg_index as isize));
                        arg_index += 1;
                        config.wld_skill[0] =
                            atoi(*argv.offset(arg_index as isize));
                        current_block_107 = 15004371738079956865;
                    }
                } else { current_block_107 = 15004371738079956865; }
                match current_block_107 {
                    2668756484064249700 => { }
                    _ => {
                        arg_index += 1;
                        if arg_index == argc {
                            help = 1;
                            current_block_107 = 2668756484064249700;
                        } else {
                            config.skill[2] =
                                atoi(*argv.offset(arg_index as isize));
                            if config.skill[2] >
                                   0 as i32 {
                                if arg_index + 2 as i32 >= argc {
                                    help = 1;
                                    current_block_107 = 2668756484064249700;
                                } else {
                                    arg_index += 1;
                                    config.exact_skill[2] =
                                        atoi(*argv.offset(arg_index as
                                                              isize));
                                    arg_index += 1;
                                    config.wld_skill[2] =
                                        atoi(*argv.offset(arg_index as
                                                              isize));
                                    current_block_107 = 10485226111480991281;
                                }
                            } else {
                                current_block_107 = 10485226111480991281;
                            }
                        }
                    }
                }
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-t\x00" as *const u8 as *const i8) ==
                      0 {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                config.tournament = 1;
                config.tournament_levels = atoi(*argv.offset(arg_index as isize));
                if arg_index + 3 as i32 * config.tournament_levels >= argc {
                    help = 1;
                    current_block_107 = 2668756484064249700;
                } else {
                    i = 0;
                    while i < config.tournament_levels {
                        j = 0;
                        while j < 3 as i32 {
                            arg_index += 1;
                            config.tournament_skill[i as usize][j as usize] =
                                atoi(*argv.offset(arg_index as isize));
                            j += 1
                        }
                        i += 1
                    }
                    current_block_107 = 10485226111480991281;
                }
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-w\x00" as *const u8 as *const i8) ==
                      0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                config.wait = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-p\x00" as *const u8 as *const i8) ==
                      0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                config.display_pv = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"?\x00" as *const u8 as *const i8) ==
                      0 {
            help = 1;
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-g\x00" as *const u8 as *const i8) ==
                      0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                game_file_name = *argv.offset(arg_index as isize);
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-r\x00" as *const u8 as *const i8) ==
                      0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                use_random = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-b\x00" as *const u8 as *const i8) ==
                      0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                config.use_book = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-time\x00" as *const u8 as *const i8)
                      == 0 {
            if arg_index + 4 as i32 >= argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                arg_index += 1;
                config.player_time[0] =
                    atoi(*argv.offset(arg_index as isize)) as f64;
                arg_index += 1;
                config.player_increment[0] =
                    atoi(*argv.offset(arg_index as isize)) as f64;
                arg_index += 1;
                config.player_time[2] =
                    atoi(*argv.offset(arg_index as isize)) as f64;
                arg_index += 1;
                config.player_increment[2] =
                    atoi(*argv.offset(arg_index as isize)) as f64;
                config.use_timer = 1;
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-learn\x00" as *const u8 as
                                 *const i8) == 0 {
            if arg_index + 2 as i32 >= argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                arg_index += 1;
                config.deviation_depth = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                config.cutoff_empty = atoi(*argv.offset(arg_index as isize));
                use_learning = 1;
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-slack\x00" as *const u8 as
                                 *const i8) == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                config.slack = atof(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-dev\x00" as *const u8 as *const i8)
                      == 0 {
            if arg_index + 3 as i32 >= argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                arg_index += 1;
                config.low_thresh = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                config.high_thresh = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                config.dev_bonus = atof(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-log\x00" as *const u8 as *const i8)
                      == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                log_file_name = *argv.offset(arg_index as isize);
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-private\x00" as *const u8 as
                                 *const i8) == 0 {
            g_book.set_game_mode(PRIVATE_GAME);
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-public\x00" as *const u8 as
                                 *const i8) == 0 {
            g_book.set_game_mode(PUBLIC_GAME);
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-keepdraw\x00" as *const u8 as
                                 *const i8) == 0 {
            g_book.set_draw_mode(NEUTRAL);
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-draw2black\x00" as *const u8 as
                                 *const i8) == 0 {
            g_book.set_draw_mode(BLACK_WINS);
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-draw2white\x00" as *const u8 as
                                 *const i8) == 0 {
            g_book.set_draw_mode(WHITE_WINS);
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-draw2none\x00" as *const u8 as
                                 *const i8) == 0 {
            g_book.set_draw_mode(OPPONENT_WINS);
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-test\x00" as *const u8 as *const i8)
                      == 0 {
            config.one_position_only = 1;
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-seq\x00" as *const u8 as *const i8)
                      == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                move_sequence = *argv.offset(arg_index as isize);
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-seqfile\x00" as *const u8 as
                                 *const i8) == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                move_file_name = *argv.offset(arg_index as isize);
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-repeat\x00" as *const u8 as
                                 *const i8) == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                repeat = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-thor\x00" as *const u8 as *const i8)
                      == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                use_thor = 1;
                config.thor_max_games = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-analyze\x00" as *const u8 as
                                 *const i8) == 0 {
            config.only_analyze = 1;
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-randmove\x00" as *const u8 as
                                 *const i8) == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                config.rand_move_freq = atoi(*argv.offset(arg_index as isize));
                if config.rand_move_freq < 0 as i32 {
                    help = 1;
                    current_block_107 = 2668756484064249700;
                } else { current_block_107 = 10485226111480991281; }
            }
        } else {
            help = 1;
            current_block_107 = 10485226111480991281;
        }
        match current_block_107 {
            10485226111480991281 => {
                if arg_index >= argc { help = 1 as i32 }
            }
            _ => { }
        }
        arg_index += 1
    }
    if help != 0 {
        puts(b"Usage:\x00" as *const u8 as *const i8);
        puts(b"  zebra [-b -e -g -h -l -p -t -time -w -learn -slack -dev -log\x00"
                 as *const u8 as *const i8);
        puts(b"         -keepdraw -draw2black -draw2white -draw2none\x00" as
                 *const u8 as *const i8);
        puts(b"         -private -public -test -seq -thor -script -analyze ?\x00"
                 as *const u8 as *const i8);
        puts(b"         -repeat -seqfile]\x00" as *const u8 as
                 *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"Flags:\x00" as *const u8 as *const i8);
        puts(b"  ? \x00" as *const u8 as *const i8);
        puts(b"    Displays this text.\x00" as *const u8 as
                 *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -b <use book?>\x00" as *const u8 as *const i8);
        printf(b"    Toggles usage of opening book on/off (default %d).\n\x00"
                   as *const u8 as *const i8, 1 as i32);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -e <echo?>\x00" as *const u8 as *const i8);
        printf(b"    Toggles screen output on/off (default %d).\n\x00" as
                   *const u8 as *const i8, 1 as i32);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -g <game file>\x00" as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -h <bits in hash key>\x00" as *const u8 as
                 *const i8);
        printf(b"    Size of hash table is 2^{this value} (default %d).\n\x00"
                   as *const u8 as *const i8, 18 as i32);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -l <black depth> [<black exact depth> <black WLD depth>]\x00"
                 as *const u8 as *const i8);
        puts(b"     <white depth> [<white exact depth> <white WLD depth>]\x00"
                 as *const u8 as *const i8);
        printf(b"    Sets the search depth. If <black depth> or <white depth> \x00"
                   as *const u8 as *const i8);
        printf(b"are set to 0, a\n\x00" as *const u8 as *const i8);
        printf(b"    human player is assumed. In this case the other \x00" as
                   *const u8 as *const i8);
        printf(b"parameters must be omitted.\n\x00" as *const u8 as
                   *const i8);
        printf(b"    <* exact depth> specify the number of moves before the \x00"
                   as *const u8 as *const i8);
        printf(b"(at move 60) when\n\x00" as *const u8 as
                   *const i8);
        printf(b"    the exact game-theoretical value is calculated. <* WLD \x00"
                   as *const u8 as *const i8);
        printf(b"depth> are used\n\x00" as *const u8 as *const i8);
        puts(b"    analogously for the calculation of Win/Loss/Draw.\x00" as
                 *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -p <display principal variation?>\x00" as *const u8 as
                 *const i8);
        printf(b"    Toggles output of principal variation on/off (default %d).\n\x00"
                   as *const u8 as *const i8, 1 as i32);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -r <use randomization?>\x00" as *const u8 as
                 *const i8);
        printf(b"    Toggles randomization on/off (default %d)\n\x00" as
                   *const u8 as *const i8, 1 as i32);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -t <number of levels> <(first) depth> ... <(last) wld depth>\x00"
                 as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -time <black time> <black increment> <white time> <white increment>\x00"
                 as *const u8 as *const i8);
        puts(b"    Tournament mode; the format for the players is as above.\x00"
                 as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -w <wait?>\x00" as *const u8 as *const i8);
        printf(b"    Toggles wait between moves on/off (default %d).\n\x00" as
                   *const u8 as *const i8, 0 as i32);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -learn <depth> <cutoff>\x00" as *const u8 as
                 *const i8);
        puts(b"    Learn the game with <depth> deviations up to <cutoff> empty.\x00"
                 as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -slack <disks>\x00" as *const u8 as *const i8);
        printf(b"    Zebra\'s opening randomness is <disks> disks (default %f).\n\x00"
                   as *const u8 as *const i8, 0.25f64);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -dev <low> <high> <bonus>\x00" as *const u8 as
                 *const i8);
        puts(b"    Give deviations before move <high> a <bonus> disk bonus but\x00"
                 as *const u8 as *const i8);
        puts(b"    don\'t give any extra bonus for deviations before move <low>.\x00"
                 as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -log <file name>\x00" as *const u8 as *const i8);
        puts(b"    Append all game results to the specified file.\x00" as
                 *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -private\x00" as *const u8 as *const i8);
        puts(b"    Treats all draws as losses for both sides.\x00" as
                 *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -public\x00" as *const u8 as *const i8);
        puts(b"    No tweaking of draw scores.\x00" as *const u8 as
                 *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -keepdraw\x00" as *const u8 as *const i8);
        puts(b"    Book draws are counted as draws.\x00" as *const u8 as
                 *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -draw2black\x00" as *const u8 as *const i8);
        puts(b"    Book draws scored as 32-31 for black.\x00" as *const u8 as
                 *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -draw2white\x00" as *const u8 as *const i8);
        puts(b"    Book draws scored as 32-31 for white.\x00" as *const u8 as
                 *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -draw2none\x00" as *const u8 as *const i8);
        puts(b"    Book draws scored as 32-31 for the opponent.\x00" as
                 *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -test\x00" as *const u8 as *const i8);
        puts(b"    Only evaluate one position, then exit.\x00" as *const u8 as
                 *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -seq <move sequence>\x00" as *const u8 as
                 *const i8);
        puts(b"    Forces the game to start with a predefined move sequence;\x00"
                 as *const u8 as *const i8);
        puts(b"    e.g. f4d6c3.\x00" as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -seqfile <filename\x00" as *const u8 as *const i8);
        puts(b"    Specifies a file from which move sequences are read.\x00"
                 as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -thor <game count>\x00" as *const u8 as *const i8);
        puts(b"    Look for each position in the Thor database; list the first <game count>.\x00"
                 as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -script <script file> <output file>\x00" as *const u8 as
                 *const i8);
        puts(b"    Solves all positions in script file for exact score.\x00"
                 as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"  -wld <only solve WLD?>\x00" as *const u8 as
                 *const i8);
        printf(b"    Toggles WLD only solve on/off (default %d).\n\n\x00" as
                   *const u8 as *const i8, 0 as i32);
        puts(b"  -analyze\x00" as *const u8 as *const i8);
        puts(b"    Used in conjunction with -seq; all positions are analyzed.\x00"
                 as *const u8 as *const i8);
        puts(b"  -repeat <#iterations>\x00" as *const u8 as
                 *const i8);
        puts(b"    Repeats the operation the specified number of times. \x00"
                 as *const u8 as *const i8);
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
    if config.use_book != 0 {
        let file_name = if let Ok(var) = std::env::var("BOOK_PATH") {
            CString::new(var).unwrap()
        } else {
            CString::new("book.bin").unwrap()
        };
        init_learn(file_name.as_ref().as_ptr() as *const u8 as *const i8,
                   1 as i32);
    }
    if use_random != 0 {
        time(&mut timer);
        let x = timer as i32;
        random_instance.my_srandom(x);
    } else {
        let x = 1 as i32;
        random_instance.my_srandom(x); }
    if config.tournament == 0 && run_script == 0 {
        while config.skill[0] < 0 as i32 {
            printf(b"Black parameters: \x00" as *const u8 as
                       *const i8);
            scanf(b"%d\x00" as *const u8 as *const i8,
                  &mut *config.skill.as_mut_ptr()
                      as *mut i32);
            if config.skill[0] > 0 as i32 {
                scanf(b"%d %d\x00" as *const u8 as *const i8,
                      &mut *config.exact_skill.as_mut_ptr() as
                          *mut i32,
                      &mut *config.wld_skill.as_mut_ptr() as
                          *mut i32);
            }
        }
        while config.skill[2] < 0 as i32 {
            printf(b"White parameters: \x00" as *const u8 as
                       *const i8);
            scanf(b"%d\x00" as *const u8 as *const i8,
                  &mut *config.skill.as_mut_ptr().offset(2)
                      as *mut i32);
            if config.skill[2] > 0 as i32 {
                scanf(b"%d %d\x00" as *const u8 as *const i8,
                      &mut *config.exact_skill.as_mut_ptr().offset(2) as
                          *mut i32,
                      &mut *config.wld_skill.as_mut_ptr().offset(2) as
                          *mut i32);
            }
        }
    }
    if config.one_position_only != 0 {
        toggle_smart_buffer_management(0 as i32);
    }
    if run_script != 0 {
        run_endgame_script(script_in_file, script_out_file,
                           script_optimal_line);
    } else if config.tournament != 0 {
        play_tournament(move_sequence, log_file_name, use_thor != 0, use_learning != 0);
    } else if config.only_analyze != 0 {
        analyze_game(move_sequence);
    } else {
        play_game(game_file_name, move_sequence, move_file_name, repeat, log_file_name, use_thor != 0, use_learning != 0);
    }
    global_terminate();
    return 0 as i32;
}
/*
   PLAY_TOURNAMENT
   Administrates the tournament between different levels
   of the program.
*/
unsafe fn play_tournament(mut move_sequence: *const i8, log_file_name_: *mut i8, use_thor_: bool, use_learning_: bool) {
    let config = &mut g_config;
    let mut result: [[[i32; 3]; 8]; 8] = [[[0; 3]; 8]; 8];
    let mut tourney_time: f64 = 0.;
    let mut score: [f64; 8] = [0.; 8];
    let mut color_score: [f64; 3] = [0.; 3];
    let mut tourney_nodes = CounterType{hi: 0, lo: 0,};
    reset_counter(&mut tourney_nodes);
    tourney_time = 0.0f64;
    let mut i = 0;
    while i < 8 as i32 {
        score[i as usize] = 0.0f64;
        i += 1
    }
    color_score[2] = 0.0f64;
    color_score[0] = color_score[2];

    let mut i = 0;
    let mut j = 0;
    let tournament_levels_ = config.tournament_levels;
    while i < tournament_levels_ {
        j = 0;
        while j < tournament_levels_ {
            config.skill[0] =
                config.tournament_skill[i as usize][0];
            config.exact_skill[0] =
                config.tournament_skill[i as usize][1];
            config.wld_skill[0] =
                config.tournament_skill[i as usize][2];
            config.skill[2] =
                config.tournament_skill[j as usize][0];
            config.exact_skill[2] =
                config.tournament_skill[j as usize][1];
            config.wld_skill[2] =
                config.tournament_skill[j as usize][2];
            play_game(0 as *const i8, move_sequence,
                      0 as *const i8, 1 as i32, log_file_name_, use_thor_, use_learning_);
            add_counter(&mut tourney_nodes, &mut search_state.total_nodes);
            tourney_time += search_state.total_time;
            result[i as usize][j as usize][0] =
                disc_count(0 as i32, &board_state.board);
            result[i as usize][j as usize][2] =
                disc_count(2 as i32, &board_state.board);
            if disc_count(0 as i32, &board_state.board) > disc_count(2 as i32, &board_state.board) {
                score[i as usize] += 1.0f64;
                color_score[0] += 1.0f64
            } else if disc_count(0 as i32, &board_state.board) ==
                          disc_count(2 as i32, &board_state.board) {
                score[i as usize] += 0.5f64;
                score[j as usize] += 0.5f64;
                color_score[0] += 0.5f64;
                color_score[2] += 0.5f64
            } else {
                score[j as usize] += 1.0f64;
                color_score[2] += 1.0f64
            }
            j += 1
        }
        i += 1
    }
    adjust_counter(&mut tourney_nodes);
    let tournament_skill_ = &config.tournament_skill;
    let tourney_counter_value = counter_value(&mut tourney_nodes);

    printf(b"\n\nTime:  %.1f s\nNodes: %.0f\n\x00" as *const u8 as
               *const i8, tourney_time,
           tourney_counter_value);
    puts(b"\nCompetitors:\x00" as *const u8 as *const i8);
    let mut i = 0;
    while i < tournament_levels_ {
        printf(b"  Player %2d: %d-%d-%d\n\x00" as *const u8 as
                   *const i8, i + 1 as i32,
               tournament_skill_[i as usize][0],
               tournament_skill_[i as usize][1],
               tournament_skill_[i as usize][2]);
        i += 1
    }
    printf(b"\n       \x00" as *const u8 as *const i8);
    let mut i = 0;
    while i < tournament_levels_ {
        printf(b" %2d    \x00" as *const u8 as *const i8,
               i + 1 as i32);
        i += 1
    }
    puts(b"  Score\x00" as *const u8 as *const i8);
    let mut i = 0;
    let mut j = 0;
    while i < tournament_levels_ {
        printf(b"  %2d   \x00" as *const u8 as *const i8,
               i + 1 as i32);
        j = 0;
        while j < tournament_levels_ {
            printf(b"%2d-%2d  \x00" as *const u8 as *const i8,
                   result[i as usize][j as usize][0],
                   result[i as usize][j as usize][2]);
            j += 1
        }
        printf(b"  %4.1f\n\x00" as *const u8 as *const i8,
               score[i as usize]);
        i += 1
    }
    puts(b"\x00" as *const u8 as *const i8);
    printf(b"Black score: %.1f\n\x00" as *const u8 as *const i8,
           color_score[0]);
    printf(b"White score: %.1f\n\x00" as *const u8 as *const i8,
           color_score[2]);
    puts(b"\x00" as *const u8 as *const i8);
}

impl InitialMoveSource for FileMoveSource {
    fn fill_line_buffer(&mut self, line_buffer: &mut [u8]) {
        self.move_file.read(line_buffer.as_mut());

        let newline_pos = line_buffer.iter()
            .enumerate()
            .find(|(i, ch)| **ch == '\n' as i8 as u8 );

        if let Some(newline_pos) = newline_pos {
            line_buffer[newline_pos.0] = 0;
        } else {
            line_buffer[line_buffer.len() - 1] = 0;
        }
    }
}

struct FileMoveSource {
    move_file: File
}

impl FileMoveSource {
    pub fn open(move_file_name: &str) -> Option<FileMoveSource> {
        File::open(move_file_name).map(|file| {
            FileMoveSource {
                move_file: file
            }
        }).ok()
    }
}
/*
   PLAY_GAME
   Administrates the game between two players, humans or computers.
*/
unsafe fn play_game(mut file_name: *const i8,
                    mut move_string: *const i8,
                    mut move_file_name: *const i8,
                    mut repeat: i32, log_file_name_: *mut i8, use_thor_: bool, use_learning_: bool) {
    let mut move_file = if move_file_name.is_null() {
        None
    } else {
        let move_file_name = CStr::from_ptr(move_file_name).to_str().unwrap();
        FileMoveSource::open(move_file_name)
    };

    engine_play_game
        ::<LibcFrontend, _, LibcDumpHandler, LibcBoardFileSource, LogFileHandler, LibcZebraOutput, LibcLearner, LibcFatalError, LegacyThor>
        (file_name, move_string, repeat, log_file_name_, move_file, use_thor_, use_learning_, &mut g_config)
}

struct LibcFrontend {} //TODO this could probably be merged with the FrontEnd trait or something
impl ZebraFrontend for LibcFrontend {
    fn set_evals(black: f64, white: f64) {
        unsafe { set_evals(black, white); }
    }

    fn set_move_list(row: i32) {
        unsafe { set_move_list(null_mut(), null_mut(), row) }
    }

    fn set_names(white_is_player: bool, black_is_player: bool) {
        let black_name = if white_is_player {
            b"Player\x00" as *const u8 as *const i8
        } else {
            b"Zebra\x00" as *const u8 as *const i8
        };
        let white_name = if black_is_player {
            b"Player\x00" as *const u8 as *const i8
        } else {
            b"Zebra\x00" as *const u8 as *const i8
        };
        unsafe { set_names(black_name, white_name) }
    }

    fn set_times(black: i32, white: i32) {
        unsafe { set_times(black, white) }
    }

    fn report_some_thor_scores(black_win_count: i32, draw_count: i32, white_win_count: i32, black_median_score: i32, black_average_score: f64) {
        unsafe {
            printf(b"%d black wins,  %d draws,  %d white wins\n\x00"
                       as *const u8 as *const i8,
                   black_win_count, draw_count,
                   white_win_count);
            printf(b"Median score %d-%d\n\x00" as *const u8 as
                       *const i8, black_median_score,
                   64 as i32 - black_median_score);
            printf(b", average score %.2f-%.2f\n\x00" as *const u8 as
                       *const i8, black_average_score,
                   64.0f64 - black_average_score);
        }
    }
    fn report_some_thor_stats(total_search_time: f64, thor_position_count: i32, db_search_time: f64) {
        unsafe {
            printf(b"%d matching games  (%.3f s search time, %.3f s total)\n\x00"
                       as *const u8 as *const i8,
                   thor_position_count, db_search_time,
                   total_search_time);
        }
    }
    fn display_board_after_thor(side_to_move: i32, give_time_: i32, board_: &[i32; 128],
                                black_moves_: &[i32; 60], white_moves_: &[i32; 60]) {
        unsafe {
            display_board(stdout, board_,
                          side_to_move, 1,
                          give_time_, 1,
                          current_row, black_player, black_time, black_eval,
                          white_player, white_time, white_eval, black_moves_, white_moves_);
        }
    }
    fn print_out_thor_matches(thor_max_games_: i32) {
        unsafe { print_thor_matches(stdout, thor_max_games_); }
    }
    fn log_game_ending(log_file_name_: &CStr, move_vec: &[i8; 121],
                              first_side_to_move: i32, second_side_to_move: i32) {
        let log_file_name_ = log_file_name_.as_ptr();
        unsafe {
            let log_file = fopen(log_file_name_,
                                 b"a\x00" as *const u8 as *const i8);

            if !log_file.is_null() {
                let mut timer = time(0 as *mut time_t);
                fprintf(log_file,
                        b"# %s#     %2d - %2d\n\x00" as *const u8 as
                            *const i8, ctime(&mut timer),
                        first_side_to_move,
                        second_side_to_move);
                fprintf(log_file,
                        b"%s\n\x00" as *const u8 as *const i8,
                        move_vec.as_ptr() as *mut i8);
                fclose(log_file);
            }
        }
    }

    fn get_pass() {
        unsafe {
            puts(b"You must pass - please press Enter\x00" as
                *const u8 as *const i8);
            dumpch();
        }
    }
    fn report_engine_override() {
        unsafe {
            puts(b"Engine override: Random move selected.\x00"
                as *const u8 as *const i8);
        }
    }

    fn prompt_get_move(side_to_move: i32, buffer: &mut [i8; 255]) -> i32 {
        unsafe {
            if side_to_move == 0 as i32 {
                printf(b"%s: \x00" as *const u8 as *const i8,
                       b"Black move\x00" as *const u8 as *const i8);
            } else {
                printf(b"%s: \x00" as *const u8 as *const i8,
                       b"White move\x00" as *const u8 as *const i8);
            }
            scanf(b"%s\x00" as *const u8 as *const i8, buffer.as_mut_ptr());
            atoi(buffer.as_mut_ptr())
        }
    }

    fn before_get_move()  {
        unsafe {
            puts(b"\x00" as *const u8 as *const i8);
        }
    }
    fn report_after_game_ended(node_val: f64, eval_val: f64, black_disc_count: i32, white_disc_count: i32, total_time_: f64) {
        unsafe {
            printf(b"\nBlack: %d   White: %d\n\x00" as *const u8 as
                       *const i8, black_disc_count,
                   white_disc_count);
            printf(b"Nodes searched:        %-10.0f\n\x00" as *const u8 as
                       *const i8, node_val);
            printf(b"Positions evaluated:   %-10.0f\n\x00" as *const u8 as
                       *const i8, eval_val);

            printf(b"Total time: %.1f s\n\x00" as *const u8 as
                       *const i8, total_time_);
        }
    }
    fn report_skill_levels(black_level: i32, white_level: i32) {
        unsafe {
            printf(b"\n\x00" as *const u8 as *const i8);
            printf(b"Black level: %d\n\x00" as *const u8 as *const i8, black_level);
            printf(b"White level: %d\n\x00" as *const u8 as *const i8, white_level);
        }
    }
    fn report_thor_matching_games_stats(total_search_time: f64, thor_position_count: i32, database_time: f64) {
        unsafe {
            printf(b"%d matching games  (%.3f s search time, %.3f s total)\n\x00"
                       as *const u8 as *const i8,
                   thor_position_count,
                   database_time,
                   total_search_time);
        }
    }
    fn report_thor_stats(black_win_count: i32, draw_count: i32, white_win_count: i32, black_median_score: i32, black_average_score: f64) {
        unsafe {
            printf(b"%d black wins, %d draws, %d white wins\n\x00"
                       as *const u8 as *const i8,
                   black_win_count, draw_count,
                   white_win_count);
            printf(b"Median score %d-%d\x00" as *const u8 as
                       *const i8,
                   black_median_score,
                   64 as i32 -
                       black_median_score);
            printf(b", average score %.2f-%.2f\n\x00" as
                       *const u8 as *const i8,
                   black_average_score,
                   64.0f64 - black_average_score);
        }
    }
    fn report_opening_name(opening_name: &CStr) {
        unsafe { printf(b"\nOpening: %s\n\x00" as *const u8 as *const i8, opening_name.as_ptr()); }
    }
    fn report_book_randomness(slack_: f64) {
        unsafe { printf(b"Book randomness: %.2f disks\n\x00" as *const u8 as *const i8, slack_); }
    }
    fn load_thor_files() { unsafe {
        /* No error checking done as it's only for testing purposes */
        let database_start =  g_timer.get_real_timer::<FE>();
        read_player_database(b"thor\\wthor.jou\x00" as *const u8 as
            *const i8);
        read_tournament_database(b"thor\\wthor.trn\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_2001.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_2000.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1999.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1998.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1997.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1996.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1995.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1994.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1993.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1992.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1991.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1990.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1989.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1988.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1987.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1986.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1985.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1984.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1983.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1982.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1981.wtb\x00" as *const u8 as
            *const i8);
        read_game_database(b"thor\\wth_1980.wtb\x00" as *const u8 as
            *const i8);
        let database_stop =  g_timer.get_real_timer::<FE>();
        printf(b"Loaded %d games in %.3f s.\n\x00" as *const u8 as
                   *const i8, get_total_game_count(),
               database_stop - database_start);
        printf(b"Each Thor game occupies %d bytes.\n\x00" as *const u8 as
                   *const i8, get_thor_game_size());
    }}

    fn print_move_alternatives(side_to_move: i32) {
        unsafe { print_move_alternatives(side_to_move) }
    }

    fn dumpch() {
        unsafe { dumpch() }
    }
}

/*
   ANALYZE_GAME
   Analyzes all positions arising from a given move sequence.
*/
unsafe fn analyze_game(mut move_string: *const i8) {
    let config = &mut g_config;
    let mut best_info1 =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut best_info2 =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut played_info1 =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut played_info2 =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut opening_name = 0 as *const i8;
    let mut move_start: f64 = 0.;
    let mut move_stop: f64 = 0.;
    let mut i: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut opponent: i32 = 0;
    let mut curr_move: i32 = 0;
    let mut resp_move: i32 = 0;
    let mut timed_search: i32 = 0;
    let mut provided_move_count: i32 = 0;
    let mut col: i32 = 0;
    let mut row: i32 = 0;
    let mut empties: i32 = 0;
    let mut provided_move: [i32; 61] = [0; 61];
    /* Decode the predefined move sequence */
    if move_string.is_null() {
        provided_move_count = 0 as i32
    } else {
        provided_move_count =
              strlen(move_string).wrapping_div(2 as i32 as
                                                 u64) as
                i32;
        if provided_move_count > 60 as i32 ||
                 strlen(move_string).wrapping_rem(2 as i32 as
                                                    u64) ==
                   1 as i32 as u64 {
            FE::invalid_move_string_provided();
        }
        i = 0;
        while i < provided_move_count {
            col =
               FE::tolower(*move_string.offset((2 as i32 * i) as isize)
                            as i32) - 'a' as i32 + 1 as i32;
            row =
                *move_string.offset((2 as i32 * i + 1 as i32)
                                        as isize) as i32 - '0' as i32;
            if col < 1 as i32 || col > 8 as i32 ||
                   row < 1 as i32 || row > 8 as i32 {
                FE::unexpected_character_in_a_move_string();
            }
            provided_move[i as usize] = 10 as i32 * row + col;
            i += 1
        }
    }
    /* Open the output log file */
    let output_stream =
        fopen(b"analysis.log\x00" as *const u8 as *const i8,
              b"w\x00" as *const u8 as *const i8);
    if output_stream.is_null() {
        fatal_error(b"Can\'t create log file analysis.log - aborting\x00" as
                        *const u8 as *const i8);
    }
    /* Set up the position and the search engine */
    if g_config.echo != 0 {
        puts(b"Analyzing provided game...\x00" as *const u8 as
                 *const i8);
    }
    generic_game_init::<LibcBoardFileSource, LibcFatalError>(0 as *const i8, &mut side_to_move);
    setup_hash(1 as i32, &mut hash_state, &mut  random_instance);
    learn_state.clear_stored_game();
    if g_config.echo != 0 && config.use_book != 0 {
        puts(b"Disabling usage of opening book\x00" as *const u8 as
                 *const i8);
    }
    config.use_book = 0;
    reset_book_search(&mut g_book);
    let black_name = b"Zebra\x00" as *const u8 as *const i8;
    let white_name = b"Zebra\x00" as *const u8 as *const i8;
    set_names(black_name, white_name);
    set_move_list(board_state.black_moves.as_mut_ptr(), board_state.white_moves.as_mut_ptr(),
                  board_state.score_sheet_row);
    set_evals(0.0f64, 0.0f64);
    let mut i = 0;
    while i < 60 as i32 {
        board_state.black_moves[i as usize] = -(1 as i32);
        board_state.white_moves[i as usize] = -(1 as i32);
        i += 1
    }
    let _black_hash1 = random_instance.my_random() as i32;
    let _black_hash2 = random_instance.my_random() as i32;
    let _white_hash1 = random_instance.my_random() as i32;
    let _white_hash2 = random_instance.my_random() as i32;
    let best_trans1 = random_instance.my_random() as u32;
    let best_trans2 = random_instance.my_random() as u32;
    let played_trans1 = random_instance.my_random() as u32;
    let played_trans2 = random_instance.my_random() as u32;
    while game_in_progress(&mut moves_state, &search_state, &board_state.board) != 0 && moves_state.disks_played < provided_move_count {
        remove_coeffs(moves_state.disks_played, &mut coeff_state);
        generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
        if side_to_move == 0 as i32 { board_state.score_sheet_row += 1 }
        if moves_state.move_count[moves_state.disks_played as usize] != 0 as i32 {
            move_start =  g_timer.get_real_timer::<FE>();
            g_timer.clear_panic_abort();
            if g_config.echo != 0 {
                set_move_list(board_state.black_moves.as_mut_ptr(),
                              board_state.white_moves.as_mut_ptr(), board_state.score_sheet_row);
                set_times(floor(config.player_time[0]) as
                              i32,
                          floor(config.player_time[2]) as
                              i32);
                opening_name = find_opening_name( &mut g_book, &board_state.board);
                if !opening_name.is_null() {
                    printf(b"\nOpening: %s\n\x00" as *const u8 as
                               *const i8, opening_name);
                }
                display_board(stdout, &board_state.board, side_to_move,
                              1, config.use_timer, 1,
                              current_row,
                              black_player, black_time, black_eval,
                              white_player, white_time, white_eval,
                              &board_state.black_moves, &board_state.white_moves);
            }
            /* Check what the Thor opening statistics has to say */
            choose_thor_opening_move(&board_state.board, side_to_move, g_config.echo);
            if g_config.echo != 0 && config.wait != 0 { dumpch(); }
             g_timer.start_move::<FE>(config.player_time[side_to_move as usize],
                             config.player_increment[side_to_move as usize],
                             moves_state.disks_played + 4 as i32);
            g_timer.determine_move_time(config.player_time[side_to_move as usize],
                                config.player_increment[side_to_move as usize],
                                moves_state.disks_played + 4 as i32);
            timed_search =
                (config.skill[side_to_move as usize] >= 60 as i32) as
                    i32;
            empties = 60 as i32 - moves_state.disks_played;
            /* Determine the score for the move actually played.
               A private hash transformation is used so that the parallel
            search trees - "played" and "best" - don't clash. This way
             all scores are comparable. */
            hash_state.set_hash_transformation(played_trans1, played_trans2);
            curr_move = provided_move[moves_state.disks_played as usize];
            opponent = 0 as i32 + 2 as i32 - side_to_move;
            make_move(side_to_move, curr_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
            if empties > config.wld_skill[side_to_move as usize] {
                reset_counter(&mut search_state.nodes);
                resp_move =
                    compute_move(opponent, 0 as i32,
                                 config.player_time[opponent as usize] as
                                     i32,
                                 config.player_increment[opponent as usize] as
                                     i32, timed_search, config.use_book,
                                 config.skill[opponent as usize] - 2 as i32,
                                 config.exact_skill[opponent as usize] -
                                     1 as i32,
                                 config.wld_skill[opponent as usize] -
                                     1 as i32, 1 as i32,
                                 &mut played_info1)
            }
            reset_counter(&mut search_state.nodes);
            resp_move =
                compute_move(opponent, 0 as i32,
                             config.player_time[opponent as usize] as i32,
                             config.player_increment[opponent as usize] as
                                 i32, timed_search, config.use_book,
                             config.skill[opponent as usize] - 1 as i32,
                             config.exact_skill[opponent as usize] -
                                 1 as i32,
                             config.wld_skill[opponent as usize] - 1 as i32,
                             1 as i32, &mut played_info2);
            let move_0 = curr_move;
            {
                unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
            };
            /* Determine the 'best' move and its score. For midgame moves,
            search twice to dampen oscillations. Unless we're in the endgame
             region, a private hash transform is used - see above. */
            if empties > config.wld_skill[side_to_move as usize] {
                hash_state.set_hash_transformation(best_trans1, best_trans2);
                reset_counter(&mut search_state.nodes);
                curr_move =
                    compute_move(side_to_move, 0 as i32,
                                 config.player_time[side_to_move as usize] as
                                     i32,
                                 config.player_increment[side_to_move as usize] as
                                     i32, timed_search, config.use_book,
                                 config.skill[side_to_move as usize] -
                                     1 as i32,
                                 config.exact_skill[side_to_move as usize],
                                 config.wld_skill[side_to_move as usize],
                                 1 as i32, &mut best_info1)
            }
            reset_counter(&mut search_state.nodes);
            curr_move =
                compute_move(side_to_move, 0 as i32,
                             config.player_time[side_to_move as usize] as
                                 i32,
                             config.player_increment[side_to_move as usize] as
                                 i32, timed_search, config.use_book,
                             config.skill[side_to_move as usize],
                             config.exact_skill[side_to_move as usize],
                             config.wld_skill[side_to_move as usize],
                             1 as i32, &mut best_info2);
            if side_to_move == 0 as i32 {
                set_evals(produce_compact_eval(best_info2), 0.0f64);
            } else { set_evals(0.0f64, produce_compact_eval(best_info2)); }
            /* Output the two score-move pairs */
            fprintf(output_stream,
                    b"%c%c \x00" as *const u8 as *const i8,
                    'a' as i32 + curr_move % 10 as i32 -
                        1 as i32,
                    '0' as i32 + curr_move / 10 as i32);
            if empties <= config.exact_skill[side_to_move as usize] {
                fprintf(output_stream,
                        b"%+6d\x00" as *const u8 as *const i8,
                        best_info2.score / 128 as i32);
            } else if empties <= config.wld_skill[side_to_move as usize] {
                if best_info2.res as u32 ==
                       WON_POSITION as i32 as u32 {
                    fputs(b"    +1\x00" as *const u8 as *const i8,
                          output_stream);
                } else if best_info2.res as u32 ==
                              LOST_POSITION as i32 as u32 {
                    fputs(b"    -1\x00" as *const u8 as *const i8,
                          output_stream);
                } else {
                    fputs(b"     0\x00" as *const u8 as *const i8,
                          output_stream);
                }
            } else if curr_move == provided_move[moves_state.disks_played as usize] &&
                          resp_move != -(1 as i32) {
                fprintf(output_stream,
                        b"%6.2f\x00" as *const u8 as *const i8,
                        -(played_info1.score + played_info2.score) as
                            f64 /
                            (2 as i32 as f64 * 128.0f64));
            } else {
                fprintf(output_stream,
                        b"%6.2f\x00" as *const u8 as *const i8,
                        (best_info1.score + best_info2.score) as
                            f64 /
                            (2 as i32 as f64 * 128.0f64));
            }
            curr_move = provided_move[moves_state.disks_played as usize];
            fprintf(output_stream,
                    b"       %c%c \x00" as *const u8 as *const i8,
                    'a' as i32 + curr_move % 10 as i32 -
                        1 as i32,
                    '0' as i32 + curr_move / 10 as i32);
            if resp_move == -(1 as i32) {
                fprintf(output_stream,
                        b"     ?\x00" as *const u8 as *const i8);
            } else if empties <= config.exact_skill[side_to_move as usize] {
                fprintf(output_stream,
                        b"%+6d\x00" as *const u8 as *const i8,
                        -played_info2.score / 128 as i32);
            } else if empties <= config.wld_skill[side_to_move as usize] {
                if played_info2.res as u32 ==
                       WON_POSITION as i32 as u32 {
                    fputs(b"    -1\x00" as *const u8 as *const i8,
                          output_stream);
                } else if played_info2.res as u32 ==
                              LOST_POSITION as i32 as u32 {
                    fputs(b"    +1\x00" as *const u8 as *const i8,
                          output_stream);
                } else {
                    fputs(b"     0\x00" as *const u8 as *const i8,
                          output_stream);
                }
            } else {
                fprintf(output_stream,
                        b"%6.2f\x00" as *const u8 as *const i8,
                        -(played_info1.score + played_info2.score) as
                            f64 /
                            (2 as i32 as f64 * 128.0f64));
            }
            fputs(b"\n\x00" as *const u8 as *const i8,
                  output_stream);
            if valid_move(curr_move, side_to_move, &board_state.board) == 0 {
                fatal_error(b"Invalid move %c%c in move sequence\x00" as
                                *const u8 as *const i8,
                            'a' as i32 + curr_move % 10 as i32 -
                                1 as i32,
                            '0' as i32 + curr_move / 10 as i32);
            }
            move_stop =  g_timer.get_real_timer::<FE>();
            if config.player_time[side_to_move as usize] != 10000000.0f64 {
                config.player_time[side_to_move as usize] -= move_stop - move_start
            }
            learn_state.store_move(moves_state.disks_played, curr_move);
            make_move(side_to_move, curr_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
            if side_to_move == 0 as i32 {
                board_state.black_moves[board_state.score_sheet_row as usize] = curr_move
            } else {
                if board_state.white_moves[board_state.score_sheet_row as usize] !=
                       -(1 as i32) {
                    board_state.score_sheet_row += 1
                }
                board_state.white_moves[board_state.score_sheet_row as usize] = curr_move
            }
        } else if side_to_move == 0 as i32 {
            board_state.black_moves[board_state.score_sheet_row as usize] = -(1 as i32)
        } else { board_state.white_moves[board_state.score_sheet_row as usize] = -(1 as i32) }
        side_to_move = 0 as i32 + 2 as i32 - side_to_move
    }
    if g_config.echo == 0 {
        printf(b"\n\x00" as *const u8 as *const i8);
        printf(b"Black level: %d\n\x00" as *const u8 as *const i8,
               config.skill[0]);
        printf(b"White level: %d\n\x00" as *const u8 as *const i8,
               config.skill[2]);
    }
    if side_to_move == 0 as i32 { board_state.score_sheet_row += 1 }
    LibcDumpHandler::dump_game_score(side_to_move, board_state.score_sheet_row, &board_state.black_moves, &board_state.white_moves);
    if g_config.echo != 0 && config.one_position_only == 0 {
        set_move_list(board_state.black_moves.as_mut_ptr(), board_state.white_moves.as_mut_ptr(),
                      board_state.score_sheet_row);
        set_times(floor(config.player_time[0]) as
                      i32,
                  floor(config.player_time[2]) as
                      i32);
        display_board(stdout, &board_state.board, side_to_move,
                      1 as i32, config.use_timer, 1 as i32,
                      current_row,
                      black_player, black_time, black_eval,
                      white_player, white_time, white_eval,
                      &board_state.black_moves, &board_state.white_moves);
    }
    fclose(output_stream);
}
unsafe fn run_endgame_script(mut in_file_name: *const i8,
                                        mut out_file_name:
                                            *const i8,
                                        mut display_line: i32) {
    let config = &mut g_config;
    let mut script_nodes = CounterType{hi: 0, lo: 0,};
    let mut eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
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
    let mut move_0: i32 = 0;
    let mut score: i32 = 0;
    let mut timed_search: i32 = 0;
    let mut scanned: i32 = 0;
    let mut token: i32 = 0;
    let mut position_count: i32 = 0;
    let mut script_stream = 0 as *mut FILE;
    let mut output_stream = 0 as *mut FILE;
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
    set_names(b"\x00" as *const u8 as *const i8,
              b"\x00" as *const u8 as *const i8);
    set_move_list(board_state.black_moves.as_mut_ptr(), board_state.white_moves.as_mut_ptr(),
                  board_state.score_sheet_row);
    set_evals(0.0f64, 0.0f64);
    i = 0;
    while i < 60 as i32 {
        board_state.black_moves[i as usize] = -(1 as i32);
        board_state.white_moves[i as usize] = -(1 as i32);
        i += 1
    }
    my_time = 100000000;
    my_incr = 0;
    timed_search = 0;
    book = config.use_book;
    mid = 60;
    if config.wld_only != 0 {
        exact = 0 as i32
    } else { exact = 60 as i32 }
    wld = 60;
    toggle_status_log(0 as i32);
    reset_counter(&mut script_nodes);
    position_count = 0;
    max_search = -0.0f64;
    start_time =  g_timer.get_real_timer::<FE>();
    /* Scan through the script file */
    i = 0;
    loop  {
        let mut pass_count = 0;
        /* Check if the line is a comment or an end marker */
        fgets(buffer.as_mut_ptr(), 256 as i32, script_stream);
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
            generic_game_init::<LibcBoardFileSource, LibcFatalError>(0 as *const i8, &mut side_to_move);
            g_book.set_slack(0.0f64 as i32);
            game_state.toggle_human_openings(0 as i32);
            reset_book_search(&mut g_book);
            set_deviation_value(0 as i32, 60 as i32, 0.0f64, &mut g_book);
            setup_hash(1 as i32, &mut hash_state, &mut  random_instance);
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
            if   strlen(stm_string.as_mut_ptr()) !=
                   1 as i32 as u64 {
                printf(b"\nAmbiguous side to move on line %d - aborting\n\n\x00"
                           as *const u8 as *const i8,
                       i + 1 as i32);
                exit(1 as i32);
            }
            match stm_string[0] as i32 {
                79 | 48 => { side_to_move = 2 as i32 }
                42 | 88 => { side_to_move = 0 as i32 }
                _ => {
                    printf(b"\nBad side-to-move indicator on line %d - aborting\n\n\x00"
                               as *const u8 as *const i8,
                           i + 1 as i32);
                }
            }
            if  strlen(board_string.as_mut_ptr()) !=
                   64 as i32 as u64 {
                printf(b"\nBoard on line %d doesn\'t contain 64 positions - aborting\n\n\x00"
                           as *const u8 as *const i8,
                       i + 1 as i32);
                exit(1 as i32);
            }
            token = 0;
            row = 1;
            while row <= 8 as i32 {
                col = 1;
                while col <= 8 as i32 {
                    pos = 10 as i32 * row + col;
                    match board_string[token as usize] as i32 {
                        42 | 88 | 120 => {
                            board_state.board[pos as usize] = 0 as i32
                        }
                        79 | 48 | 111 => {
                            board_state.board[pos as usize] = 2 as i32
                        }
                        45 | 46 => { board_state.board[pos as usize] = 1 as i32 }
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
            moves_state.disks_played =
                disc_count(0 as i32, &board_state.board) + disc_count(2 as i32, &board_state.board) -
                    4 as i32;
            /* Search the position */
            if g_config.echo != 0 {
                set_move_list(board_state.black_moves.as_mut_ptr(),
                              board_state.white_moves.as_mut_ptr(), board_state.score_sheet_row);
                display_board(stdout, &board_state.board, side_to_move,
                              1 as i32, 0 as i32,
                              1 as i32, current_row,
                              black_player, black_time, black_eval,
                              white_player, white_time, white_eval,
                              &board_state.black_moves, &board_state.white_moves);
            }
            search_start =  g_timer.get_real_timer::<FE>();
             g_timer.start_move::<FE>(my_time as f64, my_incr as f64,
                             moves_state.disks_played + 4 as i32);
            g_timer.determine_move_time(my_time as f64,
                                my_incr as f64,
                                moves_state.disks_played + 4 as i32);
            pass_count = 0;
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
                    let mut my_discs = disc_count(side_to_move, &board_state.board);
                    let mut opp_discs =
                        disc_count(0 as i32 + 2 as i32 -
                                       side_to_move, &board_state.board);
                    if my_discs > opp_discs {
                        my_discs = 64 as i32 - opp_discs
                    } else if opp_discs > my_discs {
                        opp_discs = 64 as i32 - my_discs
                    } else {
                        opp_discs = 32;
                        my_discs = opp_discs
                    }
                    eval_info.score =
                        128 as i32 * (my_discs - opp_discs);
                    pass_count += 1
                }
            }
            score = eval_info.score / 128 as i32;
            search_stop =  g_timer.get_real_timer::<FE>();
            if search_stop - search_start > max_search {
                max_search = search_stop - search_start
            }
            add_counter(&mut script_nodes, &mut search_state.nodes);
            output_stream =
                fopen(out_file_name,
                      b"a\x00" as *const u8 as *const i8);
            if output_stream.is_null() {
                printf(b"\nCan\'t append to output file \'%s\' - aborting\n\n\x00"
                           as *const u8 as *const i8,
                       out_file_name);
                exit(1 as i32);
            }
            if config.wld_only != 0 {
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
                j = 0;
                while j < search_state.full_pv_depth {
                    fputs(b" \x00" as *const u8 as *const i8,
                          output_stream);
                    display_move(output_stream, search_state.full_pv[j as usize]);
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
            if g_config.echo != 0 {
                puts(b"\n\n\n\x00" as *const u8 as *const i8);
            }
        }
        i += 1
    }
    /* Clean up and terminate */
    fclose(script_stream);
    stop_time =  g_timer.get_real_timer::<FE>();
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

struct LibcDumpHandler;
impl DumpHandler for LibcDumpHandler {
    /*
       DUMP_POSITION
       Saves the current board position to disk.
    */
    fn dump_position(side_to_move: i32, board_: &[i32; 128]) { unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut stream = 0 as *mut FILE;
        stream =
            fopen(b"current.gam\x00" as *const u8 as *const i8,
                  b"w\x00" as *const u8 as *const i8);
        if stream.is_null() {
            fatal_error(b"File creation error when writing CURRENT.GAM\n\x00" as
                *const u8 as *const i8);
        }
        i = 1;
        while i <= 8 as i32 {
            j = 1;
            while j <= 8 as i32 {
                match board_[(10 as i32 * i + j) as usize] {
                    0 => { fputc('X' as i32, stream); }
                    2 => { fputc('O' as i32, stream); }
                    1 => { fputc('-' as i32, stream); }
                    _ => {
                        /* This really can't happen but shouldn't cause a crash */
                        fputc('?' as i32, stream);
                    }
                }
                j += 1
            }
            i += 1
        }
        fputs(b"\n\x00" as *const u8 as *const i8, stream);
        if side_to_move == 0 as i32 {
            fputs(b"Black\x00" as *const u8 as *const i8, stream);
        } else {
            fputs(b"White\x00" as *const u8 as *const i8, stream);
        }
        fputs(b" to move\nThis file was automatically generated\n\x00" as
                  *const u8 as *const i8, stream);
        fclose(stream);
    }}
    /*
      DUMP_GAME_SCORE
      Writes the current game score to disk.
    */
    fn dump_game_score(side_to_move: i32, score_sheet_row_: i32,
                       black_moves_: &[i32; 60], white_moves_: &[i32; 60]) { unsafe {
        let mut stream = 0 as *mut FILE;
        let mut i: i32 = 0;
        stream =
            fopen(b"current.mov\x00" as *const u8 as *const i8,
                  b"w\x00" as *const u8 as *const i8);
        if stream.is_null() {
            fatal_error(b"File creation error when writing CURRENT.MOV\n\x00" as
                *const u8 as *const i8);
        }
        i = 0;
        while i <= score_sheet_row_ {
            fprintf(stream,
                    b"   %2d.    \x00" as *const u8 as *const i8,
                    i + 1 as i32);
            if black_moves_[i as usize] == -(1 as i32) {
                fputs(b"- \x00" as *const u8 as *const i8, stream);
            } else {
                fprintf(stream, b"%c%c\x00" as *const u8 as *const i8,
                        'a' as i32 + black_moves_[i as usize] % 10 as i32 -
                            1 as i32,
                        '0' as i32 + black_moves_[i as usize] / 10 as i32);
            }
            fputs(b"  \x00" as *const u8 as *const i8, stream);
            if i < score_sheet_row_ ||
                i == score_sheet_row_ && side_to_move == 0 as i32 {
                if white_moves_[i as usize] == -(1 as i32) {
                    fputs(b"- \x00" as *const u8 as *const i8, stream);
                } else {
                    fprintf(stream,
                            b"%c%c\x00" as *const u8 as *const i8,
                            'a' as i32 +
                                white_moves_[i as usize] % 10 as i32 -
                                1 as i32,
                            '0' as i32 +
                                white_moves_[i as usize] / 10 as i32);
                }
            }
            fputs(b"\n\x00" as *const u8 as *const i8, stream);
            i += 1
        }
        fclose(stream);
    }}
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
