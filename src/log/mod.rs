pub mod logger;


use core::GenoEval;
//use evo_sys::Program;
use std::fs::File;


//////      Logger structs   ////

pub struct Logger{
    pub freq: u32,
    pub root_dir: String,

    test_output_files: Option<FileSet>,
    cv_output_files: Option<FileSet>,
    geno_output_files: Vec<FileSet>,

    pub geno_functions: Vec<&'static GenoEval>,
    pub data_headers: Vec<String>,

    feature_count: Option<File>,
    feature_distr: Option<File>,

    current_iter: u16,

}

struct FileSet{
    max: File,
    min: File,
    ave: File,
    sd: File,
}
