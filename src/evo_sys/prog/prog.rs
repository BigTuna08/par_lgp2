use data;
//use data::metabolites;
use params as global_params;
use rand::{Rng, thread_rng, seq};
use evo_sys::eval::registers::PROG_REG;
//
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
//
use super::ops;
//use super::super::{Program, Instruction, ExecutionRegArray, params, InstructionType};

use core::config::ProgDefaults;
use evo_sys::{Program, Instruction, InstructionType, params};
use data::params::N_FEATURES;
use core::{RegIndType, FeatIndType};



impl Program {//                      Constructors                              //

    pub fn new_default(defs: &ProgDefaults) -> Program {
        let mut rng = thread_rng();
        let n_calc_regs = rng.gen_range(defs.initial_comp_reg_min, defs.initial_comp_reg_max);
        let n_feats = rng.gen_range(defs.initial_feat_min, defs.initial_feat_max);
        let n_instr = rng.gen_range(defs.initial_instr_min, defs.initial_instr_max);

        Program::new_random(n_instr, n_calc_regs, params::N_OPS, n_feats)

    }


    pub fn new_random(n_instr: usize, n_calc_regs: u8, n_ops: u8, n_feats: u8) -> Program{
        let mut rng = thread_rng();

        let features = seq::sample_iter(&mut rng, 0..data::params::N_FEATURES, n_feats as usize).unwrap();
        let mut instructions = Vec::with_capacity(n_instr);

        for _ in 0..n_instr {
            instructions.push(Instruction::new_random(n_calc_regs, n_feats, n_ops, &mut rng));
        }

        Program{ n_calc_regs, features, instructions, test_fit:None, cv_fit:None,}
    }


    //return a new empty program
    pub fn new_empty() -> Program{
        Program{
            features:Vec::new(),
            instructions:Vec::new(),
            test_fit: None,
            cv_fit:None,
            n_calc_regs:0,
        }
    }


}


impl Program{
    pub fn get_inds_simple(&self) -> (usize, usize){
        let compressed = self.create_compressed();
        let mut used_regs = HashSet::new();
        for instr in compressed.instructions.iter(){
            used_regs.insert(instr.dest);
            used_regs.insert(instr.src1);
            used_regs.insert(instr.src2);
        }
        (compressed.instructions.len(), used_regs.len())
    }

    //return # of effective features
    pub fn get_n_effective_feats(&self, return_reg_ind: u8) -> usize{
        self.get_effective_feats(0).len()
    }

    //return # of effective instructions                                 <---- make more effiecient
    pub fn get_effective_len(&self, return_reg_ind: u8) -> usize{
//        self.get_effective_instrs_good(0).len()
        self.create_compressed().instructions.len()
    }


    //return # of effective instructions
    pub fn get_abs_len(&self) -> usize{
        self.instructions.len()
    }

}



impl Program{ //                 Analysis / compress


    pub fn create_compressed(&self) -> Program{
        let return_reg_ind = 0;
        let mut eff_instrs: Vec<Instruction> = Vec::new();
        let mut eff_regs = HashSet::new();
        eff_regs.insert(return_reg_ind);
        let mut last_eff = false;
        let mut had_effect = false;

        let mut maybe_eff_regs = HashSet::new();


        for (i, instr) in self.instructions.iter().enumerate().rev(){

            let follows_branch = (i >= 1) && self.instructions[i-1].is_branch();

            match ops::get_type(instr) {

                InstructionType::Value => {
                    if eff_regs.contains(&instr.dest) {  // effective

                        if !follows_branch {
                            eff_regs.remove(&instr.dest);
                            maybe_eff_regs.insert(instr.dest);
                        }

                        eff_instrs.push(*instr);
                        eff_regs.insert(instr.src1);
                        eff_regs.insert(instr.src2);
                        had_effect = true;
                        last_eff = true;
                    }
                    else {
                        last_eff = false;
                    }
                },

                InstructionType::Skip => {
                    if last_eff { // becuase branch only ever skips one.
                        eff_regs.insert(instr.src1);
                        eff_regs.insert(instr.src2);
                        eff_instrs.push(*instr);
                    }
                },

                InstructionType::Terminate => {
                    if follows_branch { // effective
                        if had_effect{
                            eff_instrs.push(*instr);
                            for i in maybe_eff_regs.drain(){
                                eff_regs.insert(i);
                            }

                            assert_eq!(maybe_eff_regs.len(), 0);
                            last_eff = true;
                        }
                    }
                    else { // throw out everything past here (uc quit)
                        eff_regs = HashSet::new();
                        eff_regs.insert(return_reg_ind);
                        eff_instrs = Vec::new();
                        had_effect = false;
                        last_eff = false;
                    }
                },

                InstructionType::NoOp => {
                    last_eff = false //never effective
                },

            }
        }
        eff_instrs.reverse();


        Program{
            features: Vec::new(),
            instructions: eff_instrs,
            n_calc_regs: self.n_calc_regs,
            test_fit:None,
            cv_fit: None,
        }
    }




//    pub fn get_effective_instrs_good(&self, return_reg_ind: u8) -> Vec<usize>{
//        if self.get_abs_len() == 0 {
//            return Vec::new()
//        }
//
//        let mut eff_instrs = HashSet::new();
//
//        for end_i in self.get_quit_set(0) {
//            self.get_effective_instrs_from_fixed_final_point(0, end_i)       // <--- Not effeicient :(
//                .into_iter()
//                .for_each(|x| {eff_instrs.insert(x);})
//        }
//        let mut eff_instrs: Vec<usize> = eff_instrs.drain().collect();
//        eff_instrs.sort();
//        eff_instrs
//    }




}




impl Program{ //                   Getters                                   //


    pub fn get_effective_feats(&self, return_reg_ind: u8) -> HashSet<u8>{
        let mut eff_regs = HashSet::new();

        for instr in self.create_compressed().instructions.iter(){
//            let instr = self.instructions[instr_i];
            eff_regs.insert(instr.src1);
            eff_regs.insert(instr.src2);
        }

        eff_regs.retain(|&x|  x >= (global_params::params::MAX_REGS - self.features.len()) as u8);
        eff_regs.iter().map(|x| super::reg_2_feat(&self.features, x)).collect()
    }
}




