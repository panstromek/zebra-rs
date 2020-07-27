use crate::src::globals::{board, piece_count};
use crate::src::moves::disks_played;
use crate::src::patterns::{flip8, pow3};
use crate::src::stubs::{floor, free};
use crate::src::safemem::safe_malloc;
use crate::src::error::fatal_error;
use std::ffi::c_void;
use std::process::exit;

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



/*
   TERMINAL_PATTERNS
   Calculates the patterns associated with a filled board,
   only counting discs.
*/
pub unsafe fn terminal_patterns() {
    let mut result: f64 = 0.;
    let mut value: [[f64; 8]; 8] = [[0.; 8]; 8];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut row: [i32; 10] = [0; 10];
    let mut hit: [[i32; 8]; 8] = [[0; 8]; 8];
    /* Count the number of times each square is counted */
    i = 0 as i32;
    while i < 8 as i32 {
        j = 0 as i32;
        while j < 8 as i32 {
            hit[i as usize][j as usize] = 0 as i32;
            j += 1
        }
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        hit[0 as i32 as usize][i as usize] += 1;
        hit[i as usize][0 as i32 as usize] += 1;
        hit[7 as i32 as usize][i as usize] += 1;
        hit[i as usize][7 as i32 as usize] += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        hit[1 as i32 as usize][i as usize] += 1;
        hit[i as usize][1 as i32 as usize] += 1;
        hit[6 as i32 as usize][i as usize] += 1;
        hit[i as usize][6 as i32 as usize] += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        hit[2 as i32 as usize][i as usize] += 1;
        hit[i as usize][2 as i32 as usize] += 1;
        hit[5 as i32 as usize][i as usize] += 1;
        hit[i as usize][5 as i32 as usize] += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        hit[3 as i32 as usize][i as usize] += 1;
        hit[i as usize][3 as i32 as usize] += 1;
        hit[4 as i32 as usize][i as usize] += 1;
        hit[i as usize][4 as i32 as usize] += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 3 as i32 {
        j = 0 as i32;
        while j < 3 as i32 {
            hit[i as usize][j as usize] += 1;
            hit[i as usize][(7 as i32 - j) as usize] += 1;
            hit[(7 as i32 - i) as usize][j as usize] += 1;
            hit[(7 as i32 - i) as
                usize][(7 as i32 - j) as usize] += 1;
            j += 1
        }
        i += 1
    }
    i = 0 as i32;
    while i < 2 as i32 {
        j = 0 as i32;
        while j < 5 as i32 {
            hit[i as usize][j as usize] += 1;
            hit[j as usize][i as usize] += 1;
            hit[i as usize][(7 as i32 - j) as usize] += 1;
            hit[j as usize][(7 as i32 - i) as usize] += 1;
            hit[(7 as i32 - i) as usize][j as usize] += 1;
            hit[(7 as i32 - j) as usize][i as usize] += 1;
            hit[(7 as i32 - i) as
                usize][(7 as i32 - j) as usize] += 1;
            hit[(7 as i32 - j) as
                usize][(7 as i32 - i) as usize] += 1;
            j += 1
        }
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        hit[i as usize][i as usize] += 1;
        hit[i as usize][(7 as i32 - i) as usize] += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 7 as i32 {
        hit[i as usize][(i + 1 as i32) as usize] += 1;
        hit[(i + 1 as i32) as usize][i as usize] += 1;
        hit[i as usize][(6 as i32 - i) as usize] += 1;
        hit[(i + 1 as i32) as usize][(7 as i32 - i) as usize]
            += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 6 as i32 {
        hit[i as usize][(i + 2 as i32) as usize] += 1;
        hit[(i + 2 as i32) as usize][i as usize] += 1;
        hit[i as usize][(5 as i32 - i) as usize] += 1;
        hit[(i + 2 as i32) as usize][(7 as i32 - i) as usize]
            += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 5 as i32 {
        hit[i as usize][(i + 3 as i32) as usize] += 1;
        hit[(i + 3 as i32) as usize][i as usize] += 1;
        hit[i as usize][(4 as i32 - i) as usize] += 1;
        hit[(i + 3 as i32) as usize][(7 as i32 - i) as usize]
            += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 4 as i32 {
        hit[i as usize][(i + 4 as i32) as usize] += 1;
        hit[(i + 4 as i32) as usize][i as usize] += 1;
        hit[i as usize][(3 as i32 - i) as usize] += 1;
        hit[(i + 4 as i32) as usize][(7 as i32 - i) as usize]
            += 1;
        i += 1
    }
    hit[1 as i32 as usize][1 as i32 as usize] +=
        2 as i32;
    hit[1 as i32 as usize][6 as i32 as usize] +=
        2 as i32;
    hit[6 as i32 as usize][1 as i32 as usize] +=
        2 as i32;
    hit[6 as i32 as usize][6 as i32 as usize] +=
        2 as i32;
    i = 0 as i32;
    while i < 8 as i32 {
        j = 0 as i32;
        while j < 8 as i32 {
            value[i as usize][j as usize] =
                1.0f64 / hit[i as usize][j as usize] as f64;
            j += 1
        }
        i += 1
    }
    i = 0 as i32;
    while i < 10 as i32 { row[i as usize] = 0 as i32; i += 1 }
    i = 0 as i32;
    while i < 59049 as i32 {
        result = 0.0f64;
        j = 0 as i32;
        while j < 8 as i32 {
            if row[j as usize] == 0 as i32 {
                result += value[0 as i32 as usize][j as usize]
            } else if row[j as usize] == 2 as i32 {
                result -= value[0 as i32 as usize][j as usize]
            }
            j += 1
        }
        if row[8 as i32 as usize] == 0 as i32 {
            result +=
                value[1 as i32 as usize][1 as i32 as usize]
        } else if row[8 as i32 as usize] == 2 as i32 {
            result -=
                value[1 as i32 as usize][1 as i32 as usize]
        }
        if row[9 as i32 as usize] == 0 as i32 {
            result +=
                value[1 as i32 as usize][6 as i32 as usize]
        } else if row[9 as i32 as usize] == 2 as i32 {
            result -=
                value[1 as i32 as usize][6 as i32 as usize]
        }
        *set[60 as i32 as usize].afile2x.offset(i as isize) =
            floor(result * 128.0f64 + 0.5f64) as i16;
        result = 0.0f64;
        j = 0 as i32;
        while j < 5 as i32 {
            k = 0 as i32;
            while k < 2 as i32 {
                if row[(5 as i32 * k + j) as usize] ==
                    0 as i32 {
                    result += value[j as usize][k as usize]
                } else if row[(5 as i32 * k + j) as usize] ==
                    2 as i32 {
                    result -= value[j as usize][k as usize]
                }
                k += 1
            }
            j += 1
        }
        *set[60 as i32 as usize].corner52.offset(i as isize) =
            floor(result * 128.0f64 + 0.5f64) as i16;
        if i < 19683 as i32 {
            result = 0.0f64;
            j = 0 as i32;
            while j < 3 as i32 {
                k = 0 as i32;
                while k < 3 as i32 {
                    if row[(3 as i32 * j + k) as usize] ==
                        0 as i32 {
                        result += value[j as usize][k as usize]
                    } else if row[(3 as i32 * j + k) as usize] ==
                        2 as i32 {
                        result -= value[j as usize][k as usize]
                    }
                    k += 1
                }
                j += 1
            }
            *set[60 as i32 as usize].corner33.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 6561 as i32 {
            result = 0.0f64;
            j = 0 as i32;
            while j < 8 as i32 {
                if row[j as usize] == 0 as i32 {
                    result += value[1 as i32 as usize][j as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -= value[1 as i32 as usize][j as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].bfile.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16;
            result = 0.0f64;
            j = 0 as i32;
            while j < 8 as i32 {
                if row[j as usize] == 0 as i32 {
                    result += value[2 as i32 as usize][j as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -= value[2 as i32 as usize][j as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].cfile.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16;
            result = 0.0f64;
            j = 0 as i32;
            while j < 8 as i32 {
                if row[j as usize] == 0 as i32 {
                    result += value[3 as i32 as usize][j as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -= value[3 as i32 as usize][j as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].dfile.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16;
            result = 0.0f64;
            j = 0 as i32;
            while j < 8 as i32 {
                if row[j as usize] == 0 as i32 {
                    result += value[j as usize][j as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -= value[j as usize][j as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].diag8.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 2187 as i32 {
            result = 0.0f64;
            j = 0 as i32;
            while j < 7 as i32 {
                if row[j as usize] == 0 as i32 {
                    result +=
                        value[j as usize][(j + 1 as i32) as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -=
                        value[j as usize][(j + 1 as i32) as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].diag7.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 729 as i32 {
            result = 0.0f64;
            j = 0 as i32;
            while j < 6 as i32 {
                if row[j as usize] == 0 as i32 {
                    result +=
                        value[j as usize][(j + 2 as i32) as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -=
                        value[j as usize][(j + 2 as i32) as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].diag6.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 243 as i32 {
            result = 0.0f64;
            j = 0 as i32;
            while j < 5 as i32 {
                if row[j as usize] == 0 as i32 {
                    result +=
                        value[j as usize][(j + 3 as i32) as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -=
                        value[j as usize][(j + 3 as i32) as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].diag5.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 81 as i32 {
            result = 0.0f64;
            j = 0 as i32;
            while j < 4 as i32 {
                if row[j as usize] == 0 as i32 {
                    result +=
                        value[j as usize][(j + 4 as i32) as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -=
                        value[j as usize][(j + 4 as i32) as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].diag4.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16
        }
        /* Next configuration */
        j = 0 as i32;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 10 as i32) {
                break ;
            }
        }
        i += 1
    };
}
/*
   FIND_MEMORY_BLOCK
   Maintains an internal memory handler to boost
   performance and avoid heap fragmentation.
*/
pub unsafe fn find_memory_block(mut afile2x: *mut *mut i16,
                            mut bfile: *mut *mut i16,
                            mut cfile: *mut *mut i16,
                            mut dfile: *mut *mut i16,
                            mut diag8: *mut *mut i16,
                            mut diag7: *mut *mut i16,
                            mut diag6: *mut *mut i16,
                            mut diag5: *mut *mut i16,
                            mut diag4: *mut *mut i16,
                            mut corner33: *mut *mut i16,
                            mut corner52: *mut *mut i16,
                            mut index: i32)
                            -> i32 {
    let mut i: i32 = 0;
    let mut found_free: i32 = 0;
    let mut free_block: i32 = 0;
    found_free = 0 as i32;
    free_block = -(1 as i32);
    i = 0 as i32;
    while i < block_count && found_free == 0 {
        if block_allocated[i as usize] == 0 {
            found_free = 1 as i32;
            free_block = i
        }
        i += 1
    }
    if found_free == 0 {
        if block_count < 200 as i32 {
            block_list[block_count as usize] =
                safe_malloc(::std::mem::size_of::<AllocationBlock>() as
                    u64) as *mut AllocationBlock
        }
        if block_count == 200 as i32 ||
            block_list[block_count as usize].is_null() {
            fatal_error(b"%s @ #%d\n\x00" as *const u8 as *const i8,
                        b"Memory allocation failure\x00" as *const u8 as
                            *const i8, block_count);
        }
        free_block = block_count;
        block_count += 1
    }
    *afile2x = (*block_list[free_block as usize]).afile2x_block.as_mut_ptr();
    *bfile = (*block_list[free_block as usize]).bfile_block.as_mut_ptr();
    *cfile = (*block_list[free_block as usize]).cfile_block.as_mut_ptr();
    *dfile = (*block_list[free_block as usize]).dfile_block.as_mut_ptr();
    *diag8 = (*block_list[free_block as usize]).diag8_block.as_mut_ptr();
    *diag7 = (*block_list[free_block as usize]).diag7_block.as_mut_ptr();
    *diag6 = (*block_list[free_block as usize]).diag6_block.as_mut_ptr();
    *diag5 = (*block_list[free_block as usize]).diag5_block.as_mut_ptr();
    *diag4 = (*block_list[free_block as usize]).diag4_block.as_mut_ptr();
    *corner33 =
        (*block_list[free_block as usize]).corner33_block.as_mut_ptr();
    *corner52 =
        (*block_list[free_block as usize]).corner52_block.as_mut_ptr();
    block_allocated[free_block as usize] = 1 as i32;
    block_set[free_block as usize] = index;
    return free_block;
}
/*
   ALLOCATE_SET
   Finds memory for all patterns belonging to a certain stage.
*/
pub unsafe fn allocate_set(mut index: i32) {
    set[index as usize].block =
        find_memory_block(&mut (*set.as_mut_ptr().offset(index as
            isize)).afile2x,
                          &mut (*set.as_mut_ptr().offset(index as
                              isize)).bfile,
                          &mut (*set.as_mut_ptr().offset(index as
                              isize)).cfile,
                          &mut (*set.as_mut_ptr().offset(index as
                              isize)).dfile,
                          &mut (*set.as_mut_ptr().offset(index as
                              isize)).diag8,
                          &mut (*set.as_mut_ptr().offset(index as
                              isize)).diag7,
                          &mut (*set.as_mut_ptr().offset(index as
                              isize)).diag6,
                          &mut (*set.as_mut_ptr().offset(index as
                              isize)).diag5,
                          &mut (*set.as_mut_ptr().offset(index as
                              isize)).diag4,
                          &mut (*set.as_mut_ptr().offset(index as
                              isize)).corner33,
                          &mut (*set.as_mut_ptr().offset(index as
                              isize)).corner52,
                          index);
}
/*
   LOAD_SET
   Performs linear interpolation between the nearest stages to
   obtain the feature values for the stage in question.
   Also calculates the offset pointers to the last elements in each block
   (used for the inverted patterns when white is to move).
*/
pub unsafe fn load_set(mut index: i32) {
    let mut prev: i32 = 0;
    let mut next: i32 = 0;
    let mut weight1: i32 = 0;
    let mut weight2: i32 = 0;
    let mut total_weight: i32 = 0;
    if set[index as usize].permanent == 0 {
        prev = set[index as usize].prev;
        next = set[index as usize].next;
        if prev == next {
            weight1 = 1 as i32;
            weight2 = 1 as i32
        } else { weight1 = next - index; weight2 = index - prev }
        total_weight = weight1 + weight2;
        set[index as usize].constant =
            ((weight1 * set[prev as usize].constant as i32 +
                weight2 * set[next as usize].constant as i32) /
                total_weight) as i16;
        set[index as usize].parity =
            ((weight1 * set[prev as usize].parity as i32 +
                weight2 * set[next as usize].parity as i32) /
                total_weight) as i16;
        set[index as usize].parity_constant[0 as i32 as usize] =
            set[index as usize].constant;
        set[index as usize].parity_constant[1 as i32 as usize] =
            (set[index as usize].constant as i32 +
                set[index as usize].parity as i32) as i16;
        allocate_set(index);
        generate_batch(set[index as usize].afile2x, 59049 as i32,
                       set[prev as usize].afile2x, weight1,
                       set[next as usize].afile2x, weight2);
        generate_batch(set[index as usize].bfile, 6561 as i32,
                       set[prev as usize].bfile, weight1,
                       set[next as usize].bfile, weight2);
        generate_batch(set[index as usize].cfile, 6561 as i32,
                       set[prev as usize].cfile, weight1,
                       set[next as usize].cfile, weight2);
        generate_batch(set[index as usize].dfile, 6561 as i32,
                       set[prev as usize].dfile, weight1,
                       set[next as usize].dfile, weight2);
        generate_batch(set[index as usize].diag8, 6561 as i32,
                       set[prev as usize].diag8, weight1,
                       set[next as usize].diag8, weight2);
        generate_batch(set[index as usize].diag7, 2187 as i32,
                       set[prev as usize].diag7, weight1,
                       set[next as usize].diag7, weight2);
        generate_batch(set[index as usize].diag6, 729 as i32,
                       set[prev as usize].diag6, weight1,
                       set[next as usize].diag6, weight2);
        generate_batch(set[index as usize].diag5, 243 as i32,
                       set[prev as usize].diag5, weight1,
                       set[next as usize].diag5, weight2);
        generate_batch(set[index as usize].diag4, 81 as i32,
                       set[prev as usize].diag4, weight1,
                       set[next as usize].diag4, weight2);
        generate_batch(set[index as usize].corner33, 19683 as i32,
                       set[prev as usize].corner33, weight1,
                       set[next as usize].corner33, weight2);
        generate_batch(set[index as usize].corner52, 59049 as i32,
                       set[prev as usize].corner52, weight1,
                       set[next as usize].corner52, weight2);
    }
    set[index as usize].afile2x_last =
        set[index as usize].afile2x.offset(59048 as i32 as isize);
    set[index as usize].bfile_last =
        set[index as usize].bfile.offset(6560 as i32 as isize);
    set[index as usize].cfile_last =
        set[index as usize].cfile.offset(6560 as i32 as isize);
    set[index as usize].dfile_last =
        set[index as usize].dfile.offset(6560 as i32 as isize);
    set[index as usize].diag8_last =
        set[index as usize].diag8.offset(6560 as i32 as isize);
    set[index as usize].diag7_last =
        set[index as usize].diag7.offset(2186 as i32 as isize);
    set[index as usize].diag6_last =
        set[index as usize].diag6.offset(728 as i32 as isize);
    set[index as usize].diag5_last =
        set[index as usize].diag5.offset(242 as i32 as isize);
    set[index as usize].diag4_last =
        set[index as usize].diag4.offset(80 as i32 as isize);
    set[index as usize].corner33_last =
        set[index as usize].corner33.offset(19682 as i32 as isize);
    set[index as usize].corner52_last =
        set[index as usize].corner52.offset(59048 as i32 as isize);
    set[index as usize].loaded = 1 as i32;
}


pub static mut pattern_score: i16 = 0;
/*
   PATTERN_EVALUATION
   Calculates the static evaluation of the position using
   the statistically optimized pattern tables.
*/

pub unsafe fn pattern_evaluation(mut side_to_move: i32)
                                 -> i32 {
    let mut eval_phase: i32 = 0;
    let mut score: i16 = 0;
    /* Any player wiped out? Game over then... */
    if piece_count[0 as i32 as usize][disks_played as usize] ==
        0 as i32 {
        if side_to_move == 0 as i32 {
            return -(29000 as i32 + 64 as i32)
        } else { return 29000 as i32 + 64 as i32 }
    } else {
        if piece_count[2 as i32 as usize][disks_played as usize] ==
            0 as i32 {
            if side_to_move == 0 as i32 {
                return 29000 as i32 + 64 as i32
            } else { return -(29000 as i32 + 64 as i32) }
        }
    }
    /* Load and/or initialize the pattern coefficients */
    eval_phase = eval_map[disks_played as usize];
    if set[eval_phase as usize].loaded == 0 { load_set(eval_phase); }
    /* The constant feature and the parity feature */
    score =
        set[eval_phase as
            usize].parity_constant[(disks_played & 1 as i32) as
            usize];
    /* The pattern features. */
    if side_to_move == 0 as i32 {
        let mut pattern0: i32 = 0;
        pattern0 = board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[81 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[71 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[61 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[51 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[41 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[31 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[21 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].afile2x.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[88 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[78 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[68 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[58 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[48 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[38 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[28 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].afile2x.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[18 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[17 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[16 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[15 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[14 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[13 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[12 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].afile2x.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[88 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[87 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[86 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[85 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[84 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[83 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[82 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].afile2x.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[82 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[62 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[52 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[42 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[32 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[12 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].bfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[87 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[67 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[57 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[47 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[37 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[17 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].bfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[28 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[26 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[25 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[24 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[23 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[21 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].bfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[78 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[76 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[75 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[74 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[73 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[71 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].bfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[83 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[73 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[63 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[53 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[43 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[33 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[23 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[13 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].cfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[86 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[76 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[66 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[56 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[46 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[36 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[26 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[16 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].cfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[38 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[37 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[36 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[35 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[34 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[33 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[32 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[31 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].cfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[68 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[67 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[66 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[65 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[64 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[63 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[62 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[61 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].cfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[84 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[74 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[64 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[54 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[44 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[34 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[24 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[14 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].dfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[85 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[75 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[65 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[55 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[45 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[35 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[25 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[15 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].dfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[48 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[47 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[46 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[45 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[44 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[43 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[42 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[41 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].dfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[58 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[57 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[56 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[55 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[54 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[53 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[52 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[51 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].dfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[88 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[66 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[55 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[44 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[33 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag8.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[81 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[63 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[54 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[45 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[36 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag8.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[78 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[67 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[56 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[45 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[34 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[23 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[12 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag7.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[87 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[76 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[65 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[54 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[43 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[32 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[21 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag7.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[71 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[62 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[53 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[44 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[35 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[26 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[17 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag7.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[82 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[73 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[64 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[55 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[46 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[37 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[28 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag7.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[68 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[57 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[46 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[35 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[24 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[13 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag6.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[86 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[75 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[64 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[53 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[42 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[31 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag6.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[61 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[52 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[43 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[34 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[25 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[16 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag6.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[83 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[74 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[65 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[56 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[47 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[38 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag6.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[58 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[47 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[36 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[25 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[14 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag5.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[85 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[74 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[63 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[52 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[41 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag5.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[51 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[42 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[33 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[24 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[15 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag5.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[84 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[75 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[66 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[57 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[48 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag5.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[48 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[37 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[26 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[15 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag4.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[84 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[73 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[62 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[51 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag4.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[41 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[32 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[23 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[14 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag4.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[85 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[76 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[67 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[58 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag4.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[33 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[32 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[31 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[23 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[21 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[13 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[12 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner33.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[63 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[62 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[61 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[73 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[71 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[83 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[82 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner33.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[36 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[37 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[38 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[26 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[28 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[16 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[17 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner33.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[66 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[67 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[68 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[76 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[78 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[86 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[87 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[88 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner33.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[25 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[24 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[23 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[21 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[15 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[14 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[13 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[12 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[75 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[74 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[73 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[71 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[85 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[84 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[83 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[82 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[24 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[25 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[26 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[28 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[14 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[15 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[16 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[17 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[74 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[75 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[76 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[78 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[84 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[85 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[86 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[87 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[88 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[52 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[42 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[32 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[12 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[51 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[41 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[31 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[21 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[57 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[47 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[37 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[17 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[58 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[48 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[38 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[28 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[42 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[52 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[62 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[82 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[41 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[51 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[61 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[71 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[47 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[57 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[67 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[87 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[48 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[58 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[68 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[78 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[88 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16
    } else {
        let mut pattern0_0: i32 = 0;
        pattern0_0 = board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[81 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[71 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[61 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[51 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[41 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[31 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[21 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].afile2x_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[88 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[78 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[68 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[58 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[48 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[38 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[28 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].afile2x_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[18 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[17 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[16 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[15 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[14 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[13 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[12 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].afile2x_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[88 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[87 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[86 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[85 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[84 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[83 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[82 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].afile2x_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[82 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[62 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[52 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[42 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[32 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[12 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].bfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[87 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[67 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[57 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[47 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[37 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[17 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].bfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[28 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[26 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[25 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[24 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[23 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[21 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].bfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[78 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[76 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[75 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[74 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[73 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[71 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].bfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[83 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[73 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[63 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[53 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[43 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[33 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[23 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[13 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].cfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[86 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[76 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[66 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[56 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[46 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[36 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[26 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[16 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].cfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[38 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[37 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[36 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[35 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[34 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[33 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[32 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[31 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].cfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[68 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[67 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[66 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[65 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[64 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[63 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[62 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[61 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].cfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[84 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[74 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[64 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[54 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[44 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[34 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[24 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[14 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].dfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[85 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[75 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[65 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[55 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[45 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[35 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[25 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[15 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].dfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[48 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[47 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[46 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[45 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[44 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[43 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[42 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[41 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].dfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[58 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[57 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[56 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[55 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[54 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[53 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[52 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[51 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].dfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[88 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[66 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[55 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[44 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[33 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag8_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[81 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[63 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[54 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[45 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[36 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag8_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[78 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[67 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[56 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[45 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[34 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[23 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[12 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag7_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[87 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[76 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[65 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[54 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[43 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[32 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[21 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag7_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[71 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[62 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[53 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[44 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[35 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[26 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[17 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag7_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[82 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[73 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[64 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[55 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[46 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[37 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[28 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag7_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[68 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[57 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[46 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[35 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[24 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[13 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag6_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[86 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[75 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[64 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[53 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[42 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[31 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag6_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[61 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[52 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[43 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[34 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[25 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[16 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag6_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[83 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[74 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[65 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[56 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[47 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[38 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag6_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[58 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[47 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[36 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[25 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[14 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag5_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[85 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[74 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[63 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[52 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[41 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag5_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[51 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[42 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[33 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[24 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[15 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag5_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[84 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[75 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[66 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[57 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[48 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag5_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[48 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[37 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[26 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[15 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag4_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[84 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[73 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[62 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[51 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag4_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[41 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[32 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[23 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[14 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag4_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[85 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[76 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[67 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[58 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag4_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[33 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[32 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[31 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[23 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[21 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[13 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[12 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner33_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[63 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[62 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[61 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[73 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[71 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[83 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[82 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner33_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[36 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[37 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[38 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[26 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[28 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[16 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[17 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner33_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[66 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[67 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[68 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[76 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[78 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[86 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[87 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[88 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner33_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[25 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[24 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[23 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[21 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[15 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[14 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[13 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[12 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[75 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[74 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[73 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[71 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[85 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[84 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[83 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[82 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[24 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[25 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[26 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[28 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[14 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[15 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[16 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[17 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[74 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[75 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[76 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[78 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[84 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[85 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[86 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[87 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[88 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[52 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[42 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[32 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[12 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[51 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[41 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[31 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[21 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[57 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[47 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[37 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[17 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[58 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[48 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[38 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[28 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[42 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[52 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[62 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[82 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[41 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[51 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[61 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[71 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[47 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[57 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[67 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[87 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[48 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[58 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[68 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[78 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[88 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16
    }
    return score as i32;
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
    let mut i = 0 as i32;
    while i < stage[0 as i32 as usize] {
        eval_map[i as usize] = stage[0 as i32 as usize];
        i += 1
    }
    i = 0 as i32;
    while i < stage_count {
        eval_map[stage[i as usize] as usize] = stage[i as usize];
        i += 1
    }
    let mut subsequent_stage = 60 as i32;
    i = subsequent_stage;
    while i >= stage[0 as i32 as usize] {
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
pub unsafe fn unpack_batch(mut item: *mut i16,
                           mut mirror: *mut i32,
                           mut count: i32,
                           next_word: &mut impl FnMut() -> i16) {
    let mut i: i32 = 0;
    let mut buffer = 0 as *mut i16;
    buffer =
        safe_malloc((count as
            u64).wrapping_mul(::std::mem::size_of::<i16>()
            as u64)) as
            *mut i16;
    /* Unpack the coefficient block where the score is scaled
       so that 512 units corresponds to one disk. */
    i = 0 as i32;
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
    i = 0 as i32;
    while i < count {
        *item.offset(i as isize) = *buffer.offset(i as isize);
        i += 1
    }
    if !mirror.is_null() {
        i = 0 as i32;
        while i < count {
            if *item.offset(i as isize) as i32 !=
                *item.offset(*mirror.offset(i as isize) as isize) as
                    i32 {
                let first_mirror_offset = *mirror.offset(i as isize);
                let first_item = *item.offset(i as isize) as i32;
                let second_item = *item.offset(first_mirror_offset as isize) as i32;

                report_mirror_symetry_error(count, i, first_mirror_offset, first_item, second_item);
                exit(1 as i32);
            }
            i += 1
        }
    }
    free(buffer as *mut c_void);
}
extern "C" {
    fn report_mirror_symetry_error(count: i32, i: i32, first_mirror_offset: i32, first_item: i32, second_item: i32);
}
/*
   UNPACK_COEFFS
   Reads all feature values for a certain stage. To take care of
   symmetric patterns, mirror tables are calculated.
*/
pub unsafe fn unpack_coeffs(next_word: &mut impl FnMut() -> i16) {
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
        safe_malloc((27 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror4 =
        safe_malloc((81 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror5 =
        safe_malloc((243 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror6 =
        safe_malloc((729 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror7 =
        safe_malloc((2187 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror8 =
        safe_malloc((6561 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror33 =
        safe_malloc((19683 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    map_mirror8x2 =
        safe_malloc((59049 as i32 as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    /* Build the pattern tables for 8*1-patterns */
    i = 0 as i32;
    while i < 8 as i32 { row[i as usize] = 0 as i32; i += 1 }
    i = 0 as i32;
    while i < 6561 as i32 {
        mirror_pattern = 0 as i32;
        j = 0 as i32;
        while j < 8 as i32 {
            mirror_pattern +=
                row[j as usize] * pow3[(7 as i32 - j) as usize];
            j += 1
        }
        /* Create the symmetry map */
        *map_mirror8.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0 as i32;
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
    i = 0 as i32;
    while i < 7 as i32 { row[i as usize] = 0 as i32; i += 1 }
    i = 0 as i32;
    while i < 2187 as i32 {
        mirror_pattern = 0 as i32;
        j = 0 as i32;
        while j < 7 as i32 {
            mirror_pattern +=
                row[j as usize] * pow3[(6 as i32 - j) as usize];
            j += 1
        }
        *map_mirror7.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0 as i32;
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
    i = 0 as i32;
    while i < 6 as i32 { row[i as usize] = 0 as i32; i += 1 }
    i = 0 as i32;
    while i < 729 as i32 {
        mirror_pattern = 0 as i32;
        j = 0 as i32;
        while j < 6 as i32 {
            mirror_pattern +=
                row[j as usize] * pow3[(5 as i32 - j) as usize];
            j += 1
        }
        *map_mirror6.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0 as i32;
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
    i = 0 as i32;
    while i < 5 as i32 { row[i as usize] = 0 as i32; i += 1 }
    i = 0 as i32;
    while i < 243 as i32 {
        mirror_pattern = 0 as i32;
        j = 0 as i32;
        while j < 5 as i32 {
            mirror_pattern +=
                row[j as usize] * pow3[(4 as i32 - j) as usize];
            j += 1
        }
        *map_mirror5.offset(i as isize) =
            if mirror_pattern < i { mirror_pattern } else { i };
        /* Next configuration */
        j = 0 as i32;
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
    i = 0 as i32;
    while i < 4 as i32 { row[i as usize] = 0 as i32; i += 1 }
    i = 0 as i32;
    while i < 81 as i32 {
        mirror_pattern = 0 as i32;
        j = 0 as i32;
        while j < 4 as i32 {
            mirror_pattern +=
                row[j as usize] * pow3[(3 as i32 - j) as usize];
            j += 1
        }
        *map_mirror4.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0 as i32;
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
    i = 0 as i32;
    while i < 3 as i32 { row[i as usize] = 0 as i32; i += 1 }
    i = 0 as i32;
    while i < 27 as i32 {
        mirror_pattern = 0 as i32;
        j = 0 as i32;
        while j < 3 as i32 {
            mirror_pattern +=
                row[j as usize] * pow3[(2 as i32 - j) as usize];
            j += 1
        }
        *map_mirror3.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0 as i32;
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
    i = 0 as i32;
    while i < 6561 as i32 {
        j = 0 as i32;
        while j < 3 as i32 {
            k = 0 as i32;
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
    i = 0 as i32;
    while i < 9 as i32 { row[i as usize] = 0 as i32; i += 1 }
    i = 0 as i32;
    while i < 19683 as i32 {
        mirror_pattern =
            row[0 as i32 as usize] +
                3 as i32 * row[3 as i32 as usize] +
                9 as i32 * row[6 as i32 as usize] +
                27 as i32 * row[1 as i32 as usize] +
                81 as i32 * row[4 as i32 as usize] +
                243 as i32 * row[7 as i32 as usize] +
                729 as i32 * row[2 as i32 as usize] +
                2187 as i32 * row[5 as i32 as usize] +
                6561 as i32 * row[8 as i32 as usize];
        *map_mirror33.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0 as i32;
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
    i = 0 as i32;
    while i < stage_count - 1 as i32 {
        set[stage[i as usize] as usize].constant =
            (next_word() as i32 / 4 as i32) as
                i16;
        set[stage[i as usize] as usize].parity =
            (next_word() as i32 / 4 as i32) as
                i16;
        set[stage[i as usize] as
            usize].parity_constant[0 as i32 as usize] =
            set[stage[i as usize] as usize].constant;
        set[stage[i as usize] as
            usize].parity_constant[1 as i32 as usize] =
            (set[stage[i as usize] as usize].constant as i32 +
                set[stage[i as usize] as usize].parity as i32) as
                i16;
        unpack_batch(set[stage[i as usize] as usize].afile2x, map_mirror8x2,
                     59049 as i32, next_word);
        unpack_batch(set[stage[i as usize] as usize].bfile, map_mirror8,
                     6561 as i32, next_word);
        unpack_batch(set[stage[i as usize] as usize].cfile, map_mirror8,
                     6561 as i32, next_word);
        unpack_batch(set[stage[i as usize] as usize].dfile, map_mirror8,
                     6561 as i32, next_word);
        unpack_batch(set[stage[i as usize] as usize].diag8, map_mirror8,
                     6561 as i32, next_word);
        unpack_batch(set[stage[i as usize] as usize].diag7, map_mirror7,
                     2187 as i32, next_word);
        unpack_batch(set[stage[i as usize] as usize].diag6, map_mirror6,
                     729 as i32, next_word);
        unpack_batch(set[stage[i as usize] as usize].diag5, map_mirror5,
                     243 as i32, next_word);
        unpack_batch(set[stage[i as usize] as usize].diag4, map_mirror4,
                     81 as i32, next_word);
        unpack_batch(set[stage[i as usize] as usize].corner33, map_mirror33,
                     19683 as i32, next_word);
        unpack_batch(set[stage[i as usize] as usize].corner52,
                     0 as *mut i32, 59049 as i32, next_word);
        i += 1
    }
    /* Free the mirror tables - the symmetries are now implicit
       in the coefficient tables. */
    free(map_mirror3 as *mut c_void);
    free(map_mirror4 as *mut c_void);
    free(map_mirror5 as *mut c_void);
    free(map_mirror6 as *mut c_void);
    free(map_mirror7 as *mut c_void);
    free(map_mirror8 as *mut c_void);
    free(map_mirror33 as *mut c_void);
    free(map_mirror8x2 as *mut c_void);
}


pub unsafe fn process_coeffs_from_fn_source(mut next_word: &mut impl FnMut() -> i16, filename_to_report_eror_for: *mut i8) {
    /* Check the magic values in the beginning of the file to make sure
           the file format is right */
    let mut word1 = next_word() as i32;
    let mut word2 = next_word() as i32;
    if word1 != 5358 as i32 || word2 != 9793 as i32 {
        fatal_error(b"%s: %s\x00" as *const u8 as *const i8,
                    filename_to_report_eror_for,
                    b"Wrong checksum in , might be an old version\x00" as
                        *const u8 as *const i8);
    }
    /* Read the different stages for which the evaluation function
       was tuned and mark the other stages with pointers to the previous
       and next stages. */
    let mut i = 0 as i32;
    while i <= 60 as i32 {
        set[i as usize].permanent = 0 as i32;
        set[i as usize].loaded = 0 as i32;
        i += 1
    }
    stage_count = next_word() as i32;
    let mut i = 0 as i32;
    let mut j: i32 = 0;
    let mut curr_stage: i32 = 0;
    while i < stage_count - 1 as i32 {
        stage[i as usize] = next_word() as i32;
        curr_stage = stage[i as usize];
        if i == 0 as i32 {
            j = 0 as i32;
            while j < stage[0 as i32 as usize] {
                set[j as usize].prev = stage[0 as i32 as usize];
                set[j as usize].next = stage[0 as i32 as usize];
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
        set[curr_stage as usize].permanent = 1 as i32;
        allocate_set(curr_stage);
        i += 1
    }
    stage[(stage_count - 1 as i32) as usize] = 60 as i32;
    j = stage[(stage_count - 2 as i32) as usize];
    while j < 60 as i32 {
        set[j as usize].prev =
            stage[(stage_count - 2 as i32) as usize];
        set[j as usize].next = 60 as i32;
        j += 1
    }
    set[60 as i32 as usize].permanent = 1 as i32;
    allocate_set(60 as i32);
    /* Read the pattern values */
    unpack_coeffs(&mut next_word);
}


pub unsafe fn init_coeffs_calculate_patterns() {
    /* Calculate the patterns which correspond to the board being filled */
    terminal_patterns();
    set[60 as i32 as usize].constant =
        0 as i32 as i16;
    set[60 as i32 as usize].parity =
        0 as i32 as i16;
    set[60 as i32 as usize].parity_constant[0 as i32 as usize]
        = set[60 as i32 as usize].constant;
    set[60 as i32 as usize].parity_constant[1 as i32 as usize]
        =
        (set[60 as i32 as usize].constant as i32 +
            set[60 as i32 as usize].parity as i32) as
            i16;
}
