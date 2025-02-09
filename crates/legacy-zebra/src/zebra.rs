#![allow(dead_code, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Read, Write, BufRead, StdinLock};
use std::process::exit;

use engine::src::counter::{add_counter, adjust_counter, counter_value, CounterType, reset_counter};
use engine::src::error::{FatalError,  };
use engine::src::game::{generic_game_init,  };
use engine::src::getcoeff::{ remove_coeffs};
use engine::src::globals::BoardState;
use engine::src::hash::{  setup_hash};
use engine::src::moves::{game_in_progress, generate_all, make_move,  unmake_move, valid_move};

use engine::src::osfbook::{Book, find_opening_name, reset_book_search,  };

use engine::src::search::{disc_count, produce_compact_eval,  };

use engine::src::stubs::floor;
use engine::src::timer::{Timer, TimeSource};
use engine::src::zebra::{Config, EvaluationType, InitialMoveSource, set_default_engine_globals, ZebraFrontend, next_state, PlayGameState, MoveAttempt, PlayGame};
use engine::src::zebra::DrawMode::{BLACK_WINS, NEUTRAL, OPPONENT_WINS, WHITE_WINS};
use engine::src::zebra::EvalResult::{LOST_POSITION, WON_POSITION};
use engine::src::zebra::GameMode::{PRIVATE_GAME, PUBLIC_GAME};
use libc_wrapper::{scanf, stdout, time, c_time, time_t};
use crate::src::display::{dumpch, display_state, TO_SQUARE};
use crate::src::error::LibcFatalError;
use crate::src::game::{legacy_compute_move, global_setup, BasicBoardFileSource, LibcZebraOutput, LogFileHandler};
use crate::src::learn::{init_learn, LibcLearner};
use crate::src::osfbook::print_move_alternatives;
use crate::src::thordb::{choose_thor_opening_move, get_thor_game_size, get_total_game_count, init_thor_database, LegacyThor, print_thor_matches, read_game_database, read_player_database, read_tournament_database};

#[macro_use]
use crate::fatal_error;

pub struct LibcTimeSource;

