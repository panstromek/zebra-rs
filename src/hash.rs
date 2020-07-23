
use crate::src::libc;
use crate::src::safemem::safe_malloc;
use crate::src::stubs::{free};
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
