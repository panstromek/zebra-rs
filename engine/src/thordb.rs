use crate::src::bitboard::bit_reverse_32;
use crate::src::myrandom::my_random;
use crate::src::moves::dir_mask;
use crate::src::stubs::{abs};
use crate::src::safemem::safe_malloc;
use std::ffi::c_void;
use crate::src::error::{FrontEnd};
pub use thordb_types::{GameType, DatabaseType, C2RustUnnamed, EITHER_SELECTED_FILTER,
                       TournamentType, PlayerType, ThorOpeningNode, ThorOpeningNode_,
                       GameInfoType, PlayerFilterType, DatabaseInfoType, FilterType,
                       PrologType, TournamentDatabaseType, SearchResultType, PlayerDatabaseType};
use patterns::pow3;
use thor_opening_list::THOR_OPENING_LIST;

/* Local variables */
pub static mut thor_game_count: i32 = 0;
pub static mut thor_database_count: i32 = 0;
pub static mut thor_side_to_move: i32 = 0;
pub static mut thor_sort_criteria_count: i32 = 0;
pub static mut thor_games_sorted: i32 = 0;
pub static mut thor_games_filtered: i32 = 0;
pub static mut thor_row_pattern: [i32; 8] = [0; 8];
pub static mut thor_col_pattern: [i32; 8] = [0; 8];
pub static mut thor_board: [i32; 100] = [0; 100];
pub static mut b1_b1_map: [i32; 100] = [0; 100];
pub static mut g1_b1_map: [i32; 100] = [0; 100];
pub static mut g8_b1_map: [i32; 100] = [0; 100];
pub static mut b8_b1_map: [i32; 100] = [0; 100];
pub static mut a2_b1_map: [i32; 100] = [0; 100];
pub static mut a7_b1_map: [i32; 100] = [0; 100];
pub static mut h7_b1_map: [i32; 100] = [0; 100];
pub static mut h2_b1_map: [i32; 100] = [0; 100];
pub static mut primary_hash: [[u32; 6561]; 8] = [[0; 6561]; 8];
pub static mut secondary_hash: [[u32; 6561]; 8] = [[0; 6561]; 8];
pub static mut symmetry_map: [*mut i32; 8] =
    [0 as *const i32 as *mut i32; 8];
pub static mut inv_symmetry_map: [*mut i32; 8] =
    [0 as *const i32 as *mut i32; 8];
pub static mut move_mask_hi: [u32; 100] = [0; 100];
pub static mut move_mask_lo: [u32; 100] = [0; 100];
pub static mut unmove_mask_hi: [u32; 100] = [0; 100];
pub static mut unmove_mask_lo: [u32; 100] = [0; 100];
pub static mut database_head: *mut DatabaseType =
    0 as *const DatabaseType as *mut DatabaseType;
pub static mut players: PlayerDatabaseType =
    PlayerDatabaseType{prolog:
    PrologType{creation_century: 0,
        creation_year: 0,
        creation_month: 0,
        creation_day: 0,
        game_count: 0,
        item_count: 0,
        origin_year: 0,
        reserved: 0,},
        name_buffer:
        0 as *const i8 as *mut i8,
        count: 0,
        player_list:
        0 as *const PlayerType as *mut PlayerType,};
pub static mut thor_search: SearchResultType =
    SearchResultType{average_black_score: 0.,
        next_move_score: [0.; 100],
        match_count: 0,
        black_wins: 0,
        draws: 0,
        white_wins: 0,
        median_black_score: 0,
        allocation: 0,
        next_move_frequency: [0; 100],
        match_list:
        0 as *const *mut GameType as *mut *mut GameType,};
pub static mut tournaments: TournamentDatabaseType =
    TournamentDatabaseType{prolog:
    PrologType{creation_century: 0,
        creation_year: 0,
        creation_month: 0,
        creation_day: 0,
        game_count: 0,
        item_count: 0,
        origin_year: 0,
        reserved: 0,},
        name_buffer:
        0 as *const i8 as *mut i8,
        count: 0,
        tournament_list:
        0 as *const TournamentType as
            *mut TournamentType,};
pub static mut root_node: *mut ThorOpeningNode =
    0 as *const ThorOpeningNode as *mut ThorOpeningNode;
pub static mut default_sort_order: [i32; 5] =
    [2 as i32, 3 as i32, 1 as i32, 5 as i32,
        4 as i32];
pub static mut thor_sort_order: [i32; 10] = [0; 10];
pub static mut filter: FilterType =
    FilterType{game_categories: 0,
        first_year: 0,
        last_year: 0,
        player_filter: EITHER_SELECTED_FILTER,};

/*
  CLEAR_THOR_BOARD
*/
pub unsafe fn clear_thor_board() {
    let mut pos: i32 = 0;
    pos = 11 as i32;
    while pos <= 88 as i32 {
        thor_board[pos as usize] = 1 as i32;
        pos += 1
    }
    thor_board[54] = 0 as i32;
    thor_board[45] =
        thor_board[54];
    thor_board[55] = 2 as i32;
    thor_board[44] =
        thor_board[55];
}
/*
  PREPARE_THOR_BOARD
  Mark the positions outside the board as OUTSIDE.
*/
pub unsafe fn prepare_thor_board() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    i = 0 as i32;
    while i < 10 as i32 {
        j = 0 as i32;
        pos = 10 as i32 * i;
        while j < 10 as i32 {
            if i == 0 as i32 || i == 9 as i32 ||
                j == 0 as i32 || j == 9 as i32 {
                thor_board[pos as usize] = 3 as i32
            }
            j += 1;
            pos += 1
        }
        i += 1
    };
}
/*
  DIRECTIONAL_FLIP_COUNT
  Count the number of discs flipped in the direction given by INC
  when SQ is played by COLOR and flip those discs.
*/
pub unsafe fn directional_flip_count(sq: i32,
                                 inc: i32,
                                 color: i32,
                                 oppcol: i32)
                                 -> i32 {
    let mut count = 1 as i32;
    let mut pt = sq + inc;
    if thor_board[pt as usize] == oppcol {
        pt += inc;
        if thor_board[pt as usize] == oppcol {
            count += 1;
            pt += inc;
            if thor_board[pt as usize] == oppcol {
                count += 1;
                pt += inc;
                if thor_board[pt as usize] == oppcol {
                    count += 1;
                    pt += inc;
                    if thor_board[pt as usize] == oppcol {
                        count += 1;
                        pt += inc;
                        if thor_board[pt as usize] == oppcol {
                            count += 1;
                            pt += inc
                        }
                    }
                }
            }
        }
        if thor_board[pt as usize] == color {
            let mut g = count;
            loop  {
                pt -= inc;
                thor_board[pt as usize] = color;
                g -= 1;
                if !(g != 0) { break ; }
            }
            return count
        }
    }
    return 0 as i32;
}
/*
  DIRECTIONAL_FLIP_ANY
  Returns 1 if SQ is feasible for COLOR in the direction given by INC
  and flip the discs which are flipped if SQ is played.
*/
pub unsafe fn directional_flip_any(sq: i32,
                               inc: i32,
                               color: i32,
                               oppcol: i32)
                               -> i32 {
    let mut pt = sq + inc;
    if thor_board[pt as usize] == oppcol {
        pt += inc;
        if thor_board[pt as usize] == oppcol {
            pt += inc;
            if thor_board[pt as usize] == oppcol {
                pt += inc;
                if thor_board[pt as usize] == oppcol {
                    pt += inc;
                    if thor_board[pt as usize] == oppcol {
                        pt += inc;
                        if thor_board[pt as usize] == oppcol { pt += inc }
                    }
                }
            }
        }
        if thor_board[pt as usize] == color {
            pt -= inc;
            loop  {
                thor_board[pt as usize] = color;
                pt -= inc;
                if !(pt != sq) { break ; }
            }
            return 1 as i32
        }
    }
    return 0 as i32;
}
/*
  COUNT_FLIPS
  Returns the number of discs flipped if SQNUM is played by COLOR
  and flips those discs (if there are any).
*/
pub unsafe fn count_flips(sqnum: i32,
                      color: i32,
                      oppcol: i32) -> i32 {
    let mut count: i32 = 0;
    let mut mask: i32 = 0;
    count = 0 as i32;
    mask = dir_mask[sqnum as usize];
    if mask & 128 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, -(11 as i32), color, oppcol)
    }
    if mask & 64 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, 11 as i32, color, oppcol)
    }
    if mask & 32 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, -(10 as i32), color, oppcol)
    }
    if mask & 16 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, 10 as i32, color, oppcol)
    }
    if mask & 8 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, -(9 as i32), color, oppcol)
    }
    if mask & 4 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, 9 as i32, color, oppcol)
    }
    if mask & 2 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, -(1 as i32), color, oppcol)
    }
    if mask & 1 as i32 != 0 {
        count +=
            directional_flip_count(sqnum, 1 as i32, color, oppcol)
    }
    return count;
}
/*
  ANY_FLIPS
  Returns 1 if SQNUM flips any discs for COLOR, otherwise 0, and
  flips those discs.
*/
pub unsafe fn any_flips(sqnum: i32, color: i32,
                    oppcol: i32) -> i32 {
    let mut count: i32 = 0;
    let mut mask: i32 = 0;
    count = 0 as i32;
    mask = dir_mask[sqnum as usize];
    if mask & 128 as i32 != 0 {
        count |=
            directional_flip_any(sqnum, -(11 as i32), color, oppcol)
    }
    if mask & 64 as i32 != 0 {
        count |= directional_flip_any(sqnum, 11 as i32, color, oppcol)
    }
    if mask & 32 as i32 != 0 {
        count |=
            directional_flip_any(sqnum, -(10 as i32), color, oppcol)
    }
    if mask & 16 as i32 != 0 {
        count |= directional_flip_any(sqnum, 10 as i32, color, oppcol)
    }
    if mask & 8 as i32 != 0 {
        count |=
            directional_flip_any(sqnum, -(9 as i32), color, oppcol)
    }
    if mask & 4 as i32 != 0 {
        count |= directional_flip_any(sqnum, 9 as i32, color, oppcol)
    }
    if mask & 2 as i32 != 0 {
        count |=
            directional_flip_any(sqnum, -(1 as i32), color, oppcol)
    }
    if mask & 1 as i32 != 0 {
        count |= directional_flip_any(sqnum, 1 as i32, color, oppcol)
    }
    return count;
}
/*
  COMPUTE_THOR_PATTERNS
  Computes the row and column patterns.

*/
pub unsafe fn compute_thor_patterns(in_board: *mut i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    i = 0 as i32;
    while i < 8 as i32 {
        thor_row_pattern[i as usize] = 0 as i32;
        thor_col_pattern[i as usize] = 0 as i32;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        j = 0 as i32;
        pos = 10 as i32 * i + 11 as i32;
        while j < 8 as i32 {
            thor_row_pattern[i as usize] +=
                pow3[j as usize] * *in_board.offset(pos as isize);
            thor_col_pattern[j as usize] +=
                pow3[i as usize] * *in_board.offset(pos as isize);
            j += 1;
            pos += 1
        }
        i += 1
    };
}
/*
  GET_CORNER_MASK
  Returns an 32-bit mask for the corner configuration. The rotation
  which minimizes the numerical value is chosen.
  The mask is to be interpreted as follows: There are two bits
  for each corner; 00 means empty, 01 means black and 10 means white.
  The bit blocks are given in the order h8h1a8a1 (MSB to LSB).
  Furthermore, this 8-bit value is put in the leftmost byte if
  all four corners have been played, in the rightmost byte if only
  one corner has been played (obvious generalization for one or two
  corners).
*/
pub unsafe fn get_corner_mask(disc_a1: i32,
                          disc_a8: i32,
                          disc_h1: i32,
                          disc_h8: i32)
                          -> u32 {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut mask_a1: i32 = 0;
    let mut mask_a8: i32 = 0;
    let mut mask_h1: i32 = 0;
    let mut mask_h8: i32 = 0;
    let mut out_mask: u32 = 0;
    let mut config: [u32; 8] = [0; 8];
    mask_a1 = 0 as i32;
    if disc_a1 == 0 as i32 {
        mask_a1 = 1 as i32
    } else if disc_a1 == 2 as i32 { mask_a1 = 2 as i32 }
    mask_a8 = 0 as i32;
    if disc_a8 == 0 as i32 {
        mask_a8 = 1 as i32
    } else if disc_a8 == 2 as i32 { mask_a8 = 2 as i32 }
    mask_h1 = 0 as i32;
    if disc_h1 == 0 as i32 {
        mask_h1 = 1 as i32
    } else if disc_h1 == 2 as i32 { mask_h1 = 2 as i32 }
    mask_h8 = 0 as i32;
    if disc_h8 == 0 as i32 {
        mask_h8 = 1 as i32
    } else if disc_h8 == 2 as i32 { mask_h8 = 2 as i32 }
    count = 0 as i32;
    if disc_a1 != 1 as i32 { count += 1 }
    if disc_a8 != 1 as i32 { count += 1 }
    if disc_h1 != 1 as i32 { count += 1 }
    if disc_h8 != 1 as i32 { count += 1 }
    if count == 0 as i32 { return 0 as i32 as u32 }
    config[0] =
        (mask_a1 + 4 as i32 * mask_a8 + 16 as i32 * mask_h1 +
            64 as i32 * mask_h8) as u32;
    config[1] =
        (mask_a1 + 4 as i32 * mask_h1 + 16 as i32 * mask_a8 +
            64 as i32 * mask_h8) as u32;
    config[2] =
        (mask_a8 + 4 as i32 * mask_a1 + 16 as i32 * mask_h8 +
            64 as i32 * mask_h1) as u32;
    config[3] =
        (mask_a8 + 4 as i32 * mask_h8 + 16 as i32 * mask_a1 +
            64 as i32 * mask_h1) as u32;
    config[4] =
        (mask_h1 + 4 as i32 * mask_h8 + 16 as i32 * mask_a1 +
            64 as i32 * mask_a8) as u32;
    config[5] =
        (mask_h1 + 4 as i32 * mask_a1 + 16 as i32 * mask_h8 +
            64 as i32 * mask_a8) as u32;
    config[6] =
        (mask_h8 + 4 as i32 * mask_h1 + 16 as i32 * mask_a8 +
            64 as i32 * mask_a1) as u32;
    config[7] =
        (mask_h8 + 4 as i32 * mask_a8 + 16 as i32 * mask_h1 +
            64 as i32 * mask_a1) as u32;
    out_mask = config[0];
    i = 1 as i32;
    while i < 8 as i32 {
        out_mask =
            if out_mask < config[i as usize] {
                out_mask
            } else { config[i as usize] };
        i += 1
    }
    return out_mask << 8 as i32 * (count - 1 as i32);
}

