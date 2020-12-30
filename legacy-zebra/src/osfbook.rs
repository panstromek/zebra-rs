use crate::{
    src::{
        game::{global_setup, game_init},
        error::fatal_error
    }
};
use crate::src::display::{display_board, white_eval, white_time, white_player, black_eval, black_time, black_player, current_row};
use engine::src::midgame::{middle_game, tree_search, midgame_state};
use engine::src::myrandom::{random_instance, MyRandom};
use engine::src::hash::{setup_hash, clear_hash_drafts, hash_state, determine_hash_values};
use engine::src::error::{FrontEnd};
use crate::src::error::{LibcFatalError};
use engine::src::globals::board_state;
use engine::src::moves::{make_move, make_move_no_hash, moves_state, unmake_move, generate_all, generate_specific, unmake_move_no_hash};
use engine::src::stubs::{abs, floor};
use engine::src::search::{disc_count, search_state};
use engine::src::end::end_game;
use engine::src::counter::reset_counter;
use engine::src::zebra::{EvaluationType};
use engine::src::timer::{g_timer};
use crate::src::safemem::safe_malloc;
use libc_wrapper::{fclose, fprintf, fopen, puts, printf, time, fflush, putc, fputs, sprintf, free, fputc, strstr, toupper, __ctype_b_loc, strlen, sscanf, fgets, ctime, strcpy, malloc, feof, strcmp, fwrite, fread, fscanf, qsort, stdout, stderr, exit, FILE};
use engine::src::osfbook::{__time_t, probe_hash_table, get_hash, get_node_depth, clear_node_depth, fill_move_alternatives, _ISupper, _ISprint, _ISspace, _ISgraph, BookNode, adjust_score, g_book, size_t, set_node_depth, Book, reset_book_search};
use engine_traits::Offset;
use engine::src::getcoeff::{remove_coeffs, coeff_state};
use engine::src::game::{engine_game_init, setup_non_file_based_game};
use engine::src::zebra::GameMode::PRIVATE_GAME;
use engine::src::zebra::EvalResult::WON_POSITION;
use engine::src::zebra::EvalType::MIDGAME_EVAL;
use crate::src::zebra::g_config;
use flip::unflip::flip_stack_;

pub type FE = LibcFatalError;
static mut correction_script_name: *const i8 = 0 as *const i8;

pub type _IO_lock_t = ();
pub type time_t = __time_t;

/*
   MINIMAX_TREE
   Calculates the minimax values of all nodes in the tree.
*/

pub unsafe fn minimax_tree() {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    printf(b"Calculating minimax value... \x00" as *const u8 as
               *const i8);
    fflush(stdout);
    prepare_tree_traversal();
    time(&mut start_time);
    engine_minimax_tree();
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8, stop_time - start_time);
    puts(b"\x00" as *const u8 as *const i8);
}

/*
   EVALUATE_TREE
   Finds the most promising deviations from all nodes in the tree.
*/

pub unsafe fn evaluate_tree() {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    prepare_tree_traversal();
    g_book.exhausted_node_count = 0;
    g_book.evaluated_count = 0;
    g_book.evaluation_stage = 0;
    time(&mut start_time);
    let feasible_count = compute_feasible_count();
    g_book.max_eval_count =
        if feasible_count < g_book.max_batch_size {
            feasible_count
        } else { g_book.max_batch_size };
    printf(b"Evaluating to depth %d. \x00" as *const u8 as
               *const i8, g_book.search_depth);
    if g_book.min_eval_span > 0 as i32 ||
           g_book.max_eval_span < 1000 as i32 * 128 as i32 {
        printf(b"Eval interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const i8,
               g_book.min_eval_span as f64 / 128.0f64,
               g_book.max_eval_span as f64 / 128.0f64);
    }
    if g_book.min_negamax_span > 0 as i32 ||
           g_book.max_negamax_span < 1000 as i32 * 128 as i32 {
        printf(b"Negamax interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const i8,
               g_book.min_negamax_span as f64 / 128.0f64,
               g_book.max_negamax_span as f64 / 128.0f64);
    }
    if g_book.max_eval_count == feasible_count {
        printf(b"\n%d relevant nodes.\x00" as *const u8 as
                   *const i8, feasible_count);
    } else {
        printf(b"\nMax batch size is %d.\x00" as *const u8 as
                   *const i8, g_book.max_batch_size);
    }
    puts(b"\x00" as *const u8 as *const i8);
    printf(b"Progress: \x00" as *const u8 as *const i8);
    fflush(stdout);
    if feasible_count > 0 as i32 { do_evaluate::<LibcFatalError>(0 as i32, g_config.echo ); }
    time(&mut stop_time);
    printf(b"(took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    printf(b"%d nodes evaluated \x00" as *const u8 as *const i8,
           g_book.evaluated_count);
    printf(b"(%d exhausted nodes ignored)\n\x00" as *const u8 as
               *const i8, g_book.exhausted_node_count);
    puts(b"\x00" as *const u8 as *const i8);
}
/*
   EXAMINE_TREE
   Generates some statistics about the book tree.
*/

