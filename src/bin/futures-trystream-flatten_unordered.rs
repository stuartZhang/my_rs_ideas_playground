use ::futures::{channel::mpsc::{self, UnboundedReceiver}, executor, stream::{self, StreamExt, TryStreamExt}};
use ::std::thread;
fn main() {
    executor::block_on(async {
        // a. 三个信道的接口端就是三个 Stream，
        // b. 因为流内的每个迭代项都是 Result<i32, i32>，所以此流是 TryStream
        let (tx1, rx1) = mpsc::unbounded();
        let (tx2, rx2) = mpsc::unbounded();
        let (tx3, rx3) = mpsc::unbounded();
        // c. 分三个线程并行地向三个 TryStream 接收端，经由 Sink 发送端，注入
        //    Result<i32, i32> 迭代项
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
        // d. 将三个接收端 TryStream 流合并成一个流的流
        let stream = stream::iter::<[Result<UnboundedReceiver<Result<i32, i32>>, i32>; 3]>([Ok(rx1), Ok(rx2), Ok(rx3)]);
        // e. 以（迭代项）流中子迭代项的随机并发次序，压平嵌套流的流为一维的流。
        let stream = stream.try_flatten_unordered(None);
        let mut values = stream.collect::<Vec<Result<i32, i32>>>().await;
        println!("{values:?}");
        values.sort();
        assert_eq!(values, vec![Ok(1), Ok(2), Ok(4), Err(3), Err(5)]);
        Ok::<(), i32>(())
    }).unwrap();
}
