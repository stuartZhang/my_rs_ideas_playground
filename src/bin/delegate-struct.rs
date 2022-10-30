/// 这是一个在【结构体·包装类型】`Wrapper`上，将部分【成员方法】委托给其
/// 【字段】或【字段·表达式·返回值】的【成员方法】的例子。
/// ---------------------------------------------------------------
/// 使用场景：`Newtypes`设计模式
/// ---------------------------------------------------------------
/// `delegate crate`的优点：“以【成员方法】为最小委托单元，简单直观”，而
/// 缺点：“不能让【结构体·包装类型】`Wrapper`自动实现在【字段值】或【字段
/// ·表达式·返回值】上的`trait`”。
/// ---------------------------------------------------------------
/// 涉及到的功能点包括
/// （1）修改·被代理成员方法·的【方法名】
/// （2）修改·被代理成员方法·的【形参类型】— 类型转换·需要·`From trait`实现。
/// （3）预填·被代理成员方法·的【实参】— 支持【求值·表达式】与`self`引用
/// （4）修改·被代理成员方法·的【返回值·类型】— 类型转换·需要·`From / TryFrom trait`实现。
/// （5）忽略·被代理成员方法·的【返回值】
/// （6）将【成员方法】委托至不同的【字段】或【字段·表达式·返回值】
/// （7）代理·异步成员方法
mod data_structure {
    use ::async_std::{fs, future::Future};
    use ::delegate::delegate;
    use ::derive_builder::Builder;
    use ::std::{cell::RefCell, env, error::Error, rc::Rc};
    const BASE_INT: i32 = 32;
    #[derive(Builder, Debug)]
    pub struct Wrapper {
        inner1: Vec<u32>,
        inner2: Rc<RefCell<String>>,
        size: i32,
        #[builder(default)]
        polynomial: Polynomial
    }
    impl Wrapper {
        delegate! {
            to self.inner1 { // #6. 委托至【指定·字段】上的【成员方法】
                #[call(push)] // #1. 修改·被代理成员方法名`push -> add`
                pub fn add(&mut self, value: u32);
                #[call(len)]
                #[try_into(unwrap)] // #4. 修改·被代理成员方法·的返回值类型`usize -> u64`。
                                    //     利用`TryFrom trait`对【成员方法】的返回值做类型转换，
                                    //     并自动“拆箱”。若缺省元属性值`unwrap`，便需要明文手动
                                    //     “拆箱”。
                pub fn size(&self) -> u64;
                // #5. 忽略·委托成员方法·的返回值。
                pub fn pop(&mut self);
            }
            to self.inner2.borrow_mut() { // #6. 委托至【指定·字段·表达式·返回值】上的【成员方法】
                #[call(push_str)] // #1. 修改·被代理成员方法名`push_str -> push`
                pub fn push(&mut self, val: &str);
                #[call(push)] // #2. 修改·被代理成员方法·的【形参类型】`u8 -> char`
                              //     利用`From trait`对【成员方法】的【形参】做类型转换。
                pub fn push_u8(&mut self, #[into] val: u8);
            }
            to self.size { // #6. 委托至【指定·字段】上的【成员方法】
                #[into] // #4. 修改·被代理成员方法·的返回值类型`i32 -> i64`。
                        //     利用`From trait`对【成员方法】的返回值做类型转换。
                pub fn pow(&self, exp: u32) -> i64;
            }
            to self.polynomial { // #6. 委托至【指定·字段】上的【成员方法】
                // #3. 预填·被代理成员方法·的【实参】。支持
                //     （1）求值·表达式
                //     （2）`self`引用当前【结构体】实例
                pub fn polynomial(&self, [self.inner1.len() as i32], [1 + BASE_INT], [self.size], y: i32) -> i32;
                // #7. 代理·异步成员方法
                //     （1）异步函数语法糖返回`Result<String, Box<dyn Error>>`
                #[call(load_cargo_toml)]
                pub async fn load_cargo_toml1(&self) -> Result<String, Box<dyn Error>>;
                // #7. 代理·异步成员方法
                //     （2）普通函数返回`Future<Output = Result<String, Box<dyn Error>>>`
                #[call(load_cargo_toml)]
                pub fn load_cargo_toml2<'a>(&'a self) -> impl Future<Output = Result<String, Box<dyn Error + 'static>>> + 'a;
                // #7. 代理·异步成员方法
                //     （3）异步函数语法糖返回`Result<String, Box<dyn Error>>`。但，在代理函数内不执行`.await`操作。
                #[await(false)]
                #[call(load_cargo_toml)]
                pub async fn load_cargo_toml_fut1<'a>(&'a self) -> impl Future<Output = Result<String, Box<dyn Error + 'static>>> + 'a;
                // #7. 代理·异步成员方法
                //     （4）普通函数返回`Future<Output = Result<String, Box<dyn Error>>>`。但，在代理函数内不执行`.await`操作。
                #[await(false)]
                #[call(load_cargo_toml)]
                pub fn load_cargo_toml_fut2<'a>(&'a self) -> impl Future<Output = Result<String, Box<dyn Error + 'static>>> + 'a;
            }
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Polynomial;
    impl Polynomial {
        fn polynomial(&self, a: i32, x: i32, b: i32, y: i32) -> i32 {
            a + x * x + b * y
        }
        async fn load_cargo_toml(&self) -> Result<String, Box<dyn Error>> {
            let mut cargo_file_path = env::current_dir()?;
            cargo_file_path.push("Cargo.toml");
            let contents = fs::read_to_string(cargo_file_path).await?;
            Ok(contents)
        }
    }
}
use ::async_std::task;
use ::std::{cell::RefCell, error::Error, rc::Rc};
use data_structure::WrapperBuilder;
fn main() -> Result<(), Box<dyn Error>> {
    let inner2 = Rc::new(RefCell::new("1".to_string()));
    let mut wrapper = WrapperBuilder::default()
        .inner1(vec![1, 2, 3])
        .inner2(Rc::clone(&inner2))
        .size(16)
        .build()?;
    // #1. 经由【成员方法】的别名，调用【字段】上的底层成员方法。
    wrapper.add(5);
    wrapper.push("abc");
    // #2. 修改·被代理成员方法·的【形参类型】
    wrapper.push_u8(200);
    // #4. 修改被代理成员方法的返回值类型
    dbg!(wrapper.pow(2));
    dbg!(wrapper.size());
    // #5. 忽略被代理成员方法的返回值
    wrapper.pop();
    // #3. 预填·被代理成员方法·的【实参】
    dbg!(wrapper.polynomial(1));
    // #7. 代理·异步成员方法
    //     （1）异步函数语法糖返回`Result<String, Box<dyn Error>>`
    dbg!(task::block_on(wrapper.load_cargo_toml1()).expect("文件加载失败"));
    dbg!(task::block_on(async {
        let content: Result<String, Box<dyn Error>> = wrapper.load_cargo_toml1().await;
        content
    }).expect("文件加载失败"));
    // #7. 代理·异步成员方法
    //     （2）普通函数返回`Future<Output = Result<String, Box<dyn Error>>>`
    dbg!(task::block_on(async {
        let content: Result<String, Box<dyn Error>> = wrapper.load_cargo_toml2().await;
        content
    }).expect("文件加载失败"));
    // #7. 代理·异步成员方法
    //     （3）异步函数语法糖返回`Result<String, Box<dyn Error>>`。但，在代理函数内不执行`.await`操作。
    dbg!(task::block_on(async {
        let content: Result<String, Box<dyn Error>> = wrapper.load_cargo_toml_fut1().await.await;
        content
    }).expect("文件加载失败"));
    // #7. 代理·异步成员方法
    //     （4）普通函数返回`Future<Output = Result<String, Box<dyn Error>>>`。但，在代理函数内不执行`.await`操作。
    dbg!(task::block_on(async {
        let content: Result<String, Box<dyn Error>> = wrapper.load_cargo_toml_fut2().await;
        content
    }).expect("文件加载失败"));
    println!("wrapper = {:?}", wrapper);
    Ok(())
}