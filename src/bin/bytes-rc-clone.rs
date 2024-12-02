//! bytes crate 字节缓存 copy 都是被引用计数的，没有内存复制
use ::bytes::Bytes;
fn main() {
    let original = Bytes::from("Hello, World!");
    let shared = original.clone();  // 这里不会发生实际的内存拷贝

    println!("Original: {:?}", original);
    println!("Shared: {:?}", shared);
    println!("Are they equal? {}", original == shared);
}