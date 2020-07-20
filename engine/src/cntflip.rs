use crate::src::moves::first_flip_direction;

/*
   cntflip.c

   Last modified:     November 1, 2000
*/
unsafe fn AnyDrctnlFlips(mut sq: *mut i32,
                                    mut inc: i32,
                                    mut color: i32,
                                    mut oppcol: i32) -> i32 {
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
        if *pt == color { return 1 as i32 }
    }
    return 0 as i32;
}
/*
   cntflip.h

   Automatically created by ENDMACRO on Wed Mar 17 21:01:12 1999

   Last modified:   December 25, 1999
*/

pub unsafe fn AnyFlips_compact(mut board: *mut i32,
                                          mut sqnum: i32,
                                          mut color: i32,
                                          mut oppcol: i32)
 -> i32 {
    let mut sq = 0 as *mut i32;
    let mut inc = 0 as *mut i32;
    sq = &mut *board.offset(sqnum as isize) as *mut i32;
    inc = first_flip_direction[sqnum as usize];
    loop  {
        if AnyDrctnlFlips(sq, *inc, color, oppcol) != 0 {
            return 1 as i32
        }
        inc = inc.offset(1);
        if !(*inc != 0) { break ; }
    }
    return 0 as i32;
}
