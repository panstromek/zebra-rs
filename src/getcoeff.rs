use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn gzgetc(file: gzFile) -> libc::c_int;
    #[no_mangle]
    fn gzclose(file: gzFile) -> libc::c_int;
    #[no_mangle]
    fn gzopen(_: *const libc::c_char, _: *const libc::c_char) -> gzFile;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /*
   File:       error.h

   Created:    June 13, 1998

   Modified:   August 1, 2002

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The interface to the error handler.
*/
    #[no_mangle]
    fn fatal_error(format: *const libc::c_char, _: ...);
    /* Holds the current board position. Updated as the search progresses,
   but all updates must be reversed when the search stops. */
    #[no_mangle]
    static mut board: Board;
    #[no_mangle]
    static mut piece_count: [[libc::c_int; 64]; 3];
    /*
   File:           moves.h

   Created:        June 30, 1997

   Modified:       August 1, 2002

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       The move generator's interface.
*/
    /* The number of disks played from the initial position.
   Must match the current status of the BOARD variable. */
    #[no_mangle]
    static mut disks_played: libc::c_int;
    /*
   File:           patterns.h

   Created:        July 4, 1997

   Modified:       August 1, 2002

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       The patterns.
*/
    /* Predefined two-bit patterns. */
    /* Board patterns used in position evaluation */
    #[no_mangle]
    static mut pow3: [libc::c_int; 10];
    /* Symmetry maps */
    #[no_mangle]
    static mut flip8: [libc::c_int; 6561];
    /*
   File:       safemem.h

   Created:    August 30, 1998

   Modified:   January 25, 2000

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The interface to the safer version of malloc.
*/
    #[no_mangle]
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: libc::c_uint,
    pub next: *mut libc::c_uchar,
    pub pos: off_t,
}
pub type gzFile = *mut gzFile_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type Board = [libc::c_int; 128];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoeffSet {
    pub permanent: libc::c_int,
    pub loaded: libc::c_int,
    pub prev: libc::c_int,
    pub next: libc::c_int,
    pub block: libc::c_int,
    pub parity_constant: [libc::c_short; 2],
    pub parity: libc::c_short,
    pub constant: libc::c_short,
    pub afile2x: *mut libc::c_short,
    pub bfile: *mut libc::c_short,
    pub cfile: *mut libc::c_short,
    pub dfile: *mut libc::c_short,
    pub diag8: *mut libc::c_short,
    pub diag7: *mut libc::c_short,
    pub diag6: *mut libc::c_short,
    pub diag5: *mut libc::c_short,
    pub diag4: *mut libc::c_short,
    pub corner33: *mut libc::c_short,
    pub corner52: *mut libc::c_short,
    pub afile2x_last: *mut libc::c_short,
    pub bfile_last: *mut libc::c_short,
    pub cfile_last: *mut libc::c_short,
    pub dfile_last: *mut libc::c_short,
    pub diag8_last: *mut libc::c_short,
    pub diag7_last: *mut libc::c_short,
    pub diag6_last: *mut libc::c_short,
    pub diag5_last: *mut libc::c_short,
    pub diag4_last: *mut libc::c_short,
    pub corner33_last: *mut libc::c_short,
    pub corner52_last: *mut libc::c_short,
    pub alignment_padding: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AllocationBlock {
    pub afile2x_block: [libc::c_short; 59049],
    pub bfile_block: [libc::c_short; 6561],
    pub cfile_block: [libc::c_short; 6561],
    pub dfile_block: [libc::c_short; 6561],
    pub diag8_block: [libc::c_short; 6561],
    pub diag7_block: [libc::c_short; 2187],
    pub diag6_block: [libc::c_short; 729],
    pub diag5_block: [libc::c_short; 243],
    pub diag4_block: [libc::c_short; 81],
    pub corner33_block: [libc::c_short; 19683],
    pub corner52_block: [libc::c_short; 59049],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub signed_val: libc::c_short,
    pub unsigned_val: libc::c_ushort,
}
static mut stage_count: libc::c_int = 0;
static mut block_count: libc::c_int = 0;
static mut stage: [libc::c_int; 61] = [0; 61];
static mut block_allocated: [libc::c_int; 200] = [0; 200];
static mut block_set: [libc::c_int; 200] = [0; 200];
static mut eval_map: [libc::c_int; 61] = [0; 61];
static mut block_list: [*mut AllocationBlock; 200] =
    [0 as *const AllocationBlock as *mut AllocationBlock; 200];
static mut set: [CoeffSet; 61] =
    [CoeffSet{permanent: 0,
              loaded: 0,
              prev: 0,
              next: 0,
              block: 0,
              parity_constant: [0; 2],
              parity: 0,
              constant: 0,
              afile2x: 0 as *const libc::c_short as *mut libc::c_short,
              bfile: 0 as *const libc::c_short as *mut libc::c_short,
              cfile: 0 as *const libc::c_short as *mut libc::c_short,
              dfile: 0 as *const libc::c_short as *mut libc::c_short,
              diag8: 0 as *const libc::c_short as *mut libc::c_short,
              diag7: 0 as *const libc::c_short as *mut libc::c_short,
              diag6: 0 as *const libc::c_short as *mut libc::c_short,
              diag5: 0 as *const libc::c_short as *mut libc::c_short,
              diag4: 0 as *const libc::c_short as *mut libc::c_short,
              corner33: 0 as *const libc::c_short as *mut libc::c_short,
              corner52: 0 as *const libc::c_short as *mut libc::c_short,
              afile2x_last: 0 as *const libc::c_short as *mut libc::c_short,
              bfile_last: 0 as *const libc::c_short as *mut libc::c_short,
              cfile_last: 0 as *const libc::c_short as *mut libc::c_short,
              dfile_last: 0 as *const libc::c_short as *mut libc::c_short,
              diag8_last: 0 as *const libc::c_short as *mut libc::c_short,
              diag7_last: 0 as *const libc::c_short as *mut libc::c_short,
              diag6_last: 0 as *const libc::c_short as *mut libc::c_short,
              diag5_last: 0 as *const libc::c_short as *mut libc::c_short,
              diag4_last: 0 as *const libc::c_short as *mut libc::c_short,
              corner33_last: 0 as *const libc::c_short as *mut libc::c_short,
              corner52_last: 0 as *const libc::c_short as *mut libc::c_short,
              alignment_padding: [0; 12],}; 61];
/*
   TERMINAL_PATTERNS
   Calculates the patterns associated with a filled board,
   only counting discs.
*/
unsafe extern "C" fn terminal_patterns() {
    let mut result: libc::c_double = 0.;
    let mut value: [[libc::c_double; 8]; 8] = [[0.; 8]; 8];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut row: [libc::c_int; 10] = [0; 10];
    let mut hit: [[libc::c_int; 8]; 8] = [[0; 8]; 8];
    /* Count the number of times each square is counted */
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            hit[i as usize][j as usize] = 0 as libc::c_int;
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        hit[0 as libc::c_int as usize][i as usize] += 1;
        hit[i as usize][0 as libc::c_int as usize] += 1;
        hit[7 as libc::c_int as usize][i as usize] += 1;
        hit[i as usize][7 as libc::c_int as usize] += 1;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        hit[1 as libc::c_int as usize][i as usize] += 1;
        hit[i as usize][1 as libc::c_int as usize] += 1;
        hit[6 as libc::c_int as usize][i as usize] += 1;
        hit[i as usize][6 as libc::c_int as usize] += 1;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        hit[2 as libc::c_int as usize][i as usize] += 1;
        hit[i as usize][2 as libc::c_int as usize] += 1;
        hit[5 as libc::c_int as usize][i as usize] += 1;
        hit[i as usize][5 as libc::c_int as usize] += 1;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        hit[3 as libc::c_int as usize][i as usize] += 1;
        hit[i as usize][3 as libc::c_int as usize] += 1;
        hit[4 as libc::c_int as usize][i as usize] += 1;
        hit[i as usize][4 as libc::c_int as usize] += 1;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            hit[i as usize][j as usize] += 1;
            hit[i as usize][(7 as libc::c_int - j) as usize] += 1;
            hit[(7 as libc::c_int - i) as usize][j as usize] += 1;
            hit[(7 as libc::c_int - i) as
                    usize][(7 as libc::c_int - j) as usize] += 1;
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            hit[i as usize][j as usize] += 1;
            hit[j as usize][i as usize] += 1;
            hit[i as usize][(7 as libc::c_int - j) as usize] += 1;
            hit[j as usize][(7 as libc::c_int - i) as usize] += 1;
            hit[(7 as libc::c_int - i) as usize][j as usize] += 1;
            hit[(7 as libc::c_int - j) as usize][i as usize] += 1;
            hit[(7 as libc::c_int - i) as
                    usize][(7 as libc::c_int - j) as usize] += 1;
            hit[(7 as libc::c_int - j) as
                    usize][(7 as libc::c_int - i) as usize] += 1;
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        hit[i as usize][i as usize] += 1;
        hit[i as usize][(7 as libc::c_int - i) as usize] += 1;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        hit[i as usize][(i + 1 as libc::c_int) as usize] += 1;
        hit[(i + 1 as libc::c_int) as usize][i as usize] += 1;
        hit[i as usize][(6 as libc::c_int - i) as usize] += 1;
        hit[(i + 1 as libc::c_int) as usize][(7 as libc::c_int - i) as usize]
            += 1;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        hit[i as usize][(i + 2 as libc::c_int) as usize] += 1;
        hit[(i + 2 as libc::c_int) as usize][i as usize] += 1;
        hit[i as usize][(5 as libc::c_int - i) as usize] += 1;
        hit[(i + 2 as libc::c_int) as usize][(7 as libc::c_int - i) as usize]
            += 1;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        hit[i as usize][(i + 3 as libc::c_int) as usize] += 1;
        hit[(i + 3 as libc::c_int) as usize][i as usize] += 1;
        hit[i as usize][(4 as libc::c_int - i) as usize] += 1;
        hit[(i + 3 as libc::c_int) as usize][(7 as libc::c_int - i) as usize]
            += 1;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        hit[i as usize][(i + 4 as libc::c_int) as usize] += 1;
        hit[(i + 4 as libc::c_int) as usize][i as usize] += 1;
        hit[i as usize][(3 as libc::c_int - i) as usize] += 1;
        hit[(i + 4 as libc::c_int) as usize][(7 as libc::c_int - i) as usize]
            += 1;
        i += 1
    }
    hit[1 as libc::c_int as usize][1 as libc::c_int as usize] +=
        2 as libc::c_int;
    hit[1 as libc::c_int as usize][6 as libc::c_int as usize] +=
        2 as libc::c_int;
    hit[6 as libc::c_int as usize][1 as libc::c_int as usize] +=
        2 as libc::c_int;
    hit[6 as libc::c_int as usize][6 as libc::c_int as usize] +=
        2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            value[i as usize][j as usize] =
                1.0f64 / hit[i as usize][j as usize] as libc::c_double;
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int { row[i as usize] = 0 as libc::c_int; i += 1 }
    i = 0 as libc::c_int;
    while i < 59049 as libc::c_int {
        result = 0.0f64;
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            if row[j as usize] == 0 as libc::c_int {
                result += value[0 as libc::c_int as usize][j as usize]
            } else if row[j as usize] == 2 as libc::c_int {
                result -= value[0 as libc::c_int as usize][j as usize]
            }
            j += 1
        }
        if row[8 as libc::c_int as usize] == 0 as libc::c_int {
            result +=
                value[1 as libc::c_int as usize][1 as libc::c_int as usize]
        } else if row[8 as libc::c_int as usize] == 2 as libc::c_int {
            result -=
                value[1 as libc::c_int as usize][1 as libc::c_int as usize]
        }
        if row[9 as libc::c_int as usize] == 0 as libc::c_int {
            result +=
                value[1 as libc::c_int as usize][6 as libc::c_int as usize]
        } else if row[9 as libc::c_int as usize] == 2 as libc::c_int {
            result -=
                value[1 as libc::c_int as usize][6 as libc::c_int as usize]
        }
        *set[60 as libc::c_int as usize].afile2x.offset(i as isize) =
            floor(result * 128.0f64 + 0.5f64) as libc::c_short;
        result = 0.0f64;
        j = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            k = 0 as libc::c_int;
            while k < 2 as libc::c_int {
                if row[(5 as libc::c_int * k + j) as usize] ==
                       0 as libc::c_int {
                    result += value[j as usize][k as usize]
                } else if row[(5 as libc::c_int * k + j) as usize] ==
                              2 as libc::c_int {
                    result -= value[j as usize][k as usize]
                }
                k += 1
            }
            j += 1
        }
        *set[60 as libc::c_int as usize].corner52.offset(i as isize) =
            floor(result * 128.0f64 + 0.5f64) as libc::c_short;
        if i < 19683 as libc::c_int {
            result = 0.0f64;
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                k = 0 as libc::c_int;
                while k < 3 as libc::c_int {
                    if row[(3 as libc::c_int * j + k) as usize] ==
                           0 as libc::c_int {
                        result += value[j as usize][k as usize]
                    } else if row[(3 as libc::c_int * j + k) as usize] ==
                                  2 as libc::c_int {
                        result -= value[j as usize][k as usize]
                    }
                    k += 1
                }
                j += 1
            }
            *set[60 as libc::c_int as usize].corner33.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as libc::c_short
        }
        if i < 6561 as libc::c_int {
            result = 0.0f64;
            j = 0 as libc::c_int;
            while j < 8 as libc::c_int {
                if row[j as usize] == 0 as libc::c_int {
                    result += value[1 as libc::c_int as usize][j as usize]
                } else if row[j as usize] == 2 as libc::c_int {
                    result -= value[1 as libc::c_int as usize][j as usize]
                }
                j += 1
            }
            *set[60 as libc::c_int as usize].bfile.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as libc::c_short;
            result = 0.0f64;
            j = 0 as libc::c_int;
            while j < 8 as libc::c_int {
                if row[j as usize] == 0 as libc::c_int {
                    result += value[2 as libc::c_int as usize][j as usize]
                } else if row[j as usize] == 2 as libc::c_int {
                    result -= value[2 as libc::c_int as usize][j as usize]
                }
                j += 1
            }
            *set[60 as libc::c_int as usize].cfile.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as libc::c_short;
            result = 0.0f64;
            j = 0 as libc::c_int;
            while j < 8 as libc::c_int {
                if row[j as usize] == 0 as libc::c_int {
                    result += value[3 as libc::c_int as usize][j as usize]
                } else if row[j as usize] == 2 as libc::c_int {
                    result -= value[3 as libc::c_int as usize][j as usize]
                }
                j += 1
            }
            *set[60 as libc::c_int as usize].dfile.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as libc::c_short;
            result = 0.0f64;
            j = 0 as libc::c_int;
            while j < 8 as libc::c_int {
                if row[j as usize] == 0 as libc::c_int {
                    result += value[j as usize][j as usize]
                } else if row[j as usize] == 2 as libc::c_int {
                    result -= value[j as usize][j as usize]
                }
                j += 1
            }
            *set[60 as libc::c_int as usize].diag8.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as libc::c_short
        }
        if i < 2187 as libc::c_int {
            result = 0.0f64;
            j = 0 as libc::c_int;
            while j < 7 as libc::c_int {
                if row[j as usize] == 0 as libc::c_int {
                    result +=
                        value[j as usize][(j + 1 as libc::c_int) as usize]
                } else if row[j as usize] == 2 as libc::c_int {
                    result -=
                        value[j as usize][(j + 1 as libc::c_int) as usize]
                }
                j += 1
            }
            *set[60 as libc::c_int as usize].diag7.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as libc::c_short
        }
        if i < 729 as libc::c_int {
            result = 0.0f64;
            j = 0 as libc::c_int;
            while j < 6 as libc::c_int {
                if row[j as usize] == 0 as libc::c_int {
                    result +=
                        value[j as usize][(j + 2 as libc::c_int) as usize]
                } else if row[j as usize] == 2 as libc::c_int {
                    result -=
                        value[j as usize][(j + 2 as libc::c_int) as usize]
                }
                j += 1
            }
            *set[60 as libc::c_int as usize].diag6.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as libc::c_short
        }
        if i < 243 as libc::c_int {
            result = 0.0f64;
            j = 0 as libc::c_int;
            while j < 5 as libc::c_int {
                if row[j as usize] == 0 as libc::c_int {
                    result +=
                        value[j as usize][(j + 3 as libc::c_int) as usize]
                } else if row[j as usize] == 2 as libc::c_int {
                    result -=
                        value[j as usize][(j + 3 as libc::c_int) as usize]
                }
                j += 1
            }
            *set[60 as libc::c_int as usize].diag5.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as libc::c_short
        }
        if i < 81 as libc::c_int {
            result = 0.0f64;
            j = 0 as libc::c_int;
            while j < 4 as libc::c_int {
                if row[j as usize] == 0 as libc::c_int {
                    result +=
                        value[j as usize][(j + 4 as libc::c_int) as usize]
                } else if row[j as usize] == 2 as libc::c_int {
                    result -=
                        value[j as usize][(j + 4 as libc::c_int) as usize]
                }
                j += 1
            }
            *set[60 as libc::c_int as usize].diag4.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as libc::c_short
        }
        /* Next configuration */
        j = 0 as libc::c_int;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as libc::c_int {
                row[j as usize] = 0 as libc::c_int
            }
            j += 1;
            if !(row[(j - 1 as libc::c_int) as usize] == 0 as libc::c_int &&
                     j < 10 as libc::c_int) {
                break ;
            }
        }
        i += 1
    };
}
/*
   GET_WORD
   Reads a 16-bit signed integer from a file.
*/
unsafe extern "C" fn get_word(mut stream: gzFile) -> libc::c_short {
    let mut val = C2RustUnnamed{signed_val: 0,};
    let mut hi: libc::c_int = 0;
    let mut lo: libc::c_int = 0;
    hi =
        if (*stream).have != 0 {
            (*stream).have = (*stream).have.wrapping_sub(1);
            (*stream).pos += 1;
            let fresh0 = (*stream).next;
            (*stream).next = (*stream).next.offset(1);
            *fresh0 as libc::c_int
        } else { gzgetc(stream) };
    if hi != -(1 as libc::c_int) {
    } else {
        __assert_fail(b"hi != -1\x00" as *const u8 as *const libc::c_char,
                      b"getcoeff.c\x00" as *const u8 as *const libc::c_char,
                      339 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"short get_word(gzFile)\x00")).as_ptr());
    }
    lo =
        if (*stream).have != 0 {
            (*stream).have = (*stream).have.wrapping_sub(1);
            (*stream).pos += 1;
            let fresh1 = (*stream).next;
            (*stream).next = (*stream).next.offset(1);
            *fresh1 as libc::c_int
        } else { gzgetc(stream) };
    if lo != -(1 as libc::c_int) {
    } else {
        __assert_fail(b"lo != -1\x00" as *const u8 as *const libc::c_char,
                      b"getcoeff.c\x00" as *const u8 as *const libc::c_char,
                      342 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"short get_word(gzFile)\x00")).as_ptr());
    }
    val.unsigned_val = ((hi << 8 as libc::c_int) + lo) as libc::c_ushort;
    return val.signed_val;
}
/*
   UNPACK_BATCH
   Reads feature values for one specific pattern
*/
unsafe extern "C" fn unpack_batch(mut item: *mut libc::c_short,
                                  mut mirror: *mut libc::c_int,
                                  mut count: libc::c_int,
                                  mut stream: gzFile) {
    let mut i: libc::c_int = 0;
    let mut buffer = 0 as *mut libc::c_short;
    buffer =
        safe_malloc((count as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                         as libc::c_ulong)) as
            *mut libc::c_short;
    /* Unpack the coefficient block where the score is scaled
       so that 512 units corresponds to one disk. */
    i = 0 as libc::c_int;
    while i < count {
        if mirror.is_null() || *mirror.offset(i as isize) == i {
            *buffer.offset(i as isize) =
                (get_word(stream) as libc::c_int / 4 as libc::c_int) as
                    libc::c_short
        } else {
            *buffer.offset(i as isize) =
                *buffer.offset(*mirror.offset(i as isize) as isize)
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < count {
        *item.offset(i as isize) = *buffer.offset(i as isize);
        i += 1
    }
    if !mirror.is_null() {
        i = 0 as libc::c_int;
        while i < count {
            if *item.offset(i as isize) as libc::c_int !=
                   *item.offset(*mirror.offset(i as isize) as isize) as
                       libc::c_int {
                printf(b"%s @ %d <--> %d of %d\n\x00" as *const u8 as
                           *const libc::c_char,
                       b"Mirror symmetry error\x00" as *const u8 as
                           *const libc::c_char, i, *mirror.offset(i as isize),
                       count);
                printf(b"%d <--> %d\n\x00" as *const u8 as
                           *const libc::c_char,
                       *item.offset(i as isize) as libc::c_int,
                       *item.offset(*mirror.offset(i as isize) as isize) as
                           libc::c_int);
                exit(1 as libc::c_int);
            }
            i += 1
        }
    }
    free(buffer as *mut libc::c_void);
}
/*
   UNPACK_COEFFS
   Reads all feature values for a certain stage. To take care of
   symmetric patterns, mirror tables are calculated.
*/
unsafe extern "C" fn unpack_coeffs(mut stream: gzFile) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut mirror_pattern: libc::c_int = 0;
    let mut row: [libc::c_int; 10] = [0; 10];
    let mut map_mirror3 = 0 as *mut libc::c_int;
    let mut map_mirror4 = 0 as *mut libc::c_int;
    let mut map_mirror5 = 0 as *mut libc::c_int;
    let mut map_mirror6 = 0 as *mut libc::c_int;
    let mut map_mirror7 = 0 as *mut libc::c_int;
    let mut map_mirror8 = 0 as *mut libc::c_int;
    let mut map_mirror33 = 0 as *mut libc::c_int;
    let mut map_mirror8x2 = 0 as *mut libc::c_int;
    /* Allocate the memory needed for the temporary mirror maps from the
       heap rather than the stack to reduce memory requirements. */
    map_mirror3 =
        safe_malloc((27 as libc::c_int as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                         as libc::c_ulong)) as
            *mut libc::c_int;
    map_mirror4 =
        safe_malloc((81 as libc::c_int as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                         as libc::c_ulong)) as
            *mut libc::c_int;
    map_mirror5 =
        safe_malloc((243 as libc::c_int as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                         as libc::c_ulong)) as
            *mut libc::c_int;
    map_mirror6 =
        safe_malloc((729 as libc::c_int as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                         as libc::c_ulong)) as
            *mut libc::c_int;
    map_mirror7 =
        safe_malloc((2187 as libc::c_int as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                         as libc::c_ulong)) as
            *mut libc::c_int;
    map_mirror8 =
        safe_malloc((6561 as libc::c_int as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                         as libc::c_ulong)) as
            *mut libc::c_int;
    map_mirror33 =
        safe_malloc((19683 as libc::c_int as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                         as libc::c_ulong)) as
            *mut libc::c_int;
    map_mirror8x2 =
        safe_malloc((59049 as libc::c_int as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                         as libc::c_ulong)) as
            *mut libc::c_int;
    /* Build the pattern tables for 8*1-patterns */
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int { row[i as usize] = 0 as libc::c_int; i += 1 }
    i = 0 as libc::c_int;
    while i < 6561 as libc::c_int {
        mirror_pattern = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            mirror_pattern +=
                row[j as usize] * pow3[(7 as libc::c_int - j) as usize];
            j += 1
        }
        /* Create the symmetry map */
        *map_mirror8.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0 as libc::c_int;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as libc::c_int {
                row[j as usize] = 0 as libc::c_int
            }
            j += 1;
            if !(row[(j - 1 as libc::c_int) as usize] == 0 as libc::c_int &&
                     j < 8 as libc::c_int) {
                break ;
            }
        }
        i += 1
    }
    /* Build the tables for 7*1-patterns */
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int { row[i as usize] = 0 as libc::c_int; i += 1 }
    i = 0 as libc::c_int;
    while i < 2187 as libc::c_int {
        mirror_pattern = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 7 as libc::c_int {
            mirror_pattern +=
                row[j as usize] * pow3[(6 as libc::c_int - j) as usize];
            j += 1
        }
        *map_mirror7.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0 as libc::c_int;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as libc::c_int {
                row[j as usize] = 0 as libc::c_int
            }
            j += 1;
            if !(row[(j - 1 as libc::c_int) as usize] == 0 as libc::c_int &&
                     j < 7 as libc::c_int) {
                break ;
            }
        }
        i += 1
    }
    /* Build the tables for 6*1-patterns */
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int { row[i as usize] = 0 as libc::c_int; i += 1 }
    i = 0 as libc::c_int;
    while i < 729 as libc::c_int {
        mirror_pattern = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 6 as libc::c_int {
            mirror_pattern +=
                row[j as usize] * pow3[(5 as libc::c_int - j) as usize];
            j += 1
        }
        *map_mirror6.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0 as libc::c_int;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as libc::c_int {
                row[j as usize] = 0 as libc::c_int
            }
            j += 1;
            if !(row[(j - 1 as libc::c_int) as usize] == 0 as libc::c_int &&
                     j < 6 as libc::c_int) {
                break ;
            }
        }
        i += 1
    }
    /* Build the tables for 5*1-patterns */
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int { row[i as usize] = 0 as libc::c_int; i += 1 }
    i = 0 as libc::c_int;
    while i < 243 as libc::c_int {
        mirror_pattern = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            mirror_pattern +=
                row[j as usize] * pow3[(4 as libc::c_int - j) as usize];
            j += 1
        }
        *map_mirror5.offset(i as isize) =
            if mirror_pattern < i { mirror_pattern } else { i };
        /* Next configuration */
        j = 0 as libc::c_int;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as libc::c_int {
                row[j as usize] = 0 as libc::c_int
            }
            j += 1;
            if !(row[(j - 1 as libc::c_int) as usize] == 0 as libc::c_int &&
                     j < 5 as libc::c_int) {
                break ;
            }
        }
        i += 1
    }
    /* Build the tables for 4*1-patterns */
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int { row[i as usize] = 0 as libc::c_int; i += 1 }
    i = 0 as libc::c_int;
    while i < 81 as libc::c_int {
        mirror_pattern = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            mirror_pattern +=
                row[j as usize] * pow3[(3 as libc::c_int - j) as usize];
            j += 1
        }
        *map_mirror4.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0 as libc::c_int;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as libc::c_int {
                row[j as usize] = 0 as libc::c_int
            }
            j += 1;
            if !(row[(j - 1 as libc::c_int) as usize] == 0 as libc::c_int &&
                     j < 4 as libc::c_int) {
                break ;
            }
        }
        i += 1
    }
    /* Build the tables for 3*1-patterns */
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int { row[i as usize] = 0 as libc::c_int; i += 1 }
    i = 0 as libc::c_int;
    while i < 27 as libc::c_int {
        mirror_pattern = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            mirror_pattern +=
                row[j as usize] * pow3[(2 as libc::c_int - j) as usize];
            j += 1
        }
        *map_mirror3.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0 as libc::c_int;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as libc::c_int {
                row[j as usize] = 0 as libc::c_int
            }
            j += 1;
            if !(row[(j - 1 as libc::c_int) as usize] == 0 as libc::c_int &&
                     j < 3 as libc::c_int) {
                break ;
            }
        }
        i += 1
    }
    /* Build the tables for 5*2-patterns */
    /* --- none needed --- */
    /* Build the tables for edge2X-patterns */
    i = 0 as libc::c_int;
    while i < 6561 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            k = 0 as libc::c_int;
            while k < 3 as libc::c_int {
                *map_mirror8x2.offset((i + 6561 as libc::c_int * j +
                                           19683 as libc::c_int * k) as isize)
                    =
                    if flip8[i as usize] + 6561 as libc::c_int * k +
                           19683 as libc::c_int * j <
                           i + 6561 as libc::c_int * j +
                               19683 as libc::c_int * k {
                        (flip8[i as usize] + 6561 as libc::c_int * k) +
                            19683 as libc::c_int * j
                    } else {
                        (i + 6561 as libc::c_int * j) +
                            19683 as libc::c_int * k
                    };
                k += 1
            }
            j += 1
        }
        i += 1
    }
    /* Build the tables for 3*3-patterns */
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int { row[i as usize] = 0 as libc::c_int; i += 1 }
    i = 0 as libc::c_int;
    while i < 19683 as libc::c_int {
        mirror_pattern =
            row[0 as libc::c_int as usize] +
                3 as libc::c_int * row[3 as libc::c_int as usize] +
                9 as libc::c_int * row[6 as libc::c_int as usize] +
                27 as libc::c_int * row[1 as libc::c_int as usize] +
                81 as libc::c_int * row[4 as libc::c_int as usize] +
                243 as libc::c_int * row[7 as libc::c_int as usize] +
                729 as libc::c_int * row[2 as libc::c_int as usize] +
                2187 as libc::c_int * row[5 as libc::c_int as usize] +
                6561 as libc::c_int * row[8 as libc::c_int as usize];
        *map_mirror33.offset(i as isize) =
            if i < mirror_pattern { i } else { mirror_pattern };
        /* Next configuration */
        j = 0 as libc::c_int;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as libc::c_int {
                row[j as usize] = 0 as libc::c_int
            }
            j += 1;
            if !(row[(j - 1 as libc::c_int) as usize] == 0 as libc::c_int &&
                     j < 9 as libc::c_int) {
                break ;
            }
        }
        i += 1
    }
    /* Read and unpack - using symmetries - the coefficient tables. */
    i = 0 as libc::c_int;
    while i < stage_count - 1 as libc::c_int {
        set[stage[i as usize] as usize].constant =
            (get_word(stream) as libc::c_int / 4 as libc::c_int) as
                libc::c_short;
        set[stage[i as usize] as usize].parity =
            (get_word(stream) as libc::c_int / 4 as libc::c_int) as
                libc::c_short;
        set[stage[i as usize] as
                usize].parity_constant[0 as libc::c_int as usize] =
            set[stage[i as usize] as usize].constant;
        set[stage[i as usize] as
                usize].parity_constant[1 as libc::c_int as usize] =
            (set[stage[i as usize] as usize].constant as libc::c_int +
                 set[stage[i as usize] as usize].parity as libc::c_int) as
                libc::c_short;
        unpack_batch(set[stage[i as usize] as usize].afile2x, map_mirror8x2,
                     59049 as libc::c_int, stream);
        unpack_batch(set[stage[i as usize] as usize].bfile, map_mirror8,
                     6561 as libc::c_int, stream);
        unpack_batch(set[stage[i as usize] as usize].cfile, map_mirror8,
                     6561 as libc::c_int, stream);
        unpack_batch(set[stage[i as usize] as usize].dfile, map_mirror8,
                     6561 as libc::c_int, stream);
        unpack_batch(set[stage[i as usize] as usize].diag8, map_mirror8,
                     6561 as libc::c_int, stream);
        unpack_batch(set[stage[i as usize] as usize].diag7, map_mirror7,
                     2187 as libc::c_int, stream);
        unpack_batch(set[stage[i as usize] as usize].diag6, map_mirror6,
                     729 as libc::c_int, stream);
        unpack_batch(set[stage[i as usize] as usize].diag5, map_mirror5,
                     243 as libc::c_int, stream);
        unpack_batch(set[stage[i as usize] as usize].diag4, map_mirror4,
                     81 as libc::c_int, stream);
        unpack_batch(set[stage[i as usize] as usize].corner33, map_mirror33,
                     19683 as libc::c_int, stream);
        unpack_batch(set[stage[i as usize] as usize].corner52,
                     0 as *mut libc::c_int, 59049 as libc::c_int, stream);
        i += 1
    }
    /* Free the mirror tables - the symmetries are now implicit
       in the coefficient tables. */
    free(map_mirror3 as *mut libc::c_void);
    free(map_mirror4 as *mut libc::c_void);
    free(map_mirror5 as *mut libc::c_void);
    free(map_mirror6 as *mut libc::c_void);
    free(map_mirror7 as *mut libc::c_void);
    free(map_mirror8 as *mut libc::c_void);
    free(map_mirror33 as *mut libc::c_void);
    free(map_mirror8x2 as *mut libc::c_void);
}
/*
   GENERATE_BATCH
   Interpolates between two stages.
*/
unsafe extern "C" fn generate_batch(mut target: *mut libc::c_short,
                                    mut count: libc::c_int,
                                    mut source1: *mut libc::c_short,
                                    mut weight1: libc::c_int,
                                    mut source2: *mut libc::c_short,
                                    mut weight2: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut total_weight: libc::c_int = 0;
    total_weight = weight1 + weight2;
    i = 0 as libc::c_int;
    while i < count {
        *target.offset(i as isize) =
            ((weight1 * *source1.offset(i as isize) as libc::c_int +
                  weight2 * *source2.offset(i as isize) as libc::c_int) /
                 total_weight) as libc::c_short;
        i += 1
    };
}
/*
   FIND_MEMORY_BLOCK
   Maintains an internal memory handler to boost
   performance and avoid heap fragmentation.
*/
unsafe extern "C" fn find_memory_block(mut afile2x: *mut *mut libc::c_short,
                                       mut bfile: *mut *mut libc::c_short,
                                       mut cfile: *mut *mut libc::c_short,
                                       mut dfile: *mut *mut libc::c_short,
                                       mut diag8: *mut *mut libc::c_short,
                                       mut diag7: *mut *mut libc::c_short,
                                       mut diag6: *mut *mut libc::c_short,
                                       mut diag5: *mut *mut libc::c_short,
                                       mut diag4: *mut *mut libc::c_short,
                                       mut corner33: *mut *mut libc::c_short,
                                       mut corner52: *mut *mut libc::c_short,
                                       mut index: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut found_free: libc::c_int = 0;
    let mut free_block: libc::c_int = 0;
    found_free = 0 as libc::c_int;
    free_block = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < block_count && found_free == 0 {
        if block_allocated[i as usize] == 0 {
            found_free = 1 as libc::c_int;
            free_block = i
        }
        i += 1
    }
    if found_free == 0 {
        if block_count < 200 as libc::c_int {
            block_list[block_count as usize] =
                safe_malloc(::std::mem::size_of::<AllocationBlock>() as
                                libc::c_ulong) as *mut AllocationBlock
        }
        if block_count == 200 as libc::c_int ||
               block_list[block_count as usize].is_null() {
            fatal_error(b"%s @ #%d\n\x00" as *const u8 as *const libc::c_char,
                        b"Memory allocation failure\x00" as *const u8 as
                            *const libc::c_char, block_count);
        }
        free_block = block_count;
        block_count += 1
    }
    *afile2x = (*block_list[free_block as usize]).afile2x_block.as_mut_ptr();
    *bfile = (*block_list[free_block as usize]).bfile_block.as_mut_ptr();
    *cfile = (*block_list[free_block as usize]).cfile_block.as_mut_ptr();
    *dfile = (*block_list[free_block as usize]).dfile_block.as_mut_ptr();
    *diag8 = (*block_list[free_block as usize]).diag8_block.as_mut_ptr();
    *diag7 = (*block_list[free_block as usize]).diag7_block.as_mut_ptr();
    *diag6 = (*block_list[free_block as usize]).diag6_block.as_mut_ptr();
    *diag5 = (*block_list[free_block as usize]).diag5_block.as_mut_ptr();
    *diag4 = (*block_list[free_block as usize]).diag4_block.as_mut_ptr();
    *corner33 =
        (*block_list[free_block as usize]).corner33_block.as_mut_ptr();
    *corner52 =
        (*block_list[free_block as usize]).corner52_block.as_mut_ptr();
    block_allocated[free_block as usize] = 1 as libc::c_int;
    block_set[free_block as usize] = index;
    return free_block;
}
/*
   FREE_MEMORY_BLOCK
   Marks a memory block as no longer in use.
*/
unsafe extern "C" fn free_memory_block(mut block: libc::c_int) {
    block_allocated[block as usize] = 0 as libc::c_int;
}
/*
   INIT_MEMORY_HANDLER
   Mark all blocks in the memory arena as "not used".
*/
unsafe extern "C" fn init_memory_handler() {
    let mut i: libc::c_int = 0;
    block_count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        block_allocated[i as usize] = 0 as libc::c_int;
        i += 1
    };
}
/*
   ALLOCATE_SET
   Finds memory for all patterns belonging to a certain stage.
*/
unsafe extern "C" fn allocate_set(mut index: libc::c_int) {
    set[index as usize].block =
        find_memory_block(&mut (*set.as_mut_ptr().offset(index as
                                                             isize)).afile2x,
                          &mut (*set.as_mut_ptr().offset(index as
                                                             isize)).bfile,
                          &mut (*set.as_mut_ptr().offset(index as
                                                             isize)).cfile,
                          &mut (*set.as_mut_ptr().offset(index as
                                                             isize)).dfile,
                          &mut (*set.as_mut_ptr().offset(index as
                                                             isize)).diag8,
                          &mut (*set.as_mut_ptr().offset(index as
                                                             isize)).diag7,
                          &mut (*set.as_mut_ptr().offset(index as
                                                             isize)).diag6,
                          &mut (*set.as_mut_ptr().offset(index as
                                                             isize)).diag5,
                          &mut (*set.as_mut_ptr().offset(index as
                                                             isize)).diag4,
                          &mut (*set.as_mut_ptr().offset(index as
                                                             isize)).corner33,
                          &mut (*set.as_mut_ptr().offset(index as
                                                             isize)).corner52,
                          index);
}
/*
   LOAD_SET
   Performs linear interpolation between the nearest stages to
   obtain the feature values for the stage in question.
   Also calculates the offset pointers to the last elements in each block
   (used for the inverted patterns when white is to move).
*/
unsafe extern "C" fn load_set(mut index: libc::c_int) {
    let mut prev: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut weight1: libc::c_int = 0;
    let mut weight2: libc::c_int = 0;
    let mut total_weight: libc::c_int = 0;
    if set[index as usize].permanent == 0 {
        prev = set[index as usize].prev;
        next = set[index as usize].next;
        if prev == next {
            weight1 = 1 as libc::c_int;
            weight2 = 1 as libc::c_int
        } else { weight1 = next - index; weight2 = index - prev }
        total_weight = weight1 + weight2;
        set[index as usize].constant =
            ((weight1 * set[prev as usize].constant as libc::c_int +
                  weight2 * set[next as usize].constant as libc::c_int) /
                 total_weight) as libc::c_short;
        set[index as usize].parity =
            ((weight1 * set[prev as usize].parity as libc::c_int +
                  weight2 * set[next as usize].parity as libc::c_int) /
                 total_weight) as libc::c_short;
        set[index as usize].parity_constant[0 as libc::c_int as usize] =
            set[index as usize].constant;
        set[index as usize].parity_constant[1 as libc::c_int as usize] =
            (set[index as usize].constant as libc::c_int +
                 set[index as usize].parity as libc::c_int) as libc::c_short;
        allocate_set(index);
        generate_batch(set[index as usize].afile2x, 59049 as libc::c_int,
                       set[prev as usize].afile2x, weight1,
                       set[next as usize].afile2x, weight2);
        generate_batch(set[index as usize].bfile, 6561 as libc::c_int,
                       set[prev as usize].bfile, weight1,
                       set[next as usize].bfile, weight2);
        generate_batch(set[index as usize].cfile, 6561 as libc::c_int,
                       set[prev as usize].cfile, weight1,
                       set[next as usize].cfile, weight2);
        generate_batch(set[index as usize].dfile, 6561 as libc::c_int,
                       set[prev as usize].dfile, weight1,
                       set[next as usize].dfile, weight2);
        generate_batch(set[index as usize].diag8, 6561 as libc::c_int,
                       set[prev as usize].diag8, weight1,
                       set[next as usize].diag8, weight2);
        generate_batch(set[index as usize].diag7, 2187 as libc::c_int,
                       set[prev as usize].diag7, weight1,
                       set[next as usize].diag7, weight2);
        generate_batch(set[index as usize].diag6, 729 as libc::c_int,
                       set[prev as usize].diag6, weight1,
                       set[next as usize].diag6, weight2);
        generate_batch(set[index as usize].diag5, 243 as libc::c_int,
                       set[prev as usize].diag5, weight1,
                       set[next as usize].diag5, weight2);
        generate_batch(set[index as usize].diag4, 81 as libc::c_int,
                       set[prev as usize].diag4, weight1,
                       set[next as usize].diag4, weight2);
        generate_batch(set[index as usize].corner33, 19683 as libc::c_int,
                       set[prev as usize].corner33, weight1,
                       set[next as usize].corner33, weight2);
        generate_batch(set[index as usize].corner52, 59049 as libc::c_int,
                       set[prev as usize].corner52, weight1,
                       set[next as usize].corner52, weight2);
    }
    set[index as usize].afile2x_last =
        set[index as usize].afile2x.offset(59048 as libc::c_int as isize);
    set[index as usize].bfile_last =
        set[index as usize].bfile.offset(6560 as libc::c_int as isize);
    set[index as usize].cfile_last =
        set[index as usize].cfile.offset(6560 as libc::c_int as isize);
    set[index as usize].dfile_last =
        set[index as usize].dfile.offset(6560 as libc::c_int as isize);
    set[index as usize].diag8_last =
        set[index as usize].diag8.offset(6560 as libc::c_int as isize);
    set[index as usize].diag7_last =
        set[index as usize].diag7.offset(2186 as libc::c_int as isize);
    set[index as usize].diag6_last =
        set[index as usize].diag6.offset(728 as libc::c_int as isize);
    set[index as usize].diag5_last =
        set[index as usize].diag5.offset(242 as libc::c_int as isize);
    set[index as usize].diag4_last =
        set[index as usize].diag4.offset(80 as libc::c_int as isize);
    set[index as usize].corner33_last =
        set[index as usize].corner33.offset(19682 as libc::c_int as isize);
    set[index as usize].corner52_last =
        set[index as usize].corner52.offset(59048 as libc::c_int as isize);
    set[index as usize].loaded = 1 as libc::c_int;
}
/*
  DISC_COUNT_ADJUSTMENT
*/
unsafe extern "C" fn eval_adjustment(mut disc_adjust: libc::c_double,
                                     mut edge_adjust: libc::c_double,
                                     mut corner_adjust: libc::c_double,
                                     mut x_adjust: libc::c_double) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut adjust: libc::c_int = 0;
    let mut row: [libc::c_int; 10] = [0; 10];
    i = 0 as libc::c_int;
    while i < stage_count - 1 as libc::c_int {
        /* Bonuses for having more discs */
        j = 0 as libc::c_int;
        while j < 59049 as libc::c_int {
            let ref mut fresh2 =
                *set[stage[i as usize] as usize].afile2x.offset(j as isize);
            *fresh2 =
                (*fresh2 as libc::c_double +
                     *set[60 as libc::c_int as
                              usize].afile2x.offset(j as isize) as libc::c_int
                         as libc::c_double * disc_adjust) as libc::c_short;
            let ref mut fresh3 =
                *set[stage[i as usize] as usize].corner52.offset(j as isize);
            *fresh3 =
                (*fresh3 as libc::c_double +
                     *set[60 as libc::c_int as
                              usize].corner52.offset(j as isize) as
                         libc::c_int as libc::c_double * disc_adjust) as
                    libc::c_short;
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 19683 as libc::c_int {
            let ref mut fresh4 =
                *set[stage[i as usize] as usize].corner33.offset(j as isize);
            *fresh4 =
                (*fresh4 as libc::c_double +
                     *set[60 as libc::c_int as
                              usize].corner33.offset(j as isize) as
                         libc::c_int as libc::c_double * disc_adjust) as
                    libc::c_short;
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 6561 as libc::c_int {
            let ref mut fresh5 =
                *set[stage[i as usize] as usize].bfile.offset(j as isize);
            *fresh5 =
                (*fresh5 as libc::c_double +
                     *set[60 as libc::c_int as usize].bfile.offset(j as isize)
                         as libc::c_int as libc::c_double * disc_adjust) as
                    libc::c_short;
            let ref mut fresh6 =
                *set[stage[i as usize] as usize].cfile.offset(j as isize);
            *fresh6 =
                (*fresh6 as libc::c_double +
                     *set[60 as libc::c_int as usize].cfile.offset(j as isize)
                         as libc::c_int as libc::c_double * disc_adjust) as
                    libc::c_short;
            let ref mut fresh7 =
                *set[stage[i as usize] as usize].dfile.offset(j as isize);
            *fresh7 =
                (*fresh7 as libc::c_double +
                     *set[60 as libc::c_int as usize].dfile.offset(j as isize)
                         as libc::c_int as libc::c_double * disc_adjust) as
                    libc::c_short;
            let ref mut fresh8 =
                *set[stage[i as usize] as usize].diag8.offset(j as isize);
            *fresh8 =
                (*fresh8 as libc::c_double +
                     *set[60 as libc::c_int as usize].diag8.offset(j as isize)
                         as libc::c_int as libc::c_double * disc_adjust) as
                    libc::c_short;
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 2187 as libc::c_int {
            let ref mut fresh9 =
                *set[stage[i as usize] as usize].diag7.offset(j as isize);
            *fresh9 =
                (*fresh9 as libc::c_double +
                     *set[60 as libc::c_int as usize].diag7.offset(j as isize)
                         as libc::c_int as libc::c_double * disc_adjust) as
                    libc::c_short;
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 729 as libc::c_int {
            let ref mut fresh10 =
                *set[stage[i as usize] as usize].diag6.offset(j as isize);
            *fresh10 =
                (*fresh10 as libc::c_double +
                     *set[60 as libc::c_int as usize].diag6.offset(j as isize)
                         as libc::c_int as libc::c_double * disc_adjust) as
                    libc::c_short;
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 243 as libc::c_int {
            let ref mut fresh11 =
                *set[stage[i as usize] as usize].diag5.offset(j as isize);
            *fresh11 =
                (*fresh11 as libc::c_double +
                     *set[60 as libc::c_int as usize].diag5.offset(j as isize)
                         as libc::c_int as libc::c_double * disc_adjust) as
                    libc::c_short;
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 81 as libc::c_int {
            let ref mut fresh12 =
                *set[stage[i as usize] as usize].diag4.offset(j as isize);
            *fresh12 =
                (*fresh12 as libc::c_double +
                     *set[60 as libc::c_int as usize].diag4.offset(j as isize)
                         as libc::c_int as libc::c_double * disc_adjust) as
                    libc::c_short;
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 10 as libc::c_int {
            row[j as usize] = 0 as libc::c_int;
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 59049 as libc::c_int {
            adjust = 0 as libc::c_int;
            /* Bonus for having edge discs */
            k = 1 as libc::c_int;
            while k <= 6 as libc::c_int {
                if row[k as usize] == 0 as libc::c_int {
                    adjust =
                        (adjust as libc::c_double + 128.0f64 * edge_adjust) as
                            libc::c_int
                } else if row[k as usize] == 2 as libc::c_int {
                    adjust =
                        (adjust as libc::c_double - 128.0f64 * edge_adjust) as
                            libc::c_int
                }
                k += 1
            }
            /* Bonus for having corners.  The "0.5 *" is because corners are part
            of two A-file+2X patterns. */
            if row[0 as libc::c_int as usize] == 0 as libc::c_int {
                adjust =
                    (adjust as libc::c_double +
                         0.5f64 * 128.0f64 * corner_adjust) as libc::c_int
            } else if row[0 as libc::c_int as usize] == 2 as libc::c_int {
                adjust =
                    (adjust as libc::c_double -
                         0.5f64 * 128.0f64 * corner_adjust) as libc::c_int
            }
            if row[7 as libc::c_int as usize] == 0 as libc::c_int {
                adjust =
                    (adjust as libc::c_double +
                         0.5f64 * 128.0f64 * corner_adjust) as libc::c_int
            } else if row[7 as libc::c_int as usize] == 2 as libc::c_int {
                adjust =
                    (adjust as libc::c_double -
                         0.5f64 * 128.0f64 * corner_adjust) as libc::c_int
            }
            /* Bonus for having X-squares when the adjacent corners are empty.
            Scaling by 0.5 applies here too. */
            if row[8 as libc::c_int as usize] == 0 as libc::c_int &&
                   row[0 as libc::c_int as usize] == 1 as libc::c_int {
                adjust =
                    (adjust as libc::c_double + 0.5f64 * 128.0f64 * x_adjust)
                        as libc::c_int
            } else if row[8 as libc::c_int as usize] == 2 as libc::c_int &&
                          row[0 as libc::c_int as usize] == 1 as libc::c_int {
                adjust =
                    (adjust as libc::c_double - 0.5f64 * 128.0f64 * x_adjust)
                        as libc::c_int
            }
            if row[9 as libc::c_int as usize] == 0 as libc::c_int &&
                   row[7 as libc::c_int as usize] == 1 as libc::c_int {
                adjust =
                    (adjust as libc::c_double + 0.5f64 * 128.0f64 * x_adjust)
                        as libc::c_int
            } else if row[9 as libc::c_int as usize] == 2 as libc::c_int &&
                          row[7 as libc::c_int as usize] == 1 as libc::c_int {
                adjust =
                    (adjust as libc::c_double - 0.5f64 * 128.0f64 * x_adjust)
                        as libc::c_int
            }
            let ref mut fresh13 =
                *set[stage[i as usize] as usize].afile2x.offset(j as isize);
            *fresh13 = (*fresh13 as libc::c_int + adjust) as libc::c_short;
            /* Next configuration */
            k = 0 as libc::c_int;
            loop  {
                /* The odometer principle */
                row[k as usize] += 1;
                if row[k as usize] == 3 as libc::c_int {
                    row[k as usize] = 0 as libc::c_int
                }
                k += 1;
                if !(row[(k - 1 as libc::c_int) as usize] == 0 as libc::c_int
                         && k < 10 as libc::c_int) {
                    break ;
                }
            }
            j += 1
        }
        i += 1
    };
}
/*
   File:         getcoeff.h

   Created:      November 20, 1997

   Modified:     August 1, 2002

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:
*/
/*
   INIT_COEFFS
   Manages the initialization of all relevant tables.
*/
#[no_mangle]
pub unsafe extern "C" fn init_coeffs() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut word1: libc::c_int = 0;
    let mut word2: libc::c_int = 0;
    let mut subsequent_stage: libc::c_int = 0;
    let mut curr_stage: libc::c_int = 0;
    let mut coeff_stream = 0 as *mut gzFile_s;
    let mut adjust_stream = 0 as *mut FILE;
    let mut sPatternFile: [libc::c_char; 260] = [0; 260];
    init_memory_handler();
    /* Linux don't support current directory. */
    strcpy(sPatternFile.as_mut_ptr(),
           b"coeffs2.bin\x00" as *const u8 as *const libc::c_char);
    coeff_stream =
        gzopen(sPatternFile.as_mut_ptr(),
               b"rb\x00" as *const u8 as *const libc::c_char);
    if coeff_stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                    b"Unable to open coefficient file\x00" as *const u8 as
                        *const libc::c_char, sPatternFile.as_mut_ptr());
    }
    /* Check the magic values in the beginning of the file to make sure
       the file format is right */
    word1 = get_word(coeff_stream) as libc::c_int;
    word2 = get_word(coeff_stream) as libc::c_int;
    if word1 != 5358 as libc::c_int || word2 != 9793 as libc::c_int {
        fatal_error(b"%s: %s\x00" as *const u8 as *const libc::c_char,
                    sPatternFile.as_mut_ptr(),
                    b"Wrong checksum in , might be an old version\x00" as
                        *const u8 as *const libc::c_char);
    }
    /* Read the different stages for which the evaluation function
       was tuned and mark the other stages with pointers to the previous
       and next stages. */
    i = 0 as libc::c_int;
    while i <= 60 as libc::c_int {
        set[i as usize].permanent = 0 as libc::c_int;
        set[i as usize].loaded = 0 as libc::c_int;
        i += 1
    }
    stage_count = get_word(coeff_stream) as libc::c_int;
    i = 0 as libc::c_int;
    while i < stage_count - 1 as libc::c_int {
        stage[i as usize] = get_word(coeff_stream) as libc::c_int;
        curr_stage = stage[i as usize];
        if i == 0 as libc::c_int {
            j = 0 as libc::c_int;
            while j < stage[0 as libc::c_int as usize] {
                set[j as usize].prev = stage[0 as libc::c_int as usize];
                set[j as usize].next = stage[0 as libc::c_int as usize];
                j += 1
            }
        } else {
            j = stage[(i - 1 as libc::c_int) as usize];
            while j < stage[i as usize] {
                set[j as usize].prev = stage[(i - 1 as libc::c_int) as usize];
                set[j as usize].next = stage[i as usize];
                j += 1
            }
        }
        set[curr_stage as usize].permanent = 1 as libc::c_int;
        allocate_set(curr_stage);
        i += 1
    }
    stage[(stage_count - 1 as libc::c_int) as usize] = 60 as libc::c_int;
    j = stage[(stage_count - 2 as libc::c_int) as usize];
    while j < 60 as libc::c_int {
        set[j as usize].prev =
            stage[(stage_count - 2 as libc::c_int) as usize];
        set[j as usize].next = 60 as libc::c_int;
        j += 1
    }
    set[60 as libc::c_int as usize].permanent = 1 as libc::c_int;
    allocate_set(60 as libc::c_int);
    /* Read the pattern values */
    unpack_coeffs(coeff_stream);
    gzclose(coeff_stream);
    /* Calculate the patterns which correspond to the board being filled */
    terminal_patterns();
    set[60 as libc::c_int as usize].constant =
        0 as libc::c_int as libc::c_short;
    set[60 as libc::c_int as usize].parity =
        0 as libc::c_int as libc::c_short;
    set[60 as libc::c_int as usize].parity_constant[0 as libc::c_int as usize]
        = set[60 as libc::c_int as usize].constant;
    set[60 as libc::c_int as usize].parity_constant[1 as libc::c_int as usize]
        =
        (set[60 as libc::c_int as usize].constant as libc::c_int +
             set[60 as libc::c_int as usize].parity as libc::c_int) as
            libc::c_short;
    /* Adjust the coefficients so as to reflect the encouragement for
       having lots of discs */
    adjust_stream =
        fopen(b"adjust.txt\x00" as *const u8 as *const libc::c_char,
              b"r\x00" as *const u8 as *const libc::c_char);
    if !adjust_stream.is_null() {
        let mut disc_adjust = 0.0f64;
        let mut edge_adjust = 0.0f64;
        let mut corner_adjust = 0.0f64;
        let mut x_adjust = 0.0f64;
        fscanf(adjust_stream,
               b"%lf %lf %lf %lf\x00" as *const u8 as *const libc::c_char,
               &mut disc_adjust as *mut libc::c_double,
               &mut edge_adjust as *mut libc::c_double,
               &mut corner_adjust as *mut libc::c_double,
               &mut x_adjust as *mut libc::c_double);
        eval_adjustment(disc_adjust, edge_adjust, corner_adjust, x_adjust);
        fclose(adjust_stream);
    }
    /* For each of number of disks played, decide on what set of evaluation
       patterns to use.
       The following rules apply:
       - Stages from the tuning are used as evaluation stages
       - Intermediate evaluation stages are introduced 2 stages before
       each tuning stage.
       - Other stages are mapped onto the next evaluation stage
       (which may be either from the tuning or an intermediate stage).
    */
    i = 0 as libc::c_int;
    while i < stage[0 as libc::c_int as usize] {
        eval_map[i as usize] = stage[0 as libc::c_int as usize];
        i += 1
    }
    i = 0 as libc::c_int;
    while i < stage_count {
        eval_map[stage[i as usize] as usize] = stage[i as usize];
        i += 1
    }
    subsequent_stage = 60 as libc::c_int;
    i = subsequent_stage;
    while i >= stage[0 as libc::c_int as usize] {
        if eval_map[i as usize] == i {
            subsequent_stage = i
        } else if i == subsequent_stage - 2 as libc::c_int {
            eval_map[i as usize] = i;
            subsequent_stage = i
        } else { eval_map[i as usize] = subsequent_stage }
        i -= 1
    };
}
/*
   PATTERN_EVALUATION
   Calculates the static evaluation of the position using
   the statistically optimized pattern tables.
*/
#[no_mangle]
pub static mut pattern_score: libc::c_short = 0;
#[no_mangle]
pub unsafe extern "C" fn pattern_evaluation(mut side_to_move: libc::c_int)
 -> libc::c_int {
    let mut eval_phase: libc::c_int = 0;
    let mut score: libc::c_short = 0;
    /* Any player wiped out? Game over then... */
    if piece_count[0 as libc::c_int as usize][disks_played as usize] ==
           0 as libc::c_int {
        if side_to_move == 0 as libc::c_int {
            return -(29000 as libc::c_int + 64 as libc::c_int)
        } else { return 29000 as libc::c_int + 64 as libc::c_int }
    } else {
        if piece_count[2 as libc::c_int as usize][disks_played as usize] ==
               0 as libc::c_int {
            if side_to_move == 0 as libc::c_int {
                return 29000 as libc::c_int + 64 as libc::c_int
            } else { return -(29000 as libc::c_int + 64 as libc::c_int) }
        }
    }
    /* Load and/or initialize the pattern coefficients */
    eval_phase = eval_map[disks_played as usize];
    if set[eval_phase as usize].loaded == 0 { load_set(eval_phase); }
    /* The constant feature and the parity feature */
    score =
        set[eval_phase as
                usize].parity_constant[(disks_played & 1 as libc::c_int) as
                                           usize];
    /* The pattern features. */
    if side_to_move == 0 as libc::c_int {
        let mut pattern0: libc::c_int = 0;
        pattern0 = board[72 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[22 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[81 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[71 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[61 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[51 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[41 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[31 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[21 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[11 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].afile2x.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[77 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[27 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[88 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[78 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[68 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[58 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[48 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[38 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[28 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[18 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].afile2x.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[27 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[22 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[18 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[17 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[16 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[15 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[14 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[13 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[12 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[11 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].afile2x.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[77 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[72 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[88 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[87 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[86 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[85 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[84 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[83 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[82 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[81 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].afile2x.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[82 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[72 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[62 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[52 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[42 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[32 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[22 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[12 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].bfile.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[87 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[77 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[67 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[57 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[47 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[37 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[27 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[17 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].bfile.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[28 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[27 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[26 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[25 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[24 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[23 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[22 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[21 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].bfile.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[78 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[77 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[76 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[75 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[74 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[73 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[72 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[71 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].bfile.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[83 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[73 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[63 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[53 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[43 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[33 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[23 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[13 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].cfile.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[86 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[76 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[66 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[56 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[46 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[36 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[26 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[16 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].cfile.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[38 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[37 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[36 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[35 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[34 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[33 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[32 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[31 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].cfile.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[68 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[67 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[66 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[65 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[64 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[63 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[62 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[61 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].cfile.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[84 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[74 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[64 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[54 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[44 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[34 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[24 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[14 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].dfile.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[85 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[75 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[65 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[55 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[45 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[35 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[25 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[15 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].dfile.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[48 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[47 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[46 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[45 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[44 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[43 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[42 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[41 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].dfile.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[58 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[57 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[56 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[55 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[54 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[53 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[52 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[51 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].dfile.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[88 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[77 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[66 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[55 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[44 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[33 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[22 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[11 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag8.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[81 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[72 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[63 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[54 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[45 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[36 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[27 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[18 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag8.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[78 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[67 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[56 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[45 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[34 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[23 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[12 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag7.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[87 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[76 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[65 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[54 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[43 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[32 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[21 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag7.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[71 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[62 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[53 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[44 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[35 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[26 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[17 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag7.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[82 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[73 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[64 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[55 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[46 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[37 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[28 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag7.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[68 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[57 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[46 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[35 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[24 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[13 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag6.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[86 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[75 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[64 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[53 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[42 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[31 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag6.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[61 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[52 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[43 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[34 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[25 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[16 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag6.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[83 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[74 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[65 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[56 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[47 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[38 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag6.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[58 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[47 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[36 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[25 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[14 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag5.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[85 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[74 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[63 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[52 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[41 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag5.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[51 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[42 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[33 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[24 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[15 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag5.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[84 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[75 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[66 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[57 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[48 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag5.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[48 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[37 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[26 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[15 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag4.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[84 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[73 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[62 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[51 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag4.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[41 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[32 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[23 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[14 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag4.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[85 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[76 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[67 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[58 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].diag4.offset(pattern0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0 = board[33 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[32 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[31 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[23 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[22 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[21 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[13 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[12 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[11 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].corner33.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[63 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[62 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[61 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[73 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[72 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[71 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[83 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[82 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[81 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].corner33.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[36 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[37 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[38 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[26 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[27 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[28 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[16 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[17 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[18 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].corner33.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[66 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[67 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[68 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[76 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[77 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[78 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[86 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[87 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[88 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].corner33.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[25 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[24 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[23 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[22 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[21 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[15 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[14 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[13 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[12 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[11 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[75 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[74 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[73 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[72 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[71 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[85 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[84 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[83 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[82 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[81 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[24 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[25 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[26 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[27 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[28 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[14 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[15 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[16 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[17 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[18 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[74 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[75 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[76 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[77 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[78 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[84 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[85 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[86 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[87 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[88 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[52 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[42 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[32 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[22 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[12 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[51 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[41 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[31 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[21 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[11 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[57 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[47 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[37 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[27 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[17 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[58 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[48 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[38 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[28 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[18 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[42 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[52 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[62 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[72 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[82 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[41 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[51 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[61 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[71 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[81 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short;
        pattern0 = board[47 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[57 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[67 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[77 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[87 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[48 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[58 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[68 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[78 as libc::c_int as usize];
        pattern0 =
            3 as libc::c_int * pattern0 + board[88 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                     as libc::c_int) as libc::c_short
    } else {
        let mut pattern0_0: libc::c_int = 0;
        pattern0_0 = board[72 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[22 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[81 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[71 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[61 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[51 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[41 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[31 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[21 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[11 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].afile2x_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[77 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[27 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[88 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[78 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[68 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[58 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[48 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[38 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[28 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[18 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].afile2x_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[27 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[22 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[18 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[17 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[16 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[15 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[14 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[13 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[12 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[11 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].afile2x_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[77 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[72 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[88 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[87 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[86 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[85 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[84 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[83 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[82 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[81 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].afile2x_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[82 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[72 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[62 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[52 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[42 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[32 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[22 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[12 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].bfile_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[87 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[77 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[67 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[57 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[47 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[37 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[27 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[17 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].bfile_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[28 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[27 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[26 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[25 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[24 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[23 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[22 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[21 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].bfile_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[78 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[77 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[76 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[75 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[74 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[73 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[72 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[71 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].bfile_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[83 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[73 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[63 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[53 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[43 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[33 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[23 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[13 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].cfile_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[86 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[76 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[66 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[56 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[46 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[36 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[26 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[16 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].cfile_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[38 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[37 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[36 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[35 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[34 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[33 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[32 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[31 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].cfile_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[68 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[67 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[66 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[65 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[64 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[63 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[62 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[61 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].cfile_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[84 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[74 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[64 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[54 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[44 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[34 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[24 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[14 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].dfile_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[85 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[75 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[65 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[55 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[45 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[35 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[25 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[15 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].dfile_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[48 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[47 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[46 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[45 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[44 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[43 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[42 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[41 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].dfile_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[58 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[57 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[56 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[55 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[54 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[53 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[52 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[51 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].dfile_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[88 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[77 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[66 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[55 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[44 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[33 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[22 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[11 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag8_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[81 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[72 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[63 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[54 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[45 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[36 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[27 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[18 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag8_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[78 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[67 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[56 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[45 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[34 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[23 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[12 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag7_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[87 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[76 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[65 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[54 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[43 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[32 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[21 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag7_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[71 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[62 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[53 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[44 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[35 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[26 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[17 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag7_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[82 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[73 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[64 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[55 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[46 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[37 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[28 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag7_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[68 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[57 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[46 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[35 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[24 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[13 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag6_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[86 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[75 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[64 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[53 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[42 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[31 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag6_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[61 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[52 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[43 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[34 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[25 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[16 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag6_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[83 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[74 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[65 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[56 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[47 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[38 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag6_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[58 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[47 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[36 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[25 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[14 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag5_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[85 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[74 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[63 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[52 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[41 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag5_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[51 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[42 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[33 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[24 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[15 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag5_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[84 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[75 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[66 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[57 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[48 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag5_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[48 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[37 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[26 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[15 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag4_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[84 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[73 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[62 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[51 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag4_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[41 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[32 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[23 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[14 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag4_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[85 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[76 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[67 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[58 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].diag4_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[33 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[32 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[31 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[23 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[22 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[21 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[13 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[12 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[11 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].corner33_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[63 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[62 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[61 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[73 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[72 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[71 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[83 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[82 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[81 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].corner33_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[36 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[37 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[38 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[26 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[27 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[28 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[16 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[17 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[18 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].corner33_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[66 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[67 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[68 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[76 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[77 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[78 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[86 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[87 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[88 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].corner33_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[25 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[24 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[23 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[22 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[21 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[15 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[14 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[13 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[12 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[11 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].corner52_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[75 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[74 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[73 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[72 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[71 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[85 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[84 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[83 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[82 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[81 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].corner52_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[24 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[25 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[26 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[27 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[28 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[14 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[15 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[16 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[17 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[18 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].corner52_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[74 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[75 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[76 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[77 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[78 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[84 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[85 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[86 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[87 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[88 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].corner52_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[52 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[42 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[32 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[22 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[12 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[51 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[41 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[31 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[21 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[11 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].corner52_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[57 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[47 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[37 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[27 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[17 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[58 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[48 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[38 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[28 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[18 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].corner52_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[42 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[52 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[62 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[72 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[82 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[41 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[51 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[61 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[71 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[81 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].corner52_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short;
        pattern0_0 = board[47 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[57 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[67 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[77 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[87 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[48 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[58 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[68 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[78 as libc::c_int as usize];
        pattern0_0 =
            3 as libc::c_int * pattern0_0 + board[88 as libc::c_int as usize];
        score =
            (score as libc::c_int +
                 *set[eval_phase as
                          usize].corner52_last.offset(-pattern0_0 as isize) as
                     libc::c_int) as libc::c_short
    }
    return score as libc::c_int;
}
/*
   REMOVE_SPECIFIC_COEFFS
   Removes the interpolated coefficients for a
   specific game phase from memory.
*/
unsafe extern "C" fn remove_specific_coeffs(mut phase: libc::c_int) {
    if set[phase as usize].loaded != 0 {
        if set[phase as usize].permanent == 0 {
            free_memory_block(set[phase as usize].block);
        }
        set[phase as usize].loaded = 0 as libc::c_int
    };
}
/*
   REMOVE_COEFFS
   Removes pattern tables which have gone out of scope from memory.
*/
#[no_mangle]
pub unsafe extern "C" fn remove_coeffs(mut phase: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < phase { remove_specific_coeffs(i); i += 1 };
}
/*
   CLEAR_COEFFS
   Remove all coefficients loaded from memory.
*/
#[no_mangle]
pub unsafe extern "C" fn clear_coeffs() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <= 60 as libc::c_int { remove_specific_coeffs(i); i += 1 };
}
