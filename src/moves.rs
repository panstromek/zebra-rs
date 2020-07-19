use crate::src::cntflip::AnyFlips_compact;
use crate::src::globals::{board, piece_count};
use crate::src::stubs::{atoi, printf, scanf};
use crate::src::unflip::flip_stack;
use crate::src::hash::{hash_stored2, hash2, hash_stored1, hash1, hash_put_value2, hash_put_value1};
use crate::src::doflip::{DoFlips_no_hash, hash_update2, hash_update1, DoFlips_hash};
use crate::src::search::sorted_move_order;
/*
   File:           globals.h

   Created:        June 30, 1997

   Modified:       January 8, 2000

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Global state variables.
*/
/* The basic board type. One index for each position;
   a1=11, h1=18, a8=81, h8=88. */
/*
   File:              moves.c

   Created:           June 30, 1997

   Modified:          April 24, 2001

   Author:            Gunnar Andersson (gunnar@radagast.se)

   Contents:          The move generator.
*/
/* Global variables */

pub static mut disks_played: i32 = 0;

pub static mut move_count: [i32; 64] = [0; 64];

pub static mut move_list: [[i32; 64]; 64] = [[0; 64]; 64];

pub static mut first_flip_direction: [*mut i32; 100] =
    [0 as *const i32 as *mut i32; 100];

pub static mut flip_direction: [[i32; 16]; 100] = [[0; 16]; 100];
/* 100 * 9 used */

pub static mut first_flipped_disc: [*mut *mut i32; 100] =
    [0 as *const *mut i32 as *mut *mut i32; 100];

pub static mut flipped_disc: [[*mut i32; 8]; 100] =
    [[0 as *const i32 as *mut i32; 8]; 100];

pub static mut dir_mask: [i32; 100] =
    [0 as i32, 0 as i32, 0 as i32, 0 as i32,
     0 as i32, 0 as i32, 0 as i32, 0 as i32,
     0 as i32, 0 as i32, 0 as i32, 81 as i32,
     81 as i32, 87 as i32, 87 as i32,
     87 as i32, 87 as i32, 22 as i32,
     22 as i32, 0 as i32, 0 as i32, 81 as i32,
     81 as i32, 87 as i32, 87 as i32,
     87 as i32, 87 as i32, 22 as i32,
     22 as i32, 0 as i32, 0 as i32,
     121 as i32, 121 as i32, 255 as i32,
     255 as i32, 255 as i32, 255 as i32,
     182 as i32, 182 as i32, 0 as i32,
     0 as i32, 121 as i32, 121 as i32,
     255 as i32, 255 as i32, 255 as i32,
     255 as i32, 182 as i32, 182 as i32,
     0 as i32, 0 as i32, 121 as i32,
     121 as i32, 255 as i32, 255 as i32,
     255 as i32, 255 as i32, 182 as i32,
     182 as i32, 0 as i32, 0 as i32,
     121 as i32, 121 as i32, 255 as i32,
     255 as i32, 255 as i32, 255 as i32,
     182 as i32, 182 as i32, 0 as i32,
     0 as i32, 41 as i32, 41 as i32,
     171 as i32, 171 as i32, 171 as i32,
     171 as i32, 162 as i32, 162 as i32,
     0 as i32, 0 as i32, 41 as i32, 41 as i32,
     171 as i32, 171 as i32, 171 as i32,
     171 as i32, 162 as i32, 162 as i32,
     0 as i32, 0 as i32, 0 as i32, 0 as i32,
     0 as i32, 0 as i32, 0 as i32, 0 as i32,
     0 as i32, 0 as i32, 0 as i32];

pub static mut move_offset: [i32; 8] =
    [1 as i32, -(1 as i32), 9 as i32,
     -(9 as i32), 10 as i32, -(10 as i32),
     11 as i32, -(11 as i32)];
/* Local variables */
static mut flip_count: [i32; 65] = [0; 65];
static mut sweep_status: [i32; 64] = [0; 64];
/*
  INIT_MOVES
  Initialize the move generation subsystem.
*/

