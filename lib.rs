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



extern crate libc;



pub mod src {
pub mod bitbcnt;
pub mod bitbmob;
pub mod bitboard;
pub mod bitbtest;
pub mod cntflip;
pub mod counter;
pub mod display;
pub mod doflip;
pub mod end;
pub mod epcstat;
pub mod error;
pub mod eval;
pub mod game;
pub mod getcoeff;
pub mod globals;
pub mod hash;
pub mod learn;
pub mod midgame;
pub mod moves;
pub mod myrandom;
pub mod opname;
pub mod osfbook;
pub mod patterns;
pub mod pcstat;
pub mod probcut;
pub mod safemem;
pub mod search;
pub mod stable;
pub mod thordb;
pub mod timer;
pub mod unflip;
pub mod stubs;
pub mod libc;
pub mod zebra;
} // mod src

