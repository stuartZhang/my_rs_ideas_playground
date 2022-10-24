mod data_structure {
    use ::delegate::delegate;
    use ::derive_builder::Builder;
    use ::std::{cell::RefCell, rc::Rc};
    #[derive(Builder, Debug)]
    pub struct Stack {
        inner1: Vec<u32>,
        inner2: Rc<RefCell<String>>
    }
    impl Stack {
        delegate! {
            to self.inner1 { // 委托至【指定·字段】上的【成员方法】
                #[call(push)] // 修改·委托成员方法名`push -> add`
                pub fn add(&mut self, value: u32);
            }
            to self.inner2.borrow_mut() { // 委托至【指定·字段·表达式·返回值】上的【成员方法】
                #[call(push_str)] // 修改·委托成员方法名`push_str -> push`
                pub fn push(&mut self, val: &str);
            }
        }
    }
}
use ::std::{cell::RefCell, error::Error, rc::Rc};
use data_structure::StackBuilder;
fn main() -> Result<(), Box<dyn Error>> {
    let inner2 = Rc::new(RefCell::new("1".to_string()));
    let mut stack = StackBuilder::default()
        .inner1(vec![1, 2, 3])
        .inner2(Rc::clone(&inner2))
        .build()?;
    stack.add(5);
    stack.push("abc");
    println!("stack = {:?}", stack);
    Ok(())
}