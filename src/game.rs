use crate::src::globals::{pv_depth, pv, board, piece_count};
use crate::src::search::{nodes, evaluations, set_current_eval, create_eval_info, force_return, evals, disc_count, negate_current_eval, clear_ponder_move, set_ponder_move, float_move, sort_moves};
use crate::src::counter::{reset_counter};
use crate::src::stubs::{fclose, free, fprintf, abs, fputs, fopen, puts, printf, fgets, time, ctime, strcpy, stdout};
use crate::src::display::{display_optimal_line, echo, display_status, produce_eval_text, send_status, send_status_time, send_status_pv, send_status_nodes, clear_status, display_board};
use crate::src::timer::{clear_ponder_times, toggle_abort_check, start_move, ponder_depth, add_ponder_time, get_real_timer};

use crate::src::moves::{disks_played, move_list, move_count, generate_all, unmake_move, make_move};
use crate::src::midgame::{toggle_midgame_abort_check, toggle_perturbation_usage};
use crate::src::osfbook::{get_book_move, fill_move_alternatives, get_candidate, get_candidate_count};


use crate::src::myrandom::{my_random};
use crate::src::getcoeff::{pattern_evaluation, load_coeff_adjustments};
use crate::src::hash::{determine_hash_values, set_hash_transformation, find_hash, HashEntry};
use crate::src::zebra::{EvaluationType, _IO_FILE};
pub use engine::src::game::*;
use crate::src::getcoeff::zlib_source::ZLibSource;
use engine::src::error::LibcFatalError;

pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type size_t = u64;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type time_t = __time_t;
pub static mut log_file_path: [i8; 2048] = [0; 2048];
/*
   GLOBAL_SETUP
   Initialize the different sub-systems.
*/

pub unsafe fn global_setup(use_random: i32,
                                      hash_bits: i32) {
    LogFileHandler::on_global_setup();
    let coeff_adjustments = load_coeff_adjustments();
    engine_global_setup(use_random, hash_bits, coeff_adjustments, ZLibSource::new());
}
trait Logger {
    fn on_global_setup();
}
pub struct LogFileHandler {
    log_file: *mut FILE
}
impl Logger for LogFileHandler {
    fn on_global_setup() {
        unsafe { setup_log_file() }
    }
}
unsafe fn setup_log_file() {
    /* Clear the log file. No error handling done. */
    strcpy(log_file_path.as_mut_ptr(),
           b"zebra.log\x00" as *const u8 as *const i8);
    if use_log_file != 0 {
        let log_file =
            fopen(log_file_path.as_mut_ptr(),
                  b"w\x00" as *const u8 as *const i8);
        if !log_file.is_null() {
            let mut timer: time_t = 0;
            time(&mut timer);
            fprintf(log_file,
                    b"%s %s\n\x00" as *const u8 as *const i8,
                    b"Log file created\x00" as *const u8 as
                        *const i8, ctime(&mut timer));
            fprintf(log_file,
                    b"%s %s %s\n\x00" as *const u8 as *const i8,
                    b"Engine compiled\x00" as *const u8 as
                        *const i8,
                    b"Jul  2 2020\x00" as *const u8 as *const i8,
                    b"19:33:59\x00" as *const u8 as *const i8);
            fclose(log_file);
        }
    }
}

/*
   SETUP_GAME
   Prepares the board.
*/
unsafe fn setup_game(file_name: *const i8,
                                side_to_move: *mut i32) {
    generic_setup_game::<LibcBoardFileSource>(file_name, side_to_move)
}
pub struct LibcBoardFileSource {
    stream: *mut FILE
}
impl FileBoardSource for LibcBoardFileSource {
    unsafe fn open(file_name: *const i8) -> Option<LibcBoardFileSource> {
        let stream = fopen(file_name, b"r\x00" as *const u8 as *const i8);
        if stream.is_null() {
            return None;
        }
        Some(LibcBoardFileSource {
            stream
        })
    }
}
impl BoardSource for LibcBoardFileSource {
    // These methods and this whole scheme of loading the data honestly doesn't make any sense
    // but I don't want to refactor it for the better at the moment.
    fn fill_board_buffer(&mut self, buffer: &mut [i8; 70]) {
        unsafe {
            fgets(buffer.as_mut_ptr(), 70 as i32, self.stream);
        }
    }

    fn fill_buffer_with_side_to_move(&mut self, buffer: &mut [i8; 70]) {
        unsafe {
            fgets(buffer.as_mut_ptr(), 10 as i32, self.stream);
        }
    }

    fn report_unrecognized_character(unrecognized: i8) {
        unsafe {
            printf(b"%s \'%c\' %s\n\x00" as *const u8 as
                       *const i8,
                   b"Unrecognized character\x00" as *const u8 as
                       *const i8,
                   unrecognized as i32,
                   b"in game file\x00" as *const u8 as
                       *const i8);
        }
    }
}
/*
   GAME_INIT
   Prepare the relevant data structures so that a game
   can be played. The position is read from the file
   specified by FILE_NAME.
*/

pub unsafe fn game_init(file_name: *const i8, side_to_move: *mut i32) {
    generic_game_init::<LibcBoardFileSource>(file_name, side_to_move);
}
/*
  PONDER_MOVE
  Perform searches in response to the opponent's next move.
  The results are not returned, but the hash table is filled
  with useful scores and moves.
*/

