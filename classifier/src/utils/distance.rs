pub fn euclidean_distance(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    a.iter().zip(b.iter()).map(|(ai, bi)| (ai - bi).powi(2)).sum::<f64>().sqrt()
}