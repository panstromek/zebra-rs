use std::ffi::{CStr, CString};

use engine_traits::{CoeffSource, Offset};

use crate::src::counter::{add_counter, adjust_counter, counter_value, reset_counter};
use crate::src::end::{End, end_game, setup_end};
use crate::src::error::FrontEnd;
use crate::src::getcoeff::{CoeffAdjustments, eval_adjustment, init_coeffs_calculate_terminal_patterns, post_init_coeffs, process_coeffs_from_fn_source, remove_coeffs, CoeffState, pattern_evaluation};
use crate::src::globals::{Board, BoardState};
use crate::src::hash::{determine_hash_values, find_hash, HashEntry, HashState};
use crate::src::midgame::{calculate_perturbation, middle_game, MidgameState, setup_midgame};
use crate::src::moves::{generate_all, make_move, valid_move, MovesState, unmake_move};
use crate::src::osfbook::{check_forced_opening, fill_move_alternatives, get_book_move, Book};
use crate::src::probcut::{init_probcut, ProbCut};
use crate::src::search::{complete_pv, create_eval_info, disc_count, float_move, force_return, setup_search, sort_moves, SearchState};
use crate::src::stable::{init_stable, StableState};
use crate::src::stubs::abs;
use crate::src::thordb::ThorDatabase;
use crate::src::timer::{time_t, Timer};

use crate::src::zebra::EvalResult::{UNSOLVED_POSITION, WON_POSITION, LOST_POSITION, DRAWN_POSITION};
use crate::src::zebra::EvalType::{EXACT_EVAL, FORCED_EVAL, INTERRUPTED_EVAL, MIDGAME_EVAL, PASS_EVAL, UNDEFINED_EVAL, WLD_EVAL};
use crate::src::zebra::{EvaluationType, FullState};
use flip::unflip::FlipStack;
use crate::src::myrandom::MyRandom;
use crate::src::game::BoardSourceError::UnrecognizedCharacter;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EvaluatedMove {
    pub eval: EvaluationType,
    pub side_to_move: i32,
    pub move_0: i8,
    pub pv_depth: i32,
    pub pv: [i8; 60],
}

impl EvaluatedMove {
    pub fn new() -> Self {
        Self {
            eval: EvaluationType::new(),
            side_to_move: 0,
            move_0: 0,
            pv_depth: 0,
            pv: [0; 60],
        }
    }
}

pub const BOOK_MOVE: C2RustUnnamed = 1;
pub type C2RustUnnamed = u32;
pub const ENDGAME_MOVE: C2RustUnnamed = 3;
pub const MIDGAME_MOVE: C2RustUnnamed = 2;
pub const INTERRUPTED_MOVE: C2RustUnnamed = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CandidateMove {
    pub move_0: i8,
    pub score: i32,
    pub flags: i32,
    pub parent_flags: i32,
}

/* The maximum length of any system path. */
/*
  SET_FORCED_OPENING
  Specifies an opening line that Zebra is forced to follow when playing.
*/
pub struct GameState {
    forced_opening: Option<CString>,
    last_time_used: f64,
    pub max_depth_reached: i32,
    pub play_human_openings: i32,
    komi: i32,
    endgame_performed: [i32; 3],
    pub  prefix_move: i8
}

impl GameState {
    pub const fn new() -> Self {
        GameState {
            forced_opening: None,
            last_time_used: 0.,
            max_depth_reached: 0,
            play_human_openings: 1,
            komi: 0,
            endgame_performed: [0; 3],
            prefix_move: 0
        }
    }
}
/*
  TOGGLE_THOR_MATCH_OPENINGS
  Specifies whether matching Thor games are used as opening book
  before resorting to the usual opening book.
*/
static play_thor_match_openings: i32 = 1;

impl GameState {
    pub fn clear_endgame_performed(&mut self) {
        self.endgame_performed[0] = 0;
        self.endgame_performed[2] = 0;
    }
    /*
      SET_KOMI
      Set the endgame komi value.
    */

    pub fn set_komi(&mut self, in_komi: i32) {
        self.komi = in_komi;
    }
    /*
      TOGGLE_HUMAN_OPENINGS
      Specifies whether the Thor statistics should be queried for
      openings moves before resorting to the usual opening book.
    */

    pub fn toggle_human_openings(&mut self, toggle: i32) {
        self.play_human_openings = toggle;
    }
}

pub struct EvaluatedList {
    evaluated_list: [EvaluatedMove; 60],
    game_evaluated_count: i32,
    pub best_move: i8,
}

impl EvaluatedList {
    fn new() -> Self {
        Self {
            evaluated_list: [EvaluatedMove::new(); 60],
            game_evaluated_count: 0,
            best_move: 0,
        }
    }
}

impl EvaluatedList {
    pub fn get_evaluated(&self, index: i32) -> EvaluatedMove {
        return self.evaluated_list[index as usize];
    }

    /*
      GET_EVALUATED_COUNT
      GET_EVALUATED
      Accessor functions for the data structure filled by extended_compute_move().
    */
    pub fn get_evaluated_count(&self) -> i32 {
        return self.game_evaluated_count;
    }
}
/*
  EXTENDED_COMPUTE_MOVE
  This wrapper on top of compute_move() calculates the evaluation
  of all moves available as opposed to upper bounds for all moves
  except for the best.
*/

