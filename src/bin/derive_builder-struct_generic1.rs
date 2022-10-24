mod factory {
    use ::derive_builder::Builder;
    /// 泛型结构体`struct T`的泛型参数名不能是`VALUE`，因为
    /// 该名字已经被【泛型`setter`成员方法】给“抢注”了。
    #[derive(Builder, Debug, Default, Eq, PartialEq)]
    pub struct Lorem<G: Clone> {
        #[builder(default = "self.get_default()")]
        normal: String,
        generic: G
    }
    impl<G> Lorem<G> where G: Clone {
        pub fn new(normal: String, generic: G) -> Self {
            Lorem {normal, generic}
        }
    }
    impl<G> LoremBuilder<G> where G: Clone {
        fn get_default(&self) -> String {
            "i + self.normal.unwrap() + 1".into()
        }
    }
}
use ::std::error::Error;
use factory::{Lorem, LoremBuilder};
fn main() -> Result<(), Box<dyn Error>> {
    let x = LoremBuilder::default().generic(12).build()?;
    assert_eq!(x, Lorem::new("123".to_string(), 12));
    Ok(())
}
