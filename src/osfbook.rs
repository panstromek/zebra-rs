use crate::{
    src::{
        display::{send_status},
        game::{global_setup, game_init},
        error::fatal_error,
        zebra::{ _IO_FILE}
    }
};
use engine::src::osfbook::*;
use engine::src::display::{display_pv, current_row, black_player, black_time, black_eval, white_eval, white_time, white_player, echo};
use crate::src::display::display_board;
use engine::src::midgame::{middle_game, toggle_midgame_abort_check, toggle_midgame_hash_usage};
use engine::src::myrandom::{my_srandom, my_random};
use engine::src::hash::{set_hash_transformation, determine_hash_values, setup_hash};
use engine::src::error::{FrontEnd};
use crate::src::error::{LibcFatalError};
use engine::src::globals::{white_moves, black_moves, board, pv, piece_count};
use engine::src::moves::{disks_played, unmake_move, make_move, move_list, move_count, generate_all, generate_specific, unmake_move_no_hash, make_move_no_hash};
use engine::src::stubs::{abs, floor};
use engine::src::search::{root_eval, nodes, disc_count, get_ponder_move};
use engine::src::end::end_game;
use engine::src::counter::reset_counter;
use engine::src::zebra::EvaluationType;
use engine::src::timer::toggle_abort_check;
use engine::src::safemem::safe_malloc;
use engine::src::eval::toggle_experimental;
use crate::src::stubs::{fclose, fprintf, fopen, puts, printf, time, fflush, putc, fputs, sprintf, free, fputc, strstr, toupper, __ctype_b_loc, strlen, sscanf, fgets, ctime, strcpy, malloc, feof, strcmp, fwrite, fread, fscanf, qsort, stdout, stderr, exit};

pub type FE = LibcFatalError;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type time_t = __time_t;
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const std::ffi::c_void,
                                _: *const std::ffi::c_void) -> i32>;

/*
   MINIMAX_TREE
   Calculates the minimax values of all nodes in the tree.
*/

pub unsafe fn minimax_tree() {
    let mut i: i32 = 0;
    let mut dummy_black_score: i32 = 0;
    let mut dummy_white_score: i32 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    printf(b"Calculating minimax value... \x00" as *const u8 as
               *const i8);
    fflush(stdout);
    prepare_tree_traversal();
    time(&mut start_time);
    /* Mark all nodes as not traversed */
    i = 0 as i32;
    while i < book_node_count {
        let ref mut fresh15 = (*node.offset(i as isize)).flags;
        *fresh15 =
            (*fresh15 as i32 | 8 as i32) as u16;
        i += 1
    }
    do_minimax(0 as i32, &mut dummy_black_score,
               &mut dummy_white_score);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
}

pub unsafe fn report_do_evaluate(evaluation_stage_: i32) {
    putc('|' as i32, stdout);
    if evaluation_stage_ % 5 as i32 == 0 as i32 {
        printf(b" %d%% \x00" as *const u8 as *const i8,
               4 as i32 * evaluation_stage_);
    }
    fflush(stdout);
}
/*
   EVALUATE_TREE
   Finds the most promising deviations from all nodes in the tree.
*/

pub unsafe fn evaluate_tree() {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    prepare_tree_traversal();
    exhausted_node_count = 0 as i32;
    evaluated_count = 0 as i32;
    evaluation_stage = 0 as i32;
    time(&mut start_time);
    let feasible_count = compute_feasible_count();
    max_eval_count =
        if feasible_count < max_batch_size {
            feasible_count
        } else { max_batch_size };
    printf(b"Evaluating to depth %d. \x00" as *const u8 as
               *const i8, search_depth);
    if min_eval_span > 0 as i32 ||
           max_eval_span < 1000 as i32 * 128 as i32 {
        printf(b"Eval interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const i8,
               min_eval_span as f64 / 128.0f64,
               max_eval_span as f64 / 128.0f64);
    }
    if min_negamax_span > 0 as i32 ||
           max_negamax_span < 1000 as i32 * 128 as i32 {
        printf(b"Negamax interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const i8,
               min_negamax_span as f64 / 128.0f64,
               max_negamax_span as f64 / 128.0f64);
    }
    if max_eval_count == feasible_count {
        printf(b"\n%d relevant nodes.\x00" as *const u8 as
                   *const i8, feasible_count);
    } else {
        printf(b"\nMax batch size is %d.\x00" as *const u8 as
                   *const i8, max_batch_size);
    }
    puts(b"\x00" as *const u8 as *const i8);
    printf(b"Progress: \x00" as *const u8 as *const i8);
    fflush(stdout);
    if feasible_count > 0 as i32 { do_evaluate::<LibcFatalError>(0 as i32); }
    time(&mut stop_time);
    printf(b"(took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    printf(b"%d nodes evaluated \x00" as *const u8 as *const i8,
           evaluated_count);
    printf(b"(%d exhausted nodes ignored)\n\x00" as *const u8 as
               *const i8, exhausted_node_count);
    puts(b"\x00" as *const u8 as *const i8);
}
/*
   EXAMINE_TREE
   Generates some statistics about the book tree.
*/

pub unsafe fn examine_tree() {
    let mut i: i32 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    printf(b"Examining tree... \x00" as *const u8 as *const i8);
    fflush(stdout);
    prepare_tree_traversal();
    time(&mut start_time);
    i = 0 as i32;
    while i <= 60 as i32 {
        exact_count[i as usize] = 0 as i32;
        wld_count[i as usize] = 0 as i32;
        exhausted_count[i as usize] = 0 as i32;
        common_count[i as usize] = 0 as i32;
        i += 1
    }
    unreachable_count = 0 as i32;
    leaf_count = 0 as i32;
    bad_leaf_count = 0 as i32;
    /* Mark all nodes as not traversed and examine the tree */
    i = 0 as i32;
    while i < book_node_count {
        let ref mut fresh22 = (*node.offset(i as isize)).flags;
        *fresh22 =
            (*fresh22 as i32 | 8 as i32) as u16;
        i += 1
    }
    do_examine(0 as i32);
    /* Any nodes not reached by the walkthrough? */
    i = 0 as i32;
    while i < book_node_count {
        if (*node.offset(i as isize)).flags as i32 & 8 as i32
               != 0 {
            unreachable_count += 1;
            let ref mut fresh23 = (*node.offset(i as isize)).flags;
            *fresh23 =
                (*fresh23 as i32 ^ 8 as i32) as u16
        }
        i += 1
    }
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
}
unsafe extern "C" fn int_compare(i1: *const std::ffi::c_void,
                                 i2: *const std::ffi::c_void) -> i32 {
    return *(i1 as *mut i32) - *(i2 as *mut i32);
}
/*
   BOOK_STATISTICS
   Describe the status of the nodes in the tree.
*/

pub unsafe fn book_statistics(full_statistics: i32) {
    let strata: [f64; 11] =
        [0.01f64, 0.02f64, 0.03f64, 0.05f64, 0.10f64, 0.30f64, 0.50f64,
         0.70f64, 0.90f64, 0.99f64, 1.01f64];
    let mut eval_strata: [f64; 10] = [0.; 10];
    let mut negamax_strata: [f64; 10] = [0.; 10];
    let mut i: i32 = 0;
    let mut full_solved: i32 = 0;
    let mut wld_solved: i32 = 0;
    let mut unevaluated: i32 = 0;
    let mut eval_count: i32 = 0;
    let mut negamax_count: i32 = 0;
    let mut private_count: i32 = 0;
    let mut this_strata: i32 = 0;
    let mut strata_shift: i32 = 0;
    let mut first: i32 = 0;
    let mut last: i32 = 0;
    let mut evals = 0 as *mut i32;
    let mut negamax = 0 as *mut i32;
    let mut depth: [i32; 60] = [0; 60];
    let mut total_count: [i32; 61] = [0; 61];
    evals =
        safe_malloc::<FE>((book_node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                         as u64)) as
            *mut i32;
    negamax =
        safe_malloc::<FE>((book_node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                         as u64)) as
            *mut i32;
    wld_solved = 0 as i32;
    full_solved = wld_solved;
    eval_count = 0 as i32;
    negamax_count = 0 as i32;
    private_count = 0 as i32;
    unevaluated = 0 as i32;
    i = 0 as i32;
    while i < 60 as i32 {
        depth[i as usize] = 0 as i32;
        i += 1
    }
    i = 0 as i32;
    while i < book_node_count {
        if (*node.offset(i as isize)).flags as i32 & 16 as i32
               != 0 {
            full_solved += 1
        } else if (*node.offset(i as isize)).flags as i32 &
                      4 as i32 != 0 {
            wld_solved += 1
        } else {
            depth[get_node_depth(i) as usize] += 1;
            if (*node.offset(i as isize)).alternative_score as i32 ==
                   9999 as i32 &&
                   (*node.offset(i as isize)).best_alternative_move as
                       i32 == -(1 as i32) {
                unevaluated += 1
            } else {
                if (*node.offset(i as isize)).alternative_score as i32
                       != 9999 as i32 {
                    let fresh24 = eval_count;
                    eval_count = eval_count + 1;
                    *evals.offset(fresh24 as isize) =
                        abs((*node.offset(i as isize)).alternative_score as
                                i32)
                }
                let fresh25 = negamax_count;
                negamax_count = negamax_count + 1;
                *negamax.offset(fresh25 as isize) =
                    abs((*node.offset(i as isize)).black_minimax_score as
                            i32)
            }
        }
        if (*node.offset(i as isize)).flags as i32 & 32 as i32
               != 0 {
            private_count += 1
        }
        i += 1
    }
    qsort(evals as *mut std::ffi::c_void, eval_count as size_t,
          ::std::mem::size_of::<i32>() as u64,
          Some(int_compare as
                   unsafe extern "C" fn(_: *const std::ffi::c_void,
                                        _: *const std::ffi::c_void)
                       -> i32));
    qsort(negamax as *mut std::ffi::c_void, negamax_count as size_t,
          ::std::mem::size_of::<i32>() as u64,
          Some(int_compare as
                   unsafe extern "C" fn(_: *const std::ffi::c_void,
                                        _: *const std::ffi::c_void)
                       -> i32));
    puts(b"\x00" as *const u8 as *const i8);
    printf(b"#nodes:       %d\x00" as *const u8 as *const i8,
           book_node_count);
    if private_count > 0 as i32 {
        printf(b"  (%d private)\x00" as *const u8 as *const i8,
               private_count);
    }
    puts(b"\x00" as *const u8 as *const i8);
    printf(b"#full solved: %d\n\x00" as *const u8 as *const i8,
           full_solved);
    printf(b"#WLD solved:  %d\n\x00" as *const u8 as *const i8,
           wld_solved);
    printf(b"#unevaluated: %d\n\n\x00" as *const u8 as *const i8,
           unevaluated);
    i = 0 as i32;
    while i <= 59 as i32 {
        if depth[i as usize] > 0 as i32 {
            printf(b"#nodes with %2d-ply deviations: %d\n\x00" as *const u8 as
                       *const i8, i, depth[i as usize]);
        }
        i += 1
    }
    puts(b"\x00" as *const u8 as *const i8);
    this_strata = 0 as i32;
    strata_shift =
        floor(strata[this_strata as usize] * eval_count as f64) as
            i32;
    i = 0 as i32;
    while i < eval_count {
        if i == strata_shift {
            eval_strata[this_strata as usize] =
                *evals.offset(i as isize) as f64 / 128.0f64;
            this_strata += 1;
            strata_shift =
                floor(strata[this_strata as usize] *
                          eval_count as f64) as i32
        }
        i += 1
    }
    this_strata = 0 as i32;
    strata_shift =
        floor(strata[this_strata as usize] * negamax_count as f64)
            as i32;
    i = 0 as i32;
    while i < negamax_count {
        if i == strata_shift {
            negamax_strata[this_strata as usize] =
                *evals.offset(i as isize) as f64 / 128.0f64;
            this_strata += 1;
            strata_shift =
                floor(strata[this_strata as usize] *
                          negamax_count as f64) as i32
        }
        i += 1
    }
    i = 0 as i32;
    while i < 10 as i32 {
        printf(b"%2.0f%%:  \x00" as *const u8 as *const i8,
               100 as i32 as f64 * strata[i as usize]);
        printf(b"%5.2f   \x00" as *const u8 as *const i8,
               eval_strata[i as usize]);
        printf(b"%5.2f   \x00" as *const u8 as *const i8,
               negamax_strata[i as usize]);
        puts(b"\x00" as *const u8 as *const i8);
        i += 1
    }
    puts(b"\x00" as *const u8 as *const i8);
    free(negamax as *mut std::ffi::c_void);
    free(evals as *mut std::ffi::c_void);
    if full_statistics != 0 {
        examine_tree();
        first = 61 as i32;
        last = -(1 as i32);
        i = 0 as i32;
        while i <= 60 as i32 {
            total_count[i as usize] =
                exact_count[i as usize] + wld_count[i as usize] +
                    exhausted_count[i as usize] + common_count[i as usize];
            if total_count[i as usize] > 0 as i32 {
                first = if first < i { first } else { i };
                last = if last > i { last } else { i }
            }
            i += 1
        }
        printf(b"%d unreachable nodes\n\n\x00" as *const u8 as
                   *const i8, unreachable_count);
        printf(b"%d leaf nodes; %d lack exact score and %d lack WLD status\n\x00"
                   as *const u8 as *const i8, leaf_count,
               bad_leaf_count, really_bad_leaf_count);
        i = first;
        while i <= last {
            printf(b"%2d moves\x00" as *const u8 as *const i8, i);
            printf(b"   \x00" as *const u8 as *const i8);
            printf(b"%5d node\x00" as *const u8 as *const i8,
                   total_count[i as usize]);
            if total_count[i as usize] == 1 as i32 {
                printf(b" :  \x00" as *const u8 as *const i8);
            } else {
                printf(b"s:  \x00" as *const u8 as *const i8);
            }
            if common_count[i as usize] > 0 as i32 {
                printf(b"%5d midgame\x00" as *const u8 as *const i8,
                       common_count[i as usize]);
            } else {
                printf(b"             \x00" as *const u8 as
                           *const i8);
            }
            printf(b"  \x00" as *const u8 as *const i8);
            if wld_count[i as usize] > 0 as i32 {
                printf(b"%5d WLD\x00" as *const u8 as *const i8,
                       wld_count[i as usize]);
            } else {
                printf(b"         \x00" as *const u8 as *const i8);
            }
            printf(b"  \x00" as *const u8 as *const i8);
            if exact_count[i as usize] > 0 as i32 {
                printf(b"%5d exact\x00" as *const u8 as *const i8,
                       exact_count[i as usize]);
            } else {
                printf(b"           \x00" as *const u8 as
                           *const i8);
            }
            printf(b"  \x00" as *const u8 as *const i8);
            if exhausted_count[i as usize] > 0 as i32 {
                printf(b"%2d exhausted\x00" as *const u8 as
                           *const i8, exhausted_count[i as usize]);
            }
            puts(b"\x00" as *const u8 as *const i8);
            i += 1
        }
        puts(b"\x00" as *const u8 as *const i8);
    };
}
/*
   DISPLAY_OPTIMAL_LINE
   Outputs the sequence of moves which is optimal according
   to both players.
*/

