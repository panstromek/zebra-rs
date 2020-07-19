use crate::src::libc;
use crate::src::timer::{toggle_abort_check, clear_panic_abort};
use crate::src::osfbook::{write_text_database, write_binary_database, add_new_game, set_search_depth, read_text_database, read_binary_database, init_osf};
use crate::src::moves::{make_move, generate_all, disks_played, move_count};
use crate::src::game::game_init;
use crate::src::stubs::{fclose, fputs, fprintf, fopen, strcpy};
use crate::src::end::{get_earliest_wld_solve, get_earliest_full_solve};
use crate::src::zebra::_IO_FILE;

pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
static mut database_name: [libc::c_char; 256] = [0; 256];
static mut binary_database: libc::c_int = 0;
static mut learn_depth: libc::c_int = 0;
static mut cutoff_empty: libc::c_int = 0;
static mut game_move: [libc::c_short; 61] = [0; 61];
/*
   File:          learn.h

   Created:       November 29, 1997

   Modified:      November 18, 2001

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The interface to the learning module.
*/
/*
   CLEAR_STORED_GAME
   Remove all stored moves.
*/

pub unsafe fn clear_stored_game() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <= 60 as libc::c_int {
        game_move[i as usize] = -(1 as libc::c_int) as libc::c_short;
        i += 1
    };
}
/*
   STORE_MOVE
   Mark the move MOVE as being played after DISKS_PLAYED moves
   had been played.
*/

pub unsafe fn store_move(mut disks_played_0: libc::c_int,
                                    mut move_0: libc::c_int) {
    game_move[disks_played_0 as usize] = move_0 as libc::c_short;
}
/*
   SET_LEARNING_PARAMETERS
   Specify the depth to which deviations are checked for and the number
   of empty squares at which the game is considered over.
*/

pub unsafe fn set_learning_parameters(mut depth: libc::c_int,
                                                 mut cutoff: libc::c_int) {
    learn_depth = depth;
    cutoff_empty = cutoff;
}
/*
   GAME_LEARNABLE
   Checks if the current game can be learned - i.e. if the moves of the
   game are available and the game was finished or contains enough
   moves to be learned anyway.
*/

pub unsafe fn game_learnable(mut finished: libc::c_int,
                                        mut move_count_0: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut moves_available: libc::c_int = 0;
    moves_available = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < move_count_0 && i < 60 as libc::c_int - cutoff_empty {
        if game_move[i as usize] as libc::c_int == -(1 as libc::c_int) {
            moves_available = 0 as libc::c_int
        }
        i += 1
    }
    return (moves_available != 0 &&
                (finished != 0 ||
                     move_count_0 >= 60 as libc::c_int - cutoff_empty)) as
               libc::c_int;
}
/*
   INIT_LEARN
   Initialize the learning module.
*/

pub unsafe fn init_learn(mut file_name: *const libc::c_char,
                                    mut is_binary: libc::c_int) {
    init_osf(0 as libc::c_int);
    if is_binary != 0 {
        read_binary_database(file_name);
    } else { read_text_database(file_name); }
    strcpy(database_name.as_mut_ptr(), file_name);
    binary_database = is_binary;
}
/*
   LEARN_GAME
   Play through the game and obtain an end result which assumes
   perfect endgame play from both sides. Then add the game to
   the database.
*/

