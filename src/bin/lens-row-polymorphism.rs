#[path ="../utils.rs"]
#[macro_use]
mod utils;
use ::std::{error::Error, ops::AddAssign};
use ::lens_rs::{Lens, LensMut, LensRef, optics, Optics, Prism, PrismMut, PrismRef, Review, TraversalMut, TraversalRef};
/**
 * 所谓“行·多态”就是基于“鸭子类型”的多态化。即，只要数据结构（形状）相似，不管其类型命名是否一致，程序都将它们视作相
 * 互兼容的（甚至同一）数据类型来处理。打趣地话，只要扁嘴·长蹼的活物都将被视作鸭子，即便它还可能是鸭嘴兽。
 */
fn main() -> Result<(), Box<dyn Error>> {
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    { // 行多态·场景一：从同一个数据结构实例，根据不同的路径，从不同的位置，拾取/修改子数据结构。
        let mut x: (i32, Result<(Vec<Option<(String, i32)>>, i32), ()>, Vec<i32>, Result<(i32,), String>) = (
            1,
            Ok((
                vec![
                    Some(("a".to_string(), 2)),
                    None,
                    Some(("b".to_string(), 3)),
                ],
                4,
            )),
            vec![1, 2, 3],
            Err("错误提示".to_string())
        );
        let optics1: Optics![_2.[usize]] = optics!(_2.[1]);
        // 因为使用“透镜`Lens`”路径，由路径指向的目标值在数据结构内必须存在。
        fn must_have_i32<P, V, T>(t: &mut T, lens: P)
        where V: AddAssign<i32>,
              T: LensMut<P, V> {
            *t.view_mut(lens) += 1;
        }
        // - 路径目标一定存在
        compare_log!(must_have_i32(&mut x, optics!(_0)); x);
        compare_log!(must_have_i32(&mut x, optics1); x);
        // - 路径目标可能不存在的情况是处理不了的，且会导致编译失败。
        // - 路径目标一定不存在的情况是处理不了的，且会导致编译失败。
        // 因为使用“棱镜`Prism`”路径，由路径指向的目标值在数据结构内允许不存在。
        fn may_have_i32<P, V, T>(t: &mut T, prism: P)
        where V: AddAssign<i32>,
              T: PrismMut<P, V> {
            t.preview_mut(prism).map(|x| {
                *x += 1
            });
        }
        // - 路径目标不存在
        compare_log!(may_have_i32::<_, i32, _>(&mut x, optics!(_11)); x);
        compare_log!(may_have_i32::<_, i32, _>(&mut x, optics!(_1.Ok._11)); x);
        compare_log!(may_have_i32::<_, i32, _>(&mut x, optics!(_3.Ok._1)); x);
        // - 路径目标存在
        compare_log!(may_have_i32(&mut x, optics!(_1.Ok._1)); x);
        compare_log!(may_have_i32(&mut x, optics!(_0)); x);
        compare_log!(may_have_i32(&mut x, optics1); x);
        // 因为使用“棱镜`Traversal`”路径，由路径指向的目标值在数据结构内一定是集合，哪怕是单元素集合，甚至空集合。
        fn may_have_multi_i32<P, V, T>(t: &mut T, traversal: P)
        where V: AddAssign<i32>,
              T: TraversalMut<P, V> {
            t.traverse_mut(traversal).into_iter().for_each(|x| { // 遍历每一个路径匹配项
                *x += 1
            });
        }
        // - 路径目标可能存在多个或一个都没有，但不确定
        compare_log!(may_have_multi_i32(&mut x, optics!(_1.Ok._0._mapped.Some._1)); x);
        compare_log!(may_have_multi_i32(&mut x, optics!(_2._mapped)); x);
        // - 路径目标可能存在，但不确定
        compare_log!(may_have_multi_i32::<_, i32, _>(&mut x, optics!(_3.Ok._1)); x); // 之不存在
        compare_log!(may_have_multi_i32(&mut x, optics!(_1.Ok._1)); x); // 之存在
        // - 路径目标一定存在
        compare_log!(may_have_multi_i32(&mut x, optics!(_0)); x);
        compare_log!(may_have_multi_i32(&mut x, optics1); x);
        // - 路径目标一定不存在的情况是处理不了的，且会导致编译失败。
    }
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    { // 行多态·场景二：根据相同的路径，从不同类型（但形状相似）的数据结构实例，拾取/修改子数据结构。
        #[derive(Lens)]
        struct Foo<A, B> {
            #[optic]
            a: A,
            #[optic]
            b: B,
        }
        #[derive(Lens)]
        struct Bar {
            #[optic]
            a: String,
            #[optic]
            c: i32,
        }
        // 上述定义的两个数据类型`Foo`与`Bar`有着相似的形状。即，它们都有相同的字段`a`。
        let foo = (0, Foo {
            a: "abc",
            b: 12_f64
        });
        let bar = ("123", Bar {
            a: "def".to_string(),
            c: 12
        });
        // 因为使用“透镜”路径，要求目标字段在数据结构内必须存在。
        fn must_have_field_a<V, T>(t: &T) -> &V
        where T: LensRef<Optics![_1.a], V> {
            t.view_ref(optics!(_1.a))
        }
        println!("{:29}{:?}", "Foo 数据结构内一定要存在 a 字段的【单值】", must_have_field_a(&foo));
        println!("{:29}{:?}", "Bar 数据结构内一定要存在 a 字段的【单值】", must_have_field_a(&bar));
        // 因为使用“棱镜`Prism`”路径，允许目标字段在数据结构内不存在。
        fn may_has_field_c<V, T>(t: &T) -> Option<&V>
        where T: PrismRef<Optics![_1.c], V> {
            t.preview_ref(optics!(_1.c))
        }
        println!("{:29}{:?}", "Foo 数据结构内不存在 a 字段", may_has_field_c::<i32, _>(&foo));
        println!("{:29}{:?}", "Bar 数据结构内存在 a 字段", may_has_field_c(&bar));
    }
    Ok(())
}
