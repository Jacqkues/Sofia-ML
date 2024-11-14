use std::{collections::HashMap, error::Error};


use serde::Deserialize;

use crate::utils::{build_dataset, load_csv_from_struct, ToFeaturesAndLabel};

#[derive(Debug, Deserialize)]
pub struct Cvr{
    c1:String,
    c2:String,
    c3:String,
    c4:String,
    c5:String,
    c6:String,
    c7:String,
    c8:String,
    c9:String,
    c10:String,
    c11:String,
    c12:String,
    c13:String,
    c14:String,
    c15:String,
    c16:String,
    c17:String
}

impl ToFeaturesAndLabel for Cvr {
    fn to_features_and_label(&self) -> (Vec<f64>, String) {
        let features = self.preprocess();
        (features, self.c1.clone())
    }
}

impl Cvr{

    pub fn load() ->  (Vec<(Vec<f64>, i32)>, HashMap<String, i32>){
        let data: Result<Vec<Cvr>, Box<dyn Error>> = load_csv_from_struct("./src/dataset/cvr/house-votes-84.data");
        let (dataset, class) = build_dataset(data.expect("failed to read congressional_voting_records.csv"));
        (dataset,class)
    }
    fn preprocess(&self) -> Vec<f64>{
        let mut features = Vec::new();
        for i in 2..18{
            let field = format!("c{}", i);
            let value = self.get(&field).unwrap();
            let value = match value.as_str(){
                "y" => 1.0,
                "n" => 0.0,
                "?" => 0.5,
                x => panic!("Invalid value {x}")
            };
            features.push(value);
        }
        features
    }
    fn get(&self, field: &str) -> Option<String>{
        match field{
            "c1" => Some(self.c1.clone()),
            "c2" => Some(self.c2.clone()),
            "c3" => Some(self.c3.clone()),
            "c4" => Some(self.c4.clone()),
            "c5" => Some(self.c5.clone()),
            "c6" => Some(self.c6.clone()),
            "c7" => Some(self.c7.clone()),
            "c8" => Some(self.c8.clone()),
            "c9" => Some(self.c9.clone()),
            "c10" => Some(self.c10.clone()),
            "c11" => Some(self.c11.clone()),
            "c12" => Some(self.c12.clone()),
            "c13" => Some(self.c13.clone()),
            "c14" => Some(self.c14.clone()),
            "c15" => Some(self.c15.clone()),
            "c16" => Some(self.c16.clone()),
            "c17" => Some(self.c17.clone()),
            _ => None
        }
    }
}