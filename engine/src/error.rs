

extern "C" {
  #[no_mangle]
  pub fn fatal_error(_: *const i8, _: ...) -> !;
}

pub trait FatalError {
    fn invalid_move(curr_move: i32) -> !;
}

pub struct LibcFatalError;

impl FatalError for LibcFatalError {
  fn invalid_move(curr_move: i32) -> ! {
    unsafe {
      fatal_error(b"Thor book move %d is invalid!\x00" as *const u8
                      as *const i8, curr_move);
    }
  }
}

pub fn unrecognized_character(unrecognized: i8) -> ! {
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

pub unsafe fn cannot_open_game_file(file_name: *const i8) -> ! {
  fatal_error(b"%s \'%s\'\n\x00" as *const u8 as
                  *const i8,
              b"Cannot open game file\x00" as *const u8 as
                  *const i8, file_name);
}


pub fn memory_allocation_failure(block_count_: i32) -> ! {
  unsafe {
    fatal_error(b"%s @ #%d\n\x00" as *const u8 as *const i8,
                b"Memory allocation failure\x00" as *const u8 as
                    *const i8, block_count_);
  }
}

pub fn invalid_move_in_move_sequence(curr_move: i32) -> ! {
  unsafe {
    fatal_error(b"Invalid move %c%c in move sequence\x00"
                    as *const u8 as *const i8,
                'a' as i32 + curr_move % 10 as i32
                    - 1 as i32,
                '0' as i32 +
                    curr_move / 10 as i32);
  }
}

pub fn error_in_map(i: i32, pos: i32, symmetry_map_item: i32) -> ! {
  unsafe {
    fatal_error(b"Error in map %d: inv(map(%d))=%d\n\x00" as
                    *const u8 as *const i8, i, pos, symmetry_map_item);
  }
}

pub fn internal_error_in_book_code() -> ! {
    unsafe {
        fatal_error(b"Internal error in book code.\x00" as *const u8 as
            *const i8);
    }
}

pub fn book_node_list_allocation_failure(size: i32, to_report: u64) -> ! {
    unsafe {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Book node list: Failed to allocate\x00" as *const u8 as
                        *const i8,
                    to_report,
                    size);
    }
}

pub fn book_hash_table_allocaiton_failure(mut new_size: i32, mut new_memory: i32) -> ! {
    unsafe {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Book hash table: Failed to allocate\x00" as *const u8 as
                        *const i8, new_memory, new_size);
    }
}

pub fn safe_malloc_failure(size: u64) -> ! {
    unsafe {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Memory allocation failure when allocating\x00" as
                        *const u8 as *const i8, size);
    }
}

pub fn safe_realloc_failure(size: u64) -> ! {
    unsafe {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const i8,
                    b"Memory allocation failure when allocating\x00" as
                        *const u8 as *const i8, size);
    }
}


pub fn error_in_map_thor(i: i32, pos: i32, to_report: i32) -> ! {
    unsafe {
        fatal_error(b"Error in map %d: inv(map(%d))=%d\n\x00" as
                        *const u8 as *const i8, i, pos,
                    to_report);
    }
}

pub fn unexpected_character_in_a_move_string() -> ! {
    unsafe {
        fatal_error(b"Unexpected character in move string\x00" as
            *const u8 as *const i8);
    }
}

pub fn invalid_move_string_provided() -> ! {
    unsafe {
        fatal_error(b"Invalid move string provided\x00" as *const u8
            as *const i8);
    }
}
