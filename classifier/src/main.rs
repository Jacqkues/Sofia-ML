mod classifier;
mod utils;
mod validation;
mod dataset;
use std::error::Error;

use classifier::knn::KnnClassifier;
use dataset::iris::Iris;
use crate::utils::model::Model;


fn main() -> Result<(), Box<dyn Error>> {
    
    let (dataset, _class) = Iris::load();

    let mut knn_classifier = KnnClassifier::new(5);

    knn_classifier.train(&dataset);

    let accuracy = validation::k_fold_cross_validation(&dataset, 10, &mut knn_classifier);

  //  println!("Accuracy: {}", accuracy);

    let (dt,_cl) = dataset::congressional_voting_records::Cvr::load();

    let mut nb_gaussien = classifier::nbc::NaiveBayesClassifier{summaries: None};

    let acc = validation::k_fold_cross_validation(&dt, 10, &mut nb_gaussien);
    println!("Accuracy: {}", acc);
    Ok(())
}