/*
  TOURNAMENT_NAME
  Returns the name of the INDEXth tournament if available.
*/
pub unsafe fn tournament_name(index: i32)
                          -> *const i8 {
    if index < 0 as i32 || index >= tournaments.count {
        return b"<Not available>\x00" as *const u8 as *const i8
    } else {
        return tournaments.name_buffer.offset((26 as i32 * index) as
            isize)
    };
}

/*
  GET_PLAYER_NAME
  Returns the name of the INDEXth player if available.
*/

pub unsafe fn get_player_name(index: i32)
                              -> *const i8 {
    if index < 0 as i32 || index >= players.count {
        return b"< Not available >\x00" as *const u8 as *const i8
    } else {
        return players.name_buffer.offset((20 as i32 * index) as
            isize)
    };
}
/*
  GET_PLAYER_COUNT
  Returns the number of players in the database.
*/

pub unsafe fn get_player_count() -> i32 {
    return players.count;
}
/*
  PLAYER_LEX_ORDER
  Returns the index into the lexicographical order of the
  INDEXth player if available, otherwise the last index + 1.
*/
pub unsafe fn player_lex_order(index: i32) -> i32 {
    if index < 0 as i32 || index >= players.count {
        return players.count
    } else { return (*players.player_list.offset(index as isize)).lex_order };
}

/*
  GET_TOURNAMENT_NAME
  Returns the name of the INDEXth tournament if available.
*/

pub unsafe fn get_tournament_name(index: i32)
                                  -> *const i8 {
    if index < 0 as i32 || index >= tournaments.count {
        return b"< Not available >\x00" as *const u8 as *const i8
    } else {
        return tournaments.name_buffer.offset((26 as i32 * index) as
            isize)
    };
}
/*
  GET_TOURNAMENT_COUNT
  Returns the number of players in the database.
*/

pub unsafe fn get_tournament_count() -> i32 {
    return tournaments.count;
}

/*
  GET_DATABASE_COUNT
  Returns the number of game databases currently loaded.
*/

pub unsafe fn get_database_count() -> i32 {
    return thor_database_count;
}
/*
  GET_DATABASE_INFO
  Fills the vector INFO with the origin years and number of games of
  all game databases loaded.
  Enough memory must have been allocated prior to this function being
  called, that this is the case can be checked by calling GET_DATABASE_COUNT
  above.
*/

pub unsafe fn get_database_info(info: *mut DatabaseInfoType) {
    let mut i: i32 = 0;
    let mut change: i32 = 0;
    let mut temp = DatabaseInfoType{year: 0, count: 0,};
    let mut current_db = 0 as *mut DatabaseType;
    current_db = database_head;
    i = 0 as i32;
    while i < thor_database_count {
        (*info.offset(i as isize)).year = (*current_db).prolog.origin_year;
        (*info.offset(i as isize)).count = (*current_db).count;
        current_db = (*current_db).next;
        i += 1
    }
    loop
    /* Sort the list */
    {
        change = 0 as i32;
        i = 0 as i32;
        while i < thor_database_count - 1 as i32 {
            if (*info.offset(i as isize)).year >
                (*info.offset((i + 1 as i32) as isize)).year {
                change = 1 as i32;
                temp = *info.offset(i as isize);
                *info.offset(i as isize) =
                    *info.offset((i + 1 as i32) as isize);
                *info.offset((i + 1 as i32) as isize) = temp
            }
            i += 1
        }
        if !(change != 0) { break ; }
    };
}

/*
  COMPUTE_PARTIAL_HASH
  Computes the primary and secondary hash values for the
  unit element in the rotation group.
*/
pub unsafe fn compute_partial_hash(hash_val1: *mut u32,
                               hash_val2: *mut u32) {
    let mut i: i32 = 0;
    *hash_val1 = 0 as i32 as u32;
    *hash_val2 = 0 as i32 as u32;
    i = 0 as i32;
    while i < 8 as i32 {
        *hash_val1 ^=
            primary_hash[i as usize][thor_row_pattern[i as usize] as usize];
        *hash_val2 ^=
            secondary_hash[i as usize][thor_row_pattern[i as usize] as usize];
        i += 1
    };
}
/*
  COMPUTE_FULL_PRIMARY_HASH
  COMPUTE_FULL_SECONDARY_HASH
  Compute the primary and secondary hash codes respectively
  for all elements in the rotation group.
  Note: The order of the hash codes must coincide with the
        definitions in INIT_SYMMETRY_MAPS().
*/
pub unsafe fn compute_full_primary_hash(hash_val:
                                    *mut u32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 4 as i32 {
        *hash_val.offset(i as isize) = 0 as i32 as u32;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        /* b1 -> b1 */
        *hash_val.offset(0 as i32 as isize) ^=
            primary_hash[i as usize][thor_row_pattern[i as usize] as usize];
        /* b8 -> b1 */
        *hash_val.offset(1) ^=
            primary_hash[i as
                usize][thor_row_pattern[(7 as i32 - i) as
                usize] as usize];
        /* a2 -> b1 */
        *hash_val.offset(2) ^=
            primary_hash[i as usize][thor_col_pattern[i as usize] as usize];
        /* h2 -> b1 */
        *hash_val.offset(3) ^=
            primary_hash[i as
                usize][thor_col_pattern[(7 as i32 - i) as
                usize] as usize];
        i += 1
    }
    /* g1 -> b1 */
    *hash_val.offset(4) =
        bit_reverse_32(*hash_val.offset(0 as i32 as isize));
    /* g8 -> b1 */
    *hash_val.offset(5) =
        bit_reverse_32(*hash_val.offset(1));
    /* a7 -> b1 */
    *hash_val.offset(6) =
        bit_reverse_32(*hash_val.offset(2));
    /* h7 -> b1 */
    *hash_val.offset(7) =
        bit_reverse_32(*hash_val.offset(3));
}
pub unsafe fn compute_full_secondary_hash(hash_val:
                                      *mut u32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 4 as i32 {
        *hash_val.offset(i as isize) = 0 as i32 as u32;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        /* b1 -> b1 */
        *hash_val.offset(0 as i32 as isize) ^=
            secondary_hash[i as usize][thor_row_pattern[i as usize] as usize];
        /* b8 -> b1 */
        *hash_val.offset(1) ^=
            secondary_hash[i as
                usize][thor_row_pattern[(7 as i32 - i)
                as usize] as
                usize];
        /* a2 -> b1 */
        *hash_val.offset(2) ^=
            secondary_hash[i as usize][thor_col_pattern[i as usize] as usize];
        /* h2 -> b1 */
        *hash_val.offset(3) ^=
            secondary_hash[i as
                usize][thor_col_pattern[(7 as i32 - i)
                as usize] as
                usize];
        i += 1
    }
    /* g1 -> b1 */
    *hash_val.offset(4) =
        bit_reverse_32(*hash_val.offset(0 as i32 as isize));
    /* g8 -> b1 */
    *hash_val.offset(5) =
        bit_reverse_32(*hash_val.offset(1));
    /* a7 -> b1 */
    *hash_val.offset(6) =
        bit_reverse_32(*hash_val.offset(2));
    /* h7 -> b1 */
    *hash_val.offset(7) =
        bit_reverse_32(*hash_val.offset(3));
}

