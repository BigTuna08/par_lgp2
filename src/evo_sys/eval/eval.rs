//use dataMgmt::{DataSet, ValidationSet};
//use params;
//use super::super::{Program, ExecutionRegArray, InstructionResult};
//use super::ops;

use std::fs::File;
use std::io::Write;
use std::{thread, time};
use log::Logger;

use params;
use evo_sys::prog::ops;
use evo_sys::{InstructionResult, Program, ExecutionRegArray, Instruction};
use data::DataSet;

//
//pub fn eval_program_corrects(genome: &Program, data: &DataSet) -> f32 {
//
//    let mut correct = 0.0f32;
//    let compressed_prog = genome.create_compressed();
//
//
//    for record in data.record_iter(){
//        let mut regs = super::registers::PROG_REG.clone();
//
//        for (i, feature) in genome.features.iter().enumerate() { //load features
//            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]
//
//        }
//
//        let prog_output = run_prog(&compressed_prog, &mut regs);
//        let indetermine_score = 0.5;
//
//
//        if prog_output.abs() < params::params::EPS { // count zero as no prediction made
//            correct += indetermine_score;
//        }
//            else if prog_output.is_nan() { } // garbage response, treat as wrong
//                else {  // good prediction
//                    let classification_result = prog_output > 0.0;
//                    if classification_result == record.class {correct += 1.0;}
//                }
//    }
//    correct as f32
//}
////
//pub fn eval_program_corrects_pos_neg(genome: &Program, data: &DataSet) -> (f32, u8, u8) {
//
//    let mut correct = 0.0f32;
//    let compressed_prog = genome.create_compressed();
//
//    let mut pos_missed = 0;
//    let mut neg_missed = 0;
//
//    println!("\n\n data rec size is: {:?} \n\n", &data.record_iter().len());
//
//    for record in data.record_iter(){
//        let mut regs = super::registers::PROG_REG.clone();
//
//        for (i, feature) in genome.features.iter().enumerate() { //load features
//            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]
//
//        }
//
//        let prog_output = run_prog(&compressed_prog, &mut regs);
//        let indetermine_score = 0.5;
//
//        if prog_output.abs() < params::params::EPS { // count zero as no prediction
//            correct += indetermine_score;
//            pos_missed += 1;
//            neg_missed += 1;
//        }
//            else if prog_output.is_nan() {
//                // garbage response, treat as wrong
//                pos_missed += 1;
//                neg_missed += 1;
//            }
//                else {  // good prediction
//                    let classification_result = prog_output > 0.0;
//                    if classification_result == record.class {
//                        if classification_result { // positive case
//                            correct += 1.0;
//                        }
//                            else {
//                                correct += 1.0;
//                            }
//                    }
//                        else {
//                            if classification_result { // positive case
//                                pos_missed += 1;
//                            }
//                                else {
//                                    neg_missed += 1;
//                                }
//                        }
//                }
//
//
//    }
//
//    (correct as f32, pos_missed, neg_missed)
//}
//
//
//
//
//pub fn run_prog(prog: &Program, regs: &mut ExecutionRegArray) -> f32 {
//
//    let mut skip_count = 0u8; // used to implement branches
//
//    for instr in prog.instructions.iter() {
//        if skip_count > 0 {
//            skip_count -= 1;
//            continue;
//        }
//
//        let result = ops::execute_op(instr, regs);
//        match result {
//            InstructionResult::Value(result) => regs[instr.dest as usize] = result,
//            InstructionResult::Skip(result) => skip_count = result,
//            InstructionResult::Terminate => break,
//            InstructionResult::NoOp => (),
//        }
//    }
//    regs[0]
//}
//
//
//
//pub fn eval_program_cv(genome: &Program, data: &DataSet) -> f32 {
//    let correct = eval_program_corrects(genome, data);
//    correct/ data.size() as f32
//}
//
//
//pub fn eval_program_corrects_testing_with_assert(genome: &Program, data: &DataSet) -> f32 {
//
//    let mut correct = 0.0f32;
//    let compressed_prog = genome.create_compressed();
//    let mut initial_regs = [0.0f32; params::params::MAX_REGS];
//
//
//    let mut reg_val = 0.1;
//    for reg in initial_regs.iter_mut() { //semi random initilize regs
//        *reg = reg_val;
//        reg_val = -(reg_val + 0.05);
//    }
//
//    for record in data.record_iter(){
//        let mut regs = initial_regs.clone();
//        let mut regs2 = initial_regs.clone();
//
//        for (i, feature) in genome.features.iter().enumerate() { //load features
//            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize];
//            regs2[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize];
//        }
//
//        let c_prog_output = run_prog(&compressed_prog, &mut regs);
//        let prog_output = run_prog(&genome, &mut regs2);
////        println!("reg={}\tcomp={}", prog_output, c_prog_output);
////        log_after_error(genome, &format!("err-{}-{}.txt", prog_output, c_prog_output));
//        if (prog_output - c_prog_output).abs() > params::params::EPS{
//            println!("logging bad!");
//            log_after_error(genome, &format!("err-{}-{}.txt", prog_output, c_prog_output));
//
////            let ten_millis = time::Duration::from_millis(10);
////            thread::sleep(ten_millis);
//            panic!("bad excision by {}", (prog_output-c_prog_output).abs());
//        }
////        assert_eq!(prog_output, c_prog_output);
//
////        let classification_result = c_prog_output >= 0.0;
////        if classification_result == record.class {correct += 1.0;}
//    }
//    correct as f32
//}
//
//
//pub fn log_after_error(genome: &Program, file_name: &str){
//    let mut f = File::create(file_name).unwrap();
//    genome.write_header(&mut f);
//    f.write(b"\n");
//    genome.write_self_words(&mut f);
//    f.write(b"\n");
//    genome.write_effective_self_words(&mut f);
//    f.write(b"\n");
//    f.write(b"\n");
//}





