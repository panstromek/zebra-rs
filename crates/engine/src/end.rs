use flip::doflip::DoFlips_hash;

use crate::{
    src:: {
        bitbcnt::CountFlips_bitboard,
        bitbmob::{bitboard_mobility, weighted_mobility},
        bitboard::{BitBoard, set_bitboards},
        bitbtest::TestFlips_bitboard,
        counter::{adjust_counter, counter_value},
        epcstat::{END_MEAN, END_SIGMA, END_STATS_AVAILABLE},
        hash::{find_hash,  HashEntry},
        moves::{dir_mask, make_move},
        search::{create_eval_info, disc_count, force_return, hash_expand_pv, restore_pv, select_move, store_pv},
        stable::{count_edge_stable, count_stable},
    }
};
use crate::src::error::FrontEnd;
use crate::src::globals::{Board, BoardState};
use crate::src::hash::{add_hash, HashState};
use crate::src::midgame::{tree_search, MidgameState};
use crate::src::moves::{generate_all, unmake_move, valid_move, MovesState};
use crate::src::osfbook::{fill_endgame_hash, fill_move_alternatives, get_book_move, Book};
use crate::src::search::SearchState;
use crate::src::stubs::{abs, ceil};
use crate::src::zebra::EvalResult::{DRAWN_POSITION, LOST_POSITION, UNSOLVED_POSITION, WON_POSITION};
use crate::src::zebra::EvalType::{EXACT_EVAL, MIDGAME_EVAL, SELECTIVE_EVAL, WLD_EVAL};
use crate::src::zebra::{EvaluationType, FullState};
use crate::src::stable::StableState;
use std::collections::hash_map::RandomState;
use crate::src::myrandom::MyRandom;
use crate::src::getcoeff::CoeffState;
use crate::src::timer::Timer;
use flip::unflip::FlipStack;
use crate::src::probcut::ProbCut;

#[derive(Copy, Clone)]
#[repr(C)]
struct MoveLink {
    pub pred: i8,
    pub succ: i8,
}
// probably this:   enum { WIN, LOSS, DRAW, UNKNOWN } solve_status;
const DRAW: u32 = 2;
const UNKNOWN: u32 = 3;
const LOSS: u32 = 1;
const WIN: u32 = 0;

