
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
