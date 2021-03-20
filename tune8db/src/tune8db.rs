#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sqrt(_: f64) -> f64;
    fn fabs(_: f64) -> f64;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn puts(__s: *const i8) -> i32;
    fn fread(__ptr: *mut ::std::ffi::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
    fn fwrite(__ptr: *const ::std::ffi::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
    fn feof(__stream: *mut FILE) -> i32;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn malloc(_: u64) -> *mut ::std::ffi::c_void;
    fn free(__ptr: *mut ::std::ffi::c_void);
    fn exit(_: i32) -> !;
    fn abs(_: i32) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut ::std::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::std::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfoItem {
    pub solution: f64,
    pub gradient: f64,
    pub direction: f64,
    pub pattern: i32,
    pub frequency: i32,
    pub most_common: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CompactPosition {
    pub row_bit_vec: [i16; 8],
    pub side_to_move: i16,
    pub score: i16,
    pub stage: i16,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut ::std::ffi::c_void as *mut *mut i8,
                  10 as i32) as i32;
}

pub static mut objective: f64 = 0.;

pub static mut abs_error_sum: f64 = 0.;

pub static mut max_delta: f64 = 0.;

pub static mut average_delta: f64 = 0.;

pub static mut delta_sum: f64 = 0.;

pub static mut quad_coeff: f64 = 0.;

pub static mut lin_coeff: f64 = 0.;

pub static mut const_coeff: f64 = 0.;

pub static mut total_weight: f64 = 0.;

pub static mut weight: [f64; 61] = [0.; 61];

pub static mut stage_count: i32 = 0;

pub static mut analysis_stage: i32 = 0;

pub static mut update_count: i32 = 0;

pub static mut last_active: i32 = 0;

pub static mut max_positions: i32 = 0;

pub static mut position_count: i32 = 0;

pub static mut max_diff: i32 = 0;

pub static mut relevant_count: i32 = 0;

pub static mut node_count: i32 = 0;

pub static mut interval: i32 = 0;

pub static mut buffer_size: i32 = 0;

pub static mut node_buffer_pos: i32 = 0;

pub static mut short_buffer_pos: i32 = 0;

pub static mut node_allocations: i32 = 0;

pub static mut short_allocations: i32 = 0;

pub static mut stage: [i32; 64] = [0; 64];

pub static mut active: [i32; 61] = [0; 61];

pub static mut compact: [i32; 1048576] = [0; 1048576];

pub static mut mirror: [i32; 6561] = [0; 6561];

pub static mut flip52: [i32; 59049] = [0; 59049];

pub static mut mirror7: [i32; 2187] = [0; 2187];

pub static mut mirror6: [i32; 729] = [0; 729];

pub static mut mirror5: [i32; 243] = [0; 243];

pub static mut mirror4: [i32; 81] = [0; 81];

pub static mut mirror3: [i32; 27] = [0; 27];

pub static mut mirror82x: [i32; 59049] = [0; 59049];

pub static mut identity10: [i32; 59049] = [0; 59049];

pub static mut flip33: [i32; 19683] = [0; 19683];

pub static mut mirror33: [i32; 19683] = [0; 19683];

pub static mut board: [i32; 100] = [0; 100];

pub static mut row_pattern: [i32; 8] = [0; 8];

pub static mut col_pattern: [i32; 8] = [0; 8];

pub static mut diag1_pattern: [i32; 15] = [0; 15];

pub static mut diag2_pattern: [i32; 15] = [0; 15];

pub static mut row_no: [i32; 100] = [0; 100];

pub static mut row_index: [i32; 100] = [0; 100];

pub static mut col_no: [i32; 100] = [0; 100];

pub static mut col_index: [i32; 100] = [0; 100];

pub static mut diag1_no: [i32; 100] = [0; 100];

pub static mut diag1_index: [i32; 100] = [0; 100];

pub static mut diag2_no: [i32; 100] = [0; 100];

pub static mut diag2_index: [i32; 100] = [0; 100];

pub static mut short_buffer: *mut i16 =
    0 as *const i16 as *mut i16;

pub static mut position_list: *mut CompactPosition =
    0 as *const CompactPosition as *mut CompactPosition;

pub static mut constant: InfoItem =
    InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,};

pub static mut parity: InfoItem =
    InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,};

pub static mut afile: [InfoItem; 6561] =
    [InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,}; 6561];

pub static mut bfile: [InfoItem; 6561] =
    [InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,}; 6561];

pub static mut cfile: [InfoItem; 6561] =
    [InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,}; 6561];

pub static mut dfile: [InfoItem; 6561] =
    [InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,}; 6561];

pub static mut corner52: [InfoItem; 59049] =
    [InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,}; 59049];

pub static mut diag8: [InfoItem; 6561] =
    [InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,}; 6561];

pub static mut diag7: [InfoItem; 2187] =
    [InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,}; 2187];

pub static mut diag6: [InfoItem; 729] =
    [InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,}; 729];

pub static mut diag5: [InfoItem; 243] =
    [InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,}; 243];

pub static mut diag4: [InfoItem; 81] =
    [InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,}; 81];

pub static mut corner33: [InfoItem; 19683] =
    [InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,}; 19683];

pub static mut afile2x: [InfoItem; 59049] =
    [InfoItem{solution: 0.,
        gradient: 0.,
        direction: 0.,
        pattern: 0,
        frequency: 0,
        most_common: 0,}; 59049];

pub static mut inverse4: [i32; 81] = [0; 81];

pub static mut inverse5: [i32; 243] = [0; 243];

pub static mut inverse6: [i32; 729] = [0; 729];

pub static mut inverse7: [i32; 2187] = [0; 2187];

pub static mut inverse8: [i32; 6561] = [0; 6561];

pub static mut inverse9: [i32; 19683] = [0; 19683];

pub static mut inverse10: [i32; 59049] = [0; 59049];
/*
  PACK_POSITION
  Pack the information from the line BUFFER into node #INDEX
  in POSITION_LIST. Returns 1 if the position was incorporated,
  otherwise 0.
*/

pub unsafe  fn pack_position(mut buffer: *mut i8,
                                       mut index: i32)
                                       -> i32 {
    let mut black_mask: i32 = 0;
    let mut white_mask: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut mask: i32 = 0;
    let mut stage_0: i32 = 0;
    let mut score: i32 = 0;
    let mut hi_mask: u8 = 0;
    let mut lo_mask: u8 = 0;
    sscanf(buffer.offset(34),
           b"%d %d\x00" as *const u8 as *const i8,
           &mut stage_0 as *mut i32, &mut score as *mut i32);
    if abs(score) > max_diff { return 0 as i32 }
    i = 0;
    while i < 8 as i32 {
        sscanf(buffer.offset((4 as i32 * i) as isize),
               b"%c%c\x00" as *const u8 as *const i8,
               &mut hi_mask as *mut u8,
               &mut lo_mask as *mut u8);
        hi_mask = (hi_mask as i32 - ' ' as i32) as u8;
        lo_mask = (lo_mask as i32 - ' ' as i32) as u8;
        black_mask =
            ((hi_mask as i32) << 4 as i32) +
                lo_mask as i32;
        sscanf(buffer.offset((4 as i32 * i) as
            isize).offset(2),
               b"%c%c\x00" as *const u8 as *const i8,
               &mut hi_mask as *mut u8,
               &mut lo_mask as *mut u8);
        hi_mask = (hi_mask as i32 - ' ' as i32) as u8;
        lo_mask = (lo_mask as i32 - ' ' as i32) as u8;
        white_mask =
            ((hi_mask as i32) << 4 as i32) +
                lo_mask as i32;
        mask = 0;
        j = 0;
        while j < 8 as i32 {
            mask *= 4 as i32;
            if black_mask & (1 as i32) << j != 0 {
                mask += 0 as i32
            } else if white_mask & (1 as i32) << j != 0 {
                mask += 2 as i32
            } else { mask += 1 as i32 }
            j += 1
        }
        (*position_list.offset(index as isize)).row_bit_vec[i as usize] =
            mask as i16;
        i += 1
    }
    match *buffer.offset(33) as i32 {
        42 => {
            (*position_list.offset(index as isize)).side_to_move = 0;
            (*position_list.offset(index as isize)).score =
                score as i16
        }
        79 => {
            (*position_list.offset(index as isize)).side_to_move = 2;
            (*position_list.offset(index as isize)).score =
                -score as i16
        }
        _ => {
            printf(b"Invalid side to move indicator on line %d in input file\n\x00"
                       as *const u8 as *const i8, index);
        }
    }
    (*position_list.offset(index as isize)).stage = stage_0 as i16;
    return 1 as i32;
}
/*
  UNPACK_POSITION
  Expand the 128-bit compressed position into a full board.
*/

pub unsafe  fn unpack_position(mut index: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut mask: i32 = 0;
    /* Nota bene: Some care has to be taken not to mirror-reflect
     each row - the MSDB is the leftmost position on each row. */
    i = 0;
    while i < 8 as i32 {
        mask =
            (*position_list.offset(index as isize)).row_bit_vec[i as usize] as
                i32;
        j = 0;
        pos = 10 as i32 * (i + 1 as i32) + 8 as i32;
        while j < 8 as i32 {
            board[pos as usize] = mask & 3 as i32;
            mask >>= 2 as i32;
            j += 1;
            pos -= 1
        }
        i += 1
    };
}
/*
  DISPLAY_BOARD
  Provides a crude position dump.
*/

pub unsafe  fn display_board(mut index: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    puts(b"\x00" as *const u8 as *const i8);
    i = 1;
    while i <= 8 as i32 {
        write!(stdout, "      ");
        j = 1;
        while j <= 8 as i32 {
            match board[(10 as i32 * i + j) as usize] {
                1 => { write!(stdout, " "); }
                0 => { write!(stdout, "*"); }
                2 => { write!(stdout, "O"); }
                _ => { write!(stdout, "?"); }
            }
            j += 1
        }
        puts(b"\x00" as *const u8 as *const i8);
        i += 1
    }
    puts(b"\x00" as *const u8 as *const i8);
    printf(b"side_to_move=%d\n\x00" as *const u8 as *const i8,
           (*position_list.offset(index as isize)).side_to_move as
               i32);
    printf(b"stage=%d\n\x00" as *const u8 as *const i8,
           (*position_list.offset(index as isize)).stage as i32);
    printf(b"score=%d\n\x00" as *const u8 as *const i8,
           (*position_list.offset(index as isize)).score as i32);
}
/*
   READ_POSITION_FILE
   Reads a game database and creates a game tree containing its games.
*/

