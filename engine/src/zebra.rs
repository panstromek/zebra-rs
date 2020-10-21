use crate::src::display::{echo,display_pv};
use crate::src::timer::{toggle_abort_check, get_real_timer, determine_move_time, start_move, clear_panic_abort};
use crate::src::moves::{disks_played, make_move, valid_move, move_count, move_list, generate_all, game_in_progress, get_move, get_move_async};
use crate::src::search::{disc_count, total_time, total_evaluations, total_nodes, produce_compact_eval};
use crate::src::counter::{counter_value, adjust_counter};
use crate::src::stubs::{floor};
// use crate::src::thordb::{get_black_average_score, get_black_median_score, get_white_win_count, get_draw_count, get_black_win_count, get_match_count, database_search, choose_thor_opening_move};
use crate::src::globals::{board, score_sheet_row, white_moves, black_moves};
use crate::src::learn::{store_move, set_learning_parameters, clear_stored_game, Learner};
use crate::src::error::{FrontEnd};
use crate::src::myrandom::my_random;
use crate::src::osfbook::{fill_move_alternatives, find_opening_name, set_deviation_value, reset_book_search, set_slack, g_book};
use crate::src::getcoeff::remove_coeffs;
use crate::src::game::{toggle_human_openings, generic_game_init, FileBoardSource, generic_compute_move, ComputeMoveOutput, ComputeMoveLogger};
use crate::src::hash::{setup_hash};
use std::future::Future;
use std::error::Error;
use crate::src::thordb::ThorDatabase;


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
pub static mut low_thresh: i32 = 0;
pub static mut high_thresh: i32 = 0;
pub static mut rand_move_freq: i32 = 0;
pub static mut tournament: i32 = 0;
pub static mut tournament_levels: i32 = 0;
pub static mut deviation_depth: i32 = 0;
pub static mut cutoff_empty: i32 = 0;
pub static mut one_position_only: i32 = 0;
pub static mut use_timer: i32 = 0;
pub static mut only_analyze: i32 = 0;
pub static mut thor_max_games: i32 = 0;
pub static mut tournament_skill: [[i32; 3]; 8] = [[0; 3]; 8];
pub static mut wld_skill: [i32; 3] = [0; 3];
pub static mut exact_skill: [i32; 3] = [0; 3];
pub static mut player_time: [f64; 3] = [0.; 3];
pub static mut player_increment: [f64; 3] = [0.; 3];
pub static mut skill: [i32; 3] = [0; 3];
pub static mut wait: i32 = 0;
pub static mut use_book: i32 = 1;
pub static wld_only: i32 = 0;


pub unsafe fn set_default_engine_globals() {
    wait = 0;
    echo = 1;
    display_pv = 1;
    skill[2] = -1;
    skill[0] = -1;
    player_time[2] = 10000000.0f64;
    player_time[0] = 10000000.0f64;
    player_increment[2] = 0.0f64;
    player_increment[0] = 0.0f64;
}
/// This trait is unsafe because line buffer is used as a c-style string later
/// so this function needs to ensure that the line_buffer contains at
/// least one null character (there's definitely better way to do this, but I
/// don't want to deviate from the original source for first implementation)
pub unsafe trait InitialMoveSource {
    fn fill_line_buffer(&mut self, line_buffer: &mut [i8; 1000]);
}


pub unsafe fn set_names_from_skills<ZF: ZebraFrontend>() {
    let black_name = if skill[0] == 0 {
        b"Player\x00" as *const u8 as *const i8
    } else {
        b"Zebra\x00" as *const u8 as *const i8
    };
    let white_name = if skill[2] == 0 {
        b"Player\x00" as *const u8 as *const i8
    } else {
        b"Zebra\x00" as *const u8 as *const i8
    };
    ZF::set_names(black_name, white_name);
}