pub fn extended_compute_move<L: ComputeMoveLogger, Out: ComputeMoveOutput, FE: FrontEnd, Thor: ThorDatabase, StopFn: FnMut() -> bool>(side_to_move: i32,
                                           book_only: i32,
                                           mut book: i32,
                                           mut mid: i32,
                                           mut exact: i32,
                                           mut wld: i32, mut echo: i32, g_state: &mut FullState,
                                           update_cb: fn(&EvaluatedList), mut should_stop: StopFn, thor: &Thor)
                                           -> EvaluatedList {
    let mut list = EvaluatedList::new();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut index: i32 = 0;
    let mut changed: i32 = 0;
    let mut this_move = 0;
    let mut disc_diff: i32 = 0;
    let mut corrected_diff: i32 = 0;
    let mut temp_move = 0;
    let mut best_score: i32 = 0;
    let mut best_pv_depth: i32 = 0;
    let mut stored_echo: i32 = 0;
    let mut shallow_eval: i32 = 0;
    let mut empties: i32 = 0;
    let mut current_mid: i32 = 0;
    let mut current_exact: i32 = 0;
    let mut current_wld: i32 = 0;
    let mut first_iteration: i32 = 0;
    let mut unsearched: i32 = 0;
    let mut unsearched_count: i32 = 0;
    let mut unsearched_move: [i8; 61] = [0; 61];
    let mut best_pv: [i8; 60] = [0; 60];
    let mut transform1: [u32; 60] = [0; 60];
    let mut transform2: [u32; 60] = [0; 60];
    let mut book_move =
        CandidateMove{move_0: 0, score: 0, flags: 0, parent_flags: 0,};
    let mut temp = EvaluatedMove::new();
    let mut book_eval_info =EvaluationType::new();
    let mut res = WON_POSITION;
    /* Disable all time control mechanisms and randomization */
    (g_state.timer).toggle_abort_check(0);
    (g_state.midgame).toggle_midgame_abort_check(0);
    (g_state.midgame).toggle_perturbation_usage(0);
    (g_state.timer).start_move(0 as f64,
                               0 as f64,
                               disc_count(0, &(g_state.board).board) + disc_count(2, &(g_state.board).board));
    (g_state.timer).clear_ponder_times();
    determine_hash_values(side_to_move, &(g_state.board).board, (&mut g_state.hash));
    empties = 60 - (g_state.moves).disks_played;
    list.best_move = 0;
    list.game_evaluated_count = 0;
    reset_counter(&mut (g_state.search).nodes);
    generate_all(side_to_move, (&mut g_state.moves), &(g_state.search), &(g_state.board).board);
    if book_only != 0 || book != 0 {
        /* Evaluations for database moves */
        let mut flags = 0;
        if empties <= exact {
            flags = 16
        } else if empties <= wld { flags = 4 }
        fill_move_alternatives::<FE>(side_to_move, flags,
                                     (&mut g_state.g_book),
                                     &mut (g_state.board),
                                     &mut (g_state.moves),
                                     &mut (g_state.search),
                                     &mut (g_state.flip_stack),
                                     &mut (g_state.hash));
        list.game_evaluated_count = (g_state.g_book).get_candidate_count();
        i = 0;
        while i < list.game_evaluated_count {
            let mut child_flags: i32 = 0;
            book_move = (g_state.g_book).get_candidate(i);
            list.evaluated_list[i as usize].side_to_move = side_to_move;
            list.evaluated_list[i as usize].move_0 = book_move.move_0;
            list.evaluated_list[i as usize].pv_depth = 1;
            list.evaluated_list[i as usize].pv[0] = book_move.move_0;
            list.evaluated_list[i as usize].eval =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 book_move.score, 0.0f64, 0,
                                 1);
            child_flags = book_move.flags & book_move.parent_flags;
            if child_flags & (16 | 4) != 0 {
                if child_flags & 16 != 0 {
                    list.evaluated_list[i as usize].eval.type_0 = EXACT_EVAL
                } else { list.evaluated_list[i as usize].eval.type_0 = WLD_EVAL }
                if book_move.score > 0 {
                    list.evaluated_list[i as usize].eval.res = WON_POSITION;
                    /* Normalize the scores so that e.g. 33-31 becomes +256 */
                    list.evaluated_list[i as usize].eval.score -=
                        30000;
                    list.evaluated_list[i as usize].eval.score *=
                        128
                } else if book_move.score == 0 {
                    list.evaluated_list[i as usize].eval.res = DRAWN_POSITION
                } else {
                    /* score < 0 */
                    list.evaluated_list[i as usize].eval.res = LOST_POSITION;
                    /* Normalize the scores so that e.g. 30-34 becomes -512 */
                    list.evaluated_list[i as usize].eval.score +=
                        30000;
                    list.evaluated_list[i as usize].eval.score *=
                        128
                }
            } else { list.evaluated_list[i as usize].eval.type_0 = MIDGAME_EVAL }
            i += 1;
            update_cb(&list);
            // TODO here calback to UI with update
        }
    }
    if book_only != 0 {
        /* Only book moves are to be considered */
        if list.game_evaluated_count > 0 {
            list.best_move = get_book_move::<FE>(side_to_move, 0,
                                                         &mut book_eval_info, echo,
                                                         &mut (g_state.board),
                                                         &mut (g_state.g_book),
                                                         &(g_state.search),
                                                         &mut (g_state.moves),
                                                         &mut (g_state.hash),
                                                         &mut (g_state.random),
                                                         &mut (g_state.flip_stack));
            let eval = book_eval_info;
            (g_state.search).set_current_eval(eval);
        } else {
            (g_state.board).pv_depth[0] = 0;
            list.best_move = -1;
            book_eval_info = create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION, 0, 0.0f64, 0, 0);
            let eval = book_eval_info;
            (g_state.search).set_current_eval(eval);
        }
    } else {
        /* Make searches for moves not in the database */
        let mut shallow_depth: i32 = 0;
        let empties_0 = 60 - (g_state.moves).disks_played;
        book = 0;
        best_score = -(12345678);
        if list.game_evaluated_count > 0 {
            /* Book PV available */
            best_score = list.evaluated_list[0].eval.score;
            list.best_move = list.evaluated_list[0].move_0
        }
        let negate = 1;
        (g_state.search).negate_current_eval(negate);
        /* Store the available moves, clear their evaluations and sort
           them on shallow evaluation. */
        if empties_0 < 12 {
            shallow_depth = 1
        } else {
            let max_depth = if mid > (if exact > wld { exact } else { wld }) { mid } else if exact > wld { exact } else { wld };
            if max_depth >= 16 {
                shallow_depth = 6
            } else {
                shallow_depth = 4
            }
        }
        unsearched_count = 0;
        i = 0;
        while i < (g_state.moves).move_count[(g_state.moves).disks_played as usize] {
            this_move = (g_state.moves).move_list[(g_state.moves).disks_played as usize][i as usize];
            unsearched = 1;
            j = 0;
            while j < list.game_evaluated_count {
                if list.evaluated_list[j as usize].move_0 == this_move {
                    unsearched = 0
                }
                j += 1
            }
            if !(unsearched == 0) {
                unsearched_move[unsearched_count as usize] = this_move;
                unsearched_count += 1;
                make_move(side_to_move, this_move, 1, (&mut g_state.moves), (&mut g_state.board), (&mut g_state.hash), (&mut g_state.flip_stack));
                let side_to_move_argument = 0 + 2 - side_to_move;
                if shallow_depth == 1 {
                    /* Compute move doesn't allow depth 0 */
                    (g_state.search).evaluations.lo = (g_state.search).evaluations.lo.wrapping_add(1);
                    shallow_eval =
                        -pattern_evaluation(side_to_move_argument, (&mut g_state.board), &(g_state.moves), (&mut g_state.coeff))
                } else {
                    let mut shallow_info =  EvaluationType::new();
                    compute_move::<L, Out, FE, Thor>(0 + 2 - side_to_move, 0,
                                                     0, 0,
                                                     0, book,
                                                     shallow_depth - 1,
                                                     0, 0,
                                                     1, &mut shallow_info, g_state.config.display_pv,
                                                     g_state.config.echo,
                                                     g_state, thor);
                    if shallow_info.type_0 == PASS_EVAL {
                        /* Don't allow pass */
                        compute_move::<L, Out, FE, Thor>(side_to_move, 0,
                                                         0, 0,
                                                         0, book,
                                                         shallow_depth - 1,
                                                         0, 0,
                                                         1, &mut shallow_info, g_state.config.display_pv,
                                                         g_state.config.echo,
                                                         g_state, thor);
                        if shallow_info.type_0 == PASS_EVAL {
                            /* Game over */
                            disc_diff = disc_count(side_to_move, &(g_state.board).board) -
                                    disc_count(0 + 2 - side_to_move, &(g_state.board).board);
                            if disc_diff > 0 {
                                corrected_diff = 64 - 2 * disc_count(0 + 2 - side_to_move, &(g_state.board).board)
                            } else if disc_diff == 0  {
                                corrected_diff = 0
                            } else {
                                corrected_diff = 2 * disc_count(side_to_move, &(g_state.board).board) - 64
                            }
                            shallow_eval = 128 * corrected_diff
                        } else {
                            shallow_eval = shallow_info.score
                        }
                    } else {
                        /* Sign-correct the score produced */
                        shallow_eval = -shallow_info.score
                    }
                }
                unmake_move(side_to_move, this_move, &mut (g_state.board).board, (&mut g_state.moves), (&mut g_state.hash), (&mut g_state.flip_stack));
                (g_state.search).evals[(g_state.moves).disks_played as usize][this_move as usize] = shallow_eval
            }
            i += 1
        }
        loop  {
            changed = 0;
            i = 0;
            while i < unsearched_count - 1 {
                if (g_state.search).evals[(g_state.moves).disks_played as usize][unsearched_move[i as usize] as usize] <
                    (g_state.search).evals[(g_state.moves).disks_played as usize][unsearched_move[(i + 1) as usize] as usize] {
                    temp_move = unsearched_move[i as usize];
                    unsearched_move[i as usize] = unsearched_move[(i + 1) as usize];
                    unsearched_move[(i + 1) as usize] = temp_move;
                    changed = 1
                }
                i += 1
            }
            if !(changed != 0) { break ; }
        }
        /* Initialize the entire list as being empty */
        i = 0;
        index = list.game_evaluated_count;
        while i < unsearched_count {
            list.evaluated_list[index as usize].side_to_move = side_to_move;
            list.evaluated_list[index as usize].move_0 = unsearched_move[i as usize];
            list.evaluated_list[index as usize].eval = create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION, 0, 0.0f64, 0, 0);
            list.evaluated_list[index as usize].pv_depth = 1;
            list.evaluated_list[index as usize].pv[0] = unsearched_move[i as usize];
            if empties_0 > (if wld > exact { wld } else { exact }) {
                transform1[i as usize] = abs((g_state.random).my_random() as i32) as u32;
                transform2[i as usize] = abs((g_state.random).my_random() as i32) as u32
            } else {
                transform1[i as usize] = 0;
                transform2[i as usize] = 0
            }
            i += 1;
            index += 1
        }
        stored_echo = echo;
        echo = 0;
        best_pv_depth = 0;
        if mid == 1 {
            /* compute_move won't be called */
            (g_state.board).pv_depth[0] = 0;
            (g_state.board).piece_count[0][(g_state.moves).disks_played as usize] =
                disc_count(0, &(g_state.board).board);
            (g_state.board).piece_count[2][(g_state.moves).disks_played as usize] =
                disc_count(2, &(g_state.board).board)
        }
        /* Perform iterative deepening if the search depth is large enough */
        if exact > empties_0 { exact = empties_0 }
        if exact < 12 || empties_0 > exact {
            current_exact = exact
        } else {
            current_exact = 8 + exact % 2 - 2
        }
        if wld > empties_0 { wld = empties_0 }
        if wld < 14 || empties_0 > wld {
            current_wld = wld
        } else {
            current_wld = 10 + wld % 2 - 2
        }
        if (empties_0 == exact || empties_0 == wld) &&
            empties_0 > 16 &&
            mid < empties_0 - 12 {
            mid = empties_0 - 12
        }
        if mid < 10 {
            current_mid = mid
        } else {
            current_mid = 6 + mid % 2 - 2
        }
        first_iteration = 1;
        loop  {
            if current_mid < mid {
                current_mid += 2;
                /* Avoid performing deep midgame searches if the endgame
                   is reached anyway. */
                if empties_0 <= wld &&
                    current_mid + 7 >= empties_0 {
                    current_wld = wld;
                    current_mid = mid
                }
                if empties_0 <= exact &&
                    current_mid + 7 >= empties_0 {
                    current_exact = exact;
                    current_mid = mid
                }
            } else if current_wld < wld {
                current_wld = wld
            } else { current_exact = exact }
            i = 0;
            while i < unsearched_count && force_return == 0 && !should_stop() {
                let mut this_eval =  EvaluationType::new();
                this_move = unsearched_move[i as usize];
                /* Locate the current move in the list.  This has to be done
                   because the moves might have been reordered during the
                   iterative deepening. */
                index = 0;
                while list.evaluated_list[index as usize].move_0 != this_move {
                    index += 1
                }
                /* To avoid strange effects when browsing back and forth through
                   a game during the midgame, rehash the hash transformation masks
                   for each move unless the endgame is reached */
                (g_state.hash).set_hash_transformation(transform1[i as usize],
                                                       transform2[i as usize]);
                /* Determine the score for the ith move */
                g_state.game.prefix_move = this_move;
                make_move(side_to_move, this_move, 1, (&mut g_state.moves), (&mut g_state.board), (&mut g_state.hash), (&mut g_state.flip_stack));
                if current_mid == 1 {
                    /* compute_move doesn't like 0-ply searches */
                    (g_state.search).evaluations.lo = (g_state.search).evaluations.lo.wrapping_add(1);
                    let side_to_move_argument = 0 + 2 - side_to_move;
                    shallow_eval = pattern_evaluation(side_to_move_argument, (&mut g_state.board), &(g_state.moves), (&mut g_state.coeff));
                    this_eval = create_eval_info(MIDGAME_EVAL, UNSOLVED_POSITION,
                                         shallow_eval, 0.0f64,
                                         0, 0)
                } else {
                    compute_move::<L, Out, FE, Thor>(0 + 2 -
                                            side_to_move, 0,
                                                     0, 0,
                                                     0, book,
                                                     current_mid - 1,
                                                     current_exact - 1,
                                                     current_wld - 1,
                                                     1, &mut this_eval,
                                                     g_state.config.display_pv,
                                                     g_state.config.echo,
                                                     g_state,thor);
                }
                if force_return != 0 || should_stop() {
                    /* Clear eval and exit search immediately */
                    this_eval = create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                         0, 0.0f64,
                                         0, 0);
                    unmake_move(side_to_move, this_move, &mut (g_state.board).board, &mut (g_state.moves), &mut (g_state.hash), &mut (g_state.flip_stack));
                    break ;
                } else {
                    if this_eval.type_0 == PASS_EVAL {
                        /* Don't allow pass */
                        if current_mid == 1 {
                            /* compute_move doesn't like 0-ply searches */
                            (g_state.search).evaluations.lo = (g_state.search).evaluations.lo.wrapping_add(1);
                            shallow_eval = pattern_evaluation(side_to_move, &mut (g_state.board), &(g_state.moves), &mut (g_state.coeff));
                            this_eval = create_eval_info(MIDGAME_EVAL, UNSOLVED_POSITION, shallow_eval, 0.0, 0, 0)
                        } else {
                            compute_move::<L, Out, FE, Thor>(side_to_move, 0,
                                                             0, 0,
                                                             0, book,
                                                             current_mid - 1,
                                                             current_exact - 1,
                                                             current_wld - 1,
                                                             1, &mut this_eval, g_state.config.display_pv,
                                                             g_state.config.echo,
                                                             g_state, thor);
                        }
                        if this_eval.type_0 == PASS_EVAL {
                            /* Game over */
                            disc_diff = disc_count(side_to_move, &(g_state.board).board) - disc_count(0 + 2 - side_to_move, &(g_state.board).board);
                            if disc_diff > 0  {
                                corrected_diff = 64 - 2 * disc_count(0 + 2 - side_to_move, &(g_state.board).board);
                                res = WON_POSITION
                            } else if disc_diff == 0 {
                                corrected_diff = 0;
                                res = DRAWN_POSITION
                            } else {
                                corrected_diff = 2  * disc_count(side_to_move, &(g_state.board).board) - 64;
                                res = LOST_POSITION
                            }
                            this_eval = create_eval_info(EXACT_EVAL, res, 128 * corrected_diff, 0.0f64, 60 - (g_state.moves).disks_played, 0)
                        }
                    } else {
                        /* Sign-correct the score produced */
                        this_eval.score = -this_eval.score;
                        if this_eval.res == WON_POSITION {
                            this_eval.res = LOST_POSITION
                        } else if this_eval.res == LOST_POSITION {
                            this_eval.res = WON_POSITION
                        }
                    }
                    if force_return != 0 || should_stop() {
                        break;
                    }
                    list.evaluated_list[index as usize].eval = this_eval;
                    /* Store the PV corresponding to the move */
                    list.evaluated_list[index as usize].pv_depth = (g_state.board).pv_depth[0] + 1;
                    list.evaluated_list[index as usize].pv[0] = this_move;
                    j = 0;
                    while j < (g_state.board).pv_depth[0] {
                        list.evaluated_list[index as usize].pv[(j + 1) as usize] = (g_state.board).pv[0][j as usize];
                        j += 1
                    }
                    /* Store the PV corresponding to the best move */
                    if list.evaluated_list[index as usize].eval.score > best_score
                    {
                        best_score = list.evaluated_list[index as usize].eval.score;
                        list.best_move = this_move;
                        best_pv_depth = (g_state.board).pv_depth[0];
                        j = 0;
                        while j < best_pv_depth {
                            best_pv[j as usize] =
                                (g_state.board).pv[0][j as usize];
                            j += 1
                        }
                    }
                    let move_0 = this_move;
                    {
                        unmake_move(side_to_move, move_0, &mut (g_state.board).board, &mut (g_state.moves), &mut (g_state.hash), &mut (g_state.flip_stack));
                    };
                    /* Sort the moves evaluated */
                    if first_iteration != 0 { list.game_evaluated_count += 1 }
                    if force_return == 0 || should_stop() { // TODO here check UI flag
                        loop  {
                            changed = 0;
                            j = 0;
                            while j < list.game_evaluated_count - 1 {
                                if compare_eval(list.evaluated_list[j as usize].eval, list.evaluated_list[(j + 1) as usize].eval) < 0 {
                                    changed = 1;
                                    temp = list.evaluated_list[j as usize];
                                    list.evaluated_list[j as usize] = list.evaluated_list[(j + 1) as usize];
                                    list.evaluated_list[(j + 1) as usize] = temp
                                }
                                j += 1
                            }
                            if !(changed != 0) { break ; }
                        }
                    }
                    update_cb(&list);
                    i += 1
                }
            }
            first_iteration = 0;
            /* Reorder the moves after each iteration.  Each move is moved to
            the front of the list, starting with the bad moves and ending
             with the best move.  This ensures that unsearched_move will be
             sorted w.r.t. the order in evaluated_list. */
            i = list.game_evaluated_count - 1;
            while i >= 0 {
                let this_move_0 = list.evaluated_list[i as usize].move_0;
                j = 0;
                while j != unsearched_count && unsearched_move[j as usize] != this_move_0 {
                    j += 1
                }
                if !(j == unsearched_count) {
                    /* Move the move to the front of the list. */
                    while j >= 1 {
                        unsearched_move[j as usize] = unsearched_move[(j - 1) as usize];
                        j -= 1
                    }
                    unsearched_move[0] = this_move_0
                }
                /* Must be book move, skip */
                i -= 1
            }

            if !(force_return == 0 && (current_mid != mid || current_exact != exact || current_wld != wld) && !should_stop()) {
                break ;
            }
        }
        echo = stored_echo;
        list.game_evaluated_count = (g_state.moves).move_count[(g_state.moves).disks_played as usize];
        /* Make sure that the PV and the score correspond to the best move */
        (g_state.board).pv_depth[0] = best_pv_depth + 1;
        (g_state.board).pv[0][0] = list.best_move;
        i = 0;
        while i < best_pv_depth {
            (g_state.board).pv[0][(i + 1) as usize] = best_pv[i as usize];
            i += 1
        }
        let negate = 0;
        (g_state.search).negate_current_eval(negate);
        if (g_state.moves).move_count[(g_state.moves).disks_played as usize] > 0 {
            let eval_argument = list.evaluated_list[0].eval;
            (g_state.search).set_current_eval(eval_argument);
        }
    }
    /* Reset the hash transformation masks prior to leaving */
    (g_state.hash).set_hash_transformation(0, 0);
    /* Don't forget to enable the time control mechanisms when leaving */
    (g_state.timer).toggle_abort_check(1);
    (g_state.midgame).toggle_midgame_abort_check(1);
    (g_state.midgame).toggle_perturbation_usage(1);
    (g_state.game).max_depth_reached += 1;
    g_state.game.prefix_move = 0;
    return list;
}
/*
  COMPARE_EVAL
  Comparison function for two evals.  Same return value conventions
  as QuickSort.
*/
pub fn compare_eval(mut e1: EvaluationType, mut e2: EvaluationType) -> i32 {
    if e1.type_0 == WLD_EVAL || e1.type_0 == EXACT_EVAL {
        if e1.score > 0 {
            e1.score += 100000
        }
    }
    if e2.type_0 == WLD_EVAL || e2.type_0 == EXACT_EVAL {
        if e2.score > 0 {
            e2.score += 100000
        }
    }
    e1.score - e2.score
}