pub unsafe  fn read_position_file(mut file_name:
                                            *mut i8) {
    let mut stream: *mut FILE = 0 as *mut FILE;
    let mut buffer: [i8; 100] = [0; 100];
    position_list =
        malloc((max_positions as
            u64).wrapping_mul(::std::mem::size_of::<CompactPosition>()
            as u64)) as
            *mut CompactPosition;
    if position_list.is_null() {
        printf(b"Couldn\'t allocate space for %d positions\n\x00" as *const u8
                   as *const i8, max_positions);
        exit(1 as i32);
    }
    stream = fopen(file_name, b"r\x00" as *const u8 as *const i8);
    if stream.is_null() {
        printf(b"Could not open game file \'%s\'\n\x00" as *const u8 as
                   *const i8, file_name);
        exit(1 as i32);
    }
    position_count = 0;
    loop  {
        fgets(buffer.as_mut_ptr(), 100 as i32, stream);
        if feof(stream) == 0 {
            if pack_position(buffer.as_mut_ptr(), position_count) != 0 {
                position_count += 1
            }
        }
        if !(feof(stream) == 0 && position_count < max_positions) { break ; }
    }
    fclose(stream);
    printf(b"%d positions loaded\n\x00" as *const u8 as *const i8,
           position_count);
    /*
  for ( int i = 0; i < position_count; i++ ) {
    unpack_position( i );
    display_board( i );
  }
  */
}
/*
  COMPUTE_PATTERNS
  Computes the board patterns corresponding to rows, columns
  and diagonals.
*/

pub unsafe  fn compute_patterns() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    i = 0;
    while i < 8 as i32 {
        row_pattern[i as usize] = 0;
        col_pattern[i as usize] = 0;
        i += 1
    }
    i = 0;
    while i < 15 as i32 {
        diag1_pattern[i as usize] = 0;
        diag2_pattern[i as usize] = 0;
        i += 1
    }
    i = 1;
    while i <= 8 as i32 {
        j = 1;
        while j <= 8 as i32 {
            pos = 10 as i32 * i + j;
            row_pattern[row_no[pos as usize] as usize] +=
                board[pos as usize] <<
                    (row_index[pos as usize] << 1 as i32);
            col_pattern[col_no[pos as usize] as usize] +=
                board[pos as usize] <<
                    (col_index[pos as usize] << 1 as i32);
            diag1_pattern[diag1_no[pos as usize] as usize] +=
                board[pos as usize] <<
                    (diag1_index[pos as usize] << 1 as i32);
            diag2_pattern[diag2_no[pos as usize] as usize] +=
                board[pos as usize] <<
                    (diag2_index[pos as usize] << 1 as i32);
            j += 1
        }
        i += 1
    };
}
/*
   SORT
   Sorts an integer vector using bubble-sort.
*/
#[inline]
unsafe  fn sort(mut vec: *mut i32, mut count: i32) {
    let mut i: i32 = 0;
    let mut temp: i32 = 0;
    let mut changed: i32 = 0;
    loop  {
        changed = 0;
        i = 0;
        while i < count - 1 as i32 {
            if *vec.offset(i as isize) >
                *vec.offset((i + 1 as i32) as isize) {
                changed = 1;
                temp = *vec.offset(i as isize);
                *vec.offset(i as isize) =
                    *vec.offset((i + 1 as i32) as isize);
                *vec.offset((i + 1 as i32) as isize) = temp
            }
            i += 1
        }
        if !(changed != 0) { break ; }
    };
}
/*
   DETERMINE_FEATURES
   Takes a position and determines the values of all
   active features
*/

