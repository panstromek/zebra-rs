
use crate::src::libc;
use crate::src::safemem::safe_malloc;
use crate::src::stubs::{free, abs, __assert_fail};
use crate::src::myrandom::my_random;

pub type size_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashEntry {
    pub key1: libc::c_uint,
    pub key2: libc::c_uint,
    pub eval: libc::c_int,
    pub move_0: [libc::c_int; 4],
    pub draft: libc::c_short,
    pub selectivity: libc::c_short,
    pub flags: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CompactHashEntry {
    pub key2: libc::c_uint,
    pub eval: libc::c_int,
    pub moves: libc::c_uint,
    pub key1_selectivity_flags_draft: libc::c_uint,
}
/* Global variables */

pub static mut hash_size: libc::c_int = 0;

pub static mut hash1: libc::c_uint = 0;

pub static mut hash2: libc::c_uint = 0;

pub static mut hash_value1: [[libc::c_uint; 128]; 3] = [[0; 128]; 3];

pub static mut hash_value2: [[libc::c_uint; 128]; 3] = [[0; 128]; 3];

pub static mut hash_put_value1: [[libc::c_uint; 128]; 3] = [[0; 128]; 3];

pub static mut hash_put_value2: [[libc::c_uint; 128]; 3] = [[0; 128]; 3];

pub static mut hash_flip1: [libc::c_uint; 128] = [0; 128];

pub static mut hash_flip2: [libc::c_uint; 128] = [0; 128];

pub static mut hash_color1: [libc::c_uint; 3] = [0; 3];

pub static mut hash_color2: [libc::c_uint; 3] = [0; 3];

pub static mut hash_flip_color1: libc::c_uint = 0;

pub static mut hash_flip_color2: libc::c_uint = 0;

pub static mut hash_diff1: [libc::c_uint; 64] = [0; 64];

pub static mut hash_diff2: [libc::c_uint; 64] = [0; 64];

pub static mut hash_stored1: [libc::c_uint; 64] = [0; 64];

pub static mut hash_stored2: [libc::c_uint; 64] = [0; 64];
/* Local variables */
static mut hash_bits: libc::c_int = 0;
static mut hash_mask: libc::c_int = 0;
static mut rehash_count: libc::c_int = 0;
static mut hash_trans1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut hash_trans2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut hash_table: *mut CompactHashEntry =
    0 as *const CompactHashEntry as *mut CompactHashEntry;
/* The number of entries in the hash table. Always a power of 2. */
/* The 64-bit hash key. */
/* The 64-bit hash masks for a piece of a certain color in a
   certain position. */
/* 64-bit hash masks used when a disc is played on the board;
   the relation
     hash_put_value?[][] == hash_value?[][] ^ hash_flip_color?
   is guaranteed to hold. */
/* XORs of hash_value* - used for disk flipping. */
/* 64-bit hash mask for the two different sides to move. */
/* The XOR of the hash_color*, used for disk flipping. */
/* Stored 64-bit hash mask which hold the hash codes at different nodes
   in the search tree. */
/*
   INIT_HASH
   Allocate memory for the hash table.
*/

pub unsafe fn init_hash(mut in_hash_bits: libc::c_int) {
    hash_bits = in_hash_bits;
    hash_size = (1 as libc::c_int) << hash_bits;
    hash_mask = hash_size - 1 as libc::c_int;
    hash_table =
        safe_malloc((hash_size as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<CompactHashEntry>()
                                                         as libc::c_ulong)) as
            *mut CompactHashEntry;
    rehash_count = 0 as libc::c_int;
}
/*
  RESIZE_HASH
  Changes the size of the hash table.
*/

pub unsafe fn resize_hash(mut new_hash_bits: libc::c_int) {
    free(hash_table as *mut libc::c_void);
    init_hash(new_hash_bits);
    setup_hash(1 as libc::c_int);
}
/*
  POPCOUNT
*/
unsafe fn popcount(mut b: libc::c_uint) -> libc::c_uint {
    let mut n: libc::c_uint = 0;
    n = 0 as libc::c_int as libc::c_uint;
    while b != 0 as libc::c_int as libc::c_uint {
        n = n.wrapping_add(1);
        b &= b.wrapping_sub(1 as libc::c_int as libc::c_uint)
    }
    return n;
}
/*
  GET_CLOSENESS
  Returns the closeness between the 64-bit integers (a0,a1) and (b0,b1).
  A closeness of 0 means that 32 bits differ.
*/
unsafe fn get_closeness(mut a0: libc::c_uint, mut a1: libc::c_uint,
                                   mut b0: libc::c_uint, mut b1: libc::c_uint)
 -> libc::c_uint {
    return abs(popcount(a0 ^
                            b0).wrapping_add(popcount(a1 ^
                                                          b1)).wrapping_sub(32
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                   as libc::c_int) as libc::c_uint;
}
/*
   SETUP_HASH
   Determine randomized hash masks.
*/

pub unsafe fn setup_hash(mut clear: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut rand_index: libc::c_int = 0;
    let max_pair_closeness = 10 as libc::c_int as libc::c_uint;
    let max_zero_closeness = 9 as libc::c_int as libc::c_uint;
    let mut closeness: libc::c_uint = 0;
    let mut random_pair: [[libc::c_uint; 2]; 130] = [[0; 2]; 130];
    if clear != 0 {
        i = 0 as libc::c_int;
        while i < hash_size {
            (*hash_table.offset(i as isize)).key1_selectivity_flags_draft &=
                !(255 as libc::c_int) as libc::c_uint;
            (*hash_table.offset(i as isize)).key2 =
                0 as libc::c_int as libc::c_uint;
            i += 1
        }
    }
    rand_index = 0 as libc::c_int;
    while rand_index < 130 as libc::c_int {
        'c_2013:
            loop  {
                random_pair[rand_index as usize][0 as libc::c_int as usize] =
                    ((my_random() << 3 as libc::c_int) +
                         (my_random() >> 2 as libc::c_int)) as libc::c_uint;
                random_pair[rand_index as usize][1 as libc::c_int as usize] =
                    ((my_random() << 3 as libc::c_int) +
                         (my_random() >> 2 as libc::c_int)) as libc::c_uint;
                closeness =
                    get_closeness(random_pair[rand_index as
                                                  usize][0 as libc::c_int as
                                                             usize],
                                  random_pair[rand_index as
                                                  usize][1 as libc::c_int as
                                                             usize],
                                  0 as libc::c_int as libc::c_uint,
                                  0 as libc::c_int as libc::c_uint);
                if closeness > max_zero_closeness { continue ; }
                i = 0 as libc::c_int;
                loop  {
                    if !(i < rand_index) { break 'c_2013 ; }
                    closeness =
                        get_closeness(random_pair[rand_index as
                                                      usize][0 as libc::c_int
                                                                 as usize],
                                      random_pair[rand_index as
                                                      usize][1 as libc::c_int
                                                                 as usize],
                                      random_pair[i as
                                                      usize][0 as libc::c_int
                                                                 as usize],
                                      random_pair[i as
                                                      usize][1 as libc::c_int
                                                                 as usize]);
                    if closeness > max_pair_closeness { break ; }
                    closeness =
                        get_closeness(random_pair[rand_index as
                                                      usize][0 as libc::c_int
                                                                 as usize],
                                      random_pair[rand_index as
                                                      usize][1 as libc::c_int
                                                                 as usize],
                                      random_pair[i as
                                                      usize][1 as libc::c_int
                                                                 as usize],
                                      random_pair[i as
                                                      usize][0 as libc::c_int
                                                                 as usize]);
                    if closeness > max_pair_closeness { break ; }
                    i += 1
                }
            }
        rand_index += 1
    }
    rand_index = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        hash_value1[0 as libc::c_int as usize][i as usize] =
            0 as libc::c_int as libc::c_uint;
        hash_value2[0 as libc::c_int as usize][i as usize] =
            0 as libc::c_int as libc::c_uint;
        hash_value1[2 as libc::c_int as usize][i as usize] =
            0 as libc::c_int as libc::c_uint;
        hash_value2[2 as libc::c_int as usize][i as usize] =
            0 as libc::c_int as libc::c_uint;
        i += 1
    }
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            pos = 10 as libc::c_int * i + j;
            hash_value1[0 as libc::c_int as usize][pos as usize] =
                random_pair[rand_index as usize][0 as libc::c_int as usize];
            hash_value2[0 as libc::c_int as usize][pos as usize] =
                random_pair[rand_index as usize][1 as libc::c_int as usize];
            rand_index += 1;
            hash_value1[2 as libc::c_int as usize][pos as usize] =
                random_pair[rand_index as usize][0 as libc::c_int as usize];
            hash_value2[2 as libc::c_int as usize][pos as usize] =
                random_pair[rand_index as usize][1 as libc::c_int as usize];
            rand_index += 1;
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        hash_flip1[i as usize] =
            hash_value1[0 as libc::c_int as usize][i as usize] ^
                hash_value1[2 as libc::c_int as usize][i as usize];
        hash_flip2[i as usize] =
            hash_value2[0 as libc::c_int as usize][i as usize] ^
                hash_value2[2 as libc::c_int as usize][i as usize];
        i += 1
    }
    hash_color1[0 as libc::c_int as usize] =
        random_pair[rand_index as usize][0 as libc::c_int as usize];
    hash_color2[0 as libc::c_int as usize] =
        random_pair[rand_index as usize][1 as libc::c_int as usize];
    rand_index += 1;
    hash_color1[2 as libc::c_int as usize] =
        random_pair[rand_index as usize][0 as libc::c_int as usize];
    hash_color2[2 as libc::c_int as usize] =
        random_pair[rand_index as usize][1 as libc::c_int as usize];
    rand_index += 1;
    hash_flip_color1 =
        hash_color1[0 as libc::c_int as usize] ^
            hash_color1[2 as libc::c_int as usize];
    hash_flip_color2 =
        hash_color2[0 as libc::c_int as usize] ^
            hash_color2[2 as libc::c_int as usize];
    j = 0 as libc::c_int;
    while j < 128 as libc::c_int {
        hash_put_value1[0 as libc::c_int as usize][j as usize] =
            hash_value1[0 as libc::c_int as usize][j as usize] ^
                hash_flip_color1;
        hash_put_value2[0 as libc::c_int as usize][j as usize] =
            hash_value2[0 as libc::c_int as usize][j as usize] ^
                hash_flip_color2;
        hash_put_value1[2 as libc::c_int as usize][j as usize] =
            hash_value1[2 as libc::c_int as usize][j as usize] ^
                hash_flip_color1;
        hash_put_value2[2 as libc::c_int as usize][j as usize] =
            hash_value2[2 as libc::c_int as usize][j as usize] ^
                hash_flip_color2;
        j += 1
    };
}
/*
  CLEAR_HASH_DRAFTS
  Resets the draft information for all entries in the hash table.
*/

pub unsafe fn clear_hash_drafts() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < hash_size {
        /* Set the draft to 0 */
        (*hash_table.offset(i as isize)).key1_selectivity_flags_draft &=
            !(0xff as libc::c_int) as libc::c_uint;
        i += 1
    };
}
/*
   FREE_HASH
   Remove the hash table.
*/

