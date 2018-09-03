//use csv;
use csv::ReaderBuilder;
//use params as global_params;
use rand;
use rand::Rng;
use std::fs::File;
//use std::sync::Arc;
use data::{DataSet, DataRecord, params};

impl DataRecord{
    fn new_blank()->DataRecord{
        DataRecord{
            features: [0.0; params::N_FEATURES as usize],
            class: false,
        }
    }
}


impl DataSet{
    pub fn new_pair(data_file: &str) -> (Box<DataSet>, Box<DataSet>) {
        let mut rng = rand::thread_rng();

        ///             count # of cases and controls         ////
        let (mut n_case, mut n_control) = (0, 0);
        {
            let f = File::open(data_file).unwrap();
            let mut csv_rdr = ReaderBuilder::new()
                .delimiter(b'\t')
                .from_reader(f);


            for (record_i, result) in csv_rdr.records().enumerate() {
                if let Ok(result) = result {
                    for (j, next_entry) in result.iter().enumerate() {
                        if j == params::LBL_IND {
                            match next_entry {
                                "0" => n_control += 1,
                                "1" => n_case += 1,
                                _ => panic!("Invalid classification field!!")
                            };
                            break;
                        }
                    }
                } else {
                    panic!("bad record! i={}, {:?}", record_i, &result);
                }
            }
        }

        let f = File::open(data_file).unwrap();
        let mut csv_rdr = ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(f);

        let train_control_count = (params::TRAIN_PROP*(n_control as f32)) as usize;
        let train_case_count = (params::TRAIN_PROP*(n_case as f32)) as usize;
        let train_count = train_control_count + train_case_count;

        let mut train_records =  Vec::with_capacity(train_count);
        let mut test_records =  Vec::with_capacity(params::N_SAMPLES - train_count);

        for (record_i,result) in csv_rdr.records().enumerate() {
            if let Ok(result) = result{
                println!("result is {:?}", result);

                let mut class = None;
                let mut features = [0.0f32; params::N_FEATURES as usize];
                let mut feature_i = 0;

                for (j, next_entry) in result.iter().enumerate() {
                    if j == params::LBL_IND{
                        class = Some(match next_entry {
                            "0" => false,
                            "1" => true,
                            _ => panic!("Invalid classification field!!")
                        });
                    }
                    else if j >= params::FEAT_RNG.start && j < params::FEAT_RNG.end {
                        match next_entry.parse::<f32>() {
                            Ok(entry) => {
                                features[feature_i] = entry;
                                feature_i+= 1;
                            },
                            Err(e) => {
                                print!("Error reading something!! i={} j={} err is {:?}", feature_i, j, e);
                                panic!("error getting inputs!, change code if dataset containt missing");
                            }
                        }
                    }
                }
                assert_eq!(params::N_FEATURES as usize, feature_i, "error in features");
                match class {
                    Some(class) => {
                        if rng.gen::<f32>() < params::TRAIN_PROP && train_records.len() < train_count {
                            train_records.push(DataRecord{features, class});
                        }
                        else {
                            test_records.push(DataRecord{features, class});
                        }
                    },
                    None => panic!("Error getting class!"),
                }

            }
                else {
                    panic!("bad record! i={}, {:?}", record_i, &result);
                }
        }
        assert_eq!(test_records.len() + train_records.len(), params::N_SAMPLES, "wrong # of features");

        (Box::new(DataSet{
            records:test_records
        }),
        Box::new(DataSet{
            records:train_records
        })
        )

    }
}