/*
  PRIMARY_HASH_LOOKUP
  Checks if any of the rotations of the current pattern set
  match the primary hash code TARGET_HASH.
*/
pub unsafe fn primary_hash_lookup(target_hash: u32)
                              -> i32 {
    let mut i: i32 = 0;
    let mut hit_mask: i32 = 0;
    let mut hash_val: [u32; 8] = [0; 8];
    compute_full_primary_hash(hash_val.as_mut_ptr());
    hit_mask = 0 as i32;
    i = 0 as i32;
    while i < 8 as i32 {
        if hash_val[i as usize] == target_hash {
            hit_mask |= (1 as i32) << i
        }
        i += 1
    }
    return hit_mask;
}
/*
  SECONDARY_HASH_LOOKUP
  Checks if any of the rotations of the current pattern set
  match the secondary hash code TARGET_HASH.
*/
pub unsafe fn secondary_hash_lookup(target_hash: u32)
                                -> i32 {
    let mut i: i32 = 0;
    let mut hit_mask: i32 = 0;
    let mut hash_val: [u32; 8] = [0; 8];
    compute_full_secondary_hash(hash_val.as_mut_ptr());
    hit_mask = 0 as i32;
    i = 0 as i32;
    while i < 8 as i32 {
        if hash_val[i as usize] == target_hash {
            hit_mask |= (1 as i32) << i
        }
        i += 1
    }
    return hit_mask;
}

/*
  FILTER_DATABASE
  Applies the current filter rules to the database DB.
*/
pub unsafe fn filter_database(db: *mut DatabaseType) {
    let mut i: i32 = 0;
    let mut category: i32 = 0;
    let mut passes_filter: i32 = 0;
    let mut year: i32 = 0;
    let mut game = 0 as *mut GameType;
    i = 0 as i32;
    while i < (*db).count {
        game = &mut *(*db).games.offset(i as isize) as *mut GameType;
        passes_filter = 1 as i32;
        /* Apply the tournament filter */
        if passes_filter != 0 &&
            (*tournaments.tournament_list.offset((*game).tournament_no as
                isize)).selected == 0
        {
            passes_filter = 0 as i32
        }
        /* Apply the year filter */
        if passes_filter != 0 {
            year = (*(*game).database).prolog.origin_year;
            if year < filter.first_year || year > filter.last_year {
                passes_filter = 0 as i32
            }
        }
        /* Apply the player filter */
        if passes_filter != 0 {
            match filter.player_filter as u32 {
                0 => {
                    if (*players.player_list.offset((*game).black_no as
                        isize)).selected == 0
                        &&
                        (*players.player_list.offset((*game).white_no as
                            isize)).selected
                            == 0 {
                        passes_filter = 0 as i32
                    }
                }
                1 => {
                    if (*players.player_list.offset((*game).black_no as
                        isize)).selected == 0
                        ||
                        (*players.player_list.offset((*game).white_no as
                            isize)).selected
                            == 0 {
                        passes_filter = 0 as i32
                    }
                }
                2 => {
                    if (*players.player_list.offset((*game).black_no as
                        isize)).selected == 0
                    {
                        passes_filter = 0 as i32
                    }
                }
                3 => {
                    if (*players.player_list.offset((*game).white_no as
                        isize)).selected == 0
                    {
                        passes_filter = 0 as i32
                    }
                }
                _ => { }
            }
        }
        /* Apply the game type filter */
        if passes_filter != 0 {
            if (*players.player_list.offset((*game).black_no as
                isize)).is_program != 0 {
                if (*players.player_list.offset((*game).white_no as
                    isize)).is_program != 0 {
                    category = 4 as i32
                } else { category = 2 as i32 }
            } else if (*players.player_list.offset((*game).white_no as
                isize)).is_program != 0
            {
                category = 2 as i32
            } else { category = 1 as i32 }
            passes_filter = category & filter.game_categories
        }
        (*game).passes_filter = passes_filter as i16;
        i += 1
    };
}
/*
  FILTER_ALL_DATABASES
  Applies the current filter rules to all databases.
*/
pub unsafe fn filter_all_databases() {
    let mut current_db = 0 as *mut DatabaseType;
    current_db = database_head;
    while !current_db.is_null() {
        filter_database(current_db);
        current_db = (*current_db).next
    };
}

/*
  SET_PLAYER_FILTER
  Specify what players to search for. The boolean vector SELECTED
  must contain at least PLAYERS.COUNT values - check with
  GET_PLAYER_COUNT() if necessary.
*/

pub unsafe fn set_player_filter(selected: *mut i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < players.count {
        (*players.player_list.offset(i as isize)).selected =
            *selected.offset(i as isize);
        i += 1
    }
    thor_games_filtered = 0 as i32;
}
/*
  SET_PLAYER_FILTER_TYPE
  Specifies whether it suffices for a game to contain one selected
  player or if both players have to be selected for it be displayed.
*/

pub unsafe fn set_player_filter_type(player_filter:
                                     PlayerFilterType) {
    filter.player_filter = player_filter;
}
/*
  SET_TOURNAMENT_FILTER
  Specify what tournaments to search for. The boolean vector SELECTED
  must contain at least TOURNAMENTS.COUNT values - check with
  GET_TOURNAMENT_COUNT() if necessary.
*/

pub unsafe fn set_tournament_filter(selected:
                                    *mut i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < tournaments.count {
        (*tournaments.tournament_list.offset(i as isize)).selected =
            *selected.offset(i as isize);
        i += 1
    }
    thor_games_filtered = 0 as i32;
}
/*
  SET_YEAR_FILTER
  Specify the interval of years to which the search will be confined.
*/

pub unsafe fn set_year_filter(first_year: i32,
                              last_year: i32) {
    filter.first_year = first_year;
    filter.last_year = last_year;
    thor_games_filtered = 0 as i32;
}
/*
  SPECIFY_GAME_CATEGORIES
  Specify the types of games in the database that are displayed
  if they match the position probed for. The input is the binary
  OR of the flags for the types enabled.
*/

pub unsafe fn specify_game_categories(categories:
                                      i32) {
    if categories != filter.game_categories {
        filter.game_categories = categories;
        thor_games_filtered = 0 as i32
    };
}
/*
  SPECIFY_THOR_SORT_ORDER
  Specifies that in subsequent calls to SORT_THOR_MATCHES,
  the COUNT first elements of SORT_ORDER are to be used
  (in decreasing order of priority).
  Note: If there aren't (at least) COUNT elements at the location
        to which SORT_ORDER points, a crash is likely.
*/

pub unsafe fn specify_thor_sort_order(mut count: i32,
                                      sort_order:
                                      *mut i32) {
    let mut i: i32 = 0;
    /* Truncate the input vector if it is too long */
    count = if count < 10 as i32 { count } else { 10 as i32 };
    /* Check if the new order coincides with the old order */
    if count != thor_sort_criteria_count {
        thor_games_sorted = 0 as i32
    } else {
        i = 0 as i32;
        while i < count {
            if *sort_order.offset(i as isize) != thor_sort_order[i as usize] {
                thor_games_sorted = 0 as i32
            }
            i += 1
        }
    }
    thor_sort_criteria_count = count;
    i = 0 as i32;
    while i < count {
        thor_sort_order[i as usize] = *sort_order.offset(i as isize);
        i += 1
    };
}

/*
  RECURSIVE_OPENING_SCAN
  Performs a preorder traversal of the opening tree rooted
  at NODE and checks which opening nodes are compatible
  with the primary and secondary hash codes from the 8 different
  rotations.
*/
pub unsafe fn recursive_opening_scan(mut node: *mut ThorOpeningNode,
                                 depth: i32,
                                 moves_played: i32,
                                 primary_hash_0:
                                 *mut u32,
                                 secondary_hash_0:
                                 *mut u32) {
    let mut i: i32 = 0;
    let mut match_0: i32 = 0;
    let mut matching_symmetry: i32 = 0;
    let mut child = 0 as *mut ThorOpeningNode;
    /* Determine the status of the current node */
    if depth < moves_played {
        (*node).matching_symmetry = 0 as i32;
        (*node).current_match = 0 as i32
    } else if depth == moves_played {
        /* Check the hash codes */
        match_0 = 0 as i32;
        matching_symmetry = 0 as i32;
        i = 7 as i32;
        while i >= 0 as i32 {
            if (*node).hash1 == *primary_hash_0.offset(i as isize) &&
                (*node).hash2 == *secondary_hash_0.offset(i as isize) {
                match_0 = 1 as i32;
                matching_symmetry = i
            }
            i -= 1
        }
        if match_0 != 0 {
            (*node).matching_symmetry = matching_symmetry;
            (*node).current_match = 1 as i32
        } else { (*node).current_match = 2 as i32 }
    } else {
        /* depth > moves_played */
        (*node).current_match = (*(*node).parent_node).current_match;
        (*node).matching_symmetry = (*(*node).parent_node).matching_symmetry
    }
    /* Recursively search the childen */
    child = (*node).child_node;
    while !child.is_null() {
        recursive_opening_scan(child, depth + 1 as i32, moves_played,
                               primary_hash_0, secondary_hash_0);
        child = (*child).sibling_node
    };
}

