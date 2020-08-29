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
        #[derive(Copy, Clone)]
        #[repr(C)]
        pub union C2RustUnnamed {
            pub signed_val: i16,
            pub unsigned_val: u16,
        }

        let mut val = C2RustUnnamed { signed_val: 0 };

        let hi = *self.data.get(self.index)? as i32;
        self.index += 1;
        let lo = *self.data.get(self.index)? as i32;
        self.index += 1;

        val.unsigned_val = ((hi << 8 as i32) + lo) as u16;
        return Some(unsafe { val.signed_val });
    }
}
