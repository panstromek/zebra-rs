#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#![allow(clippy::missing_safety_doc)]

extern crate libc;



pub mod src {
pub use ::engine::src::*;
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
} // mod src

