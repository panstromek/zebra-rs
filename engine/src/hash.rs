use crate::src::myrandom::{MyRandom};
use std::ffi::c_void;
use crate::src::globals::Board;
use engine_traits::Offset;
use std::collections::hash_map::RandomState;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashEntry {
    pub key1: u32,
    pub key2: u32,
    pub eval: i32,
    pub move_0: [i8; 4],
    pub draft: i16,
    pub selectivity: i16,
    pub flags: i16,
}

impl HashEntry {
    pub fn new() -> Self {
        HashEntry { key1: 0, key2: 0, eval: 0, move_0: [0; 4], draft: 0, selectivity: 0, flags: 0 }
    }
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
pub struct HashState {
    pub hash1: u32,
    pub hash2: u32,
    pub hash_put_value1: [[u32; 128]; 3],
    pub hash_put_value2: [[u32; 128]; 3],
    pub hash_flip1: [u32; 128],
    pub hash_flip2: [u32; 128],
    pub hash_flip_color1: u32,
    pub hash_flip_color2: u32,
    pub hash_stored1: [u32; 64],
    pub hash_stored2: [u32; 64],
    pub hash_mask: i32,
    pub hash_trans1: u32,
    pub hash_trans2: u32,
    pub hash_table: Vec<CompactHashEntry>,
    pub hash_value1: [[u32; 128]; 3],
    pub hash_value2: [[u32; 128]; 3],
    pub hash_color1: [u32; 3],
    pub hash_color2: [u32; 3],
}
impl HashState {
    pub const fn new() -> Self {
        HashState {
            hash1: 0,
            hash2: 0,
            hash_put_value1: [[0; 128]; 3],
            hash_put_value2: [[0; 128]; 3],
            hash_flip1: [0; 128],
            hash_flip2: [0; 128],
            hash_flip_color1: 0,
            hash_flip_color2: 0,
            hash_stored1: [0; 64],
            hash_stored2: [0; 64],
            hash_mask: 0,
            hash_trans1: 0,
            hash_trans2: 0,
            hash_table: Vec::new(),
            hash_value1: [[0; 128]; 3],
            hash_value2: [[0; 128]; 3],
            hash_color1: [0; 3],
            hash_color2: [0; 3],
        }
    }
}

/*
   DETERMINE_HASH_VALUES
   Calculates the hash codes for the given board position.
*/

pub fn determine_hash_values(side_to_move: i32, board: &Board, hash_state_: &mut HashState) {
    hash_state_.hash1 = 0;
    hash_state_.hash2 = 0;
    let mut i = 1;
    while i <= 8  {
        let mut j = 1;
        while j <= 8  {
            let pos = 10 * i + j;
            match board[pos] {
                0 => {
                    hash_state_.hash1 ^= hash_state_.hash_value1[0][pos as usize];
                    hash_state_.hash2 ^= hash_state_.hash_value2[0][pos as usize]
                }
                2 => {
                    hash_state_.hash1 ^= hash_state_.hash_value1[2][pos as usize];
                    hash_state_.hash2 ^= hash_state_.hash_value2[2][pos as usize]
                }
                _ => { }
            }
            j += 1
        }
        i += 1
    }
    hash_state_.hash1 ^= hash_state_.hash_color1[side_to_move as usize];
    hash_state_.hash2 ^= hash_state_.hash_color2[side_to_move as usize];
}

/*
   FIND_HASH
   Search the hash table for the current position. The two possible
   hash table positions are probed.
*/

pub fn find_hash(entry: &mut HashEntry, reverse_mode: i32, hash_state_: &mut HashState) {
    let mut code1: u32 = 0;
    let mut code2: u32 = 0;
    if reverse_mode != 0 {
        code1 = hash_state_.hash2 ^ hash_state_.hash_trans2;
        code2 = hash_state_.hash1 ^ hash_state_.hash_trans1
    } else {
        code1 = hash_state_.hash1 ^ hash_state_.hash_trans1;
        code2 = hash_state_.hash2 ^ hash_state_.hash_trans2
    }
    let index1 = (code1 & hash_state_.hash_mask as u32) as i32;
    let index2 = index1 ^ 1;
    let hash_table_ptr = &mut hash_state_.hash_table;
    if (hash_table_ptr.offset(index1 as isize)).key2 == code2 {
        if (hash_table_ptr.offset(index1 as isize).key1_selectivity_flags_draft ^ code1
        ) & 0xff000000 as u32 == 0 {
            compact_to_wide(hash_table_ptr.offset(index1 as isize), entry);
            return
        }
    } else if (hash_table_ptr.offset(index2 as isize)).key2 == code2 &&
        ((hash_table_ptr.offset(index2 as isize)).key1_selectivity_flags_draft
            ^ code1) & 0xff000000 as u32 == 0 {
        compact_to_wide(hash_table_ptr.offset(index2 as isize), entry);
        return
    }
    entry.draft = 0;
    entry.flags = 2;
    entry.eval = 12345678;
    entry.move_0[0] = 44;
    entry.move_0[1] = 0;
    entry.move_0[2] = 0;
    entry.move_0[3] = 0;
}

/*
   COMPACT_TO_WIDE
   Expand the compact internal representation of entries
   in the hash table to something more usable.
*/
fn compact_to_wide(compact_entry: &CompactHashEntry, entry: &mut HashEntry) {
    entry.key2 = compact_entry.key2;
    entry.eval = compact_entry.eval;
    entry.move_0[0] = (compact_entry.moves & 255) as i8;
    entry.move_0[1] = (compact_entry.moves >> 8 & 255) as i8;
    entry.move_0[2] = (compact_entry.moves >> 16 & 255) as i8;
    entry.move_0[3] = (compact_entry.moves >> 24 & 255) as i8;
    entry.key1 = compact_entry.key1_selectivity_flags_draft & 0xff000000 as u32;
    entry.selectivity = ((compact_entry.key1_selectivity_flags_draft & 0xffffff) >> 16) as i16;
    entry.flags = ((compact_entry.key1_selectivity_flags_draft & 0xffff) >> 8) as i16;
    entry.draft = (compact_entry.key1_selectivity_flags_draft & 0xff) as i16;
}

/*
   WIDE_TO_COMPACT
   Convert the easily readable representation to the more
   compact one actually stored in the hash table.
*/
pub fn wide_to_compact(entry: &HashEntry, compact_entry: &mut CompactHashEntry) {
    compact_entry.key2 = (*entry).key2;
    compact_entry.eval = (*entry).eval;
    compact_entry.moves = (
        (*entry).move_0[0] as u32 +
        ((entry.move_0[1] as u32) << 8) +
        ((entry.move_0[2] as u32) << 16) +
        ((entry.move_0[3] as u32) << 24)
    );
    compact_entry.key1_selectivity_flags_draft = (entry.key1 & 0xff000000 as u32)
        .wrapping_add(((entry.selectivity as i32) << 16 as i32) as u32)
        .wrapping_add(((entry.flags as i32) << 8 as i32) as u32)
        .wrapping_add(entry.draft as u32);
}

/*
  CLEAR_HASH_DRAFTS
  Resets the draft information for all entries in the hash table.
*/

pub fn clear_hash_drafts(state: &mut HashState) {
    let mut i = 0;
    while i < state.hash_table.len() {
        /* Set the draft to 0 */
        state.hash_table[i].key1_selectivity_flags_draft &= !(0xff as i32) as u32;
        i += 1
    };
}

/*
  GET_CLOSENESS
  Returns the closeness between the 64-bit integers (a0,a1) and (b0,b1).
  A closeness of 0 means that 32 bits differ.
*/
pub fn get_closeness(a0: u32, a1: u32, b0: u32, b1: u32) -> u32 {
    (u32::count_ones(a0 ^ b0)
        .wrapping_add(u32::count_ones(a1 ^ b1))
        .wrapping_sub(32) as i32
    ).abs() as u32
}

/*
   SETUP_HASH
   Determine randomized hash masks.
*/

pub fn setup_hash(clear: i32, hash_state_: &mut HashState, random: &mut MyRandom) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut rand_index: i32 = 0;
    let max_pair_closeness = 10;
    let max_zero_closeness = 9;
    let mut closeness: u32 = 0;
    let mut random_pair: [[u32; 2]; 130] = [[0; 2]; 130];
    let has_table_ptr = &mut hash_state_.hash_table;
    if clear != 0 {
        i = 0;
        while i < has_table_ptr.len() as i32 {
            (*has_table_ptr.offset(i as isize)).key1_selectivity_flags_draft &= !(255 as i32) as u32;
            (*has_table_ptr.offset(i as isize)).key2 = 0;
            i += 1
        }
    }
    rand_index = 0;
    while rand_index < 130 as i32 {
        'c_2013: loop {
            random_pair[rand_index as usize][0] = ((random.my_random() << 3 as i32) + (random.my_random() >> 2 as i32)) as u32;
            random_pair[rand_index as usize][1] = ((random.my_random() << 3 as i32) + (random.my_random() >> 2 as i32)) as u32;
            closeness = get_closeness(random_pair[rand_index as usize][0], random_pair[rand_index as usize][1], 0 as i32 as u32, 0 as i32 as u32);
            if closeness > max_zero_closeness { continue; }
            i = 0;
            loop {
                if !(i < rand_index) { break 'c_2013; }
                closeness = get_closeness(random_pair[rand_index as usize][0], random_pair[rand_index as usize][1], random_pair[i as usize][0], random_pair[i as usize][1]);
                if closeness > max_pair_closeness { break; }
                closeness = get_closeness(random_pair[rand_index as usize][0], random_pair[rand_index as usize][1], random_pair[i as usize][1], random_pair[i as usize][0]);
                if closeness > max_pair_closeness { break; }
                i += 1
            }
        }
        rand_index += 1
    }
    rand_index = 0;
    i = 0;
    while i < 128 as i32 {
        hash_state_.hash_value1[0][i as usize] = 0;
        hash_state_.hash_value2[0][i as usize] = 0;
        hash_state_.hash_value1[2][i as usize] = 0;
        hash_state_.hash_value2[2][i as usize] = 0;
        i += 1
    }
    i = 1;
    while i <= 8 as i32 {
        j = 1;
        while j <= 8 as i32 {
            pos = 10 as i32 * i + j;
            hash_state_.hash_value1[0][pos as usize] = random_pair[rand_index as usize][0];
            hash_state_.hash_value2[0][pos as usize] = random_pair[rand_index as usize][1];
            rand_index += 1;
            hash_state_.hash_value1[2][pos as usize] = random_pair[rand_index as usize][0];
            hash_state_.hash_value2[2][pos as usize] = random_pair[rand_index as usize][1];
            rand_index += 1;
            j += 1
        }
        i += 1
    }
    i = 0;
    while i < 128 as i32 {
        hash_state_.hash_flip1[i as usize] = hash_state_.hash_value1[0][i as usize] ^ hash_state_.hash_value1[2][i as usize];
        hash_state_.hash_flip2[i as usize] = hash_state_.hash_value2[0][i as usize] ^ hash_state_.hash_value2[2][i as usize];
        i += 1
    }
    hash_state_.hash_color1[0] = random_pair[rand_index as usize][0];
    hash_state_.hash_color2[0] = random_pair[rand_index as usize][1];
    rand_index += 1;
    hash_state_.hash_color1[2] = random_pair[rand_index as usize][0];
    hash_state_.hash_color2[2] = random_pair[rand_index as usize][1];
    rand_index += 1;
    hash_state_.hash_flip_color1 = hash_state_.hash_color1[0] ^ hash_state_.hash_color1[2];
    hash_state_.hash_flip_color2 = hash_state_.hash_color2[0] ^ hash_state_.hash_color2[2];
    j = 0;
    while j < 128 as i32 {
        hash_state_.hash_put_value1[0][j as usize] = hash_state_.hash_value1[0][j as usize] ^ hash_state_.hash_flip_color1;
        hash_state_.hash_put_value2[0][j as usize] = hash_state_.hash_value2[0][j as usize] ^ hash_state_.hash_flip_color2;
        hash_state_.hash_put_value1[2][j as usize] = hash_state_.hash_value1[2][j as usize] ^ hash_state_.hash_flip_color1;
        hash_state_.hash_put_value2[2][j as usize] = hash_state_.hash_value2[2][j as usize] ^ hash_state_.hash_flip_color2;
        j += 1
    };
}


