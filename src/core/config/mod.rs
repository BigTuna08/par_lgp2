pub mod config;

#[derive(Debug)]
pub struct CoreConfig{
    pub pop_config: PopInfo,
    pub out_folder: String,
    pub data_file: String,
    pub compare_prog_method: u8,    // multi !
    pub mutate_method: u8,
    pub n_iterations: u32,
    pub pop_size: usize,
}

#[derive(Debug)]
pub enum PopInfo {
    Map(MapInfo),
    Gen(GenPopInfo),
}

#[derive(Debug)]
pub struct MapInfo {
    pub select_cell_method: u8,
    pub initial_pop: u32,
    pub n_evals: u64,
}

#[derive(Debug)]
pub struct GenPopInfo {
    pub tourn_size: u16,
    pub total_gens: u32,
    pub random_gens: u32,
}

#[derive(Debug)]
pub struct ResultMapConfig{
    pub out_folder: String,
    pub data_file: String,
    pub compare_prog_method: u8,    // multi !
    pub mutate_method: u8,
    pub n_iterations: u32,
    pub pop_size: usize,

    pub select_cell_method: u8,
    pub initial_pop: u32,
    pub n_evals: u64,
    pub prog_defaults: ProgDefaults,
}


#[derive(Debug)]
pub struct GenPopConfig{
    out_folder: String,
    data_file: String,
    compare_prog_method: u8,    // multi !
    mutate_method: u8,
    n_iterations: u32,
    pub pop_size: usize,

    tourn_size: u16,
    total_gens: u32,
    random_gens: u32,
    prog_defaults: ProgDefaults,
}

#[derive(Debug)]
pub struct ProgDefaults{
    pub initial_instr_min: usize,
    pub initial_instr_max: usize,

    pub initial_header_instr_min: usize,
    pub initial_header_instr_max: usize,

    pub initial_regs: u8,
    pub ops: Vec<u8>,

    pub initial_feat_min: u8,
    pub initial_feat_max: u8,
}



#[derive(Debug)]
pub struct ThreadDefaults{
    pub n_worker_threads: u8,
    pub worker_queue_size: u16,
    pub cap: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum Mode{
    Map,
    Gen,
}

#[derive(Debug)]
pub struct ConfigFile{
    pub mode: Mode,
    pub out_folder: String,
    pub data_file: String,
    pub n_iterations: u32,
    pub mutate_methods: Vec<u8>,
    pub compare_methods: Vec<u8>,
    pub pop_size: usize,

    pub n_evals: Vec<u64>,
    pub inital_pop_size: Vec<u32>,
    pub map_methods: Vec<u8>,

    pub total_gens: Vec<u32>,
    pub random_gens: Vec<u32>,
    pub tourn_sizes: Vec<u16>,
}