pub unsafe  fn determine_features(mut side_to_move: i32,
                                            mut stage_0: i32,
                                            mut global_parity:
                                            *mut i32,
                                            mut buffer_a: *mut i32,
                                            mut buffer_b: *mut i32,
                                            mut buffer_c: *mut i32,
                                            mut buffer_d: *mut i32,
                                            mut buffer_52: *mut i32,
                                            mut buffer_33: *mut i32,
                                            mut buffer_8: *mut i32,
                                            mut buffer_7: *mut i32,
                                            mut buffer_6: *mut i32,
                                            mut buffer_5: *mut i32,
                                            mut buffer_4: *mut i32) {
    let mut config52: i32 = 0;
    let mut config33: i32 = 0;
    compute_patterns();
    /* Non-pattern measures */
    if stage_0 % 2 as i32 == 1 as i32 {
        *global_parity = 1 as i32
    } else { *global_parity = 0 as i32 }
    /* The A file (with or without adjacent X-squares) */
    if side_to_move == 0 as i32 {
        *buffer_a =
            mirror82x[(compact[row_pattern[0] as
                usize] +
                6561 as i32 *
                    board[22] +
                19683 as i32 *
                    board[27]) as usize];
        *buffer_a.offset(1) =
            mirror82x[(compact[row_pattern[7] as
                usize] +
                6561 as i32 *
                    board[72] +
                19683 as i32 *
                    board[77]) as usize];
        *buffer_a.offset(2) =
            mirror82x[(compact[col_pattern[0] as
                usize] +
                6561 as i32 *
                    board[22] +
                19683 as i32 *
                    board[72]) as usize];
        *buffer_a.offset(3) =
            mirror82x[(compact[col_pattern[7] as
                usize] +
                6561 as i32 *
                    board[27] +
                19683 as i32 *
                    board[77]) as usize]
    } else {
        *buffer_a =
            mirror82x[inverse10[(compact[row_pattern[0] as usize] +
                6561 as i32 *
                    board[22] +
                19683 as i32 *
                    board[27]) as
                usize] as usize];
        *buffer_a.offset(1) =
            mirror82x[inverse10[(compact[row_pattern[7] as usize] +
                6561 as i32 *
                    board[72] +
                19683 as i32 *
                    board[77]) as
                usize] as usize];
        *buffer_a.offset(2) =
            mirror82x[inverse10[(compact[col_pattern[0] as usize] +
                6561 as i32 *
                    board[22] +
                19683 as i32 *
                    board[72]) as
                usize] as usize];
        *buffer_a.offset(3) =
            mirror82x[inverse10[(compact[col_pattern[7] as usize] +
                6561 as i32 *
                    board[27] +
                19683 as i32 *
                    board[77]) as
                usize] as usize]
    }
    /* The B file */
    if side_to_move == 0 as i32 {
        *buffer_b =
            mirror[compact[row_pattern[1] as usize] as
                usize];
        *buffer_b.offset(1) =
            mirror[compact[row_pattern[6] as usize] as
                usize];
        *buffer_b.offset(2) =
            mirror[compact[col_pattern[1] as usize] as
                usize];
        *buffer_b.offset(3) =
            mirror[compact[col_pattern[6] as usize] as
                usize]
    } else {
        *buffer_b =
            mirror[inverse8[compact[row_pattern[1] as
                usize] as usize] as usize];
        *buffer_b.offset(1) =
            mirror[inverse8[compact[row_pattern[6] as
                usize] as usize] as usize];
        *buffer_b.offset(2) =
            mirror[inverse8[compact[col_pattern[1] as
                usize] as usize] as usize];
        *buffer_b.offset(3) =
            mirror[inverse8[compact[col_pattern[6] as
                usize] as usize] as usize]
    }
    /* The C file */
    if side_to_move == 0 as i32 {
        *buffer_c =
            mirror[compact[row_pattern[2] as usize] as
                usize];
        *buffer_c.offset(1) =
            mirror[compact[row_pattern[5] as usize] as
                usize];
        *buffer_c.offset(2) =
            mirror[compact[col_pattern[2] as usize] as
                usize];
        *buffer_c.offset(3) =
            mirror[compact[col_pattern[5] as usize] as
                usize]
    } else {
        *buffer_c =
            mirror[inverse8[compact[row_pattern[2] as
                usize] as usize] as usize];
        *buffer_c.offset(1) =
            mirror[inverse8[compact[row_pattern[5] as
                usize] as usize] as usize];
        *buffer_c.offset(2) =
            mirror[inverse8[compact[col_pattern[2] as
                usize] as usize] as usize];
        *buffer_c.offset(3) =
            mirror[inverse8[compact[col_pattern[5] as
                usize] as usize] as usize]
    }
    /* The D file */
    if side_to_move == 0 as i32 {
        *buffer_d =
            mirror[compact[row_pattern[3] as usize] as
                usize];
        *buffer_d.offset(1) =
            mirror[compact[row_pattern[4] as usize] as
                usize];
        *buffer_d.offset(2) =
            mirror[compact[col_pattern[3] as usize] as
                usize];
        *buffer_d.offset(3) =
            mirror[compact[col_pattern[4] as usize] as
                usize]
    } else {
        *buffer_d =
            mirror[inverse8[compact[row_pattern[3] as
                usize] as usize] as usize];
        *buffer_d.offset(1) =
            mirror[inverse8[compact[row_pattern[4] as
                usize] as usize] as usize];
        *buffer_d.offset(2) =
            mirror[inverse8[compact[col_pattern[3] as
                usize] as usize] as usize];
        *buffer_d.offset(3) =
            mirror[inverse8[compact[col_pattern[4] as
                usize] as usize] as usize]
    }
    /* The 5*2 corner pattern */
    if side_to_move == 0 as i32 {
        /* a1-e1 + a2-e2 */
        config52 =
            (row_pattern[0] & 1023 as i32) +
                ((row_pattern[1] &
                    1023 as i32) << 10 as i32);
        *buffer_52 =
            compact[config52 as usize];
        /* a1-a5 + b1-b5 */
        config52 =
            (col_pattern[0] & 1023 as i32) +
                ((col_pattern[1] &
                    1023 as i32) << 10 as i32);
        *buffer_52.offset(1) =
            compact[config52 as usize];
        /* h1-d1 + h2-d2 */
        config52 =
            (row_pattern[0] >> 6 as i32) +
                ((row_pattern[1] >> 6 as i32)
                    << 10 as i32);
        *buffer_52.offset(2) =
            flip52[compact[config52 as usize] as usize];
        /* h1-h5 + g1-g5 */
        config52 =
            (col_pattern[7] & 1023 as i32) +
                ((col_pattern[6] &
                    1023 as i32) << 10 as i32);
        *buffer_52.offset(3) =
            compact[config52 as usize];
        /* a8-e8 + a7-e7 */
        config52 =
            (row_pattern[7] & 1023 as i32) +
                ((row_pattern[6] &
                    1023 as i32) << 10 as i32);
        *buffer_52.offset(4) =
            compact[config52 as usize];
        /* a8-a4 + b8-b4 */
        config52 =
            (col_pattern[0] >> 6 as i32) +
                ((col_pattern[1] >> 6 as i32)
                    << 10 as i32);
        *buffer_52.offset(5) =
            flip52[compact[config52 as usize] as usize];
        /* h8-d8 + h7-d7 */
        config52 =
            (row_pattern[7] >> 6 as i32) +
                ((row_pattern[6] >> 6 as i32)
                    << 10 as i32);
        *buffer_52.offset(6) =
            flip52[compact[config52 as usize] as usize];
        /* h8-h4 + g8-g4 */
        config52 =
            (col_pattern[7] >> 6 as i32) +
                ((col_pattern[6] >> 6 as i32)
                    << 10 as i32);
        *buffer_52.offset(7) =
            flip52[compact[config52 as usize] as usize]
    } else {
        /* a1-e1 + a2-e2 */
        config52 =
            (row_pattern[0] & 1023 as i32) +
                ((row_pattern[1] &
                    1023 as i32) << 10 as i32);
        *buffer_52 =
            inverse10[compact[config52 as usize] as usize];
        /* a1-a5 + b1-b5 */
        config52 =
            (col_pattern[0] & 1023 as i32) +
                ((col_pattern[1] &
                    1023 as i32) << 10 as i32);
        *buffer_52.offset(1) =
            inverse10[compact[config52 as usize] as usize];
        /* h1-d1 + h2-d2 */
        config52 =
            (row_pattern[0] >> 6 as i32) +
                ((row_pattern[1] >> 6 as i32)
                    << 10 as i32);
        *buffer_52.offset(2) =
            inverse10[flip52[compact[config52 as usize] as usize] as usize];
        /* h1-h5 + g1-g5 */
        config52 =
            (col_pattern[7] & 1023 as i32) +
                ((col_pattern[6] &
                    1023 as i32) << 10 as i32);
        *buffer_52.offset(3) =
            inverse10[compact[config52 as usize] as usize];
        /* a8-e8 + a7-e7 */
        config52 =
            (row_pattern[7] & 1023 as i32) +
                ((row_pattern[6] &
                    1023 as i32) << 10 as i32);
        *buffer_52.offset(4) =
            inverse10[compact[config52 as usize] as usize];
        /* a8-a4 + b8-b4 */
        config52 =
            (col_pattern[0] >> 6 as i32) +
                ((col_pattern[1] >> 6 as i32)
                    << 10 as i32);
        *buffer_52.offset(5) =
            inverse10[flip52[compact[config52 as usize] as usize] as usize];
        /* h8-e8 + h7-e7 */
        config52 =
            (row_pattern[7] >> 6 as i32) +
                ((row_pattern[6] >> 6 as i32)
                    << 10 as i32);
        *buffer_52.offset(6) =
            inverse10[flip52[compact[config52 as usize] as usize] as usize];
        /* h8-h4 + g8-g4 */
        config52 =
            (col_pattern[7] >> 6 as i32) +
                ((col_pattern[6] >> 6 as i32)
                    << 10 as i32);
        *buffer_52.offset(7) =
            inverse10[flip52[compact[config52 as usize] as usize] as usize]
    }
    /* The 3*3 corner pattern */
    if side_to_move == 0 as i32 {
        /* a1-c1 + a2-c2 + a3-c3 */
        config33 =
            (row_pattern[0] & 63 as i32) +
                ((row_pattern[1] & 63 as i32)
                    << 6 as i32) +
                ((row_pattern[2] & 63 as i32)
                    << 12 as i32);
        *buffer_33 =
            mirror33[compact[config33 as usize] as usize];
        /* h1-f1 + h2-f2 + h3-f3 */
        config33 =
            (row_pattern[0] >> 10 as i32) +
                ((row_pattern[1] >> 10 as i32)
                    << 6 as i32) +
                ((row_pattern[2] >> 10 as i32)
                    << 12 as i32);
        *buffer_33.offset(1) =
            mirror33[flip33[compact[config33 as usize] as usize] as usize];
        /* a8-c8 + a7-c7 + a6-c6 */
        config33 =
            (row_pattern[7] & 63 as i32) +
                ((row_pattern[6] & 63 as i32)
                    << 6 as i32) +
                ((row_pattern[5] & 63 as i32)
                    << 12 as i32);
        *buffer_33.offset(2) =
            mirror33[compact[config33 as usize] as usize];
        /* h8-f8 + h7-f7 + h6-f6 */
        config33 =
            (row_pattern[7] >> 10 as i32) +
                ((row_pattern[6] >> 10 as i32)
                    << 6 as i32) +
                ((row_pattern[5] >> 10 as i32)
                    << 12 as i32);
        *buffer_33.offset(3) =
            mirror33[flip33[compact[config33 as usize] as usize] as usize]
    } else {
        /* a1-c1 + a2-c2 + a3-c3 */
        config33 =
            (row_pattern[0] & 63 as i32) +
                ((row_pattern[1] & 63 as i32)
                    << 6 as i32) +
                ((row_pattern[2] & 63 as i32)
                    << 12 as i32);
        *buffer_33 =
            mirror33[inverse9[compact[config33 as usize] as usize] as usize];
        /* h1-f1 + h2-f2 + h3-f3 */
        config33 =
            (row_pattern[0] >> 10 as i32) +
                ((row_pattern[1] >> 10 as i32)
                    << 6 as i32) +
                ((row_pattern[2] >> 10 as i32)
                    << 12 as i32);
        *buffer_33.offset(1) =
            mirror33[inverse9[flip33[compact[config33 as usize] as usize] as
                usize] as usize];
        /* a8-c8 + a7-c7 + a6-c6 */
        config33 =
            (row_pattern[7] & 63 as i32) +
                ((row_pattern[6] & 63 as i32)
                    << 6 as i32) +
                ((row_pattern[5] & 63 as i32)
                    << 12 as i32);
        *buffer_33.offset(2) =
            mirror33[inverse9[compact[config33 as usize] as usize] as usize];
        /* h8-f8 + h7-f7 + h6-f6 */
        config33 =
            (row_pattern[7] >> 10 as i32) +
                ((row_pattern[6] >> 10 as i32)
                    << 6 as i32) +
                ((row_pattern[5] >> 10 as i32)
                    << 12 as i32);
        *buffer_33.offset(3) =
            mirror33[inverse9[flip33[compact[config33 as usize] as usize] as
                usize] as usize]
    }
    /* The diagonals of length 8 */
    if side_to_move == 0 as i32 {
        *buffer_8 =
            mirror[compact[diag1_pattern[7] as usize]
                as usize];
        *buffer_8.offset(1) =
            mirror[compact[diag2_pattern[7] as usize]
                as usize]
    } else {
        *buffer_8 =
            mirror[inverse8[compact[diag1_pattern[7]
                as usize] as usize] as usize];
        *buffer_8.offset(1) =
            mirror[inverse8[compact[diag2_pattern[7]
                as usize] as usize] as usize]
    }
    /* The diagonals of length 7 */
    if side_to_move == 0 as i32 {
        *buffer_7 =
            mirror7[compact[diag1_pattern[6] as usize]
                as usize];
        *buffer_7.offset(1) =
            mirror7[compact[diag1_pattern[8] as usize]
                as usize];
        *buffer_7.offset(2) =
            mirror7[compact[diag2_pattern[6] as usize]
                as usize];
        *buffer_7.offset(3) =
            mirror7[compact[diag2_pattern[8] as usize]
                as usize]
    } else {
        *buffer_7 =
            mirror7[inverse7[compact[diag1_pattern[6]
                as usize] as usize] as usize];
        *buffer_7.offset(1) =
            mirror7[inverse7[compact[diag1_pattern[8]
                as usize] as usize] as usize];
        *buffer_7.offset(2) =
            mirror7[inverse7[compact[diag2_pattern[6]
                as usize] as usize] as usize];
        *buffer_7.offset(3) =
            mirror7[inverse7[compact[diag2_pattern[8]
                as usize] as usize] as usize]
    }
    /* The diagonals of length 6 */
    if side_to_move == 0 as i32 {
        *buffer_6 =
            mirror6[compact[diag1_pattern[5] as usize]
                as usize];
        *buffer_6.offset(1) =
            mirror6[compact[diag1_pattern[9] as usize]
                as usize];
        *buffer_6.offset(2) =
            mirror6[compact[diag2_pattern[5] as usize]
                as usize];
        *buffer_6.offset(3) =
            mirror6[compact[diag2_pattern[9] as usize]
                as usize]
    } else {
        *buffer_6 =
            mirror6[inverse6[compact[diag1_pattern[5]
                as usize] as usize] as usize];
        *buffer_6.offset(1) =
            mirror6[inverse6[compact[diag1_pattern[9]
                as usize] as usize] as usize];
        *buffer_6.offset(2) =
            mirror6[inverse6[compact[diag2_pattern[5]
                as usize] as usize] as usize];
        *buffer_6.offset(3) =
            mirror6[inverse6[compact[diag2_pattern[9]
                as usize] as usize] as usize]
    }
    /* The diagonals of length 5 */
    if side_to_move == 0 as i32 {
        *buffer_5 =
            mirror5[compact[diag1_pattern[4] as usize]
                as usize];
        *buffer_5.offset(1) =
            mirror5[compact[diag1_pattern[10] as
                usize] as usize];
        *buffer_5.offset(2) =
            mirror5[compact[diag2_pattern[4] as usize]
                as usize];
        *buffer_5.offset(3) =
            mirror5[compact[diag2_pattern[10] as
                usize] as usize]
    } else {
        *buffer_5 =
            mirror5[inverse5[compact[diag1_pattern[4]
                as usize] as usize] as usize];
        *buffer_5.offset(1) =
            mirror5[inverse5[compact[diag1_pattern[10]
                as usize] as usize] as usize];
        *buffer_5.offset(2) =
            mirror5[inverse5[compact[diag2_pattern[4]
                as usize] as usize] as usize];
        *buffer_5.offset(3) =
            mirror5[inverse5[compact[diag2_pattern[10]
                as usize] as usize] as usize]
    }
    /* The diagonals of length 4 */
    if side_to_move == 0 as i32 {
        *buffer_4 =
            mirror4[compact[diag1_pattern[3] as usize]
                as usize];
        *buffer_4.offset(1) =
            mirror4[compact[diag1_pattern[11] as
                usize] as usize];
        *buffer_4.offset(2) =
            mirror4[compact[diag2_pattern[3] as usize]
                as usize];
        *buffer_4.offset(3) =
            mirror4[compact[diag2_pattern[11] as
                usize] as usize]
    } else {
        *buffer_4 =
            mirror4[inverse4[compact[diag1_pattern[3]
                as usize] as usize] as usize];
        *buffer_4.offset(1) =
            mirror4[inverse4[compact[diag1_pattern[11]
                as usize] as usize] as usize];
        *buffer_4.offset(2) =
            mirror4[inverse4[compact[diag2_pattern[3]
                as usize] as usize] as usize];
        *buffer_4.offset(3) =
            mirror4[inverse4[compact[diag2_pattern[11]
                as usize] as usize] as usize]
    };
}
/*
   PERFORM_ANALYSIS
   Updates frequency counts.
*/