/*
  OPENING_SCAN
  Fills the opening tree with information on how well
  the current pattern configuration matches the openings.
*/
pub unsafe fn opening_scan(moves_played: i32) {
    let mut primary_hash_0: [u32; 8] = [0; 8];
    let mut secondary_hash_0: [u32; 8] = [0; 8];
    compute_full_primary_hash(primary_hash_0.as_mut_ptr());
    compute_full_secondary_hash(secondary_hash_0.as_mut_ptr());
    recursive_opening_scan(root_node, 0 as i32, moves_played,
                           primary_hash_0.as_mut_ptr(),
                           secondary_hash_0.as_mut_ptr());
}
/*
  RECURSIVE_FREQUENCY_COUNT
  Recursively fills frequency table FREQ_COUNT which is to contain
  the number of times each move has been played according to the
  trimmed set of openings from the Thor database.
*/
pub unsafe fn recursive_frequency_count(node: *mut ThorOpeningNode,
                                    freq_count:
                                    *mut i32,
                                    depth: i32,
                                    moves_played: i32,
                                    symmetries:
                                    *mut i32,
                                    primary_hash_0:
                                    *mut u32,
                                    secondary_hash_0:
                                    *mut u32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut child_move: i32 = 0;
    let mut child = 0 as *mut ThorOpeningNode;
    if depth == moves_played {
        i = 0 as i32;
        while i < 8 as i32 {
            j = *symmetries.offset(i as isize);
            if (*node).hash1 == *primary_hash_0.offset(j as isize) &&
                (*node).hash2 == *secondary_hash_0.offset(j as isize) {
                child_move = (*node).child_move as i32;
                child = (*node).child_node;
                while !child.is_null() {
                    *freq_count.offset(*inv_symmetry_map[j as
                        usize].offset(child_move
                        as
                        isize)
                        as isize) += (*child).frequency;
                    child_move = (*child).sibling_move as i32;
                    child = (*child).sibling_node
                }
                break ;
            } else { i += 1 }
        }
    } else if depth < moves_played {
        child = (*node).child_node;
        while !child.is_null() {
            recursive_frequency_count(child, freq_count,
                                      depth + 1 as i32, moves_played,
                                      symmetries, primary_hash_0,
                                      secondary_hash_0);
            child = (*child).sibling_node
        }
    };
}

/*
  GET_THOR_GAME
  Returns all available information about the INDEXth game
  in the list of matching games generated by DATABASE_SEARCH.
*/

pub unsafe fn get_thor_game(index: i32)
                            -> GameInfoType {
    let mut info =
        GameInfoType{black_name: 0 as *const i8,
            white_name: 0 as *const i8,
            tournament: 0 as *const i8,
            year: 0,
            black_actual_score: 0,
            black_corrected_score: 0,};
    let mut game = 0 as *mut GameType;
    if index < 0 as i32 || index >= thor_search.match_count {
        /* Bad index, so fill with empty values */
        info.black_name = b"\x00" as *const u8 as *const i8;
        info.white_name = b"\x00" as *const u8 as *const i8;
        info.tournament = b"\x00" as *const u8 as *const i8;
        info.year = 0 as i32;
        info.black_actual_score = 32 as i32;
        info.black_corrected_score = 32 as i32
    } else {
        /* Copy name fields etc */
        game = *thor_search.match_list.offset(index as isize);
        info.black_name = get_player_name((*game).black_no as i32);
        info.white_name = get_player_name((*game).white_no as i32);
        info.tournament =
            tournament_name((*game).tournament_no as i32);
        info.year = (*(*game).database).prolog.origin_year;
        info.black_actual_score = (*game).actual_black_score as i32;
        info.black_corrected_score =
            (*game).perfect_black_score as i32
    }
    return info;
}

/*
  GET_THOR_GAME_MOVE_COUNT
  Returns the number of moves in the INDEXth game
  in the list of matching games generated by DATABASE_SEARCH.
*/

pub unsafe fn get_thor_game_move_count(index: i32)
                                       -> i32 {
    if index < 0 as i32 || index >= thor_search.match_count {
        /* Bad index */
        return -(1 as i32)
    } else {
        return (**thor_search.match_list.offset(index as isize)).move_count as
            i32
    };
}

/*
  GET_TOTAL_GAME_COUNT
  GET_MATCH_COUNT
  GET_BLACK_WIN_COUNT
  GET_DRAW_COUNT
  GET_WHITE_WIN_COUNT
  GET_BLACK_MEDIAN_SCORE
  GET_AVERAGE_BLACK_SCORE
  GET_MOVE_FREQUENCY
  GET_MOVE_WIN_RATE
  Accessor functions which return statistics from the last
  query to DATABASE_SEARCH.
*/

pub unsafe fn get_total_game_count() -> i32 {
    return thor_game_count;
}

pub unsafe fn get_match_count() -> i32 {
    return thor_search.match_count;
}

pub unsafe fn get_black_win_count() -> i32 {
    return thor_search.black_wins;
}

pub unsafe fn get_draw_count() -> i32 {
    return thor_search.draws;
}

pub unsafe fn get_white_win_count() -> i32 {
    return thor_search.white_wins;
}

pub unsafe fn get_black_median_score() -> i32 {
    return thor_search.median_black_score;
}

pub unsafe fn get_black_average_score() -> f64 {
    return thor_search.average_black_score;
}

pub unsafe fn get_move_frequency(move_0: i32)
                                 -> i32 {
    return thor_search.next_move_frequency[move_0 as usize];
}

pub unsafe fn get_move_win_rate(move_0: i32)
                                -> f64 {
    if thor_search.next_move_frequency[move_0 as usize] == 0 as i32 {
        return 0.0f64
    } else {
        return thor_search.next_move_score[move_0 as usize] /
            thor_search.next_move_frequency[move_0 as usize] as
                f64
    };
}

/*
  INIT_MOVE_MASKS
  Initializes the shape bit masks for each of the possible moves.
*/
pub unsafe fn init_move_masks() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut index: i32 = 0;
    i = 0 as i32;
    while i < 4 as i32 {
        j = 0 as i32;
        pos = 10 as i32 * i + 11 as i32;
        while j < 8 as i32 {
            index = 8 as i32 * i + j;
            move_mask_lo[pos as usize] =
                ((1 as i32) << index) as u32;
            move_mask_hi[pos as usize] = 0 as i32 as u32;
            unmove_mask_lo[pos as usize] =
                !((1 as i32) << index) as u32;
            unmove_mask_hi[pos as usize] =
                !(0 as i32) as u32;
            j += 1;
            pos += 1
        }
        i += 1
    }
    i = 0 as i32;
    while i < 4 as i32 {
        j = 0 as i32;
        pos = 10 as i32 * i + 51 as i32;
        while j < 8 as i32 {
            index = 8 as i32 * i + j;
            move_mask_lo[pos as usize] = 0 as i32 as u32;
            move_mask_hi[pos as usize] =
                ((1 as i32) << index) as u32;
            unmove_mask_lo[pos as usize] =
                !(0 as i32) as u32;
            unmove_mask_hi[pos as usize] =
                !((1 as i32) << index) as u32;
            j += 1;
            pos += 1
        }
        i += 1
    };
}
/*
  CALCULATE_OPENING_FREQUENCY
  Calculates and returns the number of lines in the Thor opening base
  that match the line defined by NODE.
*/
pub unsafe fn calculate_opening_frequency(mut node:
                                      *mut ThorOpeningNode)
                                      -> i32 {
    let mut sum: i32 = 0;
    let mut child = 0 as *mut ThorOpeningNode;
    child = (*node).child_node;
    if child.is_null() {
        return (*node).frequency
    } else {
        sum = 0 as i32;
        loop  {
            sum += calculate_opening_frequency(child);
            child = (*child).sibling_node;
            if child.is_null() { break ; }
        }
        (*node).frequency = sum;
        return sum
    };
}

/*
  GET_THOR_GAME_SIZE
  Returns the amount of memory which each game in the database takes.
*/

pub unsafe fn get_thor_game_size() -> i32 {
    return ::core::mem::size_of::<GameType>() as u64 as i32;
}
/*
  INIT_SYMMETRY_MAPS
  Initializes the mappings which the 8 elements in the board
  symmetry group induce (and their inverses).
  Note: The order of the mappings must coincide with the order
        in which they are calculated in COMPUTE_FULL_PRIMARY_HASH()
    and COMPUTE_FULL_SECONDARY_HASH().
*/
pub unsafe fn init_symmetry_maps<FE: FrontEnd>() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut pos: i32 = 0;
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            pos = 10 as i32 * i + j;
            b1_b1_map[pos as usize] = pos;
            g1_b1_map[pos as usize] =
                10 as i32 * i + (9 as i32 - j);
            g8_b1_map[pos as usize] =
                10 as i32 * (9 as i32 - i) +
                    (9 as i32 - j);
            b8_b1_map[pos as usize] =
                10 as i32 * (9 as i32 - i) + j;
            a2_b1_map[pos as usize] = 10 as i32 * j + i;
            a7_b1_map[pos as usize] =
                10 as i32 * j + (9 as i32 - i);
            h7_b1_map[pos as usize] =
                10 as i32 * (9 as i32 - j) +
                    (9 as i32 - i);
            h2_b1_map[pos as usize] =
                10 as i32 * (9 as i32 - j) + i;
            j += 1
        }
        i += 1
    }
    symmetry_map[0] = b1_b1_map.as_mut_ptr();
    inv_symmetry_map[0] = b1_b1_map.as_mut_ptr();
    symmetry_map[1] = b8_b1_map.as_mut_ptr();
    inv_symmetry_map[1] = b8_b1_map.as_mut_ptr();
    symmetry_map[2] = a2_b1_map.as_mut_ptr();
    inv_symmetry_map[2] = a2_b1_map.as_mut_ptr();
    symmetry_map[3] = h2_b1_map.as_mut_ptr();
    inv_symmetry_map[3] = a7_b1_map.as_mut_ptr();
    symmetry_map[4] = g1_b1_map.as_mut_ptr();
    inv_symmetry_map[4] = g1_b1_map.as_mut_ptr();
    symmetry_map[5] = g8_b1_map.as_mut_ptr();
    inv_symmetry_map[5] = g8_b1_map.as_mut_ptr();
    symmetry_map[6] = a7_b1_map.as_mut_ptr();
    inv_symmetry_map[6] = h2_b1_map.as_mut_ptr();
    symmetry_map[7] = h7_b1_map.as_mut_ptr();
    inv_symmetry_map[7] = h7_b1_map.as_mut_ptr();
    i = 0 as i32;
    while i < 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            k = 1 as i32;
            while k <= 8 as i32 {
                pos = 10 as i32 * j + k;
                if *inv_symmetry_map[i as
                    usize].offset(*symmetry_map[i as
                    usize].offset(pos
                    as
                    isize)
                    as isize) != pos {
                    let to_report = *inv_symmetry_map[i as
                        usize].offset(*symmetry_map[i
                        as
                        usize].offset(pos
                        as
                        isize)
                        as
                        isize);
                    FE::error_in_map_thor(i, pos, to_report);
                }
                k += 1
            }
            j += 1
        }
        i += 1
    };
}

