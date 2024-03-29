use engine::src::counter::reset_counter;
use engine::src::end::end_game;
use engine::src::error::FrontEnd;
use engine::src::game::{engine_game_init, setup_non_file_based_game};
use engine::src::getcoeff::remove_coeffs;
use engine::src::hash::{clear_hash_drafts, determine_hash_values, setup_hash};
use engine::src::midgame::{tree_search};
use engine::src::moves::{generate_all, generate_specific, make_move, unmake_move};
use engine::src::myrandom::MyRandom;
use engine::src::osfbook::{__time_t, adjust_score, Book, BOOK_MAPS, BookNode, clear_node_depth, get_hash, get_node_depth, probe_hash_table, reset_book_search, set_node_depth};
use engine::src::search::disc_count;
use engine::src::stubs::{abs};
use engine::src::zebra::EvaluationType;
use engine::src::zebra::GameMode::PRIVATE_GAME;
use engine_traits::Offset;
use libc_wrapper::{fclose, fopen, fscanf, stdout, time};
use std::io::Write;
#[macro_use]
use crate::fatal_error;

use crate::{
    src::{
        game::{global_setup}
    }
};

use crate::src::error::{LibcFatalError};
use crate::src::zebra::FullState;

use engine::src::globals::BoardState;
use std::ffi::CStr;
use crate::src::display::TO_SQUARE;

pub type FE = LibcFatalError;

pub type _IO_lock_t = ();
pub type time_t = __time_t;

/*
  ADD_NEW_GAME
  Adds a new game to the game tree.
*/

