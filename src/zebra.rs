#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]

#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]

use std::thread::sleep;
use std::time::Duration;
use std::process::exit;
use std::rc::Rc;
use crate::{
    src::{
        game::{compute_move, toggle_human_openings, game_init, toggle_status_log, global_terminate, global_setup},
        search::{full_pv, full_pv_depth, nodes, disc_count, produce_compact_eval, total_time, total_evaluations, total_nodes},
        display::{echo, display_move, display_board, set_move_list, set_evals, set_names, set_times, dumpch, display_pv, toggle_smart_buffer_management},
        timer::{get_real_timer, determine_move_time, start_move, clear_panic_abort, toggle_abort_check},
        counter::{counter_value, add_counter, reset_counter, adjust_counter, CounterType},
        error::fatal_error,
        globals::{white_moves, score_sheet_row, black_moves, board},
        stubs::*,
        moves::{disks_played, make_move, valid_move, unmake_move, move_count, generate_all, game_in_progress, move_list, get_move},
        hash::{setup_hash, set_hash_transformation},
        osfbook::{set_deviation_value, reset_book_search, set_slack, find_opening_name, print_move_alternatives, fill_move_alternatives, set_draw_mode, set_game_mode},
        learn::{store_move, clear_stored_game, learn_game, set_learning_parameters, init_learn},
        eval::toggle_experimental,
        thordb::{choose_thor_opening_move, print_thor_matches, get_black_average_score, get_black_median_score, get_white_win_count, get_draw_count, get_black_win_count, get_match_count, database_search, get_thor_game_size, get_total_game_count, read_game_database, read_tournament_database, read_player_database, init_thor_database},
        getcoeff::remove_coeffs,
        myrandom::{my_random, my_srandom}
    }
};

pub type _IO_wide_data = libc::c_void;
pub type _IO_codecvt = libc::c_void;
pub type _IO_marker = libc::c_void;

pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type time_t = __time_t;
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
pub struct EvaluationType {
    pub type_0: EvalType,
    pub res: EvalResult,
    pub score: libc::c_int,
    pub confidence: libc::c_double,
    pub search_depth: libc::c_int,
    pub is_book: libc::c_int,
}
pub type DrawMode = libc::c_uint;
pub const OPPONENT_WINS: DrawMode = 3;
pub const WHITE_WINS: DrawMode = 2;
pub const BLACK_WINS: DrawMode = 1;
pub const NEUTRAL: DrawMode = 0;
pub type GameMode = libc::c_uint;
pub const PUBLIC_GAME: GameMode = 1;
pub const PRIVATE_GAME: GameMode = 0;
/* Get rid of some ugly warnings by disallowing usage of the
   macro version of tolower (not time-critical anyway). */
