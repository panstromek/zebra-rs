use crate::{
    src::{
        search::{root_eval, force_return, hash_expand_pv, get_ponder_move, nodes, create_eval_info, inherit_move_lists, disc_count, evaluations, evals, sorted_move_order, reorder_move_list},
        timer::{frozen_ponder_depth, extended_above_recommended, above_recommended, is_panic_abort, get_elapsed_time, last_panic_check, check_panic_abort},
        display::{send_status, send_status_time, send_status_pv, send_status_nodes, produce_eval_text, clear_status, display_sweep, echo, send_sweep, clear_sweep, display_buffers},
        counter::{counter_value, adjust_counter},
        stubs::{free, sprintf, abs, stdout},
        libc,
        moves::{valid_move, disks_played, unmake_move, make_move, move_list, move_count, generate_all, unmake_move_no_hash, make_move_no_hash},
        hash::{find_hash, HashEntry, hash_flip_color2, hash2, hash_flip_color1, hash1, add_hash_extended, add_hash},
        globals::{piece_count, board, pv, pv_depth},
        getcoeff::pattern_evaluation,
        eval::terminal_evaluation,
        probcut::mpc_cut,
        myrandom::my_random,
        zebra::{EvaluationType}
    }
};
pub use engine::src::midgame::*;

/*
   ROOT_TREE_SEARCH
   The recursive tree search function that is to be called only
   for the root of the search tree.
*/

