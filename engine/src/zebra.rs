use std::error::Error;
use std::ffi::{CStr, CString};
use std::future::Future;

use engine_traits::Offset;
use flip::unflip::FlipStack;

use crate::src::counter::{adjust_counter, counter_value};
use crate::src::end::End;
use crate::src::error::FrontEnd;
use crate::src::game::{ComputeMoveLogger, ComputeMoveOutput, FileBoardSource, GameState, generic_compute_move, generic_game_init};
use crate::src::getcoeff::{CoeffState, remove_coeffs};
use crate::src::globals::BoardState;
use crate::src::hash::{HashState, setup_hash};
use crate::src::learn::{LearnState};
use crate::src::midgame::MidgameState;
use crate::src::moves::{game_in_progress, generate_all, make_move, MovesState, valid_move};
use crate::src::myrandom::MyRandom;
use crate::src::osfbook::{Book, fill_move_alternatives, find_opening_name, reset_book_search, set_deviation_value};
use crate::src::probcut::ProbCut;
use crate::src::search::{disc_count, produce_compact_eval, SearchState};
use crate::src::stable::StableState;
use crate::src::stubs::floor;
use crate::src::thordb::ThorDatabase;
use crate::src::timer::{Timer, TimeSource};
use crate::src::zebra::EvalResult::WON_POSITION;
use crate::src::zebra::EvalType::MIDGAME_EVAL;
use crate::src::zebra::MoveStringError::{InvalidMoveString, UnexpectedCharacter};
use std::thread::park_timeout;

#[derive(Copy, Clone, PartialEq)]
pub enum EvalType {
    UNINITIALIZED_EVAL = 8,
    INTERRUPTED_EVAL = 7,
    UNDEFINED_EVAL = 6,
    PASS_EVAL = 5,
    FORCED_EVAL = 4,
    SELECTIVE_EVAL = 3,
    WLD_EVAL = 2,
    EXACT_EVAL = 1,
    MIDGAME_EVAL = 0,
}

#[derive(Copy, Clone, PartialEq)]
pub enum EvalResult {
    UNSOLVED_POSITION = 3,
    LOST_POSITION = 2,
    DRAWN_POSITION = 1,
    WON_POSITION = 0,
}

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

