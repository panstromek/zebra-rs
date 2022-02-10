use ::patterns::pow3;

use crate::src::bitboard::{BitBoard, non_iterative_popcount, set_bitboards, square_mask};
use crate::src::bitbtest::TestFlips_bitboard;
use crate::src::end::End;
use crate::src::globals::Board;
use crate::src::search::position_list;
use crate::src::getcoeff::odometer_principle;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MoveLink {
    pub pred: i8,
    pub succ: i8,
}
/* When this flag is set, the DynP tables are calculated and
   output and then the program is terminated. */
/* Global variables */
pub struct StableState {
    /* All discs determined as stable last time COUNT_STABLE was called
       for the two colors */
    last_black_stable: BitBoard,
    last_white_stable: BitBoard,
    /* For each of the 3^8 edges, edge_stable[] holds an 8-bit mask
   where a bit is set if the corresponding disc can't be changed EVER. */
    edge_stable: [i16; 6561],
    /* For each edge, *_stable[] holds the number of safe discs counted
   as follows: 1 for a stable corner and 2 for a stable non-corner.
   This to avoid counting corners twice. */
    black_stable: [u8; 6561],
    white_stable: [u8; 6561],
    /* A conversion table from the 2^8 edge values for one player to
        the corresponding base-3 value. */
    base_conversion: [i16; 256],
    /* The base-3 indices for the edges */
    edge_a1h1: i32,
    edge_a8h8: i32,
    edge_a1a8: i32,
    edge_h1h8: i32,
    /* Position list used in the complete stability search */
    stab_move_list: [MoveLink; 100],
}

impl StableState {
    pub const fn new() -> Self {
        StableState {
            last_black_stable: BitBoard { high: 0, low: 0 },
            last_white_stable: BitBoard { high: 0, low: 0 },
            edge_stable: [0; 6561],
            black_stable: [0; 6561],
            white_stable: [0; 6561],
            base_conversion: [0; 256],
            edge_a1h1: 0,
            edge_a8h8: 0,
            edge_a1a8: 0,
            edge_h1h8: 0,
            stab_move_list: [MoveLink { pred: 0, succ: 0 }; 100],
        }
    }
}

