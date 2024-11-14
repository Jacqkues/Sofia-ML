use std::collections::HashMap;

use crate::utils::distance::euclidean_distance;
use crate::utils::model::Model;
pub struct KnnClassifier {
    train_data: Vec<(Vec<f64>, i32)>,
    k: usize,
}

impl Model for KnnClassifier {

    fn train(&mut self, data: &Vec<(Vec<f64>, i32)>) {
        self.train_data = data.to_vec();
    }

    fn predict(&self, x: &Vec<f64>) -> i32 {
        let mut distances: Vec<(f64, &i32)> = self
            .train_data
            .iter()
            .map(|(xi, yi)| (euclidean_distance(xi, x), yi))
            .collect();
        distances.sort_by(|a, b: &(f64, &i32)| a.0.partial_cmp(&b.0).unwrap());
        let mut class_count = HashMap::new();
        for &(_, class) in distances.iter().take(self.k) {
            *class_count.entry(class).or_insert(0) += 1;
        }
        *class_count
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(&class, _)| class)
            .unwrap()
    }
}


impl KnnClassifier {
    pub fn new(k: usize) -> Self {
        Self {
            train_data: Vec::new(),
            k,
        }
    }
}
