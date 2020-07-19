use ::libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    /*
   cntflip.h

   Automatically created by ENDMACRO on Wed Mar 17 21:01:12 1999

   Last modified:   December 25, 1999
*/
    #[no_mangle]
    fn AnyFlips_compact(board_0: *mut libc::c_int, sqnum: libc::c_int,
                        color: libc::c_int, oppcol: libc::c_int)
     -> libc::c_int;
    /*
   doflip.h

   Automatically created by ENDMACRO on Fri Feb 26 20:29:42 1999

   Last modified:   October 25, 2005
*/
    #[no_mangle]
    static mut hash_update1: libc::c_uint;
    #[no_mangle]
    static mut hash_update2: libc::c_uint;
    #[no_mangle]
    fn DoFlips_hash(sqnum: libc::c_int, color: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn DoFlips_no_hash(sqnum: libc::c_int, color: libc::c_int) -> libc::c_int;
    /* piece_count[col][n] holds the number of disks of color col after
   n moves have been played. */
    #[no_mangle]
    static mut piece_count: [[libc::c_int; 64]; 3];
    /* Holds the current board position. Updated as the search progresses,
   but all updates must be reversed when the search stops. */
    #[no_mangle]
    static mut board: Board;
    /* The 64-bit hash key. */
    #[no_mangle]
    static mut hash1: libc::c_uint;
    #[no_mangle]
    static mut hash2: libc::c_uint;
    /* 64-bit hash masks used when a disc is played on the board;
   the relation
     hash_put_value?[][] == hash_value?[][] ^ hash_flip_color?
   is guaranteed to hold. */
    #[no_mangle]
    static mut hash_put_value1: [[libc::c_uint; 128]; 3];
    #[no_mangle]
    static mut hash_put_value2: [[libc::c_uint; 128]; 3];
    /* Stored 64-bit hash mask which hold the hash codes at different nodes
   in the search tree. */
    #[no_mangle]
    static mut hash_stored1: [libc::c_uint; 64];
    #[no_mangle]
    static mut hash_stored2: [libc::c_uint; 64];
    /* Move lists */
    #[no_mangle]
    static mut sorted_move_order: [[libc::c_int; 64]; 64];
    #[no_mangle]
    static mut flip_stack: *mut *mut libc::c_int;
}
/*
   File:           globals.h

   Created:        June 30, 1997

   Modified:       January 8, 2000

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       Global state variables.
*/
/* The basic board type. One index for each position;
   a1=11, h1=18, a8=81, h8=88. */
pub type Board = [libc::c_int; 128];
/*
   File:              moves.c

   Created:           June 30, 1997

   Modified:          April 24, 2001

   Author:            Gunnar Andersson (gunnar@radagast.se)

   Contents:          The move generator.
*/
/* Global variables */
#[no_mangle]
pub static mut disks_played: libc::c_int = 0;
#[no_mangle]
pub static mut move_count: [libc::c_int; 64] = [0; 64];
#[no_mangle]
pub static mut move_list: [[libc::c_int; 64]; 64] = [[0; 64]; 64];
#[no_mangle]
pub static mut first_flip_direction: [*mut libc::c_int; 100] =
    [0 as *const libc::c_int as *mut libc::c_int; 100];
#[no_mangle]
pub static mut flip_direction: [[libc::c_int; 16]; 100] = [[0; 16]; 100];
/* 100 * 9 used */
#[no_mangle]
pub static mut first_flipped_disc: [*mut *mut libc::c_int; 100] =
    [0 as *const *mut libc::c_int as *mut *mut libc::c_int; 100];
#[no_mangle]
pub static mut flipped_disc: [[*mut libc::c_int; 8]; 100] =
    [[0 as *const libc::c_int as *mut libc::c_int; 8]; 100];