pub unsafe fn learn_game(mut game_length: libc::c_int,
                                    mut private_game: libc::c_int,
                                    mut save_database: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut dummy: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut full_solve: libc::c_int = 0;
    let mut wld_solve: libc::c_int = 0;
    clear_panic_abort();
    toggle_abort_check(0 as libc::c_int);
    full_solve = get_earliest_full_solve();
    wld_solve = get_earliest_wld_solve();
    game_init(0 as *const libc::c_char, &mut dummy);
    side_to_move = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < game_length {
        generate_all(side_to_move);
        if move_count[disks_played as usize] == 0 as libc::c_int {
            side_to_move = 0 as libc::c_int + 2 as libc::c_int - side_to_move;
            generate_all(side_to_move);
        }
        make_move(side_to_move, game_move[i as usize] as libc::c_int,
                  1 as libc::c_int);
        if side_to_move == 2 as libc::c_int {
            game_move[i as usize] =
                -(game_move[i as usize] as libc::c_int) as libc::c_short
        }
        side_to_move = 0 as libc::c_int + 2 as libc::c_int - side_to_move;
        i += 1
    }
    set_search_depth(learn_depth);
    add_new_game(game_length, game_move.as_mut_ptr(), cutoff_empty,
                 full_solve, wld_solve, 1 as libc::c_int, private_game);
    if save_database != 0 {
        if binary_database != 0 {
            write_binary_database(database_name.as_mut_ptr());
        } else { write_text_database(database_name.as_mut_ptr()); }
    }
    toggle_abort_check(1 as libc::c_int);
}
/*
  FULL_LEARN_PUBLIC_GAME
  This all-in-one function learns a public game using the
  parameters CUTOFF and DEVIATION_DEPTH with the same
  interpretation as in a call to set_learning_parameters().
*/

pub unsafe fn full_learn_public_game(mut length: libc::c_int,
                                                mut moves: *mut libc::c_int,
                                                mut cutoff: libc::c_int,
                                                mut deviation_depth:
                                                    libc::c_int,
                                                mut exact: libc::c_int,
                                                mut wld: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut dummy: libc::c_int = 0;
    let mut side_to_move: libc::c_int = 0;
    let mut stream = 0 as *mut FILE;
    stream =
        fopen(b"learn.log\x00" as *const u8 as *const libc::c_char,
              b"a\x00" as *const u8 as *const libc::c_char);
    if !stream.is_null() {
        /* Write the game learned to a log file. */
        i = 0 as libc::c_int;
        while i < length {
            fprintf(stream, b"%c%c\x00" as *const u8 as *const libc::c_char,
                    'a' as i32 + *moves.offset(i as isize) % 10 as libc::c_int
                        - 1 as libc::c_int,
                    '0' as i32 +
                        *moves.offset(i as isize) / 10 as libc::c_int);
            i += 1
        }
        fputs(b"\n\x00" as *const u8 as *const libc::c_char, stream);
        fclose(stream);
    }
    clear_panic_abort();
    toggle_abort_check(0 as libc::c_int);
    /* Copy the move list from the caller as it is modified below. */
    i = 0 as libc::c_int;
    while i < length {
        game_move[i as usize] = *moves.offset(i as isize) as libc::c_short;
        i += 1
    }
    /* Determine side to move for all positions */
    game_init(0 as *const libc::c_char, &mut dummy);
    side_to_move = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < length {
        generate_all(side_to_move);
        if move_count[disks_played as usize] == 0 as libc::c_int {
            side_to_move = 0 as libc::c_int + 2 as libc::c_int - side_to_move;
            generate_all(side_to_move);
        }
        make_move(side_to_move, game_move[i as usize] as libc::c_int,
                  1 as libc::c_int);
        if side_to_move == 2 as libc::c_int {
            game_move[i as usize] =
                -(game_move[i as usize] as libc::c_int) as libc::c_short
        }
        side_to_move = 0 as libc::c_int + 2 as libc::c_int - side_to_move;
        i += 1
    }
    /* Let the learning sub-routine in osfbook update the opening
       book and the dump it to file. */
    set_search_depth(deviation_depth);
    add_new_game(length, game_move.as_mut_ptr(), cutoff, exact, wld,
                 1 as libc::c_int, 0 as libc::c_int);
    if binary_database != 0 {
        write_binary_database(database_name.as_mut_ptr());
    } else { write_text_database(database_name.as_mut_ptr()); }
    toggle_abort_check(1 as libc::c_int);
}
