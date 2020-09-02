fn main() {}

#[cfg(test)]
mod tests {
    use flate2_coeff_source::Flate2Source;
    use std::ffi::CStr;
    use legacy_zebra::src::getcoeff::new_z_lib_source;
    use std::process::Command;
    use std::path::Path;

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

    #[test]
    fn full_game_test() {
        snapshot_test(
            "./target/release/zebra",
            "-l 16 16 16 16 16 16 -r 0",
            "./snapshots/zebra.log-full_game_test",
        );
    }

    #[test]
    fn small_game_test() {
        snapshot_test(
            "./target/release/zebra",
            "-l 6 6 6 6 6 6 -r 0",
            "./snapshots/zebra.log-small_game_test",
        );
    }

   #[test]
    fn small_game_test_without_book() {
        snapshot_test(
            "./target/release/zebra",
            "-l 6 6 6 6 6 6 -r 0 -b 0",
            "./snapshots/zebra.log-small_game_test_without_book",
        );
    }

    fn snapshot_test(binary: &str, arguments: &str, snapshot_path: &str) {
        Command::new(binary)
            .current_dir("./../")
            .args(arguments.split_whitespace())
            .output()
            .unwrap();

        assert_log_file(snapshot_path);
    }

    fn assert_log_file(snapshot_path: &str) {
        let snapshot_path: &Path = snapshot_path.as_ref();
        let log_path = "./../zebra.log";
        if !snapshot_path.exists() {
            std::fs::copy(log_path, snapshot_path).unwrap();
            panic!("WARNING: Snapshot doesn't exists, creating new one. Rerun the tests to make them green again.");
        }
        let x = |line:&&str| !(line.starts_with("-->") || line.starts_with("Log file created"));
        assert!(
            std::fs::read_to_string(snapshot_path)
                .unwrap()
                .lines()
                .skip(1)
                .filter(|line|
                    !(line.starts_with("-->") || line.starts_with("Log file created")))
                .eq(std::fs::read_to_string(log_path)
                    .unwrap()
                    .lines()
                    .skip(1)
                    .filter(x))
        );
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
//
// #[test]
// fn full_game_test_2() {
//     let output = Command::new("./../target/release/zebra")
//         .arg("-l 6 6 6 6 6 6")
//         .arg("-r 0 ")
//         .output()
//         .unwrap();
//
//     assert_eq!("dd".to_string(),  String::from_utf8(output.stdout).unwrap());
//     //
//     // assert_eq!(
//     //     std::fs::read("./../zebra.log-snapshot").unwrap(),
//     //     std::fs::read("./../zebra.log").unwrap()
//     // );
// }
}
