pub mod ops;
pub mod prog;
pub mod mutation;
mod instr;

use params as global_params;
use rand::{Rng, ThreadRng};
use evo_sys::FeatLoadInfo;




pub fn reg_2_feat(feat_list: &Vec<FeatLoadInfo>, reg: &u8) -> u8{
//    println!("list-{:?} reg-{}", feat_list, reg);
    let feat_i = global_params::params::MAX_REGS - *reg as usize -1;
    feat_list[feat_i].feat_i.clone()
}