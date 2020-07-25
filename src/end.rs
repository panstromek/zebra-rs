pub use engine::src::end::*;
use engine::{
    src:: {
        epcstat::{end_sigma, end_mean},
        moves::{dir_mask, disks_played, unmake_move, make_move, move_count, generate_all, move_list, valid_move},
        search::{force_return, hash_expand_pv, root_eval, store_pv, restore_pv, nodes, create_eval_info, disc_count, get_ponder_move, set_current_eval, select_move, evals, sorted_move_order},
        hash::{hash_flip_color2, hash2, hash_flip_color1, hash1, add_hash_extended, find_hash, HashEntry, hash_put_value2, hash_put_value1},
        unflip::UndoFlips,
        doflip::{hash_update2, hash_update1, DoFlips_hash},
        bitbcnt::CountFlips_bitboard,
        bitboard::{set_bitboards, BitBoard},
        bitbmob::{init_mmx, bitboard_mobility, weighted_mobility},
        bitbtest::{bb_flips, TestFlips_bitboard},
        probcut::{end_mpc_depth, use_end_cut},
        stable::{count_stable, count_edge_stable},
        counter::{adjust_counter, counter_value},
        globals::{piece_count, board, pv_depth, pv},
    }
};
use crate::{
    src::{
        libc,
        stubs::{ceil, abs, printf, free, fflush, sprintf, puts, stdout},
        display::{display_status, echo, reset_buffer_display, send_status, send_status_time, send_status_pv, send_status_nodes, produce_eval_text, clear_status, display_sweep, send_sweep, display_buffers, clear_sweep},
        timer::{clear_panic_abort, get_elapsed_time, is_panic_abort, check_panic_abort, check_threshold, set_panic_threshold, last_panic_check},
        midgame::{toggle_midgame_hash_usage, tree_search},
        osfbook::{fill_endgame_hash, get_book_move, fill_move_alternatives},
        hash::{add_hash},
        zebra::{EvaluationType, _IO_FILE}
    }
};

pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
/*
   File:          search.h

   Created:       July 1, 1997

   Modified:      August 1, 2002

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The interface to common search routines and variables.
*/
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