pub fn add_new_game(move_count_0: i32,
                           game_move_list: Option<&[i16]>,
                           min_empties: i32,
                           max_full_solve: i32,
                           max_wld_solve: i32,
                           update_path: i32,
                           private_game: i32, mut echo:i32, g_state: &mut FullState) {
    let mut dummy_info =EvaluationType::new();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut stored_echo: i32 = 0;
    let mut dummy_black_score: i32 = 0;
    let mut dummy_white_score: i32 = 0;
    let mut force_eval: i32 = 0;
    let mut midgame_eval_done: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move = 0;
    let mut slot: i32 = 0;
    let mut this_node: i32 = 0;
    let mut last_move_number: i32 = 0;
    let mut first_new_node: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut outcome: i32 = 0;
    let mut visited_node: [i32; 61] = [0; 61];
    let mut flags: [u16; 61] = [0; 61];
    stored_echo = echo;
    echo = 0;
    /* First create new nodes for new positions */
    prepare_tree_traversal(g_state);

    i = 0;
    while i < move_count_0 {
        if *game_move_list.unwrap_or(&g_state.learn.game_move).offset(i as isize) > 0 {
            flags[i as usize] = 1
        } else {
            flags[i as usize] = 2
        }
        i += 1
    }
    flags[move_count_0 as usize] = 0;
    first_new_node = 61;
    this_node = 0;
    side_to_move = 0;
    last_move_number = if move_count_0 < 60 - min_empties { move_count_0 } else { 60 - min_empties };
    i = 0;
    while i <= last_move_number {
        /* Look for the position in the hash table */
        get_hash(&mut val1, &mut val2, &mut orientation, &mut (g_state.g_book), &(g_state.board).board);
        slot = probe_hash_table(val1, val2, &mut (g_state.g_book));
        if slot == -1 || *(g_state.g_book).book_hash_table.offset(slot as isize) == -1 {
            this_node = create_BookNode(val1, val2, flags[i as usize], &mut (g_state.g_book));
            if private_game != 0 {
                let ref mut fresh26 = (*(g_state.g_book).node.offset(this_node as isize)).flags;
                *fresh26 = (*fresh26 as i32 | 32) as u16
            }
            if i < first_new_node {
                first_new_node = i
            }
        } else {
            this_node = *(g_state.g_book).book_hash_table.offset(slot as isize)
        }
        visited_node[i as usize] = this_node;
        /* Make the moves of the game until the cutoff point */
        if i < last_move_number {
            this_move = abs(*game_move_list.unwrap_or(&g_state.learn.game_move).offset(i as isize) as i32) as i8;
            if *game_move_list.unwrap_or(&g_state.learn.game_move).offset(i as isize) > 0 {
                side_to_move = 0
            } else {
                side_to_move = 2
            }
            if generate_specific(this_move, side_to_move, &(g_state.board).board) == 0 {
                write!(stdout, "\n");
                write!(stdout, "i={}, side_to_move={}, this_move={}\n", i, side_to_move, this_move);
                write!(stdout, "last_move_number={}, move_count={}\n", last_move_number, move_count_0);
                j = 0;
                while j < move_count_0 {
                    write!(stdout, "{:3} ", *game_move_list.unwrap_or(&g_state.learn.game_move).offset(j as isize) as i32);
                    j += 1
                }
                fatal_error!("{}: {}\n", "Invalid move generated", this_move);
            }
            make_move(side_to_move, this_move, 1, &mut (g_state.moves), &mut (g_state.board), &mut (g_state.hash), &mut (g_state.flip_stack));
        } else {
            /* No more move to make, only update the player to move */
            side_to_move = 0 + 2 - side_to_move
        }
        i += 1
    }
    if last_move_number == move_count_0 {
        /* No cutoff applies */
        let mut black_count: i32 = 0;
        let mut white_count: i32 = 0;
        black_count = disc_count(0, &(g_state.board).board);
        white_count = disc_count(2, &(g_state.board).board);
        if black_count > white_count {
            outcome = 64 - 2 * white_count
        } else if white_count > black_count {
            outcome = 2 * black_count - 64
        } else {
            outcome = 0
        }
    } else {
        generate_all(side_to_move, &mut (g_state.moves), &(g_state.search), &(g_state.board).board);
        determine_hash_values(side_to_move, &(g_state.board).board, &mut (g_state.hash));
        if echo != 0 {
            write!(stdout, "\n");
            if side_to_move == 0 {
                write!(stdout, "Full solving with {} empty (black)\n", 60 - (g_state.moves).disks_played);
            } else {
                write!(stdout, "Full solving with {} empty (white)\n", 60 - (g_state.moves).disks_played);
            }
        }
        end_game::<FE>(side_to_move, 0, 0, 1, 0,
                       &mut dummy_info, echo , g_state);
        outcome = (g_state.search).root_eval;
        if side_to_move == 2 {
            outcome = -outcome
        }
    }
    (*(g_state.g_book).node.offset(this_node as isize)).black_minimax_score = outcome as i16;
    (*(g_state.g_book).node.offset(this_node as isize)).white_minimax_score = outcome as i16;
    if outcome > 0 {
        let ref mut fresh27 = (*(g_state.g_book).node.offset(this_node as isize)).black_minimax_score;
        *fresh27 = (*fresh27 as i32 + 30000) as i16;
        let ref mut fresh28 = (*(g_state.g_book).node.offset(this_node as isize)).white_minimax_score;
        *fresh28 = (*fresh28 as i32 + 30000) as i16
    }
    if outcome < 0 {
        let ref mut fresh29 = (*(g_state.g_book).node.offset(this_node as isize)).black_minimax_score;
        *fresh29 = (*fresh29 as i32 - 30000) as i16;
        let ref mut fresh30 = (*(g_state.g_book).node.offset(this_node as isize)).white_minimax_score;
        *fresh30 = (*fresh30 as i32 - 30000) as i16
    }
    let ref mut fresh31 = (*(g_state.g_book).node.offset(this_node as isize)).flags;
    *fresh31 = (*fresh31 as i32 | 16) as u16;
    /* Take another pass through the midgame to update move
       alternatives and minimax information if requested. */
    if update_path != 0 {
        prepare_tree_traversal(g_state);
        i = 0;
        while i < last_move_number {
            this_move = abs(*game_move_list.unwrap_or(&g_state.learn.game_move).offset(i as isize) as i32) as i8;
            if *game_move_list.unwrap_or(&g_state.learn.game_move).offset(i as isize) > 0 {
                side_to_move = 0
            } else {
                side_to_move = 2
            }
            if generate_specific(this_move, side_to_move, &(g_state.board).board) == 0 {
                fatal_error!("{}: {}\n", "Invalid move generated", this_move);
            }
            make_move(side_to_move, this_move, 1, &mut (g_state.moves), &mut (g_state.board), &mut (g_state.hash), &mut (g_state.flip_stack));
            i += 1
        }
        if echo != 0 { stdout.flush(); }
        midgame_eval_done = 0;
        i = last_move_number - 1;
        while i >= 0 {
            this_move = abs(*game_move_list.unwrap_or(&g_state.learn.game_move).offset(i as isize) as i32) as i8;
            if *game_move_list.unwrap_or(&g_state.learn.game_move).offset(i as isize) as i32 > 0 {
                side_to_move = 0
            } else {
                side_to_move = 2
            }
            unmake_move(side_to_move, this_move, &mut (g_state.board).board, &mut (g_state.moves), &mut (g_state.hash), &mut (g_state.flip_stack));

            /* If the game was public, make sure that all nodes that
            previously marked as private nodes are marked as public. */
            this_node = visited_node[i as usize];
            if private_game == 0 && (*(g_state.g_book).node.offset(this_node as isize)).flags as i32 & 32 != 0 {
                let ref mut fresh32 = (*(g_state.g_book).node.offset(this_node as isize)).flags;
                *fresh32 = (*fresh32 as i32 ^ 32) as u16
            }
            if (*(g_state.g_book).node.offset(this_node as isize)).flags as i32 & 1 != 0 {
                side_to_move = 0
            } else {
                side_to_move = 2
            }
            generate_all(side_to_move, &mut (g_state.moves), &(g_state.search), &(g_state.board).board);
            determine_hash_values(side_to_move, &(g_state.board).board, &mut (g_state.hash));
            if (g_state.moves).disks_played >= 60 - max_full_solve {
                /* Only solve the position if it hasn't been solved already */
                if (*(g_state.g_book).node.offset(this_node as isize)).flags as i32 & 16 == 0 {
                    end_game::<FE>(side_to_move, 0, 0, 1, 0,
                                   &mut dummy_info, echo, g_state);
                    if side_to_move == 0 {
                        outcome = (g_state.search).root_eval
                    } else {
                        outcome = -(g_state.search).root_eval
                    }
                    (*(g_state.g_book).node.offset(this_node as isize)).black_minimax_score = outcome as i16;
                    (*(g_state.g_book).node.offset(this_node as isize)).white_minimax_score = outcome as i16;
                    if outcome > 0 {
                        let ref mut fresh33 = (*(g_state.g_book).node.offset(this_node as isize)).black_minimax_score;
                        *fresh33 = (*fresh33 as i32 + 30000) as i16;
                        let ref mut fresh34 = (*(g_state.g_book).node.offset(this_node as isize)).white_minimax_score;
                        *fresh34 = (*fresh34 as i32 + 30000) as i16
                    }
                    if outcome < 0 {
                        let ref mut fresh35 = (*(g_state.g_book).node.offset(this_node as isize)).black_minimax_score;
                        *fresh35 = (*fresh35 as i32 - 30000) as i16;
                        let ref mut fresh36 = (*(g_state.g_book).node.offset(this_node as isize)).white_minimax_score;
                        *fresh36 = (*fresh36 as i32 - 30000) as i16
                    }
                    let ref mut fresh37 = (*(g_state.g_book).node.offset(this_node as isize)).flags;
                    *fresh37 = (*fresh37 as i32 | 16) as u16
                }
            } else if (g_state.moves).disks_played >= 60 - max_wld_solve {
                /* Only solve the position if its WLD status is unknown */
                if (*(g_state.g_book).node.offset(this_node as isize)).flags as i32 & 4 == 0 {
                    end_game::<FE>(side_to_move, 1, 0, 1, 0,
                                   &mut dummy_info, echo , g_state);
                    if side_to_move == 0 {
                        outcome = (g_state.search).root_eval
                    } else {
                        outcome = -(g_state.search).root_eval
                    }
                    (*(g_state.g_book).node.offset(this_node as isize)).black_minimax_score = outcome as i16;
                    (*(g_state.g_book).node.offset(this_node as isize)).white_minimax_score = outcome as i16;
                    if outcome > 0 {
                        let ref mut fresh38 = (*(g_state.g_book).node.offset(this_node as isize)).black_minimax_score;
                        *fresh38 = (*fresh38 as i32 + 30000) as i16;
                        let ref mut fresh39 = (*(g_state.g_book).node.offset(this_node as isize)).white_minimax_score;
                        *fresh39 = (*fresh39 as i32 + 30000) as i16
                    }
                    if outcome < 0 {
                        let ref mut fresh40 = (*(g_state.g_book).node.offset(this_node as isize)).black_minimax_score;
                        *fresh40 = (*fresh40 as i32 - 30000) as i16;
                        let ref mut fresh41 = (*(g_state.g_book).node.offset(this_node as isize)).white_minimax_score;
                        *fresh41 = (*fresh41 as i32 - 30000) as i16
                    }
                    let ref mut fresh42 = (*(g_state.g_book).node.offset(this_node as isize)).flags;
                    *fresh42 = (*fresh42 as i32 | 4) as u16
                }
            } else {
                force_eval = (i >= first_new_node - 1 ||
                    (*(g_state.g_book).node.offset(this_node as isize)).best_alternative_move as i32
                        == abs(*game_move_list.unwrap_or(&g_state.learn.game_move).offset(i as isize) as i32)) as i32;
                if midgame_eval_done == 0 {
                    write!(stdout, "Evaluating: ");
                    stdout.flush();
                }
                midgame_eval_done = 1;
                if force_eval != 0 { clear_node_depth(this_node, &mut (g_state.g_book)); }
                evaluate_node(this_node, echo, g_state);
                write!(stdout, "|");
                stdout.flush();
            }
            let ref mut fresh43 = (*(g_state.g_book).node.offset(this_node as isize)).flags;
            *fresh43 = (*fresh43 as i32 | 8) as u16;
            do_minimax(this_node, &mut dummy_black_score, &mut dummy_white_score, g_state);
            if (*(g_state.g_book).node.offset(this_node as isize)).flags & 4 == 0 &&
                (*(g_state.g_book).node.offset(this_node as isize)).best_alternative_move == -1 &&
                (*(g_state.g_book).node.offset(this_node as isize)).alternative_score == 9999 {
                /* Minimax discovered that the g_book.node hasn't got a deviation any
                   longer because that move has been played. */
                evaluate_node(this_node, echo, g_state);
                write!(stdout, "-|-");
                do_minimax(this_node, &mut dummy_black_score, &mut dummy_white_score, g_state);
            }
            i -= 1
        }
        write!(stdout, "\n");
    }
    echo = stored_echo;
    (g_state.g_book).total_game_count += 1;
}
/*
   READ_TEXT_DATABASE
   Reads an existing ASCII database file.
*/