pub fn engine_game_init(g_state: &mut FullState) {
    setup_search(&mut g_state.search);
    setup_midgame(&mut g_state.midgame, &mut g_state.random);
    setup_end(&mut g_state.flip_stack, &mut g_state.end);
    g_state.timer.clear_ponder_times();
    reset_counter(&mut g_state.search.total_nodes);
    reset_counter(&mut g_state.search.total_evaluations);
    g_state.flip_stack.init_flip_stack();
    g_state.search.total_time = 0.0f64;
    g_state.game.max_depth_reached = 0;
    g_state.game.last_time_used = 0.0f64;
    g_state.game.endgame_performed[2] = 0;
    g_state.game.endgame_performed[0] = g_state.game.endgame_performed[2];
}

pub const fn create_fresh_board() -> Board {
    let mut board_ = [0; 128];
    let mut i = 0;
    while i < 10 {
        let mut j = 0;
        while j < 10 {
            let pos = 10 * i + j;
            if i == 0 || i == 9 || j == 0 || j == 9 {
                board_[pos] = 3
            } else {
                board_[pos] = 1
            }
            j += 1
        }
        i += 1
    }
    board_
}

pub fn setup_game_finalize(side_to_move:  &mut i32,
                                  board_state: &mut BoardState,
                                  hash_state: &mut HashState,
                                  moves_state: &mut MovesState,
) {
    moves_state.disks_played = disc_count(0, &board_state.board) + disc_count(2, &board_state.board) - 4;
    determine_hash_values(*side_to_move, &board_state.board, hash_state);
    /* Make the game score look right */
    if *side_to_move == 0 {
        board_state.score_sheet_row = -(1)
    } else {
        board_state.black_moves[0] = -(1);
        board_state.score_sheet_row = 0
    };
}


