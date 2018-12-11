use data::trackers;
use data;
use data::params::N_FEATURES;
use evo_sys::eval::registers::PROG_REG;
use evo_sys::prog::ops;
use evo_sys::{ResultMap, ProgInspectRequest, GenPop, Program, Instruction};
use evo_sys::pop::{PopStats};
use core::config::CoreConfig;
////use evo_sys::prog::prog::Program;
////use evo_sys::pop::Population;
use std::fs::create_dir;

use std::fs::File;
use std::io::Write;
//
use core::config::config::get_log_freq;
//
use super::{FileSet, Logger};
use std::fs::create_dir_all;
use core::GenoEval;



impl FileSet{
    fn new(output_dir: &str) -> FileSet{
        create_dir_all(output_dir);
        let max = File::create(format!("{}/max.txt", output_dir)).unwrap();
        let min = File::create(format!("{}/min.txt", output_dir)).unwrap();
        let ave = File::create(format!("{}/ave.txt", output_dir)).unwrap();
        let sd = File::create(format!("{}/sd.txt", output_dir)).unwrap();
        FileSet{ max, min, ave, sd}
    }

    pub fn write(&mut self, stat: PopStats){
        self.max.write(stat.best.to_string().as_bytes());
        self.min.write(stat.worst.to_string().as_bytes());
        self.ave.write(stat.ave.to_string().as_bytes());
        self.sd.write(stat.sd.to_string().as_bytes());

        self.max.write(b"\t");
        self.min.write(b"\t");
        self.ave.write(b"\t");
        self.sd.write(b"\t");
    }

    pub fn write_new_line(&mut self){
        self.max.write(b"\n");
        self.min.write(b"\n");
        self.ave.write(b"\n");
        self.sd.write(b"\n");
    }

    pub fn flush(&mut self){
        self.max.flush();
        self.min.flush();
        self.ave.flush();
        self.sd.flush();
    }

}




impl Logger {
    pub fn new(config: &CoreConfig) -> Logger {
        println!("begin make logger ");

        let freq = get_log_freq();
        let root_dir = &config.out_folder;
        create_dir_all(format!("{}/genos", root_dir));
        create_dir(format!("{}/cv_fit_maps", root_dir));
        create_dir(format!("{}/test_fit_maps", root_dir));
        create_dir(format!("{}/eff_feat_maps", root_dir));
        create_dir(format!("{}/eff_len_maps", root_dir));

        println!("end make logger ");

        Logger {
            freq,
            root_dir: root_dir.to_string(),
            test_output_files: None,
            cv_output_files: None,
            geno_output_files: Vec::new(),
            geno_functions: Vec::new(),
            feature_count: None,
            feature_distr: None,
            current_iter: 0,
            data_headers: data::dataset::get_headers(&config.data_file),
        }
    }

    pub fn finish_run(&mut self, final_results: &ResultMap){
        println!("Beggining finish! ");

        let file_name = format!("iter{}.txt", self.current_iter);

        self.write_genos(&format!("{}/genos/{}", self.root_dir, file_name), final_results);
//        final_results.write_pop_info(&format!("{}/test_fit_maps/{}", self.root_dir, file_name), ProgInspectRequest::TestFit);
//        final_results.write_pop_info(&format!("{}/cv_fit_maps/{}", self.root_dir, file_name), ProgInspectRequest::CV);
//        final_results.write_pop_info(&format!("{}/eff_feat_maps/{}", self.root_dir, file_name), ProgInspectRequest::Geno(&trackers::get_eff_feats));
//        final_results.write_pop_info(&format!("{}/eff_len_maps/{}", self.root_dir, file_name), ProgInspectRequest::Geno(&trackers::get_eff_geno_len));

        println!("Before new line! ");
        self.new_line();
        self.current_iter += 1;
        println!("done finish! ");
    }


}


impl Logger{  // From map

