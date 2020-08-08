use crate::src::display::{set_names, set_times, set_move_list, echo, set_evals, display_pv};
use crate::src::timer::{toggle_abort_check, get_real_timer, determine_move_time, start_move, clear_panic_abort};
use crate::src::moves::{disks_played, make_move, valid_move, move_count, move_list, generate_all, game_in_progress};
use crate::src::search::{disc_count, total_time, total_evaluations, total_nodes, produce_compact_eval};
use crate::src::counter::{counter_value, adjust_counter};
use crate::src::stubs::{floor, tolower, strlen};
use crate::src::thordb::{get_black_average_score, get_black_median_score, get_white_win_count, get_draw_count, get_black_win_count, get_match_count, database_search, choose_thor_opening_move};
use crate::src::globals::{board, score_sheet_row, white_moves, black_moves};
use crate::src::learn::{store_move, set_learning_parameters, clear_stored_game, Learner};
use crate::src::error::fatal_error;
use crate::src::myrandom::my_random;
use crate::src::eval::toggle_experimental;
use crate::src::osfbook::{fill_move_alternatives, find_opening_name, set_deviation_value, reset_book_search, set_slack};
use crate::src::getcoeff::remove_coeffs;
use crate::src::game::{toggle_human_openings, generic_game_init, FileBoardSource, generic_compute_move, ComputeMoveOutput, ComputeMoveLogger};
use crate::src::hash::setup_hash;

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
pub struct EvaluationType {
    pub type_0: EvalType,
    pub res: EvalResult,
    pub score: i32,
    pub confidence: f64,
    pub search_depth: i32,
    pub is_book: i32,
}
pub type DrawMode = u32;
pub const OPPONENT_WINS: DrawMode = 3;
pub const WHITE_WINS: DrawMode = 2;
pub const BLACK_WINS: DrawMode = 1;
pub const NEUTRAL: DrawMode = 0;
pub type GameMode = u32;
pub const PUBLIC_GAME: GameMode = 1;
pub const PRIVATE_GAME: GameMode = 0;

/* Local variables */
pub static mut slack: f64 = 0.25f64;
pub static mut dev_bonus: f64 = 0.0f64;
pub static mut low_thresh: i32 = 0 as i32;
pub static mut high_thresh: i32 = 0 as i32;
pub static mut rand_move_freq: i32 = 0 as i32;
pub static mut tournament: i32 = 0 as i32;
pub static mut tournament_levels: i32 = 0;
pub static mut deviation_depth: i32 = 0;
pub static mut cutoff_empty: i32 = 0;
pub static mut one_position_only: i32 = 0 as i32;
pub static mut use_timer: i32 = 0 as i32;
pub static mut only_analyze: i32 = 0 as i32;
pub static mut thor_max_games: i32 = 0;
pub static mut tournament_skill: [[i32; 3]; 8] = [[0; 3]; 8];
pub static mut wld_skill: [i32; 3] = [0; 3];
pub static mut exact_skill: [i32; 3] = [0; 3];
pub static mut player_time: [f64; 3] = [0.; 3];
pub static mut player_increment: [f64; 3] = [0.; 3];
pub static mut skill: [i32; 3] = [0; 3];
pub static mut wait: i32 = 0;
pub static mut use_book: i32 = 1 as i32;
pub static mut wld_only: i32 = 0 as i32;
pub static mut use_learning: i32 = 0;
pub static mut use_thor: i32 = 0;


/// This trait is unsafe because line buffer is used as a c-style string later
/// so this function needs to ensure that the line_buffer contains at
/// least one null character (there's definitely better way to do this, but I
/// don't want to deviate from the original source for first implementation)
pub unsafe trait InitialMoveSource {
    fn fill_line_buffer(&mut self, line_buffer: &mut [i8; 1000]);
}


pub unsafe fn set_names_from_skills() {
    let mut black_name = 0 as *const i8;
    if skill[0 as i32 as usize] == 0 as i32 {
        black_name = b"Player\x00" as *const u8 as *const i8
    } else {
        black_name = b"Zebra\x00" as *const u8 as *const i8
    }
    let mut white_name = 0 as *const i8;
    if skill[2 as i32 as usize] == 0 as i32 {
        white_name = b"Player\x00" as *const u8 as *const i8
    } else {
        white_name = b"Zebra\x00" as *const u8 as *const i8
    }
    set_names(black_name, white_name);
}

