use ::std::ffi::c_int;
fn main() {
    type Callback2 = extern "C" fn(c_int) -> c_int; // 对应 C 端的 Callback2 类型定义
    type Callback3 = extern "C" fn(Option<Callback2>, c_int) -> c_int; // 对应 C 端的 Callback3 类型定义
    // 导入 C 端的‘回调函数注册’程序接口
    extern "C" {
        fn register1(cb: Option<Callback3>, a: c_int) -> c_int;
    }
    // 待传值给 C 端的 C ABI 函数
    extern "C" fn callback3(process: Option<Callback2>, a: c_int) -> c_int {
        match process {
            Some(callback2) => callback2(a),
            None => a * a
        }
    }
    println!("Rust ➜ C 传值‘空函数指针’");
    let result = unsafe { register1(None, 4) };
    assert_eq!(result, 4);
    println!("C ➜ Rust 回传‘空函数指针’");
    let result = unsafe { register1(Some(callback3), 14) };
    assert_eq!(result, 196);
    println!("Rust ⮂ C 均传递‘非空函数指针’");
    let result = unsafe { register1(Some(callback3), 4) };
    assert_eq!(result, 8);
}