mod delegating_structure1 {
    /// 给`Newtypes`风格的【单·字段】结构体做委托时，不需要明文
    /// 指定`#[delegate(...)]`元属性的`target`键值对。
    use ::ambassador::{Delegate, delegatable_trait};
    use ::derive_builder::Builder;
    use crate::delegated_structure::Cat;
    // 标记【本地】`trait`为【可委托】
    #[delegatable_trait]
    pub trait Shout {
        fn shout(&self, input: &str) -> String;
    }
    // 标记【本地】`tuple struct`为【委托类】
    #[derive(Delegate)]
    #[delegate(Shout)]
    pub struct TupleWrapper(pub Cat);
    // 标记【本地】`struct`为【委托类】
    #[derive(Builder, Debug)]
    #[derive(Delegate)]
    #[delegate(Shout)]
    pub struct FieldWrapper {
        cat: Cat
    }
}
mod delegated_structure {
    use crate::delegating_structure1::Shout;
    #[derive(Clone, Debug)]
    pub struct Cat;
    impl Shout for Cat {
        fn shout(&self, input: &str) -> String {
            format!("{} - meow!", input)
        }
    }
}
use ::std::error::Error;
use delegating_structure1::{FieldWrapperBuilder, Shout, TupleWrapper};
use delegated_structure::Cat;
fn main() -> Result<(), Box<dyn Error>> {
    let wrapper = TupleWrapper(Cat);
    dbg!(wrapper.shout("input"));
    let wrapper = FieldWrapperBuilder::default().cat(Cat).build()?;
    dbg!(wrapper.shout("input"));
    Ok(())
}
