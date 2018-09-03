extern crate par_lgp2;

//use par_lgp2;

fn main() {
    let data = par_lgp2::data::DataSet::new_pair("inputs/data3.csv");
    let config = par_lgp2::config::process_prog_defaults("configs/prog_defaults.txt");
    println!("Hello, world! {:?}", &config);
}
