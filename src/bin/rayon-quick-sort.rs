/// 关于此函数形参 v 的 trait bound，我有话要讲。
/// Send trait 是“自动特征”（或也称为“派生特征”）。rustc 会自动扩散 Send trait 实现从
/// 类型定义（比如，T）至 该类型的引用（比如，&T 与 &mut T）。所以，有如下 Send trait 实现扩散链条
/// <T: Send> → (<&T: Send> 和 <&mut T: Send>) → <T: Sync> → <[T]: Sync> → (<&[T]: Sync> 和 <&mut [T]: Sync>)。
/// 于是，正是因为 (<&[T]: Sync> 和 <&mut [T]: Sync>)，所以由 rayon::join() 生成的多个线程引用与修改的是同一个 Vec<i32> 实例
fn quick_sort<T: Ord + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    fn partition<T: Ord>(v: &mut [T]) -> usize {
        let pivot = v.len() - 1;
        let mut i = 0;
        for j in 0..pivot {
            if v[j] <= v[pivot] {
                v.swap(i, j);
                i += 1;
            }
        }
        v.swap(i, pivot);
        i
    }
    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);

    rayon::join(|| quick_sort(lo),
                || quick_sort(hi));
}
fn main() {
    let mut numbers: Vec<i32> = (0..1000).rev().collect();
    quick_sort(&mut numbers);
    println!("Sorted: {:?}", &numbers[..10]);
}