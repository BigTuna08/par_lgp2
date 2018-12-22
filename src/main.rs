extern crate par_lgp2;
use par_lgp2::core::Runner;
use par_lgp2::evo_sys::Program;
use par_lgp2::core::config::ProgDefaults;
use par_lgp2::core::config::config::process_prog_defaults;
//use par_lgp2;

fn main() {


    let mut runner = par_lgp2::core::Runner::new("configs/experiment.txt");
    println!("runner {:?}", runner);
    runner.run_all_configs();


//    print_infos();

//    let cd = par_lgp2::anal::decompose(&progs[38]);
//
//    println!("{:?}", &cd.branches);
//    println!("{:?}", &cd.progs.len());
//    for p in cd.progs.iter(){
//        for i in p.instructions.iter(){
//            println!("{:?}", i)
//        }
//        println!("\n\n**");
//    }
//
//    println!("******\n\n\n");
//
//    let (test_data, training_data) = par_lgp2::data::DataSet::new_pair("inputs/data3.csv");
//
//    par_lgp2::anal::cond_eval(cd, &test_data);
//    test_compress();
}


fn print_infos(){
    let loc =  "results/info/0_1_500000_50000_0/saved_genos.txt";//"results/info/0_1_250000_25000_0/saved_genos.txt";


    let progs = par_lgp2::load_progs(&loc);



    let (test_data, training_data) = par_lgp2::data::DataSet::new_pair("inputs/data3.csv");

    for (i, prog) in progs.iter().enumerate(){
        let cd = par_lgp2::anal::decompose(prog);



        if cd.progs.len() > 4 {

//            println!("Skipping, len is {}", cd.progs.len());
            continue
        }

        println!("id# {:?}", i);

        par_lgp2::anal::cond_eval_threash(cd, &test_data, 0.85);

//        println!("******\n\n");
    }

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