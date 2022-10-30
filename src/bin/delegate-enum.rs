/// 这是一个在【枚举类】`Enum`上，将部分【成员方法】委托给其【结构体·枚举值】
/// 或【结构体·枚举值·表达式·返回值】的【成员方法】的例子。
/// ---------------------------------------------------------------
/// 使用场景：`Newtypes`设计模式
/// ---------------------------------------------------------------
/// `delegate crate`的优点：“以【成员成员】为最小委托单元，简单直观”，而
/// 缺点：“不能让【枚举类】`Enum`自动实现只有【结构体·枚举值】或【结构体·
/// 枚举值·表达式·返回值】才具备的`trait`”。
/// ---------------------------------------------------------------
/// 涉及到的功能点包括
/// （1）将【成员方法】委托于不同的【结构体·枚举值】或【结构体·枚举值·表达式·返回值】
mod delegating_structure {
    use ::delegate::delegate;
    use crate::delegated_structure::{A, B, C, D};
    #[derive(Debug)]
    pub enum Enum {
        A(A),
        B(B),
        C {
            a: A,
            b: B,
            c: C
        }
    }
    impl Enum {
        delegate! {
            // #1. 委托至【结构体·枚举值】上的【成员方法】
            // 宏展开后的完整形式
            // ```rust
            // match self {
            //     Enum::A(a) => {a}.dbg_inner(),
            //     Enum::B(b) => {println!("i am b"); b}.dbg_inner(),
            //     Enum::C {c: C} => {c}.dbg_inner(),
            // }
            // ```
            to match self {
                Enum::A(a) => a,
                Enum::B(b) => {
                    println!("i am b");
                    b
                },
                Enum::C {c, ..} => c,
            } {
                pub fn dbg_inner(&self) -> usize;
            }
            // 委托至【结构体·枚举值】上的【成员方法】
            to match self {
                Enum::C {a, ..} => a,
                _ => D
            } {
                #[call(dbg_inner)]
                pub fn dbg_inner_a(&self) -> usize;
            }
        }
    }
}
mod delegated_structure {
    use ::derive_builder::Builder;
    #[derive(Builder, Debug)]
    pub struct A {
        val_a: usize,
    }
    impl A {
        pub fn dbg_inner(&self) -> usize {
            dbg!(self.val_a);
            1
        }
    }
    #[derive(Builder, Debug)]
    pub struct B {
        #[builder(setter(into))]
        val_b: String,
    }
    impl B {
        pub fn dbg_inner(&self) -> usize {
            dbg!(self.val_b.clone());
            2
        }
    }
    #[derive(Builder, Debug)]
    pub struct C {
        val_c: f64,
    }
    impl C {
        pub fn dbg_inner(&self) -> usize {
            dbg!(self.val_c);
            3
        }
    }
    pub struct D;
    impl D {
        pub fn dbg_inner(&self) -> usize {
            unreachable!()
        }
    }
}
use ::std::error::Error;
use delegating_structure::{Enum};
use delegated_structure::{ABuilder, BBuilder, CBuilder};
fn main() -> Result<(), Box<dyn Error>> {
    let a = Enum::A(ABuilder::default().val_a(12).build()?);
    let b = Enum::B(BBuilder::default().val_b("abc").build()?);
    let c = Enum::C {
        a: ABuilder::default().val_a(1).build()?,
        b: BBuilder::default().val_b("value").build()?,
        c :CBuilder::default().val_c(12.5).build()?
    };
    a.dbg_inner();
    b.dbg_inner();
    c.dbg_inner();
    c.dbg_inner_a();
    Ok(())
}