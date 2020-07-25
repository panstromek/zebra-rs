use crate::src::libc;
use crate::src::stubs::{puts, strlen, abs, fputs, free, printf, qsort, fprintf, fclose, fopen, fread, strchr, strcmp, tolower};
use crate::src::safemem::{safe_malloc, safe_realloc};
use crate::src::bitboard::bit_reverse_32;
use crate::src::myrandom::my_random;
use crate::src::patterns::pow3;
use crate::src::zebra::_IO_FILE;
pub use engine::src::thordb::*;

pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type __compar_fn_t
    =
    Option<unsafe fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> i32>;
/*
  GET_INT_8
  Reads an 8-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
unsafe fn get_int_8(mut stream: *mut FILE, mut value: *mut int_8)
 -> i32 {
    let mut actually_read: i32 = 0;
    actually_read =
        fread(value as *mut libc::c_void,
              ::std::mem::size_of::<int_8>() as u64,
              1 as i32 as size_t, stream) as i32;
    return (actually_read == 1 as i32) as i32;
}
/*
  GET_INT_16
  Reads a 16-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
unsafe fn get_int_16(mut stream: *mut FILE, mut value: *mut int_16)
 -> i32 {
    let mut actually_read: i32 = 0;
    actually_read =
        fread(value as *mut libc::c_void,
              ::std::mem::size_of::<int_16>() as u64,
              1 as i32 as size_t, stream) as i32;
    return (actually_read == 1 as i32) as i32;
}
/*
  GET_INT_32
  Reads a 32-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
unsafe fn get_int_32(mut stream: *mut FILE, mut value: *mut int_32)
 -> i32 {
    let mut actually_read: i32 = 0;
    actually_read =
        fread(value as *mut libc::c_void,
              ::std::mem::size_of::<int_32>() as u64,
              1 as i32 as size_t, stream) as i32;
    return (actually_read == 1 as i32) as i32;
}
/*
  READ_PROLOG
  Reads the prolog from STREAM into PROLOG. As the prolog is common
  for all the three database types (game, player, tournament) also
  values which aren't used are read.
  Returns TRUE upon success, otherwise FALSE.
*/
unsafe fn read_prolog(mut stream: *mut FILE,
                                 mut prolog: *mut PrologType) -> i32 {
    let mut success: i32 = 0;
    let mut byte_val: int_8 = 0;
    let mut word_val: int_16 = 0;
    let mut longint_val: int_32 = 0;
    success = get_int_8(stream, &mut byte_val);
    (*prolog).creation_century = byte_val as i32;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            i32;
    (*prolog).creation_year = byte_val as i32;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            i32;
    (*prolog).creation_month = byte_val as i32;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            i32;
    (*prolog).creation_day = byte_val as i32;
    success =
        (success != 0 && get_int_32(stream, &mut longint_val) != 0) as
            i32;
    (*prolog).game_count = longint_val;
    success =
        (success != 0 && get_int_16(stream, &mut word_val) != 0) as
            i32;
    (*prolog).item_count = word_val as i32;
    success =
        (success != 0 && get_int_16(stream, &mut word_val) != 0) as
            i32;
    (*prolog).origin_year = word_val as i32;
    success =
        (success != 0 && get_int_32(stream, &mut longint_val) != 0) as
            i32;
    (*prolog).reserved = longint_val;
    return success;
}
/*
  THOR_COMPARE_TOURNAMENTS
  Lexicographically compares the names of two tournaments
  represented by pointers.
*/
unsafe extern "C" fn thor_compare_tournaments(mut t1: *const libc::c_void,
                                              mut t2: *const libc::c_void)
 -> i32 {
    let mut tournament1 = *(t1 as *mut *mut TournamentType);
    let mut tournament2 = *(t2 as *mut *mut TournamentType);
    return strcmp((*tournament1).name, (*tournament2).name);
}
/*
  SORT_TOURNAMENT_DATABASE
  Computes the lexicographic order of all tournaments in the database.
*/
unsafe extern "C" fn sort_tournament_database() {
    let mut tournament_buffer = 0 as *mut *mut TournamentType;
    let mut i: i32 = 0;
    tournament_buffer =
        safe_malloc((tournaments.count as
                         u64).wrapping_mul(::std::mem::size_of::<*mut TournamentType>()
                                                         as u64)) as
            *mut *mut TournamentType;
    i = 0 as i32;
    while i < tournaments.count {
        let ref mut fresh0 = *tournament_buffer.offset(i as isize);
        *fresh0 =
            &mut *tournaments.tournament_list.offset(i as isize) as
                *mut TournamentType;
        i += 1
    }
    qsort(tournament_buffer as *mut libc::c_void, tournaments.count as size_t,
          ::std::mem::size_of::<*mut TournamentType>() as u64,
          Some(thor_compare_tournaments as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> i32));
    i = 0 as i32;
    while i < tournaments.count {
        (**tournament_buffer.offset(i as isize)).lex_order = i;
        i += 1
    }
    free(tournament_buffer as *mut libc::c_void);
}
/*
  READ_TOURNAMENT_DATABASE
  Reads the tournament database from FILE_NAME.
  Returns TRUE if all went well, otherwise FALSE.
*/