pub unsafe fn free_hash() {
    free(hash_table as *mut libc::c_void);
}
/*
   DETERMINE_HASH_VALUES
   Calculates the hash codes for the given board position.
*/

pub unsafe fn determine_hash_values(mut side_to_move: libc::c_int,
                                               mut board:
                                                   *const libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    hash1 = 0 as libc::c_int as libc::c_uint;
    hash2 = 0 as libc::c_int as libc::c_uint;
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            let mut pos = 10 as libc::c_int * i + j;
            match *board.offset(pos as isize) {
                0 => {
                    hash1 ^=
                        hash_value1[0 as libc::c_int as usize][pos as usize];
                    hash2 ^=
                        hash_value2[0 as libc::c_int as usize][pos as usize]
                }
                2 => {
                    hash1 ^=
                        hash_value1[2 as libc::c_int as usize][pos as usize];
                    hash2 ^=
                        hash_value2[2 as libc::c_int as usize][pos as usize]
                }
                _ => { }
            }
            j += 1
        }
        i += 1
    }
    hash1 ^= hash_color1[side_to_move as usize];
    hash2 ^= hash_color2[side_to_move as usize];
}
/*
   WIDE_TO_COMPACT
   Convert the easily readable representation to the more
   compact one actually stored in the hash table.
*/
unsafe fn wide_to_compact(mut entry: *const HashEntry,
                                     mut compact_entry:
                                         *mut CompactHashEntry) {
    (*compact_entry).key2 = (*entry).key2;
    (*compact_entry).eval = (*entry).eval;
    (*compact_entry).moves =
        ((*entry).move_0[0 as libc::c_int as usize] +
             ((*entry).move_0[1 as libc::c_int as usize] << 8 as libc::c_int)
             +
             ((*entry).move_0[2 as libc::c_int as usize] << 16 as libc::c_int)
             +
             ((*entry).move_0[3 as libc::c_int as usize] <<
                  24 as libc::c_int)) as libc::c_uint;
    (*compact_entry).key1_selectivity_flags_draft =
        ((*entry).key1 &
             0xff000000 as
                 libc::c_uint).wrapping_add((((*entry).selectivity as
                                                  libc::c_int) <<
                                                 16 as libc::c_int) as
                                                libc::c_uint).wrapping_add((((*entry).flags
                                                                                 as
                                                                                 libc::c_int)
                                                                                <<
                                                                                8
                                                                                    as
                                                                                    libc::c_int)
                                                                               as
                                                                               libc::c_uint).wrapping_add((*entry).draft
                                                                                                              as
                                                                                                              libc::c_uint);
}
/*
   COMPACT_TO_WIDE
   Expand the compact internal representation of entries
   in the hash table to something more usable.
*/
unsafe fn compact_to_wide(mut compact_entry:
                                         *const CompactHashEntry,
                                     mut entry: *mut HashEntry) {
    (*entry).key2 = (*compact_entry).key2;
    (*entry).eval = (*compact_entry).eval;
    (*entry).move_0[0 as libc::c_int as usize] =
        ((*compact_entry).moves & 255 as libc::c_int as libc::c_uint) as
            libc::c_int;
    (*entry).move_0[1 as libc::c_int as usize] =
        ((*compact_entry).moves >> 8 as libc::c_int &
             255 as libc::c_int as libc::c_uint) as libc::c_int;
    (*entry).move_0[2 as libc::c_int as usize] =
        ((*compact_entry).moves >> 16 as libc::c_int &
             255 as libc::c_int as libc::c_uint) as libc::c_int;
    (*entry).move_0[3 as libc::c_int as usize] =
        ((*compact_entry).moves >> 24 as libc::c_int &
             255 as libc::c_int as libc::c_uint) as libc::c_int;
    (*entry).key1 =
        (*compact_entry).key1_selectivity_flags_draft &
            0xff000000 as libc::c_uint;
    (*entry).selectivity =
        (((*compact_entry).key1_selectivity_flags_draft &
              0xffffff as libc::c_int as libc::c_uint) >> 16 as libc::c_int)
            as libc::c_short;
    (*entry).flags =
        (((*compact_entry).key1_selectivity_flags_draft &
              0xffff as libc::c_int as libc::c_uint) >> 8 as libc::c_int) as
            libc::c_short;
    (*entry).draft =
        ((*compact_entry).key1_selectivity_flags_draft &
             0xff as libc::c_int as libc::c_uint) as libc::c_short;
}
/*
  SET_HASH_TRANSFORMATION
  Specify the hash code transformation masks. Changing these masks
  is the poor man's way to achieve the effect of clearing the hash
  table.
*/

