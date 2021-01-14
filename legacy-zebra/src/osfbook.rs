use engine::src::counter::reset_counter;
use engine::src::end::end_game;
use engine::src::error::FrontEnd;
use engine::src::game::{engine_game_init, setup_non_file_based_game};
use engine::src::getcoeff::remove_coeffs;
use engine::src::hash::{clear_hash_drafts, determine_hash_values, setup_hash};
use engine::src::midgame::{tree_search};
use engine::src::moves::{generate_all, generate_specific, make_move, make_move_no_hash, unmake_move, unmake_move_no_hash};
use engine::src::myrandom::MyRandom;
use engine::src::osfbook::{__time_t, _ISgraph, _ISprint, _ISspace, _ISupper, adjust_score, Book, BOOK_MAPS, BookNode, clear_node_depth, fill_move_alternatives, get_hash, get_node_depth, probe_hash_table, reset_book_search, set_node_depth, size_t};
use engine::src::search::disc_count;
use engine::src::stubs::{abs};
use engine::src::zebra::EvalResult::WON_POSITION;
use engine::src::zebra::EvalType::MIDGAME_EVAL;
use engine::src::zebra::EvaluationType;
use engine::src::zebra::GameMode::PRIVATE_GAME;
use engine_traits::Offset;
use libc_wrapper::{__ctype_b_loc, ctime, exit, fclose, feof, fflush, fgets, FILE, fopen, fprintf, fputc, fputs, fread, free, fscanf, fwrite, malloc, printf, putc, puts, qsort, sprintf, sscanf, stderr, stdout, strcmp, strcpy, strlen, strstr, time, toupper};

use crate::{
    src::{
        error::fatal_error,
        game::{global_setup}
    }
};

use crate::src::error::LibcFatalError;

use crate::src::zebra::{coeff_state, end_g, g_timer, midgame_state, moves_state, prob_cut};
use crate::src::zebra::{board_state, g_book, game_state, hash_state, random_instance, search_state};
use crate::src::zebra::flip_stack_;
use crate::src::zebra::stable_state;

pub type FE = LibcFatalError;

pub type _IO_lock_t = ();
pub type time_t = __time_t;

/*
  ADD_NEW_GAME
  Adds a new game to the game tree.
*/

