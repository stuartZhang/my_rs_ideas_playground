#[path = "../utils.rs"]
#[macro_use]
mod utils;

use ::ambassador::{delegatable_trait, delegate_remote, delegate_to_remote_methods, Delegate};
use ::std::{borrow::Borrow, cmp::{Eq, Ord}, fmt::Debug, hash::{Hash, BuildHasher}};
/// 【外部】【泛型·结构体】委托类型
use ::std::collections::{BTreeMap, HashMap};
/// 【本地】委托`trait`
#[delegatable_trait]
pub trait Map: Debug {
    type K;
    type V;
}
impl<K, V, S> Map for HashMap<K, V, S> where K: Debug,
                                             V: Debug  {
    type K = K;
    type V = V;
}
impl<K, V> Map for BTreeMap<K, V> where K: Debug,
                                        V: Debug  {
    type K = K;
    type V = V;
}
/// 【本地】【泛型】委托`trait`
#[delegatable_trait]
pub trait Get<X: ?Sized>: Map { // 由于`rustc`自身的[缺陷](https://github.com/rust-lang/rust/issues/20503)，`?Sized`在`where`从句内是不起作用的。
    fn get(&self, k: &X) -> Option<&Self::V>;
}
// 1.【外部】【泛型·结构体】委托类型  | `HashMap<K, V, S>` | `#[delegate(where)]`
// 2. 委托【本地】【泛型】委托`trait` | `Get<Q>`           | `#[delegate(generic)]`
// 3. 给它·自己                     | `HashMap<K, V, S>` | `#[delegate(target = "self")]`
//    - 虽然`HashMap<K, V, S>`并没有实现`Get<Q> trait`，
//    - 但在`HashMap<K, V, S>`的`inherent methods`里已经包含了`Get<Q> trait`定义的
//      [get](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get)成员方法。
//    - 否则，`unconditional_recursion`编译错误就会被抱怨。
// 4. 在委托之后，`HashMap<K, V, S>`也就实现了`Get<Q> trait`。
#[delegate_remote]
#[delegate(Get<X>, target = "self", generics = "X: ?Sized", where = "K: Hash + Eq + Borrow<X> + Debug, V: Debug, S: BuildHasher, X: Hash + Eq")]
struct HashMap<K, V, S>();
// 1.【外部】【泛型·结构体】委托类型  | `BTreeMap<K, V>` | `#[delegate(where)]`
// 2. 委托【本地】【泛型】委托`trait` | `Get<Q>`         | `#[delegate(generic)]`
// 3. 给它·自己                     | `BTreeMap<K, V>` | `#[delegate(target = "self")]`
//    - 虽然`BTreeMap<K, V>`并没有实现`Get<Q> trait`，
//    - 但在`BTreeMap<K, V>`的`inherent methods`里已经包含了`Get<Q> trait`定义的
//      [get](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.get)成员方法。
//    - 否则，`unconditional_recursion`编译错误就会被抱怨。
// 4. 在委托之后，`BTreeMap<K, V>`也就实现了`Get<Q> trait`。
#[delegate_remote]
#[delegate(Get<X>, target = "self", generics = "X: ?Sized", where = "K: Ord + Borrow<X> + Debug, V: Debug, X: Ord")]
struct BTreeMap<K, V>();
// 1. 【本地】【泛型·枚举类】委托类型 | `enum Either<A, B>` | `#[delegate(where)]`
// 2. 委托【本地】【泛型】委托`trait` | `Get<Q>`           | `#[delegate(generic)]`
// 3. 给它的枚举值的字段             | `A`与`B`            | 单字段·结构体，所以免`#[delegate(target)]`属性
#[derive(Delegate, Debug)]
#[delegate(Map)] // `supertrait`单独委托
#[delegate(Get<X>, generics = "X: ?Sized", where = "B: Map<K=A::K, V=A::V>")]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}
// 1. 借助【覆盖·实现】
// 2.【外部】智能指针·类型    | `Box<M> where M: Map` | `#[delegate_to_remote_methods]`
// 3. 委托【本地】委托`trait` | `Map`                 |
// 4. 至成员方法·返回值       | `&M`                  | `#[delegate(target_ref = "deref")]`
#[delegate_to_remote_methods]
#[delegate(Map, target_ref = "deref")]
impl<M: ?Sized + Map> Map for Box<M> {
    fn deref(&self) -> &M;
}
main!{{
    // 1. Either -> 委托`Get<T>` -> `HashMap`和`BTreeMap`
    // 2. `HashMap`和`BTreeMap`-> 委托`Get<T>` -> 它们自己
    let my_map = Either::<HashMap<&'static str, u32>, BTreeMap<&'static str, u32>>::Left([("a", 1)].into());
    assert_eq!(my_map.get("a"), Some(&1));
    // 1. Box    -> 委托`Map`    -> Either
    // 2. Either -> 委托`Get<T>` -> `HashMap`和`BTreeMap`
    // 3. `HashMap`和`BTreeMap`-> 委托`Get<T>` -> 它们自己
    let boxed: Box<dyn Map<K = &'static str, V = u32>> = Box::new(my_map);
    takes_map(&boxed);
}}
fn takes_map(m: &(impl Map<K = &'static str, V = u32> + Debug)) {
    dbg!(m);
}