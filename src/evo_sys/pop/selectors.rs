use evo_sys::{ResultMap, Program};
//use params::params::{MAP_ROWS, MAP_COLS};
//use super::VarPenConfig;

impl ResultMap{

    pub fn select_cell(&self, prog: &Program) -> usize {
        match self.config.select_cell_method{
            0 => self.eff_len_wid(prog),
//            1 => self.metabolite_len(prog),
            1 => self.eff_len_eff_feat(prog),
            _ => panic!("Invalid get location method!! \n{:?}", self.config),
        }
    }

    fn eff_len_wid(&self, prog: &Program)  -> usize{
        let (l,mut w) = prog.get_inds_simple();   //len, width
        let MAX_WIDTH = 30;
//        let MAX_LEN = 20;

        if w > MAX_WIDTH {w = MAX_WIDTH}

        l*MAX_WIDTH + w
    }


    fn eff_len_eff_feat(&self, prog: &Program)  -> usize{
        let mut l = prog.get_effective_len(0);   //len, width
        let w = prog.get_n_effective_feats(0);   //len, width
//        let MAX_WIDTH = 30;
        let MAX_LEN = 30;

        if l > MAX_LEN {l = MAX_LEN}

        w*MAX_LEN + l
    }


    fn metabolite_len(&self, prog: &Program) -> usize{
//        const metabolite_inds: [u8; 7] = [2,140,0,22,23,25,66,];
//        let prog_mets = prog.get_effective_feats(0);
//
//        let mut y = 0;
//        let mut y_inc = 1;
//        for mi in metabolite_inds.iter() {
//            if prog_mets.contains(mi){
//                y += y_inc;
//            }
//            y_inc *= 2;
//        }
//
//        (prog.get_effective_len(0), y)
        0
    }
}

