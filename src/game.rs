use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    #[no_mangle]
    fn init_bitboard();
    /* The time spent searching during the game. */
    #[no_mangle]
    static mut total_time: libc::c_double;
    /* The value of the root position from the last midgame or
   endgame search. Can contain strange values if an event
   occurred. */
    #[no_mangle]
    static mut root_eval: libc::c_int;
    /* Event flag which forces the search to abort immediately when set. */
    #[no_mangle]
    static mut force_return: libc::c_int;
    /* The number of positions evaluated during the current search. */
    #[no_mangle]
    static mut evaluations: CounterType;
    /* The number of positions evaluated during the entire game. */
    #[no_mangle]
    static mut total_evaluations: CounterType;
    /* Holds the number of nodes searched during the current search. */
    #[no_mangle]
    static mut nodes: CounterType;
    /* Holds the total number of nodes searched during the entire game. */
    #[no_mangle]
    static mut total_nodes: CounterType;
    /* The last available evaluations for all possible moves at all
   possible game stages. */
    #[no_mangle]
    static mut evals: [Board; 61];
    #[no_mangle]
    fn setup_search();
    #[no_mangle]
    fn disc_count(side_to_move: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sort_moves(list_size: libc::c_int);
    #[no_mangle]
    fn float_move(move_0: libc::c_int, list_size: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn clear_pv();
    #[no_mangle]
    fn set_ponder_move(move_0: libc::c_int);
    #[no_mangle]
    fn clear_ponder_move();
    #[no_mangle]
    fn get_ponder_move() -> libc::c_int;
    #[no_mangle]
    fn create_eval_info(in_type: EvalType, in_res: EvalResult,
                        in_score: libc::c_int, in_conf: libc::c_double,
                        in_depth: libc::c_int, in_book: libc::c_int)
     -> EvaluationType;
    #[no_mangle]
    fn complete_pv(side_to_move: libc::c_int);
    /*
   File:         display.h

   Created:      July 10, 1997

   Modified:     November 17, 2002

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:     Declarations of the screen output functions.
*/
    /* Flag variable, non-zero if output should be written to stdout. */
    /* Flag variable, non-zero if the principal variation is to be
   displayed. */
    #[no_mangle]
    fn send_status_pv(pv_0: *mut libc::c_int, max_depth: libc::c_int);
    /* Holds the current board position. Updated as the search progresses,
   but all updates must be reversed when the search stops. */
    #[no_mangle]
    static mut board: Board;
    #[no_mangle]
    fn set_current_eval(eval: EvaluationType);
    #[no_mangle]
    fn negate_current_eval(negate: libc::c_int);
    #[no_mangle]
    static mut echo: libc::c_int;
    #[no_mangle]
    static mut display_pv: libc::c_int;
    #[no_mangle]
    fn display_board(stream: *mut FILE, board_0: *mut libc::c_int,
                     side_to_move: libc::c_int, give_game_score: libc::c_int,
                     give_time: libc::c_int, give_evals: libc::c_int);
    #[no_mangle]
    fn display_optimal_line(stream: *mut FILE);
    #[no_mangle]
    fn send_status(format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn send_status_time(elapsed_time: libc::c_double);
    #[no_mangle]
    fn counter_value(counter: *mut CounterType) -> libc::c_double;
    #[no_mangle]
    fn adjust_counter(counter: *mut CounterType);
    #[no_mangle]
    fn add_counter(sum: *mut CounterType, term: *mut CounterType);
    #[no_mangle]
    static mut pv: [[libc::c_int; 64]; 64];
    #[no_mangle]
    static mut pv_depth: [libc::c_int; 64];
    #[no_mangle]
    static mut piece_count: [[libc::c_int; 64]; 3];
    #[no_mangle]
    static mut score_sheet_row: libc::c_int;
    #[no_mangle]
    static mut black_moves: [libc::c_int; 60];
    #[no_mangle]
    fn reset_counter(counter: *mut CounterType);
    #[no_mangle]
    fn send_status_nodes(node_count: libc::c_double);
    #[no_mangle]
    fn clear_status();
    #[no_mangle]
    fn display_status(stream: *mut FILE, allow_repeat: libc::c_int);
    #[no_mangle]
    fn reset_buffer_display();
    #[no_mangle]
    fn produce_eval_text(eval_info: EvaluationType, short_output: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn end_game(side_to_move: libc::c_int, wld: libc::c_int,
                force_echo: libc::c_int, allow_book: libc::c_int,
                komi_0: libc::c_int, eval_info: *mut EvaluationType)
     -> libc::c_int;
    #[no_mangle]
    fn setup_end();
    /*
   File:       error.h

   Created:    June 13, 1998

   Modified:   August 1, 2002

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The interface to the error handler.
*/
    #[no_mangle]
    fn fatal_error(format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn init_eval();
    /*
   File:         getcoeff.h

   Created:      November 20, 1997

   Modified:     August 1, 2002

   Author:       Gunnar Andersson (gunnar@radagast.se)

   Contents:
*/
    #[no_mangle]
    fn init_coeffs();
    #[no_mangle]
    fn remove_coeffs(phase: libc::c_int);
    #[no_mangle]
    fn clear_coeffs();
    #[no_mangle]
    fn pattern_evaluation(side_to_move: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn init_hash(in_hash_bits: libc::c_int);
    #[no_mangle]
    fn free_hash();
    #[no_mangle]
    fn determine_hash_values(side_to_move: libc::c_int,
                             board_0: *const libc::c_int);
    #[no_mangle]
    fn set_hash_transformation(trans1: libc::c_uint, trans2: libc::c_uint);
    #[no_mangle]
    fn find_hash(entry: *mut HashEntry, reverse_mode: libc::c_int);
    /*
   File:          midgame.h

   Created:       July 1, 1998

   Modified:      August 1, 2002

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The midgame search driver.
*/
    /* The minimum depth to perform Multi-ProbCut */
    #[no_mangle]
    fn setup_midgame();
    #[no_mangle]
    fn toggle_midgame_hash_usage(allow_read: libc::c_int,
                                 allow_write: libc::c_int);
    #[no_mangle]
    fn clear_midgame_abort();
    #[no_mangle]
    fn is_midgame_abort() -> libc::c_int;
    #[no_mangle]
    fn toggle_midgame_abort_check(toggle: libc::c_int);
    #[no_mangle]
    fn calculate_perturbation();
    #[no_mangle]
    fn toggle_perturbation_usage(toggle: libc::c_int);
    #[no_mangle]
    fn middle_game(side_to_move: libc::c_int, max_depth: libc::c_int,
                   update_evals: libc::c_int, eval_info: *mut EvaluationType)
     -> libc::c_int;
    /*
   File:           moves.h

   Created:        June 30, 1997

   Modified:       August 1, 2002

   Author:         Gunnar Andersson (gunnar@radagast.se)

   Contents:       The move generator's interface.
*/
    /* The number of disks played from the initial position.
   Must match the current status of the BOARD variable. */
    #[no_mangle]
    static mut disks_played: libc::c_int;
    /* The number of moves available after a certain number
   of disks played. */
    #[no_mangle]
    static mut move_count: [libc::c_int; 64];
    /* The actual moves available after a certain number of
   disks played. */
    #[no_mangle]
    static mut move_list: [[libc::c_int; 64]; 64];
    #[no_mangle]
    fn init_moves();
    #[no_mangle]
    fn generate_all(side_to_move: libc::c_int);
    #[no_mangle]
    fn make_move(side_to_move: libc::c_int, move_0: libc::c_int,
                 update_hash: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn unmake_move(side_to_move: libc::c_int, move_0: libc::c_int);
    #[no_mangle]
    fn valid_move(move_0: libc::c_int, side_to_move: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn my_srandom(x: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn my_random() -> libc::c_long;
    #[no_mangle]
    fn check_forced_opening(side_to_move: libc::c_int,
                            opening: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fill_move_alternatives(side_to_move: libc::c_int, flags: libc::c_int);
    #[no_mangle]
    fn get_candidate_count() -> libc::c_int;
    #[no_mangle]
    fn get_candidate(index: libc::c_int) -> CandidateMove;
    #[no_mangle]
    fn get_book_move(side_to_move: libc::c_int, update_slack: libc::c_int,
                     eval_info: *mut EvaluationType) -> libc::c_int;
    #[no_mangle]
    fn clear_osf();
    #[no_mangle]
    fn init_patterns();
    /*
   File:          probcut.h

   Created:       March 1, 1998

   Modified:      November 23, 2002

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The declaration of the Multi-Prob-Cut variables.
*/
    #[no_mangle]
    fn init_probcut();
    #[no_mangle]
    fn init_stable();
    #[no_mangle]
    fn choose_thor_opening_move(in_board: *mut libc::c_int,
                                side_to_move: libc::c_int,
                                echo_0: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn database_search(in_board: *mut libc::c_int, side_to_move: libc::c_int);
    #[no_mangle]
    fn get_match_count() -> libc::c_int;
    #[no_mangle]
    fn get_thor_game_move(index: libc::c_int, move_number: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    static mut ponder_depth: [libc::c_int; 100];
    #[no_mangle]
    fn determine_move_time(time_left: libc::c_double, incr: libc::c_double,
                           discs: libc::c_int);
    #[no_mangle]
    fn start_move(in_total_time: libc::c_double, increment: libc::c_double,
                  discs: libc::c_int);
    #[no_mangle]
    fn clear_panic_abort();
    #[no_mangle]
    fn is_panic_abort() -> libc::c_int;
    #[no_mangle]
    fn toggle_abort_check(enable: libc::c_int);
    #[no_mangle]
    fn init_timer();
    #[no_mangle]
    fn get_real_timer() -> libc::c_double;
    #[no_mangle]
    fn get_elapsed_time() -> libc::c_double;
    #[no_mangle]
    fn clear_ponder_times();
    #[no_mangle]
    fn add_ponder_time(move_0: libc::c_int, time_0: libc::c_double);
    #[no_mangle]
    fn init_flip_stack();
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CounterType {
    pub hi: libc::c_uint,
    pub lo: libc::c_uint,
}
pub type Board = [libc::c_int; 128];
pub type EvalType = libc::c_uint;
pub const UNINITIALIZED_EVAL: EvalType = 8;
pub const INTERRUPTED_EVAL: EvalType = 7;
pub const UNDEFINED_EVAL: EvalType = 6;
pub const PASS_EVAL: EvalType = 5;
pub const FORCED_EVAL: EvalType = 4;
pub const SELECTIVE_EVAL: EvalType = 3;
pub const WLD_EVAL: EvalType = 2;
pub const EXACT_EVAL: EvalType = 1;
pub const MIDGAME_EVAL: EvalType = 0;
pub type EvalResult = libc::c_uint;
pub const UNSOLVED_POSITION: EvalResult = 3;
pub const LOST_POSITION: EvalResult = 2;
pub const DRAWN_POSITION: EvalResult = 1;
pub const WON_POSITION: EvalResult = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EvaluationType {
    pub type_0: EvalType,
    pub res: EvalResult,
    pub score: libc::c_int,
    pub confidence: libc::c_double,
    pub search_depth: libc::c_int,
    pub is_book: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EvaluatedMove {
    pub eval: EvaluationType,
    pub side_to_move: libc::c_int,
    pub move_0: libc::c_int,
    pub pv_depth: libc::c_int,
    pub pv: [libc::c_int; 60],
}
pub const BOOK_MOVE: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const ENDGAME_MOVE: C2RustUnnamed = 3;
pub const MIDGAME_MOVE: C2RustUnnamed = 2;
pub const INTERRUPTED_MOVE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashEntry {
    pub key1: libc::c_uint,
    pub key2: libc::c_uint,
    pub eval: libc::c_int,
    pub move_0: [libc::c_int; 4],
    pub draft: libc::c_short,
    pub selectivity: libc::c_short,
    pub flags: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CandidateMove {
    pub move_0: libc::c_int,
    pub score: libc::c_int,
    pub flags: libc::c_int,
    pub parent_flags: libc::c_int,
}
/* The maximum length of any system path. */
static mut forced_opening: *const libc::c_char = 0 as *const libc::c_char;
static mut log_file_path: [libc::c_char; 2048] = [0; 2048];
static mut last_time_used: libc::c_double = 0.;
static mut max_depth_reached: libc::c_int = 0;
static mut use_log_file: libc::c_int = 1 as libc::c_int;
static mut play_human_openings: libc::c_int = 1 as libc::c_int;
static mut play_thor_match_openings: libc::c_int = 1 as libc::c_int;
static mut game_evaluated_count: libc::c_int = 0;
static mut komi: libc::c_int = 0 as libc::c_int;
static mut prefix_move: libc::c_int = 0 as libc::c_int;
static mut endgame_performed: [libc::c_int; 3] = [0; 3];
static mut evaluated_list: [EvaluatedMove; 60] =
    [EvaluatedMove{eval:
                       EvaluationType{type_0: MIDGAME_EVAL,
                                      res: WON_POSITION,
                                      score: 0,
                                      confidence: 0.,
                                      search_depth: 0,
                                      is_book: 0,},
                   side_to_move: 0,
                   move_0: 0,
                   pv_depth: 0,
                   pv: [0; 60],}; 60];
/*
  TOGGLE_STATUS_LOG
  Enable/disable the use of logging all the output that the
  text version of Zebra would output to the screen.
*/
#[no_mangle]
pub unsafe extern "C" fn toggle_status_log(mut write_log: libc::c_int) {
    use_log_file = write_log;
}
/*
   GLOBAL_SETUP
   Initialize the different sub-systems.
*/
#[no_mangle]
pub unsafe extern "C" fn global_setup(mut use_random: libc::c_int,
                                      mut hash_bits: libc::c_int) {
    let mut log_file = 0 as *mut FILE;
    let mut timer: time_t = 0;
    /* Clear the log file. No error handling done. */
    strcpy(log_file_path.as_mut_ptr(),
           b"zebra.log\x00" as *const u8 as *const libc::c_char);
    if use_log_file != 0 {
        log_file =
            fopen(log_file_path.as_mut_ptr(),
                  b"w\x00" as *const u8 as *const libc::c_char);
        if !log_file.is_null() {
            time(&mut timer);
            fprintf(log_file,
                    b"%s %s\n\x00" as *const u8 as *const libc::c_char,
                    b"Log file created\x00" as *const u8 as
                        *const libc::c_char, ctime(&mut timer));
            fprintf(log_file,
                    b"%s %s %s\n\x00" as *const u8 as *const libc::c_char,
                    b"Engine compiled\x00" as *const u8 as
                        *const libc::c_char,
                    b"Jul  2 2020\x00" as *const u8 as *const libc::c_char,
                    b"19:33:59\x00" as *const u8 as *const libc::c_char);
            fclose(log_file);
        }
    }
    if use_random != 0 {
        time(&mut timer);
        my_srandom(timer as libc::c_int);
    } else { my_srandom(1 as libc::c_int); }
    init_hash(hash_bits);
    init_bitboard();
    init_moves();
    init_patterns();
    init_coeffs();
    init_timer();
    init_probcut();
    init_stable();
    setup_search();
}
/*
   GLOBAL_TERMINATE
   Free all dynamically allocated memory.
*/
#[no_mangle]
pub unsafe extern "C" fn global_terminate() {
    free_hash();
    clear_coeffs();
    clear_osf();
}
/*
   SETUP_GAME
   Prepares the board.
*/
unsafe extern "C" fn setup_game(mut file_name: *const libc::c_char,
                                mut side_to_move: *mut libc::c_int) {
    let mut buffer: [libc::c_char; 65] = [0; 65];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut token: libc::c_int = 0;
    let mut stream = 0 as *mut FILE;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 10 as libc::c_int {
            pos = 10 as libc::c_int * i + j;
            if i == 0 as libc::c_int || i == 9 as libc::c_int ||
                   j == 0 as libc::c_int || j == 9 as libc::c_int {
                board[pos as usize] = 3 as libc::c_int
            } else { board[pos as usize] = 1 as libc::c_int }
            j += 1
        }
        i += 1
    }
    if file_name.is_null() {
        board[54 as libc::c_int as usize] = 0 as libc::c_int;
        board[45 as libc::c_int as usize] = board[54 as libc::c_int as usize];
        board[55 as libc::c_int as usize] = 2 as libc::c_int;
        board[44 as libc::c_int as usize] = board[55 as libc::c_int as usize];
        *side_to_move = 0 as libc::c_int
    } else {
        stream =
            fopen(file_name, b"r\x00" as *const u8 as *const libc::c_char);
        if stream.is_null() {
            fatal_error(b"%s \'%s\'\n\x00" as *const u8 as
                            *const libc::c_char,
                        b"Cannot open game file\x00" as *const u8 as
                            *const libc::c_char, file_name);
        }
        fgets(buffer.as_mut_ptr(), 70 as libc::c_int, stream);
        token = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= 8 as libc::c_int {
            j = 1 as libc::c_int;
            while j <= 8 as libc::c_int {
                pos = 10 as libc::c_int * i + j;
                match buffer[token as usize] as libc::c_int {
                    42 | 88 => { board[pos as usize] = 0 as libc::c_int }
                    79 | 48 => { board[pos as usize] = 2 as libc::c_int }
                    45 | 46 => { }
                    _ => {
                        printf(b"%s \'%c\' %s\n\x00" as *const u8 as
                                   *const libc::c_char,
                               b"Unrecognized character\x00" as *const u8 as
                                   *const libc::c_char,
                               buffer[pos as usize] as libc::c_int,
                               b"in game file\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                }
                token += 1;
                j += 1
            }
            i += 1
        }
        fgets(buffer.as_mut_ptr(), 10 as libc::c_int, stream);
        if buffer[0 as libc::c_int as usize] as libc::c_int == 'B' as i32 {
            *side_to_move = 0 as libc::c_int
        } else if buffer[0 as libc::c_int as usize] as libc::c_int ==
                      'W' as i32 {
            *side_to_move = 2 as libc::c_int
        } else {
            fatal_error(b"%s \'%c\' %s\n\x00" as *const u8 as
                            *const libc::c_char,
                        b"Unrecognized character\x00" as *const u8 as
                            *const libc::c_char,
                        buffer[0 as libc::c_int as usize] as libc::c_int,
                        b"in game file\x00" as *const u8 as
                            *const libc::c_char);
        }
    }
    disks_played =
        disc_count(0 as libc::c_int) + disc_count(2 as libc::c_int) -
            4 as libc::c_int;
    determine_hash_values(*side_to_move, board.as_mut_ptr());
    /* Make the game score look right */
    if *side_to_move == 0 as libc::c_int {
        score_sheet_row = -(1 as libc::c_int)
    } else {
        black_moves[0 as libc::c_int as usize] = -(1 as libc::c_int);
        score_sheet_row = 0 as libc::c_int
    };
}
/*
   GAME_INIT
   Prepare the relevant data structures so that a game
   can be played. The position is read from the file
   specified by FILE_NAME.
*/
#[no_mangle]
pub unsafe extern "C" fn game_init(mut file_name: *const libc::c_char,
                                   mut side_to_move: *mut libc::c_int) {
    setup_game(file_name, side_to_move);
    setup_search();
    setup_midgame();
    setup_end();
    init_eval();
    clear_ponder_times();
    reset_counter(&mut total_nodes);
    reset_counter(&mut total_evaluations);
    init_flip_stack();
    total_time = 0.0f64;
    max_depth_reached = 0 as libc::c_int;
    last_time_used = 0.0f64;
    endgame_performed[2 as libc::c_int as usize] = 0 as libc::c_int;
    endgame_performed[0 as libc::c_int as usize] =
        endgame_performed[2 as libc::c_int as usize];
}
/*
  SET_KOMI
  Set the endgame komi value.
*/
#[no_mangle]
pub unsafe extern "C" fn set_komi(mut in_komi: libc::c_int) {
    komi = in_komi;
}
/*
  TOGGLE_HUMAN_OPENINGS
  Specifies whether the Thor statistics should be queried for
  openings moves before resorting to the usual opening book.
*/
#[no_mangle]
pub unsafe extern "C" fn toggle_human_openings(mut toggle: libc::c_int) {
    play_human_openings = toggle;
}
/*
  TOGGLE_THOR_MATCH_OPENINGS
  Specifies whether matching Thor games are used as opening book
  before resorting to the usual opening book.
*/
#[no_mangle]
pub unsafe extern "C" fn toggle_thor_match_openings(mut toggle: libc::c_int) {
    play_thor_match_openings = toggle;
}
/*
  SET_FORCED_OPENING
  Specifies an opening line that Zebra is forced to follow when playing.
*/
#[no_mangle]
pub unsafe extern "C" fn set_forced_opening(mut opening_str:
                                                *const libc::c_char) {
    forced_opening = opening_str;
}
/*
  PONDER_MOVE
  Perform searches in response to the opponent's next move.
  The results are not returned, but the hash table is filled
  with useful scores and moves.
*/
#[no_mangle]
pub unsafe extern "C" fn ponder_move(mut side_to_move: libc::c_int,
                                     mut book: libc::c_int,
                                     mut mid: libc::c_int,
                                     mut exact: libc::c_int,
                                     mut wld: libc::c_int) {
    let mut eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut entry =
        HashEntry{key1: 0,
                  key2: 0,
                  eval: 0,
                  move_0: [0; 4],
                  draft: 0,
                  selectivity: 0,
                  flags: 0,};
    let mut move_start_time: libc::c_double = 0.;
    let mut move_stop_time: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut this_move: libc::c_int = 0;
    let mut hash_move: libc::c_int = 0;
    let mut expect_count: libc::c_int = 0;
    let mut stored_echo: libc::c_int = 0;
    let mut best_pv_depth: libc::c_int = 0;
    let mut expect_list: [libc::c_int; 64] = [0; 64];
    let mut best_pv: [libc::c_int; 61] = [0; 61];
    /* Disable all time control mechanisms as it's the opponent's
       time we're using */
    toggle_abort_check(0 as libc::c_int);
    toggle_midgame_abort_check(0 as libc::c_int);
    start_move(0 as libc::c_int as libc::c_double,
               0 as libc::c_int as libc::c_double,
               disc_count(0 as libc::c_int) + disc_count(2 as libc::c_int));
    clear_ponder_times();
    determine_hash_values(side_to_move, board.as_mut_ptr());
    reset_counter(&mut nodes);
    /* Find the scores for the moves available to the opponent. */
    hash_move = 0 as libc::c_int;
    find_hash(&mut entry, 1 as libc::c_int);
    if entry.draft as libc::c_int != 0 as libc::c_int {
        hash_move = entry.move_0[0 as libc::c_int as usize]
    } else {
        find_hash(&mut entry, 0 as libc::c_int);
        if entry.draft as libc::c_int != 0 as libc::c_int {
            hash_move = entry.move_0[0 as libc::c_int as usize]
        }
    }
    stored_echo = echo;
    echo = 0 as libc::c_int;
    compute_move(side_to_move, 0 as libc::c_int, 0 as libc::c_int,
                 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
                 if (8 as libc::c_int) < mid {
                     8 as libc::c_int
                 } else { mid }, 0 as libc::c_int, 0 as libc::c_int,
                 0 as libc::c_int, &mut eval_info);
    echo = stored_echo;
    /* Sort the opponents on the score and push the table move (if any)
       to the front of the list */
    if force_return != 0 {
        expect_count = 0 as libc::c_int
    } else {
        sort_moves(move_count[disks_played as usize]);
        float_move(hash_move, move_count[disks_played as usize]);
        expect_count = move_count[disks_played as usize];
        i = 0 as libc::c_int;
        while i < expect_count {
            expect_list[i as usize] =
                move_list[disks_played as usize][i as usize];
            i += 1
        }
        printf(b"%s=%d\n\x00" as *const u8 as *const libc::c_char,
               b"hash move\x00" as *const u8 as *const libc::c_char,
               hash_move);
        i = 0 as libc::c_int;
        while i < expect_count {
            printf(b"%c%c %-6.2f  \x00" as *const u8 as *const libc::c_char,
                   'a' as i32 +
                       move_list[disks_played as usize][i as usize] %
                           10 as libc::c_int - 1 as libc::c_int,
                   '0' as i32 +
                       move_list[disks_played as usize][i as usize] /
                           10 as libc::c_int,
                   evals[disks_played as
                             usize][move_list[disks_played as
                                                  usize][i as usize] as usize]
                       as libc::c_double / 128.0f64);
            if i % 7 as libc::c_int == 6 as libc::c_int ||
                   i == expect_count - 1 as libc::c_int {
                puts(b"\x00" as *const u8 as *const libc::c_char);
            }
            i += 1
        }
    }
    /* Go through the expected moves in order and prepare responses. */
    best_pv_depth = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while force_return == 0 && i < expect_count {
        move_start_time = get_real_timer();
        set_ponder_move(expect_list[i as usize]);
        this_move = expect_list[i as usize];
        prefix_move = this_move;
        make_move(side_to_move, this_move, 1 as libc::c_int);
        compute_move(0 as libc::c_int + 2 as libc::c_int - side_to_move,
                     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
                     1 as libc::c_int, 0 as libc::c_int, mid, exact, wld,
                     0 as libc::c_int, &mut eval_info);
        unmake_move(side_to_move, this_move);
        clear_ponder_move();
        move_stop_time = get_real_timer();
        add_ponder_time(expect_list[i as usize],
                        move_stop_time - move_start_time);
        ponder_depth[expect_list[i as usize] as usize] =
            if ponder_depth[expect_list[i as usize] as usize] >
                   max_depth_reached - 1 as libc::c_int {
                ponder_depth[expect_list[i as usize] as usize]
            } else { (max_depth_reached) - 1 as libc::c_int };
        if i == 0 as libc::c_int && force_return == 0 {
            /* Store the PV for the first move */
            best_pv_depth = pv_depth[0 as libc::c_int as usize];
            j = 0 as libc::c_int;
            while j < pv_depth[0 as libc::c_int as usize] {
                best_pv[j as usize] =
                    pv[0 as libc::c_int as usize][j as usize];
                j += 1
            }
        }
        i += 1
    }
    /* Make sure the PV looks reasonable when leaving - either by
       clearing it altogether or, preferrably, using the stored PV for
       the first move if it is available. */
    max_depth_reached += 1;
    prefix_move = 0 as libc::c_int;
    if best_pv_depth == 0 as libc::c_int {
        pv_depth[0 as libc::c_int as usize] = 0 as libc::c_int
    } else {
        pv_depth[0 as libc::c_int as usize] =
            best_pv_depth + 1 as libc::c_int;
        pv[0 as libc::c_int as usize][0 as libc::c_int as usize] =
            expect_list[0 as libc::c_int as usize];
        i = 0 as libc::c_int;
        while i < best_pv_depth {
            pv[0 as libc::c_int as usize][(i + 1 as libc::c_int) as usize] =
                best_pv[i as usize];
            i += 1
        }
    }
    /* Don't forget to enable the time control mechanisms when leaving */
    toggle_abort_check(1 as libc::c_int);
    toggle_midgame_abort_check(1 as libc::c_int);
}
/*
  COMPARE_EVAL
  Comparison function for two evals.  Same return value conventions
  as QuickSort.
*/
unsafe extern "C" fn compare_eval(mut e1: EvaluationType,
                                  mut e2: EvaluationType) -> libc::c_int {
    if e1.type_0 as libc::c_uint == WLD_EVAL as libc::c_int as libc::c_uint ||
           e1.type_0 as libc::c_uint ==
               EXACT_EVAL as libc::c_int as libc::c_uint {
        if e1.score > 0 as libc::c_int { e1.score += 100000 as libc::c_int }
    }
    if e2.type_0 as libc::c_uint == WLD_EVAL as libc::c_int as libc::c_uint ||
           e2.type_0 as libc::c_uint ==
               EXACT_EVAL as libc::c_int as libc::c_uint {
        if e2.score > 0 as libc::c_int { e2.score += 100000 as libc::c_int }
    }
    return e1.score - e2.score;
}
/*
  EXTENDED_COMPUTE_MOVE
  This wrapper on top of compute_move() calculates the evaluation
  of all moves available as opposed to upper bounds for all moves
  except for the best.
*/
#[no_mangle]
pub unsafe extern "C" fn extended_compute_move(mut side_to_move: libc::c_int,
                                               mut book_only: libc::c_int,
                                               mut book: libc::c_int,
                                               mut mid: libc::c_int,
                                               mut exact: libc::c_int,
                                               mut wld: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut changed: libc::c_int = 0;
    let mut this_move: libc::c_int = 0;
    let mut disc_diff: libc::c_int = 0;
    let mut corrected_diff: libc::c_int = 0;
    let mut best_move: libc::c_int = 0;
    let mut temp_move: libc::c_int = 0;
    let mut best_score: libc::c_int = 0;
    let mut best_pv_depth: libc::c_int = 0;
    let mut stored_echo: libc::c_int = 0;
    let mut shallow_eval: libc::c_int = 0;
    let mut empties: libc::c_int = 0;
    let mut current_mid: libc::c_int = 0;
    let mut current_exact: libc::c_int = 0;
    let mut current_wld: libc::c_int = 0;
    let mut first_iteration: libc::c_int = 0;
    let mut unsearched: libc::c_int = 0;
    let mut unsearched_count: libc::c_int = 0;
    let mut unsearched_move: [libc::c_int; 61] = [0; 61];
    let mut best_pv: [libc::c_int; 60] = [0; 60];
    let mut transform1: [libc::c_uint; 60] = [0; 60];
    let mut transform2: [libc::c_uint; 60] = [0; 60];
    let mut book_move =
        CandidateMove{move_0: 0, score: 0, flags: 0, parent_flags: 0,};
    let mut temp =
        EvaluatedMove{eval:
                          EvaluationType{type_0: MIDGAME_EVAL,
                                         res: WON_POSITION,
                                         score: 0,
                                         confidence: 0.,
                                         search_depth: 0,
                                         is_book: 0,},
                      side_to_move: 0,
                      move_0: 0,
                      pv_depth: 0,
                      pv: [0; 60],};
    let mut book_eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut res = WON_POSITION;
    /* Disable all time control mechanisms and randomization */
    toggle_abort_check(0 as libc::c_int);
    toggle_midgame_abort_check(0 as libc::c_int);
    toggle_perturbation_usage(0 as libc::c_int);
    start_move(0 as libc::c_int as libc::c_double,
               0 as libc::c_int as libc::c_double,
               disc_count(0 as libc::c_int) + disc_count(2 as libc::c_int));
    clear_ponder_times();
    determine_hash_values(side_to_move, board.as_mut_ptr());
    empties = 60 as libc::c_int - disks_played;
    best_move = 0 as libc::c_int;
    game_evaluated_count = 0 as libc::c_int;
    reset_counter(&mut nodes);
    generate_all(side_to_move);
    if book_only != 0 || book != 0 {
        /* Evaluations for database moves */
        let mut flags = 0 as libc::c_int;
        if empties <= exact {
            flags = 16 as libc::c_int
        } else if empties <= wld { flags = 4 as libc::c_int }
        fill_move_alternatives(side_to_move, flags);
        game_evaluated_count = get_candidate_count();
        i = 0 as libc::c_int;
        while i < game_evaluated_count {
            let mut child_flags: libc::c_int = 0;
            book_move = get_candidate(i);
            evaluated_list[i as usize].side_to_move = side_to_move;
            evaluated_list[i as usize].move_0 = book_move.move_0;
            evaluated_list[i as usize].pv_depth = 1 as libc::c_int;
            evaluated_list[i as usize].pv[0 as libc::c_int as usize] =
                book_move.move_0;
            evaluated_list[i as usize].eval =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 book_move.score, 0.0f64, 0 as libc::c_int,
                                 1 as libc::c_int);
            child_flags = book_move.flags & book_move.parent_flags;
            if child_flags & (16 as libc::c_int | 4 as libc::c_int) != 0 {
                if child_flags & 16 as libc::c_int != 0 {
                    evaluated_list[i as usize].eval.type_0 = EXACT_EVAL
                } else { evaluated_list[i as usize].eval.type_0 = WLD_EVAL }
                if book_move.score > 0 as libc::c_int {
                    evaluated_list[i as usize].eval.res = WON_POSITION;
                    /* Normalize the scores so that e.g. 33-31 becomes +256 */
                    evaluated_list[i as usize].eval.score -=
                        30000 as libc::c_int;
                    evaluated_list[i as usize].eval.score *=
                        128 as libc::c_int
                } else if book_move.score == 0 as libc::c_int {
                    evaluated_list[i as usize].eval.res = DRAWN_POSITION
                } else {
                    /* score < 0 */
                    evaluated_list[i as usize].eval.res = LOST_POSITION;
                    /* Normalize the scores so that e.g. 30-34 becomes -512 */
                    evaluated_list[i as usize].eval.score +=
                        30000 as libc::c_int;
                    evaluated_list[i as usize].eval.score *=
                        128 as libc::c_int
                }
            } else { evaluated_list[i as usize].eval.type_0 = MIDGAME_EVAL }
            i += 1
        }
    }
    if book_only != 0 {
        /* Only book moves are to be considered */
        if game_evaluated_count > 0 as libc::c_int {
            best_move =
                get_book_move(side_to_move, 0 as libc::c_int,
                              &mut book_eval_info);
            set_current_eval(book_eval_info);
        } else {
            pv_depth[0 as libc::c_int as usize] = 0 as libc::c_int;
            best_move = -(1 as libc::c_int);
            book_eval_info =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 0 as libc::c_int, 0.0f64, 0 as libc::c_int,
                                 0 as libc::c_int);
            set_current_eval(book_eval_info);
        }
    } else {
        /* Make searches for moves not in the database */
        let mut shallow_depth: libc::c_int = 0;
        let mut empties_0 = 60 as libc::c_int - disks_played;
        book = 0 as libc::c_int;
        best_score = -(12345678 as libc::c_int);
        if game_evaluated_count > 0 as libc::c_int {
            /* Book PV available */
            best_score = evaluated_list[0 as libc::c_int as usize].eval.score;
            best_move = evaluated_list[0 as libc::c_int as usize].move_0
        }
        negate_current_eval(1 as libc::c_int);
        /* Store the available moves, clear their evaluations and sort
           them on shallow evaluation. */
        if empties_0 < 12 as libc::c_int {
            shallow_depth = 1 as libc::c_int
        } else {
            let mut max_depth =
                if mid > (if exact > wld { exact } else { wld }) {
                    mid
                } else if exact > wld { exact } else { wld };
            if max_depth >= 16 as libc::c_int {
                shallow_depth = 6 as libc::c_int
            } else { shallow_depth = 4 as libc::c_int }
        }
        unsearched_count = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < move_count[disks_played as usize] {
            this_move = move_list[disks_played as usize][i as usize];
            unsearched = 1 as libc::c_int;
            j = 0 as libc::c_int;
            while j < game_evaluated_count {
                if evaluated_list[j as usize].move_0 == this_move {
                    unsearched = 0 as libc::c_int
                }
                j += 1
            }
            if !(unsearched == 0) {
                unsearched_move[unsearched_count as usize] = this_move;
                unsearched_count += 1;
                make_move(side_to_move, this_move, 1 as libc::c_int);
                if shallow_depth == 1 as libc::c_int {
                    /* Compute move doesn't allow depth 0 */
                    evaluations.lo = evaluations.lo.wrapping_add(1);
                    shallow_eval =
                        -pattern_evaluation(0 as libc::c_int +
                                                2 as libc::c_int -
                                                side_to_move)
                } else {
                    let mut shallow_info =
                        EvaluationType{type_0: MIDGAME_EVAL,
                                       res: WON_POSITION,
                                       score: 0,
                                       confidence: 0.,
                                       search_depth: 0,
                                       is_book: 0,};
                    compute_move(0 as libc::c_int + 2 as libc::c_int -
                                     side_to_move, 0 as libc::c_int,
                                 0 as libc::c_int, 0 as libc::c_int,
                                 0 as libc::c_int, book,
                                 shallow_depth - 1 as libc::c_int,
                                 0 as libc::c_int, 0 as libc::c_int,
                                 1 as libc::c_int, &mut shallow_info);
                    if shallow_info.type_0 as libc::c_uint ==
                           PASS_EVAL as libc::c_int as libc::c_uint {
                        /* Don't allow pass */
                        compute_move(side_to_move, 0 as libc::c_int,
                                     0 as libc::c_int, 0 as libc::c_int,
                                     0 as libc::c_int, book,
                                     shallow_depth - 1 as libc::c_int,
                                     0 as libc::c_int, 0 as libc::c_int,
                                     1 as libc::c_int, &mut shallow_info);
                        if shallow_info.type_0 as libc::c_uint ==
                               PASS_EVAL as libc::c_int as libc::c_uint {
                            /* Game over */
                            disc_diff =
                                disc_count(side_to_move) -
                                    disc_count(0 as libc::c_int +
                                                   2 as libc::c_int -
                                                   side_to_move);
                            if disc_diff > 0 as libc::c_int {
                                corrected_diff =
                                    64 as libc::c_int -
                                        2 as libc::c_int *
                                            disc_count(0 as libc::c_int +
                                                           2 as libc::c_int -
                                                           side_to_move)
                            } else if disc_diff == 0 as libc::c_int {
                                corrected_diff = 0 as libc::c_int
                            } else {
                                corrected_diff =
                                    2 as libc::c_int *
                                        disc_count(side_to_move) -
                                        64 as libc::c_int
                            }
                            shallow_eval = 128 as libc::c_int * corrected_diff
                        } else { shallow_eval = shallow_info.score }
                    } else {
                        /* Sign-correct the score produced */
                        shallow_eval = -shallow_info.score
                    }
                }
                unmake_move(side_to_move, this_move);
                evals[disks_played as usize][this_move as usize] =
                    shallow_eval
            }
            i += 1
        }
        loop  {
            changed = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < unsearched_count - 1 as libc::c_int {
                if evals[disks_played as
                             usize][unsearched_move[i as usize] as usize] <
                       evals[disks_played as
                                 usize][unsearched_move[(i + 1 as libc::c_int)
                                                            as usize] as
                                            usize] {
                    temp_move = unsearched_move[i as usize];
                    unsearched_move[i as usize] =
                        unsearched_move[(i + 1 as libc::c_int) as usize];
                    unsearched_move[(i + 1 as libc::c_int) as usize] =
                        temp_move;
                    changed = 1 as libc::c_int
                }
                i += 1
            }
            if !(changed != 0) { break ; }
        }
        /* Initialize the entire list as being empty */
        i = 0 as libc::c_int;
        index = game_evaluated_count;
        while i < unsearched_count {
            evaluated_list[index as usize].side_to_move = side_to_move;
            evaluated_list[index as usize].move_0 =
                unsearched_move[i as usize];
            evaluated_list[index as usize].eval =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 0 as libc::c_int, 0.0f64, 0 as libc::c_int,
                                 0 as libc::c_int);
            evaluated_list[index as usize].pv_depth = 1 as libc::c_int;
            evaluated_list[index as usize].pv[0 as libc::c_int as usize] =
                unsearched_move[i as usize];
            if empties_0 > (if wld > exact { wld } else { exact }) {
                transform1[i as usize] =
                    abs(my_random() as libc::c_int) as libc::c_uint;
                transform2[i as usize] =
                    abs(my_random() as libc::c_int) as libc::c_uint
            } else {
                transform1[i as usize] = 0 as libc::c_int as libc::c_uint;
                transform2[i as usize] = 0 as libc::c_int as libc::c_uint
            }
            i += 1;
            index += 1
        }
        stored_echo = echo;
        echo = 0 as libc::c_int;
        best_pv_depth = 0 as libc::c_int;
        if mid == 1 as libc::c_int {
            /* compute_move won't be called */
            pv_depth[0 as libc::c_int as usize] = 0 as libc::c_int;
            piece_count[0 as libc::c_int as usize][disks_played as usize] =
                disc_count(0 as libc::c_int);
            piece_count[2 as libc::c_int as usize][disks_played as usize] =
                disc_count(2 as libc::c_int)
        }
        /* Perform iterative deepening if the search depth is large enough */
        if exact > empties_0 { exact = empties_0 }
        if exact < 12 as libc::c_int || empties_0 > exact {
            current_exact = exact
        } else {
            current_exact =
                8 as libc::c_int + exact % 2 as libc::c_int - 2 as libc::c_int
        }
        if wld > empties_0 { wld = empties_0 }
        if wld < 14 as libc::c_int || empties_0 > wld {
            current_wld = wld
        } else {
            current_wld =
                10 as libc::c_int + wld % 2 as libc::c_int - 2 as libc::c_int
        }
        if (empties_0 == exact || empties_0 == wld) &&
               empties_0 > 16 as libc::c_int &&
               mid < empties_0 - 12 as libc::c_int {
            mid = empties_0 - 12 as libc::c_int
        }
        if mid < 10 as libc::c_int {
            current_mid = mid
        } else {
            current_mid =
                6 as libc::c_int + mid % 2 as libc::c_int - 2 as libc::c_int
        }
        first_iteration = 1 as libc::c_int;
        loop  {
            if current_mid < mid {
                current_mid += 2 as libc::c_int;
                /* Avoid performing deep midgame searches if the endgame
                   is reached anyway. */
                if empties_0 <= wld &&
                       current_mid + 7 as libc::c_int >= empties_0 {
                    current_wld = wld;
                    current_mid = mid
                }
                if empties_0 <= exact &&
                       current_mid + 7 as libc::c_int >= empties_0 {
                    current_exact = exact;
                    current_mid = mid
                }
            } else if current_wld < wld {
                current_wld = wld
            } else { current_exact = exact }
            i = 0 as libc::c_int;
            while i < unsearched_count && force_return == 0 {
                let mut this_eval =
                    EvaluationType{type_0: MIDGAME_EVAL,
                                   res: WON_POSITION,
                                   score: 0,
                                   confidence: 0.,
                                   search_depth: 0,
                                   is_book: 0,};
                this_move = unsearched_move[i as usize];
                /* Locate the current move in the list.  This has to be done
                   because the moves might have been reordered during the
                   iterative deepening. */
                index = 0 as libc::c_int;
                while evaluated_list[index as usize].move_0 != this_move {
                    index += 1
                }
                /* To avoid strange effects when browsing back and forth through
                   a game during the midgame, rehash the hash transformation masks
                   for each move unless the endgame is reached */
                set_hash_transformation(transform1[i as usize],
                                        transform2[i as usize]);
                /* Determine the score for the ith move */
                prefix_move = this_move;
                make_move(side_to_move, this_move, 1 as libc::c_int);
                if current_mid == 1 as libc::c_int {
                    /* compute_move doesn't like 0-ply searches */
                    evaluations.lo = evaluations.lo.wrapping_add(1);
                    shallow_eval =
                        pattern_evaluation(0 as libc::c_int + 2 as libc::c_int
                                               - side_to_move);
                    this_eval =
                        create_eval_info(MIDGAME_EVAL, UNSOLVED_POSITION,
                                         shallow_eval, 0.0f64,
                                         0 as libc::c_int, 0 as libc::c_int)
                } else {
                    compute_move(0 as libc::c_int + 2 as libc::c_int -
                                     side_to_move, 0 as libc::c_int,
                                 0 as libc::c_int, 0 as libc::c_int,
                                 0 as libc::c_int, book,
                                 current_mid - 1 as libc::c_int,
                                 current_exact - 1 as libc::c_int,
                                 current_wld - 1 as libc::c_int,
                                 1 as libc::c_int, &mut this_eval);
                }
                if force_return != 0 {
                    /* Clear eval and exit search immediately */
                    this_eval =
                        create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                         0 as libc::c_int, 0.0f64,
                                         0 as libc::c_int, 0 as libc::c_int);
                    unmake_move(side_to_move, this_move);
                    break ;
                } else {
                    if this_eval.type_0 as libc::c_uint ==
                           PASS_EVAL as libc::c_int as libc::c_uint {
                        /* Don't allow pass */
                        if current_mid == 1 as libc::c_int {
                            /* compute_move doesn't like 0-ply searches */
                            evaluations.lo = evaluations.lo.wrapping_add(1);
                            shallow_eval = pattern_evaluation(side_to_move);
                            this_eval =
                                create_eval_info(MIDGAME_EVAL,
                                                 UNSOLVED_POSITION,
                                                 shallow_eval, 0.0f64,
                                                 0 as libc::c_int,
                                                 0 as libc::c_int)
                        } else {
                            compute_move(side_to_move, 0 as libc::c_int,
                                         0 as libc::c_int, 0 as libc::c_int,
                                         0 as libc::c_int, book,
                                         current_mid - 1 as libc::c_int,
                                         current_exact - 1 as libc::c_int,
                                         current_wld - 1 as libc::c_int,
                                         1 as libc::c_int, &mut this_eval);
                        }
                        if this_eval.type_0 as libc::c_uint ==
                               PASS_EVAL as libc::c_int as libc::c_uint {
                            /* Game over */
                            disc_diff =
                                disc_count(side_to_move) -
                                    disc_count(0 as libc::c_int +
                                                   2 as libc::c_int -
                                                   side_to_move);
                            if disc_diff > 0 as libc::c_int {
                                corrected_diff =
                                    64 as libc::c_int -
                                        2 as libc::c_int *
                                            disc_count(0 as libc::c_int +
                                                           2 as libc::c_int -
                                                           side_to_move);
                                res = WON_POSITION
                            } else if disc_diff == 0 as libc::c_int {
                                corrected_diff = 0 as libc::c_int;
                                res = DRAWN_POSITION
                            } else {
                                corrected_diff =
                                    2 as libc::c_int *
                                        disc_count(side_to_move) -
                                        64 as libc::c_int;
                                res = LOST_POSITION
                            }
                            this_eval =
                                create_eval_info(EXACT_EVAL, res,
                                                 128 as libc::c_int *
                                                     corrected_diff, 0.0f64,
                                                 60 as libc::c_int -
                                                     disks_played,
                                                 0 as libc::c_int)
                        }
                    } else {
                        /* Sign-correct the score produced */
                        this_eval.score = -this_eval.score;
                        if this_eval.res as libc::c_uint ==
                               WON_POSITION as libc::c_int as libc::c_uint {
                            this_eval.res = LOST_POSITION
                        } else if this_eval.res as libc::c_uint ==
                                      LOST_POSITION as libc::c_int as
                                          libc::c_uint {
                            this_eval.res = WON_POSITION
                        }
                    }
                    if force_return != 0 { break ; }
                    evaluated_list[index as usize].eval = this_eval;
                    /* Store the PV corresponding to the move */
                    evaluated_list[index as usize].pv_depth =
                        pv_depth[0 as libc::c_int as usize] +
                            1 as libc::c_int;
                    evaluated_list[index as
                                       usize].pv[0 as libc::c_int as usize] =
                        this_move;
                    j = 0 as libc::c_int;
                    while j < pv_depth[0 as libc::c_int as usize] {
                        evaluated_list[index as
                                           usize].pv[(j + 1 as libc::c_int) as
                                                         usize] =
                            pv[0 as libc::c_int as usize][j as usize];
                        j += 1
                    }
                    /* Store the PV corresponding to the best move */
                    if evaluated_list[index as usize].eval.score > best_score
                       {
                        best_score =
                            evaluated_list[index as usize].eval.score;
                        best_move = this_move;
                        best_pv_depth = pv_depth[0 as libc::c_int as usize];
                        j = 0 as libc::c_int;
                        while j < best_pv_depth {
                            best_pv[j as usize] =
                                pv[0 as libc::c_int as usize][j as usize];
                            j += 1
                        }
                    }
                    unmake_move(side_to_move, this_move);
                    /* Sort the moves evaluated */
                    if first_iteration != 0 { game_evaluated_count += 1 }
                    if force_return == 0 {
                        loop  {
                            changed = 0 as libc::c_int;
                            j = 0 as libc::c_int;
                            while j < game_evaluated_count - 1 as libc::c_int
                                  {
                                if compare_eval(evaluated_list[j as
                                                                   usize].eval,
                                                evaluated_list[(j +
                                                                    1 as
                                                                        libc::c_int)
                                                                   as
                                                                   usize].eval)
                                       < 0 as libc::c_int {
                                    changed = 1 as libc::c_int;
                                    temp = evaluated_list[j as usize];
                                    evaluated_list[j as usize] =
                                        evaluated_list[(j + 1 as libc::c_int)
                                                           as usize];
                                    evaluated_list[(j + 1 as libc::c_int) as
                                                       usize] = temp
                                }
                                j += 1
                            }
                            if !(changed != 0) { break ; }
                        }
                    }
                    i += 1
                }
            }
            first_iteration = 0 as libc::c_int;
            /* Reorder the moves after each iteration.  Each move is moved to
            the front of the list, starting with the bad moves and ending
             with the best move.  This ensures that unsearched_move will be
             sorted w.r.t. the order in evaluated_list. */
            i = game_evaluated_count - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                let mut this_move_0 = evaluated_list[i as usize].move_0;
                j = 0 as libc::c_int;
                while j != unsearched_count &&
                          unsearched_move[j as usize] != this_move_0 {
                    j += 1
                }
                if !(j == unsearched_count) {
                    /* Move the move to the front of the list. */
                    while j >= 1 as libc::c_int {
                        unsearched_move[j as usize] =
                            unsearched_move[(j - 1 as libc::c_int) as usize];
                        j -= 1
                    }
                    unsearched_move[0 as libc::c_int as usize] = this_move_0
                }
                /* Must be book move, skip */
                i -= 1
            }
            if !(force_return == 0 &&
                     (current_mid != mid || current_exact != exact ||
                          current_wld != wld)) {
                break ;
            }
        }
        echo = stored_echo;
        game_evaluated_count = move_count[disks_played as usize];
        /* Make sure that the PV and the score correspond to the best move */
        pv_depth[0 as libc::c_int as usize] =
            best_pv_depth + 1 as libc::c_int;
        pv[0 as libc::c_int as usize][0 as libc::c_int as usize] = best_move;
        i = 0 as libc::c_int;
        while i < best_pv_depth {
            pv[0 as libc::c_int as usize][(i + 1 as libc::c_int) as usize] =
                best_pv[i as usize];
            i += 1
        }
        negate_current_eval(0 as libc::c_int);
        if move_count[disks_played as usize] > 0 as libc::c_int {
            set_current_eval(evaluated_list[0 as libc::c_int as usize].eval);
        }
    }
    /* Reset the hash transformation masks prior to leaving */
    set_hash_transformation(0 as libc::c_int as libc::c_uint,
                            0 as libc::c_int as libc::c_uint);
    /* Don't forget to enable the time control mechanisms when leaving */
    toggle_abort_check(1 as libc::c_int);
    toggle_midgame_abort_check(1 as libc::c_int);
    toggle_perturbation_usage(1 as libc::c_int);
    max_depth_reached += 1;
    prefix_move = 0 as libc::c_int;
    return best_move;
}
/*
  PERFORM_EXTENDED_SOLVE
  Calculates exact score or WLD status for the move ACTUAL_MOVE as
  well as for the best move in the position (if it is any other move).
*/
#[no_mangle]
pub unsafe extern "C" fn perform_extended_solve(mut side_to_move: libc::c_int,
                                                mut actual_move: libc::c_int,
                                                mut book: libc::c_int,
                                                mut exact_solve:
                                                    libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut mid: libc::c_int = 0;
    let mut wld: libc::c_int = 0;
    let mut exact: libc::c_int = 0;
    let mut best_move: libc::c_int = 0;
    let mut disc_diff: libc::c_int = 0;
    let mut corrected_diff: libc::c_int = 0;
    let mut temp =
        EvaluatedMove{eval:
                          EvaluationType{type_0: MIDGAME_EVAL,
                                         res: WON_POSITION,
                                         score: 0,
                                         confidence: 0.,
                                         search_depth: 0,
                                         is_book: 0,},
                      side_to_move: 0,
                      move_0: 0,
                      pv_depth: 0,
                      pv: [0; 60],};
    let mut res = WON_POSITION;
    /* Disable all time control mechanisms */
    toggle_abort_check(0 as libc::c_int);
    toggle_midgame_abort_check(0 as libc::c_int);
    toggle_perturbation_usage(0 as libc::c_int);
    start_move(0 as libc::c_int as libc::c_double,
               0 as libc::c_int as libc::c_double,
               disc_count(0 as libc::c_int) + disc_count(2 as libc::c_int));
    clear_ponder_times();
    determine_hash_values(side_to_move, board.as_mut_ptr());
    reset_counter(&mut nodes);
    /* Set search depths that result in Zebra solving after a brief
       midgame analysis */
    mid = 60 as libc::c_int;
    wld = 60 as libc::c_int;
    if exact_solve != 0 {
        exact = 60 as libc::c_int
    } else { exact = 0 as libc::c_int }
    game_evaluated_count = 1 as libc::c_int;
    /* Calculate the score for the preferred move */
    evaluated_list[0 as libc::c_int as usize].side_to_move = side_to_move;
    evaluated_list[0 as libc::c_int as usize].move_0 = actual_move;
    evaluated_list[0 as libc::c_int as usize].eval =
        create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION, 0 as libc::c_int,
                         0.0f64, 0 as libc::c_int, 0 as libc::c_int);
    evaluated_list[0 as libc::c_int as usize].pv_depth = 1 as libc::c_int;
    evaluated_list[0 as libc::c_int as usize].pv[0 as libc::c_int as usize] =
        actual_move;
    prefix_move = actual_move;
    negate_current_eval(1 as libc::c_int);
    make_move(side_to_move, actual_move, 1 as libc::c_int);
    compute_move(0 as libc::c_int + 2 as libc::c_int - side_to_move,
                 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
                 0 as libc::c_int, book, mid - 1 as libc::c_int,
                 exact - 1 as libc::c_int, wld - 1 as libc::c_int,
                 1 as libc::c_int,
                 &mut (*evaluated_list.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize)).eval);
    if evaluated_list[0 as libc::c_int as usize].eval.type_0 as libc::c_uint
           == PASS_EVAL as libc::c_int as libc::c_uint {
        /* Don't allow pass */
        compute_move(side_to_move, 0 as libc::c_int, 0 as libc::c_int,
                     0 as libc::c_int, 0 as libc::c_int, book,
                     mid - 1 as libc::c_int, exact - 1 as libc::c_int,
                     wld - 1 as libc::c_int, 1 as libc::c_int,
                     &mut (*evaluated_list.as_mut_ptr().offset(0 as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).eval);
        if evaluated_list[0 as libc::c_int as usize].eval.type_0 as
               libc::c_uint == PASS_EVAL as libc::c_int as libc::c_uint {
            /* Game has ended */
            disc_diff =
                disc_count(side_to_move) -
                    disc_count(0 as libc::c_int + 2 as libc::c_int -
                                   side_to_move);
            if disc_diff > 0 as libc::c_int {
                corrected_diff =
                    64 as libc::c_int -
                        2 as libc::c_int *
                            disc_count(0 as libc::c_int + 2 as libc::c_int -
                                           side_to_move);
                res = WON_POSITION
            } else if disc_diff == 0 as libc::c_int {
                corrected_diff = 0 as libc::c_int;
                res = DRAWN_POSITION
            } else {
                corrected_diff =
                    2 as libc::c_int * disc_count(side_to_move) -
                        64 as libc::c_int;
                res = LOST_POSITION
            }
            evaluated_list[0 as libc::c_int as usize].eval =
                create_eval_info(EXACT_EVAL, res,
                                 128 as libc::c_int * corrected_diff, 0.0f64,
                                 60 as libc::c_int - disks_played,
                                 0 as libc::c_int)
        }
    } else {
        /* Sign-correct the score produced */
        evaluated_list[0 as libc::c_int as usize].eval.score =
            -evaluated_list[0 as libc::c_int as usize].eval.score;
        if evaluated_list[0 as libc::c_int as usize].eval.res as libc::c_uint
               == WON_POSITION as libc::c_int as libc::c_uint {
            evaluated_list[0 as libc::c_int as usize].eval.res = LOST_POSITION
        } else if evaluated_list[0 as libc::c_int as usize].eval.res as
                      libc::c_uint ==
                      LOST_POSITION as libc::c_int as libc::c_uint {
            evaluated_list[0 as libc::c_int as usize].eval.res = WON_POSITION
        }
    }
    if force_return != 0 {
        evaluated_list[0 as libc::c_int as usize].eval =
            create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                             0 as libc::c_int, 0.0f64, 0 as libc::c_int,
                             0 as libc::c_int)
    } else {
        evaluated_list[0 as libc::c_int as usize].pv_depth =
            pv_depth[0 as libc::c_int as usize] + 1 as libc::c_int;
        evaluated_list[0 as libc::c_int as
                           usize].pv[0 as libc::c_int as usize] = actual_move;
        i = 0 as libc::c_int;
        while i < pv_depth[0 as libc::c_int as usize] {
            evaluated_list[0 as libc::c_int as
                               usize].pv[(i + 1 as libc::c_int) as usize] =
                pv[0 as libc::c_int as usize][i as usize];
            i += 1
        }
    }
    unmake_move(side_to_move, actual_move);
    prefix_move = 0 as libc::c_int;
    negate_current_eval(0 as libc::c_int);
    max_depth_reached += 1;
    /* Compute the score for the best move and store it in the move list
       if it isn't ACTUAL_MOVE */
    best_move =
        compute_move(side_to_move, 0 as libc::c_int, 0 as libc::c_int,
                     0 as libc::c_int, 0 as libc::c_int, book, mid, exact,
                     wld, 1 as libc::c_int,
                     &mut (*evaluated_list.as_mut_ptr().offset(1 as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).eval);
    if force_return == 0 && best_move != actual_move {
        /* Move list will contain best move first and then the actual move */
        game_evaluated_count = 2 as libc::c_int;
        evaluated_list[1 as libc::c_int as usize].side_to_move = side_to_move;
        evaluated_list[1 as libc::c_int as usize].move_0 = best_move;
        evaluated_list[1 as libc::c_int as usize].pv_depth =
            pv_depth[0 as libc::c_int as usize];
        i = 0 as libc::c_int;
        while i < pv_depth[0 as libc::c_int as usize] {
            evaluated_list[1 as libc::c_int as usize].pv[i as usize] =
                pv[0 as libc::c_int as usize][i as usize];
            i += 1
        }
        temp = evaluated_list[0 as libc::c_int as usize];
        evaluated_list[0 as libc::c_int as usize] =
            evaluated_list[1 as libc::c_int as usize];
        evaluated_list[1 as libc::c_int as usize] = temp
    }
    /* The PV and current eval should when correspond to the best move
       when leaving */
    pv_depth[0 as libc::c_int as usize] =
        evaluated_list[0 as libc::c_int as usize].pv_depth;
    i = 0 as libc::c_int;
    while i < pv_depth[0 as libc::c_int as usize] {
        pv[0 as libc::c_int as usize][i as usize] =
            evaluated_list[0 as libc::c_int as usize].pv[i as usize];
        i += 1
    }
    set_current_eval(evaluated_list[0 as libc::c_int as usize].eval);
    /* Don't forget to enable the time control mechanisms when leaving */
    toggle_abort_check(1 as libc::c_int);
    toggle_midgame_abort_check(1 as libc::c_int);
    toggle_perturbation_usage(0 as libc::c_int);
}
/*
  GET_EVALUATED_COUNT
  GET_EVALUATED
  Accessor functions for the data structure filled by extended_compute_move().
*/
#[no_mangle]
pub unsafe extern "C" fn get_evaluated_count() -> libc::c_int {
    return game_evaluated_count;
}
#[no_mangle]
pub unsafe extern "C" fn get_evaluated(mut index: libc::c_int)
 -> EvaluatedMove {
    return evaluated_list[index as usize];
}
/*
   COMPUTE_MOVE
   Returns the best move in a position given search parameters.
*/
#[no_mangle]
pub unsafe extern "C" fn compute_move(mut side_to_move: libc::c_int,
                                      mut update_all: libc::c_int,
                                      mut my_time: libc::c_int,
                                      mut my_incr: libc::c_int,
                                      mut timed_depth: libc::c_int,
                                      mut book: libc::c_int,
                                      mut mid: libc::c_int,
                                      mut exact: libc::c_int,
                                      mut wld: libc::c_int,
                                      mut search_forced: libc::c_int,
                                      mut eval_info: *mut EvaluationType)
 -> libc::c_int {
    let mut log_file = 0 as *mut FILE;
    let mut book_eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut mid_eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut end_eval_info =
        EvaluationType{type_0: MIDGAME_EVAL,
                       res: WON_POSITION,
                       score: 0,
                       confidence: 0.,
                       search_depth: 0,
                       is_book: 0,};
    let mut eval_str = 0 as *mut libc::c_char;
    let mut midgame_diff: libc::c_double = 0.;
    let mut move_type = INTERRUPTED_MOVE;
    let mut i: libc::c_int = 0;
    let mut curr_move: libc::c_int = 0;
    let mut midgame_move: libc::c_int = 0;
    let mut empties: libc::c_int = 0;
    let mut midgame_depth: libc::c_int = 0;
    let mut interrupted_depth: libc::c_int = 0;
    let mut max_depth: libc::c_int = 0;
    let mut book_move_found: libc::c_int = 0;
    let mut endgame_reached: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    log_file = 0 as *mut FILE;
    if use_log_file != 0 {
        log_file =
            fopen(log_file_path.as_mut_ptr(),
                  b"a\x00" as *const u8 as *const libc::c_char)
    }
    if !log_file.is_null() {
        display_board(log_file, board.as_mut_ptr(), side_to_move,
                      0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
    }
    /* Initialize various components of the move system */
    piece_count[0 as libc::c_int as usize][disks_played as usize] =
        disc_count(0 as libc::c_int);
    piece_count[2 as libc::c_int as usize][disks_played as usize] =
        disc_count(2 as libc::c_int);
    init_moves();
    generate_all(side_to_move);
    determine_hash_values(side_to_move, board.as_mut_ptr());
    calculate_perturbation();
    if !log_file.is_null() {
        fprintf(log_file, b"%d %s: \x00" as *const u8 as *const libc::c_char,
                move_count[disks_played as usize],
                b"moves generated\x00" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < move_count[disks_played as usize] {
            fprintf(log_file,
                    b"%c%c \x00" as *const u8 as *const libc::c_char,
                    'a' as i32 +
                        move_list[disks_played as usize][i as usize] %
                            10 as libc::c_int - 1 as libc::c_int,
                    '0' as i32 +
                        move_list[disks_played as usize][i as usize] /
                            10 as libc::c_int);
            i += 1
        }
        fputs(b"\n\x00" as *const u8 as *const libc::c_char, log_file);
    }
    if update_all != 0 {
        reset_counter(&mut evaluations);
        reset_counter(&mut nodes);
    }
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        evals[disks_played as usize][i as usize] = 0 as libc::c_int;
        i += 1
    }
    max_depth_reached = 1 as libc::c_int;
    empties = 60 as libc::c_int - disks_played;
    reset_buffer_display();
    determine_move_time(my_time as libc::c_double, my_incr as libc::c_double,
                        disks_played + 4 as libc::c_int);
    if get_ponder_move() == 0 { clear_ponder_times(); }
    remove_coeffs(disks_played);
    /* No feasible moves? */
    if move_count[disks_played as usize] == 0 as libc::c_int {
        *eval_info =
            create_eval_info(PASS_EVAL, UNSOLVED_POSITION,
                             0.0f64 as libc::c_int, 0.0f64, 0 as libc::c_int,
                             0 as libc::c_int);
        set_current_eval(*eval_info);
        if echo != 0 {
            eval_str = produce_eval_text(*eval_info, 0 as libc::c_int);
            send_status(b"-->         \x00" as *const u8 as
                            *const libc::c_char);
            send_status(b"%-8s  \x00" as *const u8 as *const libc::c_char,
                        eval_str);
            display_status(stdout, 0 as libc::c_int);
            free(eval_str as *mut libc::c_void);
        }
        if !log_file.is_null() {
            fprintf(log_file,
                    b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
                    b"Best move\x00" as *const u8 as *const libc::c_char,
                    b"pass\x00" as *const u8 as *const libc::c_char);
            fclose(log_file);
        }
        last_time_used = 0.0f64;
        clear_pv();
        return -(1 as libc::c_int)
    }
    /* If there is only one move available:
       Don't waste any time, unless told so or very close to the end,
       searching the position. */
    if empties > 60 as libc::c_int &&
           move_count[disks_played as usize] == 1 as libc::c_int &&
           search_forced == 0 {
        /* Forced move */
        *eval_info =
            create_eval_info(FORCED_EVAL, UNSOLVED_POSITION,
                             0.0f64 as libc::c_int, 0.0f64, 0 as libc::c_int,
                             0 as libc::c_int);
        set_current_eval(*eval_info);
        if echo != 0 {
            eval_str = produce_eval_text(*eval_info, 0 as libc::c_int);
            send_status(b"-->         \x00" as *const u8 as
                            *const libc::c_char);
            send_status(b"%-8s  \x00" as *const u8 as *const libc::c_char,
                        eval_str);
            free(eval_str as *mut libc::c_void);
            send_status(b"%c%c \x00" as *const u8 as *const libc::c_char,
                        'a' as i32 +
                            move_list[disks_played as
                                          usize][0 as libc::c_int as usize] %
                                10 as libc::c_int - 1 as libc::c_int,
                        '0' as i32 +
                            move_list[disks_played as
                                          usize][0 as libc::c_int as usize] /
                                10 as libc::c_int);
            display_status(stdout, 0 as libc::c_int);
        }
        if !log_file.is_null() {
            fprintf(log_file,
                    b"%s: %c%c  (%s)\n\x00" as *const u8 as
                        *const libc::c_char,
                    b"Best move\x00" as *const u8 as *const libc::c_char,
                    'a' as i32 +
                        move_list[disks_played as
                                      usize][0 as libc::c_int as usize] %
                            10 as libc::c_int - 1 as libc::c_int,
                    '0' as i32 +
                        move_list[disks_played as
                                      usize][0 as libc::c_int as usize] /
                            10 as libc::c_int,
                    b"forced\x00" as *const u8 as *const libc::c_char);
            fclose(log_file);
        }
        last_time_used = 0.0f64;
        return move_list[disks_played as usize][0 as libc::c_int as usize]
    }
    /* Mark the search as interrupted until a successful search
       has been performed. */
    move_type = INTERRUPTED_MOVE;
    interrupted_depth = 0 as libc::c_int;
    curr_move = move_list[disks_played as usize][0 as libc::c_int as usize];
    /* Check the opening book for midgame moves */
    book_move_found = 0 as libc::c_int;
    midgame_move = -(1 as libc::c_int);
    if !forced_opening.is_null() {
        /* Check if the position fits the currently forced opening */
        curr_move = check_forced_opening(side_to_move, forced_opening);
        if curr_move != -(1 as libc::c_int) {
            book_eval_info =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 0 as libc::c_int, 0.0f64, 0 as libc::c_int,
                                 1 as libc::c_int);
            midgame_move = curr_move;
            book_move_found = 1 as libc::c_int;
            move_type = BOOK_MOVE;
            if echo != 0 {
                send_status(b"-->   Forced opening move        \x00" as
                                *const u8 as *const libc::c_char);
                if get_ponder_move() != 0 {
                    send_status(b"{%c%c} \x00" as *const u8 as
                                    *const libc::c_char,
                                'a' as i32 +
                                    get_ponder_move() % 10 as libc::c_int -
                                    1 as libc::c_int,
                                '0' as i32 +
                                    get_ponder_move() / 10 as libc::c_int);
                }
                send_status(b"%c%c\x00" as *const u8 as *const libc::c_char,
                            'a' as i32 + curr_move % 10 as libc::c_int -
                                1 as libc::c_int,
                            '0' as i32 + curr_move / 10 as libc::c_int);
                display_status(stdout, 0 as libc::c_int);
            }
            clear_pv();
            pv_depth[0 as libc::c_int as usize] = 1 as libc::c_int;
            pv[0 as libc::c_int as usize][0 as libc::c_int as usize] =
                curr_move
        }
    }
    if book_move_found == 0 && play_thor_match_openings != 0 {
        /* Optionally use the Thor database as opening book. */
        let mut threshold = 2 as libc::c_int;
        database_search(board.as_mut_ptr(), side_to_move);
        if get_match_count() >= threshold {
            let mut game_index =
                ((my_random() >> 8 as libc::c_int) %
                     get_match_count() as libc::c_long) as libc::c_int;
            curr_move = get_thor_game_move(game_index, disks_played);
            if valid_move(curr_move, side_to_move) != 0 {
                book_eval_info =
                    create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                     0 as libc::c_int, 0.0f64,
                                     0 as libc::c_int, 1 as libc::c_int);
                midgame_move = curr_move;
                book_move_found = 1 as libc::c_int;
                move_type = BOOK_MOVE;
                if echo != 0 {
                    send_status(b"-->   %s        \x00" as *const u8 as
                                    *const libc::c_char,
                                b"Thor database\x00" as *const u8 as
                                    *const libc::c_char);
                    if get_ponder_move() != 0 {
                        send_status(b"{%c%c} \x00" as *const u8 as
                                        *const libc::c_char,
                                    'a' as i32 +
                                        get_ponder_move() % 10 as libc::c_int
                                        - 1 as libc::c_int,
                                    '0' as i32 +
                                        get_ponder_move() /
                                            10 as libc::c_int);
                    }
                    send_status(b"%c%c\x00" as *const u8 as
                                    *const libc::c_char,
                                'a' as i32 + curr_move % 10 as libc::c_int -
                                    1 as libc::c_int,
                                '0' as i32 + curr_move / 10 as libc::c_int);
                    display_status(stdout, 0 as libc::c_int);
                }
                clear_pv();
                pv_depth[0 as libc::c_int as usize] = 1 as libc::c_int;
                pv[0 as libc::c_int as usize][0 as libc::c_int as usize] =
                    curr_move
            } else {
                fatal_error(b"Thor book move %d is invalid!\x00" as *const u8
                                as *const libc::c_char, curr_move);
            }
        }
    }
    if book_move_found == 0 && play_human_openings != 0 && book != 0 {
        /* Check Thor statistics for a move */
        curr_move =
            choose_thor_opening_move(board.as_mut_ptr(), side_to_move,
                                     0 as libc::c_int);
        if curr_move != -(1 as libc::c_int) {
            book_eval_info =
                create_eval_info(UNDEFINED_EVAL, UNSOLVED_POSITION,
                                 0 as libc::c_int, 0.0f64, 0 as libc::c_int,
                                 1 as libc::c_int);
            midgame_move = curr_move;
            book_move_found = 1 as libc::c_int;
            move_type = BOOK_MOVE;
            if echo != 0 {
                send_status(b"-->   %s        \x00" as *const u8 as
                                *const libc::c_char,
                            b"Thor database\x00" as *const u8 as
                                *const libc::c_char);
                if get_ponder_move() != 0 {
                    send_status(b"{%c%c} \x00" as *const u8 as
                                    *const libc::c_char,
                                'a' as i32 +
                                    get_ponder_move() % 10 as libc::c_int -
                                    1 as libc::c_int,
                                '0' as i32 +
                                    get_ponder_move() / 10 as libc::c_int);
                }
                send_status(b"%c%c\x00" as *const u8 as *const libc::c_char,
                            'a' as i32 + curr_move % 10 as libc::c_int -
                                1 as libc::c_int,
                            '0' as i32 + curr_move / 10 as libc::c_int);
                display_status(stdout, 0 as libc::c_int);
            }
            clear_pv();
            pv_depth[0 as libc::c_int as usize] = 1 as libc::c_int;
            pv[0 as libc::c_int as usize][0 as libc::c_int as usize] =
                curr_move
        }
    }
    if book_move_found == 0 && book != 0 {
        /* Check ordinary opening book */
        let mut flags = 0 as libc::c_int;
        if empties <= 30 as libc::c_int {
            if empties <= wld { flags = 4 as libc::c_int }
            if empties <= exact { flags = 16 as libc::c_int }
        }
        fill_move_alternatives(side_to_move, flags);
        curr_move =
            get_book_move(side_to_move, update_all, &mut book_eval_info);
        if curr_move != -(1 as libc::c_int) {
            set_current_eval(book_eval_info);
            midgame_move = curr_move;
            book_move_found = 1 as libc::c_int;
            move_type = BOOK_MOVE;
            display_status(stdout, 0 as libc::c_int);
        }
    }
    /* Use iterative deepening in the midgame searches until the endgame
       is reached. If an endgame search already has been performed,
       make a much more shallow midgame search. Also perform much more
       shallow searches when there is no time limit and hence no danger
       starting to solve only to get interrupted. */
    if timed_depth == 0 && empties <= (if exact > wld { exact } else { wld })
       {
        mid =
            if (if (if mid < empties - 7 as libc::c_int {
                        mid
                    } else { (empties) - 7 as libc::c_int }) <
                       28 as libc::c_int {
                    (if mid < empties - 7 as libc::c_int {
                         mid
                     } else { (empties) - 7 as libc::c_int })
                } else { 28 as libc::c_int }) > 2 as libc::c_int {
                if (if mid < empties - 7 as libc::c_int {
                        mid
                    } else { (empties) - 7 as libc::c_int }) <
                       28 as libc::c_int {
                    if mid < empties - 7 as libc::c_int {
                        mid
                    } else { (empties) - 7 as libc::c_int }
                } else { 28 as libc::c_int }
            } else { 2 as libc::c_int }
    }
    endgame_reached =
        (timed_depth == 0 && endgame_performed[side_to_move as usize] != 0) as
            libc::c_int;
    if book_move_found == 0 && endgame_reached == 0 {
        clear_panic_abort();
        clear_midgame_abort();
        toggle_midgame_abort_check(update_all);
        toggle_midgame_hash_usage(1 as libc::c_int, 1 as libc::c_int);
        if timed_depth != 0 {
            max_depth = 64 as libc::c_int
        } else if empties <= (if exact > wld { exact } else { wld }) {
            max_depth =
                if (if (if mid < empties - 12 as libc::c_int {
                            mid
                        } else { (empties) - 12 as libc::c_int }) <
                           18 as libc::c_int {
                        (if mid < empties - 12 as libc::c_int {
                             mid
                         } else { (empties) - 12 as libc::c_int })
                    } else { 18 as libc::c_int }) > 2 as libc::c_int {
                    if (if mid < empties - 12 as libc::c_int {
                            mid
                        } else { (empties) - 12 as libc::c_int }) <
                           18 as libc::c_int {
                        if mid < empties - 12 as libc::c_int {
                            mid
                        } else { (empties) - 12 as libc::c_int }
                    } else { 18 as libc::c_int }
                } else { 2 as libc::c_int }
        } else { max_depth = mid }
        midgame_depth =
            if (2 as libc::c_int) < max_depth {
                2 as libc::c_int
            } else { max_depth };
        loop  {
            max_depth_reached = midgame_depth;
            midgame_move =
                middle_game(side_to_move, midgame_depth, update_all,
                            &mut mid_eval_info);
            set_current_eval(mid_eval_info);
            midgame_diff =
                1.3f64 * mid_eval_info.score as libc::c_double / 128.0f64;
            if side_to_move == 0 as libc::c_int {
                midgame_diff -= komi as libc::c_double
            } else { midgame_diff += komi as libc::c_double }
            if timed_depth != 0 {
                /* Check if the endgame zone has been reached */
                offset = 7 as libc::c_int;
                /* These constants were chosen rather arbitrarily but intend
                   to make Zebra solve earlier if the position is lopsided. */
                if is_panic_abort() != 0 { offset -= 1 }
                if endgame_performed[side_to_move as usize] != 0 {
                    offset += 2 as libc::c_int
                }
                if midgame_depth + offset + 27 as libc::c_int >=
                       2 as libc::c_int * empties ||
                       midgame_depth + 7 as libc::c_int >= empties {
                    endgame_reached = 1 as libc::c_int
                }
            }
            midgame_depth += 1;
            if !(is_panic_abort() == 0 && is_midgame_abort() == 0 &&
                     force_return == 0 && midgame_depth <= max_depth &&
                     midgame_depth + disks_played <= 61 as libc::c_int &&
                     endgame_reached == 0) {
                break ;
            }
        }
        if echo != 0 { display_status(stdout, 0 as libc::c_int); }
        if abs(mid_eval_info.score) == abs(-(27000 as libc::c_int)) {
            move_type = INTERRUPTED_MOVE;
            interrupted_depth = midgame_depth - 1 as libc::c_int
            /* compensate for increment */
        } else { move_type = MIDGAME_MOVE }
    }
    curr_move = midgame_move;
    /* If the endgame has been reached, solve the position */
    if force_return == 0 {
        if timed_depth != 0 && endgame_reached != 0 ||
               timed_depth != 0 && book_move_found != 0 &&
                   disks_played >= 60 as libc::c_int - 30 as libc::c_int ||
               timed_depth == 0 &&
                   empties <= (if exact > wld { exact } else { wld }) {
            max_depth_reached = empties;
            clear_panic_abort();
            if timed_depth != 0 {
                curr_move =
                    end_game(side_to_move,
                             (disks_played < 60 as libc::c_int - exact) as
                                 libc::c_int, 0 as libc::c_int, book, komi,
                             &mut end_eval_info)
            } else if empties <= exact {
                curr_move =
                    end_game(side_to_move, 0 as libc::c_int, 0 as libc::c_int,
                             book, komi, &mut end_eval_info)
            } else {
                curr_move =
                    end_game(side_to_move, 1 as libc::c_int, 0 as libc::c_int,
                             book, komi, &mut end_eval_info)
            }
            set_current_eval(end_eval_info);
            if abs(root_eval) == abs(-(27000 as libc::c_int)) {
                move_type = INTERRUPTED_MOVE
            } else { move_type = ENDGAME_MOVE }
            if update_all != 0 {
                endgame_performed[side_to_move as usize] = 1 as libc::c_int
            }
        }
    }
    match move_type as libc::c_uint {
        0 => {
            *eval_info =
                create_eval_info(INTERRUPTED_EVAL, UNSOLVED_POSITION,
                                 0.0f64 as libc::c_int, 0.0f64,
                                 0 as libc::c_int, 0 as libc::c_int);
            clear_status();
            send_status(b"--> *%2d\x00" as *const u8 as *const libc::c_char,
                        interrupted_depth);
            eval_str = produce_eval_text(*eval_info, 1 as libc::c_int);
            send_status(b"%10s  \x00" as *const u8 as *const libc::c_char,
                        eval_str);
            free(eval_str as *mut libc::c_void);
            send_status_nodes(counter_value(&mut nodes));
            send_status_pv(pv[0 as libc::c_int as usize].as_mut_ptr(),
                           interrupted_depth);
            send_status_time(get_elapsed_time());
            if get_elapsed_time() != 0.0f64 {
                send_status(b"%6.0f %s\x00" as *const u8 as
                                *const libc::c_char,
                            counter_value(&mut nodes) /
                                (get_elapsed_time() + 0.001f64),
                            b"nps\x00" as *const u8 as *const libc::c_char);
            }
        }
        1 => { *eval_info = book_eval_info }
        2 => { *eval_info = mid_eval_info }
        3 => { *eval_info = end_eval_info }
        _ => { }
    }
    set_current_eval(*eval_info);
    last_time_used = get_elapsed_time();
    if update_all != 0 {
        total_time += last_time_used;
        add_counter(&mut total_evaluations, &mut evaluations);
        add_counter(&mut total_nodes, &mut nodes);
    }
    clear_panic_abort();
    /* Write the contents of the status buffer to the log file. */
    if move_type as libc::c_uint == BOOK_MOVE as libc::c_int as libc::c_uint {
        eval_str = produce_eval_text(*eval_info, 0 as libc::c_int);
        if !log_file.is_null() {
            fprintf(log_file,
                    b"%s: %c%c  %s\n\x00" as *const u8 as *const libc::c_char,
                    b"Move chosen\x00" as *const u8 as *const libc::c_char,
                    'a' as i32 + curr_move % 10 as libc::c_int -
                        1 as libc::c_int,
                    '0' as i32 + curr_move / 10 as libc::c_int, eval_str);
        }
        free(eval_str as *mut libc::c_void);
    } else if !log_file.is_null() {
        display_status(log_file, 1 as libc::c_int);
    }
    /* Write the principal variation, if available, to the log file
       and, optionally, to screen. */
    if get_ponder_move() == 0 {
        complete_pv(side_to_move);
        if display_pv != 0 && echo != 0 { display_optimal_line(stdout); }
        if !log_file.is_null() { display_optimal_line(log_file); }
    }
    if !log_file.is_null() { fclose(log_file); }
    return curr_move;
}
/*
  GET_SEARCH_STATISTICS
  Returns some statistics about the last search made.
*/
#[no_mangle]
pub unsafe extern "C" fn get_search_statistics(mut max_depth:
                                                   *mut libc::c_int,
                                               mut node_count:
                                                   *mut libc::c_double) {
    *max_depth = max_depth_reached;
    if prefix_move != 0 as libc::c_int { *max_depth += 1 }
    adjust_counter(&mut nodes);
    *node_count = counter_value(&mut nodes);
}
/*
  GET_PV
  Returns the principal variation.
*/
#[no_mangle]
pub unsafe extern "C" fn get_pv(mut destin: *mut libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if prefix_move == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < pv_depth[0 as libc::c_int as usize] {
            *destin.offset(i as isize) =
                pv[0 as libc::c_int as usize][i as usize];
            i += 1
        }
        return pv_depth[0 as libc::c_int as usize]
    } else {
        *destin.offset(0 as libc::c_int as isize) = prefix_move;
        i = 0 as libc::c_int;
        while i < pv_depth[0 as libc::c_int as usize] {
            *destin.offset((i + 1 as libc::c_int) as isize) =
                pv[0 as libc::c_int as usize][i as usize];
            i += 1
        }
        return pv_depth[0 as libc::c_int as usize] + 1 as libc::c_int
    };
}
