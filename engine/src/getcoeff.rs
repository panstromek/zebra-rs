use crate::src::globals::{piece_count, Board};
use crate::src::moves::disks_played;
use ::patterns::{flip8, pow3};
use crate::src::stubs::{floor};
use crate::src::safemem::safe_malloc;
use crate::src::error::{FrontEnd};
use std::ffi::c_void;
use std::process::exit;
use engine_traits::CoeffSource;
use crate::src::globals;
use coeff::{constant_and_parity_feature, CoeffSet, terminal_patterns};

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
static mut stage_count: i32 = 0;
static mut block_count: i32 = 0;
static mut stage: [i32; 61] = [0; 61];
static mut block_allocated: [i32; 200] = [0; 200];
static mut eval_map: [i32; 61] = [0; 61];
macro_rules! arr_of {
    ($count:literal, $expr:expr) => {
            [$]
    };
}
static mut block_list: [Option<Box<AllocationBlock>>; 200] = [
    // This is just incredibly stupid, but I don't know any other way to do this,
    // because this type doesn't implement copy
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None,None,None,None,None,
    None,None,None,None,None,None,None,None
];
static mut set: [CoeffSet; 61] = [CoeffSet {
    permanent: 0,
    loaded: 0,
    prev: 0,
    next: 0,
    block: 0,
    parity_constant: [0; 2],
    parity: 0,
    constant: 0,
    afile2x: 0 as _,
    bfile: 0 as _,
    cfile: 0 as _,
    dfile: 0 as _,
    diag8: 0 as _,
    diag7: 0 as _,
    diag6: 0 as _,
    diag5: 0 as _,
    diag4: 0 as _,
    corner33: 0 as _,
    corner52: 0 as _,
    afile2x_last: 0 as _,
    bfile_last: 0 as _,
    cfile_last: 0 as _,
    dfile_last: 0 as _,
    diag8_last: 0 as _,
    diag7_last: 0 as _,
    diag6_last: 0 as _,
    diag5_last: 0 as _,
    diag4_last: 0 as _,
    corner33_last: 0 as _,
    corner52_last: 0 as _,
    alignment_padding: [0; 12],
}; 61];

