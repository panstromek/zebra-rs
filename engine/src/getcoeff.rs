use crate::src::globals::{board_state, Board};
use crate::src::moves::disks_played;
use ::patterns::{flip8, pow3};
use crate::src::stubs::{floor};
use crate::src::error::{FrontEnd};
use std::ffi::c_void;
use std::process::exit;
use engine_traits::{CoeffSource, Offset};
use crate::src::globals;
use coeff::{constant_and_parity_feature, CoeffSet, terminal_patterns, CoeffSetData};
use std::ptr::null_mut;
use std::slice::from_raw_parts_mut;

pub struct CoeffAdjustments {
    pub disc_adjust: f64,
    pub edge_adjust: f64,
    pub corner_adjust: f64,
    pub x_adjust: f64,
}

#[derive(Clone)]
#[repr(C)]
pub struct AllocationBlock {
    pub afile2x_block: [i16; 59049],
    pub bfile_block: [i16; 6561],
    pub cfile_block: [i16; 6561],
    pub dfile_block: [i16; 6561],
    pub diag8_block: [i16; 6561],
    pub diag7_block: [i16; 2187],
    pub diag6_block: [i16; 729],
    pub diag5_block: [i16; 243],
    pub diag4_block: [i16; 81],
    pub corner33_block: [i16; 19683],
    pub corner52_block: [i16; 59049],
}
static mut stage_count___: i32 = 0;
static mut block_count___: i32 = 0;
static mut stage___: [i32; 61] = [0; 61];
static mut block_allocated___: [bool; 200] = [false; 200];
static mut eval_map___: [i32; 61] = [0; 61];

const EMPTY_ALLOC_BLOCK: Option<Box<AllocationBlock>> = None;
const NEW_COEFF_SET : CoeffSet = CoeffSet::new();

static mut block_list___: [Option<Box<AllocationBlock>>; 200] = [EMPTY_ALLOC_BLOCK; 200];

pub static mut set___: [CoeffSet; 61] = [NEW_COEFF_SET; 61];
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
   INIT_MEMORY_HANDLER
   Mark all blocks in the memory arena as "not used".
*/
pub unsafe fn init_memory_handler() {
    block_count___ = 0;
    block_allocated___ = [false; 200];
}


/*
  DISC_COUNT_ADJUSTMENT
*/
/* Adjust the coefficients so as to reflect the encouragement for
       having lots of discs */
