mod factory {
    use ::derive_builder::Builder;
    use ::serde::Serialize;
    #[derive(Builder, Debug, Eq, PartialEq)]
    /// 给`TBuilder`派生`Serialize trait`，让其具有`json`序列化的能力。
    #[builder(derive(Serialize))]
    /// 给`TBuilder`结构体装饰元属性`#[serde(rename_all = "camelCase")]`
    #[builder_struct_attr(serde(rename_all = "camelCase"))]
    pub struct Lorem {
        /// 给`TBuilder`字段装饰元属性`#[serde(rename="my_rename1")]`
        #[builder_field_attr(serde(rename="my_rename1"))]
        my_name1: u32,
        my_name2: u32
    }
    impl Lorem {
        pub fn new(my_name1: u32, my_name2: u32) -> Self {
            Lorem {my_name1, my_name2}
        }
    }
}
use ::std::error::Error;
use factory::{Lorem, LoremBuilder};
fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = LoremBuilder::default();
    let x: Lorem = builder.my_name1(10001).my_name2(2002).build()?;
    assert_eq!(x, Lorem::new(10001, 2002));
    // `TBuilder`实例自身就具有被序列化为`json`字符串的能力。
    assert_eq!(serde_json::to_string(&builder)?, r#"{"my_rename1":10001,"myName2":2002}"#);
    Ok(())
}