/*
  UPDATE_BEST_LIST
*/
unsafe fn update_best_list(mut best_list: *mut i32,
                                      mut move_0: i32,
                                      mut best_list_index: i32,
                                      mut best_list_length: *mut i32,
                                      mut verbose: i32) {
    verbose = 0 as i32;
    if verbose != 0 {
        before_update_best_list_verbose(best_list, move_0, best_list_index, best_list_length)
    }
    if best_list_index < *best_list_length {
        let mut i = best_list_index;
        while i >= 1 as i32 {
            *best_list.offset(i as isize) =
                *best_list.offset((i - 1 as i32) as isize);
            i -= 1
        }
    } else {
        let mut i = 3 as i32;
        while i >= 1 as i32 {
            *best_list.offset(i as isize) =
                *best_list.offset((i - 1 as i32) as isize);
            i -= 1
        }
        if *best_list_length < 4 as i32 { *best_list_length += 1 }
    }
    *best_list.offset(0 as i32 as isize) = move_0;
    if verbose != 0 {
        after_update_best_list_verbose(best_list);
    };
}
#[no_mangle]
pub unsafe extern "C" fn after_update_best_list_verbose(best_list: *mut i32) {
    printf(b"      After:  \x00" as *const u8 as *const i8);
    let mut i = 0 as i32;
    while i < 4 as i32 {
        printf(b"%2d \x00" as *const u8 as *const i8,
               *best_list.offset(i as isize));
        i += 1
    }
    puts(b"\x00" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C"  fn before_update_best_list_verbose(best_list: *mut i32, mut move_0: i32, mut best_list_index: i32, best_list_length: *mut i32) {
    let mut i: i32 = 0;
    printf(b"move=%2d  index=%d  length=%d      \x00" as *const u8 as
               *const i8, move_0, best_list_index,
           *best_list_length);
    printf(b"Before:  \x00" as *const u8 as *const i8);
    i = 0 as i32;
    while i < 4 as i32 {
        printf(b"%2d \x00" as *const u8 as *const i8,
               *best_list.offset(i as isize));
        i += 1
    }
}
/*
  END_TREE_SEARCH
  Plain nega-scout with fastest-first move ordering.
*/
unsafe fn end_tree_search(mut level: i32,
                                     mut max_depth: i32,
                                     mut my_bits: BitBoard,
                                     mut opp_bits: BitBoard,
                                     mut side_to_move: i32,
                                     mut alpha: i32,
                                     mut beta: i32,
                                     mut selectivity: i32,
                                     mut selective_cutoff: *mut i32,
                                     mut void_legal: i32)
 -> i32 {
    static mut buffer: [i8; 16] = [0; 16];
    let mut node_val: f64 = 0.;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut empties: i32 = 0;
    let mut disk_diff: i32 = 0;
    let mut previous_move: i32 = 0;
    let mut result: i32 = 0;
    let mut curr_val: i32 = 0;
    let mut best: i32 = 0;
    let mut move_0: i32 = 0;
    let mut hash_hit: i32 = 0;
    let mut move_index: i32 = 0;
    let mut remains: i32 = 0;
    let mut exp_depth: i32 = 0;
    let mut pre_depth: i32 = 0;
    let mut update_pv: i32 = 0;
    let mut first: i32 = 0;
    let mut use_hash: i32 = 0;
    let mut my_discs: i32 = 0;
    let mut opp_discs: i32 = 0;
    let mut curr_alpha: i32 = 0;
    let mut pre_search_done: i32 = 0;
    let mut mobility: i32 = 0;
    let mut threshold: i32 = 0;
    let mut best_list_index: i32 = 0;
    let mut best_list_length: i32 = 0;
    let mut best_list: [i32; 4] = [0; 4];
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    let mut mid_entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    let mut stability_bound: i32 = 0;
    if level == 0 as i32 {
        sprintf(buffer.as_mut_ptr(),
                b"[%d,%d]:\x00" as *const u8 as *const i8, alpha,
                beta);
        clear_sweep();
    }
    remains = max_depth - level;
    *selective_cutoff = 0 as i32;
    /* Always (almost) check for stability cutoff in this region of search */
    if alpha >= 24 as i32 {
        stability_bound =
            64 as i32 -
                2 as i32 *
                    count_edge_stable(0 as i32 + 2 as i32 -
                                          side_to_move, opp_bits, my_bits);
        if stability_bound <= alpha {
            pv_depth[level as usize] = level;
            return alpha
        }
        stability_bound =
            64 as i32 -
                2 as i32 *
                    count_stable(0 as i32 + 2 as i32 -
                                     side_to_move, opp_bits, my_bits);
        if stability_bound < beta {
            beta = stability_bound + 1 as i32
        }
        if stability_bound <= alpha {
            pv_depth[level as usize] = level;
            return alpha
        }
    }
    /* Check if the low-level code is to be invoked */
    my_discs = piece_count[side_to_move as usize][disks_played as usize];
    opp_discs =
        piece_count[(0 as i32 + 2 as i32 - side_to_move) as
                        usize][disks_played as usize];
    empties = 64 as i32 - my_discs - opp_discs;
    if remains <= 12 as i32 {
        disk_diff = my_discs - opp_discs;
        if void_legal != 0 {
            /* Is PASS legal or was last move a pass? */
            previous_move = 44 as i32
        } else {
            previous_move = 0 as i32
        } /* d4, of course impossible */
        prepare_to_solve(board.as_mut_ptr());
        result =
            end_solve(my_bits, opp_bits, alpha, beta, side_to_move, empties,
                      disk_diff, previous_move);
        pv_depth[level as usize] = level + 1 as i32;
        pv[level as usize][level as usize] = best_move;
        if level == 0 as i32 && get_ponder_move() == 0 {
            send_sweep(b"%-10s \x00" as *const u8 as *const i8,
                       buffer.as_mut_ptr());
            send_sweep(b"%c%c\x00" as *const u8 as *const i8,
                       'a' as i32 + best_move % 10 as i32 -
                           1 as i32,
                       '0' as i32 + best_move / 10 as i32);
            if result <= alpha {
                send_sweep(b"<%d\x00" as *const u8 as *const i8,
                           result + 1 as i32);
            } else if result >= beta {
                send_sweep(b">%d\x00" as *const u8 as *const i8,
                           result - 1 as i32);
            } else {
                send_sweep(b"=%d\x00" as *const u8 as *const i8,
                           result);
            }
        }
        return result
    }
    /* Otherwise normal search */
    nodes.lo = nodes.lo.wrapping_add(1);
    use_hash = 1 as i32;
    if use_hash != 0 {
        /* Check for endgame hash table move */
        find_hash(&mut entry, 1 as i32);
        if entry.draft as i32 == remains &&
               entry.selectivity as i32 <= selectivity &&
               valid_move(entry.move_0[0 as i32 as usize],
                          side_to_move) != 0 &&
               entry.flags as i32 & 16 as i32 != 0 &&
               (entry.flags as i32 & 4 as i32 != 0 ||
                    entry.flags as i32 & 1 as i32 != 0 &&
                        entry.eval >= beta ||
                    entry.flags as i32 & 2 as i32 != 0 &&
                        entry.eval <= alpha) {
            pv[level as usize][level as usize] =
                entry.move_0[0 as i32 as usize];
            pv_depth[level as usize] = level + 1 as i32;
            if level == 0 as i32 && get_ponder_move() == 0 {
                /* Output some stats */
                send_sweep(b"%c%c\x00" as *const u8 as *const i8,
                           'a' as i32 +
                               entry.move_0[0 as i32 as usize] %
                                   10 as i32 - 1 as i32,
                           '0' as i32 +
                               entry.move_0[0 as i32 as usize] /
                                   10 as i32);
                if entry.flags as i32 & 16 as i32 != 0 &&
                       entry.flags as i32 & 4 as i32 != 0 {
                    send_sweep(b"=%d\x00" as *const u8 as *const i8,
                               entry.eval);
                } else if entry.flags as i32 & 16 as i32 != 0
                              &&
                              entry.flags as i32 & 1 as i32 !=
                                  0 {
                    send_sweep(b">%d\x00" as *const u8 as *const i8,
                               entry.eval - 1 as i32);
                } else {
                    send_sweep(b"<%d\x00" as *const u8 as *const i8,
                               entry.eval + 1 as i32);
                }
                fflush(stdout);
            }
            if entry.selectivity as i32 > 0 as i32 {
                *selective_cutoff = 1 as i32
            }
            return entry.eval
        }
        hash_hit =
            (entry.draft as i32 != 0 as i32) as i32;
        /* If not any such found, check for a midgame hash move */
        find_hash(&mut mid_entry, 0 as i32);
        if mid_entry.draft as i32 != 0 as i32 &&
               mid_entry.flags as i32 & 8 as i32 != 0 {
            if level <= 4 as i32 ||
                   mid_entry.flags as i32 &
                       (4 as i32 | 1 as i32) != 0 {
                /* Give the midgame move full priority if we're are the root
                   of the tree, no endgame hash move was found and the position
                   isn't in the wipeout zone. */
                if level == 0 as i32 && hash_hit == 0 &&
                       mid_entry.eval < 60 as i32 * 128 as i32
                   {
                    entry = mid_entry;
                    hash_hit = 1 as i32
                }
            }
        }
    }
    /* Use endgame multi-prob-cut to selectively prune the tree */
    if 1 as i32 != 0 && level > 2 as i32 &&
           selectivity > 0 as i32 {
        let mut cut: i32 = 0;
        cut = 0 as i32;
        while cut < use_end_cut[disks_played as usize] {
            let mut shallow_remains =
                end_mpc_depth[disks_played as usize][cut as usize];
            let mut mpc_bias =
                ceil(end_mean[disks_played as usize][shallow_remains as usize]
                         as f64 * 128.0f64) as i32;
            let mut mpc_window =
                ceil(end_sigma[disks_played as
                                   usize][shallow_remains as usize] as
                         f64 * end_percentile[selectivity as usize]
                         * 128.0f64) as i32;
            let mut beta_bound =
                128 as i32 * beta + mpc_bias + mpc_window;
            let mut alpha_bound =
                128 as i32 * alpha + mpc_bias - mpc_window;
            let mut shallow_val =
                tree_search(level, level + shallow_remains, side_to_move,
                            alpha_bound, beta_bound, use_hash,
                            0 as i32, void_legal);
            if shallow_val >= beta_bound {
                if use_hash != 0 {
                    add_hash(1 as i32, alpha,
                             pv[level as usize][level as usize],
                             16 as i32 | 1 as i32, remains,
                             selectivity);
                }
                *selective_cutoff = 1 as i32;
                return beta
            }
            if shallow_val <= alpha_bound {
                if use_hash != 0 {
                    add_hash(1 as i32, beta,
                             pv[level as usize][level as usize],
                             16 as i32 | 2 as i32, remains,
                             selectivity);
                }
                *selective_cutoff = 1 as i32;
                return alpha
            }
            cut += 1
        }
    }
    /* Determine the depth of the shallow search used to find
       achieve good move sorting */
    if remains >= 15 as i32 {
        if remains >= 20 as i32 {
            if remains >= 24 as i32 {
                if remains >= 30 as i32 {
                    pre_depth = 6 as i32
                } else { pre_depth = 4 as i32 }
            } else { pre_depth = 3 as i32 }
        } else { pre_depth = 2 as i32 }
    } else { pre_depth = 1 as i32 }
    if level == 0 as i32 {
        /* Deeper pre-search from the root */
        pre_depth += 2 as i32;
        if pre_depth % 2 as i32 == 1 as i32 {
            /* Avoid odd depths from the root */
            pre_depth += 1
        }
    }
    /* The nega-scout search */
    exp_depth = remains;
    first = 1 as i32;
    best = -(12345678 as i32);
    pre_search_done = 0 as i32;
    curr_alpha = alpha;
    /* Initialize the move list and check the hash table move list */
    move_count[disks_played as usize] = 0 as i32;
    best_list_length = 0 as i32;
    i = 0 as i32;
    while i < 4 as i32 {
        best_list[i as usize] = 0 as i32;
        i += 1
    }
    if hash_hit != 0 {
        i = 0 as i32;
        while i < 4 as i32 {
            if valid_move(entry.move_0[i as usize], side_to_move) != 0 {
                let fresh0 = best_list_length;
                best_list_length = best_list_length + 1;
                best_list[fresh0 as usize] = entry.move_0[i as usize];
                /* Check for ETC among the hash table moves */
                if use_hash != 0 &&
                       make_move(side_to_move, entry.move_0[i as usize],
                                 1 as i32) != 0 as i32 {
                    let mut etc_entry =
                        HashEntry{key1: 0,
                                  key2: 0,
                                  eval: 0,
                                  move_0: [0; 4],
                                  draft: 0,
                                  selectivity: 0,
                                  flags: 0,};
                    find_hash(&mut etc_entry, 1 as i32);
                    if etc_entry.flags as i32 & 16 as i32 != 0
                           &&
                           etc_entry.draft as i32 ==
                               empties - 1 as i32 &&
                           etc_entry.selectivity as i32 <= selectivity
                           &&
                           etc_entry.flags as i32 &
                               (2 as i32 | 4 as i32) != 0 &&
                           etc_entry.eval <= -beta {
                        /* Immediate cutoff from this move, move it up front */
                        j = best_list_length - 1 as i32;
                        while j >= 1 as i32 {
                            best_list[j as usize] =
                                best_list[(j - 1 as i32) as usize];
                            j -= 1
                        }
                        best_list[0 as i32 as usize] =
                            entry.move_0[i as usize]
                    }
                    unmake_move(side_to_move, entry.move_0[i as usize]);
                }
            }
            i += 1
        }
    }
    move_index = 0 as i32;
    best_list_index = 0 as i32;
    loop  {
        let mut child_selective_cutoff: i32 = 0;
        let mut new_my_bits = BitBoard{high: 0, low: 0,};
        let mut new_opp_bits = BitBoard{high: 0, low: 0,};
        /* Use results of shallow searches to determine the move order */
        if best_list_index < best_list_length {
            move_0 = best_list[best_list_index as usize];
            move_count[disks_played as usize] += 1
        } else {
            if pre_search_done == 0 {
                let mut shallow_index: i32 = 0;
                pre_search_done = 1 as i32;
                threshold =
                    if (60 as i32 * 128 as i32) <
                           128 as i32 * alpha +
                               fast_first_threshold[disks_played as
                                                        usize][pre_depth as
                                                                   usize] {
                        (60 as i32) * 128 as i32
                    } else {
                        (128 as i32 * alpha) +
                            fast_first_threshold[disks_played as
                                                     usize][pre_depth as
                                                                usize]
                    };
                shallow_index = 0 as i32;
                while shallow_index < 60 as i32 {
                    let mut already_checked: i32 = 0;
                    move_0 =
                        sorted_move_order[disks_played as
                                              usize][shallow_index as usize];
                    already_checked = 0 as i32;
                    j = 0 as i32;
                    while j < best_list_length {
                        if move_0 == best_list[j as usize] {
                            already_checked = 1 as i32
                        }
                        j += 1
                    }
                    if already_checked == 0 &&
                           board[move_0 as usize] == 1 as i32 &&
                           TestFlips_wrapper(move_0, my_bits, opp_bits) >
                               0 as i32 {
                        new_opp_bits.high = opp_bits.high & !bb_flips.high;
                        new_opp_bits.low = opp_bits.low & !bb_flips.low;
                        make_move(side_to_move, move_0, 1 as i32);
                        curr_val = 0 as i32;
                        /* Enhanced Transposition Cutoff: It's a good idea to
                           transpose back into a position in the hash table. */
                        if use_hash != 0 {
                            let mut etc_entry_0 =
                                HashEntry{key1: 0,
                                          key2: 0,
                                          eval: 0,
                                          move_0: [0; 4],
                                          draft: 0,
                                          selectivity: 0,
                                          flags: 0,};
                            find_hash(&mut etc_entry_0, 1 as i32);
                            if etc_entry_0.flags as i32 &
                                   16 as i32 != 0 &&
                                   etc_entry_0.draft as i32 ==
                                       empties - 1 as i32 {
                                curr_val += 384 as i32;
                                if etc_entry_0.selectivity as i32 <=
                                       selectivity {
                                    if etc_entry_0.flags as i32 &
                                           (2 as i32 |
                                                4 as i32) != 0 &&
                                           etc_entry_0.eval <= -beta {
                                        curr_val = 10000000 as i32
                                    }
                                    if etc_entry_0.flags as i32 &
                                           1 as i32 != 0 &&
                                           etc_entry_0.eval >= -alpha {
                                        curr_val -= 640 as i32
                                    }
                                }
                            }
                        }
                        /* Determine the midgame score. If it is worse than
                           alpha-8, a fail-high is likely so precision in that
                           range is not worth the extra nodes required. */
                        if curr_val != 10000000 as i32 {
                            curr_val -=
                                tree_search(level + 1 as i32,
                                            level + pre_depth,
                                            0 as i32 +
                                                2 as i32 -
                                                side_to_move,
                                            -(12345678 as i32),
                                            (-alpha + 8 as i32) *
                                                128 as i32,
                                            1 as i32,
                                            1 as i32,
                                            1 as i32)
                        }
                        /* Make the moves which are highly likely to result in
                           fail-high in decreasing order of mobility for the
                           opponent. */
                        if curr_val > threshold ||
                               move_0 ==
                                   mid_entry.move_0[0 as i32 as usize]
                           {
                            if curr_val >
                                   60 as i32 * 128 as i32 {
                                curr_val +=
                                    2 as i32 * 1000000 as i32
                            } else { curr_val += 1000000 as i32 }
                            if curr_val < 10000000 as i32 {
                                mobility =
                                    bitboard_mobility(new_opp_bits, bb_flips);
                                if curr_val >
                                       2 as i32 *
                                           1000000 as i32 {
                                    curr_val -=
                                        2 as i32 *
                                            ff_mob_factor[(disks_played -
                                                               1 as
                                                                   i32)
                                                              as usize] *
                                            mobility
                                } else {
                                    curr_val -=
                                        ff_mob_factor[(disks_played -
                                                           1 as i32)
                                                          as usize] * mobility
                                }
                            }
                        }
                        unmake_move(side_to_move, move_0);
                        evals[disks_played as usize][move_0 as usize] =
                            curr_val;
                        move_list[disks_played as
                                      usize][move_count[disks_played as usize]
                                                 as usize] = move_0;
                        move_count[disks_played as usize] += 1
                    }
                    shallow_index += 1
                }
            }
            if move_index == move_count[disks_played as usize] { break ; }
            move_0 =
                select_move(move_index, move_count[disks_played as usize])
        }
        node_val = counter_value(&mut nodes);
        if node_val - last_panic_check >= 250000.0f64 {
            /* Check for time abort */
            last_panic_check = node_val;
            check_panic_abort();
            /* Output status buffers if in interactive mode */
            if echo != 0 { display_buffers(); }
            /* Check for events */
            if is_panic_abort() != 0 || force_return != 0 {
                return -(27000 as i32)
            }
        }
        if level == 0 as i32 && get_ponder_move() == 0 {
            if first != 0 {
                send_sweep(b"%-10s \x00" as *const u8 as *const i8,
                           buffer.as_mut_ptr());
            }
            send_sweep(b"%c%c\x00" as *const u8 as *const i8,
                       'a' as i32 + move_0 % 10 as i32 -
                           1 as i32,
                       '0' as i32 + move_0 / 10 as i32);
        }
        make_move(side_to_move, move_0, use_hash);
        TestFlips_wrapper(move_0, my_bits, opp_bits);
        new_my_bits = bb_flips;
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        update_pv = 0 as i32;
        if first != 0 {
            curr_val =
                -end_tree_search(level + 1 as i32, level + exp_depth,
                                 new_opp_bits, new_my_bits,
                                 0 as i32 + 2 as i32 -
                                     side_to_move, -beta, -curr_alpha,
                                 selectivity, &mut child_selective_cutoff,
                                 1 as i32);
            best = curr_val;
            update_pv = 1 as i32;
            if level == 0 as i32 { best_end_root_move = move_0 }
        } else {
            curr_alpha = if best > curr_alpha { best } else { curr_alpha };
            curr_val =
                -end_tree_search(level + 1 as i32, level + exp_depth,
                                 new_opp_bits, new_my_bits,
                                 0 as i32 + 2 as i32 -
                                     side_to_move,
                                 -(curr_alpha + 1 as i32),
                                 -curr_alpha, selectivity,
                                 &mut child_selective_cutoff,
                                 1 as i32);
            if curr_val > curr_alpha && curr_val < beta {
                if selectivity > 0 as i32 {
                    curr_val =
                        -end_tree_search(level + 1 as i32,
                                         level + exp_depth, new_opp_bits,
                                         new_my_bits,
                                         0 as i32 + 2 as i32 -
                                             side_to_move, -beta,
                                         12345678 as i32, selectivity,
                                         &mut child_selective_cutoff,
                                         1 as i32)
                } else {
                    curr_val =
                        -end_tree_search(level + 1 as i32,
                                         level + exp_depth, new_opp_bits,
                                         new_my_bits,
                                         0 as i32 + 2 as i32 -
                                             side_to_move, -beta, -curr_val,
                                         selectivity,
                                         &mut child_selective_cutoff,
                                         1 as i32)
                }
                if curr_val > best {
                    best = curr_val;
                    update_pv = 1 as i32;
                    if level == 0 as i32 && is_panic_abort() == 0 &&
                           force_return == 0 {
                        best_end_root_move = move_0
                    }
                }
            } else if curr_val > best {
                best = curr_val;
                update_pv = 1 as i32;
                if level == 0 as i32 && is_panic_abort() == 0 &&
                       force_return == 0 {
                    best_end_root_move = move_0
                }
            }
        }
        if best >= beta {
            /* The other children don't matter in this case. */
            *selective_cutoff = child_selective_cutoff
        } else if child_selective_cutoff != 0 {
            *selective_cutoff = 1 as i32
        }
        unmake_move(side_to_move, move_0);
        if is_panic_abort() != 0 || force_return != 0 {
            return -(27000 as i32)
        }
        if level == 0 as i32 && get_ponder_move() == 0 {
            /* Output some stats */
            if update_pv != 0 {
                if curr_val <= alpha {
                    send_sweep(b"<%d\x00" as *const u8 as *const i8,
                               curr_val + 1 as i32);
                } else if curr_val >= beta {
                    send_sweep(b">%d\x00" as *const u8 as *const i8,
                               curr_val - 1 as i32);
                } else {
                    send_sweep(b"=%d\x00" as *const u8 as *const i8,
                               curr_val);
                    true_found = 1 as i32;
                    true_val = curr_val
                }
            }
            send_sweep(b" \x00" as *const u8 as *const i8);
            if update_pv != 0 && move_index > 0 as i32 && echo != 0 {
                display_sweep(stdout);
            }
        }
        if update_pv != 0 {
            update_best_list(best_list.as_mut_ptr(), move_0, best_list_index,
                             &mut best_list_length,
                             (level == 0 as i32) as i32);
            pv[level as usize][level as usize] = move_0;
            pv_depth[level as usize] =
                pv_depth[(level + 1 as i32) as usize];
            i = level + 1 as i32;
            while i < pv_depth[(level + 1 as i32) as usize] {
                pv[level as usize][i as usize] =
                    pv[(level + 1 as i32) as usize][i as usize];
                i += 1
            }
        }
        if best >= beta {
            /* Fail high */
            if use_hash != 0 {
                add_hash_extended(1 as i32, best,
                                  best_list.as_mut_ptr(),
                                  16 as i32 | 1 as i32,
                                  remains,
                                  if *selective_cutoff != 0 {
                                      selectivity
                                  } else { 0 as i32 });
            }
            return best
        }
        if best_list_index >= best_list_length && update_pv == 0 &&
               best_list_length < 4 as i32 {
            let fresh1 = best_list_length;
            best_list_length = best_list_length + 1;
            best_list[fresh1 as usize] = move_0
        }
        first = 0 as i32;
        move_index += 1;
        best_list_index += 1
    }
    if first == 0 {
        if use_hash != 0 {
            let mut flags = 16 as i32;
            if best > alpha {
                flags |= 4 as i32
            } else { flags |= 2 as i32 }
            add_hash_extended(1 as i32, best, best_list.as_mut_ptr(),
                              flags, remains,
                              if *selective_cutoff != 0 {
                                  selectivity
                              } else { 0 as i32 });
        }
        return best
    } else if void_legal != 0 {
        if use_hash != 0 {
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2
        }
        curr_val =
            -end_tree_search(level, max_depth, opp_bits, my_bits,
                             0 as i32 + 2 as i32 -
                                 side_to_move, -beta, -alpha, selectivity,
                             selective_cutoff, 0 as i32);
        if use_hash != 0 {
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2
        }
        return curr_val
    } else {
        pv_depth[level as usize] = level;
        my_discs = piece_count[side_to_move as usize][disks_played as usize];
        opp_discs =
            piece_count[(0 as i32 + 2 as i32 - side_to_move)
                            as usize][disks_played as usize];
        disk_diff = my_discs - opp_discs;
        if my_discs > opp_discs {
            return 64 as i32 - 2 as i32 * opp_discs
        } else if my_discs == opp_discs {
            return 0 as i32
        } else { return -(64 as i32 - 2 as i32 * my_discs) }
    };
}
/*
  END_TREE_WRAPPER
  Wrapper onto END_TREE_SEARCH which applies the knowledge that
  the range of valid scores is [-64,+64].  Komi, if any, is accounted for.
*/
unsafe fn end_tree_wrapper(mut level: i32,
                                      mut max_depth: i32,
                                      mut side_to_move: i32,
                                      mut alpha: i32,
                                      mut beta: i32,
                                      mut selectivity: i32,
                                      mut void_legal: i32)
 -> i32 {
    let mut selective_cutoff: i32 = 0;
    let mut my_bits = BitBoard{high: 0, low: 0,};
    let mut opp_bits = BitBoard{high: 0, low: 0,};
    init_mmx();
    set_bitboards(board.as_mut_ptr(), side_to_move, &mut my_bits,
                  &mut opp_bits);
    return end_tree_search(level, max_depth, my_bits, opp_bits, side_to_move,
                           (if alpha - komi_shift > -(64 as i32) {
                                (alpha) - komi_shift
                            } else { -(64 as i32) }),
                           (if beta - komi_shift < 64 as i32 {
                                (beta) - komi_shift
                            } else { 64 as i32 }), selectivity,
                           &mut selective_cutoff, void_legal) + komi_shift;
}
/*
   FULL_EXPAND_PV
   Pad the PV with optimal moves in the low-level phase.
*/
unsafe fn full_expand_pv(mut side_to_move: i32,
                                    mut selectivity: i32) {
    let mut i: i32 = 0;
    let mut pass_count: i32 = 0;
    let mut new_pv_depth: i32 = 0;
    let mut new_pv: [i32; 61] = [0; 61];
    let mut new_side_to_move: [i32; 61] = [0; 61];
    new_pv_depth = 0 as i32;
    pass_count = 0 as i32;
    while pass_count < 2 as i32 {
        let mut move_0: i32 = 0;
        generate_all(side_to_move);
        if move_count[disks_played as usize] > 0 as i32 {
            let mut empties =
                64 as i32 - disc_count(0 as i32) -
                    disc_count(2 as i32);
            end_tree_wrapper(new_pv_depth, empties, side_to_move,
                             -(64 as i32), 64 as i32,
                             selectivity, 1 as i32);
            move_0 = pv[new_pv_depth as usize][new_pv_depth as usize];
            new_pv[new_pv_depth as usize] = move_0;
            new_side_to_move[new_pv_depth as usize] = side_to_move;
            make_move(side_to_move, move_0, 1 as i32);
            new_pv_depth += 1
        } else {
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2;
            pass_count += 1
        }
        side_to_move = 0 as i32 + 2 as i32 - side_to_move
    }
    i = new_pv_depth - 1 as i32;
    while i >= 0 as i32 {
        unmake_move(new_side_to_move[i as usize], new_pv[i as usize]);
        i -= 1
    }
    i = 0 as i32;
    while i < new_pv_depth {
        pv[0 as i32 as usize][i as usize] = new_pv[i as usize];
        i += 1
    }
    pv_depth[0 as i32 as usize] = new_pv_depth;
}
/*
  SEND_SOLVE_STATUS
  Displays endgame results - partial or full.
*/
unsafe fn send_solve_status(mut empties: i32,
                                       mut side_to_move: i32,
                                       mut eval_info: *mut EvaluationType) {
    let mut eval_str = 0 as *mut i8;
    let mut node_val: f64 = 0.;
    set_current_eval(*eval_info);
    clear_status();
    send_status(b"-->  %2d  \x00" as *const u8 as *const i8,
                empties);
    eval_str = produce_eval_text(*eval_info, 1 as i32);
    send_status(b"%-10s  \x00" as *const u8 as *const i8, eval_str);
    free(eval_str as *mut libc::c_void);
    node_val = counter_value(&mut nodes);
    send_status_nodes(node_val);
    if get_ponder_move() != 0 {
        send_status(b"{%c%c} \x00" as *const u8 as *const i8,
                    'a' as i32 + get_ponder_move() % 10 as i32 -
                        1 as i32,
                    '0' as i32 + get_ponder_move() / 10 as i32);
    }
    send_status_pv(pv[0 as i32 as usize].as_mut_ptr(), empties);
    send_status_time(get_elapsed_time());
    if get_elapsed_time() > 0.0001f64 {
        send_status(b"%6.0f %s  \x00" as *const u8 as *const i8,
                    node_val / (get_elapsed_time() + 0.0001f64),
                    b"nps\x00" as *const u8 as *const i8);
    };
}
/*
  END_GAME
  Provides an interface to the fast endgame solver.
*/

pub unsafe fn end_game(mut side_to_move: i32,
                                  mut wld: i32,
                                  mut force_echo: i32,
                                  mut allow_book: i32,
                                  mut komi: i32,
                                  mut eval_info: *mut EvaluationType)
 -> i32 {
    let mut current_confidence: f64 = 0.;
    let mut solve_status = WIN;
    let mut book_move: i32 = 0;
    let mut empties: i32 = 0;
    let mut selectivity: i32 = 0;
    let mut alpha: i32 = 0;
    let mut beta: i32 = 0;
    let mut any_search_result: i32 = 0;
    let mut exact_score_failed: i32 = 0;
    let mut incomplete_search: i32 = 0;
    let mut long_selective_search: i32 = 0;
    let mut old_depth: i32 = 0;
    let mut old_eval: i32 = 0;
    let mut last_window_center: i32 = 0;
    let mut old_pv: [i32; 64] = [0; 64];
    let mut book_eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    empties =
        64 as i32 - disc_count(0 as i32) -
            disc_count(2 as i32);
    /* In komi games, the WLD window is adjusted. */
    if side_to_move == 0 as i32 {
        komi_shift = komi
    } else { komi_shift = -komi }
    /* Check if the position is solved (WLD or exact) in the book. */
    book_move = -(1 as i32);
    if allow_book != 0 {
        /* Is the exact score known? */
        fill_move_alternatives(side_to_move, 16 as i32);
        book_move = get_book_move(side_to_move, 0 as i32, eval_info);
        if book_move != -(1 as i32) {
            root_eval = (*eval_info).score / 128 as i32;
            hash_expand_pv(side_to_move, 1 as i32, 4 as i32,
                           0 as i32);
            send_solve_status(empties, side_to_move, eval_info);
            return book_move
        }
        /* Is the WLD status known? */
        fill_move_alternatives(side_to_move, 4 as i32);
        if komi_shift == 0 as i32 {
            book_move =
                get_book_move(side_to_move, 0 as i32, eval_info);
            if book_move != -(1 as i32) {
                if wld != 0 {
                    root_eval = (*eval_info).score / 128 as i32;
                    hash_expand_pv(side_to_move, 1 as i32,
                                   4 as i32 | 2 as i32 |
                                       1 as i32, 0 as i32);
                    send_solve_status(empties, side_to_move, eval_info);
                    return book_move
                } else { book_eval_info = *eval_info }
            }
        }
        fill_endgame_hash(8 as i32 + 1 as i32,
                          0 as i32);
    }
    last_panic_check = 0.0f64;
    solve_status = UNKNOWN;
    old_eval = 0 as i32;
    /* Prepare for the shallow searches using the midgame eval */
    piece_count[0 as i32 as usize][disks_played as usize] =
        disc_count(0 as i32);
    piece_count[2 as i32 as usize][disks_played as usize] =
        disc_count(2 as i32);
    if empties > 32 as i32 {
        set_panic_threshold(0.20f64);
    } else if empties < 22 as i32 {
        set_panic_threshold(0.50f64);
    } else {
        set_panic_threshold(0.50f64 -
                                (empties - 22 as i32) as
                                    f64 * 0.03f64);
    }
    reset_buffer_display();
    /* Make sure the pre-searches don't mess up the hash table */
    toggle_midgame_hash_usage(1 as i32, 0 as i32);
    incomplete_search = 0 as i32;
    any_search_result = 0 as i32;
    /* Start off by selective endgame search */
    last_window_center = 0 as i32;
    if empties > 18 as i32 {
        if wld != 0 {
            selectivity = 9 as i32;
            while selectivity > 0 as i32 && is_panic_abort() == 0 &&
                      force_return == 0 {
                let mut flags: u32 = 0;
                let mut res = WON_POSITION;
                alpha = -(1 as i32);
                beta = 1 as i32;
                root_eval =
                    end_tree_wrapper(0 as i32, empties, side_to_move,
                                     alpha, beta, selectivity,
                                     1 as i32);
                adjust_counter(&mut nodes);
                if is_panic_abort() != 0 || force_return != 0 { break ; }
                any_search_result = 1 as i32;
                old_eval = root_eval;
                store_pv(old_pv.as_mut_ptr(), &mut old_depth);
                current_confidence = confidence[selectivity as usize];
                flags = 4 as i32 as u32;
                if root_eval == 0 as i32 {
                    res = DRAWN_POSITION
                } else {
                    flags |=
                        (2 as i32 | 1 as i32) as u32;
                    if root_eval > 0 as i32 {
                        res = WON_POSITION
                    } else { res = LOST_POSITION }
                }
                *eval_info =
                    create_eval_info(SELECTIVE_EVAL, res,
                                     root_eval * 128 as i32,
                                     current_confidence, empties,
                                     0 as i32);
                if full_output_mode != 0 {
                    hash_expand_pv(side_to_move, 1 as i32,
                                   flags as i32, selectivity);
                    send_solve_status(empties, side_to_move, eval_info);
                }
                selectivity -= 1
            }
        } else {
            selectivity = 9 as i32;
            while selectivity > 0 as i32 && is_panic_abort() == 0 &&
                      force_return == 0 {
                alpha = last_window_center - 1 as i32;
                beta = last_window_center + 1 as i32;
                root_eval =
                    end_tree_wrapper(0 as i32, empties, side_to_move,
                                     alpha, beta, selectivity,
                                     1 as i32);
                if root_eval <= alpha {
                    loop  {
                        last_window_center -= 2 as i32;
                        alpha = last_window_center - 1 as i32;
                        beta = last_window_center + 1 as i32;
                        if is_panic_abort() != 0 || force_return != 0 {
                            break ;
                        }
                        root_eval =
                            end_tree_wrapper(0 as i32, empties,
                                             side_to_move, alpha, beta,
                                             selectivity, 1 as i32);
                        if !(root_eval <= alpha) { break ; }
                    }
                    root_eval = last_window_center
                } else if root_eval >= beta {
                    loop  {
                        last_window_center += 2 as i32;
                        alpha = last_window_center - 1 as i32;
                        beta = last_window_center + 1 as i32;
                        if is_panic_abort() != 0 || force_return != 0 {
                            break ;
                        }
                        root_eval =
                            end_tree_wrapper(0 as i32, empties,
                                             side_to_move, alpha, beta,
                                             selectivity, 1 as i32);
                        if !(root_eval >= beta) { break ; }
                    }
                    root_eval = last_window_center
                }
                adjust_counter(&mut nodes);
                if is_panic_abort() != 0 || force_return != 0 { break ; }
                last_window_center = root_eval;
                if is_panic_abort() == 0 && force_return == 0 {
                    any_search_result = 1 as i32;
                    old_eval = root_eval;
                    store_pv(old_pv.as_mut_ptr(), &mut old_depth);
                    current_confidence = confidence[selectivity as usize];
                    *eval_info =
                        create_eval_info(SELECTIVE_EVAL, UNSOLVED_POSITION,
                                         root_eval * 128 as i32,
                                         current_confidence, empties,
                                         0 as i32);
                    if full_output_mode != 0 {
                        hash_expand_pv(side_to_move, 1 as i32,
                                       4 as i32, selectivity);
                        send_solve_status(empties, side_to_move, eval_info);
                    }
                }
                selectivity -= 1
            }
        }
    } else { selectivity = 0 as i32 }
    /* Check if the selective search took more than 40% of the allocated
         time. If this is the case, there is no point attempting WLD. */
    long_selective_search = check_threshold(0.35f64);
    /* Make sure the panic abort flag is set properly; it must match
       the status of long_selective_search. This is not automatic as
       it is not guaranteed that any selective search was performed. */
    check_panic_abort();
    if is_panic_abort() != 0 || force_return != 0 ||
           long_selective_search != 0 {
        /* Don't try non-selective solve. */
        if any_search_result != 0 {
            if echo != 0 && (is_panic_abort() != 0 || force_return != 0) {
                printf(b"%s %.1f %c %s\n\x00" as *const u8 as
                           *const i8,
                       b"Semi-panic abort after\x00" as *const u8 as
                           *const i8, get_elapsed_time(),
                       's' as i32,
                       b"in selective search\x00" as *const u8 as
                           *const i8);
                if full_output_mode != 0 {
                    let mut flags_0: u32 = 0;
                    flags_0 = 4 as i32 as u32;
                    if solve_status as u32 !=
                           DRAW as i32 as u32 {
                        flags_0 |=
                            (2 as i32 | 1 as i32) as
                                u32
                    }
                    hash_expand_pv(side_to_move, 1 as i32,
                                   flags_0 as i32, selectivity);
                    send_solve_status(empties, side_to_move, eval_info);
                }
            }
            pv[0 as i32 as usize][0 as i32 as usize] =
                best_end_root_move;
            pv_depth[0 as i32 as usize] = 1 as i32;
            root_eval = old_eval;
            clear_panic_abort();
        } else {
            if echo != 0 {
                printf(b"%s %.1f %c %s\n\x00" as *const u8 as
                           *const i8,
                       b"Panic abort after\x00" as *const u8 as
                           *const i8, get_elapsed_time(),
                       's' as i32,
                       b"in selective search\x00" as *const u8 as
                           *const i8);
            }
            root_eval = -(27000 as i32)
        }
        if echo != 0 || force_echo != 0 {
            end_display_zero_status();
        }
        if book_move != -(1 as i32) &&
               (book_eval_info.res as u32 ==
                    WON_POSITION as i32 as u32 ||
                    book_eval_info.res as u32 ==
                        DRAWN_POSITION as i32 as u32) {
            /* If there is a known win (or mismarked draw) available,
             always play it upon timeout. */
            *eval_info = book_eval_info;
            root_eval = (*eval_info).score / 128 as i32;
            return book_move
        } else {
            return pv[0 as i32 as usize][0 as i32 as usize]
        }
    }
    /* Start non-selective solve */
    if wld != 0 {
        alpha = -(1 as i32);
        beta = 1 as i32
    } else {
        alpha = last_window_center - 1 as i32;
        beta = last_window_center + 1 as i32
    }
    root_eval =
        end_tree_wrapper(0 as i32, empties, side_to_move, alpha, beta,
                         0 as i32, 1 as i32);
    adjust_counter(&mut nodes);
    if is_panic_abort() == 0 && force_return == 0 {
        if wld == 0 {
            if root_eval <= alpha {
                let mut ceiling_value = last_window_center - 2 as i32;
                loop  {
                    alpha = ceiling_value - 1 as i32;
                    beta = ceiling_value;
                    root_eval =
                        end_tree_wrapper(0 as i32, empties,
                                         side_to_move, alpha, beta,
                                         0 as i32, 1 as i32);
                    if is_panic_abort() != 0 || force_return != 0 { break ; }
                    if root_eval > alpha { break ; }
                    ceiling_value -= 2 as i32
                }
            } else if root_eval >= beta {
                let mut floor_value = last_window_center + 2 as i32;
                loop  {
                    alpha = floor_value - 1 as i32;
                    beta = floor_value + 1 as i32;
                    root_eval =
                        end_tree_wrapper(0 as i32, empties,
                                         side_to_move, alpha, beta,
                                         0 as i32, 1 as i32);
                    if is_panic_abort() != 0 || force_return != 0 { break ; }
                    assert!( root_eval > alpha );
                    if root_eval < beta { break ; }
                    floor_value += 2 as i32
                }
            }
        }
        if is_panic_abort() == 0 && force_return == 0 {
            let mut res_0 = WON_POSITION;
            if root_eval < 0 as i32 {
                res_0 = LOST_POSITION
            } else if root_eval == 0 as i32 {
                res_0 = DRAWN_POSITION
            } else { res_0 = WON_POSITION }
            if wld != 0 {
                let mut flags_1: u32 = 0;
                if root_eval == 0 as i32 {
                    flags_1 = 4 as i32 as u32
                } else {
                    flags_1 =
                        (2 as i32 | 1 as i32) as u32
                }
                *eval_info =
                    create_eval_info(WLD_EVAL, res_0,
                                     root_eval * 128 as i32, 0.0f64,
                                     empties, 0 as i32);
                if full_output_mode != 0 {
                    hash_expand_pv(side_to_move, 1 as i32,
                                   flags_1 as i32, 0 as i32);
                    send_solve_status(empties, side_to_move, eval_info);
                }
            } else {
                *eval_info =
                    create_eval_info(EXACT_EVAL, res_0,
                                     root_eval * 128 as i32, 0.0f64,
                                     empties, 0 as i32);
                if full_output_mode != 0 {
                    hash_expand_pv(side_to_move, 1 as i32,
                                   4 as i32, 0 as i32);
                    send_solve_status(empties, side_to_move, eval_info);
                }
            }
        }
    }
    adjust_counter(&mut nodes);
    /* Check for abort. */
    if is_panic_abort() != 0 || force_return != 0 {
        if any_search_result != 0 {
            if echo != 0 {
                printf(b"%s %.1f %c %s\n\x00" as *const u8 as
                           *const i8,
                       b"Semi-panic abort after\x00" as *const u8 as
                           *const i8, get_elapsed_time(),
                       's' as i32,
                       b"in WLD search\x00" as *const u8 as
                           *const i8);
                if full_output_mode != 0 {
                    let mut flags_2: u32 = 0;
                    flags_2 = 4 as i32 as u32;
                    if root_eval != 0 as i32 {
                        flags_2 |=
                            (2 as i32 | 1 as i32) as
                                u32
                    }
                    hash_expand_pv(side_to_move, 1 as i32,
                                   flags_2 as i32, 0 as i32);
                    send_solve_status(empties, side_to_move, eval_info);
                }
                if echo != 0 || force_echo != 0 {
                    end_display_zero_status();
                }
            }
            restore_pv(old_pv.as_mut_ptr(), old_depth);
            root_eval = old_eval;
            clear_panic_abort();
        } else {
            if echo != 0 {
                printf(b"%s %.1f %c %s\n\x00" as *const u8 as
                           *const i8,
                       b"Panic abort after\x00" as *const u8 as
                           *const i8, get_elapsed_time(),
                       's' as i32,
                       b"in WLD search\x00" as *const u8 as
                           *const i8);
            }
            root_eval = -(27000 as i32)
        }
        return pv[0 as i32 as usize][0 as i32 as usize]
    }
    /* Update solve info. */
    store_pv(old_pv.as_mut_ptr(), &mut old_depth);
    old_eval = root_eval;
    if is_panic_abort() == 0 && force_return == 0 &&
           empties > earliest_wld_solve {
        earliest_wld_solve = empties
    }
    /* Check for aborted search. */
    exact_score_failed = 0 as i32;
    if incomplete_search != 0 {
        if echo != 0 {
            printf(b"%s %.1f %c %s\n\x00" as *const u8 as *const i8,
                   b"Semi-panic abort after\x00" as *const u8 as
                       *const i8, get_elapsed_time(), 's' as i32,
                   b"in exact search\x00" as *const u8 as
                       *const i8);
            if full_output_mode != 0 {
                hash_expand_pv(side_to_move, 1 as i32,
                               4 as i32, 0 as i32);
                send_solve_status(empties, side_to_move, eval_info);
            }
            if echo != 0 || force_echo != 0 {
                end_display_zero_status();
            }
        }
        pv[0 as i32 as usize][0 as i32 as usize] =
            best_end_root_move;
        pv_depth[0 as i32 as usize] = 1 as i32;
        root_eval = old_eval;
        exact_score_failed = 1 as i32;
        clear_panic_abort();
    }
    if abs(root_eval) % 2 as i32 == 1 as i32 {
        if root_eval > 0 as i32 {
            root_eval += 1
        } else { root_eval -= 1 }
    }
    if exact_score_failed == 0 && wld == 0 && empties > earliest_full_solve {
        earliest_full_solve = empties
    }
    if wld == 0 && exact_score_failed == 0 {
        (*eval_info).type_0 = EXACT_EVAL;
        (*eval_info).score = root_eval * 128 as i32
    }
    if wld == 0 && exact_score_failed == 0 {
        hash_expand_pv(side_to_move, 1 as i32, 4 as i32,
                       0 as i32);
        send_solve_status(empties, side_to_move, eval_info);
    }
    if echo != 0 || force_echo != 0 {
        end_display_zero_status();
    }
    /* For shallow endgames, we can afford to compute the entire PV
       move by move. */
    if wld == 0 && incomplete_search == 0 && force_return == 0 &&
           empties <= 16 as i32 {
        full_expand_pv(side_to_move, 0 as i32);
    }
    return pv[0 as i32 as usize][0 as i32 as usize];
}

#[no_mangle]
pub unsafe extern "C" fn end_display_zero_status() {
    display_status(stdout, 0 as i32);
}