pub unsafe fn init_moves() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut pos: i32 = 0;
    let mut feasible: i32 = 0;
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        while j <= 8 as i32 {
            pos = 10 as i32 * i + j;
            k = 0 as i32;
            while k <= 8 as i32 {
                flip_direction[pos as usize][k as usize] = 0 as i32;
                k += 1
            }
            feasible = 0 as i32;
            k = 0 as i32;
            while k < 8 as i32 {
                if dir_mask[pos as usize] & (1 as i32) << k != 0 {
                    flip_direction[pos as usize][feasible as usize] =
                        move_offset[k as usize];
                    feasible += 1
                }
                k += 1
            }
            first_flip_direction[pos as usize] =
                &mut *(*flip_direction.as_mut_ptr().offset(pos as
                                                               isize)).as_mut_ptr().offset(0
                                                                                               as
                                                                                               i32
                                                                                               as
                                                                                               isize)
                    as *mut i32;
            j += 1
        }
        i += 1
    };
}
/*
   RESET_GENERATION
   Prepare for move generation at a given level in the tree.
*/
unsafe fn reset_generation(mut side_to_move: i32) {
    sweep_status[disks_played as usize] = 0 as i32;
}
/*
   GENERATE_SPECIFIC
*/

pub unsafe fn generate_specific(mut curr_move: i32,
                                           mut side_to_move: i32)
 -> i32 {
    return AnyFlips_compact(board.as_mut_ptr(), curr_move, side_to_move,
                            0 as i32 + 2 as i32 -
                                side_to_move);
}
/*
   GENERATE_MOVE
   side_to_move = the side to generate moves for

   Generate the next move in the ordering. This way not all moves possible
   in a position are generated, only those who need be considered.
*/

pub unsafe fn generate_move(mut side_to_move: i32)
 -> i32 {
    let mut move_0: i32 = 0;
    let mut move_index = 0 as i32;
    move_index = sweep_status[disks_played as usize];
    while move_index < 60 as i32 {
        move_0 =
            sorted_move_order[disks_played as usize][move_index as usize];
        if board[move_0 as usize] == 1 as i32 &&
               generate_specific(move_0, side_to_move) != 0 {
            sweep_status[disks_played as usize] =
                move_index + 1 as i32;
            return move_0
        } else { move_index += 1 }
    }
    sweep_status[disks_played as usize] = move_index;
    return -(1 as i32);
}
/*
   GENERATE_ALL
   Generates a list containing all the moves possible in a position.
*/

pub unsafe fn generate_all(mut side_to_move: i32) {
    let mut count: i32 = 0;
    let mut curr_move: i32 = 0;
    reset_generation(side_to_move);
    count = 0 as i32;
    curr_move = generate_move(side_to_move);
    while curr_move != -(1 as i32) {
        move_list[disks_played as usize][count as usize] = curr_move;
        count += 1;
        curr_move = generate_move(side_to_move)
    }
    move_list[disks_played as usize][count as usize] = -(1 as i32);
    move_count[disks_played as usize] = count;
}
/*
  COUNT_ALL
  Counts the number of moves for one player.
*/

pub unsafe fn count_all(mut side_to_move: i32,
                                   mut empty: i32) -> i32 {
    let mut move_0: i32 = 0;
    let mut move_index: i32 = 0;
    let mut mobility: i32 = 0;
    let mut found_empty: i32 = 0;
    mobility = 0 as i32;
    found_empty = 0 as i32;
    move_index = 0 as i32;
    while move_index < 60 as i32 {
        move_0 =
            sorted_move_order[disks_played as usize][move_index as usize];
        if board[move_0 as usize] == 1 as i32 {
            if generate_specific(move_0, side_to_move) != 0 { mobility += 1 }
            found_empty += 1;
            if found_empty == empty { return mobility }
        }
        move_index += 1
    }
    return mobility;
}
/*
   GAME_IN_PROGRESS
   Determines if any of the players has a valid move.
*/

pub unsafe fn game_in_progress() -> i32 {
    let mut black_count: i32 = 0;
    let mut white_count: i32 = 0;
    generate_all(0 as i32);
    black_count = move_count[disks_played as usize];
    generate_all(2 as i32);
    white_count = move_count[disks_played as usize];
    return (black_count > 0 as i32 || white_count > 0 as i32)
               as i32;
}
/*
   MAKE_MOVE
   side_to_move = the side that is making the move
   move = the position giving the move

   Makes the necessary changes on the board and updates the
   counters.
*/