impl EvaluationType {
    pub const fn new() -> Self {
        EvaluationType { type_0: MIDGAME_EVAL, res: WON_POSITION, score: 0, confidence: 0., search_depth: 0, is_book: 0 }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum DrawMode {
    OPPONENT_WINS = 3,
    WHITE_WINS = 2,
    BLACK_WINS = 1,
    NEUTRAL = 0,
}

#[derive(Copy, Clone, PartialEq)]
pub enum GameMode {
    PUBLIC_GAME = 1,
    PRIVATE_GAME = 0,
}

/* Local variables */
pub struct Config {
    pub slack: f64,
    pub dev_bonus: f64,
    pub low_thresh: i32,
    pub high_thresh: i32,
    pub rand_move_freq: i32,
    pub tournament: i32,
    pub tournament_levels: i32,
    pub deviation_depth: i32,
    pub cutoff_empty: i32,
    pub one_position_only: i32,
    pub use_timer: i32,
    pub only_analyze: i32,
    pub thor_max_games: i32,
    pub tournament_skill: [[i32; 3]; 8],
    pub wld_skill: [i32; 3],
    pub exact_skill: [i32; 3],
    pub player_time: [f64; 3],
    pub player_increment: [f64; 3],
    pub skill: [i32; 3],
    pub wait: i32,
    pub use_book: i32,
    pub wld_only: i32,
    pub echo: i32,
    pub display_pv: i32,
    pub use_thor: bool,
    pub use_learning: bool,
}

pub const INITIAL_CONFIG: Config = Config {
    slack: 0.25f64,
    dev_bonus: 0.0f64,
    low_thresh: 0,
    high_thresh: 0,
    rand_move_freq: 0,
    tournament: 0,
    tournament_levels: 0,
    deviation_depth: 0,
    cutoff_empty: 0,
    one_position_only: 0,
    use_timer: 0,
    only_analyze: 0,
    thor_max_games: 0,
    tournament_skill: [[0; 3]; 8],
    wld_skill: [0; 3],
    exact_skill: [0; 3],
    player_time: [0.; 3],
    player_increment: [0.; 3],
    skill: [0; 3],
    wait: 0,
    use_book: 1,
    wld_only: 0,
    echo: 0,
    display_pv: 0,
    use_thor: false,
    use_learning: false,
};


pub fn set_default_engine_globals(config: &mut Config) {
    config.wait = 0;
    config.echo = 1;
    config.display_pv = 1;
    config.skill[2] = -1;
    config.skill[0] = -1;
    config.player_time[2] = 10000000.0f64;
    config.player_time[0] = 10000000.0f64;
    config.player_increment[2] = 0.0f64;
    config.player_increment[0] = 0.0f64;
}

pub trait InitialMoveSource {
    fn fill_line_buffer(&mut self, line_buffer: &mut [u8]);
}


pub trait ZebraFrontend {
    fn set_evals(black: f64, white: f64);
    fn set_move_list(row: i32);
    fn set_names(white_is_player: bool, black_is_player: bool);
    fn report_engine_override();
    fn before_get_move();
    fn report_book_randomness(slack_: f64);
    fn load_thor_files(g_timer: &mut Timer);
    fn print_move_alternatives(side_to_move: i32, board_state: &mut BoardState, g_book: &mut Book);
}

#[derive(Copy, Clone)]
pub enum PlayGameState {
    Initial,
    InGame { provided_move_count: i32 },
    AfterGame,
    AfterGameReport { node_val: f64, eval_val: f64 },
    End,
    CanLearn,
    SwitchingSides { provided_move_count: i32 },
    GetPass { provided_move_count: i32 },
    MoveStop { provided_move_count: i32, move_start: f64 },
    StartGetMove { provided_move_count: i32, move_start: f64 },
    GettingMove { provided_move_count: i32, move_start: f64, side_to_move: i32 },
    AfterDumpch { provided_move_count: i32, move_start: f64 },
    Dumpch { provided_move_count: i32, move_start: f64 },
    NeedsDump { provided_move_count: i32, move_start: f64 },
}

pub struct PlayGame<Source: InitialMoveSource> {
    file_name: Option<CString>,
    move_string: Vec<u8>,
    pub repeat: i32,
    move_file: Option<Source>,
    pub g_state: FullState,
    eval_info: EvaluationType,
    pub side_to_move: i32,
    curr_move: i8,
    rand_color: i32,
    provided_move: [i8; 61],
    pub move_vec: [i8; 122],
    line_buffer: [u8; 1001],
    pub state: PlayGameState,
}

pub struct MoveAttempt(pub i8, pub i8);

pub fn next_state<
    ZF: ZebraFrontend,
    Source: InitialMoveSource,
    BoardSrc: FileBoardSource,
    ComputeMoveLog: ComputeMoveLogger,
    ComputeMoveOut: ComputeMoveOutput,
    FE: FrontEnd,
    Thor: ThorDatabase
>(play_state: &mut PlayGame<Source>, move_attempt: Option<MoveAttempt>) -> PlayGameState {
    play_state.state = match play_state.state {
        PlayGameState::Initial => {
            /* Decode the predefined move sequence */
            let provided_move_count = parse_provided_moves(
                &mut play_state.provided_move,
                load_moves_from_source(&mut play_state.move_file, &mut play_state.line_buffer)
                    .unwrap_or(&mut play_state.move_string));

            let provided_move_count = match provided_move_count {
                Ok(c) => c,
                Err(e) => match e {
                    InvalidMoveString => FE::invalid_move_string_provided(),
                    UnexpectedCharacter => FE::unexpected_character_in_a_move_string(),
                },
            };
            /* Set up the position and the search engine */
            generic_game_init::<BoardSrc, FE>(play_state.file_name.as_ref().map(CString::as_ref), &mut play_state.side_to_move, &mut play_state.g_state);
            setup_hash(1, &mut play_state.g_state.hash_state, &mut play_state.g_state.random_instance);
            play_state.g_state.learn_state.clear_stored_game();
            if play_state.g_state.g_config.echo != 0 && play_state.g_state.g_config.use_book != 0 {
                let slack_ = play_state.g_state.g_config.slack;
                ZF::report_book_randomness(slack_);
            }
            play_state.g_state.g_book.set_slack(floor(play_state.g_state.g_config.slack * 128.0f64) as i32);
            play_state.g_state.game_state.toggle_human_openings(0);
            if play_state.g_state.g_config.use_learning {
                play_state.g_state.learn_state.set_learning_parameters(play_state.g_state.g_config.deviation_depth, play_state.g_state.g_config.cutoff_empty);
            }
            reset_book_search(&mut play_state.g_state.g_book);
            set_deviation_value(play_state.g_state.g_config.low_thresh, play_state.g_state.g_config.high_thresh, play_state.g_state.g_config.dev_bonus, &mut play_state.g_state.g_book);
            if play_state.g_state.g_config.use_thor {
                ZF::load_thor_files(&mut play_state.g_state.g_timer);
            }
            let white_is_player = play_state.g_state.g_config.skill[0] == 0;
            let black_is_player = play_state.g_state.g_config.skill[2] == 0;
            ZF::set_names(white_is_player, black_is_player);
            ZF::set_move_list(play_state.g_state.board_state.score_sheet_row);
            ZF::set_evals(0.0f64, 0.0f64);
            clear_moves(&mut play_state.g_state.board_state);
            play_state.move_vec[0] = 0;
            // these are not used because their usage was disabled by preprocessor
            // but for deterministic testing, we need to call random the same way, so we keep them.
            let _black_hash1 = play_state.g_state.random_instance.my_random();
            let _black_hash2 = play_state.g_state.random_instance.my_random();
            let _white_hash1 = play_state.g_state.random_instance.my_random();
            let _white_hash2 = play_state.g_state.random_instance.my_random();
            PlayGameState::InGame { provided_move_count }
        }
        PlayGameState::InGame { provided_move_count } => {
            if game_in_progress(&mut play_state.g_state.moves_state, &play_state.g_state.search_state, &play_state.g_state.board_state.board) != 0 {
                remove_coeffs(play_state.g_state.moves_state.disks_played, &mut play_state.g_state.coeff_state);
                generate_all(play_state.side_to_move, &mut play_state.g_state.moves_state, &play_state.g_state.search_state, &play_state.g_state.board_state.board);
                if play_state.side_to_move == 0 {
                    play_state.g_state.board_state.score_sheet_row += 1
                }
                if play_state.g_state.moves_state.move_count[play_state.g_state.moves_state.disks_played as usize] != 0 {
                    let move_start = play_state.g_state.g_timer.get_real_timer();
                    play_state.g_state.g_timer.clear_panic_abort();
                    PlayGameState::NeedsDump { provided_move_count, move_start }
                } else {
                    if play_state.side_to_move == 0 {
                        play_state.g_state.board_state.black_moves[play_state.g_state.board_state.score_sheet_row as usize] = -(1)
                    } else {
                        play_state.g_state.board_state.white_moves[play_state.g_state.board_state.score_sheet_row as usize] = -(1)
                    }
                    if play_state.g_state.g_config.skill[play_state.side_to_move as usize] == 0 {
                        PlayGameState::GetPass { provided_move_count }
                    } else {
                        PlayGameState::SwitchingSides { provided_move_count }
                    }
                }
            } else {
                if play_state.side_to_move == 0 { play_state.g_state.board_state.score_sheet_row += 1 }
                PlayGameState::AfterGame
            }
        }
        PlayGameState::NeedsDump { provided_move_count, move_start } => {
            if play_state.g_state.g_config.echo != 0 && play_state.g_state.g_config.wait != 0 {
                PlayGameState::Dumpch { provided_move_count, move_start }
            } else {
                PlayGameState::AfterDumpch { provided_move_count, move_start }
            }
        }
        PlayGameState::AfterGameReport { node_val, eval_val } => {
            play_state.repeat -= 1;
            PlayGameState::CanLearn
        }
        PlayGameState::AfterGame => {
            let node_val = counter_value(&mut play_state.g_state.search_state.total_nodes);
            let eval_val = counter_value(&mut play_state.g_state.search_state.total_evaluations);
            PlayGameState::AfterGameReport { node_val, eval_val }
        }
        PlayGameState::CanLearn => {
            if !(play_state.repeat > 0) {
                PlayGameState::End
            } else {
                PlayGameState::Initial
            }
        }
        PlayGameState::End => {
            PlayGameState::End
        }
        PlayGameState::SwitchingSides { provided_move_count } => {
            play_state.side_to_move = 2 - play_state.side_to_move;
            if play_state.g_state.g_config.one_position_only != 0 {
                if play_state.side_to_move == 0 { play_state.g_state.board_state.score_sheet_row += 1 }
                PlayGameState::AfterGame
            } else {
                PlayGameState::InGame { provided_move_count }
            }
        }
        PlayGameState::MoveStop { provided_move_count, move_start } => {
            let move_stop = play_state.g_state.g_timer.get_real_timer();
            if play_state.g_state.g_config.player_time[play_state.side_to_move as usize] != 10000000.0f64 {
                // panic!("this branch is not tested"); I don't know how to trigger this in tests

                play_state.g_state.g_config.player_time[play_state.side_to_move as usize] -= move_stop - move_start
            }
            play_state.g_state.learn_state.store_move(play_state.g_state.moves_state.disks_played, play_state.curr_move);
            push_move(&mut play_state.move_vec, play_state.curr_move, play_state.g_state.moves_state.disks_played);
            make_move(play_state.side_to_move, play_state.curr_move, 1, &mut play_state.g_state.moves_state, &mut play_state.g_state.board_state, &mut play_state.g_state.hash_state, &mut play_state.g_state.flip_stack_);
            if play_state.side_to_move == 0 {
                play_state.g_state.board_state.black_moves[play_state.g_state.board_state.score_sheet_row as usize] = play_state.curr_move
            } else {
                if play_state.g_state.board_state.white_moves[play_state.g_state.board_state.score_sheet_row as usize] != -(1) {
                    // panic!("this branch is not tested"); to trigger this in tests

                    play_state.g_state.board_state.score_sheet_row += 1
                }
                play_state.g_state.board_state.white_moves[play_state.g_state.board_state.score_sheet_row as usize] = play_state.curr_move
            }
            PlayGameState::SwitchingSides { provided_move_count }
        }
        PlayGameState::AfterDumpch { provided_move_count, move_start } => {
            if play_state.g_state.moves_state.disks_played >= provided_move_count {
                if play_state.g_state.g_config.skill[play_state.side_to_move as usize] == 0 {
                    if play_state.g_state.g_config.use_book != 0 && play_state.g_state.g_config.display_pv != 0 {
                        fill_move_alternatives::<FE>(play_state.side_to_move,
                                                     0,
                                                     &mut play_state.g_state.g_book,
                                                     &mut play_state.g_state.board_state,
                                                     &mut play_state.g_state.moves_state,
                                                     &play_state.g_state.search_state,
                                                     &mut play_state.g_state.flip_stack_,
                                                     &mut play_state.g_state.hash_state);
                        if play_state.g_state.g_config.echo != 0 {
                            ZF::print_move_alternatives(play_state.side_to_move, &mut play_state.g_state.board_state, &mut play_state.g_state.g_book);
                        }
                    }
                    PlayGameState::StartGetMove { provided_move_count, move_start }
                } else {
                    play_state.g_state.g_timer.start_move(play_state.g_state.g_config.player_time[play_state.side_to_move as usize],
                                                          play_state.g_state.g_config.player_increment[play_state.side_to_move as usize],
                                                          play_state.g_state.moves_state.disks_played + 4);
                    play_state.g_state.g_timer.determine_move_time(play_state.g_state.g_config.player_time[play_state.side_to_move as usize],
                                                                   play_state.g_state.g_config.player_increment[play_state.side_to_move as usize],
                                                                   play_state.g_state.moves_state.disks_played + 4);
                    let timed_search = (play_state.g_state.g_config.skill[play_state.side_to_move as usize] >= 60) as i32;
                    play_state.curr_move =
                        generic_compute_move::<ComputeMoveLog, ComputeMoveOut, FE, Thor>(
                            play_state.side_to_move, 1,
                            play_state.g_state.g_config.player_time[play_state.side_to_move as usize] as i32,
                            play_state.g_state.g_config.player_increment[play_state.side_to_move as usize] as i32, timed_search,
                            play_state.g_state.g_config.use_book,
                            play_state.g_state.g_config.skill[play_state.side_to_move as usize],
                            play_state.g_state.g_config.exact_skill[play_state.side_to_move as usize],
                            play_state.g_state.g_config.wld_skill[play_state.side_to_move as usize],
                            0, &mut play_state.eval_info,
                            &mut ComputeMoveLog::create_log_file_if_needed(),
                            play_state.g_state.g_config.display_pv,
                            play_state.g_state.g_config.echo,
                            &mut play_state.g_state);
                    if play_state.side_to_move == 0 {
                        ZF::set_evals(produce_compact_eval(play_state.eval_info), 0.0f64);
                    } else {
                        ZF::set_evals(0.0f64, produce_compact_eval(play_state.eval_info));
                    }
                    if play_state.eval_info.is_book != 0 &&
                        play_state.g_state.g_config.rand_move_freq > 0 &&
                        play_state.side_to_move == play_state.rand_color &&
                        play_state.g_state.random_instance.my_random() % play_state.g_state.g_config.rand_move_freq as i64 == 0 {
                        ZF::report_engine_override();
                        play_state.rand_color = 2 - play_state.rand_color;
                        play_state.curr_move = play_state.g_state.moves_state.move_list[play_state.g_state.moves_state.disks_played as usize]
                            [(play_state.g_state.random_instance.my_random() % play_state.g_state.moves_state.move_count[play_state.g_state.moves_state.disks_played as usize] as i64) as usize]
                    }
                    PlayGameState::MoveStop { provided_move_count, move_start }
                }
            } else {
                play_state.curr_move = play_state.provided_move[play_state.g_state.moves_state.disks_played as usize];
                if valid_move(play_state.curr_move, play_state.side_to_move, &play_state.g_state.board_state.board) == 0 {
                    FE::invalid_move_in_move_sequence(play_state.curr_move);
                }
                PlayGameState::MoveStop { provided_move_count, move_start }
            }
        }
        PlayGameState::Dumpch { provided_move_count, move_start } => {
            PlayGameState::AfterDumpch { provided_move_count, move_start }
        }
        PlayGameState::GetPass { provided_move_count } => {
            PlayGameState::SwitchingSides { provided_move_count }
        }
        PlayGameState::StartGetMove { provided_move_count, move_start } => {
            ZF::before_get_move();
            let side_to_move: i32 = play_state.side_to_move;
            PlayGameState::GettingMove { provided_move_count, move_start, side_to_move }
        }
        PlayGameState::GettingMove { provided_move_count, move_start, side_to_move } => {
            if let Some(MoveAttempt(curr_move, curr_move_2)) = move_attempt {
                let board = &play_state.g_state.board_state.board;
                let ready = valid_move(curr_move, side_to_move, board);
                if ready == 0 {
                    let ready = valid_move(curr_move_2, side_to_move, board);
                    if ready != 0 {
                        play_state.curr_move = curr_move_2;
                        PlayGameState::MoveStop { provided_move_count, move_start }
                    } else {
                        PlayGameState::GettingMove { provided_move_count, move_start, side_to_move }
                    }
                } else {
                    play_state.curr_move = curr_move;
                    PlayGameState::MoveStop { provided_move_count, move_start }
                }
            } else {
                PlayGameState::GettingMove { provided_move_count, move_start, side_to_move }
            }
        }
    };
    return play_state.state;
}

impl<Src: InitialMoveSource> PlayGame<Src> {
    #[inline(always)]
    pub fn new(file_name: Option<CString>, move_string: Vec<u8>,
               mut repeat: i32,
               mut move_file: Option<Src>,
               g_state: FullState,
    ) -> PlayGame<Src> {
        let mut eval_info = EvaluationType::new();
        let mut side_to_move = 0;
        let mut curr_move = 0;
        let mut rand_color = 0;
        let mut provided_move = [0; 61];
        let mut move_vec = [0; 122];
        let mut line_buffer = [0u8; 1001];
        let mut state = PlayGameState::Initial;
        let mut play_state = PlayGame {
            file_name,
            move_string,
            repeat,
            move_file,
            g_state,
            eval_info,
            side_to_move,
            curr_move,
            rand_color,
            provided_move,
            move_vec,
            line_buffer,
            state,
        };
        play_state
    }
}

fn load_moves_from_source<'a, Source: InitialMoveSource>(mut move_file: &mut Option<Source>, line_buffer: &'a mut [u8; 1001]) -> Option<&'a [u8]> {
    if let Some(ref mut move_file) = &mut move_file {
        {
            // this is kindof a hack just to preserve null teminator at the absolute end of this string
            // we slice the buffer at the end and pass along just smaller slice to the trait function
            let end = line_buffer.len() - 2;
            let mut line_buffer: &mut [u8] = &mut line_buffer[0..end];
            move_file.fill_line_buffer(&mut line_buffer);
        }
        let end = line_buffer.iter().enumerate().find(|(i, ch)| ch == &&0)
            .unwrap().0;
        Some(&line_buffer[0..end])
    } else { None }
}

