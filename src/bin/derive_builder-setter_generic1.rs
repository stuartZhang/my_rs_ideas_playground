mod factory {
    use ::derive_builder::Builder;
    #[derive(Builder, Debug, Eq, PartialEq)]
    pub struct Lorem {
        /// `TryInto<u32>`的泛型`setter`成员方法。虽然重命名了
        /// 该成员方法，但最终的`setter`函数名还是遵循`try_<setter函数名>`
        /// 的格式。即，`try_fallible_into1`。
        #[builder(try_setter, setter(name = "fallible_into1"))]
        try_into1: u32,
        /// `Into<String>`的泛型`setter`成员方法。`setter`函数名
        /// 依旧对齐于字段名。
        #[builder(setter(into))]
        into1: String,
    }
    impl Lorem {
        pub fn new(try_into1: u32, into1: String) -> Self {
            Lorem {try_into1, into1}
        }
    }
}
use ::std::error::Error;
use factory::{Lorem, LoremBuilder};
fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = LoremBuilder::default();
    let x: Lorem = builder
        .try_fallible_into1(12u64).unwrap() // `setter`返回值是`Result<&mut TBuilder, TryInto::Error>`类型
        .into1("123")
        .build()?;
    assert_eq!(x, Lorem::new(12, "123".into()));
    Ok(())
}