pub unsafe fn ponder_move(side_to_move: i32,
                                     _book: i32,
                                     mid: i32,
                                     exact: i32,
                                     wld: i32) {
    let mut eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    let mut move_start_time: f64 = 0.;
    let mut move_stop_time: f64 = 0.;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut this_move: i32 = 0;
    let mut hash_move: i32 = 0;
    let mut expect_count: i32 = 0;
    let mut stored_echo: i32 = 0;
    let mut best_pv_depth: i32 = 0;
    let mut expect_list: [i32; 64] = [0; 64];
    let mut best_pv: [i32; 61] = [0; 61];
    /* Disable all time control mechanisms as it's the opponent's
       time we're using */
    toggle_abort_check(0 as i32);
    toggle_midgame_abort_check(0 as i32);
    start_move(0 as i32 as f64,
               0 as i32 as f64,
               disc_count(0 as i32) + disc_count(2 as i32));
    clear_ponder_times();
    determine_hash_values(side_to_move, board.as_mut_ptr());
    reset_counter(&mut nodes);
    /* Find the scores for the moves available to the opponent. */
    hash_move = 0 as i32;
    find_hash(&mut entry, 1 as i32);
    if entry.draft as i32 != 0 as i32 {
        hash_move = entry.move_0[0 as i32 as usize]
    } else {
        find_hash(&mut entry, 0 as i32);
        if entry.draft as i32 != 0 as i32 {
            hash_move = entry.move_0[0 as i32 as usize]
        }
    }
    stored_echo = echo;
    echo = 0 as i32;
    compute_move(side_to_move, 0 as i32, 0 as i32,
                 0 as i32, 0 as i32, 0 as i32,
                 if (8 as i32) < mid {
                     8 as i32
                 } else { mid }, 0 as i32, 0 as i32,
                 0 as i32, &mut eval_info);
    echo = stored_echo;
    /* Sort the opponents on the score and push the table move (if any)
       to the front of the list */
    if force_return != 0 {
        expect_count = 0 as i32
    } else {
        sort_moves(move_count[disks_played as usize]);
        float_move(hash_move, move_count[disks_played as usize]);
        expect_count = move_count[disks_played as usize];
        i = 0 as i32;
        while i < expect_count {
            expect_list[i as usize] =
                move_list[disks_played as usize][i as usize];
            i += 1
        }
        printf(b"%s=%d\n\x00" as *const u8 as *const i8,
               b"hash move\x00" as *const u8 as *const i8,
               hash_move);
        i = 0 as i32;
        while i < expect_count {
            printf(b"%c%c %-6.2f  \x00" as *const u8 as *const i8,
                   'a' as i32 +
                       move_list[disks_played as usize][i as usize] %
                           10 as i32 - 1 as i32,
                   '0' as i32 +
                       move_list[disks_played as usize][i as usize] /
                           10 as i32,
                   evals[disks_played as
                             usize][move_list[disks_played as
                                                  usize][i as usize] as usize]
                       as f64 / 128.0f64);
            if i % 7 as i32 == 6 as i32 ||
                   i == expect_count - 1 as i32 {
                puts(b"\x00" as *const u8 as *const i8);
            }
            i += 1
        }
    }
    /* Go through the expected moves in order and prepare responses. */
    best_pv_depth = 0 as i32;
    i = 0 as i32;
    while force_return == 0 && i < expect_count {
        move_start_time = get_real_timer();
        set_ponder_move(expect_list[i as usize]);
        this_move = expect_list[i as usize];
        prefix_move = this_move;
        make_move(side_to_move, this_move, 1 as i32);
        compute_move(0 as i32 + 2 as i32 - side_to_move,
                     0 as i32, 0 as i32, 0 as i32,
                     1 as i32, 0 as i32, mid, exact, wld,
                     0 as i32, &mut eval_info);
        unmake_move(side_to_move, this_move);
        clear_ponder_move();
        move_stop_time = get_real_timer();
        add_ponder_time(expect_list[i as usize],
                        move_stop_time - move_start_time);
        ponder_depth[expect_list[i as usize] as usize] =
            if ponder_depth[expect_list[i as usize] as usize] >
                   max_depth_reached - 1 as i32 {
                ponder_depth[expect_list[i as usize] as usize]
            } else { (max_depth_reached) - 1 as i32 };
        if i == 0 as i32 && force_return == 0 {
            /* Store the PV for the first move */
            best_pv_depth = pv_depth[0 as i32 as usize];
            j = 0 as i32;
            while j < pv_depth[0 as i32 as usize] {
                best_pv[j as usize] =
                    pv[0 as i32 as usize][j as usize];
                j += 1
            }
        }
        i += 1
    }
    /* Make sure the PV looks reasonable when leaving - either by
       clearing it altogether or, preferrably, using the stored PV for
       the first move if it is available. */
    max_depth_reached += 1;
    prefix_move = 0 as i32;
    if best_pv_depth == 0 as i32 {
        pv_depth[0 as i32 as usize] = 0 as i32
    } else {
        pv_depth[0 as i32 as usize] =
            best_pv_depth + 1 as i32;
        pv[0 as i32 as usize][0 as i32 as usize] =
            expect_list[0 as i32 as usize];
        i = 0 as i32;
        while i < best_pv_depth {
            pv[0 as i32 as usize][(i + 1 as i32) as usize] =
                best_pv[i as usize];
            i += 1
        }
    }
    /* Don't forget to enable the time control mechanisms when leaving */
    toggle_abort_check(1 as i32);
    toggle_midgame_abort_check(1 as i32);
}
/*
  EXTENDED_COMPUTE_MOVE
  This wrapper on top of compute_move() calculates the evaluation
  of all moves available as opposed to upper bounds for all moves
  except for the best.
*/

