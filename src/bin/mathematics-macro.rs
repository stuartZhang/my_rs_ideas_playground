/*
    实现与`js`中`Math.min(...)`函数相同的功能。
    即，从任意数量的实参中，筛选出最大/小值。
    支持多种实参形式：
    （1）字面量数字
    （2）表达式
    （3）键-值对
    （4）键-表达式对
 */
macro_rules! mathematics {
    // -------- 递归执行主体 --------
    /*
        从不确定长度的输入`tt`序列中，每次仅取出第一个`tt`处理。
        而，将剩余的`tt`“打包”成一个`tt`循环和传递给递归调用。
     */
    ($action: ident; $head: tt, $($tail: tt)+) => [{
        #[cfg(debug_assertions)]
        println!(r#"{:17}| $tail = "{}""#, format!("$head = {}", stringify!($head)), stringify!($($tail)+));
        ::std::cmp::$action($head, mathematics!($action; $($tail)+))
    }];
    /*
        从不确定长度的输入`tt`序列中，每次仅取出前三个输入`tt`处理。
        即，键 = 值。而，将剩余的`tt`“打包”成一个`tt`循环和传递给递
        归调用。
     */
    ($action: ident; $name: ident = $head: tt, $($tail: tt)+) => [{
        #[cfg(debug_assertions)]
        println!(r#"{:17}| $tail = "{}""#, format!("{} = {}", stringify!($name), stringify!($head)), stringify!($($tail)+));
        ::std::cmp::$action($head, mathematics!($action; $($tail)+))
    }];
    // 将 键 = 表达式 降级为 键 = 值，再递归处理
    ($action: ident; $name: ident = $head: expr, $($tail: tt)+) => [{
        mathematics!($action; $name = $head, $($tail)+)
    }];
    /*
        从不确定长度的输入`tt`序列中，每次仅取出第一个输入`expr`来处理。
        而，将剩余的`tt`“打包”成一个`tt`循环和传递给递归调用。
    */
    ($action: ident; $head: expr, $($tail: tt)+) => [{
        #[cfg(debug_assertions)]
        println!(r#"{:17}| $tail = "{}""#, format!("$head = {}", stringify!($head)), stringify!($($tail)+));
        ::std::cmp::$action($head, mathematics!($action $($tail)+))
    }];
    // -------- 递归结束条件 --------
    /*
        不确定长度·输入`tt`序列的最后一个`tt`直接展开。
    */
    ($action: ident; $head: tt) => [{
        #[cfg(debug_assertions)]
        println!("{:17}|", format!("$head = {}", stringify!($head)));
        $head
    }];
    /*
        不确定长度·输入`tt`序列的最后三个连续的`tt`（键 = 值）。
        匹配出【值】并展开。
    */
    ($action: ident; $name: ident = $head: tt) => [{
        #[cfg(debug_assertions)]
        println!("{:17}|", format!("{} = {}", stringify!($name), stringify!($head)));
        $head
    }];
    // 将 键 = 表达式 降级为 键 = 值，再递归处理
    ($action: ident; $name: ident = $head: expr) => [{
        mathematics!($action; $name = $head)
    }];
    /*
        不确定长度·输入`tt`序列的最后一个`expr`直接展开。
    */
    ($action: ident; $head: expr) => [{
        #[cfg(debug_assertions)]
        println!("{:17}|", format!("$head = {}", stringify!($head)));
        $head
    }];
}
fn main() {
    println!("最小值是 {}", mathematics!(min; 1));
    println!("最小值是 {}", mathematics!(min; 1, operand1 = 2 * 3, 0 * 1));
    println!("最小值是 {}", mathematics!(min; 5, operand1 = 3, operand2 = 4 - 2, operand3 = 10));
    println!("最小值是 {}\n", mathematics!(min; 5, operand1 = 3, operand3 = 10, operand2 = 4 - 3));

    println!("最大值是 {}", mathematics!(max; 1));
    println!("最大值是 {}", mathematics!(max; 1, operand1 = 2 * 3, 0 * 1));
    println!("最大值是 {}", mathematics!(max; 5, operand1 = 3, operand2 = 4 - 2, operand3 = 10));
    println!("最大值是 {}", mathematics!(max; 5, operand1 = 3, operand3 = 10, operand2 = 4 - 3));
}
