mod factory {
    use ::derive_builder::Builder;
    /// `strip_option`不能与`TryInto<F>`泛型`setter`成员方法（即，`try_setter`装饰）
    /// 配套使用。
    #[derive(Builder, Debug, Eq, PartialEq)]
    pub struct Lorem {
        /// `Into<String>`的泛型`setter`成员方法。
        /// 若没有显示地设置，就会收到`UninitializedField`运行时错误。
        #[builder(setter(into, strip_option))]
        strip_into1: Option<u32>,
        /// 普通`setter`成员方法。
        /// 若没有显示地设置，就会收到`UninitializedField`运行时错误。
        #[builder(setter(strip_option))]
        strip1: Option<String>,
        /// 普通`setter`成员方法。
        /// 若没有显示地设置，就会给该字段填入【默认值·求值·表达式】的演算结果。
        #[builder(setter(strip_option), default = r#"Some("123".to_string())"#)]
        strip_default1: Option<String>,
        /// 普通`setter`成员方法。
        /// 若没有显示地设置，就会给该字段填入`Option`【类型·默认值】（即，`None`）。
        #[builder(setter(strip_option), default)]
        strip_default2: Option<String>,
    }
    impl Lorem {
        pub fn new(into_strip1: u32, strip1: String, strip_default1: String, strip_default2: Option<String>) -> Self {
            Lorem {
                strip_into1: Some(into_strip1),
                strip1: Some(strip1),
                strip_default1: Some(strip_default1),
                strip_default2: strip_default2
            }
        }
    }
}
use ::std::error::Error;
use factory::{Lorem, LoremBuilder};
fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = LoremBuilder::default();
    let x: Lorem = builder
        .strip_into1(12u8)
        .strip1("123".to_string())
        .build()?;
    assert_eq!(x, Lorem::new(12, "123".into(), "123".into(), None));
    Ok(())
}