/*
   ADD_HASH
   Add information to the hash table. Two adjacent positions are tried
   and the most shallow search is replaced.
*/

pub fn add_hash(state: &mut HashState, reverse_mode: i32,
                       score: i32,
                       best: i8,
                       flags: i32,
                       draft: i32,
                       selectivity: i32) {
    let mut old_draft: i32 = 0;
    let mut change_encouragment: i32 = 0;
    let mut index: u32 = 0;
    let mut index1: u32 = 0;
    let mut index2: u32 = 0;
    let mut code1: u32 = 0;
    let mut code2: u32 = 0;
    let mut entry = HashEntry::new();
    // TODO
    //  note for investigation. There was this assert in the original source
    //      assert( abs( score ) != SEARCH_ABORT );
    //  but SEARCH_ABORT is defined as
    //      #define SEARCH_ABORT                -27000
    //  so it cannot ever fail...
    //  And there is not comment or anything in there about it.. weird
    //
    if reverse_mode != 0 {
        code1 = state.hash2 ^ state.hash_trans2;
        code2 = state.hash1 ^ state.hash_trans1
    } else { code1 = state.hash1 ^ state.hash_trans1; code2 = state.hash2 ^ state.hash_trans2 }
    index1 = code1 & state.hash_mask as u32;
    index2 = index1 ^ 1 as i32 as u32;
    let hash_table_ptr: &mut [_] = &mut state.hash_table;
    if (*hash_table_ptr.offset(index1 as isize)).key2 == code2 {
        index = index1
    } else if (*hash_table_ptr.offset(index2 as isize)).key2 == code2 {
        index = index2
    } else if (*hash_table_ptr.offset(index1 as
        isize)).key1_selectivity_flags_draft &
        255 as i32 as u32 <=
        (*hash_table_ptr.offset(index2 as
            isize)).key1_selectivity_flags_draft
            & 255 as i32 as u32 {
        index = index1
    } else { index = index2 }
    old_draft =
        ((*hash_table_ptr.offset(index as isize)).key1_selectivity_flags_draft &
            255 as i32 as u32) as i32;
    if flags & 4 as i32 != 0 {
        /* Exact scores are potentially more useful */
        change_encouragment = 2 as i32
    } else { change_encouragment = 0 as i32 }
    if (*hash_table_ptr.offset(index as isize)).key2 == code2 {
        if old_draft > draft + change_encouragment + 2 as i32 {
            return
        }
    } else if old_draft > draft + change_encouragment + 4 as i32 {
        return
    }
    entry.key1 = code1;
    entry.key2 = code2;entry.eval = score;
    entry.move_0[0] = best;
    entry.move_0[1] = 0;
    entry.move_0[2] = 0;
    entry.move_0[3] = 0;
    entry.flags = flags as i16;
    entry.draft = draft as i16;
    entry.selectivity = selectivity as i16;
    wide_to_compact(&mut entry, hash_table_ptr.offset(index as isize));
}

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
impl HashState {
    /*
       INIT_HASH
       Allocate memory for the hash table.
    */
    pub fn init_hash(&mut self, in_hash_bits: i32) {
        let hash_size = 1i32 << in_hash_bits;
        self.hash_mask = hash_size - 1;
        self.hash_table = vec![CompactHashEntry {
            key2: 0,
            eval: 0,
            moves: 0,
            key1_selectivity_flags_draft: 0,
        }; hash_size as usize];
    }
    /*
      RESIZE_HASH
      Changes the size of the hash table.
    */
    pub fn resize_hash(&mut self, new_hash_bits: i32, random_instance_: &mut MyRandom) {
        self.hash_table.clear();
        self.init_hash(new_hash_bits);
        setup_hash(1, self, random_instance_);
    }