pub unsafe fn set_hash_transformation(mut trans1: libc::c_uint,
                                                 mut trans2: libc::c_uint) {
    hash_trans1 = trans1;
    hash_trans2 = trans2;
}
/*
   ADD_HASH
   Add information to the hash table. Two adjacent positions are tried
   and the most shallow search is replaced.
*/

pub unsafe fn add_hash(mut reverse_mode: libc::c_int,
                                  mut score: libc::c_int,
                                  mut best: libc::c_int,
                                  mut flags: libc::c_int,
                                  mut draft: libc::c_int,
                                  mut selectivity: libc::c_int) {
    let mut old_draft: libc::c_int = 0;
    let mut change_encouragment: libc::c_int = 0;
    let mut index: libc::c_uint = 0;
    let mut index1: libc::c_uint = 0;
    let mut index2: libc::c_uint = 0;
    let mut code1: libc::c_uint = 0;
    let mut code2: libc::c_uint = 0;
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    if abs(score) != -(27000 as libc::c_int) {
    } else {
        __assert_fail(b"abs( score ) != SEARCH_ABORT\x00" as *const u8 as
                          *const libc::c_char,
                      b"hash.c\x00" as *const u8 as *const libc::c_char,
                      372 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"void add_hash(int, int, int, int, int, int)\x00")).as_ptr());
    }
    if reverse_mode != 0 {
        code1 = hash2 ^ hash_trans2;
        code2 = hash1 ^ hash_trans1
    } else { code1 = hash1 ^ hash_trans1; code2 = hash2 ^ hash_trans2 }
    index1 = code1 & hash_mask as libc::c_uint;
    index2 = index1 ^ 1 as libc::c_int as libc::c_uint;
    if (*hash_table.offset(index1 as isize)).key2 == code2 {
        index = index1
    } else if (*hash_table.offset(index2 as isize)).key2 == code2 {
        index = index2
    } else if (*hash_table.offset(index1 as
                                      isize)).key1_selectivity_flags_draft &
                  255 as libc::c_int as libc::c_uint <=
                  (*hash_table.offset(index2 as
                                          isize)).key1_selectivity_flags_draft
                      & 255 as libc::c_int as libc::c_uint {
        index = index1
    } else { index = index2 }
    old_draft =
        ((*hash_table.offset(index as isize)).key1_selectivity_flags_draft &
             255 as libc::c_int as libc::c_uint) as libc::c_int;
    if flags & 4 as libc::c_int != 0 {
        /* Exact scores are potentially more useful */
        change_encouragment = 2 as libc::c_int
    } else { change_encouragment = 0 as libc::c_int }
    if (*hash_table.offset(index as isize)).key2 == code2 {
        if old_draft > draft + change_encouragment + 2 as libc::c_int {
            return
        }
    } else if old_draft > draft + change_encouragment + 4 as libc::c_int {
        return
    }
    entry.key1 = code1;
    entry.key2 = code2;
    entry.eval = score;
    entry.move_0[0 as libc::c_int as usize] = best;
    entry.move_0[1 as libc::c_int as usize] = 0 as libc::c_int;
    entry.move_0[2 as libc::c_int as usize] = 0 as libc::c_int;
    entry.move_0[3 as libc::c_int as usize] = 0 as libc::c_int;
    entry.flags = flags as libc::c_short;
    entry.draft = draft as libc::c_short;
    entry.selectivity = selectivity as libc::c_short;
    wide_to_compact(&mut entry, &mut *hash_table.offset(index as isize));
}
/*
   ADD_HASH_EXTENDED
   Add information to the hash table. Two adjacent positions are tried
   and the most shallow search is replaced.
*/

