use ::libc;
extern "C" {
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    static mut end_stats_available: [[libc::c_short; 9]; 61];
    #[no_mangle]
    static mut mid_corr: [[Correlation; 9]; 61];
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Correlation {
    pub const_base: libc::c_float,
    pub const_slope: libc::c_float,
    pub sigma_base: libc::c_float,
    pub sigma_slope: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DepthInfo {
    pub cut_tries: libc::c_int,
    pub cut_depth: [libc::c_int; 2],
    pub bias: [[libc::c_int; 61]; 2],
    pub window: [[libc::c_int; 61]; 2],
}
/*
   File:          probcut.c

   Created:       March 1, 1998

   Modified:      November 24, 2002

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The initialization of the Multi-ProbCut search parameters.
*/
/* Global variables */
#[no_mangle]
pub static mut use_end_cut: [libc::c_int; 61] = [0; 61];
#[no_mangle]
pub static mut end_mpc_depth: [[libc::c_int; 4]; 61] = [[0; 4]; 61];
#[no_mangle]
pub static mut mpc_cut: [DepthInfo; 23] =
    [DepthInfo{cut_tries: 0,
               cut_depth: [0; 2],
               bias: [[0; 61]; 2],
               window: [[0; 61]; 2],}; 23];
/*
   SET_PROBCUT
   Specifies that searches to depth DEPTH are to be
   estimated using searches to depth SHALLOW_DEPTH.
*/
unsafe extern "C" fn set_probcut(mut depth: libc::c_int,
                                 mut shallow: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut this_try: libc::c_int = 0;
    this_try = mpc_cut[depth as usize].cut_tries;
    mpc_cut[depth as usize].cut_depth[this_try as usize] = shallow;
    i = 0 as libc::c_int;
    while i <= 60 as libc::c_int {
        mpc_cut[depth as usize].bias[this_try as usize][i as usize] =
            floor(128.0f64 *
                      (mid_corr[i as usize][shallow as usize].const_base +
                           mid_corr[i as usize][shallow as usize].const_slope
                               * shallow as libc::c_float) as libc::c_double)
                as libc::c_int;
        mpc_cut[depth as usize].window[this_try as usize][i as usize] =
            floor(128.0f64 *
                      (mid_corr[i as usize][shallow as usize].sigma_base +
                           mid_corr[i as usize][shallow as usize].sigma_slope
                               * shallow as libc::c_float) as libc::c_double)
                as libc::c_int;
        i += 1
    }
    mpc_cut[depth as usize].cut_tries += 1;
}
/*
   SET_END_PROBCUT
   Specifies that endgame searches with EMPTY empty disks
   are to be estimated using searches to depth SHALLOW_DEPTH.
*/
unsafe extern "C" fn set_end_probcut(mut empty: libc::c_int,
                                     mut shallow_depth: libc::c_int) {
    let mut stage: libc::c_int = 0;
    stage = 60 as libc::c_int - empty;
    if shallow_depth <= 14 as libc::c_int {
        if end_stats_available[stage as usize][shallow_depth as usize] != 0 {
            let fresh0 = use_end_cut[stage as usize];
            use_end_cut[stage as usize] = use_end_cut[stage as usize] + 1;
            end_mpc_depth[stage as usize][fresh0 as usize] = shallow_depth
        }
    };
}
/*
   INIT_PROBCUT
   Clears the tables with MPC information and chooses some
   reasonable cut pairs.
*/
#[no_mangle]
pub unsafe extern "C" fn init_probcut() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <= 22 as libc::c_int {
        mpc_cut[i as usize].cut_tries = 0 as libc::c_int;
        i += 1
    }
    i = 0 as libc::c_int;
    while i <= 60 as libc::c_int {
        use_end_cut[i as usize] = 0 as libc::c_int;
        i += 1
    }
    set_probcut(3 as libc::c_int, 1 as libc::c_int);
    set_probcut(4 as libc::c_int, 2 as libc::c_int);
    set_probcut(5 as libc::c_int, 1 as libc::c_int);
    set_probcut(6 as libc::c_int, 2 as libc::c_int);
    set_probcut(7 as libc::c_int, 3 as libc::c_int);
    set_probcut(8 as libc::c_int, 4 as libc::c_int);
    set_probcut(9 as libc::c_int, 3 as libc::c_int);
    set_probcut(10 as libc::c_int, 4 as libc::c_int);
    set_probcut(10 as libc::c_int, 6 as libc::c_int);
    set_probcut(11 as libc::c_int, 3 as libc::c_int);
    set_probcut(11 as libc::c_int, 5 as libc::c_int);
    set_probcut(12 as libc::c_int, 4 as libc::c_int);
    set_probcut(12 as libc::c_int, 6 as libc::c_int);
    set_probcut(13 as libc::c_int, 5 as libc::c_int);
    set_probcut(13 as libc::c_int, 7 as libc::c_int);
    set_probcut(14 as libc::c_int, 6 as libc::c_int);
    set_probcut(14 as libc::c_int, 8 as libc::c_int);
    set_probcut(15 as libc::c_int, 5 as libc::c_int);
    set_probcut(15 as libc::c_int, 7 as libc::c_int);
    set_probcut(16 as libc::c_int, 6 as libc::c_int);
    set_probcut(16 as libc::c_int, 8 as libc::c_int);
    set_probcut(17 as libc::c_int, 5 as libc::c_int);
    set_probcut(17 as libc::c_int, 7 as libc::c_int);
    set_probcut(18 as libc::c_int, 6 as libc::c_int);
    set_probcut(18 as libc::c_int, 8 as libc::c_int);
    set_probcut(20 as libc::c_int, 8 as libc::c_int);
    set_end_probcut(13 as libc::c_int, 1 as libc::c_int);
    set_end_probcut(14 as libc::c_int, 1 as libc::c_int);
    set_end_probcut(15 as libc::c_int, 2 as libc::c_int);
    set_end_probcut(16 as libc::c_int, 2 as libc::c_int);
    set_end_probcut(17 as libc::c_int, 2 as libc::c_int);
    set_end_probcut(18 as libc::c_int, 2 as libc::c_int);
    set_end_probcut(19 as libc::c_int, 3 as libc::c_int);
    set_end_probcut(20 as libc::c_int, 3 as libc::c_int);
    set_end_probcut(21 as libc::c_int, 4 as libc::c_int);
    set_end_probcut(22 as libc::c_int, 4 as libc::c_int);
    set_end_probcut(23 as libc::c_int, 4 as libc::c_int);
    set_end_probcut(24 as libc::c_int, 4 as libc::c_int);
    set_end_probcut(25 as libc::c_int, 4 as libc::c_int);
    set_end_probcut(26 as libc::c_int, 4 as libc::c_int);
    set_end_probcut(27 as libc::c_int, 4 as libc::c_int);
}
