use engine_traits::CoeffSource;
use flate2::read::GzDecoder;
use std::io::Read;

pub struct Flate2Source { data: Vec<u8>, index: usize }

impl Flate2Source {
    pub fn new_from_data(data: &[u8]) -> Flate2Source {
        let mut decoder = GzDecoder::new(data);
        let mut decoded = Vec::new();
        decoder.read_to_end(&mut decoded).unwrap();

        let mut source = Flate2Source {
            data: decoded,
            index: 0,
        };

        let word1 = source.next_word();
        let word2 = source.next_word();

        if word1 != 5358 || word2 != 9793 {
            panic!("Magic words are incorrect in coeff source file.");
        }
        source
    }
}

impl CoeffSource for Flate2Source {
    fn next_word(&mut self) -> i16 {
        self.try_next_word().unwrap()
    }

    fn try_next_word(&mut self) -> Option<i16> {
        let hi = *self.data.get(self.index)? as i16;
        self.index += 1;
        let lo = *self.data.get(self.index)? as i16;
        self.index += 1;
        return Some(hi.wrapping_shl(8) + lo);
    }
}
