use crate::src::libc;
use crate::src::stubs::{fclose, fscanf, fopen, gzclose, gzopen, strcpy, free, printf, gzgetc, exit};
use crate::src::globals::{board, piece_count};
use crate::src::moves::disks_played;
use crate::src::error::fatal_error;
use crate::src::safemem::safe_malloc;
use crate::src::patterns::{flip8, pow3};
use crate::src::zebra::_IO_FILE;
pub use engine::src::getcoeff::*;

pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: u32,
    pub next: *mut u8,
    pub pos: off_t,
}
pub type gzFile = *mut gzFile_s;
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;

/*
   GET_WORD
   Reads a 16-bit signed integer from a file.
*/
unsafe fn get_word(mut stream: gzFile) -> i16 {
    let mut val = C2RustUnnamed{signed_val: 0,};
    let mut hi: i32 = 0;
    let mut lo: i32 = 0;
    hi =
        if (*stream).have != 0 {
            (*stream).have = (*stream).have.wrapping_sub(1);
            (*stream).pos += 1;
            let fresh0 = (*stream).next;
            (*stream).next = (*stream).next.offset(1);
            *fresh0 as i32
        } else { gzgetc(stream) };
    assert_ne!(hi, -(1 as i32));
    lo =
        if (*stream).have != 0 {
            (*stream).have = (*stream).have.wrapping_sub(1);
            (*stream).pos += 1;
            let fresh1 = (*stream).next;
            (*stream).next = (*stream).next.offset(1);
            *fresh1 as i32
        } else { gzgetc(stream) };
    assert_ne!(lo, -(1 as i32));
    val.unsigned_val = ((hi << 8 as i32) + lo) as u16;
    return val.signed_val;
}
/*
   UNPACK_BATCH
   Reads feature values for one specific pattern
*/
unsafe fn unpack_batch(mut item: *mut i16,
                                  mut mirror: *mut i32,
                                  mut count: i32,
                                  mut stream: gzFile) {
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
            *buffer.offset(i as isize) =
                (get_word(stream) as i32 / 4 as i32) as
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
                printf(b"%s @ %d <--> %d of %d\n\x00" as *const u8 as
                           *const i8,
                       b"Mirror symmetry error\x00" as *const u8 as
                           *const i8, i, *mirror.offset(i as isize),
                       count);
                printf(b"%d <--> %d\n\x00" as *const u8 as
                           *const i8,
                       *item.offset(i as isize) as i32,
                       *item.offset(*mirror.offset(i as isize) as isize) as
                           i32);
                exit(1 as i32);
            }
            i += 1
        }
    }
    free(buffer as *mut libc::c_void);
}
/*
   UNPACK_COEFFS
   Reads all feature values for a certain stage. To take care of
   symmetric patterns, mirror tables are calculated.
*/
unsafe fn unpack_coeffs(mut stream: gzFile) {
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
            (get_word(stream) as i32 / 4 as i32) as
                i16;
        set[stage[i as usize] as usize].parity =
            (get_word(stream) as i32 / 4 as i32) as
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
                     59049 as i32, stream);
        unpack_batch(set[stage[i as usize] as usize].bfile, map_mirror8,
                     6561 as i32, stream);
        unpack_batch(set[stage[i as usize] as usize].cfile, map_mirror8,
                     6561 as i32, stream);
        unpack_batch(set[stage[i as usize] as usize].dfile, map_mirror8,
                     6561 as i32, stream);
        unpack_batch(set[stage[i as usize] as usize].diag8, map_mirror8,
                     6561 as i32, stream);
        unpack_batch(set[stage[i as usize] as usize].diag7, map_mirror7,
                     2187 as i32, stream);
        unpack_batch(set[stage[i as usize] as usize].diag6, map_mirror6,
                     729 as i32, stream);
        unpack_batch(set[stage[i as usize] as usize].diag5, map_mirror5,
                     243 as i32, stream);
        unpack_batch(set[stage[i as usize] as usize].diag4, map_mirror4,
                     81 as i32, stream);
        unpack_batch(set[stage[i as usize] as usize].corner33, map_mirror33,
                     19683 as i32, stream);
        unpack_batch(set[stage[i as usize] as usize].corner52,
                     0 as *mut i32, 59049 as i32, stream);
        i += 1
    }
    /* Free the mirror tables - the symmetries are now implicit
       in the coefficient tables. */
    free(map_mirror3 as *mut libc::c_void);
    free(map_mirror4 as *mut libc::c_void);
    free(map_mirror5 as *mut libc::c_void);
    free(map_mirror6 as *mut libc::c_void);
    free(map_mirror7 as *mut libc::c_void);
    free(map_mirror8 as *mut libc::c_void);
    free(map_mirror33 as *mut libc::c_void);
    free(map_mirror8x2 as *mut libc::c_void);
}
/*
   File:         getcoeff.h

   Created:      November 20, 1997

   Modified:     August 1, 2002

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:
*/
/*
   INIT_COEFFS
   Manages the initialization of all relevant tables.
*/

