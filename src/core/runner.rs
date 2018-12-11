use log::Logger;
use core::config::{CoreConfig, PopInfo, MapInfo, GenPopInfo, ConfigFile, Mode, ProgDefaults, ThreadDefaults};
use core::Runner;
use core::config::config::process_experiment_config;
use data;
use evo_sys::ResultMap;

impl Runner{

    pub fn new(loc: &str) -> Runner{
        let config = process_experiment_config(loc);
        let mode = config.mode.clone();
        Runner{
            config,
            mode,
            mutate_i: 0,
            compare_i: 0,
            started: false,
            vec_1_i: 0,
            vec_2_i: 0,
            vec_3_i: 0,
        }
    }


    pub fn run_all_configs(&mut self){
        while let Some(config) = self.next_config(){
            let mut logger = Logger::new(&config);
//            println!("\n\nBefore full tracking");
            logger.full_tracking();
//            println!("\n\nAfter full tracking");
            for trial_i in 0..config.n_iterations{
                let (test_data, training_data) = data::DataSet::new_pair("inputs/data3.csv");
                let mut pop = ResultMap::new(config.create_result_map_config());
                pop.run(training_data, test_data, &mut logger);
//                println!("** runall - finished iter ");
            }

//            experiments::multi_trial_five_fold_tracking(config);
        }
    }


    pub fn next_config(&mut self) -> Option<CoreConfig>{
        if !self.incr_inds(){
            return None
        }
        let data_file = self.config.data_file.clone();
        let compare_prog_method = self.config.compare_methods[self.compare_i];
        let mutate_method = self.config.mutate_methods[self.mutate_i];
        let n_iterations = self.config.n_iterations;

        let pop_config = match self.mode {
            Mode::Map => {
                PopInfo::Map(MapInfo {
                    select_cell_method: self.config.map_methods[self.vec_3_i],
                    initial_pop: self.config.inital_pop_size[self.vec_2_i],
                    n_evals:  self.config.n_evals[self.vec_1_i],
                })
            },
            Mode::Gen => {
                PopInfo::Gen(GenPopInfo {
                    tourn_size: self.config.tourn_sizes[self.vec_3_i],
                    total_gens: self.config.total_gens[self.vec_2_i],
                    random_gens:  self.config.random_gens[self.vec_1_i],
                })
            },
        };

        let out_folder =  match self.mode {//folder name is roughly mutate_compare_total_initial_other
            Mode::Map => format!("{}/{}_{}_{}_{}_{}",
                                 self.config.out_folder,
                                 self.config.mutate_methods[self.mutate_i],
                                 self.config.compare_methods[self.compare_i],
                                 self.config.n_evals[self.vec_1_i],
                                 self.config.inital_pop_size[self.vec_2_i],
                                 self.config.map_methods[self.vec_3_i],),

            Mode::Gen => format!("{}/{}_{}_{}_{}_{}",
                                 self.config.out_folder,
                                 self.config.mutate_methods[self.mutate_i],
                                 self.config.compare_methods[self.compare_i],
                                 self.config.random_gens[self.vec_1_i],
                                 self.config.total_gens[self.vec_2_i],
                                 self.config.tourn_sizes[self.vec_3_i],)
        };

        Some(CoreConfig{
            out_folder, data_file, compare_prog_method, mutate_method, pop_config, n_iterations, pop_size:self.config.pop_size})
    }


    pub fn print_dry_run(&mut self){
        let mut i =1;
        while let Some(config) = self.next_config(){
            println!("config #{} is {:?}", i, &config);
            i += 1;
        }
    }

    fn incr_inds(&mut self) -> bool{// true means continue
        if !self.started{
            self.started = true;
            return true
        }

        self.vec_3_i += 1;

        if self.vec_3_i >= self.config.v3_len(){
            self.vec_3_i = 0;
            self.vec_2_i += 1;
        }

        if self.vec_2_i >= self.config.v2_len() {
            self.vec_2_i = 0;
            self.vec_1_i += 1;
        }

        if self.vec_1_i >= self.config.v1_len() {
            self.vec_1_i = 0;
            self.compare_i += 1;
        }

        if self.compare_i >= self.config.compare_methods.len() {
            self.compare_i = 0;
            self.mutate_i += 1;
        }

        self.mutate_i < self.config.mutate_methods.len()
    }
}



impl ConfigFile{
    pub fn v1_len(&self)-> usize{
        match self.mode {
            Mode::Map => self.n_evals.len(),
            Mode::Gen => self.total_gens.len()
        }
    }

    pub fn v2_len(&self)-> usize{
        match self.mode {
            Mode::Map => self.inital_pop_size.len(),
            Mode::Gen => self.random_gens.len()
        }
    }

    pub fn v3_len(&self)-> usize{
        match self.mode {
            Mode::Map => self.map_methods.len(),
            Mode::Gen => self.tourn_sizes.len()
        }
    }
}