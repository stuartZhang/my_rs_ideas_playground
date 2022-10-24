mod factory {
    use ::derive_builder::Builder;
    use ::std::{convert::From, num::ParseIntError};
    #[derive(Builder, Debug, Eq, PartialEq)]
    pub struct Lorem {
        #[builder(default = "self.get_ipsum_default()?")]
        ipsum: u32
    }
    impl Lorem {
        pub fn new(ipsum: u32) -> Self {
            Lorem {ipsum}
        }
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
}
use ::std::error::Error;
use factory::{Lorem, LoremBuilder};
fn main() -> Result<(), Box<dyn Error>> {
    let x: Lorem = LoremBuilder::default()
        .build()?;
    assert_eq!(x, Lorem::new(42));
    Ok(())
}
