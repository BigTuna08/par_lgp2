//use data;
//use data::metabolites;
//use params as global_params;
use rand::{Rng, thread_rng};
//
//use std::collections::HashSet;
//use std::fs::File;
//use std::io::Write;
//
//use super::ops;
//use super::super::{Program, Instruction, ExecutionRegArray, params, InstructionType};

use ProgDefaults;
use evo_sys::{Program, Instruction};
use data::params::FEAT_RNG;
use core::{RegIndType, FeatIndType};

impl Program{
    pub fn new_default(defs: &ProgDefaults) -> Program{
        let mut rng = thread_rng();
        let n_feats = rng.gen_range(defs.initial_feat_min, defs.initial_feat_max);
        let n_instr = rng.gen_range(defs.initial_instr_min, defs.initial_instr_max);
        let n_header_instr = rng.gen_range(defs.initial_header_instr_min, defs.initial_header_instr_max);


        let mut feats = Vec::with_capacity(n_feats as usize);
        for _i in 0..n_feats{
            feats.push((rng.gen_range(0, defs.initial_regs), rng.gen_range(FEAT_RNG.start, FEAT_RNG.end) as FeatIndType));
        }

        let mut instr = Vec::with_capacity(n_instr);

        let mut header_instr = Vec::with_capacity(n_header_instr);

        Program{
            features:feats,
            instructions: instr,
            header_instructions: header_instr,
            test_fit: None,
            cv_fit: None,
        }

    }

}