    pub fn write_genos(&self, file_name: &str, res_map: &ResultMap) {
        match File::create(file_name) {
            Ok(mut f) => {
                for prog in res_map.prog_map_iter(){
                    if let &Some(ref genome) = prog {
                        f.write(b"\n");
                        self.write_genome_words(genome, &mut f);
                        f.write(b"\n");
        //                genome.write_effective_self_words(&mut f);
                        f.write(b"\n");
                        f.write(b"\n");
                    }
                }
            }
            Err(e) => {
                println!("error opening! file_name={}", file_name);
                println!("error is {:?}", e);
                panic!("");
            }
        }
    }




}


impl Logger{  // From prog

    fn write_genome_words(&self, genome: &Program, f: &mut File){

        let mut used_srcs = Vec::new();    //  <--  Make into hashmap with values?

        let info = format!("{:?}\ntraining: {}, \ntest: {}\n", genome.get_inds_simple(), &genome.test_fit.unwrap(), &genome.cv_fit.unwrap());
        f.write(b"\n\n## All instructions ## \n");
        f.write(info.as_bytes());
        f.write(b"\n");

//        println!("writting genome with sizes {}, {}, {}", &genome.header_instructions.len(), &genome.features.len(), &genome.instructions.len());
        for instr in genome.header_instructions.iter(){
            let instr_str = self.string_instr(instr, &mut used_srcs);
            f.write(instr_str.as_bytes());
            f.write(b"\n");
        }
        f.write(b"\n");

        for instr in genome.features.iter(){
            let instr_str = format!("{}->\t${}", instr.feat_i, instr.reg_i);
            f.write(instr_str.as_bytes());
            f.write(b"\n");
        }
        f.write(b"\n");


        for instr in genome.instructions.iter(){
            let instr_str = self.string_instr(instr, &mut used_srcs);
            f.write(instr_str.as_bytes());
            f.write(b"\n");
        }
        f.write(b"\n");


    }


//    pub fn write_self_words(&self, f: &mut File){
//        let mut used_srcs = Vec::new();    //  <--  Make into hashmap with values?
//        f.write(b"\n\n## All instructions ##\n");
//        for instr in self.instructions.iter(){
//            let instr_str = self.string_instr(instr, &mut used_srcs);
//            f.write(instr_str.as_bytes());
//            f.write(b"\n");
//        }
//        f.write(b"\n");
//    }
//
//
//    pub fn write_effective_self_words(&self, f: &mut File){
//        let mut used_srcs = Vec::new();
//        f.write(b"## Effective instructions ## \n");
//        for instr_i in self.get_effective_instrs_good(0){
//            let instr = self.instructions[instr_i];
//            let instr_str = self.string_instr(&instr, &mut used_srcs);
//            f.write(instr_str.as_bytes());
//            f.write(b"\n");
//        }
//        f.write(b"\n");
//    }


    pub fn string_instr(&self, instr: &Instruction, used: &mut Vec<u8>) -> String{

        let src1 = if used.contains(&instr.src1){ format!("${}",instr.src1) }
                        else { format!("{}",PROG_REG[instr.src1 as usize])  };


        let src2 = if used.contains(&instr.src2){format!("${}",instr.src2) }
                        else { format!("{}", PROG_REG[instr.src2 as usize])};

        used.push(instr.dest);  //  <--- should first check not branch!
        ops::formatted_string(instr, &src1, &src2)
    }
}

// for tracking
impl Logger{

    pub fn full_tracking(&mut self){
        self.track_both_fits();
        self.track_feats();

        self.add_geno_tracker("abs_len", &trackers::get_abs_geno_len);
        self.add_geno_tracker("eff_len", &trackers::get_eff_geno_len);
        self.add_geno_tracker("eff_feats", &trackers::get_eff_feats);
    }

    pub fn track_both_fits(&mut self) {
        self.track_test();
        self.track_cv();
    }


