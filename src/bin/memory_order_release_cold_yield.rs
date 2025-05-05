#![feature(get_mut_unchecked)]
mod unsafe_string {
    use ::ambassador::{ Delegate, delegatable_trait_remote };
    use ::std::{ convert::{ AsRef, AsMut }, fmt::{ Display, Formatter, Result as IoResult }, ops::{ Deref, DerefMut } };
    #[delegatable_trait_remote]
    trait Display {
        // Required method
        fn fmt(&self, f: &mut Formatter<'_>) -> IoResult;
    }
    #[derive(Delegate)]
    #[delegate(Display)]
    pub struct UnsafeString(pub String);
    unsafe impl Sync for UnsafeString { }
    unsafe impl Send for UnsafeString { }
    impl Deref for UnsafeString {
        type Target = String;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl DerefMut for UnsafeString {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl AsRef<String> for UnsafeString {
        fn as_ref(&self) -> &String { &self.0 }
    }
    impl AsRef<str> for UnsafeString {
        fn as_ref(&self) -> &str { self.0.as_ref() }
    }
    impl AsMut<String> for UnsafeString {
        fn as_mut(&mut self) -> &mut String { &mut self.0 }
    }
    impl UnsafeString {
       pub fn new<T: AsRef<str>>(source: T) -> Self {
            Self(String::from(source.as_ref()))
       }
    }
}
use ::std::{ sync::{ Arc, atomic::{ AtomicU8, Ordering } }, thread };
use unsafe_string::UnsafeString;
fn main() {
    // 信号量 - 同步线程
    let semaphore = Arc::new(AtomicU8::new(0));
    let payload = Arc::new(UnsafeString::new("以字符串模拟复杂数据结构"));
    let mut join_handles = Vec::new();
    {
        let semaphore = Arc::clone(&semaphore);
        let payload = Arc::clone(&payload);
        join_handles.push(thread::spawn(move || {
            while semaphore.load(Ordering::Acquire) != 2 {
                thread::yield_now(); // 主动放弃 CPU 时间片
             }
            println!("3. 最后打印输出两次修改后的结果: {}", payload);
        }));
    }
    {
        let semaphore = Arc::clone(&semaphore);
        let mut payload = Arc::clone(&payload);
        join_handles.push(thread::spawn(move || {
            while semaphore.load(Ordering::Acquire) != 1 {
                thread::yield_now(); // 主动放弃 CPU 时间片
            }
            // 能看到另一线程对复杂数据结构的非原子变量的修改结果。
            // 注意：
            // (1) 对复杂数据结构变量未加任何形式的锁哟！
            // (2) 这是 unsafe 代码哟！
            unsafe { Arc::get_mut_unchecked(&mut payload).insert_str(0, "【") };
            println!("2. 再添加（首）起始符: {}", payload);
            semaphore.store(2, Ordering::Release);
        }));
    }
    {
        let semaphore = Arc::clone(&semaphore);
        let mut payload = Arc::clone(&payload);
        join_handles.push(thread::spawn(move || {
            // 这是 unsafe 代码，@Rustacean 必须人工保证，除了上面的并发线程，真没有对 payload 其它并发写操作了。
            unsafe { Arc::get_mut_unchecked(&mut payload).push_str("】") };
            println!("1. 先添加（后）结束符: {}", payload);
            semaphore.store(1, Ordering::Release);
        }));
    }
    for (index, join_handle) in join_handles.into_iter().enumerate() {
        join_handle.join().expect(&format!("第{}个线程提前崩溃了", index)[..]);
    }
    println!("4. 结束：{}", payload);
}