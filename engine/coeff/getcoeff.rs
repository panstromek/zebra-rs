use engine_traits::Offset;

#[repr(C)]
pub struct CoeffSet {
    pub permanent: i32,
    pub loaded: i32,
    pub prev: i32,
    pub next: i32,
    pub parity_constant: [i16; 2],
    pub parity: i16,
    pub constant: i16,
    pub data: Option<CoeffSetData>
}
impl CoeffSet {
    pub const fn new() -> CoeffSet {
        CoeffSet {
            permanent: 0,
            loaded: 0,
            prev: 0,
            next: 0,
            parity_constant: [0; 2],
            parity: 0,
            constant: 0,
            data: None
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct AllocationBlock {
    pub afile2x_block: [i16; 59049],
    pub bfile_block: [i16; 6561],
    pub cfile_block: [i16; 6561],
    pub dfile_block: [i16; 6561],
    pub diag8_block: [i16; 6561],
    pub diag7_block: [i16; 2187],
    pub diag6_block: [i16; 729],
    pub diag5_block: [i16; 243],
    pub diag4_block: [i16; 81],
    pub corner33_block: [i16; 19683],
    pub corner52_block: [i16; 59049],
}

#[repr(C)]
pub struct CoeffSetData {
    pub allocation: Box<AllocationBlock>, 
    // pub get_afile2x: &'a mut [i16; 59049],
    // pub get_bfile: &'a mut [i16; 6561],
    // pub get_cfile: &'a mut [i16; 6561],
    // pub get_dfile: &'a mut [i16; 6561],
    // pub get_diag8: &'a mut [i16; 6561],
    // pub get_diag7: &'a mut [i16; 2187],
    // pub get_diag6: &'a mut [i16; 729],
    // pub get_diag5: &'a mut [i16; 243],
    // pub get_diag4: &'a mut [i16; 81],
    // pub get_corner33: &'a mut [i16; 19683],
    // pub get_corner52: &'a mut [i16; 59049]
}

impl CoeffSetData {
    pub fn afile2x_mut(&mut self) -> &mut [i16; 59049] { &mut self.allocation.afile2x_block }
    pub fn bfile_mut(&mut self) -> &mut [i16; 6561] { &mut self.allocation.bfile_block }
    pub fn cfile_mut(&mut self) ->  &mut [i16; 6561] { &mut self.allocation.cfile_block }
    pub fn dfile_mut(&mut self) ->  &mut [i16; 6561] { &mut self.allocation.dfile_block }
    pub fn diag8_mut(&mut self) ->  &mut [i16; 6561] { &mut self.allocation.diag8_block }
    pub fn diag7_mut(&mut self) ->  &mut [i16; 2187] { &mut self.allocation.diag7_block }
    pub fn diag6_mut(&mut self) ->  &mut [i16; 729] { &mut self.allocation.diag6_block }
    pub fn diag5_mut(&mut self) ->  &mut [i16; 243] { &mut self.allocation.diag5_block }
    pub fn diag4_mut(&mut self) ->  &mut [i16; 81] { &mut self.allocation.diag4_block }
    pub fn corner33_mut(&mut self) ->  &mut [i16; 19683] { &mut self.allocation.corner33_block }
    pub fn corner52_mut(&mut self) ->  &mut [i16; 59049] { &mut self.allocation.corner52_block }

    pub fn afile2x(&self) -> &[i16; 59049] {  & self.allocation.afile2x_block  }
    pub fn bfile(&self) -> &[i16; 6561] {  & self.allocation.bfile_block  }
    pub fn cfile(&self) ->  &[i16; 6561] {  & self.allocation.cfile_block  }
    pub fn dfile(&self) ->  &[i16; 6561] {  & self.allocation.dfile_block  }
    pub fn diag8(&self) ->  &[i16; 6561] {  & self.allocation.diag8_block  }
    pub fn diag7(&self) ->  &[i16; 2187] {  & self.allocation.diag7_block  }
    pub fn diag6(&self) ->  &[i16; 729] {  & self.allocation.diag6_block  }
    pub fn diag5(&self) ->  &[i16; 243] {  & self.allocation.diag5_block  }
    pub fn diag4(&self) ->  &[i16; 81] {  & self.allocation.diag4_block  }
    pub fn corner33(&self) ->  &[i16; 19683] {  & self.allocation.corner33_block  }
    pub fn corner52(&self) ->  &[i16; 59049] {  & self.allocation.corner52_block  }
}

pub fn constant_and_parity_feature(side_to_move: i32, disks_played: i32,
                                          board: &[i32; 128], set: &mut CoeffSet) -> i32 {
    /* The constant feature and the parity feature */
    let mut score = set.parity_constant[(disks_played & 1 as i32) as usize];
    let set = set.data.as_mut().unwrap();

    // Following assert is an invariant of a board type in this program - it's not yet captured in a type
    // that would enforce it at compile time, but I'll do that eventually
    // assert!(board.iter().all(|&item| ([0, 1, 2, 3].iter().any(|&some| some == item))));

    // We know that board only contains (0,1,2,3) numbers and this function is called at most
    //  10 times in a row, so it can never overflow in here. Using this knowledge, we can use wrapping
    //  functions. Using this helper cuts down the number of generated llvm-ir lines by a huge number
    //  (from 14742 to 5442), which improves build time + runtime doesn't seem to be affected
    /// (tests still run in ~2.5 seconds like before this change)
    fn update_pattern( pat0:i32, board: &[i32; 128], index: usize) -> i32 {
        (3 as i32).wrapping_mul(pat0).wrapping_add(
            board[index]
        )
    }
    /* The pattern features. */
    if side_to_move == 0 as i32 {
        let mut pattern0: i32;
        pattern0 = board[72];
        pattern0 = update_pattern(pattern0, board, 22);
        pattern0 = update_pattern(pattern0, board, 81);
        pattern0 = update_pattern(pattern0, board, 71);
        pattern0 = update_pattern(pattern0, board, 61);
        pattern0 = update_pattern(pattern0, board, 51);
        pattern0 = update_pattern(pattern0, board, 41);
        pattern0 = update_pattern(pattern0, board, 31);
        pattern0 = update_pattern(pattern0, board, 21);
        pattern0 = update_pattern(pattern0, board, 11);
        score =(score as i32 +
                *set.afile2x_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[77];
        pattern0 = update_pattern(pattern0, board, 27);
        pattern0 = update_pattern(pattern0, board, 88);
        pattern0 = update_pattern(pattern0, board, 78);
        pattern0 = update_pattern(pattern0, board, 68);
        pattern0 = update_pattern(pattern0, board, 58);
        pattern0 = update_pattern(pattern0, board, 48);
        pattern0 = update_pattern(pattern0, board, 38);
        pattern0 = update_pattern(pattern0, board, 28);
        pattern0 = update_pattern(pattern0, board, 18);
        score =(score as i32 +
                *set.afile2x_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[27];
        pattern0 = update_pattern(pattern0, board, 22);
        pattern0 = update_pattern(pattern0, board, 18);
        pattern0 = update_pattern(pattern0, board, 17);
        pattern0 = update_pattern(pattern0, board, 16);
        pattern0 = update_pattern(pattern0, board, 15);
        pattern0 = update_pattern(pattern0, board, 14);
        pattern0 = update_pattern(pattern0, board, 13);
        pattern0 = update_pattern(pattern0, board, 12);
        pattern0 = update_pattern(pattern0, board, 11);
        score =(score as i32 +
                *set.afile2x_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[77];
        pattern0 = update_pattern(pattern0, board, 72);
        pattern0 = update_pattern(pattern0, board, 88);
        pattern0 = update_pattern(pattern0, board, 87);
        pattern0 = update_pattern(pattern0, board, 86);
        pattern0 = update_pattern(pattern0, board, 85);
        pattern0 = update_pattern(pattern0, board, 84);
        pattern0 = update_pattern(pattern0, board, 83);
        pattern0 = update_pattern(pattern0, board, 82);
        pattern0 = update_pattern(pattern0, board, 81);
        score =(score as i32 +
                *set.afile2x_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[82];
        pattern0 = update_pattern(pattern0, board, 72);
        pattern0 = update_pattern(pattern0, board, 62);
        pattern0 = update_pattern(pattern0, board, 52);
        pattern0 = update_pattern(pattern0, board, 42);
        pattern0 = update_pattern(pattern0, board, 32);
        pattern0 = update_pattern(pattern0, board, 22);
        pattern0 = update_pattern(pattern0, board, 12);
        score =(score as i32 +
                *set.bfile_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[87];
        pattern0 = update_pattern(pattern0, board, 77);
        pattern0 = update_pattern(pattern0, board, 67);
        pattern0 = update_pattern(pattern0, board, 57);
        pattern0 = update_pattern(pattern0, board, 47);
        pattern0 = update_pattern(pattern0, board, 37);
        pattern0 = update_pattern(pattern0, board, 27);
        pattern0 = update_pattern(pattern0, board, 17);
        score =(score as i32 +
                *set.bfile_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[28];
        pattern0 = update_pattern(pattern0, board, 27);
        pattern0 = update_pattern(pattern0, board, 26);
        pattern0 = update_pattern(pattern0, board, 25);
        pattern0 = update_pattern(pattern0, board, 24);
        pattern0 = update_pattern(pattern0, board, 23);
        pattern0 = update_pattern(pattern0, board, 22);
        pattern0 = update_pattern(pattern0, board, 21);
        score =(score as i32 +
                *set.bfile_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[78];
        pattern0 = update_pattern(pattern0, board, 77);
        pattern0 = update_pattern(pattern0, board, 76);
        pattern0 = update_pattern(pattern0, board, 75);
        pattern0 = update_pattern(pattern0, board, 74);
        pattern0 = update_pattern(pattern0, board, 73);
        pattern0 = update_pattern(pattern0, board, 72);
        pattern0 = update_pattern(pattern0, board, 71);
        score =(score as i32 +
                *set.bfile_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[83];
        pattern0 = update_pattern(pattern0, board, 73);
        pattern0 = update_pattern(pattern0, board, 63);
        pattern0 = update_pattern(pattern0, board, 53);
        pattern0 = update_pattern(pattern0, board, 43);
        pattern0 = update_pattern(pattern0, board, 33);
        pattern0 = update_pattern(pattern0, board, 23);
        pattern0 = update_pattern(pattern0, board, 13);
        score =(score as i32 +
                *set.cfile_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[86];
        pattern0 = update_pattern(pattern0, board, 76);
        pattern0 = update_pattern(pattern0, board, 66);
        pattern0 = update_pattern(pattern0, board, 56);
        pattern0 = update_pattern(pattern0, board, 46);
        pattern0 = update_pattern(pattern0, board, 36);
        pattern0 = update_pattern(pattern0, board, 26);
        pattern0 = update_pattern(pattern0, board, 16);
        score =(score as i32 +
                *set.cfile_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[38];
        pattern0 = update_pattern(pattern0, board, 37);
        pattern0 = update_pattern(pattern0, board, 36);
        pattern0 = update_pattern(pattern0, board, 35);
        pattern0 = update_pattern(pattern0, board, 34);
        pattern0 = update_pattern(pattern0, board, 33);
        pattern0 = update_pattern(pattern0, board, 32);
        pattern0 = update_pattern(pattern0, board, 31);
        score =(score as i32 +
                *set.cfile_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[68];
        pattern0 = update_pattern(pattern0, board, 67);
        pattern0 = update_pattern(pattern0, board, 66);
        pattern0 = update_pattern(pattern0, board, 65);
        pattern0 = update_pattern(pattern0, board, 64);
        pattern0 = update_pattern(pattern0, board, 63);
        pattern0 = update_pattern(pattern0, board, 62);
        pattern0 = update_pattern(pattern0, board, 61);
        score =(score as i32 +
                *set.cfile_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[84];
        pattern0 = update_pattern(pattern0, board, 74);
        pattern0 = update_pattern(pattern0, board, 64);
        pattern0 = update_pattern(pattern0, board, 54);
        pattern0 = update_pattern(pattern0, board, 44);
        pattern0 = update_pattern(pattern0, board, 34);
        pattern0 = update_pattern(pattern0, board, 24);
        pattern0 = update_pattern(pattern0, board, 14);
        score =(score as i32 +
                *set.dfile_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[85];
        pattern0 = update_pattern(pattern0, board, 75);
        pattern0 = update_pattern(pattern0, board, 65);
        pattern0 = update_pattern(pattern0, board, 55);
        pattern0 = update_pattern(pattern0, board, 45);
        pattern0 = update_pattern(pattern0, board, 35);
        pattern0 = update_pattern(pattern0, board, 25);
        pattern0 = update_pattern(pattern0, board, 15);
        score =(score as i32 +
                *set.dfile_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[48];
        pattern0 = update_pattern(pattern0, board, 47);
        pattern0 = update_pattern(pattern0, board, 46);
        pattern0 = update_pattern(pattern0, board, 45);
        pattern0 = update_pattern(pattern0, board, 44);
        pattern0 = update_pattern(pattern0, board, 43);
        pattern0 = update_pattern(pattern0, board, 42);
        pattern0 = update_pattern(pattern0, board, 41);
        score =(score as i32 +
                *set.dfile_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[58];
        pattern0 = update_pattern(pattern0, board, 57);
        pattern0 = update_pattern(pattern0, board, 56);
        pattern0 = update_pattern(pattern0, board, 55);
        pattern0 = update_pattern(pattern0, board, 54);
        pattern0 = update_pattern(pattern0, board, 53);
        pattern0 = update_pattern(pattern0, board, 52);
        pattern0 = update_pattern(pattern0, board, 51);
        score =(score as i32 +
                *set.dfile_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[88];
        pattern0 = update_pattern(pattern0, board, 77);
        pattern0 = update_pattern(pattern0, board, 66);
        pattern0 = update_pattern(pattern0, board, 55);
        pattern0 = update_pattern(pattern0, board, 44);
        pattern0 = update_pattern(pattern0, board, 33);
        pattern0 = update_pattern(pattern0, board, 22);
        pattern0 = update_pattern(pattern0, board, 11);
        score =(score as i32 +
                *set.diag8_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[81];
        pattern0 = update_pattern(pattern0, board, 72);
        pattern0 = update_pattern(pattern0, board, 63);
        pattern0 = update_pattern(pattern0, board, 54);
        pattern0 = update_pattern(pattern0, board, 45);
        pattern0 = update_pattern(pattern0, board, 36);
        pattern0 = update_pattern(pattern0, board, 27);
        pattern0 = update_pattern(pattern0, board, 18);
        score =(score as i32 +
                *set.diag8_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[78];
        pattern0 = update_pattern(pattern0, board, 67);
        pattern0 = update_pattern(pattern0, board, 56);
        pattern0 = update_pattern(pattern0, board, 45);
        pattern0 = update_pattern(pattern0, board, 34);
        pattern0 = update_pattern(pattern0, board, 23);
        pattern0 = update_pattern(pattern0, board, 12);
        score =(score as i32 +
                *set.diag7_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[87];
        pattern0 = update_pattern(pattern0, board, 76);
        pattern0 = update_pattern(pattern0, board, 65);
        pattern0 = update_pattern(pattern0, board, 54);
        pattern0 = update_pattern(pattern0, board, 43);
        pattern0 = update_pattern(pattern0, board, 32);
        pattern0 = update_pattern(pattern0, board, 21);
        score =(score as i32 +
                *set.diag7_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[71];
        pattern0 = update_pattern(pattern0, board, 62);
        pattern0 = update_pattern(pattern0, board, 53);
        pattern0 = update_pattern(pattern0, board, 44);
        pattern0 = update_pattern(pattern0, board, 35);
        pattern0 = update_pattern(pattern0, board, 26);
        pattern0 = update_pattern(pattern0, board, 17);
        score =(score as i32 +
                *set.diag7_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[82];
        pattern0 = update_pattern(pattern0, board, 73);
        pattern0 = update_pattern(pattern0, board, 64);
        pattern0 = update_pattern(pattern0, board, 55);
        pattern0 = update_pattern(pattern0, board, 46);
        pattern0 = update_pattern(pattern0, board, 37);
        pattern0 = update_pattern(pattern0, board, 28);
        score =(score as i32 +
                *set.diag7_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[68];
        pattern0 = update_pattern(pattern0, board, 57);
        pattern0 = update_pattern(pattern0, board, 46);
        pattern0 = update_pattern(pattern0, board, 35);
        pattern0 = update_pattern(pattern0, board, 24);
        pattern0 = update_pattern(pattern0, board, 13);
        score =(score as i32 +
                *set.diag6().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[86];
        pattern0 = update_pattern(pattern0, board, 75);
        pattern0 = update_pattern(pattern0, board, 64);
        pattern0 = update_pattern(pattern0, board, 53);
        pattern0 = update_pattern(pattern0, board, 42);
        pattern0 = update_pattern(pattern0, board, 31);
        score =(score as i32 +
                *set.diag6_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[61];
        pattern0 = update_pattern(pattern0, board, 52);
        pattern0 = update_pattern(pattern0, board, 43);
        pattern0 = update_pattern(pattern0, board, 34);
        pattern0 = update_pattern(pattern0, board, 25);
        pattern0 = update_pattern(pattern0, board, 16);
        score =(score as i32 +
                *set.diag6_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[83];
        pattern0 = update_pattern(pattern0, board, 74);
        pattern0 = update_pattern(pattern0, board, 65);
        pattern0 = update_pattern(pattern0, board, 56);
        pattern0 = update_pattern(pattern0, board, 47);
        pattern0 = update_pattern(pattern0, board, 38);
        score =(score as i32 +
                *set.diag6_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[58];
        pattern0 = update_pattern(pattern0, board, 47);
        pattern0 = update_pattern(pattern0, board, 36);
        pattern0 = update_pattern(pattern0, board, 25);
        pattern0 = update_pattern(pattern0, board, 14);
        score =(score as i32 +
                *set.diag5_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[85];
        pattern0 = update_pattern(pattern0, board, 74);
        pattern0 = update_pattern(pattern0, board, 63);
        pattern0 = update_pattern(pattern0, board, 52);
        pattern0 = update_pattern(pattern0, board, 41);
        score =(score as i32 +
                *set.diag5_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[51];
        pattern0 = update_pattern(pattern0, board, 42);
        pattern0 = update_pattern(pattern0, board, 33);
        pattern0 = update_pattern(pattern0, board, 24);
        pattern0 = update_pattern(pattern0, board, 15);
        score =(score as i32 +
                *set.diag5_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[84];
        pattern0 = update_pattern(pattern0, board, 75);
        pattern0 = update_pattern(pattern0, board, 66);
        pattern0 = update_pattern(pattern0, board, 57);
        pattern0 = update_pattern(pattern0, board, 48);
        score =(score as i32 +
                *set.diag5_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[48];
        pattern0 = update_pattern(pattern0, board, 37);
        pattern0 = update_pattern(pattern0, board, 26);
        pattern0 = update_pattern(pattern0, board, 15);
        score =(score as i32 +
                *set.diag4_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[84];
        pattern0 = update_pattern(pattern0, board, 73);
        pattern0 = update_pattern(pattern0, board, 62);
        pattern0 = update_pattern(pattern0, board, 51);
        score =(score as i32 +
                *set.diag4_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[41];
        pattern0 = update_pattern(pattern0, board, 32);
        pattern0 = update_pattern(pattern0, board, 23);
        pattern0 = update_pattern(pattern0, board, 14);
        score =(score as i32 +
                *set.diag4_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[85];
        pattern0 = update_pattern(pattern0, board, 76);
        pattern0 = update_pattern(pattern0, board, 67);
        pattern0 = update_pattern(pattern0, board, 58);
        score =(score as i32 +
                *set.diag4_mut().offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[33];
        pattern0 = update_pattern(pattern0, board, 32);
        pattern0 = update_pattern(pattern0, board, 31);
        pattern0 = update_pattern(pattern0, board, 23);
        pattern0 = update_pattern(pattern0, board, 22);
        pattern0 = update_pattern(pattern0, board, 21);
        pattern0 = update_pattern(pattern0, board, 13);
        pattern0 = update_pattern(pattern0, board, 12);
        pattern0 = update_pattern(pattern0, board, 11);
        score =(score as i32 +
                *set.corner33_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[63];
        pattern0 = update_pattern(pattern0, board, 62);
        pattern0 = update_pattern(pattern0, board, 61);
        pattern0 = update_pattern(pattern0, board, 73);
        pattern0 = update_pattern(pattern0, board, 72);
        pattern0 = update_pattern(pattern0, board, 71);
        pattern0 = update_pattern(pattern0, board, 83);
        pattern0 = update_pattern(pattern0, board, 82);
        pattern0 = update_pattern(pattern0, board, 81);
        score =(score as i32 +
                *set.corner33_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[36];
        pattern0 = update_pattern(pattern0, board, 37);
        pattern0 = update_pattern(pattern0, board, 38);
        pattern0 = update_pattern(pattern0, board, 26);
        pattern0 = update_pattern(pattern0, board, 27);
        pattern0 = update_pattern(pattern0, board, 28);
        pattern0 = update_pattern(pattern0, board, 16);
        pattern0 = update_pattern(pattern0, board, 17);
        pattern0 = update_pattern(pattern0, board, 18);
        score =(score as i32 +
                *set.corner33_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[66];
        pattern0 = update_pattern(pattern0, board, 67);
        pattern0 = update_pattern(pattern0, board, 68);
        pattern0 = update_pattern(pattern0, board, 76);
        pattern0 = update_pattern(pattern0, board, 77);
        pattern0 = update_pattern(pattern0, board, 78);
        pattern0 = update_pattern(pattern0, board, 86);
        pattern0 = update_pattern(pattern0, board, 87);
        pattern0 = update_pattern(pattern0, board, 88);
        score =(score as i32 +
                *set.corner33_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[25];
        pattern0 = update_pattern(pattern0, board, 24);
        pattern0 = update_pattern(pattern0, board, 23);
        pattern0 = update_pattern(pattern0, board, 22);
        pattern0 = update_pattern(pattern0, board, 21);
        pattern0 = update_pattern(pattern0, board, 15);
        pattern0 = update_pattern(pattern0, board, 14);
        pattern0 = update_pattern(pattern0, board, 13);
        pattern0 = update_pattern(pattern0, board, 12);
        pattern0 = update_pattern(pattern0, board, 11);
        score =(score as i32 +
                *set.corner52_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[75];
        pattern0 = update_pattern(pattern0, board, 74);
        pattern0 = update_pattern(pattern0, board, 73);
        pattern0 = update_pattern(pattern0, board, 72);
        pattern0 = update_pattern(pattern0, board, 71);
        pattern0 = update_pattern(pattern0, board, 85);
        pattern0 = update_pattern(pattern0, board, 84);
        pattern0 = update_pattern(pattern0, board, 83);
        pattern0 = update_pattern(pattern0, board, 82);
        pattern0 = update_pattern(pattern0, board, 81);
        score =(score as i32 +
                *set.corner52_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[24];
        pattern0 = update_pattern(pattern0, board, 25);
        pattern0 = update_pattern(pattern0, board, 26);
        pattern0 = update_pattern(pattern0, board, 27);
        pattern0 = update_pattern(pattern0, board, 28);
        pattern0 = update_pattern(pattern0, board, 14);
        pattern0 = update_pattern(pattern0, board, 15);
        pattern0 = update_pattern(pattern0, board, 16);
        pattern0 = update_pattern(pattern0, board, 17);
        pattern0 = update_pattern(pattern0, board, 18);
        score =(score as i32 +
                *set.corner52_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[74];
        pattern0 = update_pattern(pattern0, board, 75);
        pattern0 = update_pattern(pattern0, board, 76);
        pattern0 = update_pattern(pattern0, board, 77);
        pattern0 = update_pattern(pattern0, board, 78);
        pattern0 = update_pattern(pattern0, board, 84);
        pattern0 = update_pattern(pattern0, board, 85);
        pattern0 = update_pattern(pattern0, board, 86);
        pattern0 = update_pattern(pattern0, board, 87);
        pattern0 = update_pattern(pattern0, board, 88);
        score =(score as i32 +
                *set.corner52_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[52];
        pattern0 = update_pattern(pattern0, board, 42);
        pattern0 = update_pattern(pattern0, board, 32);
        pattern0 = update_pattern(pattern0, board, 22);
        pattern0 = update_pattern(pattern0, board, 12);
        pattern0 = update_pattern(pattern0, board, 51);
        pattern0 = update_pattern(pattern0, board, 41);
        pattern0 = update_pattern(pattern0, board, 31);
        pattern0 = update_pattern(pattern0, board, 21);
        pattern0 = update_pattern(pattern0, board, 11);
        score =(score as i32 +
                *set.corner52_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[57];
        pattern0 = update_pattern(pattern0, board, 47);
        pattern0 = update_pattern(pattern0, board, 37);
        pattern0 = update_pattern(pattern0, board, 27);
        pattern0 = update_pattern(pattern0, board, 17);
        pattern0 = update_pattern(pattern0, board, 58);
        pattern0 = update_pattern(pattern0, board, 48);
        pattern0 = update_pattern(pattern0, board, 38);
        pattern0 = update_pattern(pattern0, board, 28);
        pattern0 = update_pattern(pattern0, board, 18);
        score =(score as i32 +
                *set.corner52_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[42];
        pattern0 = update_pattern(pattern0, board, 52);
        pattern0 = update_pattern(pattern0, board, 62);
        pattern0 = update_pattern(pattern0, board, 72);
        pattern0 = update_pattern(pattern0, board, 82);
        pattern0 = update_pattern(pattern0, board, 41);
        pattern0 = update_pattern(pattern0, board, 51);
        pattern0 = update_pattern(pattern0, board, 61);
        pattern0 = update_pattern(pattern0, board, 71);
        pattern0 = update_pattern(pattern0, board, 81);
        score =(score as i32 +
                *set.corner52_mut().offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[47];
        pattern0 = update_pattern(pattern0, board, 57);
        pattern0 = update_pattern(pattern0, board, 67);
        pattern0 = update_pattern(pattern0, board, 77);
        pattern0 = update_pattern(pattern0, board, 87);
        pattern0 = update_pattern(pattern0, board, 48);
        pattern0 = update_pattern(pattern0, board, 58);
        pattern0 = update_pattern(pattern0, board, 68);
        pattern0 = update_pattern(pattern0, board, 78);
        pattern0 = update_pattern(pattern0, board, 88);
        score =(score as i32 +
                *set.corner52_mut().offset(pattern0 as isize)
                    as i32) as i16
    } else {
        let mut pattern0_0: i32;
        pattern0_0 = board[72];
        pattern0_0 = update_pattern(pattern0_0, board, 22);
        pattern0_0 = update_pattern(pattern0_0, board, 81);
        pattern0_0 = update_pattern(pattern0_0, board, 71);
        pattern0_0 = update_pattern(pattern0_0, board, 61);
        pattern0_0 = update_pattern(pattern0_0, board, 51);
        pattern0_0 = update_pattern(pattern0_0, board, 41);
        pattern0_0 = update_pattern(pattern0_0, board, 31);
        pattern0_0 = update_pattern(pattern0_0, board, 21);
        pattern0_0 = update_pattern(pattern0_0, board, 11);
        score =(score as i32 +
                *set.afile2x_mut().offset(59048-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[77];
        pattern0_0 = update_pattern(pattern0_0, board, 27);
        pattern0_0 = update_pattern(pattern0_0, board, 88);
        pattern0_0 = update_pattern(pattern0_0, board, 78);
        pattern0_0 = update_pattern(pattern0_0, board, 68);
        pattern0_0 = update_pattern(pattern0_0, board, 58);
        pattern0_0 = update_pattern(pattern0_0, board, 48);
        pattern0_0 = update_pattern(pattern0_0, board, 38);
        pattern0_0 = update_pattern(pattern0_0, board, 28);
        pattern0_0 = update_pattern(pattern0_0, board, 18);
        score =(score as i32 +
                *set.afile2x_mut().offset(59048-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[27];
        pattern0_0 = update_pattern(pattern0_0, board, 22);
        pattern0_0 = update_pattern(pattern0_0, board, 18);
        pattern0_0 = update_pattern(pattern0_0, board, 17);
        pattern0_0 = update_pattern(pattern0_0, board, 16);
        pattern0_0 = update_pattern(pattern0_0, board, 15);
        pattern0_0 = update_pattern(pattern0_0, board, 14);
        pattern0_0 = update_pattern(pattern0_0, board, 13);
        pattern0_0 = update_pattern(pattern0_0, board, 12);
        pattern0_0 = update_pattern(pattern0_0, board, 11);
        score =(score as i32 +
                *set.afile2x_mut().offset(59048-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[77];
        pattern0_0 = update_pattern(pattern0_0, board, 72);
        pattern0_0 = update_pattern(pattern0_0, board, 88);
        pattern0_0 = update_pattern(pattern0_0, board, 87);
        pattern0_0 = update_pattern(pattern0_0, board, 86);
        pattern0_0 = update_pattern(pattern0_0, board, 85);
        pattern0_0 = update_pattern(pattern0_0, board, 84);
        pattern0_0 = update_pattern(pattern0_0, board, 83);
        pattern0_0 = update_pattern(pattern0_0, board, 82);
        pattern0_0 = update_pattern(pattern0_0, board, 81);
        score =(score as i32 +
            *set.afile2x_mut().offset(59048 - pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[82];
        pattern0_0 = update_pattern(pattern0_0, board, 72);
        pattern0_0 = update_pattern(pattern0_0, board, 62);
        pattern0_0 = update_pattern(pattern0_0, board, 52);
        pattern0_0 = update_pattern(pattern0_0, board, 42);
        pattern0_0 = update_pattern(pattern0_0, board, 32);
        pattern0_0 = update_pattern(pattern0_0, board, 22);
        pattern0_0 = update_pattern(pattern0_0, board, 12);
        score =(score as i32 +
                *set.bfile_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[87];
        pattern0_0 = update_pattern(pattern0_0, board, 77);
        pattern0_0 = update_pattern(pattern0_0, board, 67);
        pattern0_0 = update_pattern(pattern0_0, board, 57);
        pattern0_0 = update_pattern(pattern0_0, board, 47);
        pattern0_0 = update_pattern(pattern0_0, board, 37);
        pattern0_0 = update_pattern(pattern0_0, board, 27);
        pattern0_0 = update_pattern(pattern0_0, board, 17);
        score =(score as i32 +
                *set.bfile_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[28];
        pattern0_0 = update_pattern(pattern0_0, board, 27);
        pattern0_0 = update_pattern(pattern0_0, board, 26);
        pattern0_0 = update_pattern(pattern0_0, board, 25);
        pattern0_0 = update_pattern(pattern0_0, board, 24);
        pattern0_0 = update_pattern(pattern0_0, board, 23);
        pattern0_0 = update_pattern(pattern0_0, board, 22);
        pattern0_0 = update_pattern(pattern0_0, board, 21);
        score =(score as i32 +
                *set.bfile_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[78];
        pattern0_0 = update_pattern(pattern0_0, board, 77);
        pattern0_0 = update_pattern(pattern0_0, board, 76);
        pattern0_0 = update_pattern(pattern0_0, board, 75);
        pattern0_0 = update_pattern(pattern0_0, board, 74);
        pattern0_0 = update_pattern(pattern0_0, board, 73);
        pattern0_0 = update_pattern(pattern0_0, board, 72);
        pattern0_0 = update_pattern(pattern0_0, board, 71);
        score =(score as i32 +
                *set.bfile_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[83];
        pattern0_0 = update_pattern(pattern0_0, board, 73);
        pattern0_0 = update_pattern(pattern0_0, board, 63);
        pattern0_0 = update_pattern(pattern0_0, board, 53);
        pattern0_0 = update_pattern(pattern0_0, board, 43);
        pattern0_0 = update_pattern(pattern0_0, board, 33);
        pattern0_0 = update_pattern(pattern0_0, board, 23);
        pattern0_0 = update_pattern(pattern0_0, board, 13);
        score =(score as i32 +
                *set.cfile_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[86];
        pattern0_0 = update_pattern(pattern0_0, board, 76);
        pattern0_0 = update_pattern(pattern0_0, board, 66);
        pattern0_0 = update_pattern(pattern0_0, board, 56);
        pattern0_0 = update_pattern(pattern0_0, board, 46);
        pattern0_0 = update_pattern(pattern0_0, board, 36);
        pattern0_0 = update_pattern(pattern0_0, board, 26);
        pattern0_0 = update_pattern(pattern0_0, board, 16);
        score =(score as i32 +
                *set.cfile_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[38];
        pattern0_0 = update_pattern(pattern0_0, board, 37);
        pattern0_0 = update_pattern(pattern0_0, board, 36);
        pattern0_0 = update_pattern(pattern0_0, board, 35);
        pattern0_0 = update_pattern(pattern0_0, board, 34);
        pattern0_0 = update_pattern(pattern0_0, board, 33);
        pattern0_0 = update_pattern(pattern0_0, board, 32);
        pattern0_0 = update_pattern(pattern0_0, board, 31);
        score =(score as i32 +
                *set.cfile_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[68];
        pattern0_0 = update_pattern(pattern0_0, board, 67);
        pattern0_0 = update_pattern(pattern0_0, board, 66);
        pattern0_0 = update_pattern(pattern0_0, board, 65);
        pattern0_0 = update_pattern(pattern0_0, board, 64);
        pattern0_0 = update_pattern(pattern0_0, board, 63);
        pattern0_0 = update_pattern(pattern0_0, board, 62);
        pattern0_0 = update_pattern(pattern0_0, board, 61);
        score =(score as i32 +
                *set.cfile_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[84];
        pattern0_0 = update_pattern(pattern0_0, board, 74);
        pattern0_0 = update_pattern(pattern0_0, board, 64);
        pattern0_0 = update_pattern(pattern0_0, board, 54);
        pattern0_0 = update_pattern(pattern0_0, board, 44);
        pattern0_0 = update_pattern(pattern0_0, board, 34);
        pattern0_0 = update_pattern(pattern0_0, board, 24);
        pattern0_0 = update_pattern(pattern0_0, board, 14);
        score =(score as i32 +
                *set.dfile_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[85];
        pattern0_0 = update_pattern(pattern0_0, board, 75);
        pattern0_0 = update_pattern(pattern0_0, board, 65);
        pattern0_0 = update_pattern(pattern0_0, board, 55);
        pattern0_0 = update_pattern(pattern0_0, board, 45);
        pattern0_0 = update_pattern(pattern0_0, board, 35);
        pattern0_0 = update_pattern(pattern0_0, board, 25);
        pattern0_0 = update_pattern(pattern0_0, board, 15);
        score =(score as i32 +
                *set.dfile_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[48];
        pattern0_0 = update_pattern(pattern0_0, board, 47);
        pattern0_0 = update_pattern(pattern0_0, board, 46);
        pattern0_0 = update_pattern(pattern0_0, board, 45);
        pattern0_0 = update_pattern(pattern0_0, board, 44);
        pattern0_0 = update_pattern(pattern0_0, board, 43);
        pattern0_0 = update_pattern(pattern0_0, board, 42);
        pattern0_0 = update_pattern(pattern0_0, board, 41);
        score =(score as i32 +
                *set.dfile_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[58];
        pattern0_0 = update_pattern(pattern0_0, board, 57);
        pattern0_0 = update_pattern(pattern0_0, board, 56);
        pattern0_0 = update_pattern(pattern0_0, board, 55);
        pattern0_0 = update_pattern(pattern0_0, board, 54);
        pattern0_0 = update_pattern(pattern0_0, board, 53);
        pattern0_0 = update_pattern(pattern0_0, board, 52);
        pattern0_0 = update_pattern(pattern0_0, board, 51);
        score =(score as i32 +
                *set.dfile_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[88];
        pattern0_0 = update_pattern(pattern0_0, board, 77);
        pattern0_0 = update_pattern(pattern0_0, board, 66);
        pattern0_0 = update_pattern(pattern0_0, board, 55);
        pattern0_0 = update_pattern(pattern0_0, board, 44);
        pattern0_0 = update_pattern(pattern0_0, board, 33);
        pattern0_0 = update_pattern(pattern0_0, board, 22);
        pattern0_0 = update_pattern(pattern0_0, board, 11);
        score =(score as i32 +
                *set.diag8_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[81];
        pattern0_0 = update_pattern(pattern0_0, board, 72);
        pattern0_0 = update_pattern(pattern0_0, board, 63);
        pattern0_0 = update_pattern(pattern0_0, board, 54);
        pattern0_0 = update_pattern(pattern0_0, board, 45);
        pattern0_0 = update_pattern(pattern0_0, board, 36);
        pattern0_0 = update_pattern(pattern0_0, board, 27);
        pattern0_0 = update_pattern(pattern0_0, board, 18);
        score =(score as i32 +
                *set.diag8_mut().offset(6560-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[78];
        pattern0_0 = update_pattern(pattern0_0, board, 67);
        pattern0_0 = update_pattern(pattern0_0, board, 56);
        pattern0_0 = update_pattern(pattern0_0, board, 45);
        pattern0_0 = update_pattern(pattern0_0, board, 34);
        pattern0_0 = update_pattern(pattern0_0, board, 23);
        pattern0_0 = update_pattern(pattern0_0, board, 12);
        score =(score as i32 +
                *set.diag7_mut().offset(2186-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[87];
        pattern0_0 = update_pattern(pattern0_0, board, 76);
        pattern0_0 = update_pattern(pattern0_0, board, 65);
        pattern0_0 = update_pattern(pattern0_0, board, 54);
        pattern0_0 = update_pattern(pattern0_0, board, 43);
        pattern0_0 = update_pattern(pattern0_0, board, 32);
        pattern0_0 = update_pattern(pattern0_0, board, 21);
        score =(score as i32 +
                *set.diag7_mut().offset(2186-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[71];
        pattern0_0 = update_pattern(pattern0_0, board, 62);
        pattern0_0 = update_pattern(pattern0_0, board, 53);
        pattern0_0 = update_pattern(pattern0_0, board, 44);
        pattern0_0 = update_pattern(pattern0_0, board, 35);
        pattern0_0 = update_pattern(pattern0_0, board, 26);
        pattern0_0 = update_pattern(pattern0_0, board, 17);
        score =(score as i32 +
                *set.diag7_mut().offset(2186-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[82];
        pattern0_0 = update_pattern(pattern0_0, board, 73);
        pattern0_0 = update_pattern(pattern0_0, board, 64);
        pattern0_0 = update_pattern(pattern0_0, board, 55);
        pattern0_0 = update_pattern(pattern0_0, board, 46);
        pattern0_0 = update_pattern(pattern0_0, board, 37);
        pattern0_0 = update_pattern(pattern0_0, board, 28);
        score =(score as i32 +
                *set.diag7_mut().offset(2186-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[68];
        pattern0_0 = update_pattern(pattern0_0, board, 57);
        pattern0_0 = update_pattern(pattern0_0, board, 46);
        pattern0_0 = update_pattern(pattern0_0, board, 35);
        pattern0_0 = update_pattern(pattern0_0, board, 24);
        pattern0_0 = update_pattern(pattern0_0, board, 13);
        score =(score as i32 +
                *set.diag6_mut().offset(728-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[86];
        pattern0_0 = update_pattern(pattern0_0, board, 75);
        pattern0_0 = update_pattern(pattern0_0, board, 64);
        pattern0_0 = update_pattern(pattern0_0, board, 53);
        pattern0_0 = update_pattern(pattern0_0, board, 42);
        pattern0_0 = update_pattern(pattern0_0, board, 31);
        score =(score as i32 +
                *set.diag6_mut().offset(728-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[61];
        pattern0_0 = update_pattern(pattern0_0, board, 52);
        pattern0_0 = update_pattern(pattern0_0, board, 43);
        pattern0_0 = update_pattern(pattern0_0, board, 34);
        pattern0_0 = update_pattern(pattern0_0, board, 25);
        pattern0_0 = update_pattern(pattern0_0, board, 16);
        score =(score as i32 +
                *set.diag6_mut().offset(728-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[83];
        pattern0_0 = update_pattern(pattern0_0, board, 74);
        pattern0_0 = update_pattern(pattern0_0, board, 65);
        pattern0_0 = update_pattern(pattern0_0, board, 56);
        pattern0_0 = update_pattern(pattern0_0, board, 47);
        pattern0_0 = update_pattern(pattern0_0, board, 38);
        score =(score as i32 +
                *set.diag6_mut().offset(728-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[58];
        pattern0_0 = update_pattern(pattern0_0, board, 47);
        pattern0_0 = update_pattern(pattern0_0, board, 36);
        pattern0_0 = update_pattern(pattern0_0, board, 25);
        pattern0_0 = update_pattern(pattern0_0, board, 14);
        score =(score as i32 +
                *set.diag5_mut().offset(242-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[85];
        pattern0_0 = update_pattern(pattern0_0, board, 74);
        pattern0_0 = update_pattern(pattern0_0, board, 63);
        pattern0_0 = update_pattern(pattern0_0, board, 52);
        pattern0_0 = update_pattern(pattern0_0, board, 41);
        score =(score as i32 +
                *set.diag5_mut().offset(242-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[51];
        pattern0_0 = update_pattern(pattern0_0, board, 42);
        pattern0_0 = update_pattern(pattern0_0, board, 33);
        pattern0_0 = update_pattern(pattern0_0, board, 24);
        pattern0_0 = update_pattern(pattern0_0, board, 15);
        score =(score as i32 +
                *set.diag5_mut().offset(242-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[84];
        pattern0_0 = update_pattern(pattern0_0, board, 75);
        pattern0_0 = update_pattern(pattern0_0, board, 66);
        pattern0_0 = update_pattern(pattern0_0, board, 57);
        pattern0_0 = update_pattern(pattern0_0, board, 48);
        score =(score as i32 +
                *set.diag5_mut().offset(242-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[48];
        pattern0_0 = update_pattern(pattern0_0, board, 37);
        pattern0_0 = update_pattern(pattern0_0, board, 26);
        pattern0_0 = update_pattern(pattern0_0, board, 15);
        score =(score as i32 +
                *set.diag4_mut().offset(80-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[84];
        pattern0_0 = update_pattern(pattern0_0, board, 73);
        pattern0_0 = update_pattern(pattern0_0, board, 62);
        pattern0_0 = update_pattern(pattern0_0, board, 51);
        score =(score as i32 +
                *set.diag4_mut().offset(80-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[41];
        pattern0_0 = update_pattern(pattern0_0, board, 32);
        pattern0_0 = update_pattern(pattern0_0, board, 23);
        pattern0_0 = update_pattern(pattern0_0, board, 14);
        score =(score as i32 +
                *set.diag4_mut().offset(80-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[85];
        pattern0_0 = update_pattern(pattern0_0, board, 76);
        pattern0_0 = update_pattern(pattern0_0, board, 67);
        pattern0_0 = update_pattern(pattern0_0, board, 58);
        score =(score as i32 +
                *set.diag4_mut().offset(80-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[33];
        pattern0_0 = update_pattern(pattern0_0, board, 32);
        pattern0_0 = update_pattern(pattern0_0, board, 31);
        pattern0_0 = update_pattern(pattern0_0, board, 23);
        pattern0_0 = update_pattern(pattern0_0, board, 22);
        pattern0_0 = update_pattern(pattern0_0, board, 21);
        pattern0_0 = update_pattern(pattern0_0, board, 13);
        pattern0_0 = update_pattern(pattern0_0, board, 12);
        pattern0_0 = update_pattern(pattern0_0, board, 11);
        score =(score as i32 +
                *set.corner33_mut().offset(19682-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[63];
        pattern0_0 = update_pattern(pattern0_0, board, 62);
        pattern0_0 = update_pattern(pattern0_0, board, 61);
        pattern0_0 = update_pattern(pattern0_0, board, 73);
        pattern0_0 = update_pattern(pattern0_0, board, 72);
        pattern0_0 = update_pattern(pattern0_0, board, 71);
        pattern0_0 = update_pattern(pattern0_0, board, 83);
        pattern0_0 = update_pattern(pattern0_0, board, 82);
        pattern0_0 = update_pattern(pattern0_0, board, 81);
        score =(score as i32 +
                *set.corner33_mut().offset(19682-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[36];
        pattern0_0 = update_pattern(pattern0_0, board, 37);
        pattern0_0 = update_pattern(pattern0_0, board, 38);
        pattern0_0 = update_pattern(pattern0_0, board, 26);
        pattern0_0 = update_pattern(pattern0_0, board, 27);
        pattern0_0 = update_pattern(pattern0_0, board, 28);
        pattern0_0 = update_pattern(pattern0_0, board, 16);
        pattern0_0 = update_pattern(pattern0_0, board, 17);
        pattern0_0 = update_pattern(pattern0_0, board, 18);
        score =(score as i32 +
                *set.corner33_mut().offset(19682-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[66];
        pattern0_0 = update_pattern(pattern0_0, board, 67);
        pattern0_0 = update_pattern(pattern0_0, board, 68);
        pattern0_0 = update_pattern(pattern0_0, board, 76);
        pattern0_0 = update_pattern(pattern0_0, board, 77);
        pattern0_0 = update_pattern(pattern0_0, board, 78);
        pattern0_0 = update_pattern(pattern0_0, board, 86);
        pattern0_0 = update_pattern(pattern0_0, board, 87);
        pattern0_0 = update_pattern(pattern0_0, board, 88);
        score =(score as i32 +
                *set.corner33_mut().offset(19682-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[25];
        pattern0_0 = update_pattern(pattern0_0, board, 24);
        pattern0_0 = update_pattern(pattern0_0, board, 23);
        pattern0_0 = update_pattern(pattern0_0, board, 22);
        pattern0_0 = update_pattern(pattern0_0, board, 21);
        pattern0_0 = update_pattern(pattern0_0, board, 15);
        pattern0_0 = update_pattern(pattern0_0, board, 14);
        pattern0_0 = update_pattern(pattern0_0, board, 13);
        pattern0_0 = update_pattern(pattern0_0, board, 12);
        pattern0_0 = update_pattern(pattern0_0, board, 11);
        score =(score as i32 +
                *set.corner52_mut().offset(59048-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[75];
        pattern0_0 = update_pattern(pattern0_0, board, 74);
        pattern0_0 = update_pattern(pattern0_0, board, 73);
        pattern0_0 = update_pattern(pattern0_0, board, 72);
        pattern0_0 = update_pattern(pattern0_0, board, 71);
        pattern0_0 = update_pattern(pattern0_0, board, 85);
        pattern0_0 = update_pattern(pattern0_0, board, 84);
        pattern0_0 = update_pattern(pattern0_0, board, 83);
        pattern0_0 = update_pattern(pattern0_0, board, 82);
        pattern0_0 = update_pattern(pattern0_0, board, 81);
        score =(score as i32 +
                *set.corner52_mut().offset(59048-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[24];
        pattern0_0 = update_pattern(pattern0_0, board, 25);
        pattern0_0 = update_pattern(pattern0_0, board, 26);
        pattern0_0 = update_pattern(pattern0_0, board, 27);
        pattern0_0 = update_pattern(pattern0_0, board, 28);
        pattern0_0 = update_pattern(pattern0_0, board, 14);
        pattern0_0 = update_pattern(pattern0_0, board, 15);
        pattern0_0 = update_pattern(pattern0_0, board, 16);
        pattern0_0 = update_pattern(pattern0_0, board, 17);
        pattern0_0 = update_pattern(pattern0_0, board, 18);
        score =(score as i32 +
                *set.corner52_mut().offset(59048-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[74];
        pattern0_0 = update_pattern(pattern0_0, board, 75);
        pattern0_0 = update_pattern(pattern0_0, board, 76);
        pattern0_0 = update_pattern(pattern0_0, board, 77);
        pattern0_0 = update_pattern(pattern0_0, board, 78);
        pattern0_0 = update_pattern(pattern0_0, board, 84);
        pattern0_0 = update_pattern(pattern0_0, board, 85);
        pattern0_0 = update_pattern(pattern0_0, board, 86);
        pattern0_0 = update_pattern(pattern0_0, board, 87);
        pattern0_0 = update_pattern(pattern0_0, board, 88);
        score =(score as i32 +
                *set.corner52_mut().offset(59048-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[52];
        pattern0_0 = update_pattern(pattern0_0, board, 42);
        pattern0_0 = update_pattern(pattern0_0, board, 32);
        pattern0_0 = update_pattern(pattern0_0, board, 22);
        pattern0_0 = update_pattern(pattern0_0, board, 12);
        pattern0_0 = update_pattern(pattern0_0, board, 51);
        pattern0_0 = update_pattern(pattern0_0, board, 41);
        pattern0_0 = update_pattern(pattern0_0, board, 31);
        pattern0_0 = update_pattern(pattern0_0, board, 21);
        pattern0_0 = update_pattern(pattern0_0, board, 11);
        score =(score as i32 +
                *set.corner52_mut().offset(59048-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[57];
        pattern0_0 = update_pattern(pattern0_0, board, 47);
        pattern0_0 = update_pattern(pattern0_0, board, 37);
        pattern0_0 = update_pattern(pattern0_0, board, 27);
        pattern0_0 = update_pattern(pattern0_0, board, 17);
        pattern0_0 = update_pattern(pattern0_0, board, 58);
        pattern0_0 = update_pattern(pattern0_0, board, 48);
        pattern0_0 = update_pattern(pattern0_0, board, 38);
        pattern0_0 = update_pattern(pattern0_0, board, 28);
        pattern0_0 = update_pattern(pattern0_0, board, 18);
        score =(score as i32 +
                *set.corner52_mut().offset(59048-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[42];
        pattern0_0 = update_pattern(pattern0_0, board, 52);
        pattern0_0 = update_pattern(pattern0_0, board, 62);
        pattern0_0 = update_pattern(pattern0_0, board, 72);
        pattern0_0 = update_pattern(pattern0_0, board, 82);
        pattern0_0 = update_pattern(pattern0_0, board, 41);
        pattern0_0 = update_pattern(pattern0_0, board, 51);
        pattern0_0 = update_pattern(pattern0_0, board, 61);
        pattern0_0 = update_pattern(pattern0_0, board, 71);
        pattern0_0 = update_pattern(pattern0_0, board, 81);
        score =(score as i32 +
                *set.corner52_mut().offset(59048-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[47];
        pattern0_0 = update_pattern(pattern0_0, board, 57);
        pattern0_0 = update_pattern(pattern0_0, board, 67);
        pattern0_0 = update_pattern(pattern0_0, board, 77);
        pattern0_0 = update_pattern(pattern0_0, board, 87);
        pattern0_0 = update_pattern(pattern0_0, board, 48);
        pattern0_0 = update_pattern(pattern0_0, board, 58);
        pattern0_0 = update_pattern(pattern0_0, board, 68);
        pattern0_0 = update_pattern(pattern0_0, board, 78);
        pattern0_0 = update_pattern(pattern0_0, board, 88);
        score =(score as i32 +
                *set.corner52_mut().offset(59048-pattern0_0 as isize) as
                    i32) as i16
    }
    return score as i32;
}
// FIXME get rid of this stub
#[inline(always)]
pub fn floor(num: f64) -> f64{
    num.floor()
}

/*
   TERMINAL_PATTERNS
   Calculates the patterns associated with a filled board,
   only counting discs.
*/
pub fn terminal_patterns(coeff_set: &mut CoeffSet) {
    let coeff_set = coeff_set.data.as_mut().unwrap();
    /* Calculate the patterns which correspond to the board being filled */
    let value = create_terminal_pattern();
    let mut row: [i32; 10] = [0; 10];
    let mut result: f64;
    let mut j = 0;
    let mut k = 0;
    let mut i = 0;
    while i < 59049 as i32 {
        result = 0.0f64;
        j = 0;
        while j < 8 as i32 {
            if row[j as usize] == 0 as i32 {
                result += value[0][j as usize]
            } else if row[j as usize] == 2 as i32 {
                result -= value[0][j as usize]
            }
            j += 1
        }
        if row[8] == 0 as i32 {
            result +=value[1][1]
        } else if row[8] == 2 as i32 {
            result -=value[1][1]
        }
        if row[9] == 0 as i32 {
            result +=value[1][6]
        } else if row[9] == 2 as i32 {
            result -=value[1][6]
        }
        coeff_set.afile2x_mut()[i as usize] = floor(result * 128.0f64 + 0.5f64) as i16;
        result = 0.0f64;
        j = 0;
        while j < 5 as i32 {
            k = 0;
            while k < 2 as i32 {
                if row[(5 as i32 * k + j) as usize] ==0 as i32 {
                    result += value[j as usize][k as usize]
                } else if row[(5 as i32 * k + j) as usize] ==2 as i32 {
                    result -= value[j as usize][k as usize]
                }
                k += 1
            }
            j += 1
        }
        coeff_set.corner52_mut()[i as usize] =floor(result * 128.0f64 + 0.5f64) as i16;
        if i < 19683 as i32 {
            result = 0.0f64;
            j = 0;
            while j < 3 as i32 {
                k = 0;
                while k < 3 as i32 {
                    if row[(3 as i32 * j + k) as usize] ==0 as i32 {
                        result += value[j as usize][k as usize]
                    } else if row[(3 as i32 * j + k) as usize] ==2 as i32 {
                        result -= value[j as usize][k as usize]
                    }
                    k += 1
                }
                j += 1
            }
            coeff_set.corner33_mut()[i as usize] =floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 6561 as i32 {
            result = 0.0f64;
            j = 0;
            while j < 8 as i32 {
                if row[j as usize] == 0 as i32 {
                    result += value[1][j as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -= value[1][j as usize]
                }
                j += 1
            }
            coeff_set.bfile_mut()[i as usize] = floor(result * 128.0f64 + 0.5f64) as i16;
            result = 0.0f64;
            j = 0;
            while j < 8 as i32 {
                if row[j as usize] == 0 as i32 {
                    result += value[2][j as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -= value[2][j as usize]
                }
                j += 1
            }
            coeff_set.cfile_mut()[i as usize] = floor(result * 128.0f64 + 0.5f64) as i16;
            result = 0.0f64;
            j = 0;
            while j < 8 as i32 {
                if row[j as usize] == 0 as i32 {
                    result += value[3][j as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -= value[3][j as usize]
                }
                j += 1
            }
            coeff_set.dfile_mut()[i as usize] =floor(result * 128.0f64 + 0.5f64) as i16;
            result = 0.0f64;
            j = 0;
            while j < 8 as i32 {
                if row[j as usize] == 0 as i32 {
                    result += value[j as usize][j as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -= value[j as usize][j as usize]
                }
                j += 1
            }
            coeff_set.diag8_mut()[i as usize] =floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 2187 as i32 {
            result = 0.0f64;
            j = 0;
            while j < 7 as i32 {
                if row[j as usize] == 0 as i32 {
                    result +=value[j as usize][(j + 1 as i32) as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -=value[j as usize][(j + 1 as i32) as usize]
                }
                j += 1
            }
            coeff_set.diag7_mut()[i as usize] =floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 729 as i32 {
            result = 0.0f64;
            j = 0;
            while j < 6 as i32 {
                if row[j as usize] == 0 as i32 {
                    result +=value[j as usize][(j + 2 as i32) as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -=value[j as usize][(j + 2 as i32) as usize]
                }
                j += 1
            }
            coeff_set.diag6_mut()[i as usize] =floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 243 as i32 {
            result = 0.0f64;
            j = 0;
            while j < 5 as i32 {
                if row[j as usize] == 0 as i32 {
                    result +=value[j as usize][(j + 3 as i32) as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -=value[j as usize][(j + 3 as i32) as usize]
                }
                j += 1
            }
            coeff_set.diag5_mut()[i as usize] =floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 81 as i32 {
            result = 0.0f64;
            j = 0;
            while j < 4 as i32 {
                if row[j as usize] == 0 as i32 {
                    result +=value[j as usize][(j + 4 as i32) as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -=value[j as usize][(j + 4 as i32) as usize]
                }
                j += 1
            }
            coeff_set.diag4_mut()[i as usize] = floor(result * 128.0f64 + 0.5f64) as i16
        }
        /* Next configuration */
        j = 0;
        loop  {
            /* The odometer principle */
            row[j as usize] += 1;
            if row[j as usize] == 3 as i32 {
                row[j as usize] = 0 as i32
            }
            j += 1;
            if !(row[(j - 1 as i32) as usize] == 0 as i32 &&
                j < 10 as i32) {
                break ;
            }
        }
        i += 1
    };
}

fn create_terminal_pattern() -> [[f64; 8]; 8] {
    static HIT: [[i32; 8]; 8] = create_hit();

    let mut i = 0;
    let mut value: [[f64; 8]; 8] = [[0.; 8]; 8];
    while i < 8 as i32 {
        let mut j = 0;
        while j < 8 as i32 {
            value[i as usize][j as usize] = 1.0f64 / HIT[i as usize][j as usize] as f64;
            j += 1
        }
        i += 1
    }
    i = 0;
    value
}

const fn create_hit() -> [[i32; 8]; 8] {
    let mut j: i32;
    let mut hit: [[i32; 8]; 8] = [[0; 8]; 8];
    /* Count the number of times each square is counted */

    let mut i = 0;
    while i < 8 as i32 {
        hit[0][i as usize] += 1;
        hit[i as usize][0] += 1;
        hit[7][i as usize] += 1;
        hit[i as usize][7] += 1;
        i += 1
    }
    i = 0;
    while i < 8 as i32 {
        hit[1][i as usize] += 1;
        hit[i as usize][1] += 1;
        hit[6][i as usize] += 1;
        hit[i as usize][6] += 1;
        i += 1
    }
    i = 0;
    while i < 8 as i32 {
        hit[2][i as usize] += 1;
        hit[i as usize][2] += 1;
        hit[5][i as usize] += 1;
        hit[i as usize][5] += 1;
        i += 1
    }
    i = 0;
    while i < 8 as i32 {
        hit[3][i as usize] += 1;
        hit[i as usize][3] += 1;
        hit[4][i as usize] += 1;
        hit[i as usize][4] += 1;
        i += 1
    }
    i = 0;
    while i < 3 as i32 {
        j = 0;
        while j < 3 as i32 {
            hit[i as usize][j as usize] += 1;
            hit[i as usize][(7 as i32 - j) as usize] += 1;
            hit[(7 as i32 - i) as usize][j as usize] += 1;
            hit[(7 as i32 - i) as
                usize][(7 as i32 - j) as usize] += 1;
            j += 1
        }
        i += 1
    }
    i = 0;
    while i < 2 as i32 {
        j = 0;
        while j < 5 as i32 {
            hit[i as usize][j as usize] += 1;
            hit[j as usize][i as usize] += 1;
            hit[i as usize][(7 as i32 - j) as usize] += 1;
            hit[j as usize][(7 as i32 - i) as usize] += 1;
            hit[(7 as i32 - i) as usize][j as usize] += 1;
            hit[(7 as i32 - j) as usize][i as usize] += 1;
            hit[(7 as i32 - i) as
                usize][(7 as i32 - j) as usize] += 1;
            hit[(7 as i32 - j) as
                usize][(7 as i32 - i) as usize] += 1;
            j += 1
        }
        i += 1
    }
    i = 0;
    while i < 8 as i32 {
        hit[i as usize][i as usize] += 1;
        hit[i as usize][(7 as i32 - i) as usize] += 1;
        i += 1
    }
    i = 0;
    while i < 7 as i32 {
        hit[i as usize][(i + 1 as i32) as usize] += 1;
        hit[(i + 1 as i32) as usize][i as usize] += 1;
        hit[i as usize][(6 as i32 - i) as usize] += 1;
        hit[(i + 1 as i32) as usize][(7 as i32 - i) as usize]
            += 1;
        i += 1
    }
    i = 0;
    while i < 6 as i32 {
        hit[i as usize][(i + 2 as i32) as usize] += 1;
        hit[(i + 2 as i32) as usize][i as usize] += 1;
        hit[i as usize][(5 as i32 - i) as usize] += 1;
        hit[(i + 2 as i32) as usize][(7 as i32 - i) as usize]
            += 1;
        i += 1
    }
    i = 0;
    while i < 5 as i32 {
        hit[i as usize][(i + 3 as i32) as usize] += 1;
        hit[(i + 3 as i32) as usize][i as usize] += 1;
        hit[i as usize][(4 as i32 - i) as usize] += 1;
        hit[(i + 3 as i32) as usize][(7 as i32 - i) as usize]
            += 1;
        i += 1
    }
    i = 0;
    while i < 4 as i32 {
        hit[i as usize][(i + 4 as i32) as usize] += 1;
        hit[(i + 4 as i32) as usize][i as usize] += 1;
        hit[i as usize][(3 as i32 - i) as usize] += 1;
        hit[(i + 4 as i32) as usize][(7 as i32 - i) as usize]
            += 1;
        i += 1
    }
    hit[1][1] += 2 as i32;
    hit[1][6] += 2 as i32;
    hit[6][1] += 2 as i32;
    hit[6][6] += 2 as i32;
    hit
}