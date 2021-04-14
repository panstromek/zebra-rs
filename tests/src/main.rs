use std::fs::File;
use crate::tests::{snapshot_test_with_folder, Interactive};
use rand::Rng;
use std::fmt::Write;
use rand::rngs::ThreadRng;
use std::collections::vec_deque::VecDeque;

fn main() {
    let mut rng = rand::thread_rng();
    let mut boards = VecDeque::new();
    let mut sequences = VecDeque::new();
    loop {
        if let Ok(file) = std::fs::read_to_string("fuzzer/run_dir/current.gam") {
            boards.push_back(file);
            if boards.len() > 10 {
                boards.pop_front();
            }
        }
        if let Ok(file) = std::fs::read_to_string("fuzzer/run_dir/current.mov") {
            let file = file.split_whitespace().filter_map(|item| {
                let first = *item.as_bytes().get(0)?;
                let second = *item.as_bytes().get(1)?;
                match (item.len(), first, second) {
                    (2, b'a'..=b'h', b'1'..=b'8') => {
                        Some(item)
                    }
                    _ => None
                }

            }).collect::<String>();
            sequences.push_back(file);
            if sequences.len() > 10 {
                sequences.pop_front();
            }
        }

        std::fs::remove_dir_all("fuzzer");
        let binary_folder = "../../../zebra-1/";
        let binary = "zebra";
        let mut args = String::new();
        let interactive = match rng.gen_range::<i32, _>(0..6) {
            1 => Interactive::Dumb,
            _ => Interactive::None,
        };
        match interactive {
            Interactive::Dumb => {
                args.push_str("-r 0 -l 0 0");
            }
            _ => {
                write!(args, "-r 0 -l {} {} {} {} {} {}",
                    // TODO make smaller numbers more likely
                       rng.gen_range::<i32, _>(0..10),
                       rng.gen_range::<i32, _>(0..20),
                       rng.gen_range::<i32, _>(0..20),
                       rng.gen_range::<i32, _>(0..10),
                       rng.gen_range::<i32, _>(0..20),
                       rng.gen_range::<i32, _>(0..20),
                );
            }
        }


        let flags: &[(u32, &dyn Fn(&mut String, &mut ThreadRng))] = &[
            (12, &(|s, rng| { write!(s, "-public"); })),
            (12, &(|s, rng| { write!(s, "-private"); })),
            (12, &(|s, rng| { write!(s, "-keepdraw"); })),
            (12, &(|s, rng| { write!(s, "-draw2black"); })),
            (12, &(|s, rng| { write!(s, "-draw2white"); })),
            (12, &(|s, rng| { write!(s, "-draw2none"); })),
            (8, &(|s, rng| { write!(s, "-test"); })),
            (8, &(|s, rng| { write!(s, "-analyze"); })),
            //todo make small number of repeats more likely
            (8, &(|s, rng| { write!(s, "-repeat {}", rng.gen_range::<i32, _>(0..5)); })),
            (6, &(|s, rng| { write!(s, "-slack {}", rng.gen_range::<f32, _>(0.0..10.0)); })),
            (6, &(|s, rng| { write!(s, "-randmove {}", rng.gen_range::<i32, _>(0..10)); })),
            (2, &(|s, rng| { write!(s, "-p {}", rng.gen_range::<i32, _>(0..2)); })),
            (2, &(|s, rng| { write!(s, "-e {}", rng.gen_range::<i32, _>(0..2)); })),
            (2, &(|s, rng| { write!(s, "-b {}", rng.gen_range::<i32, _>(0..2)); })),
            (4, &(|s, rng| { write!(s, "-w {}", rng.gen_range::<i32, _>(0..2)); })),
            (4, &(|s, rng| { write!(s, "-thor {}", rng.gen_range::<i32, _>(0..20)); })),
            (4, &(|s, rng| { write!(s, "-wld {}", rng.gen_range::<i32, _>(0..2)); })),
            (4, &(|s, rng| { write!(s, "-h {}", rng.gen_range::<i32, _>(0..22)); })),
            (8, &(|s, rng| {
                write!(s, "-dev {} {} {}",
                       rng.gen_range::<i32, _>(0..100),
                       rng.gen_range::<i32, _>(0..100),
                       rng.gen_range::<f32, _>(0.0..220.0),
                );
            })),
            // NOTE: -t is mutually exclusive with -l
            //  (it doesn't matter too much now, because -t will just override it)
            //   (later args override previous ones)
            (8, &(|s, rng| {
                let number_of_levels = rng.gen_range::<i32, _>(0..4);
                write!(s, "-t {}", number_of_levels);
                for _ in 0..number_of_levels {
                    write!(s, " {} {} {}",
                           //todo make small numbers more likely
                           // TODO allow human player too (by specifying zero here)
                           rng.gen_range::<i32, _>(1..10),
                           rng.gen_range::<i32, _>(0..18),
                           rng.gen_range::<i32, _>(0..18),
                    );
                }
            })),
            // TODO test randomly generated boards
            (8, &(|s, rng| {
                if boards.is_empty() {
                    write!(s, "-g ../../tests/resources/board.txt");
                } else {
                    std::fs::create_dir_all("fuzzer-data").unwrap();
                    let string = &boards[rng.gen_range(0..boards.len())];
                    std::fs::write("fuzzer-data/board-fuzzer-1",string ).unwrap();
                    write!(s, "-g ../../fuzzer-data/board-fuzzer-1");
                }
            })),
            // TODO test more randomly generated games
            (4, &(|s, rng| {
                let arg = if sequences.is_empty() {
                    "e6f6f5f4e3d6g4d3c3h3c4g3g5g6c7c6c5b6d7b5f7f3b4f8h4h5f2f1h2h1"
                } else {
                    &sequences[rng.gen_range(0..sequences.len())]
                };
                let slice_to = if arg.len() == 0 { 0 } else { rng.gen_range(0..arg.len()) };
                s.push_str("-seq ");
                s.push_str(&arg[0..slice_to]);
            })),
            (8, &(|s, rng| {
                write!(s, "-time {} {} {} {}",
                       rng.gen_range(0..50i32),
                       rng.gen_range(0..50i32),
                       rng.gen_range(0..50i32),
                       rng.gen_range(0..50i32),
                );
            })),
            // todo
            //  -learn <depth> <cutoff>
            //     Learn the game with <depth> deviations up to <cutoff> empty.
            //  -log <file name>
            //     Append all game results to the specified file.
            //  -seqfile <filename
            //     Specifies a file from which move sequences are read.
        ];

        for (denominator, flag) in flags {
            if rng.gen_ratio(1, *denominator)  {
                args.push(' ');
                flag(&mut args, &mut rng);
            }
        }
        let arguments = args.as_str();
        let adjust = if rng.gen_ratio(1, 4) {
            println!("creating adjust.txt");
            Some(format!("{} {} {} {}\n",
                         rng.gen_range::<f32, _>(0.0..20.0),
                         rng.gen_range::<f32, _>(0.0..20.0),
                         rng.gen_range::<f32, _>(0.0..20.0),
                         rng.gen_range::<f32, _>(0.0..20.0),
            ))
        } else {
            None
        };

        let has_error = false; // TODO

        println!("testing args '{}'", arguments);
        let coeffs_path_from_run_dir = "./../../coeffs2.bin";
        let book_path_from_run_dir = "./../../book.bin";
        snapshot_test_with_folder(binary_folder, binary, arguments, "fuzzer",
                                  adjust.as_ref().map(AsRef::as_ref), has_error, false, interactive,
                                  coeffs_path_from_run_dir,
                                  book_path_from_run_dir);

        let binary_folder = "../../target/release/";

        snapshot_test_with_folder(binary_folder, binary, arguments, "fuzzer",
                                  adjust.as_ref().map(AsRef::as_ref), has_error, false, interactive,
                                  coeffs_path_from_run_dir, book_path_from_run_dir);
    }

}

