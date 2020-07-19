
use crate::src::moves::first_flip_direction;
use crate::src::libc;

/*
   cntflip.c

   Last modified:     November 1, 2000
*/
unsafe extern "C" fn AnyDrctnlFlips(mut sq: *mut libc::c_int,
                                    mut inc: libc::c_int,
                                    mut color: libc::c_int,
                                    mut oppcol: libc::c_int) -> libc::c_int {
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
        if *pt == color { return 1 as libc::c_int }
    }
    return 0 as libc::c_int;
}
/*
   cntflip.h

   Automatically created by ENDMACRO on Wed Mar 17 21:01:12 1999

   Last modified:   December 25, 1999
*/

pub unsafe extern "C" fn AnyFlips_compact(mut board: *mut libc::c_int,
                                          mut sqnum: libc::c_int,
                                          mut color: libc::c_int,
                                          mut oppcol: libc::c_int)
 -> libc::c_int {
    let mut sq = 0 as *mut libc::c_int;
    let mut inc = 0 as *mut libc::c_int;
    sq = &mut *board.offset(sqnum as isize) as *mut libc::c_int;
    inc = first_flip_direction[sqnum as usize];
    loop  {
        if AnyDrctnlFlips(sq, *inc, color, oppcol) != 0 {
            return 1 as libc::c_int
        }
        inc = inc.offset(1);
        if !(*inc != 0) { break ; }
    }
    return 0 as libc::c_int;
}
