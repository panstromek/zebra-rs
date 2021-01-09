fn main() {}

#[cfg(test)]
mod tests {
    use flate2_coeff_source::Flate2Source;
    use std::ffi::CStr;
    use legacy_zebra::src::getcoeff::new_z_lib_source;
    use std::process::{Command, Stdio};
    use std::path::Path;
    use std::fs::File;
    use std::io::{Write};

    #[test]
    fn coeff_source_test() {
        use engine_traits::CoeffSource;
        let file_name: &CStr = CStr::from_bytes_with_nul(b"./../coeffs2.bin\x00").unwrap();

        let mut z_lib_source = new_z_lib_source(file_name);

        let mut flate2_source = Flate2Source::new_from_data(&std::fs::read("./../coeffs2.bin").unwrap());

        while let Some(word) = z_lib_source.try_next_word() {
            let flate_word = flate2_source.try_next_word().unwrap();
            assert_eq!(word, flate_word)
        }

        assert!(flate2_source.try_next_word().is_none());
    }
    fn create_adjust_file<P: AsRef<Path>>(path: P) {
        File::create(path)
            .unwrap()
            .write("3.5 2.8 5.1 12.3\n".as_ref())
            .unwrap();
    }
    fn delete_adjust_file<P: AsRef<Path>>(path: P) {
        if Path::new(path.as_ref()).exists() {
            std::fs::remove_file(path.as_ref()).unwrap();
        }
    }
    // TODO these snapshot tests don't test for the last position
    //  (because zebra doesn't put it in the log file for some reason)
    //  it'd be good to improve that and test that one as well
    //fixme
    // also include analysis.log file into the tests

    macro_rules! snap_test {
        ($id:ident, $args:literal) => {
            snap_test!($id, $args, false, with_adjust: true);
        };

        ($func:ident, $suffix:literal, $id:ident, $args:literal, $has_err:expr, $has_adjust:expr) => {
                #[test]
                fn $func() {
                    use crate::tests::*;
                    snapshot_test(
                        $args,
                        &("./snapshot-tests/".to_owned() + stringify!($id) + $suffix),
                        $has_adjust,
                        $has_err
                    );
                }
        };

        ($id:ident, $args:literal, $has_err:expr, with_adjust: true) => {
            mod $id {
                snap_test!(basic, "-basic", $id, $args, $has_err, false);
                snap_test!(with_adjust, "-with-adjust", $id, $args, $has_err, true);
            }
        };
        ($id:ident, $args:literal, $has_err:expr, with_adjust: false) => {
            mod $id {
                snap_test!(basic, "-basic", $id, $args, $has_err, false);
            }
        };

        ($id:ident, $args:literal, $has_err:expr) => {
            snap_test!($id, $args, $has_err, with_adjust: true);
        };
    }

    snap_test!(minus_p_zero, "-l 6 6 6 6 6 6 -r 0 -p 0");

    snap_test!(with_seq, "-seq f5d6c3 -l 6 6 6 6 6 6 -r 0");

    snap_test!(with_seq_invalid, "-seq f5d6h1 -l 6 6 6 6 6 6 -r 0", true);

    snap_test!(with_no_echo, "-l 6 6 6 6 6 6 -r 0 -e 0");

    snap_test!(with_some_slack, "-l 6 6 6 6 6 6 -r 0 -slack 8");

    snap_test!(with_hash_twelve, "-l 6 6 6 6 6 6 -r 0 -h 12");

    snap_test!(with_repeat, "-l 6 6 6 6 6 6 -r 0 -repeat 2");

    snap_test!(with_repeat_and_log, "-l 6 6 6 6 6 6 -r 0 -repeat 2 -log zebra.log");

    snap_test!(no_wld, "-l 6 6 0 6 6 0 -r 0 -repeat 2");

    // FIXME this test is failing against old zebra, investigate that
    //  it's because there's UB - index out of bounds when accessing
    //  stage_reached and stage_score in midgame.c and  list_inherited in search.c
    //  I "fixed" it just by making these arrays bigger, but it's not clear if that's
    //  actually the correct fix WRT program logic, maybe it should instead be smaller -
    //  it's unclear what was the original intent with it.
    //  Making these arrays a bit longer in the original program will change the output of this
    //  test to match the snapshot. But changing just the array length of stage_reached from
    //  62 back to 61 will change the output back. Because array length in C is not stored and
    //  and here it is defined with a literal, it means that this number can't possibly affect
    //  any logic - but it changes the behaviour of the program nevertheless, so there's
    //  probably some UB at play here. Also I know for a fact that this array is accessed out of
    //  bounds, because the initial translated Rust program would panic on these places.
    /*
    run this to verify
     cargo test --release --package tests "tests::no_exact_no_wld::no_exact_no_wld" -- --test-threads 1 --nocapture
    */
    snap_test!(no_exact_no_wld, "-l 6 0 0 6 0 0 -r 0 -repeat 2");

    snap_test!(minus_p_zero_without_book, "-l 6 6 6 6 6 6 -r 0 -p 0 -b 0");

    snap_test!(small_game_test_without_book, "-l 6 6 6 6 6 6 -r 0 -b 0", false, with_adjust: false);