/*
  PLAY_THROUGH_GAME
  Play the MAX_MOVES first moves of GAME and update THOR_BOARD
  and THOR_SIDE_TO_MOVE to represent the position after those moves.
*/
pub unsafe fn play_through_game(game: *mut GameType,
                                max_moves: i32)
                                -> i32 {
    let mut i: i32 = 0;
    let mut move_0: i32 = 0;
    let mut flipped: i32 = 0;
    clear_thor_board();
    thor_side_to_move = 0 as i32;
    i = 0 as i32;
    while i < max_moves {
        move_0 = abs((*game).moves[i as usize] as i32);
        flipped =
            any_flips(move_0, thor_side_to_move,
                      0 as i32 + 2 as i32 -
                          thor_side_to_move);
        if flipped != 0 {
            thor_board[move_0 as usize] = thor_side_to_move;
            thor_side_to_move =
                0 as i32 + 2 as i32 - thor_side_to_move
        } else {
            thor_side_to_move =
                0 as i32 + 2 as i32 - thor_side_to_move;
            flipped =
                any_flips(move_0, thor_side_to_move,
                          0 as i32 + 2 as i32 -
                              thor_side_to_move);
            if flipped != 0 {
                thor_board[move_0 as usize] = thor_side_to_move;
                thor_side_to_move =
                    0 as i32 + 2 as i32 - thor_side_to_move
            } else { return 0 as i32 }
        }
        i += 1
    }
    return 1 as i32;
}

/*
  PREPARE_GAME
  Performs off-line analysis of GAME to speed up subsequent requests.
  The main result is that the number of black discs on the board after
  each of the moves is stored.
*/
pub unsafe fn prepare_game(mut game: *mut GameType) {
    let mut i: i32 = 0;
    let mut move_0: i32 = 0;
    let mut done: i32 = 0;
    let mut flipped: i32 = 0;
    let mut opening_match: i32 = 0;
    let mut moves_played: i32 = 0;
    let mut disc_count: [i32; 3] = [0; 3];
    let mut corner_descriptor: u32 = 0;
    let mut opening = 0 as *mut ThorOpeningNode;
    let mut child = 0 as *mut ThorOpeningNode;
    /* Play through the game and count the number of black discs
       at each stage. */
    clear_thor_board();
    disc_count[2] = 2 as i32;
    disc_count[0] =
        disc_count[2];
    thor_side_to_move = 0 as i32;
    corner_descriptor = 0 as i32 as u32;
    moves_played = 0 as i32;
    done = 0 as i32;
    loop  {
        /* Store the number of black discs. */
        (*game).black_disc_count[moves_played as usize] =
            disc_count[0] as i8;
        /* Make the move, update the board and disc count,
           and change the sign for white moves */
        move_0 = (*game).moves[moves_played as usize] as i32;
        flipped =
            count_flips(move_0, thor_side_to_move,
                        0 as i32 + 2 as i32 -
                            thor_side_to_move);
        if flipped != 0 {
            thor_board[move_0 as usize] = thor_side_to_move;
            disc_count[thor_side_to_move as usize] +=
                flipped + 1 as i32;
            disc_count[(0 as i32 + 2 as i32 -
                thor_side_to_move) as usize] -= flipped;
            if thor_side_to_move == 2 as i32 {
                (*game).moves[moves_played as usize] =
                    -((*game).moves[moves_played as usize] as i32) as
                        i8
            }
            thor_side_to_move =
                0 as i32 + 2 as i32 - thor_side_to_move;
            moves_played += 1
        } else {
            thor_side_to_move =
                0 as i32 + 2 as i32 - thor_side_to_move;
            flipped =
                count_flips(move_0, thor_side_to_move,
                            0 as i32 + 2 as i32 -
                                thor_side_to_move);
            if flipped != 0 {
                thor_board[move_0 as usize] = thor_side_to_move;
                disc_count[thor_side_to_move as usize] +=
                    flipped + 1 as i32;
                disc_count[(0 as i32 + 2 as i32 -
                    thor_side_to_move) as usize] -= flipped;
                if thor_side_to_move == 2 as i32 {
                    (*game).moves[moves_played as usize] =
                        -((*game).moves[moves_played as usize] as i32)
                            as i8
                }
                thor_side_to_move =
                    0 as i32 + 2 as i32 - thor_side_to_move;
                moves_played += 1
            } else { done = 1 as i32 }
        }
        /* Update the corner descriptor if necessary */
        if move_0 == 11 as i32 || move_0 == 18 as i32 ||
            move_0 == 81 as i32 || move_0 == 88 as i32 {
            corner_descriptor |=
                get_corner_mask(thor_board[11],
                                thor_board[81],
                                thor_board[18],
                                thor_board[88])
        }
        if !(done == 0 && moves_played < 60 as i32) { break ; }
    }
    (*game).black_disc_count[moves_played as usize] =
        disc_count[0] as i8;
    (*game).move_count = moves_played as i16;
    i = moves_played + 1 as i32;
    while i <= 60 as i32 {
        (*game).black_disc_count[i as usize] =
            -(1 as i32) as i8;
        i += 1
    }
    /* Find the longest opening which coincides with the game */
    opening = root_node;
    i = 0 as i32;
    opening_match = 1 as i32;
    while opening_match != 0 {
        move_0 = (*opening).child_move as i32;
        child = (*opening).child_node;
        while !child.is_null() &&
            move_0 != abs((*game).moves[i as usize] as i32) {
            move_0 = (*child).sibling_move as i32;
            child = (*child).sibling_node
        }
        if child.is_null() {
            opening_match = 0 as i32
        } else { opening = child; i += 1 }
    }
    (*game).opening = opening;
    /* Initialize the shape state */
    (*game).shape_lo =
        ((3 as i32) << 27 as i32) as u32;
    (*game).shape_hi =
        ((3 as i32) << 3 as i32) as u32;
    (*game).shape_state_hi = 0 as i32 as i16;
    (*game).shape_state_lo = 0 as i32 as i16;
    /* Store the corner descriptor */
    (*game).corner_descriptor = corner_descriptor;
}

/*
  INIT_THOR_HASH
  Computes hash codes for each of the 6561 configurations of the 8 different
  rows. A special feature of the codes is the relation

     hash[flip[pattern]] == reverse[hash[pattern]]

  which speeds up the computation of the hash functions.
*/
pub unsafe fn init_thor_hash() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut row: [i32; 10] = [0; 10];
    let mut flip_row: [i32; 6561] = [0; 6561];
    let mut buffer: [i32; 6561] = [0; 6561];
    i = 0 as i32;
    while i < 8 as i32 { row[i as usize] = 0 as i32; i += 1 }
    i = 0 as i32;
    while i < 6561 as i32 {
        flip_row[i as usize] = 0 as i32;
        j = 0 as i32;
        while j < 8 as i32 {
            flip_row[i as usize] +=
                row[j as usize] * pow3[(7 as i32 - j) as usize];
            j += 1
        }
        /* Next configuration */
        j = 0 as i32;
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
    i = 0 as i32;
    while i < 8 as i32 {
        j = 0 as i32;
        while j < 6561 as i32 {
            buffer[j as usize] = abs(my_random() as i32);
            j += 1
        }
        j = 0 as i32;
        while j < 6561 as i32 {
            primary_hash[i as usize][j as usize] =
                buffer[j as usize] as u32 &
                    0xffff0000 as u32 |
                    bit_reverse_32(buffer[flip_row[j as usize] as usize] as
                        u32) &
                        0xffff as i32 as u32;
            j += 1
        }
        j = 0 as i32;
        while j < 6561 as i32 {
            buffer[j as usize] = abs(my_random() as i32);
            j += 1
        }
        j = 0 as i32;
        while j < 6561 as i32 {
            secondary_hash[i as usize][j as usize] =
                buffer[j as usize] as u32 &
                    0xffff0000 as u32 |
                    bit_reverse_32(buffer[flip_row[j as usize] as usize] as
                        u32) &
                        0xffff as i32 as u32;
            j += 1
        }
        i += 1
    };
}
/*
  NEW_THOR_OPENING_NODE
  Creates and initializes a new node for use in the opening tree.
*/
pub unsafe fn new_thor_opening_node<FE: FrontEnd>(parent: *mut ThorOpeningNode)
                                                  -> *mut ThorOpeningNode {
    let mut node = 0 as *mut ThorOpeningNode;
    node =
        safe_malloc::<FE>(::std::mem::size_of::<ThorOpeningNode>() as u64)
            as *mut ThorOpeningNode;
    (*node).child_move = 0 as i32 as i8;
    (*node).sibling_move = 0 as i32 as i8;
    (*node).child_node = 0 as *mut ThorOpeningNode_;
    (*node).sibling_node = 0 as *mut ThorOpeningNode_;
    (*node).parent_node = parent;
    return node;
}

