use std::{collections::HashMap, error::Error};

use serde::Deserialize;

use crate::utils::{build_dataset, load_csv_from_struct, ToFeaturesAndLabel};

#[derive(Debug, Deserialize)]

pub struct Iris {
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
    variety: String,
}

impl Iris {
    pub fn load() ->  (Vec<(Vec<f64>, i32)>, HashMap<String, i32>){
        let data: Result<Vec<Iris>, Box<dyn Error>> = load_csv_from_struct("./src/dataset/iris.csv");
        let (dataset, class) = build_dataset(data.expect("failed to read iris.csv"));
        (dataset,class)
    }
}

impl ToFeaturesAndLabel for Iris {
    fn to_features_and_label(&self) -> (Vec<f64>, String) {
        let features = vec![
            self.sepal_length,
            self.sepal_width,
            self.petal_length,
            self.petal_width,
        ];
        (features, self.variety.clone())
    }
}