enum MoveStringError {
    InvalidMoveString,
    UnexpectedCharacter,
}

fn parse_provided_moves(provided_move: &mut [i8; 61], move_string: &[u8]) -> Result<i32, MoveStringError> {
    let provided_move_count = move_string.len().wrapping_div(2) as i32;
    if provided_move_count > 60 ||
        move_string.len().wrapping_rem(2) == 1 {
        return Err(InvalidMoveString);
    }
    let mut i = 0;
    while i < provided_move_count {
        let col = (*move_string.offset((2 * i) as _) as char).to_ascii_lowercase() as u8 - b'a' + 1;
        let row = *move_string.offset((2 * i + 1) as _) as u8 - b'0';
        if col < 1 || col > 8 || row < 1 || row > 8 {
            return Err(UnexpectedCharacter);
        }
        provided_move[i as usize] = (10 * row + col) as i8;
        i += 1
    }
    Ok(provided_move_count)
}

fn push_move(move_vec: &mut [i8; 122], curr_move: i8, disks_played_: i32) {
    move_vec[(2 * disks_played_) as usize] = 'a' as i8 + (curr_move % 10) - 1;
    move_vec[(2 * disks_played_) as usize + 1] = '0' as i8 + (curr_move / 10);
}

fn clear_moves(state: &mut BoardState) {
    state.black_moves = [-1; 60];
    state.white_moves = [-1; 60];
}

