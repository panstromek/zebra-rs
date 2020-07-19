use ::libc;
/* max number of types above */
static mut my_degrees: [libc::c_int; 5] =
    [0 as libc::c_int, 7 as libc::c_int, 15 as libc::c_int, 31 as libc::c_int,
     63 as libc::c_int];
static mut my_seps: [libc::c_int; 5] =
    [0 as libc::c_int, 3 as libc::c_int, 1 as libc::c_int, 3 as libc::c_int,
     1 as libc::c_int];
/*
 * Initially, everything is set up as if from :
 *      initstate( 1, &randtbl, 128 );
 * Note that this initialization takes advantage of the fact that srandom()
 * advances the front and rear pointers 10*rand_deg times, and hence the
 * rear pointer which starts at 0 will also end up at zero; thus the zeroeth
 * element of the state information, which contains info about the current
 * position of the rear pointer is just
 *  MAX_TYPES*(rptr - state) + TYPE_3 == TYPE_3.
 */
static mut my_randtbl: [libc::c_ulong; 32] =
    [3 as libc::c_int as libc::c_ulong,
     0x9a319039 as libc::c_uint as libc::c_ulong,
     0x32d9c024 as libc::c_uint as libc::c_ulong,
     0x9b663182 as libc::c_uint as libc::c_ulong,
     0x5da1f342 as libc::c_uint as libc::c_ulong,
     0xde3b81e0 as libc::c_uint as libc::c_ulong,
     0xdf0a6fb5 as libc::c_uint as libc::c_ulong,
     0xf103bc02 as libc::c_uint as libc::c_ulong,
     0x48f340fb as libc::c_uint as libc::c_ulong,
     0x7449e56b as libc::c_uint as libc::c_ulong,
     0xbeb1dbb0 as libc::c_uint as libc::c_ulong,
     0xab5c5918 as libc::c_uint as libc::c_ulong,
     0x946554fd as libc::c_uint as libc::c_ulong,
     0x8c2e680f as libc::c_uint as libc::c_ulong,
     0xeb3d799f as libc::c_uint as libc::c_ulong,
     0xb11ee0b7 as libc::c_uint as libc::c_ulong,
     0x2d436b86 as libc::c_uint as libc::c_ulong,
     0xda672e2a as libc::c_uint as libc::c_ulong,
     0x1588ca88 as libc::c_uint as libc::c_ulong,
     0xe369735d as libc::c_uint as libc::c_ulong,
     0x904f35f7 as libc::c_uint as libc::c_ulong,
     0xd7158fd6 as libc::c_uint as libc::c_ulong,
     0x6fa6f051 as libc::c_uint as libc::c_ulong,
     0x616e6b96 as libc::c_uint as libc::c_ulong,
     0xac94efdc as libc::c_uint as libc::c_ulong,
     0x36413f93 as libc::c_uint as libc::c_ulong,
     0xc622c298 as libc::c_uint as libc::c_ulong,
     0xf5a42ab8 as libc::c_uint as libc::c_ulong,
     0x8a88d77b as libc::c_uint as libc::c_ulong,
     0xf5ad9d0e as libc::c_uint as libc::c_ulong,
     0x8999220b as libc::c_uint as libc::c_ulong,
     0x27fb47b9 as libc::c_uint as libc::c_ulong];
/*
 * fptr and rptr are two pointers into the state info, a front and a rear
 * pointer.  These two pointers are always rand_sep places aparts, as they cycle
 * cyclically through the state information.  (Yes, this does mean we could get
 * away with just one pointer, but the code for random() is more efficient this
 * way).  The pointers are left positioned as they would be from the call
 *          initstate( 1, randtbl, 128 )
 * (The position of the rear pointer, rptr, is really 0 (as explained above
 * in the initialization of randtbl) because the state table pointer is set
 * to point to randtbl[1] (as explained below).
 */
// Initialized in run_static_initializers
static mut my_fptr: *mut libc::c_long =
    0 as *const libc::c_long as *mut libc::c_long;
// Initialized in run_static_initializers
static mut my_rptr: *mut libc::c_long =
    0 as *const libc::c_long as *mut libc::c_long;
/*
 * The following things are the pointer to the state information table,
 * the type of the current generator, the degree of the current polynomial
 * being used, and the separation between the two pointers.
 * Note that for efficiency of random(), we remember the first location of
 * the state information, not the zeroeth.  Hence it is valid to access
 * state[-1], which is used to store the type of the R.N.G.
 * Also, we remember the last location, since this is more efficient than
 * indexing every time to find the address of the last element to see if
 * the front and rear pointers have wrapped.
 */
// Initialized in run_static_initializers
static mut my_state: *mut libc::c_long =
    0 as *const libc::c_long as *mut libc::c_long;
static mut my_rand_type: libc::c_int = 3 as libc::c_int;
static mut my_rand_deg: libc::c_int = 31 as libc::c_int;
static mut my_rand_sep: libc::c_int = 3 as libc::c_int;
// Initialized in run_static_initializers
static mut my_end_ptr: *mut libc::c_long =
    0 as *const libc::c_long as *mut libc::c_long;