mod tests {
    use flate2_coeff_source::Flate2Source;
    use std::ffi::CStr;
    use zlib_coeff_source::{ZLibSource};
    use std::process::{Command, Stdio, ChildStdin};
    use std::path::Path;
    use std::fs::File;
    use std::io::{Write};

    #[test]
    fn coeff_source_test() {
        use engine_traits::CoeffSource;
        let file_name: &CStr = CStr::from_bytes_with_nul(b"./../coeffs2.bin\x00").unwrap();

        let mut z_lib_source = ZLibSource::try_new(file_name).unwrap();

        let mut flate2_source = Flate2Source::new_from_data(&std::fs::read("./../coeffs2.bin").unwrap());

        while let Some(word) = z_lib_source.try_next_word() {
            let flate_word = flate2_source.try_next_word().unwrap();
            assert_eq!(word, flate_word)
        }

        assert!(flate2_source.try_next_word().is_none());
    }

    macro_rules! snap_test {
        ($id:ident, $args:literal) => {
            snap_test!($id, $args, false, with_adjust: true);
        };

        ($binary:literal, $func:ident, $suffix:literal, $id:ident, $args:literal, $has_err:expr, $has_adjust:expr) => {
            snap_test!($binary, $func, $suffix, $id, $args, $has_err,  $has_adjust, interactive: None);
        };

        ($binary:literal, $func:ident, $suffix:literal, $id:ident, $args:literal, $has_err:expr, $has_adjust:expr, interactive: $interactive:ident) => {
                #[test]
                fn $func() {
                    use crate::tests::*;
                    snapshot_test(
                        $binary,
                        $args,
                        &("./snapshot-tests/".to_owned() + stringify!($id) + $suffix),
                        $has_adjust,
                        $has_err,
                        Interactive::$interactive
                    );
                }
        };

        ($id:ident, $args:literal, $has_err:expr, with_adjust: true) => {
            mod $id {
                snap_test!("zebra", basic, "-basic", $id, $args, $has_err, false);
                snap_test!("zebra", with_adjust, "-with-adjust", $id, $args, $has_err, true);
            }
        };
        ($binary:literal, $id:ident, $args:literal, $has_err:expr, with_adjust: false) => {
            mod $id {
                snap_test!($binary, basic, "-basic", $id, $args, $has_err, false);
            }
        };

        ($binary:literal, $id:ident, $args:literal) => {
            mod $id {
                snap_test!($binary, basic, "-basic", $id, $args, false, false);
            }
        };
        ($binary:literal, $id:ident, $args:literal, $has_err:literal) => {
            mod $id {
                snap_test!($binary, basic, "-basic", $id, $args, $has_err, false);
            }
        };

        ($id:ident, $args:literal, $has_err:expr, with_adjust: false) => {
            mod $id {
                snap_test!("zebra", basic, "-basic", $id, $args, $has_err, false);
            }
        };

        ($id:ident, $args:literal, $has_err:expr, interactive: Dumb) => {
            mod $id {
                snap_test!("zebra", basic, "-basic", $id, $args, $has_err, false, interactive: Dumb);
            }
        };
        ($binary:literal, $id:ident, $args:literal, $has_err:expr, interactive: $interactive:ident) => {
            mod $id {
                snap_test!($binary, basic, "-basic", $id, $args, $has_err, false, interactive: $interactive);
            }
        };

        ($id:ident, $args:literal, $has_err:expr) => {
            snap_test!($id, $args, $has_err, with_adjust: true);
        };
    }

