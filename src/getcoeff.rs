use libc_wrapper::{fclose, fscanf, fopen, printf, gzgetc, gzFile};
use engine::src::getcoeff::{CoeffAdjustments, CoeffSource};
use flate2::read::GzDecoder;
use std::io::Read;

/*
   GET_WORD
   Reads a 16-bit signed integer from a file.
*/
unsafe fn get_word(mut stream: gzFile) -> i16 {
    match try_get_word(stream) {
        Some(w) => w,
        None => panic!("No word in the input stream.")
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub signed_val: i16,
    pub unsigned_val: u16,
}

unsafe fn try_get_word(mut stream: gzFile) -> Option<i16> {
    let mut val = C2RustUnnamed { signed_val: 0 };
    let hi =
        if (*stream).have != 0 {
            (*stream).have = (*stream).have.wrapping_sub(1);
            (*stream).pos += 1;
            let fresh0 = (*stream).next;
            (*stream).next = (*stream).next.offset(1);
            *fresh0 as i32
        } else { gzgetc(stream) };
    if hi == -1 {
        return None;
    }
    let lo = if (*stream).have != 0 {
            (*stream).have = (*stream).have.wrapping_sub(1);
            (*stream).pos += 1;
            let fresh1 = (*stream).next;
            (*stream).next = (*stream).next.offset(1);
            *fresh1 as i32
        } else { gzgetc(stream) };
    if lo == -1 {
        return None;
    }
    val.unsigned_val = ((hi << 8 as i32) + lo) as u16;
    Some(val.signed_val)
}
/*
   File:         getcoeff.h

   Created:      November 20, 1997

   Modified:     August 1, 2002

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:
*/
pub mod zlib_source {
    use libc_wrapper::{gzopen, strcpy, gzclose, gzFile_s};
    use crate::src::getcoeff::{get_word, try_get_word};
    use crate::src::error::fatal_error;
    use engine::src::getcoeff::CoeffSource;

    pub struct ZLibSource {
        coeff_stream: *mut gzFile_s
    }

    impl Drop for ZLibSource {
        fn drop(&mut self) {
            unsafe { gzclose(self.coeff_stream); }
        }
    }

    impl CoeffSource for ZLibSource {
        fn next_word(&mut self) -> i16 {
            unsafe { get_word(self.coeff_stream) }
        }
        fn try_next_word(&mut self) -> Option<i16> {
            unsafe { try_get_word(self.coeff_stream) }
        }
    }
    impl ZLibSource {
        pub fn new() -> Self {
            let mut sPatternFile: [i8; 260] = [0; 260];
            unsafe {
                /* Linux don't support current directory. */
                strcpy(sPatternFile.as_mut_ptr(), b"coeffs2.bin\x00" as *const u8 as *const i8);
                let coeff_stream = gzopen(sPatternFile.as_mut_ptr(), b"rb\x00" as *const u8 as *const i8);
                if coeff_stream.is_null() {
                    fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                                b"Unable to open coefficient file\x00" as *const u8 as
                                    *const i8, sPatternFile.as_mut_ptr());
                }
                let filename_to_report = sPatternFile.as_mut_ptr();
                /* Check the magic values in the beginning of the file to make sure
                       the file format is right */
                let word1 = get_word(coeff_stream) as i32;
                let word2 = get_word(coeff_stream) as i32;
                if word1 != 5358 as i32 || word2 != 9793 as i32 {
                    fatal_error(b"%s: %s\x00" as *const u8 as *const i8,
                                filename_to_report,
                                b"Wrong checksum in , might be an old version\x00" as
                                    *const u8 as *const i8);
                }

                ZLibSource {
                    coeff_stream
                }
            }
        }
    }
}

pub fn load_coeff_adjustments() -> Option<CoeffAdjustments> {
    let adjust_stream = unsafe {
        fopen(b"adjust.txt\x00" as *const u8 as *const i8,
              b"r\x00" as *const u8 as *const i8)
    };
    if !adjust_stream.is_null() {
        let mut disc_adjust = 0.0f64;
        let mut edge_adjust = 0.0f64;
        let mut corner_adjust = 0.0f64;
        let mut x_adjust = 0.0f64;
        unsafe {
            fscanf(adjust_stream,
                   b"%lf %lf %lf %lf\x00" as *const u8 as *const i8,
                   &mut disc_adjust as *mut f64,
                   &mut edge_adjust as *mut f64,
                   &mut corner_adjust as *mut f64,
                   &mut x_adjust as *mut f64);
            fclose(adjust_stream);
        }
        Some(CoeffAdjustments {
            disc_adjust,
            edge_adjust,
            corner_adjust,
            x_adjust
        })
    } else {
        None
    }
}

struct Flate2Source { data: Vec<u8>, index: usize }

impl Flate2Source {
    fn new() -> Flate2Source {

        let file = std::fs::read("coeffs2.bin").unwrap();
        // println!("path {}, size {}", path, file.len());
        let mut decoder = GzDecoder::new(&*file);
        let mut decoded = Vec::new();
        decoder.read_to_end(&mut decoded).unwrap();

        let mut source = Flate2Source {
            data: decoded,
            index: 0,
        };

        let word1 = source.next_word();
        let word2 = source.next_word();

        if word1 != 5358 || word2 != 9793 {
            panic!("Magic words are incorrect in coeff source file.");
        }
        source
    }
}

impl CoeffSource for Flate2Source {
    fn next_word(&mut self) -> i16 {
        self.try_next_word().unwrap()
    }

    fn try_next_word(&mut self) -> Option<i16> {
        let mut val = C2RustUnnamed { signed_val: 0 };

        let hi = *self.data.get(self.index)? as i32;
        self.index += 1;
        let lo = *self.data.get(self.index)? as i32;
        self.index += 1;

        val.unsigned_val = ((hi << 8 as i32) + lo) as u16;
        return Some(unsafe { val.signed_val });
    }
}


#[test]
fn coeff_source_test () {
    use crate::src::getcoeff::zlib_source::ZLibSource;

    let mut z_lib_source = ZLibSource::new();
    let mut flate2_source = Flate2Source::new();

    while let Some(word) = z_lib_source.try_next_word() {
        let flate_word = flate2_source.try_next_word().unwrap();
        assert_eq!(word, flate_word)
    }

    assert!(flate2_source.try_next_word().is_none());
}