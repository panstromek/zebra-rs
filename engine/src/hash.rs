
pub type size_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashEntry {
    pub key1: u32,
    pub key2: u32,
    pub eval: i32,
    pub move_0: [i32; 4],
    pub draft: i16,
    pub selectivity: i16,
    pub flags: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CompactHashEntry {
    pub key2: u32,
    pub eval: i32,
    pub moves: u32,
    pub key1_selectivity_flags_draft: u32,
}
/* Global variables */

pub static mut hash_size: i32 = 0;

pub static mut hash1: u32 = 0;

pub static mut hash2: u32 = 0;

pub static mut hash_value1: [[u32; 128]; 3] = [[0; 128]; 3];

pub static mut hash_value2: [[u32; 128]; 3] = [[0; 128]; 3];

pub static mut hash_put_value1: [[u32; 128]; 3] = [[0; 128]; 3];

pub static mut hash_put_value2: [[u32; 128]; 3] = [[0; 128]; 3];

pub static mut hash_flip1: [u32; 128] = [0; 128];

pub static mut hash_flip2: [u32; 128] = [0; 128];

pub static mut hash_color1: [u32; 3] = [0; 3];

pub static mut hash_color2: [u32; 3] = [0; 3];

pub static mut hash_flip_color1: u32 = 0;

pub static mut hash_flip_color2: u32 = 0;

pub static mut hash_diff1: [u32; 64] = [0; 64];

pub static mut hash_diff2: [u32; 64] = [0; 64];

pub static mut hash_stored1: [u32; 64] = [0; 64];

pub static mut hash_stored2: [u32; 64] = [0; 64];

/* Local variables */
pub static mut hash_bits: i32 = 0;
pub static mut hash_mask: i32 = 0;
pub static mut rehash_count: i32 = 0;
pub static mut hash_trans1: u32 = 0 as i32 as u32;
pub static mut hash_trans2: u32 = 0 as i32 as u32;
pub static mut hash_table: *mut CompactHashEntry =
    0 as *const CompactHashEntry as *mut CompactHashEntry;

/*
   DETERMINE_HASH_VALUES
   Calculates the hash codes for the given board position.
*/

pub unsafe fn determine_hash_values(mut side_to_move: i32,
                                    mut board:
                                    *const i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    hash1 = 0 as i32 as u32;
    hash2 = 0 as i32 as u32;
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            let mut pos = 10 as i32 * i + j;
            match *board.offset(pos as isize) {
                0 => {
                    hash1 ^=
                        hash_value1[0 as i32 as usize][pos as usize];
                    hash2 ^=
                        hash_value2[0 as i32 as usize][pos as usize]
                }
                2 => {
                    hash1 ^=
                        hash_value1[2 as i32 as usize][pos as usize];
                    hash2 ^=
                        hash_value2[2 as i32 as usize][pos as usize]
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
   FIND_HASH
   Search the hash table for the current position. The two possible
   hash table positions are probed.
*/

pub unsafe fn find_hash(mut entry: *mut HashEntry,
                        mut reverse_mode: i32) {
    let mut index1: i32 = 0;
    let mut index2: i32 = 0;
    let mut code1: u32 = 0;
    let mut code2: u32 = 0;
    if reverse_mode != 0 {
        code1 = hash2 ^ hash_trans2;
        code2 = hash1 ^ hash_trans1
    } else { code1 = hash1 ^ hash_trans1; code2 = hash2 ^ hash_trans2 }
    index1 = (code1 & hash_mask as u32) as i32;
    index2 = index1 ^ 1 as i32;
    if (*hash_table.offset(index1 as isize)).key2 == code2 {
        if ((*hash_table.offset(index1 as isize)).key1_selectivity_flags_draft
            ^ code1) & 0xff000000 as u32 ==
            0 as i32 as u32 {
            compact_to_wide(&mut *hash_table.offset(index1 as isize), entry);
            return
        }
    } else if (*hash_table.offset(index2 as isize)).key2 == code2 &&
        ((*hash_table.offset(index2 as
            isize)).key1_selectivity_flags_draft
            ^ code1) & 0xff000000 as u32 ==
            0 as i32 as u32 {
        compact_to_wide(&mut *hash_table.offset(index2 as isize), entry);
        return
    }
    (*entry).draft = 0 as i32 as i16;
    (*entry).flags = 2 as i32 as i16;
    (*entry).eval = 12345678 as i32;
    (*entry).move_0[0 as i32 as usize] = 44 as i32;
    (*entry).move_0[1 as i32 as usize] = 0 as i32;
    (*entry).move_0[2 as i32 as usize] = 0 as i32;
    (*entry).move_0[3 as i32 as usize] = 0 as i32;
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
    (*entry).move_0[0 as i32 as usize] =
        ((*compact_entry).moves & 255 as i32 as u32) as
            i32;
    (*entry).move_0[1 as i32 as usize] =
        ((*compact_entry).moves >> 8 as i32 &
            255 as i32 as u32) as i32;
    (*entry).move_0[2 as i32 as usize] =
        ((*compact_entry).moves >> 16 as i32 &
            255 as i32 as u32) as i32;
    (*entry).move_0[3 as i32 as usize] =
        ((*compact_entry).moves >> 24 as i32 &
            255 as i32 as u32) as i32;
    (*entry).key1 =
        (*compact_entry).key1_selectivity_flags_draft &
            0xff000000 as u32;
    (*entry).selectivity =
        (((*compact_entry).key1_selectivity_flags_draft &
            0xffffff as i32 as u32) >> 16 as i32)
            as i16;
    (*entry).flags =
        (((*compact_entry).key1_selectivity_flags_draft &
            0xffff as i32 as u32) >> 8 as i32) as
            i16;
    (*entry).draft =
        ((*compact_entry).key1_selectivity_flags_draft &
            0xff as i32 as u32) as i16;
}