    snap_test!(minus_p_zero, "-l 6 6 6 6 6 6 -r 0 -p 0");

    snap_test!(with_seq, "-seq f5d6c3 -l 6 6 6 6 6 6 -r 0");

    snap_test!(analyze_basic, "-analyze -seq e6f6f5f4e3d6g4d3c3h3c4g3g5g6c7c6c5b6d7b5f7f3b4f8h4h5f2f1h2h1 -l 7 7 7 7 7 7 -r 0");

    // TODO this is broken against original zebra (panic vs invalid move err)
    snap_test!(analyze_invalid, "-analyze -seq f1h2h1 -l 7 7 7 7 7 7 -r 0", true, with_adjust: false);

    snap_test!(with_seq_invalid, "-seq f5d6h1 -l 6 6 6 6 6 6 -r 0", true);

    snap_test!(with_no_echo, "-l 6 6 6 6 6 6 -r 0 -e 0");

    snap_test!(with_some_slack, "-l 6 6 6 6 6 6 -r 0 -slack 8");

    snap_test!(with_hash_twelve, "-l 6 6 6 6 6 6 -r 0 -h 12");

    snap_test!(with_repeat, "-l 6 6 6 6 6 6 -r 0 -repeat 2");

    snap_test!(with_repeat_and_log, "-l 6 6 6 6 6 6 -r 0 -repeat 2 -log zebra.log");

