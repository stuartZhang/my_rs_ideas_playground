use ::futures::{channel::mpsc::{self, UnboundedReceiver}, executor, stream::{self, StreamExt, TryStreamExt}};
use ::std::thread;
fn main() {
    executor::block_on(async {
        let (tx1, rx1) = mpsc::unbounded();
        let (tx2, rx2) = mpsc::unbounded();
        let (tx3, rx3) = mpsc::unbounded();

        thread::spawn(move || {
            tx1.unbounded_send(Ok(1)).unwrap();
        });
        thread::spawn(move || {
            tx2.unbounded_send(Ok(2)).unwrap();
            tx2.unbounded_send(Err(3)).unwrap();
            tx2.unbounded_send(Ok(4)).unwrap();
        });
        thread::spawn(move || {
            tx3.unbounded_send(Err(5)).unwrap();
        });
        let stream = stream::iter::<Vec<Result<UnboundedReceiver<Result<i32, i32>>, i32>>>(vec![Ok(rx1), Ok(rx2), Ok(rx3)]);
        let mut stream = stream.try_flatten();
        assert_eq!(stream.next().await, Some(Ok(1)));
        assert_eq!(stream.next().await, Some(Ok(2)));
        assert_eq!(stream.next().await, Some(Err(3)));
        assert_eq!(stream.next().await, Some(Ok(4)));
        assert_eq!(stream.next().await, Some(Err(5)));
        assert_eq!(stream.next().await, None);
    });
}