pub fn setup_non_file_based_game(side_to_move: &mut i32,
                                        board_state: &mut BoardState,
                                        hash_state: &mut HashState,
                                        moves_state: &mut MovesState,) {
    board_state.board = create_fresh_board();
    board_state.board[54] = 0;
    board_state.board[45] = 0;
    board_state.board[55] = 2;
    board_state.board[44] = 2;
    *side_to_move = 0;
    setup_game_finalize(side_to_move, board_state,hash_state,moves_state);
}


pub fn engine_global_setup<S:CoeffSource, FE: FrontEnd>(
    use_random: i32, hash_bits: i32, coeff_adjustments:
    Option<CoeffAdjustments>, coeffs: S,
    mut search_state: &mut SearchState,
    mut hash_state: &mut HashState,
    mut g_timer: &mut Timer,
    mut coeff_state: &mut CoeffState,
    mut random_instance: &mut MyRandom,
    mut stable_state: &mut StableState,
    mut prob_cut: &mut ProbCut,
) {
    let mut timer: time_t = 0;
    if use_random != 0 {
        g_timer.time(&mut timer);
        random_instance.my_srandom(timer as i32);
    } else { random_instance.my_srandom(1); }
    hash_state.init_hash(hash_bits);

    // inlined init_coeffs
    process_coeffs_from_fn_source::<FE, _>(coeffs, &mut coeff_state);
    init_coeffs_calculate_terminal_patterns(&mut coeff_state);
    if let Some(adjusts) = coeff_adjustments {
        eval_adjustment(adjusts.disc_adjust, adjusts.edge_adjust,
                        adjusts.corner_adjust, adjusts.x_adjust, &mut coeff_state);
    };
    post_init_coeffs(&mut coeff_state);

    g_timer.init_timer();
    init_probcut(&mut prob_cut.mpc_cut, &mut prob_cut.use_end_cut, &mut prob_cut.end_mpc_depth);
    init_stable(&mut stable_state);
    setup_search(&mut search_state);
}