pub unsafe  fn perform_analysis(mut index: i32) {
    let mut _coeff: i32 = 0;
    let mut start: i32 = 0;
    let mut stop: i32 = 0;
    let mut global_parity: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut stage_0: i32 = 0;
    let mut buffer_a: [i32; 4] = [0; 4];
    let mut buffer_b: [i32; 4] = [0; 4];
    let mut buffer_c: [i32; 4] = [0; 4];
    let mut buffer_d: [i32; 4] = [0; 4];
    let mut buffer_52: [i32; 8] = [0; 8];
    let mut buffer_33: [i32; 4] = [0; 4];
    let mut buffer_8: [i32; 4] = [0; 4];
    let mut buffer_7: [i32; 4] = [0; 4];
    let mut buffer_6: [i32; 4] = [0; 4];
    let mut buffer_5: [i32; 4] = [0; 4];
    let mut buffer_4: [i32; 4] = [0; 4];
    side_to_move =
        (*position_list.offset(index as isize)).side_to_move as i32;
    stage_0 = (*position_list.offset(index as isize)).stage as i32;
    determine_features(side_to_move, stage_0, &mut global_parity,
                       buffer_a.as_mut_ptr(), buffer_b.as_mut_ptr(),
                       buffer_c.as_mut_ptr(), buffer_d.as_mut_ptr(),
                       buffer_52.as_mut_ptr(), buffer_33.as_mut_ptr(),
                       buffer_8.as_mut_ptr(), buffer_7.as_mut_ptr(),
                       buffer_6.as_mut_ptr(), buffer_5.as_mut_ptr(),
                       buffer_4.as_mut_ptr());
    /* The D file */
    sort(buffer_d.as_mut_ptr(), 4 as i32);
    start = 0;
    loop  {
        stop = start + 1 as i32;
        while stop < 4 as i32 &&
            buffer_d[stop as usize] == buffer_d[start as usize] {
            stop += 1
        }
        _coeff = stop - start;
        dfile[buffer_d[start as usize] as usize].frequency += 1;
        start = stop;
        if !(start < 4 as i32) { break ; }
    }
    /* The C file */
    sort(buffer_c.as_mut_ptr(), 4 as i32);
    start = 0;
    loop  {
        stop = start + 1 as i32;
        while stop < 4 as i32 &&
            buffer_c[stop as usize] == buffer_c[start as usize] {
            stop += 1
        }
        _coeff = stop - start;
        cfile[buffer_c[start as usize] as usize].frequency += 1;
        start = stop;
        if !(start < 4 as i32) { break ; }
    }
    /* The B file */
    sort(buffer_b.as_mut_ptr(), 4 as i32);
    start = 0;
    loop  {
        stop = start + 1 as i32;
        while stop < 4 as i32 &&
            buffer_b[stop as usize] == buffer_b[start as usize] {
            stop += 1
        }
        _coeff = stop - start;
        bfile[buffer_b[start as usize] as usize].frequency += 1;
        start = stop;
        if !(start < 4 as i32) { break ; }
    }
    /* The A file */
    sort(buffer_a.as_mut_ptr(), 4 as i32);
    start = 0;
    loop  {
        stop = start + 1 as i32;
        while stop < 4 as i32 &&
            buffer_a[stop as usize] == buffer_a[start as usize] {
            stop += 1
        }
        _coeff = stop - start;
        afile2x[buffer_a[start as usize] as usize].frequency += 1;
        start = stop;
        if !(start < 4 as i32) { break ; }
    }
    /* The diagonals of length 8 */
    sort(buffer_8.as_mut_ptr(), 2 as i32);
    start = 0;
    loop  {
        stop = start + 1 as i32;
        while stop < 2 as i32 &&
            buffer_8[stop as usize] == buffer_8[start as usize] {
            stop += 1
        }
        _coeff = stop - start;
        diag8[buffer_8[start as usize] as usize].frequency += 1;
        start = stop;
        if !(start < 2 as i32) { break ; }
    }
    /* The diagonals of length 7 */
    sort(buffer_7.as_mut_ptr(), 4 as i32);
    start = 0;
    loop  {
        stop = start + 1 as i32;
        while stop < 4 as i32 &&
            buffer_7[stop as usize] == buffer_7[start as usize] {
            stop += 1
        }
        _coeff = stop - start;
        diag7[buffer_7[start as usize] as usize].frequency += 1;
        start = stop;
        if !(start < 4 as i32) { break ; }
    }
    /* The diagonals of length 6 */
    sort(buffer_6.as_mut_ptr(), 4 as i32);
    start = 0;
    loop  {
        stop = start + 1 as i32;
        while stop < 4 as i32 &&
            buffer_6[stop as usize] == buffer_6[start as usize] {
            stop += 1
        }
        _coeff = stop - start;
        diag6[buffer_6[start as usize] as usize].frequency += 1;
        start = stop;
        if !(start < 4 as i32) { break ; }
    }
    /* The diagonals of length 5 */
    sort(buffer_5.as_mut_ptr(), 4 as i32);
    start = 0;
    loop  {
        stop = start + 1 as i32;
        while stop < 4 as i32 &&
            buffer_5[stop as usize] == buffer_5[start as usize] {
            stop += 1
        }
        _coeff = stop - start;
        diag5[buffer_5[start as usize] as usize].frequency += 1;
        start = stop;
        if !(start < 4 as i32) { break ; }
    }
    /* The diagonals of length 4 */
    sort(buffer_4.as_mut_ptr(), 4 as i32);
    start = 0;
    loop  {
        stop = start + 1 as i32;
        while stop < 4 as i32 &&
            buffer_4[stop as usize] == buffer_4[start as usize] {
            stop += 1
        }
        _coeff = stop - start;
        diag4[buffer_4[start as usize] as usize].frequency += 1;
        start = stop;
        if !(start < 4 as i32) { break ; }
    }
    /* The 5*2 corner pattern */
    sort(buffer_52.as_mut_ptr(), 8 as i32);
    start = 0;
    loop  {
        stop = start + 1 as i32;
        while stop < 8 as i32 &&
            buffer_52[stop as usize] == buffer_52[start as usize] {
            stop += 1
        }
        _coeff = stop - start;
        corner52[buffer_52[start as usize] as usize].frequency += 1;
        start = stop;
        if !(start < 8 as i32) { break ; }
    }
    /* The 3*3 corner pattern */
    sort(buffer_33.as_mut_ptr(), 4 as i32);
    start = 0;
    loop  {
        stop = start + 1 as i32;
        while stop < 4 as i32 &&
            buffer_33[stop as usize] == buffer_33[start as usize] {
            stop += 1
        }
        _coeff = stop - start;
        corner33[buffer_33[start as usize] as usize].frequency += 1;
        start = stop;
        if !(start < 4 as i32) { break ; }
    };
}
/*
   PERFORM_EVALUATION
   Updates the gradient based on the position BRANCH.
*/

pub unsafe  fn perform_evaluation(mut index: i32) {
    let mut error: f64 = 0.;
    let mut grad_contrib: f64 = 0.;
    let mut curr_weight: f64 = 0.;
    let mut i: i32 = 0;
    let mut global_parity: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut stage_0: i32 = 0;
    let mut buffer_a: [i32; 4] = [0; 4];
    let mut buffer_b: [i32; 4] = [0; 4];
    let mut buffer_c: [i32; 4] = [0; 4];
    let mut buffer_d: [i32; 4] = [0; 4];
    let mut buffer_52: [i32; 8] = [0; 8];
    let mut buffer_33: [i32; 4] = [0; 4];
    let mut buffer_8: [i32; 4] = [0; 4];
    let mut buffer_7: [i32; 4] = [0; 4];
    let mut buffer_6: [i32; 4] = [0; 4];
    let mut buffer_5: [i32; 4] = [0; 4];
    let mut buffer_4: [i32; 4] = [0; 4];
    /* Get the pattern values */
    side_to_move =
        (*position_list.offset(index as isize)).side_to_move as i32;
    stage_0 = (*position_list.offset(index as isize)).stage as i32;
    determine_features(side_to_move, stage_0, &mut global_parity,
                       buffer_a.as_mut_ptr(), buffer_b.as_mut_ptr(),
                       buffer_c.as_mut_ptr(), buffer_d.as_mut_ptr(),
                       buffer_52.as_mut_ptr(), buffer_33.as_mut_ptr(),
                       buffer_8.as_mut_ptr(), buffer_7.as_mut_ptr(),
                       buffer_6.as_mut_ptr(), buffer_5.as_mut_ptr(),
                       buffer_4.as_mut_ptr());
    /* Calculate the contribution to the gradient and the objective */
    error =
        -((*position_list.offset(index as isize)).score as i32) as
            f64;
    curr_weight = weight[stage_0 as usize];
    total_weight += curr_weight;
    error += constant.solution;
    error += parity.solution * global_parity as f64;
    i = 0;
    while i < 4 as i32 {
        error += afile2x[buffer_a[i as usize] as usize].solution;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += bfile[buffer_b[i as usize] as usize].solution;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += cfile[buffer_c[i as usize] as usize].solution;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += dfile[buffer_d[i as usize] as usize].solution;
        i += 1
    }
    i = 0;
    while i < 8 as i32 {
        error += corner52[buffer_52[i as usize] as usize].solution;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += corner33[buffer_33[i as usize] as usize].solution;
        i += 1
    }
    i = 0;
    while i < 2 as i32 {
        error += diag8[buffer_8[i as usize] as usize].solution;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += diag7[buffer_7[i as usize] as usize].solution;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += diag6[buffer_6[i as usize] as usize].solution;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += diag5[buffer_5[i as usize] as usize].solution;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += diag4[buffer_4[i as usize] as usize].solution;
        i += 1
    }
    error *= curr_weight;
    objective += error * error;
    abs_error_sum += fabs(error);
    grad_contrib = 2.0f64 * curr_weight * error;
    constant.gradient += grad_contrib;
    parity.gradient += grad_contrib * global_parity as f64;
    i = 0;
    while i < 4 as i32 {
        afile2x[buffer_a[i as usize] as usize].gradient += grad_contrib;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        bfile[buffer_b[i as usize] as usize].gradient += grad_contrib;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        cfile[buffer_c[i as usize] as usize].gradient += grad_contrib;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        dfile[buffer_d[i as usize] as usize].gradient += grad_contrib;
        i += 1
    }
    i = 0;
    while i < 8 as i32 {
        corner52[buffer_52[i as usize] as usize].gradient += grad_contrib;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        corner33[buffer_33[i as usize] as usize].gradient += grad_contrib;
        i += 1
    }
    i = 0;
    while i < 2 as i32 {
        diag8[buffer_8[i as usize] as usize].gradient += grad_contrib;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        diag7[buffer_7[i as usize] as usize].gradient += grad_contrib;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        diag6[buffer_6[i as usize] as usize].gradient += grad_contrib;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        diag5[buffer_5[i as usize] as usize].gradient += grad_contrib;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        diag4[buffer_4[i as usize] as usize].gradient += grad_contrib;
        i += 1
    };
}
/*
   PERFORM_STEP_UPDATE
   Updates the parameters used to determine the optimal step length
   based on the position BRANCH.
*/