/*
  BUILD_THOR_OPENING_TREE
  Builds the opening tree from the statically computed
  structure THOR_OPENING_LIST (see thorop.c).
*/
pub unsafe fn build_thor_opening_tree<FE: FrontEnd>() {
    let mut thor_move_list: [i8; 61] = [0; 61];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut move_0: i32 = 0;
    let mut branch_depth: i32 = 0;
    let mut end_depth: i32 = 0;
    let mut flipped: i32 = 0;
    let mut hash1: u32 = 0;
    let mut hash2: u32 = 0;
    let mut parent = 0 as *mut ThorOpeningNode;
    let mut last_child = 0 as *mut ThorOpeningNode;
    let mut new_child = 0 as *mut ThorOpeningNode;
    let mut node_list: [*mut ThorOpeningNode; 61] =
        [0 as *mut ThorOpeningNode; 61];
    /* Create the root node and compute its hash value */
    root_node = new_thor_opening_node::<FE>(0 as *mut ThorOpeningNode);
    clear_thor_board();
    compute_thor_patterns(thor_board.as_mut_ptr());
    compute_partial_hash(&mut hash1, &mut hash2);
    (*root_node).hash1 = hash1;
    (*root_node).hash2 = hash2;
    node_list[0] = root_node;
    /* Add each of the openings to the tree */
    i = 0 as i32;
    while i < 741 as i32 {
        branch_depth = THOR_OPENING_LIST[i as usize].first_unique;
        end_depth =
            (branch_depth as
                u64).wrapping_add(FE::strlen(THOR_OPENING_LIST[i as
                usize].move_str).wrapping_div(2
                as
                i32
                as
                u64))
                as i32;
        j = 0 as i32;
        while j < end_depth - branch_depth {
            thor_move_list[(branch_depth + j) as usize] =
                (10 as i32 *
                    (*THOR_OPENING_LIST[i as
                        usize].move_str.offset((2 as
                        i32
                        * j +
                        1 as
                            i32)
                        as
                        isize)
                        as i32 - '0' as i32) +
                    (*THOR_OPENING_LIST[i as
                        usize].move_str.offset((2 as
                        i32
                        * j)
                        as
                        isize)
                        as i32 - 'a' as i32 + 1 as i32)) as
                    i8;
            j += 1
        }
        /* Play through the moves common with the previous line
           and the first deviation */
        clear_thor_board();
        thor_side_to_move = 0 as i32;
        j = 0 as i32;
        while j <= branch_depth {
            move_0 = thor_move_list[j as usize] as i32;
            flipped =
                any_flips(move_0, thor_side_to_move,
                          0 as i32 + 2 as i32 -
                              thor_side_to_move);
            if flipped != 0 {
                thor_board[move_0 as usize] = thor_side_to_move;
                thor_side_to_move =
                    0 as i32 + 2 as i32 - thor_side_to_move
            } else {
                thor_side_to_move =
                    0 as i32 + 2 as i32 - thor_side_to_move;
                flipped =
                    any_flips(move_0, thor_side_to_move,
                              0 as i32 + 2 as i32 -
                                  thor_side_to_move);
                if flipped != 0 {
                    thor_board[move_0 as usize] = thor_side_to_move;
                    thor_side_to_move =
                        0 as i32 + 2 as i32 -
                            thor_side_to_move
                } else {
                    FE::thordb_report_flipped_0_first();
                }
            }
            j += 1
        }
        /* Create the branch from the previous node */
        parent = node_list[branch_depth as usize];
        new_child = new_thor_opening_node::<FE>(parent);
        compute_thor_patterns(thor_board.as_mut_ptr());
        compute_partial_hash(&mut hash1, &mut hash2);
        (*new_child).hash1 = hash1;
        (*new_child).hash2 = hash2;
        if (*parent).child_node.is_null() {
            (*parent).child_node = new_child;
            (*parent).child_move = thor_move_list[branch_depth as usize]
        } else {
            last_child = (*parent).child_node;
            while !(*last_child).sibling_node.is_null() {
                last_child = (*last_child).sibling_node
            }
            (*last_child).sibling_node = new_child;
            (*last_child).sibling_move = thor_move_list[branch_depth as usize]
        }
        node_list[(branch_depth + 1 as i32) as usize] = new_child;
        /* Play through the rest of the moves and create new nodes for each
           of the resulting positions */
        j = branch_depth + 1 as i32;
        while j < end_depth {
            move_0 = thor_move_list[j as usize] as i32;
            flipped =
                any_flips(move_0, thor_side_to_move,
                          0 as i32 + 2 as i32 -
                              thor_side_to_move);
            if flipped != 0 {
                thor_board[move_0 as usize] = thor_side_to_move;
                thor_side_to_move =
                    0 as i32 + 2 as i32 - thor_side_to_move
            } else {
                thor_side_to_move =
                    0 as i32 + 2 as i32 - thor_side_to_move;
                flipped =
                    any_flips(move_0, thor_side_to_move,
                              0 as i32 + 2 as i32 -
                                  thor_side_to_move);
                if flipped != 0 {
                    thor_board[move_0 as usize] = thor_side_to_move;
                    thor_side_to_move =
                        0 as i32 + 2 as i32 -
                            thor_side_to_move
                } else {
                    FE::thordb_report_flipped_0_second();
                }
            }
            parent = new_child;
            new_child = new_thor_opening_node::<FE>(parent);
            compute_thor_patterns(thor_board.as_mut_ptr());
            compute_partial_hash(&mut hash1, &mut hash2);
            (*new_child).hash1 = hash1;
            (*new_child).hash2 = hash2;
            (*parent).child_node = new_child;
            (*parent).child_move = thor_move_list[j as usize];
            node_list[(j + 1 as i32) as usize] = new_child;
            j += 1
        }
        (*new_child).frequency = THOR_OPENING_LIST[i as usize].frequency;
        i += 1
    }
    /* Calculate opening frequencies also for interior nodes */
    calculate_opening_frequency(root_node);
}


/*
  INIT_THOR_DATABASE
  Performs the basic initializations of the Thor database interface.
  Before any operation on the database may be performed, this function
  must be called.
*/

pub unsafe fn init_thor_database<FE: FrontEnd>() {
    let mut i: i32 = 0; /* "infinity" */
    thor_game_count = 0 as i32;
    thor_database_count = 0 as i32;
    thor_search.match_list = 0 as *mut *mut GameType;
    thor_search.allocation = 0 as i32;
    thor_search.match_count = 0 as i32;
    thor_search.black_wins = 0 as i32;
    thor_search.draws = 0 as i32;
    thor_search.white_wins = 0 as i32;
    thor_search.median_black_score = 0 as i32;
    thor_search.average_black_score = 0.0f64;
    thor_sort_criteria_count = 5 as i32;
    i = 0 as i32;
    while i < 5 as i32 {
        thor_sort_order[i as usize] = default_sort_order[i as usize];
        i += 1
    }
    database_head = 0 as *mut DatabaseType;
    players.name_buffer = 0 as *mut i8;
    players.player_list = 0 as *mut PlayerType;
    players.count = 0 as i32;
    tournaments.name_buffer = 0 as *mut i8;
    tournaments.tournament_list = 0 as *mut TournamentType;
    tournaments.count = 0 as i32;
    thor_games_sorted = 0 as i32;
    thor_games_filtered = 0 as i32;
    init_move_masks();
    init_symmetry_maps::<FE>();
    init_thor_hash();
    prepare_thor_board();
    build_thor_opening_tree::<FE>();
    filter.game_categories =
        1 as i32 | 2 as i32 | 4 as i32;
    filter.player_filter = EITHER_SELECTED_FILTER;
    filter.first_year = -((1 as i32) << 25 as i32);
    filter.last_year = (1 as i32) << 25 as i32;
}

/*
  GET_THOR_GAME_MOVES
  Returns the moves, and number of moves, in the INDEXth game
  in the list of matching games generated by DATABASE_SEARCH.
  The game will not necessarily have the same rotational symmetry
  as the position searched for with database_search(); this depends
  on what rotation that gave a match.
*/

pub unsafe fn get_thor_game_moves(index: i32,
                                  move_count: *mut i32,
                                  moves: *mut i32) {
    let mut i: i32 = 0;
    let mut game = 0 as *mut GameType;
    if index < 0 as i32 || index >= thor_search.match_count {
        /* Bad index, so fill with empty values */
        *move_count = 0 as i32
    } else {
        game = *thor_search.match_list.offset(index as isize);
        *move_count = (*game).move_count as i32;
        match (*game).matching_symmetry as i32 {
            0 | 2 | 5 | 7 => {
                /* Symmetries that preserve the initial position. */
                i = 0 as i32;
                while i < (*game).move_count as i32 {
                    *moves.offset(i as isize) =
                        *symmetry_map[(*game).matching_symmetry as
                            usize].offset(abs((*game).moves[i as
                            usize]
                            as
                            i32)
                            as isize);
                    i += 1
                }
            }
            _ => {
                /* Symmetries that reverse the initial position. */
                i = 0 as i32;
                while i < (*game).move_count as i32 {
                    *moves.offset(i as isize) =
                        abs((*game).moves[i as usize] as i32);
                    i += 1
                }
            }
        }
    };
}
/*
  GET_THOR_GAME_MOVE
  Returns the MOVE_NUMBERth move in the INDEXth game
  in the list of matching games generated by DATABASE_SEARCH.
*/

pub unsafe fn get_thor_game_move(index: i32,
                                 move_number: i32)
                                 -> i32 {
    if index < 0 as i32 || index >= thor_search.match_count {
        return -(1 as i32)
    } else {
        let game = *thor_search.match_list.offset(index as isize);
        if move_number < 0 as i32 ||
            move_number >= (*game).move_count as i32 {
            return -(1 as i32)
        } else {
            return *symmetry_map[(*game).matching_symmetry as
                usize].offset(abs((*game).moves[move_number
                as
                usize]
                as i32) as
                isize)
        }
    };
}

