pub mod prog;
pub mod pop;
pub mod params;
pub mod eval;

use std::collections::VecDeque;
use params as global_params;
use core::{RegIndType, FeatIndType};

use core::GenoEval;
use core::config::{ResultMapConfig, GenPopConfig};



//////      Program structs   ////

#[derive(Debug)]
pub struct Program{
    pub features: Vec<FeatLoadInfo>,  // (reg, feat_ind)
    pub header_instructions: Vec<Instruction>,
    pub instructions: Vec<Instruction>,
    pub test_fit: Option<f32>,
    pub cv_fit: Option<f32>,
}


pub struct ProgramFragment{
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct FeatLoadInfo{
    pub reg_i: RegIndType,
    pub feat_i: FeatIndType,
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction{
    pub dest: RegIndType,
    pub op: u8,
    pub src1: RegIndType,
    pub src2: RegIndType,
}

pub enum MutationMode{
    Normal,
    Copy,
    Del,
}

pub enum InstructionResult{
    Value(f32), // return floating point value
    Skip(u8), // return # of instructions to skip
    Terminate, // return message to terminate program
    NoOp, // result can be ignored
}

pub enum InstructionType{
    Value,
    Skip,
    Terminate,
    NoOp,
}

pub type ProgramOperation = Fn(&Instruction) -> InstructionResult;
pub type ExecutionRegArray = [f32; global_params::params::MAX_REGS];



////      Population structs   ////

pub struct ResultMap{
    prog_map: Vec<Option<Program>>,
    pub config: ResultMapConfig,
    sent_count: u64,
    pub recieved_count: u64,
    instr_frags: VecDeque<ProgramFragment>,
    feat_frags: VecDeque<FeatLoadInfo>,
}


pub struct GenPop{
    progs: Vec<Program>,
    config: GenPopConfig,
    current_gen: u32,
    current_gen_recived: usize,
    current_gen_sent: usize,
}


//
////
////#[derive(Debug)]
////pub struct PopConfig {
////    pub select_cell_method: u8,
////    pub compare_prog_method: u8,
////    pub initial_pop: u32,
////    pub total_evals: u64,
////}
//
//
//
//////      Other   ////
//
pub enum ProgInspectRequest<'a>{
    TestFit,
    CV,
    Geno(&'a GenoEval),
}
