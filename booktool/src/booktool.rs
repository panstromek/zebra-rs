#![allow(dead_code,  non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut, unused_must_use)]

use engine_traits::Offset;
use engine::src::osfbook::{set_deviation_value, set_max_batch_size, size_t, BookNode, probe_hash_table, get_hash, fill_move_alternatives, clear_node_depth, get_node_depth, adjust_score, _ISgraph, _ISupper, _ISprint, _ISspace};
use engine::src::zebra::DrawMode::{BLACK_WINS, NEUTRAL, OPPONENT_WINS, WHITE_WINS};
use engine::src::zebra::GameMode::{PRIVATE_GAME, PUBLIC_GAME};
use legacy_zebra::src::error::{LibcFatalError};
use legacy_zebra::src::zebra::{FullState, LibcTimeSource};

use libc_wrapper::{time, atof, fflush, stdout, fopen, fread, fclose, FileHandle, fprintf, fputc, free, sprintf, putc, fputs, stderr, sscanf, strlen, fgets, qsort, feof, strcmp, strstr, __ctype_b_loc, fwrite, malloc, toupper, ctime, strcpy, tolower, printf, puts, strcasecmp, atoi, exit};
use legacy_zebra::src::osfbook;
use engine::src::moves::{generate_all, make_move, unmake_move, unmake_move_no_hash, make_move_no_hash, generate_specific};
use engine::src::search::disc_count;
use legacy_zebra::src::safemem::safe_malloc;
use engine::src::zebra::EvaluationType;
use engine::src::zebra::EvalType::MIDGAME_EVAL;
use engine::src::zebra::EvalResult::{WON_POSITION};
use engine::src::stubs::{abs, floor};
use legacy_zebra::src::display::{display_board, display_state};
use engine::src::hash::{setup_hash, determine_hash_values};
use engine::src::midgame::middle_game;
use engine::src::end::end_game;
use engine::src::counter::reset_counter;
use legacy_zebra::src::osfbook::{write_text_database,write_binary_database,
                                 read_text_database, read_binary_database, init_osf, time_t, add_new_game, prepare_tree_traversal,
                                 do_minimax, evaluate_node, create_BookNode};
use engine::src::error::FrontEnd;
use legacy_zebra::src::game::game_init;
use std::io::Write;

pub type FE = LibcFatalError;
use legacy_zebra::fatal_error;
use std::ffi::CStr;

pub const MIDGAME_STATISTICS: C2RustUnnamed = 0;
pub type C2RustUnnamed = u32;
pub const ENDGAME_STATISTICS: C2RustUnnamed = 1;
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8)
                 -> i32 {
    let mut import_file_name: *mut i8 = 0 as *mut i8;
    let mut input_file_name: *mut i8 = 0 as *mut i8;
    let mut output_file_name: *mut i8 = 0 as *mut i8;
    let mut statistics_file_name: *mut i8 = 0 as *mut i8;
    let mut opening_in_file: *mut i8 = 0 as *mut i8;
    let mut position_file: *mut i8 = 0 as *mut i8;
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
    static time_src: LibcTimeSource = LibcTimeSource;
    let mut full_state = FullState::new(&LibcTimeSource);
    let g_state = &mut full_state;
    init_osf(1 as i32, g_state);
    cutoff = 16;
    max_diff = 24;
    max_game_count = 0;
    import_games = 0;
    import_file_name = 0 as *mut i8;
    input_database = 0;
    input_file_name = 0 as *mut i8;
    input_binary = 0;
    output_database = 0;
    output_file_name = 0 as *mut i8;
    output_binary = 1;
    output_compressed = 0;
    uncompress_database = 0;
    calculate_minimax = 0;
    low_threshold = 60;
    high_threshold = 60;
    bonus = 0.0f64;
    evaluate_all = 0;
    display_line = 0;
    do_statistics = 0;
    statistics_file_name = 0 as *mut i8;
    probability = 0.0f64;
    max_depth = 0;
    complete_statistics = 0;
    statistics_type = MIDGAME_STATISTICS;
    give_help = 0;
    endgame_correct = 0;
    max_empty = 0;
    full_solve = 0;
    process_openings = 0;
    opening_in_file = 0 as *mut i8;
    dump_positions = 0;
    position_file = 0 as *mut i8;
    merge_script_file = 0 as *mut i8;
    merge_output_file = 0 as *mut i8;
    export_file = 0 as *mut i8;
    merge_book_file = 0 as *mut i8;
    first_stage = 1;
    last_stage = 0;
    clear_flags = 0;
    clear_low = 0;
    clear_high = 60;
    error = 0;
    arg_index = 1;
    while arg_index < argc && error == 0 {
        if strcasecmp(*argv.offset(arg_index as isize),
                      b"-i\x00" as *const u8 as *const i8) == 0 {
            import_games = 1;
            arg_index += 1;
            import_file_name = *argv.offset(arg_index as isize);
            arg_index += 1;
            max_game_count = atoi(*argv.offset(arg_index as isize));
            build_tree(import_file_name, max_game_count, max_diff, cutoff, g_state.g_config.echo,  g_state);
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
            input_database = 1;
            input_binary =
                (strcasecmp(*argv.offset(arg_index as isize),
                            b"-rb\x00" as *const u8 as *const i8) ==
                    0) as i32;
            arg_index += 1;
            input_file_name = *argv.offset(arg_index as isize);
            if input_binary != 0 {
                read_binary_database(input_file_name, &mut g_state.g_book);
            } else { read_text_database(input_file_name, &mut g_state.g_book); }
        } else if strcasecmp(*argv.offset(arg_index as isize),
                             b"-w\x00" as *const u8 as *const i8) ==
            0 ||
            strcasecmp(*argv.offset(arg_index as isize),
                       b"-wb\x00" as *const u8 as
                           *const i8) == 0 ||
            strcasecmp(*argv.offset(arg_index as isize),
                       b"-wc\x00" as *const u8 as
                           *const i8) == 0 {
            output_database = 1;
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
            unpack_compressed_database(compressed_name, target_name, g_state);
            uncompress_database = 1;
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
                set_deviation_value(low_threshold, high_threshold, bonus, &mut g_state.g_book);
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
                do_statistics = 1;
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
                do_statistics = 1;
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
               g_state.g_book.set_search_depth(atoi(*argv.offset(arg_index as isize)));
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-evalspan\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                min_eval_span = atof(*argv.offset(arg_index as isize));
                arg_index += 1;
                max_eval_span = atof(*argv.offset(arg_index as isize));
               g_state.g_book.set_eval_span(min_eval_span, max_eval_span);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-negspan\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                min_negamax_span = atof(*argv.offset(arg_index as isize));
                arg_index += 1;
                max_negamax_span = atof(*argv.offset(arg_index as isize));
               g_state.g_book.set_negamax_span(min_negamax_span, max_negamax_span);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-batch\x00" as *const u8 as
                                     *const i8) == 0 {
                arg_index += 1;
                set_max_batch_size(atoi(*argv.offset(arg_index as isize)), &mut g_state.g_book );
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
                endgame_correct = 1;
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
                g_state.g_book.set_game_mode(PRIVATE_GAME);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-public\x00" as *const u8 as
                                     *const i8) == 0 {
                g_state.g_book.set_game_mode(PUBLIC_GAME);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-keepdraw\x00" as *const u8 as
                                     *const i8) == 0 {
                g_state.g_book.set_draw_mode(NEUTRAL);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-draw2black\x00" as *const u8 as
                                     *const i8) == 0 {
                g_state.g_book.set_draw_mode(BLACK_WINS);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-draw2white\x00" as *const u8 as
                                     *const i8) == 0 {
                g_state.g_book.set_draw_mode(WHITE_WINS);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-draw2none\x00" as *const u8 as
                                     *const i8) == 0 {
                g_state.g_book.set_draw_mode(OPPONENT_WINS);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-opgen\x00" as *const u8 as
                                     *const i8) == 0 {
                process_openings = 1;
                arg_index += 1;
                opening_in_file = *argv.offset(arg_index as isize)
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-dump\x00" as *const u8 as
                                     *const i8) == 0 {
                dump_positions = 1;
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
                g_state.hash_state.resize_hash(hash_bits, &mut g_state.random_instance);
                printf(b"Hash table size changed to %d elements\n\x00" as
                           *const u8 as *const i8,
                       (1 as i32) << hash_bits);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-fb\x00" as *const u8 as
                                     *const i8) == 0 {
                g_state.g_book.set_black_force(1 as i32);
            } else if strcasecmp(*argv.offset(arg_index as isize),
                                 b"-fw\x00" as *const u8 as
                                     *const i8) == 0 {
                g_state.g_book.set_white_force(1 as i32);
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
        write!(stdout, "\n");
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
            write!(stdout, "\n");
            puts(b"Gunnar Andersson, December 30, 2004\x00" as *const u8 as
                *const i8);
        } else {
            puts(b"Try \'osf -help\' for a description of the switches.\x00"
                as *const u8 as *const i8);
        }
    }
    if error != 0 { exit(1 as i32); }
    if process_openings != 0 { convert_opening_list(opening_in_file, g_state); }
    if import_games != 0 || input_database != 0 {
        book_statistics(complete_statistics, g_state);
    }
    if clear_flags != 0 { clear_tree(clear_low, clear_high, clear_flags,g_state); }
    if !merge_book_file.is_null() { merge_binary_database(merge_book_file, g_state); }
    if evaluate_all != 0 { evaluate_tree(g_state); }
    if endgame_correct != 0 { correct_tree(max_empty, full_solve, g_state); }
    if !merge_script_file.is_null() && !merge_output_file.is_null() {
        merge_position_list::<LibcFatalError>(merge_script_file, merge_output_file, g_state);
    }
    if calculate_minimax != 0 { minimax_tree(g_state); }
    if dump_positions != 0 {
        restricted_minimax_tree(first_stage, last_stage, position_file, g_state);
    }
    if !export_file.is_null() {
        puts(b"exporting\x00" as *const u8 as *const i8);
        export_tree(export_file, g_state);
    }
    if display_line != 0 {
        display_doubly_optimal_line(0 as i32, g_state);
        display_doubly_optimal_line(2 as i32, g_state);
    }
    if do_statistics != 0 {
        if statistics_type as u32 ==
            MIDGAME_STATISTICS as i32 as u32 {
            generate_midgame_statistics(max_depth, probability,
                                        max_diff * 128 as i32,
                                        statistics_file_name, g_state);
        } else {
            generate_endgame_statistics(max_depth, probability, max_diff,
                                        statistics_file_name, g_state);
        }
    }
    if output_database != 0 {
        if output_binary != 0 {
            write_binary_database(output_file_name, &mut g_state.g_book);
        } else if output_compressed != 0 {
            write_compressed_database(output_file_name, g_state);
        } else { write_text_database(output_file_name,&mut g_state.g_book); }
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

/*
   MERGE_BINARY_DATABASE
   Merges a binary database file with the current book.
*/
pub unsafe fn merge_binary_database(file_name:
                                                   *const i8, g_state: &mut FullState) {
    let mut start_time: time_t = 0;
    time(&mut start_time);
    libc_wrapper::printf(b"Importing binary opening database... \x00" as *const u8 as
               *const i8);
    fflush(stdout);
    let stream =
        fopen(file_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error!("{} '{}'\n", "Could not open database file", &CStr::from_ptr(file_name).to_str().unwrap());
    }
    let mut magic1: i16 = 0;
    let mut magic2: i16 = 0;
    fread(&mut magic1 as *mut i16 as *mut std::ffi::c_void,
          std::mem::size_of::<i16>() as u64,
          1 as i32 as size_t, stream);
    fread(&mut magic2 as *mut i16 as *mut std::ffi::c_void,
          std::mem::size_of::<i16>() as u64,
          1 as i32 as size_t, stream);
    if magic1 as i32 != 2718 as i32 ||
           magic2 as i32 != 2818 as i32 {
        fatal_error!("{}: {}", "Wrong checksum, might be an old version", &CStr::from_ptr(file_name).to_str().unwrap());
    }
    let mut merge_book_node_count: i32 = 0;
    fread(&mut merge_book_node_count as *mut i32 as *mut std::ffi::c_void,
          std::mem::size_of::<i32>() as u64,
          1 as i32 as size_t, stream);
    let mut merge_use_count = 0;
    let mut i: i32 = 0;
    i = 0;
    while i < merge_book_node_count {
        let mut merge_node =
            BookNode{hash_val1: 0,
                     hash_val2: 0,
                     black_minimax_score: 0,
                     white_minimax_score: 0,
                     best_alternative_move: 0,
                     alternative_score: 0,
                     flags: 0,};
        /* Read g_state.g_book.node. */
        fread(&mut merge_node.hash_val1 as *mut i32 as
                  *mut std::ffi::c_void,
              std::mem::size_of::<i32>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut merge_node.hash_val2 as *mut i32 as
                  *mut std::ffi::c_void,
              std::mem::size_of::<i32>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut merge_node.black_minimax_score as *mut i16 as
                  *mut std::ffi::c_void,
              std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut merge_node.white_minimax_score as *mut i16 as
                  *mut std::ffi::c_void,
              std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut merge_node.best_alternative_move as *mut i16 as
                  *mut std::ffi::c_void,
              std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut merge_node.alternative_score as *mut i16 as
                  *mut std::ffi::c_void,
              std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut merge_node.flags as *mut u16 as
                  *mut std::ffi::c_void,
              std::mem::size_of::<u16>() as u64,
              1 as i32 as size_t, stream);
        /* Look up g_state.g_book.node in existing database. */
        let slot =
            probe_hash_table(merge_node.hash_val1, merge_node.hash_val2, &mut g_state.g_book);
        if slot == -(1 as i32) ||
               *g_state.g_book.book_hash_table.offset(slot as isize) == -(1 as i32) {
            /* New position, add it without modifications. */
            let this_node =
                osfbook::create_BookNode(merge_node.hash_val1, merge_node.hash_val2,
                                         merge_node.flags, &mut g_state.g_book);
            *g_state.g_book.node.offset(this_node as isize) = merge_node;
            merge_use_count += 1
        } else {
            /* Existing position, use the book from the merge file if it contains
            better endgame information. */
            let index = *g_state.g_book.book_hash_table.offset(slot as isize);
            if merge_node.flags as i32 & 16 as i32 != 0 &&
                   (*g_state.g_book.node.offset(index as isize)).flags as i32 &
                       16 as i32 == 0 ||
                   merge_node.flags as i32 & 4 as i32 != 0 &&
                       (*g_state.g_book.node.offset(index as isize)).flags as i32 &
                           4 as i32 == 0 {
                *g_state.g_book.node.offset(index as isize) = merge_node;
                merge_use_count += 1
            }
        }
        i += 1
    }
    fclose(stream);
    /* Make sure the tree is in reasonably good shape after the merge. */
    minimax_tree(g_state);
    let mut stop_time: time_t = 0;
    time(&mut stop_time);
    libc_wrapper::printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
                         (stop_time - start_time) as i32);
    libc_wrapper::printf(b"Used %d out of %d nodes from the merge file.\x00" as *const u8 as
               *const i8, merge_use_count, merge_book_node_count);
}

/*
  EXPORT_POSITION
  Output the position and its value according to the database
  to file.
*/
unsafe fn export_position(side_to_move: i32,
                          score: i32,
                          target_file: FileHandle, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut black_mask: i32 = 0;
    let mut white_mask: i32 = 0;
    let mut hi_mask: i32 = 0;
    let mut lo_mask: i32 = 0;
    i = 1;
    while i <= 8 as i32 {
        black_mask = 0;
        white_mask = 0;
        j = 0;
        pos = 10 as i32 * i + 1 as i32;
        while j < 8 as i32 {
            if g_state.board_state.board[pos as usize] == 0 as i32 {
                black_mask |= (1 as i32) << j
            } else if g_state.board_state.board[pos as usize] == 2 as i32 {
                white_mask |= (1 as i32) << j
            }
            j += 1;
            pos += 1
        }
        hi_mask = black_mask >> 4 as i32;
        lo_mask = black_mask % 16 as i32;
        fprintf(target_file, b"%c%c\x00" as *const u8 as *const i8,
                hi_mask + ' ' as i32, lo_mask + ' ' as i32);
        hi_mask = white_mask >> 4 as i32;
        lo_mask = white_mask % 16 as i32;
        fprintf(target_file, b"%c%c\x00" as *const u8 as *const i8,
                hi_mask + ' ' as i32, lo_mask + ' ' as i32);
        i += 1
    }
    fprintf(target_file, b" \x00" as *const u8 as *const i8);
    if side_to_move == 0 as i32 {
        fputc('*' as i32, target_file);
    } else { fputc('O' as i32, target_file); }
    fprintf(target_file,
            b" %2d %+d\n\x00" as *const u8 as *const i8,
            g_state.moves_state.disks_played, score);
}

/*
   DO_RESTRICTED_MINIMAX
   Calculates the book-only minimax value of g_state.g_book.node INDEX,
   not caring about deviations from the database.
*/
unsafe fn do_restricted_minimax(index: i32,
                                low: i32,
                                high: i32,
                                target_file: FileHandle,
                                minimax_values:
                                           *mut i32, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut corrected_score: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move = 0;
    let mut child_count: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut best_score: i16 = 0;
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    /* Recursively minimax all children of the g_state.g_book.node */
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    if side_to_move == 0 as i32 {
        best_score = -(32000 as i32) as i16
    } else { best_score = 32000 as i32 as i16 }
    generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
    child_count = 0;
    i = 0;
    while i < g_state.moves_state.move_count[g_state.moves_state.disks_played as usize] {
        g_state.board_state.piece_count[0][g_state.moves_state.disks_played as usize] =
            disc_count(0 as i32, &g_state.board_state.board);
        g_state.board_state.piece_count[2][g_state.moves_state.disks_played as usize] =
            disc_count(2 as i32, &g_state.board_state.board);
        this_move = g_state.moves_state.move_list[g_state.moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_state.g_book);
        child = *g_state.g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_restricted_minimax(child, low, high, target_file,
                                  minimax_values, g_state);
            corrected_score = *minimax_values.offset(child as isize);
            if side_to_move == 0 as i32 &&
                corrected_score > best_score as i32 ||
                side_to_move == 2 as i32 &&
                    corrected_score < best_score as i32 {
                best_score = corrected_score as i16
            }
            child_count += 1
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
        };
        i += 1
    }
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 16 as i32
        != 0 ||
        (*g_state.g_book.node.offset(index as isize)).flags as i32 &
            4 as i32 != 0 && child_count == 0 as i32 {
        best_score = (*g_state.g_book.node.offset(index as isize)).black_minimax_score
    } else if child_count == 0 as i32 {
        libc_wrapper::printf(b"%d disks played\n\x00" as *const u8 as *const i8,
                             g_state.moves_state.disks_played);
        libc_wrapper::printf(b"Node #%d has no children and lacks WLD status\n\x00" as
                   *const u8 as *const i8, index);
        libc_wrapper::exit(1 as i32);
    }
    if best_score as i32 > 30000 as i32 {
        best_score =
            (best_score as i32 - 30000 as i32) as
                i16
    } else if (best_score as i32) < -(30000 as i32) {
        best_score =
            (best_score as i32 + 30000 as i32) as
                i16
    }
    *minimax_values.offset(index as isize) = best_score as i32;
    let ref mut fresh16 = (*g_state.g_book.node.offset(index as isize)).flags;
    *fresh16 = (*fresh16 as i32 ^ 8 as i32) as u16;
    if g_state.moves_state.disks_played >= low && g_state.moves_state.disks_played <= high {
        export_position(side_to_move, best_score as i32, target_file, g_state);
    };
}

