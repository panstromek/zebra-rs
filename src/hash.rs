
use crate::src::libc;
use crate::src::safemem::safe_malloc;
use crate::src::stubs::{free, abs, __assert_fail};
use engine::src::myrandom::my_random;
pub use ::engine::src::hash::*;

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

pub unsafe fn init_hash(mut in_hash_bits: i32) {
    hash_bits = in_hash_bits;
    hash_size = (1 as i32) << hash_bits;
    hash_mask = hash_size - 1 as i32;
    hash_table =
        safe_malloc((hash_size as
                         u64).wrapping_mul(::std::mem::size_of::<CompactHashEntry>()
                                                         as u64)) as
            *mut CompactHashEntry;
    rehash_count = 0 as i32;
}
/*
  RESIZE_HASH
  Changes the size of the hash table.
*/

pub unsafe fn resize_hash(mut new_hash_bits: i32) {
    free(hash_table as *mut libc::c_void);
    init_hash(new_hash_bits);
    setup_hash(1 as i32);
}
/*
  GET_CLOSENESS
  Returns the closeness between the 64-bit integers (a0,a1) and (b0,b1).
  A closeness of 0 means that 32 bits differ.
*/
unsafe fn get_closeness(mut a0: u32, mut a1: u32,
                                   mut b0: u32, mut b1: u32)
 -> u32 {
    return abs(popcount(a0 ^
                            b0).wrapping_add(popcount(a1 ^
                                                          b1)).wrapping_sub(32
                                                                                as
                                                                                i32
                                                                                as
                                                                                u32)
                   as i32) as u32;
}
/*
   SETUP_HASH
   Determine randomized hash masks.
*/

