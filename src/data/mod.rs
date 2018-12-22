pub mod dataset;
pub mod trackers; //functions for tracking info about programs during evolution
pub mod params;
pub mod metabolites;

use core::{ClassType};


#[derive(Copy, Clone)]
pub struct DataRecord{
    pub features: [f32; params::N_FEATURES as usize],
    pub class: ClassType,
}


pub struct DataSet{
    pub records: Vec<DataRecord>,
}


