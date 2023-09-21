use futures::{channel::mpsc, future, executor, stream::TryStreamExt, StreamExt};
fn main() {
    executor::block_on(async {
        let (mut tx, rx) = mpsc::channel::<Result<i32, ()>>(2);
        let rx = rx.and_then(|result| {
            future::ok(if result % 2 == 0 {
                Some(result)
            } else {
                None
            })
        });
        tx.try_send(Ok(11)).unwrap();
        tx.try_send(Ok(12)).unwrap();
        tx.try_send(Ok(13)).unwrap();
        tx.close_channel();
        let result = rx.collect::<Vec<Result<Option<i32>, ()>>>().await;
        println!("{:?}", result);
    });
}
