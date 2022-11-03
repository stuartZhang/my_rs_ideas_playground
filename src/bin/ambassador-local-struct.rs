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
    impl Pet {
        #[cfg(feature = "ambassador-where")]
        pub fn name(&self) -> &str {
            &self.name
        }
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
/// 【自己·委托·自己】对【委托`trait`】提供`trait methods`与`inherent methods`的双份实现。
/// 【使用场景】需要满足如下几个条件：
///     1. `lib target`工程
///     2. 版本升级时，新版本·重构了·导出`pub`结构体`struct`。
///     3. 重构目标：使用不同的`trait`对【导出·结构体】的【成员方法】做分类
///         3.1 被用作分类的`trait`既不包含“关联·类型”也不包含“关联·常量”。
///     4. 要求新版本的【导出·结构体】
///         4.1 既适用于【旧版】的具体类型·普通函数·调用方式`func_a(_: Cat)`
///         4.2 也适用于【新版】的`trait bound`泛型函数·调用方式`func_a<T: Shout>(_: T)`
/// 特点：因为`inherent methods`的`method resolution`优先级更高，所以若没有明文地使用
///       `<Type as Trait>::method(&self, ...)`语法或`trait Object`，那么`trait methods`
///       几乎不会被调用到。
mod delegating_structure3 {
    use ::ambassador::Delegate;
    use ::derive_builder::Builder;
    use crate::delegated_structure::Shout;
    #[derive(Builder, Debug)]
    #[derive(Delegate)]
    #[delegate(Shout, target = "self")] // 它会给`Cat`结构体再生成一个`impl Shout for Cat {...}`
                                        // 的`trait methods`实现块。
    pub struct Cat {
        aggressive: bool
    }
    ///【手写】`Inherent Methods`实现块 - 适用于旧版本`lib`调用端的`func_a(_: Cat)`普通函数
    impl Cat {
        pub fn shout(&self, input: &str) -> String {
            format!("[aggressive = {}] {} - meow!", self.aggressive, input)
        }
    }
    //【生成】`trait methods`实现块 - 适用于新版本`lib`调用端的`func_a<T: Shout>(_: T)`泛型函数
}
/// 【泛型·结构体】委托至【泛型·类型·字段】。其中，作为委托目标的【泛型·字段】【具体类型】需要满足两个条件：
/// （1）实现【委托`trait`】 — 在本例中是`Shout trait`
///      - 该限制是默认开启的。另一个属性`#[delegate(automatic_where_clause = "false")]`可被用来关闭它。
/// （2）实现由`#[delegate(where)]`属性键-值对指定的`trait bounds` — 在本例中是`Display trait`
///      - 此限制是默认关闭的。
mod delegating_structure4 {
    use ::ambassador::Delegate;
    use ::derive_builder::Builder;
    #[cfg(feature = "ambassador-where")]
    use ::std::fmt::{Display, Formatter, Result};
    #[cfg(feature = "ambassador-where")]
    use crate::delegated_structure::Pet;
    use crate::delegated_structure::Shout;
    #[derive(Builder, Debug)]
    #[derive(Delegate)]
    /// (1) 【泛型·字段`cat`】至少得实现【委托`trait`】。
    #[cfg_attr(not(feature = "ambassador-where"), delegate(Shout))]
    /// (2) 若`where`属性键-值对被设置，【泛型·字段`cat`】还得满足额外的`trait bounds`。
    #[cfg_attr(feature = "ambassador-where", delegate(Shout, where = "T: Display"))]
    pub struct FieldWrapper<T> /* 等效于
        (1) where T: Shout 或
        (2) where T: Shout + Display */ {
        cat: T
    }
    #[cfg(feature = "ambassador-where")]
    impl Display for Pet {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "[{}]", self.name())
        }
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
    { // 【自己·委托·自己】提供对`trait Trait`的`trait methods`与`inherent methods`双份实现。
        use delegating_structure3::{Cat, CatBuilder};
        let cat = CatBuilder::default().aggressive(true).build()?;
        dbg!(cat.shout("input"));                    // 调用的`inherent method`实现
        dbg!(<Cat as Shout>::shout(&cat, "input2")); // 调用的`trait method`实现
                                                     // 这两者不一样。
    }
    { // 【泛型·结构体】委托至【泛型·类型·字段】
        use delegating_structure4::FieldWrapperBuilder;
        let cat = PetBuilder::default().name("a").build()?;
        #[cfg(not(feature = "ambassador-where"))]
        dbg!(&cat);
        #[cfg(feature = "ambassador-where")]
        dbg!(cat.to_string());
        let wrapper = FieldWrapperBuilder::default().cat(cat).build()?;
        dbg!(wrapper.shout("input"));
    }
    Ok(())
}