    snap_test!(no_wld, "-l 6 6 0 6 6 0 -r 0 -repeat 2");

    /*
    run this to verify
     cargo test --release --package tests "tests::no_exact_no_wld::no_exact_no_wld" -- --test-threads 1 --nocapture
    */
    snap_test!(no_exact_no_wld, "-l 6 0 0 6 0 0 -r 0 -repeat 2");

    snap_test!(minus_p_zero_without_book, "-l 6 6 6 6 6 6 -r 0 -p 0 -b 0");

    snap_test!(small_game_test_without_book, "-l 6 6 6 6 6 6 -r 0 -b 0", false, with_adjust: false);

    snap_test!(full_game_test, "-l 16 16 16 16 16 16 -r 0", false, with_adjust: false);

    snap_test!(small_game_test, "-l 6 6 6 6 6 6 -r 0", false, with_adjust: false);

    snap_test!(micro_game, "-l 1 1 1 1 1 1 -r 0", false, with_adjust: false);

    // TODO find some thor files to verify - this doesn't really do anything at the moment
    snap_test!(thor_five, "-l 2 2 2 2 2 2 -r 0 -thor 5", false, with_adjust: false);

    snap_test!(uneven, "-l 1 1 1 8 8 8 -r 0", false, with_adjust: false);

    snap_test!(rand_move, "-l 6 6 6 6 6 6 -r 0 -randmove 3", false, with_adjust: false);

    snap_test!(rand_move_without_book, "-l 6 6 6 6 6 6 -r 0 -randmove 3 -b 0", false, with_adjust: false);

    snap_test!(rand_move_one, "-l 6 6 6 6 6 6 -r 0 -randmove 1", false, with_adjust: false);

    snap_test!(basic_interactive, "-l 6 6 6 0 -r 0 -b 0 -repeat 2", false, interactive: Dumb);

    snap_test!(basic_interactive_flipped, "-l 0 6 6 6 -r 0 -b 0 -repeat 1", false, interactive: Dumb);

    snap_test!(two_players, "-l 0 0 -r 0 -b 0 -repeat 2", false, interactive: Dumb);

    snap_test!(two_players_with_log, "-l 0 0 -r 0 -b 0 -repeat 2  -log zebra.log", false, interactive: Dumb);

    snap_test!(learn, "-l 2 2 2 2 2 2 -r 0 -learn 3 5", false, with_adjust: false);

    snap_test!(
        seqfile,
         "-l 2 2 2 2 2 2 -r 0 -seqfile ../../../resources/seq-file.txt -log zebra.log",
        false,
        with_adjust: false
    );

    snap_test!(
        seqfile_too_long,
         "-l 2 2 2 2 2 2 -r 0 -seqfile ../../../resources/seq-file-too-long.txt -log zebra.log",
        true,
        with_adjust: false
    );

    snap_test!(
        seqfile_invalid,
         "-l 2 2 2 2 2 2 -r 0 -seqfile ../../../resources/seq-file-invalid.txt -log zebra.log",
        true,
        with_adjust: false
    );

