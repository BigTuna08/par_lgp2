use evo_sys::{ResultMap};
use data;
use ResultMapConfig;

impl ResultMap{
    pub fn new(config: ResultMapConfig) -> ResultMap{
        let mut prog_map = Vec::with_capacity(config.pop_size);
        for _ in 0..config.pop_size{
            prog_map.push(None);
        }
        ResultMap{
            prog_map,
            config,
            sent_count: 0,
            recieved_count: 0,
        }
    }


//    pub fn run()
}