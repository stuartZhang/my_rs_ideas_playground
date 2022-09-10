mod factory {
    use ::derive_builder::Builder;
    #[derive(Builder, Debug, Eq, PartialEq)]
    pub struct Lorem {
        /// `struct TBuilder`的`Option<u32>`字段还是会被生成的。
        /// 但是，该字段对应的的`setter`成员方法却需要开发者自己
        /// 定义了。
        #[builder(setter(custom))]
        custom1: u32
    }
    impl Lorem {
        pub fn new(custom1: u32) -> Self {
            Lorem {custom1}
        }
    }
    impl LoremBuilder {
        /// 定义自己的`setter`成员方法。注意：`TBuilder`字段的类型
        /// 是`Option<F>`而不仅仅是`F`。
        pub fn custom1(&mut self, value: u32) -> &mut Self {
            self.custom1 = Some(value + 1);
            self
        }
    }
}
use ::std::error::Error;
use factory::{Lorem, LoremBuilder};
fn main() -> Result<(), Box<dyn Error>> {
    let x: Lorem = LoremBuilder::default().custom1(12).build()?;
    assert_eq!(x, Lorem::new(13));
    Ok(())
}