    /*
      SET_HASH_TRANSFORMATION
      Specify the hash code transformation masks. Changing these masks
      is the poor man's way to achieve the effect of clearing the hash
      table.
    */

    pub fn set_hash_transformation(&mut self, trans1: u32, trans2: u32) {
        self.hash_trans1 = trans1;
        self.hash_trans2 = trans2;
    }


    /*
       ADD_HASH_EXTENDED
       Add information to the hash table. Two adjacent positions are tried
       and the most shallow search is replaced.
    */

    pub fn add_hash_extended(&mut self, reverse_mode: i32,
                             score: i32,
                             best: &[i8; 4],
                             flags: i32,
                             draft: i32,
                             selectivity: i32) {
        let mut i: i32 = 0;
        let mut old_draft: i32 = 0;
        let mut change_encouragment: i32 = 0;
        let mut index: u32 = 0;
        let mut index1: u32 = 0;
        let mut index2: u32 = 0;
        let mut code1: u32 = 0;
        let mut code2: u32 = 0;
        let mut entry = HashEntry::new();
        if reverse_mode != 0 {
            code1 = self.hash2 ^ self.hash_trans2;
            code2 = self.hash1 ^ self.hash_trans1
        } else { code1 = self.hash1 ^ self.hash_trans1; code2 = self.hash2 ^ self.hash_trans2 }
        index1 = code1 & self.hash_mask as u32;
        index2 = index1 ^ 1 as i32 as u32;
        let hash_table_ptr: &mut [_] = &mut self.hash_table;
        if (*hash_table_ptr.offset(index1 as isize)).key2 == code2 {
            index = index1
        } else if (*hash_table_ptr.offset(index2 as isize)).key2 == code2 {
            index = index2
        } else if (*hash_table_ptr.offset(index1 as
            isize)).key1_selectivity_flags_draft &
            255 as i32 as u32 <=
            (*hash_table_ptr.offset(index2 as
                isize)).key1_selectivity_flags_draft
                & 255 as i32 as u32 {
            index = index1
        } else { index = index2 }
        old_draft =
            ((*hash_table_ptr.offset(index as isize)).key1_selectivity_flags_draft &
                255 as i32 as u32) as i32;
        if flags & 4 as i32 != 0 {
            /* Exact scores are potentially more useful */
            change_encouragment = 2 as i32
        } else { change_encouragment = 0 as i32 }
        if (*hash_table_ptr.offset(index as isize)).key2 == code2 {
            if old_draft > draft + change_encouragment + 2 as i32 {
                return
            }
        } else if old_draft > draft + change_encouragment + 4 as i32 {
            return
        }
        entry.key1 = code1;
        entry.key2 = code2;
        entry.eval = score;
        i = 0;
        while i < 4 as i32 {
            entry.move_0[i as usize] = best[i as usize];
            i += 1
        }
        entry.flags = flags as i16;
        entry.draft = draft as i16;
        entry.selectivity = selectivity as i16;
        wide_to_compact(&mut entry, hash_table_ptr.offset(index as isize));
    }

}
