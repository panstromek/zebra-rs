use std::ffi::c_void;
use crate::src::timer::get_real_timer;
use crate::src::error::FrontEnd;

pub static mut echo: i32 = 0;
pub static mut display_pv: i32 = 0;
