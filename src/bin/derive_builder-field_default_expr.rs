use ::derive_builder::Builder;
use ::std::{error::Error, convert::From, num::ParseIntError};
#[derive(Builder)]
struct Lorem {
    #[builder(default = "self.get_ipsum_default()?")]
    ipsum: u32
}
impl LoremBuilder {
    // 允许从【默认值】的【求值表达式】向`TBuilder::build()`成员方法上抛错误
    fn get_ipsum_default(&self) -> Result<u32, ParseIntError> {
        "42".parse()
    }
}
// 但是，从【被抛出错误类型】到`TBuilderError`的类型转换必须由开发者自己徒手提供
impl From<ParseIntError> for LoremBuilderError {
    fn from(error: ParseIntError) -> Self {
        LoremBuilderError::ValidationError(error.to_string())
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let x: Lorem = LoremBuilder::default()
        .build()?;
    assert_eq!(x.ipsum, 42);
    Ok(())
}
