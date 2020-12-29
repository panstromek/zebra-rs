use crate::src::cntflip::AnyFlips_compact;
use crate::src::globals::{board_state, Board};
use crate::src::hash::{hash_state};
use crate::src::search::{search_state, SearchState};
use crate::src::zebra::ZebraFrontend;
use std::future::Future;
use flip::unflip::flip_stack_;
use flip::doflip::{DoFlips_no_hash, DoFlips_hash};
use std::error::Error;
use engine_traits::Offset;
/*
   File:              moves.c

   Created:           June 30, 1997

   Modified:          April 24, 2001

   Author:            Gunnar Andersson (gunnar@radagast.se)

   Contents:          The move generator.
*/

pub struct MovesState {
    pub disks_played: i32,
    pub move_count: [i32; 64],
    pub move_list: [[i32; 64]; 64],
    flip_count: [i32; 65],
    sweep_status: [i32; 64],
}

pub static mut moves_state: MovesState = MovesState {
    disks_played: 0,
    move_count: [0; 64],
    move_list: [[0; 64]; 64],
    flip_count: [0; 65],
    sweep_status: [0; 64],
};

pub static flip_direction: [[i32; 16]; 100] = init_flip_direction();

pub static dir_mask: [i32; 100] = const_dir_mask;
pub const const_dir_mask: [i32; 100] =
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

pub const const_move_offset: [i32; 8] = [1, -1, 9, -9, 10, -10, 11, -11];
pub static move_offset: [i32; 8] = const_move_offset;

/*
   MAKE_MOVE
   side_to_move = the side that is making the move
   move = the position giving the move

   Makes the necessary changes on the board and updates the
   counters.
*/

/* Local variables */


/*
  INIT_MOVES
  Initialize the move generation subsystem.
*/

pub const fn init_flip_direction() -> [[i32; 16]; 100] {
    let mut flip_direction_ : [[i32; 16]; 100] = [[0; 16]; 100];
    let mut feasible = 0;
    let mut i = 1;
    while i <= 8 {
        let mut j = 1;
        while j <= 8 {
            let pos = 10 * i + j;
            let mut k = 0;
            while k <= 8 {
                flip_direction_[pos][k] = 0;
                k += 1
            }
            feasible = 0;
            let mut k = 0;
            while k < 8 {
                if const_dir_mask[pos] & 1 << k != 0 {
                    flip_direction_[pos][feasible] = const_move_offset[k];
                    feasible += 1
                }
                k += 1
            }
            j += 1
        }
        i += 1
    };
    flip_direction_
}
impl MovesState {
    /*
       RESET_GENERATION
       Prepare for move generation at a given level in the tree.
    */
    fn reset_generation(&mut self, _side_to_move: i32) {
        self.sweep_status[self.disks_played as usize] = 0;
    }
}
pub unsafe fn make_move(side_to_move: i32,
                        move_0: i32,
                        update_hash: i32)
                        -> i32 {
    let mut flipped: i32 = 0;
    let mut diff1: u32 = 0;
    let mut diff2: u32 = 0;
    if update_hash != 0 {
        let (flipped_, hash_update1_, hash_update2_) = DoFlips_hash(
            move_0, side_to_move, &mut board_state.board,
            &mut hash_state.hash_flip1, &mut hash_state.hash_flip2, &mut flip_stack_);
        flipped = flipped_;
        if flipped == 0 as i32 { return 0 as i32 }
        diff1 =
            hash_update1_ ^
                hash_state.hash_put_value1[side_to_move as usize][move_0 as usize];
        diff2 =
            hash_update2_ ^
                hash_state.hash_put_value2[side_to_move as usize][move_0 as usize];
        hash_state.hash_stored1[moves_state.disks_played as usize] = hash_state.hash1;
        hash_state.hash_stored2[moves_state.disks_played as usize] = hash_state.hash2;
        hash_state.hash1 ^= diff1;
        hash_state.hash2 ^= diff2
    } else {
        flipped = DoFlips_no_hash(move_0, side_to_move, &mut board_state.board, &mut flip_stack_);
        if flipped == 0 as i32 { return 0 as i32 }
        hash_state.hash_stored1[moves_state.disks_played as usize] = hash_state.hash1;
        hash_state.hash_stored2[moves_state.disks_played as usize] = hash_state.hash2
    }
    moves_state.flip_count[moves_state.disks_played as usize] = flipped;
    board_state.board[move_0 as usize] = side_to_move;
    if side_to_move == 0 as i32 {
        board_state.piece_count[0][(moves_state.disks_played + 1 as i32) as usize] =
            board_state.piece_count[0][moves_state.disks_played as usize] +
                flipped + 1 as i32;
        board_state.piece_count[2][(moves_state.disks_played + 1 as i32) as usize] =
            board_state.piece_count[2][moves_state.disks_played as usize] -
                flipped
    } else {
        /* side_to_move == WHITESQ */
        board_state.piece_count[2][(moves_state.disks_played + 1 as i32) as usize] =
            board_state.piece_count[2][moves_state.disks_played as usize] +
                flipped + 1 as i32;
        board_state.piece_count[0][(moves_state.disks_played + 1 as i32) as usize] =
            board_state.piece_count[0][moves_state.disks_played as usize] -
                flipped
    }
    moves_state.disks_played += 1;
    return flipped;
}
/*
  UNMAKE_MOVE
  Takes back a move.
*/