pub unsafe fn root_tree_search(mut level: i32,
                                          mut max_depth: i32,
                                          mut side_to_move: i32,
                                          mut alpha: i32,
                                          mut beta: i32,
                                          mut allow_hash: i32,
                                          mut allow_mpc: i32,
                                          mut void_legal: i32)
 -> i32 {
    let mut buffer: [i8; 32] = [0; 32];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut curr_val: i32 = 0;
    let mut best: i32 = 0;
    let mut pre_best: i32 = 0;
    let mut searched: i32 = 0;
    let mut move_0: i32 = 0;
    let mut move_index: i32 = 0;
    let mut best_move_index: i32 = 0;
    let mut hash_hit: i32 = 0;
    let mut pre_depth: i32 = 0;
    let mut update_pv: i32 = 0;
    let mut remains: i32 = 0;
    let mut use_hash: i32 = 0;
    let mut pre_search_done: i32 = 0;
    let mut curr_alpha: i32 = 0;
    let mut best_index: i32 = 0;
    let mut best_score: i32 = 0;
    let mut best_list_index: i32 = 0;
    let mut best_list_length: i32 = 0;
    let mut selectivity: i32 = 0;
    let mut offset: i32 = 0;
    let mut best_list: [i32; 4] = [0; 4];
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    remains = max_depth - level;
    nodes.lo = nodes.lo.wrapping_add(1);
    use_hash =
        (remains >= 2 as i32 && 1 as i32 != 0 &&
             allow_hash != 0) as i32;
    if 1 as i32 != 0 && allow_mpc != 0 {
        selectivity = 1 as i32
    } else { selectivity = 0 as i32 }
    /* Hash strategy at the root: Only use hash table information for
       move ordering purposes.  This guarantees that score perturbation
       is applied for all moves. */
    hash_hit = 0 as i32;
    if use_hash != 0 && allow_midgame_hash_probe != 0 {
        find_hash(&mut entry, 0 as i32);
        if entry.draft as i32 != 0 as i32 {
            hash_hit = 1 as i32
        }
    }
    pre_search_done = 0 as i32;
    if get_ponder_move() == 0 {
        if alpha <= -(29000 as i32) && beta >= 29000 as i32 {
            sprintf(buffer.as_mut_ptr(),
                    b"[-inf,inf]:\x00" as *const u8 as *const i8);
        } else if alpha <= -(29000 as i32) &&
                      beta < 29000 as i32 {
            sprintf(buffer.as_mut_ptr(),
                    b"[-inf,%.1f]:\x00" as *const u8 as *const i8,
                    beta as f64 / 128.0f64);
        } else if alpha > -(29000 as i32) &&
                      beta >= 29000 as i32 {
            sprintf(buffer.as_mut_ptr(),
                    b"[%.1f,inf]:\x00" as *const u8 as *const i8,
                    alpha as f64 / 128.0f64);
        } else {
            sprintf(buffer.as_mut_ptr(),
                    b"[%.1f,%.1f]:\x00" as *const u8 as *const i8,
                    alpha as f64 / 128.0f64,
                    beta as f64 / 128.0f64);
        }
        clear_sweep();
        send_sweep(b"%-14s \x00" as *const u8 as *const i8,
                   buffer.as_mut_ptr());
    }
    /* Full negascout search */
    searched = 0 as i32;
    best = -(12345678 as i32);
    best_move_index = -(1 as i32);
    curr_alpha = alpha;
    best_list_length = 0 as i32;
    i = 0 as i32;
    while i < 4 as i32 {
        best_list[i as usize] = 0 as i32;
        i += 1
    }
    if pre_search_done == 0 {
        move_count[disks_played as usize] = 0 as i32;
        if hash_hit != 0 {
            i = 0 as i32;
            while i < 4 as i32 {
                if valid_move(entry.move_0[i as usize], side_to_move) != 0 {
                    let fresh1 = best_list_length;
                    best_list_length = best_list_length + 1;
                    best_list[fresh1 as usize] = entry.move_0[i as usize]
                }
                i += 1
            }
        }
    }
    i = 0 as i32;
    best_list_index = 0 as i32;
    loop 
         /* Try the hash table move(s) first if feasible */
         {
        if pre_search_done == 0 && best_list_index < best_list_length {
            move_count[disks_played as usize] += 1;
            move_index = 0 as i32;
            while sorted_move_order[disks_played as
                                        usize][move_index as usize] !=
                      best_list[best_list_index as usize] {
                move_index += 1
            }
        } else {
            /* Otherwise use information from shallow searches */
            if pre_search_done == 0 {
                if remains < 10 as i32 {
                    pre_depth = 1 as i32
                } else { pre_depth = 2 as i32 }
                pre_best = -(12345678 as i32);
                move_index = 0 as i32;
                while move_index < 60 as i32 {
                    let mut already_checked: i32 = 0;
                    move_0 =
                        sorted_move_order[disks_played as
                                              usize][move_index as usize];
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
                           make_move(side_to_move, move_0, 1 as i32)
                               != 0 as i32 {
                        curr_val =
                            -tree_search(level + 1 as i32,
                                         level + pre_depth,
                                         0 as i32 + 2 as i32 -
                                             side_to_move,
                                         -(12345678 as i32),
                                         -pre_best, 0 as i32,
                                         0 as i32, 1 as i32);
                        pre_best =
                            if pre_best > curr_val {
                                pre_best
                            } else { curr_val };
                        unmake_move(side_to_move, move_0);
                        evals[disks_played as usize][move_0 as usize] =
                            curr_val;
                        feas_index_list[disks_played as
                                            usize][move_count[disks_played as
                                                                  usize] as
                                                       usize] = move_index;
                        move_count[disks_played as usize] += 1
                    }
                    move_index += 1
                }
                pre_search_done = 1 as i32
            }
            if i == move_count[disks_played as usize] { break ; }
            best_index = i;
            best_score =
                evals[disks_played as
                          usize][sorted_move_order[disks_played as
                                                       usize][feas_index_list[disks_played
                                                                                  as
                                                                                  usize][i
                                                                                             as
                                                                                             usize]
                                                                  as usize] as
                                     usize];
            j = i + 1 as i32;
            while j < move_count[disks_played as usize] {
                let mut cand_move: i32 = 0;
                cand_move =
                    sorted_move_order[disks_played as
                                          usize][feas_index_list[disks_played
                                                                     as
                                                                     usize][j
                                                                                as
                                                                                usize]
                                                     as usize];
                if evals[disks_played as usize][cand_move as usize] >
                       best_score {
                    best_score =
                        evals[disks_played as usize][cand_move as usize];
                    best_index = j
                }
                j += 1
            }
            move_index =
                feas_index_list[disks_played as usize][best_index as usize];
            feas_index_list[disks_played as usize][best_index as usize] =
                feas_index_list[disks_played as usize][i as usize]
        }
        move_0 =
            sorted_move_order[disks_played as usize][move_index as usize];
        if get_ponder_move() == 0 {
            send_sweep(b"%c%c\x00" as *const u8 as *const i8,
                       'a' as i32 + move_0 % 10 as i32 -
                           1 as i32,
                       '0' as i32 + move_0 / 10 as i32);
        }
        make_move(side_to_move, move_0, 1 as i32);
        update_pv = 0 as i32;
        offset = score_perturbation[move_0 as usize];
        if searched == 0 as i32 {
            curr_val =
                perturb_score(-tree_search(level + 1 as i32,
                                           max_depth,
                                           0 as i32 + 2 as i32
                                               - side_to_move,
                                           -(beta - offset),
                                           -(curr_alpha - offset), allow_hash,
                                           allow_mpc, 1 as i32),
                              offset);
            best = curr_val;
            best_move_index = move_index;
            update_pv = 1 as i32;
            best_mid_root_move = move_0
        } else {
            curr_alpha = if best > curr_alpha { best } else { curr_alpha };
            curr_val =
                perturb_score(-tree_search(level + 1 as i32,
                                           max_depth,
                                           0 as i32 + 2 as i32
                                               - side_to_move,
                                           -(curr_alpha - offset +
                                                 1 as i32),
                                           -(curr_alpha - offset), allow_hash,
                                           allow_mpc, 1 as i32),
                              offset);
            if curr_val > curr_alpha && curr_val < beta {
                curr_val =
                    perturb_score(-tree_search(level + 1 as i32,
                                               max_depth,
                                               0 as i32 +
                                                   2 as i32 -
                                                   side_to_move,
                                               -(beta - offset),
                                               12345678 as i32,
                                               allow_hash, allow_mpc,
                                               1 as i32), offset);
                if curr_val > best {
                    best = curr_val;
                    best_move_index = move_index;
                    update_pv = 1 as i32;
                    if is_panic_abort() == 0 && force_return == 0 {
                        best_mid_root_move = move_0
                    }
                }
            } else if curr_val > best {
                best = curr_val;
                best_move_index = move_index;
                update_pv = 1 as i32
            }
        }
        unmake_move(side_to_move, move_0);
        if is_panic_abort() != 0 || force_return != 0 {
            return -(27000 as i32)
        }
        evals[disks_played as usize][move_0 as usize] = curr_val;
        if get_ponder_move() == 0 {
            if update_pv != 0 {
                if curr_val <= alpha {
                    send_sweep(b"<%.2f\x00" as *const u8 as
                                   *const i8,
                               (curr_val + 1 as i32) as f64
                                   / 128.0f64);
                } else if curr_val >= beta {
                    send_sweep(b">%.2f\x00" as *const u8 as
                                   *const i8,
                               (curr_val - 1 as i32) as f64
                                   / 128.0f64);
                } else {
                    send_sweep(b"=%.2f\x00" as *const u8 as
                                   *const i8,
                               curr_val as f64 / 128.0f64);
                }
            }
            send_sweep(b" \x00" as *const u8 as *const i8);
            if update_pv != 0 && searched > 0 as i32 && echo != 0 &&
                   max_depth >= 10 as i32 {
                display_sweep(stdout);
            }
        }
        if update_pv != 0 {
            midgame_c__update_best_list(best_list.as_mut_ptr(), move_0,
                                        best_list_index, best_list_length);
            pv[level as usize][level as usize] = move_0;
            pv_depth[level as usize] =
                pv_depth[(level + 1 as i32) as usize];
            j = level + 1 as i32;
            while j < pv_depth[(level + 1 as i32) as usize] {
                pv[level as usize][j as usize] =
                    pv[(level + 1 as i32) as usize][j as usize];
                j += 1
            }
        }
        if best >= beta {
            advance_move(move_index);
            if use_hash != 0 && allow_midgame_hash_update != 0 {
                add_hash_extended(0 as i32, best,
                                  best_list.as_mut_ptr(),
                                  8 as i32 | 1 as i32,
                                  remains, selectivity);
            }
            return best
        }
        /* For symmetry reasons, the score for any move is the score of the
           position for the initial position. */
        if disks_played == 0 as i32 {
            add_hash_extended(0 as i32, best, best_list.as_mut_ptr(),
                              8 as i32 | 4 as i32, remains,
                              selectivity);
            return best
        }
        searched += 1;
        i += 1;
        best_list_index += 1
    }
    /* Post-processing */
    if move_count[disks_played as usize] > 0 as i32 {
        advance_move(best_move_index);
        if use_hash != 0 && allow_midgame_hash_update != 0 {
            if best > alpha {
                add_hash_extended(0 as i32, best,
                                  best_list.as_mut_ptr(),
                                  8 as i32 | 4 as i32,
                                  remains, selectivity);
            } else {
                add_hash_extended(0 as i32, best,
                                  best_list.as_mut_ptr(),
                                  8 as i32 | 2 as i32,
                                  remains, selectivity);
            }
        }
        return best
    } else if void_legal != 0 {
        /* No feasible moves */
        hash1 ^= hash_flip_color1;
        hash2 ^= hash_flip_color2;
        curr_val =
            -root_tree_search(level, max_depth,
                              0 as i32 + 2 as i32 -
                                  side_to_move, -beta, -alpha, allow_hash,
                              allow_mpc, 0 as i32);
        hash1 ^= hash_flip_color1;
        hash2 ^= hash_flip_color2;
        return curr_val
    } else {
        pv_depth[level as usize] = level;
        return terminal_evaluation(side_to_move)
    };
}
/*
   MIDDLE_GAME
   side_to_move = the side whose turn it is to move
*/