pub trait ZebraFrontend {
    fn set_evals(black: f64, white: f64);
    unsafe fn set_move_list(black: *mut i32, white: *mut i32, row: i32);
    unsafe fn set_names(black_name: *const i8, white_name: *const i8);
    fn set_times(black: i32, white: i32);
    fn report_some_thor_scores(black_win_count: i32, draw_count: i32, white_win_count: i32, black_median_score: i32, black_average_score: f64);
    fn report_some_thor_stats(total_search_time: f64, thor_position_count: i32, db_search_time: f64);
    unsafe fn display_board_after_thor(side_to_move: i32, give_time_: i32, board_: &[i32; 128],
                                black_moves_: &[i32; 60], white_moves_: &[i32; 60]);
    fn print_out_thor_matches(thor_max_games_: i32);
    unsafe fn log_game_ending(log_file_name_: *mut i8, move_vec: &[i8; 121], first_side_to_move: i32, second_side_to_move: i32);
    unsafe fn push_move(move_vec: &mut [i8; 121], curr_move: i32, disks_played_: i32);
    fn get_pass();
    fn report_engine_override();
    fn prompt_get_move(side_to_move: i32, buffer: &mut [i8; 255]) -> i32;
    fn before_get_move();
    fn report_after_game_ended(node_val: f64, eval_val: f64, black_disc_count: i32, white_disc_count: i32, total_time_: f64);
    fn report_skill_levels(black_level: i32, white_level: i32);
    fn report_thor_matching_games_stats(total_search_time: f64, thor_position_count: i32, database_time: f64);
    fn report_thor_stats(black_win_count: i32, draw_count: i32, white_win_count: i32, black_median_score: i32, black_average_score: f64);
    unsafe fn report_opening_name(opening_name: *const i8);
    fn report_book_randomness(slack_: f64);
    unsafe fn load_thor_files();
    fn print_move_alternatives(side_to_move: i32);
    fn dumpch();
}
/* File handling procedures */
pub trait DumpHandler {
    fn dump_position(side_to_move: i32, board_: &[i32; 128]);
    fn dump_game_score(side_to_move: i32, score_sheet_row_: i32, x: &[i32; 60], x: &[i32; 60]);
}


pub unsafe fn engine_play_game<
    ZF: ZebraFrontend,
    Source: InitialMoveSource,
    Dump: DumpHandler,
    BoardSrc : FileBoardSource,
    ComputeMoveLog: ComputeMoveLogger,
    ComputeMoveOut: ComputeMoveOutput,
    Learn: Learner,
    FE: FrontEnd,
    Thor: ThorDatabase