pub unsafe fn add_new_game(move_count_0: i32,
                           game_move_list: *mut i16,
                           min_empties: i32,
                           max_full_solve: i32,
                           max_wld_solve: i32,
                           update_path: i32,
                           private_game: i32, mut echo:i32) {
    let mut dummy_info =
        EvaluationType{type_0: MIDGAME_EVAL,
            res: WON_POSITION,
            score: 0,
            confidence: 0.,
            search_depth: 0,
            is_book: 0,};
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut stored_echo: i32 = 0;
    let mut dummy_black_score: i32 = 0;
    let mut dummy_white_score: i32 = 0;
    let mut force_eval: i32 = 0;
    let mut midgame_eval_done: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
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
    prepare_tree_traversal();
    i = 0;
    while i < move_count_0 {
        if *game_move_list.offset(i as isize) as i32 >
            0 as i32 {
            flags[i as usize] = 1 as i32 as u16
        } else { flags[i as usize] = 2 as i32 as u16 }
        i += 1
    }
    flags[move_count_0 as usize] = 0;
    first_new_node = 61;
    this_node = 0;
    side_to_move = 0;
    last_move_number =
        if move_count_0 < 60 as i32 - min_empties {
            move_count_0
        } else { (60 as i32) - min_empties };
    i = 0;
    while i <= last_move_number {
        /* Look for the position in the hash table */
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        if slot == -(1 as i32) ||
            *g_book.book_hash_table.offset(slot as isize) == -(1 as i32) {
            this_node = create_BookNode(val1, val2, flags[i as usize], &mut g_book);
            if private_game != 0 {
                let ref mut fresh26 =
                    (*g_book.node.offset(this_node as isize)).flags;
                *fresh26 =
                    (*fresh26 as i32 | 32 as i32) as
                        u16
            }
            if i < first_new_node { first_new_node = i }
        } else { this_node = *g_book.book_hash_table.offset(slot as isize) }
        visited_node[i as usize] = this_node;
        /* Make the moves of the game until the cutoff point */
        if i < last_move_number {
            this_move =
                abs(*game_move_list.offset(i as isize) as i32);
            if *game_move_list.offset(i as isize) as i32 >
                0 as i32 {
                side_to_move = 0 as i32
            } else { side_to_move = 2 as i32 }
            if generate_specific(this_move, side_to_move, &board_state.board) == 0 {
                puts(b"\x00" as *const u8 as *const i8);
                printf(b"i=%d, side_to_move=%d, this_move=%d\n\x00" as
                           *const u8 as *const i8, i, side_to_move,
                       this_move);
                printf(b"last_move_number=%d, move_count=%d\n\x00" as
                           *const u8 as *const i8, last_move_number,
                       move_count_0);
                j = 0;
                while j < move_count_0 {
                    printf(b"%3d \x00" as *const u8 as *const i8,
                           *game_move_list.offset(j as isize) as i32);
                    j += 1
                }
                fatal_error(b"%s: %d\n\x00" as *const u8 as
                                *const i8,
                            b"Invalid move generated\x00" as *const u8 as
                                *const i8, this_move);
            }
            make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        } else {
            /* No more move to make, only update the player to move */
            side_to_move = 0 as i32 + 2 as i32 - side_to_move
        }
        i += 1
    }
    if last_move_number == move_count_0 {
        /* No cutoff applies */
        let mut black_count: i32 = 0;
        let mut white_count: i32 = 0;
        black_count = disc_count(0 as i32, &board_state.board);
        white_count = disc_count(2 as i32, &board_state.board);
        if black_count > white_count {
            outcome = 64 as i32 - 2 as i32 * white_count
        } else if white_count > black_count {
            outcome = 2 as i32 * black_count - 64 as i32
        } else { outcome = 0 as i32 }
    } else {
        generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
        determine_hash_values(side_to_move, &board_state.board, &mut hash_state);
        if echo != 0 {
            puts(b"\x00" as *const u8 as *const i8);
            if side_to_move == 0 as i32 {
                printf(b"Full solving with %d empty (black)\n\x00" as
                           *const u8 as *const i8,
                       60 as i32 - moves_state.disks_played);
            } else {
                printf(b"Full solving with %d empty (white)\n\x00" as
                           *const u8 as *const i8,
                       60 as i32 - moves_state.disks_played);
            }
        }
        end_game::<FE>(side_to_move, 0 as i32, 0 as i32,
                       1 as i32, 0 as i32, &mut dummy_info, echo , &mut flip_stack_
                       , &mut search_state
                       , &mut board_state
                       , &mut hash_state
                       , &mut g_timer
                       , &mut end_g
                       , &mut midgame_state
                       , &mut coeff_state
                       , &mut moves_state
                       , &mut random_instance
                       , &mut g_book
                       , &mut stable_state
                       , &mut prob_cut);
        outcome = search_state.root_eval;
        if side_to_move == 2 as i32 { outcome = -outcome }
    }
    (*g_book.node.offset(this_node as isize)).black_minimax_score =
        outcome as i16;
    (*g_book.node.offset(this_node as isize)).white_minimax_score =
        outcome as i16;
    if outcome > 0 as i32 {
        let ref mut fresh27 =
            (*g_book.node.offset(this_node as isize)).black_minimax_score;
        *fresh27 =
            (*fresh27 as i32 + 30000 as i32) as i16;
        let ref mut fresh28 =
            (*g_book.node.offset(this_node as isize)).white_minimax_score;
        *fresh28 =
            (*fresh28 as i32 + 30000 as i32) as i16
    }
    if outcome < 0 as i32 {
        let ref mut fresh29 =
            (*g_book.node.offset(this_node as isize)).black_minimax_score;
        *fresh29 =
            (*fresh29 as i32 - 30000 as i32) as i16;
        let ref mut fresh30 =
            (*g_book.node.offset(this_node as isize)).white_minimax_score;
        *fresh30 =
            (*fresh30 as i32 - 30000 as i32) as i16
    }
    let ref mut fresh31 = (*g_book.node.offset(this_node as isize)).flags;
    *fresh31 =
        (*fresh31 as i32 | 16 as i32) as u16;
    /* Take another pass through the midgame to update move
       alternatives and minimax information if requested. */
    if update_path != 0 {
        prepare_tree_traversal();
        i = 0;
        while i < last_move_number {
            this_move =
                abs(*game_move_list.offset(i as isize) as i32);
            if *game_move_list.offset(i as isize) as i32 >
                0 as i32 {
                side_to_move = 0 as i32
            } else { side_to_move = 2 as i32 }
            if generate_specific(this_move, side_to_move, &board_state.board) == 0 {
                fatal_error(b"%s: %d\n\x00" as *const u8 as
                                *const i8,
                            b"Invalid move generated\x00" as *const u8 as
                                *const i8, this_move);
            }
            make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
            i += 1
        }
        if echo != 0 { fflush(stdout); }
        midgame_eval_done = 0;
        i = last_move_number - 1 as i32;
        while i >= 0 as i32 {
            this_move =
                abs(*game_move_list.offset(i as isize) as i32);
            if *game_move_list.offset(i as isize) as i32 >
                0 as i32 {
                side_to_move = 0 as i32
            } else { side_to_move = 2 as i32 }
            let move_0 = this_move;
            {
                unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
            };
            /* If the game was public, make sure that all nodes that
            previously marked as private nodes are marked as public. */
            this_node = visited_node[i as usize];
            if private_game == 0 &&
                (*g_book.node.offset(this_node as isize)).flags as i32 &
                    32 as i32 != 0 {
                let ref mut fresh32 =
                    (*g_book.node.offset(this_node as isize)).flags;
                *fresh32 =
                    (*fresh32 as i32 ^ 32 as i32) as
                        u16
            }
            if (*g_book.node.offset(this_node as isize)).flags as i32 &
                1 as i32 != 0 {
                side_to_move = 0 as i32
            } else { side_to_move = 2 as i32 }
            generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
            determine_hash_values(side_to_move, &board_state.board, &mut hash_state);
            if moves_state.disks_played >= 60 as i32 - max_full_solve {
                /* Only solve the position if it hasn't been solved already */
                if (*g_book.node.offset(this_node as isize)).flags as i32 &
                    16 as i32 == 0 {
                    end_game::<FE>(side_to_move, 0 as i32, 0 as i32,
                                   1 as i32, 0 as i32,
                                   &mut dummy_info, echo , &mut flip_stack_
                                   , &mut search_state
                                   , &mut board_state
                                   , &mut hash_state
                                   , &mut g_timer
                                   , &mut end_g
                                   , &mut midgame_state
                                   , &mut coeff_state
                                   , &mut moves_state
                                   , &mut random_instance
                                   , &mut g_book
                                   , &mut stable_state
                                   , &mut prob_cut);
                    if side_to_move == 0 as i32 {
                        outcome = search_state.root_eval
                    } else { outcome = -search_state.root_eval }
                    (*g_book.node.offset(this_node as isize)).black_minimax_score =
                        outcome as i16;
                    (*g_book.node.offset(this_node as isize)).white_minimax_score =
                        outcome as i16;
                    if outcome > 0 as i32 {
                        let ref mut fresh33 =
                            (*g_book.node.offset(this_node as
                                isize)).black_minimax_score;
                        *fresh33 =
                            (*fresh33 as i32 + 30000 as i32)
                                as i16;
                        let ref mut fresh34 =
                            (*g_book.node.offset(this_node as
                                isize)).white_minimax_score;
                        *fresh34 =
                            (*fresh34 as i32 + 30000 as i32)
                                as i16
                    }
                    if outcome < 0 as i32 {
                        let ref mut fresh35 =
                            (*g_book.node.offset(this_node as
                                isize)).black_minimax_score;
                        *fresh35 =
                            (*fresh35 as i32 - 30000 as i32)
                                as i16;
                        let ref mut fresh36 =
                            (*g_book.node.offset(this_node as
                                isize)).white_minimax_score;
                        *fresh36 =
                            (*fresh36 as i32 - 30000 as i32)
                                as i16
                    }
                    let ref mut fresh37 =
                        (*g_book.node.offset(this_node as isize)).flags;
                    *fresh37 =
                        (*fresh37 as i32 | 16 as i32) as
                            u16
                }
            } else if moves_state.disks_played >= 60 as i32 - max_wld_solve {
                /* Only solve the position if its WLD status is unknown */
                if (*g_book.node.offset(this_node as isize)).flags as i32 &
                    4 as i32 == 0 {
                    end_game::<FE>(side_to_move, 1 as i32, 0 as i32,
                                   1 as i32, 0 as i32,
                                   &mut dummy_info, echo , &mut flip_stack_
                                   , &mut search_state
                                   , &mut board_state
                                   , &mut hash_state
                                   , &mut g_timer
                                   , &mut end_g
                                   , &mut midgame_state
                                   , &mut coeff_state
                                   , &mut moves_state
                                   , &mut random_instance
                                   , &mut g_book
                                   , &mut stable_state
                                   , &mut prob_cut);
                    if side_to_move == 0 as i32 {
                        outcome = search_state.root_eval
                    } else { outcome = -search_state.root_eval }
                    (*g_book.node.offset(this_node as isize)).black_minimax_score =
                        outcome as i16;
                    (*g_book.node.offset(this_node as isize)).white_minimax_score =
                        outcome as i16;
                    if outcome > 0 as i32 {
                        let ref mut fresh38 =
                            (*g_book.node.offset(this_node as
                                isize)).black_minimax_score;
                        *fresh38 =
                            (*fresh38 as i32 + 30000 as i32)
                                as i16;
                        let ref mut fresh39 =
                            (*g_book.node.offset(this_node as
                                isize)).white_minimax_score;
                        *fresh39 =
                            (*fresh39 as i32 + 30000 as i32)
                                as i16
                    }
                    if outcome < 0 as i32 {
                        let ref mut fresh40 =
                            (*g_book.node.offset(this_node as
                                isize)).black_minimax_score;
                        *fresh40 =
                            (*fresh40 as i32 - 30000 as i32)
                                as i16;
                        let ref mut fresh41 =
                            (*g_book.node.offset(this_node as
                                isize)).white_minimax_score;
                        *fresh41 =
                            (*fresh41 as i32 - 30000 as i32)
                                as i16
                    }
                    let ref mut fresh42 =
                        (*g_book.node.offset(this_node as isize)).flags;
                    *fresh42 =
                        (*fresh42 as i32 | 4 as i32) as
                            u16
                }
            } else {
                force_eval =
                    (i >= first_new_node - 1 as i32 ||
                        (*g_book.node.offset(this_node as
                            isize)).best_alternative_move as
                            i32 ==
                            abs(*game_move_list.offset(i as isize) as
                                i32)) as i32;
                if midgame_eval_done == 0 {
                    printf(b"Evaluating: \x00" as *const u8 as
                        *const i8);
                    fflush(stdout);
                }
                midgame_eval_done = 1;
                if force_eval != 0 { clear_node_depth(this_node, &mut g_book); }
                evaluate_node::<LibcFatalError>(this_node, echo);
                printf(b"|\x00" as *const u8 as *const i8);
                fflush(stdout);
            }
            let ref mut fresh43 = (*g_book.node.offset(this_node as isize)).flags;
            *fresh43 =
                (*fresh43 as i32 | 8 as i32) as
                    u16;
            do_minimax(this_node, &mut dummy_black_score,
                       &mut dummy_white_score);
            if (*g_book.node.offset(this_node as isize)).flags as i32 &
                4 as i32 == 0 &&
                (*g_book.node.offset(this_node as isize)).best_alternative_move as
                    i32 == -(1 as i32) &&
                (*g_book.node.offset(this_node as isize)).alternative_score as
                    i32 == 9999 as i32 {
                /* Minimax discovered that the g_book.node hasn't got a deviation any
                   longer because that move has been played. */
                evaluate_node::<FE>(this_node, echo);
                printf(b"-|-\x00" as *const u8 as *const i8);
                do_minimax(this_node, &mut dummy_black_score,
                           &mut dummy_white_score);
            }
            i -= 1
        }
        puts(b"\x00" as *const u8 as *const i8);
    }
    echo = stored_echo;
    g_book.total_game_count += 1;
}
/*
   READ_TEXT_DATABASE
   Reads an existing ASCII database file.
*/

