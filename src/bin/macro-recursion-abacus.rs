fn main() {
    macro_rules! count_tokens {
        (@replace $_t: tt @with $sub: expr) => { $sub };
        ($($tts: tt)*) => { [$( count_tokens!(@replace $tts @with ()) ),*].len() };
    }
    macro_rules! abacus {
        // 日志记录·内部规则
        (@stringify [$l1: tt  $($left: tt)+]) => {
            (&[concat!("[", stringify!($l1), "]"), $(stringify!($left)),+] as &[&str]).join("")
        };
        (@stringify [$l1: tt]) => {
            concat!("[", stringify!($l1), "]")
        };
        (@stringify []) => {
            ""
        };
        (@log left = [$($left: tt)*], right = [$($right: tt)*]) => {
            let left = abacus!(@stringify [$($left)*]);
            let right = abacus!(@stringify [$($right)*]);
            println!("最新吐 {left:>24}  最后吞 {right:>7}");
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
            count_tokens!($($count)+)
        });
    }
    println!("* 最左是栈顶，最右是栈底");
    println!("* 最新吐与最后吞符号都被包裹在一对`[..]`内");
    println!();
    println!("{:_^29}  {:_^12}", "吐栈", "吞栈");
    assert_eq!(abacus!((++-+-+++--++---++----+) -> ()), 0);
    println!("{:_^29}  {:_^12}", "吐栈", "吞栈");
    assert_eq!(abacus!((++--+--+---) -> ()), 3);
}
