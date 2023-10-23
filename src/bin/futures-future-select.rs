use ::futures::{future::{self, Either, FutureExt}, executor};
use ::futures_time::{prelude::*, time::Duration};
use ::std::future::Future;
fn main() {
    executor::block_on(async {
        let fut1 = async move { 12 }.delay(Duration::from_millis(100));
        let fut2 = async move { 44 };
        //
        futures::pin_mut!(fut1);
        futures::pin_mut!(fut2);
        //
        let (x, y) = join(fut1, fut2).await;
        dbg!(x, y);
    });
}
fn join<A, B>(a: A, b: B) -> impl Future<Output=(A::Output, B::Output)>
where A: Future + Unpin,
      B: Future + Unpin {
    future::select(a, b).then(|either| {
        match either {
            Either::Left((x, b)) => b.map(move |y| (x, y)).left_future(),
            Either::Right((y, a)) => a.map(move |x| (x, y)).right_future(),
        }
    })
}