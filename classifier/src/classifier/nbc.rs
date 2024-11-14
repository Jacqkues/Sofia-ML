use std::collections::HashMap;

use crate::utils::{model::Model, stat::{separate_by_class, mean, stdev, summarize}};

pub struct NaiveBayesClassifier {
    pub summaries: Option<HashMap<i32, Vec<(f64, f64)>>>,
}

impl Model for NaiveBayesClassifier {
    fn predict(&self, x: &Vec<f64>) -> i32 {
        match &self.summaries {
            Some(summaries) => {
                let mut best_class = -1;
                let mut best_prob = f64::MIN;

                for (class, class_summaries) in summaries {
                    let mut log_prob = 0.0;
                    for i in 0..x.len() {
                        let (mean, stdev) = class_summaries[i];
                        let normal_prob = (1.0 / (2.0 * std::f64::consts::PI * stdev.powi(2)).sqrt()) *
                            (-((x[i] - mean).powi(2) / (2.0 * stdev.powi(2)))).exp();
                        log_prob += normal_prob.ln();
                    }

                    if log_prob > best_prob {
                        best_prob = log_prob;
                        best_class = *class;
                    }
                }

                best_class
            },
            None => panic!("Model not trained yet"),
        }
    }

    fn train(&mut self, data: &Vec<(Vec<f64>, i32)>) {
        let separated = separate_by_class(data);
        let mut summaries = HashMap::new();
        for (class, instances) in separated {
            summaries.insert(class, summarize(&instances));
        }
        self.summaries = Some(summaries);
    }
}