extern crate par_lgp2;
use par_lgp2::core::Runner;
use par_lgp2::evo_sys::Program;
use par_lgp2::core::config::ProgDefaults;
use par_lgp2::core::config::config::process_prog_defaults;
//use par_lgp2;

fn main() {


//    let mut runner = par_lgp2::core::Runner::new("configs/experiment.txt");
//    println!("runner {:?}", runner);
//    runner.run_all_configs();



    test_compress();

}




fn test_compress(){
    let defs = process_prog_defaults("configs/prog_defaults.txt");
    let n = 5000;
    let (test_data, training_data) = par_lgp2::data::DataSet::new_pair("inputs/data3.csv");

    for i in 1..n {
        let p = Program::new_default(&defs);
        par_lgp2::evo_sys::eval::eval::eval_compress(&p, &test_data);
    }
}