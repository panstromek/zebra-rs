use std::ffi::c_void;
use std::process::exit;
use std::ptr::null_mut;
use std::slice::from_raw_parts_mut;

use ::patterns::{flip8, pow3};
use coeff::terminal_patterns;
use coeff::constant_and_parity_feature;
use coeff::CoeffSetData;
use coeff::CoeffSet;
use coeff::AllocationBlock;

pub use coeff::odometer_principle;
use engine_traits::{CoeffSource, Offset};

use crate::src::error::FrontEnd;
use crate::src::globals::{Board, BoardState};
use crate::src::globals;
use crate::src::moves::MovesState;
use crate::src::stubs::floor;

pub struct CoeffAdjustments {
    pub disc_adjust: f64,
    pub edge_adjust: f64,
    pub corner_adjust: f64,
    pub x_adjust: f64,
}

const EMPTY_ALLOC_BLOCK: Option<Box<AllocationBlock>> = None;
const NEW_COEFF_SET : CoeffSet = CoeffSet::new();

pub struct CoeffState {
    stage_count: i32,
    stage: [i32; 61],
    eval_map: [i32; 61],
    pub set: [CoeffSet; 61],
}

impl CoeffState {
    pub const fn new() -> CoeffState {
        CoeffState {
            stage_count: 0,
            stage: [0; 61],
            eval_map: [0; 61],
            set: [NEW_COEFF_SET; 61],
        }
    }
}

/*
   GENERATE_BATCH
   Interpolates between two stages.
*/
fn generate_batch(target: &mut [i16], source1: &[i16], weight1: i32, source2: &[i16], weight2: i32) {
    let total_weight = weight1 + weight2;
    source1.iter()
        .zip(source2.iter())
        .zip(target.iter_mut())
        .for_each(|((&source1, &source2), target)| {
            *target = (
                (weight1 * source1 as i32 + weight2 * source2 as i32) / total_weight
            ) as i16;
        });
}

/*
  DISC_COUNT_ADJUSTMENT
*/
/* Adjust the coefficients so as to reflect the encouragement for
       having lots of discs */