/*
 * srandom:
 * Initialize the random number generator based on the given seed.  If the
 * type is the trivial no-state-information type, just remember the seed.
 * Otherwise, initializes state[] based on the given "seed" via a linear
 * congruential generator.  Then, the pointers are set to known locations
 * that are exactly rand_sep places apart.  Lastly, it cycles the state
 * information a given number of times to get rid of any initial dependencies
 * introduced by the L.C.R.N.G.
 * Note that the initialization of randtbl[] for default usage relies on
 * values produced by this routine.
 */
#[no_mangle]
pub unsafe extern "C" fn my_srandom(mut x: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if my_rand_type == 0 as libc::c_int {
        *my_state.offset(0 as libc::c_int as isize) = x as libc::c_long
    } else {
        j = 1 as libc::c_int;
        *my_state.offset(0 as libc::c_int as isize) = x as libc::c_long;
        i = 1 as libc::c_int;
        while i < my_rand_deg {
            *my_state.offset(i as isize) =
                1103515245 as libc::c_int as libc::c_long *
                    *my_state.offset((i - 1 as libc::c_int) as isize) +
                    12345 as libc::c_int as libc::c_long;
            i += 1
        }
        my_fptr =
            &mut *my_state.offset(my_rand_sep as isize) as *mut libc::c_long;
        my_rptr =
            &mut *my_state.offset(0 as libc::c_int as isize) as
                *mut libc::c_long;
        i = 0 as libc::c_int;
        while i < 10 as libc::c_int * my_rand_deg { my_random(); i += 1 }
    }
    return 0 as libc::c_int;
}
/*
 * initstate:
 * Initialize the state information in the given array of n bytes for
 * future random number generation.  Based on the number of bytes we
 * are given, and the break values for the different R.N.G.'s, we choose
 * the best (largest) one we can and set things up for it.  srandom() is
 * then called to initialize the state information.
 * Note that on return from srandom(), we set state[-1] to be the type
 * multiplexed with the current value of the rear pointer; this is so
 * successive calls to initstate() won't lose this information and will
 * be able to restart with setstate().
 * Note: the first thing we do is save the current state, if any, just like
 * setstate() so that it doesn't matter when initstate is called.
 * Returns a pointer to the old state.
 */
#[no_mangle]
pub unsafe extern "C" fn my_initstate(mut seed: libc::c_uint,
                                      mut arg_state: *mut libc::c_char,
                                      mut n: libc::c_int)
 -> *mut libc::c_char {
    let mut ostate =
        &mut *my_state.offset(-(1 as libc::c_int) as isize) as
            *mut libc::c_long as *mut libc::c_char; /* first location */
    if my_rand_type == 0 as libc::c_int {
        *my_state.offset(-(1 as libc::c_int) as isize) =
            my_rand_type as libc::c_long
    } else {
        *my_state.offset(-(1 as libc::c_int) as isize) =
            5 as libc::c_int as libc::c_long *
                my_rptr.wrapping_offset_from(my_state) as libc::c_long +
                my_rand_type as libc::c_long
    } /* must set end_ptr before srandom */
    if n < 32 as libc::c_int {
        if n < 8 as libc::c_int { return 0 as *mut libc::c_char }
        my_rand_type = 0 as libc::c_int;
        my_rand_deg = 0 as libc::c_int;
        my_rand_sep = 0 as libc::c_int
    } else if n < 64 as libc::c_int {
        my_rand_type = 1 as libc::c_int;
        my_rand_deg = 7 as libc::c_int;
        my_rand_sep = 3 as libc::c_int
    } else if n < 128 as libc::c_int {
        my_rand_type = 2 as libc::c_int;
        my_rand_deg = 15 as libc::c_int;
        my_rand_sep = 1 as libc::c_int
    } else if n < 256 as libc::c_int {
        my_rand_type = 3 as libc::c_int;
        my_rand_deg = 31 as libc::c_int;
        my_rand_sep = 3 as libc::c_int
    } else {
        my_rand_type = 4 as libc::c_int;
        my_rand_deg = 63 as libc::c_int;
        my_rand_sep = 1 as libc::c_int
    }
    my_state =
        &mut *(arg_state as
                   *mut libc::c_long).offset(1 as libc::c_int as isize) as
            *mut libc::c_long;
    my_end_ptr =
        &mut *my_state.offset(my_rand_deg as isize) as *mut libc::c_long;
    my_srandom(seed as libc::c_int);
    if my_rand_type == 0 as libc::c_int {
        *my_state.offset(-(1 as libc::c_int) as isize) =
            my_rand_type as libc::c_long
    } else {
        *my_state.offset(-(1 as libc::c_int) as isize) =
            5 as libc::c_int as libc::c_long *
                my_rptr.wrapping_offset_from(my_state) as libc::c_long +
                my_rand_type as libc::c_long
    }
    return ostate;
}
/*
 * setstate:
 * Restore the state from the given state array.
 * Note: it is important that we also remember the locations of the pointers
 * in the current state information, and restore the locations of the pointers
 * from the old state information.  This is done by multiplexing the pointer
 * location into the zeroeth word of the state information.
 * Note that due to the order in which things are done, it is OK to call
 * setstate() with the same state as the current state.
 * Returns a pointer to the old state information.
 */
