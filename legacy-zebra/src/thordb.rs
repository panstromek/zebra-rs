use libc_wrapper::{fclose, fopen};
use crate::src::error::LibcFatalError;
use engine::src::stubs::abs;
use thordb_types::{Int8, Int16, Int32, OpeningNodeRef, ThorOpeningTree};

use engine::src::bitboard::bit_reverse_32;

use engine::src::moves::dir_mask;
use thordb_types::{GameType, DatabaseType, C2RustUnnamed, EITHER_SELECTED_FILTER,
                       TournamentType, PlayerType, ThorOpeningNode,
                       GameInfoType, DatabaseInfoType, FilterType,
                       PrologType, TournamentDatabaseType, SearchResultType, PlayerDatabaseType};
use thor_opening_list::THOR_OPENING_LIST;
use engine_traits::Offset;
use engine::src::patterns::pow3;
use engine::src::thordb::ThorDatabase;
use crate::src::zebra::{FullState};
use engine::src::myrandom::MyRandom;
use engine::src::getcoeff::odometer_principle;
use std::io::{Read, Write};
use engine::src::game::to_lower;
use std::cmp::Ordering;
use std::fs::File;
use std::ptr::null_mut;

/* Local variables */
static mut thor_game_count: i32 = 0;
static mut thor_sort_criteria_count: i32 = 0;
static mut thor_games_sorted: i32 = 0;
static mut thor_games_filtered: i32 = 0;
struct SymmetryMaps {
    b1_b1_map: [i32; 100],
    g1_b1_map: [i32; 100],
    g8_b1_map: [i32; 100],
    b8_b1_map: [i32; 100],
    a2_b1_map: [i32; 100],
    a7_b1_map: [i32; 100],
    h7_b1_map: [i32; 100],
    h2_b1_map: [i32; 100],
}
static SYMMENTRY_MAPS: SymmetryMaps = create_symetry_maps();

struct ThorBoard {
    side_to_move: i32,
    board: [i32; 100],
}

static mut board: ThorBoard = ThorBoard {
    side_to_move: 0,
    board: [0; 100]
};

struct ThorHash {
    primary_hash: [[u32; 6561]; 8],
    secondary_hash: [[u32; 6561]; 8],
    thor_row_pattern: [i32; 8],
    thor_col_pattern: [i32; 8],
}

static mut thor_hash: ThorHash = ThorHash {
    primary_hash: [[0; 6561]; 8],
    secondary_hash: [[0; 6561]; 8],
    thor_row_pattern: [0; 8],
    thor_col_pattern: [0; 8],
};