#[no_mangle]
pub static mut dir_mask: [libc::c_int; 100] =
    [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 81 as libc::c_int,
     81 as libc::c_int, 87 as libc::c_int, 87 as libc::c_int,
     87 as libc::c_int, 87 as libc::c_int, 22 as libc::c_int,
     22 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 81 as libc::c_int,
     81 as libc::c_int, 87 as libc::c_int, 87 as libc::c_int,
     87 as libc::c_int, 87 as libc::c_int, 22 as libc::c_int,
     22 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     121 as libc::c_int, 121 as libc::c_int, 255 as libc::c_int,
     255 as libc::c_int, 255 as libc::c_int, 255 as libc::c_int,
     182 as libc::c_int, 182 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 121 as libc::c_int, 121 as libc::c_int,
     255 as libc::c_int, 255 as libc::c_int, 255 as libc::c_int,
     255 as libc::c_int, 182 as libc::c_int, 182 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 121 as libc::c_int,
     121 as libc::c_int, 255 as libc::c_int, 255 as libc::c_int,
     255 as libc::c_int, 255 as libc::c_int, 182 as libc::c_int,
     182 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     121 as libc::c_int, 121 as libc::c_int, 255 as libc::c_int,
     255 as libc::c_int, 255 as libc::c_int, 255 as libc::c_int,
     182 as libc::c_int, 182 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 41 as libc::c_int, 41 as libc::c_int,
     171 as libc::c_int, 171 as libc::c_int, 171 as libc::c_int,
     171 as libc::c_int, 162 as libc::c_int, 162 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 41 as libc::c_int, 41 as libc::c_int,
     171 as libc::c_int, 171 as libc::c_int, 171 as libc::c_int,
     171 as libc::c_int, 162 as libc::c_int, 162 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int];
#[no_mangle]
pub static mut move_offset: [libc::c_int; 8] =
    [1 as libc::c_int, -(1 as libc::c_int), 9 as libc::c_int,
     -(9 as libc::c_int), 10 as libc::c_int, -(10 as libc::c_int),
     11 as libc::c_int, -(11 as libc::c_int)];
