use crate::src::stubs::{puts, abs, fputs, free, printf, qsort, fprintf, fclose, fopen, fread, strchr, strcmp};
use crate::src::safemem::{safe_malloc, safe_realloc};
use crate::src::zebra::_IO_FILE;
pub use engine::src::thordb::*;
use crate::src::error::LibcFatalError;
use engine::src::error::FrontEnd;

pub type FE = LibcFatalError;

pub type size_t = u64;

pub type FILE = _IO_FILE;
pub type __compar_fn_t
    =
    Option<unsafe fn(_: *const std::ffi::c_void,
                                _: *const std::ffi::c_void) -> i32>;
/*
  GET_INT_8
  Reads an 8-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
unsafe fn get_int_8(stream: *mut FILE, value: *mut int_8)
 -> i32 {
    let mut actually_read: i32 = 0;
    actually_read =
        fread(value as *mut std::ffi::c_void,
              ::std::mem::size_of::<int_8>() as u64,
              1 as i32 as size_t, stream) as i32;
    return (actually_read == 1 as i32) as i32;
}
/*
  GET_INT_16
  Reads a 16-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
unsafe fn get_int_16(stream: *mut FILE, value: *mut int_16)
 -> i32 {
    let mut actually_read: i32 = 0;
    actually_read =
        fread(value as *mut std::ffi::c_void,
              ::std::mem::size_of::<int_16>() as u64,
              1 as i32 as size_t, stream) as i32;
    return (actually_read == 1 as i32) as i32;
}
/*
  GET_INT_32
  Reads a 32-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
unsafe fn get_int_32(stream: *mut FILE, value: *mut int_32)
 -> i32 {
    let mut actually_read: i32 = 0;
    actually_read =
        fread(value as *mut std::ffi::c_void,
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
unsafe fn read_prolog(stream: *mut FILE,
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
unsafe extern "C" fn thor_compare_tournaments(t1: *const std::ffi::c_void,
                                              t2: *const std::ffi::c_void)
 -> i32 {
    let tournament1 = *(t1 as *mut *mut TournamentType);
    let tournament2 = *(t2 as *mut *mut TournamentType);
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
        safe_malloc::<FE>((tournaments.count as
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
    qsort(tournament_buffer as *mut std::ffi::c_void, tournaments.count as size_t,
          ::std::mem::size_of::<*mut TournamentType>() as u64,
          Some(thor_compare_tournaments as
                   unsafe extern "C" fn(_: *const std::ffi::c_void,
                                        _: *const std::ffi::c_void)
                       -> i32));
    i = 0 as i32;
    while i < tournaments.count {
        (**tournament_buffer.offset(i as isize)).lex_order = i;
        i += 1
    }
    free(tournament_buffer as *mut std::ffi::c_void);
}
/*
  READ_TOURNAMENT_DATABASE
  Reads the tournament database from FILE_NAME.
  Returns TRUE if all went well, otherwise FALSE.
*/

