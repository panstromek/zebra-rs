use engine::src::getcoeff::{CoeffAdjustments};


use std::ffi::CStr;
use zlib_coeff_source::{ZLibSource, LoadError};
use std::fs::File;
use std::io::Error;
use std::str::{SplitWhitespace, FromStr};
#[macro_use]
use crate::fatal_error;

pub fn new_z_lib_source(file_name: &CStr) -> ZLibSource {
    match ZLibSource::try_new(file_name) {
        Ok(f) => f,
        Err(LoadError::UnableToOpenCoefficientFile) => {
            fatal_error!("{} '{}'\n", "Unable to open coefficient file", &file_name.to_str().unwrap());
        },
        Err(LoadError::WrongChecksum) => {
            fatal_error!("{}: {}", &file_name.to_str().unwrap(), "Wrong checksum in , might be an old version");
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
