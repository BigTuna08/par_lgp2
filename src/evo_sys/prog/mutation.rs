//use data;
use params as global_params;
use rand;
use rand::{Rng, ThreadRng};
use rand::distributions::Distribution;
use rand::distributions::Exp1;
use evo_sys::{Program, Instruction, ProgramFragment, MutationMode, FeatLoadInfo};
use core::{FeatIndType, RegIndType};
use evo_sys::params as evo_params;
use data::params::N_FEATURES;
use std::collections::VecDeque;


pub fn test_rand(){
    let n = 1000;
    let mut rng = rand::thread_rng();
//    let exp = Exp1;

    for i in 0..n {
        let v = (Exp1.sample(&mut rng) + 1.0) as usize;
        println!("V is {}", v);
    }
}


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
    evo_params::INSTR_OPTIONS[rng.gen_range(0, evo_params::N_INSTR_OPTIONS)]
}


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

impl Program{

    pub fn mutate_copy(&self, instr_frags: &mut VecDeque<ProgramFragment>, feat_frags: &mut VecDeque<FeatLoadInfo>, code: u8) -> Program{
        let mut rng = rand::thread_rng();

        let mut features = Vec::with_capacity(self.features.len() + 2);
        let mut instructions = Vec::with_capacity(self.instructions.len() + 2);
        let mut header_instructions = Vec::with_capacity(self.header_instructions.len() + 2);


        for &FeatLoadInfo{reg_i, feat_i} in self.features.iter(){

            if rng.gen::<f32>() < evo_params::ADD_FEAT_RATE {
                features.push(feat_frags.pop_front().unwrap());
                feat_frags.push_back(FeatLoadInfo{reg_i, feat_i});
            }
            if rng.gen::<f32>() < evo_params::DEL_FEAT_RATE {
                continue
            }
            if rng.gen::<f32>() < evo_params::CHG_FEAT_RATE {
                let feat_i = rng.gen_range(0, N_FEATURES);
                features.push(FeatLoadInfo{reg_i, feat_i});
            }
        }


        let mut mut_mode = MutationMode::Normal;
        let mut copied = Vec::new();

        for instr in self.instructions.iter(){

            match mut_mode {    // Update mode
                MutationMode::Normal => {
                    if rng.gen::<f32>() < evo_params::DEL_RATE {
                        mut_mode = MutationMode::Del
                    }
                    else if rng.gen::<f32>() < evo_params::COPY_RATE {
                        mut_mode = MutationMode::Copy;
                    }
                }
                MutationMode::Copy => {
                    if rng.gen::<f32>() > evo_params::COPY_CONT_RATE {  // exit copy mode -> normal
                        mut_mode = MutationMode::Normal;

                        if copied.len() > 0 {
                            // choose between queue and local
                            if rng.gen::<f32>() < evo_params::QUEUE_V_LOCAL_COPY_RATE {
                                for instr in instr_frags.pop_front().unwrap().instructions {
                                    instructions.push(instr);
                                }

                                instr_frags.push_back(ProgramFragment{instructions:copied}); // push copied to back
                                copied = Vec::new();
                            }
                            else {
                                for instr in copied{
                                    instructions.push(instr);
                                }
                                copied = Vec::new();
                            }
                        }
                    }
                }
                MutationMode::Del => {
                    if rng.gen::<f32>() > evo_params::DEL_CONT_RATE { // exit del mode -> normal
                        mut_mode = MutationMode::Normal
                    }
                }
            }

            match mut_mode {    // Take action
                MutationMode::Normal => {
                    instructions.push(instr.mutate_copy(&mut rng))
                }
                MutationMode::Copy => {
                    copied.push(instr.mutate_copy(&mut rng));
                    if rng.gen::<f32>() > evo_params::COPY_CONT_RATE {
                        mut_mode = MutationMode::Normal
                    }
                }
                MutationMode::Del => {
                    if rng.gen::<f32>() > evo_params::DEL_CONT_RATE {
                        mut_mode = MutationMode::Normal
                    }
                }
            }
        }

        features.shrink_to_fit();
        instructions.shrink_to_fit();
        header_instructions.shrink_to_fit();
        Program{
            features,
            header_instructions,
            instructions,
            test_fit:None,
            cv_fit:None,
        }
    }
}