fn and_line_shift_64(target: &mut BitBoard, base: BitBoard, shift: i32, mut dir_ss: BitBoard) {
    /* Shift to the left */
    dir_ss.high |= base.high << shift | base.low >> 32 - shift;
    dir_ss.low |= base.low << shift;
    /* Shift to the right */
    dir_ss.high |= base.high >> shift;
    dir_ss.low |= base.low >> shift | base.high << 32 - shift;
    (*target).high &= dir_ss.high;
    (*target).low &= dir_ss.low;
}
/*
  EDGE_ZARDOZ_STABLE
  Determines the bit mask for (a subset of) the stable discs in a position.
  Zardoz' algorithm + edge tables is used.
*/
fn edge_zardoz_stable(ss: &mut BitBoard, dd: BitBoard, od: BitBoard) {
    /* dd is the disks of the side we are looking for stable disks for
       od is the opponent
       ss are the stable disks */
    let mut ost = BitBoard{high: 0, low: 0,};
    let mut fb = BitBoard{high: 0, low: 0,};
    let mut lrf = BitBoard{high: 0, low: 0,};
    let mut udf = BitBoard{high: 0, low: 0,};
    let mut daf = BitBoard{high: 0, low: 0,};
    let mut dbf = BitBoard{high: 0, low: 0,};
    let mut expand_ss = BitBoard{high: 0, low: 0,};
    let mut t: u32 = 0;
    /* ost is a simple test to see if numbers of
       stable disks have stopped increasing.

       fb is the squares which have been played
       ie either by white or black

       udf are the up-down columns that are filled, and so no vertical flips
       lrf are the left-right
       daf are the NE-SW diags filled
       dbf are the NW-SE diags filled */
    /* a stable disk is a disk that has a stable disk on one
       side in each of the 4 directions
       N.B. beyond the edges is of course stable */
    fb.high = dd.high | od.high; /* rotate within bit 1 and bit 28 */
    fb.low = dd.low | od.low; /* rotate within bit 3 and bit 30 */
    t = fb.high;
    t &= t >> 4;
    t &= t >> 2;
    t &= t >> 1;
    lrf.high = (t & 0x1010101 as i32 as u32).wrapping_mul(255) | 0x81818181 as u32;
    t = fb.low;
    t &= t >> 4;
    t &= t >> 2;
    t &= t >> 1;
    lrf.low = (t & 0x1010101 as i32 as u32).wrapping_mul(255) | 0x81818181 as u32;
    t = fb.high & fb.low;
    t &= t >> 16 | t << 16;
    t &= t >> 8 | t << 24;
    udf.high = t | 0xff000000 as u32;
    udf.low = t | 0xff as i32 as u32;
    daf.high = 0xff818181 as u32;
    daf.low = 0x818181ff as u32;
    t = ((fb.high << 4 | 0xf0f0f0f as i32 as u32) & fb.low | 0xe0c08000 as u32) & 0x1ffffffe as i32 as u32;
    t &= t >> 14 | t << 14;
    t &= t >> 7 | t << 21;
    daf.low |= t & 0x1f3f7efc as i32 as u32;
    daf.high |= t >> 4 & 0x103070f as i32 as u32;
    t = ((fb.low >> 4 | 0xf0f0f0f0 as u32) & fb.high | 0x10307 as i32 as u32) & 0x7ffffff8 as i32 as u32;
    t &= t >> 14 | t << 14;
    t &= t >> 7 | t << 21;
    daf.high |= t & 0x3e7cf8f0 as i32 as u32;
    daf.low |= t << 4 & 0xe0c08000 as u32;
    dbf.high = 0xff818181 as u32;
    dbf.low = 0x818181ff as u32;
    t = (fb.high >> 4 | 0xf0f0f0f0 as u32) & fb.low;
    /* 17 16 15 14 13 12 11 10  9  8 NG  6  5  4  3  2  1  0 */
    t &= t >> 18 | 0x3c000 as i32 as u32; /*  *  *  *  * 31 30 29 28 27 26 25 NG 23 22 21 20 19 18 */
    t &= t >> 9 | t << 9; /*  8 NG  6  5  4  3  2  1  0 17 16 15 14 13 12 11 10  9 */
    t |= t << 18; /* 26 25 NG 23 22 21 20 19 18  *  *  *  * 31 30 29 28 27 */
    dbf.low |= t & 0xf8fc7e3f as u32;
    dbf.high |= t << 4 & 0x80c0e0f0 as u32;
    t = (fb.low << 4 | 0xf0f0f0f as i32 as u32) & fb.high;
    t &= t >> 18 | 0x3c000 as i32 as u32;
    t &= t >> 9 | t << 9;
    t |= t << 18;
    dbf.high |= t & 0x7c3e1f0f as i32 as u32;
    dbf.low |= t >> 4 & 0x7030100 as i32 as u32;
    (*ss).high |= lrf.high & udf.high & daf.high & dbf.high & dd.high;
    (*ss).low |= lrf.low & udf.low & daf.low & dbf.low & dd.low;
    if (*ss).high | (*ss).low == 0 { return }
    loop  {
        ost = *ss;
        expand_ss.high = lrf.high | ost.high << 1 | ost.high >> 1;
        expand_ss.low = lrf.low | ost.low << 1 | ost.low >> 1;
        and_line_shift_64(&mut expand_ss, ost, 8, udf);
        and_line_shift_64(&mut expand_ss, ost, 7, daf);
        and_line_shift_64(&mut expand_ss, ost, 9, dbf);
        (*ss).high = ost.high | expand_ss.high & dd.high;
        (*ss).low = ost.low | expand_ss.low & dd.low;
        if !(ost.high ^ (*ss).high | ost.low ^ (*ss).low != 0) { break ; }
    };
    /* changing */
    // ss->high &= dd.high;
    // ss->low &= dd.low;
}
/*
  COUNT_EDGE_STABLE
  Returns the number of stable edge discs for COLOR.
  Side effect: The edge indices are calculated. They are needed
  by COUNT_STABLE below.
*/

