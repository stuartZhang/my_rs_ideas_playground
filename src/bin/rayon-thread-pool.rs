
use ::rayon::{ prelude::*, ThreadPoolBuilder };

fn main() {
    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    let result = pool.install(|| {
        (0..100000)
            .into_par_iter()
            .map(|i| i * i)// 这一步将在线程池内完成
            .sum::<i64>()
    });

    println!("Sum of squares: {}", result);
}