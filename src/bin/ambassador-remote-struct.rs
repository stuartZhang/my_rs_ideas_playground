/// 模仿在`crate`外定义的【外部】`struct / enum`。
mod remote_structure {
    use ::derive_builder::Builder;
    use ::std::ops::{Deref, DerefMut};
    pub trait Shout {
        fn shout(&self, input: &str) -> String;
        fn alias(&mut self, name: &str);
    }
    #[derive(Builder, Clone, Debug)]
    pub struct Pet {
        #[builder(setter(into))]
        name: String
    }
    impl Pet {
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn change_name(&mut self, name: String) {
            self.name = name;
        }
    }
    /// 【单字段·结构体】委托
    pub struct SingleElementWrapper(pub Pet);
    #[derive(Builder, Debug)]
    pub struct SingleFieldWrapper {
        pub cat: Pet
    }
    /// 【多字段·结构体】委托至指定字段
    pub struct MultiElementWrapper(pub Pet, pub Pet);
    #[derive(Builder, Debug)]
    pub struct MultiFieldWrapper {
        pub cat: Pet,
        pub dog: Pet
    }
    /// 【自己·委托·自己】对【委托`trait`】提供`trait methods`与`inherent methods`的双份实现。
    #[derive(Builder, Debug)]
    pub struct SelfWrapper {
        aggressive: bool,
        #[builder(default)]
        name: Option<String>
    }
    impl SelfWrapper {
        pub fn aggressive(&self) -> bool {
            self.aggressive
        }
        pub fn change_name(&mut self, name: String) {
            self.name = Some(name);
        }
    }
    /// 【泛型·结构体】委托至【泛型·类型·字段】
    #[derive(Builder, Debug)]
    pub struct GenericFieldWrapper<T> /* #3. 自动添加
        (1) where T: Shout 或
        (2) where T: Shout + Display */ {
        pub cat: T
    }
    /// 委托【泛型`trait`】
    #[derive(Builder, Debug)]
    pub struct GenericTraitWrapper {
        pub cat: Pet
    }
    /// 委托至【智能·指针】（或称“间接”委托）
    #[derive(Builder)]
    #[builder(pattern = "owned", setter(into))]
    pub struct SmartPointerWrapper {
        pub pet: Box<dyn Shout>
    }
    /// 委托至【成员方法·返回值】
    #[derive(Builder, Debug)]
    pub struct TargetMethodWrapper {
        #[builder(setter(into))]
        pub pet: Box<Pet>
    }
    impl Deref for TargetMethodWrapper {
        type Target = Pet;
        fn deref(&self) -> &Self::Target {
            self.pet.deref()
        }
    }
    impl DerefMut for TargetMethodWrapper {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.pet.deref_mut()
        }
    }
}
///（1）`trait`先定义，`strut / enum`再定义的次序很重要。否则，`ambassador crate`
///    给`trait`生成的【过程宏】对`struct / enum`定义不可见。
///（2）复制【外部】`struct / enum`定义至本地代码，并冠以`#[delegate_remote]`元属性。
///（3）【外部】`struct / enum`仅需要保证【委托·目标·字段】是`pub`的，即可。而不需要
///     所有字段都是`pub`。
#[macro_use]
mod delegated_structure {
    use ::ambassador::{delegatable_trait, delegatable_trait_remote};
    use ::std::fmt::Display;
    use crate::remote_structure::{Pet, Shout};
    #[delegatable_trait_remote]
    pub trait Shout {
        fn shout(&self, input: &str) -> String;
        fn alias(&mut self, name: &str);
    }
    /// 它的作用可渗透至下游`crate`
    #[delegatable_trait]
    pub trait ShoutGeneric<'a, 'b, T, R> where 'a: 'b, T: Display, R: Display {
        fn shout(&self, input1: &'a str, input2: &'b T) -> R;
    }
    /// 【委托·目标（字段）类型】得实现【委托`trait`】。
    impl Shout for Pet {
        fn shout(&self, input: &str) -> String {
            format!("[{}] {} - meow!", self.name(), input)
        }
        fn alias(&mut self, name: &str) {
            self.change_name(name.into());
        }
    }
}
/// 【单字段·结构体】委托
mod single_field_demo {
    mod delegating_structure {
        use ::ambassador::delegate_remote;
        use crate::remote_structure::{Pet, Shout, SingleElementWrapper, SingleFieldWrapper};
        /// 标记【外部】`tuple struct`为【委托类】
        /// 在给【单·字段】结构体做委托时，不需要明文指定
        /// `#[delegate(...)]`元属性的`target`键-值对。
        #[delegate_remote]
        #[delegate(Shout)]
        struct SingleElementWrapper(Pet);
        /// 标记【外部】`struct`为【委托类】
        /// 在给【单·字段】结构体做委托时，不需要明文指定
        /// `#[delegate(...)]`元属性的`target`键-值对。
        #[delegate_remote]
        #[delegate(Shout)]
        struct SingleFieldWrapper {
            cat: Pet
        }
        // 给【委托·类型】自动生成`trait`实现块
    }
    use ::std::error::Error;
    use crate::remote_structure::{PetBuilder, Shout, SingleElementWrapper, SingleFieldWrapperBuilder};
    pub fn main() -> Result<(), Box<dyn Error>> {
        let wrapper = SingleElementWrapper(PetBuilder::default().name("a").build()?);
        dbg!(wrapper.shout("input"));
        let wrapper = SingleFieldWrapperBuilder::default().cat(PetBuilder::default().name("a").build()?).build()?;
        dbg!(wrapper.shout("input"));
        Ok(())
    }
}
/// 【多字段·结构体】委托至指定字段
mod multiple_field_demo {
    mod delegating_structure {
        use ::ambassador::delegate_remote;
        use crate::remote_structure::{Pet, MultiElementWrapper, MultiFieldWrapper, Shout};
        /// 标记【外部】`tuple struct`为【委托类】
        /// 注意：`#[delegate(...)]`元属性的`target`键-值对可以是序号
        #[delegate_remote]
        #[delegate(Shout, target = "1")]
        struct MultiElementWrapper(Pet, Pet);
        /// 标记【外部】`struct`为【委托类】
        /// 注意：`#[delegate(...)]`元属性的`target`键-值对也可以是字段名
        #[delegate_remote]
        #[delegate(Shout, target = "cat")]
        struct MultiFieldWrapper {
            cat: Pet,
            dog: Pet
        }
        // 给【委托·类型】自动生成`trait`实现块
    }
    use ::std::error::Error;
    use crate::remote_structure::{MultiElementWrapper, MultiFieldWrapperBuilder, PetBuilder, Shout};
    pub fn main() -> Result<(), Box<dyn Error>> {
        let wrapper = MultiElementWrapper(
            PetBuilder::default().name("a").build()?,
            PetBuilder::default().name("b").build()?
        );
        dbg!(wrapper.shout("input"));
        let wrapper = MultiFieldWrapperBuilder::default()
            .cat(PetBuilder::default().name("a").build()?)
            .dog(PetBuilder::default().name("b").build()?)
            .build()?;
        dbg!(wrapper.shout("input"));
        Ok(())
    }
}
/// 【自己·委托·自己】对【委托`trait`】提供`trait methods`与`inherent methods`的双份实现。
/// 【使用场景】需要满足如下几个条件：
///     1. `lib target`工程
///     2. 版本升级时，新版本·重构了·导出结构体`pub struct`的成员方法布局。
///     3. 重构目标：使用不同的`trait`对【导出·结构体】的【成员方法】做分组
///         3.1 被用作分组的`trait`既不能包含“关联·类型”也能不包含“关联·常量”。
///         3.2 若被用作分组`trait`的成员方法并没有被【委托·目标·类型`self`】逐一被实现（毕
///             竟，并没有从语法上`impl trait`），那么`[unconditional_recursion]`编译错误
///             就会出现。
///     4. 要求新版本的【导出·结构体】
///         4.1 既适用于【旧版】的具体类型·普通函数·调用方式`func_a(_: Cat)`
///         4.2 也适用于【新版】的`trait bound`·泛型函数·调用方式`func_a<T: Shout>(_: T)`
/// 特点：因为`inherent methods`的`method resolution`优先级更高，所以若没有使用完全限定语法
///       `<Type as Trait>::method(&self, ...)`语法或`trait Object`，那么`trait methods`
///       几乎不会被调用到。
mod to_self_demo {
    mod delegating_structure {
        use ::ambassador::delegate_remote;
        use crate::remote_structure::{SelfWrapper, Shout};
        #[delegate_remote]
        #[delegate(Shout, target = "self")] // 它会给`Cat`结构体再生成一个`impl Shout for Cat {...}`
                                            // 的`trait methods`实现块。
        struct SelfWrapper {}
        ///【手写】`Inherent Methods`实现块 - 适用于旧版本`lib`调用端的`func_a(_: Cat)`普通函数
        impl SelfWrapper {
            pub fn shout(&self, input: &str) -> String {
                format!("[aggressive = {}] {} - meow!", self.aggressive(), input)
            }
            fn alias(&mut self, name: &str) {
                self.change_name(name.into());
            }
        }
        //【生成】`trait methods`实现块 - 适用于新版本`lib`调用端的`func_a<T: Shout>(_: T)`泛型函数
    }
    use ::std::error::Error;
    use crate::remote_structure::{SelfWrapper, SelfWrapperBuilder, Shout};
    pub fn main() -> Result<(), Box<dyn Error>> {
        let cat = SelfWrapperBuilder::default().aggressive(true).build()?;
        dbg!(cat.shout("input"));                    // 调用的`inherent method`实现
        dbg!(<SelfWrapper as Shout>::shout(&cat, "input2")); // 调用的`trait method`实现
                                                             // 这两者不一样。
        Ok(())
    }
}
/// 【泛型·结构体】委托至【泛型·类型·字段】`where`。其中，委托·目标【泛型·字段】需要满足两个条件：
/// （1）实现【委托`trait`】 — 在本例中是`Shout trait`
///      - 另一个属性`#[delegate(automatic_where_clause = "false")]`可被用来关闭该限制。
/// （2）实现由`#[delegate(where)]`属性键-值对额外指定的`trait bounds` — 在本例中是`Display trait`
/// 最后，由`Ambassador crate`派生的过程宏·会给【委托·类型】自动添加`where`从句，来落实上述两条约束。
mod generic_type_demo {
    mod delegating_structure {
        use ::ambassador::delegate_remote;
        #[cfg(feature = "ambassador-where")]
        use ::std::fmt::{Display, Formatter, Result};
        #[cfg(feature = "ambassador-where")]
        use crate::remote_structure::Pet;
        use crate::remote_structure::{GenericFieldWrapper, Shout};
        #[delegate_remote]
        #[cfg_attr(not(feature = "ambassador-where"), delegate(Shout))]
        #[cfg_attr(feature = "ambassador-where", delegate(Shout, where = "T: Display"))]
        pub struct GenericFieldWrapper<T> /* #3. 自动添加
            (1) where T: Shout 或
            (2) where T: Shout + Display */ {
            cat: T
        }
        /// #1. 【委托·目标（字段）类型】至少得实现【委托`trait`】。
        /// #2. 【委托·目标（字段）类型】还得实现由`#[delegate(where)]`属性键-值对额外指定的`trait bounds`
        #[cfg(feature = "ambassador-where")]
        impl Display for Pet {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "[{}]", self.name())
            }
        }
        // 给【委托·类型】自动生成`trait`实现块
    }
    use ::std::error::Error;
    use crate::remote_structure::{GenericFieldWrapperBuilder, PetBuilder, Shout};
    pub fn main() -> Result<(), Box<dyn Error>> {
        let cat = PetBuilder::default().name("a").build()?;
        #[cfg(not(feature = "ambassador-where"))]
        dbg!(&cat);
        #[cfg(feature = "ambassador-where")]
        dbg!(cat.to_string());
        let wrapper = GenericFieldWrapperBuilder::default().cat(cat).build()?;
        dbg!(wrapper.shout("input"));
        Ok(())
    }
}
/// 委托【泛型`trait`】`generics`。其中，【`trait`泛型参数】（含【限定条件】）
/// (1) 既要·被注册于`#[delegate(generics)]`属性键-值对
/// (2) 还要·被添加于【委托·目标（字段）类型】的`trait`实现块上。譬如，`impl<T> ShoutGeneric<T> for Pet where T: *** {`。
/// (3) 由`Ambassador crate`派生的过程宏·会自动“同步”【委托·目标（字段）类型】`trait`实现块
///     上的【`trait`泛型参数】（含【限定条件】）至【委托·类型】的`trait`实现块上。
mod generic_trait_demo {
    mod delegating_structure {
        use ::ambassador::delegate_remote;
        use ::std::fmt::Display;
        use crate::{delegated_structure::{ShoutGeneric}, remote_structure::{GenericTraitWrapper, Pet}};
        #[delegate_remote]
        /// #1. 【`trait`泛型参数】（含【限定条件】）被注册于`#[delegate(generics)]`属性键-值对
        #[delegate(ShoutGeneric<'a, 'b, T, R>, generics="'a: 'b, 'b, T: Display, R: Display")]
        struct GenericTraitWrapper {
            cat: Pet
        }
        /// #2. 【`trait`泛型参数】（含【限定条件】）被添加于【委托·目标（字段）类型】的
        ///     `trait`实现块上。
        impl<'a, 'b, T> ShoutGeneric<'a, 'b, T, String> for Pet where 'a: 'b, T: Display {
            fn shout(&self, input1: &'a str, input2: &'b T) -> String {
                format!("[{}] {}, {} - meow!", self.name(), input1, input2)
            }
        }
        // #3. 给【委托·类型】生成【`trait`实现块】和添加【`trait`泛型参数】（含【限定条件】）
    }
    use ::std::{error::Error, net::{IpAddr, Ipv4Addr}};
    use crate::remote_structure::{GenericTraitWrapper, GenericTraitWrapperBuilder, PetBuilder};
    use crate::delegated_structure::ShoutGeneric;
    pub fn main() -> Result<(), Box<dyn Error>> {
        let cat = PetBuilder::default().name("a").build()?;
        let wrapper = GenericTraitWrapperBuilder::default().cat(cat).build()?;
        let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        dbg!(<GenericTraitWrapper as ShoutGeneric<'_, '_, IpAddr, _>>::shout(&wrapper, "input1", &addr));
        Ok(())
    }
}
/// 委托至【智能·指针】（或称“间接”委托）。即，
/// （1）【智能·指针】类型自身并未直接实现【委托`trait`】。
/// （2）但由【智能·指针】引用的内部类型却实现了【委托`trait`】。
/// （3）借助于`.`操作符的【自动解引用】语法糖，【委托`trait`】的【成员方法】被允许从
///     【智能·指针】实例直接“点”出并调用。
/// 默认情况下，`Ambassador crate`要求【委托·目标（字段）类型】与【委托·类型】皆实现
/// 相同的【委托`trait`】。
/// `#[delegate(automatic_where_clause = "false")]`属性可用来用选择退出这个限制。
/// 它可以被理解为`#[delegate_to_remote_methods]`针对`Deref::deref()`与`DerefMut::deref_mut()`
/// 场景的语法糖。
mod to_smart_pointer_demo {
    mod delegating_structure {
        use ::ambassador::delegate_remote;
        use crate::remote_structure::{Pet, Shout, SmartPointerWrapper};
        #[delegate_remote]
        /// 【委托·目标（字段）类型】`Box<T>`自身并没有实现【委托`trait`】，虽然它被解引用后可调
        /// 用【委托`trait`】的成员方法。
        #[delegate(Shout, automatic_where_clause = "false")]
        pub struct SmartPointerWrapper {
            pet: Box<dyn Shout>
        }
        impl From<Pet> for Box<dyn Shout> {
            fn from(pet: Pet) -> Self {
                Box::new(pet)
            }
        }
        // 给【委托·类型】自动生成`trait`实现块
    }
    use ::std::error::Error;
    use crate::remote_structure::{PetBuilder, Shout, SmartPointerWrapperBuilder};
    pub fn main() -> Result<(), Box<dyn Error>> {
        let cat = PetBuilder::default().name("a").build()?;
        let boxed_pet = SmartPointerWrapperBuilder::default().pet(cat).build()?;
        dbg!(boxed_pet.shout("input"));
        Ok(())
    }
}
/// 委托至【成员方法·返回值】。其中，
/// （1）【成员方法】也被称为“委托·目标·成员方法”`target method`。
/// （2）【成员方法】既可以是`inherent method`也可以是`trait method`。
/// 若【委托·目标·成员方法】自身就是`trait method`，那么它就不能够与【委托`trait`】成员
/// 方法同名。或者，给他做一个`inherent method`包装函数；和委托至该包装函数。
/// `#[delegate(...)]`提供了三个属性键-值对`target_ref`, `target_mut`, `target_owned`
/// （1）分别对应于【委托`trait`】内三类“接受者·类型”（`&self`, `&mut self`, `self`）的成员方法
/// （2）分别对应于三款样式的成员方法签名
///     - target_ref   | &self     | fn get_delegate_target(&self)         -> &X
///     - target_mut   | &mut self | fn get_delegate_target_mut(&mut self) -> &mut X
///     - target_owned | self      | fn get_delegate_target_owned(self)    -> X
/// （3）对它们，按需设置就好，不必每次都全部配置。
/// `#[delegate]`与`#[delegate_to_remote_methods]`被修饰于`impl`块，而不是类型定义。
mod to_method_return_value {
    mod delegating_structure {
        use ::ambassador::delegate_to_remote_methods;
        use ::std::ops::{Deref, DerefMut};
        use crate::remote_structure::{Pet, Shout, TargetMethodWrapper};
        ///（1）先单独定义`target method`。
        /// 因为【委托·类型】是外部的，所以不能直接使用`#[delegate_to_methods]`元属性
        /// 来直接修饰`impl`块。
        impl TargetMethodWrapper {
            fn get_delegate_target(&self) -> &Pet {
                self.pet.deref()
            }
        }
        //（2）再委托`trait`实现·给该成员方法的返回值。
        #[delegate_to_remote_methods] // 下面的`impl`块是虚的。`Ambassador crate`过程宏并不会
                                      // 给`get_delegate_target()`与`deref_mut()`方法签名生
                                      // 成实际的`impl`块。
        #[delegate(Shout, target_ref = "get_delegate_target", target_mut = "deref_mut")]
        impl TargetMethodWrapper { // 混合不同源的`Inherent method`与`trait method`
            // 此是【本地】定义的`Inherent method`
            fn get_delegate_target(&self) -> &Pet;
            // 此是【外部】定义的`trait method`
            fn deref_mut(&mut self) -> &mut Pet;
            // 绝不能包含非`target method`成员方法
        }
    }
    use ::std::error::Error;
    use crate::remote_structure::{PetBuilder, Shout, TargetMethodWrapperBuilder};
    pub fn main() -> Result<(), Box<dyn Error>> {
        let cat = PetBuilder::default().name("a").build()?;
        let mut boxed_pet = TargetMethodWrapperBuilder::default().pet(cat).build()?;
        print(&boxed_pet, "input1");
        boxed_pet.alias("b");
        print(&boxed_pet, "input2");
        Ok(())
    }
    fn print(i: &impl Shout, s: &str) {
        dbg!(i.shout(s));
    }
}
/// 借助【覆盖·实现`Blanket Implementation`】，将满足`trait bounds`的一类【类型】都作为【委托·类型】。
/// 它的目标用户就是：其内部类型已经实现了指定`trait`的【智能指针】。
/// 另外，因为符合`trait bounds`的类型定义一般并不在当前`crate`内（即，【外部】委托类型），所以
/// `#[delegate_to_remote_methods]`元属性被用来注册委托。
mod blanket_impl_demo {
    #[allow(drop_bounds)]
    mod delegating_structure {
        use ::ambassador::delegate_to_remote_methods;
        use ::std::ops::DerefMut;
        use crate::remote_structure::Shout;
        #[delegate_to_remote_methods] // 允许【外部】委托·类型
        #[delegate(Shout, target_ref = "deref", target_mut = "deref_mut")] // 委托至【成员方法·返回值】
        // 满足如下三个条件的任何类型
        // 1. 同时实现了`DerefMut`与`Drop`两个`trait`。即，该类型至少是一个【智能指针】类型
        // 2. 其内部类型实现了`Shout trait`
        // 3. 允许其内部类型是`trait Object`，因为`?Sized`限定条件选择退出了“固定大小类型·约束”`T: Sized`。
        // 自动成为`Shout trait`的委托类型
        impl<S, T> T where T: DerefMut<Target = S> + Drop,
                           S: ?Sized + Shout {
            fn deref(&self) -> &S;
            fn deref_mut(&mut self) -> &mut S;
        }
    }
    use ::std::error::Error;
    use crate::remote_structure::{PetBuilder, Shout};
    pub fn main() -> Result<(), Box<dyn Error>> {
        let pet = PetBuilder::default().name("a").build()?;
        let wrapper = Box::new(pet);
        print(wrapper, "input3");
        Ok(())
    }
    fn print(i: impl Shout, s: &str) {
        dbg!(i.shout(s));
    }
}
use ::std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    single_field_demo::main()?;
    multiple_field_demo::main()?;
    to_self_demo::main()?;
    generic_type_demo::main()?;
    generic_trait_demo::main()?;
    to_smart_pointer_demo::main()?;
    to_method_return_value::main()?;
    blanket_impl_demo::main()?;
    Ok(())
}
