use engine::src::getcoeff::{CoeffAdjustments};


use std::str::{FromStr};
#[macro_use]
use crate::fatal_error;
use flate2_coeff_source::{Flate2Source, Flate2SourceError};

pub fn new_coeff_source(file_name: &str) -> Flate2Source {
    match std::fs::read(file_name) {
        Ok(data) => match Flate2Source::try_from_data(&data) {
            Ok(src) => src,
            Err(e) => match e {
                Flate2SourceError::IncorrectMagicWords => fatal_error!("{}: {}", file_name, "Wrong checksum in , might be an old version")
            },
        },
        Err(e) => fatal_error!("{} '{}'\n", "Unable to open coefficient file", file_name),
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
