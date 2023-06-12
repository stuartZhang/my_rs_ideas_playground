#[path ="../utils.rs"]
#[macro_use]
mod utils;
use ::std::error::Error;
use ::lens_rs::{Lens, LensMut, optics, Prism, PrismMut, Review, TraversalMut};
/*
 * 一定要向`Cargo.toml`文件添加包元数据。
 * ```toml
 * [package.metadata.inwelling]
 * lens-rs_generator = true
 * ```
 */
fn main() -> Result<(), Box<dyn Error>> {
    #[derive(Debug, Review, Prism)]
    enum Either<L, R> {
        #[optic]
        Left(L),
        #[optic]
        Right(R),
    }
    #[derive(Debug, Lens)]
    struct Tuple<A, B>(
        #[optic] A,
        #[optic] B
    );
    #[derive(Debug, Lens)]
    struct Foo<A, B> {
        #[optic]
        f1: A,
        #[optic]
        f2: B,
    }
    #[derive(Debug, Lens)]
    struct Baz<'a, A, B, C, D>{
        #[optic(ref)]
        immutable: &'a A,
        #[optic(mut)]
        mutable: &'a mut B,
        #[optic]
        ownership1: C,
        #[optic]
        ownership2: D
    }
    #[derive(Debug, Lens)]
    struct Custom<'a, L, R, F, S, E, G, A = F, B = S, C = A, D = B> {
        #[optic]
        either1: Either<L, R>,
        #[optic]
        either2: Either<L, R>,
        #[optic]
        tuple: Tuple<F, S>,
        #[optic]
        foo: Foo<A, B>,
        #[optic]
        baz: Baz<'a, C, D, E, G>
    }
    let array1 = vec![1, 2, 3];
    let mut array2 = vec![5, 6, 8];
    let mut custom = Custom {
        either1: Either::Right::<(i32, i32), _>("right".to_string()),
        either2: Either::Right::<(i32, i32), _>("right".to_string()),
        tuple: Tuple(12_u8, 100_u32),
        foo: Foo {
            f1: 45_i64,
            f2: "f2".to_string()
        },
        baz: Baz {
            immutable: &array1,
            mutable: &mut array2,
            ownership1: ["1".to_string(), "2".to_string(), "c".to_string()],
            ownership2: vec!["1".to_string(), "2".to_string(), "c".to_string()]
        }
    };
    // 类似于`R.set(R.lensPath(路径), 值, 数据结构)`，修改数据结构内指定位置上的一个值。
    { // + `view_mut()`：目标值一定存在且仅有一个
        // - 虽然目标值不能是【枚举值】下的内部数据（比如，`_1.Ok._1`）
        compare_log!(*custom.view_mut(optics!(tuple._0)) += 1; custom);
        // - 但目标值可以是【枚举值】自身。即，在路径内不能显示地看到`Some | None | Ok | Err`保留字。
        compare_log!(*custom.view_mut(optics!(either1)) = Either::Left((15, 16)); custom);
        // - 目标值也可以是【集合】内被索引的单个【元素】。
        //   注意：对【集合】的引用/指针无效。
        compare_log!(*custom.view_mut(optics!(baz.ownership1.[0])) = "12".to_string(); custom);
        /* 禁忌：
        （1）就自定义枚举类而言，`trait ::lens_rs::LensMut`没有被默认实现，所以不能像`Option<T>`与`Result<T, E>`
             那样直接定位和修改被封装于枚举值内的【单值】。于是，如下语句都是非法的。
             ```
             let _ = custom.view_mut(optics!(either.Right)).as_mut().map(|s: &mut String| {
                *s = "right_2".to_string();
            });
            let _ = *custom.view_mut(optics!(either.Left._0)).as_mut().map(|n: &mut i32| {
                *n += 10;
            });
            ```
         */
    }
    { // + `preview_mut()`：目标值不一定存在，且至多一个。
        // - 在“路径”内可包含自定义【枚举值】。
        compare_log!(let _ = custom.preview_mut(optics!(either1.Left._0)).map(|n: &mut i32| {
            *n *= 3;
        }); custom);
        compare_log!(let _ = custom.preview_mut(optics!(either2.Right)).map(|s: &mut String| {
            *s = "right_2".to_string();
        }); custom);
    }
    { // + `traverse_mut()`：目标值不一定存在或有多个，因为在“路径”内包含【集合/切片】。
        // - `_mapped`意味着【遍历】全部元素，但仅适用于变长数组
        compare_log!(let _ = custom.traverse_mut(optics!(baz.ownership2._mapped)).into_iter().for_each(|s| {
            *s = s.to_uppercase();
        }); custom);
        // - `[N..]`索引一段【切片】需要对迭代器使用扁平化配合器`.flatten()`。
        compare_log!(let _ = custom.traverse_mut(optics!(baz.ownership2.[1..])).into_iter().flatten().for_each(|s| {
            *s = s.to_lowercase();
        }); custom);
        // - `_mapped`不适用于数组。相反，需要对迭代器使用扁平化配合器`.flatten()`。
        compare_log!(let _ = custom.traverse_mut(optics!(baz.ownership1)).into_iter().flatten().for_each(|s: &mut String| {
            *s = s.to_uppercase();
        }); custom);
        // - 完全兼容于`preview_mut()`，因为【枚举值】也能被当作至多包含一个元素项的【集合】来处理。
        compare_log!(let _ = custom.traverse_mut(optics!(either1.Left._0)).into_iter().for_each(|n: &mut i32| {
            *n += 3;
        }); custom);
        // - 完全兼容于`view_mut()`，因为【一定存在值】也能被当作仅只包含一个元素项的【集合】来处理。
        compare_log!(let _ = custom.traverse_mut(optics!(tuple._0)).into_iter().for_each(|n| {
            *n += 1;
        }); custom);
        compare_log!(let _ = custom.traverse_mut(optics!(baz.ownership1.[2])).into_iter().for_each(|s: &mut String| {
            *s = s.to_lowercase();
        }); custom);
    }
    Ok(())
}