fn main() {
    macro_rules! abacus {
        // 日志记录·内部规则
        (@log left = [$($left: tt)*], right = [$($right: tt)*]) => {
            let left = (&[$(stringify!($left)),*] as &[&str]).join("");
            let right = (&[$(stringify!($right)),*] as &[&str]).join("");
            println!("此次输入 {left:>22} 上次输出 {right}");
        };
        // +-       抵消输出尾的 +
        ((- $($moves: tt)*) -> (+ $($count: tt)*)) => ({
            abacus!(@log left = [- $($moves)*], right = [+ $($count)*]);
            abacus!(($($moves)*) -> ($($count)*))
        });
        // -- 或 _- 新添输出尾的 -
        ((- $($moves: tt)*) -> ($($count: tt)*)) => ({
            abacus!(@log left = [- $($moves)*], right = [$($count)*]);
           abacus!(($($moves)*) -> (- $($count)*))
        });
        // -+       抵消输出尾的 -
        ((+ $($moves: tt)*) -> (- $($count: tt)*)) => ({
            abacus!(@log left = [+ $($moves)*], right = [- $($count)*]);
            abacus!(($($moves)*) -> ($($count)*))
        });
        // ++ 或 _+ 新添输出尾的 +
        ((+ $($moves: tt)*) -> ($($count: tt)*)) => ({
            abacus!(@log left = [+ $($moves)*], right = [$($count)*]);
            abacus!(($($moves)*) -> (+ $($count)*))
        });
        // 全部 +- 抵消为空
        (() -> ()) => ({
            abacus!(@log left = [], right = []);
            0
        });
        // +- 个数不一致，抵消后还有剩余
        (() -> ($($count: tt)+)) => ({
            abacus!(@log left = [], right = [$($count)*]);
            [$(stringify!($count)),*].iter().filter_map(|&token| Some(if token == "+" {
                1
            } else {
                -1
            })).sum::<i32>()
        });
    }
    assert_eq!(abacus!((++-+-+++--++---++----+) -> ()), 0);
    assert_eq!(abacus!((++-+-+++--++---++----+++) -> ()), 2);
}