pub trait BoardSource {
    fn fill_board_buffer(&mut self, buffer: &mut String);
    fn fill_buffer_with_side_to_move(&mut self, buffer: &mut Vec<u8>);
    fn report_unrecognized_character(unrecognized: i8);
}

pub enum BoardSourceError {
    UnrecognizedCharacter(u8),
    InitialSquaresAreEmpty
}
pub fn process_board_source<S: BoardSource>(side_to_move: &mut i32, mut file_source: S, board_state_: &mut BoardState) -> Result<(), BoardSourceError> {
    let mut buffer = String::with_capacity(70);
    file_source.fill_board_buffer(&mut buffer);
    let mut token = 0;
    let mut i = 1;
    while i <= 8 {
        let mut j = 1;
        while j <= 8 {
            let pos = 10 * i + j;
            match buffer.as_bytes().get(token as usize) {
                Some(b'*' | b'X') => { board_state_.board[pos as usize] = 0 }
                Some(b'O' | b'0') => { board_state_.board[pos as usize] = 2 }
                Some(b'-' | b'.') => {},
                Some(c) => {
                    let unrecognized = *c;
                    S::report_unrecognized_character(unrecognized as _);
                },
                None => {
                    j += 1;
                    continue;
                },
            }
            token += 1;
            j += 1
        }
        i += 1
    }
    let mut buffer = buffer.into_bytes();
    file_source.fill_buffer_with_side_to_move(&mut buffer);
    if buffer[0] == b'B' {
        *side_to_move = 0
    } else if buffer[0] == b'W' {
        *side_to_move = 2
    } else {
        let unrecognized = buffer[0];
        return Err(UnrecognizedCharacter(unrecognized));
    }
    const EMPTY: i32 = 1;
    if board_state_.board[45] == EMPTY || board_state_.board[54] == EMPTY || board_state_.board[44] == EMPTY || board_state_.board[55] == EMPTY {
        // various pieces of the program are not ready for this, even though we have end solve routines for it
        // TODO enable this use case (it requires changes to the endgame solver,
        //   various helper structure only initialize for 60 moves and leave out the initial 4)
        return Err(BoardSourceError::InitialSquaresAreEmpty);
    }
    return Ok(())
}


