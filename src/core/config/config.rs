use std::fs::File;
use std::io::prelude::*;
use core::config;
use core::config::*;
use std;


use std::collections::HashMap;

pub struct ConfigManager{
    info: HashMap<String, Vec<String>>
}

impl ConfigManager{
    pub fn new(loc: &str) -> ConfigManager{
        let mut info = HashMap::new();
        println!("opening {}", loc);

        let mut f = File::open(loc).expect("error oping file!");
        let mut c = String::new();
        f.read_to_string(&mut c);

        for line in c.lines(){

            let mut parts = line.split_whitespace();
            let first = parts.next();
            match first {
                None => (),
                Some(text) if text.starts_with("//") => (), // comment -> do nothing
                Some(text) => {
                    info.insert(String::from(text), get_string_vect(&mut parts ));
                },
//                _ => (),
            }
        }

        ConfigManager{
            info
        }
    }


    pub fn get_single_value<T: std::str::FromStr>(&self, key: &str) -> T {
        match self.info.get(key) {
            Some(val_vec) =>{
                match val_vec.len() {
                    0 => panic!("No values given with key={}", key),
                    _ => match val_vec[0].parse::<T>() {
                        Ok(val) => val,
                        Err(_e) => panic!("error parsing configs with key={}", key),
                    }
                }
            },
            None => panic!("not in config file! key={}", key),
        }
    }


    pub fn get_value_vec<T: std::str::FromStr>(&self, key: &str) -> Vec<T> {
        let mut list = Vec::new();

        match self.info.get(key) {
            Some(val_vec) =>{
                for val in val_vec{
                    match val.parse::<T>() {
                        Ok(val) => list.push(val),
                        Err(e) => panic!("error parsing configs with key={}", key),
                    }

                }
            },
            None => panic!("not in config file! key={}", key),
        }
        list
    }
}







pub fn process_thread_defaults(loc: &str) -> ThreadDefaults {
    let info = ConfigManager::new(loc);
    ThreadDefaults{
        n_worker_threads: info.get_single_value("N_WORKER_THREADS:"),
        worker_queue_size: info.get_single_value("WORKER_QUEUE_SIZE:"),
        cap: info.get_single_value("WORKER_QUEUE_SIZE:"),
    }
}


pub fn process_prog_defaults(loc: &str) -> ProgDefaults {
    let info = ConfigManager::new(loc);
    ProgDefaults{
        initial_instr_min: info.get_single_value("INITIAL_INSTR_MIN:"),
        initial_instr_max: info.get_single_value("INITIAL_INSTR_MAX:"),
        initial_comp_reg_min: info.get_single_value("INITIAL_COMP_REG_MIN:"),
        initial_comp_reg_max: info.get_single_value("INITIAL_COMP_REG_MAX:"),

        initial_feat_min: info.get_single_value("INITIAL_FEAT_MIN:"),
        initial_feat_max: info.get_single_value("INITIAL_FEAT_MAX:"),
    }
}


pub fn get_log_freq() -> u32 {
    let info = ConfigManager::new("configs/experiment.txt");
    info.get_single_value("LOG_FREQ:")
}


pub fn process_experiment_config(loc: &str) -> ConfigFile{
    let info = ConfigManager::new(loc);

    ConfigFile {
        mode: str_to_mode(&info.get_single_value::<String>("MODE:")) ,
        data_file: info.get_single_value("DATA_FILE:"),
        out_folder: format!("results/{}", info.get_single_value::<String>("OUT_FOLDER:")),
        n_iterations: info.get_single_value("N_ITERATIONS:"),
        mutate_methods: info.get_value_vec("MUTATION_METHODS:"),
        compare_methods: info.get_value_vec("COMPARE_METHODS:"),
        pop_size: info.get_single_value("POPULATION_SIZE:"),


        n_evals: info.get_value_vec("N_EVALS:"),
        inital_pop_size: info.get_value_vec("INITIAL_POP_SIZES:"),
        map_methods: info.get_value_vec("MAP_METHODS:"),

        total_gens: info.get_value_vec("TOTAL_GENS:"),
        random_gens: info.get_value_vec("INIT_GENS:"),
        tourn_sizes: info.get_value_vec("TOURN_SIZE:"),
    }
}


fn str_to_mode(s: &str)->Mode{
    if s.eq_ignore_ascii_case("MAP"){
        return Mode::Map
    }
        else if s.eq_ignore_ascii_case("GEN"){
            return Mode::Gen
        }
    panic!("Error reading mode!!")
}


fn get_string_vect(parts: &mut std::str::SplitWhitespace) -> Vec<String>{
    let mut list = Vec::new();
    while let Some(text) = parts.next() {
        if text.starts_with("//") {
            break;
        }
        list.push(String::from(text));
    }
    list
}


impl CoreConfig{
    pub fn create_result_map_config(&self) -> ResultMapConfig{
        match self.pop_config {
            PopInfo::Map(ref info) => {
                ResultMapConfig{
                    out_folder: self.out_folder.clone(),
                    data_file: self.data_file.clone(),
                    compare_prog_method: self.compare_prog_method,
                    mutate_method: self.mutate_method,
                    n_iterations: self.n_iterations,
                    pop_size: self.pop_size,

                    select_cell_method: info.select_cell_method,
                    initial_pop: info.initial_pop,
                    n_evals: info.n_evals,
                    prog_defaults: process_prog_defaults("configs/prog_defaults.txt")
                }
            },
            PopInfo::Gen(_) => panic!("Not in gen mode!!")
        }
    }

    pub fn create_gen_pop_config(&self) -> GenPopConfig{
        match self.pop_config {
            PopInfo::Map(_) => panic!("Not in map mode!!"),
            PopInfo::Gen(ref info) => {
                GenPopConfig{
                    out_folder: self.out_folder.clone(),
                    data_file: self.data_file.clone(),
                    compare_prog_method: self.compare_prog_method,
                    mutate_method: self.mutate_method,
                    n_iterations: self.n_iterations,
                    pop_size:self.pop_size,

                    tourn_size: info.tourn_size,
                    total_gens: info.total_gens,
                    random_gens: info.random_gens,
                    prog_defaults: process_prog_defaults("configs/prog_defaults.txt")
                }
            }
        }
    }
}