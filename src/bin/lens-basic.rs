#[path ="../utils.rs"]
#[macro_use]
mod utils;
use ::std::error::Error;
use ::lens_rs::{LensMut, LensRef, optics, Optics, PrismMut, PrismRef, Review, TraversalMut, TraversalRef};
fn main() -> Result<(), Box<dyn Error>> {
    let mut x: (i32, Result<(Vec<Option<(String, i32)>>, i32), ()>, Vec<i32>, Option<String>) = (
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
        None
    );
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    let optics1: Optics![_1.Ok._1] = optics!(_1.Ok._1);
    // 类似于`R.set(R.lensPath(路径), 值, 数据结构)`，修改数据结构内指定位置上的一个值。
    { // + `view_mut()`：目标值一定存在且仅有一个
        // - 虽然目标值不能是【枚举值】下的内部数据（比如，`_1.Ok._1`）
        compare_log!(*x.view_mut(optics!(_0)) += 1; x);
        // - 但目标值可以是【枚举值】自身。即，在路径内不能显示地看到`Some | None | Ok | Err`保留字。
        compare_log!(let _ = x.view_mut(optics!(_3)).replace("F".to_string()); x);
        compare_log!(let _ = x.view_mut(optics!(_3)).as_mut().map(|s| {
            *s = "E".to_string();
        }); x);
        // - 目标值也可以是【集合】内被索引的单个【元素】
        compare_log!(*x.view_mut(optics!(_2.[0])) *= 2; x);
    }
    { // + `preview_mut()`：目标值不一定存在，且至多一个。
        // - 在“路径”内可包含`Some | None | Ok | Err`保留字，和修改【枚举值】下的内部数据。
        compare_log!(let _ = x.preview_mut(optics1).map(|n| {
            *n *= 2;
        }); x);
        compare_log!(let _ = x.preview_mut(optics!(_1.Ok._0.[0].Some._0)).map(|s| {
            *s = "c".to_string();
        }); x);
        // - 相比于上面的`x.view_mut(optics!(_3)).as_mut()`，`preview_mut(_3.Some)`隐式执行了`.as_mut()`操作。
        compare_log!(let _ = x.preview_mut(optics!(_3.Some)).map(|s| {
            *s = s.to_lowercase();
        }); x);
        // - 完全兼容于`view_mut()`，因为“一定存在的”【单值】也能被当作`Some()`枚举值来处理。
        compare_log!(let _ = x.preview_mut(optics!(_0)).map(|n| {
            *n += 3;
        }); x);
    }
    { // + `traverse_mut()`：目标值不一定存在或有多个，因为在“路径”内包含【集合·片段】。
        // - `_mapped.Some`意味着【遍历 ➔ 过滤】出全部`Some`元素
        compare_log!(let _ = x.traverse_mut(optics!(_1.Ok._0._mapped.Some._0)).into_iter().for_each(|s| {
            *s = s.to_uppercase();
        }); x);
        // - `_mapped`意味着【遍历】全部元素
        compare_log!(let _ = x.traverse_mut(optics!(_2._mapped)).into_iter().for_each(|n: &mut i32| {
            *n -= 1;
        }); x);
        // - `[N..]`索引一段【切片】，但不可整体赋值，因为`Slice: ?Sized`
        compare_log!(let _ = x.traverse_mut(optics!(_2.[1..])).into_iter().for_each(|n: &mut [i32]| {
            n[0] = 10;
            n[1] = 13;
            // *n = *[10, 13].as_mut_slice(); // `Slice: ?Sized`导致编译错误。
        }); x);
        // - 完全兼容于`preview_mut()`，因为【枚举值】也能被当作至多包含一个元素项的【集合】来处理。
        compare_log!(let _ = x.traverse_mut(optics1).into_iter().for_each(|n: &mut i32| {
            *n += 3;
        }); x);
        compare_log!(let _ = x.traverse_mut(optics!(_2.[1])).into_iter().for_each(|n: &mut i32| {
            *n += 2;
        }); x);
        // - 完全兼容于`view_mut()`，因为【一定存在值】也能被当作仅只包含一个元素项的【集合】来处理。
        compare_log!(let _ = x.traverse_mut(optics!(_0)).into_iter().for_each(|n: &mut i32| {
            *n += 3;
        }); x);
        compare_log!(let _ = x.traverse_mut(optics!(_2.[1])).into_iter().for_each(|n: &mut i32| {
            *n += 1;
        }); x);
    }
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    // 类似于`R.view(R.lensPath(路径), 数据结构)`，拾取出数据结构内指定位置上的一个值。
    { // + `view_ref()`：目标值一定存在且仅有一个
        println!("{:29}{:?}", "目标值是一定存在的【单值】", x.view_ref(optics!(_0)));
        println!("{:31}{:?}", "目标值是【枚举值】自身", x.view_ref(optics!(_3)));
        println!("{:32}{:?}", "目标值是【集合】自身", x.view_ref(optics!(_2)));
        println!("{:23}{:?}", "目标值是【集合】内被索引的一段【切片】", x.view_ref(optics!(_2.[1..])));
        println!("{:23}{:?}", "目标值是【集合】内被索引的单个【元素】", x.view_ref(optics!(_2.[1..].[1])));
    }
    { // + `preview_ref()`：在“路径”内包含`Some | None | Ok | Err`保留字，和拾取出【枚举值】下的内部数据。
        println!("{:25}{:?}", "目标值是【枚举值】下的直接内部数据", x.preview_ref(optics!(_3.Some)));
        println!("{:25}{:?}", "目标值是【枚举值】下的嵌套内部数据", x.preview_ref(optics!(_1.Ok._0.[0].Some._0)));
    }
    { // + `traverse_ref()`：在“路径”内包含【集合】，和拾取出【集合】下的内部数据。
        println!("{:32}{:?}", "目标值是【集合】自身", x.traverse_ref(optics!(_2._mapped)));
        println!("{:29}{:?}", "目标值是【集合】的过滤子集", x.traverse_ref(optics!(_1.Ok._0._mapped.Some)));
        println!("{:23}{:?}", "目标值是【集合】内被索引的一段【切片】", x.traverse_ref(optics!(_2.[1..])));
        println!("{:23}{:?}", "目标值是【集合】内被索引的单个【元素】", x.traverse_ref(optics!(_2.[1])));
    }
    // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    { // + 根据“路径”从零构造出一个数据结构实例
        let y: Result<Option<Result<(), (i32, i32)>>, ()> = Review::review(optics!(Ok.Some.Err), (1, 2));
        println!("{:34}{:?}", "构造数据结构实例", y);
    }
    Ok(())
}