>(file_name: *const i8, mut move_string: *const i8,
                  mut repeat: i32, log_file_name_: *mut i8,
                  mut move_file: Option<Source>, use_thor_: bool, use_learning_: bool) {
    let mut eval_info = EvaluationType {
        type_0: MIDGAME_EVAL,
        res: WON_POSITION,
        score: 0,
        confidence: 0.,
        search_depth: 0,
        is_book: 0,
    };
    let mut total_search_time = 0.0f64;
    let mut side_to_move = 0;
    let mut curr_move = 0;
    let mut rand_color = 0;
    let mut thor_position_count= 0;
    let mut provided_move = [0; 61];
    let mut move_vec = [0; 121];
    let mut line_buffer = [0; 1000];

    loop  {
        /* Decode the predefined move sequence */
        if let Some(ref mut move_file) = &mut move_file {
            move_file.fill_line_buffer(&mut line_buffer);
            move_string = line_buffer.as_mut_ptr()
        }
        let mut provided_move_count = 0;
        if move_string.is_null() {
            provided_move_count = 0
        } else {
            provided_move_count = FE::strlen(move_string).wrapping_div(2) as i32;
            if provided_move_count > 60 ||
                FE::strlen(move_string).wrapping_rem(2) == 1 {
                FE::invalid_move_string_provided();
            }
            let mut i = 0;
            while i < provided_move_count {
                let col = FE::tolower(
                    *move_string.offset((2 * i) as _) as i32) - 'a' as i32 + 1;
                let row =
                    *move_string.offset((2 * i + 1) as _) as i32 - '0' as i32;
                if col < 1 || col > 8 || row < 1 || row > 8 {
                    FE::unexpected_character_in_a_move_string();
                }
                provided_move[i as usize] = 10 * row + col;
                i += 1
            }
        }
        /* Set up the position and the search engine */
        generic_game_init::<BoardSrc, FE>(file_name, &mut side_to_move);
        setup_hash(1);
        clear_stored_game();
        if echo != 0 && use_book != 0 {
            let slack_ = slack;
            ZF::report_book_randomness(slack_);
        }
        set_slack(floor(slack * 128.0f64) as i32);
        toggle_human_openings(0);
        if use_learning_ {
            set_learning_parameters(deviation_depth, cutoff_empty);
        }
        reset_book_search();
        set_deviation_value(low_thresh, high_thresh, dev_bonus, &mut g_book);
        if use_thor_ {
            ZF::load_thor_files();
        }
        set_names_from_skills::<ZF>();
        ZF::set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                      score_sheet_row);
        ZF::set_evals(0.0f64, 0.0f64);
        clear_moves();
        move_vec[0] = 0;
        // these are not used because their usage was disabled by preprocessor
        // byt for deterministic testing, we need to call random the same way, so we keep them.
        let _black_hash1 = my_random();
        let _black_hash2 = my_random();
        let _white_hash1 = my_random();
        let _white_hash2 = my_random();
        while game_in_progress() != 0 {
            remove_coeffs(disks_played);
            generate_all(side_to_move);
            if side_to_move == 0 {
                score_sheet_row += 1
            }
            if move_count[disks_played as usize] != 0 {
                let move_start = get_real_timer::<FE>();
                clear_panic_abort();
                if echo != 0 {
                    ZF::set_move_list(black_moves.as_mut_ptr(),
                                  white_moves.as_mut_ptr(), score_sheet_row);
                    ZF::set_times(floor(player_time[0]) as i32,
                              floor(player_time[2]) as i32);
                    let opening_name = find_opening_name();
                    if !opening_name.is_null() {
                        ZF::report_opening_name(opening_name);
                    }
                    if use_thor_ {
                        let database_start = get_real_timer::<FE>();
                        Thor::database_search(&board, side_to_move);
                        thor_position_count = Thor::get_match_count();
                        let database_stop = get_real_timer::<FE>();
                        let database_time = database_stop - database_start;
                        total_search_time += database_time;
                        ZF::report_thor_matching_games_stats(total_search_time, thor_position_count, database_time);
                        if thor_position_count > 0 as i32 {
                            let black_win_count = Thor::get_black_win_count();
                            let draw_count = Thor::get_draw_count();
                            let white_win_count = Thor::get_white_win_count();
                            let black_median_score = Thor::get_black_median_score();
                            let black_average_score = Thor::get_black_average_score();

                            ZF::report_thor_stats(black_win_count, draw_count, white_win_count, black_median_score, black_average_score);
                        }
                        ZF::print_out_thor_matches(thor_max_games);
                    }
                    ZF::display_board_after_thor(side_to_move, use_timer,
                                                 &board, &black_moves, &white_moves);
                }
                Dump::dump_position(side_to_move, &board);
                Dump::dump_game_score(side_to_move, score_sheet_row, &black_moves, &white_moves);
                /* Check what the Thor opening statistics has to say */
                Thor::choose_thor_opening_move(&board, side_to_move, echo);
                if echo != 0 && wait != 0 { ZF::dumpch(); }
                if disks_played >= provided_move_count {
                    if skill[side_to_move as usize] == 0 as i32 {
                        if use_book != 0 && display_pv != 0 {
                            fill_move_alternatives::<FE>(side_to_move,
                                                   0 as i32);
                            if echo != 0 {
                                ZF::print_move_alternatives(side_to_move);
                            }
                        }
                        ZF::before_get_move();
                        curr_move = get_move::<ZF>(side_to_move);
                    } else {
                        start_move::<FE>(player_time[side_to_move as usize],
                                   player_increment[side_to_move as usize],
                                   disks_played + 4);
                        determine_move_time(player_time[side_to_move as usize],
                                            player_increment[side_to_move as usize],
                                            disks_played + 4);
                        let timed_search = (skill[side_to_move as usize] >= 60) as i32;
                        curr_move =
                            generic_compute_move::<ComputeMoveLog, ComputeMoveOut, FE, Thor>(
                                side_to_move, 1,
                                player_time[side_to_move as usize] as i32,
                                player_increment[side_to_move as usize] as i32, timed_search,
                                use_book,
                                skill[side_to_move as usize],
                                exact_skill[side_to_move as usize],
                                wld_skill[side_to_move as usize],
                                0 as i32, &mut eval_info,
                                &mut ComputeMoveLog::create_log_file_if_needed());
                        if side_to_move == 0 as i32 {
                            ZF::set_evals(produce_compact_eval(eval_info), 0.0f64);
                        } else {
                            ZF::set_evals(0.0f64, produce_compact_eval(eval_info));
                        }
                        if eval_info.is_book != 0 &&
                            rand_move_freq > 0 &&
                            side_to_move == rand_color &&
                            my_random() % rand_move_freq as i64 == 0 {
                            ZF::report_engine_override();
                            rand_color = 2 - rand_color;
                            curr_move = move_list[disks_played as usize]
                                [(my_random() % move_count[disks_played as usize] as i64) as usize]
                        }
                    }
                } else {
                    curr_move = provided_move[disks_played as usize];
                    if valid_move(curr_move, side_to_move) == 0 {
                        FE::invalid_move_in_move_sequence(curr_move);
                    }
                }
                let move_stop = get_real_timer::<FE>();
                if player_time[side_to_move as usize] != 10000000.0f64 {
                    player_time[side_to_move as usize] -= move_stop - move_start
                }
                store_move(disks_played, curr_move);
                ZF::push_move(&mut move_vec, curr_move, disks_played);
                make_move(side_to_move, curr_move, 1);
                if side_to_move == 0 as i32 {
                    black_moves[score_sheet_row as usize] = curr_move
                } else {
                    if white_moves[score_sheet_row as usize] != -(1) {
                        score_sheet_row += 1
                    }
                    white_moves[score_sheet_row as usize] = curr_move
                }
            } else {
                if side_to_move == 0 {
                    black_moves[score_sheet_row as usize] = -(1)
                } else {
                    white_moves[score_sheet_row as usize] = -(1)
                }
                if skill[side_to_move as usize] == 0 {
                    ZF::get_pass();
                }
            }
            side_to_move = 2 - side_to_move;
            if one_position_only != 0 { break; }
        }
        if echo == 0 && one_position_only == 0 {
            let black_level = skill[0];
            let white_level = skill[2];
            ZF::report_skill_levels(black_level, white_level);
        }
        if side_to_move == 0 as i32 { score_sheet_row += 1 }
        Dump::dump_game_score(side_to_move, score_sheet_row, &black_moves, &white_moves);
        if echo != 0 && one_position_only == 0 {
            ZF::set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                          score_sheet_row);
            if use_thor_ {
                let database_start = get_real_timer::<FE>();
                Thor::database_search(&board, side_to_move);
                thor_position_count = Thor::get_match_count();
                let database_stop = get_real_timer::<FE>();
                let db_search_time = database_stop - database_start;
                total_search_time += db_search_time;
                ZF::report_some_thor_stats(total_search_time, thor_position_count, db_search_time);
                if thor_position_count > 0 {
                    let black_win_count = Thor::get_black_win_count();
                    let draw_count = Thor::get_draw_count();
                    let white_win_count = Thor::get_white_win_count();
                    let black_median_score = Thor::get_black_median_score();
                    let black_average_score = Thor::get_black_average_score();
                    ZF::report_some_thor_scores(black_win_count, draw_count, white_win_count, black_median_score, black_average_score);
                }
                ZF::print_out_thor_matches(thor_max_games);
            }
            ZF::set_times(floor(player_time[0]) as _, floor(player_time[2]) as _);
            ZF::display_board_after_thor(side_to_move, use_timer, &board,
                                          &black_moves,
                                         &white_moves,
            );
        }
        adjust_counter(&mut total_nodes);
        let node_val = counter_value(&mut total_nodes);
        adjust_counter(&mut total_evaluations);
        let eval_val = counter_value(&mut total_evaluations);
        let black_disc_count = disc_count(0, &board);
        let white_disc_count = disc_count(2, &board);
        let total_time_ = total_time;
        ZF::report_after_game_ended(node_val, eval_val, black_disc_count, white_disc_count, total_time_);

        if !log_file_name_.is_null() && one_position_only == 0 {
            ZF::log_game_ending(log_file_name_,
                                &mut move_vec,
                                disc_count(0, &board),
                                disc_count(2, &board))
        }
        repeat -= 1;
        toggle_abort_check(0 as i32);
        if use_learning_ && one_position_only == 0 {
            Learn::learn_game(disks_played,
                              (skill[0] != 0 && skill[2] != 0) as i32,
                              (repeat == 0 as i32) as i32);
        }
        toggle_abort_check(1);
        if !(repeat > 0) { break; }
    }
}
pub async unsafe fn engine_play_game_async<
    ZF: ZebraFrontend,
    Source: InitialMoveSource,
    Dump: DumpHandler,
    BoardSrc : FileBoardSource,
    ComputeMoveLog: ComputeMoveLogger,
    ComputeMoveOut: ComputeMoveOutput,
    Learn: Learner,
    FE: FrontEnd,
    Thor: ThorDatabase,
    GetMove,
    Fut