pub unsafe fn read_text_database(file_name: *const i8, g_book: &mut Book) {
    let mut magic1: i32 = 0;
    let mut magic2: i32 = 0;
    let mut new_book_node_count: i32 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    time(&mut start_time);
    write!(stdout, "Reading text opening database... ");
    stdout.flush();
    let stream = fopen(file_name, b"r\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error!("{} '{}'\n","Could not open database file", &CStr::from_ptr(file_name).to_str().unwrap());
    }
    fscanf(stream.file(), b"%d\x00" as *const u8 as *const i8, &mut magic1 as *mut i32);
    fscanf(stream.file(), b"%d\x00" as *const u8 as *const i8, &mut magic2 as *mut i32);
    if magic1 != 2718 || magic2 != 2818 {
        fatal_error!("{}: {}", "Wrong checksum, might be an old version\x00" , &CStr::from_ptr(file_name).to_str().unwrap());
    }
    fscanf(stream.file(), b"%d\x00" as *const u8 as *const i8, &mut new_book_node_count as *mut i32);
    set_allocation(new_book_node_count + 1000, g_book);
    let mut i = 0;
    while i < new_book_node_count {
        fscanf(stream.file(), b"%d %d %hd %hd %hd %hd %hd\n\x00" as *const u8 as *const i8,
               &mut (*g_book.node.offset(i as isize)).hash_val1 as *mut i32,
               &mut (*g_book.node.offset(i as isize)).hash_val2 as *mut i32,
               &mut (*g_book.node.offset(i as isize)).black_minimax_score as *mut i16,
               &mut (*g_book.node.offset(i as isize)).white_minimax_score as *mut i16,
               &mut (*g_book.node.offset(i as isize)).best_alternative_move as *mut i16,
               &mut (*g_book.node.offset(i as isize)).alternative_score as *mut i16,
               &mut (*g_book.node.offset(i as isize)).flags as *mut u16);
        i += 1
    }
    g_book.book_node_count = new_book_node_count;
    create_hash_reference(g_book);
    fclose(stream);
    time(&mut stop_time);
    write!(stdout, "done (took {} s)\n", (stop_time - start_time) as i32);
    write!(stdout, "\n");
}
/*
   READ_BINARY_DATABASE
   Reads a binary database file.
*/

pub unsafe fn read_binary_database(file_name_: *const i8, g_book: &mut Book) -> Option<()> {
    let file_name = CStr::from_ptr(file_name_).to_str().unwrap();

    let mut new_book_node_count: i32 = 0;
    let mut magic1: i16 = 0;
    let mut magic2: i16 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    time(&mut start_time);
    write!(stdout, "Reading binary opening database... ");
    stdout.flush();

    let stream = std::fs::read(file_name);
    if let Err(_) = stream {
        fatal_error!("{} '{}'\n", "Could not open database file", file_name);
    }
    struct Reader<T> {
        inner: T
    }
    let mut stream = Reader {
        inner: stream.unwrap().into_iter()
    };
    trait Parse<T>{
        fn parse(&mut self) -> Option<T>;
    }
    impl<T: Iterator<Item=u8>> Parse<i32> for Reader<T> {
        fn parse(&mut self) -> Option<i32> {
            Some(i32::from_le_bytes([self.inner.next()?, self.inner.next()?, self.inner.next()?, self.inner.next()?]))
        }
    }
    impl<T: Iterator<Item=u8>> Parse<i16> for Reader<T> {
        fn parse(&mut self) -> Option<i16> {
            Some(i16::from_le_bytes([self.inner.next()?, self.inner.next()?]))
        }
    }
    impl<T: Iterator<Item=u8>> Parse<u16> for Reader<T> {
        fn parse(&mut self) -> Option<u16> {
            Some(u16::from_le_bytes([self.inner.next()?, self.inner.next()?]))
        }
    }

    magic1 = stream.parse().unwrap_or(0);
    magic2 = stream.parse().unwrap_or(0);

    if magic1 as i32 != 2718 || magic2 as i32 != 2818 {
        fatal_error!("{}: {}", "Wrong checksum, might be an old version", file_name);
    }

    new_book_node_count = stream.parse()?;
    set_allocation(new_book_node_count + 1000, g_book);
    let mut i = 0;
    while i < new_book_node_count as usize {
        let node = &mut g_book.node[i];
        node.hash_val1 = stream.parse()?;
        node.hash_val2 = stream.parse()?;
        node.black_minimax_score = stream.parse()?;
        node.white_minimax_score = stream.parse()?;
        node.best_alternative_move = stream.parse()?;
        node.alternative_score = stream.parse()?;
        node.flags = stream.parse()?;
        i += 1
    }

    g_book.book_node_count = new_book_node_count;
    create_hash_reference(g_book);
    time(&mut stop_time);
    write!(stdout, "done (took {} s)\n", (stop_time - start_time) as i32);
    Some(())
}

/*
   WRITE_TEXT_DATABASE
   Writes the database to an ASCII file.
*/

pub unsafe fn write_text_database(file_name: *const i8, g_book: &mut Book) {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    time(&mut start_time);
    write!(stdout, "Writing text database... ");
    stdout.flush();
    let mut stream = fopen(file_name, b"w\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error!("{} '{}'\n", "Could not create database file", CStr::from_ptr(file_name).to_str().unwrap());
    }
    write!(stream, "{}\n{}\n", 2718, 2818);
    write!(stream, "{}\n", g_book.book_node_count);
    let mut i = 0;
    while i < g_book.book_node_count {
        let node = &g_book.node[i as usize];
        write!(stream,
               "{} {} {} {} {} {} {}\n", node.hash_val1,
               node.hash_val2,
               node.black_minimax_score as i32,
               node.white_minimax_score as i32,
               node.best_alternative_move as i32,
               node.alternative_score as i32,
               node.flags as i32);
        i += 1
    }
    fclose(stream);
    time(&mut stop_time);
    write!(stdout, "done (took {} s)\n", stop_time - start_time);
    write!(stdout, "\n");
}
/*
   WRITE_BINARY_DATABASE
   Writes the database to a binary file.
*/

