
use evo_sys::*;
use data::DataSet;
use std::collections::{HashMap, HashSet};
use params;

pub struct DecompSet{
    pub branches: Vec<Instruction>,
    pub progs: Vec<Program>,
    pub original: Program,
}

pub struct SomeResult{
    correct: u16,
    n: u16,
    fpos: u16,
    fneg: u16,
    tpos: u16,
    tneg: u16,
}


impl SomeResult{
    pub fn new_blank() -> SomeResult {SomeResult{correct:0, n:0,fpos:0,fneg:0,tpos:0,tneg:0}}

    pub fn add(&mut self, other: &SomeResult){
        self.correct += other.correct;
        self.n += other.n;
        self.fpos += other.fpos;
        self.fneg += other.fneg;
        self.tpos += other.tpos;
        self.tneg += other.tneg;
    }

    pub fn to_string(&self) -> String {format!("{}\t{:1.4}\t{}\t{}\t{}\t{}\t{}",
                                        self.n , (self.correct as f32/ self.n as f32),
                                               self.correct, self.fpos, self.fneg,
                                               self.tpos, self.tneg )}

    pub fn prop_correct(&self)->f32 {return self.correct as f32 / self.n as f32 }
}

pub fn cond_eval_threash(ds: DecompSet, data: &DataSet, fitt: f32) {

    let paths = get_paths(&ds.original, data);
    let mut taken_paths = HashSet::new();
    for p in paths.iter(){
        taken_paths.insert(p.clone());
    }

    if taken_paths.len() > 4 {
//        println!("skipped- to many paths\t{}", taken_paths.len());
        return;
    }



    let mut res = eval_program_corrects(&ds.original, data, &paths);
//    let mut totals = SomeResult{correct:0, n:0,fpos:0,fneg:0,tpos:0,tneg:0};
    let totals =  res.remove("total").unwrap();

    if totals.prop_correct() > fitt {
        println!("for original prog with feats {:?}", &ds.original.features);
        for (k, v) in res.iter() {
            println!("{:8}\t{}", k, v.to_string());
//        totals.add(v);
        }

        println!("totals {:8}", totals.to_string());
    }

    for (i, prog) in ds.progs.iter().enumerate(){

        let mut res = eval_program_corrects(prog, data, &paths);
        let totals =  res.remove("total").unwrap();// SomeResult{correct:0, n:0,fpos:0,fneg:0,tpos:0,tneg:0};

        if totals.prop_correct() > fitt {

            println!("\nsub # {} len {:?}",i, prog.instructions.len());
            for (k,v) in res.iter(){
                println!("{:10}\t{}", k, v.to_string());
//            totals.add(v);
            }
            println!("{:10}\t{}","totals", totals.to_string() );
        }
//        else { println!("{:10}\t{}","totals", totals.to_string() ); }

    }

}



pub fn cond_eval(ds: DecompSet, data: &DataSet) {

   let paths = get_paths(&ds.original, data);

    println!("for original prog with feats {:?}", &ds.original.features);

    let res = eval_program_corrects(&ds.original, data, &paths);
    let mut totals = SomeResult{correct:0, n:0,fpos:0,fneg:0,tpos:0,tneg:0};
    for (k,v) in res.iter(){
        println!("{:8}\t{}", k, v.to_string());
        totals.add(v);
    }
    println!("totals {:8}",totals.to_string() );

    for (i, prog) in ds.progs.iter().enumerate(){
        println!("\nsub # {} len {:?}",i, prog.instructions.len());

        let res = eval_program_corrects(prog, data, &paths);
        let mut totals = SomeResult{correct:0, n:0,fpos:0,fneg:0,tpos:0,tneg:0};

        for (k,v) in res.iter(){
            println!("{:10}\t{}", k, v.to_string());
            totals.add(v);
        }
        println!("{:10}\t{}","totals", totals.to_string() );

    }

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




pub fn eval_program_corrects(genome: &Program, data: &DataSet, groups: &Vec<String>) -> HashMap<String, SomeResult> {


    let indetermine_score = 0.5;
    let compressed_prog = genome.create_compressed();

    let mut resmap: HashMap<String, SomeResult> = HashMap::new();


    for (record, key ) in data.records.iter().zip(groups.iter()) {


        let mut regs = eval::registers::PROG_REG.clone();


        for (i, feature) in genome.features.iter().enumerate() { //load features
            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]

        }

        let (prog_output, _) = run_instructions(&compressed_prog.instructions, &mut regs);

        let (correct, fpos, fneg, tpos, tneg) =
            if prog_output.abs() < params::params::EPS { // count zero as no prediction made
                (0,1,1,0,0)
            }
            else if prog_output.is_nan() { (0,1,1,0,0) } // garbage response, treat as wrong
            else {  // good prediction
                let classification_result = prog_output > 0.0;
                if classification_result == record.class {
                    if classification_result {(1, 0, 0, 1, 0)}
                    else {(1, 0, 0, 0, 1)}
                }
                else if classification_result {(0, 1, 0, 0, 0)}
                else if record.class {(0, 0, 1, 0, 0)}
                else { panic!("BAD") }
            };

        let  inserted =
            if let Some(mut v) = resmap.get_mut(key){
                v.n += 1;
                v.fneg += fneg;
                v.fpos += fpos;
                v.correct += correct;
                v.tneg += tneg;
                v.tpos += tpos;
                true
            }
            else {
                false
            };

        if !inserted{
            resmap.insert(key.clone(), SomeResult{
                n:1,
                correct,
                fpos,
                fneg,
                tpos,
                tneg,
            });
        }

//        total.add(resmap.get(key).unwrap());

    }
    let mut total = SomeResult::new_blank();
    for (k,v) in resmap.iter(){
        total.add(v);
    }
    resmap.insert(String::from("total"), total);
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



pub fn decompose(prog: &Program) -> DecompSet{
    let mut paths: Vec<Vec<Instruction>> = Vec::new();
    paths.push(Vec::new());
    let mut dead_paths = Vec::new();
    let mut branches = Vec::new();
    let mut last_branch = false;
    let compressed_prog = prog.create_compressed();

    for instr in compressed_prog.instructions.iter(){
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

    let mut progs =  Vec::new();
    for instructions in paths{
        let p = Program{
            features:prog.features.clone(),
            instructions,
            test_fit: None,
            cv_fit:None,
            n_calc_regs:0,
        };
        let p = p.create_compressed();
        let mut already_in = false;
        for pprog in progs.iter(){
            let sam: bool = p.same_instr(pprog);
            if sam {
                already_in = true;
                break;
            }
        }
        if !already_in {
            progs.push(p);
        }
    }

    DecompSet{branches, progs, original:compressed_prog}
}