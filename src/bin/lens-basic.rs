use ::std::error::Error;
use ::lens_rs::{LensMut, optics, PrismMut, TraversalMut};
#[path ="../utils.rs"]
#[macro_use]
mod utils;
fn main() -> Result<(), Box<dyn Error>> {
    let mut x: (i32, Result<(Vec<Option<(String, i32)>>, i32), ()>, Vec<i32>) = (
        1,
        Ok((
            vec![
                Some(("a".to_string(), 2)),
                None,
                Some(("b".to_string(), 3)),
            ],
            4,
        )),
        vec![1, 2, 3]
    );
    // 类似于`R.set(R.lensPath(路径), 值, 数据结构)`，修改数据结构内指定位置上的一个值。
    // + 场景一：目标值一定存在且仅有一个，因为在“路径”内不包含【枚举值】与【集合·片段】。
    compare_log!(*x.view_mut(optics!(_0)) += 1; x);
    compare_log!(*x.view_mut(optics!(_2.[0])) *= 2; x); // 索引集合内的单个值，也按“一定存在”处理。
    // + 场景二：目标值不一定存在，因为在“路径”内包含【枚举值】。
    compare_log!(let _ = x.preview_mut(optics!(_1.Ok._1)).map(|v| {
        *v *= 2
    }); x);
    compare_log!(let _ = x.preview_mut(optics!(_1.Ok._0.[0].Some._0)).map(|s| {
        *s = s.to_uppercase()
    }); x);
    // + 场景三：目标值不一定存在，因为在“路径”内包含【集合·片段】。
    //   `_mapped.Some`意味着【遍历 ➔ 过滤出】全部`Some`元素
    compare_log!(let _ = x.traverse_mut(optics!(_1.Ok._0._mapped.Some._0)).into_iter().for_each(|s| {
        *s = s.to_lowercase()
    }); x);
    //   【枚举值】也能被当作是【集合·片段】来处理。即，至多有一个元素的集合片段。
    compare_log!(let _ = x.traverse_mut(optics!(_1.Ok._0.[0].Some._0)).into_iter().for_each(|s| {
        *s = s.to_uppercase()
    }); x);
    Ok(())
}
