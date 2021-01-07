fn main() {}

#[cfg(test)]
mod tests {
    use flate2_coeff_source::Flate2Source;
    use std::ffi::CStr;
    use legacy_zebra::src::getcoeff::new_z_lib_source;
    use std::process::Command;
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
    fn create_adjust_file() {
        File::create("./../adjust.txt")
            .unwrap()
            .write("3.5 2.8 5.1 12.3\n".as_ref())
            .unwrap();
    }
    fn delete_adjust_file() {
        let filename = "./../adjust.txt";
        if Path::new(filename).exists() {
            std::fs::remove_file(filename).unwrap();
        }
    }
    // TODO these snapshot tests don't test for the last position
    //  (because zebra doesn't put it in the log file for some reason)
    //  it'd be good to improve that and test that one as well

    #[test]
    fn full_game_test() {
        snapshot_test(
            "./target/release/zebra",
            "-l 16 16 16 16 16 16 -r 0",
            "./snapshots/zebra.log-full_game_test",
            false
        );
    }

    #[test]
    fn small_game_test() {
        snapshot_test(
            "./target/release/zebra",
            "-l 6 6 6 6 6 6 -r 0",
            "./snapshots/zebra.log-small_game_test",
            false
        );
    }

    macro_rules! snap_test {
        ($id:ident, $args:literal) => {
            mod $id {
                use crate::tests::*;
                #[test]
                fn $id() {
                    snapshot_test(
                        "./target/release/zebra",
                        $args,
                        &("./snapshots/zebra.log-".to_owned() + stringify!($id)),
                        false
                    );
                }
                mod with_adjust {
                    use crate::tests::*;
                    #[test]
                    fn $id() {
                        snapshot_test(
                            "./target/release/zebra",
                            $args,
                            &("./snapshots/zebra.log-".to_owned() + stringify!($id) + "-with-adjust"),
                            true
                        );
                    }
                }
            }
        };
    }

    snap_test!(minus_p_zero, "-l 6 6 6 6 6 6 -r 0 -p 0");

    snap_test!(with_no_echo, "-l 6 6 6 6 6 6 -r 0 -e 0");

    snap_test!(with_some_slack, "-l 6 6 6 6 6 6 -r 0 -slack 8");

    snap_test!(with_hash_twelve, "-l 6 6 6 6 6 6 -r 0 -h 12");

    snap_test!(with_repeat, "-l 6 6 6 6 6 6 -r 0 -repeat 2");

    snap_test!(with_repeat_and_log, "-l 6 6 6 6 6 6 -r 0 -repeat 2 -log zebra.log");

    snap_test!(no_wld, "-l 6 6 0 6 6 0 -r 0 -repeat 2");

    snap_test!(wld_only, "-l 6 6 6 6 6 6 -r 0 -repeat 2 -wld 1");

    snap_test!(no_exact_no_wld, "-l 6 0 0 6 0 0 -r 0 -repeat 2");

    snap_test!(minus_p_zero_without_book, "-l 6 6 6 6 6 6 -r 0 -p 0 -b 0");

    #[test]
    fn small_game_test_without_book() {
        snapshot_test(
             // TODO run those tests on original zebra too
            "./target/release/zebra",
            "-l 6 6 6 6 6 6 -r 0 -b 0",
            "./snapshots/zebra.log-small_game_test_without_book",
            false,
        );
    }

    fn snapshot_test(binary: &str, arguments: &str, snapshot_path: &str, with_adjust: bool) {
        if with_adjust {
            create_adjust_file();
        } else {
            delete_adjust_file();
        }
        let output = Command::new(binary)
            .current_dir("./../")
            .args(arguments.split_whitespace())
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stderr).trim() , "");
        // TODO maybe assert stdout too?? for echo tests for example
        assert_log_file(snapshot_path);
    }

    fn assert_log_file(snapshot_path: &str) {
        let snapshot_path: &Path = snapshot_path.as_ref();
        let log_path = "./../zebra.log";
        if !snapshot_path.exists() {
            std::fs::copy(log_path, snapshot_path).unwrap();
            panic!("WARNING: Snapshot doesn't exists, creating new one. Rerun the tests to make them green again.");
        }
        fn variable_lines(line: &&str) -> bool {
            !(
                line.starts_with("-->")
                || line.starts_with("Log file created")
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
        std::fs::read_to_string(snapshot_path)
            .unwrap()
            .lines()
            .skip(1)
            .filter(variable_lines)
            .zip(std::fs::read_to_string(log_path)
                .unwrap()
                .lines()
                .skip(1)
                .filter(variable_lines))
            .for_each(|(expected, actual)| assert_eq!(expected, actual))
    }

    #[test]
    fn help_works() {
        let output = Command::new("./../target/release/zebra")
            .arg("?")
            .output()
            .unwrap();

        assert_eq!(
            std::fs::read("./snapshots/zebra.log-help_works").unwrap(),
            output.stdout
        );
    }
}