pub unsafe  fn perform_step_update(mut index: i32) {
    let mut error: f64 = 0.;
    let mut grad_contrib: f64 = 0.;
    let mut curr_weight: f64 = 0.;
    let mut i: i32 = 0;
    let mut global_parity: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut stage_0: i32 = 0;
    let mut buffer_a: [i32; 4] = [0; 4];
    let mut buffer_b: [i32; 4] = [0; 4];
    let mut buffer_c: [i32; 4] = [0; 4];
    let mut buffer_d: [i32; 4] = [0; 4];
    let mut buffer_52: [i32; 8] = [0; 8];
    let mut buffer_33: [i32; 4] = [0; 4];
    let mut buffer_8: [i32; 4] = [0; 4];
    let mut buffer_7: [i32; 4] = [0; 4];
    let mut buffer_6: [i32; 4] = [0; 4];
    let mut buffer_5: [i32; 4] = [0; 4];
    let mut buffer_4: [i32; 4] = [0; 4];
    /* Get the pattern values */
    side_to_move =
        (*position_list.offset(index as isize)).side_to_move as i32;
    stage_0 = (*position_list.offset(index as isize)).stage as i32;
    determine_features(side_to_move, stage_0, &mut global_parity,
                       buffer_a.as_mut_ptr(), buffer_b.as_mut_ptr(),
                       buffer_c.as_mut_ptr(), buffer_d.as_mut_ptr(),
                       buffer_52.as_mut_ptr(), buffer_33.as_mut_ptr(),
                       buffer_8.as_mut_ptr(), buffer_7.as_mut_ptr(),
                       buffer_6.as_mut_ptr(), buffer_5.as_mut_ptr(),
                       buffer_4.as_mut_ptr());
    /* Calculate the contribution to the gradient and the objective */
    error =
        -((*position_list.offset(index as isize)).score as i32) as
            f64;
    grad_contrib = 0.0f64;
    error += constant.solution;
    grad_contrib += constant.direction;
    error += parity.solution * global_parity as f64;
    grad_contrib += parity.direction * global_parity as f64;
    i = 0;
    while i < 4 as i32 {
        error += afile2x[buffer_a[i as usize] as usize].solution;
        grad_contrib += afile2x[buffer_a[i as usize] as usize].direction;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += bfile[buffer_b[i as usize] as usize].solution;
        grad_contrib += bfile[buffer_b[i as usize] as usize].direction;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += cfile[buffer_c[i as usize] as usize].solution;
        grad_contrib += cfile[buffer_c[i as usize] as usize].direction;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += dfile[buffer_d[i as usize] as usize].solution;
        grad_contrib += dfile[buffer_d[i as usize] as usize].direction;
        i += 1
    }
    i = 0;
    while i < 8 as i32 {
        error += corner52[buffer_52[i as usize] as usize].solution;
        grad_contrib += corner52[buffer_52[i as usize] as usize].direction;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += corner33[buffer_33[i as usize] as usize].solution;
        grad_contrib += corner33[buffer_33[i as usize] as usize].direction;
        i += 1
    }
    i = 0;
    while i < 2 as i32 {
        error += diag8[buffer_8[i as usize] as usize].solution;
        grad_contrib += diag8[buffer_8[i as usize] as usize].direction;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += diag7[buffer_7[i as usize] as usize].solution;
        grad_contrib += diag7[buffer_7[i as usize] as usize].direction;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += diag6[buffer_6[i as usize] as usize].solution;
        grad_contrib += diag6[buffer_6[i as usize] as usize].direction;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += diag5[buffer_5[i as usize] as usize].solution;
        grad_contrib += diag5[buffer_5[i as usize] as usize].direction;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        error += diag4[buffer_4[i as usize] as usize].solution;
        grad_contrib += diag4[buffer_4[i as usize] as usize].direction;
        i += 1
    }
    curr_weight = weight[stage_0 as usize];
    error *= curr_weight;
    grad_contrib *= curr_weight;
    grad_contrib /= total_weight;
    quad_coeff += grad_contrib * grad_contrib;
    lin_coeff += 2.0f64 * grad_contrib * error;
    const_coeff += error * error;
}
/*
   PERFORM_ACTION
   A wrapper to the function given by the function pointer BFUNC.
*/
#[inline]
unsafe fn perform_action(mut bfunc: Option<unsafe fn(_: i32) -> ()>, mut index: i32) {
    node_count += 1;
    if active[(*position_list.offset(index as isize)).stage as usize] != 0 {
        relevant_count += 1;
        if interval != 0 as i32 &&
            relevant_count % interval == 0 as i32 {
            printf(b" %d\x00" as *const u8 as *const i8,
                   relevant_count);
            fflush(stdout);
        }
        unpack_position(index);
        bfunc.expect("non-null function pointer")(index);
    };
}
/*
   ITERATE
   Applies the function BFUNC to all the (relevant)
   positions in the position list.
*/
pub unsafe fn iterate(mut bfunc: Option<unsafe fn(_: i32) -> ()>) {
    let mut index: i32 = 0;
    index = 0;
    while index < position_count { perform_action(bfunc, index); index += 1 };
}
/*
   ANALYZE_GAMES
   Creates frequency statistics.
*/
pub unsafe fn analyze_games() {
    node_count = 0;
    relevant_count = 0;
    interval = 0;
    iterate(Some(perform_analysis as
        unsafe  fn(_: i32) -> ()));
}
/*
   EVALUATE_GAMES
   Determines the gradient for all patterns.
*/
pub unsafe  fn evaluate_games() {
    node_count = 0;
    relevant_count = 0;
    iterate(Some(perform_evaluation as unsafe fn(_: i32) -> ()));
}
/*
   DETERMINE_GAMES
   Determines the optimal step length.
*/
pub unsafe  fn determine_games() {
    node_count = 0;
    relevant_count = 0;
    iterate(Some(perform_step_update as unsafe fn(_: i32) -> ()));
}
/*
   PATTERN_SETUP
   Creates a bunch of maps between patterns.
*/
pub unsafe fn pattern_setup() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut pos: i32 = 0;
    let mut pattern: i32 = 0;
    let mut mirror_pattern: i32 = 0;
    let mut power3: i32 = 0;
    let mut flip8: [i32; 6561] = [0; 6561];
    let mut flip5: [i32; 81] = [0; 81];
    let mut flip3: [i32; 27] = [0; 27];
    let mut row: [i32; 10] = [0; 10];
    /* The inverse patterns */
    i = 0;
    while i < 81 as i32 {
        inverse4[i as usize] = 80 as i32 - i;
        i += 1
    }
    i = 0;
    while i < 243 as i32 {
        inverse5[i as usize] = 242 as i32 - i;
        i += 1
    }
    i = 0;
    while i < 729 as i32 {
        inverse6[i as usize] = 728 as i32 - i;
        i += 1
    }
    i = 0;
    while i < 2187 as i32 {
        inverse7[i as usize] = 2186 as i32 - i;
        i += 1
    }
    i = 0;
    while i < 6561 as i32 {
        inverse8[i as usize] = 6560 as i32 - i;
        i += 1
    }
    i = 0;
    while i < 19683 as i32 {
        inverse9[i as usize] = 19682 as i32 - i;
        i += 1
    }
    i = 0;
    while i < 59049 as i32 {
        inverse10[i as usize] = 59048 as i32 - i;
        i += 1
    }
    /* Build the common pattern maps */
    i = 0;
    while i < 10 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 59049 as i32 {
        pattern = 0;
        j = 0;
        while j < 10 as i32 {
            /* Create the corresponding pattern. */
            pattern |= row[j as usize] << (j << 1 as i32);
            j += 1
        }
        /* Map the base-4 pattern onto the corresponding base-3 pattern */
        compact[pattern as usize] = i;
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 10 as i32) {
                break ;
            }
        }
        i += 1
    }
    /* Build the pattern tables for 8*1-patterns */
    i = 0;
    while i < 8 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 6561 as i32 {
        mirror_pattern = 0;
        power3 = 1;
        j = 7;
        while j >= 0 as i32 {
            mirror_pattern += row[j as usize] * power3;
            power3 *= 3 as i32;
            j -= 1
        }
        /* Create the symmetry map */
        mirror[i as usize] =
            if i < mirror_pattern { i } else { mirror_pattern };
        flip8[i as usize] = mirror_pattern;
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 8 as i32) {
                break ;
            }
        }
        i += 1
    }
    /* Build the tables for 7*1-patterns */
    i = 0;
    while i < 7 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 2187 as i32 {
        mirror_pattern = 0;
        power3 = 1;
        j = 6;
        while j >= 0 as i32 {
            mirror_pattern += row[j as usize] * power3;
            power3 *= 3 as i32;
            j -= 1
        }
        mirror7[i as usize] =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 7 as i32) {
                break ;
            }
        }
        i += 1
    }
    /* Build the tables for 6*1-patterns */
    i = 0;
    while i < 6 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 729 as i32 {
        mirror_pattern = 0;
        power3 = 1;
        j = 5;
        while j >= 0 as i32 {
            mirror_pattern += row[j as usize] * power3;
            power3 *= 3 as i32;
            j -= 1
        }
        mirror6[i as usize] =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 6 as i32) {
                break ;
            }
        }
        i += 1
    }
    /* Build the tables for 5*1-patterns */
    i = 0;
    while i < 5 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 243 as i32 {
        mirror_pattern = 0;
        power3 = 1;
        j = 4;
        while j >= 0 as i32 {
            mirror_pattern += row[j as usize] * power3;
            power3 *= 3 as i32;
            j -= 1
        }
        mirror5[i as usize] =
            if i < mirror_pattern { i } else { mirror_pattern };
        flip5[i as usize] = mirror_pattern;
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 5 as i32) {
                break ;
            }
        }
        i += 1
    }
    /* Build the tables for 4*1-patterns */
    i = 0;
    while i < 4 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 81 as i32 {
        mirror_pattern = 0;
        power3 = 1;
        j = 3;
        while j >= 0 as i32 {
            mirror_pattern += row[j as usize] * power3;
            power3 *= 3 as i32;
            j -= 1
        }
        mirror4[i as usize] =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 4 as i32) {
                break ;
            }
        }
        i += 1
    }
    /* Build the tables for 3*1-patterns */
    i = 0;
    while i < 3 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 27 as i32 {
        mirror_pattern = 0;
        power3 = 1;
        j = 2;
        while j >= 0 as i32 {
            mirror_pattern += row[j as usize] * power3;
            power3 *= 3 as i32;
            j -= 1
        }
        mirror3[i as usize] =
            if i < mirror_pattern { i } else { mirror_pattern };
        flip3[i as usize] = mirror_pattern;
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 3 as i32) {
                break ;
            }
        }
        i += 1
    }
    /* Build the tables for 5*2-patterns */
    i = 0;
    while i < 243 as i32 {
        j = 0;
        while j < 243 as i32 {
            flip52[(243 as i32 * i + j) as usize] =
                243 as i32 * flip5[i as usize] + flip5[j as usize];
            j += 1
        }
        i += 1
    }
    i = 0;
    while i < 59049 as i32 { identity10[i as usize] = i; i += 1 }
    /* Build the tables for 3*3-patterns */
    i = 0;
    while i < 27 as i32 {
        j = 0;
        while j < 27 as i32 {
            k = 0;
            while k < 27 as i32 {
                flip33[(729 as i32 * i + 27 as i32 * j + k) as
                    usize] =
                    729 as i32 * flip3[i as usize] +
                        27 as i32 * flip3[j as usize] +
                        flip3[k as usize];
                k += 1
            }
            j += 1
        }
        i += 1
    }
    i = 0;
    while i < 9 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
    while i < 19683 as i32 {
        mirror_pattern =
            row[0] +
                3 as i32 * row[3] +
                9 as i32 * row[6] +
                27 as i32 * row[1] +
                81 as i32 * row[4] +
                243 as i32 * row[7] +
                729 as i32 * row[2] +
                2187 as i32 * row[5] +
                6561 as i32 * row[8];
        mirror33[i as usize] =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 9 as i32) {
                break ;
            }
        }
        i += 1
    }
    /* Build the tables for edge2X-patterns */
    i = 0;
    while i < 6561 as i32 {
        j = 0;
        while j < 3 as i32 {
            k = 0;
            while k < 3 as i32 {
                mirror82x[(i + 6561 as i32 * j +
                    19683 as i32 * k) as usize] =
                    if flip8[i as usize] + 6561 as i32 * k +
                        19683 as i32 * j <
                        i + 6561 as i32 * j +
                            19683 as i32 * k {
                        (flip8[i as usize] + 6561 as i32 * k) +
                            19683 as i32 * j
                    } else {
                        (i + 6561 as i32 * j) +
                            19683 as i32 * k
                    };
                k += 1
            }
            j += 1
        }
        i += 1
    }
    /* Create the connections position <--> patterns affected */
    i = 1;
    while i <= 8 as i32 {
        j = 1;
        while j <= 8 as i32 {
            pos = 10 as i32 * i + j;
            row_no[pos as usize] = i - 1 as i32;
            row_index[pos as usize] = j - 1 as i32;
            col_no[pos as usize] = j - 1 as i32;
            col_index[pos as usize] = i - 1 as i32;
            diag1_no[pos as usize] = j - i + 7 as i32;
            if i >= j {
                /* First 8 diagonals */
                diag1_index[pos as usize] = j - 1 as i32
            } else { diag1_index[pos as usize] = i - 1 as i32 }
            diag2_no[pos as usize] = j + i - 2 as i32;
            if i + j <= 9 as i32 {
                /* First 8 diagonals */
                diag2_index[pos as usize] = j - 1 as i32
            } else { diag2_index[pos as usize] = 8 as i32 - i }
            j += 1
        }
        i += 1
    }
    /* Reset the coefficients for the different patterns */
    constant.solution = 0.0f64;
    constant.direction = 0.0f64;
    parity.solution = 0.0f64;
    parity.direction = 0.0f64;
    i = 0;
    while i < 59049 as i32 {
        afile2x[i as usize].pattern = i;
        afile2x[i as usize].frequency = 0;
        afile2x[i as usize].direction = 0.0f64;
        afile2x[i as usize].most_common = 0;
        corner52[i as usize].pattern = i;
        corner52[i as usize].frequency = 0;
        corner52[i as usize].direction = 0.0f64;
        corner52[i as usize].most_common = 0;
        i += 1
    }
    i = 0;
    while i < 19683 as i32 {
        corner33[i as usize].pattern = i;
        corner33[i as usize].frequency = 0;
        corner33[i as usize].direction = 0.0f64;
        corner33[i as usize].most_common = 0;
        i += 1
    }
    i = 0;
    while i < 6561 as i32 {
        afile[i as usize].pattern = i;
        afile[i as usize].frequency = 0;
        afile[i as usize].direction = 0.0f64;
        afile[i as usize].most_common = 0;
        bfile[i as usize].pattern = i;
        bfile[i as usize].frequency = 0;
        bfile[i as usize].direction = 0.0f64;
        bfile[i as usize].most_common = 0;
        cfile[i as usize].pattern = i;
        cfile[i as usize].frequency = 0;
        cfile[i as usize].direction = 0.0f64;
        cfile[i as usize].most_common = 0;
        dfile[i as usize].pattern = i;
        dfile[i as usize].frequency = 0;
        dfile[i as usize].direction = 0.0f64;
        dfile[i as usize].most_common = 0;
        diag8[i as usize].pattern = i;
        diag8[i as usize].frequency = 0;
        diag8[i as usize].direction = 0.0f64;
        diag8[i as usize].most_common = 0;
        i += 1
    }
    i = 0;
    while i < 2187 as i32 {
        diag7[i as usize].pattern = i;
        diag7[i as usize].frequency = 0;
        diag7[i as usize].direction = 0.0f64;
        diag7[i as usize].most_common = 0;
        i += 1
    }
    i = 0;
    while i < 729 as i32 {
        diag6[i as usize].pattern = i;
        diag6[i as usize].frequency = 0;
        diag6[i as usize].direction = 0.0f64;
        diag6[i as usize].most_common = 0;
        i += 1
    }
    i = 0;
    while i < 243 as i32 {
        diag5[i as usize].pattern = i;
        diag5[i as usize].frequency = 0;
        diag5[i as usize].direction = 0.0f64;
        diag5[i as usize].most_common = 0;
        i += 1
    }
    i = 0;
    while i < 81 as i32 {
        diag4[i as usize].pattern = i;
        diag4[i as usize].frequency = 0;
        diag4[i as usize].direction = 0.0f64;
        diag4[i as usize].most_common = 0;
        i += 1
    };
}
/*
   SAVE
   Writes a set of pattern values to disc.
*/

