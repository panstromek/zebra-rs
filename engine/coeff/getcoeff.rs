
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoeffSet {
    pub permanent: i32,
    pub loaded: i32,
    pub prev: i32,
    pub next: i32,
    pub block: i32,
    pub parity_constant: [i16; 2],
    pub parity: i16,
    pub constant: i16,
    pub afile2x: *mut i16,
    pub bfile: *mut i16,
    pub cfile: *mut i16,
    pub dfile: *mut i16,
    pub diag8: *mut i16,
    pub diag7: *mut i16,
    pub diag6: *mut i16,
    pub diag5: *mut i16,
    pub diag4: *mut i16,
    pub corner33: *mut i16,
    pub corner52: *mut i16,
    pub afile2x_last: *mut i16,
    pub bfile_last: *mut i16,
    pub cfile_last: *mut i16,
    pub dfile_last: *mut i16,
    pub diag8_last: *mut i16,
    pub diag7_last: *mut i16,
    pub diag6_last: *mut i16,
    pub diag5_last: *mut i16,
    pub diag4_last: *mut i16,
    pub corner33_last: *mut i16,
    pub corner52_last: *mut i16,
    pub alignment_padding: [i8; 12],
}

pub unsafe fn constant_and_parity_feature(side_to_move: i32, eval_phase: i32, disks_played: i32,
                                          board: &mut [i32; 128], set: &mut [CoeffSet; 61]) -> i32 {
    /* The constant feature and the parity feature */
    let mut score =
        set[eval_phase as
            usize].parity_constant[(disks_played & 1 as i32) as
            usize];
    /* The pattern features. */
    if side_to_move == 0 as i32 {
        let mut pattern0: i32;
        pattern0 = board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[81 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[71 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[61 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[51 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[41 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[31 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[21 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].afile2x.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[88 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[78 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[68 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[58 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[48 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[38 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[28 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].afile2x.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[18 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[17 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[16 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[15 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[14 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[13 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[12 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].afile2x.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[88 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[87 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[86 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[85 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[84 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[83 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[82 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].afile2x.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[82 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[62 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[52 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[42 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[32 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[12 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].bfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[87 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[67 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[57 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[47 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[37 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[17 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].bfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[28 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[26 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[25 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[24 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[23 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[21 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].bfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[78 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[76 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[75 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[74 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[73 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[71 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].bfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[83 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[73 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[63 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[53 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[43 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[33 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[23 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[13 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].cfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[86 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[76 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[66 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[56 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[46 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[36 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[26 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[16 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].cfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[38 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[37 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[36 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[35 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[34 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[33 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[32 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[31 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].cfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[68 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[67 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[66 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[65 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[64 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[63 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[62 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[61 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].cfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[84 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[74 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[64 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[54 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[44 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[34 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[24 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[14 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].dfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[85 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[75 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[65 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[55 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[45 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[35 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[25 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[15 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].dfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[48 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[47 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[46 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[45 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[44 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[43 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[42 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[41 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].dfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[58 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[57 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[56 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[55 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[54 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[53 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[52 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[51 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].dfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[88 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[66 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[55 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[44 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[33 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag8.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[81 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[63 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[54 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[45 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[36 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag8.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[78 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[67 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[56 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[45 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[34 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[23 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[12 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag7.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[87 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[76 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[65 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[54 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[43 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[32 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[21 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag7.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[71 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[62 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[53 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[44 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[35 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[26 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[17 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag7.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[82 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[73 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[64 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[55 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[46 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[37 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[28 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag7.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[68 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[57 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[46 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[35 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[24 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[13 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag6.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[86 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[75 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[64 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[53 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[42 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[31 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag6.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[61 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[52 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[43 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[34 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[25 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[16 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag6.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[83 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[74 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[65 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[56 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[47 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[38 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag6.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[58 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[47 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[36 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[25 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[14 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag5.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[85 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[74 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[63 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[52 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[41 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag5.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[51 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[42 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[33 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[24 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[15 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag5.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[84 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[75 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[66 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[57 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[48 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag5.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[48 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[37 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[26 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[15 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag4.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[84 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[73 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[62 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[51 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag4.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[41 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[32 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[23 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[14 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag4.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[85 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[76 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[67 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[58 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].diag4.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[33 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[32 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[31 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[23 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[21 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[13 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[12 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner33.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[63 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[62 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[61 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[73 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[71 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[83 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[82 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner33.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[36 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[37 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[38 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[26 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[28 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[16 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[17 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner33.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[66 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[67 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[68 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[76 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[78 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[86 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[87 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[88 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner33.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[25 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[24 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[23 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[21 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[15 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[14 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[13 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[12 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[75 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[74 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[73 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[71 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[85 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[84 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[83 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[82 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[24 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[25 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[26 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[28 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[14 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[15 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[16 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[17 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[74 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[75 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[76 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[78 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[84 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[85 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[86 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[87 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[88 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[52 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[42 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[32 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[22 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[12 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[51 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[41 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[31 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[21 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[57 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[47 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[37 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[27 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[17 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[58 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[48 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[38 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[28 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[42 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[52 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[62 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[72 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[82 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[41 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[51 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[61 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[71 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[47 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[57 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[67 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[77 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[87 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[48 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[58 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[68 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[78 as i32 as usize];
        pattern0 =
            3 as i32 * pattern0 + board[88 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as usize].corner52.offset(pattern0 as isize)
                    as i32) as i16
    } else {
        let mut pattern0_0: i32;
        pattern0_0 = board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[81 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[71 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[61 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[51 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[41 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[31 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[21 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].afile2x_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[88 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[78 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[68 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[58 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[48 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[38 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[28 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].afile2x_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[18 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[17 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[16 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[15 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[14 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[13 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[12 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].afile2x_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[88 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[87 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[86 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[85 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[84 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[83 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[82 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].afile2x_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[82 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[62 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[52 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[42 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[32 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[12 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].bfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[87 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[67 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[57 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[47 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[37 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[17 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].bfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[28 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[26 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[25 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[24 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[23 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[21 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].bfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[78 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[76 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[75 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[74 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[73 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[71 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].bfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[83 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[73 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[63 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[53 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[43 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[33 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[23 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[13 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].cfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[86 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[76 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[66 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[56 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[46 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[36 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[26 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[16 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].cfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[38 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[37 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[36 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[35 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[34 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[33 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[32 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[31 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].cfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[68 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[67 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[66 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[65 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[64 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[63 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[62 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[61 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].cfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[84 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[74 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[64 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[54 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[44 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[34 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[24 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[14 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].dfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[85 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[75 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[65 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[55 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[45 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[35 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[25 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[15 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].dfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[48 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[47 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[46 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[45 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[44 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[43 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[42 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[41 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].dfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[58 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[57 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[56 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[55 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[54 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[53 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[52 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[51 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].dfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[88 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[66 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[55 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[44 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[33 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag8_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[81 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[63 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[54 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[45 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[36 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag8_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[78 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[67 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[56 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[45 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[34 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[23 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[12 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag7_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[87 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[76 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[65 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[54 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[43 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[32 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[21 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag7_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[71 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[62 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[53 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[44 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[35 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[26 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[17 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag7_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[82 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[73 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[64 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[55 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[46 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[37 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[28 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag7_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[68 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[57 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[46 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[35 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[24 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[13 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag6_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[86 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[75 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[64 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[53 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[42 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[31 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag6_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[61 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[52 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[43 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[34 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[25 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[16 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag6_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[83 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[74 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[65 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[56 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[47 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[38 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag6_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[58 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[47 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[36 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[25 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[14 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag5_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[85 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[74 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[63 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[52 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[41 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag5_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[51 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[42 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[33 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[24 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[15 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag5_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[84 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[75 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[66 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[57 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[48 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag5_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[48 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[37 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[26 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[15 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag4_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[84 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[73 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[62 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[51 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag4_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[41 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[32 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[23 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[14 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag4_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[85 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[76 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[67 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[58 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].diag4_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[33 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[32 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[31 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[23 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[21 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[13 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[12 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner33_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[63 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[62 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[61 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[73 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[71 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[83 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[82 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner33_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[36 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[37 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[38 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[26 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[28 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[16 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[17 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner33_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[66 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[67 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[68 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[76 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[78 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[86 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[87 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[88 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner33_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[25 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[24 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[23 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[21 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[15 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[14 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[13 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[12 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[75 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[74 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[73 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[71 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[85 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[84 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[83 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[82 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[24 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[25 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[26 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[28 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[14 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[15 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[16 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[17 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[74 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[75 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[76 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[78 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[84 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[85 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[86 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[87 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[88 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[52 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[42 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[32 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[22 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[12 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[51 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[41 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[31 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[21 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[11 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[57 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[47 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[37 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[27 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[17 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[58 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[48 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[38 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[28 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[18 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[42 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[52 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[62 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[72 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[82 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[41 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[51 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[61 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[71 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[81 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[47 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[57 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[67 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[77 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[87 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[48 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[58 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[68 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[78 as i32 as usize];
        pattern0_0 =
            3 as i32 * pattern0_0 + board[88 as i32 as usize];
        score =
            (score as i32 +
                *set[eval_phase as
                    usize].corner52_last.offset(-pattern0_0 as isize) as
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
pub unsafe fn terminal_patterns(set: &mut [CoeffSet; 61]) {
    let mut result: f64 = 0.;
    let mut value: [[f64; 8]; 8] = [[0.; 8]; 8];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut row: [i32; 10] = [0; 10];
    let mut hit: [[i32; 8]; 8] = [[0; 8]; 8];
    /* Count the number of times each square is counted */
    i = 0 as i32;
    while i < 8 as i32 {
        j = 0 as i32;
        while j < 8 as i32 {
            hit[i as usize][j as usize] = 0 as i32;
            j += 1
        }
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        hit[0 as i32 as usize][i as usize] += 1;
        hit[i as usize][0 as i32 as usize] += 1;
        hit[7 as i32 as usize][i as usize] += 1;
        hit[i as usize][7 as i32 as usize] += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        hit[1 as i32 as usize][i as usize] += 1;
        hit[i as usize][1 as i32 as usize] += 1;
        hit[6 as i32 as usize][i as usize] += 1;
        hit[i as usize][6 as i32 as usize] += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        hit[2 as i32 as usize][i as usize] += 1;
        hit[i as usize][2 as i32 as usize] += 1;
        hit[5 as i32 as usize][i as usize] += 1;
        hit[i as usize][5 as i32 as usize] += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        hit[3 as i32 as usize][i as usize] += 1;
        hit[i as usize][3 as i32 as usize] += 1;
        hit[4 as i32 as usize][i as usize] += 1;
        hit[i as usize][4 as i32 as usize] += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 3 as i32 {
        j = 0 as i32;
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
    i = 0 as i32;
    while i < 2 as i32 {
        j = 0 as i32;
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
    i = 0 as i32;
    while i < 8 as i32 {
        hit[i as usize][i as usize] += 1;
        hit[i as usize][(7 as i32 - i) as usize] += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 7 as i32 {
        hit[i as usize][(i + 1 as i32) as usize] += 1;
        hit[(i + 1 as i32) as usize][i as usize] += 1;
        hit[i as usize][(6 as i32 - i) as usize] += 1;
        hit[(i + 1 as i32) as usize][(7 as i32 - i) as usize]
            += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 6 as i32 {
        hit[i as usize][(i + 2 as i32) as usize] += 1;
        hit[(i + 2 as i32) as usize][i as usize] += 1;
        hit[i as usize][(5 as i32 - i) as usize] += 1;
        hit[(i + 2 as i32) as usize][(7 as i32 - i) as usize]
            += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 5 as i32 {
        hit[i as usize][(i + 3 as i32) as usize] += 1;
        hit[(i + 3 as i32) as usize][i as usize] += 1;
        hit[i as usize][(4 as i32 - i) as usize] += 1;
        hit[(i + 3 as i32) as usize][(7 as i32 - i) as usize]
            += 1;
        i += 1
    }
    i = 0 as i32;
    while i < 4 as i32 {
        hit[i as usize][(i + 4 as i32) as usize] += 1;
        hit[(i + 4 as i32) as usize][i as usize] += 1;
        hit[i as usize][(3 as i32 - i) as usize] += 1;
        hit[(i + 4 as i32) as usize][(7 as i32 - i) as usize]
            += 1;
        i += 1
    }
    hit[1 as i32 as usize][1 as i32 as usize] +=
        2 as i32;
    hit[1 as i32 as usize][6 as i32 as usize] +=
        2 as i32;
    hit[6 as i32 as usize][1 as i32 as usize] +=
        2 as i32;
    hit[6 as i32 as usize][6 as i32 as usize] +=
        2 as i32;
    i = 0 as i32;
    while i < 8 as i32 {
        j = 0 as i32;
        while j < 8 as i32 {
            value[i as usize][j as usize] =
                1.0f64 / hit[i as usize][j as usize] as f64;
            j += 1
        }
        i += 1
    }
    i = 0 as i32;
    while i < 10 as i32 { row[i as usize] = 0 as i32; i += 1 }
    i = 0 as i32;
    while i < 59049 as i32 {
        result = 0.0f64;
        j = 0 as i32;
        while j < 8 as i32 {
            if row[j as usize] == 0 as i32 {
                result += value[0 as i32 as usize][j as usize]
            } else if row[j as usize] == 2 as i32 {
                result -= value[0 as i32 as usize][j as usize]
            }
            j += 1
        }
        if row[8 as i32 as usize] == 0 as i32 {
            result +=
                value[1 as i32 as usize][1 as i32 as usize]
        } else if row[8 as i32 as usize] == 2 as i32 {
            result -=
                value[1 as i32 as usize][1 as i32 as usize]
        }
        if row[9 as i32 as usize] == 0 as i32 {
            result +=
                value[1 as i32 as usize][6 as i32 as usize]
        } else if row[9 as i32 as usize] == 2 as i32 {
            result -=
                value[1 as i32 as usize][6 as i32 as usize]
        }
        *set[60 as i32 as usize].afile2x.offset(i as isize) =
            floor(result * 128.0f64 + 0.5f64) as i16;
        result = 0.0f64;
        j = 0 as i32;
        while j < 5 as i32 {
            k = 0 as i32;
            while k < 2 as i32 {
                if row[(5 as i32 * k + j) as usize] ==
                    0 as i32 {
                    result += value[j as usize][k as usize]
                } else if row[(5 as i32 * k + j) as usize] ==
                    2 as i32 {
                    result -= value[j as usize][k as usize]
                }
                k += 1
            }
            j += 1
        }
        *set[60 as i32 as usize].corner52.offset(i as isize) =
            floor(result * 128.0f64 + 0.5f64) as i16;
        if i < 19683 as i32 {
            result = 0.0f64;
            j = 0 as i32;
            while j < 3 as i32 {
                k = 0 as i32;
                while k < 3 as i32 {
                    if row[(3 as i32 * j + k) as usize] ==
                        0 as i32 {
                        result += value[j as usize][k as usize]
                    } else if row[(3 as i32 * j + k) as usize] ==
                        2 as i32 {
                        result -= value[j as usize][k as usize]
                    }
                    k += 1
                }
                j += 1
            }
            *set[60 as i32 as usize].corner33.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 6561 as i32 {
            result = 0.0f64;
            j = 0 as i32;
            while j < 8 as i32 {
                if row[j as usize] == 0 as i32 {
                    result += value[1 as i32 as usize][j as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -= value[1 as i32 as usize][j as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].bfile.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16;
            result = 0.0f64;
            j = 0 as i32;
            while j < 8 as i32 {
                if row[j as usize] == 0 as i32 {
                    result += value[2 as i32 as usize][j as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -= value[2 as i32 as usize][j as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].cfile.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16;
            result = 0.0f64;
            j = 0 as i32;
            while j < 8 as i32 {
                if row[j as usize] == 0 as i32 {
                    result += value[3 as i32 as usize][j as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -= value[3 as i32 as usize][j as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].dfile.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16;
            result = 0.0f64;
            j = 0 as i32;
            while j < 8 as i32 {
                if row[j as usize] == 0 as i32 {
                    result += value[j as usize][j as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -= value[j as usize][j as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].diag8.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 2187 as i32 {
            result = 0.0f64;
            j = 0 as i32;
            while j < 7 as i32 {
                if row[j as usize] == 0 as i32 {
                    result +=
                        value[j as usize][(j + 1 as i32) as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -=
                        value[j as usize][(j + 1 as i32) as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].diag7.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 729 as i32 {
            result = 0.0f64;
            j = 0 as i32;
            while j < 6 as i32 {
                if row[j as usize] == 0 as i32 {
                    result +=
                        value[j as usize][(j + 2 as i32) as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -=
                        value[j as usize][(j + 2 as i32) as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].diag6.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 243 as i32 {
            result = 0.0f64;
            j = 0 as i32;
            while j < 5 as i32 {
                if row[j as usize] == 0 as i32 {
                    result +=
                        value[j as usize][(j + 3 as i32) as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -=
                        value[j as usize][(j + 3 as i32) as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].diag5.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16
        }
        if i < 81 as i32 {
            result = 0.0f64;
            j = 0 as i32;
            while j < 4 as i32 {
                if row[j as usize] == 0 as i32 {
                    result +=
                        value[j as usize][(j + 4 as i32) as usize]
                } else if row[j as usize] == 2 as i32 {
                    result -=
                        value[j as usize][(j + 4 as i32) as usize]
                }
                j += 1
            }
            *set[60 as i32 as usize].diag4.offset(i as isize) =
                floor(result * 128.0f64 + 0.5f64) as i16
        }
        /* Next configuration */
        j = 0 as i32;
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