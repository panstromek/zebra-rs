use crate::src::libc;
use crate::src::bitboard::{BitBoard, non_iterative_popcount, set_bitboards, square_mask};
use crate::src::patterns::pow3;
use crate::src::search::position_list;
use crate::src::bitbtest::{bb_flips, TestFlips_bitboard};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MoveLink {
    pub pred: libc::c_int,
    pub succ: libc::c_int,
}
/* When this flag is set, the DynP tables are calculated and
   output and then the program is terminated. */
/* Global variables */
/* All discs determined as stable last time COUNT_STABLE was called
   for the two colors */

pub static mut last_black_stable: BitBoard = BitBoard{high: 0, low: 0,};

pub static mut last_white_stable: BitBoard = BitBoard{high: 0, low: 0,};
/* Local variables */
/* For each of the 3^8 edges, edge_stable[] holds an 8-bit mask
   where a bit is set if the corresponding disc can't be changed EVER. */
static mut edge_stable: [libc::c_short; 6561] = [0; 6561];
/* For each edge, *_stable[] holds the number of safe discs counted
   as follows: 1 for a stable corner and 2 for a stable non-corner.
   This to avoid counting corners twice. */
static mut black_stable: [libc::c_uchar; 6561] = [0; 6561];
static mut white_stable: [libc::c_uchar; 6561] = [0; 6561];
/* A conversion table from the 2^8 edge values for one player to
   the corresponding base-3 value. */
static mut base_conversion: [libc::c_short; 256] = [0; 256];
/* The base-3 indices for the edges */
static mut edge_a1h1: libc::c_int = 0;
static mut edge_a8h8: libc::c_int = 0;
static mut edge_a1a8: libc::c_int = 0;
static mut edge_h1h8: libc::c_int = 0;
/* Position list used in the complete stability search */

pub static mut stab_move_list: [MoveLink; 100] =
    [MoveLink{pred: 0, succ: 0,}; 100];
