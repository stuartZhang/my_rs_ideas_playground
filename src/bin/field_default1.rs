mod factory {
    use ::derive_builder::Builder;
    use ::std::{convert::From, num::ParseIntError};
    #[derive(Builder, Debug, Default, Eq, PartialEq)]
    #[builder(default)]
    pub struct Lorem {
        normal: u32,
        default_type1: u32,
        /// 求值·表达式 - 在`TBuilder::build()`成员方法被调用时·被（运行时）求值。
        #[builder(default = r#""123".to_string()"#)]
        default_expr1: String,
        /// 调用`TBuilder`（私有）成员方法 - 在`TBuilder::build()`成员方法被调用时·被（运行时）求值。
        #[builder(default = "self.get_default(12u32)")]
        default_method_invoke1: u32,
        /// 调用`TBuilder`（私有）成员方法 - 在`TBuilder::build()`成员方法被调用时·被（运行时）求值。
        /// 以`?`操作符从被调用的`TBuilder`成员方法向`TBuilder::build()`上抛错误。
        /// 但是，从【被抛出错误类型】到`TBuilderError`的类型转换必须由开发者自己徒手提供
        #[builder(default = "self.get_default_throw_err1()?")]
        default_throw_err1: u32
    }
    impl Lorem {
        pub fn new(normal: u32, default_type1: u32, default_expr1: String, default_method_invoke1: u32, default_throw_err1: u32) -> Self {
            Lorem {normal, default_type1, default_expr1, default_method_invoke1, default_throw_err1}
        }
    }
    impl LoremBuilder {
        fn get_default(&self, i: u32) -> u32 {
            // 可以引用`TBuilder`上任意字段的值。
            // 注意：字段类型是`Option<F>`而不是`F`。
            i + self.normal.unwrap() + 1
        }
        /// 允许从【默认值】的【求值表达式】向`TBuilder::build()`成员方法·上抛错误
        fn get_default_throw_err1(&self) -> Result<u32, ParseIntError> {
            "42".parse()
        }
    }
    /// 但是，从【被抛出错误类型】到`TBuilderError`的类型转换必须由开发者自己徒手提供
    impl From<ParseIntError> for LoremBuilderError {
        fn from(error: ParseIntError) -> Self {
            LoremBuilderError::ValidationError(error.to_string())
        }
    }
}
use ::std::error::Error;
use factory::{Lorem, LoremBuilder};
fn main() -> Result<(), Box<dyn Error>> {
    let x: Lorem = LoremBuilder::default().normal(1).build()?;
    assert_eq!(x, Lorem::new(1, 0, "123".to_string(), 14, 42));
    Ok(())
}
