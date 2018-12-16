extern crate par_lgp2;
use par_lgp2::core::Runner;
use par_lgp2::evo_sys::Program;
use par_lgp2::core::config::ProgDefaults;
use par_lgp2::core::config::config::process_prog_defaults;
//use par_lgp2;

fn main() {


    let mut runner = par_lgp2::core::Runner::new("configs/experiment.txt");
    println!("runner {:?}", runner);
//    runner.run_all_configs();


    let loc = "results/new/0_1_50_50_0/saved_genos.txt";//format!("{:?}/saved_genos.txt", runner.config.out_folder);
    let progs = par_lgp2::load_progs(&loc);

    let cd = par_lgp2::anal::decompose(&progs[8]);

    println!("{:?}", &cd.branches);
    println!("{:?}", &cd.progs.len());
    for p in cd.progs.iter(){
        for i in p.instructions.iter(){
            println!("{:?}", i)
        }
        println!("\n\n**");
    }

//    for p in progs.iter(){
//        for i in p.instructions.iter(){
//            println!("{:?}", i)
//        }
//        println!("finished prog len {}\n", p.instructions.len());
//    }
//    for i in progs[8].instructions.iter(){
//        println!("{:?}", i)
//    }
//    println!("{:?}",progs[8])

//    test_compress();

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