pub unsafe  fn save(mut base: *const i8,
                              mut suffix: *mut i8,
                              mut items: *mut InfoItem,
                              mut count: i32) {
    let mut file_name: [i8; 32] = [0; 32];
    let mut vals: [f32; 59049] = [0.; 59049];
    let mut i: i32 = 0;
    let mut freq: [i32; 59049] = [0; 59049];
    let mut stream: *mut FILE = 0 as *mut FILE;
    sprintf(file_name.as_mut_ptr(),
            b"%s%s\x00" as *const u8 as *const i8, base, suffix);
    stream =
        fopen(file_name.as_mut_ptr(),
              b"wb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        printf(b"Error creating \'%s\'\n\x00" as *const u8 as
                   *const i8, file_name.as_mut_ptr());
    } else {
        i = 0;
        while i < count {
            vals[i as usize] =
                (*items.offset(i as isize)).solution as f32;
            freq[i as usize] = (*items.offset(i as isize)).frequency;
            i += 1
        }
        fwrite(vals.as_mut_ptr() as *const ::std::ffi::c_void,
               ::std::mem::size_of::<f32>() as u64,
               count as size_t, stream);
        fwrite(freq.as_mut_ptr() as *const ::std::ffi::c_void,
               ::std::mem::size_of::<i32>() as u64,
               count as size_t, stream);
        fclose(stream);
    };
}
/*
   STORE_PATTERNS
   Writes all sets of feature values to disc.
*/

pub unsafe  fn store_patterns() {
    let mut suffix: [i8; 8] = [0; 8];
    let mut file_name: [i8; 16] = [0; 16];
    let mut stream: *mut FILE = 0 as *mut FILE;
    write!(stdout, "Saving patterns...");
    fflush(stdout);
    sprintf(suffix.as_mut_ptr(),
            b".b%d\x00" as *const u8 as *const i8, analysis_stage);
    save(b"afile2x\x00" as *const u8 as *const i8,
         suffix.as_mut_ptr(), afile2x.as_mut_ptr(), 59049 as i32);
    save(b"bfile\x00" as *const u8 as *const i8,
         suffix.as_mut_ptr(), bfile.as_mut_ptr(), 6561 as i32);
    save(b"cfile\x00" as *const u8 as *const i8,
         suffix.as_mut_ptr(), cfile.as_mut_ptr(), 6561 as i32);
    save(b"dfile\x00" as *const u8 as *const i8,
         suffix.as_mut_ptr(), dfile.as_mut_ptr(), 6561 as i32);
    save(b"diag8\x00" as *const u8 as *const i8,
         suffix.as_mut_ptr(), diag8.as_mut_ptr(), 6561 as i32);
    save(b"diag7\x00" as *const u8 as *const i8,
         suffix.as_mut_ptr(), diag7.as_mut_ptr(), 2187 as i32);
    save(b"diag6\x00" as *const u8 as *const i8,
         suffix.as_mut_ptr(), diag6.as_mut_ptr(), 729 as i32);
    save(b"diag5\x00" as *const u8 as *const i8,
         suffix.as_mut_ptr(), diag5.as_mut_ptr(), 243 as i32);
    save(b"diag4\x00" as *const u8 as *const i8,
         suffix.as_mut_ptr(), diag4.as_mut_ptr(), 81 as i32);
    save(b"corner33\x00" as *const u8 as *const i8,
         suffix.as_mut_ptr(), corner33.as_mut_ptr(), 19683 as i32);
    save(b"corner52\x00" as *const u8 as *const i8,
         suffix.as_mut_ptr(), corner52.as_mut_ptr(), 59049 as i32);
    sprintf(file_name.as_mut_ptr(),
            b"main.s%d\x00" as *const u8 as *const i8,
            analysis_stage);
    stream =
        fopen(file_name.as_mut_ptr(),
              b"w\x00" as *const u8 as *const i8);
    if stream.is_null() {
        printf(b"Error creating \'%s\'\n\x00" as *const u8 as
                   *const i8, file_name.as_mut_ptr());
    } else {
        fprintf(stream, b"%.8f\n\x00" as *const u8 as *const i8,
                constant.solution);
        fprintf(stream, b"%.8f\n\x00" as *const u8 as *const i8,
                parity.solution);
        fprintf(stream, b"\n\x00" as *const u8 as *const i8);
        fprintf(stream,
                b"Target value: %.8f\n\x00" as *const u8 as
                    *const i8, objective);
        fprintf(stream,
                b"Average error: %.8f\n\x00" as *const u8 as
                    *const i8, abs_error_sum);
        fprintf(stream,
                b"Maximum change: %.8f\n\x00" as *const u8 as
                    *const i8, max_delta);
        fprintf(stream,
                b"Average change: %.8f\n\x00" as *const u8 as
                    *const i8, average_delta);
        fclose(stream);
    }
    puts(b" done\x00" as *const u8 as *const i8);
}
/*
   WRITE_LOG
   Saves info on the state of the optimization to disc.
*/

pub unsafe  fn write_log(mut iteration: i32) {
    let mut file_name: [i8; 32] = [0; 32];
    let mut stream: *mut FILE = 0 as *mut FILE;
    sprintf(file_name.as_mut_ptr(),
            b"log.s%d\x00" as *const u8 as *const i8,
            analysis_stage);
    stream =
        fopen(file_name.as_mut_ptr(),
              b"a\x00" as *const u8 as *const i8);
    if stream.is_null() {
        printf(b"Error appending to \'%s\'\n\x00" as *const u8 as
                   *const i8, file_name.as_mut_ptr());
    } else {
        fprintf(stream,
                b"#%3d  Obj=%.8f  Error=%.8f  Max=%.6f  Av=%.6f\n\x00" as
                    *const u8 as *const i8, iteration, objective,
                abs_error_sum, max_delta, average_delta);
        fclose(stream);
    };
}
/*
   INITIALIZE_SOLUTION
   Reads the starting point from disc if available, otherwise
   zeroes all values.
   Note: One-dimensional linear regression is no longer performed
   due to its poor performance.
*/

