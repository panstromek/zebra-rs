use engine::src::getcoeff::{CoeffAdjustments};


use crate::src::error::{fatal_error_2};
use std::ffi::CStr;
use zlib_coeff_source::{ZLibSource, LoadError};
use std::fs::File;
use std::io::Error;
use std::str::{SplitWhitespace, FromStr};

pub fn new_z_lib_source(file_name: &CStr) -> ZLibSource {
    match ZLibSource::try_new(file_name) {
        Ok(f) => f,
        Err(LoadError::UnableToOpenCoefficientFile) => unsafe {
            fatal_error_2(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                        b"Unable to open coefficient file\x00" as *const u8 as
                            *const i8, file_name.as_ptr());
        },
        Err(LoadError::WrongChecksum) => unsafe {
            fatal_error_2(b"%s: %s\x00" as *const u8 as *const i8,
                        file_name.as_ptr(),
                        b"Wrong checksum in , might be an old version\x00" as
                            *const u8 as *const i8);
        }
    }
}

pub fn load_coeff_adjustments() -> Option<CoeffAdjustments> {
    let adjust_stream = std::fs::read_to_string("adjust.txt").ok()?;
    let mut split = adjust_stream.split_whitespace()
        .map(f64::from_str)
        .filter_map(Result::ok);

    Some(CoeffAdjustments {
        disc_adjust: split.next()?,
        edge_adjust: split.next()?,
        corner_adjust: split.next()?,
        x_adjust: split.next()?,
    })
}