static mut symmetry_map: [&'static [i32]; 8] = [&[]; 8];
static mut inv_symmetry_map: [&'static [i32]; 8] = [&[]; 8];
static move_mask_hi: [u32; 100] = init_move_masks()[0];
static move_mask_lo: [u32; 100] = init_move_masks()[1];
static unmove_mask_hi: [u32; 100] = init_move_masks()[2];
static unmove_mask_lo: [u32; 100] = init_move_masks()[3];
static mut database_head: Option<Box<DatabaseType>> = None;
static mut players: PlayerDatabaseType =
    PlayerDatabaseType{prolog:
    PrologType{creation_century: 0,
        creation_year: 0,
        creation_month: 0,
        creation_day: 0,
        game_count: 0,
        item_count: 0,
        origin_year: 0,
        reserved: 0,},
        name_buffer:
        b"",
        player_list: Vec::new(),};
static mut thor_search: SearchResultType =
    SearchResultType{average_black_score: 0.,
        next_move_score: [0.; 100],
        match_count: 0,
        black_wins: 0,
        draws: 0,
        white_wins: 0,
        median_black_score: 0,
        allocation: 0,
        next_move_frequency: [0; 100],
        match_list: Vec::new(),
    };
static mut tournaments: TournamentDatabaseType =
    TournamentDatabaseType{prolog:
    PrologType{creation_century: 0,
        creation_year: 0,
        creation_month: 0,
        creation_day: 0,
        game_count: 0,
        item_count: 0,
        origin_year: 0,
        reserved: 0,},
        name_buffer: b"",
        tournament_list: Vec::new()};
static mut thor_opening_tree: ThorOpeningTree = ThorOpeningTree::new();
static default_sort_order: [i32; 5] = [2, 3, 1, 5, 4];
static mut thor_sort_order: [i32; 10] = [0; 10];
static mut filter: FilterType =
    FilterType{game_categories: 0,
        first_year: 0,
        last_year: 0,
        player_filter: EITHER_SELECTED_FILTER,};


pub struct LegacyThor;

impl ThorDatabase for LegacyThor {
    fn choose_thor_opening_move_report(freq_sum: i32, match_count: i32, move_list: &[C2RustUnnamed; 64]) {
        { LibcFatalError::choose_thor_opening_move_report(freq_sum, match_count, move_list) }
    }

    fn get_thor_game_move(index: i32, move_number: i32) -> i32 {
        unsafe { get_thor_game_move(index, move_number) }
    }

    fn database_search(in_board: &[i32], side_to_move: i32) {
        unsafe { database_search(in_board, side_to_move) }
    }

    fn get_match_count() -> i32 {
        unsafe { thor_search.get_match_count() }
    }

    fn get_black_win_count() -> i32 {
        unsafe { thor_search.get_black_win_count() }
    }

    fn get_draw_count() -> i32 {
        unsafe { thor_search.get_draw_count() }
    }

    fn get_white_win_count() -> i32 {
        unsafe { thor_search.get_white_win_count() }
    }

    fn get_black_median_score() -> i32 {
        unsafe { thor_search.get_black_median_score() }
    }

    fn get_black_average_score() -> f64 {
        unsafe { thor_search.get_black_average_score() }
    }

    fn choose_thor_opening_move(in_board: &[i32], side_to_move: i32, echo: i32, random: &mut MyRandom) -> i32 {
        unsafe { choose_thor_opening_move(in_board, side_to_move, echo, random) }
    }
}



pub type FE = LibcFatalError;

/*
  GET_INT_8
  Reads an 8-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
fn get_int_8(stream: &mut impl Read, value: &mut Int8) -> i32 {
    let mut buf = [0_u8;1];
    match stream.read_exact(&mut buf) {
        Ok(_) => {
            *value = buf[0] as i8;
            1
        },
        Err(_) => 0,
    }
}
/*
  GET_INT_16
  Reads a 16-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
fn get_int_16(mut stream: &mut impl Read, value: &mut Int16)
                     -> i32 {
    let mut buf = [0_u8;2];
    match stream.read_exact(&mut buf) {
        Ok(_) => {
            *value = Int16::from_le_bytes(buf);
            1
        },
        Err(_) => 0,
    }
}
/*
  GET_INT_32
  Reads a 32-bit signed integer from STREAM. Returns TRUE upon
  success, FALSE otherwise.
*/
fn get_int_32(mut stream: &mut impl Read, value: &mut Int32) -> i32 {
    let mut buf = [0_u8; 4];
    match stream.read_exact(&mut buf) {
        Ok(_) => {
            *value = Int32::from_le_bytes(buf);
            1
        }
        Err(_) => 0,
    }
}
/*
  READ_PROLOG
  Reads the prolog from STREAM into PROLOG. As the prolog is common
  for all the three database types (game, player, tournament) also
  values which aren't used are read.
  Returns TRUE upon success, otherwise FALSE.
*/
fn read_prolog(stream: &mut impl Read, mut prolog: &mut PrologType) -> i32 {
    let mut success: i32 = 0;
    let mut byte_val: Int8 = 0;
    let mut word_val: Int16 = 0;
    let mut longint_val: Int32 = 0;
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
fn thor_compare_tournaments(tournament1: &TournamentType, tournament2: &TournamentType) -> Ordering {
    return tournament1.name.cmp(tournament2.name);
}
/*
  SORT_TOURNAMENT_DATABASE
  Computes the lexicographic order of all tournaments in the database.
*/
fn sort_tournament_database(db: &mut [TournamentType]) {
    let mut tournament_buffer = db.iter_mut().collect::<Vec<_>>();
    tournament_buffer.sort_by(|t1, t2| thor_compare_tournaments(t1, t2));
    tournament_buffer.into_iter().enumerate().for_each(|(i, tournament) | tournament.lex_order = i as i32);
}
/*
  READ_TOURNAMENT_DATABASE
  Reads the tournament database from FILE_NAME.
  Returns TRUE if all went well, otherwise FALSE.
*/

pub unsafe fn read_tournament_database(file_name:
                                                      *const i8)
 -> i32 {
    let mut i: i32 = 0;
    let mut success: i32 = 0;
    let mut actually_read: i32 = 0;
    let mut buffer_size: i32 = 0;
    let mut stream = fopen(file_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() { return 0 }
    if read_prolog(&mut stream, &mut tournaments.prolog) == 0 {
        fclose(stream);
        return 0
    }
    let tournaments_count = tournaments.prolog.item_count;
    buffer_size = 26 * tournaments.prolog.item_count;
    let mut name_buffer = Vec::new();
    name_buffer.resize(buffer_size as usize, 0);

    actually_read = stream.read(&mut name_buffer).unwrap()/*.map_or(0, |_| name_buffer.len())*/ as i32;
    tournaments.name_buffer = Vec::leak(name_buffer);
    success = (actually_read == buffer_size) as i32;
    fclose(stream);
    if success != 0 {
        tournaments.tournament_list.resize(tournaments_count as usize, TournamentType {
            lex_order: 0,
            selected: 0,
            name: &[]
        });
        i = 0;
        while i < tournaments_count {
            tournaments.tournament_list[i as usize] = (TournamentType {
                lex_order: 0,
                selected: 1,
                name: tournaments.tournament_name(i)
            });
            i += 1
        }
        sort_tournament_database(&mut tournaments.tournament_list);
        thor_games_sorted = 0;
        thor_games_filtered = 0
    }
    return success;
}
/*
  THOR_COMPARE_PLAYERS
  Lexicographically compares the names of two players
  represented by pointers.
*/
fn thor_compare_players(player1: &PlayerType, player2: &PlayerType) -> Ordering {
    let mut buffer1: [u8; 20] = [0; 20];
    let mut buffer2: [u8; 20] = [0; 20];
    let mut i = 0;
    let first_len = buffer1.len().min(player1.name.len());
    while i < first_len {
        let ch = *player1.name.offset(i as isize) as u8;
        buffer1[i as usize] = to_lower(ch as i32) as u8;
        i += 1;
    }
    if buffer1[0] == b'?' {
        /* Put unknown players LAST */
        buffer1[0] = b'~'
    }
    let mut i = 0;
    let second_len = buffer2.len().min(player2.name.len());

    while i < second_len {
        let ch = *player2.name.offset(i as isize) as u8;
        buffer2[i as usize] = to_lower(ch as i32) as u8;
        i += 1;
    }
    if buffer2[0] == b'?' {
        /* Put unknown players LAST */
        buffer2[0] = b'~'
    }

    return buffer1.cmp(&buffer2)
}
/*
  SORT_PLAYER_DATABASE
  Computes the lexicographic order of all players in the database.
*/
fn sort_player_database(db: &mut [PlayerType]) {
    let mut player_buffer = db.iter_mut().collect::<Vec<_>>();
    player_buffer.sort_by(|p1, p2| thor_compare_players(p1, p2));
    player_buffer.into_iter().enumerate().for_each(|(i, p)| p.lex_order = i as i32);
}
/*
  READ_PLAYER_DATABASE
  Reads the player database from FILE_NAME.
  Returns TRUE if all went well, otherwise FALSE.
*/

pub unsafe fn read_player_database(file_name: &str) -> i32 {
    let mut success: i32 = 0;
    let mut actually_read: i32 = 0;
    let mut buffer_size: i32 = 0;
    let mut stream = match File::open(file_name) {
        Ok(s) => s,
        Err(_) => return 0,
    };
    if read_prolog(&mut stream, &mut players.prolog) == 0 {
        return 0
    }
    let players_count = players.prolog.item_count;
    buffer_size = 20 * players_count;
    let mut players_name_buffer = Vec::new();
    players_name_buffer.resize(buffer_size as usize, 0);
    actually_read = stream.read(&mut players_name_buffer).unwrap() as i32 ;//_or( players_name_buffer.len()) as i32;

    players.name_buffer = Vec::leak(players_name_buffer);
    success = (actually_read == buffer_size) as i32;
    drop(stream);
    if success != 0 {
        players.player_list = vec![PlayerType::default(); players_count as usize];
        let mut i = 0;
        while i < players_count {
            let name = players.get_player_name(i);
            players.player_list[i as usize] = (PlayerType {
                lex_order: 0,
                /* By convention, names of computer programs always contain
                 parenthesis within which the name of the creator of the
                  program is given. E.g. "Zebra (andersson)", "Sethos()". */
                is_program: name.contains(&b'(') as i32,
                selected: 1,
                name
            });
            i += 1
        }
        sort_player_database(&mut players.player_list);
        thor_games_sorted = 0;
        thor_games_filtered = 0
    }
    return success;
}
/*
  READ_GAME
  Reads a game from STREAM in GAME and prepares the game
  for database questions. Returns TRUE upon success,
  otherwise FALSE.
*/
unsafe fn read_game(mut stream: &mut impl Read, mut game: &mut GameType) -> i32 {
    let mut success: i32 = 0;
    let mut byte_val: Int8 = 0;
    let mut word_val: Int16 = 0;
    success = get_int_16(stream, &mut word_val);
    (*game).tournament_no = word_val;
    success = (success != 0 && get_int_16(stream, &mut word_val) != 0) as i32;
    (*game).black_no = word_val;
    success = (success != 0 && get_int_16(stream, &mut word_val) != 0) as i32;
    (*game).white_no = word_val;
    success = (success != 0 && get_int_8(stream, &mut byte_val) != 0) as i32;
    (*game).actual_black_score = byte_val as i16;
    success = (success != 0 && get_int_8(stream, &mut byte_val) != 0) as i32;
    (*game).perfect_black_score = byte_val as i16;
    let mut bytes = [0; 60];
    let actually_read = stream.read(&mut bytes).unwrap_or(0);
    (*game).moves.iter_mut().zip(bytes.iter().take(actually_read)).for_each(|(g, b)| {
        *g = *b as i8
    });
    prepare_game(game, &mut board, &mut thor_opening_tree);
    return (success != 0 && actually_read == 60) as i32;
}
/*
  READ_GAME_DATABASE
  Reads a game database from FILE_NAME.
*/

pub unsafe fn read_game_database(file_name: &str)
 -> i32 {
    let mut i: i32 = 0;
    let mut success: i32 = 0;
    let mut old_database_head = None;
    let mut stream = match File::open(file_name) {
        Ok(s) => s,
        Err(_) => return 0
    };
    old_database_head = database_head.take();
    let prolog_type = PrologType {
        creation_century: 0,
        creation_year: 0,
        creation_month: 0,
        creation_day: 0,
        game_count: 0,
        item_count: 0,
        origin_year: 0,
        reserved: 0
    };

    let mut db_head = DatabaseType {
        prolog: prolog_type,
        games: Vec::new(),
        count: 0,
        next: old_database_head
    };

    if read_prolog(&mut stream, &mut db_head.prolog) == 0 {
        // FIXME This is here to preserve consistency with the old version but seems wrong
        //  why we would assign database head when we fail to parse the game??
        database_head = Some(Box::new(db_head));
        return 0
    }
    success = 1;
    (db_head).count = (db_head).prolog.game_count;
    (db_head).games = vec![GameType::new(); (db_head).count as usize];
    i = 0;
    let mut db_head = Box::new(db_head);

    while i < (db_head).count {
        success =
            (success != 0 &&
                 read_game(&mut stream,
                           &mut *(db_head).games.offset(i as isize)) !=
                     0) as i32;
        let ref mut fresh4 =
            (*(db_head).games.offset(i as isize)).origin_year;
        *fresh4 = db_head.prolog.origin_year;
        i += 1
    }
    thor_game_count += (db_head).count;
    thor_games_sorted = 0;
    thor_games_filtered = 0;

    drop(stream);
    database_head = Some(db_head);
    return success;
}
/*
  GAME_DATABASE_ALREADY_LOADED
  Checks if the game database in FILE_NAME already exists in memory
  (the only thing actually checked is the prolog but this suffices
  according to the specification of the database format).
*/

pub unsafe fn game_database_already_loaded(file_name: &str) -> i32 {
    let mut new_prolog =
        PrologType{creation_century: 0,
                   creation_year: 0,
                   creation_month: 0,
                   creation_day: 0,
                   game_count: 0,
                   item_count: 0,
                   origin_year: 0,
            reserved: 0,
        };
    let mut stream = match File::open(file_name) {
        Ok(s) => s,
        Err(_) => return 0
    };
    if read_prolog(&mut stream, &mut new_prolog) == 0 {
        return 0
    }
    drop(stream);
    let mut current_db_ = &database_head;
    while let Some(current_db) = current_db_ {
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
            return 1
        }
        current_db_ = &(*current_db).next
    }
    return 0;
}

/*
  PRINT_GAME
  Outputs the information about the game GAME to STREAM.
  The flag DISPLAY_MOVES specifies if the moves of the
  game are to be output or not.
*/
fn print_game(stream: &mut impl Write, game: &GameType, display_moves: i32, tournaments_: &TournamentDatabaseType, players_: &PlayerDatabaseType) {
    stream.write(tournaments_.tournament_name(game.tournament_no as i32));
    write!(stream, "  {}\n", game.origin_year);
    stream.write(players_.get_player_name(game.black_no as i32));
    stream.write(b" vs ");
    stream.write(players_.get_player_name(game.white_no as i32));
    stream.write(b"\n");
    write!(stream, "{} - {}   ", game.actual_black_score, 64 - game.actual_black_score);
    write!(stream, "[ {} - {} {} ]\n", game.perfect_black_score, 64 - game.perfect_black_score, "perfect");
    if display_moves != 0 {
        let mut i = 0;
        while i < 60 {
            write!(stream, " {}", abs(game.moves[i as usize] as i32));
            if i % 20 == 19 {
                stream.write(b"\n");
            }
            i += 1
        }
    }
    stream.write(b"\n");
}

/*
  SORT_THOR_GAMES
  Sorts the COUNT first games in the list THOR_SEARCH.MATCH_LIST.
  The first THOR_SORT_CRITERIA_COUNT entries of THOR_SORT_ORDER are
  used (in order) to sort the matches.
*/
pub unsafe fn sort_thor_games(count: i32) {
    if count <= 1 {
        /* No need to sort 0 or 1 games. */
        return
    }
    let sord_order = &thor_sort_order[0..thor_sort_criteria_count as usize];
    thor_search.match_list.sort_by(|g1, g2| {
        match unsafe { thor_compare(&**g1, &**g2, sord_order, &players, &tournaments) } {
            i32::MIN..=-1_i32 => Ordering::Less,
            0 => Ordering::Equal,
            1_i32..=i32::MAX => Ordering::Greater,
        }
    });
}

/*
  PRINT_THOR_MATCHES
  Outputs the MAX_GAMES first games found by the latest
  database search to STREAM.
*/

pub unsafe fn print_thor_matches(mut stream: &mut impl Write, max_games: i32) {
    let mut i: i32 = 0;
    while i < (if thor_search.match_count < max_games { thor_search.match_count } else { max_games }) {
        if i == 0 {
            stream.write(b"\n");
        }
        print_game(&mut stream, &**thor_search.match_list.offset(i as isize), 0, &tournaments, &players);
        i += 1
    };
}

/*
  CLEAR_THOR_BOARD
*/
fn clear_thor_board(thor_board_: &mut [i32; 100]) {
    let mut pos = 11;
    while pos <= 88 {
        thor_board_[pos as usize] = 1;
        pos += 1
    }
    thor_board_[54] = 0;
    thor_board_[45] = thor_board_[54];
    thor_board_[55] = 2;
    thor_board_[44] = thor_board_[55];
}
/*
  PREPARE_THOR_BOARD
  Mark the positions outside the board as OUTSIDE.
*/
fn prepare_thor_board(thor_board_: &mut [i32; 100]) {
    let mut i = 0;
    while i < 10 {
        let mut j = 0;
        let mut pos = 10 * i;
        while j < 10 {
            if i == 0 || i == 9 || j == 0 || j == 9 {
                thor_board_[pos as usize] = 3
            }
            j += 1;
            pos += 1
        }
        i += 1
    };
}
/*
  DIRECTIONAL_FLIP_COUNT
  Count the number of discs flipped in the direction given by INC
  when SQ is played by COLOR and flip those discs.
*/
fn directional_flip_count(
    sq: i32,
    inc: i32,
    color: i32,
    oppcol: i32,
    thor_board_: &mut [i32; 100],
) -> i32 {
    let mut count = 1;
    let mut pt = sq + inc;
    if thor_board_[pt as usize] == oppcol {
        pt += inc;
        if thor_board_[pt as usize] == oppcol {
            count += 1;
            pt += inc;
            if thor_board_[pt as usize] == oppcol {
                count += 1;
                pt += inc;
                if thor_board_[pt as usize] == oppcol {
                    count += 1;
                    pt += inc;
                    if thor_board_[pt as usize] == oppcol {
                        count += 1;
                        pt += inc;
                        if thor_board_[pt as usize] == oppcol {
                            count += 1;
                            pt += inc
                        }
                    }
                }
            }
        }
        if thor_board_[pt as usize] == color {
            let mut g = count;
            loop  {
                pt -= inc;
                thor_board_[pt as usize] = color;
                g -= 1;
                if !(g != 0) { break ; }
            }
            return count
        }
    }
    0
}
/*
  DIRECTIONAL_FLIP_ANY
  Returns 1 if SQ is feasible for COLOR in the direction given by INC
  and flip the discs which are flipped if SQ is played.
*/
fn directional_flip_any(sq: i32,
                               inc: i32,
                               color: i32,
                               oppcol: i32,
                               thor_board_: &mut [i32; 100])
                               -> i32 {
    let mut pt = sq + inc;
    if thor_board_[pt as usize] == oppcol {
        pt += inc;
        if thor_board_[pt as usize] == oppcol {
            pt += inc;
            if thor_board_[pt as usize] == oppcol {
                pt += inc;
                if thor_board_[pt as usize] == oppcol {
                    pt += inc;
                    if thor_board_[pt as usize] == oppcol {
                        pt += inc;
                        if thor_board_[pt as usize] == oppcol { pt += inc }
                    }
                }
            }
        }
        if thor_board_[pt as usize] == color {
            pt -= inc;
            loop  {
                thor_board_[pt as usize] = color;
                pt -= inc;
                if !(pt != sq) { break ; }
            }
            return 1
        }
    }
    return 0;
}
/*
  COUNT_FLIPS
  Returns the number of discs flipped if SQNUM is played by COLOR
  and flips those discs (if there are any).
*/
fn count_flips(
    sqnum: i32,
    color: i32,
    oppcol: i32,
    thor_board_: &mut [i32; 100],
) -> i32 {
    let mut count = 0;
    let mask = dir_mask[sqnum as usize];
    if mask & 128 != 0 {
        count += directional_flip_count(sqnum, -(11), color, oppcol, thor_board_)
    }
    if mask & 64 != 0 {
        count += directional_flip_count(sqnum, 11, color, oppcol, thor_board_)
    }
    if mask & 32 != 0 {
        count += directional_flip_count(sqnum, -(10), color, oppcol, thor_board_)
    }
    if mask & 16 != 0 {
        count += directional_flip_count(sqnum, 10, color, oppcol, thor_board_)
    }
    if mask & 8 != 0 {
        count += directional_flip_count(sqnum, -(9), color, oppcol, thor_board_)
    }
    if mask & 4 != 0 {
        count += directional_flip_count(sqnum, 9, color, oppcol, thor_board_)
    }
    if mask & 2 != 0 {
        count += directional_flip_count(sqnum, -1, color, oppcol, thor_board_)
    }
    if mask & 1 != 0 {
        count += directional_flip_count(sqnum, 1, color, oppcol, thor_board_)
    }
    return count;
}
/*
  ANY_FLIPS
  Returns 1 if SQNUM flips any discs for COLOR, otherwise 0, and
  flips those discs.
*/
fn any_flips(sqnum: i32, color: i32, oppcol: i32, thor_board_: &mut [i32; 100]) -> i32 {
    let mut count = 0;
    let mask = dir_mask[sqnum as usize];
    if mask & 128 != 0 {
        count |= directional_flip_any(sqnum, -(11), color, oppcol, thor_board_)
    }
    if mask & 64 != 0 {
        count |= directional_flip_any(sqnum, 11, color, oppcol, thor_board_)
    }
    if mask & 32 != 0 {
        count |= directional_flip_any(sqnum, -(10), color, oppcol, thor_board_)
    }
    if mask & 16 != 0 {
        count |= directional_flip_any(sqnum, 10, color, oppcol, thor_board_)
    }
    if mask & 8 != 0 {
        count |= directional_flip_any(sqnum, -(9), color, oppcol, thor_board_)
    }
    if mask & 4 != 0 {
        count |= directional_flip_any(sqnum, 9, color, oppcol, thor_board_)
    }
    if mask & 2 != 0 {
        count |= directional_flip_any(sqnum, -1, color, oppcol, thor_board_)
    }
    if mask & 1 != 0 {
        count |= directional_flip_any(sqnum, 1, color, oppcol, thor_board_)
    }
    return count;
}
/*
  COMPUTE_THOR_PATTERNS
  Computes the row and column patterns.

*/
fn compute_thor_patterns(
    row_pattern: &mut [i32; 8],
    col_pattern: &mut [i32; 8],
    in_board: &[i32],
) {
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut i = 0;
    while i < 8 {
        row_pattern[i as usize] = 0;
        col_pattern[i as usize] = 0;
        i += 1
    }
    i = 0;
    while i < 8 {
        j = 0;
        pos = 10 * i + 11;
        while j < 8 {
            row_pattern[i as usize] += pow3(j as usize) * *in_board.offset(pos as isize);
            col_pattern[j as usize] += pow3(i as usize) * *in_board.offset(pos as isize);
            j += 1;
            pos += 1
        }
        i += 1
    };
}
/*
  GET_CORNER_MASK
  Returns an 32-bit mask for the corner configuration. The rotation
  which minimizes the numerical value is chosen.
  The mask is to be interpreted as follows: There are two bits
  for each corner; 00 means empty, 01 means black and 10 means white.
  The bit blocks are given in the order h8h1a8a1 (MSB to LSB).
  Furthermore, this 8-bit value is put in the leftmost byte if
  all four corners have been played, in the rightmost byte if only
  one corner has been played (obvious generalization for one or two
  corners).
*/
fn get_corner_mask(disc_a1: i32, disc_a8: i32, disc_h1: i32, disc_h8: i32) -> u32 {
    let mut config: [u32; 8] = [0; 8];
    let mut mask_a1 = 0;
    if disc_a1 == 0 {
        mask_a1 = 1
    } else if disc_a1 == 2 {
        mask_a1 = 2
    }
    let mut mask_a8 = 0;
    if disc_a8 == 0 {
        mask_a8 = 1
    } else if disc_a8 == 2 {
        mask_a8 = 2
    }
    let mut mask_h1 = 0;
    if disc_h1 == 0 {
        mask_h1 = 1
    } else if disc_h1 == 2 {
        mask_h1 = 2
    }
    let mut mask_h8 = 0;
    if disc_h8 == 0 {
        mask_h8 = 1
    } else if disc_h8 == 2 {
        mask_h8 = 2
    }
    let mut count = 0;
    if disc_a1 != 1 { count += 1 }
    if disc_a8 != 1 { count += 1 }
    if disc_h1 != 1 { count += 1 }
    if disc_h8 != 1 { count += 1 }
    if count == 0 { return 0 }
    config[0] = (mask_a1 + 4 * mask_a8 + 16 * mask_h1 + 64 * mask_h8) as u32;
    config[1] = (mask_a1 + 4 * mask_h1 + 16 * mask_a8 + 64 * mask_h8) as u32;
    config[2] = (mask_a8 + 4 * mask_a1 + 16 * mask_h8 + 64 * mask_h1) as u32;
    config[3] = (mask_a8 + 4 * mask_h8 + 16 * mask_a1 + 64 * mask_h1) as u32;
    config[4] = (mask_h1 + 4 * mask_h8 + 16 * mask_a1 + 64 * mask_a8) as u32;
    config[5] = (mask_h1 + 4 * mask_a1 + 16 * mask_h8 + 64 * mask_a8) as u32;
    config[6] = (mask_h8 + 4 * mask_h1 + 16 * mask_a8 + 64 * mask_a1) as u32;
    config[7] = (mask_h8 + 4 * mask_a8 + 16 * mask_h1 + 64 * mask_a1) as u32;
    let mut out_mask = config[0];
    let mut i = 1;
    while i < 8 {
        out_mask = if out_mask < config[i as usize] { out_mask } else { config[i as usize] };
        i += 1
    }
    return out_mask << 8 * (count - 1);
}

/*
  GET_DATABASE_INFO
  Fills the vector INFO with the origin years and number of games of
  all game databases loaded.
  Enough memory must have been allocated prior to this function being
  called, that this is the case can be checked by calling GET_DATABASE_COUNT
  above.
*/

unsafe fn get_database_info(info: &mut [DatabaseInfoType]) {
    let mut change: i32 = 0;
    let mut temp = DatabaseInfoType{year: 0, count: 0,};
    let mut current_db_ = &database_head;
    let mut i = 0;
    while let Some(current_db) = current_db_.as_ref() {
        (*info.offset(i as isize)).year = (*current_db).prolog.origin_year;
        (*info.offset(i as isize)).count = (*current_db).count;
        current_db_ = &(*current_db).next;
        i += 1
    }
    let database_count = i;
    /* Sort the list */
    loop {
        change = 0;
        let mut i = 0;
        while i < database_count - 1 {
            if (*info.offset(i as isize)).year > (*info.offset((i + 1) as isize)).year {
                change = 1;
                temp = *info.offset(i as isize);
                *info.offset(i as isize) = *info.offset((i + 1) as isize);
                *info.offset((i + 1) as isize) = temp
            }
            i += 1
        }
        if !(change != 0) {
            break;
        }
    };
}

impl ThorHash {
    /*
      COMPUTE_PARTIAL_HASH
      Computes the primary and secondary hash values for the
      unit element in the rotation group.
    */
    fn compute_partial_hash(&self, hash_val1: &mut u32, hash_val2: &mut u32) {
        let mut i: i32 = 0;
        *hash_val1 = 0;
        *hash_val2 = 0;
        i = 0;
        while i < 8 {
            *hash_val1 ^= self.primary_hash[i as usize][self.thor_row_pattern[i as usize] as usize];
            *hash_val2 ^= self.secondary_hash[i as usize][self.thor_row_pattern[i as usize] as usize];
            i += 1
        };
    }
    /*
      COMPUTE_FULL_PRIMARY_HASH
      COMPUTE_FULL_SECONDARY_HASH
      Compute the primary and secondary hash codes respectively
      for all elements in the rotation group.
      Note: The order of the hash codes must coincide with the
            definitions in INIT_SYMMETRY_MAPS().
    */
    fn compute_full_primary_hash(&self, hash_val: &mut [u32]) {
        let mut i = 0;
        while i < 4 {
            *hash_val.offset(i as isize) = 0;
            i += 1
        }
        i = 0;
        while i < 8 {
            /* b1 -> b1 */
            hash_val[0] ^= self.primary_hash[i as usize][self.thor_row_pattern[i as usize] as usize];
            /* b8 -> b1 */
            *hash_val.offset(1) ^= self.primary_hash[i as usize][self.thor_row_pattern[(7 - i) as usize] as usize];
            /* a2 -> b1 */
            *hash_val.offset(2) ^= self.primary_hash[i as usize][self.thor_col_pattern[i as usize] as usize];
            /* h2 -> b1 */
            *hash_val.offset(3) ^= self.primary_hash[i as usize][self.thor_col_pattern[(7 - i) as usize] as usize];
            i += 1
        }
        /* g1 -> b1 */
        *hash_val.offset(4) = bit_reverse_32(hash_val[0]);
        /* g8 -> b1 */
        *hash_val.offset(5) = bit_reverse_32(*hash_val.offset(1));
        /* a7 -> b1 */
        *hash_val.offset(6) = bit_reverse_32(*hash_val.offset(2));
        /* h7 -> b1 */
        *hash_val.offset(7) = bit_reverse_32(*hash_val.offset(3));
    }
    fn compute_full_secondary_hash(&self, hash_val: &mut [u32]) {
        let mut i = 0;
        while i < 4 {
            *hash_val.offset(i as isize) = 0;
            i += 1
        }
        i = 0;
        while i < 8 {
            /* b1 -> b1 */
            hash_val[0] ^= self.secondary_hash[i as usize][self.thor_row_pattern[i as usize] as usize];
            /* b8 -> b1 */
            *hash_val.offset(1) ^= self.secondary_hash[i as usize][self.thor_row_pattern[(7 - i) as usize] as usize];
            /* a2 -> b1 */
            *hash_val.offset(2) ^= self.secondary_hash[i as usize][self.thor_col_pattern[i as usize] as usize];
            /* h2 -> b1 */
            *hash_val.offset(3) ^= self.secondary_hash[i as usize][self.thor_col_pattern[(7 - i) as usize] as usize];
            i += 1
        }
        /* g1 -> b1 */
        *hash_val.offset(4) = bit_reverse_32(hash_val[0]);
        /* g8 -> b1 */
        *hash_val.offset(5) = bit_reverse_32(*hash_val.offset(1));
        /* a7 -> b1 */
        *hash_val.offset(6) = bit_reverse_32(*hash_val.offset(2));
        /* h7 -> b1 */
        *hash_val.offset(7) = bit_reverse_32(*hash_val.offset(3));
    }

    /*
      PRIMARY_HASH_LOOKUP
      Checks if any of the rotations of the current pattern set
      match the primary hash code TARGET_HASH.
    */
    fn primary_hash_lookup(&self, target_hash: u32) -> i32 {
        let mut i: i32 = 0;
        let mut hit_mask: i32 = 0;
        let mut hash_val: [u32; 8] = [0; 8];
        self.compute_full_primary_hash(&mut hash_val);
        hit_mask = 0;
        i = 0;
        while i < 8 {
            if hash_val[i as usize] == target_hash {
                hit_mask |= (1) << i
            }
            i += 1
        }
        return hit_mask;
    }
    /*
      SECONDARY_HASH_LOOKUP
      Checks if any of the rotations of the current pattern set
      match the secondary hash code TARGET_HASH.
    */
    fn secondary_hash_lookup(&self, target_hash: u32)
                             -> i32 {
        let mut i: i32 = 0;
        let mut hit_mask: i32 = 0;
        let mut hash_val: [u32; 8] = [0; 8];
        self.compute_full_secondary_hash(&mut hash_val);
        hit_mask = 0;
        i = 0;
        while i < 8 {
            if hash_val[i as usize] == target_hash {
                hit_mask |= (1) << i
            }
            i += 1
        }
        return hit_mask;
    }
}
/*
  FILTER_DATABASE
  Applies the current filter rules to the database DB.
*/
fn filter_database(db: &mut DatabaseType, tournaments_: &[TournamentType], players_: &[PlayerType], filter_: &FilterType) {
    let mut category: i32 = 0;
    let mut passes_filter: i32 = 0;
    let mut year: i32 = 0;
    let mut i = 0;
    while i < (*db).count {
        let game = (*db).games.offset(i as isize);
        passes_filter = 1;
        /* Apply the tournament filter */
        if passes_filter != 0 && (*tournaments_.offset((*game).tournament_no as isize)).selected == 0 {
            passes_filter = 0
        }
        /* Apply the year filter */
        if passes_filter != 0 {
            year = (*game).origin_year;
            if year < filter_.first_year || year > filter_.last_year {
                passes_filter = 0
            }
        }
        /* Apply the player filter */
        if passes_filter != 0 {
            match filter_.player_filter as u32 {
                0 => if (*players_.offset((*game).black_no as isize)).selected == 0 &&
                    (*players_.offset((*game).white_no as isize)).selected == 0 {
                    passes_filter = 0
                },
                1 => if (*players_.offset((*game).black_no as isize)).selected == 0
                    || (*players_.offset((*game).white_no as isize)).selected == 0 {
                    passes_filter = 0
                },
                2 => if (*players_.offset((*game).black_no as isize)).selected == 0 {
                    passes_filter = 0
                },
                3 => if (*players_.offset((*game).white_no as isize)).selected == 0 {
                    passes_filter = 0
                },
                _ => {}
            }
        }
        /* Apply the game type filter */
        if passes_filter != 0 {
            if (*players_.offset((*game).black_no as isize)).is_program != 0 {
                if (*players_.offset((*game).white_no as isize)).is_program != 0 {
                    category = 4
                } else {
                    category = 2
                }
            } else if (*players_.offset((*game).white_no as isize)).is_program != 0 {
                category = 2
            } else {
                category = 1
            }
            passes_filter = category & filter_.game_categories
        }
        (*game).passes_filter = passes_filter as i16;
        i += 1
    };
}
/*
  FILTER_ALL_DATABASES
  Applies the current filter rules to all databases.
*/
unsafe fn filter_all_databases() {
    let mut current_db_ = &mut database_head;
    while let Some(current_db) = current_db_ {
        filter_database(current_db, &tournaments.tournament_list, &players.player_list, &filter);
        current_db_ = &mut (*current_db).next
    };
}

/*
  SET_PLAYER_FILTER
  Specify what players to search for. The boolean vector SELECTED
  must contain at least PLAYERS.COUNT values - check with
  GET_PLAYER_COUNT() if necessary.
*/

unsafe fn set_player_filter(selected: &mut [i32]) {
    let mut i: i32 = 0;
    while i < players.count() {
        (*players.player_list.offset(i as isize)).selected = *selected.offset(i as isize);
        i += 1
    }
    thor_games_filtered = 0;
}

/*
  SET_TOURNAMENT_FILTER
  Specify what tournaments to search for. The boolean vector SELECTED
  must contain at least TOURNAMENTS.COUNT values - check with
  GET_TOURNAMENT_COUNT() if necessary.
*/

unsafe fn set_tournament_filter(selected: &mut [i32]) {
    let mut i: i32 = 0;
    while i < tournaments.count() {
        (*tournaments.tournament_list.offset(i as isize)).selected =
            *selected.offset(i as isize);
        i += 1
    }
    thor_games_filtered = 0;
}
/*
  SET_YEAR_FILTER
  Specify the interval of years to which the search will be confined.
*/

unsafe fn set_year_filter(first_year: i32, last_year: i32) {
    filter.first_year = first_year;
    filter.last_year = last_year;
    thor_games_filtered = 0;
}
/*
  SPECIFY_GAME_CATEGORIES
  Specify the types of games in the database that are displayed
  if they match the position probed for. The input is the binary
  OR of the flags for the types enabled.
*/

unsafe fn specify_game_categories(categories: i32) {
    if categories != filter.game_categories {
        filter.game_categories = categories;
        thor_games_filtered = 0
    };
}
/*
  SPECIFY_THOR_SORT_ORDER
  Specifies that in subsequent calls to SORT_THOR_MATCHES,
  the COUNT first elements of SORT_ORDER are to be used
  (in decreasing order of priority).
  Note: If there aren't (at least) COUNT elements at the location
        to which SORT_ORDER points, a crash is likely.
*/

unsafe fn specify_thor_sort_order(mut count: i32, sort_order: &[i32]) {
    /* Truncate the input vector if it is too long */
    count = if count < 10 { count } else { 10 };
    /* Check if the new order coincides with the old order */
    if count != thor_sort_criteria_count {
        thor_games_sorted = 0
    } else {
        let mut i = 0;
        while i < count {
            if sort_order[i as usize] != thor_sort_order[i as usize] {
                thor_games_sorted = 0
            }
            i += 1
        }
    }
    thor_sort_criteria_count = count;
    let mut i = 0;
    while i < count {
        thor_sort_order[i as usize] = sort_order[i as usize];
        i += 1
    };
}

/*
  RECURSIVE_OPENING_SCAN
  Performs a preorder traversal of the opening tree rooted
  at NODE and checks which opening nodes are compatible
  with the primary and secondary hash codes from the 8 different
  rotations.
*/
fn recursive_opening_scan(tree: &mut ThorOpeningTree,
                                 node: OpeningNodeRef,
                                 depth: i32,
                                 moves_played: i32,
                                 primary_hash_0: &mut [u32],
                                 secondary_hash_0: &mut [u32]) {
    /* Determine the status of the current node */
    if depth < moves_played {
        (tree[node]).matching_symmetry = 0;
        (tree[node]).current_match = 0
    } else if depth == moves_played {
        /* Check the hash codes */
        let mut match_0 = 0;
        let mut matching_symmetry = 0;
        let mut i = 7;
        while i >= 0 {
            if (tree[node]).hash1 == *primary_hash_0.offset(i as isize) &&
                (tree[node]).hash2 == *secondary_hash_0.offset(i as isize) {
                match_0 = 1;
                matching_symmetry = i
            }
            i -= 1
        }
        if match_0 != 0 {
            (tree[node]).matching_symmetry = matching_symmetry;
            (tree[node]).current_match = 1
        } else {
            (tree[node]).current_match = 2
        }
    } else {
        /* depth > moves_played */
        (tree[node]).current_match = (tree[(tree[node]).parent_node.unwrap()]).current_match;
        (tree[node]).matching_symmetry = (tree[(tree[node]).parent_node.unwrap()]).matching_symmetry
    }
    /* Recursively search the childen */
    let mut child = (tree[node]).child_node;
    while let Some(child_) = child {
        recursive_opening_scan(tree, child_, depth + 1, moves_played, primary_hash_0, secondary_hash_0);
        child = tree[child_].sibling_node
    };
}

/*
  OPENING_SCAN
  Fills the opening tree with information on how well
  the current pattern configuration matches the openings.
*/
fn opening_scan(moves_played: i32, thor_hash_: &mut ThorHash, tree: &mut ThorOpeningTree) {
    let mut primary_hash_0: [u32; 8] = [0; 8];
    let mut secondary_hash_0: [u32; 8] = [0; 8];
    thor_hash_.compute_full_primary_hash(&mut primary_hash_0);
    thor_hash_.compute_full_secondary_hash(&mut secondary_hash_0);
    recursive_opening_scan(tree, tree.root().unwrap(), 0, moves_played, &mut primary_hash_0, &mut secondary_hash_0);
}
/*
  RECURSIVE_FREQUENCY_COUNT
  Recursively fills frequency table FREQ_COUNT which is to contain
  the number of times each move has been played according to the
  trimmed set of openings from the Thor database.
*/
fn recursive_frequency_count(tree: &mut ThorOpeningTree,
                                    node: OpeningNodeRef,
                                    freq_count: &mut [i32],
                                    depth: i32,
                                    moves_played: i32,
                                    symmetries: &mut [i32],
                                    primary_hash_0: &mut [u32],
                                    secondary_hash_0: &mut [u32],
                                    inv_symmetry_map_: &[&[i32]; 8]) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut child_move: i32 = 0;
    if depth == moves_played {
        i = 0;
        while i < 8 {
            j = *symmetries.offset(i as isize);
            if (tree[node]).hash1 == *primary_hash_0.offset(j as isize) &&
                (tree[node]).hash2 == *secondary_hash_0.offset(j as isize) {
                child_move = (tree[node]).child_move as i32;
                let mut child = (tree[node]).child_node;
                while let Some(child_) = child {
                    *freq_count.offset(
                        *inv_symmetry_map_[j as usize].offset(child_move as isize) as isize
                    ) += (tree[child_]).frequency;
                    child_move = (tree[child_]).sibling_move as i32;
                    child = (tree[child_]).sibling_node
                }
                break ;
            } else { i += 1 }
        }
    } else if depth < moves_played {
        let mut child = (tree[node]).child_node;
        while let Some(child_) = child {
            recursive_frequency_count(tree, child_, freq_count,
                                      depth + 1, moves_played,
                                      symmetries, primary_hash_0,
                                      secondary_hash_0, inv_symmetry_map_);
            child = (tree[child_]).sibling_node
        }
    };
}

/*
  GET_THOR_GAME
  Returns all available information about the INDEXth game
  in the list of matching games generated by DATABASE_SEARCH.
*/

unsafe fn get_thor_game(index: i32) -> GameInfoType {
    let mut info = GameInfoType {
        black_name: b"",
        white_name: b"",
        tournament: b"",
        year: 0,
        black_actual_score: 0,
        black_corrected_score: 0,
    };
    if index < 0 || index >= thor_search.match_count {
        /* Bad index, so fill with empty values */
        info.black_name = b"";
        info.white_name = b"";
        info.tournament = b"";
        info.year = 0;
        info.black_actual_score = 32;
        info.black_corrected_score = 32
    } else {
        /* Copy name fields etc */
        let game = *thor_search.match_list.offset(index as isize);
        info.black_name = players.get_player_name((*game).black_no as i32);
        info.white_name = players.get_player_name((*game).white_no as i32);
        info.tournament = tournaments.tournament_name((*game).tournament_no as i32);
        info.year = (*game).origin_year;
        info.black_actual_score = (*game).actual_black_score as i32;
        info.black_corrected_score = (*game).perfect_black_score as i32
    }
    return info;
}

/*
  GET_TOTAL_GAME_COUNT
*/

pub unsafe fn get_total_game_count() -> i32 {
    return thor_game_count;
}

/*
  INIT_MOVE_MASKS
  Initializes the shape bit masks for each of the possible moves.
*/
const fn init_move_masks() -> [[u32; 100]; 4] {
    let mut move_mask_hi_ = [0; 100];
    let mut move_mask_lo_ = [0; 100];
    let mut unmove_mask_hi_ = [0; 100];
    let mut unmove_mask_lo_ = [0; 100];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut index: i32 = 0;
    i = 0;
    while i < 4 {
        j = 0;
        pos = 10 * i + 11;
        while j < 8 {
            index = 8 * i + j;
            move_mask_lo_[pos as usize] = ((1) << index) as u32;
            move_mask_hi_[pos as usize] = 0;
            unmove_mask_lo_[pos as usize] = !((1) << index) as u32;
            unmove_mask_hi_[pos as usize] = !(0) as u32;
            j += 1;
            pos += 1
        }
        i += 1
    }
    i = 0;
    while i < 4 {
        j = 0;
        pos = 10 * i + 51;
        while j < 8 {
            index = 8 * i + j;
            move_mask_lo_[pos as usize] = 0;
            move_mask_hi_[pos as usize] = ((1) << index) as u32;
            unmove_mask_lo_[pos as usize] = !(0) as u32;
            unmove_mask_hi_[pos as usize] = !((1) << index) as u32;
            j += 1;
            pos += 1
        }
        i += 1
    };
    [move_mask_hi_, move_mask_lo_, unmove_mask_hi_, unmove_mask_lo_]
}
/*
  CALCULATE_OPENING_FREQUENCY
  Calculates and returns the number of lines in the Thor opening base
  that match the line defined by NODE.
*/
fn calculate_opening_frequency(tree: &mut ThorOpeningTree, node: OpeningNodeRef) -> i32 {
    if let Some(mut child) = tree[node].child_node {
        let mut sum = 0;
        loop {
            sum += calculate_opening_frequency(tree, child);
            if let Some(new_child) = tree[child].sibling_node {
                child = new_child
            } else {
                break;
            }
        }
        tree[node].frequency = sum;
        sum
    } else {
        tree[node].frequency
    }
}

/*
  GET_THOR_GAME_SIZE
  Returns the amount of memory which each game in the database takes.
*/

pub fn get_thor_game_size() -> i32 {
    return ::core::mem::size_of::<GameType>() as u64 as i32;
}

const fn create_symetry_maps() -> SymmetryMaps {
    let mut res = SymmetryMaps {
        b1_b1_map: [0; 100],
        g1_b1_map: [0; 100],
        g8_b1_map: [0; 100],
        b8_b1_map: [0; 100],
        a2_b1_map: [0; 100],
        a7_b1_map: [0; 100],
        h7_b1_map: [0; 100],
        h2_b1_map: [0; 100]
    };
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    while i <= 8 {
        j = 1;
        while j <= 8 {
            pos = 10 * i + j;
            res.b1_b1_map[pos as usize] = pos;
            res.g1_b1_map[pos as usize] = 10 * i + (9 - j);
            res.g8_b1_map[pos as usize] = 10 * (9 - i) +                 (9 - j);
            res.b8_b1_map[pos as usize] = 10 * (9 - i) + j;
            res.a2_b1_map[pos as usize] = 10 * j + i;
            res.a7_b1_map[pos as usize] = 10 * j + (9 - i);
            res.h7_b1_map[pos as usize] = 10 * (9 - j) +                 (9 - i);
            res.h2_b1_map[pos as usize] = 10 * (9 - j) + i;
            j += 1
        }
        i += 1
    }
    res
}
/*
  INIT_SYMMETRY_MAPS
  Initializes the mappings which the 8 elements in the board
  symmetry group induce (and their inverses).
  Note: The order of the mappings must coincide with the order
        in which they are calculated in COMPUTE_FULL_PRIMARY_HASH()
    and COMPUTE_FULL_SECONDARY_HASH().
*/
unsafe fn init_symmetry_maps() {
    symmetry_map[0] = &SYMMENTRY_MAPS.b1_b1_map;
    inv_symmetry_map[0] = &SYMMENTRY_MAPS.b1_b1_map;
    symmetry_map[1] = &SYMMENTRY_MAPS.b8_b1_map;
    inv_symmetry_map[1] = &SYMMENTRY_MAPS.b8_b1_map;
    symmetry_map[2] = &SYMMENTRY_MAPS.a2_b1_map;
    inv_symmetry_map[2] = &SYMMENTRY_MAPS.a2_b1_map;
    symmetry_map[3] = &SYMMENTRY_MAPS.h2_b1_map;
    inv_symmetry_map[3] = &SYMMENTRY_MAPS.a7_b1_map;
    symmetry_map[4] = &SYMMENTRY_MAPS.g1_b1_map;
    inv_symmetry_map[4] = &SYMMENTRY_MAPS.g1_b1_map;
    symmetry_map[5] = &SYMMENTRY_MAPS.g8_b1_map;
    inv_symmetry_map[5] = &SYMMENTRY_MAPS.g8_b1_map;
    symmetry_map[6] = &SYMMENTRY_MAPS.a7_b1_map;
    inv_symmetry_map[6] = &SYMMENTRY_MAPS.h2_b1_map;
    symmetry_map[7] = &SYMMENTRY_MAPS.h7_b1_map;
    inv_symmetry_map[7] = &SYMMENTRY_MAPS.h7_b1_map;
    let mut i = 0;
    while i < 8 {
        let mut j = 1;
        while j <= 8 {
            let mut k = 1;
            while k <= 8 {
                let pos = 10 * j + k;
                if *inv_symmetry_map[i as usize].offset(*symmetry_map[i as usize].offset(pos as isize) as isize) != pos {
                    let to_report = *inv_symmetry_map[i as usize].offset(*symmetry_map[i as usize].offset(pos as isize) as isize);
                    LibcFatalError::error_in_map_thor(i, pos, to_report);
                }
                k += 1
            }
            j += 1
        }
        i += 1
    };
}
impl ThorBoard {
/*
  PLAY_THROUGH_GAME
  Play the MAX_MOVES first moves of GAME and update THOR_BOARD
  and THOR_SIDE_TO_MOVE to represent the position after those moves.
*/
fn play_through_game(&mut self, game: &mut GameType, max_moves: i32) -> i32 {
    let mut move_0: i32 = 0;
    let mut flipped: i32 = 0;
    clear_thor_board(&mut self.board);
    self.side_to_move = 0;
    let mut i = 0;
    while i < max_moves {
        move_0 = abs((*game).moves[i as usize] as i32);
        flipped = any_flips(move_0, self.side_to_move, 0 + 2 - self.side_to_move, &mut self.board);
        if flipped != 0 {
            self.board[move_0 as usize] = self.side_to_move;
            self.side_to_move = 0 + 2 - self.side_to_move
        } else {
            self.side_to_move = 0 + 2 - self.side_to_move;
            flipped = any_flips(move_0, self.side_to_move, 0 + 2 - self.side_to_move, &mut self.board);
            if flipped != 0 {
                self.board[move_0 as usize] = self.side_to_move;
                self.side_to_move = 0 + 2 - self.side_to_move
            } else {
                return 0
            }
        }
        i += 1
    }
    return 1;
}
}
/*
  PREPARE_GAME
  Performs off-line analysis of GAME to speed up subsequent requests.
  The main result is that the number of black discs on the board after
  each of the moves is stored.
*/
fn prepare_game(mut game: &mut GameType, thor_board: &mut ThorBoard, tree: &mut ThorOpeningTree) {
    let mut i: i32 = 0;
    let mut move_0: i32 = 0;
    let mut done: i32 = 0;
    let mut flipped: i32 = 0;
    let mut opening_match: i32 = 0;
    let mut moves_played: i32 = 0;
    let mut disc_count: [i32; 3] = [0; 3];
    let mut corner_descriptor: u32 = 0;
    /* Play through the game and count the number of black discs
       at each stage. */
    clear_thor_board(&mut thor_board.board);
    disc_count[2] = 2;
    disc_count[0] = disc_count[2];
    thor_board.side_to_move = 0;
    corner_descriptor = 0;
    moves_played = 0;
    done = 0;
    loop  {
        /* Store the number of black discs. */
        (*game).black_disc_count[moves_played as usize] = disc_count[0] as i8;
        /* Make the move, update the board and disc count,
           and change the sign for white moves */
        move_0 = (*game).moves[moves_played as usize] as i32;
        flipped = count_flips(move_0, thor_board.side_to_move, 0 + 2 - thor_board.side_to_move, &mut thor_board.board);
        if flipped != 0 {
            thor_board.board[move_0 as usize] = thor_board.side_to_move;
            disc_count[thor_board.side_to_move as usize] += flipped + 1;
            disc_count[(0 + 2 - thor_board.side_to_move) as usize] -= flipped;
            if thor_board.side_to_move == 2 {
                (*game).moves[moves_played as usize] = -((*game).moves[moves_played as usize] as i32) as i8
            }
            thor_board.side_to_move = 0 + 2 - thor_board.side_to_move;
            moves_played += 1
        } else {
            thor_board.side_to_move = 0 + 2 - thor_board.side_to_move;
            flipped = count_flips(move_0, thor_board.side_to_move, 0 + 2 - thor_board.side_to_move, &mut thor_board.board);
            if flipped != 0 {
                thor_board.board[move_0 as usize] = thor_board.side_to_move;
                disc_count[thor_board.side_to_move as usize] += flipped + 1;
                disc_count[(0 + 2 - thor_board.side_to_move) as usize] -= flipped;
                if thor_board.side_to_move == 2 {
                    (*game).moves[moves_played as usize] = -((*game).moves[moves_played as usize] as i32) as i8
                }
                thor_board.side_to_move = 0 + 2 - thor_board.side_to_move;
                moves_played += 1
            } else {
                done = 1
            }
        }
        /* Update the corner descriptor if necessary */
        if move_0 == 11 || move_0 == 18 || move_0 == 81 || move_0 == 88 {
            corner_descriptor |= get_corner_mask(thor_board.board[11], thor_board.board[81], thor_board.board[18], thor_board.board[88])
        }
        if !(done == 0 && moves_played < 60) {
            break;
        }
    }
    (*game).black_disc_count[moves_played as usize] = disc_count[0] as i8;
    (*game).move_count = moves_played as i16;
    i = moves_played + 1;
    while i <= 60 {
        (*game).black_disc_count[i as usize] = -1;
        i += 1
    }
    /* Find the longest opening which coincides with the game */
    let mut opening = tree.root().unwrap();
    let mut child = None;

    i = 0;
    opening_match = 1;
    while opening_match != 0 {
        move_0 = (tree[opening]).child_move as i32;
        child = (tree[opening]).child_node;
        while let Some(child_) = child {
            if move_0 == abs((*game).moves[i as usize] as i32) {
                break;
            }
            move_0 = (tree[child_]).sibling_move as i32;
            child = (tree[child_]).sibling_node
        }
        if let Some(child) = child {
            opening = child;
            i += 1
        } else {
            opening_match = 0
        }
    }
    (*game).opening = opening;
    /* Initialize the shape state */
    (*game).shape_lo = 3 << 27;
    (*game).shape_hi = 3 << 3;
    (*game).shape_state_hi = 0;
    (*game).shape_state_lo = 0;
    /* Store the corner descriptor */
    (*game).corner_descriptor = corner_descriptor;
}

impl ThorHash {
    /*
      INIT_THOR_HASH
      Computes hash codes for each of the 6561 configurations of the 8 different
      rows. A special feature of the codes is the relation

         hash[flip[pattern]] == reverse[hash[pattern]]

      which speeds up the computation of the hash functions.
    */
    fn init_thor_hash(&mut self, random_instance: &mut MyRandom) {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut row: [i32; 10] = [0; 10];
        let mut flip_row: [i32; 6561] = [0; 6561];
        let mut buffer: [i32; 6561] = [0; 6561];
        while i < 8 {
            row[i as usize] = 0;
            i += 1
        }
        i = 0;
        while i < 6561 {
            flip_row[i as usize] = 0;
            j = 0;
            while j < 8 {
                flip_row[i as usize] += row[j as usize] * pow3((7 - j) as usize);
                j += 1
            }
            /* Next configuration */
            odometer_principle(&mut row, 8);
            i += 1
        }
        i = 0;
        while i < 8 {
            j = 0;
            while j < 6561 {
                buffer[j as usize] = abs(random_instance.my_random() as i32);
                j += 1
            }
            j = 0;
            while j < 6561 {
                self.primary_hash[i as usize][j as usize] =
                    buffer[j as usize] as u32 &
                        0xffff0000 as u32 |
                        bit_reverse_32(buffer[flip_row[j as usize] as usize] as
                            u32) &
                            0xffff as i32 as u32;
                j += 1
            }
            j = 0;
            while j < 6561 {
                buffer[j as usize] = abs(random_instance.my_random() as i32);
                j += 1
            }
            j = 0;
            while j < 6561 {
                self.secondary_hash[i as usize][j as usize] =
                    buffer[j as usize] as u32 &
                        0xffff0000 as u32 |
                        bit_reverse_32(buffer[flip_row[j as usize] as usize] as
                            u32) &
                            0xffff as i32 as u32;
                j += 1
            }
            i += 1
        };
    }
}
/*
  NEW_THOR_OPENING_NODE
  Creates and initializes a new node for use in the opening tree.
*/
fn new_thor_opening_node(tree: &mut ThorOpeningTree, parent: Option<OpeningNodeRef>) -> OpeningNodeRef {
    let mut node = ThorOpeningNode::new();
    node.child_move = 0;
    node.sibling_move = 0;
    node.child_node = None;
    node.sibling_node = None;
    node.parent_node = parent;
    tree.add(node)
}

/*
  BUILD_THOR_OPENING_TREE
  Builds the opening tree from the statically computed
  structure THOR_OPENING_LIST (see thorop.c).
*/
fn build_thor_opening_tree(thor_board: &mut ThorBoard, thor_hash_: &mut ThorHash, tree: &mut ThorOpeningTree) {
    type FE = LibcFatalError;
    let mut thor_move_list: [i8; 61] = [0; 61];
    let mut j: i32 = 0;
    let mut move_0: i32 = 0;
    let mut branch_depth: i32 = 0;
    let mut end_depth: i32 = 0;
    let mut flipped: i32 = 0;
    let mut hash1: u32 = 0;
    let mut hash2: u32 = 0;
    // let mut parent = None;
    // let mut last_child = None;
    // let mut new_child = None;
    let mut node_list: [Option<OpeningNodeRef>; 61] = [None; 61];
    /* Create the root node and compute its hash value */
    let mut root_node = new_thor_opening_node(tree, None);
    tree.set_root(root_node);

    clear_thor_board(&mut thor_board.board);
    compute_thor_patterns(&mut thor_hash_.thor_row_pattern, &mut thor_hash_.thor_col_pattern, &thor_board.board);
    thor_hash_.compute_partial_hash(&mut hash1, &mut hash2);
    (tree[root_node]).hash1 = hash1;
    (tree[root_node]).hash2 = hash2;
    node_list[0] = Some(root_node);
    /* Add each of the openings to the tree */
    let mut i = 0;
    while i < 741 {
        branch_depth = THOR_OPENING_LIST[i as usize].first_unique;
        end_depth =
            (branch_depth as u64).wrapping_add(
                (THOR_OPENING_LIST[i as usize].move_str.len() as u64).wrapping_div(2)) as i32;
        j = 0;
        while j < end_depth - branch_depth {
            thor_move_list[(branch_depth + j) as usize] = (10 *
                (*THOR_OPENING_LIST[i as usize].move_str.offset((2 * j + 1) as isize) as i32 - '0' as i32) +
                (*THOR_OPENING_LIST[i as usize].move_str.offset((2 * j) as isize) as i32 - 'a' as i32 + 1)
            ) as i8;
            j += 1
        }
        /* Play through the moves common with the previous line
           and the first deviation */
        clear_thor_board(&mut thor_board.board);
        thor_board.side_to_move = 0;
        j = 0;
        while j <= branch_depth {
            move_0 = thor_move_list[j as usize] as i32;
            flipped = any_flips(move_0, thor_board.side_to_move, 0 + 2 - thor_board.side_to_move, &mut thor_board.board);
            if flipped != 0 {
                thor_board.board[move_0 as usize] = thor_board.side_to_move;
                thor_board.side_to_move = 0 + 2 - thor_board.side_to_move
            } else {
                thor_board.side_to_move = 0 + 2 - thor_board.side_to_move;
                flipped = any_flips(move_0, thor_board.side_to_move, 0 + 2 - thor_board.side_to_move, &mut thor_board.board);
                if flipped != 0 {
                    thor_board.board[move_0 as usize] = thor_board.side_to_move;
                    thor_board.side_to_move = 0 + 2 - thor_board.side_to_move
                } else {
                    FE::thordb_report_flipped_0_first();
                }
            }
            j += 1
        }
        /* Create the branch from the previous node */
        let mut parent = node_list[branch_depth as usize].unwrap();
        let mut new_child = new_thor_opening_node(tree, Some(parent));
        compute_thor_patterns(&mut thor_hash_.thor_row_pattern, &mut thor_hash_.thor_col_pattern, &thor_board.board);
        thor_hash_.compute_partial_hash(&mut hash1, &mut hash2);
        (tree[new_child]).hash1 = hash1;
        (tree[new_child]).hash2 = hash2;
        if (tree[parent]).child_node.is_none() {
            (tree[parent]).child_node = Some(new_child);
            (tree[parent]).child_move = thor_move_list[branch_depth as usize]
        } else {
            let mut last_child = (tree[parent]).child_node.unwrap();
            while let Some(next_child) = (tree[last_child]).sibling_node {
                last_child = next_child
            }
            (tree[last_child]).sibling_node = Some(new_child);
            (tree[last_child]).sibling_move = thor_move_list[branch_depth as usize]
        }
        node_list[(branch_depth + 1) as usize] = Some(new_child);
        /* Play through the rest of the moves and create new nodes for each
           of the resulting positions */
        j = branch_depth + 1;
        while j < end_depth {
            move_0 = thor_move_list[j as usize] as i32;
            flipped = any_flips(move_0, thor_board.side_to_move, 0 + 2 - thor_board.side_to_move, &mut thor_board.board);
            if flipped != 0 {
                thor_board.board[move_0 as usize] = thor_board.side_to_move;
                thor_board.side_to_move = 0 + 2 - thor_board.side_to_move
            } else {
                thor_board.side_to_move = 0 + 2 - thor_board.side_to_move;
                flipped = any_flips(move_0, thor_board.side_to_move, 0 + 2 - thor_board.side_to_move, &mut thor_board.board);
                if flipped != 0 {
                    thor_board.board[move_0 as usize] = thor_board.side_to_move;
                    thor_board.side_to_move = 0 + 2 - thor_board.side_to_move
                } else {
                    FE::thordb_report_flipped_0_second();
                }
            }
            parent = new_child;
            new_child = new_thor_opening_node(tree, Some(parent));
            compute_thor_patterns(&mut thor_hash_.thor_row_pattern, &mut thor_hash_.thor_col_pattern, &thor_board.board);
            thor_hash_.compute_partial_hash(&mut hash1, &mut hash2);
            tree[new_child].hash1 = hash1;
            tree[new_child].hash2 = hash2;
            tree[parent].child_node = Some(new_child);
            tree[parent].child_move = thor_move_list[j as usize];
            node_list[(j + 1) as usize] = Some(new_child);
            j += 1
        }
        (tree[new_child]).frequency = THOR_OPENING_LIST[i as usize].frequency;
        i += 1
    }
    /* Calculate opening frequencies also for interior nodes */
    calculate_opening_frequency(tree, root_node);
}


/*
  INIT_THOR_DATABASE
  Performs the basic initializations of the Thor database interface.
  Before any operation on the database may be performed, this function
  must be called.
*/

pub unsafe fn init_thor_database(g_state: &mut FullState) {
    thor_game_count = 0;
    thor_search.match_list = Vec::new();
    thor_search.allocation = 0;
    thor_search.match_count = 0;
    thor_search.black_wins = 0;
    thor_search.draws = 0;
    thor_search.white_wins = 0;
    thor_search.median_black_score = 0;
    thor_search.average_black_score = 0.0f64;
    thor_sort_criteria_count = 5;
    let mut i: i32 = 0; /* "infinity" */
    while i < 5 {
        thor_sort_order[i as usize] = default_sort_order[i as usize];
        i += 1
    }
    database_head = None;
    players.name_buffer = b"";
    players.player_list = Vec::new();
    tournaments.tournament_list.clear();
    tournaments.name_buffer = b"";
    thor_games_sorted = 0;
    thor_games_filtered = 0;
    init_move_masks();
    init_symmetry_maps();
    thor_hash.init_thor_hash(&mut g_state.random_instance);
    prepare_thor_board(&mut board.board);
    build_thor_opening_tree(&mut board, &mut thor_hash, &mut thor_opening_tree);
    filter.game_categories = 1 | 2 | 4;
    filter.player_filter = EITHER_SELECTED_FILTER;
    filter.first_year = -((1) << 25);
    filter.last_year = (1) << 25;
}

/*
  GET_THOR_GAME_MOVES
  Returns the moves, and number of moves, in the INDEXth game
  in the list of matching games generated by DATABASE_SEARCH.
  The game will not necessarily have the same rotational symmetry
  as the position searched for with database_search(); this depends
  on what rotation that gave a match.
*/

unsafe fn get_thor_game_moves(index: i32, move_count: &mut i32, moves: &mut [i32]) {
    let mut game = 0 as *mut GameType;
    if index < 0 || index >= thor_search.match_count {
        /* Bad index, so fill with empty values */
        *move_count = 0
    } else {
        game = *thor_search.match_list.offset(index as isize);
        *move_count = (*game).move_count as i32;
        match (*game).matching_symmetry as i32 {
            0 | 2 | 5 | 7 => {
                /* Symmetries that preserve the initial position. */
                let mut i = 0;
                while i < (*game).move_count as i32 {
                    *moves.offset(i as isize) = *symmetry_map[(*game).matching_symmetry as usize].offset(abs((*game).moves[i as usize] as i32) as isize);
                    i += 1
                }
            }
            _ => {
                /* Symmetries that reverse the initial position. */
                let mut i = 0;
                while i < (*game).move_count as i32 {
                    *moves.offset(i as isize) = abs((*game).moves[i as usize] as i32);
                    i += 1
                }
            }
        }
    };
}
/*
  GET_THOR_GAME_MOVE
  Returns the MOVE_NUMBERth move in the INDEXth game
  in the list of matching games generated by DATABASE_SEARCH.
*/

pub unsafe fn get_thor_game_move(index: i32, move_number: i32) -> i32 {
    if index < 0 || index >= thor_search.match_count {
        -1
    } else {
        let game = *thor_search.match_list.offset(index as isize);
        if move_number < 0 ||
            move_number >= (*game).move_count as i32 {
            -1
        } else {
            *symmetry_map[(*game).matching_symmetry as usize].offset(abs((*game).moves[move_number as usize] as i32) as isize)
        }
    }
}

/*
  POSITION_MATCH
  Returns TRUE if the position after MOVE_COUNT moves of GAME, with
  SIDE_TO_MOVE being the player to move, matches the hash codes
  IN_HASH1 and IN_HASH2, otherwise FALSE.
*/
fn position_match(mut game: &mut GameType,
                         thor_board: &mut ThorBoard,
                         thor_hash_: &mut ThorHash,
                         tree: &mut ThorOpeningTree,
                         move_count: i32,
                         side_to_move: i32,
                         shape_lo: &mut [u32],
                         shape_hi: &mut [u32],
                         corner_mask: u32,
                         in_hash1: u32,
                         in_hash2: u32)
                         -> i32 {
    let mut i: i32 = 0;
    let mut shape_match: i32 = 0;
    let mut primary_hit_mask: i32 = 0;
    let mut secondary_hit_mask: i32 = 0;
    /* Verify that the number of moves and the side-to-move status
       are correct */
    if move_count >= (*game).move_count as i32 {
        if move_count > (*game).move_count as i32 {
            /* Too many moves! */
            return 0
        }
        /* No side-to-move status to check if the game is over */
    } else if side_to_move == 0 {
        /* Black to move */
        if ((*game).moves[move_count as usize] as i32) < 0 {
            /* White to move in the game */
            return 0
        }
    } else if (*game).moves[move_count as usize] as i32 > 0 {
        /* White to move */
        /* Black to move in the game */
        return 0
    }
    /* Check if the opening information suffices to
       determine if the position matches or not. */
    if (tree[game.opening]).current_match == 1 {
        (*game).matching_symmetry = (tree[game.opening]).matching_symmetry as i16;
        return 1
    } else {
        if (tree[game.opening]).current_match == 2 {
            return 0
        }
    }
    /* Check if the lower 32 bits of the shape state coincide */
    if ((*game).shape_state_lo as i32) < move_count {
        i = (*game).shape_state_lo as i32;
        while i < move_count {
            (*game).shape_lo |= move_mask_lo[abs((*game).moves[i as usize] as i32) as usize];
            i += 1
        }
        (*game).shape_state_lo = move_count as i16
    } else if (*game).shape_state_lo as i32 > move_count {
        i = (*game).shape_state_lo as i32 - 1;
        while i >= move_count {
            (*game).shape_lo &= !move_mask_lo[abs((*game).moves[i as usize] as i32) as usize];
            i -= 1
        }
        (*game).shape_state_lo = move_count as i16
    }
    shape_match = 0;
    i = 0;
    while i < 8 {
        shape_match |= ((*game).shape_lo == *shape_lo.offset(i as isize)) as i32;
        i += 1
    }
    if shape_match == 0 {
        return 0
    }
    /* Check if the upper 32 bits of the shape state coincide */
    if ((*game).shape_state_hi as i32) < move_count {
        i = (*game).shape_state_hi as i32;
        while i < move_count {
            (*game).shape_hi |= move_mask_hi[abs((*game).moves[i as usize] as i32) as usize];
            i += 1
        }
        (*game).shape_state_hi = move_count as i16
    } else if (*game).shape_state_hi as i32 > move_count {
        i = (*game).shape_state_hi as i32 - 1;
        while i >= move_count {
            (*game).shape_hi &= !move_mask_hi[abs((*game).moves[i as usize] as i32) as usize];
            i -= 1
        }
        (*game).shape_state_hi = move_count as i16
    }
    shape_match = 0;
    i = 0;
    while i < 8 {
        shape_match |= ((*game).shape_hi == *shape_hi.offset(i as isize)) as i32;
        i += 1
    }
    if shape_match == 0 {
        return 0
    }
    /* Check if the corner mask is compatible with that of the game */
    if corner_mask & !(*game).corner_descriptor != 0 {
        return 0
    }
    /* Otherwise play through the moves of the game until the
       number of discs is correct and check if the hash
       functions match the given hash values for at least one
       rotation (common to the two hash functions). */
    if thor_board.play_through_game(game, move_count) != 0 {
        compute_thor_patterns(&mut thor_hash_.thor_row_pattern, &mut thor_hash_.thor_col_pattern, &thor_board.board);
        primary_hit_mask = thor_hash_.primary_hash_lookup(in_hash1);
        if primary_hit_mask != 0 {
            secondary_hit_mask = thor_hash_.secondary_hash_lookup(in_hash2);
            if primary_hit_mask & secondary_hit_mask != 0 {
                i = 0;
                while i < 8 {
                    if primary_hit_mask & secondary_hit_mask & (1) << i != 0 {
                        (*game).matching_symmetry = i as i16;
                        return 1
                    }
                    i += 1
                }
            }
        }
    }
    return 0;
}

/*
  THOR_COMPARE
  Compares two games from a list of pointers to games.
  Only to be called by QSORT. A full comparison is
  performed using the priority order from THOR_SORT_ORDER.
*/
pub fn thor_compare(game1: &GameType, game2: &GameType, sord_order: &[i32], players_: &PlayerDatabaseType, tournaments_: &TournamentDatabaseType) -> i32 {
    for sort_order_item in sord_order {
        let result = match sort_order_item {
            1 => game2.origin_year - game1.origin_year,
            2 => players_.player_lex_order((*game1).black_no as i32) - players_.player_lex_order((*game2).black_no as i32),
            3 => players_.player_lex_order((*game1).white_no as i32) - players_.player_lex_order((*game2).white_no as i32),
            4 => tournaments_.tournament_lex_order((*game1).tournament_no as i32) - tournaments_.tournament_lex_order((*game2).tournament_no as i32),
            5 => (*game1).actual_black_score as i32 - (*game2).actual_black_score as i32,
            6 => (*game2).actual_black_score as i32 - (*game1).actual_black_score as i32,

            /* Really can't happen */
            0 | _ => game1.origin_year - game2.origin_year,
        };
        if result != 0 {
            return result
        }
    }
    /* If control reaches this point, the two games couldn't be
       distinguished by the current search criteria. */
    return 0;
}

/*
  CHOOSE_THOR_OPENING_MOVE
  Computes frequencies for all moves from the given position,
  display these and chooses one if from a distribution skewed
  towards common moves. (If no moves are found, PASS is returned.)
*/
pub unsafe fn choose_thor_opening_move(in_board: &[i32], side_to_move: i32, echo: i32, random_instance: &mut MyRandom) -> i32 {
    let tree: &mut ThorOpeningTree = &mut thor_opening_tree;
    let mut j: i32 = 0;
    let mut temp_symm: i32 = 0;
    let mut pos: i32 = 0;
    let mut freq_sum: i32 = 0;
    let mut acc_freq_sum: i32 = 0;
    let mut random_move: i32 = 0;
    let mut random_value: i32 = 0;
    let mut match_count: i32 = 0;
    let mut symmetries: [i32; 8] = [0; 8];
    let mut freq_count: [i32; 100] = [0; 100];
    let mut primary_hash_0: [u32; 8] = [0; 8];
    let mut secondary_hash_0: [u32; 8] = [0; 8];
    let mut move_list: [C2RustUnnamed; 64] = [C2RustUnnamed{move_0: 0, frequency: 0,}; 64];
    let mut temp = C2RustUnnamed{move_0: 0, frequency: 0,};
    let mut disc_count = 0;
    let mut i = 1;
    while i <= 8 {
        j = 1;
        pos = 10 * i + 1;
        while j <= 8 {
            freq_count[pos as usize] = 0;
            if *in_board.offset(pos as isize) != 1 {
                disc_count += 1
            }
            j += 1;
            pos += 1
        }
        i += 1
    }
    /* Check that the parity of the board coincides with standard
       Othello - if this is not the case, the Thor opening lines are useless
       as they don't contain any passes. */
    if side_to_move == 0 &&
        disc_count % 2 == 1 {
        return -1
    }
    if side_to_move == 2 &&
        disc_count % 2 == 0 {
        return -1
    }
    /* Create a random permutation of the symmetries to avoid the same
       symmetry always being chosen in e.g. the initial position */
    i = 0;
    while i < 8 { symmetries[i as usize] = i; i += 1 }
    i = 0;
    while i < 7 {
        j = i + abs(random_instance.my_random() as i32) % (8 - i);
        temp_symm = symmetries[i as usize];
        symmetries[i as usize] = symmetries[j as usize];
        symmetries[j as usize] = temp_symm;
        i += 1
    }
    /* Calculate frequencies for all moves */
    compute_thor_patterns(&mut thor_hash.thor_row_pattern, &mut thor_hash.thor_col_pattern, &in_board);
    thor_hash.compute_full_primary_hash(&mut primary_hash_0);
    thor_hash.compute_full_secondary_hash(&mut secondary_hash_0);
    recursive_frequency_count(tree, tree.root().unwrap(), &mut freq_count, 0, disc_count - 4,
                              &mut symmetries, &mut primary_hash_0, &mut secondary_hash_0, &inv_symmetry_map);
    freq_sum = 0;
    i = 1;
    while i <= 8 {
        j = 1;
        pos = 10 * i + 1;
        while j <= 8 {
            freq_sum += freq_count[pos as usize];
            j += 1;
            pos += 1
        }
        i += 1
    }
    if freq_sum > 0 {
        /* Position found in Thor opening tree */
        /* Create a list of the moves chosen from the position and also
           randomly select one of them. Probability for each move is
           proportional to the frequency of that move being played here. */
        random_value = abs(random_instance.my_random() as i32) % freq_sum;
        random_move = -1;
        acc_freq_sum = 0;
        match_count = 0;
        i = 1;
        while i <= 8 {
            j = 1;
            pos = 10 * i + 1;
            while j <= 8 {
                if freq_count[pos as usize] > 0 {
                    move_list[match_count as usize].move_0 = pos;
                    move_list[match_count as usize].frequency = freq_count[pos as usize];
                    match_count += 1;
                    if acc_freq_sum < random_value && acc_freq_sum + freq_count[pos as usize] >= random_value {
                        random_move = pos
                    }
                    acc_freq_sum += freq_count[pos as usize]
                }
                j += 1;
                pos += 1
            }
            i += 1
        }
        /* Optionally display the database moves sorted on frequency */
        if echo != 0 {
            i = 0;
            while i < match_count {
                j = 0;
                while j < match_count - 1 {
                    if move_list[j as usize].frequency < move_list[(j + 1) as usize].frequency {
                        temp = move_list[j as usize];
                        move_list[j as usize] = move_list[(j + 1) as usize];
                        move_list[(j + 1) as usize] = temp
                    }
                    j += 1
                }
                i += 1
            }
            LibcFatalError::choose_thor_opening_move_report(freq_sum, match_count, &move_list)
        }
        return random_move
    }
    return -1;
}

/*
  DATABASE_SEARCH
  Determines what positions in the Thor database match the position
  given by IN_BOARD with SIDE_TO_MOVE being the player whose turn it is.
*/

pub unsafe fn database_search(in_board: &[i32], side_to_move: i32) {
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
    let mut game;
    /* We need a player and a tournament database. */
    if players.count() == 0 ||
        tournaments.count() == 0 {
        thor_search.match_count = 0;
        return
    }
    /* Make sure there's memory allocated if all positions
       in all databases match the position */
    if thor_search.allocation == 0 {
        thor_search.match_list = vec![null_mut(); thor_game_count as usize];
        thor_search.allocation = thor_game_count
    } else if thor_search.allocation < thor_game_count {
        thor_search.match_list = vec![null_mut(); thor_game_count as usize];
        thor_search.allocation = thor_game_count
    }
    /* If necessary, filter all games in the database */
    if thor_games_filtered == 0 {
        filter_all_databases();
        thor_games_filtered = 1
    }
    /* If necessary, sort all games in the database */
    if thor_games_sorted == 0 {
        let mut current_db_ = &mut database_head;
        i = 0;
        while let Some(current_db) = current_db_ {
            j = 0;
            while j < (*current_db).count {
                let ref mut fresh5 = *thor_search.match_list.offset(i as isize);
                *fresh5 = &mut *(*current_db).games.offset(j as isize) as *mut GameType;
                i += 1;
                j += 1
            }
            current_db_ = &mut (*current_db).next
        }
        sort_thor_games(thor_game_count);
        j = 0;
        while j < thor_game_count {
            (**thor_search.match_list.offset(j as isize)).sort_order = j;
            j += 1
        }
        thor_games_sorted = 1
    }
    /* Determine disc count, hash codes, patterns and opening
       for the position */
    disc_count[2] = 0;
    disc_count[0] = disc_count[2];
    i = 1;
    while i <= 8 {
        j = 1;
        pos = 10 * i + 1;
        while j <= 8 {
            if *in_board.offset(pos as isize) == 0 {
                disc_count[0] += 1
            } else if *in_board.offset(pos as isize) == 2 {
                disc_count[2] += 1
            }
            j += 1;
            pos += 1
        }
        i += 1
    }
    move_count = disc_count[0] + disc_count[2] - 4;
    compute_thor_patterns(&mut thor_hash.thor_row_pattern, &mut thor_hash.thor_col_pattern, in_board);
    thor_hash.compute_partial_hash(&mut target_hash1, &mut target_hash2);
    opening_scan(move_count, (&mut thor_hash), &mut thor_opening_tree);
    /* Determine the shape masks */
    i = 0;
    while i < 8 {
        shape_lo[i as usize] = 0;
        shape_hi[i as usize] = 0;
        i += 1
    }
    i = 0;
    while i < 8 {
        j = 0;
        pos = 10 * i + 11;
        while j < 8 {
            if *in_board.offset(pos as isize) != 1 {
                index = 8 * i + j;
                if index < 32 {
                    shape_lo[0] |= ((1) << index) as u32
                } else {
                    shape_hi[0] |= ((1) << index - 32) as u32
                }
                index = 8 * i + (7 - j);
                if index < 32 {
                    shape_lo[1] |= ((1) << index) as u32
                } else {
                    shape_hi[1] |= ((1) << index - 32) as u32
                }
                index = 8 * j + i;
                if index < 32 {
                    shape_lo[2] |= ((1) << index) as u32
                } else {
                    shape_hi[2] |= ((1) << index - 32) as u32
                }
                index = 8 * j + (7 - i);
                if index < 32 {
                    shape_lo[3] |= ((1) << index) as u32
                } else {
                    shape_hi[3] |= ((1) << index - 32) as u32
                }
                index = 8 * (7 - i) + j;
                if index < 32 {
                    shape_lo[4] |= ((1) << index) as u32
                } else {
                    shape_hi[4] |= ((1) << index - 32) as u32
                }
                index = 8 * (7 - i) + (7 - j);
                if index < 32 {
                    shape_lo[5] |= ((1) << index) as u32
                } else {
                    shape_hi[5] |= ((1) << index - 32) as u32
                }
                index = 8 * (7 - j) + i;
                if index < 32 {
                    shape_lo[6] |= ((1) << index) as u32
                } else {
                    shape_hi[6] |= ((1) << index - 32) as u32
                }
                index = 8 * (7 - j) + (7 - i);
                if index < 32 {
                    shape_lo[7] |= ((1) << index) as u32
                } else {
                    shape_hi[7] |= ((1) << index - 32) as u32
                }
            }
            j += 1;
            pos += 1
        }
        i += 1
    }
    /* Get the corner mask */
    corner_mask = get_corner_mask(*in_board.offset(11), *in_board.offset(81), *in_board.offset(18), *in_board.offset(88));
    /* Query the database about all positions in all databases.
       Only games which pass the currently applied filter are scanned.
       Also compute the frequency table and the next move table.
       To speed up sorting the games, the match table is first filled
       with NULLs and when a matching game is found, a pointer to it is
       inserted at a position determined by the field SORT_ORDER
       in the game. As this index is unique, no over-write
       can occur. */
    thor_search.match_count = 0;
    i = 0;
    while i < thor_game_count {
        let ref mut fresh6 = *thor_search.match_list.offset(i as isize);
        *fresh6 = 0 as *mut GameType;
        i += 1
    }
    i = 0;
    while i <= 64 {
        frequency[i as usize] = 0;
        i += 1
    }
    i = 0;
    while i < 100 {
        thor_search.next_move_frequency[i as usize] = 0;
        thor_search.next_move_score[i as usize] = 0.0f64;
        i += 1
    }
    let mut current_db_ = &mut database_head;
    while let Some(current_db) = current_db_ {
        i = 0;
        while i < (*current_db).count {
            game = &mut *(*current_db).games.offset(i as isize);
            if (*game).passes_filter != 0 {
                if disc_count[0] == (*game).black_disc_count[move_count as usize] as i32 {
                    if position_match(game, &mut board, &mut thor_hash, &mut thor_opening_tree, move_count, side_to_move, &mut shape_lo, &mut shape_hi, corner_mask, target_hash1, target_hash2) != 0 {
                        let ref mut fresh7 = *thor_search.match_list.offset((*game).sort_order as isize);
                        *fresh7 = game;
                        symmetry = (*game).matching_symmetry as i32;
                        if move_count < (*game).move_count as i32 {
                            next_move = *symmetry_map[symmetry as usize].offset(
                                abs((*game).moves[move_count as usize] as i32) as isize
                            );
                            thor_search.next_move_frequency[next_move as usize] += 1;
                            if (*game).actual_black_score as i32 == 32 {
                                thor_search.next_move_score[next_move as usize] += 0.5f64
                            } else if (*game).actual_black_score as i32 > 32 {
                                if side_to_move == 0 {
                                    thor_search.next_move_score[next_move as usize] += 1.0f64
                                }
                            } else if side_to_move == 2 {
                                thor_search.next_move_score[next_move as usize] += 1.0f64
                            }
                        }
                        frequency[(*game).actual_black_score as usize] += 1;
                        thor_search.match_count += 1
                    }
                }
            }
            i += 1
        }
        current_db_ = &mut (*current_db).next
    }
    /* Remove the NULLs from the list of matching games if there are any.
       This gives a sorted list. */
    if thor_search.match_count > 0 &&
        thor_search.match_count < thor_game_count {
        i = 0;
        j = 0;
        while i < thor_search.match_count {
            if !(*thor_search.match_list.offset(j as isize)).is_null() {
                let ref mut fresh8 = *thor_search.match_list.offset(i as isize);
                *fresh8 = *thor_search.match_list.offset(j as isize);
                i += 1
            }
            j += 1
        }
    }
    /* Count the number of black wins, draws and white wins.
       Also determine the average score. */
    sum = 0;
    i = 0;
    thor_search.white_wins = 0;
    while i <= 31 {
        thor_search.white_wins += frequency[i as usize];
        sum += i * frequency[i as usize];
        i += 1
    }
    thor_search.draws = frequency[32];
    sum += 32 * frequency[32];
    i = 33;
    thor_search.black_wins = 0;
    while i <= 64 {
        thor_search.black_wins += frequency[i as usize];
        sum += i * frequency[i as usize];
        i += 1
    }
    if thor_search.match_count == 0 {
        /* Average of 0 values is pointless */
        thor_search.average_black_score = 32.0f64
    } else {
        thor_search.average_black_score = sum as f64 / thor_search.match_count as f64
    }
    /* Determine the median score */
    if thor_search.match_count == 0 {
        /* ...and so is median of 0 values */
        thor_search.median_black_score = 32
    } else {
        cumulative[0] = frequency[0];
        i = 1;
        while i <= 64 {
            cumulative[i as usize] = cumulative[(i - 1) as usize] + frequency[i as usize];
            i += 1
        }
        /* Median is average between first value for which cumulative
           frequency reaches 50% and first value for which it is
           strictly larger than 50%. This definition works regardless
           of the parity of the number of values.
           By definition of median, both loops terminate for indices <= 64. */
        i = 0;
        while 2 * cumulative[i as usize] < thor_search.match_count {
            i += 1
        }
        j = i;
        while 2 * cumulative[j as usize] < thor_search.match_count + 1 {
            j += 1
        }
        thor_search.median_black_score = (i + j) / 2
    };
}
