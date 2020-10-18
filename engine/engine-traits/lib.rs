
pub trait CoeffSource {
    fn next_word(&mut self) -> i16;
    fn try_next_word(&mut self) -> Option<i16>;
}

// This is a bit hacky solution just so that I
// can easily replace pointers with references
pub trait Offset<T> {
    fn offset(self, count: isize) -> T;
}
impl<'a, 'b: 'a, T> Offset<&'a T> for &'b [T] {
    #[inline(always)]
    fn offset(self, count: isize) -> &'a T {
        &self[count as usize]
    }
}
impl<'a, 'b: 'a, T> Offset<&'a mut T> for &'b mut [T] {
    #[inline(always)]
    fn offset(self, count: isize) -> &'a mut T {
        &mut self[count as usize]
    }
}
impl Offset<usize> for usize {
    fn offset(self, count: isize) -> usize {
        (self as isize + count) as usize
    }
}
//
// impl<'a, 'b: 'a, T> Offset<&'a T> for &'b Vec<T> { // This thing causes conflicts with mut version
//     fn offset(self, count: isize) -> &'a T {
//         let slice: &[_] = self.as_ref();
//         &slice[count as usize]
//     }
// }
impl<'a, 'b: 'a, T> Offset<&'a mut T> for &'b mut Vec<T> {
    fn offset(self, count: isize) -> &'a mut T {
        let slice: &mut [_] = self.as_mut();
        &mut slice[count as usize]
    }
}