    snap_test!(
        board_source,
         "-l 2 2 2 2 2 2 -r 0 -g ../../../resources/board.txt -log zebra.log",
        false,
        with_adjust: false
    );

    // TODO test all these parameters at once: -g, -seq and -seqfile, how they interact??
    //  what if they conflict??

    snap_test!("practice", practice, "./../../../../book.bin", false, interactive: Practice);
    snap_test!("practice", practice_help, "./../../../../book.bin dd dd", true);

    #[derive(Copy, Clone)]
    pub enum Interactive {
        Dumb,
        Practice,
        None
    }
    fn interact_practice(input: &mut ChildStdin) {
        let moves = "e6  f6 f5  f4 e3  d6 g4  d3 c3  h3 c4  g3 g5  g6 c7  c6 c5  b6 7  b5 f7 \
         f3 b4  f8 h4  h5 f2  f1 g2  e8 e7  a3 h2  h1 h6  d8 c8  b8 g7  b7 a4  g8 a8  a7 h8  h7 \
         g1  e1 e2  d1 a5  c2 d2  a6 b3  c1 a2  b2 a1  b1 quit";
        moves.split_whitespace().for_each(|move_| {
            let _ = input.write(move_.as_bytes());
            let _ = input.write("\n".as_bytes());
            let _ = input.flush();
        });
    }
    fn interact_basically(input: &mut ChildStdin) {
        let mut move_buf = String::with_capacity(3);
        loop {
            let mut written = 0;
            // TODO this is very dumb, we should prerecord some games and test them directly
            for char in 'a'..='h' {
                for num in '1'..='8' {
                    move_buf.truncate(0);
                    move_buf.push(char);
                    move_buf.push(num);
                    move_buf.push('\n');
                    written = input.write(move_buf.as_ref()).unwrap_or(0);
                    let _ = input.flush();
                    if written == 0 {
                        break;
                    }
                }
                if written == 0 {
                    break;
                }
            }
            if written == 0 {
                break;
            }
            move_buf.truncate(0);
            move_buf.push('\n'); // try pass
            written = input.write(move_buf.as_ref()).unwrap_or(0);
            let _ = input.flush();
            if written == 0 {
                break;
            }
        }
    }

    fn snapshot_test(binary: &str, arguments: &str, snapshot_test_dir: &str, with_adjust: bool, has_error: bool, interactive: Interactive) {
        let binary_folder =
            // "./../../../../../zebra-1/"
            "./../../../../target/release/"
            // "./../../../../../bisection/target/release/"
            ;
        let coeffs_path_from_run_dir = "./../../../../coeffs2.bin" ;
        let book_path_from_run_dir = "./../../../../book.bin" ;
        let with_adjust = if with_adjust {
            Some("3.5 2.8 5.1 12.3\n".as_ref())
        } else {
            None
        };
        snapshot_test_with_folder(binary_folder, binary, arguments, snapshot_test_dir, with_adjust, has_error,
                                  true,
                                  interactive, coeffs_path_from_run_dir, book_path_from_run_dir);
    }

