use crate::{
    src::{
        epcstat::{end_sigma, end_mean, end_stats_available},
        libc,
        stubs::{ceil, abs, printf, __assert_fail, free, fflush, sprintf, puts, stdout},
        moves::{dir_mask, disks_played, unmake_move, make_move, move_count, generate_all, move_list, valid_move},
        search::{force_return, hash_expand_pv, root_eval, store_pv, restore_pv, nodes, create_eval_info, disc_count, get_ponder_move, set_current_eval, select_move, evals, sorted_move_order},
        display::{display_status, echo, reset_buffer_display, send_status, send_status_time, send_status_pv, send_status_nodes, produce_eval_text, clear_status, display_sweep, send_sweep, display_buffers, clear_sweep},
        timer::{clear_panic_abort, get_elapsed_time, is_panic_abort, check_panic_abort, check_threshold, set_panic_threshold, last_panic_check},
        counter::{adjust_counter, counter_value},
        midgame::{toggle_midgame_hash_usage, tree_search},
        globals::{piece_count, board, pv_depth, pv},
        osfbook::{fill_endgame_hash, get_book_move, fill_move_alternatives},
        hash::{hash_flip_color2, hash2, hash_flip_color1, hash1, add_hash_extended, find_hash, HashEntry, add_hash, hash_put_value2, hash_put_value1},
        bitboard::{set_bitboards, BitBoard},
        bitbmob::{init_mmx, bitboard_mobility, weighted_mobility},
        bitbtest::{bb_flips, TestFlips_bitboard},
        probcut::{end_mpc_depth, use_end_cut},
        stable::{count_stable, count_edge_stable},
        unflip::UndoFlips,
        doflip::{hash_update2, hash_update1, DoFlips_hash},
        bitbcnt::CountFlips_bitboard,
        zebra::{EvaluationType, _IO_FILE}
    }
};

pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
/*
   File:           globals.h

   Created:        June 30, 1997

   Modified:       January 8, 2000

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Global state variables.
*/
/* The basic board type. One index for each position;
   a1=11, h1=18, a8=81, h8=88. */

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
/* All information available about a move decision. */

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MoveLink {
    pub pred: i32,
    pub succ: i32,
}

pub const DRAW: C2RustUnnamed = 2;
pub type C2RustUnnamed = u32;
pub const UNKNOWN: C2RustUnnamed = 3;
pub const LOSS: C2RustUnnamed = 1;
pub const WIN: C2RustUnnamed = 0;

pub static mut end_move_list: [MoveLink; 100] =
    [MoveLink{pred: 0, succ: 0,}; 100];
/* The parities of the regions are in the region_parity bit vector. */
static mut region_parity: u32 = 0;
/* Pseudo-probabilities corresponding to the percentiles.
   These are taken from the normal distribution; to the percentile
   x corresponds the probability Pr(-x <= Y <= x) where Y is a N(0,1)
   variable. */
static mut confidence: [f64; 10] =
    [1.000f64, 0.99f64, 0.98f64, 0.954f64, 0.911f64, 0.838f64, 0.729f64,
     0.576f64, 0.383f64, 0.197f64];
/* Percentiles used in the endgame MPC */
static mut end_percentile: [f64; 10] =
    [100.0f64, 4.0f64, 3.0f64, 2.0f64, 1.7f64, 1.4f64, 1.1f64, 0.8f64, 0.5f64,
     0.25f64];
static mut stability_threshold: [i32; 19] =
    [65 as i32, 65 as i32, 65 as i32,
     65 as i32, 65 as i32, 46 as i32,
     38 as i32, 30 as i32, 24 as i32,
     24 as i32, 24 as i32, 24 as i32,
     0 as i32, 0 as i32, 0 as i32, 0 as i32,
     0 as i32, 0 as i32, 0 as i32];
static mut fast_first_mean: [[f64; 64]; 61] = [[0.; 64]; 61];
static mut fast_first_sigma: [[f64; 64]; 61] = [[0.; 64]; 61];
static mut best_move: i32 = 0;
static mut best_end_root_move: i32 = 0;
static mut true_found: i32 = 0;
static mut true_val: i32 = 0;
static mut full_output_mode: i32 = 0;
static mut earliest_wld_solve: i32 = 0;
static mut earliest_full_solve: i32 = 0;
static mut fast_first_threshold: [[i32; 64]; 61] = [[0; 64]; 61];
static mut ff_mob_factor: [i32; 61] = [0; 61];
static mut neighborhood_mask: [BitBoard; 100] =
    [BitBoard{high: 0, low: 0,}; 100];

pub static quadrant_mask: [u32; 100] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 2, 2, 2, 2, 0,
    0, 1, 1, 1, 1, 2, 2, 2, 2, 0,
    0, 1, 1, 1, 1, 2, 2, 2, 2, 0,
    0, 1, 1, 1, 1, 2, 2, 2, 2, 0,
    0, 4, 4, 4, 4, 8, 8, 8, 8, 0,
    0, 4, 4, 4, 4, 8, 8, 8, 8, 0,
    0, 4, 4, 4, 4, 8, 8, 8, 8, 0,
    0, 4, 4, 4, 4, 8, 8, 8, 8, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];
