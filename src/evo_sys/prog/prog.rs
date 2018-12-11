//use data;
//use data::metabolites;
use params as global_params;
use rand::{Rng, thread_rng};
use evo_sys::eval::registers::PROG_REG;
//
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
//
use super::ops;
//use super::super::{Program, Instruction, ExecutionRegArray, params, InstructionType};

use core::config::ProgDefaults;
use evo_sys::{Program, Instruction, InstructionType, FeatLoadInfo};
use data::params::N_FEATURES;
use core::{RegIndType, FeatIndType};


impl Program {//                      Constructors                              //

    pub fn new_default(defs: &ProgDefaults) -> Program {
//        println!("Creating new default prog with {:?}", &defs);
        let mut rng = thread_rng();
        let n_feats = rng.gen_range(defs.initial_feat_min, defs.initial_feat_max);
        let n_instr = rng.gen_range(defs.initial_instr_min, defs.initial_instr_max);
        let n_header_instr = rng.gen_range(defs.initial_header_instr_min, defs.initial_header_instr_max);


        let mut header_instr = Vec::with_capacity(n_header_instr);
        for _i in 0..n_header_instr {
            header_instr.push(Instruction::new_rand_instr(defs, &mut rng))
        }


        let mut feats = Vec::with_capacity(n_feats as usize);
        for _i in 0..n_feats {
            let reg_i = rng.gen_range(0, defs.initial_regs);
            let feat_i = rng.gen_range(0, N_FEATURES);
            feats.push(FeatLoadInfo{reg_i, feat_i});
        }

        let mut instr = Vec::with_capacity(n_instr);
        for _i in 0..n_instr {
            instr.push(Instruction::new_rand_instr(defs, &mut rng))
        }


//        println!("Created new default prog with {} {} {}", &defs);
        Program {
            features: feats,
            instructions: instr,
            header_instructions: header_instr,
            test_fit: None,
            cv_fit: None,
        }
    }


    //return a new empty program
    pub fn new_empty() -> Program{
        Program{
            header_instructions:Vec::new(),
            features:Vec::new(),
            instructions:Vec::new(),
            test_fit: None,
            cv_fit:None,
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
        self.get_effective_feats(0).len()                                                             //   <---- Fill in
    }

    //return # of effective instructions
    pub fn get_effective_len(&self, return_reg_ind: u8) -> usize{
        self.get_effective_instrs_good(0).len()                                                               //   <---- Fill in
    }


    //return # of effective instructions
    pub fn get_abs_len(&self) -> usize{
        self.instructions.len()
    }

}



impl Program{ //                 Analysis / compress              ///     <--- Clean up needed!

    pub fn create_compressed(&self) -> Program{
        let instr_i = self.get_effective_instrs_good(0);
        let instrs: Vec<Instruction> = instr_i.into_iter().map(|i| {self.instructions[i].clone()}).collect();
        Program{
            features: Vec::new(),
            instructions: instrs,
            header_instructions: Vec::new(),
            test_fit:None,
            cv_fit: None,
        }
    }


    pub fn get_effective_instrs_good(&self, return_reg_ind: u8) -> Vec<usize>{
        if self.get_abs_len() == 0 {
            return Vec::new()
        }

        let mut eff_instrs = HashSet::new();

        for end_i in self.get_quit_set(0) {
            self.get_effective_instrs_from_fixed_final_point(0, end_i)       // <--- Not effeicient :(
                .into_iter()
                .for_each(|x| {eff_instrs.insert(x);})
        }
        let mut eff_instrs: Vec<usize> = eff_instrs.drain().collect();
        eff_instrs.sort();
        eff_instrs
    }


    //return index of all possible final instruction indexs upto and including prog termination point
    fn get_quit_set(&self, return_reg_ind: u8) -> HashSet<usize>{
        let last_ind = self.get_exit_index(return_reg_ind);
        let mut quit_set = HashSet::new();
        quit_set.insert(last_ind);

        for  (i, instr) in self.instructions.iter().enumerate() {
            if i == last_ind {
                break;
            }
            if let InstructionType::Terminate =  ops::get_type(instr){
                quit_set.insert(i);
            }
        }
        quit_set
    }


    // returns index of last important instruction
    fn get_exit_index(&self, return_reg_ind: u8) -> usize{

        let mut last_br = false;  // last instr a branch?
        let mut first_uc_quit_index = 0; // index of first unconditional quit


        for instr in self.instructions.iter() {  // iterate forwards
            let op_type = ops::get_type(instr);
            if let InstructionType::Skip = op_type{
                last_br = true;
            }
            else {
                if let InstructionType::Terminate = op_type{
                    if !last_br {  //unconditional quit
                        break;
                    }
                }
                last_br = false;
            }
            first_uc_quit_index += 1;                   //    <------ Move to top of loop?????????
        }


        for (i, instr) in self.instructions.iter().enumerate().rev(){ // find last effective assignment instr
            if i <= first_uc_quit_index && instr.dest == return_reg_ind {
                return i
            }
        }
        0 // no assignemt to return reg before uc quit
    }


    pub fn get_effective_instrs_from_fixed_final_point(&self, return_reg_ind: u8, end_i: usize) -> HashSet<usize>{

        let mut eff_regs = HashSet::new();
        let mut eff_instrs = HashSet::new();

        let mut last_eff = false;
        eff_regs.insert(return_reg_ind);
        eff_instrs.insert(end_i);


        for (i, instr) in self.instructions.iter().enumerate().rev(){
            if i > end_i {continue;}
            let follows_branch = i >= 1 && self.instructions[i-1].is_branch();

            match ops::get_type(instr) {

                InstructionType::Value => {
                    if eff_regs.contains(&instr.dest) {
                        if !follows_branch {
                            eff_regs.remove(&instr.dest);
                        }
                        eff_regs.insert(instr.src1);
                        eff_regs.insert(instr.src2);
                        eff_instrs.insert(i);
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
                        eff_instrs.insert(i);
                    }
                },

                InstructionType::Terminate => {
                    last_eff = i == end_i; //asssume only effective is end_i
                },

                InstructionType::NoOp => {
                    last_eff = false //never effective
                },

            }
        }
//        println!("from end i {} set is {:?}", end_i, &eff_instrs);
        eff_instrs
    }
}




impl Program{ //                   Getters                                   //


    pub fn get_effective_feats(&self, return_reg_ind: u8) -> HashSet<u8>{
        let mut eff_regs = HashSet::new();

        for instr_i in self.get_effective_instrs_good(0){
            let instr = self.instructions[instr_i];
            eff_regs.insert(instr.src1);
            eff_regs.insert(instr.src2);
        }

        eff_regs.retain(|&x|  x >= (global_params::params::MAX_REGS - self.features.len()) as u8);
        eff_regs.iter().map(|x| super::reg_2_feat(&self.features, x)).collect()
    }
}