    pub fn snapshot_test_with_folder(binary_folder: &str,
                                     binary: &str,
                                     arguments: &str,
                                     snapshot_test_dir: &str,
                                     adjust: Option<&str>,
                                     has_error: bool,
                                     check_exit_status: bool,
                                     interactive: Interactive,
                                     coeffs_path_from_run_dir: &str,
                                     book_path_from_run_dir: &str) {

        let snapshot_test_dir = Path::new(snapshot_test_dir);
        if !snapshot_test_dir.exists() {
            std::fs::create_dir_all(snapshot_test_dir).unwrap();
        }
        let snapshots_dir = snapshot_test_dir.join("snapshots");
        if !snapshots_dir.exists() {
            std::fs::create_dir_all(&snapshots_dir).unwrap();
        }

        let run_directory = snapshot_test_dir.join("run_dir");
        let _ = std::fs::remove_dir_all(&run_directory);
        std::fs::create_dir_all(&run_directory).unwrap();

        if let Some(s) = adjust {
            let path = run_directory.join("adjust.txt");
            File::create(path)
                .unwrap()
                .write(s.as_bytes())
                .unwrap();
        }
        let binpath = run_directory
            .join(binary_folder)
            .join(binary)
            .canonicalize()
            .unwrap();

        let coeffs_path = run_directory.join(coeffs_path_from_run_dir).canonicalize().unwrap();
        let mut book_path = run_directory.join(book_path_from_run_dir).canonicalize().unwrap();
        let canon_run_dir = run_directory.canonicalize().unwrap();

        let compare_books = arguments.contains("-learn");
        if compare_books {
            let buf = canon_run_dir.join("book.bin");
            std::fs::copy("resources/book-tmp.bin", &buf).unwrap();
            book_path = buf;
        }

        let mut child = Command::new(binpath)
            .current_dir(&canon_run_dir)
            .args(arguments.split_whitespace())
            .env("COEFFS_PATH", coeffs_path.to_str().unwrap())

            // we probably don't need this when -learn parameter is set, because we copy the
            // investigate that
            .env("BOOK_PATH", book_path.to_str().unwrap())
            .env("MOCK_TIME", "true")
            .stdin(Stdio::piped())
            .stderr(Stdio::from(File::create(canon_run_dir.join("zebra-stderr")).unwrap()))
            .stdout(Stdio::from(File::create(canon_run_dir.join("zebra-stdout")).unwrap()))
            .spawn()
            .unwrap();

        match interactive {
            Interactive::Dumb => {
                let input = child.stdin.as_mut().unwrap();
                interact_basically(input);
            },
            Interactive::Practice => {
                let input = child.stdin.as_mut().unwrap();
                interact_practice(input);
            }
            _ => {}
        }
        let exit_status = child
            .wait()
            .unwrap();
        // TODO make this flag part of some metadata snapshot file
        //  so that we don't need to guess its value when writing new test
        if check_exit_status {
            assert_eq!(exit_status.success(), !has_error);
        }

        if compare_books {
            assert_snapshot(&*snapshots_dir.join("book.bin"), &*book_path, true);
        }
        // TODO detect other files that may be created or not created durinng run and report them
        assert_snapshot(snapshots_dir.join("zebra.log").as_ref(), run_directory.join("zebra.log").as_ref() , false);
        assert_snapshot(snapshots_dir.join("zebra-stderr").as_ref(), run_directory.join("zebra-stderr").as_ref() , false);
        assert_snapshot(snapshots_dir.join("zebra-stdout").as_ref(), run_directory.join("zebra-stdout").as_ref() , false);
        assert_snapshot(snapshots_dir.join("current.gam").as_ref(), run_directory.join("current.gam").as_ref() , false);
        assert_snapshot(snapshots_dir.join("current.mov").as_ref(), run_directory.join("current.mov").as_ref(), false);
        assert_snapshot(snapshots_dir.join("analysis.log").as_ref(), run_directory.join("analysis.log").as_ref(), false);
    }

    fn assert_snapshot(snapshot_path: &Path, result_path: &Path, binary: bool) {

        if result_path.exists() {
            if !snapshot_path.exists() {
                if std::env::var("BLESS").map(|v| v == "true").unwrap_or(false) {
                    std::fs::copy(result_path, snapshot_path).unwrap();
                } else {
                    panic!(
                        "\n\nWARNING: Snapshot for \n{:?}\n ... doesn't exist. Rerun the tests with BLESS=true environment variable to make them green again.\n\n\n",
                        result_path
                    );
                }
            }
        } else {
            if !snapshot_path.exists() {
                return // this means that this run doesn't have output any snapshot
            }
        }
        if binary {
            let actual = std::fs::read(snapshot_path).unwrap() ;
            let expected = std::fs::read(result_path).unwrap();
            assert_eq!(expected, actual);
            return;
        }

        fn variable_lines(line: &&str) -> bool {
            !(line.starts_with("Engine compiled")
                || line.starts_with("Zebra (c) 1997-2005 Gunnar Andersson, compile date")
                || line.starts_with("Gunnar Andersson"))
        }

        let snapshot = std::fs::read_to_string(snapshot_path).unwrap();
        let output = std::fs::read_to_string(result_path).unwrap();

        let mut first = snapshot.lines().filter(variable_lines);
        let mut second = output.lines().filter(variable_lines);

        while let Some(expected) = first.next() {
            assert_eq!(expected, second.next().unwrap())
        }
        assert!(first.next().is_none());
        assert!(second.next().is_none());
    }

