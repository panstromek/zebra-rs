use flate2_coeff_source::Flate2Source;
use std::ffi::CStr;

fn main() {}

#[test]
fn coeff_source_test () {
    use legacy_zebra::src::getcoeff::zlib_source::ZLibSource;
    use engine_traits::CoeffSource;
    let file_name: &CStr = CStr::from_bytes_with_nul(b"./../coeffs2.bin\x00").unwrap();

    let mut z_lib_source = ZLibSource::new(file_name);

    let mut flate2_source = Flate2Source::new_from_data(&std::fs::read("./../coeffs2.bin").unwrap());

    while let Some(word) = z_lib_source.try_next_word() {
        let flate_word = flate2_source.try_next_word().unwrap();
        assert_eq!(word, flate_word)
    }

    assert!(flate2_source.try_next_word().is_none());
}
