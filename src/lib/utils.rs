extern crate rand;
use rand::Rng;

pub fn random_int(min: i64, max: i64) -> i64 {
    rand::thread_rng().gen_range(min, max)
}

pub fn random_index(min: usize, max:usize) -> usize {
    rand::thread_rng().gen_range(min, max)
}

pub fn random_percentage(percentage: f32) -> bool {
    percentage / 100.0 >= rand::thread_rng().gen_range(0.0, 1.0)
}

pub fn random_element<T>(values: &[T]) -> &T {
    rand::thread_rng().choose(values).unwrap()
}