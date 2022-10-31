/// `trait`先定义，`strut / enum`再定义的次序很重要。
/// - 否则，`ambassador crate`给`trait`生成的【过程宏】对`struct / enum`定义不可见。
/// - 相反，对于【远程`trait`】，此定义的先后次序就无所谓了。
#[macro_use]
mod delegated_structure {
    use ::ambassador::delegatable_trait;
    #[delegatable_trait]
    pub trait Shout {
        fn shout(&self, input: &str) -> String;
    }
    #[derive(Clone, Debug)]
    pub struct Cat;
    impl Shout for Cat {
        fn shout(&self, input: &str) -> String {
            format!("{} - meow!", input)
        }
    }
}
/// 【单字段·结构体】委托
mod delegating_structure1 {
    use ::ambassador::{Delegate};
    use ::derive_builder::Builder;
    use crate::delegated_structure::{Cat, Shout};
    /// 标记【本地】`tuple struct`为【委托类】
    /// 在给【单·字段】结构体做委托时，不需要明文指定
    /// `#[delegate(...)]`元属性的`target`键-值对。
    #[derive(Delegate)]
    #[delegate(Shout)]
    pub struct TupleWrapper(pub Cat);
    /// 标记【本地】`struct`为【委托类】
    /// 在给【单·字段】结构体做委托时，不需要明文指定
    /// `#[delegate(...)]`元属性的`target`键-值对。
    #[derive(Builder, Debug)]
    #[derive(Delegate)]
    #[delegate(Shout)]
    pub struct FieldWrapper {
        cat: Cat
    }
}
use ::std::error::Error;
use delegated_structure::{Cat, Shout};
fn main() -> Result<(), Box<dyn Error>> {
    { // 【单字段·结构体】委托
        use delegating_structure1::{FieldWrapperBuilder, TupleWrapper};

        let wrapper = TupleWrapper(Cat);
        dbg!(wrapper.shout("input"));
        let wrapper = FieldWrapperBuilder::default().cat(Cat).build()?;
        dbg!(wrapper.shout("input"));
    }
    Ok(())
}