/*
   RESTRICTED_MINIMAX_TREE
   Calculates the minimax values of all nodes in the tree,
   not
*/
pub unsafe fn restricted_minimax_tree(low: i32,
                                      high: i32,
                                      pos_file_name:
                                                 *const i8, g_state:&mut FullState) {
    let mut pos_file: FileHandle = FileHandle::null();
    let mut i: i32 = 0;
    let mut minimax_values: *mut i32 = 0 as *mut i32;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    libc_wrapper::printf(b"Calculating restricted minimax value... \x00" as *const u8 as
        *const i8);
    fflush(stdout);
    osfbook::prepare_tree_traversal(g_state);
    time(&mut start_time);
    /* Mark all nodes as not traversed */
    i = 0;
    while i < g_state.g_book.book_node_count {
        let ref mut fresh17 = (*g_state.g_book.node.offset(i as isize)).flags;
        *fresh17 =
            (*fresh17 as i32 | 8 as i32) as u16;
        i += 1
    }
    minimax_values =
        safe_malloc((g_state.g_book.book_node_count as
            u64).wrapping_mul(std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    pos_file =
        fopen(pos_file_name, b"a\x00" as *const u8 as *const i8);
    do_restricted_minimax(0 as i32, low, high, pos_file,
                          minimax_values, g_state);
    time(&mut stop_time);
    libc_wrapper::printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
                         (stop_time - start_time) as i32);
    libc_wrapper::puts(b"\x00" as *const u8 as *const i8);
    free(minimax_values as *mut ::std::ffi::c_void);
    fclose(pos_file);
}

/*
   DO_MIDGAME_STATISTICS
   Recursively makes sure a subtree is evaluated to the specified depth.
*/
unsafe fn do_midgame_statistics(index: i32,
                                spec: StatisticsSpec, echo:i32, g_state: &mut FullState) {
    let mut dummy_info: EvaluationType = EvaluationType::new();
    let mut i: i32 = 0;
    let mut depth: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut eval_list: [i32; 64] = [0; 64];
    let mut out_file: FileHandle = FileHandle::null();
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
    /* With a certain probability, search the position to a variety
     of different depths in order to determine correlations. */
    if ((g_state.random_instance.my_random() % 1000 as i32 as i64) as f64)
        < 1000.0f64 * spec.prob &&
        abs((*g_state.g_book.node.offset(index as isize)).black_minimax_score as
            i32) < spec.max_diff {
        display_board(&mut stdout, &g_state.board_state.board, 0 as i32,
                      0 as i32, 0 as i32, 0 as i32,
                      display_state.current_row,
                      display_state.black_player, display_state.black_time, display_state.black_eval,
                      display_state.white_player, display_state.white_time, display_state.white_eval,
                      &g_state.board_state.black_moves, &g_state.board_state.white_moves
        );
        setup_hash(0 as i32, &mut g_state.hash_state, &mut  g_state.random_instance);
        determine_hash_values(side_to_move, &g_state.board_state.board, &mut g_state.hash_state);
        depth = 1;
        while depth <= spec.max_depth {
            middle_game::<osfbook::FE>(side_to_move, depth, 0 as i32,
                                       &mut dummy_info, echo, &mut g_state.moves_state,
                                       &mut g_state.search_state,
                                       &mut g_state.board_state,
                                       &mut g_state.hash_state,
                                       &mut g_state.flip_stack_,
                                       &mut g_state.coeff_state, &mut g_state.prob_cut,
                                       &mut g_state.g_timer, &mut g_state.midgame_state);
            eval_list[depth as usize] = g_state.search_state.root_eval;
            libc_wrapper::printf(b"%2d: %-5d \x00" as *const u8 as *const i8,
                                 depth, eval_list[depth as usize]);
            depth += 2 as i32
        }
        libc_wrapper::puts(b"\x00" as *const u8 as *const i8);
        setup_hash(0 as i32, &mut g_state.hash_state, &mut  g_state.random_instance);
        determine_hash_values(side_to_move, &g_state.board_state.board, &mut g_state.hash_state);
        depth = 2;
        while depth <= spec.max_depth {
            middle_game::<osfbook::FE>(side_to_move, depth, 0 as i32,
                                       &mut dummy_info, echo, &mut g_state.moves_state,
                                       &mut g_state.search_state,
                                       &mut g_state.board_state,
                                       &mut g_state.hash_state,
                                       &mut g_state.flip_stack_,
                                       &mut g_state.coeff_state, &mut g_state.prob_cut,
                                       &mut g_state.g_timer, &mut g_state.midgame_state);
            eval_list[depth as usize] = g_state.search_state.root_eval;
            libc_wrapper::printf(b"%2d: %-5d \x00" as *const u8 as *const i8,
                                 depth, eval_list[depth as usize]);
            depth += 2 as i32
        }
        libc_wrapper::puts(b"\x00" as *const u8 as *const i8);
        /* Store the scores if the last eval is in the range [-20,20] */
        out_file =
            fopen(spec.out_file_name,
                  b"a\x00" as *const u8 as *const i8);
        if !out_file.is_null() &&
            abs(eval_list[spec.max_depth as usize]) <=
                20 as i32 * 128 as i32 {
            let val0___ = &mut val1;
            let val1___ = &mut val2;
            let orientation___ = &mut orientation;
            get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
            fprintf(out_file,
                    b"%08x%08x %2d \x00" as *const u8 as *const i8,
                    val1, val2, g_state.moves_state.disks_played);
            fprintf(out_file,
                    b"%2d %2d \x00" as *const u8 as *const i8,
                    1 as i32, spec.max_depth);
            i = 1;
            while i <= spec.max_depth {
                fprintf(out_file,
                        b"%5d \x00" as *const u8 as *const i8,
                        eval_list[i as usize]);
                i += 1
            }
            fprintf(out_file, b"\n\x00" as *const u8 as *const i8);
            fclose(out_file);
        }
    }
    /* Recursively search the children of the g_state.g_book.node */
    i = 0;
    while i < g_state.moves_state.move_count[g_state.moves_state.disks_played as usize] {
        this_move = g_state.moves_state.move_list[g_state.moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_state.g_book);
        child = *g_state.g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_midgame_statistics(child, spec, echo, g_state);
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
        };
        i += 1
    }
    let ref mut fresh18 = (*g_state.g_book.node.offset(index as isize)).flags;
    *fresh18 = (*fresh18 as i32 ^ 8 as i32) as u16;
}

/*
   GENERATE_MIDGAME_STATISTICS
   Calculates the minimax values of all nodes in the tree.
*/
pub unsafe fn generate_midgame_statistics(max_depth:
                                                     i32,
                                          probability:
                                                     f64,
                                          max_diff:
                                                     i32,
                                          statistics_file_name:
                                                     *const i8, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut spec: StatisticsSpec =
        StatisticsSpec{out_file_name: 0 as *const i8,
            prob: 0.,
            max_diff: 0,
            max_depth: 0,};
    libc_wrapper::puts(b"Generating statistics...\n\x00" as *const u8 as
        *const i8);
    osfbook::prepare_tree_traversal(g_state);
    g_state.g_timer.toggle_abort_check(0 as i32);
    time(&mut start_time);
    i = 0;
    while i < g_state.g_book.book_node_count {
        let ref mut fresh19 = (*g_state.g_book.node.offset(i as isize)).flags;
        *fresh19 =
            (*fresh19 as i32 | 8 as i32) as u16;
        i += 1
    }
    spec.prob = probability;
    spec.max_diff = max_diff;
    spec.max_depth = max_depth;
    spec.out_file_name = statistics_file_name;
    let x = start_time as i32;
    g_state.random_instance.my_srandom(x);
    do_midgame_statistics(0 as i32, spec, g_state.g_config.echo, g_state);
    time(&mut stop_time);
    libc_wrapper::printf(b"\nDone (took %d s)\n\x00" as *const u8 as *const i8,
                         (stop_time - start_time) as i32);
    libc_wrapper::puts(b"\x00" as *const u8 as *const i8);
}

/*
   ENDGAME_CORRELATION
   Compare the scores produced by shallow searches to the
   exact score in an endgame position.
*/
unsafe fn endgame_correlation(mut side_to_move: i32,
                              best_score: i32,
                              best_move: i8,
                              min_disks: i32,
                              max_disks: i32,
                              spec: StatisticsSpec, echo:i32, g_state: &mut FullState) {
    let mut dummy_info: EvaluationType =EvaluationType::new();
    let mut out_file: FileHandle = FileHandle::null();
    let mut i: i32 = 0;
    let mut depth: i32 = 0;
    let mut stored_side_to_move: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut eval_list: [i32; 64] = [0; 64];
    display_board(&mut stdout, &g_state.board_state.board, 0 as i32,
                  0 as i32, 0 as i32, 0 as i32,
                  display_state.current_row,
                  display_state.black_player, display_state.black_time, display_state.black_eval,
                  display_state.white_player, display_state.white_time, display_state.white_eval,
                  &g_state.board_state.black_moves, &g_state.board_state.white_moves
    );
    g_state.hash_state.set_hash_transformation(abs(g_state.random_instance.my_random() as i32) as u32,
                                       abs(g_state.random_instance.my_random() as i32) as u32);
    determine_hash_values(side_to_move, &g_state.board_state.board, &mut g_state.hash_state);
    depth = 1;
    while depth <= spec.max_depth {
        middle_game::<osfbook::FE>(side_to_move, depth, 0 as i32, &mut dummy_info, echo, &mut g_state.moves_state,
                                   &mut g_state.search_state,
                                   &mut g_state.board_state,
                                   &mut g_state.hash_state,
                                   &mut g_state.flip_stack_,
                                   &mut g_state.coeff_state, &mut g_state.prob_cut,
                                   &mut g_state.g_timer, &mut g_state.midgame_state);
        eval_list[depth as usize] = g_state.search_state.root_eval;
        libc_wrapper::printf(b"%2d: %-6.2f \x00" as *const u8 as *const i8, depth,
                             eval_list[depth as usize] as f64 / 128.0f64);
        depth += 1
    }
    out_file =
        fopen(spec.out_file_name,
              b"a\x00" as *const u8 as *const i8);
    if !out_file.is_null() {
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
        fprintf(out_file,
                b"%08x%08x %2d \x00" as *const u8 as *const i8,
                val1, val2, g_state.moves_state.disks_played);
        fprintf(out_file, b"%+3d \x00" as *const u8 as *const i8,
                best_score);
        fprintf(out_file, b"%2d %2d \x00" as *const u8 as *const i8,
                1 as i32, spec.max_depth);
        i = 1;
        while i <= spec.max_depth {
            fprintf(out_file, b"%5d \x00" as *const u8 as *const i8,
                    eval_list[i as usize]);
            i += 1
        }
        fprintf(out_file, b"\n\x00" as *const u8 as *const i8);
        fclose(out_file);
    }
    if g_state.moves_state.disks_played < max_disks {
        make_move(side_to_move, best_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
        stored_side_to_move = side_to_move;
        side_to_move = 0 as i32 + 2 as i32 - side_to_move;
        generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
        if g_state.moves_state.move_count[g_state.moves_state.disks_played as usize] > 0 as i32 {
            libc_wrapper::printf(b"\nSolving with %d empty...\n\n\x00" as *const u8 as
                       *const i8, 60 as i32 - g_state.moves_state.disks_played);
            fill_move_alternatives::<osfbook::FE>(side_to_move, 16 as i32,
                                                  &mut g_state.g_book,
                                                  &mut g_state.board_state,
                                                  &mut g_state.moves_state,
                                                  &g_state.search_state,
                                                  &mut g_state.flip_stack_,
                                                  &mut g_state.hash_state);
            if g_state.g_book.get_candidate_count() > 0 as i32 ||
                g_state.moves_state.disks_played >= 40 as i32 {
                osfbook::print_move_alternatives(side_to_move, &mut g_state.board_state, &mut g_state.g_book);
                g_state.hash_state.set_hash_transformation(0 as i32 as u32,
                                        0 as i32 as u32);
               end_game::<osfbook::FE>(side_to_move, 0 as i32, 1 as i32,
                                       1 as i32, 0 as i32, &mut dummy_info, echo, &mut g_state.flip_stack_
                                       , &mut g_state.search_state
                                       , &mut g_state.board_state
                                       , &mut g_state.hash_state
                                       , &mut g_state.g_timer
                                       , &mut g_state.end_g
                                       , &mut g_state.midgame_state
                                       , &mut g_state.coeff_state
                                       , &mut g_state.moves_state
                                       , &mut g_state.random_instance
                                       , &mut g_state.g_book
                                       , &mut g_state.stable_state
                                       , &mut g_state.prob_cut);
                endgame_correlation(side_to_move, g_state.search_state.root_eval,
                                    g_state.board_state.pv[0][0],
                                    min_disks, max_disks, spec, echo, g_state);
            }
        }
        let side_to_move = stored_side_to_move;
        let move_0 = best_move;
        {
            unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
        };
    };
}

/*
   DO_ENDGAME_STATISTICS
   Recursively makes sure a subtree is evaluated to
   the specified depth.
*/
unsafe fn do_endgame_statistics(index: i32,
                                spec: StatisticsSpec, echo:i32 , g_state: &mut FullState) {
    let mut dummy_info: EvaluationType =EvaluationType::new();
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
    /* With a certain probability, search the position to a variety
     of different depths in order to determine correlations. */
    if g_state.moves_state.disks_played == 33 as i32 &&
        ((g_state.random_instance.my_random() % 1000 as i32 as i64) as
            f64) < 1000.0f64 * spec.prob {
        setup_hash(0 as i32, &mut g_state.hash_state, &mut  g_state.random_instance);
        determine_hash_values(side_to_move, &g_state.board_state.board, &mut g_state.hash_state);
        libc_wrapper::printf(b"\nSolving with %d empty...\n\n\x00" as *const u8 as
                   *const i8, 60 as i32 - g_state.moves_state.disks_played);
        fill_move_alternatives::<osfbook::FE>(side_to_move, 16 as i32,
                                              &mut g_state.g_book,
                                              &mut g_state.board_state,
                                              &mut g_state.moves_state,
                                              &g_state.search_state,
                                              &mut g_state.flip_stack_,
                                              &mut g_state.hash_state);
        if g_state.g_book.get_candidate_count() > 0 as i32 ||
            g_state.moves_state.disks_played >= 40 as i32 {
            osfbook::print_move_alternatives(side_to_move, &mut g_state.board_state, &mut g_state.g_book);
            g_state.hash_state.set_hash_transformation(0 as i32 as u32,
                                    0 as i32 as u32);
           end_game::<osfbook::FE>(side_to_move, 0 as i32, 1 as i32,
                                   1 as i32, 0 as i32, &mut dummy_info, echo, &mut g_state.flip_stack_
                                   , &mut g_state.search_state
                                   , &mut g_state.board_state
                                   , &mut g_state.hash_state
                                   , &mut g_state.g_timer
                                   , &mut g_state.end_g
                                   , &mut g_state.midgame_state
                                   , &mut g_state.coeff_state
                                   , &mut g_state.moves_state
                                   , &mut g_state.random_instance
                                   , &mut g_state.g_book
                                   , &mut g_state.stable_state
                                   , &mut g_state.prob_cut);
            if abs(g_state.search_state.root_eval) <= spec.max_diff {
                endgame_correlation(side_to_move, g_state.search_state.root_eval,
                                    g_state.board_state.pv[0][0],
                                    g_state.moves_state.disks_played, 48 as i32, spec, echo, g_state);
            }
        }
    }
    /* Recursively search the children of the g_state.g_book.node */
    i = 0;
    while i < g_state.moves_state.move_count[g_state.moves_state.disks_played as usize] {
        this_move = g_state.moves_state.move_list[g_state.moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_state.g_book);
        child = *g_state.g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_endgame_statistics(child, spec, echo, g_state);
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
        };
        i += 1
    }
    let ref mut fresh20 = (*g_state.g_book.node.offset(index as isize)).flags;
    *fresh20 = (*fresh20 as i32 ^ 8 as i32) as u16;
}

