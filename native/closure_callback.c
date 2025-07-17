#include <stddef.h>
//
// 回调闭包，对应 Rust 端的 bin/ffi-closure-callback.rs 与 ffi-closure-callback-by-zst.rs 文件
//
typedef void (*Callback1)(int result, void* closure);

void add_two_numbers(int a, int b, Callback1 cb, void* closure)
{
    int result = a + b;
    cb(result, closure);
}
//
// 空指针优化，对应 Ruest 端的 bin/ffi-nullable-pointer-optimization.rs 文件
//
typedef int (*Callback2)(int a); // 对应 Rust 端的 Option<Callback2> 类型定义
typedef int (*Callback3)(Callback2 cb2, int a); // 对应 Rust 端的 Option<Callback3> 类型定义

static int callback2(int result)
{
    return result + result;
}
int register1(Callback3 cb3, int a)
{
    if (cb3 == NULL) { // 接收来自 Rust 端的空函数指针
        return a;
    }
    if (a > 10) { // 向 Rust 端回传空函数指针
        return cb3(NULL, a);
    }
    return cb3(callback2, a); // 不接收也不回传空函数指针
}