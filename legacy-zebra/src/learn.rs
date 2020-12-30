use crate::src::osfbook::{write_text_database, write_binary_database, add_new_game, read_text_database, read_binary_database, init_osf};
use crate::src::game::{game_init, LibcBoardFileSource};
use libc_wrapper::{fclose, fputs, fprintf, fopen, strcpy};
use engine::src::game::generic_game_init;
use crate::src::error::LibcFatalError;
use engine::src::timer::{g_timer};
use engine::src::moves::{make_move,  moves_state, generate_all};
use engine::src::end::{end_g};
use engine::src::learn::{Learner};
use crate::src::zebra::g_config;
use engine::src::zebra::learn_state;
use engine::src::globals::board_state;
use engine::src::search::search_state;
use engine::src::osfbook::g_book;
use flip::unflip::flip_stack_;
use engine::src::hash::hash_state;

pub static mut binary_database: i32 = 0;
pub static mut database_name: [i8; 256] = [0; 256];

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

pub struct LibcLearner;
impl Learner for LibcLearner {
    fn learn_game(game_length: i32, private_game: i32, save_database: i32) {
        unsafe {
            learn_game(game_length, private_game, save_database, g_config.echo)
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
                                    save_database: i32, echo:i32) {
    g_timer.clear_panic_abort();
    g_timer.toggle_abort_check(0 as i32);
    let full_solve = end_g.get_earliest_full_solve();
    let wld_solve = end_g.get_earliest_wld_solve();
    let mut dummy: i32 = 0;
    generic_game_init::<LibcBoardFileSource, LibcFatalError>(0 as *const i8, &mut dummy);
    let mut side_to_move = 0;
    let mut i = 0;
    while i < game_length {
        generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
        if moves_state.move_count[moves_state.disks_played as usize] == 0 as i32 {
            side_to_move = 0 as i32 + 2 as i32 - side_to_move;
            generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
        }
        make_move(side_to_move, learn_state.game_move[i as usize] as i32,
                  1 as i32, &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        if side_to_move == 2 as i32 {
            learn_state.game_move[i as usize] =
                -(learn_state.game_move[i as usize] as i32) as i16
        }
        side_to_move = 0 as i32 + 2 as i32 - side_to_move;
        i += 1
    }
   g_book.set_search_depth(learn_state.learn_depth);
    add_new_game(game_length, learn_state.game_move.as_mut_ptr(), learn_state.cutoff_empty,
                 full_solve, wld_solve, 1 as i32, private_game, echo);
    if save_database != 0 {
        if binary_database != 0 {
            write_binary_database(database_name.as_mut_ptr());
        } else { write_text_database(database_name.as_mut_ptr()); }
    }
    g_timer.toggle_abort_check(1 as i32);
}
/*
  FULL_LEARN_PUBLIC_GAME
  This all-in-one function learns a public game using the
  parameters CUTOFF and DEVIATION_DEPTH with the same
  interpretation as in a call to set_learning_parameters().
*/

pub unsafe fn full_learn_public_game(length: i32,
                                                moves: *mut i32,
                                                cutoff: i32,
                                                deviation_depth:
                                                    i32,
                                                exact: i32,
                                                wld: i32, echo:i32) {
    let stream =
        fopen(b"learn.log\x00" as *const u8 as *const i8,
              b"a\x00" as *const u8 as *const i8);
    if !stream.is_null() {
        /* Write the game learned to a log file. */
        let mut i = 0;
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
    g_timer.clear_panic_abort();
    g_timer.toggle_abort_check(0 as i32);
    /* Copy the move list from the caller as it is modified below. */
    let mut i = 0;
    while i < length {
        learn_state.game_move[i as usize] = *moves.offset(i as isize) as i16;
        i += 1
    }
    let mut dummy: i32 = 0;
    /* Determine side to move for all positions */
    game_init(0 as *const i8, &mut dummy);
    let mut side_to_move = 0;
    let mut i = 0;
    while i < length {
        generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
        if moves_state.move_count[moves_state.disks_played as usize] == 0 as i32 {
            side_to_move = 0 as i32 + 2 as i32 - side_to_move;
            generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
        }
        make_move(side_to_move, learn_state.game_move[i as usize] as i32,
                  1 as i32, &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        if side_to_move == 2 as i32 {
            learn_state.game_move[i as usize] =
                -(learn_state.game_move[i as usize] as i32) as i16
        }
        side_to_move = 0 as i32 + 2 as i32 - side_to_move;
        i += 1
    }
    /* Let the learning sub-routine in osfbook update the opening
       book and the dump it to file. */
   g_book.set_search_depth(deviation_depth);
    add_new_game(length, learn_state.game_move.as_mut_ptr(), cutoff, exact, wld,
                 1 as i32, 0 as i32, echo);
    if binary_database != 0 {
        write_binary_database(database_name.as_mut_ptr());
    } else { write_text_database(database_name.as_mut_ptr()); }
    g_timer.toggle_abort_check(1 as i32);
}