pub fn count_edge_stable(color: i32, col_bits: BitBoard, opp_bits: BitBoard, state: &mut StableState) -> i32 {
    let col_mask = (col_bits.low & 0x1010101 as u32).wrapping_add((col_bits.high & 0x1010101 as u32) << 4).wrapping_mul(0x1020408 as u32) >> 24;
    let opp_mask = (opp_bits.low & 0x1010101 as u32).wrapping_add((opp_bits.high & 0x1010101 as u32) << 4).wrapping_mul(0x1020408 as u32) >> 24;
    let ix_a1a8 = (state.base_conversion[col_mask as usize] as i32 - state.base_conversion[opp_mask as usize] as i32) as u32;
    let col_mask = ((col_bits.low & 0x80808080 as u32) >> 4).wrapping_add(col_bits.high & 0x80808080 as u32).wrapping_mul((0x1020408 / 8) as u32) >> 24;
    let opp_mask = ((opp_bits.low & 0x80808080 as u32) >> 4).wrapping_add(opp_bits.high & 0x80808080 as u32).wrapping_mul((0x1020408 / 8) as u32) >> 24;
    let ix_h1h8 = (state.base_conversion[col_mask as usize] as i32 - state.base_conversion[opp_mask as usize] as i32) as u32;
    let ix_a1h1 = (state.base_conversion[(col_bits.low & 255) as usize] as i32 - state.base_conversion[(opp_bits.low & 255) as usize] as i32) as u32;
    let ix_a8h8 = (state.base_conversion[(col_bits.high >> 24) as usize] as i32 - state.base_conversion[(opp_bits.high >> 24) as usize] as i32) as u32;
    if color == 0 {
        state.edge_a1h1 = 3280_u32.wrapping_sub(ix_a1h1) as i32;
        state.edge_a8h8 = 3280_u32.wrapping_sub(ix_a8h8) as i32;
        state.edge_a1a8 = 3280_u32.wrapping_sub(ix_a1a8) as i32;
        state.edge_h1h8 = 3280_u32.wrapping_sub(ix_h1h8) as i32;
        return (state.black_stable[state.edge_a1h1 as usize] as i32 +
                    state.black_stable[state.edge_a1a8 as usize] as i32 +
                    state.black_stable[state.edge_a8h8 as usize] as i32 +
                    state.black_stable[state.edge_h1h8 as usize] as i32) as u8 as i32 / 2
    } else {
        state.edge_a1h1 = 3280_u32.wrapping_add(ix_a1h1) as i32;
        state.edge_a8h8 = 3280_u32.wrapping_add(ix_a8h8) as i32;
        state.edge_a1a8 = 3280_u32.wrapping_add(ix_a1a8) as i32;
        state.edge_h1h8 = 3280_u32.wrapping_add(ix_h1h8) as i32;
        return (state.white_stable[state.edge_a1h1 as usize] as i32 +
                    state.white_stable[state.edge_a1a8 as usize] as i32 +
                    state.white_stable[state.edge_a8h8 as usize] as i32 +
                    state.white_stable[state.edge_h1h8 as usize] as i32) as
                   u8 as i32 / 2
    };
}
/*
  COUNT_STABLE
  Returns the number of stable discs for COLOR.
  Side effect: last_black_stable or last_white_stable is modified.
  Note: COUNT_EDGE_STABLE must have been called immediately
        before this function is called *or you lose big*.
*/