>(file_name: *const i8, mut move_string: *const i8,
  mut repeat: i32, log_file_name_: *mut i8,
  mut move_file: Option<Source>, use_thor_: bool,
  use_learning_: bool, mut get_move_cb: GetMove) -> Result<(), Box<dyn Error>>
    where
        GetMove: FnMut(i32) -> Fut,
        Fut: Future<Output=Result<i32, Box<dyn Error>>>
{
    let mut eval_info = EvaluationType {
        type_0: MIDGAME_EVAL,
        res: WON_POSITION,
        score: 0,
        confidence: 0.,
        search_depth: 0,
        is_book: 0,
    };
    let mut total_search_time = 0.0f64;
    let mut side_to_move = 0;
    let mut curr_move = 0;
    let mut rand_color = 0;
    let mut thor_position_count= 0;
    let mut provided_move = [0; 61];
    let mut move_vec = [0; 121];
    let mut line_buffer = [0; 1000];

    loop  {
        /* Decode the predefined move sequence */
        if let Some(ref mut move_file) = &mut move_file {
            move_file.fill_line_buffer(&mut line_buffer);
            move_string = line_buffer.as_mut_ptr()
        }
        let mut provided_move_count = 0;
        if move_string.is_null() {
            provided_move_count = 0
        } else {
            provided_move_count = FE::strlen(move_string).wrapping_div(2) as i32;
            if provided_move_count > 60 ||
                FE::strlen(move_string).wrapping_rem(2) == 1 {
                FE::invalid_move_string_provided();
            }
            let mut i = 0;
            while i < provided_move_count {
                let col = FE::tolower(
                    *move_string.offset((2 * i) as _) as i32) - 'a' as i32 + 1;
                let row =
                    *move_string.offset((2 * i + 1) as _) as i32 - '0' as i32;
                if col < 1 || col > 8 || row < 1 || row > 8 {
                    FE::unexpected_character_in_a_move_string();
                }
                provided_move[i as usize] = 10 * row + col;
                i += 1
            }
        }
        /* Set up the position and the search engine */
        generic_game_init::<BoardSrc, FE>(file_name, &mut side_to_move);
        setup_hash(1);
        clear_stored_game();
        if echo != 0 && use_book != 0 {
            let slack_ = slack;
            ZF::report_book_randomness(slack_);
        }
        set_slack(floor(slack * 128.0f64) as i32);
        toggle_human_openings(0);
        if use_learning_ {
            set_learning_parameters(deviation_depth, cutoff_empty);
        }
        reset_book_search();
        set_deviation_value(low_thresh, high_thresh, dev_bonus, &mut g_book);
        if use_thor_ {
            ZF::load_thor_files();
        }
        set_names_from_skills::<ZF>();
        ZF::set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                      score_sheet_row);
        ZF::set_evals(0.0f64, 0.0f64);
        clear_moves();
        move_vec[0] = 0;
        // these are not used because their usage was disabled by preprocessor
        // byt for deterministic testing, we need to call random the same way, so we keep them.
        let _black_hash1 = my_random();
        let _black_hash2 = my_random();
        let _white_hash1 = my_random();
        let _white_hash2 = my_random();
        while game_in_progress() != 0 {
            remove_coeffs(disks_played);
            generate_all(side_to_move);
            if side_to_move == 0 {
                score_sheet_row += 1
            }
            if move_count[disks_played as usize] != 0 {
                let move_start = get_real_timer::<FE>();
                clear_panic_abort();
                if echo != 0 {
                    ZF::set_move_list(black_moves.as_mut_ptr(),
                                  white_moves.as_mut_ptr(), score_sheet_row);
                    ZF::set_times(floor(player_time[0]) as i32,
                              floor(player_time[2]) as i32);
                    let opening_name = find_opening_name();
                    if !opening_name.is_null() {
                        ZF::report_opening_name(opening_name);
                    }
                    if use_thor_ {
                        let database_start = get_real_timer::<FE>();
                        Thor::database_search(&board, side_to_move);
                        thor_position_count = Thor::get_match_count();
                        let database_stop = get_real_timer::<FE>();
                        let database_time = database_stop - database_start;
                        total_search_time += database_time;
                        ZF::report_thor_matching_games_stats(total_search_time, thor_position_count, database_time);
                        if thor_position_count > 0 as i32 {
                            let black_win_count = Thor::get_black_win_count();
                            let draw_count = Thor::get_draw_count();
                            let white_win_count = Thor::get_white_win_count();
                            let black_median_score = Thor::get_black_median_score();
                            let black_average_score = Thor::get_black_average_score();

                            ZF::report_thor_stats(black_win_count, draw_count, white_win_count, black_median_score, black_average_score);
                        }
                        ZF::print_out_thor_matches(thor_max_games);
                    }
                    ZF::display_board_after_thor(side_to_move, use_timer,
                                                 &board, &black_moves, &white_moves);
                }
                Dump::dump_position(side_to_move, &board);
                Dump::dump_game_score(side_to_move, score_sheet_row, &black_moves, &white_moves);
                /* Check what the Thor opening statistics has to say */
                Thor::choose_thor_opening_move(&board, side_to_move, echo);
                if echo != 0 && wait != 0 { ZF::dumpch(); }
                if disks_played >= provided_move_count {
                    if skill[side_to_move as usize] == 0 as i32 {
                        if use_book != 0 && display_pv != 0 {
                            fill_move_alternatives::<FE>(side_to_move,
                                                   0 as i32);
                            if echo != 0 {
                                ZF::print_move_alternatives(side_to_move);
                            }
                        }
                        ZF::before_get_move();
                        curr_move = get_move_async(side_to_move, &mut get_move_cb).await?;
                    } else {
                        start_move::<FE>(player_time[side_to_move as usize],
                                   player_increment[side_to_move as usize],
                                   disks_played + 4);
                        determine_move_time(player_time[side_to_move as usize],
                                            player_increment[side_to_move as usize],
                                            disks_played + 4);
                        let timed_search = (skill[side_to_move as usize] >= 60) as i32;
                        curr_move =
                            generic_compute_move::<ComputeMoveLog, ComputeMoveOut, FE, Thor>(
                                side_to_move, 1,
                                player_time[side_to_move as usize] as i32,
                                player_increment[side_to_move as usize] as i32, timed_search,
                                use_book,
                                skill[side_to_move as usize],
                                exact_skill[side_to_move as usize],
                                wld_skill[side_to_move as usize],
                                0 as i32, &mut eval_info,
                                &mut ComputeMoveLog::create_log_file_if_needed());
                        if side_to_move == 0 as i32 {
                            ZF::set_evals(produce_compact_eval(eval_info), 0.0f64);
                        } else {
                            ZF::set_evals(0.0f64, produce_compact_eval(eval_info));
                        }
                        if eval_info.is_book != 0 &&
                            rand_move_freq > 0 &&
                            side_to_move == rand_color &&
                            my_random() % rand_move_freq as i64 == 0 {
                            ZF::report_engine_override();
                            rand_color = 2 - rand_color;
                            curr_move = move_list[disks_played as usize]
                                [(my_random() % move_count[disks_played as usize] as i64) as usize]
                        }
                    }
                } else {
                    curr_move = provided_move[disks_played as usize];
                    if valid_move(curr_move, side_to_move) == 0 {
                        FE::invalid_move_in_move_sequence(curr_move);
                    }
                }
                let move_stop = get_real_timer::<FE>();
                if player_time[side_to_move as usize] != 10000000.0f64 {
                    player_time[side_to_move as usize] -= move_stop - move_start
                }
                store_move(disks_played, curr_move);
                ZF::push_move(&mut move_vec, curr_move, disks_played);
                make_move(side_to_move, curr_move, 1);
                if side_to_move == 0 as i32 {
                    black_moves[score_sheet_row as usize] = curr_move
                } else {
                    if white_moves[score_sheet_row as usize] != -(1) {
                        score_sheet_row += 1
                    }
                    white_moves[score_sheet_row as usize] = curr_move
                }
            } else {
                if side_to_move == 0 {
                    black_moves[score_sheet_row as usize] = -(1)
                } else {
                    white_moves[score_sheet_row as usize] = -(1)
                }
                if skill[side_to_move as usize] == 0 {
                    get_move_cb(-1).await;
                }
            }
            side_to_move = 2 - side_to_move;
            if one_position_only != 0 { break; }
        }
        if echo == 0 && one_position_only == 0 {
            let black_level = skill[0];
            let white_level = skill[2];
            ZF::report_skill_levels(black_level, white_level);
        }
        if side_to_move == 0 as i32 { score_sheet_row += 1 }
        Dump::dump_game_score(side_to_move, score_sheet_row, &black_moves, &white_moves);
        if echo != 0 && one_position_only == 0 {
            ZF::set_move_list(black_moves.as_mut_ptr(), white_moves.as_mut_ptr(),
                          score_sheet_row);
            if use_thor_ {
                let database_start = get_real_timer::<FE>();
                Thor::database_search(&board, side_to_move);
                thor_position_count = Thor::get_match_count();
                let database_stop = get_real_timer::<FE>();
                let db_search_time = database_stop - database_start;
                total_search_time += db_search_time;
                ZF::report_some_thor_stats(total_search_time, thor_position_count, db_search_time);
                if thor_position_count > 0 {
                    let black_win_count = Thor::get_black_win_count();
                    let draw_count = Thor::get_draw_count();
                    let white_win_count = Thor::get_white_win_count();
                    let black_median_score = Thor::get_black_median_score();
                    let black_average_score = Thor::get_black_average_score();
                    ZF::report_some_thor_scores(black_win_count, draw_count, white_win_count, black_median_score, black_average_score);
                }
                ZF::print_out_thor_matches(thor_max_games);
            }
            ZF::set_times(floor(player_time[0]) as _, floor(player_time[2]) as _);
            ZF::display_board_after_thor(side_to_move, use_timer, &board,
                                         &black_moves,
                                         &white_moves,
            );
        }
        adjust_counter(&mut total_nodes);
        let node_val = counter_value(&mut total_nodes);
        adjust_counter(&mut total_evaluations);
        let eval_val = counter_value(&mut total_evaluations);
        let black_disc_count = disc_count(0, &board);
        let white_disc_count = disc_count(2, &board);
        let total_time_ = total_time;
        ZF::report_after_game_ended(node_val, eval_val, black_disc_count, white_disc_count, total_time_);

        if !log_file_name_.is_null() && one_position_only == 0 {
            ZF::log_game_ending(log_file_name_,
                                &mut move_vec,
                                disc_count(0, &board),
                                disc_count(2, &board))
        }
        repeat -= 1;
        toggle_abort_check(0 as i32);
        if use_learning_ && one_position_only == 0 {
            Learn::learn_game(disks_played,
                              (skill[0] != 0 && skill[2] != 0) as i32,
                              (repeat == 0 as i32) as i32);
        }
        toggle_abort_check(1);
        if !(repeat > 0) { break; }
    }
    Ok(())
}

unsafe fn clear_moves() {
    black_moves = [-1; 60];
    white_moves = [-1; 60];
}