static quadrant_mask: [u8; 100] = [
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

pub struct End {
    /* The parities of the regions are in the region_parity bit vector. */
    region_parity: u32,
    end_move_list: [MoveLink; 100],
    best_move: i8,
    best_end_root_move: i8,
    full_output_mode: i32,
    earliest_wld_solve: i32,
    earliest_full_solve: i32,
    fast_first_threshold: [[i32; 64]; 61],
    ff_mob_factor: [i32; 61],
    neighborhood_mask: [BitBoard; 100],
    /* Number of discs that the side to move at the root has to win with. */
    komi_shift: i32,
    pub bb_flips: BitBoard
}

impl End {
    pub const fn new() -> End {
        End {
            /* The parities of the regions are in the region_parity bit vector. */
            region_parity: 0,
            end_move_list: [MoveLink { pred: 0, succ: 0 }; 100],
            best_move: 0,
            best_end_root_move: 0,
            full_output_mode: 0,
            earliest_wld_solve: 0,
            earliest_full_solve: 0,
            fast_first_threshold: [[0; 64]; 61],
            ff_mob_factor: [0; 61],
            neighborhood_mask: [BitBoard { high: 0, low: 0 }; 100],
            komi_shift: 0,
            bb_flips: BitBoard { high: 0, low: 0 }
        }
    }
}


/* Pseudo-probabilities corresponding to the percentiles.
   These are taken from the normal distribution; to the percentile
   x corresponds the probability Pr(-x <= Y <= x) where Y is a N(0,1)
   variable. */
static confidence: [f64; 10] = [1.000, 0.99, 0.98, 0.954, 0.911, 0.838, 0.729, 0.576, 0.383, 0.197];
/* Percentiles used in the endgame MPC */
static end_percentile: [f64; 10] = [100.0f64, 4.0f64, 3.0f64, 2.0f64, 1.7f64, 1.4f64, 1.1f64, 0.8f64, 0.5f64, 0.25f64];
static stability_threshold: [i32; 19] = [65, 65, 65, 65, 65, 46, 38, 30, 24, 24, 24, 24, 0, 0, 0, 0, 0, 0, 0];
/*
  TESTFLIPS_WRAPPER
  Checks if SQ is a valid move by
  (1) verifying that there exists a neighboring opponent disc,
  (2) verifying that the move flips some disc.
*/
fn TestFlips_wrapper(end: &mut End, sq: i8, my_bits: BitBoard, opp_bits: BitBoard) -> i32 {
    if end.neighborhood_mask[sq as usize].high & opp_bits.high |
        end.neighborhood_mask[sq as usize].low & opp_bits.low != 0 {

        let (flipped, bb) =
            TestFlips_bitboard[(sq - 11) as usize](my_bits.high, my_bits.low, opp_bits.high, opp_bits.low);
        end.bb_flips = bb;
        flipped
    } else {
        0
    }
}
/*
  PREPARE_TO_SOLVE
  Create the list of empty squares.
*/
fn prepare_to_solve(board_0: &Board, end:&mut End) {
    /* fixed square ordering: */
    /* jcw's order, which is the best of 4 tried (according to Warren Smith) */
    static worst2best: [i8; 64] = [
        /*B2*/      22 , 27 , 72 , 77 ,
        /*B1*/      12 , 17 , 21 , 28 , 71 , 78 , 82,  87 ,
        /*C2*/      23 , 26 , 32 , 37 , 62 , 67 , 73 , 76 ,
        /*D2*/      24 , 25 , 42 , 47 , 52 , 57 , 74 , 75 ,
        /*D3*/      34 , 35 , 43 , 46 , 53 , 56 , 64 , 65 ,
        /*C1*/      13 , 16 , 31 , 38 , 61 , 68 , 83 , 86 ,
        /*D1*/      14 , 15 , 41 , 48 , 51 , 58 , 84 , 85 ,
        /*C3*/      33 , 36 , 63 , 66 ,
        /*A1*/      11 , 18 , 81 , 88 ,
        /*D4*/      44 , 45 , 54 , 45
    ];
    end.region_parity = 0;
    let mut last_sq = 0;
    let mut i = 59;
    loop {
        let sq = worst2best[i];
        if board_0[sq as usize] == 1 {
            end. end_move_list[last_sq as usize].succ = sq;
            end. end_move_list[sq as usize].pred = last_sq;
           end.region_parity ^= quadrant_mask[sq as usize] as u32;
            last_sq = sq
        }
        if i == 0 {
            break;
        }
        i -= 1
    }
    end. end_move_list[last_sq as usize].succ = 99;
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
fn solve_two_empty(end: &mut End, my_bits: BitBoard,
                          opp_bits: BitBoard,
                          sq1: i8,
                          sq2: i8,
                          mut alpha: i32,
                          beta: i32,
                          disc_diff: i32,
                          pass_legal: i32, search_state_: &mut SearchState)
                          -> i32 {
    // BitBoard new_opp_bits;
    let mut score = -(12345678);
    let mut flipped: i32 = 0;
    let mut ev: i32 = 0;
    let search_state_ = search_state_;
    search_state_.nodes.lo = search_state_.nodes.lo.wrapping_add(1);
    /* Overall strategy: Lazy evaluation whenever possible, i.e., don't
       update bitboards until they are used. Also look at alpha and beta
       in order to perform strength reduction: Feasibility testing is
       faster than counting number of flips. */
    /* Try the first of the two empty squares... */
    flipped = TestFlips_wrapper(end,sq1, my_bits, opp_bits);
    if flipped != 0 {
        /* SQ1 feasible for me */
        search_state_.nodes.lo = search_state_.nodes.lo.wrapping_add(1);
        ev = disc_diff + 2 * flipped;
        flipped = CountFlips_bitboard[(sq2 - 11) as usize](opp_bits.high & !end.bb_flips.high, opp_bits.low & !end.bb_flips.low);
        if flipped != 0 {
            ev -= 2 * flipped
        } else if ev >= 0 {
            /* He passes, check if SQ2 is feasible for me */
            /* I'm ahead, so EV will increase by at least 2 */
            ev += 2;
            if ev < beta {
                /* Only bother if not certain fail-high */
                ev += 2 * CountFlips_bitboard[(sq2 - 11) as usize](end.bb_flips.high, end.bb_flips.low)
            }
        } else if ev < beta {
            /* Only bother if not fail-high already */
            flipped = CountFlips_bitboard[(sq2 - 11) as usize](end.bb_flips.high, end.bb_flips.low);
            if flipped != 0 {
                /* ELSE: SQ2 will end up empty, game over */
                /* SQ2 feasible for me, game over */
                ev += 2 * (flipped + 1)
            }
        }
        /* Being legal, the first move is the best so far */
        score = ev;
        if score > alpha { if score >= beta { return score } alpha = score }
    }
    /* ...and then the second */
    flipped = TestFlips_wrapper(end,sq2, my_bits, opp_bits, );
    if flipped != 0 {
        /* SQ2 feasible for me */
        search_state_.nodes.lo = search_state_.nodes.lo.wrapping_add(1);
        ev = disc_diff + 2 * flipped;
        flipped = CountFlips_bitboard[(sq1 - 11) as usize](opp_bits.high & !end.bb_flips.high, opp_bits.low & !end.bb_flips.low);
        if flipped != 0 {
            /* SQ1 feasible for him, game over */
            ev -= 2 * flipped
        } else if ev >= 0 {
            /* He passes, check if SQ1 is feasible for me */
            /* I'm ahead, so EV will increase by at least 2 */
            ev += 2;
            if ev < beta {
                /* Only bother if not certain fail-high */
                ev += 2 * CountFlips_bitboard[(sq1 - 11) as usize](end.bb_flips.high, end.bb_flips.low)
            }
        } else if ev < beta {
            /* Only bother if not fail-high already */
            flipped = CountFlips_bitboard[(sq1 - 11) as usize](end.bb_flips.high, end.bb_flips.low);
            if flipped != 0 {
                /* ELSE: SQ1 will end up empty, game over */
                /* SQ1 feasible for me, game over */
                ev += 2 * (flipped + 1)
            }
        }
        /* If the second move is better than the first (if that move was legal),
           its score is the score of the position */
        if ev >= score { return ev }
    }
    /* If both SQ1 and SQ2 are illegal I have to pass,
       otherwise return the best score. */
    return if score == -12345678 {
        if pass_legal == 0 {
            /* Two empty squares */
            if disc_diff > 0 {
                disc_diff + 2
            } else if disc_diff < 0 {
                disc_diff - 2
            } else {
                0
            }
        } else {
            -solve_two_empty(end, opp_bits, my_bits, sq1, sq2, -beta,
                             -alpha, -disc_diff, 0, search_state_)
        }
    } else { score };
}
fn solve_three_empty(end: &mut End, my_bits: BitBoard,
                            opp_bits: BitBoard,
                            sq1: i8,
                            sq2: i8,
                            sq3: i8,
                            mut alpha: i32,
                            beta: i32,
                            disc_diff: i32,
                            pass_legal: i32, state: &mut SearchState)
                            -> i32 {
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut score = -(12345678);
    let mut flipped: i32 = 0;
    let mut new_disc_diff: i32 = 0;
    let mut ev: i32 = 0;
    state.nodes.lo = state.nodes.lo.wrapping_add(1);
    flipped = TestFlips_wrapper(end,sq1, my_bits, opp_bits, );
    if flipped != 0 {
        new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
        new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 * flipped - 1;
        score =
            -solve_two_empty(end, new_opp_bits, end.bb_flips, sq2, sq3, -beta, -alpha,
                             new_disc_diff, 1, state);
        if score >= beta {
            return score
        } else { if score > alpha { alpha = score } }
    }
    flipped = TestFlips_wrapper(end,sq2, my_bits, opp_bits, );
    if flipped != 0 {
        new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
        new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 * flipped - 1;
        ev =
            -solve_two_empty(end, new_opp_bits, end.bb_flips, sq1, sq3, -beta, -alpha,
                             new_disc_diff, 1,  state);
        if ev >= beta {
            return ev
        } else {
            if ev > score { score = ev; if score > alpha { alpha = score } }
        }
    }
    flipped = TestFlips_wrapper(end,sq3, my_bits, opp_bits, );
    if flipped != 0 {
        new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
        new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
        new_disc_diff =
            -disc_diff - 2 * flipped - 1;
        ev =
            -solve_two_empty(end, new_opp_bits, end.bb_flips, sq1, sq2, -beta, -alpha,
                             new_disc_diff, 1,  state);
        if ev >= score { return ev }
    }
    if score == -(12345678) {
        if pass_legal == 0 {
            /* Three empty squares */
            if disc_diff > 0 {
                return disc_diff + 3
            }
            if disc_diff < 0 {
                return disc_diff - 3
            }
            return 0
            /* Can't reach this code, only keep it for symmetry */
        } else {
            return -solve_three_empty(end, opp_bits, my_bits, sq1, sq2, sq3, -beta,
                                      -alpha, -disc_diff, 0, state)
        }
    }
    return score;
}
fn solve_four_empty(end: &mut End, my_bits: BitBoard,
                           opp_bits: BitBoard,
                           sq1: i8,
                           sq2: i8,
                           sq3: i8,
                           sq4: i8,
                           mut alpha: i32,
                           beta: i32,
                           disc_diff: i32,
                           pass_legal: i32, search_state_: &mut SearchState)
                           -> i32 {
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut score = -(12345678);
    let mut flipped: i32 = 0;
    let mut new_disc_diff: i32 = 0;
    let mut ev: i32 = 0;
    search_state_.nodes.lo = search_state_.nodes.lo.wrapping_add(1);
    flipped = TestFlips_wrapper(end,sq1, my_bits, opp_bits, );
    if flipped != 0 {
        new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
        new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
        new_disc_diff = -disc_diff - 2 * flipped - 1;
        score = -solve_three_empty(end, new_opp_bits, end.bb_flips, sq2, sq3, sq4, -beta,
                               -alpha, new_disc_diff, 1,  search_state_);
        if score >= beta {
            return score
        } else if score > alpha {
            alpha = score
        }
    }
    flipped = TestFlips_wrapper(end,sq2, my_bits, opp_bits, );
    if flipped != 0 {
        new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
        new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
        new_disc_diff = -disc_diff - 2 * flipped - 1;
        ev = -solve_three_empty(end, new_opp_bits, end.bb_flips, sq1, sq3, sq4, -beta,
                                -alpha, new_disc_diff, 1, search_state_);
        if ev >= beta {
            return ev
        } else if ev > score {
            score = ev;
            if score > alpha {
                alpha = score
            }
        }
    }
    flipped = TestFlips_wrapper(end,sq3, my_bits, opp_bits, );
    if flipped != 0 {
        new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
        new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
        new_disc_diff = -disc_diff - 2 * flipped - 1;
        ev = -solve_three_empty(end, new_opp_bits, end.bb_flips, sq1, sq2, sq4, -beta,
                               -alpha, new_disc_diff, 1,  search_state_);
        if ev >= beta {
            return ev
        } else if ev > score {
            score = ev;
            if score > alpha {
                alpha = score
            }
        }
    }
    flipped = TestFlips_wrapper(end,sq4, my_bits, opp_bits, );
    if flipped != 0 {
        new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
        new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
        new_disc_diff = -disc_diff - 2 * flipped - 1;
        ev = -solve_three_empty(end, new_opp_bits, end.bb_flips, sq1, sq2, sq3, -beta,
                               -alpha, new_disc_diff, 1, search_state_);
        if ev >= score { return ev }
    }
    if score == -(12345678) {
        if pass_legal == 0 {
            /* Four empty squares */
            if disc_diff > 0 {
                return disc_diff + 4
            }
            if disc_diff < 0 {
                return disc_diff - 4
            }
            return 0
        } else {
            return -solve_four_empty(end, opp_bits, my_bits, sq1, sq2, sq3, sq4,
                                     -beta, -alpha, -disc_diff,
                                     0,  search_state_)
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
impl End {
    /*
      GET_EARLIEST_WLD_SOLVE
      GET_EARLIEST_FULL_SOLVE
      Return the highest #empty when WLD and full solve respectively
      were completed (not initiated).
    */
    pub fn get_earliest_wld_solve(&self) -> i32 {
        return self.earliest_wld_solve;
    }

    pub fn get_earliest_full_solve(&self) -> i32 {
        return self.earliest_full_solve;
    }
}
// /*
//   SET_OUTPUT_MODE
//   Toggles output of intermediate search status on/off.
// */
//
// pub un safe fn set_output_mode(end:&mut End, full: i32) {
//     end.full_output_mode = full;
// }


fn solve_parity(end:&mut End, my_bits: BitBoard,
                           opp_bits: BitBoard,
                           mut alpha: i32,
                           mut beta: i32,
                           color: i32,
                           empties: i32,
                           disc_diff: i32,
                           pass_legal: i32,
                       mut search_state: &mut SearchState,
                       mut stable_state: &mut StableState,
)
                           -> i32 {
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut score = -(12345678);
    let oppcol = 0 + 2 - color;
    let mut ev: i32 = 0;
    let mut flipped: i32 = 0;
    let mut new_disc_diff: i32 = 0;
    let mut sq = 0;
    let mut old_sq = 0;
    let mut best_sq = 0;
    let mut parity_mask: u32 = 0;
    search_state.nodes.lo = search_state.nodes.lo.wrapping_add(1);
    /* Check for stability cutoff */
    if alpha >= stability_threshold[empties as usize] {
        let mut stability_bound: i32 = 0;
        stability_bound = 64 - 2 * count_edge_stable(oppcol, opp_bits, my_bits, &mut stable_state);
        if stability_bound <= alpha { return alpha }
        stability_bound = 64 - 2 * count_stable(oppcol, opp_bits, my_bits, &mut stable_state);
        if stability_bound < beta {
            beta = stability_bound + 1
        }
        if stability_bound <= alpha { return alpha }
    }
    /* Odd parity */
    parity_mask =end.region_parity;
    if end.region_parity != 0 {
        /* Is there any region with odd parity? */
        old_sq = 0;
        sq = end. end_move_list[old_sq as usize].succ;
        while sq != 99 {
            let holepar = quadrant_mask[sq as usize] as u32;
            if holepar & parity_mask != 0 {
                flipped = TestFlips_wrapper(end,sq, my_bits, opp_bits, );
                if flipped != 0 {
                    new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
                    new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
                    end.end_move_list[old_sq as usize].succ = end.end_move_list[sq as usize].succ;
                    new_disc_diff = -disc_diff - 2 * flipped - 1;
                    if empties == 5 {
                        let sq1 = end.end_move_list[0].succ;
                        let sq2 = end.end_move_list[sq1 as usize].succ;
                        let sq3 = end.end_move_list[sq2 as usize].succ;
                        let sq4 = end.end_move_list[sq3 as usize].succ;
                        ev = -solve_four_empty(end, new_opp_bits, end.bb_flips, sq1,
                                              sq2, sq3, sq4, -beta, -alpha,
                                              new_disc_diff, 1, &mut search_state)
                    } else {
                        end.region_parity ^= holepar;
                        ev = -solve_parity(end, new_opp_bits, end.bb_flips, -beta,
                                          -alpha, oppcol,
                                          empties - 1,
                                          new_disc_diff, 1, &mut search_state, &mut stable_state);
                        end.region_parity ^= holepar
                    }
                    end.end_move_list[old_sq as usize].succ = sq;
                    if ev > score {
                        if ev > alpha {
                            if ev >= beta { end.  best_move = sq; return ev }
                            alpha = ev
                        }
                        score = ev;
                        best_sq = sq
                    }
                }
            }
            old_sq = sq;
            sq = end. end_move_list[sq as usize].succ
        }
    }
    /* Even parity */
    parity_mask = !parity_mask;
    old_sq = 0;
    sq = end. end_move_list[old_sq as usize].succ;
    while sq != 99 {
        let holepar_0 = quadrant_mask[sq as usize] as u32;
        if holepar_0 & parity_mask != 0 {
            flipped = TestFlips_wrapper(end,sq, my_bits, opp_bits, );
            if flipped != 0 {
                new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
                new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
                end. end_move_list[old_sq as usize].succ = end. end_move_list[sq as usize].succ;
                new_disc_diff = -disc_diff - 2 * flipped - 1;
                if empties == 5 {
                    let sq1_0 = end.end_move_list[0].succ;
                    let sq2_0 = end.end_move_list[sq1_0 as usize].succ;
                    let sq3_0 = end.end_move_list[sq2_0 as usize].succ;
                    let sq4_0 = end.end_move_list[sq3_0 as usize].succ;
                    ev = -solve_four_empty(end, new_opp_bits, end.bb_flips, sq1_0,
                                          sq2_0, sq3_0, sq4_0, -beta, -alpha,
                                          new_disc_diff, 1,  &mut search_state)
                } else {
                    end.region_parity ^= holepar_0;
                    ev = -solve_parity(end, new_opp_bits, end.bb_flips, -beta, -alpha,
                                      oppcol, empties - 1,
                                      new_disc_diff, 1,  &mut search_state, &mut stable_state);
                    end.region_parity ^= holepar_0
                }
                end. end_move_list[old_sq as usize].succ = sq;
                if ev > score {
                    if ev > alpha {
                        if ev >= beta { end.  best_move = sq; return ev }
                        alpha = ev
                    }
                    score = ev;
                    best_sq = sq
                }
            }
        }
        old_sq = sq;
        sq = end. end_move_list[sq as usize].succ
    }
    if score == -12345678 {
        if pass_legal == 0 {
            if disc_diff > 0 { return disc_diff + empties }
            if disc_diff < 0 { return disc_diff - empties }
            return 0
        } else {
            return -solve_parity(end, opp_bits, my_bits, -beta, -alpha, oppcol,
                                 empties, -disc_diff, 0,  &mut search_state, &mut stable_state)
        }
    }
    end.best_move = best_sq;
    return score;
}
/*
   SETUP_END
   Prepares the endgame solver for a new game.
   This means clearing a few status fields.
*/
pub fn setup_end(flip_stack_: &mut FlipStack, end_g: &mut End) {
    let mut last_mean: f64 = 0.;
    let mut last_sigma: f64 = 0.;
    let mut ff_threshold: [f64; 61] = [0.; 61];
    let mut prelim_threshold: [[f64; 64]; 61] = [[0.; 64]; 61];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    static dir_shift: [i32; 8] =
        [1, -(1), 7,
            -(7), 8, -(8),
            9, -(9)];
    let end = end_g;
    end.earliest_wld_solve = 0;
    end.earliest_full_solve = 0;
    end.full_output_mode = 1;
    /* Calculate the neighborhood masks */
    i = 1;
    while i <= 8 {
        j = 1;
        while j <= 8 {
            /* Create the neighborhood mask for the square POS */
            let pos = 10 * i + j;
            let shift =
                8 * (i - 1) +
                    (j - 1);
            let mut k: u32 = 0;
            end.neighborhood_mask[pos as usize].low = 0;
            end.neighborhood_mask[pos as usize].high = 0;
            k = 0;
            while k < 8 {
                if dir_mask[pos as usize] & (1) << k != 0 {
                    let neighbor =
                        (shift + dir_shift[k as usize]) as u32;
                    if neighbor < 32 {
                        end.neighborhood_mask[pos as usize].low |=
                            ((1) << neighbor) as u32
                    } else {
                        end.neighborhood_mask[pos as usize].high |=
                            ((1) <<
                                neighbor.wrapping_sub(32)) as
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
    i = 0;
    while i <= 60 {
        end.ff_mob_factor[i as usize] = 460;
        i += 1
    }
    i = 0;
    while i <= 60 {
        ff_threshold[i as usize] = 0.45f64;
        i += 1
    }
    /* Calculate the alpha thresholds for using fastest-first for
       each #empty and shallow search depth. */
    j = 0; /* Infinity in disc difference */
    while j <= 8 {
        last_sigma = 100.0f64;
        last_mean = 0.0f64;
        i = 60;
        while i >= 0 {
            if END_STATS_AVAILABLE[i as usize][j as usize] != 0 {
                last_mean =
                    END_MEAN[i as usize][j as usize] as f64;
                last_sigma =
                    ff_threshold[i as usize] *
                        END_SIGMA[i as usize][j as usize] as f64
            }
            // fast_first_mean[i as usize][j as usize] = last_mean;
            // fast_first_sigma[i as usize][j as usize] = last_sigma;
            prelim_threshold[i as usize][j as usize] = last_mean + last_sigma;
            i -= 1
        }
        j += 1
    }
    j = 8 + 1;
    while j < 64 {
        i = 0;
        while i <= 60 {
            prelim_threshold[i as usize][j as usize] =
                prelim_threshold[i as usize][8];
            i += 1
        }
        j += 1
    }
    i = 0;
    while i <= 60 {
        j = 0;
        while j < 64 {
            end.fast_first_threshold[i as usize][j as usize] =
                ceil(prelim_threshold[i as usize][j as usize] * 128.0f64) as
                    i32;
            j += 1
        }
        i += 1
    };
}


fn solve_parity_hash(end:&mut End, my_bits: BitBoard,
                                opp_bits: BitBoard,
                                mut alpha: i32,
                                mut beta: i32,
                                color: i32,
                                empties: i32,
                                disc_diff: i32,
                                pass_legal: i32,
                            mut search_state: &mut SearchState,
                            mut board_state: &mut BoardState,
                            mut hash_state: &mut HashState,
                            mut stable_state: &mut StableState,)
                                -> i32 {
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut score = -(12345678);
    let oppcol = 0 + 2 - color;
    let in_alpha = alpha;
    let mut ev: i32 = 0;
    let mut flipped: i32 = 0;
    let mut new_disc_diff: i32 = 0;
    let mut sq = 0;
    let mut old_sq = 0;
    let mut best_sq = 0;
    let mut parity_mask: u32 = 0;
    let mut entry = HashEntry::new();
    search_state.nodes.lo = search_state.nodes.lo.wrapping_add(1);
    find_hash(&mut entry, 1, &mut hash_state);
    if entry.draft as i32 == empties &&
        entry.selectivity as i32 == 0 &&
        valid_move(entry.move_0[0], color, &board_state.board) != 0 &&
        entry.flags as i32 & 16 != 0 &&
        (entry.flags as i32 & 4 != 0 ||
            entry.flags as i32 & 1 != 0 &&
                entry.eval >= beta ||
            entry.flags as i32 & 2 != 0 &&
                entry.eval <= alpha) {
        end.  best_move = entry.move_0[0] as i8;
        return entry.eval
    }
    /* Check for stability cutoff */
    if alpha >= stability_threshold[empties as usize] {
        let mut stability_bound: i32 = 0;
        stability_bound =
            64 -
                2 *
                    count_edge_stable(oppcol, opp_bits, my_bits, &mut stable_state);
        if stability_bound <= alpha { return alpha }
        stability_bound =
            64 -
                2 * count_stable(oppcol, opp_bits, my_bits, &mut stable_state);
        if stability_bound < beta {
            beta = stability_bound + 1
        }
        if stability_bound <= alpha { return alpha }
    }
    /* Odd parity. */
    parity_mask = end.region_parity;
    if end.region_parity != 0 {
        /* Is there any region with odd parity? */
        old_sq = 0;
        sq = end. end_move_list[old_sq as usize].succ;
        while sq != 99 {
            let holepar = quadrant_mask[sq as usize] as u32;
            if holepar & parity_mask != 0 {
                flipped = TestFlips_wrapper(end,sq, my_bits, opp_bits, );
                if flipped != 0 {
                    new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
                    new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
                    end.region_parity ^= holepar;
                    end. end_move_list[old_sq as usize].succ =
                        end. end_move_list[sq as usize].succ;
                    new_disc_diff =
                        -disc_diff - 2 * flipped -
                            1;
                    ev =
                        -solve_parity(end, new_opp_bits, end.bb_flips, -beta, -alpha,
                                      oppcol, empties - 1,
                                      new_disc_diff, 1,  &mut search_state, &mut stable_state);
                    end.region_parity ^= holepar;
                    end. end_move_list[old_sq as usize].succ = sq;
                    if ev > score {
                        score = ev;
                        if ev > alpha {
                            if ev >= beta {
                                end.  best_move = sq;
                                add_hash(&mut hash_state, 1, score, end.  best_move,
                                         16 | 1,
                                         empties, 0);
                                return score
                            }
                            alpha = ev
                        }
                        best_sq = sq
                    }
                }
            }
            old_sq = sq;
            sq = end. end_move_list[sq as usize].succ
        }
    }
    /* Even parity. */
    parity_mask = !parity_mask;
    old_sq = 0;
    sq = end. end_move_list[old_sq as usize].succ;
    while sq != 99 {
        let holepar_0 = quadrant_mask[sq as usize] as u32;
        if holepar_0 & parity_mask != 0 {
            flipped = TestFlips_wrapper(end,sq, my_bits, opp_bits, );
            if flipped != 0 {
                new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
                new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
                end.region_parity ^= holepar_0;
                end. end_move_list[old_sq as usize].succ =
                    end. end_move_list[sq as usize].succ;
                new_disc_diff =
                    -disc_diff - 2 * flipped -
                        1;
                ev =
                    -solve_parity(end, new_opp_bits,end.bb_flips, -beta, -alpha,
                                  oppcol, empties - 1,
                                  new_disc_diff, 1, &mut search_state, &mut stable_state);
                end.region_parity ^= holepar_0;
                end. end_move_list[old_sq as usize].succ = sq;
                if ev > score {
                    score = ev;
                    if ev > alpha {
                        if ev >= beta {
                            end.  best_move = sq;
                            add_hash(&mut hash_state,1, score, end.best_move,
                                     16 | 1,
                                     empties, 0);
                            return score
                        }
                        alpha = ev
                    }
                    best_sq = sq
                }
            }
        }
        old_sq = sq;
        sq = end. end_move_list[sq as usize].succ
    }
    if score == -(12345678) {
        if pass_legal == 0 {
            if disc_diff > 0 { return disc_diff + empties }
            if disc_diff < 0 { return disc_diff - empties }
            return 0
        } else {
            hash_state.hash1 ^= hash_state.hash_flip_color1;
            hash_state.hash2 ^= hash_state.hash_flip_color2;
            score =
                -solve_parity_hash(end, opp_bits, my_bits, -beta, -alpha, oppcol,
                                   empties, -disc_diff, 0, search_state
                                   ,board_state
                                   ,hash_state
                                   ,stable_state);
            hash_state.hash1 ^= hash_state.hash_flip_color1;
            hash_state.hash2 ^= hash_state.hash_flip_color2
        }
    } else {
        end.  best_move = best_sq;
        if score > in_alpha {
            add_hash(&mut hash_state,1, score, end.  best_move,
                     16 | 4, empties,
                     0);
        } else {
            add_hash(&mut hash_state,1, score, end.  best_move,
                     16 | 2, empties,
                     0);
        }
    }
    return score;
}

fn solve_parity_hash_high(end: &mut End, my_bits: BitBoard,
                                     opp_bits: BitBoard,
                                     mut alpha: i32,
                                     mut beta: i32,
                                     color: i32,
                                     empties: i32,
                                     disc_diff: i32,
                                     pass_legal: i32,
                                 mut flip_stack_: &mut FlipStack,
                                 mut search_state: &mut SearchState,
                                 mut board_state: &mut BoardState,
                                 mut hash_state: &mut HashState,
                                 mut stable_state: &mut StableState,)
                                     -> i32 {
    /* Move bonuses without and with parity for the squares.
       These are only used when sorting moves in the 9-12 empties
       range and were automatically tuned by OPTIMIZE. */
    static move_bonus: [[u8; 128]; 2] =
        [[0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            24 as u8,
            1 as u8,
            0 as u8,
            25 as u8,
            25 as u8,
            0 as u8,
            1 as u8,
            24 as u8,
            0 as u8,
            0 as u8,
            1 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            1 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            25 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            25 as u8,
            0 as u8,
            0 as u8,
            25 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            25 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            1 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            1 as u8,
            0 as u8,
            0 as u8,
            24 as u8,
            1 as u8,
            0 as u8,
            25 as u8,
            25 as u8,
            0 as u8,
            1 as u8,
            24 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8,
            0 as u8],
            [0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                128 as u8,
                86 as u8,
                122 as u8,
                125 as u8,
                125 as u8,
                122 as u8,
                86 as u8,
                128 as u8,
                0 as u8,
                0 as u8,
                86 as u8,
                117 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                117 as u8,
                86 as u8,
                0 as u8,
                0 as u8,
                122 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                122 as u8,
                0 as u8,
                0 as u8,
                125 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                125 as u8,
                0 as u8,
                0 as u8,
                125 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                125 as u8,
                0 as u8,
                0 as u8,
                122 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                122 as u8,
                0 as u8,
                0 as u8,
                86 as u8,
                117 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                128 as u8,
                117 as u8,
                86 as u8,
                0 as u8,
                0 as u8,
                128 as u8,
                86 as u8,
                122 as u8,
                125 as u8,
                125 as u8,
                122 as u8,
                86 as u8,
                128 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8,
                0 as u8]];
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut best_new_my_bits = BitBoard{high: 0, low: 0,};
    let mut best_new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut i: i32 = 0;
    let mut score: i32 = 0;
    let in_alpha = alpha;
    let oppcol = 0 + 2 - color;
    let mut flipped: i32 = 0;
    let mut best_flipped: i32 = 0;
    let mut new_disc_diff: i32 = 0;
    let mut ev: i32 = 0;
    let mut hash_move = 0;
    let mut moves: i32 = 0;
    let mut parity: i32 = 0;
    let mut best_value: i32 = 0;
    let mut best_index: i32 = 0;
    let mut pred = 0;
    let mut succ = 0;
    let mut sq = 0;
    let mut old_sq = 0;
    let mut best_sq = 0;
    let mut move_order: [i8; 64] = [0; 64];
    let mut goodness: [i32; 64] = [0; 64];
    let mut diff1: u32 = 0;
    let mut diff2: u32 = 0;
    let mut entry = HashEntry::new();
    search_state.nodes.lo = search_state.nodes.lo.wrapping_add(1);
    hash_move = -1;
    find_hash(&mut entry, 1, &mut hash_state);
    if entry.draft as i32 == empties {
        if entry.selectivity as i32 == 0 &&
            entry.flags as i32 & 16 != 0 &&
            valid_move(entry.move_0[0], color, &board_state.board) != 0
            &&
            (entry.flags as i32 & 4 != 0 ||
                entry.flags as i32 & 1 != 0 &&
                    entry.eval >= beta ||
                entry.flags as i32 & 2 != 0 &&
                    entry.eval <= alpha) {
            end.  best_move = entry.move_0[0];
            return entry.eval
        }
    }
    /* Check for stability cutoff */
    if alpha >= stability_threshold[empties as usize] {
        let mut stability_bound: i32 = 0;
        stability_bound =
            64 -
                2 *
                    count_edge_stable(oppcol, opp_bits, my_bits, &mut stable_state);
        if stability_bound <= alpha { return alpha }
        stability_bound =
            64 -
                2 * count_stable(oppcol, opp_bits, my_bits, &mut stable_state);
        if stability_bound < beta {
            beta = stability_bound + 1
        }
        if stability_bound <= alpha { return alpha }
    }
    /* Calculate goodness values for all moves */
    moves = 0;
    best_value = -(12345678);
    best_index = 0;
    best_flipped = 0;
    old_sq = 0;
    sq = end. end_move_list[old_sq as usize].succ;
    while sq != 99 {
        flipped = TestFlips_wrapper(end,sq, my_bits, opp_bits, );
        if flipped != 0 {
            search_state.nodes.lo = search_state.nodes.lo.wrapping_add(1);
            new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
            new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
            end. end_move_list[old_sq as usize].succ =
                end. end_move_list[sq as usize].succ;
            if quadrant_mask[sq as usize] as u32 & end.region_parity != 0 {
                parity = 1
            } else { parity = 0 }
            goodness[moves as usize] =
                move_bonus[parity as usize][sq as usize] as i32;
            if sq == hash_move {
                goodness[moves as usize] += 128
            }
            goodness[moves as usize] -=
                weighted_mobility(new_opp_bits, end.bb_flips);
            if goodness[moves as usize] > best_value {
                best_value = goodness[moves as usize];
                best_index = moves;
                best_new_my_bits = end.bb_flips;
                best_new_opp_bits = new_opp_bits;
                best_flipped = flipped
            }
            end. end_move_list[old_sq as usize].succ = sq;
            move_order[moves as usize] = sq;
            moves += 1
        }
        old_sq = sq;
        sq = end. end_move_list[sq as usize].succ
    }
    /* Maybe there aren't any legal moves */
    if moves == 0 {
        /* I have to pass */
        if pass_legal == 0 {
            /* Last move also pass, game over */
            if disc_diff > 0 { return disc_diff + empties }
            if disc_diff < 0 { return disc_diff - empties }
            return 0
        } else {
            /* Opponent gets the chance to play */
            hash_state.hash1 ^= hash_state.hash_flip_color1;
            hash_state.hash2 ^= hash_state.hash_flip_color2;
            score =
                -solve_parity_hash_high(end, opp_bits, my_bits, -beta, -alpha,
                                        oppcol, empties, -disc_diff,
                                        0, flip_stack_
                                        ,search_state
                                        ,board_state
                                        ,hash_state
                                        ,stable_state);
            hash_state.hash1 ^= hash_state.hash_flip_color1;
            hash_state.hash2 ^= hash_state.hash_flip_color2;
            return score
        }
    }
    /* Try move with highest goodness value */
    sq = move_order[best_index as usize];
    let (_, hash_update1_, hash_update2_) = DoFlips_hash(sq, color, &mut board_state.board, &mut hash_state.hash_flip1, &mut hash_state.hash_flip2, &mut flip_stack_);
    board_state.board[sq as usize] = color;
    diff1 = hash_update1_ ^ hash_state.hash_put_value1[color as usize][sq as usize];
    diff2 = hash_update2_ ^ hash_state.hash_put_value2[color as usize][sq as usize];
    hash_state.hash1 ^= diff1;
    hash_state.hash2 ^= diff2;
    end.region_parity ^= quadrant_mask[sq as usize] as u32;
    pred = end. end_move_list[sq as usize].pred;
    succ = end. end_move_list[sq as usize].succ;
    end. end_move_list[pred as usize].succ = succ;
    end. end_move_list[succ as usize].pred = pred;
    new_disc_diff =
        -disc_diff - 2 * best_flipped - 1;
    if empties <= 8 + 1 {
        score =
            -solve_parity_hash(end, best_new_opp_bits, best_new_my_bits, -beta,
                               -alpha, oppcol, empties - 1,
                               new_disc_diff, 1, search_state
                               ,board_state
                               ,hash_state
                               ,stable_state)
    } else {
        score =
            -solve_parity_hash_high(end, best_new_opp_bits, best_new_my_bits,
                                    -beta, -alpha, oppcol,
                                    empties - 1, new_disc_diff,
                                    1, flip_stack_
                                    ,search_state
                                    ,board_state
                                    ,hash_state
                                    ,stable_state)
    }
    flip_stack_.UndoFlips(&mut board_state.board, best_flipped, oppcol);
    hash_state.hash1 ^= diff1;
    hash_state.hash2 ^= diff2;
    board_state.board[sq as usize] = 1;
    end.region_parity ^= quadrant_mask[sq as usize] as u32;
    end. end_move_list[pred as usize].succ = sq;
    end. end_move_list[succ as usize].pred = sq;
    best_sq = sq;
    if score > alpha {
        if score >= beta {
            end.  best_move = best_sq;
            add_hash(&mut hash_state,1, score, end.  best_move,
                     16 | 1, empties,
                     0);
            return score
        }
        alpha = score
    }
    /* Play through the rest of the moves */
    move_order[best_index as usize] = move_order[0];
    goodness[best_index as usize] = goodness[0];
    i = 1;
    while i < moves {
        let mut j: i32 = 0;
        best_value = goodness[i as usize];
        best_index = i;
        j = i + 1;
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
        flipped = TestFlips_wrapper(end,sq, my_bits, opp_bits, );
        new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
        new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
        let (_, hash_update1_, hash_update2_) = DoFlips_hash(
            sq, color, &mut board_state.board, &mut hash_state.hash_flip1, &mut hash_state.hash_flip2, &mut flip_stack_);
        board_state.board[sq as usize] = color;
        diff1 = hash_update1_ ^ hash_state.hash_put_value1[color as usize][sq as usize];
        diff2 = hash_update2_ ^ hash_state.hash_put_value2[color as usize][sq as usize];
        hash_state.hash1 ^= diff1;
        hash_state.hash2 ^= diff2;
        end.region_parity ^= quadrant_mask[sq as usize] as u32;
        pred = end. end_move_list[sq as usize].pred;
        succ = end. end_move_list[sq as usize].succ;
        end. end_move_list[pred as usize].succ = succ;
        end. end_move_list[succ as usize].pred = pred;
        new_disc_diff =
            -disc_diff - 2 * flipped - 1;
        if empties <= 8 {
            /* Fail-high for opp is likely. */
            ev =
                -solve_parity_hash(end,new_opp_bits, end.bb_flips, -beta, -alpha,
                                   oppcol, empties - 1,
                                   new_disc_diff, 1, search_state
                                   ,board_state
                                   ,hash_state
                                   ,stable_state)
        } else {
            ev =
                -solve_parity_hash_high(end, new_opp_bits, end.bb_flips, -beta, -alpha,
                                        oppcol, empties - 1,
                                        new_disc_diff, 1, flip_stack_
                                        ,search_state
                                        ,board_state
                                        ,hash_state
                                        ,stable_state)
        }
        end.region_parity ^= quadrant_mask[sq as usize] as u32;
        flip_stack_.UndoFlips(&mut board_state.board, flipped, oppcol);
        hash_state.hash1 ^= diff1;
        hash_state.hash2 ^= diff2;
        board_state.board[sq as usize] = 1;
        end. end_move_list[pred as usize].succ = sq;
        end. end_move_list[succ as usize].pred = sq;
        if ev > score {
            score = ev;
            if ev > alpha {
                if ev >= beta {
                    end.  best_move = sq;
                    add_hash(&mut hash_state,1, score, end.  best_move,
                             16 | 1, empties,
                             0);
                    return score
                }
                alpha = ev
            }
            best_sq = sq
        }
        i += 1
    }
    end.  best_move = best_sq;

    let flags = 16 | if score > in_alpha { 4 } else { 2 };
    add_hash(&mut hash_state, 1, score, end.best_move,
             flags, empties, 0);
    return score;
}

/*
  END_SOLVE
  The search itself. Assumes relevant data structures have been set up with
  PREPARE_TO_SOLVE(). Returns difference between disc count for
  COLOR and disc count for the opponent of COLOR.
*/
fn end_solve(end:&mut End, my_bits: BitBoard, opp_bits: BitBoard,
                    alpha: i32, beta: i32,
                    color: i32,
                    empties: i32,
                    discdiff: i32,
                    prevmove: i32
                    ,flip_stack: &mut FlipStack
                    ,search_state: &mut SearchState
                    ,board_state: &mut BoardState
                    ,hash_state: &mut HashState
                    ,stable_state: &mut StableState

) -> i32 {
    let mut result: i32 = 0;
    if empties <= 8 {
        result =
            solve_parity(end, my_bits, opp_bits, alpha, beta, color, empties,
                         discdiff, prevmove, search_state, stable_state)
    } else {
        result =
            solve_parity_hash_high(end, my_bits, opp_bits, alpha, beta, color,
                                   empties, discdiff, prevmove, flip_stack
                                   ,search_state
                                   ,board_state
                                   ,hash_state
                                   ,stable_state)
    }
    return result;
}


/*
  UPDATE_BEST_LIST
*/
pub fn update_best_list<FE: FrontEnd>(best_list: &mut [i8; 4],
                           move_0: i8,
                           best_list_index: i32,
                           best_list_length: &mut i32,
                           mut verbose: i32) {
    use engine_traits::Offset;
    verbose = 0;
    if verbose != 0 {
        FE::before_update_best_list_verbose(best_list, move_0, best_list_index, *best_list_length)
    }
    if best_list_index < *best_list_length {
        let mut i = best_list_index;
        while i >= 1 {
            *(best_list as &mut[_]).offset(i as isize) =
                *best_list.offset((i - 1) as isize);
            i -= 1
        }
    } else {
        let mut i = 3;
        while i >= 1 {
            *(best_list as &mut[_]).offset(i as isize) =
                *best_list.offset((i - 1) as isize);
            i -= 1
        }
        if *best_list_length < 4 { *best_list_length += 1 }
    }
    best_list[0] = move_0;
    if verbose != 0 {
        FE::after_update_best_list_verbose(best_list);
    };
}


/*
  END_TREE_SEARCH
  Plain nega-scout with fastest-first move ordering.
*/
fn end_tree_search<FE: FrontEnd>(end: &mut End,level: i32,
                                            max_depth: i32,
                                            my_bits: BitBoard,
                                            opp_bits: BitBoard,
                                            side_to_move: i32,
                                            alpha: i32,
                                            mut beta: i32,
                                            selectivity: i32,
                                            selective_cutoff: &mut i32,
                                            void_legal: i32, echo: i32,
                                        mut flip_stack_: &mut FlipStack,
                                        mut search_state: &mut SearchState,
                                        mut board_state: &mut BoardState,
                                        mut hash_state: &mut HashState,
                                        mut g_timer: &mut Timer,
                                        mut midgame_state: &mut MidgameState,
                                        mut coeff_state: &mut CoeffState,
                                        mut moves_state: &mut MovesState,
                                        mut stable_state: &mut StableState,
                                        mut prob_cut: &mut ProbCut
)
                                            -> i32 {
    let mut node_val: f64 = 0.;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut empties: i32 = 0;
    let mut disk_diff: i32 = 0;
    let mut previous_move: i32 = 0;
    let mut result: i32 = 0;
    let mut curr_val: i32 = 0;
    let mut best: i32 = 0;
    let mut move_0 = 0;
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
    let mut best_list = [0; 4];
    let mut entry = HashEntry::new();
    let mut mid_entry = HashEntry::new();
    let mut stability_bound: i32 = 0;
    if level == 0 {
        FE::end_tree_search_level_0_report(alpha, beta);
    }
    remains = max_depth - level;
    *selective_cutoff = 0;
    /* Always (almost) check for stability cutoff in this region of search */
    if alpha >= 24 {
        stability_bound =
            64 -
                2 *
                    count_edge_stable(0 + 2 -
                                          side_to_move, opp_bits, my_bits, &mut stable_state);
        if stability_bound <= alpha {
            board_state.pv_depth[level as usize] = level;
            return alpha
        }
        stability_bound =
            64 -
                2 *
                    count_stable(0 + 2 -
                                     side_to_move, opp_bits, my_bits, &mut stable_state);
        if stability_bound < beta {
            beta = stability_bound + 1
        }
        if stability_bound <= alpha {
            board_state.pv_depth[level as usize] = level;
            return alpha
        }
    }
    /* Check if the low-level code is to be invoked */
    my_discs = board_state.piece_count[side_to_move as usize][moves_state.disks_played as usize];
    opp_discs =
        board_state.piece_count[(0 + 2 - side_to_move) as
            usize][moves_state.disks_played as usize];
    empties = 64 - my_discs - opp_discs;
    if remains <= 12 {
        disk_diff = my_discs - opp_discs;
        if void_legal != 0 {
            /* Is PASS legal or was last move a pass? */
            previous_move = 44
        } else {
            previous_move = 0
        } /* d4, of course impossible */
        prepare_to_solve(&board_state.board, end);
        result =
            end_solve(end, my_bits, opp_bits, alpha, beta, side_to_move, empties,
                      disk_diff, previous_move, flip_stack_
                      ,search_state
                      ,board_state
                      ,hash_state
                      ,stable_state);
        board_state.pv_depth[level as usize] = level + 1;
        board_state.pv[level as usize][level as usize] = end.best_move;
        if level == 0 && search_state.get_ponder_move() == 0 {
            FE::end_tree_search_level_0_ponder_0_report(alpha, beta, result, end.best_move as i32)
        }
        return result
    }
    /* Otherwise normal search */
    search_state.nodes.lo = search_state.nodes.lo.wrapping_add(1);
    use_hash = 1;
    if use_hash != 0 {
        /* Check for endgame hash table move */
        find_hash(&mut entry, 1, &mut hash_state);
        if entry.draft as i32 == remains &&
            entry.selectivity as i32 <= selectivity &&
            valid_move(entry.move_0[0],
                       side_to_move, &board_state.board) != 0 &&
            entry.flags as i32 & 16 != 0 &&
            (entry.flags as i32 & 4 != 0 ||
                entry.flags as i32 & 1 != 0 &&
                    entry.eval >= beta ||
                entry.flags as i32 & 2 != 0 &&
                    entry.eval <= alpha) {
            board_state.pv[level as usize][level as usize] =
                entry.move_0[0];
            board_state.pv_depth[level as usize] = level + 1;
            if level == 0 && search_state.get_ponder_move() == 0 {
                FE::end_tree_search_output_some_stats(&entry);
            }
            if entry.selectivity as i32 > 0 {
                *selective_cutoff = 1
            }
            return entry.eval
        }
        hash_hit =
            (entry.draft as i32 != 0) as i32;
        /* If not any such found, check for a midgame hash move */
        find_hash(&mut mid_entry, 0, &mut hash_state);
        if mid_entry.draft as i32 != 0 &&
            mid_entry.flags as i32 & 8 != 0 {
            if level <= 4 ||
                mid_entry.flags as i32 &
                    (4 | 1) != 0 {
                /* Give the midgame move full priority if we're are the root
                   of the tree, no endgame hash move was found and the position
                   isn't in the wipeout zone. */
                if level == 0 && hash_hit == 0 &&
                    mid_entry.eval < 60 * 128
                {
                    entry = mid_entry;
                    hash_hit = 1
                }
            }
        }
    }
    /* Use endgame multi-prob-cut to selectively prune the tree */
    if 1 != 0 && level > 2 &&
        selectivity > 0 {
        let mut cut: i32 = 0;
        cut = 0;
        while cut < prob_cut.use_end_cut[moves_state.disks_played as usize] {
            let shallow_remains =
                prob_cut.end_mpc_depth[moves_state.disks_played as usize][cut as usize];
            let mpc_bias =
                ceil(END_MEAN[moves_state.disks_played as usize][shallow_remains as usize]
                    as f64 * 128.0f64) as i32;
            let mpc_window =
                ceil(END_SIGMA[moves_state.disks_played as
                    usize][shallow_remains as usize] as
                    f64 * end_percentile[selectivity as usize]
                    * 128.0f64) as i32;
            let beta_bound =
                128 * beta + mpc_bias + mpc_window;
            let alpha_bound =
                128 * alpha + mpc_bias - mpc_window;
            let shallow_val =
                tree_search::<FE>(level, level + shallow_remains, side_to_move,
                            alpha_bound, beta_bound, use_hash,
                            0, void_legal, echo,  &mut moves_state ,
                                  &mut search_state ,
                                  &mut board_state ,
                                  &mut hash_state,
                                  &mut flip_stack_,
                                  &mut coeff_state,
                                  &mut prob_cut, &mut g_timer, &mut midgame_state);
            if shallow_val >= beta_bound {
                if use_hash != 0 {
                    add_hash(&mut hash_state, 1, alpha,
                             board_state.pv[level as usize][level as usize] as i8,
                             16 | 1, remains,
                             selectivity);
                }
                *selective_cutoff = 1;
                return beta
            }
            if shallow_val <= alpha_bound {
                if use_hash != 0 {
                    add_hash(&mut hash_state, 1, beta,
                             board_state.pv[level as usize][level as usize] as i8,
                             16 | 2, remains,
                             selectivity);
                }
                *selective_cutoff = 1;
                return alpha
            }
            cut += 1
        }
    }
    /* Determine the depth of the shallow search used to find
       achieve good move sorting */
    if remains >= 15 {
        if remains >= 20 {
            if remains >= 24 {
                if remains >= 30 {
                    pre_depth = 6
                } else { pre_depth = 4 }
            } else { pre_depth = 3 }
        } else { pre_depth = 2 }
    } else { pre_depth = 1 }
    if level == 0 {
        /* Deeper pre-search from the root */
        pre_depth += 2;
        if pre_depth % 2 == 1 {
            /* Avoid odd depths from the root */
            pre_depth += 1
        }
    }
    /* The nega-scout search */
    exp_depth = remains;
    first = 1;
    best = -(12345678);
    pre_search_done = 0;
    curr_alpha = alpha;
    /* Initialize the move list and check the hash table move list */
    moves_state.move_count[moves_state.disks_played as usize] = 0;
    best_list_length = 0;
    i = 0;
    while i < 4 {
        best_list[i as usize] = 0;
        i += 1
    }
    if hash_hit != 0 {
        i = 0;
        while i < 4 {
            if valid_move(entry.move_0[i as usize], side_to_move, &board_state.board) != 0 {
                let fresh0 = best_list_length;
                best_list_length = best_list_length + 1;
                best_list[fresh0 as usize] = entry.move_0[i as usize];
                /* Check for ETC among the hash table moves */
                if use_hash != 0 &&
                    make_move(side_to_move, entry.move_0[i as usize],
                              1, &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ ) != 0 {
                    let mut etc_entry =
                        HashEntry{key1: 0,
                            key2: 0,
                            eval: 0,
                            move_0: [0; 4],
                            draft: 0,
                            selectivity: 0,
                            flags: 0,};
                    find_hash(&mut etc_entry, 1, &mut hash_state);
                    if etc_entry.flags as i32 & 16 != 0
                        &&
                        etc_entry.draft as i32 ==
                            empties - 1 &&
                        etc_entry.selectivity as i32 <= selectivity
                        &&
                        etc_entry.flags as i32 &
                            (2 | 4) != 0 &&
                        etc_entry.eval <= -beta {
                        /* Immediate cutoff from this move, move it up front */
                        j = best_list_length - 1;
                        while j >= 1 {
                            best_list[j as usize] =
                                best_list[(j - 1) as usize];
                            j -= 1
                        }
                        best_list[0] =
                            entry.move_0[i as usize]
                    }
                    let move_0 = entry.move_0[i as usize];
                    {
                        unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
                    };
                }
            }
            i += 1
        }
    }
    move_index = 0;
    best_list_index = 0;
    loop  {
        let mut child_selective_cutoff: i32 = 0;
        let mut new_my_bits = BitBoard{high: 0, low: 0,};
        let mut new_opp_bits = BitBoard{high: 0, low: 0,};
        /* Use results of shallow searches to determine the move order */
        if best_list_index < best_list_length {
            move_0 = best_list[best_list_index as usize];
            moves_state.move_count[moves_state.disks_played as usize] += 1
        } else {
            if pre_search_done == 0 {
                let mut shallow_index: i32 = 0;
                pre_search_done = 1;
                threshold =
                    if (60 * 128) <
                        128 * alpha +
                            end.fast_first_threshold[moves_state.disks_played as
                                usize][pre_depth as
                                usize] {
                        (60) * 128
                    } else {
                        (128 * alpha) +
                            end.fast_first_threshold[moves_state.disks_played as
                                usize][pre_depth as
                                usize]
                    };
                shallow_index = 0;
                while shallow_index < 60 {
                    let mut already_checked: i32 = 0;
                    move_0 =
                        search_state.sorted_move_order[moves_state.disks_played as
                            usize][shallow_index as usize] as i8;
                    already_checked = 0;
                    j = 0;
                    while j < best_list_length {
                        if move_0 == best_list[j as usize] {
                            already_checked = 1
                        }
                        j += 1
                    }
                    if already_checked == 0 &&
                        board_state.board[move_0 as usize] == 1 &&
                        TestFlips_wrapper(end,move_0, my_bits, opp_bits) >
                            0 {
                        new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
                        new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
                        make_move(side_to_move, move_0, 1 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
                        curr_val = 0;
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
                            find_hash(&mut etc_entry_0, 1, &mut hash_state);
                            if etc_entry_0.flags as i32 &
                                16 != 0 &&
                                etc_entry_0.draft as i32 ==
                                    empties - 1 {
                                curr_val += 384;
                                if etc_entry_0.selectivity as i32 <=
                                    selectivity {
                                    if etc_entry_0.flags as i32 &
                                        (2 |
                                            4) != 0 &&
                                        etc_entry_0.eval <= -beta {
                                        curr_val = 10000000
                                    }
                                    if etc_entry_0.flags as i32 &
                                        1 != 0 &&
                                        etc_entry_0.eval >= -alpha {
                                        curr_val -= 640
                                    }
                                }
                            }
                        }
                        /* Determine the midgame score. If it is worse than
                           alpha-8, a fail-high is likely so precision in that
                           range is not worth the extra nodes required. */
                        if curr_val != 10000000 {
                            curr_val -=
                                tree_search::<FE>(level + 1,
                                            level + pre_depth,
                                            0 +
                                                2 -
                                                side_to_move,
                                            -(12345678),
                                            (-alpha + 8) *
                                                128,
                                            1,
                                            1,
                                            1,echo,  &mut moves_state ,
                                                  &mut search_state ,
                                                  &mut board_state ,
                                                  &mut hash_state,
                                                  &mut flip_stack_,
                                                  &mut coeff_state,
                                                  &mut prob_cut, &mut g_timer, &mut midgame_state)
                        }
                        /* Make the moves which are highly likely to result in
                           fail-high in decreasing order of mobility for the
                           opponent. */
                        if curr_val > threshold ||
                            move_0 ==
                                mid_entry.move_0[0]
                        {
                            if curr_val >
                                60 * 128 {
                                curr_val +=
                                    2 * 1000000
                            } else { curr_val += 1000000 }
                            if curr_val < 10000000 {
                                mobility =
                                    bitboard_mobility(new_opp_bits, end.bb_flips);
                                if curr_val >
                                    2 *
                                        1000000 {
                                    curr_val -=
                                        2 *
                                            end.ff_mob_factor[(moves_state.disks_played -
                                                1)
                                                as usize] *
                                            mobility
                                } else {
                                    curr_val -=
                                        end.ff_mob_factor[(moves_state.disks_played -
                                            1)
                                            as usize] * mobility
                                }
                            }
                        }
                        {
                            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
                        };
                        search_state.evals[moves_state.disks_played as usize][move_0 as usize] =
                            curr_val;
                        moves_state.move_list[moves_state.disks_played as
                            usize][moves_state.move_count[moves_state.disks_played as usize]
                            as usize] = move_0;
                        moves_state.move_count[moves_state.disks_played as usize] += 1
                    }
                    shallow_index += 1
                }
            }
            if move_index == moves_state.move_count[moves_state.disks_played as usize] { break ; }
            move_0 =
                select_move(move_index,
                            moves_state.move_count[moves_state.disks_played as usize],
                    &mut search_state, &mut moves_state)
        }
        node_val = counter_value(&mut search_state.nodes);
        if node_val - g_timer.last_panic_check >= 250000.0f64 {
            /* Check for time abort */
            g_timer.last_panic_check = node_val;
             g_timer.check_panic_abort();
            /* Output status buffers if in interactive mode */
            if echo != 0 { FE::display_buffers(g_timer); }
            /* Check for events */
            if g_timer.is_panic_abort() != 0 || force_return != 0 {
                return -(27000)
            }
        }
        if level == 0 && search_state.get_ponder_move() == 0 {
            FE::end_tree_search_level_0_ponder_0_short_report(move_0, first);
        }
        make_move(side_to_move, move_0, use_hash,  &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        TestFlips_wrapper(end,move_0, my_bits, opp_bits);
        new_my_bits = end.bb_flips;
        new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
        new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
        update_pv = 0;
        if first != 0 {
            curr_val =
                -end_tree_search::<FE>(end,level + 1, level + exp_depth,
                                       new_opp_bits, new_my_bits,
                                       0 + 2 -
                                     side_to_move, -beta, -curr_alpha,
                                       selectivity, &mut child_selective_cutoff,
                                       1, echo
                                       , flip_stack_
                                       , search_state
                                       , board_state
                                       , hash_state
                                       , g_timer
                                       , midgame_state
                                       , coeff_state
                                       , moves_state
                                       , stable_state
                                       , prob_cut);
            best = curr_val;
            update_pv = 1;
            if level == 0 { end. best_end_root_move = move_0 }
        } else {
            curr_alpha = if best > curr_alpha { best } else { curr_alpha };
            curr_val =
                -end_tree_search::<FE>(end,level + 1, level + exp_depth,
                                       new_opp_bits, new_my_bits,
                                       0 + 2 -
                                     side_to_move,
                                       -(curr_alpha + 1),
                                       -curr_alpha, selectivity,
                                       &mut child_selective_cutoff,
                                       1, echo, flip_stack_
                                       , search_state
                                       , board_state
                                       , hash_state
                                       , g_timer
                                       , midgame_state
                                       , coeff_state
                                       , moves_state
                                       , stable_state
                                       , prob_cut);
            if curr_val > curr_alpha && curr_val < beta {
                if selectivity > 0 {
                    curr_val =
                        -end_tree_search::<FE>(end,level + 1,
                                               level + exp_depth, new_opp_bits,
                                               new_my_bits,
                                               0 + 2 -
                                             side_to_move, -beta,
                                               12345678, selectivity,
                                               &mut child_selective_cutoff,
                                               1, echo, flip_stack_
                                               , search_state
                                               , board_state
                                               , hash_state
                                               , g_timer
                                               , midgame_state
                                               , coeff_state
                                               , moves_state
                                               , stable_state
                                               , prob_cut)
                } else {
                    curr_val =
                        -end_tree_search::<FE>(end,level + 1,
                                               level + exp_depth, new_opp_bits,
                                               new_my_bits,
                                               0 + 2 -
                                             side_to_move, -beta, -curr_val,
                                               selectivity,
                                               &mut child_selective_cutoff,
                                               1, echo, flip_stack_
                                               , search_state
                                               , board_state
                                               , hash_state
                                               , g_timer
                                               , midgame_state
                                               , coeff_state
                                               , moves_state
                                               , stable_state
                                               , prob_cut)
                }
                if curr_val > best {
                    best = curr_val;
                    update_pv = 1;
                    if level == 0 && g_timer.is_panic_abort() == 0 &&
                        force_return == 0 {
                        end. best_end_root_move = move_0
                    }
                }
            } else if curr_val > best {
                best = curr_val;
                update_pv = 1;
                if level == 0 && g_timer.is_panic_abort() == 0 &&
                    force_return == 0 {
                    end. best_end_root_move = move_0
                }
            }
        }
        if best >= beta {
            /* The other children don't matter in this case. */
            *selective_cutoff = child_selective_cutoff
        } else if child_selective_cutoff != 0 {
            *selective_cutoff = 1
        }
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        if g_timer.is_panic_abort() != 0 || force_return != 0 {
            return -(27000)
        }
        if level == 0 && search_state.get_ponder_move() == 0 {
            /* Output some stats */
            FE::end_tree_search_output_some_second_stats(alpha, beta, curr_val, update_pv, move_index, echo)
        }
        if update_pv != 0 {
            update_best_list::<FE>(&mut best_list, move_0, best_list_index,
                             &mut best_list_length,
                             (level == 0) as i32);
            board_state.pv[level as usize][level as usize] = move_0;
            board_state.pv_depth[level as usize] =
                board_state.pv_depth[(level + 1) as usize];
            i = level + 1;
            while i < board_state.pv_depth[(level + 1) as usize] {
                board_state.pv[level as usize][i as usize] =
                    board_state.pv[(level + 1) as usize][i as usize];
                i += 1
            }
        }
        if best >= beta {
            /* Fail high */
            if use_hash != 0 {
                hash_state.add_hash_extended(1, best,
                                  &best_list,
                                  16 | 1,
                                  remains,
                                  if *selective_cutoff != 0 {
                                      selectivity
                                  } else { 0 });
            }
            return best
        }
        if best_list_index >= best_list_length && update_pv == 0 &&
            best_list_length < 4 {
            let fresh1 = best_list_length;
            best_list_length = best_list_length + 1;
            best_list[fresh1 as usize] = move_0
        }
        first = 0;
        move_index += 1;
        best_list_index += 1
    }
    if first == 0 {
        if use_hash != 0 {
            let mut flags = 16;
            if best > alpha {
                flags |= 4
            } else { flags |= 2 }
            hash_state.add_hash_extended(1, best, &best_list,
                              flags, remains,
                              if *selective_cutoff != 0 {
                                  selectivity
                              } else { 0 });
        }
        return best
    } else if void_legal != 0 {
        if use_hash != 0 {
            hash_state.hash1 ^= hash_state.hash_flip_color1;
            hash_state.hash2 ^= hash_state.hash_flip_color2
        }
        curr_val =
            -end_tree_search::<FE>(end ,level, max_depth, opp_bits, my_bits,
                                   0 + 2 -
                                 side_to_move, -beta, -alpha, selectivity,
                                   selective_cutoff, 0, echo, flip_stack_
                                   , search_state
                                   , board_state
                                   , hash_state
                                   , g_timer
                                   , midgame_state
                                   , coeff_state
                                   , moves_state
                                   , stable_state
                                   , prob_cut);
        if use_hash != 0 {
            hash_state.hash1 ^= hash_state.hash_flip_color1;
            hash_state.hash2 ^= hash_state.hash_flip_color2
        }
        return curr_val
    } else {
        board_state.pv_depth[level as usize] = level;
        my_discs = board_state.piece_count[side_to_move as usize][moves_state.disks_played as usize];
        opp_discs =
            board_state.piece_count[(0 + 2 - side_to_move)
                as usize][moves_state.disks_played as usize];
        disk_diff = my_discs - opp_discs;
        if my_discs > opp_discs {
            return 64 - 2 * opp_discs
        } else if my_discs == opp_discs {
            return 0
        } else { return -(64 - 2 * my_discs) }
    };
}


/*
  END_TREE_WRAPPER
  Wrapper onto END_TREE_SEARCH which applies the knowledge that
  the range of valid scores is [-64,+64].  Komi, if any, is accounted for.
*/
fn end_tree_wrapper<FE: FrontEnd>(end: &mut End, level: i32,
                                             max_depth: i32,
                                             side_to_move: i32,
                                             alpha: i32,
                                             beta: i32,
                                             selectivity: i32,
                                             void_legal: i32,
                                             echo:i32,
                                         flip_stack_: &mut FlipStack,
                                         search_state: &mut SearchState,
                                         board_state: &mut BoardState,
                                         hash_state: &mut HashState,
                                         g_timer: &mut Timer,
                                         midgame_state: &mut MidgameState,
                                         coeff_state: &mut CoeffState,
                                         moves_state: &mut MovesState,
                                         random_instance: &mut MyRandom,
                                         stable_state: &mut StableState,
                                         prob_cut: &mut ProbCut)
                                             -> i32 {
    let mut selective_cutoff: i32 = 0;
    let mut my_bits = BitBoard{high: 0, low: 0,};
    let mut opp_bits = BitBoard{high: 0, low: 0,};
    set_bitboards(&board_state.board, side_to_move, &mut my_bits,
                  &mut opp_bits);
    return end_tree_search::<FE>(end,level, max_depth, my_bits, opp_bits, side_to_move,
                           if alpha - end. komi_shift > -(64) {
                               (alpha) - end. komi_shift
                           } else { -(64) },
                           if beta - end. komi_shift < 64 {
                               (beta) - end. komi_shift
                           } else { 64 }, selectivity,
                                 &mut selective_cutoff, void_legal,
                                 echo, flip_stack_
                                 , search_state
                                 , board_state
                                 , hash_state
                                 , g_timer
                                 , midgame_state
                                 , coeff_state
                                 , moves_state
                                 , stable_state
                                 , prob_cut) + end. komi_shift;
}
/*
   FULL_EXPAND_PV
   Pad the PV with optimal moves in the low-level phase.
*/
fn full_expand_pv<FE: FrontEnd>(end: &mut End, mut side_to_move: i32,
                                           selectivity: i32, echo:i32,
                                mut flip_stack_: &mut FlipStack,
                                search_state: &mut SearchState,
                                mut board_state: &mut BoardState,
                                mut hash_state: &mut HashState,
                                g_timer: &mut Timer,
                                midgame_state: &mut MidgameState,
                                coeff_state: &mut CoeffState,
                                mut moves_state: &mut MovesState,
                                random_instance: &mut MyRandom,
                                stable_state: &mut StableState,
                                prob_cut: &mut ProbCut) {
    let mut i: i32 = 0;
    let mut pass_count: i32 = 0;
    let mut new_pv_depth: i32 = 0;
    let mut new_pv: [i8; 61] = [0; 61];
    let mut new_side_to_move: [i32; 61] = [0; 61];
    new_pv_depth = 0;
    pass_count = 0;
    while pass_count < 2 {
        let mut move_0 = 0;
        generate_all(side_to_move, &mut moves_state, search_state, &board_state.board);
        if moves_state.move_count[moves_state.disks_played as usize] > 0 {
            let empties =
                64 - disc_count(0, &board_state.board) -
                    disc_count(2, &board_state.board);
            end_tree_wrapper::<FE>(end, new_pv_depth, empties, side_to_move,
                             -(64), 64,
                             selectivity, 1, echo
            ,flip_stack_
            ,search_state
            ,board_state
            ,hash_state
            ,g_timer
            ,midgame_state
            ,coeff_state
            ,moves_state
            ,random_instance
            ,stable_state
            ,prob_cut
            );
            move_0 = board_state.pv[new_pv_depth as usize][new_pv_depth as usize];
            new_pv[new_pv_depth as usize] = move_0;
            new_side_to_move[new_pv_depth as usize] = side_to_move;
            make_move(side_to_move, move_0, 1 , moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
            new_pv_depth += 1
        } else {
            hash_state.hash1 ^= hash_state.hash_flip_color1;
            hash_state.hash2 ^= hash_state.hash_flip_color2;
            pass_count += 1
        }
        side_to_move = 0 + 2 - side_to_move
    }
    i = new_pv_depth - 1;
    while i >= 0 {
        let side_to_move = new_side_to_move[i as usize];
        let move_0 = new_pv[i as usize];
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i -= 1
    }
    i = 0;
    while i < new_pv_depth {
        board_state.pv[0][i as usize] = new_pv[i as usize];
        i += 1
    }
    board_state.pv_depth[0] = new_pv_depth;
}


/*
  END_GAME
  Provides an interface to the fast endgame solver.
*/

pub fn end_game<FE: FrontEnd>(side_to_move: i32,
                              wld: i32,
                              force_echo: i32,
                              allow_book: i32,
                              komi: i32,
                              mut eval_info: &mut EvaluationType, echo: i32,
                              g_state: &mut FullState) -> i8 {
    let mut flip_stack_ = &mut g_state.flip_stack;
    let mut search_state = &mut g_state.search;
    let mut board_state = &mut g_state.board;
    let mut hash_state = &mut g_state.hash;
    let mut g_timer = &mut g_state.timer;
    let mut end = &mut g_state.end;
    let mut midgame_state = &mut g_state.midgame;
    let mut coeff_state = &mut g_state.coeff;
    let mut moves_state = &mut g_state.moves;
    let mut random_instance = &mut g_state.random;
    let mut g_book = &mut g_state.g_book;
    let mut stable_state = &mut g_state.stable;
    let mut prob_cut = &mut g_state.prob_cut;
    let mut current_confidence: f64 = 0.;
    let mut solve_status = WIN;
    let mut book_move = 0;
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
    let mut old_pv: [i8; 64] = [0; 64];
    let mut book_eval_info = EvaluationType::new();
    let mut empties = 64- disc_count(0, &board_state.board) -
            disc_count(2, &board_state.board);
    /* In komi games, the WLD window is adjusted. */
    if side_to_move == 0 {
        end. komi_shift = komi
    } else { end. komi_shift = -komi }
    /* Check if the position is solved (WLD or exact) in the book. */
    book_move = -1;
    if allow_book != 0 {
        /* Is the exact score known? */
        fill_move_alternatives::<FE>(side_to_move, 16,
                                     &mut g_book,
                                     &mut board_state,
                                     &mut moves_state,
                                     &search_state,
                                     &mut flip_stack_,
                                     &mut hash_state
        );
        book_move = get_book_move::<FE>(side_to_move, 0, eval_info, echo,
                                        &mut board_state,
                                        &mut g_book,
                                        &search_state,
                                        &mut moves_state,
                                        &mut hash_state,
                                        &mut random_instance,
                                        &mut flip_stack_
        );
        if book_move != -(1) {
            search_state.root_eval = (*eval_info).score / 128;
            hash_expand_pv(side_to_move, 1, 4, 0, &mut board_state, &mut hash_state, &mut moves_state, &mut flip_stack_);
            FE::send_solve_status(empties, side_to_move, eval_info, &mut board_state.pv[0], board_state.pv_depth[0], g_timer, search_state);
            return book_move
        }
        /* Is the WLD status known? */
        fill_move_alternatives::<FE>(side_to_move, 4,
                                     &mut g_book,
                                     &mut board_state,
                                     &mut moves_state,
                                     &search_state,
                                     &mut flip_stack_,
                                     &mut hash_state);
        if end. komi_shift == 0 {
            book_move = get_book_move::<FE>(side_to_move, 0, eval_info, echo,
                                            &mut board_state,
                                            &mut g_book,
                                            &search_state,
                                            &mut moves_state,
                                            &mut hash_state,
                                            &mut random_instance,
                                            &mut flip_stack_);
            if book_move != -1 {
                if wld != 0 {
                    search_state.root_eval = eval_info.score / 128;
                    hash_expand_pv(side_to_move, 1, 4 | 2 | 1, 0, &mut board_state, &mut hash_state, &mut moves_state, &mut flip_stack_);
                    FE::send_solve_status(empties, side_to_move, eval_info,
                                          &mut board_state.pv[0],
                                          board_state.pv_depth[0], g_timer, search_state);
                    return book_move
                } else { book_eval_info = *eval_info }
            }
        }
        fill_endgame_hash(8 + 1, 0,
                          &mut g_book
            ,                  &mut board_state
            ,&mut moves_state
            ,&search_state
            ,&mut hash_state
            ,&mut flip_stack_
        );
    }
    g_timer.last_panic_check = 0.0f64;
    solve_status = UNKNOWN;
    old_eval = 0;
    /* Prepare for the shallow searches using the midgame eval */
    board_state.piece_count[0][moves_state.disks_played as usize] = disc_count(0, &board_state.board);
    board_state.piece_count[2][moves_state.disks_played as usize] = disc_count(2, &board_state.board);
    if empties > 32 {
        let value = 0.20f64;
        g_timer.set_panic_threshold(value);
    } else if empties < 22 {
        let value = 0.50f64;
        g_timer.set_panic_threshold(value);
    } else {
        let value = 0.50f64 - (empties - 22) as f64 * 0.03f64;
        g_timer.set_panic_threshold(value);
    }
    FE::reset_buffer_display(g_timer);
    /* Make sure the pre-searches don't mess up the hash table */
    midgame_state.toggle_midgame_hash_usage(1, 0);
    incomplete_search = 0;
    any_search_result = 0;
    /* Start off by selective endgame search */
    last_window_center = 0;
    if empties > 18 {
        if wld != 0 {
            selectivity = 9;
            while selectivity > 0 && g_timer.is_panic_abort() == 0 && force_return == 0 {
                let mut flags: u32 = 0;
                let mut res = WON_POSITION;
                alpha = -1;
                beta = 1;
                search_state.root_eval = end_tree_wrapper::<FE>(end, 0, empties, side_to_move,
                                                                alpha, beta, selectivity, 1, echo, flip_stack_
                                                                , search_state
                                                                , board_state
                                                                , hash_state
                                                                , g_timer
                                                                , midgame_state
                                                                , coeff_state
                                                                , moves_state
                                                                , random_instance
                                                                , stable_state
                                                                , prob_cut);
                adjust_counter(&mut search_state.nodes);
                if g_timer.is_panic_abort() != 0 || force_return != 0 { break ; }
                any_search_result = 1;
                old_eval = search_state.root_eval;
                store_pv(&mut old_pv, &mut old_depth, &board_state);
                current_confidence = confidence[selectivity as usize];
                flags = 4;
                if search_state.root_eval == 0 {
                    res = DRAWN_POSITION
                } else {
                    flags |=
                        (2 | 1) as u32;
                    if search_state.root_eval > 0 {
                        res = WON_POSITION
                    } else { res = LOST_POSITION }
                }
                *eval_info = create_eval_info(SELECTIVE_EVAL, res,
                                     search_state.root_eval * 128,
                                     current_confidence, empties,
                                     0);
                if end.full_output_mode != 0 {
                    hash_expand_pv(side_to_move, 1, flags as i32, selectivity, &mut board_state, &mut hash_state, &mut moves_state, &mut flip_stack_);
                    FE::send_solve_status(empties, side_to_move, eval_info,  &mut board_state.pv[0], board_state.pv_depth[0], g_timer, search_state);
                }
                selectivity -= 1
            }
        } else {
            selectivity = 9;
            while selectivity > 0 && g_timer.is_panic_abort() == 0 &&
                force_return == 0 {
                alpha = last_window_center - 1;
                beta = last_window_center + 1;
                search_state.root_eval =
                    end_tree_wrapper::<FE>(end, 0, empties, side_to_move,
                                           alpha, beta, selectivity,
                                           1, echo, flip_stack_
                                           , search_state
                                           , board_state
                                           , hash_state
                                           , g_timer
                                           , midgame_state
                                           , coeff_state
                                           , moves_state
                                           , random_instance
                                           , stable_state
                                           , prob_cut);
                if search_state.root_eval <= alpha {
                    loop  {
                        last_window_center -= 2;
                        alpha = last_window_center - 1;
                        beta = last_window_center + 1;
                        if g_timer.is_panic_abort() != 0 || force_return != 0 {
                            break ;
                        }
                        search_state.root_eval = end_tree_wrapper::<FE>(end, 0, empties,
                                                                        side_to_move, alpha, beta,
                                                                        selectivity, 1, echo, flip_stack_
                                                                        , search_state
                                                                        , board_state
                                                                        , hash_state
                                                                        , g_timer
                                                                        , midgame_state
                                                                        , coeff_state
                                                                        , moves_state
                                                                        , random_instance
                                                                        , stable_state
                                                                        , prob_cut);
                        if !(search_state.root_eval <= alpha) { break ; }
                    }
                    search_state.root_eval = last_window_center
                } else if search_state.root_eval >= beta {
                    loop  {
                        last_window_center += 2;
                        alpha = last_window_center - 1;
                        beta = last_window_center + 1;
                        if g_timer.is_panic_abort() != 0 || force_return != 0 {
                            break ;
                        }
                        search_state.root_eval =
                           end_tree_wrapper::<FE>(end, 0, empties,
                                                  side_to_move, alpha, beta,
                                                  selectivity, 1, echo, flip_stack_
                                                  , search_state
                                                  , board_state
                                                  , hash_state
                                                  , g_timer
                                                  , midgame_state
                                                  , coeff_state
                                                  , moves_state
                                                  , random_instance
                                                  , stable_state
                                                  , prob_cut);
                        if !(search_state.root_eval >= beta) { break ; }
                    }
                    search_state.root_eval = last_window_center
                }
                adjust_counter(&mut search_state.nodes);
                if g_timer.is_panic_abort() != 0 || force_return != 0 { break ; }
                last_window_center = search_state.root_eval;
                if g_timer.is_panic_abort() == 0 && force_return == 0 {
                    any_search_result = 1;
                    old_eval = search_state.root_eval;
                    store_pv(&mut old_pv, &mut old_depth, &board_state);
                    current_confidence = confidence[selectivity as usize];
                    *eval_info =
                        create_eval_info(SELECTIVE_EVAL, UNSOLVED_POSITION,
                                         search_state.root_eval * 128,
                                         current_confidence, empties,
                                         0);
                    if end.full_output_mode != 0 {
                        hash_expand_pv(side_to_move, 1, 4, selectivity, &mut board_state, &mut hash_state, &mut moves_state, &mut flip_stack_);
                        FE::send_solve_status(empties, side_to_move, eval_info,  &mut board_state.pv[0], board_state.pv_depth[0], g_timer, search_state);
                    }
                }
                selectivity -= 1
            }
        }
    } else { selectivity = 0 }
    /* Check if the selective search took more than 40% of the allocated
         time. If this is the case, there is no point attempting WLD. */
    long_selective_search =  g_timer.check_threshold(0.35f64);
    /* Make sure the panic abort flag is set properly; it must match
       the status of long_selective_search. This is not automatic as
       it is not guaranteed that any selective search was performed. */
     g_timer.check_panic_abort();
    if g_timer.is_panic_abort() != 0 || force_return != 0 ||
        long_selective_search != 0 {
        /* Don't try non-selective solve. */
        if any_search_result != 0 {
            if echo != 0 && (g_timer.is_panic_abort() != 0 || force_return != 0) {
                FE::end_report_semi_panic_abort( g_timer.get_elapsed_time());
                if end.full_output_mode != 0 {
                    let mut flags_0: u32 = 4;
                    if solve_status != DRAW as i32 as u32 {
                        flags_0 |= (2 | 1)
                    }
                    hash_expand_pv(side_to_move, 1, flags_0 as i32, selectivity, &mut board_state, &mut hash_state, &mut moves_state, &mut flip_stack_);
                    FE::send_solve_status(empties, side_to_move, eval_info,  &mut board_state.pv[0], board_state.pv_depth[0], g_timer, search_state);
                }
            }
            board_state.pv[0][0] = end. best_end_root_move;
            board_state.pv_depth[0] = 1;
            search_state.root_eval = old_eval;
            g_timer.clear_panic_abort();
        } else {
            if echo != 0 {
                FE::end_report_panic_abort_2( g_timer.get_elapsed_time());
            }
            search_state.root_eval = -(27000)
        }
        if echo != 0 || force_echo != 0 {
            FE::end_display_zero_status();
        }
        if book_move != -(1) &&
            (book_eval_info.res == WON_POSITION || book_eval_info.res == DRAWN_POSITION) {
            /* If there is a known win (or mismarked draw) available,
             always play it upon timeout. */
            *eval_info = book_eval_info;
            search_state.root_eval = (*eval_info).score / 128;
            return book_move
        } else {
            return board_state.pv[0][0]
        }
    }
    /* Start non-selective solve */
    if wld != 0 {
        alpha = -(1);
        beta = 1;
    } else {
        alpha = last_window_center - 1;
        beta = last_window_center + 1;
    }
    search_state.root_eval = end_tree_wrapper::<FE>(end, 0, empties, side_to_move, alpha, beta,
                                                    0, 1, echo, flip_stack_
                                                    , search_state
                                                    , board_state
                                                    , hash_state
                                                    , g_timer
                                                    , midgame_state
                                                    , coeff_state
                                                    , moves_state
                                                    , random_instance
                                                    , stable_state
                                                    , prob_cut);
    adjust_counter(&mut search_state.nodes);
    if g_timer.is_panic_abort() == 0 && force_return == 0 {
        if wld == 0 {
            if search_state.root_eval <= alpha {
                let mut ceiling_value = last_window_center - 2;
                loop  {
                    alpha = ceiling_value - 1;
                    beta = ceiling_value;
                    search_state.root_eval = end_tree_wrapper::<FE>(end, 0, empties,
                                                                    side_to_move, alpha, beta,
                                                                    0, 1, echo, flip_stack_
                                                                    , search_state
                                                                    , board_state
                                                                    , hash_state
                                                                    , g_timer
                                                                    , midgame_state
                                                                    , coeff_state
                                                                    , moves_state
                                                                    , random_instance
                                                                    , stable_state
                                                                    , prob_cut);
                    if g_timer.is_panic_abort() != 0 || force_return != 0 { break ; }
                    if search_state.root_eval > alpha { break ; }
                    ceiling_value -= 2
                }
            } else if search_state.root_eval >= beta {
                let mut floor_value = last_window_center + 2;
                loop  {
                    alpha = floor_value - 1;
                    beta = floor_value + 1;
                    search_state.root_eval =
                       end_tree_wrapper::<FE>(end, 0, empties,
                                              side_to_move, alpha, beta,
                                              0, 1, echo, flip_stack_
                                              , search_state
                                              , board_state
                                              , hash_state
                                              , g_timer
                                              , midgame_state
                                              , coeff_state
                                              , moves_state
                                              , random_instance
                                              , stable_state
                                              , prob_cut);
                    if g_timer.is_panic_abort() != 0 || force_return != 0 { break ; }
                    assert!( search_state.root_eval > alpha );
                    if search_state.root_eval < beta { break ; }
                    floor_value += 2
                }
            }
        }
        if g_timer.is_panic_abort() == 0 && force_return == 0 {
            let mut res_0 = WON_POSITION;
            if search_state.root_eval < 0 {
                res_0 = LOST_POSITION
            } else if search_state.root_eval == 0 {
                res_0 = DRAWN_POSITION
            } else { res_0 = WON_POSITION }
            if wld != 0 {
                let mut flags_1: u32 = 0;
                if search_state.root_eval == 0 {
                    flags_1 = 4
                } else {
                    flags_1 = (2 | 1)
                }
                *eval_info =
                    create_eval_info(WLD_EVAL, res_0,
                                     search_state.root_eval * 128, 0.0f64,
                                     empties, 0);
                if end.full_output_mode != 0 {
                    hash_expand_pv(side_to_move, 1, flags_1 as i32, 0, &mut board_state, &mut hash_state, &mut moves_state, &mut flip_stack_);
                    FE::send_solve_status(empties, side_to_move, eval_info,  &mut board_state.pv[0], board_state.pv_depth[0], g_timer, search_state);
                }
            } else {
                *eval_info =
                    create_eval_info(EXACT_EVAL, res_0,
                                     search_state.root_eval * 128, 0.0f64,
                                     empties, 0);
                if end.full_output_mode != 0 {
                    hash_expand_pv(side_to_move, 1, 4, 0, &mut board_state, &mut hash_state, &mut moves_state, &mut flip_stack_);
                    FE::send_solve_status(empties, side_to_move, eval_info,  &mut board_state.pv[0], board_state.pv_depth[0], g_timer, search_state);
                }
            }
        }
    }
    adjust_counter(&mut search_state.nodes);
    /* Check for abort. */
    if g_timer.is_panic_abort() != 0 || force_return != 0 {
        if any_search_result != 0 {
            if echo != 0 {
                FE::end_report_semi_panic_abort_3( g_timer.get_elapsed_time());
                if end.full_output_mode != 0 {
                    let mut flags_2: u32 = 0;
                    flags_2 = 4;
                    if search_state.root_eval != 0 {
                        flags_2 |= 2 | 1
                    }
                    hash_expand_pv(side_to_move, 1, flags_2 as i32, 0, &mut board_state, &mut hash_state, &mut moves_state, &mut flip_stack_);
                    FE::send_solve_status(empties, side_to_move, eval_info,
                                          &mut board_state.pv[0], board_state.pv_depth[0], g_timer, search_state);
                }
                if echo != 0 || force_echo != 0 {
                    FE::end_display_zero_status();
                }
            }
            restore_pv(&old_pv, old_depth, &mut board_state);
            search_state.root_eval = old_eval;
            g_timer.clear_panic_abort();
        } else {
            if echo != 0 {
                FE::end_report_panic_abort( g_timer.get_elapsed_time());
            }
            search_state.root_eval = - 27000;
        }
        return board_state.pv[0][0]
    }
    /* Update solve info. */
    store_pv(&mut old_pv, &mut old_depth, &board_state);
    old_eval = search_state.root_eval;
    if g_timer.is_panic_abort() == 0 && force_return == 0 &&
        empties > end.earliest_wld_solve {
        end.earliest_wld_solve = empties
    }
    /* Check for aborted search. */
    exact_score_failed = 0;
    if incomplete_search != 0 {
        if echo != 0 {
            FE::end_report_semi_panic_abort_2( g_timer.get_elapsed_time());
            if end.full_output_mode != 0 {
                hash_expand_pv(side_to_move, 1, 4, 0, &mut board_state, &mut hash_state, &mut moves_state, &mut flip_stack_);
                FE::send_solve_status(empties, side_to_move, eval_info,  &mut board_state.pv[0], board_state.pv_depth[0], g_timer, search_state);
            }
            if echo != 0 || force_echo != 0 {
                FE::end_display_zero_status();
            }
        }
        board_state.pv[0][0] = end. best_end_root_move;
        board_state.pv_depth[0] = 1;
        search_state.root_eval = old_eval;
        exact_score_failed = 1;
        g_timer.clear_panic_abort();
    }
    if abs(search_state.root_eval) % 2 == 1 {
        if search_state.root_eval > 0 {
            search_state.root_eval += 1
        } else { search_state.root_eval -= 1 }
    }
    if exact_score_failed == 0 && wld == 0 && empties > end.earliest_full_solve {
        end.earliest_full_solve = empties
    }
    if wld == 0 && exact_score_failed == 0 {
        eval_info.type_0 = EXACT_EVAL;
        eval_info.score = search_state.root_eval * 128
    }
    if wld == 0 && exact_score_failed == 0 {
        hash_expand_pv(side_to_move, 1, 4, 0, &mut board_state, &mut hash_state, &mut moves_state, &mut flip_stack_);
        FE::send_solve_status(empties, side_to_move, eval_info,  &mut board_state.pv[0], board_state.pv_depth[0], g_timer, search_state);
    }
    if echo != 0 || force_echo != 0 {
        FE::end_display_zero_status();
    }
    /* For shallow endgames, we can afford to compute the entire PV
       move by move. */
    if wld == 0 && incomplete_search == 0 && force_return == 0 && empties <= 16 {
        full_expand_pv::<FE>(end, side_to_move, 0, echo, flip_stack_
                             , search_state
                             , board_state
                             , hash_state
                             , g_timer
                             , midgame_state
                             , coeff_state
                             , moves_state
                             , random_instance
                             , stable_state
                             , prob_cut);
    }
    board_state.pv[0][0]
}
