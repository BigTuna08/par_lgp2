pub mod runner;
pub mod config;

use evo_sys::Program;

pub type GenoEval = Fn(&Program) -> f32 + 'static;
pub type RegIndType = u8;
pub type FeatIndType = u8;
pub type ClassType = bool;


#[derive(Debug)]
pub struct Runner{
    config: config::ConfigFile,
    mode: config::Mode,
    mutate_i: usize,
    compare_i: usize,

    started: bool,

    //vec 1..3 have different meanings based on mode
    vec_1_i: usize,  //index of n_evals or total_gens
    vec_2_i: usize, //index of inital_pop_size or init_gens
    vec_3_i: usize, //index of map_methods or tourn_size

}


#[derive(Debug)]
pub enum Message {
    Cont(Program),
    Quit,
}


pub struct EvalResult{
    pub prog: Program
}

impl EvalResult{
    pub fn new(prog: Program)-> EvalResult{
        EvalResult{prog}
    }
}
