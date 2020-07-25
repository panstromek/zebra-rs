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
use crate::src::stubs::ceil;
use crate::src::hash::add_hash;
extern "C" {
    #[no_mangle]
    fn after_update_best_list_verbose(best_list: *mut i32);

    #[no_mangle]
     fn before_update_best_list_verbose(best_list: *mut i32, move_0: i32, best_list_index: i32, best_list_length: *mut i32);
}

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


pub unsafe fn solve_parity_hash(mut my_bits: BitBoard,
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

pub unsafe fn solve_parity_hash_high(mut my_bits: BitBoard,
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
pub unsafe fn end_solve(mut my_bits: BitBoard, mut opp_bits: BitBoard,
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
pub unsafe fn update_best_list(mut best_list: *mut i32,
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
