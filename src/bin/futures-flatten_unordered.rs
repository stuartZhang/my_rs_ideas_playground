use futures::{channel::mpsc, stream::StreamExt};
use std::thread;
fn main() {
    futures::executor::block_on(async {
        let (tx1, rx1) = mpsc::unbounded();
        let (tx2, rx2) = mpsc::unbounded();
        let (tx3, rx3) = mpsc::unbounded();

        thread::spawn(move || {
            tx1.unbounded_send(1).unwrap();
            tx1.unbounded_send(2).unwrap();
        });
        thread::spawn(move || {
            tx2.unbounded_send(3).unwrap();
            tx2.unbounded_send(4).unwrap();
        });
        thread::spawn(move || {
            tx3.unbounded_send(rx1).unwrap();
            tx3.unbounded_send(rx2).unwrap();
        });
        let output = rx3.flatten_unordered(None).collect::<Vec<_>>().await;
        println!("{output:?}");
    });
}