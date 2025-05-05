use ::std::{ sync::{ Arc, atomic::{ AtomicBool, AtomicU8, Ordering } }, thread };
fn main() {
    let x = Arc::new(AtomicBool::new(false));
    let y = Arc::new(AtomicBool::new(false));
    let z = Arc::new(AtomicU8::new(0));
    let mut join_handles = Vec::new();
    {
        let x = Arc::clone(&x);
        join_handles.push(thread::spawn(move || {
            x.store(true, Ordering::SeqCst);
        }));
    }
    {
        let y = Arc::clone(&y);
        join_handles.push(thread::spawn(move || {
            y.store(true, Ordering::SeqCst);
        }));
    }
    {
        let x = Arc::clone(&x);
        let y = Arc::clone(&y);
        let z = Arc::clone(&z);
        join_handles.push(thread::spawn(move || {
            while x.load(Ordering::SeqCst) == false { /* 热轮询 - 等待信号量就位 */ }
            if y.load(Ordering::SeqCst) == true {
                z.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    {
        let x = Arc::clone(&x);
        let y = Arc::clone(&y);
        let z = Arc::clone(&z);
        join_handles.push(thread::spawn(move || {
            while y.load(Ordering::SeqCst) == false { /* 热轮询 - 等待信号量就位 */ }
            if x.load(Ordering::SeqCst) == true {
                z.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    for (index, join_handle) in join_handles.into_iter().enumerate() {
        join_handle.join().expect(&format!("第{}个线程提前崩溃了", index)[..]);
    }
    let z = z.load(Ordering::SeqCst);
    assert_ne!(z, 0_u8);
    dbg!(z);
}