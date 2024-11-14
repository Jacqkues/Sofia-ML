use crate::utils::model::Model;

pub fn k_fold_cross_validation(
    data: &[(Vec<f64>, i32)],
    k: usize,
    classifier: &mut dyn Model,
) -> f64 {
    let fold_size = data.len() / k;
    let mut accuracy = 0.0;

    for i in 0..k {
        let test_start = i * fold_size;
        let test_end = (i + 1) * fold_size;
        let test_data: &[(Vec<f64>, i32)] = &data[test_start..test_end];
        let train_data = [&data[0..test_start], &data[test_end..data.len()]].concat();

        
        classifier.train(&train_data.to_vec(),);
        let correct = test_data
            .iter()
            .filter(|(x, y)| classifier.predict(x) == *y)
            .count();
        accuracy += correct as f64 / test_data.len() as f64;
    }
    accuracy / k as f64
}
