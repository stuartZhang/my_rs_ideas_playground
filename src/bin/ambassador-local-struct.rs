/// `trait`先定义，`strut / enum`再定义的次序很重要。
/// - 否则，`ambassador crate`给`trait`生成的【过程宏】对`struct / enum`定义不可见。
/// - 相反，对于【远程`trait`】，此定义的先后次序就无所谓了。
#[macro_use]
mod delegated_structure {
    use ::derive_builder::Builder;
    use ::ambassador::delegatable_trait;
    #[delegatable_trait]
    pub trait Shout {
        fn shout(&self, input: &str) -> String;
    }
    #[derive(Builder, Clone, Debug)]
    pub struct Pet {
        #[builder(setter(into))]
        name: String
    }
    impl Shout for Pet {
        fn shout(&self, input: &str) -> String {
            format!("[{}] {} - meow!", self.name, input)
        }
    }
}
/// 【单字段·结构体】委托
mod delegating_structure1 {
    use ::ambassador::Delegate;
    use ::derive_builder::Builder;
    use crate::delegated_structure::{Pet, Shout};
    /// 标记【本地】`tuple struct`为【委托类】
    /// 在给【单·字段】结构体做委托时，不需要明文指定
    /// `#[delegate(...)]`元属性的`target`键-值对。
    #[derive(Delegate)]
    #[delegate(Shout)]
    pub struct TupleWrapper(pub Pet);
    /// 标记【本地】`struct`为【委托类】
    /// 在给【单·字段】结构体做委托时，不需要明文指定
    /// `#[delegate(...)]`元属性的`target`键-值对。
    #[derive(Builder, Debug)]
    #[derive(Delegate)]
    #[delegate(Shout)]
    pub struct FieldWrapper {
        cat: Pet
    }
}
/// 【多字段·结构体】委托至指定字段
mod delegating_structure2 {
    use ::ambassador::Delegate;
    use ::derive_builder::Builder;
    use crate::delegated_structure::{Pet, Shout};
    /// 标记【本地】`tuple struct`为【委托类】
    /// 注意：`#[delegate(...)]`元属性的`target`键-值对可以是序号
    #[derive(Delegate)]
    #[delegate(Shout, target = "1")]
    pub struct TupleWrapper(pub Pet, pub Pet);
    /// 标记【本地】`struct`为【委托类】
    /// 注意：`#[delegate(...)]`元属性的`target`键-值对也可以是字段名
    #[derive(Builder, Debug)]
    #[derive(Delegate)]
    #[delegate(Shout, target = "cat")]
    pub struct FieldWrapper {
        cat: Pet,
        #[allow(dead_code)]
        dog: Pet
    }
}
use ::std::error::Error;
use delegated_structure::{PetBuilder, Shout};
fn main() -> Result<(), Box<dyn Error>> {
    { // 【单字段·结构体】委托
        use delegating_structure1::{FieldWrapperBuilder, TupleWrapper};
        let wrapper = TupleWrapper(PetBuilder::default().name("a").build()?);
        dbg!(wrapper.shout("input"));
        let wrapper = FieldWrapperBuilder::default().cat(PetBuilder::default().name("a").build()?).build()?;
        dbg!(wrapper.shout("input"));
    }
    { // 【多字段·结构体】委托至指定字段
        use delegating_structure2::{FieldWrapperBuilder, TupleWrapper};
        let wrapper = TupleWrapper(
            PetBuilder::default().name("a").build()?,
            PetBuilder::default().name("b").build()?
        );
        dbg!(wrapper.shout("input"));
        let wrapper = FieldWrapperBuilder::default()
            .cat(PetBuilder::default().name("a").build()?)
            .dog(PetBuilder::default().name("b").build()?)
            .build()?;
        dbg!(wrapper.shout("input"));
    }
    Ok(())
}
