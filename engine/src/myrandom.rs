/* max number of types above */
use flip::doflip::WrappingOffsetFrom;
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
static mut my_fptr: usize = 4;
static mut my_rptr: usize = 1;
static mut my_state: usize = 1;
static mut my_end_ptr: usize = 32;
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
static my_rand_deg: i32 = 31;
static my_rand_sep: i32 = 3;

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
    my_randtbl[my_state] = x as u64;
    let mut i = 1;
    while i < my_rand_deg {
        my_randtbl[(my_state + (i as usize) )] =
            1103515245i64
                .wrapping_mul(my_randtbl[(my_state + ((i - 1) as usize))] as i64)
                .wrapping_add(12345) as u64;
        i += 1
    }
    my_fptr = (my_state + (my_rand_sep as usize));
    my_rptr = my_state;
    let mut i = 0;
    while i < 10 * my_rand_deg {
        my_random();
        i += 1
    }
    0
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
    my_randtbl[my_fptr] = my_randtbl[my_fptr].wrapping_add(my_randtbl[my_rptr]);
    i = (my_randtbl[my_fptr] >> 1 & 0x7fffffff) as i64;
    my_fptr = my_fptr + (1);
    if my_fptr >= my_end_ptr {
        my_fptr = my_state;
        my_rptr = my_rptr + (1)
    } else {
        my_rptr = my_rptr + 1;
        if my_rptr >= my_end_ptr { my_rptr = my_state }
    }
    i
}