pub unsafe fn unmake_move(side_to_move: i32,
                          move_0: i32) {
    board_state.board[move_0 as usize] = 1;
    moves_state.disks_played -= 1;
    hash_state.hash1 = hash_state.hash_stored1[moves_state.disks_played as usize];
    hash_state.hash2 = hash_state.hash_stored2[moves_state.disks_played as usize];
    let mut UndoFlips__flip_count = moves_state.flip_count[moves_state.disks_played as usize];
    let UndoFlips__oppcol =
        0 as i32 + 2 as i32 - side_to_move;
    if UndoFlips__flip_count & 1 as i32 != 0 {
        UndoFlips__flip_count -= 1;
        flip_stack_.flip_stack = flip_stack_.flip_stack.offset(-1);
        board_state.board[flip_stack_.global_flip_stack[flip_stack_.flip_stack]] = UndoFlips__oppcol
    }
    while UndoFlips__flip_count != 0 {
        UndoFlips__flip_count -= 2 as i32;
        flip_stack_.flip_stack = flip_stack_.flip_stack.offset(-1);
        board_state.board[flip_stack_.global_flip_stack[flip_stack_.flip_stack]] = UndoFlips__oppcol;
        flip_stack_.flip_stack = flip_stack_.flip_stack.offset(-1);
        board_state.board[flip_stack_.global_flip_stack[flip_stack_.flip_stack]] = UndoFlips__oppcol
    };
}


/*
   GENERATE_SPECIFIC
*/

pub fn generate_specific(curr_move: i32, side_to_move: i32, board: &[i32; 128]) -> i32 {
    let inc = &flip_direction[curr_move as usize]; //first_flip_direction[curr_move as usize];
    return AnyFlips_compact(board, inc, curr_move, side_to_move,
                            0 as i32 + 2 as i32 - side_to_move);
}
/*
   GENERATE_MOVE
   side_to_move = the side to generate moves for

   Generate the next move in the ordering. This way not all moves possible
   in a position are generated, only those who need be considered.
*/

