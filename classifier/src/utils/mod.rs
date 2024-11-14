use csv;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
pub(crate) mod distance;
pub(crate) mod model;
pub(crate) mod stat;
pub fn load_csv_from_struct<T: DeserializeOwned + Debug, P: AsRef<Path>>(
    path: P,
) -> Result<Vec<T>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let mut rows = Vec::new();

    for result in reader.deserialize() {
        let record: T = result?;

        rows.push(record);
    }

    Ok(rows)
}

pub fn load_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();

    for result in reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        rows.push(row);
    }

    Ok(rows)
}


pub trait ToFeaturesAndLabel {
    fn to_features_and_label(&self) -> (Vec<f64>, String);
}

pub fn build_dataset<T: ToFeaturesAndLabel>(rows: Vec<T>) -> (Vec<(Vec<f64>, i32)>, HashMap<String, i32>) {
    let mut class_map = HashMap::new();
    let mut next_class_id = 0;
    let dataset = rows.into_iter().map(|row| {
        let (features, label) = row.to_features_and_label();
        let class_id = *class_map.entry(label.clone()).or_insert_with(|| {
            let id = next_class_id;
            next_class_id += 1;
            id
        });
        (features, class_id)
    }).collect();
    (dataset, class_map)
}

