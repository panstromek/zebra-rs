#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(unused)]

pub mod src {
    pub use ::bitbcnt::bitbcnt;
    pub use ::bitboard::bitboard;
    pub use ::bitboard::bitbmob;
    pub use ::bitboard::bitbtest;
    pub use ::my_random as myrandom;

    pub mod counter;
    pub mod opname;
    pub use ::patterns;
    pub use ::pcstat::pcstat;
    pub mod globals;

    pub use ::epcstat::epcstat;
    pub mod search;
    pub mod zebra;
    pub mod moves;
    pub mod hash;
    pub mod cntflip;
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
}