pub unsafe fn make_move(mut side_to_move: i32,
                                   mut move_0: i32,
                                   mut update_hash: i32)
 -> i32 {
    let mut flipped: i32 = 0;
    let mut diff1: u32 = 0;
    let mut diff2: u32 = 0;
    if update_hash != 0 {
        flipped = DoFlips_hash(move_0, side_to_move);
        if flipped == 0 as i32 { return 0 as i32 }
        diff1 =
            hash_update1 ^
                hash_put_value1[side_to_move as usize][move_0 as usize];
        diff2 =
            hash_update2 ^
                hash_put_value2[side_to_move as usize][move_0 as usize];
        hash_stored1[disks_played as usize] = hash1;
        hash_stored2[disks_played as usize] = hash2;
        hash1 ^= diff1;
        hash2 ^= diff2
    } else {
        flipped = DoFlips_no_hash(move_0, side_to_move);
        if flipped == 0 as i32 { return 0 as i32 }
        hash_stored1[disks_played as usize] = hash1;
        hash_stored2[disks_played as usize] = hash2
    }
    flip_count[disks_played as usize] = flipped;
    board[move_0 as usize] = side_to_move;
    if side_to_move == 0 as i32 {
        piece_count[0 as i32 as
                        usize][(disks_played + 1 as i32) as usize] =
            piece_count[0 as i32 as usize][disks_played as usize] +
                flipped + 1 as i32;
        piece_count[2 as i32 as
                        usize][(disks_played + 1 as i32) as usize] =
            piece_count[2 as i32 as usize][disks_played as usize] -
                flipped
    } else {
        /* side_to_move == WHITESQ */
        piece_count[2 as i32 as
                        usize][(disks_played + 1 as i32) as usize] =
            piece_count[2 as i32 as usize][disks_played as usize] +
                flipped + 1 as i32;
        piece_count[0 as i32 as
                        usize][(disks_played + 1 as i32) as usize] =
            piece_count[0 as i32 as usize][disks_played as usize] -
                flipped
    }
    disks_played += 1;
    return flipped;
}
/*
   MAKE_MOVE_NO_HASH
   side_to_move = the side that is making the move
   move = the position giving the move

   Makes the necessary changes on the board. Note that the hash table
   is not updated - the move has to be unmade using UNMAKE_MOVE_NO_HASH().
*/

pub unsafe fn make_move_no_hash(mut side_to_move: i32,
                                           mut move_0: i32)
 -> i32 {
    let mut flipped: i32 = 0;
    flipped = DoFlips_no_hash(move_0, side_to_move);
    if flipped == 0 as i32 { return 0 as i32 }
    flip_count[disks_played as usize] = flipped;
    board[move_0 as usize] = side_to_move;
    if side_to_move == 0 as i32 {
        piece_count[0 as i32 as
                        usize][(disks_played + 1 as i32) as usize] =
            piece_count[0 as i32 as usize][disks_played as usize] +
                flipped + 1 as i32;
        piece_count[2 as i32 as
                        usize][(disks_played + 1 as i32) as usize] =
            piece_count[2 as i32 as usize][disks_played as usize] -
                flipped
    } else {
        /* side_to_move == WHITESQ */
        piece_count[2 as i32 as
                        usize][(disks_played + 1 as i32) as usize] =
            piece_count[2 as i32 as usize][disks_played as usize] +
                flipped + 1 as i32;
        piece_count[0 as i32 as
                        usize][(disks_played + 1 as i32) as usize] =
            piece_count[0 as i32 as usize][disks_played as usize] -
                flipped
    }
    disks_played += 1;
    return flipped;
}
/*
  UNMAKE_MOVE
  Takes back a move.
*/

