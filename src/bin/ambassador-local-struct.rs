#[path = "../utils.rs"]
#[macro_use]
mod utils;
/// 【`trait`先定义，`strut / enum`再定义】的次序很重要。否则，`ambassador crate`
/// 给`trait`生成的【过程宏】对`struct / enum`定义不可见。
#[macro_use]
mod delegated_structure {
    use ::ambassador::delegatable_trait;
    use ::derive_builder::Builder;
    use ::std::fmt::Display;
    /// 它的作用可渗透至下游`crate`
    #[delegatable_trait]
    pub trait Shout {
        fn shout(&self, input: &str) -> String;
        fn alias(&mut self, name: &str);
    }
    /// 它的作用可渗透至下游`crate`
    #[delegatable_trait]
    pub trait ShoutGeneric<'a, 'b, T, R> where 'a: 'b, T: Display, R: Display {
        fn shout(&self, input1: &'a str, input2: &'b T) -> R;
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
    /// 【委托·目标（字段）类型】得实现【委托`trait`】。
    impl Shout for Pet {
        fn shout(&self, input: &str) -> String {
            format!("[{}] {} - meow!", self.name, input)
        }
        fn alias(&mut self, name: &str) {
            self.name = name.into();
        }
    }
}
/// 【单字段·结构体】委托
mod single_field_demo {
    mod delegating_structure {
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
        // 给【委托·类型】自动生成`trait`实现块
    }
    use delegating_structure::{FieldWrapperBuilder, TupleWrapper};
    use crate::delegated_structure::{PetBuilder, Shout};
    main!{pub, {
        let wrapper = TupleWrapper(PetBuilder::default().name("a").build()?);
        dbg!(wrapper.shout("input"));
        let wrapper = FieldWrapperBuilder::default().cat(PetBuilder::default().name("a").build()?).build()?;
        dbg!(wrapper.shout("input"));
    }}
}
/// 【多字段·结构体】委托至指定字段
mod multiple_field_demo {
    mod delegating_structure {
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
        // 给【委托·类型】自动生成`trait`实现块
    }
    use delegating_structure::{FieldWrapperBuilder, TupleWrapper};
    use crate::delegated_structure::{PetBuilder, Shout};
    main!{pub, {
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
    }}
}
/// 【自己·委托·自己】对【委托`trait`】提供`trait methods`与`inherent methods`的双份实现。
/// 特点：
/// （1）委托（目标）类型·即是·委托类型
/// （2）委托类型·不必·已实现【委托`trait`】（自身），但一定要已包含【委托`trait`】内所有成
///      员方法`trait methods`的实现块。否则，`unconditional_recursion`编译错误就会被抱怨。
/// （3）委托后的【委托类型】对【委托`trait`】定义了双份实现。包括：
///      - 原有的`inherent methods`实现
///      - 代理的`trait methods`实现
/// （4）因为`inherent methods`的`method resolution`优先级更高，所以若没有使用完全限定语法
///       `<Type as Trait>::method(&self, ...)`语法或`trait Object`，那么`trait methods`
///       几乎不会被调用到。
/// （5）在委托之后，【委托类型】也就实现了【委托`trait`】。
/// 【使用场景】需要满足如下几个条件：
///     1. `lib target`工程
///     2. 版本升级时，新版本·重构了·导出结构体`pub struct`的成员方法布局。
///     3. 重构目标：使用不同的`trait`对【导出·结构体】的【成员方法】做分组
///         3.1 被用作分组的`trait`既不能包含“关联·类型”也能不包含“关联·常量”。
///         3.2 若被用作分组`trait`的成员方法并没有被【委托·目标·类型`self`】逐一被实现（毕
///             竟，并没有从语法上`impl trait`），那么`unconditional_recursion`编译错误
///             就会出现。
///     4. 要求新版本的【导出·结构体】
///         4.1 既适用于【旧版】的具体类型·普通函数·调用方式`func_a(_: Cat)`
///         4.2 也适用于【新版】的`trait bound`·泛型函数·调用方式`func_a<T: Shout>(_: T)`
mod to_self_demo {
    mod delegating_structure {
        use ::ambassador::Delegate;
        use ::derive_builder::Builder;
        use crate::delegated_structure::Shout;
        #[derive(Builder, Debug)]
        #[derive(Delegate)]
        #[delegate(Shout, target = "self")] // 它会给`Cat`结构体再生成一个`impl Shout for Cat {...}`
                                            // 的`trait methods`实现块。
        pub struct Cat {
            aggressive: bool,
            #[builder(default)]
            name: Option<String>
        }
        ///【手写】`Inherent Methods`实现块 - 适用于旧版本`lib`调用端的`func_a(_: Cat)`普通函数
        impl Cat {
            pub fn shout(&self, input: &str) -> String {
                format!("[aggressive = {}] {} - meow!", self.aggressive, input)
            }
            fn alias(&mut self, name: &str) {
                self.name = Some(name.into());
            }
        }
        //【生成】`trait methods`实现块 - 适用于新版本`lib`调用端的`func_a<T: Shout>(_: T)`泛型函数
    }
    use delegating_structure::{Cat, CatBuilder};
    use crate::delegated_structure::Shout;
    main!{pub, {
        let cat = CatBuilder::default().aggressive(true).build()?;
        dbg!(cat.shout("input"));                    // 调用的`inherent method`实现
        dbg!(<Cat as Shout>::shout(&cat, "input2")); // 调用的`trait method`实现
                                                     // 这两者不一样。
    }}
}
/// 【泛型·结构体】委托至【泛型·类型·字段】`where`。其中，委托·目标【泛型·字段】需要满足两个条件：
/// （1）实现【委托`trait`】 — 在本例中是`Shout trait`
///      - 另一个属性`#[delegate(automatic_where_clause = "false")]`可被用来关闭该限制。但，
///        它的·解引用·类型依旧得实现【委托`trait`】。
/// （2）实现由`#[delegate(where)]`属性键-值对额外指定的`trait bounds` — 在本例中是`Display trait`
/// 最后，由`Ambassador crate`派生的过程宏·会给【委托·类型】自动添加`where`从句，来落实上述两条约束。
mod generic_type_demo {
    mod delegating_structure {
        use ::ambassador::Delegate;
        use ::derive_builder::Builder;
        #[cfg(feature = "ambassador-where")]
        use ::std::fmt::{Display, Formatter, Result};
        #[cfg(feature = "ambassador-where")]
        use crate::delegated_structure::Pet;
        use crate::delegated_structure::Shout;
        #[derive(Builder, Debug)]
        #[derive(Delegate)]
        #[cfg_attr(not(feature = "ambassador-where"), delegate(Shout))]
        #[cfg_attr(feature = "ambassador-where", delegate(Shout, where = "T: Display"))]
        pub struct FieldWrapper<T> /* #3. 自动添加
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
    use delegating_structure::FieldWrapperBuilder;
    use crate::delegated_structure::{PetBuilder, Shout};
    main!{pub, {
        let cat = PetBuilder::default().name("a").build()?;
        #[cfg(not(feature = "ambassador-where"))]
        dbg!(&cat);
        #[cfg(feature = "ambassador-where")]
        dbg!(cat.to_string());
        let wrapper = FieldWrapperBuilder::default().cat(cat).build()?;
        dbg!(wrapper.shout("input"));
    }}
}
/// 委托【泛型`trait`】`generics`。其中，【泛型`trait`】的【泛型参数】（含【限定条件】）
/// (1) 既要·被注册于`#[delegate(generics)]`属性键-值对内
/// (2) 还要·被添加于【委托·目标（字段）类型】的`trait`实现块的`where`从句内。譬如，
///     `impl<T> ShoutGeneric<T> for Pet where T: *** {`。
/// (3) 由`Ambassador crate`派生的过程宏·会自动“同步”【委托·目标（字段）类型】`trait`实现块
///     上的【`trait`泛型参数】（含【限定条件】）至【委托·类型】的`trait`实现块上。
/// 当`#[delegate(generics)]`与`#[delegate(where)]`并用时，
/// (1) `generics`注册来自于【泛型`trait`】的基本限定条件。宏展开后，出现于`trait A<T: Trait1>`的泛型
///     参数列表内。
/// (2) `where`注册来自于【委托·类型】的额外限定条件。宏展开后，出现于`struct A<T: Trait2, K: Trait3>`的泛型
///     参数列表内。（假设`K`是【委托·目标·字段】的泛型类型）
/// (3) 【委托·目标·类型】的泛型参数`T`需要满足`Trait1 + Trait2`的全部限定条件。
/// (4) 【委托·目标·类型】自身需要满足`A + Trait3`的限定条件。
mod generic_trait_demo {
    mod delegating_structure {
        use ::ambassador::Delegate;
        use ::derive_builder::Builder;
        use ::std::fmt::Display;
        use crate::delegated_structure::{Pet, ShoutGeneric};
        #[derive(Builder, Debug)]
        #[derive(Delegate)]
        /// #1. 【`trait`泛型参数】（含【限定条件】）被注册于`#[delegate(generics)]`属性键-值对
        #[delegate(ShoutGeneric<'a, 'b, T, R>, generics="'a: 'b, 'b, T: Display, R: Display")]
        pub struct Wrapper {
            cat: Pet
        }
        /// #2. 【`trait`泛型参数】（含【限定条件】）被添加于【委托·目标（字段）类型】的
        ///     `trait`实现块的`where`从句内。
        impl<'a, 'b, T> ShoutGeneric<'a, 'b, T, String> for Pet where 'a: 'b, T: Display {
            fn shout(&self, input1: &'a str, input2: &'b T) -> String {
                format!("[{}] {}, {} - meow!", self.name(), input1, input2)
            }
        }
        // #3. 给【委托·类型】生成【`trait`实现块】和添加【`trait`泛型参数】（含【限定条件】）
    }
    use ::std::net::{IpAddr, Ipv4Addr};
    use delegating_structure::{Wrapper, WrapperBuilder};
    use crate::delegated_structure::{PetBuilder, ShoutGeneric};
    main!{pub, {
        let cat = PetBuilder::default().name("a").build()?;
        let wrapper = WrapperBuilder::default().cat(cat).build()?;
        let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        dbg!(<Wrapper as ShoutGeneric<'_, '_, IpAddr, _>>::shout(&wrapper, "input1", &addr));
    }}
}
/// 委托至【智能·指针】（或称“间接”委托）。即，
/// （1）【智能·指针】类型自身并未直接实现【委托`trait`】。
/// （2）但由【智能·指针】引用的内部类型却实现了【委托`trait`】。
/// （3）借助于`.`操作符的【自动解引用】语法糖，【委托`trait`】的【成员方法】被允许从
///     【智能·指针】实例直接“点”出并调用。
/// 默认情况下，`Ambassador crate`要求【委托·目标（字段）类型】与【委托·类型】皆实现
/// 相同的【委托`trait`】。
/// `#[delegate(automatic_where_clause = "false")]`属性可用来用选择退出这个限制。
/// 它可以被理解为对`#[delegate_to_methods]`针对`Deref::deref()`与`DerefMut::deref_mut()`
/// 场景的语法糖。
mod to_smart_pointer_demo {
    mod delegating_structure {
        use ::ambassador::Delegate;
        use ::derive_builder::Builder;
        use crate::delegated_structure::{Pet, Shout};
        #[derive(Builder)]
        #[builder(pattern = "owned", setter(into))]
        #[derive(Delegate)]
        /// 【委托·目标（字段）类型】`Box<T>`自身并没有实现【委托`trait`】，虽然它被解引用后可调
        /// 用【委托`trait`】的成员方法。
        #[delegate(Shout, automatic_where_clause = "false")]
        pub struct BoxedPet {
            pet: Box<dyn Shout>
        }
        impl From<Pet> for Box<dyn Shout> {
            fn from(pet: Pet) -> Self {
                Box::new(pet)
            }
        }
        // 给【委托·类型】自动生成`trait`实现块
    }
    use delegating_structure::BoxedPetBuilder;
    use crate::delegated_structure::{PetBuilder, Shout};
    main!{pub, {
        let cat = PetBuilder::default().name("a").build()?;
        let boxed_pet = BoxedPetBuilder::default().pet(cat).build()?;
        dbg!(boxed_pet.shout("input"));
    }}
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
/// `#[delegate]`与`#[delegate_to_methods]`被修饰于`impl`块，而不是类型定义。
mod to_method_return_value {
    mod delegating_structure {
        use ::ambassador::delegate_to_methods;
        use ::derive_builder::Builder;
        use ::std::ops::{Deref, DerefMut};
        use crate::delegated_structure::{Pet, Shout};
        /// 注意：在类型定义上，没有`#[delegate]`属性。
        #[derive(Builder, Debug)]
        pub struct TargetMethodWrapper {
            #[builder(setter(into))]
            pet: Box<Pet>
        }
        #[delegate_to_methods]
        #[delegate(Shout, target_ref = "get_delegate_target", target_mut = "get_delegate_target_mut")]
        impl TargetMethodWrapper {
            fn get_delegate_target(&self) -> &Pet {
                self.pet.deref()
            }
            fn get_delegate_target_mut(&mut self) -> &mut Pet {
                self.pet.deref_mut()
            }
        }
    }
    use delegating_structure::TargetMethodWrapperBuilder;
    use crate::delegated_structure::{PetBuilder, Shout};
    main!{pub, {
        let cat = PetBuilder::default().name("a").build()?;
        let mut boxed_pet = TargetMethodWrapperBuilder::default().pet(cat).build()?;
        dbg!(boxed_pet.shout("input"));
        boxed_pet.alias("b");
        dbg!(boxed_pet.shout("input2"));
    }}
}
main!{{
    single_field_demo::main()?;
    multiple_field_demo::main()?;
    to_self_demo::main()?;
    generic_type_demo::main()?;
    generic_trait_demo::main()?;
    to_smart_pointer_demo::main()?;
    to_method_return_value::main()?;
}}