    pub fn track_test(&mut self){
        match self.test_output_files {
            Some(_) => panic!("Already tracking test!!!"),
            None => {
                let test_dir = format!("{}/test_fits", self.root_dir);
                create_dir(&test_dir);
                self.test_output_files = Some(FileSet::new(&test_dir));
            },
        }
    }

    pub fn track_cv(&mut self){
        match self.cv_output_files {
            Some(_) => panic!("Already tracking cv!!!"),
            None => {
                let cv_dir = format!("{}/cv_fits", self.root_dir);
                create_dir(&cv_dir);
                self.cv_output_files = Some(FileSet::new(&cv_dir));
            },
        }
    }


    pub fn track_feats(&mut self){
        match self.feature_count {
            Some(_) => panic!("Already tracking feats!!!!!"),
            None => {
                let feat_dir = format!("{}/feats", self.root_dir);
                create_dir(&feat_dir);
                self.feature_count = Some(File::create(&format!("{}/counts.txt", self.root_dir)).unwrap());
                match self.feature_distr {
                    Some(_) => panic!("Already tracking feats!!!!!"),
                    None =>{
                        let file_name = format!("iter{}.txt", self.current_iter);
                        self.feature_distr = Some(File::create(&format!("{}/{}", feat_dir, file_name)).unwrap());
                    },
                };
            },
        }
    }


    pub fn add_geno_tracker(&mut self, name: &str, geno_eval: &'static GenoEval){
        let out_dir = format!("{}/{}", self.root_dir, name);
        create_dir(&out_dir);
        self.geno_output_files.push(FileSet::new(&out_dir));
        self.geno_functions.push(geno_eval);
    }
}


// for writing
impl Logger{
    pub fn log_test_fits(&mut self, stats: PopStats){
        match self.test_output_files {
            Some(ref mut out_f) => out_f.write(stats),
            None => panic!("No test out file!!")
        };
    }

    pub fn log_cv_fits(&mut self, stats: PopStats){
        match self.cv_output_files {
            Some(ref mut out_f) => out_f.write(stats),
            None => panic!("No cv out file!!")
        };
    }

    pub fn log_feat_count(&mut self, count: u8) {
        match self.feature_count {
            Some(ref mut f) => {
                f.write(count.to_string().as_bytes());
                f.write(b"\t");
            },
            None => panic!("Not tracking feats!!!!!"),
        };
    }


    pub fn log_feat_distr(&mut self, distr: &[u16; N_FEATURES as usize]) {
        match self.feature_distr {
            Some(ref mut f) => {
                f.write(array_2_str(distr).as_bytes());
                f.write(b"\n");
            },
            None => panic!("Not tracking feats!!!!!"),
        };
    }

    pub fn log_geno_stat(&mut self, stats: PopStats, stat_ind: usize){
        self.geno_output_files[stat_ind].write(stats);
    }


    pub fn new_line(&mut self){
        match self.test_output_files {
            Some(ref mut out_f) => out_f.write_new_line(),
            None => (),
        };
        match self.cv_output_files {
            Some(ref mut out_f) => out_f.write_new_line(),
            None => (),
        };
        match self.feature_count{
            Some(ref mut f) => {f.write(b"\n");},
            None => (),
        };
        for f in self.geno_output_files.iter_mut(){
            f.write_new_line();
        }

    }


    pub fn flush(&mut self){
        match self.test_output_files {
            Some(ref mut out_f) => out_f.flush(),
            None => (),
        };
        match self.cv_output_files {
            Some(ref mut out_f) => out_f.flush(),
            None => (),
        };
        for f in self.geno_output_files.iter_mut(){
            f.flush();
        }
    }

}

pub fn array_2_str(arr: &[u16]) -> String{
    arr.iter().fold(String::new(), |acc, &x| format!("{}\t{}", acc, x.to_string()))
}

pub fn a_2_s(arr: &[u8]) -> String{
    let v: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
    v.join("\t")
}