pub unsafe fn unmake_move(mut side_to_move: i32,
                                     mut move_0: i32) {
    board[move_0 as usize] = 1 as i32;
    disks_played -= 1;
    hash1 = hash_stored1[disks_played as usize];
    hash2 = hash_stored2[disks_played as usize];
    let mut UndoFlips__flip_count = flip_count[disks_played as usize];
    let mut UndoFlips__oppcol =
        0 as i32 + 2 as i32 - side_to_move;
    if UndoFlips__flip_count & 1 as i32 != 0 {
        UndoFlips__flip_count -= 1;
        flip_stack = flip_stack.offset(-1);
        **flip_stack = UndoFlips__oppcol
    }
    while UndoFlips__flip_count != 0 {
        UndoFlips__flip_count -= 2 as i32;
        flip_stack = flip_stack.offset(-1);
        **flip_stack = UndoFlips__oppcol;
        flip_stack = flip_stack.offset(-1);
        **flip_stack = UndoFlips__oppcol
    };
}
/*
  UNMAKE_MOVE_NO_HASH
  Takes back a move. Only to be called when the move was made without
  updating hash table, preferrable through MAKE_MOVE_NO_HASH().
*/

pub unsafe fn unmake_move_no_hash(mut side_to_move: i32,
                                             mut move_0: i32) {
    board[move_0 as usize] = 1 as i32;
    disks_played -= 1;
    let mut UndoFlips__flip_count = flip_count[disks_played as usize];
    let mut UndoFlips__oppcol =
        0 as i32 + 2 as i32 - side_to_move;
    if UndoFlips__flip_count & 1 as i32 != 0 {
        UndoFlips__flip_count -= 1;
        flip_stack = flip_stack.offset(-1);
        **flip_stack = UndoFlips__oppcol
    }
    while UndoFlips__flip_count != 0 {
        UndoFlips__flip_count -= 2 as i32;
        flip_stack = flip_stack.offset(-1);
        **flip_stack = UndoFlips__oppcol;
        flip_stack = flip_stack.offset(-1);
        **flip_stack = UndoFlips__oppcol
    };
}
/*
   VALID_MOVE
   Determines if a move is legal.
*/

pub unsafe fn valid_move(mut move_0: i32,
                                    mut side_to_move: i32)
 -> i32 {
    let mut i: i32 = 0;
    let mut pos: i32 = 0;
    let mut count: i32 = 0;
    if move_0 < 11 as i32 || move_0 > 88 as i32 ||
           board[move_0 as usize] != 1 as i32 {
        return 0 as i32
    }
    i = 0 as i32;
    while i < 8 as i32 {
        if dir_mask[move_0 as usize] & (1 as i32) << i != 0 {
            pos = move_0 + move_offset[i as usize];
            count = 0 as i32;
            while board[pos as usize] ==
                      0 as i32 + 2 as i32 - side_to_move {
                pos += move_offset[i as usize];
                count += 1
            }
            if board[pos as usize] == side_to_move {
                if count >= 1 as i32 { return 1 as i32 }
            }
        }
        i += 1
    }
    return 0 as i32;
}
/*
   GET_MOVE
   Prompts the user to enter a move and checks if the move is legal.
*/

pub unsafe fn get_move(mut side_to_move: i32)
 -> i32 {
    let mut buffer: [i8; 255] = [0; 255];
    let mut ready = 0 as i32;
    let mut curr_move: i32 = 0;
    while ready == 0 {
        if side_to_move == 0 as i32 {
            printf(b"%s: \x00" as *const u8 as *const i8,
                   b"Black move\x00" as *const u8 as *const i8);
        } else {
            printf(b"%s: \x00" as *const u8 as *const i8,
                   b"White move\x00" as *const u8 as *const i8);
        }
        scanf(b"%s\x00" as *const u8 as *const i8,
              buffer.as_mut_ptr());
        curr_move = atoi(buffer.as_mut_ptr());
        ready = valid_move(curr_move, side_to_move);
        if ready == 0 {
            curr_move =
                buffer[0 as i32 as usize] as i32 - 'a' as i32
                    + 1 as i32 +
                    10 as i32 *
                        (buffer[1 as i32 as usize] as i32 -
                             '0' as i32);
            ready = valid_move(curr_move, side_to_move)
        }
    }
    return curr_move;
}