pub trait FileBoardSource : BoardSource {
    fn open(file_name: &CStr) -> Option<Self> where Self: Sized;
}
/*
   GAME_INIT
   Prepare the relevant data structures so that a game
   can be played. The position is read from the file
   specified by FILE_NAME.
*/
pub fn generic_game_init<Source: FileBoardSource, FE: FrontEnd>(file_name: Option<&CStr>, side_to_move: &mut i32, g_state: &mut FullState) {
    if let Some(file_name) = file_name {
        g_state.board.board = create_fresh_board();
        match Source::open((file_name)) {
            Some(file_source) => {
                match process_board_source(side_to_move, file_source, &mut g_state.board) {
                    Ok(_) => {}
                    Err(BoardSourceError::UnrecognizedCharacter(unrecognized)) => FE::unrecognized_character(unrecognized as _),
                    Err(BoardSourceError::InitialSquaresAreEmpty) => {
                        FE::initial_squares_are_empty()
                    }
                }
            },
            None => {
                FE::cannot_open_game_file((file_name).to_str().unwrap());
            },
        };
        setup_game_finalize(side_to_move, &mut g_state.board, &mut g_state.hash, &mut g_state.moves);
    } else {
        setup_non_file_based_game(side_to_move, &mut g_state.board, &mut g_state.hash, &mut g_state.moves);
    }
    engine_game_init(g_state);
}

