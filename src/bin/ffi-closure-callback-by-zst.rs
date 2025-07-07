/**
 * 宏
 * 1. 功能：将（被导入）外部函数 extern "C" fn 形参中的【函数指针】升级为【闭包】。
 * 2. 对（被导入）外部函数的要求
 *    a. 末尾形参·必须是 void * 类型的指针变量，因为它被用来搬运来自 rust 端的【闭包】捕获变量。C 端不需要解构它们，而仅只透传。
 *    b. 倒数第二个形参·必须是【函数指针】回调函数。而且，回调函数的末尾形参·也必须是 void * 类型的指针变量。它被用来将【闭包】捕获变量送回给 rust 端
 *    c. 仅只支持一个回调函数。
 * 3. 例子
 *    C 端
 *      // 回调函数的函数签名。注意末尾形参 void *
 *      typedef void (*Callback)(int result, void *closure);
 *      // 导出函数的函数签名。注意末尾两个形参 Callback 和 void *
 *      void add_two_numbers(int a, int b, Callback cb, void *closure) {...}
 *    Rust 端
 *      ffi_closure_shim_fn!(
 *          // （被导入）外部函数名
 *          add_two_numbers(
 *              // 形参列表内不包括末尾的 void * 参数，因为宏会自动为其添加上【回调·垫片·函数】
 *              a: c_int, b: c_int
 *              // 注意双逗号分隔
 *              ,,
 *              // （被导出）回调函数。形参列表内也不包括末尾的 void * 参数，因为宏会自动为其添加上【回调·垫片·函数】
 *              callback(result: c_int)
 *          )
 *      );
 *      风格有点类似于 nodejs 里的 require('util').promisify(fn) 不包含形参列表里的回调函数。
 * 4. 宏私下做了哪些工作？
 *    就上例而言，给（导入）外部函数与（导出）回调函数生成【垫片函数】。
 *    a. （导入）外部函数的【垫片函数】装箱【闭包】捕获变量，经由 void * 类型抹平指针，透传给 C 端程序。
 *    b. （导出）回调函数的【垫片函数】拆箱被透传的【闭包】捕获变量。再以回调函数的“有效载荷”实参，调用【闭包】。
 */
macro_rules! ffi_closure_shim_fn {
    (
        // 被导入的外部函数。
        $c_fn_name: ident (
            // 形参列表内不包括末尾的 void * 参数，因为宏会自动为其添加上【回调·垫片·函数】
            $( $c_fn_param_name: ident : $c_fn_param_type: ty),*,,
            // 被导出回调函数。形参列表内也不包括末尾的 void * 参数，因为宏会自动为其添加上【回调·垫片·函数】
            callback( $( $cb_fn_param_name: ident : $cb_fn_param_type: ty ),* )
        )
    ) => {
        /**
         * 【FFI·垫片·调用函数】
         * 1. 首先，对【闭包】捕获变量“装箱”，
         * 2. 然后，代理调用真正的 extern fn 外部函数和以 void * 指针透传【闭包】。
         * 3. 接着，以【FFI·垫片·回调函数】接收 C 端的反馈结果和被回传的【闭包】。
         * 4. 于是，“拆箱”【闭包】捕获变量。
         * 5. 最后，凭借 C 端反馈结果与被“拆箱”的【闭包】捕获变量，调用【闭包】。
         */
        fn $c_fn_name<F>($( $c_fn_param_name: $c_fn_param_type, )* mut closure: F)
            where F: FnMut($( $cb_fn_param_type),*) {
            /**
             * 【FFI·垫片·回调函数·签名】
             * 不得不写在【外部块】之外，因为【外部类型】还没有被正式地支持需要开启 feature-gate 开关
             */
            type ShimCallback = unsafe extern "C" fn($( $cb_fn_param_type, )* *mut ());
            extern "C" {
                /**
                 * 【外部函数】
                 * 导入 C 端的功能函数 API
                 * void add_two_numbers(int a, int b, Callback cb, void *closure)
                 */
                fn $c_fn_name(
                    $( $c_fn_param_name: $c_fn_param_type, )*
                     // 回调函数 — 【垫片·回调函数】将作为其实参
                    shim_callback: ShimCallback,
                    // 作为状态值“兜转”传递的 Rust 端【闭包】结构体
                    closure: *mut ()
                );
            }
            let shim_callback = specialize(&closure);
            unsafe {
                $c_fn_name($( $c_fn_param_name, )*  shim_callback, &mut closure as *mut _ as *mut ());
            }
            /**
             * 【FFI·垫片·回调函数·定义】
             * 此导出函数被刻意设计为 unsafe 的，因为需要由它的调用端自觉地确保 void * 指针背后
             * 的数据值是类型匹配的【闭包】<T: Fn(..)>结构体
             */
            unsafe extern "C" fn shim_closure<F>($( $cb_fn_param_name: $cb_fn_param_type, )* closure: *mut ())
                where F: FnMut($( $cb_fn_param_type),*) {
                let closure = &mut *(closure as *mut F);
                closure($($cb_fn_param_name),*);
            }
            /**
             * 【专化函数】
             * 首先，从【闭包】对象实参推断【闭包】的具体类型 — 这只有编译器能做。
             * 然后，返回被“专化”的【垫片·回调函数】。
             */
            const fn specialize<F>(_closure: &F) -> ShimCallback
                where F: FnMut($( $cb_fn_param_type),*) {
                shim_closure::<F>
            }
        }
    };
    (
        // 被导入的外部函数。
        $c_fn_name: ident (
            // 形参列表内不包括末尾的 void * 参数，因为宏会自动为其添加上【回调·垫片·函数】
            // 被导出回调函数。形参列表内也不包括末尾的 void * 参数，因为宏会自动为其添加上【回调·垫片·函数】
            callback( $( $cb_fn_param_name: ident : $cb_fn_param_type: ty ),* )
        )
    ) => {
        ffi_closure_shim_fn!(,, $( $cb_fn_param_name: $cb_fn_param_type),* );
    }
}
fn main() {
    use libc::c_int;
    ffi_closure_shim_fn!(
        add_two_numbers(a: c_int, b: c_int,, callback(result: c_int))
    );
    let mut got = 0;
    add_two_numbers(1, 2, |result: c_int| got = result);
    assert_eq!(got, 1 + 2);
}