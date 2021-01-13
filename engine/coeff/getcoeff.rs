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
    let mut  score = set.parity_constant[(disks_played & 1 as i32) as usize];
    let set = set.data.as_mut().unwrap();

    // Following assert is an invariant of a board type in this program - it's not yet captured in a type
    // that would enforce it at compile time, but I'll do that eventually
    // assert!(board.iter().all(|&item| ([0, 1, 2, 3].iter().any(|&some| some == item))));

    // We know that board only contains (0,1,2,3) numbers and this function is called at most
    //  10 times in a row, so it can never overflow in here. Using this knowledge, we can use wrapping
    //  functions. Using this helper cuts down the number of generated llvm-ir lines by a huge number
    //  (from 14742 to 5442), which improves build time + runtime doesn't seem to be affected
    /// (tests still run in ~2.5 seconds like before this change)
    fn update_pattern( pat0:i32, board: &[i32; 128], index: u8) -> i32 {
        (3 as i32).wrapping_mul(pat0).wrapping_add(
            board[index as usize]
        )
    }
    fn compute_pattern(board: &[i32; 128], indexes: &[u8]) -> i32 {
        let mut pattern = 0;
        let mut indexes = indexes.iter();
        while let Some(&index) = indexes.next() {
            pattern = update_pattern(pattern, board, index);
        }
        return pattern
    }

    fn update_score(board: &[i32; 128], side_to_move: i32, score: i16, set_item: &[i16], indexes: &[u8]) -> i16 {
        let pattern = compute_pattern(board, indexes);
        let off = if side_to_move == 0 as i32 {
            pattern as isize
        } else {
            (set_item.len() as isize - 1 - pattern as isize) as isize
        };
        let score =(score as i32 + *set_item.offset(off) as i32) as i16;
        return score;
    }

    score = update_score(board, side_to_move, score, set.afile2x(), &[72, 22, 81, 71, 61, 51, 41, 31, 21, 11]);
    score = update_score(board, side_to_move, score, set.afile2x(), &[77, 27, 88, 78, 68, 58, 48, 38, 28, 18]);
    score = update_score(board, side_to_move, score, set.afile2x(), &[27, 22, 18, 17, 16, 15, 14, 13, 12, 11]);
    score = update_score(board, side_to_move, score, set.afile2x(), &[77, 72, 88, 87, 86, 85, 84, 83, 82, 81]);
    score = update_score(board, side_to_move, score, set.bfile(), &[82, 72, 62, 52, 42, 32, 22, 12]);
    score = update_score(board, side_to_move, score, set.bfile(), &[87, 77, 67, 57, 47, 37, 27, 17]);
    score = update_score(board, side_to_move, score, set.bfile(), &[28, 27, 26, 25, 24, 23, 22, 21]);
    score = update_score(board, side_to_move, score, set.bfile(), &[78, 77, 76, 75, 74, 73, 72, 71]);
    score = update_score(board, side_to_move, score, set.cfile(), &[83, 73, 63, 53, 43, 33, 23, 13]);
    score = update_score(board, side_to_move, score, set.cfile(), &[86, 76, 66, 56, 46, 36, 26, 16]);
    score = update_score(board, side_to_move, score, set.cfile(), &[38, 37, 36, 35, 34, 33, 32, 31]);
    score = update_score(board, side_to_move, score, set.cfile(), &[68, 67, 66, 65, 64, 63, 62, 61]);
    score = update_score(board, side_to_move, score, set.dfile(), &[84, 74, 64, 54, 44, 34, 24, 14]);
    score = update_score(board, side_to_move, score, set.dfile(), &[85, 75, 65, 55, 45, 35, 25, 15]);
    score = update_score(board, side_to_move, score, set.dfile(), &[48, 47, 46, 45, 44, 43, 42, 41]);
    score = update_score(board, side_to_move, score, set.dfile(), &[58, 57, 56, 55, 54, 53, 52, 51]);
    score = update_score(board, side_to_move, score, set.diag8(), &[88, 77, 66, 55, 44, 33, 22, 11]);
    score = update_score(board, side_to_move, score, set.diag8(), &[81, 72, 63, 54, 45, 36, 27, 18]);
    score = update_score(board, side_to_move, score, set.diag7(), &[78, 67, 56, 45, 34, 23, 12]);
    score = update_score(board, side_to_move, score, set.diag7(), &[87, 76, 65, 54, 43, 32, 21]);
    score = update_score(board, side_to_move, score, set.diag7(), &[71, 62, 53, 44, 35, 26, 17]);
    score = update_score(board, side_to_move, score, set.diag7(), &[82, 73, 64, 55, 46, 37, 28]);
    score = update_score(board, side_to_move, score, set.diag6(), &[68, 57, 46, 35, 24, 13]);
    score = update_score(board, side_to_move, score, set.diag6(), &[86, 75, 64, 53, 42, 31]);
    score = update_score(board, side_to_move, score, set.diag6(), &[61, 52, 43, 34, 25, 16]);
    score = update_score(board, side_to_move, score, set.diag6(), &[83, 74, 65, 56, 47, 38]);
    score = update_score(board, side_to_move, score, set.diag5(), &[58, 47, 36, 25, 14]);
    score = update_score(board, side_to_move, score, set.diag5(), &[85, 74, 63, 52, 41]);
    score = update_score(board, side_to_move, score, set.diag5(), &[51, 42, 33, 24, 15]);
    score = update_score(board, side_to_move, score, set.diag5(), &[84, 75, 66, 57, 48]);
    score = update_score(board, side_to_move, score, set.diag4(), &[48, 37, 26, 15]);
    score = update_score(board, side_to_move, score, set.diag4(), &[84, 73, 62, 51]);
    score = update_score(board, side_to_move, score, set.diag4(), &[41, 32, 23, 14]);
    score = update_score(board, side_to_move, score, set.diag4(), &[85, 76, 67, 58]);
    score = update_score(board, side_to_move, score, set.corner33(), &[33, 32, 31, 23, 22, 21, 13, 12, 11]);
    score = update_score(board, side_to_move, score, set.corner33(), &[63, 62, 61, 73, 72, 71, 83, 82, 81]);
    score = update_score(board, side_to_move, score, set.corner33(), &[36, 37, 38, 26, 27, 28, 16, 17, 18]);
    score = update_score(board, side_to_move, score, set.corner33(), &[66, 67, 68, 76, 77, 78, 86, 87, 88]);
    score = update_score(board, side_to_move, score, set.corner52(), &[25, 24, 23, 22, 21, 15, 14, 13, 12, 11]);
    score = update_score(board, side_to_move, score, set.corner52(), &[75, 74, 73, 72, 71, 85, 84, 83, 82, 81]);
    score = update_score(board, side_to_move, score, set.corner52(), &[24, 25, 26, 27, 28, 14, 15, 16, 17, 18]);
    score = update_score(board, side_to_move, score, set.corner52(), &[74, 75, 76, 77, 78, 84, 85, 86, 87, 88]);
    score = update_score(board, side_to_move, score, set.corner52(), &[52, 42, 32, 22, 12, 51, 41, 31, 21, 11]);
    score = update_score(board, side_to_move, score, set.corner52(), &[57, 47, 37, 27, 17, 58, 48, 38, 28, 18]);
    score = update_score(board, side_to_move, score, set.corner52(), &[42, 52, 62, 72, 82, 41, 51, 61, 71, 81]);
    score = update_score(board, side_to_move, score, set.corner52(), &[47, 57, 67, 77, 87, 48, 58, 68, 78, 88]);
    return score as i32;
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
    let mut j;
    let mut k;
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
        coeff_set.afile2x_mut()[i as usize] = f64::floor(result * 128.0f64 + 0.5f64) as i16;
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
        coeff_set.corner52_mut()[i as usize] =f64::floor(result * 128.0f64 + 0.5f64) as i16;
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
            coeff_set.corner33_mut()[i as usize] =f64::floor(result * 128.0f64 + 0.5f64) as i16
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
            coeff_set.bfile_mut()[i as usize] = f64::floor(result * 128.0f64 + 0.5f64) as i16;
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
            coeff_set.cfile_mut()[i as usize] = f64::floor(result * 128.0f64 + 0.5f64) as i16;
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
            coeff_set.dfile_mut()[i as usize] =f64::floor(result * 128.0f64 + 0.5f64) as i16;
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
            coeff_set.diag8_mut()[i as usize] =f64::floor(result * 128.0f64 + 0.5f64) as i16
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
            coeff_set.diag7_mut()[i as usize] =f64::floor(result * 128.0f64 + 0.5f64) as i16
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
            coeff_set.diag6_mut()[i as usize] =f64::floor(result * 128.0f64 + 0.5f64) as i16
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
            coeff_set.diag5_mut()[i as usize] =f64::floor(result * 128.0f64 + 0.5f64) as i16
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
            coeff_set.diag4_mut()[i as usize] = f64::floor(result * 128.0f64 + 0.5f64) as i16
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
    // created byt function: create_hit(); commented out because we don't
    // need to run it during each compilation. IF you need to change it,
    // uncomment it and inline instead of the following expression
    static HIT: [[u8; 8]; 8] = [
        [6, 6, 5, 6, 6, 5, 6, 6],
        [6, 8, 6, 6, 6, 6, 8, 6],
        [5, 6, 5, 4, 4, 5, 6, 5],
        [6, 6, 4, 4, 4, 4, 6, 6],
        [6, 6, 4, 4, 4, 4, 6, 6],
        [5, 6, 5, 4, 4, 5, 6, 5],
        [6, 8, 6, 6, 6, 6, 8, 6],
        [6, 6, 5, 6, 6, 5, 6, 6]
    ];

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
    value
}
// computes static HIT - uncomment if you need to recompute it
// const fn create_hit() -> [[i32; 8]; 8] {
//     let mut j: i32;
//     let mut hit: [[i32; 8]; 8] = [[0; 8]; 8];
//     /* Count the number of times each square is counted */
//
//     let mut i = 0;
//     while i < 8 as i32 {
//         hit[0][i as usize] += 1;
//         hit[i as usize][0] += 1;
//         hit[7][i as usize] += 1;
//         hit[i as usize][7] += 1;
//         i += 1
//     }
//     i = 0;
//     while i < 8 as i32 {
//         hit[1][i as usize] += 1;
//         hit[i as usize][1] += 1;
//         hit[6][i as usize] += 1;
//         hit[i as usize][6] += 1;
//         i += 1
//     }
//     i = 0;
//     while i < 8 as i32 {
//         hit[2][i as usize] += 1;
//         hit[i as usize][2] += 1;
//         hit[5][i as usize] += 1;
//         hit[i as usize][5] += 1;
//         i += 1
//     }
//     i = 0;
//     while i < 8 as i32 {
//         hit[3][i as usize] += 1;
//         hit[i as usize][3] += 1;
//         hit[4][i as usize] += 1;
//         hit[i as usize][4] += 1;
//         i += 1
//     }
//     i = 0;
//     while i < 3 as i32 {
//         j = 0;
//         while j < 3 as i32 {
//             hit[i as usize][j as usize] += 1;
//             hit[i as usize][(7 as i32 - j) as usize] += 1;
//             hit[(7 as i32 - i) as usize][j as usize] += 1;
//             hit[(7 as i32 - i) as
//                 usize][(7 as i32 - j) as usize] += 1;
//             j += 1
//         }
//         i += 1
//     }
//     i = 0;
//     while i < 2 as i32 {
//         j = 0;
//         while j < 5 as i32 {
//             hit[i as usize][j as usize] += 1;
//             hit[j as usize][i as usize] += 1;
//             hit[i as usize][(7 as i32 - j) as usize] += 1;
//             hit[j as usize][(7 as i32 - i) as usize] += 1;
//             hit[(7 as i32 - i) as usize][j as usize] += 1;
//             hit[(7 as i32 - j) as usize][i as usize] += 1;
//             hit[(7 as i32 - i) as
//                 usize][(7 as i32 - j) as usize] += 1;
//             hit[(7 as i32 - j) as
//                 usize][(7 as i32 - i) as usize] += 1;
//             j += 1
//         }
//         i += 1
//     }
//     i = 0;
//     while i < 8 as i32 {
//         hit[i as usize][i as usize] += 1;
//         hit[i as usize][(7 as i32 - i) as usize] += 1;
//         i += 1
//     }
//     i = 0;
//     while i < 7 as i32 {
//         hit[i as usize][(i + 1 as i32) as usize] += 1;
//         hit[(i + 1 as i32) as usize][i as usize] += 1;
//         hit[i as usize][(6 as i32 - i) as usize] += 1;
//         hit[(i + 1 as i32) as usize][(7 as i32 - i) as usize]
//             += 1;
//         i += 1
//     }
//     i = 0;
//     while i < 6 as i32 {
//         hit[i as usize][(i + 2 as i32) as usize] += 1;
//         hit[(i + 2 as i32) as usize][i as usize] += 1;
//         hit[i as usize][(5 as i32 - i) as usize] += 1;
//         hit[(i + 2 as i32) as usize][(7 as i32 - i) as usize]
//             += 1;
//         i += 1
//     }
//     i = 0;
//     while i < 5 as i32 {
//         hit[i as usize][(i + 3 as i32) as usize] += 1;
//         hit[(i + 3 as i32) as usize][i as usize] += 1;
//         hit[i as usize][(4 as i32 - i) as usize] += 1;
//         hit[(i + 3 as i32) as usize][(7 as i32 - i) as usize]
//             += 1;
//         i += 1
//     }
//     i = 0;
//     while i < 4 as i32 {
//         hit[i as usize][(i + 4 as i32) as usize] += 1;
//         hit[(i + 4 as i32) as usize][i as usize] += 1;
//         hit[i as usize][(3 as i32 - i) as usize] += 1;
//         hit[(i + 4 as i32) as usize][(7 as i32 - i) as usize]
//             += 1;
//         i += 1
//     }
//     hit[1][1] += 2 as i32;
//     hit[1][6] += 2 as i32;
//     hit[6][1] += 2 as i32;
//     hit[6][6] += 2 as i32;
//     hit
// }