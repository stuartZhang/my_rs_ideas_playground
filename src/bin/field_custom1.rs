mod factory {
    use ::std::{convert::From, num::ParseIntError};
    use ::derive_builder::Builder;
    #[derive(Builder, Debug, Eq, PartialEq)]
    #[builder(derive(Debug, Eq, PartialEq))]
    pub struct Lorem<'a> {
        /// （1）定制`TBuilder`字段类型不再是`Option<F>`而是`F`。
        /// （2）这要求`impl Default for F`，以便`TBuilder`结构体
        ///     可被零实参地被实例化。
        #[builder(field(type = "&'a str"))]
        custom1: &'a str,
        /// （1）装饰【求值·表达式】允许`T`字段与其对应的`TBuilder`字段归属
        ///     不同的数据类型。
        /// （2）允许【类型转换】操作·上抛错误给`TBuilder::build()`成员方法，
        ///     但从【被上抛·错误类型】至`TBuilderError`的【类型转换】得由
        ///     开发者显示地编写。
        #[builder(setter(into), field(type = "String", build = "self.custom2.parse()?"))]
        custom2: u32,
        /// 无论给`custom3`字段的`setter`成员方法传递什么值，该字段的值
        /// 都将永远是`None`，因为【求值·表达式】硬编码了`None`。
        #[builder(setter(into), field(type = "String", build = "None"))]
        custom3: Option<u32>,
    }
    impl<'a> Lorem<'a> {
        pub fn new(custom1: &'a str, custom2: u32, custom3: Option<u32>) -> Self {
            Lorem {custom1, custom2, custom3}
        }
    }
    impl<'a> LoremBuilder<'a> {
        pub fn new(custom1: &'a str, custom2: String, custom3: String) -> Self {
            LoremBuilder {custom1, custom2, custom3}
        }
    }
    /// 允许【类型转换】操作·上抛错误给`TBuilder::build()`成员方法，
    /// 但从【被上抛错误类型】至`TBuilderError`的【类型转换】处理得
    /// 由开发者显示地提供。
    impl From<ParseIntError> for LoremBuilderError {
        fn from(error: ParseIntError) -> Self {
            LoremBuilderError::ValidationError(error.to_string())
        }
    }
}
use ::std::error::Error;
use factory::{Lorem, LoremBuilder};
fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = LoremBuilder::default();
    // （1）未显示设置的`custom1`字段值保存了【类型·默认值】空串，
    //     因为字段类型被强制要求实现`Default trait`。
    // （2）无论给`custom3`字段的`setter`传递什么值，被构造的结构
    //     体实例的`custom3`字段值都仅只会是`None`。
    let x: Lorem = builder.custom2("123").custom3("123").build()?;
    assert_eq!(x, Lorem::new("", 123, None));
    assert_eq!(builder, LoremBuilder::new("", "123".into(), "123".into()));
    // 给`custom1`字段填充若它的值。
    let x = builder.custom1("123").build()?;
    assert_eq!(x, Lorem::new("123", 123, None));
    assert_eq!(builder, LoremBuilder::new("123", "123".into(), "123".into()));
    Ok(())
}
