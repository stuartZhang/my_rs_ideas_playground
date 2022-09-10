mod factory {
    use ::derive_builder::Builder;
    #[derive(Builder, Debug, Eq, PartialEq)]
    pub struct Lorem {
        /// 被跳过字段的数据类型
        /// 要么，实现了`Default trait`来提供（类型）默认值。
        #[builder(setter(skip))]
        skip1: u32,
        /// 要么，装饰【默认值·求值·表达式】，以在`build()`过程中，计算默认值。
        #[builder(setter(skip), default = "self.get_skip2_default()")]
        skip2: String,
        /// 就效果而言，【设置·字段·默认值】与【跳过·字段】是一样的。但是，
        /// （1）前者是有`setter`而选择性地不调用。即，有能力而不用。
        /// （2）后者是完全没有定制的机会了，因为从`TBuilder`字段到`setter`都没有被生成。即，没有能力。
        /// 【类型·默认值】将在`TBuilder::build()`执行过程中，被用来填充字段值。
        #[builder(default)]
        skip3: u32,
    }
    impl Lorem {
        pub fn new(skip1: u32, skip2: String, skip3: u32) -> Self {
            Lorem {skip1, skip2, skip3}
        }
    }
    impl LoremBuilder {
        fn get_skip2_default(&self) -> String {
            "123".to_string()
        }
    }
}
use ::std::error::Error;
use factory::{Lorem, LoremBuilder};
fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = LoremBuilder::default();
    let x: Lorem = builder.build()?;
    assert_eq!(x, Lorem::new(0, "123".into(), 0));
    // 针对`skip3`，还是保留了对其做定制的机会的。
    let x = builder.skip3(2).build()?;
    assert_eq!(x, Lorem::new(0, "123".into(), 2));
    Ok(())
}