/* Number of discs that the side to move at the root has to win with. */
static mut komi_shift: i32 = 0;
/*
  TESTFLIPS_WRAPPER
  Checks if SQ is a valid move by
  (1) verifying that there exists a neighboring opponent disc,
  (2) verifying that the move flips some disc.
*/
unsafe fn TestFlips_wrapper(mut sq: i32,
                                       mut my_bits: BitBoard,
                                       mut opp_bits: BitBoard)
 -> i32 {
    let mut flipped: i32 = 0;
    if neighborhood_mask[sq as usize].high & opp_bits.high |
           neighborhood_mask[sq as usize].low & opp_bits.low !=
           0 as i32 as u32 {
        flipped =
            TestFlips_bitboard[(sq - 11 as i32) as
                                   usize](my_bits.high,
                                                                              my_bits.low,
                                                                              opp_bits.high,
                                                                              opp_bits.low)
    } else { flipped = 0 as i32 }
    return flipped;
}
/*
  PREPARE_TO_SOLVE
  Create the list of empty squares.
*/
unsafe fn prepare_to_solve(mut board_0: *const i32) {
    /* fixed square ordering: */
    /* jcw's order, which is the best of 4 tried (according to Warren Smith) */
    static mut worst2best: [u8; 64] =
        [22 as i32 as u8,
         27 as i32 as u8,
         72 as i32 as u8,
         77 as i32 as u8,
         12 as i32 as u8,
         17 as i32 as u8,
         21 as i32 as u8,
         28 as i32 as u8,
         71 as i32 as u8,
         78 as i32 as u8,
         82 as i32 as u8,
         87 as i32 as u8,
         23 as i32 as u8,
         26 as i32 as u8,
         32 as i32 as u8,
         37 as i32 as u8,
         62 as i32 as u8,
         67 as i32 as u8,
         73 as i32 as u8,
         76 as i32 as u8,
         24 as i32 as u8,
         25 as i32 as u8,
         42 as i32 as u8,
         47 as i32 as u8,
         52 as i32 as u8,
         57 as i32 as u8,
         74 as i32 as u8,
         75 as i32 as u8,
         34 as i32 as u8,
         35 as i32 as u8,
         43 as i32 as u8,
         46 as i32 as u8,
         53 as i32 as u8,
         56 as i32 as u8,
         64 as i32 as u8,
         65 as i32 as u8,
         13 as i32 as u8,
         16 as i32 as u8,
         31 as i32 as u8,
         38 as i32 as u8,
         61 as i32 as u8,
         68 as i32 as u8,
         83 as i32 as u8,
         86 as i32 as u8,
         14 as i32 as u8,
         15 as i32 as u8,
         41 as i32 as u8,
         48 as i32 as u8,
         51 as i32 as u8,
         58 as i32 as u8,
         84 as i32 as u8,
         85 as i32 as u8,
         33 as i32 as u8,
         36 as i32 as u8,
         63 as i32 as u8,
         66 as i32 as u8,
         11 as i32 as u8,
         18 as i32 as u8,
         81 as i32 as u8,
         88 as i32 as u8,
         44 as i32 as u8,
         45 as i32 as u8,
         54 as i32 as u8,
         45 as i32 as u8];
    let mut i: i32 = 0;
    let mut last_sq: i32 = 0;
    region_parity = 0 as i32 as u32;
    last_sq = 0 as i32;
    i = 59 as i32;
    while i >= 0 as i32 {
        let mut sq = worst2best[i as usize] as i32;
        if *board_0.offset(sq as isize) == 1 as i32 {
            end_move_list[last_sq as usize].succ = sq;
            end_move_list[sq as usize].pred = last_sq;
            region_parity ^= quadrant_mask[sq as usize];
            last_sq = sq
        }
        i -= 1
    }
    end_move_list[last_sq as usize].succ = 99 as i32;
}
/*
  SOLVE_TWO_EMPTY
  SOLVE_THREE_EMPTY
  SOLVE_FOUR_EMPTY
  SOLVE_PARITY
  SOLVE_PARITY_HASH
  SOLVE_PARITY_HASH_HIGH
  These are the core routines of the low level endgame code.
  They all perform the same task: Return the score for the side to move.
  Structural differences:
  * SOLVE_TWO_EMPTY may only be called for *exactly* two empty
  * SOLVE_THREE_EMPTY may only be called for *exactly* three empty
  * SOLVE_FOUR_EMPTY may only be called for *exactly* four empty
  * SOLVE_PARITY uses stability, parity and fixed move ordering
  * SOLVE_PARITY_HASH uses stability, hash table and fixed move ordering
  * SOLVE_PARITY_HASH_HIGH uses stability, hash table and (non-thresholded)
    fastest first
*/
unsafe fn solve_two_empty(mut my_bits: BitBoard,
                                     mut opp_bits: BitBoard,
                                     mut sq1: i32,
                                     mut sq2: i32,
                                     mut alpha: i32,
                                     mut beta: i32,
                                     mut disc_diff: i32,
                                     mut pass_legal: i32)
 -> i32 {
    // BitBoard new_opp_bits;
    let mut score = -(12345678 as i32);
    let mut flipped: i32 = 0;
    let mut ev: i32 = 0;
    nodes.lo = nodes.lo.wrapping_add(1);
    /* Overall strategy: Lazy evaluation whenever possible, i.e., don't
       update bitboards until they are used. Also look at alpha and beta
       in order to perform strength reduction: Feasibility testing is
       faster than counting number of flips. */
    /* Try the first of the two empty squares... */
    flipped = TestFlips_wrapper(sq1, my_bits, opp_bits);
    if flipped != 0 as i32 {
        /* SQ1 feasible for me */
        nodes.lo = nodes.lo.wrapping_add(1);
        ev = disc_diff + 2 as i32 * flipped;
        flipped =
            CountFlips_bitboard[(sq2 - 11 as i32) as
                                    usize](opp_bits.high
                                                                                   &
                                                                                   !bb_flips.high,
                                                                               opp_bits.low
                                                                                   &
                                                                                   !bb_flips.low);
        if flipped != 0 as i32 {
            ev -= 2 as i32 * flipped
        } else if ev >= 0 as i32 {
            /* He passes, check if SQ2 is feasible for me */
            /* I'm ahead, so EV will increase by at least 2 */
            ev += 2 as i32;
            if ev < beta {
                /* Only bother if not certain fail-high */
                ev +=
                    2 as i32 *
                        CountFlips_bitboard[(sq2 - 11 as i32) as
                                                usize](bb_flips.high,
                                                                                           bb_flips.low)
            }
        } else if ev < beta {
            /* Only bother if not fail-high already */
            flipped =
                CountFlips_bitboard[(sq2 - 11 as i32) as
                                        usize](bb_flips.high,
                                                                                   bb_flips.low);
            if flipped != 0 as i32 {
                /* ELSE: SQ2 will end up empty, game over */
                /* SQ2 feasible for me, game over */
                ev += 2 as i32 * (flipped + 1 as i32)
            }
        }
        /* Being legal, the first move is the best so far */
        score = ev;
        if score > alpha { if score >= beta { return score } alpha = score }
    }
    /* ...and then the second */
    flipped = TestFlips_wrapper(sq2, my_bits, opp_bits);
    if flipped != 0 as i32 {
        /* SQ2 feasible for me */
        nodes.lo = nodes.lo.wrapping_add(1);
        ev = disc_diff + 2 as i32 * flipped;
        flipped =
            CountFlips_bitboard[(sq1 - 11 as i32) as
                                    usize](opp_bits.high
                                                                                   &
                                                                                   !bb_flips.high,
                                                                               opp_bits.low
                                                                                   &
                                                                                   !bb_flips.low);
        if flipped != 0 as i32 {
            /* SQ1 feasible for him, game over */
            ev -= 2 as i32 * flipped
        } else if ev >= 0 as i32 {
            /* He passes, check if SQ1 is feasible for me */
            /* I'm ahead, so EV will increase by at least 2 */
            ev += 2 as i32;
            if ev < beta {
                /* Only bother if not certain fail-high */
                ev +=
                    2 as i32 *
                        CountFlips_bitboard[(sq1 - 11 as i32) as
                                                usize](bb_flips.high,
                                                                                           bb_flips.low)
            }
        } else if ev < beta {
            /* Only bother if not fail-high already */
            flipped =
                CountFlips_bitboard[(sq1 - 11 as i32) as
                                        usize](bb_flips.high,
                                                                                   bb_flips.low);
            if flipped != 0 as i32 {
                /* ELSE: SQ1 will end up empty, game over */
                /* SQ1 feasible for me, game over */
                ev += 2 as i32 * (flipped + 1 as i32)
            }
        }
        /* If the second move is better than the first (if that move was legal),
           its score is the score of the position */
        if ev >= score { return ev }
    }
    /* If both SQ1 and SQ2 are illegal I have to pass,
       otherwise return the best score. */
    if score == -(12345678 as i32) {
        if pass_legal == 0 {
            /* Two empty squares */
            if disc_diff > 0 as i32 {
                return disc_diff + 2 as i32
            }
            if disc_diff < 0 as i32 {
                return disc_diff - 2 as i32
            }
            return 0 as i32
        } else {
            return -solve_two_empty(opp_bits, my_bits, sq1, sq2, -beta,
                                    -alpha, -disc_diff, 0 as i32)
        }
    } else { return score };
}
unsafe fn solve_three_empty(mut my_bits: BitBoard,
                                       mut opp_bits: BitBoard,
                                       mut sq1: i32,
                                       mut sq2: i32,
                                       mut sq3: i32,
                                       mut alpha: i32,
                                       mut beta: i32,
                                       mut disc_diff: i32,
                                       mut pass_legal: i32)
 -> i32 {
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut score = -(12345678 as i32);
    let mut flipped: i32 = 0;
    let mut new_disc_diff: i32 = 0;
    let mut ev: i32 = 0;
    nodes.lo = nodes.lo.wrapping_add(1);
    flipped = TestFlips_wrapper(sq1, my_bits, opp_bits);
    if flipped != 0 as i32 {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as i32 * flipped - 1 as i32;
        score =
            -solve_two_empty(new_opp_bits, bb_flips, sq2, sq3, -beta, -alpha,
                             new_disc_diff, 1 as i32);
        if score >= beta {
            return score
        } else { if score > alpha { alpha = score } }
    }
    flipped = TestFlips_wrapper(sq2, my_bits, opp_bits);
    if flipped != 0 as i32 {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as i32 * flipped - 1 as i32;
        ev =
            -solve_two_empty(new_opp_bits, bb_flips, sq1, sq3, -beta, -alpha,
                             new_disc_diff, 1 as i32);
        if ev >= beta {
            return ev
        } else {
            if ev > score { score = ev; if score > alpha { alpha = score } }
        }
    }
    flipped = TestFlips_wrapper(sq3, my_bits, opp_bits);
    if flipped != 0 as i32 {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as i32 * flipped - 1 as i32;
        ev =
            -solve_two_empty(new_opp_bits, bb_flips, sq1, sq2, -beta, -alpha,
                             new_disc_diff, 1 as i32);
        if ev >= score { return ev }
    }
    if score == -(12345678 as i32) {
        if pass_legal == 0 {
            /* Three empty squares */
            if disc_diff > 0 as i32 {
                return disc_diff + 3 as i32
            }
            if disc_diff < 0 as i32 {
                return disc_diff - 3 as i32
            }
            return 0 as i32
            /* Can't reach this code, only keep it for symmetry */
        } else {
            return -solve_three_empty(opp_bits, my_bits, sq1, sq2, sq3, -beta,
                                      -alpha, -disc_diff, 0 as i32)
        }
    }
    return score;
}
unsafe fn solve_four_empty(mut my_bits: BitBoard,
                                      mut opp_bits: BitBoard,
                                      mut sq1: i32,
                                      mut sq2: i32,
                                      mut sq3: i32,
                                      mut sq4: i32,
                                      mut alpha: i32,
                                      mut beta: i32,
                                      mut disc_diff: i32,
                                      mut pass_legal: i32)
 -> i32 {
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut score = -(12345678 as i32);
    let mut flipped: i32 = 0;
    let mut new_disc_diff: i32 = 0;
    let mut ev: i32 = 0;
    nodes.lo = nodes.lo.wrapping_add(1);
    flipped = TestFlips_wrapper(sq1, my_bits, opp_bits);
    if flipped != 0 as i32 {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as i32 * flipped - 1 as i32;
        score =
            -solve_three_empty(new_opp_bits, bb_flips, sq2, sq3, sq4, -beta,
                               -alpha, new_disc_diff, 1 as i32);
        if score >= beta {
            return score
        } else { if score > alpha { alpha = score } }
    }
    flipped = TestFlips_wrapper(sq2, my_bits, opp_bits);
    if flipped != 0 as i32 {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as i32 * flipped - 1 as i32;
        ev =
            -solve_three_empty(new_opp_bits, bb_flips, sq1, sq3, sq4, -beta,
                               -alpha, new_disc_diff, 1 as i32);
        if ev >= beta {
            return ev
        } else {
            if ev > score { score = ev; if score > alpha { alpha = score } }
        }
    }
    flipped = TestFlips_wrapper(sq3, my_bits, opp_bits);
    if flipped != 0 as i32 {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as i32 * flipped - 1 as i32;
        ev =
            -solve_three_empty(new_opp_bits, bb_flips, sq1, sq2, sq4, -beta,
                               -alpha, new_disc_diff, 1 as i32);
        if ev >= beta {
            return ev
        } else {
            if ev > score { score = ev; if score > alpha { alpha = score } }
        }
    }
    flipped = TestFlips_wrapper(sq4, my_bits, opp_bits);
    if flipped != 0 as i32 {
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 as i32 * flipped - 1 as i32;
        ev =
            -solve_three_empty(new_opp_bits, bb_flips, sq1, sq2, sq3, -beta,
                               -alpha, new_disc_diff, 1 as i32);
        if ev >= score { return ev }
    }
    if score == -(12345678 as i32) {
        if pass_legal == 0 {
            /* Four empty squares */
            if disc_diff > 0 as i32 {
                return disc_diff + 4 as i32
            }
            if disc_diff < 0 as i32 {
                return disc_diff - 4 as i32
            }
            return 0 as i32
        } else {
            return -solve_four_empty(opp_bits, my_bits, sq1, sq2, sq3, sq4,
                                     -beta, -alpha, -disc_diff,
                                     0 as i32)
        }
    }
    return score;
}
unsafe fn solve_parity(mut my_bits: BitBoard,
                                  mut opp_bits: BitBoard,
                                  mut alpha: i32,
                                  mut beta: i32,
                                  mut color: i32,
                                  mut empties: i32,
                                  mut disc_diff: i32,
                                  mut pass_legal: i32)
 -> i32 {
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut score = -(12345678 as i32);
    let mut oppcol = 0 as i32 + 2 as i32 - color;
    let mut ev: i32 = 0;
    let mut flipped: i32 = 0;
    let mut new_disc_diff: i32 = 0;
    let mut sq: i32 = 0;
    let mut old_sq: i32 = 0;
    let mut best_sq = 0 as i32;
    let mut parity_mask: u32 = 0;
    nodes.lo = nodes.lo.wrapping_add(1);
    /* Check for stability cutoff */
    if alpha >= stability_threshold[empties as usize] {
        let mut stability_bound: i32 = 0;
        stability_bound =
            64 as i32 -
                2 as i32 *
                    count_edge_stable(oppcol, opp_bits, my_bits);
        if stability_bound <= alpha { return alpha }
        stability_bound =
            64 as i32 -
                2 as i32 * count_stable(oppcol, opp_bits, my_bits);
        if stability_bound < beta {
            beta = stability_bound + 1 as i32
        }
        if stability_bound <= alpha { return alpha }
    }
    /* Odd parity */
    parity_mask = region_parity;
    if region_parity != 0 as i32 as u32 {
        /* Is there any region with odd parity? */
        old_sq = 0 as i32;
        sq = end_move_list[old_sq as usize].succ;
        while sq != 99 as i32 {
            let mut holepar = quadrant_mask[sq as usize];
            if holepar & parity_mask != 0 {
                flipped = TestFlips_wrapper(sq, my_bits, opp_bits);
                if flipped != 0 as i32 {
                    new_opp_bits.high = opp_bits.high & !bb_flips.high;
                    new_opp_bits.low = opp_bits.low & !bb_flips.low;
                    end_move_list[old_sq as usize].succ =
                        end_move_list[sq as usize].succ;
                    new_disc_diff =
                        -disc_diff - 2 as i32 * flipped -
                            1 as i32;
                    if empties == 5 as i32 {
                        let mut sq1 =
                            end_move_list[0 as i32 as usize].succ;
                        let mut sq2 = end_move_list[sq1 as usize].succ;
                        let mut sq3 = end_move_list[sq2 as usize].succ;
                        let mut sq4 = end_move_list[sq3 as usize].succ;
                        ev =
                            -solve_four_empty(new_opp_bits, bb_flips, sq1,
                                              sq2, sq3, sq4, -beta, -alpha,
                                              new_disc_diff, 1 as i32)
                    } else {
                        region_parity ^= holepar;
                        ev =
                            -solve_parity(new_opp_bits, bb_flips, -beta,
                                          -alpha, oppcol,
                                          empties - 1 as i32,
                                          new_disc_diff, 1 as i32);
                        region_parity ^= holepar
                    }
                    end_move_list[old_sq as usize].succ = sq;
                    if ev > score {
                        if ev > alpha {
                            if ev >= beta { best_move = sq; return ev }
                            alpha = ev
                        }
                        score = ev;
                        best_sq = sq
                    }
                }
            }
            old_sq = sq;
            sq = end_move_list[sq as usize].succ
        }
    }
    /* Even parity */
    parity_mask = !parity_mask;
    old_sq = 0 as i32;
    sq = end_move_list[old_sq as usize].succ;
    while sq != 99 as i32 {
        let mut holepar_0 = quadrant_mask[sq as usize];
        if holepar_0 & parity_mask != 0 {
            flipped = TestFlips_wrapper(sq, my_bits, opp_bits);
            if flipped != 0 as i32 {
                new_opp_bits.high = opp_bits.high & !bb_flips.high;
                new_opp_bits.low = opp_bits.low & !bb_flips.low;
                end_move_list[old_sq as usize].succ =
                    end_move_list[sq as usize].succ;
                new_disc_diff =
                    -disc_diff - 2 as i32 * flipped -
                        1 as i32;
                if empties == 5 as i32 {
                    let mut sq1_0 =
                        end_move_list[0 as i32 as usize].succ;
                    let mut sq2_0 = end_move_list[sq1_0 as usize].succ;
                    let mut sq3_0 = end_move_list[sq2_0 as usize].succ;
                    let mut sq4_0 = end_move_list[sq3_0 as usize].succ;
                    ev =
                        -solve_four_empty(new_opp_bits, bb_flips, sq1_0,
                                          sq2_0, sq3_0, sq4_0, -beta, -alpha,
                                          new_disc_diff, 1 as i32)
                } else {
                    region_parity ^= holepar_0;
                    ev =
                        -solve_parity(new_opp_bits, bb_flips, -beta, -alpha,
                                      oppcol, empties - 1 as i32,
                                      new_disc_diff, 1 as i32);
                    region_parity ^= holepar_0
                }
                end_move_list[old_sq as usize].succ = sq;
                if ev > score {
                    if ev > alpha {
                        if ev >= beta { best_move = sq; return ev }
                        alpha = ev
                    }
                    score = ev;
                    best_sq = sq
                }
            }
        }
        old_sq = sq;
        sq = end_move_list[sq as usize].succ
    }
    if score == -(12345678 as i32) {
        if pass_legal == 0 {
            if disc_diff > 0 as i32 { return disc_diff + empties }
            if disc_diff < 0 as i32 { return disc_diff - empties }
            return 0 as i32
        } else {
            return -solve_parity(opp_bits, my_bits, -beta, -alpha, oppcol,
                                 empties, -disc_diff, 0 as i32)
        }
    }
    best_move = best_sq;
    return score;
}
unsafe fn solve_parity_hash(mut my_bits: BitBoard,
                                       mut opp_bits: BitBoard,
                                       mut alpha: i32,
                                       mut beta: i32,
                                       mut color: i32,
                                       mut empties: i32,
                                       mut disc_diff: i32,
                                       mut pass_legal: i32)
 -> i32 {
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut score = -(12345678 as i32);
    let mut oppcol = 0 as i32 + 2 as i32 - color;
    let mut in_alpha = alpha;
    let mut ev: i32 = 0;
    let mut flipped: i32 = 0;
    let mut new_disc_diff: i32 = 0;
    let mut sq: i32 = 0;
    let mut old_sq: i32 = 0;
    let mut best_sq = 0 as i32;
    let mut parity_mask: u32 = 0;
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    nodes.lo = nodes.lo.wrapping_add(1);
    find_hash(&mut entry, 1 as i32);
    if entry.draft as i32 == empties &&
           entry.selectivity as i32 == 0 as i32 &&
           valid_move(entry.move_0[0 as i32 as usize], color) != 0 &&
           entry.flags as i32 & 16 as i32 != 0 &&
           (entry.flags as i32 & 4 as i32 != 0 ||
                entry.flags as i32 & 1 as i32 != 0 &&
                    entry.eval >= beta ||
                entry.flags as i32 & 2 as i32 != 0 &&
                    entry.eval <= alpha) {
        best_move = entry.move_0[0 as i32 as usize];
        return entry.eval
    }
    /* Check for stability cutoff */
    if alpha >= stability_threshold[empties as usize] {
        let mut stability_bound: i32 = 0;
        stability_bound =
            64 as i32 -
                2 as i32 *
                    count_edge_stable(oppcol, opp_bits, my_bits);
        if stability_bound <= alpha { return alpha }
        stability_bound =
            64 as i32 -
                2 as i32 * count_stable(oppcol, opp_bits, my_bits);
        if stability_bound < beta {
            beta = stability_bound + 1 as i32
        }
        if stability_bound <= alpha { return alpha }
    }
    /* Odd parity. */
    parity_mask = region_parity;
    if region_parity != 0 as i32 as u32 {
        /* Is there any region with odd parity? */
        old_sq = 0 as i32;
        sq = end_move_list[old_sq as usize].succ;
        while sq != 99 as i32 {
            let mut holepar = quadrant_mask[sq as usize];
            if holepar & parity_mask != 0 {
                flipped = TestFlips_wrapper(sq, my_bits, opp_bits);
                if flipped != 0 as i32 {
                    new_opp_bits.high = opp_bits.high & !bb_flips.high;
                    new_opp_bits.low = opp_bits.low & !bb_flips.low;
                    region_parity ^= holepar;
                    end_move_list[old_sq as usize].succ =
                        end_move_list[sq as usize].succ;
                    new_disc_diff =
                        -disc_diff - 2 as i32 * flipped -
                            1 as i32;
                    ev =
                        -solve_parity(new_opp_bits, bb_flips, -beta, -alpha,
                                      oppcol, empties - 1 as i32,
                                      new_disc_diff, 1 as i32);
                    region_parity ^= holepar;
                    end_move_list[old_sq as usize].succ = sq;
                    if ev > score {
                        score = ev;
                        if ev > alpha {
                            if ev >= beta {
                                best_move = sq;
                                add_hash(1 as i32, score, best_move,
                                         16 as i32 | 1 as i32,
                                         empties, 0 as i32);
                                return score
                            }
                            alpha = ev
                        }
                        best_sq = sq
                    }
                }
            }
            old_sq = sq;
            sq = end_move_list[sq as usize].succ
        }
    }
    /* Even parity. */
    parity_mask = !parity_mask;
    old_sq = 0 as i32;
    sq = end_move_list[old_sq as usize].succ;
    while sq != 99 as i32 {
        let mut holepar_0 = quadrant_mask[sq as usize];
        if holepar_0 & parity_mask != 0 {
            flipped = TestFlips_wrapper(sq, my_bits, opp_bits);
            if flipped != 0 as i32 {
                new_opp_bits.high = opp_bits.high & !bb_flips.high;
                new_opp_bits.low = opp_bits.low & !bb_flips.low;
                region_parity ^= holepar_0;
                end_move_list[old_sq as usize].succ =
                    end_move_list[sq as usize].succ;
                new_disc_diff =
                    -disc_diff - 2 as i32 * flipped -
                        1 as i32;
                ev =
                    -solve_parity(new_opp_bits, bb_flips, -beta, -alpha,
                                  oppcol, empties - 1 as i32,
                                  new_disc_diff, 1 as i32);
                region_parity ^= holepar_0;
                end_move_list[old_sq as usize].succ = sq;
                if ev > score {
                    score = ev;
                    if ev > alpha {
                        if ev >= beta {
                            best_move = sq;
                            add_hash(1 as i32, score, best_move,
                                     16 as i32 | 1 as i32,
                                     empties, 0 as i32);
                            return score
                        }
                        alpha = ev
                    }
                    best_sq = sq
                }
            }
        }
        old_sq = sq;
        sq = end_move_list[sq as usize].succ
    }
    if score == -(12345678 as i32) {
        if pass_legal == 0 {
            if disc_diff > 0 as i32 { return disc_diff + empties }
            if disc_diff < 0 as i32 { return disc_diff - empties }
            return 0 as i32
        } else {
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2;
            score =
                -solve_parity_hash(opp_bits, my_bits, -beta, -alpha, oppcol,
                                   empties, -disc_diff, 0 as i32);
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2
        }
    } else {
        best_move = best_sq;
        if score > in_alpha {
            add_hash(1 as i32, score, best_move,
                     16 as i32 | 4 as i32, empties,
                     0 as i32);
        } else {
            add_hash(1 as i32, score, best_move,
                     16 as i32 | 2 as i32, empties,
                     0 as i32);
        }
    }
    return score;
}
unsafe fn solve_parity_hash_high(mut my_bits: BitBoard,
                                            mut opp_bits: BitBoard,
                                            mut alpha: i32,
                                            mut beta: i32,
                                            mut color: i32,
                                            mut empties: i32,
                                            mut disc_diff: i32,
                                            mut pass_legal: i32)
 -> i32 {
    /* Move bonuses without and with parity for the squares.
       These are only used when sorting moves in the 9-12 empties
       range and were automatically tuned by OPTIMIZE. */
    static mut move_bonus: [[u8; 128]; 2] =
        [[0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          24 as i32 as u8,
          1 as i32 as u8,
          0 as i32 as u8,
          25 as i32 as u8,
          25 as i32 as u8,
          0 as i32 as u8,
          1 as i32 as u8,
          24 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          1 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          1 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          25 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          25 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          25 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          25 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          1 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          1 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          24 as i32 as u8,
          1 as i32 as u8,
          0 as i32 as u8,
          25 as i32 as u8,
          25 as i32 as u8,
          0 as i32 as u8,
          1 as i32 as u8,
          24 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8],
         [0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          128 as i32 as u8,
          86 as i32 as u8,
          122 as i32 as u8,
          125 as i32 as u8,
          125 as i32 as u8,
          122 as i32 as u8,
          86 as i32 as u8,
          128 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          86 as i32 as u8,
          117 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          117 as i32 as u8,
          86 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          122 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          122 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          125 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          125 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          125 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          125 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          122 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          122 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          86 as i32 as u8,
          117 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          128 as i32 as u8,
          117 as i32 as u8,
          86 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          128 as i32 as u8,
          86 as i32 as u8,
          122 as i32 as u8,
          125 as i32 as u8,
          125 as i32 as u8,
          122 as i32 as u8,
          86 as i32 as u8,
          128 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8,
          0 as i32 as u8]];
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut best_new_my_bits = BitBoard{high: 0, low: 0,};
    let mut best_new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut i: i32 = 0;
    let mut score: i32 = 0;
    let mut in_alpha = alpha;
    let mut oppcol = 0 as i32 + 2 as i32 - color;
    let mut flipped: i32 = 0;
    let mut best_flipped: i32 = 0;
    let mut new_disc_diff: i32 = 0;
    let mut ev: i32 = 0;
    let mut hash_move: i32 = 0;
    let mut moves: i32 = 0;
    let mut parity: i32 = 0;
    let mut best_value: i32 = 0;
    let mut best_index: i32 = 0;
    let mut pred: i32 = 0;
    let mut succ: i32 = 0;
    let mut sq: i32 = 0;
    let mut old_sq: i32 = 0;
    let mut best_sq = 0 as i32;
    let mut move_order: [i32; 64] = [0; 64];
    let mut goodness: [i32; 64] = [0; 64];
    let mut diff1: u32 = 0;
    let mut diff2: u32 = 0;
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    nodes.lo = nodes.lo.wrapping_add(1);
    hash_move = -(1 as i32);
    find_hash(&mut entry, 1 as i32);
    if entry.draft as i32 == empties {
        if entry.selectivity as i32 == 0 as i32 &&
               entry.flags as i32 & 16 as i32 != 0 &&
               valid_move(entry.move_0[0 as i32 as usize], color) != 0
               &&
               (entry.flags as i32 & 4 as i32 != 0 ||
                    entry.flags as i32 & 1 as i32 != 0 &&
                        entry.eval >= beta ||
                    entry.flags as i32 & 2 as i32 != 0 &&
                        entry.eval <= alpha) {
            best_move = entry.move_0[0 as i32 as usize];
            return entry.eval
        }
    }
    /* Check for stability cutoff */
    if alpha >= stability_threshold[empties as usize] {
        let mut stability_bound: i32 = 0;
        stability_bound =
            64 as i32 -
                2 as i32 *
                    count_edge_stable(oppcol, opp_bits, my_bits);
        if stability_bound <= alpha { return alpha }
        stability_bound =
            64 as i32 -
                2 as i32 * count_stable(oppcol, opp_bits, my_bits);
        if stability_bound < beta {
            beta = stability_bound + 1 as i32
        }
        if stability_bound <= alpha { return alpha }
    }
    /* Calculate goodness values for all moves */
    moves = 0 as i32;
    best_value = -(12345678 as i32);
    best_index = 0 as i32;
    best_flipped = 0 as i32;
    old_sq = 0 as i32;
    sq = end_move_list[old_sq as usize].succ;
    while sq != 99 as i32 {
        flipped = TestFlips_wrapper(sq, my_bits, opp_bits);
        if flipped != 0 as i32 {
            nodes.lo = nodes.lo.wrapping_add(1);
            new_opp_bits.high = opp_bits.high & !bb_flips.high;
            new_opp_bits.low = opp_bits.low & !bb_flips.low;
            end_move_list[old_sq as usize].succ =
                end_move_list[sq as usize].succ;
            if quadrant_mask[sq as usize] & region_parity != 0 {
                parity = 1 as i32
            } else { parity = 0 as i32 }
            goodness[moves as usize] =
                move_bonus[parity as usize][sq as usize] as i32;
            if sq == hash_move {
                goodness[moves as usize] += 128 as i32
            }
            goodness[moves as usize] -=
                weighted_mobility(new_opp_bits, bb_flips);
            if goodness[moves as usize] > best_value {
                best_value = goodness[moves as usize];
                best_index = moves;
                best_new_my_bits = bb_flips;
                best_new_opp_bits = new_opp_bits;
                best_flipped = flipped
            }
            end_move_list[old_sq as usize].succ = sq;
            move_order[moves as usize] = sq;
            moves += 1
        }
        old_sq = sq;
        sq = end_move_list[sq as usize].succ
    }
    /* Maybe there aren't any legal moves */
    if moves == 0 as i32 {
        /* I have to pass */
        if pass_legal == 0 {
            /* Last move also pass, game over */
            if disc_diff > 0 as i32 { return disc_diff + empties }
            if disc_diff < 0 as i32 { return disc_diff - empties }
            return 0 as i32
        } else {
            /* Opponent gets the chance to play */
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2;
            score =
                -solve_parity_hash_high(opp_bits, my_bits, -beta, -alpha,
                                        oppcol, empties, -disc_diff,
                                        0 as i32);
            hash1 ^= hash_flip_color1;
            hash2 ^= hash_flip_color2;
            return score
        }
    }
    /* Try move with highest goodness value */
    sq = move_order[best_index as usize];
    DoFlips_hash(sq, color);
    board[sq as usize] = color;
    diff1 = hash_update1 ^ hash_put_value1[color as usize][sq as usize];
    diff2 = hash_update2 ^ hash_put_value2[color as usize][sq as usize];
    hash1 ^= diff1;
    hash2 ^= diff2;
    region_parity ^= quadrant_mask[sq as usize];
    pred = end_move_list[sq as usize].pred;
    succ = end_move_list[sq as usize].succ;
    end_move_list[pred as usize].succ = succ;
    end_move_list[succ as usize].pred = pred;
    new_disc_diff =
        -disc_diff - 2 as i32 * best_flipped - 1 as i32;
    if empties <= 8 as i32 + 1 as i32 {
        score =
            -solve_parity_hash(best_new_opp_bits, best_new_my_bits, -beta,
                               -alpha, oppcol, empties - 1 as i32,
                               new_disc_diff, 1 as i32)
    } else {
        score =
            -solve_parity_hash_high(best_new_opp_bits, best_new_my_bits,
                                    -beta, -alpha, oppcol,
                                    empties - 1 as i32, new_disc_diff,
                                    1 as i32)
    }
    UndoFlips(best_flipped, oppcol);
    hash1 ^= diff1;
    hash2 ^= diff2;
    board[sq as usize] = 1 as i32;
    region_parity ^= quadrant_mask[sq as usize];
    end_move_list[pred as usize].succ = sq;
    end_move_list[succ as usize].pred = sq;
    best_sq = sq;
    if score > alpha {
        if score >= beta {
            best_move = best_sq;
            add_hash(1 as i32, score, best_move,
                     16 as i32 | 1 as i32, empties,
                     0 as i32);
            return score
        }
        alpha = score
    }
    /* Play through the rest of the moves */
    move_order[best_index as usize] = move_order[0 as i32 as usize];
    goodness[best_index as usize] = goodness[0 as i32 as usize];
    i = 1 as i32;
    while i < moves {
        let mut j: i32 = 0;
        best_value = goodness[i as usize];
        best_index = i;
        j = i + 1 as i32;
        while j < moves {
            if goodness[j as usize] > best_value {
                best_value = goodness[j as usize];
                best_index = j
            }
            j += 1
        }
        sq = move_order[best_index as usize];
        move_order[best_index as usize] = move_order[i as usize];
        goodness[best_index as usize] = goodness[i as usize];
        flipped = TestFlips_wrapper(sq, my_bits, opp_bits);
        new_opp_bits.high = opp_bits.high & !bb_flips.high;
        new_opp_bits.low = opp_bits.low & !bb_flips.low;
        DoFlips_hash(sq, color);
        board[sq as usize] = color;
        diff1 = hash_update1 ^ hash_put_value1[color as usize][sq as usize];
        diff2 = hash_update2 ^ hash_put_value2[color as usize][sq as usize];
        hash1 ^= diff1;
        hash2 ^= diff2;
        region_parity ^= quadrant_mask[sq as usize];
        pred = end_move_list[sq as usize].pred;
        succ = end_move_list[sq as usize].succ;
        end_move_list[pred as usize].succ = succ;
        end_move_list[succ as usize].pred = pred;
        new_disc_diff =
            -disc_diff - 2 as i32 * flipped - 1 as i32;
        if empties <= 8 as i32 {
            /* Fail-high for opp is likely. */
            ev =
                -solve_parity_hash(new_opp_bits, bb_flips, -beta, -alpha,
                                   oppcol, empties - 1 as i32,
                                   new_disc_diff, 1 as i32)
        } else {
            ev =
                -solve_parity_hash_high(new_opp_bits, bb_flips, -beta, -alpha,
                                        oppcol, empties - 1 as i32,
                                        new_disc_diff, 1 as i32)
        }
        region_parity ^= quadrant_mask[sq as usize];
        UndoFlips(flipped, oppcol);
        hash1 ^= diff1;
        hash2 ^= diff2;
        board[sq as usize] = 1 as i32;
        end_move_list[pred as usize].succ = sq;
        end_move_list[succ as usize].pred = sq;
        if ev > score {
            score = ev;
            if ev > alpha {
                if ev >= beta {
                    best_move = sq;
                    add_hash(1 as i32, score, best_move,
                             16 as i32 | 1 as i32, empties,
                             0 as i32);
                    return score
                }
                alpha = ev
            }
            best_sq = sq
        }
        i += 1
    }
    best_move = best_sq;
    if score > in_alpha {
        add_hash(1 as i32, score, best_move,
                 16 as i32 | 4 as i32, empties,
                 0 as i32);
    } else {
        add_hash(1 as i32, score, best_move,
                 16 as i32 | 2 as i32, empties,
                 0 as i32);
    }
    return score;
}
/*
  END_SOLVE
  The search itself. Assumes relevant data structures have been set up with
  PREPARE_TO_SOLVE(). Returns difference between disc count for
  COLOR and disc count for the opponent of COLOR.
*/
unsafe fn end_solve(mut my_bits: BitBoard, mut opp_bits: BitBoard,
                               mut alpha: i32, mut beta: i32,
                               mut color: i32,
                               mut empties: i32,
                               mut discdiff: i32,
                               mut prevmove: i32) -> i32 {
    let mut result: i32 = 0;
    if empties <= 8 as i32 {
        result =
            solve_parity(my_bits, opp_bits, alpha, beta, color, empties,
                         discdiff, prevmove)
    } else {
        result =
            solve_parity_hash_high(my_bits, opp_bits, alpha, beta, color,
                                   empties, discdiff, prevmove)
    }
    return result;
}
/*
  UPDATE_BEST_LIST
*/
unsafe fn update_best_list(mut best_list: *mut i32,
                                      mut move_0: i32,
                                      mut best_list_index: i32,
                                      mut best_list_length: *mut i32,
                                      mut verbose: i32) {
    let mut i: i32 = 0;
    verbose = 0 as i32;
    if verbose != 0 {
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
    if best_list_index < *best_list_length {
        i = best_list_index;
        while i >= 1 as i32 {
            *best_list.offset(i as isize) =
                *best_list.offset((i - 1 as i32) as isize);
            i -= 1
        }
    } else {
        i = 3 as i32;
        while i >= 1 as i32 {
            *best_list.offset(i as isize) =
                *best_list.offset((i - 1 as i32) as isize);
            i -= 1
        }
        if *best_list_length < 4 as i32 { *best_list_length += 1 }
    }
    *best_list.offset(0 as i32 as isize) = move_0;
    if verbose != 0 {
        printf(b"      After:  \x00" as *const u8 as *const i8);
        i = 0 as i32;
        while i < 4 as i32 {
            printf(b"%2d \x00" as *const u8 as *const i8,
                   *best_list.offset(i as isize));
            i += 1
        }
        puts(b"\x00" as *const u8 as *const i8);
    };
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
            display_status(stdout, 0 as i32);
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
                    if root_eval > alpha {
                    } else {
                        __assert_fail(b"root_eval > alpha\x00" as *const u8 as
                                          *const i8,
                                      b"end.c\x00" as *const u8 as
                                          *const i8,
                                      2126 as i32 as u32,
                                      (*::std::mem::transmute::<&[u8; 56],
                                                                &[i8; 56]>(b"int end_game(int, int, int, int, int, EvaluationType *)\x00")).as_ptr());
                    }
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
                    display_status(stdout, 0 as i32);
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
                display_status(stdout, 0 as i32);
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
        display_status(stdout, 0 as i32);
    }
    /* For shallow endgames, we can afford to compute the entire PV
       move by move. */
    if wld == 0 && incomplete_search == 0 && force_return == 0 &&
           empties <= 16 as i32 {
        full_expand_pv(side_to_move, 0 as i32);
    }
    return pv[0 as i32 as usize][0 as i32 as usize];
}
/*
   SETUP_END
   Prepares the endgame solver for a new game.
   This means clearing a few status fields.
*/