pub trait ZebraFrontend {
    fn report_some_thor_scores(black_win_count: i32, draw_count: i32, white_win_count: i32, black_median_score: i32, black_average_score: f64);
    fn report_some_thor_stats(total_search_time: f64, thor_position_count: i32, db_search_time: f64);
    fn display_board_after_thor(side_to_move: i32);
    fn print_out_thor_matches();
    unsafe fn log_game_ending(log_file_name_: *mut i8, move_vec: &mut [i8; 121], first_side_to_move: i32, second_side_to_move: i32);
    unsafe fn push_move(move_vec: &mut [i8; 121], curr_move: i32, disks_played_: i32);
    fn get_pass();
    fn report_engine_override();
    fn ui_get_move(side_to_move: i32) -> i32;
    fn report_after_game_ended(node_val: f64, eval_val: f64, black_disc_count: i32, white_disc_count: i32, total_time_: f64);
    fn report_skill_levels(black_level: i32, white_level: i32);
    fn report_thor_matching_games_stats(total_search_time: f64, thor_position_count: i32, database_time: f64);
    unsafe fn clear_moves();
    fn report_thor_stats(black_win_count: i32, draw_count: i32, white_win_count: i32, black_median_score: i32, black_average_score: f64);
    unsafe fn report_opening_name(opening_name: *const i8);
    fn report_book_randomness(slack_: f64);
    unsafe fn load_thor_files();
    fn print_move_alternatives(side_to_move: i32);
    fn dumpch();
}
pub trait DumpHandler {
    fn dump_position(side_to_move: i32);
    fn dump_game_score(side_to_move: i32);
}


pub unsafe fn engine_play_game<
    ZF: ZebraFrontend,
    Source: InitialMoveSource,
    Dump: DumpHandler,
    BoardSrc : FileBoardSource,
    ComputeMoveLog: ComputeMoveLogger,
    ComputeMoveOut: ComputeMoveOutput,
    Learn: Learner
