use evo_sys::eval::eval;
use evo_sys::{ResultMap, Program, Instruction, ProgInspectRequest};
use evo_sys::eval::Evaluator;
use evo_sys::params as evo_params;
use data::{DataSet};
use core::config::{ResultMapConfig};
use core::{Message, EvalResult};
use std::sync::Arc;
use log::Logger;
use rand;
use rand::{Rng, thread_rng};
use std::collections::VecDeque;
use data::params::N_FEATURES;
use std;
use evo_sys::pop::PopStats;
use std::fs::File;
use params;
use std::io::Write;
use std::slice::Iter;

impl ResultMap{

    pub fn new(config: ResultMapConfig) -> ResultMap{
        let mut prog_map = Vec::with_capacity(config.pop_size);
        for _ in 0..config.pop_size{
            prog_map.push(None);
        }
//        let feat_frags = VecDeque::with_capacity(evo_params::QUEUE_LEN);
//        let instr_frags = VecDeque::with_capacity(evo_params::QUEUE_LEN);

        ResultMap{
            prog_map,
            config,
            sent_count: 0,
            recieved_count: 0,
//            feat_frags,
//            instr_frags,
        }
    }


    pub fn run(&mut self, training_data: Arc<DataSet>, test_data: Box<DataSet>, logger: &mut Logger) {
        let mut evaluator = Evaluator::new_default(training_data);
        let mutate_method = self.config.mutate_method;

        let mut rng = rand::thread_rng();

//        for _ in 0..evo_params::QUEUE_LEN{
//            let mut frag = Vec::with_capacity(1);
//            frag.push(Instruction::new_rand_instr(&self.config.prog_defaults, &mut rng));
//            self.instr_frags.push_back(ProgramFragment{instructions:frag});
//            let reg_i = rng.gen_range(0, self.config.prog_defaults.initial_regs);
//            let feat_i = rng.gen_range(0, N_FEATURES);
//            self.feat_frags.push_back(FeatLoadInfo{reg_i ,feat_i});
//        }

        while self.recieved_count < self.config.n_evals {
            if evaluator.can_recieve() {
                evaluator.add_task(Message::Cont(self.next_new_prog(mutate_method)));
            }
            else {
                self.try_put(evaluator.next_result_wait());

                if self.recieved_count % logger.freq as u64 == 0 {
                    self.update_cv(&test_data);
                    self.log_full(logger);
                }
            }
        }

        evaluator.terminate();
        self.update_cv(&test_data);
        logger.finish_run(&self);
    }

    pub fn prog_map_iter(&self) -> Iter<Option<Program>>{
        self.prog_map.iter()
    }

}


impl ResultMap{

    fn try_put(&mut self, new_entry: EvalResult) {
        self.recieved_count += 1;
        let prog = new_entry.prog;
        let inds = self.select_cell(&prog);
        let mut replace = false;

        if inds < self.config.pop_size {
            match self.prog_map[inds] {
                Some(ref old_prog) => {
                    if self.compare(&prog, old_prog){
                        replace = true
                    }
                }
                None => replace = true
            }
        }

        if replace {
            self.prog_map[inds] = Some(prog);
        }
    }


    // Returns either new random, or mutated program
    fn next_new_prog(&mut self, mutation_code: u8) -> Program{
        self.sent_count += 1;

        if self.sent_count <= self.config.initial_pop as u64{
            self.new_random_prog()  //old version had other checks
        }
        else {
            self.get_simple_mutated_genome_rand(mutation_code)
        }
    }


    //pick random prog from map and return mutated copy
    fn get_simple_mutated_genome_rand(&mut self, mutation_code: u8) -> Program {
        let mut tries = 0;
        let mut tr  = rand::thread_rng();

        while tries < self.config.pop_size * 10000 {
            if let Some(ref parent) = self.prog_map[tr.gen_range(0, self.config.pop_size)] {
                let prog = parent.mutate_copy( mutation_code);
                let inds = self.select_cell(&prog);

                if self.is_in_bounds(inds){
                    tries = 0;
                    return prog
                }
            }
            tries += 1;
        }
        //self.printout_pop_info();
        panic!("Timed out when trying to select a parent genome from results map!!");
    }


