use crate::src::stubs::{vfprintf, ctime, fprintf, time, fopen, stderr, exit};
use crate::src::zebra::_IO_FILE;
use engine::src::error::FatalError;
use std::env::args;

pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut std::ffi::c_void,
    pub reg_save_area: *mut std::ffi::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type time_t = __time_t;
/*
   File:       error.h

   Created:    June 13, 1998

   Modified:   August 1, 2002

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The interface to the error handler.
*/
/*
   File:       error.c

   Created:    June 13, 1998

   Modified:   November 12, 2001

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The text-based error handler.
*/
/* not Windows CE */
#[no_mangle]
pub unsafe extern "C" fn fatal_error(format: *const i8,
                                     args: ...) -> ! {
    let mut stream = 0 as *mut FILE;
    let mut timer: time_t = 0;
    let mut arg_ptr: ::std::ffi::VaListImpl;
    arg_ptr = args.clone();
    fprintf(stderr, b"\n%s: \x00" as *const u8 as *const i8,
            b"Fatal error\x00" as *const u8 as *const i8);
    vfprintf(stderr, format, arg_ptr.as_va_list());
    stream =
        fopen(b"zebra.err\x00" as *const u8 as *const i8,
              b"a\x00" as *const u8 as *const i8);
    if !stream.is_null() {
        time(&mut timer);
        fprintf(stream,
                b"%s @ %s\n  \x00" as *const u8 as *const i8,
                b"Fatal error\x00" as *const u8 as *const i8,
                ctime(&mut timer));
        arg_ptr = args.clone();
        vfprintf(stream, format, arg_ptr.as_va_list());
    }
    exit(1 as i32);
}

pub struct LibcFatalError;
impl FatalError for LibcFatalError {
  fn invalid_move(curr_move: i32) -> ! {
    unsafe {
      fatal_error(b"Thor book move %d is invalid!\x00" as *const u8
                      as *const i8, curr_move);
    }
  }

 fn unrecognized_character(unrecognized: i8) -> ! {
  unsafe {
    fatal_error(b"%s \'%c\' %s\n\x00" as *const u8 as
                    *const i8,
                b"Unrecognized character\x00" as *const u8 as
                    *const i8,
                unrecognized as i32,
                b"in game file\x00" as *const u8 as
                    *const i8);
  }
}

unsafe fn cannot_open_game_file(file_name: *const i8) -> ! {
  fatal_error(b"%s \'%s\'\n\x00" as *const u8 as
                  *const i8,
              b"Cannot open game file\x00" as *const u8 as
                  *const i8, file_name);
}


 fn memory_allocation_failure(block_count_: i32) -> ! {
  unsafe {
    fatal_error(b"%s @ #%d\n\x00" as *const u8 as *const i8,
                b"Memory allocation failure\x00" as *const u8 as
                    *const i8, block_count_);
  }
}

fn invalid_move_in_move_sequence(curr_move: i32) -> ! {
  unsafe {
    fatal_error(b"Invalid move %c%c in move sequence\x00"
                    as *const u8 as *const i8,
                'a' as i32 + curr_move % 10 as i32
                    - 1 as i32,
                '0' as i32 +
                    curr_move / 10 as i32);
  }
}

 fn error_in_map(i: i32, pos: i32, symmetry_map_item: i32) -> ! {
  unsafe {
    fatal_error(b"Error in map %d: inv(map(%d))=%d\n\x00" as
                    *const u8 as *const i8, i, pos, symmetry_map_item);
  }
}

 fn internal_error_in_book_code() -> ! {
    unsafe {
        fatal_error(b"Internal error in book code.\x00" as *const u8 as
            *const i8);
    }
}

 fn book_node_list_allocation_failure(size: i32, to_report: u64) -> ! {
    unsafe {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Book node list: Failed to allocate\x00" as *const u8 as
                        *const i8,
                    to_report,
                    size);
    }
}

 fn book_hash_table_allocaiton_failure(new_size: i32, new_memory: i32) -> ! {
    unsafe {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Book hash table: Failed to allocate\x00" as *const u8 as
                        *const i8, new_memory, new_size);
    }
}

 fn safe_malloc_failure(size: u64) -> ! {
    unsafe {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Memory allocation failure when allocating\x00" as
                        *const u8 as *const i8, size);
    }
}

 fn safe_realloc_failure(size: u64) -> ! {
    unsafe {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Memory allocation failure when allocating\x00" as
                        *const u8 as *const i8, size);
    }
}


 fn error_in_map_thor(i: i32, pos: i32, to_report: i32) -> ! {
    unsafe {
        fatal_error(b"Error in map %d: inv(map(%d))=%d\n\x00" as
                        *const u8 as *const i8, i, pos,
                    to_report);
    }
}

 fn unexpected_character_in_a_move_string() -> ! {
    unsafe {
        fatal_error(b"Unexpected character in move string\x00" as
            *const u8 as *const i8);
    }
}

 fn invalid_move_string_provided() -> ! {
    unsafe {
        fatal_error(b"Invalid move string provided\x00" as *const u8
            as *const i8);
    }
}
}
