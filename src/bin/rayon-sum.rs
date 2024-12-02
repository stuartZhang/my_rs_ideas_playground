use ::rayon::prelude::*;

fn main() {
    let numbers: Vec<i64> = (1..10001).collect();
    let sum: i64 = numbers.par_iter().sum();
    println!("Sum: {}", sum);
}