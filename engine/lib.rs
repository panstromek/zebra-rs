#![feature(label_break_value)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

pub mod src {
    pub use ::bitbcnt::bitbcnt;
    pub use ::bitboard::bitboard;
    pub use ::bitboard::bitbmob;
    pub use ::bitboard::bitbtest;

    pub mod counter;
    pub mod opname;
    pub mod patterns;
    pub use ::pcstat::pcstat;
    pub mod unflip;
    pub mod globals;
    pub mod myrandom;
    pub use ::epcstat::epcstat;
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
    pub mod learn;
    pub mod osfbook;
    pub mod thordb;
    pub mod stubs;
    pub mod timer;
    pub mod error;
    pub mod safemem;
    pub mod display;
}