//pub fn eval_program_corrects(genome: &Program, data: &DataSet) -> f32 {
//
//    let mut correct = 0.0f32;
//    let compressed_prog = genome.create_compressed();
//
//
//    for record in data.record_iter(){
//        let mut regs = super::registers::PROG_REG.clone();
//
//        for (i, feature) in genome.features.iter().enumerate() { //load features
//            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]
//
//        }
//
//        let prog_output = run_prog(&compressed_prog, &mut regs);
//        let indetermine_score = 0.5;
//
//
//        if prog_output.abs() < params::params::EPS { // count zero as no prediction made
//            correct += indetermine_score;
//        }
//            else if prog_output.is_nan() { } // garbage response, treat as wrong
//                else {  // good prediction
//                    let classification_result = prog_output > 0.0;
//                    if classification_result == record.class {correct += 1.0;}
//                }
//    }
//    correct as f32
//}



//use params;
//use evo_sys::prog::ops;
//use evo_sys::{InstructionResult, Program, ExecutionRegArray, Instruction};
//use data::DataSet;
//
//
//


pub fn eval_program_vec(genome: &Program, data: &DataSet) -> Vec<bool> {

    let mut correct = 0.0f32;
    let indetermine_score = 0.5;
    let compressed_prog = genome.create_compressed();

    let mut result_vec = Vec::with_capacity(data.records.len());

    for record in data.records.iter() {

        let mut regs = super::registers::PROG_REG.clone();


        for (i, feature) in genome.features.iter().enumerate() { //load features
            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]

        }

        let prog_output = run_instructions(&compressed_prog.instructions, &mut regs);

        let correct =
            if prog_output.abs() < params::params::EPS { // count zero as no prediction made
                false
            }
            else if prog_output.is_nan() { false} // garbage response, treat as wrong
            else {  // good prediction
                let classification_result = prog_output > 0.0;
                classification_result == record.class
            };
        result_vec.push(correct);
    }
    result_vec
}



pub fn eval_program_corrects(genome: &Program, data: &DataSet) -> f32 {

    let mut correct = 0.0f32;
    let indetermine_score = 0.5;
    let compressed_prog = genome.create_compressed();

//    let mut initial_regs = super::registers::PROG_REG.clone();
//
//    {
//        run_instructions(&genome.header_instructions, &mut initial_regs);  // initialize regs once
//    }


    for record in data.records.iter() {

        let mut regs = super::registers::PROG_REG.clone();


        for (i, feature) in genome.features.iter().enumerate() { //load features
            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]

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






pub fn eval_compress(genome: &Program, data: &DataSet) {

    let mut correct2 = 0.0f32;
    let mut correctt = 0.0f32;
    let indetermine_score = 0.5;
    let cp2 = genome.create_compressed();




    for record in data.records.iter() {

        let mut regs = super::registers::PROG_REG.clone();


        for (i, feature) in genome.features.iter().enumerate() { //load features
            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]

        }

        let prog_output = run_instructions(&cp2.instructions, &mut regs);

        if prog_output.abs() < params::params::EPS { // count zero as no prediction made
            correct2 += indetermine_score;
        }
            else if prog_output.is_nan() { } // garbage response, treat as wrong
                else {  // good prediction
                    let classification_result = prog_output > 0.0;
                    if classification_result == record.class {correct2 += 1.0;}
                }
    }


    for record in data.records.iter() {

        let mut regs = super::registers::PROG_REG.clone();


        for (i, feature) in genome.features.iter().enumerate() { //load features
            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]

        }

        let prog_output = run_instructions(&genome.instructions, &mut regs);

        if prog_output.abs() < params::params::EPS { // count zero as no prediction made
            correctt += indetermine_score;
        }
            else if prog_output.is_nan() { } // garbage response, treat as wrong
                else {  // good prediction
                    let classification_result = prog_output > 0.0;
                    if classification_result == record.class {correctt += 1.0;}
                }
    }



    println!("compressed: {} \t\t true: {}\n\ncompressed", correct2, correctt);


    let mut used_srcs = Vec::new();    //  <--  for printing


    for instr in cp2.instructions.iter(){
        let instr_str = Logger::string_instr(instr, &mut used_srcs, &cp2.features);
        println!("{}", instr_str);
    }

    println!("\n\ntrue");

    used_srcs = Vec::new();
    for instr in genome.instructions.iter(){
        let instr_str = Logger::string_instr(instr, &mut used_srcs, &cp2.features);
        println!("{}", instr_str);
    }
    println!("*********");

    assert_eq!(correctt, correct2);


}