pub unsafe fn read_tournament_database(mut file_name:
                                                      *const i8)
 -> i32 {
    let mut stream = 0 as *mut FILE;
    let mut i: i32 = 0;
    let mut success: i32 = 0;
    let mut actually_read: i32 = 0;
    let mut buffer_size: i32 = 0;
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() { return 0 as i32 }
    if read_prolog(stream, &mut tournaments.prolog) == 0 {
        fclose(stream);
        return 0 as i32
    }
    tournaments.count = tournaments.prolog.item_count;
    buffer_size = 26 as i32 * tournaments.prolog.item_count;
    tournaments.name_buffer =
        safe_realloc(tournaments.name_buffer as *mut libc::c_void,
                     buffer_size as size_t) as *mut i8;
    actually_read =
        fread(tournaments.name_buffer as *mut libc::c_void,
              1 as i32 as size_t, buffer_size as size_t, stream) as
            i32;
    success = (actually_read == buffer_size) as i32;
    fclose(stream);
    if success != 0 {
        tournaments.tournament_list =
            safe_realloc(tournaments.tournament_list as *mut libc::c_void,
                         (tournaments.count as
                              u64).wrapping_mul(::std::mem::size_of::<TournamentType>()
                                                              as
                                                              u64))
                as *mut TournamentType;
        i = 0 as i32;
        while i < tournaments.count {
            let ref mut fresh1 =
                (*tournaments.tournament_list.offset(i as isize)).name;
            *fresh1 = tournament_name(i);
            (*tournaments.tournament_list.offset(i as isize)).selected =
                1 as i32;
            i += 1
        }
        sort_tournament_database();
        thor_games_sorted = 0 as i32;
        thor_games_filtered = 0 as i32
    }
    return success;
}
/*
  THOR_COMPARE_PLAYERS
  Lexicographically compares the names of two players
  represented by pointers.
*/
unsafe extern "C" fn thor_compare_players(mut p1: *const libc::c_void,
                                          mut p2: *const libc::c_void)
 -> i32 {
    let mut ch: i8 = 0;
    let mut buffer1: [i8; 20] = [0; 20];
    let mut buffer2: [i8; 20] = [0; 20];
    let mut i: i32 = 0;
    let mut player1 = *(p1 as *mut *mut PlayerType);
    let mut player2 = *(p2 as *mut *mut PlayerType);
    i = 0 as i32;
    loop  {
        ch = *(*player1).name.offset(i as isize);
        buffer1[i as usize] = tolower(ch as i32) as i8;
        i += 1;
        if !(ch as i32 != 0 as i32) { break ; }
    }
    if buffer1[0 as i32 as usize] as i32 == '?' as i32 {
        /* Put unknown players LAST */
        buffer1[0 as i32 as usize] = '~' as i32 as i8
    }
    i = 0 as i32;
    loop  {
        ch = *(*player2).name.offset(i as isize);
        buffer2[i as usize] = tolower(ch as i32) as i8;
        i += 1;
        if !(ch as i32 != 0 as i32) { break ; }
    }
    if buffer2[0 as i32 as usize] as i32 == '?' as i32 {
        /* Put unknown players LAST */
        buffer2[0 as i32 as usize] = '~' as i32 as i8
    }
    return strcmp(buffer1.as_mut_ptr(), buffer2.as_mut_ptr());
}
/*
  SORT_PLAYER_DATABASE
  Computes the lexicographic order of all players in the database.
*/
unsafe fn sort_player_database() {
    let mut player_buffer = 0 as *mut *mut PlayerType;
    let mut i: i32 = 0;
    player_buffer =
        safe_malloc((players.count as
                         u64).wrapping_mul(::std::mem::size_of::<*mut PlayerType>()
                                                         as u64)) as
            *mut *mut PlayerType;
    i = 0 as i32;
    while i < players.count {
        let ref mut fresh2 = *player_buffer.offset(i as isize);
        *fresh2 =
            &mut *players.player_list.offset(i as isize) as *mut PlayerType;
        i += 1
    }
    qsort(player_buffer as *mut libc::c_void, players.count as size_t,
          ::std::mem::size_of::<*mut PlayerType>() as u64,
          Some(thor_compare_players as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> i32));
    i = 0 as i32;
    while i < players.count {
        (**player_buffer.offset(i as isize)).lex_order = i;
        i += 1
    }
    free(player_buffer as *mut libc::c_void);
}
/*
  READ_PLAYER_DATABASE
  Reads the player database from FILE_NAME.
  Returns TRUE if all went well, otherwise FALSE.
*/