pub unsafe fn read_text_database(file_name:
                                                *const i8) {
    let mut i: i32 = 0;
    let mut magic1: i32 = 0;
    let mut magic2: i32 = 0;
    let mut new_book_node_count: i32 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut stream = 0 as *mut FILE;
    time(&mut start_time);
    printf(b"Reading text opening database... \x00" as *const u8 as
               *const i8);
    fflush(stdout);
    stream = fopen(file_name, b"r\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Could not open database file\x00" as *const u8 as
                        *const i8, file_name);
    }
    fscanf(stream, b"%d\x00" as *const u8 as *const i8,
           &mut magic1 as *mut i32);
    fscanf(stream, b"%d\x00" as *const u8 as *const i8,
           &mut magic2 as *mut i32);
    if magic1 != 2718 as i32 || magic2 != 2818 as i32 {
        fatal_error(b"%s: %s\x00" as *const u8 as *const i8,
                    b"Wrong checksum, might be an old version\x00" as
                        *const u8 as *const i8, file_name);
    }
    fscanf(stream, b"%d\x00" as *const u8 as *const i8,
           &mut new_book_node_count as *mut i32);
    set_allocation(new_book_node_count + 1000 as i32, &mut g_book);
    i = 0;
    while i < new_book_node_count {
        fscanf(stream,
               b"%d %d %hd %hd %hd %hd %hd\n\x00" as *const u8 as
                   *const i8,
               &mut (*g_book.node.offset(i as isize)).hash_val1 as *mut i32,
               &mut (*g_book.node.offset(i as isize)).hash_val2 as *mut i32,
               &mut (*g_book.node.offset(i as isize)).black_minimax_score as
                   *mut i16,
               &mut (*g_book.node.offset(i as isize)).white_minimax_score as
                   *mut i16,
               &mut (*g_book.node.offset(i as isize)).best_alternative_move as
                   *mut i16,
               &mut (*g_book.node.offset(i as isize)).alternative_score as
                   *mut i16,
               &mut (*g_book.node.offset(i as isize)).flags as *mut u16);
        i += 1
    }
    g_book.book_node_count = new_book_node_count;
    create_hash_reference(&mut g_book);
    fclose(stream);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
}
/*
   READ_BINARY_DATABASE
   Reads a binary database file.
*/