    snap_test!(help, "?", true);

    // These are failing cases found by fuzzer. Some of them are caused by
    // UB sanitizer logs in the original zebra

//     testing args '-r 0 -l 1 3 8 8 1 7 -repeat 4 -h 1'
// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', tests/src/main.rs:480:48
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    //testing args '-r 0 -l 9 4 4 7 4 8 -public -draw2none -repeat 4 -slack 0.33416033'
    // thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', tests/src/main.rs:480:48
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//     testing args '-r 0 -l 9 3 7 5 2 6 -e 1'
// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', tests/src/main.rs:480:48
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//     testing args '-r 0 -l 7 5 0 8 8 7 -repeat 3 -e 1'
// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', tests/src/main.rs:480:48
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//     testing args '-r 0 -l 5 7 6 5 1 2 -slack 8.41116'
// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', tests/src/main.rs:480:48
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//     testing args '-r 0 -l 8 9 1 1 4 2 -draw2black -draw2none -repeat 3 -e 1'
// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', tests/src/main.rs:480:48
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//     testing args '-r 0 -l 2 4 7 9 9 6 -public -private -draw2black -draw2white -repeat 4 -slack 4.9125338'
// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', tests/src/main.rs:480:48
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // testing args '-r 0 -l 5 6 6 6 6 4 -private -slack 0.28241396 -e 0 -h 7'
    // thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', tests/src/main.rs:480:48
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `"      1        O   O   O      Black    Zebra"`,
//  right: `"      1      O     O   O      Black    Zebra"`', tests/src/main.rs:486:13
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//     testing args '-r 0 -l 9 1 9 12 4 9 -keepdraw -test -slack 8.871107 -p 0 -b 0 -thor 2'
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `"Loaded 0 games in 1.000 s."`,
//  right: `"Loaded 0 games in 0.000 s."`', tests/src/main.rs:505:13

//     testing args '-r 0 -l 9 16 11 8 18 6 -draw2none -p 0 -w 0 -thor 9 -h 4'
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `"b2=4"`,
//  right: `"0 matching games  (0.000 s search time, 0.000 s total)"`', tests/src/main.rs:551:13
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//     testing args '-r 0 -l 15 19 7 9 2 19 -keepdraw -draw2black -draw2white -randmove 1 -p 0 -thor 4'
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `"-->  15  -7.34         346445  f3 d2 c5 c6 c1    77.0 s    4385 nps"`,
//  right: `"-->  15  -7.34         337693  f3 d2 c5 c6 c1    77.0 s    4275 nps"`', tests/src/main.rs:552:13
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//     testing args '-r 0 -l 0 0 -public -analyze -p 0 -g ../../tests/resources/board.txt -seq e6f6f5f4e3d6g4d3c3h3c4g3g5g6c7c6c5b6d7b5f7f3b4f8h4h5f2f1h2h1'
// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', tests/src/main.rs:522:48
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    //testing args '-r 0 -l 0 0 -draw2black -repeat 0 -p 1 -b 1 -time 1.6256914422442514 4.570592846993699 2.5283251123785977 3.7189676525522044'
    // thread 'main' panicked at 'assertion failed: `(left == right)`
    //   left: `"      2                                00:01        "`,
    //  right: `"      2                                00:00        "`', tests/src/main.rs:534:13
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//     testing args '-r 0 -l 8 7 2 8 3 7 -b 0 -w 1 -time 4.529340827768474 4.712636376864859 4.990958221355872 4.91143980476709'
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `"-->   2  +5.48             39  e7 f6     0.0 s  "`,
//  right: `"-->   8  +4.53          37988  e7 f6 a7 g2 f7     0.0 s  "`', tests/src/main.rs:534:13
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//     creating adjust.txt
// testing args '-r 0 -l 5 17 9 1 3 11 -draw2none -time 2 0 1 3'
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `"================================================================="`,
//  right: `"thread \'main\' panicked at \'index out of bounds: the len is 121 but the index is 121\', engine/src/zebra.rs:542:5"`', tests/src/main.rs:534:13

//     testing args '-r 0 -l 4 17 1 7 14 17 -p 0 -e 0 -time 2 0 0 2'
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `"-->  10  +4                 3  b3 a2 b7 a7        0.0 s       3 nps  "`,
//  right: `"-->  10  +4                 3  b3 a2 b7 a7        0.0 s  "`', tests/src/main.rs:534:13
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// matyas@DESKTOP-LCL64AA:~/zebra-fresh$ bash run-tests.sh
// warning: profiles for the non root package will be ignored, specify profiles at the workspace root

//     testing args '-r 0 -l 3 4 14 9 8 19 -repeat 0 -slack 5.739609 -p 1 -e 0 -b 0 -w 0 -h 3 -time 4 3 2 1'
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `"-->   5  -12              249  b8                 1.0 s     249 nps  "`,
//  right: `"-->   5  -12              249  b8                 0.0 s     249 nps  "`', tests/src/main.rs:534:13
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//     testing args '-r 0 -l 6 10 4 8 6 4 -randmove 5 -p 0 -e 0 -h 17 -time 1 0 2 3'
// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', tests/src/main.rs:534:48
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//     testing args '-r 0 -l 5 2 16 8 15 0 -analyze -repeat 3 -p 0 -b 0 -t 2 3 4 5 6 4 2 -time 2 0 1 0'
// thread 'main' panicked at 'assertion failed: `(left == right)`

//   left: `"-->   2  -3.12             36  f5        0.0 s      36 nps"`,
//  right: `"-->   2  -3.12             36  f5        0.0 s  "`', tests/src/main.rs:534:13
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//     testing args '-r 0 -l 5 10 3 3 19 8 -private -p 0 -b 1 -dev 89 93 211.67601 -t 2 4 9 5 8 9 1 -time 2 4 5 40'
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `"-->   2  +8.97             22  f4        1.0 s      22 nps"`,
//  right: `"-->   2  +8.97             22  f4        0.0 s  "`', tests/src/main.rs:534:13
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // Note(matyas): I have no idea what's wrong with this one
    //  It's buffer overflow in original and panic out of bounds in rust,
    //   because disks_played is 61 roughly after midgame. Very weird bug.
    //   There's probably some place in the code where unmake_move call is missing or smth like that?
// testing args '-r 0 -l 9 6 3 5 19 0 -repeat 4 -p 1 -b 1 -w 0 -h 19 -dev 17 75 94.33498 -g ../../tests/resources/board.txt -time 40 48 6 24'
// thread 'main' panicked at 'assertion failed: second.next().is_none()', tests/src/main.rs:537:9

//similar:     testing args '-r 0 -l 7 0 14 9 4 18 -repeat 0 -randmove 5 -p 1 -e 1 -h 9 -g ../../tests/resources/board.txt -time 13 47 5 16'
//     testing args '-r 0 -l 3 6 0 4 13 12 -repeat 4 -e 0 -b 0 -g ../../tests/resources/board.txt -time 12 23 5 24'
// testing args '-r 0 -l 9 8 7 8 19 14 -randmove 4 -p 1 -e 0 -h 5 -dev 8 8 123.677826 -t 3 4 17 10 7 3 7 8 1 10 -seq e6f6f5f4e3d6g4d3c3h3c4g3g5g6c7c6c5b6d7b5f7f3b4f8h4h5f2f1 -time 4 24 24 22'
}
