#[path ="../utils.rs"]
#[macro_use]
mod utils;
use ::std::error::Error;
use ::lens_rs::{Lens, LensMut, optics, Prism, Review};
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
    struct Baz<'a, A, B, C>{
        #[optic(ref)]
        immutable: &'a A,
        #[optic(mut)]
        mutable: &'a mut B,
        #[optic]
        ownership: C
    }
    #[derive(Debug, Lens)]
    struct Custom<'a, L, R, F, S, E, A = F, B = S, C = A, D = B> {
        #[optic]
        either: Either<L, R>,
        #[optic]
        tuple: Tuple<F, S>,
        #[optic]
        foo: Foo<A, B>,
        #[optic]
        baz: Baz<'a, C, D, E>
    }
    let array1 = vec![1, 2, 3];
    let mut array2 = vec![5, 6, 8];
    let mut custom = Custom {
        either: Either::Right::<i32, _>("right".to_string()),
        tuple: Tuple(12_u8, 100_u32),
        foo: Foo {
            f1: 45_i64,
            f2: "f2".to_string()
        },
        baz: Baz {
            immutable: &array1,
            mutable: &mut array2,
            ownership: ["1", "2", "3"]
        }
    };
    // 类似于`R.set(R.lensPath(路径), 值, 数据结构)`，修改数据结构内指定位置上的一个值。
    { // + `view_mut()`：目标值一定存在且仅有一个
        // - 虽然目标值不能是【枚举值】下的内部数据（比如，`_1.Ok._1`）
        compare_log!(*custom.view_mut(optics!(tuple._0)) += 1; custom);
        // - 但目标值可以是【枚举值】自身。即，在路径内不能显示地看到`Some | None | Ok | Err`保留字。
        compare_log!(*custom.view_mut(optics!(either)) = Either::Left(15_i32); custom);
        // - 目标值也可以是【集合】内被索引的单个【元素】。但，对【集合】的引用/指针无效。
        compare_log!(*custom.view_mut(optics!(baz.ownership.[0])) = "12"; custom);
    }


    Ok(())
}