pub unsafe  fn initialize_solution(mut base: *const i8,
                                             mut item: *mut InfoItem,
                                             mut count: i32,
                                             _my_mirror:
                                             *mut i32) {
    let mut file_name: [i8; 32] = [0; 32];
    let mut vals: *mut f32 = 0 as *mut f32;
    let mut i: i32 = 0;
    let mut freq: *mut i32 = 0 as *mut i32;
    let mut stream: *mut FILE = 0 as *mut FILE;
    sprintf(file_name.as_mut_ptr(),
            b"%s.b%d\x00" as *const u8 as *const i8, base,
            analysis_stage);
    stream =
        fopen(file_name.as_mut_ptr(),
              b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        i = 0;
        while i < count {
            (*item.offset(i as isize)).solution = 0.0f64;
            i += 1
        }
    } else {
        vals =
            malloc((count as
                u64).wrapping_mul(::std::mem::size_of::<f32>()
                as u64)) as
                *mut f32;
        freq =
            malloc((count as
                u64).wrapping_mul(::std::mem::size_of::<i32>()
                as u64)) as
                *mut i32;
        fread(vals as *mut ::std::ffi::c_void,
              ::std::mem::size_of::<f32>() as u64,
              count as size_t, stream);
        fread(freq as *mut ::std::ffi::c_void,
              ::std::mem::size_of::<i32>() as u64,
              count as size_t, stream);
        fclose(stream);
        i = 0;
        while i < count {
            if *freq.offset(i as isize) > 0 as i32 {
                (*item.offset(i as isize)).solution =
                    *vals.offset(i as isize) as f64;
                if *vals.offset(i as isize) as f64 > 100.0f64 {
                    printf(b"i=%d, strange value %.2f, freq=%d\n\x00" as
                               *const u8 as *const i8, i,
                           *vals.offset(i as isize) as f64,
                           *freq.offset(i as isize));
                }
            } else { (*item.offset(i as isize)).solution = 0.0f64 }
            i += 1
        }
        free(freq as *mut ::std::ffi::c_void);
        free(vals as *mut ::std::ffi::c_void);
    };
}
/*
   FIND_MOST_COMMON
   Finds and marks the most common pattern of a feature.
*/

pub unsafe  fn find_most_common(mut item: *mut InfoItem,
                                          mut count: i32) {
    let mut i: i32 = 0;
    let mut index: i32 = 0;
    let mut value: i32 = 0;
    value = -(1 as i32);
    index = 0;
    i = 0;
    while i < count {
        if (*item.offset(i as isize)).frequency > value {
            index = i;
            value = (*item.offset(i as isize)).frequency
        }
        i += 1
    }
    (*item.offset(index as isize)).most_common = 1;
    (*item.offset(index as isize)).solution = 0.0f64;
}
/*
   INITIALIZE_NON_PATTERNS
   Reads or calculates the starting point for features not
   corresponding to patterns in the board.
*/

pub unsafe  fn initialize_non_patterns(mut base:
                                                 *const i8) {
    let mut file_name: [i8; 32] = [0; 32];
    let mut stream: *mut FILE = 0 as *mut FILE;
    sprintf(file_name.as_mut_ptr(),
            b"%s.s%d\x00" as *const u8 as *const i8, base,
            analysis_stage);
    stream =
        fopen(file_name.as_mut_ptr(),
              b"r\x00" as *const u8 as *const i8);
    if stream.is_null() {
        parity.solution = 0.0f64;
        constant.solution = 0.0f64
    } else {
        fscanf(stream, b"%lf\x00" as *const u8 as *const i8,
               &mut constant.solution as *mut f64);
        fscanf(stream, b"%lf\x00" as *const u8 as *const i8,
               &mut parity.solution as *mut f64);
        fclose(stream);
    };
}
/*
   LIMIT_CHANGE
   Change one feature value, but not more than the damping specifies.
*/

pub unsafe  fn limit_change(mut value: *mut f64,
                                      mut change: f32) {
    if change as f64 > 0.50f64 {
        change = 0.50f64 as f32
    } else if (change as f64) < -0.50f64 {
        change = -0.50f64 as f32
    }
    *value += change as f64;
}
/*
   UPDATE_SOLUTION
   Changes a specific set of pattern using a specified scale.
   Notice that pattern 0 is not updated; it is removed to
   obtain linear independence,
*/

pub unsafe  fn update_solution(mut item: *mut InfoItem,
                                         mut count: i32,
                                         mut scale: f64) {
    let mut change: f64 = 0.;
    let mut abs_change: f64 = 0.;
    let mut i: i32 = 0;
    i = 0;
    while i < count {
        if (*item.offset(i as isize)).frequency > 0 as i32 &&
            (*item.offset(i as isize)).most_common == 0 {
            change = scale * (*item.offset(i as isize)).direction;
            abs_change = fabs(change);
            if abs_change > max_delta { max_delta = abs_change }
            delta_sum += abs_change;
            if change > 0.50f64 {
                change = 0.50f64
            } else if change < -0.50f64 { change = -0.50f64 }
            limit_change(&mut (*item.offset(i as isize)).solution,
                         change as f32);
            update_count += 1
        }
        i += 1
    };
}
/*
   UPDATE_SEARCH_DIRECTION
   Update the search direction for a set of pattern using
   Fletcher-Reeves' update rule.
*/