pub unsafe fn middle_game(mut side_to_move: i32,
                                     mut max_depth: i32,
                                     mut update_evals: i32,
                                     mut eval_info: *mut EvaluationType)
 -> i32 {
    let mut eval_str =
        0 as *mut i8; /* Disable I.D. in this function */
    let mut node_val: f64 = 0.;
    let mut val: i32 = 0;
    let mut old_val: i32 = 0;
    let mut adjusted_val: i32 = 0;
    let mut initial_depth: i32 = 0;
    let mut depth: i32 = 0;
    let mut alpha: i32 = 0;
    let mut beta: i32 = 0;
    let mut enable_mpc: i32 = 0;
    let mut base_stage: i32 = 0;
    let mut full_length_line: i32 = 0;
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    last_panic_check = 0.0f64;
    counter_phase = 0 as i32;
    piece_count[0 as i32 as usize][disks_played as usize] =
        disc_count(0 as i32);
    piece_count[2 as i32 as usize][disks_played as usize] =
        disc_count(2 as i32);
    base_stage =
        disc_count(0 as i32) + disc_count(2 as i32) -
            4 as i32;
    val = 0 as i32;
    old_val = --(27000 as i32);
    enable_mpc = (max_depth >= 9 as i32) as i32;
    initial_depth =
        if 1 as i32 > max_depth - 2 as i32 {
            1 as i32
        } else { (max_depth) - 2 as i32 };
    initial_depth = max_depth;
    *eval_info =
        create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION, 0 as i32,
                         0.0f64, 0 as i32, 0 as i32);
    depth = initial_depth;
    while depth <= max_depth {
        alpha = -(12345678 as i32);
        beta = 12345678 as i32;
        inherit_move_lists(disks_played + max_depth);
        /* The actual search */
        if depth == 1 as i32 {
            /* Fix to make it harder to wipe out depth-1 Zebra */
            val = protected_one_ply_search(side_to_move)
        } else if enable_mpc != 0 {
            val =
                root_tree_search(0 as i32, depth, side_to_move, alpha,
                                 beta, 1 as i32, 1 as i32,
                                 1 as i32);
            if force_return == 0 && is_panic_abort() == 0 &&
                   (val <= alpha || val >= beta) {
                val =
                    root_tree_search(0 as i32, depth, side_to_move,
                                     -(12345678 as i32),
                                     12345678 as i32,
                                     1 as i32, 1 as i32,
                                     1 as i32)
            }
        } else {
            val =
                root_tree_search(0 as i32, depth, side_to_move, alpha,
                                 beta, 1 as i32, 0 as i32,
                                 1 as i32);
            if is_panic_abort() == 0 && force_return == 0 {
                if val <= alpha {
                    val =
                        root_tree_search(0 as i32, depth,
                                         side_to_move,
                                         -(29000 as i32), alpha,
                                         1 as i32, 0 as i32,
                                         1 as i32)
                } else if val >= beta {
                    val =
                        root_tree_search(0 as i32, depth,
                                         side_to_move, beta,
                                         29000 as i32,
                                         1 as i32, 0 as i32,
                                         1 as i32)
                }
            }
        }
        /* Adjust scores and PV if search is aborted */
        if is_panic_abort() != 0 || force_return != 0 {
            pv[0 as i32 as usize][0 as i32 as usize] =
                best_mid_root_move;
            pv_depth[0 as i32 as usize] = 1 as i32;
            hash_expand_pv(side_to_move, 0 as i32, 4 as i32,
                           12345678 as i32);
            if base_stage + depth - 2 as i32 >= 0 as i32 &&
                   stage_reached[(base_stage + depth - 2 as i32) as
                                     usize] != 0 {
                val =
                    stage_score[(base_stage + depth - 2 as i32) as
                                    usize];
                if side_to_move == 2 as i32 { val = -val }
            } else { val = old_val }
        }
        /* Check if the search info corresponds to a variation of
           depth exactly DEPTH which would mean that the search
           gives new score information */
        full_length_line = 0 as i32;
        find_hash(&mut entry, 0 as i32);
        if force_return == 0 && is_panic_abort() == 0 &&
               entry.draft as i32 != 0 as i32 &&
               valid_move(entry.move_0[0 as i32 as usize],
                          side_to_move) != 0 &&
               entry.draft as i32 == depth {
            full_length_line = 1 as i32
        }
        /* Update the stored scores */
        if (stage_reached[(base_stage + depth) as usize] == 0 ||
                full_length_line != 0) && update_evals != 0 {
            stage_reached[(base_stage + depth) as usize] = 1 as i32;
            if side_to_move == 0 as i32 {
                stage_score[(base_stage + depth) as usize] = val
            } else { stage_score[(base_stage + depth) as usize] = -val }
        }
        /* Adjust the eval for oscillations odd/even by simply averaging the
           last two stages (if they are available). */
        if stage_reached[(base_stage + depth) as usize] != 0 &&
               stage_reached[(base_stage + depth - 1 as i32) as usize]
                   != 0 && update_evals != 0 {
            if side_to_move == 0 as i32 {
                adjusted_val =
                    (stage_score[(base_stage + depth) as usize] +
                         stage_score[(base_stage + depth - 1 as i32)
                                         as usize]) / 2 as i32
            } else {
                adjusted_val =
                    -(stage_score[(base_stage + depth) as usize] +
                          stage_score[(base_stage + depth - 1 as i32)
                                          as usize]) / 2 as i32
            }
        } else if depth == initial_depth {
            adjusted_val = val
        } else { adjusted_val = (val + old_val) / 2 as i32 }
        /* In case the search reached the end of the game, the score
           must be converted into an endgame score */
        if val >= 29000 as i32 {
            *eval_info =
                create_eval_info(EXACT_EVAL, WON_POSITION,
                                 (val - 29000 as i32) *
                                     128 as i32, 0.0f64, depth,
                                 0 as i32)
        } else if val <= -(29000 as i32) {
            *eval_info =
                create_eval_info(EXACT_EVAL, LOST_POSITION,
                                 (val + 29000 as i32) *
                                     128 as i32, 0.0f64, depth,
                                 0 as i32)
        } else {
            *eval_info =
                create_eval_info(MIDGAME_EVAL, UNSOLVED_POSITION,
                                 adjusted_val, 0.0f64, depth,
                                 0 as i32)
        }
        /* Display and store search info */
        if depth == max_depth {
            clear_status();
            send_status(b"--> \x00" as *const u8 as *const i8);
            if is_panic_abort() != 0 || force_return != 0 {
                send_status(b"*\x00" as *const u8 as *const i8);
            } else {
                send_status(b" \x00" as *const u8 as *const i8);
            }
            send_status(b"%2d  \x00" as *const u8 as *const i8,
                        depth);
            eval_str = produce_eval_text(*eval_info, 1 as i32);
            send_status(b"%-10s  \x00" as *const u8 as *const i8,
                        eval_str);
            free(eval_str as *mut libc::c_void);
            node_val = counter_value(&mut nodes);
            send_status_nodes(node_val);
            if get_ponder_move() != 0 {
                send_status(b"{%c%c} \x00" as *const u8 as
                                *const i8,
                            'a' as i32 + get_ponder_move() % 10 as i32
                                - 1 as i32,
                            '0' as i32 +
                                get_ponder_move() / 10 as i32);
            }
            hash_expand_pv(side_to_move, 0 as i32, 4 as i32,
                           12345678 as i32);
            send_status_pv(pv[0 as i32 as usize].as_mut_ptr(),
                           max_depth);
            send_status_time(get_elapsed_time());
            if get_elapsed_time() != 0.0f64 {
                send_status(b"%6.0f %s\x00" as *const u8 as
                                *const i8,
                            node_val / (get_elapsed_time() + 0.001f64),
                            b"nps\x00" as *const u8 as *const i8);
            }
        }
        if is_panic_abort() != 0 || force_return != 0 { break ; }
        /* Check if search time or adjusted search time are long enough
           for the search to be discontinued */
        old_val = adjusted_val;
        if do_check_midgame_abort != 0 {
            if above_recommended() != 0 ||
                   extended_above_recommended() != 0 &&
                       depth >= frozen_ponder_depth {
                set_midgame_abort();
                break ;
            }
        }
        depth += 1
    }
    root_eval = val;
    return pv[0 as i32 as usize][0 as i32 as usize];
}
