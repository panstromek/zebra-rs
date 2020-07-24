use crate::src::stubs::{printf, fabs};
pub use engine::src::timer::*;

#[no_mangle]
pub unsafe extern "C" fn report_ponder_time() {
    printf(b"Ponder time: %.1f s\n\x00" as *const u8 as *const i8,
           current_ponder_time);
    printf(b"Ponder depth: %d\n\x00" as *const u8 as *const i8,
           current_ponder_depth);
}