pub fn generate_move(side_to_move: i32, board: &[i32; 128],
                            moves_state_: &mut MovesState, search_state_: &SearchState)
                            -> i32 {
    let mut move_0: i32 = 0;
    let mut move_index = 0;
    move_index = moves_state_.sweep_status[moves_state_.disks_played as usize];
    while move_index < 60 as i32 {
        move_0 =
            search_state_.sorted_move_order[moves_state_.disks_played as usize][move_index as usize];
        if board[move_0 as usize] == 1 as i32 &&
            generate_specific(move_0, side_to_move, board) != 0 {
            moves_state_.sweep_status[moves_state_.disks_played as usize] =
                move_index + 1 as i32;
            return move_0
        } else { move_index += 1 }
    }
    moves_state_.sweep_status[moves_state_.disks_played as usize] = move_index;
    return -(1 as i32);
}
/*
   GENERATE_ALL
   Generates a list containing all the moves possible in a position.
*/

pub unsafe fn generate_all_unsafe(side_to_move: i32) {
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
}

fn generate_all(side_to_move: i32, moves_state_: &mut MovesState, search_state_: &SearchState, board: &Board) {
    moves_state_.reset_generation(side_to_move);
    let mut count = 0;
    let mut curr_move = generate_move(side_to_move, board, moves_state_, search_state_);
    while curr_move != -(1 as i32) {
        moves_state_.move_list[moves_state_.disks_played as usize][count as usize] = curr_move;
        count += 1;
        curr_move = generate_move(side_to_move, board, moves_state_, search_state_)
    }
    moves_state_.move_list[moves_state_.disks_played as usize][count as usize] = -(1 as i32);
    moves_state_.move_count[moves_state_.disks_played as usize] = count;
}
/*
  COUNT_ALL
  Counts the number of moves for one player.
*/