    snap_test!(full_game_test, "-l 16 16 16 16 16 16 -r 0", false, with_adjust: false);

    snap_test!(small_game_test, "-l 6 6 6 6 6 6 -r 0", false, with_adjust: false);

    #[test]
    fn basic_interactive() {
        delete_adjust_file("./../adjust.txt");
        let mut command = Command::new("./target/release/zebra");
        let command = command
            .stdin(Stdio::piped())
            .stderr(Stdio::from(File::create("./../zebra-stderr").unwrap() ))
            .stdout(Stdio::from(File::create("./../zebra-stdout").unwrap() ))
            .current_dir("./../"); // TODO use temp to run in parallel

        command.args("-l 6 6 6 0 -r 0 -b 0 -repeat 2".split_whitespace());

        let mut child = command.spawn().unwrap();

        let input = child.stdin.as_mut().unwrap();
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

        // TODO add other tests that are in non-interactive snaphsot test
        // TODO assert stderr is empty
        assert_snapshot("./snapshots/zebra.log-basic_interactive", "./../zebra.log");
    }

    fn snapshot_test(arguments: &str, snapshot_test_dir: &str, with_adjust: bool, has_error: bool) {
        let binary: &str = "./../../../../target/release/zebra";
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

        if with_adjust {
            create_adjust_file(run_directory.join("adjust.txt"));
        }
        let binpath = run_directory.join(binary).canonicalize().unwrap();
        let coeffs_path = run_directory.join("./../../../../coeffs2.bin").canonicalize().unwrap();
        let book_path = run_directory.join("./../../../../book.bin").canonicalize().unwrap();
        let canon_run_dir = run_directory.canonicalize().unwrap();

        let exit_status = Command::new(binpath)
            .current_dir(&canon_run_dir)
            .args(arguments.split_whitespace())
            .env("COEFFS_PATH", coeffs_path.to_str().unwrap())
            .env("BOOK_PATH", book_path.to_str().unwrap())
            .stdin(Stdio::piped())
            .stderr(Stdio::from(File::create(canon_run_dir.join("zebra-stderr")).unwrap() ))
            .stdout(Stdio::from(File::create(canon_run_dir.join("zebra-stdout")).unwrap() ))
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        // TODO make this flag part of some metadata snapshot file
        //  so that we don't need to guess its value when writing new test
        assert_eq!(exit_status.success(), !has_error);

        assert_snapshot(snapshots_dir.join("zebra.log").to_str().unwrap(), run_directory.join("zebra.log").to_str().unwrap() );
        assert_snapshot(snapshots_dir.join("zebra-stderr").to_str().unwrap(), run_directory.join("zebra-stderr").to_str().unwrap() );
        assert_snapshot(snapshots_dir.join("zebra-stdout").to_str().unwrap(), run_directory.join("zebra-stdout").to_str().unwrap() );
        assert_snapshot(snapshots_dir.join("current.gam").to_str().unwrap(), run_directory.join("current.gam").to_str().unwrap() );
        assert_snapshot(snapshots_dir.join("current.mov").to_str().unwrap(), run_directory.join("current.mov").to_str().unwrap() );
    }

    fn assert_snapshot(snapshot_path: &str, result_path: &str) {
        let snapshot_path: &Path = snapshot_path.as_ref();
        let result_path: &Path = result_path.as_ref();

        if result_path.exists() {
            if !snapshot_path.exists() {
                if std::env::var("BLESS").map(|v| v == "true").unwrap_or(false) {
                    std::fs::copy(result_path, snapshot_path).unwrap();
                } else {
                    panic!("\n\nWARNING: Snapshot doesn't exists. Rerun the tests with BLESS=true environment variable to make them green again.\n\n\n");
                }
            }
        } else {
            if !snapshot_path.exists() {
                return // this means that this run doesn't have output any snapshot
            }
        }

        fn variable_lines(line: &&str) -> bool {
            !(
                line.starts_with("-->")
                || line.starts_with("Log file created")
                || line.starts_with("Engine compiled")
                || line.starts_with("Total time:")
                || line.starts_with("[-inf,inf]:")
                || line.starts_with("Reading binary opening database... done (took ")
                || (line.starts_with("#") && matches_ctime(&line[1..]).unwrap_or(false) )
            )
        }
        fn matches_ctime(line :&str) -> Option<bool> {
            // Www Mmm dd hh:mm:ss yyyy
            let mut split = line.trim().split_whitespace();
            // Just rougly matches ctime by lenghts - there's very low chance that some
            // other text will match this too
            Some(split.next()?.len() == 3 &&
                split.next()?.len() == 3 &&
                split.next()?.len() < 3 &&
                split.next()?.len() == 8 &&
                split.next()?.len() == 4 &&
                split.next().is_none())
        }
        let snapshot = std::fs::read_to_string(snapshot_path).unwrap();
        let output = std::fs::read_to_string(result_path).unwrap();

        let mut first = snapshot.lines().filter(variable_lines);
        let mut second = output.lines().filter(variable_lines);

        while let (Some(expected), Some(actual)) = (first.next(), second.next()) {
            assert_eq!(expected, actual)
        }
        assert!(first.next().is_none());
        assert!(second.next().is_none());
    }

    snap_test!(help, "?", true);
}