pub unsafe fn read_binary_database(file_name: *const i8) {
    let mut i: i32 = 0;
    let mut new_book_node_count: i32 = 0;
    let mut magic1: i16 = 0;
    let mut magic2: i16 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut stream = 0 as *mut FILE;
    time(&mut start_time);
    printf(b"Reading binary opening database... \x00" as *const u8 as
               *const i8);
    fflush(stdout);
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Could not open database file\x00" as *const u8 as
                        *const i8, file_name);
    }
    fread(&mut magic1 as *mut i16 as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          1 as i32 as size_t, stream);
    fread(&mut magic2 as *mut i16 as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          1 as i32 as size_t, stream);
    if magic1 as i32 != 2718 as i32 ||
           magic2 as i32 != 2818 as i32 {
        fatal_error(b"%s: %s\x00" as *const u8 as *const i8,
                    b"Wrong checksum, might be an old version\x00" as
                        *const u8 as *const i8, file_name);
    }
    fread(&mut new_book_node_count as *mut i32 as *mut std::ffi::c_void,
          ::std::mem::size_of::<i32>() as u64,
          1 as i32 as size_t, stream);
    set_allocation(new_book_node_count + 1000 as i32, &mut g_book);
    i = 0;
    while i < new_book_node_count {
        fread(&mut (*g_book.node.offset(i as isize)).hash_val1 as *mut i32 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i32>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*g_book.node.offset(i as isize)).hash_val2 as *mut i32 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i32>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*g_book.node.offset(i as isize)).black_minimax_score as
                  *mut i16 as *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*g_book.node.offset(i as isize)).white_minimax_score as
                  *mut i16 as *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*g_book.node.offset(i as isize)).best_alternative_move as
                  *mut i16 as *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*g_book.node.offset(i as isize)).alternative_score as
                  *mut i16 as *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut (*g_book.node.offset(i as isize)).flags as *mut u16 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<u16>() as u64,
              1 as i32 as size_t, stream);
        i += 1
    }
    fclose(stream);
    g_book.book_node_count = new_book_node_count;
    create_hash_reference(&mut g_book);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
}

/*
   WRITE_TEXT_DATABASE
   Writes the database to an ASCII file.
*/

pub unsafe fn write_text_database(file_name:
                                                 *const i8) {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    time(&mut start_time);
    printf(b"Writing text database... \x00" as *const u8 as *const i8);
    fflush(stdout);
    let stream = fopen(file_name, b"w\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Could not create database file\x00" as *const u8 as
                        *const i8, file_name);
    }
    fprintf(stream, b"%d\n%d\n\x00" as *const u8 as *const i8,
            2718 as i32, 2818 as i32);
    fprintf(stream, b"%d\n\x00" as *const u8 as *const i8,
            g_book.book_node_count);
    let mut i = 0;
    while i < g_book.book_node_count {
        fprintf(stream,
                b"%d %d %d %d %d %d %d\n\x00" as *const u8 as
                    *const i8, (*g_book.node.offset(i as isize)).hash_val1,
                (*g_book.node.offset(i as isize)).hash_val2,
                (*g_book.node.offset(i as isize)).black_minimax_score as i32,
                (*g_book.node.offset(i as isize)).white_minimax_score as i32,
                (*g_book.node.offset(i as isize)).best_alternative_move as
                    i32,
                (*g_book.node.offset(i as isize)).alternative_score as i32,
                (*g_book.node.offset(i as isize)).flags as i32);
        i += 1
    }
    fclose(stream);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
}
/*
   WRITE_BINARY_DATABASE
   Writes the database to a binary file.
*/

pub unsafe fn write_binary_database(file_name:
                                                   *const i8) {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    time(&mut start_time);
    printf(b"Writing binary database... \x00" as *const u8 as *const i8);
    fflush(stdout);
    let stream = fopen(file_name, b"wb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error(b"%s \'%s\'\n\x00" as *const u8 as *const i8,
                    b"Could not create database file\x00" as *const u8 as
                        *const i8, file_name);
    }
    let mut magic = 2718 as i32 as i16;
    fwrite(&mut magic as *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    let mut magic = 2818 as i32 as i16;
    fwrite(&mut magic as *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut g_book.book_node_count as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    let mut i = 0;
    while i < g_book.book_node_count {
        fwrite(&mut (*g_book.node.offset(i as isize)).hash_val1 as *mut i32
                   as *const std::ffi::c_void,
               ::std::mem::size_of::<i32>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_book.node.offset(i as isize)).hash_val2 as *mut i32
                   as *const std::ffi::c_void,
               ::std::mem::size_of::<i32>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_book.node.offset(i as isize)).black_minimax_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_book.node.offset(i as isize)).white_minimax_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_book.node.offset(i as isize)).best_alternative_move as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_book.node.offset(i as isize)).alternative_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_book.node.offset(i as isize)).flags as *mut u16 as
                   *const std::ffi::c_void,
               ::std::mem::size_of::<u16>() as u64,
               1 as i32 as size_t, stream);
        i += 1
    }
    fclose(stream);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    puts(b"\x00" as *const u8 as *const i8);
}

/*
   PRINT_MOVE_ALTERNATIVES
   Displays all available book moves from a position.
   FLAGS specifies a subset of the flag bits which have to be set
   for a position to be considered. Notice that FLAGS=0 accepts
   any flag combination.
*/