pub unsafe fn extended_compute_move(side_to_move: i32,
                                               book_only: i32,
                                               mut book: i32,
                                               mut mid: i32,
                                               mut exact: i32,
                                               mut wld: i32)
 -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut index: i32 = 0;
    let mut changed: i32 = 0;
    let mut this_move: i32 = 0;
    let mut disc_diff: i32 = 0;
    let mut corrected_diff: i32 = 0;
    let mut best_move: i32 = 0;
    let mut temp_move: i32 = 0;
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
    let mut unsearched_move: [i32; 61] = [0; 61];
    let mut best_pv: [i32; 60] = [0; 60];
    let mut transform1: [u32; 60] = [0; 60];
    let mut transform2: [u32; 60] = [0; 60];
    let mut book_move =
        CandidateMove{move_0: 0, score: 0, flags: 0, parent_flags: 0,};
    let mut temp =
        EvaluatedMove{eval:
                          EvaluationType{type_0: MIDGAME_EVAL,
                                         res: WON_POSITION,
                                         score: 0,
                                         confidence: 0.,
                                         search_depth: 0,
                                         is_book: 0,},
                      side_to_move: 0,
                      move_0: 0,
                      pv_depth: 0,
                      pv: [0; 60],};
    let mut book_eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut res = WON_POSITION;
    /* Disable all time control mechanisms and randomization */
    toggle_abort_check(0 as i32);
    toggle_midgame_abort_check(0 as i32);
    toggle_perturbation_usage(0 as i32);
    start_move(0 as i32 as f64,
               0 as i32 as f64,
               disc_count(0 as i32) + disc_count(2 as i32));
    clear_ponder_times();
    determine_hash_values(side_to_move, board.as_mut_ptr());
    empties = 60 as i32 - disks_played;
    best_move = 0 as i32;
    game_evaluated_count = 0 as i32;
    reset_counter(&mut nodes);
    generate_all(side_to_move);
    if book_only != 0 || book != 0 {
        /* Evaluations for database moves */
        let mut flags = 0 as i32;
        if empties <= exact {
            flags = 16 as i32
        } else if empties <= wld { flags = 4 as i32 }
        fill_move_alternatives(side_to_move, flags);
        game_evaluated_count = get_candidate_count();
        i = 0 as i32;
        while i < game_evaluated_count {
            let mut child_flags: i32 = 0;
            book_move = get_candidate(i);
            evaluated_list[i as usize].side_to_move = side_to_move;
            evaluated_list[i as usize].move_0 = book_move.move_0;
            evaluated_list[i as usize].pv_depth = 1 as i32;
            evaluated_list[i as usize].pv[0 as i32 as usize] =
                book_move.move_0;
            evaluated_list[i as usize].eval =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 book_move.score, 0.0f64, 0 as i32,
                                 1 as i32);
            child_flags = book_move.flags & book_move.parent_flags;
            if child_flags & (16 as i32 | 4 as i32) != 0 {
                if child_flags & 16 as i32 != 0 {
                    evaluated_list[i as usize].eval.type_0 = EXACT_EVAL
                } else { evaluated_list[i as usize].eval.type_0 = WLD_EVAL }
                if book_move.score > 0 as i32 {
                    evaluated_list[i as usize].eval.res = WON_POSITION;
                    /* Normalize the scores so that e.g. 33-31 becomes +256 */
                    evaluated_list[i as usize].eval.score -=
                        30000 as i32;
                    evaluated_list[i as usize].eval.score *=
                        128 as i32
                } else if book_move.score == 0 as i32 {
                    evaluated_list[i as usize].eval.res = DRAWN_POSITION
                } else {
                    /* score < 0 */
                    evaluated_list[i as usize].eval.res = LOST_POSITION;
                    /* Normalize the scores so that e.g. 30-34 becomes -512 */
                    evaluated_list[i as usize].eval.score +=
                        30000 as i32;
                    evaluated_list[i as usize].eval.score *=
                        128 as i32
                }
            } else { evaluated_list[i as usize].eval.type_0 = MIDGAME_EVAL }
            i += 1
        }
    }
    if book_only != 0 {
        /* Only book moves are to be considered */
        if game_evaluated_count > 0 as i32 {
            best_move =
                get_book_move(side_to_move, 0 as i32,
                              &mut book_eval_info);
            set_current_eval(book_eval_info);
        } else {
            pv_depth[0 as i32 as usize] = 0 as i32;
            best_move = -(1 as i32);
            book_eval_info =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 0 as i32, 0.0f64, 0 as i32,
                                 0 as i32);
            set_current_eval(book_eval_info);
        }
    } else {
        /* Make searches for moves not in the database */
        let mut shallow_depth: i32 = 0;
        let empties_0 = 60 as i32 - disks_played;
        book = 0 as i32;
        best_score = -(12345678 as i32);
        if game_evaluated_count > 0 as i32 {
            /* Book PV available */
            best_score = evaluated_list[0 as i32 as usize].eval.score;
            best_move = evaluated_list[0 as i32 as usize].move_0
        }
        negate_current_eval(1 as i32);
        /* Store the available moves, clear their evaluations and sort
           them on shallow evaluation. */
        if empties_0 < 12 as i32 {
            shallow_depth = 1 as i32
        } else {
            let max_depth =
                if mid > (if exact > wld { exact } else { wld }) {
                    mid
                } else if exact > wld { exact } else { wld };
            if max_depth >= 16 as i32 {
                shallow_depth = 6 as i32
            } else { shallow_depth = 4 as i32 }
        }
        unsearched_count = 0 as i32;
        i = 0 as i32;
        while i < move_count[disks_played as usize] {
            this_move = move_list[disks_played as usize][i as usize];
            unsearched = 1 as i32;
            j = 0 as i32;
            while j < game_evaluated_count {
                if evaluated_list[j as usize].move_0 == this_move {
                    unsearched = 0 as i32
                }
                j += 1
            }
            if !(unsearched == 0) {
                unsearched_move[unsearched_count as usize] = this_move;
                unsearched_count += 1;
                make_move(side_to_move, this_move, 1 as i32);
                if shallow_depth == 1 as i32 {
                    /* Compute move doesn't allow depth 0 */
                    evaluations.lo = evaluations.lo.wrapping_add(1);
                    shallow_eval =
                        -pattern_evaluation(0 as i32 +
                                                2 as i32 -
                                                side_to_move)
                } else {
                    let mut shallow_info =
                        EvaluationType{type_0: MIDGAME_EVAL,
                                       res: WON_POSITION,
                                       score: 0,
                                       confidence: 0.,
                                       search_depth: 0,
                                       is_book: 0,};
                    compute_move(0 as i32 + 2 as i32 -
                                     side_to_move, 0 as i32,
                                 0 as i32, 0 as i32,
                                 0 as i32, book,
                                 shallow_depth - 1 as i32,
                                 0 as i32, 0 as i32,
                                 1 as i32, &mut shallow_info);
                    if shallow_info.type_0 as u32 ==
                           PASS_EVAL as i32 as u32 {
                        /* Don't allow pass */
                        compute_move(side_to_move, 0 as i32,
                                     0 as i32, 0 as i32,
                                     0 as i32, book,
                                     shallow_depth - 1 as i32,
                                     0 as i32, 0 as i32,
                                     1 as i32, &mut shallow_info);
                        if shallow_info.type_0 as u32 ==
                               PASS_EVAL as i32 as u32 {
                            /* Game over */
                            disc_diff =
                                disc_count(side_to_move) -
                                    disc_count(0 as i32 +
                                                   2 as i32 -
                                                   side_to_move);
                            if disc_diff > 0 as i32 {
                                corrected_diff =
                                    64 as i32 -
                                        2 as i32 *
                                            disc_count(0 as i32 +
                                                           2 as i32 -
                                                           side_to_move)
                            } else if disc_diff == 0 as i32 {
                                corrected_diff = 0 as i32
                            } else {
                                corrected_diff =
                                    2 as i32 *
                                        disc_count(side_to_move) -
                                        64 as i32
                            }
                            shallow_eval = 128 as i32 * corrected_diff
                        } else { shallow_eval = shallow_info.score }
                    } else {
                        /* Sign-correct the score produced */
                        shallow_eval = -shallow_info.score
                    }
                }
                unmake_move(side_to_move, this_move);
                evals[disks_played as usize][this_move as usize] =
                    shallow_eval
            }
            i += 1
        }
        loop  {
            changed = 0 as i32;
            i = 0 as i32;
            while i < unsearched_count - 1 as i32 {
                if evals[disks_played as
                             usize][unsearched_move[i as usize] as usize] <
                       evals[disks_played as
                                 usize][unsearched_move[(i + 1 as i32)
                                                            as usize] as
                                            usize] {
                    temp_move = unsearched_move[i as usize];
                    unsearched_move[i as usize] =
                        unsearched_move[(i + 1 as i32) as usize];
                    unsearched_move[(i + 1 as i32) as usize] =
                        temp_move;
                    changed = 1 as i32
                }
                i += 1
            }
            if !(changed != 0) { break ; }
        }
        /* Initialize the entire list as being empty */
        i = 0 as i32;
        index = game_evaluated_count;
        while i < unsearched_count {
            evaluated_list[index as usize].side_to_move = side_to_move;
            evaluated_list[index as usize].move_0 =
                unsearched_move[i as usize];
            evaluated_list[index as usize].eval =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 0 as i32, 0.0f64, 0 as i32,
                                 0 as i32);
            evaluated_list[index as usize].pv_depth = 1 as i32;
            evaluated_list[index as usize].pv[0 as i32 as usize] =
                unsearched_move[i as usize];
            if empties_0 > (if wld > exact { wld } else { exact }) {
                transform1[i as usize] =
                    abs(my_random() as i32) as u32;
                transform2[i as usize] =
                    abs(my_random() as i32) as u32
            } else {
                transform1[i as usize] = 0 as i32 as u32;
                transform2[i as usize] = 0 as i32 as u32
            }
            i += 1;
            index += 1
        }
        stored_echo = echo;
        echo = 0 as i32;
        best_pv_depth = 0 as i32;
        if mid == 1 as i32 {
            /* compute_move won't be called */
            pv_depth[0 as i32 as usize] = 0 as i32;
            piece_count[0 as i32 as usize][disks_played as usize] =
                disc_count(0 as i32);
            piece_count[2 as i32 as usize][disks_played as usize] =
                disc_count(2 as i32)
        }
        /* Perform iterative deepening if the search depth is large enough */
        if exact > empties_0 { exact = empties_0 }
        if exact < 12 as i32 || empties_0 > exact {
            current_exact = exact
        } else {
            current_exact =
                8 as i32 + exact % 2 as i32 - 2 as i32
        }
        if wld > empties_0 { wld = empties_0 }
        if wld < 14 as i32 || empties_0 > wld {
            current_wld = wld
        } else {
            current_wld =
                10 as i32 + wld % 2 as i32 - 2 as i32
        }
        if (empties_0 == exact || empties_0 == wld) &&
               empties_0 > 16 as i32 &&
               mid < empties_0 - 12 as i32 {
            mid = empties_0 - 12 as i32
        }
        if mid < 10 as i32 {
            current_mid = mid
        } else {
            current_mid =
                6 as i32 + mid % 2 as i32 - 2 as i32
        }
        first_iteration = 1 as i32;
        loop  {
            if current_mid < mid {
                current_mid += 2 as i32;
                /* Avoid performing deep midgame searches if the endgame
                   is reached anyway. */
                if empties_0 <= wld &&
                       current_mid + 7 as i32 >= empties_0 {
                    current_wld = wld;
                    current_mid = mid
                }
                if empties_0 <= exact &&
                       current_mid + 7 as i32 >= empties_0 {
                    current_exact = exact;
                    current_mid = mid
                }
            } else if current_wld < wld {
                current_wld = wld
            } else { current_exact = exact }
            i = 0 as i32;
            while i < unsearched_count && force_return == 0 {
                let mut this_eval =
                    EvaluationType{type_0: MIDGAME_EVAL,
                                   res: WON_POSITION,
                                   score: 0,
                                   confidence: 0.,
                                   search_depth: 0,
                                   is_book: 0,};
                this_move = unsearched_move[i as usize];
                /* Locate the current move in the list.  This has to be done
                   because the moves might have been reordered during the
                   iterative deepening. */
                index = 0 as i32;
                while evaluated_list[index as usize].move_0 != this_move {
                    index += 1
                }
                /* To avoid strange effects when browsing back and forth through
                   a game during the midgame, rehash the hash transformation masks
                   for each move unless the endgame is reached */
                set_hash_transformation(transform1[i as usize],
                                        transform2[i as usize]);
                /* Determine the score for the ith move */
                prefix_move = this_move;
                make_move(side_to_move, this_move, 1 as i32);
                if current_mid == 1 as i32 {
                    /* compute_move doesn't like 0-ply searches */
                    evaluations.lo = evaluations.lo.wrapping_add(1);
                    shallow_eval =
                        pattern_evaluation(0 as i32 + 2 as i32
                                               - side_to_move);
                    this_eval =
                        create_eval_info(MIDGAME_EVAL, UNSOLVED_POSITION,
                                         shallow_eval, 0.0f64,
                                         0 as i32, 0 as i32)
                } else {
                    compute_move(0 as i32 + 2 as i32 -
                                     side_to_move, 0 as i32,
                                 0 as i32, 0 as i32,
                                 0 as i32, book,
                                 current_mid - 1 as i32,
                                 current_exact - 1 as i32,
                                 current_wld - 1 as i32,
                                 1 as i32, &mut this_eval);
                }
                if force_return != 0 {
                    /* Clear eval and exit search immediately */
                    this_eval =
                        create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                         0 as i32, 0.0f64,
                                         0 as i32, 0 as i32);
                    unmake_move(side_to_move, this_move);
                    break ;
                } else {
                    if this_eval.type_0 as u32 ==
                           PASS_EVAL as i32 as u32 {
                        /* Don't allow pass */
                        if current_mid == 1 as i32 {
                            /* compute_move doesn't like 0-ply searches */
                            evaluations.lo = evaluations.lo.wrapping_add(1);
                            shallow_eval = pattern_evaluation(side_to_move);
                            this_eval =
                                create_eval_info(MIDGAME_EVAL,
                                                 UNSOLVED_POSITION,
                                                 shallow_eval, 0.0f64,
                                                 0 as i32,
                                                 0 as i32)
                        } else {
                            compute_move(side_to_move, 0 as i32,
                                         0 as i32, 0 as i32,
                                         0 as i32, book,
                                         current_mid - 1 as i32,
                                         current_exact - 1 as i32,
                                         current_wld - 1 as i32,
                                         1 as i32, &mut this_eval);
                        }
                        if this_eval.type_0 as u32 ==
                               PASS_EVAL as i32 as u32 {
                            /* Game over */
                            disc_diff =
                                disc_count(side_to_move) -
                                    disc_count(0 as i32 +
                                                   2 as i32 -
                                                   side_to_move);
                            if disc_diff > 0 as i32 {
                                corrected_diff =
                                    64 as i32 -
                                        2 as i32 *
                                            disc_count(0 as i32 +
                                                           2 as i32 -
                                                           side_to_move);
                                res = WON_POSITION
                            } else if disc_diff == 0 as i32 {
                                corrected_diff = 0 as i32;
                                res = DRAWN_POSITION
                            } else {
                                corrected_diff =
                                    2 as i32 *
                                        disc_count(side_to_move) -
                                        64 as i32;
                                res = LOST_POSITION
                            }
                            this_eval =
                                create_eval_info(EXACT_EVAL, res,
                                                 128 as i32 *
                                                     corrected_diff, 0.0f64,
                                                 60 as i32 -
                                                     disks_played,
                                                 0 as i32)
                        }
                    } else {
                        /* Sign-correct the score produced */
                        this_eval.score = -this_eval.score;
                        if this_eval.res as u32 ==
                               WON_POSITION as i32 as u32 {
                            this_eval.res = LOST_POSITION
                        } else if this_eval.res as u32 ==
                                      LOST_POSITION as i32 as
                                          u32 {
                            this_eval.res = WON_POSITION
                        }
                    }
                    if force_return != 0 { break ; }
                    evaluated_list[index as usize].eval = this_eval;
                    /* Store the PV corresponding to the move */
                    evaluated_list[index as usize].pv_depth =
                        pv_depth[0 as i32 as usize] +
                            1 as i32;
                    evaluated_list[index as
                                       usize].pv[0 as i32 as usize] =
                        this_move;
                    j = 0 as i32;
                    while j < pv_depth[0 as i32 as usize] {
                        evaluated_list[index as
                                           usize].pv[(j + 1 as i32) as
                                                         usize] =
                            pv[0 as i32 as usize][j as usize];
                        j += 1
                    }
                    /* Store the PV corresponding to the best move */
                    if evaluated_list[index as usize].eval.score > best_score
                       {
                        best_score =
                            evaluated_list[index as usize].eval.score;
                        best_move = this_move;
                        best_pv_depth = pv_depth[0 as i32 as usize];
                        j = 0 as i32;
                        while j < best_pv_depth {
                            best_pv[j as usize] =
                                pv[0 as i32 as usize][j as usize];
                            j += 1
                        }
                    }
                    unmake_move(side_to_move, this_move);
                    /* Sort the moves evaluated */
                    if first_iteration != 0 { game_evaluated_count += 1 }
                    if force_return == 0 {
                        loop  {
                            changed = 0 as i32;
                            j = 0 as i32;
                            while j < game_evaluated_count - 1 as i32
                                  {
                                if compare_eval(evaluated_list[j as
                                                                   usize].eval,
                                                evaluated_list[(j +
                                                                    1 as
                                                                        i32)
                                                                   as
                                                                   usize].eval)
                                       < 0 as i32 {
                                    changed = 1 as i32;
                                    temp = evaluated_list[j as usize];
                                    evaluated_list[j as usize] =
                                        evaluated_list[(j + 1 as i32)
                                                           as usize];
                                    evaluated_list[(j + 1 as i32) as
                                                       usize] = temp
                                }
                                j += 1
                            }
                            if !(changed != 0) { break ; }
                        }
                    }
                    i += 1
                }
            }
            first_iteration = 0 as i32;
            /* Reorder the moves after each iteration.  Each move is moved to
            the front of the list, starting with the bad moves and ending
             with the best move.  This ensures that unsearched_move will be
             sorted w.r.t. the order in evaluated_list. */
            i = game_evaluated_count - 1 as i32;
            while i >= 0 as i32 {
                let this_move_0 = evaluated_list[i as usize].move_0;
                j = 0 as i32;
                while j != unsearched_count &&
                          unsearched_move[j as usize] != this_move_0 {
                    j += 1
                }
                if !(j == unsearched_count) {
                    /* Move the move to the front of the list. */
                    while j >= 1 as i32 {
                        unsearched_move[j as usize] =
                            unsearched_move[(j - 1 as i32) as usize];
                        j -= 1
                    }
                    unsearched_move[0 as i32 as usize] = this_move_0
                }
                /* Must be book move, skip */
                i -= 1
            }
            if !(force_return == 0 &&
                     (current_mid != mid || current_exact != exact ||
                          current_wld != wld)) {
                break ;
            }
        }
        echo = stored_echo;
        game_evaluated_count = move_count[disks_played as usize];
        /* Make sure that the PV and the score correspond to the best move */
        pv_depth[0 as i32 as usize] =
            best_pv_depth + 1 as i32;
        pv[0 as i32 as usize][0 as i32 as usize] = best_move;
        i = 0 as i32;
        while i < best_pv_depth {
            pv[0 as i32 as usize][(i + 1 as i32) as usize] =
                best_pv[i as usize];
            i += 1
        }
        negate_current_eval(0 as i32);
        if move_count[disks_played as usize] > 0 as i32 {
            set_current_eval(evaluated_list[0 as i32 as usize].eval);
        }
    }
    /* Reset the hash transformation masks prior to leaving */
    set_hash_transformation(0 as i32 as u32,
                            0 as i32 as u32);
    /* Don't forget to enable the time control mechanisms when leaving */
    toggle_abort_check(1 as i32);
    toggle_midgame_abort_check(1 as i32);
    toggle_perturbation_usage(1 as i32);
    max_depth_reached += 1;
    prefix_move = 0 as i32;
    return best_move;
}
/*
  PERFORM_EXTENDED_SOLVE
  Calculates exact score or WLD status for the move ACTUAL_MOVE as
  well as for the best move in the position (if it is any other move).
*/

