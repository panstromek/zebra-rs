/* max number of types above */
use flip::doflip::WrappingOffsetFrom;

static my_degrees: [i32; 5] =
    [0 as i32, 7 as i32, 15 as i32, 31 as i32,
     63 as i32];
static my_seps: [i32; 5] =
    [0 as i32, 3 as i32, 1 as i32, 3 as i32,
     1 as i32];
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
static mut my_randtbl: [u64; 32] =
    [3 as i32 as u64,
     0x9a319039 as u32 as u64,
     0x32d9c024 as u32 as u64,
     0x9b663182 as u32 as u64,
     0x5da1f342 as u32 as u64,
     0xde3b81e0 as u32 as u64,
     0xdf0a6fb5 as u32 as u64,
     0xf103bc02 as u32 as u64,
     0x48f340fb as u32 as u64,
     0x7449e56b as u32 as u64,
     0xbeb1dbb0 as u32 as u64,
     0xab5c5918 as u32 as u64,
     0x946554fd as u32 as u64,
     0x8c2e680f as u32 as u64,
     0xeb3d799f as u32 as u64,
     0xb11ee0b7 as u32 as u64,
     0x2d436b86 as u32 as u64,
     0xda672e2a as u32 as u64,
     0x1588ca88 as u32 as u64,
     0xe369735d as u32 as u64,
     0x904f35f7 as u32 as u64,
     0xd7158fd6 as u32 as u64,
     0x6fa6f051 as u32 as u64,
     0x616e6b96 as u32 as u64,
     0xac94efdc as u32 as u64,
     0x36413f93 as u32 as u64,
     0xc622c298 as u32 as u64,
     0xf5a42ab8 as u32 as u64,
     0x8a88d77b as u32 as u64,
     0xf5ad9d0e as u32 as u64,
     0x8999220b as u32 as u64,
     0x27fb47b9 as u32 as u64];
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
static mut my_fptr: *mut i64 =
    0 as *const i64 as *mut i64;
// Initialized in run_static_initializers
static mut my_rptr: *mut i64 =
    0 as *const i64 as *mut i64;
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
static mut my_state: *mut i64 =
    0 as *const i64 as *mut i64;
static mut my_rand_type: i32 = 3 as i32;
static mut my_rand_deg: i32 = 31 as i32;
static mut my_rand_sep: i32 = 3 as i32;
// Initialized in run_static_initializers
static mut my_end_ptr: *mut i64 =
    0 as *const i64 as *mut i64;
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