pub unsafe fn print_move_alternatives(side_to_move:
                                                     i32) {
    let mut i: i32 = 0;
    let mut sign: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut score: i32 = 0;
    let mut output_score: i32 = 0;
    if g_book.candidate_count > 0 as i32 {
        if side_to_move == 0 as i32 {
            sign = 1 as i32
        } else { sign = -(1 as i32) }
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        /* Check that the position is in the opening book after all */
        if slot == -(1 as i32) ||
               *g_book.book_hash_table.offset(slot as isize) == -(1 as i32) {
            return
        }
        /* Pick the book score corresponding to the player to move and
           remove draw avoidance and the special scores for nodes WLD. */
        if side_to_move == 0 as i32 {
            score =
                (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as
                                  isize)).black_minimax_score as i32
        } else {
            score =
                (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as
                                  isize)).white_minimax_score as i32
        }
        if score == 30000 as i32 - 1 as i32 ||
               score == -(30000 as i32 - 1 as i32) {
            score = 0 as i32
        }
        if score > 30000 as i32 { score -= 30000 as i32 }
        if score < -(30000 as i32) { score += 30000 as i32 }
        printf(b"Book score is \x00" as *const u8 as *const i8);
        if (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as
                             isize)).flags as i32 & 16 as i32
               != 0 {
            printf(b"%+d (exact score).\x00" as *const u8 as
                       *const i8, sign * score);
        } else if (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as
                                    isize)).flags as i32 &
                      4 as i32 != 0 {
            printf(b"%+d (W/L/D solved).\x00" as *const u8 as
                       *const i8, sign * score);
        } else {
            printf(b"%+.2f.\x00" as *const u8 as *const i8,
                   (sign * score) as f64 / 128.0f64);
        }
        if (*g_book.node.offset(*g_book.book_hash_table.offset(slot as isize) as
                             isize)).flags as i32 & 32 as i32
               != 0 {
            printf(b" Private g_book.node.\x00" as *const u8 as *const i8);
        }
        puts(b"\x00" as *const u8 as *const i8);
        i = 0;
        while i < g_book.candidate_count {
            printf(b"   %c%c   \x00" as *const u8 as *const i8,
                   'a' as i32 +
                       g_book.candidate_list[i as usize].move_0 % 10 as i32 -
                       1 as i32,
                   '0' as i32 +
                       g_book.candidate_list[i as usize].move_0 / 10 as i32);
            output_score = g_book.candidate_list[i as usize].score;
            if output_score >= 30000 as i32 {
                output_score -= 30000 as i32
            } else if output_score <= -(30000 as i32) {
                output_score += 30000 as i32
            }
            if g_book.candidate_list[i as usize].flags & 16 as i32 != 0 {
                printf(b"%+-6d  (exact score)\x00" as *const u8 as
                           *const i8, output_score);
            } else if g_book.candidate_list[i as usize].flags & 4 as i32 != 0
             {
                printf(b"%+-6d  (W/L/D solved)\x00" as *const u8 as
                           *const i8, output_score);
            } else {
                printf(b"%+-6.2f\x00" as *const u8 as *const i8,
                       output_score as f64 / 128.0f64);
                if g_book.candidate_list[i as usize].flags & 64 as i32 != 0 {
                    printf(b"  (deviation)\x00" as *const u8 as
                               *const i8);
                }
            }
            puts(b"\x00" as *const u8 as *const i8);
            i += 1
        }
    };
}

/*
   INIT_OSF
   Makes sure all data structures are initialized.
*/

pub unsafe fn init_osf(do_global_setup: i32) {
    engine_init_osf::<LibcFatalError>();
    if do_global_setup != 0 {
        global_setup(0 as i32, 19 as i32);
    };
}