pub fn generic_compute_move<L: ComputeMoveLogger, Out: ComputeMoveOutput, FE: FrontEnd, Thor: ThorDatabase>(side_to_move: i32,
                                                                                               update_all: i32,
                                                                                               my_time: i32,
                                                                                               my_incr: i32,
                                                                                               timed_depth: i32,
                                                                                               book: i32,
                                                                                               mut mid: i32,
                                                                                               exact: i32,
                                                                                               wld: i32,
                                                                                               search_forced: i32,
                                                                                               eval_info: &mut EvaluationType,
                                                                                               logger: &mut Option<L>,
                                                                                               display_pv:i32,
                                                                                               echo:i32, g_state: &mut FullState,
                                                                                               thor: &Thor
)
                                                                                               -> i8 {
    let mut book_eval_info = EvaluationType::new();
    let mut mid_eval_info = EvaluationType::new();
    let mut end_eval_info = EvaluationType::new();
    let mut midgame_diff: f64 = 0.;
    let mut midgame_depth: i32 = 0;
    let mut max_depth: i32 = 0;
    let mut endgame_reached: i32 = 0;
    let mut offset: i32 = 0;

    if let Some(logger) = logger {

        let side_to_move_ = side_to_move;
        L::log_board(logger, &g_state.board, side_to_move_);
    }
    /* Initialize various components of the move system */
    g_state.board.piece_count[0][g_state.moves.disks_played as usize] =
        disc_count(0, &g_state.board.board);
    g_state.board.piece_count[2][g_state.moves.disks_played as usize] =
        disc_count(2, &g_state.board.board);
    generate_all(side_to_move, &mut g_state.moves, &g_state.search, &g_state.board.board);
    determine_hash_values(side_to_move, &g_state.board.board, &mut g_state.hash);
    calculate_perturbation(&mut g_state.midgame, &mut g_state.random);
    if let Some(logger) = logger {
        let moves_generated = g_state.moves.move_count[g_state.moves.disks_played as usize];
        let move_list_for_disks_played = &g_state.moves.move_list[g_state.moves.disks_played as usize];

        L::log_moves_generated(logger, moves_generated, move_list_for_disks_played);
    }
    if update_all != 0 {
        reset_counter(&mut g_state.search.evaluations);
        reset_counter(&mut g_state.search.nodes);
    }
    let mut i = 0;
    while i < 100 {
        g_state.search.evals[g_state.moves.disks_played as usize][i as usize] = 0;
        i += 1
    }
    g_state.game.max_depth_reached = 1;
    let empties = 60 - g_state.moves.disks_played;
    FE::reset_buffer_display(&mut g_state.timer);
    g_state.timer.determine_move_time(my_time as f64, my_incr as f64,
                                      g_state.moves.disks_played + 4);
    if g_state.search.get_ponder_move() == 0 {  g_state.timer.clear_ponder_times(); }
    remove_coeffs(g_state.moves.disks_played, &mut g_state.coeff);
    /* No feasible moves? */
    if g_state.moves.move_count[g_state.moves.disks_played as usize] == 0 {
        *eval_info =
            create_eval_info(PASS_EVAL, UNSOLVED_POSITION,
                             0.0f64 as i32, 0.0f64, 0,
                             0);
        let eval = *eval_info;
        g_state.search.set_current_eval(eval);
        if echo != 0 {
            let info = &*eval_info;
            Out::echo_compute_move_1(info);
        }
        if let Some(logger) = logger {
            L::log_best_move_pass(logger);
        }
        g_state.game.last_time_used = 0.0f64;
        g_state.board.clear_pv();
        return -(1)
    }
    /* If there is only one move available:
       Don't waste any time, unless told so or very close to the end,
       searching the position. */
    if empties > 60 &&
        g_state.moves.move_count[g_state.moves.disks_played as usize] == 1 &&
        search_forced == 0 {
        /* Forced move */
        *eval_info =
            create_eval_info(FORCED_EVAL, UNSOLVED_POSITION,
                             0.0f64 as i32, 0.0f64, 0,
                             0);
        let eval = *eval_info;
        g_state.search.set_current_eval(eval);
        if echo != 0 {
            let info = &*eval_info;
            let disk = g_state.moves.move_list[g_state.moves.disks_played as usize][0];
            Out::echo_compute_move_2(info, disk);
        }
        if let Some(logger) = logger {
            let best_move = g_state.moves.move_list[g_state.moves.disks_played as usize][0];
            L::log_best_move(logger, best_move);
        }
        g_state.game.last_time_used = 0.0f64;
        return g_state.moves.move_list[g_state.moves.disks_played as usize][0]
    }
    /* Mark the search as interrupted until a successful search
       has been performed. */
    let mut move_type = INTERRUPTED_MOVE;
    let mut interrupted_depth = 0;
    let mut curr_move = g_state.moves.move_list[g_state.moves.disks_played as usize][0];
    /* Check the opening book for midgame moves */
    let mut book_move_found = 0;
    let mut midgame_move = -1;
    if let Some(forced_opening) = g_state.game.forced_opening.as_ref() {
        /* Check if the position fits the currently forced opening */
        curr_move = check_forced_opening::<FE>(
            side_to_move,
            ForcedOpening::from_c_str(forced_opening),
            &g_state.board.board,
            g_state.moves.disks_played,
            &g_state.g_book,
            &mut g_state.random) as i8;
        if curr_move != -1 {
            book_eval_info =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 0, 0.0f64, 0,
                                 1);
            midgame_move = curr_move;
            book_move_found = 1;
            move_type = BOOK_MOVE;
            if echo != 0 {
                let ponder_move = g_state.search.get_ponder_move();
                Out::echo_ponder_move(curr_move, ponder_move);
            }
            g_state.board.clear_pv();
            g_state.board.pv_depth[0] = 1;
            g_state.board.pv[0][0] =
                curr_move
        }
    }
    if book_move_found == 0 && play_thor_match_openings != 0 {
        /* Optionally use the Thor database as opening book. */
        let threshold = 2;
        Thor::database_search(thor, &g_state.board.board, side_to_move);
        if Thor::get_match_count(thor) >= threshold {
            let game_index =
                ((g_state.random.my_random() >> 8) %
                    Thor::get_match_count(thor) as i64) as i32;
            curr_move = Thor::get_thor_game_move(thor, game_index, g_state.moves.disks_played) as i8;
            if valid_move(curr_move, side_to_move, &g_state.board.board) != 0 {
                book_eval_info =
                    create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                     0, 0.0f64,
                                     0, 1);
                midgame_move = curr_move;
                book_move_found = 1;
                move_type = BOOK_MOVE;
                if echo != 0 {
                    let ponder_move = g_state.search.get_ponder_move();
                    Out::echo_ponder_move_2(curr_move, ponder_move);
                }
                g_state.board.clear_pv();
                g_state.board.pv_depth[0] = 1;
                g_state.board.pv[0][0] =
                    curr_move
            } else {
                FE::invalid_move(curr_move);
            }
        }
    }
    if book_move_found == 0 && g_state.game.play_human_openings != 0 && book != 0 {
        /* Check Thor statistics for a move */
        curr_move =
            Thor::choose_thor_opening_move(thor, &g_state.board.board, side_to_move,
                                           0, &mut g_state.random) as i8;
        if curr_move != -(1) {
            book_eval_info =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 0, 0.0f64, 0,
                                 1);
            midgame_move = curr_move;
            book_move_found = 1;
            move_type = BOOK_MOVE;
            if echo != 0 {
                let ponder_move = g_state.search.get_ponder_move();
                Out::echo_ponder_move_4(curr_move, ponder_move);
            }
            g_state.board.clear_pv();
            g_state.board.pv_depth[0] = 1;
            g_state.board.pv[0][0] =
                curr_move
        }
    }
    if book_move_found == 0 && book != 0 {
        /* Check ordinary opening book */
        let mut flags = 0;
        if empties <= 30 {
            if empties <= wld { flags = 4 }
            if empties <= exact { flags = 16 }
        }
        fill_move_alternatives::<FE>(side_to_move, flags,
                                     &mut g_state.g_book,
                                     &mut g_state.board,
                                     &mut g_state.moves,
                                     &g_state.search,
                                     &mut g_state.flip_stack,
                                     &mut g_state.hash);
        curr_move =
             get_book_move::<FE>(side_to_move, update_all, &mut book_eval_info, echo,
                                 &mut g_state.board,
                                 &mut g_state.g_book,
                                 &g_state.search,
                                 &mut g_state.moves,
                                 &mut g_state.hash,
                                 &mut g_state.random,
                                 &mut g_state.flip_stack);
        if curr_move != -(1) {
            let eval = book_eval_info;
            g_state.search.set_current_eval(eval);
            midgame_move = curr_move;
            book_move_found = 1;
            move_type = BOOK_MOVE;
            Out::display_status_out();
        }
    }
    /* Use iterative deepening in the midgame searches until the endgame
       is reached. If an endgame search already has been performed,
       make a much more shallow midgame search. Also perform much more
       shallow searches when there is no time limit and hence no danger
       starting to solve only to get interrupted. */
    if timed_depth == 0 && empties <= (if exact > wld { exact } else { wld })
    {
        mid =
            if (if (if mid < empties - 7 {
                mid
            } else { (empties) - 7 }) <
                28 {
                if mid < empties - 7 {
                    mid
                } else { (empties) - 7 }
            } else { 28 }) > 2 {
                if (if mid < empties - 7 {
                    mid
                } else { (empties) - 7 }) <
                    28 {
                    if mid < empties - 7 {
                        mid
                    } else { (empties) - 7 }
                } else { 28 }
            } else { 2 }
    }
    endgame_reached =
        (timed_depth == 0 && g_state.game.endgame_performed[side_to_move as usize] != 0) as
            i32;
    if book_move_found == 0 && endgame_reached == 0 {
        g_state.timer.clear_panic_abort();
        g_state.midgame.clear_midgame_abort();
        g_state.midgame.toggle_midgame_abort_check(update_all);
        g_state.midgame.toggle_midgame_hash_usage(1, 1);
        if timed_depth != 0 {
            max_depth = 64
        } else if empties <= (if exact > wld { exact } else { wld }) {
            max_depth =
                if (if (if mid < empties - 12 {
                    mid
                } else { (empties) - 12 }) <
                    18 {
                    if mid < empties - 12 {
                        mid
                    } else { (empties) - 12 }
                } else { 18 }) > 2 {
                    if (if mid < empties - 12 {
                        mid
                    } else { (empties) - 12 }) <
                        18 {
                        if mid < empties - 12 {
                            mid
                        } else { (empties) - 12 }
                    } else { 18 }
                } else { 2 }
        } else { max_depth = mid }
        midgame_depth =
            if (2) < max_depth {
                2
            } else { max_depth };
        loop  {
            g_state.game.max_depth_reached = midgame_depth;
            midgame_move =
                middle_game::<FE>(side_to_move, midgame_depth, update_all,
                                  &mut mid_eval_info, echo, &mut g_state.moves,
                                  &mut g_state.search,
                                  &mut g_state.board,
                                  &mut g_state.hash,
                                  &mut g_state.flip_stack,
                                  &mut g_state.coeff, &mut g_state.prob_cut,
                                  &mut g_state.timer, &mut g_state.midgame);
            let eval = mid_eval_info;
            g_state.search.set_current_eval(eval);
            midgame_diff =
                1.3f64 * mid_eval_info.score as f64 / 128.0f64;
            if side_to_move == 0 {
                midgame_diff -= g_state.game.komi as f64
            } else { midgame_diff += g_state.game.komi as f64 }
            if timed_depth != 0 {
                /* Check if the endgame zone has been reached */
                offset = 7;
                /* These constants were chosen rather arbitrarily but intend
                   to make Zebra solve earlier if the position is lopsided. */
                if g_state.timer.is_panic_abort() != 0 { offset -= 1 }
                if g_state.game.endgame_performed[side_to_move as usize] != 0 {
                    offset += 2
                }
                if midgame_depth + offset + 27 >=
                    2 * empties ||
                    midgame_depth + 7 >= empties {
                    endgame_reached = 1
                }
            }
            midgame_depth += 1;
            if !(g_state.timer.is_panic_abort() == 0 && g_state.midgame.is_midgame_abort() == 0 &&
                force_return == 0 && midgame_depth <= max_depth &&
                midgame_depth + g_state.moves.disks_played <= 61 &&
                endgame_reached == 0) {
                break ;
            }
        }
        if echo != 0 { Out::display_status_out(); }
        if abs(mid_eval_info.score) == abs(-(27000)) {
            move_type = INTERRUPTED_MOVE;
            interrupted_depth = midgame_depth - 1
            /* compensate for increment */
        } else { move_type = MIDGAME_MOVE }
    }
    let mut curr_move = midgame_move;
    /* If the endgame has been reached, solve the position */
    if force_return == 0 {
        if timed_depth != 0 && endgame_reached != 0 ||
            timed_depth != 0 && book_move_found != 0 &&
                g_state.moves.disks_played >= 60 - 30 ||
            timed_depth == 0 &&
                empties <= (if exact > wld { exact } else { wld }) {
            g_state.game.max_depth_reached = empties;
            g_state.timer.clear_panic_abort();
            if timed_depth != 0 {
                curr_move =
                   end_game::<FE>(side_to_move,
                                  (g_state.moves.disks_played < 60 - exact) as
                                 i32, 0, book, g_state.game.komi,
                                  &mut end_eval_info, echo, g_state)
            } else if empties <= exact {
                curr_move = end_game::<FE>(side_to_move, 0, 0,
                                           book, g_state.game.komi,
                                           &mut end_eval_info, echo, g_state)
            } else {
                curr_move = end_game::<FE>(side_to_move, 1, 0,
                                           book, g_state.game.komi,
                                           &mut end_eval_info, echo, g_state)
            }
            let eval = end_eval_info;
            g_state.search.set_current_eval(eval);
            if abs(g_state.search.root_eval) == abs(-(27000)) {
                move_type = INTERRUPTED_MOVE
            } else { move_type = ENDGAME_MOVE }
            if update_all != 0 {
                g_state.game.endgame_performed[side_to_move as usize] = 1
            }
        }
    }
    match move_type as u32 {
        0 => {
            *eval_info =
                create_eval_info(INTERRUPTED_EVAL, UNSOLVED_POSITION,
                                 0.0f64 as i32, 0.0f64,
                                 0, 0);
            let info = &*eval_info;
            let counter_value = counter_value(&mut g_state.search.nodes);
            Out::send_move_type_0_status(interrupted_depth, info, counter_value, &mut g_state.timer, &g_state.board);
        }
        1 => { *eval_info = book_eval_info }
        2 => { *eval_info = mid_eval_info }
        3 => { *eval_info = end_eval_info }
        _ => { }
    }
    let eval = *eval_info;
    g_state.search.set_current_eval(eval);
    g_state.game.last_time_used =  g_state.timer.get_elapsed_time();
    if update_all != 0 {
        g_state.search.total_time += g_state.game.last_time_used;
        add_counter(&mut g_state.search.total_evaluations, &mut g_state.search.evaluations);
        add_counter(&mut g_state.search.total_nodes, &mut g_state.search.nodes);
    }
    g_state.timer.clear_panic_abort();
    /* Write the contents of the status buffer to the log file. */
    if move_type as u32 == BOOK_MOVE as i32 as u32 {
        if let Some(logger) = logger {
            let info = &*eval_info;
            L::log_chosen_move(logger, curr_move, info);
        }
    } else if let Some(logger) = logger {
        L::log_status(logger);
    }
    /* Write the principal variation, if available, to the log file
       and, optionally, to screen. */
    if g_state.search.get_ponder_move() == 0 {
        let res = complete_pv(side_to_move, &mut g_state.search, &mut g_state.board, &mut g_state.flip_stack, &mut g_state.hash, &mut g_state.moves);
        if let Err(e) = res {
            FE::handle_fatal_pv_error(e.pv_depth_index, g_state.board.pv_depth[0], &g_state.board.pv[0]);
        }
        if display_pv != 0 && echo != 0 { Out::display_out_optimal_line(&g_state.search); }
        if let Some(logger) = logger { L::log_optimal_line(logger, &g_state.search); }
    }
    if let Some(logger) = logger {
        L::close_logger(logger);
    }
    return curr_move;
}