pub struct FullState {
    pub g_config: Config,
    pub learn_state: LearnState,
    pub midgame_state: MidgameState,
    pub game_state: GameState,
    pub end_g: End,
    pub coeff_state: CoeffState,
    pub g_timer: Timer,
    pub moves_state: MovesState,
    pub stable_state: StableState,
    pub board_state: BoardState,
    pub hash_state: HashState,
    pub random_instance: MyRandom,
    pub g_book: Book,
    pub prob_cut: ProbCut,
    pub search_state: SearchState,
    pub flip_stack_: FlipStack,
}

impl FullState {
    #[inline(always)]
    pub fn new(time_source: &'static dyn TimeSource) -> Self {
        FullState {
            g_config: INITIAL_CONFIG,
            learn_state: LearnState::new(),
            midgame_state: MidgameState::new(),
            game_state: GameState::new(),
            end_g: End::new(),
            coeff_state: CoeffState::new(),
            g_timer: Timer::new(time_source),
            moves_state: MovesState::new(),
            stable_state: StableState::new(),
            board_state: BoardState::new(),
            hash_state: HashState::new(),
            random_instance: MyRandom::new(),
            g_book: Book::new(),
            prob_cut: ProbCut::new(),
            search_state: SearchState::new(),
            flip_stack_: FlipStack::new(),
        }
    }
}
