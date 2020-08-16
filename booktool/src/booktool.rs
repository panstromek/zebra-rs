#![allow(dead_code,  non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut)]

use c2rust_out::src::osfbook::{write_text_database, write_compressed_database, write_binary_database, generate_endgame_statistics, generate_midgame_statistics, display_doubly_optimal_line, export_tree, restricted_minimax_tree, minimax_tree, merge_position_list, correct_tree, evaluate_tree, merge_binary_database, clear_tree, book_statistics, convert_opening_list, set_output_script_name, unpack_compressed_database, read_text_database, read_binary_database, build_tree, init_osf};
use engine::src::osfbook::{set_draw_mode, set_black_force, set_white_force, set_deviation_value, set_search_depth, set_eval_span, set_negamax_span, set_max_batch_size, set_game_mode};
use engine::src::hash::resize_hash;
use engine::src::error::{LibcFatalError};
pub type FE = LibcFatalError;

extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strtod(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char)
              -> libc::c_double;
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
                  -> libc::c_int;
}
pub type DrawMode = libc::c_uint;
pub const OPPONENT_WINS: DrawMode = 3;
pub const WHITE_WINS: DrawMode = 2;
pub const BLACK_WINS: DrawMode = 1;
pub const NEUTRAL: DrawMode = 0;
pub type GameMode = libc::c_uint;
pub const PUBLIC_GAME: GameMode = 1;
pub const PRIVATE_GAME: GameMode = 0;
pub const MIDGAME_STATISTICS: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const ENDGAME_STATISTICS: C2RustUnnamed = 1;
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int) as libc::c_int;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
                 -> libc::c_int {
    let mut import_file_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut input_file_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut output_file_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut statistics_file_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opening_in_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut position_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opening_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut merge_script_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut merge_output_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut export_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut merge_book_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut statistics_type: C2RustUnnamed = MIDGAME_STATISTICS;
    let mut probability: libc::c_double = 0.;
    let mut bonus: libc::c_double = 0.;
    let mut min_eval_span: libc::c_double = 0.;
    let mut max_eval_span: libc::c_double = 0.;
    let mut min_negamax_span: libc::c_double = 0.;
    let mut max_negamax_span: libc::c_double = 0.;
    let mut error: libc::c_int = 0;
    let mut arg_index: libc::c_int = 0;
    let mut max_game_count: libc::c_int = 0;
    let mut max_diff: libc::c_int = 0;
    let mut cutoff: libc::c_int = 0;
    let mut import_games: libc::c_int = 0;
    let mut input_database: libc::c_int = 0;
    let mut output_database: libc::c_int = 0;
    let mut input_binary: libc::c_int = 0;
    let mut output_binary: libc::c_int = 0;
    let mut output_compressed: libc::c_int = 0;
    let mut calculate_minimax: libc::c_int = 0;
    let mut evaluate_all: libc::c_int = 0;
    let mut display_line: libc::c_int = 0;
    let mut do_statistics: libc::c_int = 0;
    let mut max_depth: libc::c_int = 0;
    let mut low_threshold: libc::c_int = 0;
    let mut high_threshold: libc::c_int = 0;
    let mut complete_statistics: libc::c_int = 0;
    let mut give_help: libc::c_int = 0;
    let mut process_openings: libc::c_int = 0;
    let mut uncompress_database: libc::c_int = 0;
    let mut endgame_correct: libc::c_int = 0;
    let mut max_empty: libc::c_int = 0;
    let mut full_solve: libc::c_int = 0;
    let mut dump_positions: libc::c_int = 0;
    let mut first_stage: libc::c_int = 0;
    let mut last_stage: libc::c_int = 0;
    let mut clear_flags: libc::c_int = 0;
    let mut clear_low: libc::c_int = 0;
    let mut clear_high: libc::c_int = 0;
    init_osf(1 as libc::c_int);
    cutoff = 16 as libc::c_int;
    max_diff = 24 as libc::c_int;
    max_game_count = 0 as libc::c_int;
    import_games = 0 as libc::c_int;
    import_file_name = 0 as *mut libc::c_char;
    input_database = 0 as libc::c_int;
    input_file_name = 0 as *mut libc::c_char;
    input_binary = 0 as libc::c_int;
    output_database = 0 as libc::c_int;
    output_file_name = 0 as *mut libc::c_char;
    output_binary = 1 as libc::c_int;
    output_compressed = 0 as libc::c_int;
    uncompress_database = 0 as libc::c_int;
    calculate_minimax = 0 as libc::c_int;
    low_threshold = 60 as libc::c_int;
    high_threshold = 60 as libc::c_int;
    bonus = 0.0f64;
    evaluate_all = 0 as libc::c_int;
    display_line = 0 as libc::c_int;
    do_statistics = 0 as libc::c_int;
    statistics_file_name = 0 as *mut libc::c_char;
    probability = 0.0f64;
    max_depth = 0 as libc::c_int;
    complete_statistics = 0 as libc::c_int;
    statistics_type = MIDGAME_STATISTICS;
    give_help = 0 as libc::c_int;
    endgame_correct = 0 as libc::c_int;
    max_empty = 0 as libc::c_int;
    full_solve = 0 as libc::c_int;
    process_openings = 0 as libc::c_int;
    opening_in_file = 0 as *mut libc::c_char;
    dump_positions = 0 as libc::c_int;
    position_file = 0 as *mut libc::c_char;
    opening_file = 0 as *mut libc::c_char;
    merge_script_file = 0 as *mut libc::c_char;
    merge_output_file = 0 as *mut libc::c_char;
    export_file = 0 as *mut libc::c_char;
    merge_book_file = 0 as *mut libc::c_char;
    first_stage = 1 as libc::c_int;
    last_stage = 0 as libc::c_int;
    clear_flags = 0 as libc::c_int;
    clear_low = 0 as libc::c_int;
    clear_high = 60 as libc::c_int;
    error = 0 as libc::c_int;
    arg_index = 1 as libc::c_int;
    while arg_index < argc && error == 0 {
        if strcasecmp(*argv.offset(arg_index as isize),
                      b"-i\x00" as *const u8 as *const libc::c_char) == 0 {
            import_games = 1 as libc::c_int;
            arg_index += 1;
            import_file_name = *argv.offset(arg_index as isize);
            arg_index += 1;
            max_game_count = atoi(*argv.offset(arg_index as isize));
            build_tree(import_file_name, max_game_count, max_diff, cutoff);
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-r\x00" as *const u8 as *const libc::c_char) ==
            0 ||
            strcasecmp(*argv.offset(arg_index as isize),
                       b"-rb\x00" as *const u8 as
                           *const libc::c_char) == 0 {
            if input_database != 0 {
                puts(b"Only one database can be read.\x00" as *const u8 as
                    *const libc::c_char);
                exit(1 as libc::c_int);
            }
            if import_games != 0 {
                puts(b"Can\'t load database after having imported games.\x00"
                    as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
            input_database = 1 as libc::c_int;
            input_binary =
                (strcasecmp(*argv.offset(arg_index as isize),
                            b"-rb\x00" as *const u8 as *const libc::c_char) ==
                    0) as libc::c_int;
            arg_index += 1;
            input_file_name = *argv.offset(arg_index as isize);
            if input_binary != 0 {
                read_binary_database(input_file_name);
            } else { read_text_database(input_file_name); }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-w\x00" as *const u8 as *const libc::c_char) ==
            0 ||
            strcasecmp(*argv.offset(arg_index as isize),
                       b"-wb\x00" as *const u8 as
                           *const libc::c_char) == 0 ||
            strcasecmp(*argv.offset(arg_index as isize),
                       b"-wc\x00" as *const u8 as
                           *const libc::c_char) == 0 {
            output_database = 1 as libc::c_int;
            output_binary =
                (strcasecmp(*argv.offset(arg_index as isize),
                            b"-wb\x00" as *const u8 as *const libc::c_char) ==
                    0) as libc::c_int;
            output_compressed =
                (strcasecmp(*argv.offset(arg_index as isize),
                            b"-wc\x00" as *const u8 as *const libc::c_char) ==
                    0) as libc::c_int;
            arg_index += 1;
            output_file_name = *argv.offset(arg_index as isize)
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-uc\x00" as *const u8 as *const libc::c_char)
            == 0 {
            arg_index += 1;
            let mut compressed_name: *mut libc::c_char =
                *argv.offset(arg_index as isize);
            arg_index += 1;
            let mut target_name: *mut libc::c_char =
                *argv.offset(arg_index as isize);
            unpack_compressed_database(compressed_name, target_name);
            uncompress_database = 1 as libc::c_int;
            exit(0 as libc::c_int);
        } else {
            if strcasecmp(*argv.offset(arg_index as isize),
                          b"-m\x00" as *const u8 as *const libc::c_char) == 0
            {
                calculate_minimax = 1 as libc::c_int
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-ld\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                arg_index += 1;
                low_threshold = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                high_threshold = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                bonus = atof(*argv.offset(arg_index as isize));
                set_deviation_value(low_threshold, high_threshold, bonus);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-e\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                evaluate_all = 1 as libc::c_int
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-d\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                display_line = 1 as libc::c_int
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-c\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                arg_index += 1;
                cutoff = atoi(*argv.offset(arg_index as isize))
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-pm\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                do_statistics = 1 as libc::c_int;
                statistics_type = MIDGAME_STATISTICS;
                arg_index += 1;
                max_depth = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                probability = atof(*argv.offset(arg_index as isize));
                arg_index += 1;
                max_diff = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                statistics_file_name = *argv.offset(arg_index as isize)
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-pe\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                do_statistics = 1 as libc::c_int;
                statistics_type = ENDGAME_STATISTICS;
                arg_index += 1;
                max_depth = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                probability = atof(*argv.offset(arg_index as isize));
                arg_index += 1;
                max_diff = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                statistics_file_name = *argv.offset(arg_index as isize)
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-o\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                arg_index += 1;
                max_diff = atoi(*argv.offset(arg_index as isize))
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-l\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                arg_index += 1;
                set_search_depth(atoi(*argv.offset(arg_index as isize)));
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-evalspan\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                arg_index += 1;
                min_eval_span = atof(*argv.offset(arg_index as isize));
                arg_index += 1;
                max_eval_span = atof(*argv.offset(arg_index as isize));
                set_eval_span(min_eval_span, max_eval_span);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-negspan\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                arg_index += 1;
                min_negamax_span = atof(*argv.offset(arg_index as isize));
                arg_index += 1;
                max_negamax_span = atof(*argv.offset(arg_index as isize));
                set_negamax_span(min_negamax_span, max_negamax_span);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-batch\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                arg_index += 1;
                set_max_batch_size(atoi(*argv.offset(arg_index as isize)));
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-stat\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                complete_statistics = 1 as libc::c_int
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-help\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                give_help = 1 as libc::c_int
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-end\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                endgame_correct = 1 as libc::c_int;
                arg_index += 1;
                max_empty = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                full_solve = atoi(*argv.offset(arg_index as isize))
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-script\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                arg_index += 1;
                set_output_script_name(*argv.offset(arg_index as isize));
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-private\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                set_game_mode(PRIVATE_GAME);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-public\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                set_game_mode(PUBLIC_GAME);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-keepdraw\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                set_draw_mode(NEUTRAL);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-draw2black\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                set_draw_mode(BLACK_WINS);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-draw2white\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                set_draw_mode(WHITE_WINS);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-draw2none\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                set_draw_mode(OPPONENT_WINS);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-opgen\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                process_openings = 1 as libc::c_int;
                arg_index += 1;
                opening_in_file = *argv.offset(arg_index as isize)
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-dump\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                dump_positions = 1 as libc::c_int;
                arg_index += 1;
                position_file = *argv.offset(arg_index as isize);
                arg_index += 1;
                first_stage = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                last_stage = atoi(*argv.offset(arg_index as isize))
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-clearmid\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                clear_flags |= 1 as libc::c_int
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-clearwld\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                clear_flags |= 2 as libc::c_int
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-clearexact\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                clear_flags |= 4 as libc::c_int
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-clearbounds\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                arg_index += 1;
                clear_low = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                clear_high = atoi(*argv.offset(arg_index as isize))
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-h\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                arg_index += 1;
                let mut hash_bits: libc::c_int =
                    atoi(*argv.offset(arg_index as isize));
                resize_hash::<FE>(hash_bits);
                printf(b"Hash table size changed to %d elements\n\x00" as
                           *const u8 as *const libc::c_char,
                       (1 as libc::c_int) << hash_bits);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-fb\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                set_black_force(1 as libc::c_int);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-fw\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                set_white_force(1 as libc::c_int);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-merge\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                arg_index += 1;
                merge_script_file = *argv.offset(arg_index as isize);
                arg_index += 1;
                merge_output_file = *argv.offset(arg_index as isize)
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-export\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                arg_index += 1;
                export_file = *argv.offset(arg_index as isize)
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-mergebook\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                arg_index += 1;
                merge_book_file = *argv.offset(arg_index as isize)
            } else { error = 1 as libc::c_int }
        }
        if arg_index >= argc { error = 1 as libc::c_int }
        arg_index += 1
    }
    if import_games == 0 && input_database == 0 && process_openings == 0 &&
        uncompress_database == 0 {
        error = 1 as libc::c_int
    }
    if error != 0 || give_help != 0 {
        puts(b"Usage:\x00" as *const u8 as *const libc::c_char);
        puts(b"  osf [-i <game file> <max #games>]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-r <database> | -rb <database>]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-w <database> | -wb <database> | -wc <database>]\x00" as
            *const u8 as *const libc::c_char);
        puts(b"      [-uc <compressed file> <binary database>]\x00" as
            *const u8 as *const libc::c_char);
        puts(b"      [-c <cutoff>] [-o <outcome>] [-d]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-m] [-e] [-l <depth>]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-ld <low> <high> <bonus>]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [{-pm | pe} <depth> <prob> <max diff> <file name>]\x00"
            as *const u8 as *const libc::c_char);
        puts(b"      [-batch <size>] [-stat]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-negspan <min> <max>] [-evalspan <min> <max>]\x00" as
            *const u8 as *const libc::c_char);
        puts(b"      [-end <max empty> <full>]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-script <script name>]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-private] [-public]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-keepdraw] [-draw2black] [-draw2white] [-draw2none]\x00"
            as *const u8 as *const libc::c_char);
        puts(b"      [-opgen <opening list file>]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-dump <position file> <stage lo> <stage hi>]\x00" as
            *const u8 as *const libc::c_char);
        puts(b"      [-clearmid] [-clearwld] [-clearexact]\x00" as *const u8
            as *const libc::c_char);
        puts(b"      [-clearbounds <low> <high>]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-h <hash bits>]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-fb -fw]\x00" as *const u8 as *const libc::c_char);
        puts(b"      [-merge <script file> <output file>]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-mergebook <binary book file>]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-export <file>]\x00" as *const u8 as
            *const libc::c_char);
        puts(b"      [-help]\x00" as *const u8 as *const libc::c_char);
        puts(b"\x00" as *const u8 as *const libc::c_char);
        if give_help != 0 {
            puts(b"Flags:\x00" as *const u8 as *const libc::c_char);
            puts(b"  -i        Imports the game list in <game file>. At most <#games> are loaded.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -r/rb     Reads a database as text (-r) or binary (-rb).\x00"
                as *const u8 as *const libc::c_char);
            printf(b"  -c        Import games up to <cutoff> empties. (Default: %d)\n\x00"
                       as *const u8 as *const libc::c_char,
                   16 as libc::c_int);
            puts(b"            Only applies to subsequent \'-i\' commands.\x00"
                as *const u8 as *const libc::c_char);
            printf(b"  -o        Only import games with result < <outcom>. (Default: %d)\n\x00"
                       as *const u8 as *const libc::c_char,
                   24 as libc::c_int);
            puts(b"            Only applies to subsequent \'-i\' commands.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -d        Displays the optimal minimax book line.\x00" as
                *const u8 as *const libc::c_char);
            puts(b"  -w/wb/wc  Saves db as text (-w) / binary (-wb) / compressed (wc).\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -uc       Uncompresses compressed db to binary db.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -m        Calculate the minimax values of all nodes.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -ld       Deviations before <high> disks played are given a\x00"
                as *const u8 as *const libc::c_char);
            puts(b"            bonus of <bonus> per disk. Before <low> disks played\x00"
                as *const u8 as *const libc::c_char);
            puts(b"            no bonus is given.\x00" as *const u8 as
                *const libc::c_char);
            puts(b"            punishment of <punishment> disks per move.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -e        Evaluates all the nodes in the tree.\x00" as
                *const u8 as *const libc::c_char);
            puts(b"  -l        Learn the games using searches to depth <depth>.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -pm       Generate midgame Prob-Cut statistics.\x00" as
                *const u8 as *const libc::c_char);
            puts(b"  -pe       Generate endgame Prob-Cut statistics.\x00" as
                *const u8 as *const libc::c_char);
            puts(b"  -evalspan Select nodes with evals in <minspan>-<maxspan>\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -negspan  Select nodes with negamax in <minspan>-<maxspan>\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -batch    At most search <size> nodes.\x00" as *const u8
                as *const libc::c_char);
            puts(b"  -stat     Give full statistics for the tree.\x00" as
                *const u8 as *const libc::c_char);
            puts(b"  -help     Displays this text.\x00" as *const u8 as
                *const libc::c_char);
            puts(b"  -end      Corrects all nodes with <= <empty> disks.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"            <full>=0 ==> WLD, otherwise exact score.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -script   With -end: Positions are written to <script file>.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -private  Treats all draws as losses for both sides (default).\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -public   No tweaking of draw scores.\x00" as *const u8
                as *const libc::c_char);
            puts(b"  -keepdraw Book draws are counted as draws.\x00" as
                *const u8 as *const libc::c_char);
            puts(b"  -draw2black Book draws scored as 32-31 for black.\x00" as
                *const u8 as *const libc::c_char);
            puts(b"  -draw2white Book draws scored as 32-31 for white.\x00" as
                *const u8 as *const libc::c_char);
            puts(b"  -draw2none  Book draws scored as 32-31 for the opponent.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -opgen    Converts the openings in <list file> to source code.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -clear*   Removes midgame/wld/full status from nodes.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -clearbounds   Only remove from <low> to <high> moves.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -dump     Save scores for positions with <lo> to <hi> moves.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -h        Changes hash table size to 2^<hash bits>.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -fb/fw    Force black/white\'s to only recurse optimal moves.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -merge    Adds the scores from <output> defined in <script> to the book.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -mergebook  Adds the positions in <book file> to the book.\x00"
                as *const u8 as *const libc::c_char);
            puts(b"  -export   Saves all lines in the book to <file>.\x00" as
                *const u8 as *const libc::c_char);
            puts(b"\x00" as *const u8 as *const libc::c_char);
            puts(b"Gunnar Andersson, December 30, 2004\x00" as *const u8 as
                *const libc::c_char);
        } else {
            puts(b"Try \'osf -help\' for a description of the switches.\x00"
                as *const u8 as *const libc::c_char);
        }
    }
    if error != 0 { exit(1 as libc::c_int); }
    if process_openings != 0 { convert_opening_list(opening_in_file); }
    if import_games != 0 || input_database != 0 {
        book_statistics(complete_statistics);
    }
    if clear_flags != 0 { clear_tree(clear_low, clear_high, clear_flags); }
    if !merge_book_file.is_null() { merge_binary_database(merge_book_file); }
    if evaluate_all != 0 { evaluate_tree(); }
    if endgame_correct != 0 { correct_tree(max_empty, full_solve); }
    if !merge_script_file.is_null() && !merge_output_file.is_null() {
        merge_position_list::<LibcFatalError>(merge_script_file, merge_output_file);
    }
    if calculate_minimax != 0 { minimax_tree(); }
    if dump_positions != 0 {
        restricted_minimax_tree(first_stage, last_stage, position_file);
    }
    if !export_file.is_null() {
        puts(b"exporting\x00" as *const u8 as *const libc::c_char);
        export_tree(export_file);
    }
    if display_line != 0 {
        display_doubly_optimal_line(0 as libc::c_int);
        display_doubly_optimal_line(2 as libc::c_int);
    }
    if do_statistics != 0 {
        if statistics_type as libc::c_uint ==
            MIDGAME_STATISTICS as libc::c_int as libc::c_uint {
            generate_midgame_statistics(max_depth, probability,
                                        max_diff * 128 as libc::c_int,
                                        statistics_file_name);
        } else {
            generate_endgame_statistics(max_depth, probability, max_diff,
                                        statistics_file_name);
        }
    }
    if output_database != 0 {
        if output_binary != 0 {
            write_binary_database(output_file_name);
        } else if output_compressed != 0 {
            write_compressed_database(output_file_name);
        } else { write_text_database(output_file_name); }
    }
    return 0 as libc::c_int;
}

pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