pub unsafe fn write_binary_database(file_name: *const i8, mut g_book: &mut Book) {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    time(&mut start_time);
    write!(stdout, "Writing binary database... ");
    stdout.flush();
    let mut stream = fopen(file_name, b"wb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error!("{} '{}'\n", "Could not create database file", CStr::from_ptr(file_name).to_str().unwrap());
    }
    let mut magic = 2718_i16;
    stream.write(&magic.to_le_bytes());
    let mut magic = 2818_i16;
    stream.write(&magic.to_le_bytes());
    stream.write(&g_book.book_node_count.to_le_bytes());
    let mut i = 0;
    while i < g_book.book_node_count {
        let node = &g_book.node[i as usize];
        stream.write(&node.hash_val1.to_le_bytes());
        stream.write(&node.hash_val2.to_le_bytes());
        stream.write(&node.black_minimax_score.to_le_bytes());
        stream.write(&node.white_minimax_score.to_le_bytes());
        stream.write(&node.best_alternative_move.to_le_bytes());
        stream.write(&node.alternative_score.to_le_bytes());
        stream.write(&node.flags.to_le_bytes());
        i += 1
    }
    fclose(stream);
    time(&mut stop_time);
    write!(stdout, "done (took {} s)\n",
           (stop_time - start_time) as i32);
    write!(stdout, "\n");
}

/*
   PRINT_MOVE_ALTERNATIVES
   Displays all available book moves from a position.
   FLAGS specifies a subset of the flag bits which have to be set
   for a position to be considered. Notice that FLAGS=0 accepts
   any flag combination.
*/

pub fn print_move_alternatives(side_to_move: i32, board_state: &BoardState, g_book: &mut Book) {
    let mut i: i32 = 0;
    let mut sign: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut score: i32 = 0;
    let mut output_score: i32 = 0;
    if g_book.candidate_count > 0 {
        if side_to_move == 0 {
            sign = 1
        } else {
            sign = -1
        }
        get_hash(&mut val1, &mut val2, &mut orientation, &g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, g_book);
        /* Check that the position is in the opening book after all */
        if slot == -1 || *g_book.book_hash_table.offset(slot as isize) == -1 {
            return
        }
        /* Pick the book score corresponding to the player to move and
           remove draw avoidance and the special scores for nodes WLD. */
        if side_to_move == 0 {
            score = (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as isize)).black_minimax_score as i32
        } else {
            score = (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as isize)).white_minimax_score as i32
        }
        if score == 30000 - 1 || score == -(30000 - 1) {
            score = 0
        }
        if score > 30000 {
            score -= 30000
        }
        if score < -(30000) {
            score += 30000
        }
        write!(stdout, "Book score is ");
        if (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as isize)).flags & 16 != 0 {
            write!(stdout, "{:+} (exact score).", sign * score);
        } else if (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as isize)).flags  & 4 != 0 {
            write!(stdout, "{:+} (W/L/D solved).", sign * score);
        } else {
            write!(stdout, "{:+.2}.", (sign * score) as f64 / 128.0f64);
        }
        if (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as isize)).flags & 32 != 0 {
            write!(stdout, " Private node.");
        }
        write!(stdout, "\n");
        i = 0;
        while i < g_book.candidate_count {
            write!(stdout, "   {}   ", TO_SQUARE(g_book.candidate_list[i as usize].move_0));
            output_score = g_book.candidate_list[i as usize].score;
            if output_score >= 30000 {
                output_score -= 30000
            } else if output_score <= -(30000) {
                output_score += 30000
            }
            if g_book.candidate_list[i as usize].flags & 16 != 0 {
                write!(stdout, "{:<+6}  (exact score)", output_score);
            } else if g_book.candidate_list[i as usize].flags & 4 != 0 {
                write!(stdout, "{:<+6}  (W/L/D solved)", output_score);
            } else {
                write!(stdout, "{:<+6.2}", output_score as f64 / 128.0f64);
                if g_book.candidate_list[i as usize].flags & 64 != 0 {
                    write!(stdout, "  (deviation)");
                }
            }
            write!(stdout, "\n");
            i += 1
        }
    };
}

/*
   INIT_OSF
   Makes sure all data structures are initialized.
*/

pub unsafe fn init_osf(do_global_setup: i32, g_state: &mut FullState) {
    engine_init_osf(g_state);
    if do_global_setup != 0 {
        global_setup(0, 19, g_state);
    };
}


