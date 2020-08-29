
pub trait CoeffSource {
    fn next_word(&mut self) -> i16;
    fn try_next_word(&mut self) -> Option<i16>;
}
