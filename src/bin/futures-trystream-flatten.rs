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
        // e. 以尊重迭代项的次序，压平嵌套流的流为一维的流。即，前一个（迭代项）流
        //    的迭代项全部被收拢之后，才开始后一个（迭代项）流的迭代项的收拢处理。
        let mut stream = stream.try_flatten();
        assert_eq!(stream.try_next().await?, Some(1));
        assert_eq!(stream.try_next().await?, Some(2));
        assert_eq!(stream.next().await, Some(Err(3)));
        assert_eq!(stream.try_next().await?, Some(4));
        assert_eq!(stream.next().await, Some(Err(5)));
        assert_eq!(stream.try_next().await?, None);
        Ok::<(), i32>(())
    }).unwrap();
}
