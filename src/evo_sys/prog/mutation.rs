//use dataMgmt;
//use params as global_params;
//use rand;
//use rand::{Rng};
//use super::super::{Program};


use params as global_params;
use rand;
use rand::{Rng, ThreadRng};
use rand::distributions::Distribution;
use rand::distributions::Exp1;
use evo_sys::{Program, Instruction};
use core::{FeatIndType, RegIndType};
use evo_sys::params as evo_params;
use data::params::N_FEATURES;
use std::collections::VecDeque;
use evo_sys::params::N_OPS;

impl Program{

    //creates a mutated copy of prog
    pub fn mutate_copy(&self, code: u8) -> Program{
        match code {
            0 => self.test_mutate_copy(),
            1 => self.no_meta_mutate_copy(),
            _ => panic!("Invalid mutate copy code!")
        }
    }

    pub fn test_mutate_copy(&self) -> Program{
        let n = rand::thread_rng().gen_range(0, 25);
        match n {
            0 => self.ins_instr_copy(),
            1 => self.del_instr_copy(),
            2 => self.ins_comp_copy(),
            3 => self.del_comp_copy(),
            4 => self.ins_feat_copy(),
            5 => self.del_feat_copy(),
            6 => self.swap_feat_copy(),
            _ => self.mut_instr_copy(),
        }
    }

    pub fn no_meta_mutate_copy(&self) -> Program{
        let n = rand::thread_rng().gen_range(0, 10);
        match n {
            0 => self.ins_instr_copy(),
            1 => self.del_instr_copy(),
            _ => self.mut_instr_copy(),
        }
    }

    pub fn mut_instr_copy(&self) -> Program{
        let features = self.features.clone();
        let n_calc_regs = self.n_calc_regs;
        let mut rng = rand::thread_rng();

        let instructions = self.instructions.iter().map(|instr| {
            if rand::thread_rng().gen_weighted_bool(super::super::params::MUT_INSTR_COPY_RATE){
                instr.mutate_copy(&mut rng)
            }
                else {
                    instr.clone()
                }
        }).collect();

        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None}
    }


    pub fn ins_instr_copy(&self) -> Program{
        let features = self.features.clone();
        let n_calc_regs = self.n_calc_regs;
        let mut rng = rand::thread_rng();

        let mut instructions = Vec::with_capacity(self.instructions.len() + 10 ); //random 10 to allow for insertions

        for instr in self.instructions.iter() {
            instructions.push(instr.clone());
            if rng.gen_weighted_bool(super::super::params::INSTR_INSERT_RATE) {
                instructions.push(self.rand_instr(&mut rng))
            }
        }
        instructions.shrink_to_fit();

        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None, }
    }


    pub fn del_instr_copy(&self) -> Program {
        let features = self.features.clone();
        let n_calc_regs = self.n_calc_regs;
        let mut rng = rand::thread_rng();

        let mut instructions = Vec::with_capacity(self.instructions.len());

        for instr in self.instructions.iter() {

            if !rng.gen_weighted_bool(super::super::params::INSTR_DEL_RATE) {
                instructions.push(instr.clone());
            }
        }
        instructions.shrink_to_fit();
        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None, }
    }


    // very simple now, only add reg as option, does not change instructions
    // in future may want to find a way to distribute some work to new reg while
    // maintaining the programs correctness
    pub fn ins_comp_copy(&self) -> Program{
        if self.features.len() + self.n_calc_regs as usize> global_params::params::MAX_REGS - 2 { //just do micro mutation
            return self.mut_instr_copy()
        }

        let features = self.features.clone();
        let n_calc_regs = self.n_calc_regs +1;

        let mut instructions = Vec::with_capacity(self.instructions.len());

        for instr in self.instructions.iter() {
            instructions.push(instr.clone());
        }

        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None,}
    }


    //very simple now, just rm instr if it has deleted reg
    pub fn del_comp_copy(&self) -> Program{
        if self.n_calc_regs == 1 {
            return Program::new_random(self.instructions.len(), 5, N_OPS, self.features.len() as u8)
        }
        let features = self.features.clone();
        let n_calc_regs = self.n_calc_regs -1;

        let mut instructions = Vec::with_capacity(self.instructions.len());

        for instr in self.instructions.iter() {
            if !instr.contains_reg(n_calc_regs){
                instructions.push(instr.clone());
            }
        }
        instructions.shrink_to_fit();
        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None,}
    }


    pub fn ins_feat_copy(&self) -> Program{
        let mut rng = rand::thread_rng();
        let mut features = self.features.clone();

        if self.features.len() == N_FEATURES as usize{ //just do micro mutation
            return self.mut_instr_copy()
        }

        let mut new_feat = rng.gen_range(0, N_FEATURES);
        let mut tries = 0;
        while features.contains(&new_feat) {
            new_feat =  rng.gen_range(0, N_FEATURES);
            tries += 1;
            if tries > global_params::params::DUPLICATE_TIME_OUT { panic!("Error getting non dupicate!, {:?}", &self)}
        }
        features.push(new_feat);

        let n_calc_regs = self.n_calc_regs;


        let mut instructions = Vec::with_capacity(self.instructions.len());

        for instr in self.instructions.iter() {
            instructions.push(instr.clone());
        }

        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None,}
    }


    //very simple now, just rm instr if it has deleted reg
    pub fn del_feat_copy(&self) -> Program{
        if self.features.len() == 1 { //do micro instead
            return self.mut_instr_copy()
        }

        let mut features = self.features.clone();
        features.pop();
        let removed_reg = (global_params::params::MAX_REGS - 1- features.len()) as u8;

        let n_calc_regs = self.n_calc_regs;


        let mut instructions = Vec::with_capacity(self.instructions.len());

        for instr in self.instructions.iter() {
            if !instr.contains_reg(removed_reg){
                instructions.push(instr.clone());
            }
        }
        instructions.shrink_to_fit();
        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None,}
    }


    pub fn swap_feat_copy(&self) -> Program{

        if self.features.len() == N_FEATURES as usize { //just do micro mutation
            return self.mut_instr_copy()
        }
        if self.features.len() == 0 {
            return self.ins_feat_copy()
        }

        let mut rng = rand::thread_rng();
        let mut features = self.features.clone();

        let mut new_feat = rng.gen_range(0, N_FEATURES);
        let mut tries = 0;
        while features.contains(&new_feat) {
            new_feat =  rng.gen_range(0, N_FEATURES);
            tries += 1;
            if tries > global_params::params::DUPLICATE_TIME_OUT { panic!("Error getting non dupicate!, {:?}", &self)}
        }


        let to_replace = rng.gen_range(0, features.len());
        features[to_replace] = new_feat;

        let n_calc_regs = self.n_calc_regs;

        let mut instructions = Vec::with_capacity(self.instructions.len());

        for instr in self.instructions.iter() {
            instructions.push(instr.clone());
        }

        Program{n_calc_regs, features, instructions, test_fit:None, cv_fit:None, }
    }
}