pub unsafe fn my_srandom(x: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if my_rand_type == 0 as i32 {
        *my_state.offset(0 as i32 as isize) = x as i64
    } else {
        j = 1 as i32;
        *my_state.offset(0 as i32 as isize) = x as i64;
        i = 1 as i32;
        while i < my_rand_deg {
            *my_state.offset(i as isize) =
                (1103515245 as i32 as i64)
                    .wrapping_mul(*my_state.offset((i - 1 as i32) as isize))
                    .wrapping_add(12345 as i32 as i64);
            i += 1
        }
        my_fptr =
            &mut *my_state.offset(my_rand_sep as isize) as *mut i64;
        my_rptr =
            &mut *my_state.offset(0 as i32 as isize) as
                *mut i64;
        i = 0 as i32;
        while i < 10 as i32 * my_rand_deg { my_random(); i += 1 }
    }
    return 0 as i32;
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

pub unsafe fn my_initstate(seed: u32,
                                      arg_state: *mut i8,
                                      n: i32)
 -> *mut i8 {
    let ostate =
        &mut *my_state.offset(-(1 as i32) as isize) as
            *mut i64 as *mut i8; /* first location */
    if my_rand_type == 0 as i32 {
        *my_state.offset(-(1 as i32) as isize) =
            my_rand_type as i64
    } else {
        *my_state.offset(-(1 as i32) as isize) =
            5 as i32 as i64 *
                my_rptr.wrapping_offset_from_(my_state) as i64 +
                my_rand_type as i64
    } /* must set end_ptr before srandom */
    if n < 32 as i32 {
        if n < 8 as i32 { return 0 as *mut i8 }
        my_rand_type = 0 as i32;
        my_rand_deg = 0 as i32;
        my_rand_sep = 0 as i32
    } else if n < 64 as i32 {
        my_rand_type = 1 as i32;
        my_rand_deg = 7 as i32;
        my_rand_sep = 3 as i32
    } else if n < 128 as i32 {
        my_rand_type = 2 as i32;
        my_rand_deg = 15 as i32;
        my_rand_sep = 1 as i32
    } else if n < 256 as i32 {
        my_rand_type = 3 as i32;
        my_rand_deg = 31 as i32;
        my_rand_sep = 3 as i32
    } else {
        my_rand_type = 4 as i32;
        my_rand_deg = 63 as i32;
        my_rand_sep = 1 as i32
    }
    my_state =
        &mut *(arg_state as
                   *mut i64).offset(1) as
            *mut i64;
    my_end_ptr =
        &mut *my_state.offset(my_rand_deg as isize) as *mut i64;
    my_srandom(seed as i32);
    if my_rand_type == 0 as i32 {
        *my_state.offset(-(1 as i32) as isize) =
            my_rand_type as i64
    } else {
        *my_state.offset(-(1 as i32) as isize) =
            5 as i32 as i64 *
                my_rptr.wrapping_offset_from_(my_state) as i64 +
                my_rand_type as i64
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

pub unsafe fn my_setstate(arg_state: *mut i8)
 -> *mut i8 {
    let new_state = arg_state as *mut i64; /* set end_ptr too */
    let type_0 =
        (*new_state.offset(0 as i32 as isize) %
             5 as i32 as i64) as i32;
    let rear =
        (*new_state.offset(0 as i32 as isize) /
             5 as i32 as i64) as i32;
    let ostate =
        &mut *my_state.offset(-(1 as i32) as isize) as
            *mut i64 as *mut i8;
    if my_rand_type == 0 as i32 {
        *my_state.offset(-(1 as i32) as isize) =
            my_rand_type as i64
    } else {
        *my_state.offset(-(1 as i32) as isize) =
            5 as i32 as i64 *
                my_rptr.wrapping_offset_from_(my_state) as i64 +
                my_rand_type as i64
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
        &mut *new_state.offset(1) as
            *mut i64;
    if my_rand_type != 0 as i32 {
        my_rptr = &mut *my_state.offset(rear as isize) as *mut i64;
        my_fptr =
            &mut *my_state.offset(((rear + my_rand_sep) % my_rand_deg) as
                                      isize) as *mut i64
    }
    my_end_ptr =
        &mut *my_state.offset(my_rand_deg as isize) as *mut i64;
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

pub unsafe fn my_random() -> i64 {
    let mut i: i64 = 0; /* chucking least random bit */
    if my_rand_type == 0 as i32 {
        let ref mut fresh0 = *my_state.offset(0 as i32 as isize);
        *fresh0 =
            *my_state.offset(0 as i32 as isize) *
                1103515245 as i32 as i64 +
                12345 as i32 as i64 &
                0x7fffffff as i32 as i64;
        i = *fresh0
    } else {
        *my_fptr = (*my_fptr).wrapping_add(*my_rptr);
        i =
            *my_fptr >> 1 as i32 &
                0x7fffffff as i32 as i64;
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
pub unsafe fn run_static_initializers() {
    my_fptr =  my_randtbl.as_ptr().offset((4)) as *mut i64;
    my_rptr = my_randtbl.as_ptr().offset(1) as *mut i64;
    my_state = my_randtbl.as_ptr().offset(1) as *mut i64;
    my_end_ptr = my_randtbl.as_ptr().offset((32)) as *mut i64;
}