pub fn count_stable(color: i32, col_bits: BitBoard, opp_bits: BitBoard, state: &mut StableState) -> i32 {
    let mut col_stable = BitBoard{high: 0, low: 0,};
    let mut common_stable = BitBoard{high: 0, low: 0,};
    /* Stable edge discs */
    common_stable.low = state.edge_stable[state.edge_a1h1 as usize] as u32;
    common_stable.high = ((state.edge_stable[state.edge_a8h8 as usize] as i32) << 24) as u32;
    let t = state.edge_stable[state.edge_a1a8 as usize] as u32;
    common_stable.low |= (t & 0xf as i32 as u32).wrapping_mul(0x204081 as i32 as u32) & 0x1010101 as i32 as u32;
    common_stable.high |= (t >> 4).wrapping_mul(0x204081 as i32 as u32) & 0x1010101 as i32 as u32;
    let t = state.edge_stable[state.edge_h1h8 as usize] as u32;
    common_stable.low |= (t & 0xf as i32 as u32).wrapping_mul(0x10204080 as i32 as u32) & 0x80808080 as u32;
    common_stable.high |= (t >> 4).wrapping_mul(0x10204080 as i32 as u32) & 0x80808080 as u32;
    /* Expand the stable edge discs into a full set of stable discs */
    col_stable.high = col_bits.high & common_stable.high;
    col_stable.low = col_bits.low & common_stable.low;
    edge_zardoz_stable(&mut col_stable, col_bits, opp_bits);
    if color == 0 {
        state.last_black_stable = col_stable
    } else {
        state.last_white_stable = col_stable
    }

    return if col_stable.high | col_stable.low != 0 {
        non_iterative_popcount(col_stable.high, col_stable.low) as i32
    } else {
        0
    };
}
/*
  STABILITY_SEARCH
  Searches the subtree rooted at the current position and tries to
  find variations in which the discs in CANDIDATE_BITS are
  flipped. Aborts if all those discs are stable in the subtree.
*/
fn stability_search(end: &mut End, my_bits: BitBoard,
                           opp_bits: BitBoard,
                           side_to_move: i32,
                           mut candidate_bits: &mut BitBoard,
                           max_depth: i32,
                           last_was_pass: i32,
                           stability_nodes: &mut i32, state: &mut StableState) {
    let mut sq = 0;
    let mut old_sq = 0;
    let mut mobility: i32 = 0;
    let mut black_bits = BitBoard{high: 0, low: 0,};
    let mut white_bits = BitBoard{high: 0, low: 0,};
    let mut new_my_bits = BitBoard{high: 0, low: 0,};
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut all_stable_bits = BitBoard{high: 0, low: 0,};
    *stability_nodes += 1;
    if *stability_nodes > 10000 { return }
    if max_depth >= 3 {
        if side_to_move == 0 {
            black_bits = my_bits;
            white_bits = opp_bits
        } else { black_bits = opp_bits; white_bits = my_bits }
        all_stable_bits.high = 0;
        all_stable_bits.low = 0;
        count_edge_stable(0, black_bits, white_bits,  state);
        if (*candidate_bits).high & black_bits.high != 0 ||
               (*candidate_bits).low & black_bits.low != 0 {
            count_stable(0, black_bits, white_bits, state);
            all_stable_bits.high |= state.last_black_stable.high;
            all_stable_bits.low |= state.last_black_stable.low
        }
        if (*candidate_bits).high & white_bits.high != 0 ||
               (*candidate_bits).low & white_bits.low != 0 {
            count_stable(2, white_bits, black_bits,  state);
            all_stable_bits.high |= state.last_white_stable.high;
            all_stable_bits.low |= state.last_white_stable.low
        }
        if (*candidate_bits).high & !all_stable_bits.high == 0 &&
               (*candidate_bits).low & !all_stable_bits.low == 0 {
            return
        }
    }
    mobility = 0;
    old_sq = 0;
    sq = state.stab_move_list[old_sq as usize].succ;
    while sq != 99 {
        let flip_test_result =
            TestFlips_bitboard[(sq - 11) as usize](my_bits.high, my_bits.low, opp_bits.high, opp_bits.low);
        end.bb_flips.high = flip_test_result.1.high;
        end.bb_flips.low = flip_test_result.1.low;
        if flip_test_result.0 != 0 {
            new_my_bits = end.bb_flips;
            end.bb_flips.high &= !my_bits.high;
            end.bb_flips.low &= !my_bits.low;
            (*candidate_bits).high &= !end.bb_flips.high;
            (*candidate_bits).low &= !end.bb_flips.low;
            if max_depth > 1 {
                new_opp_bits.high = opp_bits.high & !end.bb_flips.high;
                new_opp_bits.low = opp_bits.low & !end.bb_flips.low;
                state.stab_move_list[old_sq as usize].succ = state.stab_move_list[sq as usize].succ;
                stability_search(end, new_opp_bits, new_my_bits, 0 + 2 - side_to_move, candidate_bits,
                                 max_depth - 1, 0, stability_nodes, state);
                state.stab_move_list[old_sq as usize].succ = sq
            }
            mobility += 1
        }
        old_sq = sq;
        sq = state.stab_move_list[sq as usize].succ
    }
    if mobility == 0 && last_was_pass == 0 {
        stability_search(end, opp_bits, my_bits,
                         0 + 2 - side_to_move,
                         candidate_bits, max_depth, 1,
                         stability_nodes,  state);
    };
}
/*
  COMPLETE_STABILITY_SEARCH
  Tries to compute all stable discs by search the entire game tree.
  The actual work is performed by STABILITY_SEARCH above.
*/
fn complete_stability_search(end: &mut End,  board: &Board,
                                    side_to_move: i32,
                                    mut stable_bits:
                                                   &mut BitBoard, state: &mut StableState) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut empties: i32 = 0;
    let mut shallow_depth: i32 = 0;
    let mut stability_nodes: i32 = 0;
    let mut abort: i32 = 0;
    let mut my_bits = BitBoard{high: 0, low: 0,};
    let mut opp_bits = BitBoard{high: 0, low: 0,};
    let mut all_bits = BitBoard{high: 0, low: 0,};
    let mut candidate_bits = BitBoard{high: 0, low: 0,};
    let mut test_bits = BitBoard{high: 0, low: 0,};
    /* Prepare the move list */
    let mut last_sq = 0;
    i = 0;
    while i < 60 {
        let sq = position_list[i as usize];
        if board[sq as usize] == 1 {
            state.stab_move_list[last_sq as usize].succ = sq;
            state.stab_move_list[sq as usize].pred = last_sq;
            last_sq = sq
        }
        i += 1
    }
    state.stab_move_list[last_sq as usize].succ = 99;
    empties = 0;
    i = 1;
    while i <= 8 {
        j = 1;
        while j <= 8 {
            if board[(10 * i + j) as usize] == 1 {
                empties += 1
            }
            j += 1
        }
        i += 1
    }
    /* Prepare the bitmaps for the stability search */
    set_bitboards(board, side_to_move, &mut my_bits, &mut opp_bits);
    all_bits.high = my_bits.high | opp_bits.high;
    all_bits.low = my_bits.low | opp_bits.low;
    candidate_bits.high = all_bits.high & !(*stable_bits).high;
    candidate_bits.low = all_bits.low & !(*stable_bits).low;
    /* Search all potentially stable discs for at most 4 plies
       to weed out those easily flippable */
    stability_nodes = 0;
    shallow_depth = 4;
    stability_search(end, my_bits, opp_bits, side_to_move, &mut candidate_bits,
                     if empties < shallow_depth {
                         empties
                     } else { shallow_depth }, 0,
                     &mut stability_nodes, state);
    /* Scan through the rest of the discs one at a time until the
       maximum number of stability nodes is exceeded. Hopefully
       a subset of the stable discs is found also if this happens. */
    abort = 0;
    i = 1;
    while i <= 8 && abort == 0 {
        j = 1;
        while j <= 8 && abort == 0 {
            let sq_0 = 10 * i + j;
            test_bits = square_mask[sq_0 as usize];
            if test_bits.high & candidate_bits.high |
                   test_bits.low & candidate_bits.low != 0 {
                stability_search(end, my_bits, opp_bits, side_to_move,
                                 &mut test_bits, empties, 0,
                                 &mut stability_nodes,  state);
                abort =
                    (stability_nodes > 10000) as i32;
                if abort == 0 {
                    if test_bits.high | test_bits.low != 0 {
                        (*stable_bits).high |= test_bits.high;
                        (*stable_bits).low |= test_bits.low
                    }
                }
            }
            j += 1
        }
        i += 1
    };
}
/*
  GET_STABLE
  Determines what discs on BOARD are stableIDE_TO_MOVE to play next.
  The stability status of all squares (black, white and empty)
  is returned in the boolean vector IS_STABLE.
*/