/*
   GENERATE_ENDGAME_STATISTICS
   Calculates the minimax values of all nodes in the tree.
*/
pub unsafe fn generate_endgame_statistics(max_depth:
                                                     i32,
                                          probability:
                                                     f64,
                                          max_diff:
                                                     i32,
                                          statistics_file_name:
                                                     *const i8, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut spec: StatisticsSpec =
        StatisticsSpec{out_file_name: 0 as *const i8,
            prob: 0.,
            max_diff: 0,
            max_depth: 0,};
    libc_wrapper::puts(b"Generating endgame statistics...\x00" as *const u8 as
        *const i8);
    osfbook::prepare_tree_traversal(g_state);
    g_state.g_timer.toggle_abort_check(0 as i32);
    time(&mut start_time);
    i = 0;
    while i < g_state.g_book.book_node_count {
        let ref mut fresh21 = (*g_state.g_book.node.offset(i as isize)).flags;
        *fresh21 =
            (*fresh21 as i32 | 8 as i32) as u16;
        i += 1
    }
    spec.prob = probability;
    spec.max_diff = max_diff;
    spec.max_depth = max_depth;
    spec.out_file_name = statistics_file_name;
    let x = start_time as i32;
    g_state.random_instance.my_srandom(x);
    do_endgame_statistics(0 as i32, spec, g_state.g_config.echo, g_state);
    time(&mut stop_time);
    libc_wrapper::printf(b"\nDone (took %d s)\n\x00" as *const u8 as *const i8,
                         (stop_time - start_time) as i32);
    libc_wrapper::puts(b"\x00" as *const u8 as *const i8);
}


/*
   DO_CLEAR
   Clears depth and flag information for all nodes with >= LOW
   and <= HIGH discs played. FLAGS specifies what kind of information
   is to be cleared - midgame, WLD or exact.
*/
unsafe fn do_clear(index: i32, low: i32,
                              high: i32, flags: i32, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if g_state.moves_state.disks_played >= low && g_state.moves_state.disks_played <= high {
        if flags & 1 as i32 != 0 { clear_node_depth(index, &mut g_state.g_book); }
        if (*g_state.g_book.node.offset(index as isize)).flags as i32 &
            4 as i32 != 0 && flags & 2 as i32 != 0 {
            let ref mut fresh27 = (*g_state.g_book.node.offset(index as isize)).flags;
            *fresh27 =
                (*fresh27 as i32 ^ 4 as i32) as u16
        }
        if (*g_state.g_book.node.offset(index as isize)).flags as i32 &
            16 as i32 != 0 && flags & 4 as i32 != 0 {
            let ref mut fresh28 = (*g_state.g_book.node.offset(index as isize)).flags;
            *fresh28 =
                (*fresh28 as i32 ^ 16 as i32) as
                    u16
        }
    }
    if g_state.moves_state.disks_played <= high {
        if (*g_state.g_book.node.offset(index as isize)).flags as i32 &
            1 as i32 != 0 {
            side_to_move = 0 as i32
        } else { side_to_move = 2 as i32 }
        generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
        i = 0;
        while i < g_state.moves_state.move_count[g_state.moves_state.disks_played as usize] {
            this_move = g_state.moves_state.move_list[g_state.moves_state.disks_played as usize][i as usize];
            make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
            let val0___ = &mut val1;
            let val1___ = &mut val2;
            let orientation___ = &mut orientation;
            get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
            slot = probe_hash_table(val1, val2, &mut g_state.g_book);
            child = *g_state.g_book.book_hash_table.offset(slot as isize);
            if child != -(1 as i32) {
                do_clear(child, low, high, flags, g_state);
            }
            let move_0 = this_move;
            {
                unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
            };
            i += 1
        }
    }
    let ref mut fresh29 = (*g_state.g_book.node.offset(index as isize)).flags;
    *fresh29 = (*fresh29 as i32 ^ 8 as i32) as u16;
}

/*
   CLEAR_TREE
   Resets the labels on nodes satisfying certain conditions.
*/
pub unsafe fn clear_tree(low: i32,
                         high: i32,
                         flags: i32, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    osfbook::prepare_tree_traversal(g_state);
    libc_wrapper::printf(b"Clearing from %d moves to %d modes: \x00" as *const u8 as
               *const i8, low, high);
    if flags & 1 as i32 != 0 {
        libc_wrapper::printf(b"midgame \x00" as *const u8 as *const i8);
    }
    if flags & 2 as i32 != 0 {
        libc_wrapper::printf(b"wld \x00" as *const u8 as *const i8);
    }
    if flags & 4 as i32 != 0 {
        libc_wrapper::printf(b"exact \x00" as *const u8 as *const i8);
    }
    libc_wrapper::puts(b"\x00" as *const u8 as *const i8);
    time(&mut start_time);
    i = 0;
    while i < g_state.g_book.book_node_count {
        let ref mut fresh30 = (*g_state.g_book.node.offset(i as isize)).flags;
        *fresh30 =
            (*fresh30 as i32 | 8 as i32) as u16;
        i += 1
    }
    do_clear(0 as i32, low, high, flags, g_state);
    time(&mut stop_time);
    libc_wrapper::printf(b"(took %d s)\n\x00" as *const u8 as *const i8,
                         (stop_time - start_time) as i32);
    libc_wrapper::puts(b"\x00" as *const u8 as *const i8);
}

/*
   DO_CORRECT
   Performs endgame correction (WLD or full solve) of a g_state.g_book.node
   and (recursively) the subtree below it.
*/
unsafe fn do_correct(index: i32,
                     max_empty: i32,
                     full_solve: i32,
                     target_name: *const i8,
                     move_hist: *mut i8, echo:i32, g_state: &mut FullState) {
    let mut dummy_info: EvaluationType =EvaluationType::new();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move = 0;
    let mut outcome: i32 = 0;
    let mut really_evaluate: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut child_count: i32 = 0;
    let mut child_move: [i8; 64] = [0; 64];
    let mut child_node: [i32; 64] = [0; 64];
    if g_state.g_book.evaluated_count >= g_state.g_book.max_eval_count { return }
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    /* First correct the children */
    generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
    child_count = 0;
    i = 0;
    while i < g_state.moves_state.move_count[g_state.moves_state.disks_played as usize] {
        this_move = g_state.moves_state.move_list[g_state.moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_state.g_book);
        child = *g_state.g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            child_move[child_count as usize] = this_move;
            child_node[child_count as usize] = child;
            child_count += 1
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
        };
        i += 1
    }
    let mut current_block_29: u64;
    i = 0;
    while i < child_count {
        if side_to_move == 0 as i32 {
            if g_state.g_book.force_black != 0 &&
                (*g_state.g_book.node.offset(child_node[i as usize] as
                    isize)).black_minimax_score as
                    i32 !=
                    (*g_state.g_book.node.offset(index as isize)).black_minimax_score as
                        i32 {
                current_block_29 = 14818589718467733107;
            } else { current_block_29 = 11913429853522160501; }
        } else if g_state.g_book.force_white != 0 &&
            (*g_state.g_book.node.offset(child_node[i as usize] as
                isize)).white_minimax_score as
                i32 !=
                (*g_state.g_book.node.offset(index as isize)).white_minimax_score
                    as i32 {
            current_block_29 = 14818589718467733107;
        } else { current_block_29 = 11913429853522160501; }
        match current_block_29 {
            11913429853522160501 => {
                this_move = child_move[i as usize];
                sprintf(move_hist.offset((2 as i32 * g_state.moves_state.disks_played) as
                    isize),
                        b"%c%c\x00" as *const u8 as *const i8,
                        'a' as i32 + this_move as i32 % 10 as i32 -
                            1 as i32,
                        '0' as i32 + this_move as i32 / 10 as i32);
                make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
                do_correct(child_node[i as usize] as i32, max_empty, full_solve,
                           target_name, move_hist, echo, g_state);
                let move_0 = this_move;
                {
                    unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
                };
                *move_hist.offset((2 as i32 * g_state.moves_state.disks_played) as isize)
                    = '\u{0}' as i32 as i8
            }
            _ => { }
        }
        i += 1
    }
    /* Then correct the g_state.g_book.node itself (hopefully exploiting lots
     of useful information in the hash table) */
    generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
    determine_hash_values(side_to_move, &g_state.board_state.board, &mut g_state.hash_state);
    if g_state.moves_state.disks_played >= 60 as i32 - max_empty {
        really_evaluate =
            (full_solve != 0 &&
                (*g_state.g_book.node.offset(index as isize)).flags as i32 &
                    16 as i32 == 0 ||
                full_solve == 0 &&
                    (*g_state.g_book.node.offset(index as isize)).flags as i32 &
                        (4 as i32 | 16 as i32) == 0) as
                i32;
        if abs((*g_state.g_book.node.offset(index as isize)).alternative_score as
            i32) < g_state.g_book.min_eval_span ||
            abs((*g_state.g_book.node.offset(index as isize)).alternative_score as
                i32) > g_state.g_book.max_eval_span {
            really_evaluate = 0 as i32
        }
        if abs((*g_state.g_book.node.offset(index as isize)).black_minimax_score as
            i32) < g_state.g_book.min_negamax_span ||
            abs((*g_state.g_book.node.offset(index as isize)).black_minimax_score as
                i32) > g_state.g_book.max_negamax_span {
            really_evaluate = 0 as i32
        }
        if really_evaluate != 0 {
            if target_name.is_null() {
                /* Solve now */
                reset_counter(&mut g_state.search_state.nodes);
               end_game::<osfbook::FE>(side_to_move, (full_solve == 0) as i32,
                                       0 as i32, 1 as i32, 0 as i32,
                                       &mut dummy_info, echo, &mut g_state.flip_stack_
                                       , &mut g_state.search_state
                                       , &mut g_state.board_state
                                       , &mut g_state.hash_state
                                       , &mut g_state.g_timer
                                       , &mut g_state.end_g
                                       , &mut g_state.midgame_state
                                       , &mut g_state.coeff_state
                                       , &mut g_state.moves_state
                                       , &mut g_state.random_instance
                                       , &mut g_state.g_book
                                       , &mut g_state.stable_state
                                       , &mut g_state.prob_cut);
                if side_to_move == 0 as i32 {
                    outcome = g_state.search_state.root_eval
                } else { outcome = -g_state.search_state.root_eval }
                let ref mut fresh31 =
                    (*g_state.g_book.node.offset(index as isize)).white_minimax_score;
                *fresh31 = outcome as i16;
                (*g_state.g_book.node.offset(index as isize)).black_minimax_score = *fresh31;
                if outcome > 0 as i32 {
                    let ref mut fresh32 =
                        (*g_state.g_book.node.offset(index as isize)).black_minimax_score;
                    *fresh32 =
                        (*fresh32 as i32 + 30000 as i32) as
                            i16;
                    let ref mut fresh33 =
                        (*g_state.g_book.node.offset(index as isize)).white_minimax_score;
                    *fresh33 =
                        (*fresh33 as i32 + 30000 as i32) as
                            i16
                }
                if outcome < 0 as i32 {
                    let ref mut fresh34 =
                        (*g_state.g_book.node.offset(index as isize)).black_minimax_score;
                    *fresh34 =
                        (*fresh34 as i32 - 30000 as i32) as
                            i16;
                    let ref mut fresh35 =
                        (*g_state.g_book.node.offset(index as isize)).white_minimax_score;
                    *fresh35 =
                        (*fresh35 as i32 - 30000 as i32) as
                            i16
                }
                if full_solve != 0 {
                    let ref mut fresh36 =
                        (*g_state.g_book.node.offset(index as isize)).flags;
                    *fresh36 =
                        (*fresh36 as i32 | 16 as i32) as
                            u16
                } else {
                    let ref mut fresh37 =
                        (*g_state.g_book.node.offset(index as isize)).flags;
                    *fresh37 =
                        (*fresh37 as i32 | 4 as i32) as
                            u16
                }
            } else {
                /* Defer solving to a standalone scripted solver */
                let target_file: FileHandle =
                    fopen(target_name,
                          b"a\x00" as *const u8 as *const i8);
                if !target_file.is_null() {
                    fprintf(target_file,
                            b"%% %s\n\x00" as *const u8 as
                                *const i8, move_hist);
                    let val0___ = &mut val1;
                    let val1___ = &mut val2;
                    let orientation___ = &mut orientation;
                    get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
                    fprintf(target_file,
                            b"%% %d %d\n\x00" as *const u8 as
                                *const i8, val1, val2);
                    i = 1;
                    while i <= 8 as i32 {
                        j = 1;
                        while j <= 8 as i32 {
                            pos = 10 as i32 * i + j;
                            if g_state.board_state.board[pos as usize] == 0 as i32 {
                                putc('X' as i32, target_file);
                            } else if g_state.board_state.board[pos as usize] == 2 as i32
                            {
                                putc('O' as i32, target_file);
                            } else { putc('-' as i32, target_file); }
                            j += 1
                        }
                        i += 1
                    }
                    if side_to_move == 0 as i32 {
                        fputs(b" X\n\x00" as *const u8 as *const i8,
                              target_file);
                    } else {
                        fputs(b" O\n\x00" as *const u8 as *const i8,
                              target_file);
                    }
                    fputs(b"%\n\x00" as *const u8 as *const i8,
                          target_file);
                    fclose(target_file);
                }
            }
            g_state.g_book.evaluated_count += 1
        }
    }
    if g_state.g_book.evaluated_count >=
        (g_state.g_book.evaluation_stage + 1 as i32) * g_state.g_book.max_eval_count /
            25 as i32 {
        g_state.g_book.evaluation_stage += 1;
        putc('|' as i32, stdout);
        if g_state.g_book.evaluation_stage % 5 as i32 == 0 as i32 {
            libc_wrapper::printf(b" %d%% \x00" as *const u8 as *const i8,
                                 4 as i32 * g_state.g_book.evaluation_stage);
        }
        fflush(stdout);
    }
    let ref mut fresh38 = (*g_state.g_book.node.offset(index as isize)).flags;
    *fresh38 = (*fresh38 as i32 ^ 8 as i32) as u16;
}
static mut correction_script_name: *const i8 = 0 as *const i8;