pub unsafe fn perform_extended_solve(side_to_move: i32,
                                                actual_move: i32,
                                                book: i32,
                                                exact_solve:
                                                    i32) {
    let mut i: i32 = 0;
    let mut mid: i32 = 0;
    let mut wld: i32 = 0;
    let mut exact: i32 = 0;
    let mut best_move: i32 = 0;
    let mut disc_diff: i32 = 0;
    let mut corrected_diff: i32 = 0;
    let mut temp =
        EvaluatedMove{eval:
                          EvaluationType{type_0: MIDGAME_EVAL,
                                         res: WON_POSITION,
                                         score: 0,
                                         confidence: 0.,
                                         search_depth: 0,
                                         is_book: 0,},
                      side_to_move: 0,
                      move_0: 0,
                      pv_depth: 0,
                      pv: [0; 60],};
    let mut res = WON_POSITION;
    /* Disable all time control mechanisms */
    toggle_abort_check(0 as i32);
    toggle_midgame_abort_check(0 as i32);
    toggle_perturbation_usage(0 as i32);
    start_move(0 as i32 as f64,
               0 as i32 as f64,
               disc_count(0 as i32) + disc_count(2 as i32));
    clear_ponder_times();
    determine_hash_values(side_to_move, board.as_mut_ptr());
    reset_counter(&mut nodes);
    /* Set search depths that result in Zebra solving after a brief
       midgame analysis */
    mid = 60 as i32;
    wld = 60 as i32;
    if exact_solve != 0 {
        exact = 60 as i32
    } else { exact = 0 as i32 }
    game_evaluated_count = 1 as i32;
    /* Calculate the score for the preferred move */
    evaluated_list[0 as i32 as usize].side_to_move = side_to_move;
    evaluated_list[0 as i32 as usize].move_0 = actual_move;
    evaluated_list[0 as i32 as usize].eval =
        create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION, 0 as i32,
                         0.0f64, 0 as i32, 0 as i32);
    evaluated_list[0 as i32 as usize].pv_depth = 1 as i32;
    evaluated_list[0 as i32 as usize].pv[0 as i32 as usize] =
        actual_move;
    prefix_move = actual_move;
    negate_current_eval(1 as i32);
    make_move(side_to_move, actual_move, 1 as i32);
    compute_move(0 as i32 + 2 as i32 - side_to_move,
                 0 as i32, 0 as i32, 0 as i32,
                 0 as i32, book, mid - 1 as i32,
                 exact - 1 as i32, wld - 1 as i32,
                 1 as i32,
                 &mut (*evaluated_list.as_mut_ptr().offset(0 as i32 as
                                                               isize)).eval);
    if evaluated_list[0 as i32 as usize].eval.type_0 as u32
           == PASS_EVAL as i32 as u32 {
        /* Don't allow pass */
        compute_move(side_to_move, 0 as i32, 0 as i32,
                     0 as i32, 0 as i32, book,
                     mid - 1 as i32, exact - 1 as i32,
                     wld - 1 as i32, 1 as i32,
                     &mut (*evaluated_list.as_mut_ptr().offset(0 as
                                                                   i32
                                                                   as
                                                                   isize)).eval);
        if evaluated_list[0 as i32 as usize].eval.type_0 as
               u32 == PASS_EVAL as i32 as u32 {
            /* Game has ended */
            disc_diff =
                disc_count(side_to_move) -
                    disc_count(0 as i32 + 2 as i32 -
                                   side_to_move);
            if disc_diff > 0 as i32 {
                corrected_diff =
                    64 as i32 -
                        2 as i32 *
                            disc_count(0 as i32 + 2 as i32 -
                                           side_to_move);
                res = WON_POSITION
            } else if disc_diff == 0 as i32 {
                corrected_diff = 0 as i32;
                res = DRAWN_POSITION
            } else {
                corrected_diff =
                    2 as i32 * disc_count(side_to_move) -
                        64 as i32;
                res = LOST_POSITION
            }
            evaluated_list[0 as i32 as usize].eval =
                create_eval_info(EXACT_EVAL, res,
                                 128 as i32 * corrected_diff, 0.0f64,
                                 60 as i32 - disks_played,
                                 0 as i32)
        }
    } else {
        /* Sign-correct the score produced */
        evaluated_list[0 as i32 as usize].eval.score =
            -evaluated_list[0 as i32 as usize].eval.score;
        if evaluated_list[0 as i32 as usize].eval.res as u32
               == WON_POSITION as i32 as u32 {
            evaluated_list[0 as i32 as usize].eval.res = LOST_POSITION
        } else if evaluated_list[0 as i32 as usize].eval.res as
                      u32 ==
                      LOST_POSITION as i32 as u32 {
            evaluated_list[0 as i32 as usize].eval.res = WON_POSITION
        }
    }
    if force_return != 0 {
        evaluated_list[0 as i32 as usize].eval =
            create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                             0 as i32, 0.0f64, 0 as i32,
                             0 as i32)
    } else {
        evaluated_list[0 as i32 as usize].pv_depth =
            pv_depth[0 as i32 as usize] + 1 as i32;
        evaluated_list[0 as i32 as
                           usize].pv[0 as i32 as usize] = actual_move;
        i = 0 as i32;
        while i < pv_depth[0 as i32 as usize] {
            evaluated_list[0 as i32 as
                               usize].pv[(i + 1 as i32) as usize] =
                pv[0 as i32 as usize][i as usize];
            i += 1
        }
    }
    unmake_move(side_to_move, actual_move);
    prefix_move = 0 as i32;
    negate_current_eval(0 as i32);
    max_depth_reached += 1;
    /* Compute the score for the best move and store it in the move list
       if it isn't ACTUAL_MOVE */
    best_move =
        compute_move(side_to_move, 0 as i32, 0 as i32,
                     0 as i32, 0 as i32, book, mid, exact,
                     wld, 1 as i32,
                     &mut (*evaluated_list.as_mut_ptr().offset(1 as
                                                                   i32
                                                                   as
                                                                   isize)).eval);
    if force_return == 0 && best_move != actual_move {
        /* Move list will contain best move first and then the actual move */
        game_evaluated_count = 2 as i32;
        evaluated_list[1 as i32 as usize].side_to_move = side_to_move;
        evaluated_list[1 as i32 as usize].move_0 = best_move;
        evaluated_list[1 as i32 as usize].pv_depth =
            pv_depth[0 as i32 as usize];
        i = 0 as i32;
        while i < pv_depth[0 as i32 as usize] {
            evaluated_list[1 as i32 as usize].pv[i as usize] =
                pv[0 as i32 as usize][i as usize];
            i += 1
        }
        temp = evaluated_list[0 as i32 as usize];
        evaluated_list[0 as i32 as usize] =
            evaluated_list[1 as i32 as usize];
        evaluated_list[1 as i32 as usize] = temp
    }
    /* The PV and current eval should when correspond to the best move
       when leaving */
    pv_depth[0 as i32 as usize] =
        evaluated_list[0 as i32 as usize].pv_depth;
    i = 0 as i32;
    while i < pv_depth[0 as i32 as usize] {
        pv[0 as i32 as usize][i as usize] =
            evaluated_list[0 as i32 as usize].pv[i as usize];
        i += 1
    }
    set_current_eval(evaluated_list[0 as i32 as usize].eval);
    /* Don't forget to enable the time control mechanisms when leaving */
    toggle_abort_check(1 as i32);
    toggle_midgame_abort_check(1 as i32);
    toggle_perturbation_usage(0 as i32);
}

