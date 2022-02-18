use engine::src::game::generic_game_init;
use engine::src::moves::{generate_all, make_move};

use crate::src::error::{LibcFatalError};
use crate::src::game::{game_init, BasicBoardFileSource};
use crate::src::osfbook::{add_new_game, init_osf, read_binary_database, read_text_database, write_binary_database, write_text_database};
use crate::src::zebra::FullState;
#[macro_use]
use crate::fatal_error;
use std::ffi::CStr;
use std::fs::File;
use crate::src::display::TO_SQUARE;

/*
   INIT_LEARN
   Initialize the learning module.
*/

pub unsafe fn init_learn(file_name: *const i8, is_binary: i32, g_state: &mut FullState) {
    init_osf(0, g_state);
    if is_binary != 0 {
        read_binary_database(file_name, &mut g_state.g_book);
    } else { read_text_database(file_name, &mut g_state.g_book); }

    // strcpy replacement
    CStr::from_ptr(file_name)
        .to_bytes_with_nul()
        .iter()
        .take(g_state.learn_state.database_name.len() - 1)
        .zip(g_state.learn_state.database_name.iter_mut())
        .for_each(|(from, to) | *to = *from as _);
    g_state.learn_state.binary_database = is_binary;
}

pub struct LibcLearner;
impl LibcLearner {
    pub fn learn_game(game_length: i32, private_game: i32, save_database: i32, g_state: &mut FullState) {
        unsafe {
            learn_game(game_length, private_game, save_database, g_state.g_config.echo , g_state)
        }
    }
}
/*
   LEARN_GAME
   Play through the game and obtain an end result which assumes
   perfect endgame play from both sides. Then add the game to
   the database.
*/

pub unsafe fn learn_game(game_length: i32,
                                    private_game: i32,
                                    save_database: i32, echo:i32, g_state: &mut FullState) {
    (g_state.g_timer).clear_panic_abort();
    (g_state.g_timer).toggle_abort_check(0);
    let full_solve = (g_state.end_g).get_earliest_full_solve();
    let wld_solve = (g_state.end_g).get_earliest_wld_solve();
    let mut dummy: i32 = 0;
    generic_game_init::<BasicBoardFileSource, LibcFatalError>(None, &mut dummy, g_state);
    let mut side_to_move = 0;
    let mut i = 0;
    while i < game_length {
        generate_all(side_to_move, &mut (g_state.moves_state), &(g_state.search_state), &(g_state.board_state).board);
        if (g_state.moves_state).move_count[(g_state.moves_state).disks_played as usize] == 0 {
            side_to_move = 0 + 2 - side_to_move;
            generate_all(side_to_move, &mut (g_state.moves_state), &(g_state.search_state), &(g_state.board_state).board);
        }
        if (g_state.learn_state).game_move[i as usize] as i32 == -1 {
            fatal_error!("Cannot learn game. Missing move no. {}", i);
        }
        make_move(side_to_move, (g_state.learn_state).game_move[i as usize] as i8,
                  1, &mut (g_state.moves_state), &mut (g_state.board_state), &mut (g_state.hash_state), &mut (g_state.flip_stack_));
        if side_to_move == 2 {
            (g_state.learn_state).game_move[i as usize] =
                -((g_state.learn_state).game_move[i as usize] as i32) as i16
        }
        side_to_move = 0 + 2 - side_to_move;
        i += 1
    }
   (g_state.g_book).set_search_depth((g_state.learn_state).learn_depth);
    add_new_game(game_length, None, (g_state.learn_state).cutoff_empty,
                 full_solve, wld_solve, 1, private_game, echo, g_state);
    if save_database != 0 {
        if g_state.learn_state.binary_database != 0 {
            write_binary_database(g_state.learn_state.database_name.as_mut_ptr(), &mut g_state.g_book);
        } else { write_text_database(g_state.learn_state.database_name.as_mut_ptr(), &mut g_state.g_book); }
    }
    (g_state.g_timer).toggle_abort_check(1);
}
/*
  FULL_LEARN_PUBLIC_GAME
  This all-in-one function learns a public game using the
  parameters CUTOFF and DEVIATION_DEPTH with the same
  interpretation as in a call to set_learning_parameters().
*/

pub unsafe fn full_learn_public_game(moves: &[i32],
                                                cutoff: i32,
                                                deviation_depth:
                                                    i32,
                                                exact: i32,
                                                wld: i32, echo:i32, g_state: &mut FullState) {
    use std::io::Write;
    let length = moves.len();
    let mut stream = File::options().create(true).append(true).open("learn.log");
    if let Ok (mut stream) = stream {
        /* Write the game learned to a log file. */
        let mut i = 0;
        while i < length {
            let move_ = moves[i];
            write!(stream, "{}", TO_SQUARE(move_));
            i += 1
        }
        write!(stream, "\n");
        drop(stream);
    }
    ( g_state.g_timer).clear_panic_abort();
    ( g_state.g_timer).toggle_abort_check(0);
    /* Copy the move list from the caller as it is modified below. */
    let mut i = 0;
    while i < length {
        g_state.learn_state.game_move[i as usize] = moves[i] as i16;
        i += 1
    }
    let mut dummy: i32 = 0;
    /* Determine side to move for all positions */
    game_init(&mut dummy, g_state);
    let mut side_to_move = 0;
    let mut i = 0;
    while i < length {
        generate_all(side_to_move, &mut ( g_state.moves_state), &( g_state.search_state), &( g_state.board_state).board);
        if ( g_state.moves_state).move_count[( g_state.moves_state).disks_played as usize] == 0 {
            side_to_move = 0 + 2 - side_to_move;
            generate_all(side_to_move, &mut ( g_state.moves_state), &( g_state.search_state), &( g_state.board_state).board);
        }
        make_move(side_to_move, ( g_state.learn_state).game_move[i as usize] as i8,
                  1, &mut ( g_state.moves_state), &mut ( g_state.board_state), &mut ( g_state.hash_state), &mut ( g_state.flip_stack_));
        if side_to_move == 2 {
            ( g_state.learn_state).game_move[i as usize] =
                -(( g_state.learn_state).game_move[i as usize] as i32) as i16
        }
        side_to_move = 0 + 2 - side_to_move;
        i += 1
    }
    /* Let the learning sub-routine in osfbook update the opening
       book and the dump it to file. */
   ( g_state.g_book).set_search_depth(deviation_depth);
    add_new_game(length as _, None, cutoff, exact, wld,
                 1, 0, echo, g_state);
    if g_state.learn_state.binary_database != 0 {
        write_binary_database(g_state.learn_state.database_name.as_mut_ptr(), &mut g_state.g_book);
    } else { write_text_database(g_state.learn_state.database_name.as_mut_ptr(), &mut g_state.g_book); }
    ( g_state.g_timer).toggle_abort_check(1);
}