/*
   EVALUATE_NODE
   Applies a search to a predetermined depth to find the best
   alternative move in a position.
   Note: This function assumes that generate_all() has been
         called prior to it being called.
*/
pub fn evaluate_node(index: i32, echo: i32, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut alternative_move_count: i32 = 0;
    let mut this_move = 0;
    let mut best_move = 0;
    let mut child: i32 = 0;
    let mut allow_mpc: i32 = 0;
    let mut depth: i32 = 0;
    let mut best_index: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut feasible_move: [i8; 64] = [0; 64];
    let mut best_score: i32 = 0;
    /* Don't evaluate nodes that already have been searched deep enough */
    depth = get_node_depth(index, &mut ( g_state.g_book));
    if depth >= ( g_state.g_book).search_depth &&
        (*( g_state.g_book).node.offset(index as isize)).alternative_score as i32 !=
            9999 {
        return
    }
    /* If the g_book.node has been evaluated and its score is outside the
       eval and minimax windows, bail out. */
    if (*( g_state.g_book).node.offset(index as isize)).alternative_score as i32 !=
        9999 {
        if abs((*( g_state.g_book).node.offset(index as isize)).alternative_score as
            i32) < ( g_state.g_book).min_eval_span ||
            abs((*( g_state.g_book).node.offset(index as isize)).alternative_score as
                i32) > ( g_state.g_book).max_eval_span {
            return
        }
        if abs((*( g_state.g_book).node.offset(index as isize)).black_minimax_score as
            i32) < ( g_state.g_book).min_negamax_span ||
            abs((*( g_state.g_book).node.offset(index as isize)).black_minimax_score as
                i32) > ( g_state.g_book).max_negamax_span {
            return
        }
    }
    if (*( g_state.g_book).node.offset(index as isize)).flags as i32 & 1
        != 0 {
        side_to_move = 0
    } else { side_to_move = 2 }
    remove_coeffs(( g_state.moves).disks_played - 8, &mut ( g_state.coeff));
    ( g_state.timer).clear_panic_abort();
    ( g_state.board).piece_count[0][( g_state.moves).disks_played as usize] =
        disc_count(0, &( g_state.board).board);
    ( g_state.board).piece_count[2][( g_state.moves).disks_played as usize] =
        disc_count(2, &( g_state.board).board);
    /* Find the moves which haven't been tried from this position */
    alternative_move_count = 0;
    i = 0;
    while i < ( g_state.moves).move_count[( g_state.moves).disks_played as usize] {
        this_move = ( g_state.moves).move_list[( g_state.moves).disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1, &mut ( g_state.moves), &mut ( g_state.board), &mut ( g_state.hash), &mut ( g_state.flip_stack));
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut ( g_state.g_book), &( g_state.board).board);
        slot = probe_hash_table(val1, val2, &mut ( g_state.g_book));
        child = *( g_state.g_book).book_hash_table.offset(slot as isize);
        if child == -1 {
            let fresh16 = alternative_move_count;
            alternative_move_count = alternative_move_count + 1;
            feasible_move[fresh16 as usize] = this_move
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut ( g_state.board).board, &mut ( g_state.moves), &mut ( g_state.hash), &mut ( g_state.flip_stack));
        };
        i += 1
    }
    if alternative_move_count == 0 {
        /* There weren't any such moves */
        ( g_state.g_book).exhausted_node_count += 1;
        (*( g_state.g_book).node.offset(index as isize)).best_alternative_move =
            -(2) as i16;
        (*( g_state.g_book).node.offset(index as isize)).alternative_score =
            9999 as i16
    } else {
        /* Find the best of those moves */
        allow_mpc = (( g_state.g_book).search_depth >= 9) as i32;
        nega_scout::<FE>(( g_state.g_book).search_depth, allow_mpc, side_to_move,
                         alternative_move_count, &mut feasible_move,
                         -(12345678), 12345678,
                         &mut best_score, &mut best_index, echo, g_state);
        best_move = feasible_move[best_index as usize];
        ( g_state.g_book).evaluated_count += 1;
        if side_to_move == 0 {
            (*( g_state.g_book).node.offset(index as isize)).alternative_score =
                best_score as i16
        } else {
            (*( g_state.g_book).node.offset(index as isize)).alternative_score =
                -best_score as i16
        }
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut ( g_state.g_book), &( g_state.board).board);
        (*( g_state.g_book).node.offset(index as isize)).best_alternative_move =
            *( g_state.g_book).symmetry_map[orientation as usize].offset(best_move as isize) as
                i16
    }
    clear_node_depth(index, &mut ( g_state.g_book));
    set_node_depth(index, ( g_state.g_book).search_depth, &mut ( g_state.g_book));
}

