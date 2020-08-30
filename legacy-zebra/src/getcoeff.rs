use libc_wrapper::{fclose, fscanf, fopen};
use engine::src::getcoeff::{CoeffAdjustments};


use crate::src::error::fatal_error;
use std::ffi::CStr;
use zlib_coeff_source::{ZLibSource, LoadError};

pub fn new_z_lib_source(file_name: &CStr) -> ZLibSource {
    match ZLibSource::try_new(file_name) {
        Ok(f) => f,
        Err(LoadError::UnableToOpenCoefficientFile) => unsafe {
            fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                        b"Unable to open coefficient file\x00" as *const u8 as
                            *const i8, file_name.as_ptr());
        },
        Err(LoadError::WrongChecksum) => unsafe {
            fatal_error(b"%s: %s\x00" as *const u8 as *const i8,
                        file_name.as_ptr(),
                        b"Wrong checksum in , might be an old version\x00" as
                            *const u8 as *const i8);
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