pub unsafe fn display_doubly_optimal_line(original_side_to_move:
                                                         i32) {
    let mut i: i32 = 0;
    let mut done: i32 = 0;
    let mut show_move: i32 = 0;
    let mut line: i32 = 0;
    let mut root_score: i32 = 0;
    let mut child_score: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut base_orientation: i32 = 0;
    let mut child_orientation: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut current: i32 = 0;
    let mut child: i32 = 0;
    let mut next: i32 = 0;
    prepare_tree_traversal();
    printf(b"Root evaluation with Zebra playing \x00" as *const u8 as
               *const i8);
    if original_side_to_move == 0 as i32 {
        root_score =
            (*node.offset(0 as i32 as isize)).black_minimax_score as
                i32;
        printf(b"black\x00" as *const u8 as *const i8);
    } else {
        root_score =
            (*node.offset(0 as i32 as isize)).white_minimax_score as
                i32;
        printf(b"white\x00" as *const u8 as *const i8);
    }
    printf(b": %+.2f\n\x00" as *const u8 as *const i8,
           root_score as f64 / 128.0f64);
    current = 0 as i32;
    puts(b"Preferred line: \x00" as *const u8 as *const i8);
    line = 0 as i32;
    done = 0 as i32;
    show_move = 1 as i32;
    while (*node.offset(current as isize)).flags as i32 &
              (16 as i32 | 4 as i32) == 0 && done == 0 {
        if (*node.offset(current as isize)).flags as i32 &
               1 as i32 != 0 {
            side_to_move = 0 as i32
        } else { side_to_move = 2 as i32 }
        generate_all(side_to_move);
        next = -(1 as i32);
        this_move = -(1 as i32);
        i = 0 as i32;
        while i < move_count[disks_played as usize] {
            get_hash(&mut val1, &mut val2, &mut base_orientation);
            this_move = move_list[disks_played as usize][i as usize];
            make_move(side_to_move, this_move, 1 as i32);
            get_hash(&mut val1, &mut val2, &mut child_orientation);
            slot = probe_hash_table(val1, val2);
            child = *book_hash_table.offset(slot as isize);
            if child != -(1 as i32) {
                if original_side_to_move == 0 as i32 {
                    child_score =
                        (*node.offset(child as isize)).black_minimax_score as
                            i32
                } else {
                    child_score =
                        (*node.offset(child as isize)).white_minimax_score as
                            i32
                }
                if child_score == root_score { next = child }
            }
            if child != -(1 as i32) && next == child { break ; }
            unmake_move(side_to_move, this_move);
            i += 1
        }
        if next == -(1 as i32) {
            done = 1 as i32;
            if adjust_score((*node.offset(current as isize)).alternative_score
                                as i32, side_to_move) != root_score {
                puts(b"(failed to find continuation)\x00" as *const u8 as
                         *const i8);
                show_move = 0 as i32
            } else {
                this_move =
                    (*node.offset(current as isize)).best_alternative_move as
                        i32;
                this_move =
                    *inv_symmetry_map[base_orientation as
                                          usize].offset(this_move as isize)
            }
        }
        if show_move != 0 {
            if side_to_move == 0 as i32 {
                line += 1;
                printf(b"%2d. \x00" as *const u8 as *const i8,
                       line);
            }
            printf(b"%c%c  \x00" as *const u8 as *const i8,
                   'a' as i32 + this_move % 10 as i32 -
                       1 as i32,
                   '0' as i32 + this_move / 10 as i32);
            if side_to_move == 2 as i32 {
                puts(b"\x00" as *const u8 as *const i8);
            }
            if done != 0 {
                puts(b"(deviation)\x00" as *const u8 as *const i8);
            }
        }
        current = next
    }
    puts(b"\x00" as *const u8 as *const i8);
}
/*
  ADD_NEW_GAME
  Adds a new game to the game tree.
*/

pub unsafe fn add_new_game(move_count_0: i32,
                                      game_move_list: *mut i16,
                                      min_empties: i32,
                                      max_full_solve: i32,
                                      max_wld_solve: i32,
                                      update_path: i32,
                                      private_game: i32) {
    let mut dummy_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut stored_echo: i32 = 0;
    let mut dummy_black_score: i32 = 0;
    let mut dummy_white_score: i32 = 0;
    let mut force_eval: i32 = 0;
    let mut midgame_eval_done: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut slot: i32 = 0;
    let mut this_node: i32 = 0;
    let mut last_move_number: i32 = 0;
    let mut first_new_node: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut outcome: i32 = 0;
    let mut visited_node: [i32; 61] = [0; 61];
    let mut flags: [u16; 61] = [0; 61];
    stored_echo = echo;
    echo = 0 as i32;
    /* First create new nodes for new positions */
    prepare_tree_traversal();
    i = 0 as i32;
    while i < move_count_0 {
        if *game_move_list.offset(i as isize) as i32 >
               0 as i32 {
            flags[i as usize] = 1 as i32 as u16
        } else { flags[i as usize] = 2 as i32 as u16 }
        i += 1
    }
    flags[move_count_0 as usize] = 0 as i32 as u16;
    first_new_node = 61 as i32;
    this_node = 0 as i32;
    side_to_move = 0 as i32;
    last_move_number =
        if move_count_0 < 60 as i32 - min_empties {
            move_count_0
        } else { (60 as i32) - min_empties };
    i = 0 as i32;
    while i <= last_move_number {
        /* Look for the position in the hash table */
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        if slot == -(1 as i32) ||
               *book_hash_table.offset(slot as isize) == -(1 as i32) {
            this_node = create_BookNode::<FE>(val1, val2, flags[i as usize]);
            if private_game != 0 {
                let ref mut fresh26 =
                    (*node.offset(this_node as isize)).flags;
                *fresh26 =
                    (*fresh26 as i32 | 32 as i32) as
                        u16
            }
            if i < first_new_node { first_new_node = i }
        } else { this_node = *book_hash_table.offset(slot as isize) }
        visited_node[i as usize] = this_node;
        /* Make the moves of the game until the cutoff point */
        if i < last_move_number {
            this_move =
                abs(*game_move_list.offset(i as isize) as i32);
            if *game_move_list.offset(i as isize) as i32 >
                   0 as i32 {
                side_to_move = 0 as i32
            } else { side_to_move = 2 as i32 }
            if generate_specific(this_move, side_to_move) == 0 {
                puts(b"\x00" as *const u8 as *const i8);
                printf(b"i=%d, side_to_move=%d, this_move=%d\n\x00" as
                           *const u8 as *const i8, i, side_to_move,
                       this_move);
                printf(b"last_move_number=%d, move_count=%d\n\x00" as
                           *const u8 as *const i8, last_move_number,
                       move_count_0);
                j = 0 as i32;
                while j < move_count_0 {
                    printf(b"%3d \x00" as *const u8 as *const i8,
                           *game_move_list.offset(j as isize) as i32);
                    j += 1
                }
                fatal_error(b"%s: %d\n\x00" as *const u8 as
                                *const i8,
                            b"Invalid move generated\x00" as *const u8 as
                                *const i8, this_move);
            }
            make_move(side_to_move, this_move, 1 as i32);
        } else {
            /* No more move to make, only update the player to move */
            side_to_move = 0 as i32 + 2 as i32 - side_to_move
        }
        i += 1
    }
    if last_move_number == move_count_0 {
        /* No cutoff applies */
        let mut black_count: i32 = 0;
        let mut white_count: i32 = 0;
        black_count = disc_count(0 as i32);
        white_count = disc_count(2 as i32);
        if black_count > white_count {
            outcome = 64 as i32 - 2 as i32 * white_count
        } else if white_count > black_count {
            outcome = 2 as i32 * black_count - 64 as i32
        } else { outcome = 0 as i32 }
    } else {
        generate_all(side_to_move);
        determine_hash_values(side_to_move, board.as_mut_ptr());
        if echo != 0 {
            puts(b"\x00" as *const u8 as *const i8);
            if side_to_move == 0 as i32 {
                printf(b"Full solving with %d empty (black)\n\x00" as
                           *const u8 as *const i8,
                       60 as i32 - disks_played);
            } else {
                printf(b"Full solving with %d empty (white)\n\x00" as
                           *const u8 as *const i8,
                       60 as i32 - disks_played);
            }
        }
       end_game::<FE>(side_to_move, 0 as i32, 0 as i32,
                 1 as i32, 0 as i32, &mut dummy_info);
        outcome = root_eval;
        if side_to_move == 2 as i32 { outcome = -outcome }
    }
    (*node.offset(this_node as isize)).black_minimax_score =
        outcome as i16;
    (*node.offset(this_node as isize)).white_minimax_score =
        outcome as i16;
    if outcome > 0 as i32 {
        let ref mut fresh27 =
            (*node.offset(this_node as isize)).black_minimax_score;
        *fresh27 =
            (*fresh27 as i32 + 30000 as i32) as i16;
        let ref mut fresh28 =
            (*node.offset(this_node as isize)).white_minimax_score;
        *fresh28 =
            (*fresh28 as i32 + 30000 as i32) as i16
    }
    if outcome < 0 as i32 {
        let ref mut fresh29 =
            (*node.offset(this_node as isize)).black_minimax_score;
        *fresh29 =
            (*fresh29 as i32 - 30000 as i32) as i16;
        let ref mut fresh30 =
            (*node.offset(this_node as isize)).white_minimax_score;
        *fresh30 =
            (*fresh30 as i32 - 30000 as i32) as i16
    }
    let ref mut fresh31 = (*node.offset(this_node as isize)).flags;
    *fresh31 =
        (*fresh31 as i32 | 16 as i32) as u16;
    /* Take another pass through the midgame to update move
       alternatives and minimax information if requested. */
    if update_path != 0 {
        prepare_tree_traversal();
        i = 0 as i32;
        while i < last_move_number {
            this_move =
                abs(*game_move_list.offset(i as isize) as i32);
            if *game_move_list.offset(i as isize) as i32 >
                   0 as i32 {
                side_to_move = 0 as i32
            } else { side_to_move = 2 as i32 }
            if generate_specific(this_move, side_to_move) == 0 {
                fatal_error(b"%s: %d\n\x00" as *const u8 as
                                *const i8,
                            b"Invalid move generated\x00" as *const u8 as
                                *const i8, this_move);
            }
            make_move(side_to_move, this_move, 1 as i32);
            i += 1
        }
        if echo != 0 { fflush(stdout); }
        midgame_eval_done = 0 as i32;
        i = last_move_number - 1 as i32;
        while i >= 0 as i32 {
            this_move =
                abs(*game_move_list.offset(i as isize) as i32);
            if *game_move_list.offset(i as isize) as i32 >
                   0 as i32 {
                side_to_move = 0 as i32
            } else { side_to_move = 2 as i32 }
            unmake_move(side_to_move, this_move);
            /* If the game was public, make sure that all nodes that
            previously marked as private nodes are marked as public. */
            this_node = visited_node[i as usize];
            if private_game == 0 &&
                   (*node.offset(this_node as isize)).flags as i32 &
                       32 as i32 != 0 {
                let ref mut fresh32 =
                    (*node.offset(this_node as isize)).flags;
                *fresh32 =
                    (*fresh32 as i32 ^ 32 as i32) as
                        u16
            }
            if (*node.offset(this_node as isize)).flags as i32 &
                   1 as i32 != 0 {
                side_to_move = 0 as i32
            } else { side_to_move = 2 as i32 }
            generate_all(side_to_move);
            determine_hash_values(side_to_move, board.as_mut_ptr());
            if disks_played >= 60 as i32 - max_full_solve {
                /* Only solve the position if it hasn't been solved already */
                if (*node.offset(this_node as isize)).flags as i32 &
                       16 as i32 == 0 {
                   end_game::<FE>(side_to_move, 0 as i32, 0 as i32,
                             1 as i32, 0 as i32,
                             &mut dummy_info);
                    if side_to_move == 0 as i32 {
                        outcome = root_eval
                    } else { outcome = -root_eval }
                    (*node.offset(this_node as isize)).black_minimax_score =
                        outcome as i16;
                    (*node.offset(this_node as isize)).white_minimax_score =
                        outcome as i16;
                    if outcome > 0 as i32 {
                        let ref mut fresh33 =
                            (*node.offset(this_node as
                                              isize)).black_minimax_score;
                        *fresh33 =
                            (*fresh33 as i32 + 30000 as i32)
                                as i16;
                        let ref mut fresh34 =
                            (*node.offset(this_node as
                                              isize)).white_minimax_score;
                        *fresh34 =
                            (*fresh34 as i32 + 30000 as i32)
                                as i16
                    }
                    if outcome < 0 as i32 {
                        let ref mut fresh35 =
                            (*node.offset(this_node as
                                              isize)).black_minimax_score;
                        *fresh35 =
                            (*fresh35 as i32 - 30000 as i32)
                                as i16;
                        let ref mut fresh36 =
                            (*node.offset(this_node as
                                              isize)).white_minimax_score;
                        *fresh36 =
                            (*fresh36 as i32 - 30000 as i32)
                                as i16
                    }
                    let ref mut fresh37 =
                        (*node.offset(this_node as isize)).flags;
                    *fresh37 =
                        (*fresh37 as i32 | 16 as i32) as
                            u16
                }
            } else if disks_played >= 60 as i32 - max_wld_solve {
                /* Only solve the position if its WLD status is unknown */
                if (*node.offset(this_node as isize)).flags as i32 &
                       4 as i32 == 0 {
                   end_game::<FE>(side_to_move, 1 as i32, 0 as i32,
                             1 as i32, 0 as i32,
                             &mut dummy_info);
                    if side_to_move == 0 as i32 {
                        outcome = root_eval
                    } else { outcome = -root_eval }
                    (*node.offset(this_node as isize)).black_minimax_score =
                        outcome as i16;
                    (*node.offset(this_node as isize)).white_minimax_score =
                        outcome as i16;
                    if outcome > 0 as i32 {
                        let ref mut fresh38 =
                            (*node.offset(this_node as
                                              isize)).black_minimax_score;
                        *fresh38 =
                            (*fresh38 as i32 + 30000 as i32)
                                as i16;
                        let ref mut fresh39 =
                            (*node.offset(this_node as
                                              isize)).white_minimax_score;
                        *fresh39 =
                            (*fresh39 as i32 + 30000 as i32)
                                as i16
                    }
                    if outcome < 0 as i32 {
                        let ref mut fresh40 =
                            (*node.offset(this_node as
                                              isize)).black_minimax_score;
                        *fresh40 =
                            (*fresh40 as i32 - 30000 as i32)
                                as i16;
                        let ref mut fresh41 =
                            (*node.offset(this_node as
                                              isize)).white_minimax_score;
                        *fresh41 =
                            (*fresh41 as i32 - 30000 as i32)
                                as i16
                    }
                    let ref mut fresh42 =
                        (*node.offset(this_node as isize)).flags;
                    *fresh42 =
                        (*fresh42 as i32 | 4 as i32) as
                            u16
                }
            } else {
                force_eval =
                    (i >= first_new_node - 1 as i32 ||
                         (*node.offset(this_node as
                                           isize)).best_alternative_move as
                             i32 ==
                             abs(*game_move_list.offset(i as isize) as
                                     i32)) as i32;
                if midgame_eval_done == 0 {
                    printf(b"Evaluating: \x00" as *const u8 as
                               *const i8);
                    fflush(stdout);
                }
                midgame_eval_done = 1 as i32;
                if force_eval != 0 { clear_node_depth(this_node); }
                evaluate_node::<LibcFatalError>(this_node);
                printf(b"|\x00" as *const u8 as *const i8);
                fflush(stdout);
            }
            let ref mut fresh43 = (*node.offset(this_node as isize)).flags;
            *fresh43 =
                (*fresh43 as i32 | 8 as i32) as
                    u16;
            do_minimax(this_node, &mut dummy_black_score,
                       &mut dummy_white_score);
            if (*node.offset(this_node as isize)).flags as i32 &
                   4 as i32 == 0 &&
                   (*node.offset(this_node as isize)).best_alternative_move as
                       i32 == -(1 as i32) &&
                   (*node.offset(this_node as isize)).alternative_score as
                       i32 == 9999 as i32 {
                /* Minimax discovered that the node hasn't got a deviation any
                   longer because that move has been played. */
                evaluate_node::<FE>(this_node);
                printf(b"-|-\x00" as *const u8 as *const i8);
                do_minimax(this_node, &mut dummy_black_score,
                           &mut dummy_white_score);
            }
            i -= 1
        }
        puts(b"\x00" as *const u8 as *const i8);
    }
    echo = stored_echo;
    total_game_count += 1;
}
/*
   BUILD_TREE
   Reads games from the file pointed to by FILE_NAME and
   incorporates them into the game tree.
*/

