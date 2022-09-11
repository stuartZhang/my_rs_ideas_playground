mod factory {
    use ::derive_builder::Builder;
    #[derive(Builder, Debug, Eq, PartialEq)]
    /// 重命名`TBuilder`结构体
    #[builder(name = "LoremConstructor")]
    /// 前缀每个`setter`成员方法的函数名。此属性也能仅只装饰一个`struct T`字段。
    #[builder(setter(prefix = "global"))]
    pub struct Lorem {
        /// 重命名`setter`成员方法。其优先级要高于
        /// 【前缀·字段名】的装饰。
        #[builder(setter(name = "rename1"))]
        name1: u32,
        /// 被（结构体）全局·`setter`成员方法函数名·前缀化了。
        prefix1: String,
        /// 将`setter`成员方法的可见性设置为【私有】。于是，从`mod factory`模块
        /// 之外，此`setter`就不能被调用了。所以，需要以【类型·默认值】作为该字段
        /// 的默认值。
        #[builder(private, default)]
        private: u32,
    }
    impl Lorem {
        pub fn new(name1: u32, prefix1: String, private: u32) -> Self {
            Lorem {name1, prefix1, private}
        }
    }
}
use ::std::error::Error;
use factory::{Lorem, LoremConstructor};
fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = LoremConstructor::default();
    let x: Lorem = builder.rename1(12).global_prefix1("123".into()).build()?;
    assert_eq!(x, Lorem::new(12, "123".into(), 0));
    Ok(())
}
