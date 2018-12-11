use params;
use evo_sys::prog::ops;
use evo_sys::{InstructionResult, Program, ExecutionRegArray, Instruction, FeatLoadInfo};
use data::DataSet;



pub fn eval_program_corrects(genome: &Program, data: &DataSet) -> f32 {

    let mut correct = 0.0f32;
    let indetermine_score = 0.5;
    let compressed_prog = genome.create_compressed();

    let mut initial_regs = super::registers::PROG_REG.clone();

    {
        run_instructions(&genome.header_instructions, &mut initial_regs);  // initialize regs once
    }


    for record in data.records.iter() {

        let mut regs = initial_regs.clone();


        for &FeatLoadInfo{reg_i, feat_i} in genome.features.iter() { //load features
            regs[reg_i as usize] = record.features[feat_i as usize]
        }

        let prog_output = run_instructions(&compressed_prog.instructions, &mut regs);

        if prog_output.abs() < params::params::EPS { // count zero as no prediction made
            correct += indetermine_score;
        }
        else if prog_output.is_nan() { } // garbage response, treat as wrong
        else {  // good prediction
            let classification_result = prog_output > 0.0;
            if classification_result == record.class {correct += 1.0;}
        }
    }
    correct / data.size() as f32
}


pub fn run_instructions(instrs: &Vec<Instruction>, regs: &mut ExecutionRegArray) -> f32 {
    let mut skip_count = 0u8; // used to implement branches

    for instr in instrs.iter() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        let result = ops::execute_op(instr, regs);
        match result {
            InstructionResult::Value(result) => regs[instr.dest as usize] = result,
            InstructionResult::Skip(result) => skip_count = result,
            InstructionResult::Terminate => break,
            InstructionResult::NoOp => (),
        }
    }
    regs[0]
}