extern crate par_lgp2;

//use par_lgp2;

fn main() {
//    let data = par_lgp2::data::DataSet::new_pair("inputs/data3.csv");
//    let config = par_lgp2::core::config::config::process_prog_defaults("configs/prog_defaults.txt");
//    let runner = par_lgp2::core::Runner::new("configs/experiment.txt");

    let mut runner = par_lgp2::core::Runner::new("configs/experiment.txt");
    println!("runner {:?}", runner);
    runner.run_all_configs();

//    par_lgp2::evo_sys::prog::mutation::test_rand();
//    println!("Hello, world! {:?}", &config);
}