//pub fn eval_compress(genome: &Program, data: &DataSet) -> f32 {
//
//    let mut correct = 0.0f32;
//    let mut correct2 = 0.0f32;
//    let mut correctt = 0.0f32;
//    let indetermine_score = 0.5;
//    let compressed_prog = genome.create_compressed();
//    let cp2 = genome.compress();
//
//
//
//    for record in data.records.iter() {
//
//        let mut regs = super::registers::PROG_REG.clone();
//
//
//        for (i, feature) in genome.features.iter().enumerate() { //load features
//            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]
//
//        }
//
//        let prog_output = run_instructions(&compressed_prog.instructions, &mut regs);
//
//        if prog_output.abs() < params::params::EPS { // count zero as no prediction made
//            correct += indetermine_score;
//        }
//        else if prog_output.is_nan() { } // garbage response, treat as wrong
//        else {  // good prediction
//            let classification_result = prog_output > 0.0;
//            if classification_result == record.class {correct += 1.0;}
//        }
//    }
//
//
//    for record in data.records.iter() {
//
//        let mut regs = super::registers::PROG_REG.clone();
//
//
//        for (i, feature) in genome.features.iter().enumerate() { //load features
//            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]
//
//        }
//
//        let prog_output = run_instructions(&cp2.instructions, &mut regs);
//
//        if prog_output.abs() < params::params::EPS { // count zero as no prediction made
//            correct2 += indetermine_score;
//        }
//        else if prog_output.is_nan() { } // garbage response, treat as wrong
//        else {  // good prediction
//            let classification_result = prog_output > 0.0;
//            if classification_result == record.class {correct2 += 1.0;}
//        }
//    }
//
//
//    for record in data.records.iter() {
//
//        let mut regs = super::registers::PROG_REG.clone();
//
//
//        for (i, feature) in genome.features.iter().enumerate() { //load features
//            regs[params::params::MAX_REGS - 1 - i] = record.features[*feature as usize]
//
//        }
//
//        let prog_output = run_instructions(&genome.instructions, &mut regs);
//
//        if prog_output.abs() < params::params::EPS { // count zero as no prediction made
//            correctt += indetermine_score;
//        }
//            else if prog_output.is_nan() { } // garbage response, treat as wrong
//                else {  // good prediction
//                    let classification_result = prog_output > 0.0;
//                    if classification_result == record.class {correctt += 1.0;}
//                }
//    }
//
//
//
//    println!("1: {} \t\t 2: {} \t\t true: {}\n\n1", correct, correct2, correctt);
//
//
//    let mut used_srcs = Vec::new();    //  <--  Make into hashmap with values?
//
//
//    for instr in genome.instructions.iter(){
//        let instr_str = Logger::string_instr_shoulduse(instr, &mut used_srcs);
//        println!("{}", instr_str);
//    }
//
//    println!("\n\n2");
//
//    used_srcs = Vec::new();
//    for instr in compressed_prog.instructions.iter(){
//        let instr_str = Logger::string_instr_shoulduse(instr, &mut used_srcs);
//        println!("{}", instr_str);
//    }
//
//    println!("\n\n3");
//
//    used_srcs = Vec::new();
//    for instr in cp2.instructions.iter(){
//        let instr_str = Logger::string_instr_shoulduse(instr, &mut used_srcs);
//        println!("{}", instr_str);
//    }
//    println!("*********");
//
//    assert_eq!(correct, correct2);
//
//
//    correct / data.size() as f32
//}