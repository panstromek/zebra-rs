use crate::{
    src:: {
        epcstat::{end_sigma, end_mean, end_stats_available},
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



/* The parities of the regions are in the region_parity bit vector. */
pub static mut region_parity: u32 = 0;
/* Pseudo-probabilities corresponding to the percentiles.
   These are taken from the normal distribution; to the percentile
   x corresponds the probability Pr(-x <= Y <= x) where Y is a N(0,1)
   variable. */
pub static mut confidence: [f64; 10] =
    [1.000f64, 0.99f64, 0.98f64, 0.954f64, 0.911f64, 0.838f64, 0.729f64,
        0.576f64, 0.383f64, 0.197f64];
/* Percentiles used in the endgame MPC */
pub static mut end_percentile: [f64; 10] =
    [100.0f64, 4.0f64, 3.0f64, 2.0f64, 1.7f64, 1.4f64, 1.1f64, 0.8f64, 0.5f64,
        0.25f64];
pub static mut stability_threshold: [i32; 19] =
    [65 as i32, 65 as i32, 65 as i32,
        65 as i32, 65 as i32, 46 as i32,
        38 as i32, 30 as i32, 24 as i32,
        24 as i32, 24 as i32, 24 as i32,
        0 as i32, 0 as i32, 0 as i32, 0 as i32,
        0 as i32, 0 as i32, 0 as i32];
pub static mut fast_first_mean: [[f64; 64]; 61] = [[0.; 64]; 61];
pub static mut fast_first_sigma: [[f64; 64]; 61] = [[0.; 64]; 61];
pub static mut best_move: i32 = 0;
pub static mut best_end_root_move: i32 = 0;
pub static mut true_found: i32 = 0;
pub static mut true_val: i32 = 0;
pub static mut full_output_mode: i32 = 0;
pub static mut earliest_wld_solve: i32 = 0;
pub static mut earliest_full_solve: i32 = 0;
pub static mut fast_first_threshold: [[i32; 64]; 61] = [[0; 64]; 61];
pub static mut ff_mob_factor: [i32; 61] = [0; 61];
pub static mut neighborhood_mask: [BitBoard; 100] =
    [BitBoard{high: 0, low: 0,}; 100];

/* Number of discs that the side to move at the root has to win with. */
pub static mut komi_shift: i32 = 0;
/*
  TESTFLIPS_WRAPPER
  Checks if SQ is a valid move by
  (1) verifying that there exists a neighboring opponent disc,
  (2) verifying that the move flips some disc.
*/
pub unsafe fn TestFlips_wrapper(mut sq: i32,
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
pub unsafe fn prepare_to_solve(mut board_0: *const i32) {
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
pub unsafe fn solve_four_empty(mut my_bits: BitBoard,
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


pub unsafe fn solve_parity(mut my_bits: BitBoard,
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