/*
  POSITION_MATCH
  Returns TRUE if the position after MOVE_COUNT moves of GAME, with
  SIDE_TO_MOVE being the player to move, matches the hash codes
  IN_HASH1 and IN_HASH2, otherwise FALSE.
*/
pub unsafe fn position_match(mut game: *mut GameType,
                         move_count: i32,
                         side_to_move: i32,
                         shape_lo: *mut u32,
                         shape_hi: *mut u32,
                         corner_mask: u32,
                         in_hash1: u32,
                         in_hash2: u32)
                         -> i32 {
    let mut i: i32 = 0;
    let mut shape_match: i32 = 0;
    let mut primary_hit_mask: i32 = 0;
    let mut secondary_hit_mask: i32 = 0;
    /* Verify that the number of moves and the side-to-move status
       are correct */
    if move_count >= (*game).move_count as i32 {
        if move_count > (*game).move_count as i32 {
            /* Too many moves! */
            return 0 as i32
        }
        /* No side-to-move status to check if the game is over */
    } else if side_to_move == 0 as i32 {
        /* Black to move */
        if ((*game).moves[move_count as usize] as i32) <
            0 as i32 {
            /* White to move in the game */
            return 0 as i32
        }
    } else if (*game).moves[move_count as usize] as i32 >
        0 as i32 {
        /* White to move */
        /* Black to move in the game */
        return 0 as i32
    }
    /* Check if the opening information suffices to
       determine if the position matches or not. */
    if (*(*game).opening).current_match == 1 as i32 {
        (*game).matching_symmetry =
            (*(*game).opening).matching_symmetry as i16;
        return 1 as i32
    } else {
        if (*(*game).opening).current_match == 2 as i32 {
            return 0 as i32
        }
    }
    /* Check if the lower 32 bits of the shape state coincide */
    if ((*game).shape_state_lo as i32) < move_count {
        i = (*game).shape_state_lo as i32;
        while i < move_count {
            (*game).shape_lo |=
                move_mask_lo[abs((*game).moves[i as usize] as i32) as
                    usize];
            i += 1
        }
        (*game).shape_state_lo = move_count as i16
    } else if (*game).shape_state_lo as i32 > move_count {
        i = (*game).shape_state_lo as i32 - 1 as i32;
        while i >= move_count {
            (*game).shape_lo &=
                !move_mask_lo[abs((*game).moves[i as usize] as i32) as
                    usize];
            i -= 1
        }
        (*game).shape_state_lo = move_count as i16
    }
    shape_match = 0 as i32;
    i = 0 as i32;
    while i < 8 as i32 {
        shape_match |=
            ((*game).shape_lo == *shape_lo.offset(i as isize)) as i32;
        i += 1
    }
    if shape_match == 0 { return 0 as i32 }
    /* Check if the upper 32 bits of the shape state coincide */
    if ((*game).shape_state_hi as i32) < move_count {
        i = (*game).shape_state_hi as i32;
        while i < move_count {
            (*game).shape_hi |=
                move_mask_hi[abs((*game).moves[i as usize] as i32) as
                    usize];
            i += 1
        }
        (*game).shape_state_hi = move_count as i16
    } else if (*game).shape_state_hi as i32 > move_count {
        i = (*game).shape_state_hi as i32 - 1 as i32;
        while i >= move_count {
            (*game).shape_hi &=
                !move_mask_hi[abs((*game).moves[i as usize] as i32) as
                    usize];
            i -= 1
        }
        (*game).shape_state_hi = move_count as i16
    }
    shape_match = 0 as i32;
    i = 0 as i32;
    while i < 8 as i32 {
        shape_match |=
            ((*game).shape_hi == *shape_hi.offset(i as isize)) as i32;
        i += 1
    }
    if shape_match == 0 { return 0 as i32 }
    /* Check if the corner mask is compatible with that of the game */
    if corner_mask & !(*game).corner_descriptor != 0 {
        return 0 as i32
    }
    /* Otherwise play through the moves of the game until the
       number of discs is correct and check if the hash
       functions match the given hash values for at least one
       rotation (common to the two hash functions). */
    if play_through_game(game, move_count) != 0 {
        compute_thor_patterns(thor_board.as_mut_ptr());
        primary_hit_mask = primary_hash_lookup(in_hash1);
        if primary_hit_mask != 0 {
            secondary_hit_mask = secondary_hash_lookup(in_hash2);
            if primary_hit_mask & secondary_hit_mask != 0 {
                i = 0 as i32;
                while i < 8 as i32 {
                    if primary_hit_mask & secondary_hit_mask &
                        (1 as i32) << i != 0 {
                        (*game).matching_symmetry = i as i16;
                        return 1 as i32
                    }
                    i += 1
                }
            }
        }
    }
    return 0 as i32;
}

/*
  TOURNAMENT_LEX_ORDER
  Returns the index into the lexicographical order of the
  INDEXth tournament if available, otherwise the last
  index + 1.
*/
pub unsafe fn tournament_lex_order(index: i32)
                                   -> i32 {
    if index < 0 as i32 || index >= tournaments.count {
        return tournaments.count
    } else {
        return (*tournaments.tournament_list.offset(index as isize)).lex_order
    };
}


/*
  THOR_COMPARE
  Compares two games from a list of pointers to games.
  Only to be called by QSORT. A full comparison is
  performed using the priority order from THOR_SORT_ORDER.
*/

pub unsafe fn thor_compare(g1: *const c_void,
                                  g2: *const c_void)
                                  -> i32 {
    let mut i: i32 = 0;
    let mut result: i32 = 0;
    let game1 = *(g1 as *mut *mut GameType);
    let game2 = *(g2 as *mut *mut GameType);
    i = 0 as i32;
    while i < thor_sort_criteria_count {
        match thor_sort_order[i as usize] {
            1 => {
                result =
                    (*(*game2).database).prolog.origin_year -
                        (*(*game1).database).prolog.origin_year
            }
            2 => {
                result =
                    player_lex_order((*game1).black_no as i32) -
                        player_lex_order((*game2).black_no as i32)
            }
            3 => {
                result =
                    player_lex_order((*game1).white_no as i32) -
                        player_lex_order((*game2).white_no as i32)
            }
            4 => {
                result =
                    tournament_lex_order((*game1).tournament_no as
                        i32) -
                        tournament_lex_order((*game2).tournament_no as
                            i32)
            }
            5 => {
                result =
                    (*game1).actual_black_score as i32 -
                        (*game2).actual_black_score as i32
            }
            6 => {
                result =
                    (*game2).actual_black_score as i32 -
                        (*game1).actual_black_score as i32
            }
            0 | _ => {
                /* Really can't happen */
                result =
                    (*(*game1).database).prolog.origin_year -
                        (*(*game2).database).prolog.origin_year
            }
        }
        if result != 0 as i32 { return result }
        i += 1
    }
    /* If control reaches this point, the two games couldn't be
       distinguished by the current search criteria. */
    return 0 as i32;
}


/*
  CHOOSE_THOR_OPENING_MOVE
  Computes frequencies for all moves from the given position,
  display these and chooses one if from a distribution skewed
  towards common moves. (If no moves are found, PASS is returned.)
*/

pub unsafe fn choose_thor_opening_move<FE:FrontEnd>(in_board:
                                       *mut i32,
                                       side_to_move:
                                       i32,
                                       echo: i32)
                                       -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut temp_symm: i32 = 0;
    let mut pos: i32 = 0;
    let mut disc_count: i32 = 0;
    let mut freq_sum: i32 = 0;
    let mut acc_freq_sum: i32 = 0;
    let mut random_move: i32 = 0;
    let mut random_value: i32 = 0;
    let mut match_count: i32 = 0;
    let mut symmetries: [i32; 8] = [0; 8];
    let mut freq_count: [i32; 100] = [0; 100];
    let mut primary_hash_0: [u32; 8] = [0; 8];
    let mut secondary_hash_0: [u32; 8] = [0; 8];
    let mut move_list: [C2RustUnnamed; 64] =
        [C2RustUnnamed{move_0: 0, frequency: 0,}; 64];
    let mut temp = C2RustUnnamed{move_0: 0, frequency: 0,};
    disc_count = 0 as i32;
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        pos = 10 as i32 * i + 1 as i32;
        while j <= 8 as i32 {
            freq_count[pos as usize] = 0 as i32;
            if *in_board.offset(pos as isize) != 1 as i32 {
                disc_count += 1
            }
            j += 1;
            pos += 1
        }
        i += 1
    }
    /* Check that the parity of the board coincides with standard
       Othello - if this is not the case, the Thor opening lines are useless
       as they don't contain any passes. */
    if side_to_move == 0 as i32 &&
        disc_count % 2 as i32 == 1 as i32 {
        return -(1 as i32)
    }
    if side_to_move == 2 as i32 &&
        disc_count % 2 as i32 == 0 as i32 {
        return -(1 as i32)
    }
    /* Create a random permutation of the symmetries to avoid the same
       symmetry always being chosen in e.g. the initial position */
    i = 0 as i32;
    while i < 8 as i32 { symmetries[i as usize] = i; i += 1 }
    i = 0 as i32;
    while i < 7 as i32 {
        j = i + abs(my_random() as i32) % (8 as i32 - i);
        temp_symm = symmetries[i as usize];
        symmetries[i as usize] = symmetries[j as usize];
        symmetries[j as usize] = temp_symm;
        i += 1
    }
    /* Calculate frequencies for all moves */
    compute_thor_patterns(in_board);
    compute_full_primary_hash(primary_hash_0.as_mut_ptr());
    compute_full_secondary_hash(secondary_hash_0.as_mut_ptr());
    recursive_frequency_count(root_node, freq_count.as_mut_ptr(),
                              0 as i32, disc_count - 4 as i32,
                              symmetries.as_mut_ptr(),
                              primary_hash_0.as_mut_ptr(),
                              secondary_hash_0.as_mut_ptr());
    freq_sum = 0 as i32;
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        pos = 10 as i32 * i + 1 as i32;
        while j <= 8 as i32 {
            freq_sum += freq_count[pos as usize];
            j += 1;
            pos += 1
        }
        i += 1
    }
    if freq_sum > 0 as i32 {
        /* Position found in Thor opening tree */
        /* Create a list of the moves chosen from the position and also
           randomly select one of them. Probability for each move is
           proportional to the frequency of that move being played here. */
        random_value = abs(my_random() as i32) % freq_sum;
        random_move = -(1 as i32);
        acc_freq_sum = 0 as i32;
        match_count = 0 as i32;
        i = 1 as i32;
        while i <= 8 as i32 {
            j = 1 as i32;
            pos = 10 as i32 * i + 1 as i32;
            while j <= 8 as i32 {
                if freq_count[pos as usize] > 0 as i32 {
                    move_list[match_count as usize].move_0 = pos;
                    move_list[match_count as usize].frequency =
                        freq_count[pos as usize];
                    match_count += 1;
                    if acc_freq_sum < random_value &&
                        acc_freq_sum + freq_count[pos as usize] >=
                            random_value {
                        random_move = pos
                    }
                    acc_freq_sum += freq_count[pos as usize]
                }
                j += 1;
                pos += 1
            }
            i += 1
        }
        /* Optionally display the database moves sorted on frequency */
        if echo != 0 {
            i = 0 as i32;
            while i < match_count {
                j = 0 as i32;
                while j < match_count - 1 as i32 {
                    if move_list[j as usize].frequency <
                        move_list[(j + 1 as i32) as
                            usize].frequency {
                        temp = move_list[j as usize];
                        move_list[j as usize] =
                            move_list[(j + 1 as i32) as usize];
                        move_list[(j + 1 as i32) as usize] = temp
                    }
                    j += 1
                }
                i += 1
            }
            FE::choose_thor_opening_move_report(freq_sum, match_count, &move_list)
        }
        return random_move
    }
    return -(1 as i32);
}

