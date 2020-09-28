/*
   cntflip.h

   Automatically created by ENDMACRO on Wed Mar 17 21:01:12 1999

   Last modified:   December 25, 1999
*/
pub unsafe fn AnyFlips_compact(board: &mut [i32; 128],
                               first_flip_direction: &mut [*mut i32; 100],
                               sqnum: i32, color: i32, oppcol: i32) -> i32 {

    let sq = &board[sqnum as usize];
    let mut inc = first_flip_direction[sqnum as usize];
    loop  {
        let any_drctnl_flips = {
            let sq: &i32 = sq;
            let inc: i32 = *inc;
            let sq = sq as *const i32;
            let mut pt = sq.offset(inc as isize);
            if *pt == oppcol {
                pt = pt.offset(inc as isize);
                if *pt == oppcol {
                    pt = pt.offset(inc as isize);
                    if *pt == oppcol {
                        pt = pt.offset(inc as isize);
                        if *pt == oppcol {
                            pt = pt.offset(inc as isize);
                            if *pt == oppcol {
                                pt = pt.offset(inc as isize);
                                if *pt == oppcol { pt = pt.offset(inc as isize) }
                            }
                        }
                    }
                }
                if *pt == color {
                    return 1
                }
            }
            0
        };//  AnyDrctnlFlips(sq, *inc, color, oppcol);

        if any_drctnl_flips != 0 {
            return 1 as i32
        }
        inc = inc.offset(1);
        if !(*inc != 0) { break ; }
    }
    0
}
