#![feature(ptr_wrapping_offset_from)]
#![feature(label_break_value)]
#![no_std]

pub mod src {
    pub mod bitbcnt;
    pub mod bitboard;
    pub mod bitbmob;
    pub mod bitbtest;
    pub mod counter;
    pub mod opname;
    pub mod patterns;
    pub mod pcstat;
    pub mod unflip;
    pub mod globals;
    pub mod myrandom;
    pub mod epcstat;
    pub mod search;
    pub mod zebra;
    pub mod moves;
    pub mod hash;
    pub mod cntflip;
    pub mod doflip;
    pub mod eval;
    pub mod stable;
    pub mod probcut;
    pub mod midgame;
    pub mod end;
    pub mod game;
    pub mod getcoeff;
}