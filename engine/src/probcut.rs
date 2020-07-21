use crate::src::epcstat::end_stats_available;
use crate::src::pcstat::mid_corr;
use crate::src::midgame::DepthInfo;

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
pub unsafe fn set_end_probcut(mut empty: i32,
                          mut shallow_depth: i32) {
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