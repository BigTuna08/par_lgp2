
use evo_sys::*;
use data::DataSet;
use std::collections::HashMap;
use params;

pub struct DecompSet{
    pub branches: Vec<Instruction>,
    pub progs: Vec<Program>,
    pub original: Program,
//    pub sample_paths: Vec<String>,
}

pub struct SomeResult{
    correct: u16,
    n: u16,
    fpos: u16,
    fneg: u16,
}

pub enum Path{
    T,
    F,
    S,  //skipped
}

pub const N_RES_FIELDS: usize = 4;
pub struct CondEvalResult{
    uc: [f32; N_RES_FIELDS],                   // unconditional
    single: Vec<[f32; N_RES_FIELDS]>,          // single condition
    mulit: Option<Vec<[f32; N_RES_FIELDS]>>,   // all conditions
}

pub fn cond_eval(ds: DecompSet, data: &DataSet) {

   let paths = get_paths(&ds.progs[0], data);
}


pub fn decompose(prog: &Program) -> DecompSet{
    let mut paths: Vec<Vec<Instruction>> = Vec::new();
    paths.push(Vec::new());
    let mut dead_paths = Vec::new();
    let mut branches = Vec::new();
    let mut last_branch = false;

    for instr in prog.instructions.iter(){
        match prog::ops::get_type(&instr) {
            InstructionType::Skip => {
                branches.push(instr.clone());
                last_branch = true;
            }
            InstructionType::Terminate => {
                if last_branch{
                    for path in paths.iter(){
                        dead_paths.push(path.clone());
                    }
                }
            }
            InstructionType::Value => {
                if last_branch {
                    let mut new_paths = Vec::new();
                    for path in paths.iter_mut() {
                        new_paths.push(path.clone()); // not skipped
                        path.push(instr.clone())  // instr skipped
                    }
                    paths.append(&mut new_paths);
                }
                else {
                    for path in paths.iter_mut() {
                        path.push(instr.clone())
                    }
                }
            }
            InstructionType::NoOp => panic!("should remove noop in compress!!")
        }
    }

    let mut progs = Vec::new();
    for instructions in paths{
        progs.push(Program{
            features:Vec::new(),
            instructions,
            test_fit: None,
            cv_fit:None,
            n_calc_regs:0,
        })
    }

    DecompSet{branches, progs, original:*prog}
}




pub fn get_paths(genome: &Program, data: &DataSet) -> Vec<String> {

    let indetermine_score = 0.5;
    let compressed_prog = genome.create_compressed();

    let mut paths = Vec::new();

    for record in data.records.iter() {

        let mut regs = eval::registers::PROG_REG.clone();

        for (i, feature) in genome.features.iter().enumerate() { //load features
            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]

        }

        let (prog_output, key) = run_instructions(&compressed_prog.instructions, &mut regs);

        paths.push(key)

    }
    paths
}




pub fn eval_program_corrects(genome: &Program, data: &DataSet) -> HashMap<String, SomeResult> {


    let indetermine_score = 0.5;
    let compressed_prog = genome.create_compressed();

    let mut resmap: HashMap<String, SomeResult> = HashMap::new();

    for record in data.records.iter() {


//        let mut results = [Path::S; 4];


        let mut regs = eval::registers::PROG_REG.clone();


        for (i, feature) in genome.features.iter().enumerate() { //load features
            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]

        }

        let (prog_output, key) = run_instructions(&compressed_prog.instructions, &mut regs);

        let (correct, fpos, fneg) =
            if prog_output.abs() < params::params::EPS { // count zero as no prediction made
                (0,1,1)
            }
            else if prog_output.is_nan() { (0,1,1) } // garbage response, treat as wrong
            else {  // good prediction
                let classification_result = prog_output > 0.0;
                if classification_result == record.class {(1,0,0)}
                else if classification_result {(0, 1, 0)}
                else if record.class {(0, 0, 1)}
                else { panic!("BAD") }
            };

        let  inserted =
            if let Some(mut v) = resmap.get_mut(&key){
                v.n += 1;
                v.fneg += fneg;
                v.fpos += fpos;
                v.correct += correct;
                true
            }
            else {
                false
            };

        if !inserted{
            resmap.insert(key, SomeResult{
                n:1,
                correct,
                fpos,
                fneg,
            });
        }

    }
    resmap
}


pub fn run_instructions(instrs: &Vec<Instruction>, regs: &mut ExecutionRegArray) -> (f32, String) {
    let mut skip_count = 0u8; // used to implement branches
    let mut path = String::new();

    for instr in instrs.iter() {


        if skip_count > 0 {
            if let InstructionType::Skip = prog::ops::get_type(&instr) {
                path = format!("{}S", path);
            }
            skip_count -= 1;
            continue;
        }

        let result = prog::ops::execute_op(instr, regs);
        match result {
            InstructionResult::Value(result) => regs[instr.dest as usize] = result,
            InstructionResult::Skip(result) => {
                match result {
                    0 => path = format!("{}F", path),
                    1 => path = format!("{}T", path),
                    _ => panic!("should be no multi skips!")
                }
                skip_count = result
            },
            InstructionResult::Terminate => break,
            InstructionResult::NoOp => (),
        }
    }
    (regs[0], path)
}


