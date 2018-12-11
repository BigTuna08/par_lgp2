
// Rates are expressed as 1 in RATE chance (eg RATE = 20 => 1/20 = 4% chance)

pub const REPLACE_EQ_FIT: u32 = 2; //rate to replace best when fitness is eq


pub const MICRO_MUT_RATE: f32 = 0.03f32;


pub const DEL_RATE: f32 = 0.03f32;
pub const DEL_CONT_RATE: f32 = 0.5;

//pub const DUP_RATE: f32 = 0.02f32;
//pub const DUP_CONT_RATE: f32 = 0.5;

pub const COPY_RATE: f32 = 0.02f32;
pub const COPY_CONT_RATE: f32 = 0.5;
pub const QUEUE_V_LOCAL_COPY_RATE: f32 = 0.0f32;

pub const ADD_FEAT_RATE: f32 = 0.02f32;
pub const DEL_FEAT_RATE: f32 = 0.02f32;
pub const CHG_FEAT_RATE: f32 = 0.02f32;


pub const ADD_FEAT_MAX_REG: u8 = 50;  // Features will be added to regs with inds less than this

pub const N_INSTR_OPTIONS: usize = 6;
pub const INSTR_OPTIONS: [u8; N_INSTR_OPTIONS] = [0, 1, 2, 3, 6, 7];

pub const QUEUE_LEN: usize = 20;





//
pub const N_OPS: u8 = 6;




//       I think the below are main ones used


pub const INSTR_INSERT_RATE: u32 = 50; //rate to insert new instruction after copying instruction
//pub const INSTR_DUPL_RATE: u32 = 50; //rate to duplicate new instruction after copying instruction
pub const INSTR_DEL_RATE: u32 = 25; //rate to insert new instruction after copying instruction

pub const MUT_INSTR_COPY_RATE: u32 = 20; // was 200