pub unsafe fn setup_hash(mut clear: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut rand_index: i32 = 0;
    let max_pair_closeness = 10 as i32 as u32;
    let max_zero_closeness = 9 as i32 as u32;
    let mut closeness: u32 = 0;
    let mut random_pair: [[u32; 2]; 130] = [[0; 2]; 130];
    if clear != 0 {
        i = 0 as i32;
        while i < hash_size {
            (*hash_table.offset(i as isize)).key1_selectivity_flags_draft &=
                !(255 as i32) as u32;
            (*hash_table.offset(i as isize)).key2 =
                0 as i32 as u32;
            i += 1
        }
    }
    rand_index = 0 as i32;
    while rand_index < 130 as i32 {
        'c_2013:
            loop  {
                random_pair[rand_index as usize][0 as i32 as usize] =
                    ((my_random() << 3 as i32) +
                         (my_random() >> 2 as i32)) as u32;
                random_pair[rand_index as usize][1 as i32 as usize] =
                    ((my_random() << 3 as i32) +
                         (my_random() >> 2 as i32)) as u32;
                closeness =
                    get_closeness(random_pair[rand_index as
                                                  usize][0 as i32 as
                                                             usize],
                                  random_pair[rand_index as
                                                  usize][1 as i32 as
                                                             usize],
                                  0 as i32 as u32,
                                  0 as i32 as u32);
                if closeness > max_zero_closeness { continue ; }
                i = 0 as i32;
                loop  {
                    if !(i < rand_index) { break 'c_2013 ; }
                    closeness =
                        get_closeness(random_pair[rand_index as
                                                      usize][0 as i32
                                                                 as usize],
                                      random_pair[rand_index as
                                                      usize][1 as i32
                                                                 as usize],
                                      random_pair[i as
                                                      usize][0 as i32
                                                                 as usize],
                                      random_pair[i as
                                                      usize][1 as i32
                                                                 as usize]);
                    if closeness > max_pair_closeness { break ; }
                    closeness =
                        get_closeness(random_pair[rand_index as
                                                      usize][0 as i32
                                                                 as usize],
                                      random_pair[rand_index as
                                                      usize][1 as i32
                                                                 as usize],
                                      random_pair[i as
                                                      usize][1 as i32
                                                                 as usize],
                                      random_pair[i as
                                                      usize][0 as i32
                                                                 as usize]);
                    if closeness > max_pair_closeness { break ; }
                    i += 1
                }
            }
        rand_index += 1
    }
    rand_index = 0 as i32;
    i = 0 as i32;
    while i < 128 as i32 {
        hash_value1[0 as i32 as usize][i as usize] =
            0 as i32 as u32;
        hash_value2[0 as i32 as usize][i as usize] =
            0 as i32 as u32;
        hash_value1[2 as i32 as usize][i as usize] =
            0 as i32 as u32;
        hash_value2[2 as i32 as usize][i as usize] =
            0 as i32 as u32;
        i += 1
    }
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            pos = 10 as i32 * i + j;
            hash_value1[0 as i32 as usize][pos as usize] =
                random_pair[rand_index as usize][0 as i32 as usize];
            hash_value2[0 as i32 as usize][pos as usize] =
                random_pair[rand_index as usize][1 as i32 as usize];
            rand_index += 1;
            hash_value1[2 as i32 as usize][pos as usize] =
                random_pair[rand_index as usize][0 as i32 as usize];
            hash_value2[2 as i32 as usize][pos as usize] =
                random_pair[rand_index as usize][1 as i32 as usize];
            rand_index += 1;
            j += 1
        }
        i += 1
    }
    i = 0 as i32;
    while i < 128 as i32 {
        hash_flip1[i as usize] =
            hash_value1[0 as i32 as usize][i as usize] ^
                hash_value1[2 as i32 as usize][i as usize];
        hash_flip2[i as usize] =
            hash_value2[0 as i32 as usize][i as usize] ^
                hash_value2[2 as i32 as usize][i as usize];
        i += 1
    }
    hash_color1[0 as i32 as usize] =
        random_pair[rand_index as usize][0 as i32 as usize];
    hash_color2[0 as i32 as usize] =
        random_pair[rand_index as usize][1 as i32 as usize];
    rand_index += 1;
    hash_color1[2 as i32 as usize] =
        random_pair[rand_index as usize][0 as i32 as usize];
    hash_color2[2 as i32 as usize] =
        random_pair[rand_index as usize][1 as i32 as usize];
    rand_index += 1;
    hash_flip_color1 =
        hash_color1[0 as i32 as usize] ^
            hash_color1[2 as i32 as usize];
    hash_flip_color2 =
        hash_color2[0 as i32 as usize] ^
            hash_color2[2 as i32 as usize];
    j = 0 as i32;
    while j < 128 as i32 {
        hash_put_value1[0 as i32 as usize][j as usize] =
            hash_value1[0 as i32 as usize][j as usize] ^
                hash_flip_color1;
        hash_put_value2[0 as i32 as usize][j as usize] =
            hash_value2[0 as i32 as usize][j as usize] ^
                hash_flip_color2;
        hash_put_value1[2 as i32 as usize][j as usize] =
            hash_value1[2 as i32 as usize][j as usize] ^
                hash_flip_color1;
        hash_put_value2[2 as i32 as usize][j as usize] =
            hash_value2[2 as i32 as usize][j as usize] ^
                hash_flip_color2;
        j += 1
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
   ADD_HASH
   Add information to the hash table. Two adjacent positions are tried
   and the most shallow search is replaced.
*/

pub unsafe fn add_hash(mut reverse_mode: i32,
                                  mut score: i32,
                                  mut best: i32,
                                  mut flags: i32,
                                  mut draft: i32,
                                  mut selectivity: i32) {
    let mut old_draft: i32 = 0;
    let mut change_encouragment: i32 = 0;
    let mut index: u32 = 0;
    let mut index1: u32 = 0;
    let mut index2: u32 = 0;
    let mut code1: u32 = 0;
    let mut code2: u32 = 0;
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    if abs(score) != -(27000 as i32) {
    } else {
        __assert_fail(b"abs( score ) != SEARCH_ABORT\x00" as *const u8 as
                          *const i8,
                      b"hash.c\x00" as *const u8 as *const i8,
                      372 as i32 as u32,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[i8; 44]>(b"void add_hash(int, int, int, int, int, int)\x00")).as_ptr());
    }
    if reverse_mode != 0 {
        code1 = hash2 ^ hash_trans2;
        code2 = hash1 ^ hash_trans1
    } else { code1 = hash1 ^ hash_trans1; code2 = hash2 ^ hash_trans2 }
    index1 = code1 & hash_mask as u32;
    index2 = index1 ^ 1 as i32 as u32;
    if (*hash_table.offset(index1 as isize)).key2 == code2 {
        index = index1
    } else if (*hash_table.offset(index2 as isize)).key2 == code2 {
        index = index2
    } else if (*hash_table.offset(index1 as
                                      isize)).key1_selectivity_flags_draft &
                  255 as i32 as u32 <=
                  (*hash_table.offset(index2 as
                                          isize)).key1_selectivity_flags_draft
                      & 255 as i32 as u32 {
        index = index1
    } else { index = index2 }
    old_draft =
        ((*hash_table.offset(index as isize)).key1_selectivity_flags_draft &
             255 as i32 as u32) as i32;
    if flags & 4 as i32 != 0 {
        /* Exact scores are potentially more useful */
        change_encouragment = 2 as i32
    } else { change_encouragment = 0 as i32 }
    if (*hash_table.offset(index as isize)).key2 == code2 {
        if old_draft > draft + change_encouragment + 2 as i32 {
            return
        }
    } else if old_draft > draft + change_encouragment + 4 as i32 {
        return
    }
    entry.key1 = code1;
    entry.key2 = code2;entry.eval = score;
    entry.move_0[0 as i32 as usize] = best;
    entry.move_0[1 as i32 as usize] = 0 as i32;
    entry.move_0[2 as i32 as usize] = 0 as i32;
    entry.move_0[3 as i32 as usize] = 0 as i32;
    entry.flags = flags as i16;
    entry.draft = draft as i16;
    entry.selectivity = selectivity as i16;
    wide_to_compact(&mut entry, &mut *hash_table.offset(index as isize));
}