/*
   COMPUTE_MOVE
   Returns the best move in a position given search parameters.
*/
pub unsafe fn compute_move(side_to_move: i32,
                           update_all: i32,
                           my_time: i32,
                           my_incr: i32,
                           timed_depth: i32,
                           book: i32,
                           mid: i32,
                           exact: i32,
                           wld: i32,
                           search_forced: i32,
                           eval_info: *mut EvaluationType)
                           -> i32 {
    return generic_compute_move::<LogFileHandler, LibcZebraOutput, LibcFatalError>(side_to_move, update_all, my_time,
                                my_incr, timed_depth,
                                book, mid,
                                exact, wld,
                                search_forced, eval_info, &mut LogFileHandler::create_log_file_if_needed());
}

pub struct LibcZebraOutput;
impl ComputeMoveOutput for LibcZebraOutput {
fn display_out_optimal_line() {
    unsafe { display_optimal_line(stdout) }
}

fn send_move_type_0_status(interrupted_depth: i32, info: &EvaluationType, counter_value: f64, elapsed_time: f64) {
    unsafe {
        clear_status();
        send_status(b"--> *%2d\x00" as *const u8 as *const i8,
                    interrupted_depth);
        let eval_str = produce_eval_text(info, 1 as i32);
        send_status(b"%10s  \x00" as *const u8 as *const i8,
                    eval_str);
        free(eval_str as *mut std::ffi::c_void);
        send_status_nodes(counter_value);
        send_status_pv(pv[0 as i32 as usize].as_mut_ptr(),
                       interrupted_depth);
        send_status_time(elapsed_time);
        if elapsed_time != 0.0f64 {
            send_status(b"%6.0f %s\x00" as *const u8 as
                            *const i8,
                        counter_value /
                            (elapsed_time + 0.001f64),
                        b"nps\x00" as *const u8 as *const i8);
        }
    }
}

fn display_status_out() {
    unsafe { display_status(stdout, 0 as i32); }
}

fn echo_ponder_move_4(curr_move: i32, ponder_move: i32) {
    unsafe {
        send_status(b"-->   %s        \x00" as *const u8 as
                        *const i8,
                    b"Thor database\x00" as *const u8 as
                        *const i8);
        if ponder_move != 0 {
            send_status(b"{%c%c} \x00" as *const u8 as
                            *const i8,
                        'a' as i32 +
                            ponder_move % 10 as i32 -
                            1 as i32,
                        '0' as i32 +
                            ponder_move / 10 as i32);
        }
        send_status(b"%c%c\x00" as *const u8 as *const i8,
                    'a' as i32 + curr_move % 10 as i32 -
                        1 as i32,
                    '0' as i32 + curr_move / 10 as i32);
        display_status(stdout, 0 as i32);
    }
}

fn echo_ponder_move_2(curr_move: i32, ponder_move: i32) {
    unsafe {
        send_status(b"-->   %s        \x00" as *const u8 as
                        *const i8,
                    b"Thor database\x00" as *const u8 as
                        *const i8);
        if ponder_move != 0 {
            send_status(b"{%c%c} \x00" as *const u8 as
                            *const i8,
                        'a' as i32 +
                            ponder_move % 10 as i32
                            - 1 as i32,
                        '0' as i32 +
                            ponder_move /
                                10 as i32);
        }
        send_status(b"%c%c\x00" as *const u8 as
                        *const i8,
                    'a' as i32 + curr_move % 10 as i32 -
                        1 as i32,
                    '0' as i32 + curr_move / 10 as i32);
        display_status(stdout, 0 as i32);
    }
}

fn echo_ponder_move(curr_move: i32, ponder_move: i32) {
    unsafe {
        send_status(b"-->   Forced opening move        \x00" as
            *const u8 as *const i8);
        if ponder_move != 0 {
            send_status(b"{%c%c} \x00" as *const u8 as
                            *const i8,
                        'a' as i32 +
                            ponder_move % 10 as i32 -
                            1 as i32,
                        '0' as i32 +
                            ponder_move / 10 as i32);
        }
        send_status(b"%c%c\x00" as *const u8 as *const i8,
                    'a' as i32 + curr_move % 10 as i32 -
                        1 as i32,
                    '0' as i32 + curr_move / 10 as i32);
        display_status(stdout, 0 as i32);
    }
}

fn echo_compute_move_2(info: &EvaluationType, disk: i32) {
    unsafe {
        let eval_str = produce_eval_text(info, 0 as i32);
        send_status(b"-->         \x00" as *const u8 as
            *const i8);
        send_status(b"%-8s  \x00" as *const u8 as *const i8,
                    eval_str);
        free(eval_str as *mut std::ffi::c_void);
        send_status(b"%c%c \x00" as *const u8 as *const i8,
                    'a' as i32 +
                        disk %
                            10 as i32 - 1 as i32,
                    '0' as i32 +
                        disk /
                            10 as i32);
        display_status(stdout, 0 as i32);
    }
}

fn echo_compute_move_1(info: &EvaluationType) {
    unsafe {
        let eval_str = produce_eval_text(info, 0 as i32);
        send_status(b"-->         \x00" as *const u8 as
            *const i8);
        send_status(b"%-8s  \x00" as *const u8 as *const i8,
                    eval_str);
        display_status(stdout, 0 as i32);
        free(eval_str as *mut std::ffi::c_void);
    }
}
}
impl ComputeMoveLogger for LogFileHandler {
fn create(log_file_path_: &mut [i8]) -> Option<Self> {
    let log_file = unsafe {
        fopen(log_file_path_.as_mut_ptr(),
              b"a\x00" as *const u8 as *const i8)
    };
    if !log_file.is_null() {
        let logger = LogFileHandler { log_file };
        Some(logger)
    } else {
        None
    }
}

fn create_log_file_if_needed() -> Option<Self> {
    unsafe {
        if use_log_file != 0 {
            let log_file_path_ = &mut log_file_path as &mut [i8];
            Self::create(log_file_path_)
        } else {
            None
        }
    }
}

fn log_moves_generated(logger: &mut LogFileHandler, moves_generated: i32, move_list_for_disks_played: &[i32; 64]) {
    unsafe {
        fprintf(logger.log_file, b"%d %s: \x00" as *const u8 as *const i8,
                moves_generated,
                b"moves generated\x00" as *const u8 as *const i8);
        let mut i = 0 as i32;
        while i < moves_generated {
            fprintf(logger.log_file,
                    b"%c%c \x00" as *const u8 as *const i8,
                    'a' as i32 +
                        move_list_for_disks_played[i as usize] %
                            10 as i32 - 1 as i32,
                    '0' as i32 +
                        move_list_for_disks_played[i as usize] /
                            10 as i32);
            i += 1
        }
        fputs(b"\n\x00" as *const u8 as *const i8, logger.log_file);
    }
}

fn log_best_move_pass(logger: &mut LogFileHandler) {
   unsafe{ fprintf(logger.log_file,
            b"%s: %s\n\x00" as *const u8 as *const i8,
            b"Best move\x00" as *const u8 as *const i8,
            b"pass\x00" as *const u8 as *const i8);
     fclose(logger.log_file);
   }
}

fn log_best_move(logger: &mut LogFileHandler, best_move: i32) {
    unsafe {
        fprintf(logger.log_file,
                b"%s: %c%c  (%s)\n\x00" as *const u8 as
                    *const i8,
                b"Best move\x00" as *const u8 as *const i8,
                'a' as i32 +
                    best_move %
                        10 as i32 - 1 as i32,
                '0' as i32 +
                    best_move /
                        10 as i32,
                b"forced\x00" as *const u8 as *const i8);
        fclose(logger.log_file);
    }
}

fn log_chosen_move(logger: &mut LogFileHandler, curr_move: i32, info: &EvaluationType) {
    unsafe {
        let eval_str = produce_eval_text(info, 0 as i32);
        fprintf(logger.log_file,
                b"%s: %c%c  %s\n\x00" as *const u8 as *const i8,
                b"Move chosen\x00" as *const u8 as *const i8,
                'a' as i32 + curr_move % 10 as i32 -
                    1 as i32,
                '0' as i32 + curr_move / 10 as i32, eval_str);
        free(eval_str as *mut std::ffi::c_void);
    }
}

fn log_status(logger: &mut LogFileHandler) {
    unsafe { display_status(logger.log_file, 1 as i32); }
}

fn log_optimal_line(logger: &mut LogFileHandler) {
    unsafe { display_optimal_line(logger.log_file); }
}

fn close_logger(logger: &mut LogFileHandler) {
    if !logger.log_file.is_null() { unsafe { fclose(logger.log_file); } }
}

fn log_board(logger: &mut LogFileHandler, board_: &mut [i32; 128], side_to_move_: i32) {
    unsafe {
        display_board(logger.log_file, board_.as_mut_ptr(), side_to_move_,
                      0 as i32, 0 as i32, 0 as i32);
    }
}
}