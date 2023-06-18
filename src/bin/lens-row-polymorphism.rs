#[path ="../utils.rs"]
#[macro_use]
mod utils;
use ::std::{error::Error, ops::AddAssign};
use ::lens_rs::{LensMut, LensRef, optics, Optics, Prism, PrismMut, PrismRef, Review, TraversalMut, TraversalRef};
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
        // 在数据结构中，由路径指向的目标值必须存在，因为“透镜`Lens`”。
        fn must_have_i32<P, V, T: LensMut<P, V>>(t: &mut T, lens: P)
        where V: AddAssign<i32> {
            *t.view_mut(lens) += 1;
        }
        // - 路径目标一定存在
        compare_log!(must_have_i32(&mut x, optics!(_0)); x);
        compare_log!(must_have_i32(&mut x, optics1); x);
        // - 路径目标可能不存在的情况是处理不了的，且会导致编译失败。
        // - 路径目标一定不存在的情况是处理不了的，且会导致编译失败。
        // 在数据结构中，由路径指向的目标值既可存在也可不存在，因为“棱镜`Prism`”。
        fn may_have_i32<P, V, T: PrismMut<P, V>>(t: &mut T, prism: P)
        where V: AddAssign<i32> {
            t.preview_mut(prism).map(|x| {
                *x += 1
            });
        }
        // - 路径目标可能存在，但不确定
        compare_log!(may_have_i32::<_, i32, _>(&mut x, optics!(_3.Ok._1)); x); // 之不存在
        compare_log!(may_have_i32(&mut x, optics!(_1.Ok._1)); x); // 之存在
        // - 路径目标一定存在
        compare_log!(may_have_i32(&mut x, optics!(_0)); x);
        compare_log!(may_have_i32(&mut x, optics1); x);
        // - 路径目标一定不存在的情况是处理不了的，且会导致编译失败。
        // 在数据结构中，由路径寻找指向的目标值是一个集合，因为“棱镜`Traversal`”。
        fn may_have_multi_i32<P, V, T: TraversalMut<P, V>>(t: &mut T, traversal: P)
        where V: AddAssign<i32> {
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
    {

    }
    Ok(())
}
