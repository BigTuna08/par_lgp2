use std::ops::Range;

pub const N_FEATURES: u8 = 156;

pub const TRAIN_PROP: f32 = 0.6;
pub const N_SAMPLES: usize = 389;

pub const LBL_IND: usize = 2;
pub const FEAT_RNG: Range<usize> = 3..159;