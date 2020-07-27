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
    let mut sPatternFile: [i8; 260] = [0; 260];
    init_memory_handler();
    /* Linux don't support current directory. */
    strcpy(sPatternFile.as_mut_ptr(),
           b"coeffs2.bin\x00" as *const u8 as *const i8);
    let mut coeff_stream =
        gzopen(sPatternFile.as_mut_ptr(),
               b"rb\x00" as *const u8 as *const i8);
    if coeff_stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Unable to open coefficient file\x00" as *const u8 as
                        *const i8, sPatternFile.as_mut_ptr());
    }
    let mut next_word = || get_word(coeff_stream);
    /* Check the magic values in the beginning of the file to make sure
       the file format is right */
    let mut word1 = next_word() as i32;
    let mut word2 = next_word() as i32;
    if word1 != 5358 as i32 || word2 != 9793 as i32 {
        fatal_error(b"%s: %s\x00" as *const u8 as *const i8,
                    sPatternFile.as_mut_ptr(),
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
    let mut adjust_stream =
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


#[no_mangle]
pub unsafe extern "C"  fn report_mirror_symetry_error(mut count: i32, mut i: i32, first_mirror_offset: i32, first_item: i32, second_item: i32) {
    printf(b"%s @ %d <--> %d of %d\n\x00" as *const u8 as
               *const i8,
           b"Mirror symmetry error\x00" as *const u8 as
               *const i8, i, first_mirror_offset,
           count);
    printf(b"%d <--> %d\n\x00" as *const u8 as
               *const i8,
           first_item,
           second_item);
}