#[no_mangle]
pub unsafe extern "C" fn my_setstate(mut arg_state: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut new_state = arg_state as *mut libc::c_long; /* set end_ptr too */
    let mut type_0 =
        (*new_state.offset(0 as libc::c_int as isize) %
             5 as libc::c_int as libc::c_long) as libc::c_int;
    let mut rear =
        (*new_state.offset(0 as libc::c_int as isize) /
             5 as libc::c_int as libc::c_long) as libc::c_int;
    let mut ostate =
        &mut *my_state.offset(-(1 as libc::c_int) as isize) as
            *mut libc::c_long as *mut libc::c_char;
    if my_rand_type == 0 as libc::c_int {
        *my_state.offset(-(1 as libc::c_int) as isize) =
            my_rand_type as libc::c_long
    } else {
        *my_state.offset(-(1 as libc::c_int) as isize) =
            5 as libc::c_int as libc::c_long *
                my_rptr.wrapping_offset_from(my_state) as libc::c_long +
                my_rand_type as libc::c_long
    }
    match type_0 {
        0 | 1 | 2 | 3 | 4 => {
            my_rand_type = type_0;
            my_rand_deg = my_degrees[type_0 as usize];
            my_rand_sep = my_seps[type_0 as usize]
        }
        _ => { }
    }
    my_state =
        &mut *new_state.offset(1 as libc::c_int as isize) as
            *mut libc::c_long;
    if my_rand_type != 0 as libc::c_int {
        my_rptr = &mut *my_state.offset(rear as isize) as *mut libc::c_long;
        my_fptr =
            &mut *my_state.offset(((rear + my_rand_sep) % my_rand_deg) as
                                      isize) as *mut libc::c_long
    }
    my_end_ptr =
        &mut *my_state.offset(my_rand_deg as isize) as *mut libc::c_long;
    return ostate;
}
/*
 * random:
 * If we are using the trivial TYPE_0 R.N.G., just do the old linear
 * congruential bit.  Otherwise, we do our fancy trinomial stuff, which is the
 * same in all ther other cases due to all the global variables that have been
 * set up.  The basic operation is to add the number at the rear pointer into
 * the one at the front pointer.  Then both pointers are advanced to the next
 * location cyclically in the table.  The value returned is the sum generated,
 * reduced to 31 bits by throwing away the "least random" low bit.
 * Note: the code takes advantage of the fact that both the front and
 * rear pointers can't wrap on the same call by not testing the rear
 * pointer if the front one has wrapped.
 * Returns a 31-bit random number.
 */
#[no_mangle]
pub unsafe extern "C" fn my_random() -> libc::c_long {
    let mut i: libc::c_long = 0; /* chucking least random bit */
    if my_rand_type == 0 as libc::c_int {
        let ref mut fresh0 = *my_state.offset(0 as libc::c_int as isize);
        *fresh0 =
            *my_state.offset(0 as libc::c_int as isize) *
                1103515245 as libc::c_int as libc::c_long +
                12345 as libc::c_int as libc::c_long &
                0x7fffffff as libc::c_int as libc::c_long;
        i = *fresh0
    } else {
        *my_fptr += *my_rptr;
        i =
            *my_fptr >> 1 as libc::c_int &
                0x7fffffff as libc::c_int as libc::c_long;
        my_fptr = my_fptr.offset(1);
        if my_fptr >= my_end_ptr {
            my_fptr = my_state;
            my_rptr = my_rptr.offset(1)
        } else {
            my_rptr = my_rptr.offset(1);
            if my_rptr >= my_end_ptr { my_rptr = my_state }
        }
    }
    return i;
}
unsafe extern "C" fn run_static_initializers() {
    my_fptr =
        &mut *my_randtbl.as_mut_ptr().offset((3 as libc::c_int +
                                                  1 as libc::c_int) as isize)
            as *mut libc::c_ulong as *mut libc::c_long;
    my_rptr =
        &mut *my_randtbl.as_mut_ptr().offset(1 as libc::c_int as isize) as
            *mut libc::c_ulong as *mut libc::c_long;
    my_state =
        &mut *my_randtbl.as_mut_ptr().offset(1 as libc::c_int as isize) as
            *mut libc::c_ulong as *mut libc::c_long;
    my_end_ptr =
        &mut *my_randtbl.as_mut_ptr().offset((31 as libc::c_int +
                                                  1 as libc::c_int) as isize)
            as *mut libc::c_ulong as *mut libc::c_long
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