pub unsafe fn examine_tree() {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    printf(b"Examining tree... \x00" as *const u8 as *const i8);
    fflush(stdout);
    prepare_tree_traversal();
    time(&mut start_time);
    engine_examine_tree();
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
        safe_malloc((g_book.book_node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                         as u64)) as
            *mut i32;
    negamax =
        safe_malloc((g_book.book_node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                         as u64)) as
            *mut i32;
    wld_solved = 0;
    full_solved = wld_solved;
    eval_count = 0;
    negamax_count = 0;
    private_count = 0;
    unevaluated = 0;
    i = 0;
    while i < 60 as i32 {
        depth[i as usize] = 0;
        i += 1
    }
    i = 0;
    while i < g_book.book_node_count {
        if (*g_book.node.offset(i as isize)).flags as i32 & 16 as i32
               != 0 {
            full_solved += 1
        } else if (*g_book.node.offset(i as isize)).flags as i32 &
                      4 as i32 != 0 {
            wld_solved += 1
        } else {
            depth[get_node_depth(i, &mut g_book) as usize] += 1;
            if (*g_book.node.offset(i as isize)).alternative_score as i32 ==
                   9999 as i32 &&
                   (*g_book.node.offset(i as isize)).best_alternative_move as
                       i32 == -(1 as i32) {
                unevaluated += 1
            } else {
                if (*g_book.node.offset(i as isize)).alternative_score as i32
                       != 9999 as i32 {
                    let fresh24 = eval_count;
                    eval_count = eval_count + 1;
                    *evals.offset(fresh24 as isize) =
                        abs((*g_book.node.offset(i as isize)).alternative_score as
                                i32)
                }
                let fresh25 = negamax_count;
                negamax_count = negamax_count + 1;
                *negamax.offset(fresh25 as isize) =
                    abs((*g_book.node.offset(i as isize)).black_minimax_score as
                            i32)
            }
        }
        if (*g_book.node.offset(i as isize)).flags as i32 & 32 as i32
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
           g_book.book_node_count);
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
    i = 0;
    while i <= 59 as i32 {
        if depth[i as usize] > 0 as i32 {
            printf(b"#nodes with %2d-ply deviations: %d\n\x00" as *const u8 as
                       *const i8, i, depth[i as usize]);
        }
        i += 1
    }
    puts(b"\x00" as *const u8 as *const i8);
    this_strata = 0;
    strata_shift =
        floor(strata[this_strata as usize] * eval_count as f64) as
            i32;
    i = 0;
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
    this_strata = 0;
    strata_shift =
        floor(strata[this_strata as usize] * negamax_count as f64)
            as i32;
    i = 0;
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
    i = 0;
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
        first = 61;
        last = -(1 as i32);
        i = 0;
        while i <= 60 as i32 {
            total_count[i as usize] =
                g_book.exact_count[i as usize] + g_book.wld_count[i as usize] +
                    g_book.exhausted_count[i as usize] + g_book.common_count[i as usize];
            if total_count[i as usize] > 0 as i32 {
                first = if first < i { first } else { i };
                last = if last > i { last } else { i }
            }
            i += 1
        }
        printf(b"%d unreachable nodes\n\n\x00" as *const u8 as
                   *const i8, g_book.unreachable_count);
        printf(b"%d leaf nodes; %d lack exact score and %d lack WLD status\n\x00"
                   as *const u8 as *const i8, g_book.leaf_count,
               g_book.bad_leaf_count, g_book.really_bad_leaf_count);
        i = first;
        while i <= last {
            printf(b"%2d moves\x00" as *const u8 as *const i8, i);
            printf(b"   \x00" as *const u8 as *const i8);
            printf(b"%5d g_book.node\x00" as *const u8 as *const i8,
                   total_count[i as usize]);
            if total_count[i as usize] == 1 as i32 {
                printf(b" :  \x00" as *const u8 as *const i8);
            } else {
                printf(b"s:  \x00" as *const u8 as *const i8);
            }
            if g_book.common_count[i as usize] > 0 as i32 {
                printf(b"%5d midgame\x00" as *const u8 as *const i8,
                       g_book.common_count[i as usize]);
            } else {
                printf(b"             \x00" as *const u8 as
                           *const i8);
            }
            printf(b"  \x00" as *const u8 as *const i8);
            if g_book.wld_count[i as usize] > 0 as i32 {
                printf(b"%5d WLD\x00" as *const u8 as *const i8,
                       g_book.wld_count[i as usize]);
            } else {
                printf(b"         \x00" as *const u8 as *const i8);
            }
            printf(b"  \x00" as *const u8 as *const i8);
            if g_book.exact_count[i as usize] > 0 as i32 {
                printf(b"%5d exact\x00" as *const u8 as *const i8,
                       g_book.exact_count[i as usize]);
            } else {
                printf(b"           \x00" as *const u8 as
                           *const i8);
            }
            printf(b"  \x00" as *const u8 as *const i8);
            if g_book.exhausted_count[i as usize] > 0 as i32 {
                printf(b"%2d exhausted\x00" as *const u8 as
                           *const i8, g_book.exhausted_count[i as usize]);
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
            g_book.node[0].black_minimax_score as
                i32;
        printf(b"black\x00" as *const u8 as *const i8);
    } else {
        root_score =
            g_book.node[0].white_minimax_score as
                i32;
        printf(b"white\x00" as *const u8 as *const i8);
    }
    printf(b": %+.2f\n\x00" as *const u8 as *const i8,
           root_score as f64 / 128.0f64);
    current = 0;
    puts(b"Preferred line: \x00" as *const u8 as *const i8);
    line = 0;
    done = 0;
    show_move = 1;
    while (*g_book.node.offset(current as isize)).flags as i32 &
              (16 as i32 | 4 as i32) == 0 && done == 0 {
        if (*g_book.node.offset(current as isize)).flags as i32 &
               1 as i32 != 0 {
            side_to_move = 0 as i32
        } else { side_to_move = 2 as i32 }
        generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
        next = -(1 as i32);
        this_move = -(1 as i32);
        i = 0;
        while i < moves_state.move_count[moves_state.disks_played as usize] {
            let val0___ = &mut val1;
            let val1___ = &mut val2;
            let orientation___ = &mut base_orientation;
            get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
            this_move = moves_state.move_list[moves_state.disks_played as usize][i as usize];
            make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
            let val0___ = &mut val1;
            let val1___ = &mut val2;
            let orientation___ = &mut child_orientation;
            get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
            slot = probe_hash_table(val1, val2, &mut g_book);
            child = *g_book.book_hash_table.offset(slot as isize);
            if child != -(1 as i32) {
                if original_side_to_move == 0 as i32 {
                    child_score =
                        (*g_book.node.offset(child as isize)).black_minimax_score as
                            i32
                } else {
                    child_score =
                        (*g_book.node.offset(child as isize)).white_minimax_score as
                            i32
                }
                if child_score == root_score { next = child }
            }
            if child != -(1 as i32) && next == child { break ; }
            let move_0 = this_move;
            {
                unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
            };
            i += 1
        }
        if next == -(1 as i32) {
            done = 1;
            if adjust_score((*g_book.node.offset(current as isize)).alternative_score
                                as i32, side_to_move, &mut g_book, moves_state.disks_played) != root_score {
                puts(b"(failed to find continuation)\x00" as *const u8 as
                         *const i8);
                show_move = 0 as i32
            } else {
                this_move =
                    (*g_book.node.offset(current as isize)).best_alternative_move as
                        i32;
                this_move =
                    *g_book.inv_symmetry_map[base_orientation as
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
                                      private_game: i32, mut echo:i32) {
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
    echo = 0;
    /* First create new nodes for new positions */
    prepare_tree_traversal();
    i = 0;
    while i < move_count_0 {
        if *game_move_list.offset(i as isize) as i32 >
               0 as i32 {
            flags[i as usize] = 1 as i32 as u16
        } else { flags[i as usize] = 2 as i32 as u16 }
        i += 1
    }
    flags[move_count_0 as usize] = 0;
    first_new_node = 61;
    this_node = 0;
    side_to_move = 0;
    last_move_number =
        if move_count_0 < 60 as i32 - min_empties {
            move_count_0
        } else { (60 as i32) - min_empties };
    i = 0;
    while i <= last_move_number {
        /* Look for the position in the hash table */
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        if slot == -(1 as i32) ||
               *g_book.book_hash_table.offset(slot as isize) == -(1 as i32) {
            this_node = create_BookNode(val1, val2, flags[i as usize], &mut g_book);
            if private_game != 0 {
                let ref mut fresh26 =
                    (*g_book.node.offset(this_node as isize)).flags;
                *fresh26 =
                    (*fresh26 as i32 | 32 as i32) as
                        u16
            }
            if i < first_new_node { first_new_node = i }
        } else { this_node = *g_book.book_hash_table.offset(slot as isize) }
        visited_node[i as usize] = this_node;
        /* Make the moves of the game until the cutoff point */
        if i < last_move_number {
            this_move =
                abs(*game_move_list.offset(i as isize) as i32);
            if *game_move_list.offset(i as isize) as i32 >
                   0 as i32 {
                side_to_move = 0 as i32
            } else { side_to_move = 2 as i32 }
            if generate_specific(this_move, side_to_move, &board_state.board) == 0 {
                puts(b"\x00" as *const u8 as *const i8);
                printf(b"i=%d, side_to_move=%d, this_move=%d\n\x00" as
                           *const u8 as *const i8, i, side_to_move,
                       this_move);
                printf(b"last_move_number=%d, move_count=%d\n\x00" as
                           *const u8 as *const i8, last_move_number,
                       move_count_0);
                j = 0;
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
            make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
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
        black_count = disc_count(0 as i32, &board_state.board);
        white_count = disc_count(2 as i32, &board_state.board);
        if black_count > white_count {
            outcome = 64 as i32 - 2 as i32 * white_count
        } else if white_count > black_count {
            outcome = 2 as i32 * black_count - 64 as i32
        } else { outcome = 0 as i32 }
    } else {
        generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
        determine_hash_values(side_to_move, &board_state.board, &mut hash_state);
        if echo != 0 {
            puts(b"\x00" as *const u8 as *const i8);
            if side_to_move == 0 as i32 {
                printf(b"Full solving with %d empty (black)\n\x00" as
                           *const u8 as *const i8,
                       60 as i32 - moves_state.disks_played);
            } else {
                printf(b"Full solving with %d empty (white)\n\x00" as
                           *const u8 as *const i8,
                       60 as i32 - moves_state.disks_played);
            }
        }
       end_game::<FE>(side_to_move, 0 as i32, 0 as i32,
                 1 as i32, 0 as i32, &mut dummy_info, echo);
        outcome = search_state.root_eval;
        if side_to_move == 2 as i32 { outcome = -outcome }
    }
    (*g_book.node.offset(this_node as isize)).black_minimax_score =
        outcome as i16;
    (*g_book.node.offset(this_node as isize)).white_minimax_score =
        outcome as i16;
    if outcome > 0 as i32 {
        let ref mut fresh27 =
            (*g_book.node.offset(this_node as isize)).black_minimax_score;
        *fresh27 =
            (*fresh27 as i32 + 30000 as i32) as i16;
        let ref mut fresh28 =
            (*g_book.node.offset(this_node as isize)).white_minimax_score;
        *fresh28 =
            (*fresh28 as i32 + 30000 as i32) as i16
    }
    if outcome < 0 as i32 {
        let ref mut fresh29 =
            (*g_book.node.offset(this_node as isize)).black_minimax_score;
        *fresh29 =
            (*fresh29 as i32 - 30000 as i32) as i16;
        let ref mut fresh30 =
            (*g_book.node.offset(this_node as isize)).white_minimax_score;
        *fresh30 =
            (*fresh30 as i32 - 30000 as i32) as i16
    }
    let ref mut fresh31 = (*g_book.node.offset(this_node as isize)).flags;
    *fresh31 =
        (*fresh31 as i32 | 16 as i32) as u16;
    /* Take another pass through the midgame to update move
       alternatives and minimax information if requested. */
    if update_path != 0 {
        prepare_tree_traversal();
        i = 0;
        while i < last_move_number {
            this_move =
                abs(*game_move_list.offset(i as isize) as i32);
            if *game_move_list.offset(i as isize) as i32 >
                   0 as i32 {
                side_to_move = 0 as i32
            } else { side_to_move = 2 as i32 }
            if generate_specific(this_move, side_to_move, &board_state.board) == 0 {
                fatal_error(b"%s: %d\n\x00" as *const u8 as
                                *const i8,
                            b"Invalid move generated\x00" as *const u8 as
                                *const i8, this_move);
            }
            make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
            i += 1
        }
        if echo != 0 { fflush(stdout); }
        midgame_eval_done = 0;
        i = last_move_number - 1 as i32;
        while i >= 0 as i32 {
            this_move =
                abs(*game_move_list.offset(i as isize) as i32);
            if *game_move_list.offset(i as isize) as i32 >
                   0 as i32 {
                side_to_move = 0 as i32
            } else { side_to_move = 2 as i32 }
            let move_0 = this_move;
            {
                unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
            };
            /* If the game was public, make sure that all nodes that
            previously marked as private nodes are marked as public. */
            this_node = visited_node[i as usize];
            if private_game == 0 &&
                   (*g_book.node.offset(this_node as isize)).flags as i32 &
                       32 as i32 != 0 {
                let ref mut fresh32 =
                    (*g_book.node.offset(this_node as isize)).flags;
                *fresh32 =
                    (*fresh32 as i32 ^ 32 as i32) as
                        u16
            }
            if (*g_book.node.offset(this_node as isize)).flags as i32 &
                   1 as i32 != 0 {
                side_to_move = 0 as i32
            } else { side_to_move = 2 as i32 }
            generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
            determine_hash_values(side_to_move, &board_state.board, &mut hash_state);
            if moves_state.disks_played >= 60 as i32 - max_full_solve {
                /* Only solve the position if it hasn't been solved already */
                if (*g_book.node.offset(this_node as isize)).flags as i32 &
                       16 as i32 == 0 {
                   end_game::<FE>(side_to_move, 0 as i32, 0 as i32,
                             1 as i32, 0 as i32,
                             &mut dummy_info, echo);
                    if side_to_move == 0 as i32 {
                        outcome = search_state.root_eval
                    } else { outcome = -search_state.root_eval }
                    (*g_book.node.offset(this_node as isize)).black_minimax_score =
                        outcome as i16;
                    (*g_book.node.offset(this_node as isize)).white_minimax_score =
                        outcome as i16;
                    if outcome > 0 as i32 {
                        let ref mut fresh33 =
                            (*g_book.node.offset(this_node as
                                              isize)).black_minimax_score;
                        *fresh33 =
                            (*fresh33 as i32 + 30000 as i32)
                                as i16;
                        let ref mut fresh34 =
                            (*g_book.node.offset(this_node as
                                              isize)).white_minimax_score;
                        *fresh34 =
                            (*fresh34 as i32 + 30000 as i32)
                                as i16
                    }
                    if outcome < 0 as i32 {
                        let ref mut fresh35 =
                            (*g_book.node.offset(this_node as
                                              isize)).black_minimax_score;
                        *fresh35 =
                            (*fresh35 as i32 - 30000 as i32)
                                as i16;
                        let ref mut fresh36 =
                            (*g_book.node.offset(this_node as
                                              isize)).white_minimax_score;
                        *fresh36 =
                            (*fresh36 as i32 - 30000 as i32)
                                as i16
                    }
                    let ref mut fresh37 =
                        (*g_book.node.offset(this_node as isize)).flags;
                    *fresh37 =
                        (*fresh37 as i32 | 16 as i32) as
                            u16
                }
            } else if moves_state.disks_played >= 60 as i32 - max_wld_solve {
                /* Only solve the position if its WLD status is unknown */
                if (*g_book.node.offset(this_node as isize)).flags as i32 &
                       4 as i32 == 0 {
                   end_game::<FE>(side_to_move, 1 as i32, 0 as i32,
                             1 as i32, 0 as i32,
                             &mut dummy_info, echo);
                    if side_to_move == 0 as i32 {
                        outcome = search_state.root_eval
                    } else { outcome = -search_state.root_eval }
                    (*g_book.node.offset(this_node as isize)).black_minimax_score =
                        outcome as i16;
                    (*g_book.node.offset(this_node as isize)).white_minimax_score =
                        outcome as i16;
                    if outcome > 0 as i32 {
                        let ref mut fresh38 =
                            (*g_book.node.offset(this_node as
                                              isize)).black_minimax_score;
                        *fresh38 =
                            (*fresh38 as i32 + 30000 as i32)
                                as i16;
                        let ref mut fresh39 =
                            (*g_book.node.offset(this_node as
                                              isize)).white_minimax_score;
                        *fresh39 =
                            (*fresh39 as i32 + 30000 as i32)
                                as i16
                    }
                    if outcome < 0 as i32 {
                        let ref mut fresh40 =
                            (*g_book.node.offset(this_node as
                                              isize)).black_minimax_score;
                        *fresh40 =
                            (*fresh40 as i32 - 30000 as i32)
                                as i16;
                        let ref mut fresh41 =
                            (*g_book.node.offset(this_node as
                                              isize)).white_minimax_score;
                        *fresh41 =
                            (*fresh41 as i32 - 30000 as i32)
                                as i16
                    }
                    let ref mut fresh42 =
                        (*g_book.node.offset(this_node as isize)).flags;
                    *fresh42 =
                        (*fresh42 as i32 | 4 as i32) as
                            u16
                }
            } else {
                force_eval =
                    (i >= first_new_node - 1 as i32 ||
                         (*g_book.node.offset(this_node as
                                           isize)).best_alternative_move as
                             i32 ==
                             abs(*game_move_list.offset(i as isize) as
                                     i32)) as i32;
                if midgame_eval_done == 0 {
                    printf(b"Evaluating: \x00" as *const u8 as
                               *const i8);
                    fflush(stdout);
                }
                midgame_eval_done = 1;
                if force_eval != 0 { clear_node_depth(this_node, &mut g_book); }
                evaluate_node::<LibcFatalError>(this_node, echo);
                printf(b"|\x00" as *const u8 as *const i8);
                fflush(stdout);
            }
            let ref mut fresh43 = (*g_book.node.offset(this_node as isize)).flags;
            *fresh43 =
                (*fresh43 as i32 | 8 as i32) as
                    u16;
            do_minimax(this_node, &mut dummy_black_score,
                       &mut dummy_white_score);
            if (*g_book.node.offset(this_node as isize)).flags as i32 &
                   4 as i32 == 0 &&
                   (*g_book.node.offset(this_node as isize)).best_alternative_move as
                       i32 == -(1 as i32) &&
                   (*g_book.node.offset(this_node as isize)).alternative_score as
                       i32 == 9999 as i32 {
                /* Minimax discovered that the g_book.node hasn't got a deviation any
                   longer because that move has been played. */
                evaluate_node::<FE>(this_node, echo);
                printf(b"-|-\x00" as *const u8 as *const i8);
                do_minimax(this_node, &mut dummy_black_score,
                           &mut dummy_white_score);
            }
            i -= 1
        }
        puts(b"\x00" as *const u8 as *const i8);
    }
    echo = stored_echo;
    g_book.total_game_count += 1;
}
/*
   BUILD_TREE
   Reads games from the file pointed to by FILE_NAME and
   incorporates them into the game tree.
*/

pub unsafe fn build_tree(file_name: *const i8,
                                    max_game_count: i32,
                                    max_diff: i32,
                                    min_empties: i32, echo:i32) {
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
    games_parsed = 0;
    games_imported = 0;
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
        i = 0;
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
                         0 as i32, 0 as i32, echo);
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
    set_allocation(new_book_node_count + 1000 as i32, &mut g_book);
    i = 0;
    while i < new_book_node_count {
        fscanf(stream,
               b"%d %d %hd %hd %hd %hd %hd\n\x00" as *const u8 as
                   *const i8,
               &mut (*g_book.node.offset(i as isize)).hash_val1 as *mut i32,
               &mut (*g_book.node.offset(i as isize)).hash_val2 as *mut i32,
               &mut (*g_book.node.offset(i as isize)).black_minimax_score as
                   *mut i16,
               &mut (*g_book.node.offset(i as isize)).white_minimax_score as
                   *mut i16,
               &mut (*g_book.node.offset(i as isize)).best_alternative_move as
                   *mut i16,
               &mut (*g_book.node.offset(i as isize)).alternative_score as
                   *mut i16,
               &mut (*g_book.node.offset(i as isize)).flags as *mut u16);
        i += 1
    }
    g_book.book_node_count = new_book_node_count;
    create_hash_reference(&mut g_book);
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

pub unsafe fn read_binary_database(file_name: *const i8) {
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
    set_allocation(new_book_node_count + 1000 as i32, &mut g_book);
    i = 0;
    while i < new_book_node_count {
        fread(&mut (*g_book.node.offset(i as isize)).hash_val1 as *mut i32 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i32>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*g_book.node.offset(i as isize)).hash_val2 as *mut i32 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i32>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*g_book.node.offset(i as isize)).black_minimax_score as
                  *mut i16 as *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*g_book.node.offset(i as isize)).white_minimax_score as
                  *mut i16 as *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*g_book.node.offset(i as isize)).best_alternative_move as
                  *mut i16 as *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*g_book.node.offset(i as isize)).alternative_score as
                  *mut i16 as *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*g_book.node.offset(i as isize)).flags as *mut u16 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<u16>() as u64,
              1 as i32 as size_t, stream);
        i += 1
    }
    fclose(stream);
    g_book.book_node_count = new_book_node_count;
    create_hash_reference(&mut g_book);
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
    let mut merge_use_count = 0;
    let mut i: i32 = 0;
    i = 0;
    while i < merge_book_node_count {
        let mut merge_node =
            BookNode{hash_val1: 0,
                     hash_val2: 0,
                     black_minimax_score: 0,
                     white_minimax_score: 0,
                     best_alternative_move: 0,
                     alternative_score: 0,
                     flags: 0,};
        /* Read g_book.node. */
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
        /* Look up g_book.node in existing database. */
        let slot =
            probe_hash_table(merge_node.hash_val1, merge_node.hash_val2, &mut g_book);
        if slot == -(1 as i32) ||
               *g_book.book_hash_table.offset(slot as isize) == -(1 as i32) {
            /* New position, add it without modifications. */
            let this_node =
                create_BookNode(merge_node.hash_val1, merge_node.hash_val2,
                                merge_node.flags, &mut g_book);
            *g_book.node.offset(this_node as isize) = merge_node;
            merge_use_count += 1
        } else {
            /* Existing position, use the book from the merge file if it contains
            better endgame information. */
            let index = *g_book.book_hash_table.offset(slot as isize);
            if merge_node.flags as i32 & 16 as i32 != 0 &&
                   (*g_book.node.offset(index as isize)).flags as i32 &
                       16 as i32 == 0 ||
                   merge_node.flags as i32 & 4 as i32 != 0 &&
                       (*g_book.node.offset(index as isize)).flags as i32 &
                           4 as i32 == 0 {
                *g_book.node.offset(index as isize) = merge_node;
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
            g_book.book_node_count);
    let mut i = 0;
    while i < g_book.book_node_count {
        fprintf(stream,
                b"%d %d %d %d %d %d %d\n\x00" as *const u8 as
                    *const i8, (*g_book.node.offset(i as isize)).hash_val1,
                (*g_book.node.offset(i as isize)).hash_val2,
                (*g_book.node.offset(i as isize)).black_minimax_score as i32,
                (*g_book.node.offset(i as isize)).white_minimax_score as i32,
                (*g_book.node.offset(i as isize)).best_alternative_move as
                    i32,
                (*g_book.node.offset(i as isize)).alternative_score as i32,
                (*g_book.node.offset(i as isize)).flags as i32);
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
    fwrite(&mut g_book.book_node_count as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    let mut i = 0;
    while i < g_book.book_node_count {
        fwrite(&mut (*g_book.node.offset(i as isize)).hash_val1 as *mut i32
                   as *const std::ffi::c_void,
               ::std::mem::size_of::<i32>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_book.node.offset(i as isize)).hash_val2 as *mut i32
                   as *const std::ffi::c_void,
               ::std::mem::size_of::<i32>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_book.node.offset(i as isize)).black_minimax_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_book.node.offset(i as isize)).white_minimax_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_book.node.offset(i as isize)).best_alternative_move as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_book.node.offset(i as isize)).alternative_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_book.node.offset(i as isize)).flags as *mut u16 as
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
        safe_malloc((g_book.book_node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i32>()
                                                         as u64)) as
            *mut i32;
    let child_count =
        safe_malloc((g_book.book_node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                         as u64)) as
            *mut i16;
    let child =
        malloc((g_book.book_node_count as
                    u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                    as u64)) as
            *mut i16;
    let mut i = 0;
    while i < g_book.book_node_count {
        let ref mut fresh45 = (*g_book.node.offset(i as isize)).flags;
        *fresh45 =
            (*fresh45 as i32 | 8 as i32) as u16;
        i += 1
    }
    let mut node_index = 0;
    let mut child_index = 0;
    do_compress(0 as i32, node_order, child_count, &mut node_index,
                child, &mut child_index);
    fwrite(&mut g_book.book_node_count as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut child_index as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    fwrite(child_count as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           g_book.book_node_count as size_t, stream);
    fwrite(child as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           child_index as size_t, stream);
    i = 0;
    while i < g_book.book_node_count {
        fwrite(&mut (*g_book.node.offset(*node_order.offset(i as isize) as
                                      isize)).black_minimax_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_book.node.offset(*node_order.offset(i as isize) as
                                      isize)).white_minimax_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        i += 1
    }
    i = 0;
    while i < g_book.book_node_count {
        fwrite(&mut (*g_book.node.offset(*node_order.offset(i as isize) as
                                      isize)).best_alternative_move as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        i += 1
    }
    i = 0;
    while i < g_book.book_node_count {
        fwrite(&mut (*g_book.node.offset(*node_order.offset(i as isize) as
                                      isize)).alternative_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        i += 1
    }
    i = 0;
    while i < g_book.book_node_count {
        fwrite(&mut (*g_book.node.offset(*node_order.offset(i as isize) as
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
  Uncompress the subtree below the current g_book.node. This is done
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
    let val0___ = &mut val1;
    let val1___ = &mut val2;
    let orientation___ = &mut orientation;
    get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
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
    i = 0;
    while i < saved_child_count {
        let mut flipped: i32 = 0;
        this_move =
            *child.offset((saved_child_index + i) as isize) as i32;
        flipped = make_move_no_hash(side_to_move, this_move, &mut board_state, &mut moves_state, &mut flip_stack_ );
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
        let side_to_move___unmake_move_no_hash = side_to_move;
        let move_0___unmake_move_no_hash = this_move;
        {
            unmake_move_no_hash(side_to_move___unmake_move_no_hash, move_0___unmake_move_no_hash, &mut board_state.board, &mut moves_state, &mut flip_stack_);
        };
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
        safe_malloc((node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                         as u64)) as
            *mut i16;
    child =
        safe_malloc((child_list_size as
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
        safe_malloc((node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                         as u64)) as
            *mut i16;
    white_score =
        safe_malloc((node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                         as u64)) as
            *mut i16;
    alt_move =
        safe_malloc((node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                         as u64)) as
            *mut i16;
    alt_score =
        safe_malloc((node_count as
                         u64).wrapping_mul(::std::mem::size_of::<i16>()
                                                         as u64)) as
            *mut i16;
    flags =
        safe_malloc((node_count as
                         u64).wrapping_mul(::std::mem::size_of::<u16>()
                                                         as u64)) as
            *mut u16;
    i = 0;
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
    game_init(0 as *const i8, &mut dummy);
    midgame_state.toggle_midgame_hash_usage(1 as i32, 1 as i32);
    g_timer.toggle_abort_check(0 as i32);
    midgame_state.toggle_midgame_abort_check(0 as i32);
    magic = 2718;
    fwrite(&mut magic as *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    magic = 2818;
    fwrite(&mut magic as *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut node_count as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    node_index = 0;
    child_index = 0;
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
    line = 1;
    position_count = 0;
    already_wld_count = 0;
    already_exact_count = 0;
    new_nodes_created = 0;
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
            *ch = 0;
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
            *ch = 0;
            ch = ch.offset(-1)
        }
        if line % 4 as i32 == 3 as i32 {
            /* The position/result lines */
            position_count += 1;
            /* Parse the board */
            moves_state.disks_played = 0; /* The initial board contains 4 discs */
            col = 0;
            i = 1;
            while i <= 8 as i32 {
                j = 1;
                while j <= 8 as i32 {
                    pos = 10 as i32 * i + j;
                    match script_buffer[col as usize] as i32 {
                        42 | 88 | 120 => {
                            board_state.board[pos as usize] = 0;
                            moves_state.disks_played += 1
                        }
                        79 | 48 | 111 => {
                            board_state.board[pos as usize] = 2;
                            moves_state.disks_played += 1
                        }
                        45 | 46 => { board_state.board[pos as usize] = 1 as i32 }
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
            match script_buffer[65] as i32 {
                42 | 88 | 120 => { side_to_move = 0 as i32 }
                79 | 48 | 111 => { side_to_move = 2 as i32 }
                _ => {
                    fprintf(stderr,
                            b"\nBad side to move \'%c\' in board on line %d\n\n\x00"
                                as *const u8 as *const i8,
                            script_buffer[65] as
                                i32, line);
                    exit(1 as i32);
                }
            }
            moves_state.disks_played -= 4 as i32;
            /* Parse the result */
            wld_only = 1;
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
                score = 0;
                tokens_read =
                    sscanf(result_buffer.as_mut_ptr(),
                           b"%*s %s\x00" as *const u8 as *const i8,
                           move_buffer.as_mut_ptr());
                moves_read = tokens_read
            } else {
                /* Exact score */
                let mut black_discs: i32 = 0;
                let mut white_discs: i32 = 0;
                wld_only = 0;
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
            /* Set the score for the g_book.node corresponding to the position */
            let val0___ = &mut val1;
            let val1___ = &mut val2;
            let orientation___ = &mut orientation;
            get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
            slot = probe_hash_table(val1, val2, &mut g_book);
            index = *g_book.book_hash_table.offset(slot as isize);
            if index == -(1 as i32) {
                fprintf(stderr,
                        b"Position on line %d not found in book\n\x00" as
                            *const u8 as *const i8, line);
                exit(0 as i32);
            }
            probable_error = 0;
            if (*g_book.node.offset(index as isize)).flags as i32 &
                   4 as i32 != 0 {
                already_wld_count += 1;
                if score > 0 as i32 &&
                       (*g_book.node.offset(index as isize)).black_minimax_score as
                           i32 <= 0 as i32 ||
                       score == 0 as i32 &&
                           (*g_book.node.offset(index as isize)).black_minimax_score
                               as i32 != 0 as i32 ||
                       score < 0 as i32 &&
                           (*g_book.node.offset(index as isize)).black_minimax_score
                               as i32 > 0 as i32 {
                    probable_error = 1;
                    fprintf(stderr,
                            b"Line %d: New WLD score %d conflicts with old score %d\n\x00"
                                as *const u8 as *const i8, line,
                            score,
                            (*g_book.node.offset(index as isize)).black_minimax_score
                                as i32);
                }
            }
            if (*g_book.node.offset(index as isize)).flags as i32 &
                   16 as i32 != 0 {
                already_exact_count += 1;
                if wld_only == 0 &&
                       score !=
                           (*g_book.node.offset(index as isize)).black_minimax_score
                               as i32 {
                    probable_error = 1;
                    fprintf(stderr,
                            b"Line %d: New exact score %d conflicts with old score %d\n\x00"
                                as *const u8 as *const i8, line,
                            score,
                            (*g_book.node.offset(index as isize)).black_minimax_score
                                as i32);
                }
            }
            if probable_error != 0 || wld_only == 0 ||
                   (*g_book.node.offset(index as isize)).flags as i32 &
                       16 as i32 == 0 {
                let ref mut fresh46 =
                    (*g_book.node.offset(index as isize)).white_minimax_score;
                *fresh46 = score as i16;
                (*g_book.node.offset(index as isize)).black_minimax_score = *fresh46
            }
            if probable_error != 0 {
                /* Clear the old flags if score was wrong */
                let ref mut fresh47 = (*g_book.node.offset(index as isize)).flags;
                *fresh47 =
                    (*fresh47 as i32 &
                         !(4 as i32 | 16 as i32)) as
                        u16
            }
            if wld_only != 0 {
                let ref mut fresh48 = (*g_book.node.offset(index as isize)).flags;
                *fresh48 =
                    (*fresh48 as i32 | 4 as i32) as
                        u16
            } else {
                let ref mut fresh49 = (*g_book.node.offset(index as isize)).flags;
                *fresh49 =
                    (*fresh49 as i32 |
                         (4 as i32 | 16 as i32)) as
                        u16
            }
            /* Examine the position arising from the PV move; if it exists it
            need only be checked for sanity, otherwise a new g_book.node is
             created. */
            if moves_read > 0 as i32 {
                /* Make sure the optimal move leads to a position in the hash table */
                let mut row: i32 = 0;
                let mut col_0: i32 = 0;
                row =
                    move_buffer[1] as i32 -
                        '0' as i32;
                col_0 =
                   FE::tolower(move_buffer[0] as
                                i32) - 'a' as i32 + 1 as i32;
                move_0 = 10 as i32 * row + col_0;
                if row >= 1 as i32 && row <= 8 as i32 &&
                       col_0 >= 1 as i32 && col_0 <= 8 as i32
                       && make_move_no_hash(side_to_move, move_0, &mut board_state, &mut moves_state, &mut flip_stack_ ) != 0 {
                    let mut new_side_to_move =
                        0 as i32 + 2 as i32 - side_to_move;
                    let side_to_move = new_side_to_move;
                    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
                    if moves_state.move_count[moves_state.disks_played as usize] == 0 as i32 {
                        new_side_to_move = side_to_move
                    }
                    let val0___ = &mut val1;
                    let val1___ = &mut val2;
                    let orientation___ = &mut orientation;
                    get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
                    slot = probe_hash_table(val1, val2, &mut g_book);
                    index = *g_book.book_hash_table.offset(slot as isize);
                    if index == -(1 as i32) {
                        index =
                            create_BookNode(val1, val2,
                                            32 as i32 as
                                                u16, &mut g_book);
                        let ref mut fresh50 =
                            (*g_book.node.offset(index as
                                              isize)).white_minimax_score;
                        *fresh50 = score as i16;
                        (*g_book.node.offset(index as isize)).black_minimax_score =
                            *fresh50;
                        if new_side_to_move == 0 as i32 {
                            let ref mut fresh51 =
                                (*g_book.node.offset(index as isize)).flags;
                            *fresh51 =
                                (*fresh51 as i32 | 1 as i32)
                                    as u16
                        } else {
                            let ref mut fresh52 =
                                (*g_book.node.offset(index as isize)).flags;
                            *fresh52 =
                                (*fresh52 as i32 | 2 as i32)
                                    as u16
                        }
                        if wld_only != 0 {
                            let ref mut fresh53 =
                                (*g_book.node.offset(index as isize)).flags;
                            *fresh53 =
                                (*fresh53 as i32 | 4 as i32)
                                    as u16
                        } else {
                            let ref mut fresh54 =
                                (*g_book.node.offset(index as isize)).flags;
                            *fresh54 =
                                (*fresh54 as i32 |
                                     (4 as i32 | 16 as i32))
                                    as u16
                        }
                        new_nodes_created += 1
                    } else {
                        /* Position already exists, sanity-check it */
                        probable_error = 0;
                        if (*g_book.node.offset(index as isize)).flags as i32
                               & 4 as i32 != 0 {
                            if score > 0 as i32 &&
                                   (*g_book.node.offset(index as
                                                     isize)).black_minimax_score
                                       as i32 <= 0 as i32 ||
                                   score == 0 as i32 &&
                                       (*g_book.node.offset(index as
                                                         isize)).black_minimax_score
                                           as i32 != 0 as i32
                                   ||
                                   score < 0 as i32 &&
                                       (*g_book.node.offset(index as
                                                         isize)).black_minimax_score
                                           as i32 > 0 as i32 {
                                probable_error = 1;
                                fprintf(stderr,
                                        b"Line %d: New child WLD score %d conflicts with old score %d\n\x00"
                                            as *const u8 as
                                            *const i8, line, score,
                                        (*g_book.node.offset(index as
                                                          isize)).black_minimax_score
                                            as i32);
                            }
                        }
                        if (*g_book.node.offset(index as isize)).flags as i32
                               & 16 as i32 != 0 {
                            if wld_only == 0 &&
                                   score !=
                                       (*g_book.node.offset(index as
                                                         isize)).black_minimax_score
                                           as i32 {
                                probable_error = 1;
                                fprintf(stderr,
                                        b"Line %d: New child exact score %d conflicts with old score %d\n\x00"
                                            as *const u8 as
                                            *const i8, line, score,
                                        (*g_book.node.offset(index as
                                                          isize)).black_minimax_score
                                            as i32);
                            }
                        }
                        if probable_error != 0 {
                            /* Correct errors encountered */
                            let ref mut fresh55 =
                                (*g_book.node.offset(index as
                                                  isize)).white_minimax_score;
                            *fresh55 = score as i16;
                            (*g_book.node.offset(index as isize)).black_minimax_score
                                = *fresh55;
                            let ref mut fresh56 =
                                (*g_book.node.offset(index as isize)).flags;
                            *fresh56 =
                                (*fresh56 as i32 &
                                     !(4 as i32 | 16 as i32))
                                    as u16;
                            if wld_only != 0 {
                                let ref mut fresh57 =
                                    (*g_book.node.offset(index as isize)).flags;
                                *fresh57 =
                                    (*fresh57 as i32 |
                                         4 as i32) as u16
                            } else {
                                let ref mut fresh58 =
                                    (*g_book.node.offset(index as isize)).flags;
                                *fresh58 =
                                    (*fresh58 as i32 |
                                         (4 as i32 |
                                              16 as i32)) as
                                        u16
                            }
                        }
                    }
                    let side_to_move___unmake_move_no_hash = side_to_move;
                    let move_0___unmake_move_no_hash = move_0;
                    {
                        unmake_move_no_hash(side_to_move___unmake_move_no_hash, move_0___unmake_move_no_hash, &mut board_state.board, &mut moves_state, &mut flip_stack_);
                    };
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
    if g_book.candidate_count > 0 as i32 {
        if side_to_move == 0 as i32 {
            sign = 1 as i32
        } else { sign = -(1 as i32) }
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        /* Check that the position is in the opening book after all */
        if slot == -(1 as i32) ||
               *g_book.book_hash_table.offset(slot as isize) == -(1 as i32) {
            return
        }
        /* Pick the book score corresponding to the player to move and
           remove draw avoidance and the special scores for nodes WLD. */
        if side_to_move == 0 as i32 {
            score =
                (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as
                                  isize)).black_minimax_score as i32
        } else {
            score =
                (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as
                                  isize)).white_minimax_score as i32
        }
        if score == 30000 as i32 - 1 as i32 ||
               score == -(30000 as i32 - 1 as i32) {
            score = 0 as i32
        }
        if score > 30000 as i32 { score -= 30000 as i32 }
        if score < -(30000 as i32) { score += 30000 as i32 }
        printf(b"Book score is \x00" as *const u8 as *const i8);
        if (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as
                             isize)).flags as i32 & 16 as i32
               != 0 {
            printf(b"%+d (exact score).\x00" as *const u8 as
                       *const i8, sign * score);
        } else if (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as
                                    isize)).flags as i32 &
                      4 as i32 != 0 {
            printf(b"%+d (W/L/D solved).\x00" as *const u8 as
                       *const i8, sign * score);
        } else {
            printf(b"%+.2f.\x00" as *const u8 as *const i8,
                   (sign * score) as f64 / 128.0f64);
        }
        if (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as
                             isize)).flags as i32 & 32 as i32
               != 0 {
            printf(b" Private g_book.node.\x00" as *const u8 as *const i8);
        }
        puts(b"\x00" as *const u8 as *const i8);
        i = 0;
        while i < g_book.candidate_count {
            printf(b"   %c%c   \x00" as *const u8 as *const i8,
                   'a' as i32 +
                       g_book.candidate_list[i as usize].move_0 % 10 as i32 -
                       1 as i32,
                   '0' as i32 +
                       g_book.candidate_list[i as usize].move_0 / 10 as i32);
            output_score = g_book.candidate_list[i as usize].score;
            if output_score >= 30000 as i32 {
                output_score -= 30000 as i32
            } else if output_score <= -(30000 as i32) {
                output_score += 30000 as i32
            }
            if g_book.candidate_list[i as usize].flags & 16 as i32 != 0 {
                printf(b"%+-6d  (exact score)\x00" as *const u8 as
                           *const i8, output_score);
            } else if g_book.candidate_list[i as usize].flags & 4 as i32 != 0
             {
                printf(b"%+-6d  (W/L/D solved)\x00" as *const u8 as
                           *const i8, output_score);
            } else {
                printf(b"%+-6.2f\x00" as *const u8 as *const i8,
                       output_score as f64 / 128.0f64);
                if g_book.candidate_list[i as usize].flags & 64 as i32 != 0 {
                    printf(b"  (deviation)\x00" as *const u8 as
                               *const i8);
                }
            }
            puts(b"\x00" as *const u8 as *const i8);
            i += 1
        }
    };
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
    level = 0;
    i = 0;
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
        *scan_ptr = 0;
        op_move_count =
              FE::strlen(move_seq.as_mut_ptr()).wrapping_div(2 as i32 as
                                                           u64) as
                i32;
        j = 0;
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
        j = 0;
        while j < op_move_count {
            if generate_specific(op_move[j as usize],
                                 side_to_move[j as usize], &board_state.board) == 0 {
                printf(b"Move %d in opening #%d is illegal\n\x00" as *const u8
                           as *const i8, j + 1 as i32, i);
                exit(1 as i32);
            }
            make_move(side_to_move[j as usize], op_move[j as usize],
                      1 as i32, &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
            j += 1
        }
        /* Write the code fragment  */
        let val0___ = &mut hash_val1;
        let val1___ = &mut hash_val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
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
            let side_to_move_argument = side_to_move[j as usize];
            let move_0 = op_move[j as usize];
            {
                unmake_move(side_to_move_argument, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
            };
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

/*
  EXPORT_POSITION
  Output the position and its value according to the database
  to file.
*/
unsafe fn export_position(side_to_move: i32,
                                     score: i32,
                                     target_file: *mut FILE) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut black_mask: i32 = 0;
    let mut white_mask: i32 = 0;
    let mut hi_mask: i32 = 0;
    let mut lo_mask: i32 = 0;
    i = 1;
    while i <= 8 as i32 {
        black_mask = 0;
        white_mask = 0;
        j = 0;
        pos = 10 as i32 * i + 1 as i32;
        while j < 8 as i32 {
            if board_state.board[pos as usize] == 0 as i32 {
                black_mask |= (1 as i32) << j
            } else if board_state.board[pos as usize] == 2 as i32 {
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
            moves_state.disks_played, score);
}
/*
   DO_RESTRICTED_MINIMAX
   Calculates the book-only minimax value of g_book.node INDEX,
   not caring about deviations from the database.
*/
unsafe fn do_restricted_minimax(index: i32,
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
    if (*g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    /* Recursively minimax all children of the g_book.node */
    if (*g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    if side_to_move == 0 as i32 {
        best_score = -(32000 as i32) as i16
    } else { best_score = 32000 as i32 as i16 }
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
    child_count = 0;
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        board_state.piece_count[0][moves_state.disks_played as usize] =
            disc_count(0 as i32, &board_state.board);
        board_state.piece_count[2][moves_state.disks_played as usize] =
            disc_count(2 as i32, &board_state.board);
        this_move = moves_state.move_list[moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        child = *g_book.book_hash_table.offset(slot as isize);
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
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i += 1
    }
    if (*g_book.node.offset(index as isize)).flags as i32 & 16 as i32
        != 0 ||
        (*g_book.node.offset(index as isize)).flags as i32 &
            4 as i32 != 0 && child_count == 0 as i32 {
        best_score = (*g_book.node.offset(index as isize)).black_minimax_score
    } else if child_count == 0 as i32 {
        printf(b"%d disks played\n\x00" as *const u8 as *const i8,
               moves_state.disks_played);
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
    let ref mut fresh16 = (*g_book.node.offset(index as isize)).flags;
    *fresh16 = (*fresh16 as i32 ^ 8 as i32) as u16;
    if moves_state.disks_played >= low && moves_state.disks_played <= high {
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
    i = 0;
    while i < g_book.book_node_count {
        let ref mut fresh17 = (*g_book.node.offset(i as isize)).flags;
        *fresh17 =
            (*fresh17 as i32 | 8 as i32) as u16;
        i += 1
    }
    minimax_values =
        safe_malloc((g_book.book_node_count as
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
unsafe fn do_midgame_statistics(index: i32,
                                           spec: StatisticsSpec, echo:i32) {
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
    if (*g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
    /* With a certain probability, search the position to a variety
     of different depths in order to determine correlations. */
    if ((engine::src::myrandom::random_instance.my_random() % 1000 as i32 as i64) as f64)
        < 1000.0f64 * spec.prob &&
        abs((*g_book.node.offset(index as isize)).black_minimax_score as
            i32) < spec.max_diff {
        display_board(stdout, &board_state.board, 0 as i32,
                      0 as i32, 0 as i32, 0 as i32,
                      current_row,
                      black_player, black_time, black_eval,
                      white_player, white_time, white_eval,
                      &board_state.black_moves, &board_state.white_moves
        );
        setup_hash(0 as i32, &mut hash_state, &mut  random_instance);
        determine_hash_values(side_to_move, &board_state.board, &mut hash_state);
        depth = 1;
        while depth <= spec.max_depth {
            middle_game::<FE>(side_to_move, depth, 0 as i32,
                        &mut dummy_info, echo);
            eval_list[depth as usize] = search_state.root_eval;
            printf(b"%2d: %-5d \x00" as *const u8 as *const i8,
                   depth, eval_list[depth as usize]);
            depth += 2 as i32
        }
        puts(b"\x00" as *const u8 as *const i8);
        setup_hash(0 as i32, &mut hash_state, &mut  random_instance);
        determine_hash_values(side_to_move, &board_state.board, &mut hash_state);
        depth = 2;
        while depth <= spec.max_depth {
            middle_game::<FE>(side_to_move, depth, 0 as i32,
                        &mut dummy_info, echo);
            eval_list[depth as usize] = search_state.root_eval;
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
            let val0___ = &mut val1;
            let val1___ = &mut val2;
            let orientation___ = &mut orientation;
            get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
            fprintf(out_file,
                    b"%08x%08x %2d \x00" as *const u8 as *const i8,
                    val1, val2, moves_state.disks_played);
            fprintf(out_file,
                    b"%2d %2d \x00" as *const u8 as *const i8,
                    1 as i32, spec.max_depth);
            i = 1;
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
    /* Recursively search the children of the g_book.node */
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        this_move = moves_state.move_list[moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        child = *g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_midgame_statistics(child, spec, echo);
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i += 1
    }
    let ref mut fresh18 = (*g_book.node.offset(index as isize)).flags;
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
    g_timer.toggle_abort_check(0 as i32);
    time(&mut start_time);
    i = 0;
    while i < g_book.book_node_count {
        let ref mut fresh19 = (*g_book.node.offset(i as isize)).flags;
        *fresh19 =
            (*fresh19 as i32 | 8 as i32) as u16;
        i += 1
    }
    spec.prob = probability;
    spec.max_diff = max_diff;
    spec.max_depth = max_depth;
    spec.out_file_name = statistics_file_name;
    let x = start_time as i32;
    engine::src::myrandom::random_instance.my_srandom(x);
    do_midgame_statistics(0 as i32, spec, g_config.echo);
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
unsafe fn endgame_correlation(mut side_to_move: i32,
                                         best_score: i32,
                                         best_move: i32,
                                         min_disks: i32,
                                         max_disks: i32,
                                         spec: StatisticsSpec, echo:i32) {
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
    display_board(stdout, &board_state.board, 0 as i32,
                  0 as i32, 0 as i32, 0 as i32,
                  current_row,
                  black_player, black_time, black_eval,
                  white_player, white_time, white_eval,
                  &board_state.black_moves, &board_state.white_moves
    );
    hash_state.set_hash_transformation(abs(engine::src::myrandom::random_instance.my_random() as i32) as u32,
                                       abs(engine::src::myrandom::random_instance.my_random() as i32) as u32);
    determine_hash_values(side_to_move, &board_state.board, &mut hash_state);
    depth = 1;
    while depth <= spec.max_depth {
        middle_game::<FE>(side_to_move, depth, 0 as i32, &mut dummy_info, echo);
        eval_list[depth as usize] = search_state.root_eval;
        printf(b"%2d: %-6.2f \x00" as *const u8 as *const i8, depth,
               eval_list[depth as usize] as f64 / 128.0f64);
        depth += 1
    }
    out_file =
        fopen(spec.out_file_name,
              b"a\x00" as *const u8 as *const i8);
    if !out_file.is_null() {
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        fprintf(out_file,
                b"%08x%08x %2d \x00" as *const u8 as *const i8,
                val1, val2, moves_state.disks_played);
        fprintf(out_file, b"%+3d \x00" as *const u8 as *const i8,
                best_score);
        fprintf(out_file, b"%2d %2d \x00" as *const u8 as *const i8,
                1 as i32, spec.max_depth);
        i = 1;
        while i <= spec.max_depth {
            fprintf(out_file, b"%5d \x00" as *const u8 as *const i8,
                    eval_list[i as usize]);
            i += 1
        }
        fprintf(out_file, b"\n\x00" as *const u8 as *const i8);
        fclose(out_file);
    }
    if moves_state.disks_played < max_disks {
        make_move(side_to_move, best_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        stored_side_to_move = side_to_move;
        side_to_move = 0 as i32 + 2 as i32 - side_to_move;
        generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
        if moves_state.move_count[moves_state.disks_played as usize] > 0 as i32 {
            printf(b"\nSolving with %d empty...\n\n\x00" as *const u8 as
                       *const i8, 60 as i32 - moves_state.disks_played);
            fill_move_alternatives::<FE>(side_to_move, 16 as i32);
            if g_book.get_candidate_count() > 0 as i32 ||
                moves_state.disks_played >= 40 as i32 {
                print_move_alternatives(side_to_move);
                hash_state.set_hash_transformation(0 as i32 as u32,
                                        0 as i32 as u32);
               end_game::<FE>(side_to_move, 0 as i32, 1 as i32,
                         1 as i32, 0 as i32, &mut dummy_info, echo);
                endgame_correlation(side_to_move, search_state.root_eval,
                                    board_state.pv[0][0],
                                    min_disks, max_disks, spec, echo);
            }
        }
        let side_to_move = stored_side_to_move;
        let move_0 = best_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
    };
}
/*
   DO_ENDGAME_STATISTICS
   Recursively makes sure a subtree is evaluated to
   the specified depth.
*/
unsafe fn do_endgame_statistics(index: i32,
                                           spec: StatisticsSpec, echo:i32 ) {
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
    if (*g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
    /* With a certain probability, search the position to a variety
     of different depths in order to determine correlations. */
    if moves_state.disks_played == 33 as i32 &&
        ((engine::src::myrandom::random_instance.my_random() % 1000 as i32 as i64) as
            f64) < 1000.0f64 * spec.prob {
        setup_hash(0 as i32, &mut hash_state, &mut  random_instance);
        determine_hash_values(side_to_move, &board_state.board, &mut hash_state);
        printf(b"\nSolving with %d empty...\n\n\x00" as *const u8 as
                   *const i8, 60 as i32 - moves_state.disks_played);
        fill_move_alternatives::<FE>(side_to_move, 16 as i32);
        if g_book.get_candidate_count() > 0 as i32 ||
            moves_state.disks_played >= 40 as i32 {
            print_move_alternatives(side_to_move);
            hash_state.set_hash_transformation(0 as i32 as u32,
                                    0 as i32 as u32);
           end_game::<FE>(side_to_move, 0 as i32, 1 as i32,
                     1 as i32, 0 as i32, &mut dummy_info, echo);
            if abs(search_state.root_eval) <= spec.max_diff {
                endgame_correlation(side_to_move, search_state.root_eval,
                                    board_state.pv[0][0],
                                    moves_state.disks_played, 48 as i32, spec, echo);
            }
        }
    }
    /* Recursively search the children of the g_book.node */
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        this_move = moves_state.move_list[moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        child = *g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_endgame_statistics(child, spec, echo);
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i += 1
    }
    let ref mut fresh20 = (*g_book.node.offset(index as isize)).flags;
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
    g_timer.toggle_abort_check(0 as i32);
    time(&mut start_time);
    i = 0;
    while i < g_book.book_node_count {
        let ref mut fresh21 = (*g_book.node.offset(i as isize)).flags;
        *fresh21 =
            (*fresh21 as i32 | 8 as i32) as u16;
        i += 1
    }
    spec.prob = probability;
    spec.max_diff = max_diff;
    spec.max_depth = max_depth;
    spec.out_file_name = statistics_file_name;
    let x = start_time as i32;
    engine::src::myrandom::random_instance.my_srandom(x);
    do_endgame_statistics(0 as i32, spec, g_config.echo);
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
unsafe fn do_clear(index: i32, low: i32,
                              high: i32, flags: i32) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    if (*g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if moves_state.disks_played >= low && moves_state.disks_played <= high {
        if flags & 1 as i32 != 0 { clear_node_depth(index, &mut g_book); }
        if (*g_book.node.offset(index as isize)).flags as i32 &
            4 as i32 != 0 && flags & 2 as i32 != 0 {
            let ref mut fresh27 = (*g_book.node.offset(index as isize)).flags;
            *fresh27 =
                (*fresh27 as i32 ^ 4 as i32) as u16
        }
        if (*g_book.node.offset(index as isize)).flags as i32 &
            16 as i32 != 0 && flags & 4 as i32 != 0 {
            let ref mut fresh28 = (*g_book.node.offset(index as isize)).flags;
            *fresh28 =
                (*fresh28 as i32 ^ 16 as i32) as
                    u16
        }
    }
    if moves_state.disks_played <= high {
        if (*g_book.node.offset(index as isize)).flags as i32 &
            1 as i32 != 0 {
            side_to_move = 0 as i32
        } else { side_to_move = 2 as i32 }
        generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
        i = 0;
        while i < moves_state.move_count[moves_state.disks_played as usize] {
            this_move = moves_state.move_list[moves_state.disks_played as usize][i as usize];
            make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
            let val0___ = &mut val1;
            let val1___ = &mut val2;
            let orientation___ = &mut orientation;
            get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
            slot = probe_hash_table(val1, val2, &mut g_book);
            child = *g_book.book_hash_table.offset(slot as isize);
            if child != -(1 as i32) {
                do_clear(child, low, high, flags);
            }
            let move_0 = this_move;
            {
                unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
            };
            i += 1
        }
    }
    let ref mut fresh29 = (*g_book.node.offset(index as isize)).flags;
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
    i = 0;
    while i < g_book.book_node_count {
        let ref mut fresh30 = (*g_book.node.offset(i as isize)).flags;
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
   Performs endgame correction (WLD or full solve) of a g_book.node
   and (recursively) the subtree below it.
*/
unsafe fn do_correct(index: i32,
                                max_empty: i32,
                                full_solve: i32,
                                target_name: *const i8,
                                move_hist: *mut i8, echo:i32) {
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
    if g_book.evaluated_count >= g_book.max_eval_count { return }
    if (*g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    /* First correct the children */
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
    child_count = 0;
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        this_move = moves_state.move_list[moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        child = *g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            child_move[child_count as usize] = this_move;
            child_node[child_count as usize] = child;
            child_count += 1
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i += 1
    }
    let mut current_block_29: u64;
    i = 0;
    while i < child_count {
        if side_to_move == 0 as i32 {
            if g_book.force_black != 0 &&
                (*g_book.node.offset(child_node[i as usize] as
                    isize)).black_minimax_score as
                    i32 !=
                    (*g_book.node.offset(index as isize)).black_minimax_score as
                        i32 {
                current_block_29 = 14818589718467733107;
            } else { current_block_29 = 11913429853522160501; }
        } else if g_book.force_white != 0 &&
            (*g_book.node.offset(child_node[i as usize] as
                isize)).white_minimax_score as
                i32 !=
                (*g_book.node.offset(index as isize)).white_minimax_score
                    as i32 {
            current_block_29 = 14818589718467733107;
        } else { current_block_29 = 11913429853522160501; }
        match current_block_29 {
            11913429853522160501 => {
                this_move = child_move[i as usize];
                sprintf(move_hist.offset((2 as i32 * moves_state.disks_played) as
                    isize),
                        b"%c%c\x00" as *const u8 as *const i8,
                        'a' as i32 + this_move % 10 as i32 -
                            1 as i32,
                        '0' as i32 + this_move / 10 as i32);
                make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
                do_correct(child_node[i as usize], max_empty, full_solve,
                           target_name, move_hist, echo);
                let move_0 = this_move;
                {
                    unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
                };
                *move_hist.offset((2 as i32 * moves_state.disks_played) as isize)
                    = '\u{0}' as i32 as i8
            }
            _ => { }
        }
        i += 1
    }
    /* Then correct the g_book.node itself (hopefully exploiting lots
     of useful information in the hash table) */
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
    determine_hash_values(side_to_move, &board_state.board, &mut hash_state);
    if moves_state.disks_played >= 60 as i32 - max_empty {
        really_evaluate =
            (full_solve != 0 &&
                (*g_book.node.offset(index as isize)).flags as i32 &
                    16 as i32 == 0 ||
                full_solve == 0 &&
                    (*g_book.node.offset(index as isize)).flags as i32 &
                        (4 as i32 | 16 as i32) == 0) as
                i32;
        if abs((*g_book.node.offset(index as isize)).alternative_score as
            i32) < g_book.min_eval_span ||
            abs((*g_book.node.offset(index as isize)).alternative_score as
                i32) > g_book.max_eval_span {
            really_evaluate = 0 as i32
        }
        if abs((*g_book.node.offset(index as isize)).black_minimax_score as
            i32) < g_book.min_negamax_span ||
            abs((*g_book.node.offset(index as isize)).black_minimax_score as
                i32) > g_book.max_negamax_span {
            really_evaluate = 0 as i32
        }
        if really_evaluate != 0 {
            if target_name.is_null() {
                /* Solve now */
                reset_counter(&mut search_state.nodes);
               end_game::<FE>(side_to_move, (full_solve == 0) as i32,
                         0 as i32, 1 as i32, 0 as i32,
                         &mut dummy_info, echo);
                if side_to_move == 0 as i32 {
                    outcome = search_state.root_eval
                } else { outcome = -search_state.root_eval }
                let ref mut fresh31 =
                    (*g_book.node.offset(index as isize)).white_minimax_score;
                *fresh31 = outcome as i16;
                (*g_book.node.offset(index as isize)).black_minimax_score = *fresh31;
                if outcome > 0 as i32 {
                    let ref mut fresh32 =
                        (*g_book.node.offset(index as isize)).black_minimax_score;
                    *fresh32 =
                        (*fresh32 as i32 + 30000 as i32) as
                            i16;
                    let ref mut fresh33 =
                        (*g_book.node.offset(index as isize)).white_minimax_score;
                    *fresh33 =
                        (*fresh33 as i32 + 30000 as i32) as
                            i16
                }
                if outcome < 0 as i32 {
                    let ref mut fresh34 =
                        (*g_book.node.offset(index as isize)).black_minimax_score;
                    *fresh34 =
                        (*fresh34 as i32 - 30000 as i32) as
                            i16;
                    let ref mut fresh35 =
                        (*g_book.node.offset(index as isize)).white_minimax_score;
                    *fresh35 =
                        (*fresh35 as i32 - 30000 as i32) as
                            i16
                }
                if full_solve != 0 {
                    let ref mut fresh36 =
                        (*g_book.node.offset(index as isize)).flags;
                    *fresh36 =
                        (*fresh36 as i32 | 16 as i32) as
                            u16
                } else {
                    let ref mut fresh37 =
                        (*g_book.node.offset(index as isize)).flags;
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
                    let val0___ = &mut val1;
                    let val1___ = &mut val2;
                    let orientation___ = &mut orientation;
                    get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
                    fprintf(target_file,
                            b"%% %d %d\n\x00" as *const u8 as
                                *const i8, val1, val2);
                    i = 1;
                    while i <= 8 as i32 {
                        j = 1;
                        while j <= 8 as i32 {
                            pos = 10 as i32 * i + j;
                            if board_state.board[pos as usize] == 0 as i32 {
                                putc('X' as i32, target_file);
                            } else if board_state.board[pos as usize] == 2 as i32
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
            g_book.evaluated_count += 1
        }
    }
    if g_book.evaluated_count >=
        (g_book.evaluation_stage + 1 as i32) * g_book.max_eval_count /
            25 as i32 {
        g_book.evaluation_stage += 1;
        putc('|' as i32, stdout);
        if g_book.evaluation_stage % 5 as i32 == 0 as i32 {
            printf(b" %d%% \x00" as *const u8 as *const i8,
                   4 as i32 * g_book.evaluation_stage);
        }
        fflush(stdout);
    }
    let ref mut fresh38 = (*g_book.node.offset(index as isize)).flags;
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
    g_book.exhausted_node_count = 0;
    g_book.evaluated_count = 0;
    g_book.evaluation_stage = 0;
    time(&mut start_time);
    i = 0;
    while i < g_book.book_node_count {
        let ref mut fresh39 = (*g_book.node.offset(i as isize)).flags;
        *fresh39 =
            (*fresh39 as i32 | 8 as i32) as u16;
        i += 1
    }
    feasible_count = 0;
    i = 0;
    while i < g_book.book_node_count {
        let ref mut fresh40 = (*g_book.node.offset(i as isize)).flags;
        *fresh40 =
            (*fresh40 as i32 | 8 as i32) as u16;
        if get_node_depth(i, &mut g_book) < max_empty &&
            abs((*g_book.node.offset(i as isize)).alternative_score as
                i32) >= g_book.min_eval_span &&
            abs((*g_book.node.offset(i as isize)).alternative_score as
                i32) <= g_book.max_eval_span &&
            abs((*g_book.node.offset(i as isize)).black_minimax_score as
                i32) >= g_book.min_negamax_span &&
            abs((*g_book.node.offset(i as isize)).black_minimax_score as
                i32) <= g_book.max_negamax_span {
            feasible_count += 1
        }
        i += 1
    }
    g_book.max_eval_count =
        if feasible_count < g_book.max_batch_size {
            feasible_count
        } else { g_book.max_batch_size };
    printf(b"Correcting <= %d empty \x00" as *const u8 as *const i8,
           max_empty);
    if full_solve != 0 {
        printf(b"(full solve). \x00" as *const u8 as *const i8);
    } else {
        printf(b"(WLD solve). \x00" as *const u8 as *const i8);
    }
    if g_book.min_eval_span > 0 as i32 ||
        g_book.max_eval_span < 1000 as i32 * 128 as i32 {
        printf(b"Eval interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const i8,
               g_book.min_eval_span as f64 / 128.0f64,
               g_book.max_eval_span as f64 / 128.0f64);
    }
    if g_book.min_negamax_span > 0 as i32 ||
        g_book.max_negamax_span < 1000 as i32 * 128 as i32 {
        printf(b"Negamax interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const i8,
               g_book.min_negamax_span as f64 / 128.0f64,
               g_book.max_negamax_span as f64 / 128.0f64);
    }
    if g_book.max_eval_count == feasible_count {
        printf(b"\n%d relevant nodes.\x00" as *const u8 as
                   *const i8, feasible_count);
    } else {
        printf(b"\nMax batch size is %d.\x00" as *const u8 as
                   *const i8, g_book.max_batch_size);
    }
    puts(b"\x00" as *const u8 as *const i8);
    printf(b"Progress: \x00" as *const u8 as *const i8);
    fflush(stdout);
    move_buffer[0] = '\u{0}' as i32 as i8;
    do_correct(0 as i32, max_empty, full_solve,
               correction_script_name, move_buffer.as_mut_ptr(), g_config.echo);
    time(&mut stop_time);
    printf(b"(took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    if correction_script_name.is_null() {
        /* Positions solved */
        printf(b"%d nodes solved\n\x00" as *const u8 as *const i8,
               g_book.evaluated_count);
    } else {
        printf(b"%d nodes exported to %s\n\x00" as *const u8 as
                   *const i8, g_book.evaluated_count,
               correction_script_name);
    }
    puts(b"\x00" as *const u8 as *const i8);
}
/*
   DO_EXPORT
   Recursively exports all variations rooted at book position # INDEX.
*/
unsafe fn do_export(index: i32, stream: *mut FILE,
                               move_vec: &mut [i32; 60]) {
    let mut i: i32 = 0;
    let mut child_count: i32 = 0;
    let mut allow_branch: i32 = 0;
    let mut side_to_move: i32 = 0;
    allow_branch =
        (*g_book.node.offset(index as isize)).flags as i32 &
            8 as i32;
    if (*g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
    child_count = 0;
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        let mut child: i32 = 0;
        let mut slot: i32 = 0;
        let mut val1: i32 = 0;
        let mut val2: i32 = 0;
        let mut orientation: i32 = 0;
        let this_move: i32 =
            moves_state.move_list[moves_state.disks_played as usize][i as usize];
        *(move_vec.as_mut_ptr()).offset(moves_state.disks_played as isize) = this_move;
        make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        child = *g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_export(child, stream, move_vec);
            child_count += 1
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        if child_count == 1 as i32 && allow_branch == 0 { break ; }
        i += 1
    }
    if child_count == 0 as i32 {
        /* We've reached a leaf in the opening tree. */
        i = 0;
        while i < moves_state.disks_played {
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
    let ref mut fresh41 = (*g_book.node.offset(index as isize)).flags;
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
    let mut i = 0;
    while i < g_book.book_node_count {
        let ref mut fresh42 = (*g_book.node.offset(i as isize)).flags;
        *fresh42 =
            (*fresh42 as i32 | 8 as i32) as u16;
        i += 1
    }
    do_export(0 as i32, stream, &mut move_vec);
    fclose(stream);
}

// ============================
// Following functions were moved back here from the engine,
// even though they don't depend on libc (and I moved them to engine manually before)
// These was done to ease engine cleanup, but these function could
// be moved to libc-independent crate when time comes.
// ============================
/*
  VALIDATE_TREE
  Makes sure all nodes are either exhausted, solved or have a deviation.
  The number of positions evaluated is returned.
*/

pub unsafe fn validate_tree<FE: FrontEnd>(echo: i32) -> i32 {
    prepare_tree_traversal();
    validate_prepared_tree::<FE>(echo)
}

// extracted from validate_tree
pub unsafe fn validate_prepared_tree<FE: FrontEnd>(echo: i32) -> i32 {
    g_book.exhausted_node_count = 0;
    g_book.evaluated_count = 0;
    g_book.evaluation_stage = 0;
    let mut feasible_count = 0;
    let mut i = 0;
    while i < g_book.book_node_count {
        if (*g_book.node.offset(i as isize)).flags as i32 &
            (4 as i32 | 16 as i32) == 0 &&
            (*g_book.node.offset(i as isize)).alternative_score as i32 ==
                9999 as i32 &&
            (*g_book.node.offset(i as isize)).best_alternative_move as i32
                != -(2 as i32) {
            feasible_count += 1
        }
        i += 1
    }
    g_book.max_eval_count =
        if feasible_count < g_book.max_batch_size {
            feasible_count
        } else { g_book.max_batch_size };
    if feasible_count > 0 as i32 {
        i = 0;
        while i < g_book.book_node_count {
            let ref mut fresh20 = (*g_book.node.offset(i as isize)).flags;
            *fresh20 =
                (*fresh20 as i32 | 8 as i32) as
                    u16;
            i += 1
        }
        do_validate::<FE>(0 as i32, echo);
    }
    return g_book.evaluated_count;
}

/*
   DO_VALIDATE
   Recursively makes sure a subtree doesn't contain any midgame
   g_book.node without a deviation move.
*/
pub unsafe fn do_validate<FE: FrontEnd>(index: i32, echo:i32) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    if g_book.evaluated_count >= g_book.max_eval_count { return }
    if (*g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
    if (*g_book.node.offset(index as isize)).flags as i32 &
        (16 as i32 | 4 as i32) == 0 &&
        (*g_book.node.offset(index as isize)).alternative_score as i32 ==
            9999 as i32 &&
        (*g_book.node.offset(index as isize)).best_alternative_move as i32
            != -(2 as i32) {
        evaluate_node::<FE>(index, echo);
    }
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        this_move = moves_state.move_list[moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        child = *g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) { do_validate::<FE>(child, echo); }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i += 1
    }
    let ref mut fresh19 = (*g_book.node.offset(index as isize)).flags;
    *fresh19 = (*fresh19 as i32 ^ 8 as i32) as u16;
}


/*
   DO_EVALUATE
   Recursively makes sure a subtree is evaluated to
   the specified depth.
*/
pub unsafe fn do_evaluate<FE: FrontEnd>(index: i32, echo:i32) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    if g_book.evaluated_count >= g_book.max_eval_count { return }
    if (*g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
    if (*g_book.node.offset(index as isize)).flags as i32 &
        (16 as i32 | 4 as i32) == 0 {
        evaluate_node::<FE>(index, echo);
    }
    if g_book.evaluated_count >=
        (g_book.evaluation_stage + 1 as i32) * g_book.max_eval_count /
            25 as i32 {
        g_book.evaluation_stage += 1;
        FE::report_do_evaluate(g_book.evaluation_stage);
    }
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        this_move = moves_state.move_list[moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        child = *g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) { do_evaluate::<FE>(child, echo); }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i += 1
    }
    let ref mut fresh17 = (*g_book.node.offset(index as isize)).flags;
    *fresh17 = (*fresh17 as i32 ^ 8 as i32) as u16;
}


pub unsafe fn compute_feasible_count() -> i32 {
    let mut feasible_count = 0;
    let mut i = 0;
    while i < g_book.book_node_count {
        let ref mut fresh18 = (*g_book.node.offset(i as isize)).flags;
        *fresh18 =
            (*fresh18 as i32 | 8 as i32) as u16;
        if ((*g_book.node.offset(i as isize)).alternative_score as i32 ==
            9999 as i32 ||
            get_node_depth(i, &mut g_book) < g_book.search_depth &&
                abs((*g_book.node.offset(i as isize)).alternative_score as
                    i32) >= g_book.min_eval_span &&
                abs((*g_book.node.offset(i as isize)).alternative_score as
                    i32) <= g_book.max_eval_span &&
                abs((*g_book.node.offset(i as isize)).black_minimax_score as
                    i32) >= g_book.min_negamax_span &&
                abs((*g_book.node.offset(i as isize)).black_minimax_score as
                    i32) <= g_book.max_negamax_span) &&
            (*g_book.node.offset(i as isize)).flags as i32 &
                (4 as i32 | 16 as i32) == 0 {
            feasible_count += 1
        }
        i += 1
    }
    feasible_count
}


pub unsafe fn engine_minimax_tree() {
    /* Mark all nodes as not traversed */
    let mut i = 0;
    while i < g_book.book_node_count {
        let ref mut fresh15 = (*g_book.node.offset(i as isize)).flags;
        *fresh15 =
            (*fresh15 as i32 | 8 as i32) as u16;
        i += 1
    }
    let mut dummy_black_score: i32 = 0;
    let mut dummy_white_score: i32 = 0;
    do_minimax(0 as i32, &mut dummy_black_score, &mut dummy_white_score);
}

pub unsafe fn engine_examine_tree() {
    let mut i = 0;
    while i <= 60 as i32 {
        g_book.exact_count[i as usize] = 0;
        g_book.wld_count[i as usize] = 0;
        g_book.exhausted_count[i as usize] = 0;
        g_book.common_count[i as usize] = 0;
        i += 1
    }
    g_book.unreachable_count = 0;
    g_book.leaf_count = 0;
    g_book.bad_leaf_count = 0;
    /* Mark all nodes as not traversed and examine the tree */
    i = 0;
    while i < g_book.book_node_count {
        let ref mut fresh22 = (*g_book.node.offset(i as isize)).flags;
        *fresh22 =
            (*fresh22 as i32 | 8 as i32) as u16;
        i += 1
    }
    do_examine(0 as i32);
    /* Any nodes not reached by the walkthrough? */
    i = 0;
    while i < g_book.book_node_count {
        if (*g_book.node.offset(i as isize)).flags as i32 & 8 as i32
            != 0 {
            g_book.unreachable_count += 1;
            let ref mut fresh23 = (*g_book.node.offset(i as isize)).flags;
            *fresh23 =
                (*fresh23 as i32 ^ 8 as i32) as u16
        }
        i += 1
    }
}

/*
   DO_EXAMINE
   Add the properties of node INDEX to the statistics being gathered
   and recursively traverse the subtree of the node, doing the same
   thing in all nodes.
*/
pub unsafe fn do_examine(index: i32) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut child_count: i32 = 0;
    let mut child_move: [i32; 64] = [0; 64];
    let mut child_node: [i32; 64] = [0; 64];
    if (*g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*g_book.node.offset(index as isize)).flags as i32 & 16 as i32
        != 0 {
        g_book.exact_count[moves_state.disks_played as usize] += 1
    } else if (*g_book.node.offset(index as isize)).flags as i32 &
        4 as i32 != 0 {
        g_book.wld_count[moves_state.disks_played as usize] += 1
    } else if (*g_book.node.offset(index as isize)).best_alternative_move as
        i32 == -(2 as i32) {
        g_book.exhausted_count[moves_state.disks_played as usize] += 1
    } else { g_book.common_count[moves_state.disks_played as usize] += 1 }
    /* Examine all the children of the g_book.node */
    if (*g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
    child_count = 0;
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        this_move = moves_state.move_list[moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        child = *g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            child_move[child_count as usize] = this_move;
            child_node[child_count as usize] = child;
            child_count += 1
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i += 1
    }
    if child_count == 0 as i32 {
        g_book.leaf_count += 1;
        if (*g_book.node.offset(index as isize)).flags as i32 &
            16 as i32 == 0 {
            g_book.bad_leaf_count += 1
        }
        if (*g_book.node.offset(index as isize)).flags as i32 &
            4 as i32 == 0 {
            g_book.really_bad_leaf_count += 1
        }
    } else {
        let mut current_block_38: u64;
        i = 0;
        while i < child_count {
            if side_to_move == 0 as i32 {
                if g_book.force_black != 0 &&
                    (*g_book.node.offset(child_node[i as usize] as
                        isize)).black_minimax_score as
                        i32 !=
                        (*g_book.node.offset(index as isize)).black_minimax_score
                            as i32 {
                    current_block_38 = 2873832966593178012;
                } else { current_block_38 = 10891380440665537214; }
            } else if g_book.force_white != 0 &&
                (*g_book.node.offset(child_node[i as usize] as
                    isize)).white_minimax_score as
                    i32 !=
                    (*g_book.node.offset(index as
                        isize)).white_minimax_score as
                        i32 {
                current_block_38 = 2873832966593178012;
            } else { current_block_38 = 10891380440665537214; }
            match current_block_38 {
                10891380440665537214 => {
                    this_move = child_move[i as usize];
                    make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
                    do_examine(child_node[i as usize]);
                    let move_0 = this_move;
                    {
                        unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
                    };
                }
                _ => { }
            }
            i += 1
        }
    }
    let ref mut fresh21 = (*g_book.node.offset(index as isize)).flags;
    *fresh21 = (*fresh21 as i32 ^ 8 as i32) as u16;
}


/*
   EVALUATE_NODE
   Applies a search to a predetermined depth to find the best
   alternative move in a position.
   Note: This function assumes that generate_all() has been
         called prior to it being called.
*/
pub unsafe fn evaluate_node<FE: FrontEnd>(index: i32, echo: i32) {
    let mut i: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut alternative_move_count: i32 = 0;
    let mut this_move: i32 = 0;
    let mut best_move: i32 = 0;
    let mut child: i32 = 0;
    let mut allow_mpc: i32 = 0;
    let mut depth: i32 = 0;
    let mut best_index: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut feasible_move: [i32; 64] = [0; 64];
    let mut best_score: i32 = 0;
    /* Don't evaluate nodes that already have been searched deep enough */
    depth = get_node_depth(index, &mut g_book);
    if depth >= g_book.search_depth &&
        (*g_book.node.offset(index as isize)).alternative_score as i32 !=
            9999 as i32 {
        return
    }
    /* If the g_book.node has been evaluated and its score is outside the
       eval and minimax windows, bail out. */
    if (*g_book.node.offset(index as isize)).alternative_score as i32 !=
        9999 as i32 {
        if abs((*g_book.node.offset(index as isize)).alternative_score as
            i32) < g_book.min_eval_span ||
            abs((*g_book.node.offset(index as isize)).alternative_score as
                i32) > g_book.max_eval_span {
            return
        }
        if abs((*g_book.node.offset(index as isize)).black_minimax_score as
            i32) < g_book.min_negamax_span ||
            abs((*g_book.node.offset(index as isize)).black_minimax_score as
                i32) > g_book.max_negamax_span {
            return
        }
    }
    if (*g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    remove_coeffs(moves_state.disks_played - 8 as i32, &mut coeff_state);
    g_timer.clear_panic_abort();
    board_state.piece_count[0][moves_state.disks_played as usize] =
        disc_count(0 as i32, &board_state.board);
    board_state.piece_count[2][moves_state.disks_played as usize] =
        disc_count(2 as i32, &board_state.board);
    /* Find the moves which haven't been tried from this position */
    alternative_move_count = 0;
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        this_move = moves_state.move_list[moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        child = *g_book.book_hash_table.offset(slot as isize);
        if child == -(1 as i32) {
            let fresh16 = alternative_move_count;
            alternative_move_count = alternative_move_count + 1;
            feasible_move[fresh16 as usize] = this_move
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i += 1
    }
    if alternative_move_count == 0 as i32 {
        /* There weren't any such moves */
        g_book.exhausted_node_count += 1;
        (*g_book.node.offset(index as isize)).best_alternative_move =
            -(2 as i32) as i16;
        (*g_book.node.offset(index as isize)).alternative_score =
            9999 as i32 as i16
    } else {
        /* Find the best of those moves */
        allow_mpc = (g_book.search_depth >= 9 as i32) as i32;
        nega_scout::<FE>(g_book.search_depth, allow_mpc, side_to_move,
                         alternative_move_count, &mut feasible_move,
                         -(12345678 as i32), 12345678 as i32,
                         &mut best_score, &mut best_index, echo);
        best_move = feasible_move[best_index as usize];
        g_book.evaluated_count += 1;
        if side_to_move == 0 as i32 {
            (*g_book.node.offset(index as isize)).alternative_score =
                best_score as i16
        } else {
            (*g_book.node.offset(index as isize)).alternative_score =
                -best_score as i16
        }
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        (*g_book.node.offset(index as isize)).best_alternative_move =
            *g_book.symmetry_map[orientation as usize].offset(best_move as isize) as
                i16
    }
    clear_node_depth(index, &mut g_book);
    set_node_depth(index, g_book.search_depth, &mut g_book);
}

/*
  NEGA_SCOUT
  This wrapper on top of TREE_SEARCH is used by EVALUATE_NODE
  to search the possible deviations.
*/
pub unsafe fn nega_scout<FE: FrontEnd>(depth: i32,
                                       allow_mpc: i32,
                                       side_to_move: i32,
                                       allowed_count: i32,
                                       allowed_moves: &mut [i32],
                                       _alpha: i32, _beta: i32,
                                       best_score: &mut i32,
                                       best_index: &mut i32, echo:i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut curr_alpha: i32 = 0;
    let mut curr_depth: i32 = 0;
    let mut low_score: i32 = 0;
    let mut high_score: i32 = 0;
    let mut best_move: i32 = 0;
    let mut current_score: i32 = 0;
    reset_counter(&mut search_state.nodes);
    low_score = -(12345678 as i32);
    /* To avoid spurious hash table entries to take out the effect
       of the averaging done, the hash table drafts are changed prior
       to each g_book.node being searched. */
    clear_hash_drafts(&mut hash_state);
    determine_hash_values(side_to_move, &board_state.board, &mut hash_state);
    /* First determine the best move in the current position
       and its score when searched to depth DEPTH.
       This is done using standard negascout with iterative deepening. */
    curr_depth = 2 as i32 - depth % 2 as i32;
    while curr_depth <= depth {
        low_score = -(12345678 as i32);
        curr_alpha = -(12345678 as i32);
        i = 0;
        while i < allowed_count {
            make_move(side_to_move, *allowed_moves.offset(i as isize),
                      1 as i32, &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
            board_state.piece_count[0][moves_state.disks_played as usize] =
                disc_count(0 as i32, &board_state.board);
            board_state.piece_count[2][moves_state.disks_played as usize] =
                disc_count(2 as i32, &board_state.board);
            g_timer.last_panic_check = 0.0f64;
            if i == 0 as i32 {
                current_score =
                    -tree_search::<FE>(1 as i32, curr_depth,
                                       0 as i32 + 2 as i32 -
                                           side_to_move, -(12345678 as i32),
                                       12345678 as i32, 1 as i32,
                                       allow_mpc, 1 as i32, echo);
                low_score = current_score;
                *best_index = i
            } else {
                curr_alpha =
                    if low_score > curr_alpha {
                        low_score
                    } else { curr_alpha };
                current_score =
                    -tree_search::<FE>(1 as i32, curr_depth,
                                       0 as i32 + 2 as i32 -
                                           side_to_move,
                                       -(curr_alpha + 1 as i32),
                                       -curr_alpha, 1 as i32, allow_mpc,
                                       1 as i32, echo);
                if current_score > curr_alpha {
                    current_score =
                        -tree_search::<FE>(1 as i32, curr_depth,
                                           0 as i32 + 2 as i32 -
                                               side_to_move,
                                           -(12345678 as i32),
                                           12345678 as i32,
                                           1 as i32, allow_mpc,
                                           1 as i32, echo);
                    if current_score > low_score {
                        low_score = current_score;
                        *best_index = i
                    }
                } else if current_score > low_score {
                    low_score = current_score;
                    *best_index = i
                }
            }
            let move_0 = *allowed_moves.offset(i as isize);
            {
                unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
            };
            i += 1
        }
        /* Float the best move so far to the top of the list */
        best_move = *allowed_moves.offset(*best_index as isize);
        j = *best_index;
        while j >= 1 as i32 {
            *allowed_moves.offset(j as isize) =
                *allowed_moves.offset((j - 1 as i32) as isize);
            j -= 1
        }
        allowed_moves[0] = best_move;
        *best_index = 0;
        curr_depth += 2 as i32
    }
    /* Then find the score for the best move when searched
       to depth DEPTH+1 */
    make_move(side_to_move, *allowed_moves.offset(*best_index as isize),
              1 as i32, &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
    board_state.piece_count[0][moves_state.disks_played as usize] =
        disc_count(0 as i32, &board_state.board);
    board_state.piece_count[2][moves_state.disks_played as usize] =
        disc_count(2 as i32, &board_state.board);
    g_timer.last_panic_check = 0.0f64;
    high_score =
        -tree_search::<FE>(1 as i32, depth + 1 as i32,
                           0 as i32 + 2 as i32 - side_to_move,
                           -(12345678 as i32), 12345678 as i32,
                           1 as i32, allow_mpc, 1 as i32, echo);
    let move_0 = *allowed_moves.offset(*best_index as isize);
    {
        unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
    };
    /* To remove the oscillations between odd and even search depths
       the score for the deviation is the average between the two scores. */
    *best_score = (low_score + high_score) / 2 as i32;
}



/*
   REBUILD_HASH_TABLE
   Resize the hash table for a requested number of nodes.
*/
pub fn rebuild_hash_table(book: &mut Book, requested_items: i32) {
    let new_size = 2 * requested_items;
    if book.hash_table_size == 0 {
        book.book_hash_table = vec![0; new_size as usize];
    } else {
        book.book_hash_table.resize(new_size as usize, 0);
    }
    book.hash_table_size = new_size;
    create_hash_reference(book);
}



/*
   SET_ALLOCATION
   Changes the number of nodes for which memory is allocated.
*/
pub fn set_allocation(size: i32, book: &mut Book) {
    if book.node.is_empty() {
        book.node = vec![BookNode {
            hash_val1: 0,
            hash_val2: 0,
            black_minimax_score: 0,
            white_minimax_score: 0,
            best_alternative_move: 0,
            alternative_score: 0,
            flags: 0
        }; size as usize];
    } else {
        book.node.resize(size as usize, BookNode {
            hash_val1: 0,
            hash_val2: 0,
            black_minimax_score: 0,
            white_minimax_score: 0,
            best_alternative_move: 0,
            alternative_score: 0,
            flags: 0
        });
    }
    book.node_table_size = size;
    if book.node_table_size as f64 > 0.80f64 * book.hash_table_size as f64 {
        rebuild_hash_table(book, book.node_table_size);
    };
}
/*
   INCREASE_ALLOCATION
   Allocate more memory for the book tree.
*/
pub fn increase_allocation(book: &mut Book) {
    set_allocation(book.node_table_size + 50000 as i32, book);
}
/*
   CREATE_BOOK_NODE
   Creates a new book node without any connections whatsoever
   to the rest of the tree.
*/
pub fn create_BookNode(val1: i32, val2: i32, flags: u16, book: &mut Book) -> i32 {
    let mut index: i32 = 0;
    if book.book_node_count == book.node_table_size { increase_allocation(book); }
    index = book.book_node_count;
    (*book.node.offset(index as isize)).hash_val1 = val1;
    (*book.node.offset(index as isize)).hash_val2 = val2;
    (*book.node.offset(index as isize)).black_minimax_score = 9999;
    (*book.node.offset(index as isize)).white_minimax_score = 9999;
    (*book.node.offset(index as isize)).best_alternative_move =
        -(1 as i32) as i16;
    (*book.node.offset(index as isize)).alternative_score = 9999;
    (*book.node.offset(index as isize)).flags = flags;
    select_hash_slot(index, book);
    book.book_node_count += 1;
    return index;
}

/*
   DO_MINIMAX
   Calculates the minimax value of g_book.node INDEX.
*/
pub unsafe fn do_minimax(index: i32,
                         black_score: &mut i32,
                         white_score: &mut i32) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut child_black_score: i32 = 0;
    let mut child_white_score: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut alternative_move: i32 = 0;
    let mut alternative_move_found: i32 = 0;
    let mut child_count: i32 = 0;
    let mut best_black_child_val: i32 = 0;
    let mut best_white_child_val: i32 = 0;
    let mut worst_black_child_val: i32 = 0;
    let mut worst_white_child_val: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut best_black_score: i16 = 0;
    let mut best_white_score: i16 = 0;
    /* If the node has been visited AND it is a midgame node, meaning
       that the minimax values are not to be tweaked, return the
       stored values. */
    if (*g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        if (*g_book.node.offset(index as isize)).flags as i32 &
            (4 as i32 | 16 as i32) == 0 {
            *black_score =
                (*g_book.node.offset(index as isize)).black_minimax_score as
                    i32;
            *white_score =
                (*g_book.node.offset(index as isize)).white_minimax_score as
                    i32;
            return
        }
    }
    /* Correct WLD solved nodes corresponding to draws to be represented
       as full solved and make sure full solved nodes are marked as
       WLD solved as well */
    if (*g_book.node.offset(index as isize)).flags as i32 & 4 as i32
        != 0 &&
        (*g_book.node.offset(index as isize)).black_minimax_score as i32
            == 0 as i32 &&
        (*g_book.node.offset(index as isize)).white_minimax_score as i32
            == 0 as i32 {
        let ref mut fresh2 = (*g_book.node.offset(index as isize)).flags;
        *fresh2 =
            (*fresh2 as i32 | 16 as i32) as u16
    }
    if (*g_book.node.offset(index as isize)).flags as i32 & 16 as i32
        != 0 &&
        (*g_book.node.offset(index as isize)).flags as i32 &
            4 as i32 == 0 {
        let ref mut fresh3 = (*g_book.node.offset(index as isize)).flags;
        *fresh3 =
            (*fresh3 as i32 | 4 as i32) as u16
    }
    /* Recursively minimax all children of the node */
    if (*g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    best_black_child_val = -(99999 as i32);
    best_white_child_val = -(99999 as i32);
    worst_black_child_val = 99999;
    worst_white_child_val = 99999;
    if (*g_book.node.offset(index as isize)).alternative_score as i32 !=
        9999 as i32 {
        best_black_score =
            adjust_score((*g_book.node.offset(index as isize)).alternative_score as
                             i32, side_to_move, &mut g_book, moves_state.disks_played) as i16;
        best_white_score = best_black_score;
        worst_black_child_val = best_black_score as i32;
        best_black_child_val = worst_black_child_val;
        worst_white_child_val = best_white_score as i32;
        best_white_child_val = worst_white_child_val;
        alternative_move_found = 0;
        alternative_move =
            (*g_book.node.offset(index as isize)).best_alternative_move as
                i32;
        if alternative_move > 0 as i32 {
            get_hash(&mut val1, &mut val2, &mut orientation, &mut g_book, &board_state.board);
            alternative_move =
                *g_book.inv_symmetry_map[orientation as
                    usize].offset(alternative_move as isize)
        }
    } else {
        alternative_move_found = 1;
        alternative_move = 0;
        if side_to_move == 0 as i32 {
            best_black_score = -(32000 as i32) as i16;
            best_white_score = -(32000 as i32) as i16
        } else {
            best_black_score = 32000;
            best_white_score = 32000 as i32 as i16
        }
    }
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
    child_count = 0;
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        board_state.piece_count[0][moves_state.disks_played as usize] =
            disc_count(0 as i32, &board_state.board);
        board_state.piece_count[2][moves_state.disks_played as usize] =
            disc_count(2 as i32, &board_state.board);
        this_move = moves_state.move_list[moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        get_hash(&mut val1, &mut val2, &mut orientation, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        child = *g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_minimax(child, &mut child_black_score, &mut child_white_score);
            best_black_child_val =
                if best_black_child_val > child_black_score {
                    best_black_child_val
                } else { child_black_score };
            best_white_child_val =
                if best_white_child_val > child_white_score {
                    best_white_child_val
                } else { child_white_score };
            worst_black_child_val =
                if worst_black_child_val < child_black_score {
                    worst_black_child_val
                } else { child_black_score };
            worst_white_child_val =
                if worst_white_child_val < child_white_score {
                    worst_white_child_val
                } else { child_white_score };
            if side_to_move == 0 as i32 {
                best_black_score =
                    if child_black_score > best_black_score as i32 {
                        child_black_score
                    } else { best_black_score as i32 } as
                        i16;
                best_white_score =
                    if child_white_score > best_white_score as i32 {
                        child_white_score
                    } else { best_white_score as i32 } as
                        i16
            } else {
                best_black_score =
                    if child_black_score < best_black_score as i32 {
                        child_black_score
                    } else { best_black_score as i32 } as
                        i16;
                best_white_score =
                    if child_white_score < best_white_score as i32 {
                        child_white_score
                    } else { best_white_score as i32 } as
                        i16
            }
            child_count += 1
        } else if alternative_move_found == 0 && this_move == alternative_move
        {
            alternative_move_found = 1 as i32
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i += 1
    }
    if alternative_move_found == 0 {
        /* The was-to-be deviation now leads to a position in the database,
           hence it can no longer be used. */
        (*g_book.node.offset(index as isize)).alternative_score = 9999;
        (*g_book.node.offset(index as isize)).best_alternative_move =
            -(1 as i32) as i16
    }
    /* Try to infer the WLD status from the children */
    if (*g_book.node.offset(index as isize)).flags as i32 &
        (16 as i32 | 4 as i32) == 0 &&
        child_count > 0 as i32 {
        if side_to_move == 0 as i32 {
            if best_black_child_val >= 30000 as i32 &&
                best_white_child_val >= 30000 as i32 {
                /* Black win */
                let ref mut fresh4 =
                    (*g_book.node.offset(index as isize)).white_minimax_score;
                *fresh4 =
                    if best_black_child_val < best_white_child_val {
                        best_black_child_val
                    } else { best_white_child_val } as i16;
                (*g_book.node.offset(index as isize)).black_minimax_score = *fresh4;
                let ref mut fresh5 = (*g_book.node.offset(index as isize)).flags;
                *fresh5 =
                    (*fresh5 as i32 | 4 as i32) as
                        u16
            } else if best_black_child_val <= -(30000 as i32) &&
                best_white_child_val <= -(30000 as i32) {
                /* Black loss */
                let ref mut fresh6 =
                    (*g_book.node.offset(index as isize)).white_minimax_score;
                *fresh6 =
                    if best_black_child_val > best_white_child_val {
                        best_black_child_val
                    } else { best_white_child_val } as i16;
                (*g_book.node.offset(index as isize)).black_minimax_score = *fresh6;
                let ref mut fresh7 = (*g_book.node.offset(index as isize)).flags;
                *fresh7 =
                    (*fresh7 as i32 | 4 as i32) as
                        u16
            }
        } else if worst_black_child_val <= -(30000 as i32) &&
            worst_white_child_val <= -(30000 as i32) {
            /* White win */
            let ref mut fresh8 =
                (*g_book.node.offset(index as isize)).white_minimax_score;
            *fresh8 =
                if worst_black_child_val > worst_white_child_val {
                    worst_black_child_val
                } else { worst_white_child_val } as i16;
            (*g_book.node.offset(index as isize)).black_minimax_score = *fresh8;
            let ref mut fresh9 = (*g_book.node.offset(index as isize)).flags;
            *fresh9 =
                (*fresh9 as i32 | 4 as i32) as u16
        } else if worst_black_child_val >= 30000 as i32 &&
            worst_white_child_val >= 30000 as i32 {
            /* White loss */
            let ref mut fresh10 =
                (*g_book.node.offset(index as isize)).white_minimax_score;
            *fresh10 =
                if worst_black_child_val < worst_white_child_val {
                    worst_black_child_val
                } else { worst_white_child_val } as i16;
            (*g_book.node.offset(index as isize)).black_minimax_score = *fresh10;
            let ref mut fresh11 = (*g_book.node.offset(index as isize)).flags;
            *fresh11 =
                (*fresh11 as i32 | 4 as i32) as u16
        }
    }
    /* Tweak the minimax scores for draws to give the right
       draw avoidance behavior */
    if (*g_book.node.offset(index as isize)).flags as i32 &
        (16 as i32 | 4 as i32) != 0 {
        *black_score =
            (*g_book.node.offset(index as isize)).black_minimax_score as i32;
        *white_score =
            (*g_book.node.offset(index as isize)).white_minimax_score as i32;
        if (*g_book.node.offset(index as isize)).black_minimax_score as i32
            == 0 as i32 &&
            (*g_book.node.offset(index as isize)).white_minimax_score as
                i32 == 0 as i32 {
            /* Is it a position in which a draw should be avoided? */
            if g_book.game_mode as u32 ==
                PRIVATE_GAME as i32 as u32 ||
                (*g_book.node.offset(index as isize)).flags as i32 &
                    32 as i32 == 0 {
                match g_book.draw_mode as u32 {
                    1 => {
                        *black_score =
                            30000 as i32 - 1 as i32;
                        *white_score = 30000 as i32 - 1 as i32
                    }
                    2 => {
                        *black_score =
                            -(30000 as i32 - 1 as i32);
                        *white_score =
                            -(30000 as i32 - 1 as i32)
                    }
                    3 => {
                        *black_score =
                            -(30000 as i32 - 1 as i32);
                        *white_score = 30000 as i32 - 1 as i32
                    }
                    0 | _ => { }
                }
            }
        }
    } else {
        let ref mut fresh12 =
            (*g_book.node.offset(index as isize)).black_minimax_score;
        *fresh12 = best_black_score;
        *black_score = *fresh12 as i32;
        let ref mut fresh13 =
            (*g_book.node.offset(index as isize)).white_minimax_score;
        *fresh13 = best_white_score;
        *white_score = *fresh13 as i32
    }
    let ref mut fresh14 = (*g_book.node.offset(index as isize)).flags;
    *fresh14 = (*fresh14 as i32 ^ 8 as i32) as u16;
}



pub unsafe fn engine_init_osf<FE: FrontEnd>() {
    init_maps::<FE>();
    prepare_hash(&mut g_book, &mut engine::src::myrandom::random_instance);
    setup_hash(1 as i32, &mut hash_state, &mut random_instance);
    init_book_tree(&mut g_book);
    reset_book_search(&mut g_book);
    g_book.search_depth = 2;
    g_book.max_slack = 0;
    g_book.low_deviation_threshold = 60;
    g_book.high_deviation_threshold = 60;
    g_book.deviation_bonus = 0.0f64;
    g_book.min_eval_span = 0;
    g_book.max_eval_span = 1000 as i32 * 128 as i32;
    g_book.min_negamax_span = 0;
    g_book.max_negamax_span = 1000 as i32 * 128 as i32;
    g_book.max_batch_size = 10000000;
    g_book.force_black = 0;
    g_book.force_white = 0;
}


/*
   PREPATE_TREE_TRAVERSAL
   Prepares all relevant data structures for a tree search
   or traversal.
*/
pub unsafe fn prepare_tree_traversal() {
    let mut side_to_move: i32 = 0;
    setup_non_file_based_game(&mut side_to_move);
    engine_game_init();
    midgame_state.toggle_midgame_hash_usage(1 as i32, 1 as i32);
    g_timer.toggle_abort_check(0 as i32);
    midgame_state.toggle_midgame_abort_check(0 as i32);
}



/*
   INIT_MAPS
   Initializes the 8 symmetry maps.
   Notice that the order of these MUST coincide with the returned
   orientation value from get_hash() OR YOU WILL LOSE BIG.
*/
pub unsafe fn init_maps<FE: FrontEnd>() {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut pos = 0;
    i = 1;
    while i <= 8 {
        j = 1;
        while j <= 8 {
            pos = 10  * i + j;
            g_book.b1_b1_map[pos as usize] = pos;
            g_book.g1_b1_map[pos as usize] = 10 * i + (9 - j);
            g_book.g8_b1_map[pos as usize] = 10 * (9 - i) + (9 - j);
            g_book.b8_b1_map[pos as usize] = 10 * (9 - i) + j;
            g_book.a2_b1_map[pos as usize] = 10 * j + i;
            g_book.a7_b1_map[pos as usize] = 10 * j + (9 - i);
            g_book.h7_b1_map[pos as usize] = 10 * (9 - j) + (9 - i);
            g_book.h2_b1_map[pos as usize] = 10 * (9 - j) + i;
            j += 1
        }
        i += 1
    }
    g_book.symmetry_map[0] = g_book.b1_b1_map.as_mut_ptr();
    g_book.inv_symmetry_map[0] = g_book.b1_b1_map.as_mut_ptr();
    g_book.symmetry_map[1] = g_book.g1_b1_map.as_mut_ptr();
    g_book.inv_symmetry_map[1] = g_book.g1_b1_map.as_mut_ptr();
    g_book.symmetry_map[2] = g_book.g8_b1_map.as_mut_ptr();
    g_book.inv_symmetry_map[2] = g_book.g8_b1_map.as_mut_ptr();
    g_book.symmetry_map[3] = g_book.b8_b1_map.as_mut_ptr();
    g_book.inv_symmetry_map[3] = g_book.b8_b1_map.as_mut_ptr();
    g_book.symmetry_map[4] = g_book.a2_b1_map.as_mut_ptr();
    g_book.inv_symmetry_map[4] = g_book.a2_b1_map.as_mut_ptr();
    g_book.symmetry_map[5] = g_book.a7_b1_map.as_mut_ptr();
    g_book.inv_symmetry_map[5] = g_book.h2_b1_map.as_mut_ptr();
    g_book.symmetry_map[6] = g_book.h7_b1_map.as_mut_ptr();
    g_book.inv_symmetry_map[6] = g_book.h7_b1_map.as_mut_ptr();
    g_book.symmetry_map[7] = g_book.h2_b1_map.as_mut_ptr();
    g_book.inv_symmetry_map[7] = g_book.a7_b1_map.as_mut_ptr();
    i = 0;
    while i < 8 as i32 {
        *g_book.symmetry_map[i as usize] = 0;
        i += 1
    }
    i = 0;
    while i < 8 as i32 {
        j = 1;
        while j <= 8 as i32 {
            k = 1;
            while k <= 8 as i32 {
                pos = 10 as i32 * j + k;
                if *g_book.inv_symmetry_map[i as usize]
                    .offset(*g_book.symmetry_map[i as usize].offset(pos as isize) as isize) != pos {
                    let symmetry_map_item = *g_book.inv_symmetry_map[i as usize].offset(*g_book.symmetry_map[i as usize].offset(pos as isize) as isize);
                    FE::error_in_map(i, pos, symmetry_map_item);
                }
                k += 1
            }
            j += 1
        }
        i += 1
    };
}


/*
   DO_COMPRESS
   Compresses the subtree below the current node.
*/
pub unsafe fn do_compress(index: i32,
                          node_order: *mut i32,
                          child_count: *mut i16,
                          node_index: &mut i32,
                          child_list: *mut i16,
                          child_index: &mut i32) {
    use engine_traits::Offset;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut child: i32 = 0;
    let mut valid_child_count: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut found: i32 = 0;
    let mut local_child_list: [i32; 64] = [0; 64];
    let mut this_move: i16 = 0;
    let mut local_child_move: [i16; 64] = [0; 64];
    if (*g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    *node_order.offset(*node_index as isize) = index;
    if (*g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    valid_child_count = 0;
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        this_move =
            moves_state.move_list[moves_state.disks_played as usize][i as usize] as i16;
        make_move(side_to_move, this_move as i32, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        get_hash(&mut val1, &mut val2, &mut orientation, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        child = *g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) &&
            (*g_book.node.offset(child as isize)).flags as i32 &
                8 as i32 != 0 {
            j = 0;
            found = 0;
            while j < valid_child_count {
                if child == local_child_list[j as usize] {
                    found = 1 as i32
                }
                j += 1
            }
            if found == 0 {
                local_child_list[valid_child_count as usize] = child;
                local_child_move[valid_child_count as usize] = this_move;
                valid_child_count += 1;
                *child_list.offset(*child_index as isize) = this_move;
                *child_index += 1
            }
        }
        let move_0 = this_move as i32;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i += 1
    }
    *child_count.offset(*node_index as isize) =
        valid_child_count as i16;
    *node_index += 1;
    i = 0;
    while i < valid_child_count {
        this_move = local_child_move[i as usize];
        make_move(side_to_move, this_move as i32, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        do_compress(local_child_list[i as usize], node_order, child_count,
                    node_index, child_list, child_index);
        let move_0 = this_move as i32;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i += 1
    }
    let ref mut fresh44 = (*g_book.node.offset(index as isize)).flags;
    *fresh44 = (*fresh44 as i32 ^ 8 as i32) as u16;
}


/*
   INIT_BOOK_TREE
   Initializes the node tree by creating the root of the tree.
*/
pub fn init_book_tree(book: &mut Book) {
    book.book_node_count = 0;
    book.node = Vec::new();
}

/*
   PREPARE_HASH
   Compute the position hash codes.
*/
pub fn prepare_hash(book: &mut Book, random: &mut MyRandom) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    /* The hash keys are static, hence the same keys must be
       produced every time the program is run. */
    let x = 0 as i32;
    random.my_srandom(x);
    i = 0;
    while i < 2 {
        j = 0;
        while j < 8 {
            k = 0;
            while k < 6561 {
                book.line_hash[i][j][k] =
                    if random.my_random() % 2 as i64 != 0 {
                        random.my_random()
                    } else { -random.my_random() } as i32;
                k += 1
            }
            j += 1
        }
        i += 1
    }
    book.hash_table_size = 0;
}

/*
   CREATE_HASH_REFERENCEE
   Takes the node list and fills the hash table with indices
   into the node list.
*/
pub fn create_hash_reference(book: &mut Book) {
    let mut i = 0;
    while i < book.hash_table_size {
        *book.book_hash_table.offset(i as isize) = -(1 as i32);
        i += 1
    }
    let mut i = 0;
    while i < book.book_node_count {
        select_hash_slot(i, book);
        i += 1
    };
}


/*
   SELECT_HASH_SLOT
   Finds a slot in the hash table for the node INDEX
   using linear probing.
*/
pub fn select_hash_slot(index: i32, book: &mut Book) {
    let mut slot: i32 = 0;
    slot = (*book.node.offset(index as isize)).hash_val1 % book.hash_table_size;
    while *book.book_hash_table.offset(slot as isize) != -(1 as i32) {
        slot = (slot + 1 as i32) % book.hash_table_size
    }
    *book.book_hash_table.offset(slot as isize) = index;
}