pub unsafe fn read_player_database(mut file_name:
                                                  *const i8)
 -> i32 {
    let mut stream = 0 as *mut FILE;
    let mut i: i32 = 0;
    let mut success: i32 = 0;
    let mut actually_read: i32 = 0;
    let mut buffer_size: i32 = 0;
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() { return 0 as i32 }
    if read_prolog(stream, &mut players.prolog) == 0 {
        fclose(stream);
        return 0 as i32
    }
    players.count = players.prolog.item_count;
    buffer_size = 20 as i32 * players.count;
    players.name_buffer =
        safe_realloc(players.name_buffer as *mut libc::c_void,
                     buffer_size as size_t) as *mut i8;
    actually_read =
        fread(players.name_buffer as *mut libc::c_void,
              1 as i32 as size_t, buffer_size as size_t, stream) as
            i32;
    success = (actually_read == buffer_size) as i32;
    fclose(stream);
    if success != 0 {
        players.player_list =
            safe_realloc(players.player_list as *mut libc::c_void,
                         (players.count as
                              u64).wrapping_mul(::std::mem::size_of::<PlayerType>()
                                                              as
                                                              u64))
                as *mut PlayerType;
        i = 0 as i32;
        while i < players.count {
            let ref mut fresh3 =
                (*players.player_list.offset(i as isize)).name;
            *fresh3 = get_player_name(i);
            /* By convention, names of computer programs always contain
            parenthesis within which the name of the creator of the
             program is given. E.g. "Zebra (andersson)", "Sethos()". */
            if !strchr((*players.player_list.offset(i as isize)).name,
                       '(' as i32).is_null() {
                (*players.player_list.offset(i as isize)).is_program =
                    1 as i32
            } else {
                (*players.player_list.offset(i as isize)).is_program =
                    0 as i32
            }
            (*players.player_list.offset(i as isize)).selected =
                1 as i32;
            i += 1
        }
        sort_player_database();
        thor_games_sorted = 0 as i32;
        thor_games_filtered = 0 as i32
    }
    return success;
}
/*
  READ_GAME
  Reads a game from STREAM in GAME and prepares the game
  for database questions. Returns TRUE upon success,
  otherwise FALSE.
*/
unsafe fn read_game(mut stream: *mut FILE, mut game: *mut GameType)
 -> i32 {
    let mut success: i32 = 0;
    let mut actually_read: i32 = 0;
    let mut byte_val: int_8 = 0;
    let mut word_val: int_16 = 0;
    success = get_int_16(stream, &mut word_val);
    (*game).tournament_no = word_val;
    success =
        (success != 0 && get_int_16(stream, &mut word_val) != 0) as
            i32;
    (*game).black_no = word_val;
    success =
        (success != 0 && get_int_16(stream, &mut word_val) != 0) as
            i32;
    (*game).white_no = word_val;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            i32;
    (*game).actual_black_score = byte_val as i16;
    success =
        (success != 0 && get_int_8(stream, &mut byte_val) != 0) as
            i32;
    (*game).perfect_black_score = byte_val as i16;
    actually_read =
        fread(&mut (*game).moves as *mut [i8; 60] as
                  *mut libc::c_void, 1 as i32 as size_t,
              60 as i32 as size_t, stream) as i32;
    prepare_game(game);
    return (success != 0 && actually_read == 60 as i32) as
               i32;
}
/*
  READ_GAME_DATABASE
  Reads a game database from FILE_NAME.
*/