>(
    mut file_name: *const i8, mut move_string: *const i8,
    mut repeat: i32, log_file_name_: *mut i8,
    mut move_file: Option<Source>) {
    let mut eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
            res: WON_POSITION,
            score: 0,
            confidence: 0.,
            search_depth: 0,
            is_book: 0,};
    let mut total_search_time = 0.0f64;
    let mut side_to_move: i32 = 0;
    let mut curr_move: i32 = 0;
    let mut rand_color = 0 as i32;
    let mut thor_position_count: i32 = 0;
    let mut provided_move: [i32; 61] = [0; 61];
    let mut move_vec: [i8; 121] = [0; 121];
    let mut line_buffer: [i8; 1000] = [0; 1000];

    loop  {
        /* Decode the predefined move sequence */
        if let Some(ref mut move_file) = &mut move_file {
            move_file.fill_line_buffer(&mut line_buffer);
            move_string = line_buffer.as_mut_ptr()
        }
        let mut provided_move_count = 0 as i32;
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
                fatal_error(b"Invalid move string provided\x00" as *const u8
                    as *const i8);
            }
            let mut i = 0 as i32;
            while i < provided_move_count {
                let col =
                    tolower(*move_string.offset((2 as i32 * i) as
                        isize) as i32) -
                        'a' as i32 + 1 as i32;
                let row =
                    *move_string.offset((2 as i32 * i +
                        1 as i32) as isize) as
                        i32 - '0' as i32;
                if col < 1 as i32 || col > 8 as i32 ||
                    row < 1 as i32 || row > 8 as i32 {
                    fatal_error(b"Unexpected character in move string\x00" as
                        *const u8 as *const i8);
                }
                provided_move[i as usize] = 10 as i32 * row + col;
                i += 1
            }
        }
        /* Set up the position and the search engine */
        generic_game_init::<BoardSrc>(file_name, &mut side_to_move);
        setup_hash(1 as i32);
        clear_stored_game();
        if echo != 0 && use_book != 0 {
            let slack_ = slack;
            ZF::report_book_randomness(slack_);
        }
        set_slack(floor(slack * 128.0f64) as i32);
        toggle_human_openings(0 as i32);
        if use_learning != 0 {
            set_learning_parameters(deviation_depth, cutoff_empty);
        }
        reset_book_search();
        set_deviation_value(low_thresh, high_thresh, dev_bonus);
        if use_thor != 0 {
            ZF::load_thor_files();
        }
        set_names_from_skills();
        set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                      score_sheet_row);
        set_evals(0.0f64, 0.0f64);
        ZF::clear_moves();
        move_vec[0 as i32 as usize] = 0 as i32 as i8;
        // these are not used because their usage was disabled by preprocessor
        // byt for deterministic testing, we need to call random the same way, so we keep them.
        let _black_hash1 = my_random() as i32;
        let _black_hash2 = my_random() as i32;
        let _white_hash1 = my_random() as i32;
        let _white_hash2 = my_random() as i32;
        while game_in_progress() != 0 {
            remove_coeffs(disks_played);
            generate_all(side_to_move);
            if side_to_move == 0 as i32 { score_sheet_row += 1 }
            if move_count[disks_played as usize] != 0 as i32 {
                let move_start = get_real_timer();
                clear_panic_abort();
                if echo != 0 {
                    set_move_list(black_moves.as_mut_ptr(),
                                  white_moves.as_mut_ptr(), score_sheet_row);
                    set_times(floor(player_time[0 as i32 as usize]) as
                                  i32,
                              floor(player_time[2 as i32 as usize]) as
                                  i32);
                    let opening_name = find_opening_name();
                    if !opening_name.is_null() {
                        ZF::report_opening_name(opening_name);
                    }
                    if use_thor != 0 {
                        let database_start = get_real_timer();
                        database_search(board.as_mut_ptr(), side_to_move);
                        thor_position_count = get_match_count();
                        let database_stop = get_real_timer();
                        let database_time = database_stop - database_start;
                        total_search_time += database_time;
                        ZF::report_thor_matching_games_stats(total_search_time, thor_position_count, database_time);
                        if thor_position_count > 0 as i32 {
                            let black_win_count = get_black_win_count();
                            let draw_count = get_draw_count();
                            let white_win_count = get_white_win_count();
                            let black_median_score = get_black_median_score();
                            let black_average_score = get_black_average_score();

                            ZF::report_thor_stats(black_win_count, draw_count, white_win_count, black_median_score, black_average_score);
                        }
                        ZF::print_out_thor_matches();
                    }
                    ZF::display_board_after_thor(side_to_move);
                }
                Dump::dump_position(side_to_move);
                Dump::dump_game_score(side_to_move);
                /* Check what the Thor opening statistics has to say */
                choose_thor_opening_move(board.as_mut_ptr(), side_to_move,
                                         echo);
                if echo != 0 && wait != 0 { ZF::dumpch(); }
                if disks_played >= provided_move_count {
                    if skill[side_to_move as usize] == 0 as i32 {
                        if use_book != 0 && display_pv != 0 {
                            fill_move_alternatives(side_to_move,
                                                   0 as i32);
                            if echo != 0 {
                                ZF::print_move_alternatives(side_to_move);
                            }
                        }
                        curr_move = ZF::ui_get_move(side_to_move);
                    } else {
                        start_move(player_time[side_to_move as usize],
                                   player_increment[side_to_move as usize],
                                   disks_played + 4 as i32);
                        determine_move_time(player_time[side_to_move as
                            usize],
                                            player_increment[side_to_move as
                                                usize],
                                            disks_played + 4 as i32);
                        let timed_search =
                            (skill[side_to_move as usize] >=
                                60 as i32) as i32;
                        toggle_experimental(0 as i32);
                        curr_move =
                            generic_compute_move::<ComputeMoveLog, ComputeMoveOut>(
                                side_to_move, 1 as i32,
                                player_time[side_to_move as usize] as
                                    i32,
                                player_increment[side_to_move as
                                    usize] as
                                    i32, timed_search,
                                use_book,
                                skill[side_to_move as usize],
                                exact_skill[side_to_move as usize],
                                wld_skill[side_to_move as usize],
                                0 as i32, &mut eval_info,
                                &mut ComputeMoveLog::create_log_file_if_needed());
                        if side_to_move == 0 as i32 {
                            set_evals(produce_compact_eval(eval_info),
                                      0.0f64);
                        } else {
                            set_evals(0.0f64,
                                      produce_compact_eval(eval_info));
                        }
                        if eval_info.is_book != 0 &&
                            rand_move_freq > 0 as i32 &&
                            side_to_move == rand_color &&
                            my_random() % rand_move_freq as i64 ==
                                0 as i32 as i64 {
                            ZF::report_engine_override();
                            rand_color =
                                0 as i32 + 2 as i32 -
                                    rand_color;
                            curr_move =
                                move_list[disks_played as
                                    usize][(my_random() %
                                    move_count[disks_played
                                        as
                                        usize]
                                        as i64)
                                    as usize]
                        }
                    }
                } else {
                    curr_move = provided_move[disks_played as usize];
                    if valid_move(curr_move, side_to_move) == 0 {
                        fatal_error(b"Invalid move %c%c in move sequence\x00"
                                        as *const u8 as *const i8,
                                    'a' as i32 + curr_move % 10 as i32
                                        - 1 as i32,
                                    '0' as i32 +
                                        curr_move / 10 as i32);
                    }
                }
                let move_stop = get_real_timer();
                if player_time[side_to_move as usize] != 10000000.0f64 {
                    player_time[side_to_move as usize] -=
                        move_stop - move_start
                }
                store_move(disks_played, curr_move);
                ZF::push_move(&mut move_vec, curr_move, disks_played);
                make_move(side_to_move, curr_move, 1 as i32);
                if side_to_move == 0 as i32 {
                    black_moves[score_sheet_row as usize] = curr_move
                } else {
                    if white_moves[score_sheet_row as usize] !=
                        -(1 as i32) {
                        score_sheet_row += 1
                    }
                    white_moves[score_sheet_row as usize] = curr_move
                }
            } else {
                if side_to_move == 0 as i32 {
                    black_moves[score_sheet_row as usize] =
                        -(1 as i32)
                } else {
                    white_moves[score_sheet_row as usize] =
                        -(1 as i32)
                }
                if skill[side_to_move as usize] == 0 as i32 {
                    ZF::get_pass();
                }
            }
            side_to_move = 0 as i32 + 2 as i32 - side_to_move;
            if one_position_only != 0 { break ; }
        }
        if echo == 0 && one_position_only == 0 {
            let black_level = skill[0 as i32 as usize];
            let white_level = skill[2 as i32 as usize];
            ZF::report_skill_levels(black_level, white_level);
        }
        if side_to_move == 0 as i32 { score_sheet_row += 1 }
        Dump::dump_game_score(side_to_move);
        if echo != 0 && one_position_only == 0 {
            set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                          score_sheet_row);
            if use_thor != 0 {
                let database_start = get_real_timer();
                database_search(board.as_mut_ptr(), side_to_move);
                thor_position_count = get_match_count();
                let database_stop = get_real_timer();
                let db_search_time = database_stop - database_start;
                total_search_time += db_search_time;
                ZF::report_some_thor_stats(total_search_time, thor_position_count, db_search_time);
                if thor_position_count > 0 as i32 {
                    let black_win_count = get_black_win_count();
                    let draw_count = get_draw_count();
                    let white_win_count = get_white_win_count();
                    let black_median_score = get_black_median_score();
                    let black_average_score = get_black_average_score();
                    ZF::report_some_thor_scores(black_win_count, draw_count, white_win_count, black_median_score, black_average_score);
                }
                ZF::print_out_thor_matches();
            }
            set_times(floor(player_time[0 as i32 as usize]) as
                          i32,
                      floor(player_time[2 as i32 as usize]) as
                          i32);
            ZF::display_board_after_thor(side_to_move);
        }
        adjust_counter(&mut total_nodes);
        let node_val = counter_value(&mut total_nodes);
        adjust_counter(&mut total_evaluations);
        let eval_val = counter_value(&mut total_evaluations);
        let black_disc_count = disc_count(0 as i32);
        let white_disc_count = disc_count(2 as i32);
        let total_time_ = total_time;
        ZF::report_after_game_ended(node_val, eval_val, black_disc_count, white_disc_count, total_time_);

        if !log_file_name_.is_null() && one_position_only == 0 {
            ZF::log_game_ending(log_file_name_,
                                &mut move_vec,
                                disc_count(0 as i32),
                                disc_count(2 as i32))
        }
        repeat -= 1;
        toggle_abort_check(0 as i32);
        if use_learning != 0 && one_position_only == 0 {
            Learn::learn_game(disks_played,
                              (skill[0 as i32 as usize] != 0 as i32
                                  &&
                                  skill[2 as i32 as usize] !=
                                      0 as i32) as i32,
                              (repeat == 0 as i32) as i32);
        }
        toggle_abort_check(1 as i32);
        if !(repeat > 0 as i32) { break ; }
    }
}
