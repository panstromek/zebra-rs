use crate::src::stubs::{printf};
pub use engine::src::timer::*;

pub unsafe fn report_ponder_time() {
    printf(b"Ponder time: %.1f s\n\x00" as *const u8 as *const i8,
           current_ponder_time);
    printf(b"Ponder depth: %d\n\x00" as *const u8 as *const i8,
           current_ponder_depth);
}