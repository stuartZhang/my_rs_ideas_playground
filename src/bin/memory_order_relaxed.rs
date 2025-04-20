use ::std::{ sync::{ Arc, atomic::{ AtomicI32, Ordering } }, thread };
fn main() {
    let counter = Arc::new(AtomicI32::new(0));
    let mut join_handles = Vec::new();
    for _ in 0..110 {
        let counter = Arc::clone(&counter);
        join_handles.push(thread::spawn(move || {
            counter.fetch_add(1, Ordering::Relaxed);
        }));
    }
    for (index, join_handle) in join_handles.into_iter().enumerate() {
        join_handle.join().expect(&format!("第{}个线程提前崩溃了", index)[..]);
    }
    println!("最终的计数结果是：{}。它总是正确的", counter.load(Ordering::Relaxed));
}