pub unsafe fn build_tree(file_name: *const i8,
                                    max_game_count: i32,
                                    max_diff: i32,
                                    min_empties: i32) {
    let mut move_string: [i8; 200] = [0; 200];
    let mut line_buffer: [i8; 1000] = [0; 1000];
    let mut sign: i8 = 0;
    let mut column: i8 = 0;
    let mut row: i8 = 0;
    let mut i: i32 = 0;
    let mut games_parsed: i32 = 0;
    let mut games_imported: i32 = 0;
    let mut move_count_0: i32 = 0;
    let mut diff: i32 = 0;
    let mut game_move_list: [i16; 60] = [0; 60];
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut stream = 0 as *mut FILE;
    puts(b"Importing game list...\x00" as *const u8 as *const i8);
    fflush(stdout);
    stream = fopen(file_name, b"r\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Could not open game file\x00" as *const u8 as
                        *const i8, file_name);
    }
    time(&mut start_time);
    games_parsed = 0 as i32;
    games_imported = 0 as i32;
    loop  {
        fgets(line_buffer.as_mut_ptr(), 998 as i32, stream);
        sscanf(line_buffer.as_mut_ptr(),
               b"%s %d\x00" as *const u8 as *const i8,
               move_string.as_mut_ptr(), &mut diff as *mut i32);
        move_count_0 =
              FE::strlen(move_string.as_mut_ptr()).wrapping_sub(1 as i32 as
                                                              u64).wrapping_div(3
                                                                                              as
                                                                                              i32
                                                                                              as
                                                                                              u64)
                as i32;
        games_parsed += 1;
        i = 0 as i32;
        while i < move_count_0 {
            sscanf(move_string.as_mut_ptr().offset((3 as i32 * i) as
                                                       isize),
                   b"%c%c%c\x00" as *const u8 as *const i8,
                   &mut sign as *mut i8,
                   &mut column as *mut i8,
                   &mut row as *mut i8);
            game_move_list[i as usize] =
                (10 as i32 * (row as i32 - '0' as i32) +
                     (column as i32 - 'a' as i32 + 1 as i32))
                    as i16;
            if sign as i32 == '-' as i32 {
                game_move_list[i as usize] =
                    -(game_move_list[i as usize] as i32) as
                        i16
            }
            i += 1
        }
        if abs(diff) <= max_diff {
            add_new_game(move_count_0, game_move_list.as_mut_ptr(),
                         min_empties, 0 as i32, 0 as i32,
                         0 as i32, 0 as i32);
            printf(b"|\x00" as *const u8 as *const i8);
            if games_imported % 100 as i32 == 0 as i32 {
                printf(b" --- %d games --- \x00" as *const u8 as
                           *const i8, games_imported);
            }
            fflush(stdout);
            games_imported += 1
        }
        if !(games_parsed < max_game_count) { break ; }
    }
    time(&mut stop_time);
    fclose(stream);
    printf(b"\ndone (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    printf(b"%d games read; %d games imported\n\x00" as *const u8 as
               *const i8, games_parsed, games_imported);
    printf(b"Games with final difference <= %d were read until %d empties.\n\x00"
               as *const u8 as *const i8, max_diff, min_empties);
    puts(b"\x00" as *const u8 as *const i8);
}
/*
   READ_TEXT_DATABASE
   Reads an existing ASCII database file.
*/

pub unsafe fn read_text_database(file_name:
                                                *const i8) {
    let mut i: i32 = 0;
    let mut magic1: i32 = 0;
    let mut magic2: i32 = 0;
    let mut new_book_node_count: i32 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut stream = 0 as *mut FILE;
    time(&mut start_time);
    printf(b"Reading text opening database... \x00" as *const u8 as
               *const i8);
    fflush(stdout);
    stream = fopen(file_name, b"r\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Could not open database file\x00" as *const u8 as
                        *const i8, file_name);
    }
    fscanf(stream, b"%d\x00" as *const u8 as *const i8,
           &mut magic1 as *mut i32);
    fscanf(stream, b"%d\x00" as *const u8 as *const i8,
           &mut magic2 as *mut i32);
    if magic1 != 2718 as i32 || magic2 != 2818 as i32 {
        fatal_error(b"%s: %s\x00" as *const u8 as *const i8,
                    b"Wrong checksum, might be an old version\x00" as
                        *const u8 as *const i8, file_name);
    }
    fscanf(stream, b"%d\x00" as *const u8 as *const i8,
           &mut new_book_node_count as *mut i32);
    set_allocation::<FE>(new_book_node_count + 1000 as i32);
    i = 0 as i32;
    while i < new_book_node_count {
        fscanf(stream,
               b"%d %d %hd %hd %hd %hd %hd\n\x00" as *const u8 as
                   *const i8,
               &mut (*node.offset(i as isize)).hash_val1 as *mut i32,
               &mut (*node.offset(i as isize)).hash_val2 as *mut i32,
               &mut (*node.offset(i as isize)).black_minimax_score as
                   *mut i16,
               &mut (*node.offset(i as isize)).white_minimax_score as
                   *mut i16,
               &mut (*node.offset(i as isize)).best_alternative_move as
                   *mut i16,
               &mut (*node.offset(i as isize)).alternative_score as
                   *mut i16,
               &mut (*node.offset(i as isize)).flags as *mut u16);
        i += 1
    }
    book_node_count = new_book_node_count;
    create_hash_reference();
    fclose(stream);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
}
/*
   READ_BINARY_DATABASE
   Reads a binary database file.
*/

pub unsafe fn read_binary_database(file_name:
                                                  *const i8) {
    let mut i: i32 = 0;
    let mut new_book_node_count: i32 = 0;
    let mut magic1: i16 = 0;
    let mut magic2: i16 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut stream = 0 as *mut FILE;
    time(&mut start_time);
    printf(b"Reading binary opening database... \x00" as *const u8 as
               *const i8);
    fflush(stdout);
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Could not open database file\x00" as *const u8 as
                        *const i8, file_name);
    }
    fread(&mut magic1 as *mut i16 as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          1 as i32 as size_t, stream);
    fread(&mut magic2 as *mut i16 as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          1 as i32 as size_t, stream);
    if magic1 as i32 != 2718 as i32 ||
           magic2 as i32 != 2818 as i32 {
        fatal_error(b"%s: %s\x00" as *const u8 as *const i8,
                    b"Wrong checksum, might be an old version\x00" as
                        *const u8 as *const i8, file_name);
    }
    fread(&mut new_book_node_count as *mut i32 as *mut std::ffi::c_void,
          ::std::mem::size_of::<i32>() as u64,
          1 as i32 as size_t, stream);
    set_allocation::<FE>(new_book_node_count + 1000 as i32);
    i = 0 as i32;
    while i < new_book_node_count {
        fread(&mut (*node.offset(i as isize)).hash_val1 as *mut i32 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i32>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*node.offset(i as isize)).hash_val2 as *mut i32 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i32>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*node.offset(i as isize)).black_minimax_score as
                  *mut i16 as *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*node.offset(i as isize)).white_minimax_score as
                  *mut i16 as *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*node.offset(i as isize)).best_alternative_move as
                  *mut i16 as *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*node.offset(i as isize)).alternative_score as
                  *mut i16 as *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*node.offset(i as isize)).flags as *mut u16 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<u16>() as u64,
              1 as i32 as size_t, stream);
        i += 1
    }
    fclose(stream);
    book_node_count = new_book_node_count;
    create_hash_reference();
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
}
/*
   MERGE_BINARY_DATABASE
   Merges a binary database file with the current book.
*/

pub unsafe fn merge_binary_database(file_name:
                                                   *const i8) {
    let mut start_time: time_t = 0;
    time(&mut start_time);
    printf(b"Importing binary opening database... \x00" as *const u8 as
               *const i8);
    fflush(stdout);
    let stream =
        fopen(file_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Could not open database file\x00" as *const u8 as
                        *const i8, file_name);
    }
    let mut magic1: i16 = 0;
    let mut magic2: i16 = 0;
    fread(&mut magic1 as *mut i16 as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          1 as i32 as size_t, stream);
    fread(&mut magic2 as *mut i16 as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          1 as i32 as size_t, stream);
    if magic1 as i32 != 2718 as i32 ||
           magic2 as i32 != 2818 as i32 {
        fatal_error(b"%s: %s\x00" as *const u8 as *const i8,
                    b"Wrong checksum, might be an old version\x00" as
                        *const u8 as *const i8, file_name);
    }
    let mut merge_book_node_count: i32 = 0;
    fread(&mut merge_book_node_count as *mut i32 as *mut std::ffi::c_void,
          ::std::mem::size_of::<i32>() as u64,
          1 as i32 as size_t, stream);
    let mut merge_use_count = 0 as i32;
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < merge_book_node_count {
        let mut merge_node =
            BookNode{hash_val1: 0,
                     hash_val2: 0,
                     black_minimax_score: 0,
                     white_minimax_score: 0,
                     best_alternative_move: 0,
                     alternative_score: 0,
                     flags: 0,};
        /* Read node. */
        fread(&mut merge_node.hash_val1 as *mut i32 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i32>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut merge_node.hash_val2 as *mut i32 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i32>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut merge_node.black_minimax_score as *mut i16 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut merge_node.white_minimax_score as *mut i16 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut merge_node.best_alternative_move as *mut i16 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut merge_node.alternative_score as *mut i16 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut merge_node.flags as *mut u16 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<u16>() as u64,
              1 as i32 as size_t, stream);
        /* Look up node in existing database. */
        let slot =
            probe_hash_table(merge_node.hash_val1, merge_node.hash_val2);
        if slot == -(1 as i32) ||
               *book_hash_table.offset(slot as isize) == -(1 as i32) {
            /* New position, add it without modifications. */
            let this_node =
                create_BookNode::<FE>(merge_node.hash_val1, merge_node.hash_val2,
                                merge_node.flags);
            *node.offset(this_node as isize) = merge_node;
            merge_use_count += 1
        } else {
            /* Existing position, use the book from the merge file if it contains
            better endgame information. */
            let index = *book_hash_table.offset(slot as isize);
            if merge_node.flags as i32 & 16 as i32 != 0 &&
                   (*node.offset(index as isize)).flags as i32 &
                       16 as i32 == 0 ||
                   merge_node.flags as i32 & 4 as i32 != 0 &&
                       (*node.offset(index as isize)).flags as i32 &
                           4 as i32 == 0 {
                *node.offset(index as isize) = merge_node;
                merge_use_count += 1
            }
        }
        i += 1
    }
    fclose(stream);
    /* Make sure the tree is in reasonably good shape after the merge. */
    minimax_tree();
    let mut stop_time: time_t = 0;
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    printf(b"Used %d out of %d nodes from the merge file.\x00" as *const u8 as
               *const i8, merge_use_count, merge_book_node_count);
}
/*
   WRITE_TEXT_DATABASE
   Writes the database to an ASCII file.
*/

pub unsafe fn write_text_database(file_name:
                                                 *const i8) {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    time(&mut start_time);
    printf(b"Writing text database... \x00" as *const u8 as *const i8);
    fflush(stdout);
    let stream = fopen(file_name, b"w\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Could not create database file\x00" as *const u8 as
                        *const i8, file_name);
    }
    fprintf(stream, b"%d\n%d\n\x00" as *const u8 as *const i8,
            2718 as i32, 2818 as i32);
    fprintf(stream, b"%d\n\x00" as *const u8 as *const i8,
            book_node_count);
    let mut i = 0 as i32;
    while i < book_node_count {
        fprintf(stream,
                b"%d %d %d %d %d %d %d\n\x00" as *const u8 as
                    *const i8, (*node.offset(i as isize)).hash_val1,
                (*node.offset(i as isize)).hash_val2,
                (*node.offset(i as isize)).black_minimax_score as i32,
                (*node.offset(i as isize)).white_minimax_score as i32,
                (*node.offset(i as isize)).best_alternative_move as
                    i32,
                (*node.offset(i as isize)).alternative_score as i32,
                (*node.offset(i as isize)).flags as i32);
        i += 1
    }
    fclose(stream);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
}
/*
   WRITE_BINARY_DATABASE
   Writes the database to a binary file.
*/

pub unsafe fn write_binary_database(file_name:
                                                   *const i8) {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    time(&mut start_time);
    printf(b"Writing binary database... \x00" as *const u8 as *const i8);
    fflush(stdout);
    let stream = fopen(file_name, b"wb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Could not create database file\x00" as *const u8 as
                        *const i8, file_name);
    }
    let mut magic = 2718 as i32 as i16;
    fwrite(&mut magic as *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    let mut magic = 2818 as i32 as i16;
    fwrite(&mut magic as *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut book_node_count as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    let mut i = 0 as i32;
    while i < book_node_count {
        fwrite(&mut (*node.offset(i as isize)).hash_val1 as *mut i32
                   as *const std::ffi::c_void,
               ::std::mem::size_of::<i32>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*node.offset(i as isize)).hash_val2 as *mut i32
                   as *const std::ffi::c_void,
               ::std::mem::size_of::<i32>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*node.offset(i as isize)).black_minimax_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*node.offset(i as isize)).white_minimax_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*node.offset(i as isize)).best_alternative_move as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*node.offset(i as isize)).alternative_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*node.offset(i as isize)).flags as *mut u16 as
                   *const std::ffi::c_void,
               ::std::mem::size_of::<u16>() as u64,
               1 as i32 as size_t, stream);
        i += 1
    }
    fclose(stream);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
}

