use std::collections::HashMap;

pub fn separate_by_class(data: &Vec<(Vec<f64>, i32)>) -> HashMap<i32, Vec<Vec<f64>>> {
    let mut separated = HashMap::new();
    for (features, class) in data.iter() {
        separated.entry(*class).or_insert_with(Vec::new).push(features.clone());
    }
    separated
}

pub fn mean(values: &Vec<f64>) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

pub fn stdev(values: &Vec<f64>) -> f64 {
    let avg = mean(values);
    let variance = values.iter().map(|v| (v - avg).powi(2)).sum::<f64>() / values.len() as f64;
    variance.sqrt()
}
pub fn summarize(data: &Vec<Vec<f64>>) -> Vec<(f64, f64)> {
    data.iter().map(|features| {
        (mean(features), stdev(features))
    }).collect()
}


pub fn probablity(x: f64, mean: f64, stdev: f64) -> f64 {
    let exponent = (-((x - mean).powi(2) / (2.0 * stdev.powi(2)))).exp();
    (1.0 / (stdev * (2.0 * std::f64::consts::PI).sqrt())) * exponent
}

pub fn class_probabilities(summaries: &Vec<Vec<(f64, f64)>>, input: &Vec<f64>) -> Vec<f64> {
    summaries.iter().enumerate().map(|(class_value, class_summaries)| {
        let mut prob = 1.0;
        for i in 0..class_summaries.len() {
            let (mean, stdev) = class_summaries[i];
            prob *= probablity(input[i], mean, stdev);
        }
        prob
    }).collect()
}