/*
   EVALUATE_NODE
   Applies a search to a predetermined depth to find the best
   alternative move in a position.
   Note: This function assumes that generate_all() has been
         called prior to it being called.
*/
pub unsafe fn evaluate_node<FE: FrontEnd>(index: i32, echo: i32) {
    let mut i: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut alternative_move_count: i32 = 0;
    let mut this_move: i32 = 0;
    let mut best_move: i32 = 0;
    let mut child: i32 = 0;
    let mut allow_mpc: i32 = 0;
    let mut depth: i32 = 0;
    let mut best_index: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut feasible_move: [i32; 64] = [0; 64];
    let mut best_score: i32 = 0;
    /* Don't evaluate nodes that already have been searched deep enough */
    depth = get_node_depth(index, &mut g_book);
    if depth >= g_book.search_depth &&
        (*g_book.node.offset(index as isize)).alternative_score as i32 !=
            9999 as i32 {
        return
    }
    /* If the g_book.node has been evaluated and its score is outside the
       eval and minimax windows, bail out. */
    if (*g_book.node.offset(index as isize)).alternative_score as i32 !=
        9999 as i32 {
        if abs((*g_book.node.offset(index as isize)).alternative_score as
            i32) < g_book.min_eval_span ||
            abs((*g_book.node.offset(index as isize)).alternative_score as
                i32) > g_book.max_eval_span {
            return
        }
        if abs((*g_book.node.offset(index as isize)).black_minimax_score as
            i32) < g_book.min_negamax_span ||
            abs((*g_book.node.offset(index as isize)).black_minimax_score as
                i32) > g_book.max_negamax_span {
            return
        }
    }
    if (*g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    remove_coeffs(moves_state.disks_played - 8 as i32, &mut coeff_state);
    g_timer.clear_panic_abort();
    board_state.piece_count[0][moves_state.disks_played as usize] =
        disc_count(0 as i32, &board_state.board);
    board_state.piece_count[2][moves_state.disks_played as usize] =
        disc_count(2 as i32, &board_state.board);
    /* Find the moves which haven't been tried from this position */
    alternative_move_count = 0;
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        this_move = moves_state.move_list[moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        child = *g_book.book_hash_table.offset(slot as isize);
        if child == -(1 as i32) {
            let fresh16 = alternative_move_count;
            alternative_move_count = alternative_move_count + 1;
            feasible_move[fresh16 as usize] = this_move
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i += 1
    }
    if alternative_move_count == 0 as i32 {
        /* There weren't any such moves */
        g_book.exhausted_node_count += 1;
        (*g_book.node.offset(index as isize)).best_alternative_move =
            -(2 as i32) as i16;
        (*g_book.node.offset(index as isize)).alternative_score =
            9999 as i32 as i16
    } else {
        /* Find the best of those moves */
        allow_mpc = (g_book.search_depth >= 9 as i32) as i32;
        nega_scout::<FE>(g_book.search_depth, allow_mpc, side_to_move,
                         alternative_move_count, &mut feasible_move,
                         -(12345678 as i32), 12345678 as i32,
                         &mut best_score, &mut best_index, echo);
        best_move = feasible_move[best_index as usize];
        g_book.evaluated_count += 1;
        if side_to_move == 0 as i32 {
            (*g_book.node.offset(index as isize)).alternative_score =
                best_score as i16
        } else {
            (*g_book.node.offset(index as isize)).alternative_score =
                -best_score as i16
        }
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_book, &board_state.board);
        (*g_book.node.offset(index as isize)).best_alternative_move =
            *g_book.symmetry_map[orientation as usize].offset(best_move as isize) as
                i16
    }
    clear_node_depth(index, &mut g_book);
    set_node_depth(index, g_book.search_depth, &mut g_book);
}

/*
  NEGA_SCOUT
  This wrapper on top of TREE_SEARCH is used by EVALUATE_NODE
  to search the possible deviations.
*/
pub unsafe fn nega_scout<FE: FrontEnd>(depth: i32,
                                       allow_mpc: i32,
                                       side_to_move: i32,
                                       allowed_count: i32,
                                       allowed_moves: &mut [i32],
                                       _alpha: i32, _beta: i32,
                                       best_score: &mut i32,
                                       best_index: &mut i32, echo:i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut curr_alpha: i32 = 0;
    let mut curr_depth: i32 = 0;
    let mut low_score: i32 = 0;
    let mut high_score: i32 = 0;
    let mut best_move: i32 = 0;
    let mut current_score: i32 = 0;
    reset_counter(&mut search_state.nodes);
    low_score = -(12345678 as i32);
    /* To avoid spurious hash table entries to take out the effect
       of the averaging done, the hash table drafts are changed prior
       to each g_book.node being searched. */
    clear_hash_drafts(&mut hash_state);
    determine_hash_values(side_to_move, &board_state.board, &mut hash_state);
    /* First determine the best move in the current position
       and its score when searched to depth DEPTH.
       This is done using standard negascout with iterative deepening. */
    curr_depth = 2 as i32 - depth % 2 as i32;
    while curr_depth <= depth {
        low_score = -(12345678 as i32);
        curr_alpha = -(12345678 as i32);
        i = 0;
        while i < allowed_count {
            make_move(side_to_move, *allowed_moves.offset(i as isize),
                      1 as i32, &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
            board_state.piece_count[0][moves_state.disks_played as usize] =
                disc_count(0 as i32, &board_state.board);
            board_state.piece_count[2][moves_state.disks_played as usize] =
                disc_count(2 as i32, &board_state.board);
            g_timer.last_panic_check = 0.0f64;
            if i == 0 as i32 {
                current_score =
                    -tree_search::<FE>(1 as i32, curr_depth,
                                       0 as i32 + 2 as i32 -
                                           side_to_move, -(12345678 as i32),
                                       12345678 as i32, 1 as i32,
                                       allow_mpc, 1 as i32, echo,  &mut moves_state ,
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
                    -tree_search::<FE>(1 as i32, curr_depth,
                                       0 as i32 + 2 as i32 -
                                           side_to_move,
                                       -(curr_alpha + 1 as i32),
                                       -curr_alpha, 1 as i32, allow_mpc,
                                       1 as i32, echo,  &mut moves_state ,
                                       &mut search_state ,
                                       &mut board_state ,
                                       &mut hash_state,
                                       &mut flip_stack_,
                                       &mut coeff_state,
                                       &mut prob_cut ,&mut g_timer, &mut midgame_state);
                if current_score > curr_alpha {
                    current_score =
                        -tree_search::<FE>(1 as i32, curr_depth,
                                           0 as i32 + 2 as i32 -
                                               side_to_move,
                                           -(12345678 as i32),
                                           12345678 as i32,
                                           1 as i32, allow_mpc,
                                           1 as i32, echo,  &mut moves_state ,
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
        while j >= 1 as i32 {
            *allowed_moves.offset(j as isize) =
                *allowed_moves.offset((j - 1 as i32) as isize);
            j -= 1
        }
        allowed_moves[0] = best_move;
        *best_index = 0;
        curr_depth += 2 as i32
    }
    /* Then find the score for the best move when searched
       to depth DEPTH+1 */
    make_move(side_to_move, *allowed_moves.offset(*best_index as isize),
              1 as i32, &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
    board_state.piece_count[0][moves_state.disks_played as usize] =
        disc_count(0 as i32, &board_state.board);
    board_state.piece_count[2][moves_state.disks_played as usize] =
        disc_count(2 as i32, &board_state.board);
    g_timer.last_panic_check = 0.0f64;
    high_score =
        -tree_search::<FE>(1 as i32, depth + 1 as i32,
                           0 as i32 + 2 as i32 - side_to_move,
                           -(12345678 as i32), 12345678 as i32,
                           1 as i32, allow_mpc, 1 as i32, echo,  &mut moves_state ,
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
    *best_score = (low_score + high_score) / 2 as i32;
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
    set_allocation(book.node_table_size + 50000 as i32, book);
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
        -(1 as i32) as i16;
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
pub unsafe fn do_minimax(index: i32,
                         black_score: &mut i32,
                         white_score: &mut i32) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut child_black_score: i32 = 0;
    let mut child_white_score: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move: i32 = 0;
    let mut alternative_move: i32 = 0;
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
    if (*g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        if (*g_book.node.offset(index as isize)).flags as i32 &
            (4 as i32 | 16 as i32) == 0 {
            *black_score =
                (*g_book.node.offset(index as isize)).black_minimax_score as
                    i32;
            *white_score =
                (*g_book.node.offset(index as isize)).white_minimax_score as
                    i32;
            return
        }
    }
    /* Correct WLD solved nodes corresponding to draws to be represented
       as full solved and make sure full solved nodes are marked as
       WLD solved as well */
    if (*g_book.node.offset(index as isize)).flags as i32 & 4 as i32
        != 0 &&
        (*g_book.node.offset(index as isize)).black_minimax_score as i32
            == 0 as i32 &&
        (*g_book.node.offset(index as isize)).white_minimax_score as i32
            == 0 as i32 {
        let ref mut fresh2 = (*g_book.node.offset(index as isize)).flags;
        *fresh2 =
            (*fresh2 as i32 | 16 as i32) as u16
    }
    if (*g_book.node.offset(index as isize)).flags as i32 & 16 as i32
        != 0 &&
        (*g_book.node.offset(index as isize)).flags as i32 &
            4 as i32 == 0 {
        let ref mut fresh3 = (*g_book.node.offset(index as isize)).flags;
        *fresh3 =
            (*fresh3 as i32 | 4 as i32) as u16
    }
    /* Recursively minimax all children of the node */
    if (*g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    best_black_child_val = -(99999 as i32);
    best_white_child_val = -(99999 as i32);
    worst_black_child_val = 99999;
    worst_white_child_val = 99999;
    if (*g_book.node.offset(index as isize)).alternative_score as i32 !=
        9999 as i32 {
        best_black_score =
            adjust_score((*g_book.node.offset(index as isize)).alternative_score as
                             i32, side_to_move, &mut g_book, moves_state.disks_played) as i16;
        best_white_score = best_black_score;
        worst_black_child_val = best_black_score as i32;
        best_black_child_val = worst_black_child_val;
        worst_white_child_val = best_white_score as i32;
        best_white_child_val = worst_white_child_val;
        alternative_move_found = 0;
        alternative_move =
            (*g_book.node.offset(index as isize)).best_alternative_move as
                i32;
        if alternative_move > 0 as i32 {
            get_hash(&mut val1, &mut val2, &mut orientation, &mut g_book, &board_state.board);
            alternative_move =
                *g_book.inv_symmetry_map[orientation as
                    usize].offset(alternative_move as isize)
        }
    } else {
        alternative_move_found = 1;
        alternative_move = 0;
        if side_to_move == 0 as i32 {
            best_black_score = -(32000 as i32) as i16;
            best_white_score = -(32000 as i32) as i16
        } else {
            best_black_score = 32000;
            best_white_score = 32000 as i32 as i16
        }
    }
    generate_all(side_to_move, &mut moves_state, &search_state, &board_state.board);
    child_count = 0;
    i = 0;
    while i < moves_state.move_count[moves_state.disks_played as usize] {
        board_state.piece_count[0][moves_state.disks_played as usize] =
            disc_count(0 as i32, &board_state.board);
        board_state.piece_count[2][moves_state.disks_played as usize] =
            disc_count(2 as i32, &board_state.board);
        this_move = moves_state.move_list[moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut moves_state, &mut board_state, &mut hash_state, &mut flip_stack_ );
        get_hash(&mut val1, &mut val2, &mut orientation, &mut g_book, &board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_book);
        child = *g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_minimax(child, &mut child_black_score, &mut child_white_score);
            best_black_child_val =
                if best_black_child_val > child_black_score {
                    best_black_child_val
                } else { child_black_score };
            best_white_child_val =
                if best_white_child_val > child_white_score {
                    best_white_child_val
                } else { child_white_score };
            worst_black_child_val =
                if worst_black_child_val < child_black_score {
                    worst_black_child_val
                } else { child_black_score };
            worst_white_child_val =
                if worst_white_child_val < child_white_score {
                    worst_white_child_val
                } else { child_white_score };
            if side_to_move == 0 as i32 {
                best_black_score =
                    if child_black_score > best_black_score as i32 {
                        child_black_score
                    } else { best_black_score as i32 } as
                        i16;
                best_white_score =
                    if child_white_score > best_white_score as i32 {
                        child_white_score
                    } else { best_white_score as i32 } as
                        i16
            } else {
                best_black_score =
                    if child_black_score < best_black_score as i32 {
                        child_black_score
                    } else { best_black_score as i32 } as
                        i16;
                best_white_score =
                    if child_white_score < best_white_score as i32 {
                        child_white_score
                    } else { best_white_score as i32 } as
                        i16
            }
            child_count += 1
        } else if alternative_move_found == 0 && this_move == alternative_move
        {
            alternative_move_found = 1 as i32
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut board_state.board, &mut moves_state, &mut hash_state, &mut flip_stack_);
        };
        i += 1
    }
    if alternative_move_found == 0 {
        /* The was-to-be deviation now leads to a position in the database,
           hence it can no longer be used. */
        (*g_book.node.offset(index as isize)).alternative_score = 9999;
        (*g_book.node.offset(index as isize)).best_alternative_move =
            -(1 as i32) as i16
    }
    /* Try to infer the WLD status from the children */
    if (*g_book.node.offset(index as isize)).flags as i32 &
        (16 as i32 | 4 as i32) == 0 &&
        child_count > 0 as i32 {
        if side_to_move == 0 as i32 {
            if best_black_child_val >= 30000 as i32 &&
                best_white_child_val >= 30000 as i32 {
                /* Black win */
                let ref mut fresh4 =
                    (*g_book.node.offset(index as isize)).white_minimax_score;
                *fresh4 =
                    if best_black_child_val < best_white_child_val {
                        best_black_child_val
                    } else { best_white_child_val } as i16;
                (*g_book.node.offset(index as isize)).black_minimax_score = *fresh4;
                let ref mut fresh5 = (*g_book.node.offset(index as isize)).flags;
                *fresh5 =
                    (*fresh5 as i32 | 4 as i32) as
                        u16
            } else if best_black_child_val <= -(30000 as i32) &&
                best_white_child_val <= -(30000 as i32) {
                /* Black loss */
                let ref mut fresh6 =
                    (*g_book.node.offset(index as isize)).white_minimax_score;
                *fresh6 =
                    if best_black_child_val > best_white_child_val {
                        best_black_child_val
                    } else { best_white_child_val } as i16;
                (*g_book.node.offset(index as isize)).black_minimax_score = *fresh6;
                let ref mut fresh7 = (*g_book.node.offset(index as isize)).flags;
                *fresh7 =
                    (*fresh7 as i32 | 4 as i32) as
                        u16
            }
        } else if worst_black_child_val <= -(30000 as i32) &&
            worst_white_child_val <= -(30000 as i32) {
            /* White win */
            let ref mut fresh8 =
                (*g_book.node.offset(index as isize)).white_minimax_score;
            *fresh8 =
                if worst_black_child_val > worst_white_child_val {
                    worst_black_child_val
                } else { worst_white_child_val } as i16;
            (*g_book.node.offset(index as isize)).black_minimax_score = *fresh8;
            let ref mut fresh9 = (*g_book.node.offset(index as isize)).flags;
            *fresh9 =
                (*fresh9 as i32 | 4 as i32) as u16
        } else if worst_black_child_val >= 30000 as i32 &&
            worst_white_child_val >= 30000 as i32 {
            /* White loss */
            let ref mut fresh10 =
                (*g_book.node.offset(index as isize)).white_minimax_score;
            *fresh10 =
                if worst_black_child_val < worst_white_child_val {
                    worst_black_child_val
                } else { worst_white_child_val } as i16;
            (*g_book.node.offset(index as isize)).black_minimax_score = *fresh10;
            let ref mut fresh11 = (*g_book.node.offset(index as isize)).flags;
            *fresh11 =
                (*fresh11 as i32 | 4 as i32) as u16
        }
    }
    /* Tweak the minimax scores for draws to give the right
       draw avoidance behavior */
    if (*g_book.node.offset(index as isize)).flags as i32 &
        (16 as i32 | 4 as i32) != 0 {
        *black_score =
            (*g_book.node.offset(index as isize)).black_minimax_score as i32;
        *white_score =
            (*g_book.node.offset(index as isize)).white_minimax_score as i32;
        if (*g_book.node.offset(index as isize)).black_minimax_score as i32
            == 0 as i32 &&
            (*g_book.node.offset(index as isize)).white_minimax_score as
                i32 == 0 as i32 {
            /* Is it a position in which a draw should be avoided? */
            if g_book.game_mode as u32 ==
                PRIVATE_GAME as i32 as u32 ||
                (*g_book.node.offset(index as isize)).flags as i32 &
                    32 as i32 == 0 {
                match g_book.draw_mode as u32 {
                    1 => {
                        *black_score =
                            30000 as i32 - 1 as i32;
                        *white_score = 30000 as i32 - 1 as i32
                    }
                    2 => {
                        *black_score =
                            -(30000 as i32 - 1 as i32);
                        *white_score =
                            -(30000 as i32 - 1 as i32)
                    }
                    3 => {
                        *black_score =
                            -(30000 as i32 - 1 as i32);
                        *white_score = 30000 as i32 - 1 as i32
                    }
                    0 | _ => { }
                }
            }
        }
    } else {
        let ref mut fresh12 =
            (*g_book.node.offset(index as isize)).black_minimax_score;
        *fresh12 = best_black_score;
        *black_score = *fresh12 as i32;
        let ref mut fresh13 =
            (*g_book.node.offset(index as isize)).white_minimax_score;
        *fresh13 = best_white_score;
        *white_score = *fresh13 as i32
    }
    let ref mut fresh14 = (*g_book.node.offset(index as isize)).flags;
    *fresh14 = (*fresh14 as i32 ^ 8 as i32) as u16;
}



pub unsafe fn engine_init_osf<FE: FrontEnd>() {
    init_maps::<FE>(); //FIXME why is this not called from zebra everytime in the engine?????
    prepare_hash(&mut g_book, &mut crate::src::zebra::random_instance);
    setup_hash(1 as i32, &mut hash_state, &mut random_instance);
    init_book_tree(&mut g_book);
    reset_book_search(&mut g_book);
    g_book.search_depth = 2;
    g_book.max_slack = 0;
    g_book.low_deviation_threshold = 60;
    g_book.high_deviation_threshold = 60;
    g_book.deviation_bonus = 0.0f64;
    g_book.min_eval_span = 0;
    g_book.max_eval_span = 1000 as i32 * 128 as i32;
    g_book.min_negamax_span = 0;
    g_book.max_negamax_span = 1000 as i32 * 128 as i32;
    g_book.max_batch_size = 10000000;
    g_book.force_black = 0;
    g_book.force_white = 0;
}


/*
   PREPATE_TREE_TRAVERSAL
   Prepares all relevant data structures for a tree search
   or traversal.
*/
pub unsafe fn prepare_tree_traversal() {
    let mut side_to_move: i32 = 0;
    setup_non_file_based_game(&mut side_to_move,&mut board_state
                              ,&mut hash_state
                              ,&mut moves_state);
    engine_game_init(&mut flip_stack_, &mut search_state, &mut board_state, &mut hash_state, &mut g_timer,
                     &mut end_g, &mut midgame_state, &mut coeff_state, &mut moves_state, &mut random_instance, &mut g_book, &mut stable_state, &mut game_state);
    midgame_state.toggle_midgame_hash_usage(1 as i32, 1 as i32);
    g_timer.toggle_abort_check(0 as i32);
    midgame_state.toggle_midgame_abort_check(0 as i32);
}



/*
   INIT_MAPS
   Initializes the 8 symmetry maps.
   Notice that the order of these MUST coincide with the returned
   orientation value from get_hash() OR YOU WILL LOSE BIG.
*/
pub unsafe fn init_maps<FE: FrontEnd>() {
    let book = &mut g_book;
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
    let book = & g_book;
    let mut i = 0;
    let mut k = 0;
    while i < 8 as i32 {
        let mut j = 1;
        while j <= 8 as i32 {
            k = 1;
            while k <= 8 as i32 {
                let pos = 10 as i32 * j + k;
                if *book.inv_symmetry_map[i as usize]
                    .offset(*book.symmetry_map[i as usize].offset(pos as isize) as isize) != pos {
                    let symmetry_map_item = *book.inv_symmetry_map[i as usize].offset(*book.symmetry_map[i as usize].offset(pos as isize) as isize);
                    FE::error_in_map(i, pos, symmetry_map_item);
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
    let x = 0 as i32;
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
        *book.book_hash_table.offset(i as isize) = -(1 as i32);
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
    while *book.book_hash_table.offset(slot as isize) != -(1 as i32) {
        slot = (slot + 1 as i32) % book.hash_table_size
    }
    *book.book_hash_table.offset(slot as isize) = index;
}
