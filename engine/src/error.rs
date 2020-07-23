

pub static mut fatal_error: unsafe extern "C" fn(*const i8, ...) = default_fatal_error;


unsafe extern "C" fn default_fatal_error(mut format: *const i8, mut args: ...) {
  panic!("FATAL ERROR")
}
