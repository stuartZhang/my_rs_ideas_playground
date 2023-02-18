/*
 > 最近在【知乎】上收到一个问题。题面：若【匿名-类型】元组结构体内的元素个数超过 20 个，
   则该【元组-结构体】实例就不可被`{:?}`打印输出了。如何搞才能让该元组可被输出？
 > 我当时给出的答案是：【匿名】的不行，那就显示地创建【具名】的呗。但是，这个答案
   不足够完美，因为使用【匿名】的初衷就是图省事。我的答案虽然解决问题了，但是把问
   题给费事化了。
 > 所以，我要写一个【规则-宏】让该问题再次回归省事（其实，还是比原版【匿名-元组结
   构体】费事了一点儿，真的就一点儿）。
 > 该【规则-宏】同时使用了四种设计模式
   （1）`Incremental TT munchers`
        - 代码见由`@typeVal`引导片段内`->`左侧的子片段
   （2）`Push-down Accumulation`
        - 代码见由`@typeVal`引导片段内`->`右侧的子片段
   （3）`TT Bundling`
        - 代码见由`@decorate`引导的片段
        - 功能：将`$(#[$meta: meta])+`当作`$($meta: tt)*`透传至递归结束宏规则。
   （4）内部宏
        - 代码见`@typeVal`与`@decorate`字面量
 > 【宏调用】的实参格式：
    （1）(值1;类型1, 值2;类型2, ...)
    （2）(元属性1
          元属性2
          ...
          值1;类型1, 值2;类型2, ...)
 > 【宏】工作原理：
    （1）构建一个临时的，局部作用域代码【块】。在该【块】内，
    （2）使用被提供的类型与元属性，定义一个【具名-元组结构体】项。
         - 该【具名-元组结构体】项至少会派生`std::fmt::Debug trait`。
    （3）使用被提供的值，构造一个【具名-元组结构体】实例
    （4）将该【具名-元组结构体】实例作为当前【块】的返回值
 */
macro_rules! printable_tuple {
    [@typeVal  () -> (($($vo: tt)*), ($($to: tt)*))
        @decorate $(#[$meta: meta])*] => {{ // `TT Bundling`将`: tt`升级为`: meta`
        $(#[$meta])*
        #[derive(Debug)]
        struct Printable($($to)*);
        Printable($($vo)*)
    }};
    [@typeVal  ($vi: expr; $ti: ty) -> (($($vo: tt)*), ($($to: tt)*))
        @decorate $($meta: tt)*] => { // `TT Bundling`作为`: tt`透传
        printable_tuple!(@typeVal  () -> (($($vo)* $vi,), ($($to)* $ti,))
                            @decorate $($meta)*)
    };
    [@typeVal  ($vi: expr; $ti: ty, $($rest: tt)*) -> (($($vo: tt)*), ($($to: tt)*))
        @decorate $($meta: tt)*] => { // `TT Bundling`作为`: tt`透传
        printable_tuple!(@typeVal  ($($rest)*) -> (($($vo)* $vi,), ($($to)* $ti,))
                            @decorate $($meta)*)
    };
    ($(#[$meta: meta])+ $($body:tt)*) => {
        printable_tuple!(@typeVal  ($($body)*) -> ((), ())
                            @decorate $(#[$meta])*) // `TT Bundling`将`: meta`降级为`: tt`
    };
    ($($body:tt)*) => {
        printable_tuple!(@typeVal  ($($body)*) -> ((), ())
                            @decorate)
    };
}
fn main() {
    // 测试
    let printable = printable_tuple!(1;u32, 2;i32, 3;u8, 4;i8,
                                     "1";&'static str, 2.0;f32, 3.0;f64, true;bool,
                                     1;i32, 2;i32, 3;u8, 4;u32,
                                     1;i32, 2;i32, 3;u8, 4;u32,
                                     1;i32, 2;i32, 3;u8, 4;u32);
    println!("printable_tuples1: {:?}", printable);
    let printable = printable_tuple!(#[derive(Clone, Copy)]
                                     1;u32, 2;i32, 3;u8, 4;i8,
                                     "1";&'static str, 2.0;f32, 3.0;f64, true;bool,
                                     1;i32, 2;i32, 3;u8, 4;u32,
                                     1;i32, 2;i32, 3;u8, 4;u32,
                                     1;i32, 2;i32, 3;u8, 4;u32);
    println!("printable_tuples2: {:?}", printable);
}