pub unsafe fn count_all(side_to_move: i32,
                        empty: i32) -> i32 {
    let mut move_0: i32 = 0;
    let mut move_index: i32 = 0;
    let mut mobility: i32 = 0;
    let mut found_empty: i32 = 0;
    mobility = 0;
    found_empty = 0;
    move_index = 0;
    while move_index < 60 as i32 {
        move_0 =
            search_state.sorted_move_order[moves_state.disks_played as usize][move_index as usize];
        if board_state.board[move_0 as usize] == 1 as i32 {
            if generate_specific(move_0, side_to_move, &board_state.board) != 0 { mobility += 1 }
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
    generate_all_unsafe(0 as i32);
    black_count = moves_state.move_count[moves_state.disks_played as usize];
    generate_all_unsafe(2 as i32);
    white_count = moves_state.move_count[moves_state.disks_played as usize];
    return (black_count > 0 as i32 || white_count > 0 as i32)
        as i32;
}

/*
   MAKE_MOVE_NO_HASH
   side_to_move = the side that is making the move
   move = the position giving the move

   Makes the necessary changes on the board. Note that the hash table
   is not updated - the move has to be unmade using UNMAKE_MOVE_NO_HASH().
*/

pub unsafe fn make_move_no_hash(side_to_move: i32,
                                move_0: i32)
                                -> i32 {
    let mut flipped: i32 = 0;
    flipped = DoFlips_no_hash(move_0, side_to_move, &mut board_state.board, &mut flip_stack_);
    if flipped == 0 as i32 { return 0 as i32 }
    moves_state.flip_count[moves_state.disks_played as usize] = flipped;
    board_state.board[move_0 as usize] = side_to_move;
    if side_to_move == 0 as i32 {
        board_state.piece_count[0][(moves_state.disks_played + 1 as i32) as usize] =
            board_state.piece_count[0][moves_state.disks_played as usize] +
                flipped + 1 as i32;
        board_state.piece_count[2][(moves_state.disks_played + 1 as i32) as usize] =
            board_state.piece_count[2][moves_state.disks_played as usize] -
                flipped
    } else {
        /* side_to_move == WHITESQ */
        board_state.piece_count[2][(moves_state.disks_played + 1 as i32) as usize] =
            board_state.piece_count[2][moves_state.disks_played as usize] +
                flipped + 1 as i32;
        board_state.piece_count[0][(moves_state.disks_played + 1 as i32) as usize] =
            board_state.piece_count[0][moves_state.disks_played as usize] -
                flipped
    }
    moves_state.disks_played += 1;
    return flipped;
}

/*
  UNMAKE_MOVE_NO_HASH
  Takes back a move. Only to be called when the move was made without
  updating hash table, preferrable through MAKE_MOVE_NO_HASH().
*/

pub unsafe fn unmake_move_no_hash(side_to_move: i32,
                                  move_0: i32) {
    board_state.board[move_0 as usize] = 1;
    moves_state.disks_played -= 1;
    let mut UndoFlips__flip_count = moves_state.flip_count[moves_state.disks_played as usize];
    let UndoFlips__oppcol =
        0 as i32 + 2 as i32 - side_to_move;
    if UndoFlips__flip_count & 1 as i32 != 0 {
        UndoFlips__flip_count -= 1;
        flip_stack_.flip_stack = flip_stack_.flip_stack - 1;
        board_state.board[flip_stack_.global_flip_stack[flip_stack_.flip_stack]] = UndoFlips__oppcol
    }
    while UndoFlips__flip_count != 0 {
        UndoFlips__flip_count -= 2 as i32;
        flip_stack_.flip_stack = flip_stack_.flip_stack - 1;
        board_state.board[flip_stack_.global_flip_stack[flip_stack_.flip_stack]] = UndoFlips__oppcol;
        flip_stack_.flip_stack = flip_stack_.flip_stack - 1;
        board_state.board[flip_stack_.global_flip_stack[flip_stack_.flip_stack]] = UndoFlips__oppcol
    };
}
/*
   VALID_MOVE
   Determines if a move is legal.
*/

pub unsafe fn valid_move(move_0: i32,
                         side_to_move: i32)
                         -> i32 {
    let mut i: i32 = 0;
    let mut pos: i32 = 0;
    let mut count: i32 = 0;
    if move_0 < 11 as i32 || move_0 > 88 as i32 ||
        board_state.board[move_0 as usize] != 1 as i32 {
        return 0 as i32
    }
    i = 0;
    while i < 8 as i32 {
        if dir_mask[move_0 as usize] & (1 as i32) << i != 0 {
            pos = move_0 + move_offset[i as usize];
            count = 0;
            while board_state.board[pos as usize] ==
                0 as i32 + 2 as i32 - side_to_move {
                pos += move_offset[i as usize];
                count += 1
            }
            if board_state.board[pos as usize] == side_to_move {
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
pub unsafe fn get_move<ZFE: ZebraFrontend>(side_to_move: i32) -> i32 {
    let mut buffer: [i8; 255] = [0; 255];
    let mut ready = 0;
    let mut curr_move: i32 = 0;
    while ready == 0 {
        ZFE::prompt_get_move(side_to_move, &mut buffer);
        ready = valid_move(curr_move, side_to_move);
        if ready == 0 {
            curr_move =
                buffer[0] as i32 - 'a' as i32 + 1 + 10 * (buffer[1] as i32 - '0' as i32);
            ready = valid_move(curr_move, side_to_move)
        }
    }
    curr_move
}

/*
   GET_MOVE
   Prompts the user to enter a move and checks if the move is legal.
*/
pub async unsafe fn get_move_async<GetMove, Fut>(side_to_move: i32, get_move: &mut GetMove) -> Result<i32, Box<dyn Error>>
    where
        GetMove: FnMut(i32) -> Fut,
        Fut: Future<Output=Result<i32, Box<dyn Error>>>
{
    let mut buffer: [i8; 255] = [0; 255];
    let mut ready = 0;
    let mut curr_move: i32 = 0;
    while ready == 0 {
        curr_move = get_move(side_to_move).await?;
        ready = valid_move(curr_move, side_to_move);
        if ready == 0 {
            curr_move =
                buffer[0] as i32 - 'a' as i32 + 1 + 10 * (buffer[1] as i32 - '0' as i32);
            ready = valid_move(curr_move, side_to_move)
        }
    }
    Ok(curr_move)
}