pub fn get_stable(end:&mut End, board: &Board,
                         side_to_move: i32,
                         is_stable: &mut [i32], state: &mut StableState) {
    use engine_traits::Offset;
    let mut j: i32 = 0;
    let mut mask: u32 = 0;
    let mut black_bits = BitBoard{high: 0, low: 0,};
    let mut white_bits = BitBoard{high: 0, low: 0,};
    let mut all_stable = BitBoard{high: 0, low: 0,};
    set_bitboards(board, 0, &mut black_bits, &mut white_bits);
    let mut i = 0;
    while i < 100 {
        *is_stable.offset(i as isize) = 0;
        i += 1
    }
    if black_bits.high | black_bits.low == 0 || white_bits.high | white_bits.low == 0 {
        i = 1;
        while i <= 8 {
            j = 1;
            while j <= 8 {
                *is_stable.offset((10 * i + j) as isize) = 1;
                j += 1
            }
            i += 1
        }
    } else {
        /* Nobody wiped out */
        count_edge_stable(0, black_bits, white_bits, state);
        count_stable(0, black_bits, white_bits, state);
        count_stable(2, white_bits, black_bits, state);
        all_stable.high = state.last_black_stable.high | state.last_white_stable.high;
        all_stable.low = state.last_black_stable.low | state.last_white_stable.low;
        complete_stability_search(end, board, side_to_move, &mut all_stable, state);
        i = 1;
        mask = 1;
        while i <= 4 {
            j = 1;
            while j <= 8 {
                if all_stable.low & mask != 0 {
                    *is_stable.offset((10 * i + j) as isize) = 1
                }
                j += 1;
                mask <<= 1
            }
            i += 1
        }
        i = 5;
        mask = 1;
        while i <= 8 {
            j = 1;
            while j <= 8 {
                if all_stable.high & mask != 0 {
                    *is_stable.offset((10 * i + j) as isize) = 1
                }
                j += 1;
                mask <<= 1
            }
            i += 1
        }
    };
}
impl StableState {
/*
  RECURSIVE_FIND_STABLE
  Returns a bit mask describing the set of stable discs in the
  edge PATTERN. When a bit mask is calculated, it's stored in
  a table so that any particular bit mask only is generated once.
*/
fn recursive_find_stable(&mut self, pattern: i32)
 -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut new_pattern: i32 = 0;
    let mut stable: i32 = 0;
    let mut temp: i32 = 0;
    let mut row: [i32; 8] = [0; 8];
    let mut stored_row: [i32; 8] = [0; 8];
    if self.edge_stable[pattern as usize] as i32 != -(1) {
        return self.edge_stable[pattern as usize] as i32
    }
    temp = pattern;
    i = 0;
    while i < 8 {
        row[i as usize] = temp % 3;
        i += 1;
        temp /= 3
    }
    /* All positions stable unless proved otherwise. */
    stable = 255;
    /* Play out the 8 different moves and AND together the stability masks. */
    j = 0;
    while j < 8 {
        stored_row[j as usize] = row[j as usize];
        j += 1
    }
    i = 0;
    while i < 8 {
        /* Make sure we work with the original configuration */
        j = 0;
        while j < 8 {
            row[j as usize] = stored_row[j as usize];
            j += 1
        }
        if row[i as usize] == 1 {
            /* Empty ==> playable! */
            /* Mark the empty square as unstable and store position */
            stable &= !((1) << i);
            /* Play out a black move */
            row[i as usize] = 0;
            if i >= 2 {
                j = i - 1;
                while j >= 1 &&
                          row[j as usize] == 2 {
                    j -= 1
                }
                if row[j as usize] == 0 {
                    j += 1;
                    while j < i {
                        row[j as usize] = 0;
                        stable &= !((1) << j);
                        j += 1
                    }
                }
            }
            if i <= 5 {
                j = i + 1;
                while j <= 6 &&
                          row[j as usize] == 2 {
                    j += 1
                }
                if row[j as usize] == 0 {
                    j -= 1;
                    while j > i {
                        row[j as usize] = 0;
                        stable &= !((1) << j);
                        j -= 1
                    }
                }
            }
            new_pattern = 0;
            j = 0;
            while j < 8 {
                new_pattern += pow3(j as usize) * row[j as usize];
                j += 1
            }
            stable &= self.recursive_find_stable(new_pattern);
            /* Restore position */
            j = 0;
            while j < 8 {
                row[j as usize] = stored_row[j as usize];
                j += 1
            }
            /* Play out a white move */
            row[i as usize] = 2;
            if i >= 2 {
                j = i - 1;
                while j >= 1 &&
                          row[j as usize] == 0 {
                    j -= 1
                }
                if row[j as usize] == 2 {
                    j += 1;
                    while j < i {
                        row[j as usize] = 2;
                        stable &= !((1) << j);
                        j += 1
                    }
                }
            }
            if i <= 5 {
                j = i + 1;
                while j <= 6 &&
                          row[j as usize] == 0 {
                    j += 1
                }
                if row[j as usize] == 2 {
                    j -= 1;
                    while j > i {
                        row[j as usize] = 2;
                        stable &= !((1) << j);
                        j -= 1
                    }
                }
            }
            new_pattern = 0;
            j = 0;
            while j < 8 {
                new_pattern += pow3(j as usize) * row[j as usize];
                j += 1
            }
            stable &= self.recursive_find_stable(new_pattern)
        }
        i += 1
    }
    /* Store and return */
    self.edge_stable[pattern as usize] = stable as i16;
    return stable;
}
}
/*
  COUNT_COLOR_STABLE
  Determines the number of stable discs for each of the edge configurations
  for the two colors. This is done using the following convention:
  - a stable corner disc gives stability of 1
  - a stable non-corner disc gives stability of 2
  This way the stability values for the four edges can be added together
  without any risk for double-counting.
*/
fn count_color_stable(stable_state_: &mut StableState) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pattern: i32 = 0;
    let mut row: [i32; 10] = [0; 10];
    static stable_incr: [i32; 8] =
        [1, 2, 2,
         2, 2, 2,
         2, 1];
    pattern = 0;
    while pattern < 6561 {
        stable_state_.black_stable[pattern as usize] = 0;
        stable_state_.white_stable[pattern as usize] = 0;
        j = 0;
        while j < 8 {
            if stable_state_.edge_stable[pattern as usize] as i32 &
                   (1) << j != 0 {
                if row[j as usize] == 0 {
                    stable_state_.black_stable[pattern as usize] =
                        (stable_state_.black_stable[pattern as usize] as i32 +
                             stable_incr[j as usize]) as u8
                } else if row[j as usize] == 2 {
                    stable_state_.white_stable[pattern as usize] =
                        (stable_state_.white_stable[pattern as usize] as i32 +
                             stable_incr[j as usize]) as u8
                }
            }
            j += 1
        }
        /* Next configuration */
        odometer_principle(&mut row, 8);
        pattern += 1
    };
}
/*
  INIT_STABLE
  Build the table containing the stability masks for all edge
  configurations. This is done using dynamic programming.
*/

pub fn init_stable(state: &mut StableState) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < 256 {
        state.base_conversion[i as usize] = 0;
        j = 0;
        while j < 8 {
            if i & (1) << j != 0 {
                state.base_conversion[i as usize] =
                    (state.base_conversion[i as usize] as i32 +
                         pow3(j as usize)) as i16
            }
            j += 1
        }
        i += 1
    }
    i = 0;
    while i < 6561 {
        state.edge_stable[i as usize] = -(1) as i16;
        i += 1
    }
    i = 0;
    while i < 6561 {
        if state.edge_stable[i as usize] as i32 == -(1) {
            state.recursive_find_stable(i);
        }
        i += 1
    }
    count_color_stable(state);
}
