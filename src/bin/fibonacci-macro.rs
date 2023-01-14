mod fibonacci {
    use ::std::{iter::Iterator, mem};
    use index_offset::IndexOffset;
    type ValueType = u64;
    const VALUE_COUNT: usize = 2;
    #[derive(Clone)]
    pub struct Fibonacci {
        // 缓存序列中的最后2个值。然后，根据被缓存下来的值，计算序列下一个新值。
        init_values: [ValueType; VALUE_COUNT],
        sequence_length: usize
    }
    impl Default for Fibonacci {
        fn default() -> Self {
            Fibonacci {
                init_values: [0 as ValueType, 1 as ValueType],
                sequence_length: 0
            }
        }
    }
    impl Iterator for Fibonacci {
        type Item = ValueType;
        #[inline(always)]
        fn next(&mut self) -> Option<Self::Item> {
            let sequence_length = self.sequence_length;
            self.sequence_length += 1;
            if sequence_length < VALUE_COUNT {
                return Some(self.init_values[sequence_length]);
            }
            let init_values = IndexOffset::new(&self.init_values, &sequence_length);
            let mut next_value = init_values[sequence_length - 1] + init_values[sequence_length - 2];
            for index in (0..VALUE_COUNT).rev() {
                // 因为【引用】的【非词法生命周期】，所以上面【只读引用】`&self.init_values`
                // 并不会阻碍这里【可修改引用】`&mut self.init_values[index]`的使用。
                mem::swap(&mut self.init_values[index], &mut next_value);
            }
            Some(self.init_values[VALUE_COUNT - 1])
        }
    }
    mod index_offset {
        use ::std::ops::Index;
        use super::{ValueType, VALUE_COUNT};
        pub struct IndexOffset<'a> {
            slice: &'a [ValueType; VALUE_COUNT],
            sequence_length: &'a usize
        }
        impl<'a> IndexOffset<'a> {
            pub fn new(slice: &'a [ValueType; VALUE_COUNT], sequence_length: &'a usize) -> Self {
                IndexOffset {
                    slice,
                    sequence_length
                }
            }
        }
        impl<'a> Index<usize> for IndexOffset<'a> {
            type Output = ValueType;
            #[inline(always)]
            fn index(&self, index: usize) -> &Self::Output {
                // 无符号数的环绕减法避免负值溢出
                if self.sequence_length.wrapping_sub(index) > VALUE_COUNT {
                    panic!("未缓存 {} 索引位置上的值", index);
                }
                &self.slice[index % VALUE_COUNT]
            }
        }
    }
}
macro_rules! sequence {
    // 1. 缓存序列中的最后`$length`个值。
    // 2. 根据被缓存下来的值，计算序列下一个新值。
    // 3. 懒生成一个定制规则的数字序列
    ($array: ident [$type : ty; $length: ident] = $($init: expr),+; ...; $eval: expr) => [{
        mod sequence {
            use ::std::{iter::Iterator, mem};
            use index_offset::IndexOffset;
            type ValueType = $type;
            const VALUE_COUNT: usize = sequence!(@len $($init),+);
            #[derive(Clone)]
            pub struct Sequence {
                // 缓存序列中的最后`VALUE_COUNT`个值。然后，根据被缓存下来的值，计算序列下一个新值。
                init_values: [ValueType; VALUE_COUNT],
                sequence_length: usize
            }
            impl Default for Sequence {
                fn default() -> Self {
                    Sequence {
                        init_values: [$($init),+],
                        sequence_length: 0
                    }
                }
            }
            impl Iterator for Sequence {
                type Item = ValueType;
                #[inline(always)]
                fn next(&mut self) -> Option<Self::Item> {
                    let $length = self.sequence_length;
                    self.sequence_length += 1;
                    if $length < VALUE_COUNT {
                        return Some(self.init_values[$length]);
                    }
                    let $array = IndexOffset::new(&self.init_values, &$length);
                    let mut next_value = $eval;
                    for index in (0..VALUE_COUNT).rev() {
                        // 因为【引用】的【非词法生命周期】，所以上面【只读引用】`&self.init_values`
                        // 并不会阻碍这里【可修改引用】`&mut self.init_values[index]`的使用。
                        mem::swap(&mut self.init_values[index], &mut next_value);
                    }
                    Some(self.init_values[VALUE_COUNT - 1])
                }
            }
            mod index_offset {
                use ::std::ops::Index;
                use super::{ValueType, VALUE_COUNT};
                pub struct IndexOffset<'a> {
                    slice: &'a [ValueType; VALUE_COUNT],
                    sequence_length: &'a usize
                }
                impl<'a> IndexOffset<'a> {
                    pub fn new(slice: &'a [ValueType; VALUE_COUNT], sequence_length: &'a usize) -> Self {
                        IndexOffset {
                            slice,
                            sequence_length
                        }
                    }
                }
                impl<'a> Index<usize> for IndexOffset<'a> {
                    type Output = ValueType;
                    #[inline(always)]
                    fn index(&self, index: usize) -> &Self::Output {
                        // 无符号数的环绕减法避免负值溢出
                        if self.sequence_length.wrapping_sub(index) > VALUE_COUNT {
                            panic!("未缓存 {} 索引位置上的值", index);
                        }
                        &self.slice[index % VALUE_COUNT]
                    }
                }
            }
        }
        sequence::Sequence::default()
    }];
    // 利用`Incremental TT Muncher`与`Push-down Accumulation`设计模式，
    // 计算宏循环结构的迭代次数。
    (@len ($muncher: tt) -> ($count: expr)) => {
        1 + $count
    };
    (@len ($muncher: tt, $($remainder: tt),+) -> ($count: expr)) => {
        sequence!(@len ($($remainder),+) -> (1 + $count))
    };
    (@len $($remainder: tt),+) => {
        sequence!(@len ($($remainder),+) -> (0))
    };
}
use fibonacci::Fibonacci;
fn main() {
    let sequence = Fibonacci::default().take(10).collect::<Vec<u64>>();
    dbg!(sequence);
    // 【斐波纳契】数列
    let sequence = sequence!(cache_slice[u64; length] = 0, 1; ...; cache_slice[length - 1] + cache_slice[length - 2]).take(10).collect::<Vec<u64>>();
    dbg!(sequence);
    // 其它数列
    let sequence = sequence!(cache_slice[f64; length] = 1_f64; ...; cache_slice[length - 1] * length as f64).take(10).collect::<Vec<f64>>();
    dbg!(sequence);
}