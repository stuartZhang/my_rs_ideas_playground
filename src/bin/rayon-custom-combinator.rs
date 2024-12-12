
use ::rayon::prelude::*;

fn is_prime(n: u64) -> bool { // 自定义配合器
    if n <= 1 {
        return false;
    }
    (2..=(n as f64).sqrt() as u64).all(|i| n % i != 0)
}

fn main() {
    let primes: Vec<u64> = (1..100000)
        .into_par_iter()
        .filter(|&x| is_prime(x))
        .collect();
    println!("Found {} primes", primes.len());
}