pub unsafe fn add_hash_extended(mut reverse_mode: libc::c_int,
                                           mut score: libc::c_int,
                                           mut best: *mut libc::c_int,
                                           mut flags: libc::c_int,
                                           mut draft: libc::c_int,
                                           mut selectivity: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut old_draft: libc::c_int = 0;
    let mut change_encouragment: libc::c_int = 0;
    let mut index: libc::c_uint = 0;
    let mut index1: libc::c_uint = 0;
    let mut index2: libc::c_uint = 0;
    let mut code1: libc::c_uint = 0;
    let mut code2: libc::c_uint = 0;
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    if reverse_mode != 0 {
        code1 = hash2 ^ hash_trans2;
        code2 = hash1 ^ hash_trans1
    } else { code1 = hash1 ^ hash_trans1; code2 = hash2 ^ hash_trans2 }
    index1 = code1 & hash_mask as libc::c_uint;
    index2 = index1 ^ 1 as libc::c_int as libc::c_uint;
    if (*hash_table.offset(index1 as isize)).key2 == code2 {
        index = index1
    } else if (*hash_table.offset(index2 as isize)).key2 == code2 {
        index = index2
    } else if (*hash_table.offset(index1 as
                                      isize)).key1_selectivity_flags_draft &
                  255 as libc::c_int as libc::c_uint <=
                  (*hash_table.offset(index2 as
                                          isize)).key1_selectivity_flags_draft
                      & 255 as libc::c_int as libc::c_uint {
        index = index1
    } else { index = index2 }
    old_draft =
        ((*hash_table.offset(index as isize)).key1_selectivity_flags_draft &
             255 as libc::c_int as libc::c_uint) as libc::c_int;
    if flags & 4 as libc::c_int != 0 {
        /* Exact scores are potentially more useful */
        change_encouragment = 2 as libc::c_int
    } else { change_encouragment = 0 as libc::c_int }
    if (*hash_table.offset(index as isize)).key2 == code2 {
        if old_draft > draft + change_encouragment + 2 as libc::c_int {
            return
        }
    } else if old_draft > draft + change_encouragment + 4 as libc::c_int {
        return
    }
    entry.key1 = code1;
    entry.key2 = code2;
    entry.eval = score;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        entry.move_0[i as usize] = *best.offset(i as isize);
        i += 1
    }
    entry.flags = flags as libc::c_short;
    entry.draft = draft as libc::c_short;
    entry.selectivity = selectivity as libc::c_short;
    wide_to_compact(&mut entry, &mut *hash_table.offset(index as isize));
}
/*
   FIND_HASH
   Search the hash table for the current position. The two possible
   hash table positions are probed.
*/

