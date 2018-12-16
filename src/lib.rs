extern crate csv;
extern crate rand;

//#[macro_use]
//extern crate serde_derive;
//extern crate serde;
//extern crate serde_json;

pub mod params;
pub mod evo_sys;
pub mod data;
pub mod log;
pub mod core;
pub mod anal;

use std::fs::File;
use std::io::Read;

pub fn load_progs(loc: &str) -> Vec<evo_sys::Program> {
    let mut progs = Vec::new();
    let mut f = File::open(loc).expect("error oping file!");
    let mut c = String::new();
    f.read_to_string(&mut c);

    for prog_line in c.lines(){
        let mut parts = prog_line.split("|");

        let mut features = Vec::new();
        if let Some(p) = parts.next(){
            for x in p.split(" "){
                if let Ok(x) =  x.parse::<u8>(){
                    features.push(x)
                }
            }
        }


        let mut instructions = Vec::new();
        if let Some(p) = parts.next(){
            for x in p.split("\t"){
                if x.len() > 0 {
                    instructions.push(evo_sys::Instruction::from_load_string(x))
                }
            }
        }


//        let features = parts.next().unwrap().split(" ").map(|x| x.parse::<u8>().unwrap()).collect();
//
//        let instructions = parts.next().unwrap().split("\t").map(|x| evo_sys::Instruction::from_load_string(x)).collect();

        progs.push(evo_sys::Program{
                                    features,
                                    instructions,
                                    test_fit: None,
                                    cv_fit:None,
                                    n_calc_regs:0,
                                })
    }

    progs
}