pub unsafe fn generate_batch(target: *mut i16,
                         count: usize,
                         source1: *mut i16,
                         weight1: i32,
                         source2: *mut i16,
                             weight2: i32) {
    generate_batch_(std::slice::from_raw_parts_mut(target, count),
                    std::slice::from_raw_parts(source1, count),
                    weight1,
                    std::slice::from_raw_parts(source2, count),
                    weight2
    );
}
/*
   GENERATE_BATCH
   Interpolates between two stages.
*/
fn generate_batch_(target: &mut [i16], source1: &[i16], weight1: i32, source2: &[i16], weight2: i32) {
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
   FREE_MEMORY_BLOCK
   Marks a memory block as no longer in use.
*/
pub unsafe fn free_memory_block(block: i32) {
    block_allocated[block as usize] = 0;
}
/*
   INIT_MEMORY_HANDLER
   Mark all blocks in the memory arena as "not used".
*/
pub unsafe fn init_memory_handler() {
    let mut i: i32 = 0;
    block_count = 0;
    i = 0;
    while i < 200 as i32 {
        block_allocated[i as usize] = 0;
        i += 1
    };
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
    while i < stage_count - 1 as i32 {
        /* Bonuses for having more discs */
        j = 0;
        while j < 59049 as i32 {
            let ref mut fresh2 =
                *set[stage[i as usize] as usize].afile2x.offset(j as isize);
            *fresh2 =
                (*fresh2 as f64 +
                    *set[60].afile2x.offset(j as isize) as i32
                        as f64 * disc_adjust) as i16;
            let ref mut fresh3 =
                *set[stage[i as usize] as usize].corner52.offset(j as isize);
            *fresh3 =
                (*fresh3 as f64 +
                    *set[60].corner52.offset(j as isize) as
                        i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 19683 as i32 {
            let ref mut fresh4 =
                *set[stage[i as usize] as usize].corner33.offset(j as isize);
            *fresh4 =
                (*fresh4 as f64 +
                    *set[60].corner33.offset(j as isize) as
                        i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 6561 as i32 {
            let ref mut fresh5 =
                *set[stage[i as usize] as usize].bfile.offset(j as isize);
            *fresh5 =
                (*fresh5 as f64 +
                    *set[60].bfile.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            let ref mut fresh6 =
                *set[stage[i as usize] as usize].cfile.offset(j as isize);
            *fresh6 =
                (*fresh6 as f64 +
                    *set[60].cfile.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            let ref mut fresh7 =
                *set[stage[i as usize] as usize].dfile.offset(j as isize);
            *fresh7 =
                (*fresh7 as f64 +
                    *set[60].dfile.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            let ref mut fresh8 =
                *set[stage[i as usize] as usize].diag8.offset(j as isize);
            *fresh8 =
                (*fresh8 as f64 +
                    *set[60].diag8.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 2187 as i32 {
            let ref mut fresh9 =
                *set[stage[i as usize] as usize].diag7.offset(j as isize);
            *fresh9 =
                (*fresh9 as f64 +
                    *set[60].diag7.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 729 as i32 {
            let ref mut fresh10 =
                *set[stage[i as usize] as usize].diag6.offset(j as isize);
            *fresh10 =
                (*fresh10 as f64 +
                    *set[60].diag6.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 243 as i32 {
            let ref mut fresh11 =
                *set[stage[i as usize] as usize].diag5.offset(j as isize);
            *fresh11 =
                (*fresh11 as f64 +
                    *set[60].diag5.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0;
        while j < 81 as i32 {
            let ref mut fresh12 =
                *set[stage[i as usize] as usize].diag4.offset(j as isize);
            *fresh12 =
                (*fresh12 as f64 +
                    *set[60].diag4.offset(j as isize)
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
                *set[stage[i as usize] as usize].afile2x.offset(j as isize);
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
pub unsafe fn remove_specific_coeffs(phase: i32) {
    if set[phase as usize].loaded != 0 {
        if set[phase as usize].permanent == 0 {
            free_memory_block(set[phase as usize].block);
        }
        set[phase as usize].loaded = 0 as i32
    };
}
/*
   REMOVE_COEFFS
   Removes pattern tables which have gone out of scope from memory.
*/

pub unsafe fn remove_coeffs(phase: i32) {
    let mut i: i32 = 0;
    i = 0;
    while i < phase { remove_specific_coeffs(i); i += 1 };
}
/*
   CLEAR_COEFFS
   Remove all coefficients loaded from memory.
*/

pub unsafe fn clear_coeffs() {
    let mut i: i32 = 0;
    i = 0;
    while i <= 60 as i32 { remove_specific_coeffs(i); i += 1 };
}



/*
   FIND_MEMORY_BLOCK
   Maintains an internal memory handler to boost
   performance and avoid heap fragmentation.
*/
pub unsafe fn find_memory_block<FE: FrontEnd>(afile2x: *mut *mut i16,
                                              bfile: *mut *mut i16,
                                              cfile: *mut *mut i16,
                                              dfile: *mut *mut i16,
                                              diag8: *mut *mut i16,
                                              diag7: *mut *mut i16,
                                              diag6: *mut *mut i16,
                                              diag5: *mut *mut i16,
                                              diag4: *mut *mut i16,
                                              corner33: *mut *mut i16,
                                              corner52: *mut *mut i16,
                                              index: i32)
                                              -> i32 {
    let mut i: i32 = 0;
    let mut found_free: i32 = 0;
    let mut free_block: i32 = 0;
    found_free = 0;
    free_block = -(1 as i32);
    i = 0;
    while i < block_count && found_free == 0 {
        if block_allocated[i as usize] == 0 {
            found_free = 1;
            free_block = i
        }
        i += 1
    }
    if found_free == 0 {
        if block_count < 200 as i32 {
            block_list[block_count as usize] = Some(Box::new(AllocationBlock {
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
        if block_count == 200 || block_list[block_count as usize].is_none() {
            let block_count_ = block_count;
            FE::memory_allocation_failure(block_count_);
        }
        free_block = block_count;
        block_count += 1
    }
    let mut block_list_item = (block_list[free_block as usize]).as_mut().unwrap();
    *afile2x = (*block_list_item).afile2x_block.as_mut_ptr();
    *bfile = (*block_list_item).bfile_block.as_mut_ptr();
    *cfile = (*block_list_item).cfile_block.as_mut_ptr();
    *dfile = (*block_list_item).dfile_block.as_mut_ptr();
    *diag8 = (*block_list_item).diag8_block.as_mut_ptr();
    *diag7 = (*block_list_item).diag7_block.as_mut_ptr();
    *diag6 = (*block_list_item).diag6_block.as_mut_ptr();
    *diag5 = (*block_list_item).diag5_block.as_mut_ptr();
    *diag4 = (*block_list_item).diag4_block.as_mut_ptr();
    *corner33 = (*block_list_item).corner33_block.as_mut_ptr();
    *corner52 = (*block_list_item).corner52_block.as_mut_ptr();
    block_allocated[free_block as usize] = 1;
    return free_block;
}
/*
   ALLOCATE_SET
   Finds memory for all patterns belonging to a certain stage.
*/
pub unsafe fn allocate_set<FE: FrontEnd>(index: i32) {
    let coeff_set = &mut set[index as usize];
    coeff_set.block = find_memory_block::<FE>(
        &mut coeff_set.afile2x,
        &mut coeff_set.bfile,
        &mut coeff_set.cfile,
        &mut coeff_set.dfile,
        &mut coeff_set.diag8,
        &mut coeff_set.diag7,
        &mut coeff_set.diag6,
        &mut coeff_set.diag5,
        &mut coeff_set.diag4,
        &mut coeff_set.corner33,
        &mut coeff_set.corner52,
        index);
}
/*
   LOAD_SET
   Performs linear interpolation between the nearest stages to
   obtain the feature values for the stage in question.
   Also calculates the offset pointers to the last elements in each block
   (used for the inverted patterns when white is to move).
*/
pub unsafe fn load_set<FE: FrontEnd>(index: i32) {
    let set_item = &mut set[(index as usize)];
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
        let previous_set_item = &mut set[prev as usize];
        let next_set_item = &mut set[next as usize];
        set_item.constant = ((weight1 * previous_set_item.constant as i32 +
                weight2 * next_set_item.constant as i32) /
                total_weight) as i16;
        set_item.parity = ((weight1 * previous_set_item.parity as i32 +
                weight2 * next_set_item.parity as i32) /
                total_weight) as i16;
        set_item.parity_constant[0] = set_item.constant;
        set_item.parity_constant[1] = set_item.constant + set_item.parity;
        allocate_set::<FE>(index);
        generate_batch(set_item.afile2x, 59049,
                       previous_set_item.afile2x, weight1,
                       next_set_item.afile2x, weight2);
        generate_batch(set_item.bfile, 6561,
                       previous_set_item.bfile, weight1,
                       next_set_item.bfile, weight2);
        generate_batch(set_item.cfile, 6561,
                       previous_set_item.cfile, weight1,
                       next_set_item.cfile, weight2);
        generate_batch(set_item.dfile, 6561,
                       previous_set_item.dfile, weight1,
                       next_set_item.dfile, weight2);
        generate_batch(set_item.diag8, 6561,
                       previous_set_item.diag8, weight1,
                       next_set_item.diag8, weight2);
        generate_batch(set_item.diag7, 2187,
                       previous_set_item.diag7, weight1,
                       next_set_item.diag7, weight2);
        generate_batch(set_item.diag6, 729,
                       previous_set_item.diag6, weight1,
                       next_set_item.diag6, weight2);
        generate_batch(set_item.diag5, 243,
                       previous_set_item.diag5, weight1,
                       next_set_item.diag5, weight2);
        generate_batch(set_item.diag4, 81,
                       previous_set_item.diag4, weight1,
                       next_set_item.diag4, weight2);
        generate_batch(set_item.corner33, 19683,
                       previous_set_item.corner33, weight1,
                       next_set_item.corner33, weight2);
        generate_batch(set_item.corner52, 59049,
                       previous_set_item.corner52, weight1,
                       next_set_item.corner52, weight2);
    }
    set_item.afile2x_last = set_item.afile2x.offset(59048);
    set_item.bfile_last = set_item.bfile.offset(6560);
    set_item.cfile_last = set_item.cfile.offset(6560);
    set_item.dfile_last = set_item.dfile.offset(6560);
    set_item.diag8_last = set_item.diag8.offset(6560);
    set_item.diag7_last = set_item.diag7.offset(2186);
    set_item.diag6_last = set_item.diag6.offset(728);
    set_item.diag5_last = set_item.diag5.offset(242);
    set_item.diag4_last = set_item.diag4.offset(80);
    set_item.corner33_last = set_item.corner33.offset(19682);
    set_item.corner52_last = set_item.corner52.offset(59048);
    set_item.loaded = 1;
}

/*
   PATTERN_EVALUATION
   Calculates the static evaluation of the position using
   the statistically optimized pattern tables.
*/

pub unsafe fn pattern_evaluation<FE: FrontEnd>(side_to_move: i32)
                                               -> i32 {
    let mut eval_phase: i32 = 0;
    let mut score: i16 = 0;
    /* Any player wiped out? Game over then... */
    if piece_count[0][disks_played as usize] ==
        0 as i32 {
        if side_to_move == 0 as i32 {
            return -(29000 as i32 + 64 as i32)
        } else { return 29000 as i32 + 64 as i32 }
    } else {
        if piece_count[2][disks_played as usize] ==
            0 as i32 {
            if side_to_move == 0 as i32 {
                return 29000 as i32 + 64 as i32
            } else { return -(29000 as i32 + 64 as i32) }
        }
    }
    /* Load and/or initialize the pattern coefficients */
    eval_phase = eval_map[disks_played as usize];
    if set[eval_phase as usize].loaded == 0 {
        load_set::<FE>(eval_phase);
    }
    constant_and_parity_feature(side_to_move, eval_phase, disks_played, &mut globals::board, &mut set)
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
    while i < stage[0] {
        eval_map[i as usize] = stage[0];
        i += 1
    }
    i = 0;
    while i < stage_count {
        eval_map[stage[i as usize] as usize] = stage[i as usize];
        i += 1
    }
    let mut subsequent_stage = 60;
    i = subsequent_stage;
    while i >= stage[0] {
        if eval_map[i as usize] == i {
            subsequent_stage = i
        } else if i == subsequent_stage - 2 as i32 {
            eval_map[i as usize] = i;
            subsequent_stage = i
        } else { eval_map[i as usize] = subsequent_stage }
        i -= 1
    };
}


/*
   UNPACK_BATCH
   Reads feature values for one specific pattern
*/
pub unsafe fn unpack_batch<FE: FrontEnd, S:FnMut() -> i16>(item: *mut i16,
                                                           mirror: *mut i32,
                                                           count: i32,
                                                           next_word: &mut S) {
    let mut buf = vec![0; count as usize];
    let mut buffer: *mut i16 = buf.as_mut_ptr();
    /* Unpack the coefficient block where the score is scaled
       so that 512 units corresponds to one disk. */
    let mut i: i32 = 0;
    while i < count {
        if mirror.is_null() || *mirror.offset(i as isize) == i {
            let i1 = next_word();
            *buffer.offset(i as isize) =
                (i1 as i32 / 4 as i32) as
                    i16
        } else {
            *buffer.offset(i as isize) =
                *buffer.offset(*mirror.offset(i as isize) as isize)
        }
        i += 1
    }
    i = 0;
    while i < count {
        *item.offset(i as isize) = *buffer.offset(i as isize);
        i += 1
    }
    if !mirror.is_null() {
        i = 0;
        while i < count {
            if *item.offset(i as isize) as i32 !=
                *item.offset(*mirror.offset(i as isize) as isize) as
                    i32 {
                let first_mirror_offset = *mirror.offset(i as isize);
                let first_item = *item.offset(i as isize) as i32;
                let second_item = *item.offset(first_mirror_offset as isize) as i32;

                FE::report_mirror_symetry_error(count, i, first_mirror_offset, first_item, second_item);
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
    let mut map_mirror3 = 0 as *mut i32;
    let mut map_mirror4 = 0 as *mut i32;
    let mut map_mirror5 = 0 as *mut i32;
    let mut map_mirror6 = 0 as *mut i32;
    let mut map_mirror7 = 0 as *mut i32;
    let mut map_mirror8 = 0 as *mut i32;
    let mut map_mirror33 = 0 as *mut i32;
    let mut map_mirror8x2 = 0 as *mut i32;
    /* Allocate the memory needed for the temporary mirror maps from the
       heap rather than the stack to reduce memory requirements. */
    map_mirror3 =
        safe_malloc::<FE>((27 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror4 =
        safe_malloc::<FE>((81 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror5 =
        safe_malloc::<FE>((243 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror6 =
        safe_malloc::<FE>((729 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror7 =
        safe_malloc::<FE>((2187 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror8 =
        safe_malloc::<FE>((6561 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror33 =
        safe_malloc::<FE>((19683 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror8x2 =
        safe_malloc::<FE>((59049 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
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
    while i < stage_count - 1 as i32 {
        set[stage[i as usize] as usize].constant =
            (next_word() as i32 / 4 as i32) as
                i16;
        set[stage[i as usize] as usize].parity =
            (next_word() as i32 / 4 as i32) as
                i16;
        set[stage[i as usize] as
            usize].parity_constant[0] =
            set[stage[i as usize] as usize].constant;
        set[stage[i as usize] as
            usize].parity_constant[1] =
            (set[stage[i as usize] as usize].constant as i32 +
                set[stage[i as usize] as usize].parity as i32) as
                i16;
        unpack_batch::<FE, S>(set[stage[i as usize] as usize].afile2x, map_mirror8x2,
                     59049 as i32, next_word);
        unpack_batch::<FE, S>(set[stage[i as usize] as usize].bfile, map_mirror8,
                     6561 as i32, next_word);
        unpack_batch::<FE, S>(set[stage[i as usize] as usize].cfile, map_mirror8,
                     6561 as i32, next_word);
        unpack_batch::<FE, S>(set[stage[i as usize] as usize].dfile, map_mirror8,
                     6561 as i32, next_word);
        unpack_batch::<FE, S>(set[stage[i as usize] as usize].diag8, map_mirror8,
                     6561 as i32, next_word);
        unpack_batch::<FE, S>(set[stage[i as usize] as usize].diag7, map_mirror7,
                     2187 as i32, next_word);
        unpack_batch::<FE, S>(set[stage[i as usize] as usize].diag6, map_mirror6,
                     729 as i32, next_word);
        unpack_batch::<FE, S>(set[stage[i as usize] as usize].diag5, map_mirror5,
                     243 as i32, next_word);
        unpack_batch::<FE, S>(set[stage[i as usize] as usize].diag4, map_mirror4,
                     81 as i32, next_word);
        unpack_batch::<FE, S>(set[stage[i as usize] as usize].corner33, map_mirror33,
                     19683 as i32, next_word);
        unpack_batch::<FE, S>(set[stage[i as usize] as usize].corner52,
                     0 as *mut i32, 59049 as i32, next_word);
        i += 1
    }
    /* Free the mirror tables - the symmetries are now implicit
       in the coefficient tables. */
    FE::free(map_mirror3 as *mut c_void);
    FE::free(map_mirror4 as *mut c_void);
    FE::free(map_mirror5 as *mut c_void);
    FE::free(map_mirror6 as *mut c_void);
    FE::free(map_mirror7 as *mut c_void);
    FE::free(map_mirror8 as *mut c_void);
    FE::free(map_mirror33 as *mut c_void);
    FE::free(map_mirror8x2 as *mut c_void);
}

pub unsafe fn process_coeffs_from_fn_source<FE: FrontEnd, Source:CoeffSource>(mut coeffs: Source) {
    let mut next_word = || coeffs.next_word();
    /* Read the different stages for which the evaluation function
       was tuned and mark the other stages with pointers to the previous
       and next stages. */
    let mut i = 0;
    while i <= 60 as i32 {
        set[i as usize].permanent = 0;
        set[i as usize].loaded = 0;
        i += 1
    }
    stage_count = next_word() as i32;
    let mut i = 0;
    let mut j: i32 = 0;
    let mut curr_stage: i32 = 0;
    while i < stage_count - 1 as i32 {
        stage[i as usize] = next_word() as i32;
        curr_stage = stage[i as usize];
        if i == 0 as i32 {
            j = 0;
            while j < stage[0] {
                set[j as usize].prev = stage[0];
                set[j as usize].next = stage[0];
                j += 1
            }
        } else {
            j = stage[(i - 1 as i32) as usize];
            while j < stage[i as usize] {
                set[j as usize].prev = stage[(i - 1 as i32) as usize];
                set[j as usize].next = stage[i as usize];
                j += 1
            }
        }
        set[curr_stage as usize].permanent = 1;
        allocate_set::<FE>(curr_stage);
        i += 1
    }
    stage[(stage_count - 1 as i32) as usize] = 60;
    j = stage[(stage_count - 2 as i32) as usize];
    while j < 60 as i32 {
        set[j as usize].prev =
            stage[(stage_count - 2 as i32) as usize];
        set[j as usize].next = 60;
        j += 1
    }
    set[60].permanent = 1;
    allocate_set::<FE>(60 as i32);
    /* Read the pattern values */
    unpack_coeffs::<FE, _>(&mut next_word);
}


pub unsafe fn init_coeffs_calculate_patterns() {
    /* Calculate the patterns which correspond to the board being filled */
    terminal_patterns(&mut set);
    set[60].constant = 0;
    set[60].parity = 0;
    set[60].parity_constant[0]
        = set[60].constant;
    set[60].parity_constant[1]
        =
        (set[60].constant as i32 +
            set[60].parity as i32) as
            i16;
}