pub unsafe fn setup_end() {
    let mut last_mean: f64 = 0.;
    let mut last_sigma: f64 = 0.;
    let mut ff_threshold: [f64; 61] = [0.; 61];
    let mut prelim_threshold: [[f64; 64]; 61] = [[0.; 64]; 61];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    static mut dir_shift: [i32; 8] =
        [1 as i32, -(1 as i32), 7 as i32,
         -(7 as i32), 8 as i32, -(8 as i32),
         9 as i32, -(9 as i32)];
    earliest_wld_solve = 0 as i32;
    earliest_full_solve = 0 as i32;
    full_output_mode = 1 as i32;
    /* Calculate the neighborhood masks */
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            /* Create the neighborhood mask for the square POS */
            let mut pos = 10 as i32 * i + j;
            let mut shift =
                8 as i32 * (i - 1 as i32) +
                    (j - 1 as i32);
            let mut k: u32 = 0;
            neighborhood_mask[pos as usize].low =
                0 as i32 as u32;
            neighborhood_mask[pos as usize].high =
                0 as i32 as u32;
            k = 0 as i32 as u32;
            while k < 8 as i32 as u32 {
                if dir_mask[pos as usize] & (1 as i32) << k != 0 {
                    let mut neighbor =
                        (shift + dir_shift[k as usize]) as u32;
                    if neighbor < 32 as i32 as u32 {
                        neighborhood_mask[pos as usize].low |=
                            ((1 as i32) << neighbor) as u32
                    } else {
                        neighborhood_mask[pos as usize].high |=
                            ((1 as i32) <<
                                 neighbor.wrapping_sub(32 as i32 as
                                                           u32)) as
                                u32
                    }
                }
                k = k.wrapping_add(1)
            }
            j += 1
        }
        i += 1
    }
    /* Set the fastest-first mobility encouragements and thresholds */
    i = 0 as i32;
    while i <= 60 as i32 {
        ff_mob_factor[i as usize] = 460 as i32;
        i += 1
    }
    i = 0 as i32;
    while i <= 60 as i32 {
        ff_threshold[i as usize] = 0.45f64;
        i += 1
    }
    /* Calculate the alpha thresholds for using fastest-first for
       each #empty and shallow search depth. */
    j = 0 as i32; /* Infinity in disc difference */
    while j <= 8 as i32 {
        last_sigma = 100.0f64;
        last_mean = 0.0f64;
        i = 60 as i32;
        while i >= 0 as i32 {
            if end_stats_available[i as usize][j as usize] != 0 {
                last_mean =
                    end_mean[i as usize][j as usize] as f64;
                last_sigma =
                    ff_threshold[i as usize] *
                        end_sigma[i as usize][j as usize] as f64
            }
            fast_first_mean[i as usize][j as usize] = last_mean;
            fast_first_sigma[i as usize][j as usize] = last_sigma;
            prelim_threshold[i as usize][j as usize] = last_mean + last_sigma;
            i -= 1
        }
        j += 1
    }
    j = 8 as i32 + 1 as i32;
    while j < 64 as i32 {
        i = 0 as i32;
        while i <= 60 as i32 {
            prelim_threshold[i as usize][j as usize] =
                prelim_threshold[i as usize][8 as i32 as usize];
            i += 1
        }
        j += 1
    }
    i = 0 as i32;
    while i <= 60 as i32 {
        j = 0 as i32;
        while j < 64 as i32 {
            fast_first_threshold[i as usize][j as usize] =
                ceil(prelim_threshold[i as usize][j as usize] * 128.0f64) as
                    i32;
            j += 1
        }
        i += 1
    };
}
/*
   File:          end.h

   Created:       June 25, 1997

   Modified:      November 24, 2005

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The interface to the endgame solver.
*/
/*
  GET_EARLIEST_WLD_SOLVE
  GET_EARLIEST_FULL_SOLVE
  Return the highest #empty when WLD and full solve respectively
  were completed (not initiated).
*/

pub unsafe fn get_earliest_wld_solve() -> i32 {
    return earliest_wld_solve;
}

pub unsafe fn get_earliest_full_solve() -> i32 {
    return earliest_full_solve;
}
/*
  SET_OUTPUT_MODE
  Toggles output of intermediate search status on/off.
*/

pub unsafe fn set_output_mode(mut full: i32) {
    full_output_mode = full;
}
