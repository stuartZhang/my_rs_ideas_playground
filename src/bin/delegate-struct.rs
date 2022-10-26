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