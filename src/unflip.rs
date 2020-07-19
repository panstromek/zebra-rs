/*
   File:           unflip.c

   Created:        February 26, 1999

   Modified:       July 12, 1999

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Low-level code to flip back the discs flipped by a move.
*/
/* Global variables */
use crate::src::libc;

pub static mut global_flip_stack: [*mut libc::c_int; 2048] =
    [0 as *const libc::c_int as *mut libc::c_int; 2048];
// Initialized in run_static_initializers

pub static mut flip_stack: *mut *mut libc::c_int =
    0 as *const *mut libc::c_int as *mut *mut libc::c_int;
/*
   File:          unflip.h

   Created:       February 26, 1999

   Modified:      December 25, 1999

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      Low-level macro code to flip back the discs
                  flipped by a move.
*/
/*
  UNDOFLIPS
  Flip back the FLIP_COUNT topmost squares on GLOBALFLIPSTACK.
  This seems to be the fastest way to do it and doesn't matter
  much anyway as this function consumes a negligible percentage
  of the time.
*/

pub unsafe extern "C" fn UndoFlips(mut flip_count: libc::c_int,
                                   mut oppcol: libc::c_int) {
    let mut UndoFlips__flip_count = flip_count;
    let mut UndoFlips__oppcol = oppcol;
    if UndoFlips__flip_count & 1 as libc::c_int != 0 {
        UndoFlips__flip_count -= 1;
        flip_stack = flip_stack.offset(-1);
        **flip_stack = UndoFlips__oppcol
    }
    while UndoFlips__flip_count != 0 {
        UndoFlips__flip_count -= 2 as libc::c_int;
        flip_stack = flip_stack.offset(-1);
        **flip_stack = UndoFlips__oppcol;
        flip_stack = flip_stack.offset(-1);
        **flip_stack = UndoFlips__oppcol
    };
}
/*
  INIT_FLIP_STACK
  Reset the flip stack.
*/

pub unsafe extern "C" fn init_flip_stack() {
    flip_stack =
        &mut *global_flip_stack.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut *mut libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    flip_stack =
        &mut *global_flip_stack.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut *mut libc::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