/*
  SET_OUTPUT_SCRIPT_NAME
  Makes SCRIPT_NAME the target for the positions generated by
  do_correct() (instead of the positions being solved, the normal
  mode of operation).
*/
pub unsafe fn set_output_script_name(script_name:
                                                *const i8) {
    correction_script_name = script_name;
}

/*
   CORRECT_TREE
   Endgame-correct the lowest levels of the tree.
*/
pub unsafe fn correct_tree(max_empty: i32,
                           full_solve: i32, g_state: &mut FullState) {
    let mut move_buffer: [i8; 150] = [0; 150];
    let mut i: i32 = 0;
    let mut feasible_count: i32 = 0;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    osfbook::prepare_tree_traversal(g_state);
    g_state.g_book.exhausted_node_count = 0;
    g_state.g_book.evaluated_count = 0;
    g_state.g_book.evaluation_stage = 0;
    time(&mut start_time);
    i = 0;
    while i < g_state.g_book.book_node_count {
        let ref mut fresh39 = (*g_state.g_book.node.offset(i as isize)).flags;
        *fresh39 =
            (*fresh39 as i32 | 8 as i32) as u16;
        i += 1
    }
    feasible_count = 0;
    i = 0;
    while i < g_state.g_book.book_node_count {
        let ref mut fresh40 = (*g_state.g_book.node.offset(i as isize)).flags;
        *fresh40 =
            (*fresh40 as i32 | 8 as i32) as u16;
        if get_node_depth(i, &mut g_state.g_book) < max_empty &&
            abs((*g_state.g_book.node.offset(i as isize)).alternative_score as
                i32) >= g_state.g_book.min_eval_span &&
            abs((*g_state.g_book.node.offset(i as isize)).alternative_score as
                i32) <= g_state.g_book.max_eval_span &&
            abs((*g_state.g_book.node.offset(i as isize)).black_minimax_score as
                i32) >= g_state.g_book.min_negamax_span &&
            abs((*g_state.g_book.node.offset(i as isize)).black_minimax_score as
                i32) <= g_state.g_book.max_negamax_span {
            feasible_count += 1
        }
        i += 1
    }
    g_state.g_book.max_eval_count =
        if feasible_count < g_state.g_book.max_batch_size {
            feasible_count
        } else { g_state.g_book.max_batch_size };
    libc_wrapper::printf(b"Correcting <= %d empty \x00" as *const u8 as *const i8,
                         max_empty);
    if full_solve != 0 {
        libc_wrapper::printf(b"(full solve). \x00" as *const u8 as *const i8);
    } else {
        libc_wrapper::printf(b"(WLD solve). \x00" as *const u8 as *const i8);
    }
    if g_state.g_book.min_eval_span > 0 as i32 ||
        g_state.g_book.max_eval_span < 1000 as i32 * 128 as i32 {
        libc_wrapper::printf(b"Eval interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const i8,
                             g_state.g_book.min_eval_span as f64 / 128.0f64,
                             g_state.g_book.max_eval_span as f64 / 128.0f64);
    }
    if g_state.g_book.min_negamax_span > 0 as i32 ||
        g_state.g_book.max_negamax_span < 1000 as i32 * 128 as i32 {
        libc_wrapper::printf(b"Negamax interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const i8,
                             g_state.g_book.min_negamax_span as f64 / 128.0f64,
                             g_state.g_book.max_negamax_span as f64 / 128.0f64);
    }
    if g_state.g_book.max_eval_count == feasible_count {
        libc_wrapper::printf(b"\n%d relevant nodes.\x00" as *const u8 as
                   *const i8, feasible_count);
    } else {
        libc_wrapper::printf(b"\nMax batch size is %d.\x00" as *const u8 as
                   *const i8, g_state.g_book.max_batch_size);
    }
    libc_wrapper::puts(b"\x00" as *const u8 as *const i8);
    libc_wrapper::printf(b"Progress: \x00" as *const u8 as *const i8);
    fflush(stdout);
    move_buffer[0] = '\u{0}' as i32 as i8;
    do_correct(0 as i32, max_empty, full_solve,
               correction_script_name, move_buffer.as_mut_ptr(), g_state.g_config.echo, g_state);
    time(&mut stop_time);
    libc_wrapper::printf(b"(took %d s)\n\x00" as *const u8 as *const i8,
                         (stop_time - start_time) as i32);
    if correction_script_name.is_null() {
        /* Positions solved */
        libc_wrapper::printf(b"%d nodes solved\n\x00" as *const u8 as *const i8,
                             g_state.g_book.evaluated_count);
    } else {
        libc_wrapper::printf(b"%d nodes exported to %s\n\x00" as *const u8 as
                   *const i8, g_state.g_book.evaluated_count,
                             correction_script_name);
    }
    libc_wrapper::puts(b"\x00" as *const u8 as *const i8);
}

/*
   DO_EXPORT
   Recursively exports all variations rooted at book position # INDEX.
*/

unsafe fn do_export(index: i32, stream: FileHandle,
                    move_vec: &mut [i32; 60], g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut child_count: i32 = 0;
    let mut allow_branch: i32 = 0;
    let mut side_to_move: i32 = 0;
    allow_branch =
        (*g_state.g_book.node.offset(index as isize)).flags as i32 &
            8 as i32;
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
    child_count = 0;
    i = 0;
    while i < g_state.moves_state.move_count[g_state.moves_state.disks_played as usize] {
        let mut child: i32 = 0;
        let mut slot: i32 = 0;
        let mut val1: i32 = 0;
        let mut val2: i32 = 0;
        let mut orientation: i32 = 0;
        let this_move =
            g_state.moves_state.move_list[g_state.moves_state.disks_played as usize][i as usize];
        *(move_vec.as_mut_ptr()).offset(g_state.moves_state.disks_played as isize) = this_move as i32;
        make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_state.g_book);
        child = *g_state.g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            do_export(child, stream, move_vec, g_state);
            child_count += 1
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
        };
        if child_count == 1 as i32 && allow_branch == 0 { break ; }
        i += 1
    }
    if child_count == 0 as i32 {
        /* We've reached a leaf in the opening tree. */
        i = 0;
        while i < g_state.moves_state.disks_played {
            fprintf(stream, b"%c%c\x00" as *const u8 as *const i8,
                    'a' as i32 +
                        *(move_vec.as_mut_ptr()).offset(i as isize) % 10 as i32 -
                        1 as i32,
                    '0' as i32 +
                        *(move_vec.as_mut_ptr()).offset(i as isize) / 10 as i32);
            i += 1
        }
        fprintf(stream, b"\n\x00" as *const u8 as *const i8);
    }
    let ref mut fresh41 = (*g_state.g_book.node.offset(index as isize)).flags;
    *fresh41 =
        (*fresh41 as i32 & !(8 as i32)) as u16;
}



/*
  EXPORT_TREE
  Exports a set of lines that cover the tree.
*/

pub unsafe fn export_tree(file_name: *const i8, g_state: &mut FullState) {
    let stream = fopen(file_name, b"w\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fprintf(stderr,
                b"Cannot open %s for writing.\n\x00" as *const u8 as
                    *const i8, file_name);
        return
    }
    osfbook::prepare_tree_traversal(g_state);
    let mut move_vec: [i32; 60] = [0; 60];
    let mut i = 0;
    while i < g_state.g_book.book_node_count {
        let ref mut fresh42 = (*g_state.g_book.node.offset(i as isize)).flags;
        *fresh42 =
            (*fresh42 as i32 | 8 as i32) as u16;
        i += 1
    }
    do_export(0 as i32, stream, &mut move_vec, g_state);
    fclose(stream);
}


/*
   MINIMAX_TREE
   Calculates the minimax values of all nodes in the tree.
*/

pub unsafe fn minimax_tree(g_state: &mut FullState) {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    printf(b"Calculating minimax value... \x00" as *const u8 as
        *const i8);
    fflush(stdout);
    prepare_tree_traversal(g_state);
    time(&mut start_time);
    engine_minimax_tree(g_state);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8, stop_time - start_time);
    write!(stdout, "\n");
}

/*
   EVALUATE_TREE
   Finds the most promising deviations from all nodes in the tree.
*/

pub unsafe fn evaluate_tree(g_state: &mut FullState) {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    prepare_tree_traversal(g_state);
    g_state.g_book.exhausted_node_count = 0;
    g_state.g_book.evaluated_count = 0;
    g_state.g_book.evaluation_stage = 0;
    time(&mut start_time);
    let feasible_count = compute_feasible_count(g_state);
    g_state.g_book.max_eval_count =
        if feasible_count < g_state.g_book.max_batch_size {
            feasible_count
        } else { g_state.g_book.max_batch_size };
    printf(b"Evaluating to depth %d. \x00" as *const u8 as
               *const i8, g_state.g_book.search_depth);
    if g_state.g_book.min_eval_span > 0 as i32 ||
        g_state.g_book.max_eval_span < 1000 as i32 * 128 as i32 {
        printf(b"Eval interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const i8,
               g_state.g_book.min_eval_span as f64 / 128.0f64,
               g_state.g_book.max_eval_span as f64 / 128.0f64);
    }
    if g_state.g_book.min_negamax_span > 0 as i32 ||
        g_state.g_book.max_negamax_span < 1000 as i32 * 128 as i32 {
        printf(b"Negamax interval is [%.2f,%.2f]. \x00" as *const u8 as
                   *const i8,
               g_state.g_book.min_negamax_span as f64 / 128.0f64,
               g_state.g_book.max_negamax_span as f64 / 128.0f64);
    }
    if g_state.g_book.max_eval_count == feasible_count {
        printf(b"\n%d relevant nodes.\x00" as *const u8 as
                   *const i8, feasible_count);
    } else {
        printf(b"\nMax batch size is %d.\x00" as *const u8 as
                   *const i8, g_state.g_book.max_batch_size);
    }
    write!(stdout, "\n");
    write!(stdout, "Progress: ");
    fflush(stdout);
    if feasible_count > 0 as i32 { do_evaluate(0 as i32, g_state.g_config.echo, g_state); }
    time(&mut stop_time);
    printf(b"(took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    printf(b"%d nodes evaluated \x00" as *const u8 as *const i8,
           g_state.g_book.evaluated_count);
    printf(b"(%d exhausted nodes ignored)\n\x00" as *const u8 as
               *const i8, g_state.g_book.exhausted_node_count);
    write!(stdout, "\n");
}
/*
   EXAMINE_TREE
   Generates some statistics about the book tree.
*/

pub unsafe fn examine_tree(g_state: &mut FullState) {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    write!(stdout, "Examining tree... ");
    fflush(stdout);
    prepare_tree_traversal(g_state);
    time(&mut start_time);
    engine_examine_tree(g_state);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    write!(stdout, "\n");
}
unsafe extern "C" fn int_compare(i1: *const std::ffi::c_void,
                                 i2: *const std::ffi::c_void) -> i32 {
    return *(i1 as *mut i32) - *(i2 as *mut i32);
}
/*
   BOOK_STATISTICS
   Describe the status of the nodes in the tree.
*/

