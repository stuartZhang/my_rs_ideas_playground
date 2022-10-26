/// 这是一个在【结构体·包装类型】`Wrapper`上，将部分【成员方法】委托给其
/// 【字段值】或【字段值·表达式·返回值】的【成员方法】的例子。
/// ---------------------------------------------------------------
/// `delegate crate`的优点：“以【成员成员】为最小委托单元，简单直观”，而
/// 缺点：“不能让【结构体·包装类型】`Wrapper`自动实现只有【字段值】或【字
/// 段值·表达式·返回值】才具备的`trait`”。
/// ---------------------------------------------------------------
/// 涉及到的功能点包括
/// （1）修改·被代理成员方法·的【方法名】
/// （2）修改·被代理成员方法·的【返回值类型】— 类型转换·需要·`From / TryFrom trait`实现。
/// （3）忽略·被代理成员方法·的【返回值】
/// （4）将【成员方法】委托于不同的【字段值】或【字段值·表达式·返回值】
mod data_structure {
    use ::delegate::delegate;
    use ::derive_builder::Builder;
    use ::std::{cell::RefCell, rc::Rc};
    #[derive(Builder, Debug)]
    pub struct Wrapper {
        inner1: Vec<u32>,
        inner2: Rc<RefCell<String>>,
        size: i32
    }
    impl Wrapper {
        delegate! {
            to self.inner1 { // 委托至【指定·字段】上的【成员方法】
                #[call(push)] // 修改·被代理成员方法名`push -> add`
                pub fn add(&mut self, value: u32);
                #[call(len)]
                #[try_into(unwrap)] // 修改·被代理成员方法·的返回值类型`usize -> u64`。
                                    // 利用`TryFrom trait`对【成员方法】的返回值做类型转换，
                                    // 并自动“拆箱”。
                pub fn size(&self) -> u64;
                // 忽略·委托成员方法·的返回值。
                pub fn pop(&mut self);
            }
            to self.inner2.borrow_mut() { // 委托至【指定·字段·表达式·返回值】上的【成员方法】
                #[call(push_str)] // 修改·被代理成员方法名`push_str -> push`
                pub fn push(&mut self, val: &str);
            }
            to self.size {
                #[into] // 修改·被代理成员方法·的返回值类型`i32 -> i64`。
                        // 利用`From trait`对【成员方法】的返回值做类型转换。
                pub fn pow(&self, exp: u32) -> i64;
            }
        }
    }
}
use ::std::{cell::RefCell, error::Error, rc::Rc};
use data_structure::WrapperBuilder;
fn main() -> Result<(), Box<dyn Error>> {
    let inner2 = Rc::new(RefCell::new("1".to_string()));
    let mut wrapper = WrapperBuilder::default()
        .inner1(vec![1, 2, 3])
        .inner2(Rc::clone(&inner2))
        .size(16)
        .build()?;
    // 经由【成员方法】的别名，调用【字段值】上的底层成员方法。
    wrapper.add(5);
    wrapper.push("abc");
    // 修改被代理成员方法的返回值类型
    dbg!(wrapper.pow(2));
    dbg!(wrapper.size());
    // 忽略被代理成员方法的返回值
    wrapper.pop();
    println!("wrapper = {:?}", wrapper);
    Ok(())
}