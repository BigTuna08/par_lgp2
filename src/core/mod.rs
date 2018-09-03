use evo_sys::Program;

pub type GenoEval = Fn(&Program) -> f32 + 'static;
pub type RegIndType = u8;
pub type FeatIndType = u8;
pub type ClassType = bool;


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
