use ::libc::{c_int, c_void};
/**
 * 【FFI·垫片·回调函数·签名】
 * 呼应于 C 端【回调函数】的定义
 * typedef void (*Callback)(int result, void *closure);
 */
type ShimCallback = unsafe extern "C" fn(c_int, *mut c_void);
/**
 * 【FFI·垫片·回调函数·定义】
 * 此导出函数被刻意设计为 unsafe 的，因为需要由它的调用端自觉地确保 void * 指针之后
 * 的数据是类型匹配的闭包结构体
 */
unsafe extern "C" fn shim_closure<F>(result: c_int, closure: *mut c_void)
where F: FnMut(c_int) {
    let closure = &mut *(closure as *mut F);
    closure(result);
}

extern "C" {
    /**
     * 【外部函数】
     * 导入 C 端的功能函数 API
     * void add_two_numbers(int a, int b, Callback cb, void *closure)
     */
    fn add_two_numbers(
        a: c_int,
        b: c_int,
        shim_callback: ShimCallback, // 回调函数 — 【垫片·回调函数】将作为其实参
        closure: *mut c_void         // 作为状态值“兜转”传递的 Rust 端【闭包】结构体
    );
}
/**
 * 【垫片·调用函数】
 */
fn shim_add_two_numbers<F>(a: i32, b: i32, mut closure: F)
where F: FnMut(i32) {
    let shim_callback = specialize(&closure);
    unsafe {
        add_two_numbers(a, b, shim_callback, &mut closure as *mut _ as *mut c_void);
    }
}
/**
 * 【专化函数】
 * 从【闭包】实例的实参推断【闭包】的具体类型。这步操作只有编译器能做。
 * 然后，返回被“专化”的【垫片·回调函数】。
 */
const fn specialize<F>(_closure: &F) -> ShimCallback
where F: FnMut(c_int) {
    shim_closure::<F>
}
fn main() {
    let mut got = 0;
    shim_add_two_numbers(1, 2, |result: c_int| got = result);
    assert_eq!(got, 1 + 2);
}