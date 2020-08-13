use crate::src::epcstat::end_stats_available;
use crate::src::pcstat::mid_corr;
use crate::src::midgame::DepthInfo;
use crate::src::stubs::floor;
/*
   File:          probcut.c

   Created:       March 1, 1998

   Modified:      November 24, 2002

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The initialization of the Multi-ProbCut search parameters.
*/
pub static mut use_end_cut: [i32; 61] = [0; 61];

pub static mut end_mpc_depth: [[i32; 4]; 61] = [[0; 4]; 61];

pub static mut mpc_cut: [DepthInfo; 23] =
    [DepthInfo{cut_tries: 0,
        cut_depth: [0; 2],
        bias: [[0; 61]; 2],
        window: [[0; 61]; 2],}; 23];

/*
   SET_END_PROBCUT
   Specifies that endgame searches with EMPTY empty disks
   are to be estimated using searches to depth SHALLOW_DEPTH.
*/
pub unsafe fn set_end_probcut(empty: i32,
                          shallow_depth: i32) {
    let mut stage: i32 = 0;
    stage = 60 as i32 - empty;
    if shallow_depth <= 14 as i32 {
        if end_stats_available[stage as usize][shallow_depth as usize] != 0 {
            let fresh0 = use_end_cut[stage as usize];
            use_end_cut[stage as usize] = use_end_cut[stage as usize] + 1;
            end_mpc_depth[stage as usize][fresh0 as usize] = shallow_depth
        }
    };
}

/*
   SET_PROBCUT
   Specifies that searches to depth DEPTH are to be
   estimated using searches to depth SHALLOW_DEPTH.
*/
unsafe fn set_probcut(depth: i32,
                      shallow: i32) {
    let mut i: i32 = 0;
    let mut this_try: i32 = 0;
    this_try = mpc_cut[depth as usize].cut_tries;
    mpc_cut[depth as usize].cut_depth[this_try as usize] = shallow;
    i = 0 as i32;
    while i <= 60 as i32 {
        mpc_cut[depth as usize].bias[this_try as usize][i as usize] =
            floor(128.0f64 *
                (mid_corr[i as usize][shallow as usize].const_base +
                    mid_corr[i as usize][shallow as usize].const_slope
                        * shallow as f32) as f64)
                as i32;
        mpc_cut[depth as usize].window[this_try as usize][i as usize] =
            floor(128.0f64 *
                (mid_corr[i as usize][shallow as usize].sigma_base +
                    mid_corr[i as usize][shallow as usize].sigma_slope
                        * shallow as f32) as f64)
                as i32;
        i += 1
    }
    mpc_cut[depth as usize].cut_tries += 1;
}
/*
   INIT_PROBCUT
   Clears the tables with MPC information and chooses some
   reasonable cut pairs.
*/

pub unsafe fn init_probcut() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i <= 22 as i32 {
        mpc_cut[i as usize].cut_tries = 0 as i32;
        i += 1
    }
    i = 0 as i32;
    while i <= 60 as i32 {
        use_end_cut[i as usize] = 0 as i32;
        i += 1
    }
    set_probcut(3 as i32, 1 as i32);
    set_probcut(4 as i32, 2 as i32);
    set_probcut(5 as i32, 1 as i32);
    set_probcut(6 as i32, 2 as i32);
    set_probcut(7 as i32, 3 as i32);
    set_probcut(8 as i32, 4 as i32);
    set_probcut(9 as i32, 3 as i32);
    set_probcut(10 as i32, 4 as i32);
    set_probcut(10 as i32, 6 as i32);
    set_probcut(11 as i32, 3 as i32);
    set_probcut(11 as i32, 5 as i32);
    set_probcut(12 as i32, 4 as i32);
    set_probcut(12 as i32, 6 as i32);
    set_probcut(13 as i32, 5 as i32);
    set_probcut(13 as i32, 7 as i32);
    set_probcut(14 as i32, 6 as i32);
    set_probcut(14 as i32, 8 as i32);
    set_probcut(15 as i32, 5 as i32);
    set_probcut(15 as i32, 7 as i32);
    set_probcut(16 as i32, 6 as i32);
    set_probcut(16 as i32, 8 as i32);
    set_probcut(17 as i32, 5 as i32);
    set_probcut(17 as i32, 7 as i32);
    set_probcut(18 as i32, 6 as i32);
    set_probcut(18 as i32, 8 as i32);
    set_probcut(20 as i32, 8 as i32);
    set_end_probcut(13 as i32, 1 as i32);
    set_end_probcut(14 as i32, 1 as i32);
    set_end_probcut(15 as i32, 2 as i32);
    set_end_probcut(16 as i32, 2 as i32);
    set_end_probcut(17 as i32, 2 as i32);
    set_end_probcut(18 as i32, 2 as i32);
    set_end_probcut(19 as i32, 3 as i32);
    set_end_probcut(20 as i32, 3 as i32);
    set_end_probcut(21 as i32, 4 as i32);
    set_end_probcut(22 as i32, 4 as i32);
    set_end_probcut(23 as i32, 4 as i32);
    set_end_probcut(24 as i32, 4 as i32);
    set_end_probcut(25 as i32, 4 as i32);
    set_end_probcut(26 as i32, 4 as i32);
    set_end_probcut(27 as i32, 4 as i32);
}