pub unsafe fn find_hash(mut entry: *mut HashEntry,
                                   mut reverse_mode: libc::c_int) {
    let mut index1: libc::c_int = 0;
    let mut index2: libc::c_int = 0;
    let mut code1: libc::c_uint = 0;
    let mut code2: libc::c_uint = 0;
    if reverse_mode != 0 {
        code1 = hash2 ^ hash_trans2;
        code2 = hash1 ^ hash_trans1
    } else { code1 = hash1 ^ hash_trans1; code2 = hash2 ^ hash_trans2 }
    index1 = (code1 & hash_mask as libc::c_uint) as libc::c_int;
    index2 = index1 ^ 1 as libc::c_int;
    if (*hash_table.offset(index1 as isize)).key2 == code2 {
        if ((*hash_table.offset(index1 as isize)).key1_selectivity_flags_draft
                ^ code1) & 0xff000000 as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            compact_to_wide(&mut *hash_table.offset(index1 as isize), entry);
            return
        }
    } else if (*hash_table.offset(index2 as isize)).key2 == code2 &&
                  ((*hash_table.offset(index2 as
                                           isize)).key1_selectivity_flags_draft
                       ^ code1) & 0xff000000 as libc::c_uint ==
                      0 as libc::c_int as libc::c_uint {
        compact_to_wide(&mut *hash_table.offset(index2 as isize), entry);
        return
    }
    (*entry).draft = 0 as libc::c_int as libc::c_short;
    (*entry).flags = 2 as libc::c_int as libc::c_short;
    (*entry).eval = 12345678 as libc::c_int;
    (*entry).move_0[0 as libc::c_int as usize] = 44 as libc::c_int;
    (*entry).move_0[1 as libc::c_int as usize] = 0 as libc::c_int;
    (*entry).move_0[2 as libc::c_int as usize] = 0 as libc::c_int;
    (*entry).move_0[3 as libc::c_int as usize] = 0 as libc::c_int;
}