//
////use data;
//use params as global_params;
//use rand;
//use rand::{Rng, ThreadRng};
//use rand::distributions::Distribution;
//use rand::distributions::Exp1;
//use evo_sys::{Program, Instruction, ProgramFragment, MutationMode, FeatLoadInfo};
//use core::{FeatIndType, RegIndType};
//use evo_sys::params as evo_params;
//use N_FEATURES;
//use std::collections::VecDeque;
//
//
//pub fn test_rand(){
//    let n = 1000;
//    let mut rng = rand::thread_rng();
////    let exp = Exp1;
//
//    for i in 0..n {
//        let v = (Exp1.sample(&mut rng) + 1.0) as usize;
//        println!("V is {}", v);
//    }
//}
//
//
fn mutate_reg(orig: u8, rng: &mut ThreadRng)->u8{
    let v = (Exp1.sample(rng) + 1.0) as u8;

    if rng.gen() && v <= orig {
//        println!("true");
        return orig - v
    }
    else if (v as usize + orig as usize) < global_params::params::MAX_REGS {
//        println!("false");
        return orig + v
    }

//    println!("other");
    return orig
}


fn mutate_op(rng: &mut ThreadRng)->u8{ // pick random
//    evo_params::INSTR_OPTIONS[rng.gen_range(0, evo_params::N_INSTR_OPTIONS)]
    rng.gen_range(0, evo_params::N_OPS)
}
//
//
impl Instruction{
    pub fn mutate_copy(&self, rng: &mut ThreadRng) -> Instruction{

        let dest = if rng.gen::<f32>() > evo_params::MICRO_MUT_RATE {
                            self.dest
                        }
                        else{
                            mutate_reg(self.dest, rng)
                        };

        let src1 = if rng.gen::<f32>() > evo_params::MICRO_MUT_RATE {
                            self.src1
                        }
                        else{
                            mutate_reg(self.src1, rng)
                        };


        let src2 = if rng.gen::<f32>() > evo_params::MICRO_MUT_RATE {
                            self.src2
                        }
                        else{
                            mutate_reg(self.src2, rng)
                        };


        let op = if rng.gen::<f32>() > evo_params::MICRO_MUT_RATE {
                            self.op
                        }
                        else{
                            mutate_op(rng)
                        };

        Instruction{dest, op, src1, src2}
    }
}

