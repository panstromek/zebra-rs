#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(ptr_wrapping_offset_from)]
#![allow(clippy::missing_safety_doc)]

extern crate libc;

pub mod src {
    pub mod display;
    pub mod end;
    pub mod error;
    pub mod game;
    pub mod getcoeff;
    pub mod learn;
    pub mod midgame;
    pub mod moves;
    pub mod osfbook;
    pub mod search;
    pub mod thordb;
    pub mod timer;
    pub mod stubs;
    pub mod zebra;
}