pub unsafe fn init_coeffs() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut word1: i32 = 0;
    let mut word2: i32 = 0;
    let mut curr_stage: i32 = 0;
    let mut coeff_stream = 0 as *mut gzFile_s;
    let mut adjust_stream = 0 as *mut FILE;
    let mut sPatternFile: [i8; 260] = [0; 260];
    init_memory_handler();
    /* Linux don't support current directory. */
    strcpy(sPatternFile.as_mut_ptr(),
           b"coeffs2.bin\x00" as *const u8 as *const i8);
    coeff_stream =
        gzopen(sPatternFile.as_mut_ptr(),
               b"rb\x00" as *const u8 as *const i8);
    if coeff_stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Unable to open coefficient file\x00" as *const u8 as
                        *const i8, sPatternFile.as_mut_ptr());
    }
    /* Check the magic values in the beginning of the file to make sure
       the file format is right */
    word1 = get_word(coeff_stream) as i32;
    word2 = get_word(coeff_stream) as i32;
    if word1 != 5358 as i32 || word2 != 9793 as i32 {
        fatal_error(b"%s: %s\x00" as *const u8 as *const i8,
                    sPatternFile.as_mut_ptr(),
                    b"Wrong checksum in , might be an old version\x00" as
                        *const u8 as *const i8);
    }
    /* Read the different stages for which the evaluation function
       was tuned and mark the other stages with pointers to the previous
       and next stages. */
    i = 0 as i32;
    while i <= 60 as i32 {
        set[i as usize].permanent = 0 as i32;
        set[i as usize].loaded = 0 as i32;
        i += 1
    }
    stage_count = get_word(coeff_stream) as i32;
    i = 0 as i32;
    while i < stage_count - 1 as i32 {
        stage[i as usize] = get_word(coeff_stream) as i32;
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
    unpack_coeffs(coeff_stream);
    gzclose(coeff_stream);
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
    /* Adjust the coefficients so as to reflect the encouragement for
       having lots of discs */
    adjust_stream =
        fopen(b"adjust.txt\x00" as *const u8 as *const i8,
              b"r\x00" as *const u8 as *const i8);
    if !adjust_stream.is_null() {
        let mut disc_adjust = 0.0f64;
        let mut edge_adjust = 0.0f64;
        let mut corner_adjust = 0.0f64;
        let mut x_adjust = 0.0f64;
        fscanf(adjust_stream,
               b"%lf %lf %lf %lf\x00" as *const u8 as *const i8,
               &mut disc_adjust as *mut f64,
               &mut edge_adjust as *mut f64,
               &mut corner_adjust as *mut f64,
               &mut x_adjust as *mut f64);
        eval_adjustment(disc_adjust, edge_adjust, corner_adjust, x_adjust);
        fclose(adjust_stream);
    }
    post_init_coeffs();
}
