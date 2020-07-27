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
    // FIXME
    //  If the file doesn't have more data, the error is handled by assert in
    //  get_word function. I we want to have a different source, it would probably
    //  be better to make the closure return result instead and make the initializaiton
    //  fallible (but initial version can panic I guess, because the midgame solver
    //  doesn't really make sense without these coeffs, as far as I can tell)
    let mut next_word = || get_word(coeff_stream);
    let filename_to_report = sPatternFile.as_mut_ptr();
    process_coeffs_from_fn_source(&mut next_word, filename_to_report);
    gzclose(coeff_stream);
    init_coeffs_calculate_patterns();
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