pub unsafe  fn update_search_direction(mut item: *mut InfoItem,
                                                 mut count: i32,
                                                 mut beta: f64) {
    let mut i: i32 = 0;
    i = 0;
    while i < count {
        if (*item.offset(i as isize)).most_common == 0 {
            (*item.offset(i as isize)).direction =
                beta * (*item.offset(i as isize)).direction -
                    (*item.offset(i as isize)).gradient
        } else { (*item.offset(i as isize)).direction = 0.0f64 }
        i += 1
    };
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8)
                 -> i32 {
    let mut game_file: *mut i8 = 0 as *mut i8;
    let mut option_file: *mut i8 = 0 as *mut i8;
    let mut prefix: [i8; 32] = [0; 32];
    let mut alpha: f64 = 0.;
    let mut beta: f64 = 0.;
    let mut predicted_objective: f64 = 0.;
    let mut grad_sum: f64 = 0.;
    let mut old_grad_sum: f64 = 0.;
    let mut i: i32 = 0;
    let mut iteration: i32 = 0;
    let mut max_iterations: i32 = 0;
    let mut start_time: time_t = 0;
    let mut curr_time: time_t = 0;
    let mut option_stream: *mut FILE = 0 as *mut FILE;
    time(&mut start_time);
    if argc < 4 as i32 || argc > 7 as i32 {
        puts(b"Usage:\x00" as *const u8 as *const i8);
        puts(b"  tune8dbs <position file> <option file> <stage>\x00" as
            *const u8 as *const i8);
        puts(b"           [<max #positions>] [<iterations>] [<max diff>]\x00"
            as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        puts(b"Gunnar Andersson, July 19, 1999\x00" as *const u8 as
            *const i8);
        exit(1 as i32);
    }
    game_file = *argv.offset(1);
    option_file = *argv.offset(2);
    analysis_stage = atoi(*argv.offset(3));
    if argc >= 5 as i32 {
        max_positions = atoi(*argv.offset(4))
    } else { max_positions = 10000 as i32 }
    if argc >= 6 as i32 {
        max_iterations = atoi(*argv.offset(5))
    } else { max_iterations = 100000000 as i32 }
    if argc >= 7 as i32 {
        max_diff = atoi(*argv.offset(6))
    } else { max_diff = 64 as i32 }
    /* Create pattern tables and reset all feature values */
    printf(b"Building pattern tables... \x00" as *const u8 as
        *const i8);
    fflush(stdout);
    pattern_setup();
    puts(b"done\x00" as *const u8 as *const i8);
    /* Parse the option file */
    option_stream =
        fopen(option_file, b"r\x00" as *const u8 as *const i8);
    if option_stream.is_null() {
        printf(b"Unable to open option file \'%s\'\n\x00" as *const u8 as
                   *const i8, option_file);
        exit(1 as i32);
    }
    fscanf(option_stream, b"%s\x00" as *const u8 as *const i8,
           prefix.as_mut_ptr());
    fscanf(option_stream, b"%d\x00" as *const u8 as *const i8,
           &mut stage_count as *mut i32);
    i = 0;
    while i < stage_count {
        fscanf(option_stream, b"%d\x00" as *const u8 as *const i8,
               &mut *stage.as_mut_ptr().offset(i as isize) as
                   *mut i32);
        i += 1
    }
    fclose(option_stream);
    i = 0;
    while i <= 60 as i32 {
        active[i as usize] = 0;
        i += 1
    }
    if analysis_stage != 0 as i32 {
        i =
            stage[(analysis_stage - 1 as i32) as usize] +
                1 as i32;
        while i <= stage[analysis_stage as usize] {
            active[i as usize] = 1;
            weight[i as usize] =
                sqrt(1.0f64 *
                    (i -
                        stage[(analysis_stage - 1 as i32) as
                            usize]) as f64 /
                    (stage[analysis_stage as usize] -
                        stage[(analysis_stage - 1 as i32) as
                            usize]) as f64);
            i += 1
        }
    }
    if analysis_stage != stage_count - 1 as i32 {
        i = stage[analysis_stage as usize];
        while i < stage[(analysis_stage + 1 as i32) as usize] {
            active[i as usize] = 1;
            weight[i as usize] =
                sqrt(1.0f64 *
                    (stage[(analysis_stage + 1 as i32) as usize]
                        - i) as f64 /
                    (stage[(analysis_stage + 1 as i32) as usize]
                        - stage[analysis_stage as usize]) as
                        f64);
            i += 1
        }
    }
    i = 0;
    while i <= 60 as i32 {
        if active[i as usize] != 0 { last_active = i }
        i += 1
    }
    printf(b"Last active phase: %d\n\x00" as *const u8 as *const i8,
           last_active);
    /* Initialize the database */
    read_position_file(game_file);
    /* Determine pattern frequencies */
    write!(stdout, "\nPreparing...");
    fflush(stdout);
    analyze_games();
    printf(b" done (%d relevant nodes out of %d)\n\x00" as *const u8 as
               *const i8, relevant_count, node_count);
    interval =
        (relevant_count / 5 as i32 + 9 as i32) /
            10 as i32 * 10 as i32;
    printf(b"Reading pattern values... \x00" as *const u8 as
        *const i8);
    fflush(stdout);
    initialize_non_patterns(b"main\x00" as *const u8 as *const i8);
    initialize_solution(b"afile2x\x00" as *const u8 as *const i8,
                        afile2x.as_mut_ptr(), 59049 as i32,
                        mirror82x.as_mut_ptr());
    find_most_common(afile2x.as_mut_ptr(), 59049 as i32);
    initialize_solution(b"bfile\x00" as *const u8 as *const i8,
                        bfile.as_mut_ptr(), 6561 as i32,
                        mirror.as_mut_ptr());
    find_most_common(bfile.as_mut_ptr(), 6561 as i32);
    initialize_solution(b"cfile\x00" as *const u8 as *const i8,
                        cfile.as_mut_ptr(), 6561 as i32,
                        mirror.as_mut_ptr());
    find_most_common(cfile.as_mut_ptr(), 6561 as i32);
    initialize_solution(b"dfile\x00" as *const u8 as *const i8,
                        dfile.as_mut_ptr(), 6561 as i32,
                        mirror.as_mut_ptr());
    find_most_common(dfile.as_mut_ptr(), 6561 as i32);
    initialize_solution(b"diag8\x00" as *const u8 as *const i8,
                        diag8.as_mut_ptr(), 6561 as i32,
                        mirror.as_mut_ptr());
    find_most_common(diag8.as_mut_ptr(), 6561 as i32);
    initialize_solution(b"diag7\x00" as *const u8 as *const i8,
                        diag7.as_mut_ptr(), 2187 as i32,
                        mirror7.as_mut_ptr());
    find_most_common(diag7.as_mut_ptr(), 2187 as i32);
    initialize_solution(b"diag6\x00" as *const u8 as *const i8,
                        diag6.as_mut_ptr(), 729 as i32,
                        mirror6.as_mut_ptr());
    find_most_common(diag6.as_mut_ptr(), 729 as i32);
    initialize_solution(b"diag5\x00" as *const u8 as *const i8,
                        diag5.as_mut_ptr(), 243 as i32,
                        mirror5.as_mut_ptr());
    find_most_common(diag5.as_mut_ptr(), 243 as i32);
    initialize_solution(b"diag4\x00" as *const u8 as *const i8,
                        diag4.as_mut_ptr(), 81 as i32,
                        mirror4.as_mut_ptr());
    find_most_common(diag4.as_mut_ptr(), 81 as i32);
    initialize_solution(b"corner33\x00" as *const u8 as *const i8,
                        corner33.as_mut_ptr(), 19683 as i32,
                        mirror33.as_mut_ptr());
    find_most_common(corner33.as_mut_ptr(), 19683 as i32);
    initialize_solution(b"corner52\x00" as *const u8 as *const i8,
                        corner52.as_mut_ptr(), 59049 as i32,
                        identity10.as_mut_ptr());
    find_most_common(corner52.as_mut_ptr(), 59049 as i32);
    puts(b"done\x00" as *const u8 as *const i8);
    /* Scan through the database and generate the data points */
    grad_sum = 0.0f64;
    max_delta = 0.0f64;
    average_delta = 0.0f64;
    iteration = 1;
    while iteration <= max_iterations {
        constant.gradient = 0.0f64;
        parity.gradient = 0.0f64;
        i = 0;
        while i < 59059 as i32 {
            afile2x[i as usize].gradient = 0.0f64;
            i += 1
        }
        i = 0;
        while i < 59049 as i32 {
            corner52[i as usize].gradient = 0.0f64;
            i += 1
        }
        i = 0;
        while i < 19683 as i32 {
            corner33[i as usize].gradient = 0.0f64;
            i += 1
        }
        i = 0;
        while i < 6561 as i32 {
            bfile[i as usize].gradient = 0.0f64;
            cfile[i as usize].gradient = 0.0f64;
            dfile[i as usize].gradient = 0.0f64;
            diag8[i as usize].gradient = 0.0f64;
            i += 1
        }
        i = 0;
        while i < 2187 as i32 {
            diag7[i as usize].gradient = 0.0f64;
            i += 1
        }
        i = 0;
        while i < 729 as i32 {
            diag6[i as usize].gradient = 0.0f64;
            i += 1
        }
        i = 0;
        while i < 243 as i32 {
            diag5[i as usize].gradient = 0.0f64;
            i += 1
        }
        i = 0;
        while i < 81 as i32 {
            diag4[i as usize].gradient = 0.0f64;
            i += 1
        }
        objective = 0.0f64;
        abs_error_sum = 0.0f64;
        printf(b"\nDetermining gradient:      \x00" as *const u8 as
            *const i8);
        fflush(stdout);
        total_weight = 0.0f64;
        evaluate_games();
        printf(b" %d\n\x00" as *const u8 as *const i8,
               relevant_count);
        objective /= total_weight;
        abs_error_sum /= total_weight;
        store_patterns();
        time(&mut curr_time);
        printf(b"Objective: %.8f    Av. error: %.8f    Time: %ld s    Iter %d\n\x00"
                   as *const u8 as *const i8, objective,
               abs_error_sum, curr_time - start_time, iteration);
        /* Measure the gradient */
        printf(b"Updating the gradient length... \x00" as *const u8 as
            *const i8);
        fflush(stdout);
        old_grad_sum = grad_sum;
        grad_sum = 0.0f64;
        grad_sum += constant.gradient * constant.gradient;
        grad_sum += parity.gradient * parity.gradient;
        i = 0;
        while i < 59049 as i32 {
            if afile2x[i as usize].most_common == 0 {
                grad_sum +=
                    afile2x[i as usize].gradient *
                        afile2x[i as usize].gradient
            }
            i += 1
        }
        i = 0;
        while i < 59049 as i32 {
            if corner52[i as usize].most_common == 0 {
                grad_sum +=
                    corner52[i as usize].gradient *
                        corner52[i as usize].gradient
            }
            i += 1
        }
        i = 0;
        while i < 19683 as i32 {
            if corner33[i as usize].most_common == 0 {
                grad_sum +=
                    corner33[i as usize].gradient *
                        corner33[i as usize].gradient
            }
            i += 1
        }
        i = 0;
        while i < 6561 as i32 {
            if bfile[i as usize].most_common == 0 {
                grad_sum +=
                    bfile[i as usize].gradient * bfile[i as usize].gradient
            }
            if cfile[i as usize].most_common == 0 {
                grad_sum +=
                    cfile[i as usize].gradient * cfile[i as usize].gradient
            }
            if dfile[i as usize].most_common == 0 {
                grad_sum +=
                    dfile[i as usize].gradient * dfile[i as usize].gradient
            }
            if diag8[i as usize].most_common == 0 {
                grad_sum +=
                    diag8[i as usize].gradient * diag8[i as usize].gradient
            }
            i += 1
        }
        i = 0;
        while i < 2187 as i32 {
            if diag7[i as usize].most_common == 0 {
                grad_sum +=
                    diag7[i as usize].gradient * diag7[i as usize].gradient
            }
            i += 1
        }
        i = 0;
        while i < 729 as i32 {
            if diag6[i as usize].most_common == 0 {
                grad_sum +=
                    diag6[i as usize].gradient * diag6[i as usize].gradient
            }
            i += 1
        }
        i = 0;
        while i < 243 as i32 {
            if diag5[i as usize].most_common == 0 {
                grad_sum +=
                    diag5[i as usize].gradient * diag5[i as usize].gradient
            }
            i += 1
        }
        i = 0;
        while i < 81 as i32 {
            if diag4[i as usize].most_common == 0 {
                grad_sum +=
                    diag4[i as usize].gradient * diag4[i as usize].gradient
            }
            i += 1
        }
        /* Determine the current search direction */
        if iteration > 1 as i32 {
            beta = grad_sum / old_grad_sum
        } else { beta = 0.0f64 }
        printf(b"beta=%.8f\n\x00" as *const u8 as *const i8, beta);
        constant.direction = beta * constant.direction - constant.gradient;
        parity.direction = beta * parity.direction - parity.gradient;
        update_search_direction(afile2x.as_mut_ptr(), 59049 as i32,
                                beta);
        update_search_direction(bfile.as_mut_ptr(), 6561 as i32,
                                beta);
        update_search_direction(cfile.as_mut_ptr(), 6561 as i32,
                                beta);
        update_search_direction(dfile.as_mut_ptr(), 6561 as i32,
                                beta);
        update_search_direction(diag8.as_mut_ptr(), 6561 as i32,
                                beta);
        update_search_direction(diag7.as_mut_ptr(), 2187 as i32,
                                beta);
        update_search_direction(diag6.as_mut_ptr(), 729 as i32, beta);
        update_search_direction(diag5.as_mut_ptr(), 243 as i32, beta);
        update_search_direction(diag4.as_mut_ptr(), 81 as i32, beta);
        update_search_direction(corner33.as_mut_ptr(), 19683 as i32,
                                beta);
        update_search_direction(corner52.as_mut_ptr(), 59049 as i32,
                                beta);
        /* Find the best one-dimensional step */
        printf(b"Determining step:          \x00" as *const u8 as
            *const i8);
        fflush(stdout);
        quad_coeff = 0.0f64;
        lin_coeff = 0.0f64;
        const_coeff = 0.0f64;
        determine_games();
        printf(b" %d\n\x00" as *const u8 as *const i8,
               relevant_count);
        quad_coeff /= total_weight;
        lin_coeff /= total_weight;
        const_coeff /= total_weight;
        alpha = -lin_coeff / (2.0f64 * quad_coeff);
        predicted_objective = const_coeff - quad_coeff * alpha * alpha;
        printf(b"alpha=%.8f predicts %.8f\n\x00" as *const u8 as
                   *const i8, alpha, predicted_objective);
        /* Update the solution */
        max_delta = 0.0f64;
        delta_sum = 0.0f64;
        update_count = 0;
        limit_change(&mut constant.solution,
                     (alpha * constant.direction / total_weight) as
                         f32);
        limit_change(&mut parity.solution,
                     (alpha * parity.direction / total_weight) as
                         f32);
        update_solution(afile2x.as_mut_ptr(), 59049 as i32,
                        alpha / total_weight);
        update_solution(bfile.as_mut_ptr(), 6561 as i32,
                        alpha / total_weight);
        update_solution(cfile.as_mut_ptr(), 6561 as i32,
                        alpha / total_weight);
        update_solution(dfile.as_mut_ptr(), 6561 as i32,
                        alpha / total_weight);
        update_solution(diag8.as_mut_ptr(), 6561 as i32,
                        alpha / total_weight);
        update_solution(diag7.as_mut_ptr(), 2187 as i32,
                        alpha / total_weight);
        update_solution(diag6.as_mut_ptr(), 729 as i32,
                        alpha / total_weight);
        update_solution(diag5.as_mut_ptr(), 243 as i32,
                        alpha / total_weight);
        update_solution(diag4.as_mut_ptr(), 81 as i32,
                        alpha / total_weight);
        update_solution(corner33.as_mut_ptr(), 19683 as i32,
                        alpha / total_weight);
        update_solution(corner52.as_mut_ptr(), 59049 as i32,
                        alpha / total_weight);
        average_delta = delta_sum / update_count as f64;
        printf(b"Constant: %.4f  Parity: %.4f  Max change: %.5f  Av. change: %.5f\n\x00"
                   as *const u8 as *const i8, constant.solution,
               parity.solution, max_delta, average_delta);
        if iteration % 10 as i32 == 0 as i32 {
            write_log(iteration);
        }
        iteration += 1
    }
    return 0 as i32;
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as i32,
                                    args.as_mut_ptr() as
                                        *mut *mut i8) as i32)
    }
}
