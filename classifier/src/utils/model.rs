pub trait Model {
    fn train(&mut self, data: &Vec<(Vec<f64>, i32)>);
    fn predict(&self, x: &Vec<f64>) -> i32;
}