    fn new_random_prog(&self) -> Program{
        Program::new_default(&self.config.prog_defaults)
    }


    fn is_in_bounds(&self, ind: usize)->bool{
        ind < self.config.pop_size
    }

    fn update_cv(&mut self, cv_data: &DataSet) {
        for prog in self.prog_map.iter_mut(){
            if let &mut Some(ref mut genome) = prog {
                match genome.cv_fit {
                    Some(_) => (),
                    None => genome.cv_fit = Some(eval::eval_program_corrects(&genome, cv_data)),
                }
            }
        }
    }

}


impl ResultMap{ // Mess!

    pub fn write_pop_info(&self, file_name: &str, eval: ProgInspectRequest) {
        let mut f = File::create(file_name).unwrap();
        for prog in self.prog_map.iter(){
            let value = if let &Some(ref genome) = prog {
                match eval {
                    ProgInspectRequest::TestFit => genome.test_fit.unwrap(),
                    ProgInspectRequest::CV => genome.cv_fit.unwrap(),
                    ProgInspectRequest::Geno(eval) => eval(genome),
                }

                }else {
                    params::params::MIN_FIT
                };

            f.write(value.to_string().as_bytes());
            f.write(b"\t");
            f.write(b"\n");
        }
    }


    pub fn log_full(&mut self, logger: &mut Logger){
        let mut count = 0.0;

        let n_evals = logger.geno_functions.len();
        let mut bests = vec![std::f32::MIN; n_evals+2];  // +2 for 2 fitnesses
        let mut worsts = vec![std::f32::MAX; n_evals+2];
        let mut aves = vec![0f64; n_evals+2];
        let mut varis = vec![0f64; n_evals+2]; //variences

        let mut feats_distr = [0; N_FEATURES as usize];


        for prog in self.prog_map.iter_mut(){
            if let &mut Some(ref mut genome) = prog {
                count += 1.0;
                for feat in genome.get_effective_feats(0) {
                    feats_distr[feat as usize] += 1;
                }

                let values = vec![genome.test_fit.unwrap(), genome.cv_fit.unwrap()];
                let others: Vec<f32> = logger.geno_functions.iter().map(|f| f(genome)).collect();

                for (i, value) in values.iter().chain(others.iter()).enumerate(){
                    aves[i] += *value as f64;

                    if *value > bests[i] {bests[i] = *value}
                    if *value < worsts[i] {worsts[i] =*value }
                }
            }
        }

        for value in aves.iter_mut(){
            *value /= count;
        }

        for prog in self.prog_map.iter_mut(){
            if let &mut Some(ref mut genome) = prog {
                let values = vec![genome.test_fit.unwrap(), genome.cv_fit.unwrap()];
                let others: Vec<f32> = logger.geno_functions.iter().map(|f| f(genome)).collect();

                for (i, value) in values.iter().chain(others.iter()).enumerate(){
                    varis[i] += (*value as f64-aves[i]).powi(2);
                }
            }
        }

        for value in varis.iter_mut(){
            *value /= count;
        }

        logger.log_test_fits(PopStats{
            best:bests[0],
            worst:worsts[0],
            ave:aves[0],
            sd:varis[0].sqrt(),
        });

        logger.log_cv_fits(PopStats{
            best:bests[1],
            worst:worsts[1],
            ave:aves[1],
            sd:varis[1].sqrt(),
        });

        for i in 0..n_evals{
            logger.log_geno_stat(PopStats{
                best:bests[i+2],
                worst:worsts[i+2],
                ave:aves[i+2],
                sd:varis[i+2].sqrt(),
            }, i);
        }

        let unique_feat_count = feats_distr.iter().fold(0u8, |mut acc, x| {if *x > 0 {acc+=1;} acc});
        logger.log_feat_count(unique_feat_count);
        logger.log_feat_distr(&feats_distr);
    }


}