use crate::src::globals::{board, piece_count};
use crate::src::moves::disks_played;
use crate::src::patterns::{flip8, pow3};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoeffSet {
    pub permanent: i32,
    pub loaded: i32,
    pub prev: i32,
    pub next: i32,
    pub block: i32,
    pub parity_constant: [i16; 2],
    pub parity: i16,
    pub constant: i16,
    pub afile2x: *mut i16,
    pub bfile: *mut i16,
    pub cfile: *mut i16,
    pub dfile: *mut i16,
    pub diag8: *mut i16,
    pub diag7: *mut i16,
    pub diag6: *mut i16,
    pub diag5: *mut i16,
    pub diag4: *mut i16,
    pub corner33: *mut i16,
    pub corner52: *mut i16,
    pub afile2x_last: *mut i16,
    pub bfile_last: *mut i16,
    pub cfile_last: *mut i16,
    pub dfile_last: *mut i16,
    pub diag8_last: *mut i16,
    pub diag7_last: *mut i16,
    pub diag6_last: *mut i16,
    pub diag5_last: *mut i16,
    pub diag4_last: *mut i16,
    pub corner33_last: *mut i16,
    pub corner52_last: *mut i16,
    pub alignment_padding: [i8; 12],
}
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub signed_val: i16,
    pub unsigned_val: u16,
}
pub static mut stage_count: i32 = 0;
pub static mut block_count: i32 = 0;
pub static mut stage: [i32; 61] = [0; 61];
pub static mut block_allocated: [i32; 200] = [0; 200];
pub static mut block_set: [i32; 200] = [0; 200];
pub static mut eval_map: [i32; 61] = [0; 61];
pub static mut block_list: [*mut AllocationBlock; 200] =
    [0 as *const AllocationBlock as *mut AllocationBlock; 200];
pub static mut set: [CoeffSet; 61] =
    [CoeffSet{permanent: 0,
        loaded: 0,
        prev: 0,
        next: 0,
        block: 0,
        parity_constant: [0; 2],
        parity: 0,
        constant: 0,
        afile2x: 0 as *const i16 as *mut i16,
        bfile: 0 as *const i16 as *mut i16,
        cfile: 0 as *const i16 as *mut i16,
        dfile: 0 as *const i16 as *mut i16,
        diag8: 0 as *const i16 as *mut i16,
        diag7: 0 as *const i16 as *mut i16,
        diag6: 0 as *const i16 as *mut i16,
        diag5: 0 as *const i16 as *mut i16,
        diag4: 0 as *const i16 as *mut i16,
        corner33: 0 as *const i16 as *mut i16,
        corner52: 0 as *const i16 as *mut i16,
        afile2x_last: 0 as *const i16 as *mut i16,
        bfile_last: 0 as *const i16 as *mut i16,
        cfile_last: 0 as *const i16 as *mut i16,
        dfile_last: 0 as *const i16 as *mut i16,
        diag8_last: 0 as *const i16 as *mut i16,
        diag7_last: 0 as *const i16 as *mut i16,
        diag6_last: 0 as *const i16 as *mut i16,
        diag5_last: 0 as *const i16 as *mut i16,
        diag4_last: 0 as *const i16 as *mut i16,
        corner33_last: 0 as *const i16 as *mut i16,
        corner52_last: 0 as *const i16 as *mut i16,
        alignment_padding: [0; 12],}; 61];
/*
   GENERATE_BATCH
   Interpolates between two stages.
*/
pub unsafe fn generate_batch(mut target: *mut i16,
                         mut count: i32,
                         mut source1: *mut i16,
                         mut weight1: i32,
                         mut source2: *mut i16,
                         mut weight2: i32) {
    let mut i: i32 = 0;
    let mut total_weight: i32 = 0;
    total_weight = weight1 + weight2;
    i = 0 as i32;
    while i < count {
        *target.offset(i as isize) =
            ((weight1 * *source1.offset(i as isize) as i32 +
                weight2 * *source2.offset(i as isize) as i32) /
                total_weight) as i16;
        i += 1
    };
}

/*
   FREE_MEMORY_BLOCK
   Marks a memory block as no longer in use.
*/
pub unsafe fn free_memory_block(mut block: i32) {
    block_allocated[block as usize] = 0 as i32;
}
/*
   INIT_MEMORY_HANDLER
   Mark all blocks in the memory arena as "not used".
*/
pub unsafe fn init_memory_handler() {
    let mut i: i32 = 0;
    block_count = 0 as i32;
    i = 0 as i32;
    while i < 200 as i32 {
        block_allocated[i as usize] = 0 as i32;
        i += 1
    };
}


