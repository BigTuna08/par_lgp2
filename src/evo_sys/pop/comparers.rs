use super::super::{ResultMap, Program};
use rand;
use rand::Rng;
//
//use data;
//
//use super::VarPenConfig;


impl ResultMap{

    pub fn compare(&self, new_prog: &Program, old_prog: &Program) -> bool {
        match self.config.compare_prog_method {
            0 => self.simple_tie_shortest(new_prog, old_prog),
            1 => self.simple_tie_rand(new_prog, old_prog),
//            2 => self.pen_small(new_prog, old_prog),
            _ => panic!("Invalid compare method!! \n{:?}", self.config),
        }
    }


    fn simple_tie_shortest(&self, new_prog: &Program, old_prog: &Program) -> bool{
        if new_prog.test_fit.unwrap() == old_prog.test_fit.unwrap(){
            if new_prog.get_effective_len(0) == old_prog.get_effective_len(0){
                return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
            }
            else {
                return new_prog.get_effective_len(0) < old_prog.get_effective_len(0)
            }
        }
        else {
            return new_prog.test_fit.unwrap() > old_prog.test_fit.unwrap()
        }
    }


    fn simple_tie_rand(&self, new_prog: &Program, old_prog: &Program) -> bool{
        if new_prog.test_fit.unwrap() == old_prog.test_fit.unwrap(){
            return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
        }
        else {
            return new_prog.test_fit.unwrap() > old_prog.test_fit.unwrap()
        }
    }


//    fn pen_small(&self, new_prog: &Program, old_prog: &Program) -> bool{
//
//        let v = 1.0 / dataMgmt::params::N_SAMPLES as f32;
//
//        let new = new_prog.test_fit.unwrap() - v*new_prog.get_effective_len(0) as f32;
//        let old = old_prog.test_fit.unwrap() - v*old_prog.get_effective_len(0) as f32;
//
//        if new == old {
//            return rand::thread_rng().gen_weighted_bool(super::super::params::REPLACE_EQ_FIT);
//        }
//        return new > old
//    }

}