pub unsafe fn book_statistics(full_statistics: i32, g_state: &mut FullState) {
    let strata: [f64; 11] =
        [0.01f64, 0.02f64, 0.03f64, 0.05f64, 0.10f64, 0.30f64, 0.50f64,
            0.70f64, 0.90f64, 0.99f64, 1.01f64];
    let mut eval_strata: [f64; 10] = [0.; 10];
    let mut negamax_strata: [f64; 10] = [0.; 10];
    let mut i: i32 = 0;
    let mut full_solved: i32 = 0;
    let mut wld_solved: i32 = 0;
    let mut unevaluated: i32 = 0;
    let mut eval_count: i32 = 0;
    let mut negamax_count: i32 = 0;
    let mut private_count: i32 = 0;
    let mut this_strata: i32 = 0;
    let mut strata_shift: i32 = 0;
    let mut first: i32 = 0;
    let mut last: i32 = 0;
    let mut evals = 0 as *mut i32;
    let mut negamax = 0 as *mut i32;
    let mut depth: [i32; 60] = [0; 60];
    let mut total_count: [i32; 61] = [0; 61];
    evals =
        safe_malloc((g_state.g_book.book_node_count as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    negamax =
        safe_malloc((g_state.g_book.book_node_count as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    wld_solved = 0;
    full_solved = wld_solved;
    eval_count = 0;
    negamax_count = 0;
    private_count = 0;
    unevaluated = 0;
    i = 0;
    while i < 60 as i32 {
        depth[i as usize] = 0;
        i += 1
    }
    i = 0;
    while i < g_state.g_book.book_node_count {
        if (*g_state.g_book.node.offset(i as isize)).flags as i32 & 16 as i32
            != 0 {
            full_solved += 1
        } else if (*g_state.g_book.node.offset(i as isize)).flags as i32 &
            4 as i32 != 0 {
            wld_solved += 1
        } else {
            depth[get_node_depth(i, &mut g_state.g_book) as usize] += 1;
            if (*g_state.g_book.node.offset(i as isize)).alternative_score as i32 ==
                9999 as i32 &&
                (*g_state.g_book.node.offset(i as isize)).best_alternative_move as
                    i32 == -(1 as i32) {
                unevaluated += 1
            } else {
                if (*g_state.g_book.node.offset(i as isize)).alternative_score as i32
                    != 9999 as i32 {
                    let fresh24 = eval_count;
                    eval_count = eval_count + 1;
                    *evals.offset(fresh24 as isize) =
                        abs((*g_state.g_book.node.offset(i as isize)).alternative_score as
                            i32)
                }
                let fresh25 = negamax_count;
                negamax_count = negamax_count + 1;
                *negamax.offset(fresh25 as isize) =
                    abs((*g_state.g_book.node.offset(i as isize)).black_minimax_score as
                        i32)
            }
        }
        if (*g_state.g_book.node.offset(i as isize)).flags as i32 & 32 as i32
            != 0 {
            private_count += 1
        }
        i += 1
    }
    qsort(evals as *mut std::ffi::c_void, eval_count as _,
          ::std::mem::size_of::<i32>() as _,
          Some(int_compare as
              unsafe extern "C" fn(_: *const std::ffi::c_void,
                                   _: *const std::ffi::c_void)
                                   -> i32));
    qsort(negamax as *mut std::ffi::c_void, negamax_count as _,
          ::std::mem::size_of::<i32>() as _,
          Some(int_compare as
              unsafe extern "C" fn(_: *const std::ffi::c_void,
                                   _: *const std::ffi::c_void)
                                   -> i32));
    write!(stdout, "\n");
    printf(b"#nodes:       %d\x00" as *const u8 as *const i8,
           g_state.g_book.book_node_count);
    if private_count > 0 as i32 {
        printf(b"  (%d private)\x00" as *const u8 as *const i8,
               private_count);
    }
    write!(stdout, "\n");
    printf(b"#full solved: %d\n\x00" as *const u8 as *const i8,
           full_solved);
    printf(b"#WLD solved:  %d\n\x00" as *const u8 as *const i8,
           wld_solved);
    printf(b"#unevaluated: %d\n\n\x00" as *const u8 as *const i8,
           unevaluated);
    i = 0;
    while i <= 59 as i32 {
        if depth[i as usize] > 0 as i32 {
            printf(b"#nodes with %2d-ply deviations: %d\n\x00" as *const u8 as
                       *const i8, i, depth[i as usize]);
        }
        i += 1
    }
    write!(stdout, "\n");
    this_strata = 0;
    strata_shift =
        floor(strata[this_strata as usize] * eval_count as f64) as
            i32;
    i = 0;
    while i < eval_count {
        if i == strata_shift {
            eval_strata[this_strata as usize] =
                *evals.offset(i as isize) as f64 / 128.0f64;
            this_strata += 1;
            strata_shift =
                floor(strata[this_strata as usize] *
                    eval_count as f64) as i32
        }
        i += 1
    }
    this_strata = 0;
    strata_shift =
        floor(strata[this_strata as usize] * negamax_count as f64)
            as i32;
    i = 0;
    while i < negamax_count {
        if i == strata_shift {
            negamax_strata[this_strata as usize] =
                *evals.offset(i as isize) as f64 / 128.0f64;
            this_strata += 1;
            strata_shift =
                floor(strata[this_strata as usize] *
                    negamax_count as f64) as i32
        }
        i += 1
    }
    i = 0;
    while i < 10 as i32 {
        printf(b"%2.0f%%:  \x00" as *const u8 as *const i8,
               100 as i32 as f64 * strata[i as usize]);
        printf(b"%5.2f   \x00" as *const u8 as *const i8,
               eval_strata[i as usize]);
        printf(b"%5.2f   \x00" as *const u8 as *const i8,
               negamax_strata[i as usize]);
        write!(stdout, "\n");
        i += 1
    }
    write!(stdout, "\n");
    free(negamax as *mut std::ffi::c_void);
    free(evals as *mut std::ffi::c_void);
    if full_statistics != 0 {
        examine_tree(g_state);
        first = 61;
        last = -(1 as i32);
        i = 0;
        while i <= 60 as i32 {
            total_count[i as usize] =
                g_state.g_book.exact_count[i as usize] + g_state.g_book.wld_count[i as usize] +
                    g_state.g_book.exhausted_count[i as usize] + g_state.g_book.common_count[i as usize];
            if total_count[i as usize] > 0 as i32 {
                first = if first < i { first } else { i };
                last = if last > i { last } else { i }
            }
            i += 1
        }
        printf(b"%d unreachable nodes\n\n\x00" as *const u8 as
                   *const i8, g_state.g_book.unreachable_count);
        printf(b"%d leaf nodes; %d lack exact score and %d lack WLD status\n\x00"
                   as *const u8 as *const i8, g_state.g_book.leaf_count,
               g_state.g_book.bad_leaf_count, g_state.g_book.really_bad_leaf_count);
        i = first;
        while i <= last {
            printf(b"%2d moves\x00" as *const u8 as *const i8, i);
            write!(stdout, "   ");
            printf(b"%5d g_state.g_book.node\x00" as *const u8 as *const i8,
                   total_count[i as usize]);
            if total_count[i as usize] == 1 as i32 {
                write!(stdout, " :  ");
            } else {
                write!(stdout, "s:  ");
            }
            if g_state.g_book.common_count[i as usize] > 0 as i32 {
                printf(b"%5d midgame\x00" as *const u8 as *const i8,
                       g_state.g_book.common_count[i as usize]);
            } else {
                printf(b"             \x00" as *const u8 as
                    *const i8);
            }
            write!(stdout, "  ");
            if g_state.g_book.wld_count[i as usize] > 0 as i32 {
                printf(b"%5d WLD\x00" as *const u8 as *const i8,
                       g_state.g_book.wld_count[i as usize]);
            } else {
                write!(stdout, "         ");
            }
            write!(stdout, "  ");
            if g_state.g_book.exact_count[i as usize] > 0 as i32 {
                printf(b"%5d exact\x00" as *const u8 as *const i8,
                       g_state.g_book.exact_count[i as usize]);
            } else {
                printf(b"           \x00" as *const u8 as
                    *const i8);
            }
            write!(stdout, "  ");
            if g_state.g_book.exhausted_count[i as usize] > 0 as i32 {
                printf(b"%2d exhausted\x00" as *const u8 as
                           *const i8, g_state.g_book.exhausted_count[i as usize]);
            }
            write!(stdout, "\n");
            i += 1
        }
        write!(stdout, "\n");
    };
}
/*
   DISPLAY_OPTIMAL_LINE
   Outputs the sequence of moves which is optimal according
   to both players.
*/

pub unsafe fn display_doubly_optimal_line(original_side_to_move:
                                          i32, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut done: i32 = 0;
    let mut show_move: i32 = 0;
    let mut line: i32 = 0;
    let mut root_score: i32 = 0;
    let mut child_score: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut base_orientation: i32 = 0;
    let mut child_orientation: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move = 0;
    let mut current: i32 = 0;
    let mut child: i32 = 0;
    let mut next: i32 = 0;
    prepare_tree_traversal(g_state);
    printf(b"Root evaluation with Zebra playing \x00" as *const u8 as
        *const i8);
    if original_side_to_move == 0 as i32 {
        root_score =
            g_state.g_book.node[0].black_minimax_score as
                i32;
        write!(stdout, "black");
    } else {
        root_score =
            g_state.g_book.node[0].white_minimax_score as
                i32;
        write!(stdout, "white");
    }
    printf(b": %+.2f\n\x00" as *const u8 as *const i8,
           root_score as f64 / 128.0f64);
    current = 0;
    puts(b"Preferred line: \x00" as *const u8 as *const i8);
    line = 0;
    done = 0;
    show_move = 1;
    while (*g_state.g_book.node.offset(current as isize)).flags as i32 &
        (16 as i32 | 4 as i32) == 0 && done == 0 {
        if (*g_state.g_book.node.offset(current as isize)).flags as i32 &
            1 as i32 != 0 {
            side_to_move = 0 as i32
        } else { side_to_move = 2 as i32 }
        generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
        next = -(1 as i32);
        this_move = -1;
        i = 0;
        while i < g_state.moves_state.move_count[g_state.moves_state.disks_played as usize] {
            let val0___ = &mut val1;
            let val1___ = &mut val2;
            let orientation___ = &mut base_orientation;
            get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
            this_move = g_state.moves_state.move_list[g_state.moves_state.disks_played as usize][i as usize];
            make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
            let val0___ = &mut val1;
            let val1___ = &mut val2;
            let orientation___ = &mut child_orientation;
            get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
            slot = probe_hash_table(val1, val2, &mut g_state.g_book);
            child = *g_state.g_book.book_hash_table.offset(slot as isize);
            if child != -(1 as i32) {
                if original_side_to_move == 0 as i32 {
                    child_score =
                        (*g_state.g_book.node.offset(child as isize)).black_minimax_score as
                            i32
                } else {
                    child_score =
                        (*g_state.g_book.node.offset(child as isize)).white_minimax_score as
                            i32
                }
                if child_score == root_score { next = child }
            }
            if child != -(1 as i32) && next == child { break ; }
            let move_0 = this_move;
            {
                unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
            };
            i += 1
        }
        if next == -(1 as i32) {
            done = 1;
            if adjust_score((*g_state.g_book.node.offset(current as isize)).alternative_score
                                as i32, side_to_move, &mut g_state.g_book, g_state.moves_state.disks_played) != root_score {
                puts(b"(failed to find continuation)\x00" as *const u8 as
                    *const i8);
                show_move = 0 as i32
            } else {
                this_move =
                    (*g_state.g_book.node.offset(current as isize)).best_alternative_move as
                        i8;
                this_move =
                    *g_state.g_book.inv_symmetry_map[base_orientation as
                        usize].offset(this_move as isize) as _
            }
        }
        if show_move != 0 {
            if side_to_move == 0 as i32 {
                line += 1;
                printf(b"%2d. \x00" as *const u8 as *const i8,
                       line);
            }
            printf(b"%c%c  \x00" as *const u8 as *const i8,
                   'a' as i32 + this_move as i32 % 10 as i32 -
                       1 as i32,
                   '0' as i32 + this_move as i32 / 10 as i32);
            if side_to_move == 2 as i32 {
                write!(stdout, "\n");
            }
            if done != 0 {
                puts(b"(deviation)\x00" as *const u8 as *const i8);
            }
        }
        current = next
    }
    write!(stdout, "\n");
}

/*
   BUILD_TREE
   Reads games from the file pointed to by FILE_NAME and
   incorporates them into the game tree.
*/

pub unsafe fn build_tree(file_name: *const i8,
                         max_game_count: i32,
                         max_diff: i32,
                         min_empties: i32, echo:i32, g_state: &mut FullState) {
    let mut move_string: [i8; 200] = [0; 200];
    let mut line_buffer: [i8; 1000] = [0; 1000];
    let mut sign: i8 = 0;
    let mut column: i8 = 0;
    let mut row: i8 = 0;
    let mut i: i32 = 0;
    let mut games_parsed: i32 = 0;
    let mut games_imported: i32 = 0;
    let mut move_count_0: i32 = 0;
    let mut diff: i32 = 0;
    let mut game_move_list: [i16; 60] = [0; 60];
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut stream = FileHandle::null();
    puts(b"Importing game list...\x00" as *const u8 as *const i8);
    fflush(stdout);
    stream = fopen(file_name, b"r\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error!("{} '{}'\n", "Could not open game file", &CStr::from_ptr(file_name).to_str().unwrap());
    }
    time(&mut start_time);
    games_parsed = 0;
    games_imported = 0;
    loop  {
        fgets(line_buffer.as_mut_ptr(), 998 as i32, stream);
        sscanf(line_buffer.as_mut_ptr(),
               b"%s %d\x00" as *const u8 as *const i8,
               move_string.as_mut_ptr(), &mut diff as *mut i32);
        move_count_0 = strlen(move_string.as_mut_ptr()).wrapping_sub(1).wrapping_div(3) as i32;
        games_parsed += 1;
        i = 0;
        while i < move_count_0 {
            sscanf(move_string.as_mut_ptr().offset((3 as i32 * i) as
                isize),
                   b"%c%c%c\x00" as *const u8 as *const i8,
                   &mut sign as *mut i8,
                   &mut column as *mut i8,
                   &mut row as *mut i8);
            game_move_list[i as usize] =
                (10 as i32 * (row as i32 - '0' as i32) +
                    (column as i32 - 'a' as i32 + 1 as i32))
                    as i16;
            if sign as i32 == '-' as i32 {
                game_move_list[i as usize] =
                    -(game_move_list[i as usize] as i32) as
                        i16
            }
            i += 1
        }
        if abs(diff) <= max_diff {
            add_new_game(move_count_0, Some(&game_move_list),
                         min_empties, 0 as i32, 0 as i32,
                         0 as i32, 0 as i32, echo, g_state);
            write!(stdout, "|");
            if games_imported % 100 as i32 == 0 as i32 {
                printf(b" --- %d games --- \x00" as *const u8 as
                           *const i8, games_imported);
            }
            fflush(stdout);
            games_imported += 1
        }
        if !(games_parsed < max_game_count) { break ; }
    }
    time(&mut stop_time);
    fclose(stream);
    printf(b"\ndone (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    printf(b"%d games read; %d games imported\n\x00" as *const u8 as
               *const i8, games_parsed, games_imported);
    printf(b"Games with final difference <= %d were read until %d empties.\n\x00"
               as *const u8 as *const i8, max_diff, min_empties);
    write!(stdout, "\n");
}

/*
  DUPSTR
  A strdup clone.
*/
unsafe fn dupstr(str: *const i8) -> *mut i8 {
    let new_str = malloc(strlen(str).wrapping_add(1)) as *mut i8;
    strcpy(new_str, str);
    return new_str;
}
/*
  CONVERT_OPENING_LIST
  Convert a list of openings on Robert Gatliff's format
  to a hash table representation containing the same information.
*/

pub unsafe fn convert_opening_list(base_file:
                                   *const i8, g_state: &mut FullState) {
    let mut in_stream =
        FileHandle::null(); /* Max number of opening names occurring */
    let mut out_stream = FileHandle::null();
    let mut name_start = 0 as *mut i8;
    let mut scan_ptr = 0 as *mut i8;
    let mut move_ptr = 0 as *mut i8;
    let mut source_file_name = 0 as *const i8;
    let mut header_file_name = 0 as *const i8;
    let mut parent: [*mut i8; 1000] =
        [0 as *mut i8; 1000];
    let mut buffer: [i8; 1024] = [0; 1024];
    let mut move_seq: [i8; 256] = [0; 256];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut opening_count: i32 = 0;
    let mut op_move_count: i32 = 0;
    let mut level: i32 = 0;
    let mut hash_val1: i32 = 0;
    let mut hash_val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut op_move: [i8; 60] = [0; 60];
    let mut side_to_move: [i32; 60] = [0; 60];
    let mut timer: time_t = 0;
    in_stream =
        fopen(base_file, b"r\x00" as *const u8 as *const i8);
    if in_stream.is_null() {
        printf(b"Cannot open opening file \'%s\'\n\x00" as *const u8 as
                   *const i8, base_file);
        exit(1 as i32);
    }
    /* Get the number of openings */
    fgets(buffer.as_mut_ptr(), 1023 as i32, in_stream);
    sscanf(buffer.as_mut_ptr(), b"%d\x00" as *const u8 as *const i8,
           &mut opening_count as *mut i32);
    /* Prepare the header file */
    header_file_name = b"opname.h\x00" as *const u8 as *const i8;
    out_stream =
        fopen(header_file_name, b"w\x00" as *const u8 as *const i8);
    if out_stream.is_null() {
        printf(b"Cannot create header file \'%s\'\n\x00" as *const u8 as
                   *const i8, header_file_name);
        exit(1 as i32);
    }
    time(&mut timer);
    fprintf(out_stream, b"/*\n\x00" as *const u8 as *const i8);
    fprintf(out_stream, b"   %s\n\n\x00" as *const u8 as *const i8,
            header_file_name);
    fprintf(out_stream,
            b"   Automatically created by BOOKTOOL on %s\x00" as *const u8 as
                *const i8, ctime(&mut timer));
    fprintf(out_stream, b"*/\x00" as *const u8 as *const i8);
    fprintf(out_stream, b"\n\n\n\x00" as *const u8 as *const i8);
    fputs(b"#ifndef OPNAME_H\n\x00" as *const u8 as *const i8,
          out_stream);
    fputs(b"#define OPNAME_H\n\n\n\x00" as *const u8 as *const i8,
          out_stream);
    fprintf(out_stream,
            b"#define OPENING_COUNT       %d\n\n\n\x00" as *const u8 as
                *const i8, opening_count);
    fputs(b"typedef struct {\n\x00" as *const u8 as *const i8,
          out_stream);
    fputs(b"  const char *name;\n\x00" as *const u8 as *const i8,
          out_stream);
    fputs(b"  const char *sequence;\n\x00" as *const u8 as
              *const i8, out_stream);
    fputs(b"  int hash_val1;\n\x00" as *const u8 as *const i8,
          out_stream);
    fputs(b"  int hash_val2;\n\x00" as *const u8 as *const i8,
          out_stream);
    fputs(b"  int level;\n\x00" as *const u8 as *const i8,
          out_stream);
    fputs(b"} OpeningDescriptor;\n\n\n\x00" as *const u8 as
              *const i8, out_stream);
    fputs(b"extern OpeningDescriptor opening_list[OPENING_COUNT];\n\x00" as
              *const u8 as *const i8, out_stream);
    fputs(b"\n\n#endif  /* OPNAME_H */\n\x00" as *const u8 as
              *const i8, out_stream);
    fclose(out_stream);
    /* Prepare the source file */
    source_file_name = b"opname.c\x00" as *const u8 as *const i8;
    out_stream =
        fopen(source_file_name, b"w\x00" as *const u8 as *const i8);
    if out_stream.is_null() {
        printf(b"Cannot create source file \'%s\'\n\x00" as *const u8 as
                   *const i8, source_file_name);
        exit(1 as i32);
    }
    time(&mut timer);
    fprintf(out_stream, b"/*\n\x00" as *const u8 as *const i8);
    fprintf(out_stream, b"   %s\n\n\x00" as *const u8 as *const i8,
            source_file_name);
    fprintf(out_stream,
            b"   Automatically created by BOOKTOOL on %s\x00" as *const u8 as
                *const i8, ctime(&mut timer));
    fprintf(out_stream, b"*/\x00" as *const u8 as *const i8);
    fprintf(out_stream, b"\n\n\n\x00" as *const u8 as *const i8);
    fprintf(out_stream,
            b"#include \"%s\"\n\n\n\x00" as *const u8 as *const i8,
            header_file_name);
    fputs(b"OpeningDescriptor opening_list[OPENING_COUNT] = {\n\x00" as
              *const u8 as *const i8, out_stream);
    /* Read the list of openings */
    prepare_tree_traversal(g_state);
    level = 0;
    i = 0;
    while i < opening_count {
        fgets(buffer.as_mut_ptr(), 1023 as i32, in_stream);
        /* Each line in the input file corresponds to one opening.
           First separate the line into opening moves and name. */
        sscanf(buffer.as_mut_ptr(),
               b"%s\x00" as *const u8 as *const i8,
               move_seq.as_mut_ptr());
        name_start =
            buffer.as_mut_ptr().offset(strlen(move_seq.as_mut_ptr()) as
                isize);
        while *(*__ctype_b_loc()).offset(*name_start as i32 as isize)
            as i32 &
            _ISspace as i32 as u16 as i32 !=
            0 {
            name_start = name_start.offset(1)
        }
        scan_ptr = name_start;
        while *(*__ctype_b_loc()).offset(*scan_ptr as i32 as isize) as
            i32 &
            _ISprint as i32 as u16 as i32 !=
            0 {
            scan_ptr = scan_ptr.offset(1)
        }
        *scan_ptr = 0;
        op_move_count = strlen(move_seq.as_mut_ptr()).wrapping_div(2) as i32;
        j = 0;
        move_ptr = buffer.as_mut_ptr();
        while j < op_move_count {
            if *(*__ctype_b_loc()).offset(*move_ptr as i32 as isize)
                as i32 &
                _ISupper as i32 as u16 as i32 !=
                0 {
                side_to_move[j as usize] = 0 as i32
            } else { side_to_move[j as usize] = 2 as i32 }
            col =
                toupper(*move_ptr as i32) - 'A' as i32 +
                    1 as i32;
            move_ptr = move_ptr.offset(1);
            row = *move_ptr as i32 - '0' as i32;
            move_ptr = move_ptr.offset(1);
            op_move[j as usize] = (10 * row + col) as i8;
            j += 1
        }
        /* Check out how the relation between this openings and the ones
           in the hierachy created to far */
        while level > 0 as i32 &&
            strstr(move_seq.as_mut_ptr(),
                   parent[(level - 1 as i32) as usize]) !=
                move_seq.as_mut_ptr() {
            level -= 1;
            free(parent[level as usize] as *mut std::ffi::c_void);
        }
        parent[level as usize] = dupstr(move_seq.as_mut_ptr());
        level += 1;
        /* Create the board position characteristic for the opening. */
        j = 0;
        while j < op_move_count {
            if generate_specific(op_move[j as usize],
                                 side_to_move[j as usize], &g_state.board_state.board) == 0 {
                printf(b"Move %d in opening #%d is illegal\n\x00" as *const u8
                           as *const i8, j + 1 as i32, i);
                exit(1 as i32);
            }
            make_move(side_to_move[j as usize], op_move[j as usize],
                      1 as i32, &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
            j += 1
        }
        /* Write the code fragment  */
        let val0___ = &mut hash_val1;
        let val1___ = &mut hash_val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
        fprintf(out_stream,
                b"   { \"%s\",\n     \"%s\",\n     %d, %d, %d }\x00" as
                    *const u8 as *const i8, name_start,
                move_seq.as_mut_ptr(), hash_val1, hash_val2,
                level - 1 as i32);
        if i != opening_count - 1 as i32 {
            fputs(b" ,\n\x00" as *const u8 as *const i8,
                  out_stream);
        }
        /* Undo the moves */
        j = op_move_count - 1 as i32;
        while j >= 0 as i32 {
            let side_to_move_argument = side_to_move[j as usize];
            let move_0 = op_move[j as usize];
            {
                unmake_move(side_to_move_argument, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
            };
            j -= 1
        }
        i += 1
    }
    fputs(b"\n};\n\x00" as *const u8 as *const i8, out_stream);
    /* Remove the hierarchy data */
    while level > 0 as i32 {
        level -= 1;
        free(parent[level as usize] as *mut std::ffi::c_void);
    }
    fclose(out_stream);
    fclose(in_stream);
}


/*
   WRITE_COMPRESSED_DATABASE
   Creates and saves a compressed database file.
*/

pub unsafe fn write_compressed_database(file_name:
                                        *const i8, g_state: &mut FullState) {
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    time(&mut start_time);
    write!(stdout, "Writing compressed database... ");
    fflush(stdout);
    let stream = fopen(file_name, b"wb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error!("{} '{}'\n", "Could not create database file", &CStr::from_ptr(file_name).to_str().unwrap());
    }
    prepare_tree_traversal(g_state);
    let node_order =
        safe_malloc((g_state.g_book.book_node_count as
            u64).wrapping_mul(::std::mem::size_of::<i32>()
            as u64)) as
            *mut i32;
    let child_count =
        safe_malloc((g_state.g_book.book_node_count as
            u64).wrapping_mul(::std::mem::size_of::<i16>()
            as u64)) as
            *mut i16;
    let child =
        malloc((g_state.g_book.book_node_count as usize).wrapping_mul(::std::mem::size_of::<i16>())) as
            *mut i16;
    let mut i = 0;
    while i < g_state.g_book.book_node_count {
        let ref mut fresh45 = (*g_state.g_book.node.offset(i as isize)).flags;
        *fresh45 =
            (*fresh45 as i32 | 8 as i32) as u16;
        i += 1
    }
    let mut node_index = 0;
    let mut child_index = 0;
    do_compress(0 as i32, node_order, child_count, &mut node_index,
                child, &mut child_index, g_state);
    fwrite(&mut g_state.g_book.book_node_count as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut child_index as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    fwrite(child_count as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           g_state.g_book.book_node_count as size_t, stream);
    fwrite(child as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           child_index as size_t, stream);
    i = 0;
    while i < g_state.g_book.book_node_count {
        fwrite(&mut (*g_state.g_book.node.offset(*node_order.offset(i as isize) as
            isize)).black_minimax_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        fwrite(&mut (*g_state.g_book.node.offset(*node_order.offset(i as isize) as
            isize)).white_minimax_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        i += 1
    }
    i = 0;
    while i < g_state.g_book.book_node_count {
        fwrite(&mut (*g_state.g_book.node.offset(*node_order.offset(i as isize) as
            isize)).best_alternative_move as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        i += 1
    }
    i = 0;
    while i < g_state.g_book.book_node_count {
        fwrite(&mut (*g_state.g_book.node.offset(*node_order.offset(i as isize) as
            isize)).alternative_score as
                   *mut i16 as *const std::ffi::c_void,
               ::std::mem::size_of::<i16>() as u64,
               1 as i32 as size_t, stream);
        i += 1
    }
    i = 0;
    while i < g_state.g_book.book_node_count {
        fwrite(&mut (*g_state.g_book.node.offset(*node_order.offset(i as isize) as
            isize)).flags as *mut u16 as
                   *const std::ffi::c_void,
               ::std::mem::size_of::<u16>() as u64,
               1 as i32 as size_t, stream);
        i += 1
    }
    fclose(stream);
    free(node_order as *mut std::ffi::c_void);
    free(child_count as *mut std::ffi::c_void);
    free(child as *mut std::ffi::c_void);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    write!(stdout, "\n");
}
/*
  DO_UNCOMPRESS
  Uncompress the subtree below the current g_state.g_book.node. This is done
  in preorder.
*/
unsafe fn do_uncompress(depth: i32,
                        stream: FileHandle,
                        node_index: *mut i32,
                        child_index: *mut i32,
                        child_count: *mut i16,
                        child: *mut i16,
                        black_score: *mut i16,
                        white_score: *mut i16,
                        alt_move: *mut i16,
                        alt_score: *mut i16,
                        flags: *mut u16, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut saved_child_index: i32 = 0;
    let mut saved_child_count: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut this_move = 0;
    if *flags.offset(*node_index as isize) as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    saved_child_count =
        *child_count.offset(*node_index as isize) as i32;
    saved_child_index = *child_index;
    *child_index += saved_child_count;
    /* Write the data for the current node */
    let val0___ = &mut val1;
    let val1___ = &mut val2;
    let orientation___ = &mut orientation;
    get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
    fwrite(&mut val1 as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut val2 as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut *black_score.offset(*node_index as isize) as
               *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut *white_score.offset(*node_index as isize) as
               *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut *alt_move.offset(*node_index as isize) as *mut i16
               as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut *alt_score.offset(*node_index as isize) as *mut i16
               as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut *flags.offset(*node_index as isize) as *mut u16 as
               *const std::ffi::c_void,
           ::std::mem::size_of::<u16>() as u64,
           1 as i32 as size_t, stream);
    *node_index += 1;
    /* Recursively traverse the children */
    i = 0;
    while i < saved_child_count {
        let mut flipped: i32 = 0;
        this_move = *child.offset((saved_child_index + i) as isize) as i8;
        flipped = make_move_no_hash(side_to_move, this_move, &mut g_state.board_state, &mut g_state.moves_state, &mut g_state.flip_stack_ );
        if flipped == 0 as i32 {
            printf(b"%c%c flips %d discs for %d\n\x00" as *const u8 as
                       *const i8,
                   'a' as i32 + this_move as i32 % 10 as i32 -
                       1 as i32,
                   '0' as i32 + this_move as i32 / 10 as i32, flipped,
                   side_to_move);
        }
        do_uncompress(depth + 1 as i32, stream, node_index,
                      child_index, child_count, child, black_score,
                      white_score, alt_move, alt_score, flags, g_state);
        let side_to_move___unmake_move_no_hash = side_to_move;
        let move_0___unmake_move_no_hash = this_move;
        {
            unmake_move_no_hash(side_to_move___unmake_move_no_hash, move_0___unmake_move_no_hash, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.flip_stack_);
        };
        i += 1
    };
}
/*
  UNPACK_COMPRESSED_DATABASE
  Reads a database compressed with WRITE_COMPRESSED_DATABASE
  and unpacks it into an ordinary .bin file.
*/

pub unsafe fn unpack_compressed_database(in_name:
                                         *const i8,
                                         out_name:
                                         *const i8, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut dummy: i32 = 0;
    let mut node_count: i32 = 0;
    let mut child_list_size: i32 = 0;
    let mut node_index: i32 = 0;
    let mut child_index: i32 = 0;
    let mut magic: i16 = 0;
    let mut child_count = 0 as *mut i16;
    let mut child = 0 as *mut i16;
    let mut black_score = 0 as *mut i16;
    let mut white_score = 0 as *mut i16;
    let mut alt_move = 0 as *mut i16;
    let mut alt_score = 0 as *mut i16;
    let mut start_time: time_t = 0;
    let mut stop_time: time_t = 0;
    let mut flags = 0 as *mut u16;
    let mut stream = FileHandle::null();
    printf(b"Uncompressing compressed database... \x00" as *const u8 as
        *const i8);
    fflush(stdout);
    time(&mut start_time);
    /* Read the compressed database */
    stream = fopen(in_name, b"rb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error!("{} '{}'\n", "Could not open database file", &CStr::from_ptr(in_name).to_str().unwrap());
    }
    fread(&mut node_count as *mut i32 as *mut std::ffi::c_void,
          ::std::mem::size_of::<i32>() as u64,
          1 as i32 as size_t, stream);
    fread(&mut child_list_size as *mut i32 as *mut std::ffi::c_void,
          ::std::mem::size_of::<i32>() as u64,
          1 as i32 as size_t, stream);
    child_count =
        safe_malloc((node_count as
            u64).wrapping_mul(::std::mem::size_of::<i16>()
            as u64)) as
            *mut i16;
    child =
        safe_malloc((child_list_size as
            u64).wrapping_mul(::std::mem::size_of::<i16>()
            as u64)) as
            *mut i16;
    fread(child_count as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          node_count as size_t, stream);
    fread(child as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          child_list_size as size_t, stream);
    black_score =
        safe_malloc((node_count as
            u64).wrapping_mul(::std::mem::size_of::<i16>()
            as u64)) as
            *mut i16;
    white_score =
        safe_malloc((node_count as
            u64).wrapping_mul(::std::mem::size_of::<i16>()
            as u64)) as
            *mut i16;
    alt_move =
        safe_malloc((node_count as
            u64).wrapping_mul(::std::mem::size_of::<i16>()
            as u64)) as
            *mut i16;
    alt_score =
        safe_malloc((node_count as
            u64).wrapping_mul(::std::mem::size_of::<i16>()
            as u64)) as
            *mut i16;
    flags =
        safe_malloc((node_count as
            u64).wrapping_mul(::std::mem::size_of::<u16>()
            as u64)) as
            *mut u16;
    i = 0;
    while i < node_count {
        fread(&mut *black_score.offset(i as isize) as *mut i16 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        fread(&mut *white_score.offset(i as isize) as *mut i16 as
                  *mut std::ffi::c_void,
              ::std::mem::size_of::<i16>() as u64,
              1 as i32 as size_t, stream);
        i += 1
    }
    fread(alt_move as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          node_count as size_t, stream);
    fread(alt_score as *mut std::ffi::c_void,
          ::std::mem::size_of::<i16>() as u64,
          node_count as size_t, stream);
    fread(flags as *mut std::ffi::c_void,
          ::std::mem::size_of::<u16>() as u64,
          node_count as size_t, stream);
    fclose(stream);
    /* Traverse the tree described by the database and create the .bin file */
    stream = fopen(out_name, b"wb\x00" as *const u8 as *const i8);
    if stream.is_null() {
        fatal_error!("{} '{}'\n", "Could not create database file", &CStr::from_ptr(out_name).to_str().unwrap());
    }
    game_init(0 as *const i8, &mut dummy,g_state);
    g_state.midgame_state.toggle_midgame_hash_usage(1 as i32, 1 as i32);
    g_state.g_timer.toggle_abort_check(0 as i32);
    g_state.midgame_state.toggle_midgame_abort_check(0 as i32);
    magic = 2718;
    fwrite(&mut magic as *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    magic = 2818;
    fwrite(&mut magic as *mut i16 as *const std::ffi::c_void,
           ::std::mem::size_of::<i16>() as u64,
           1 as i32 as size_t, stream);
    fwrite(&mut node_count as *mut i32 as *const std::ffi::c_void,
           ::std::mem::size_of::<i32>() as u64,
           1 as i32 as size_t, stream);
    node_index = 0;
    child_index = 0;
    do_uncompress(0 as i32, stream, &mut node_index, &mut child_index,
                  child_count, child, black_score, white_score, alt_move,
                  alt_score, flags, g_state);
    fclose(stream);
    /* Free tables */
    free(child_count as *mut std::ffi::c_void);
    free(child as *mut std::ffi::c_void);
    free(black_score as *mut std::ffi::c_void);
    free(white_score as *mut std::ffi::c_void);
    free(alt_move as *mut std::ffi::c_void);
    free(alt_score as *mut std::ffi::c_void);
    free(flags as *mut std::ffi::c_void);
    time(&mut stop_time);
    printf(b"done (took %d s)\n\x00" as *const u8 as *const i8,
           (stop_time - start_time) as i32);
    write!(stdout, "\n");
}
/*
  MERGE_POSITION_LIST
  Adds the scores from the positions defined in SCRIPT_FILE and solved
  in OUTPUT_FILE to the book.  The two files are checked for sanity -
  if they don't describe the same set of positions, something has gone awry.
*/

pub unsafe fn merge_position_list<FE: FrontEnd>(script_file:
                                                *const i8,
                                                output_file:
                                                *const i8, g_state: &mut FullState) {
    let mut script_buffer: [i8; 1024] = [0; 1024];
    let mut result_buffer: [i8; 1024] = [0; 1024];
    let mut move_buffer: [i8; 1024] = [0; 1024];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pos: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut col: i32 = 0;
    let mut line: i32 = 0;
    let mut score: i32 = 0;
    let mut move_0 = 0;
    let mut wld_only: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut slot: i32 = 0;
    let mut index: i32 = 0;
    let mut position_count: i32 = 0;
    let mut already_wld_count: i32 = 0;
    let mut already_exact_count: i32 = 0;
    let mut tokens_read: i32 = 0;
    let mut moves_read: i32 = 0;
    let mut new_nodes_created: i32 = 0;
    let mut probable_error: i32 = 0;
    let mut script_stream = FileHandle::null();
    let mut result_stream = FileHandle::null();
    script_stream =
        fopen(script_file, b"r\x00" as *const u8 as *const i8);
    if script_stream.is_null() {
        fprintf(stderr,
                b"Can\'t open %s\n\x00" as *const u8 as *const i8,
                script_file);
        exit(1 as i32);
    }
    result_stream =
        fopen(output_file, b"r\x00" as *const u8 as *const i8);
    if result_stream.is_null() {
        fprintf(stderr,
                b"Can\'t open %s\n\x00" as *const u8 as *const i8,
                output_file);
        exit(1 as i32);
    }
    prepare_tree_traversal(g_state);
    line = 1;
    position_count = 0;
    already_wld_count = 0;
    already_exact_count = 0;
    new_nodes_created = 0;
    fgets(script_buffer.as_mut_ptr(), 1024 as i32, script_stream);
    fgets(result_buffer.as_mut_ptr(), 1024 as i32, result_stream);
    while feof(script_stream) == 0 && feof(result_stream) == 0 {
        let mut ch = 0 as *mut i8;
        ch =
            script_buffer.as_mut_ptr().offset(strlen(script_buffer.as_mut_ptr())
                as
                isize).offset(-(1 as
                i32
                as
                isize));
        while ch >= script_buffer.as_mut_ptr() &&
            *(*__ctype_b_loc()).offset(*ch as i32 as isize) as
                i32 &
                _ISgraph as i32 as u16 as i32
                == 0 {
            *ch = 0;
            ch = ch.offset(-1)
        }
        ch =
            result_buffer.as_mut_ptr().offset(strlen(result_buffer.as_mut_ptr())
                as
                isize).offset(-(1 as
                i32
                as
                isize));
        while ch >= result_buffer.as_mut_ptr() &&
            *(*__ctype_b_loc()).offset(*ch as i32 as isize) as
                i32 &
                _ISgraph as i32 as u16 as i32
                == 0 {
            *ch = 0;
            ch = ch.offset(-1)
        }
        if line % 4 as i32 == 3 as i32 {
            /* The position/result lines */
            position_count += 1;
            /* Parse the board */
            g_state.moves_state.disks_played = 0; /* The initial board contains 4 discs */
            col = 0;
            i = 1;
            while i <= 8 as i32 {
                j = 1;
                while j <= 8 as i32 {
                    pos = 10 as i32 * i + j;
                    match script_buffer[col as usize] as i32 {
                        42 | 88 | 120 => {
                            g_state.board_state.board[pos as usize] = 0;
                            g_state.moves_state.disks_played += 1
                        }
                        79 | 48 | 111 => {
                            g_state.board_state.board[pos as usize] = 2;
                            g_state.moves_state.disks_played += 1
                        }
                        45 | 46 => { g_state.board_state.board[pos as usize] = 1 as i32 }
                        _ => {
                            fprintf(stderr,
                                    b"\nBad character \'%c\' in board on line %d\n\n\x00"
                                        as *const u8 as *const i8,
                                    script_buffer[col as usize] as
                                        i32, line);
                            exit(1 as i32);
                        }
                    }
                    col += 1;
                    j += 1
                }
                i += 1
            }
            match script_buffer[65] as i32 {
                42 | 88 | 120 => { side_to_move = 0 as i32 }
                79 | 48 | 111 => { side_to_move = 2 as i32 }
                _ => {
                    fprintf(stderr,
                            b"\nBad side to move \'%c\' in board on line %d\n\n\x00"
                                as *const u8 as *const i8,
                            script_buffer[65] as
                                i32, line);
                    exit(1 as i32);
                }
            }
            g_state.moves_state.disks_played -= 4 as i32;
            /* Parse the result */
            wld_only = 1;
            if strstr(result_buffer.as_mut_ptr(),
                      b"Black win\x00" as *const u8 as *const i8) ==
                result_buffer.as_mut_ptr() {
                score = 30000 as i32 + 2 as i32;
                tokens_read =
                    sscanf(result_buffer.as_mut_ptr(),
                           b"%*s %*s %s\x00" as *const u8 as
                               *const i8, move_buffer.as_mut_ptr());
                moves_read = tokens_read
            } else if strstr(result_buffer.as_mut_ptr(),
                             b"White win\x00" as *const u8 as
                                 *const i8) ==
                result_buffer.as_mut_ptr() {
                score = -(30000 as i32 + 2 as i32);
                tokens_read =
                    sscanf(result_buffer.as_mut_ptr(),
                           b"%*s %*s %s\x00" as *const u8 as
                               *const i8, move_buffer.as_mut_ptr());
                moves_read = tokens_read
            } else if strstr(result_buffer.as_mut_ptr(),
                             b"Draw\x00" as *const u8 as *const i8)
                == result_buffer.as_mut_ptr() {
                score = 0;
                tokens_read =
                    sscanf(result_buffer.as_mut_ptr(),
                           b"%*s %s\x00" as *const u8 as *const i8,
                           move_buffer.as_mut_ptr());
                moves_read = tokens_read
            } else {
                /* Exact score */
                let mut black_discs: i32 = 0;
                let mut white_discs: i32 = 0;
                wld_only = 0;
                tokens_read =
                    sscanf(result_buffer.as_mut_ptr(),
                           b"%d %*s %d %s\x00" as *const u8 as
                               *const i8,
                           &mut black_discs as *mut i32,
                           &mut white_discs as *mut i32,
                           move_buffer.as_mut_ptr());
                moves_read = tokens_read - 2 as i32;
                score = black_discs - white_discs;
                if score > 0 as i32 {
                    score += 30000 as i32
                } else if score < 0 as i32 {
                    score -= 30000 as i32
                }
            }
            /* Set the score for the g_state.g_book.node corresponding to the position */
            let val0___ = &mut val1;
            let val1___ = &mut val2;
            let orientation___ = &mut orientation;
            get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
            slot = probe_hash_table(val1, val2, &mut g_state.g_book);
            index = *g_state.g_book.book_hash_table.offset(slot as isize);
            if index == -(1 as i32) {
                fprintf(stderr,
                        b"Position on line %d not found in book\n\x00" as
                            *const u8 as *const i8, line);
                exit(0 as i32);
            }
            probable_error = 0;
            if (*g_state.g_book.node.offset(index as isize)).flags as i32 &
                4 as i32 != 0 {
                already_wld_count += 1;
                if score > 0 as i32 &&
                    (*g_state.g_book.node.offset(index as isize)).black_minimax_score as
                        i32 <= 0 as i32 ||
                    score == 0 as i32 &&
                        (*g_state.g_book.node.offset(index as isize)).black_minimax_score
                            as i32 != 0 as i32 ||
                    score < 0 as i32 &&
                        (*g_state.g_book.node.offset(index as isize)).black_minimax_score
                            as i32 > 0 as i32 {
                    probable_error = 1;
                    fprintf(stderr,
                            b"Line %d: New WLD score %d conflicts with old score %d\n\x00"
                                as *const u8 as *const i8, line,
                            score,
                            (*g_state.g_book.node.offset(index as isize)).black_minimax_score
                                as i32);
                }
            }
            if (*g_state.g_book.node.offset(index as isize)).flags as i32 &
                16 as i32 != 0 {
                already_exact_count += 1;
                if wld_only == 0 &&
                    score !=
                        (*g_state.g_book.node.offset(index as isize)).black_minimax_score
                            as i32 {
                    probable_error = 1;
                    fprintf(stderr,
                            b"Line %d: New exact score %d conflicts with old score %d\n\x00"
                                as *const u8 as *const i8, line,
                            score,
                            (*g_state.g_book.node.offset(index as isize)).black_minimax_score
                                as i32);
                }
            }
            if probable_error != 0 || wld_only == 0 ||
                (*g_state.g_book.node.offset(index as isize)).flags as i32 &
                    16 as i32 == 0 {
                let ref mut fresh46 =
                    (*g_state.g_book.node.offset(index as isize)).white_minimax_score;
                *fresh46 = score as i16;
                (*g_state.g_book.node.offset(index as isize)).black_minimax_score = *fresh46
            }
            if probable_error != 0 {
                /* Clear the old flags if score was wrong */
                let ref mut fresh47 = (*g_state.g_book.node.offset(index as isize)).flags;
                *fresh47 =
                    (*fresh47 as i32 &
                        !(4 as i32 | 16 as i32)) as
                        u16
            }
            if wld_only != 0 {
                let ref mut fresh48 = (*g_state.g_book.node.offset(index as isize)).flags;
                *fresh48 =
                    (*fresh48 as i32 | 4 as i32) as
                        u16
            } else {
                let ref mut fresh49 = (*g_state.g_book.node.offset(index as isize)).flags;
                *fresh49 =
                    (*fresh49 as i32 |
                        (4 as i32 | 16 as i32)) as
                        u16
            }
            /* Examine the position arising from the PV move; if it exists it
            need only be checked for sanity, otherwise a new g_state.g_book.node is
             created. */
            if moves_read > 0 as i32 {
                /* Make sure the optimal move leads to a position in the hash table */

                let row = move_buffer[1] - b'0' as i8;
                let col_0 = (tolower(move_buffer[0] as i32) as i8 - b'a' as i8 + 1) as i8;
                move_0 = 10 * row + col_0;
                if row >= 1 && row <= 8 &&
                    col_0 >= 1 && col_0 <= 8
                    && make_move_no_hash(side_to_move, move_0, &mut g_state.board_state, &mut g_state.moves_state, &mut g_state.flip_stack_ ) != 0 {
                    let mut new_side_to_move =
                        0 as i32 + 2 as i32 - side_to_move;
                    let side_to_move = new_side_to_move;
                    generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
                    if g_state.moves_state.move_count[g_state.moves_state.disks_played as usize] == 0 as i32 {
                        new_side_to_move = side_to_move
                    }
                    let val0___ = &mut val1;
                    let val1___ = &mut val2;
                    let orientation___ = &mut orientation;
                    get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
                    slot = probe_hash_table(val1, val2, &mut g_state.g_book);
                    index = *g_state.g_book.book_hash_table.offset(slot as isize);
                    if index == -(1 as i32) {
                        index =
                            create_BookNode(val1, val2,
                                            32 as i32 as
                                                u16, &mut g_state.g_book);
                        let ref mut fresh50 =
                            (*g_state.g_book.node.offset(index as
                                isize)).white_minimax_score;
                        *fresh50 = score as i16;
                        (*g_state.g_book.node.offset(index as isize)).black_minimax_score =
                            *fresh50;
                        if new_side_to_move == 0 as i32 {
                            let ref mut fresh51 =
                                (*g_state.g_book.node.offset(index as isize)).flags;
                            *fresh51 =
                                (*fresh51 as i32 | 1 as i32)
                                    as u16
                        } else {
                            let ref mut fresh52 =
                                (*g_state.g_book.node.offset(index as isize)).flags;
                            *fresh52 =
                                (*fresh52 as i32 | 2 as i32)
                                    as u16
                        }
                        if wld_only != 0 {
                            let ref mut fresh53 =
                                (*g_state.g_book.node.offset(index as isize)).flags;
                            *fresh53 =
                                (*fresh53 as i32 | 4 as i32)
                                    as u16
                        } else {
                            let ref mut fresh54 =
                                (*g_state.g_book.node.offset(index as isize)).flags;
                            *fresh54 =
                                (*fresh54 as i32 |
                                    (4 as i32 | 16 as i32))
                                    as u16
                        }
                        new_nodes_created += 1
                    } else {
                        /* Position already exists, sanity-check it */
                        probable_error = 0;
                        if (*g_state.g_book.node.offset(index as isize)).flags as i32
                            & 4 as i32 != 0 {
                            if score > 0 as i32 &&
                                (*g_state.g_book.node.offset(index as
                                    isize)).black_minimax_score
                                    as i32 <= 0 as i32 ||
                                score == 0 as i32 &&
                                    (*g_state.g_book.node.offset(index as
                                        isize)).black_minimax_score
                                        as i32 != 0 as i32
                                ||
                                score < 0 as i32 &&
                                    (*g_state.g_book.node.offset(index as
                                        isize)).black_minimax_score
                                        as i32 > 0 as i32 {
                                probable_error = 1;
                                fprintf(stderr,
                                        b"Line %d: New child WLD score %d conflicts with old score %d\n\x00"
                                            as *const u8 as
                                            *const i8, line, score,
                                        (*g_state.g_book.node.offset(index as
                                            isize)).black_minimax_score
                                            as i32);
                            }
                        }
                        if (*g_state.g_book.node.offset(index as isize)).flags as i32
                            & 16 as i32 != 0 {
                            if wld_only == 0 &&
                                score !=
                                    (*g_state.g_book.node.offset(index as
                                        isize)).black_minimax_score
                                        as i32 {
                                probable_error = 1;
                                fprintf(stderr,
                                        b"Line %d: New child exact score %d conflicts with old score %d\n\x00"
                                            as *const u8 as
                                            *const i8, line, score,
                                        (*g_state.g_book.node.offset(index as
                                            isize)).black_minimax_score
                                            as i32);
                            }
                        }
                        if probable_error != 0 {
                            /* Correct errors encountered */
                            let ref mut fresh55 =
                                (*g_state.g_book.node.offset(index as
                                    isize)).white_minimax_score;
                            *fresh55 = score as i16;
                            (*g_state.g_book.node.offset(index as isize)).black_minimax_score
                                = *fresh55;
                            let ref mut fresh56 =
                                (*g_state.g_book.node.offset(index as isize)).flags;
                            *fresh56 =
                                (*fresh56 as i32 &
                                    !(4 as i32 | 16 as i32))
                                    as u16;
                            if wld_only != 0 {
                                let ref mut fresh57 =
                                    (*g_state.g_book.node.offset(index as isize)).flags;
                                *fresh57 =
                                    (*fresh57 as i32 |
                                        4 as i32) as u16
                            } else {
                                let ref mut fresh58 =
                                    (*g_state.g_book.node.offset(index as isize)).flags;
                                *fresh58 =
                                    (*fresh58 as i32 |
                                        (4 as i32 |
                                            16 as i32)) as
                                        u16
                            }
                        }
                    }
                    let side_to_move___unmake_move_no_hash = side_to_move;
                    let move_0___unmake_move_no_hash = move_0;
                    {
                        unmake_move_no_hash(side_to_move___unmake_move_no_hash, move_0___unmake_move_no_hash, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.flip_stack_);
                    };
                } else {
                    fprintf(stderr,
                            b"Line %d: The PV move \'%s\' is invalid\n\x00" as
                                *const u8 as *const i8, line,
                            move_buffer.as_mut_ptr());
                    exit(1 as i32);
                }
            }
        } else if strcmp(script_buffer.as_mut_ptr(),
                         result_buffer.as_mut_ptr()) != 0 {
            fprintf(stderr,
                    b"Script and result files differ unexpectedly on line %d\n\x00"
                        as *const u8 as *const i8, line);
            exit(1 as i32);
        }
        fgets(script_buffer.as_mut_ptr(), 1024 as i32, script_stream);
        fgets(result_buffer.as_mut_ptr(), 1024 as i32, result_stream);
        line += 1
    }
    line -= 1;
    printf(b"%d lines read from the script and result files\n\x00" as
               *const u8 as *const i8, line);
    if feof(script_stream) == 0 || feof(result_stream) == 0 {
        puts(b"Warning: The two files don\'t have the same number of lines.\x00"
            as *const u8 as *const i8);
    }
    printf(b"%d positions merged with the book\n\x00" as *const u8 as
               *const i8, position_count);
    printf(b"%d positions were already solved for exact score\n\x00" as
               *const u8 as *const i8, already_exact_count);
    printf(b"%d positions were already solved WLD\n\x00" as *const u8 as
               *const i8, already_wld_count);
    printf(b"%d positions had optimal moves leading to new positions\n\x00" as
               *const u8 as *const i8, new_nodes_created);
    write!(stdout, "\n");
    fclose(script_stream);
    fclose(result_stream);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct StatisticsSpec {
    pub out_file_name: *const i8,
    pub prob: f64,
    pub max_diff: i32,
    pub max_depth: i32,
}

/*
  VALIDATE_TREE
  Makes sure all nodes are either exhausted, solved or have a deviation.
  The number of positions evaluated is returned.
*/

pub unsafe fn validate_tree(echo: i32, g_state: &mut FullState) -> i32 {
    prepare_tree_traversal(g_state);
    validate_prepared_tree(echo, g_state)
}

// extracted from validate_tree
pub unsafe fn validate_prepared_tree(echo: i32, g_state: &mut FullState) -> i32 {
    g_state.g_book.exhausted_node_count = 0;
    g_state.g_book.evaluated_count = 0;
    g_state.g_book.evaluation_stage = 0;
    let mut feasible_count = 0;
    let mut i = 0;
    while i < g_state.g_book.book_node_count {
        if (*g_state.g_book.node.offset(i as isize)).flags as i32 &
            (4 as i32 | 16 as i32) == 0 &&
            (*g_state.g_book.node.offset(i as isize)).alternative_score as i32 ==
                9999 as i32 &&
            (*g_state.g_book.node.offset(i as isize)).best_alternative_move as i32
                != -(2 as i32) {
            feasible_count += 1
        }
        i += 1
    }
    g_state.g_book.max_eval_count =
        if feasible_count < g_state.g_book.max_batch_size {
            feasible_count
        } else { g_state.g_book.max_batch_size };
    if feasible_count > 0 as i32 {
        i = 0;
        while i < g_state.g_book.book_node_count {
            let ref mut fresh20 = (*g_state.g_book.node.offset(i as isize)).flags;
            *fresh20 =
                (*fresh20 as i32 | 8 as i32) as
                    u16;
            i += 1
        }
        do_validate(0 as i32, echo, g_state);
    }
    return g_state.g_book.evaluated_count;
}

/*
   DO_VALIDATE
   Recursively makes sure a subtree doesn't contain any midgame
   g_state.g_book.node without a deviation move.
*/
pub unsafe fn do_validate(index: i32, echo:i32, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    if g_state.g_book.evaluated_count >= g_state.g_book.max_eval_count { return }
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 &
        (16 as i32 | 4 as i32) == 0 &&
        (*g_state.g_book.node.offset(index as isize)).alternative_score as i32 ==
            9999 as i32 &&
        (*g_state.g_book.node.offset(index as isize)).best_alternative_move as i32
            != -(2 as i32) {
        evaluate_node(index, echo, g_state);
    }
    i = 0;
    while i < g_state.moves_state.move_count[g_state.moves_state.disks_played as usize] {
        this_move = g_state.moves_state.move_list[g_state.moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_state.g_book);
        child = *g_state.g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) { do_validate(child, echo, g_state); }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
        };
        i += 1
    }
    let ref mut fresh19 = (*g_state.g_book.node.offset(index as isize)).flags;
    *fresh19 = (*fresh19 as i32 ^ 8 as i32) as u16;
}


/*
   DO_EVALUATE
   Recursively makes sure a subtree is evaluated to
   the specified depth.
*/
pub unsafe fn do_evaluate(index: i32, echo:i32, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    if g_state.g_book.evaluated_count >= g_state.g_book.max_eval_count { return }
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 &
        (16 as i32 | 4 as i32) == 0 {
        evaluate_node(index, echo, g_state);
    }
    if g_state.g_book.evaluated_count >=
        (g_state.g_book.evaluation_stage + 1 as i32) * g_state.g_book.max_eval_count /
            25 as i32 {
        g_state.g_book.evaluation_stage += 1;
        LibcFatalError::report_do_evaluate(g_state.g_book.evaluation_stage);
    }
    i = 0;
    while i < g_state.moves_state.move_count[g_state.moves_state.disks_played as usize] {
        this_move = g_state.moves_state.move_list[g_state.moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_state.g_book);
        child = *g_state.g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) { do_evaluate(child, echo, g_state); }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
        };
        i += 1
    }
    let ref mut fresh17 = (*g_state.g_book.node.offset(index as isize)).flags;
    *fresh17 = (*fresh17 as i32 ^ 8 as i32) as u16;
}


pub unsafe fn compute_feasible_count(g_state: &mut FullState) -> i32 {
    let mut feasible_count = 0;
    let mut i = 0;
    while i < g_state.g_book.book_node_count {
        let ref mut fresh18 = (*g_state.g_book.node.offset(i as isize)).flags;
        *fresh18 =
            (*fresh18 as i32 | 8 as i32) as u16;
        if ((*g_state.g_book.node.offset(i as isize)).alternative_score as i32 ==
            9999 as i32 ||
            get_node_depth(i, &mut g_state.g_book) < g_state.g_book.search_depth &&
                abs((*g_state.g_book.node.offset(i as isize)).alternative_score as
                    i32) >= g_state.g_book.min_eval_span &&
                abs((*g_state.g_book.node.offset(i as isize)).alternative_score as
                    i32) <= g_state.g_book.max_eval_span &&
                abs((*g_state.g_book.node.offset(i as isize)).black_minimax_score as
                    i32) >= g_state.g_book.min_negamax_span &&
                abs((*g_state.g_book.node.offset(i as isize)).black_minimax_score as
                    i32) <= g_state.g_book.max_negamax_span) &&
            (*g_state.g_book.node.offset(i as isize)).flags as i32 &
                (4 as i32 | 16 as i32) == 0 {
            feasible_count += 1
        }
        i += 1
    }
    feasible_count
}


pub unsafe fn engine_minimax_tree(g_state: &mut FullState) {
    /* Mark all nodes as not traversed */
    let mut i = 0;
    while i < g_state.g_book.book_node_count {
        let ref mut fresh15 = (*g_state.g_book.node.offset(i as isize)).flags;
        *fresh15 =
            (*fresh15 as i32 | 8 as i32) as u16;
        i += 1
    }
    let mut dummy_black_score: i32 = 0;
    let mut dummy_white_score: i32 = 0;
    do_minimax(0 as i32, &mut dummy_black_score, &mut dummy_white_score, g_state);
}

pub unsafe fn engine_examine_tree(g_state: &mut FullState) {
    let mut i = 0;
    while i <= 60 as i32 {
        g_state.g_book.exact_count[i as usize] = 0;
        g_state.g_book.wld_count[i as usize] = 0;
        g_state.g_book.exhausted_count[i as usize] = 0;
        g_state.g_book.common_count[i as usize] = 0;
        i += 1
    }
    g_state.g_book.unreachable_count = 0;
    g_state.g_book.leaf_count = 0;
    g_state.g_book.bad_leaf_count = 0;
    /* Mark all nodes as not traversed and examine the tree */
    i = 0;
    while i < g_state.g_book.book_node_count {
        let ref mut fresh22 = (*g_state.g_book.node.offset(i as isize)).flags;
        *fresh22 =
            (*fresh22 as i32 | 8 as i32) as u16;
        i += 1
    }
    do_examine(0 as i32, g_state);
    /* Any nodes not reached by the walkthrough? */
    i = 0;
    while i < g_state.g_book.book_node_count {
        if (*g_state.g_book.node.offset(i as isize)).flags as i32 & 8 as i32
            != 0 {
            g_state.g_book.unreachable_count += 1;
            let ref mut fresh23 = (*g_state.g_book.node.offset(i as isize)).flags;
            *fresh23 =
                (*fresh23 as i32 ^ 8 as i32) as u16
        }
        i += 1
    }
}

/*
   DO_EXAMINE
   Add the properties of node INDEX to the statistics being gathered
   and recursively traverse the subtree of the node, doing the same
   thing in all nodes.
*/
pub unsafe fn do_examine(index: i32,  g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut child: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut this_move = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut child_count: i32 = 0;
    let mut child_move: [i8; 64] = [0; 64];
    let mut child_node: [i32; 64] = [0; 64];
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 16 as i32
        != 0 {
        g_state.g_book.exact_count[g_state.moves_state.disks_played as usize] += 1
    } else if (*g_state.g_book.node.offset(index as isize)).flags as i32 &
        4 as i32 != 0 {
        g_state.g_book.wld_count[g_state.moves_state.disks_played as usize] += 1
    } else if (*g_state.g_book.node.offset(index as isize)).best_alternative_move as
        i32 == -(2 as i32) {
        g_state.g_book.exhausted_count[g_state.moves_state.disks_played as usize] += 1
    } else { g_state.g_book.common_count[g_state.moves_state.disks_played as usize] += 1 }
    /* Examine all the children of the g_state.g_book.node */
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
    child_count = 0;
    i = 0;
    while i < g_state.moves_state.move_count[g_state.moves_state.disks_played as usize] {
        this_move = g_state.moves_state.move_list[g_state.moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
        let val0___ = &mut val1;
        let val1___ = &mut val2;
        let orientation___ = &mut orientation;
        get_hash(val0___, val1___, orientation___, &mut g_state.g_book, &g_state.board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_state.g_book);
        child = *g_state.g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) {
            child_move[child_count as usize] = this_move;
            child_node[child_count as usize] = child;
            child_count += 1
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
        };
        i += 1
    }
    if child_count == 0 as i32 {
        g_state.g_book.leaf_count += 1;
        if (*g_state.g_book.node.offset(index as isize)).flags as i32 &
            16 as i32 == 0 {
            g_state.g_book.bad_leaf_count += 1
        }
        if (*g_state.g_book.node.offset(index as isize)).flags as i32 &
            4 as i32 == 0 {
            g_state.g_book.really_bad_leaf_count += 1
        }
    } else {
        let mut current_block_38: u64;
        i = 0;
        while i < child_count {
            if side_to_move == 0 as i32 {
                if g_state.g_book.force_black != 0 &&
                    (*g_state.g_book.node.offset(child_node[i as usize] as
                        isize)).black_minimax_score as
                        i32 !=
                        (*g_state.g_book.node.offset(index as isize)).black_minimax_score
                            as i32 {
                    current_block_38 = 2873832966593178012;
                } else { current_block_38 = 10891380440665537214; }
            } else if g_state.g_book.force_white != 0 &&
                (*g_state.g_book.node.offset(child_node[i as usize] as
                    isize)).white_minimax_score as
                    i32 !=
                    (*g_state.g_book.node.offset(index as
                        isize)).white_minimax_score as
                        i32 {
                current_block_38 = 2873832966593178012;
            } else { current_block_38 = 10891380440665537214; }
            match current_block_38 {
                10891380440665537214 => {
                    this_move = child_move[i as usize];
                    make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
                    do_examine(child_node[i as usize], g_state);
                    let move_0 = this_move;
                    {
                        unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
                    };
                }
                _ => { }
            }
            i += 1
        }
    }
    let ref mut fresh21 = (*g_state.g_book.node.offset(index as isize)).flags;
    *fresh21 = (*fresh21 as i32 ^ 8 as i32) as u16;
}


/*
   DO_COMPRESS
   Compresses the subtree below the current node.
*/
pub unsafe fn do_compress(index: i32,
                          node_order: *mut i32,
                          child_count: *mut i16,
                          node_index: &mut i32,
                          child_list: *mut i16,
                          child_index: &mut i32, g_state: &mut FullState) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut child: i32 = 0;
    let mut valid_child_count: i32 = 0;
    let mut side_to_move: i32 = 0;
    let mut slot: i32 = 0;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut orientation: i32 = 0;
    let mut found: i32 = 0;
    let mut local_child_list: [i32; 64] = [0; 64];
    let mut this_move = 0;
    let mut local_child_move: [i8; 64] = [0; 64];
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 8 as i32
        == 0 {
        return
    }
    *node_order.offset(*node_index as isize) = index;
    if (*g_state.g_book.node.offset(index as isize)).flags as i32 & 1 as i32
        != 0 {
        side_to_move = 0 as i32
    } else { side_to_move = 2 as i32 }
    valid_child_count = 0;
    generate_all(side_to_move, &mut g_state.moves_state, &g_state.search_state, &g_state.board_state.board);
    i = 0;
    while i < g_state.moves_state.move_count[g_state.moves_state.disks_played as usize] {
        this_move = g_state.moves_state.move_list[g_state.moves_state.disks_played as usize][i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
        get_hash(&mut val1, &mut val2, &mut orientation, &mut g_state.g_book, &g_state.board_state.board);
        slot = probe_hash_table(val1, val2, &mut g_state.g_book);
        child = *g_state.g_book.book_hash_table.offset(slot as isize);
        if child != -(1 as i32) &&
            (*g_state.g_book.node.offset(child as isize)).flags as i32 &
                8 as i32 != 0 {
            j = 0;
            found = 0;
            while j < valid_child_count {
                if child == local_child_list[j as usize] {
                    found = 1 as i32
                }
                j += 1
            }
            if found == 0 {
                local_child_list[valid_child_count as usize] = child;
                local_child_move[valid_child_count as usize] = this_move;
                valid_child_count += 1;
                *child_list.offset(*child_index as isize) = this_move as i16;
                *child_index += 1
            }
        }
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
        };
        i += 1
    }
    *child_count.offset(*node_index as isize) =
        valid_child_count as i16;
    *node_index += 1;
    i = 0;
    while i < valid_child_count {
        this_move = local_child_move[i as usize];
        make_move(side_to_move, this_move, 1 as i32 , &mut g_state.moves_state, &mut g_state.board_state, &mut g_state.hash_state, &mut g_state.flip_stack_ );
        do_compress(local_child_list[i as usize], node_order, child_count,
                    node_index, child_list, child_index, g_state);
        let move_0 = this_move;
        {
            unmake_move(side_to_move, move_0, &mut g_state.board_state.board, &mut g_state.moves_state, &mut g_state.hash_state, &mut g_state.flip_stack_);
        };
        i += 1
    }
    let ref mut fresh44 = (*g_state.g_book.node.offset(index as isize)).flags;
    *fresh44 = (*fresh44 as i32 ^ 8 as i32) as u16;
}
