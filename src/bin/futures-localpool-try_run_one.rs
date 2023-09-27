use ::futures::{executor::LocalPool, future::{ready, pending}, task::LocalSpawnExt};
use ::futures_time::{prelude::*, time::Duration};
use ::std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    // 实例化【任务池】
    let mut pool = LocalPool::new();
    // 获得（可克隆的）任务生成器（句柄）
    let spawner = pool.spawner();
    // 经由任务生成器（句柄），向任务池中【异步】提交任务 Task
    //  1. 已经 Ready 的 Future
    spawner.spawn_local(ready(()))?;
    //  2. 延迟 Ready 的 Future
    spawner.spawn_local(ready(()).delay(Duration::from_millis(1)))?;
    //  3. 永远 Pending 的 Future
    spawner.spawn_local(pending())?;
    // Pool::try_run_one() 是非阻塞的。它不会同步等待任何一个 Future 完成，而是立即返回。
    dbg!(pool.try_run_one()); // 因为第一个 Future 是初始 Ready 的，所以返回 true
    dbg!(pool.try_run_one()); // 因为第二个 Future 是延迟 Ready 的，所以返回 false
    dbg!(pool.try_run_one());
    dbg!(pool.try_run_one()); // 因为第三个 Future 是延迟 永远 Pending 的，所以还是返回 false
    Ok(())
}
