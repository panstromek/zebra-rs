use flate2_coeff_source::Flate2Source;
use std::ffi::CStr;
use legacy_zebra::src::getcoeff::new_z_lib_source;
use std::process::Command;

fn main() {}

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
        "./../zebra.log-snapshot"
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
    assert!(
        std::fs::read_to_string(snapshot_path)
            .unwrap()
            .lines()
            .skip(1)
            .filter(|line| !line.starts_with("-->"))
            .eq(std::fs::read_to_string("./../zebra.log")
                .unwrap()
                .lines()
                .skip(1)
                .filter(|line| !line.starts_with("-->")))
    );
}

#[test]
fn help_works() {
    let output = Command::new("./../target/release/zebra")
        .arg("?")
        .output()
        .unwrap();

    assert_eq!(
        std::fs::read("./../help-snapshot.txt").unwrap(),
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
