
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

pub unsafe fn constant_and_parity_feature(side_to_move: i32, disks_played: i32,
                                          board: &mut [i32; 128], set: &CoeffSet) -> i32 {
    /* The constant feature and the parity feature */
    let mut score = set.parity_constant[(disks_played & 1 as i32) as usize];
    /* The pattern features. */
    if side_to_move == 0 as i32 {
        let mut pattern0: i32;
        pattern0 = board[72];
        pattern0 = 3 * pattern0 + board[22];
        pattern0 = 3 * pattern0 + board[81];
        pattern0 = 3 * pattern0 + board[71];
        pattern0 = 3 * pattern0 + board[61];
        pattern0 = 3 * pattern0 + board[51];
        pattern0 = 3 * pattern0 + board[41];
        pattern0 = 3 * pattern0 + board[31];
        pattern0 = 3 * pattern0 + board[21];
        pattern0 = 3 * pattern0 + board[11];
        score =(score as i32 +
                *set.afile2x.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[77];
        pattern0 = 3 * pattern0 + board[27];
        pattern0 = 3 * pattern0 + board[88];
        pattern0 = 3 * pattern0 + board[78];
        pattern0 = 3 * pattern0 + board[68];
        pattern0 = 3 * pattern0 + board[58];
        pattern0 = 3 * pattern0 + board[48];
        pattern0 = 3 * pattern0 + board[38];
        pattern0 = 3 * pattern0 + board[28];
        pattern0 = 3 * pattern0 + board[18];
        score =(score as i32 +
                *set.afile2x.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[27];
        pattern0 = 3 * pattern0 + board[22];
        pattern0 = 3 * pattern0 + board[18];
        pattern0 = 3 * pattern0 + board[17];
        pattern0 = 3 * pattern0 + board[16];
        pattern0 = 3 * pattern0 + board[15];
        pattern0 = 3 * pattern0 + board[14];
        pattern0 = 3 * pattern0 + board[13];
        pattern0 = 3 * pattern0 + board[12];
        pattern0 = 3 * pattern0 + board[11];
        score =(score as i32 +
                *set.afile2x.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[77];
        pattern0 = 3 * pattern0 + board[72];
        pattern0 = 3 * pattern0 + board[88];
        pattern0 = 3 * pattern0 + board[87];
        pattern0 = 3 * pattern0 + board[86];
        pattern0 = 3 * pattern0 + board[85];
        pattern0 = 3 * pattern0 + board[84];
        pattern0 = 3 * pattern0 + board[83];
        pattern0 = 3 * pattern0 + board[82];
        pattern0 = 3 * pattern0 + board[81];
        score =(score as i32 +
                *set.afile2x.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[82];
        pattern0 = 3 * pattern0 + board[72];
        pattern0 = 3 * pattern0 + board[62];
        pattern0 = 3 * pattern0 + board[52];
        pattern0 = 3 * pattern0 + board[42];
        pattern0 = 3 * pattern0 + board[32];
        pattern0 = 3 * pattern0 + board[22];
        pattern0 = 3 * pattern0 + board[12];
        score =(score as i32 +
                *set.bfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[87];
        pattern0 = 3 * pattern0 + board[77];
        pattern0 = 3 * pattern0 + board[67];
        pattern0 = 3 * pattern0 + board[57];
        pattern0 = 3 * pattern0 + board[47];
        pattern0 = 3 * pattern0 + board[37];
        pattern0 = 3 * pattern0 + board[27];
        pattern0 = 3 * pattern0 + board[17];
        score =(score as i32 +
                *set.bfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[28];
        pattern0 = 3 * pattern0 + board[27];
        pattern0 = 3 * pattern0 + board[26];
        pattern0 = 3 * pattern0 + board[25];
        pattern0 = 3 * pattern0 + board[24];
        pattern0 = 3 * pattern0 + board[23];
        pattern0 = 3 * pattern0 + board[22];
        pattern0 = 3 * pattern0 + board[21];
        score =(score as i32 +
                *set.bfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[78];
        pattern0 = 3 * pattern0 + board[77];
        pattern0 = 3 * pattern0 + board[76];
        pattern0 = 3 * pattern0 + board[75];
        pattern0 = 3 * pattern0 + board[74];
        pattern0 = 3 * pattern0 + board[73];
        pattern0 = 3 * pattern0 + board[72];
        pattern0 = 3 * pattern0 + board[71];
        score =(score as i32 +
                *set.bfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[83];
        pattern0 = 3 * pattern0 + board[73];
        pattern0 = 3 * pattern0 + board[63];
        pattern0 = 3 * pattern0 + board[53];
        pattern0 = 3 * pattern0 + board[43];
        pattern0 = 3 * pattern0 + board[33];
        pattern0 = 3 * pattern0 + board[23];
        pattern0 = 3 * pattern0 + board[13];
        score =(score as i32 +
                *set.cfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[86];
        pattern0 = 3 * pattern0 + board[76];
        pattern0 = 3 * pattern0 + board[66];
        pattern0 = 3 * pattern0 + board[56];
        pattern0 = 3 * pattern0 + board[46];
        pattern0 = 3 * pattern0 + board[36];
        pattern0 = 3 * pattern0 + board[26];
        pattern0 = 3 * pattern0 + board[16];
        score =(score as i32 +
                *set.cfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[38];
        pattern0 = 3 * pattern0 + board[37];
        pattern0 = 3 * pattern0 + board[36];
        pattern0 = 3 * pattern0 + board[35];
        pattern0 = 3 * pattern0 + board[34];
        pattern0 = 3 * pattern0 + board[33];
        pattern0 = 3 * pattern0 + board[32];
        pattern0 = 3 * pattern0 + board[31];
        score =(score as i32 +
                *set.cfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[68];
        pattern0 = 3 * pattern0 + board[67];
        pattern0 = 3 * pattern0 + board[66];
        pattern0 = 3 * pattern0 + board[65];
        pattern0 = 3 * pattern0 + board[64];
        pattern0 = 3 * pattern0 + board[63];
        pattern0 = 3 * pattern0 + board[62];
        pattern0 = 3 * pattern0 + board[61];
        score =(score as i32 +
                *set.cfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[84];
        pattern0 = 3 * pattern0 + board[74];
        pattern0 = 3 * pattern0 + board[64];
        pattern0 = 3 * pattern0 + board[54];
        pattern0 = 3 * pattern0 + board[44];
        pattern0 = 3 * pattern0 + board[34];
        pattern0 = 3 * pattern0 + board[24];
        pattern0 = 3 * pattern0 + board[14];
        score =(score as i32 +
                *set.dfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[85];
        pattern0 = 3 * pattern0 + board[75];
        pattern0 = 3 * pattern0 + board[65];
        pattern0 = 3 * pattern0 + board[55];
        pattern0 = 3 * pattern0 + board[45];
        pattern0 = 3 * pattern0 + board[35];
        pattern0 = 3 * pattern0 + board[25];
        pattern0 = 3 * pattern0 + board[15];
        score =(score as i32 +
                *set.dfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[48];
        pattern0 = 3 * pattern0 + board[47];
        pattern0 = 3 * pattern0 + board[46];
        pattern0 = 3 * pattern0 + board[45];
        pattern0 = 3 * pattern0 + board[44];
        pattern0 = 3 * pattern0 + board[43];
        pattern0 = 3 * pattern0 + board[42];
        pattern0 = 3 * pattern0 + board[41];
        score =(score as i32 +
                *set.dfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[58];
        pattern0 = 3 * pattern0 + board[57];
        pattern0 = 3 * pattern0 + board[56];
        pattern0 = 3 * pattern0 + board[55];
        pattern0 = 3 * pattern0 + board[54];
        pattern0 = 3 * pattern0 + board[53];
        pattern0 = 3 * pattern0 + board[52];
        pattern0 = 3 * pattern0 + board[51];
        score =(score as i32 +
                *set.dfile.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[88];
        pattern0 = 3 * pattern0 + board[77];
        pattern0 = 3 * pattern0 + board[66];
        pattern0 = 3 * pattern0 + board[55];
        pattern0 = 3 * pattern0 + board[44];
        pattern0 = 3 * pattern0 + board[33];
        pattern0 = 3 * pattern0 + board[22];
        pattern0 = 3 * pattern0 + board[11];
        score =(score as i32 +
                *set.diag8.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[81];
        pattern0 = 3 * pattern0 + board[72];
        pattern0 = 3 * pattern0 + board[63];
        pattern0 = 3 * pattern0 + board[54];
        pattern0 = 3 * pattern0 + board[45];
        pattern0 = 3 * pattern0 + board[36];
        pattern0 = 3 * pattern0 + board[27];
        pattern0 = 3 * pattern0 + board[18];
        score =(score as i32 +
                *set.diag8.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[78];
        pattern0 = 3 * pattern0 + board[67];
        pattern0 = 3 * pattern0 + board[56];
        pattern0 = 3 * pattern0 + board[45];
        pattern0 = 3 * pattern0 + board[34];
        pattern0 = 3 * pattern0 + board[23];
        pattern0 = 3 * pattern0 + board[12];
        score =(score as i32 +
                *set.diag7.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[87];
        pattern0 = 3 * pattern0 + board[76];
        pattern0 = 3 * pattern0 + board[65];
        pattern0 = 3 * pattern0 + board[54];
        pattern0 = 3 * pattern0 + board[43];
        pattern0 = 3 * pattern0 + board[32];
        pattern0 = 3 * pattern0 + board[21];
        score =(score as i32 +
                *set.diag7.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[71];
        pattern0 = 3 * pattern0 + board[62];
        pattern0 = 3 * pattern0 + board[53];
        pattern0 = 3 * pattern0 + board[44];
        pattern0 = 3 * pattern0 + board[35];
        pattern0 = 3 * pattern0 + board[26];
        pattern0 = 3 * pattern0 + board[17];
        score =(score as i32 +
                *set.diag7.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[82];
        pattern0 = 3 * pattern0 + board[73];
        pattern0 = 3 * pattern0 + board[64];
        pattern0 = 3 * pattern0 + board[55];
        pattern0 = 3 * pattern0 + board[46];
        pattern0 = 3 * pattern0 + board[37];
        pattern0 = 3 * pattern0 + board[28];
        score =(score as i32 +
                *set.diag7.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[68];
        pattern0 = 3 * pattern0 + board[57];
        pattern0 = 3 * pattern0 + board[46];
        pattern0 = 3 * pattern0 + board[35];
        pattern0 = 3 * pattern0 + board[24];
        pattern0 = 3 * pattern0 + board[13];
        score =(score as i32 +
                *set.diag6.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[86];
        pattern0 = 3 * pattern0 + board[75];
        pattern0 = 3 * pattern0 + board[64];
        pattern0 = 3 * pattern0 + board[53];
        pattern0 = 3 * pattern0 + board[42];
        pattern0 = 3 * pattern0 + board[31];
        score =(score as i32 +
                *set.diag6.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[61];
        pattern0 = 3 * pattern0 + board[52];
        pattern0 = 3 * pattern0 + board[43];
        pattern0 = 3 * pattern0 + board[34];
        pattern0 = 3 * pattern0 + board[25];
        pattern0 = 3 * pattern0 + board[16];
        score =(score as i32 +
                *set.diag6.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[83];
        pattern0 = 3 * pattern0 + board[74];
        pattern0 = 3 * pattern0 + board[65];
        pattern0 = 3 * pattern0 + board[56];
        pattern0 = 3 * pattern0 + board[47];
        pattern0 = 3 * pattern0 + board[38];
        score =(score as i32 +
                *set.diag6.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[58];
        pattern0 = 3 * pattern0 + board[47];
        pattern0 = 3 * pattern0 + board[36];
        pattern0 = 3 * pattern0 + board[25];
        pattern0 = 3 * pattern0 + board[14];
        score =(score as i32 +
                *set.diag5.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[85];
        pattern0 = 3 * pattern0 + board[74];
        pattern0 = 3 * pattern0 + board[63];
        pattern0 = 3 * pattern0 + board[52];
        pattern0 = 3 * pattern0 + board[41];
        score =(score as i32 +
                *set.diag5.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[51];
        pattern0 = 3 * pattern0 + board[42];
        pattern0 = 3 * pattern0 + board[33];
        pattern0 = 3 * pattern0 + board[24];
        pattern0 = 3 * pattern0 + board[15];
        score =(score as i32 +
                *set.diag5.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[84];
        pattern0 = 3 * pattern0 + board[75];
        pattern0 = 3 * pattern0 + board[66];
        pattern0 = 3 * pattern0 + board[57];
        pattern0 = 3 * pattern0 + board[48];
        score =(score as i32 +
                *set.diag5.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[48];
        pattern0 = 3 * pattern0 + board[37];
        pattern0 = 3 * pattern0 + board[26];
        pattern0 = 3 * pattern0 + board[15];
        score =(score as i32 +
                *set.diag4.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[84];
        pattern0 = 3 * pattern0 + board[73];
        pattern0 = 3 * pattern0 + board[62];
        pattern0 = 3 * pattern0 + board[51];
        score =(score as i32 +
                *set.diag4.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[41];
        pattern0 = 3 * pattern0 + board[32];
        pattern0 = 3 * pattern0 + board[23];
        pattern0 = 3 * pattern0 + board[14];
        score =(score as i32 +
                *set.diag4.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[85];
        pattern0 = 3 * pattern0 + board[76];
        pattern0 = 3 * pattern0 + board[67];
        pattern0 = 3 * pattern0 + board[58];
        score =(score as i32 +
                *set.diag4.offset(pattern0 as isize) as
                    i32) as i16;
        pattern0 = board[33];
        pattern0 = 3 * pattern0 + board[32];
        pattern0 = 3 * pattern0 + board[31];
        pattern0 = 3 * pattern0 + board[23];
        pattern0 = 3 * pattern0 + board[22];
        pattern0 = 3 * pattern0 + board[21];
        pattern0 = 3 * pattern0 + board[13];
        pattern0 = 3 * pattern0 + board[12];
        pattern0 = 3 * pattern0 + board[11];
        score =(score as i32 +
                *set.corner33.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[63];
        pattern0 = 3 * pattern0 + board[62];
        pattern0 = 3 * pattern0 + board[61];
        pattern0 = 3 * pattern0 + board[73];
        pattern0 = 3 * pattern0 + board[72];
        pattern0 = 3 * pattern0 + board[71];
        pattern0 = 3 * pattern0 + board[83];
        pattern0 = 3 * pattern0 + board[82];
        pattern0 = 3 * pattern0 + board[81];
        score =(score as i32 +
                *set.corner33.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[36];
        pattern0 = 3 * pattern0 + board[37];
        pattern0 = 3 * pattern0 + board[38];
        pattern0 = 3 * pattern0 + board[26];
        pattern0 = 3 * pattern0 + board[27];
        pattern0 = 3 * pattern0 + board[28];
        pattern0 = 3 * pattern0 + board[16];
        pattern0 = 3 * pattern0 + board[17];
        pattern0 = 3 * pattern0 + board[18];
        score =(score as i32 +
                *set.corner33.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[66];
        pattern0 = 3 * pattern0 + board[67];
        pattern0 = 3 * pattern0 + board[68];
        pattern0 = 3 * pattern0 + board[76];
        pattern0 = 3 * pattern0 + board[77];
        pattern0 = 3 * pattern0 + board[78];
        pattern0 = 3 * pattern0 + board[86];
        pattern0 = 3 * pattern0 + board[87];
        pattern0 = 3 * pattern0 + board[88];
        score =(score as i32 +
                *set.corner33.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[25];
        pattern0 = 3 * pattern0 + board[24];
        pattern0 = 3 * pattern0 + board[23];
        pattern0 = 3 * pattern0 + board[22];
        pattern0 = 3 * pattern0 + board[21];
        pattern0 = 3 * pattern0 + board[15];
        pattern0 = 3 * pattern0 + board[14];
        pattern0 = 3 * pattern0 + board[13];
        pattern0 = 3 * pattern0 + board[12];
        pattern0 = 3 * pattern0 + board[11];
        score =(score as i32 +
                *set.corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[75];
        pattern0 = 3 * pattern0 + board[74];
        pattern0 = 3 * pattern0 + board[73];
        pattern0 = 3 * pattern0 + board[72];
        pattern0 = 3 * pattern0 + board[71];
        pattern0 = 3 * pattern0 + board[85];
        pattern0 = 3 * pattern0 + board[84];
        pattern0 = 3 * pattern0 + board[83];
        pattern0 = 3 * pattern0 + board[82];
        pattern0 = 3 * pattern0 + board[81];
        score =(score as i32 +
                *set.corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[24];
        pattern0 = 3 * pattern0 + board[25];
        pattern0 = 3 * pattern0 + board[26];
        pattern0 = 3 * pattern0 + board[27];
        pattern0 = 3 * pattern0 + board[28];
        pattern0 = 3 * pattern0 + board[14];
        pattern0 = 3 * pattern0 + board[15];
        pattern0 = 3 * pattern0 + board[16];
        pattern0 = 3 * pattern0 + board[17];
        pattern0 = 3 * pattern0 + board[18];
        score =(score as i32 +
                *set.corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[74];
        pattern0 = 3 * pattern0 + board[75];
        pattern0 = 3 * pattern0 + board[76];
        pattern0 = 3 * pattern0 + board[77];
        pattern0 = 3 * pattern0 + board[78];
        pattern0 = 3 * pattern0 + board[84];
        pattern0 = 3 * pattern0 + board[85];
        pattern0 = 3 * pattern0 + board[86];
        pattern0 = 3 * pattern0 + board[87];
        pattern0 = 3 * pattern0 + board[88];
        score =(score as i32 +
                *set.corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[52];
        pattern0 = 3 * pattern0 + board[42];
        pattern0 = 3 * pattern0 + board[32];
        pattern0 = 3 * pattern0 + board[22];
        pattern0 = 3 * pattern0 + board[12];
        pattern0 = 3 * pattern0 + board[51];
        pattern0 = 3 * pattern0 + board[41];
        pattern0 = 3 * pattern0 + board[31];
        pattern0 = 3 * pattern0 + board[21];
        pattern0 = 3 * pattern0 + board[11];
        score =(score as i32 +
                *set.corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[57];
        pattern0 = 3 * pattern0 + board[47];
        pattern0 = 3 * pattern0 + board[37];
        pattern0 = 3 * pattern0 + board[27];
        pattern0 = 3 * pattern0 + board[17];
        pattern0 = 3 * pattern0 + board[58];
        pattern0 = 3 * pattern0 + board[48];
        pattern0 = 3 * pattern0 + board[38];
        pattern0 = 3 * pattern0 + board[28];
        pattern0 = 3 * pattern0 + board[18];
        score =(score as i32 +
                *set.corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[42];
        pattern0 = 3 * pattern0 + board[52];
        pattern0 = 3 * pattern0 + board[62];
        pattern0 = 3 * pattern0 + board[72];
        pattern0 = 3 * pattern0 + board[82];
        pattern0 = 3 * pattern0 + board[41];
        pattern0 = 3 * pattern0 + board[51];
        pattern0 = 3 * pattern0 + board[61];
        pattern0 = 3 * pattern0 + board[71];
        pattern0 = 3 * pattern0 + board[81];
        score =(score as i32 +
                *set.corner52.offset(pattern0 as isize)
                    as i32) as i16;
        pattern0 = board[47];
        pattern0 = 3 * pattern0 + board[57];
        pattern0 = 3 * pattern0 + board[67];
        pattern0 = 3 * pattern0 + board[77];
        pattern0 = 3 * pattern0 + board[87];
        pattern0 = 3 * pattern0 + board[48];
        pattern0 = 3 * pattern0 + board[58];
        pattern0 = 3 * pattern0 + board[68];
        pattern0 = 3 * pattern0 + board[78];
        pattern0 = 3 * pattern0 + board[88];
        score =(score as i32 +
                *set.corner52.offset(pattern0 as isize)
                    as i32) as i16
    } else {
        let mut pattern0_0: i32;
        pattern0_0 = board[72];
        pattern0_0 = 3 as i32 * pattern0_0 + board[22];
        pattern0_0 = 3 as i32 * pattern0_0 + board[81];
        pattern0_0 = 3 as i32 * pattern0_0 + board[71];
        pattern0_0 = 3 as i32 * pattern0_0 + board[61];
        pattern0_0 = 3 as i32 * pattern0_0 + board[51];
        pattern0_0 = 3 as i32 * pattern0_0 + board[41];
        pattern0_0 = 3 as i32 * pattern0_0 + board[31];
        pattern0_0 = 3 as i32 * pattern0_0 + board[21];
        pattern0_0 = 3 as i32 * pattern0_0 + board[11];
        score =(score as i32 +
                *set.afile2x_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[77];
        pattern0_0 = 3 as i32 * pattern0_0 + board[27];
        pattern0_0 = 3 as i32 * pattern0_0 + board[88];
        pattern0_0 = 3 as i32 * pattern0_0 + board[78];
        pattern0_0 = 3 as i32 * pattern0_0 + board[68];
        pattern0_0 = 3 as i32 * pattern0_0 + board[58];
        pattern0_0 = 3 as i32 * pattern0_0 + board[48];
        pattern0_0 = 3 as i32 * pattern0_0 + board[38];
        pattern0_0 = 3 as i32 * pattern0_0 + board[28];
        pattern0_0 = 3 as i32 * pattern0_0 + board[18];
        score =(score as i32 +
                *set.afile2x_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[27];
        pattern0_0 = 3 as i32 * pattern0_0 + board[22];
        pattern0_0 = 3 as i32 * pattern0_0 + board[18];
        pattern0_0 = 3 as i32 * pattern0_0 + board[17];
        pattern0_0 = 3 as i32 * pattern0_0 + board[16];
        pattern0_0 = 3 as i32 * pattern0_0 + board[15];
        pattern0_0 = 3 as i32 * pattern0_0 + board[14];
        pattern0_0 = 3 as i32 * pattern0_0 + board[13];
        pattern0_0 = 3 as i32 * pattern0_0 + board[12];
        pattern0_0 = 3 as i32 * pattern0_0 + board[11];
        score =(score as i32 +
                *set.afile2x_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[77];
        pattern0_0 = 3 as i32 * pattern0_0 + board[72];
        pattern0_0 = 3 as i32 * pattern0_0 + board[88];
        pattern0_0 = 3 as i32 * pattern0_0 + board[87];
        pattern0_0 = 3 as i32 * pattern0_0 + board[86];
        pattern0_0 = 3 as i32 * pattern0_0 + board[85];
        pattern0_0 = 3 as i32 * pattern0_0 + board[84];
        pattern0_0 = 3 as i32 * pattern0_0 + board[83];
        pattern0_0 = 3 as i32 * pattern0_0 + board[82];
        pattern0_0 = 3 as i32 * pattern0_0 + board[81];
        score =(score as i32 +
                *set.afile2x_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[82];
        pattern0_0 = 3 as i32 * pattern0_0 + board[72];
        pattern0_0 = 3 as i32 * pattern0_0 + board[62];
        pattern0_0 = 3 as i32 * pattern0_0 + board[52];
        pattern0_0 = 3 as i32 * pattern0_0 + board[42];
        pattern0_0 = 3 as i32 * pattern0_0 + board[32];
        pattern0_0 = 3 as i32 * pattern0_0 + board[22];
        pattern0_0 = 3 as i32 * pattern0_0 + board[12];
        score =(score as i32 +
                *set.bfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[87];
        pattern0_0 = 3 as i32 * pattern0_0 + board[77];
        pattern0_0 = 3 as i32 * pattern0_0 + board[67];
        pattern0_0 = 3 as i32 * pattern0_0 + board[57];
        pattern0_0 = 3 as i32 * pattern0_0 + board[47];
        pattern0_0 = 3 as i32 * pattern0_0 + board[37];
        pattern0_0 = 3 as i32 * pattern0_0 + board[27];
        pattern0_0 = 3 as i32 * pattern0_0 + board[17];
        score =(score as i32 +
                *set.bfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[28];
        pattern0_0 = 3 as i32 * pattern0_0 + board[27];
        pattern0_0 = 3 as i32 * pattern0_0 + board[26];
        pattern0_0 = 3 as i32 * pattern0_0 + board[25];
        pattern0_0 = 3 as i32 * pattern0_0 + board[24];
        pattern0_0 = 3 as i32 * pattern0_0 + board[23];
        pattern0_0 = 3 as i32 * pattern0_0 + board[22];
        pattern0_0 = 3 as i32 * pattern0_0 + board[21];
        score =(score as i32 +
                *set.bfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[78];
        pattern0_0 = 3 as i32 * pattern0_0 + board[77];
        pattern0_0 = 3 as i32 * pattern0_0 + board[76];
        pattern0_0 = 3 as i32 * pattern0_0 + board[75];
        pattern0_0 = 3 as i32 * pattern0_0 + board[74];
        pattern0_0 = 3 as i32 * pattern0_0 + board[73];
        pattern0_0 = 3 as i32 * pattern0_0 + board[72];
        pattern0_0 = 3 as i32 * pattern0_0 + board[71];
        score =(score as i32 +
                *set.bfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[83];
        pattern0_0 = 3 as i32 * pattern0_0 + board[73];
        pattern0_0 = 3 as i32 * pattern0_0 + board[63];
        pattern0_0 = 3 as i32 * pattern0_0 + board[53];
        pattern0_0 = 3 as i32 * pattern0_0 + board[43];
        pattern0_0 = 3 as i32 * pattern0_0 + board[33];
        pattern0_0 = 3 as i32 * pattern0_0 + board[23];
        pattern0_0 = 3 as i32 * pattern0_0 + board[13];
        score =(score as i32 +
                *set.cfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[86];
        pattern0_0 = 3 as i32 * pattern0_0 + board[76];
        pattern0_0 = 3 as i32 * pattern0_0 + board[66];
        pattern0_0 = 3 as i32 * pattern0_0 + board[56];
        pattern0_0 = 3 as i32 * pattern0_0 + board[46];
        pattern0_0 = 3 as i32 * pattern0_0 + board[36];
        pattern0_0 = 3 as i32 * pattern0_0 + board[26];
        pattern0_0 = 3 as i32 * pattern0_0 + board[16];
        score =(score as i32 +
                *set.cfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[38];
        pattern0_0 = 3 as i32 * pattern0_0 + board[37];
        pattern0_0 = 3 as i32 * pattern0_0 + board[36];
        pattern0_0 = 3 as i32 * pattern0_0 + board[35];
        pattern0_0 = 3 as i32 * pattern0_0 + board[34];
        pattern0_0 = 3 as i32 * pattern0_0 + board[33];
        pattern0_0 = 3 as i32 * pattern0_0 + board[32];
        pattern0_0 = 3 as i32 * pattern0_0 + board[31];
        score =(score as i32 +
                *set.cfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[68];
        pattern0_0 = 3 as i32 * pattern0_0 + board[67];
        pattern0_0 = 3 as i32 * pattern0_0 + board[66];
        pattern0_0 = 3 as i32 * pattern0_0 + board[65];
        pattern0_0 = 3 as i32 * pattern0_0 + board[64];
        pattern0_0 = 3 as i32 * pattern0_0 + board[63];
        pattern0_0 = 3 as i32 * pattern0_0 + board[62];
        pattern0_0 = 3 as i32 * pattern0_0 + board[61];
        score =(score as i32 +
                *set.cfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[84];
        pattern0_0 = 3 as i32 * pattern0_0 + board[74];
        pattern0_0 = 3 as i32 * pattern0_0 + board[64];
        pattern0_0 = 3 as i32 * pattern0_0 + board[54];
        pattern0_0 = 3 as i32 * pattern0_0 + board[44];
        pattern0_0 = 3 as i32 * pattern0_0 + board[34];
        pattern0_0 = 3 as i32 * pattern0_0 + board[24];
        pattern0_0 = 3 as i32 * pattern0_0 + board[14];
        score =(score as i32 +
                *set.dfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[85];
        pattern0_0 = 3 as i32 * pattern0_0 + board[75];
        pattern0_0 = 3 as i32 * pattern0_0 + board[65];
        pattern0_0 = 3 as i32 * pattern0_0 + board[55];
        pattern0_0 = 3 as i32 * pattern0_0 + board[45];
        pattern0_0 = 3 as i32 * pattern0_0 + board[35];
        pattern0_0 = 3 as i32 * pattern0_0 + board[25];
        pattern0_0 = 3 as i32 * pattern0_0 + board[15];
        score =(score as i32 +
                *set.dfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[48];
        pattern0_0 = 3 as i32 * pattern0_0 + board[47];
        pattern0_0 = 3 as i32 * pattern0_0 + board[46];
        pattern0_0 = 3 as i32 * pattern0_0 + board[45];
        pattern0_0 = 3 as i32 * pattern0_0 + board[44];
        pattern0_0 = 3 as i32 * pattern0_0 + board[43];
        pattern0_0 = 3 as i32 * pattern0_0 + board[42];
        pattern0_0 = 3 as i32 * pattern0_0 + board[41];
        score =(score as i32 +
                *set.dfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[58];
        pattern0_0 = 3 as i32 * pattern0_0 + board[57];
        pattern0_0 = 3 as i32 * pattern0_0 + board[56];
        pattern0_0 = 3 as i32 * pattern0_0 + board[55];
        pattern0_0 = 3 as i32 * pattern0_0 + board[54];
        pattern0_0 = 3 as i32 * pattern0_0 + board[53];
        pattern0_0 = 3 as i32 * pattern0_0 + board[52];
        pattern0_0 = 3 as i32 * pattern0_0 + board[51];
        score =(score as i32 +
                *set.dfile_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[88];
        pattern0_0 = 3 as i32 * pattern0_0 + board[77];
        pattern0_0 = 3 as i32 * pattern0_0 + board[66];
        pattern0_0 = 3 as i32 * pattern0_0 + board[55];
        pattern0_0 = 3 as i32 * pattern0_0 + board[44];
        pattern0_0 = 3 as i32 * pattern0_0 + board[33];
        pattern0_0 = 3 as i32 * pattern0_0 + board[22];
        pattern0_0 = 3 as i32 * pattern0_0 + board[11];
        score =(score as i32 +
                *set.diag8_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[81];
        pattern0_0 = 3 as i32 * pattern0_0 + board[72];
        pattern0_0 = 3 as i32 * pattern0_0 + board[63];
        pattern0_0 = 3 as i32 * pattern0_0 + board[54];
        pattern0_0 = 3 as i32 * pattern0_0 + board[45];
        pattern0_0 = 3 as i32 * pattern0_0 + board[36];
        pattern0_0 = 3 as i32 * pattern0_0 + board[27];
        pattern0_0 = 3 as i32 * pattern0_0 + board[18];
        score =(score as i32 +
                *set.diag8_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[78];
        pattern0_0 = 3 as i32 * pattern0_0 + board[67];
        pattern0_0 = 3 as i32 * pattern0_0 + board[56];
        pattern0_0 = 3 as i32 * pattern0_0 + board[45];
        pattern0_0 = 3 as i32 * pattern0_0 + board[34];
        pattern0_0 = 3 as i32 * pattern0_0 + board[23];
        pattern0_0 = 3 as i32 * pattern0_0 + board[12];
        score =(score as i32 +
                *set.diag7_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[87];
        pattern0_0 = 3 as i32 * pattern0_0 + board[76];
        pattern0_0 = 3 as i32 * pattern0_0 + board[65];
        pattern0_0 = 3 as i32 * pattern0_0 + board[54];
        pattern0_0 = 3 as i32 * pattern0_0 + board[43];
        pattern0_0 = 3 as i32 * pattern0_0 + board[32];
        pattern0_0 = 3 as i32 * pattern0_0 + board[21];
        score =(score as i32 +
                *set.diag7_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[71];
        pattern0_0 = 3 as i32 * pattern0_0 + board[62];
        pattern0_0 = 3 as i32 * pattern0_0 + board[53];
        pattern0_0 = 3 as i32 * pattern0_0 + board[44];
        pattern0_0 = 3 as i32 * pattern0_0 + board[35];
        pattern0_0 = 3 as i32 * pattern0_0 + board[26];
        pattern0_0 = 3 as i32 * pattern0_0 + board[17];
        score =(score as i32 +
                *set.diag7_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[82];
        pattern0_0 = 3 as i32 * pattern0_0 + board[73];
        pattern0_0 = 3 as i32 * pattern0_0 + board[64];
        pattern0_0 = 3 as i32 * pattern0_0 + board[55];
        pattern0_0 = 3 as i32 * pattern0_0 + board[46];
        pattern0_0 = 3 as i32 * pattern0_0 + board[37];
        pattern0_0 = 3 as i32 * pattern0_0 + board[28];
        score =(score as i32 +
                *set.diag7_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[68];
        pattern0_0 = 3 as i32 * pattern0_0 + board[57];
        pattern0_0 = 3 as i32 * pattern0_0 + board[46];
        pattern0_0 = 3 as i32 * pattern0_0 + board[35];
        pattern0_0 = 3 as i32 * pattern0_0 + board[24];
        pattern0_0 = 3 as i32 * pattern0_0 + board[13];
        score =(score as i32 +
                *set.diag6_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[86];
        pattern0_0 = 3 as i32 * pattern0_0 + board[75];
        pattern0_0 = 3 as i32 * pattern0_0 + board[64];
        pattern0_0 = 3 as i32 * pattern0_0 + board[53];
        pattern0_0 = 3 as i32 * pattern0_0 + board[42];
        pattern0_0 = 3 as i32 * pattern0_0 + board[31];
        score =(score as i32 +
                *set.diag6_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[61];
        pattern0_0 = 3 as i32 * pattern0_0 + board[52];
        pattern0_0 = 3 as i32 * pattern0_0 + board[43];
        pattern0_0 = 3 as i32 * pattern0_0 + board[34];
        pattern0_0 = 3 as i32 * pattern0_0 + board[25];
        pattern0_0 = 3 as i32 * pattern0_0 + board[16];
        score =(score as i32 +
                *set.diag6_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[83];
        pattern0_0 = 3 as i32 * pattern0_0 + board[74];
        pattern0_0 = 3 as i32 * pattern0_0 + board[65];
        pattern0_0 = 3 as i32 * pattern0_0 + board[56];
        pattern0_0 = 3 as i32 * pattern0_0 + board[47];
        pattern0_0 = 3 as i32 * pattern0_0 + board[38];
        score =(score as i32 +
                *set.diag6_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[58];
        pattern0_0 = 3 as i32 * pattern0_0 + board[47];
        pattern0_0 = 3 as i32 * pattern0_0 + board[36];
        pattern0_0 = 3 as i32 * pattern0_0 + board[25];
        pattern0_0 = 3 as i32 * pattern0_0 + board[14];
        score =(score as i32 +
                *set.diag5_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[85];
        pattern0_0 = 3 as i32 * pattern0_0 + board[74];
        pattern0_0 = 3 as i32 * pattern0_0 + board[63];
        pattern0_0 = 3 as i32 * pattern0_0 + board[52];
        pattern0_0 = 3 as i32 * pattern0_0 + board[41];
        score =(score as i32 +
                *set.diag5_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[51];
        pattern0_0 = 3 as i32 * pattern0_0 + board[42];
        pattern0_0 = 3 as i32 * pattern0_0 + board[33];
        pattern0_0 = 3 as i32 * pattern0_0 + board[24];
        pattern0_0 = 3 as i32 * pattern0_0 + board[15];
        score =(score as i32 +
                *set.diag5_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[84];
        pattern0_0 = 3 as i32 * pattern0_0 + board[75];
        pattern0_0 = 3 as i32 * pattern0_0 + board[66];
        pattern0_0 = 3 as i32 * pattern0_0 + board[57];
        pattern0_0 = 3 as i32 * pattern0_0 + board[48];
        score =(score as i32 +
                *set.diag5_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[48];
        pattern0_0 = 3 as i32 * pattern0_0 + board[37];
        pattern0_0 = 3 as i32 * pattern0_0 + board[26];
        pattern0_0 = 3 as i32 * pattern0_0 + board[15];
        score =(score as i32 +
                *set.diag4_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[84];
        pattern0_0 = 3 as i32 * pattern0_0 + board[73];
        pattern0_0 = 3 as i32 * pattern0_0 + board[62];
        pattern0_0 = 3 as i32 * pattern0_0 + board[51];
        score =(score as i32 +
                *set.diag4_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[41];
        pattern0_0 = 3 as i32 * pattern0_0 + board[32];
        pattern0_0 = 3 as i32 * pattern0_0 + board[23];
        pattern0_0 = 3 as i32 * pattern0_0 + board[14];
        score =(score as i32 +
                *set.diag4_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[85];
        pattern0_0 = 3 as i32 * pattern0_0 + board[76];
        pattern0_0 = 3 as i32 * pattern0_0 + board[67];
        pattern0_0 = 3 as i32 * pattern0_0 + board[58];
        score =(score as i32 +
                *set.diag4_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[33];
        pattern0_0 = 3 as i32 * pattern0_0 + board[32];
        pattern0_0 = 3 as i32 * pattern0_0 + board[31];
        pattern0_0 = 3 as i32 * pattern0_0 + board[23];
        pattern0_0 = 3 as i32 * pattern0_0 + board[22];
        pattern0_0 = 3 as i32 * pattern0_0 + board[21];
        pattern0_0 = 3 as i32 * pattern0_0 + board[13];
        pattern0_0 = 3 as i32 * pattern0_0 + board[12];
        pattern0_0 = 3 as i32 * pattern0_0 + board[11];
        score =(score as i32 +
                *set.corner33_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[63];
        pattern0_0 = 3 as i32 * pattern0_0 + board[62];
        pattern0_0 = 3 as i32 * pattern0_0 + board[61];
        pattern0_0 = 3 as i32 * pattern0_0 + board[73];
        pattern0_0 = 3 as i32 * pattern0_0 + board[72];
        pattern0_0 = 3 as i32 * pattern0_0 + board[71];
        pattern0_0 = 3 as i32 * pattern0_0 + board[83];
        pattern0_0 = 3 as i32 * pattern0_0 + board[82];
        pattern0_0 = 3 as i32 * pattern0_0 + board[81];
        score =(score as i32 +
                *set.corner33_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[36];
        pattern0_0 = 3 as i32 * pattern0_0 + board[37];
        pattern0_0 = 3 as i32 * pattern0_0 + board[38];
        pattern0_0 = 3 as i32 * pattern0_0 + board[26];
        pattern0_0 = 3 as i32 * pattern0_0 + board[27];
        pattern0_0 = 3 as i32 * pattern0_0 + board[28];
        pattern0_0 = 3 as i32 * pattern0_0 + board[16];
        pattern0_0 = 3 as i32 * pattern0_0 + board[17];
        pattern0_0 = 3 as i32 * pattern0_0 + board[18];
        score =(score as i32 +
                *set.corner33_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[66];
        pattern0_0 = 3 as i32 * pattern0_0 + board[67];
        pattern0_0 = 3 as i32 * pattern0_0 + board[68];
        pattern0_0 = 3 as i32 * pattern0_0 + board[76];
        pattern0_0 = 3 as i32 * pattern0_0 + board[77];
        pattern0_0 = 3 as i32 * pattern0_0 + board[78];
        pattern0_0 = 3 as i32 * pattern0_0 + board[86];
        pattern0_0 = 3 as i32 * pattern0_0 + board[87];
        pattern0_0 = 3 as i32 * pattern0_0 + board[88];
        score =(score as i32 +
                *set.corner33_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[25];
        pattern0_0 = 3 as i32 * pattern0_0 + board[24];
        pattern0_0 = 3 as i32 * pattern0_0 + board[23];
        pattern0_0 = 3 as i32 * pattern0_0 + board[22];
        pattern0_0 = 3 as i32 * pattern0_0 + board[21];
        pattern0_0 = 3 as i32 * pattern0_0 + board[15];
        pattern0_0 = 3 as i32 * pattern0_0 + board[14];
        pattern0_0 = 3 as i32 * pattern0_0 + board[13];
        pattern0_0 = 3 as i32 * pattern0_0 + board[12];
        pattern0_0 = 3 as i32 * pattern0_0 + board[11];
        score =(score as i32 +
                *set.corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[75];
        pattern0_0 = 3 as i32 * pattern0_0 + board[74];
        pattern0_0 = 3 as i32 * pattern0_0 + board[73];
        pattern0_0 = 3 as i32 * pattern0_0 + board[72];
        pattern0_0 = 3 as i32 * pattern0_0 + board[71];
        pattern0_0 = 3 as i32 * pattern0_0 + board[85];
        pattern0_0 = 3 as i32 * pattern0_0 + board[84];
        pattern0_0 = 3 as i32 * pattern0_0 + board[83];
        pattern0_0 = 3 as i32 * pattern0_0 + board[82];
        pattern0_0 = 3 as i32 * pattern0_0 + board[81];
        score =(score as i32 +
                *set.corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[24];
        pattern0_0 = 3 as i32 * pattern0_0 + board[25];
        pattern0_0 = 3 as i32 * pattern0_0 + board[26];
        pattern0_0 = 3 as i32 * pattern0_0 + board[27];
        pattern0_0 = 3 as i32 * pattern0_0 + board[28];
        pattern0_0 = 3 as i32 * pattern0_0 + board[14];
        pattern0_0 = 3 as i32 * pattern0_0 + board[15];
        pattern0_0 = 3 as i32 * pattern0_0 + board[16];
        pattern0_0 = 3 as i32 * pattern0_0 + board[17];
        pattern0_0 = 3 as i32 * pattern0_0 + board[18];
        score =(score as i32 +
                *set.corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[74];
        pattern0_0 = 3 as i32 * pattern0_0 + board[75];
        pattern0_0 = 3 as i32 * pattern0_0 + board[76];
        pattern0_0 = 3 as i32 * pattern0_0 + board[77];
        pattern0_0 = 3 as i32 * pattern0_0 + board[78];
        pattern0_0 = 3 as i32 * pattern0_0 + board[84];
        pattern0_0 = 3 as i32 * pattern0_0 + board[85];
        pattern0_0 = 3 as i32 * pattern0_0 + board[86];
        pattern0_0 = 3 as i32 * pattern0_0 + board[87];
        pattern0_0 = 3 as i32 * pattern0_0 + board[88];
        score =(score as i32 +
                *set.corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[52];
        pattern0_0 = 3 as i32 * pattern0_0 + board[42];
        pattern0_0 = 3 as i32 * pattern0_0 + board[32];
        pattern0_0 = 3 as i32 * pattern0_0 + board[22];
        pattern0_0 = 3 as i32 * pattern0_0 + board[12];
        pattern0_0 = 3 as i32 * pattern0_0 + board[51];
        pattern0_0 = 3 as i32 * pattern0_0 + board[41];
        pattern0_0 = 3 as i32 * pattern0_0 + board[31];
        pattern0_0 = 3 as i32 * pattern0_0 + board[21];
        pattern0_0 = 3 as i32 * pattern0_0 + board[11];
        score =(score as i32 +
                *set.corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[57];
        pattern0_0 = 3 as i32 * pattern0_0 + board[47];
        pattern0_0 = 3 as i32 * pattern0_0 + board[37];
        pattern0_0 = 3 as i32 * pattern0_0 + board[27];
        pattern0_0 = 3 as i32 * pattern0_0 + board[17];
        pattern0_0 = 3 as i32 * pattern0_0 + board[58];
        pattern0_0 = 3 as i32 * pattern0_0 + board[48];
        pattern0_0 = 3 as i32 * pattern0_0 + board[38];
        pattern0_0 = 3 as i32 * pattern0_0 + board[28];
        pattern0_0 = 3 as i32 * pattern0_0 + board[18];
        score =(score as i32 +
                *set.corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[42];
        pattern0_0 = 3 as i32 * pattern0_0 + board[52];
        pattern0_0 = 3 as i32 * pattern0_0 + board[62];
        pattern0_0 = 3 as i32 * pattern0_0 + board[72];
        pattern0_0 = 3 as i32 * pattern0_0 + board[82];
        pattern0_0 = 3 as i32 * pattern0_0 + board[41];
        pattern0_0 = 3 as i32 * pattern0_0 + board[51];
        pattern0_0 = 3 as i32 * pattern0_0 + board[61];
        pattern0_0 = 3 as i32 * pattern0_0 + board[71];
        pattern0_0 = 3 as i32 * pattern0_0 + board[81];
        score =(score as i32 +
                *set.corner52_last.offset(-pattern0_0 as isize) as
                    i32) as i16;
        pattern0_0 = board[47];
        pattern0_0 = 3 as i32 * pattern0_0 + board[57];
        pattern0_0 = 3 as i32 * pattern0_0 + board[67];
        pattern0_0 = 3 as i32 * pattern0_0 + board[77];
        pattern0_0 = 3 as i32 * pattern0_0 + board[87];
        pattern0_0 = 3 as i32 * pattern0_0 + board[48];
        pattern0_0 = 3 as i32 * pattern0_0 + board[58];
        pattern0_0 = 3 as i32 * pattern0_0 + board[68];
        pattern0_0 = 3 as i32 * pattern0_0 + board[78];
        pattern0_0 = 3 as i32 * pattern0_0 + board[88];
        score =(score as i32 +
                *set.corner52_last.offset(-pattern0_0 as isize) as
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
    let mut result: f64;
    let mut value: [[f64; 8]; 8] = [[0.; 8]; 8];
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut row: [i32; 10] = [0; 10];
    let mut hit: [[i32; 8]; 8] = [[0; 8]; 8];
    /* Count the number of times each square is counted */
    i = 0;
    while i < 8 as i32 {
        j = 0;
        while j < 8 as i32 {
            hit[i as usize][j as usize] = 0;
            j += 1
        }
        i += 1
    }
    i = 0;
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
    hit[1][1] +=2 as i32;
    hit[1][6] +=2 as i32;
    hit[6][1] +=2 as i32;
    hit[6][6] +=2 as i32;
    i = 0;
    while i < 8 as i32 {
        j = 0;
        while j < 8 as i32 {
            value[i as usize][j as usize] =1.0f64 / hit[i as usize][j as usize] as f64;
            j += 1
        }
        i += 1
    }
    i = 0;
    while i < 10 as i32 { row[i as usize] = 0; i += 1 }
    i = 0;
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
        *set[60].afile2x.offset(i as isize) =floor(result * 128.0f64 + 0.5f64) as i16;
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
        *set[60].corner52.offset(i as isize) =floor(result * 128.0f64 + 0.5f64) as i16;
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
            *set[60].corner33.offset(i as isize) =floor(result * 128.0f64 + 0.5f64) as i16
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
            *set[60].bfile.offset(i as isize) =floor(result * 128.0f64 + 0.5f64) as i16;
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
            *set[60].cfile.offset(i as isize) =floor(result * 128.0f64 + 0.5f64) as i16;
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
            *set[60].dfile.offset(i as isize) =floor(result * 128.0f64 + 0.5f64) as i16;
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
            *set[60].diag8.offset(i as isize) =floor(result * 128.0f64 + 0.5f64) as i16
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
            *set[60].diag7.offset(i as isize) =floor(result * 128.0f64 + 0.5f64) as i16
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
            *set[60].diag6.offset(i as isize) =floor(result * 128.0f64 + 0.5f64) as i16
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
            *set[60].diag5.offset(i as isize) =floor(result * 128.0f64 + 0.5f64) as i16
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
            *set[60].diag4.offset(i as isize) =floor(result * 128.0f64 + 0.5f64) as i16
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