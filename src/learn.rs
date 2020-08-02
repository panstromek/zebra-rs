use crate::src::timer::{toggle_abort_check, clear_panic_abort};
use crate::src::osfbook::{write_text_database, write_binary_database, add_new_game, set_search_depth, read_text_database, read_binary_database, init_osf};
use crate::src::moves::{make_move, generate_all, disks_played, move_count};
use crate::src::game::game_init;
use crate::src::stubs::{fclose, fputs, fprintf, fopen, strcpy};
use crate::src::end::{get_earliest_wld_solve, get_earliest_full_solve};
use crate::src::zebra::_IO_FILE;
pub use engine::src::learn::*;

pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
/*
   INIT_LEARN
   Initialize the learning module.
*/

pub unsafe fn init_learn(file_name: *const i8, is_binary: i32) {
    init_osf(0 as i32);
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

pub unsafe fn learn_game(mut game_length: i32,
                                    mut private_game: i32,
                                    mut save_database: i32) {
    clear_panic_abort();
    toggle_abort_check(0 as i32);
    let full_solve = get_earliest_full_solve();
    let wld_solve = get_earliest_wld_solve();
    let mut dummy: i32 = 0;
    game_init(0 as *const i8, &mut dummy);
    let mut side_to_move = 0 as i32;
    let mut i = 0 as i32;
    while i < game_length {
        generate_all(side_to_move);
        if move_count[disks_played as usize] == 0 as i32 {
            side_to_move = 0 as i32 + 2 as i32 - side_to_move;
            generate_all(side_to_move);
        }
        make_move(side_to_move, game_move[i as usize] as i32,
                  1 as i32);
        if side_to_move == 2 as i32 {
            game_move[i as usize] =
                -(game_move[i as usize] as i32) as i16
        }
        side_to_move = 0 as i32 + 2 as i32 - side_to_move;
        i += 1
    }
    set_search_depth(learn_depth);
    add_new_game(game_length, game_move.as_mut_ptr(), cutoff_empty,
                 full_solve, wld_solve, 1 as i32, private_game);
    if save_database != 0 {
        if binary_database != 0 {
            write_binary_database(database_name.as_mut_ptr());
        } else { write_text_database(database_name.as_mut_ptr()); }
    }
    toggle_abort_check(1 as i32);
}
/*
  FULL_LEARN_PUBLIC_GAME
  This all-in-one function learns a public game using the
  parameters CUTOFF and DEVIATION_DEPTH with the same
  interpretation as in a call to set_learning_parameters().
*/

pub unsafe fn full_learn_public_game(mut length: i32,
                                                mut moves: *mut i32,
                                                mut cutoff: i32,
                                                mut deviation_depth:
                                                    i32,
                                                mut exact: i32,
                                                mut wld: i32) {
    let mut stream =
        fopen(b"learn.log\x00" as *const u8 as *const i8,
              b"a\x00" as *const u8 as *const i8);
    if !stream.is_null() {
        /* Write the game learned to a log file. */
        let mut i = 0 as i32;
        while i < length {
            fprintf(stream, b"%c%c\x00" as *const u8 as *const i8,
                    'a' as i32 + *moves.offset(i as isize) % 10 as i32
                        - 1 as i32,
                    '0' as i32 +
                        *moves.offset(i as isize) / 10 as i32);
            i += 1
        }
        fputs(b"\n\x00" as *const u8 as *const i8, stream);
        fclose(stream);
    }
    clear_panic_abort();
    toggle_abort_check(0 as i32);
    /* Copy the move list from the caller as it is modified below. */
    let mut i = 0 as i32;
    while i < length {
        game_move[i as usize] = *moves.offset(i as isize) as i16;
        i += 1
    }
    let mut dummy: i32 = 0;
    /* Determine side to move for all positions */
    game_init(0 as *const i8, &mut dummy);
    let mut side_to_move = 0 as i32;
    let mut i = 0 as i32;
    while i < length {
        generate_all(side_to_move);
        if move_count[disks_played as usize] == 0 as i32 {
            side_to_move = 0 as i32 + 2 as i32 - side_to_move;
            generate_all(side_to_move);
        }
        make_move(side_to_move, game_move[i as usize] as i32,
                  1 as i32);
        if side_to_move == 2 as i32 {
            game_move[i as usize] =
                -(game_move[i as usize] as i32) as i16
        }
        side_to_move = 0 as i32 + 2 as i32 - side_to_move;
        i += 1
    }
    /* Let the learning sub-routine in osfbook update the opening
       book and the dump it to file. */
    set_search_depth(deviation_depth);
    add_new_game(length, game_move.as_mut_ptr(), cutoff, exact, wld,
                 1 as i32, 0 as i32);
    if binary_database != 0 {
        write_binary_database(database_name.as_mut_ptr());
    } else { write_text_database(database_name.as_mut_ptr()); }
    toggle_abort_check(1 as i32);
}