/*
   WRITE_COMPRESSED_DATABASE
   Creates and saves a compressed database file.
*/

pub unsafe fn write_compressed_database(file_name:
                                                       *const i8) {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    time(&mut start_time);
    printf(b"Writing compressed database... \x00" as *const u8 as *const i8);
    fflush(stdout);
    let stream = fopen(file_name, b"wb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Could not create database file\x00" as *const u8 as
                        *const i8, file_name);
    }
    prepare_tree_traversal();
    let node_order =
        safe_malloc::<FE>((book_node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                         as u64)) as
            *mut i32;
    let child_count =
        safe_malloc::<FE>((book_node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                         as u64)) as
            *mut i16;
    let child =
        malloc((book_node_count as
                    u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                    as u64)) as
            *mut i16;
    let mut i = 0 as i32;
    while i < book_node_count {
        let ref mut fresh45 = (*node.offset(i as isize)).flags;
        *fresh45 =
            (*fresh45 as i32 | 8 as i32) as u16;
        i += 1
    }
    let mut node_index = 0 as i32;
    let mut child_index = 0 as i32;
    do_compress(0 as i32, node_order, child_count, &mut node_index,
                child, &mut child_index);
    fwrite(&mut book_node_count as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut child_index as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    fwrite(child_count as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           book_node_count as size_t, stream);
    fwrite(child as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           child_index as size_t, stream);
    i = 0 as i32;
    while i < book_node_count {
        fwrite(&mut (*node.offset(*node_order.offset(i as isize) as
                                      isize)).black_minimax_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*node.offset(*node_order.offset(i as isize) as
                                      isize)).white_minimax_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        i += 1
    }
    i = 0 as i32;
    while i < book_node_count {
        fwrite(&mut (*node.offset(*node_order.offset(i as isize) as
                                      isize)).best_alternative_move as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        i += 1
    }
    i = 0 as i32;
    while i < book_node_count {
        fwrite(&mut (*node.offset(*node_order.offset(i as isize) as
                                      isize)).alternative_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        i += 1
    }
    i = 0 as i32;
    while i < book_node_count {
        fwrite(&mut (*node.offset(*node_order.offset(i as isize) as
                                      isize)).flags as *mut u16 as
                   *const std::ffi::c_void,
               ::std::mem::size_of::<u16>() as u64,
               1 as i32 as size_t, stream);
        i += 1
    }
    fclose(stream);
    free(node_order as *mut std::ffi::c_void);
    free(child_count as *mut std::ffi::c_void);
    free(child as *mut std::ffi::c_void);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
}
/*
  DO_UNCOMPRESS
  Uncompress the subtree below the current node. This is done
  in preorder.
*/
unsafe fn do_uncompress(depth: i32,
                                   stream: *mut FILE,
                                   node_index: *mut i32,
                                   child_index: *mut i32,
                                   child_count: *mut i16,
                                   child: *mut i16,
                                   black_score: *mut i16,
                                   white_score: *mut i16,
                                   alt_move: *mut i16,
                                   alt_score: *mut i16,
                                   flags: *mut u16) {
    let mut i: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut saved_child_index: i32 = 0;
    let mut saved_child_count: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut this_move: i32 = 0;
    if *flags.offset(*node_index as isize) as i32 & 1 as i32
           != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    saved_child_count =
        *child_count.offset(*node_index as isize) as i32;
    saved_child_index = *child_index;
    *child_index += saved_child_count;
    /* Write the data for the current node */
    get_hash(&mut val1, &mut val2, &mut orientation);
    fwrite(&mut val1 as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut val2 as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut *black_score.offset(*node_index as isize) as
               *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut *white_score.offset(*node_index as isize) as
               *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut *alt_move.offset(*node_index as isize) as *mut i16
               as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut *alt_score.offset(*node_index as isize) as *mut i16
               as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut *flags.offset(*node_index as isize) as *mut u16 as
               *const std::ffi::c_void,
           ::std::mem::size_of::<u16>() as u64,
           1 as i32 as size_t, stream);
    *node_index += 1;
    /* Recursively traverse the children */
    i = 0 as i32;
    while i < saved_child_count {
        let mut flipped: i32 = 0;
        this_move =
            *child.offset((saved_child_index + i) as isize) as i32;
        flipped = make_move_no_hash(side_to_move, this_move);
        if flipped == 0 as i32 {
            printf(b"%c%c flips %d discs for %d\n\x00" as *const u8 as
                       *const i8,
                   'a' as i32 + this_move % 10 as i32 -
                       1 as i32,
                   '0' as i32 + this_move / 10 as i32, flipped,
                   side_to_move);
        }
        do_uncompress(depth + 1 as i32, stream, node_index,
                      child_index, child_count, child, black_score,
                      white_score, alt_move, alt_score, flags);
        unmake_move_no_hash(side_to_move, this_move);
        i += 1
    };
}
/*
  UNPACK_COMPRESSED_DATABASE
  Reads a database compressed with WRITE_COMPRESSED_DATABASE
  and unpacks it into an ordinary .bin file.
*/

pub unsafe fn unpack_compressed_database(in_name:
                                                        *const i8,
                                                    out_name:
                                                        *const i8) {
    let mut i: i32 = 0;
    let mut dummy: i32 = 0;
    let mut node_count: i32 = 0;
    let mut child_list_size: i32 = 0;
    let mut node_index: i32 = 0;
    let mut child_index: i32 = 0;
    let mut magic: i16 = 0;
    let mut child_count = 0 as *mut i16;
    let mut child = 0 as *mut i16;
    let mut black_score = 0 as *mut i16;
    let mut white_score = 0 as *mut i16;
    let mut alt_move = 0 as *mut i16;
    let mut alt_score = 0 as *mut i16;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut flags = 0 as *mut u16;
    let mut stream = 0 as *mut FILE;
    printf(b"Uncompressing compressed database... \x00" as *const u8 as
               *const i8);
    fflush(stdout);
    time(&mut start_time);
    /* Read the compressed database */
    stream = fopen(in_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Could not open database file\x00" as *const u8 as
                        *const i8, in_name);
    }
    fread(&mut node_count as *mut i32 as *mut std::ffi::c_void,
          ::std::mem::size_of::<i32>() as u64,
          1 as i32 as size_t, stream);
    fread(&mut child_list_size as *mut i32 as *mut std::ffi::c_void,
          ::std::mem::size_of::<i32>() as u64,
          1 as i32 as size_t, stream);
    child_count =
        safe_malloc::<FE>((node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                         as u64)) as
            *mut i16;
    child =
        safe_malloc::<FE>((child_list_size as
                         u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                         as u64)) as
            *mut i16;
    fread(child_count as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          node_count as size_t, stream);
    fread(child as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          child_list_size as size_t, stream);
    black_score =
        safe_malloc::<FE>((node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                         as u64)) as
            *mut i16;
    white_score =
        safe_malloc::<FE>((node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                         as u64)) as
            *mut i16;
    alt_move =
        safe_malloc::<FE>((node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                         as u64)) as
            *mut i16;
    alt_score =
        safe_malloc::<FE>((node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                         as u64)) as
            *mut i16;
    flags =
        safe_malloc::<FE>((node_count as
                         u64).wrapping_mul(::std::mem::size_of::<u16>()
                                                         as u64)) as
            *mut u16;
    i = 0 as i32;
    while i < node_count {
        fread(&mut *black_score.offset(i as isize) as *mut i16 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut *white_score.offset(i as isize) as *mut i16 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        i += 1
    }
    fread(alt_move as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          node_count as size_t, stream);
    fread(alt_score as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          node_count as size_t, stream);
    fread(flags as *mut std::ffi::c_void,
          ::std::mem::size_of::<u16>() as u64,
          node_count as size_t, stream);
    fclose(stream);
    /* Traverse the tree described by the database and create the .bin file */
    stream = fopen(out_name, b"wb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Could not create database file\x00" as *const u8 as
                        *const i8, out_name);
    }
    toggle_experimental(0 as i32);
    game_init(0 as *const i8, &mut dummy);
    toggle_midgame_hash_usage(1 as i32, 1 as i32);
    toggle_abort_check(0 as i32);
    toggle_midgame_abort_check(0 as i32);
    magic = 2718 as i32 as i16;
    fwrite(&mut magic as *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    magic = 2818 as i32 as i16;
    fwrite(&mut magic as *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut node_count as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    node_index = 0 as i32;
    child_index = 0 as i32;
    do_uncompress(0 as i32, stream, &mut node_index, &mut child_index,
                  child_count, child, black_score, white_score, alt_move,
                  alt_score, flags);
    fclose(stream);
    /* Free tables */
    free(child_count as *mut std::ffi::c_void);
    free(child as *mut std::ffi::c_void);
    free(black_score as *mut std::ffi::c_void);
    free(white_score as *mut std::ffi::c_void);
    free(alt_move as *mut std::ffi::c_void);
    free(alt_score as *mut std::ffi::c_void);
    free(flags as *mut std::ffi::c_void);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
}
/*
  MERGE_POSITION_LIST
  Adds the scores from the positions defined in SCRIPT_FILE and solved
  in OUTPUT_FILE to the book.  The two files are checked for sanity -
  if they don't describe the same set of positions, something has gone awry.
*/

pub unsafe fn merge_position_list<FE: FrontEnd>(script_file:
                                                 *const i8,
                                                output_file:
                                                 *const i8) {
    let mut script_buffer: [i8; 1024] = [0; 1024];
    let mut result_buffer: [i8; 1024] = [0; 1024];
    let mut move_buffer: [i8; 1024] = [0; 1024];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut col: i32 = 0;
    let mut line: i32 = 0;
    let mut score: i32 = 0;
    let mut move_0: i32 = 0;
    let mut wld_only: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut slot: i32 = 0;
    let mut index: i32 = 0;
    let mut position_count: i32 = 0;
    let mut already_wld_count: i32 = 0;
    let mut already_exact_count: i32 = 0;
    let mut tokens_read: i32 = 0;
    let mut moves_read: i32 = 0;
    let mut new_nodes_created: i32 = 0;
    let mut probable_error: i32 = 0;
    let mut script_stream = 0 as *mut FILE;
    let mut result_stream = 0 as *mut FILE;
    script_stream =
        fopen(script_file, b"r\x00" as *const u8 as *const i8);
    if script_stream.is_null() {
        fprintf(stderr,
                b"Can\'t open %s\n\x00" as *const u8 as *const i8,
                script_file);
        exit(1 as i32);
    }
    result_stream =
        fopen(output_file, b"r\x00" as *const u8 as *const i8);
    if result_stream.is_null() {
        fprintf(stderr,
                b"Can\'t open %s\n\x00" as *const u8 as *const i8,
                output_file);
        exit(1 as i32);
    }
    prepare_tree_traversal();
    line = 1 as i32;
    position_count = 0 as i32;
    already_wld_count = 0 as i32;
    already_exact_count = 0 as i32;
    new_nodes_created = 0 as i32;
    fgets(script_buffer.as_mut_ptr(), 1024 as i32, script_stream);
    fgets(result_buffer.as_mut_ptr(), 1024 as i32, result_stream);
    while feof(script_stream) == 0 && feof(result_stream) == 0 {
        let mut ch = 0 as *mut i8;
        ch =
            script_buffer.as_mut_ptr().offset(strlen(script_buffer.as_mut_ptr())
                                                  as
                                                  isize).offset(-(1 as
                                                                      i32
                                                                      as
                                                                      isize));
        while ch >= script_buffer.as_mut_ptr() &&
                  *(*__ctype_b_loc()).offset(*ch as i32 as isize) as
                      i32 &
                      _ISgraph as i32 as u16 as i32
                      == 0 {
            *ch = 0 as i32 as i8;
            ch = ch.offset(-1)
        }
        ch =
            result_buffer.as_mut_ptr().offset(strlen(result_buffer.as_mut_ptr())
                                                  as
                                                  isize).offset(-(1 as
                                                                      i32
                                                                      as
                                                                      isize));
        while ch >= result_buffer.as_mut_ptr() &&
                  *(*__ctype_b_loc()).offset(*ch as i32 as isize) as
                      i32 &
                      _ISgraph as i32 as u16 as i32
                      == 0 {
            *ch = 0 as i32 as i8;
            ch = ch.offset(-1)
        }
        if line % 4 as i32 == 3 as i32 {
            /* The position/result lines */
            position_count += 1;
            /* Parse the board */
            disks_played =
                0 as i32; /* The initial board contains 4 discs */
            col = 0 as i32;
            i = 1 as i32;
            while i <= 8 as i32 {
                j = 1 as i32;
                while j <= 8 as i32 {
                    pos = 10 as i32 * i + j;
                    match script_buffer[col as usize] as i32 {
                        42 | 88 | 120 => {
                            board[pos as usize] = 0 as i32;
                            disks_played += 1
                        }
                        79 | 48 | 111 => {
                            board[pos as usize] = 2 as i32;
                            disks_played += 1
                        }
                        45 | 46 => { board[pos as usize] = 1 as i32 }
                        _ => {
                            fprintf(stderr,
                                    b"\nBad character \'%c\' in board on line %d\n\n\x00"
                                        as *const u8 as *const i8,
                                    script_buffer[col as usize] as
                                        i32, line);
                            exit(1 as i32);
                        }
                    }
                    col += 1;
                    j += 1
                }
                i += 1
            }
            match script_buffer[65 as i32 as usize] as i32 {
                42 | 88 | 120 => { side_to_move = 0 as i32 }
                79 | 48 | 111 => { side_to_move = 2 as i32 }
                _ => {
                    fprintf(stderr,
                            b"\nBad side to move \'%c\' in board on line %d\n\n\x00"
                                as *const u8 as *const i8,
                            script_buffer[65 as i32 as usize] as
                                i32, line);
                    exit(1 as i32);
                }
            }
            disks_played -= 4 as i32;
            /* Parse the result */
            wld_only = 1 as i32;
            if strstr(result_buffer.as_mut_ptr(),
                      b"Black win\x00" as *const u8 as *const i8) ==
                   result_buffer.as_mut_ptr() {
                score = 30000 as i32 + 2 as i32;
                tokens_read =
                    sscanf(result_buffer.as_mut_ptr(),
                           b"%*s %*s %s\x00" as *const u8 as
                               *const i8, move_buffer.as_mut_ptr());
                moves_read = tokens_read
            } else if strstr(result_buffer.as_mut_ptr(),
                             b"White win\x00" as *const u8 as
                                 *const i8) ==
                          result_buffer.as_mut_ptr() {
                score = -(30000 as i32 + 2 as i32);
                tokens_read =
                    sscanf(result_buffer.as_mut_ptr(),
                           b"%*s %*s %s\x00" as *const u8 as
                               *const i8, move_buffer.as_mut_ptr());
                moves_read = tokens_read
            } else if strstr(result_buffer.as_mut_ptr(),
                             b"Draw\x00" as *const u8 as *const i8)
                          == result_buffer.as_mut_ptr() {
                score = 0 as i32;
                tokens_read =
                    sscanf(result_buffer.as_mut_ptr(),
                           b"%*s %s\x00" as *const u8 as *const i8,
                           move_buffer.as_mut_ptr());
                moves_read = tokens_read
            } else {
                /* Exact score */
                let mut black_discs: i32 = 0;
                let mut white_discs: i32 = 0;
                wld_only = 0 as i32;
                tokens_read =
                    sscanf(result_buffer.as_mut_ptr(),
                           b"%d %*s %d %s\x00" as *const u8 as
                               *const i8,
                           &mut black_discs as *mut i32,
                           &mut white_discs as *mut i32,
                           move_buffer.as_mut_ptr());
                moves_read = tokens_read - 2 as i32;
                score = black_discs - white_discs;
                if score > 0 as i32 {
                    score += 30000 as i32
                } else if score < 0 as i32 {
                    score -= 30000 as i32
                }
            }
            /* Set the score for the node corresponding to the position */
            get_hash(&mut val1, &mut val2, &mut orientation);
            slot = probe_hash_table(val1, val2);
            index = *book_hash_table.offset(slot as isize);
            if index == -(1 as i32) {
                fprintf(stderr,
                        b"Position on line %d not found in book\n\x00" as
                            *const u8 as *const i8, line);
                exit(0 as i32);
            }
            probable_error = 0 as i32;
            if (*node.offset(index as isize)).flags as i32 &
                   4 as i32 != 0 {
                already_wld_count += 1;
                if score > 0 as i32 &&
                       (*node.offset(index as isize)).black_minimax_score as
                           i32 <= 0 as i32 ||
                       score == 0 as i32 &&
                           (*node.offset(index as isize)).black_minimax_score
                               as i32 != 0 as i32 ||
                       score < 0 as i32 &&
                           (*node.offset(index as isize)).black_minimax_score
                               as i32 > 0 as i32 {
                    probable_error = 1 as i32;
                    fprintf(stderr,
                            b"Line %d: New WLD score %d conflicts with old score %d\n\x00"
                                as *const u8 as *const i8, line,
                            score,
                            (*node.offset(index as isize)).black_minimax_score
                                as i32);
                }
            }
            if (*node.offset(index as isize)).flags as i32 &
                   16 as i32 != 0 {
                already_exact_count += 1;
                if wld_only == 0 &&
                       score !=
                           (*node.offset(index as isize)).black_minimax_score
                               as i32 {
                    probable_error = 1 as i32;
                    fprintf(stderr,
                            b"Line %d: New exact score %d conflicts with old score %d\n\x00"
                                as *const u8 as *const i8, line,
                            score,
                            (*node.offset(index as isize)).black_minimax_score
                                as i32);
                }
            }
            if probable_error != 0 || wld_only == 0 ||
                   (*node.offset(index as isize)).flags as i32 &
                       16 as i32 == 0 {
                let ref mut fresh46 =
                    (*node.offset(index as isize)).white_minimax_score;
                *fresh46 = score as i16;
                (*node.offset(index as isize)).black_minimax_score = *fresh46
            }
            if probable_error != 0 {
                /* Clear the old flags if score was wrong */
                let ref mut fresh47 = (*node.offset(index as isize)).flags;
                *fresh47 =
                    (*fresh47 as i32 &
                         !(4 as i32 | 16 as i32)) as
                        u16
            }
            if wld_only != 0 {
                let ref mut fresh48 = (*node.offset(index as isize)).flags;
                *fresh48 =
                    (*fresh48 as i32 | 4 as i32) as
                        u16
            } else {
                let ref mut fresh49 = (*node.offset(index as isize)).flags;
                *fresh49 =
                    (*fresh49 as i32 |
                         (4 as i32 | 16 as i32)) as
                        u16
            }
            /* Examine the position arising from the PV move; if it exists it
            need only be checked for sanity, otherwise a new node is
             created. */
            if moves_read > 0 as i32 {
                /* Make sure the optimal move leads to a position in the hash table */
                let mut row: i32 = 0;
                let mut col_0: i32 = 0;
                row =
                    move_buffer[1 as i32 as usize] as i32 -
                        '0' as i32;
                col_0 =
                   FE::tolower(move_buffer[0 as i32 as usize] as
                                i32) - 'a' as i32 + 1 as i32;
                move_0 = 10 as i32 * row + col_0;
                if row >= 1 as i32 && row <= 8 as i32 &&
                       col_0 >= 1 as i32 && col_0 <= 8 as i32
                       && make_move_no_hash(side_to_move, move_0) != 0 {
                    let mut new_side_to_move =
                        0 as i32 + 2 as i32 - side_to_move;
                    generate_all(new_side_to_move);
                    if move_count[disks_played as usize] == 0 as i32 {
                        new_side_to_move = side_to_move
                    }
                    get_hash(&mut val1, &mut val2, &mut orientation);
                    slot = probe_hash_table(val1, val2);
                    index = *book_hash_table.offset(slot as isize);
                    if index == -(1 as i32) {
                        index =
                            create_BookNode::<FE>(val1, val2,
                                            32 as i32 as
                                                u16);
                        let ref mut fresh50 =
                            (*node.offset(index as
                                              isize)).white_minimax_score;
                        *fresh50 = score as i16;
                        (*node.offset(index as isize)).black_minimax_score =
                            *fresh50;
                        if new_side_to_move == 0 as i32 {
                            let ref mut fresh51 =
                                (*node.offset(index as isize)).flags;
                            *fresh51 =
                                (*fresh51 as i32 | 1 as i32)
                                    as u16
                        } else {
                            let ref mut fresh52 =
                                (*node.offset(index as isize)).flags;
                            *fresh52 =
                                (*fresh52 as i32 | 2 as i32)
                                    as u16
                        }
                        if wld_only != 0 {
                            let ref mut fresh53 =
                                (*node.offset(index as isize)).flags;
                            *fresh53 =
                                (*fresh53 as i32 | 4 as i32)
                                    as u16
                        } else {
                            let ref mut fresh54 =
                                (*node.offset(index as isize)).flags;
                            *fresh54 =
                                (*fresh54 as i32 |
                                     (4 as i32 | 16 as i32))
                                    as u16
                        }
                        new_nodes_created += 1
                    } else {
                        /* Position already exists, sanity-check it */
                        probable_error = 0 as i32;
                        if (*node.offset(index as isize)).flags as i32
                               & 4 as i32 != 0 {
                            if score > 0 as i32 &&
                                   (*node.offset(index as
                                                     isize)).black_minimax_score
                                       as i32 <= 0 as i32 ||
                                   score == 0 as i32 &&
                                       (*node.offset(index as
                                                         isize)).black_minimax_score
                                           as i32 != 0 as i32
                                   ||
                                   score < 0 as i32 &&
                                       (*node.offset(index as
                                                         isize)).black_minimax_score
                                           as i32 > 0 as i32 {
                                probable_error = 1 as i32;
                                fprintf(stderr,
                                        b"Line %d: New child WLD score %d conflicts with old score %d\n\x00"
                                            as *const u8 as
                                            *const i8, line, score,
                                        (*node.offset(index as
                                                          isize)).black_minimax_score
                                            as i32);
                            }
                        }
                        if (*node.offset(index as isize)).flags as i32
                               & 16 as i32 != 0 {
                            if wld_only == 0 &&
                                   score !=
                                       (*node.offset(index as
                                                         isize)).black_minimax_score
                                           as i32 {
                                probable_error = 1 as i32;
                                fprintf(stderr,
                                        b"Line %d: New child exact score %d conflicts with old score %d\n\x00"
                                            as *const u8 as
                                            *const i8, line, score,
                                        (*node.offset(index as
                                                          isize)).black_minimax_score
                                            as i32);
                            }
                        }
                        if probable_error != 0 {
                            /* Correct errors encountered */
                            let ref mut fresh55 =
                                (*node.offset(index as
                                                  isize)).white_minimax_score;
                            *fresh55 = score as i16;
                            (*node.offset(index as isize)).black_minimax_score
                                = *fresh55;
                            let ref mut fresh56 =
                                (*node.offset(index as isize)).flags;
                            *fresh56 =
                                (*fresh56 as i32 &
                                     !(4 as i32 | 16 as i32))
                                    as u16;
                            if wld_only != 0 {
                                let ref mut fresh57 =
                                    (*node.offset(index as isize)).flags;
                                *fresh57 =
                                    (*fresh57 as i32 |
                                         4 as i32) as u16
                            } else {
                                let ref mut fresh58 =
                                    (*node.offset(index as isize)).flags;
                                *fresh58 =
                                    (*fresh58 as i32 |
                                         (4 as i32 |
                                              16 as i32)) as
                                        u16
                            }
                        }
                    }
                    unmake_move_no_hash(side_to_move, move_0);
                } else {
                    fprintf(stderr,
                            b"Line %d: The PV move \'%s\' is invalid\n\x00" as
                                *const u8 as *const i8, line,
                            move_buffer.as_mut_ptr());
                    exit(1 as i32);
                }
            }
        } else if strcmp(script_buffer.as_mut_ptr(),
                         result_buffer.as_mut_ptr()) != 0 {
            fprintf(stderr,
                    b"Script and result files differ unexpectedly on line %d\n\x00"
                        as *const u8 as *const i8, line);
            exit(1 as i32);
        }
        fgets(script_buffer.as_mut_ptr(), 1024 as i32, script_stream);
        fgets(result_buffer.as_mut_ptr(), 1024 as i32, result_stream);
        line += 1
    }
    line -= 1;
    printf(b"%d lines read from the script and result files\n\x00" as
               *const u8 as *const i8, line);
    if feof(script_stream) == 0 || feof(result_stream) == 0 {
        puts(b"Warning: The two files don\'t have the same number of lines.\x00"
                 as *const u8 as *const i8);
    }
    printf(b"%d positions merged with the book\n\x00" as *const u8 as
               *const i8, position_count);
    printf(b"%d positions were already solved for exact score\n\x00" as
               *const u8 as *const i8, already_exact_count);
    printf(b"%d positions were already solved WLD\n\x00" as *const u8 as
               *const i8, already_wld_count);
    printf(b"%d positions had optimal moves leading to new positions\n\x00" as
               *const u8 as *const i8, new_nodes_created);
    puts(b"\x00" as *const u8 as *const i8);
    fclose(script_stream);
    fclose(result_stream);
}

pub unsafe fn report_unwanted_book_draw(this_move: i32) {
    printf(b"%c%c leads to an unwanted book draw\n\x00" as *const u8 as *const i8, 'a' as i32 + this_move % 10 as i32 - 1 as i32, '0' as i32 + this_move / 10 as i32);
}
/*
   PRINT_MOVE_ALTERNATIVES
   Displays all available book moves from a position.
   FLAGS specifies a subset of the flag bits which have to be set
   for a position to be considered. Notice that FLAGS=0 accepts
   any flag combination.
*/

pub unsafe fn print_move_alternatives(side_to_move:
                                                     i32) {
    let mut i: i32 = 0;
    let mut sign: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut score: i32 = 0;
    let mut output_score: i32 = 0;
    if candidate_count > 0 as i32 {
        if side_to_move == 0 as i32 {
            sign = 1 as i32
        } else { sign = -(1 as i32) }
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        /* Check that the position is in the opening book after all */
        if slot == -(1 as i32) ||
               *book_hash_table.offset(slot as isize) == -(1 as i32) {
            return
        }
        /* Pick the book score corresponding to the player to move and
           remove draw avoidance and the special scores for nodes WLD. */
        if side_to_move == 0 as i32 {
            score =
                (*node.offset(*book_hash_table.offset(slot as isize) as
                                  isize)).black_minimax_score as i32
        } else {
            score =
                (*node.offset(*book_hash_table.offset(slot as isize) as
                                  isize)).white_minimax_score as i32
        }
        if score == 30000 as i32 - 1 as i32 ||
               score == -(30000 as i32 - 1 as i32) {
            score = 0 as i32
        }
        if score > 30000 as i32 { score -= 30000 as i32 }
        if score < -(30000 as i32) { score += 30000 as i32 }
        printf(b"Book score is \x00" as *const u8 as *const i8);
        if (*node.offset(*book_hash_table.offset(slot as isize) as
                             isize)).flags as i32 & 16 as i32
               != 0 {
            printf(b"%+d (exact score).\x00" as *const u8 as
                       *const i8, sign * score);
        } else if (*node.offset(*book_hash_table.offset(slot as isize) as
                                    isize)).flags as i32 &
                      4 as i32 != 0 {
            printf(b"%+d (W/L/D solved).\x00" as *const u8 as
                       *const i8, sign * score);
        } else {
            printf(b"%+.2f.\x00" as *const u8 as *const i8,
                   (sign * score) as f64 / 128.0f64);
        }
        if (*node.offset(*book_hash_table.offset(slot as isize) as
                             isize)).flags as i32 & 32 as i32
               != 0 {
            printf(b" Private node.\x00" as *const u8 as *const i8);
        }
        puts(b"\x00" as *const u8 as *const i8);
        i = 0 as i32;
        while i < candidate_count {
            printf(b"   %c%c   \x00" as *const u8 as *const i8,
                   'a' as i32 +
                       candidate_list[i as usize].move_0 % 10 as i32 -
                       1 as i32,
                   '0' as i32 +
                       candidate_list[i as usize].move_0 / 10 as i32);
            output_score = candidate_list[i as usize].score;
            if output_score >= 30000 as i32 {
                output_score -= 30000 as i32
            } else if output_score <= -(30000 as i32) {
                output_score += 30000 as i32
            }
            if candidate_list[i as usize].flags & 16 as i32 != 0 {
                printf(b"%+-6d  (exact score)\x00" as *const u8 as
                           *const i8, output_score);
            } else if candidate_list[i as usize].flags & 4 as i32 != 0
             {
                printf(b"%+-6d  (W/L/D solved)\x00" as *const u8 as
                           *const i8, output_score);
            } else {
                printf(b"%+-6.2f\x00" as *const u8 as *const i8,
                       output_score as f64 / 128.0f64);
                if candidate_list[i as usize].flags & 64 as i32 != 0 {
                    printf(b"  (deviation)\x00" as *const u8 as
                               *const i8);
                }
            }
            puts(b"\x00" as *const u8 as *const i8);
            i += 1
        }
    };
}

pub unsafe fn report_in_get_book_move_1(side_to_move: i32, remaining_slack: i32) {
    printf(b"Slack left is %.2f. \x00" as *const u8 as
               *const i8,
           remaining_slack as f64 / 128.0f64);
    print_move_alternatives(side_to_move);
}

pub unsafe fn report_in_get_book_move_2(chosen_score: i32, chosen_index: i32, flags: &i32) {
    send_status(b"-->   Book     \x00" as *const u8 as
        *const i8);
    if flags & 16 as i32 != 0 {
        send_status(b"%+3d (exact)   \x00" as *const u8 as
                        *const i8,
                    chosen_score / 128 as i32);
    } else if flags & 4 as i32 != 0 {
        send_status(b"%+3d (WLD)     \x00" as *const u8 as
                        *const i8,
                    chosen_score / 128 as i32);
    } else {
        send_status(b"%+6.2f        \x00" as *const u8 as
                        *const i8,
                    chosen_score as f64 / 128.0f64);
    }
    if get_ponder_move() != 0 {
        send_status(b"{%c%c} \x00" as *const u8 as *const i8,
                    'a' as i32 + get_ponder_move() % 10 as i32 -
                        1 as i32,
                    '0' as i32 + get_ponder_move() / 10 as i32);
    }
    send_status(b"%c%c\x00" as *const u8 as *const i8,
                'a' as i32 +
                    candidate_list[chosen_index as usize].move_0 %
                        10 as i32 - 1 as i32,
                '0' as i32 +
                    candidate_list[chosen_index as usize].move_0 /
                        10 as i32);
}
/*
  DUPSTR
  A strdup clone.
*/
unsafe fn dupstr(str: *const i8)
 -> *mut i8 {
    let new_str =
        malloc(strlen(str).wrapping_add(1 as i32 as u64)) as
            *mut i8;
    strcpy(new_str, str);
    return new_str;
}
/*
  CONVERT_OPENING_LIST
  Convert a list of openings on Robert Gatliff's format
  to a hash table representation containing the same information.
*/

pub unsafe fn convert_opening_list(base_file:
                                                  *const i8) {
    let mut in_stream =
        0 as *mut FILE; /* Max number of opening names occurring */
    let mut out_stream = 0 as *mut FILE;
    let mut name_start = 0 as *mut i8;
    let mut scan_ptr = 0 as *mut i8;
    let mut move_ptr = 0 as *mut i8;
    let mut source_file_name = 0 as *const i8;
    let mut header_file_name = 0 as *const i8;
    let mut parent: [*mut i8; 1000] =
        [0 as *mut i8; 1000];
    let mut buffer: [i8; 1024] = [0; 1024];
    let mut move_seq: [i8; 256] = [0; 256];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut opening_count: i32 = 0;
    let mut op_move_count: i32 = 0;
    let mut level: i32 = 0;
    let mut hash_val1: i32 = 0;
    let mut hash_val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut op_move: [i32; 60] = [0; 60];
    let mut side_to_move: [i32; 60] = [0; 60];
    let mut timer: time_t = 0;
    in_stream =
        fopen(base_file, b"r\x00" as *const u8 as *const i8);
    if in_stream.is_null() {
        printf(b"Cannot open opening file \'%s\'\n\x00" as *const u8 as
                   *const i8, base_file);
        exit(1 as i32);
    }
    /* Get the number of openings */
    fgets(buffer.as_mut_ptr(), 1023 as i32, in_stream);
    sscanf(buffer.as_mut_ptr(), b"%d\x00" as *const u8 as *const i8,
           &mut opening_count as *mut i32);
    /* Prepare the header file */
    header_file_name = b"opname.h\x00" as *const u8 as *const i8;
    out_stream =
        fopen(header_file_name, b"w\x00" as *const u8 as *const i8);
    if out_stream.is_null() {
        printf(b"Cannot create header file \'%s\'\n\x00" as *const u8 as
                   *const i8, header_file_name);
        exit(1 as i32);
    }
    time(&mut timer);
    fprintf(out_stream, b"/*\n\x00" as *const u8 as *const i8);
    fprintf(out_stream, b"   %s\n\n\x00" as *const u8 as *const i8,
            header_file_name);
    fprintf(out_stream,
            b"   Automatically created by BOOKTOOL on %s\x00" as *const u8 as
                *const i8, ctime(&mut timer));
    fprintf(out_stream, b"*/\x00" as *const u8 as *const i8);
    fprintf(out_stream, b"\n\n\n\x00" as *const u8 as *const i8);
    fputs(b"#ifndef OPNAME_H\n\x00" as *const u8 as *const i8,
          out_stream);
    fputs(b"#define OPNAME_H\n\n\n\x00" as *const u8 as *const i8,
          out_stream);
    fprintf(out_stream,
            b"#define OPENING_COUNT       %d\n\n\n\x00" as *const u8 as
                *const i8, opening_count);
    fputs(b"typedef struct {\n\x00" as *const u8 as *const i8,
          out_stream);
    fputs(b"  const char *name;\n\x00" as *const u8 as *const i8,
          out_stream);
    fputs(b"  const char *sequence;\n\x00" as *const u8 as
              *const i8, out_stream);
    fputs(b"  int hash_val1;\n\x00" as *const u8 as *const i8,
          out_stream);
    fputs(b"  int hash_val2;\n\x00" as *const u8 as *const i8,
          out_stream);
    fputs(b"  int level;\n\x00" as *const u8 as *const i8,
          out_stream);
    fputs(b"} OpeningDescriptor;\n\n\n\x00" as *const u8 as
              *const i8, out_stream);
    fputs(b"extern OpeningDescriptor opening_list[OPENING_COUNT];\n\x00" as
              *const u8 as *const i8, out_stream);
    fputs(b"\n\n#endif  /* OPNAME_H */\n\x00" as *const u8 as
              *const i8, out_stream);
    fclose(out_stream);
    /* Prepare the source file */
    source_file_name = b"opname.c\x00" as *const u8 as *const i8;
    out_stream =
        fopen(source_file_name, b"w\x00" as *const u8 as *const i8);
    if out_stream.is_null() {
        printf(b"Cannot create source file \'%s\'\n\x00" as *const u8 as
                   *const i8, source_file_name);
        exit(1 as i32);
    }
    time(&mut timer);
    fprintf(out_stream, b"/*\n\x00" as *const u8 as *const i8);
    fprintf(out_stream, b"   %s\n\n\x00" as *const u8 as *const i8,
            source_file_name);
    fprintf(out_stream,
            b"   Automatically created by BOOKTOOL on %s\x00" as *const u8 as
                *const i8, ctime(&mut timer));
    fprintf(out_stream, b"*/\x00" as *const u8 as *const i8);
    fprintf(out_stream, b"\n\n\n\x00" as *const u8 as *const i8);
    fprintf(out_stream,
            b"#include \"%s\"\n\n\n\x00" as *const u8 as *const i8,
            header_file_name);
    fputs(b"OpeningDescriptor opening_list[OPENING_COUNT] = {\n\x00" as
              *const u8 as *const i8, out_stream);
    /* Read the list of openings */
    prepare_tree_traversal();
    level = 0 as i32;
    i = 0 as i32;
    while i < opening_count {
        fgets(buffer.as_mut_ptr(), 1023 as i32, in_stream);
        /* Each line in the input file corresponds to one opening.
           First separate the line into opening moves and name. */
        sscanf(buffer.as_mut_ptr(),
               b"%s\x00" as *const u8 as *const i8,
               move_seq.as_mut_ptr());
        name_start =
            buffer.as_mut_ptr().offset(strlen(move_seq.as_mut_ptr()) as
                                           isize);
        while *(*__ctype_b_loc()).offset(*name_start as i32 as isize)
                  as i32 &
                  _ISspace as i32 as u16 as i32 !=
                  0 {
            name_start = name_start.offset(1)
        }
        scan_ptr = name_start;
        while *(*__ctype_b_loc()).offset(*scan_ptr as i32 as isize) as
                  i32 &
                  _ISprint as i32 as u16 as i32 !=
                  0 {
            scan_ptr = scan_ptr.offset(1)
        }
        *scan_ptr = 0 as i32 as i8;
        op_move_count =
              FE::strlen(move_seq.as_mut_ptr()).wrapping_div(2 as i32 as
                                                           u64) as
                i32;
        j = 0 as i32;
        move_ptr = buffer.as_mut_ptr();
        while j < op_move_count {
            if *(*__ctype_b_loc()).offset(*move_ptr as i32 as isize)
                   as i32 &
                   _ISupper as i32 as u16 as i32 !=
                   0 {
                side_to_move[j as usize] = 0 as i32
            } else { side_to_move[j as usize] = 2 as i32 }
            col =
                toupper(*move_ptr as i32) - 'A' as i32 +
                    1 as i32;
            move_ptr = move_ptr.offset(1);
            row = *move_ptr as i32 - '0' as i32;
            move_ptr = move_ptr.offset(1);
            op_move[j as usize] = 10 as i32 * row + col;
            j += 1
        }
        /* Check out how the relation between this openings and the ones
           in the hierachy created to far */
        while level > 0 as i32 &&
                  strstr(move_seq.as_mut_ptr(),
                         parent[(level - 1 as i32) as usize]) !=
                      move_seq.as_mut_ptr() {
            level -= 1;
            free(parent[level as usize] as *mut std::ffi::c_void);
        }
        parent[level as usize] = dupstr(move_seq.as_mut_ptr());
        level += 1;
        /* Create the board position characteristic for the opening. */
        j = 0 as i32;
        while j < op_move_count {
            if generate_specific(op_move[j as usize],
                                 side_to_move[j as usize]) == 0 {
                printf(b"Move %d in opening #%d is illegal\n\x00" as *const u8
                           as *const i8, j + 1 as i32, i);
                exit(1 as i32);
            }
            make_move(side_to_move[j as usize], op_move[j as usize],
                      1 as i32);
            j += 1
        }
        /* Write the code fragment  */
        get_hash(&mut hash_val1, &mut hash_val2, &mut orientation);
        fprintf(out_stream,
                b"   { \"%s\",\n     \"%s\",\n     %d, %d, %d }\x00" as
                    *const u8 as *const i8, name_start,
                move_seq.as_mut_ptr(), hash_val1, hash_val2,
                level - 1 as i32);
        if i != opening_count - 1 as i32 {
            fputs(b" ,\n\x00" as *const u8 as *const i8,
                  out_stream);
        }
        /* Undo the moves */
        j = op_move_count - 1 as i32;
        while j >= 0 as i32 {
            unmake_move(side_to_move[j as usize], op_move[j as usize]);
            j -= 1
        }
        i += 1
    }
    fputs(b"\n};\n\x00" as *const u8 as *const i8, out_stream);
    /* Remove the hierarchy data */
    while level > 0 as i32 {
        level -= 1;
        free(parent[level as usize] as *mut std::ffi::c_void);
    }
    fclose(out_stream);
    fclose(in_stream);
}

/*
   INIT_OSF
   Makes sure all data structures are initialized.
*/

pub unsafe fn init_osf(do_global_setup: i32) {
    engine_init_osf::<LibcFatalError>();
    if do_global_setup != 0 {
        global_setup(0 as i32, 19 as i32);
    };
}

// #ifdef INCLUDE_BOOKTOOL
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StatisticsSpec {
    pub out_file_name: *const i8,
    pub prob: f64,
    pub max_diff: i32,
    pub max_depth: i32,
}
static mut correction_script_name: *const i8 = 0 as *const i8;
/*
  EXPORT_POSITION
  Output the position and its value according to the database
  to file.
*/
unsafe extern "C" fn export_position(side_to_move: i32,
                                     score: i32,
                                     target_file: *mut FILE) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut black_mask: i32 = 0;
    let mut white_mask: i32 = 0;
    let mut hi_mask: i32 = 0;
    let mut lo_mask: i32 = 0;
    i = 1 as i32;
    while i <= 8 as i32 {
        black_mask = 0 as i32;
        white_mask = 0 as i32;
        j = 0 as i32;
        pos = 10 as i32 * i + 1 as i32;
        while j < 8 as i32 {
            if board[pos as usize] == 0 as i32 {
                black_mask |= (1 as i32) << j
            } else if board[pos as usize] == 2 as i32 {
                white_mask |= (1 as i32) << j
            }
            j += 1;
            pos += 1
        }
        hi_mask = black_mask >> 4 as i32;
        lo_mask = black_mask % 16 as i32;
        fprintf(target_file, b"%c%c\x00" as *const u8 as *const i8,
                hi_mask + ' ' as i32, lo_mask + ' ' as i32);
        hi_mask = white_mask >> 4 as i32;
        lo_mask = white_mask % 16 as i32;
        fprintf(target_file, b"%c%c\x00" as *const u8 as *const i8,
                hi_mask + ' ' as i32, lo_mask + ' ' as i32);
        i += 1
    }
    fprintf(target_file, b" \x00" as *const u8 as *const i8);
    if side_to_move == 0 as i32 {
        fputc('*' as i32, target_file);
    } else { fputc('O' as i32, target_file); }
    fprintf(target_file,
            b" %2d %+d\n\x00" as *const u8 as *const i8,
            disks_played, score);
}
/*
   DO_RESTRICTED_MINIMAX
   Calculates the book-only minimax value of node INDEX,
   not caring about deviations from the database.
*/
unsafe extern "C" fn do_restricted_minimax(index: i32,
                                           low: i32,
                                           high: i32,
                                           target_file: *mut FILE,
                                           minimax_values:
                                           *mut i32) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut corrected_score: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut child_count: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut best_score: i16 = 0;
    if (*node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    /* Recursively minimax all children of the node */
    if (*node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    if side_to_move == 0 as i32 {
        best_score = -(32000 as i32) as i16
    } else { best_score = 32000 as i32 as i16 }
    generate_all(side_to_move);
    child_count = 0 as i32;
    i = 0 as i32;
    while i < move_count[disks_played as usize] {
        piece_count[0 as i32 as usize][disks_played as usize] =
            disc_count(0 as i32);
        piece_count[2 as i32 as usize][disks_played as usize] =
            disc_count(2 as i32);
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_restricted_minimax(child, low, high, target_file,
                                  minimax_values);
            corrected_score = *minimax_values.offset(child as isize);
            if side_to_move == 0 as i32 &&
                corrected_score > best_score as i32 ||
                side_to_move == 2 as i32 &&
                    corrected_score < best_score as i32 {
                best_score = corrected_score as i16
            }
            child_count += 1
        }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    if (*node.offset(index as isize)).flags as i32 & 16 as i32
        != 0 ||
        (*node.offset(index as isize)).flags as i32 &
            4 as i32 != 0 && child_count == 0 as i32 {
        best_score = (*node.offset(index as isize)).black_minimax_score
    } else if child_count == 0 as i32 {
        printf(b"%d disks played\n\x00" as *const u8 as *const i8,
               disks_played);
        printf(b"Node #%d has no children and lacks WLD status\n\x00" as
                   *const u8 as *const i8, index);
        exit(1 as i32);
    }
    if best_score as i32 > 30000 as i32 {
        best_score =
            (best_score as i32 - 30000 as i32) as
                i16
    } else if (best_score as i32) < -(30000 as i32) {
        best_score =
            (best_score as i32 + 30000 as i32) as
                i16
    }
    *minimax_values.offset(index as isize) = best_score as i32;
    let ref mut fresh16 = (*node.offset(index as isize)).flags;
    *fresh16 = (*fresh16 as i32 ^ 8 as i32) as u16;
    if disks_played >= low && disks_played <= high {
        export_position(side_to_move, best_score as i32, target_file);
    };
}
/*
   RESTRICTED_MINIMAX_TREE
   Calculates the minimax values of all nodes in the tree,
   not
*/
pub unsafe fn restricted_minimax_tree(low: i32,
                                                 high: i32,
                                                 pos_file_name:
                                                 *const i8) {
    let mut pos_file: *mut FILE = 0 as *mut FILE;
    let mut i: i32 = 0;
    let mut minimax_values: *mut i32 = 0 as *mut i32;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    printf(b"Calculating restricted minimax value... \x00" as *const u8 as
        *const i8);
    fflush(stdout);
    prepare_tree_traversal();
    time(&mut start_time);
    /* Mark all nodes as not traversed */
    i = 0 as i32;
    while i < book_node_count {
        let ref mut fresh17 = (*node.offset(i as isize)).flags;
        *fresh17 =
            (*fresh17 as i32 | 8 as i32) as u16;
        i += 1
    }
    minimax_values =
        safe_malloc::<FE>((book_node_count as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    pos_file =
        fopen(pos_file_name, b"a\x00" as *const u8 as *const i8);
    do_restricted_minimax(0 as i32, low, high, pos_file,
                          minimax_values);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
    free(minimax_values as *mut ::std::ffi::c_void);
    fclose(pos_file);
}
/*
   DO_MIDGAME_STATISTICS
   Recursively makes sure a subtree is evaluated to the specified depth.
*/
unsafe extern "C" fn do_midgame_statistics(index: i32,
                                           spec: StatisticsSpec) {
    let mut dummy_info: EvaluationType =
        EvaluationType{type_0: MIDGAME_EVAL,
            res: WON_POSITION,
            score: 0,
            confidence: 0.,
            search_depth: 0,
            is_book: 0,};
    let mut i: i32 = 0;
    let mut depth: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut eval_list: [i32; 64] = [0; 64];
    let mut out_file: *mut FILE = 0 as *mut FILE;
    if (*node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move);
    /* With a certain probability, search the position to a variety
     of different depths in order to determine correlations. */
    if ((my_random() % 1000 as i32 as i64) as f64)
        < 1000.0f64 * spec.prob &&
        abs((*node.offset(index as isize)).black_minimax_score as
            i32) < spec.max_diff {
        display_board(stdout, board.as_mut_ptr(), 0 as i32,
                      0 as i32, 0 as i32, 0 as i32,
                      current_row,
                      black_player, black_time, black_eval,
                      white_player, white_time, white_eval,
                      &black_moves, &white_moves
        );
        setup_hash(0 as i32);
        determine_hash_values(side_to_move, board.as_mut_ptr());
        depth = 1 as i32;
        while depth <= spec.max_depth {
            middle_game::<FE>(side_to_move, depth, 0 as i32,
                        &mut dummy_info);
            eval_list[depth as usize] = root_eval;
            printf(b"%2d: %-5d \x00" as *const u8 as *const i8,
                   depth, eval_list[depth as usize]);
            depth += 2 as i32
        }
        puts(b"\x00" as *const u8 as *const i8);
        setup_hash(0 as i32);
        determine_hash_values(side_to_move, board.as_mut_ptr());
        depth = 2 as i32;
        while depth <= spec.max_depth {
            middle_game::<FE>(side_to_move, depth, 0 as i32,
                        &mut dummy_info);
            eval_list[depth as usize] = root_eval;
            printf(b"%2d: %-5d \x00" as *const u8 as *const i8,
                   depth, eval_list[depth as usize]);
            depth += 2 as i32
        }
        puts(b"\x00" as *const u8 as *const i8);
        /* Store the scores if the last eval is in the range [-20,20] */
        out_file =
            fopen(spec.out_file_name,
                  b"a\x00" as *const u8 as *const i8);
        if !out_file.is_null() &&
            abs(eval_list[spec.max_depth as usize]) <=
                20 as i32 * 128 as i32 {
            get_hash(&mut val1, &mut val2, &mut orientation);
            fprintf(out_file,
                    b"%08x%08x %2d \x00" as *const u8 as *const i8,
                    val1, val2, disks_played);
            fprintf(out_file,
                    b"%2d %2d \x00" as *const u8 as *const i8,
                    1 as i32, spec.max_depth);
            i = 1 as i32;
            while i <= spec.max_depth {
                fprintf(out_file,
                        b"%5d \x00" as *const u8 as *const i8,
                        eval_list[i as usize]);
                i += 1
            }
            fprintf(out_file, b"\n\x00" as *const u8 as *const i8);
            fclose(out_file);
        }
    }
    /* Recursively search the children of the node */
    i = 0 as i32;
    while i < move_count[disks_played as usize] {
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_midgame_statistics(child, spec);
        }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    let ref mut fresh18 = (*node.offset(index as isize)).flags;
    *fresh18 = (*fresh18 as i32 ^ 8 as i32) as u16;
}
/*
   GENERATE_MIDGAME_STATISTICS
   Calculates the minimax values of all nodes in the tree.
*/
pub unsafe fn generate_midgame_statistics(max_depth:
                                                     i32,
                                                     probability:
                                                     f64,
                                                     max_diff:
                                                     i32,
                                                     statistics_file_name:
                                                     *const i8) {
    let mut i: i32 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut spec: StatisticsSpec =
        StatisticsSpec{out_file_name: 0 as *const i8,
            prob: 0.,
            max_diff: 0,
            max_depth: 0,};
    puts(b"Generating statistics...\n\x00" as *const u8 as
        *const i8);
    prepare_tree_traversal();
    toggle_abort_check(0 as i32);
    time(&mut start_time);
    i = 0 as i32;
    while i < book_node_count {
        let ref mut fresh19 = (*node.offset(i as isize)).flags;
        *fresh19 =
            (*fresh19 as i32 | 8 as i32) as u16;
        i += 1
    }
    spec.prob = probability;
    spec.max_diff = max_diff;
    spec.max_depth = max_depth;
    spec.out_file_name = statistics_file_name;
    my_srandom(start_time as i32);
    do_midgame_statistics(0 as i32, spec);
    time(&mut stop_time);
    printf(b"\nDone (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
}
/*
   ENDGAME_CORRELATION
   Compare the scores produced by shallow searches to the
   exact score in an endgame position.
*/
unsafe extern "C" fn endgame_correlation(mut side_to_move: i32,
                                         best_score: i32,
                                         best_move: i32,
                                         min_disks: i32,
                                         max_disks: i32,
                                         spec: StatisticsSpec) {
    let mut dummy_info: EvaluationType =
        EvaluationType{type_0: MIDGAME_EVAL,
            res: WON_POSITION,
            score: 0,
            confidence: 0.,
            search_depth: 0,
            is_book: 0,};
    let mut out_file: *mut FILE = 0 as *mut FILE;
    let mut i: i32 = 0;
    let mut depth: i32 = 0;
    let mut stored_side_to_move: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut eval_list: [i32; 64] = [0; 64];
    display_board(stdout, board.as_mut_ptr(), 0 as i32,
                  0 as i32, 0 as i32, 0 as i32,
                  current_row,
                  black_player, black_time, black_eval,
                  white_player, white_time, white_eval,
                  &black_moves, &white_moves
    );
    set_hash_transformation(abs(my_random() as i32) as u32,
                            abs(my_random() as i32) as u32);
    determine_hash_values(side_to_move, board.as_mut_ptr());
    depth = 1 as i32;
    while depth <= spec.max_depth {
        middle_game::<FE>(side_to_move, depth, 0 as i32, &mut dummy_info);
        eval_list[depth as usize] = root_eval;
        printf(b"%2d: %-6.2f \x00" as *const u8 as *const i8, depth,
               eval_list[depth as usize] as f64 / 128.0f64);
        depth += 1
    }
    out_file =
        fopen(spec.out_file_name,
              b"a\x00" as *const u8 as *const i8);
    if !out_file.is_null() {
        get_hash(&mut val1, &mut val2, &mut orientation);
        fprintf(out_file,
                b"%08x%08x %2d \x00" as *const u8 as *const i8,
                val1, val2, disks_played);
        fprintf(out_file, b"%+3d \x00" as *const u8 as *const i8,
                best_score);
        fprintf(out_file, b"%2d %2d \x00" as *const u8 as *const i8,
                1 as i32, spec.max_depth);
        i = 1 as i32;
        while i <= spec.max_depth {
            fprintf(out_file, b"%5d \x00" as *const u8 as *const i8,
                    eval_list[i as usize]);
            i += 1
        }
        fprintf(out_file, b"\n\x00" as *const u8 as *const i8);
        fclose(out_file);
    }
    if disks_played < max_disks {
        make_move(side_to_move, best_move, 1 as i32);
        stored_side_to_move = side_to_move;
        side_to_move = 0 as i32 + 2 as i32 - side_to_move;
        generate_all(side_to_move);
        if move_count[disks_played as usize] > 0 as i32 {
            printf(b"\nSolving with %d empty...\n\n\x00" as *const u8 as
                       *const i8, 60 as i32 - disks_played);
            fill_move_alternatives::<FE>(side_to_move, 16 as i32);
            if get_candidate_count() > 0 as i32 ||
                disks_played >= 40 as i32 {
                print_move_alternatives(side_to_move);
                set_hash_transformation(0 as i32 as u32,
                                        0 as i32 as u32);
               end_game::<FE>(side_to_move, 0 as i32, 1 as i32,
                         1 as i32, 0 as i32, &mut dummy_info);
                endgame_correlation(side_to_move, root_eval,
                                    pv[0 as i32 as
                                        usize][0 as i32 as usize],
                                    min_disks, max_disks, spec);
            }
        }
        unmake_move(stored_side_to_move, best_move);
    };
}
/*
   DO_ENDGAME_STATISTICS
   Recursively makes sure a subtree is evaluated to
   the specified depth.
*/
unsafe extern "C" fn do_endgame_statistics(index: i32,
                                           spec: StatisticsSpec) {
    let mut dummy_info: EvaluationType =
        EvaluationType{type_0: MIDGAME_EVAL,
            res: WON_POSITION,
            score: 0,
            confidence: 0.,
            search_depth: 0,
            is_book: 0,};
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    if (*node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move);
    /* With a certain probability, search the position to a variety
     of different depths in order to determine correlations. */
    if disks_played == 33 as i32 &&
        ((my_random() % 1000 as i32 as i64) as
            f64) < 1000.0f64 * spec.prob {
        setup_hash(0 as i32);
        determine_hash_values(side_to_move, board.as_mut_ptr());
        printf(b"\nSolving with %d empty...\n\n\x00" as *const u8 as
                   *const i8, 60 as i32 - disks_played);
        fill_move_alternatives::<FE>(side_to_move, 16 as i32);
        if get_candidate_count() > 0 as i32 ||
            disks_played >= 40 as i32 {
            print_move_alternatives(side_to_move);
            set_hash_transformation(0 as i32 as u32,
                                    0 as i32 as u32);
           end_game::<FE>(side_to_move, 0 as i32, 1 as i32,
                     1 as i32, 0 as i32, &mut dummy_info);
            if abs(root_eval) <= spec.max_diff {
                endgame_correlation(side_to_move, root_eval,
                                    pv[0 as i32 as
                                        usize][0 as i32 as usize],
                                    disks_played, 48 as i32, spec);
            }
        }
    }
    /* Recursively search the children of the node */
    i = 0 as i32;
    while i < move_count[disks_played as usize] {
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_endgame_statistics(child, spec);
        }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    let ref mut fresh20 = (*node.offset(index as isize)).flags;
    *fresh20 = (*fresh20 as i32 ^ 8 as i32) as u16;
}
/*
   GENERATE_ENDGAME_STATISTICS
   Calculates the minimax values of all nodes in the tree.
*/
pub unsafe fn generate_endgame_statistics(max_depth:
                                                     i32,
                                                     probability:
                                                     f64,
                                                     max_diff:
                                                     i32,
                                                     statistics_file_name:
                                                     *const i8) {
    let mut i: i32 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut spec: StatisticsSpec =
        StatisticsSpec{out_file_name: 0 as *const i8,
            prob: 0.,
            max_diff: 0,
            max_depth: 0,};
    puts(b"Generating endgame statistics...\x00" as *const u8 as
        *const i8);
    prepare_tree_traversal();
    toggle_abort_check(0 as i32);
    time(&mut start_time);
    i = 0 as i32;
    while i < book_node_count {
        let ref mut fresh21 = (*node.offset(i as isize)).flags;
        *fresh21 =
            (*fresh21 as i32 | 8 as i32) as u16;
        i += 1
    }
    spec.prob = probability;
    spec.max_diff = max_diff;
    spec.max_depth = max_depth;
    spec.out_file_name = statistics_file_name;
    my_srandom(start_time as i32);
    do_endgame_statistics(0 as i32, spec);
    time(&mut stop_time);
    printf(b"\nDone (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
}
// #ifdef INCLUDE_BOOKTOOL
/*
   DO_CLEAR
   Clears depth and flag information for all nodes with >= LOW
   and <= HIGH discs played. FLAGS specifies what kind of information
   is to be cleared - midgame, WLD or exact.
*/
unsafe extern "C" fn do_clear(index: i32, low: i32,
                              high: i32, flags: i32) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    if (*node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if disks_played >= low && disks_played <= high {
        if flags & 1 as i32 != 0 { clear_node_depth(index); }
        if (*node.offset(index as isize)).flags as i32 &
            4 as i32 != 0 && flags & 2 as i32 != 0 {
            let ref mut fresh27 = (*node.offset(index as isize)).flags;
            *fresh27 =
                (*fresh27 as i32 ^ 4 as i32) as u16
        }
        if (*node.offset(index as isize)).flags as i32 &
            16 as i32 != 0 && flags & 4 as i32 != 0 {
            let ref mut fresh28 = (*node.offset(index as isize)).flags;
            *fresh28 =
                (*fresh28 as i32 ^ 16 as i32) as
                    u16
        }
    }
    if disks_played <= high {
        if (*node.offset(index as isize)).flags as i32 &
            1 as i32 != 0 {
            side_to_move = 0 as i32
        } else { side_to_move = 2 as i32 }
        generate_all(side_to_move);
        i = 0 as i32;
        while i < move_count[disks_played as usize] {
            this_move = move_list[disks_played as usize][i as usize];
            make_move(side_to_move, this_move, 1 as i32);
            get_hash(&mut val1, &mut val2, &mut orientation);
            slot = probe_hash_table(val1, val2);
            child = *book_hash_table.offset(slot as isize);
            if child != -(1 as i32) {
                do_clear(child, low, high, flags);
            }
            unmake_move(side_to_move, this_move);
            i += 1
        }
    }
    let ref mut fresh29 = (*node.offset(index as isize)).flags;
    *fresh29 = (*fresh29 as i32 ^ 8 as i32) as u16;
}
/*
   CLEAR_TREE
   Resets the labels on nodes satisfying certain conditions.
*/
pub unsafe fn clear_tree(low: i32,
                                    high: i32,
                                    flags: i32) {
    let mut i: i32 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    prepare_tree_traversal();
    printf(b"Clearing from %d moves to %d modes: \x00" as *const u8 as
               *const i8, low, high);
    if flags & 1 as i32 != 0 {
        printf(b"midgame \x00" as *const u8 as *const i8);
    }
    if flags & 2 as i32 != 0 {
        printf(b"wld \x00" as *const u8 as *const i8);
    }
    if flags & 4 as i32 != 0 {
        printf(b"exact \x00" as *const u8 as *const i8);
    }
    puts(b"\x00" as *const u8 as *const i8);
    time(&mut start_time);
    i = 0 as i32;
    while i < book_node_count {
        let ref mut fresh30 = (*node.offset(i as isize)).flags;
        *fresh30 =
            (*fresh30 as i32 | 8 as i32) as u16;
        i += 1
    }
    do_clear(0 as i32, low, high, flags);
    time(&mut stop_time);
    printf(b"(took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
}
/*
   DO_CORRECT
   Performs endgame correction (WLD or full solve) of a node
   and (recursively) the subtree below it.
*/
unsafe extern "C" fn do_correct(index: i32,
                                max_empty: i32,
                                full_solve: i32,
                                target_name: *const i8,
                                move_hist: *mut i8) {
    let mut dummy_info: EvaluationType =
        EvaluationType{type_0: MIDGAME_EVAL,
            res: WON_POSITION,
            score: 0,
            confidence: 0.,
            search_depth: 0,
            is_book: 0,};
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut outcome: i32 = 0;
    let mut really_evaluate: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut child_count: i32 = 0;
    let mut child_move: [i32; 64] = [0; 64];
    let mut child_node: [i32; 64] = [0; 64];
    if evaluated_count >= max_eval_count { return }
    if (*node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    /* First correct the children */
    generate_all(side_to_move);
    child_count = 0 as i32;
    i = 0 as i32;
    while i < move_count[disks_played as usize] {
        this_move = move_list[disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            child_move[child_count as usize] = this_move;
            child_node[child_count as usize] = child;
            child_count += 1
        }
        unmake_move(side_to_move, this_move);
        i += 1
    }
    let mut current_block_29: u64;
    i = 0 as i32;
    while i < child_count {
        if side_to_move == 0 as i32 {
            if force_black != 0 &&
                (*node.offset(child_node[i as usize] as
                    isize)).black_minimax_score as
                    i32 !=
                    (*node.offset(index as isize)).black_minimax_score as
                        i32 {
                current_block_29 = 14818589718467733107;
            } else { current_block_29 = 11913429853522160501; }
        } else if force_white != 0 &&
            (*node.offset(child_node[i as usize] as
                isize)).white_minimax_score as
                i32 !=
                (*node.offset(index as isize)).white_minimax_score
                    as i32 {
            current_block_29 = 14818589718467733107;
        } else { current_block_29 = 11913429853522160501; }
        match current_block_29 {
            11913429853522160501 => {
                this_move = child_move[i as usize];
                sprintf(move_hist.offset((2 as i32 * disks_played) as
                    isize),
                        b"%c%c\x00" as *const u8 as *const i8,
                        'a' as i32 + this_move % 10 as i32 -
                            1 as i32,
                        '0' as i32 + this_move / 10 as i32);
                make_move(side_to_move, this_move, 1 as i32);
                do_correct(child_node[i as usize], max_empty, full_solve,
                           target_name, move_hist);
                unmake_move(side_to_move, this_move);
                *move_hist.offset((2 as i32 * disks_played) as isize)
                    = '\u{0}' as i32 as i8
            }
            _ => { }
        }
        i += 1
    }
    /* Then correct the node itself (hopefully exploiting lots
     of useful information in the hash table) */
    generate_all(side_to_move);
    determine_hash_values(side_to_move, board.as_mut_ptr());
    if disks_played >= 60 as i32 - max_empty {
        really_evaluate =
            (full_solve != 0 &&
                (*node.offset(index as isize)).flags as i32 &
                    16 as i32 == 0 ||
                full_solve == 0 &&
                    (*node.offset(index as isize)).flags as i32 &
                        (4 as i32 | 16 as i32) == 0) as
                i32;
        if abs((*node.offset(index as isize)).alternative_score as
            i32) < min_eval_span ||
            abs((*node.offset(index as isize)).alternative_score as
                i32) > max_eval_span {
            really_evaluate = 0 as i32
        }
        if abs((*node.offset(index as isize)).black_minimax_score as
            i32) < min_negamax_span ||
            abs((*node.offset(index as isize)).black_minimax_score as
                i32) > max_negamax_span {
            really_evaluate = 0 as i32
        }
        if really_evaluate != 0 {
            if target_name.is_null() {
                /* Solve now */
                reset_counter(&mut nodes);
               end_game::<FE>(side_to_move, (full_solve == 0) as i32,
                         0 as i32, 1 as i32, 0 as i32,
                         &mut dummy_info);
                if side_to_move == 0 as i32 {
                    outcome = root_eval
                } else { outcome = -root_eval }
                let ref mut fresh31 =
                    (*node.offset(index as isize)).white_minimax_score;
                *fresh31 = outcome as i16;
                (*node.offset(index as isize)).black_minimax_score = *fresh31;
                if outcome > 0 as i32 {
                    let ref mut fresh32 =
                        (*node.offset(index as isize)).black_minimax_score;
                    *fresh32 =
                        (*fresh32 as i32 + 30000 as i32) as
                            i16;
                    let ref mut fresh33 =
                        (*node.offset(index as isize)).white_minimax_score;
                    *fresh33 =
                        (*fresh33 as i32 + 30000 as i32) as
                            i16
                }
                if outcome < 0 as i32 {
                    let ref mut fresh34 =
                        (*node.offset(index as isize)).black_minimax_score;
                    *fresh34 =
                        (*fresh34 as i32 - 30000 as i32) as
                            i16;
                    let ref mut fresh35 =
                        (*node.offset(index as isize)).white_minimax_score;
                    *fresh35 =
                        (*fresh35 as i32 - 30000 as i32) as
                            i16
                }
                if full_solve != 0 {
                    let ref mut fresh36 =
                        (*node.offset(index as isize)).flags;
                    *fresh36 =
                        (*fresh36 as i32 | 16 as i32) as
                            u16
                } else {
                    let ref mut fresh37 =
                        (*node.offset(index as isize)).flags;
                    *fresh37 =
                        (*fresh37 as i32 | 4 as i32) as
                            u16
                }
            } else {
                /* Defer solving to a standalone scripted solver */
                let target_file: *mut FILE =
                    fopen(target_name,
                          b"a\x00" as *const u8 as *const i8);
                if !target_file.is_null() {
                    fprintf(target_file,
                            b"%% %s\n\x00" as *const u8 as
                                *const i8, move_hist);
                    get_hash(&mut val1, &mut val2, &mut orientation);
                    fprintf(target_file,
                            b"%% %d %d\n\x00" as *const u8 as
                                *const i8, val1, val2);
                    i = 1 as i32;
                    while i <= 8 as i32 {
                        j = 1 as i32;
                        while j <= 8 as i32 {
                            pos = 10 as i32 * i + j;
                            if board[pos as usize] == 0 as i32 {
                                putc('X' as i32, target_file);
                            } else if board[pos as usize] == 2 as i32
                            {
                                putc('O' as i32, target_file);
                            } else { putc('-' as i32, target_file); }
                            j += 1
                        }
                        i += 1
                    }
                    if side_to_move == 0 as i32 {
                        fputs(b" X\n\x00" as *const u8 as *const i8,
                              target_file);
                    } else {
                        fputs(b" O\n\x00" as *const u8 as *const i8,
                              target_file);
                    }
                    fputs(b"%\n\x00" as *const u8 as *const i8,
                          target_file);
                    fclose(target_file);
                }
            }
            evaluated_count += 1
        }
    }
    if evaluated_count >=
        (evaluation_stage + 1 as i32) * max_eval_count /
            25 as i32 {
        evaluation_stage += 1;
        putc('|' as i32, stdout);
        if evaluation_stage % 5 as i32 == 0 as i32 {
            printf(b" %d%% \x00" as *const u8 as *const i8,
                   4 as i32 * evaluation_stage);
        }
        fflush(stdout);
    }
    let ref mut fresh38 = (*node.offset(index as isize)).flags;
    *fresh38 = (*fresh38 as i32 ^ 8 as i32) as u16;
}
/*
  SET_OUTPUT_SCRIPT_NAME
  Makes SCRIPT_NAME the target for the positions generated by
  do_correct() (instead of the positions being solved, the normal
  mode of operation).
*/
pub unsafe fn set_output_script_name(script_name:
                                                *const i8) {
    correction_script_name = script_name;
}
/*
   CORRECT_TREE
   Endgame-correct the lowest levels of the tree.
*/
pub unsafe fn correct_tree(max_empty: i32,
                                      full_solve: i32) {
    let mut move_buffer: [i8; 150] = [0; 150];
    let mut i: i32 = 0;
    let mut feasible_count: i32 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    prepare_tree_traversal();
    exhausted_node_count = 0 as i32;
    evaluated_count = 0 as i32;
    evaluation_stage = 0 as i32;
    time(&mut start_time);
    i = 0 as i32;
    while i < book_node_count {
        let ref mut fresh39 = (*node.offset(i as isize)).flags;
        *fresh39 =
            (*fresh39 as i32 | 8 as i32) as u16;
        i += 1
    }
    feasible_count = 0 as i32;
    i = 0 as i32;
    while i < book_node_count {
        let ref mut fresh40 = (*node.offset(i as isize)).flags;
        *fresh40 =
            (*fresh40 as i32 | 8 as i32) as u16;
        if get_node_depth(i) < max_empty &&
            abs((*node.offset(i as isize)).alternative_score as
                i32) >= min_eval_span &&
            abs((*node.offset(i as isize)).alternative_score as
                i32) <= max_eval_span &&
            abs((*node.offset(i as isize)).black_minimax_score as
                i32) >= min_negamax_span &&
            abs((*node.offset(i as isize)).black_minimax_score as
                i32) <= max_negamax_span {
            feasible_count += 1
        }
        i += 1
    }
    max_eval_count =
        if feasible_count < max_batch_size {
            feasible_count
        } else { max_batch_size };
    printf(b"Correcting <= %d empty \x00" as *const u8 as *const i8,
           max_empty);
    if full_solve != 0 {
        printf(b"(full solve). \x00" as *const u8 as *const i8);
    } else {
        printf(b"(WLD solve). \x00" as *const u8 as *const i8);
    }
    if min_eval_span > 0 as i32 ||
        max_eval_span < 1000 as i32 * 128 as i32 {
        printf(b"Eval interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const i8,
               min_eval_span as f64 / 128.0f64,
               max_eval_span as f64 / 128.0f64);
    }
    if min_negamax_span > 0 as i32 ||
        max_negamax_span < 1000 as i32 * 128 as i32 {
        printf(b"Negamax interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const i8,
               min_negamax_span as f64 / 128.0f64,
               max_negamax_span as f64 / 128.0f64);
    }
    if max_eval_count == feasible_count {
        printf(b"\n%d relevant nodes.\x00" as *const u8 as
                   *const i8, feasible_count);
    } else {
        printf(b"\nMax batch size is %d.\x00" as *const u8 as
                   *const i8, max_batch_size);
    }
    puts(b"\x00" as *const u8 as *const i8);
    printf(b"Progress: \x00" as *const u8 as *const i8);
    fflush(stdout);
    move_buffer[0 as i32 as usize] = '\u{0}' as i32 as i8;
    do_correct(0 as i32, max_empty, full_solve,
               correction_script_name, move_buffer.as_mut_ptr());
    time(&mut stop_time);
    printf(b"(took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    if correction_script_name.is_null() {
        /* Positions solved */
        printf(b"%d nodes solved\n\x00" as *const u8 as *const i8,
               evaluated_count);
    } else {
        printf(b"%d nodes exported to %s\n\x00" as *const u8 as
                   *const i8, evaluated_count,
               correction_script_name);
    }
    puts(b"\x00" as *const u8 as *const i8);
}
/*
   DO_EXPORT
   Recursively exports all variations rooted at book position # INDEX.
*/
unsafe extern "C" fn do_export(index: i32, stream: *mut FILE,
                               move_vec: &mut [i32; 60]) {
    let mut i: i32 = 0;
    let mut child_count: i32 = 0;
    let mut allow_branch: i32 = 0;
    let mut side_to_move: i32 = 0;
    allow_branch =
        (*node.offset(index as isize)).flags as i32 &
            8 as i32;
    if (*node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move);
    child_count = 0 as i32;
    i = 0 as i32;
    while i < move_count[disks_played as usize] {
        let mut child: i32 = 0;
        let mut slot: i32 = 0;
        let mut val1: i32 = 0;
        let mut val2: i32 = 0;
        let mut orientation: i32 = 0;
        let this_move: i32 =
            move_list[disks_played as usize][i as usize];
        *(move_vec.as_mut_ptr()).offset(disks_played as isize) = this_move;
        make_move(side_to_move, this_move, 1 as i32);
        get_hash(&mut val1, &mut val2, &mut orientation);
        slot = probe_hash_table(val1, val2);
        child = *book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_export(child, stream, move_vec);
            child_count += 1
        }
        unmake_move(side_to_move, this_move);
        if child_count == 1 as i32 && allow_branch == 0 { break ; }
        i += 1
    }
    if child_count == 0 as i32 {
        /* We've reached a leaf in the opening tree. */
        i = 0 as i32;
        while i < disks_played {
            fprintf(stream, b"%c%c\x00" as *const u8 as *const i8,
                    'a' as i32 +
                        *(move_vec.as_mut_ptr()).offset(i as isize) % 10 as i32 -
                        1 as i32,
                    '0' as i32 +
                        *(move_vec.as_mut_ptr()).offset(i as isize) / 10 as i32);
            i += 1
        }
        fprintf(stream, b"\n\x00" as *const u8 as *const i8);
    }
    let ref mut fresh41 = (*node.offset(index as isize)).flags;
    *fresh41 =
        (*fresh41 as i32 & !(8 as i32)) as u16;
}

/*
  EXPORT_TREE
  Exports a set of lines that cover the tree.
*/
pub unsafe fn export_tree(file_name: *const i8) {
    let stream = fopen(file_name, b"w\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fprintf(stderr,
                b"Cannot open %s for writing.\n\x00" as *const u8 as
                    *const i8, file_name);
        return
    }
    prepare_tree_traversal();
    let mut move_vec: [i32; 60] = [0; 60];
    let mut i = 0 as i32;
    while i < book_node_count {
        let ref mut fresh42 = (*node.offset(i as isize)).flags;
        *fresh42 =
            (*fresh42 as i32 | 8 as i32) as u16;
        i += 1
    }
    do_export(0 as i32, stream, &mut move_vec);
    fclose(stream);
}