/*
  DISC_COUNT_ADJUSTMENT
*/
pub unsafe fn eval_adjustment(mut disc_adjust: f64,
                          mut edge_adjust: f64,
                          mut corner_adjust: f64,
                          mut x_adjust: f64) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut adjust: i32 = 0;
    let mut row: [i32; 10] = [0; 10];
    i = 0 as i32;
    while i < stage_count - 1 as i32 {
        /* Bonuses for having more discs */
        j = 0 as i32;
        while j < 59049 as i32 {
            let ref mut fresh2 =
                *set[stage[i as usize] as usize].afile2x.offset(j as isize);
            *fresh2 =
                (*fresh2 as f64 +
                    *set[60 as i32 as
                        usize].afile2x.offset(j as isize) as i32
                        as f64 * disc_adjust) as i16;
            let ref mut fresh3 =
                *set[stage[i as usize] as usize].corner52.offset(j as isize);
            *fresh3 =
                (*fresh3 as f64 +
                    *set[60 as i32 as
                        usize].corner52.offset(j as isize) as
                        i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0 as i32;
        while j < 19683 as i32 {
            let ref mut fresh4 =
                *set[stage[i as usize] as usize].corner33.offset(j as isize);
            *fresh4 =
                (*fresh4 as f64 +
                    *set[60 as i32 as
                        usize].corner33.offset(j as isize) as
                        i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0 as i32;
        while j < 6561 as i32 {
            let ref mut fresh5 =
                *set[stage[i as usize] as usize].bfile.offset(j as isize);
            *fresh5 =
                (*fresh5 as f64 +
                    *set[60 as i32 as usize].bfile.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            let ref mut fresh6 =
                *set[stage[i as usize] as usize].cfile.offset(j as isize);
            *fresh6 =
                (*fresh6 as f64 +
                    *set[60 as i32 as usize].cfile.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            let ref mut fresh7 =
                *set[stage[i as usize] as usize].dfile.offset(j as isize);
            *fresh7 =
                (*fresh7 as f64 +
                    *set[60 as i32 as usize].dfile.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            let ref mut fresh8 =
                *set[stage[i as usize] as usize].diag8.offset(j as isize);
            *fresh8 =
                (*fresh8 as f64 +
                    *set[60 as i32 as usize].diag8.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0 as i32;
        while j < 2187 as i32 {
            let ref mut fresh9 =
                *set[stage[i as usize] as usize].diag7.offset(j as isize);
            *fresh9 =
                (*fresh9 as f64 +
                    *set[60 as i32 as usize].diag7.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0 as i32;
        while j < 729 as i32 {
            let ref mut fresh10 =
                *set[stage[i as usize] as usize].diag6.offset(j as isize);
            *fresh10 =
                (*fresh10 as f64 +
                    *set[60 as i32 as usize].diag6.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0 as i32;
        while j < 243 as i32 {
            let ref mut fresh11 =
                *set[stage[i as usize] as usize].diag5.offset(j as isize);
            *fresh11 =
                (*fresh11 as f64 +
                    *set[60 as i32 as usize].diag5.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0 as i32;
        while j < 81 as i32 {
            let ref mut fresh12 =
                *set[stage[i as usize] as usize].diag4.offset(j as isize);
            *fresh12 =
                (*fresh12 as f64 +
                    *set[60 as i32 as usize].diag4.offset(j as isize)
                        as i32 as f64 * disc_adjust) as
                    i16;
            j += 1
        }
        j = 0 as i32;
        while j < 10 as i32 {
            row[j as usize] = 0 as i32;
            j += 1
        }
        j = 0 as i32;
        while j < 59049 as i32 {
            adjust = 0 as i32;
            /* Bonus for having edge discs */
            k = 1 as i32;
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
            if row[0 as i32 as usize] == 0 as i32 {
                adjust =
                    (adjust as f64 +
                        0.5f64 * 128.0f64 * corner_adjust) as i32
            } else if row[0 as i32 as usize] == 2 as i32 {
                adjust =
                    (adjust as f64 -
                        0.5f64 * 128.0f64 * corner_adjust) as i32
            }
            if row[7 as i32 as usize] == 0 as i32 {
                adjust =
                    (adjust as f64 +
                        0.5f64 * 128.0f64 * corner_adjust) as i32
            } else if row[7 as i32 as usize] == 2 as i32 {
                adjust =
                    (adjust as f64 -
                        0.5f64 * 128.0f64 * corner_adjust) as i32
            }
            /* Bonus for having X-squares when the adjacent corners are empty.
            Scaling by 0.5 applies here too. */
            if row[8 as i32 as usize] == 0 as i32 &&
                row[0 as i32 as usize] == 1 as i32 {
                adjust =
                    (adjust as f64 + 0.5f64 * 128.0f64 * x_adjust)
                        as i32
            } else if row[8 as i32 as usize] == 2 as i32 &&
                row[0 as i32 as usize] == 1 as i32 {
                adjust =
                    (adjust as f64 - 0.5f64 * 128.0f64 * x_adjust)
                        as i32
            }
            if row[9 as i32 as usize] == 0 as i32 &&
                row[7 as i32 as usize] == 1 as i32 {
                adjust =
                    (adjust as f64 + 0.5f64 * 128.0f64 * x_adjust)
                        as i32
            } else if row[9 as i32 as usize] == 2 as i32 &&
                row[7 as i32 as usize] == 1 as i32 {
                adjust =
                    (adjust as f64 - 0.5f64 * 128.0f64 * x_adjust)
                        as i32
            }
            let ref mut fresh13 =
                *set[stage[i as usize] as usize].afile2x.offset(j as isize);
            *fresh13 = (*fresh13 as i32 + adjust) as i16;
            /* Next configuration */
            k = 0 as i32;
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
pub unsafe fn remove_specific_coeffs(mut phase: i32) {
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

pub unsafe fn remove_coeffs(mut phase: i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < phase { remove_specific_coeffs(i); i += 1 };
}
/*
   CLEAR_COEFFS
   Remove all coefficients loaded from memory.
*/

pub unsafe fn clear_coeffs() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i <= 60 as i32 { remove_specific_coeffs(i); i += 1 };
}
