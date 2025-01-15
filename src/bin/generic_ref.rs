mod generic_ref {
    use ::std::convert::{ AsRef, AsMut };

    pub struct GenericRef<T> {
        value: T
    }
    impl<T> GenericRef<T> {
        pub fn new(value: T) -> Self {
            GenericRef { value }
        }
    }
    impl<T> AsRef<T> for GenericRef<T> {
        fn as_ref(&self) -> &T {
            &self.value
        }
    }
    impl<T> AsMut<T> for GenericRef<T> {
        fn as_mut(&mut self) -> &mut T {
            &mut self.value
        }
    }
}
use ::std::convert::AsRef;
use generic_ref::GenericRef;

type Ref1 = GenericRef<&'static str>;
type Ref2 = GenericRef<Ref1>;

fn test_nested_ref2<I: AsRef<&'static str>>(_: I) {}
fn test_nested_ref1<I: AsRef<&'static str>, K: AsRef<I>>(_: K) {}
fn test_ref<K: AsRef<Ref1>>(_: K) {}

fn main() {
    let x = Ref2::new(Ref1::new("12"));

    // test_nested_ref2(&x);
    test_nested_ref1(&x);
    test_ref(&x);

    // 来自标准库对 &GenericRef<T> 的泛型覆盖实现。
    // test_ref(&x);
    // 来自 @Rustacean 对 AsRef<T> 实现块的直接定义。
    // test_ref(x);


    // 1. 作为函数调用的实参，从 &Deref2 自动解引用
    //    至函数的形参 &str
    // test_auto_deref(&x);
    // 2. 作为成员方法调用的 self，从 Deref2 自动解
    //    引用至 &str 和找到目标成员方法
    // assert_eq!("A", x.to_uppercase());
}