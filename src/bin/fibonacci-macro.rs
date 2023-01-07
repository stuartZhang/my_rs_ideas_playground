mod fibonacci {
    use ::std::{iter::Iterator, mem};
    #[derive(Clone, Default)]
    pub struct Fibonacci {
        init_values: [u64; 2],
        length: usize
    }
    impl Iterator for Fibonacci {
        type Item = u64;
        fn next(&mut self) -> Option<Self::Item> {
            let length = self.length;
            self.length += 1;
            if length < 2 {
                self.init_values[length] = length as u64;
                return Some(self.init_values[length]);
            }
            let max_index = self.init_values.len() - 1;
            let init_values = &self.init_values;
            let mut next_value = init_values[max_index] + init_values[max_index - 1];
            for index in (0..=max_index).rev() {
                // 因为【引用】的【非词法生命周期】，所以上面【只读引用】`init_values`
                // 并不会阻碍这里【可修改引用】的使用。
                mem::swap(&mut self.init_values[index], &mut next_value);
            }
            Some(self.init_values[max_index])
        }
    }
}
use fibonacci::Fibonacci;
fn main() {
    let fibonacci = Fibonacci::default();
    let sequence = fibonacci.take(10).collect::<Vec<u64>>();
    dbg!(sequence);
}