/*
  NEGA_SCOUT
  This wrapper on top of TREE_SEARCH is used by EVALUATE_NODE
  to search the possible deviations.
*/
pub fn nega_scout<FE: FrontEnd>(depth: i32,
                                       allow_mpc: i32,
                                       side_to_move: i32,
                                       allowed_count: i32,
                                       allowed_moves: &mut [i8],
                                       _alpha: i32, _beta: i32,
                                       best_score: &mut i32,
                                       best_index: &mut i32, echo:i32, g_state: &mut FullState) {
    let mut midgame_state = (&mut g_state.midgame);
    let mut coeff_state = (&mut g_state.coeff);
    let mut g_timer = (&mut g_state.timer);
    let mut moves_state = (&mut g_state.moves);
    let mut board_state = (&mut g_state.board);
    let mut hash_state = (&mut g_state.hash);
    let mut prob_cut = (&mut g_state.prob_cut);
    let mut search_state = (&mut g_state.search);
    let mut flip_stack_ = (&mut g_state.flip_stack);

    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut curr_alpha: i32 = 0;
    let mut curr_depth: i32 = 0;
    let mut low_score: i32 = 0;
    let mut high_score: i32 = 0;
    let mut best_move = 0;
    let mut current_score: i32 = 0;
    reset_counter(&mut search_state.nodes);
    low_score = -(12345678);
    /* To avoid spurious hash table entries to take out the effect
       of the averaging done, the hash table drafts are changed prior
       to each g_book.node being searched. */
    clear_hash_drafts(&mut hash_state);
    determine_hash_values(side_to_move, &board_state.board, &mut hash_state);
    /* First determine the best move in the current position
       and its score when searched to depth DEPTH.
       This is done using standard negascout with iterative deepening. */
    curr_depth = 2 - depth % 2;
    while curr_depth <= depth {
        low_score = -(12345678);
        curr_alpha = -(12345678);
        i = 0;
        while i < allowed_count {
            make_move(side_to_move, *allowed_moves.offset(i as isize),
                      1, &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
            board_state.piece_count[0][moves_state.disks_played as usize] =
                disc_count(0, &board_state.board);
            board_state.piece_count[2][moves_state.disks_played as usize] =
                disc_count(2, &board_state.board);
            g_timer.last_panic_check = 0.0f64;
            if i == 0 {
                current_score =
                    -tree_search::<FE>(1, curr_depth,
                                       0 + 2 -
                                           side_to_move, -(12345678),
                                       12345678, 1,
                                       allow_mpc, 1, echo,  &mut moves_state ,
                                       &mut search_state ,
                                       &mut board_state ,
                                       &mut hash_state,
                                       &mut flip_stack_,
                                       &mut coeff_state,
                                       &mut prob_cut ,&mut g_timer, &mut midgame_state);
                low_score = current_score;
                *best_index = i
            } else {
                curr_alpha =
                    if low_score > curr_alpha {
                        low_score
                    } else { curr_alpha };
                current_score =
                    -tree_search::<FE>(1, curr_depth,
                                       0 + 2 -
                                           side_to_move,
                                       -(curr_alpha + 1),
                                       -curr_alpha, 1, allow_mpc,
                                       1, echo,  &mut moves_state ,
                                       &mut search_state ,
                                       &mut board_state ,
                                       &mut hash_state,
                                       &mut flip_stack_,
                                       &mut coeff_state,
                                       &mut prob_cut ,&mut g_timer, &mut midgame_state);
                if current_score > curr_alpha {
                    current_score =
                        -tree_search::<FE>(1, curr_depth,
                                           0 + 2 -
                                               side_to_move,
                                           -(12345678),
                                           12345678,
                                           1, allow_mpc,
                                           1, echo,  &mut moves_state ,
                                           &mut search_state ,
                                           &mut board_state ,
                                           &mut hash_state,
                                           &mut flip_stack_,
                                           &mut coeff_state,
                                           &mut prob_cut ,&mut g_timer, &mut midgame_state);
                    if current_score > low_score {
                        low_score = current_score;
                        *best_index = i
                    }
                } else if current_score > low_score {
                    low_score = current_score;
                    *best_index = i
                }
            }
            let move_0 = *allowed_moves.offset(i as isize);
            {
                unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
            };
            i += 1
        }
        /* Float the best move so far to the top of the list */
        best_move = *allowed_moves.offset(*best_index as isize);
        j = *best_index;
        while j >= 1 {
            *allowed_moves.offset(j as isize) =
                *allowed_moves.offset((j - 1) as isize);
            j -= 1
        }
        allowed_moves[0] = best_move;
        *best_index = 0;
        curr_depth += 2
    }
    /* Then find the score for the best move when searched
       to depth DEPTH+1 */
    make_move(side_to_move, *allowed_moves.offset(*best_index as isize),
              1, &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
    board_state.piece_count[0][moves_state.disks_played as usize] =
        disc_count(0, &board_state.board);
    board_state.piece_count[2][moves_state.disks_played as usize] =
        disc_count(2, &board_state.board);
    g_timer.last_panic_check = 0.0f64;
    high_score =
        -tree_search::<FE>(1, depth + 1,
                           0 + 2 - side_to_move,
                           -(12345678), 12345678,
                           1, allow_mpc, 1, echo,  &mut moves_state ,
                           &mut search_state ,
                           &mut board_state ,
                           &mut hash_state,
                           &mut flip_stack_,
                           &mut coeff_state,
                           &mut prob_cut ,&mut g_timer, &mut midgame_state);
    let move_0 = *allowed_moves.offset(*best_index as isize);
    {
        unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
    };
    /* To remove the oscillations between odd and even search depths
       the score for the deviation is the average between the two scores. */
    *best_score = (low_score + high_score) / 2;
}



/*
   REBUILD_HASH_TABLE
   Resize the hash table for a requested number of nodes.
*/
pub fn rebuild_hash_table(book: &mut Book, requested_items: i32) {
    let new_size = 2 * requested_items;
    if book.hash_table_size == 0 {
        book.book_hash_table = vec![0; new_size as usize];
    } else {
        book.book_hash_table.resize(new_size as usize, 0);
    }
    book.hash_table_size = new_size;
    create_hash_reference(book);
}



/*
   SET_ALLOCATION
   Changes the number of nodes for which memory is allocated.
*/
pub fn set_allocation(size: i32, book: &mut Book) {
    if book.node.is_empty() {
        book.node = vec![BookNode {
            hash_val1: 0,
            hash_val2: 0,
            black_minimax_score: 0,
            white_minimax_score: 0,
            best_alternative_move: 0,
            alternative_score: 0,
            flags: 0
        }; size as usize];
    } else {
        book.node.resize(size as usize, BookNode {
            hash_val1: 0,
            hash_val2: 0,
            black_minimax_score: 0,
            white_minimax_score: 0,
            best_alternative_move: 0,
            alternative_score: 0,
            flags: 0
        });
    }
    book.node_table_size = size;
    if book.node_table_size as f64 > 0.80f64 * book.hash_table_size as f64 {
        rebuild_hash_table(book, book.node_table_size);
    };
}
/*
   INCREASE_ALLOCATION
   Allocate more memory for the book tree.
*/
pub fn increase_allocation(book: &mut Book) {
    set_allocation(book.node_table_size + 50000, book);
}
/*
   CREATE_BOOK_NODE
   Creates a new book node without any connections whatsoever
   to the rest of the tree.
*/
pub fn create_BookNode(val1: i32, val2: i32, flags: u16, book: &mut Book) -> i32 {
    let mut index: i32 = 0;
    if book.book_node_count == book.node_table_size { increase_allocation(book); }
    index = book.book_node_count;
    (*book.node.offset(index as isize)).hash_val1 = val1;
    (*book.node.offset(index as isize)).hash_val2 = val2;
    (*book.node.offset(index as isize)).black_minimax_score = 9999;
    (*book.node.offset(index as isize)).white_minimax_score = 9999;
    (*book.node.offset(index as isize)).best_alternative_move =
        -1 as i16;
    (*book.node.offset(index as isize)).alternative_score = 9999;
    (*book.node.offset(index as isize)).flags = flags;
    select_hash_slot(index, book);
    book.book_node_count += 1;
    return index;
}

/*
   DO_MINIMAX
   Calculates the minimax value of g_book.node INDEX.
*/
pub fn do_minimax(index: i32, black_score: &mut i32,
                  white_score: &mut i32, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut child_black_score: i32 = 0;
    let mut child_white_score: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move = 0;
    let mut alternative_move = 0;
    let mut alternative_move_found: i32 = 0;
    let mut child_count: i32 = 0;
    let mut best_black_child_val: i32 = 0;
    let mut best_white_child_val: i32 = 0;
    let mut worst_black_child_val: i32 = 0;
    let mut worst_white_child_val: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut best_black_score: i16 = 0;
    let mut best_white_score: i16 = 0;
    /* If the node has been visited AND it is a midgame node, meaning
       that the minimax values are not to be tweaked, return the
       stored values. */
    if (*(g_state.g_book).node.offset(index as isize)).flags as i32 & 8 == 0 {
        if (*(g_state.g_book).node.offset(index as isize)).flags as i32 & (4 | 16) == 0 {
            *black_score = (*(g_state.g_book).node.offset(index as isize)).black_minimax_score as i32;
            *white_score = (*(g_state.g_book).node.offset(index as isize)).white_minimax_score as i32;
            return
        }
    }
    /* Correct WLD solved nodes corresponding to draws to be represented
       as full solved and make sure full solved nodes are marked as
       WLD solved as well */
    if (*(g_state.g_book).node.offset(index as isize)).flags as i32 & 4 != 0 &&
        (*(g_state.g_book).node.offset(index as isize)).black_minimax_score as i32 == 0 &&
        (*(g_state.g_book).node.offset(index as isize)).white_minimax_score as i32 == 0 {
        let ref mut fresh2 = (*(g_state.g_book).node.offset(index as isize)).flags;
        *fresh2 = (*fresh2 as i32 | 16) as u16
    }
    if (*(g_state.g_book).node.offset(index as isize)).flags as i32 & 16 != 0 &&
        (*(g_state.g_book).node.offset(index as isize)).flags as i32 & 4 == 0 {
        let ref mut fresh3 = (*(g_state.g_book).node.offset(index as isize)).flags;
        *fresh3 = (*fresh3 as i32 | 4) as u16
    }
    /* Recursively minimax all children of the node */
    if (*(g_state.g_book).node.offset(index as isize)).flags as i32 & 1 != 0 {
        side_to_move = 0
    } else {
        side_to_move = 2
    }
    best_black_child_val = -99999;
    best_white_child_val = -99999;
    worst_black_child_val = 99999;
    worst_white_child_val = 99999;
    if (*(g_state.g_book).node.offset(index as isize)).alternative_score as i32 != 9999 {
        best_black_score = adjust_score((*(g_state.g_book).node.offset(index as isize)).alternative_score as
                             i32, side_to_move, &mut (g_state.g_book), ( g_state.moves).disks_played) as i16;
        best_white_score = best_black_score;
        worst_black_child_val = best_black_score as i32;
        best_black_child_val = worst_black_child_val;
        worst_white_child_val = best_white_score as i32;
        best_white_child_val = worst_white_child_val;
        alternative_move_found = 0;
        alternative_move = (*(g_state.g_book).node.offset(index as isize)).best_alternative_move as i8;
        if alternative_move > 0 {
            get_hash(&mut val1, &mut val2, &mut orientation, &mut (g_state.g_book), &( g_state.board).board);
            alternative_move = *(g_state.g_book).inv_symmetry_map[orientation as usize].offset(alternative_move as isize)
        }
    } else {
        alternative_move_found = 1;
        alternative_move = 0;
        if side_to_move == 0 {
            best_black_score = -32000;
            best_white_score = -32000
        } else {
            best_black_score = 32000;
            best_white_score = 32000
        }
    }
    generate_all(side_to_move, &mut ( g_state.moves), &(&g_state.search), &( g_state.board).board);
    child_count = 0;
    i = 0;
    while i < ( g_state.moves).move_count[( g_state.moves).disks_played as usize] {
        ( g_state.board).piece_count[0][( g_state.moves).disks_played as usize] = disc_count(0, &( g_state.board).board);
        ( g_state.board).piece_count[2][( g_state.moves).disks_played as usize] = disc_count(2, &( g_state.board).board);
        this_move = ( g_state.moves).move_list[( g_state.moves).disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1, &mut ( g_state.moves), &mut ( g_state.board), &mut (g_state.hash), &mut ( g_state.flip_stack));
        get_hash(&mut val1, &mut val2, &mut orientation, &mut (g_state.g_book), &( g_state.board).board);
        slot = probe_hash_table(val1, val2, &mut (g_state.g_book));
        child = *(g_state.g_book).book_hash_table.offset(slot as isize);
        if child != -1 {
            do_minimax(child, &mut child_black_score, &mut child_white_score, g_state);
            best_black_child_val = if best_black_child_val > child_black_score { best_black_child_val } else { child_black_score };
            best_white_child_val = if best_white_child_val > child_white_score { best_white_child_val } else { child_white_score };
            worst_black_child_val = if worst_black_child_val < child_black_score { worst_black_child_val } else { child_black_score };
            worst_white_child_val = if worst_white_child_val < child_white_score { worst_white_child_val } else { child_white_score };
            if side_to_move == 0 {
                best_black_score = if child_black_score > best_black_score as i32 { child_black_score } else { best_black_score as i32 } as i16;
                best_white_score = if child_white_score > best_white_score as i32 { child_white_score } else { best_white_score as i32 } as i16
            } else {
                best_black_score = if child_black_score < best_black_score as i32 { child_black_score } else { best_black_score as i32 } as i16;
                best_white_score = if child_white_score < best_white_score as i32 { child_white_score } else { best_white_score as i32 } as i16
            }
            child_count += 1
        } else if alternative_move_found == 0 && this_move == alternative_move {
            alternative_move_found = 1
        }
        unmake_move(side_to_move, this_move, &mut (g_state.board).board, &mut (g_state.moves), &mut (g_state.hash), &mut (g_state.flip_stack));
        i += 1
    }
    if alternative_move_found == 0 {
        /* The was-to-be deviation now leads to a position in the database,
           hence it can no longer be used. */
        (*(g_state.g_book).node.offset(index as isize)).alternative_score = 9999;
        (*(g_state.g_book).node.offset(index as isize)).best_alternative_move = -1
    }
    /* Try to infer the WLD status from the children */
    if (*(g_state.g_book).node.offset(index as isize)).flags as i32 & (16 | 4) == 0 && child_count > 0 {
        if side_to_move == 0 {
            if best_black_child_val >= 30000 && best_white_child_val >= 30000 {
                /* Black win */
                let ref mut fresh4 = (*(g_state.g_book).node.offset(index as isize)).white_minimax_score;
                *fresh4 = if best_black_child_val < best_white_child_val {
                    best_black_child_val
                } else {
                    best_white_child_val
                } as i16;
                (*(g_state.g_book).node.offset(index as isize)).black_minimax_score = *fresh4;
                let ref mut fresh5 = (*(g_state.g_book).node.offset(index as isize)).flags;
                *fresh5 = (*fresh5 as i32 | 4) as u16
            } else if best_black_child_val <= -30000 && best_white_child_val <= -30000 {
                /* Black loss */
                let ref mut fresh6 = (*(g_state.g_book).node.offset(index as isize)).white_minimax_score;
                *fresh6 = if best_black_child_val > best_white_child_val {
                    best_black_child_val
                } else {
                    best_white_child_val
                } as i16;
                (*(g_state.g_book).node.offset(index as isize)).black_minimax_score = *fresh6;
                let ref mut fresh7 = (*(g_state.g_book).node.offset(index as isize)).flags;
                *fresh7 = (*fresh7 as i32 | 4) as u16
            }
        } else if worst_black_child_val <= -30000 && worst_white_child_val <= -30000 {
            /* White win */
            let ref mut fresh8 = (*(g_state.g_book).node.offset(index as isize)).white_minimax_score;
            *fresh8 = if worst_black_child_val > worst_white_child_val { worst_black_child_val } else { worst_white_child_val } as i16;
            (*(g_state.g_book).node.offset(index as isize)).black_minimax_score = *fresh8;
            let ref mut fresh9 = (*(g_state.g_book).node.offset(index as isize)).flags;
            *fresh9 = (*fresh9 as i32 | 4) as u16
        } else if worst_black_child_val >= 30000 && worst_white_child_val >= 30000 {
            /* White loss */
            let ref mut fresh10 = (*(g_state.g_book).node.offset(index as isize)).white_minimax_score;
            *fresh10 = if worst_black_child_val < worst_white_child_val {
                worst_black_child_val
            } else {
                worst_white_child_val
            } as i16;
            (*(g_state.g_book).node.offset(index as isize)).black_minimax_score = *fresh10;
            let ref mut fresh11 = (*(g_state.g_book).node.offset(index as isize)).flags;
            *fresh11 = (*fresh11 as i32 | 4) as u16
        }
    }
    /* Tweak the minimax scores for draws to give the right
       draw avoidance behavior */
    if (*(g_state.g_book).node.offset(index as isize)).flags as i32 &
        (16 | 4) != 0 {
        *black_score = (*(g_state.g_book).node.offset(index as isize)).black_minimax_score as i32;
        *white_score = (*(g_state.g_book).node.offset(index as isize)).white_minimax_score as i32;
        if (*(g_state.g_book).node.offset(index as isize)).black_minimax_score as i32 == 0 && (*(g_state.g_book).node.offset(index as isize)).white_minimax_score as i32 == 0 {
            /* Is it a position in which a draw should be avoided? */if (g_state.g_book).game_mode as u32 == PRIVATE_GAME as i32 as u32 || (*(g_state.g_book).node.offset(index as isize)).flags as i32 & 32 == 0 {
                match (g_state.g_book).draw_mode as u32 {
                    1 => {
                        *black_score = 30000 - 1;
                        *white_score = 30000 - 1
                    }
                    2 => {
                        *black_score = -(30000 - 1);
                        *white_score = -(30000 - 1)
                    }
                    3 => {
                        *black_score = -(30000 - 1);
                        *white_score = 30000 - 1
                    }
                    0 | _ => {}
                }
            }
        }
    } else {
        let ref mut fresh12 = (*(g_state.g_book).node.offset(index as isize)).black_minimax_score;
        *fresh12 = best_black_score;
        *black_score = *fresh12 as i32;
        let ref mut fresh13 = (*(g_state.g_book).node.offset(index as isize)).white_minimax_score;
        *fresh13 = best_white_score;
        *white_score = *fresh13 as i32
    }
    let ref mut fresh14 = (*(g_state.g_book).node.offset(index as isize)).flags;
    *fresh14 = (*fresh14 as i32 ^ 8) as u16;
}



pub fn engine_init_osf(g_state :&mut FullState) {

    init_maps(&mut g_state.g_book); //FIXME why is this not called from zebra everytime in the engine?????
    let mut hash_state = (&mut g_state.hash);
    let mut random_instance = (&mut g_state.random);
    let mut g_book = (&mut g_state.g_book);

    prepare_hash(&mut g_book, &mut random_instance);
    setup_hash(1, &mut hash_state, &mut random_instance);
    init_book_tree(&mut g_book);
    reset_book_search(&mut g_book);
    g_book.search_depth = 2;
    g_book.max_slack = 0;
    g_book.low_deviation_threshold = 60;
    g_book.high_deviation_threshold = 60;
    g_book.deviation_bonus = 0.0f64;
    g_book.min_eval_span = 0;
    g_book.max_eval_span = 1000 * 128;
    g_book.min_negamax_span = 0;
    g_book.max_negamax_span = 1000 * 128;
    g_book.max_batch_size = 10000000;
    g_book.force_black = 0;
    g_book.force_white = 0;
}


/*
   PREPATE_TREE_TRAVERSAL
   Prepares all relevant data structures for a tree search
   or traversal.
*/
pub fn prepare_tree_traversal(g_state: &mut FullState) {
    let mut side_to_move: i32 = 0;
    setup_non_file_based_game(&mut side_to_move, &mut g_state.board, &mut g_state.hash, &mut g_state.moves);
    engine_game_init(g_state);
    g_state.midgame.toggle_midgame_hash_usage(1, 1);
    g_state.timer.toggle_abort_check(0);
    g_state.midgame.toggle_midgame_abort_check(0);
}



/*
   INIT_MAPS
   Initializes the 8 symmetry maps.
   Notice that the order of these MUST coincide with the returned
   orientation value from get_hash() OR YOU WILL LOSE BIG.
*/
pub fn init_maps(book: &mut Book) {
    book.symmetry_map[0] = &BOOK_MAPS.b1_b1_map;
    book.inv_symmetry_map[0] = &BOOK_MAPS.b1_b1_map;
    book.symmetry_map[1] = &BOOK_MAPS.g1_b1_map;
    book.inv_symmetry_map[1] = &BOOK_MAPS.g1_b1_map;
    book.symmetry_map[2] = &BOOK_MAPS.g8_b1_map;
    book.inv_symmetry_map[2] = &BOOK_MAPS.g8_b1_map;
    book.symmetry_map[3] = &BOOK_MAPS.b8_b1_map;
    book.inv_symmetry_map[3] = &BOOK_MAPS.b8_b1_map;
    book.symmetry_map[4] = &BOOK_MAPS.a2_b1_map;
    book.inv_symmetry_map[4] = &BOOK_MAPS.a2_b1_map;
    book.symmetry_map[5] = &BOOK_MAPS.a7_b1_map;
    book.inv_symmetry_map[5] = &BOOK_MAPS.h2_b1_map;
    book.symmetry_map[6] = &BOOK_MAPS.h7_b1_map;
    book.inv_symmetry_map[6] = &BOOK_MAPS.h7_b1_map;
    book.symmetry_map[7] = &BOOK_MAPS.h2_b1_map;
    book.inv_symmetry_map[7] = &BOOK_MAPS.a7_b1_map;
    let book = &book;
    let mut i = 0;
    let mut k = 0;
    while i < 8 {
        let mut j = 1;
        while j <= 8 {
            k = 1;
            while k <= 8 {
                let pos = 10 * j + k;
                if *book.inv_symmetry_map[i as usize]
                    .offset(*book.symmetry_map[i as usize].offset(pos as isize) as isize) as i32 != pos {
                    let symmetry_map_item = *book.inv_symmetry_map[i as usize].offset(*book.symmetry_map[i as usize].offset(pos as isize) as isize);
                    LibcFatalError::error_in_map(i, pos, symmetry_map_item as _);
                }
                k += 1
            }
            j += 1
        }
        i += 1
    };
}


/*
   INIT_BOOK_TREE
   Initializes the node tree by creating the root of the tree.
*/
pub fn init_book_tree(book: &mut Book) {
    book.book_node_count = 0;
    book.node = Vec::new();
}

/*
   PREPARE_HASH
   Compute the position hash codes.
*/
pub fn prepare_hash(book: &mut Book, random: &mut MyRandom) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    /* The hash keys are static, hence the same keys must be
       produced every time the program is run. */
    let x = 0;
    random.my_srandom(x);
    i = 0;
    while i < 2 {
        j = 0;
        while j < 8 {
            k = 0;
            while k < 6561 {
                book.line_hash[i][j][k] =
                    if random.my_random() % 2 as i64 != 0 {
                        random.my_random()
                    } else { -random.my_random() } as i32;
                k += 1
            }
            j += 1
        }
        i += 1
    }
    book.hash_table_size = 0;
}

/*
   CREATE_HASH_REFERENCEE
   Takes the node list and fills the hash table with indices
   into the node list.
*/
pub fn create_hash_reference(book: &mut Book) {
    let mut i = 0;
    while i < book.hash_table_size {
        *book.book_hash_table.offset(i as isize) = -1;
        i += 1
    }
    let mut i = 0;
    while i < book.book_node_count {
        select_hash_slot(i, book);
        i += 1
    };
}


/*
   SELECT_HASH_SLOT
   Finds a slot in the hash table for the node INDEX
   using linear probing.
*/
pub fn select_hash_slot(index: i32, book: &mut Book) {
    let mut slot: i32 = 0;
    slot = (*book.node.offset(index as isize)).hash_val1 % book.hash_table_size;
    while *book.book_hash_table.offset(slot as isize) != -1 {
        slot = (slot + 1) % book.hash_table_size
    }
    *book.book_hash_table.offset(slot as isize) = index;
}
