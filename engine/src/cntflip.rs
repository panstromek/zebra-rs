/*
   cntflip.h

   Automatically created by ENDMACRO on Wed Mar 17 21:01:12 1999

   Last modified:   December 25, 1999
*/
pub fn AnyFlips_compact(board: &[i32; 128], mut inc : &[i32],
                               sqnum: i32, color: i32, oppcol: i32) -> i32 {

    let sq = sqnum;
    loop  {
        let any_drctnl_flips = {
            let inc: i32 = inc[0];
            let mut pt = sq + (inc);
            if board[pt as usize] == oppcol {
                pt = pt + (inc);
                if board[pt as usize] == oppcol {
                    pt = pt + (inc);
                    if board[pt as usize] == oppcol {
                        pt = pt + (inc);
                        if board[pt as usize] == oppcol {
                            pt = pt + (inc);
                            if board[pt as usize] == oppcol {
                                pt = pt + (inc);
                                if board[pt as usize] == oppcol {
                                    pt = pt + (inc)
                                }
                            }
                        }
                    }
                }
                if board[pt as usize] == color {
                    return 1
                }
            }
            0
        };

        if any_drctnl_flips != 0 {
            return 1
        }
        inc = &inc[1..];
        if !(inc[0] != 0) {
            break;
        }
    }
    0
}