/*
  DATABASE_SEARCH
  Determines what positions in the Thor database match the position
  given by IN_BOARD with SIDE_TO_MOVE being the player whose turn it is.
*/

pub unsafe fn database_search<FE: FrontEnd>(in_board: *mut i32,
                                            side_to_move: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut index: i32 = 0;
    let mut pos: i32 = 0;
    let mut sum: i32 = 0;
    let mut move_count: i32 = 0;
    let mut symmetry: i32 = 0;
    let mut next_move: i32 = 0;
    let mut disc_count: [i32; 3] = [0; 3];
    let mut frequency: [i32; 65] = [0; 65];
    let mut cumulative: [i32; 65] = [0; 65];
    let mut target_hash1: u32 = 0;
    let mut target_hash2: u32 = 0;
    let mut corner_mask: u32 = 0;
    let mut shape_lo: [u32; 8] = [0; 8];
    let mut shape_hi: [u32; 8] = [0; 8];
    let mut current_db = 0 as *mut DatabaseType;
    let mut game = 0 as *mut GameType;
    /* We need a player and a tournament database. */
    if players.count == 0 as i32 ||
        tournaments.count == 0 as i32 {
        thor_search.match_count = 0 as i32;
        return
    }
    /* Make sure there's memory allocated if all positions
       in all databases match the position */
    if thor_search.allocation == 0 as i32 {
        thor_search.match_list =
            safe_malloc::<FE>((thor_game_count as
                u64).wrapping_mul(::std::mem::size_of::<*mut GameType>()
                as
                u64))
                as *mut *mut GameType;
        thor_search.allocation = thor_game_count
    } else if thor_search.allocation < thor_game_count {
        FE::free(thor_search.match_list as *mut c_void);
        thor_search.match_list =
            safe_malloc::<FE>((thor_game_count as
                u64).wrapping_mul(::std::mem::size_of::<*mut GameType>()
                as
                u64))
                as *mut *mut GameType;
        thor_search.allocation = thor_game_count
    }
    /* If necessary, filter all games in the database */
    if thor_games_filtered == 0 {
        filter_all_databases();
        thor_games_filtered = 1 as i32
    }
    /* If necessary, sort all games in the database */
    if thor_games_sorted == 0 {
        current_db = database_head;
        i = 0 as i32;
        while !current_db.is_null() {
            j = 0 as i32;
            while j < (*current_db).count {
                let ref mut fresh5 =
                    *thor_search.match_list.offset(i as isize);
                *fresh5 =
                    &mut *(*current_db).games.offset(j as isize) as
                        *mut GameType;
                i += 1;
                j += 1
            }
            current_db = (*current_db).next
        }
        FE::sort_thor_games(thor_game_count);
        j = 0 as i32;
        while j < thor_game_count {
            (**thor_search.match_list.offset(j as isize)).sort_order = j;
            j += 1
        }
        thor_games_sorted = 1 as i32
    }
    /* Determine disc count, hash codes, patterns and opening
       for the position */
    disc_count[2] = 0 as i32;
    disc_count[0] =
        disc_count[2];
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        pos = 10 as i32 * i + 1 as i32;
        while j <= 8 as i32 {
            if *in_board.offset(pos as isize) == 0 as i32 {
                disc_count[0] += 1
            } else if *in_board.offset(pos as isize) == 2 as i32 {
                disc_count[2] += 1
            }
            j += 1;
            pos += 1
        }
        i += 1
    }
    move_count =
        disc_count[0] +
            disc_count[2] - 4 as i32;
    compute_thor_patterns(in_board);
    compute_partial_hash(&mut target_hash1, &mut target_hash2);
    opening_scan(move_count);
    /* Determine the shape masks */
    i = 0 as i32;
    while i < 8 as i32 {
        shape_lo[i as usize] = 0 as i32 as u32;
        shape_hi[i as usize] = 0 as i32 as u32;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        j = 0 as i32;
        pos = 10 as i32 * i + 11 as i32;
        while j < 8 as i32 {
            if *in_board.offset(pos as isize) != 1 as i32 {
                index = 8 as i32 * i + j;
                if index < 32 as i32 {
                    shape_lo[0] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[0] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * i + (7 as i32 - j);
                if index < 32 as i32 {
                    shape_lo[1] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[1] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * j + i;
                if index < 32 as i32 {
                    shape_lo[2] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[2] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * j + (7 as i32 - i);
                if index < 32 as i32 {
                    shape_lo[3] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[3] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * (7 as i32 - i) + j;
                if index < 32 as i32 {
                    shape_lo[4] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[4] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index =
                    8 as i32 * (7 as i32 - i) +
                        (7 as i32 - j);
                if index < 32 as i32 {
                    shape_lo[5] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[5] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * (7 as i32 - j) + i;
                if index < 32 as i32 {
                    shape_lo[6] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[6] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index =
                    8 as i32 * (7 as i32 - j) +
                        (7 as i32 - i);
                if index < 32 as i32 {
                    shape_lo[7] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[7] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
            }
            j += 1;
            pos += 1
        }
        i += 1
    }
    /* Get the corner mask */
    corner_mask =
        get_corner_mask(*in_board.offset(11),
                        *in_board.offset(81),
                        *in_board.offset(18),
                        *in_board.offset(88));
    /* Query the database about all positions in all databases.
       Only games which pass the currently applied filter are scanned.
       Also compute the frequency table and the next move table.
       To speed up sorting the games, the match table is first filled
       with NULLs and when a matching game is found, a pointer to it is
       inserted at a position determined by the field SORT_ORDER
       in the game. As this index is unique, no over-write
       can occur. */
    thor_search.match_count = 0 as i32;
    i = 0 as i32;
    while i < thor_game_count {
        let ref mut fresh6 = *thor_search.match_list.offset(i as isize);
        *fresh6 = 0 as *mut GameType;
        i += 1
    }
    i = 0 as i32;
    while i <= 64 as i32 {
        frequency[i as usize] = 0 as i32;
        i += 1
    }
    i = 0 as i32;
    while i < 100 as i32 {
        thor_search.next_move_frequency[i as usize] = 0 as i32;
        thor_search.next_move_score[i as usize] = 0.0f64;
        i += 1
    }
    current_db = database_head;
    while !current_db.is_null() {
        i = 0 as i32;
        while i < (*current_db).count {
            game =
                &mut *(*current_db).games.offset(i as isize) as *mut GameType;
            if (*game).passes_filter != 0 {
                if disc_count[0] ==
                    (*game).black_disc_count[move_count as usize] as
                        i32 {
                    if position_match(game, move_count, side_to_move,
                                      shape_lo.as_mut_ptr(),
                                      shape_hi.as_mut_ptr(), corner_mask,
                                      target_hash1, target_hash2) != 0 {
                        let ref mut fresh7 =
                            *thor_search.match_list.offset((*game).sort_order
                                as isize);
                        *fresh7 = game;
                        symmetry = (*game).matching_symmetry as i32;
                        if move_count < (*game).move_count as i32 {
                            next_move =
                                *symmetry_map[symmetry as
                                    usize].offset(abs((*game).moves[move_count
                                    as
                                    usize]
                                    as
                                    i32)
                                    as isize);
                            thor_search.next_move_frequency[next_move as
                                usize] += 1;
                            if (*game).actual_black_score as i32 ==
                                32 as i32 {
                                thor_search.next_move_score[next_move as
                                    usize] +=
                                    0.5f64
                            } else if (*game).actual_black_score as
                                i32 > 32 as i32 {
                                if side_to_move == 0 as i32 {
                                    thor_search.next_move_score[next_move as
                                        usize] +=
                                        1.0f64
                                }
                            } else if side_to_move == 2 as i32 {
                                thor_search.next_move_score[next_move as
                                    usize] +=
                                    1.0f64
                            }
                        }
                        frequency[(*game).actual_black_score as usize] += 1;
                        thor_search.match_count += 1
                    }
                }
            }
            i += 1
        }
        current_db = (*current_db).next
    }
    /* Remove the NULLs from the list of matching games if there are any.
       This gives a sorted list. */
    if thor_search.match_count > 0 as i32 &&
        thor_search.match_count < thor_game_count {
        i = 0 as i32;
        j = 0 as i32;
        while i < thor_search.match_count {
            if !(*thor_search.match_list.offset(j as isize)).is_null() {
                let ref mut fresh8 =
                    *thor_search.match_list.offset(i as isize);
                *fresh8 = *thor_search.match_list.offset(j as isize);
                i += 1
            }
            j += 1
        }
    }
    /* Count the number of black wins, draws and white wins.
       Also determine the average score. */
    sum = 0 as i32;
    i = 0 as i32;
    thor_search.white_wins = 0 as i32;
    while i <= 31 as i32 {
        thor_search.white_wins += frequency[i as usize];
        sum += i * frequency[i as usize];
        i += 1
    }
    thor_search.draws = frequency[32];
    sum += 32 as i32 * frequency[32];
    i = 33 as i32;
    thor_search.black_wins = 0 as i32;
    while i <= 64 as i32 {
        thor_search.black_wins += frequency[i as usize];
        sum += i * frequency[i as usize];
        i += 1
    }
    if thor_search.match_count == 0 as i32 {
        /* Average of 0 values is pointless */
        thor_search.average_black_score = 32.0f64
    } else {
        thor_search.average_black_score =
            sum as f64 / thor_search.match_count as f64
    }
    /* Determine the median score */
    if thor_search.match_count == 0 as i32 {
        /* ...and so is median of 0 values */
        thor_search.median_black_score = 32 as i32
    } else {
        cumulative[0] =
            frequency[0];
        i = 1 as i32;
        while i <= 64 as i32 {
            cumulative[i as usize] =
                cumulative[(i - 1 as i32) as usize] +
                    frequency[i as usize];
            i += 1
        }
        /* Median is average between first value for which cumulative
           frequency reaches 50% and first value for which it is
           strictly larger than 50%. This definition works regardless
           of the parity of the number of values.
           By definition of median, both loops terminate for indices <= 64. */
        i = 0 as i32;
        while 2 as i32 * cumulative[i as usize] <
            thor_search.match_count {
            i += 1
        }
        j = i;
        while 2 as i32 * cumulative[j as usize] <
            thor_search.match_count + 1 as i32 {
            j += 1
        }
        thor_search.median_black_score = (i + j) / 2 as i32
    };
}