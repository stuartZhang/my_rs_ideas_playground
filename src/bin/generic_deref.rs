mod generic_deref {
    use ::std::ops::{ Deref, DerefMut };

    pub struct GenericDeref<T> {
        value: T
    }
    impl<T> GenericDeref<T> {
        pub fn new(value: T) -> Self {
            GenericDeref { value }
        }
    }
    impl<T> Deref for GenericDeref<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl<T> DerefMut for GenericDeref<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.value
        }
    }
}
use ::std::{ cmp::Ordering, hash::{ DefaultHasher, Hash, Hasher } };
use generic_deref::GenericDeref;
type Deref1<'a> = GenericDeref<&'a str>;
type Deref2<'a> = GenericDeref<Deref1<'a>>;
macro_rules! assert_hash {
    [@hash $value: expr] => ({
        let mut hasher = DefaultHasher::new();
        $value.hash(&mut hasher);
        hasher.finish()
    });
    ($($value: expr),+) => {
        assert_eq!( $( assert_hash!(@hash $value) ),+ );
    };
}
fn test_auto_deref(_: &str) {}
fn main() {
    let x = Deref2::new(Deref1::new("a"));
    // 1. 作为函数调用的实参，从 &Deref2 自动解引用
    //    至函数的形参 &str
    test_auto_deref(&x);
    // 2. 作为成员方法调用的 self，从 Deref2 自动解
    //    引用至 &str 和找到目标成员方法
    assert_eq!("A", x.to_uppercase());
    // 3. 智能指针自带 Borrow<_> 与 BorrowMut<_> 的能力
    //    3.1 相同的 trait Eq 实现
    assert!(x.eq("a"));
    //    3.2 相同的 trait Ord 实现
    assert_eq!(Ordering::Equal, x.cmp("a"));
    //    3.3 相同的 trait Hash 实现
    assert_hash!(x, "a");
}