impl TimeSource for LibcTimeSource {
    fn time(& self, __timer: &mut i64) -> i64 {
        // fixme.. with the mutable reference here.. this is probably unsound
        /// this can be called from multiple threads. Is it safe?
        unsafe { time(__timer) }
    }
}
// This function mimics the behaviour of atoi function (except for the UB)
/// ie. parses an integer, skips leading whitespace and stop at first non-numeric character
pub trait Atoi {
    fn atoi(&self) -> i32;
}
impl<T> Atoi for T where T: AsRef<[u8]> {
    fn atoi(&self) -> i32 {
        fn atoi(s: &[u8]) -> i32 {
            let mut sign = None;
            s.iter()
                .map(|&byte| byte)
                .skip_while(u8::is_ascii_whitespace)
                .skip_while(|ch| if sign.is_some() {
                    false
                } else if *ch == b'-' {
                    sign = Some(-1);
                    true
                } else if *ch == b'+' {
                    sign = Some(1);
                    true
                } else {
                    false
                })
                .take_while(u8::is_ascii_digit)
                .fold(0i32, |acc, val| acc * 10 + (val - b'0') as i32)
                .mul(sign.unwrap_or(1))
        }
        atoi(self.as_ref())
    }
}
/* ------------------- Function prototypes ---------------------- */
/* Administrative routines */
/* ---------------------- Functions ------------------------ */
/*
   MAIN
Interprets the command-line parameters and starts the game.
*/
unsafe fn main_0()
 -> i32 {
    let mut argv = Vec::new();
    let args1 = ::std::env::args().collect::<Vec<_>>();
    for arg in args1.iter() {
        argv.push(arg.as_str());
    };
    let mut argc = (argv.len()) as i32;

    print!("\nZebra (c) 1997-2005 Gunnar Andersson, compile date {} at {}\n\n",
           // TODO add macro or smth for these (it's in the C code)
           "Jul  2 2020",
           "19:33:54");

    static time_src: LibcTimeSource = LibcTimeSource {};
    let mut g_state = FullState::new(&time_src);


    let mut move_sequence = "";
    let mut move_file_name = "";
    let mut repeat = 1;
    let mut script_optimal_line = 0;
    let mut timer: time_t = 0;
    let mut use_random = 1;
    let mut hash_bits = 18;
    let mut game_file_name = "";
    let mut log_file_name = "";
    let script_out_file = "";
    let script_in_file = "";
    set_default_engine_globals((&mut g_state.config));
    let mut current_block_107: u64;
    let mut arg_index = 1;
    let mut help = 0;
    while arg_index < argc && help == 0 {
        if argv[arg_index as usize] == "-e" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                (g_state.config).echo = (argv[arg_index as usize]).parse().unwrap_or(0);
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-h" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                hash_bits = (argv[arg_index as usize]).parse().unwrap_or(0);
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-l" {
            (g_state.config).tournament = 0;
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                (g_state.config).skill[0] =
                    (argv[arg_index as usize]).parse().unwrap_or(0);
                if (g_state.config).skill[0] > 0 {
                    if arg_index + 2 >= argc {
                        help = 1;
                        current_block_107 = 2668756484064249700;
                    } else {
                        arg_index += 1;
                        (g_state.config).exact_skill[0] =
                            (argv[arg_index as usize]).parse().unwrap_or(0);
                        arg_index += 1;
                        (g_state.config).wld_skill[0] =
                            (argv[arg_index as usize]).parse().unwrap_or(0);
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
                            (g_state.config).skill[2] =
                                (argv[arg_index as usize]).parse().unwrap_or(0);
                            if (g_state.config).skill[2] >
                                   0 {
                                if arg_index + 2 >= argc {
                                    help = 1;
                                    current_block_107 = 2668756484064249700;
                                } else {
                                    arg_index += 1;
                                    (g_state.config).exact_skill[2] = argv[arg_index as usize].parse().unwrap_or(0);
                                    arg_index += 1;
                                    (g_state.config).wld_skill[2] = argv[arg_index as usize].parse().unwrap_or(0);
                                    current_block_107 = 10485226111480991281;
                                }
                            } else {
                                current_block_107 = 10485226111480991281;
                            }
                        }
                    }
                }
            }
        } else if argv[arg_index as usize] == "-t" {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                (g_state.config).tournament = 1;
                (g_state.config).tournament_levels = (argv[arg_index as usize]).parse().unwrap_or(0);
                if arg_index + 3 * (g_state.config).tournament_levels >= argc {
                    help = 1;
                    current_block_107 = 2668756484064249700;
                } else {
                    i = 0;
                    while i < (g_state.config).tournament_levels {
                        j = 0;
                        while j < 3 {
                            arg_index += 1;
                            (g_state.config).tournament_skill[i as usize][j as usize] =
                                (argv[arg_index as usize]).parse().unwrap_or(0);
                            j += 1
                        }
                        i += 1
                    }
                    current_block_107 = 10485226111480991281;
                }
            }
        } else if argv[arg_index as usize] == "-w" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                (g_state.config).wait = (argv[arg_index as usize]).parse().unwrap_or(0);
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-p" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                (g_state.config).display_pv = (argv[arg_index as usize]).parse().unwrap_or(0);
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "?" {
            help = 1;
            current_block_107 = 10485226111480991281;
        } else if argv[arg_index as usize] == "-g" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                game_file_name = argv.offset(arg_index as isize);
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-r" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                use_random = (argv[arg_index as usize]).parse().unwrap_or(0);
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-b" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                (g_state.config).use_book = (argv[arg_index as usize]).parse().unwrap_or(0);
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-time" {
            if arg_index + 4 >= argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                arg_index += 1;
                (g_state.config).player_time[0] =
                    (argv[arg_index as usize]).atoi() as f64;
                arg_index += 1;
                (g_state.config).player_increment[0] =
                    (argv[arg_index as usize]).atoi() as f64;
                arg_index += 1;
                (g_state.config).player_time[2] =
                    (argv[arg_index as usize]).atoi() as f64;
                arg_index += 1;
                (g_state.config).player_increment[2] =
                    (argv[arg_index as usize]).atoi() as f64;
                (g_state.config).use_timer = 1;
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-learn" {
            if arg_index + 2 >= argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                arg_index += 1;
                (g_state.config).deviation_depth = (argv[arg_index as usize]).parse().unwrap_or(0);
                arg_index += 1;
                (g_state.config).cutoff_empty = (argv[arg_index as usize]).parse().unwrap_or(0);
                g_state.config.use_learning = true;
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-slack" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                (g_state.config).slack = (argv[arg_index as usize]).parse().unwrap_or(0.0);
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-dev" {
            if arg_index + 3 >= argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                arg_index += 1;
                (g_state.config).low_thresh = (argv[arg_index as usize]).parse().unwrap_or(0);
                arg_index += 1;
                (g_state.config).high_thresh = (argv[arg_index as usize]).parse().unwrap_or(0);
                arg_index += 1;
                (g_state.config).dev_bonus = (argv[arg_index as usize]).parse().unwrap_or(0.0);
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-log" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                log_file_name = argv.offset(arg_index as isize);
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-private" {
            (g_state.g_book).set_game_mode(PRIVATE_GAME);
            current_block_107 = 10485226111480991281;
        } else if argv[arg_index as usize] == "-public" {
            (g_state.g_book).set_game_mode(PUBLIC_GAME);
            current_block_107 = 10485226111480991281;
        } else if argv[arg_index as usize] == "-keepdraw" {
            (g_state.g_book).set_draw_mode(NEUTRAL);
            current_block_107 = 10485226111480991281;
        } else if argv[arg_index as usize] == "-draw2black" {
            (g_state.g_book).set_draw_mode(BLACK_WINS);
            current_block_107 = 10485226111480991281;
        } else if argv[arg_index as usize] == "-draw2white" {
            (g_state.g_book).set_draw_mode(WHITE_WINS);
            current_block_107 = 10485226111480991281;
        } else if argv[arg_index as usize] == "-draw2none" {
            (g_state.g_book).set_draw_mode(OPPONENT_WINS);
            current_block_107 = 10485226111480991281;
        } else if argv[arg_index as usize] == "-test" {
            (g_state.config).one_position_only = 1;
            current_block_107 = 10485226111480991281;
        } else if argv[arg_index as usize] == "-seq" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                move_sequence = (argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-seqfile" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                move_file_name = argv.offset(arg_index as isize);
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-repeat" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                repeat = (argv[arg_index as usize]).parse().unwrap_or(0);
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-thor" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                g_state.config.use_thor = true;
                (g_state.config).thor_max_games = (argv[arg_index as usize]).parse().unwrap_or(0);
                current_block_107 = 10485226111480991281;
            }
        } else if argv[arg_index as usize] == "-analyze" {
            (g_state.config).only_analyze = 1;
            current_block_107 = 10485226111480991281;
        } else if argv[arg_index as usize] == "-randmove" {
            arg_index += 1;
            if arg_index == argc {
                help = 1;
                current_block_107 = 2668756484064249700;
            } else {
                (g_state.config).rand_move_freq = (argv[arg_index as usize]).parse().unwrap_or(0);
                if (g_state.config).rand_move_freq < 0 {
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
                if arg_index >= argc { help = 1 }
            }
            _ => { }
        }
        arg_index += 1
    }
    if help != 0 {
        println!(r"Usage:
  zebra [-b -e -g -h -l -p -t -time -w -learn -slack -dev -log
         -keepdraw -draw2black -draw2white -draw2none
         -private -public -test -seq -thor -script -analyze ?
         -repeat -seqfile]

Flags:
  ?{space}
    Displays this text.

  -b <use book?>
    Toggles usage of opening book on/off (default {default_use_book}).

  -e <echo?>
    Toggles screen output on/off (default {default_echo}).

  -g <game file>

  -h <bits in hash key>
    Size of hash table is 2^{{this value}} (default {default_hash_bits}).

  -l <black depth> [<black exact depth> <black WLD depth>]
     <white depth> [<white exact depth> <white WLD depth>]
    Sets the search depth. If <black depth> or <white depth> are set to 0, a
    human player is assumed. In this case the other parameters must be omitted.
    <* exact depth> specify the number of moves before the (at move 60) when
    the exact game-theoretical value is calculated. <* WLD depth> are used
    analogously for the calculation of Win/Loss/Draw.

  -p <display principal variation?>
    Toggles output of principal variation on/off (default {default_pv}).

  -r <use randomization?>
    Toggles randomization on/off (default {default_random})

  -t <number of levels> <(first) depth> ... <(last) wld depth>

  -time <black time> <black increment> <white time> <white increment>
    Tournament mode; the format for the players is as above.

  -w <wait?>
    Toggles wait between moves on/off (default {default_wait}).

  -learn <depth> <cutoff>
    Learn the game with <depth> deviations up to <cutoff> empty.

  -slack <disks>
    Zebra's opening randomness is <disks> disks (default {default_slack:.6}).

  -dev <low> <high> <bonus>
    Give deviations before move <high> a <bonus> disk bonus but
    don't give any extra bonus for deviations before move <low>.

  -log <file name>
    Append all game results to the specified file.

  -private
    Treats all draws as losses for both sides.

  -public
    No tweaking of draw scores.

  -keepdraw
    Book draws are counted as draws.

  -draw2black
    Book draws scored as 32-31 for black.

  -draw2white
    Book draws scored as 32-31 for white.

  -draw2none
    Book draws scored as 32-31 for the opponent.

  -test
    Only evaluate one position, then exit.

  -seq <move sequence>
    Forces the game to start with a predefined move sequence;
    e.g. f4d6c3.

  -seqfile <filename
    Specifies a file from which move sequences are read.

  -thor <game count>
    Look for each position in the Thor database; list the first <game count>.

  -script <script file> <output file>
    Solves all positions in script file for exact score.

  -wld <only solve WLD?>
    Toggles WLD only solve on/off (default {default_wld}).

  -analyze
    Used in conjunction with -seq; all positions are analyzed.
  -repeat <#iterations>
    Repeats the operation the specified number of times.{space}
",
                 default_use_book = 1,
                 default_echo = 1,
                 default_hash_bits = 18,
                 default_pv = 1,
                 default_random = 1,
                 default_wait = 0,
                 default_slack = 0.25f64,
                 default_wld = 0,
                 // This is kindof a hack to get around CLion stripping trailing
                 // whitespace from the format string - todo report this
                 space = ' ');
        exit(1);
    }
    if hash_bits < 1 {
        write!(stdout, "Hash table key must contain at least 1 bit\n");
        exit(1);
    }
    global_setup(use_random, hash_bits,&mut g_state);
    init_thor_database(&mut g_state.random);
    if (g_state.config).use_book != 0 {
        let file_name = if let Ok(var) = std::env::var("BOOK_PATH") {
            CString::new(var).unwrap()
        } else {
            CString::new("book.bin").unwrap()
        };
        init_learn(file_name.as_ref().as_ptr() as *const u8 as *const i8,
                   1, &mut g_state);
    }
    if use_random != 0 {
        time(&mut timer);
        let x = timer as i32;
        (g_state.random).my_srandom(x);
    } else {
        let x = 1;
        (g_state.random).my_srandom(x); }
    if (g_state.config).tournament == 0 {
        while (g_state.config).skill[0] < 0 {
            write!(stdout, "Black parameters: ");
            scanf(b"%d\x00" as *const u8 as *const i8,
                  &mut *(g_state.config).skill.as_mut_ptr()
                      as *mut i32);
            if (g_state.config).skill[0] > 0 {
                scanf(b"%d %d\x00" as *const u8 as *const i8,
                      &mut *(g_state.config).exact_skill.as_mut_ptr() as
                          *mut i32,
                      &mut *(g_state.config).wld_skill.as_mut_ptr() as
                          *mut i32);
            }
        }
        while (g_state.config).skill[2] < 0 {
            write!(stdout, "White parameters: ");
            scanf(b"%d\x00" as *const u8 as *const i8,
                  &mut *(g_state.config).skill.as_mut_ptr().offset(2)
                      as *mut i32);
            if (g_state.config).skill[2] > 0 {
                scanf(b"%d %d\x00" as *const u8 as *const i8,
                      &mut *(g_state.config).exact_skill.as_mut_ptr().offset(2) as
                          *mut i32,
                      &mut *(g_state.config).wld_skill.as_mut_ptr().offset(2) as
                          *mut i32);
            }
        }
    }
    if (g_state.config).one_position_only != 0 {
        display_state.toggle_smart_buffer_management(0);
    }
    if (g_state.config).tournament != 0 {
        play_tournament(move_sequence, log_file_name, g_state);
    } else if (g_state.config).only_analyze != 0 {
        analyze_game(move_sequence, &mut g_state);
    } else {
        play_game(game_file_name, move_sequence, move_file_name, repeat, log_file_name, g_state);
    }
    0
}
/*
   PLAY_TOURNAMENT
   Administrates the tournament between different levels
   of the program.
*/
unsafe fn play_tournament(move_sequence: &str, log_file_name_: &str, mut g_state: FullState) {
    let mut result: [[[i32; 3]; 8]; 8] = [[[0; 3]; 8]; 8];
    let mut tourney_time: f64 = 0.;
    let mut score: [f64; 8] = [0.; 8];
    let mut color_score: [f64; 3] = [0.; 3];
    let mut tourney_nodes = CounterType{hi: 0, lo: 0,};
    reset_counter(&mut tourney_nodes);
    tourney_time = 0.0f64;
    let mut i = 0;
    while i < 8 {
        score[i as usize] = 0.0f64;
        i += 1
    }
    color_score[2] = 0.0f64;
    color_score[0] = color_score[2];

    let mut i = 0;
    let mut j = 0;
    let tournament_levels_ = (g_state.config).tournament_levels;
    while i < tournament_levels_ {
        j = 0;
        while j < tournament_levels_ {
            (g_state.config).skill[0] = (g_state.config).tournament_skill[i as usize][0];
            (g_state.config).exact_skill[0] = (g_state.config).tournament_skill[i as usize][1];
            (g_state.config).wld_skill[0] = (g_state.config).tournament_skill[i as usize][2];
            (g_state.config).skill[2] = (g_state.config).tournament_skill[j as usize][0];
            (g_state.config).exact_skill[2] = (g_state.config).tournament_skill[j as usize][1];
            (g_state.config).wld_skill[2] = (g_state.config).tournament_skill[j as usize][2];
            g_state = play_game("", move_sequence,
                      "", 1, log_file_name_, g_state);
            add_counter(&mut tourney_nodes, &mut (g_state.search).total_nodes);
            tourney_time += (&mut g_state.search).total_time;
            result[i as usize][j as usize][0] =
                disc_count(0, &( g_state.board).board);
            result[i as usize][j as usize][2] =
                disc_count(2, &( g_state.board).board);
            if disc_count(0, &( g_state.board).board) > disc_count(2, &( g_state.board).board) {
                score[i as usize] += 1.0f64;
                color_score[0] += 1.0f64
            } else if disc_count(0, &(&mut g_state.board).board) ==
                          disc_count(2, &(&mut g_state.board).board) {
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
    let tournament_skill_ = &&mut (g_state.config).tournament_skill;
    let tourney_counter_value = counter_value(&mut tourney_nodes);

    write!(stdout, "\n\nTime:  {:.1} s\nNodes: {:.0}\n", tourney_time,
           tourney_counter_value);
    write!(stdout, "\nCompetitors:\n");
    let mut i = 0;
    while i < tournament_levels_ {
        write!(stdout, "  Player {:2}: {}-{}-{}\n", i + 1,
               tournament_skill_[i as usize][0],
               tournament_skill_[i as usize][1],
               tournament_skill_[i as usize][2]);
        i += 1
    }
    write!(stdout, "\n       ");
    let mut i = 0;
    while i < tournament_levels_ {
        write!(stdout, " {:2}    ",
               i + 1);
        i += 1
    }
    write!(stdout, "  Score\n");
    let mut i = 0;
    let mut j = 0;
    while i < tournament_levels_ {
        write!(stdout, "  {:2}   ",
               i + 1);
        j = 0;
        while j < tournament_levels_ {
            write!(stdout, "{:2}-{:2}  ",
                   result[i as usize][j as usize][0],
                   result[i as usize][j as usize][2]);
            j += 1
        }
        write!(stdout, "  {:4.1}\n", score[i as usize]);
        i += 1
    }
    write!(stdout, "\n");
    write!(stdout, "Black score: {:.1}\n",
           color_score[0]);
    write!(stdout, "White score: {:.1}\n",
           color_score[2]);
    write!(stdout, "\n");
}

impl InitialMoveSource for FileMoveSource {
    fn fill_line_buffer(&mut self, line_buffer: &mut [u8]) {
        self.move_file.read(line_buffer.as_mut());

        let newline_pos = line_buffer.iter()
            .enumerate()
            .find(|(_i, ch)| **ch == '\n' as i8 as u8 );

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
fn play_game(mut file_name: &str,
                    mut move_string: &str,
                    mut move_file_name: &str,
                    mut repeat: i32,
                    log_file_name_: &str,
                    g_state: FullState
) -> FullState {
    let move_file = if move_file_name.is_empty() {
        None
    } else {
        FileMoveSource::open(move_file_name)
    };
    let file_name: Option<CString> = (!file_name.is_empty()).then(|| CString::new(file_name).unwrap());
    let log_file_name_: Option<CString> = (!log_file_name_.is_empty()).then(|| CString::new(log_file_name_).unwrap());

    let move_string = if move_string.is_empty() {
        vec![]
    } else {
        move_string.as_bytes().into()
    };
    type ZF = LibcFrontend;
    type Source = FileMoveSource;
    type BoardSrc = BasicBoardFileSource;
    type ComputeMoveLog = LogFileHandler;
    type ComputeMoveOut = LibcZebraOutput;
    type Learn = LibcLearner;
    type FE = LibcFatalError;
    type Thor = LegacyThor;
    let mut play_state: PlayGame<Source> = PlayGame::new(file_name, move_string, repeat, move_file, g_state);
    let mut move_attempt = None;
    let mut total_search_time: f64 = 0.;

    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    loop {
        let state = next_state::<
            ZF, Source, BoardSrc, ComputeMoveLog, ComputeMoveOut, FE, Thor
        >(&mut play_state, move_attempt.take(), &LegacyThor);
        match state {
            // TODO here in all these branches, we should ideally not need mutable reference to play_state
            PlayGameState::End => {
                return play_state.g_state;
            }
            PlayGameState::Dumpch { provided_move_count, move_start } => {
                dumpch(&mut stdin_lock);
            }
            PlayGameState::GetPass { provided_move_count } => {
                write!(stdout, "You must pass - please press Enter\n");
                dumpch(&mut stdin_lock);
            }
            PlayGameState::GettingMove { provided_move_count, move_start, side_to_move } => {
                let res = ZF::prompt_get_move(side_to_move, &mut stdin_lock);
                move_attempt = Some(MoveAttempt(res.0 as i8, res.1 as i8))
            }
            PlayGameState::NeedsDump {..} => {
                if play_state.g_state.config.echo != 0 {
                    ZF::set_move_list(play_state.g_state.board.score_sheet_row);
                    ZF::set_times(floor(play_state.g_state.config.player_time[0]) as i32,
                                  floor(play_state.g_state.config.player_time[2]) as i32);
                    let opening_name = find_opening_name(&play_state.g_state.g_book, &play_state.g_state.board.board);
                    if let Some(opening_name) = opening_name {
                        let opening_name = CStr::from_bytes_with_nul(opening_name).unwrap();
                        write!(stdout, "\nOpening: {}\n", opening_name.to_str().unwrap());
                    }
                    deal_with_thor_1(play_state.g_state.config.use_thor,
                                     play_state.side_to_move,
                                     &play_state.g_state.config,
                                     &play_state.g_state.timer,
                                     &play_state.g_state.board,
                                     &mut total_search_time);

                    ZF::display_board_after_thor(play_state.side_to_move, play_state.g_state.config.use_timer,
                                                 &play_state.g_state.board.board,
                                                 &play_state.g_state.board.black_moves,
                                                 &play_state.g_state.board.white_moves);
                }
                dump_position(play_state.side_to_move, &play_state.g_state.board.board);
                dump_game_score(play_state.side_to_move, play_state.g_state.board.score_sheet_row, &play_state.g_state.board.black_moves, &play_state.g_state.board.white_moves);
                /* Check what the Thor opening statistics has to say */
                Thor::choose_thor_opening_move(&play_state.g_state.board.board, play_state.side_to_move, play_state.g_state.config.echo, &mut play_state.g_state.random);
            }
            PlayGameState::CanLearn => {
                if play_state.g_state.config.use_learning && play_state.g_state.config.one_position_only == 0 {
                    play_state.g_state.timer.toggle_abort_check(0);
                    Learn::learn_game(play_state.g_state.moves.disks_played,
                                      (play_state.g_state.config.skill[0] != 0 && play_state.g_state.config.skill[2] != 0) as i32,
                                      (play_state.repeat == 0) as i32, &mut play_state.g_state);
                    play_state.g_state.timer.toggle_abort_check(1);
                }
            }
            PlayGameState::AfterGameReport { node_val, eval_val } => {
                if play_state.g_state.config.echo == 0 && play_state.g_state.config.one_position_only == 0 {
                    let black_level = play_state.g_state.config.skill[0];
                    let white_level = play_state.g_state.config.skill[2];
                    ZF::report_skill_levels(black_level, white_level);
                }
                dump_game_score(play_state.side_to_move, play_state.g_state.board.score_sheet_row, &play_state.g_state.board.black_moves, &play_state.g_state.board.white_moves);
                if play_state.g_state.config.echo != 0 && play_state.g_state.config.one_position_only == 0 {
                    ZF::set_move_list(
                        play_state.g_state.board.score_sheet_row);
                    deal_with_thor_2(play_state.g_state.config.use_thor, play_state.side_to_move,
                                     &play_state.g_state.config,
                                     &play_state.g_state.timer,
                                     &play_state.g_state.board,
                                     &mut total_search_time);
                    ZF::set_times(floor(play_state.g_state.config.player_time[0]) as _, floor(play_state.g_state.config.player_time[2]) as _);
                    ZF::display_board_after_thor(play_state.side_to_move, play_state.g_state.config.use_timer, &play_state.g_state.board.board,
                                                 &play_state.g_state.board.black_moves,
                                                 &play_state.g_state.board.white_moves,
                    );
                }
                let black_disc_count = disc_count(0, &play_state.g_state.board.board);
                let white_disc_count = disc_count(2, &play_state.g_state.board.board);
                let total_time_ = play_state.g_state.search.total_time;
                ZF::report_after_game_ended(node_val, eval_val, black_disc_count, white_disc_count, total_time_);

                if let (Some(log_file_name_), 0) = (log_file_name_.as_ref(), play_state.g_state.config.one_position_only) {
                    ZF::log_game_ending((&*log_file_name_),
                                        &play_state.move_vec,
                                        disc_count(0, &play_state.g_state.board.board),
                                        disc_count(2, &play_state.g_state.board.board))
                }
            }
            _ => {}
        }
    }
}

fn deal_with_thor_1(use_thor_: bool, side_to_move: i32,
                                                           mut config: &Config, g_timer: &Timer,
                                                           board_state: &BoardState, total_search_time: &mut f64) {
    type ZF = LibcFrontend;
    type Thor = LegacyThor;

    if use_thor_ {
        let database_start = g_timer.get_real_timer();
        Thor::database_search(&board_state.board, side_to_move);
        let thor_position_count = Thor::get_match_count();
        let database_stop = g_timer.get_real_timer();
        let database_time = database_stop - database_start;
        *total_search_time += database_time;
        ZF::report_thor_matching_games_stats(*total_search_time, thor_position_count, database_time);
        if thor_position_count > 0 {
            let black_win_count = Thor::get_black_win_count();
            let draw_count = Thor::get_draw_count();
            let white_win_count = Thor::get_white_win_count();
            let black_median_score = Thor::get_black_median_score();
            let black_average_score = Thor::get_black_average_score();

            ZF::report_thor_stats(black_win_count, draw_count, white_win_count, black_median_score, black_average_score);
        }
        ZF::print_out_thor_matches(config.thor_max_games);
    }
}

fn deal_with_thor_2(use_thor_: bool, side_to_move: i32,
                                                          config: &Config, g_timer: &Timer,
                                                          board_state: &BoardState, total_search_time: &mut f64){
    type ZF = LibcFrontend;
    type Thor = LegacyThor;
    if use_thor_ {
        let database_start = g_timer.get_real_timer();
        Thor::database_search(&board_state.board, side_to_move);
        let thor_position_count = Thor::get_match_count();
        let database_stop = g_timer.get_real_timer();
        let db_search_time = database_stop - database_start;
        *total_search_time += db_search_time;
        ZF::report_some_thor_stats(*total_search_time, thor_position_count, db_search_time);
        if thor_position_count > 0 {
            let black_win_count = Thor::get_black_win_count();
            let draw_count = Thor::get_draw_count();
            let white_win_count = Thor::get_white_win_count();
            let black_median_score = Thor::get_black_median_score();
            let black_average_score = Thor::get_black_average_score();
            ZF::report_some_thor_scores(black_win_count, draw_count, white_win_count, black_median_score, black_average_score);
        }
        ZF::print_out_thor_matches(config.thor_max_games);
    }
}


pub struct LibcFrontend {} //TODO this could probably be merged with the FrontEnd trait or something
impl LibcFrontend {

    fn set_times(black: i32, white: i32) {
        unsafe { display_state.set_times(black, white) }
    }

    fn report_some_thor_scores(black_win_count: i32, draw_count: i32, white_win_count: i32, black_median_score: i32, black_average_score: f64) {
        write!(stdout, "{} black wins,  {} draws,  {} white wins\n", black_win_count, draw_count, white_win_count);
        write!(stdout, "Median score {}-{}\n", black_median_score, 64 - black_median_score);
        write!(stdout, ", average score {:.2}-{:.2}\n", black_average_score, 64.0f64 - black_average_score);
    }

    fn report_some_thor_stats(total_search_time: f64, thor_position_count: i32, db_search_time: f64) {
        write!(stdout, "{} matching games  ({:.3} s search time, {:.3} s total)\n",
               thor_position_count, db_search_time, total_search_time);
    }

    fn display_board_after_thor(side_to_move: i32, give_time_: i32, board_: &[i32; 128],
                                black_moves_: &[i8; 60], white_moves_: &[i8; 60]) {
        unsafe {
            display_state.display_board(&mut stdout, board_,
                          side_to_move, 1, give_time_, 1,
                          black_moves_, white_moves_);
        }
    }

    fn print_out_thor_matches(thor_max_games_: i32) {
        unsafe { print_thor_matches(&mut stdout, thor_max_games_); }
    }

    fn log_game_ending(log_file_name_: &CStr, move_vec: &[i8; 122],
                       first_side_to_move: i32, second_side_to_move: i32) {
        let log_file_name_ = log_file_name_.to_str().unwrap();
        unsafe {
            let log_file = File::options().append(true).create(true).open(log_file_name_);

            if let Ok(mut log_file) = log_file {
                let mut timer = time(0 as *mut time_t);
                write!(log_file,
                        "# {}#     {:2} - {:2}\n" , c_time(timer),
                        first_side_to_move,
                        second_side_to_move);
                for &c in move_vec.into_iter().take_while(|&&c| c != 0) {
                    log_file.write(&[c as _]);
                }
                log_file.write(b"\n");
                drop(log_file);
            }
        }
    }

    fn prompt_get_move(side_to_move: i32, stdin_lock : &mut StdinLock) -> (i32, i32) {
        let mut buffer1 = [0; 4];
        let mut buffer = &mut buffer1[0..3];
        if side_to_move == 0 {
            write!(stdout, "{}: ", "Black move");
        } else {
            write!(stdout, "{}: ", "White move");
        }
        loop {
            match stdin_lock.fill_buf() {
                Ok(&[next, ..])  if next.is_ascii_whitespace() => {
                    stdin_lock.consume(1);
                }
                _ => break
            }
        }
        for current in buffer.chunks_exact_mut(1) {
            match stdin_lock.fill_buf() {
                Ok(&[next, ..])  if !next.is_ascii_whitespace() => {
                    current[0] = next;
                    stdin_lock.consume(1);
                }
                _ => break
            }
        }

        let curr_move = buffer1.atoi();
        let curr_move_2 = buffer1[0] as i32 - 'a' as i32 + 1 + 10 * (buffer1[1] as i32 - '0' as i32);
        (curr_move, curr_move_2)
    }

    fn report_after_game_ended(node_val: f64, eval_val: f64, black_disc_count: i32, white_disc_count: i32, total_time_: f64) {
        write!(stdout, "\nBlack: {}   White: {}\n", black_disc_count, white_disc_count);
        write!(stdout, "Nodes searched:        {:<-10.0}\n", node_val);
        write!(stdout, "Positions evaluated:   {:<-10.0}\n", eval_val);
        write!(stdout, "Total time: {:.1} s\n", total_time_);
    }

    fn report_skill_levels(black_level: i32, white_level: i32) {
        write!(stdout, "\n");
        write!(stdout, "Black level: {}\n", black_level);
        write!(stdout, "White level: {}\n", white_level);
    }

    fn report_thor_matching_games_stats(total_search_time: f64, thor_position_count: i32, database_time: f64) {
        write!(stdout, "{} matching games  ({:.3} s search time, {:.3} s total)\n",
               thor_position_count, database_time, total_search_time);
    }

    fn report_thor_stats(black_win_count: i32, draw_count: i32, white_win_count: i32, black_median_score: i32, black_average_score: f64) {
        write!(stdout, "{} black wins, {} draws, {} white wins\n", black_win_count, draw_count, white_win_count);
        write!(stdout, "Median score {}-{}", black_median_score, 64 - black_median_score);
        write!(stdout, ", average score {:.2}-{:.2}\n", black_average_score, 64.0f64 - black_average_score);
    }
}
impl ZebraFrontend for LibcFrontend {
    fn set_evals(black: f64, white: f64) {
        unsafe { display_state.set_evals(black, white); }
    }

    fn set_move_list(row: i32) {
        unsafe { display_state.set_move_list(row) }
    }

    fn set_names(white_is_player: bool, black_is_player: bool) {
        let black_name = if white_is_player {
            "Player"
        } else {
            "Zebra"
        };
        let white_name = if black_is_player {
            "Player"
        } else {
            "Zebra"
        };
        unsafe { display_state.set_names(black_name, white_name) }
    }

    fn report_engine_override() {
        write!(stdout, "Engine override: Random move selected.\n");
    }

    fn before_get_move() {
        write!(stdout, "\n");
    }

    fn report_book_randomness(slack_: f64) {
        write!(stdout, "Book randomness: {:.2} disks\n", slack_);
    }
    fn load_thor_files(g_timer: &mut Timer) { unsafe {
        /* No error checking done as it's only for testing purposes */
        let database_start =  g_timer.get_real_timer();
        read_player_database("thor/wthor.jou");
        read_tournament_database("thor/wthor.trn");
        read_game_database("thor/wth_2001.wtb");
        read_game_database("thor/wth_2000.wtb");
        read_game_database("thor/wth_1999.wtb");
        read_game_database("thor/wth_1998.wtb");
        read_game_database("thor/wth_1997.wtb");
        read_game_database("thor/wth_1996.wtb");
        read_game_database("thor/wth_1995.wtb");
        read_game_database("thor/wth_1994.wtb");
        read_game_database("thor/wth_1993.wtb");
        read_game_database("thor/wth_1992.wtb");
        read_game_database("thor/wth_1991.wtb");
        read_game_database("thor/wth_1990.wtb");
        read_game_database("thor/wth_1989.wtb");
        read_game_database("thor/wth_1988.wtb");
        read_game_database("thor/wth_1987.wtb");
        read_game_database("thor/wth_1986.wtb");
        read_game_database("thor/wth_1985.wtb");
        read_game_database("thor/wth_1984.wtb");
        read_game_database("thor/wth_1983.wtb");
        read_game_database("thor/wth_1982.wtb");
        read_game_database("thor/wth_1981.wtb");
        read_game_database("thor/wth_1980.wtb");
        let database_stop =  g_timer.get_real_timer();
        write!(stdout, "Loaded {} games in {:.3} s.\n", get_total_game_count(), database_stop - database_start);
        write!(stdout, "Each Thor game occupies {} bytes.\n", get_thor_game_size());
    }}

    fn print_move_alternatives(side_to_move: i32, mut board_state: &mut BoardState, mut g_book: &mut Book) {
        print_move_alternatives(side_to_move, board_state, g_book)
    }
}

/*
   ANALYZE_GAME
   Analyzes all positions arising from a given move sequence.
*/
unsafe fn analyze_game(mut move_string: &str, g_state : &mut FullState) {
    let mut best_info1 =  EvaluationType::new();
    let mut best_info2 =  EvaluationType::new();
    let mut played_info1 =  EvaluationType::new();
    let mut played_info2 =  EvaluationType::new();
    let mut move_start: f64 = 0.;
    let mut move_stop: f64 = 0.;
    let mut i: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut opponent: i32 = 0;
    let mut curr_move = 0;
    let mut resp_move = 0;
    let mut timed_search: i32 = 0;
    let mut provided_move_count = 0;
    let mut col = 0;
    let mut row = 0;
    let mut empties: i32 = 0;
    let mut provided_move: [i8; 61] = [0; 61];
    /* Decode the predefined move sequence */
    if move_string.is_empty() {
        provided_move_count = 0
    } else {
        provided_move_count = (move_string.len()).wrapping_div(2) as i32;
        if provided_move_count > 60 || (move_string.len()).wrapping_rem(2) == 1 {
            LibcFatalError::invalid_move_string_provided();
        }
        i = 0;
        let move_string = move_string.as_bytes();
        while i < provided_move_count {
            col = (*move_string.offset((2 * i) as isize) as char).to_ascii_lowercase() as u8 - b'a' + 1;
            row = *move_string.offset((2 * i + 1) as isize) - b'0';
            if col < 1 || col > 8 || row < 1 || row > 8 {
                LibcFatalError::unexpected_character_in_a_move_string();
            }
            provided_move[i as usize] = (10 * row + col) as i8;
            i += 1
        }
    }
    /* Open the output log file */
    let mut output_stream = match File::options().create(true).write(true).truncate(true).open("analysis.log") {
        Ok(s) => s,
        Err(_) => fatal_error!("Can\'t create log file analysis.log - aborting")
    };
    /* Set up the position and the search engine */
    if (&mut g_state.config).echo != 0 {
        write!(stdout, "Analyzing provided game...\n");
    }
    generic_game_init::<BasicBoardFileSource, LibcFatalError>(None, &mut side_to_move, g_state);
    setup_hash(1, &mut (&mut g_state.hash), &mut (&mut g_state.random));
    (&mut g_state.learn).clear_stored_game();
    if (&mut g_state.config).echo != 0 && (&mut g_state.config).use_book != 0 {
        write!(stdout, "Disabling usage of opening book\n");
    }
    (&mut g_state.config).use_book = 0;
    reset_book_search(&mut (&mut g_state.g_book));
    let black_name = "Zebra";
    let white_name = "Zebra";
    display_state.set_names(black_name, white_name);
    display_state.set_move_list((g_state.board).score_sheet_row);
    display_state.set_evals(0.0f64, 0.0f64);
    g_state.board.black_moves = [-1; 60];
    g_state.board.white_moves = [-1; 60];
    let _black_hash1 = (g_state.random).my_random() as i32;
    let _black_hash2 = (g_state.random).my_random() as i32;
    let _white_hash1 = (g_state.random).my_random() as i32;
    let _white_hash2 = (g_state.random).my_random() as i32;
    let best_trans1 = (g_state.random).my_random() as u32;
    let best_trans2 = (g_state.random).my_random() as u32;
    let played_trans1 = (g_state.random).my_random() as u32;
    let played_trans2 = (g_state.random).my_random() as u32;
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    while game_in_progress((&mut g_state.moves), &(g_state.search), &(g_state.board).board) != 0 && (g_state.moves).disks_played < provided_move_count {
        remove_coeffs((g_state.moves).disks_played, &mut ( g_state.coeff));
        generate_all(side_to_move, (&mut g_state.moves), &(g_state.search), &(g_state.board).board);
        if side_to_move == 0 { (g_state.board).score_sheet_row += 1 }
        if (g_state.moves).move_count[(g_state.moves).disks_played as usize] != 0 {
            move_start =  (&mut g_state.timer).get_real_timer();
            (&mut g_state.timer).clear_panic_abort();
            if (&mut g_state.config).echo != 0 {
                display_state.set_move_list((g_state.board).score_sheet_row);
                display_state.set_times(floor((&mut g_state.config).player_time[0]) as i32,
                                        floor((&mut g_state.config).player_time[2]) as i32);
                let opening_name = find_opening_name(&mut (&mut g_state.g_book), &(g_state.board).board);
                if let Some(opening_name) = opening_name {
                    write!(stdout, "\nOpening: {}\n", CStr::from_bytes_with_nul(opening_name).map(CStr::to_str).unwrap().unwrap());
                }
                display_state.display_board(&mut stdout, &(g_state.board).board, side_to_move,
                                            1, (&mut g_state.config).use_timer, 1,
                                            &(g_state.board).black_moves, &(g_state.board).white_moves);
            }
            /* Check what the Thor opening statistics has to say */
            choose_thor_opening_move(&(g_state.board).board, side_to_move, (&mut g_state.config).echo, &mut (&mut g_state.random));
            if (&mut g_state.config).echo != 0 && (&mut g_state.config).wait != 0 {
                dumpch(&mut stdin_lock);
            }
             (&mut g_state.timer).start_move((&mut g_state.config).player_time[side_to_move as usize],
                                             (&mut g_state.config).player_increment[side_to_move as usize],
                                             (g_state.moves).disks_played + 4);
            (&mut g_state.timer).determine_move_time((&mut g_state.config).player_time[side_to_move as usize],
                                                     (&mut g_state.config).player_increment[side_to_move as usize],
                                                     (g_state.moves).disks_played + 4);
            timed_search = ((&mut g_state.config).skill[side_to_move as usize] >= 60) as i32;
            empties = 60 - (g_state.moves).disks_played;
            /* Determine the score for the move actually played.
               A private hash transformation is used so that the parallel
            search trees - "played" and "best" - don't clash. This way
             all scores are comparable. */
            (&mut g_state.hash).set_hash_transformation(played_trans1, played_trans2);
            curr_move = provided_move[(g_state.moves).disks_played as usize];
            opponent = 0 + 2 - side_to_move;
            make_move(side_to_move, curr_move, 1, (&mut g_state.moves), &mut (&mut g_state.board), &mut (&mut g_state.hash), &mut (&mut g_state.flip_stack));
            if empties > (&mut g_state.config).wld_skill[side_to_move as usize] {
                reset_counter(&mut (&mut g_state.search).nodes);
                resp_move = legacy_compute_move(opponent, 0,
                                                (&mut g_state.config).player_time[opponent as usize] as i32,
                                                (&mut g_state.config).player_increment[opponent as usize] as i32,
                                                timed_search, (&mut g_state.config).use_book,
                                                (&mut g_state.config).skill[opponent as usize] - 2,
                                                (&mut g_state.config).exact_skill[opponent as usize] - 1,
                                                (&mut g_state.config).wld_skill[opponent as usize] - 1,
                                                1, &mut played_info1, g_state)
            }
            reset_counter(&mut (&mut g_state.search).nodes);
            resp_move = legacy_compute_move(opponent, 0,
                                            (&mut g_state.config).player_time[opponent as usize] as i32,
                                            (&mut g_state.config).player_increment[opponent as usize] as i32,
                                            timed_search, (&mut g_state.config).use_book,
                                            (&mut g_state.config).skill[opponent as usize] - 1,
                                            (&mut g_state.config).exact_skill[opponent as usize] - 1,
                                            (&mut g_state.config).wld_skill[opponent as usize] - 1,
                                            1, &mut played_info2, g_state);
            unmake_move(side_to_move, curr_move, &mut (g_state.board).board, (&mut g_state.moves), &mut (&mut g_state.hash), &mut (&mut g_state.flip_stack));
            /* Determine the 'best' move and its score. For midgame moves,
            search twice to dampen oscillations. Unless we're in the endgame
             region, a private hash transform is used - see above. */
            if empties > (&mut g_state.config).wld_skill[side_to_move as usize] {
                (&mut g_state.hash).set_hash_transformation(best_trans1, best_trans2);
                reset_counter(&mut (&mut g_state.search).nodes);
                curr_move = legacy_compute_move(side_to_move, 0,
                                                (&mut g_state.config).player_time[side_to_move as usize] as i32,
                                                (&mut g_state.config).player_increment[side_to_move as usize] as i32,
                                                timed_search, (&mut g_state.config).use_book,
                                                (&mut g_state.config).skill[side_to_move as usize] - 1,
                                                (&mut g_state.config).exact_skill[side_to_move as usize],
                                                (&mut g_state.config).wld_skill[side_to_move as usize],
                                                1, &mut best_info1, g_state)
            }
            reset_counter(&mut (&mut g_state.search).nodes);
            curr_move = legacy_compute_move(side_to_move, 0,
                                            (&mut g_state.config).player_time[side_to_move as usize] as i32,
                                            (&mut g_state.config).player_increment[side_to_move as usize] as i32,
                                            timed_search, (&mut g_state.config).use_book,
                                            (&mut g_state.config).skill[side_to_move as usize],
                                            (&mut g_state.config).exact_skill[side_to_move as usize],
                                            (&mut g_state.config).wld_skill[side_to_move as usize],
                                            1, &mut best_info2, g_state);
            if side_to_move == 0 {
                display_state.set_evals(produce_compact_eval(best_info2), 0.0f64);
            } else {
                display_state.set_evals(0.0f64, produce_compact_eval(best_info2));
            }
            /* Output the two score-move pairs */
            write!(output_stream, "{} ", TO_SQUARE(curr_move));
            if empties <= (&mut g_state.config).exact_skill[side_to_move as usize] {
                write!(output_stream, "{:+6}", best_info2.score / 128);
            } else if empties <= (&mut g_state.config).wld_skill[side_to_move as usize] {
                if best_info2.res == WON_POSITION {
                    write!(output_stream, "    +1");
                } else if best_info2.res == LOST_POSITION {
                    write!(output_stream, "    -1");
                } else {
                    write!(output_stream, "     0");
                }
            } else if curr_move == provided_move[(g_state.moves).disks_played as usize] && resp_move != -1 {
                write!(output_stream, "{:6.2}", -(played_info1.score + played_info2.score) as f64 / (2. * 128.0f64));
            } else {
                write!(output_stream, "{:6.2}", (best_info1.score + best_info2.score) as f64 / (2. * 128.0f64));
            }
            curr_move = provided_move[(g_state.moves).disks_played as usize];
            write!(output_stream, "       {} ", TO_SQUARE(curr_move));
            if resp_move == -1 {
                write!(output_stream, "     ?");
            } else if empties <= (&mut g_state.config).exact_skill[side_to_move as usize] {
                write!(output_stream, "{:+6}", -played_info2.score / 128);
            } else if empties <= (&mut g_state.config).wld_skill[side_to_move as usize] {
                if played_info2.res == WON_POSITION {
                    write!(output_stream, "    -1");
                } else if played_info2.res == LOST_POSITION {
                    write!(output_stream, "    +1");
                } else {
                    write!(output_stream, "     0");
                }
            } else {
                write!(output_stream, "{:6.2}", -(played_info1.score + played_info2.score) as f64 / (2 as f64 * 128.0f64));
            }
            write!(output_stream, "\n");
            if valid_move(curr_move, side_to_move, &(g_state.board).board) == 0 {
                fatal_error!("Invalid move {} in move sequence", TO_SQUARE(curr_move));
            }
            move_stop =  (&mut g_state.timer).get_real_timer();
            if (&mut g_state.config).player_time[side_to_move as usize] != 10000000.0f64 {
                (&mut g_state.config).player_time[side_to_move as usize] -= move_stop - move_start
            }
            (&mut g_state.learn).store_move((g_state.moves).disks_played, curr_move);
            make_move(side_to_move, curr_move, 1, (&mut g_state.moves), &mut (&mut g_state.board), &mut (&mut g_state.hash), &mut (&mut g_state.flip_stack));
            if side_to_move == 0 {
                (g_state.board).black_moves[(g_state.board).score_sheet_row as usize] = curr_move
            } else {
                if (g_state.board).white_moves[(g_state.board).score_sheet_row as usize] != -1 {
                    (g_state.board).score_sheet_row += 1
                }
                (g_state.board).white_moves[(g_state.board).score_sheet_row as usize] = curr_move
            }
        } else if side_to_move == 0 {
            (g_state.board).black_moves[(g_state.board).score_sheet_row as usize] = -1
        } else { (g_state.board).white_moves[(g_state.board).score_sheet_row as usize] = -1 }
        side_to_move = 0 + 2 - side_to_move
    }
    if (g_state.config).echo == 0 {
        write!(stdout, "\n");
        write!(stdout, "Black level: {}\n", (&mut g_state.config).skill[0]);
        write!(stdout, "White level: {}\n", (&mut g_state.config).skill[2]);
    }
    if side_to_move == 0 { (g_state.board).score_sheet_row += 1 }
    dump_game_score(side_to_move, (g_state.board).score_sheet_row, &(g_state.board).black_moves, &(g_state.board).white_moves);
    if (g_state.config).echo != 0 && (&mut g_state.config).one_position_only == 0 {
        display_state.set_move_list((g_state.board).score_sheet_row);
        display_state.set_times(floor((&mut g_state.config).player_time[0]) as i32,
                                floor((&mut g_state.config).player_time[2]) as i32);
        display_state.display_board(&mut stdout, &(g_state.board).board, side_to_move,
                                    1, (&mut g_state.config).use_timer, 1,
                                    &(g_state.board).black_moves, &(g_state.board).white_moves);
    }
}

/*
   DUMP_POSITION
   Saves the current board position to disk.
*/
fn dump_position(side_to_move: i32, board_: &[i32; 128]) {
    // let mut stream = 0 as *mut FILE;
    let mut stream = File::create("current.gam").unwrap_or_else(|_| {
        fatal_error!("File creation error when writing CURRENT.GAM\n");
    });

    let mut i: i32 = 1;
    let mut j: i32 = 0;
    while i <= 8 {
        j = 1;
        while j <= 8 {
            match board_[(10 * i + j) as usize] {
                0 => stream.write(b"X"),
                2 => stream.write(b"O"),
                1 => stream.write(b"-"),
                _ => {
                    /* This really can't happen but shouldn't cause a crash */
                    stream.write(b"?")
                }
            };
            j += 1
        }
        i += 1
    }
    stream.write(&['\n' as u8]);
    if side_to_move == 0 {
        stream.write(b"Black");
    } else {
        stream.write(b"White");
    }
    stream.write(b" to move\nThis file was automatically generated\n");
}
/*
  DUMP_GAME_SCORE
  Writes the current game score to disk.
*/
fn dump_game_score(side_to_move: i32, score_sheet_row_: i32,
                   black_moves_: &[i8; 60], white_moves_: &[i8; 60]) {
    let mut stream = File::create("current.mov").unwrap_or_else(|_| {
        fatal_error!("File creation error when writing CURRENT.MOV\n");
    });

    let mut i: i32 = 0;
    while i <= score_sheet_row_ {
        write!(stream, "   {: >2}.    ", i + 1);
        if black_moves_[i as usize] == -1 {
            write!(stream, "- ");
        } else {
            write!(stream, "{}", TO_SQUARE(black_moves_[i as usize]));
        }
        write!(stream, "  ");
        if i < score_sheet_row_ || i == score_sheet_row_ && side_to_move == 0 {
            if white_moves_[i as usize] == -1 {
                write!(stream, "- ");
            } else {
                write!(stream, "{}", TO_SQUARE(white_moves_[i as usize]));
            }
        }
        write!(stream, "\n");
        i += 1
    }
}


pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32)
    }
}
pub use engine::src::zebra::FullState;
use engine::src::thordb::ThorDatabase;
use engine_traits::Offset;
use std::ops::Mul;
