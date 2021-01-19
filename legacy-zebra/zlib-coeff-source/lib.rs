/*
   GET_WORD
   Reads a 16-bit signed integer from a file.
*/
use libc_wrapper::{gzgetc, gzFile};
use libc_wrapper::{gzopen, strcpy, gzclose, gzFile_s};
use engine_traits::CoeffSource;
use std::ffi::CStr;

unsafe fn get_word(stream: gzFile) -> i16 {
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

unsafe fn try_get_word(stream: gzFile) -> Option<i16> {
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

#[derive(Debug)]
pub enum LoadError {
    UnableToOpenCoefficientFile,
    WrongChecksum,
}

impl ZLibSource {
    pub fn try_new(file_name: &CStr) -> Result<Self, LoadError> {
        let mut file_name_copy: [i8; 260] = [0; 260];
        unsafe {
            /* Linux don't support current directory. */
            strcpy(file_name_copy.as_mut_ptr(), file_name.as_ptr() as *const u8 as *const i8);
            let coeff_stream = gzopen(file_name_copy.as_mut_ptr(), b"rb\x00" as *const u8 as *const i8);
            if coeff_stream.is_null() {
                return Err(LoadError::UnableToOpenCoefficientFile);
            }
            /* Check the magic values in the beginning of the file to make sure
                   the file format is right */
            let word1 = get_word(coeff_stream) as i32;
            let word2 = get_word(coeff_stream) as i32;

            if word1 != 5358 as i32 || word2 != 9793 as i32 {
                return Err(LoadError::WrongChecksum);
            }

            Ok(ZLibSource {
                coeff_stream
            })
        }
    }
}

