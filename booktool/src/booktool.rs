#![allow(dead_code,  non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut)]

use c2rust_out::src::osfbook::{write_text_database, write_compressed_database, write_binary_database, generate_endgame_statistics, generate_midgame_statistics, display_doubly_optimal_line, export_tree, restricted_minimax_tree, minimax_tree, merge_position_list, correct_tree, evaluate_tree, merge_binary_database, clear_tree, book_statistics, convert_opening_list, set_output_script_name, unpack_compressed_database, read_text_database, read_binary_database, build_tree, init_osf};
use engine::src::osfbook::{set_draw_mode, set_black_force, set_white_force, set_deviation_value, set_search_depth, set_eval_span, set_negamax_span, set_max_batch_size, set_game_mode};
use engine::src::hash::resize_hash;
use c2rust_out::src::error::{LibcFatalError};
pub type FE = LibcFatalError;

extern "C" {
    #[no_mangle]
    fn printf(_: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn puts(__s: *const i8) -> i32;
    #[no_mangle]
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8)
              -> f64;
    #[no_mangle]
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8,
              __base: i32) -> i64;
    #[no_mangle]
    fn exit(_: i32) -> !;
    #[no_mangle]
    fn strcasecmp(_: *const i8, _: *const i8)
                  -> i32;
}
pub type DrawMode = u32;
pub const OPPONENT_WINS: DrawMode = 3;
pub const WHITE_WINS: DrawMode = 2;
pub const BLACK_WINS: DrawMode = 1;
pub const NEUTRAL: DrawMode = 0;
pub type GameMode = u32;
pub const PUBLIC_GAME: GameMode = 1;
pub const PRIVATE_GAME: GameMode = 0;
pub const MIDGAME_STATISTICS: C2RustUnnamed = 0;
pub type C2RustUnnamed = u32;
pub const ENDGAME_STATISTICS: C2RustUnnamed = 1;
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const i8) -> f64 {
    return strtod(__nptr, 0 as *mut ::std::ffi::c_void as *mut *mut i8);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut ::std::ffi::c_void as *mut *mut i8,
                  10 as i32) as i32;
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8)
                 -> i32 {
    let mut import_file_name: *mut i8 = 0 as *mut i8;
    let mut input_file_name: *mut i8 = 0 as *mut i8;
    let mut output_file_name: *mut i8 = 0 as *mut i8;
    let mut statistics_file_name: *mut i8 = 0 as *mut i8;
    let mut opening_in_file: *mut i8 = 0 as *mut i8;
    let mut position_file: *mut i8 = 0 as *mut i8;
    let mut opening_file: *mut i8 = 0 as *mut i8;
    let mut merge_script_file: *mut i8 = 0 as *mut i8;
    let mut merge_output_file: *mut i8 = 0 as *mut i8;
    let mut export_file: *mut i8 = 0 as *mut i8;
    let mut merge_book_file: *mut i8 = 0 as *mut i8;
    let mut statistics_type: C2RustUnnamed = MIDGAME_STATISTICS;
    let mut probability: f64 = 0.;
    let mut bonus: f64 = 0.;
    let mut min_eval_span: f64 = 0.;
    let mut max_eval_span: f64 = 0.;
    let mut min_negamax_span: f64 = 0.;
    let mut max_negamax_span: f64 = 0.;
    let mut error: i32 = 0;
    let mut arg_index: i32 = 0;
    let mut max_game_count: i32 = 0;
    let mut max_diff: i32 = 0;
    let mut cutoff: i32 = 0;
    let mut import_games: i32 = 0;
    let mut input_database: i32 = 0;
    let mut output_database: i32 = 0;
    let mut input_binary: i32 = 0;
    let mut output_binary: i32 = 0;
    let mut output_compressed: i32 = 0;
    let mut calculate_minimax: i32 = 0;
    let mut evaluate_all: i32 = 0;
    let mut display_line: i32 = 0;
    let mut do_statistics: i32 = 0;
    let mut max_depth: i32 = 0;
    let mut low_threshold: i32 = 0;
    let mut high_threshold: i32 = 0;
    let mut complete_statistics: i32 = 0;
    let mut give_help: i32 = 0;
    let mut process_openings: i32 = 0;
    let mut uncompress_database: i32 = 0;
    let mut endgame_correct: i32 = 0;
    let mut max_empty: i32 = 0;
    let mut full_solve: i32 = 0;
    let mut dump_positions: i32 = 0;
    let mut first_stage: i32 = 0;
    let mut last_stage: i32 = 0;
    let mut clear_flags: i32 = 0;
    let mut clear_low: i32 = 0;
    let mut clear_high: i32 = 0;
    init_osf(1 as i32);
    cutoff = 16 as i32;
    max_diff = 24 as i32;
    max_game_count = 0 as i32;
    import_games = 0 as i32;
    import_file_name = 0 as *mut i8;
    input_database = 0 as i32;
    input_file_name = 0 as *mut i8;
    input_binary = 0 as i32;
    output_database = 0 as i32;
    output_file_name = 0 as *mut i8;
    output_binary = 1 as i32;
    output_compressed = 0 as i32;
    uncompress_database = 0 as i32;
    calculate_minimax = 0 as i32;
    low_threshold = 60 as i32;
    high_threshold = 60 as i32;
    bonus = 0.0f64;
    evaluate_all = 0 as i32;
    display_line = 0 as i32;
    do_statistics = 0 as i32;
    statistics_file_name = 0 as *mut i8;
    probability = 0.0f64;
    max_depth = 0 as i32;
    complete_statistics = 0 as i32;
    statistics_type = MIDGAME_STATISTICS;
    give_help = 0 as i32;
    endgame_correct = 0 as i32;
    max_empty = 0 as i32;
    full_solve = 0 as i32;
    process_openings = 0 as i32;
    opening_in_file = 0 as *mut i8;
    dump_positions = 0 as i32;
    position_file = 0 as *mut i8;
    opening_file = 0 as *mut i8;
    merge_script_file = 0 as *mut i8;
    merge_output_file = 0 as *mut i8;
    export_file = 0 as *mut i8;
    merge_book_file = 0 as *mut i8;
    first_stage = 1 as i32;
    last_stage = 0 as i32;
    clear_flags = 0 as i32;
    clear_low = 0 as i32;
    clear_high = 60 as i32;
    error = 0 as i32;
    arg_index = 1 as i32;
    while arg_index < argc && error == 0 {
        if strcasecmp(*argv.offset(arg_index as isize),
                      b"-i\x00" as *const u8 as *const i8) == 0 {
            import_games = 1 as i32;
            arg_index += 1;
            import_file_name = *argv.offset(arg_index as isize);
            arg_index += 1;
            max_game_count = atoi(*argv.offset(arg_index as isize));
            build_tree(import_file_name, max_game_count, max_diff, cutoff);
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-r\x00" as *const u8 as *const i8) ==
            0 ||
            strcasecmp(*argv.offset(arg_index as isize),
                       b"-rb\x00" as *const u8 as
                           *const i8) == 0 {
            if input_database != 0 {
                puts(b"Only one database can be read.\x00" as *const u8 as
                    *const i8);
                exit(1 as i32);
            }
            if import_games != 0 {
                puts(b"Can\'t load database after having imported games.\x00"
                    as *const u8 as *const i8);
                exit(1 as i32);
            }
            input_database = 1 as i32;
            input_binary =
                (strcasecmp(*argv.offset(arg_index as isize),
                            b"-rb\x00" as *const u8 as *const i8) ==
                    0) as i32;
            arg_index += 1;
            input_file_name = *argv.offset(arg_index as isize);
            if input_binary != 0 {
                read_binary_database(input_file_name);
            } else { read_text_database(input_file_name); }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-w\x00" as *const u8 as *const i8) ==
            0 ||
            strcasecmp(*argv.offset(arg_index as isize),
                       b"-wb\x00" as *const u8 as
                           *const i8) == 0 ||
            strcasecmp(*argv.offset(arg_index as isize),
                       b"-wc\x00" as *const u8 as
                           *const i8) == 0 {
            output_database = 1 as i32;
            output_binary =
                (strcasecmp(*argv.offset(arg_index as isize),
                            b"-wb\x00" as *const u8 as *const i8) ==
                    0) as i32;
            output_compressed =
                (strcasecmp(*argv.offset(arg_index as isize),
                            b"-wc\x00" as *const u8 as *const i8) ==
                    0) as i32;
            arg_index += 1;
            output_file_name = *argv.offset(arg_index as isize)
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-uc\x00" as *const u8 as *const i8)
            == 0 {
            arg_index += 1;
            let mut compressed_name: *mut i8 =
                *argv.offset(arg_index as isize);
            arg_index += 1;
            let mut target_name: *mut i8 =
                *argv.offset(arg_index as isize);
            unpack_compressed_database(compressed_name, target_name);
            uncompress_database = 1 as i32;
            exit(0 as i32);
        } else {
            if strcasecmp(*argv.offset(arg_index as isize),
                          b"-m\x00" as *const u8 as *const i8) == 0
            {
                calculate_minimax = 1 as i32
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-ld\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                low_threshold = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                high_threshold = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                bonus = atof(*argv.offset(arg_index as isize));
                set_deviation_value(low_threshold, high_threshold, bonus);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-e\x00" as *const u8 as
                                     *const i8) == 0 {
                evaluate_all = 1 as i32
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-d\x00" as *const u8 as
                                     *const i8) == 0 {
                display_line = 1 as i32
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-c\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                cutoff = atoi(*argv.offset(arg_index as isize))
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-pm\x00" as *const u8 as
                                     *const i8) == 0 {
                do_statistics = 1 as i32;
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
                                     *const i8) == 0 {
                do_statistics = 1 as i32;
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
                                     *const i8) == 0 {
                arg_index += 1;
                max_diff = atoi(*argv.offset(arg_index as isize))
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-l\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                set_search_depth(atoi(*argv.offset(arg_index as isize)));
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-evalspan\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                min_eval_span = atof(*argv.offset(arg_index as isize));
                arg_index += 1;
                max_eval_span = atof(*argv.offset(arg_index as isize));
                set_eval_span(min_eval_span, max_eval_span);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-negspan\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                min_negamax_span = atof(*argv.offset(arg_index as isize));
                arg_index += 1;
                max_negamax_span = atof(*argv.offset(arg_index as isize));
                set_negamax_span(min_negamax_span, max_negamax_span);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-batch\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                set_max_batch_size(atoi(*argv.offset(arg_index as isize)));
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-stat\x00" as *const u8 as
                                     *const i8) == 0 {
                complete_statistics = 1 as i32
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-help\x00" as *const u8 as
                                     *const i8) == 0 {
                give_help = 1 as i32
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-end\x00" as *const u8 as
                                     *const i8) == 0 {
                endgame_correct = 1 as i32;
                arg_index += 1;
                max_empty = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                full_solve = atoi(*argv.offset(arg_index as isize))
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-script\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                set_output_script_name(*argv.offset(arg_index as isize));
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-private\x00" as *const u8 as
                                     *const i8) == 0 {
                set_game_mode(PRIVATE_GAME);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-public\x00" as *const u8 as
                                     *const i8) == 0 {
                set_game_mode(PUBLIC_GAME);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-keepdraw\x00" as *const u8 as
                                     *const i8) == 0 {
                set_draw_mode(NEUTRAL);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-draw2black\x00" as *const u8 as
                                     *const i8) == 0 {
                set_draw_mode(BLACK_WINS);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-draw2white\x00" as *const u8 as
                                     *const i8) == 0 {
                set_draw_mode(WHITE_WINS);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-draw2none\x00" as *const u8 as
                                     *const i8) == 0 {
                set_draw_mode(OPPONENT_WINS);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-opgen\x00" as *const u8 as
                                     *const i8) == 0 {
                process_openings = 1 as i32;
                arg_index += 1;
                opening_in_file = *argv.offset(arg_index as isize)
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-dump\x00" as *const u8 as
                                     *const i8) == 0 {
                dump_positions = 1 as i32;
                arg_index += 1;
                position_file = *argv.offset(arg_index as isize);
                arg_index += 1;
                first_stage = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                last_stage = atoi(*argv.offset(arg_index as isize))
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-clearmid\x00" as *const u8 as
                                     *const i8) == 0 {
                clear_flags |= 1 as i32
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-clearwld\x00" as *const u8 as
                                     *const i8) == 0 {
                clear_flags |= 2 as i32
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-clearexact\x00" as *const u8 as
                                     *const i8) == 0 {
                clear_flags |= 4 as i32
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-clearbounds\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                clear_low = atoi(*argv.offset(arg_index as isize));
                arg_index += 1;
                clear_high = atoi(*argv.offset(arg_index as isize))
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-h\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                let mut hash_bits: i32 =
                    atoi(*argv.offset(arg_index as isize));
                resize_hash::<FE>(hash_bits);
                printf(b"Hash table size changed to %d elements\n\x00" as
                           *const u8 as *const i8,
                       (1 as i32) << hash_bits);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-fb\x00" as *const u8 as
                                     *const i8) == 0 {
                set_black_force(1 as i32);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-fw\x00" as *const u8 as
                                     *const i8) == 0 {
                set_white_force(1 as i32);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-merge\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                merge_script_file = *argv.offset(arg_index as isize);
                arg_index += 1;
                merge_output_file = *argv.offset(arg_index as isize)
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-export\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                export_file = *argv.offset(arg_index as isize)
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-mergebook\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                merge_book_file = *argv.offset(arg_index as isize)
            } else { error = 1 as i32 }
        }
        if arg_index >= argc { error = 1 as i32 }
        arg_index += 1
    }
    if import_games == 0 && input_database == 0 && process_openings == 0 &&
        uncompress_database == 0 {
        error = 1 as i32
    }
    if error != 0 || give_help != 0 {
        puts(b"Usage:\x00" as *const u8 as *const i8);
        puts(b"  osf [-i <game file> <max #games>]\x00" as *const u8 as
            *const i8);
        puts(b"      [-r <database> | -rb <database>]\x00" as *const u8 as
            *const i8);
        puts(b"      [-w <database> | -wb <database> | -wc <database>]\x00" as
            *const u8 as *const i8);
        puts(b"      [-uc <compressed file> <binary database>]\x00" as
            *const u8 as *const i8);
        puts(b"      [-c <cutoff>] [-o <outcome>] [-d]\x00" as *const u8 as
            *const i8);
        puts(b"      [-m] [-e] [-l <depth>]\x00" as *const u8 as
            *const i8);
        puts(b"      [-ld <low> <high> <bonus>]\x00" as *const u8 as
            *const i8);
        puts(b"      [{-pm | pe} <depth> <prob> <max diff> <file name>]\x00"
            as *const u8 as *const i8);
        puts(b"      [-batch <size>] [-stat]\x00" as *const u8 as
            *const i8);
        puts(b"      [-negspan <min> <max>] [-evalspan <min> <max>]\x00" as
            *const u8 as *const i8);
        puts(b"      [-end <max empty> <full>]\x00" as *const u8 as
            *const i8);
        puts(b"      [-script <script name>]\x00" as *const u8 as
            *const i8);
        puts(b"      [-private] [-public]\x00" as *const u8 as
            *const i8);
        puts(b"      [-keepdraw] [-draw2black] [-draw2white] [-draw2none]\x00"
            as *const u8 as *const i8);
        puts(b"      [-opgen <opening list file>]\x00" as *const u8 as
            *const i8);
        puts(b"      [-dump <position file> <stage lo> <stage hi>]\x00" as
            *const u8 as *const i8);
        puts(b"      [-clearmid] [-clearwld] [-clearexact]\x00" as *const u8
            as *const i8);
        puts(b"      [-clearbounds <low> <high>]\x00" as *const u8 as
            *const i8);
        puts(b"      [-h <hash bits>]\x00" as *const u8 as
            *const i8);
        puts(b"      [-fb -fw]\x00" as *const u8 as *const i8);
        puts(b"      [-merge <script file> <output file>]\x00" as *const u8 as
            *const i8);
        puts(b"      [-mergebook <binary book file>]\x00" as *const u8 as
            *const i8);
        puts(b"      [-export <file>]\x00" as *const u8 as
            *const i8);
        puts(b"      [-help]\x00" as *const u8 as *const i8);
        puts(b"\x00" as *const u8 as *const i8);
        if give_help != 0 {
            puts(b"Flags:\x00" as *const u8 as *const i8);
            puts(b"  -i        Imports the game list in <game file>. At most <#games> are loaded.\x00"
                as *const u8 as *const i8);
            puts(b"  -r/rb     Reads a database as text (-r) or binary (-rb).\x00"
                as *const u8 as *const i8);
            printf(b"  -c        Import games up to <cutoff> empties. (Default: %d)\n\x00"
                       as *const u8 as *const i8,
                   16 as i32);
            puts(b"            Only applies to subsequent \'-i\' commands.\x00"
                as *const u8 as *const i8);
            printf(b"  -o        Only import games with result < <outcom>. (Default: %d)\n\x00"
                       as *const u8 as *const i8,
                   24 as i32);
            puts(b"            Only applies to subsequent \'-i\' commands.\x00"
                as *const u8 as *const i8);
            puts(b"  -d        Displays the optimal minimax book line.\x00" as
                *const u8 as *const i8);
            puts(b"  -w/wb/wc  Saves db as text (-w) / binary (-wb) / compressed (wc).\x00"
                as *const u8 as *const i8);
            puts(b"  -uc       Uncompresses compressed db to binary db.\x00"
                as *const u8 as *const i8);
            puts(b"  -m        Calculate the minimax values of all nodes.\x00"
                as *const u8 as *const i8);
            puts(b"  -ld       Deviations before <high> disks played are given a\x00"
                as *const u8 as *const i8);
            puts(b"            bonus of <bonus> per disk. Before <low> disks played\x00"
                as *const u8 as *const i8);
            puts(b"            no bonus is given.\x00" as *const u8 as
                *const i8);
            puts(b"            punishment of <punishment> disks per move.\x00"
                as *const u8 as *const i8);
            puts(b"  -e        Evaluates all the nodes in the tree.\x00" as
                *const u8 as *const i8);
            puts(b"  -l        Learn the games using searches to depth <depth>.\x00"
                as *const u8 as *const i8);
            puts(b"  -pm       Generate midgame Prob-Cut statistics.\x00" as
                *const u8 as *const i8);
            puts(b"  -pe       Generate endgame Prob-Cut statistics.\x00" as
                *const u8 as *const i8);
            puts(b"  -evalspan Select nodes with evals in <minspan>-<maxspan>\x00"
                as *const u8 as *const i8);
            puts(b"  -negspan  Select nodes with negamax in <minspan>-<maxspan>\x00"
                as *const u8 as *const i8);
            puts(b"  -batch    At most search <size> nodes.\x00" as *const u8
                as *const i8);
            puts(b"  -stat     Give full statistics for the tree.\x00" as
                *const u8 as *const i8);
            puts(b"  -help     Displays this text.\x00" as *const u8 as
                *const i8);
            puts(b"  -end      Corrects all nodes with <= <empty> disks.\x00"
                as *const u8 as *const i8);
            puts(b"            <full>=0 ==> WLD, otherwise exact score.\x00"
                as *const u8 as *const i8);
            puts(b"  -script   With -end: Positions are written to <script file>.\x00"
                as *const u8 as *const i8);
            puts(b"  -private  Treats all draws as losses for both sides (default).\x00"
                as *const u8 as *const i8);
            puts(b"  -public   No tweaking of draw scores.\x00" as *const u8
                as *const i8);
            puts(b"  -keepdraw Book draws are counted as draws.\x00" as
                *const u8 as *const i8);
            puts(b"  -draw2black Book draws scored as 32-31 for black.\x00" as
                *const u8 as *const i8);
            puts(b"  -draw2white Book draws scored as 32-31 for white.\x00" as
                *const u8 as *const i8);
            puts(b"  -draw2none  Book draws scored as 32-31 for the opponent.\x00"
                as *const u8 as *const i8);
            puts(b"  -opgen    Converts the openings in <list file> to source code.\x00"
                as *const u8 as *const i8);
            puts(b"  -clear*   Removes midgame/wld/full status from nodes.\x00"
                as *const u8 as *const i8);
            puts(b"  -clearbounds   Only remove from <low> to <high> moves.\x00"
                as *const u8 as *const i8);
            puts(b"  -dump     Save scores for positions with <lo> to <hi> moves.\x00"
                as *const u8 as *const i8);
            puts(b"  -h        Changes hash table size to 2^<hash bits>.\x00"
                as *const u8 as *const i8);
            puts(b"  -fb/fw    Force black/white\'s to only recurse optimal moves.\x00"
                as *const u8 as *const i8);
            puts(b"  -merge    Adds the scores from <output> defined in <script> to the book.\x00"
                as *const u8 as *const i8);
            puts(b"  -mergebook  Adds the positions in <book file> to the book.\x00"
                as *const u8 as *const i8);
            puts(b"  -export   Saves all lines in the book to <file>.\x00" as
                *const u8 as *const i8);
            puts(b"\x00" as *const u8 as *const i8);
            puts(b"Gunnar Andersson, December 30, 2004\x00" as *const u8 as
                *const i8);
        } else {
            puts(b"Try \'osf -help\' for a description of the switches.\x00"
                as *const u8 as *const i8);
        }
    }
    if error != 0 { exit(1 as i32); }
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
        puts(b"exporting\x00" as *const u8 as *const i8);
        export_tree(export_file);
    }
    if display_line != 0 {
        display_doubly_optimal_line(0 as i32);
        display_doubly_optimal_line(2 as i32);
    }
    if do_statistics != 0 {
        if statistics_type as u32 ==
            MIDGAME_STATISTICS as i32 as u32 {
            generate_midgame_statistics(max_depth, probability,
                                        max_diff * 128 as i32,
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
    return 0 as i32;
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as i32,
                                    args.as_mut_ptr() as
                                        *mut *mut i8) as i32)
    }
}
