use crate::src::epcstat::END_STATS_AVAILABLE;
use crate::src::pcstat::MID_CORR;
use crate::src::midgame::DepthInfo;
use crate::src::stubs::floor;
/*
   File:          probcut.c

   Created:       March 1, 1998

   Modified:      November 24, 2002

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      The initialization of the Multi-ProbCut search parameters.
*/
pub static mut use_end_cut___: [i32; 61] = [0; 61];

pub static mut end_mpc_depth___: [[i32; 4]; 61] = [[0; 4]; 61];

pub static mut mpc_cut___: [DepthInfo; 23] = [DepthInfo { cut_tries: 0, cut_depth: [0; 2], bias: [[0; 61]; 2], window: [[0; 61]; 2] }; 23];

/*
   SET_END_PROBCUT
   Specifies that endgame searches with EMPTY empty disks
   are to be estimated using searches to depth SHALLOW_DEPTH.
*/
fn set_end_probcut(use_end_cut_: &mut [i32; 61], end_mpc_depth_: &mut [[i32; 4]; 61],
                          empty: usize, shallow_depth: i32) {
    let mut stage = 60 - empty;
    if shallow_depth <= 14 {
        if END_STATS_AVAILABLE[stage][shallow_depth as usize] != 0 {
            let fresh0 = use_end_cut_[stage];
            use_end_cut_[stage] = use_end_cut_[stage] + 1;
            end_mpc_depth_[stage][fresh0 as usize] = shallow_depth
        }
    };
}

/*
   SET_PROBCUT
   Specifies that searches to depth DEPTH are to be
   estimated using searches to depth SHALLOW_DEPTH.
*/
fn set_probcut(mpc_cut_: &mut [DepthInfo; 23], depth: i32, shallow: i32) {
    let mut this_try = mpc_cut_[depth as usize].cut_tries;
    mpc_cut_[depth as usize].cut_depth[this_try as usize] = shallow;
    let mut i = 0;
    while i <= 60 {
        mpc_cut_[depth as usize].bias[this_try as usize][i] =
            floor(128.0f64 *
                (MID_CORR[i][shallow as usize].const_base +
                    MID_CORR[i][shallow as usize].const_slope
                        * shallow as f32) as f64)
                as i32;
        mpc_cut_[depth as usize].window[this_try as usize][i] =
            floor(128.0f64 *
                (MID_CORR[i][shallow as usize].sigma_base +
                    MID_CORR[i][shallow as usize].sigma_slope
                        * shallow as f32) as f64)
                as i32;
        i += 1
    }
    mpc_cut_[depth as usize].cut_tries += 1;
}
/*
   INIT_PROBCUT
   Clears the tables with MPC information and chooses some
   reasonable cut pairs.
*/

pub fn init_probcut(mpc_cut_: &mut [DepthInfo; 23], use_end_cut_: &mut [i32; 61], end_mpc_depth_: &mut [[i32; 4]; 61]) {
    *use_end_cut_ = [0; 61];
    mpc_cut_.iter_mut().for_each(|info| info.cut_tries = 0);
    set_probcut(mpc_cut_, 3, 1);
    set_probcut(mpc_cut_, 4, 2);
    set_probcut(mpc_cut_, 5, 1);
    set_probcut(mpc_cut_, 6, 2);
    set_probcut(mpc_cut_, 7, 3);
    set_probcut(mpc_cut_, 8, 4);
    set_probcut(mpc_cut_, 9, 3);
    set_probcut(mpc_cut_, 10, 4);
    set_probcut(mpc_cut_, 10, 6);
    set_probcut(mpc_cut_, 11, 3);
    set_probcut(mpc_cut_, 11, 5);
    set_probcut(mpc_cut_, 12, 4);
    set_probcut(mpc_cut_, 12, 6);
    set_probcut(mpc_cut_, 13, 5);
    set_probcut(mpc_cut_, 13, 7);
    set_probcut(mpc_cut_, 14, 6);
    set_probcut(mpc_cut_, 14, 8);
    set_probcut(mpc_cut_, 15, 5);
    set_probcut(mpc_cut_, 15, 7);
    set_probcut(mpc_cut_, 16, 6);
    set_probcut(mpc_cut_, 16, 8);
    set_probcut(mpc_cut_, 17, 5);
    set_probcut(mpc_cut_, 17, 7);
    set_probcut(mpc_cut_, 18, 6);
    set_probcut(mpc_cut_, 18, 8);
    set_probcut(mpc_cut_, 20, 8);
    set_end_probcut(use_end_cut_, end_mpc_depth_,13, 1);
    set_end_probcut(use_end_cut_, end_mpc_depth_,14, 1);
    set_end_probcut(use_end_cut_, end_mpc_depth_,15, 2);
    set_end_probcut(use_end_cut_, end_mpc_depth_,16, 2);
    set_end_probcut(use_end_cut_, end_mpc_depth_,17, 2);
    set_end_probcut(use_end_cut_, end_mpc_depth_,18, 2);
    set_end_probcut(use_end_cut_, end_mpc_depth_,19, 3);
    set_end_probcut(use_end_cut_, end_mpc_depth_,20, 3);
    set_end_probcut(use_end_cut_, end_mpc_depth_,21, 4);
    set_end_probcut(use_end_cut_, end_mpc_depth_,22, 4);
    set_end_probcut(use_end_cut_, end_mpc_depth_,23, 4);
    set_end_probcut(use_end_cut_, end_mpc_depth_,24, 4);
    set_end_probcut(use_end_cut_, end_mpc_depth_,25, 4);
    set_end_probcut(use_end_cut_, end_mpc_depth_,26, 4);
    set_end_probcut(use_end_cut_, end_mpc_depth_,27, 4);
}