pub unsafe fn read_game_database(mut file_name:
                                                *const i8)
 -> i32 {
    let mut stream = 0 as *mut FILE;
    let mut i: i32 = 0;
    let mut success: i32 = 0;
    let mut old_database_head = 0 as *mut DatabaseType;
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() { return 0 as i32 }
    old_database_head = database_head;
    database_head =
        safe_malloc(::std::mem::size_of::<DatabaseType>() as u64) as
            *mut DatabaseType;
    (*database_head).next = old_database_head;
    if read_prolog(stream, &mut (*database_head).prolog) == 0 {
        fclose(stream);
        return 0 as i32
    }
    success = 1 as i32;
    (*database_head).count = (*database_head).prolog.game_count;
    (*database_head).games =
        safe_malloc(((*database_head).count as
                         u64).wrapping_mul(::std::mem::size_of::<GameType>()
                                                         as u64)) as
            *mut GameType;
    i = 0 as i32;
    while i < (*database_head).count {
        success =
            (success != 0 &&
                 read_game(stream,
                           &mut *(*database_head).games.offset(i as isize)) !=
                     0) as i32;
        let ref mut fresh4 =
            (*(*database_head).games.offset(i as isize)).database;
        *fresh4 = database_head;
        i += 1
    }
    thor_database_count += 1;
    thor_game_count += (*database_head).count;
    thor_games_sorted = 0 as i32;
    thor_games_filtered = 0 as i32;
    fclose(stream);
    return success;
}
/*
  GAME_DATABASE_ALREADY_LOADED
  Checks if the game database in FILE_NAME already exists in memory
  (the only thing actually checked is the prolog but this suffices
  according to the specification of the database format).
*/