/* Local variables */
static mut slack: libc::c_double = 0.25f64;
static mut dev_bonus: libc::c_double = 0.0f64;
static mut low_thresh: libc::c_int = 0 as libc::c_int;
static mut high_thresh: libc::c_int = 0 as libc::c_int;
static mut rand_move_freq: libc::c_int = 0 as libc::c_int;
static mut tournament: libc::c_int = 0 as libc::c_int;
static mut tournament_levels: libc::c_int = 0;
static mut deviation_depth: libc::c_int = 0;
static mut cutoff_empty: libc::c_int = 0;
static mut one_position_only: libc::c_int = 0 as libc::c_int;
static mut use_timer: libc::c_int = 0 as libc::c_int;
static mut only_analyze: libc::c_int = 0 as libc::c_int;
static mut thor_max_games: libc::c_int = 0;
static mut tournament_skill: [[libc::c_int; 3]; 8] = [[0; 3]; 8];
static mut wld_skill: [libc::c_int; 3] = [0; 3];
static mut exact_skill: [libc::c_int; 3] = [0; 3];
static mut log_file_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut player_time: [libc::c_double; 3] = [0.; 3];
static mut player_increment: [libc::c_double; 3] = [0.; 3];
static mut skill: [libc::c_int; 3] = [0; 3];
static mut wait: libc::c_int = 0;
static mut use_book: libc::c_int = 1 as libc::c_int;
static mut wld_only: libc::c_int = 0 as libc::c_int;
static mut use_learning: libc::c_int = 0;
static mut use_thor: libc::c_int = 0;
/* ------------------- Function prototypes ---------------------- */
/* Administrative routines */
/* ---------------------- Functions ------------------------ */
/*
   MAIN
Interprets the command-line parameters and starts the game.
*/
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut game_file_name = 0 as *const libc::c_char;
    let mut script_in_file = 0 as *const libc::c_char;
    let mut script_out_file = 0 as *const libc::c_char;
    let mut move_sequence = 0 as *const libc::c_char;
    let mut move_file_name = 0 as *const libc::c_char;
    let mut arg_index: libc::c_int = 0;
    let mut help: libc::c_int = 0;
    let mut hash_bits: libc::c_int = 0;
    let mut use_random: libc::c_int = 0;
    let mut repeat = 1 as libc::c_int;
    let mut run_script: libc::c_int = 0;
    let mut script_optimal_line = 0 as libc::c_int;
    let mut komi: libc::c_int = 0;
    let mut timer: time_t = 0;
    printf(b"\nZebra (c) 1997-2005 Gunnar Andersson, compile date %s at %s\n\n\x00"
               as *const u8 as *const libc::c_char,
           b"Jul  2 2020\x00" as *const u8 as *const libc::c_char,
           b"19:33:54\x00" as *const u8 as *const libc::c_char);
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
    let mut current_block_107: u64;
    arg_index = 1 as libc::c_int;
    help = 0 as libc::c_int;
    while arg_index < argc && help == 0 {
        if strcasecmp(*argv.offset(arg_index as isize),
                      b"-e\x00" as *const u8 as *const libc::c_char) == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                echo = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-h\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                hash_bits = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-l\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            tournament = 0 as libc::c_int;
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                skill[0 as libc::c_int as usize] =
                    atoi(*argv.offset(arg_index as isize));
                if skill[0 as libc::c_int as usize] > 0 as libc::c_int {
                    if arg_index + 2 as libc::c_int >= argc {
                        help = 1 as libc::c_int;
                        current_block_107 = 2668756484064249700;
                    } else {
                        arg_index += 1;
                        exact_skill[0 as libc::c_int as usize] =
                            atoi(*argv.offset(arg_index as isize));
                        arg_index += 1;
                        wld_skill[0 as libc::c_int as usize] =
                            atoi(*argv.offset(arg_index as isize));
                        current_block_107 = 15004371738079956865;
                    }
                } else { current_block_107 = 15004371738079956865; }
                match current_block_107 {
                    2668756484064249700 => { }
                    _ => {
                        arg_index += 1;
                        if arg_index == argc {
                            help = 1 as libc::c_int;
                            current_block_107 = 2668756484064249700;
                        } else {
                            skill[2 as libc::c_int as usize] =
                                atoi(*argv.offset(arg_index as isize));
                            if skill[2 as libc::c_int as usize] >
                                   0 as libc::c_int {
                                if arg_index + 2 as libc::c_int >= argc {
                                    help = 1 as libc::c_int;
                                    current_block_107 = 2668756484064249700;
                                } else {
                                    arg_index += 1;
                                    exact_skill[2 as libc::c_int as usize] =
                                        atoi(*argv.offset(arg_index as
                                                              isize));
                                    arg_index += 1;
                                    wld_skill[2 as libc::c_int as usize] =
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
                             b"-t\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            let mut i: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                tournament = 1 as libc::c_int;
                tournament_levels = atoi(*argv.offset(arg_index as isize));
                if arg_index + 3 as libc::c_int * tournament_levels >= argc {
                    help = 1 as libc::c_int;
                    current_block_107 = 2668756484064249700;
                } else {
                    i = 0 as libc::c_int;
                    while i < tournament_levels {
                        j = 0 as libc::c_int;
                        while j < 3 as libc::c_int {
                            arg_index += 1;
                            tournament_skill[i as usize][j as usize] =
                                atoi(*argv.offset(arg_index as isize));
                            j += 1
                        }
                        i += 1
                    }
                    current_block_107 = 10485226111480991281;
                }
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-w\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                wait = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-p\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                display_pv = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"?\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            help = 1 as libc::c_int;
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-g\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                game_file_name = *argv.offset(arg_index as isize);
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-r\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                use_random = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-b\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                use_book = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-time\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if arg_index + 4 as libc::c_int >= argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                arg_index += 1;
                player_time[0 as libc::c_int as usize] =
                    atoi(*argv.offset(arg_index as isize)) as libc::c_double;
                arg_index += 1;
                player_increment[0 as libc::c_int as usize] =
                    atoi(*argv.offset(arg_index as isize)) as libc::c_double;
                arg_index += 1;
                player_time[2 as libc::c_int as usize] =
                    atoi(*argv.offset(arg_index as isize)) as libc::c_double;
                arg_index += 1;
                player_increment[2 as libc::c_int as usize] =
                    atoi(*argv.offset(arg_index as isize)) as libc::c_double;
                use_timer = 1 as libc::c_int;
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-learn\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if arg_index + 2 as libc::c_int >= argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                arg_index += 1;
                deviation_depth = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                cutoff_empty = atoi(*argv.offset(arg_index as isize));
                use_learning = 1 as libc::c_int;
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-slack\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                slack = atof(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-dev\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if arg_index + 3 as libc::c_int >= argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                arg_index += 1;
                low_thresh = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                high_thresh = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                dev_bonus = atof(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-log\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                log_file_name = *argv.offset(arg_index as isize);
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-private\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            set_game_mode(PRIVATE_GAME);
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-public\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            set_game_mode(PUBLIC_GAME);
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-keepdraw\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            set_draw_mode(NEUTRAL);
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-draw2black\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            set_draw_mode(BLACK_WINS);
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-draw2white\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            set_draw_mode(WHITE_WINS);
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-draw2none\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            set_draw_mode(OPPONENT_WINS);
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-test\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            one_position_only = 1 as libc::c_int;
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-seq\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                move_sequence = *argv.offset(arg_index as isize);
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-seqfile\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                move_file_name = *argv.offset(arg_index as isize);
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-repeat\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                repeat = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-thor\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                use_thor = 1 as libc::c_int;
                thor_max_games = atoi(*argv.offset(arg_index as isize));
                current_block_107 = 10485226111480991281;
            }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-analyze\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            only_analyze = 1 as libc::c_int;
            current_block_107 = 10485226111480991281;
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-randmove\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            arg_index += 1;
            if arg_index == argc {
                help = 1 as libc::c_int;
                current_block_107 = 2668756484064249700;
            } else {
                rand_move_freq = atoi(*argv.offset(arg_index as isize));
                if rand_move_freq < 0 as libc::c_int {
                    help = 1 as libc::c_int;
                    current_block_107 = 2668756484064249700;
                } else { current_block_107 = 10485226111480991281; }
            }
        } else {
            help = 1 as libc::c_int;
            current_block_107 = 10485226111480991281;
        }
        match current_block_107 {
            10485226111480991281 => {
                if arg_index >= argc { help = 1 as libc::c_int }
            }
            _ => { }
        }
        arg_index += 1
    }
    if help != 0 {
        puts(b"Usage:\x00" as *const u8 as *const libc::c_char);
        puts(b"  zebra [-b -e -g -h -l -p -t -time -w -learn -slack -dev -log\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"         -keepdraw -draw2black -draw2white -draw2none\x00" as
                 *const u8 as *const libc::c_char);
        puts(b"         -private -public -test -seq -thor -script -analyze ?\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"         -repeat -seqfile]\x00" as *const u8 as
                 *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"Flags:\x00" as *const u8 as *const libc::c_char);
        puts(b"  ? \x00" as *const u8 as *const libc::c_char);
        puts(b"    Displays this text.\x00" as *const u8 as
                 *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -b <use book?>\x00" as *const u8 as *const libc::c_char);
        printf(b"    Toggles usage of opening book on/off (default %d).\n\x00"
                   as *const u8 as *const libc::c_char, 1 as libc::c_int);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -e <echo?>\x00" as *const u8 as *const libc::c_char);
        printf(b"    Toggles screen output on/off (default %d).\n\x00" as
                   *const u8 as *const libc::c_char, 1 as libc::c_int);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -g <game file>\x00" as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -h <bits in hash key>\x00" as *const u8 as
                 *const libc::c_char);
        printf(b"    Size of hash table is 2^{this value} (default %d).\n\x00"
                   as *const u8 as *const libc::c_char, 18 as libc::c_int);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -l <black depth> [<black exact depth> <black WLD depth>]\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"     <white depth> [<white exact depth> <white WLD depth>]\x00"
                 as *const u8 as *const libc::c_char);
        printf(b"    Sets the search depth. If <black depth> or <white depth> \x00"
                   as *const u8 as *const libc::c_char);
        printf(b"are set to 0, a\n\x00" as *const u8 as *const libc::c_char);
        printf(b"    human player is assumed. In this case the other \x00" as
                   *const u8 as *const libc::c_char);
        printf(b"parameters must be omitted.\n\x00" as *const u8 as
                   *const libc::c_char);
        printf(b"    <* exact depth> specify the number of moves before the \x00"
                   as *const u8 as *const libc::c_char);
        printf(b"(at move 60) when\n\x00" as *const u8 as
                   *const libc::c_char);
        printf(b"    the exact game-theoretical value is calculated. <* WLD \x00"
                   as *const u8 as *const libc::c_char);
        printf(b"depth> are used\n\x00" as *const u8 as *const libc::c_char);
        puts(b"    analogously for the calculation of Win/Loss/Draw.\x00" as
                 *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -p <display principal variation?>\x00" as *const u8 as
                 *const libc::c_char);
        printf(b"    Toggles output of principal variation on/off (default %d).\n\x00"
                   as *const u8 as *const libc::c_char, 1 as libc::c_int);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -r <use randomization?>\x00" as *const u8 as
                 *const libc::c_char);
        printf(b"    Toggles randomization on/off (default %d)\n\x00" as
                   *const u8 as *const libc::c_char, 1 as libc::c_int);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -t <number of levels> <(first) depth> ... <(last) wld depth>\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -time <black time> <black increment> <white time> <white increment>\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"    Tournament mode; the format for the players is as above.\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -w <wait?>\x00" as *const u8 as *const libc::c_char);
        printf(b"    Toggles wait between moves on/off (default %d).\n\x00" as
                   *const u8 as *const libc::c_char, 0 as libc::c_int);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -learn <depth> <cutoff>\x00" as *const u8 as
                 *const libc::c_char);
        puts(b"    Learn the game with <depth> deviations up to <cutoff> empty.\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -slack <disks>\x00" as *const u8 as *const libc::c_char);
        printf(b"    Zebra\'s opening randomness is <disks> disks (default %f).\n\x00"
                   as *const u8 as *const libc::c_char, 0.25f64);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -dev <low> <high> <bonus>\x00" as *const u8 as
                 *const libc::c_char);
        puts(b"    Give deviations before move <high> a <bonus> disk bonus but\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"    don\'t give any extra bonus for deviations before move <low>.\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -log <file name>\x00" as *const u8 as *const libc::c_char);
        puts(b"    Append all game results to the specified file.\x00" as
                 *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -private\x00" as *const u8 as *const libc::c_char);
        puts(b"    Treats all draws as losses for both sides.\x00" as
                 *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -public\x00" as *const u8 as *const libc::c_char);
        puts(b"    No tweaking of draw scores.\x00" as *const u8 as
                 *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -keepdraw\x00" as *const u8 as *const libc::c_char);
        puts(b"    Book draws are counted as draws.\x00" as *const u8 as
                 *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -draw2black\x00" as *const u8 as *const libc::c_char);
        puts(b"    Book draws scored as 32-31 for black.\x00" as *const u8 as
                 *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -draw2white\x00" as *const u8 as *const libc::c_char);
        puts(b"    Book draws scored as 32-31 for white.\x00" as *const u8 as
                 *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -draw2none\x00" as *const u8 as *const libc::c_char);
        puts(b"    Book draws scored as 32-31 for the opponent.\x00" as
                 *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -test\x00" as *const u8 as *const libc::c_char);
        puts(b"    Only evaluate one position, then exit.\x00" as *const u8 as
                 *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -seq <move sequence>\x00" as *const u8 as
                 *const libc::c_char);
        puts(b"    Forces the game to start with a predefined move sequence;\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"    e.g. f4d6c3.\x00" as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -seqfile <filename\x00" as *const u8 as *const libc::c_char);
        puts(b"    Specifies a file from which move sequences are read.\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -thor <game count>\x00" as *const u8 as *const libc::c_char);
        puts(b"    Look for each position in the Thor database; list the first <game count>.\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -script <script file> <output file>\x00" as *const u8 as
                 *const libc::c_char);
        puts(b"    Solves all positions in script file for exact score.\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        puts(b"  -wld <only solve WLD?>\x00" as *const u8 as
                 *const libc::c_char);
        printf(b"    Toggles WLD only solve on/off (default %d).\n\n\x00" as
                   *const u8 as *const libc::c_char, 0 as libc::c_int);
        puts(b"  -analyze\x00" as *const u8 as *const libc::c_char);
        puts(b"    Used in conjunction with -seq; all positions are analyzed.\x00"
                 as *const u8 as *const libc::c_char);
        puts(b"  -repeat <#iterations>\x00" as *const u8 as
                 *const libc::c_char);
        puts(b"    Repeats the operation the specified number of times. \x00"
                 as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if hash_bits < 1 as libc::c_int {
        printf(b"Hash table key must contain at least 1 bit\n\x00" as
                   *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    global_setup(use_random, hash_bits);
    init_thor_database();
    if use_book != 0 {
        init_learn(b"book.bin\x00" as *const u8 as *const libc::c_char,
                   1 as libc::c_int);
    }
    if use_random != 0 {
        time(&mut timer);
        my_srandom(timer as libc::c_int);
    } else { my_srandom(1 as libc::c_int); }
    if tournament == 0 && run_script == 0 {
        while skill[0 as libc::c_int as usize] < 0 as libc::c_int {
            printf(b"Black parameters: \x00" as *const u8 as
                       *const libc::c_char);
            scanf(b"%d\x00" as *const u8 as *const libc::c_char,
                  &mut *skill.as_mut_ptr().offset(0 as libc::c_int as isize)
                      as *mut libc::c_int);
            if skill[0 as libc::c_int as usize] > 0 as libc::c_int {
                scanf(b"%d %d\x00" as *const u8 as *const libc::c_char,
                      &mut *exact_skill.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
                          *mut libc::c_int,
                      &mut *wld_skill.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize) as
                          *mut libc::c_int);
            }
        }
        while skill[2 as libc::c_int as usize] < 0 as libc::c_int {
            printf(b"White parameters: \x00" as *const u8 as
                       *const libc::c_char);
            scanf(b"%d\x00" as *const u8 as *const libc::c_char,
                  &mut *skill.as_mut_ptr().offset(2 as libc::c_int as isize)
                      as *mut libc::c_int);
            if skill[2 as libc::c_int as usize] > 0 as libc::c_int {
                scanf(b"%d %d\x00" as *const u8 as *const libc::c_char,
                      &mut *exact_skill.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize) as
                          *mut libc::c_int,
                      &mut *wld_skill.as_mut_ptr().offset(2 as libc::c_int as
                                                              isize) as
                          *mut libc::c_int);
            }
        }
    }
    if one_position_only != 0 {
        toggle_smart_buffer_management(0 as libc::c_int);
    }
    if run_script != 0 {
        run_endgame_script(script_in_file, script_out_file,
                           script_optimal_line);
    } else if tournament != 0 {
        play_tournament(move_sequence);
    } else if only_analyze != 0 {
        analyze_game(move_sequence);
    } else {
        play_game(game_file_name, move_sequence, move_file_name, repeat);
    }
    global_terminate();
    return 0 as libc::c_int;
}
/*
   PLAY_TOURNAMENT
   Administrates the tournament between different levels
   of the program.
*/
unsafe extern "C" fn play_tournament(mut move_sequence: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut result: [[[libc::c_int; 3]; 8]; 8] = [[[0; 3]; 8]; 8];
    let mut tourney_time: libc::c_double = 0.;
    let mut score: [libc::c_double; 8] = [0.; 8];
    let mut color_score: [libc::c_double; 3] = [0.; 3];
    let mut tourney_nodes = CounterType{hi: 0, lo: 0,};
    reset_counter(&mut tourney_nodes);
    tourney_time = 0.0f64;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int { score[i as usize] = 0.0f64; i += 1 }
    color_score[2 as libc::c_int as usize] = 0.0f64;
    color_score[0 as libc::c_int as usize] =
        color_score[2 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < tournament_levels {
        j = 0 as libc::c_int;
        while j < tournament_levels {
            skill[0 as libc::c_int as usize] =
                tournament_skill[i as usize][0 as libc::c_int as usize];
            exact_skill[0 as libc::c_int as usize] =
                tournament_skill[i as usize][1 as libc::c_int as usize];
            wld_skill[0 as libc::c_int as usize] =
                tournament_skill[i as usize][2 as libc::c_int as usize];
            skill[2 as libc::c_int as usize] =
                tournament_skill[j as usize][0 as libc::c_int as usize];
            exact_skill[2 as libc::c_int as usize] =
                tournament_skill[j as usize][1 as libc::c_int as usize];
            wld_skill[2 as libc::c_int as usize] =
                tournament_skill[j as usize][2 as libc::c_int as usize];
            play_game(0 as *const libc::c_char, move_sequence,
                      0 as *const libc::c_char, 1 as libc::c_int);
            add_counter(&mut tourney_nodes, &mut total_nodes);
            tourney_time += total_time;
            result[i as usize][j as usize][0 as libc::c_int as usize] =
                disc_count(0 as libc::c_int);
            result[i as usize][j as usize][2 as libc::c_int as usize] =
                disc_count(2 as libc::c_int);
            if disc_count(0 as libc::c_int) > disc_count(2 as libc::c_int) {
                score[i as usize] += 1.0f64;
                color_score[0 as libc::c_int as usize] += 1.0f64
            } else if disc_count(0 as libc::c_int) ==
                          disc_count(2 as libc::c_int) {
                score[i as usize] += 0.5f64;
                score[j as usize] += 0.5f64;
                color_score[0 as libc::c_int as usize] += 0.5f64;
                color_score[2 as libc::c_int as usize] += 0.5f64
            } else {
                score[j as usize] += 1.0f64;
                color_score[2 as libc::c_int as usize] += 1.0f64
            }
            j += 1
        }
        i += 1
    }
    adjust_counter(&mut tourney_nodes);
    printf(b"\n\nTime:  %.1f s\nNodes: %.0f\n\x00" as *const u8 as
               *const libc::c_char, tourney_time,
           counter_value(&mut tourney_nodes));
    puts(b"\nCompetitors:\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < tournament_levels {
        printf(b"  Player %2d: %d-%d-%d\n\x00" as *const u8 as
                   *const libc::c_char, i + 1 as libc::c_int,
               tournament_skill[i as usize][0 as libc::c_int as usize],
               tournament_skill[i as usize][1 as libc::c_int as usize],
               tournament_skill[i as usize][2 as libc::c_int as usize]);
        i += 1
    }
    printf(b"\n       \x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < tournament_levels {
        printf(b" %2d    \x00" as *const u8 as *const libc::c_char,
               i + 1 as libc::c_int);
        i += 1
    }
    puts(b"  Score\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < tournament_levels {
        printf(b"  %2d   \x00" as *const u8 as *const libc::c_char,
               i + 1 as libc::c_int);
        j = 0 as libc::c_int;
        while j < tournament_levels {
            printf(b"%2d-%2d  \x00" as *const u8 as *const libc::c_char,
                   result[i as usize][j as usize][0 as libc::c_int as usize],
                   result[i as usize][j as usize][2 as libc::c_int as usize]);
            j += 1
        }
        printf(b"  %4.1f\n\x00" as *const u8 as *const libc::c_char,
               score[i as usize]);
        i += 1
    }
    puts(b"\x00" as *const u8 as *const libc::c_char);
    printf(b"Black score: %.1f\n\x00" as *const u8 as *const libc::c_char,
           color_score[0 as libc::c_int as usize]);
    printf(b"White score: %.1f\n\x00" as *const u8 as *const libc::c_char,
           color_score[2 as libc::c_int as usize]);
    puts(b"\x00" as *const u8 as *const libc::c_char);
}
/*
   PLAY_GAME
   Administrates the game between two players, humans or computers.
*/
unsafe extern "C" fn play_game(mut file_name: *const libc::c_char,
                               mut move_string: *const libc::c_char,
                               mut move_file_name: *const libc::c_char,
                               mut repeat: libc::c_int) {
    let mut eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut black_name = 0 as *const libc::c_char;
    let mut white_name = 0 as *const libc::c_char;
    let mut opening_name = 0 as *const libc::c_char;
    let mut node_val: libc::c_double = 0.;
    let mut eval_val: libc::c_double = 0.;
    let mut move_start: libc::c_double = 0.;
    let mut move_stop: libc::c_double = 0.;
    let mut database_start: libc::c_double = 0.;
    let mut database_stop: libc::c_double = 0.;
    let mut total_search_time = 0.0f64;
    let mut i: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut curr_move: libc::c_int = 0;
    let mut timed_search: libc::c_int = 0;
    let mut rand_color = 0 as libc::c_int;
    let mut black_hash1: libc::c_int = 0;
    let mut black_hash2: libc::c_int = 0;
    let mut white_hash1: libc::c_int = 0;
    let mut white_hash2: libc::c_int = 0;
    let mut provided_move_count: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut thor_position_count: libc::c_int = 0;
    let mut provided_move: [libc::c_int; 61] = [0; 61];
    let mut move_vec: [libc::c_char; 121] = [0; 121];
    let mut line_buffer: [libc::c_char; 1000] = [0; 1000];
    let mut timer: time_t = 0;
    let mut log_file = 0 as *mut FILE;
    let mut move_file = 0 as *mut FILE;
    if !move_file_name.is_null() {
        move_file =
            fopen(move_file_name,
                  b"r\x00" as *const u8 as *const libc::c_char)
    } else { move_file = 0 as *mut FILE }
    loop  {
        /* Decode the predefined move sequence */
        if !move_file.is_null() {
            let mut newline_pos = 0 as *mut libc::c_char;
            fgets(line_buffer.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 1000]>() as
                      libc::c_ulong as libc::c_int, move_file);
            newline_pos = strchr(line_buffer.as_mut_ptr(), '\n' as i32);
            if !newline_pos.is_null() {
                *newline_pos = 0 as libc::c_int as libc::c_char
            }
            move_string = line_buffer.as_mut_ptr()
        }
        if move_string.is_null() {
            provided_move_count = 0 as libc::c_int
        } else {
            provided_move_count =
                strlen(move_string).wrapping_div(2 as libc::c_int as
                                                     libc::c_ulong) as
                    libc::c_int;
            if provided_move_count > 60 as libc::c_int ||
                   strlen(move_string).wrapping_rem(2 as libc::c_int as
                                                        libc::c_ulong) ==
                       1 as libc::c_int as libc::c_ulong {
                fatal_error(b"Invalid move string provided\x00" as *const u8
                                as *const libc::c_char);
            }
            i = 0 as libc::c_int;
            while i < provided_move_count {
                col =
                    tolower(*move_string.offset((2 as libc::c_int * i) as
                                                    isize) as libc::c_int) -
                        'a' as i32 + 1 as libc::c_int;
                row =
                    *move_string.offset((2 as libc::c_int * i +
                                             1 as libc::c_int) as isize) as
                        libc::c_int - '0' as i32;
                if col < 1 as libc::c_int || col > 8 as libc::c_int ||
                       row < 1 as libc::c_int || row > 8 as libc::c_int {
                    fatal_error(b"Unexpected character in move string\x00" as
                                    *const u8 as *const libc::c_char);
                }
                provided_move[i as usize] = 10 as libc::c_int * row + col;
                i += 1
            }
        }
        /* Set up the position and the search engine */
        game_init(file_name, &mut side_to_move);
        setup_hash(1 as libc::c_int);
        clear_stored_game();
        if echo != 0 && use_book != 0 {
            printf(b"Book randomness: %.2f disks\n\x00" as *const u8 as
                       *const libc::c_char, slack);
        }
        set_slack(floor(slack * 128.0f64) as libc::c_int);
        toggle_human_openings(0 as libc::c_int);
        if use_learning != 0 {
            set_learning_parameters(deviation_depth, cutoff_empty);
        }
        reset_book_search();
        set_deviation_value(low_thresh, high_thresh, dev_bonus);
        if use_thor != 0 {
            /* No error checking done as it's only for testing purposes */
            database_start = get_real_timer();
            read_player_database(b"thor\\wthor.jou\x00" as *const u8 as
                                     *const libc::c_char);
            read_tournament_database(b"thor\\wthor.trn\x00" as *const u8 as
                                         *const libc::c_char);
            read_game_database(b"thor\\wth_2001.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_2000.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1999.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1998.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1997.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1996.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1995.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1994.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1993.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1992.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1991.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1990.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1989.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1988.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1987.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1986.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1985.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1984.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1983.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1982.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1981.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            read_game_database(b"thor\\wth_1980.wtb\x00" as *const u8 as
                                   *const libc::c_char);
            database_stop = get_real_timer();
            printf(b"Loaded %d games in %.3f s.\n\x00" as *const u8 as
                       *const libc::c_char, get_total_game_count(),
                   database_stop - database_start);
            printf(b"Each Thor game occupies %d bytes.\n\x00" as *const u8 as
                       *const libc::c_char, get_thor_game_size());
        }
        if skill[0 as libc::c_int as usize] == 0 as libc::c_int {
            black_name = b"Player\x00" as *const u8 as *const libc::c_char
        } else {
            black_name = b"Zebra\x00" as *const u8 as *const libc::c_char
        }
        if skill[2 as libc::c_int as usize] == 0 as libc::c_int {
            white_name = b"Player\x00" as *const u8 as *const libc::c_char
        } else {
            white_name = b"Zebra\x00" as *const u8 as *const libc::c_char
        }
        set_names(black_name, white_name);
        set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                      score_sheet_row);
        set_evals(0.0f64, 0.0f64);
        i = 0 as libc::c_int;
        while i < 60 as libc::c_int {
            black_moves[i as usize] = -(1 as libc::c_int);
            white_moves[i as usize] = -(1 as libc::c_int);
            i += 1
        }
        move_vec[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char;
        black_hash1 = my_random() as libc::c_int;
        black_hash2 = my_random() as libc::c_int;
        white_hash1 = my_random() as libc::c_int;
        white_hash2 = my_random() as libc::c_int;
        while game_in_progress() != 0 {
            remove_coeffs(disks_played);
            generate_all(side_to_move);
            if side_to_move == 0 as libc::c_int { score_sheet_row += 1 }
            if move_count[disks_played as usize] != 0 as libc::c_int {
                move_start = get_real_timer();
                clear_panic_abort();
                if echo != 0 {
                    set_move_list(black_moves.as_mut_ptr(),
                                  white_moves.as_mut_ptr(), score_sheet_row);
                    set_times(floor(player_time[0 as libc::c_int as usize]) as
                                  libc::c_int,
                              floor(player_time[2 as libc::c_int as usize]) as
                                  libc::c_int);
                    opening_name = find_opening_name();
                    if !opening_name.is_null() {
                        printf(b"\nOpening: %s\n\x00" as *const u8 as
                                   *const libc::c_char, opening_name);
                    }
                    if use_thor != 0 {
                        database_start = get_real_timer();
                        database_search(board.as_mut_ptr(), side_to_move);
                        thor_position_count = get_match_count();
                        database_stop = get_real_timer();
                        total_search_time += database_stop - database_start;
                        printf(b"%d matching games  (%.3f s search time, %.3f s total)\n\x00"
                                   as *const u8 as *const libc::c_char,
                               thor_position_count,
                               database_stop - database_start,
                               total_search_time);
                        if thor_position_count > 0 as libc::c_int {
                            printf(b"%d black wins, %d draws, %d white wins\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   get_black_win_count(), get_draw_count(),
                                   get_white_win_count());
                            printf(b"Median score %d-%d\x00" as *const u8 as
                                       *const libc::c_char,
                                   get_black_median_score(),
                                   64 as libc::c_int -
                                       get_black_median_score());
                            printf(b", average score %.2f-%.2f\n\x00" as
                                       *const u8 as *const libc::c_char,
                                   get_black_average_score(),
                                   64.0f64 - get_black_average_score());
                        }
                        print_thor_matches(stdout, thor_max_games);
                    }
                    display_board(stdout, board.as_mut_ptr(), side_to_move,
                                  1 as libc::c_int, use_timer,
                                  1 as libc::c_int);
                }
                dump_position(side_to_move);
                dump_game_score(side_to_move);
                /* Check what the Thor opening statistics has to say */
                choose_thor_opening_move(board.as_mut_ptr(), side_to_move,
                                         echo);
                if echo != 0 && wait != 0 { dumpch(); }
                if disks_played >= provided_move_count {
                    if skill[side_to_move as usize] == 0 as libc::c_int {
                        if use_book != 0 && display_pv != 0 {
                            fill_move_alternatives(side_to_move,
                                                   0 as libc::c_int);
                            if echo != 0 {
                                print_move_alternatives(side_to_move);
                            }
                        }
                        puts(b"\x00" as *const u8 as *const libc::c_char);
                        curr_move = get_move(side_to_move)
                    } else {
                        start_move(player_time[side_to_move as usize],
                                   player_increment[side_to_move as usize],
                                   disks_played + 4 as libc::c_int);
                        determine_move_time(player_time[side_to_move as
                                                            usize],
                                            player_increment[side_to_move as
                                                                 usize],
                                            disks_played + 4 as libc::c_int);
                        timed_search =
                            (skill[side_to_move as usize] >=
                                 60 as libc::c_int) as libc::c_int;
                        toggle_experimental(0 as libc::c_int);
                        curr_move =
                            compute_move(side_to_move, 1 as libc::c_int,
                                         player_time[side_to_move as usize] as
                                             libc::c_int,
                                         player_increment[side_to_move as
                                                              usize] as
                                             libc::c_int, timed_search,
                                         use_book,
                                         skill[side_to_move as usize],
                                         exact_skill[side_to_move as usize],
                                         wld_skill[side_to_move as usize],
                                         0 as libc::c_int, &mut eval_info);
                        if side_to_move == 0 as libc::c_int {
                            set_evals(produce_compact_eval(eval_info),
                                      0.0f64);
                        } else {
                            set_evals(0.0f64,
                                      produce_compact_eval(eval_info));
                        }
                        if eval_info.is_book != 0 &&
                               rand_move_freq > 0 as libc::c_int &&
                               side_to_move == rand_color &&
                               my_random() % rand_move_freq as libc::c_long ==
                                   0 as libc::c_int as libc::c_long {
                            puts(b"Engine override: Random move selected.\x00"
                                     as *const u8 as *const libc::c_char);
                            rand_color =
                                0 as libc::c_int + 2 as libc::c_int -
                                    rand_color;
                            curr_move =
                                move_list[disks_played as
                                              usize][(my_random() %
                                                          move_count[disks_played
                                                                         as
                                                                         usize]
                                                              as libc::c_long)
                                                         as usize]
                        }
                    }
                } else {
                    curr_move = provided_move[disks_played as usize];
                    if valid_move(curr_move, side_to_move) == 0 {
                        fatal_error(b"Invalid move %c%c in move sequence\x00"
                                        as *const u8 as *const libc::c_char,
                                    'a' as i32 + curr_move % 10 as libc::c_int
                                        - 1 as libc::c_int,
                                    '0' as i32 +
                                        curr_move / 10 as libc::c_int);
                    }
                }
                move_stop = get_real_timer();
                if player_time[side_to_move as usize] != 10000000.0f64 {
                    player_time[side_to_move as usize] -=
                        move_stop - move_start
                }
                store_move(disks_played, curr_move);
                sprintf(move_vec.as_mut_ptr().offset((2 as libc::c_int *
                                                          disks_played) as
                                                         isize),
                        b"%c%c\x00" as *const u8 as *const libc::c_char,
                        'a' as i32 + curr_move % 10 as libc::c_int -
                            1 as libc::c_int,
                        '0' as i32 + curr_move / 10 as libc::c_int);
                make_move(side_to_move, curr_move, 1 as libc::c_int);
                if side_to_move == 0 as libc::c_int {
                    black_moves[score_sheet_row as usize] = curr_move
                } else {
                    if white_moves[score_sheet_row as usize] !=
                           -(1 as libc::c_int) {
                        score_sheet_row += 1
                    }
                    white_moves[score_sheet_row as usize] = curr_move
                }
            } else {
                if side_to_move == 0 as libc::c_int {
                    black_moves[score_sheet_row as usize] =
                        -(1 as libc::c_int)
                } else {
                    white_moves[score_sheet_row as usize] =
                        -(1 as libc::c_int)
                }
                if skill[side_to_move as usize] == 0 as libc::c_int {
                    puts(b"You must pass - please press Enter\x00" as
                             *const u8 as *const libc::c_char);
                    dumpch();
                }
            }
            side_to_move = 0 as libc::c_int + 2 as libc::c_int - side_to_move;
            if one_position_only != 0 { break ; }
        }
        if echo == 0 && one_position_only == 0 {
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
            printf(b"Black level: %d\n\x00" as *const u8 as
                       *const libc::c_char, skill[0 as libc::c_int as usize]);
            printf(b"White level: %d\n\x00" as *const u8 as
                       *const libc::c_char, skill[2 as libc::c_int as usize]);
        }
        if side_to_move == 0 as libc::c_int { score_sheet_row += 1 }
        dump_game_score(side_to_move);
        if echo != 0 && one_position_only == 0 {
            set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                          score_sheet_row);
            if use_thor != 0 {
                database_start = get_real_timer();
                database_search(board.as_mut_ptr(), side_to_move);
                thor_position_count = get_match_count();
                database_stop = get_real_timer();
                total_search_time += database_stop - database_start;
                printf(b"%d matching games  (%.3f s search time, %.3f s total)\n\x00"
                           as *const u8 as *const libc::c_char,
                       thor_position_count, database_stop - database_start,
                       total_search_time);
                if thor_position_count > 0 as libc::c_int {
                    printf(b"%d black wins,  %d draws,  %d white wins\n\x00"
                               as *const u8 as *const libc::c_char,
                           get_black_win_count(), get_draw_count(),
                           get_white_win_count());
                    printf(b"Median score %d-%d\n\x00" as *const u8 as
                               *const libc::c_char, get_black_median_score(),
                           64 as libc::c_int - get_black_median_score());
                    printf(b", average score %.2f-%.2f\n\x00" as *const u8 as
                               *const libc::c_char, get_black_average_score(),
                           64.0f64 - get_black_average_score());
                }
                print_thor_matches(stdout, thor_max_games);
            }
            set_times(floor(player_time[0 as libc::c_int as usize]) as
                          libc::c_int,
                      floor(player_time[2 as libc::c_int as usize]) as
                          libc::c_int);
            display_board(stdout, board.as_mut_ptr(), side_to_move,
                          1 as libc::c_int, use_timer, 1 as libc::c_int);
        }
        adjust_counter(&mut total_nodes);
        node_val = counter_value(&mut total_nodes);
        adjust_counter(&mut total_evaluations);
        eval_val = counter_value(&mut total_evaluations);
        printf(b"\nBlack: %d   White: %d\n\x00" as *const u8 as
                   *const libc::c_char, disc_count(0 as libc::c_int),
               disc_count(2 as libc::c_int));
        printf(b"Nodes searched:        %-10.0f\n\x00" as *const u8 as
                   *const libc::c_char, node_val);
        printf(b"Positions evaluated:   %-10.0f\n\x00" as *const u8 as
                   *const libc::c_char, eval_val);
        printf(b"Total time: %.1f s\n\x00" as *const u8 as
                   *const libc::c_char, total_time);
        if !log_file_name.is_null() && one_position_only == 0 {
            log_file =
                fopen(log_file_name,
                      b"a\x00" as *const u8 as *const libc::c_char);
            if !log_file.is_null() {
                timer = time(0 as *mut time_t);
                fprintf(log_file,
                        b"# %s#     %2d - %2d\n\x00" as *const u8 as
                            *const libc::c_char, ctime(&mut timer),
                        disc_count(0 as libc::c_int),
                        disc_count(2 as libc::c_int));
                fprintf(log_file,
                        b"%s\n\x00" as *const u8 as *const libc::c_char,
                        move_vec.as_mut_ptr());
                fclose(log_file);
            }
        }
        repeat -= 1;
        toggle_abort_check(0 as libc::c_int);
        if use_learning != 0 && one_position_only == 0 {
            learn_game(disks_played,
                       (skill[0 as libc::c_int as usize] != 0 as libc::c_int
                            &&
                            skill[2 as libc::c_int as usize] !=
                                0 as libc::c_int) as libc::c_int,
                       (repeat == 0 as libc::c_int) as libc::c_int);
        }
        toggle_abort_check(1 as libc::c_int);
        if !(repeat > 0 as libc::c_int) { break ; }
    }
    if !move_file.is_null() { fclose(move_file); };
}
/*
   ANALYZE_GAME
   Analyzes all positions arising from a given move sequence.
*/
unsafe extern "C" fn analyze_game(mut move_string: *const libc::c_char) {
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
    let mut black_name = 0 as *const libc::c_char;
    let mut white_name = 0 as *const libc::c_char;
    let mut opening_name = 0 as *const libc::c_char;
    let mut move_start: libc::c_double = 0.;
    let mut move_stop: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut opponent: libc::c_int = 0;
    let mut curr_move: libc::c_int = 0;
    let mut resp_move: libc::c_int = 0;
    let mut timed_search: libc::c_int = 0;
    let mut black_hash1: libc::c_int = 0;
    let mut black_hash2: libc::c_int = 0;
    let mut white_hash1: libc::c_int = 0;
    let mut white_hash2: libc::c_int = 0;
    let mut provided_move_count: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut empties: libc::c_int = 0;
    let mut provided_move: [libc::c_int; 61] = [0; 61];
    let mut best_trans1: libc::c_uint = 0;
    let mut best_trans2: libc::c_uint = 0;
    let mut played_trans1: libc::c_uint = 0;
    let mut played_trans2: libc::c_uint = 0;
    let mut output_stream = 0 as *mut FILE;
    /* Decode the predefined move sequence */
    if move_string.is_null() {
        provided_move_count = 0 as libc::c_int
    } else {
        provided_move_count =
            strlen(move_string).wrapping_div(2 as libc::c_int as
                                                 libc::c_ulong) as
                libc::c_int;
        if provided_move_count > 60 as libc::c_int ||
               strlen(move_string).wrapping_rem(2 as libc::c_int as
                                                    libc::c_ulong) ==
                   1 as libc::c_int as libc::c_ulong {
            fatal_error(b"Invalid move string provided\x00" as *const u8 as
                            *const libc::c_char);
        }
        i = 0 as libc::c_int;
        while i < provided_move_count {
            col =
                tolower(*move_string.offset((2 as libc::c_int * i) as isize)
                            as libc::c_int) - 'a' as i32 + 1 as libc::c_int;
            row =
                *move_string.offset((2 as libc::c_int * i + 1 as libc::c_int)
                                        as isize) as libc::c_int - '0' as i32;
            if col < 1 as libc::c_int || col > 8 as libc::c_int ||
                   row < 1 as libc::c_int || row > 8 as libc::c_int {
                fatal_error(b"Unexpected character in move string\x00" as
                                *const u8 as *const libc::c_char);
            }
            provided_move[i as usize] = 10 as libc::c_int * row + col;
            i += 1
        }
    }
    /* Open the output log file */
    output_stream =
        fopen(b"analysis.log\x00" as *const u8 as *const libc::c_char,
              b"w\x00" as *const u8 as *const libc::c_char);
    if output_stream.is_null() {
        fatal_error(b"Can\'t create log file analysis.log - aborting\x00" as
                        *const u8 as *const libc::c_char);
    }
    /* Set up the position and the search engine */
    if echo != 0 {
        puts(b"Analyzing provided game...\x00" as *const u8 as
                 *const libc::c_char);
    }
    game_init(0 as *const libc::c_char, &mut side_to_move);
    setup_hash(1 as libc::c_int);
    clear_stored_game();
    if echo != 0 && use_book != 0 {
        puts(b"Disabling usage of opening book\x00" as *const u8 as
                 *const libc::c_char);
    }
    use_book = 0 as libc::c_int;
    reset_book_search();
    black_name = b"Zebra\x00" as *const u8 as *const libc::c_char;
    white_name = b"Zebra\x00" as *const u8 as *const libc::c_char;
    set_names(black_name, white_name);
    set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                  score_sheet_row);
    set_evals(0.0f64, 0.0f64);
    i = 0 as libc::c_int;
    while i < 60 as libc::c_int {
        black_moves[i as usize] = -(1 as libc::c_int);
        white_moves[i as usize] = -(1 as libc::c_int);
        i += 1
    }
    black_hash1 = my_random() as libc::c_int;
    black_hash2 = my_random() as libc::c_int;
    white_hash1 = my_random() as libc::c_int;
    white_hash2 = my_random() as libc::c_int;
    best_trans1 = my_random() as libc::c_uint;
    best_trans2 = my_random() as libc::c_uint;
    played_trans1 = my_random() as libc::c_uint;
    played_trans2 = my_random() as libc::c_uint;
    while game_in_progress() != 0 && disks_played < provided_move_count {
        remove_coeffs(disks_played);
        generate_all(side_to_move);
        if side_to_move == 0 as libc::c_int { score_sheet_row += 1 }
        if move_count[disks_played as usize] != 0 as libc::c_int {
            move_start = get_real_timer();
            clear_panic_abort();
            if echo != 0 {
                set_move_list(black_moves.as_mut_ptr(),
                              white_moves.as_mut_ptr(), score_sheet_row);
                set_times(floor(player_time[0 as libc::c_int as usize]) as
                              libc::c_int,
                          floor(player_time[2 as libc::c_int as usize]) as
                              libc::c_int);
                opening_name = find_opening_name();
                if !opening_name.is_null() {
                    printf(b"\nOpening: %s\n\x00" as *const u8 as
                               *const libc::c_char, opening_name);
                }
                display_board(stdout, board.as_mut_ptr(), side_to_move,
                              1 as libc::c_int, use_timer, 1 as libc::c_int);
            }
            /* Check what the Thor opening statistics has to say */
            choose_thor_opening_move(board.as_mut_ptr(), side_to_move, echo);
            if echo != 0 && wait != 0 { dumpch(); }
            start_move(player_time[side_to_move as usize],
                       player_increment[side_to_move as usize],
                       disks_played + 4 as libc::c_int);
            determine_move_time(player_time[side_to_move as usize],
                                player_increment[side_to_move as usize],
                                disks_played + 4 as libc::c_int);
            timed_search =
                (skill[side_to_move as usize] >= 60 as libc::c_int) as
                    libc::c_int;
            toggle_experimental(0 as libc::c_int);
            empties = 60 as libc::c_int - disks_played;
            /* Determine the score for the move actually played.
               A private hash transformation is used so that the parallel
            search trees - "played" and "best" - don't clash. This way
             all scores are comparable. */
            set_hash_transformation(played_trans1, played_trans2);
            curr_move = provided_move[disks_played as usize];
            opponent = 0 as libc::c_int + 2 as libc::c_int - side_to_move;
            make_move(side_to_move, curr_move, 1 as libc::c_int);
            if empties > wld_skill[side_to_move as usize] {
                reset_counter(&mut nodes);
                resp_move =
                    compute_move(opponent, 0 as libc::c_int,
                                 player_time[opponent as usize] as
                                     libc::c_int,
                                 player_increment[opponent as usize] as
                                     libc::c_int, timed_search, use_book,
                                 skill[opponent as usize] - 2 as libc::c_int,
                                 exact_skill[opponent as usize] -
                                     1 as libc::c_int,
                                 wld_skill[opponent as usize] -
                                     1 as libc::c_int, 1 as libc::c_int,
                                 &mut played_info1)
            }
            reset_counter(&mut nodes);
            resp_move =
                compute_move(opponent, 0 as libc::c_int,
                             player_time[opponent as usize] as libc::c_int,
                             player_increment[opponent as usize] as
                                 libc::c_int, timed_search, use_book,
                             skill[opponent as usize] - 1 as libc::c_int,
                             exact_skill[opponent as usize] -
                                 1 as libc::c_int,
                             wld_skill[opponent as usize] - 1 as libc::c_int,
                             1 as libc::c_int, &mut played_info2);
            unmake_move(side_to_move, curr_move);
            /* Determine the 'best' move and its score. For midgame moves,
            search twice to dampen oscillations. Unless we're in the endgame
             region, a private hash transform is used - see above. */
            if empties > wld_skill[side_to_move as usize] {
                set_hash_transformation(best_trans1, best_trans2);
                reset_counter(&mut nodes);
                curr_move =
                    compute_move(side_to_move, 0 as libc::c_int,
                                 player_time[side_to_move as usize] as
                                     libc::c_int,
                                 player_increment[side_to_move as usize] as
                                     libc::c_int, timed_search, use_book,
                                 skill[side_to_move as usize] -
                                     1 as libc::c_int,
                                 exact_skill[side_to_move as usize],
                                 wld_skill[side_to_move as usize],
                                 1 as libc::c_int, &mut best_info1)
            }
            reset_counter(&mut nodes);
            curr_move =
                compute_move(side_to_move, 0 as libc::c_int,
                             player_time[side_to_move as usize] as
                                 libc::c_int,
                             player_increment[side_to_move as usize] as
                                 libc::c_int, timed_search, use_book,
                             skill[side_to_move as usize],
                             exact_skill[side_to_move as usize],
                             wld_skill[side_to_move as usize],
                             1 as libc::c_int, &mut best_info2);
            if side_to_move == 0 as libc::c_int {
                set_evals(produce_compact_eval(best_info2), 0.0f64);
            } else { set_evals(0.0f64, produce_compact_eval(best_info2)); }
            /* Output the two score-move pairs */
            fprintf(output_stream,
                    b"%c%c \x00" as *const u8 as *const libc::c_char,
                    'a' as i32 + curr_move % 10 as libc::c_int -
                        1 as libc::c_int,
                    '0' as i32 + curr_move / 10 as libc::c_int);
            if empties <= exact_skill[side_to_move as usize] {
                fprintf(output_stream,
                        b"%+6d\x00" as *const u8 as *const libc::c_char,
                        best_info2.score / 128 as libc::c_int);
            } else if empties <= wld_skill[side_to_move as usize] {
                if best_info2.res as libc::c_uint ==
                       WON_POSITION as libc::c_int as libc::c_uint {
                    fputs(b"    +1\x00" as *const u8 as *const libc::c_char,
                          output_stream);
                } else if best_info2.res as libc::c_uint ==
                              LOST_POSITION as libc::c_int as libc::c_uint {
                    fputs(b"    -1\x00" as *const u8 as *const libc::c_char,
                          output_stream);
                } else {
                    fputs(b"     0\x00" as *const u8 as *const libc::c_char,
                          output_stream);
                }
            } else if curr_move == provided_move[disks_played as usize] &&
                          resp_move != -(1 as libc::c_int) {
                fprintf(output_stream,
                        b"%6.2f\x00" as *const u8 as *const libc::c_char,
                        -(played_info1.score + played_info2.score) as
                            libc::c_double /
                            (2 as libc::c_int as libc::c_double * 128.0f64));
            } else {
                fprintf(output_stream,
                        b"%6.2f\x00" as *const u8 as *const libc::c_char,
                        (best_info1.score + best_info2.score) as
                            libc::c_double /
                            (2 as libc::c_int as libc::c_double * 128.0f64));
            }
            curr_move = provided_move[disks_played as usize];
            fprintf(output_stream,
                    b"       %c%c \x00" as *const u8 as *const libc::c_char,
                    'a' as i32 + curr_move % 10 as libc::c_int -
                        1 as libc::c_int,
                    '0' as i32 + curr_move / 10 as libc::c_int);
            if resp_move == -(1 as libc::c_int) {
                fprintf(output_stream,
                        b"     ?\x00" as *const u8 as *const libc::c_char);
            } else if empties <= exact_skill[side_to_move as usize] {
                fprintf(output_stream,
                        b"%+6d\x00" as *const u8 as *const libc::c_char,
                        -played_info2.score / 128 as libc::c_int);
            } else if empties <= wld_skill[side_to_move as usize] {
                if played_info2.res as libc::c_uint ==
                       WON_POSITION as libc::c_int as libc::c_uint {
                    fputs(b"    -1\x00" as *const u8 as *const libc::c_char,
                          output_stream);
                } else if played_info2.res as libc::c_uint ==
                              LOST_POSITION as libc::c_int as libc::c_uint {
                    fputs(b"    +1\x00" as *const u8 as *const libc::c_char,
                          output_stream);
                } else {
                    fputs(b"     0\x00" as *const u8 as *const libc::c_char,
                          output_stream);
                }
            } else {
                fprintf(output_stream,
                        b"%6.2f\x00" as *const u8 as *const libc::c_char,
                        -(played_info1.score + played_info2.score) as
                            libc::c_double /
                            (2 as libc::c_int as libc::c_double * 128.0f64));
            }
            fputs(b"\n\x00" as *const u8 as *const libc::c_char,
                  output_stream);
            if valid_move(curr_move, side_to_move) == 0 {
                fatal_error(b"Invalid move %c%c in move sequence\x00" as
                                *const u8 as *const libc::c_char,
                            'a' as i32 + curr_move % 10 as libc::c_int -
                                1 as libc::c_int,
                            '0' as i32 + curr_move / 10 as libc::c_int);
            }
            move_stop = get_real_timer();
            if player_time[side_to_move as usize] != 10000000.0f64 {
                player_time[side_to_move as usize] -= move_stop - move_start
            }
            store_move(disks_played, curr_move);
            make_move(side_to_move, curr_move, 1 as libc::c_int);
            if side_to_move == 0 as libc::c_int {
                black_moves[score_sheet_row as usize] = curr_move
            } else {
                if white_moves[score_sheet_row as usize] !=
                       -(1 as libc::c_int) {
                    score_sheet_row += 1
                }
                white_moves[score_sheet_row as usize] = curr_move
            }
        } else if side_to_move == 0 as libc::c_int {
            black_moves[score_sheet_row as usize] = -(1 as libc::c_int)
        } else { white_moves[score_sheet_row as usize] = -(1 as libc::c_int) }
        side_to_move = 0 as libc::c_int + 2 as libc::c_int - side_to_move
    }
    if echo == 0 {
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        printf(b"Black level: %d\n\x00" as *const u8 as *const libc::c_char,
               skill[0 as libc::c_int as usize]);
        printf(b"White level: %d\n\x00" as *const u8 as *const libc::c_char,
               skill[2 as libc::c_int as usize]);
    }
    if side_to_move == 0 as libc::c_int { score_sheet_row += 1 }
    dump_game_score(side_to_move);
    if echo != 0 && one_position_only == 0 {
        set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                      score_sheet_row);
        set_times(floor(player_time[0 as libc::c_int as usize]) as
                      libc::c_int,
                  floor(player_time[2 as libc::c_int as usize]) as
                      libc::c_int);
        display_board(stdout, board.as_mut_ptr(), side_to_move,
                      1 as libc::c_int, use_timer, 1 as libc::c_int);
    }
    fclose(output_stream);
}
unsafe extern "C" fn run_endgame_script(mut in_file_name: *const libc::c_char,
                                        mut out_file_name:
                                            *const libc::c_char,
                                        mut display_line: libc::c_int) {
    let mut script_nodes = CounterType{hi: 0, lo: 0,};
    let mut eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut comment = 0 as *mut libc::c_char;
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
    let mut script_stream = 0 as *mut FILE;
    let mut output_stream = 0 as *mut FILE;
    /* If the played move is the best, output the already calculated
                   score for the best move - that way we avoid a subtle problem:
                   Suppose (N-1)-ply move is X but N-ply move is Y, where Y is
                   the best move. Then averaging the corresponding scores won't
                   coincide with the N-ply averaged score for Y. */
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
        let mut pass_count = 0 as libc::c_int;
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
                    let mut my_discs = disc_count(side_to_move);
                    let mut opp_discs =
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
/* File handling procedures */
/*
   DUMP_POSITION
   Saves the current board position to disk.
*/
unsafe extern "C" fn dump_position(mut side_to_move: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut stream = 0 as *mut FILE;
    stream =
        fopen(b"current.gam\x00" as *const u8 as *const libc::c_char,
              b"w\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() {
        fatal_error(b"File creation error when writing CURRENT.GAM\n\x00" as
                        *const u8 as *const libc::c_char);
    }
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            match board[(10 as libc::c_int * i + j) as usize] {
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
    fputs(b"\n\x00" as *const u8 as *const libc::c_char, stream);
    if side_to_move == 0 as libc::c_int {
        fputs(b"Black\x00" as *const u8 as *const libc::c_char, stream);
    } else {
        fputs(b"White\x00" as *const u8 as *const libc::c_char, stream);
    }
    fputs(b" to move\nThis file was automatically generated\n\x00" as
              *const u8 as *const libc::c_char, stream);
    fclose(stream);
}
/*
  DUMP_GAME_SCORE
  Writes the current game score to disk.
*/
unsafe extern "C" fn dump_game_score(mut side_to_move: libc::c_int) {
    let mut stream = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    stream =
        fopen(b"current.mov\x00" as *const u8 as *const libc::c_char,
              b"w\x00" as *const u8 as *const libc::c_char);
    if stream.is_null() {
        fatal_error(b"File creation error when writing CURRENT.MOV\n\x00" as
                        *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i <= score_sheet_row {
        fprintf(stream,
                b"   %2d.    \x00" as *const u8 as *const libc::c_char,
                i + 1 as libc::c_int);
        if black_moves[i as usize] == -(1 as libc::c_int) {
            fputs(b"- \x00" as *const u8 as *const libc::c_char, stream);
        } else {
            fprintf(stream, b"%c%c\x00" as *const u8 as *const libc::c_char,
                    'a' as i32 + black_moves[i as usize] % 10 as libc::c_int -
                        1 as libc::c_int,
                    '0' as i32 + black_moves[i as usize] / 10 as libc::c_int);
        }
        fputs(b"  \x00" as *const u8 as *const libc::c_char, stream);
        if i < score_sheet_row ||
               i == score_sheet_row && side_to_move == 0 as libc::c_int {
            if white_moves[i as usize] == -(1 as libc::c_int) {
                fputs(b"- \x00" as *const u8 as *const libc::c_char, stream);
            } else {
                fprintf(stream,
                        b"%c%c\x00" as *const u8 as *const libc::c_char,
                        'a' as i32 +
                            white_moves[i as usize] % 10 as libc::c_int -
                            1 as libc::c_int,
                        '0' as i32 +
                            white_moves[i as usize] / 10 as libc::c_int);
            }
        }
        fputs(b"\n\x00" as *const u8 as *const libc::c_char, stream);
        i += 1
    }
    fclose(stream);
}
#[main]
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
