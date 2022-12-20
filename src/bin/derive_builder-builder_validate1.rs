mod factory {
    use ::derive_builder::Builder;
    #[derive(Builder, Debug, Eq, PartialEq)]
    /// 注意`validate`不是【求值·表达式】而是【项·路径】。
    #[builder(build_fn(validate = "Self::validate"))]
    pub struct Lorem {
        #[builder(default = "12")]
        default1: u32,
        validate1: u32,
    }
    impl Lorem {
        pub fn new(default1: u32, validate1: u32) -> Self {
            Lorem {default1, validate1}
        }
    }
    impl LoremBuilder {
        ///（1）验证函数的签名是固定的`(&TBuilder) -> Result<(), String>`
        ///（2）在`TBuilder::build()`执行期间，【验证函数】被（运行时）调用执行。
        ///（3）因为被生成的`TBuilderError`已经包含了对`From<String> trait`的实现，
        ///    所以从【验证函数】向`TBuilder::build()`成员方法·上抛的错误能够被无缝
        ///    地兼容。
        fn validate(&self) -> Result<(), String> {
            // 【字段·默认值】是不过【验证·函数】的。所以，`default1 = 12`不会
            // 引起结构体验证失败。
            // 【默认值】被应用于【验证·通过】之后。
            if let Some(default1) = self.default1 {
                if default1 <= 15 {
                    return Err("无效默认值".into());
                }
            }
            if let Some(validate1) = self.validate1 {
                if validate1 <= 20 {
                    return Err("无效验证值".into());
                }
            }
            Ok(())
        }
    }
}
use ::std::error::Error;
use factory::{Lorem, LoremBuilder};
fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = LoremBuilder::default();
    let x: Lorem = builder.validate1(100).build()?;
    assert_eq!(x, Lorem::new(12, 100));
    // 只会验证失败于被显示地设置的字段值上。
    let x = builder.validate1(11).build().unwrap_err();
    assert_eq!(x.to_string(), "无效验证值");
    Ok(())
}