pub fn eval_adjustment(disc_adjust: f64,
                              edge_adjust: f64,
                              corner_adjust: f64,
                              x_adjust: f64, state: &mut CoeffState) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut adjust: i32 = 0;
    let mut row: [i32; 10] = [0; 10];
    i = 0;
    while i < state.stage_count - 1 as i32 {
        /* Bonuses for having more discs */
        j = 0;
        let stage_i = state.stage[i as usize] as usize;
        let set = &mut state.set.split_at_mut(60);
        let stage_set = set.0[stage_i].data.as_mut().unwrap();
        let sixty_set = set.1[0].data.as_mut().unwrap();
        while j < 59049 as i32 {
            let ref mut fresh2 =
                *(stage_set.afile2x_mut()as &mut[i16]).offset(j as isize);
            *fresh2 =
                (*fresh2 as f64 +
                    *sixty_set.afile2x_mut().offset(j as isize) as i32
                        as f64 * disc_adjust) as i16;
            let ref mut fresh3 =
                *(stage_set.corner52_mut()as &mut[i16]).offset(j as isize);
            *fresh3 =
                (*fresh3 as f64 +
                    *sixty_set.corner52_mut().offset(j as isize) as
                        i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 19683 as i32 {
            let ref mut fresh4 =
                *(stage_set.corner33_mut()as &mut [i16]).offset(j as isize);
            *fresh4 =
                (*fresh4 as f64 +
                    *sixty_set.corner33_mut().offset(j as isize) as
                        i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 6561 as i32 {
            let ref mut fresh5 =
                *(stage_set.bfile_mut()as &mut[i16]).offset(j as isize);
            *fresh5 =
                (*fresh5 as f64 +
                    *sixty_set.bfile_mut().offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            let ref mut fresh6 =
                *(stage_set.cfile_mut()as &mut[i16]).offset(j as isize);
            *fresh6 =
                (*fresh6 as f64 +
                    *sixty_set.cfile_mut().offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            let ref mut fresh7 =
                *(stage_set.dfile_mut()as &mut[i16]).offset(j as isize);
            *fresh7 =
                (*fresh7 as f64 +
                    *sixty_set.dfile_mut().offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            let ref mut fresh8 =
                *(stage_set.diag8_mut()as &mut[i16]).offset(j as isize);
            *fresh8 =
                (*fresh8 as f64 +
                    *sixty_set.diag8_mut().offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 2187 as i32 {
            let ref mut fresh9 =
                *(stage_set.diag7_mut()as &mut[i16]).offset(j as isize);
            *fresh9 =
                (*fresh9 as f64 +
                    *sixty_set.diag7_mut().offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 729 as i32 {
            let ref mut fresh10 =
                *(stage_set.diag6_mut()as &mut[i16]).offset(j as isize);
            *fresh10 =
                (*fresh10 as f64 +
                    *sixty_set.diag6_mut().offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 243 as i32 {
            let ref mut fresh11 =
                *(stage_set.diag5_mut()as &mut[i16]).offset(j as isize);
            *fresh11 =
                (*fresh11 as f64 +
                    *sixty_set.diag5_mut().offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 81 as i32 {
            let ref mut fresh12 =
                *(stage_set.diag4_mut()as &mut[i16]).offset(j as isize);
            *fresh12 =
                (*fresh12 as f64 +
                    *sixty_set.diag4_mut().offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 10 as i32 {
            row[j as usize] = 0;
            j += 1
        }
        j = 0;
        while j < 59049 as i32 {
            adjust = 0;
            /* Bonus for having edge discs */
            k = 1;
            while k <= 6 as i32 {
                if row[k as usize] == 0 as i32 {
                    adjust =
                        (adjust as f64 + 128.0f64 * edge_adjust) as
                            i32
                } else if row[k as usize] == 2 as i32 {
                    adjust =
                        (adjust as f64 - 128.0f64 * edge_adjust) as
                            i32
                }
                k += 1
            }
            /* Bonus for having corners.  The "0.5 *" is because corners are part
            of two A-file+2X patterns. */
            if row[0] == 0 as i32 {
                adjust =
                    (adjust as f64 +
                        0.5f64 * 128.0f64 * corner_adjust) as i32
            } else if row[0] == 2 as i32 {
                adjust =
                    (adjust as f64 -
                        0.5f64 * 128.0f64 * corner_adjust) as i32
            }
            if row[7] == 0 as i32 {
                adjust =
                    (adjust as f64 +
                        0.5f64 * 128.0f64 * corner_adjust) as i32
            } else if row[7] == 2 as i32 {
                adjust =
                    (adjust as f64 -
                        0.5f64 * 128.0f64 * corner_adjust) as i32
            }
            /* Bonus for having X-squares when the adjacent corners are empty.
            Scaling by 0.5 applies here too. */
            if row[8] == 0 as i32 &&
                row[0] == 1 as i32 {
                adjust =
                    (adjust as f64 + 0.5f64 * 128.0f64 * x_adjust)
                        as i32
            } else if row[8] == 2 as i32 &&
                row[0] == 1 as i32 {
                adjust =
                    (adjust as f64 - 0.5f64 * 128.0f64 * x_adjust)
                        as i32
            }
            if row[9] == 0 as i32 &&
                row[7] == 1 as i32 {
                adjust =
                    (adjust as f64 + 0.5f64 * 128.0f64 * x_adjust)
                        as i32
            } else if row[9] == 2 as i32 &&
                row[7] == 1 as i32 {
                adjust =
                    (adjust as f64 - 0.5f64 * 128.0f64 * x_adjust)
                        as i32
            }
            let ref mut fresh13 =
                *(stage_set.afile2x_mut() as &mut [i16]).offset(j as isize);
            *fresh13 = (*fresh13 as i32 + adjust) as i16;
            /* Next configuration */
            odometer_principle(&mut row, 10);
            j += 1
        }
        i += 1
    };
}


/*
   REMOVE_SPECIFIC_COEFFS
   Removes the interpolated coefficients for a
   specific game phase from memory.
*/
pub fn remove_specific_coeffs(coeff_set: &mut CoeffSet) {
    let coeff_set = coeff_set;
    if coeff_set.loaded != 0 {
        if coeff_set.permanent == 0 {
            // block_allocated_[coeff_set.block as usize] = false;
        }
        coeff_set.loaded = 0
    };
}
/*
   REMOVE_COEFFS
   Removes pattern tables which have gone out of scope from memory.
*/

pub fn remove_coeffs(phase: i32, state: &mut CoeffState) {
    let mut i: i32 = 0;
    while i < phase {
        remove_specific_coeffs(&mut state.set[i as usize]);
        i += 1
    };
}


/*
   ALLOCATE_SET
   Finds memory for all patterns belonging to a certain stage.
   Maintains an internal memory handler to boost
   performance and avoid heap fragmentation.
*/
pub fn allocate_set(curr_stage: i32, state: &mut CoeffState, interpolate_from : Option<(i32, i32, i32, i32)>) {
    let mut set_data = CoeffSetData {
       allocation: Box::new(AllocationBlock {
           afile2x_block: [0; 59049],
           bfile_block: [0; 6561],
           cfile_block: [0; 6561],
           dfile_block: [0; 6561],
           diag8_block: [0; 6561],
           diag7_block: [0; 2187],
           diag6_block: [0; 729],
           diag5_block: [0; 243],
           diag4_block: [0; 81],
           corner33_block: [0; 19683],
           corner52_block: [0; 59049],
       })
    };
    // This is kinda ugly quick workaround for some aliasing issues.
    // Ideally, this code shouldn't be here, it was in load_set before
    if let Some((prev, next, weight1, weight2 )) = interpolate_from {
        let previous_set_item = state.set[prev as usize].data.as_ref().unwrap();
        let next_set_item = state.set[next as usize].data.as_ref().unwrap();
        generate_batch(set_data.afile2x_mut(), previous_set_item.afile2x() , weight1, next_set_item.afile2x() , weight2);
        generate_batch(set_data.bfile_mut() , previous_set_item.bfile() , weight1, next_set_item.bfile() , weight2);
        generate_batch(set_data.cfile_mut() , previous_set_item.cfile() , weight1, next_set_item.cfile() , weight2);
        generate_batch(set_data.dfile_mut() , previous_set_item.dfile() , weight1, next_set_item.dfile() , weight2);
        generate_batch(set_data.diag8_mut() , previous_set_item.diag8() , weight1, next_set_item.diag8() , weight2);
        generate_batch(set_data.diag7_mut() , previous_set_item.diag7() , weight1, next_set_item.diag7() , weight2);
        generate_batch(set_data.diag6_mut() , previous_set_item.diag6() , weight1, next_set_item.diag6() , weight2);
        generate_batch(set_data.diag5_mut() , previous_set_item.diag5() , weight1, next_set_item.diag5() , weight2);
        generate_batch(set_data.diag4_mut() , previous_set_item.diag4() , weight1, next_set_item.diag4() , weight2);
        generate_batch(set_data.corner33_mut() , previous_set_item.corner33() , weight1, next_set_item.corner33() , weight2);
        generate_batch(set_data.corner52_mut() , previous_set_item.corner52() , weight1, next_set_item.corner52() , weight2);
    }
    let coeff_set = &mut state.set[curr_stage as usize];

    coeff_set.data = Some(set_data);
}
/*
   LOAD_SET
   Performs linear interpolation between the nearest stages to
   obtain the feature values for the stage in question.
   Also calculates the offset pointers to the last elements in each block
   (used for the inverted patterns when white is to move).
*/
pub fn load_set(index: i32, state: &mut CoeffState) {
    // let set_item: &CoeffSet = &mut state.set[(index as usize)];
    if state.set[(index as usize)].permanent == 0 {
        let mut weight1 = 0;
        let mut weight2 = 0;
        let mut prev = state.set[(index as usize)].prev;
        let mut next = state.set[(index as usize)].next;
        if prev == next {
            weight1 = 1;
            weight2 = 1;
        } else {
            weight1 = next - index;
            weight2 = index - prev;
        }
        let total_weight = weight1 + weight2;

        // let next_set_item = &state.set[next as usize];
        // let set_item: &mut CoeffSet = &mut state.set[(index as usize)];
        state.set[(index as usize)].constant = ((weight1 * state.set[prev as usize].constant as i32 +
            weight2 * state.set[next as usize].constant as i32) /
            total_weight) as i16;
        state.set[(index as usize)].parity = ((weight1 * state.set[prev as usize].parity as i32 +
            weight2 * state.set[next as usize].parity as i32) /
            total_weight) as i16;
        state.set[(index as usize)].parity_constant[0] = state.set[(index as usize)].constant;
        state.set[(index as usize)].parity_constant[1] = state.set[(index as usize)].constant + state.set[(index as usize)].parity;

        allocate_set(index, state, Some((prev, next, weight1, weight2)));

        // let previous_set_item = &state.set[prev as usize];
        // let next_set_item = &state.set[next as usize];
        //  generate_batch(set_data.corner52, previous_set_item.corner52, weight1, next_set_item.corner52, weight2);
        //  .... this was here before
    }
    state.set[(index as usize)].loaded = 1;
}

/*
   PATTERN_EVALUATION
   Calculates the static evaluation of the position using
   the statistically optimized pattern tables.
*/

pub fn pattern_evaluation(side_to_move: i32, board_state_: &mut BoardState, moves_state_: &MovesState, coeff_state_: &mut CoeffState) -> i32 {
    /* Any player wiped out? Game over then... */
    let mut eval_phase: i32 = 0;
    if board_state_.piece_count[0][moves_state_.disks_played as usize] ==
        0 as i32 {
        if side_to_move == 0 as i32 {
            return -(29000 as i32 + 64 as i32)
        } else { return 29000 as i32 + 64 as i32 }
    } else {
        if board_state_.piece_count[2][moves_state_.disks_played as usize] ==
            0 as i32 {
            if side_to_move == 0 as i32 {
                return 29000 as i32 + 64 as i32
            } else { return -(29000 as i32 + 64 as i32) }
        }
    }
    /* Load and/or initialize the pattern coefficients */
    eval_phase = coeff_state_.eval_map[moves_state_.disks_played as usize];
    if coeff_state_.set[eval_phase as usize].loaded == 0 {
        load_set(eval_phase, coeff_state_);
    }
    constant_and_parity_feature(side_to_move, moves_state_.disks_played, &mut board_state_.board, &mut coeff_state_.set[eval_phase as usize])
}

pub fn post_init_coeffs(state: &mut CoeffState) {
    /* For each of number of disks played, decide on what set of evaluation
           patterns to use.
           The following rules apply:
           - Stages from the tuning are used as evaluation stages
           - Intermediate evaluation stages are introduced 2 stages before
           each tuning stage.
           - Other stages are mapped onto the next evaluation stage
           (which may be either from the tuning or an intermediate stage).
        */
    let mut i = 0;
    while i < state.stage[0] {
        state.eval_map[i as usize] = state.stage[0];
        i += 1
    }
    i = 0;
    while i < state.stage_count {
        state.eval_map[state.stage[i as usize] as usize] = state.stage[i as usize];
        i += 1
    }
    let mut subsequent_stage = 60;
    i = subsequent_stage;
    while i >= state.stage[0] {
        if state.eval_map[i as usize] == i {
            subsequent_stage = i
        } else if i == subsequent_stage - 2 as i32 {
            state.eval_map[i as usize] = i;
            subsequent_stage = i
        } else { state.eval_map[i as usize] = subsequent_stage }
        i -= 1
    };
}


/*
   UNPACK_BATCH
   Reads feature values for one specific pattern
*/
pub fn unpack_batch<FE: FrontEnd, S:FnMut() -> i16>(item: &mut [i16],
                                                           mirror: Option<&[i32]>,
                                                           next_word: &mut S) {
    let count = item.len();
    let mut buffer = &mut vec![0; count as usize];
    let mut buffer = buffer.as_mut_slice();
    /* Unpack the coefficient block where the score is scaled
       so that 512 units corresponds to one disk. */
    let mut i = 0;
    while i < count {
        if mirror.is_none() || *mirror.unwrap().offset(i as isize) == i as i32 {
            let i1 = next_word();
            *buffer.offset(i as isize) =
                (i1 as i32 / 4 as i32) as
                    i16
        } else {
            *buffer.offset(i as isize) =
                *buffer.offset(*mirror.unwrap().offset(i as isize) as isize)
        }
        i += 1
    }
    i = 0;
    while i < count {
        *item.offset(i as isize) = *buffer.offset(i as isize);
        i += 1
    }
    if let Some(mirror) = mirror {
        i = 0;
        while i < count {
            if *item.offset(i as isize) as i32 !=
                *item.offset(*mirror.offset(i as isize) as isize) as
                    i32 {
                let first_mirror_offset = *mirror.offset(i as isize);
                let first_item = *item.offset(i as isize) as i32;
                let second_item = *item.offset(first_mirror_offset as isize) as i32;

                FE::report_mirror_symetry_error(count as i32, i as i32, first_mirror_offset, first_item, second_item);
                exit(1 as i32);
            }
            i += 1
        }
    }
}
/*
   UNPACK_COEFFS
   Reads all feature values for a certain stage. To take care of
   symmetric patterns, mirror tables are calculated.
*/
pub fn unpack_coeffs<FE: FrontEnd, S: FnMut() -> i16 >(next_word: &mut S, state: &mut CoeffState) {

    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut mirror_pattern: i32 = 0;
    let mut row: [i32; 10] = [0; 10];

    /* Allocate the memory needed for the temporary mirror maps from the
       heap rather than the stack to reduce memory requirements. */
    let mut map_mirror3 = vec![0; 27];
    let mut map_mirror4 = vec![0; 81];
    let mut map_mirror5 = vec![0; 243];
    let mut map_mirror6 = vec![0; 729];
    let mut map_mirror7 = vec![0; 2187];
    let mut map_mirror8 = vec![0; 6561];
    let mut map_mirror33 = vec![0; 19683];
    let mut map_mirror8x2 = vec![0; 59049];

    /* Build the pattern tables for 8*1-patterns */
    let mut i = 0;
    while i < 6561 as i32 {
        mirror_pattern = 0;
        j = 0;
        while j < 8 as i32 {
            mirror_pattern +=
                row[j as usize] * pow3((7 as i32 - j) as usize);
            j += 1
        }
        /* Create the symmetry map */
        *map_mirror8.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        odometer_principle(&mut row, 8);
        i += 1
    }
    /* Build the tables for 7*1-patterns */
    i = 0;
    while i < 7 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 2187 as i32 {
        mirror_pattern = 0;
        j = 0;
        while j < 7 as i32 {
            mirror_pattern +=
                row[j as usize] * pow3((6 as i32 - j) as usize);
            j += 1
        }
        *map_mirror7.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        odometer_principle(&mut row, 7);
        i += 1
    }
    /* Build the tables for 6*1-patterns */
    i = 0;
    while i < 6 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 729 as i32 {
        mirror_pattern = 0;
        j = 0;
        while j < 6 as i32 {
            mirror_pattern +=
                row[j as usize] * pow3((5 as i32 - j) as usize);
            j += 1
        }
        *map_mirror6.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        odometer_principle(&mut row, 6);
        i += 1
    }
    /* Build the tables for 5*1-patterns */
    i = 0;
    while i < 5 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 243 as i32 {
        mirror_pattern = 0;
        j = 0;
        while j < 5 as i32 {
            mirror_pattern +=
                row[j as usize] * pow3((4 as i32 - j) as usize);
            j += 1
        }
        *map_mirror5.offset(i as isize) =
            if mirror_pattern < i { mirror_pattern } else { i };
        /* Next configuration */
        odometer_principle(&mut row, 5);
        i += 1
    }
    /* Build the tables for 4*1-patterns */
    i = 0;
    while i < 4 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 81 as i32 {
        mirror_pattern = 0;
        j = 0;
        while j < 4 as i32 {
            mirror_pattern +=
                row[j as usize] * pow3((3 as i32 - j) as usize);
            j += 1
        }
        *map_mirror4.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        odometer_principle(&mut row, 4);
        i += 1
    }
    /* Build the tables for 3*1-patterns */
    i = 0;
    while i < 3 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 27 as i32 {
        mirror_pattern = 0;
        j = 0;
        while j < 3 as i32 {
            mirror_pattern +=
                row[j as usize] * pow3((2 as i32 - j) as usize);
            j += 1
        }
        *map_mirror3.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0;
        odometer_principle(&mut row, 3);
        i += 1
    }
    /* Build the tables for 5*2-patterns */
    /* --- none needed --- */
    /* Build the tables for edge2X-patterns */
    i = 0;
    while i < 6561 as i32 {
        j = 0;
        while j < 3 as i32 {
            k = 0;
            while k < 3 as i32 {
                *map_mirror8x2.offset((i + 6561 as i32 * j +
                    19683 as i32 * k) as isize)
                    =
                    if flip8[i as usize]  as i32 + 6561 as i32 * k +
                        19683 as i32 * j <
                        i + 6561 as i32 * j +
                            19683 as i32 * k {
                        (flip8[i as usize] as i32 + 6561 as i32 * k) +
                            19683 as i32 * j
                    } else {
                        (i + 6561 as i32 * j) +
                            19683 as i32 * k
                    };
                k += 1
            }
            j += 1
        }
        i += 1
    }
    /* Build the tables for 3*3-patterns */
    i = 0;
    while i < 9 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 19683 as i32 {
        mirror_pattern =
            row[0] +
                3 as i32 * row[3] +
                9 as i32 * row[6] +
                27 as i32 * row[1] +
                81 as i32 * row[4] +
                243 as i32 * row[7] +
                729 as i32 * row[2] +
                2187 as i32 * row[5] +
                6561 as i32 * row[8];
        *map_mirror33.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0;
        odometer_principle(&mut row, 9);
        i += 1
    }
    /* Read and unpack - using symmetries - the coefficient tables. */
    i = 0;
    while i < state.stage_count - 1 as i32 {
        let stage_set = &mut state.set[state.stage[i as usize] as usize];
        stage_set.constant = (next_word() / 4);
        stage_set.parity = (next_word() / 4);
        stage_set.parity_constant[0] = stage_set.constant;
        stage_set.parity_constant[1] = (stage_set.constant as i32 + stage_set.parity as i32) as i16;
        let mut stage_set = stage_set.data.as_mut().unwrap();
        unpack_batch::<FE, S>(stage_set.afile2x_mut() , Some(&map_mirror8x2), next_word);
        unpack_batch::<FE, S>(stage_set.bfile_mut() , Some(&map_mirror8), next_word);
        unpack_batch::<FE, S>(stage_set.cfile_mut() , Some(&map_mirror8), next_word);
        unpack_batch::<FE, S>(stage_set.dfile_mut() , Some(&map_mirror8), next_word);
        unpack_batch::<FE, S>(stage_set.diag8_mut() , Some(&map_mirror8), next_word);
        unpack_batch::<FE, S>(stage_set.diag7_mut() , Some(&map_mirror7), next_word);
        unpack_batch::<FE, S>(stage_set.diag6_mut() , Some(&map_mirror6), next_word);
        unpack_batch::<FE, S>(stage_set.diag5_mut() , Some(&map_mirror5), next_word);
        unpack_batch::<FE, S>(stage_set.diag4_mut() , Some(&map_mirror4), next_word);
        unpack_batch::<FE, S>(stage_set.corner33_mut() , Some(&map_mirror33), next_word);
        unpack_batch::<FE, S>(stage_set.corner52_mut() , None, next_word);
        i += 1
    }
}

pub fn process_coeffs_from_fn_source<FE: FrontEnd, Source:CoeffSource>(mut coeffs: Source, coeff_state_: &mut CoeffState) {
    let mut next_word = || coeffs.next_word();
    /* Read the different stages for which the evaluation function
       was tuned and mark the other stages with pointers to the previous
       and next stages. */
    let mut i = 0;
    while i <= 60 {
        let coeff_set = &mut coeff_state_.set[i as usize];
        coeff_set.permanent = 0;
        coeff_set.loaded = 0;
        i += 1
    }
    coeff_state_.stage_count = next_word() as i32;
    let mut i = 0;
    let mut j = 0;
    let mut curr_stage = 0;
    while i < coeff_state_.stage_count - 1 {
        coeff_state_.stage[i as usize] = next_word() as i32;
        curr_stage = coeff_state_.stage[i as usize];
        if i == 0 {
            j = 0;
            while j < coeff_state_.stage[0] {
                let coeff_set = &mut coeff_state_.set[j as usize];
                coeff_set.prev = coeff_state_.stage[0];
                coeff_set.next = coeff_state_.stage[0];
                j += 1
            }
        } else {
            j = coeff_state_.stage[(i - 1 as i32) as usize];
            while j < coeff_state_.stage[i as usize] {
                let coeff_set = &mut coeff_state_.set[j as usize];
                coeff_set.prev = coeff_state_.stage[(i - 1 as i32) as usize];
                coeff_set.next = coeff_state_.stage[i as usize];
                j += 1
            }
        }
        coeff_state_.set[curr_stage as usize].permanent = 1;
        allocate_set(curr_stage, coeff_state_, None);
        i += 1
    }
    coeff_state_.stage[(coeff_state_.stage_count - 1) as usize] = 60;
    j = coeff_state_.stage[(coeff_state_.stage_count - 2) as usize];
    while j < 60 {
        let coeff_set = &mut coeff_state_.set[j as usize];
        coeff_set.prev = coeff_state_.stage[(coeff_state_.stage_count - 2 as i32) as usize];
        coeff_set.next = 60;
        j += 1
    }
    coeff_state_.set[60].permanent = 1;
    allocate_set(60, coeff_state_, None);
    /* Read the pattern values */
    unpack_coeffs::<FE, _>(&mut next_word, coeff_state_);
}


pub fn init_coeffs_calculate_terminal_patterns(state: &mut CoeffState) {
    let coeff_set = &mut state.set[60];
    terminal_patterns(coeff_set);
    coeff_set.constant = 0;
    coeff_set.parity = 0;
    coeff_set.parity_constant[0] = coeff_set.constant;
    coeff_set.parity_constant[1] = (coeff_set.constant as i32 + coeff_set.parity as i32) as i16;
}