pub unsafe fn read_tournament_database(file_name:
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
        safe_realloc::<FE>(tournaments.name_buffer as *mut std::ffi::c_void,
                     buffer_size as size_t) as *mut i8;
    actually_read =
        fread(tournaments.name_buffer as *mut std::ffi::c_void,
              1 as i32 as size_t, buffer_size as size_t, stream) as
            i32;
    success = (actually_read == buffer_size) as i32;
    fclose(stream);
    if success != 0 {
        tournaments.tournament_list =
            safe_realloc::<FE>(tournaments.tournament_list as *mut std::ffi::c_void,
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
unsafe extern "C" fn thor_compare_players(p1: *const std::ffi::c_void,
                                          p2: *const std::ffi::c_void)
 -> i32 {
    let mut ch: i8 = 0;
    let mut buffer1: [i8; 20] = [0; 20];
    let mut buffer2: [i8; 20] = [0; 20];
    let mut i: i32 = 0;
    let player1 = *(p1 as *mut *mut PlayerType);
    let player2 = *(p2 as *mut *mut PlayerType);
    i = 0 as i32;
    loop  {
        ch = *(*player1).name.offset(i as isize);
        buffer1[i as usize] =FE::tolower(ch as i32) as i8;
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
        buffer2[i as usize] =FE::tolower(ch as i32) as i8;
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
        safe_malloc::<FE>((players.count as
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
    qsort(player_buffer as *mut std::ffi::c_void, players.count as size_t,
          ::std::mem::size_of::<*mut PlayerType>() as u64,
          Some(thor_compare_players as
                   unsafe extern "C" fn(_: *const std::ffi::c_void,
                                        _: *const std::ffi::c_void)
                       -> i32));
    i = 0 as i32;
    while i < players.count {
        (**player_buffer.offset(i as isize)).lex_order = i;
        i += 1
    }
    free(player_buffer as *mut std::ffi::c_void);
}
/*
  READ_PLAYER_DATABASE
  Reads the player database from FILE_NAME.
  Returns TRUE if all went well, otherwise FALSE.
*/

pub unsafe fn read_player_database(file_name:
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
        safe_realloc::<FE>(players.name_buffer as *mut std::ffi::c_void,
                     buffer_size as size_t) as *mut i8;
    actually_read =
        fread(players.name_buffer as *mut std::ffi::c_void,
              1 as i32 as size_t, buffer_size as size_t, stream) as
            i32;
    success = (actually_read == buffer_size) as i32;
    fclose(stream);
    if success != 0 {
        players.player_list =
            safe_realloc::<FE>(players.player_list as *mut std::ffi::c_void,
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
unsafe fn read_game(stream: *mut FILE, mut game: *mut GameType)
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
                  *mut std::ffi::c_void, 1 as i32 as size_t,
              60 as i32 as size_t, stream) as i32;
    prepare_game(game);
    return (success != 0 && actually_read == 60 as i32) as
               i32;
}
/*
  READ_GAME_DATABASE
  Reads a game database from FILE_NAME.
*/

pub unsafe fn read_game_database(file_name:
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
        safe_malloc::<FE>(::std::mem::size_of::<DatabaseType>() as u64) as
            *mut DatabaseType;
    (*database_head).next = old_database_head;
    if read_prolog(stream, &mut (*database_head).prolog) == 0 {
        fclose(stream);
        return 0 as i32
    }
    success = 1 as i32;
    (*database_head).count = (*database_head).prolog.game_count;
    (*database_head).games =
        safe_malloc::<FE>(((*database_head).count as
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

pub unsafe fn game_database_already_loaded(file_name:
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
unsafe fn print_game(stream: *mut FILE,
                                game: *mut GameType,
                                display_moves: i32) {
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
// This is a wrapper around thor_compare that has C linkage,
// because we don't want any C linkage in the engine
pub unsafe extern "C" fn extern_thor_compare(g1: *const std::ffi::c_void,
                                             g2: *const std::ffi::c_void)
                                             -> i32 {
    thor_compare(g1, g2)
}

/*
  SORT_THOR_GAMES
  Sorts the COUNT first games in the list THOR_SEARCH.MATCH_LIST.
  The first THOR_SORT_CRITERIA_COUNT entries of THOR_SORT_ORDER are
  used (in order) to sort the matches.
*/
pub unsafe fn sort_thor_games(count: i32) {
    if count <= 1 as i32 {
        /* No need to sort 0 or 1 games. */
        return
    }
    qsort(thor_search.match_list as *mut std::ffi::c_void, count as size_t,
          ::std::mem::size_of::<*mut GameType>() as u64,
          Some(extern_thor_compare as
                   unsafe extern "C" fn(_: *const std::ffi::c_void,
                                        _: *const std::ffi::c_void)
                       -> i32));
}

pub unsafe fn choose_thor_opening_move_report(
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
  PRINT_THOR_MATCHES
  Outputs the MAX_GAMES first games found by the latest
  database search to STREAM.
*/

pub unsafe fn print_thor_matches(stream: *mut FILE,
                                            max_games: i32) {
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