unsafe fn and_line_shift_64(mut target: *mut BitBoard,
                                       mut base: BitBoard,
                                       mut shift: libc::c_int,
                                       mut dir_ss: BitBoard) {
    /* Shift to the left */
    dir_ss.high |= base.high << shift | base.low >> 32 as libc::c_int - shift;
    dir_ss.low |= base.low << shift;
    /* Shift to the right */
    dir_ss.high |= base.high >> shift;
    dir_ss.low |= base.low >> shift | base.high << 32 as libc::c_int - shift;
    (*target).high &= dir_ss.high;
    (*target).low &= dir_ss.low;
}
/*
  EDGE_ZARDOZ_STABLE
  Determines the bit mask for (a subset of) the stable discs in a position.
  Zardoz' algorithm + edge tables is used.
*/
unsafe fn edge_zardoz_stable(mut ss: *mut BitBoard,
                                        mut dd: BitBoard, mut od: BitBoard) {
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
    let mut t: libc::c_uint = 0;
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
    t &= t >> 4 as libc::c_int;
    t &= t >> 2 as libc::c_int;
    t &= t >> 1 as libc::c_int;
    lrf.high =
        (t &
             0x1010101 as libc::c_int as
                 libc::c_uint).wrapping_mul(255 as libc::c_int as
                                                libc::c_uint) |
            0x81818181 as libc::c_uint;
    t = fb.low;
    t &= t >> 4 as libc::c_int;
    t &= t >> 2 as libc::c_int;
    t &= t >> 1 as libc::c_int;
    lrf.low =
        (t &
             0x1010101 as libc::c_int as
                 libc::c_uint).wrapping_mul(255 as libc::c_int as
                                                libc::c_uint) |
            0x81818181 as libc::c_uint;
    t = fb.high & fb.low;
    t &= t >> 16 as libc::c_int | t << 16 as libc::c_int;
    t &= t >> 8 as libc::c_int | t << 24 as libc::c_int;
    udf.high = t | 0xff000000 as libc::c_uint;
    udf.low = t | 0xff as libc::c_int as libc::c_uint;
    daf.high = 0xff818181 as libc::c_uint;
    daf.low = 0x818181ff as libc::c_uint;
    t =
        ((fb.high << 4 as libc::c_int |
              0xf0f0f0f as libc::c_int as libc::c_uint) & fb.low |
             0xe0c08000 as libc::c_uint) &
            0x1ffffffe as libc::c_int as libc::c_uint;
    t &= t >> 14 as libc::c_int | t << 14 as libc::c_int;
    t &= t >> 7 as libc::c_int | t << 21 as libc::c_int;
    daf.low |= t & 0x1f3f7efc as libc::c_int as libc::c_uint;
    daf.high |=
        t >> 4 as libc::c_int & 0x103070f as libc::c_int as libc::c_uint;
    t =
        ((fb.low >> 4 as libc::c_int | 0xf0f0f0f0 as libc::c_uint) & fb.high |
             0x10307 as libc::c_int as libc::c_uint) &
            0x7ffffff8 as libc::c_int as libc::c_uint;
    t &= t >> 14 as libc::c_int | t << 14 as libc::c_int;
    t &= t >> 7 as libc::c_int | t << 21 as libc::c_int;
    daf.high |= t & 0x3e7cf8f0 as libc::c_int as libc::c_uint;
    daf.low |= t << 4 as libc::c_int & 0xe0c08000 as libc::c_uint;
    dbf.high = 0xff818181 as libc::c_uint;
    dbf.low = 0x818181ff as libc::c_uint;
    t = (fb.high >> 4 as libc::c_int | 0xf0f0f0f0 as libc::c_uint) & fb.low;
    /* 17 16 15 14 13 12 11 10  9  8 NG  6  5  4  3  2  1  0 */
    t &=
        t >> 18 as libc::c_int |
            0x3c000 as libc::c_int as
                libc::c_uint; /*  *  *  *  * 31 30 29 28 27 26 25 NG 23 22 21 20 19 18 */
    t &=
        t >> 9 as libc::c_int |
            t <<
                9 as
                    libc::c_int; /*  8 NG  6  5  4  3  2  1  0 17 16 15 14 13 12 11 10  9 */
    t |=
        t <<
            18 as
                libc::c_int; /* 26 25 NG 23 22 21 20 19 18  *  *  *  * 31 30 29 28 27 */
    dbf.low |= t & 0xf8fc7e3f as libc::c_uint;
    dbf.high |= t << 4 as libc::c_int & 0x80c0e0f0 as libc::c_uint;
    t =
        (fb.low << 4 as libc::c_int |
             0xf0f0f0f as libc::c_int as libc::c_uint) & fb.high;
    t &= t >> 18 as libc::c_int | 0x3c000 as libc::c_int as libc::c_uint;
    t &= t >> 9 as libc::c_int | t << 9 as libc::c_int;
    t |= t << 18 as libc::c_int;
    dbf.high |= t & 0x7c3e1f0f as libc::c_int as libc::c_uint;
    dbf.low |=
        t >> 4 as libc::c_int & 0x7030100 as libc::c_int as libc::c_uint;
    (*ss).high |= lrf.high & udf.high & daf.high & dbf.high & dd.high;
    (*ss).low |= lrf.low & udf.low & daf.low & dbf.low & dd.low;
    if (*ss).high | (*ss).low == 0 as libc::c_int as libc::c_uint { return }
    loop  {
        ost = *ss;
        expand_ss.high =
            lrf.high | ost.high << 1 as libc::c_int |
                ost.high >> 1 as libc::c_int;
        expand_ss.low =
            lrf.low | ost.low << 1 as libc::c_int |
                ost.low >> 1 as libc::c_int;
        and_line_shift_64(&mut expand_ss, ost, 8 as libc::c_int, udf);
        and_line_shift_64(&mut expand_ss, ost, 7 as libc::c_int, daf);
        and_line_shift_64(&mut expand_ss, ost, 9 as libc::c_int, dbf);
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

pub unsafe fn count_edge_stable(mut color: libc::c_int,
                                           mut col_bits: BitBoard,
                                           mut opp_bits: BitBoard)
 -> libc::c_int {
    let mut col_mask: libc::c_uint = 0;
    let mut opp_mask: libc::c_uint = 0;
    let mut ix_a1a8: libc::c_uint = 0;
    let mut ix_h1h8: libc::c_uint = 0;
    let mut ix_a1h1: libc::c_uint = 0;
    let mut ix_a8h8: libc::c_uint = 0;
    col_mask =
        (col_bits.low &
             0x1010101 as libc::c_int as
                 libc::c_uint).wrapping_add((col_bits.high &
                                                 0x1010101 as libc::c_int as
                                                     libc::c_uint) <<
                                                4 as
                                                    libc::c_int).wrapping_mul(0x1020408
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
            >> 24 as libc::c_int;
    opp_mask =
        (opp_bits.low &
             0x1010101 as libc::c_int as
                 libc::c_uint).wrapping_add((opp_bits.high &
                                                 0x1010101 as libc::c_int as
                                                     libc::c_uint) <<
                                                4 as
                                                    libc::c_int).wrapping_mul(0x1020408
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
            >> 24 as libc::c_int;
    ix_a1a8 =
        (base_conversion[col_mask as usize] as libc::c_int -
             base_conversion[opp_mask as usize] as libc::c_int) as
            libc::c_uint;
    col_mask =
        ((col_bits.low & 0x80808080 as libc::c_uint) >>
             4 as
                 libc::c_int).wrapping_add(col_bits.high &
                                               0x80808080 as
                                                   libc::c_uint).wrapping_mul((0x1020408
                                                                                   as
                                                                                   libc::c_int
                                                                                   /
                                                                                   8
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  libc::c_uint)
            >> 24 as libc::c_int;
    opp_mask =
        ((opp_bits.low & 0x80808080 as libc::c_uint) >>
             4 as
                 libc::c_int).wrapping_add(opp_bits.high &
                                               0x80808080 as
                                                   libc::c_uint).wrapping_mul((0x1020408
                                                                                   as
                                                                                   libc::c_int
                                                                                   /
                                                                                   8
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  libc::c_uint)
            >> 24 as libc::c_int;
    ix_h1h8 =
        (base_conversion[col_mask as usize] as libc::c_int -
             base_conversion[opp_mask as usize] as libc::c_int) as
            libc::c_uint;
    ix_a1h1 =
        (base_conversion[(col_bits.low & 255 as libc::c_int as libc::c_uint)
                             as usize] as libc::c_int -
             base_conversion[(opp_bits.low &
                                  255 as libc::c_int as libc::c_uint) as
                                 usize] as libc::c_int) as libc::c_uint;
    ix_a8h8 =
        (base_conversion[(col_bits.high >> 24 as libc::c_int) as usize] as
             libc::c_int -
             base_conversion[(opp_bits.high >> 24 as libc::c_int) as usize] as
                 libc::c_int) as libc::c_uint;
    if color == 0 as libc::c_int {
        edge_a1h1 =
            ((3280 as libc::c_int * 1 as libc::c_int) as
                 libc::c_uint).wrapping_sub(ix_a1h1) as libc::c_int;
        edge_a8h8 =
            ((3280 as libc::c_int * 1 as libc::c_int) as
                 libc::c_uint).wrapping_sub(ix_a8h8) as libc::c_int;
        edge_a1a8 =
            ((3280 as libc::c_int * 1 as libc::c_int) as
                 libc::c_uint).wrapping_sub(ix_a1a8) as libc::c_int;
        edge_h1h8 =
            ((3280 as libc::c_int * 1 as libc::c_int) as
                 libc::c_uint).wrapping_sub(ix_h1h8) as libc::c_int;
        return (black_stable[edge_a1h1 as usize] as libc::c_int +
                    black_stable[edge_a1a8 as usize] as libc::c_int +
                    black_stable[edge_a8h8 as usize] as libc::c_int +
                    black_stable[edge_h1h8 as usize] as libc::c_int) as
                   libc::c_uchar as libc::c_int / 2 as libc::c_int
    } else {
        edge_a1h1 =
            ((3280 as libc::c_int * 1 as libc::c_int) as
                 libc::c_uint).wrapping_add(ix_a1h1) as libc::c_int;
        edge_a8h8 =
            ((3280 as libc::c_int * 1 as libc::c_int) as
                 libc::c_uint).wrapping_add(ix_a8h8) as libc::c_int;
        edge_a1a8 =
            ((3280 as libc::c_int * 1 as libc::c_int) as
                 libc::c_uint).wrapping_add(ix_a1a8) as libc::c_int;
        edge_h1h8 =
            ((3280 as libc::c_int * 1 as libc::c_int) as
                 libc::c_uint).wrapping_add(ix_h1h8) as libc::c_int;
        return (white_stable[edge_a1h1 as usize] as libc::c_int +
                    white_stable[edge_a1a8 as usize] as libc::c_int +
                    white_stable[edge_a8h8 as usize] as libc::c_int +
                    white_stable[edge_h1h8 as usize] as libc::c_int) as
                   libc::c_uchar as libc::c_int / 2 as libc::c_int
    };
}
/*
  COUNT_STABLE
  Returns the number of stable discs for COLOR.
  Side effect: last_black_stable or last_white_stable is modified.
  Note: COUNT_EDGE_STABLE must have been called immediately
        before this function is called *or you lose big*.
*/

pub unsafe fn count_stable(mut color: libc::c_int,
                                      mut col_bits: BitBoard,
                                      mut opp_bits: BitBoard) -> libc::c_int {
    let mut t: libc::c_uint = 0;
    let mut col_stable = BitBoard{high: 0, low: 0,};
    let mut common_stable = BitBoard{high: 0, low: 0,};
    /* Stable edge discs */
    common_stable.low = edge_stable[edge_a1h1 as usize] as libc::c_uint;
    common_stable.high =
        ((edge_stable[edge_a8h8 as usize] as libc::c_int) <<
             24 as libc::c_int) as libc::c_uint;
    t = edge_stable[edge_a1a8 as usize] as libc::c_uint;
    common_stable.low |=
        (t &
             0xf as libc::c_int as
                 libc::c_uint).wrapping_mul(0x204081 as libc::c_int as
                                                libc::c_uint) &
            0x1010101 as libc::c_int as libc::c_uint;
    common_stable.high |=
        (t >>
             4 as
                 libc::c_int).wrapping_mul(0x204081 as libc::c_int as
                                               libc::c_uint) &
            0x1010101 as libc::c_int as libc::c_uint;
    t = edge_stable[edge_h1h8 as usize] as libc::c_uint;
    common_stable.low |=
        (t &
             0xf as libc::c_int as
                 libc::c_uint).wrapping_mul(0x10204080 as libc::c_int as
                                                libc::c_uint) &
            0x80808080 as libc::c_uint;
    common_stable.high |=
        (t >>
             4 as
                 libc::c_int).wrapping_mul(0x10204080 as libc::c_int as
                                               libc::c_uint) &
            0x80808080 as libc::c_uint;
    /* Expand the stable edge discs into a full set of stable discs */
    col_stable.high = col_bits.high & common_stable.high;
    col_stable.low = col_bits.low & common_stable.low;
    edge_zardoz_stable(&mut col_stable, col_bits, opp_bits);
    if color == 0 as libc::c_int {
        last_black_stable = col_stable
    } else { last_white_stable = col_stable }
    if col_stable.high | col_stable.low != 0 {
        return non_iterative_popcount(col_stable.high, col_stable.low) as
                   libc::c_int
    } else { return 0 as libc::c_int };
}
/*
  STABILITY_SEARCH
  Searches the subtree rooted at the current position and tries to
  find variations in which the discs in CANDIDATE_BITS are
  flipped. Aborts if all those discs are stable in the subtree.
*/
unsafe fn stability_search(mut my_bits: BitBoard,
                                      mut opp_bits: BitBoard,
                                      mut side_to_move: libc::c_int,
                                      mut candidate_bits: *mut BitBoard,
                                      mut max_depth: libc::c_int,
                                      mut last_was_pass: libc::c_int,
                                      mut stability_nodes: *mut libc::c_int) {
    let mut sq: libc::c_int = 0;
    let mut old_sq: libc::c_int = 0;
    let mut mobility: libc::c_int = 0;
    let mut black_bits = BitBoard{high: 0, low: 0,};
    let mut white_bits = BitBoard{high: 0, low: 0,};
    let mut new_my_bits = BitBoard{high: 0, low: 0,};
    let mut new_opp_bits = BitBoard{high: 0, low: 0,};
    let mut all_stable_bits = BitBoard{high: 0, low: 0,};
    *stability_nodes += 1;
    if *stability_nodes > 10000 as libc::c_int { return }
    if max_depth >= 3 as libc::c_int {
        if side_to_move == 0 as libc::c_int {
            black_bits = my_bits;
            white_bits = opp_bits
        } else { black_bits = opp_bits; white_bits = my_bits }
        all_stable_bits.high = 0 as libc::c_int as libc::c_uint;
        all_stable_bits.low = 0 as libc::c_int as libc::c_uint;
        count_edge_stable(0 as libc::c_int, black_bits, white_bits);
        if (*candidate_bits).high & black_bits.high != 0 ||
               (*candidate_bits).low & black_bits.low != 0 {
            count_stable(0 as libc::c_int, black_bits, white_bits);
            all_stable_bits.high |= last_black_stable.high;
            all_stable_bits.low |= last_black_stable.low
        }
        if (*candidate_bits).high & white_bits.high != 0 ||
               (*candidate_bits).low & white_bits.low != 0 {
            count_stable(2 as libc::c_int, white_bits, black_bits);
            all_stable_bits.high |= last_white_stable.high;
            all_stable_bits.low |= last_white_stable.low
        }
        if (*candidate_bits).high & !all_stable_bits.high ==
               0 as libc::c_int as libc::c_uint &&
               (*candidate_bits).low & !all_stable_bits.low ==
                   0 as libc::c_int as libc::c_uint {
            return
        }
    }
    mobility = 0 as libc::c_int;
    old_sq = 0 as libc::c_int;
    sq = stab_move_list[old_sq as usize].succ;
    while sq != 99 as libc::c_int {
        if TestFlips_bitboard[(sq - 11 as libc::c_int) as
                                  usize].expect("non-null function pointer")(my_bits.high,
                                                                             my_bits.low,
                                                                             opp_bits.high,
                                                                             opp_bits.low)
               != 0 {
            new_my_bits = bb_flips;
            bb_flips.high &= !my_bits.high;
            bb_flips.low &= !my_bits.low;
            (*candidate_bits).high &= !bb_flips.high;
            (*candidate_bits).low &= !bb_flips.low;
            if max_depth > 1 as libc::c_int {
                new_opp_bits.high = opp_bits.high & !bb_flips.high;
                new_opp_bits.low = opp_bits.low & !bb_flips.low;
                stab_move_list[old_sq as usize].succ =
                    stab_move_list[sq as usize].succ;
                stability_search(new_opp_bits, new_my_bits,
                                 0 as libc::c_int + 2 as libc::c_int -
                                     side_to_move, candidate_bits,
                                 max_depth - 1 as libc::c_int,
                                 0 as libc::c_int, stability_nodes);
                stab_move_list[old_sq as usize].succ = sq
            }
            mobility += 1
        }
        old_sq = sq;
        sq = stab_move_list[sq as usize].succ
    }
    if mobility == 0 as libc::c_int && last_was_pass == 0 {
        stability_search(opp_bits, my_bits,
                         0 as libc::c_int + 2 as libc::c_int - side_to_move,
                         candidate_bits, max_depth, 1 as libc::c_int,
                         stability_nodes);
    };
}
/*
  COMPLETE_STABILITY_SEARCH
  Tries to compute all stable discs by search the entire game tree.
  The actual work is performed by STABILITY_SEARCH above.
*/
unsafe fn complete_stability_search(mut board: *mut libc::c_int,
                                               mut side_to_move: libc::c_int,
                                               mut stable_bits:
                                                   *mut BitBoard) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut empties: libc::c_int = 0;
    let mut shallow_depth: libc::c_int = 0;
    let mut stability_nodes: libc::c_int = 0;
    let mut abort: libc::c_int = 0;
    let mut my_bits = BitBoard{high: 0, low: 0,};
    let mut opp_bits = BitBoard{high: 0, low: 0,};
    let mut all_bits = BitBoard{high: 0, low: 0,};
    let mut candidate_bits = BitBoard{high: 0, low: 0,};
    let mut test_bits = BitBoard{high: 0, low: 0,};
    /* Prepare the move list */
    let mut last_sq = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 60 as libc::c_int {
        let mut sq = position_list[i as usize];
        if *board.offset(sq as isize) == 1 as libc::c_int {
            stab_move_list[last_sq as usize].succ = sq;
            stab_move_list[sq as usize].pred = last_sq;
            last_sq = sq
        }
        i += 1
    }
    stab_move_list[last_sq as usize].succ = 99 as libc::c_int;
    empties = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            if *board.offset((10 as libc::c_int * i + j) as isize) ==
                   1 as libc::c_int {
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
    stability_nodes = 0 as libc::c_int;
    shallow_depth = 4 as libc::c_int;
    stability_search(my_bits, opp_bits, side_to_move, &mut candidate_bits,
                     if empties < shallow_depth {
                         empties
                     } else { shallow_depth }, 0 as libc::c_int,
                     &mut stability_nodes);
    /* Scan through the rest of the discs one at a time until the
       maximum number of stability nodes is exceeded. Hopefully
       a subset of the stable discs is found also if this happens. */
    abort = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int && abort == 0 {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int && abort == 0 {
            let mut sq_0 = 10 as libc::c_int * i + j;
            test_bits = square_mask[sq_0 as usize];
            if test_bits.high & candidate_bits.high |
                   test_bits.low & candidate_bits.low != 0 {
                stability_search(my_bits, opp_bits, side_to_move,
                                 &mut test_bits, empties, 0 as libc::c_int,
                                 &mut stability_nodes);
                abort =
                    (stability_nodes > 10000 as libc::c_int) as libc::c_int;
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
  Determines what discs on BOARD are stable with SIDE_TO_MOVE to play next.
  The stability status of all squares (black, white and empty)
  is returned in the boolean vector IS_STABLE.
*/

pub unsafe fn get_stable(mut board: *mut libc::c_int,
                                    mut side_to_move: libc::c_int,
                                    mut is_stable: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut mask: libc::c_uint = 0;
    let mut black_bits = BitBoard{high: 0, low: 0,};
    let mut white_bits = BitBoard{high: 0, low: 0,};
    let mut all_stable = BitBoard{high: 0, low: 0,};
    set_bitboards(board, 0 as libc::c_int, &mut black_bits, &mut white_bits);
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        *is_stable.offset(i as isize) = 0 as libc::c_int;
        i += 1
    }
    if black_bits.high | black_bits.low == 0 as libc::c_int as libc::c_uint ||
           white_bits.high | white_bits.low ==
               0 as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int;
        while i <= 8 as libc::c_int {
            j = 1 as libc::c_int;
            while j <= 8 as libc::c_int {
                *is_stable.offset((10 as libc::c_int * i + j) as isize) =
                    1 as libc::c_int;
                j += 1
            }
            i += 1
        }
    } else {
        /* Nobody wiped out */
        count_edge_stable(0 as libc::c_int, black_bits, white_bits);
        count_stable(0 as libc::c_int, black_bits, white_bits);
        count_stable(2 as libc::c_int, white_bits, black_bits);
        all_stable.high = last_black_stable.high | last_white_stable.high;
        all_stable.low = last_black_stable.low | last_white_stable.low;
        complete_stability_search(board, side_to_move, &mut all_stable);
        i = 1 as libc::c_int;
        mask = 1 as libc::c_int as libc::c_uint;
        while i <= 4 as libc::c_int {
            j = 1 as libc::c_int;
            while j <= 8 as libc::c_int {
                if all_stable.low & mask != 0 {
                    *is_stable.offset((10 as libc::c_int * i + j) as isize) =
                        1 as libc::c_int
                }
                j += 1;
                mask <<= 1 as libc::c_int
            }
            i += 1
        }
        i = 5 as libc::c_int;
        mask = 1 as libc::c_int as libc::c_uint;
        while i <= 8 as libc::c_int {
            j = 1 as libc::c_int;
            while j <= 8 as libc::c_int {
                if all_stable.high & mask != 0 {
                    *is_stable.offset((10 as libc::c_int * i + j) as isize) =
                        1 as libc::c_int
                }
                j += 1;
                mask <<= 1 as libc::c_int
            }
            i += 1
        }
    };
}
/*
  RECURSIVE_FIND_STABLE
  Returns a bit mask describing the set of stable discs in the
  edge PATTERN. When a bit mask is calculated, it's stored in
  a table so that any particular bit mask only is generated once.
*/
unsafe fn recursive_find_stable(mut pattern: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut new_pattern: libc::c_int = 0;
    let mut stable: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut row: [libc::c_int; 8] = [0; 8];
    let mut stored_row: [libc::c_int; 8] = [0; 8];
    if edge_stable[pattern as usize] as libc::c_int != -(1 as libc::c_int) {
        return edge_stable[pattern as usize] as libc::c_int
    }
    temp = pattern;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        row[i as usize] = temp % 3 as libc::c_int;
        i += 1;
        temp /= 3 as libc::c_int
    }
    /* All positions stable unless proved otherwise. */
    stable = 255 as libc::c_int;
    /* Play out the 8 different moves and AND together the stability masks. */
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        stored_row[j as usize] = row[j as usize];
        j += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        /* Make sure we work with the original configuration */
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            row[j as usize] = stored_row[j as usize];
            j += 1
        }
        if row[i as usize] == 1 as libc::c_int {
            /* Empty ==> playable! */
            /* Mark the empty square as unstable and store position */
            stable &= !((1 as libc::c_int) << i);
            /* Play out a black move */
            row[i as usize] = 0 as libc::c_int;
            if i >= 2 as libc::c_int {
                j = i - 1 as libc::c_int;
                while j >= 1 as libc::c_int &&
                          row[j as usize] == 2 as libc::c_int {
                    j -= 1
                }
                if row[j as usize] == 0 as libc::c_int {
                    j += 1;
                    while j < i {
                        row[j as usize] = 0 as libc::c_int;
                        stable &= !((1 as libc::c_int) << j);
                        j += 1
                    }
                }
            }
            if i <= 5 as libc::c_int {
                j = i + 1 as libc::c_int;
                while j <= 6 as libc::c_int &&
                          row[j as usize] == 2 as libc::c_int {
                    j += 1
                }
                if row[j as usize] == 0 as libc::c_int {
                    j -= 1;
                    while j > i {
                        row[j as usize] = 0 as libc::c_int;
                        stable &= !((1 as libc::c_int) << j);
                        j -= 1
                    }
                }
            }
            new_pattern = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < 8 as libc::c_int {
                new_pattern += pow3[j as usize] * row[j as usize];
                j += 1
            }
            stable &= recursive_find_stable(new_pattern);
            /* Restore position */
            j = 0 as libc::c_int;
            while j < 8 as libc::c_int {
                row[j as usize] = stored_row[j as usize];
                j += 1
            }
            /* Play out a white move */
            row[i as usize] = 2 as libc::c_int;
            if i >= 2 as libc::c_int {
                j = i - 1 as libc::c_int;
                while j >= 1 as libc::c_int &&
                          row[j as usize] == 0 as libc::c_int {
                    j -= 1
                }
                if row[j as usize] == 2 as libc::c_int {
                    j += 1;
                    while j < i {
                        row[j as usize] = 2 as libc::c_int;
                        stable &= !((1 as libc::c_int) << j);
                        j += 1
                    }
                }
            }
            if i <= 5 as libc::c_int {
                j = i + 1 as libc::c_int;
                while j <= 6 as libc::c_int &&
                          row[j as usize] == 0 as libc::c_int {
                    j += 1
                }
                if row[j as usize] == 2 as libc::c_int {
                    j -= 1;
                    while j > i {
                        row[j as usize] = 2 as libc::c_int;
                        stable &= !((1 as libc::c_int) << j);
                        j -= 1
                    }
                }
            }
            new_pattern = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < 8 as libc::c_int {
                new_pattern += pow3[j as usize] * row[j as usize];
                j += 1
            }
            stable &= recursive_find_stable(new_pattern)
        }
        i += 1
    }
    /* Store and return */
    edge_stable[pattern as usize] = stable as libc::c_short;
    return stable;
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
unsafe fn count_color_stable() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pattern: libc::c_int = 0;
    let mut row: [libc::c_int; 8] = [0; 8];
    static mut stable_incr: [libc::c_int; 8] =
        [1 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
         2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
         2 as libc::c_int, 1 as libc::c_int];
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int { row[i as usize] = 0 as libc::c_int; i += 1 }
    pattern = 0 as libc::c_int;
    while pattern < 6561 as libc::c_int {
        black_stable[pattern as usize] = 0 as libc::c_int as libc::c_uchar;
        white_stable[pattern as usize] = 0 as libc::c_int as libc::c_uchar;
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            if edge_stable[pattern as usize] as libc::c_int &
                   (1 as libc::c_int) << j != 0 {
                if row[j as usize] == 0 as libc::c_int {
                    black_stable[pattern as usize] =
                        (black_stable[pattern as usize] as libc::c_int +
                             stable_incr[j as usize]) as libc::c_uchar
                } else if row[j as usize] == 2 as libc::c_int {
                    white_stable[pattern as usize] =
                        (white_stable[pattern as usize] as libc::c_int +
                             stable_incr[j as usize]) as libc::c_uchar
                }
            }
            j += 1
        }
        /* Next configuration */
        i = 0 as libc::c_int;
        loop  {
            /* The odometer principle */
            row[i as usize] += 1;
            if row[i as usize] == 3 as libc::c_int {
                row[i as usize] = 0 as libc::c_int
            }
            i += 1;
            if !(row[(i - 1 as libc::c_int) as usize] == 0 as libc::c_int &&
                     i < 8 as libc::c_int) {
                break ;
            }
        }
        pattern += 1
    };
}
/*
  INIT_STABLE
  Build the table containing the stability masks for all edge
  configurations. This is done using dynamic programming.
*/

pub unsafe fn init_stable() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        base_conversion[i as usize] = 0 as libc::c_int as libc::c_short;
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            if i & (1 as libc::c_int) << j != 0 {
                base_conversion[i as usize] =
                    (base_conversion[i as usize] as libc::c_int +
                         pow3[j as usize]) as libc::c_short
            }
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 6561 as libc::c_int {
        edge_stable[i as usize] = -(1 as libc::c_int) as libc::c_short;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 6561 as libc::c_int {
        if edge_stable[i as usize] as libc::c_int == -(1 as libc::c_int) {
            recursive_find_stable(i);
        }
        i += 1
    }
    count_color_stable();
}