/* Local variables */
static mut flip_count: [libc::c_int; 65] = [0; 65];
static mut sweep_status: [libc::c_int; 64] = [0; 64];
/*
  INIT_MOVES
  Initialize the move generation subsystem.
*/
#[no_mangle]
pub unsafe extern "C" fn init_moves() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut feasible: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= 8 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= 8 as libc::c_int {
            pos = 10 as libc::c_int * i + j;
            k = 0 as libc::c_int;
            while k <= 8 as libc::c_int {
                flip_direction[pos as usize][k as usize] = 0 as libc::c_int;
                k += 1
            }
            feasible = 0 as libc::c_int;
            k = 0 as libc::c_int;
            while k < 8 as libc::c_int {
                if dir_mask[pos as usize] & (1 as libc::c_int) << k != 0 {
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
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize)
                    as *mut libc::c_int;
            j += 1
        }
        i += 1
    };
}
/*
   RESET_GENERATION
   Prepare for move generation at a given level in the tree.
*/
unsafe extern "C" fn reset_generation(mut side_to_move: libc::c_int) {
    sweep_status[disks_played as usize] = 0 as libc::c_int;
}
/*
   GENERATE_SPECIFIC
*/
#[no_mangle]
pub unsafe extern "C" fn generate_specific(mut curr_move: libc::c_int,
                                           mut side_to_move: libc::c_int)
 -> libc::c_int {
    return AnyFlips_compact(board.as_mut_ptr(), curr_move, side_to_move,
                            0 as libc::c_int + 2 as libc::c_int -
                                side_to_move);
}
/*
   GENERATE_MOVE
   side_to_move = the side to generate moves for

   Generate the next move in the ordering. This way not all moves possible
   in a position are generated, only those who need be considered.
*/
#[no_mangle]
pub unsafe extern "C" fn generate_move(mut side_to_move: libc::c_int)
 -> libc::c_int {
    let mut move_0: libc::c_int = 0;
    let mut move_index = 0 as libc::c_int;
    move_index = sweep_status[disks_played as usize];
    while move_index < 60 as libc::c_int {
        move_0 =
            sorted_move_order[disks_played as usize][move_index as usize];
        if board[move_0 as usize] == 1 as libc::c_int &&
               generate_specific(move_0, side_to_move) != 0 {
            sweep_status[disks_played as usize] =
                move_index + 1 as libc::c_int;
            return move_0
        } else { move_index += 1 }
    }
    sweep_status[disks_played as usize] = move_index;
    return -(1 as libc::c_int);
}
/*
   GENERATE_ALL
   Generates a list containing all the moves possible in a position.
*/
#[no_mangle]
pub unsafe extern "C" fn generate_all(mut side_to_move: libc::c_int) {
    let mut count: libc::c_int = 0;
    let mut curr_move: libc::c_int = 0;
    reset_generation(side_to_move);
    count = 0 as libc::c_int;
    curr_move = generate_move(side_to_move);
    while curr_move != -(1 as libc::c_int) {
        move_list[disks_played as usize][count as usize] = curr_move;
        count += 1;
        curr_move = generate_move(side_to_move)
    }
    move_list[disks_played as usize][count as usize] = -(1 as libc::c_int);
    move_count[disks_played as usize] = count;
}
/*
  COUNT_ALL
  Counts the number of moves for one player.
*/
#[no_mangle]
pub unsafe extern "C" fn count_all(mut side_to_move: libc::c_int,
                                   mut empty: libc::c_int) -> libc::c_int {
    let mut move_0: libc::c_int = 0;
    let mut move_index: libc::c_int = 0;
    let mut mobility: libc::c_int = 0;
    let mut found_empty: libc::c_int = 0;
    mobility = 0 as libc::c_int;
    found_empty = 0 as libc::c_int;
    move_index = 0 as libc::c_int;
    while move_index < 60 as libc::c_int {
        move_0 =
            sorted_move_order[disks_played as usize][move_index as usize];
        if board[move_0 as usize] == 1 as libc::c_int {
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
#[no_mangle]
pub unsafe extern "C" fn game_in_progress() -> libc::c_int {
    let mut black_count: libc::c_int = 0;
    let mut white_count: libc::c_int = 0;
    generate_all(0 as libc::c_int);
    black_count = move_count[disks_played as usize];
    generate_all(2 as libc::c_int);
    white_count = move_count[disks_played as usize];
    return (black_count > 0 as libc::c_int || white_count > 0 as libc::c_int)
               as libc::c_int;
}
/*
   MAKE_MOVE
   side_to_move = the side that is making the move
   move = the position giving the move

   Makes the necessary changes on the board and updates the
   counters.
*/
#[no_mangle]
pub unsafe extern "C" fn make_move(mut side_to_move: libc::c_int,
                                   mut move_0: libc::c_int,
                                   mut update_hash: libc::c_int)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    let mut diff1: libc::c_uint = 0;
    let mut diff2: libc::c_uint = 0;
    if update_hash != 0 {
        flipped = DoFlips_hash(move_0, side_to_move);
        if flipped == 0 as libc::c_int { return 0 as libc::c_int }
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
        if flipped == 0 as libc::c_int { return 0 as libc::c_int }
        hash_stored1[disks_played as usize] = hash1;
        hash_stored2[disks_played as usize] = hash2
    }
    flip_count[disks_played as usize] = flipped;
    board[move_0 as usize] = side_to_move;
    if side_to_move == 0 as libc::c_int {
        piece_count[0 as libc::c_int as
                        usize][(disks_played + 1 as libc::c_int) as usize] =
            piece_count[0 as libc::c_int as usize][disks_played as usize] +
                flipped + 1 as libc::c_int;
        piece_count[2 as libc::c_int as
                        usize][(disks_played + 1 as libc::c_int) as usize] =
            piece_count[2 as libc::c_int as usize][disks_played as usize] -
                flipped
    } else {
        /* side_to_move == WHITESQ */
        piece_count[2 as libc::c_int as
                        usize][(disks_played + 1 as libc::c_int) as usize] =
            piece_count[2 as libc::c_int as usize][disks_played as usize] +
                flipped + 1 as libc::c_int;
        piece_count[0 as libc::c_int as
                        usize][(disks_played + 1 as libc::c_int) as usize] =
            piece_count[0 as libc::c_int as usize][disks_played as usize] -
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
#[no_mangle]
pub unsafe extern "C" fn make_move_no_hash(mut side_to_move: libc::c_int,
                                           mut move_0: libc::c_int)
 -> libc::c_int {
    let mut flipped: libc::c_int = 0;
    flipped = DoFlips_no_hash(move_0, side_to_move);
    if flipped == 0 as libc::c_int { return 0 as libc::c_int }
    flip_count[disks_played as usize] = flipped;
    board[move_0 as usize] = side_to_move;
    if side_to_move == 0 as libc::c_int {
        piece_count[0 as libc::c_int as
                        usize][(disks_played + 1 as libc::c_int) as usize] =
            piece_count[0 as libc::c_int as usize][disks_played as usize] +
                flipped + 1 as libc::c_int;
        piece_count[2 as libc::c_int as
                        usize][(disks_played + 1 as libc::c_int) as usize] =
            piece_count[2 as libc::c_int as usize][disks_played as usize] -
                flipped
    } else {
        /* side_to_move == WHITESQ */
        piece_count[2 as libc::c_int as
                        usize][(disks_played + 1 as libc::c_int) as usize] =
            piece_count[2 as libc::c_int as usize][disks_played as usize] +
                flipped + 1 as libc::c_int;
        piece_count[0 as libc::c_int as
                        usize][(disks_played + 1 as libc::c_int) as usize] =
            piece_count[0 as libc::c_int as usize][disks_played as usize] -
                flipped
    }
    disks_played += 1;
    return flipped;
}
/*
  UNMAKE_MOVE
  Takes back a move.
*/
#[no_mangle]
pub unsafe extern "C" fn unmake_move(mut side_to_move: libc::c_int,
                                     mut move_0: libc::c_int) {
    board[move_0 as usize] = 1 as libc::c_int;
    disks_played -= 1;
    hash1 = hash_stored1[disks_played as usize];
    hash2 = hash_stored2[disks_played as usize];
    let mut UndoFlips__flip_count = flip_count[disks_played as usize];
    let mut UndoFlips__oppcol =
        0 as libc::c_int + 2 as libc::c_int - side_to_move;
    if UndoFlips__flip_count & 1 as libc::c_int != 0 {
        UndoFlips__flip_count -= 1;
        flip_stack = flip_stack.offset(-1);
        **flip_stack = UndoFlips__oppcol
    }
    while UndoFlips__flip_count != 0 {
        UndoFlips__flip_count -= 2 as libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn unmake_move_no_hash(mut side_to_move: libc::c_int,
                                             mut move_0: libc::c_int) {
    board[move_0 as usize] = 1 as libc::c_int;
    disks_played -= 1;
    let mut UndoFlips__flip_count = flip_count[disks_played as usize];
    let mut UndoFlips__oppcol =
        0 as libc::c_int + 2 as libc::c_int - side_to_move;
    if UndoFlips__flip_count & 1 as libc::c_int != 0 {
        UndoFlips__flip_count -= 1;
        flip_stack = flip_stack.offset(-1);
        **flip_stack = UndoFlips__oppcol
    }
    while UndoFlips__flip_count != 0 {
        UndoFlips__flip_count -= 2 as libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn valid_move(mut move_0: libc::c_int,
                                    mut side_to_move: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    if move_0 < 11 as libc::c_int || move_0 > 88 as libc::c_int ||
           board[move_0 as usize] != 1 as libc::c_int {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if dir_mask[move_0 as usize] & (1 as libc::c_int) << i != 0 {
            pos = move_0 + move_offset[i as usize];
            count = 0 as libc::c_int;
            while board[pos as usize] ==
                      0 as libc::c_int + 2 as libc::c_int - side_to_move {
                pos += move_offset[i as usize];
                count += 1
            }
            if board[pos as usize] == side_to_move {
                if count >= 1 as libc::c_int { return 1 as libc::c_int }
            }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*
   GET_MOVE
   Prompts the user to enter a move and checks if the move is legal.
*/
#[no_mangle]
pub unsafe extern "C" fn get_move(mut side_to_move: libc::c_int)
 -> libc::c_int {
    let mut buffer: [libc::c_char; 255] = [0; 255];
    let mut ready = 0 as libc::c_int;
    let mut curr_move: libc::c_int = 0;
    while ready == 0 {
        if side_to_move == 0 as libc::c_int {
            printf(b"%s: \x00" as *const u8 as *const libc::c_char,
                   b"Black move\x00" as *const u8 as *const libc::c_char);
        } else {
            printf(b"%s: \x00" as *const u8 as *const libc::c_char,
                   b"White move\x00" as *const u8 as *const libc::c_char);
        }
        scanf(b"%s\x00" as *const u8 as *const libc::c_char,
              buffer.as_mut_ptr());
        curr_move = atoi(buffer.as_mut_ptr());
        ready = valid_move(curr_move, side_to_move);
        if ready == 0 {
            curr_move =
                buffer[0 as libc::c_int as usize] as libc::c_int - 'a' as i32
                    + 1 as libc::c_int +
                    10 as libc::c_int *
                        (buffer[1 as libc::c_int as usize] as libc::c_int -
                             '0' as i32);
            ready = valid_move(curr_move, side_to_move)
        }
    }
    return curr_move;
}