pub unsafe fn eval_adjustment(disc_adjust: f64,
                          edge_adjust: f64,
                          corner_adjust: f64,
                          x_adjust: f64) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut adjust: i32 = 0;
    let mut row: [i32; 10] = [0; 10];
    i = 0;
    while i < stage_count___ - 1 as i32 {
        /* Bonuses for having more discs */
        j = 0;
        let stage_set = set___[stage___[i as usize] as usize].data.as_mut().unwrap();
        let sixty_set = set___[60].data.as_mut().unwrap();
        while j < 59049 as i32 {
            let ref mut fresh2 =
                *(stage_set.afile2x  as &mut[i16]).offset(j as isize);
            *fresh2 =
                (*fresh2 as f64 +
                    *sixty_set.afile2x.offset(j as isize) as i32
                        as f64 * disc_adjust) as i16;
            let ref mut fresh3 =
                *(stage_set.corner52  as &mut[i16]).offset(j as isize);
            *fresh3 =
                (*fresh3 as f64 +
                    *sixty_set.corner52.offset(j as isize) as
                        i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 19683 as i32 {
            let ref mut fresh4 =
                *(stage_set.corner33 as &mut [i16]).offset(j as isize);
            *fresh4 =
                (*fresh4 as f64 +
                    *sixty_set.corner33.offset(j as isize) as
                        i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 6561 as i32 {
            let ref mut fresh5 =
                *(stage_set.bfile  as &mut[i16]).offset(j as isize);
            *fresh5 =
                (*fresh5 as f64 +
                    *sixty_set.bfile.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            let ref mut fresh6 =
                *(stage_set.cfile  as &mut[i16]).offset(j as isize);
            *fresh6 =
                (*fresh6 as f64 +
                    *sixty_set.cfile.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            let ref mut fresh7 =
                *(stage_set.dfile  as &mut[i16]).offset(j as isize);
            *fresh7 =
                (*fresh7 as f64 +
                    *sixty_set.dfile.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            let ref mut fresh8 =
                *(stage_set.diag8  as &mut[i16]).offset(j as isize);
            *fresh8 =
                (*fresh8 as f64 +
                    *sixty_set.diag8.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 2187 as i32 {
            let ref mut fresh9 =
                *(stage_set.diag7  as &mut[i16]).offset(j as isize);
            *fresh9 =
                (*fresh9 as f64 +
                    *sixty_set.diag7.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 729 as i32 {
            let ref mut fresh10 =
                *(stage_set.diag6  as &mut[i16]).offset(j as isize);
            *fresh10 =
                (*fresh10 as f64 +
                    *sixty_set.diag6.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 243 as i32 {
            let ref mut fresh11 =
                *(stage_set.diag5  as &mut[i16]).offset(j as isize);
            *fresh11 =
                (*fresh11 as f64 +
                    *sixty_set.diag5.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 81 as i32 {
            let ref mut fresh12 =
                *(stage_set.diag4 as &mut[i16]).offset(j as isize);
            *fresh12 =
                (*fresh12 as f64 +
                    *sixty_set.diag4.offset(j as isize)
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
                *(stage_set.afile2x as &mut [i16]).offset(j as isize);
            *fresh13 = (*fresh13 as i32 + adjust) as i16;
            /* Next configuration */
            k = 0;
            loop  {
                /* The odometer principle */
                row[k as usize] += 1;
                if row[k as usize] == 3 as i32 {
                    row[k as usize] = 0 as i32
                }
                k += 1;
                if !(row[(k - 1 as i32) as usize] == 0 as i32
                    && k < 10 as i32) {
                    break ;
                }
            }
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
pub fn remove_specific_coeffs(coeff_set: &mut CoeffSet, block_allocated_: &mut [bool; 200]) {
    let coeff_set = coeff_set;
    if coeff_set.loaded != 0 {
        if coeff_set.permanent == 0 {
            block_allocated_[coeff_set.block as usize] = false;
        }
        coeff_set.loaded = 0
    };
}
/*
   REMOVE_COEFFS
   Removes pattern tables which have gone out of scope from memory.
*/

pub unsafe fn remove_coeffs(phase: i32) {
    let mut i: i32 = 0;
    while i < phase {
        remove_specific_coeffs(&mut set___[i as usize], &mut block_allocated___);
        i += 1
    };
}
/*
   CLEAR_COEFFS
   Remove all coefficients loaded from memory.
*/
pub unsafe fn clear_coeffs(set_: &mut [CoeffSet; 61]) {
    remove_coeffs(set_.len() as i32);
}


/*
   FIND_MEMORY_BLOCK
   Maintains an internal memory handler to boost
   performance and avoid heap fragmentation.
*/
pub unsafe fn find_memory_block<FE: FrontEnd>(coeff_set: &mut CoeffSet) -> i32 {
    let mut found_free = 0;
    let mut free_block = -1;
    let mut i = 0;
    while i < block_count___ && found_free == 0 {
        if block_allocated___[i as usize] == false {
            found_free = 1;
            free_block = i
        }
        i += 1
    }
    if found_free == 0 {
        if block_count___ < 200 as i32 {
            block_list___[block_count___ as usize] = Some(Box::new(AllocationBlock {
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
            }));
        }
        if block_count___ == 200 || block_list___[block_count___ as usize].is_none() {
            FE::memory_allocation_failure(block_count___);
        }
        free_block = block_count___;
        block_count___ += 1
    }

    let mut block_list_item = (block_list___[free_block as usize]).as_mut().unwrap();
    coeff_set.data = Some(CoeffSetData {
        afile2x: &mut block_list_item.afile2x_block,
        bfile: &mut block_list_item.bfile_block,
        cfile: &mut block_list_item.cfile_block,
        dfile: &mut block_list_item.dfile_block,
        diag8: &mut block_list_item.diag8_block,
        diag7: &mut block_list_item.diag7_block,
        diag6: &mut block_list_item.diag6_block,
        diag5: &mut block_list_item.diag5_block,
        diag4: &mut block_list_item.diag4_block,
        corner33: &mut block_list_item.corner33_block,
        corner52: &mut block_list_item.corner52_block,
    });
    block_allocated___[free_block as usize] = true;
    return free_block;
}
/*
   ALLOCATE_SET
   Finds memory for all patterns belonging to a certain stage.
*/
pub unsafe fn allocate_set<FE: FrontEnd>(coeff_set: &mut CoeffSet) {
    coeff_set.block = find_memory_block::<FE>(coeff_set);
}
/*
   LOAD_SET
   Performs linear interpolation between the nearest stages to
   obtain the feature values for the stage in question.
   Also calculates the offset pointers to the last elements in each block
   (used for the inverted patterns when white is to move).
*/
pub unsafe fn load_set<FE: FrontEnd>(index: i32, set_item: &mut CoeffSet) {
    if set_item.permanent == 0 {
        let mut weight1 = 0;
        let mut weight2 = 0;
        let mut prev = set_item.prev;
        let mut next = set_item.next;
        if prev == next {
            weight1 = 1;
            weight2 = 1;
        } else {
            weight1 = next - index;
            weight2 = index - prev;
        }
        let total_weight = weight1 + weight2;
        let previous_set_item = &mut set___[prev as usize];
        let next_set_item = &mut set___[next as usize];
        set_item.constant = ((weight1 * previous_set_item.constant as i32 +
                weight2 * next_set_item.constant as i32) /
                total_weight) as i16;
        set_item.parity = ((weight1 * previous_set_item.parity as i32 +
                weight2 * next_set_item.parity as i32) /
                total_weight) as i16;
        set_item.parity_constant[0] = set_item.constant;
        set_item.parity_constant[1] = set_item.constant + set_item.parity;
        allocate_set::<FE>(set_item);
        let set_item = set_item.data.as_mut().unwrap();
        let previous_set_item = previous_set_item.data.as_mut().unwrap();
        let next_set_item = next_set_item.data.as_mut().unwrap();
        generate_batch(set_item.afile2x, previous_set_item.afile2x, weight1, next_set_item.afile2x, weight2);
        generate_batch(set_item.bfile, previous_set_item.bfile, weight1, next_set_item.bfile, weight2);
        generate_batch(set_item.cfile, previous_set_item.cfile, weight1, next_set_item.cfile, weight2);
        generate_batch(set_item.dfile, previous_set_item.dfile, weight1, next_set_item.dfile, weight2);
        generate_batch(set_item.diag8, previous_set_item.diag8, weight1, next_set_item.diag8, weight2);
        generate_batch(set_item.diag7, previous_set_item.diag7, weight1, next_set_item.diag7, weight2);
        generate_batch(set_item.diag6, previous_set_item.diag6, weight1, next_set_item.diag6, weight2);
        generate_batch(set_item.diag5, previous_set_item.diag5, weight1, next_set_item.diag5, weight2);
        generate_batch(set_item.diag4, previous_set_item.diag4, weight1, next_set_item.diag4, weight2);
        generate_batch(set_item.corner33, previous_set_item.corner33, weight1, next_set_item.corner33, weight2);
        generate_batch(set_item.corner52, previous_set_item.corner52, weight1, next_set_item.corner52, weight2);
    }
    set_item.loaded = 1;
}

/*
   PATTERN_EVALUATION
   Calculates the static evaluation of the position using
   the statistically optimized pattern tables.
*/

pub unsafe fn pattern_evaluation<FE: FrontEnd>(side_to_move: i32) -> i32 {
    let mut eval_phase: i32 = 0;
    /* Any player wiped out? Game over then... */
    if board_state.piece_count[0][disks_played as usize] ==
        0 as i32 {
        if side_to_move == 0 as i32 {
            return -(29000 as i32 + 64 as i32)
        } else { return 29000 as i32 + 64 as i32 }
    } else {
        if board_state.piece_count[2][disks_played as usize] ==
            0 as i32 {
            if side_to_move == 0 as i32 {
                return 29000 as i32 + 64 as i32
            } else { return -(29000 as i32 + 64 as i32) }
        }
    }
    /* Load and/or initialize the pattern coefficients */
    eval_phase = eval_map___[disks_played as usize];
    if set___[eval_phase as usize].loaded == 0 {
        load_set::<FE>(eval_phase, &mut set___[(eval_phase as usize)]);
    }
    constant_and_parity_feature(side_to_move, disks_played, &mut globals::board_state.board, &mut set___[eval_phase as usize])
}

pub unsafe fn post_init_coeffs() {
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
    while i < stage___[0] {
        eval_map___[i as usize] = stage___[0];
        i += 1
    }
    i = 0;
    while i < stage_count___ {
        eval_map___[stage___[i as usize] as usize] = stage___[i as usize];
        i += 1
    }
    let mut subsequent_stage = 60;
    i = subsequent_stage;
    while i >= stage___[0] {
        if eval_map___[i as usize] == i {
            subsequent_stage = i
        } else if i == subsequent_stage - 2 as i32 {
            eval_map___[i as usize] = i;
            subsequent_stage = i
        } else { eval_map___[i as usize] = subsequent_stage }
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
pub unsafe fn unpack_coeffs<FE: FrontEnd, S: FnMut() -> i16 >(next_word: &mut S) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut mirror_pattern: i32 = 0;
    let mut row: [i32; 10] = [0; 10];

    let mut map_mirror3 = vec![0; 27];
    let mut map_mirror4 = vec![0; 81];
    let mut map_mirror5 = vec![0; 243];
    let mut map_mirror6 = vec![0; 729];
    let mut map_mirror7 = vec![0; 2187];
    let mut map_mirror8 = vec![0; 6561];
    let mut map_mirror33 = vec![0; 19683];
    let mut map_mirror8x2 = vec![0; 59049];

    /* Allocate the memory needed for the temporary mirror maps from the
       heap rather than the stack to reduce memory requirements. */
    let mut map_mirror3 = map_mirror3.as_mut_slice();
    let mut map_mirror4 = map_mirror4.as_mut_slice();
    let mut map_mirror5 = map_mirror5.as_mut_slice();
    let mut map_mirror6 = map_mirror6.as_mut_slice();
    let mut map_mirror7 = map_mirror7.as_mut_slice();
    let mut map_mirror8 = map_mirror8.as_mut_slice();
    let mut map_mirror33 = map_mirror33.as_mut_slice();
    let mut map_mirror8x2 = map_mirror8x2.as_mut_slice();
    /* Build the pattern tables for 8*1-patterns */
    i = 0;
    while i < 8 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 6561 as i32 {
        mirror_pattern = 0;
        j = 0;
        while j < 8 as i32 {
            mirror_pattern +=
                row[j as usize] * pow3[(7 as i32 - j) as usize];
            j += 1
        }
        /* Create the symmetry map */
        *map_mirror8.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 8 as i32) {
                break ;
            }
        }
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
                row[j as usize] * pow3[(6 as i32 - j) as usize];
            j += 1
        }
        *map_mirror7.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 7 as i32) {
                break ;
            }
        }
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
                row[j as usize] * pow3[(5 as i32 - j) as usize];
            j += 1
        }
        *map_mirror6.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 6 as i32) {
                break ;
            }
        }
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
                row[j as usize] * pow3[(4 as i32 - j) as usize];
            j += 1
        }
        *map_mirror5.offset(i as isize) =
            if mirror_pattern < i { mirror_pattern } else { i };
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 5 as i32) {
                break ;
            }
        }
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
                row[j as usize] * pow3[(3 as i32 - j) as usize];
            j += 1
        }
        *map_mirror4.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 4 as i32) {
                break ;
            }
        }
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
                row[j as usize] * pow3[(2 as i32 - j) as usize];
            j += 1
        }
        *map_mirror3.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 3 as i32) {
                break ;
            }
        }
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
                    if flip8[i as usize] + 6561 as i32 * k +
                        19683 as i32 * j <
                        i + 6561 as i32 * j +
                            19683 as i32 * k {
                        (flip8[i as usize] + 6561 as i32 * k) +
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
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 9 as i32) {
                break ;
            }
        }
        i += 1
    }
    /* Read and unpack - using symmetries - the coefficient tables. */
    i = 0;
    while i < stage_count___ - 1 as i32 {
        let stage_set = &mut set___[stage___[i as usize] as usize];
        stage_set.constant = (next_word() / 4);
        stage_set.parity = (next_word() / 4);
        stage_set.parity_constant[0] = stage_set.constant;
        stage_set.parity_constant[1] = (stage_set.constant as i32 + stage_set.parity as i32) as i16;
        let mut stage_set = stage_set.data.as_mut().unwrap();
        unpack_batch::<FE, S>(stage_set.afile2x, Some(&map_mirror8x2), next_word);
        unpack_batch::<FE, S>(stage_set.bfile, Some(&map_mirror8), next_word);
        unpack_batch::<FE, S>(stage_set.cfile, Some(&map_mirror8), next_word);
        unpack_batch::<FE, S>(stage_set.dfile, Some(&map_mirror8), next_word);
        unpack_batch::<FE, S>(stage_set.diag8, Some(&map_mirror8), next_word);
        unpack_batch::<FE, S>(stage_set.diag7, Some(&map_mirror7), next_word);
        unpack_batch::<FE, S>(stage_set.diag6, Some(&map_mirror6), next_word);
        unpack_batch::<FE, S>(stage_set.diag5, Some(&map_mirror5), next_word);
        unpack_batch::<FE, S>(stage_set.diag4, Some(&map_mirror4), next_word);
        unpack_batch::<FE, S>(stage_set.corner33, Some(&map_mirror33), next_word);
        unpack_batch::<FE, S>(stage_set.corner52, None, next_word);
        i += 1
    }
}

pub unsafe fn process_coeffs_from_fn_source<FE: FrontEnd, Source:CoeffSource>(mut coeffs: Source) {
    let mut next_word = || coeffs.next_word();
    /* Read the different stages for which the evaluation function
       was tuned and mark the other stages with pointers to the previous
       and next stages. */
    let mut i = 0;
    while i <= 60 {
        let coeff_set = &mut set___[i as usize];
        coeff_set.permanent = 0;
        coeff_set.loaded = 0;
        i += 1
    }
    stage_count___ = next_word() as i32;
    let mut i = 0;
    let mut j = 0;
    let mut curr_stage = 0;
    while i < stage_count___ - 1 {
        stage___[i as usize] = next_word() as i32;
        curr_stage = stage___[i as usize];
        if i == 0 {
            j = 0;
            while j < stage___[0] {
                let coeff_set = &mut set___[j as usize];
                coeff_set.prev = stage___[0];
                coeff_set.next = stage___[0];
                j += 1
            }
        } else {
            j = stage___[(i - 1 as i32) as usize];
            while j < stage___[i as usize] {
                let coeff_set = &mut set___[j as usize];
                coeff_set.prev = stage___[(i - 1 as i32) as usize];
                coeff_set.next = stage___[i as usize];
                j += 1
            }
        }
        set___[curr_stage as usize].permanent = 1;
        allocate_set::<FE>( &mut set___[curr_stage as usize]);
        i += 1
    }
    stage___[(stage_count___ - 1) as usize] = 60;
    j = stage___[(stage_count___ - 2) as usize];
    while j < 60 {
        let coeff_set = &mut set___[j as usize];
        coeff_set.prev = stage___[(stage_count___ - 2 as i32) as usize];
        coeff_set.next = 60;
        j += 1
    }
    set___[60].permanent = 1;
    allocate_set::<FE>(&mut set___[60]);
    /* Read the pattern values */
    unpack_coeffs::<FE, _>(&mut next_word);
}


pub unsafe fn init_coeffs_calculate_patterns() {
    let coeff_set = &mut set___[60];
    terminal_patterns(coeff_set);
    coeff_set.constant = 0;
    coeff_set.parity = 0;
    coeff_set.parity_constant[0] = coeff_set.constant;
    coeff_set.parity_constant[1] = (coeff_set.constant as i32 + coeff_set.parity as i32) as i16;
}