/*
   COMPUTE_MOVE
   Returns the best move in a position given search parameters.
*/
pub fn compute_move<L: ComputeMoveLogger, Out: ComputeMoveOutput, FE: FrontEnd, Thor: ThorDatabase>(
    side_to_move: i32,
    update_all: i32,
    my_time: i32,
    my_incr: i32,
    timed_depth: i32,
    book: i32,
    mid: i32,
    exact: i32,
    wld: i32,
    search_forced: i32,
    eval_info: &mut EvaluationType, display_pv:i32, echo:i32,
    g_state: &mut FullState,
    thor: &Thor
)
    -> i8 {
    return generic_compute_move::<L, Out, FE, Thor>(
        side_to_move, update_all, my_time,
        my_incr, timed_depth,
        book, mid,
        exact, wld,
        search_forced, eval_info, &mut L::create_log_file_if_needed(), display_pv, echo,
        g_state,
        thor
    );
}

pub trait ComputeMoveOutput {
    fn display_out_optimal_line(search_state: &SearchState);
    fn send_move_type_0_status(interrupted_depth: i32, info: &EvaluationType, counter_value: f64, timer: &mut Timer, board_state: &BoardState);
    fn display_status_out();
    fn echo_ponder_move_4(curr_move: i8, ponder_move: i8);
    fn echo_ponder_move_2(curr_move: i8, ponder_move: i8);
    fn echo_ponder_move(curr_move: i8, ponder_move: i8);
    fn echo_compute_move_2(info: &EvaluationType, disk: i8);
    fn echo_compute_move_1(info: &EvaluationType);
}
pub trait ComputeMoveLogger {
    fn log_moves_generated(logger: &mut Self, moves_generated: i32, move_list_for_disks_played: &[i8; 64]);
    fn log_best_move_pass(logger: &mut Self);
    fn log_best_move(logger: &mut Self, best_move: i8);
    fn log_chosen_move(logger: &mut Self, curr_move: i8, info: &EvaluationType);
    fn log_status(logger: &mut Self);
    fn log_optimal_line(logger: &mut Self, search_state: &SearchState);
    fn close_logger(logger: &mut Self);
    fn log_board(logger: &mut Self, board_: & BoardState, side_to_move_: i32);
    fn create_log_file_if_needed() -> Option<Self> where Self:Sized;
}
pub fn to_lower(ch: i32) -> i32 {
    char::from(ch as u8).to_ascii_lowercase() as i32
}
pub struct ForcedOpening {
    pub move_count: i32,
    pub moves: [i32; 60],
}
impl ForcedOpening {
    pub fn from_c_str(opening: &CStr) -> Self {
        let opening = opening.to_bytes();
        let mut i = 0;
        let mut move_0: [i32; 60] = [0; 60];
        let move_count_0 = opening.len().wrapping_div(2) as i32;
        while i < move_count_0 {
            move_0[i as usize] = 10 * (*opening.offset((2 * i + 1) as isize) as i32 - '0' as i32) +
                to_lower(*opening.offset((2 * i) as isize) as i32) - 'a' as i32 + 1;
            i += 1
        };
        ForcedOpening {
            move_count: move_count_0,
            moves: move_0
        }
    }
}