pub unsafe fn game_database_already_loaded(mut file_name:
                                                          *const i8)
 -> i32 {
    let mut stream = 0 as *mut FILE;
    let mut current_db = 0 as *mut DatabaseType;
    let mut new_prolog =
        PrologType{creation_century: 0,
                   creation_year: 0,
                   creation_month: 0,
                   creation_day: 0,
                   game_count: 0,
                   item_count: 0,
                   origin_year: 0,
                   reserved: 0,};
    stream = fopen(file_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() { return 0 as i32 }
    if read_prolog(stream, &mut new_prolog) == 0 {
        fclose(stream);
        return 0 as i32
    }
    fclose(stream);
    current_db = database_head;
    while !current_db.is_null() {
        if (*current_db).prolog.creation_century ==
               new_prolog.creation_century &&
               (*current_db).prolog.creation_year == new_prolog.creation_year
               &&
               (*current_db).prolog.creation_month ==
                   new_prolog.creation_month &&
               (*current_db).prolog.creation_day == new_prolog.creation_day &&
               (*current_db).prolog.game_count == new_prolog.game_count &&
               (*current_db).prolog.item_count == new_prolog.item_count &&
               (*current_db).prolog.origin_year ==
                   (*current_db).prolog.origin_year {
            return 1 as i32
        }
        current_db = (*current_db).next
    }
    return 0 as i32;
}
/*
  PRINT_GAME
  Outputs the information about the game GAME to STREAM.
  The flag DISPLAY_MOVES specifies if the moves of the
  game are to be output or not.
*/
unsafe fn print_game(mut stream: *mut FILE,
                                mut game: *mut GameType,
                                mut display_moves: i32) {
    let mut i: i32 = 0;
    fprintf(stream, b"%s  %d\n\x00" as *const u8 as *const i8,
            tournament_name((*game).tournament_no as i32),
            (*(*game).database).prolog.origin_year);
    fprintf(stream, b"%s %s %s\n\x00" as *const u8 as *const i8,
            get_player_name((*game).black_no as i32),
            b"vs\x00" as *const u8 as *const i8,
            get_player_name((*game).white_no as i32));
    fprintf(stream, b"%d - %d   \x00" as *const u8 as *const i8,
            (*game).actual_black_score as i32,
            64 as i32 - (*game).actual_black_score as i32);
    fprintf(stream,
            b"[ %d - %d %s ]\n\x00" as *const u8 as *const i8,
            (*game).perfect_black_score as i32,
            64 as i32 - (*game).perfect_black_score as i32,
            b"perfect\x00" as *const u8 as *const i8);
    if display_moves != 0 {
        i = 0 as i32;
        while i < 60 as i32 {
            fprintf(stream, b" %d\x00" as *const u8 as *const i8,
                    abs((*game).moves[i as usize] as i32));
            if i % 20 as i32 == 19 as i32 {
                fputs(b"\n\x00" as *const u8 as *const i8, stream);
            }
            i += 1
        }
    }
    fputs(b"\n\x00" as *const u8 as *const i8, stream);
}
/*
  SORT_THOR_GAMES
  Sorts the COUNT first games in the list THOR_SEARCH.MATCH_LIST.
  The first THOR_SORT_CRITERIA_COUNT entries of THOR_SORT_ORDER are
  used (in order) to sort the matches.
*/
unsafe fn sort_thor_games(mut count: i32) {
    if count <= 1 as i32 {
        /* No need to sort 0 or 1 games. */
        return
    }
    qsort(thor_search.match_list as *mut libc::c_void, count as size_t,
          ::std::mem::size_of::<*mut GameType>() as u64,
          Some(thor_compare as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> i32));
}

#[no_mangle]
pub unsafe extern "C" fn choose_thor_opening_move_report(
    freq_sum: i32, match_count: i32, move_list: &[C2RustUnnamed; 64]) {
    printf(b"%s:        \x00" as *const u8 as *const i8,
           b"Thor database\x00" as *const u8 as *const i8);
    let mut i = 0 as i32;
    while i < match_count {
        printf(b"%c%c: %4.1f%%    \x00" as *const u8 as
                   *const i8,
               'a' as i32 +
                   move_list[i as usize].move_0 % 10 as i32 -
                   1 as i32,
               '0' as i32 +
                   move_list[i as usize].move_0 / 10 as i32,
               100.0f64 *
                   move_list[i as usize].frequency as f64 /
                   freq_sum as f64);
        if i % 6 as i32 == 4 as i32 {
            puts(b"\x00" as *const u8 as *const i8);
        }
        i += 1
    }
    if match_count % 6 as i32 != 5 as i32 {
        puts(b"\x00" as *const u8 as *const i8);
    }
}
/*
  DATABASE_SEARCH
  Determines what positions in the Thor database match the position
  given by IN_BOARD with SIDE_TO_MOVE being the player whose turn it is.
*/

pub unsafe fn database_search(mut in_board: *mut i32,
                                         mut side_to_move: i32) {
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
            safe_malloc((thor_game_count as
                             u64).wrapping_mul(::std::mem::size_of::<*mut GameType>()
                                                             as
                                                             u64))
                as *mut *mut GameType;
        thor_search.allocation = thor_game_count
    } else if thor_search.allocation < thor_game_count {
        free(thor_search.match_list as *mut libc::c_void);
        thor_search.match_list =
            safe_malloc((thor_game_count as
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
        sort_thor_games(thor_game_count);
        j = 0 as i32;
        while j < thor_game_count {
            (**thor_search.match_list.offset(j as isize)).sort_order = j;
            j += 1
        }
        thor_games_sorted = 1 as i32
    }
    /* Determine disc count, hash codes, patterns and opening
       for the position */
    disc_count[2 as i32 as usize] = 0 as i32;
    disc_count[0 as i32 as usize] =
        disc_count[2 as i32 as usize];
    i = 1 as i32;
    while i <= 8 as i32 {
        j = 1 as i32;
        pos = 10 as i32 * i + 1 as i32;
        while j <= 8 as i32 {
            if *in_board.offset(pos as isize) == 0 as i32 {
                disc_count[0 as i32 as usize] += 1
            } else if *in_board.offset(pos as isize) == 2 as i32 {
                disc_count[2 as i32 as usize] += 1
            }
            j += 1;
            pos += 1
        }
        i += 1
    }
    move_count =
        disc_count[0 as i32 as usize] +
            disc_count[2 as i32 as usize] - 4 as i32;
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
                    shape_lo[0 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[0 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * i + (7 as i32 - j);
                if index < 32 as i32 {
                    shape_lo[1 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[1 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * j + i;
                if index < 32 as i32 {
                    shape_lo[2 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[2 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * j + (7 as i32 - i);
                if index < 32 as i32 {
                    shape_lo[3 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[3 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * (7 as i32 - i) + j;
                if index < 32 as i32 {
                    shape_lo[4 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[4 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index =
                    8 as i32 * (7 as i32 - i) +
                        (7 as i32 - j);
                if index < 32 as i32 {
                    shape_lo[5 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[5 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index = 8 as i32 * (7 as i32 - j) + i;
                if index < 32 as i32 {
                    shape_lo[6 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[6 as i32 as usize] |=
                        ((1 as i32) << index - 32 as i32) as
                            u32
                }
                index =
                    8 as i32 * (7 as i32 - j) +
                        (7 as i32 - i);
                if index < 32 as i32 {
                    shape_lo[7 as i32 as usize] |=
                        ((1 as i32) << index) as u32
                } else {
                    shape_hi[7 as i32 as usize] |=
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
        get_corner_mask(*in_board.offset(11 as i32 as isize),
                        *in_board.offset(81 as i32 as isize),
                        *in_board.offset(18 as i32 as isize),
                        *in_board.offset(88 as i32 as isize));
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
                if disc_count[0 as i32 as usize] ==
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
    thor_search.draws = frequency[32 as i32 as usize];
    sum += 32 as i32 * frequency[32 as i32 as usize];
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
        cumulative[0 as i32 as usize] =
            frequency[0 as i32 as usize];
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
/*
  PRINT_THOR_MATCHES
  Outputs the MAX_GAMES first games found by the latest
  database search to STREAM.
*/

pub unsafe fn print_thor_matches(mut stream: *mut FILE,
                                            mut max_games: i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i <
              (if thor_search.match_count < max_games {
                   thor_search.match_count
               } else { max_games }) {
        if i == 0 as i32 {
            fputs(b"\n\x00" as *const u8 as *const i8, stream);
        }
        print_game(stream, *thor_search.match_list.offset(i as isize),
                   0 as i32);
        i += 1
    };
}

#[no_mangle]
pub unsafe extern "C" fn thordb_report_flipped_0_second() {
    puts(b"This COULD happen (2) in BUILD_THOR_OPENING_TREE\x00" as *const u8 as *const i8);
}

#[no_mangle]
pub unsafe extern "C" fn thordb_report_flipped_0_first() {
    puts(b"This COULD happen (1) in BUILD_THOR_OPENING_TREE\x00" as *const u8 as *const i8);
}