//impl Program{
//
//    pub fn mutate_copy(&self, instr_frags: &mut VecDeque<ProgramFragment>, feat_frags: &mut VecDeque<FeatLoadInfo>, code: u8) -> Program{
//        let mut rng = rand::thread_rng();
//
//        let mut features = Vec::with_capacity(self.features.len() + 2);
//        let mut instructions = Vec::with_capacity(self.instructions.len() + 2);
//        let mut header_instructions = Vec::with_capacity(self.header_instructions.len() + 2);
//
//
//        for &FeatLoadInfo{reg_i, feat_i} in self.features.iter(){
//
//            if rng.gen::<f32>() < evo_params::ADD_FEAT_RATE {
//                features.push(feat_frags.pop_front().unwrap());
//                feat_frags.push_back(FeatLoadInfo{reg_i, feat_i});
//            }
//            if rng.gen::<f32>() < evo_params::DEL_FEAT_RATE {
//                continue
//            }
//            if rng.gen::<f32>() < evo_params::CHG_FEAT_RATE {
//                let feat_i = rng.gen_range(0, N_FEATURES);
//                features.push(FeatLoadInfo{reg_i, feat_i});
//            }
//        }
//
//
//        let mut mut_mode = MutationMode::Normal;
//        let mut copied = Vec::new();
//
//        for instr in self.instructions.iter(){
//
//            match mut_mode {    // Update mode
//                MutationMode::Normal => {
//                    if rng.gen::<f32>() < evo_params::DEL_RATE {
//                        mut_mode = MutationMode::Del
//                    }
//                    else if rng.gen::<f32>() < evo_params::COPY_RATE {
//                        mut_mode = MutationMode::Copy;
//                    }
//                }
//                MutationMode::Copy => {
//                    if rng.gen::<f32>() > evo_params::COPY_CONT_RATE {  // exit copy mode -> normal
//                        mut_mode = MutationMode::Normal;
//
//                        if copied.len() > 0 {
//                            // choose between queue and local
//                            if rng.gen::<f32>() < evo_params::QUEUE_V_LOCAL_COPY_RATE {
//                                for instr in instr_frags.pop_front().unwrap().instructions {
//                                    instructions.push(instr);
//                                }
//
//                                instr_frags.push_back(ProgramFragment{instructions:copied}); // push copied to back
//                                copied = Vec::new();
//                            }
//                            else {
//                                for instr in copied{
//                                    instructions.push(instr);
//                                }
//                                copied = Vec::new();
//                            }
//                        }
//                    }
//                }
//                MutationMode::Del => {
//                    if rng.gen::<f32>() > evo_params::DEL_CONT_RATE { // exit del mode -> normal
//                        mut_mode = MutationMode::Normal
//                    }
//                }
//            }
//
//            match mut_mode {    // Take action
//                MutationMode::Normal => {
//                    instructions.push(instr.mutate_copy(&mut rng))
//                }
//                MutationMode::Copy => {
//                    copied.push(instr.mutate_copy(&mut rng));
//                    if rng.gen::<f32>() > evo_params::COPY_CONT_RATE {
//                        mut_mode = MutationMode::Normal
//                    }
//                }
//                MutationMode::Del => {
//                    if rng.gen::<f32>() > evo_params::DEL_CONT_RATE {
//                        mut_mode = MutationMode::Normal
//                    }
//                }
//            }
//        }
//
//        features.shrink_to_fit();
//        instructions.shrink_to_fit();
//        header_instructions.shrink_to_fit();
//        Program{
//            features,
//            header_instructions,
//            instructions,
//            test_fit:None,
//            cv_fit:None,
//        }
//    }
//}