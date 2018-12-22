use evo_sys::eval::eval;
use evo_sys::{CVTMap, Program, Instruction, ProgInspectRequest};
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




impl CVTMap{

    pub fn new(config: ResultMapConfig) -> CVTMap{
        let mut prog_map = Vec::with_capacity(config.pop_size);
        for _ in 0..config.pop_size{
            prog_map.push(None);
        }




        CVTMap{
            prog_map,
            config,
            sent_count: 0,
            recieved_count: 0,
        }
    }





    pub fn run(&mut self, training_data: Arc<DataSet>, test_data: Box<DataSet>, logger: &mut Logger) {
        let mut evaluator = Evaluator::new_default(training_data);
        let mutate_method = self.config.mutate_method;

        let mut rng = rand::thread_rng();

        let centriods = basic_multi_trial(389, self.config.pop_size);


        while self.recieved_count < self.config.n_evals {
            if evaluator.can_recieve() {
                evaluator.add_task(Message::Cont(self.next_new_prog(mutate_method)));
            }
            else {
                self.try_put(evaluator.next_result_wait(), &centriods);

                if self.recieved_count % logger.freq as u64 == 0 {
                    self.update_cv(&test_data);
                    self.log_full(logger);
                }
            }
        }

        evaluator.terminate();
        self.update_cv(&test_data);
//        logger.finish_run(&self);
    }

    pub fn prog_map_iter(&self) -> Iter<Option<Program>>{
        self.prog_map.iter()
    }

}


impl CVTMap{

    fn select_cell(&self, prog: &Program, centriods: &Vec<Vec<bool>>,) -> usize{
        let (mut min_d, mut min_i) = (0, 0);
//        for
    }


    fn try_put(&mut self, new_entry: EvalResult, centriods: &Vec<Vec<bool>>,) {
        self.recieved_count += 1;
        let prog = new_entry.prog;
        let inds = self.select_cell(&prog, centriods);
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


impl CVTMap{ // Mess!

    pub fn write_to_store_compressed(&self, file_name: &str){
        let mut f = File::create(file_name).unwrap();
        for prog in self.prog_map.iter(){
            if let &Some(ref genome) = prog {
                for feat in genome.features.iter(){
                    f.write(feat.to_string().as_bytes());
                    f.write(b" ");
                }
                f.write(b"|");

                for instr in genome.create_compressed().instructions.iter(){
                    f.write(instr.to_save_string().as_bytes());
                }
                f.write(b"\n");
            }
//            f.write(value.to_string().as_bytes());
//            f.write(b"\t");
//            f.write(b"\n");
        }
    }

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




fn random_bvecs(n_vec: usize, n_bools: usize) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();
    (0..n_vec).map(|_|
        (0..n_bools).map(|_|
            rng.gen()).collect()).collect()
}


fn total_hd(x: &Vec<Vec<bool>>) -> u32{
    let mut s = 0;
    let k = x.len();
    for i in 0..k{
        for j in 0..k{
            s += dh(&x[i], &x[j]);
        }
    }
    s
}

// m n-bit vecs into k cluseters, m >> k
fn clustering(n: usize, k: usize, m: usize, max_iter: usize) -> (Vec<Vec<bool>>, u32){
    let samples = random_bvecs(m,n);
    let mut centers = random_bvecs(k, n);
    let mut clusters: Vec<Vec<usize>> = (0..k).map(|_| Vec::with_capacity(m/k)).collect();


    for trial in 0..max_iter{
        let (mut min_d, mut min_i) = (n as u32 ,n);
        for c in clusters.iter_mut(){ c.clear();}

        for (samp_i, samp) in samples.iter().enumerate() {
            for (cent_i, cent) in centers.iter().enumerate() {
                let d = dh(samp, cent);
                if d < min_d {
                    min_d = d;
                    min_i = cent_i;
//                    (min_d, min_i) = (d, cent_i);
                }
            }
            clusters[min_i].push(samp_i); //assign to closest centriod
        }


        for (clust_i, clust) in clusters.iter().enumerate(){
            let mut counts = vec![0; n];
            let clust_size = clust.len() as f32;
            for samp_i in clust.iter(){
                for (i, v) in samples[*samp_i].iter().enumerate() {
                    if *v {
                        counts[i] += 1;
                    }
                }
            }
            for (i, count) in counts.iter().enumerate() {
                centers[clust_i][i] =
                    if *count as f32 > clust_size/2.0 {true }
                        else { false }
            }
        }
    }
    let d = total_hd(&centers);
    (centers, d)
}



fn basic_multi_trial(n: usize, k: usize) -> (Vec<Vec<bool>>, u32){
    let trials = 1000;

    let mut max_s = 0;
    let mut max_x = Vec::new();

    for _ in 0..trials{


        let x = random_bvecs(k, n);

        let s = total_hd(&x);

        if s > max_s{
            max_x = x.clone();
            max_s = s;
        }
    }

    (max_x, max_s)
}

fn dh(v1: &Vec<bool>, v2: &Vec<bool>) -> u32{
    v1.iter().zip(v2.iter()).fold(0, |a, (b,c) | (*c!=*b) as u32 + a)
}
